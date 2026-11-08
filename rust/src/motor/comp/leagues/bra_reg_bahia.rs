use super::*;

/// Describes one state league.
///
/// It holds the vtable, the stages beyond the league (+0x2c: 1 in the round-robin ones with a knockout, 3 in the ones
/// with two rounds and a final; the vector [+0xc] = malloc of 4 × stages), the rules (thiscall, ret 0), the table
/// (thiscall, ret 0), the predicted positions (thiscall, ret 0), the source file and the error-box lines (rules, table,
/// fixtures, order, statistics).
///
/// [pt] Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
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
///
/// [pt] O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn bra_reg_bahia_construct_state_league(this: usize, ano: u32, registro: usize, e: &BraRegBahiaStateChampionship, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + league::YEAR) as *mut u16, ano as u16);
    *((this + league::DISCIPLINE_RULE) as *mut u8) = disciplina;
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch(ramo);
    *((this + league::IMPORTANCE) as *mut u8) = 0xf;
    *((this + league::STAGE_INDEX) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, -1);
    core::ptr::write_unaligned((this + league::STAGE_COUNT) as *mut u32, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    core::ptr::write_unaligned((this + league::STAGES) as *mut usize, v);
    if regras(this) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this;
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// Comparer for the state league constructors: runs only the original and checks return, vtable, year, record,
/// discipline and new-league fields.
///
/// [pt] O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn bra_reg_bahia_compare_state_league_ctor(name: &str, this: usize, year: u32, record: usize, e: &BraRegBahiaStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + league::YEAR) as *const u16) == year as u16
        && read_usize(this + 4) == record && *((this + league::DISCIPLINE_RULE) as *const u8) == 7;
    if *((this + league::STAGE_INDEX) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + league::STAGE_COUNT) == e.stages as usize && read_usize(this + league::STAGES) != 0 && *((this + league::IMPORTANCE) as *const u8) == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this, r, read_usize(this), read_usize(this + league::STAGE_COUNT), read_usize(this + league::STAGES), read_usize(this + 8)));
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
///
/// [pt] { 0x42f2d0 } (thiscall, ret 8, bra_reg_bahia.cpp): `BraRegBahia::BraRegBahia(ano, registro)`: o Baiano (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_bahia(this: usize, year: u32, record: usize) -> usize {
    bra_reg_bahia_construct_state_league(this, year, record, &BAHIA_CHAMPIONSHIP, 7, true, "cqt_1 baiano novo") // cqt1: the discipline and expected positions of the Baiano
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
///
/// [pt] A virada da temporada de um estadual (`start_new_season`, slot 3, +0x08; o mesmo molde nos nove estaduais de pontos
/// corridos com mata-mata: só mudam o fonte, as linhas das caixas e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o ano (+0x40, word) sobe um (`ano`; o contador `ramo`) e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras da temporada
/// (virtual +0x8c, set_up_rules), a tabela com os clubes do estadual (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c) e a renovação dos árbitros do estadual
/// (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
pub(crate) unsafe fn state_league_new_season(this: usize, table: usize, file: usize, rows: [u32; 4], year: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    core::ptr::write_unaligned((this + league::SEASON_CLOSED) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + league::ROUND_DATES);
    if l != 0 {
        exe::crt_free(l);
        core::ptr::write_unaligned((this + league::ROUND_DATES) as *mut usize, 0);
    }
    let stats = read_usize(this + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (read_usize(this + league::CURRENT_STAGE) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + league::STAGES) + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > read_usize(this + league::CURRENT_STAGE) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + league::YEAR) as *const i16);
    core::ptr::write_unaligned((this + league::YEAR) as *mut i16, a.wrapping_add(year));
    crate::comparison::branch(ramo);
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if rules(this) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if sort(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    predicted(this);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4))
}

/// The next stage of a state league (`advance_stage`, slot 11, +0x28).
///
/// At the last stage (+0x30 ≥ +0x2c − 1, signed), the error box and nothing else; otherwise +0x30 goes up by one and,
/// on reaching 0 (the end of the round-robin phase), builds the knockout (`mata_mata`, direct call with the league's
/// ecx -> thiscall, ret 0).
///
/// [pt] A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
pub(crate) unsafe fn state_league_advance_stage(this: usize, file: usize, row: u32, knockout_knockout: usize) {
    let stage = read_usize(this + league::CURRENT_STAGE) as i32;
    if stage >= (read_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, stage);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout_knockout);
        f(this);
    }
}

