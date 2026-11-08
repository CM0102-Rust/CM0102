use super::*;


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct BraRegBahiaStateChampionship {
    pub(crate) vtabela: usize,
    pub(crate) fases: u32,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    pub(crate) previstas: usize,
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// Shared constructor of the Brazilian state leagues: League base, vtable, year, discipline; loads saved state or sets
/// up rules, table, fixtures, sorting, stats and predicted positions.
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
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        show_error_at(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        show_error_at(e.arquivo, e.linhas[4]);
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
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == year as u16
        && read_usize(this + 4) == record && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + 0x2c) == e.fases as usize && read_usize(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this, r, read_usize(this), read_usize(this + 0x2c), read_usize(this + 0xc), read_usize(this + 8)));
    r
}

pub(crate) const BAHIA_CHAMPIONSHIP: BraRegBahiaStateChampionship = BraRegBahiaStateChampionship {
    vtabela: 0x967fb4,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x42fd10,
    previstas: 0x430340,
    arquivo: 0x98acac,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const CENTRO_OESTE_STATE_CHAMPIONSHIP: BraRegBahiaStateChampionship = BraRegBahiaStateChampionship {
    vtabela: 0x968068,
    fases: 1,
    regras: 0x430ff0,
    tabela: 0x430ef0,
    previstas: 0x431610,
    arquivo: 0x98ae10,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

pub(crate) const GAUCHO_STATE_LEAGUE: BraRegBahiaStateChampionship = BraRegBahiaStateChampionship {
    vtabela: 0x96811c,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x4321d0,
    previstas: 0x432980,
    arquivo: 0x98ae50,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Bahia state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegBahia::BraRegBahia` (0x42f2d0, bra_reg_bahia.cpp, thiscall, ret 8).
///
/// [pt] { 0x42f2d0 } (thiscall, ret 8, bra_reg_bahia.cpp): `BraRegBahia::BraRegBahia(ano, registro)`: o Baiano (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_bahia(this: usize, year: u32, record: usize) -> usize {
    bra_reg_bahia_construct_state_league(this, year, record, &BAHIA_CHAMPIONSHIP, 7, true, "cqt_1 baiano novo") // cqt1: a disciplina e as posições previstas do Baiano
}







/// Shared start_new_season of state leagues: frees table/lists/stages, archives stats, bumps year, rebuilds rules,
/// table, fixtures and renews officials.
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
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + 0xba);
    if l != 0 {
        free(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let stats = read_usize(this + 8);
    if stats != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        arquivar(stats, 1);
    }
    if (read_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > read_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(year));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
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
    let games: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if games(this) == 0 {
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
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4))
}

/// Shared advance_stage of state leagues: error box at last stage, else +0x30 + 1 and builds the knockout on reaching
/// 0.
///
/// [pt] A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
pub(crate) unsafe fn state_league_advance_stage(this: usize, file: usize, row: u32, mata_mata: usize) {
    let stage = read_usize(this + 0x30) as i32;
    if stage >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, stage);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this);
    }
}

/// Comparer for the new season: runs only the original and checks that the year (+0x40) went up by one.
///
/// [pt] O comparador da virada: só a original (simplification: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
pub(crate) unsafe fn compare_state_league_new_season(name: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let before = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let after = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(after == before.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", name, this, before, after));
    r
}

/// Comparer for advance_stage: runs only the original and checks +0x30 unchanged at the last stage, else + 1.
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
pub(crate) unsafe fn compare_state_league_advance_stage(name: &str, this: usize, original: impl FnOnce()) {
    let before = read_usize(this + 0x30) as i32;
    let last = before >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let after = read_usize(this + 0x30) as i32;
    PLACAR.registrar(after == if last { before } else { before + 1 }, || format!("{}({:#x}) antes={} depois={}", name, this, before, after));
}

/// The Bahia turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegBahia::start_new_season` (0x42fe10, bra_reg_bahia.cpp, thiscall, ret 0).
///
/// [pt] { 0x42fe10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::start_new_season()`, a virada da temporada do Baiano (a tabela:
/// 0x42fd10; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x42fd10, 0x98acac, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_7 virada do baiano") // cqp7: o ano seguinte do Baiano
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
pub(crate) unsafe fn is_in_state_league(clube: usize, g: usize) -> bool {
    let comp = read_usize(clube + club::RESERVE_DIVISION);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// Shared table setup of state leagues: allocates n rows and inits them with the state's clubs in database order;
/// returns 1 when full.
///
/// [pt] O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
pub(crate) unsafe fn fill_state_league_table(this: usize, n: i16, file: usize, row: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if is_in_state_league(clube, g) {
            // simplification: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != read_i16(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
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



/// Comparer for the state league table: plans the clubs in database order, runs the original and checks count, return
/// and table rows.
///
/// [pt] O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn bra_reg_bahia_compare_state_league_table(name: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if is_in_state_league(clube, g) {
            plan.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && read_i16(this + 0x3e) == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
    }
    PLACAR.registrar(same, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", name, this, r, plan, found));
    r
}






/// Club of row l of the league table (+0xb1, 0x3b-byte rows).
///
/// [pt] O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
pub(crate) unsafe fn bra_reg_bahia_table_row_club(league: usize, l: usize) -> usize {
    read_usize(read_usize(league + 0xb1) + l * 0x3b)
}

/// Builds the knockout stage from given table rows: entrant list, round table (virtual +0x3c), Cup_Stage with seeds,
/// set_up_round, frees, +0x30 = 0.
///
/// [pt] O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe fn bra_reg_bahia_build_knockout_from_table(this: usize, rows: &[usize], sementes: &[u8], file: usize, row_without_list: u32, row_without_stage: u32, ramo: &'static str) {
    let n = rows.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let list = alocar((n * 4) as u32);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    for (k, &l) in rows.iter().enumerate() {
        core::ptr::write_unaligned((list + 4 * k) as *mut usize, bra_reg_bahia_table_row_club(this, l));
    }
    crate::comparacao::ramo(ramo);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 0, &mut rodadas, &mut extra, 0);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = new(0xb2);
    // simplification: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n as u32, list, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(read_usize(this + 0xc) as *mut usize, stage);
    let stage = read_usize(read_usize(this + 0xc));
    if stage == 0 {
        return show_error_at(file, row_without_stage);
    }
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    build(stage, 0);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(t);
    free(list);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// Bahia: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed;
/// the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegBahia::create_semi_final_stage` (0x430050, bra_reg_bahia.cpp, thiscall, ret 0).
///
/// [pt] { 0x430050 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98acac, linhas 0x21b (sem lista) e 0x22f (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_create_semi_final_stage(this: usize) {
    bra_reg_bahia_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // cqn7: 1º × 4º, 2º × 3º Baiano
}

/// North: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed;
/// the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegNorth::create_semi_final_stage` (0x4360c0, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x4360c0 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af14, linhas 0x21b (sem lista) e 0x22f (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_create_semi_final_stage_4360c0(this: usize) {
    bra_reg_bahia_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af14, 0x21b, 0x22f, "cqn_7 norte"); // cqn7: 1º × 4º, 2º × 3º Norte
}


/// Comparer for the knockout build: runs only the original and checks the stage exists with the planned clubs (as a
/// set), size and +0x30 = 0.
///
/// [pt] O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
pub(crate) unsafe fn bra_reg_bahia_compare_knockout_from_table(name: &str, this: usize, rows: &[usize], original: impl FnOnce()) {
    let mut plan: Vec<usize> = rows.iter().map(|&l| bra_reg_bahia_table_row_club(this, l)).collect();
    original();
    let stage = read_usize(read_usize(this + 0xc));
    if stage == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", name, this));
        return;
    }
    let list = read_usize(stage + 0xa7);
    let mut found: Vec<usize> = (0..rows.len()).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && read_i16(stage + 0x3e) as usize == rows.len() && read_usize(stage + 4) == read_usize(this + 4) && read_usize(this + 0x30) == 0;
    PLACAR.registrar(same, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", name, this, pl, stage, ac, read_i16(stage + 0x3e)));
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
pub(crate) unsafe extern "thiscall" fn bra_reg_bahia_update_staff_records_for_status(this: usize, clube: usize, status: u32, stage: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    baiano_staff_records(this, clube, status, stage, jogo, true).0
}

