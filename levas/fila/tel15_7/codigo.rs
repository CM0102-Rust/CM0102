// ---------- batch %%LEVA%%: the tactics screen (tactics_screens.cpp, 6): the leave-the-match text and the pitch drawing ----------

/// The on (`SUBS_ON`) and off (`SUBS_OFF`) rows of the substitutions queued on the screen (i8[9] each, `SUBS_MADE`
/// of them).
#[allow(dead_code)]
mod tactics_subs_fields {
    pub(crate) const SUBS_ON: usize = 0x4a0d;
    pub(crate) const SUBS_OFF: usize = 0x4a16;
}

/// "\n\n", the paragraph break of the screen messages.
const TACTICS_PARAGRAPH: usize = 0x9ed918;

/// `Text::clear` (0x4afa40, thiscall).
unsafe fn tactics_text_clear(text: usize) {
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize);
    clear(text);
}

/// Appends the screen text (`JOB_TEXT`) to `message` as a paragraph: "\n\n" first when the message is not empty
/// (the inline `strcat`s of the original).
unsafe fn tactics_append_paragraph(message: *mut u8) {
    let append = |src: *const u8| {
        let mut end = 0;
        while *message.add(end) != 0 {
            end += 1;
        }
        tel26_copy_text(message.add(end), src);
    };
    if *message != 0 {
        append(TACTICS_PARAGRAPH as *const u8);
    }
    append(JOB_TEXT as *const u8);
}

/// The link of a player in a screen text: the national team kind (0x7dc) when the screen's club is a national team,
/// else the club kind (0x7d7).
unsafe fn tactics_player_link(text: usize, club: usize, name: *const u8, staff: usize) {
    let kind = if exe::database_is_national_team(club) != 0 { 0x7dc } else { 0x7d7 };
    exe::text_add_link(text, name, kind, staff);
}

/// The shirt number shown for a substitution row: the squad number (its absolute value) when the first team's
/// club uses squad numbers, else the row + 1.
unsafe fn tactics_substitution_shirt(ts: usize, row: i8, mp: usize) -> i8 {
    let club = read_usize(ts + tactics_screen::CLUB);
    let squad = read_usize(G_SQUAD_MANAGER);
    if read_u32(ts + tactics_screen_state::RESERVE) == 0 && exe::squad_manager_club_uses_squad_numbers(squad, club, 0) != 0 {
        let n = exe::squad_manager_get_squad_number(squad, read_usize(mp + match_player::STAFF), club);
        return (n as i32).wrapping_abs() as u8 as i8;
    }
    (row as u8).wrapping_add(1) as i8
}

