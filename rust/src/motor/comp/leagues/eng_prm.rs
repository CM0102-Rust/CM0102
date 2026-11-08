use super::*;



/// O que muda de uma divisão inglesa para outra no construtor.
pub(crate) struct EnglishDivision {
    /// a vtable da classe derivada
    pub(crate) vtabela: usize,
    /// a capacidade mínima do estádio (+0xe2) e os lugares sentados mínimos (+0xe4), quando a divisão os grava
    pub(crate) estadio: Option<(u16, u16)>,
    /// quantas fases além da liga (+0x2c): 1 = o play-off de acesso (o vetor [+0xc] de 1 fase, malloc de 4)
    pub(crate) fases: u32,
    /// set_up_rules e create_table da divisão (mov ecx, this -> thiscall, ret 0; devolvem 0 na falha)
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    /// o fonte e as linhas das cinco caixas (regras, tabela, jogos, ordem, estatísticas)
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// Body of the four English division constructors: League base, record, vtable, year, discipline rule 9, minimum
/// stadium; loads from the temp file or builds a new league (stages, rules, table, fixtures, stats, predicted
/// positions).
///
/// [pt] O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o ano (+0x40, palavra), o registro da competição (+4), a vtable,
/// a regra de disciplina 9 (+0x50) e o estádio mínimo; tenta carregar o estado do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova:
/// fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c fases (e o vetor [+0xc]), as regras, a tabela, os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0), a ordem da tabela (League::sort_table 0x6827d0:
/// push 0; mov ecx -> thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl;
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (League::set_predicted_positions 0x68a850: mov ecx -> thiscall, ret 0). Cada passo que falha mostra a caixa da
/// sua linha e encerra. Devolve this.
pub(crate) unsafe fn construct_english_division(this: usize, ano: u32, registro: usize, p: &EnglishDivision) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_PREDICTED_POSITIONS);
    base(this);
    write_u32_from_usize(this + 4, registro);
    write_u32_from_usize(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 9; // cql6: a regra de disciplina das ligas inglesas
    if let Some((capacidade, sentados)) = p.estadio {
        core::ptr::write_unaligned((this + 0xe2) as *mut u16, capacidade);
        core::ptr::write_unaligned((this + 0xe4) as *mut u16, sentados);
    }
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cql_6 divisao inglesa nova");
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    write_u32_from_usize(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(4);
        write_u32_from_usize(this + 0xc, v);
    } else {
        write_u32_from_usize(this + 0xc, 0);
    }
    if regras(this) == 0 {
        show_error_at(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    write_u32_from_usize(this + 8, s);
    if s == 0 {
        show_error_at(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    this
}

pub(crate) const PREMIER_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969d1c,
    estadio: Some((0x2710, 0x2710)),
    fases: 0,
    regras: 0x574b70,
    tabela: 0x574a20,
    arquivo: 0x9c43b0,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
pub(crate) const FIRST_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969c68,
    estadio: None,
    fases: 1, // cql6: o play-off da First Division
    regras: 0x572d90,
    tabela: 0x572bf0,
    arquivo: 0x9c4374,
    linhas: [0x49, 0x50, 0x57, 0x60, 0x68],
};
pub(crate) const ENG_SECOND_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969dd0,
    estadio: None,
    fases: 1, // cql6: o play-off da Second Division
    regras: 0x576780,
    tabela: 0x576630,
    arquivo: 0x9c43e8,
    linhas: [0x4a, 0x51, 0x58, 0x61, 0x68],
};
pub(crate) const THIRD_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969e84,
    estadio: Some((0x1770, 0x3e8)),
    fases: 1,
    regras: 0x5780c0,
    tabela: 0x577f70,
    arquivo: 0x9c4424,
    linhas: [0x4e, 0x55, 0x5c, 0x65, 0x6d],
};

/// the Premiership: discipline rule 9 (the English leagues one), minimum ground of 10,000 all seated, only the league
/// (no play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted
/// positions
/// Original: `EngPrm::EngPrm` (0x573380, eng_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x573380 } (thiscall, ret 8, eng_prm.cpp): `EngPrm::EngPrm(ano, registro)`. A Premier: sem fases além da liga
/// (+0x2c = 0, +0xc = 0) e o estádio mínimo de 10.000 lugares, todos sentados.
pub(crate) unsafe extern "thiscall" fn construct_eng_prm(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &PREMIER_DIVISION)
}



