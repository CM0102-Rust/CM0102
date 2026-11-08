// ---------- %%LEVA%%a leva: a semana do treino, a reserva de ficha de não jogador e a lesão que volta (training_manager.cpp, player_regen.cpp, injury.cpp) ----------

unsafe fn cpt10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// { 0x8b1730 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::advance_week_counter(&dia)`, todo dia pelo
/// laço do dia (game_main_day_loop 0x5cccd0, com o dia do ano da data do jogo 0xae2c90): no dia múltiplo de 7 (resto
/// com sinal), a semana do treino (+0x688d, 16 bits) sobe 1; a update_player_training (0x8b1750) põe em dia as semanas
/// que faltam de cada jogador. Devolve dia / 7 (o chamador não lê).
unsafe extern "thiscall" fn advance_week_counter(this: usize, dia: *const i16) -> i32 {
    let v = *dia as i32;
    let resto = v % 7; // cpt10: a semana de 7 dias
    if resto == 0 { // cpt10: o sétimo dia
        crate::comparacao::ramo("cpt_10 semana");
        let s = (this + 0x688d) as *mut u16;
        *s = (*s).wrapping_add(1); // cpt10: mais uma semana
    }
    v / 7
}

/// Comparador: lado a lado na semana do treino, com a devolução.
unsafe extern "thiscall" fn comparar_advance_week_counter(this: usize, dia: *const i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i16) -> i32>(%%I0%%);
    let regioes = [(this + 0x688d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, dia), || advance_week_counter(this, dia));
    PLACAR.registrar(igual && r == n, || format!("advance_week_counter(dia {}) original={} rust={} {:x?} x {:x?}", *dia, r, n, mo.0, mr.0));
    r
}

/// { 0x7a6e30 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::take_spare_non_player_record(&id, &nacao,
/// &segunda)`, pela contract_manager (0x4ce4e0, duas chamadas): copia o primeiro item da reserva de fichas de não
/// jogador (this +0x1c: id, nação e segunda nação) para as três saídas e tira-o da reserva
/// (PLAYER_REGEN::remove_spare_non_player_record 0x7a82b0 com o id lido da saída); devolve o eax dela.
unsafe extern "thiscall" fn take_spare_non_player_record(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    crate::comparacao::ramo("cpt_10 ficha reservada");
    *id = cpt10_d(cpt10_d(this + 0x1c)) as u32;
    *nacao = cpt10_d(cpt10_d(this + 0x1c) + 4) as u32;
    *segunda = cpt10_d(cpt10_d(this + 0x1c) + 8) as u32;
    let tirar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7a82b0usize);
    tirar(this, *id)
}

// ponytail: a reserva perde o item (não roda duas vezes): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_take_spare_non_player_record(this: usize, id: *mut u32, nacao: *mut u32, segunda: *mut u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u32) -> u32>(%%I1%%)(this, id, nacao, segunda)
}

/// { 0x62ed30 } (thiscall, ret 4, injury.cpp): `Injury::get_recurring_injury_type(pessoa)`, pelas notícias (0x77bc80),
/// pela avaliação do fisioterapeuta (0x7a2650) e pelas telas: o tipo da lesão que volta (+0x12 da ficha de forma de 31
/// bytes da pessoa, em [this]); pessoa nula, id a partir de [0xae2364] ou sem a ficha de jogador (+0x61): 0xff. Os
/// chamadores leem só o al.
unsafe extern "thiscall" fn get_recurring_injury_type(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 || cpt10_d(pessoa) as i32 >= cpt10_d(0xae2364) as i32 || cpt10_d(pessoa + 0x61) == 0 {
        return 0xff;
    }
    *((cpt10_d(this) + 31 * cpt10_d(pessoa)) as *const u8).add(0x12)
}

/// Comparador: o al devolvido.
unsafe extern "thiscall" fn comparar_get_recurring_injury_type(this: usize, pessoa: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, pessoa);
    let n = get_recurring_injury_type(this, pessoa);
    PLACAR.registrar(r == n, || format!("get_recurring_injury_type(pessoa {:x}) original={:x} rust={:x}", pessoa, r, n));
    r
}