/// Builds the confirmation text for leaving the tactics screen back to the live match, and says whether it is needed.
///
/// The Text is cleared; outside a live match nothing more (0). The message, paragraph by paragraph: more than 3
/// substitutions made gives "You have made <number in words> substitutions." (line 0xa8c6a8); 1 to 3 give one
/// "<shirt> <player> has been brought on to replace <shirt> <player>." each (the shirt is the squad number when the
/// first team's club uses them, else the row + 1; both players linked). When the captain (the row whose player has
/// the `CAPTAIN` id) is not on the pitch, the player on the pitch with the highest captain score
/// (`match_man_get_captain_score`; the first on a tie) is named: "You have not selected a captain. <player> will be
/// selected as captain." (linked). The players of the match squad (the rows within the bench size of a match record)
/// neither on the pitch nor on the bench who are among the 10 set piece takers (the taker dword compared with the
/// player record's address, as the original does) are listed once each ("A", "A and B", "A, B, and C"; linked) in
/// "<list> have been specified as set piece takers, but are no longer on the pitch." (one: "... has been specified as
/// a set piece taker ..."). A non-empty message on a screen with `FIXTURE` set gets "Return to the match ?" and is
/// set into the Text: 1. Otherwise 0 (the links stay in the Text).
/// Original: `Tactics_Screen::build_return_to_match_text` (0x8a8630, tactics_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn build_return_to_match_text(ts: usize, text: usize) -> u32 {
    use tactics_screen as t;
    use tactics_subs_fields as f;
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    tactics_text_clear(text);
    if read_usize(ts + t::MATCH) == 0 {
        return 0;
    }
    let club = read_usize(ts + t::CLUB);
    let mut message = vec![0u8; 0x2700];
    let msg = message.as_mut_ptr();
    let (mut name, mut name2) = ([0u8; 0xc8], [0u8; 0xc8]);
    let made = read_i8(ts + t::SUBS_MADE);
    if made > 3 {
        covered!("tel15_7 return text: the substitutions count");
        exe::utils_number_to_words(made as i32, name.as_mut_ptr(), 0xc8, 1);
        sprintf(JOB_TEXT, 0xa8c6a8, name.as_ptr());
        tactics_append_paragraph(msg);
    } else if made > 0 {
        covered!("tel15_7 return text: each substitution");
        for k in 0..made as usize {
            let on = read_i8(ts + f::SUBS_ON + k);
            let mp_on = tactics_row_match_player(ts, on, false);
            let shirt_on = tactics_substitution_shirt(ts, on, mp_on);
            exe::database_format_staff_name(read_usize(mp_on + match_player::STAFF), name.as_mut_ptr() as usize, 0xc8, 1);
            tactics_player_link(text, club, name.as_ptr(), read_usize(mp_on + match_player::STAFF));
            let off = read_i8(ts + f::SUBS_OFF + k);
            let mp_off = tactics_row_match_player(ts, off, false);
            let shirt_off = tactics_substitution_shirt(ts, off, mp_off);
            exe::database_format_staff_name(read_usize(mp_off + match_player::STAFF), name2.as_mut_ptr() as usize, 0xc8, 1);
            tactics_player_link(text, club, name2.as_ptr(), read_usize(mp_off + match_player::STAFF));
            sprintf(JOB_TEXT, 0xa8c64c, shirt_on as i32, name.as_ptr(), shirt_off as i32, name2.as_ptr());
            tactics_append_paragraph(msg);
        }
    }
    let captain = (0..0x14i8).find(|&r| {
        let p = tactics_calls::slot_player(ts, r);
        p != 0 && read_u32(p) == read_u32(ts + t::CAPTAIN)
    }).unwrap_or(-1);
    if tactics_calls::slot_pitch_position(ts, captain, core::ptr::null_mut()) == 0 && read_usize(ts + t::MATCH) != 0 {
        let (mut best, mut top) = (-1i8, 0i16);
        for k in 0..0x14i8 {
            let mp = tactics_row_match_player(ts, k, false);
            if read_i8(mp + match_player::POSITION) >= 0 {
                let v = exe::match_man_get_captain_score(0usize, mp, club);
                if best < 0 || v > top {
                    best = k;
                    top = v;
                }
            }
        }
        if best > -1 {
            covered!("tel15_7 return text: the captain picked");
            let mp = tactics_row_match_player(ts, best, false);
            exe::database_format_staff_name(read_usize(mp + match_player::STAFF), name.as_mut_ptr() as usize, 0xc8, 1);
            tactics_player_link(text, club, name.as_ptr(), read_usize(mp + match_player::STAFF));
            sprintf(JOB_TEXT, 0xa16fec, name.as_ptr());
            tactics_append_paragraph(msg);
        }
    }
    let (mut takers, mut count) = ([0usize; 10], 0usize);
    for k in 0..0x14i8 {
        if read_u32(ts + t::HAS_MATCH_RECORD) != 0 && k >= ((read_u8(ts + t::BENCH_SIZE) & 0xf) + 0xb) as i8 {
            continue;
        }
        if tactics_calls::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 || tactics_calls::slot_on_bench(ts, k) != 0 {
            continue;
        }
        let p = tactics_calls::slot_player(ts, k);
        if p == 0 {
            continue;
        }
        for s in 0..10 {
            if read_u32(ts + tactics_screen_state::SET_PIECE_TAKERS + s * 4) == p as u32 && !takers[..count].contains(&p) {
                takers[count] = p;
                count += 1;
            }
        }
    }
    if count > 0 {
        covered!("tel15_7 return text: set piece takers off the pitch");
        let (mut list, mut joined) = (vec![0u8; 0x7d0], vec![0u8; 0x7d0]);
        for (k, &staff) in takers[..count].iter().enumerate() {
            exe::database_format_staff_name(staff, name.as_mut_ptr() as usize, 0xc8, 1);
            if k == 0 {
                tel26_copy_text(joined.as_mut_ptr(), name.as_ptr());
            } else {
                let format = if k != count - 1 { 0xa17af4 } else if count == 2 { 0xa17b40 } else { 0xa17b18 };
                sprintf(joined.as_mut_ptr() as usize, format, list.as_ptr(), name.as_ptr());
            }
            tel26_copy_text(list.as_mut_ptr(), joined.as_ptr());
            tactics_player_link(text, club, name.as_ptr(), staff);
        }
        sprintf(JOB_TEXT, if count > 1 { 0xa8c5ec } else { 0xa8c594 }, list.as_ptr());
        tactics_append_paragraph(msg);
    }
    if message[0] == 0 || read_u32(ts + t::FIXTURE) == 0 {
        return 0;
    }
    covered!("tel15_7 return text: return to the match");
    sprintf(JOB_TEXT, 0xa8c57c);
    tactics_append_paragraph(msg);
    exe::text_set_string(text, msg);
    1
}