/// the Third Division: discipline 9, minimum ground of 6,000 with 1,000 seated, the league and the play-off
/// Original: `EngThird::EngThird` (0x576dd0, eng_third.cpp, thiscall, ret 8).
///
/// [pt] { 0x576dd0 } (thiscall, ret 8, eng_third.cpp): `EngThird::EngThird(ano, registro)`. A Third Division: 1 fase além da
/// liga (o play-off) e o estádio mínimo de 6.000 lugares, 1.000 sentados.
pub(crate) unsafe extern "thiscall" fn construct_eng_third(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &THIRD_DIVISION)
}

/// Comparer of the four constructors: runs only the original and checks return value, vtable, year, record, discipline,
/// stadium and, for a new league, stages and stats.
///
/// [pt] O comparador dos quatro: alocam (as estatísticas, a tabela, os jogos): só a original, conferindo o que ela grava:
/// a devolução, a vtable, o ano, o registro e a regra de disciplina; na liga nova (+0x44 = 0xff), +0x2c, o vetor das
/// fases, a fase atual −1 e as estatísticas em +8.
pub(crate) unsafe fn compare_english_constructor(nome: &str, this: usize, ano: u32, registro: usize, p: &EnglishDivision, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_u32_as_usize(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_u32_as_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 9;
    if let Some((c, s)) = p.estadio {
        ok = ok && core::ptr::read_unaligned((this + 0xe2) as *const u16) == c && core::ptr::read_unaligned((this + 0xe4) as *const u16) == s;
    }
    if *((this + 0x44) as *const u8) == 0xff && read_u32_as_usize(this + 8) != 0 {
        ok = ok && read_u32_as_usize(this + 0x2c) == p.fases as usize && (read_u32_as_usize(this + 0xc) != 0) == (p.fases != 0);
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_u32_as_usize(this), read_u32_as_usize(this + 0x2c), read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 8)));
    r
}

/// Comparer for `construct_eng_prm` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_prm(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_PRM);
    compare_english_constructor("construct_eng_prm", this, ano, registro, &PREMIER_DIVISION, || o(this, ano, registro))
}



/// Comparer for `construct_eng_third` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_THIRD);
    compare_english_constructor("construct_eng_third", this, ano, registro, &THIRD_DIVISION, || o(this, ano, registro))
}


/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static mut CQI4_ENSAIO: bool = false;

/// Writes the rule fields (offset, size, value) into the league and builds the main-stage round table via virtual +0x3c
/// into +0xba (skipped in dry run); returns 1.
///
/// [pt] Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn eng_prm_apply_league_rules(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI4_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// Rule writes common to the four English divisions (draw points, match category, national clubs, tiebreakers 1-3,
/// +0x52 = 2).
///
/// [pt] As regras comuns às quatro divisões (os turnos +0x3c e os pontos da vitória +0xc2 vão em cada uma): 1 ponto pelo empate (+0xc3),
/// categoria 2 nas partidas (+0xc4, liga), clubes nacionais (+0x42 = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7: saldo,
/// gols pró e a posição [inferência pelos códigos da Inglaterra]) e +0x52 = 2 [sem nome].
pub(crate) fn english_common_rules() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0x52, 1, 2)]
}

/// the Premier (20 clubs): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go straight down
/// to the First; tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; builds the round
/// schedule
/// Original: `EngPrm::set_up_rules` (0x574b70, eng_prm.cpp, thiscall, ret 0).
///
/// [pt] { 0x574b70 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::set_up_rules()`. A Premier: as comuns, +0xab e +0xdb = 0
/// (as palavras copiadas em cada partida), ninguém sobe (+0xbe = 0, +0xbf = 0, +0xc0 = 0), 3 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1) e a First abaixo (+0x20 = [0x9cf5c0]).
pub(crate) unsafe extern "thiscall" fn eng_prm_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da premier");
    eng_prm_apply_league_rules(this, &premier_rules())
}

