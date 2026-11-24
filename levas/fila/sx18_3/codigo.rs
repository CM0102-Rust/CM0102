// ---------- sx18_3: the international (FIFA) ruling body's card rules and new season (ruling_body/rb_international.cpp) ----------

/// The international card rules (`plain_card_verdict`): one match for a red card and one for every second booking
/// (counted one per match), under the FIFA rule set in the fourth slot (rules ff ff ff 00).
pub(crate) unsafe fn international_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_3 international cards");
    plain_card_verdict(row, entry, false, 2, [0xff, 0xff, 0xff, 0]) // sx18_3: every second international booking
}

/// Applies a player's cards of one match under the international rules (`international_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbInternational::process_match_cards` (0x7df1c0, rb_international.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_international_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| international_card_verdict(MatchRecordRow(row), e));
}

/// The international body's new season: for each occupied entry (14 bytes in `ruling_body::ENTRIES`, count
/// `ruling_body::ENTRY_COUNT`; id −1 = free): without a ban (`ruling_body_entry::BAN_INDEX` −1) and with no booking nor
/// red card, the player's slot is cleared (`Discipline::set_slot` with the body's record slot and rule set, −1) and the
/// entry freed; with a ban whose start year (+6 of the 20-byte ban) is 4 or more years before today's year, the ban is
/// released (`Discipline::release_ban`), the slot cleared and the entry freed; the others stay as they are (their counts
/// are not cleared, unlike the base `RulingBody::start_new_season`). At the end the entries are compacted
/// (`RulingBody::compact_entries`).
/// Original: `RbInternational::start_new_season` (0x7df3c0, rb_international.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rb_international_start_new_season(this: usize) {
    covered!("sx18_3 international new season");
    let d = read_usize(G_DISCIPLINE);
    let mut e = read_usize(this + ruling_body::ENTRIES);
    let mut i = 0i32;
    while i < read_i32(this + ruling_body::ENTRY_COUNT) {
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 {
            let s = read_i16(e + ruling_body_entry::BAN_INDEX);
            let free = if s == -1 {
                read_u8(e + ruling_body_entry::YELLOW_CARDS) == 0 && read_u8(e + ruling_body_entry::RED_CARDS) == 0
            } else {
                let ban = read_usize(d + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
                let old = (read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(ban + ban_record::START_DATE + 2) as i32) >= 4;
                if old {
                    // simplification: the original pushes the index with stale high 16 bits; release_ban reads only the word.
                    exe::discipline_release_ban(d, id as u32, s as u16 as u32);
                }
                old
            };
            if free {
                exe::discipline_set_slot(d, id, read_i8(this + ruling_body::RECORD_SLOT), read_u8(this + ruling_body::RULE_SET_INDEX), u32::MAX);
                write_u32(e + ruling_body_entry::PERSON_ID, u32::MAX);
            }
        }
        i += 1;
        e += 0xe;
    }
    exe::rulingbody_compact_entries(this);
}

pub(crate) unsafe extern "thiscall" fn compare_rb_international_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_card_rules(this, row, || o(this, row, fixture), |e| international_card_verdict(MatchRecordRow(row), e), "rb_international_process_match_cards");
}

/// Comparer for `rb_international_start_new_season`: it releases bans and reallocates (it cannot run twice); the
/// entries the copy would free are listed first, then only the original runs, and the compacted table must hold
/// exactly the others, in order.
pub(crate) unsafe extern "thiscall" fn compare_rb_international_start_new_season(this: usize) {
    let d = read_usize(G_DISCIPLINE);
    let mut kept = Vec::new();
    let e0 = read_usize(this + ruling_body::ENTRIES);
    for i in 0..read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize {
        let e = e0 + i * 0xe;
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id == -1 {
            continue;
        }
        let s = read_i16(e + ruling_body_entry::BAN_INDEX);
        let free = if s == -1 {
            read_u8(e + ruling_body_entry::YELLOW_CARDS) == 0 && read_u8(e + ruling_body_entry::RED_CARDS) == 0
        } else {
            let ban = read_usize(d + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
            read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(ban + ban_record::START_DATE + 2) as i32 >= 4
        };
        if !free {
            kept.push(id);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let e0 = read_usize(this + ruling_body::ENTRIES);
    let n = read_i32(this + ruling_body::ENTRY_COUNT);
    let ok = n as usize == kept.len() && kept.iter().enumerate().all(|(i, &id)| read_i32(e0 + i * 0xe) == id);
    SCOREBOARD.record(ok, || format!("rb_international_start_new_season: {} entries kept (expected {})", n, kept.len()));
}
