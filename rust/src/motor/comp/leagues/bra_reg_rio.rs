use super::*;


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct BraRegRioStateChampionship {
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
pub(crate) unsafe fn bra_reg_rio_construct_state_league(this: usize, ano: u32, registro: usize, e: &BraRegRioStateChampionship, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
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
pub(crate) unsafe fn bra_reg_rio_compare_state_league_ctor(nome: &str, this: usize, ano: u32, registro: usize, e: &BraRegRioStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + 0x2c) == e.fases as usize && read_usize(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_usize(this), read_usize(this + 0x2c), read_usize(this + 0xc), read_usize(this + 8)));
    r
}

pub(crate) const CARIOCA_CHAMPIONSHIP: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x968608,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x43a500,
    previstas: 0x43add0,
    arquivo: 0x98b014,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const CATARINENSE_STATE_CHAMPIONSHIP: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x9686bc,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x43bb70,
    previstas: 0x43c110,
    arquivo: 0x98b050,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const PAULISTA_STATE_LEAGUE: BraRegRioStateChampionship = BraRegRioStateChampionship {
    vtabela: 0x968770,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x43cc40,
    previstas: 0x43d140,
    arquivo: 0x98b090,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Rio state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegRio::BraRegRio` (0x439a40, bra_reg_rio.cpp, thiscall, ret 8).
///
/// [pt] { 0x439a40 } (thiscall, ret 8, bra_reg_rio.cpp): `BraRegRio::BraRegRio(ano, registro)`: o Carioca (três fases além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_rio(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_rio_construct_state_league(this, ano, registro, &CARIOCA_CHAMPIONSHIP, 7, true, "cqt_4 carioca novo") // cqt4: a disciplina e as posições previstas do Carioca
}

/// The Santa Catarina state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the
/// table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegSanta::BraRegSanta` (0x43b130, bra_reg_santa.cpp, thiscall, ret 8).
///
/// [pt] { 0x43b130 } (thiscall, ret 8, bra_reg_santa.cpp): `BraRegSanta::BraRegSanta(ano, registro)`: o Catarinense (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_santa(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_rio_construct_state_league(this, ano, registro, &CATARINENSE_STATE_CHAMPIONSHIP, 7, true, "cqt_4 catarinense novo") // cqt4: a disciplina e as posições previstas do Catarinense
}

/// The Sao Paulo state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegSp::BraRegSp` (0x43c280, bra_reg_sp.cpp, thiscall, ret 8).
///
/// [pt] { 0x43c280 } (thiscall, ret 8, bra_reg_sp.cpp): `BraRegSp::BraRegSp(ano, registro)`: o Paulista (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_sp(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_rio_construct_state_league(this, ano, registro, &PAULISTA_STATE_LEAGUE, 7, true, "cqt_4 paulista novo") // cqt4: a disciplina e as posições previstas do Paulista
}

/// Comparer for `construct_bra_reg_rio` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_rio(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_RIO);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_rio", this, ano, registro, &CARIOCA_CHAMPIONSHIP, || o(this, ano, registro))
}

/// Comparer for `construct_bra_reg_santa` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_santa(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_SANTA);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_santa", this, ano, registro, &CATARINENSE_STATE_CHAMPIONSHIP, || o(this, ano, registro))
}

/// Comparer for `construct_bra_reg_sp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_sp(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_SP);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_sp", this, ano, registro, &PAULISTA_STATE_LEAGUE, || o(this, ano, registro))
}




/// Competition reputation of a league (word +0x69 of the competition record at +4).
///
/// [pt] A reputação da competição de uma liga (+0x69 do registro +4, word).
pub(crate) unsafe fn league_comp_reputation(liga: usize) -> i32 {
    read_i16(read_usize(liga + 4) + 0x69) as i32
}

