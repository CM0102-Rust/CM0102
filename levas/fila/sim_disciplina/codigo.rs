// ---------- %%LEVA%%a leva: a disciplina (discipline.cpp): a temporada nova, o dia e a vaga da tabela ----------

/// { 0x556210 } (thiscall, ret 4, discipline.cpp): começa a temporada nova da competição `*id`: as
/// regras da disciplina dela (índice +0x50 da competição em [0xadadfc], 0xff sem) recebem o método 0
/// da tabela virtual.
unsafe extern "thiscall" fn disciplina_temporada_nova(this: usize, id: *const i32) {
    let competicao = *((*(0xadadfc as *const usize) as isize + 4 * *id as isize) as *const usize);
    if competicao == 0 {
        return;
    }
    let k = *((competicao + 0x50) as *const i8);
    if k == -1 {
        return;
    }
    let regras = *((*(this as *const usize) as isize + 4 * k as isize) as *const usize);
    if regras != 0 {
        let metodo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(*(*(regras as *const usize) as *const usize));
        metodo(regras);
    }
}

/// { 0x556240 } (thiscall, ret 4, discipline.cpp): o dia da disciplina: cada uma das 43 regras ([this],
/// as não nulas) recebe o método 1 da tabela virtual com a data `hoje`.
unsafe extern "thiscall" fn disciplina_do_dia(this: usize, hoje: usize) {
    for k in 0..43usize {
        let regras = *((*(this as *const usize) + 4 * k) as *const usize);
        if regras != 0 {
            let metodo: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(*((*(regras as *const usize) + 4) as *const usize));
            metodo(regras, hoje);
        }
    }
}

/// { 0x55cb20 } (thiscall, ret 0x10, discipline.cpp): grava a vaga `vaga` (5 bytes: o byte `tipo` e o
/// valor) da pessoa `id` na tabela +0x14 (29 bytes por pessoa).
unsafe extern "thiscall" fn disciplina_gravar_vaga(this: usize, id: i32, vaga: i8, tipo: u8, valor: u32) {
    let a = (*((this + 0x14) as *const usize) as isize + id as isize * 29 + vaga as isize * 5) as usize;
    *(a as *mut u8) = tipo;
    core::ptr::write_unaligned((a + 1) as *mut u32, valor);
}

// ponytail: as duas primeiras só despacham pela tabela virtual (efeitos fora): provadas pela carreira
unsafe extern "thiscall" fn comparar_disciplina_temporada_nova(this: usize, id: *const i32) {
    orig::<unsafe extern "thiscall" fn(usize, *const i32)>(%%I0%%)(this, id)
}

unsafe extern "thiscall" fn comparar_disciplina_do_dia(this: usize, hoje: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, hoje)
}

unsafe extern "thiscall" fn comparar_disciplina_gravar_vaga(this: usize, id: i32, vaga: i8, tipo: u8, valor: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i8, u8, u32)>(%%I2%%);
    let a = (*((this + 0x14) as *const usize) as isize + id as isize * 29 + vaga as isize * 5) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 5)], || o(this, id, vaga, tipo, valor), || disciplina_gravar_vaga(this, id, vaga, tipo, valor));
    PLACAR.registrar(igual, || format!("disciplina_gravar_vaga({}, {}) {:x?} x {:x?}", id, vaga, mo, mr));
}
