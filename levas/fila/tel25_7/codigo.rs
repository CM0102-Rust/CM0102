// ---------- batch %%LEVA%%: the printouts (3): a competition's statistics table and its league tables (printouts.cpp) ----------

/// The row of equals signs under and over a printout section's heading (0xa78950).
pub(crate) const PRINTOUT_EQUALS: usize = 0xa78950;
/// The two spaces between a printout row's columns (0xa78880).
pub(crate) const PRINTOUT_GAP: usize = 0xa78880;
/// "%s\n" (0x9a7d28) and " " (0x9f1308): a section ends with a line holding one space (a page break when printed).
pub(crate) const PRINTOUT_FORMAT_LINE: usize = 0x9a7d28;
pub(crate) const PRINTOUT_PAGE_BREAK: usize = 0x9f1308;
/// The exe's money text buffer (0xde1f64, 0x7d0 bytes): the printouts write long dates, labels and money there.
pub(crate) const PRINTOUT_SCRATCH_TEXT: usize = 0xde1f64;

/// The exe's `fprintf` (0x9464f0, cdecl): the file, the format and its arguments.
pub(crate) unsafe fn printout_fprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(0x9464f0usize)
}

/// A text into the printout file. The original passes the text itself as the format of `fprintf` (a '%' in a name
/// would be read as a conversion): so does the copy.
pub(crate) unsafe fn printout_write(file: usize, text: usize) {
    printout_fprintf()(file, text);
}

/// The texts of `texts` into the printout file, in order.
pub(crate) unsafe fn printout_write_all(file: usize, texts: &[usize]) {
    for &t in texts {
        printout_write(file, t);
    }
}

/// The end of a printout section: two newlines and a line with one space ("%s\n").
pub(crate) unsafe fn printout_end_section(file: usize) {
    printout_write(file, PRINTOUT_NEWLINE);
    printout_write(file, PRINTOUT_NEWLINE);
    printout_fprintf()(file, PRINTOUT_FORMAT_LINE, PRINTOUT_PAGE_BREAK);
}

/// A section heading already in [`PRINTOUT_LINE`]: equals, newline, the heading, newline, equals, two newlines.
pub(crate) unsafe fn printout_write_heading(file: usize) {
    printout_write_all(file, &[PRINTOUT_EQUALS, PRINTOUT_NEWLINE, PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_EQUALS, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE]);
}

/// The column titles `titles` (a format with up to one argument) and the dashes `dashes` under them.
pub(crate) unsafe fn printout_write_titles(file: usize, titles: usize, argument: usize, dashes: usize) {
    comp_lang_format()(PRINTOUT_LINE, titles, argument);
    printout_write_all(file, &[PRINTOUT_LINE, PRINTOUT_NEWLINE, dashes, PRINTOUT_NEWLINE]);
}

/// `strcpy`.
pub(crate) unsafe fn printout_copy(dest: usize, src: usize) {
    core::ptr::copy(src as *const u8, dest as *mut u8, printout_text_length(src) + 1);
}

/// `strcat`.
pub(crate) unsafe fn printout_append(dest: usize, src: usize) {
    printout_copy(dest + printout_text_length(dest), src);
}

/// A column of a printout row: `text` padded (or cut) to `width` (the inlined `printouts_pad_string`), appended to
/// [`PRINTOUT_LINE`] with the two spaces after it.
pub(crate) unsafe fn printout_column(text: usize, width: u32) {
    printouts_pad_string(text, width);
    printout_append(PRINTOUT_LINE, text);
    printout_append(PRINTOUT_LINE, PRINTOUT_GAP);
}

/// The row in [`PRINTOUT_LINE`] into the file, and a newline.
pub(crate) unsafe fn printout_write_row(file: usize) {
    printout_write(file, PRINTOUT_LINE);
    printout_write(file, PRINTOUT_NEWLINE);
}

/// The scratch files of the side-by-side comparison of a printout writer.
const PRINTOUT_SCRATCH_ORIGINAL: &[u8] = b"~printout_original.tmp\0";
const PRINTOUT_SCRATCH_COPY: &[u8] = b"~printout_copy.tmp\0";

