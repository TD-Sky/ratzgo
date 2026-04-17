use crate::scroll::{repos_y, repos_y_anchored};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAction {
    Fixed(i16),
    Viewport(i16),
}

pub fn scroll_vertical(
    action: ScrollAction,
    mut scroll_y: u16,
    total_lines: usize,
    viewport_height: u16,
) -> u16 {
    let offset = match action {
        ScrollAction::Fixed(n) => n,
        ScrollAction::Viewport(n) => (viewport_height as f32 * n as f32 * 0.01) as i16,
    };
    scroll_y = scroll_y.saturating_add_signed(offset);
    repos_y(scroll_y, total_lines, viewport_height)
}

pub fn scroll_vertical_anchored(
    action: ScrollAction,
    mut scroll_y: u16,
    total_lines: usize,
    viewport_height: u16,
    threshold_lines: u16,
    anchor_line: usize,
) -> u16 {
    let offset = match action {
        ScrollAction::Fixed(n) => n,
        ScrollAction::Viewport(n) => (viewport_height as f32 * n as f32 * 0.01) as i16,
    };
    scroll_y = scroll_y.saturating_add_signed(offset);
    repos_y_anchored(
        scroll_y,
        total_lines,
        viewport_height,
        threshold_lines,
        anchor_line,
    )
}