/// Raises club attendance (+0x73) to a floor rising with reputation r and factor f, and the record (+0x7b) with it.
///
/// [pt] O piso que sobe com a reputação `r` (dos ramos que puxam o público para cima): com r < 5, 250 × r × f; senão
/// 750 × (r − 4) × f + 1000 (a original compara com a parte inteira + 1000 e grava a conta com 1000 somado antes do
/// truncamento). Abaixo dele, o público vai a ele e o recorde (+0x7b) sobe até ele.
pub(crate) unsafe fn raise_attendance_floor(c: usize, r: i32, f: f64) {
    let lim = if r < 5 {
        let lim = match_pl_ftol((r.wrapping_mul(250)) as f64 * f);
        if read_i32(c + 0x73) >= lim {
            return;
        }
        lim
    } else {
        let x = (r.wrapping_sub(4).wrapping_mul(750)) as f64 * f;
        if read_i32(c + 0x73) >= match_pl_ftol(x).wrapping_add(1000) {
            return;
        }
        match_pl_ftol(x + 1000.0)
    };
    write_i32(c + 0x73, lim);
    if lim > read_i32(c + 0x7b) {
        write_i32(c + 0x7b, lim);
    }
}

/// Caps club attendance (+0x73) at lim, lowering the minimum (+0x77) to it.
///
/// [pt] O teto `lim` do público: acima dele, o público cai a ele e o mínimo (+0x77) desce até ele.
pub(crate) unsafe fn cap_attendance(c: usize, lim: i32) {
    if read_i32(c + 0x73) > lim {
        write_i32(c + 0x73, lim);
        if lim < read_i32(c + 0x77) {
            write_i32(c + 0x77, lim);
        }
    }
}

