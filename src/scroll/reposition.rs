pub fn repos_y(scroll_y: u16, total_lines: usize, viewport_height: u16) -> u16 {
    let viewport_height = viewport_height as usize;
    if viewport_height == 0 {
        return 0;
    }

    if total_lines <= viewport_height {
        return 0;
    }

    let scroll_y = scroll_y as usize;
    let max_scroll = total_lines - viewport_height;
    scroll_y.min(max_scroll) as u16
}

pub fn repos_y_anchored(
    scroll_y: u16,
    total_lines: usize,
    viewport_height: u16,
    threshold_lines: u16,
    anchor_line: usize,
) -> u16 {
    let viewport_lines = viewport_height as usize;
    if viewport_lines == 0 {
        return 0;
    }
    let threshold_lines = usize::from(threshold_lines).min(viewport_lines.saturating_sub(1));

    if total_lines <= viewport_lines {
        return 0;
    }

    let mut scroll_y = scroll_y as usize;
    if anchor_line < scroll_y.saturating_add(threshold_lines) {
        /*
         * before: anchor_line < scroll_y + threshold_lines
         *
         *  | scroll_y    | }
         *  |             | }
         *  | anchor_line | }
         *  |             |_}-threshold_lines
         *  |             |
         *  |             |
         *
         * after: anchor_line == new_scroll_y + threshold_lines
         *
         *  | anchor_line - threshold_lines | }
         *  |                               | }
         *  |                               | }
         *  |                               |_}-threshold_lines
         *  |        anchor_line            |
         *  |                               |
         *  |                               |
         *
         * */
        scroll_y = anchor_line.saturating_sub(threshold_lines);
    }
    if anchor_line
        > scroll_y
            + viewport_lines
                .saturating_sub(1)
                .saturating_sub(threshold_lines)
    {
        /*
         * before: anchor_line > scroll_y + viewport_lines - 1 - threshold_lines
         *
         *                { |          scroll_y             |
         *                { |                               |
         *                { |                               |
         *                { |                               |
         *                { |                               |
         *                { |                               | }
         *                { |                               | }
         *                { |         anchor_line           | }
         * viewport_lines-{_| scroll_y + viewport_lines - 1 |_}-threshold_lines
         *
         * after: new_scroll_y = anchor_line - (viewport_lines - 1 - threshold_lines)
         *
         *                { |  scroll_y   |
         *                { |             |
         *                { |             |
         *                { |             |
         *                { | anchor_line |---(viewport_lines - 1 - threshold_lines)
         *                { |             | }
         *                { |             | }
         *                { |             | }
         * viewport_lines-{_|             |_}-threshold_lines
         *
         * */
        scroll_y = anchor_line.saturating_sub(
            viewport_lines
                .saturating_sub(1)
                .saturating_sub(threshold_lines),
        );
    }

    /*
     * before:
     *
     *                    "0"
     *                { | "1" [scroll_y]    |
     *                { | "2"               | }
     * viewport_lines-{_| "3" [anchor_line] |_}-threshold_lines
     *                    "4"
     *
     * scroll: occur empty area
     *
     *                    "0"
     *                    "1"
     *                    "2"
     *                { | "3" [scroll_y / anchor_line] |
     *                { | "4"                          | }
     * viewport_lines-{_|                              |_}-threshold_lines
     *
     *
     * after: `total_lines` truncate `viewport_lines`
     *
     *                    "0"
     *                    "1"
     *                { | "2" [scroll_y]    |
     *                { | "3" [anchor_line] | }
     * viewport_lines-{_| "4"               |_}-threshold_lines
     *
     * */
    let max_scroll = total_lines - viewport_lines;
    scroll_y = scroll_y.min(max_scroll);

    scroll_y as u16
}
