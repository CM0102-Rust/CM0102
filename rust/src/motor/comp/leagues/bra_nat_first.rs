use super::*;

/// The play-off of a Brazilian Série (the same template in A and B, checked instruction by instruction: only the source
/// file and the box lines change): builds the 4-club list, creates the Cup_Stage in the first stage slot, sets up round
/// 0, frees the list and sets +0x30 = 0.
///
/// The list of 4 (malloc `exe::crt_malloc` of 0x10; out of memory: the box `linha_sem_lista` and return) with the clubs of the
/// table's `linhas`, in this order (the ties: 0 × 1, 2 × 3); the virtual +0x3c (create_round_table: thiscall(this,
/// stage 0, &rounds, &extra, 0), ret 0x10) builds the round table.
///
/// The new stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `construct_cup_stage`, thiscall, ret 0x3c: the league, 4, the
/// list, the rounds, +4, the table, the year +0x40, 0, 1, the extra, 0x14, 0, 0, 0 and no seed: the order is not by
/// seeds) goes to the first stage slot ([+0xc]) and the round table is freed (free `exe::crt_free`).
///
/// No stage: the box `linha_sem_fase`, the list freed and return. With it: Cup::set_up_round (`set_up_round`, thiscall(stage,
/// 0), ret 4), the list freed and +0x30 = 0. The original's SEH frame only covers the constructor (no destructor).
pub(crate) unsafe fn create_play_off_stage(this: League, rows: &[usize; 4], file: usize, row_without_list: u32, row_without_stage: u32, ramo: &'static str) {
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        write_usize(list + 4 * k, bra_reg_bahia_table_row_club(this, l));
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in dx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 4, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        show_error_at(file, row_without_stage);
        exe::crt_free(list);
        return;
    }
    covered!(ramo);
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(list);
    this.set_current_stage((0) as usize);
}

/// The next stage of a Série (`advance_stage`, slot 11, +0x28).
///
/// At the last stage (+0x30 ≥ +0x2c − 1, signed), the error box and nothing else; otherwise +0x30 goes up by one and,
/// on reaching 0 (the end of the round-robin phase), builds the play-off (`play_off`, direct call with the league's ecx
/// -> thiscall, ret 0).
pub(crate) unsafe fn advance_series_stage(this: League, file: usize, row: u32, play_off: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Series A has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0
/// and the play-off is built; past the last stage it is an error (the box)
/// Original: `BraNatFirst::advance_stage` (0x42a950, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_advance_stage(this: usize) {
    advance_series_stage(League(this), 0x98aa30, 0x264, 0x42a9e0);
}

/// The Series A play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds: the list
/// order) [inference: two-legged semi-finals and final] that decides the Brazilian champion
/// Original: `BraNatFirst::create_play_off_stage` (0x42a9e0, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_play_off_stage(this: usize) {
    create_play_off_stage(League(this), &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A
}

/// Comparer of the play-off creation: runs only the original routine and then checks the new stage against the plan.
///
/// Only the original runs (it allocates the list, the table and the stage and draws the round), with the plan made
/// beforehand: the clubs of the rows.
///
/// Afterwards, the stage in the first slot must exist, with +0x3e = 4, +4 = the league's +4 and the first 4 clubs of
/// its list (+0xa7, 6 bytes) equal to those of the plan (as a set: the round draw may reorder them), and +0x30 = 0.
pub(crate) unsafe fn compare_play_off_stage(name: &str, this: League, rows: &[usize; 4], original: impl FnOnce()) {
    let mut plan: Vec<usize> = rows.iter().map(|&l| bra_reg_bahia_table_row_club(this, l)).collect();
    original();
    let stage = Cup(read_usize(this.stages()));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("{}({:#x}) without stage", name, this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found: Vec<usize> = (0..4).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 4 && read_usize(stage.addr() + 4) == this.competition().addr() && this.current_stage() == 0;
    SCOREBOARD.record(same, || format!("{}({:#x}) plan={:x?} stage={:#x} list={:x?} +0x3e={}", name, this.addr(), pl, stage.addr(), ac, stage.team_count()));
}

/// Competition object whose id is in global `global` (table G_COMP_OBJECT_TABLE).
pub(crate) unsafe fn comp_by_global(global: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(global))
}

/// Calls the thiscall virtual at offset deslocamento of object c with no arguments → u32.
pub(crate) unsafe fn bra_nat_first_call_virtual_no_args(c: usize, offset: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + offset));
    f(c)
}

/// The common body of the season rollover of a Brazilian Série (the same as the state leagues' rollover, `cqp_7..9`):
/// frees and rebuilds everything; returns false after an error box.
///
/// The table is freed (League::free_table `0x687970`, thiscall(this, 0), ret 4); the list at `+0xba` is freed (free
/// `0x9452ca`) and zeroed; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`, thiscall(`+8`, 1),
/// ret 4); with the current stage ≥ 0, the existing stages 0..=`+0x30` are destroyed (virtual 0 with 1); the year
/// (`+0x40`, word) goes up by `ano` and `+0x30` = −1.
///
/// Then, each with its own box if it returns 0: the rules (virtual `+0x8c`), the table (`tabela`, thiscall, ret 0), the
/// fixtures (League::create_fixtures `0x6835c0`, thiscall, ret 0) and the table order (League::sort_table `0x6827d0`,
/// thiscall(this, 0), ret 4); finally the predicted positions (virtual `+0x5c`). Returns false after a box.
pub(crate) unsafe fn rebuild_season(this: League, table: usize, file: usize, rows: [u32; 4], year: i16) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    true
}

/// The common end of the season rollover: sets each club's board expectations (League::set_club_season_expectations
/// `set_club_season_expectations`, thiscall, ret 0) and renews the officials (OfficialsManager::renew_officials `renew_officials`,
/// thiscall([0xb74340], +4), ret 4), whose eax the rollover returns.
pub(crate) unsafe fn set_expectations_and_renew_officials(this: League) -> u32 {
    let expectations: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectations(this.addr());
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32  // cqq8: os árbitros
}

/// The season turnovers of the other fourteen competitions in Brazil, in the original routine's order.
///
/// Série B, Série C and the twelve state leagues (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste,
/// Paranaense, Pernambucano, Carioca, Catarinense and Paulista: the id globals 0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998,
/// 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4, 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4).
pub(crate) const BRAZIL_TURNOVER_COMPS: [usize; 14] = [KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP];