/// The clubs average attendance at the end of each league season: it moves with the final position against the
/// predicted one and with the competition reputation, with a weight by nation (0.1, 0.25 or 0.5 in some, 1 in the
/// others): the champion and runner-up without promotion gain (reputation + places above the prediction) × 200/100 or
/// 125/65 (stadium owner or not); promoted clubs gain by club reputation and the reputation gap to the league above,
/// with that league floor; relegated clubs lose by the gap to the league below, with its ceiling; promotion play-off
/// and mid-table clubs gain or lose by a draw, mixed with the season real attendance (1/5 of the average home crowd +
/// 4/5 of the average) and reputation floors and ceilings (2000, 3000 or 5000 times); at the end, at least 500 and
/// between the club minimum and record
/// Original: `atualizar_media_de_publico (sem nome no mapa)` (0x5d6280, game.cpp, cdecl).
///
/// [pt] { 0x5d6280 } (cdecl, game.cpp, sem nome no mapa): `atualizar_media_de_publico(liga)`, do laço das temporadas
/// (0x5d5ae0, em 0x5d5cfb: push liga; call; add esp, 4), no fim da temporada de cada liga da nação (a de cima e as de
/// baixo, pela +0x20). A média de público do clube (+0x73, Attendance) anda com o resultado da temporada, entre o
/// mínimo (+0x77) e o recorde (+0x7b).
/// O peso `f` vem da nação da competição (o primeiro dword de [+4]+0x5d): 0,1 em [0x9cf510], [0x9cf2dc], [0x9cf3e4],
/// [0x9cf224]; 0,25 em [0x9cf2c0], [0x9cf2fc], [0x9cf408], [0x9cf4a8], [0x9cf4f8]; 0,5 em [0x9cf45c]; 1 nas outras.
/// `r` = a reputação da competição (+0x69 do registro, word). A tabela final vem das estatísticas (Comp_Stats::
/// get_sorted_team_table 0x4a4b20, thiscall(+8, 1, &n, 1000), ret 0xc: registros de 0x76 bytes, +0 o id do clube,
/// +0x72 a posição prevista, +0x73 a final, +5 os jogos em casa [inferência], +6 o público somado [inferência]);
/// para cada clube (os ramos na ordem da original, com as mesmas chamadas às virtuais +0x98 "tem acesso" e +0x9c "tem
/// rebaixamento"; d = prevista − final + 1, word):
/// - campeão sem acesso: + (r + max(d, 0)) × 200 × f (dono do estádio, +0x6d) ou × 100 × f;
/// - 2º (ou posição 0) sem acesso: + (r + max(d, 0)) × 125 × f ou × 65 × f;
/// - subiu (posição ≤ +0xbe): pela liga de cima ([0xadadfc][+0x1c]; sem ela, nada): v = +0x80 do clube (a reputação,
///   word) × max(r de cima − r, 1) × f × 0,1, no mínimo 100; + v + sorteio(v) (dono) ou + v; e o piso pela reputação
///   da liga de cima;
/// - play-off de acesso (+0xbf ≠ 0 e posição ≤ +0xbf): com d − 1 > 0: + sorteio(r(d − 1) × 75 × f) + r(d − 1) × 75 × f
///   (dono) ou + sorteio(r(d − 1) × 25 × f) + r(d − 1) × 10 × f; senão + sorteio(r × 50 × f ou r × 25 × f); com
///   jogos em casa, a mistura com o público real: (público somado / jogos + 4 × média) × f × 0,2;
/// - caiu (posição > +0x3e − +0xc1): pela liga de baixo (sem ela, nada): v = +0x80 × min(r de baixo − r, −1) × f × 0,2,
///   no máximo −100 (sem rebaixamento na de baixo, +0x20 = −1: v = 2v − 250 − sorteio(750)); + v + sorteio(v); e o
///   teto de 2000 (r de baixo ≤ 7) ou 3000 vezes a r de baixo, × f;
/// - os outros: + sorteio(r × d × 5 × f), × 10 para o dono com d > 0 ou com +0x80 acima de 500 × r; a mistura com o
///   público real (sem o peso f); o piso pela reputação; e o teto de 2000 r (r ≤ 7), 3000 r (r ≤ 12) ou 5000 r.
/// No fim de cada um: no mínimo 500, e dentro de [+0x77, +0x7b]. A tabela é liberada (free 0x9452ca).
pub(crate) unsafe extern "C" fn cqt4_atualizar_media_de_publico(liga: usize) {
    let tabela_c = read_usize(G_COMP_OBJECT_TABLE);
    let cima = if read_i32(liga + 0x1c) == -1 { 0 } else { read_usize(tabela_c + 4 * read_usize(liga + 0x1c)) };
    let baixo = if read_i32(liga + 0x20) == -1 { 0 } else { read_usize(tabela_c + 4 * read_usize(liga + 0x20)) };
    let mut f = 1.0f64;
    let reg = read_usize(liga + 4);
    if reg != 0 && read_usize(reg + 0x5d) != 0 {
        let n = read_usize(read_usize(reg + 0x5d));
        let em = |gs: &[usize]| gs.iter().any(|&g| n == read_usize(g));
        if em(&[KEY_NATION_WALES, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_AUSTRALIA]) {
            f = f64::from_bits(0x3fb9_9999_9999_999a);
        } else if em(&[KEY_NATION_DENMARK, KEY_NATION_FINLAND, KEY_NATION_NORWAY, KEY_NATION_SWEDEN, KEY_NATION_UNITED_STATES]) {
            f = 0.25;
        } else if n == read_usize(KEY_NATION_SCOTLAND) {
            f = 0.5;
        }
    }
    let ordenada: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32) -> usize = core::mem::transmute(FN_COMP_STATS_GET_SORTED_TEAM_TABLE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let acesso = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(liga) + 0x98));
        v(liga)
    };
    let rebaixamento = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(liga) + 0x9c));
        v(liga)
    };
    let mut n = 0i32;
    let t = ordenada(read_usize(liga + 8), 1, &mut n, 1000);
    if t == 0 {
        return;
    }
    let mut i = 0i32;
    while i < n {
        let e = t + 0x76 * i as usize;
        let c = read_usize(G_CLUB_TABLE) + 0x245 * read_usize(e);
        let pos = *((e + 0x73) as *const u8);
        let prevista = *((e + 0x72) as *const u8);
        let dono = *((c + 0x6d) as *const u8) != 0;
        let somar = |v: i32| write_i32(c + 0x73, read_i32(c + 0x73).wrapping_add(v));
        let d = (prevista as i16).wrapping_sub(pos as i16).wrapping_add(1);
        if pos == 1 && acesso() == 0 {
            let k = league_comp_reputation(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(200) } else { k.wrapping_mul(100) };
            somar(match_pl_ftol(val as f64 * f));
        } else if acesso() == 0 && pos < 3 {
            let k = league_comp_reputation(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(125) } else { k.wrapping_mul(65) };
            somar(match_pl_ftol(val as f64 * f));
        } else if acesso() != 0 && pos as i32 <= *((liga + 0xbe) as *const i8) as i32 {
            if cima != 0 {
                let mut a = read_i16(read_usize(cima + 4) + 0x69).wrapping_sub(read_i16(read_usize(liga + 4) + 0x69));
                if a < 1 {
                    a = 1;
                }
                let v = match_pl_ftol(read_i16(c + 0x80) as f64 * a as f64 * f * 0.1).max(100);
                if dono {
                    let r = sorteio(v);
                    somar(r.wrapping_add(v));
                } else {
                    somar(v);
                }
                raise_attendance_floor(c, league_comp_reputation(cima), f);
                crate::comparacao::ramo("cqt_4 publico de quem subiu");
            }
        } else if *((liga + 0xbf) as *const i8) != 0 && pos as i32 <= *((liga + 0xbf) as *const i8) as i32 {
            let a = (prevista as i16).wrapping_sub(pos as i16);
            if a > 0 {
                let s = a as i32;
                if dono {
                    let v = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    let r = sorteio(v);
                    let w = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    somar(r.wrapping_add(w));
                } else {
                    let v = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(25) as f64 * f);
                    let r = sorteio(v);
                    let w = match_pl_ftol(league_comp_reputation(liga).wrapping_mul(s).wrapping_mul(10) as f64 * f);
                    somar(r.wrapping_add(w));
                }
            } else {
                let r = league_comp_reputation(liga);
                let val = if dono { r.wrapping_mul(50) } else { r.wrapping_mul(25) };
                let v = match_pl_ftol(val as f64 * f);
                somar(sorteio(v));
            }
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (read_i32(e + 6) / jogos as i32).wrapping_add(read_i32(c + 0x73).wrapping_mul(4));
                write_i32(c + 0x73, match_pl_ftol(val as f64 * f * 0.2));
            }
        } else if rebaixamento() != 0
            && pos as i32 > read_i16(liga + 0x3e) as i32 - *((liga + 0xc1) as *const i8) as i32
        {
            if baixo != 0 {
                let mut a = read_i16(read_usize(baixo + 4) + 0x69).wrapping_sub(read_i16(read_usize(liga + 4) + 0x69));
                if a > -1 {
                    a = -1;
                }
                let mut v = match_pl_ftol(read_i16(c + 0x80) as f64 * a as f64 * f * 0.2).min(-100);
                if read_i32(baixo + 0x20) == -1 {
                    let r = sorteio(0x2ee);
                    v = v.wrapping_mul(2).wrapping_sub(0xfa).wrapping_sub(r);
                }
                let r = sorteio(v);
                somar(r.wrapping_add(v));
                let rb = league_comp_reputation(baixo);
                let lim = if rb > 7 { match_pl_ftol(rb.wrapping_mul(3000) as f64 * f) } else { match_pl_ftol(rb.wrapping_mul(2000) as f64 * f) };
                cap_attendance(c, lim);
                crate::comparacao::ramo("cqt_4 publico de quem caiu");
            }
        } else {
            let r16 = read_i16(read_usize(liga + 4) + 0x69);
            let r = r16 as i32;
            let val = if d <= 0 && read_i16(c + 0x80) <= r16.wrapping_mul(500) {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            } else if dono {
                r.wrapping_mul(d as i32).wrapping_mul(10)
            } else {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            };
            let v = match_pl_ftol(val as f64 * f);
            somar(sorteio(v));
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (read_i32(e + 6) / jogos as i32).wrapping_add(read_i32(c + 0x73).wrapping_mul(4));
                write_i32(c + 0x73, match_pl_ftol(val as f64 * 0.2)); // cqt4: a mistura com o público real
            }
            crate::comparacao::ramo("cqt_4 publico do meio da tabela");
            raise_attendance_floor(c, league_comp_reputation(liga), f);
            let r = league_comp_reputation(liga);
            let teto = if r > 12 { r.wrapping_mul(5000) } else if r > 7 { r.wrapping_mul(3000) } else { r.wrapping_mul(2000) };
            cap_attendance(c, teto);
        }
        if read_i32(c + 0x73) < 500 {
            write_i32(c + 0x73, 500);
        }
        let a = read_i32(c + 0x73);
        if a < read_i32(c + 0x77) {
            write_i32(c + 0x73, read_i32(c + 0x77));
        } else if a > read_i32(c + 0x7b) {
            write_i32(c + 0x73, read_i32(c + 0x7b));
        }
        i += 1;
    }
    soltar(t);
}

