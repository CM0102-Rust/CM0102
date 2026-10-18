// ---------- centesima sexagesima primeira leva: lista de observacao, codigo da posicao e criterio da comissao ----------

/// { 0x844300 } (stdcall, ret 8, shortlist_manager.cpp): o jogador `j` (ficha em [j+0x61]) serve
/// para a vaga `vaga` 0..15 da lista de observação: 0..14 pedem nota 15 nas posições da vaga
/// (setor e lado: +0xf..+0x16 e +0x17..+0x19); 15 pede ficha de contrato ([j+0x69]), id abaixo de
/// [0xae2364]−16 e, com treinador em [j+0x39] de função 11, 13 ou 15 em +0x3d, a 0x600a50 em zero.
/// Sem ficha (fora a vaga 15): 0. Vaga fora de 0..15: a original (caixa de erro). Devolve só `al`.
unsafe extern "stdcall" fn serve_para_a_vaga(j: usize, vaga: i8) -> u8 {
    let ficha = core::ptr::read_unaligned((j + 0x61) as *const usize);
    if ficha == 0 && vaga != 15 {
        return 0;
    }
    if !(0..=15).contains(&vaga) {
        return orig::<unsafe extern "stdcall" fn(usize, i8) -> u8>(355)(j, vaga);
    }
    let n = |o: usize| *((ficha + o) as *const i8) >= 15;
    // (setor, lado): 0 = so o setor
    let (setor, lado): (usize, usize) = match vaga {
        0 => (0xf, 0),
        1 => (0x10, 0x19),
        2 => (0x11, 0x18),
        3 => (0x11, 0x17),
        4 => (0x11, 0x19),
        5 => (0x16, 0x18),
        6 => (0x12, 0x19),
        7 => (0x16, 0x17),
        8 => (0x13, 0x18),
        9 => (0x13, 0x19),
        10 => (0x13, 0x17),
        11 => (0x14, 0x18),
        12 => (0x14, 0x19),
        13 => (0x14, 0x17),
        14 => (0x15, 0),
        _ => {
            let tecnico = core::ptr::read_unaligned((j + 0x39) as *const usize);
            if tecnico != 0 && matches!(*((j + 0x3d) as *const u8), 0xb | 0xf | 0xd) {
                let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
                if f(tecnico, 0, 1, 0, 0) != 0 {
                    return 0;
                }
            }
            let contrato = core::ptr::read_unaligned((j + 0x69) as *const usize);
            return (contrato != 0 && *(j as *const i32) < (*(0xae2364 as *const i32)).wrapping_sub(0x10)) as u8;
        }
    };
    (n(setor) && (lado == 0 || n(lado))) as u8
}

/// { 0x83bd00 } (stdcall, ret 4, shortlist_manager.cpp): o código da posição (0..13, 0x11..0x15)
/// da máscara `m` de um bit de setor com um de lado (0x80, 0x100, 0x200, 0x400, 0x800); fora das
/// combinações conhecidas, 0xe. Devolve só `al`.
unsafe extern "stdcall" fn codigo_da_posicao(m: i16) -> u8 {
    match m {
        1 => 0,
        2 => 1,
        4 => 0x11,
        8 => 0x13,
        0x10 => 0x12,
        0x20 => 0x15,
        0x40 => 0x14,
        0x84 => 2,
        0x88 => 5,
        0x90 => 8,
        0xa0 => 0xb,
        0x104 | 0x204 | 0x404 => 4,
        0x108 | 0x208 | 0x408 => 6,
        0x110 | 0x210 | 0x410 => 9,
        0x120 | 0x220 | 0x420 => 0xc,
        0x804 => 3,
        0x808 => 7,
        0x810 => 0xa,
        0x820 => 0xd,
        _ => 0xe,
    }
}

