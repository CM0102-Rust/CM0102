// ---------- %%LEVA%%a leva: a família da cor, se duas cores se confundem e o atributo calculado do não jogador (Database.cpp) ----------

/// As cores fixas (chaves do index.cpp em 0x9cf178..0x9cf1fc, ids) e a família de cada uma, na ordem
/// das comparações de 0x540520.
const COLOUR_FAMILIES: [(usize, u8); 34] = [
    (0x9cf1e4, 10), (0x9cf1e8, 10), (0x9cf178, 1), (0x9cf1d0, 1), (0x9cf17c, 2), (0x9cf180, 3), (0x9cf184, 3),
    (0x9cf188, 3), (0x9cf18c, 8), (0x9cf190, 8), (0x9cf194, 8), (0x9cf198, 8), (0x9cf19c, 9), (0x9cf1a0, 9),
    (0x9cf1f0, 4), (0x9cf1a4, 4), (0x9cf1a8, 4), (0x9cf1fc, 4), (0x9cf1ac, 5), (0x9cf1b0, 5), (0x9cf1b4, 5),
    (0x9cf1b8, 5), (0x9cf1bc, 5), (0x9cf1c0, 6), (0x9cf1c4, 6), (0x9cf1c8, 6), (0x9cf1cc, 6), (0x9cf1f4, 6),
    (0x9cf1f8, 6), (0x9cf1d4, 7), (0x9cf1d8, 7), (0x9cf1dc, 7), (0x9cf1e0, 7), (0x9cf1ec, 7),
];

/// A família da `cor` pela tabela; None se o id não está em nenhuma chave (caixa na original).
unsafe fn colour_family(cor: usize) -> Option<u8> {
    let id = *(cor as *const u32);
    COLOUR_FAMILIES.iter().find(|&&(a, _)| *(a as *const u32) == id).map(|&(_, f)| f)
}

/// { 0x540520 } (cdecl, Database.cpp): a família (1..10) da `cor` pelo id (+0), comparado com as chaves
/// de cor em `COLOUR_FAMILIES` (a primeira que casa). Cor fora da lista: a original (caixa "Error",
/// [0xb67a34] = 0 e 2).
unsafe extern "C" fn database_get_colour_family(cor: usize) -> u8 {
    match colour_family(cor) {
        Some(f) => f,
        None => original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(cor),
    }
}

/// { 0x540490 } (cdecl, Database.cpp): as cores `a` e `b` se confundem? Pelas famílias (0x540520, `a`
/// primeiro): iguais; 8 com 9; 8 com 7; 6 com 7, salvo quando a da família 6 está contra a cor com o id
/// da chave [0x9cf1dc] (exceção: não se confundem).
unsafe extern "C" fn database_colours_clash(a: usize, b: usize) -> u8 {
    let familia: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x540520usize);
    let fa = familia(a);
    let fb = familia(b);
    let par = |x: u8, y: u8| (fa == x && fb == y) || (fb == x && fa == y);
    let chave = *(0x9cf1dc as *const u32);
    if fa == fb || par(8, 9) || par(8, 7) {
        return 1;
    }
    // a família 6 contra a cor da chave é exceção: não se confundem (a original sai com 0 antes do par 6/7)
    if (fa == 6 && *(b as *const u32) == chave) || (fb == 6 && *(a as *const u32) == chave) {
        return 0;
    }
    par(6, 7) as u8
}

/// { 0x543fd0 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`, do `nao_jogador`
/// (+0x69 dela). Diretos: 1 pessoa +0x56, 4 pessoa +0x58, 7 não jogador +0x14, 12 +0x22. Os outros
/// (2 +0x11, 3 +0x10, 5 +0x17, 6 +0x18, 8 +0x19, 9 +0x1b, 10 +0x1e, 11 +0x21) pela fórmula de 0x544410:
/// x = (palavra +4 + 2 × byte) × 0,05 + 10; (x + 0,1 x²) / 3 + 0,5, preso em 1..20 e truncado. Pessoa
/// ou não jogador nulos ou atributo fora de 1..12: a original (caixa e −1).
unsafe extern "C" fn staff_compute_non_player_attribute(pessoa: usize, nao_jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) || nao_jogador == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I2%%)(pessoa, nao_jogador, atributo);
    }
    let byte = |p: usize, d: usize| *((p + d) as *const u8);
    let formula = |d: usize| {
        let b = *((nao_jogador + d) as *const i8) as f64;
        let x = (*((nao_jogador + 4) as *const i16) as f64 + (b + b)) * 0.05 + 10.0;
        let y = (x * x * 0.1 + x) * read_f64_at(0x968a80) + 0.5;
        match_pl_ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y }) as u8
    };
    match a {
        1 => byte(pessoa, 0x56),
        2 => formula(0x11),
        3 => formula(0x10),
        4 => byte(pessoa, 0x58),
        5 => formula(0x17),
        6 => formula(0x18),
        7 => byte(nao_jogador, 0x14),
        8 => formula(0x19),
        9 => formula(0x1b),
        10 => formula(0x1e),
        11 => formula(0x21),
        _ => byte(nao_jogador, 0x22),
    }
}

unsafe extern "C" fn comparar_database_get_colour_family(cor: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%);
    let r = o(cor);
    if colour_family(cor).is_none() {
        return r; // caixa: só a original
    }
    let n = database_get_colour_family(cor);
    PLACAR.registrar(r == n, || format!("database_get_colour_family({:x}: id {}) original={} rust={}", cor, *(cor as *const u32), r, n));
    r
}

unsafe extern "C" fn comparar_database_colours_clash(a: usize, b: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I1%%);
    let r = o(a, b);
    if colour_family(a).is_none() || colour_family(b).is_none() {
        return r; // caixa: só a original
    }
    let n = database_colours_clash(a, b);
    PLACAR.registrar(r == n, || format!("database_colours_clash({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

unsafe extern "C" fn comparar_staff_compute_non_player_attribute(pessoa: usize, nao_jogador: usize, atributo: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I2%%);
    let r = o(pessoa, nao_jogador, atributo);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) || nao_jogador == 0 {
        return r; // caixa: só a original
    }
    let n = staff_compute_non_player_attribute(pessoa, nao_jogador, atributo);
    PLACAR.registrar(r == n, || format!("staff_compute_non_player_attribute({:x}, {:x}, {}) original={} rust={}", pessoa, nao_jogador, a, r, n));
    r
}
