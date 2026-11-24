// ---------- sx21_1: the base transfer rules: building them, who a club may sign and which offers it may make (transfer_offer.cpp) ----------

/// Builds the base transfer rules of a nation, a continent or the whole world.
///
/// `TRANSFER_RULES::TRANSFER_RULES(rule, nation, continent, _a4, _a5)`, from every country's constructor and from the
/// transfer manager's set-up. The vtable 0x970458, the rule number (+9, a byte). With a nation index (above -1): the
/// nation record (`G_NATION_TABLE` + index x `nation::SIZE`) in `transfer_rules::NATION_ID`, its continent
/// (`nation::CONTINENT`) in `transfer_rules::CONTINENT_ID` and `transfer_rules::SCOPE` 1. Otherwise, with a continent
/// index (above -1): no nation, the continent record (`G_CONTINENT_TABLE` + index x 0xc6) and scope 2. Otherwise
/// neither, scope 4. Always: the window table (+4) and `transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY` cleared,
/// `transfer_rules::FOREIGN_PLAYER_LIMIT` 50 and `transfer_rules::NON_EU_PLAYER_LIMIT` 30. The window count (+8) is left
/// for the country to set; the last two arguments are not read. Returns the object.
/// Original: `TRANSFER_RULES::TRANSFER_RULES` (0x8e84a0, transfer_offer.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_transfer_rules(this: usize, rule: u32, nation: i32, continent: i32, _a4: u32, _a5: u32) -> usize {
    covered!("sx21_1 base rules");
    write_u8(this + 9, rule as u8);
    write_u32(this, 0x970458); // the TRANSFER_RULES vtable
    let scope;
    if nation > -1 {
        let n = read_usize(G_NATION_TABLE).wrapping_add((nation as usize).wrapping_mul(nation::SIZE));
        write_usize(this + transfer_rules::NATION_ID, n);
        write_usize(this + transfer_rules::CONTINENT_ID, read_usize(n + nation::CONTINENT));
        scope = 1;
    } else {
        write_usize(this + transfer_rules::NATION_ID, 0);
        if continent > -1 {
            write_usize(this + transfer_rules::CONTINENT_ID, read_usize(G_CONTINENT_TABLE).wrapping_add((continent as usize).wrapping_mul(0xc6))); // the 198-byte continent record
            scope = 2;
        } else {
            write_usize(this + transfer_rules::CONTINENT_ID, 0);
            scope = 4;
        }
    }
    write_u32(this + 4, 0);
    write_u32(this + transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY, 0);
    write_u8(this + transfer_rules::SCOPE, scope);
    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x32); // sx21_1: 50 foreign players
    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0x1e);
    this
}

/// Comparer for `construct_transfer_rules` (`compare_transfer_rules_ctor`: the original on the object, the copy on a
/// copy of its first bytes).
pub(crate) unsafe extern "thiscall" fn compare_construct_transfer_rules(this: usize, rule: u32, nation: i32, continent: i32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, i32, i32, u32, u32) -> usize>(%%I0%%);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_transfer_rules", this, || r = o(this, rule, nation, continent, a4, a5), |s| {
        construct_transfer_rules(s, rule, nation, continent, a4, a5);
    });
    r
}

/// Whether `date` falls inside one of the rule set's transfer windows (`exe::transfer_window_contains_date` on each
/// 12-byte window of the table at +4, count at +8). A rule set without windows (count 0) allows any date; a negative
/// count (never seen) refuses.
pub(crate) unsafe fn date_in_transfer_window(this: usize, date: usize) -> bool {
    let n = read_u8(this + 8) as i8;
    for k in 0..(n.max(0) as usize) {
        if exe::transfer_window_contains_date(this, date, read_usize(this + 4) + k * 12) != 0 {
            return true;
        }
    }
    n == 0
}

/// Counts the squad (the 50 slots of `club.squad()`) for `can_sign_player`: a player with an id below `G_STAFF_COUNT`
/// for whom `counted` holds adds to the count, anyone else to the others. True as soon as the count reaches `limit`;
/// the scan stops once the others pass 50 - `limit`.
unsafe fn squad_count_reaches(club: Club, limit: i8, counted: impl Fn(usize) -> bool) -> bool {
    let (mut count, mut others) = (0i8, 0i8);
    for k in 0..50usize {
        let p = club.squad(k);
        if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) && counted(p) {
            count = count.wrapping_add(1);
        } else {
            others = others.wrapping_add(1);
        }
        if count == limit {
            return true;
        }
        if others as i32 > 0x32 - limit as i32 {
            break;
        }
    }
    count == limit
}