/// Comparer for the new season: runs only the original and checks that the year (+0x40) went up by one.
///
/// [pt] O comparador da virada: só a original (simplification: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
pub(crate) unsafe fn compare_state_league_new_season(name: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let before = core::ptr::read_unaligned((this + league::YEAR) as *const i16);
    let r = original();
    let after = core::ptr::read_unaligned((this + league::YEAR) as *const i16);
    SCOREBOARD.record(after == before.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", name, this, before, after));
    r
}

/// Comparer for advance_stage: runs only the original and checks +0x30 unchanged at the last stage, else + 1.
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
pub(crate) unsafe fn compare_state_league_advance_stage(name: &str, this: usize, original: impl FnOnce()) {
    let before = read_usize(this + league::CURRENT_STAGE) as i32;
    let last = before >= (read_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1);
    original();
    let after = read_usize(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("{}({:#x}) antes={} depois={}", name, this, before, after));
}

/// The Bahia turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegBahia::start_new_season` (0x42fe10, bra_reg_bahia.cpp, thiscall, ret 0).
///
/// [pt] { 0x42fe10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::start_new_season()`, a virada da temporada do Baiano (a tabela:
/// 0x42fd10; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x42fd10, 0x98acac, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_7 virada do baiano") // cqp7: the next year of the Baiano
}

/// The Bahia next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegBahia::advance_stage` (0x42ffc0, bra_reg_bahia.cpp, thiscall, ret 0).
///
/// [pt] { 0x42ffc0 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::advance_stage()`, a fase seguinte do Baiano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x430050); a caixa da linha 0x1f8.
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98acac, 0x1f8, 0x430050);
}

/// Is the club in the state league of key g (reserve division +0x60, which in Brazil holds the state league)?
///
/// [pt] O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
pub(crate) unsafe fn is_in_state_league(club: usize, g: usize) -> bool {
    let comp = read_usize(club + club::RESERVE_DIVISION);
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
///
/// [pt] O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
pub(crate) unsafe fn fill_state_league_table(this: usize, n: i16, file: usize, row: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + league::TABLE) as *mut usize, t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if is_in_state_league(club, g) {
            // simplification: the original pushes the row in eax with the high bits of the state league’s id; 0x687430 only reads the
            // byte. With girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            exe::league_init_table_row(this, l as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + league::CLUB_COUNT) {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(0x245);
    }
    if k as i8 as i16 != read_i16(this + league::CLUB_COUNT) {
        return 0;
    }
    crate::comparison::branch(ramo);
    1
}

/// the Bahia league has 10 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegBahia::create_table` (0x42fd10, bra_reg_bahia.cpp, thiscall, ret 0).
///
/// [pt] { 0x42fd10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_table()`: o Baiano tem 10 clubes (divisão reserva +0x60 = [0x9cf9c0]).
/// Fonte 0x98acac, linha 0x167.
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_create_table(this: usize) -> u32 {
    fill_state_league_table(this, 0xa, 0x98acac, 0x167, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 0, "cql_8 baiano cheio")
}

/// Club of row l of the league table (+0xb1, 0x3b-byte rows).
///
/// [pt] O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
pub(crate) unsafe fn bra_reg_bahia_table_row_club(league: usize, l: usize) -> usize {
    read_usize(read_usize(league + league::TABLE) + l * 0x3b)
}

/// Bahia: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed;
/// the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegBahia::create_semi_final_stage` (0x430050, bra_reg_bahia.cpp, thiscall, ret 0).
///
/// [pt] { 0x430050 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98acac, linhas 0x21b (sem lista) e 0x22f (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_create_semi_final_stage(this: usize) {
    create_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // cqn7: 1º × 4º, 2º × 3º Baiano
}

/// Bahia, Gaucho, North, Northeast, Pernambuco, Santa Catarina and Paulista: at the end of the round robin, the clubs
/// through to the final stage get the qualification record (round 0x82, weight 0x32) and the relegated the relegation
/// record; in the knockout, the champion gets the champion record and fate 5; the eliminated, the knocked-out record
/// weighted 30 to 50 by the round they fell in, and fate -2
/// Original: `BraRegBahia::update_staff_records_for_status` (0x430210, bra_reg_bahia.cpp, thiscall).
///
/// [pt] { 0x430210 } (thiscall, ret 0x18, bra_reg_bahia.cpp): `BraRegBahia::update_staff_records_for_status` (o slot 18 de
/// sete estaduais: Baiano, Gaúcho, Norte, Nordeste, Pernambucano, Catarinense e Paulista): na fase de pontos corridos, os
/// classificados (6) ganham o registro de classificação (rodada 0x82, peso 0x32) e os rebaixados (3) o de rebaixamento;
/// no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5; os eliminados (2 ou -1), o registro
/// de eliminação com o peso da rodada (0x4bf850: de 30 a 50 pela rodada do jogo) e o destino -2. Devolve 0.
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    baiano_staff_records(this, club, status, stage, game, true).0
}