/// The pitch drawing's static colours, set on the first call (guard bits of `GUARD`): the dark green of the pitch
/// (rgb 0, 0x40, 0) and of its stripes (rgb 0, 0x38, 0).
mod pitch_statics {
    pub(crate) const GUARD: usize = 0xde1e20;
    pub(crate) const GREEN_40: usize = 0xde1f34;
    pub(crate) const GREEN_38: usize = 0xde1e24;
}

/// 22.0, the half width of the penalty area in pitch units (`g_f64_22_0`).
const PITCH_F64_22: usize = 0x96af40;

/// One shape of the pitch drawing: `Display::add_item` kind 1 (the rectangle, the style, the colour, the outline
/// colour of a circle and the area).
unsafe fn pitch_item(x1: i32, y1: i32, x2: i32, y2: i32, style: u32, colour: u32, outline: u32, area: u32) {
    tactics_item([1, x1 as u32, y1 as u32, x2 as u32, y2 as u32, 0, 0, style, colour, 0, 0, 0, outline, 0, 0, 0, 0, area]);
}

/// The game's `_ftol` (truncation toward zero).
fn pitch_ftol(v: f64) -> i32 {
    v as i32
}

/// Draws a football pitch in a rectangle of the current human's display (the display items of a pitch).
///
/// The first call sets the two static greens (`pitch_statics`). The line colour is rgb 0, 0x80, 0 without an area
/// (`area` -1 in its low word) and rgb 0, 0x50, 0 with one. The pitch: the rectangle in the dark green (style 0x30,
/// area -1; 0x70 in an area), and three stripes in the stripe green over the second, fourth and sixth of seven bands
/// (one pixel in from the sides). The touchlines sit `2 x scale` inside the rectangle (written to the four outputs:
/// left, top, right, bottom) and are drawn as an outline (0x220); then the halfway line through the centre, the centre
/// circle (radius 10 x scale), and at each end the penalty arc (from 2 to 22 x scale deep, 10 x scale each side of the
/// centre line) masked by a dark rectangle up to the penalty area (and the middle stripe drawn again over the mask),
/// the penalty spot (12 x scale), the penalty area (6 deep, 10 each side) and the goal area (18 deep, 22 each side),
/// all in pitch units of `scale`. `across` set lays the pitch with the goals left and right (stripes across, halfway
/// line upright), else with the goals top and bottom. The coordinates are computed on the x87 in double precision and
/// truncated like `_ftol`. Returns 1.
/// Original: `draw_pitch` (0x8ac600, tactics_screens.cpp, cdecl, 11 arguments: the float `scale` last).
pub(crate) unsafe extern "C" fn draw_pitch(left: i32, top: i32, right: i32, bottom: i32, out_left: *mut i32, out_top: *mut i32, out_right: *mut i32,
                                           out_bottom: *mut i32, across: u32, area: u32, scale: f32) -> u32 {
    use pitch_statics as s;
    covered!("tel15_7 draw a pitch");
    let rgb = |g: u32| exe::graphics_rgb_to_pixel(0, g, 0, 0);
    let guard = read_u8(s::GUARD);
    if guard & 1 == 0 {
        write_u8(s::GUARD, guard | 1);
        write_u16(s::GREEN_40, rgb(0x40) as u16);
    }
    let guard = read_u8(s::GUARD);
    if guard & 2 == 0 {
        write_u8(s::GUARD, guard | 2);
        write_u16(s::GREEN_38, rgb(0x38) as u16);
    }
    let no_area = area as u16 == 0xffff;
    let line = rgb(if no_area { 0x80 } else { 0x50 });
    let dark = || read_u16(s::GREEN_40) as u32;
    let stripe = || read_u16(s::GREEN_38) as u32;
    let (first_style, first_area) = if no_area { (0x30, u32::MAX) } else { (0x70, area) };
    pitch_item(left, top, right, bottom, first_style, dark(), 0, first_area);
    let scaled = |k: usize| scale as f64 * read_f64(k);
    let two = scale as f64 + scale as f64;
    if across != 0 {
        covered!("tel15_7 pitch with the goals left and right");
        let h = bottom - top;
        for k in [1, 3, 5] {
            pitch_item(left + 1, top + k * h / 7, right - 1, top + (k + 1) * h / 7, 0x10, stripe(), 0, area);
        }
        let (band_top, band_bottom) = (top + 3 * h / 7, top + 4 * h / 7);
        *out_left = pitch_ftol(left as f64 + two);
        *out_top = pitch_ftol(top as f64 + two);
        *out_right = pitch_ftol(right as f64 - two);
        let y1 = pitch_ftol(bottom as f64 - two);
        *out_bottom = y1;
        let (x0, y0, x1) = (*out_left, *out_top, *out_right);
        let cx = (x1 - x0 + 1) / 2 + *out_left;
        let cy = (y1 - y0 + 1) / 2 + y0;
        pitch_item(*out_left, y0, *out_right, y1, 0x220, line, 0, area);
        pitch_item(cx, *out_top, cx, *out_bottom, 0x400, line, 0, area);
        let r10 = scaled(rdata::F64_10);
        let cyf = cy as f64;
        let (upper, lower) = (pitch_ftol(cyf - r10), pitch_ftol(cyf + r10));
        pitch_item(pitch_ftol(cx as f64 - r10), upper, pitch_ftol(cx as f64 + r10), lower, 0x10020, line, line, area);
        let (r22, r18, r12, r6) = (scaled(PITCH_F64_22), scaled(rdata::F64_18), scaled(rdata::F64_12), scaled(rdata::F64_6));
        let one = read_f64(rdata::F64_1);
        let x0f = *out_left as f64;
        pitch_item(pitch_ftol(x0f + two), upper, pitch_ftol(x0f + r22), lower, 0x10020, line, line, area);
        pitch_item(pitch_ftol(x0f + two), upper, pitch_ftol(r18 + x0f - one), lower, 0x10, dark(), 0, area);
        let x0f = *out_left as f64;
        pitch_item(pitch_ftol(x0f + two), band_top, pitch_ftol(r18 + x0f - one), band_bottom, 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_left as f64 + r12);
        pitch_item(spot, cy, spot, cy, 0x400, line, 0, area);
        let x0 = *out_left;
        pitch_item(x0, upper, pitch_ftol(x0 as f64 + r6), lower, 0x220, line, 0, area);
        let (goal_upper, goal_lower) = (pitch_ftol(cyf - r22), pitch_ftol(r22 + cyf));
        let x0 = *out_left;
        pitch_item(x0, goal_upper, pitch_ftol(x0 as f64 + r18), goal_lower, 0x220, line, 0, area);
        let x1f = *out_right as f64;
        pitch_item(pitch_ftol(x1f - r22), upper, pitch_ftol(x1f - two), lower, 0x10020, line, line, area);
        let x1f = *out_right as f64;
        pitch_item(pitch_ftol(x1f - r18 + one), upper, pitch_ftol(x1f - two), lower, 0x10, dark(), 0, area);
        let x1f = *out_right as f64;
        pitch_item(pitch_ftol(x1f - r18 + one), band_top, pitch_ftol(x1f - two), band_bottom, 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_right as f64 - r12);
        pitch_item(spot, cy, spot, cy, 0x400, line, 0, area);
        let x1 = *out_right;
        pitch_item(pitch_ftol(x1 as f64 - r6), upper, x1, lower, 0x220, line, 0, area);
        let x1 = *out_right;
        pitch_item(pitch_ftol(x1 as f64 - r18), goal_upper, x1, goal_lower, 0x220, line, 0, area);
    } else {
        covered!("tel15_7 pitch with the goals top and bottom");
        let w = right - left;
        for k in [1, 3, 5] {
            pitch_item(left + k * w / 7, top + 1, left + (k + 1) * w / 7, bottom - 1, 0x10, stripe(), 0, area);
        }
        let (band_left, band_right) = (left + 3 * w / 7, left + 4 * w / 7);
        *out_left = pitch_ftol(left as f64 + two);
        *out_top = pitch_ftol(top as f64 + two);
        *out_right = pitch_ftol(right as f64 - two);
        let y1 = pitch_ftol(bottom as f64 - two);
        *out_bottom = y1;
        let (x0, x1, y0) = (*out_left, *out_right, *out_top);
        let cx = (x1 - x0 + 1) / 2 + x0;
        let cy = (y1 - y0 + 1) / 2 + y0;
        pitch_item(x0, y0, x1, y1, 0x220, line, 0, area);
        pitch_item(*out_left, cy, *out_right, cy, 0x400, line, 0, area);
        let r10 = scaled(rdata::F64_10);
        let cxf = cx as f64;
        let (side_left, side_right) = (pitch_ftol(cxf - r10), pitch_ftol(cxf + r10));
        pitch_item(side_left, pitch_ftol(cy as f64 - r10), side_right, pitch_ftol(cy as f64 + r10), 0x10020, line, line, area);
        let (r22, r18, r12, r6) = (scaled(PITCH_F64_22), scaled(rdata::F64_18), scaled(rdata::F64_12), scaled(rdata::F64_6));
        let one = read_f64(rdata::F64_1);
        let y0f = *out_top as f64;
        pitch_item(side_left, pitch_ftol(y0f + two), side_right, pitch_ftol(y0f + r22), 0x10020, line, line, area);
        let y0f = *out_top as f64;
        pitch_item(side_left, pitch_ftol(y0f + two), side_right, pitch_ftol(y0f + r18 - one), 0x10, dark(), 0, area);
        let y0f = *out_top as f64;
        pitch_item(band_left, pitch_ftol(y0f + two), band_right, pitch_ftol(y0f + r18 - one), 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_top as f64 + r12);
        pitch_item(cx, spot, cx, spot, 0x400, line, 0, area);
        let y0 = *out_top;
        pitch_item(side_left, y0, side_right, pitch_ftol(y0 as f64 + r6), 0x220, line, 0, area);
        let y0 = *out_top;
        pitch_item(pitch_ftol(cxf - r22), y0, pitch_ftol(r22 + cxf), pitch_ftol(y0 as f64 + r18), 0x220, line, 0, area);
        let y1f = *out_bottom as f64;
        pitch_item(side_left, pitch_ftol(y1f - r22), side_right, pitch_ftol(y1f - two), 0x10020, line, line, area);
        let y1f = *out_bottom as f64;
        pitch_item(side_left, pitch_ftol(y1f - r18 + one), side_right, pitch_ftol(y1f - two), 0x10, dark(), 0, area);
        let y1f = *out_bottom as f64;
        pitch_item(band_left, pitch_ftol(y1f - r18 + one), band_right, pitch_ftol(y1f - two), 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_bottom as f64 - r12);
        pitch_item(cx, spot, cx, spot, 0x400, line, 0, area);
        let y1 = *out_bottom;
        pitch_item(side_left, pitch_ftol(y1 as f64 - r6), side_right, y1, 0x220, line, 0, area);
        let y1 = *out_bottom;
        pitch_item(pitch_ftol(cxf - r22), pitch_ftol(y1 as f64 - r18), pitch_ftol(r22 + cxf), y1, 0x220, line, 0, area);
    }
    1
}

