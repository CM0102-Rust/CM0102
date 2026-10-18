// ---------- centesima sexagesima sexta leva: o observador do humano, o humano no clube e a categoria do contrato ----------

/// { 0x7f6cd0 } (thiscall, ret 8, scout_manager.cpp): o registro de observação `qual` 0..9 do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]): o índice (id −
/// [0xae2364] + 16) × 10 + `qual`, até [this+8] − 1, com 0x2b0 bytes cada a partir de [this].
/// `qual` fora, sem técnico ou índice fora: a original (caixa de erro).
unsafe extern "thiscall" fn registro_do_observador(t: usize, qual: i8, tecnico: *const i32) -> usize {
    let mut p = tecnico;
    if (0..=9).contains(&qual) && p.is_null() {
        let vez = *(0xb77c7a as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    if !(0..=9).contains(&qual) || p.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(370)(t, qual, tecnico);
    }
    let i = ((*p).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10)).wrapping_mul(10).wrapping_add(qual as i32);
    if i < 0 || i > (*((t + 8) as *const i32)).wrapping_sub(1) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(370)(t, qual, tecnico);
    }
    crate::comparacao::ramo("registro do observador");
    (*(t as *const usize)).wrapping_add(i as usize * 0x2b0)
}

/// { 0x600c90 } (cdecl, human_manager.cpp): o técnico humano da vez manda no `clube` (ou, nas
/// segundas seleções, na seleção principal da nação, 0x53b3d0)? Precisa ser o técnico do clube
/// (+0xcf); sem `qualquer`, não pode ter as marcas +0x9ace (0x5fe040) nem +0x9a01 no registro de
/// humanos [0xb63c98]; e então basta `forcar`, o clube ser seleção (0x53b900) ou a marca +0x9a05
/// (0x5fd480). Clube nulo: a original (caixa de erro e 0).
unsafe extern "C" fn humano_manda_no_clube(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let tecnico_9ace: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fe040usize);
    let tecnico_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let e_segunda_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let selecao_principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let mut c = clube;
    loop {
        if c == 0 {
            return orig::<unsafe extern "C" fn(usize, u32, i8) -> u32>(371)(0, qualquer, forcar);
        }
        let vez = *(0xb77c7a as *const i16) as isize;
        let humano = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const usize);
        let tecnico = core::ptr::read_unaligned((c + 0xcf) as *const usize);
        'proximo: {
            if tecnico == 0 || humano != tecnico {
                break 'proximo;
            }
            let humanos = *(0xb63c98 as *const usize);
            if qualquer == 0 {
                if tecnico_9ace(humanos, humano) != 0 {
                    break 'proximo;
                }
                let i = (*(humano as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
                if i >= 0
                    && i <= (*((humanos + 4) as *const i32)).wrapping_sub(1)
                    && core::ptr::read_unaligned((*(humanos as *const usize) + i as usize * 0xa6ef + 0x9a01) as *const u32) != 0
                {
                    break 'proximo;
                }
            }
            if forcar != 0 || e_selecao(c) != 0 || tecnico_9a05(humanos, humano) != 0 {
                crate::comparacao::ramo("humano manda no clube");
                return 1;
            }
        }
        if e_segunda_selecao(c) == 0 {
            return 0;
        }
        c = selecao_principal(core::ptr::read_unaligned((c + 0x53) as *const usize));
    }
}

/// { 0x4ca470 } (stdcall, ret 8, contract_manager.cpp): a categoria 1..8 do jogador `p` no contrato
/// `c`: sem ficha (+0x61), 7; sem contrato, 4; contrato de tipo 5 (+0x35 & 0x3f), 8; senão pela
/// situação (+0x4f >> 4, 0..6; acima, 7), a satisfação (+0x3d, com sinal), a idade (+0x18), a
/// ambição (+0x57) e a reputação da ficha (+0xb, +0xd).
unsafe extern "stdcall" fn categoria_do_contrato(p: usize, c: usize) -> u8 {
    if p == 0 {
        return 7;
    }
    let ficha = core::ptr::read_unaligned((p + 0x61) as *const usize);
    if ficha == 0 {
        return 7;
    }
    if c == 0 {
        return 4;
    }
    if *((c + 0x35) as *const u8) & 0x3f == 5 {
        return 8;
    }
    let k = *((c + 0x4f) as *const u8) >> 4;
    let v = *((c + 0x3d) as *const i8);
    let idade = *((p + 0x18) as *const i8);
    let ambicao = *((p + 0x57) as *const i8);
    let rep = |o: usize| core::ptr::read_unaligned((ficha + o) as *const i16);
    let baixo = |limite: i8| if v <= limite { 7 } else { 4 };
    match k {
        0 => {
            if idade > 0x15 {
                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { baixo(-30) }
            } else if v > 0 {
                5
            } else if v > -30 || ambicao > 5 {
                6
            } else {
                7
            }
        }
        1 => if v > 0x2d { 1 } else { (v <= 0x19) as u8 + 2 },
        2 => {
            let nota = ambicao as i32 * 25 - *((p + 0x59) as *const i8) as i32 * 10 + rep(0xd) as i32;
            if v > 0x4b && nota > 0x186a && idade > 0x17 && idade < 0x22 {
                1
            } else if v > 0x19 {
                2
            } else {
                (v <= 0) as u8 + 3
            }
        }
        3 => if v > 0x37 && idade > 0x17 && idade < 0x22 { 2 } else { (v <= 0) as u8 + 3 },
        4 => {
            if v > 0x41 && idade > 0x17 {
                (idade < 0x22) as u8 + 2
            } else if v > -30 {
                4
            } else if idade >= 0x18 {
                7
            } else {
                5
            }
        }
        5 => {
            if idade > 0x18 {
                if idade > 0x22 { 4 } else if v > 0x4b { 2 } else if v > 0 { 3 } else { baixo(-30) }
            } else if ambicao > 5 && rep(0xb) > 0x4e2 {
                5
            } else {
                6
            }
        }
        6 => {
            if idade > 0x17 {
                if v > 0x4b { 1 } else if v > 0x1e { 2 } else if v > 0 { 3 } else { baixo(-50) }
            } else if rep(0xb) > 0x4e2 && ambicao > 0xf {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

unsafe extern "thiscall" fn comparar_registro_do_observador(t: usize, qual: i8, tecnico: *const i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(370)(t, qual, tecnico);
    let n = registro_do_observador(t, qual, tecnico);
    PLACAR.registrar(r == n, || format!("registro_do_observador({:#x}, {}, {:p}) original={:#x} rust={:#x}", t, qual, tecnico, r, n));
    r
}

unsafe extern "C" fn comparar_humano_manda_no_clube(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32, i8) -> u32>(371)(clube, qualquer, forcar);
    let n = humano_manda_no_clube(clube, qualquer, forcar);
    PLACAR.registrar(r == n, || format!("humano_manda_no_clube({:#x}, {}, {}) original={} rust={}", clube, qualquer, forcar, r, n));
    r
}

unsafe extern "stdcall" fn comparar_categoria_do_contrato(p: usize, c: usize) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u8>(372)(p, c);
    let n = categoria_do_contrato(p, c);
    PLACAR.registrar(r == n, || format!("categoria_do_contrato({:#x}, {:#x}) original={} rust={}", p, c, r, n));
    r
}