/// The Brazilian turnover, led by Series A: first the promotion and relegation chain; then the Brazilian Cup entrants,
/// the 16 best clubs without a national division into Series C and the swaps between the state leagues and their lower
/// divisions; the division tier of the A and B clubs is capped; Series A is rebuilt (next year, rules, 28 clubs,
/// fixtures); then Series B, Series C and the twelve state leagues turn over (Bahia, Centre-West, Rio Grande do Sul,
/// Goias, Minas Gerais, North, Northeast, Parana, Pernambuco, Rio, Santa Catarina and Sao Paulo), with the board
/// expectations and the referee renewal
/// Original: `BraNatFirst::start_new_season` (0x42a640, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed((0) as usize);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    cadeia(this.addr(), 1);
    for f in [0x42b760usize, 0x42b520, 0x42b130] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    }
    exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION), 1, 3, -3, 1);
    if !rebuild_season(this, 0x42a480, 0x98aa30, [0x223, 0x22a, 0x230, 0x237], 1) {
        return 0;
    }
    for (k, g) in BRAZIL_TURNOVER_COMPS.iter().enumerate() {
        if k == 13 {
            covered!("cqq_8 paulista rollover");
        }
        bra_nat_first_call_virtual_no_args(comp_by_global(*g), comp_vtable::START_NEW_SEASON); // cqq8: the rollover of each one
    }
    set_expectations_and_renew_officials(this)
}

/// Promotion and relegation in Brazil: records the final positions of Series A, Series B, Series C and the seven Series
/// C stages; then swaps the marked clubs between A and B, between B and C and between B and each C stage (who goes up
/// and down comes from the end-of-season marks, with no fixed number here)
/// Original: `BraNatFirst::process_promotion_relegation_chain` (0x42ae90, bra_nat_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b, c) = (KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION);
    bra_nat_first_call_virtual_no_args(comp_by_global(a), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(b), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(c), league_vtable::RECORD_FINAL_POSITIONS);
    for k in 0..7 {
        bra_nat_first_call_virtual_no_args(read_usize(read_usize(comp_by_global(c) + league::STAGES) + 4 * k), 0xa4);
    }
    covered!("cqq_8 promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, -1, -1); // cqq8: A e B
    let mut r = exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c), 1, 1, -1, -1); // cqq8: B e C
    for k in 0..7 {
        r = exe::league_promote_and_relegate(this, comp_by_global(b), read_usize(read_usize(comp_by_global(c) + league::STAGES) + 4 * k), 1, 1, -1, -1);
    }
    r
}

/// True when the club's competition pointer at campo (+0x57 national division) is set and its id equals global g's.
pub(crate) unsafe fn club_in_comp_global(club: usize, field: usize, g: usize) -> bool {
    let comp = read_usize(club + field);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// The common body of the Série A and Série B table setup (the same template as the state leagues): fills the table
/// with the division's clubs; returns 1 if it is full.
///
/// +0x3e = `n` clubs; the table (malloc `exe::crt_malloc` of n × 0x3b; out of memory: the error box of source `arquivo` at line
/// `linha` and returns 0); the clubs of [0xae23b4] (0x245 bytes, [0xae235c] of them) in the competition of global `g`,
/// in database order, each in the next row (League::init_table_row `init_table_row`: thiscall(league, row, club, 0, 0), ret
/// 0x10), until full.
///
/// `girar` is 0 (the original: row k gets the k-th club); it exists only for each division's mutations.
pub(crate) unsafe fn create_division_table(this: League, n: i16, file: usize, row: u32, field: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if club_in_comp_global(club, field, g) {
            // simplification: the original pushes the row in eax with the high bits of the competition id; 0x687430 only reads
            // the byte. With girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            exe::league_init_table_row(this, l as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!(ramo);
    1
}

/// Builds the Series A table: 28 clubs, those whose national division is Series A, in database order; when full, TV
/// pays 150,000 to each club (no prize fund).
///
/// `BraNatFirst::create_table()`: Series A has 28 clubs (those with national division +0x57 =
/// `[KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION]`); when full, TV pays 150,000 to each club (League::pay_tv_rights
/// `exe::league_pay_tv_rights`: push 0; push 0x249f0; mov ecx, edi -> thiscall(league, amount, filter), ret 8; it sends
/// the TV news) and returns 1. Source 0x98aa30, line 0x1a1.
/// Original: `BraNatFirst::create_table` (0x42a480, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_table(this: usize) -> u32 {
    if create_division_table(League(this), 0x1c, 0x98aa30, 0x1a1, 0x57, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, 0, "cql_7 serie a cheia") == 0 {
        return 0;
    }
    exe::league_pay_tv_rights(this, 0x249f0, 0);
    1
}

/// The clubs of global g's competition (pointer at campo) in database order, up to n.
pub(crate) unsafe fn division_clubs_plan(n: usize, field: usize, g: usize) -> Vec<usize> {
    let mut plan = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && plan.len() < n {
        let club = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if club_in_comp_global(club, field, g) {
            plan.push(club);
        }
        i += 1;
    }
    plan
}

/// Comparer of the Série A and Série B table creation: runs only the original routine, then checks +0x3e, the return
/// value and the table rows against the planned clubs.
///
/// The original allocates the table (and in Série A pays the TV money with a news item), so only it runs. The plan,
/// made beforehand: the division's clubs in database order (up to `n`).
///
/// Afterwards: +0x3e = `n`, the return value = 1 if the plan filled up (otherwise 0) and the table rows (+0 the club,
/// 0x3b bytes) hold the planned clubs, in order.
pub(crate) unsafe fn compare_division_table(name: &str, this: League, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let plan = division_clubs_plan(n as usize, 0x57, g);
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// Creates the main stage schedule via vtable slot 16 (create_schedule, stage -1, outputs +0xa9/+0x3a) and stores the
/// pointer in +0xba.
pub(crate) unsafe fn create_main_schedule(o: League) {
    let vt = read_usize(o.addr());
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(vt + 0x3c));
    let r = f(o.addr(), -1, o.addr() + league::ROUND_COUNT, o.addr() + league::STAGE_ID, 0);
    write_u32_at(o.addr(), 0xba, r as u32);
}

/// Memory regions of the league object (0xee bytes) minus the schedule outputs (+0x3a, +0xa9, +0xba), for the comparer.
pub(crate) fn league_regions_without_schedule(o: usize) -> [(usize, usize); 4] {
    let o = League(o);
    [(o.addr(), 0x3a), (o.addr() + league::LEGS, 0xa9 - 0x3c), (o.addr() + league::MATCH_RULES, 0xba - 0xab), (o.addr() + league::PROMOTED_COUNT, 0xee - 0xbe)]
}

/// Writes the Série A rule fields (what `bra_nat_first_set_up_rules` writes before the schedule).
pub(crate) unsafe fn set_serie_a_rules(o: usize) {
    write_u16_at(o, 0x3c, 1); // um turno
    write_u8_at(o, 0xc2, 3); // cqi7: the Serie A points per win
    write_u8_at(o, 0xc3, 1); // pontos por empate
    write_u8_at(o, 0xc4, 2); // match category: league
    write_u8_at(o, 0x42, 1); // national club
    write_u8_at(o, 0xc5, 3); // tiebreakers: head-to-head, goal difference, none
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, 4); // top places
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 4); // rebaixados
    write_u32_at(o, 0x1c, 0xffff_ffff); // nenhuma liga acima
    write_u8_at(o, 0x4a, 3); // substitutions
    write_u32_at(o, 0x20, read_u32(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION)); // relegated to Série B
    write_u16_at(o, 0xd9, 8); // bit 0x8: the top ones go to the next stage (no promotion, no champion band)
    write_u8_at(o, 0x52, 2); // fixed squad numbering
    write_u8_at(o, 0x49, 7); // substitutes on the bench
}

