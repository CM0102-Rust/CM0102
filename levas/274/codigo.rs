// ---------- 274a leva: as regras dos grupos continentais de clubes: a Libertadores, a Copa Merconorte e o Campeonato de Clubes da Oceania (conmebol_liber.cpp, con_merc_cup.cpp, oceania_club_champ.cpp) ----------

/// As regras de um grupo continental (a virtual +0x8c, set_up_rules, numa liga de grupo): os campos de byte e palavra
/// gravados (deslocamento, tamanho 1 ou 2, valor), a data de começo (0x549f70: dia, mês, ano da competição +0x40, dia da
/// semana; −1 = qualquer) e o fundo de prêmios (+0x28), quando há.
struct Cqj11Regras {
    campos: &'static [(usize, u8, u32)],
    dia: u32,
    mes: u32,
    semana: u32,
    premio: Option<u32>,
}

/// Grava os campos de `g` (os pontos da vitória, +0xc2, vêm de `vitoria`); a data de começo numa Date local (0x549f70: push semana; push ano; push mês; push dia; lea
/// ecx, local -> thiscall, ret 0x10) e a primeira palavra dela em +0xb6, +0xb8 = 0; o prêmio; depois a tabela das
/// rodadas da virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba.
/// Devolve 1.
unsafe fn cqj11_regras(this: usize, g: &Cqj11Regras, dia: u32, vitoria: u8) -> u32 {
    for &(o, n, v) in g.campos {
        let v = if o == 0xc2 { vitoria as u32 } else { v };
        if n == 1 {
            *((this + o) as *mut u8) = v as u8;
        } else if n == 2 {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        } else {
            core::ptr::write_unaligned((this + o) as *mut u32, v);
        }
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    // ponytail: o ano vai num registrador que pode ter lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    data(d.as_mut_ptr(), dia, g.mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, g.semana);
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, core::ptr::read_unaligned(d.as_ptr() as *const u16));
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, 0);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + 0x28) as *mut u32, p);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// O plano: os campos de `g`, a palavra da data (calculada antes numa Date local pela 0x549f70, sem efeito fora dela),
/// +0xb8 = 0, o prêmio e a devolução 1.
unsafe fn cqj11_confere(nome: &str, this: usize, g: &Cqj11Regras, o: impl FnOnce() -> u32) -> u32 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    data(d.as_mut_ptr(), g.dia, g.mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, g.semana);
    let palavra = core::ptr::read_unaligned(d.as_ptr() as *const u16);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in g.campos {
        let lido = match n {
            1 => *((this + off) as *const u8) as u32,
            2 => core::ptr::read_unaligned((this + off) as *const u16) as u32,
            _ => core::ptr::read_unaligned((this + off) as *const u32),
        };
        let v = if n == 1 { v & 0xff } else if n == 2 { v & 0xffff } else { v };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = core::ptr::read_unaligned((this + 0xb6) as *const u16);
    let b8 = core::ptr::read_unaligned((this + 0xb8) as *const u16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + 0x28) as *const u32) == p);
    PLACAR.registrar(r == 1 && k == 0 && b6 == palavra && b8 == 0 && premio, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {} prêmio ok {}", nome, this, r, &errados[..k], b6, palavra, b8, premio));
    r
}

const CQJ11_LIBERTADORES: Cqj11Regras = Cqj11Regras {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7),
        (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    dia: 0xf, mes: 0, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

const CQJ11_MERCONORTE: Cqj11Regras = Cqj11Regras {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1), (0xbe, 1, 1),
        (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff)],
    dia: 1, mes: 7, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

const CQJ11_OCEANIA: Cqj11Regras = Cqj11Regras {
    campos: &[(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x49, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 8),
        (0xc5, 1, 1), (0xc7, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xd9, 2, 0x28)],
    dia: 0x14, mes: 7, semana: 5, premio: None,
};

/// { 0x4c3890 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `ConmebolLiber::set_up_rules()`: os grupos da Libertadores em dois turnos, 3 pontos a vitória e 1 o empate, os
/// desempates (1, 2, 3, 0), dois classificados por grupo, clubes internacionais (+0x42 = 2), categoria 7, 7 no banco e 3
/// substituições, começo no dia 15 do mês 0 do ano da competição, prêmios de 2.000.000; depois a tabela das rodadas.
unsafe extern "thiscall" fn cqj11_libertadores_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da libertadores");
    let dia = 0xf; // cqj11: o dia do começo da Libertadores
    let vitoria = 3; // cqj11: a vitória nos grupos da Libertadores
    cqj11_regras(this, &CQJ11_LIBERTADORES, dia, vitoria)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj11_libertadores_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1697);
    cqj11_confere("cqj11_libertadores_regras", this, &CQJ11_LIBERTADORES, || o(this))
}

/// { 0x4c1ed0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x8c; quadro SEH só da data local): `ConMercCup::
/// set_up_rules()`: os grupos da Copa Merconorte em dois turnos, 3/1 pontos, desempates (1, 2, 3, 0), um classificado
/// por grupo, clubes internacionais, categoria 7, começo no dia 1 do mês 7, prêmios de 2.000.000.
unsafe extern "thiscall" fn cqj11_merconorte_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da merconorte");
    cqj11_regras(this, &CQJ11_MERCONORTE, CQJ11_MERCONORTE.dia, 3)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj11_merconorte_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1698);
    cqj11_confere("cqj11_merconorte_regras", this, &CQJ11_MERCONORTE, || o(this))
}

/// { 0x7979b0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `OceaniaClubChamp::set_up_rules()`: os grupos do Campeonato de Clubes da Oceania em turno único, 3/1 pontos,
/// desempates (1, 2, 4, 0), dois classificados, categoria 8, 3 no banco e 3 substituições, +0x43 = 2, os bits de regra
/// 0x28 (os de cima classificados, os outros eliminados), começo no dia 20 do mês 7 num dia da semana 5; sem prêmio.
unsafe extern "thiscall" fn cqj11_oceania_clubes_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da oceania");
    let dia = 0x14; // cqj11: o dia do começo na Oceania
    cqj11_regras(this, &CQJ11_OCEANIA, dia, 3)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj11_oceania_clubes_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1699);
    cqj11_confere("cqj11_oceania_clubes_regras", this, &CQJ11_OCEANIA, || o(this))
}

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
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
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

/// { 0x9191e0 } (thiscall, ret 0, wc_europe_league.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
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

/// { 0x921040 } (thiscall, ret 0, wc_south_american_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcSouthAmericanLeague::set_up_rules()`: com o objeto da competição [0x9cf76c], +0xef recebe o +0xee dele [inferência:
/// as nações isentas por serem a campeã]; a liga única sul-americana em dois turnos, 3/1 pontos, a categoria 0xa, os
/// desempates (1, 2, 3), vagas diretas 4 − +0xef − +0xee (as isentas saem das quatro), uma de repescagem, seleções, 7 no
/// banco e 3 substituições, +0xa7 = −1, +0xab = +0xdb = 0, começo em 25 do mês 0 num dia da semana 5.
unsafe extern "thiscall" fn cqj12_eliminatoria_sul_americana_regras(this: usize) -> u32 {
    let lista = core::ptr::read_unaligned(0xadadfc as *const usize);
    if lista != 0 {
        let o = core::ptr::read_unaligned((lista + core::ptr::read_unaligned(0x9cf76c as *const usize).wrapping_mul(4)) as *const usize);
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
        let c = core::ptr::read_unaligned((lista + core::ptr::read_unaligned(0x9cf76c as *const usize).wrapping_mul(4)) as *const usize);
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

/// { 0x912eb0 } (thiscall, ret 0, wc_asia_league.cpp; a virtual +0x8c; quadro SEH só da data local):
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

// ---------- 274a leva: as regras dos torneios de seleções: a Copa América, a Copa Africana de Nações e a eliminatória da Oceania (goldcup.cpp, african_nations.cpp, wc_oceania_league.cpp) ----------

/// Os campos de byte e palavra que a virtual +0x8c (set_up_rules) de um torneio de seleções grava na liga do grupo:
/// (deslocamento, tamanho 1 ou 2, valor). Os pontos da vitória (+0xc2) vêm à parte, para as mutações.
unsafe fn cqk10_campos(this: usize, campos: &[(usize, u8, u32)], vitoria: u8) {
    for &(o, n, v) in campos {
        if n == 1 {
            *((this + o) as *mut u8) = if o == 0xc2 { vitoria } else { v as u8 };
        } else {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        }
    }
}

/// A primeira palavra da Date(dia, mês, ano da competição +0x40 + `ano`, dia da semana) (0x549f70: push semana; push
/// ano; push mês; push dia; lea ecx, local -> thiscall, ret 0x10).
unsafe fn cqk10_data(this: usize, dia: u32, mes: u32, ano: i16, semana: u32) -> u16 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16).wrapping_add(ano) as u16 as u32;
    data(d.as_mut_ptr(), dia, mes, a, semana);
    core::ptr::read_unaligned(d.as_ptr() as *const u16)
}

