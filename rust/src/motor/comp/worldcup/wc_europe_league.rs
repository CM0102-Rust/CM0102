// ---------- 274a leva: as regras das eliminatórias da Copa do Mundo: Europa, América do Sul e Ásia (wc_europe_league.cpp, wc_south_american_league.cpp, wc_asia_league.cpp) ----------

/// Os campos de byte e palavra de uma eliminatória (a virtual +0x8c, set_up_rules, na liga do grupo): (deslocamento,
/// tamanho 1 ou 2, valor). Os pontos da vitória (+0xc2) e a categoria (+0xc4) vêm à parte, para as mutações.
unsafe fn cqj12_campos(this: usize, campos: &[(usize, u8, u32)]) {
    for &(o, n, v) in campos {
        if n == 1 {
            *((this + o) as *mut u8) = v as u8;
        } else {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        }
    }
}

/// A primeira palavra da Date(dia, mês, ano da competição +0x40, dia da semana) (0x549f70: push semana; push ano; push
/// mês; push dia; lea ecx, local -> thiscall, ret 0x10).
unsafe fn cqj12_data(this: usize, dia: u32, mes: u32, semana: u32) -> u16 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    data(d.as_mut_ptr(), dia, mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, semana);
    core::ptr::read_unaligned(d.as_ptr() as *const u16)
}

/// O fim comum: +0xb6 = a palavra da data, +0xb8 = `b8`, e a tabela das rodadas da virtual +0x3c (push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
unsafe fn cqj12_fim(this: usize, palavra: u16, b8: u16) -> u32 {
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, palavra);
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, b8);
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// O plano: os campos, +0xb6 (a data, calculada antes), +0xb8 e a devolução 1, lidos depois da original `o`.
unsafe fn cqj12_confere(nome: &str, this: usize, campos: &[(usize, u8, u32)], data: (u32, u32, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let palavra = cqj12_data(this, data.0, data.1, data.2);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in campos {
        let (lido, v) = if n == 1 { (*((this + off) as *const u8) as u32, v & 0xff) } else { (core::ptr::read_unaligned((this + off) as *const u16) as u32, v & 0xffff) };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = core::ptr::read_unaligned((this + 0xb6) as *const u16);
    let lido8 = core::ptr::read_unaligned((this + 0xb8) as *const u16);
    PLACAR.registrar(r == 1 && k == 0 && b6 == palavra && lido8 == b8, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {}", nome, this, r, &errados[..k], b6, palavra, lido8));
    r
}

const CQJ12_EUROPA: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2), (0xc5, 1, 1),
    (0xc6, 1, 2), (0xc7, 1, 4), (0xc8, 1, 5), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// rule: the European groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2, 4 and 5
/// [inference: goal difference, goals for, head-to-head, away goals]; each group winner goes straight through; national
/// teams (+0x43 = 2); category 10 (national teams) on each match; 7 substitutes and 3 changes; starts on 25 of month 2
/// [inference: 25 March] moved to weekday 5
/// Original: `WcEuropeLeague::set_up_rules` (0x9191e0, wc_europe_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x9191e0 } (thiscall, ret 0, wc_europe_league.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `WcEuropeLeague::set_up_rules()`: os grupos europeus em dois turnos, 3 pontos a vitória e 1 o empate, a categoria
/// 0xa (seleções), os desempates (1, 2, 4, 5), um classificado direto por grupo, seleções (+0x43 = 2), 7 no banco e 3
/// substituições, começo em 25 do mês 2 num dia da semana 5; depois a tabela das rodadas.
unsafe extern "thiscall" fn cqj12_eliminatoria_europa_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_12 regras da europa");
    cqj12_campos(this, &CQJ12_EUROPA);
    *((this + 0xc2) as *mut u8) = 3; // cqj12: a vitória nas eliminatórias europeias
    *((this + 0xc4) as *mut u8) = 0xa; // cqj12: a categoria dos jogos das eliminatórias europeias
    let dia = 0x19; // cqj12: o dia do começo das eliminatórias europeias
    let palavra = cqj12_data(this, dia, 2, 5);
    cqj12_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj12_eliminatoria_europa_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1700);
    cqj12_confere("cqj12_eliminatoria_europa_regras", this, &CQJ12_EUROPA, (0x19, 2, 5), 0, || o(this))
}

const CQJ12_SUL: [(usize, u8, u32); 16] = [(0x3c, 2, 2), (0x43, 1, 2), (0xc6, 1, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 0xa), (0xab, 2, 0),
    (0xdb, 2, 0), (0xc5, 1, 1), (0xc7, 1, 3), (0xbf, 1, 1), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0x4a, 1, 3), (0xa7, 2, 0xffff)];