/// Whether a club may sign a player: refuses a foreign player when the squad already holds the foreign limit (0x14), a
/// non-EU player in a European nation when it holds the non-EU limit (0x15), and any signing outside the windows (9).
///
/// `TRANSFER_RULES::can_sign_player(club, player, _a3, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER`. The limits are `transfer_rules::FOREIGN_PLAYER_LIMIT` and
/// `transfer_rules::NON_EU_PLAYER_LIMIT`, with two exceptions by the club: a Turkish club (`KEY_NATION_TURKEY`) outside
/// the Turkish Premier Division (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`, or without a division) has a foreign limit
/// of 0; a Spanish club (`KEY_NATION_SPAIN`) has a non-EU limit of 4 in the First Division, 3 in the Second and 0
/// elsewhere. A player foreign to the club's nation (`exe::database_is_staff_foreign(player, nation, 1)`), with a limit
/// other than 0xff: when the squad's foreign players reach the limit (`squad_count_reaches`), reason 0x14 and 0. In a
/// rule set whose nation (`transfer_rules::NATION_ID`) is in Europe (`KEY_CONTINENT_EUROPE`), a foreign player who is
/// not an EU national (`exe::database_is_staff_eu_national(player, club nation)`), with a non-EU limit other than
/// 0xff: when the squad's foreign non-EU players reach it, reason 0x15 and 0. Then the windows
/// (`date_in_transfer_window`): outside them, reason 9 and 0; otherwise 1. `_a3` is not read.
/// Defect of the original: the squad count asks `database_is_staff_eu_national` with nation 0 instead of the club's
/// nation, unlike the check of the player being signed.
/// Original: `TRANSFER_RULES::can_sign_player` (0x8e8560, transfer_offer.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn transfer_rules_can_sign_player(this: usize, club: usize, player: usize, _a3: u32, date: usize, reason: *mut u16) -> u32 {
    let club = Club(club);
    let mut non_eu_limit = read_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT) as i8;
    let mut foreign_limit = read_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT) as i8;
    let nation = read_u32(club.nation().addr());
    let division = club.division().addr();
    let in_division = |key: usize| division != 0 && read_u32(division) == read_u32(key);
    if nation == read_u32(KEY_NATION_TURKEY) && !in_division(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
        foreign_limit = 0;
    } else if nation == read_u32(KEY_NATION_SPAIN) {
        non_eu_limit = if in_division(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) {
            4
        } else if in_division(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            3
        } else {
            0
        };
    }
    let foreign = |p: usize| exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0;
    if player != 0 && foreign(player) && foreign_limit != -1 && squad_count_reaches(club, foreign_limit, foreign) {
        covered!("sx21_1 foreign limit");
        if !reason.is_null() {
            *reason = 0x14; // sx21_1: the foreign limit
        }
        return 0;
    }
    let rules_nation = read_usize(this + transfer_rules::NATION_ID);
    let continent = if rules_nation != 0 { read_usize(rules_nation + nation::CONTINENT) } else { 0 };
    if continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_EUROPE) && player != 0 && foreign(player)
        && exe::database_is_staff_eu_national(player, club.nation().addr()) == 0 && non_eu_limit != -1
        && squad_count_reaches(club, non_eu_limit, |p| foreign(p) && exe::database_is_staff_eu_national(p, 0usize) == 0) {
        covered!("sx21_1 non-EU limit");
        if !reason.is_null() {
            *reason = 0x15;
        }
        return 0;
    }
    if date_in_transfer_window(this, date) {
        return 1;
    }
    covered!("sx21_1 sign outside the window");
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

/// Side-by-side comparer of a transfer rule check with a reason (`can_sign_player`, `can_make_offer`): the return,
/// the reason word and `G_CONTRACT_LOOKUP_RESULT`.
pub(crate) unsafe fn compare_rule_check(name: &str, reason: *mut u16, o: impl FnOnce() -> u32, r: impl FnOnce() -> u32, what: impl Fn() -> String) -> u32 {
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if !reason.is_null() {
        regions.push((reason as usize, 2));
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, o, r);
    SCOREBOARD.record(equal && ro == rr, || format!("{}({}) original={} {:x?} rust={} {:x?}", name, what(), ro, mo, rr, mr));
    ro
}

