// ---------- %%LEVA%%a leva: o progresso do treino nos atributos: aplicar o progresso das cinco categorias, achar o treino de posição do jogador e se ele aprende uma posição depressa (training_manager.cpp) ----------

unsafe fn cpt9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpt9_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// O atributo de 1 a 20 em `a` mais `v`, entre 1 e 20.
unsafe fn add_clamped_1_20(a: usize, v: i32) {
    *(a as *mut i8) = (cpt9_b(a) as i32 + v).clamp(1, 20) as i8;
}

/// O atributo de −125 a 125 em `a` mais `v`, entre −125 e 125.
unsafe fn add_clamped_125(a: usize, v: i32) {
    *(a as *mut i8) = (cpt9_b(a) as i32 + v).clamp(-125, 125) as i8;
}

/// Quanto o atributo anda com o progresso `d` sobre o meio byte guardado `n` (com sinal): (d + n) / 2 − n / 2, as
/// divisões truncadas para o zero e a conta em 8 bits.
fn progress_step(d: i8, n: i32) -> i32 {
    (((d as i32 + n) / 2) as i8).wrapping_sub((n / 2) as i8) as i32
}

/// O meio byte alto de `a` mais `d` (o baixo fica).
unsafe fn add_high_nibble(a: usize, d: i8) {
    let v = *(a as *const u8);
    *(a as *mut u8) = ((v & 0xf0).wrapping_add((d as u8) << 4)) | (v & 0x0f);
}

/// O meio byte baixo de `a` mais `d` (o alto fica).
unsafe fn add_low_nibble(a: usize, d: i8) {
    let v = *(a as *const u8);
    *(a as *mut u8) = (v & 0xf0) | (v.wrapping_add(d as u8) & 0x0f);
}

