// --- sx17_7: what a scout, coach or physio notices in a player (player_search.cpp): player_search_assess_player_traits
// (0x7bd9d0), player_search_judge_notices_trait (0x7bdd70) and player_search_write_traits_report_text (0x7bdee0) ---

/// The trait bits of the strengths and weaknesses words (`player_search_assess_player_traits`); the set pieces and
/// consistency only as strengths, consistency as a weakness has its own bit.
const TRAIT_LEADERSHIP: u16 = 0x1;
const TRAIT_LOYALTY: u16 = 0x2;
const TRAIT_PRESSURE: u16 = 0x4;
const TRAIT_DETERMINATION: u16 = 0x8;
const TRAIT_WORK_RATE: u16 = 0x10;
const TRAIT_PROFESSIONALISM: u16 = 0x20;
const TRAIT_TEMPERAMENT: u16 = 0x40;
const TRAIT_BRAVERY: u16 = 0x80;
const TRAIT_CORNERS: u16 = 0x100;
const TRAIT_PENALTIES: u16 = 0x200;
const TRAIT_FREE_KICKS: u16 = 0x400;
const TRAIT_INCONSISTENT: u16 = 0x800;
const TRAIT_CONSISTENCY: u16 = 0x1000;
/// The goalkeeper rating of a player sheet that marks a goalkeeper (no set-piece strengths for him).
const GOALKEEPER_RATING: i8 = 0x14;

/// How far the judge's eye is off for this player: 0 when 30 − his judging ability (non-player attribute 5) is below
/// the player's id modulo 30; otherwise (30 − ability) mod 8 + id mod 7 − 3, negated when odd.
unsafe fn player_search_judging_offset(judge: Staff, player: Staff) -> i8 {
    let blind = 30i8.wrapping_sub(exe::staff_get_non_player_attribute(judge, 5)) as i32;
    let id = read_i32(player.addr());
    if blind < id % 30 {
        return 0;
    }
    let offset = ((blind % 8) as i8).wrapping_add((id % 7) as i8).wrapping_sub(3);
    if offset as i32 % 2 != 0 { offset.wrapping_neg() } else { offset }
}

