// ---------- batch %%LEVA%%: the line graph: building a plot, its series and points, and drawing it with its axes (plot.cpp) ----------

/// A line graph (`Plot`, 0x1c bytes): the series array, the value range (from and to on each axis; "to" may be below
/// "from", which turns the axis around), how many series there are and how many fit.
mod plot_fields {
    pub(crate) const SERIES: usize = 0x0;
    pub(crate) const X_FROM: usize = 0x4;
    pub(crate) const Y_FROM: usize = 0x8;
    pub(crate) const X_TO: usize = 0xc;
    pub(crate) const Y_TO: usize = 0x10;
    pub(crate) const SERIES_COUNT: usize = 0x14;
    pub(crate) const SERIES_CAPACITY: usize = 0x18;
    pub(crate) const SIZE: usize = 0x1c;
}

/// A plot series (0x12 bytes, packed): its points (x, y dwords), how many fit and how many there are, its colour
/// (word) and its label (a copy).
mod plot_series {
    pub(crate) const POINTS: usize = 0x0;
    pub(crate) const CAPACITY: usize = 0x4;
    pub(crate) const COUNT: usize = 0x8;
    pub(crate) const COLOUR: usize = 0xc;
    pub(crate) const LABEL: usize = 0xe;
    pub(crate) const SIZE: usize = 0x12;
}

/// "E:\dev\CM3\cm3 00-01\si\code\plot.cpp" (the error boxes) and "%d" (the axis labels).
const PLOT_CPP: usize = 0xa7853c;
const PLOT_NUMBER_FORMAT: usize = 0x98806c;

/// Series `s` of a plot.
unsafe fn plot_series_at(plot: usize, s: i32) -> usize {
    (read_usize(plot + plot_fields::SERIES) as isize + s as isize * plot_series::SIZE as isize) as usize
}

/// The magnitude of a value (the original's cdq/xor/sub).
fn plot_abs(v: i32) -> i32 {
    v.wrapping_abs()
}

/// A value's position on an axis drawn from `lo` to `hi` over the range `from`..`to`: `|(v − from) × (hi − lo) / |to −
/// from||` from `lo`, or from `hi` backwards when `to` is not above `from`.
fn plot_scale(v: i32, from: i32, to: i32, length: i32, lo: i32, hi: i32) -> i32 {
    let q = plot_abs(v.wrapping_sub(from).wrapping_mul(length).wrapping_div(plot_abs(to.wrapping_sub(from))));
    if to > from { q.wrapping_add(lo) } else { hi.wrapping_sub(q) }
}

/// Rule: a new plot keeps its value range and room for `capacity` series; no room asked (`capacity` < 1): no array.
///
/// `X_FROM`, `Y_FROM`, `X_TO`, `Y_TO` = the four values, `SERIES_CAPACITY` = `capacity`, `SERIES_COUNT` = 0; the
/// series array (`exe::crt_malloc`, 0x12 bytes each, not zeroed); out of memory: the box `plot.cpp 47`. Returns the
/// plot.
/// Original: `Plot::Plot` (0x7c6fc0, plot.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_plot(plot: usize, x_from: i32, y_from: i32, x_to: i32, y_to: i32, capacity: i32) -> usize {
    use plot_fields as p;
    write_i32(plot + p::X_FROM, x_from);
    write_i32(plot + p::Y_FROM, y_from);
    write_i32(plot + p::X_TO, x_to);
    write_i32(plot + p::Y_TO, y_to);
    write_i32(plot + p::SERIES_CAPACITY, capacity);
    write_u32(plot + p::SERIES_COUNT, 0);
    if capacity < 1 {
        write_usize(plot + p::SERIES, 0);
        return plot;
    }
    let series = exe::crt_malloc((capacity as u32).wrapping_mul(plot_series::SIZE as u32));
    write_usize(plot + p::SERIES, series);
    if series == 0 {
        show_error_at(PLOT_CPP, 0x2f);
    }
    plot
}

