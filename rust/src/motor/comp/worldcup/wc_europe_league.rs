use super::*;

/// Writes byte/word rule fields (offset, size, value) of a qualifier league.
///
/// [pt] Os campos de byte e palavra de uma eliminatória (a virtual +0x8c, set_up_rules, na liga do grupo): (deslocamento,
/// tamanho 1 ou 2, valor). Os pontos da vitória (+0xc2) e a categoria (+0xc4) vêm à parte, para as mutações.
pub(crate) unsafe fn wc_europe_league_write_rule_fields(this: usize, campos: &[(usize, u8, u32)]) {
    for &(o, n, v) in campos {
        if n == 1 {
            *((this + o) as *mut u8) = v as u8;
        } else {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        }
    }
}

/// First word of Date(day, month, competition year +0x40, weekday).
///
/// [pt] A primeira palavra da Date(dia, mês, ano da competição +0x40, dia da semana) (0x549f70: push semana; push ano; push
/// mês; push dia; lea ecx, local -> thiscall, ret 0x10).
pub(crate) unsafe fn comp_date_word(this: usize, day: u32, month: u32, week: u32) -> u16 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    // simplification: the year goes in a register with garbage in the high 16 bits; 0x549f70 reads it as a word
    data(d.as_mut_ptr(), day, month, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, week);
    core::ptr::read_unaligned(d.as_ptr() as *const u16)
}

/// Shared end of rule setup: +0xb6 = date word, +0xb8 = b8, round table (virtual +0x3c) at +0xba; returns 1.
///
/// [pt] O fim comum: +0xb6 = a palavra da data, +0xb8 = `b8`, e a tabela das rodadas da virtual +0x3c (push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
pub(crate) unsafe fn finish_rules_setup(this: usize, word: u16, b8: u16) -> u32 {
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, word);
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, b8);
    let vt = core::ptr::read_unaligned(this as *const usize);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = table(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// Comparer: runs the original and checks fields, +0xb6 date, +0xb8 and return 1.
///
/// [pt] O plano: os campos, +0xb6 (a data, calculada antes), +0xb8 e a devolução 1, lidos depois da original `o`.
pub(crate) unsafe fn check_qualifier_rules(name: &str, this: usize, campos: &[(usize, u8, u32)], data: (u32, u32, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let word = comp_date_word(this, data.0, data.1, data.2);
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
    PLACAR.record(r == 1 && k == 0 && b6 == word && lido8 == b8, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {}", name, this, r, &errados[..k], b6, word, lido8));
    r
}

pub(crate) const EUROPE_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2), (0xc5, 1, 1),
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
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_12 regras da europa");
    wc_europe_league_write_rule_fields(this, &EUROPE_RULE_FIELDS);
    *((this + league::WIN_POINTS) as *mut u8) = 3; // cqj12: a vitória nas eliminatórias europeias
    *((this + league::MATCH_CATEGORY) as *mut u8) = 0xa; // cqj12: a categoria dos jogos das eliminatórias europeias
    let day = 0x19; // cqj12: o dia do começo das eliminatórias europeias
    let word = comp_date_word(this, day, 2, 5);
    finish_rules_setup(this, word, 0)
}

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

pub(crate) const SOUTH_AMERICA_QUALIFIER_RULE_FIELDS: [(usize, u8, u32); 16] = [(0x3c, 2, 2), (0x43, 1, 2), (0xc6, 1, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 0xa), (0xab, 2, 0),
    (0xdb, 2, 0), (0xc5, 1, 1), (0xc7, 1, 3), (0xbf, 1, 1), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0x4a, 1, 3), (0xa7, 2, 0xffff)];


// simplification: virtual +0x3c allocates the round table: does not run twice. Original only; the plan checks the rules and
// the direct places (4 − +0xef − +0xee, with the +0xef the routine itself copies, computed before).

pub(crate) const ASIA_QUALIFIER_RULE_FIELDS: [(usize, u8, u32); 12] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2), (0xc5, 1, 1),
    (0xc6, 1, 2), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0)];


// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

