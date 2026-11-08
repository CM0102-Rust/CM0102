// ---------- %%LEVA%%a leva: as regras dos grupos continentais de clubes: a Libertadores, a Copa Merconorte e o Campeonato de Clubes da Oceania (conmebol_liber.cpp, con_merc_cup.cpp, oceania_club_champ.cpp) ----------

/// As regras de um grupo continental (a virtual +0x8c, set_up_rules, numa liga de grupo): os campos de byte e palavra
/// gravados (deslocamento, tamanho 1 ou 2, valor), a data de começo (0x549f70: dia, mês, ano da competição +0x40, dia da
/// semana; −1 = qualquer) e o fundo de prêmios (+0x28), quando há.
struct ContinentalGroupRules {
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
unsafe fn apply_cup_rules(this: usize, g: &ContinentalGroupRules, dia: u32, vitoria: u8) -> u32 {
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
unsafe fn check_cup_rules(nome: &str, this: usize, g: &ContinentalGroupRules, o: impl FnOnce() -> u32) -> u32 {
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
    PLACAR.record(r == 1 && k == 0 && b6 == palavra && b8 == 0 && premio, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {} prêmio ok {}", nome, this, r, &errados[..k], b6, palavra, b8, premio));
    r
}

const LIBERTADORES_RULES: ContinentalGroupRules = ContinentalGroupRules {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7),
        (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    dia: 0xf, mes: 0, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

const MERCONORTE_RULES: ContinentalGroupRules = ContinentalGroupRules {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1), (0xbe, 1, 1),
        (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff)],
    dia: 1, mes: 7, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

const OCEANIA_RULES: ContinentalGroupRules = ContinentalGroupRules {
    campos: &[(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x49, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 8),
        (0xc5, 1, 1), (0xc7, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xd9, 2, 0x28)],
    dia: 0x14, mes: 7, semana: 5, premio: None,
};

/// { 0x4c3890 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `ConmebolLiber::set_up_rules()`: os grupos da Libertadores em dois turnos, 3 pontos a vitória e 1 o empate, os
/// desempates (1, 2, 3, 0), dois classificados por grupo, clubes internacionais (+0x42 = 2), categoria 7, 7 no banco e 3
/// substituições, começo no dia 15 do mês 0 do ano da competição, prêmios de 2.000.000; depois a tabela das rodadas.
unsafe extern "thiscall" fn conmebol_liber_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_11 regras da libertadores");
    let dia = 0xf; // cqj11: o dia do começo da Libertadores
    let vitoria = 3; // cqj11: a vitória nos grupos da Libertadores
    apply_cup_rules(this, &LIBERTADORES_RULES, dia, vitoria)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_conmebol_liber_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    check_cup_rules("conmebol_liber_set_up_rules", this, &LIBERTADORES_RULES, || o(this))
}

/// { 0x4c1ed0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x8c; quadro SEH só da data local): `ConMercCup::
/// set_up_rules()`: os grupos da Copa Merconorte em dois turnos, 3/1 pontos, desempates (1, 2, 3, 0), um classificado
/// por grupo, clubes internacionais, categoria 7, começo no dia 1 do mês 7, prêmios de 2.000.000.
unsafe extern "thiscall" fn con_merc_cup_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_11 regras da merconorte");
    apply_cup_rules(this, &MERCONORTE_RULES, MERCONORTE_RULES.dia, 3)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_con_merc_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    check_cup_rules("con_merc_cup_set_up_rules", this, &MERCONORTE_RULES, || o(this))
}

/// { 0x7979b0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `OceaniaClubChamp::set_up_rules()`: os grupos do Campeonato de Clubes da Oceania em turno único, 3/1 pontos,
/// desempates (1, 2, 4, 0), dois classificados, categoria 8, 3 no banco e 3 substituições, +0x43 = 2, os bits de regra
/// 0x28 (os de cima classificados, os outros eliminados), começo no dia 20 do mês 7 num dia da semana 5; sem prêmio.
unsafe extern "thiscall" fn oceania_club_champ_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_11 regras da oceania");
    let dia = 0x14; // cqj11: o dia do começo na Oceania
    apply_cup_rules(this, &OCEANIA_RULES, dia, 3)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_oceania_club_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    check_cup_rules("oceania_club_champ_set_up_rules", this, &OCEANIA_RULES, || o(this))
}