// simplification: allocates the series array: it does not run twice. Original only; the plan: the range, the capacity,
// no series yet and an array exactly when room was asked.
/// Comparer for `construct_plot`.
pub(crate) unsafe extern "thiscall" fn compare_construct_plot(plot: usize, x_from: i32, y_from: i32, x_to: i32, y_to: i32, capacity: i32) -> usize {
    use plot_fields as p;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, i32) -> usize>(%%I0%%)(plot, x_from, y_from, x_to, y_to, capacity);
    let ok = r == plot && read_i32(plot + p::X_FROM) == x_from && read_i32(plot + p::Y_FROM) == y_from && read_i32(plot + p::X_TO) == x_to
        && read_i32(plot + p::Y_TO) == y_to && read_i32(plot + p::SERIES_CAPACITY) == capacity && read_u32(plot + p::SERIES_COUNT) == 0
        && (capacity >= 1 || read_usize(plot + p::SERIES) == 0);
    SCOREBOARD.record(ok, || format!("construct_plot({}, {}, {}, {}, {}): the fields came out otherwise", x_from, y_from, x_to, y_to, capacity));
    r
}

/// Rule: a plot is freed: for every slot of the series array (all `SERIES_CAPACITY` of them) its points and its label,
/// then the array (`exe::crt_free`); nothing without an array. Defect of the original: the slots past `SERIES_COUNT`
/// were never written (the array is not zeroed), so a plot with fewer series than room frees leftover pointers.
/// Original: `Plot::~Plot` (0x7c70a0, plot.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn destroy_plot(plot: usize) {
    use plot_series as s;
    if read_usize(plot + plot_fields::SERIES) == 0 {
        return;
    }
    let mut k = 0;
    while k < read_i32(plot + plot_fields::SERIES_CAPACITY) {
        let slot = plot_series_at(plot, k);
        if read_usize(slot + s::POINTS) != 0 {
            exe::crt_free(read_usize(slot + s::POINTS));
        }
        let slot = plot_series_at(plot, k);
        if read_usize(slot + s::LABEL) != 0 {
            exe::crt_free(read_usize(slot + s::LABEL)); // tel38_8: each slot's label freed
        }
        k += 1;
    }
    exe::crt_free(read_usize(plot + plot_fields::SERIES));
}

// simplification: frees memory: it does not run twice and leaves nothing to check. Original only.
/// Comparer for `destroy_plot`.
pub(crate) unsafe extern "thiscall" fn compare_destroy_plot(plot: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(plot);
}

/// Rule: a new series with room for `capacity` points, a colour and a copy of its label; no room left in the plot
/// (`SERIES_COUNT` ≥ `SERIES_CAPACITY`): the box `plot.cpp 86` and nothing.
///
/// The points array (`exe::crt_malloc`, 8 bytes a point) goes to the next slot; with none and `capacity` > 0: the box
/// `plot.cpp 94` and nothing more. Then the slot's capacity, no points yet, the colour, the label (`_strdup` 0x952f5b)
/// and the series count goes up.
/// Original: `Plot::add_series` (0x7c7100, plot.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn plot_append_series(plot: usize, capacity: i32, colour: u32, label: usize) {
    use plot_fields as p;
    use plot_series as s;
    if read_i32(plot + p::SERIES_COUNT) >= read_i32(plot + p::SERIES_CAPACITY) {
        show_error_at(PLOT_CPP, 0x56);
        return;
    }
    let points = exe::crt_malloc((capacity as u32).wrapping_mul(8));
    write_usize(plot_series_at(plot, read_i32(plot + p::SERIES_COUNT)) + s::POINTS, points);
    let slot = plot_series_at(plot, read_i32(plot + p::SERIES_COUNT));
    if read_usize(slot + s::POINTS) == 0 && capacity > 0 {
        show_error_at(PLOT_CPP, 0x5e);
        return;
    }
    write_i32(slot + s::CAPACITY, capacity);
    write_u32(plot_series_at(plot, read_i32(plot + p::SERIES_COUNT)) + s::COUNT, 0);
    write_u16(plot_series_at(plot, read_i32(plot + p::SERIES_COUNT)) + s::COLOUR, colour as u16);
    let strdup: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x952f5busize);
    let copy = strdup(label);
    write_usize(plot_series_at(plot, read_i32(plot + p::SERIES_COUNT)) + s::LABEL, copy);
    write_i32(plot + p::SERIES_COUNT, read_i32(plot + p::SERIES_COUNT) + 1); // tel38_8: one series more
}

