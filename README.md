<h1 align="center">Rat'⚡go</h1>

<p align="center"><strong><em>Empower ratatui widgets event handling easily, let's go into carnival of ratatui right now !!!</em></strong></p>

<p align="center">
  <a href="https://crates.io/crates/ratzgo"><img src="https://img.shields.io/crates/v/ratzgo.svg" alt="crates.io"></a>
  <a href="https://docs.rs/ratzgo/"><img src="https://docs.rs/ratzgo/badge.svg" alt="docs.rs"></a>
  <a href="https://deepwiki.com/TD-Sky/ratzgo"><img src="https://deepwiki.com/badge.svg" alt="Ask DeepWiki"></a>
</p>



## Table of contents

- [Introduction](#introduction)
- [Features](#features)
- [Examples](#examples)
- [Manual](#manual)
- [Roadmap](#roadmap)
- [Note](#note)



## Introduction

ratzgo is a **composable**, **async-first**, **Elm‑like** ratatui framework. Rather than offering a batteries‑included component library, it focuses on simplifying the composition of ratatui widgets and reducing the complexity of maintaining TUI logic.



## Features

- **Composable**: Compose view units `ratzgo::core::Widget` however you like.
- **Async-first**: The application runs inside an async event loop.
- **Elm-like**: *View-Message-Update* architecture, implemented in Rust with async support.
- **Controllable reactive**: Control whether a widget reacts to events via `ratzgo::core::Widget::active`.
- **Message passing**: Send message directly, or spawn background tasks that return message with `UnsyncQueue`.
- **Custom event sources**: Listen to multiple `Stream`s simultaneously using `SelectEventSource`.
- **Yield foreground**: Hand back interactive control to the terminal with `YieldFg`.
- **Debouncing**: Drop stale `Future`s effortlessly with `UnsyncDebounce`.
- **Logging**: Publish logs to `LogStream` from anywhere.
- **Z‑axis stacking**: Use `Stack` container and `MountPoint` for floating layers, opening modals, popups, and toast anywhere in view logic.
- **Scroll & fit**: Scroll by fixed row/column count or screen percentage, with optional margins.



## Examples

For a working example, you can refer to my [jujutsu TUI](https://github.com/TD-Sky/jj-bond).



## Manual

TODO



## Roadmap

Refer to github issue: [ratzgo Roadmap](https://github.com/TD-Sky/ratzgo/issues/1)



## Note

You may have noticed that the word **`Unsync...`** appears several times throughout this article.
This is because I chose [compio](https://crates.io/crates/compio) as the async runtime.
For client‑side applications, UI logic always runs on a single thread,
and **completion‑based async** (e.g., io_uring) can initiate I/O operations at a lower cost than **polling‑based async** (e.g., epoll).
I'm not meaning opposed to multi‑threaded parallelism – you are free to bring in your own thread pools and channels, and combine them with ratzgo async utilities to maximize task throughput.

Still, [tokio](https://crates.io/crates/tokio) is brilliant in the Rust world.
The well-known `reqwest` crate, for instance, has a hard dependency on tokio.
Tokio’s work‑stealing mechanism, designed for load balancing,
also delivers higher task throughput than the thread-per-core model under moderate QPS.
Tokio also natively supports an M:N model, making it easy to run asynchronous tasks on thread pools.
All of this is undeniably tempting.
But I haven’t figured out if or how to support both I/O models.
Since the divergence between completion-based and polling-based async has profound implications for Rust’s async runtime modeling.
Adding such support would inevitably bring significant challenges to ratzgo.
For now, I’ll leave the answer to time.