/// Comparer for `cqt4_atualizar_media_de_publico` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da média de público: lado a lado (a cópia sem contar os sorteios, com o estado do sorteador
/// restaurado entre as duas) sobre a tabela dos clubes inteira (onde ficam +0x73, +0x77 e +0x7b) e o estado do
/// sorteador; a tabela final (0x4a4b20) é alocada e liberada nas duas.
pub(crate) unsafe extern "C" fn comparar_cqt4_atualizar_media_de_publico(liga: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_CQT4_ATUALIZAR_MEDIA_DE_PUBLICO);
    let mut regioes = vec![(read_usize(G_CLUB_TABLE), read_usize(G_CLUB_COUNT) * club::SIZE)];
    regioes.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(liga), || crate::aleatorio::sem_contar(|| cqt4_atualizar_media_de_publico(liga)));
    PLACAR.registrar(igual, || format!("cqt4_atualizar_media_de_publico({:#x}): o público dos clubes não bate", liga));
}




/// Creates half-season stage k of a two-half league: collects the table clubs, builds a League_Stage (0x68b100), stores
/// it in [+0xc][k], sets +0x30 = k and frees the vector.
///
/// [pt] O turno `k` de uma liga de dois turnos (o slot create_half_season_stage; o mesmo molde nos três estaduais e na J1,
/// conferido instrução a instrução: só mudam o fonte, a linha da caixa e o 20º argumento do construtor, 0xb nos
/// estaduais e 0xf na J1). Chamado do resultado de cada jogo (0x43af20 e as irmãs: push dl = +0x30 + 1; mov ecx ->
/// thiscall, ret 4; a original lê o argumento inteiro, usa o byte baixo e o empurra inteiro ao construtor).
/// 1) Os clubes da tabela da liga (+0xb1, linhas de 0x3b bytes, as +0x3e linhas) num vetor (malloc 0x944e46 de
///    4 × +0x3e; sem memória: a caixa e volta).
/// 2) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, 1, +4, 0, 0, os pontos por vitória +0xc2 (`pv`), por empate +0xc3, a
///    categoria dos jogos +0xc4, &+0xc5 (os critérios de desempate), &{1,0,0,0} (uma vaga de cima: o campeão do
///    turno), o ano +0x40 (word), k, o id 0x439 (primeiro turno) ou 0x43a (segundo), 0, 1, 0, 8, `vinte`, 0, 2; a
///    original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores
///    limpos; o k vai inteiro, como chegou). Sem memória para a fase: 0 na vaga, sem caixa.
/// 3) [+0xc][k] = a fase, +0x30 = k (o byte com sinal) e o vetor liberado (free 0x9452ca). O quadro SEH da original só
///    cobre o construtor (sem destrutor no caminho normal).
pub(crate) unsafe fn create_half_season_stage(this: usize, k: u32, arquivo: usize, linha: u32, vinte: u32, pv: usize, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, u32, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let vagas: [u8; 4] = [1, 0, 0, 0];
    let n = read_i16(this + 0x3e);
    let clubes = alocar(((n as i32) << 2) as u32);
    if clubes == 0 {
        return show_error_at(arquivo, linha);
    }
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        core::ptr::write_unaligned((clubes + 4 * i as usize) as *mut usize, read_usize(read_usize(this + 0xb1) + i as usize * 0x3b));
        i += 1;
    }
    let id = 0x439 + (k as u8 != 0) as u32;
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, read_i16(this + 0x3e) as u16 as u32, clubes, 1, read_usize(this + 4), 0, 0,
                  read_u8(this + pv) as u32,
                  read_u8(this + 0xc3) as u32, read_u8(this + 0xc4) as u32, this + 0xc5, vagas.as_ptr(),
                  read_i16(this + 0x40) as u16 as u32, k, id, 0, 1, 0, 8, vinte, 0, 2)
    } else {
        0
    };
    let k = k as u8 as i8 as i32;
    core::ptr::write_unaligned((read_usize(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
    crate::comparacao::ramo(ramo);
    soltar(clubes);
}

/// Comparer for a half-season stage: only the original runs; checks that slot k has the stage, +0x30 = k and the stage
/// has the league's clubs.
///
/// [pt] O comparador de um turno: só a original (ponytail: aloca o vetor e a fase, que monta a tabela do turno), com o plano:
/// depois, a vaga k tem a fase (o operator new não falha na carreira), +0x30 = k e a fase tem os +0x3e clubes da liga.
pub(crate) unsafe fn compare_half_season_stage(nome: &str, this: usize, k: u32, original: impl FnOnce()) {
    original();
    let kk = k as u8 as i8 as i32;
    let fase = read_usize(read_usize(this + 0xc) + 4 * kk as usize);
    let igual = fase != 0 && read_usize(this + 0x30) as i32 == kk && read_i16(fase + 0x3e) == read_i16(this + 0x3e);
    PLACAR.registrar(igual, || format!("{}({:#x}, {}) fase={:#x} +0x30={}", nome, this, kk, fase, read_usize(this + 0x30) as i32));
}

/// One half of the Rio state league: when the first game of a half is processed, it builds that half's round-robin
/// stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as
/// the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by
/// the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `BraRegRio::create_half_season_stage` (0x43afa0, bra_reg_rio.cpp, thiscall, ret 4).
///
/// [pt] { 0x43afa0 } (thiscall, ret 4, bra_reg_rio.cpp): `BraRegRio::create_half_season_stage(k)`, um turno do Carioca (os
/// doze clubes; a caixa da linha 0x3d5).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x98b014, 0x3d5, 0xb, 0xc2, "cqr_1 turno do carioca") // cqr1: os pontos por vitória no Carioca
}




/// Comparer for `bra_reg_rio_create_half_season_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BRA_REG_RIO_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("bra_reg_rio_create_half_season_stage", this, k, || o(this, k))
}





