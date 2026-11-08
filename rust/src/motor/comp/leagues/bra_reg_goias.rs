use super::*;


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct BraRegGoiasStateChampionship {
    pub(crate) vtabela: usize,
    pub(crate) fases: u32,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    pub(crate) previstas: usize,
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// Shared constructor of the Brazilian state leagues: League::League, vtable, year, discipline; loads saved state or
/// sets up a new league (rules, table, fixtures, sort, Comp_Stats, predicted positions).
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
pub(crate) unsafe fn bra_reg_goias_construct_state_league(this: usize, ano: u32, registro: usize, e: &BraRegGoiasStateChampionship, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
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

/// Comparer for state league constructors: runs only the original and checks return, vtable, year, record, discipline
/// and, for a new league, stages, stage vector and importance.
///
/// [pt] O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn bra_reg_goias_compare_state_league_ctor(name: &str, this: usize, year: u32, record: usize, e: &BraRegGoiasStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == year as u16
        && read_usize(this + 4) == record && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + 0x2c) == e.fases as usize && read_usize(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this, r, read_usize(this), read_usize(this + 0x2c), read_usize(this + 0xc), read_usize(this + 8)));
    r
}

pub(crate) const GOIANO_STATE_CHAMPIONSHIP: BraRegGoiasStateChampionship = BraRegGoiasStateChampionship {
    vtabela: 0x9681d0,
    fases: 1,
    regras: 0x433640,
    tabela: 0x433540,
    previstas: 0x433b40,
    arquivo: 0x98ae90,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

pub(crate) const MINEIRO_STATE_LEAGUE: BraRegGoiasStateChampionship = BraRegGoiasStateChampionship {
    vtabela: 0x968284,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x434810,
    previstas: 0x434f80,
    arquivo: 0x98aed0,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const NORTH_CHAMPIONSHIP: BraRegGoiasStateChampionship = BraRegGoiasStateChampionship {
    vtabela: 0x968338,
    fases: 1,
    regras: 0x439430,
    tabela: 0x435d80,
    previstas: 0x436280,
    arquivo: 0x98af14,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Goias state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegGoias::BraRegGoias` (0x432b50, bra_reg_goias.cpp, thiscall, ret 8).
///
/// [pt] { 0x432b50 } (thiscall, ret 8, bra_reg_goias.cpp): `BraRegGoias::BraRegGoias(ano, registro)`: o Goiano (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_goias(this: usize, year: u32, record: usize) -> usize {
    bra_reg_goias_construct_state_league(this, year, record, &GOIANO_STATE_CHAMPIONSHIP, 7, true, "cqt_2 goiano novo") // cqt2: a disciplina e as posições previstas do Goiano
}


/// The North state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegNorth::BraRegNorth` (0x435340, bra_reg_north.cpp, thiscall, ret 8).
///
/// [pt] { 0x435340 } (thiscall, ret 8, bra_reg_north.cpp): `BraRegNorth::BraRegNorth(ano, registro)`: o Norte (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_north(this: usize, year: u32, record: usize) -> usize {
    bra_reg_goias_construct_state_league(this, year, record, &NORTH_CHAMPIONSHIP, 7, true, "cqt_2 norte novo") // cqt2: a disciplina e as posições previstas do Norte
}









/// The Goias turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegGoias::start_new_season` (0x4336f0, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x4336f0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::start_new_season()`, a virada da temporada do Goiano (a tabela:
/// 0x433540; caixas nas linhas 0x1c3, 0x1ca, 0x1d0 e 0x1d7).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x433540, 0x98ae90, [0x1c3, 0x1ca, 0x1d0, 0x1d7], 1, "cqp_8 virada do goiano") // cqp8: o ano seguinte do Goiano
}

/// The Goias next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the
/// quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box)
/// Original: `BraRegGoias::advance_stage` (0x4338a0, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x4338a0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::advance_stage()`, a fase seguinte do Goiano: no fim dos pontos corridos,
/// o mata-mata de 8 (0x433930); a caixa da linha 0x1f1.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98ae90, 0x1f1, 0x433930);
}

/// The North turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegNorth::start_new_season` (0x435e80, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x435e80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::start_new_season()`, a virada da temporada do Norte (a tabela:
/// 0x435d80; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_start_new_season_435e80(this: usize) -> u32 {
    state_league_new_season(this, 0x435d80, 0x98af14, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_8 virada do norte") // cqp8: o ano seguinte do Norte
}

/// The North next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNorth::advance_stage` (0x436030, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x436030 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::advance_stage()`, a fase seguinte do Norte: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4360c0); a caixa da linha 0x1f8.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_advance_stage_436030(this: usize) {
    state_league_advance_stage(this, 0x98af14, 0x1f8, 0x4360c0);
}












