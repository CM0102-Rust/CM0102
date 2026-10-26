// ---------- %%LEVA%%a leva: as ordens dos clubes pelo nome curto e das pessoas pelo nome completo, e se a pessoa confere com os nomes (Database.cpp) ----------

/// A comparação de textos sem maiúsculas 0x90c1d0(a, b, 1, 0) que as ordens do Database.cpp usam.
unsafe fn dbr_comparar_textos(a: *const u8, b: *const u8) -> i32 {
    let textos: unsafe extern "C" fn(*const u8, *const u8, i32, u32) -> i32 = core::mem::transmute(0x90c1d0usize);
    textos(a, b, 1, 0)
}

/// { 0x540080 } (cdecl, Database.cpp): `database_compare_clubs_by_short_name(a, b)`: comparação do qsort de
/// ponteiros de clube pelo nome curto (+0x38) na comparação de textos 0x90c1d0 (sem maiúsculas); clube nulo vale o
/// texto vazio. A original copia os dois nomes para a pilha antes de comparar; a cópia compara no lugar (a 0x90c1d0
/// só lê). Nas telas de clubes e de busca, no Setup (0x822260) e na semeadura da UEFA (0x906410).
unsafe extern "C" fn ordem_dos_clubes_pelo_nome_curto(a: usize, b: usize) -> i32 {
    let nome = |p: usize| {
        let c = *(p as *const usize);
        if c == 0 { b"\0".as_ptr() } else { (c + 0x38) as *const u8 } // o nome curto
    };
    dbr_comparar_textos(nome(a), nome(b))
}

unsafe extern "C" fn comparar_ordem_dos_clubes_pelo_nome_curto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = ordem_dos_clubes_pelo_nome_curto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_clubes_pelo_nome_curto({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x540270 } (cdecl, Database.cpp): `database_compare_staff_by_full_name(a, b)`: comparação do qsort de
/// ponteiros de pessoa pelo nome no estilo 1 ("nome sobrenome", 0x53dba0 em 0x7d0 bytes) na comparação de textos
/// 0x90c1d0; pessoa nula vale o texto vazio. No menu (0x7612c0) e no gerente de transferências (0x8e55f0).
unsafe extern "C" fn ordem_das_pessoas_pelo_nome_completo(a: usize, b: usize) -> i32 {
    let formatar: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (*(a as *const usize), *(b as *const usize));
    if pa != 0 {
        formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 1); // "nome sobrenome"
    }
    if pb != 0 {
        formatar(pb, nb.as_mut_ptr() as usize, 0x7d0, 1);
    }
    dbr_comparar_textos(na.as_ptr(), nb.as_ptr())
}

unsafe extern "C" fn comparar_ordem_das_pessoas_pelo_nome_completo(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = ordem_das_pessoas_pelo_nome_completo(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_pessoas_pelo_nome_completo({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53e990 } (cdecl, Database.cpp): `database_staff_matches_names(pessoa, nome, sobrenome, comum, data)`: 1 se a
/// pessoa, que tem de ter registro de jogador (+0x61) ou de não jogador (+0x69), confere: cada nome não nulo (+4, +8,
/// +0xc) tem o id (+0x33) pedido e, com `data`, a data de nascimento (+0x10 dia, +0x12 ano) é a mesma (0x4a7f20);
/// senão 0. Pessoa nula: a original (caixa e 0). Telas, regeneração e transferências (~137 chamadores).
unsafe extern "C" fn pessoa_confere_com_os_nomes(pessoa: usize, nome: u32, sobrenome: u32, comum: u32, data: usize) -> u32 {
    if pessoa == 0 {
        return orig::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(%%I2%%)(pessoa, nome, sobrenome, comum, data);
    }
    let ler = |d: usize| core::ptr::read_unaligned((pessoa + d) as *const usize);
    if ler(0x61) == 0 && ler(0x69) == 0 {
        return 0;
    }
    for (d, id) in [(4, nome), (8, sobrenome), (0xc, comum)] {
        let n = ler(d);
        if n != 0 && core::ptr::read_unaligned((n + 0x33) as *const u32) != id {
            return 0; // o nome não confere
        }
    }
    if data != 0 {
        let w = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const u16);
        if w(pessoa, 0x12) != w(data, 2) || w(pessoa, 0x10) != w(data, 0) {
            return 0; // outra data de nascimento
        }
    }
    1
}

unsafe extern "C" fn comparar_pessoa_confere_com_os_nomes(pessoa: usize, nome: u32, sobrenome: u32, comum: u32, data: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(%%I2%%);
    if pessoa == 0 {
        return o(pessoa, nome, sobrenome, comum, data); // caixa: só a original
    }
    let r = o(pessoa, nome, sobrenome, comum, data);
    let n = pessoa_confere_com_os_nomes(pessoa, nome, sobrenome, comum, data);
    PLACAR.registrar(r == n, || format!("pessoa_confere_com_os_nomes({:x}, {:x}, {:x}, {:x}, {:x}) original={} rust={}", pessoa, nome, sobrenome, comum, data, r, n));
    r
}
