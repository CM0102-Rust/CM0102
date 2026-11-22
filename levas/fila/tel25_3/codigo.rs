// ---------- batch %%LEVA%%: the printouts (1): padding a text, the transfer order by date and a competition's printout (printouts.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\printouts.cpp", the path the printouts.cpp error boxes print.
pub(crate) const PRINTOUTS_CPP: usize = 0xa78a50;
/// "~printout.txt", the file a printout goes to when no file is given (it is printed and removed).
pub(crate) const PRINTOUT_TEMP_FILE: usize = 0xa78a40;
/// The period a printout covers: its start [0xb74360] and end [0xb74358] (8-byte dates, the end a year after the
/// start), and the printout's line buffer [0xb74368] (0xc8 bytes).
pub(crate) const PRINTOUT_PERIOD_START: usize = 0xb74360;
pub(crate) const PRINTOUT_PERIOD_END: usize = 0xb74358;
pub(crate) const PRINTOUT_LINE: usize = 0xb74368;
/// The row of asterisks above and below a printout's heading (0xa789bc) and "\n" (0x99cae4).
const PRINTOUT_STARS: usize = 0xa789bc;
const PRINTOUT_NEWLINE: usize = 0x99cae4;

/// The text into the file through the exe's `fprintf` (0x9464f0, cdecl: the file, then the text as the format).
pub(crate) unsafe fn printout_put(text: usize, file: usize) {
    // fprintf(file, format) (0x9464f0, cdecl): at 0x7d7e6e the original pushes the text, then the file (esi)
    let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x9464f0usize);
    f(file, text);
}

/// `fgets` (0x94700c, cdecl): a line of at most `size` − 1 bytes into `buffer`; 0 at the end of the file.
pub(crate) unsafe fn printout_get_line(buffer: usize, size: u32, file: usize) -> usize {
    let f: unsafe extern "C" fn(usize, u32, usize) -> usize = core::mem::transmute(0x94700cusize);
    f(buffer, size, file)
}

/// The length of the C string at `s`.
unsafe fn printout_text_length(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// Rule: a text is padded with spaces up to a width, or cut at it.
///
/// No text, or a width below 1: the printouts.cpp 1600 error box and 0. A text shorter than the width gets spaces
/// (each followed by the terminator) up to it; a longer or equal one is cut at the width (the terminator written
/// there). Returns the text.
/// Original: `printouts_pad_string` (0x7da070, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_pad_string(text: usize, width: u32) -> usize {
    if text == 0 || width < 1 {
        tel26_error_box(PRINTOUTS_CPP, 0x640);
        return 0;
    }
    let width = width as usize;
    let mut n = printout_text_length(text);
    if n >= width {
        write_u8(text + width, 0);
        return text;
    }
    covered!("tel25_3 pad a printout text");
    while n < width {
        write_u8(text + n, b' ');
        n += 1;
        write_u8(text + n, 0);
    }
    text
}

/// Comparer for `printouts_pad_string`: side by side over the text (up to the longer of its length and the width, and
/// the terminator) and the return; the error box (no text or no width) runs the original only.
pub(crate) unsafe extern "C" fn compare_printouts_pad_string(text: usize, width: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I0%%);
    if text == 0 || width < 1 {
        return o(text, width);
    }
    let regions = [(text, printout_text_length(text).max(width as usize) + 1)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(text, width), || printouts_pad_string(text, width));
    SCOREBOARD.record(equal && ro == rr, || format!("printouts_pad_string(width {}): return {:#x} x {:#x}, differences in {:?}", width, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: the club transfers printout lists its transfers by date, earliest first.
///
/// `a` and `b` are two past transfer records (qsort over the records themselves). No first: 1; no second: −1; then
/// the dates at +0x1d (8 bytes): the first earlier than the second (`Date::operator<` 0x456770): −1; later
/// (`Date::operator>` 0x456740): 1; the same day: 0.
/// Original: `printouts_compare_by_date` (0x7da130, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_compare_by_date(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    if exe::date_operator_3c(a + 0x1d, read_u32(b + 0x1d), read_u32(b + 0x21)) != 0 {
        covered!("tel25_3 transfer earlier");
        return -1;
    }
    (exe::date_operator_3e(a + 0x1d, read_u32(b + 0x1d), read_u32(b + 0x21)) != 0) as i32
}

compare_order!(compare_printouts_compare_by_date, printouts_compare_by_date, %%I1%%);

/// The period `printouts_print_comp_report` covers for a competition: (day, year) of its start, from the nation's
/// season end day (`key_nation_record::SEASON_END_DAY` of the competition's entry in [`G_LEAGUE_TABLE`], 0x48 bytes
/// each, when the entry has its first dword and `SPLIT_SEASON_FLAG`): the last such day not after today (today before
/// it: last year's); without them, day 0 of this year.
unsafe fn printout_comp_period_start(comp: usize) -> (u32, u32) {
    let entry = G_LEAGUE_TABLE + exe::key_nation_index_of_comp(comp) as i8 as isize as usize * 0x48;
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    if read_usize(entry) != 0 && read_u32(entry + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
        let day = read_u16(entry + key_nation_record::SEASON_END_DAY);
        if (read_i16(G_CURRENT_DATE) as i32) < day as i32 {
            return (day as u32, year.wrapping_sub(1) & 0xffff);
        }
        return (day as u32, year);
    }
    (0, year)
}