/// O fim comum: +0xb6 = a palavra da data, +0xb8 = `b8`, e a tabela das rodadas da virtual +0x3c (push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
unsafe fn cqk10_fim(this: usize, palavra: u16, b8: u16) -> u32 {
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, palavra);
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, b8);
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// O plano: os campos, +0xb6 (a data, calculada antes), +0xb8 e a devolução 1, lidos depois da original `o`.
unsafe fn cqk10_confere(nome: &str, this: usize, campos: &[(usize, u8, u32)], data: (u32, u32, i16, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let palavra = cqk10_data(this, data.0, data.1, data.2, data.3);
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
    PLACAR.registrar(r == 1 && k == 0 && b6 == palavra && lido8 == b8, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {:#x}", nome, this, r, &errados[..k], b6, palavra, lido8));
    r
}

const CQK10_COPA_AMERICA: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2),
    (0xc5, 1, 1), (0xc6, 1, 2), (0xc8, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// { 0x5e12d0 } (thiscall, ret 0, goldcup.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `GoldCup::set_up_rules()`: os grupos da Copa América em turno único, 3 pontos a vitória e 1 o empate, a categoria 0xa
/// (seleções), os desempates (1, 2, 3, 0), dois classificados por grupo, seleções (+0x43 = 2), 7 no banco e 3
/// substituições, começo em 20 do mês 2 do ano da competição num dia da semana 5; depois a tabela das rodadas.
unsafe extern "thiscall" fn cqk10_copa_america_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da copa america");
    let vitoria = 3; // cqk10: a vitória nos grupos da Copa América
    cqk10_campos(this, &CQK10_COPA_AMERICA, vitoria);
    let dia = 0x14; // cqk10: o dia da data da Copa América
    let palavra = cqk10_data(this, dia, 2, 0, 5);
    cqk10_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqk10_copa_america_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1703);
    cqk10_confere("cqk10_copa_america_regras", this, &CQK10_COPA_AMERICA, (0x14, 2, 0, 5), 0, || o(this))
}

const CQK10_AFRICANA: [(usize, u8, u32); 14] = [(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1),
    (0xc3, 1, 1), (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// { 0x401ab0 } (thiscall, ret 0, african_nations.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `AfricanNations::set_up_rules()`: os grupos da Copa Africana em turno único, 3/1 pontos, a categoria 0xa, os
/// desempates (1, 2, 3), dois classificados por grupo, seleções, 7 no banco e 3 substituições, a data em 5 do mês 9 do
/// ano anterior ao da competição num dia da semana 5 [inferência: o sorteio] e +0xb8 = −1.
unsafe extern "thiscall" fn cqk10_copa_africana_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da copa africana");
    let vitoria = 3; // cqk10: a vitória nos grupos da Copa Africana
    cqk10_campos(this, &CQK10_AFRICANA, vitoria);
    let dia = 5; // cqk10: o dia da data da Copa Africana
    let palavra = cqk10_data(this, dia, 9, -1, 5);
    cqk10_fim(this, palavra, 0xffff)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqk10_copa_africana_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1704);
    cqk10_confere("cqk10_copa_africana_regras", this, &CQK10_AFRICANA, (5, 9, -1, 5), 0xffff, || o(this))
}

const CQK10_OCEANIA: [(usize, u8, u32); 14] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc6, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// { 0x91ecf0 } (thiscall, ret 0, wc_oceania_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcOceaniaLeague::set_up_rules()`: os grupos da eliminatória da Oceania em turno único, 3/1 pontos, a categoria 0xa,
/// os desempates (1, 2, 3), um classificado por grupo, seleções, 7 no banco e 3 substituições, começo em 15 do mês 0
/// do ano da competição num dia da semana 5.
unsafe extern "thiscall" fn cqk10_eliminatoria_oceania_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da oceania");
    cqk10_campos(this, &CQK10_OCEANIA, 3);
    let dia = 0xf; // cqk10: o dia do começo na Oceania
    let palavra = cqk10_data(this, dia, 0, 0, 5);
    cqk10_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqk10_eliminatoria_oceania_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1705);
    cqk10_confere("cqk10_eliminatoria_oceania_regras", this, &CQK10_OCEANIA, (0xf, 0, 0, 5), 0, || o(this))
}

// ---------- 274a leva: a Libertadores: os 32 classificados, o destino de cada clube (o campeão vai ao Mundial Interclubes) e a fase seguinte (conmebol_liber.cpp) ----------

unsafe fn cqk11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os clubes da competição da Libertadores na ordem do banco ([0xae23b4], 0x245 bytes, [0xae235c] clubes): os que têm
/// a competição europeia/continental da temporada (+0x1db) igual ao id da competição ([+4]), até `max`.
unsafe fn cqk11_inscritos(this: usize, max: i32, saida: &mut [usize; 32]) -> i32 {
    let mut n = 0i32;
    let mut i = 0i16;
    let mut clube = cqk11_d(0xae23b4);
    if (cqk11_d(0xae235c) as i32) > 0 {
        loop {
            if cqk11_d(clube + 0x1db) == cqk11_d(cqk11_d(this + 4)) && n < max {
                if (n as usize) < saida.len() {
                    saida[n as usize] = clube;
                }
                n += 1;
            }
            i = i.wrapping_add(1);
            clube = clube.wrapping_add(0x245);
            if i as i32 >= cqk11_d(0xae235c) as i32 {
                break;
            }
        }
    }
    n
}

/// { 0x4c3f40 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::select_qualified_clubs()`: +0xf2 = 32 e a lista
/// +0xee (0x944e46(0x80): push 0x80; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x285 e devolve 0) com os
/// clubes, na ordem do banco, cuja competição da temporada (+0x1db) é a Libertadores, até 32; menos de 32, a caixa da
/// linha 0x297 e devolve 0; senão a lista embaralhada (0x4afb40(lista, 32): push n; push lista; add esp, 8 -> cdecl; 3 ×
/// 32 trocas sorteadas) e devolve 1.
unsafe extern "thiscall" fn cqk11_libertadores_classificados(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    let lista = novo(0x80);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, lista as u32);
    if lista == 0 {
        aviso_em(0x9a7de4, 0x285);
        return 0;
    }
    let mut v = [0usize; 32];
    let n = cqk11_inscritos(this, cqk11_d(this + 0xf2) as i32, &mut v);
    for (k, &c) in v.iter().enumerate().take(n.min(32) as usize) {
        core::ptr::write_unaligned((lista + k * 4) as *mut u32, c as u32); // cqk11: o clube na ordem do banco
    }
    if n != cqk11_d(this + 0xf2) as i32 {
        aviso_em(0x9a7de4, 0x297);
        return 0;
    }
    crate::comparacao::ramo("cqk_11 classificados da libertadores");
    embaralhar(lista, cqk11_w(this + 0xf2) as u16 as u32); // cqk11: os 32 da Libertadores embaralhados
    1
}

/// Comparador da 0x4c3f40: lado a lado com as duas listas (a original aloca a dela, a cópia a sua): a original roda e a
/// lista dela fica; o estado do sorteador volta, a cópia roda sem contar e a lista dela é comparada com a da original e
/// liberada (0x9452ca: push; add esp, 4 -> cdecl). Com menos de 32 inscritos (as caixas): só a original.
unsafe extern "thiscall" fn comparar_cqk11_libertadores_classificados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1706);
    let mut v = [0usize; 32];
    if cqk11_inscritos(this, 0x20, &mut v) != 0x20 {
        return o(this); // ponytail: as caixas de erro: só a original
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (ee, f2) = (cqk11_d(this + 0xee), cqk11_d(this + 0xf2));
    let r = o(this);
    let (la, depois) = (cqk11_d(this + 0xee), crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO));
    let a: [u32; 32] = if la != 0 { core::ptr::read_unaligned(la as *const [u32; 32]) } else { [0; 32] };
    estado.escrever();
    core::ptr::write_unaligned((this + 0xee) as *mut u32, ee as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, f2 as u32);
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cqk11_libertadores_classificados(this)));
    let lb = cqk11_d(this + 0xee);
    let b: [u32; 32] = if lb != 0 { core::ptr::read_unaligned(lb as *const [u32; 32]) } else { [0; 32] };
    let igual_estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois;
    if lb != 0 && lb != la {
        soltar(lb);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut u32, la as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    depois.escrever();
    PLACAR.registrar(r == rr && a == b && igual_estado, || format!("cqk11_libertadores_classificados({:#x}) devolveu {} x {} estado igual {} original {:x?} rust {:x?}", this, r, rr, igual_estado, a, b));
    r
}