// simplification: allocates the points and the label copy: it does not run twice. Original only; the plan: with room,
// one series more with the capacity, no points and the colour.
/// Comparer for `plot_append_series`.
pub(crate) unsafe extern "thiscall" fn compare_plot_append_series(plot: usize, capacity: i32, colour: u32, label: usize) {
    use plot_fields as p;
    use plot_series as s;
    let n = read_i32(plot + p::SERIES_COUNT);
    let room = n < read_i32(plot + p::SERIES_CAPACITY);
    original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, usize)>(%%I2%%)(plot, capacity, colour, label);
    if !room {
        return;
    }
    let slot = plot_series_at(plot, n);
    let ok = read_usize(slot + s::POINTS) == 0
        || (read_i32(plot + p::SERIES_COUNT) == n + 1 && read_i32(slot + s::CAPACITY) == capacity && read_i32(slot + s::COUNT) == 0
            && read_u16(slot + s::COLOUR) == colour as u16);
    SCOREBOARD.record(ok, || format!("plot_append_series({}, colour {:#x}): series {} came out otherwise", capacity, colour, n));
}

/// Rule: a point (x, y) is appended to series `series`; a series past the plot's room (`SERIES_CAPACITY`, not the
/// series count) gives the box `plot.cpp 117`, a full series the box `plot.cpp 123`, and nothing is added.
/// Original: `Plot::add_point` (0x7c7240, plot.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn plot_append_point(plot: usize, series: i32, x: i32, y: i32) {
    use plot_series as s;
    if series >= read_i32(plot + plot_fields::SERIES_CAPACITY) {
        show_error_at(PLOT_CPP, 0x75);
        return;
    }
    let slot = plot_series_at(plot, series);
    let n = read_i32(slot + s::COUNT);
    if n >= read_i32(slot + s::CAPACITY) {
        show_error_at(PLOT_CPP, 0x7b);
        return;
    }
    let point = read_usize(slot + s::POINTS) + n as usize * 8;
    write_i32(point, x);
    write_i32(point + 4, y); // tel38_8: the point's value
    write_i32(slot + s::COUNT, n + 1);
}

/// Comparer for `plot_append_point`: side by side over the series and its next point (the error paths only through the
/// original: the box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_plot_append_point(plot: usize, series: i32, x: i32, y: i32) {
    use plot_series as s;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32)>(%%I3%%);
    if series >= read_i32(plot + plot_fields::SERIES_CAPACITY) {
        return o(plot, series, x, y);
    }
    let slot = plot_series_at(plot, series);
    let n = read_i32(slot + s::COUNT);
    if n >= read_i32(slot + s::CAPACITY) {
        return o(plot, series, x, y);
    }
    let regions = [(slot, s::SIZE), (read_usize(slot + s::POINTS) + n as usize * 8, 8)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(plot, series, x, y), || plot_append_point(plot, series, x, y));
    SCOREBOARD.record(equal, || format!("plot_append_point(series {}, {}, {}): differences in {:?}", series, x, y, first_differences(&mo, &mr)));
}

/// `Display::add_item` on the screen manager's current slot: a line (type 1, style 0x400) from (x1, y1) to (x2, y2)
/// in `colour`, no area.
unsafe fn plot_line(x1: i32, y1: i32, x2: i32, y2: i32, colour: u32) {
    let add: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    add(screens_current_display(SCREEN_MANAGER), 1, x1 as u32, y1 as u32, x2 as u32, y2 as u32, 0, 0, 0x400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX);
}

/// `ScreenManager::add_text_item(x1, y1, x2, y2, 0, colour, font, 1, colour, text, 1)` with `G_GUI_COLOUR_PALE_GREY`.
unsafe fn plot_label(x1: i32, y1: i32, x2: i32, y2: i32, font: u32, text: usize) {
    let add: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, u32, u32, u32, u32, u32, usize, u32) -> u16 =
        core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let colour = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    add(SCREEN_MANAGER, x1, y1, x2, y2, 0, colour, font, 1, colour, text, 1);
}

