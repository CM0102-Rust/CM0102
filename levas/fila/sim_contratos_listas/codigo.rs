// ---------- %%LEVA%%a leva: o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto (contract_manager.cpp, shortlist_manager.cpp, human_manager.cpp) ----------

/// A entrada de 79 bytes da pessoa `*id` na tabela [0xae2d0c] (em +4 a vaga do empréstimo, −1 sem).
unsafe fn loan_state_entry(id: *const i32) -> usize {
    (*(0xae2d0c as *const usize) as isize + *id as isize * 79) as usize
}

/// { 0x4d0bd0 } (thiscall, ret 4, contract_manager.cpp): libera a vaga do contrato de empréstimo da
/// pessoa, se ela tiver uma: −1 no começo da vaga (80 bytes cada, em [gerente]) e na entrada da pessoa.
unsafe extern "thiscall" fn remove_loan_contract(gerente: usize, id: *const i32) {
    let e = loan_state_entry(id);
    let vaga = core::ptr::read_unaligned((e + 4) as *const i32);
    if vaga != -1 {
        *((*(gerente as *const usize) as isize + vaga as isize * 80) as *mut i32) = -1;
        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, -1);
    }
}

/// A lista de observados do clube `*clube` (9 bytes por clube em [gerente+0xdc]: +0 as vagas, +4 a marca).
unsafe fn club_shortlist(gerente: usize, clube: *const i32) -> usize {
    (*((gerente + 0xdc) as *const usize) as isize + *clube as isize * 9) as usize
}

/// { 0x84aeb0 } (thiscall, ret 8, shortlist_manager.cpp): com a lista do clube ativa (marca 1), tira a
/// marca de lance (+0x17 = −1) das 40 vagas (0x25 bytes cada) cujo alvo (+0xf) é o jogador `*jogador`.
unsafe extern "thiscall" fn clear_bid_markers_for_player(gerente: usize, clube: *const i32, jogador: *const i32) {
    let l = club_shortlist(gerente, clube);
    if *((l + 4) as *const u8) != 1 {
        return;
    }
    for i in 0..40 {
        let v = core::ptr::read_unaligned(l as *const usize) + i * 0x25;
        if core::ptr::read_unaligned((v + 0xf) as *const i32) == *jogador {
            core::ptr::write_unaligned((v + 0x17) as *mut i32, -1);
        }
    }
}

/// { 0x600e90 } (cdecl, human_manager.cpp): quem responde pelo técnico do clube: o auxiliar (+0xd3)
/// se houver; senão, dos cinco da comissão (+0x19f) com dados (+0x69), o de maior reputação (+0xa,
/// ficando o primeiro no empate); 0 se nenhum.
unsafe extern "C" fn human_manager_get_stand_in_manager(clube: usize) -> usize {
    let auxiliar = core::ptr::read_unaligned((clube + 0xd3) as *const usize);
    if auxiliar != 0 {
        return auxiliar;
    }
    let injury_word_0c = |p: usize| core::ptr::read_unaligned((core::ptr::read_unaligned((p + 0x69) as *const usize) + 0xa) as *const i16);
    let mut melhor = 0usize;
    for k in 0..5 {
        let c = core::ptr::read_unaligned((clube + 0x19f + k * 4) as *const usize);
        if c == 0 || core::ptr::read_unaligned((c + 0x69) as *const usize) == 0 {
            continue;
        }
        if melhor == 0 || injury_word_0c(c) > injury_word_0c(melhor) {
            melhor = c;
        }
    }
    melhor
}

unsafe extern "thiscall" fn comparar_remove_loan_contract(gerente: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(%%I0%%);
    let e = loan_state_entry(id);
    let mut reg = vec![(e + 4, 4)];
    let vaga = core::ptr::read_unaligned((e + 4) as *const i32);
    if vaga != -1 {
        reg.push(((*(gerente as *const usize) as isize + vaga as isize * 80) as usize, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || remove_loan_contract(gerente, id));
    PLACAR.registrar(igual, || format!("remove_loan_contract({}) {:x?} x {:x?}", *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_clear_bid_markers_for_player(gerente: usize, clube: *const i32, jogador: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, *const i32)>(%%I1%%);
    let l = club_shortlist(gerente, clube);
    let reg = if *((l + 4) as *const u8) == 1 { vec![(core::ptr::read_unaligned(l as *const usize), 40 * 0x25)] } else { vec![(l, 9)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, clube, jogador), || clear_bid_markers_for_player(gerente, clube, jogador));
    PLACAR.registrar(igual, || format!("clear_bid_markers_for_player({}, {}) {:x?} x {:x?}", *clube, *jogador, mo, mr));
}

unsafe extern "C" fn comparar_human_manager_get_stand_in_manager(clube: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I2%%)(clube);
    let n = human_manager_get_stand_in_manager(clube);
    PLACAR.registrar(r == n, || format!("human_manager_get_stand_in_manager({:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}