/// rule: the South American qualifier is a single home and away league, 3/1 points, tiebreak 1, 2 and 3; the direct
/// places are four minus the exempt nations (the world champion and South American hosts, counted in +0xee and +0xef
/// [inference]); the fifth goes to the play-off (+0xbf = 1); category 10, 7 substitutes and 3 changes; starts on 25 of
/// month 0 on weekday 5
/// Original: `WcSouthAmericanLeague::set_up_rules` (0x921040, wc_south_american_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x921040 } (thiscall, ret 0, wc_south_american_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcSouthAmericanLeague::set_up_rules()`: com o objeto da competição [0x9cf76c], +0xef recebe o +0xee dele [inferência:
/// as nações isentas por serem a campeã]; a liga única sul-americana em dois turnos, 3/1 pontos, a categoria 0xa, os
/// desempates (1, 2, 3), vagas diretas 4 − +0xef − +0xee (as isentas saem das quatro), uma de repescagem, seleções, 7 no
/// banco e 3 substituições, +0xa7 = −1, +0xab = +0xdb = 0, começo em 25 do mês 0 num dia da semana 5.
unsafe extern "thiscall" fn cqj12_eliminatoria_sul_americana_regras(this: usize) -> u32 {
    let lista = core::ptr::read_unaligned(0xadadfc as *const usize);
    if lista != 0 {
        let o = core::ptr::read_unaligned((lista + core::ptr::read_unaligned(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as *const usize).wrapping_mul(4)) as *const usize);
        if o != 0 {
            *((this + 0xef) as *mut u8) = *((o + 0xee) as *const u8);
        }
    }
    crate::comparacao::ramo("cqj_12 regras da america do sul");
    cqj12_campos(this, &CQJ12_SUL);
    *((this + 0xc2) as *mut u8) = 3; // cqj12: a vitória nas eliminatórias sul-americanas
    *((this + 0xbe) as *mut u8) = 4u8.wrapping_sub(*((this + 0xef) as *const u8)).wrapping_sub(*((this + 0xee) as *const u8)); // cqj12: as vagas diretas da América do Sul
    let dia = 0x19; // cqj12: o dia do começo da eliminatória sul-americana
    let palavra = cqj12_data(this, dia, 0, 5);
    cqj12_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras e
// as vagas diretas (4 − +0xef − +0xee, com o +0xef que a própria rotina copia, calculado antes).
unsafe extern "thiscall" fn comparar_cqj12_eliminatoria_sul_americana_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1701);
    let lista = core::ptr::read_unaligned(0xadadfc as *const usize);
    let mut ef = *((this + 0xef) as *const u8);
    if lista != 0 {
        let c = core::ptr::read_unaligned((lista + core::ptr::read_unaligned(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as *const usize).wrapping_mul(4)) as *const usize);
        if c != 0 {
            ef = *((c + 0xee) as *const u8);
        }
    }
    let vagas = 4u8.wrapping_sub(ef).wrapping_sub(*((this + 0xee) as *const u8));
    let r = cqj12_confere("cqj12_eliminatoria_sul_americana_regras", this, &CQJ12_SUL, (0x19, 0, 5), 0, || o(this));
    let (lido_ef, lido_be) = (*((this + 0xef) as *const u8), *((this + 0xbe) as *const u8));
    PLACAR.registrar(lido_ef == ef && lido_be == vagas, || format!("cqj12_eliminatoria_sul_americana_regras({:#x}) +0xef {} previsto {} vagas {} previstas {}", this, lido_ef, ef, lido_be, vagas));
    r
}

const CQJ12_ASIA: [(usize, u8, u32); 12] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2), (0xc5, 1, 1),
    (0xc6, 1, 2), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0)];

/// rule: the Asian groups are home and away, 3/1 points, tiebreak 1, 2 and 3, each group winner goes on, category 10,
/// starts on 25 of month 2 on weekday 5, +0xb8 = 1 [inference: the stage starts the following year]
/// Original: `WcAsiaLeague::set_up_rules` (0x912eb0, wc_asia_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x912eb0 } (thiscall, ret 0, wc_asia_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcAsiaLeague::set_up_rules()`: os grupos asiáticos em dois turnos, 3/1 pontos, a categoria 0xa, os desempates (1, 2,
/// 3), um classificado por grupo, seleções, começo em 25 do mês 2 num dia da semana 5, +0xb8 = 1.
unsafe extern "thiscall" fn cqj12_eliminatoria_asia_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_12 regras da asia");
    cqj12_campos(this, &CQJ12_ASIA);
    let dia = 0x19; // cqj12: o dia do começo das eliminatórias asiáticas
    let palavra = cqj12_data(this, dia, 2, 5);
    cqj12_fim(this, palavra, 1)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj12_eliminatoria_asia_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1702);
    cqj12_confere("cqj12_eliminatoria_asia_regras", this, &CQJ12_ASIA, (0x19, 2, 5), 1, || o(this))
}

