// ---------- batch %%LEVA%%: the small routines of the screens: the background picture settings, the Text copy, a list's deleting destructor, the contract news buttons, the waiting screen, a static date's destructor and the search filter record ----------

/// u32: the fixed background flag of the game settings (Display::draw_screen uses the fixed picture when it is set).
const BACKGROUND_FIXED_FLAG: usize = 0xdd6bda;

/// Frees the pics.cfg tables: for each of the 11 sections with subsections, each subsection's two picture lists
/// (every non-null name, then the list; the list pointer and its count cleared) and its name, then the subsection
/// array (pointer and count cleared). The section names stay.
///
/// `background_free_config_tables()`: section k (`background_section`, 10 bytes): with a non-null subsection array,
/// subsection j < the count (i16): `background_sub::LIST` (when non-null: its `background_sub::COUNT` names, each
/// freed when non-null, then the list freed, the pointer and the count zeroed), the same with
/// `background_sub::SECOND_LIST` and `background_sub::SECOND_COUNT`, then the name freed when non-null (the pointer
/// is not cleared); then the array is freed and the pointer and the count zeroed. `free` of the CRT throughout.
/// Original: `background_free_config_tables` (0x41bad0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_free_config_tables() {
    for k in 0..BACKGROUND_SECTION_COUNT as i32 {
        let section = background_section(k);
        if read_usize(section + 6) == 0 {
            continue;
        }
        covered!("tel39_2 free the pictures tables");
        let mut j = 0i16;
        while j < read_i16(section + 4) {
            let sub = read_usize(section + 6) + j as usize * background_sub::SIZE;
            for (list, count) in [(background_sub::LIST, background_sub::COUNT), (background_sub::SECOND_LIST, background_sub::SECOND_COUNT)] {
                if read_usize(sub + list) == 0 {
                    continue;
                }
                let mut i = 0i16;
                while i < read_i16(sub + count) {
                    let name = read_usize(read_usize(sub + list) + i as usize * 4);
                    if name != 0 {
                        exe::crt_free(name);
                    }
                    i += 1;
                }
                exe::crt_free(read_usize(sub + list));
                write_usize(sub + list, 0);
                // tel39_2: the list's count cleared
                write_i16(sub + count, 0);
            }
            if read_usize(sub + background_sub::NAME) != 0 {
                exe::crt_free(read_usize(sub + background_sub::NAME));
            }
            j += 1;
        }
        exe::crt_free(read_usize(section + 6));
        write_usize(section + 6, 0);
        write_i16(section + 4, 0);
    }
}

/// Sets the fixed background flag of the game settings to the inverse of `on`.
///
/// `background_set_pictures_disabled(on)`: [0xdd6bda] = 1 when `on` is 0, else 0.
/// Original: `background_set_pictures_disabled` (0x41bf80, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_pictures_disabled(on: u32) {
    covered!("tel39_2 picture setting");
    // tel39_2: the inverse of the argument
    write_u32(BACKGROUND_FIXED_FLAG, (on == 0) as u32);
}

/// The inverse of the fixed background flag: 1 when [0xdd6bda] is 0 (what `background_set_pictures_disabled` got).
/// Original: `background_pictures_disabled` (0x41bfa0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_pictures_disabled() -> u32 {
    (read_u32(BACKGROUND_FIXED_FLAG) == 0) as u32
}

/// The scalar deleting destructor of a linked list: `LLIST::~LLIST`, then operator delete when bit 0 of the flags is
/// set; returns the list.
/// Original: `LLIST::scalar_deleting_destructor` (0x497910, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn llist_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    covered!("tel39_2 list deleting destructor");
    exe::drop_llist(this);
    if flags & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Constructs a Text as a copy of another: its string and each of its links.
///
/// `Text::Text(this, other)` (the other by pointer, may be null): string and link list null; with another Text, its
/// string (when there is one) through `Text::set_string`, then, with a link list, the list read from its head (the
/// other's cursor moves) and each link added through `Text::add_link(phrase, kind, data)`. Returns `this`.
/// Original: `Text::Text` (0x4ae5c0, comp_text.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn text_construct_copy(this: usize, other: usize) -> usize {
    write_usize(this + TEXT_STRING, 0);
    write_usize(this + TEXT_LINKS, 0);
    if other == 0 {
        return this;
    }
    covered!("tel39_2 copy a text");
    let s = read_usize(other + TEXT_STRING);
    if s != 0 {
        text_set_string(this, s);
    }
    if read_usize(other + TEXT_LINKS) == 0 {
        return this;
    }
    list_rewind(read_usize(other + TEXT_LINKS));
    loop {
        let e = exe::llist_get_next(read_usize(other + TEXT_LINKS));
        if e == 0 {
            break;
        }
        // tel39_2: each link with its kind and data
        text_add_link(this, e, read_u32(e + TEXT_LINK_KIND), read_u32(e + TEXT_LINK_DATA));
    }
    this
}


