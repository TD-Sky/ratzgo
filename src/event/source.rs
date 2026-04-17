use std::{any::type_name_of_val, pin::Pin, task::Poll};

use futures_util::{Stream, StreamExt, TryStream, stream::LocalBoxStream};

use crate::utils::future::OkOrPending;

pub struct SelectEventSource<Message> {
    streams: Vec<LocalBoxStream<'static, Message>>,
}

impl<Message> std::fmt::Debug for SelectEventSource<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectEventSource")
            .field(
                "streams",
                &format_args!("<list of `{}`>", type_name_of_val(&self.streams)),
            )
            .finish()
    }
}

impl<Message> Default for SelectEventSource<Message> {
    fn default() -> Self {
        Self {
            streams: Default::default(),
        }
    }
}

impl<Message> SelectEventSource<Message> {
    pub fn source<S>(&mut self, stream: S) -> &mut Self
    where
        S: Stream<Item = Message> + 'static,
    {
        self.streams.push(stream.boxed_local());
        self
    }

    pub fn source_try<S>(&mut self, stream: S) -> &mut Self
    where
        S: TryStream<Ok = Message> + 'static,
    {
        self.streams.push(OkOrPending::new(stream).boxed_local());
        self
    }
}

impl<Message> Stream for SelectEventSource<Message> {
    type Item = Message;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if self.streams.is_empty() {
            return Poll::Pending;
        }

        let mut drop_first_n = 0;
        let mut res = Poll::Pending;

        for i in 0..self.streams.len() {
            match self.streams[i].poll_next_unpin(cx) {
                Poll::Ready(Some(msg)) => {
                    res = Poll::Ready(msg);
                    break;
                }
                Poll::Ready(None) => {
                    self.streams[..=i].rotate_right(1);
                    drop_first_n += 1;
                }
                Poll::Pending => (),
            }
        }

        if drop_first_n > 0 {
            self.streams.drain(..drop_first_n);
        }

        res.map(Some)
    }
}
