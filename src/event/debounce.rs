use std::{
    any::type_name_of_val,
    task::{Poll, Waker},
    time::Duration,
};

use compio_runtime::time::sleep;
use futures_util::{
    FutureExt,
    future::{LocalBoxFuture, poll_fn},
    select,
};
use thin_cell::unsync::ThinCell;

use crate::event::UnsyncQueue;

#[derive(Debug)]
pub struct UnsyncDebounce<Message> {
    next_version: u32,
    inner: ThinCell<Inner<Message>>,
}

impl<Message> UnsyncDebounce<Message> {
    pub fn new(dur: Duration, queue: UnsyncQueue<Message>) -> Self
    where
        Message: 'static,
    {
        let inner = ThinCell::default();

        {
            let inner = inner.clone();
            compio_runtime::spawn(async move {
                loop {
                    let mut task = poll_fn(|cx| listen(&inner, cx)).await;

                    loop {
                        select! {
                            v = poll_fn(|cx| listen(&inner, cx)).fuse() => {
                                task = v;
                            }
                            _ = sleep(dur).fuse() => {
                                if let Some(msg) = task.await {
                                    queue.push(msg);
                                }
                                break;
                            }
                        }
                    }
                }
            })
            .detach();
        }

        Self {
            next_version: 0,
            inner,
        }
    }

    pub fn version(&self) -> u32 {
        self.next_version.overflowing_sub(1).0
    }

    pub fn next_version(&self) -> u32 {
        self.next_version
    }

    pub fn spawn<F, Fut, T>(&mut self, f: F)
    where
        F: FnOnce(u32) -> Fut,
        Fut: Future<Output = T> + 'static,
        T: Into<Message>,
    {
        self.spawn_opt(|version| f(version).map(|v| Some(v.into())));
    }

    pub fn spawn_opt<F, Fut, T>(&mut self, f: F)
    where
        F: FnOnce(u32) -> Fut,
        Fut: Future<Output = Option<T>> + 'static,
        T: Into<Message>,
    {
        let version = self.next_version;
        self.next_version = version.overflowing_add(1).0;

        let mut inner = self.inner.borrow();
        inner.base = Some(f(version).map(|v| v.map(Into::into)).boxed_local());
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }

    pub fn spawn_try<F, Fut, T, E>(&mut self, f: F)
    where
        F: FnOnce(u32) -> Fut,
        Fut: Future<Output = Result<T, E>> + 'static,
        T: Into<Message>,
    {
        self.spawn_opt(|version| f(version).map(|v| v.ok().map(Into::into)));
    }

    pub fn cancel(&mut self) {
        self.spawn_opt(|_| async { Option::<Message>::None });
    }
}

struct Inner<Message> {
    base: Option<LocalBoxFuture<'static, Option<Message>>>,
    waker: Option<Waker>,
}

impl<Message> std::fmt::Debug for Inner<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inner")
            .field(
                "base",
                &format_args!("<future of `{}`>", type_name_of_val(&self.base)),
            )
            .field("waker", &self.waker)
            .finish()
    }
}

impl<Message> Default for Inner<Message> {
    fn default() -> Self {
        Self {
            base: None,
            waker: None,
        }
    }
}

fn listen<Message>(
    inner: &ThinCell<Inner<Message>>,
    cx: &mut std::task::Context<'_>,
) -> Poll<LocalBoxFuture<'static, Option<Message>>> {
    if let Some(task) = inner.borrow().base.take() {
        return Poll::Ready(task);
    }

    inner.borrow().waker = Some(cx.waker().clone());
    Poll::Pending
}