/// Rule: a graph's frame: an area over the box, its four sides as double lines, the two titles and the numbered ticks
/// of both axes; with `pad` = 1 each range is one wider while drawn.
///
/// With `pad`: `X_TO` + 1 and the larger of `Y_FROM`/`Y_TO` + 1 for the area, undone after it. The area
/// (`Display::add_area`: the box, |x range| columns, |y range| rows, style 0x80000, no owner). The sides, each a line
/// and a second one a pixel outside (`plot_line`): left and top in `G_GUI_COLOUR_BLUE`, bottom and right in
/// `G_GUI_COLOUR_BLACK`. The x title under the box ((x0, y1 + 20)–(x1, y1 + 30), font 0xc), the y title over it ((x0,
/// y0 − 20)–(x1, y0), font 1). The x step: the smallest from 1 that puts the ticks more than 15 pixels apart (at most
/// the range); each x value from `X_FROM` to `X_TO` by the step gets its number ("%d") under the box: with `pad`
/// between its position and the next one's, otherwise 20 pixels wide around it (`plot_scale`). The y step: the
/// smallest that puts the ticks more than 10 pixels apart; each y value from the lower to the higher end gets its
/// number left of the box, 20 pixels wide (with `pad` between its position and the next one's, the higher end + 1
/// while computed). A range of 0 divides by zero, as in the original.
/// Original: `Plot::draw_axes` (0x7c7580, plot.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn plot_draw_axes(plot: usize, x0: i32, y0: i32, x1: i32, y1: i32, pad: i32, x_title: usize, y_title: usize) {
    use plot_fields as p;
    let get = |f: usize| read_i32(plot + f);
    let set = |f: usize, v: i32| write_i32(plot + f, v);
    let widen = |by: i32| {
        set(p::X_TO, get(p::X_TO) + by);
        let (yf, yt) = (get(p::Y_FROM), get(p::Y_TO));
        if yf.max(yt) == yt { set(p::Y_TO, yt + by) } else { set(p::Y_FROM, yf + by) }
    };
    if pad == 1 {
        widen(1);
    }
    let add_area: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    add_area(screens_current_display(SCREEN_MANAGER), x0 as u32, y0 as u32, x1 as u32, y1 as u32, plot_abs(get(p::X_TO) - get(p::X_FROM)) as u32, 0,
        plot_abs(get(p::Y_TO) - get(p::Y_FROM)) as u32, 0, 0x80000, 0, u32::MAX);
    if pad == 1 {
        widen(-1);
    }
    let (blue, black) = (read_u16(G_GUI_COLOUR_BLUE) as u32, read_u16(G_GUI_COLOUR_BLACK) as u32);
    plot_line(x0, y0, x0, y1, blue);
    plot_line(x0 - 1, y0 - 1, x0 - 1, y1 + 1, blue);
    plot_line(x0, y1, x1, y1, black);
    plot_line(x0 - 1, y1 + 1, x1 + 1, y1 + 1, black);
    plot_line(x1, y0, x1, y1, black);
    plot_line(x1 + 1, y0 - 1, x1 + 1, y1 + 1, black);
    plot_line(x0, y0, x1, y0, blue); // tel38_8: the top side
    plot_line(x0 - 1, y0 - 1, x1 + 1, y0 - 1, blue);
    plot_label(x0, y1 + 0x14, x1, y1 + 0x1e, 0xc, x_title);
    plot_label(x0, y0 - 0x14, x1, y0, 1, y_title);
    let mut number = [0u8; 20];
    let format: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // the x ticks
    let width = x1 - x0;
    let range = get(p::X_TO) - get(p::X_FROM);
    let mut step = 1;
    if range > 1 {
        let mut gap = width;
        loop {
            if gap.wrapping_div(plot_abs(range)) > 0xf {
                break;
            }
            step += 1;
            gap = gap.wrapping_add(width);
            if step >= range {
                break;
            }
        }
    }
    let mut v = get(p::X_FROM);
    while v <= get(p::X_TO) {
        format(number.as_mut_ptr(), PLOT_NUMBER_FORMAT, v);
        let text = number.as_ptr() as usize;
        let (from, length) = (get(p::X_FROM), plot_abs(x1 - x0));
        if pad == 1 {
            let to = get(p::X_TO) + 1;
            let left = plot_scale(v, from, to, length, x0, x1);
            let right = plot_scale(v + 1, from, to, length, x0, x1);
            plot_label(left, y1, right, y1 + 0x14, 0xc, text);
        } else {
            let centre = plot_scale(v, from, get(p::X_TO), length, x0, x1);
            plot_label(centre - 10, y1, centre + 10, y1 + 0x14, 0xc, text); // tel38_8: an x number
        }
        v += step;
    }
    // the y ticks
    let range = get(p::Y_TO) - get(p::Y_FROM);
    let span = plot_abs(range);
    let mut step = 1;
    if span > 1 {
        loop {
            let q = if pad == 1 {
                (y0 - y1).wrapping_mul(step).wrapping_div(plot_abs(range + 1))
            } else {
                (y0 - y1).wrapping_mul(step).wrapping_div(span)
            };
            if plot_abs(q) > 10 {
                break;
            }
            step += 1;
            if step >= span {
                break;
            }
        }
    }
    let mut v = get(p::Y_FROM).min(get(p::Y_TO));
    while v <= get(p::Y_FROM).max(get(p::Y_TO)) {
        format(number.as_mut_ptr(), PLOT_NUMBER_FORMAT, v);
        let text = number.as_ptr() as usize;
        let length = plot_abs(y1 - y0);
        if pad == 1 {
            let (top, bottom);
            let (yf, yt) = (get(p::Y_FROM), get(p::Y_TO));
            if yt > yf {
                set(p::Y_TO, yt + 1);
                top = plot_scale(v, yf, yt + 1, length, y0, y1);
                bottom = plot_scale(v + 1, yf, yt + 1, length, y0, y1);
                set(p::Y_TO, yt);
            } else {
                set(p::Y_FROM, yf + 1);
                top = plot_scale(v, yf + 1, yt, length, y0, y1);
                bottom = plot_scale(v + 1, yf + 1, yt, length, y0, y1);
                set(p::Y_FROM, yf);
            }
            plot_label(x0 - 0x14, top, x0, bottom, 0xc, text);
        } else {
            let centre = plot_scale(v, get(p::Y_FROM), get(p::Y_TO), length, y0, y1);
            plot_label(x0 - 0x14, centre - 10, x0, centre + 10, 0xc, text);
        }
        v += plot_abs(step);
    }
}

