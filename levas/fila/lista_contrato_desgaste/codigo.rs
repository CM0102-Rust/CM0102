// ---------- %%LEVA%%a leva: a pessoa na lista do tecnico, a ficha de contrato e o desgaste diario ----------

/// { 0x69fc60 } (thiscall, ret 4, manager_manager.cpp): a pessoa `*p` está na lista [this+8]
/// (palavra da quantidade em +0, iterador 0x68d1b0 com posição em +0xa, recomeçada de +2 e
/// devolvida no fim)? Lista que acaba antes da quantidade: a original (caixa de erro). Devolve só `al`.
unsafe extern "thiscall" fn person_in_list(t: usize, p: *const i32) -> u8 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let lista = *((t + 8) as *const usize);
    let guardado = core::ptr::read_unaligned((lista + 0xa) as *const u32);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut i = 0i32;
    while i < *(lista as *const i16) as i32 {
        let e = proximo(lista);
        if e == 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, guardado);
            return original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(%%I0%%)(t, p);
        }
        if core::ptr::read_unaligned((e + 6) as *const i32) == *p {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, guardado);
            return 1;
        }
        i += 1;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, guardado);
    0
}

/// { 0x4cf6d0 } (thiscall, ret 4, contract_manager.cpp): a ficha de contrato (0x50 bytes, a partir de
/// [this]) da pessoa `*p`, pelo índice em +4 do registro de 79 bytes em [0xae2d0c]; 0 sem pessoa, com
/// id fora de [0xae2364] ou índice negativo.
unsafe extern "thiscall" fn contract_record(t: usize, p: *const i32) -> usize {
    if p.is_null() {
        return 0;
    }
    let id = *p;
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79)).wrapping_add(4) as *const i32);
    if k < 0 {
        return 0;
    }
    (*(t as *const usize)).wrapping_add(k as usize * 0x50)
}

/// { 0x62c2f0 } (thiscall, ret 4, injury.cpp): o desgaste do dia na palavra +0xc do registro de 31
/// bytes da pessoa `id` (a partir de [this]): abaixo de −1000 desce 1; abaixo de 19 anos (+0x18 da
/// pessoa em [0xae23bc]) e negativo, zera; senão, de −500 para cima e com idade ≥ sorteio(25), desce
/// 13, e nos outros casos desce sorteio(13); goleiro (posição +0xf da ficha = 20) fica até 1050.
/// Id fora de [0xae2364]: nada.
unsafe extern "thiscall" fn daily_wear(t: usize, id: i32) {
    if id >= *(0xae2364 as *const i32) {
        return;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let e = (*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        core::ptr::write_unaligned(e, v.wrapping_sub(1));
    }
    let p = (*(0xae23bc as *const usize)).wrapping_add((id as usize).wrapping_mul(0x6e));
    let idade = *((p + 0x18) as *const i8);
    if idade < 0x13 && core::ptr::read_unaligned(e) < 0 {
        core::ptr::write_unaligned(e, 0);
    } else if core::ptr::read_unaligned(e) >= -500 && idade as i32 >= sorteio(0x19) {
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xd));
    } else {
        let r = sorteio(0xd) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(r));
    }
    let ficha = core::ptr::read_unaligned((p + 0x61) as *const usize);
    if ficha != 0 && *((ficha + 0xf) as *const u8) == 0x14 && core::ptr::read_unaligned(e) >= 0x41a {
        core::ptr::write_unaligned(e, 0x41a);
    }
}

compare_value!(compare_person_in_list, person_in_list, %%I0%%, (p: *const i32) -> u8);
compare_value!(compare_contract_record, contract_record, %%I1%%, (p: *const i32) -> usize);

unsafe extern "thiscall" fn compare_daily_wear(t: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if id < *(0xae2364 as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc), 2));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, id), || crate::rng::without_counting(|| daily_wear(t, id)));
    SCOREBOARD.record(igual, || format!("daily_wear({:#x}, {}) {:x?} x {:x?}", t, id, mo, mr));
}
