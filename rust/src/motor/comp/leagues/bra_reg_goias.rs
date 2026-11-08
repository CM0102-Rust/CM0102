use super::*;

// ---------- 283a leva: os construtores dos estaduais: Goiano, Mineiro e Norte (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct Cqt2Estadual {
    pub(crate) vtabela: usize,
    pub(crate) fases: u32,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    pub(crate) previstas: usize,
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn cqt2_construir(this: usize, ano: u32, registro: usize, e: &Cqt2Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
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
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn cqt2_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt2Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + 0x2c) == e.fases as usize && read_usize(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_usize(this), read_usize(this + 0x2c), read_usize(this + 0xc), read_usize(this + 8)));
    r
}

pub(crate) const CQT2_GOIANO: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x9681d0,
    fases: 1,
    regras: 0x433640,
    tabela: 0x433540,
    previstas: 0x433b40,
    arquivo: 0x98ae90,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

pub(crate) const CQT2_MINEIRO: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x968284,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x434810,
    previstas: 0x434f80,
    arquivo: 0x98aed0,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const CQT2_NORTE: Cqt2Estadual = Cqt2Estadual {
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
pub(crate) unsafe extern "thiscall" fn cqt2_construir_goiano(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_GOIANO, 7, true, "cqt_2 goiano novo") // cqt2: a disciplina e as posições previstas do Goiano
}


/// The North state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegNorth::BraRegNorth` (0x435340, bra_reg_north.cpp, thiscall, ret 8).
///
/// [pt] { 0x435340 } (thiscall, ret 8, bra_reg_north.cpp): `BraRegNorth::BraRegNorth(ano, registro)`: o Norte (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn cqt2_construir_norte(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_NORTE, 7, true, "cqt_2 norte novo") // cqt2: a disciplina e as posições previstas do Norte
}

pub(crate) unsafe extern "thiscall" fn comparar_cqt2_construir_goiano(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2049);
    cqt2_comparar("cqt2_construir_goiano", this, ano, registro, &CQT2_GOIANO, || o(this, ano, registro))
}


pub(crate) unsafe extern "thiscall" fn comparar_cqt2_construir_norte(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2051);
    cqt2_comparar("cqt2_construir_norte", this, ano, registro, &CQT2_NORTE, || o(this, ano, registro))
}

// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Goiano, Norte e Nordeste (bra_reg_goias.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp) ----------






/// The Goias turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegGoias::start_new_season` (0x4336f0, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x4336f0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::start_new_season()`, a virada da temporada do Goiano (a tabela:
/// 0x433540; caixas nas linhas 0x1c3, 0x1ca, 0x1d0 e 0x1d7).
pub(crate) unsafe extern "thiscall" fn cqp8_virada_do_goiano(this: usize) -> u32 {
    cqp7_virada(this, 0x433540, 0x98ae90, [0x1c3, 0x1ca, 0x1d0, 0x1d7], 1, "cqp_8 virada do goiano") // cqp8: o ano seguinte do Goiano
}

/// The Goias next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the
/// quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box)
/// Original: `BraRegGoias::advance_stage` (0x4338a0, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x4338a0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::advance_stage()`, a fase seguinte do Goiano: no fim dos pontos corridos,
/// o mata-mata de 8 (0x433930); a caixa da linha 0x1f1.
pub(crate) unsafe extern "thiscall" fn cqp8_fase_seguinte_do_goiano(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae90, 0x1f1, 0x433930);
}

/// The North turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegNorth::start_new_season` (0x435e80, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x435e80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::start_new_season()`, a virada da temporada do Norte (a tabela:
/// 0x435d80; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
pub(crate) unsafe extern "thiscall" fn cqp8_virada_do_norte(this: usize) -> u32 {
    cqp7_virada(this, 0x435d80, 0x98af14, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_8 virada do norte") // cqp8: o ano seguinte do Norte
}

/// The North next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNorth::advance_stage` (0x436030, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x436030 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::advance_stage()`, a fase seguinte do Norte: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4360c0); a caixa da linha 0x1f8.
pub(crate) unsafe extern "thiscall" fn cqp8_fase_seguinte_do_norte(this: usize) {
    cqp7_fase_seguinte(this, 0x98af14, 0x1f8, 0x4360c0);
}



pub(crate) unsafe extern "thiscall" fn comparar_cqp8_virada_do_goiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1932);
    cqp7_comparar_virada("cqp8_virada_do_goiano", this, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_goiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1933);
    cqp7_comparar_fase_seguinte("cqp8_fase_seguinte_do_goiano", this, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp8_virada_do_norte(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1934);
    cqp7_comparar_virada("cqp8_virada_do_norte", this, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_norte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1935);
    cqp7_comparar_fase_seguinte("cqp8_fase_seguinte_do_norte", this, || o(this))
}



