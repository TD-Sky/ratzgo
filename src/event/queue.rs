use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Poll, Waker},
};

use thin_cell::unsync::ThinCell;

#[derive(Debug)]
pub struct UnsyncQueue<Message> {
    inner: ThinCell<Inner<Message>>,
}

impl<Message> Clone for UnsyncQueue<Message> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Message> Default for UnsyncQueue<Message> {
    fn default() -> Self {
        Self {
            inner: ThinCell::new(Inner {
                base: Default::default(),
                waker: None,
            }),
        }
    }
}

#[derive(Debug)]
struct Inner<Message> {
    base: VecDeque<Message>,
    waker: Option<Waker>,
}

impl<Message> UnsyncQueue<Message> {
    pub fn push(&self, elt: impl Into<Message>) {
        let mut inner = self.inner.borrow();
        inner.base.push_front(elt.into());
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }

    pub fn pop(&self) -> UnsyncQueuePop<'_, Message> {
        UnsyncQueuePop { queue: self }
    }

    pub fn spawn<F, T>(&self, f: F)
    where
        F: Future<Output = T> + 'static,
        T: Into<Message>,
        Message: 'static,
    {
        let queue = Self::clone(self);
        compio_runtime::spawn(async move {
            queue.push(f.await);
        })
        .detach();
    }

    pub fn spawn_opt<F, T, E>(&self, f: F)
    where
        F: Future<Output = Option<T>> + 'static,
        T: Into<Message>,
        Message: 'static,
    {
        let queue = Self::clone(self);
        compio_runtime::spawn(async move {
            if let Some(v) = f.await {
                queue.push(v);
            }
        })
        .detach();
    }

    pub fn spawn_try<F, T, E>(&self, f: F)
    where
        F: Future<Output = Result<T, E>> + 'static,
        T: Into<Message>,
        Message: 'static,
    {
        let queue = Self::clone(self);
        compio_runtime::spawn(async move {
            if let Ok(v) = f.await {
                queue.push(v);
            }
        })
        .detach();
    }
}

#[derive(Debug)]
pub struct UnsyncQueuePop<'a, Message> {
    queue: &'a UnsyncQueue<Message>,
}

impl<Message> Future for UnsyncQueuePop<'_, Message> {
    type Output = Message;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if let Some(elt) = self.queue.inner.borrow().base.pop_back() {
            return Poll::Ready(elt);
        }

        self.queue.inner.borrow().waker = Some(cx.waker().clone());
        Poll::Pending
    }
}
