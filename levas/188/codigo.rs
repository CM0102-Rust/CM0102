// ---------- 188a leva: a lista de observados do gerente de transferências, a entrada do treino e a carga de treino (game.cpp, injury.cpp) ----------

/// { 0x5d5ab0 } (thiscall, game.cpp): o gerente da lista de observados guardado em +0x856 do gerente
/// de transferências.
unsafe extern "thiscall" fn lista_de_observados(gerente: usize) -> usize {
    core::ptr::read_unaligned((gerente + 0x856) as *const usize)
}

/// { 0x5d5ac0 } (thiscall, ret 4, game.cpp): a entrada de 6 bytes do jogador na tabela do treino
/// ([treino+0x651d]), pelo id dos dados do jogador ([[jogador+0x61]]).
unsafe extern "thiscall" fn entrada_do_treino(treino: usize, jogador: usize) -> usize {
    let tabela = core::ptr::read_unaligned((treino + 0x651d) as *const usize);
    let id = *(core::ptr::read_unaligned((jogador + 0x61) as *const usize) as *const i32);
    (tabela as isize + id as isize * 6) as usize
}

/// O byte da carga de treino do jogador `*id` no registro de condição (31 bytes por jogador, +0x1e).
unsafe fn byte_da_carga(lesoes: usize, id: *const i32) -> *mut u8 {
    (*(lesoes as *const usize) as isize + *id as isize * 31 + 0x1e) as *mut u8
}

/// { 0x62c660 } (thiscall, ret 8, injury.cpp): grava a carga de treino `valor` do jogador `*id`.
unsafe extern "thiscall" fn gravar_carga_de_treino(lesoes: usize, id: *const i32, valor: u8) {
    *byte_da_carga(lesoes, id) = valor;
}

unsafe extern "thiscall" fn comparar_lista_de_observados(gerente: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> usize>(436)(gerente);
    let n = lista_de_observados(gerente);
    PLACAR.registrar(r == n, || format!("lista_de_observados original={:#x} rust={:#x}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_entrada_do_treino(treino: usize, jogador: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(437)(treino, jogador);
    let n = entrada_do_treino(treino, jogador);
    PLACAR.registrar(r == n, || format!("entrada_do_treino original={:#x} rust={:#x}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_gravar_carga_de_treino(lesoes: usize, id: *const i32, valor: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, u8)>(438);
    let a = byte_da_carga(lesoes, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(lesoes, id, valor), || gravar_carga_de_treino(lesoes, id, valor));
    PLACAR.registrar(igual, || format!("gravar_carga_de_treino({}, {}) {:x?} x {:x?}", *id, valor, mo, mr));
}
