// ---------- %%LEVA%%a leva: o progresso do treino guardado: tirar e devolver o progresso, o quanto o treino soma a um atributo e a aptidão para uma posição (training_manager.cpp) ----------

unsafe fn cqd9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd9_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
unsafe fn player_training_entry(this: usize, p: usize) -> usize {
    cqd9_d(this + 0x651d).wrapping_add((*(cqd9_d(p + 0x61) as *const i32) as isize * 6) as usize)
}

/// O meio byte com sinal da categoria `k` na entrada do treino: 0 o baixo de +3, 1 o alto de +3, 2 o baixo de +4, 3 o
/// alto de +4, 4 o baixo de +5.
unsafe fn training_nibble(e: usize, k: usize) -> i8 {
    let b = *((e + 3 + k / 2) as *const u8);
    if k % 2 == 0 {
        ((b << 4) as i8) >> 4
    } else {
        (b as i8) >> 4
    }
}

unsafe fn training_manager_apply_training_progress(this: usize, p: usize, d: &[i8; 8]) {
    let f: unsafe extern "thiscall" fn(usize, usize, *const i8) = core::mem::transmute(0x8b2be0usize);
    f(this, p, d.as_ptr());
}

/// { 0x8b3640 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::remove_or_restore_training_progress(pessoa,
/// devolver)`, pelas normalizações dos atributos do jogador (0x538bc0 e 0x538ec0: no envelhecimento do diário das
/// pessoas 0x5ce2d0, no regen jovem 0x7abac0 e na carga), que tiram o progresso antes e o devolvem depois. Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x621). Tirar (`devolver` = 0): guarda os cinco meios bytes do progresso da
/// entrada do treino em 0xde1f3c..0xde1f40 (categoria 3, 2, 4, 0, 1) e aplica o progresso negado (0x8b2be0); devolver:
/// aplica o guardado.
unsafe extern "thiscall" fn remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x621);
        return;
    }
    let e = player_training_entry(this, p);
    let mut d = [0i8; 8];
    let g = |a: usize| *(a as *const i8);
    if devolver != 0 {
        crate::comparison::branch("cqd_9 devolver");
        d[0] = g(0xde1f3f); // cqd9: devolve a categoria 0
        d[1] = g(0xde1f40);
        d[2] = g(0xde1f3d);
        d[3] = g(0xde1f3c);
        d[4] = g(0xde1f3e); // cqd9: devolve a categoria 4
    } else {
        crate::comparison::branch("cqd_9 tirar");
        *(0xde1f3f as *mut i8) = training_nibble(e, 0);
        *(0xde1f40 as *mut i8) = training_nibble(e, 1); // cqd9: guarda a categoria 1
        *(0xde1f3d as *mut i8) = training_nibble(e, 2);
        *(0xde1f3c as *mut i8) = training_nibble(e, 3);
        *(0xde1f3e as *mut i8) = training_nibble(e, 4);
        for k in 0..5 {
            d[k] = training_nibble(e, k).wrapping_neg();
        }
    }
    training_manager_apply_training_progress(this, p, &d);
}

/// Comparador: lado a lado na ficha (+0xf..+0x41), na entrada do treino (6 bytes) e no guardado (0xde1f3c, 5 bytes);
/// sem pessoa ou sem ficha (a caixa), só a original.
unsafe extern "thiscall" fn compare_remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        return o(this, p, devolver);
    }
    let f = cqd9_d(p + 0x61);
    let reg = [(f + 0xf, 0x33), (player_training_entry(this, p), 6), (0xde1f3c, 5)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, p, devolver), || remove_or_restore_training_progress(this, p, devolver));
    SCOREBOARD.record(igual, || format!("remove_or_restore_training_progress(pessoa {:x}, devolver {}) {:x?} x {:x?}", p, devolver, mo.0, mr.0));
}

/// { 0x8b37c0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_training_attribute_change(pessoa,
/// atributo)`, pela tela do treino (0x8b8184): o atributo (0x5431e0) menos o mesmo atributo sem o progresso do treino
/// (aplica o progresso negado, lê e reaplica o progresso), no al. Pessoa nula ou sem a ficha: a caixa "Error" (linha
/// 0x657) e 0.
unsafe extern "thiscall" fn get_training_attribute_change(this: usize, p: usize, atributo: u32) -> u8 {
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x657);
        return 0;
    }
    let e = player_training_entry(this, p);
    let ler: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x5431e0usize);
    let antes = ler(p, atributo);
    let mut d = [0i8; 8];
    for k in 0..5 {
        d[k] = training_nibble(e, k).wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &d);
    let sem = ler(p, atributo);
    let mut v = [0i8; 8];
    for k in 0..5 {
        v[k] = d[k].wrapping_neg();
    }
    training_manager_apply_training_progress(this, p, &v);
    antes.wrapping_sub(sem)
}

