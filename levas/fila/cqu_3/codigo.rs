// ---------- %%LEVA%%a leva: o resultado de cada jogo do Mineiro, do Paranaense, do Carioca e da J1 (a tabela geral e a do turno) (bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, bra_reg_rio.cpp, jap_j1.cpp) ----------

unsafe fn cqu3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O resultado de um jogo numa liga de dois turnos (o mesmo molde de 128 bytes nas quatro, conferido instrução a
/// instrução: só mudam a rodada em que começa o segundo turno, 11 nos estaduais e 15 na J1, e a rotina do turno).
/// 1) O jogo vai para a tabela geral (League::add_result 0x685d30, thiscall(this, jogo, estat), ret 8).
/// 2) Se o jogo ainda não tem fase (+0x42 = 0xff): a fase dele é o turno, 0 até a rodada `segundo` − 1 e 1 a partir
///    dela (a rodada é a palavra +0x34 do jogo, com sinal), gravada em +0x42; enquanto a fase atual da liga (+0x30) for
///    menor que ela, monta o turno seguinte (a rotina do turno, cqr_1: thiscall(this, +0x30 + 1), ret 4; a original
///    empurra o dword com o byte no de baixo e sobra de registrador em cima; o turno só usa o byte: a cópia passa o
///    byte limpo); então o jogo vai também para a tabela do turno (o slot +4 da fase [+0xc][fase]: thiscall(fase, jogo,
///    estat), ret 8) com o bloco de estatísticas da liga (+8) guardado e zerado (para não contar o jogo duas vezes) e
///    devolvido; a marca +0x42 do jogo volta a 0xff.
/// Devolve 1 nos dois caminhos.
unsafe fn cqu3_resultado(this: usize, jogo: usize, estat: u32, segundo: i16, turno: usize, ramo: &'static str) -> u32 {
    let geral: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x685d30usize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(turno);
    geral(this, jogo, estat);
    if *((jogo + 0x42) as *const u8) != 0xff {
        return 1;
    }
    let rodada = core::ptr::read_unaligned((jogo + 0x34) as *const i16);
    let fase: i8 = (rodada >= segundo) as i8; // cqu3: o turno do jogo
    *((jogo + 0x42) as *mut i8) = fase;
    while (cqu3_d(this + 0x30) as i32) < fase as i32 {
        montar(this, (*((this + 0x30) as *const u8)).wrapping_add(1) as u32);
    }
    crate::comparacao::ramo(ramo);
    let guardado = cqu3_d(this + 8);
    core::ptr::write_unaligned((this + 8) as *mut usize, 0);
    let f = cqu3_d(cqu3_d(this + 0xc) + 4 * (*((jogo + 0x42) as *const i8)) as i32 as usize);
    let somar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(cqu3_d(cqu3_d(f) + 4));
    somar(f, jogo, estat); // cqu3: o jogo na tabela do turno
    core::ptr::write_unaligned((this + 8) as *mut usize, guardado);
    *((jogo + 0x42) as *mut u8) = 0xff; // cqu3: a marca do jogo de volta
    1
}

/// O comparador dos quatro: só a original (o jogo entra nas tabelas e o turno pode ser montado; não roda duas vezes),
/// com o plano feito antes: a devolução 1, a marca +0x42 do jogo como estava (0xff de volta, se estava), o bloco +8 da
/// liga igual e, quando o jogo não tinha fase, a fase atual da liga (+0x30) já no turno do jogo, com a fase dele montada.
unsafe fn cqu3_comparar(nome: &str, this: usize, jogo: usize, segundo: i16, original: impl FnOnce() -> u32) -> u32 {
    let marca = *((jogo + 0x42) as *const u8);
    let bloco = cqu3_d(this + 8);
    let fase = (core::ptr::read_unaligned((jogo + 0x34) as *const i16) >= segundo) as i32;
    let r = original();
    let mut ok = r == 1 && *((jogo + 0x42) as *const u8) == marca && cqu3_d(this + 8) == bloco;
    if marca == 0xff {
        ok = ok && cqu3_d(this + 0x30) as i32 >= fase && cqu3_d(cqu3_d(this + 0xc) + 4 * fase as usize) != 0;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}, {:#x}): devolveu {}, marca {:#x} -> {:#x}, +0x30 = {}, fase do jogo {}", nome, this, jogo, r, marca, *((jogo + 0x42) as *const u8), cqu3_d(this + 0x30) as i32, fase));
    r
}

/// { 0x435130 } (thiscall, ret 8, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::add_result(jogo, estat)`: o jogo do Mineiro (o segundo turno a partir da 11ª rodada).
unsafe extern "thiscall" fn cqu3_resultado_do_mineiro(this: usize, jogo: usize, estat: u32) -> u32 {
    cqu3_resultado(this, jogo, estat, 0xb, 0x4351b0, "cqu_3 turno do mineiro") // cqu3: a rodada do segundo turno do Mineiro
}

/// { 0x4386e0 } (thiscall, ret 8, bra_reg_parana.cpp): `BraRegParana::add_result(jogo, estat)`: o jogo do Paranaense (o segundo turno a partir da 11ª rodada).
unsafe extern "thiscall" fn cqu3_resultado_do_paranaense(this: usize, jogo: usize, estat: u32) -> u32 {
    cqu3_resultado(this, jogo, estat, 0xb, 0x438760, "cqu_3 turno do paranaense") // cqu3: o Paranaense
}

/// { 0x43af20 } (thiscall, ret 8, bra_reg_rio.cpp): `BraRegRio::add_result(jogo, estat)`: o jogo do Carioca (o segundo turno a partir da 11ª rodada).
unsafe extern "thiscall" fn cqu3_resultado_do_carioca(this: usize, jogo: usize, estat: u32) -> u32 {
    cqu3_resultado(this, jogo, estat, 0xb, 0x43afa0, "cqu_3 turno do carioca") // cqu3: o Carioca
}

/// { 0x663940 } (thiscall, ret 8, jap_j1.cpp): `JapJ1::add_result(jogo, estat)`: o jogo da J1 (o segundo turno a partir da 15ª rodada).
unsafe extern "thiscall" fn cqu3_resultado_da_j1(this: usize, jogo: usize, estat: u32) -> u32 {
    cqu3_resultado(this, jogo, estat, 0xf, 0x663c20, "cqu_3 turno da j1") // cqu3: a rodada do segundo turno da J1
}

unsafe extern "thiscall" fn comparar_cqu3_resultado_do_mineiro(this: usize, jogo: usize, estat: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    cqu3_comparar("cqu3_resultado_do_mineiro", this, jogo, 0xb, || o(this, jogo, estat))
}

unsafe extern "thiscall" fn comparar_cqu3_resultado_do_paranaense(this: usize, jogo: usize, estat: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    cqu3_comparar("cqu3_resultado_do_paranaense", this, jogo, 0xb, || o(this, jogo, estat))
}

unsafe extern "thiscall" fn comparar_cqu3_resultado_do_carioca(this: usize, jogo: usize, estat: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    cqu3_comparar("cqu3_resultado_do_carioca", this, jogo, 0xb, || o(this, jogo, estat))
}

unsafe extern "thiscall" fn comparar_cqu3_resultado_da_j1(this: usize, jogo: usize, estat: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I3%%);
    cqu3_comparar("cqu3_resultado_da_j1", this, jogo, 0xf, || o(this, jogo, estat))
}
