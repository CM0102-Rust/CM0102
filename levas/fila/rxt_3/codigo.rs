// --- rxt_3: the scout's opposition report text (player_search.cpp) ---

/// A best-player entry of the club scouting report (0x1a bytes): +0 the rating, +4 the player's id, +8/+0xc/+0x10 his
/// name ids, +0x14 the flags (`scout_day_player_flags`), +0x18 his current ability (word).
const REPORT_PLAYER_SIZE: usize = 0x1a;
/// How many entries each position pick's frame array holds (0x84 bytes).
const REPORT_PICK_ENTRIES: usize = 5;
/// The position picks of the report (sx17_8): cdecl(count (byte), entries, club (only checked for null), out) -> how many.
const FN_PLAYER_SEARCH_COLLECT_REPORT_MIDFIELDERS: usize = 0x7bfbd0;
const FN_PLAYER_SEARCH_COLLECT_REPORT_STRIKERS: usize = 0x7bfc50;
const FN_PLAYER_SEARCH_COLLECT_REPORT_DEFENDERS: usize = 0x7bfcd0;

/// One position pick (`FN_PLAYER_SEARCH_COLLECT_REPORT_*`) into `out`.
unsafe fn collect_report_pick(f: usize, count: u32, players: usize, club: Club, out: &mut [[u8; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES]) -> i8 {
    let f: unsafe extern "C" fn(u32, usize, Club, usize) -> u8 = core::mem::transmute(f);
    f(count, players, club, out.as_mut_ptr() as usize) as i8
}

/// The copy met a value the original reads from its uninitialised frame (the formation of a manager without a staff
/// sheet, the second midfielder's flags with one midfielder, the weak-point paragraph without the strength one): the
/// comparer then leaves the text alone (DEFEITOS_DO_ORIGINAL).
pub(crate) static OPPOSITION_REPORT_UNSET: AtomicBool = AtomicBool::new(false);

/// The full (style 1) or short (style 5) name of a report entry into a 0xc8-byte buffer.
unsafe fn report_player_name(e: usize, buf: &mut [u8; 0xc8], style: u32) {
    exe::database_format_name_from_ids(read_u32(e + 8), read_u32(e + 0xc), read_u32(e + 0x10), buf.as_mut_ptr(), 0xc8, style, read_u32(e + 4));
}

fn report_player_flags(e: &[u8]) -> u32 {
    u32::from_le_bytes([e[0x14], e[0x15], e[0x16], e[0x17]])
}

fn report_player_id(e: &[u8]) -> i32 {
    i32::from_le_bytes([e[4], e[5], e[6], e[7]])
}

/// The report's words: the scouted club (`target`) and the manager's own club (`own`), each with its short name and
/// gender, and the eleven answers about the scouted club (`answers`: +0..+0xb, the club state at +0xc).
struct OppositionReport {
    own: Club,
    target: Club,
    answers: [u8; 0xd],
    /// the scouted club's best players and the position picks (`exe::player_search_collect_report_*`).
    players: usize,
    count: u32,
    defenders: [[u8; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES],
    midfielders: [[u8; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES],
    strikers: [[u8; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES],
    n_def: i8,
    n_mid: i8,
    n_str: i8,
}

impl OppositionReport {
    unsafe fn gender(c: Club) -> i32 {
        read_i8(c.addr() + club::SHORT_GENDER_NAME) as i32
    }
    unsafe fn short(c: Club) -> usize {
        c.addr() + club::SHORT_NAME
    }
    /// The first best player's flags (several sentences test his position, not the pick's).
    unsafe fn first_flags(&self) -> u32 {
        read_u32(self.players + 0x14)
    }
}

/// The scouted club's formation sentence into `para`: "<manager> normally prefers <club> to play <style> <formation>"
/// with the manager (+0xcf, else the assistant +0xd3) by name; without one, "<club> are expected to play a 4-4-2
/// formation." The formation by the manager's staff sheet (`formation()` 1..9, else "4-4-2") and the style by his
/// `attacking()` and `directness()` (15 and up / 5 and below).
unsafe fn opposition_formation(r: &OppositionReport, para: usize) {
    let t = r.target;
    let mut manager = t.manager();
    if manager.is_null() {
        manager = Staff(t.assistant_manager());
    }
    let (mut formation, mut style) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (fp, sp) = (formation.as_mut_ptr() as usize, style.as_mut_ptr() as usize);
    let mut styled = false;
    let np = if manager.is_null() { NonPlayer(0) } else { manager.non_player() };
    if np.is_null() {
        story_lang!(para, 0xa78130); // "a 4-4-2"
        OPPOSITION_REPORT_UNSET.store(!manager.is_null(), Relaxed);
    } else {
        let name = match np.formation() {
            1 => 0xa782c0, // "5-3-2 Sweeper"
            2 => 0xa782b8, // "5-3-2"
            4 => 0xa782a8, // "4-3-3"
            5 => 0xa782a0, // "4-2-4"
            6 => 0xa78298, // "3-5-2"
            7 => 0xa7828c, // "4-1-2-1-2"
            8 => 0xa78284, // "4-5-1"
            9 => 0xa1594c, // "3-4-3"
            _ => 0xa782b0, // "4-4-2"
        };
        story_lang!(fp, name);
        let (att, dir) = (np.attacking(), np.directness());
        let tpl = if att >= 15 {
            Some(if dir <= 5 { 0xa78220 } else { 0xa78258 }) // "an attractive attacking" / "an attacking"
        } else if att <= 5 {
            Some(if dir >= 15 { 0xa781ec } else if dir <= 5 { 0xa781b4 } else { 0xa78188 }) // "a cautious but direct" / "a patient and defensive" / "a defensive"
        } else if dir >= 15 {
            Some(0xa78160) // "a direct"
        } else if dir <= 5 {
            Some(0xa78138) // "a patient"
        } else {
            None
        };
        if let Some(tpl) = tpl {
            story_lang!(sp, tpl, 0xad9c64);
            styled = true;
        }
    }
    let g = OppositionReport::gender(t);
    let club_name = OppositionReport::short(t);
    if manager.is_null() {
        story_lang!(para, 0xa77fa4, g, g, club_name);
        return;
    }
    let mut who = [0u8; 0x78];
    exe::database_format_staff_name(manager, who.as_mut_ptr() as usize, 0x78, 1);
    if styled {
        story_lang!(para, 0xa78084, 0x64, g, g, who.as_ptr(), club_name, sp, fp);
    } else {
        story_lang!(para, 0xa77ff4, 0x64, g, g, who.as_ptr(), club_name, fp);
    }
}

/// The squad strength sentence into `para` by the band (0 pathetic .. 6 world class), the answers +0xb (organised,
/// industrious, hard working) and +0xc (below −3: a poor league position) and the reputations of the two clubs; a band
/// outside 0..6 writes nothing (false).
unsafe fn opposition_strength(r: &OppositionReport, band: i8, para: usize) -> bool {
    let a = &r.answers;
    let (t, own) = (r.target, r.own);
    let poor = (a[0xc] as i8) < -3;
    let tpl = match band {
        0 => if a[0xb] == 0 && t.reputation() > own.reputation() { 0xa77adc } else { 0xa77a98 },
        1 => if poor { 0xa77bbc } else if a[0xb] == 0 && t.reputation() > own.reputation() { 0xa77b70 } else { 0xa77b2c },
        2 => 0xa77a54,
        3 => if a[0xb] != 0 { 0xa77c8c } else { 0xa77c48 },
        4 => if a[0xb] != 0 && t.reputation() < own.reputation() { 0xa77d8c } else if poor { 0xa77d10 } else { 0xa77cd4 },
        5 => if a[0xb] != 0 && t.reputation() < own.reputation() { 0xa77e90 } else if poor { 0xa77e14 } else { 0xa77dd0 },
        6 => if poor { 0xa77f30 } else { 0xa77eec },
        _ => return false,
    };
    let g = OppositionReport::gender(t);
    story_lang!(para, tpl, g, g, OppositionReport::short(t));
    true
}

/// The weak point of a weak team (band below 2) into `para`, by the answers: crosses (+5, with +0xb unset and a better
/// reputation than the manager's club), +5, pace (+7), +0 with +2, then +1 (with +0 and +2 unset: no creativity;
/// otherwise a reliable defence). None: false.
unsafe fn opposition_weak_point(r: &OppositionReport, para: usize) -> bool {
    let a = &r.answers;
    let own = r.own;
    let (g, name) = (OppositionReport::gender(own), OppositionReport::short(own));
    if a[5] != 0 && a[0xb] == 0 && r.target.reputation() > own.reputation() {
        story_lang!(para, 0xa77a1c);
    } else if a[5] != 0 {
        story_lang!(para, 0xa77998, g, g, name);
    } else if a[7] != 0 {
        story_lang!(para, 0xa77924, g, g, name);
    } else if a[0] != 0 && a[2] != 0 {
        story_lang!(para, 0xa778ac, g, g, name);
    } else if a[1] != 0 {
        story_lang!(para, if a[2] == 0 && a[0] == 0 { 0xa77858 } else { 0xa77828 });
    } else {
        return false;
    }
    true
}

/// What the key player paragraph came to.
enum KeyPlayers {
    /// a sentence in the weak-point paragraph, naming one or two players.
    Named,
    /// no key player: the "no obvious weakness" sentences overwrite the strength paragraph and the report has two
    /// paragraphs.
    NoWeakness,
    Nothing,
}

/// The key players of a team of band 2 and up into `para` (the first match wins): the injured best player; the link-up
/// between the first midfielder and striker; two strikers, two midfielders, two defenders; one striker by his flags and
/// ability; a winger, a creative or a combative midfielder; one defender; one midfielder. The players named go to
/// `ids` (`n` of them). Without any, the "no obvious weakness" sentences by the answers +1, +2, +0, +4 and +9 go to
/// `strength`.
unsafe fn opposition_key_players(r: &OppositionReport, para: usize, strength: usize, ids: *mut i32, n: *mut i8) -> KeyPlayers {
    let (mut a, mut b) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (ap, bp) = (a.as_ptr() as usize, b.as_ptr() as usize);
    let (own, t) = (r.own, r.target);
    let (go, gt) = (OppositionReport::gender(own), OppositionReport::gender(t));
    let (own_name, target_name) = (OppositionReport::short(own), OppositionReport::short(t));
    let first = r.first_flags();
    let (d0, d1) = (&r.defenders[0], &r.defenders[1]);
    let (m0, m1) = (&r.midfielders[0], &r.midfielders[1]);
    let (s0, s1) = (&r.strikers[0], &r.strikers[1]);
    let named = |list: &[i32]| {
        write_i8(n as usize, list.len() as i8);
        for (k, &id) in list.iter().enumerate() {
            write_i32(ids as usize + 4 * k, id);
        }
        KeyPlayers::Named
    };
    if r.count as u8 != 0 && first & 0x40000 != 0 {
        covered!("rxt_3 injured key player");
        report_player_name(r.players, &mut a, 1);
        let tpl = if first & 0x200 != 0 { 0xa777c8 } else if first & 0x400 != 0 { 0xa77764 } else if first & 0x100 != 0 { 0xa77704 } else { 0xa776a8 };
        story_lang!(para, tpl, ap);
        return named(&[read_i32(r.players + 4)]);
    }
    if r.n_str != 0 && r.n_mid != 0 && report_player_flags(m0) & 1 != 0 {
        covered!("rxt_3 link-up play");
        let (mut c, mut d) = ([0u8; 0xc8], [0u8; 0xc8]);
        report_player_name(m0.as_ptr() as usize, &mut a, 1);
        report_player_name(s0.as_ptr() as usize, &mut b, 1);
        report_player_name(m0.as_ptr() as usize, &mut c, 5);
        report_player_name(s0.as_ptr() as usize, &mut d, 5);
        let (mf, sf) = (report_player_flags(m0), report_player_flags(s0));
        if mf & 0x1000 != 0 && sf & 0x4000 != 0 {
            story_lang!(para, 0xa775e0, 0x64, go, go, bp, ap, own_name);
        } else if sf & 4 != 0 {
            story_lang!(para, 0xa774a0, 0x64, go, go, bp, ap, own_name, c.as_ptr(), d.as_ptr());
        } else {
            story_lang!(para, 0xa77410, ap, bp);
        }
        return named(&[report_player_id(m0), report_player_id(s0)]);
    }
    if r.n_str > 1 && first & 0x100 != 0 {
        report_player_name(s0.as_ptr() as usize, &mut a, 1);
        report_player_name(s1.as_ptr() as usize, &mut b, 1);
        story_lang!(para, 0xa7737c, ap, bp); // "Their strike force of <A> and <B>"
        return named(&[report_player_id(s0), report_player_id(s1)]);
    }
    if r.n_mid > 1 && first & 0x400 != 0 {
        report_player_name(m0.as_ptr() as usize, &mut a, 1);
        report_player_name(m1.as_ptr() as usize, &mut b, 1);
        let (f0, f1) = (report_player_flags(m0), report_player_flags(m1));
        if f0 & 0x1000 != 0 && f1 & 1 != 0 {
            story_lang!(para, if f0 & 0x2000 != 0 { 0xa772e0 } else { 0xa77244 }, bp, ap);
        } else if f1 & 0x1000 != 0 && f0 & 1 != 0 {
            if f1 & 0x2000 != 0 {
                story_lang!(para, 0xa772e0, bp, ap);
            } else {
                story_lang!(para, 0xa77244, ap, bp);
            }
        } else {
            story_lang!(para, 0xa771c0, ap, bp);
        }
        return named(&[report_player_id(m0), report_player_id(m1)]);
    }
    if r.n_def > 1 && first & 0x200 != 0 {
        report_player_name(d0.as_ptr() as usize, &mut a, 1);
        report_player_name(d1.as_ptr() as usize, &mut b, 1);
        story_lang!(para, 0xa7712c, gt, gt, ap, bp, target_name);
        return named(&[report_player_id(d0), report_player_id(d1)]);
    }
    if r.n_str != 0 && first & 0x100 != 0 {
        covered!("rxt_3 one striker");
        report_player_name(s0.as_ptr() as usize, &mut a, 1);
        lone_striker(s0, para, ap, (go, own_name), (gt, target_name));
        return named(&[report_player_id(s0)]);
    }
    if r.n_mid != 0 {
        let f0 = report_player_flags(m0);
        if f0 & 0x1000 != 0 && first & 0x400 != 0 {
            report_player_name(m0.as_ptr() as usize, &mut a, 1);
            if r.n_mid == 1 {
                OPPOSITION_REPORT_UNSET.store(true, Relaxed); // the second midfielder's flags are not written
            }
            story_lang!(para, if report_player_flags(m1) & 1 != 0 { 0xa76178 } else { 0xa760f8 }, ap); // "(Tricky) winger <A>"
            return named(&[report_player_id(m0)]);
        }
        if first & 0x400 != 0 && f0 & 3 != 0 {
            report_player_name(m0.as_ptr() as usize, &mut a, 1);
            story_lang!(para, if f0 & 1 != 0 { 0xa7607c } else { 0xa75fe8 }, ap); // creative / combative midfielder
            return named(&[report_player_id(m0)]);
        }
    }
    if r.n_def != 0 {
        report_player_name(d0.as_ptr() as usize, &mut a, 1);
        let f = report_player_flags(d0);
        let tpl = if f & 0x80 != 0 { 0xa75f5c } else if f & 0x80000 != 0 { 0xa75edc } else { 0xa75e70 };
        story_lang!(para, tpl, gt, gt, ap, target_name);
        return named(&[report_player_id(d0)]);
    }
    if r.n_mid != 0 {
        report_player_name(m0.as_ptr() as usize, &mut a, 1);
        story_lang!(para, 0xa75dec, gt, gt, ap, target_name); // "Midfielder <A> is a key player"
        return named(&[report_player_id(m0)]);
    }
    covered!("rxt_3 no weakness");
    let q = &r.answers;
    if q[1] != 0 && q[2] != 0 && q[0] != 0 {
        story_lang!(strength, 0xa75d48, gt, gt, target_name);
    } else if q[1] != 0 && q[2] == 0 && q[0] == 0 {
        story_lang!(strength, 0xa75cd0, gt, gt, target_name);
    } else if q[2] != 0 && q[0] != 0 && q[1] == 0 {
        story_lang!(strength, 0xa75bfc, gt, gt, go, go, target_name, own_name);
    } else if q[1] != 0 {
        story_lang!(strength, 0xa75b88, gt, gt, target_name);
    } else if q[4] != 0 {
        story_lang!(strength, 0xa75aec, gt, gt, target_name);
    } else if q[9] != 0 {
        story_lang!(strength, 0xa75a74, gt, gt, target_name);
    } else {
        return KeyPlayers::Nothing;
    }
    KeyPlayers::NoWeakness
}

/// One striker (`e`, his full name at `name`) by his flags and current ability (over 160, below 100, below 125, the
/// rest): ageing, new signing, young, then pace, power, crosses, form and skill; the manager's club (`own`) or the
/// scouted one (`target`) as the template names them.
unsafe fn lone_striker(e: &[u8], para: usize, name: usize, own: (i32, usize), target: (i32, usize)) {
    let f = report_player_flags(e);
    let ability = i16::from_le_bytes([e[0x18], e[0x19]]);
    let (go, own_name) = own;
    let (gt, target_name) = target;
    let with_own = |tpl: u32| story_lang!(para, tpl, go, go, name, own_name);
    let alone = |tpl: u32| story_lang!(para, tpl, name);
    let powerful = f & 0x20 != 0 && f & 2 != 0;
    if f & 0x10000 != 0 {
        alone(0xa770a4); // "Ageing striker"
    } else if f & 0x20000 != 0 {
        with_own(0xa7700c); // "New signing"
    } else if f & 0x8000 != 0 {
        alone(0xa76fb8); // "Young striker"
    } else if ability > 0xa0 {
        if f & 0x1000000 != 0 && f & 0x200000 != 0 {
            story_lang!(para, 0xa76ea4, go, go, gt, gt, name, target_name, own_name);
        } else if f & 0x100000 != 0 {
            with_own(0xa76dec);
        } else if f & 4 != 0 {
            with_own(0xa76d34);
        } else if powerful {
            with_own(0xa76c94);
        } else if f & 0x20 != 0 {
            with_own(0xa76bdc);
        } else {
            with_own(0xa76b48);
        }
    } else if ability < 0x64 {
        let reliable = f & 0x80000 != 0;
        if reliable && f & 0x1000000 != 0 && f & 0x200000 != 0 {
            alone(0xa76ae0); // "Normally reliable striker"
        } else if reliable && f & 0x800000 != 0 && f & 0x200000 != 0 {
            with_own(0xa76a2c); // "Consistent striker"
        } else if reliable && f & 0x800000 != 0 {
            with_own(0xa7699c); // "Reliable striker"
        } else if f & 0x400000 != 0 && f & 0x1000000 != 0 {
            with_own(0xa768ec);
        } else if f & 4 != 0 {
            with_own(0xa76854);
        } else if powerful {
            with_own(0xa767ac);
        } else if f & 0x20 != 0 {
            story_lang!(para, 0xa766d8, 0x64, gt, gt, name, target_name);
        } else {
            alone(0xa7666c);
        }
    } else if ability < 0x7d {
        if f & 0x80000 != 0 {
            alone(0xa765f8);
        } else if powerful {
            with_own(0xa76564);
        } else if f & 0x20 != 0 {
            with_own(0xa76494);
        } else {
            alone(0xa7641c);
        }
    } else if f & 1 != 0 {
        alone(0xa763ac);
    } else if powerful {
        with_own(0xa76300);
    } else if f & 0x20 != 0 {
        with_own(0xa76230);
    } else {
        alone(0xa761c0);
    }
}

/// Writes the scout's opposition report on a club into `out`: its formation and style, its strength, then its weak
/// point (a weak team) or its key players, which go to `ids` (`n` of them, for the links).
///
/// `player_search_write_opposition_report_text(out, own, target, band, answers, count, players, ids, n)`: `own` is the
/// manager's club, `target` the scouted one, `band` its squad strength band (`exe::club_squad_strength_band`, 0..6),
/// `answers` the thirteen bytes of the club scouting news item, `players` its `count` best players (0x1a bytes each).
/// Without `out`: the box of line 0x24b6. Without `own`, `target`, `players` or `ids`: "Unfortunately I couldn't find
/// their training ground..." The paragraphs (`opposition_formation`, `opposition_strength`, `opposition_weak_point`
/// below band 2, `opposition_key_players` from band 2) are joined by blank lines (0xa6a4e0, 0x991948, 0x991944).
/// Original: `player_search_write_opposition_report_text` (0x7be670, player_search.cpp, cdecl). Defects of the
/// original: a manager without a staff sheet gets a formation from an uninitialised buffer; the winger sentence reads
/// the second midfielder's flags with one midfielder; a weak-point paragraph without a strength one (a band below 0)
/// prints whatever langlib left in edx; the key-player checks test the first best player's position, not the pick's;
/// the "no obvious weakness" sentences overwrite the strength paragraph.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn write_opposition_report_text(out: usize, own: Club, target: Club, band: u32, answers: usize, count: u32,
                                                             players: usize, ids: *mut i32, n: *mut i8) {
    if out == 0 {
        show_error_at(0xa701b0, 0x24b6);
        return;
    }
    if own.is_null() || target.is_null() || players == 0 || ids.is_null() {
        story_crt!(out, 0xa759b4); // "Unfortunately I couldn't find their training ground..."
        return;
    }
    covered!("rxt_3 opposition report");
    write_i8(n as usize, 0);
    OPPOSITION_REPORT_UNSET.store(false, Relaxed);
    let mut r = OppositionReport { own, target, answers: [0; 0xd], players, count, defenders: [[0; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES],
        midfielders: [[0; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES], strikers: [[0; REPORT_PLAYER_SIZE]; REPORT_PICK_ENTRIES], n_def: 0, n_mid: 0, n_str: 0 };
    for (k, v) in r.answers.iter_mut().enumerate() {
        *v = read_u8(answers + k);
    }
    let (mut formation, mut strength, mut weak) = ([0u8; 0x400], [0u8; 0x400], [0u8; 0x400]);
    let (p1, p2, p3) = (formation.as_mut_ptr() as usize, strength.as_mut_ptr() as usize, weak.as_mut_ptr() as usize);
    opposition_formation(&r, p1);
    let band = band as i8;
    let strong = opposition_strength(&r, band, p2);
    r.n_def = collect_report_pick(FN_PLAYER_SEARCH_COLLECT_REPORT_DEFENDERS, count, players, target, &mut r.defenders);
    r.n_mid = collect_report_pick(FN_PLAYER_SEARCH_COLLECT_REPORT_MIDFIELDERS, count, players, target, &mut r.midfielders);
    r.n_str = collect_report_pick(FN_PLAYER_SEARCH_COLLECT_REPORT_STRIKERS, count, players, target, &mut r.strikers);
    let weak_written = if band < 2 {
        opposition_weak_point(&r, p3)
    } else {
        match opposition_key_players(&r, p3, p2, ids, n) {
            KeyPlayers::Named => true,
            KeyPlayers::NoWeakness => {
                story_crt!(out, 0x991948, p1, p2); // "%s\n\n%s"
                return;
            }
            KeyPlayers::Nothing => false,
        }
    };
    match (strong, weak_written) {
        (true, true) => story_crt!(out, 0xa6a4e0, p1, p2, p3), // "%s\n\n%s\n\n%s"
        (true, false) => story_crt!(out, 0x991948, p1, p2),
        (false, true) => {
            OPPOSITION_REPORT_UNSET.store(true, Relaxed); // the original prints whatever langlib left in edx
            story_crt!(out, 0x991944, p1);
        }
        (false, false) => story_crt!(out, 0x991944, p1), // "%s"
    }
}

/// Comparer for `write_opposition_report_text`: the text, the players named and the generator side by side; the text
/// only when the copy met no uninitialised value (`OPPOSITION_REPORT_UNSET`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_write_opposition_report_text(out: usize, own: Club, target: Club, band: u32, answers: usize, count: u32,
                                                                     players: usize, ids: *mut i32, n: *mut i8) {
    type F = unsafe extern "C" fn(usize, Club, Club, u32, usize, u32, usize, *mut i32, *mut i8);
    let o = original_fn::<F>(%%I0%%);
    if out == 0 || n.is_null() || ids.is_null() {
        return o(out, own, target, band, answers, count, players, ids, n);
    }
    let mut regions = vec![(out, 0x1000), (ids as usize, 8), (n as usize, 1)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(out, own, target, band, answers, count, players, ids, n),
        || write_opposition_report_text(out, own, target, band, answers, count, players, ids, n));
    let unset = OPPOSITION_REPORT_UNSET.load(Relaxed);
    let text = |s: &crate::comparison::MemorySnapshot| {
        let b = &s.0[0].1;
        String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned()
    };
    SCOREBOARD.record(same || (unset && mo.0[1..] == mr.0[1..]), || format!("write_opposition_report_text(band {}, count {}): original {:?} rust {:?} {:?}",
        band as i8, count as i32, text(&mo), text(&mr), first_differences(&mo, &mr)));
}
