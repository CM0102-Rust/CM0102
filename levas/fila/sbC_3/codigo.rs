// ---------- sbC_3: the graphics base: the progress box, the circle and its points (goldcup.cpp by the map; the graphics library; cm0102.exe only) ----------

/// The screen size the circle routines assume (cm0102.exe; the 1080 exe has its own copies with 1920 × 1080, so these
/// three routines are detoured in cm0102.exe only).
const CIRCLE_SCREEN_WIDTH: i32 = 800;
const CIRCLE_SCREEN_HEIGHT: i32 = 600;
/// A row span of a filled circle: the leftmost and rightmost x plotted on the row.
const SPAN_SIZE: usize = 8;

/// Draws the bottom progress box: the frame, the text and, when 0 < `total` and `step` ≤ `total`, a bar of `step` ÷
/// `total` of its width; returns 1, or 0 when the display is off or without a text.
///
/// The messages are pumped first (`exe::pump_messages`). With step 0 the current screen is darkened once
/// (`Display::clear_screen_once` of the current human's Display). The box is the panel (100, 555)-(790, 590) (flags
/// 0x30, grey). With a bar: the text at the left, (120, 565)-(250, 580) (centred, font 1, pale grey), and the bar's
/// trough (270, 565)-(770, 580) (flags 0x60, grey) filled from x 272 to 272 + step × 496 ÷ total, y 567 to 578, in dark
/// blue (`graphics_draw_rect`, style 4); otherwise the text over the whole width, (120, 565)-(770, 580). The box is then
/// flipped to the screen.
/// Original: `gui_draw_progress_box` (0x5e76f0, goldcup.cpp, cdecl; cm0102.exe: the 1080 exe calls moved copies).
pub(crate) unsafe extern "C" fn gui_draw_progress_box(text: usize, step: i32, total: i32) -> u32 {
    if read_u32(DISPLAY_OFF) != 0 {
        return 0;
    }
    exe::pump_messages();
    if text == 0 {
        return 0;
    }
    if step == 0 {
        display_clear_screen_once(current_human_display());
    }
    exe::gui_draw_panel(0x64, 0x22b, 0x316, 0x24e, 0x30, read_u16(G_GUI_COLOUR_GREY) as u32, 0);
    let pale_grey = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    if total > 0 && step <= total {
        covered!("sbC_3 progress bar");
        exe::gui_draw_text_box(0x78, 0x235, 0xfa, 0x244, 0xc, 1, pale_grey, text, -1);
        exe::gui_draw_panel(0x10e, 0x235, 0x302, 0x244, 0x60, read_u16(G_GUI_COLOUR_GREY) as u32, 0);
        let end = step.wrapping_mul(0x1f0) / total + 0x110;
        exe::graphics_draw_rect(0x110, 0x237, end, 0x242, 4, read_u16(G_GUI_COLOUR_DARK_BLUE) as u32);
    } else {
        exe::gui_draw_text_box(0x78, 0x235, 0x302, 0x244, 0xc, 1, pale_grey, text, -1);
    }
    exe::graphics_flip_rect(0x64, 0x22b, 0x316, 0x24e);
    1
}

/// Plots the symmetric points of one step of a circle centred in the box (`left`, `top`)-(`right`, `bottom`) on the
/// locked screen `pixels` (a fixed pitch of 800 pixels), and widens the row spans of a filled circle (`spans`, entry
/// y − top + 1).
///
/// The centre is the box's middle ((right − left + 1) ÷ 2 + left, the same down). For k from `dx` while k < dx + 1 (as a
/// 16-bit number) the points (cx ± k, cy ± dy), then for m from `dy` while m < dy + 1 the points (cx ± m, cy ± dx); each
/// only when it lies within one pixel around the box, where it gets `colour` and widens its row's span.
/// Defect of the original: the screen pitch is taken as 800 pixels, not the surface's.
/// Original: `circle_plot_points` (0x5e7aa0, goldcup.cpp, cdecl; cm0102.exe: the 1080 exe has a 1920 pitch).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn circle_plot_points(pixels: usize, dx: i32, dy: i32, left: i32, top: i32, right: i32, bottom: i32, colour: u32, spans: usize) {
    let cx = (right - left + 1) / 2 + left;
    let cy = (bottom - top + 1) / 2 + top;
    // (x, y) checked against the box and the spans; the pixel written at column `column` of row y
    let point = |x: i32, y: i32, column: i32| {
        if y < top - 1 || y > bottom + 1 || x < left - 1 || x > right + 1 {
            return;
        }
        if spans != 0 {
            let span = spans.wrapping_add((y - top + 1) as isize as usize * SPAN_SIZE);
            if x < read_i32(span) {
                write_i32(span, x);
            }
            if x > read_i32(span + 4) {
                write_i32(span + 4, x);
            }
        }
        write_u16(pixels.wrapping_add((CIRCLE_SCREEN_WIDTH.wrapping_mul(y).wrapping_add(column) as isize as usize).wrapping_mul(2)), colour as u16);
    };
    for (a, b) in [(dx, dy), (dy, dx)] {
        let mut k = a;
        let (mut right_x, mut left_x) = (a + cx, cx - a);
        while k < a.wrapping_add(1) as i16 as i32 {
            point(right_x, cy + b, k + cx);
            point(right_x, cy - b, k + cx);
            point(left_x, cy - b, cx - k);
            point(left_x, cy + b, cx - k);
            k += 1;
            right_x += 1;
            left_x -= 1;
        }
    }
}