/// Sets the strength and weakness bits a judge (scout, coach or physio, with a club) notices in a player. Both words
/// are zeroed first; a judge without a club: 0. In languages 4 and 5 nothing is assessed (1). A player without a sheet:
/// 0. With the judge's offset (`player_search_judging_offset`), each attribute above 14 + offset (consistency above 15)
/// is a strength when the judge notices it (`player_search_judge_notices_trait`): leadership, consistency, loyalty,
/// pressure, determination, work rate, professionalism, temperament, bravery and, unless a goalkeeper, corners, free
/// kicks and penalties. Only a regen (`staff_state_status_flags::REGEN_MARK`) has weaknesses: loyalty (with the judge's
/// club below reputation 4750), consistency, pressure, determination, work rate, professionalism, temperament and
/// bravery below 6 + offset, when noticed. Answers 1.
/// Defect of the original: for work rate and professionalism as strengths the judge is asked about the player's
/// determination value instead of the attribute's own.
/// Original: `player_search_assess_player_traits` (0x7bd9d0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_assess_player_traits(player: Staff, judge: Staff, strengths: *mut u16, weaknesses: *mut u16) -> u8 {
    let judge_club = judge.club_job();
    *strengths = 0;
    *weaknesses = 0;
    if judge_club.is_null() {
        return 0;
    }
    if matches!(exe::langlib_get_language(), 4 | 5) {
        return 1;
    }
    let offset = player_search_judging_offset(judge, player) as i32;
    let sheet = player.player();
    if sheet.is_null() {
        return 0;
    }
    let notices = |value: i8| player_search_judge_notices_trait(judge, player, value as u32) != 0;
    let high = offset + 14; // sx17_7: the strength threshold
    let mut found = 0u16;
    let mut strong = |attribute: i8, threshold: i32, asked: i8, bit: u16| {
        if attribute as i32 > threshold && notices(asked) {
            found |= bit;
        }
    };
    strong(sheet.leadership(), high, sheet.leadership(), TRAIT_LEADERSHIP);
    strong(sheet.consistency(), offset + 15, sheet.consistency(), TRAIT_CONSISTENCY);
    strong(player.loyalty(), high, player.loyalty(), TRAIT_LOYALTY);
    strong(player.pressure(), high, player.pressure(), TRAIT_PRESSURE);
    strong(player.determination(), high, player.determination(), TRAIT_DETERMINATION);
    strong(sheet.work_rate(), high, player.determination(), TRAIT_WORK_RATE);
    strong(player.professionalism(), high, player.determination(), TRAIT_PROFESSIONALISM);
    strong(player.temperament(), high, player.temperament(), TRAIT_TEMPERAMENT);
    strong(sheet.bravery(), high, sheet.bravery(), TRAIT_BRAVERY);
    if sheet.goalkeeper() != GOALKEEPER_RATING {
        strong(sheet.corners(), high, sheet.corners(), TRAIT_CORNERS);
    }
    if sheet.goalkeeper() != GOALKEEPER_RATING {
        strong(sheet.free_kicks(), high, sheet.free_kicks(), TRAIT_FREE_KICKS);
    }
    if sheet.goalkeeper() != GOALKEEPER_RATING {
        strong(sheet.penalties(), high, sheet.penalties(), TRAIT_PENALTIES);
    }
    *strengths = found;
    let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(player.addr()) as usize).wrapping_mul(staff_state::SIZE));
    if read_u8(state + staff_state::STATUS_FLAGS) & staff_state_status_flags::REGEN_MARK == 0 {
        *weaknesses = 0;
        return 1;
    }
    covered!("sx17_7 a regen's weaknesses");
    let low = offset + 6;
    let mut weak = 0u16;
    if judge_club.reputation() < 0x128e && (player.loyalty() as i32) < low && notices(player.loyalty()) {
        weak |= TRAIT_LOYALTY;
    }
    for (attribute, bit) in [(sheet.consistency(), TRAIT_INCONSISTENT), (player.pressure(), TRAIT_PRESSURE),
                             (player.determination(), TRAIT_DETERMINATION), (sheet.work_rate(), TRAIT_WORK_RATE),
                             (player.professionalism(), TRAIT_PROFESSIONALISM), (player.temperament(), TRAIT_TEMPERAMENT),
                             (sheet.bravery(), TRAIT_BRAVERY)] {
        if (attribute as i32) < low && notices(attribute) {
            weak |= bit;
        }
    }
    *weaknesses = weak;
    1
}

/// Comparer for `player_search_assess_player_traits`: it only reads, so both run and the two words and the answer must
/// match.
pub(crate) unsafe extern "C" fn compare_player_search_assess_player_traits(player: Staff, judge: Staff, strengths: *mut u16, weaknesses: *mut u16) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(Staff, Staff, *mut u16, *mut u16) -> u8>(%%I0%%)(player, judge, strengths, weaknesses);
    let (s, w) = (*strengths, *weaknesses);
    let (mut s2, mut w2) = (0u16, 0u16);
    let n = player_search_assess_player_traits(player, judge, &mut s2, &mut w2);
    SCOREBOARD.record(r == n && s == s2 && w == w2, || format!("player_search_assess_player_traits(player {:#x}, judge {:#x}) original=({}, {:#x}, {:#x}) rust=({}, {:#x}, {:#x})",
        player.addr(), judge.addr(), r, s, w, n, s2, w2));
    r
}

