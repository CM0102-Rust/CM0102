// ---------- %%LEVA%%a leva: o lado do jogo, a ordem dos coeficientes das nações e o pedido de observação (comp_util.cpp, scout_manager.cpp) ----------

unsafe fn cpq12_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpq12_us(a: usize) -> usize {
    cpq12_d(a) as usize
}

/// { 0x4afa80 } (cdecl, comp_util.cpp): `comp_util_get_fixture_venue_side(clube, jogo)`: o lado do clube no jogo:
/// 1 em casa, 2 fora, 3 neutro. Sem campo no jogo (+0x18): pelo mandante (+0x1c), 3 se ele não tem estádio (+0x69),
/// senão 1 para o mandante e 2 para o outro. Seleção (0x53b900): 1 se a nação do campo (+0x38, +0x1f) é a do clube
/// (+0x53), 2 se é a do adversário, senão 3. Clube: o campo é o estádio do clube e não o do adversário, 1; dos dois,
/// 1 para o mandante e 2 para o outro; só do adversário, 2; de nenhum, 3. Quem chama lê só o al.
unsafe extern "C" fn comp_util_get_fixture_venue_side(clube: usize, jogo: usize) -> u8 {
    let campo = cpq12_d(jogo + 0x18);
    let casa = cpq12_us(jogo + 0x1c);
    if campo == 0 {
        if cpq12_d(casa + 0x69) == 0 {
            return 3;
        }
        return if clube != casa { 2 } else { 1 };
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        let nacao = cpq12_us(campo as usize + 0x38);
        if nacao != 0 && cpq12_d(nacao + 0x1f) == cpq12_d(clube + 0x53) {
            return 1;
        }
        let casa = cpq12_us(jogo + 0x1c);
        let outro = if casa == clube { cpq12_us(jogo + 0x20) } else { casa };
        if nacao != 0 && cpq12_d(nacao + 0x1f) == cpq12_d(outro + 0x53) {
            return 2;
        }
        return 3;
    }
    let casa = cpq12_us(jogo + 0x1c);
    let outro = if casa == clube { cpq12_us(jogo + 0x20) } else { casa };
    let campo = cpq12_d(jogo + 0x18);
    if campo != cpq12_d(clube + 0x69) {
        return if campo != cpq12_d(outro + 0x69) { 3 } else { 2 };
    }
    if campo == cpq12_d(outro + 0x69) {
        return if clube != casa { 2 } else { 1 }; // o mesmo estádio dos dois
    }
    crate::comparacao::ramo("cpq_12 em casa");
    1 // o estádio do clube
}

/// Comparador: pura (consulta 0x53b900, sem efeitos): pelo lado.
unsafe extern "C" fn comparar_comp_util_get_fixture_venue_side(clube: usize, jogo: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u8>(%%I0%%);
    let r = o(clube, jogo);
    let n = comp_util_get_fixture_venue_side(clube, jogo);
    PLACAR.registrar(r == n, || format!("comp_util_get_fixture_venue_side(clube {:x}, jogo {:x}) original={} rust={}", clube, jogo, r, n));
    r
}

/// { 0x4afba0 } (cdecl, comp_util.cpp): `compare_nation_coefficients_desc(a, b)`: a ordem do qsort (12 bytes por
/// entrada, o coeficiente em double em +4) da game_update_nation_coefficients 0x5d6187 (no dia do mês da tabela
/// 0x9cd200): maior antes (−1); menor ou sem ordem (NaN, a flag C0 do fcomp), 1; igual, 0. Quente (o qsort): sem
/// contador. A original ordena e libera o vetor logo depois: a ordem não chega ao jogo (só o comparando acusa).
unsafe extern "C" fn compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let x = core::ptr::read_unaligned((a + 4) as *const f64);
    let y = core::ptr::read_unaligned((b + 4) as *const f64);
    if x > y {
        return -1; // o maior antes
    }
    if !(x >= y) {
        return 1;
    }
    0
}

unsafe extern "C" fn comparar_compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%);
    let r = o(a, b);
    let n = compare_nation_coefficients_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_nation_coefficients_desc original={} rust={}", r, n));
    r
}

/// { 0x7f8d70 } (thiscall, ret 0xc, scout_manager.cpp): `SCOUT_MANAGER::has_scout_request(jogador, clube, so_vencidos)`:
/// se o clube (nulo: o do humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]] +0x39) pediu a observação do jogador: busca
/// binária (0x946464 com 0x7fc270) pelo par (id do clube, id do jogador) nos pedidos (+0x5c, conta +0x64, 17 bytes)
/// com menos de 3 relatórios (+0x10, com sinal); com `so_vencidos`, só se a data do pedido (+8: dia, +0xa: ano) é
/// anterior a hoje ([0xae2c90], ano sem sinal, dia com sinal). Devolve 1 ou 0. Jogador nulo: a original (caixa da
/// linha 0x5da). A data local da original (0x54a110) só ocupa a chave depois do par.
unsafe extern "thiscall" fn has_scout_request(this: usize, jogador: usize, clube: usize, so_vencidos: u32) -> u32 {
    if jogador == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I2%%)(this, jogador, clube, so_vencidos);
    }
    let mut clube = clube;
    if clube == 0 {
        let humano = cpq12_us(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize));
        if humano == 0 {
            return 0;
        }
        clube = cpq12_us(humano + 0x39);
        if clube == 0 {
            return 0;
        }
    }
    let lista = cpq12_us(this + 0x5c);
    let achado = if lista == 0 {
        0
    } else {
        let data: unsafe extern "thiscall" fn(*mut u8) -> *mut u8 = core::mem::transmute(0x54a110usize);
        let buscar: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        let mut chave = [0u8; 16];
        data(chave.as_mut_ptr().add(8));
        chave[4..8].copy_from_slice(&cpq12_d(jogador).to_le_bytes());
        chave[0..4].copy_from_slice(&cpq12_d(clube).to_le_bytes());
        buscar(chave.as_ptr(), lista, cpq12_d(this + 0x64), 0x11, 0x7fc270)
    };
    if achado == 0 {
        crate::comparacao::ramo("cpq_12 sem pedido");
        return 0; // sem pedido
    }
    if *((achado + 0x10) as *const i8) >= 3 {
        return 0;
    }
    if so_vencidos != 0 {
        let ano = core::ptr::read_unaligned((achado + 0xa) as *const u16);
        let hoje = *(0xae2c92 as *const u16);
        if ano > hoje {
            return 0;
        }
        if ano == hoje && core::ptr::read_unaligned((achado + 8) as *const i16) >= *(0xae2c90 as *const i16) {
            return 0;
        }
    }
    1
}

/// Comparador: consulta pura (a busca binária e a data local): pelo valor; jogador nulo só a original (a caixa).
unsafe extern "thiscall" fn comparar_has_scout_request(this: usize, jogador: usize, clube: usize, so_vencidos: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I2%%);
    if jogador == 0 {
        return o(this, jogador, clube, so_vencidos);
    }
    let r = o(this, jogador, clube, so_vencidos);
    let n = has_scout_request(this, jogador, clube, so_vencidos);
    PLACAR.registrar(r == n, || format!("has_scout_request(jogador {:x}, clube {:x}, {}) original={} rust={}", jogador, clube, so_vencidos, r, n));
    r
}