// ---------- 277a leva: a tabela dos estaduais: Goiano, Mineiro e Norte (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------




/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
pub(crate) unsafe fn cql9_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if cql8_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
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
pub(crate) unsafe extern "thiscall" fn cql9_tabela_do_goiano(this: usize) -> u32 {
    cql9_tabela(this, 0xe, 0x98ae90, 0x160, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0, "cql_9 goiano cheio")
}


/// the North league has 10 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNorth::create_table` (0x435d80, bra_reg_north.cpp, thiscall, ret 0).
///
/// [pt] { 0x435d80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::create_table()`: o Norte tem 10 clubes (divisão reserva +0x60 = [0x9cf994]).
/// Fonte 0x98af14, linha 0x167.
pub(crate) unsafe extern "thiscall" fn cql9_tabela_do_norte(this: usize) -> u32 {
    cql9_tabela(this, 0xa, 0x98af14, 0x167, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0, "cql_9 norte cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn cql9_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * 0x245;
        if cql8_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_cql9_tabela_do_goiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1831);
    cql9_comparar("cql9_tabela_do_goiano", this, 0xe, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, || o(this))
}


pub(crate) unsafe extern "thiscall" fn comparar_cql9_tabela_do_norte(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1833);
    cql9_comparar("cql9_tabela_do_norte", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, || o(this))
}

// ---------- 274a leva: as regras do Goiano e das duas divisões da J-League (bra_reg_goias.cpp, jap_j1.cpp, jap_j2.cpp) ----------






/// As regras do Goiano (o que a 0x433640 grava antes do calendário).
pub(crate) unsafe fn cqj7_regras_goiano(o: usize) {
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

/// As regras da J1 (0x663620).
pub(crate) unsafe fn cqj7_regras_j1(o: usize) {
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

/// As regras da J2 (0x665140): não grava +0x52 nem a palavra +0xd9 (fica a de `League::League`).
pub(crate) unsafe fn cqj7_regras_j2(o: usize) {
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
pub(crate) unsafe extern "thiscall" fn cqj7_regras_do_goiano(this: usize) -> u32 {
    cqj7_regras_goiano(this);
    crate::comparacao::ramo("cqj_7 regras do goiano");
    cqi7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_cqj7_regras_do_goiano(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1721)(this), || {
        cqj7_regras_goiano(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_do_goiano({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}



/// J2: four round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; 2 go up to J1 (the
/// rule word stays the base one, 3: champion and promotion), no relegation, no league below; 5 bench substitutes, 3
/// substitutions. Then the schedule
/// Original: `JapJ2::set_up_rules` (0x665140, jap_j2.cpp, thiscall).
///
/// [pt] { 0x665140 } (thiscall, jap_j2.cpp): `JapJ2::set_up_rules`, as regras da J2 e o calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn cqj7_regras_da_j2(this: usize) -> u32 {
    cqj7_regras_j2(this);
    crate::comparacao::ramo("cqj_7 regras da j2");
    cqi7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
pub(crate) unsafe extern "thiscall" fn comparar_cqj7_regras_da_j2(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1723)(this), || {
        cqj7_regras_j2(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_da_j2({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Goias: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th (the
/// seeds are the positions [inference: two legs])
/// Original: `BraRegGoias::create_knockout_stage` (0x433930, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x433930 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae90, linhas 0x214 (sem lista) e 0x22c (sem fase).
pub(crate) unsafe extern "thiscall" fn cqn9_mata_mata_do_goiano(this: usize) {
    cqn9_mata_mata(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae90, 0x214, 0x22c, "cqn_9 goiano"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Goiano
}

pub(crate) unsafe extern "thiscall" fn comparar_cqn9_mata_mata_do_goiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1851);
    cqn9_comparar("cqn9_mata_mata_do_goiano", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}