/// Common table body of the state leagues: allocates n rows and fills them with the clubs of state global g in database
/// order; returns 1 if filled.
///
/// [pt] O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
pub(crate) unsafe fn bra_reg_goias_create_state_league_table(this: usize, n: i16, file: usize, row: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
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

/// the Goias league has 14 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegGoias::create_table` (0x433540, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x433540 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_table()`: o Goiano tem 14 clubes (divisão reserva +0x60 = [0x9cf9b4]).
/// Fonte 0x98ae90, linha 0x160.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_create_table(this: usize) -> u32 {
    bra_reg_goias_create_state_league_table(this, 0xe, 0x98ae90, 0x160, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0, "cql_9 goiano cheio")
}


/// the North league has 10 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNorth::create_table` (0x435d80, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x435d80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_table()`: o Norte tem 10 clubes (divisão reserva +0x60 = [0x9cf994]).
/// Fonte 0x98af14, linha 0x167.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_create_table_435d80(this: usize) -> u32 {
    bra_reg_goias_create_state_league_table(this, 0xa, 0x98af14, 0x167, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0, "cql_9 norte cheio")
}

/// Comparer for state league tables: only the original runs; checks n, the return and the row clubs against the planned
/// clubs.
///
/// [pt] O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn bra_reg_goias_compare_state_league_table(name: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
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









/// Writes the Goiano league rules (what 0x433640 writes before the calendar): points, tiebreaks, 8 qualifiers,
/// substitutions.
///
/// [pt] As regras do Goiano (o que a 0x433640 grava antes do calendário).
pub(crate) unsafe fn goiano_rules(o: usize) {
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3); // cqj7: o primeiro desempate do Goiano
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xbe, 8); // os 8 classificados
    write_u16_at(o, 0xd9, 8);
    write_u16_at(o, 0x3c, 1);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc1, 4);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
}

/// Writes the Japanese J1 rules (0x663620): two rounds, points, tiebreaks, two relegated to J2.
///
/// [pt] As regras da J1 (0x663620).
pub(crate) unsafe fn j1_rules(o: usize) {
    write_u16_at(o, 0x3c, 2);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1); // cqj7: os pontos por empate da J1
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0x82);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora, nenhum
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbe, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 2); // dois caem para a J2
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u8_at(o, 0x52, 2);
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2 as *const u32)); // cai para a J2
    write_u16_at(o, 0xd9, 8);
    write_u8_at(o, 0x49, 5);
}

/// Writes the Japanese J2 rules (0x665140): four rounds, points, tiebreaks, two promoted to J1; leaves +0x52 and +0xd9
/// as League::League set them.
///
/// [pt] As regras da J2 (0x665140): não grava +0x52 nem a palavra +0xd9 (fica a de `League::League`).
pub(crate) unsafe fn j2_rules(o: usize) {
    write_u16_at(o, 0x3c, 4); // quatro turnos
    write_u8_at(o, 0xc2, 3); // cqj7: os pontos por vitória da J2
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0x82);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbe, 2); // dois sobem para a J1
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1 as *const u32)); // sobe para a J1
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 5);
}

/// Goias: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; 8 top places (rule word
/// 8: they lead to the next stage, the knockout [inference]); 4 relegated, no league above or below; 7 bench
/// substitutes, 3 substitutions; category 2 matches. Then the schedule
/// Original: `BraRegGoias::set_up_rules` (0x433640, bra_reg_goias.cpp, thiscall).
///
/// [pt] { 0x433640 } (thiscall, bra_reg_goias.cpp): `BraRegGoias::set_up_rules` (slot 36), as regras do Goiano e o
/// calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_set_up_rules(this: usize) -> u32 {
    goiano_rules(this);
    crate::comparacao::ramo("cqj_7 regras do goiano");
    create_main_schedule(this);
    1
}




/// J2: four round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; 2 go up to J1 (the
/// rule word stays the base one, 3: champion and promotion), no relegation, no league below; 5 bench substitutes, 3
/// substitutions. Then the schedule
/// Original: `JapJ2::set_up_rules` (0x665140, jap_j2.cpp, thiscall).
///
/// [pt] { 0x665140 } (thiscall, jap_j2.cpp): `JapJ2::set_up_rules`, as regras da J2 e o calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_set_up_rules_665140(this: usize) -> u32 {
    j2_rules(this);
    crate::comparacao::ramo("cqj_7 regras da j2");
    create_main_schedule(this);
    1
}


/// Goias: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th (the
/// seeds are the positions [inference: two legs])
/// Original: `BraRegGoias::create_knockout_stage` (0x433930, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x433930 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae90, linhas 0x214 (sem lista) e 0x22c (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_create_knockout_stage(this: usize) {
    create_knockout_from_table(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae90, 0x214, 0x22c, "cqn_9 goiano"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Goiano
}