/// Comparer for `transfer_rules_can_sign_player`.
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_can_sign_player(this: usize, club: usize, player: usize, a3: u32, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, *mut u16) -> u32>(%%I1%%);
    compare_rule_check("transfer_rules_can_sign_player", reason, || o(this, club, player, a3, date, reason), || transfer_rules_can_sign_player(this, club, player, a3, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}", this, club, player))
}

/// Whether a club may take part in an offer: refuses when the move would leave the club with more foreign players than
/// the foreign limit (0x14), and a buying club outside the windows (9).
///
/// `TRANSFER_RULES::can_make_offer(offer, club, date, reason)`, the vtable slot `transfer_rules_vtable::CAN_MAKE_OFFER`.
/// The limit is `transfer_rules::FOREIGN_PLAYER_LIMIT`, 0 for a Turkish club (`KEY_NATION_TURKEY`) outside the Turkish
/// Premier Division. The players the move brings in, foreign to the club's nation (`exe::database_is_staff_foreign(p,
/// nation, 1)`): as the buyer (`offer.buyer_club_id()` = the club's id), the target (`offer.target_staff_id()`) counts 1,
/// and any foreign exchange player in the offer (`offer.exchange_players()`, walked with `exe::llist_get_next` from its
/// head) skips the check; as the seller, the foreign exchange players count, less 1 when the target is foreign, and
/// none skips the check. With a positive count, the squad (`club.squad()`, no id check) is counted on top of it, the
/// others starting at minus the count: as soon as the total passes the limit, reason 0x14 and 0; the scan stops when
/// the others pass 50 - limit. Then, for the buyer only, the windows (`date_in_transfer_window`): outside them,
/// reason 9 and 0; otherwise 1.
/// Original: `TRANSFER_RULES::can_make_offer` (0x8e8800, transfer_offer.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn transfer_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let club = Club(club);
    let offer = Offer(offer);
    let mut limit = read_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT) as i8;
    let division = club.division().addr();
    if read_u32(club.nation().addr()) == read_u32(KEY_NATION_TURKEY) && (division == 0 || read_u32(division) != read_u32(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION)) {
        limit = 0;
    }
    let foreign = |p: usize| exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0;
    let buyer = read_usize(club.addr()) == offer.buyer_club_id();
    let list = offer.exchange_players().addr();
    // the exchange players in the offer: Some(foreign) for each, from the head
    let next = || {
        let d = exe::llist_get_next(list);
        if d == 0 { None } else { Some(foreign(staff_at(read_i32(d) as isize).addr())) }
    };
    let mut count: i8 = 0;
    let mut check = true;
    if buyer {
        if foreign(staff_at(offer.target_staff_id() as isize).addr()) {
            count = 1;
        }
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            while let Some(f) = next() {
                if f {
                    check = false; // a foreign exchange player skips the limit
                    break;
                }
            }
        }
    } else if list == 0 {
        check = false;
    } else {
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        while let Some(f) = next() {
            if f {
                count = count.wrapping_add(1);
            }
        }
        if count == 0 {
            check = false;
        } else if foreign(staff_at(offer.target_staff_id() as isize).addr()) {
            count = count.wrapping_sub(1);
        }
    }
    if check && count > 0 {
        let mut others = count.wrapping_neg();
        for k in 0..50usize {
            let p = club.squad(k);
            if p != 0 && foreign(p) {
                count = count.wrapping_add(1);
            } else {
                others = others.wrapping_add(1);
            }
            if count > limit || others as i32 > 0x32 - limit as i32 {
                break;
            }
        }
        if count > limit {
            covered!("sx21_1 offer over the foreign limit");
            if !reason.is_null() {
                *reason = 0x14; // sx21_1: offer over the foreign limit
            }
            return 0;
        }
    }
    if read_usize(club.addr()) != offer.buyer_club_id() || date_in_transfer_window(this, date) {
        return 1;
    }
    covered!("sx21_1 offer outside the window");
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

/// Comparer for `transfer_rules_can_make_offer` (the walk of the exchange list moves only its cursor, reset first).
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_can_make_offer(this: usize, offer: usize, club: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    compare_rule_check("transfer_rules_can_make_offer", reason, || o(this, offer, club, date, reason), || transfer_rules_can_make_offer(this, offer, club, date, reason), || format!("rules {:#x}, offer {:#x}, club {:#x}", this, offer, club))
}
