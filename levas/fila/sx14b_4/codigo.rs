// ---------- batch %%LEVA%%: the board's view of releases and the request and memo news (manager_manager.cpp) ----------

/// `MANAGER_MANAGER::judge_player_release` (0x697240; thiscall(this, player, club), ret 8), through the exe address.
const FN_MANAGER_MANAGER_JUDGE_PLAYER_RELEASE: usize = 0x697240;
/// `open_resign_job_screen` (0x6b1d10; cdecl(club)), through the exe address.
const FN_OPEN_RESIGN_JOB_SCREEN: usize = 0x6b1d10;

/// Comparer of a label writer: the original writes into `out`, the copy into a local buffer that starts with the same
/// bytes; the `size` bytes (at most 0x200) must match.
unsafe fn compare_label_writer(name: &str, out: usize, size: u32, original: impl FnOnce(), copy: impl FnOnce(usize)) {
    let n = if out != 0 { (size as i16).clamp(0, 0x200) as usize } else { 0 };
    let mut local = [0u8; 0x200];
    if n != 0 {
        core::ptr::copy_nonoverlapping(out as *const u8, local.as_mut_ptr(), n);
    }
    original();
    copy(if out != 0 { local.as_mut_ptr() as usize } else { 0 });
    let o = if n != 0 { core::slice::from_raw_parts(out as *const u8, n) } else { &[] };
    SCOREBOARD.record(o == &local[..n], || format!("{}: original {:?} rust {:?}", name, String::from_utf8_lossy(o), String::from_utf8_lossy(&local[..n])));
}

/// The board's money threshold for a player of the club: (reputation × 0.2)², truncated.
unsafe fn release_value_threshold(club: Club) -> i32 {
    let x = club.reputation() as i32 as f64 * read_f64(rdata::F64_0_2);
    (x * x) as i32
}

/// The player's value to the club (`exe::contract_manager_calc_player_value`, mode 1 for a human club).
unsafe fn release_player_value(player: Staff, club: Club) -> i32 {
    let human = manager_manager_is_human_club(club.addr(), 1, 0) != 0;
    exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, club, 1, -1, human as u32)
}

/// Fewer than 15 in the club's squad (`exe::database_count_club_squad`).
unsafe fn release_small_squad(club: Club) -> bool {
    (exe::database_count_club_squad(club, 0, 0) as i8) < 0xf
}

/// Opens the resign confirmation dialog for the club (`open_resign_job_screen`, through the exe address).
/// Original: `open_resign_confirm_dialog` (0x69b780, manager_manager.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn open_resign_confirm_dialog(club: usize) {
    covered!("sx14b_4 resign dialog");
    let open: unsafe extern "C" fn(usize) = core::mem::transmute(FN_OPEN_RESIGN_JOB_SCREEN);
    open(club);
}

/// Comparer for `open_resign_confirm_dialog`: it opens a screen, so only the original runs.
pub(crate) unsafe extern "stdcall" fn compare_open_resign_confirm_dialog(club: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize)>(%%I0%%)(club)
}

/// Whether a board request option is open: 0 for the youth development request (2) when the club's training
/// facilities (`.training()`) are at the maximum of 20, else 1.
/// Original: `is_board_request_available` (0x6ae180, manager_manager.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn is_board_request_available(club: usize, request: u32) -> u8 {
    (!(request as u8 == 2 && Club(club).training() == 0x14)) as u8
}

/// Comparer for `is_board_request_available`: side by side on the answer.
pub(crate) unsafe extern "stdcall" fn compare_is_board_request_available(club: usize, request: u32) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u8>(%%I1%%)(club, request);
    let r = is_board_request_available(club, request);
    SCOREBOARD.record(o == r, || format!("is_board_request_available(club {:#x}, request {}): original {} rust {}", club, request as i8, o, r));
    o
}

/// Writes the label of an FA request option into `out` (size at least 15): 1 "Complain about a referee", 2 "Request
/// match postponement" (langlib); otherwise nothing.
/// Original: `get_fa_request_option_text` (0x6a9e20, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn get_fa_request_option_text(option: u32, out: usize, size: u32) {
    if out == 0 || (size as i16) < 0xf {
        return;
    }
    covered!("sx14b_4 FA request label");
    match option as i8 {
        1 => story_lang!(out, 0xa1153c), // "Complain about a referee"
        2 => story_lang!(out, 0xa11558), // "Request match postponement"
        _ => {}
    }
}

