// ---------- batch %%LEVA%%: the board ultimatum confirm screen (ultimatum.cpp) ----------

/// The ultimatum kinds (screen parameter 1): 1 the stadium, 2 the training facilities, 3 the transfer funds, another
/// the wage budget.
const ULTIMATUM_STADIUM: i8 = 1;
const ULTIMATUM_TRAINING: i8 = 2;
const ULTIMATUM_TRANSFER_FUNDS: i8 = 3;

/// Rule: the ultimatum news action opens the confirm screen when the manager has a club and the news item exists.
///
/// `ultimatum_news_issue_ultimatum_action(person, news_id)`: a local NewsItem (`NewsItem::NewsItem(0, 0)`); nothing
/// with no person or no `staff::CLUB_JOB`. `News::find_item_by_id(person, news_id, &item)` on `G_NEWS`; not found:
/// the ultimatum.cpp 69 error box. Otherwise `open_screen` with the builder 0x906610, the handler 0x906830, single
/// instance, the no-op leave routine 0x5a8f60 and 0; when it opens: parameter 0 = the person's id, 1 = slot 1 of the
/// news item (the kind), 2 = the club's id, 3 = `news_id`.
/// Original: `ultimatum_news_issue_ultimatum_action` (0x9064a0, ultimatum.cpp, cdecl).
pub(crate) unsafe extern "C" fn ultimatum_news_issue_ultimatum_action(person: usize, news_id: u32) {
    let mut item = [0u32; 0xb8];
    let item_at = item.as_mut_ptr() as usize;
    exe::new_newsitem(item_at, 0, 0);
    if person == 0 || read_usize(person + staff::CLUB_JOB) == 0 {
        return;
    }
    let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    if find(read_usize(G_NEWS), person, news_id, item_at) == 0 {
        covered!("tel36_2 ultimatum news item not found");
        return tel26_error_box(0xaa1b90, 0x45);
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x906610usize, 0x906830usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel36_2 open the ultimatum");
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, exe::newsitem_get_slot(item_at, 1) as usize);
    set_screen_param(2, read_u32(read_usize(person + staff::CLUB_JOB)) as usize);
    set_screen_param(3, news_id as usize);
}

/// Rule: the ultimatum screen asks whether to issue the board of the club an ultimatum: expand the stadium, improve
/// the training facilities, make more transfer funds available or increase the wage budget, with No / Yes.
///
/// `ultimatum_screen_draw()`: `tel36_confirm_build` with the question by the kind (parameter 1, a signed char): 1 the
/// stadium (0xaa1bbc), 2 the training (0xaa1ca8), 3 the transfer funds (0xaa1da0), another the wage budget (0xaa1e94),
/// each with the club's short name gender (`club::SHORT_GENDER_NAME`, twice) and short name (`club::SHORT_NAME`).
/// Original: `ultimatum_screen_draw` (0x906610, ultimatum.cpp, cdecl).
pub(crate) unsafe extern "C" fn ultimatum_screen_draw() {
    covered!("tel36_2 draw the ultimatum");
    let kind = get_screen_param(1) as u8 as i8;
    let club = tel36_club_param(2);
    tel36_confirm_build(|_| {
        let question = match kind {
            ULTIMATUM_STADIUM => 0xaa1bbc,
            ULTIMATUM_TRAINING => 0xaa1ca8,
            ULTIMATUM_TRANSFER_FUNDS => 0xaa1da0,
            _ => 0xaa1e94,
        };
        let gender = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
        tel36_langlib()(JOB_TEXT, question, gender, gender, club + club::SHORT_NAME);
    });
}

/// Rule: confirming the ultimatum hands it to the board (`MANAGER_MANAGER::handle_manager_ultimatum`) when the manager
/// has a club and marks the news item answered (slot 5).
///
/// `ultimatum_screen_handle_event(item)`: `tel36_confirm_answer`; "Yes": with a club (`staff::CLUB_JOB`),
/// `MANAGER_MANAGER::handle_manager_ultimatum(person, club, kind)` on `G_MANAGER_MANAGER` (the kind is parameter 1's
/// byte); then `News::set_item_slot(person, news_id, 5, 1, 0)` on `G_NEWS` (parameter 3).
/// Original: `ultimatum_screen_handle_event` (0x906830, ultimatum.cpp, cdecl).
pub(crate) unsafe extern "C" fn ultimatum_screen_handle_event(raw: u32) -> i32 {
    let person = tel36_staff_param(0);
    let kind = get_screen_param(1) as u8;
    let news_id = get_screen_param(3) as u32;
    tel36_confirm_answer(raw, || {
        covered!("tel36_2 issue the ultimatum");
        let club = read_usize(person + staff::CLUB_JOB);
        if club != 0 {
            let ultimatum: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6ad610usize);
            ultimatum(read_usize(G_MANAGER_MANAGER), person, club, kind as u32);
        }
        let set_slot: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
        set_slot(read_usize(G_NEWS), person, news_id, 5, 1, 0);
    })
}

/// Comparer for `ultimatum_news_issue_ultimatum_action`.
pub(crate) unsafe extern "C" fn compare_ultimatum_news_issue_ultimatum_action(person: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(person, news_id)
}

/// Comparer for `ultimatum_screen_draw`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_ultimatum_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("ultimatum screen (ultimatum_screen_draw)", &[], || o(), || ultimatum_screen_draw());
}

/// Comparer for `ultimatum_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_ultimatum_screen_handle_event(item: u32) -> i32 {
    // simplification: the ultimatum, the news slot and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}