/// Comparador: lado a lado na ficha e na entrada do treino, com a devolução; sem pessoa ou sem ficha, só a original.
unsafe extern "thiscall" fn compare_get_training_attribute_change(this: usize, p: usize, atributo: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I1%%);
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        return o(this, p, atributo);
    }
    let f = cqd9_d(p + 0x61);
    let reg = [(f + 0xf, 0x33), (player_training_entry(this, p), 6)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, p, atributo), || get_training_attribute_change(this, p, atributo));
    SCOREBOARD.record(igual && r == n, || format!("get_training_attribute_change(pessoa {:x}, atributo {}) original={} rust={} {:x?} x {:x?}", p, atributo as u8 as i8, r as i8, n as i8, mo.0, mr.0));
    r
}

/// As máscaras de posição que a 0x8b4770 aceita (as outras dão a caixa).
fn is_valid_position_mask(m: i32) -> bool {
    matches!(m, 1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x200 | 0x800)
}

/// { 0x8b4770 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_position_aptitude(pessoa, mascara)`
/// (this não usado), pela update_player_training (0x8b1750, a reclamação do treino de posição) e pela
/// set_position_training_rate (0x8b4450): a média da adaptabilidade (+0x56 da pessoa) com os atributos que a posição
/// pede (notas de 1 a 20 pela 0x544380 sobre a ficha +0x61): goleiro (1) +0x39, +0x2a e +0x35, por 4; defesa (2, 4)
/// +0x3e, +0x39 e +0x32, por 4; meio e frente (8, 0x10, 0x20) +0x37, +0x32, +0x40 e +0x3f, por 5; 0x40 +0x33, +0x27,
/// +0x40 e +0x36, por 5; 0x80 +0x30, por 2; 0x200 só a adaptabilidade (no al, os bytes de cima com a máscara); 0x800
/// +0x3b, por 2 (divisões truncadas). Outra máscara: a caixa "Error" (linha 0x945) e 0.
unsafe extern "thiscall" fn get_position_aptitude(this: usize, p: usize, mascara: u32) -> u32 {
    let _ = this;
    let m = mascara as u16 as i16 as i32;
    if !is_valid_position_mask(m) {
        show_error_at(0xa8ccd0, 0x945);
        return 0;
    }
    let f = cqd9_d(p + 0x61);
    let nota_de: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x544380usize);
    // ponytail: a original empurra o byte do atributo num registrador com os bytes de cima que sobram; a 0x544380 lê
    // o byte
    let nota = |o: usize| nota_de(f, *((f + o) as *const u8) as u32) as u8 as i8 as i32;
    let b = |o: usize| cqd9_b(f + o) as i32;
    let adapt = cqd9_b(p + 0x56) as i32;
    let r = match m {
        1 => {
            let x = nota(0x39);
            let y = nota(0x2a);
            (b(0x35) + x + y + adapt) / 4
        }
        2 | 4 => {
            let x = nota(0x3e);
            let y = nota(0x39);
            let z = nota(0x32);
            (x + y + z + adapt) / 4
        }
        8 | 0x10 | 0x20 => {
            let x = nota(0x37);
            let y = nota(0x32);
            (x + y + adapt + b(0x40) + b(0x3f)) / 5
        }
        0x40 => {
            let x = nota(0x33);
            let y = nota(0x27);
            (x + y + adapt + b(0x40) + b(0x36)) / 5
        }
        0x80 => (adapt + b(0x30)) / 2,
        0x200 => return (m as u32 & 0xffff_ff00) | *((p + 0x56) as *const u8) as u32,
        _ => (adapt + b(0x3b)) / 2,
    };
    r as u32
}

/// Comparador: a devolução inteira (só lê); máscara fora (a caixa), só a original.
unsafe extern "thiscall" fn compare_get_position_aptitude(this: usize, p: usize, mascara: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    if !is_valid_position_mask(mascara as u16 as i16 as i32) {
        return o(this, p, mascara);
    }
    let r = o(this, p, mascara);
    let n = get_position_aptitude(this, p, mascara);
    SCOREBOARD.record(r == n, || format!("get_position_aptitude(pessoa {:x}, mascara {:x}) original={:x} rust={:x}", p, mascara as u16, r, n));
    r
}