/// Writes the Série B rule fields (`bra_nat_second_set_up_rules`).
pub(crate) unsafe fn set_serie_b_rules(o: usize) {
    write_u16_at(o, 0x3c, 1);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1); // cqi7: the Serie B points per draw
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, 4); // quatro sobem
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 4); // quatro caem
    write_u32_at(o, 0x1c, read_u32(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION)); // goes up to Série A
    write_u16_at(o, 0xd9, 2); // bit 0x2: the top places are promotion places
    write_u8_at(o, 0x52, 2);
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, read_u32(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION)); // drops to Série C
    write_u8_at(o, 0x49, 7);
}

/// Writes the Série C rule fields (`bra_nat_third_set_up_rules`).
pub(crate) unsafe fn set_serie_c_rules(o: usize) {
    write_u16_at(o, 0x3c, 2); // cqi7: the Serie C round-robin legs
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals for, away goals
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xbe, 2);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 2);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0x52, 2);
    write_u32_at(o, 0x1c, read_u32(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION)); // promoted to Série B
    write_u32_at(o, 0x20, 0xffff_ffff); // nenhuma liga abaixo
    write_u16_at(o, 0xd9, 8);
    write_u8_at(o, 0x49, 7);
}

/// Series A: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no league above, drops
/// to Series B; 4 top places that, by the rule word 8, lead to the next stage (the play-offs; no promotion and no
/// champion band: the champion comes from the knockout) [inference: the next stage is the slot 11 one]; 4 relegated; 7
/// substitutes on the bench and 3 substitutions; fixed squad numbers; category 2 (league) matches. Then the main stage
/// schedule
/// Original: `BraNatFirst::set_up_rules` (0x42a5a0, bra_nat_first.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_set_up_rules(this: usize) -> u32 {
    set_serie_a_rules(this);
    covered!("cqi_7 serie a rules");
    create_main_schedule(League(this));
    1
}

/// Staff_Records add_qualified_for_round_record (club, comp, stage, round, weight).
pub(crate) unsafe fn bra_nat_first_add_qualified_record(club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    exe::staff_records_add_qualified_for_round_record(read_usize(G_STAFF_RECORDS), club, comp, stage, round, weight);
}

/// Staff_Records add_relegation_record (club, league).
pub(crate) unsafe fn bra_nat_first_add_relegation_record(club: usize, comp: usize) {
    exe::staff_records_add_relegation_record(read_usize(G_STAFF_RECORDS), club, comp);
}

/// Staff_Records add_league_champions_record (club, league).
pub(crate) unsafe fn add_champions_record(club: usize, comp: usize) {
    exe::staff_records_add_league_champions_record(read_usize(G_STAFF_RECORDS), club, comp);
}

/// Staff_Records add_promotion_record (club, league, weight).
pub(crate) unsafe fn bra_nat_first_add_promotion_record(club: usize, comp: usize, weight: u32) {
    exe::staff_records_add_promotion_record(read_usize(G_STAFF_RECORDS), club, comp, weight);
}

/// club.find_club_row(0): the club's row in the league table.
pub(crate) unsafe fn bra_nat_first_find_club_row(league: usize, club: usize) -> usize {
    exe::league_find_club_row(league, club, 0)
}

/// Série A update_staff_records_for_status (`bra_nat_first_update_staff_records_for_status`); fazer = false is the comparer's dry run → (return, row written,
/// destination).
pub(crate) unsafe fn serie_a_staff_records(this: usize, club: usize, status: u32, stage: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8);
    let comp = read_usize(this + 4);
    if fa == 0xff {
        // the round-robin stage
        if st == 6 {
            if fazer {
                covered!("cqj_9 serie a qualified");
                bra_nat_first_add_qualified_record(club, comp, 0x41c, 0, 0x28);
            }
        } else if st == 3 && fazer {
            bra_nat_first_add_relegation_record(club, comp);
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    // o mata-mata
    if st == 1 {
        let l = bra_nat_first_find_club_row(this, club);
        if fazer {
            covered!("cqj_9 serie a champion");
            add_champions_record(club, comp); // cqj9: the Serie A champion record
            write_u8(l + league_table_row::STATUS, 5);
        }
        return (0, l, 5);
    }
    if st == 2 || st == 0xff {
        let l = bra_nat_first_find_club_row(this, club);
        if fazer {
            write_u8(l + league_table_row::STATUS, 0xfe); // eliminado
        }
        return (0, l, 0xfe);
    }
    (0, 0, 0)
}

/// Série C update_staff_records_for_status (`bra_nat_third_update_staff_records_for_status`), like Série A's.
pub(crate) unsafe fn serie_c_staff_records(this: League, club: usize, status: u32, stage: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if fa < 7 {
        // the groups
        if st == 6 && fazer {
            bra_nat_first_add_qualified_record(club, comp, 0x14, 0, 0x28);
        }
        return (0, 0, 0);
    }
    if fa == 7 {
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            bra_nat_first_add_qualified_record(club, comp, 0, 0x1e, 0x28);
        }
        return (0, 0, 0);
    }
    if fa < 10 {
        // the final round-robin of four: the two promoted
        if st != 0 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            bra_nat_first_add_promotion_record(club, comp, 0x32);
            bra_nat_first_add_qualified_record(club, comp, 0, 0x96, 0x28);
        }
        // the club’s row in the league (k = -1) or in stages 0..6, the first that has the club, gets destination 0
        let mut k: i8 = -1;
        let mut found = 0;
        while k < 7 {
            let league = League(if k == -1 { this.addr() } else { read_usize(this.stages() + 4 * k as i32 as usize) });
            let n = league.club_count();
            if n > 0 {
                let t = league.table();
                let mut i: i8 = 0;
                while (i as i16) < n {
                    let r = t.wrapping_add(0x3b * i as i32 as usize);
                    if read_usize(r) == club {
                        found = r;
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if found != 0 {
                    break;
                }
            }
            k += 1;
        }
        if found != 0 && fazer {
            covered!("cqj_9 serie c promoted");
            write_u8(found + league_table_row::STATUS, 0); // cqj9: the destination of those promoted from Serie C
        }
        return (0, found, 0);
    }
    if fa == 10 {
        // a final
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            add_champions_record(club, comp);
        }
    }
    (0, 0, 0)
}