/// start_new_season of a two-half state league with final: frees table, stages and lists, archives stats, allocates 3
/// stage slots, bumps the year and rebuilds rules, table, fixtures, order and officials.
///
/// [pt] A virada da temporada de um estadual de dois turnos e final (`start_new_season`, slot 3, +0x08; o mesmo molde no
/// Carioca, no Mineiro e no Paranaense: só mudam o fonte e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o vetor das fases liberado (free) e trocado por um novo de três vagas (+0x2c = 3, malloc 0x944e46 de 0xc, sem
/// conferir nem zerar: os turnos e a final o preenchem); o ano (+0x40, word) sobe `ano` (o contador `ramo`) e
/// +0x30 = −1; então, cada um com a sua caixa (linhas 0x1d4, 0x1db, 0x1e1 e 0x1e8) se devolver 0: as regras da
/// temporada (virtual +0x8c), a tabela com os doze clubes (`tabela`, thiscall, ret 0), os jogos (League::create_fixtures
/// 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as
/// posições previstas (virtual +0x5c) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0,
/// thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com caixa, a cópia devolve 0 (a original devolve o eax da
/// caixa, que a virada do Brasil não lê).
pub(crate) unsafe fn start_new_season_two_halves(this: usize, tabela: usize, arquivo: usize, ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = read_usize(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        arquivar(estatisticas, 1);
    }
    if (read_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > read_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    liberar(read_usize(this + 0xc));
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, 3);
    let v = alocar(0xc);
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(arquivo, 0x1d4);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        show_error_at(arquivo, 0x1db);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if jogos(this) == 0 {
        show_error_at(arquivo, 0x1e1);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if ordenar(this, 0) == 0 {
        show_error_at(arquivo, 0x1e8);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4))
}