/// The point of a position on a pitch drawn in a rectangle, from its position flag (`Formation` position bits).
///
/// The goalkeeper (bit 1): the middle of the width, 10 above the bottom (no column needed). Otherwise the row from
/// bits 2..0x40 (bottom up: bottom - h/12 - 15, bottom - h/6 - 20, then 2/6, 3/6, 4/6 and 5/6 of the height up from
/// the bottom, the first one 10 higher) and the column from bits 0x80..0x800 (1/10, 3/10, 5/10, 7/10 and 9/10 of
/// the width with `wide`, else 1/8, 3/8, 1/2, 5/8 and 7/8), each division truncated like the compiler's. A flag
/// without a row shows the error box of line 0x21a8, without a column the box of line 0x21dd (after the row is
/// written): 0. Returns 1.
/// Original: `get_pitch_point_for_position_flags` (0x8ad780, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_pitch_point_for_position_flags(flags: u32, left: i32, top: i32, right: i32, bottom: i32, out_x: *mut i32, out_y: *mut i32,
                                                                   wide: u32) -> u32 {
    if flags & 1 != 0 {
        *out_x = (right - left) / 2 + left;
        *out_y = bottom - 10;
        return 1;
    }
    let (h, up) = (bottom - top, top - bottom);
    *out_y = if flags & 2 != 0 {
        bottom - h / 12 - 0xf
    } else if flags & 4 != 0 {
        bottom - h / 6 - 0x14
    } else if flags & 8 != 0 {
        2 * up / 6 + bottom - 10
    } else if flags & 0x10 != 0 {
        3 * up / 6 + bottom
    } else if flags & 0x20 != 0 {
        4 * up / 6 + bottom
    } else if flags & 0x40 != 0 {
        5 * up / 6 + bottom
    } else {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x21a8);
        return 0;
    };
    let w = right - left;
    let wide = wide != 0;
    let x = if flags & 0x80 != 0 {
        if wide { w / 10 } else { w / 8 }
    } else if flags & 0x100 != 0 {
        if wide { 3 * w / 10 } else { 3 * w / 8 }
    } else if flags & 0x200 != 0 {
        5 * w / 10
    } else if flags & 0x400 != 0 {
        if wide { 7 * w / 10 } else { 5 * w / 8 }
    } else if flags & 0x800 != 0 {
        if wide { 9 * w / 10 } else { 7 * w / 8 }
    } else {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x21dd);
        return 0;
    };
    *out_x = x + left;
    1
}

