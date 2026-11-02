// ---------- %%LEVA%%a leva: a Liga dos Campeões: a tabela das rodadas das fases de grupos (european_cup.cpp) ----------

/// As datas das 6 rodadas das fases de grupos (0x68a160: rodada, dia, mês 0..11, anos a somar, dia da semana; código 2,
/// valor 0), pela metade da fase: [0] primeira fase, grupos 0..3; [1] primeira fase, 4..7; [2] segunda fase, 8..9; [3]
/// segunda fase, 10..11.
const CQH4_DATAS: [[(i32, i32, i32, i32, i32); 6]; 4] = [
    [(0, 0xc, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // cqh4: a primeira fase, grupos A a D
    [(0, 0xd, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // cqh4: a primeira fase, grupos E a H
    [(0, 0x15, 0xa, 0, 1), (1, 6, 0xb, 0, 2), (2, 0xe, 1, 1, 2), (3, 0x14, 1, 1, 1), (4, 7, 2, 1, 2), (5, 0xd, 2, 1, 1)],
    [(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0xe, 2, 1, 2)], // cqh4: a segunda fase, grupos C e D
];

/// { 0x583c50 } (thiscall, ret 0x10, european_cup.cpp, método +0x3c da Liga dos Campeões, quadro SEH pelas datas locais
/// sem destrutor no caminho normal): `EuropeanCup::create_round_table(fase, &rodadas, &id, &é_liga)`, dos sorteios dos
/// grupos (0x585290, 0x585ab0) e das posições previstas (0x586900). Nas fases de grupos (char 0..11, e as negativas
/// abaixo de −1, que caem no mesmo ramo): é_liga (se pedido) = 1, rodadas = 6, id = 0x43d + fase; a tabela de 6 rodadas
/// de 0x41 bytes (0x944e46(0x186); sem memória, a caixa da linha 0x17a ou 0x1aa e devolve 0); na segunda fase, antes, a
/// data de 1º de março do ano seguinte levada à terça (0x549f70), um dia antes (0x54c8e0, que grava a data estática dela)
/// e a cópia (0x417c10), sem uso depois (a cópia as chama do mesmo jeito); as datas de `CQH4_DATAS` (0x68a160 com o ano
/// +0x40 da copa) e o jogo 0 de cada rodada vazio (0x68a1c0 com −1). As preliminares (−1) e o mata-mata (12 em diante):
/// a original pelo trampolim. Devolve a tabela.
unsafe extern "thiscall" fn campeoes_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 6;
    *id = (f as i16).wrapping_add(0x43d);
    let t = alocar(0x186);
    if t == 0 {
        aviso_em(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
        return 0;
    }
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as u16 as u32;
    let metade = if f < 8 {
        if f < 4 { 0 } else { 1 }
    } else {
        let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
        let menos: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let copia: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], usize) -> usize = core::mem::transmute(0x417c10usize);
        let (mut d, mut t1, mut t2) = ([0u16; 4], [0u16; 4], [0u16; 4]);
        // ponytail: o ano + 1 vai no ax com o resto do eax nos bytes altos; a 0x549f70 lê o ano como word
        construir(&mut d, 1, 2, (ano as u16).wrapping_add(1) as u32, 2);
        let r = menos(&mut d, &mut t1, 1);
        copia(&mut d, &mut t2, r);
        if f < 10 { 2 } else { 3 }
    };
    crate::comparacao::ramo("cqh_4 tabela das rodadas");
    for &(i, dia, mes, anos, semana) in CQH4_DATAS[metade].iter() {
        data(t, i, dia, mes, anos, semana, 2, ano, 0);
    }
    for r in 0..6 {
        jogo(t, r, 0, -1, -1, -1, 0);
    }
    t
}

/// Os bytes que a tabela tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4, +0x3d..+0x40
/// e o jogo 0 (+5..+0xb).
unsafe fn cqh4_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..6usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). As
/// preliminares e o mata-mata: só a original.
unsafe extern "thiscall" fn comparar_campeoes_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || campeoes_rodadas_da_fase(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqh4_gravados(a) == cqh4_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("campeoes_rodadas_da_fase({}): saídas {} tabela {}", f, saidas, tabela));
    a
}