/// Comparer for the season change: only the original runs; checks year +1, +0x2c = 3 and +0x30 = −1.
///
/// [pt] O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um, +0x2c = 3 e +0x30 = −1.
pub(crate) unsafe fn compare_new_season(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let igual = depois == antes.wrapping_add(1) && read_usize(this + 0x2c) == 3 && read_usize(this + 0x30) as i32 == -1;
    PLACAR.registrar(igual, || format!("{}({:#x}) ano antes={} depois={} +0x2c={} +0x30={}", nome, this, antes, depois, read_usize(this + 0x2c), read_usize(this + 0x30) as i32));
    r
}

/// The Rio state league turnover: archives the season in the statistics, clears the table, the schedule and the stages
/// (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the rules,
/// the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the referee
/// renewal. The halves are only built at each one's first game (cqr_1)
/// Original: `BraRegRio::start_new_season` (0x43a6a0, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a6a0 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::start_new_season()`, a virada do Carioca (a tabela: 0x43a500).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_start_new_season(this: usize) -> u32 {
    start_new_season_two_halves(this, 0x43a500, 0x98b014, 1, "cqr_2 virada do carioca") // cqr2: o ano seguinte do Carioca
}



/// Comparer for `bra_reg_rio_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_START_NEW_SEASON);
    compare_new_season("bra_reg_rio_start_new_season", this, || o(this))
}






