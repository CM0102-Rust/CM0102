use super::*;

// ---------- 279a leva: o calendário da CFA e da Série C1 italiana (as duas chaves): as rodadas da temporada e os play-offs de acesso (fra_cfa.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type Cqo5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn cqo5_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqo5Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}


/// O que muda entre as ligas de fase única.
pub(crate) struct Cqo5Liga {
    pub(crate) rodadas: i16,
    pub(crate) arquivo: usize,
    pub(crate) linha: u32,
    pub(crate) tabela: &'static [Cqo5Rodada],
    pub(crate) ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
pub(crate) unsafe fn cqo5_calendario(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo5Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rodadas;
    *extra = 0;
    let t = alocar(p.rodadas as u32 * 0x41);
    if t == 0 {
        aviso_em(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    cqo5_rodadas(t, ano, 0, p.tabela);
    t
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
pub(crate) unsafe fn cqo5_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        cqo4_gravados(a, a, n) == cqo4_gravados(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqo4_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn cqo5_comparar_fases(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                             original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqo4_gravados(a, a, n) == cqo4_gravados(b, a, n)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, *rodadas)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a CFA (de fundo, 4 grupos de 18), 34 rodadas de 31/jul a 20/mai; quase todas com duas faixas de TV
pub(crate) const CQO5_CFA: [Cqo5Rodada; 34] = [
    (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 0: 31/jul
    (6, 7, 0, 4, 2, &[(-1, -1, -1)]), // 1: 6/ago
    (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 2: 14/ago
    (20, 7, 0, 4, 2, &[(-1, -1, -1)]), // 3: 20/ago
    (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 4: 28/ago
    (10, 8, 0, 4, 2, &[(-1, -1, -1)]), // 5: 10/set
    (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 4, 2, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 8: 2/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 9: 12/out
    (15, 9, 0, 4, 2, &[(-1, -1, -1)]), // 10: 15/out
    (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 11: 23/out
    (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 12: 30/out
    (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 13: 6/nov
    (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 14: 10/nov
    (19, 10, 0, 4, 2, &[(-1, -1, -1)]), // 15: 19/nov
    (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 16: 27/nov
    (3, 11, 0, 4, 2, &[(-1, -1, -1)]), // 17: 3/dez
    (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 18: 11/dez
    (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 19: 15/dez
    (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 20: 18/dez
    (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 21: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 22: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 23: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 24: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 25: 16/fev (+1 ano)
    (27, 1, 1, 4, 2, &[(-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 27: 11/mar (+1 ano)
    (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 28: 25/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 29: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 30: 15/abr (+1 ano)
    (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 31: 29/abr (+1 ano)
    (13, 4, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 32: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(4, 2, 1), (4, 1, 1), (-1, -1, -1)]), // 33: 20/mai (+1 ano)
];

/// a Série C1 (as duas chaves, a mesma tabela), 34 rodadas de domingo de 5/set a 7/mai, sem faixas de TV
pub(crate) const CQO5_C1: [Cqo5Rodada; 34] = [
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 0: 5/set
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 12/set
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 2: 19/set
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 26/set
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 4: 3/out
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 5: 10/out
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 7/nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 10: 14/nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 11: 21/nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 12: 28/nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 13: 5/dez
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 14: 12/dez
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 15: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 16: 6/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]), // 17: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]), // 18: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]), // 19: 30/jan (+1 ano)
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(-1, -1, -1)]), // 23: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (5, 3, 1, 2, 2, &[(-1, -1, -1)]), // 28: 5/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 29: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 7/mai (+1 ano)
];

pub(crate) const CQO5_LIGA_CFA: Cqo5Liga = Cqo5Liga { rodadas: 0x22, arquivo: 0x9ccaf0, linha: 0xab, tabela: &CQO5_CFA, ramo: "cqo_5 calendario da cfa" };

/// the CFA schedule (the French fourth level, background, 4 groups of 18): 34 rounds, from 31 July to 20 May of the
/// next year; in 25 rounds two matches go to TV slots (mostly day 6 or 4, slots 1 and 2); main stage only
/// Original: `FraCfa::create_schedule` (0x5b9510, fra_cfa.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5b9510 } (thiscall, ret 0x10, fra_cfa.cpp, 2.703 bytes, método +0x3c): `FraCfa::create_schedule(fase, &rodadas, &extra,
/// &é_liga)`: a CFA, 34 rodadas (`CQO5_CFA`). Fonte 0x9ccaf0, linha 0xab.
pub(crate) unsafe extern "thiscall" fn calendario_da_cfa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_calendario(this, fase, rodadas, extra, liga, &CQO5_LIGA_CFA)
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_cfa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1893);
    cqo5_comparar("calendario_da_cfa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_cfa(this, fase, r, e, l))
}

/// As fases de copa da C1 (rodadas de 0x68 bytes, `cql10_rodada`; categoria 5, sem prêmios, o ano +0x40 com sinal):
/// a fase 0, os play-offs de acesso: as semifinais (sorteio em 8/mai, ida em 21/mai, ida e volta com 7 dias entre os
/// jogos, 4 times em 2 confrontos, todos novos; 0xa00 no jogo único = sem sorteio (0x200)
/// mais 0x800, 0x802 no confronto de dois jogos) e a final (sorteio em 29/mai, ida em 4/jun, 2 times); a fase 1, os
/// play-outs [inferência: o rebaixamento]: uma rodada de 2 confrontos de ida e volta nas datas das semifinais, sem
/// sorteio (0x200), nome 0xbe.
pub(crate) const CQO5_SEMIFINAIS: Cql10R = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0x82, (0, 0xa00, 0x802), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0)); // cqo5: as semifinais da C1
pub(crate) const CQO5_FINAL: Cql10R = cql10r((0x1d, 4, 1, 0), (4, 5, 1, 6, 1, 0), 0x96, (0, 0x800, 0x802), 5, (2, 1, 0, 0), (0, 2, 7), (0, 0, 0));
pub(crate) const CQO5_PLAYOUT: Cql10R = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0xbe, (0, 0x200, 0x800), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));

/// O que muda entre as duas chaves da C1: a fonte e as linhas das caixas (fase −1, 0 e 1).
pub(crate) struct Cqo5C1 {
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 3],
}

/// O corpo comum da C1 A e da C1 B (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo), pela
/// fase (o byte): −1 a fase principal (é_liga = 1, 34 rodadas, extra 0, a tabela de 0x8a2 bytes com `CQO5_C1`, o ano
/// +0x40 como palavra); 0 os play-offs (é_liga = 0, 2 rodadas, extra 0xa0, a tabela de 0xd0 bytes com as semifinais e a
/// final); 1 os play-outs (é_liga = 0, 1 rodada, extra 0, 0x68 bytes); as outras devolvem 0 sem gravar nada. Sem memória
/// (malloc 0x944e46: push; add esp, 4 -> cdecl): a caixa da linha da fase e 0. Devolve a tabela.
pub(crate) unsafe fn cqo5_c1(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo5C1) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let (k, l, n, x, tamanho) = match fase as u8 {
        0xff => (0, 1, 0x22, 0, 0x8a2),
        0 => (1, 0, 2, 0xa0, 0xd0),
        1 => (2, 0, 1, 0, 0x68),
        _ => return 0,
    };
    if !liga.is_null() {
        *liga = l;
    }
    *rodadas = n;
    *extra = x;
    let t = alocar(tamanho);
    if t == 0 {
        aviso_em(p.arquivo, p.linhas[k]);
        return 0;
    }
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16);
    if k == 0 {
        crate::comparacao::ramo("cqo_5 calendario da c1");
        cqo5_rodadas(t, ano as u16 as u32, 0, &CQO5_C1);
    } else {
        crate::comparacao::ramo("cqo_5 play-offs da c1");
        let ano = ano as i32 as u32;
        if k == 1 {
            cql10_rodada(t, 0, &CQO5_SEMIFINAIS, ano);
            cql10_rodada(t, 1, &CQO5_FINAL, ano);
        } else {
            cql10_rodada(t, 0, &CQO5_PLAYOUT, ano);
        }
    }
    t
}



/// the same for the other Serie C1 group (the same table)
/// Original: `ItaSerC1b::create_schedule` (0x64ef30, ita_ser_c1b.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x64ef30 } (thiscall, ret 0x10, ita_ser_c1b.cpp, 2.279 bytes, método +0x3c): `ItaSerC1b::create_schedule`: a
/// chave B, o mesmo código. Fonte 0x9ea558, linhas 0xac, 0x156, 0x190.
pub(crate) unsafe extern "thiscall" fn calendario_da_c1_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_c1(this, fase, rodadas, extra, liga, &Cqo5C1 { arquivo: 0x9ea558, linhas: [0xac, 0x156, 0x190] })
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_c1_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1895);
    cqo5_comparar_fases("calendario_da_c1_b", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_c1_b(this, fase, r, e, l))
}

// ---------- 278a leva: as ligas de fundo francesas: a CFA (4 grupos de 18) e a divisão de baixo (2 grupos de 20): os construtores, as regras e os destrutores (fra_cfa.cpp, fra_lower.cpp) ----------




/// the rules of the two French background leagues: home and away, 3 points a win and 1 a draw, tie-breaks 4, 1, 2
/// [inference: position, goal difference, goals], no promotion, play-off or relegation places, rule bits 0x43 (a
/// champion, top places as promotion and fixed round order); the league above gets the id at [0x9cf624] (the same
/// global the Spanish Segunda uses for the Primera) [inference: an original slip, harmless since there is no promotion]
/// Original: `FraCfa::set_up_rules` (0x5ba0a0, fra_cfa.cpp, thiscall, ret 0).
///
/// [pt] { 0x5ba0a0 } (thiscall, ret 0, fra_cfa.cpp; usada também pela divisão de baixo): `FraCfa::set_up_rules()`: turno e
/// returno (+0x3c = 2), 3 pontos por vitória (+0xc2) e 1 por empate (+0xc3), a categoria 2 das partidas (+0xc4), os
/// desempates 4, 1, 2 (+0xc5..+0xc7: [inferência] a posição, o saldo, os gols), o tipo 1 (+0x42), nenhuma vaga de
/// acesso, de play-off nem de rebaixamento (+0xbe..+0xc1 = 0), a liga de cima de id [0x9cf624] (+0x1c) e nenhuma de
/// baixo (+0x20 = −1), os bits de regra 0x43 (+0xd9); por fim o calendário (a virtual +0x3c: push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, esi -> thiscall, ret 0x10) em +0xba. Devolve 1.
pub(crate) unsafe extern "thiscall" fn regras_da_cfa(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3c) as *mut u16, 2);
    *((this + 0xc4) as *mut u8) = 2;
    *((this + 0xc7) as *mut u8) = 2;
    *((this + 0xc2) as *mut u8) = 3; // cqn4: os pontos por vitória nas ligas de fundo francesas
    *((this + 0xc3) as *mut u8) = 1;
    *((this + 0x42) as *mut u8) = 1;
    *((this + 0xc5) as *mut u8) = 4;
    *((this + 0xc6) as *mut u8) = 1;
    *((this + 0xbe) as *mut u8) = 0;
    *((this + 0xbf) as *mut u8) = 0;
    *((this + 0xc0) as *mut u8) = 0;
    *((this + 0xc1) as *mut u8) = 0;
    // ponytail: a original grava em +0x1c o id de [0x9cf624] (o mesmo global que a Segunda espanhola usa para a
    // Primera); fiel.
    write_u32_from_usize(this + 0x1c, read_u32_as_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION));
    write_u32_from_usize(this + 0x20, 0xffff_ffff);
    core::ptr::write_unaligned((this + 0xd9) as *mut u16, 0x43); // cqn4: os bits de regra das ligas de fundo
    let calendario: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let g = calendario(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    write_u32_from_usize(this + 0xba, g);
    1
}

/// O calendário (a virtual +0x3c) aloca: só a original, conferindo os campos e a devolução 1.
pub(crate) unsafe extern "thiscall" fn comparar_regras_da_cfa(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1876);
    let r = o(this);
    let b = |k: usize| *((this + k) as *const u8);
    let ok = r == 1 && core::ptr::read_unaligned((this + 0x3c) as *const u16) == 2 && b(0xc2) == 3 && b(0xc3) == 1 && b(0xc4) == 2
        && b(0xc5) == 4 && b(0xc6) == 1 && b(0xc7) == 2 && b(0x42) == 1 && b(0xbe) == 0 && b(0xbf) == 0 && b(0xc0) == 0 && b(0xc1) == 0
        && read_u32_as_usize(this + 0x1c) == read_u32_as_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && read_u32_as_usize(this + 0x20) == 0xffff_ffff
        && core::ptr::read_unaligned((this + 0xd9) as *const u16) == 0x43;
    PLACAR.registrar(ok, || format!("regras_da_cfa({:#x}): devolveu {}, campos fora do esperado", this, r));
    r
}

/// O que muda entre as duas ligas de fundo.
pub(crate) struct Cqn4Fundo {
    pub(crate) vtabela: usize,
    /// o id do registro da competição (+4)
    pub(crate) competicao: usize,
    /// os ids das competições cujos clubes entram todos; os outros vêm da divisão genérica ([0x9cf764])
    pub(crate) todas: &'static [usize],
    /// só os da nação de id [g] (a divisão de baixo: a França) entre os da genérica, e sem competição de reserva
    /// (+0x60 = 0); a CFA pede, ao contrário, a competição de reserva = a própria ([[+0x60]] = [competicao])
    pub(crate) nacao: Option<usize>,
    /// o máximo da lista (72 a CFA, 40 a de baixo) e o máximo dos da genérica (34 e 20)
    pub(crate) maximo: i16,
    pub(crate) genericos: i16,
    /// os grupos e a tabela de cada um (thiscall(liga, lista), ret 4: 0x5b9fa0 e 0x5be230)
    pub(crate) grupos: usize,
    pub(crate) por_grupo: usize,
    pub(crate) tabela: usize,
    pub(crate) arquivo: usize,
    /// as caixas: regras, tabela do grupo, jogos
    pub(crate) linhas: [u32; 3],
}

/// O corpo dos dois construtores (thiscall(this, ano), ret 4, quadro SEH só no caminho de exceção; sem estado salvo):
/// a base (League::League 0x682200: mov ecx -> thiscall, ret 0), a vtable, o ano (+0x40), +0x2c = 0, sem disciplina
/// (+0x50 = 0xff), o registro (+4), a fase 0xff (+0x44) e −1 (+0x30); as regras (0x5ba0a0: mov ecx -> thiscall, ret 0;
/// falhando, a caixa e fim). Depois a lista dos clubes, na ordem do banco (os [0xae235c] − 2 × [0xae2350] primeiros),
/// até o máximo: os das competições `todas`; e, até o máximo dos genéricos, os da divisão genérica com a condição da
/// liga. A lista é embaralhada (comp_util_shuffle_pointer_array 0x4afb40: push quantos; push lista; add esp, 8 ->
/// cdecl; sorteia) e cada grupo, com os seus clubes em sequência na lista, ganha a tabela (falhando, a caixa), os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0; falhando, a caixa) e a tabela solta
/// (League::free_table 0x687970: push 0; mov ecx -> thiscall, ret 4): os jogos do grupo ficam no calendário do jogo e a
/// liga segue sem tabela. A lista é um registro local de 72 ou 40 ponteiros; a cópia usa um vetor zerado (a carreira
/// enche a lista; a original leria lixo da pilha sem clubes bastantes).
pub(crate) unsafe fn cqn4_construir(this: usize, ano: u32, p: &Cqn4Fundo) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FRACFA_SET_UP_RULES);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    base(this);
    write_u32_from_usize(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    write_u32_from_usize(this + 0x2c, 0);
    *((this + 0x50) as *mut u8) = 0xff;
    write_u32_from_usize(this + 4, cqm6_registro(p.competicao));
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    crate::comparacao::ramo("cqn_4 liga de fundo francesa");
    let mut lista = vec![0usize; p.maximo as usize];
    let mut k: i16 = 0;
    let mut genericos: i16 = 0;
    let n = (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base_clubes = read_u32_as_usize(G_CLUB_TABLE);
    let mut i: i32 = 0;
    while i < n {
        let c = base_clubes + (i as usize) * 0x245;
        let comp = read_u32_as_usize(c + 0x57);
        let id = if comp != 0 { read_u32_as_usize(comp) } else { 0 };
        if comp != 0 && p.todas.iter().any(|&g| id == read_u32_as_usize(g)) {
            lista[k as usize] = c;
            k += 1;
            if k == p.maximo {
                break;
            }
        } else {
            let entra = match p.nacao {
                Some(g) => {
                    let nacao = read_u32_as_usize(c + 0x53);
                    nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(g) && read_u32_as_usize(c + 0x60) == 0 && comp != 0 && id == read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)
                }
                None => {
                    let reserva = read_u32_as_usize(c + 0x60);
                    comp != 0 && id == read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && reserva != 0 && read_u32_as_usize(reserva) == read_u32_as_usize(p.competicao)
                }
            };
            if entra && genericos < p.genericos { // cqn4: o máximo dos clubes da divisão genérica
                lista[k as usize] = c;
                k += 1;
                genericos += 1;
                if k == p.maximo {
                    break;
                }
            }
        }
        i += 1;
    }
    embaralhar(lista.as_mut_ptr() as usize, k as u16 as u32); // cqn4: o sorteio da lista
    for g in 0..p.grupos {
        if tabela(this, lista.as_ptr() as usize + g * p.por_grupo * 4) == 0 {
            aviso_em(p.arquivo, p.linhas[1]);
            return this;
        }
        if jogos(this) == 0 {
            aviso_em(p.arquivo, p.linhas[2]);
            return this;
        }
        soltar(this, 0);
    }
    this
}

pub(crate) const CQN4_CFA: Cqn4Fundo = Cqn4Fundo {
    vtabela: 0x96a7b0,
    competicao: KEY_CLUB_COMP_FRENCH_CFA,
    todas: &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION], // cqn4: os clubes da primeira e da segunda divisões na CFA
    nacao: None,
    maximo: 0x48,
    genericos: 0x22,
    grupos: 4,
    por_grupo: 0x12,
    tabela: 0x5b9fa0,
    arquivo: 0x9ccaf0,
    linhas: [0x40, 0x64, 0x6b],
};
pub(crate) const CQN4_BAIXO: Cqn4Fundo = Cqn4Fundo {
    vtabela: 0x96ab78,
    competicao: KEY_CLUB_COMP_FRENCH_LOWER_DIVISION,
    todas: &[KEY_CLUB_COMP_FRENCH_NATIONAL],
    nacao: Some(KEY_NATION_FRANCE),
    maximo: 0x28,
    genericos: 0x14,
    grupos: 2,
    por_grupo: 0x14,
    tabela: 0x5be230,
    arquivo: 0x9ccbd0,
    linhas: [0x40, 0x62, 0x69],
};

/// the CFA (the French fourth level, background): up to 72 clubs, all of the first and second divisions [inference: the
/// reserve sides] and up to 34 of the generic division whose reserve competition is the CFA, drawn into 4 groups of 18;
/// each group gets the table and the fixtures (in the game calendar) and the table is freed
/// Original: `FraCfa::FraCfa` (0x5b91c0, fra_cfa.cpp, thiscall, ret 4).
///
/// [pt] { 0x5b91c0 } (thiscall, ret 4, fra_cfa.cpp, 596 bytes): `FraCfa::FraCfa(ano)`, a CFA (a quarta divisão francesa, de
/// fundo; competição de id [0x9cf5d8]): até 72 clubes, os da primeira e da segunda divisões ([0x9cf5cc], [0x9cf5d0];
/// [inferência: os times reserva]) e até 34 da divisão genérica cuja competição de reserva é a CFA, embaralhados em 4
/// grupos de 18. Fonte 0x9ccaf0, linhas 0x40, 0x64, 0x6b.
pub(crate) unsafe extern "thiscall" fn construir_cfa(this: usize, ano: u32) -> usize {
    cqn4_construir(this, ano, &CQN4_CFA)
}

/// the French lower division (background): up to 40 clubs, those of the National and up to 20 French clubs of the
/// generic division with no reserve competition, drawn into 2 groups of 20
/// Original: `FraLower::FraLower` (0x5bd600, fra_lower.cpp, thiscall, ret 4).
///
/// [pt] { 0x5bd600 } (thiscall, ret 4, fra_lower.cpp, 578 bytes): `FraLower::FraLower(ano)`, a divisão de baixo francesa
/// (competição de id [0x9cf548]): até 40 clubes, os do National ([0x9cf5d4]) e até 20 franceses ([0x9cf300]) da divisão
/// genérica sem competição de reserva, embaralhados em 2 grupos de 20. Fonte 0x9ccbd0, linhas 0x40, 0x62, 0x69.
pub(crate) unsafe extern "thiscall" fn construir_divisao_de_baixo_francesa(this: usize, ano: u32) -> usize {
    cqn4_construir(this, ano, &CQN4_BAIXO)
}

/// Sorteiam e alocam: só a original, conferindo a devolução, a vtable, o ano, a disciplina, o registro e a liga sem
/// tabela no fim (+0x3e = 0, +0xb1 = 0: a última tabela foi solta).
pub(crate) unsafe fn cqn4_comparar(nome: &str, this: usize, ano: u32, p: &Cqn4Fundo, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let ok = r == this && read_u32_as_usize(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && read_u32_as_usize(this + 4) == cqm6_registro(p.competicao)
        && read_u32_as_usize(this + 0xb1) == 0;
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}, +0xb1 = {:#x}", nome, this, r, read_u32_as_usize(this), read_u32_as_usize(this + 4), read_u32_as_usize(this + 0xb1)));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_cfa(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1877);
    cqn4_comparar("construir_cfa", this, ano, &CQN4_CFA, || o(this, ano))
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_divisao_de_baixo_francesa(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1878);
    cqn4_comparar("construir_divisao_de_baixo_francesa", this, ano, &CQN4_BAIXO, || o(this, ano))
}

/// O corpo dos dois destrutores (thiscall, ret 0, quadro SEH só no caminho de exceção; iguais fora a vtable): a vtable
/// da classe, a tabela solta (League::free_table 0x687970: push 0 -> thiscall, ret 4), o calendário [+0xba] solto
/// (0x9452ca: push; add esp, 4 -> cdecl) e zerado, cada fase de 0 a +0x30 (com +0x30 ≥ 0) destruída pela sua virtual
/// 0 com 1 (o destrutor que solta: push 1; mov ecx, fase -> thiscall, ret 4), o vetor [+0xc] solto e zerado e, por fim,
/// League::~League (0x682300: mov ecx -> thiscall, ret 0).
pub(crate) unsafe fn cqn4_destruir(this: usize, vtabela: usize) {
    let soltar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_DESTRUCTOR_LEAGUE);
    write_u32_from_usize(this, vtabela);
    soltar_tabela(this, 0);
    let g = read_u32_as_usize(this + 0xba);
    if g != 0 {
        soltar(g);
        write_u32_from_usize(this + 0xba, 0);
    }
    if (read_u32_as_usize(this + 0x30) as i32) >= 0 {
        let mut i: i32 = 0;
        loop {
            let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + i as usize * 4);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > read_u32_as_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    let v = read_u32_as_usize(this + 0xc);
    if v != 0 {
        soltar(v);
        write_u32_from_usize(this + 0xc, 0);
    }
    base(this);
}

/// frees the table, the schedule, the stages and their vector and destroys the league
/// Original: `FraCfa::~FraCfa` (0x5b9460, fra_cfa.cpp, thiscall, ret 0).
///
/// [pt] { 0x5b9460 } (thiscall, ret 0, fra_cfa.cpp): `FraCfa::~FraCfa()`.
pub(crate) unsafe extern "thiscall" fn destruir_cfa(this: usize) {
    crate::comparacao::ramo("cqn_4 destrutor da cfa");
    cqn4_destruir(this, 0x96a7b0)
}

/// the same for the lower division
/// Original: `FraLower::~FraLower` (0x5bd870, fra_lower.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bd870 } (thiscall, ret 0, fra_lower.cpp): `FraLower::~FraLower()`.
pub(crate) unsafe extern "thiscall" fn destruir_divisao_de_baixo_francesa(this: usize) {
    crate::comparacao::ramo("cqn_4 destrutor da divisao de baixo");
    cqn4_destruir(this, 0x96ab78)
}

/// Soltam memória: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_destruir_cfa(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1879)(this)
}

pub(crate) unsafe extern "thiscall" fn comparar_destruir_divisao_de_baixo_francesa(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1880)(this)
}

// ---------- 275a leva: as tabelas dos grupos da CFA e da divisão de baixo francesa (a marca dos clubes de divisão maior) e a situação nas estatísticas da Série C1 italiana (o play-off e o play-out em faixas) (fra_cfa.cpp, fra_lower.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------



/// O corpo das duas tabelas francesas: +0x3e = `n`, a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do
/// fonte `arquivo` na linha `linha` e 0) e os `n` clubes da lista `clubes`, na ordem, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, marca, 0), ret 0x10), com a marca 1 nos clubes
/// cuja competição (+0x57, o id no +0) é uma de `marcadas`. Devolve se encheu (a conta das linhas = +0x3e).
pub(crate) unsafe fn cqk6_tabela(this: usize, clubes: usize, n: i16, arquivo: usize, linha: u32, marcadas: &[usize]) -> u32 {
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
    while i < read_i16(this + 0x3e) {
        let clube = read_usize(clubes + (i as usize) * 4);
        let id = read_usize(read_usize(clube + 0x57));
        let marca = marcadas.iter().any(|&g| read_usize(g) == id) as u32; // cqk6: a marca dos clubes de divisão maior
        // ponytail: a original empurra a linha no ecx com os bits de cima do id da competição; a 0x687430 só lê o
        // byte.
        let linha = k; // cqk6: a linha k recebe o k-ésimo clube da lista
        iniciar(this, linha as u32, clube, marca, 0);
        k = k.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqk_6 tabela francesa de baixo");
    (k as i8 as i16 == read_i16(this + 0x3e)) as u32
}

/// a CFA group has 18 clubs (4 groups, drawn by the constructor); clubs that are in the first or second division get
/// the row mark [inference: the reserve sides of professional clubs, which cannot go up]
/// Original: `FraCfa::create_group_table` (0x5b9fa0, fra_cfa.cpp, thiscall, ret 4).
///
/// [pt] { 0x5b9fa0 } (thiscall, ret 4, fra_cfa.cpp): `FraCfa::create_group_table(clubes)`: um grupo da CFA com 18 clubes
/// da lista (o construtor 0x5b91c0 sorteia os clubes dos 4 grupos), marcando os que estão na primeira ([0x9cf5cc]) ou
/// na segunda ([0x9cf5d0]) divisão [inferência: os times reserva dos profissionais, que não sobem]. Fonte 0x9ccaf0,
/// linha 0x185.
pub(crate) unsafe extern "thiscall" fn tabela_do_grupo_da_cfa(this: usize, clubes: usize) -> u32 {
    cqk6_tabela(this, clubes, 0x12, 0x9ccaf0, 0x185, &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION])
}

/// the French lower division has 20 clubs (chosen by the constructor); those in the National get the mark
/// Original: `FraLower::create_table` (0x5be230, fra_lower.cpp, thiscall, ret 4).
///
/// [pt] { 0x5be230 } (thiscall, ret 4, fra_lower.cpp): `FraLower::create_table(clubes)`: a divisão de baixo francesa com
/// 20 clubes da lista (o construtor 0x5bd600 escolhe-os), marcando os que estão no National ([0x9cf5d4]). Fonte
/// 0x9ccbd0, linha 0x175.
pub(crate) unsafe extern "thiscall" fn tabela_da_divisao_de_baixo_francesa(this: usize, clubes: usize) -> u32 {
    cqk6_tabela(this, clubes, 0x14, 0x9ccbd0, 0x175, &[KEY_CLUB_COMP_FRENCH_NATIONAL])
}

/// O comparador das duas: a original aloca a tabela: só ela. O plano, feito antes: os `n` clubes da lista e a marca de
/// cada um. Depois: +0x3e = `n`, a devolução 1 e as linhas com os clubes na ordem e a marca (+0x38).
pub(crate) unsafe fn cqk6_comparar(nome: &str, this: usize, clubes: usize, n: i16, marcadas: &[usize], original: impl FnOnce() -> u32) -> u32 {
    let plano: Vec<(usize, u8)> = (0..n as usize)
        .map(|i| {
            let c = read_usize(clubes + i * 4);
            let id = read_usize(read_usize(c + 0x57));
            (c, marcadas.iter().any(|&g| read_usize(g) == id) as u8)
        })
        .collect();
    let r = original();
    let t = read_usize(this + 0xb1);
    let mut achou: Vec<(usize, u8)> = Vec::new();
    let mut igual = r == 1 && read_i16(this + 0x3e) == n && t != 0;
    if igual {
        achou = (0..n as usize).map(|k| (read_usize(t + k * 0x3b), *((t + k * 0x3b + 0x38) as *const u8))).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_do_grupo_da_cfa(this: usize, clubes: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1768);
    cqk6_comparar("tabela_do_grupo_da_cfa", this, clubes, 0x12, &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION], || o(this, clubes))
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_divisao_de_baixo_francesa(this: usize, clubes: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1769);
    cqk6_comparar("tabela_da_divisao_de_baixo_francesa", this, clubes, 0x14, &[KEY_CLUB_COMP_FRENCH_NATIONAL], || o(this, clubes))
}

/// A situação do clube nas estatísticas da Série C1 (o corpo de 0x64bd80 e 0x64ff40): na entrada do clube
/// (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor (+0x74)
/// e a pior (+0x75). Na liga (fase 0xff): a melhor de 2 a 5 (o play-off de acesso) vira 2 e de 14 a 17 (o play-out)
/// vira 14; a pior de 2 a 5 vira 5 e de 14 a 17 vira 17. No play-off (fase 0): as três + 1. No play-out (fase 1): a
/// melhor 1 vira 14 e 2 vira 16. Noutra fase, como vieram.
pub(crate) unsafe fn cqk6_situacao_na_c1(this: usize, clube: usize, fase: u8, atual: u8, melhor: u8, pior: u8) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = achar(read_usize(this + 8), clube);
    let (mut a, mut m, mut p) = (atual, melhor, pior);
    if fase == 0xff {
        m = match m {
            2..=5 => 2,
            14..=17 => {
                crate::comparacao::ramo("cqk_6 faixa do play-out");
                0xe // cqk6: a melhor na faixa do play-out
            }
            v => v,
        };
        p = match p {
            2..=5 => 5,
            14..=17 => 0x11,
            v => v,
        };
    } else if fase == 0 {
        m = m.wrapping_add(1);
        p = p.wrapping_add(1);
        a = a.wrapping_add(1);
    } else if fase == 1 {
        m = match m {
            1 => 0xe,
            2 => 0x10,
            v => v,
        };
    }
    *((e + 0x73) as *mut u8) = a;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}


/// the same rule in C1/B
/// Original: `ItaSerC1b::set_club_progress` (0x64ff40, ita_ser_c1b.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x64ff40 } (thiscall, ret 0x14, ita_ser_c1b.cpp): `ItaSerC1b::set_club_progress(...)`, igual à da C1/A.
pub(crate) unsafe extern "thiscall" fn situacao_na_c1b(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cqk6_situacao_na_c1(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8)
}

/// O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
pub(crate) unsafe fn cqk6_comparar_situacao(nome: &str, this: usize, clube: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = achar(read_usize(this + 8), clube);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", nome, this, clube, mo, mr));
}


pub(crate) unsafe extern "thiscall" fn comparar_situacao_na_c1b(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1771);
    cqk6_comparar_situacao("situacao_na_c1b", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_c1b(this, clube, fase, atual, melhor, pior))
}