/// Comparer for `get_fa_request_option_text`: side by side on the label.
pub(crate) unsafe extern "stdcall" fn compare_get_fa_request_option_text(option: u32, out: usize, size: u32) {
    compare_label_writer("get_fa_request_option_text (FA request screen)", out, size,
        || original_fn::<unsafe extern "stdcall" fn(u32, usize, u32)>(%%I2%%)(option, out, size),
        |b| get_fa_request_option_text(option, b, size));
}

/// Writes the label of one of the six board request options into `out` (size at least 15, langlib): 1 "Expand the
/// Stadium", 2 "Invest in youth development", 3 "Make extra transfer funds available", 4 "Make extra funds for wages
/// available", 5 "Extra time to rebuild the squad", 6 "Win bonus for upcoming match"; otherwise nothing.
/// Original: `get_board_request_option_text` (0x6a8770, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn get_board_request_option_text(option: u32, out: usize, size: u32) {
    if out == 0 || (size as i16) < 0xf {
        return;
    }
    let text = match option as i8 {
        1 => 0xa0fd70, // "Expand the Stadium"
        2 => 0xa0fd54, // "Invest in youth development"
        3 => 0xa0fd30, // "Make extra transfer funds available"
        4 => 0xa0fd08, // "Make extra funds for wages available"
        5 => 0xa0fce8, // "Extra time to rebuild the squad"
        6 => 0xa0fcc8, // "Win bonus for upcoming match"
        _ => return,
    };
    story_lang!(out, text);
}

/// Comparer for `get_board_request_option_text`: side by side on the label.
pub(crate) unsafe extern "stdcall" fn compare_get_board_request_option_text(option: u32, out: usize, size: u32) {
    compare_label_writer("get_board_request_option_text (board request screen)", out, size,
        || original_fn::<unsafe extern "stdcall" fn(u32, usize, u32)>(%%I3%%)(option, out, size),
        |b| get_board_request_option_text(option, b, size));
}

/// The board's anger at a blocked release: board confidence − 1250 (at least 1), `board_record::TRANSFER_MOOD`
/// − 1250 and `board_record::FAVOURITES_MOOD` − 750.
unsafe fn board_blocks_release_penalty(row: BoardRecord) {
    row.set_confidence(row.confidence().wrapping_sub(1250));
    row.set_transfer_mood(row.transfer_mood().wrapping_sub(1250));
    row.set_favourites_mood(row.favourites_mood().wrapping_sub(750));
    if row.confidence() < 1 {
        row.set_confidence(1);
    }
}

/// Decides whether the board blocks a free release of the player: 0 for no contract with the club
/// (`exe::contract_manager_get_contract_for_club`), a contract of type 1 or 3, a player in dispute
/// (`exe::dispute_is_in_dispute`) or an unhappy one (happiness 0 or less). Worth more than (reputation × 0.2)² to the
/// club: blocked, with the penalty (`board_blocks_release_penalty`), answering 0x200 with fewer than 15 in the squad,
/// else 0x800. Worth less: blocked (0x200, with the penalty) only with fewer than 15 in the squad, else 0.
/// Original: `MANAGER_MANAGER::check_board_blocks_release` (0x696940, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn check_board_blocks_release(this: usize, player: usize, club: usize) -> u32 {
    let (player, club) = (Staff(player), Club(club));
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if c.is_null() || matches!(c.contract_type() & 0x3f, 1 | 3) {
        return 0;
    }
    let dispute = read_usize(G_DISPUTE);
    if dispute != 0 && exe::dispute_is_in_dispute(dispute, player, 0) != 0 {
        return 0;
    }
    let threshold = release_value_threshold(club);
    let row = BoardRecord(board_row(this, club));
    let answer = if release_player_value(player, club) > threshold {
        let small = release_small_squad(club);
        if c.happiness() <= 0 {
            return 0;
        }
        if small { 0x200 } else { 0x800 }
    } else {
        if c.happiness() <= 0 || !release_small_squad(club) {
            return 0;
        }
        0x200
    };
    covered!("sx14b_4 board blocks release");
    board_blocks_release_penalty(row);
    answer
}