/// The Rio de Janeiro state league has two halves (stages 0 and 1, each a round-robin league) and, if needed, a final
/// (stage 2): the next stage moves the current stage on and, at the end of the second half, builds the final; past the
/// last stage it is an error (the box)
/// Original: `BraRegRio::advance_stage` (0x43a870, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a870 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::advance_stage()`, a fase seguinte do Carioca (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x43a900, call direto com o ecx da liga).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_advance_stage(this: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x98b014, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRAREGRIO_CREATE_FINAL_STAGE);
        final_(this); // cqo9: a final depois do segundo turno
    }
}

/// The Rio de Janeiro final: the winners of the first and second halves meet over two games [inference: home and away],
/// the first-half winner as seed 0; if the same club wins both halves, it is champion without a final (the season ends
/// with the two stages and it gets status 5, champion, in the three tables)
/// Original: `BraRegRio::create_final_stage` (0x43a900, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a900 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_final_stage(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let lista = alocar(8);
    if lista == 0 {
        return show_error_at(0x98b014, 0x226);
    }
    let a = half_champion(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = half_champion(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_9 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo9: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < read_i16(this + 0x3e) {
            let linha = read_usize(this + 0xb1) + i as usize * 0x3b;
            if read_usize(linha) == read_usize(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 2, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 8) as *mut usize, fase);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    if fase == 0 {
        return show_error_at(0x98b014, 0x250);
    }
    crate::comparacao::ramo("cqo_9 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// The end of the Rio de Janeiro season: with a final, the honours come from the final; without one, from the league,
/// and the champion gets 675,000 and the runner-up 250,000 in prize money (to the balance, with the prize money news).
/// Then, once the twelve state leagues are over, the champions go into the Champions Cup
/// Original: `BraRegRio::record_season_honours` (0x43ad20, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43ad20 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::record_season_honours()` (slot 13), o fim da temporada do Carioca.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 675.000 ao campeão (primeira linha da tabela +0xb1) e 250.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_record_season_honours(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_BRA_NAT_FIRST_FILL_CHAMPIONS_CUP_ENTRANTS);
    if read_usize(this + 0x30) == 2 {
        let f = read_usize(read_usize(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_9 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_RECORD_SEASON_HONOURS);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    for (linha, valor, posicao) in [(0usize, 675000i32, 1i32), (0x3b, 250000, 2)] {
        let c = read_usize(read_usize(this + 0xb1) + linha);
        premio(financas(read_usize(G_FINANCE_TABLE), c), valor);
        noticia(this, read_usize(read_usize(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_9 premios");
    campeoes()
}

/// Comparer for `bra_reg_rio_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_RIO_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("bra_reg_rio_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// Comparer for `bra_reg_rio_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_RIO_CREATE_FINAL_STAGE)(this);
    let igual = if a == b {
        let marca = |k: usize| *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..read_i16(this + 0x3e).max(0) as usize).all(|i| {
            let linha = read_usize(this + 0xb1) + i * 0x3b;
            read_usize(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        read_usize(this + 0x30) == 1 && read_usize(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = read_usize(read_usize(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = read_usize(fase + 0xa7);
            let (x, y) = (read_usize(l), read_usize(l + 6));
            read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("bra_reg_rio_create_final_stage({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, read_usize(this + 0x30), read_usize(this + 0x2c)));
}

/// Comparer for `bra_reg_rio_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_9 historico da final" e "cqo_9 premios") e é provada pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_RECORD_SEASON_HONOURS)(this)
}




/// Common table body of the state leagues: allocates n rows and fills them with the clubs of state global g in database
/// order (League::init_table_row); returns 1 if filled.
///
/// [pt] O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
pub(crate) unsafe fn bra_reg_rio_create_state_league_table(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if is_in_state_league(clube, g) {
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

/// the Rio league has 12 clubs: those with it as their reserve division in the database (the field that holds the state
/// or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegRio::create_table` (0x43a500, bra_reg_rio.cpp, thiscall, ret 0).
///
/// [pt] { 0x43a500 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_table()`: o Carioca tem 12 clubes (divisão reserva +0x60 = [0x9cf9dc]).
/// Fonte 0x98b014, linha 0x169.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_table(this: usize) -> u32 {
    bra_reg_rio_create_state_league_table(this, 0xc, 0x98b014, 0x169, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 0, "cqm_8 carioca cheio")
}

/// the Santa Catarina league has 10 clubs: those with it as their reserve division in the database (the field that
/// holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegSanta::create_table` (0x43bb70, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bb70 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_table()`: o Catarinense tem 10 clubes (divisão reserva +0x60 = [0x9cf9b0]).
/// Fonte 0x98b050, linha 0x167.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_table_43bb70(this: usize) -> u32 {
    bra_reg_rio_create_state_league_table(this, 0xa, 0x98b050, 0x167, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 0, "cqm_8 catarinense cheio")
}

/// the Sao Paulo league has 16 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegSp::create_table` (0x43cc40, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cc40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_table()`: o Paulista tem 16 clubes (divisão reserva +0x60 = [0x9cf9e4]).
/// Fonte 0x98b090, linha 0x159.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_table_43cc40(this: usize) -> u32 {
    bra_reg_rio_create_state_league_table(this, 0x10, 0x98b090, 0x159, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 0, "cqm_8 paulista cheio")
}

/// Comparer for state league tables: only the original runs; checks n, the return and the row clubs against the planned
/// clubs in database order.
///
/// [pt] O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn bra_reg_rio_compare_state_league_table(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if is_in_state_league(clube, g) {
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

/// Comparer for `bra_reg_rio_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table", this, 0xc, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, || o(this))
}

/// Comparer for `bra_reg_rio_create_table_43bb70` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table_43bb70(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_CREATE_TABLE_43BB70);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table_43bb70", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, || o(this))
}

/// Comparer for `bra_reg_rio_create_table_43cc40` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table_43cc40(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_CREATE_TABLE_43CC40);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table_43cc40", this, 0x10, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, || o(this))
}

/// Minas Gerais and Rio (the same routine in the two vtables): no top places in the overall table (the halves have
/// stages of their own, created by add_result 0x43af20 / 0x435130 [inference: they decide the title]); 2 relegated;
/// double round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no
/// league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to
/// the next stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the
/// main stage schedule
/// Original: `BraRegRio::set_up_rules` (0x43a600, bra_reg_rio.cpp, thiscall).
///
/// [pt] { 0x43a600 } (thiscall, bra_reg_rio.cpp): `BraRegRio::set_up_rules`, nas vtables do Mineiro (0x968284) e do
/// Carioca (0x968608); devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_set_up_rules(this: usize) -> u32 {
    carioca_rules(this);
    crate::comparacao::ramo("cqi_9 regras do carioca");
    create_main_schedule(this);
    1
}

/// Comparador: como o do Paranaense.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_SET_UP_RULES)(this), || {
        carioca_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_rio_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
