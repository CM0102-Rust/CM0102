// ---------- %%LEVA%%a leva: as regras por nação: o clube fora da Argentina, fora da Argentina, França e Croácia, e a nação da Melanésia (Database.cpp) ----------

/// O id (+0) da nação do `clube` (+0x53); clube ou nação nulos: None.
unsafe fn database_club_nation_id(clube: usize) -> Option<u32> {
    if clube == 0 {
        return None;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 {
        return None;
    }
    Some(*(nacao as *const u32))
}

/// { 0x5487b0 } (cdecl, Database.cpp): `club_nation_is_not_argentina(clube)`: 1 se a nação do clube não tem o id
/// da chave Argentina 0x9cf218; clube ou nação nulos: 0. A resposta vai em `al` e o resto do eax é o do id (`setne
/// al` depois do `mov eax, [eax]`), como na original. Na carreira: a disciplina (0x55e300), o diário dos técnicos
/// (manager_manager 0x6ab800) e as notícias (0x782f70).
unsafe extern "C" fn club_nation_is_not_argentina(clube: usize) -> u32 {
    match database_club_nation_id(clube) {
        None => 0,
        Some(id) => (id & 0xffff_ff00) | (id != *(0x9cf218 as *const u32)) as u32, // a Argentina
    }
}

unsafe extern "C" fn compare_club_nation_is_not_argentina(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(clube);
    let n = club_nation_is_not_argentina(clube);
    SCOREBOARD.record(r == n, || format!("club_nation_is_not_argentina({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// { 0x5487d0 } (cdecl, Database.cpp): `club_nation_not_argentina_france_croatia(clube)`: 1 se a nação do clube não
/// tem o id de Argentina 0x9cf218, França 0x9cf300 nem Croácia 0x9cf2ac; clube ou nação nulos: 0. Em `al`, o resto
/// do eax é o do id. Na carreira, pelo diário dos técnicos (manager_manager 0x6ab800).
unsafe extern "C" fn club_nation_not_argentina_france_croatia(clube: usize) -> u32 {
    match database_club_nation_id(clube) {
        None => 0,
        Some(id) => {
            let chaves = [0x9cf218usize, 0x9cf300, 0x9cf2ac]; // Argentina, França, Croácia
            (id & 0xffff_ff00) | (!chaves.iter().any(|&k| *(k as *const u32) == id)) as u32
        }
    }
}

unsafe extern "C" fn compare_club_nation_not_argentina_france_croatia(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(clube);
    let n = club_nation_not_argentina_france_croatia(clube);
    SCOREBOARD.record(r == n, || format!("club_nation_not_argentina_france_croatia({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// 1 se o id (+0) da `nacao` (não nula; a original não confere) é o de uma das `chaves`, senão 0 (o eax inteiro).
unsafe fn nation_is_one_of(nacao: usize, chaves: &[usize]) -> u32 {
    let id = *(nacao as *const u32);
    chaves.iter().any(|&k| *(k as *const u32) == id) as u32
}

/// { 0x53d900 } (cdecl, Database.cpp): `database_is_melanesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (as melanésias), senão 0. Nas copas da Oceania
/// (oceania_club_champ 0x798220, oceania_nations 0x799e30).
unsafe extern "C" fn database_is_melanesian_nation(nacao: usize) -> u32 {
    nation_is_one_of(nacao, &[0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500]) // melanésias
}

unsafe extern "C" fn compare_database_is_melanesian_nation(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(nacao);
    let n = database_is_melanesian_nation(nacao);
    SCOREBOARD.record(r == n, || format!("database_is_melanesian_nation({:x}) original={} rust={}", nacao, r, n));
    r
}
