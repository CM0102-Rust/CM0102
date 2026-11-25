// ---------- rx13_18: printing the print list's lines (simulated_stats.cpp) ----------

/// The Windows imports `print_text_lines` calls through the game's import table (all stdcall): PrintDlgA,
/// StartDocA, GetDeviceCaps, CreateFontA, GetTextMetricsA, StartPage, SelectObject, TextOutA, EndPage, DeleteObject,
/// EndDoc.
const IAT_PRINT_DLG_A: usize = 0x967260;
const IAT_START_DOC_A: usize = 0x967040;
const IAT_GET_DEVICE_CAPS: usize = 0x96701c;
const IAT_CREATE_FONT_A: usize = 0x967020;
const IAT_GET_TEXT_METRICS_A: usize = 0x967024;
const IAT_START_PAGE: usize = 0x96702c;
const IAT_SELECT_OBJECT: usize = 0x967030;
const IAT_TEXT_OUT_A: usize = 0x967034;
const IAT_END_PAGE: usize = 0x967028;
const IAT_DELETE_OBJECT: usize = 0x967038;
const IAT_END_DOC: usize = 0x96703c;
/// The print dialog's owner window (the game's main window) [inference].
const PRINT_OWNER_WINDOW: usize = 0xb67a2c;
/// The document name "..." (0xa827dc) and the face "Courier" (0xa827d4).
const PRINT_DOC_NAME: usize = 0xa827dc;
const PRINT_FONT_FACE: usize = 0xa827d4;
/// GetDeviceCaps indices: HORZRES, VERTRES, PHYSICALOFFSETX, PHYSICALOFFSETY.
const HORZRES: i32 = 8;
const VERTRES: i32 = 0xa;
const PHYSICALOFFSETX: i32 = 0x70;
const PHYSICALOFFSETY: i32 = 0x71;

/// GetDeviceCaps(`dc`, `index`) through the game's import.
unsafe fn device_caps(dc: usize, index: i32) -> i32 {
    let f: unsafe extern "system" fn(usize, i32) -> i32 = core::mem::transmute(read_usize(IAT_GET_DEVICE_CAPS));
    f(dc, index)
}

/// rule: printing a list of text lines: the Windows print dialog, then the lines in Courier, as many characters across
/// the page as asked, one under the other from the printable top; a page is started when the next line would pass the
/// page's height, and a null line (not the last) starts a new page
///
/// `print_text_lines(lines, count, chars)` (the object in ecx is not read). The print dialog (PrintDlgA with a 0x42
/// byte PRINTDLGA: the owner window, `PD_RETURNDC` 0x100, from/to page 1, one copy); cancelled, nothing. StartDocA
/// (DOCINFOA of 0x14 bytes with the document name); failing, nothing more. Without `lines`, only EndDoc. The font
/// (CreateFontA: height 0, width HORZRES / `chars` as a signed word, the rest 0, "Courier"); none, EndDoc. The text
/// metrics (GetTextMetricsA); failing, or no lines, the font deleted and EndDoc. Then for each line k: the first, a
/// null line before the last, or a line whose bottom (y + tmHeight − PHYSICALOFFSETY − 1) reaches VERTRES starts a page
/// (EndPage of the open one, StartPage; failing, the font deleted and EndDoc; SelectObject of the font; y =
/// PHYSICALOFFSETY); a non-null line is written (TextOutA at x = PHYSICALOFFSETX, y, its length) and y moves down
/// tmHeight. At the end, EndPage of the open page, DeleteObject of the font and EndDoc. Defect of the original: the
/// printer DC and the dialog's hDevMode and hDevNames are never freed.
/// Original: `print_text_lines` (0x84c830, simulated_stats.cpp, stdcall (ecx unread), ret 0xc).
pub(crate) unsafe extern "stdcall" fn print_text_lines(lines: usize, count: i32, chars: u32) {
    let mut pd = [0u8; 0x42];
    let p = pd.as_mut_ptr() as usize;
    write_u32(p, 0x42);
    write_usize(p + 4, read_usize(PRINT_OWNER_WINDOW));
    write_u32(p + 0x14, 0x100); // PD_RETURNDC
    write_u16(p + 0x18, 1);
    write_u16(p + 0x1a, 1);
    write_u16(p + 0x20, 1);
    let print_dlg: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(IAT_PRINT_DLG_A));
    if print_dlg(p) == 0 {
        return;
    }
    covered!("rx13_18 print dialog accepted");
    let dc = read_usize(p + 0x10);
    let doc: [u32; 5] = [0x14, PRINT_DOC_NAME as u32, 0, 0, 0];
    let start_doc: unsafe extern "system" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(IAT_START_DOC_A));
    if start_doc(dc, doc.as_ptr() as usize) <= 0 {
        return;
    }
    if lines != 0 {
        print_lines_on(dc, lines, count, chars);
    }
    let end_doc: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(IAT_END_DOC));
    end_doc(dc);
}

/// The body of `print_text_lines` between StartDocA and EndDoc: the font, the metrics, the pages and the lines, and the
/// font deleted when it was made.
unsafe fn print_lines_on(dc: usize, lines: usize, count: i32, chars: u32) {
    let width = device_caps(dc, HORZRES) / (chars as i16 as i32);
    let create_font: unsafe extern "system" fn(i32, i32, i32, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(read_usize(IAT_CREATE_FONT_A));
    let font = create_font(0, width, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, PRINT_FONT_FACE);
    if font == 0 {
        return;
    }
    let mut tm = [0u8; 0x38];
    let metrics: unsafe extern "system" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(IAT_GET_TEXT_METRICS_A));
    if metrics(dc, tm.as_mut_ptr() as usize) != 0 && count > 0 {
        let height = read_i32(tm.as_ptr() as usize); // tmHeight
        let page = |name: usize| -> unsafe extern "system" fn(usize) -> i32 { core::mem::transmute(read_usize(name)) };
        let (mut open, mut y) = (false, 0i32);
        for k in 0..count {
            let line = read_usize(lines + k as usize * 4);
            let new_page = if k == 0 {
                true
            } else if line == 0 {
                k < count - 1 // rx13_18: a null line breaks the page
            } else {
                let bottom = height + y - device_caps(dc, PHYSICALOFFSETY) - 1;
                bottom >= device_caps(dc, VERTRES)
            };
            if new_page {
                if open {
                    page(IAT_END_PAGE)(dc);
                }
                open = page(IAT_START_PAGE)(dc) > 0;
                if !open {
                    break;
                }
                let select: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(read_usize(IAT_SELECT_OBJECT));
                select(dc, font);
                y = device_caps(dc, PHYSICALOFFSETY);
            }
            if line != 0 {
                let mut len = 0usize;
                while read_u8(line + len) != 0 {
                    len += 1;
                }
                let x = device_caps(dc, PHYSICALOFFSETX);
                let text_out: unsafe extern "system" fn(usize, i32, i32, usize, i32) -> i32 = core::mem::transmute(read_usize(IAT_TEXT_OUT_A));
                text_out(dc, x, y, line, len as i32);
                y += height;
            }
        }
        if open {
            page(IAT_END_PAGE)(dc);
        }
    }
    let delete: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(IAT_DELETE_OBJECT));
    delete(font);
}

/// Only the original (it prints).
pub(crate) unsafe extern "stdcall" fn compare_print_text_lines(lines: usize, count: i32, chars: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, i32, u32)>(%%I0%%)(lines, count, chars)
}
