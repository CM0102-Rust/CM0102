// ---------- batch %%LEVA%%: the staff profile's Profile tab (staff_screens.cpp, part 2) ----------

/// The attribute rows of the Profile tab (shared with the compare players screen, the same table at 0xddf570): the
/// name (`database_get_player_attribute_name` / `database_get_non_player_attribute_name`) and, at +0x64, the value
/// or the code of an extra row (0xff preferred foot, 0xfe form, 0xfd morale, 0xfc condition, 0xfb on loan, 0xfa
/// coaching style). Rows of 0x65 bytes; 31 player attributes plus 5 extra rows.
const PROFILE_TABLE: usize = 0xddf570;
const PROFILE_ROW: usize = 0x65;
const PROFILE_ROW_VALUE: usize = 0x64;
const PROFILE_PLAYER_ROWS: usize = 0x24;
/// The qsort comparer of the attribute rows (by name).
const PROFILE_ROW_COMPARER: usize = 0x88bc50;
/// Colour of the loan and free transfer lines (a word).
const PROFILE_COLOUR_3152: usize = 0xae3152;
/// `staff_state::STATUS_FLAGS` bit 0x40 (the regen mark of docs/MARCA_DO_REGEN.md): here it also gates the
/// chairman table and lets a chairman see the formation and style table [inference: what the bit means for a
/// non-player].
const PROFILE_STATE_SHOWN: u8 = 0x40;

/// The row `k` of the attribute table.
fn profile_row(k: usize) -> usize {
    PROFILE_TABLE + k * PROFILE_ROW
}

/// Balances `n` rows over `columns` columns one by one: the rows of each column and the tallest column (at least
/// `rows`, unsigned bytes like the original).
fn profile_columns(n: usize, columns: usize, rows: u8) -> ([u8; 3], u8) {
    let mut counts = [0u8; 3];
    let mut tallest = rows;
    let mut k = 0;
    for _ in 0..n {
        counts[k] = counts[k].wrapping_add(1);
        if counts[k] > tallest {
            tallest = counts[k];
        }
        k += 1;
        if k >= columns {
            k = 0;
        }
    }
    (counts, tallest)
}

/// Empty cells of the column pair (`col`, `col + 1`) of `area` from `*row` up to `tallest`.
unsafe fn profile_blank_rows(area: u32, col: u32, row: &mut i32, tallest: u8) {
    while *row < tallest as i32 {
        staff_tab_cell(area, col, *row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        staff_tab_cell(area, col + 1, *row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        *row += 1;
    }
}

/// The text of an extra row of the player attribute table into `out` (200 bytes): the preferred foot (left foot ≥
/// 15: "Left Only" when the right foot is under 8, "Left" under 14, else "Either"; otherwise "Right Only" when the
/// left foot is under 8, else "Right"), the form (`PlayerStats::format_recent_ratings`; "-" for an id past the person
/// count), the morale (`contract_manager_write_morale_text`) and the condition ("Injured" for an injury status 2,
/// else the condition `%d%%`). Other codes leave `out` as it is.
unsafe fn profile_extra_row_text(code: u8, p: Staff, out: usize) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    match code {
        0xff => {
            let sheet = p.player().addr();
            let left = read_i8(sheet + player_sheet::LEFT_FOOT);
            if left >= 15 {
                let right = read_i8(sheet + player_sheet::RIGHT_FOOT);
                if right < 8 {
                    lang(out, 0xa86374usize); // "Left Only"
                } else if right < 14 {
                    lang(out, 0xa8634cusize, 0xad9c64usize); // "Left<%s - COMMENT - prefers left foot>"
                } else {
                    lang(out, 0xa86344usize); // "Either"
                }
            } else if left < 8 {
                lang(out, 0xa86338usize); // "Right Only"
            } else {
                lang(out, 0xa8630cusize, 0xad9c64usize); // "Right<%s - COMMENT - prefers right foot>"
            }
        }
        0xfe => {
            if !p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
                format(out, 0x98ba0cusize); // "-"
            } else {
                let ratings: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x7c1660usize);
                ratings(read_usize(G_PLAYER_STATS), p.player().addr(), out, 0xc8);
            }
        }
        0xfd => {
            let morale: unsafe extern "stdcall" fn(usize, usize, u32) = core::mem::transmute(0x4cc2e0usize);
            morale(p.addr(), out, 0xc8);
        }
        0xfc => {
            if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
                lang(out, 0xa86304usize); // "Injured"
            } else {
                let condition = exe::injury_get_condition(read_usize(G_INJURY), p, 1);
                format(out, 0x98e8d4usize, condition as i32); // "%d%%"
            }
        }
        _ => {}
    }
}

