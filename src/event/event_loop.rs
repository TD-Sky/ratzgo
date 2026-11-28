use std::{
    io::{self, Stdout, stdout},
    mem::DropGuard,
    panic,
    time::Duration,
};

use futures_util::{FutureExt, StreamExt, select};
use ratatui_core::{backend::Backend, layout::Rect, terminal::Terminal};
use ratatui_crossterm::{
    CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

use crate::{
    core::Component,
    event::{SelectEventSource, UnsyncDebounce, UnsyncQueue, YieldFg},
};

pub async fn default_event_loop<S, State, Init, Message, Update, View>(
    mut state: S,
    init: Init,
    mut update: Update,
    view: View,
) -> io::Result<()>
where
    S: AsRef<State> + AsMut<State>,
    Message: std::fmt::Debug,
    Init: AsyncFnOnce(&mut State, &mut RuntimeContext<Message, State>),
    Update: AsyncFnMut(&mut State, Message, &mut RuntimeContext<Message, State>),
    View: for<'a> Fn(&'a mut State) -> Box<dyn Component<Message> + 'a>,
{
    let _restore = DropGuard::new((), |_| try_restore().expect("try restoring terminal"));

    let mut terminal = try_init()?;
    let mut event_stream = EventStream::new();
    let mut ctx = RuntimeContext {
        queue: UnsyncQueue::default(),
        select: SelectEventSource::default(),
        yield_fg: None,
        exit: false,
    };

    init(state.as_mut(), &mut ctx).await;

    if ctx.exit {
        return Ok(());
    }

    let mut cpt = view(state.as_mut());
    terminal.draw(|frame| {
        cpt.render(frame.area(), frame.buffer_mut());
    })?;

    loop {
        select! {
            (event, _) = (&mut event_stream).into_future() => {
                match event {
                    Some(Ok(Event::Resize(..))) => {
                        drop(cpt);
                    }
                    Some(Ok(event)) => {
                        let (msg, rebuild) = handle_terminal_event(event, cpt.as_mut());

                        if let Some(msg) = msg {
                            ctx.queue.push(msg);
                        }

                        // NOTE: Keep the drawn tree: rebuilding it here would leave
                        //       every area at `Rect::ZERO` until some message forces
                        //       the redraw at the bottom of the loop.
                        if !rebuild {
                            continue;
                        }

                        drop(cpt);
                    },
                    Some(Err(_)) => continue,
                    None => break,
                }
            }

            msg = ctx.queue.pop().fuse() => {
                drop(cpt);
                update(state.as_mut(), msg, &mut ctx).await;

                if ctx.exit {
                    break;
                }
            }

            msg = ctx.select.next().fuse() => {
                let msg = msg.expect("always get `Some` if ready");
                drop(cpt);
                update(state.as_mut(), msg, &mut ctx).await;

                if ctx.exit {
                    break;
                }
            }
        }

        if let Some(f) = ctx.yield_fg.take() {
            drop(event_stream);

            let msg = f.run(state.as_mut(), &mut ctx, &mut terminal).await;

            event_stream = EventStream::new();

            if let Some(msg) = msg {
                update(state.as_mut(), msg, &mut ctx).await;

                if ctx.exit {
                    break;
                }
            }
        }

        cpt = view(state.as_mut());
        terminal.draw(|frame| {
            cpt.render(frame.area(), frame.buffer_mut());
        })?;
    }

    Ok(())
}

pub fn try_init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = try_restore();
        old_hook(info);
    }));

    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn try_restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen)?;

    Ok(())
}

pub fn reinit<B>(terminal: &mut Terminal<B>) -> io::Result<()>
where
    B: Backend<Error = io::Error>,
{
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // NOTE: ratatui `terminal.clear()` would query cursor position,
    //       which conflicts with crossterm event reading,
    //       so we choose `resize` as ratatui would clean screen,
    //       reset the buffer and won't query cursor position.
    let area: Rect = terminal.size()?.into();
    terminal.resize(area)
}

/// Dispatches a terminal event, reporting whether the view must be rebuilt.
///
/// `rebuild` is `false` when the event can be handed to the current tree without
/// invalidating it: handlers that are reusable (`FnMut`) and events with no
/// handler at all. The caller must keep that tree, because a rebuilt tree has not
/// been drawn yet and therefore still reports `Rect::ZERO` for every area.
fn handle_terminal_event<Message>(
    event: Event,
    root: &mut dyn Component<Message>,
) -> (Option<Message>, bool) {
    match event {
        Event::Key(event) => (root.handle_key(&event), true),
        Event::Mouse(event) => {
            let msg = root.handle_mouse(&event);
            let rebuild = msg.is_some();
            (msg, rebuild)
        }
        Event::Paste(content) => (root.handle_paste(&content), true),
        _ => (None, false),
    }
}

#[derive(Debug)]
pub struct RuntimeContext<Message, State> {
    queue: UnsyncQueue<Message>,
    select: SelectEventSource<Message>,
    yield_fg: Option<YieldFg<Message, State, Self>>,
    exit: bool,
}

impl<Message, State> RuntimeContext<Message, State> {
    pub fn queue(&self) -> &UnsyncQueue<Message> {
        &self.queue
    }

    pub fn make_debounce(&self, dur: Duration) -> UnsyncDebounce<Message>
    where
        Message: 'static,
    {
        UnsyncDebounce::new(dur, self.queue.clone())
    }

    pub fn select_mut(&mut self) -> &mut SelectEventSource<Message> {
        &mut self.select
    }

    pub fn set_fg(&mut self, yield_fg: YieldFg<Message, State, Self>) {
        self.yield_fg = Some(yield_fg);
    }

    pub fn exit(&mut self, yes: bool) {
        self.exit = yes;
    }
}
