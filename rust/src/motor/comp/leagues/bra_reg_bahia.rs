use super::*;

/// Describes one state league.
///
/// It holds the vtable, the stages beyond the league (+0x2c: 1 in the round-robin ones with a knockout, 3 in the ones
/// with two rounds and a final; the vector [+0xc] = malloc of 4 × stages), the rules (thiscall, ret 0), the table
/// (thiscall, ret 0), the predicted positions (thiscall, ret 0), the source file and the error-box lines (rules, table,
/// fixtures, order, statistics).
pub(crate) struct BraRegBahiaStateChampionship {
    pub(crate) vtabela: usize,
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) predicted: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The constructor of a Brazilian state league: League base, vtable, year, discipline; then either loads the saved
/// state or sets up rules, table, fixtures, order, statistics and predicted positions.
///
/// It is the same 544-byte template in all twelve, checked instruction by instruction: only the vtable, the stages, the
/// three own routines, the source file and the lines change.
///
/// League::League (`0x682200`, thiscall, ret 0); `+4` = the competition record; the vtable; the year (`+0x40`, word) =
/// `ano`; the discipline rule (`+0x50`) = 7. If the saved state loads (League::load_from_temp_file `0x687b10`,
/// thiscall(this, 1), ret 4; returns ≠ 0), returns this.
///
/// Otherwise (a new career): `+0x51` = 0xf (the importance), `+0x44` = 0xff, `+0x30` = −1, `+0x2c` = the stages and
/// their vector (malloc `0x944e46`, unchecked); then, each with its own box (and returning this) if it returns 0: the
/// rules, the table, the fixtures (League::create_fixtures `0x6835c0`, thiscall, ret 0) and the order
/// (League::sort_table `0x6827d0`, thiscall(this, 0), ret 4); the statistics (operator new `0x944cf1` of 0x5ce and
/// Comp_Stats::Comp_Stats `0x49ee70`, thiscall(new, this), ret 4; the original routine's SEH frame covers only this
/// `new`) into `+8` (without them: the box); finally the predicted positions.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn bra_reg_bahia_construct_state_league(this: League, year: u32, registro: usize, e: &BraRegBahiaStateChampionship, discipline: u8, predicted_yes: bool, ramo: &'static str) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), e.vtabela);
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(ramo);
    this.set_importance(0xf);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    this.set_stages(v);
    if regras(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    if predicted_yes {
        previstas(this.addr());
    }
    this.addr()
}

/// Comparer for the state league constructors: runs only the original and checks return, vtable, year, record,
/// discipline and new-league fields.
pub(crate) unsafe fn bra_reg_bahia_compare_state_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraRegBahiaStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == discipline_rule::BRAZIL_REGIONAL;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, importance {}, stage index {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), this.importance(), this.stage_index() as i32, r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

