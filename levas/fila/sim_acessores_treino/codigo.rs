// ---------- %%LEVA%%a leva: a lista de observados do gerente de transferências, a entrada do treino e a carga de treino (game.cpp, injury.cpp) ----------

/// { 0x5d5ab0 } (thiscall, game.cpp): o gerente da lista de observados guardado em +0x856 do gerente
/// de transferências.
unsafe extern "thiscall" fn transfer_manager_get_shortlist_manager(gerente: usize) -> usize {
    core::ptr::read_unaligned((gerente + 0x856) as *const usize)
}

/// { 0x5d5ac0 } (thiscall, ret 4, game.cpp): a entrada de 6 bytes do jogador na tabela do treino
/// ([treino+0x651d]), pelo id dos dados do jogador ([[jogador+0x61]]).
unsafe extern "thiscall" fn training_manager_get_player_entry(treino: usize, jogador: usize) -> usize {
    let tabela = core::ptr::read_unaligned((treino + 0x651d) as *const usize);
    let id = *(core::ptr::read_unaligned((jogador + 0x61) as *const usize) as *const i32);
    (tabela as isize + id as isize * 6) as usize
}

/// O byte da carga de treino do jogador `*id` no registro de condição (31 bytes por jogador, +0x1e).
unsafe fn training_load_byte(lesoes: usize, id: *const i32) -> *mut u8 {
    (*(lesoes as *const usize) as isize + *id as isize * 31 + 0x1e) as *mut u8
}

/// { 0x62c660 } (thiscall, ret 8, injury.cpp): grava a carga de treino `valor` do jogador `*id`.
unsafe extern "thiscall" fn set_training_load(lesoes: usize, id: *const i32, valor: u8) {
    *training_load_byte(lesoes, id) = valor;
}

unsafe extern "thiscall" fn compare_transfer_manager_get_shortlist_manager(gerente: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(gerente);
    let n = transfer_manager_get_shortlist_manager(gerente);
    SCOREBOARD.record(r == n, || format!("transfer_manager_get_shortlist_manager original={:#x} rust={:#x}", r, n));
    r
}

unsafe extern "thiscall" fn compare_training_manager_get_player_entry(treino: usize, jogador: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(treino, jogador);
    let n = training_manager_get_player_entry(treino, jogador);
    SCOREBOARD.record(r == n, || format!("training_manager_get_player_entry original={:#x} rust={:#x}", r, n));
    r
}

unsafe extern "thiscall" fn compare_set_training_load(lesoes: usize, id: *const i32, valor: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, u8)>(%%I2%%);
    let a = training_load_byte(lesoes, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(a, 1)], || o(lesoes, id, valor), || set_training_load(lesoes, id, valor));
    SCOREBOARD.record(igual, || format!("set_training_load({}, {}) {:x?} x {:x?}", *id, valor, mo, mr));
}