/// The whole text of an open scratch file.
unsafe fn printout_file_text(f: usize) -> Vec<u8> {
    exe::crt_fseek(f, 0, 2);
    let n = exe::crt_ftell(f).max(0) as usize;
    exe::crt_fseek(f, 0, 0);
    let mut v = vec![0u8; n];
    if n > 0 {
        let got = exe::crt_fread(v.as_mut_ptr() as usize, 1, n as u32, f) as usize;
        v.truncate(got);
    }
    v
}

/// Up to 40 bytes of a printout text from `k`, for the divergence message.
fn printout_snippet(t: &[u8], k: usize) -> String {
    String::from_utf8_lossy(&t[k.min(t.len())..(k + 40).min(t.len())]).into_owned()
}

/// Side-by-side comparison of a printout writer: the original writes into one scratch file and the copy into
/// another (the exe's own `fopen`, binary "w+b", so no newline is translated), each from the same [`PRINTOUT_LINE`]
/// and [`PRINTOUT_SCRATCH_TEXT`]; the two texts, the returns and those two buffers must match. Then the original's
/// text is copied into the real file (`fwrite`, which translates the newlines as the original's `fprintf` would have)
/// and the scratch files are removed: the real file gets the text once. Without scratch files only the original runs.
pub(crate) unsafe fn compare_printout_text(file: usize, what: impl FnOnce() -> String, original: impl FnOnce(usize) -> u32, rust: impl FnOnce(usize) -> u32) -> u32 {
    let (name_o, name_r) = (PRINTOUT_SCRATCH_ORIGINAL.as_ptr(), PRINTOUT_SCRATCH_COPY.as_ptr());
    let a = exe::crt_fopen(name_o as usize, b"w+b\0".as_ptr() as usize);
    let b = if a != 0 { exe::crt_fopen(name_r as usize, b"w+b\0".as_ptr() as usize) } else { 0 };
    if b == 0 {
        if a != 0 {
            exe::crt_fclose(a);
            exe::crt_remove(name_o);
        }
        return original(file);
    }
    let regions = [(PRINTOUT_LINE, 0xc8), (PRINTOUT_SCRATCH_TEXT, 0x7d0)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || original(a), || rust(b));
    let (to, tr) = (printout_file_text(a), printout_file_text(b));
    let k = to.iter().zip(tr.iter()).position(|(x, y)| x != y).unwrap_or(to.len().min(tr.len()));
    SCOREBOARD.record(equal && ro == rr && to == tr, || {
        format!("{}: return {} x {}, text {} x {} bytes, first difference at {} ({:?} x {:?}), buffers {:?}",
                what(), ro, rr, to.len(), tr.len(), k, printout_snippet(&to, k), printout_snippet(&tr, k), first_differences(&mo, &mr))
    });
    if !to.is_empty() {
        exe::crt_fwrite(to.as_ptr() as usize, 1, to.len() as u32, file);
    }
    exe::crt_fclose(a);
    exe::crt_fclose(b);
    exe::crt_remove(name_o);
    exe::crt_remove(name_r);
    ro
}

/// The competition's season label (0x6697c0, key_nation.cpp, cdecl): the start year, the competition, the buffer and
/// its size.
unsafe fn printout_comp_season(year: u32, comp: usize, out: usize, size: u32) {
    let f: unsafe extern "C" fn(u32, usize, usize, u32) -> u32 = core::mem::transmute(0x6697c0usize);
    f(year, comp, out, size);
}

/// The competition's statistics object (`Comp_Stats`) methods called by address: `get_player_stat_label` (0x4a2b10,
/// ret 0xc: the statistic, the buffer, its size), `get_sorted_player_table` (0x4a4dc0, ret 0x10: the statistic, the
/// count out, the most rows, the position mask; a malloc'd array of 0x2d-byte rows or null) and `format_player_stat`
/// (0x4a3ee0, ret 0x14: the row, the statistic, the buffer, its size, the substitutes split).
unsafe fn printout_stat_label(stats: usize, stat: u32, out: usize, size: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x4a2b10usize);
    f(stats, stat, out, size);
}