/// Draws a circle inside the box (x1, y1)-(x2, y2) (its inner box one pixel in), solid with bit 1 of `style`, dashed
/// otherwise (4 steps on, 2 off), filled with bit 2.
///
/// A filled circle gets its row spans (operator new, rows + 3 entries starting at (800, 0)). The whole screen (0, 0,
/// 800, 600) is locked (0x21; failing: nothing more, the spans not freed) and the midpoint walk plots each step
/// (`circle_plot_points`): radius r = (rows + 1) ÷ 2, decision 3 − 2r (16-bit), x from 0 while x < y, the decision
/// growing by 4x + 6 or, when not negative, by 4(x − y) + 10 with y down by one (each step 16-bit); then the diagonal
/// point (y, y) when y is not 0. After the unlock each span with a right end becomes a solid line
/// (`graphics_draw_line`) and the spans are freed.
/// Defect of the original: no check of the display or the back buffer before the lock.
/// Original: `gui_draw_circle` (0x5e7840, goldcup.cpp, cdecl; cm0102.exe: the 1080 exe locks 1920 × 1080).
pub(crate) unsafe extern "C" fn gui_draw_circle(x1: i32, y1: i32, x2: i32, y2: i32, style: u32, colour: u32) {
    let (left, top, right, bottom) = (x1 + 1, y1 + 1, x2 - 1, y2 - 1);
    let filled = style & 4 != 0;
    let rows = bottom.wrapping_sub(top);
    let spans = if filled { exe::crt_operator_new(rows.wrapping_mul(SPAN_SIZE as i32).wrapping_add(0x18) as u32) } else { 0 };
    if spans != 0 {
        for k in 0..rows.wrapping_add(3).max(0) as usize {
            write_i32(spans + k * SPAN_SIZE, CIRCLE_SCREEN_WIDTH);
            write_i32(spans + k * SPAN_SIZE + 4, 0);
        }
    }
    let Some((surface, _)) = lock_back_buffer_pixels(&[0, 0, CIRCLE_SCREEN_WIDTH, CIRCLE_SCREEN_HEIGHT], 0x21) else { return };
    let visible = |steps: i32| filled || style & 2 != 0 || steps % 6 < 4;
    let (mut x, mut y, mut steps) = (0i32, (rows + 1) / 2, 0i32);
    let mut decision = (3 - 2 * y) as i16 as i32;
    if y > 0 {
        loop {
            if visible(steps) {
                circle_plot_points(surface, x, y, left, top, right, bottom, colour, spans);
            }
            if decision < 0 {
                decision += (4 * x + 6) as i16 as i32;
            } else {
                decision += (4 * (x - y) + 10) as i16 as i32;
                y -= 1;
            }
            x += 1;
            steps += 1;
            if x >= y {
                break;
            }
        }
    }
    if y != 0 && visible(steps) {
        circle_plot_points(surface, y, y, left, top, right, bottom, colour, spans);
    }
    unlock_back_buffer();
    if spans != 0 {
        covered!("sbC_3 filled circle");
        for k in 0..rows.wrapping_add(3).max(0) {
            let span = spans + k as usize * SPAN_SIZE;
            let end = read_i32(span + 4);
            if end != 0 {
                exe::graphics_draw_line(read_i32(span), top - 1 + k, end, top - 1 + k, 2, colour);
            }
        }
        exe::crt_operator_delete(spans);
    }
}

/// Comparers: the progress box only the original (it pumps the messages and flips to the screen); the circle side by
/// side on the back buffer (`compare_drawing_in_box`); its points only the original (they write a locked surface).
pub(crate) unsafe extern "C" fn compare_gui_draw_progress_box(text: usize, step: i32, total: i32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, i32, i32) -> u32>(%%I0%%)(text, step, total)
}

pub(crate) unsafe extern "C" fn compare_gui_draw_circle(x1: i32, y1: i32, x2: i32, y2: i32, style: u32, colour: u32) {
    compare_drawing_in_box("gui_draw_circle", [x1, y1, x2, y2],
        || original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, u32, u32)>(%%I1%%)(x1, y1, x2, y2, style, colour),
        || gui_draw_circle(x1, y1, x2, y2, style, colour));
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_circle_plot_points(pixels: usize, dx: i32, dy: i32, left: i32, top: i32, right: i32, bottom: i32, colour: u32, spans: usize) {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, usize)>(%%I2%%)(pixels, dx, dy, left, top, right, bottom, colour, spans)
}
