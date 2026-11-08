// ---------- %%LEVA%%a leva: a comissão do clube com a do time parceiro: o auxiliar, os treinadores e a contagem dos treinadores (Database.cpp) ----------

/// { 0x53cb80 } (cdecl, Database.cpp): `database_get_club_assistant(clube)`: o auxiliar (+0xd3) do clube; sem ele, o
/// do time parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro); nenhum: 0.
/// Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos treinadores (0x8b2880, nas
/// contratações e transferências), pela regeneração (0x7a65d0) e pelas telas.
unsafe extern "C" fn database_get_club_assistant(clube: usize) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> usize>(%%I0%%)(clube);
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let a = rd(clube + 0xd3);
    if a != 0 {
        return a;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, 0, 1);
    if p == 0 {
        return 0;
    }
    rd(p + 0xd3) // o auxiliar do parceiro
}

unsafe extern "C" fn comparar_database_get_club_assistant(clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(%%I0%%);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube), || database_get_club_assistant(clube));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_assistant({:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// { 0x53cc30 } (cdecl, Database.cpp): `database_get_club_coaches(clube, vetor)`: os 5 treinadores (+0x19f) do clube no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): as 5 casas como estão. Com parceiro: os não nulos do clube e
/// depois os do parceiro até 5; o resto zerado só quando os dois juntos não enchem as 5. Contador de 8 bits na
/// original (no máximo 5). Devolve 1. Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos
/// treinadores (0x8b2880).
unsafe extern "C" fn database_get_club_coaches(clube: usize, vetor: *mut usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut usize) -> u32>(%%I1%%)(clube, vetor);
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let p = parceiro(clube, 0, 1);
    if p == 0 {
        for j in 0..5 {
            core::ptr::write_unaligned(vetor.add(j), rd(clube + 0x19f + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..5 {
        let t = rd(clube + 0x19f + 4 * j);
        if t != 0 {
            core::ptr::write_unaligned(vetor.add(n), t);
            n += 1;
        }
    }
    for j in 0..5 {
        if n >= 5 {
            return 1;
        }
        let t = rd(p + 0x19f + 4 * j); // os treinadores do parceiro
        if t != 0 {
            core::ptr::write_unaligned(vetor.add(n), t);
            n += 1;
        }
    }
    for k in n..5 {
        core::ptr::write_unaligned(vetor.add(k), 0);
    }
    1
}

unsafe extern "C" fn comparar_database_get_club_coaches(clube: usize, vetor: *mut usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut usize) -> u32>(%%I1%%);
    if clube == 0 {
        return o(clube, vetor); // caixa: só a original
    }
    // o vetor e a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(vetor as usize, 20), (clube + 0x244, 1)],
        || o(clube, vetor), || database_get_club_coaches(clube, vetor));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_coaches({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// { 0x53c8d0 } (cdecl, Database.cpp): `database_count_club_coaches(clube)`: quantos dos 5 treinadores (+0x19f) do
/// clube são pessoas válidas (não nulas, id (+0) < [0xae2364], com sinal) mais os do time parceiro A/B (0x540a50 com
/// nulo e 1, sempre chamada). Só `al` (o resto do eax é o do parceiro). Clube nulo: a original (caixa "Error" e 0).
/// Na carreira, pela fila da regeneração (PLAYER_REGEN::process_regen_queue 0x7a65d0).
unsafe extern "C" fn database_count_club_coaches(clube: usize) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u8>(%%I2%%)(clube);
    }
    let contar = |c: usize| {
        let total = *(0xae2364 as *const i32);
        (0..5).filter(|&j| {
            let t = core::ptr::read_unaligned((c + 0x19f + 4 * j) as *const usize);
            t != 0 && *(t as *const i32) < total // treinador válido
        }).count() as u8
    };
    let mut n = contar(clube);
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let p = parceiro(clube, 0, 1);
    if p != 0 {
        n = n.wrapping_add(contar(p)); // e os do parceiro
    }
    n
}

unsafe extern "C" fn comparar_database_count_club_coaches(clube: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(%%I2%%);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube), || database_count_club_coaches(clube));
    PLACAR.registrar(igual && r == n, || format!("database_count_club_coaches({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