unsafe fn printout_sorted_stat_table(stats: usize, stat: u32, count: *mut i32, most: u32, positions: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32, u32) -> usize = core::mem::transmute(0x4a4dc0usize);
    f(stats, stat, count, most, positions)
}

unsafe fn printout_format_stat(stats: usize, row: usize, stat: u32, out: usize, size: u32, split: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x4a3ee0usize);
    f(stats, row, stat, out, size, split);
}

/// The kind of competition whose statistics list nations instead of clubs (`+0x42` of the competition object: 4,
/// the international ones) [inference from the column titles].
const PRINTOUT_NATIONAL_COMP: u8 = 4;

/// Rule: a competition's statistics table, the leading 30 players of one statistic.
///
/// `write_comp_stat_table(comp, table, file)`. The competition object is the entry of [`G_COMP_OBJECT_TABLE`] for
/// the competition's id (none: the printouts.cpp 1462 box and 0) and its statistics object is at `+8` (none: the 1470
/// box and 0). The heading " <%s - season> <%s - stat>" gets the competition's season from the printout's start year
/// (0x6697c0) and the statistic's label (`get_player_stat_label`, into the money text buffer); then the column titles
/// with the statistic's short name (2 "Gls", 5 "Asts", 0x11 "Av R", 0xc "MoM", any other "???"), the "Nation" column
/// when the competition object's kind (`+0x42`) is 4, else "Club", and the dashes. The rows come from
/// `get_sorted_player_table(table, &count, 30, all positions)`: none ("count" 0) prints "No stats available"; each
/// row whose staff id (`+4`) is not −1 prints its position (`utils_format_ordinal(k + 1, −1)`, 4 wide), the staff's
/// name (style 1, 25 wide), the nation's or the club's short name (none: empty; 25 wide), the appearances (statistic
/// 1 with the substitutes split, 7 wide) and the statistic itself (5 wide). The table is freed and the section ends
/// (two newlines and a lone space). Returns 1.
/// Original: `printouts_write_comp_stat_table` (0x7d9720, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_comp_stat_table(comp: usize, table: u32, file: usize) -> u32 {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if object == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x5b6);
        return 0;
    }
    let stats = read_usize(object + 8);
    if stats == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x5be);
        return 0;
    }
    covered!("tel25_7 competition statistics table");
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_comp_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, comp, t, 0xc8);
    printout_stat_label(stats, table, PRINTOUT_SCRATCH_TEXT, 0x7d0);
    comp_lang_format()(PRINTOUT_LINE, 0xa791a4, t, PRINTOUT_SCRATCH_TEXT);
    printout_write_heading(file);
    match table as u8 {
        2 => { comp_lang_format()(t, 0xa78490); }
        5 => { comp_lang_format()(t, 0xa7845c); }
        0x11 => { comp_lang_format()(t, 0xa78434); }
        0xc => { comp_lang_format()(t, 0xa7843c); }
        _ => printout_copy(t, 0xa14f84),
    }
    let national = read_u8(object + 0x42) == PRINTOUT_NATIONAL_COMP;
    printout_write_titles(file, if national { 0xa79150 } else { 0xa790fc }, t, 0xa790b0);
    let mut count = 0i32;
    let rows = printout_sorted_stat_table(stats, table, &mut count, 0x1e, 0xffff_ffff);
    if count == 0 {
        comp_lang_format()(PRINTOUT_LINE, 0xa7909c);
        printout_write_row(file);
    } else {
        for k in 0..count.max(0) {
            let row = rows + k as usize * 0x2d;
            let id = read_i32(row + 4);
            if id == -1 {
                continue;
            }
            let person = read_usize(G_STAFF_TABLE) + id as isize as usize * staff::SIZE;
            write_u8(PRINTOUT_LINE, 0);
            printout_copy(t, exe::utils_format_ordinal(k + 1, -1));
            printout_column(t, 4);
            exe::database_format_staff_name(person, t, 0xc8, 1);
            printout_column(t, 0x19);
            let (owner, name) = if national { (read_usize(person + staff::NATION), nation::SHORT_NAME) } else { (read_usize(person + staff::CLUB_JOB), club::SHORT_NAME) };
            if owner != 0 {
                printout_copy(t, owner + name);
            } else {
                write_u8(t, 0);
            }
            printout_column(t, 0x19);
            printout_format_stat(stats, row, 1, t, 0xc8, 1);
            printout_column(t, 7);
            printout_format_stat(stats, row, table, t, 0xc8, 1);
            printout_column(t, 5);
            printout_write_row(file);
        }
    }
    if rows != 0 {
        exe::crt_free(rows);
    }
    printout_end_section(file);
    1
}

