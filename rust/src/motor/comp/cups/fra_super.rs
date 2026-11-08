use super::*;

// ---------- 276a leva: o calendário das supercopas: o jogo da Supercopa da França, da Espanha e do Japão (fra_super.cpp, spa_super.cpp, jap_super.cpp) ----------


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
pub(crate) struct Cql12R {
    pub(crate) sorteio: (u32, u32, u32, u32),
    pub(crate) jogo: (u32, u32, u32, u32, u32, u32),
    pub(crate) codigo: u16,
    pub(crate) bits: (u16, u16, u16),
    pub(crate) categoria: u8,
    pub(crate) times: (u16, u16, u16, u16),
    pub(crate) confronto: (u8, u8, u8),
    pub(crate) premios: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
pub(crate) const fn cql12r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> Cql12R {
    Cql12R { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
pub(crate) unsafe fn cql12_rodada(t: usize, i: u32, r: &Cql12R, ano: u32) {
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    sorteio(t, i, r.sorteio.0, r.sorteio.1, r.sorteio.2, r.sorteio.3, ano);
    jogo(t, i, r.jogo.0, r.jogo.1, r.jogo.2, r.jogo.3, r.jogo.4, ano, r.jogo.5);
    let e = t + i as usize * 0x68;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((e + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((e + o) as *mut u32, v);
    w(7, r.codigo);
    w(9, i as u16);
    w(0xb, r.bits.0);
    w(0xd, r.bits.1);
    w(0xf, r.bits.2);
    *((e + 0x17) as *mut u8) = r.categoria;
    w(0x18, r.times.0);
    w(0x1a, r.times.1);
    w(0x1c, r.times.2);
    w(0x1e, r.times.3);
    *((e + 0x20) as *mut u8) = r.confronto.0;
    *((e + 0x21) as *mut u8) = r.confronto.1;
    *((e + 0x22) as *mut u8) = r.confronto.2;
    d(0x5c, r.premios.0);
    d(0x60, r.premios.1);
    d(0x64, r.premios.2);
}

/// O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela. `ajuste` mexe na primeira rodada depois da alocação (o mando sorteado da
/// Supercopa francesa).
pub(crate) unsafe fn cql12_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cql12R], arquivo: usize, linha: u32, nome: &'static str,
    ajuste: impl FnOnce(&mut Cql12R)) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = tabela.len() as i16;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(tabela.len() as u32 * 0x68);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let mut t0 = tabela[0];
    ajuste(&mut t0);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        cql12_rodada(t, i as u32, if i == 0 { &t0 } else { r }, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
pub(crate) unsafe fn cql12_gravados(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x68;
        for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
            v.extend_from_slice(core::slice::from_raw_parts((e + a) as *const u8, b - a));
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada).
pub(crate) unsafe fn cql12_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if fase as u8 != 0xff {
        // ponytail: fora da fase −1 nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut s2 = if saida.is_null() { 0 } else { *saida };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if saida.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (saida.is_null() || *saida == s2);
    let tabela = if a != 0 && b != 0 { cql12_gravados(a, *rodadas) == cql12_gravados(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Supercopa da França: um jogo em julho; o prêmio de 60 mil por chegar. O mando vem à parte (`cql12_mando_da_franca`).
pub(crate) const CQL12_SUPERCOPA_DA_FRANCA: [Cql12R; 1] = [
    cql12r((5, 6, 0, 5), (0x18, 6, 0, 5, 2, 0x400), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0xea60, 0, 0)), // cql12: o jogo da Supercopa da França
];

/// O mando da Supercopa da França: 0x400 em 2001 (+0x40 = 0x7d1); nos outros anos, um dos seis (0x200, 0x400, 0x800,
/// 0x1000, 0x2000, 0x4000) pela 0x90cfc0 (push 6; add esp, 4 -> cdecl, o sorteio de 0 a 5).
pub(crate) unsafe fn cql12_mando_da_franca(this: usize) -> u32 {
    if core::ptr::read_unaligned((this + 0x40) as *const i16) == 0x7d1 {
        return 0x400;
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    const MANDOS: [i16; 6] = [0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000];
    MANDOS[sortear(6) as usize] as i32 as u32
}

/// rule: the French Super Cup (Trophée des Champions) is a single match between 2 clubs, drawn on 5/7 and played on
/// 24/7 (moved to weekday 5), decided on penalties [inference: tiebreak 1]; in 2001 at venue 0x400, in other years at
/// one of six drawn venues (0x200 to 0x4000: [inference] the country's largest grounds); 60 thousand to each club for
/// taking part; category 6
/// Original: `FraSuper::create_round_table` (0x5bf5d0, fra_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bf5d0 } (thiscall, ret 0x10, fra_super.cpp; a virtual +0x3c da Supercopa da França): `FraSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única com o mando de
/// `cql12_mando_da_franca` (a original sorteia depois da data do sorteio, que não sorteia: a cópia sorteia logo antes);
/// a caixa da linha 0x97 sem memória.
pub(crate) unsafe extern "thiscall" fn cql12_supercopa_da_franca_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql12_tabela(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DA_FRANCA, 0x9ccc48, 0x97, "cql_12 tabela da supercopa da franca",
        |r| r.jogo.5 = cql12_mando_da_franca(this))
}

// ponytail: fora de 2001 a original sorteia o mando (0x90cfc0): só ela roda nesse caso.
pub(crate) unsafe extern "thiscall" fn comparar_cql12_supercopa_da_franca_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1778);
    if core::ptr::read_unaligned((this + 0x40) as *const i16) != 0x7d1 {
        return o(this, fase, rodadas, extra, saida);
    }
    cql12_comparar("cql12_supercopa_da_franca_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql12_supercopa_da_franca_tabela(this, fase, r, e, s))
}

/// A Supercopa da Espanha: ida e volta (7 dias entre os jogos) em agosto, sem sorteio (+0xd = 0x200), desempate de ida e
/// volta 1.
pub(crate) const CQL12_SUPERCOPA_DA_ESPANHA: [Cql12R; 1] = [
    cql12r((5, 6, 0, 5), (0x14, 7, 0, 6, 1, 0), 0, (0, 0x200, 1), 6, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)), // cql12: a ida da Supercopa da Espanha
];



/// A Supercopa do Japão: um jogo em março no estádio neutro, 150 mil por vencer.
pub(crate) const CQL12_SUPERCOPA_DO_JAPAO: [Cql12R; 1] = [
    cql12r((1, 1, 0, 5), (4, 2, 0, 5, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0x249f0, 0)), // cql12: o jogo da Supercopa do Japão
];

/// rule: the Japanese Super Cup (Xerox Super Cup) is a single match between 2 clubs, drawn on 1/2 and played on 4/3
/// (moved to weekday 5) at a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; 150
/// thousand to the winner; category 6
/// Original: `JapSuper::create_round_table` (0x666020, jap_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x666020 } (thiscall, ret 0x10, jap_super.cpp; a virtual +0x3c da Supercopa do Japão): `JapSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
pub(crate) unsafe extern "thiscall" fn cql12_supercopa_do_japao_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql12_tabela(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DO_JAPAO, 0x9eaf4c, 0x98, "cql_12 tabela da supercopa do japao", |_| {})
}

pub(crate) unsafe extern "thiscall" fn comparar_cql12_supercopa_do_japao_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1780);
    cql12_comparar("cql12_supercopa_do_japao_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql12_supercopa_do_japao_tabela(this, fase, r, e, s))
}

/// rule: the French Super Cup is between the First Division champion and the Coupe de France winner; if the same club
/// won both, the First Division runner-up takes its place; if one is missing, the error box
/// Original: `FraSuper::select_teams` (0x5bf770, fra_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da França é entre o campeão da Primeira e o da Copa da França; se o mesmo clube ganhou as duas,
/// entra o vice da Primeira; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn cqm11_supercopa_clubes(this: usize) -> i32 {
    cqm11_supercopa(this).0
}

pub(crate) unsafe extern "thiscall" fn comparar_cqm11_supercopa_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1786);
    cqm11_comparar("cqm11_supercopa_clubes", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm11_supercopa(this),
        |r, _| if r == 1 { 12 } else { 0 })
}