/// { 0x4c4080 } (thiscall, ret 0x18, conmebol_liber.cpp; a virtual +0x44): `ConmebolLiber::
/// update_staff_records_for_status(clube, desfecho, fase, _, jogo, a6)`; devolve 0. Fase (byte com sinal) abaixo de 7,
/// os grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x14, peso 0x28); 0xfe, a não classificação (0x86c1d0:
/// 0x41c, 0x28). Fase 7, o mata-mata: peso 0x4bf850(0x32, 0x64, rodada do jogo +0x34, 4); 0xff, a eliminação (0x86c000:
/// +0x32, +0x30 do jogo, peso); 0, a classificação com +0x32 e o +0x6f da rodada do jogo na tabela de rodadas da fase 7
/// ([+0xc] + 0x1c, +0xa3, 0x68 bytes; os 16 bits altos que a original empurra são os do ponteiro da fase); 1, o registro
/// de campeão (0x86a800) e o clube vai ao Mundial Interclubes (o objeto [[0xadadfc] + 4 × [0x9cf6e4]]): na vaga 0 da
/// tabela +0xa7 dele se o clube que lá está é do continente [0x9cfa1c] (a nação +0x53, o continente +0x71 da nação, +0
/// o id), senão na vaga 1; depois o arquivo da temporada das estatísticas dele (0x4a1c50 em +8: push 1 -> thiscall, ret
/// 4) e as posições previstas (virtual +0x5c); 2, o registro de vice (0x86b0b0).
unsafe extern "thiscall" fn cqk11_libertadores_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqk11_d(this + 4);
    let final_ = |g: usize| {
        let h: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(g);
        h(cpz4_registros(), clube, jogo, a6);
    };
    if f < 7 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x14, 0x28);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x41c, 0x28);
        }
    } else if f == 7 {
        match d {
            0xff => {
                let peso = cpz4_escala(0x32, 0x64, cqk11_w(jogo + 0x34), 4);
                // ponytail: +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
                cpz4_eliminado(clube, comp, cqk11_w(jogo + 0x32) as u16 as u32, cqk11_w(jogo + 0x30) as u16 as u32, peso);
            }
            0 => {
                let peso = cpz4_escala(0x32, 0x64, cqk11_w(jogo + 0x34), 4);
                let mata = cqk11_d(cqk11_d(this + 0xc) + 0x1c);
                let r = cqk11_w(jogo + 0x34) as isize;
                let rodada = (mata as u32 & 0xffff_0000) | core::ptr::read_unaligned((cqk11_d(mata + 0xa3).wrapping_add((r * 0x68) as usize) + 0x6f) as *const u16) as u32;
                cpz4_classificado(clube, comp, core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, rodada, peso);
            }
            1 => {
                final_(0x86a800);
                let mundial = cqk11_d(cqk11_d(0xadadfc).wrapping_add(cqk11_d(0x9cf6e4).wrapping_mul(4)));
                let t = cqk11_d(mundial + 0xa7);
                let nacao = cqk11_d(cqk11_d(t) + 0x53);
                crate::comparacao::ramo("cqk_11 campeao da libertadores");
                if nacao != 0 && cqk11_d(cqk11_d(nacao + 0x71)) == cqk11_d(0x9cfa1c) {
                    core::ptr::write_unaligned(t as *mut u32, clube as u32); // cqk11: o campeão na vaga sul-americana
                } else {
                    core::ptr::write_unaligned((t + 6) as *mut u32, clube as u32);
                }
                let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
                arquivar(cqk11_d(mundial + 8), 1);
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqk11_d(cqk11_d(mundial) + 0x5c));
                prever(mundial);
            }
            2 => final_(0x86b0b0),
            _ => {}
        }
    }
    0
}

// ponytail: os registros (histórico, convocações), o arquivo das estatísticas e as posições previstas do Mundial não
// rodam duas vezes. Só a original; o plano: a devolução 0 e, no título (fase 7, desfecho 1), o campeão na vaga do
// Mundial Interclubes que a regra escolhe (0 se lá está um clube do continente [0x9cfa1c], senão 1).
unsafe extern "thiscall" fn comparar_cqk11_libertadores_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1707);
    let titulo = fase as u8 == 7 && desfecho as u8 == 1;
    let mut vaga = 0usize;
    if titulo {
        let mundial = cqk11_d(cqk11_d(0xadadfc).wrapping_add(cqk11_d(0x9cf6e4).wrapping_mul(4)));
        let t = cqk11_d(mundial + 0xa7);
        let nacao = cqk11_d(cqk11_d(t) + 0x53);
        vaga = if nacao != 0 && cqk11_d(cqk11_d(nacao + 0x71)) == cqk11_d(0x9cfa1c) { t } else { t + 6 };
    }
    let r = o(this, clube, desfecho, fase, a4, jogo, a6);
    let igual = r == 0 && (!titulo || cqk11_d(vaga) == clube);
    PLACAR.registrar(igual, || format!("cqk11_libertadores_registros({:#x}, {}, {}) devolveu {} titulo={} vaga {:#x}", clube, desfecho as u8, fase as u8, r, titulo, vaga));
    r
}

/// { 0x4c3a80 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x28): `ConmebolLiber::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x1f6; senão +0x30 sobe e, chegando a 7 (o fim dos sete grupos), o
/// mata-mata (0x4c3d10: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqk11_libertadores_fase_seguinte(this: usize) {
    let f = cqk11_d(this + 0x30) as i32;
    if f >= (cqk11_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(0x9a7de4, 0x1f6);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 7 { // cqk11: o mata-mata depois dos sete grupos
        crate::comparacao::ramo("cqk_11 mata-mata da libertadores");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c3d10usize);
        mata(this);
    }
}

// ponytail: o mata-mata (0x4c3d10) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
unsafe extern "thiscall" fn comparar_cqk11_libertadores_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1708);
    let f = cqk11_d(this + 0x30) as i32;
    if f >= (cqk11_d(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = cqk11_d(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("cqk11_libertadores_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

// ---------- 274a leva: a Copa Africana de Nações: a fase seguinte, a tabela do grupo A e o histórico da temporada (african_nations.cpp) ----------

unsafe fn cqk12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x401e10 } (thiscall, ret 0, african_nations.cpp; a virtual +0x28): `AfricanNations::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x2ba; senão +0x30 sobe e, chegando a 3 (depois dos grupos), o mata-mata
/// (0x402090: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqk12_africana_fase_seguinte(this: usize) {
    let f = cqk12_d(this + 0x30) as i32;
    if f >= (cqk12_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(0x9870f8, 0x2ba);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 3 { // cqk12: o mata-mata da Copa Africana depois dos grupos
        crate::comparacao::ramo("cqk_12 mata-mata da copa africana");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x402090usize);
        mata(this);
    }
}

// ponytail: o mata-mata (0x402090) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
unsafe extern "thiscall" fn comparar_cqk12_africana_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1709);
    let f = cqk12_d(this + 0x30) as i32;
    if f >= (cqk12_d(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = cqk12_d(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("cqk12_africana_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// { 0x4019c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::init_group_a_table()`: o grupo A com 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa
/// da linha 0x1f1 e devolve 0), cada linha i com a seleção i da lista dos sorteados (+0xee) (0x687430: push 0; push 0;
/// push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this
/// -> thiscall, ret 0) e devolve 1.
unsafe extern "thiscall" fn cqk12_africana_grupo_a(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    let times = 4i16; // cqk12: os quatro do grupo A
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, times);
    let t = novo(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        aviso_em(0x9870f8, 0x1f1);
        return 0;
    }
    crate::comparacao::ramo("cqk_12 grupo a da copa africana");
    let mut i = 0i8;
    while (i as i16) < cqk12_w(this + 0x3e) {
        let selecao = cqk12_d(cqk12_d(this + 0xee).wrapping_add((i as isize * 4) as usize)); // cqk12: a seleção da linha
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, selecao, 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico de posições) e a notícia vai aos técnicos: não roda duas
// vezes. Só a original; o plano: 4 times, a tabela, e em cada linha i a seleção i dos sorteados (+0xee) com a posição i
// (+4 da linha, 0x3b bytes).
unsafe extern "thiscall" fn comparar_cqk12_africana_grupo_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1710);
    let lista = cqk12_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqk12_d(lista + i * 4));
    let r = o(this);
    let t = cqk12_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqk12_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqk12_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqk12_africana_grupo_a({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x401b90 } (thiscall, ret 0, african_nations.cpp; a virtual +0x30): `AfricanNations::record_season_honours()`: o
/// histórico vem do mata-mata: salta para a virtual +0x30 da fase 3 ([+0xc] + 0xc; jmp, sem quadro).
unsafe extern "thiscall" fn cqk12_africana_historico(this: usize) {
    let fase = cqk12_d(cqk12_d(this + 0xc) + 0xc);
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqk12_d(cqk12_d(fase) + 0x30));
    h(fase);
}

// ponytail: o histórico (a virtual +0x30 do mata-mata, a 0x519a90 já em Rust com o seu comparador) não roda duas vezes;
// a rotina não grava nada além do salto. Só a original.
unsafe extern "thiscall" fn comparar_cqk12_africana_historico(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1711)(this)
}

// ---------- 274a leva: as regras das três divisões nacionais do Brasil (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------

unsafe fn cqi7_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqi7_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqi7_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x429b50, 0x42c140, 0x42d620): estágio -1, as saídas +0xa9 e +0x3a (palavras), sem a marca (0); o ponteiro
/// devolvido vai para +0xba.
unsafe fn cqi7_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqi7_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba), que o ensaio não chama: os
/// três calendários não gravam nada no objeto além delas (lido nas três rotinas: nenhum `mov [edi+..]`).
fn cqi7_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// As regras da Série A (o que a 0x42a5a0 grava antes do calendário).
unsafe fn cqi7_regras_serie_a(o: usize) {
    cqi7_w(o, 0x3c, 1); // um turno
    cqi7_b(o, 0xc2, 3); // cqi7: os pontos por vitória da Série A
    cqi7_b(o, 0xc3, 1); // pontos por empate
    cqi7_b(o, 0xc4, 2); // categoria das partidas: liga
    cqi7_b(o, 0x42, 1); // clube nacional
    cqi7_b(o, 0xc5, 3); // desempates: confronto direto, saldo, nenhum
    cqi7_b(o, 0xc6, 1);
    cqi7_b(o, 0xc7, 0);
    cqi7_b(o, 0xbe, 4); // vagas de cima
    cqi7_b(o, 0xbf, 0);
    cqi7_b(o, 0xc0, 0);
    cqi7_b(o, 0xc1, 4); // rebaixados
    cqi7_d(o, 0x1c, 0xffff_ffff); // nenhuma liga acima
    cqi7_b(o, 0x4a, 3); // substituições
    cqi7_d(o, 0x20, *(0x9cf9e0 as *const u32)); // cai para a Série B
    cqi7_w(o, 0xd9, 8); // bit 0x8: os de cima vão para a fase seguinte (sem acesso, sem faixa de campeão)
    cqi7_b(o, 0x52, 2); // numeração fixa do elenco
    cqi7_b(o, 0x49, 7); // reservas no banco
}

