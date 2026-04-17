use std::{
    pin::Pin,
    task::{Poll, ready},
};

use futures_util::{Stream, TryStream};
use pin_project_lite::pin_project;

pin_project! {
    #[derive(Debug)]
    pub struct OkOrPending<S> {
        #[pin]
        base: S,
    }
}

impl<S> OkOrPending<S> {
    pub fn new(stream: S) -> Self {
        Self { base: stream }
    }
}

impl<S> Stream for OkOrPending<S>
where
    S: TryStream,
{
    type Item = <S as TryStream>::Ok;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let stream = self.project();

        match ready!(stream.base.try_poll_next(cx)) {
            Some(Ok(v)) => Poll::Ready(Some(v)),
            Some(Err(_)) => Poll::Pending,
            None => Poll::Ready(None),
        }
    }
}