/// Comparer for `check_board_blocks_release`: side by side on the answer and the club's board record.
pub(crate) unsafe extern "thiscall" fn compare_check_board_blocks_release(this: usize, player: usize, club: usize) -> u32 {
    let row = if club != 0 { board_row(this, Club(club)) } else { 0 };
    let regions = if row != 0 { vec![(row, 0x49)] } else { vec![] };
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&regions,
        || original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I4%%)(this, player, club),
        || check_board_blocks_release(this, player, club));
    SCOREBOARD.record(o == r && equal, || format!("check_board_blocks_release (release screen): original {:#x} rust {:#x}, board record {:?}",
        o, r, first_differences(&mo, &mr)));
    o
}

/// The board's reaction news to a release (through the exe address, so the hook and its comparer run).
unsafe fn release_reaction_news(this: usize, player: Staff, club: Club, flags: u32) {
    exe::manager_manager_post_board_release_reaction_news(this, player, club, flags);
}

/// Rates how the board views releasing the player, posting the matching reaction news
/// (`post_board_release_reaction_news`) and answering its code: 4 (no news) for a contract of type 1 or 5, or a player
/// worth under 2.5 million whom the chairman dislikes (his `.staff_preferences()` +0x28, +0x2c or +0x30). Worth more
/// than (reputation × 0.2)²: happy (happiness above 0) gives 0x800, else fewer than 15 in the squad 0x400 (news
/// 0x32) and otherwise 1. Worth less: happy with 15 or more 1, with fewer the news 0x32 but the answer 0x800; unhappy,
/// with a player record whose home reputation beats 0.8 × the club's: fewer than 15, 1 (news 0x32), else 2; otherwise
/// fewer than 15, 2 (news 0x32), else 4.
///
/// Defects of the original: no check that the player has a contract with the club (a null contract is read); a happy
/// player worth less with a small squad posts flags 0x32 but answers 0x800.
/// Original: `MANAGER_MANAGER::judge_player_release` (0x697240, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn judge_player_release(this: usize, player: usize, club: usize) -> u32 {
    let (player, club) = (Staff(player), Club(club));
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if matches!(c.contract_type() & 0x3f, 1 | 5) {
        return 4;
    }
    let threshold = release_value_threshold(club);
    let value = release_player_value(player, club);
    let chairman = club.chairman();
    if value < 2_500_000 && !chairman.is_null() && !chairman.non_player().is_null() && chairman.staff_preferences() != 0 {
        let p = chairman.staff_preferences();
        // the chairman's disliked staff (+0x28 STAFF_DISLIKED_STAFF1, +0x2c, +0x30)
        if [0x28, 0x2c, 0x30].iter().any(|&k| read_usize(p + k) == player.addr()) {
            return 4;
        }
    }
    let (code, flags) = if value > threshold {
        let small = release_small_squad(club);
        if c.happiness() > 0 {
            (0x800, 0x800)
        } else if small {
            (0x400, 0x32)
        } else {
            (1, 1)
        }
    } else if c.happiness() > 0 {
        if release_small_squad(club) { (0x800, 0x32) } else { (1, 1) }
    } else {
        let sheet = player.player();
        let renowned = !sheet.is_null() && (sheet.home_reputation() as i32 as f64) > club.reputation() as i32 as f64 * read_f64(rdata::F64_0_8);
        let small = release_small_squad(club);
        match (renowned, small) {
            (true, true) => (1, 0x32),
            (true, false) => (2, 2),
            (false, true) => (2, 0x32),
            (false, false) => return 4,
        }
    };
    covered!("sx14b_4 release judged");
    release_reaction_news(this, player, club, flags);
    code
}

/// Comparer for `judge_player_release`: it posts the news, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_judge_player_release(this: usize, player: usize, club: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I5%%)(this, player, club)
}