/// Rule writes for the Premier Division (2 rounds, 3 points, 3 relegated, lower division First).
pub(crate) unsafe fn premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Premier, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32)]);
    v
}


/// Rule writes for the First Division.
pub(crate) unsafe fn eng_prm_first_division_rules() -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32), (0x49, 1, 5)]);
    v
}


/// Rule writes for the Second Division.
pub(crate) unsafe fn eng_prm_second_division_rules() -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Second dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) as u32), (0x49, 1, 5)]);
    v
}

/// the Third (24): 3 go up directly, 4th to 7th to the play-offs, the bottom club goes down to the Conference when the
/// nation has its lower divisions loaded [inference from nation bit 4] (otherwise there is no league below)
/// Original: `EngThird::set_up_rules` (0x5780c0, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5780c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::set_up_rules()`. A Third: as comuns, 3 sobem direto
/// (+0xbe), 4 ao mata-mata (+0xbf, do 4º ao 7º), 1 cai (+0xc1), a Second acima ([0x9cf5c4]) e abaixo a Conference
/// ([0x9cf69c]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da nação, [[+4]+0x5d]
/// [inferência]), senão −1; 5 reservas e 3 substituições.
pub(crate) unsafe extern "thiscall" fn eng_prm_set_up_rules_5780c0(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da third");
    eng_prm_apply_league_rules(this, &third_division_rules(this))
}

/// Rule writes for the Third Division (lower division Conference only when selected).
pub(crate) unsafe fn third_division_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Third, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xbe, 1, 3), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32)]);
    let nacao = read_usize(read_usize(this + 4) + 0x5d);
    let abaixo = if *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) & 4 != 0 { read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x4a, 1, 3), (0x49, 1, 5)]);
    v
}

/// Dry-run comparer: runs the original, restores +0x1c..+0xec, runs the port without the virtual call and compares
/// every written field and the return value; keeps the original's state.
///
/// [pt] O comparador das quatro: a original monta a tabela de rodadas (aloca; não roda duas vezes), então é ensaio. A
/// original roda inteira; a faixa +0x1c..+0xec volta como estava e a cópia grava as regras sem a virtual +0x3c; os
/// campos que a cópia grava têm de sair iguais aos da original, e a devolução 1; no fim fica a faixa da original
/// (com a tabela de rodadas dela em +0xba, +0xa9 e +0x3a).
pub(crate) unsafe fn eng_prm_compare_league_rules(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let band = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&band);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&band);
    antes.escrever();
    CQI4_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&band);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

/// Comparer for `eng_prm_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_prm_set_up_rules", this, premier_rules(), || o(this), || eng_prm_set_up_rules(this))
}



/// Comparer for `eng_prm_set_up_rules_5780c0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_set_up_rules_5780c0(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_SET_UP_RULES_5780C0);
    eng_prm_compare_league_rules("eng_prm_set_up_rules_5780c0", this, third_division_rules(this), || o(this), || eng_prm_set_up_rules_5780c0(this))
}



/// Club competition record (0x6b bytes in G_CLUB_COMP_TABLE) of the id stored in global g.
///
/// [pt] O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
pub(crate) unsafe fn comp_record_of(g: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_usize(g) as i32 as usize).wrapping_mul(0x6b))
}

/// True if the current game year is after 1999 (larger TV money).
///
/// [pt] O ano do jogo ([0xae2c92]) passou de 1999 (o dinheiro da TV maior).
pub(crate) unsafe fn is_after_1999() -> bool {
    core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16) > 0x7cf
}

/// League pay_tv_rights (0x683010): pays the value to every table club, or only those that came from competition
/// `filtro`.
///
/// [pt] League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8): o valor a cada clube da tabela, ou só aos
/// que vieram da competição `filtro` (+0x5b do clube).
pub(crate) unsafe fn eng_prm_pay_tv_rights(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, valor, filtro);
}

