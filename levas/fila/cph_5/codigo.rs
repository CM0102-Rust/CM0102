// ---------- %%LEVA%%a leva: as aposentadorias (player_regen.cpp): tirar a pendente e a decisão do desempregado ----------

unsafe fn cph5_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// { 0x7a8690 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::remove_retirement(chave)`: busca (bsearch 0x946464
/// com 0x7ae2a0) a aposentadoria pendente no vetor ([this+4], [this+8] registros de 26 bytes); achou: puxa as
/// seguintes um lugar (0x7a8780, a cópia do registro), a última fica com pessoa −1 e +0x18, +8, +0xa, +0x10, +0x12
/// zerados, e a contagem desce. Sem reordenar.
unsafe extern "thiscall" fn remove_retirement(this: usize, chave: usize) {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let vetor = cph5_d(this + 4) as usize;
    let s = bsearch(chave, vetor, cph5_d(this + 8), 0x1a, 0x7ae2a0);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(vetor) as u32 as u64 * 0x4ec4ec4f) >> 35) as i32; // o lugar (/ 26)
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7a8780usize);
    while k < cph5_d(this + 8) as i32 - 1 {
        let r = (cph5_d(this + 4) as usize).wrapping_add(k as usize * 26);
        copiar(r, r + 0x1a);
        k += 1;
    }
    let fim = (cph5_d(this + 4) as usize).wrapping_add((cph5_d(this + 8) as usize).wrapping_mul(26)).wrapping_sub(26);
    core::ptr::write_unaligned(fim as *mut u32, u32::MAX); // a última sem pessoa
    *((fim + 0x18) as *mut u8) = 0;
    for o in [8usize, 0xa, 0x10, 0x12] {
        core::ptr::write_unaligned((fim + o) as *mut u16, 0);
    }
    core::ptr::write_unaligned((this + 8) as *mut u32, cph5_d(this + 8).wrapping_sub(1));
}

/// Está na lista de observados (0x842080 no gerente da lista [[0xde1f54]+0x856], sem clube)?
unsafe fn player_regen_is_on_shortlist(pessoa: usize) -> bool {
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
    observado(cph5_d(cph5_d(0xde1f54) as usize + 0x856) as usize, pessoa, 0) != 0
}

unsafe fn cph5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x7a8050 } (stdcall, ret 8, player_regen.cpp): `player_regen_check_unemployed_retirement(pessoa, saida)`: quem
/// está na lista de observados não se aposenta. Com registro de técnico (+0x69): sem clube (+0x39) e sem +0x24, com
/// +0xa do registro abaixo de 50 e sorteio(5) = 0. Sem ele, com registro de jogador (+0x61): fora da lista (conferida
/// de novo), sem clube e sem +0x24, desempregado há mais de 365 dias (365 × (ano − ano de +0x40) − dia de +0x3e + dia)
/// e sorteio(nota +0xb do jogador) < 750. Sem nenhum dos dois registros: sempre. Aposenta: a saída (26 bytes) recebe a
/// pessoa, o motivo 0x10, +0x19 = 0, o clube (−1 sem) e a data do jogo em +8 e +0x10; devolve 1. Senão 0.
unsafe extern "stdcall" fn player_regen_check_unemployed_retirement(pessoa: usize, saida: usize) -> u8 {
    let mut sem_registro = false;
    if player_regen_is_on_shortlist(pessoa) {
        return 0;
    }
    let tecnico = cph5_d(pessoa + 0x69) as usize;
    if tecnico == 0 && cph5_d(pessoa + 0x61) == 0 {
        sem_registro = true;
    }
    let aposenta = if tecnico != 0 {
        if cph5_d(pessoa + 0x39) != 0 || cph5_d(pessoa + 0x24) != 0 {
            return 0;
        }
        *((tecnico + 0xa) as *const i16) < 50 && cph5_sortear(5) == 0 // o técnico fraco
    } else if cph5_d(pessoa + 0x61) == 0 {
        false
    } else {
        if player_regen_is_on_shortlist(pessoa) {
            return 0;
        }
        if cph5_d(pessoa + 0x39) != 0 || cph5_d(pessoa + 0x24) != 0 {
            false
        } else {
            let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(*((pessoa + 0x40) as *const u16) as i32);
            let dias = anos.wrapping_mul(365).wrapping_sub(*((pessoa + 0x3e) as *const i16) as i32).wrapping_add(*(0xae2c90 as *const i16) as i32);
            if dias > 0x16d {
                let nota = *((cph5_d(pessoa + 0x61) as usize + 0xb) as *const i16) as i32;
                cph5_sortear(nota) < 0x2ee // o sorteio pela nota
            } else {
                false
            }
        }
    };
    if !aposenta && !sem_registro {
        return 0;
    }
    core::ptr::write_unaligned(saida as *mut u32, cph5_d(pessoa));
    *((saida + 0x18) as *mut u8) = 0x10; // o motivo: desempregado
    *((saida + 0x19) as *mut u8) = 0;
    let clube = cph5_d(pessoa + 0x39) as usize;
    core::ptr::write_unaligned((saida + 4) as *mut u32, if clube != 0 { cph5_d(clube) } else { u32::MAX });
    for o in [8usize, 0x10] {
        core::ptr::write_unaligned((saida + o) as *mut u16, *(0xae2c90 as *const u16));
        core::ptr::write_unaligned((saida + o + 2) as *mut u16, *(0xae2c92 as *const u16));
    }
    1
}

unsafe extern "thiscall" fn comparar_remove_retirement(this: usize, chave: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let n = (*((this + 8) as *const i32)).max(0) as usize;
    let reg = [(this + 4, 8), (cph5_d(this + 4) as usize, n * 26)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, chave), || remove_retirement(this, chave));
    SCOREBOARD.record(igual, || format!("remove_retirement {:x?} x {:x?}", mo, mr));
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pela saída (26 bytes), a cópia em sem_contar.
unsafe extern "stdcall" fn comparar_player_regen_check_unemployed_retirement(pessoa: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(%%I1%%);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((saida, 0x1a));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(pessoa, saida),
        || crate::rng::without_counting(|| player_regen_check_unemployed_retirement(pessoa, saida)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_check_unemployed_retirement({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}