/// Adds the three deltas to the board confidence, `TRANSFER_MOOD` and `FAVOURITES_MOOD` (0 leaves one alone).
unsafe fn release_mood(row: BoardRecord, confidence: i16, transfer: i16, favourites: i16) {
    row.set_confidence(row.confidence().wrapping_add(confidence));
    row.set_transfer_mood(row.transfer_mood().wrapping_add(transfer));
    if favourites != 0 {
        row.set_favourites_mood(row.favourites_mood().wrapping_add(favourites));
    }
}

/// Cuts the club's board confidence and moods after a release, by the board's reaction (`judge_player_release`),
/// the player's age and injury and his happiness. Reaction 1: with the player over 31 and injured for more than 90
/// (injury record +0x28) −25/−25/−150 first; then happiness above 50 −250/−500/−500, above 0 −125/−250/−250, else
/// −50/−100. Reaction 2: old and injured −25/−50/−50 only; else above 50 −125/−250/−250, above 0 −25/−50/−50, else
/// −10/−10. Reaction 0x400: old and injured −125/−250/−200 first; then above 50 −500/−1000/−1000, above 0
/// −250/−500/−500, else −125/−250. Then −500 favourites mood if the club favours him (`exe::club_favours_staff`) and
/// −500 if the chairman does (`exe::staff_is_favoured_by`); confidence at least 1, the moods at least −10000. No
/// contract, no club or no player: the manager_manager.cpp "Error" box (lines 0x20c5, 0x20cc, 0x20d1).
/// Original: `MANAGER_MANAGER::apply_release_confidence_penalty` (0x697430, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn apply_release_confidence_penalty(this: usize, player: usize, club: usize) {
    const MANAGER_MANAGER_CPP: usize = 0x9f18a4;
    let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if c.is_null() {
        return show_error_box(MANAGER_MANAGER_CPP, 0x20c5);
    }
    if club == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x20cc);
    }
    if player == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x20d1);
    }
    let judge: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_JUDGE_PLAYER_RELEASE);
    let reaction = judge(this, player, club);
    let (player, club) = (Staff(player), Club(club));
    let age = player.age() as i8;
    let injury = exe::injury_get_player_injury(read_usize(G_INJURY), player);
    let old_and_injured = age > 0x1f && injury != 0 && read_i16(injury + 0x28) > 0x5a; // +0x28: days out [inference]
    let row = BoardRecord(board_row(this, club));
    let happiness = c.happiness();
    match reaction {
        1 => {
            if old_and_injured {
                release_mood(row, -25, -25, -150);
            }
            if happiness > 50 {
                release_mood(row, -250, -500, -500);
            } else if happiness > 0 {
                release_mood(row, -125, -250, -250);
            } else {
                release_mood(row, -50, -100, 0);
            }
        }
        2 => {
            if old_and_injured {
                release_mood(row, -25, -50, -50);
            } else if happiness > 50 {
                release_mood(row, -125, -250, -250);
            } else if happiness > 0 {
                release_mood(row, -25, -50, -50);
            } else {
                release_mood(row, -10, -10, 0);
            }
        }
        0x400 => {
            if old_and_injured {
                release_mood(row, -125, -250, -200);
            }
            if happiness > 50 {
                release_mood(row, -500, -1000, -1000);
            } else if happiness > 0 {
                release_mood(row, -250, -500, -500);
            } else {
                release_mood(row, -125, -250, 0);
            }
        }
        _ => {}
    }
    covered!("sx14b_4 release penalty");
    if exe::club_favours_staff(player, club) != 0 {
        row.set_favourites_mood(row.favourites_mood().wrapping_sub(500));
    }
    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
        row.set_favourites_mood(row.favourites_mood().wrapping_sub(500));
    }
    if row.confidence() < 1 {
        row.set_confidence(1);
    }
    row.set_transfer_mood(row.transfer_mood().max(-10000));
    row.set_favourites_mood(row.favourites_mood().max(-10000));
}

/// Comparer for `apply_release_confidence_penalty`: it calls `judge_player_release`, which posts the news, so only
/// the original runs.
pub(crate) unsafe extern "thiscall" fn compare_apply_release_confidence_penalty(this: usize, player: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I6%%)(this, player, club)
}

