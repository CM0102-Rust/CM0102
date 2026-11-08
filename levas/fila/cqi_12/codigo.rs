// ---------- %%LEVA%%a leva: as regras da fase de grupos das copas: a Copa da Itália, a Copa da Itália da Série C e a Copa dos Campeões do Brasil (ita_cup.cpp, ita_c_cup.cpp, bra_champ_cup.cpp) ----------

/// As regras que a virtual +0x8c (set_up_rules) de uma copa com fase de grupos grava na liga do grupo, antes da tabela
/// das rodadas. Desempates: os quatro códigos de +0xc5..+0xc8 (docs/.md §2.7).
#[derive(Clone, Copy)]
struct GroupStageRules {
    vitoria: u8,     // +0xc2: pontos por vitória
    empate: u8,      // +0xc3: pontos por empate
    categoria: u8,   // +0xc4: a categoria de cada jogo (vai para o +0x3a da partida)
    desempate: [u8; 4], // +0xc5..+0xc8
    vagas: u8,       // +0xbe: vagas de cima (classificados por grupo)
    premio: Option<u32>, // +0x28: o fundo de prêmios (a Copa dos Campeões não grava)
    bits: Option<u16>,   // +0xd9: a palavra de bits de regra (só a Copa dos Campeões grava: 8)
}

const ITALIAN_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0xc3500), bits: None };
const SERIE_C_CUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0x493e0), bits: None };
const CHAMPIONS_CUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 0x17, desempate: [3, 1, 2, 4], vagas: 2, premio: None, bits: Some(8) };

/// Grava as regras `g` na liga `this`: sobe/cai para nenhuma (+0x1c, +0x20 = −1), um turno (+0x3c = 1), o tipo 1
/// (+0x42, clubes), sem play-offs nem rebaixamento (+0xbf, +0xc0, +0xc1 = 0); depois a tabela das rodadas da virtual
/// +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
unsafe fn set_cup_league_rules(this: usize, g: &GroupStageRules, turnos: i16) -> u32 {
    let b = |o: usize, v: u8| *((this + o) as *mut u8) = v;
    b(0xc2, g.vitoria);
    b(0xc3, g.empate);
    b(0xc4, g.categoria);
    for (k, &v) in g.desempate.iter().enumerate() {
        b(0xc5 + k, v);
    }
    b(0xbe, g.vagas);
    b(0xbf, 0);
    b(0xc0, 0);
    b(0xc1, 0);
    b(0x42, 1);
    core::ptr::write_unaligned((this + 0x1c) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x20) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x3c) as *mut i16, turnos);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + 0x28) as *mut u32, p);
    }
    if let Some(w) = g.bits {
        core::ptr::write_unaligned((this + 0xd9) as *mut u16, w);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// O plano: os campos que `set_cup_league_rules` grava, lidos depois da original.
unsafe fn check_cup_league_rules(nome: &str, this: usize, g: &GroupStageRules, r: u32) {
    let b = |o: usize| *((this + o) as *const u8);
    let lido = (b(0xc2), b(0xc3), b(0xc4), [b(0xc5), b(0xc6), b(0xc7), b(0xc8)], b(0xbe), [b(0xbf), b(0xc0), b(0xc1)], b(0x42),
        core::ptr::read_unaligned((this + 0x1c) as *const i32), core::ptr::read_unaligned((this + 0x20) as *const i32),
        core::ptr::read_unaligned((this + 0x3c) as *const i16));
    let previsto = (g.vitoria, g.empate, g.categoria, g.desempate, g.vagas, [0u8; 3], 1u8, -1i32, -1i32, 1i16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + 0x28) as *const u32) == p);
    let bits = g.bits.map_or(true, |w| core::ptr::read_unaligned((this + 0xd9) as *const u16) == w);
    PLACAR.registrar(r == 1 && lido == previsto && premio && bits, || format!("{}({:#x}) devolveu {} regras {:?} previstas {:?} prêmio ok {} bits ok {}", nome, this, r, lido, previsto, premio, bits));
}

/// { 0x63f350 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x8c): `ItaCup::set_up_rules()`: os grupos da Copa da Itália
/// em turno único, 3 pontos a vitória e 1 o empate, a categoria 4 (copa nacional), o desempate (4, 1, 2), um
/// classificado por grupo e o fundo de prêmios de 800.000; depois a tabela das rodadas.
unsafe extern "thiscall" fn ita_cup_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa da italia");
    let mut g = ITALIAN_CUP_GROUP_RULES;
    g.vitoria = 3; // cqi12: a vitória nos grupos da Copa da Itália
    g.categoria = 4; // cqi12: a categoria dos jogos da Copa da Itália
    set_cup_league_rules(this, &g, 1)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_ita_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    check_cup_league_rules("ita_cup_set_up_rules", this, &ITALIAN_CUP_GROUP_RULES, r);
    r
}

/// { 0x63d6e0 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x8c): `ItaCCup::set_up_rules()`: os grupos da Copa da
/// Itália da Série C, com as mesmas regras da Copa da Itália e o fundo de prêmios de 300.000.
unsafe extern "thiscall" fn ita_cup_set_up_rules_63d6e0(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa da serie c");
    let turnos = 1i16; // cqi12: o turno único dos grupos da Série C
    set_cup_league_rules(this, &SERIE_C_CUP_RULES, turnos)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_ita_cup_set_up_rules_63d6e0(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    check_cup_league_rules("ita_cup_set_up_rules_63d6e0", this, &SERIE_C_CUP_RULES, r);
    r
}

/// { 0x426230 } (thiscall, ret 0, bra_champ_cup.cpp; a virtual +0x8c): `BraChampCup::set_up_rules()`: o grupo da
/// Copa dos Campeões do Brasil em turno único, 3 pontos a vitória e 1 o empate, a categoria 0x17 (23, a das finais de
/// campeões), o desempate (3, 1, 2, 4), dois classificados e os bits de regra 8 (quem garante as vagas vai à fase
/// seguinte); sem fundo de prêmios. Depois a tabela das rodadas.
unsafe extern "thiscall" fn bra_champ_cup_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa dos campeoes");
    set_cup_league_rules(this, &CHAMPIONS_CUP_RULES, 1)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_bra_champ_cup_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    check_cup_league_rules("bra_champ_cup_set_up_rules", this, &CHAMPIONS_CUP_RULES, r);
    r
}
