// --- sx16_7: the news store: News::News (0x785b20), News::~News (0x785d00), News::load_from_zip (0x787780) ---

/// The game CD check of the constructors (News, OfficialsManager, PLAYER_REGEN): among the drives
/// (GetLogicalDriveStringsA, 200 bytes) the first CD drive (GetDriveTypeA 5) holding "cm0102.exe" (`file_find_first`
/// answering 0; otherwise the search is closed with `file_find_close`); none: the "Insert CD" box (`langlib_sprintf` of
/// 0x987df4 and 0x987dc4, the box with 1) and again, forever. `drive_jne` and `found_test` are the addresses of the
/// drive type jump (`jne`) and of the found test (`test eax, eax`) in the constructor: the no-CD exes (cm0102-1080)
/// make the first a nop and the second a xor, and the copy reads those bytes from the running exe.
pub(crate) unsafe fn game_cd_check(drive_jne: usize, found_test: usize) {
    let drive_strings: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let drive_type: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_jne) == 0x90;
    let always_found = read_u8(found_test) != 0x85; // the original has `test eax, eax` (85 c0)
    loop {
        let mut drives = [0u8; 0xc8];
        if drive_strings(0xc8, drives.as_mut_ptr()) != 0 && drives[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = drives.as_ptr() as usize + i;
                if any_drive || drive_type(root) == 5 {
                    let mut name = [0u8; 0x104];
                    let mut found = [0u8; 0x114];
                    let n = name.as_mut_ptr() as usize;
                    // "%s" + "cm0102.exe" passed as ten characters, as the original does
                    sprintf(n, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(n, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with a signed byte (steps of 4); it would only pass 127 with more
                // than 32 drives
                i += 4;
                if i >= drives.len() || drives[i] == 0 {
                    break;
                }
            }
        }
        let mut title = [0u8; 0x100];
        let mut message = [0u8; 0x100];
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The source path of news.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\news.cpp").
const NEWS_CPP_PATH: usize = 0xa693ec;
/// The save item "news.dat".
const NEWS_DAT: usize = 0xa69490;
/// The news item size (223 bytes) and the inbox record size (start, position, count).
const NEWS_ITEM_SIZE: u32 = 0xdf;
const NEWS_INBOX_SIZE: u32 = 0xc;
/// The inbox count of a new store: one per human manager slot.
const NEWS_INBOXES: u32 = 0x10;

/// The news.cpp "Error" box at `row` that stops the game (the fatal cleanup and exit(-1)).
unsafe fn news_fatal(row: u32) -> ! {
    show_error_box(NEWS_CPP_PATH, row);
    fatal_exit()
}

/// Builds the news store: first the game CD (`game_cd_check`), then, with a save, the store read from it
/// (`News::load_from_zip` through the exe); without a save or when the reading fails, an empty store: no items
/// (`news_fields::SLOT_COUNT` 0 and the item pointer 0), 16 inboxes of 12 bytes (`malloc(0xc0)`, each start and
/// position −1 and count 0; no memory: the fatal box at line 0x1ef9) and the last news id 0. Returns the store.
/// Defect of the original: a store whose reading failed halfway keeps the buffers it had allocated (they leak).
/// Original: `News::News` (0x785b20, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_news(this: usize, save: usize) -> usize {
    game_cd_check(0x785b5a, 0x785b9e);
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x787780usize); // News::load_from_zip
    if save != 0 && load(this, save) != 0 {
        covered!("sx16_7 news from the save");
        return this;
    }
    covered!("sx16_7 empty news store");
    write_u32(this + news_fields::SLOT_COUNT, 0);
    write_usize(this, 0); // the item store
    write_u32(this + 0xc, NEWS_INBOXES); // the inbox count
    let inboxes = exe::crt_malloc(NEWS_INBOXES * NEWS_INBOX_SIZE);
    write_usize(this + news_fields::HUMAN_INBOXES, inboxes);
    if inboxes == 0 {
        news_fatal(0x1ef9);
    }
    for k in 0..read_i32(this + 0xc).max(0) as usize {
        let inbox = inboxes + k * NEWS_INBOX_SIZE as usize;
        write_i32(inbox, -1);
        write_i32(inbox + 4, -1);
        write_u32(inbox + 8, 0);
    }
    write_u32(this + news_fields::LAST_ID, 0);
    this
}

/// Comparer for `new_news`: it asks for the CD, allocates and may read the save, so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_news(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Frees the news store: the item store (its pointer and `news_fields::SLOT_COUNT` zeroed) and the inboxes (pointer
/// and count zeroed), each when present.
/// Original: `News::~News` (0x785d00, news.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_news(this: usize) {
    let items = read_usize(this);
    if items != 0 {
        exe::crt_free(items);
        write_usize(this, 0);
        write_u32(this + news_fields::SLOT_COUNT, 0);
    }
    let inboxes = read_usize(this + news_fields::HUMAN_INBOXES);
    if inboxes != 0 {
        exe::crt_free(inboxes);
        write_usize(this + news_fields::HUMAN_INBOXES, 0);
        write_u32(this + 0xc, 0); // the inbox count
    }
}

/// Comparer for `drop_news`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Reads the news store from the save's "news.dat": the item count (`news_fields::SLOT_COUNT`), the 223-byte items
/// (`malloc`; none, the store pointer 0), the inbox count, the 12-byte inboxes and the last news id; closes the item
/// and answers 1. Without a save (box line 0x2500) or without the item (line 0x2508): the "Error" box and 0. A short
/// read or a failed allocation stops the game (fatal box, lines 0x250f, 0x251b, 0x2523, 0x2531, 0x253a, 0x2542,
/// 0x254a).
/// Original: `News::load_from_zip` (0x787780, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn news_load_from_zip(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(NEWS_CPP_PATH, 0x2500);
        return 0;
    }
    let mut z = exe::zopen(save, NEWS_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_box(NEWS_CPP_PATH, 0x2508);
        return 0;
    }
    covered!("sx16_7 load news.dat");
    let count = this + news_fields::SLOT_COUNT;
    if exe::zread(count, 4, 1, z) != 1 {
        news_fatal(0x250f);
    }
    if read_u32(count) != 0 {
        let items = exe::crt_malloc(read_u32(count).wrapping_mul(NEWS_ITEM_SIZE));
        write_usize(this, items);
        if items == 0 {
            news_fatal(0x251b);
        }
        if exe::zread(items, NEWS_ITEM_SIZE, read_u32(count), z) != read_u32(count) {
            news_fatal(0x2523);
        }
    } else {
        write_usize(this, 0);
    }
    let inbox_count = this + 0xc;
    if exe::zread(inbox_count, 4, 1, z) != 1 {
        news_fatal(0x2531);
    }
    let inboxes = exe::crt_malloc(read_u32(inbox_count).wrapping_mul(NEWS_INBOX_SIZE));
    write_usize(this + news_fields::HUMAN_INBOXES, inboxes);
    if inboxes == 0 {
        news_fatal(0x253a);
    }
    if exe::zread(inboxes, NEWS_INBOX_SIZE, read_u32(inbox_count), z) != read_u32(inbox_count) {
        news_fatal(0x2542);
    }
    if exe::zread(this + news_fields::LAST_ID, 4, 1, z) != 1 {
        news_fatal(0x254a);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `news_load_from_zip`: it reads the save item and allocates, so only the original runs. The copy is
/// proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_news_load_from_zip(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
