use std::{any::type_name_of_val, io::Stdout};

use futures_util::{FutureExt, future::LocalBoxFuture};
use ratatui_core::terminal::Terminal;
use ratatui_crossterm::CrosstermBackend;

use crate::event::event_loop::{reinit, try_restore};

pub struct YieldFg<Message, State, Ctx> {
    inner: Inner<Message, State, Ctx>,
}

type Inner<Message, State, Ctx> = Box<
    dyn for<'a> FnOnce(&'a mut State, &'a mut Ctx) -> LocalBoxFuture<'a, Option<Message>> + 'static,
>;

impl<Message, State, Ctx> std::fmt::Debug for YieldFg<Message, State, Ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YieldFg")
            .field(
                "inner",
                &format_args!("<async closure of `{}`>", type_name_of_val(&self.inner)),
            )
            .finish()
    }
}

impl<Message, State, Ctx> YieldFg<Message, State, Ctx> {
    pub fn new<F, T>(f: F) -> Self
    where
        F: for<'a> AsyncFnOnce(&'a mut State, &'a mut Ctx) -> T + 'static,
        T: Into<Message>,
        Message: 'static,
    {
        Self::new_opt(async move |state, ctx| f(state, ctx).map(|v| Some(v.into())).await)
    }

    pub fn new_opt<F, T>(f: F) -> Self
    where
        F: for<'a> AsyncFnOnce(&'a mut State, &'a mut Ctx) -> Option<T> + 'static,
        T: Into<Message>,
        Message: 'static,
    {
        Self {
            inner: Box::new(|state, ctx| f(state, ctx).map(|v| v.map(Into::into)).boxed_local()),
        }
    }

    pub fn new_try<F, T, E>(f: F) -> Self
    where
        F: for<'a> AsyncFnOnce(&'a mut State, &'a mut Ctx) -> Result<T, E> + 'static,
        T: Into<Message>,
        Message: 'static,
    {
        Self::new_opt(async move |state, ctx| f(state, ctx).map(|v| v.ok().map(Into::into)).await)
    }

    pub fn new_ignore<F>(f: F) -> Self
    where
        F: for<'a> AsyncFnOnce(&'a mut State, &'a mut Ctx) + 'static,
        Message: 'static,
    {
        Self::new_opt(async move |state, ctx| {
            f(state, ctx).await;
            Option::<Message>::None
        })
    }

    pub async fn run(
        self,
        state: &mut State,
        ctx: &mut Ctx,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Option<Message> {
        try_restore().expect("fail at restoring to terminal");
        let msg = (self.inner)(state, ctx).await;
        reinit(terminal).expect("fail at reinit to TUI");
        msg
    }
}