/// Baiano update_staff_records_for_status (`bra_reg_bahia_update_staff_records_for_status`), like Série A's but the club row comes from the table (+0xb1).
pub(crate) unsafe fn baiano_staff_records(this: League, club: usize, status: u32, stage: u32, game: usize, fazer: bool) -> (u32, usize, u8) {
    let game = Fixture(game);
    let (st, fa) = (status as u8, stage as u8);
    let comp = this.competition().addr();
    if fa == 0xff {
        if st == 6 {
            if fazer {
                bra_nat_first_add_qualified_record(club, comp, 0, 0x82, 0x32);
            }
        } else if st == 3 && fazer {
            covered!("cqj_9 baiano relegated");
            bra_nat_first_add_relegation_record(club, comp); // cqj9: the club relegated from the Baiano
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let n = this.club_count() as i32;
    let mut i = 0i32;
    while i < n {
        if read_usize(this.table() + league_table_row::SIZE * i as usize) == club {
            if st == 1 {
                if fazer {
                    add_champions_record(club, comp);
                }
                let l = this.table() + league_table_row::SIZE * i as usize;
                if fazer {
                    write_u8(l + league_table_row::STATUS, 5);
                }
                return (0, l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // simplification: the original pushes the whole registers (eax with the caller’s garbage above the
                    // word, ecx with the upper half of the table pointer); 0x4bf850 and 0x86c000 only read the
                    // words (0x4b0870 / `as u16`): the words go.
                    let weight = exe::comp_util_scale_by_round(0x1e, 0x32, game.round_number() as u16 as u32, 2) as u32;
                    exe::staff_records_add_knocked_out_record(read_usize(G_STAFF_RECORDS), club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
                }
                let l = this.table() + league_table_row::SIZE * i as usize;
                if fazer {
                    write_u8(l + league_table_row::STATUS, 0xfe);
                }
                return (0, l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0, 0)
}

/// Series A: at the end of the round robin, the clubs through to the knockout get the qualification record (stage
/// 0x41c, weight 0x28) and the relegated the relegation record; in the knockout, the champion gets the champion record
/// and fate 5 in the table, the eliminated fate -2
/// Original: `BraNatFirst::update_staff_records_for_status` (0x42abc0, bra_nat_first.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    serie_a_staff_records(this, club, status, stage, true).0
}

/// The schedule of the Brazilian Série A: stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`brazil_serie_a_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraNatFirst::create_schedule` (0x429b50, bra_nat_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), brazil_serie_a_schedule_stage(stage), rounds, extra, league, 0x98aa30)
}

/// Comparer for `bra_nat_first_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_NAT_FIRST_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_nat_first_create_schedule", stage, brazil_serie_a_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_first_create_schedule(this, stage, r, e, l))
}

/// The Brazilian first division at career creation: the year and the database competition, discipline rule 6
/// (`discipline_rule::BRAZIL_NATIONAL`) and, with no saved state, importance 10, a round robin and the knockout (one
/// stage besides the league), the rules (no league above, the second division below, eight relegated [inference:
/// maxevans60 map]), the table (the 28 national first division clubs [inference: map]), the fixtures, the order, the
/// statistics and the predicted positions by club strength (`bra_nat_first_construct_league` with
/// `BRA_NAT_FIRST_LEAGUE`).
/// Original: `BraNatFirst::BraNatFirst` (0x429850, bra_nat_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_nat_first(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &BRA_NAT_FIRST_LEAGUE, discipline_rule::BRAZIL_NATIONAL, "cqu_1 bra_nat_first new") // cqu1: the Brazilian first division
}

/// Comparer for `construct_bra_nat_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_NAT_FIRST);
    bra_nat_first_compare_league_ctor("construct_bra_nat_first", League(this), year, record, &BRA_NAT_FIRST_LEAGUE, discipline_rule::BRAZIL_NATIONAL, || o(this, year, record))
}

/// At the Brazilian season turnover, promotion and relegation between the twelve state leagues
/// (`BRA_STATE_LEAGUE_KEYS`) and their twelve lower divisions (`BRA_STATE_LOWER_DIVISION_KEYS`); as many clubs go up
/// from each lower division as its state league's `league::RELEGATED_COUNT` (signed byte).
/// 1) A list of 200 clubs (`exe::crt_malloc` of 0x320; out of memory: the error box of line 0x3d0 and return) with the
///    database clubs whose regional division (`club::RESERVE_DIVISION`) is one of the twelve lower ones, in database
///    order, up to 200.
/// 2) The list sorted by reputation and the first 75 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
///    25), cdecl: it draws).
/// 3) In list order, each club goes up to its division's state league while there are places
///    (`club::RESERVE_DIVISION` = the state league's record, its `league::COMPETITION`); if it has a national division
///    (`club::DIVISION`) and no manager (`club::MANAGER` = 0), it opens the job (`exe::manager_manager_add_job_vacancy`
///    ([`G_MANAGER_MANAGER`], club, 0), thiscall, ret 8).
/// 4) In each state league, the clubs of the last `league::RELEGATED_COUNT` table rows (`exe::league_get_table_row`)
///    go down to the lower division (`club::RESERVE_DIVISION` = its record) and leave the division
///    (`bra_nat_first_club_left_division`, with the state league's record).
/// 5) The list is freed (`exe::crt_free`).
/// Original: `bra_nat_first_exchange_state_league_clubs` (0x42b130, bra_nat_first.cpp, cdecl).
pub(crate) unsafe extern "C" fn bra_nat_first_exchange_state_league_clubs() {
    let leagues: [usize; 12] = core::array::from_fn(|i| comp_object_of_key(BRA_STATE_LEAGUE_KEYS[i]));
    let lower: [usize; 12] = core::array::from_fn(|i| comp_record_of_key(BRA_STATE_LOWER_DIVISION_KEYS[i]));
    let mut places: [i8; 12] = core::array::from_fn(|i| read_i8(leagues[i] + league::RELEGATED_COUNT));
    let list = exe::crt_malloc(0x320);
    if list == 0 {
        return show_error_at(0x98aa30, 0x3d0);
    }
    let mut n = 0i16;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 0xc8 {
        let c = club_at(i as isize).addr();
        let d = read_usize(c + club::RESERVE_DIVISION);
        if d != 0 && lower.contains(&d) {
            write_usize(list + 4 * n as usize, c);
            n += 1;
        }
        i += 1;
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 0x19);
    for k in 0..n.max(0) as usize { // cqv3: the list in order
        let c = read_usize(list + 4 * k);
        let d = read_usize(c + club::RESERVE_DIVISION);
        let e = match lower.iter().position(|&b| b == d) {
            Some(e) => e,
            None => continue, // ponytail: every club of the list belongs to one of the twelve
        };
        if places[e] > 0 { // cqv3: the state league's promotion places
            covered!("cqv_3 promotion to the state league");
            write_usize(c + club::RESERVE_DIVISION, read_usize(leagues[e] + league::COMPETITION));
            if read_usize(c + club::DIVISION) != 0 && read_usize(c + club::MANAGER) == 0 {
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, 0usize);
            }
            places[e] -= 1;
        }
    }
    for e in 0..12 {
        let l = League(leagues[e]);
        let total = l.club_count() as i32;
        let mut s = total - (l.relegated_count() as i8) as i32;
        while s < l.club_count() as i32 {
            let c = read_usize(exe::league_get_table_row(l, s));
            write_usize(c + club::RESERVE_DIVISION, lower[e]); // cqv3: the relegated club goes to the lower division
            covered!("cqv_3 relegated from the state league");
            bra_nat_first_club_left_division(c, l.competition().addr());
            s += 1;
        }
    }
    exe::crt_free(list);
}

