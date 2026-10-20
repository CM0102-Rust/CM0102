// ---------- %%LEVA%%a leva: o clube do contrato acertado, o jogo pela seleção e a punição guardada (transfer_manager.cpp, discipline.cpp) ----------

/// { 0x8c5450 } (thiscall, ret 8, transfer_manager.cpp): com a marca de contrato acertado (0x40) da
/// pessoa `*id`, procura o contrato (0x4cca30 em [0xae1050]): achou, grava em `clube` (se não nulo) o
/// clube do contrato ([0xae23b4], 581 bytes por clube, pelo id em +4) e devolve 1; não achou, desliga a
/// marca e devolve 0.
unsafe extern "thiscall" fn clube_do_contrato_acertado(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let marcas = |g: usize| (*((g + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marcas(gerente) & 0x40 == 0 {
        return 0;
    }
    let procurar: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x4cca30usize);
    let contrato = procurar(*(0xae1050 as *const usize), *id);
    if contrato == 0 {
        *marcas(gerente) &= 0xbf;
        return 0;
    }
    if !clube.is_null() {
        *clube = (*(0xae23b4 as *const usize) as isize + *((contrato + 4) as *const i32) as isize * 581) as usize;
    }
    1
}

/// O registro de 8 bytes da pessoa `*id` no gerente de transferências (+0x86e).
unsafe fn registro_da_pessoa(gerente: usize, id: *const i32) -> usize {
    (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as usize
}

/// { 0x8ce920 } (thiscall, ret 8, transfer_manager.cpp): registra que o jogador jogou o jogo `numero`
/// da seleção: os bits dos últimos jogos (+3, 16 bits) andam um por jogo desde o último registrado
/// (+5), o mais novo é ligado e o número fica em +5.
unsafe extern "thiscall" fn registrar_jogo_pela_selecao(gerente: usize, id: *const i32, numero: i16) {
    let r = registro_da_pessoa(gerente, id);
    let bits = (r + 3) as *mut u16;
    let ultimo = (r + 5) as *mut i16;
    let d = numero.wrapping_sub(core::ptr::read_unaligned(ultimo));
    for _ in 0..d.max(0) {
        core::ptr::write_unaligned(bits, core::ptr::read_unaligned(bits) << 1);
        *(bits as *mut u8) &= 0xfe;
    }
    *(bits as *mut u8) |= 1;
    core::ptr::write_unaligned(ultimo, numero);
}

/// { 0x55cb60 } (thiscall, ret 8, discipline.cpp): a punição guardada do jogador `id` na vaga `vaga`:
/// percorre a lista das guardadas (+0x1c, do começo, 0x68d1b0) enquanto o jogador tem alguma
/// (byte +0x1c do registro de 29 bytes em [+0x14]) e devolve a de mesmo jogador e vaga; 0 se nenhuma.
unsafe extern "thiscall" fn punicao_guardada(disciplina: usize, id: i32, vaga: i8) -> usize {
    let lista = disciplina + 0x1c;
    let registro = (*((disciplina + 0x14) as *const usize) as isize + id as isize * 29) as usize;
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let proxima: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut p = proxima(lista);
    while p != 0 {
        if *((registro + 0x1c) as *const i8) <= 0 {
            return 0;
        }
        if *(p as *const i32) == id && *((p + 4) as *const i8) == vaga {
            return p;
        }
        p = proxima(lista);
    }
    0
}

unsafe extern "thiscall" fn comparar_clube_do_contrato_acertado(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, *mut usize) -> u32>(%%I0%%);
    let mut reg = vec![(registro_da_pessoa(gerente, id), 1)];
    if !clube.is_null() {
        reg.push((clube as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, clube), || clube_do_contrato_acertado(gerente, id, clube));
    PLACAR.registrar(igual && ro == rr, || format!("clube_do_contrato_acertado({}) original={} rust={} {:x?} x {:x?}", *id, ro, rr, mo, mr));
    ro
}

unsafe extern "thiscall" fn comparar_registrar_jogo_pela_selecao(gerente: usize, id: *const i32, numero: i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, i16)>(%%I1%%);
    let reg = [(registro_da_pessoa(gerente, id) + 3, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, numero), || registrar_jogo_pela_selecao(gerente, id, numero));
    PLACAR.registrar(igual, || format!("registrar_jogo_pela_selecao({}, {}) {:x?} x {:x?}", *id, numero, mo, mr));
}

unsafe extern "thiscall" fn comparar_punicao_guardada(disciplina: usize, id: i32, vaga: i8) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i8) -> usize>(%%I2%%);
    // a lista guarda o cursor: as duas partem do mesmo e têm de deixá-lo igual
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(disciplina + 0x1c, 0x12)], || o(disciplina, id, vaga), || punicao_guardada(disciplina, id, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("punicao_guardada({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", id, vaga, ro, rr, mo, mr));
    ro
}
