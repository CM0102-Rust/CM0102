// ---------- %%LEVA%%a leva: a Liga dos Campeões: a tabela das rodadas das fases de grupos; a liga: as expectativas da temporada e o rebaixamento de um clube (european_cup.cpp, league.cpp) ----------

/// As datas das 6 rodadas das fases de grupos (0x68a160: rodada, dia, mês 0..11, anos a somar, dia da semana; código 2,
/// valor 0), pela metade da fase: [0] primeira fase, grupos 0..3; [1] primeira fase, 4..7; [2] segunda fase, 8..9; [3]
/// segunda fase, 10..11.
const GROUP_STAGE_DATES: [[(i32, i32, i32, i32, i32); 6]; 4] = [
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
/// e a cópia (0x417c10), sem uso depois (a cópia as chama do mesmo jeito); as datas de `GROUP_STAGE_DATES` (0x68a160 com o ano
/// +0x40 da copa) e o jogo 0 de cada rodada vazio (0x68a1c0 com −1). As preliminares (−1) e o mata-mata (12 em diante):
/// a original pelo trampolim. Devolve a tabela.
unsafe extern "thiscall" fn european_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%)(this, fase, rodadas, id, liga);
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
        show_error_at(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
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
    crate::comparison::branch("cqh_4 tabela das rodadas");
    for &(i, dia, mes, anos, semana) in GROUP_STAGE_DATES[metade].iter() {
        data(t, i, dia, mes, anos, semana, 2, ano, 0);
    }
    for r in 0..6 {
        jogo(t, r, 0, -1, -1, -1, 0);
    }
    t
}

/// Os bytes que a tabela tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4, +0x3d..+0x40
/// e o jogo 0 (+5..+0xb).
unsafe fn league_written_table_bytes(t: usize) -> Vec<u8> {
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
unsafe extern "thiscall" fn comparar_european_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    let f = fase as u8 as i8;
    if f == -1 || f >= 0xc {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || european_cup_create_round_table(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { league_written_table_bytes(a) == league_written_table_bytes(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.record(igual && saidas && tabela, || format!("european_cup_create_round_table({}): saídas {} tabela {}", f, saidas, tabela));
    a
}

unsafe fn cqh4l_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh4l_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x68aa80 } (thiscall, ret 0, league.cpp, método da liga): `League::set_club_season_expectations()`: com o
/// gerenciador dos técnicos ([0xb67a44]), as expectativas da temporada de cada clube da tabela (+0xb1, linhas de 0x3b
/// bytes, +0x3e clubes) pela 0x6a3770 (push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8: com a notícia).
unsafe extern "thiscall" fn set_club_season_expectations(this: usize) {
    if cqh4l_d(0xb67a44) == 0 {
        return;
    }
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    let mut i = 0i16;
    while i < cqh4l_w(this + 0x3e) {
        crate::comparison::branch("cqh_4 expectativa do clube");
        let clube = cqh4l_d(cqh4l_d(this + 0xb1) + i as usize * 0x3b);
        expectativas(cqh4l_d(0xb67a44), clube, 1); // cqh4: com a notícia
        i += 1; // cqh4: o próximo clube
    }
}

/// Só a original (as expectativas mudam a diretoria e mandam notícias; não rodam duas vezes): a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_set_club_season_expectations(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// { 0x6831a0 } (thiscall, ret 0xc, league.cpp; this não é lido): `League::relegate_club_to_division(clube, divisão,
/// _)`: a competição do clube (+0x57) vira a divisão e a anterior vai para +0x5b; as cláusulas de rebaixamento (0x4cd030
/// push antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8); se a divisão não é jogada (sem a tabela das
/// competições [0xadadfc], sem divisão ou sem a competição do id dela) e o clube tem técnico humano que decide
/// (0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl), o técnico sai (0x69b790 push 0; push
/// 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10).
unsafe extern "thiscall" fn relegate_club_to_division(_this: usize, clube: usize, divisao: usize, _x: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let antiga = cqh4l_d(clube + 0x57);
    core::ptr::write_unaligned((clube + 0x57) as *mut u32, divisao as u32);
    core::ptr::write_unaligned((clube + 0x5b) as *mut u32, antiga as u32);
    clausulas(cqh4l_d(0xae1050), clube, antiga);
    let comps = cqh4l_d(0xadadfc);
    if comps != 0 && divisao != 0 && cqh4l_d(comps + cqh4l_d(divisao) * 4) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 {
        crate::comparison::branch("cqh_4 técnico sai");
        tirar(cqh4l_d(0xb67a44), clube, 1, 7, 0);
    }
}

/// Só a original (cláusulas de contrato e o técnico que sai não rodam duas vezes), conferindo o que ela grava: +0x57 =
/// a divisão e +0x5b = a competição de antes.
unsafe extern "thiscall" fn comparar_relegate_club_to_division(this: usize, clube: usize, divisao: usize, x: u32) {
    let antiga = cqh4l_d(clube + 0x57);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I2%%)(this, clube, divisao, x);
    PLACAR.record(cqh4l_d(clube + 0x57) == divisao && cqh4l_d(clube + 0x5b) == antiga, || format!("relegate_club_to_division: +0x57 {:#x} +0x5b {:#x}", cqh4l_d(clube + 0x57), cqh4l_d(clube + 0x5b)));
}