/// Rule: a competition's printout: its tables and statistics written to a file (by default "~printout.txt"), then
/// sent to the printer when asked; the temporary file is removed.
///
/// `print_comp_report(comp, kind, file, print)`. No competition: the printouts.cpp 1000 box and 0. The period is set
/// first: [`PRINTOUT_PERIOD_START`] from `printout_comp_period_start` and [`PRINTOUT_PERIOD_END`] a year later (both
/// through `Date::Date` and `Date::operator=`). The file opens for writing ("w"; failing: the 1025 box and 0). The
/// heading " <%s - comp> - {}<%s - date>" (langlib_sprintf with 0x64, the competition's name +4 and today's long date)
/// goes into [`PRINTOUT_LINE`]; the file gets a row of asterisks, a newline, the heading, a newline, the asterisks and
/// four newlines. Then by the bits of `kind`: 1 the league tables (`printouts_write_league_tables` 0x7d8120), 2, 4, 8
/// and 0x10 the statistics tables 2, 5, 0x11 and 0xc (`printouts_write_comp_stat_table` 0x7d9720); any of them
/// returning 0 makes the result 0 (otherwise 1). The file is closed. With `print`: it opens again for reading ("r";
/// failing: the 1094 box and 0), a print job of 0x8c starts and each line read (0xc8 bytes, its final newline cut)
/// becomes a page break when it is a single space, otherwise a printed line; then the job is printed and ended and the
/// file closed. Without a file given, "~printout.txt" is removed. Returns the result.
/// Original: `printouts_print_comp_report` (0x7d7c80, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_print_comp_report(comp: usize, kind: u32, file: usize, print: u32) -> u32 {
    if comp == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x3e8);
        return 0;
    }
    let (day, year) = printout_comp_period_start(comp);
    let (mut date, mut out) = ([0u8; 8], [0u8; 8]);
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    exe::date_operator_3d(PRINTOUT_PERIOD_START, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    exe::new_date(date.as_mut_ptr() as usize, read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2).wrapping_add(1) as u32);
    exe::date_operator_3d(PRINTOUT_PERIOD_END, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    let name = if file != 0 { file } else { PRINTOUT_TEMP_FILE };
    let f = exe::crt_fopen(name, 0x99c370usize);
    if f == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x401);
        return 0;
    }
    covered!("tel25_3 competition printout");
    let mut today = [0u8; 0xc8];
    exe::date_get_long_date_string(G_CURRENT_DATE, today.as_mut_ptr() as usize, 0xc8, 0xff);
    comp_lang_format()(PRINTOUT_LINE, 0xa78ef0, 0x64u32, comp + 4, today.as_ptr());
    for text in [PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE] {
        printout_put(text, f);
    }
    let mut result = 1;
    if kind & 1 != 0 {
        let tables: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7d8120usize);
        if tables(comp, f) == 0 {
            result = 0;
        }
    }
    let stats: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x7d9720usize);
    for (bit, table) in [(2u32, 2u32), (4, 5), (8, 0x11), (0x10, 0xc)] {
        if kind & bit != 0 && stats(comp, table, f) == 0 {
            result = 0;
        }
    }
    exe::crt_fclose(f);
    if print != 0 {
        let f = exe::crt_fopen(name, 0x9899fcusize);
        if f == 0 {
            tel26_error_box(PRINTOUTS_CPP, 0x446);
            return 0;
        }
        covered!("tel25_3 print the competition printout");
        let begin: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8033e0usize);
        let line: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x803710usize);
        let page_break: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x803930usize);
        let print_job: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x803a90usize);
        let end: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8035b0usize);
        begin(G_SCREEN_MANAGER, 0x8c);
        while printout_get_line(PRINTOUT_LINE, 0xc8, f) != 0 {
            let n = printout_text_length(PRINTOUT_LINE);
            if n != 0 && read_u8(PRINTOUT_LINE + n - 1) == b'\n' {
                write_u8(PRINTOUT_LINE + n - 1, 0);
            }
            if read_u8(PRINTOUT_LINE) == b' ' && read_u8(PRINTOUT_LINE + 1) == 0 {
                page_break(G_SCREEN_MANAGER);
            } else {
                line(G_SCREEN_MANAGER, PRINTOUT_LINE);
            }
        }
        print_job(G_SCREEN_MANAGER);
        end(G_SCREEN_MANAGER);
        exe::crt_fclose(f);
    }
    if file == 0 {
        exe::crt_remove(PRINTOUT_TEMP_FILE as *const u8);
    }
    result
}

/// Comparer for `printouts_print_comp_report`: only the original runs (it writes a file and may print), and the period
/// it leaves ([`PRINTOUT_PERIOD_START`], [`PRINTOUT_PERIOD_END`]) is checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_printouts_print_comp_report(comp: usize, kind: u32, file: usize, print: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32) -> u32>(%%I2%%);
    if comp == 0 {
        return o(comp, kind, file, print);
    }
    let (day, year) = printout_comp_period_start(comp);
    let r = o(comp, kind, file, print);
    let got = (read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2) as u32,
               read_u16(PRINTOUT_PERIOD_END) as u32, read_u16(PRINTOUT_PERIOD_END + 2) as u32);
    let plan = (day, year, day, (year + 1) & 0xffff);
    SCOREBOARD.record(got == plan, || format!("competition printout ({:#x}, kind {:#x}): period {:?} x plan {:?}", comp, kind, got, plan));
    r
}
