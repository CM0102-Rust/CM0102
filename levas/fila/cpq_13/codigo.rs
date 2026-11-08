// ---------- %%LEVA%%a leva: a escalação guardada: limpar uma vaga e o batedor de bola parada (tactics.cpp) ----------

unsafe fn cpq13_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// A linha do clube na tabela tática (0x8982b0, o `this` não é usado; −1 sem linha).
unsafe fn club_tactics_row(clube: usize, time: u32) -> i32 {
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    linha(clube, time)
}

/// A célula `c` (dword) da linha `k` da tabela tática: [this + 0x601] + 0x91 × k + `c`.
unsafe fn tactics_cell(this: usize, k: i32, c: usize) -> usize {
    (cpq13_d(this + 0x601) as usize).wrapping_add((k as isize * 0x91) as usize).wrapping_add(c)
}

/// { 0x896e90 } (thiscall, ret 0xc, tactics.cpp): `Tactics::clear_selection_slot(clube, vaga, time)`: a vaga (0..=19)
/// da escalação do clube na tabela tática (linha da 0x8982b0, dword em +2 + 4 × vaga) fica −1 e, com espaços guardados
/// (+0x5f0), os do mesmo clube e time (16 espaços de 0x5c bytes a partir de +0x30: clube +0x30, time +0x34, ocupado
/// +0x38) são soltos (+0x38 = 0, +0x5f0 − 1). Devolve 1; sem linha, 0. Clube nulo ou vaga fora: a original (caixa).
unsafe extern "thiscall" fn clear_selection_slot(this: usize, clube: usize, vaga: u32, time: u32) -> u32 {
    let v = vaga as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I0%%)(this, clube, vaga, time);
    }
    let k = club_tactics_row(clube, time);
    if k == -1 {
        return 0;
    }
    crate::comparacao::ramo("cpq_13 limpar vaga");
    core::ptr::write_unaligned(tactics_cell(this, k, 2 + 4 * v as usize) as *mut i32, -1); // a vaga
    if *((this + 0x5f0) as *const u8) != 0 {
        for e in 0..16 {
            let a = this + 0x30 + 0x5c * e;
            if cpq13_d(a + 8) != 0 && cpq13_d(a) as usize == clube && cpq13_d(a + 4) == time {
                core::ptr::write_unaligned((a + 8) as *mut u32, 0);
                *((this + 0x5f0) as *mut u8) = (*((this + 0x5f0) as *const u8)).wrapping_sub(1);
            }
        }
    }
    1
}

/// Comparador: só grava a vaga e os espaços guardados: lado a lado pela linha da escalação (20 dwords), pelos 16
/// espaços e pela conta (+0x5f0). Clube nulo ou vaga fora: só a original (a caixa).
unsafe extern "thiscall" fn comparar_clear_selection_slot(this: usize, clube: usize, vaga: u32, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I0%%);
    let v = vaga as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return o(this, clube, vaga, time);
    }
    let mut reg = vec![(this + 0x30, 16 * 0x5c), (this + 0x5f0, 1)];
    let k = club_tactics_row(clube, time);
    if k != -1 {
        reg.push((tactics_cell(this, k, 2), 20 * 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, vaga, time), || clear_selection_slot(this, clube, vaga, time));
    PLACAR.registrar(igual && r == n, || format!("clear_selection_slot(vaga {}, time {}) original={} rust={} {:x?} x {:x?}", v, time, r, n, mo, mr));
    r
}

/// { 0x897bb0 } (thiscall, ret 0x14, tactics.cpp): `Tactics::set_set_piece_taker(clube, papel, escolha, pessoa, time)`:
/// o batedor `escolha` (0..=1) da bola parada `papel` (0..=4) na linha tática do clube (0x8982b0; dword em +0x5e +
/// 4 × (escolha + 2 × papel)). Com `pessoa` (ponteiro para o id), grava o id. Sem ela, a célula fica −1 e, sendo a
/// primeira escolha, a seguinte com batedor (> −1) sobe para ela e fica −1. Devolve 1; sem linha, 0. Clube nulo,
/// papel ou escolha fora: a original (caixa).
unsafe extern "thiscall" fn set_set_piece_taker(this: usize, clube: usize, papel: u32, escolha: u32, pessoa: usize, time: u32) -> u32 {
    let (p, e) = (papel as u8 as i8, escolha as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(%%I1%%)(this, clube, papel, escolha, pessoa, time);
    }
    let k = club_tactics_row(clube, time);
    if k == -1 {
        return 0;
    }
    let celula = |j: i8| tactics_cell(this, k, 0x5e + 4 * (j as i32 + 2 * p as i32) as usize) as *mut i32;
    if pessoa != 0 {
        core::ptr::write_unaligned(celula(e), cpq13_d(pessoa) as i32);
        return 1;
    }
    crate::comparacao::ramo("cpq_13 batedor limpo");
    core::ptr::write_unaligned(celula(e), -1); // sem batedor
    if e < 1 {
        let mut j = e + 1;
        while j < 2 {
            let c = core::ptr::read_unaligned(celula(j));
            if c > -1 {
                crate::comparacao::ramo("cpq_13 batedor sobe");
                core::ptr::write_unaligned(celula(e), c);
                core::ptr::write_unaligned(celula(j), -1);
                return 1;
            }
            j += 1;
        }
    }
    1
}

/// Comparador: só grava a linha dos batedores: lado a lado pelos 10 dwords (+0x5e) da linha do clube. Clube nulo,
/// papel ou escolha fora: só a original (a caixa).
unsafe extern "thiscall" fn comparar_set_set_piece_taker(this: usize, clube: usize, papel: u32, escolha: u32, pessoa: usize, time: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(%%I1%%);
    let (p, e) = (papel as u8 as i8, escolha as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return o(this, clube, papel, escolha, pessoa, time);
    }
    let k = club_tactics_row(clube, time);
    let reg = if k != -1 { vec![(tactics_cell(this, k, 0x5e), 10 * 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, papel, escolha, pessoa, time),
        || set_set_piece_taker(this, clube, papel, escolha, pessoa, time));
    PLACAR.registrar(igual && r == n, || format!("set_set_piece_taker(papel {}, escolha {}, pessoa {:x}) original={} rust={} {:x?} x {:x?}", p, e, pessoa, r, n, mo, mr));
    r
}
