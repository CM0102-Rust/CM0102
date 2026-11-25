// ---------- rx08_10: the news of a manager's ultimatum (manager_manager.cpp) ----------

/// The stadium capacity a club of its reputation is expected to have, by the strength of its nation's league
/// (`nation::LEAGUE_STANDARD`, signed): above 14, three times the reputation + 5000; above 8, twice it + 2500;
/// otherwise twice it + 1250 (words); a club below 4250 reputation needs at most 5000; a part-time club
/// (`club::PROFESSIONAL_STATUS` 3) half of it (`rdata::F64_0_5`, truncated).
unsafe fn ultimatum_expected_capacity(club: Club) -> i16 {
    let reputation = read_i16(club.addr() + club::REPUTATION);
    let standard = read_i8(club.nation().addr() + nation::LEAGUE_STANDARD);
    let mut expected = if standard > 0xe {
        reputation.wrapping_mul(3).wrapping_add(0x1388)
    } else if standard > 8 {
        reputation.wrapping_mul(2).wrapping_add(0x9c4)
    } else {
        reputation.wrapping_mul(2).wrapping_add(0x4e2)
    };
    if reputation < 0x109a && expected > 0x1388 {
        expected = 0x1388;
    }
    if read_u8(club.addr() + club::PROFESSIONAL_STATUS) == 3 {
        expected = (expected as f64 * read_f64(rdata::F64_0_5)) as i32 as i16; // rx08_10: half for a part-time club
    }
    expected
}

/// Whether a club's training facilities (`club::TRAINING`) are poor for its reputation: below 10 above 4250 reputation,
/// below 14 above 5750.
unsafe fn ultimatum_poor_training(club: Club) -> bool {
    let reputation = read_i16(club.addr() + club::REPUTATION);
    let training = read_i8(club.addr() + club::TRAINING);
    (reputation > 0x109a && training < 0xa) || (reputation > 0x1676 && training < 0xe)
}

/// rule: the news of a manager's ultimatum to his board (item 0x178f) names the manager, the club and the four answer
/// codes, and says whether the stadium is too small for the club (by its reputation and its league's strength) and
/// whether its training facilities are poor
///
/// `this.post_ultimatum_response_news(manager, club, a, b, c, d)`: no manager or no club: the manager_manager.cpp box of
/// line 0x5cd2. Otherwise a news item 0x178f (`exe::new_newsitem`) with the slots (`exe::newsitem_set_slot`): 0 the
/// manager's id, 1 to 3 the ids (+0x33) of his first, second and common name records (`staff::FIRST_NAME`,
/// `staff::SECOND_NAME`, `staff::COMMON_NAME`), 4 the club's id, 5 to 8 the four codes (signed bytes), 9 whether the
/// stadium (`club::STADIUM`, its `stadium::CAPACITY`) holds less than the expected capacity
/// (`ultimatum_expected_capacity`), 10 whether the training facilities are poor (`ultimatum_poor_training`); sent to the
/// club's manager (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_ultimatum_response_news` (0x6ad930, manager_manager.cpp, thiscall, ret 0x18).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn post_ultimatum_response_news(_this: usize, manager: usize, club: usize, a: u32, b: u32, c: u32, d: u32) {
    if manager == 0 || club == 0 {
        show_error_at(appoint_fns::MANAGER_MANAGER_CPP, 0x5cd2);
        return;
    }
    covered!("rx08_10 ultimatum news");
    let club = Club(club);
    let mut news = [0u32; 0x3a]; // the local news item (0xe0 bytes in the original)
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x178f, 0);
    let name_id = |field: usize| read_u32(read_usize(manager + field) + 0x33); // the name record's id
    let small_stadium = read_i32(read_usize(club.addr() + club::STADIUM) + stadium::CAPACITY) < ultimatum_expected_capacity(club) as i32; // rx08_10: the stadium against the expected capacity
    let slots = [read_u32(manager), name_id(staff::FIRST_NAME), name_id(staff::SECOND_NAME), name_id(staff::COMMON_NAME), read_u32(club.addr()),
        a as i8 as i32 as u32, b as i8 as i32 as u32, c as i8 as i32 as u32, d as i8 as i32 as u32, small_stadium as u32, ultimatum_poor_training(club) as u32];
    for (k, v) in slots.into_iter().enumerate() {
        exe::newsitem_set_slot(n, k as i8, v);
    }
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_ultimatum_response_news`: it sends a news item (it cannot run twice), so only the original runs;
/// the copy is proven by playing comparing (the board answer news shows its slots).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_post_ultimatum_response_news(this: usize, manager: usize, club: usize, a: u32, b: u32, c: u32, d: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(%%I0%%)(this, manager, club, a, b, c, d);
}