/// Whether a judge notices an attribute `value` of a player. Without a judge or one who is not a non-player: no. The
/// bar starts at 35 − value, plus 2 a year under 22. The player's reputation as the judge sees it
/// (`database_get_staff_reputation_for_viewer`), when both have been at the same club over 150 days, becomes the larger
/// of 1.5 × it and (days the player has been there + 1300) × 5; above 5000 the bar drops by (10000 − reputation) / 1000.
/// The judge's eye: (both ids' sum mod 17) + 4 × |his judging rating (`non_player_attribute_rating` of non-player
/// byte +0x19) − 10| − 8 must pass the bar.
/// Original: `player_search_judge_notices_trait` (0x7bdd70, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_judge_notices_trait(judge: Staff, player: Staff, value: u32) -> u8 {
    if judge.is_null() || judge.non_player().is_null() {
        return 0;
    }
    let mut bar = 35 - value as u8 as i8 as i32;
    let age = read_i8(player.addr() + staff::AGE);
    if age < 22 {
        bar += (22 - age as i32) * 2; // sx17_7: the young player is harder to read
    }
    let mut reputation = exe::database_get_staff_reputation_for_viewer(player, judge, 0) as i32;
    let days_at_club = |p: Staff| exe::date_days_since(G_CURRENT_DATE, read_u32(p.addr() + staff::DATE_JOINED_CLUB), read_u32(p.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    if judge.club_job() == player.club_job() && days_at_club(judge) > 150 && days_at_club(player) > 150 {
        covered!("sx17_7 the judge knows him from his club");
        let seen = reputation as i16 as f64 * read_f64(rdata::F64_1_5);
        let known = ((days_at_club(player) + 1300) * 5) as f64;
        reputation = ftol(if known > seen { ((days_at_club(player) + 1300) * 5) as f64 } else { seen });
    }
    let reputation = reputation as i16 as i32;
    if reputation > 5000 {
        bar += (10000 - reputation) / -1000;
    }
    let np = judge.non_player();
    let eye = (exe::non_player_attribute_rating(np, read_i8(np.addr() + 0x19)) as i32 - 10).abs(); // +0x19: the judging attribute
    let roll = (read_i32(judge.addr()).wrapping_add(read_i32(player.addr()))) % 17;
    (roll + eye * 4 - 8 > bar) as u8
}

/// Comparer for `player_search_judge_notices_trait`: it only reads, so both run and the answers must match.
pub(crate) unsafe extern "C" fn compare_player_search_judge_notices_trait(judge: Staff, player: Staff, value: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(Staff, Staff, u32) -> u8>(%%I1%%)(judge, player, value);
    let n = player_search_judge_notices_trait(judge, player, value);
    SCOREBOARD.record(r == n, || format!("player_search_judge_notices_trait(judge {:#x}, player {:#x}, {}) original={} rust={}", judge.addr(), player.addr(), value as u8 as i8, r, n));
    r
}

/// The phrases a traits report is made of (each up to 100 bytes, as the original's buffers).
struct TraitPhrases {
    specialities: [[u8; 0x64]; 2],
    positives: [[u8; 0x64]; 2],
    negatives: [[u8; 0x64]; 2],
    speciality_count: i8,
    positive_count: i8,
    negative_count: i8,
}

/// Writes the next phrase of a list (up to two) with `langlib_sprintf`.
unsafe fn trait_phrase(list: &mut [[u8; 0x64]; 2], count: &mut i8, text: usize) {
    story_lang!(list[*count as usize].as_mut_ptr(), text);
    *count += 1;
}

/// The set-piece phrases: corners ("an expert in dead-ball situations" with free kicks too, else "an excellent corner
/// taker"), else free kicks ("a free-kick specialist"); then penalties ("an iron nerve" with pressure, else "takes an
/// excellent penalty").
unsafe fn trait_specialities(p: &mut TraitPhrases, strengths: u16) {
    if strengths & TRAIT_CORNERS != 0 {
        let text = if strengths & TRAIT_FREE_KICKS != 0 { 0xa7598c } else { 0xa7596c };
        trait_phrase(&mut p.specialities, &mut p.speciality_count, text);
    } else if strengths & TRAIT_FREE_KICKS != 0 {
        trait_phrase(&mut p.specialities, &mut p.speciality_count, 0xa75950);
    }
    if strengths & TRAIT_PENALTIES != 0 {
        let text = if strengths & TRAIT_PRESSURE != 0 { 0xa7591c } else { 0xa75900 };
        trait_phrase(&mut p.specialities, &mut p.speciality_count, text);
    }
}

/// The good points, two at most: "reliability" (consistency and professionalism) or "consistency", else "professional
/// approach"; then loyalty, leadership, pressure, determination, work rate, maturity and bravery in that order.
unsafe fn trait_positives(p: &mut TraitPhrases, strengths: u16) {
    if strengths & TRAIT_CONSISTENCY != 0 {
        let text = if strengths & TRAIT_PROFESSIONALISM != 0 { 0xa758f4 } else { 0xa758e8 };
        trait_phrase(&mut p.positives, &mut p.positive_count, text);
    } else if strengths & TRAIT_PROFESSIONALISM != 0 {
        trait_phrase(&mut p.positives, &mut p.positive_count, 0xa758d0);
    }
    for (bit, text) in [(TRAIT_LOYALTY, 0xa758c8), (TRAIT_LEADERSHIP, 0xa758bc), (TRAIT_PRESSURE, 0xa758a0), (TRAIT_DETERMINATION, 0xa75890),
                        (TRAIT_WORK_RATE, 0xa75880), (TRAIT_TEMPERAMENT, 0xa75874), (TRAIT_BRAVERY, 0xa7586c)] {
        if strengths & bit != 0 && p.positive_count < 2 {
            trait_phrase(&mut p.positives, &mut p.positive_count, text);
        }
    }
}

/// The bad points: low loyalty reads as ambition (under 23 and the club below reputation 3250 "ambitious...", below
/// 6250 "very ambitious..."; over 25 "loyal only to the highest paying club"; otherwise a good point, "hungry for
/// success"); then, as the first bad point only, "lazy", "inconsistent", "unprofessional"; "highly strung" up to two;
/// pressure as the first only ("too young..." under 21, else "unable to handle pressure"); determination up to two
/// ("lacking in determination", or "also lacks determination" after another).
unsafe fn trait_negatives(p: &mut TraitPhrases, weaknesses: u16, club_reputation: i16, age: i8) {
    if weaknesses & TRAIT_LOYALTY != 0 {
        if club_reputation < 0xcb2 && age < 23 {
            trait_phrase(&mut p.negatives, &mut p.negative_count, 0xa75820);
        } else if club_reputation < 0x186a && age < 23 {
            trait_phrase(&mut p.negatives, &mut p.negative_count, 0xa757c4);
        } else if age > 25 {
            trait_phrase(&mut p.negatives, &mut p.negative_count, 0xa7579c);
        } else if p.positive_count < 2 {
            trait_phrase(&mut p.positives, &mut p.positive_count, 0xa75788);
        }
    }
    for (bit, text) in [(TRAIT_WORK_RATE, 0xa75780), (TRAIT_INCONSISTENT, 0xa75770), (TRAIT_PROFESSIONALISM, 0xa75760)] {
        if weaknesses & bit != 0 && p.negative_count == 0 {
            trait_phrase(&mut p.negatives, &mut p.negative_count, text);
        }
    }
    if weaknesses & TRAIT_TEMPERAMENT != 0 && p.negative_count < 2 {
        trait_phrase(&mut p.negatives, &mut p.negative_count, 0xa75750);
    }
    if weaknesses & TRAIT_PRESSURE != 0 && p.negative_count == 0 {
        let text = if age < 0x15 { 0xa75728 } else { 0xa7570c };
        trait_phrase(&mut p.negatives, &mut p.negative_count, text);
    }
    if weaknesses & TRAIT_DETERMINATION != 0 && p.negative_count < 2 {
        let text = if p.negative_count == 0 { 0xa756f0 } else { 0xa756d4 };
        trait_phrase(&mut p.negatives, &mut p.negative_count, text);
    }
}

/// Writes the report sentences about a player's set pieces and good or bad points into `out`; with no trait at all, or
/// when nothing is said, `out` is emptied (without `out`, nothing). The phrases (`trait_specialities`,
/// `trait_positives`, `trait_negatives`) are joined into the speciality sentences (with the player's name, and with
/// "the player"), the good points ("X" or "X and his Y") and the bad points ("X" or "X and Y"), then into the sentence
/// the combination calls for: with good points, with or without bad points and set pieces (four formats naming the
/// scout and the player); without good points, the bad points with or without the set pieces, or the set pieces alone.
/// Defect of the original: in the "good and bad points, no set pieces" sentence the speciality count (always 0 there)
/// is passed where the other formats pass a gender value.
/// Original: `player_search_write_traits_report_text` (0x7bdee0, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_write_traits_report_text(strengths: u32, weaknesses: u32, club_reputation: u32, age: u32,
                                                                      out: usize, scout: usize, player: usize) {
    if out == 0 {
        return;
    }
    let (strengths, weaknesses) = (strengths as u16, weaknesses as u16);
    if strengths == 0 && weaknesses == 0 {
        write_u8(out, 0);
        return;
    }
    let mut p = TraitPhrases { specialities: [[0; 0x64]; 2], positives: [[0; 0x64]; 2], negatives: [[0; 0x64]; 2],
                               speciality_count: 0, positive_count: 0, negative_count: 0 };
    trait_specialities(&mut p, strengths);
    trait_positives(&mut p, strengths);
    trait_negatives(&mut p, weaknesses, club_reputation as i16, age as u8 as i8);
    let (mut named, mut plain, mut good, mut bad) = ([0u8; 0x3e8], [0u8; 0x3e8], [0u8; 0x3e8], [0u8; 0x3e8]);
    let (spec0, spec1) = (p.specialities[0].as_ptr(), p.specialities[1].as_ptr());
    match p.speciality_count {
        1 => {
            story_lang!(named.as_mut_ptr(), 0xa75604, player, spec0); // "<player> <speciality>"
            story_lang!(plain.as_mut_ptr(), 0xa755e0, spec0); // "the player <speciality>"
        }
        2 => {
            story_lang!(named.as_mut_ptr(), 0xa75680, player, spec0, spec1);
            story_lang!(plain.as_mut_ptr(), 0xa7563c, spec0, spec1);
        }
        _ => {}
    }
    match p.positive_count {
        1 => story_lang!(good.as_mut_ptr(), 0xa75548, p.positives[0].as_ptr()),
        2 => story_lang!(good.as_mut_ptr(), 0xa75578, p.positives[0].as_ptr(), p.positives[1].as_ptr()),
        _ => {}
    }
    match p.negative_count {
        1 => story_lang!(bad.as_mut_ptr(), 0xa754b4, p.negatives[0].as_ptr()),
        2 => story_lang!(bad.as_mut_ptr(), 0xa754e4, p.negatives[0].as_ptr(), p.negatives[1].as_ptr()),
        _ => {}
    }
    let (named, plain, good, bad) = (named.as_ptr(), plain.as_ptr(), good.as_ptr(), bad.as_ptr());
    let has_specialities = p.speciality_count != 0;
    match (p.positive_count != 0, p.negative_count != 0, has_specialities) {
        (true, false, true) => story_lang!(out, 0xa753d0, 0x64, 0x64, 0x64, scout, player, good, plain),
        (true, false, false) => story_lang!(out, 0xa75360, 0x64, 0x64, player, good),
        (true, true, true) => story_lang!(out, 0xa7522c, 0x64, 0x64, 0x64, player, good, bad, scout, plain),
        (true, true, false) => story_lang!(out, 0xa75150, 0x64, 0x64, 0x64, p.speciality_count as i32, scout, player, good, bad),
        (false, true, true) => story_lang!(out, 0xa750b0, named, bad), // sx17_7: the set pieces and the bad points
        (false, true, false) => story_lang!(out, 0xa75048, scout, bad),
        (false, false, true) => story_lang!(out, 0xa74fec, named),
        (false, false, false) => write_u8(out, 0),
    }
}

/// Comparer for `player_search_write_traits_report_text`: it only writes `out`, so the original writes first, then the
/// copy, the two texts are compared and the original's is put back.
pub(crate) unsafe extern "C" fn compare_player_search_write_traits_report_text(strengths: u32, weaknesses: u32, club_reputation: u32, age: u32,
                                                                              out: usize, scout: usize, player: usize) {
    original_fn::<unsafe extern "C" fn(u32, u32, u32, u32, usize, usize, usize)>(%%I2%%)(strengths, weaknesses, club_reputation, age, out, scout, player);
    if out == 0 {
        return;
    }
    let original = c_string_bytes(out);
    player_search_write_traits_report_text(strengths, weaknesses, club_reputation, age, out, scout, player);
    let copy = c_string_bytes(out);
    core::ptr::copy_nonoverlapping(original.as_ptr(), out as *mut u8, original.len());
    write_u8(out + original.len(), 0);
    SCOREBOARD.record(original == copy, || format!("player_search_write_traits_report_text({:#x}, {:#x}) original={:?} rust={:?}", strengths, weaknesses,
        String::from_utf8_lossy(&original), String::from_utf8_lossy(&copy)));
}
