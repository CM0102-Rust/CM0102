// ---------- cx03_17: the Asian Club Championship fate news and two Asian Cup Winners' Cup helpers (asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// rule: the Asian Club Championship news of a club's fate: in the zone rounds (stages below 1) the cup tie result; in
/// the two groups (stages 1 and 2) qualification through the generic group sentence and elimination with "{club} have
/// failed to qualify from Phase 2 of the {competition}." (the full name) with a news object, "{club} knocked out of
/// {competition}" (the short name) without; from the semi-finals on the generic competition news.
///
/// `build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28). Stage (a
/// signed byte) below 1: `exe::comp_util_write_cup_tie_result_news`(club_id, a5, a8, outcome, a6, text) (cdecl). Below
/// 3: outcome 6, `exe::comp_util_write_group_qualify_news`(club_id, a4, the competition `+4`, 6, news, text) (cdecl);
/// 0xfe, `european_cup_fate_text` with the format 0x987ae0, `competition::GENDER_NAME` and `competition::NAME` when
/// `news` is set, else the format 0x987a80 ("{club} knocked out of {competition}"), `competition::SHORT_GENDER_NAME` and
/// `competition::SHORT_NAME`, the competition linked; other outcomes nothing. From stage 3: Comp::build_fate_news_text
/// (`exe::comp_build_fate_news_text`, thiscall, ret 0x28) with the same arguments but a7 cleared.
/// Original: `AsiaClubChamp::build_fate_news_text` (0x40b460, asia_club_champ.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn asia_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = Competition(read_usize(this + 4));
    if f < 1 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    if f >= 3 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
    if d == 6 {
        exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
    } else if d == 0xfe {
        covered!("cx03_17 asian club championship knocked out in the groups");
        if news != 0 {
            european_cup_fate_text(text, club_id, 0x987ae0, comp.gender_name(), comp.addr() + competition::NAME, comp.addr());
        } else {
            european_cup_fate_text(text, club_id, 0x987a80, comp.short_gender_name(), comp.addr() + competition::SHORT_NAME, comp.addr());
        }
    }
}

/// The slot of the club's nation in a list of `count` nation ids (`club.nation()`, the id its record starts with; no
/// club or no nation: −1, which no slot holds in practice), or 0xff when it is not there. `count` is a signed byte.
/// Original: `asia_cup_winner_find_club_nation_slot` (0x40cba0, asia_cup_winner.cpp, cdecl).
pub(crate) unsafe extern "C" fn asia_cup_winner_find_club_nation_slot(club: usize, list: usize, count: u32) -> u8 {
    let nation = if club != 0 && Club(club).nation().addr() != 0 { read_u32(Club(club).nation().addr()) } else { 0xffff_ffff };
    let count = count as u8 as i8;
    let mut k: i8 = 0;
    while k < count {
        if read_u32(list + k as usize * 4) == nation {
            return k as u8;
        }
        k += 1;
    }
    0xff
}

/// rule: whether a club plays the Asian Club Championship this season: it is among the cup's teams, or among the
/// teams of its first stage (the same slots).
///
/// The Asian Club Championship is found once and kept (the static at 0xad9c5c): the first active key-nation entry
/// (`key_nation_entry`, `G_KEY_NATION_ACTIVE_COUNT` of them) with no nation whose continent (+4) is Asia
/// (`KEY_CONTINENT_ASIA`), its competition object ([[+0x10]]); none: the original reads through a null object. Then
/// each of the cup's `cup::TEAM_COUNT` slots, in the cup's `cup::TEAM_TABLE` and in the same slot of its first stage's
/// (`cup::STAGES`[0]): the club there gives 1. Otherwise 0.
/// Original: `asia_cup_winner_club_in_club_championship` (0x40e7b0, asia_cup_winner.cpp, cdecl).
pub(crate) unsafe extern "C" fn asia_cup_winner_club_in_club_championship(club: usize) -> u32 {
    let mut champ = read_usize(0xad9c5c); // the Asian Club Championship, found once
    if champ == 0 {
        let n = read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8 as i16;
        let mut k: i16 = 0;
        while k < n {
            let entry = key_nation_entry(k as i8);
            let continent = read_usize(entry + 4);
            if continent != 0 && read_usize(continent) == read_usize(KEY_CONTINENT_ASIA) && read_usize(entry) == 0 {
                covered!("cx03_17 asian club championship found");
                champ = read_usize(read_usize(entry + 0x10)); // the entry's competition object
                write_usize(0xad9c5c, champ);
                break;
            }
            k += 1;
        }
    }
    let cup = Cup(champ);
    let first = Cup(read_usize(cup.stages()));
    let n = cup.team_count();
    let mut i: i16 = 0;
    while i < n {
        let slot = i as usize * 6;
        if read_usize(cup.team_table() + slot) == club || read_usize(first.team_table() + slot) == club {
            return 1;
        }
        i += 1;
    }
    0
}

/// Comparer for `asia_club_champ_build_fate_news_text`. The paths that only write the text (the zone tie result 0x4b0b80 and the "knocked out" text) side by side with
/// `compare_text_writer`; the group sentence and the generic competition news (which may also write the news object)
/// only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    let f = stage as u8 as i8;
    let text_only = text != 0 && (f < 1 || (f < 3 && outcome as u8 == 0xfe));
    if !text_only {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| asia_club_champ_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("asia_club_champ_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, f));
}

/// Comparer for `asia_cup_winner_find_club_nation_slot` (pure: both run, the slots compared).
pub(crate) unsafe extern "C" fn compare_asia_cup_winner_find_club_nation_slot(club: usize, list: usize, count: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%)(club, list, count);
    let c = asia_cup_winner_find_club_nation_slot(club, list, count);
    SCOREBOARD.record(r == c, || format!("asia_cup_winner_find_club_nation_slot({:#x}, {} ids) original {} copy {}", club, count as u8 as i8, r, c));
    r
}

/// Comparer for `asia_cup_winner_club_in_club_championship` (both run: the copy finds the cup the original kept; the
/// answers compared).
pub(crate) unsafe extern "C" fn compare_asia_cup_winner_club_in_club_championship(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(club);
    let c = asia_cup_winner_club_in_club_championship(club);
    SCOREBOARD.record(r == c, || format!("asia_cup_winner_club_in_club_championship({:#x}) original {} copy {}", club, r, c));
    r
}