/// { 0x866f60 } (thiscall, staff_contracts.cpp): o critério da comissão técnica `t` (pessoa [t] em
/// [0xae23bc], 0x6e bytes; clube [t+4] em [0xae23b4], 0x245 bytes; funções em [t+0x45]): 0 se a
/// pessoa é o técnico ou o auxiliar do clube (+0xcf, +0xd3) ou não tem função; senão uma nota em 8
/// bits (id mod 7 + +0x59 + +0x5a + +0x5b, descontos pelas notas baixas e pela idade, +0x44 em
/// faixas, a reputação do clube, e menos o número de funções acima de uma) decide pelas funções e
/// pelas notas se devolve 1.
unsafe extern "thiscall" fn criterio_da_comissao(t: usize) -> u32 {
    let p = (*(0xae23bc as *const usize)).wrapping_add((*(t as *const i32) as usize).wrapping_mul(0x6e));
    let clube = (*(0xae23b4 as *const usize)).wrapping_add((*((t + 4) as *const i32) as usize).wrapping_mul(0x245));
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let tecnico = u(clube + 0xcf);
    if tecnico != 0 && p == tecnico {
        return 0;
    }
    let auxiliar = u(clube + 0xd3);
    if auxiliar != 0 && p == auxiliar {
        return 0;
    }
    let funcoes = u(t + 0x45) as u32;
    if funcoes == 0 {
        return 0;
    }
    let b = |o: usize| *((p + o) as *const i8);
    let (b59, b5a, b5b, b5d) = (b(0x59), b(0x5a), b(0x5b), b(0x5d));
    let b44 = *((t + 0x44) as *const i8);
    let mut nota = ((*(p as *const i32) % 7) as u8).wrapping_add(b59 as u8).wrapping_add(b5a as u8).wrapping_add(b5b as u8);
    if b5a == 1 {
        nota = nota.wrapping_sub(2);
    }
    if b59 == 1 {
        nota = nota.wrapping_sub(3);
    } else if b59 < 6 {
        nota = nota.wrapping_sub(1);
    }
    if b5b == 1 {
        nota = nota.wrapping_sub(3);
    } else if b5b < 6 {
        nota = nota.wrapping_sub(1);
    }
    if b5d == 1 {
        nota = nota.wrapping_sub(2);
    }
    if b44 > 0x4b {
        nota = nota.wrapping_add(10);
    } else if b44 > 0x32 {
        nota = nota.wrapping_add(5);
    } else if b44 > 0 {
        nota = nota.wrapping_add(2);
    }
    let ficha = u(p + 0x61);
    if ficha != 0 {
        let rep = core::ptr::read_unaligned((ficha + 0xb) as *const i16);
        if rep > 0x1c52 {
            nota = nota.wrapping_sub(5);
        } else if rep > 0x1676 {
            nota = nota.wrapping_sub(3);
        }
    }
    let tecnica = *((t + 0x40) as *const u32) & 8 != 0;
    let m1 = funcoes & 0x1000002;
    let mut k = 0u8;
    for bits in [0x20000u32, 0x40000, 0x1000002, 0x200600, 0x2840, 0x14] {
        if funcoes & bits != 0 {
            k += 1;
        }
    }
    if tecnica || funcoes & 0x4582208 != 0 {
        k += 1;
    }
    if k > 1 {
        nota = nota.wrapping_sub(3);
        if b5b < 6 {
            nota = nota.wrapping_sub(k);
        }
        if b5d < 6 {
            nota = nota.wrapping_sub(k);
        }
        if b59 == 1 {
            nota = nota.wrapping_add((k as i8).wrapping_mul(-2) as u8);
        } else if b59 < 6 {
            nota = nota.wrapping_sub(k);
        }
        if b5a < 6 {
            nota = nota.wrapping_sub(k);
        }
    }
    let nota = nota as i8;
    let sim = (b59 < 15 && *((t + 0x4e) as *const u8) & 1 != 0)
        || (m1 != 0 && b59 < 10 && nota < 25)
        || (funcoes & 0x20020 != 0 && nota < 25)
        || (funcoes & 0x200 != 0 && nota < 18)
        || (funcoes & 4 != 0 && b(0x57) > 15 && b5b < 10)
        || (funcoes & 0x2808 != 0 && b59 < 10 && b5a < 6 && nota < 30)
        || ((funcoes & 0x82200 != 0 || tecnica) && b5b < 10 && nota < 20)
        || funcoes & 0x111 != 0
        || (b44 as i32) < -0x46 - b59 as i32;
    sim as u32
}

unsafe extern "stdcall" fn comparar_serve_para_a_vaga(j: usize, vaga: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, i8) -> u8>(355)(j, vaga);
    let n = serve_para_a_vaga(j, vaga);
    PLACAR.registrar(r == n, || format!("serve_para_a_vaga({:#x}, {}) original={} rust={}", j, vaga, r, n));
    r
}

unsafe extern "stdcall" fn comparar_codigo_da_posicao(m: i16) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i16) -> u8>(356)(m);
    let n = codigo_da_posicao(m);
    PLACAR.registrar(r == n, || format!("codigo_da_posicao({:#x}) original={:#x} rust={:#x}", m, r, n));
    r
}

comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);
