// ---------- %%LEVA%%a leva: a liga: as expectativas da temporada, o rebaixamento de um clube e a cadeia de promoções (league.cpp) ----------

unsafe fn cqh5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x68aa80 } (thiscall, ret 0, league.cpp, método da liga): `League::set_club_season_expectations()`: com o
/// gerenciador dos técnicos ([0xb67a44]), as expectativas da temporada de cada clube da tabela (+0xb1, linhas de 0x3b
/// bytes, +0x3e clubes) pela 0x6a3770 (push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8: com a notícia).
unsafe extern "thiscall" fn liga_expectativas_da_temporada(this: usize) {
    if cqh5_d(0xb67a44) == 0 {
        return;
    }
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    let mut i = 0i16;
    while i < cqh5_w(this + 0x3e) {
        crate::comparacao::ramo("cqh_5 expectativa do clube");
        let clube = cqh5_d(cqh5_d(this + 0xb1) + i as usize * 0x3b);
        expectativas(cqh5_d(0xb67a44), clube, 1); // cqh5: com a notícia
        i += 1; // cqh5: o próximo clube
    }
}

/// Só a original (as expectativas mudam a diretoria e mandam notícias; não rodam duas vezes): a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_liga_expectativas_da_temporada(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x6831a0 } (thiscall, ret 0xc, league.cpp; this não é lido): `League::relegate_club_to_division(clube, divisão,
/// _)`: a competição do clube (+0x57) vira a divisão e a anterior vai para +0x5b; as cláusulas de rebaixamento (0x4cd030
/// push antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8); se a divisão não é jogada (sem a tabela das
/// competições [0xadadfc], sem divisão ou sem a competição do id dela) e o clube tem técnico humano que decide
/// (0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl), o técnico sai (0x69b790 push 0; push
/// 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10).
unsafe extern "thiscall" fn liga_rebaixar_clube(_this: usize, clube: usize, divisao: usize, _x: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let antiga = cqh5_d(clube + 0x57);
    core::ptr::write_unaligned((clube + 0x57) as *mut u32, divisao as u32);
    core::ptr::write_unaligned((clube + 0x5b) as *mut u32, antiga as u32);
    clausulas(cqh5_d(0xae1050), clube, antiga);
    let comps = cqh5_d(0xadadfc);
    if comps != 0 && divisao != 0 && cqh5_d(comps + cqh5_d(divisao) * 4) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 {
        crate::comparacao::ramo("cqh_5 técnico sai");
        tirar(cqh5_d(0xb67a44), clube, 1, 7, 0);
    }
}

/// Só a original (cláusulas de contrato e o técnico que sai não rodam duas vezes), conferindo o que ela grava: +0x57 =
/// a divisão e +0x5b = a competição de antes.
unsafe extern "thiscall" fn comparar_liga_rebaixar_clube(this: usize, clube: usize, divisao: usize, x: u32) {
    let antiga = cqh5_d(clube + 0x57);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I1%%)(this, clube, divisao, x);
    PLACAR.registrar(cqh5_d(clube + 0x57) == divisao && cqh5_d(clube + 0x5b) == antiga, || format!("liga_rebaixar_clube: +0x57 {:#x} +0x5b {:#x}", cqh5_d(clube + 0x57), cqh5_d(clube + 0x5b)));
}

/// { 0x689c20 } (thiscall, ret 4, league.cpp): `League::process_promotion_relegation_chain(aos_pares)`, do fim da
/// temporada: só na liga do topo (+0x1c < 0): as posições finais dela (método +0xa4) e, descendo a cadeia das ligas de
/// baixo (+0x20, o índice em [0xadadfc]), as posições finais de cada uma (método +0xa4) e a troca dos rebaixados e
/// promovidos entre a de cima e ela (0x689c80: push −1; push −1; push aos_pares; push 1; push abaixo; push acima; mov
/// ecx, this -> thiscall, ret 0x18).
unsafe extern "thiscall" fn liga_cadeia_de_promocoes(this: usize, aos_pares: u32) {
    if (cqh5_d(this + 0x1c) as i32) >= 0 {
        return;
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, i32) = core::mem::transmute(0x689c80usize);
    let posicoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqh5_d(cqh5_d(this) + 0xa4));
    posicoes(this);
    let mut acima = this;
    let mut i = cqh5_d(this + 0x20) as i32;
    while i >= 0 {
        let abaixo = cqh5_d(cqh5_d(0xadadfc) + i as usize * 4);
        let posicoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqh5_d(cqh5_d(abaixo) + 0xa4));
        posicoes(abaixo);
        crate::comparacao::ramo("cqh_5 troca entre ligas");
        trocar(this, acima, abaixo, 1, aos_pares, -1, -1);
        i = cqh5_d(abaixo + 0x20) as i32;
        acima = abaixo;
    }
}

/// Só a original (as posições finais e as trocas entre ligas não rodam duas vezes): a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_liga_cadeia_de_promocoes(this: usize, aos_pares: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%)(this, aos_pares)
}
