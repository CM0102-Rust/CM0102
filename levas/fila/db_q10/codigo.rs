// ---------- %%LEVA%%a leva: a contagem e a lista do elenco do clube e se o clube pode ser dirigido (Database.cpp) ----------

/// Um dos 50 lugares do elenco (+0xd7, ponteiros de pessoa) do `clube`.
unsafe fn squad_slot(clube: usize, i: usize) -> usize {
    core::ptr::read_unaligned((clube + 0xd7 + 4 * i) as *const usize)
}

/// A pessoa do elenco conta: não nula e com id abaixo de [0xae2364] (com sinal).
unsafe fn is_valid_squad_person(p: usize) -> bool {
    p != 0 && *(p as *const i32) < *(0xae2364 as *const i32)
}

/// { 0x53c590 } (cdecl, Database.cpp): quantas pessoas válidas há no elenco do `clube` (byte). Com
/// parceiro A/B (0x540a50 com 1): `modo` 2 e clube A conta só o parceiro; senão conta o clube e, fora
/// `modo` 1 com clube B, também o parceiro. Sem parceiro e com time reserva (0x542740): só as pessoas
/// com bits `modo` nas marcas de elenco (+0x5e), se `modo` não for 0. Com `filtro`, quem tem ficha
/// de contrato (0x4cf6d0) e clube (+0x39) diferente do `clube` só conta se o clube do contrato
/// (0x4cf5a0, guardado em [0xae1080]; +4 = id) também for outro. Clube nulo: a original (caixa).
unsafe extern "C" fn database_count_club_squad(clube: usize, modo: u32, filtro: u32) -> u8 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u8>(%%I0%%)(clube, modo, filtro);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let tem_reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x542740usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let modo = modo as u8;
    let contar = |c: usize, marcas: u8| -> u8 {
        let mut n = 0u8;
        for i in 0..50 {
            let p = squad_slot(c, i);
            if !is_valid_squad_person(p) || (marcas != 0 && *((p + 0x5e) as *const u8) & marcas == 0) {
                continue;
            }
            if filtro != 0 && ficha(*(0xae1050 as *const usize), p) != 0
                && core::ptr::read_unaligned((p + 0x39) as *const usize) != clube {
                let k = contrato(*(0xae1050 as *const usize), p);
                *(0xae1080 as *mut usize) = k;
                let dono = if k == 0 {
                    0
                } else {
                    (*(0xae23b4 as *const usize)).wrapping_add((core::ptr::read_unaligned((k + 4) as *const u32) as usize).wrapping_mul(581))
                };
                if dono == clube {
                    continue;
                }
            }
            n = n.wrapping_add(1);
        }
        n
    };
    // ponytail: o original passa o endereço de um local; 0x540a50 só devolve parceiro depois de gravá-lo
    let mut eh_a = 0u32;
    let p = parceiro(clube, &mut eh_a, 1);
    if p == 0 {
        if tem_reservas(clube) != 0 {
            return contar(clube, modo);
        }
        return contar(clube, 0);
    }
    if modo == 2 && eh_a != 0 {
        return contar(p, 0);
    }
    let n = contar(clube, 0);
    if modo == 1 && eh_a == 0 {
        return n;
    }
    n.wrapping_add(contar(p, 0))
}