/// The display-list writes of a plot drawing: the gui builder's (`gui_build_regions` of tel38_3, one area), the plot
/// itself (its range moves while drawn) and the next `items` item slots and free-order slots (clamped to the tables),
/// with the network packet up to 0x4000 bytes.
unsafe fn plot_regions(plot: usize, items: usize) -> Vec<(usize, usize)> {
    use display_list as l;
    let mut r = gui_build_regions(1, 0);
    r.push((plot, plot_fields::SIZE));
    let (base, pos, limit) = (read_usize(0xae2908), read_usize(0xae290c), read_usize(0xae2910));
    if base != 0 && pos < limit {
        r.push((base + pos, (limit - pos).min(0x4000)));
    }
    let d = screens_current_display(SCREEN_MANAGER);
    if d != 0 {
        let ic = read_i16(d + l::ITEM_COUNT).clamp(0, 0x4af) as usize;
        r.push((display_item_at(d, ic as i16), items.min(0x4af - ic) * l::ITEM_SIZE));
        let oc = read_i16(d + l::ITEM_ORDER_COUNT).clamp(0, 0x4b0) as usize;
        r.push((d + l::ITEM_ORDER + oc * 2, items.min(0x4b0 - oc) * 2));
    }
    r
}

/// How many items a frame can add at most: 8 sides, 2 titles and a number per value of each range (step 1).
unsafe fn plot_axes_items(plot: usize) -> usize {
    use plot_fields as p;
    let x = plot_abs(read_i32(plot + p::X_TO) - read_i32(plot + p::X_FROM)) as usize;
    let y = plot_abs(read_i32(plot + p::Y_TO) - read_i32(plot + p::Y_FROM)) as usize;
    10 + x + 2 + y + 2
}

