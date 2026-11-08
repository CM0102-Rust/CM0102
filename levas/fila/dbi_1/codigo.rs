// ---------- %%LEVA%%a leva: liberar os históricos de pessoas e de prêmios e crescer o histórico de competições (Database.cpp) ----------

/// Solta uma tabela do banco como 0x527ee0/0x527f20: `GlobalFree` (IAT 0x9670f4) com o banco no bloco
/// compartilhado ([0xae2340] ≠ 0), senão `free` 0x9452ca.
unsafe fn free_database_table(tabela: usize) {
    if *(0xae2340 as *const u32) != 0 {
        let global_free: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(*(0x9670f4 as *const usize));
        global_free(tabela);
    } else {
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        free(tabela);
    }
}

/// { 0x527ee0 } (cdecl, Database.cpp): `database_free_staff_history_table()`: com a tabela do histórico de
/// pessoas ([0xae23cc], 17 bytes) presente, solta (0x9670f4 ou 0x9452ca, pelo modo do banco) e zera o ponteiro
/// e a contagem [0xae2374]. TapeCall por PlayerStats::PlayerStats (0x7c0ccd): o histórico passa a viver no
/// `player stats history.tmp`. Sem quadro de pilha.
unsafe extern "C" fn database_free_staff_history_table() {
    let tabela = *(0xae23cc as *const usize);
    if tabela == 0 {
        return;
    }
    crate::comparison::branch("database free people history");
    free_database_table(tabela);
    *(0xae23cc as *mut usize) = 0;
    *(0xae2374 as *mut u32) = 0;
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// ponytail: só a original; a prova é o .qst (a tabela sai do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_database_free_staff_history_table() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// { 0x527f20 } (cdecl, Database.cpp): `database_free_staff_comp_history_table()`: o mesmo para o histórico
/// de prêmios ([0xae23e4], 58 bytes; contagem [0xae238c]). TapeCall por AWARD_MANAGER::AWARD_MANAGER (0x413360).
unsafe extern "C" fn database_free_staff_comp_history_table() {
    let tabela = *(0xae23e4 as *const usize);
    if tabela == 0 {
        return;
    }
    crate::comparison::branch("database free award history");
    free_database_table(tabela);
    *(0xae23e4 as *mut usize) = 0;
    *(0xae238c as *mut u32) = 0;
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// ponytail: só a original; a prova é o .qst (a tabela sai do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_database_free_staff_comp_history_table() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// { 0x527f60 } (cdecl, Database.cpp): `database_grow_comp_history_table(n)`: `realloc` 0x945667 do histórico de
/// competições ([0xae23d4], 26 bytes) para [0xae237c] + `n` linhas; sem memória devolve 0 e nada muda. Senão soma
/// `n` à contagem, guarda o bloco, refaz o ponteiro da parte das seleções [0xae23dc] (a partir da linha [0xae2384])
/// e limpa as `n` linhas novas (+0 = o índice da linha, o resto zero); devolve 1. TapeCall por
/// comp_util_add_history_record (0x4afd2c), no fim de competição. Sem quadro de pilha.
unsafe extern "C" fn database_grow_comp_history_table(n: i32) -> u32 {
    crate::comparison::branch("database grow competition history");
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let total = (*(0xae237c as *const i32)).wrapping_add(n);
    let nova = realloc(*(0xae23d4 as *const usize), (total as u32).wrapping_mul(26));
    if nova == 0 {
        return 0;
    }
    *(0xae237c as *mut i32) = total;
    *(0xae23d4 as *mut usize) = nova;
    let selecoes = total.wrapping_sub(*(0xae2384 as *const i32));
    *(0xae23dc as *mut usize) = nova.wrapping_add((selecoes as usize).wrapping_mul(26));
    for k in 1..=n {
        let i = total.wrapping_sub(k);
        let linha = nova.wrapping_add((i as usize).wrapping_mul(26)) as *mut u8;
        core::ptr::write_bytes(linha, 0, 26);
        core::ptr::write_unaligned(linha as *mut i32, i); // a linha nova guarda o próprio índice
    }
    1
}

/// Comparador: o `realloc` não se desfaz (crescer duas vezes deixaria a tabela com 2n linhas a mais).
// ponytail: só a original; a prova é o .qst (a tabela vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_database_grow_comp_history_table(n: i32) -> u32 {
    original_fn::<unsafe extern "C" fn(i32) -> u32>(%%I2%%)(n)
}