/// { 0x8b2be0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::apply_training_progress(pessoa, d)`, pela
/// update_player_training (0x8b1750, cada jogador em cada atualização do treino) e pelas 0x8b3640 e 0x8b37c0. Pessoa
/// nula: a caixa "Error" (linha 0x4b3); sem a ficha (+0x61): a caixa (linha 0x4ba); id a partir de [0xae2364]: nada.
/// Com a entrada de 6 bytes do jogador (this +0x651d + 6 × id da ficha), cada categoria com progresso `d[k]` diferente
/// de zero anda o passo (progress_step sobre o meio byte guardado) nos seus atributos da ficha (os de 1 a 20 o passo, os de
/// −125 a 125 seis vezes o passo) e guarda o meio byte mais `d[k]`: a 3 (+4 alto), a 0 (+3 baixo), a 2 (+4 baixo), a 1
/// (+3 alto) e a 4 (+5 baixo). Depois o treino de posição (0x8b4b20): com a entrada de aprendizado de 15 para cima, a
/// posição ou o lado da máscara vai a 20; abaixo, volta a 1 se ainda sobrar mais de uma posição (+0xf..+0x15) ou lado
/// (+0x17..+0x19) de 15 para cima. Quente: sem contador.
unsafe extern "thiscall" fn apply_training_progress(this: usize, pessoa: usize, d: usize) {
    if pessoa == 0 {
        show_error_at(0xa8ccd0, 0x4b3);
        return;
    }
    let f = cpt9_d(pessoa + 0x61);
    if f == 0 {
        show_error_at(0xa8ccd0, 0x4ba);
        return;
    }
    if cpt9_i(pessoa) >= cpt9_i(0xae2364) {
        return;
    }
    let e = cpt9_d(this + 0x651d) + 6 * cpt9_i(f) as usize;
    // a categoria 3 (o meio byte alto de +4)
    let k = cpt9_b(d + 3);
    if k != 0 {
        let dd = progress_step(k, (cpt9_b(e + 4) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_1_20(f + 0x22, dd);
            add_clamped_125(f + 0x23, seis);
            add_clamped_125(f + 0x26, seis);
            add_clamped_1_20(f + 0x29, dd);
            add_clamped_125(f + 0x2b, seis);
            add_clamped_125(f + 0x37, seis);
            add_clamped_125(f + 0x3e, seis);
            add_clamped_1_20(f + 0x40, dd);
            add_clamped_125(f + 0x41, seis);
        }
        add_high_nibble(e + 4, k); // cpt9: guarda o progresso da categoria 3
    }
    // a categoria 0 (o meio byte baixo de +3)
    let k = cpt9_b(d);
    if k != 0 {
        let dd = progress_step(k, (cpt9_b(e + 3) << 4 >> 4) as i32);
        if dd != 0 {
            add_clamped_1_20(f + 0x1b, dd); // cpt9: o primeiro atributo da categoria 0
            add_clamped_1_20(f + 0x1d, dd);
            add_clamped_1_20(f + 0x2e, dd);
            add_clamped_1_20(f + 0x34, dd);
            add_clamped_1_20(f + 0x36, dd);
            add_clamped_125(f + 0x3a, dd * 6);
            add_clamped_1_20(f + 0x3c, dd);
            add_clamped_1_20(f + 0x3d, dd);
        }
        add_low_nibble(e + 3, k);
    }
    // a categoria 2 (o meio byte baixo de +4)
    let k = cpt9_b(d + 2);
    if k != 0 {
        let dd = progress_step(k, (cpt9_b(e + 4) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_125(f + 0x27, seis);
            add_clamped_125(f + 0x31, seis);
            add_clamped_125(f + 0x38, seis);
        }
        add_low_nibble(e + 4, k);
    }
    // a categoria 1 (o meio byte alto de +3)
    let k = cpt9_b(d + 1);
    if k != 0 {
        let dd = progress_step(k, (cpt9_b(e + 3) >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6; // cpt9: a categoria 1, seis vezes o passo
            add_clamped_125(f + 0x24, seis);
            add_clamped_125(f + 0x32, seis);
            add_clamped_125(f + 0x33, seis);
            add_clamped_1_20(f + 0x3f, dd);
            add_clamped_125(f + 0x39, seis);
        }
        add_high_nibble(e + 3, k);
    }
    // a categoria 4 (o meio byte baixo de +5)
    let k = cpt9_b(d + 4);
    if k != 0 {
        let dd = progress_step(k, (cpt9_b(e + 5) << 4 >> 4) as i32);
        if dd != 0 {
            let seis = dd * 6;
            add_clamped_125(f + 0x2a, seis);
            add_clamped_125(f + 0x35, seis);
        }
        add_low_nibble(e + 5, k);
    }
    // o treino de posição
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let mut p = 0usize;
    let n = achar(this, pessoa, &mut p);
    if n <= 0 {
        return;
    }
    let de15 = |o: usize| (cpt9_b(f + o) >= 0xf) as i8;
    let mut posicoes = de15(0xf) + de15(0x10) + de15(0x11) + de15(0x12) + de15(0x13) + de15(0x14) + de15(0x15);
    let mut lados = de15(0x18) + de15(0x19) + de15(0x17);
    for j in 0..n as usize {
        let item = p + 8 * j;
        let campo = match *((item + 4) as *const i16) as i32 {
            1 => 0xf,
            2 => 0x10,
            4 => 0x11,
            8 => 0x12,
            0x10 => 0x13,
            0x20 => 0x14,
            0x40 => 0x15,
            0x80 => 0x18,
            0x200 => 0x19,
            0x800 => 0x17,
            _ => continue,
        };
        if cpt9_b(item + 6) >= 0xf {
            *((f + campo) as *mut u8) = 0x14;
        } else if campo == 0x18 || campo == 0x19 || campo == 0x17 {
            if lados > 1 {
                *((f + campo) as *mut u8) = 1;
                lados -= 1;
            }
        } else if posicoes > 1 {
            *((f + campo) as *mut u8) = 1;
            posicoes -= 1;
        }
    }
}

/// Comparador: lado a lado na ficha (+0xf..+0x41) e na entrada de treino do jogador (6 bytes), sem os casos de erro e
/// sem as pessoas que não são jogadores (só a original).
unsafe extern "thiscall" fn comparar_apply_training_progress(this: usize, pessoa: usize, d: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if pessoa == 0 || cpt9_d(pessoa + 0x61) == 0 || cpt9_i(pessoa) >= cpt9_i(0xae2364) {
        return o(this, pessoa, d);
    }
    let f = cpt9_d(pessoa + 0x61);
    let e = cpt9_d(this + 0x651d) + 6 * cpt9_i(f) as usize;
    let regioes = [(f + 0xf, 0x33), (e, 6)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, d), || apply_training_progress(this, pessoa, d));
    PLACAR.registrar(igual, || format!("apply_training_progress(pessoa {:x}, d {:x?}) {:x?} x {:x?}", pessoa, core::slice::from_raw_parts(d as *const i8, 5), mo.0, mr.0));
}

/// { 0x8b4b20 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::find_position_training(pessoa, &primeira)`:
/// pessoa nula, a caixa "Error" (linha 0x995); saída nula, a caixa (linha 0x99c); sem a ficha (+0x61), a caixa (linha
/// 0x9a3): 0. Sem a marca 0x10 em +5 da entrada de treino do jogador (this +0x651d + 6 × id da ficha): 0. Senão a busca
/// binária (0x946464, ordem 0x8b4e00) do id da pessoa na lista do treino de posição (this +0x6521, conta +0x6525, itens
/// de 8 bytes); não achou: 0; achou: recua até o primeiro item da pessoa, grava-o na saída e devolve quantos seguidos são
/// dela.
unsafe extern "thiscall" fn find_position_training(this: usize, pessoa: usize, saida: *mut usize) -> i32 {
    if pessoa == 0 {
        show_error_at(0xa8ccd0, 0x995);
        return 0;
    }
    if saida.is_null() {
        show_error_at(0xa8ccd0, 0x99c);
        return 0;
    }
    let f = cpt9_d(pessoa + 0x61);
    if f == 0 {
        show_error_at(0xa8ccd0, 0x9a3);
        return 0;
    }
    if *((cpt9_d(this + 0x651d) + 6 * cpt9_i(f) as usize + 5) as *const u8) & 0x10 == 0 {
        return 0;
    }
    let chave = cpt9_i(pessoa);
    let busca: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    let base = cpt9_d(this + 0x6521);
    let mut a = busca(&chave, base, cpt9_d(this + 0x6525), 8, 0x8b4e00);
    if a == 0 {
        return 0;
    }
    while a > base {
        if cpt9_i(a - 8) != chave {
            break;
        }
        a -= 8;
    }
    *saida = a;
    let ultimo = cpt9_d(this + 0x6521) + cpt9_d(this + 0x6525) * 8 - 8;
    let mut n = 0i32;
    while a <= ultimo && cpt9_i(a) == chave {
        a += 8;
        n += 1;
    }
    n
}

/// Comparador: a devolução e a saída (reposta entre as duas), sem os casos de erro (só a original).
unsafe extern "thiscall" fn comparar_find_position_training(this: usize, pessoa: usize, saida: *mut usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32>(%%I1%%);
    if pessoa == 0 || saida.is_null() || cpt9_d(pessoa + 0x61) == 0 {
        return o(this, pessoa, saida);
    }
    let antes = *saida;
    let r = o(this, pessoa, saida);
    let so = *saida;
    *saida = antes;
    let n = find_position_training(this, pessoa, saida);
    let sr = *saida;
    *saida = so;
    PLACAR.registrar(r == n && so == sr, || format!("find_position_training(pessoa {:x}) original={} {:x} rust={} {:x}", pessoa, r, so, n, sr));
    r
}

/// { 0x8b4db0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::has_fast_position_training(pessoa)`, pelo
/// diário das pessoas (game_daily_staff_update 0x5ce2d0): 1 se algum item do treino de posição da pessoa (0x8b4b20) tem
/// aprendizado (+7) acima de 50; senão 0.
unsafe extern "thiscall" fn has_fast_position_training(this: usize, pessoa: usize) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let mut p = 0usize;
    let n = achar(this, pessoa, &mut p);
    for j in 0..n.max(0) as usize {
        if cpt9_b(p + 8 * j + 7) > 0x32 {
            return 1;
        }
    }
    0
}

/// Comparador: a devolução (as duas só leem; a busca roda pelas duas).
unsafe extern "thiscall" fn comparar_has_fast_position_training(this: usize, pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, pessoa);
    let n = has_fast_position_training(this, pessoa);
    PLACAR.registrar(r == n, || format!("has_fast_position_training(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}