/// Comparer for `printouts_write_comp_stat_table`: side by side over the text written (`compare_printout_text`);
/// without the competition object or its statistics (the error boxes) only the original runs.
pub(crate) unsafe extern "C" fn compare_printouts_write_comp_stat_table(comp: usize, table: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I0%%);
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if object == 0 || read_usize(object + 8) == 0 {
        return o(comp, table, file);
    }
    compare_printout_text(file, || format!("competition statistics printout ({:#x}, table {:#x})", comp, table as u8),
                          |f| o(comp, table, f), |f| printouts_write_comp_stat_table(comp, table, f))
}

/// "%d" (0x98806c), "-" (0x98ba0c) and the empty text (0xad9c64) the comments of the language files take.
pub(crate) const PRINTOUT_FORMAT_NUMBER: usize = 0x98806c;
pub(crate) const PRINTOUT_DASH: usize = 0x98ba0c;
pub(crate) const PRINTOUT_NO_TEXT: usize = 0xad9c64;
/// The dashes under the league tables' and the squad statistics' column titles (0xa78a7c).
pub(crate) const PRINTOUT_WIDE_DASHES: usize = 0xa78a7c;

/// The kind of competition object whose league tables show each club's nation (`+0x42`: 2) [inference: the
/// three-letter column only filled there].
const PRINTOUT_CLUBS_OF_NATIONS_COMP: u8 = 2;

/// A stage of the competition object that is a league (`+0x43` of the stage: 2).
const PRINTOUT_LEAGUE_STAGE: u8 = 2;

/// The numeric columns of a league table row after the team and its nation (3 wide each, signed): played (`+0x11`),
/// home won, drawn (or won on penalties, or by shoot-out), lost, for and against (`+0x20`, `+0x21`, `+0x22` bytes,
/// `+0x23`, `+0x25` words), away the same (`+0x2e`, `+0x2f`, `+0x30` bytes, `+0x31`, `+0x33` words) and the points
/// (`+0x19` word): (offset, is a word) [inference from the column titles].
const PRINTOUT_TABLE_COLUMNS: [(usize, bool); 12] = [
    (0x11, false), (0x20, false), (0x21, false), (0x22, false), (0x23, true), (0x25, true),
    (0x2e, false), (0x2f, false), (0x30, false), (0x31, true), (0x33, true), (0x19, true),
];

/// Whether a dashes line goes before row `k` of a league stage of `rows` rows: after the first row when the stage
/// has no promotion places (`+0xbe`) and flag 1 of `+0xd9`; after the promotion places (`+0xbe`), after the play-off
/// places that follow them (`+0xbf`), before the relegation play-off places (`+0xc0`) and before the relegation
/// places (`+0xc1`) [inference from the row marks].
unsafe fn printout_table_rule_before(stage: usize, k: i8, rows: i16) -> bool {
    let (k, rows) = (k as i32, rows as i32);
    let up = read_i8(stage + 0xbe) as i32;
    if up == 0 && read_u8(stage + 0xd9) & 1 != 0 && k == 1 {
        return true;
    }
    let playoff = read_i8(stage + 0xbf) as i32;
    let down_playoff = read_i8(stage + 0xc0) as i32;
    let down = read_i8(stage + 0xc1) as i32;
    (up > 0 && k == up) || (playoff > 0 && k == playoff + up) || (down_playoff > 0 && k == rows - down - down_playoff) || (down > 0 && k == rows - down)
}