/// Comparer for `build_return_to_match_text`: the Text side by side (`compare_text_writer`: the strings and the
/// links) and the result.
pub(crate) unsafe extern "thiscall" fn compare_build_return_to_match_text(ts: usize, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(text, || ro.set(o(ts, text)), |t| rr.set(build_return_to_match_text(ts, t)), || "tactics screen build_return_to_match_text".to_string());
    SCOREBOARD.record(ro.get() == rr.get(), || format!("tactics screen build_return_to_match_text: returned {}/{}", ro.get(), rr.get()));
    ro.get()
}

/// Comparer for `draw_pitch`: side by side over the built display list (`compare_match_screen_build`), the four
/// outputs and the static colours.
pub(crate) unsafe extern "C" fn compare_draw_pitch(left: i32, top: i32, right: i32, bottom: i32, out_left: *mut i32, out_top: *mut i32, out_right: *mut i32,
                                                   out_bottom: *mut i32, across: u32, area: u32, scale: f32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, f32) -> u32>(%%I1%%);
    let r = core::cell::Cell::new(0u32);
    let extra = [(out_left as usize, 4), (out_top as usize, 4), (out_right as usize, 4), (out_bottom as usize, 4), (pitch_statics::GUARD, 1),
        (pitch_statics::GREEN_40, 2), (pitch_statics::GREEN_38, 2)];
    compare_match_screen_build("draw_pitch", &extra, || r.set(o(left, top, right, bottom, out_left, out_top, out_right, out_bottom, across, area, scale)),
                               || { draw_pitch(left, top, right, bottom, out_left, out_top, out_right, out_bottom, across, area, scale); });
    r.get()
}

/// Comparer for `get_pitch_point_for_position_flags`: side by side over the two outputs and the result.
pub(crate) unsafe extern "C" fn compare_get_pitch_point_for_position_flags(flags: u32, left: i32, top: i32, right: i32, bottom: i32, out_x: *mut i32,
                                                                           out_y: *mut i32, wide: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, i32, i32, i32, i32, *mut i32, *mut i32, u32) -> u32>(%%I2%%);
    if flags & 1 == 0 && (flags & 0x7e == 0 || flags & 0xf80 == 0) {
        return o(flags, left, top, right, bottom, out_x, out_y, wide); // simplification: the box, original only
    }
    compare_tactics_method("get_pitch_point_for_position_flags", &[(out_x as usize, 4), (out_y as usize, 4)],
                           || o(flags, left, top, right, bottom, out_x, out_y, wide), || get_pitch_point_for_position_flags(flags, left, top, right, bottom, out_x, out_y, wide))
}