/// Posts the 0x1778 news of the board's reaction to a released (or blocked) player, for a human-managed club.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_board_release_reaction_news(player, club, flags)`: no player,
/// no club, a club without a human manager (`exe::news_club_has_human_manager`) or before the news start date
/// (`before_news_start_date`): nothing. News 0x1778: 0 the player's id, 1..3 his name ids (`names::ID` of
/// `.first_name()`, `.second_name()`, `.common_name()`), 4 the club's id, 5 the flags (signed word); to the club's
/// manager (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_board_release_reaction_news` (0x6a0990, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_board_release_reaction_news(_this: usize, player: usize, club: usize, flags: u32) {
    let player = Staff(player);
    if player.is_null() || club == 0 || exe::news_club_has_human_manager(read_usize(G_NEWS), club) == 0 || before_news_start_date() {
        return;
    }
    covered!("sx14b_4 release reaction news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1778, 0);
    news_set_person(n, 0, player);
    exe::newsitem_set_slot(n, 4, read_u32(club));
    exe::newsitem_set_slot(n, 5, flags as i16 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_board_release_reaction_news`: the news goes to the manager's inbox (allocates); only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_post_board_release_reaction_news(this: usize, player: usize, club: usize, flags: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I7%%)(this, player, club, flags)
}

/// Posts the 0x1390 news that the manager's board knows he applied for another club's job.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_board_interest_memo_news(person, club, other)`: news 0x1390:
/// 0 the person's id, 1 the club's id, 2 the other club's id, 3..5 his name ids; a human manager also gets his two
/// name strings from slot 7 with the next free slot in 6 (`news_set_manager_names`). Delivered to him
/// (`exe::news_deliver_to_manager`, 0).
/// Original: `MANAGER_MANAGER::post_board_interest_memo_news` (0x69c7e0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_board_interest_memo_news(_this: usize, person: usize, club: usize, other: usize) {
    let person = Staff(person);
    let mut local = [0u32; 0x39]; // the local news item (0xe4 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1390, 0);
    exe::newsitem_set_slot(n, 0, read_u32(person.addr()));
    exe::newsitem_set_slot(n, 1, read_u32(club));
    exe::newsitem_set_slot(n, 2, read_u32(other));
    exe::newsitem_set_slot(n, 3, read_u32(person.first_name() + names::ID));
    exe::newsitem_set_slot(n, 4, read_u32(person.second_name() + names::ID));
    exe::newsitem_set_slot(n, 5, read_u32(person.common_name() + names::ID));
    if is_human_manager(person) {
        covered!("sx14b_4 memo to a human");
        news_set_manager_names(n, person, 7, 6);
    }
    exe::news_deliver_to_manager(read_usize(G_NEWS), n, person, 0);
}

/// Comparer for `post_board_interest_memo_news`: the news goes to the manager's inbox (allocates); only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_board_interest_memo_news(this: usize, person: usize, club: usize, other: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I8%%)(this, person, club, other)
}