/// Comparer for `text_construct_copy`: the copy builds a local Text, the original the real one; string and links
/// equal.
pub(crate) unsafe extern "thiscall" fn compare_text_construct_copy(this: usize, other: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TEXT_CONSTRUCT_COPY);
    let mut local = [0usize; 2];
    text_construct_copy(local.as_mut_ptr() as usize, other);
    let r = o(this, other);
    let (mo, mr) = (text_contents(this), text_contents(local.as_ptr() as usize));
    comp_text_free(&mut local);
    SCOREBOARD.record(r == this && mo == mr, || format!("Text::Text (copy): original {:?} {:x?}, rust {:?} {:x?}", String::from_utf8_lossy(&mo.0), mo.1,
        String::from_utf8_lossy(&mr.0), mr.1));
    r
}

/// The staff record of a staff id (`staff::SIZE` × id into the staff table, in 32 bits like the original).
unsafe fn tel39_staff(id: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE as u32) as usize)
}

/// The manager's news item `id` into a local NewsItem (`NewsItem::NewsItem(0, 0)`, `News::find_item_by_id`).
unsafe fn tel39_news_item(manager: usize, id: u32, item: usize) {
    exe::new_newsitem(item, 0, 0);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, id, item);
}

/// The news type of a contract news item that comes with a transfer offer.
const NEWS_TYPE_OFFER_ACCEPTED: u32 = 0xfa4;

/// Rule: the "Offer Contract" button of a news item opens the contract offer for the player it names.
///
/// `contract_screens_news_offer_contract_action(manager, id)`: the news item (`tel39_news_item`); its slot 2 is the
/// person's id; `contract_screens_offer_contract_to_staff(person, 0, 0, 1)`.
/// Original: `contract_screens_news_offer_contract_action` (0x4dc9b0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_offer_contract_action(manager: usize, id: u32) {
    covered!("tel39_2 news offer contract");
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    tel39_news_item(manager, id, item);
    // tel39_2: the player of slot 2
    contract_screens_offer_contract_to_staff(tel39_staff(exe::newsitem_get_slot(item, 2)), 0, 0, 1);
}

/// Rule: the "Negotiate" button of a contract news item opens the negotiation with the person it names; after an
/// accepted transfer offer (news type 0xfa4) it negotiates on that offer.
///
/// `contract_screens_news_negotiate_action(manager, id)`: the news item; its slot 0 is the person's id. With the
/// item's type (+0) 0xfa4: `contract_screens_offer_contract_to_staff(person, TRANSFER_MANAGER::get_offer_slot(slot
/// 7), 1, 0)`; otherwise `(person, 0, 0, 0)`.
/// Original: `contract_screens_news_negotiate_action` (0x4e5770, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_negotiate_action(manager: usize, id: u32) {
    covered!("tel39_2 news negotiate");
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    tel39_news_item(manager, id, item);
    let person = tel39_staff(exe::newsitem_get_slot(item, 0));
    if read_u32(item) == NEWS_TYPE_OFFER_ACCEPTED {
        let offer = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), exe::newsitem_get_slot(item, 7) as i32);
        contract_screens_offer_contract_to_staff(person, offer, 1, 0);
    } else {
        contract_screens_offer_contract_to_staff(person, 0, 0, 0);
    }
}

/// The waiting for players screen (a network game): only the background picture is cleared
/// (`background_set_picture(null)`).
/// Original: `match_screens_draw_waiting_screen` (0x727fa0, match_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_screens_draw_waiting_screen() {
    covered!("tel39_2 waiting screen");
    background_set_picture(0);
}

/// The empty atexit destructor of the static Date that 0x729ff0 builds on its first call: nothing.
/// Original: `match_stats_static_date_atexit` (0x72aa70, match_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_stats_static_date_atexit() {}