/// The promotion comparer: runs only the original (it moves clubs between divisions, draws the shuffle and calls the
/// manager and contract code; it cannot run twice), with the plan made before: the clubs of each lower division in the
/// list (up to 200) and those of the last rows of each state league. Afterwards: exactly min(places, its clubs in the
/// list) clubs go up from each division (`club::RESERVE_DIVISION` = the state league's record) and its others stay;
/// those of the last rows are in the lower division.
pub(crate) unsafe extern "C" fn compare_bra_nat_first_exchange_state_league_clubs() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BRA_NAT_FIRST_EXCHANGE_STATE_LEAGUE_CLUBS);
    let leagues: [usize; 12] = core::array::from_fn(|i| comp_object_of_key(BRA_STATE_LEAGUE_KEYS[i]));
    let lower: [usize; 12] = core::array::from_fn(|i| comp_record_of_key(BRA_STATE_LOWER_DIVISION_KEYS[i]));
    let mut theirs: Vec<Vec<usize>> = vec![Vec::new(); 12];
    let mut n = 0;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 0xc8 {
        let c = club_at(i as isize).addr();
        if let Some(e) = lower.iter().position(|&b| b == read_usize(c + club::RESERVE_DIVISION) && b != 0) {
            theirs[e].push(c);
            n += 1;
        }
        i += 1;
    }
    let places: Vec<i32> = leagues.iter().map(|&l| read_i8(l + league::RELEGATED_COUNT) as i32).collect();
    let records: Vec<usize> = leagues.iter().map(|&l| read_usize(l + league::COMPETITION)).collect();
    let mut going_down: Vec<(usize, usize)> = Vec::new();
    for e in 0..12 {
        let l = leagues[e];
        let t = read_i16(l + league::CLUB_COUNT) as i32;
        for s in (t - places[e]).max(0)..t {
            going_down.push((read_usize(exe::league_get_table_row(l, s)), lower[e]));
        }
    }
    o();
    let mut errors = Vec::new();
    for e in 0..12 {
        let went_up = theirs[e].iter().filter(|&&c| read_usize(c + club::RESERVE_DIVISION) == records[e]).count();
        let stayed = theirs[e].iter().filter(|&&c| read_usize(c + club::RESERVE_DIVISION) == lower[e]).count();
        let expected = (places[e].max(0) as usize).min(theirs[e].len());
        if went_up != expected || went_up + stayed != theirs[e].len() {
            errors.push(format!("division {}: {} went up (expected {}), {} of {} stayed", e, went_up, expected, stayed, theirs[e].len()));
        }
    }
    for &(c, b) in &going_down {
        if read_usize(c + club::RESERVE_DIVISION) != b {
            errors.push(format!("relegated {:#x} in {:#x} (expected {:#x})", c, read_usize(c + club::RESERVE_DIVISION), b));
        }
    }
    SCOREBOARD.record(errors.is_empty(), || format!("bra_nat_first_exchange_state_league_clubs: {}", errors.join("; ")));
}

/// At the Brazilian season turnover, the third division renews itself.
/// 1) A list of 160 (`exe::crt_malloc` of 0x280; out of memory: the error box of line 0x43a and return) with the
///    Brazilian clubs without a national division (`bra_nat_first_clubs_without_division`). Fewer than 16: the list is
///    freed and nothing changes.
/// 2) The list sorted by reputation and the first 48 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
///    16), cdecl: it draws); the first 16 enter the third division (`club::DIVISION` = the third division's record) and
///    those with a regional division (`club::RESERVE_DIVISION`) and no manager (`club::MANAGER` = 0) open the job
///    (`exe::manager_manager_add_job_vacancy`([`G_MANAGER_MANAGER`], club, 0), thiscall, ret 8).
/// 3) In the third division's table and in those of its seven groups (stages 0 to 6 of `league::STAGES`), the clubs of
///    the rows marked `league_table_row::STATUS` = 3 (eliminated in the first stage [inference]) leave the national
///    division (`club::DIVISION` = 0) and leave the division (`bra_nat_first_club_left_division`, with the record of the
///    table they leave).
/// 4) The list is freed (`exe::crt_free`).
/// Original: `bra_nat_first_refresh_third_division_clubs` (0x42b520, bra_nat_first.cpp, cdecl).
pub(crate) unsafe extern "C" fn bra_nat_first_refresh_third_division_clubs() {
    let (t, record) = bra_nat_first_third_division();
    let list = exe::crt_malloc(0x280);
    if list == 0 {
        return show_error_at(0x98aa30, 0x43a);
    }
    let v = bra_nat_first_clubs_without_division();
    for (k, &c) in v.iter().enumerate() {
        write_usize(list + 4 * k, c);
    }
    if v.len() < 0x10 {
        return exe::crt_free(list);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list, v.len() as u32, 0x10);
    for k in 0..0x10 { // cqv3: the 16 entering the third division
        let c = read_usize(list + 4 * k);
        write_usize(c + club::DIVISION, record);
        if read_usize(c + club::RESERVE_DIVISION) != 0 && read_usize(c + club::MANAGER) == 0 {
            exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, 0usize);
        }
    }
    covered!("cqv_3 third division renewed");
    for g in -1i32..7 {
        let s = League(if g == -1 { t } else { read_usize(read_usize(t + league::STAGES) + 4 * g as usize) });
        let mut e = 0i32;
        while e < s.club_count() as i32 {
            let r = exe::league_get_table_row(s, e);
            if read_u8(r + league_table_row::STATUS) == 3 {
                let c = read_usize(r + league_table_row::CLUB);
                write_usize(c + club::DIVISION, 0); // cqv3: the eliminated club leaves the third division
                bra_nat_first_club_left_division(c, s.competition().addr());
            }
            e += 1;
        }
    }
    exe::crt_free(list);
}

