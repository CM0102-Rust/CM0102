// ---------- %%LEVA%%a leva: as regras do contrato: o clube em temporada e o clube e a nação entre as seis grandes (Database.cpp) ----------

/// { 0x547770 } (cdecl, Database.cpp): `club_is_in_season(clube)`: clube nulo, 0; senão o que
/// `key_nation_club_in_season` 0x669bd0(clube, nulo, nulo) devolve (1 = hoje dentro da temporada da nação do clube),
/// o eax inteiro (o contrato padrão 0x864550 testa o eax). Na criação: o contrato padrão das pessoas
/// (CONTRACT_MANAGER::initialise_all_staff_contracts → 0x85e960 → 0x864550) e o salário padrão das listas iniciais
/// (0x83a1b0 → 0x862720).
unsafe extern "C" fn club_is_in_season(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);
    temporada(clube, 0, 0) // a temporada da nação do clube
}

unsafe extern "C" fn compare_club_is_in_season(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(clube);
    let n = club_is_in_season(clube);
    SCOREBOARD.record(r == n, || format!("club_is_in_season({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549e50 } (cdecl, Database.cpp): `club_is_in_major_nation(clube)`: 1 se a nação do clube (+0x53) tem o id
/// (+0) de uma das chaves Inglaterra 0x9cf2e4, França 0x9cf300, Alemanha 0x9cf314, Itália 0x9cf360, Espanha 0x9cf488 ou
/// Holanda 0x9cf338; clube ou nação nulos: 0. Só `al` (o resto do eax é o id da nação). Na criação, pelo contrato
/// padrão (0x864550) e pelo salário padrão (0x862720).
unsafe extern "C" fn club_is_in_major_nation(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const i32);
    let chaves = [0x9cf2e4usize, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338]; // Holanda (clube)
    chaves.iter().any(|&k| *(k as *const i32) == id) as u8
}

unsafe extern "C" fn compare_club_is_in_major_nation(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I1%%)(clube);
    let n = club_is_in_major_nation(clube);
    SCOREBOARD.record(r == n, || format!("club_is_in_major_nation({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549ea0 } (cdecl, Database.cpp): `nation_is_major_nation(nacao)`: o mesmo teste das seis chaves de 0x549e50
/// sobre o id (+0) da própria nação; nula: 0. Só `al`. Na carreira, pela regeneração dos jovens
/// (PLAYER_REGEN::regenerate_as_youth_player 0x7abac0).
unsafe extern "C" fn nation_is_major_nation(nacao: usize) -> u8 {
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const i32);
    let chaves = [0x9cf2e4usize, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338]; // Holanda (nação)
    chaves.iter().any(|&k| *(k as *const i32) == id) as u8
}

unsafe extern "C" fn compare_nation_is_major_nation(nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%)(nacao);
    let n = nation_is_major_nation(nacao);
    SCOREBOARD.record(r == n, || format!("nation_is_major_nation({:x}) original={} rust={}", nacao, r, n));
    r
}