/// { 0x541f90 } (cdecl, Database.cpp): copia o elenco do `clube` (50 ponteiros em +0xd7) para o
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): os 50 como estão e, com `dobro`, mais 50 zeros.
/// Com parceiro: só as pessoas válidas do clube e depois as do parceiro até o limite (100 com
/// `dobro`, senão 50), o resto até o limite zerado. Devolve 1. Clube nulo: a original (caixa e 0).
unsafe extern "C" fn database_collect_club_squad(clube: usize, vetor: *mut usize, dobro: u32) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(%%I1%%)(clube, vetor, dobro);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let limit: i8 = if dobro != 0 { 100 } else { 50 };
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p == 0 {
        for i in 0..50 {
            core::ptr::write_unaligned(vetor.add(i), squad_slot(clube, i));
        }
        if dobro != 0 {
            for i in 50..100 {
                core::ptr::write_unaligned(vetor.add(i), 0);
            }
        }
        return 1;
    }
    let mut n: i8 = 0;
    for i in 0..50 {
        let q = squad_slot(clube, i);
        if is_valid_squad_person(q) {
            core::ptr::write_unaligned(vetor.add(n as usize), q);
            n += 1;
        }
    }
    for i in 0..50 {
        if n >= limit {
            break;
        }
        let q = squad_slot(p, i);
        if is_valid_squad_person(q) {
            core::ptr::write_unaligned(vetor.add(n as usize), q);
            n += 1;
        }
    }
    for k in n..limit {
        core::ptr::write_unaligned(vetor.add(k as usize), 0);
    }
    1
}

/// { 0x544820 } (cdecl, Database.cpp): o `clube` pode ser dirigido? Precisa de nação (+0x53). Seleções
/// (id ≥ [0xae235c] − 2 × [0xae2350], com sinal): só as principais (id < [0xae235c] − [0xae2350]) de
/// nação com +0x88 > 150. Clubes: com competição (+0x57), que não sejam o time B de um par (0x540a50
/// com 1), com a competição no vetor [[0xadadfc] + id × 4] e a nação com o bit 2 de +0x11c.
unsafe extern "C" fn club_can_be_managed(clube: usize) -> u8 {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let u = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    if clube == 0 || u(0x53) == 0 {
        return 0;
    }
    let id = *(clube as *const i32);
    let (n, total) = (*(0xae2350 as *const i32), *(0xae235c as *const i32));
    if id >= total.wrapping_sub(n.wrapping_add(n)) {
        return (id < total.wrapping_sub(n) && core::ptr::read_unaligned((u(0x53) + 0x88) as *const i32) > 0x96) as u8;
    }
    if u(0x57) == 0 {
        return 0;
    }
    // ponytail: o original usa o lugar do próprio argumento como saída; só é lido com parceiro, e aí já foi gravado
    let mut eh_a = 0u32;
    if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
        return 0; // time B
    }
    let tabela = *(0xadadfc as *const usize);
    if tabela == 0 {
        return 0;
    }
    let comp = *(u(0x57) as *const i32);
    if *((tabela as isize).wrapping_add((comp as isize).wrapping_mul(4)) as *const u32) == 0 {
        return 0;
    }
    (*((u(0x53) + 0x11c) as *const u8) & 2 != 0) as u8
}

unsafe extern "C" fn comparar_database_count_club_squad(clube: usize, modo: u32, filtro: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u8>(%%I0%%);
    if clube == 0 {
        return o(clube, modo, filtro); // caixa: só a original
    }
    // 0x540a50 pode zerar +0x244 do clube; o filtro grava o último contrato em [0xae1080]
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(clube + 0x244, 1), (0xae1080, 4)],
        || o(clube, modo, filtro), || database_count_club_squad(clube, modo, filtro));
    SCOREBOARD.record(igual && r == n, || format!("database_count_club_squad({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", clube, modo as u8, filtro, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_collect_club_squad(clube: usize, vetor: *mut usize, dobro: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(%%I1%%);
    if clube == 0 {
        return o(clube, vetor, dobro); // caixa: só a original
    }
    let regioes = [(vetor as usize, if dobro != 0 { 400 } else { 200 }), (clube + 0x244, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(clube, vetor, dobro),
        || database_collect_club_squad(clube, vetor, dobro));
    SCOREBOARD.record(igual && r == n, || format!("database_collect_club_squad({:x}, {}) original={} rust={} {:x?} x {:x?}", clube, dobro, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_club_can_be_managed(clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%);
    let marca = [(clube.wrapping_add(0x244), 1)];
    let regioes = if clube == 0 { &marca[..0] } else { &marca[..] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(regioes, || o(clube), || club_can_be_managed(clube));
    SCOREBOARD.record(igual && r == n, || format!("club_can_be_managed({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