/// The renewal comparer: runs only the original (it draws the shuffle and calls the manager and contract code; it
/// cannot run twice), with the plan made before: the clubs without a division (up to 160) and those of the rows
/// marked 3 in the eight tables. Afterwards, with 16 or more without a division: exactly 16 of them are in the third
/// division and the others without a division; the marked ones, without a division. With fewer than 16: nothing
/// changed for them.
pub(crate) unsafe extern "C" fn compare_bra_nat_first_refresh_third_division_clubs() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BRA_NAT_FIRST_REFRESH_THIRD_DIVISION_CLUBS);
    let (t, record) = bra_nat_first_third_division();
    let v = bra_nat_first_clubs_without_division();
    let mut marked = Vec::new();
    if v.len() >= 0x10 {
        for g in -1i32..7 {
            let s = League(if g == -1 { t } else { read_usize(read_usize(t + league::STAGES) + 4 * g as usize) });
            for e in 0..s.club_count() as i32 {
                let r = exe::league_get_table_row(s, e);
                if read_u8(r + league_table_row::STATUS) == 3 {
                    marked.push(read_usize(r + league_table_row::CLUB));
                }
            }
        }
    }
    o();
    let entered = v.iter().filter(|&&c| read_usize(c + club::DIVISION) == record).count();
    let outside = v.iter().filter(|&&c| read_usize(c + club::DIVISION) == 0).count();
    let expected = if v.len() >= 0x10 { 0x10 } else { 0 };
    let left = marked.iter().all(|&c| read_usize(c + club::DIVISION) == 0);
    SCOREBOARD.record(entered == expected && entered + outside == v.len() && left,
                     || format!("bra_nat_first_refresh_third_division_clubs: {} without a division, {} entered (expected {}), {} stayed out, {} marked, all out: {}", v.len(), entered, expected, outside, marked.len(), left));
}

/// The stages of the round table of the Brazilian Série A (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 27 rounds from 5 Jul to 2 Nov; stage 0, the knockout or final stage: 2 rounds from 3 Nov to 12 Nov; any other stage has no table (None).
pub(crate) fn brazil_serie_a_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 27, extra: 0, extra_check: false, size: 0x6db, row: 0xb6,
            ops: &BRAZIL_SERIE_A_SCHEDULE_LEAGUE_OPS, name: "cqw_4 serie a league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x15f,
            ops: &BRAZIL_SERIE_A_SCHEDULE_STAGE_0_OPS, name: "cqw_4 serie a stage 0" }),
        _ => None,
    }
}

/// The competition object (league) of key `key`: [[`G_COMP_OBJECT_TABLE`] + 4 × [key]].
pub(crate) unsafe fn comp_object_of_key(key: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(key))
}

/// What a club leaving a division goes through in the original (both relegations): if it cannot be managed
/// (`exe::club_can_be_managed`, cdecl, returns al = 0) and a human manager decides for it
/// (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0), cdecl), the manager leaves
/// (`exe::manager_manager_remove_club_manager`([`G_MANAGER_MANAGER`], club, 1, 7, 0), thiscall, ret 0x10); then the
/// relegation clauses of the contracts (`exe::contract_manager_trigger_relegation_clauses`([`G_CONTRACT_MANAGER`],
/// club, the record of the division it left), thiscall, ret 8).
pub(crate) unsafe fn bra_nat_first_club_left_division(club: usize, record: usize) {
    if exe::club_can_be_managed(club) == 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
    }
    exe::contract_manager_trigger_relegation_clauses(read_usize(G_CONTRACT_MANAGER), club, record);
}

/// The third division league object and its record.
pub(crate) unsafe fn bra_nat_first_third_division() -> (usize, usize) {
    (comp_object_of_key(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION), comp_record_of_key(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION))
}

/// The Brazilian clubs without a national division, in database order, up to 160: a club (outside the national teams:
/// the ids below [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]) with a nation (`club::NATION`), no division
/// (`club::DIVISION` = 0) and Brazil's nation (the nation id = [`KEY_NATION_BRAZIL`]).
pub(crate) unsafe fn bra_nat_first_clubs_without_division() -> Vec<usize> {
    let mut v = Vec::new();
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32 && v.len() < 0xa0 {
        let c = Club(club_at(i as isize).addr());
        let nation = c.nation().addr();
        if nation != 0 && c.division().addr() == 0 && read_usize(nation) == read_usize(KEY_NATION_BRAZIL) {
            v.push(c.addr());
        }
        i += 1;
    }
    v
}

/// The Brazilian Série A, the round-robin stage (−1): 27 rounds (5 Jul to 2 Nov), 1755 bytes, 88 operations.
pub(crate) const BRAZIL_SERIE_A_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 88] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 8, 7, 0, 2, 2, 0), Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 7, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0),
    Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0), Cqp11D(3, 0xf, 7, 0, 2, 2, 0), Cqp11L(3, 0, -1, -1, -1, 0),
    Cqp11D(4, 0x13, 7, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x1a, 7, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x1d, 7, 0, 2, 2, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 2, 8, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 5, 8, 0, 2, 2, 0),
    Cqp11L(8, 0, -1, -1, -1, 0), Cqp11D(9, 9, 8, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0x10, 8, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xb, 0, -1, -1, -1, 0), Cqp11D(0xc, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0),
    Cqp11L(0xc, 1, 5, 1, 1, 0), Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1e, 8, 0, 6, 1, 0),
    Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0),
    Cqp11D(0xe, 3, 9, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0xa, 9, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xe, 9, 0, 6, 1, 0),
    Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0), Cqp11L(0x11, 2, -1, -1, -1, 0),
    Cqp11D(0x12, 0x15, 9, 0, 6, 1, 0), Cqp11L(0x12, 0, 5, 1, 1, 0), Cqp11L(0x12, 1, 5, 1, 1, 0),
    Cqp11L(0x12, 2, -1, -1, -1, 0), Cqp11D(0x13, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0),
    Cqp11L(0x13, 1, 5, 1, 1, 0), Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(0x14, 0, 5, 1, 1, 0), Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0),
    Cqp11D(0x15, 7, 0xa, 0, 2, 2, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0xb, 0xa, 0, 6, 1, 0),
    Cqp11L(0x16, 0, 5, 1, 1, 0), Cqp11L(0x16, 1, 5, 1, 1, 0), Cqp11L(0x16, 2, -1, -1, -1, 0),
    Cqp11D(0x17, 0xe, 0xa, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0), Cqp11D(0x18, 0x12, 0xa, 0, 6, 1, 0),
    Cqp11L(0x18, 0, 5, 1, 1, 0), Cqp11L(0x18, 1, 5, 1, 1, 0), Cqp11L(0x18, 2, -1, -1, -1, 0),
    Cqp11D(0x19, 0x19, 0xa, 0, 6, 1, 0), Cqp11L(0x19, 0, 5, 1, 1, 0), Cqp11L(0x19, 1, 5, 1, 1, 0),
    Cqp11L(0x19, 2, -1, -1, -1, 0), Cqp11D(0x1a, 2, 0xb, 0, 6, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
];

