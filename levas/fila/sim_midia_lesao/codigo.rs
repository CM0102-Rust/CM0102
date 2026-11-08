// ---------- %%LEVA%%a leva: a história punível da imprensa, o tipo de lesão sorteado e a categoria da notícia (media.cpp, injury.cpp, news.cpp) ----------

/// { 0x7357c0 } (thiscall, ret 4, media.cpp): o tipo da história guardada na imprensa sobre o jogador
/// quando é de indisciplina (lista de tipos em 0xa17be0, terminada em 0xff), senão 0. Histórias: [+0]
/// vetor de 0x17 bytes, [+8] quantas; em +8 o id do jogador, em +0x11 o tipo (palavra). Jogador nulo ou
/// com +0x39 zerado: 0.
// ponytail: só o ax é fiel; os 16 bits altos do eax da original são resto de registrador (quem chama usa ax)
unsafe extern "thiscall" fn get_finable_story_kind(midia: usize, jogador: usize) -> u16 {
    if jogador == 0 || *((jogador + 0x39) as *const u32) == 0 {
        return 0;
    }
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    let id = *(jogador as *const u32);
    for i in 0..n.max(0) as usize {
        let e = base.wrapping_add(i * 0x17);
        if core::ptr::read_unaligned((e + 8) as *const u32) != id || *(0xa17be0 as *const u8) == 0xff {
            continue;
        }
        let tipo = core::ptr::read_unaligned((e + 0x11) as *const u16);
        let mut p = 0xa17be0usize;
        loop {
            if tipo == *(p as *const i8) as i16 as u16 {
                return tipo;
            }
            p += 1;
            if *(p as *const u8) == 0xff {
                break;
            }
        }
    }
    0 // nenhuma história de indisciplina
}

/// { 0x62d140 } (thiscall, ret 0xc, injury.cpp): o tipo de lesão da categoria (parte do corpo) e da
/// gravidade pedidas: se o jogador já tem lesão atual (0x62ed70) de tipo fora da lista de leves, devolve
/// esse tipo. Senão sorteia cada tipo da categoria (primeiro/último em 0x9e7f05/06 + 3 × categoria; a
/// tabela de tipos tem 13 bytes a partir de 0x9e7ad8: +1 categoria, +2 peso, +0xc gravidade, 0xff =
/// qualquer) e fica com o menor sorteio (menor que 100); o tipo recorrente do jogador ([this] + 31 × id +
/// 0x12) sorteia só 0..2 − 1. Gravidade 3: nenhum (0xff).
// ponytail: só o al é fiel (a original devolve char)
unsafe extern "thiscall" fn choose_injury_type(lesoes: usize, categoria: i8, gravidade: i8, jogador: usize) -> u8 {
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut melhor: i8 = 0x64;
    let mut escolhido: u8 = 0xff;
    let mut recorrente: u8 = 0xff;
    let a = atual(lesoes, jogador);
    if a != 0 {
        let t = *(a as *const u8);
        if !matches!(t, 0 | 1 | 2 | 3 | 4 | 0x4c | 0x4d | 0x4e | 0x4f | 0x50 | 0x51) {
            return t;
        }
    }
    let tipo = |k: i8, o: usize| *((0x9e7ad8 + o) as *const u8).offset(k as isize * 13);
    if jogador != 0 {
        let id = *(jogador as *const u32);
        let r = *((*(lesoes as *const u32)).wrapping_add(id.wrapping_mul(31)).wrapping_add(0x12) as usize as *const u8);
        if r != 0xff && tipo(r as i8, 1) as i8 == categoria {
            recorrente = r;
        }
    }
    let c = categoria as isize * 3;
    let mut bl = *((0x9e7f05 + c) as *const i8);
    let ultimo = (0x9e7f06 + c) as *const i8;
    if bl > *ultimo {
        return escolhido;
    }
    loop {
        let g = tipo(bl, 0xc) as i8;
        let serve = (gravidade == -1 || g == gravidade || g == -1) && gravidade != 3;
        if serve {
            let r = if bl as u8 == recorrente {
                (sorteio(3) as u8).wrapping_sub(1) as i8
            } else {
                sorteio(tipo(bl, 2) as i8 as i32) as i8
            };
            if r < melhor {
                melhor = r;
                escolhido = bl as u8;
            }
        }
        bl = bl.wrapping_add(1);
        if bl > *ultimo {
            return escolhido;
        }
    }
}

/// { 0x785670 } (thiscall, news.cpp): a categoria da notícia pelo milhar do tipo (+0): 1..999 → 2,
/// 1000.. → 7, 2000.. → 3, 3000.. → 4, 4000.. → 5, 5000.. → 1, 6000..0x1790 → 6. Fora disso: a original
/// (caixa de erro). Devolve o eax inteiro como a original (o tipo com o al trocado).
unsafe extern "thiscall" fn get_category(noticia: usize) -> u32 {
    let t = *(noticia as *const i32);
    let c = match t {
        1..=999 => 2,
        1000..=1999 => 7,
        2000..=2999 => 3,
        3000..=3999 => 4,
        4000..=4999 => 5,
        5000..=5999 => 1,
        6000..=0x1790 => 6,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(noticia),
    };
    (t as u32 & 0xffff_ff00) | c
}

unsafe extern "thiscall" fn comparar_get_finable_story_kind(midia: usize, jogador: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u16>(%%I0%%)(midia, jogador);
    let n = get_finable_story_kind(midia, jogador);
    PLACAR.record(r == n, || format!("get_finable_story_kind original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_choose_injury_type(lesoes: usize, categoria: i8, gravidade: i8, jogador: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8>(%%I1%%);
    // ponytail: jogador nulo abre a caixa de erro do 0x62ed70; não roda duas vezes
    if jogador == 0 {
        return o(lesoes, categoria, gravidade, jogador);
    }
    // ponytail: a 0x62ed70 (primeiro efeito da 0x62d140, antes de qualquer sorteio dela) passa pela
    // 0x62c680, que sorteia só na primeira consulta do dia e guarda em cache; chamá-la aqui antes enche o
    // cache na mesma ordem da original, e os dois lados do lado_a_lado acham o cache cheio
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    atual(lesoes, jogador);
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS,
        || o(lesoes, categoria, gravidade, jogador), || crate::rng::without_counting(|| choose_injury_type(lesoes, categoria, gravidade, jogador)));
    PLACAR.record(igual && r == n, || format!("choose_injury_type({}, {}) original={} rust={} sorteio igual={}", categoria, gravidade, r, n, igual));
    r
}

unsafe extern "thiscall" fn comparar_get_category(noticia: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let t = *(noticia as *const i32);
    if !(1..=0x1790).contains(&t) {
        return o(noticia);
    }
    let r = o(noticia);
    let n = get_category(noticia);
    PLACAR.record(r == n, || format!("get_category({}) original={:x} rust={:x}", t, r, n));
    r
}
