// ---------- sx19b_1: a ruling body's match and its destructor (ruling_body/ruling_body.cpp) ----------

/// `RulingBody::process_match_cards` (vtable +8 of every ruling body; thiscall(body, match record row, fixture), ret 8).
unsafe fn ruling_body_process_match_cards(this: usize, row: usize, fixture: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(read_usize(read_usize(this) + 8));
    f(this, row, fixture);
}

/// `Discipline::handle_referee_assault` (0x55e300, discipline.cpp; thiscall(discipline, row, fixture, body), ret 0xc).
const FN_DISCIPLINE_HANDLE_REFEREE_ASSAULT: usize = 0x55e300;

/// Whether the person is on a contract of type 5 (`contract_type() & 0x3f`), whose matches the ruling body skips.
unsafe fn on_contract_type_5(person: usize) -> bool {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    !c.is_null() && c.contract_type() & 0x3f == 5
}

/// One served match on the bans of each of the club's 50 squad players under this body's rule set, or, for a national
/// side that is not a club side and has the fixture's side mark set, the national team bans served at once.
unsafe fn ruling_body_serve_squad(this: usize, club: Club, side_mark: u8) {
    let d = read_usize(G_DISCIPLINE);
    if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 && side_mark != 0 {
        covered!("sx19b_1 national side served");
        serve_international_bans(d, club.addr());
        return;
    }
    for i in 0..50 {
        let p = club.squad(i);
        if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || on_contract_type_5(p) {
            continue;
        }
        exe::discipline_update_ban_progress(d, read_u32(p), read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 1); // sx19b_1: one match served
    }
}

/// Counts a served match on each squad player's bans, then passes each player's match record to the body's card rules.
///
/// `RulingBody::process_match(fixture, record, home, away, serve)`, from `Discipline::process_match` (each body that
/// applies to the fixture, with the home and away flags of the sides it covers). In a friendly
/// (`Friendly::is_friendly_competition` of the fixture's competition) or a match of the reserve competition
/// (`fixture::COMP_ID` = `[KEY_CLUB_COMP_RESERVE]`) only some cards count (below). With `serve`, each covered side
/// (home: `fixture::HOME_CLUB` with `home_side_mark()`, away: `fixture::AWAY_CLUB` with `away_side_mark()`) serves one
/// match (`ruling_body_serve_squad`). Then, for each covered side, the 20 rows of its team block in the match record
/// (`record` + 0x625 × side + 0x190, 0x49 bytes each): a row with a person (`staff_id`) and a player (`player_id`), a
/// substitute (row 11 on) only when he came on (`minute_on` ≠ 0xff), a person below `G_STAFF_COUNT`, without a referee
/// assault ban (`Discipline::handle_referee_assault` with the row, the fixture and the body; 1 ends the row), not on a
/// contract of type 5 unless the home club is a national side, and, in a friendly or reserve match, only a red card
/// (`red_card` ≠ 0 with fewer than 2 bookings) whose offence code (`first_card_offence` or `second_card_offence`) is 10
/// or more, and only in the reserve competition, goes to the body's card rules (vtable +8, `process_match_cards`).
/// Original: `RulingBody::process_match` (0x7e7ff0, ruling_body.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ruling_body_process_match(this: usize, fixture: usize, record: usize, home: u32, away: u32, serve: u32) {
    let f = Fixture(fixture);
    let reserve = read_u32(KEY_CLUB_COMP_RESERVE);
    let friendly = exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) != 0 || f.comp_id() as u32 == reserve;
    let national_home = exe::database_is_national_team(f.home_club()) != 0;
    if home != 0 && serve != 0 {
        ruling_body_serve_squad(this, f.home_club(), f.home_side_mark());
    }
    if away != 0 && serve != 0 {
        ruling_body_serve_squad(this, f.away_club(), f.away_side_mark() as u8);
    }
    let assault: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_DISCIPLINE_HANDLE_REFEREE_ASSAULT);
    for side in 0..2usize {
        if (side == 0 && home == 0) || (side == 1 && away == 0) {
            continue;
        }
        for k in 0..20usize {
            let row = MatchRecordRow(record + side * 0x625 + 0x190 + k * 0x49);
            if row.staff_id() == -1 || row.player_id() == u32::MAX || (k > 10 && row.minute_on() as u8 == 0xff) {
                continue;
            }
            let person = staff_at(row.staff_id() as isize);
            if !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            if assault(read_usize(G_DISCIPLINE), row.addr(), fixture, this) != 0 {
                continue;
            }
            if !national_home && on_contract_type_5(person.addr()) {
                continue;
            }
            if friendly {
                if f.comp_id() as u32 != reserve || row.red_card() == 0 || row.yellow_cards() >= 2
                    || ((row.first_card_offence() as u8) < 10 && (row.second_card_offence() as u8) < 10) {
                    continue;
                }
                covered!("sx19b_1 reserve red card");
            }
            covered!("sx19b_1 card rules");
            ruling_body_process_match_cards(this, row.addr(), fixture); // sx19b_1: the body's card rules
        }
    }
}

/// Comparer for `ruling_body_process_match`: the served matches, the assault bans, the card bans and their news
/// cannot be handed out twice, so only the original runs (the career and the mutations prove the copy).
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_process_match(this: usize, fixture: usize, record: usize, home: u32, away: u32, serve: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I0%%)(this, fixture, record, home, away, serve);
}

/// Destroys a ruling body: its vtable back to `RULING_BODY_VTABLE`, its pending ban list (`RULING_BODY_LIST`)
/// destroyed (`LLIST::~LLIST`) and deleted, and its entry array (`ruling_body::ENTRIES`) freed. The fields keep their
/// values.
/// Original: `RulingBody::~RulingBody` (0x7e7860, ruling_body.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn ruling_body_destructor(this: usize) {
    write_u32(this, RULING_BODY_VTABLE);
    let list = read_usize(this + RULING_BODY_LIST);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    let entries = read_usize(this + ruling_body::ENTRIES);
    if entries != 0 {
        exe::crt_free(entries);
    }
}

/// Comparer for `ruling_body_destructor`: it frees memory, so only the original runs; checked afterwards: the vtable.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    SCOREBOARD.record(read_u32(this) == RULING_BODY_VTABLE, || format!("ruling_body_destructor({:#x}): vtable {:#x}", this, read_u32(this)));
}