/// The player attribute table of the Profile tab: six columns (weights 3 2 3 2 3 2) between `top` and `bottom`, as
/// many rows as fit in font 2. Brings the player's training up to date (`update_player_training`), fills the 31
/// attribute rows (the value the human sees with attribute masking on, `staff_get_player_attribute_for_viewer`, else
/// `staff_compute_player_attribute`), sorts them by name and adds the five extra rows; then three name/value column
/// pairs of 12 rows: the value in yellow, the extra rows' texts in light blue (`profile_extra_row_text`; the form
/// with one more space and another style), and for the On Loan row an empty item across the value columns and "On
/// loan from <club>" (linked 0x7d5) or "Given free transfer" wrapped from x 0x238. Each column is padded with empty
/// rows to the tallest.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
unsafe fn profile_tab_player_attributes(top: i32, bottom: i32, p: Staff, manager: usize, t: *mut usize, weights: &[u8; 6]) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let rows = staff_tab_rows(top, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 6, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), p, 0);
    let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5428a0usize);
    let for_viewer: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543440usize);
    for i in 1..=31u32 {
        let r = profile_row(i as usize - 1);
        name(i, r, 100, 0);
        let v = if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
            for_viewer(manager, p.addr(), i)
        } else {
            exe::staff_compute_player_attribute(p, p.player(), i)
        };
        write_u8(r + PROFILE_ROW_VALUE, v);
    }
    exe::crt_qsort(PROFILE_TABLE, 0x1f, PROFILE_ROW as u32, PROFILE_ROW_COMPARER);
    let extra = [
        (0xa85874usize, 0xffu8), // "Preferred Foot"
        (0x98e9f8, 0xfe),        // "Form"
        (0x98e9f0, 0xfd),        // "Morale"
        (0xa86380, 0xfc),        // "Condition"
        (0xa85860, 0xfb),        // "On Loan"
    ];
    for (k, (s, code)) in extra.iter().enumerate() {
        let r = profile_row(31 + k);
        lang(r, *s);
        write_u8(r + PROFILE_ROW_VALUE, *code);
    }
    let (counts, tallest) = profile_columns(PROFILE_PLAYER_ROWS, 3, rows);
    let mut col: u32 = 0;
    let mut row: i32 = 0;
    let mut out = [0u8; 0x100];
    let o = out.as_mut_ptr() as usize;
    for k in 0..PROFILE_PLAYER_ROWS {
        let r = profile_row(k);
        let code = read_u8(r + PROFILE_ROW_VALUE);
        if code == 0xfb {
            tel26_add_item(current_human_display(), [1, 0x22e, 0, 0x30c, 0, 0, row as u32, 2, 0, 0, 0xc, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0, 0, 0, 0, area]);
            let mut done = false;
            if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                let (c, club) = contract_tab_lookup(p);
                if !c.is_null() && club != 0 {
                    covered!("tel08_2 profile tab on loan");
                    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
                    lang(JOB_TEXT, 0xa85834usize, gender, gender, club + club::SHORT_NAME); // "On loan from {}<%s - Club>{}"
                    exe::text_set_string(t as usize, JOB_TEXT as *const u8);
                    exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
                    staff_tab_draw_wrapped(t, area, 0x238, 0x30c, &mut row, 2, PROFILE_COLOUR_3152, 3, 0);
                    done = true;
                }
            }
            if !done && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) != 0 {
                lang(JOB_TEXT, 0xa85820usize); // "Given free transfer"
                exe::text_set_string(t as usize, JOB_TEXT as *const u8);
                staff_tab_draw_wrapped(t, area, 0x238, 0x30c, &mut row, 2, PROFILE_COLOUR_3152, 3, 0);
            }
        } else {
            format(JOB_TEXT, 0x987ff0usize, r); // "  %s"
            staff_tab_cell(area, col, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            if (code as i8) >= 0 {
                format(JOB_TEXT, 0x98806cusize, code as i8 as i32); // "%d" tel08_2 the attribute value
                staff_tab_cell(area, col + 1, row, 2, 0, 0xc, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
            } else {
                profile_extra_row_text(code, p, o);
                if code == 0xfe {
                    format(JOB_TEXT, 0xa694f4usize, o); // "   %s"
                    staff_tab_cell(area, col + 1, row, 2, 0, 1, 1, JOB_COLOUR_BDD4, JOB_TEXT);
                } else if code == 0xf9 {
                    format(JOB_TEXT, 0x98e8c8usize); // "  -"
                    staff_tab_cell(area, col + 1, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
                } else {
                    format(JOB_TEXT, 0x987ff0usize, o); // "  %s"
                    staff_tab_cell(area, col + 1, row, 2, 0, 1, 2, JOB_COLOUR_BDD4, JOB_TEXT);
                }
            }
        }
        row += 1;
        if row == counts[(col / 2) as usize] as i32 {
            profile_blank_rows(area, col, &mut row, tallest);
            col += 2;
            row = 0;
            if col + 1 > 5 {
                return;
            }
        }
    }
    while col < 6 {
        profile_blank_rows(area, col, &mut row, tallest);
        col += 2;
        row = 0;
    }
}

/// One line of the chairman table: the label, the value (`%d`, light blue) and the description, row `row`.
unsafe fn profile_chairman_line(area: u32, row: i32, label: usize, value: i8, description: usize) {
    let lang = staff_tab_lang();
    lang(JOB_TEXT, label);
    staff_tab_cell(area, 0, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
    staff_tab_format()(JOB_TEXT, 0x98806cusize, value as i32); // "%d"
    staff_tab_cell(area, 1, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, JOB_TEXT);
    lang(JOB_TEXT, description);
    staff_tab_cell(area, 2, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
}

/// The chairman table of the Profile tab (three columns, weights 3 1 7, the whole tab): business acumen (> 15
/// astute, > 10 competent, > 5 average, else poor), discipline (> 15 harsh, > 10 strict, else average), interference
/// (> 15 very hands on, else quite reserved), man handling (`non_player_attribute_rating` of the man handling, asked
/// again for each threshold: > 15 good, > 3 average, else poor), patience (> 15 very patient, > 5 patient, else
/// impatient), resources (> 15 vastly wealthy, > 5 rich, else wealthy) and ambition (the person's: 20 with resources
/// > 15 incredibly ambitious, > 14 very, < 5 content, else ambitious); then empty rows. Returns the table's height
/// (rows × the font 2 height).
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
unsafe fn profile_tab_chairman(top: i32, bottom: i32, p: Staff, weights: &[u8; 3]) -> i32 {
    let rows = staff_tab_rows(top, bottom, 2);
    let height = exe::font_get_height(2) * rows as i32;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let np = p.non_player().addr();
    let a = |k: usize| read_i8(np + k);
    let mut row: i32 = 0;

    let v = a(non_player::BUSINESS);
    let s = if v > 15 { 0xa86260 } else if v > 10 { 0xa86224 } else if v > 5 { 0xa861e8 } else { 0xa861ac };
    profile_chairman_line(area, row, 0xa86290, v, s); // "  Business Acumen"
    row += 1;
    let v = a(non_player::DISCIPLINE);
    let s = if v > 15 { 0xa86168 } else if v > 10 { 0xa8612c } else { 0xa860ec };
    profile_chairman_line(area, row, 0xa8619c, v, s); // "  Discipline"
    row += 1;
    let v = a(non_player::INTERFERENCE);
    let s = if v > 15 { 0xa860a0 } else { 0xa8605c };
    profile_chairman_line(area, row, 0xa860dc, v, s); // "  Interference"
    row += 1;
    let rating = || exe::non_player_attribute_rating(np, a(non_player::MAN_HANDLING));
    let v = rating();
    let s = if rating() > 15 { 0xa86014 } else if rating() > 3 { 0xa85fe0 } else { 0xa85fa8 };
    profile_chairman_line(area, row, 0xa8604c, v, s); // "  Man Handling"
    row += 1;
    let v = a(non_player::PATIENCE);
    let s = if v > 15 { 0xa85f64 } else if v > 5 { 0xa85f38 } else { 0xa85f04 };
    profile_chairman_line(area, row, 0xa85f9c, v, s); // "  Patience"
    row += 1;
    let v = a(non_player::RESOURCES);
    let s = if v > 15 { 0xa85ec0 } else if v > 5 { 0xa85e98 } else { 0xa85e6c };
    profile_chairman_line(area, row, 0xa85ef8, v, s); // "  Resources"
    row += 1;
    let v = p.ambition();
    let s = if v == 20 && a(non_player::RESOURCES) > 15 {
        0xa85e1c
    } else if v > 14 {
        0xa85ddc
    } else if v < 5 {
        0xa85da0
    } else {
        0xa85d5c
    };
    profile_chairman_line(area, row, 0xa85e60, v, s); // "  Ambition"
    row += 1;
    while row < rows as i32 {
        staff_tab_cell(area, 0, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, 0xad9c64);
        staff_tab_cell(area, 1, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, 0xad9c64);
        staff_tab_cell(area, 2, row, 2, 0, 0xc, 2, JOB_COLOUR_BDD4, 0xad9c64);
        row += 1;
    }
    height
}

/// The preferred style text of a coach into the screens' buffer, by attacking (`a`) and directness (`d`): both ≥ 15
/// direct attacking, attacking ≥ 15 and directness ≤ 5 attractive attacking, attacking ≤ 5 and directness ≥ 15
/// cautious but direct, both ≤ 5 patient and defensive; otherwise attacking (≥ 15), defensive (≤ 5), direct
/// (directness ≥ 15), patient (≤ 5) or none.
unsafe fn profile_style_text(a: i8, d: i8) {
    let s = if a >= 15 && d >= 15 {
        0xa85cb0
    } else if a >= 15 && d <= 5 {
        0xa85c80
    } else if a <= 5 && d >= 15 {
        0xa85c50
    } else if a <= 5 && d <= 5 {
        0xa85c20
    } else if a >= 15 {
        0xa85bf8
    } else if a <= 5 {
        0xa85bd4
    } else if d >= 15 {
        0xa85bb0
    } else if d <= 5 {
        0xa85b8c
    } else {
        0xa85b68
    };
    staff_tab_lang()(JOB_TEXT, s);
}

/// The formation and style table of a non-player (two columns, weights 3 7) from `y` to `bottom`: the preferred
/// formation (1..9: 5-3-2 sweeper, 5-3-2, 4-4-2, 4-3-3, 4-2-4, 3-5-2, 4-1-2-1-2, 4-5-1, 3-4-3; else "-"), the
/// preferred style (`profile_style_text`) and "Other": pressing, offside trap and man-marking ≥ 15 each on its own
/// row ("-" for none); then empty rows.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
unsafe fn profile_tab_formation(y: i32, bottom: i32, p: Staff, weights: &[u8; 2]) {
    let lang = staff_tab_lang();
    let rows = staff_tab_rows(y, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let np = p.non_player().addr();
    let a = |k: usize| read_i8(np + k);
    let label = |row: i32, s: usize| {
        staff_tab_cell(area, 0, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, s);
    };
    let value = |row: i32, s: usize| {
        staff_tab_cell(area, 1, row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, s);
    };
    let mut row: i32 = 0;
    lang(JOB_TEXT, 0xa85d44usize); // "  Preferred Formation"
    label(row, JOB_TEXT);
    let formations = [0xa85d34usize, 0xa85d2c, 0xa85d24, 0xa85d1c, 0xa85d14, 0xa85d0c, 0xa85d00, 0xa85cf8, 0xa85cf0];
    match a(non_player::FORMATION) {
        f @ 1..=9 => {
            lang(JOB_TEXT, formations[f as usize - 1]);
        }
        _ => staff_tab_strcpy(JOB_TEXT, 0x98e8c8), // "  -"
    }
    value(row, JOB_TEXT);
    row += 1;
    lang(JOB_TEXT, 0xa85cdcusize); // "  Preferred Style"
    label(row, JOB_TEXT);
    profile_style_text(a(non_player::ATTACKING), a(non_player::DIRECTNESS));
    value(row, JOB_TEXT);
    row += 1;
    lang(JOB_TEXT, 0xa85b38usize, 0xad9c64usize); // "  Other<%s - COMMENTS - other playing styles>"
    label(row, JOB_TEXT);
    let start = row;
    if a(non_player::PRESSING) >= 15 {
        lang(JOB_TEXT, 0xa85b04usize); // "  Likes his players to close down the opposition"
        value(row, JOB_TEXT);
        row += 1;
    }
    for (field, s) in [(non_player::OFFSIDE, 0xa85ae0usize), (non_player::MARKING, 0xa85ab8)] {
        if a(field) >= 15 {
            if row > start {
                label(row, 0);
            }
            lang(JOB_TEXT, s); // the offside trap, the man-marking system
            value(row, JOB_TEXT);
            row += 1;
        }
    }
    if row == start {
        value(row, 0x98e8c8); // "  -"
        row += 1;
    }
    profile_blank_rows(area, 0, &mut row, rows);
}

/// The non-player profile of the Profile tab. For a person who is not a chairman (`job_for_club` ≠ 1), the attribute
/// table: the 12 non-player attributes (the 10th only when the person's club or national job is 10 or the attribute
/// is at least 10), sorted by name, plus the coaching style row (coaching technique ≤ 5 fitness-based, ≥ 15
/// technique-based, else general), in two name/value column pairs (weights 3 2 3 2) as tall as half the rows. For a
/// chairman whose state record has bit 0x40, the chairman table (`profile_tab_chairman`). Then, unless he is a
/// chairman without that bit, the formation table (`profile_tab_formation`) 5 pixels under the table above.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
unsafe fn profile_tab_non_player(top: i32, bottom: i32, p: Staff, w4: &[u8; 4], w3: &[u8; 3], w2: &[u8; 2]) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let np = p.non_player();
    let chairman = p.job_for_club() == 1;
    let mut height = 0;
    if !chairman {
        covered!("tel08_2 profile tab non-player attributes");
        let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x543b10usize);
        let mut n = 0usize;
        for i in 1..=12u32 {
            if i == 10
                && p.job_for_club() != 10
                && p.job_for_nation() != 10
                && (exe::staff_compute_non_player_attribute(p, np, 10) as i8) < 10
            {
                continue;
            }
            let r = profile_row(n);
            name(i, r, 100, 0);
            write_u8(r + PROFILE_ROW_VALUE, exe::staff_compute_non_player_attribute(p, np, i));
            n += 1;
        }
        exe::crt_qsort(PROFILE_TABLE, n as u32, PROFILE_ROW as u32, PROFILE_ROW_COMPARER);
        lang(profile_row(n), 0xa862f4usize); // "Coaching Style"
        write_u8(profile_row(n) + PROFILE_ROW_VALUE, 0xfa);
        n += 1;
        let half = ((n as i32 + 1) / 2) as u8;
        height = exe::font_get_height(2) * half as i32;
        let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, (top + height) as u32, 4, w4.as_ptr() as u32, half as u32, 0, 1, 0, u32::MAX]);
        let (counts, tallest) = profile_columns(n, 2, 0);
        let mut col: u32 = 0;
        let mut row: i32 = 0;
        for k in 0..n {
            let r = profile_row(k);
            format(JOB_TEXT, 0x987ff0usize, r); // "  %s"
            staff_tab_cell(area, col, row, 2, 0, 1, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
            let code = read_u8(r + PROFILE_ROW_VALUE);
            let colour = if code == 0xfa {
                let c = read_i8(np.addr() + non_player::COACHING_TECHNIQUE);
                if c <= 5 {
                    lang(JOB_TEXT, 0xa862e4usize); // "Fitness-based"
                } else if c >= 15 {
                    lang(JOB_TEXT, 0xa862d4usize); // "Technique-based"
                } else {
                    lang(JOB_TEXT, 0xa862a4usize, 0xad9c64usize); // "General<%s - COMMENT - general coaching style>"
                }
                JOB_COLOUR_BDD4
            } else {
                format(JOB_TEXT, 0x98806cusize, code as i8 as i32); // "%d"
                G_GUI_COLOUR_YELLOW
            };
            staff_tab_cell(area, col + 1, row, 2, 0, 0xc, 2, colour, JOB_TEXT);
            row += 1;
            if row == counts[(col / 2) as usize] as i32 {
                profile_blank_rows(area, col, &mut row, tallest);
                col += 2;
                row = 0;
                if col > 2 {
                    break;
                }
            }
        }
        while col < 4 {
            profile_blank_rows(area, col, &mut row, tallest);
            col += 2;
            row = 0;
        }
    }
    let state = read_u8(read_usize(G_STAFF_STATE_TABLE) + read_i32(p.addr()) as usize * staff_state::SIZE + staff_state::STATUS_FLAGS);
    let shown = state & PROFILE_STATE_SHOWN != 0;
    if shown && chairman {
        covered!("tel08_2 profile tab chairman");
        height = profile_tab_chairman(top, bottom, p, w3);
    }
    if !shown && chairman {
        return;
    }
    profile_tab_formation(top + height + 5, bottom, p, w2);
}

/// The headers of the match history columns 7 to 12 for statistics view `kind` (0 general, 1 passing and shooting,
/// 2 tackles and headers, other discipline): each string and whether its format takes the comment argument.
fn profile_history_headers(kind: u8) -> [(usize, bool); 6] {
    match kind {
        0 => [(0xa78490, false), (0xa7845c, false), (0xa78464, true), (0x98e978, false), (0xa78440, false), (0xa78434, false)],
        1 => [(0x9b7964, false), (0xa161d0, false), (0xa85a98, true), (0xa169cc, true), (0xa169a8, true), (0xa78490, false)],
        2 => [(0xa161c8, false), (0xa16968, true), (0xa85a78, true), (0x9b7a28, false), (0xa169e8, true), (0xa85a58, true)],
        _ => [(0x990e40, false), (0xa161b0, false), (0xa161a8, false), (0xa161a4, false), (0xa16924, true), (0xa16944, true)],
    }
}

/// One statistic of a match history entry for column 7..12 and view `kind`: the byte offset and whether it is signed
/// (column 7 of the discipline view is written apart: penalties scored and taken).
fn profile_history_stat(kind: u8, k: usize) -> (usize, bool) {
    match kind {
        0 => [(0x10, true), (0x11, true), (0x12, true), (0x13, true), (0x14, true), (0x15, false)][k],
        1 => [(0x16, false), (0x17, false), (0x18, false), (0x21, false), (0x22, false), (0x10, true)][k],
        2 => [(0x19, false), (0x1a, false), (0x1b, false), (0x1c, false), (0x1d, false), (0x1e, false)][k],
        _ => [(0x26, false), (0x20, false), (0x23, false), (0x24, false), (0x25, false), (0x1f, false)][k],
    }
}

/// The match history of the Profile tab (the person has a scouting search entry): a frame (14 columns), the header
/// row (the arrows 4 and 5, buttons 0x27 and 0x28, as font characters 0x2a/0x2b or "<<"/">>" by `G_FONT_MODE`; the
/// statistics headers of view `kind` (screen parameter 0x13, `profile_history_headers`) and "Min") and up to 10
/// matches (`player_search_slot_get_match_entry`): the date, "<score> v <opposition> (<venue>)" wrapped from x 0xbd
/// to 0x15c (the score linked 0x7d3 to the match when the person's club played it, `fixture_entry_matches`; the club
/// linked 0x7d5; grey for an opposition id below 0), the six statistics of the view and the rating.
/// Original: part of `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp).
unsafe fn profile_tab_match_history(top: i32, bottom: i32, p: Staff, entry: usize, t: *mut usize, w_header: &[u8; 14], w_matches: &[u8; 14]) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let kind = get_screen_param(0x13) as u8;
    let d = current_human_display();
    tel26_add_area(d, [0x6e, top as u32, 0x30c, bottom as u32, 0xe, w_header.as_ptr() as u32, 1, 0, 2, 0, u32::MAX]);
    let (y1, y2) = (top + 4, bottom - 4);
    let rows = staff_tab_rows(y1, y2, 2);
    let mut row: i32 = 0;
    let area = tel26_add_area(d, [0x6e, y1 as u32, 0x30c, y2 as u32, 0xe, w_header.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    for col in 0..3 {
        staff_tab_cell(area, col, row, 1, 0, 0xc, 1, G_GUI_COLOUR_PALE_GREY, 0);
    }
    let blue = job_colour(G_GUI_COLOUR_DARK_BLUE);
    for (col, ch, arrows, id) in [(4u32, 0x2au32, 0x98e7c4usize, 0x27u32), (5, 0x2b, 0x98e7a8, 0x28)] {
        if read_i32(G_FONT_MODE) != -1 {
            format(JOB_TEXT, 0x99ec1cusize, ch); // "%c"
            staff_tab_button(area, col, row, 0x30, blue, 0xc, 0, G_GUI_COLOUR_PALE_GREY, JOB_TEXT, id, 0);
        } else {
            staff_tab_button(area, col, row, 0x30, blue, 0xc, 1, G_GUI_COLOUR_PALE_GREY, arrows, id, 0); // "<<" / ">>"
        }
    }
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let headers = profile_history_headers(kind);
    for (k, (s, comment)) in headers.iter().chain([(0xa85a3c, true)].iter()).enumerate() {
        if *comment {
            lang(JOB_TEXT, *s, 0xad9c64usize);
        } else {
            lang(JOB_TEXT, *s);
        }
        staff_tab_cell(area, 7 + k as u32, row, 0x30, grey, 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
    }

    let table = tel26_add_area(d, [0x6e, y1 as u32, 0x30c, y2 as u32, 0xe, w_matches.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    row = 1;
    let get_entry: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7bd830usize);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54a650usize);
    let venue_text: unsafe extern "C" fn(u32, usize, u32) = core::mem::transmute(0x7e4ca0usize);
    let (mut score, mut venue) = ([0u8; 0x100], [0u8; 0x100]);
    let (sb, vb) = (score.as_mut_ptr() as usize, venue.as_mut_ptr() as usize);
    for k in 0..10u32 {
        let e = get_entry(entry, k);
        if e == 0 {
            break;
        }
        covered!("tel08_2 profile tab match history");
        let mut date = [0u16; 4];
        exe::new_date(date.as_mut_ptr() as usize, read_u16(e) as u32, read_u16(e + 2) as u32);
        short_date(date.as_ptr() as usize, JOB_TEXT, 2000, 0xff);
        staff_tab_cell(table, 0, row, 0x30, blue, 0xc, 1, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
        lang(sb, 0x990bb4usize, read_u8(e + 8) as u32, read_u8(e + 9) as u32); // "<%d - home score>:<%d - away score>"
        venue_text(read_u8(e + 0xe) as u32, vb, 0xc8);
        let opposition = read_i32(e + 4);
        let club = club_at(opposition.wrapping_abs() as isize).addr();
        let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        // "  <%s - score> v {}<%s - opposition team>{} (<%s - venue e.g. H>)"
        lang(JOB_TEXT, 0xa859f8usize, gender, gender, sb, club + club::SHORT_NAME, vb);
        staff_tab_text_clear(t);
        exe::text_set_string(t as usize, JOB_TEXT as *const u8);
        let own = p.club_job();
        if !own.is_null()
            && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(e + 0xa), read_i32(own.addr()), opposition, read_u16(e) as i32, read_u16(e + 2) as i32) != 0
        {
            exe::text_add_link(t as usize, sb as *const u8, 0x7d3, read_usize(e + 0xa));
        }
        let club = club_at(read_i32(e + 4).wrapping_abs() as isize).addr();
        exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
        let colour = if read_i32(e + 4) > -1 { G_GUI_COLOUR_PALE_GREY } else { G_GUI_COLOUR_GREY };
        staff_tab_draw_wrapped(t, table, 0xbd, 0x15c, &mut row, 2, colour, 2, 0);
        for c in 0..6 {
            let (at, signed) = profile_history_stat(kind, c);
            if kind > 2 && c == 0 {
                let scored = read_u8(e + 0x26);
                if scored == 0 {
                    format(JOB_TEXT, 0x99cc54usize); // "0"
                } else {
                    format(JOB_TEXT, 0x99cc58usize, scored as u32, read_u8(e + 0x27) as u32); // "%d (%d)"
                }
            } else {
                let v = if signed { read_i8(e + at) as i32 } else { read_u8(e + at) as i32 };
                format(JOB_TEXT, 0x98806cusize, v); // "%d"
            }
            staff_tab_cell(table, 7 + c as u32, row, 1, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, JOB_TEXT);
        }
        format(JOB_TEXT, 0x98806cusize, read_u8(e + 0xf) as u32); // "%d" the rating
        staff_tab_cell(table, 0xd, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT);
        row += 1;
    }
}

/// Draws the Profile tab of the staff profile (the person of screen parameter 0, view mode in parameter 4) between
/// screen rows `top` and `bottom`.
///
/// No profile ("There is no profile available.", font 3) for a person whose `job_for_club` is 2 or whose id is from
/// the person count − 16 up (past the count, or a null person). View 2 (the scouting view): the match history when
/// the scouting manager has a search entry for him (`find_club_search_entry`, `profile_tab_match_history`), else the
/// player attributes for a player or the non-player profile; view 0: the player attributes of a player (nothing for
/// a non-player); view 1: the non-player profile of a person with a non-player sheet.
///
/// Convention: cdecl(top, bottom), SEH frame (a Text, destroyed at the end). Split into named pieces; the whole
/// routine is hooked.
/// Original: `staff_screens_draw_profile_tab` (0x87a510, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_profile_tab(top: i32, bottom: i32) {
    let w6: [u8; 6] = [3, 2, 3, 2, 3, 2];
    let w4: [u8; 4] = [3, 2, 3, 2];
    let w2: [u8; 2] = [3, 7];
    let w3: [u8; 3] = [3, 1, 7];
    let w_matches: [u8; 14] = [7, 0xe, 4, 6, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4];
    let w_header: [u8; 14] = [7, 0xe, 4, 1, 2, 2, 1, 4, 4, 4, 4, 4, 4, 4];
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let manager = tel26_slot_manager().addr();
    let p = Staff(get_screen_param(0));
    let mode = get_screen_param(4);
    let count = read_i32(G_STAFF_COUNT);
    let id = read_i32(p.addr());
    if p.job_for_club() == 2 || (id >= count - 0x10 && (p.is_null() || id >= count)) {
        covered!("tel08_2 profile tab without profile");
        staff_tab_no_information(top, bottom, 0xa859d8); // "There is no profile available."
        exe::drop_text(t);
        return;
    }
    let has_player = !p.player().is_null();
    let has_non_player = !p.non_player().is_null();
    if mode == 2 {
        let find_entry: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7f8610usize);
        let entry = find_entry(read_usize(G_SCOUT_MANAGER), p.addr());
        if entry != 0 {
            profile_tab_match_history(top, bottom, p, entry, t, &w_header, &w_matches);
        } else if has_player {
            profile_tab_player_attributes(top, bottom, p, manager, t, &w6);
        } else if has_non_player {
            profile_tab_non_player(top, bottom, p, &w4, &w3, &w2);
        }
    } else if mode == 0 {
        if has_player {
            covered!("tel08_2 profile tab player");
            profile_tab_player_attributes(top, bottom, p, manager, t, &w6);
        }
    } else if mode == 1 && has_non_player {
        profile_tab_non_player(top, bottom, p, &w4, &w3, &w2);
    }
    exe::drop_text(t);
}

/// Comparer for `staff_screens_draw_profile_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`) plus the attribute table the tab fills (0xddf570, 36 rows) and the contract lookup
/// result. Both sides bring the player's training up to date (`update_player_training`, as the compare players
/// screen does); when it is due (`training_update_pending`, tel07_2) only the original runs, so the week's training
/// rolls once.
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_profile_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    if training_update_pending(get_screen_param(0)) {
        // simplification: the training roll runs once, in the original
        return o(top, bottom);
    }
    let extra = [(PROFILE_TABLE, PROFILE_PLAYER_ROWS * PROFILE_ROW), (G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("staff profile profile tab", &extra, || o(top, bottom), || staff_screens_draw_profile_tab(top, bottom));
}
