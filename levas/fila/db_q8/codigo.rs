// ---------- %%LEVA%%a leva: as cores do kit do clube, o técnico do clube e o nome do clube em inglês (Database.cpp) ----------

/// { 0x53b640 } (cdecl, Database.cpp): as cores do `kit` (byte: 1 casa +0x83/+0x87, 2 fora
/// +0x8b/+0x8f, 3 terceiro +0x93/+0x97; 2 e 3 sem uma das duas cores: as de casa trocadas) do `clube`
/// em pixels (0x5e4800 com os bytes +0x37, +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que
/// começam com [0xaebdf0] e [0xae3184]. Outro kit: só os padrões. Devolve 1. Clube nulo: a
/// original (padrões, caixa "Error" e 0).
unsafe extern "C" fn database_get_club_kit_colours(clube: usize, kit: u32, frente: *mut u16, fundo: *mut u16) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(%%I0%%)(clube, kit, frente, fundo);
    }
    // ponytail: a original empurra dwords com só o byte de baixo escrito (o resto é lixo de pilha);
    // a 0x5e4800 faz & 0xff nos três, então os bytes estendidos dão o mesmo pixel
    let cor: unsafe extern "C" fn(u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5e4800usize);
    core::ptr::write_unaligned(frente, *(0xaebdf0 as *const u16));
    core::ptr::write_unaligned(fundo, *(0xae3184 as *const u16));
    let r = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    let (a, b) = match kit as u8 {
        1 => (r(0x83), r(0x87)), // casa
        2 | 3 => {
            let d = if kit as u8 == 2 { 0x8b } else { 0x93 };
            let (a, b) = (r(d), r(d + 4));
            if a == 0 || b == 0 { (r(0x87), r(0x83)) } else { (a, b) }
        }
        _ => return 1,
    };
    let pixel = |c: usize| {
        let byte = |d: usize| *((c + d) as *const u8) as u32;
        cor(byte(0x37), byte(0x38), byte(0x39), 0) as u16
    };
    if a != 0 {
        core::ptr::write_unaligned(frente, pixel(a));
    }
    if b != 0 {
        core::ptr::write_unaligned(fundo, pixel(b));
    }
    1
}

/// { 0x53ca90 } (cdecl, Database.cpp): o técnico (+0xcf) do `clube`; sem ele, o do parceiro A/B
/// (0x540a50 com 0 e 1); sem os dois, clube comum dá 0 e seleção (id ≥ [0xae235c] − [0xae2350]) repete
/// com a seleção da nação dela (0x53b3d0 com +0x53, a sub-21 chega na principal). Clube nulo (no começo
/// ou vindo da 0x53b3d0): a original com nulo (caixa "Error" e 0).
unsafe extern "C" fn database_get_club_manager(clube: usize) -> usize {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let selecao_da_nacao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let tecnico = |c: usize| core::ptr::read_unaligned((c + 0xcf) as *const usize);
    let mut c = clube;
    loop {
        if c == 0 {
            return original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%)(0);
        }
        let t = tecnico(c);
        if t != 0 {
            return t; // o técnico do clube
        }
        let p = parceiro(c, 0, 1);
        if p != 0 && tecnico(p) != 0 {
            return tecnico(p);
        }
        if *(c as *const i32) < (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32)) {
            return 0;
        }
        c = selecao_da_nacao(core::ptr::read_unaligned((c + 0x53) as *const usize));
    }
}

/// { 0x545880 } (cdecl, Database.cpp): o nome do `clube` em inglês. Idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20): pelo mapa
/// de ids [0xae23fc] se houver, senão pelo id se < quantos (+0), senão 0.
unsafe extern "C" fn club_get_english_name(clube: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return clube.wrapping_add(4); // idioma 0 ou 1
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I2%%)(clube);
    }
    let linhas = *((t + 0x20) as *const usize);
    let id = *(clube as *const i32);
    let mapa = *(0xae23fc as *const usize);
    if mapa != 0 {
        let i = *((mapa.wrapping_add((id as usize).wrapping_mul(4))) as *const i32);
        return linhas.wrapping_add((i as usize).wrapping_mul(79));
    }
    if id < *(t as *const i32) {
        linhas.wrapping_add((id as usize).wrapping_mul(79))
    } else {
        0
    }
}

unsafe extern "C" fn comparar_database_get_club_kit_colours(clube: usize, kit: u32, frente: *mut u16, fundo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(%%I0%%);
    if clube == 0 {
        return o(clube, kit, frente, fundo); // caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(frente as usize, 2), (fundo as usize, 2)],
        || o(clube, kit, frente, fundo), || database_get_club_kit_colours(clube, kit, frente, fundo));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_kit_colours({:x}, {}) original={} rust={} {:x?} x {:x?}", clube, kit as u8, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_get_club_manager(clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // 0x540a50 zera a marca +0x244 do clube quando recusa o parceiro (e a recusa se repete com ela zerada):
    // a região volta entre os lados
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube),
        || database_get_club_manager(clube));
    PLACAR.registrar(igual && r == n, || format!("database_get_club_manager({:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_club_get_english_name(clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I2%%);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(clube); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(clube);
    let n = club_get_english_name(clube);
    PLACAR.registrar(r == n, || format!("club_get_english_name({:x}) idioma {} original={:x} rust={:x}", clube, idioma, r, n));
    r
}
