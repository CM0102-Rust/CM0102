// ---------- %%LEVA%%a leva: as cláusulas do contrato no gerente de transferências: percentual da próxima venda, bônus por jogo e bônus pela seleção (transfer_manager.cpp) ----------

/// O comum das três: desliga o bit `bit` das marcas do jogador `*id` (+0x86e, 8 bytes por pessoa) e
/// tira o registro dele (13 bytes, o id em `chave`) da lista [gerente+`lista`] (quantos em
/// [gerente+`quantos`]), puxando os seguintes uma vaga para trás.
unsafe fn tirar_da_lista_de_pagamentos(gerente: usize, id: *const i32, bit: u8, lista: usize, quantos: usize, chave: usize) {
    let marcas = (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marcas & bit != 0 {
        *marcas -= bit;
    }
    let n = *((gerente + quantos) as *const i32);
    let base = *((gerente + lista) as *const usize);
    let Some(i) = (0..n.max(0) as usize).find(|&i| core::ptr::read_unaligned((base + i * 13 + chave) as *const i32) == *id) else {
        return;
    };
    let mut j = i + 1;
    while (j as i32) < *((gerente + quantos) as *const i32) {
        let origem = *((gerente + lista) as *const usize) + j * 13;
        core::ptr::copy(origem as *const u8, (base + (j - 1) * 13) as *mut u8, 13);
        j += 1;
    }
    *((gerente + quantos) as *mut i32) -= 1;
}

/// { 0x8cb4f0 } (thiscall, ret 4, transfer_manager.cpp): tira o percentual da próxima venda (bit 8;
/// lista +0x880, quantos +0x888, o id em +9).
unsafe extern "thiscall" fn remove_next_sale_percentage(gerente: usize, id: *const i32) {
    tirar_da_lista_de_pagamentos(gerente, id, 8, 0x880, 0x888, 9);
}

/// { 0x8cb590 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo no time principal (bit 2;
/// lista +0x88c, quantos +0x894, o id em +0).
unsafe extern "thiscall" fn remove_first_team_appearance_money(gerente: usize, id: *const i32) {
    tirar_da_lista_de_pagamentos(gerente, id, 2, 0x88c, 0x894, 0);
}

/// { 0x8cb630 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo pela seleção (bit 4;
/// lista +0x898, quantos +0x8a0, o id em +0).
unsafe extern "thiscall" fn remove_international_team_appearance_money(gerente: usize, id: *const i32) {
    tirar_da_lista_de_pagamentos(gerente, id, 4, 0x898, 0x8a0, 0);
}

/// As regiões das três: o byte das marcas, a lista inteira e o contador.
unsafe fn regioes_da_lista_de_pagamentos(gerente: usize, id: *const i32, lista: usize, quantos: usize) -> Vec<(usize, usize)> {
    let marcas = (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as usize;
    let n = (*((gerente + quantos) as *const i32)).max(0) as usize;
    let mut r = vec![(marcas, 1), (gerente + quantos, 4)];
    if n > 0 {
        r.push((*((gerente + lista) as *const usize), n * 13));
    }
    r
}

unsafe fn comparar_lista_de_pagamentos(nome: &str, k: usize, rust: unsafe extern "thiscall" fn(usize, *const i32), gerente: usize, id: *const i32, lista: usize, quantos: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(k);
    let reg = regioes_da_lista_de_pagamentos(gerente, id, lista, quantos);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || rust(gerente, id));
    PLACAR.registrar(igual, || format!("{}({}) {:x?} x {:x?}", nome, *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_remove_next_sale_percentage(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_next_sale_percentage", %%I0%%, remove_next_sale_percentage, gerente, id, 0x880, 0x888);
}

unsafe extern "thiscall" fn comparar_remove_first_team_appearance_money(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_first_team_appearance_money", %%I1%%, remove_first_team_appearance_money, gerente, id, 0x88c, 0x894);
}

unsafe extern "thiscall" fn comparar_remove_international_team_appearance_money(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_international_team_appearance_money", %%I2%%, remove_international_team_appearance_money, gerente, id, 0x898, 0x8a0);
}
