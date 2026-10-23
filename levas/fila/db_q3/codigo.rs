// ---------- %%LEVA%%a leva: se a pessoa está na lista de transferências e os atributos de jogador de 1 a 20 (Database.cpp) ----------

/// { 0x544520 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências, pelo clube ou a pedido (vínculo +0x4e, bits 1 e 8). Sem clube, o do
/// contrato (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id do clube na tabela [0xae23b4], 581
/// bytes por clube).
unsafe extern "C" fn pessoa_listada_para_transferencia(pessoa: usize, clube: usize) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    if pessoa == 0 {
        return 0;
    }
    let mut clube = clube;
    if clube == 0 {
        let c = contrato_de(*(0xae1050 as *const usize), pessoa);
        *(0xae1080 as *mut usize) = c;
        if c == 0 {
            return 0;
        }
        let id = core::ptr::read_unaligned((c + 4) as *const u32) as usize;
        clube = (*(0xae23b4 as *const usize)).wrapping_add(id.wrapping_mul(581));
        if clube == 0 {
            return 0;
        }
    }
    let v = vinculo(*(0xae1050 as *const usize), pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 9 != 0) as u32
}

/// A nota de 1 a 20 de um atributo relativo (de 0x543540, 0x544380 e 0x5442f0): x = v × 0,05 + 10;
/// (x² × 0,1 + x) × 1/3 + 0,5, entre 1 e 20, truncada (`_ftol`).
unsafe fn nota_do_atributo(v: f64) -> u8 {
    let x = v * f64_em(0x967888) + f64_em(0x9688b0);
    let z = (x * x * f64_em(0x967880) + x) * f64_em(0x968a80) + f64_em(0x968968); // arredonda
    let (minimo, maximo) = (f64_em(0x967890), f64_em(0x968ea8));
    ftol(if z > minimo {
        if z < maximo {
            z
        } else {
            maximo
        }
    } else {
        minimo
    }) as u8
}

/// { 0x543540 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador de 1 a 20. Uns são o byte do
/// registro de `jogador`; os relativos dão a nota (`nota_do_atributo`) de 2 × byte + habilidade atual
/// (+5), ou de 2 × byte + habilidade × 0,1 conforme o goleiro (+0xf: = 20 ou < 15); o 10 é o byte
/// +0x58 da `pessoa`; uns vão a 0x544380 (ou 0x5442f0). Sem pessoa, sem jogador ou atributo fora:
/// caixa "Error" e 0xff (a original). Só o al é devolvido.
unsafe extern "C" fn atributo_calculado_do_jogador(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%)(pessoa, jogador, atributo);
    }
    // ponytail: a original empurra o registrador inteiro com o byte embaixo; as duas só leem o byte
    let relativo: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x544380usize);
    let relativo_goleiro: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x5442f0usize);
    let b = |d: usize| *((jogador + d) as *const i8);
    let byte = |d: usize| b(d) as u8;
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as i32;
    let nota = |d: usize| nota_do_atributo((2 * b(d) as i32 + habilidade) as f64);
    let nota_h = |d: usize| nota_do_atributo(habilidade as f64 * f64_em(0x967880) + (2 * b(d) as i32) as f64);
    let goleiro = b(0xf);
    match a {
        1 => byte(0x1b),
        2 => byte(0x1c),
        3 => byte(0x1d),
        4 => nota(0x1e),
        5 => byte(0x1f),
        6 => byte(0x20),
        7 => if goleiro == 20 { nota_h(0x43) } else { nota(0x43) },
        8 => if goleiro == 20 { nota_h(0x23) } else { nota(0x23) },
        9 => nota(0x24),
        10 => *((pessoa + 0x58) as *const u8),
        11 => if goleiro == 20 { nota_h(0x26) } else { nota(0x26) },
        12 => if goleiro == 20 { relativo_goleiro(jogador, byte(0x27) as u32) } else { relativo(jogador, byte(0x27) as u32) },
        13 => byte(0x28),
        14 => if goleiro < 15 { nota_h(0x2a) } else { nota(0x2a) },
        15 => nota(0x2b),
        16 => byte(0x2f),
        17 => byte(0x2e),
        18 => relativo(jogador, byte(0x31) as u32),
        19 => if goleiro == 20 { nota_h(0x32) } else { nota(0x32) },
        20 => if goleiro == 20 { nota_h(0x33) } else { nota(0x33) },
        21 => byte(0x36),
        22 => nota(0x37),
        23 => nota(0x39),
        24 => if goleiro < 15 { nota_h(0x3a) } else { relativo(jogador, byte(0x3a) as u32) },
        25 => byte(0x29),
        26 => byte(0x3c),
        27 => byte(0x3d),
        28 => relativo(jogador, byte(0x3e) as u32),
        29 => byte(0x3f),
        30 => byte(0x40),
        _ => byte(0x44),
    }
}

/// { 0x5431e0 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador da `pessoa`: 0x543540 com os
/// dados de jogador (+0x61). Sem pessoa ou atributo fora: caixa "Error" e 0xff (a original).
unsafe extern "C" fn atributo_da_pessoa(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return orig::<unsafe extern "C" fn(usize, u32) -> u8>(%%I2%%)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543540usize);
    // a original empurra o argumento inteiro, como veio
    calcular(pessoa, core::ptr::read_unaligned((pessoa + 0x61) as *const usize), atributo)
}

unsafe extern "C" fn comparar_pessoa_listada_para_transferencia(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0 e 0x4cf5f0 só leem
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_para_transferencia(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_para_transferencia({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_atributo_calculado_do_jogador(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return o(pessoa, jogador, atributo); // caixa: só a original
    }
    let r = o(pessoa, jogador, atributo);
    let n = atributo_calculado_do_jogador(pessoa, jogador, atributo);
    PLACAR.registrar(r == n, || format!("atributo_calculado_do_jogador({:x}, {}) original={} rust={}", jogador, a, r, n));
    r
}

unsafe extern "C" fn comparar_atributo_da_pessoa(pessoa: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u8>(%%I2%%);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return o(pessoa, atributo); // caixa: só a original
    }
    let r = o(pessoa, atributo);
    let n = atributo_da_pessoa(pessoa, atributo);
    PLACAR.registrar(r == n, || format!("atributo_da_pessoa({:x}, {}) original={} rust={}", pessoa, a, r, n));
    r
}
