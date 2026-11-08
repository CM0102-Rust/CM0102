// ---------- %%LEVA%%a leva: as listas de reserva do gerador de jogadores e o prêmio da liga (player_regen.cpp, league.cpp) ----------

/// O comum das duas: tira da lista de registros de 16 bytes ([regen+`lista`], quantos em
/// [regen+`quantos`]) o de chave `id` (+0), puxa os seguintes uma vaga para trás, marca o último com −1
/// em +0, +4 e +8 e diminui a conta.
unsafe fn remove_from_list16(regen: usize, id: i32, lista: usize, quantos: usize) {
    let n = || *((regen + quantos) as *const i32);
    let base = || *((regen + lista) as *const usize);
    let Some(i) = (0..n().max(0) as usize).find(|&i| *((base() + i * 16) as *const i32) == id) else {
        return;
    };
    let mut j = i;
    while (j as i32) < n() - 1 {
        core::ptr::copy((base() + (j + 1) * 16) as *const u8, (base() + j * 16) as *mut u8, 16);
        j += 1;
    }
    let ultimo = (base() as isize + (n() as isize - 1) * 16) as usize;
    for o in [0, 4, 8] {
        *((ultimo + o) as *mut i32) = -1;
    }
    *((regen + quantos) as *mut i32) -= 1;
}

/// { 0x7a82b0 } (thiscall, ret 4, player_regen.cpp): tira o registro de reserva de não jogador (técnicos
/// e afins) de id `id` (lista +0x1c, quantos +0x20).
unsafe extern "thiscall" fn remove_spare_non_player_record(regen: usize, id: i32) {
    remove_from_list16(regen, id, 0x1c, 0x20);
}

/// { 0x7a8470 } (thiscall, ret 4, player_regen.cpp): tira do banco de jogadores livres o registro dos
/// dados de jogador `id` (lista +0x3c, quantos +0x40).
unsafe extern "thiscall" fn remove_player_data_from_pool(regen: usize, id: i32) {
    remove_from_list16(regen, id, 0x3c, 0x40);
}

/// { 0x682f70 } (thiscall, ret 4, league.cpp): o prêmio total da liga: `cota` para o último colocado e
/// uma cota a mais por posição acima (cota × (1 + 2 + … + times), times = +0x3e). Sem times: a
/// original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn calc_prize_fund(liga: usize, cota: i32) -> i32 {
    let times = *((liga + 0x3e) as *const i16);
    if times == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(%%I2%%)(liga, cota);
    }
    let mut total = 0i32;
    let mut parcela = cota;
    for _ in 0..times.max(0) {
        total = total.wrapping_add(parcela);
        parcela = parcela.wrapping_add(cota);
    }
    total
}

/// As regiões das duas listas: a lista inteira e o contador.
unsafe fn list16_regions(regen: usize, lista: usize, quantos: usize) -> Vec<(usize, usize)> {
    let n = (*((regen + quantos) as *const i32)).max(0) as usize;
    let mut r = vec![(regen + quantos, 4)];
    if n > 0 {
        r.push((*((regen + lista) as *const usize), n * 16));
    }
    r
}

unsafe extern "thiscall" fn compare_remove_spare_non_player_record(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&list16_regions(regen, 0x1c, 0x20), || o(regen, id), || remove_spare_non_player_record(regen, id));
    SCOREBOARD.record(igual, || format!("remove_spare_non_player_record({}) {:x?} x {:x?}", id, mo, mr));
}

unsafe extern "thiscall" fn compare_remove_player_data_from_pool(regen: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I1%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&list16_regions(regen, 0x3c, 0x40), || o(regen, id), || remove_player_data_from_pool(regen, id));
    SCOREBOARD.record(igual, || format!("remove_player_data_from_pool({}) {:x?} x {:x?}", id, mo, mr));
}

unsafe extern "thiscall" fn compare_calc_prize_fund(liga: usize, cota: i32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(%%I2%%)(liga, cota);
    if *((liga + 0x3e) as *const i16) == 0 {
        return r;
    }
    let n = calc_prize_fund(liga, cota);
    SCOREBOARD.record(r == n, || format!("calc_prize_fund({}) original={} rust={}", cota, r, n));
    r
}
