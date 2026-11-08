// ---------- %%LEVA%%a leva: o calendário da CFA e da Série C1 italiana (as duas chaves): as rodadas da temporada e os play-offs de acesso (fra_cfa.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqo5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqo5_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqo5Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
unsafe fn cqo5_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// O que muda entre as ligas de fase única.
struct Cqo5Liga {
    rodadas: i16,
    arquivo: usize,
    linha: u32,
    tabela: &'static [Cqo5Rodada],
    ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
unsafe fn cqo5_calendario(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo5Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
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
unsafe fn cqo5_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        cqo5_gravados(a, a, n) == cqo5_gravados(b, a, n)
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
/// `cqo5_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqo5_comparar_fases(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                             original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqo5_gravados(a, a, n) == cqo5_gravados(b, a, n)
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
const CQO5_CFA: [Cqo5Rodada; 34] = [
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
const CQO5_C1: [Cqo5Rodada; 34] = [
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

const CQO5_LIGA_CFA: Cqo5Liga = Cqo5Liga { rodadas: 0x22, arquivo: 0x9ccaf0, linha: 0xab, tabela: &CQO5_CFA, ramo: "cqo_5 calendario da cfa" };

/// { 0x5b9510 } (thiscall, ret 0x10, fra_cfa.cpp, 2.703 bytes, método +0x3c): `FraCfa::create_schedule(fase, &rodadas, &extra,
/// &é_liga)`: a CFA, 34 rodadas (`CQO5_CFA`). Fonte 0x9ccaf0, linha 0xab.
unsafe extern "thiscall" fn fra_cfa_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_calendario(this, fase, rodadas, extra, liga, &CQO5_LIGA_CFA)
}

unsafe extern "thiscall" fn comparar_fra_cfa_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cqo5_comparar("fra_cfa_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_cfa_create_schedule(this, fase, r, e, l))
}

/// As fases de copa da C1 (rodadas de 0x68 bytes, `cql10_rodada`; categoria 5, sem prêmios, o ano +0x40 com sinal):
/// a fase 0, os play-offs de acesso: as semifinais (sorteio em 8/mai, ida em 21/mai, ida e volta com 7 dias entre os
/// jogos, 4 times em 2 confrontos, todos novos; 0xa00 no jogo único = sem sorteio (0x200)
/// mais 0x800, 0x802 no confronto de dois jogos) e a final (sorteio em 29/mai, ida em 4/jun, 2 times); a fase 1, os
/// play-outs [inferência: o rebaixamento]: uma rodada de 2 confrontos de ida e volta nas datas das semifinais, sem
/// sorteio (0x200), nome 0xbe.
const CQO5_SEMIFINAIS: Cql10R = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0x82, (0, 0xa00, 0x802), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0)); // cqo5: as semifinais da C1
const CQO5_FINAL: Cql10R = cql10r((0x1d, 4, 1, 0), (4, 5, 1, 6, 1, 0), 0x96, (0, 0x800, 0x802), 5, (2, 1, 0, 0), (0, 2, 7), (0, 0, 0));
const CQO5_PLAYOUT: Cql10R = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0xbe, (0, 0x200, 0x800), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));

/// O que muda entre as duas chaves da C1: a fonte e as linhas das caixas (fase −1, 0 e 1).
struct Cqo5C1 {
    arquivo: usize,
    linhas: [u32; 3],
}

/// O corpo comum da C1 A e da C1 B (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo), pela
/// fase (o byte): −1 a fase principal (é_liga = 1, 34 rodadas, extra 0, a tabela de 0x8a2 bytes com `CQO5_C1`, o ano
/// +0x40 como palavra); 0 os play-offs (é_liga = 0, 2 rodadas, extra 0xa0, a tabela de 0xd0 bytes com as semifinais e a
/// final); 1 os play-outs (é_liga = 0, 1 rodada, extra 0, 0x68 bytes); as outras devolvem 0 sem gravar nada. Sem memória
/// (malloc 0x944e46: push; add esp, 4 -> cdecl): a caixa da linha da fase e 0. Devolve a tabela.
unsafe fn cqo5_c1(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo5C1) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
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

/// { 0x64ad80 } (thiscall, ret 0x10, ita_ser_c1a.cpp, 2.279 bytes, método +0x3c): `ItaSerC1a::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a chave A da Série C1. Fonte 0x9ea474, linhas 0xab, 0x155, 0x18f.
unsafe extern "thiscall" fn ita_ser_c1a_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_c1(this, fase, rodadas, extra, liga, &Cqo5C1 { arquivo: 0x9ea474, linhas: [0xab, 0x155, 0x18f] })
}

unsafe extern "thiscall" fn comparar_ita_ser_c1a_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    cqo5_comparar_fases("ita_ser_c1a_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| ita_ser_c1a_create_schedule(this, fase, r, e, l))
}

/// { 0x64ef30 } (thiscall, ret 0x10, ita_ser_c1b.cpp, 2.279 bytes, método +0x3c): `ItaSerC1b::create_schedule`: a
/// chave B, o mesmo código. Fonte 0x9ea558, linhas 0xac, 0x156, 0x190.
unsafe extern "thiscall" fn fra_cfa_create_schedule_64ef30(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_c1(this, fase, rodadas, extra, liga, &Cqo5C1 { arquivo: 0x9ea558, linhas: [0xac, 0x156, 0x190] })
}

unsafe extern "thiscall" fn comparar_fra_cfa_create_schedule_64ef30(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    cqo5_comparar_fases("fra_cfa_create_schedule_64ef30", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_cfa_create_schedule_64ef30(this, fase, r, e, l))
}
