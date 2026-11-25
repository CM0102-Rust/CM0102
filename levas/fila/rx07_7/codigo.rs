// ---------- rx07_7: the contract news posted to a human manager: offer and trial cancelled, loan recalled (contract_manager.cpp) ----------

/// Slots 0 to 3 of a contract news item: the person's id and the ids of his first, second and common names.
pub(crate) unsafe fn set_news_person_name_slots(n: usize, person: usize) {
    exe::newsitem_set_slot(n, 0, read_u32(person));
    exe::newsitem_set_slot(n, 1, read_u32(read_usize(person + staff::FIRST_NAME) + names::ID));
    exe::newsitem_set_slot(n, 2, read_u32(read_usize(person + staff::SECOND_NAME) + names::ID));
    exe::newsitem_set_slot(n, 3, read_u32(read_usize(person + staff::COMMON_NAME) + names::ID));
}

/// News item `kind` of a person and a club with a human manager (`exe::news_club_has_human_manager`), sent to that
/// club's manager: the person and his names (slots 0 to 3) and the club (slot 4).
pub(crate) unsafe fn post_person_club_news(kind: u32, person: usize, club: usize) {
    if exe::news_club_has_human_manager(read_usize(G_NEWS), club) == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, kind, 0);
    set_news_person_name_slots(n, person);
    exe::newsitem_set_slot(n, 4, read_u32(club));
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Rule: a human manager's contract offer to a player is cancelled because his squad has no room: news item 0xbc4 to
/// the club's manager (only a club with a human manager), with the person, his names and the club.
/// Original: `CONTRACT_MANAGER::post_offer_cancelled_no_space_news` (0x4d64e0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_offer_cancelled_no_space_news(_this: usize, person: usize, club: usize) {
    covered!("rx07_7 offer cancelled news");
    post_person_club_news(0xbc4, person, club);
}

/// Comparer for `post_offer_cancelled_no_space_news`: the news goes to the manager's inbox (allocates): only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_post_offer_cancelled_no_space_news(this: usize, person: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, person, club)
}

/// Rule: a player's proposed trial at a human manager's club is cancelled because the squad has no room: news item
/// 0xbbc to the club's manager (only a club with a human manager), with the person, his names and the club.
/// Original: `CONTRACT_MANAGER::post_trial_cancelled_news` (0x4dade0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_trial_cancelled_news(_this: usize, person: usize, club: usize) {
    covered!("rx07_7 trial cancelled news");
    post_person_club_news(0xbbc, person, club);
}

/// Comparer for `post_trial_cancelled_news`: only the original runs (the news goes to the inbox).
pub(crate) unsafe extern "thiscall" fn compare_post_trial_cancelled_news(this: usize, person: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, person, club)
}

/// Rule: a loan is recalled or ended by `club`: news item 0xfb8 with the person and his names (slots 0 to 3), the club
/// he plays for (slot 4), his main contract's club (slot 5) and `club` (slot 6). It goes to the club on the other side:
/// when `club` is the one he plays for, to his main contract's club; otherwise to the one he plays for. Without a main
/// contract (`main_contract_lookup`, which also keeps it in `G_CONTRACT_LOOKUP_RESULT`), nothing.
/// Original: `CONTRACT_MANAGER::post_loan_recall_news` (0x4db0b0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_loan_recall_news(_this: usize, person: usize, club: usize) {
    let main_club = |p: usize| {
        let c = main_contract_lookup(Staff(p));
        if c == 0 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() }
    };
    if person == 0 {
        write_u32(G_CONTRACT_LOOKUP_RESULT, 0); // the lookup of a null person
        return;
    }
    if main_club(person) == 0 {
        return;
    }
    covered!("rx07_7 loan recall news");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfb8, 0);
    set_news_person_name_slots(n, person);
    let plays_for = read_usize(person + staff::CLUB_JOB);
    exe::newsitem_set_slot(n, 4, read_u32(plays_for));
    exe::newsitem_set_slot(n, 5, read_u32(main_club(person)));
    exe::newsitem_set_slot(n, 6, read_u32(club));
    let to = if club == read_usize(person + staff::CLUB_JOB) { main_club(person) } else { read_usize(person + staff::CLUB_JOB) };
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, to);
}

/// Comparer for `post_loan_recall_news`: only the original runs (the news goes to the inbox).
pub(crate) unsafe extern "thiscall" fn compare_post_loan_recall_news(this: usize, person: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, person, club)
}
