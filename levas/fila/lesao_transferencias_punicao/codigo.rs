// ---------- %%LEVA%%a leva: a recuperacao da lesao, a pessoa na lista de transferencias e a punicao da disciplina ----------

/// { 0x62bd10 } (thiscall, ret 0xc, injury.cpp): o dia de recuperação da pessoa `p` no registro de
/// 31 bytes (a partir de [this]): +0xa recebe `marca`; a palavra +0xc, abaixo de −1000, sobe
/// trunc((v × −0.0035 − 2.5) × `ritmo`); senão, abaixo de 1200 − (ficha +0x44 + ficha +0x3c +
/// pessoa +0x5b, 16 bits), sobe `ritmo`; senão, abaixo de 1250, sobe sorteio(`ritmo`) + 1.
/// Pessoa nula ou id fora de [0xae2364]: nada.
unsafe extern "thiscall" fn recover_injury(t: usize, p: *const i32, marca: i16, ritmo: i16) {
    if p.is_null() || *p >= *(0xae2364 as *const i32) {
        return;
    }
    let r = (*(t as *const usize)).wrapping_add((*p as usize).wrapping_mul(31));
    core::ptr::write_unaligned((r + 0xa) as *mut i16, marca);
    let e = (r + 0xc) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        let x = (v as f64 * read_f64_at(0x96bac0) - read_f64_at(0x968e18)) * ritmo as f64;
        core::ptr::write_unaligned(e, (x as i32).wrapping_add(v as i32) as i16);
        return;
    }
    let ficha = core::ptr::read_unaligned((p as usize + 0x61) as *const usize);
    let b = |a: usize| *(a as *const i8) as i16;
    let soma = b(ficha + 0x44).wrapping_add(b(ficha + 0x3c)).wrapping_add(b(p as usize + 0x5b));
    if (v as i32) < 0x4b0 - soma as i32 {
        core::ptr::write_unaligned(e, v.wrapping_add(ritmo));
    } else if v < 0x4e2 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let s = sorteio(ritmo as i32).wrapping_add(1) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_add(s));
    }
}

/// { 0x8c1870 } (thiscall, ret 4, transfer_manager.cpp): a pessoa `*p` está na lista +0x92c (recomeça
/// o iterador +0xa de +2 e anda pela 0x68d1b0, comparando o primeiro dword de cada item)?
unsafe extern "thiscall" fn person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let lista = t + 0x92c;
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((t + 0x92e) as *const u32));
    loop {
        let e = proximo(lista);
        if e == 0 {
            return 0;
        }
        if *p == *(e as *const i32) {
            return 1;
        }
    }
}

/// { 0x55cae0 } (thiscall, ret 0xc, discipline.cpp): a punição da linha `linha` (29 bytes) e da coluna
/// `coluna` (5 bytes) da tabela [this+0x14]: o dword +1 se o byte +0 for `tipo`, senão −1.
unsafe extern "thiscall" fn table_penalty(t: usize, linha: i32, coluna: i8, tipo: i8) -> i32 {
    let e = (*((t + 0x14) as *const usize))
        .wrapping_add((linha as usize).wrapping_mul(29))
        .wrapping_add((coluna as isize * 5) as usize);
    if *(e as *const i8) == tipo {
        core::ptr::read_unaligned((e + 1) as *const i32)
    } else {
        -1
    }
}

unsafe extern "thiscall" fn comparar_recover_injury(t: usize, p: *const i32, marca: i16, ritmo: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, i16, i16)>(%%I0%%);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if !p.is_null() && *p < *(0xae2364 as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((*p as usize).wrapping_mul(31)).wrapping_add(0xa), 4));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, p, marca, ritmo), || crate::rng::without_counting(|| recover_injury(t, p, marca, ritmo)));
    PLACAR.record(igual, || format!("recover_injury({:#x}, {:p}, {}, {}) {:x?} x {:x?}", t, p, marca, ritmo, mo, mr));
}

unsafe extern "thiscall" fn comparar_person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(%%I1%%);
    let reg = [(t + 0x92c + 0xa, 4)];
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(t, p), || person_in_transfer_list(t, p));
    PLACAR.record(igual && r == v, || format!("person_in_transfer_list({:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, p, r, mo, v, mr));
    r
}

comparar_valor!(comparar_table_penalty, table_penalty, %%I2%%, (linha: i32, coluna: i8, tipo: i8) -> i32);