/// The Brazilian Série A, stage 0: 2 rounds (3 Nov to 12 Nov), 208 bytes, 38 operations.
pub(crate) const BRAZIL_SERIE_A_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 38] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 5, 0xb, 0, 2, 2, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 8),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 6, 0xb, 0, 3), Cqp11J(1, 0xc, 0xb, 0, 2, 2, 0), Cqp11W(0x6f, 0x96),
    Cqp11W(0x71, 1), Cqp11W(0x73, 8), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 5), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x77, 1), Cqp11W(0x79, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 4), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0xf4240), Cqp11X(0xcc, 0),
];

pub(crate) const BRA_NAT_FIRST_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x967d98,
    importance: 0xa,
    importance_before_load: false,
    stages: 1,
    rules: 0x42a5a0,
    table: 0x42a480,
    file: 0x98aa30,
    rows: [0x56, 0x5d, 0x64, 0x6d, 0x74],
};

/// The keys of the twelve state leagues (the league object in [[`G_COMP_OBJECT_TABLE`] + key × 4]) and those of each
/// one's lower division (the record in [`G_CLUB_COMP_TABLE`] + key × 0x6b), in the original's order.
pub(crate) const BRA_STATE_LEAGUE_KEYS: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP,
];

pub(crate) const BRA_STATE_LOWER_DIVISION_KEYS: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_GOIAS_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_LOWER_DIVISION,
];

/// Fills the Brazilian Cup entrant book at the Brazilian turnover (`this` is not read). Left out: the champion and the
/// runner-up of Series A, the Brazilian Cup winner and the Champions Cup winner (`exe::comp_util_get_latest_placed_club`
/// of their competition records). The slots go from 70 down to 0, each step one slot lower:
/// 1. each state league (`BRAZILIAN_CUP_STATE_LEAGUES` order) sends the first club of its table by position
///    (`exe::league_get_table_row`) that is not left out; a league with none still uses up its slot;
/// 2. Series A sends its clubs from the third row of its table on (`league::TABLE`, rows of 0x3b bytes), skipping the
///    clubs left out and those already in a higher slot;
/// 3. then, row by row from the second to the tenth, each state league in turn sends the club of that row of its table,
///    if it has not yet sent its quota, the club is not left out nor already in.
/// It stops when the slots run out. Defect of the original: step 3 starts without checking the slot, so it would write
/// one club before the book if steps 1 and 2 had used up all the slots (not with the real leagues).
/// Original: `bra_nat_first_fill_brazilian_cup_entrants` (0x42b760, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_fill_brazilian_cup_entrants(_this: usize) {
    let cup = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CUP));
    let serie_a = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION));
    let leagues: [League; 12] = core::array::from_fn(|k| League(comp_by_global(BRAZILIAN_CUP_STATE_LEAGUES[k].0)));
    let champions_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP).wrapping_mul(0x6b));
    let left_out = [
        exe::comp_util_get_latest_placed_club(serie_a.competition(), 0) as u32,
        exe::comp_util_get_latest_placed_club(serie_a.competition(), 1) as u32,
        exe::comp_util_get_latest_placed_club(cup.competition(), 0) as u32,
        exe::comp_util_get_latest_placed_club(champions_cup, 0) as u32,
    ];
    let already_in = |club: u32, slot: i16| (slot + 1..=BRAZILIAN_CUP_LAST_SLOT).any(|s| read_u32(cup.entrant_book() + s as usize * 6) == club);
    let table_club = |league: League, row: i8| read_u32(league.table().wrapping_add((row as isize as usize).wrapping_mul(0x3b)));
    let mut slot = BRAZILIAN_CUP_LAST_SLOT;
    covered!("sbD_6 brazilian cup entrants");
    for &league in &leagues {
        if slot < 0 {
            break;
        }
        let champion = (0..league.club_count()).map(|r| read_u32(exe::league_get_table_row(league, r as i32))).find(|c| !left_out.contains(c));
        if let Some(club) = champion {
            write_entrant_slot(cup, slot, club);
        }
        slot -= 1;
    }
    let mut row: i8 = 2;
    if serie_a.club_count() > 2 {
        while slot >= 0 {
            let club = table_club(serie_a, row);
            if !already_in(club, slot) && !left_out.contains(&club) {
                write_entrant_slot(cup, slot, club);
                slot -= 1;
            }
            row = row.wrapping_add(1);
            if row as i16 >= serie_a.club_count() {
                break;
            }
        }
    }
    let mut sent = [0u8; 12];
    for row in 1..10i8 {
        for (k, &league) in leagues.iter().enumerate() {
            if sent[k] as i8 >= BRAZILIAN_CUP_STATE_LEAGUES[k].1 as i8 || row as i16 >= league.club_count() {
                continue;
            }
            let club = table_club(league, row);
            if left_out.contains(&club) || already_in(club, slot) {
                continue;
            }
            write_entrant_slot(cup, slot, club);
            slot -= 1;
            if slot < 0 {
                return;
            }
            sent[k] += 1;
        }
        if slot < 0 {
            return;
        }
    }
}

/// Puts the state champions into the Champions Cup entrant book once all twelve state leagues are complete
/// (`champions_cup_entrants`), then sets the cup up (`FN_BRA_CHAMP_CUP_SET_UP`: rank, table, fixtures; active = 1).
/// Defect of the original: a state league with no club of final position below 100 leaves its champion and ratio as
/// stack garbage.
/// Original: `bra_nat_first_fill_champions_cup_entrants` (0x42bba0, bra_nat_first.cpp, cdecl).
pub(crate) unsafe extern "C" fn bra_nat_first_fill_champions_cup_entrants() {
    let Some(entrants) = champions_cup_entrants() else {
        return;
    };
    covered!("sbD_6 champions cup entrants");
    let cup = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP));
    for (slot, club) in entrants {
        write_u32(cup.entrant_book() + slot * 6, club);
    }
    let set_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRA_CHAMP_CUP_SET_UP);
    set_up(cup.addr());
}

