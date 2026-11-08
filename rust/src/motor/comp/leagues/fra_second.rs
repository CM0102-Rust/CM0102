use super::*;

// ---------- 278a leva: os construtores da segunda e da terceira divisões francesas e da Primera e da Segunda espanholas (as regras, a tabela, os jogos, as estatísticas, as posições previstas e a divisão de baixo espanhola na criação) (fra_second.cpp, fra_third.cpp, spa_second.cpp, spa_first.cpp) ----------



/// O que muda de uma divisão para outra no construtor.
pub(crate) struct Cqm5Divisao {
    pub(crate) vtabela: usize,
    /// a regra de disciplina (+0x50): 0xc a francesa, 0x1a a espanhola
    pub(crate) disciplina: u8,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    pub(crate) arquivo: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    pub(crate) linhas: [u32; 5],
    /// a Primera cria no fim a divisão de baixo espanhola (+0xee)
    pub(crate) baixo: bool,
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para os operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a disciplina
/// (+0x50); tenta carregar o estado (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4).
/// Carregou: pronto. Senão, a liga nova, só a liga: fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c = 0, +0xc = 0; as
/// regras, a tabela, os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 ->
/// thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats 0x49ee70:
/// push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0);
/// cada passo que falha mostra a caixa da sua linha e encerra. Na Primera, por fim: se a nação da competição
/// ([+4]+0x5d) não tem o bit 4 de +0x11c [inferência: a divisão de baixo jogável], a divisão de baixo espanhola
/// (operator new de 0xee; SpaLower::SpaLower 0x851020: push ano; mov ecx, novo -> thiscall, ret 4) vai para +0xee
/// (0 sem memória); com o bit, +0xee = 0. Devolve this.
pub(crate) unsafe fn cqm5_construir(this: usize, ano: u32, registro: usize, p: &Cqm5Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_PREDICTED_POSITIONS);
    let baixo: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_SPALOWER_SPALOWER);
    base(this);
    write_u32_from_usize(this + 4, registro);
    write_u32_from_usize(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = p.disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_5 divisao nova");
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    write_u32_from_usize(this + 0x2c, 0);
    write_u32_from_usize(this + 0xc, 0);
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    write_u32_from_usize(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    if p.baixo {
        let nacao = read_u32_as_usize(read_u32_as_usize(this + 4) + 0x5d);
        if *((nacao + 0x11c) as *const u8) & 4 == 0 { // cqm5: a divisão de baixo espanhola
            crate::comparacao::ramo("cqm_5 divisao de baixo espanhola");
            let e = novo(0xee);
            // ponytail: a original empurra o ano na palavra de baixo do ecx, com o resto do ponteiro do registro em
            // cima; a 0x851020 só lê a palavra (mov ax, word ptr [esp + 0x21c]).
            let b = if e != 0 { baixo(e, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32) } else { 0 };
            write_u32_from_usize(this + 0xee, b);
        } else {
            write_u32_from_usize(this + 0xee, 0);
        }
    }
    this
}

pub(crate) const CQM5_FRANCA_2: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96ac2c,
    disciplina: 0xc, // cqm5: a regra de disciplina francesa
    regras: 0x5bf0a0,
    tabela: 0x5bef10,
    arquivo: 0x9ccc0c,
    linhas: [0x43, 0x4a, 0x51, 0x5a, 0x61],
    baixo: false,
};
pub(crate) const CQM5_FRANCA_3: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96ad80,
    disciplina: 0xc,
    regras: 0x5c0700,
    tabela: 0x5c05d0,
    arquivo: 0x9ccc80,
    linhas: [0x42, 0x49, 0x50, 0x59, 0x60],
    baixo: false,
};
pub(crate) const CQM5_SEGUNDA: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96fbd0,
    disciplina: 0x1a, // cqm5: a regra de disciplina espanhola
    regras: 0x855cc0,
    tabela: 0x855b40,
    arquivo: 0xa828bc,
    linhas: [0x46, 0x4d, 0x54, 0x5b, 0x62],
    baixo: false,
};
pub(crate) const CQM5_PRIMERA: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96fa68,
    disciplina: 0x1a,
    regras: 0x84fdf0,
    tabela: 0x84fc70,
    arquivo: 0xa82844,
    linhas: [0x47, 0x4e, 0x55, 0x5c, 0x63],
    baixo: true,
};

/// the French second division: discipline rule 0xc (the French one), only the league; with no saved state, the rules,
/// the table, the fixtures, the order, the stats and the predicted positions
/// Original: `FraSecond::FraSecond` (0x5be330, fra_second.cpp, thiscall, ret 8).
///
/// [pt] { 0x5be330 } (thiscall, ret 8, fra_second.cpp): `FraSecond::FraSecond(ano, registro)`: a segunda divisão francesa
/// (a Division 2), disciplina 0xc, só a liga.
pub(crate) unsafe extern "thiscall" fn construir_franca_2(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_FRANCA_2)
}




