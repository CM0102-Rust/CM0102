// ---------- %%LEVA%%a leva: as regras dos torneios de seleções: a Copa América, a Copa Africana de Nações e a eliminatória da Oceania (goldcup.cpp, african_nations.cpp, wc_oceania_league.cpp) ----------

/// Os campos de byte e palavra que a virtual +0x8c (set_up_rules) de um torneio de seleções grava na liga do grupo:
/// (deslocamento, tamanho 1 ou 2, valor). Os pontos da vitória (+0xc2) vêm à parte, para as mutações.
unsafe fn goldcup_write_rule_fields(this: usize, campos: &[(usize, u8, u32)], vitoria: u8) {
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
unsafe fn competition_date_word(this: usize, dia: u32, mes: u32, ano: i16, semana: u32) -> u16 {
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
unsafe fn check_set_up_rules(nome: &str, this: usize, campos: &[(usize, u8, u32)], data: (u32, u32, i16, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let palavra = competition_date_word(this, data.0, data.1, data.2, data.3);
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
unsafe extern "thiscall" fn goldcup_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da copa america");
    let vitoria = 3; // cqk10: a vitória nos grupos da Copa América
    goldcup_write_rule_fields(this, &CQK10_COPA_AMERICA, vitoria);
    let dia = 0x14; // cqk10: o dia da data da Copa América
    let palavra = competition_date_word(this, dia, 2, 0, 5);
    cqk10_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_goldcup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    check_set_up_rules("goldcup_set_up_rules", this, &CQK10_COPA_AMERICA, (0x14, 2, 0, 5), 0, || o(this))
}

const CQK10_AFRICANA: [(usize, u8, u32); 14] = [(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1),
    (0xc3, 1, 1), (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// { 0x401ab0 } (thiscall, ret 0, african_nations.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `AfricanNations::set_up_rules()`: os grupos da Copa Africana em turno único, 3/1 pontos, a categoria 0xa, os
/// desempates (1, 2, 3), dois classificados por grupo, seleções, 7 no banco e 3 substituições, a data em 5 do mês 9 do
/// ano anterior ao da competição num dia da semana 5 [inferência: o sorteio] e +0xb8 = −1.
unsafe extern "thiscall" fn african_nations_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da copa africana");
    let vitoria = 3; // cqk10: a vitória nos grupos da Copa Africana
    goldcup_write_rule_fields(this, &CQK10_AFRICANA, vitoria);
    let dia = 5; // cqk10: o dia da data da Copa Africana
    let palavra = competition_date_word(this, dia, 9, -1, 5);
    cqk10_fim(this, palavra, 0xffff)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_african_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    check_set_up_rules("african_nations_set_up_rules", this, &CQK10_AFRICANA, (5, 9, -1, 5), 0xffff, || o(this))
}

const CQK10_OCEANIA: [(usize, u8, u32); 14] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc6, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// { 0x91ecf0 } (thiscall, ret 0, wc_oceania_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcOceaniaLeague::set_up_rules()`: os grupos da eliminatória da Oceania em turno único, 3/1 pontos, a categoria 0xa,
/// os desempates (1, 2, 3), um classificado por grupo, seleções, 7 no banco e 3 substituições, começo em 15 do mês 0
/// do ano da competição num dia da semana 5.
unsafe extern "thiscall" fn goldcup_set_up_rules_91ecf0(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da oceania");
    goldcup_write_rule_fields(this, &CQK10_OCEANIA, 3);
    let dia = 0xf; // cqk10: o dia do começo na Oceania
    let palavra = competition_date_word(this, dia, 0, 0, 5);
    cqk10_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_goldcup_set_up_rules_91ecf0(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    check_set_up_rules("goldcup_set_up_rules_91ecf0", this, &CQK10_OCEANIA, (0xf, 0, 0, 5), 0, || o(this))
}