pub(crate) const BAHIA_CHAMPIONSHIP: BraRegBahiaStateChampionship = BraRegBahiaStateChampionship {
    vtabela: 0x967fb4,
    stages: 1,
    rules: 0x43bc70,
    table: 0x42fd10,
    predicted: 0x430340,
    file: 0x98acac,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const CENTRO_OESTE_STATE_CHAMPIONSHIP: BraRegBahiaStateChampionship = BraRegBahiaStateChampionship {
    vtabela: 0x968068,
    stages: 1,
    rules: 0x430ff0,
    table: 0x430ef0,
    predicted: 0x431610,
    file: 0x98ae10,
    rows: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

pub(crate) const GAUCHO_STATE_LEAGUE: BraRegBahiaStateChampionship = BraRegBahiaStateChampionship {
    vtabela: 0x96811c,
    stages: 1,
    rules: 0x4322d0,
    table: 0x4321d0,
    predicted: 0x432980,
    file: 0x98ae50,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Bahia state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegBahia::BraRegBahia` (0x42f2d0, bra_reg_bahia.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_bahia(this: usize, year: u32, record: usize) -> usize {
    bra_reg_bahia_construct_state_league(League(this), year, record, &BAHIA_CHAMPIONSHIP, 7, true, "cqt_1 baiano novo") // cqt1: the discipline and expected positions of the Baiano
}

/// Shared season turnover of the state leagues (`start_new_season`, slot 3, +0x08): frees the table, lists and stages,
/// archives the statistics, advances the year, rebuilds rules, table and fixtures and renews the officials.
///
/// The same template is used in the nine round-robin state leagues with a knockout; only the source file, the error-box
/// lines and the table routine change, checked instruction by instruction.
///
/// +0x4c = 0 and the table freed (League::free_table `free_table`, thiscall(this, 0), ret 4); the list at +0xba freed (free
/// `exe::crt_free`) and zeroed; with statistics (+8), Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(+8, 1), ret 4);
/// with the current stage ≥ 0, the existing stages 0..=+0x30 destroyed (virtual 0, the destructor with 1: frees); the
/// year (+0x40, word) goes up by one (`ano`; the counter `ramo`) and +0x30 = −1.
///
/// Then, each with its own error box if it returns 0: the season rules (virtual +0x8c, set_up_rules), the table with
/// the state league's clubs (`tabela`, thiscall, ret 0), the fixtures (League::create_fixtures `create_fixtures`, thiscall, ret
/// 0) and the table order (League::sort_table `sort_table`, thiscall(this, 0), ret 4).
///
/// Finally the predicted positions (virtual +0x5c) and the renewal of the state league's referees
/// (OfficialsManager::renew_officials `renew_officials`, thiscall([0xb74340], +4), ret 4), whose eax it returns.
pub(crate) unsafe fn state_league_new_season(this: League, table: usize, file: usize, rows: [u32; 4], year: i16, ramo: &'static str) -> u32 {
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
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
    covered!(ramo);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    if rules(this.addr()) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this.addr());
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32 
}

/// The next stage of a state league (`advance_stage`, slot 11, +0x28).
///
/// At the last stage (+0x30 ≥ +0x2c − 1, signed), the error box and nothing else; otherwise +0x30 goes up by one and,
/// on reaching 0 (the end of the round-robin phase), builds the knockout (`mata_mata`, direct call with the league's
/// ecx -> thiscall, ret 0).
pub(crate) unsafe fn state_league_advance_stage(this: League, file: usize, row: u32, knockout_knockout: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout_knockout);
        f(this.addr());
    }
}

/// Comparer for the new season: runs only the original and checks that the year (+0x40) went up by one.
pub(crate) unsafe fn compare_state_league_new_season(name: &str, this: League, original: impl FnOnce() -> u32) -> u32 {
    let before = this.year();
    let r = original();
    let after = this.year();
    SCOREBOARD.record(after == before.wrapping_add(1), || format!("{}({:#x}) year before={} after={}", name, this.addr(), before, after));
    r
}

/// Comparer for advance_stage: runs only the original and checks +0x30 unchanged at the last stage, else + 1.
pub(crate) unsafe fn compare_state_league_advance_stage(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("{}({:#x}) before={} after={}", name, this.addr(), before, after));
}

/// The Bahia turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegBahia::start_new_season` (0x42fe10, bra_reg_bahia.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x42fd10, 0x98acac, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_7 virada do baiano") // cqp7: the next year of the Baiano
}

/// The Bahia next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegBahia::advance_stage` (0x42ffc0, bra_reg_bahia.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98acac, 0x1f8, 0x430050);
}

/// Is the club in the state league of key g (reserve division +0x60, which in Brazil holds the state league)?
pub(crate) unsafe fn is_in_state_league(club: Club, g: usize) -> bool {
    let comp = club.reserve_division().addr();
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// The common table setup of the state leagues (the same template in all twelve, only the constants change): fills the
/// table with the state's clubs; returns 1 if it is full.
///
/// +0x3e = `n` clubs; the table (malloc `exe::crt_malloc` of n × 0x3b; out of memory: the error box of source `arquivo` at line
/// `linha` and returns 0); the clubs of [0xae23b4] (0x245 bytes, [0xae235c] of them) in the state league of global `g`,
/// in database order, each in the next row (League::init_table_row `init_table_row`: thiscall(league, row, club, 0, 0), ret
/// 0x10), until full.
///
/// `girar` is 0 (the original: row k gets the k-th club); it exists only for each state league's mutations.
pub(crate) unsafe fn fill_state_league_table(this: League, n: i16, file: usize, row: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
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
        if is_in_state_league(Club(club), g) {
            // simplification: the original pushes the row in eax with the high bits of the state league’s id; 0x687430 only reads the
            // byte. With girar = 0, (k + 0) % n = k.
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

/// the Bahia league has 10 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegBahia::create_table` (0x42fd10, bra_reg_bahia.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xa, 0x98acac, 0x167, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 0, "cql_8 baiano cheio")
}

/// Club of row l of the league table (+0xb1, 0x3b-byte rows).
pub(crate) unsafe fn bra_reg_bahia_table_row_club(league: League, l: usize) -> usize {
    read_usize(league.table() + l * league_table_row::SIZE)
}

/// Bahia: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed;
/// the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegBahia::create_semi_final_stage` (0x430050, bra_reg_bahia.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_create_semi_final_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // cqn7: 1º × 4º, 2º × 3º Baiano
}

/// Bahia, Gaucho, North, Northeast, Pernambuco, Santa Catarina and Paulista: at the end of the round robin, the clubs
/// through to the final stage get the qualification record (round 0x82, weight 0x32) and the relegated the relegation
/// record; in the knockout, the champion gets the champion record and fate 5; the eliminated, the knocked-out record
/// weighted 30 to 50 by the round they fell in, and fate -2
/// Original: `BraRegBahia::update_staff_records_for_status` (0x430210, bra_reg_bahia.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    baiano_staff_records(League(this), club, status, stage, game, true).0
}

