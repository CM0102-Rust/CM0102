// ---------- %%LEVA%%a leva: a idade da pessoa, se a nação tem transferência livre e se a pessoa é da UE (Database.cpp) ----------

/// { 0x53ab90 } (cdecl, Database.cpp): a idade da `pessoa` em anos completos na `data` (palavra +0 dia
/// do ano, +2 ano): ano − ano de nascimento (+0x12), menos 1 se o dia de nascimento (+0x10) ainda não
/// chegou. Pessoa nula: caixa "Error" e 0xff (a original). Só o ax é devolvido (quem chama usa al).
unsafe extern "C" fn database_get_staff_age(pessoa: usize, data: *const u16) -> u16 {
    if pessoa == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *const u16) -> u16>(%%I0%%)(pessoa, data);
    }
    let dia_nasc = core::ptr::read_unaligned((pessoa + 0x10) as *const i16) as i32;
    let anos = core::ptr::read_unaligned(data.add(1)).wrapping_sub(core::ptr::read_unaligned((pessoa + 0x12) as *const u16));
    if dia_nasc - core::ptr::read_unaligned(data as *const i16) as i32 > 0 {
        anos.wrapping_sub(1) // ainda não fez aniversário
    } else {
        anos
    }
}

/// { 0x544670 } (cdecl, Database.cpp): 1 se a `nacao` tem transferência livre: com as regras novas da
/// FIFA ([0xae233c] = 1), no Brasil ou numa nação da Europa (continente +0x71, [0x9cfa10]) fora de
/// Polônia, Turquia, Croácia e Rússia.
unsafe extern "C" fn nation_allows_free_transfers(nacao: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    if *(0xae233c as *const u8) == 1 {
        return 1;
    }
    let id = g(nacao);
    if id == g(0x9cf260) {
        return 1; // Brasil
    }
    let continente = core::ptr::read_unaligned((nacao + 0x71) as *const usize);
    if continente == 0 {
        return 0;
    }
    if id == g(0x9cf430) || id == g(0x9cf4dc) || id == g(0x9cf2ac) || id == g(0x9cf448) {
        return 0;
    }
    (g(continente) == g(0x9cfa10)) as u32 // Europa
}

/// { 0x53d740 } (cdecl, Database.cpp): 1 se a `pessoa` conta como da UE para um clube da
/// `nacao_do_clube`: nacionalidade (+0x1a) ou segunda (+0x1e) com a marca de UE (nação +0x7f = 2) ou
/// Suíça ([0x9cf4ac]); na Alemanha, qualquer europeu (nação +0x71 = o continente [0x9cfa10] na tabela
/// [0xae23a4], 198 bytes); em [0x9cf360], todos; na Dinamarca, [0x9cf348], Noruega e [0x9cf3a0].
unsafe extern "C" fn database_is_staff_eu_national(pessoa: usize, nacao_do_clube: usize) -> u32 {
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let g = |a: usize| *(a as *const u32);
    let n = ler(pessoa, 0x1a);
    if *((n + 0x7f) as *const u8) == 2 { return 1; } // nacionalidade da UE
    let id = g(n);
    if id == g(0x9cf4ac) {
        return 1;
    }
    let n2 = ler(pessoa, 0x1e);
    if n2 != 0 && *((n2 + 0x7f) as *const u8) == 2 {
        return 1;
    }
    if nacao_do_clube == 0 {
        return 0;
    }
    let c = g(nacao_do_clube);
    if c == g(0x9cf314) {
        let europa = (*(0xae23a4 as *const usize)).wrapping_add((g(0x9cfa10) as usize).wrapping_mul(0xc6));
        if ler(n, 0x71) == europa {
            return 1;
        }
    }
    if c == g(0x9cf360) {
        return 1;
    }
    if c == g(0x9cf2c0) && (id == g(0x9cf348) || id == g(0x9cf408) || id == g(0x9cf3a0)) {
        return 1;
    }
    0
}

unsafe extern "C" fn comparar_database_get_staff_age(pessoa: usize, data: *const u16) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize, *const u16) -> u16>(%%I0%%);
    if pessoa == 0 {
        return o(pessoa, data); // caixa: só a original
    }
    let r = o(pessoa, data);
    let n = database_get_staff_age(pessoa, data);
    SCOREBOARD.record(r == n, || format!("database_get_staff_age({:x}) original={} rust={}", pessoa, r, n));
    r
}

unsafe extern "C" fn comparar_nation_allows_free_transfers(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(nacao);
    let n = nation_allows_free_transfers(nacao);
    SCOREBOARD.record(r == n, || format!("nation_allows_free_transfers({:x}) original={} rust={}", nacao, r, n));
    r
}

unsafe extern "C" fn comparar_database_is_staff_eu_national(pessoa: usize, nacao_do_clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I2%%)(pessoa, nacao_do_clube);
    let n = database_is_staff_eu_national(pessoa, nacao_do_clube);
    SCOREBOARD.record(r == n, || format!("database_is_staff_eu_national({:x}, {:x}) original={} rust={}", pessoa, nacao_do_clube, r, n));
    r
}