/// As regras da Série B (0x42cb70).
unsafe fn cqi7_regras_serie_b(o: usize) {
    cqi7_w(o, 0x3c, 1);
    cqi7_b(o, 0xc2, 3);
    cqi7_b(o, 0xc3, 1); // cqi7: os pontos por empate da Série B
    cqi7_b(o, 0xc4, 2);
    cqi7_b(o, 0x42, 1);
    cqi7_b(o, 0xc5, 3);
    cqi7_b(o, 0xc6, 1);
    cqi7_b(o, 0xc7, 0);
    cqi7_b(o, 0xbe, 4); // quatro sobem
    cqi7_b(o, 0xbf, 0);
    cqi7_b(o, 0xc0, 0);
    cqi7_b(o, 0xc1, 4); // quatro caem
    cqi7_d(o, 0x1c, *(0x9cf9ec as *const u32)); // sobe para a Série A
    cqi7_w(o, 0xd9, 2); // bit 0x2: as vagas de cima são de acesso
    cqi7_b(o, 0x52, 2);
    cqi7_b(o, 0x4a, 3);
    cqi7_d(o, 0x20, *(0x9cf9ac as *const u32)); // cai para a Série C
    cqi7_b(o, 0x49, 7);
}

/// As regras da Série C (0x42de50).
unsafe fn cqi7_regras_serie_c(o: usize) {
    cqi7_w(o, 0x3c, 2); // cqi7: os turnos da Série C
    cqi7_b(o, 0xc2, 3);
    cqi7_b(o, 0xc3, 1);
    cqi7_b(o, 0xc4, 2);
    cqi7_b(o, 0x42, 1);
    cqi7_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora
    cqi7_b(o, 0xc6, 2);
    cqi7_b(o, 0xc7, 4);
    cqi7_b(o, 0xbe, 2);
    cqi7_b(o, 0xbf, 0);
    cqi7_b(o, 0xc0, 0);
    cqi7_b(o, 0xc1, 2);
    cqi7_b(o, 0x4a, 3);
    cqi7_b(o, 0x52, 2);
    cqi7_d(o, 0x1c, *(0x9cf9e0 as *const u32)); // sobe para a Série B
    cqi7_d(o, 0x20, 0xffff_ffff); // nenhuma liga abaixo
    cqi7_w(o, 0xd9, 8);
    cqi7_b(o, 0x49, 7);
}