/// Comparer for `bra_nat_first_fill_brazilian_cup_entrants`: side by side over the 71 entrant slots of the Brazilian
/// Cup (the routine only writes there).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_first_fill_brazilian_cup_entrants(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_FILL_BRAZILIAN_CUP_ENTRANTS);
    let book = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CUP)).entrant_book();
    let reg = [(book, (BRAZILIAN_CUP_LAST_SLOT as usize + 1) * 6)];
    let (_, _, same, a, b) = crate::comparison::side_by_side(&reg, || o(this), || bra_nat_first_fill_brazilian_cup_entrants(this));
    SCOREBOARD.record(same, || format!("bra_nat_first_fill_brazilian_cup_entrants: entrant slots {:?} x {:?}", a, b));
}

/// Comparer for `bra_nat_first_fill_champions_cup_entrants`: only the original runs (it sets the cup up); the copy's
/// entrants, worked out before, must be the nine slots the original wrote (nothing when the copy expects nothing).
pub(crate) unsafe extern "C" fn compare_bra_nat_first_fill_champions_cup_entrants() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BRA_NAT_FIRST_FILL_CHAMPIONS_CUP_ENTRANTS);
    let plan = champions_cup_entrants();
    let book = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP)).entrant_book();
    let before: [u32; 9] = core::array::from_fn(|s| if book == 0 { 0 } else { read_u32(book + s * 6) });
    o();
    let after: [u32; 9] = core::array::from_fn(|s| if book == 0 { 0 } else { read_u32(book + s * 6) });
    let expected = match plan {
        Some(e) => {
            let mut v = before;
            for (s, c) in e {
                v[s] = c;
            }
            v
        }
        None => before,
    };
    SCOREBOARD.record(after == expected, || format!("bra_nat_first_fill_champions_cup_entrants: original {:x?} copy {:x?}", after, expected));
}

/// Writes `club` into entrant slot `slot` (6-byte entries) of a competition's entrant book; a negative slot writes
/// before the book, as the original's arithmetic does.
pub(crate) unsafe fn write_entrant_slot(comp: League, slot: i16, club: u32) {
    write_u32(comp.entrant_book().wrapping_add((slot as isize as usize).wrapping_mul(6)), club);
}

/// The nine Champions Cup entrants as (slot, club), in the order the original writes them, or None while a state
/// league is not complete (virtual `comp_vtable::CHECK_COMPLETE`(1, 0)) or the cup is already active
/// (`league::ACTIVE`). Slots 3 to 7: the champions of Rio, São Paulo, Minas Gerais, Rio Grande do Sul and Paraná;
/// slot 0: the better of the North and Northeast champions by points per game (the Northeast on a tie) if strictly
/// better than the Centre-West champion, otherwise the Centre-West one; slot 1: the Goiás champion if strictly better
/// than the Santa Catarina one, otherwise the latter; slots 8 and 2: the Bahia and Pernambuco champions, the strictly
/// better of the two in slot 8.
pub(crate) unsafe fn champions_cup_entrants() -> Option<[(usize, u32); 9]> {
    let leagues: [League; 12] = core::array::from_fn(|k| League(comp_by_global(CHAMPIONS_CUP_STATE_LEAGUES[k])));
    for league in leagues {
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(league.addr()) + comp_vtable::CHECK_COMPLETE));
        if complete(league.addr(), 1, 0) == 0 {
            return None;
        }
    }
    if League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP)).active() != 0 {
        return None;
    }
    let c: [(u32, f32); 12] = core::array::from_fn(|k| state_league_champion(leagues[k]));
    let (sao_paulo, rio, minas, parana, centre_west, pernambuco, bahia, santa_catarina, goias, north, northeast, gaucho) =
        (c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10], c[11]);
    let north_best = if north.1 > northeast.1 { north } else { northeast };
    let first = if north_best.1 > centre_west.1 { north_best.0 } else { centre_west.0 };
    let second = if goias.1 > santa_catarina.1 { goias.0 } else { santa_catarina.0 };
    let (better, other) = if bahia.1 > pernambuco.1 { (bahia.0, pernambuco.0) } else { (pernambuco.0, bahia.0) };
    Some([(3, rio.0), (4, sao_paulo.0), (5, minas.0), (6, gaucho.0), (7, parana.0), (0, first), (1, second), (8, better), (2, other)])
}

/// The champion of a state league and his points per game (`f32`, as the original stores them): the table row whose
/// club has the best final position (`comp_stats_club::CURRENT_POSITION` of its statistics entry,
/// `exe::comp_stats_find_club_entry` on the league's statistics) below 100, the first of equals. With no such row the
/// original leaves both unset (stack garbage); the copy gives (0, 0.0).
pub(crate) unsafe fn state_league_champion(league: League) -> (u32, f32) {
    let (mut best, mut champion) = (100u16, (0u32, 0f32));
    for r in 0..league.club_count().max(0) as usize {
        let row = league.table() + r * 0x3b;
        let entry = exe::comp_stats_find_club_entry(league.comp_stats(), read_usize(row));
        if entry != 0 && (read_u8(entry + comp_stats_club::CURRENT_POSITION) as u16) < best {
            best = read_u8(entry + comp_stats_club::CURRENT_POSITION) as u16;
            let points = read_i16(row + league_table_row::POINTS) as f64;
            let played = read_i8(row + league_table_row::PLAYED) as f64;
            champion = (read_u32(row), (points / played) as f32);
        }
    }
    champion
}

/// The twelve state leagues in the order the Brazilian Cup reads them, with how many clubs each may send besides its
/// champion: São Paulo, Rio, Minas Gerais, Paraná, Rio Grande do Sul, Goiás, Pernambuco and Bahia 5; Northeast 7;
/// Santa Catarina 5; Centre-West and North 7.
pub(crate) const BRAZILIAN_CUP_STATE_LEAGUES: [(usize, u8); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 5),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 5),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 7),
];

/// The last entrant slot of the Brazilian Cup: the slots are filled from 70 down to 0.
pub(crate) const BRAZILIAN_CUP_LAST_SLOT: i16 = 0x46;

/// BraChampCup's season set-up (bra_champ_cup.cpp, thiscall, ret 0): virtual `comp_vtable::SET_PREDICTED_POSITIONS`
/// (rank), the table, the fixtures and the order (each with its error box), then `league::ACTIVE` = 1.
pub(crate) const FN_BRA_CHAMP_CUP_SET_UP: usize = 0x427320;

/// The twelve state leagues in the order the Champions Cup reads their champions: São Paulo, Rio, Minas Gerais, Paraná,
/// Centre-West, Pernambuco, Bahia, Santa Catarina, Goiás, North, Northeast and Rio Grande do Sul.
pub(crate) const CHAMPIONS_CUP_STATE_LEAGUES: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP,
    KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS,
    KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS,
    KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP,
];