/// O comparador dos quatro: alocam: só a original, conferindo a devolução, a vtable, o ano, o registro e a disciplina;
/// na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c = 0 e +0xc = 0; na Primera, a divisão de baixo em +0xee
/// conforme o bit 4 da nação.
pub(crate) unsafe fn cqm5_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cqm5Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_u32_as_usize(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_u32_as_usize(this + 4) == registro && *((this + 0x50) as *const u8) == p.disciplina;
    if *((this + 0x44) as *const u8) == 0xff && read_u32_as_usize(this + 8) != 0 {
        ok = ok && read_u32_as_usize(this + 0x2c) == 0 && read_u32_as_usize(this + 0xc) == 0;
        if p.baixo {
            let nacao = read_u32_as_usize(read_u32_as_usize(this + 4) + 0x5d);
            let tem = read_u32_as_usize(this + 0xee) != 0;
            ok = ok && tem == (*((nacao + 0x11c) as *const u8) & 4 == 0);
        }
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +8 = {:#x}", nome, this, r, read_u32_as_usize(this), read_u32_as_usize(this + 0x2c), read_u32_as_usize(this + 8)));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_franca_2(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1869);
    cqm5_comparar("construir_franca_2", this, ano, registro, &CQM5_FRANCA_2, || o(this, ano, registro))
}

/// the second: 4 go up and 2 go down in 2001, then 3 up and 3 down; the rest as the top division
/// Original: `FraSecond::set_up_rules` (0x5bf0a0, fra_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bf0a0 } (thiscall, ret 0, fra_second.cpp): `FraSecond::set_up_rules()`. A segunda: as comuns, 2 turnos, 3
/// pontos; em 2001 sobem 4 (+0xbe) e caem 2 (+0xc1), depois sobem 3 e caem 3; a primeira acima ([0x9cf5cc]), a
/// terceira abaixo ([0x9cf5d4]), +0x52 = 2 e 5 reservas.
pub(crate) unsafe extern "thiscall" fn regras_da_franca_2(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras da segunda");
    cqj5_regras(this, &cqj5_segunda(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_franca_2(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1757);
    cqj5_comparar("regras_da_franca_2", this, cqj5_segunda(this), || o(this), || regras_da_franca_2(this))
}

/// the second has 20 clubs; no prize fund; TV pays by the division the club came from: 2,000,000 to those relegated
/// from the top flight and 1,500,000 to those who stayed and those promoted from the National
/// Original: `FraSecond::create_table` (0x5bef10, fra_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bef10 } (thiscall, ret 0, fra_second.cpp): `FraSecond::create_table()`: 20 clubes ([0x9cf5d0]); cheia, sem
/// fundo (+0x28 = 0, +0xe1 = 2: nada a pagar no fim) e a TV pela divisão de onde o clube veio (+0x5b): 2.000.000 aos
/// que caíram da primeira ([0x9cf5cc]), 1.500.000 aos que ficaram ([0x9cf5d0]) e 1.500.000 aos que subiram do
/// National ([0x9cf5d4]); 1. Fonte 0x9ccc0c, linha 0x171.
pub(crate) unsafe extern "thiscall" fn tabela_da_franca_2(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    if cqj6_encher(this, 0x9ccc0c, 0x171, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION) != Some(true) {
        return 0;
    }
    crate::comparacao::ramo("cqj_6 tabela da segunda");
    core::ptr::write_unaligned((this + 0x28) as *mut i32, 0);
    *((this + 0xe1) as *mut u8) = 2;
    cqi5_pagar(this, 0x1e8480, cqi5_registro(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION)); // cqj6: a TV de quem caiu da primeira
    cqi5_pagar(this, 0x16e360, cqi5_registro(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION));
    cqi5_pagar(this, 0x16e360, cqi5_registro(KEY_CLUB_COMP_FRENCH_NATIONAL));
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_franca_2(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1760);
    cqj6_comparar("tabela_da_franca_2", this, 0x14, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION, None, || o(this))
}

/// the second division schedule: 38 rounds, from 31 July to 20 May; in 18 rounds one match goes to a TV slot (day 5,
/// slot 1) [inference: the televised match]
/// Original: `FraSecond::create_schedule` (0x5be620, fra_second.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5be620 } (thiscall, ret 0x10, fra_second.cpp, 2.275 bytes, método +0x3c): `FraSecond::create_schedule`: a segunda divisão,
/// 38 rodadas (`CQO4_SEGUNDA`). Fonte 0x9ccc0c, linha 0xa7.
pub(crate) unsafe extern "thiscall" fn calendario_da_segunda_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo4_calendario(this, fase, rodadas, extra, liga, &CQO4_LIGA_SEGUNDA)
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_segunda_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1891);
    cqo4_comparar("calendario_da_segunda_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_segunda_francesa(this, fase, r, e, l))
}