/// Comparer for `plot_draw_axes`: side by side over `plot_regions`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_plot_draw_axes(plot: usize, x0: i32, y0: i32, x1: i32, y1: i32, pad: i32, x_title: usize, y_title: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, i32, usize, usize)>(%%I5%%);
    let regions = plot_regions(plot, plot_axes_items(plot));
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(plot, x0, y0, x1, y1, pad, x_title, y_title),
        || plot_draw_axes(plot, x0, y0, x1, y1, pad, x_title, y_title));
    SCOREBOARD.record(equal, || format!("plot_draw_axes(({}, {})-({}, {}), pad {}): differences in {:?}", x0, y0, x1, y1, pad, first_differences(&mo, &mr)));
}

/// Rule: draws a line graph in the box (x0, y0)–(x1, y1): the frame and each series as a line through its points.
///
/// The plot area: left = the smaller x + 20, top = the smaller y + 20, right = the larger x, bottom = the larger y −
/// 30; the frame (`Plot::draw_axes` with no padding and the two titles). Each series with points: each point is scaled
/// into the area (`plot_scale`, x over the width from the left, y over the height from the top, an axis turned around
/// when "to" is not above "from"); a line from the previous point to it and a second one a pixel higher, in the
/// series' colour (`plot_line`); the first point of a series is its own previous point (two lines of no length).
/// Original: `Plot::draw_line_graph` (0x7c7340, plot.cpp, thiscall, ret 0x18).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn plot_draw_line_graph(plot: usize, x0: i32, y0: i32, x1: i32, y1: i32, x_title: usize, y_title: usize) {
    use plot_fields as p;
    use plot_series as s;
    let (left, right) = (x0.min(x1) + 0x14, x0.max(x1));
    let (top, bottom) = (y0.min(y1) + 0x14, y0.max(y1) - 0x1e);
    let axes: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, i32, usize, usize) = core::mem::transmute(0x7c7580usize);
    axes(plot, left, top, right, bottom, 0, x_title, y_title);
    let (width, height) = (right - left, bottom - top);
    // simplification: the original's previous point starts as a leftover local; the first point always sets it
    let (mut px, mut py) = (0i32, 0i32);
    let mut k = 0;
    while k < read_i32(plot + p::SERIES_COUNT) {
        let mut j = 0;
        while j < read_i32(plot_series_at(plot, k) + s::COUNT) {
            let point = read_usize(plot_series_at(plot, k) + s::POINTS) + j as usize * 8;
            let x = plot_scale(read_i32(point), read_i32(plot + p::X_FROM), read_i32(plot + p::X_TO), width, left, right);
            let y = plot_scale(read_i32(point + 4), read_i32(plot + p::Y_FROM), read_i32(plot + p::Y_TO), height, top, bottom);
            if j == 0 {
                px = x;
                py = y;
            }
            let colour = read_u16(plot_series_at(plot, k) + s::COLOUR) as u32;
            plot_line(px, py, x, y, colour);
            plot_line(px, py - 1, x, y - 1, colour); // tel38_8: the line a pixel thicker
            px = x;
            py = y;
            j += 1;
        }
        k += 1;
    }
}

/// Comparer for `plot_draw_line_graph`: side by side over `plot_regions` (the frame's items and two a point).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_plot_draw_line_graph(plot: usize, x0: i32, y0: i32, x1: i32, y1: i32, x_title: usize, y_title: usize) {
    use plot_series as s;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, usize, usize)>(%%I4%%);
    let mut points = 0usize;
    for k in 0..read_i32(plot + plot_fields::SERIES_COUNT).max(0) {
        points += read_i32(plot_series_at(plot, k) + s::COUNT).max(0) as usize;
    }
    let regions = plot_regions(plot, plot_axes_items(plot) + 2 * points);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(plot, x0, y0, x1, y1, x_title, y_title),
        || plot_draw_line_graph(plot, x0, y0, x1, y1, x_title, y_title));
    SCOREBOARD.record(equal, || format!("plot_draw_line_graph(({}, {})-({}, {})): differences in {:?}", x0, y0, x1, y1, first_differences(&mo, &mr)));
}