/// { 0x42a5a0 } (thiscall, bra_nat_first.cpp): `BraNatFirst::set_up_rules` (slot 36), as regras da Série A e o
/// calendário da fase principal; devolve 1.
unsafe extern "thiscall" fn cqi7_regras_da_serie_a(this: usize) -> u32 {
    cqi7_regras_serie_a(this);
    crate::comparacao::ramo("cqi_7 regras da serie a");
    cqi7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto inteiro
/// menos as saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqi7_regras_da_serie_a(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1712)(this), || {
        cqi7_regras_serie_a(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi7_regras_da_serie_a({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x42cb70 } (thiscall, bra_nat_second.cpp): `BraNatSecond::set_up_rules`, as regras da Série B e o calendário.
unsafe extern "thiscall" fn cqi7_regras_da_serie_b(this: usize) -> u32 {
    cqi7_regras_serie_b(this);
    crate::comparacao::ramo("cqi_7 regras da serie b");
    cqi7_calendario(this);
    1
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_cqi7_regras_da_serie_b(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1713)(this), || {
        cqi7_regras_serie_b(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi7_regras_da_serie_b({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x42de50 } (thiscall, bra_nat_third.cpp): `BraNatThird::set_up_rules`, as regras da Série C e o calendário.
unsafe extern "thiscall" fn cqi7_regras_da_serie_c(this: usize) -> u32 {
    cqi7_regras_serie_c(this);
    crate::comparacao::ramo("cqi_7 regras da serie c");
    cqi7_calendario(this);
    1
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_cqi7_regras_da_serie_c(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1714)(this), || {
        cqi7_regras_serie_c(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi7_regras_da_serie_c({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- 274a leva: as regras dos estaduais do Centro-Oeste, Gaúcho (Nordeste, Paulista), Baiano e Catarinense (bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_santa.cpp) ----------

unsafe fn cqi8_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqi8_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqi8_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// As regras comuns dos estaduais (as três rotinas gravam o mesmo conjunto; mudam os turnos +0x3c, as vagas de cima
/// +0xbe e os rebaixados +0xc1): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates confronto
/// direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb (copiadas em cada partida: +0x38 e +0x36) em 0.
unsafe fn cqi8_regras(o: usize, turnos: u16, cima: u8, rebaixados: u8) {
    cqi8_w(o, 0xab, 0);
    cqi8_w(o, 0xdb, 0);
    cqi8_b(o, 0xc7, 0);
    cqi8_b(o, 0xbf, 0);
    cqi8_b(o, 0xc0, 0);
    cqi8_b(o, 0xc2, 3);
    cqi8_b(o, 0xc5, 3);
    cqi8_b(o, 0xc1, rebaixados);
    cqi8_b(o, 0x4a, 3);
    cqi8_b(o, 0xbe, cima);
    cqi8_w(o, 0xd9, 8);
    cqi8_w(o, 0x3c, turnos);
    cqi8_b(o, 0xc3, 1);
    cqi8_b(o, 0xc4, 2);
    cqi8_b(o, 0x42, 1);
    cqi8_b(o, 0xc6, 1);
    cqi8_d(o, 0x1c, 0xffff_ffff);
    cqi8_d(o, 0x20, 0xffff_ffff);
    cqi8_b(o, 0x49, 7);
}

/// O Centro-Oeste (0x430ff0): turno único, 8 vagas de cima (o mata-mata de 8), 3 rebaixados.
unsafe fn cqi8_regras_centro(o: usize) {
    cqi8_regras(o, 1, 8, 3); // cqi8: o Centro-Oeste
}

/// O Gaúcho (0x4322d0, também o Nordeste e o Paulista): turno único, 4 vagas de cima, 2 rebaixados.
unsafe fn cqi8_regras_gaucho(o: usize) {
    cqi8_regras(o, 1, 4, 2); // cqi8: o Gaúcho
}

/// O Catarinense (0x43bc70, também o Baiano): dois turnos, 4 vagas de cima, 1 rebaixado.
unsafe fn cqi8_regras_catarinense(o: usize) {
    cqi8_regras(o, 2, 4, 1); // cqi8: o Catarinense
}

/// O calendário da fase principal pelo slot 16 (`create_schedule`, thiscall, ret 0x10, conferido nos seis alvos das
/// vtables destas rotinas): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqi8_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqi8_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os calendários não gravam nada
/// no objeto além delas (nenhum `mov [edi+..]` nos seis).
fn cqi8_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// { 0x430ff0 } (thiscall, bra_reg_central.cpp): `BraRegCentral::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    cqi8_regras_centro(this);
    crate::comparacao::ramo("cqi_8 regras do centro-oeste");
    cqi8_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1715)(this), || {
        cqi8_regras_centro(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_centro_oeste({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x4322d0 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::set_up_rules`, nas vtables do Gaúcho (0x96811c), do
/// Nordeste (0x9683ec) e do Paulista (0x968770); devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_gaucho(this: usize) -> u32 {
    cqi8_regras_gaucho(this);
    crate::comparacao::ramo("cqi_8 regras do gaucho");
    cqi8_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_gaucho(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1716)(this), || {
        cqi8_regras_gaucho(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_gaucho({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x43bc70 } (thiscall, bra_reg_santa.cpp): `BraRegSanta::set_up_rules`, nas vtables do Baiano (0x967fb4) e do
/// Catarinense (0x9686bc); devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_catarinense(this: usize) -> u32 {
    cqi8_regras_catarinense(this);
    crate::comparacao::ramo("cqi_8 regras do catarinense");
    cqi8_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_catarinense(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1717)(this), || {
        cqi8_regras_catarinense(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_catarinense({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- 274a leva: as regras dos estaduais Paranaense, Pernambucano (Norte), Carioca e Mineiro (bra_reg_parana.cpp, bra_reg_pern.cpp, bra_reg_rio.cpp) ----------

unsafe fn cqi9_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqi9_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqi9_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// As regras comuns destes estaduais (as três rotinas gravam o mesmo conjunto; mudam as vagas de cima +0xbe e os
/// rebaixados +0xc1; os três têm dois turnos): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates
/// confronto direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb em 0.
unsafe fn cqi9_regras(o: usize, cima: u8, rebaixados: u8) {
    cqi9_w(o, 0x3c, 2);
    cqi9_b(o, 0xc4, 2);
    cqi9_w(o, 0xab, 0);
    cqi9_w(o, 0xdb, 0);
    cqi9_b(o, 0xc7, 0);
    cqi9_b(o, 0xbe, cima);
    cqi9_b(o, 0xbf, 0);
    cqi9_b(o, 0xc0, 0);
    cqi9_b(o, 0xc2, 3);
    cqi9_b(o, 0xc5, 3);
    cqi9_b(o, 0x4a, 3);
    cqi9_b(o, 0xc3, 1);
    cqi9_b(o, 0x42, 1);
    cqi9_b(o, 0xc6, 1);
    cqi9_b(o, 0xc1, rebaixados);
    cqi9_d(o, 0x1c, 0xffff_ffff);
    cqi9_d(o, 0x20, 0xffff_ffff);
    cqi9_b(o, 0x49, 7);
    cqi9_w(o, 0xd9, 8);
}

/// O Paranaense (0x437fc0): nenhuma vaga de cima na tabela geral (os turnos têm estágios próprios), 4 rebaixados.
unsafe fn cqi9_regras_parana(o: usize) {
    cqi9_regras(o, 0, 4); // cqi9: o Paranaense
}

/// O Pernambucano (0x439430, também o Norte): 4 vagas de cima, 2 rebaixados.
unsafe fn cqi9_regras_pernambucano(o: usize) {
    cqi9_regras(o, 4, 2); // cqi9: o Pernambucano
}

/// O Carioca (0x43a600, também o Mineiro): nenhuma vaga de cima na tabela geral, 2 rebaixados.
unsafe fn cqi9_regras_carioca(o: usize) {
    cqi9_regras(o, 0, 2); // cqi9: o Carioca
}

/// O calendário da fase principal pelo slot 16 (`create_schedule`, thiscall, ret 0x10, conferido nos cinco alvos das
/// vtables destas rotinas): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqi9_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqi9_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os calendários não gravam nada
/// no objeto além delas (nenhum `mov [edi+..]` nos cinco).
fn cqi9_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// { 0x437fc0 } (thiscall, bra_reg_parana.cpp): `BraRegParana::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
unsafe extern "thiscall" fn cqi9_regras_do_paranaense(this: usize) -> u32 {
    cqi9_regras_parana(this);
    crate::comparacao::ramo("cqi_9 regras do paranaense");
    cqi9_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqi9_regras_do_paranaense(this: usize) -> u32 {
    let reg = cqi9_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1718)(this), || {
        cqi9_regras_parana(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi9_regras_do_paranaense({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x439430 } (thiscall, bra_reg_pern.cpp): `BraRegPern::set_up_rules`, nas vtables do Norte (0x968338) e do
/// Pernambucano (0x968554); devolve 1.
unsafe extern "thiscall" fn cqi9_regras_do_pernambucano(this: usize) -> u32 {
    cqi9_regras_pernambucano(this);
    crate::comparacao::ramo("cqi_9 regras do pernambucano");
    cqi9_calendario(this);
    1
}

/// Comparador: como o do Paranaense.
unsafe extern "thiscall" fn comparar_cqi9_regras_do_pernambucano(this: usize) -> u32 {
    let reg = cqi9_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1719)(this), || {
        cqi9_regras_pernambucano(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi9_regras_do_pernambucano({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x43a600 } (thiscall, bra_reg_rio.cpp): `BraRegRio::set_up_rules`, nas vtables do Mineiro (0x968284) e do
/// Carioca (0x968608); devolve 1.
unsafe extern "thiscall" fn cqi9_regras_do_carioca(this: usize) -> u32 {
    cqi9_regras_carioca(this);
    crate::comparacao::ramo("cqi_9 regras do carioca");
    cqi9_calendario(this);
    1
}

/// Comparador: como o do Paranaense.
unsafe extern "thiscall" fn comparar_cqi9_regras_do_carioca(this: usize) -> u32 {
    let reg = cqi9_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1720)(this), || {
        cqi9_regras_carioca(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi9_regras_do_carioca({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- 274a leva: as regras do Goiano e das duas divisões da J-League (bra_reg_goias.cpp, jap_j1.cpp, jap_j2.cpp) ----------

unsafe fn cqj7_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqj7_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqj7_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x432e50, 0x662e30, 0x664780): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqj7_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqj7_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os três calendários não gravam
/// nada no objeto além delas (nenhum `mov [edi+..]`).
fn cqj7_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// As regras do Goiano (o que a 0x433640 grava antes do calendário).
unsafe fn cqj7_regras_goiano(o: usize) {
    cqj7_w(o, 0xab, 0);
    cqj7_w(o, 0xdb, 0);
    cqj7_b(o, 0xc7, 0);
    cqj7_b(o, 0xbf, 0);
    cqj7_b(o, 0xc0, 0);
    cqj7_b(o, 0xc2, 3);
    cqj7_b(o, 0xc5, 3); // cqj7: o primeiro desempate do Goiano
    cqj7_b(o, 0x4a, 3);
    cqj7_b(o, 0xbe, 8); // os 8 classificados
    cqj7_w(o, 0xd9, 8);
    cqj7_w(o, 0x3c, 1);
    cqj7_b(o, 0xc3, 1);
    cqj7_b(o, 0xc4, 2);
    cqj7_b(o, 0x42, 1);
    cqj7_b(o, 0xc6, 1);
    cqj7_b(o, 0xc1, 4);
    cqj7_d(o, 0x1c, 0xffff_ffff);
    cqj7_d(o, 0x20, 0xffff_ffff);
    cqj7_b(o, 0x49, 7);
}

/// As regras da J1 (0x663620).
unsafe fn cqj7_regras_j1(o: usize) {
    cqj7_w(o, 0x3c, 2);
    cqj7_b(o, 0xc2, 3);
    cqj7_b(o, 0xc3, 1); // cqj7: os pontos por empate da J1
    cqj7_b(o, 0xc4, 2);
    cqj7_w(o, 0xab, 0x82);
    cqj7_w(o, 0xdb, 0);
    cqj7_b(o, 0x42, 1);
    cqj7_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora, nenhum
    cqj7_b(o, 0xc6, 2);
    cqj7_b(o, 0xc7, 4);
    cqj7_b(o, 0xc8, 0);
    cqj7_b(o, 0xbe, 0);
    cqj7_b(o, 0xbf, 0);
    cqj7_b(o, 0xc0, 0);
    cqj7_b(o, 0xc1, 2); // dois caem para a J2
    cqj7_d(o, 0x1c, 0xffff_ffff);
    cqj7_b(o, 0x52, 2);
    cqj7_b(o, 0x4a, 3);
    cqj7_d(o, 0x20, *(0x9cf768 as *const u32)); // cai para a J2
    cqj7_w(o, 0xd9, 8);
    cqj7_b(o, 0x49, 5);
}

/// As regras da J2 (0x665140): não grava +0x52 nem a palavra +0xd9 (fica a de `League::League`).
unsafe fn cqj7_regras_j2(o: usize) {
    cqj7_w(o, 0x3c, 4); // quatro turnos
    cqj7_b(o, 0xc2, 3); // cqj7: os pontos por vitória da J2
    cqj7_b(o, 0xc3, 1);
    cqj7_b(o, 0xc4, 2);
    cqj7_w(o, 0xab, 0x82);
    cqj7_w(o, 0xdb, 0);
    cqj7_b(o, 0x42, 1);
    cqj7_b(o, 0xc5, 1);
    cqj7_b(o, 0xc6, 2);
    cqj7_b(o, 0xc7, 4);
    cqj7_b(o, 0xc8, 0);
    cqj7_b(o, 0xbe, 2); // dois sobem para a J1
    cqj7_b(o, 0xbf, 0);
    cqj7_b(o, 0xc0, 0);
    cqj7_b(o, 0xc1, 0);
    cqj7_d(o, 0x1c, *(0x9cf65c as *const u32)); // sobe para a J1
    cqj7_b(o, 0x4a, 3);
    cqj7_d(o, 0x20, 0xffff_ffff);
    cqj7_b(o, 0x49, 5);
}

/// { 0x433640 } (thiscall, bra_reg_goias.cpp): `BraRegGoias::set_up_rules` (slot 36), as regras do Goiano e o
/// calendário; devolve 1.
unsafe extern "thiscall" fn cqj7_regras_do_goiano(this: usize) -> u32 {
    cqj7_regras_goiano(this);
    crate::comparacao::ramo("cqj_7 regras do goiano");
    cqj7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqj7_regras_do_goiano(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1721)(this), || {
        cqj7_regras_goiano(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_do_goiano({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x663620 } (thiscall, jap_j1.cpp): `JapJ1::set_up_rules`, as regras da J1 e o calendário; devolve 1.
unsafe extern "thiscall" fn cqj7_regras_da_j1(this: usize) -> u32 {
    cqj7_regras_j1(this);
    crate::comparacao::ramo("cqj_7 regras da j1");
    cqj7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
unsafe extern "thiscall" fn comparar_cqj7_regras_da_j1(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1722)(this), || {
        cqj7_regras_j1(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_da_j1({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x665140 } (thiscall, jap_j2.cpp): `JapJ2::set_up_rules`, as regras da J2 e o calendário; devolve 1.
unsafe extern "thiscall" fn cqj7_regras_da_j2(this: usize) -> u32 {
    cqj7_regras_j2(this);
    crate::comparacao::ramo("cqj_7 regras da j2");
    cqj7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
unsafe extern "thiscall" fn comparar_cqj7_regras_da_j2(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1723)(this), || {
        cqj7_regras_j2(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj7_regras_da_j2({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- 274a leva: as regras da NSL australiana e da MLS, e o histórico da temporada dos estaduais e das Séries A e B (aus_nsl.cpp, usa_mls.cpp, bra_reg_gaucho.cpp, arg_second.cpp) ----------

unsafe fn cqj8_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqj8_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqj8_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// As regras da NSL (o que a 0x411ce0 grava antes do calendário).
unsafe fn cqj8_regras_nsl(o: usize) {
    cqj8_w(o, 0xab, 0);
    cqj8_b(o, 0xc8, 0);
    cqj8_b(o, 0xbf, 0);
    cqj8_b(o, 0xc0, 0);
    cqj8_b(o, 0xc1, 0);
    cqj8_b(o, 0xc4, 2);
    cqj8_b(o, 0xc6, 2);
    cqj8_b(o, 0x52, 2);
    cqj8_w(o, 0x3c, 3); // três turnos
    cqj8_b(o, 0xc2, 3); // cqj8: os pontos por vitória da NSL
    cqj8_b(o, 0x4a, 3);
    cqj8_b(o, 0xc3, 1);
    cqj8_b(o, 0x42, 1);
    cqj8_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora
    cqj8_b(o, 0xc7, 4);
    cqj8_b(o, 0xbe, 6); // os seis da fase final
    cqj8_d(o, 0x1c, 0xffff_ffff);
    cqj8_d(o, 0x20, 0xffff_ffff);
    cqj8_w(o, 0xd9, 0x28); // 0x8 fase seguinte, 0x20 eliminados marcados
    cqj8_b(o, 0x49, 4);
}

/// As regras da MLS (0x908660; sem calendário: os jogos da MLS são montados à mão pela 0x9086e0).
unsafe fn cqj8_regras_mls(o: usize) {
    cqj8_b(o, 0xc4, 2);
    cqj8_b(o, 0xc6, 2);
    cqj8_b(o, 0xc2, 3);
    cqj8_b(o, 0xc8, 0);
    cqj8_b(o, 0xbf, 0);
    cqj8_b(o, 0xc0, 0);
    cqj8_b(o, 0xc1, 0);
    cqj8_b(o, 0xc3, 1); // cqj8: os pontos por empate da MLS
    cqj8_w(o, 0xab, 0xc2);
    cqj8_b(o, 0x42, 1);
    cqj8_b(o, 0xc5, 1); // desempates: saldo, gols pró, confronto direto
    cqj8_b(o, 0xc7, 3);
    cqj8_b(o, 0xbe, 1);
    cqj8_d(o, 0x1c, 0xffff_ffff);
    cqj8_d(o, 0x20, 0xffff_ffff);
    cqj8_w(o, 0xd9, 0x48); // 0x8 fase seguinte, 0x40 rodadas em ordem fixa
    cqj8_w(o, 0x3c, 4);
    cqj8_w(o, 0xa7, 0x20); // 32 jogos por time
}

/// { 0x411ce0 } (thiscall, aus_nsl.cpp): `AusNsl::set_up_rules` (slot 36), as regras da NSL e o calendário da fase
/// principal pelo slot 16 (`create_schedule` 0x4111a0, thiscall, ret 0x10: estágio -1, as saídas +0xa9 e +0x3a, sem
/// a marca; o ponteiro vai para +0xba); devolve 1.
unsafe extern "thiscall" fn cqj8_regras_da_nsl(this: usize) -> u32 {
    cqj8_regras_nsl(this);
    crate::comparacao::ramo("cqj_8 regras da nsl");
    let vt = *(this as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(this, -1, this + 0xa9, this + 0x3a, 0);
    cqj8_d(this, 0xba, r as u32);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto da liga
/// (0xee) menos as saídas do calendário (+0x3a, +0xa9, +0xba; a 0x4111a0 não grava nada no objeto além delas); a
/// devolução.
unsafe extern "thiscall" fn comparar_cqj8_regras_da_nsl(this: usize) -> u32 {
    let o = this;
    let reg = [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1724)(this), || {
        cqj8_regras_nsl(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj8_regras_da_nsl({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x908660 } (thiscall, usa_mls.cpp): `UsaMls::set_up_rules`, as regras da MLS; devolve 1 (o eax que ela usa como
/// valor).
unsafe extern "thiscall" fn cqj8_regras_da_mls(this: usize) -> u32 {
    cqj8_regras_mls(this);
    crate::comparacao::ramo("cqj_8 regras da mls");
    1
}

/// Comparador: lado a lado sobre o objeto da liga (0xee; só grava nele); a devolução.
unsafe extern "thiscall" fn comparar_cqj8_regras_da_mls(this: usize) -> u32 {
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xee)], || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1725)(this), || cqj8_regras_da_mls(this));
    PLACAR.registrar(igual && r == n, || format!("cqj8_regras_da_mls({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// O histórico do primeiro estágio: o slot 13 (+0x30, `record_season_honours`, thiscall sem argumentos) do estágio
/// [[this+0xc]].
unsafe fn cqj8_historico_do_estagio(this: usize) -> u32 {
    let e = *(*((this + 0xc) as *const usize) as *const usize);
    let vt = *(e as *const usize);
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(*((vt + 0x30) as *const usize));
    f(e)
}

/// { 0x432970 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::record_season_honours` (slot 13 de nove estaduais: Baiano,
/// Centro-Oeste, Gaúcho, Goiano, Norte, Nordeste, Pernambucano, Catarinense e Paulista): o histórico do primeiro estágio
/// e, em salto final, a 0x42bba0 (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze
/// estaduais terminam, põe os campeões na Copa dos Campeões); devolve o eax dela.
unsafe extern "thiscall" fn cqj8_historico_do_estadual(this: usize) -> u32 {
    cqj8_historico_do_estagio(this); // cqj8: o histórico do estadual
    crate::comparacao::ramo("cqj_8 historico do estadual");
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    campeoes()
}

/// Comparador: só a original (ponytail: grava o histórico, aloca o registro dele e pode montar a Copa dos Campeões; não
/// pode rodar duas vezes). A cópia roda no modo Rust (contador "cqj_8 historico do estadual") e é provada pelas
/// mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqj8_historico_do_estadual(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1726)(this)
}

/// { 0x408eb0 } (thiscall, arg_second.cpp): `ArgSecond::record_season_honours` (slot 13 da segunda divisão argentina e
/// das Séries A e B): só o histórico do primeiro estágio, em salto final; devolve o eax dele.
unsafe extern "thiscall" fn cqj8_historico_do_primeiro_estagio(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_8 historico do primeiro estagio");
    cqj8_historico_do_estagio(this)
}

/// Comparador: só a original (ponytail: grava o histórico e aloca o registro; não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_cqj8_historico_do_primeiro_estagio(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1727)(this)
}

// ---------- 274a leva: o destino de cada clube no fim das fases da Série A, da Série C e do Baiano (bra_nat_first.cpp, bra_nat_third.cpp, bra_reg_bahia.cpp) ----------

unsafe fn cqj9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqj9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, todos já em Rust):
/// 0x86bdd0 classificou-se (clube, comp, fase, rodada, peso; ret 0x14).
unsafe fn cqj9_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cqj9_d(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86a1c0 rebaixado (clube, liga; ret 8).
unsafe fn cqj9_rebaixado(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
    f(cqj9_d(0xdd7f00), clube, comp);
}

/// 0x868c50 campeão da liga (clube, liga; ret 8).
unsafe fn cqj9_campeao(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
    f(cqj9_d(0xdd7f00), clube, comp);
}

/// 0x869480 promovido (clube, liga, peso; ret 0xc).
unsafe fn cqj9_promovido(clube: usize, comp: usize, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
    f(cqj9_d(0xdd7f00), clube, comp, peso);
}

/// 0x68ab70 `League::find_club_row(clube, 0)` (thiscall, ret 8): a linha do clube na tabela da liga.
unsafe fn cqj9_linha(liga: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x68ab70usize);
    f(liga, clube, 0)
}

/// A Série A (0x42abc0). `fazer` = falso: o ensaio do comparador, sem os registros nem a gravação; devolve
/// (devolução, linha gravada ou 0, destino gravado).
unsafe fn cqj9_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = cqj9_d(this + 4);
    if fa == 0xff {
        // a fase de pontos corridos
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqj_9 serie a classificado");
                cqj9_classificou(clube, comp, 0x41c, 0, 0x28);
            }
        } else if st == 3 && fazer {
            cqj9_rebaixado(clube, comp);
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    // o mata-mata
    if st == 1 {
        let l = cqj9_linha(this, clube);
        if fazer {
            crate::comparacao::ramo("cqj_9 serie a campeao");
            cqj9_campeao(clube, comp); // cqj9: o registro do campeão da Série A
            *((l + 0x37) as *mut u8) = 5;
        }
        return (0, l, 5);
    }
    if st == 2 || st == 0xff {
        let l = cqj9_linha(this, clube);
        if fazer {
            *((l + 0x37) as *mut u8) = 0xfe; // eliminado
        }
        return (0, l, 0xfe);
    }
    (0, 0, 0)
}

/// A Série C (0x42eb00); como a da Série A.
unsafe fn cqj9_serie_c(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    let comp = cqj9_d(this + 4);
    if fa < 7 {
        // os grupos
        if st == 6 && fazer {
            cqj9_classificou(clube, comp, 0x14, 0, 0x28);
        }
        return (0, 0, 0);
    }
    if fa == 7 {
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_classificou(clube, comp, 0, 0x1e, 0x28);
        }
        return (0, 0, 0);
    }
    if fa < 10 {
        // o quadrangular final: os dois que sobem
        if st != 0 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_promovido(clube, comp, 0x32);
            cqj9_classificou(clube, comp, 0, 0x96, 0x28);
        }
        // a linha do clube na liga (k = -1) ou nos estágios 0..6, a primeira que tiver o clube, ganha o destino 0
        let mut k: i8 = -1;
        let mut achada = 0;
        while k < 7 {
            let liga = if k == -1 { this } else { cqj9_d(cqj9_d(this + 0xc) + 4 * k as i32 as usize) };
            let n = cqj9_w(liga + 0x3e);
            if n > 0 {
                let t = cqj9_d(liga + 0xb1);
                let mut i: i8 = 0;
                while (i as i16) < n {
                    let r = t.wrapping_add(0x3b * i as i32 as usize);
                    if cqj9_d(r) == clube {
                        achada = r;
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if achada != 0 {
                    break;
                }
            }
            k += 1;
        }
        if achada != 0 && fazer {
            crate::comparacao::ramo("cqj_9 serie c promovido");
            *((achada + 0x37) as *mut u8) = 0; // cqj9: o destino de quem sobe da Série C
        }
        return (0, achada, 0);
    }
    if fa == 10 {
        // a final
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_campeao(clube, comp);
        }
    }
    (0, 0, 0)
}

/// O Baiano (0x430210); como a da Série A. O clube vem da tabela (+0xb1, +0x3e linhas), não da 0x68ab70.
unsafe fn cqj9_baiano(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = cqj9_d(this + 4);
    if fa == 0xff {
        if st == 6 {
            if fazer {
                cqj9_classificou(clube, comp, 0, 0x82, 0x32);
            }
        } else if st == 3 && fazer {
            crate::comparacao::ramo("cqj_9 baiano rebaixado");
            cqj9_rebaixado(clube, comp); // cqj9: o rebaixado do Baiano
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let n = cqj9_w(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if cqj9_d(cqj9_d(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    cqj9_campeao(clube, comp);
                }
                let l = cqj9_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (0, l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // ponytail: a original empurra os registradores inteiros (eax com o lixo do chamador acima da
                    // palavra, ecx com a metade de cima do ponteiro da tabela); a 0x4bf850 e a 0x86c000 só leem as
                    // palavras (0x4b0870 / `as u16`): vão as palavras.
                    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
                    let peso = escala(0x1e, 0x32, cqj9_w(jogo + 0x34) as u16 as u32, 2);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                    elim(cqj9_d(0xdd7f00), clube, comp, cqj9_w(jogo + 0x32) as u16 as u32, cqj9_w(jogo + 0x30) as u16 as u32, peso);
                }
                let l = cqj9_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (0, l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0, 0)
}

/// { 0x42abc0 } (thiscall, ret 0x18, bra_nat_first.cpp): `BraNatFirst::update_staff_records_for_status` (slot 18):
/// na fase de pontos corridos, os 8 classificados (situação 6) ganham o registro de classificação e os rebaixados (3) o
/// de rebaixamento; no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5, os eliminados (2 ou
/// -1) o destino -2. Devolve 0.
unsafe extern "thiscall" fn cqj9_destino_na_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqj9_serie_a(this, clube, situacao, fase, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação): a devolução e o destino gravado na linha.
unsafe extern "thiscall" fn comparar_cqj9_destino_na_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_serie_a(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(1728)(this, clube, situacao, fase, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqj9_destino_na_serie_a({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x42eb00 } (thiscall, ret 0x18, bra_nat_third.cpp): `BraNatThird::update_staff_records_for_status`: nos grupos
/// (fase < 7), os classificados (6) ganham o registro de classificação; na fase 7, quem passa (1) também (os outros: -1);
/// no quadrangular (fases 8 e 9), os que sobem (0) ganham os registros de acesso e de classificação e o destino 0 na
/// primeira tabela que os tiver (a liga ou os estágios 0..6); na final (10), o campeão (1) ganha o registro de campeão.
/// Devolve 0, ou -1 nas situações que a fase não espera.
unsafe extern "thiscall" fn cqj9_destino_na_serie_c(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqj9_serie_c(this, clube, situacao, fase, true).0
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_cqj9_destino_na_serie_c(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_serie_c(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(1729)(this, clube, situacao, fase, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqj9_destino_na_serie_c({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x430210 } (thiscall, ret 0x18, bra_reg_bahia.cpp): `BraRegBahia::update_staff_records_for_status` (o slot 18 de
/// sete estaduais: Baiano, Gaúcho, Norte, Nordeste, Pernambucano, Catarinense e Paulista): na fase de pontos corridos, os
/// classificados (6) ganham o registro de classificação (rodada 0x82, peso 0x32) e os rebaixados (3) o de rebaixamento;
/// no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5; os eliminados (2 ou -1), o registro
/// de eliminação com o peso da rodada (0x4bf850: de 30 a 50 pela rodada do jogo) e o destino -2. Devolve 0.
unsafe extern "thiscall" fn cqj9_destino_no_baiano(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqj9_baiano(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_cqj9_destino_no_baiano(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_baiano(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1730)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqj9_destino_no_baiano({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

// ---------- 274a leva: o destino de cada clube no fim das fases do Centro-Oeste e do Goiano, do Mineiro, Paranaense e Carioca, e da NSL (bra_reg_central.cpp, bra_reg_minas_gerais.cpp, aus_nsl.cpp) ----------

unsafe fn cqk7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, já em Rust): 0x86bdd0 classificou-se (clube, comp,
/// fase, rodada, peso; ret 0x14).
unsafe fn cqk7_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cqk7_d(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86a1c0 rebaixado (clube, liga; ret 8).
unsafe fn cqk7_rebaixado(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
    f(cqk7_d(0xdd7f00), clube, comp);
}

/// 0x868c50 campeão da liga (clube, liga; ret 8).
unsafe fn cqk7_campeao(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
    f(cqk7_d(0xdd7f00), clube, comp);
}

/// A fase de pontos corridos (fase -1) dos estaduais: os classificados (6) ganham o registro de classificação (rodada
/// 0x82, peso 0x32), os rebaixados (3) o de rebaixamento. Igual nas duas rotinas dos estaduais.
unsafe fn cqk7_pontos_corridos(this: usize, clube: usize, st: u8, fazer: bool) {
    let comp = cqk7_d(this + 4);
    if st == 6 {
        if fazer {
            crate::comparacao::ramo("cqk_7 classificado do estadual");
            cqk7_classificou(clube, comp, 0, 0x82, 0x32); // cqk7: o classificado do estadual
        }
    } else if st == 3 && fazer {
        crate::comparacao::ramo("cqk_7 rebaixado do estadual");
        cqk7_rebaixado(clube, comp); // cqk7: o rebaixado do estadual
    }
}

/// O mata-mata dos estaduais: a linha do clube na tabela (+0xb1, +0x3e linhas); o campeão (1) ganha o registro de
/// campeão e o destino 5; os eliminados (2 ou -1), o registro de eliminação (0x86c000) com o peso da escala 0x4bf850
/// (30 a 50 pela rodada do jogo em `rodadas`) e o destino -2. Devolve (linha gravada ou 0, destino).
unsafe fn cqk7_mata_mata(this: usize, clube: usize, st: u8, jogo: usize, rodadas: u32, fazer: bool) -> (usize, u8) {
    let comp = cqk7_d(this + 4);
    let n = cqk7_w(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if cqk7_d(cqk7_d(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    crate::comparacao::ramo("cqk_7 campeao do estadual");
                    cqk7_campeao(clube, comp); // cqk7: o registro do campeão do estadual
                }
                let l = cqk7_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // ponytail: a original empurra eax (lixo do chamador acima da palavra) e ecx/edx inteiros; a
                    // 0x4bf850 e a 0x86c000 só leem as palavras: vão as palavras.
                    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
                    crate::comparacao::ramo("cqk_7 eliminado do estadual");
                    let peso = escala(0x1e, 0x32, cqk7_w(jogo + 0x34) as u16 as u32, rodadas);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                    elim(cqk7_d(0xdd7f00), clube, comp, cqk7_w(jogo + 0x32) as u16 as u32, cqk7_w(jogo + 0x30) as u16 as u32, peso); // cqk7: o registro do eliminado
                }
                let l = cqk7_d(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0)
}

/// O Centro-Oeste e o Goiano (0x4314e0): a fase -1 de pontos corridos e o mata-mata na fase 0 (escala de 3 rodadas).
/// `fazer` = falso: o ensaio do comparador. Devolve (devolução, linha gravada ou 0, destino).
unsafe fn cqk7_centro(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    if fa == 0xff {
        cqk7_pontos_corridos(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let (l, d) = cqk7_mata_mata(this, clube, st, jogo, 3, fazer);
    (0, l, d)
}

/// O Mineiro, o Paranaense e o Carioca (0x434d90): a fase -1 de pontos corridos; as fases 0 e 1 (os turnos) devolvem -1;
/// o mata-mata na fase 2 (escala de 1 rodada: a média, 40); as outras, 0.
unsafe fn cqk7_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    if fa == -1 {
        cqk7_pontos_corridos(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa < 2 {
        return (u32::MAX, 0, 0);
    }
    if fa != 2 {
        return (0, 0, 0);
    }
    let (l, d) = cqk7_mata_mata(this, clube, st, jogo, 1, fazer);
    (0, l, d)
}

/// A NSL (0x411980). Devolve (devolução, 0, 0): não grava na tabela.
unsafe fn cqk7_nsl(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = cqk7_d(this + 4);
    if fa == 0xff {
        // a temporada regular
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqk_7 nsl classificado");
                cqk7_classificou(clube, comp, 0x435, 0, 0x28); // cqk7: o classificado da NSL
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if fa == 0 {
        // a fase final
        if st == 6 {
            if fazer {
                cqk7_classificou(clube, comp, 0, 0x96, 0x28);
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            if fazer {
                let nao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
                nao(cqk7_d(0xdd7f00), clube, comp, 0x435, 0x28);
            }
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if st == 1 && fazer {
        cqk7_campeao(clube, comp);
    }
    (0, 0, 0)
}

/// { 0x4314e0 } (thiscall, ret 0x18, bra_reg_central.cpp): `BraRegCentral::update_staff_records_for_status` (slot 18
/// do Centro-Oeste e do Goiano): os classificados e rebaixados dos pontos corridos; no mata-mata (fase 0), o campeão
/// (destino 5) e os eliminados (destino -2, registro de eliminação com peso de 30 a 50 em 3 rodadas). Devolve 0.
unsafe extern "thiscall" fn cqk7_destino_no_centro_oeste(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqk7_centro(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e o destino na linha.
unsafe extern "thiscall" fn comparar_cqk7_destino_no_centro_oeste(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqk7_centro(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1731)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqk7_destino_no_centro_oeste({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x434d90 } (thiscall, ret 0x18, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::update_staff_records_for_status`
/// (slot 18 do Mineiro, do Paranaense e do Carioca): os pontos corridos como no Centro-Oeste; os turnos (fases 0 e 1)
/// devolvem -1; o mata-mata na fase 2 (peso pela escala de uma rodada só: 40). Devolve 0 ou -1.
unsafe extern "thiscall" fn cqk7_destino_no_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqk7_mineiro(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqk7_destino_no_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqk7_mineiro(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1732)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqk7_destino_no_mineiro({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// { 0x411980 } (thiscall, ret 0x18, aus_nsl.cpp): `AusNsl::update_staff_records_for_status`: na temporada regular, os
/// seis classificados (6) ganham o registro de classificação para a fase final (fase 0x435); os de fora (-2) devolvem
/// -1; na fase final (fase 0), quem avança (6) ganha o registro (rodada 0x96) e quem cai (-2) o de não classificado
/// (fase 0x435) e -1; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
unsafe extern "thiscall" fn cqk7_destino_na_nsl(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqk7_nsl(this, clube, situacao, fase, true).0
}

/// Comparador: só a original (os registros alocam) com a devolução prevista pelo ensaio.
unsafe extern "thiscall" fn comparar_cqk7_destino_na_nsl(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, _, _) = cqk7_nsl(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(1733)(this, clube, situacao, fase, a4, a5, a6);
    PLACAR.registrar(r == n, || format!("cqk7_destino_na_nsl({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}

// ---------- 274a leva: o progresso do clube nas estatísticas da competição no Mineiro (Paranaense, Carioca), na MLS e na Série C (bra_reg_minas_gerais.cpp, usa_mls.cpp, bra_nat_third.cpp) ----------

/// 0x4a4850 `Comp_Stats::find_club_entry(clube)` (thiscall, ret 4): a entrada do clube nas estatísticas da competição
/// ([this+8]); ordena a tabela pelo clube quando preciso (marca +0x18) e busca. Original (não portada).
unsafe fn cqk8_entrada(this: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    f(core::ptr::read_unaligned((this + 8) as *const u32) as usize, clube)
}

unsafe fn cqk8_gravar(e: usize, atual: u8, minima: u8, maxima: u8) {
    *((e + 0x73) as *mut u8) = atual;
    *((e + 0x74) as *mut u8) = minima;
    *((e + 0x75) as *mut u8) = maxima;
}

/// { 0x4350d0 } (thiscall, ret 0x14, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::set_club_progress(clube, fase, a,
/// b, c)` (slot 27 do Mineiro, do Paranaense e do Carioca): grava os três códigos de progresso do clube na entrada dele
/// (+0x73, +0x74, +0x75); na fase de pontos corridos (-1), o segundo abaixo de 3 vira 1 e o terceiro abaixo de 3 vira 2.
/// Devolve a entrada.
unsafe extern "thiscall" fn cqk8_progresso_no_mineiro(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (a, mut b, c) = (a as u8, b as u8, c as u8);
    if fase as u8 == 0xff {
        if b < 3 {
            b = 1;
        }
        if c < 3 {
            crate::comparacao::ramo("cqk_8 mineiro terceiro abaixo de 3");
            cqk8_gravar(e, a, b, 2); // cqk8: o terceiro código do Mineiro
            return e;
        }
    }
    crate::comparacao::ramo("cqk_8 mineiro como vem");
    cqk8_gravar(e, a, b, c); // cqk8: os códigos do Mineiro como vêm
    e
}

/// Comparador: lado a lado pela entrada do clube (+0x73..+0x75), achada antes (a 0x4a4850 ordena na primeira vez e só
/// busca depois; a original e a cópia a chamam de novo); a devolução.
unsafe extern "thiscall" fn comparar_cqk8_progresso_no_mineiro(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(1734)(this, clube, fase, a, b, c), || cqk8_progresso_no_mineiro(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("cqk8_progresso_no_mineiro({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// O código da MLS na temporada regular: 1..=3 viram `ate_3`, 4 vira 10, os outros ficam.
fn cqk8_mls(x: u8, ate_3: u8) -> u8 {
    match x {
        1..=3 => ate_3,
        4 => 10,
        _ => x,
    }
}

/// { 0x909dd0 } (thiscall, ret 0x14, usa_mls.cpp): `UsaMls::set_club_progress(clube, fase, a, b, c)`: nas fases 0 e 1
/// e na temporada (-1), a: 1 → 1, 2 → 3, 3 → 7, 4 → 10; b: 1..3 → 1, 4 → 10; c: 1..3 → 7, 4 → 10; na fase 2, cada
/// código vira 2 × código − 1; depois, os códigos como vêm. Devolve a entrada.
unsafe extern "thiscall" fn cqk8_progresso_na_mls(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 2 {
        let (b, c) = (cqk8_mls(b, 1), cqk8_mls(c, 7));
        let a = match a {
            1 => 1,
            2 => {
                crate::comparacao::ramo("cqk_8 mls segundo");
                3 // cqk8: o código 2 da MLS
            }
            3 => 7,
            4 => 10,
            _ => a,
        };
        cqk8_gravar(e, a, b, c);
    } else if fa == 2 {
        cqk8_gravar(e, a.wrapping_mul(2).wrapping_sub(1), b.wrapping_mul(2).wrapping_sub(1), c.wrapping_mul(2).wrapping_sub(1));
    } else {
        cqk8_gravar(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
unsafe extern "thiscall" fn comparar_cqk8_progresso_na_mls(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(1735)(this, clube, fase, a, b, c), || cqk8_progresso_na_mls(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("cqk8_progresso_na_mls({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// O código da Série C nos grupos (fases até 6): 1 e 2 viram `um_dois`, 3 → 0x11, 4 → 0x19, 5 → 0x21, 6 → 0x29,
/// 7 → 0x31 (a posição no grupo levada para a escala de 49 lugares [inferência]); os outros ficam.
fn cqk8_serie_c(x: u8, um_dois: u8) -> u8 {
    match x {
        1 | 2 => um_dois,
        3 => 0x11,
        4 => 0x19,
        5 => 0x21,
        6 => 0x29,
        7 => 0x31,
        _ => x,
    }
}

/// { 0x42e100 } (thiscall, ret 0x14, bra_nat_third.cpp): `BraNatThird::set_club_progress(clube, fase, a, b, c)`: nos
/// grupos (fase < 7), a e b: 1, 2 → 1, 3 → 0x11 ... 7 → 0x31; c: 1, 2 → 9, depois igual; nas fases 7 em diante, os
/// códigos como vêm (o trecho das fases 1 a 3 que a original tem depois do teste `fase >= 7` nunca roda). Devolve a
/// entrada.
unsafe extern "thiscall" fn cqk8_progresso_na_serie_c(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (fa, a, b, c) = (fase as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 7 {
        let (a, c) = (cqk8_serie_c(a, 1), cqk8_serie_c(c, 9));
        let b2 = cqk8_serie_c(b, 1);
        if b == 3 {
            crate::comparacao::ramo("cqk_8 serie c segundo 3");
        }
        cqk8_gravar(e, a, b2, c); // cqk8: os códigos dos grupos da Série C
    } else {
        cqk8_gravar(e, a, b, c);
    }
    e
}

/// Comparador: como o do Mineiro.
unsafe extern "thiscall" fn comparar_cqk8_progresso_na_serie_c(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = cqk8_entrada(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(1736)(this, clube, fase, a, b, c), || cqk8_progresso_na_serie_c(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("cqk8_progresso_na_serie_c({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}
