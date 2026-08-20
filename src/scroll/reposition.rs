pub fn repos_y(scroll_y: u16, height: usize, viewport_height: u16) -> u16 {
    let viewport_height = viewport_height as usize;
    if viewport_height == 0 {
        return 0;
    }

    let max_scroll = height.saturating_sub(viewport_height);
    scroll_y.min(max_scroll as u16)
}

pub fn repos_y_anchored(
    scroll_y: u16,
    height: usize,
    viewport_height: u16,
    threshold_lines: u16,
    anchor_line: usize,
) -> u16 {
    let viewport_height = viewport_height as usize;
    if viewport_height == 0 {
        return 0;
    }
    let threshold_lines = usize::from(threshold_lines).min(viewport_height.saturating_sub(1));

    if height <= viewport_height {
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
            + viewport_height
                .saturating_sub(1)
                .saturating_sub(threshold_lines)
    {
        /*
         * before: anchor_line > scroll_y + viewport_height - 1 - threshold_lines
         *
         *                 { |          scroll_y              |
         *                 { |                                |
         *                 { |                                |
         *                 { |                                |
         *                 { |                                |
         *                 { |                                | }
         *                 { |                                | }
         *                 { |         anchor_line            | }
         * viewport_height-{_| scroll_y + viewport_height - 1 |_}-threshold_lines
         *
         * after: new_scroll_y = anchor_line - (viewport_height - 1 - threshold_lines)
         *
         *                 { |  scroll_y   |
         *                 { |             |
         *                 { |             |
         *                 { |             |
         *                 { | anchor_line |---(viewport_height - 1 - threshold_lines)
         *                 { |             | }
         *                 { |             | }
         *                 { |             | }
         * viewport_height-{_|             |_}-threshold_lines
         *
         * */
        scroll_y = anchor_line.saturating_sub(
            viewport_height
                .saturating_sub(1)
                .saturating_sub(threshold_lines),
        );
    }

    /*
     * before:
     *
     *                     "0"
     *                 { | "1" [scroll_y]    |
     *                 { | "2"               | }
     * viewport_height-{_| "3" [anchor_line] |_}-threshold_lines
     *                     "4"
     *
     * scroll: occur empty area
     *
     *                     "0"
     *                     "1"
     *                     "2"
     *                 { | "3" [scroll_y / anchor_line] |
     *                 { | "4"                          | }
     * viewport_height-{_|                              |_}-threshold_lines
     *
     *
     * after: `height` truncate `viewport_height`
     *
     *                     "0"
     *                     "1"
     *                 { | "2" [scroll_y]    |
     *                 { | "3" [anchor_line] | }
     * viewport_height-{_| "4"               |_}-threshold_lines
     *
     * */
    let max_scroll = height - viewport_height;
    scroll_y = scroll_y.min(max_scroll);

    scroll_y as u16
}

pub fn repos_x(scroll_x: u16, width: usize, viewport_width: u16) -> u16 {
    let viewport_width = viewport_width as usize;
    if viewport_width == 0 {
        return 0;
    }

    let max_scroll = width.saturating_sub(viewport_width);
    scroll_x.min(max_scroll as u16)
}

pub fn repos_x_anchored(
    scroll_x: u16,
    width: usize,
    viewport_width: u16,
    threshold_columns: u16,
    anchor_column: usize,
) -> u16 {
    let viewport_columns = viewport_width as usize;
    if viewport_columns == 0 {
        return 0;
    }
    let threshold_columns = usize::from(threshold_columns).min(viewport_columns.saturating_sub(1));

    if width <= viewport_columns {
        return 0;
    }

    let mut scroll_x = scroll_x as usize;
    if anchor_column < scroll_x.saturating_add(threshold_columns) {
        scroll_x = anchor_column.saturating_sub(threshold_columns);
    }
    if anchor_column
        > scroll_x
            + viewport_columns
                .saturating_sub(1)
                .saturating_sub(threshold_columns)
    {
        scroll_x = anchor_column.saturating_sub(
            viewport_columns
                .saturating_sub(1)
                .saturating_sub(threshold_columns),
        );
    }

    let max_scroll = width - viewport_columns;
    scroll_x = scroll_x.min(max_scroll);

    scroll_x as u16
}