/// Common table body: allocates the table and fills it with the division's clubs; when full sets the prize fund, +0xe1
/// and pays TV money; returns 1 if full.
///
/// [pt] O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Sem encher: 0. Cheia: o fundo de prêmios
/// +0x28 (League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4), +0xe1 = 1, o dinheiro da TV (`pagar`) e 1.
pub(crate) unsafe fn eng_prm_fill_league_table(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, cota: i32, pagar: impl FnOnce()) -> u32 {
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
        let comp = read_usize(clube + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let linha = k; // cqi5: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
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
    crate::comparacao::ramo("cqi_5 tabela cheia");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar();
    1
}

/// the Premier has 20 clubs; the prize fund starts at 250,000 per place; TV pays 12,000,000 to each club (5,000,000 up
/// to 1999)
/// Original: `EngPrm::create_table` (0x574a20, eng_prm.cpp, thiscall, ret 0).
///
/// [pt] { 0x574a20 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::create_table()`: 20 clubes ([0x9cf5bc]), cota 250.000 do
/// fundo de prêmios e a TV: 12.000.000 a cada clube depois de 1999 (5.000.000 antes). Fonte 0x9c43b0, linha 0x1ee.
pub(crate) unsafe extern "thiscall" fn eng_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(this, 0x14, 0x9c43b0, 0x1ee, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || {
        let valor = if is_after_1999() { 0xb71b00 } else { 0x4c4b40 }; // cqi5: a TV da Premier
        eng_prm_pay_tv_rights(this, valor, 0)
    })
}


/// the Third has 24 clubs; prize fund of 10,000 per place; TV pays 1,000,000 to each club (200,000 up to 1999)
/// Original: `EngThird::create_table` (0x577f70, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x577f70 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_table()`: 24 clubes ([0x9cf5c8]), cota 10.000
/// e a TV: 1.000.000 depois de 1999 (200.000 antes). Fonte 0x9c4424, linha 0x1fa.
pub(crate) unsafe extern "thiscall" fn eng_prm_create_table_577f70(this: usize) -> u32 {
    eng_prm_fill_league_table(this, 0x18, 0x9c4424, 0x1fa, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || eng_prm_pay_tv_rights(this, if is_after_1999() { 0xf4240 } else { 0x30d40 }, 0))
}


/// Comparer: plans the division's clubs beforehand, runs only the original and checks count, return value, rows and
/// prize fund.
///
/// [pt] O comparador das quatro: a original aloca a tabela e paga os clubes (não roda duas vezes): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu
/// (senão 0) e as linhas da tabela com os clubes do plano, na ordem; cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn eng_prm_compare_league_table(nome: &str, this: usize, n: i16, g: usize, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        let comp = read_usize(clube + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
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
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

/// Comparer for `eng_prm_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("eng_prm_create_table", this, 0x14, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || o(this))
}


/// Comparer for `eng_prm_create_table_577f70` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_table_577f70(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_PRM_CREATE_TABLE_577F70);
    eng_prm_compare_league_table("eng_prm_create_table_577f70", this, 0x18, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || o(this))
}

/// the Premiership schedule: 38 rounds (20 clubs), from 19 August to 20 May; Christmas (four fixed-date rounds) by the
/// day the 16 December round falls on; no play-offs (main stage only)
/// Original: `EngPrm::create_schedule` (0x573660, eng_prm.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x573660 } (thiscall, ret 0x10, eng_prm.cpp, 5.019 bytes, método +0x3c): `EngPrm::create_schedule(fase, &rodadas,
/// &extra, &é_liga)`: só a fase −1, 38 rodadas (`PREMIERSHIP_CHRISTMAS`); as outras devolvem 0. Fonte 0x9c43b0.
pub(crate) unsafe extern "thiscall" fn eng_prm_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 38;
            *extra = 0;
            let t = alocar(0x9a6);
            if t == 0 {
                show_error_at(0x9c43b0, 0xa7);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da premiership");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            eng_first_set_christmas_rounds(t, ano, &PREMIERSHIP_CHRISTMAS, 0x9c43b0);
            t
        }
        _ => 0,
    }
}

/// Comparer for `eng_prm_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_prm_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_PRM_CREATE_SCHEDULE);
    eng_first_compare_round_table("eng_prm_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_prm_create_schedule(this, fase, r, e, l))
}
