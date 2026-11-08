// ---------- %%LEVA%%a leva: as ordens das pessoas pelo sobrenome e dos clubes pela reputação, e o clube da CEE (Database.cpp) ----------

/// { 0x53ff60 } (cdecl, Database.cpp): `database_compare_staff_by_surname(a, b)`: comparação do qsort de ponteiros
/// de pessoa pelo nome no estilo 4 ("sobrenome, nome", 0x53dba0 em 0x7d0 bytes) na comparação de textos 0x90c1d0
/// (sem maiúsculas, `compare_text_nocase`); pessoa nula vale o texto vazio (a original zera o primeiro byte das duas
/// locais antes). Na tela de oferta de contrato (contract_screens 0x4de5d0).
unsafe extern "C" fn database_compare_staff_by_surname(a: usize, b: usize) -> i32 {
    let formatar: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (*(a as *const usize), *(b as *const usize));
    if pa != 0 {
        formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 4); // "sobrenome, nome" (dbt)
    }
    if pb != 0 {
        formatar(pb, nb.as_mut_ptr() as usize, 0x7d0, 4);
    }
    compare_text_nocase(na.as_ptr(), nb.as_ptr())
}

unsafe extern "C" fn comparar_database_compare_staff_by_surname(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = database_compare_staff_by_surname(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_staff_by_surname({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53ffe0 } (cdecl, Database.cpp): `database_compare_clubs_by_reputation(a, b)`: comparação do qsort de
/// ponteiros de clube pela reputação (+0x80, curto com sinal; clube nulo = 0), a maior primeiro: a de `a` menor: 1;
/// maior: -1; iguais: 0. (A 0x540020, já em Rust, é a mesma ordem sobre ids.) No menu dos times do clube
/// (0x482070) e nas abas de Escolher Amistosos (find_screens 0x5a1480, 0x5a1e10, 0x5a2250).
unsafe extern "C" fn database_compare_clubs_by_reputation(a: usize, b: usize) -> i32 {
    let injury_word_0c = |p: usize| {
        let c = *(p as *const usize);
        if c == 0 { 0 } else { core::ptr::read_unaligned((c + 0x80) as *const i16) }
    };
    let (ra, rb) = (injury_word_0c(a), injury_word_0c(b));
    if ra < rb {
        1 // a de menor reputação depois (dbt)
    } else if ra > rb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn comparar_database_compare_clubs_by_reputation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = database_compare_clubs_by_reputation(a, b);
    SCOREBOARD.record(r == n, || format!("database_compare_clubs_by_reputation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53d7d0 } (cdecl, Database.cpp): `database_club_nation_is_eec(clube)`: 1 se a nação do clube (+0x53) não é
/// nula e está na CEE (+0x7f = 2), senão 0 (o eax inteiro). O clube não é conferido (a original lê o +0x53 dele).
/// Na aba de informações da tela do clube (club_screens 0x462070, os estrangeiros).
unsafe extern "C" fn database_club_nation_is_eec(clube: usize) -> u32 {
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    (nacao != 0 && *((nacao + 0x7f) as *const u8) == 2) as u32 // na CEE (dbt)
}

unsafe extern "C" fn comparar_database_club_nation_is_eec(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(clube);
    let n = database_club_nation_is_eec(clube);
    SCOREBOARD.record(r == n, || format!("database_club_nation_is_eec({:x}) original={} rust={}", clube, r, n));
    r
}
