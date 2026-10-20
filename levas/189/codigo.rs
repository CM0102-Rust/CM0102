// ---------- 189a leva: a competição do amistoso, a data internacional e o registro de técnico (friendly.cpp, staff_records.cpp) ----------

/// { 0x5c2990 } (thiscall, ret 4, friendly.cpp): `competicao` é a competição dos amistosos (+4)?
unsafe extern "thiscall" fn e_competicao_do_amistoso(amistoso: usize, competicao: usize) -> u32 {
    (competicao == *((amistoso + 4) as *const usize)) as u32
}

/// { 0x5cb9b0 } (thiscall, ret 0x18, friendly.cpp): preenche a data internacional de 20 bytes: dia,
/// mês e duração (+0..+2), o ano a somar (+0xb), o continente (+0xc) e a competição (+0x10).
unsafe extern "thiscall" fn preencher_data_internacional(data: usize, dia: u8, mes: u8, duracao: u8, ano: u8, continente: u32, competicao: u32) {
    let b = data as *mut u8;
    *b = dia;
    *b.add(1) = mes;
    *b.add(2) = duracao;
    *b.add(0xb) = ano;
    core::ptr::write_unaligned(b.add(0xc) as *mut u32, continente);
    core::ptr::write_unaligned(b.add(0x10) as *mut u32, competicao);
}

/// O registro de técnico de 210 bytes da entrada `*id` ([registros+8]).
unsafe fn registro_de_tecnico(registros: usize, id: *const i32) -> usize {
    (*((registros + 8) as *const usize) as isize + *id as isize * 210) as usize
}

/// { 0x86fc20 } (thiscall, ret 4, staff_records.cpp): zera o registro de técnico da entrada e põe −1
/// nos ids do dono, da liga e dos clubes das passagens (+0x4a, +0x5e, +0x72, +0x86, +0xc2, +0xc6).
unsafe extern "thiscall" fn limpar_registro_de_tecnico(registros: usize, id: *const i32) {
    let r = registro_de_tecnico(registros, id);
    core::ptr::write_bytes(r as *mut u8, 0, 210);
    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        core::ptr::write_unaligned((r + o) as *mut i32, -1);
    }
}

unsafe extern "thiscall" fn comparar_e_competicao_do_amistoso(amistoso: usize, competicao: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(439)(amistoso, competicao);
    let n = e_competicao_do_amistoso(amistoso, competicao);
    PLACAR.registrar(r == n, || format!("e_competicao_do_amistoso({:#x}) original={} rust={}", competicao, r, n));
    r
}

unsafe extern "thiscall" fn comparar_preencher_data_internacional(data: usize, dia: u8, mes: u8, duracao: u8, ano: u8, continente: u32, competicao: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8, u8, u8, u8, u32, u32)>(440);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(data, 0x14)], || o(data, dia, mes, duracao, ano, continente, competicao),
        || preencher_data_internacional(data, dia, mes, duracao, ano, continente, competicao));
    PLACAR.registrar(igual, || format!("preencher_data_internacional {:x?} x {:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_limpar_registro_de_tecnico(registros: usize, id: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(441);
    let r = registro_de_tecnico(registros, id);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 210)], || o(registros, id), || limpar_registro_de_tecnico(registros, id));
    PLACAR.registrar(igual, || format!("limpar_registro_de_tecnico({}) {:x?} x {:x?}", *id, mo, mr));
}