/// The constructor of the player search's filter record (0x11 bytes at PLAYER_SEARCH +0x293): dword +0 = 0x81,
/// dword +4 = 4, bytes +8 and +9 = 0xff, dword +0xa = 0, word +0xe = 1, byte +0x10 = 1 [inference: the default
/// filters]; returns `this`.
/// Original: `search_filter_record_init` (0x81b800, search_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_filter_record_init(this: usize) -> usize {
    covered!("tel39_2 search filter record");
    write_u8(this + 8, 0xff);
    write_u8(this + 9, 0xff);
    write_u32(this + 4, 4);
    // tel39_2: the default filter word
    write_u32(this, 0x81);
    write_u32(this + 0xa, 0);
    write_u16(this + 0xe, 1);
    write_u8(this + 0x10, 1);
    this
}

/// Comparer for `background_free_config_tables`: only the original runs (freeing does not run twice); then every
/// section must have no subsections left (array and count zero).
pub(crate) unsafe extern "C" fn compare_background_free_config_tables() {
    original_fn::<unsafe extern "C" fn()>(IDX_BACKGROUND_FREE_CONFIG_TABLES)();
    let left: Vec<i32> = (0..BACKGROUND_SECTION_COUNT as i32)
        .filter(|&k| read_usize(background_section(k) + 6) != 0 || read_i16(background_section(k) + 4) != 0)
        .collect();
    SCOREBOARD.record(left.is_empty(), || format!("background_free_config_tables: sections left {:?}", left));
}

/// Comparer for `background_set_pictures_disabled`: side by side over the flag.
pub(crate) unsafe extern "C" fn compare_background_set_pictures_disabled(on: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_BACKGROUND_SET_PICTURES_DISABLED);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(BACKGROUND_FIXED_FLAG, 4)], || o(on), || background_set_pictures_disabled(on));
    SCOREBOARD.record(same, || format!("background_set_pictures_disabled({}) original {:x?} rust {:x?}", on, mo, mr));
}

/// Comparer for `background_pictures_disabled`: the answer side by side.
pub(crate) unsafe extern "C" fn compare_background_pictures_disabled() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(IDX_BACKGROUND_PICTURES_DISABLED)();
    let n = background_pictures_disabled();
    SCOREBOARD.record(r == n, || format!("background_pictures_disabled() original={} rust={}", r, n));
    r
}

/// Comparer for `llist_scalar_deleting_destructor`: only the original runs (freeing does not run twice).
pub(crate) unsafe extern "thiscall" fn compare_llist_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_LLIST_SCALAR_DELETING_DESTRUCTOR)(this, flags)
}

/// Comparer for `contract_screens_news_offer_contract_action`: only the original runs (it opens a screen; the
/// contract offer's own comparer checks it).
pub(crate) unsafe extern "C" fn compare_contract_screens_news_offer_contract_action(manager: usize, id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_OFFER_CONTRACT_ACTION)(manager, id)
}

/// Comparer for `contract_screens_news_negotiate_action`: only the original runs (it opens a screen; the contract
/// offer's own comparer checks it).
pub(crate) unsafe extern "C" fn compare_contract_screens_news_negotiate_action(manager: usize, id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_NEGOTIATE_ACTION)(manager, id)
}

/// Comparer for `match_screens_draw_waiting_screen`: side by side over the screen's parameter block (where the
/// picture name goes).
pub(crate) unsafe extern "C" fn compare_match_screens_draw_waiting_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_MATCH_SCREENS_DRAW_WAITING_SCREEN);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(), || match_screens_draw_waiting_screen());
    SCOREBOARD.record(same, || format!("waiting for players screen: differences {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `match_stats_static_date_atexit`: the original (nothing to compare).
pub(crate) unsafe extern "C" fn compare_match_stats_static_date_atexit() {
    original_fn::<unsafe extern "C" fn()>(IDX_MATCH_STATS_STATIC_DATE_ATEXIT)()
}

/// Comparer for `search_filter_record_init`: side by side over the 0x11 bytes and the return.
pub(crate) unsafe extern "thiscall" fn compare_search_filter_record_init(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SEARCH_FILTER_RECORD_INIT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x11)], || o(this), || search_filter_record_init(this));
    SCOREBOARD.record(same && r == n, || format!("search_filter_record_init({:#x}) original {:x?} rust {:x?}", this, mo, mr));
    r
}
