// ---------- %%LEVA%%a leva: os vínculos de opinião do contrato: limpar, juntar e tirar (Database.cpp) ----------

/// O registro de contrato (79 bytes, tabela [0xae2d0c]) da `pessoa` como as 0x549c90/0x549cf0/0x549d60 o acham:
/// pessoa nula ou id (+0) ≥ [0xae2364] (comparação com sinal: id negativo passa, como na original): nenhum.
/// Os dois vínculos ficam em +0x27/+0x37 e +0x3b/+0x4b (id da outra pessoa, valor), −1 = vazio.
unsafe fn database_staff_state_record(pessoa: usize) -> Option<usize> {
    if pessoa == 0 {
        return None;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return None;
    }
    Some((*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79)))
}

unsafe fn read_state_i32(c: usize, d: usize) -> i32 {
    core::ptr::read_unaligned(c.wrapping_add(d) as *const i32)
}

unsafe fn write_state_i32(c: usize, d: usize, v: i32) {
    core::ptr::write_unaligned(c.wrapping_add(d) as *mut i32, v)
}

/// { 0x549c90 } (cdecl, Database.cpp): `staff_clear_opinion_links(pessoa)`: −1 nos dois vínculos do contrato da
/// pessoa (+0x27, +0x37, +0x3b, +0x4b). Na criação, a cada contrato gerado (0x85e960, por
/// CONTRACT_MANAGER::initialise_all_staff_contracts); na carreira, nas propostas e nas queixas.
unsafe extern "C" fn staff_clear_opinion_links(pessoa: usize) {
    if let Some(c) = database_staff_state_record(pessoa) {
        for k in [0usize, 0x14] {
            write_state_i32(c, k + 0x27, -1);
            write_state_i32(c, k + 0x37, -1); // o valor do vínculo vazio
        }
    }
}

unsafe extern "C" fn comparar_staff_clear_opinion_links(pessoa: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I0%%);
    let Some(c) = database_staff_state_record(pessoa) else { return o(pessoa) }; // não escreve nada
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(pessoa), || staff_clear_opinion_links(pessoa));
    PLACAR.registrar(igual, || format!("staff_clear_opinion_links({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// { 0x549cf0 } (cdecl, Database.cpp): `staff_add_opinion_link(pessoa, outra, valor)`: com as duas pessoas válidas
/// (não nulas, id < [0xae2364]), grava o id da `outra` e o `valor` no primeiro vínculo se ele está vazio (+0x27 = −1),
/// senão no segundo (+0x3b/+0x4b, por cima do que houver). Na carreira, pelos eventos do contrato (0x867700).
unsafe extern "C" fn staff_add_opinion_link(pessoa: usize, outra: usize, valor: i32) {
    let Some(c) = database_staff_state_record(pessoa) else { return };
    if outra == 0 {
        return;
    }
    let id = *(outra as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return;
    }
    if read_state_i32(c, 0x27) == -1 {
        write_state_i32(c, 0x27, id);
        write_state_i32(c, 0x37, valor);
    } else {
        write_state_i32(c, 0x3b, id); // o segundo vínculo
        write_state_i32(c, 0x4b, valor);
    }
}

unsafe extern "C" fn comparar_staff_add_opinion_link(pessoa: usize, outra: usize, valor: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32)>(%%I1%%);
    let Some(c) = database_staff_state_record(pessoa) else { return o(pessoa, outra, valor) };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(pessoa, outra, valor),
        || staff_add_opinion_link(pessoa, outra, valor));
    PLACAR.registrar(igual, || format!("staff_add_opinion_link({:x}, {:x}, {}) {:x?} x {:x?}", pessoa, outra, valor, mo, mr));
}

/// { 0x549d60 } (cdecl, Database.cpp): `staff_remove_opinion_link(pessoa, valor)`: esvazia (−1 no id e no valor) o
/// primeiro vínculo com esse `valor` (+0x37) ou, se não, o segundo (+0x4b); nenhum com o valor: nada. Na carreira,
/// quando a queixa passa (0x867240, 0x8674e0).
unsafe extern "C" fn staff_remove_opinion_link(pessoa: usize, valor: i32) {
    let Some(c) = database_staff_state_record(pessoa) else { return };
    if read_state_i32(c, 0x37) == valor {
        write_state_i32(c, 0x27, -1);
        write_state_i32(c, 0x37, -1);
    } else if read_state_i32(c, 0x4b) == valor {
        write_state_i32(c, 0x3b, -1); // esvazia o segundo vínculo
        write_state_i32(c, 0x4b, -1);
    }
}

unsafe extern "C" fn comparar_staff_remove_opinion_link(pessoa: usize, valor: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32)>(%%I2%%);
    let Some(c) = database_staff_state_record(pessoa) else { return o(pessoa, valor) };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 79)], || o(pessoa, valor), || staff_remove_opinion_link(pessoa, valor));
    PLACAR.registrar(igual, || format!("staff_remove_opinion_link({:x}, {}) {:x?} x {:x?}", pessoa, valor, mo, mr));
}