/// The mark of a league table row by its fate (`+0x37` of the row): 5 champions "C", 6/7/8 qualified "Q", 0
/// promoted "P", 1/2 play-offs "Pl", 3 relegated "R", any other none.
unsafe fn printout_table_mark(fate: u8, t: usize) {
    let format = match fate {
        5 => 0xa78f10,
        6 | 7 | 8 => 0x99c1b8,
        0 => 0x99c20c,
        1 | 2 => 0x99c1d4,
        3 => 0x99c1f0,
        _ => {
            write_u8(t, 0);
            return;
        }
    };
    comp_lang_format()(t, format, PRINTOUT_NO_TEXT);
}

/// Rule: a competition's league tables, one per league stage, sorted first.
///
/// `write_league_tables(comp, file)`. The competition object is the entry of [`G_COMP_OBJECT_TABLE`] for the
/// competition's id (none: the printouts.cpp 1155 box and 0). Its stages −1 (the object itself) to `+0x30`
/// (`Comp::get_stage`) that are leagues (`+0x43` 2) are counted: more than one makes the heading " <%s - season>
/// Tables", otherwise "... Table" (the competition's season of the printout's start year). For each league stage:
/// with more than one, its name (`comp_util_get_stage_name` of `+0x3a`, or "Table" without one) and two newlines; the
/// column titles by the stage's flags (`+0xab`: 0x20 shoot-out wins "SOW", 1 penalty wins "Pen", else draws "Drn") and
/// the dashes. A stage whose draw date (day `+0xb6`, year `+0xb8` + `+0x40`; `+0xb6` 0xffff: none) is still after
/// today prints "Draw not yet made"; otherwise `+0xad` is cleared, the table sorted (`League::sort_table(stage, 0)`)
/// and each of its `+0x3e` rows (`League::get_table_row`) printed, with a dashes line before the rows where the
/// places change (`printout_table_rule_before`): the position (`utils_format_ordinal(+4 + 1, −1)`, 4 wide, one space),
/// the fate mark (2 wide, one space), the club's short name (25 wide), its nation's three letters when the object's
/// kind (`+0x42`) is 2 (3 wide), and the numbers of `PRINTOUT_TABLE_COLUMNS` (3 wide each). Each stage printed ends
/// with two newlines; the printout ends with a lone space. Returns 1.
/// Original: `printouts_write_league_tables` (0x7d8120, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_league_tables(comp: usize, file: usize) -> u32 {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    if object == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x483);
        return 0;
    }
    let mut leagues = 0i8;
    if read_i32(object + 0x30) >= -1 {
        let mut s = -1i8;
        loop {
            let stage = get_stage(object, s as i32);
            if stage != 0 && read_u8(stage + 0x43) == PRINTOUT_LEAGUE_STAGE {
                leagues = leagues.wrapping_add(1);
            }
            s = s.wrapping_add(1);
            if s as i32 > read_i32(object + 0x30) {
                break;
            }
        }
    }
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_comp_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, comp, t, 0xc8);
    comp_lang_format()(PRINTOUT_LINE, if leagues > 1 { 0xa79084 } else { 0xa7906c }, t);
    printout_write_heading(file);
    if read_i32(object + 0x30) >= -1 {
        let mut s = -1i8;
        loop {
            let stage = get_stage(object, s as i32);
            if stage != 0 && read_u8(stage + 0x43) == PRINTOUT_LEAGUE_STAGE {
                covered!("tel25_7 league table");
                if leagues > 1 {
                    let name = read_u16(stage + 0x3a);
                    if name != 0 {
                        exe::comp_util_get_stage_name(PRINTOUT_LINE as *mut u8, 0xc8, name as u32, 0);
                    } else {
                        comp_lang_format()(PRINTOUT_LINE, 0x99c144);
                    }
                    printout_write_all(file, &[PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE]);
                }
                let flags = read_u16(stage + 0xab);
                let titles = if flags & 0x20 != 0 { 0xa79008 } else if flags & 1 != 0 { 0xa78fa4 } else { 0xa78f40 };
                printout_write_titles(file, titles, 0, PRINTOUT_WIDE_DASHES);
                let draw_day = read_u16(stage + 0xb6);
                let mut not_drawn = false;
                if draw_day != 0xffff {
                    let mut date = [0u8; 8];
                    let year = read_u16(stage + 0xb8).wrapping_add(read_u16(stage + 0x40));
                    let d = exe::new_date(date.as_mut_ptr() as usize, draw_day as u32, year as u32);
                    let (today_day, today_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
                    not_drawn = today_year < read_u16(d + 2) || (today_year == read_u16(d + 2) && today_day < read_i16(d));
                }
                if not_drawn {
                    covered!("tel25_7 draw not yet made");
                    comp_lang_format()(PRINTOUT_LINE, 0xa78f2c);
                    printout_write_row(file);
                } else {
                    write_u32(stage + 0xad, 0);
                    sort_table(stage, 0);
                    let mut k = 0i8;
                    while (k as i16) < read_i16(stage + 0x3e) {
                        if printout_table_rule_before(stage, k, read_i16(stage + 0x3e)) {
                            printout_write_all(file, &[PRINTOUT_WIDE_DASHES, PRINTOUT_NEWLINE]);
                        }
                        let row = league_record(stage, k as i32);
                        let team = read_usize(row);
                        write_u8(PRINTOUT_LINE, 0);
                        printout_copy(t, exe::utils_format_ordinal(read_i8(row + 4) as i32 + 1, -1));
                        printouts_pad_string(t, 4);
                        printout_append(PRINTOUT_LINE, t);
                        printout_append(PRINTOUT_LINE, PRINTOUT_PAGE_BREAK);
                        printout_table_mark(read_u8(row + 0x37), t);
                        printouts_pad_string(t, 2);
                        printout_append(PRINTOUT_LINE, t);
                        printout_append(PRINTOUT_LINE, PRINTOUT_PAGE_BREAK);
                        printout_copy(t, team + club::SHORT_NAME);
                        printout_column(t, 0x19);
                        let nation = read_usize(team + club::NATION);
                        if read_u8(object + 0x42) == PRINTOUT_CLUBS_OF_NATIONS_COMP && nation != 0 {
                            printout_copy(t, nation + nation::THREE_LETTER_NAME);
                        } else {
                            write_u8(t, 0);
                        }
                        printout_column(t, 3);
                        for (at, word) in PRINTOUT_TABLE_COLUMNS {
                            let n = if word { read_i16(row + at) as i32 } else { read_i8(row + at) as i32 };
                            comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, n);
                            printout_column(t, 3);
                        }
                        printout_write_row(file);
                        k = k.wrapping_add(1);
                    }
                }
                printout_write(file, PRINTOUT_NEWLINE);
                printout_write(file, PRINTOUT_NEWLINE);
            }
            s = s.wrapping_add(1);
            if s as i32 > read_i32(object + 0x30) {
                break;
            }
        }
    }
    printout_fprintf()(file, PRINTOUT_FORMAT_LINE, PRINTOUT_PAGE_BREAK);
    1
}

/// Comparer for `printouts_write_league_tables`: side by side over the text written (`compare_printout_text`); the
/// original sorts the tables first and the copy sorts them again, finding them in order. Without the competition
/// object (the error box) only the original runs.
pub(crate) unsafe extern "C" fn compare_printouts_write_league_tables(comp: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4) == 0 {
        return o(comp, file);
    }
    compare_printout_text(file, || format!("league tables printout ({:#x})", comp), |f| o(comp, f), |f| printouts_write_league_tables(comp, f))
}