/// Posts the 0x1786 news of the board's reply to a manager request.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_board_request_reply_news(club, request, answer)`: no club or
/// no finances (`exe::financetable_get_club_finance`): nothing. News 0x1786: 0 the club's id, 1 the request, 2 the
/// answer (signed bytes), 3 the board confidence, 4 low funds (wealth band −2 or −1, `exe::finance_get_wealth_band`,
/// or a balance below 1 million), 5 zero; to the club's manager (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_board_request_reply_news` (0x6a9350, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_board_request_reply_news(this: usize, club: usize, request: u32, answer: u32) {
    let club = Club(club);
    if club.is_null() {
        return;
    }
    let finance = Finance(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club));
    if finance.is_null() {
        return;
    }
    covered!("sx14b_4 board reply news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1786, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 1, request as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 2, answer as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 3, BoardRecord(board_row(this, club)).confidence() as i32 as u32);
    let band = exe::finance_get_wealth_band(finance) as i8;
    let low_funds = (-2..0).contains(&band) || finance.balance() < 1_000_000;
    exe::newsitem_set_slot(n, 4, low_funds as u32);
    exe::newsitem_set_slot(n, 5, 0);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_board_request_reply_news`: the news goes to the manager's inbox (allocates); only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_board_request_reply_news(this: usize, club: usize, request: u32, answer: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I9%%)(this, club, request, answer)
}

/// Posts the 0x178a news of the FA's verdict on a player's ban appeal, to his club's manager.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_ban_appeal_result_news(player, verdict, games, ban, other)`:
/// no player, no club (`.club_job()`) or a club without a nation: nothing. News 0x178a: 0 the player's id, 1..3 his
/// name ids, 4 the verdict, 5 the club nation's id, 6 the games, 7 the ban (signed word), 8 the other games (signed
/// bytes); to the club's manager (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_ban_appeal_result_news` (0x6ac060, manager_manager.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn post_ban_appeal_result_news(_this: usize, player: usize, verdict: u32, games: u32, ban: u32, other: u32) {
    let player = Staff(player);
    if player.is_null() || player.club_job().is_null() || player.club_job().nation().is_null() {
        return;
    }
    covered!("sx14b_4 ban appeal news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x178a, 0);
    news_set_person(n, 0, player);
    exe::newsitem_set_slot(n, 4, verdict as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 5, read_u32(player.club_job().nation().addr()));
    exe::newsitem_set_slot(n, 6, games as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 7, ban as i16 as i32 as u32);
    exe::newsitem_set_slot(n, 8, other as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, player.club_job());
}

/// Comparer for `post_ban_appeal_result_news`: the news goes to the manager's inbox (allocates); only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_ban_appeal_result_news(this: usize, player: usize, verdict: u32, games: u32, ban: u32, other: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I10%%)(this, player, verdict, games, ban, other)
}

/// Queues the 0x1774 offer of a club's manager job to the (human) person, with the transfer budget and the board's
/// expectations worked out as if he were in charge.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_job_offer_to_human_news(club, person)`: news 0x1774: 0 the
/// club's id, 1 zero. A club that has a manager (`.manager()`): nothing more (and nothing sent). A national team: 2
/// zero, 3 -1, 4 zero. A club: its board record's `SQUAD_INVESTMENT` cleared, 2 the transfer budget
/// (`exe::finance_get_transfer_budget` of its finances, low dword); with the person put in as manager and the
/// `NEW_MANAGER` flag (bit 0 of `board_record::FLAGS`) set, the season expectations (`set_season_expectations`, no
/// news), then the manager and the flag's old value back; 3 the `EXPECTATION_FLAGS`, 4 a random(2). Delivered to the
/// person (`exe::news_deliver_to_manager`, 1).
/// Original: `MANAGER_MANAGER::post_job_offer_to_human_news` (0x6a4f80, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_job_offer_to_human_news(this: usize, club: usize, person: usize) {
    let club = Club(club);
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1774, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 1, 0);
    if !club.manager().is_null() {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        exe::newsitem_set_slot(n, 2, 0);
        exe::newsitem_set_slot(n, 3, u32::MAX);
        exe::newsitem_set_slot(n, 4, 0);
    } else {
        covered!("sx14b_4 job offer to a human");
        let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let row = BoardRecord(board_row(this, club));
        write_u32(row.addr() + board_record::SQUAD_INVESTMENT, 0);
        write_u32(row.addr() + board_record::SQUAD_INVESTMENT + 4, 0);
        let budget = exe::finance_get_transfer_budget(finance, 0, 0, 0usize, 1, 0);
        exe::newsitem_set_slot(n, 2, budget as u32);
        let manager = club.manager();
        let new_manager = row.flags() & 1;
        club.set_manager(Staff(person));
        row.set_flags(row.flags() | 1);
        set_season_expectations(this, club.addr(), 0);
        club.set_manager(manager);
        row.set_flags((row.flags() & !1) | new_manager);
        exe::newsitem_set_slot(n, 3, row.expectation_flags() as i32 as u32);
        exe::newsitem_set_slot(n, 4, exe::utils_random(2) as u32);
    }
    exe::news_deliver_to_manager(read_usize(G_NEWS), n, person, 1);
}

/// Comparer for `post_job_offer_to_human_news`: it delivers the news and draws, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_post_job_offer_to_human_news(this: usize, club: usize, person: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I11%%)(this, club, person)
}
