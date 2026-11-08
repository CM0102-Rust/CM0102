// ---------- %%LEVA%%a leva: as regiões em ordem de nome e a pessoa de um laço de opinião (Database.cpp) ----------

/// { 0x545dc0 } (cdecl, Database.cpp): `get_regions_sorted_by_name(idioma)`: enche o vetor global de 14 regiões
/// (0xae2104) com 1..14, guarda o `idioma` (byte) em [0xae232b] e o ordena pelo qsort da CRT (0x945333) com a
/// comparação 0x545e00 (pelo endereço do jogo); devolve o vetor. Nas telas de busca (search_screens 0x80970c,
/// 0x814b7e, 0x815d5e: o filtro de região).
unsafe extern "C" fn get_regions_sorted_by_name(idioma: u32) -> usize {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    for k in 0..14u8 {
        *((0xae2104 + k as usize) as *mut u8) = k + 1;
    }
    *(0xae232b as *mut u8) = idioma as u8;
    qsort(0xae2104, 14, 1, 0x545e00); // (dbt)
    0xae2104
}

unsafe extern "C" fn comparar_get_regions_sorted_by_name(idioma: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I0%%);
    let reg = [(0xae2104, 14), (0xae232b, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(idioma), || get_regions_sorted_by_name(idioma));
    PLACAR.record(igual && r == n, || format!("get_regions_sorted_by_name({}) original={:x} rust={:x} {:x?} x {:x?}", idioma as u8, r, n, mo, mr));
    r
}

/// { 0x545e00 } (cdecl, Database.cpp): `compare_region_names(a, b)`: comparação do qsort de bytes de região pelo
/// nome (0x616f20, do index.cpp, em 0xc8 bytes, no idioma de [0xae232b]) na comparação de textos 0x90c1d0 (sem
/// maiúsculas); `a` nulo: 1; `b` nulo: -1. A 0x616f20 zera o primeiro byte do texto antes de tudo (as locais nunca
/// são lidas sem valor). Pela 0x545dc0.
unsafe extern "C" fn compare_region_names(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let nome: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x616f20usize);
    let idioma = *(0xae232b as *const u8) as u32;
    let mut na = [0u8; 0xc8];
    let mut nb = [0u8; 0xc8];
    nome(*(a as *const u8) as u32, na.as_mut_ptr() as usize, 0xc8, idioma);
    nome(*(b as *const u8) as u32, nb.as_mut_ptr() as usize, 0xc8, idioma);
    compare_text_nocase(na.as_ptr(), nb.as_ptr()) // (dbt)
}

unsafe extern "C" fn comparar_compare_region_names(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_region_names(a, b);
    PLACAR.record(r == n, || format!("compare_region_names({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x549dd0 } (cdecl, Database.cpp): `staff_get_opinion_link_staff(pessoa, alvo)`: o outro lado do laço de opinião
/// da pessoa: no registro da pessoa ([0xae2d0c] + id × 79; pessoa nula ou id >= [0xae2364]: 0), com laço (+0x27 ≠
/// -1), se o alvo é o +0x37: a pessoa de índice +0x27 ([0xae23bc] + 110 × índice); senão, se o alvo é o +0x4b: a de
/// índice +0x3b; senão 0. Na aba de transferências do perfil da pessoa (staff_screens 0x87dd80).
unsafe extern "C" fn staff_get_opinion_link_staff(pessoa: usize, alvo: u32) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let reg = (*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
    let ler = |d: usize| core::ptr::read_unaligned((reg + d) as *const u32);
    let pessoas = *(0xae23bc as *const usize);
    if ler(0x27) == u32::MAX {
        return 0;
    }
    let indice = if ler(0x37) == alvo {
        ler(0x27)
    } else if ler(0x4b) == alvo {
        ler(0x3b) // (dbt)
    } else {
        return 0;
    };
    pessoas.wrapping_add((indice as usize).wrapping_mul(110))
}

unsafe extern "C" fn comparar_staff_get_opinion_link_staff(pessoa: usize, alvo: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I2%%)(pessoa, alvo);
    let n = staff_get_opinion_link_staff(pessoa, alvo);
    PLACAR.record(r == n, || format!("staff_get_opinion_link_staff({:x}, {:x}) original={:x} rust={:x}", pessoa, alvo, r, n));
    r
}
