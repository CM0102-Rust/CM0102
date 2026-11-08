// ---------- %%LEVA%%a leva: se a pessoa está na lista de transferências e os atributos de jogador de 1 a 20 (Database.cpp) ----------

/// { 0x544520 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências, pelo clube ou a pedido (vínculo +0x4e, bits 1 e 8). Sem clube, o do
/// contrato (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id do clube na tabela [0xae23b4], 581
/// bytes por clube).
unsafe extern "C" fn staff_is_transfer_listed_or_requested(pessoa: usize, clube: usize) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    if pessoa == 0 {
        return 0;
    }
    let mut clube = clube;
    if clube == 0 {
        let c = contract_of(*(0xae1050 as *const usize), pessoa);
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
    let v = contract_bond(*(0xae1050 as *const usize), pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 9 != 0) as u32
}

/// A nota de 1 a 20 de um atributo relativo (de 0x543540, 0x544380 e 0x5442f0): x = v × 0,05 + 10;
/// (x² × 0,1 + x) × 1/3 + 0,5, entre 1 e 20, truncada (`_ftol`).
unsafe fn attribute_rating(v: f64) -> u8 {
    let x = v * read_f64_at(0x967888) + read_f64_at(0x9688b0);
    let z = (x * x * read_f64_at(0x967880) + x) * read_f64_at(0x968a80) + read_f64_at(0x968968); // arredonda
    let (minimo, maximo) = (read_f64_at(0x967890), read_f64_at(0x968ea8));
    match_pl_ftol(if z > minimo {
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
/// registro de `jogador`; os relativos dão a nota (`attribute_rating`) de 2 × byte + habilidade atual
/// (+5), ou de 2 × byte + habilidade × 0,1 conforme o goleiro (+0xf: = 20 ou < 15); o 10 é o byte
/// +0x58 da `pessoa`; uns vão a 0x544380 (ou 0x5442f0). Sem pessoa, sem jogador ou atributo fora:
/// caixa "Error" e 0xff (a original). Só o al é devolvido.
unsafe extern "C" fn staff_compute_player_attribute(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%)(pessoa, jogador, atributo);
    }
    // ponytail: a original empurra o registrador inteiro com o byte embaixo; as duas só leem o byte
    let relativo: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x544380usize);
    let relativo_goleiro: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x5442f0usize);
    let b = |d: usize| *((jogador + d) as *const i8);
    let byte = |d: usize| b(d) as u8;
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as i32;
    let nota = |d: usize| attribute_rating((2 * b(d) as i32 + habilidade) as f64);
    let nota_h = |d: usize| attribute_rating(habilidade as f64 * read_f64_at(0x967880) + (2 * b(d) as i32) as f64);
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
unsafe extern "C" fn staff_get_player_attribute(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(%%I2%%)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543540usize);
    // a original empurra o argumento inteiro, como veio
    calcular(pessoa, core::ptr::read_unaligned((pessoa + 0x61) as *const usize), atributo)
}

unsafe extern "C" fn compare_staff_is_transfer_listed_or_requested(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0 e 0x4cf5f0 só leem
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(pessoa, clube),
        || staff_is_transfer_listed_or_requested(pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_transfer_listed_or_requested({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn compare_staff_compute_player_attribute(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return o(pessoa, jogador, atributo); // caixa: só a original
    }
    let r = o(pessoa, jogador, atributo);
    let n = staff_compute_player_attribute(pessoa, jogador, atributo);
    SCOREBOARD.record(r == n, || format!("staff_compute_player_attribute({:x}, {}) original={} rust={}", jogador, a, r, n));
    r
}

unsafe extern "C" fn compare_staff_get_player_attribute(pessoa: usize, atributo: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(%%I2%%);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return o(pessoa, atributo); // caixa: só a original
    }
    let r = o(pessoa, atributo);
    let n = staff_get_player_attribute(pessoa, atributo);
    SCOREBOARD.record(r == n, || format!("staff_get_player_attribute({:x}, {}) original={} rust={}", pessoa, a, r, n));
    r
}
