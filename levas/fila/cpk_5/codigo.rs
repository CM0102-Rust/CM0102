// ---------- %%LEVA%%a leva: as notícias para os técnicos humanos de um continente e da região da competição, e se a notícia é importante para a pessoa ----------

unsafe fn cpk5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x786640 } (thiscall, ret 8, news.cpp): `News::send_to_continent_managers(noticia, continente)`: alcance do
/// continente (+0xde = 3) e a entrega (0x785d40 com 0) a cada técnico humano ativo cujo clube (+0x39) tem a nação
/// (+0x53) no `continente` (+0x71) ou cuja nação (+0x1a) está nele. Devolve 1, ou 0 na primeira recusada. Notícia ou
/// continente nulos: a original (caixa "Error").
unsafe extern "thiscall" fn send_to_continent_managers(this: usize, noticia: usize, continente: usize) -> u32 {
    if noticia == 0 || continente == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, noticia, continente); // caixa: só a original
    }
    deliver_to_human_managers(this, noticia, 3, |p| {
        let c = cpk5_d(p + 0x39);
        if c != 0 {
            let n = cpk5_d(c + 0x53);
            if n != 0 && cpk5_d(n + 0x71) == continente { // o continente da nação do clube
                crate::comparison::branch("cpk_5 continent by club");
                return true;
            }
        }
        let s = cpk5_d(p + 0x1a);
        s != 0 && cpk5_d(s + 0x71) == continente // o continente da nação da pessoa
    }) as u32
}

/// { 0x786860 } (thiscall, ret 8, news.cpp): `News::send_to_comp_region(noticia, comp)`: com a nação da competição
/// (+0x5d), para os técnicos da nação (0x786520); senão, com o continente (+0x59), para os do continente (0x786640);
/// senão para todos (0x786780). Devolve o que a entrega devolve. Notícia ou competição nulas: a original (caixa
/// "Error").
unsafe extern "thiscall" fn send_to_comp_region(this: usize, noticia: usize, comp: usize) -> u32 {
    if noticia == 0 || comp == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, noticia, comp); // caixa: só a original
    }
    let nacao = cpk5_d(comp + 0x5d);
    if nacao != 0 {
        crate::comparison::branch("cpk_5 news of the competition nation");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x786520usize);
        return f(this, noticia, nacao);
    }
    let continente = cpk5_d(comp + 0x59);
    if continente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x786640usize);
        return f(this, noticia, continente);
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
    f(this, noticia)
}

/// O técnico (+0xcf) do clube cujo id está no dado da notícia em `o` ([0xae23b4], 0x245 bytes por clube).
unsafe fn manager_of_news_club(noticia: usize, o: usize) -> usize {
    cpk5_d(cpk5_d(0xae23b4).wrapping_add(cpk5_d(noticia + o).wrapping_mul(0x245)) + 0xcf)
}

/// { 0x784eb0 } (thiscall, ret 4, news.cpp): `NewsItem::is_important_for(pessoa)`: a notícia é importante para a
/// pessoa (o byte +4 que a entrega 0x785d40 grava)? Pela categoria (0x785670) e o tipo (+0): categoria 2, os tipos 7 a
/// 0xb, 0x27 e 0x28; categoria 4, 0xbb8, 0xbb9 e 0xbc4; categoria 5, uma lista fixa, 0xfa1 se a pessoa é o técnico do
/// clube do dado 5 (+0x19) e 0xfb6 se é o do dado 4 (+0x15) ou do 5; categoria 6, 0x1770 se o dado 0 (+5) é um dos
/// 16 técnicos humanos; categoria 1, 0x1388; o resto, 0. A original chama a 0x785670 até seis vezes (só lê): a cópia,
/// uma. Pessoa nula ou tipo fora de 1..0x1790 (caixa da 0x785670): a original.
unsafe extern "thiscall" fn is_important_for(noticia: usize, pessoa: usize) -> u8 {
    let t = cpk5_d(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(noticia, pessoa); // caixa: só a original
    }
    let categoria: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
    let r = match categoria(noticia) as u8 {
        2 => matches!(t, 7..=0xb | 0x27 | 0x28),
        4 => matches!(t, 0xbb8 | 0xbb9 | 0xbc4),
        5 => match t {
            0xfa0 | 0xfa4 | 0xfad | 0xfae | 0xfaf | 0xfb0 | 0xfb2 | 0xfb4 | 0xfb7 | 0xfb8 | 0xfbb | 0xfbf | 0xfc0 => true,
            0xfa1 => {
                crate::comparison::branch("cpk_5 important offer by club");
                manager_of_news_club(noticia, 0x19) == pessoa // o técnico do clube da oferta
            }
            0xfb6 => manager_of_news_club(noticia, 0x15) == pessoa || manager_of_news_club(noticia, 0x19) == pessoa,
            _ => false,
        },
        6 => t == 0x1770 && cpk5_d(noticia + 5) as i32 >= (cpk5_d(0xae2364) as i32).wrapping_sub(0x10),
        1 => t == 0x1388,
        _ => false, // 3 e 7: nenhum
    };
    r as u8
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn compare_send_to_continent_managers(this: usize, noticia: usize, continente: usize) -> u32 {
    crate::comparison::branch("send_to_continent_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, noticia, continente)
}

unsafe extern "thiscall" fn compare_send_to_comp_region(this: usize, noticia: usize, comp: usize) -> u32 {
    crate::comparison::branch("send_to_comp_region");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, noticia, comp)
}

/// Comparador: pelo valor (a 0x785670 só lê).
unsafe extern "thiscall" fn compare_is_important_for(noticia: usize, pessoa: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(noticia, pessoa);
    let t = cpk5_d(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return r;
    }
    let n = is_important_for(noticia, pessoa);
    SCOREBOARD.record(r == n, || format!("is_important_for(tipo {:#x}, {:x}) original={} rust={}", t, pessoa, r, n));
    r
}
