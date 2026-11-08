// ---------- %%LEVA%%a leva: a mídia e a caixa: a nota do jogador para uma história, o texto da crise financeira e as notícias da caixa do humano (media.cpp, news.cpp) ----------

unsafe fn cpn3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpn3_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpn3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A notícia guardada na caixa: 0xdf bytes (+0xd6 id).
const MEDIA_NEWS_ITEM_SIZE: usize = 0xdf;

/// A razão entre a reputação do jogador (+0xb do registro +0x61, no mínimo 2000) e a do clube (+0x80, entre 2000 e
/// 7500), como o fild/fidiv do original.
unsafe fn player_club_reputation_ratio(jogador: usize, clube: usize) -> f64 {
    let a = cpn3_w(cpn3_d(jogador + 0x61) + 0xb);
    let a = if a < 2000 { 2000 } else { a as i32 };
    let b = cpn3_w(clube + 0x80);
    let b = if b < 2000 { 2000 } else if b > 7500 { 7500 } else { b as i32 };
    a as f64 / b as f64
}

/// O tipo de história (0..0x52) → o caso do original (a tabela de bytes de 0x736b8c; 12 = a nota como está).
fn story_type_case(tipo: i16) -> u8 {
    match tipo {
        0 => 0,
        1 | 2 | 9 => 1,
        3 => 2,
        4 => 3,
        6 => 4,
        7 => 5,
        12 => 6,
        13 => 7,
        17 => 8,
        30 => 9,
        35 => 10,
        82 => 11,
        _ => 12,
    }
}

/// { 0x736720 } (thiscall, ret 0x18, media.cpp): `Media::rate_player_form_for_story(jogador, clube, contrato, tipo,
/// modo, peso)`: a nota (x87, em st0) do jogador para uma história da mídia. Sem contrato, o do jogador com o clube
/// (0x4cf5f0); sem nenhum, 0. Base: a nota média da liga x ([0xb74350] +0x10 / +0x18, sem jogos 6,8), as estatísticas
/// do jogador (0x7c1e30, categoria 6: s1 campo 1, s2 campo 0x11, s3 campo 2, padrão 6,5) e a média das notas
/// (0x7c1950) × 0,2: r = (s2 − x) + (média − x) − peso/60; com s1 < 12, r × s1/12; negativa, × a razão das
/// reputações. Depois, pelo tipo (0..0x52), somas do tipo de contrato (+0x4f >> 4), da idade, dos dias de casa
/// (0x54cdb0), de atributos do registro +0x61, do peso; o tipo 0x52 dá r × 0,75 + s3/max(s1, 1) − 0,4.
unsafe extern "thiscall" fn rate_player_form_for_story(_this: usize, jogador: usize, clube: usize, contrato: usize, tipo: u32, modo: u32, peso: f32) -> f64 {
    let mut contrato = contrato;
    if contrato == 0 {
        let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        contrato = com_clube(cpn3_d(0xae1050), jogador, clube);
        if contrato == 0 {
            return 0.0;
        }
    }
    let estat: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let media_notas: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7c1950usize);
    let g = cpn3_d(0xb74350);
    let jogos = core::ptr::read_unaligned((g + 0x18) as *const f64);
    let x = if jogos == 0.0 || jogos.is_nan() { 6.8 } else { core::ptr::read_unaligned((g + 0x10) as *const f64) / jogos };
    let xf = x as f32;
    let s1f = estat(cpn3_d(0xb74350), cpn3_d(jogador + 0x61), 6, 1, 6.5) as f32;
    let s2f = estat(cpn3_d(0xb74350), cpn3_d(jogador + 0x61), 6, 0x11, 6.5) as f32;
    let media = media_notas(cpn3_d(0xb74350), cpn3_d(jogador + 0x61)) as u8 as i8 as i32;
    let mut l0 = (media as f64 * 0.2) as f32;
    let y = if s1f as f64 > 1.0 { s1f as f64 } else { 1.0 }; // as partidas, no mínimo 1
    let s3 = estat(cpn3_d(0xb74350), cpn3_d(jogador + 0x61), 6, 2, 6.5);
    let l2 = (s3 / y) as f32;
    l0 = (l0 as f64 - xf as f64) as f32;
    let dif = s2f as f64 - xf as f64;
    let dif_f = dif as f32;
    crate::comparison::branch("cpn_3 nota do jogador");
    let mut r = dif + l0 as f64 - peso as f64 * (1.0f32 / 60.0) as f64; // o peso da história
    if !(s1f as f64 >= 12.0) {
        crate::comparison::branch("cpn_3 poucas partidas");
        r = r * s1f as f64 * (1.0f32 / 12.0) as f64; // menos de 12 partidas
    }
    if !(r >= 0.0) {
        crate::comparison::branch("cpn_3 nota negativa");
        r = r * player_club_reputation_ratio(jogador, clube); // a negativa pela reputação
    }
    let rf = r as f32;
    let tipo = tipo as u16 as i16;
    if !(0..=0x52).contains(&tipo) {
        return r;
    }
    let c = *((contrato + 0x4f) as *const u8) >> 4; // o tipo de contrato
    match story_type_case(tipo) {
        0 => match c {
            1 => r - 0.5,
            2 => r - 0.35f32 as f64,
            3 => r - 0.2f32 as f64,
            _ => r + 0.25,
        },
        1 => {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            let d = dias(0xae2c90, cpn3_d(jogador + 0x3e) as u32, cpn3_d(jogador + 0x42) as u32);
            (200i32.wrapping_sub(d)) as f64 * 0.1f32 as f64 + rf as f64
        }
        2 => {
            let r2 = r + dif_f as f64;
            match c {
                1 => r2 + 2.0,
                2 => r2 + 1.5,
                3 => r2 + 1.0,
                _ => r2,
            }
        }
        3 => r + (0x23 - cpn3_b(jogador + 0x18) as i32) as f64 * (1.0f32 / 3.0) as f64,
        4 => r + (10 - cpn3_b(cpn3_d(jogador + 0x61) + 0x25) as i32) as f64 * 0.2f32 as f64,
        5 => r + (cpn3_b(jogador + 0x18) as i32 - 0x13) as f64 * (1.0f32 / 3.0) as f64,
        6 => r + (cpn3_b(cpn3_d(jogador + 0x61) + 0x2f) as i32 - 10) as f64 * (1.0f32 / 3.0) as f64,
        7 => r + l0 as f64,
        8 => {
            let r2 = if r > 0.0 { r / player_club_reputation_ratio(jogador, clube) } else { r };
            match c {
                1 => r2 - 0.5,
                2 => r2 - 0.35f32 as f64,
                3 => r2 - 0.2f32 as f64,
                _ => r2,
            }
        }
        9 => r - peso as f64 * 0.1f32 as f64,
        10 => match c {
            0 => r - 1.0,
            1 => r + 2.0,
            2 => r + 1.5,
            3 => r + 1.0,
            7 => {
                if modo as u8 == 3 { 0.0 } else { r - 3.0 }
            }
            _ => r,
        },
        11 => r * 0.75 + l2 as f64 - 0.4f32 as f64,
        _ => r,
    }
}

/// { 0x74af60 } (thiscall, ret 0xc, media.cpp): `Media::write_financial_trouble_news_text(noticia, corpo, saida)`: o
/// título ou o corpo da notícia do clube em crise: lê os campos 0 (o clube), 1 (a gravidade) e 2 da notícia (0x785830),
/// monta em 0xde1f64 (0x66f4e0) o texto pela gravidade (> 0x1a5e, > 0xcb2 no corpo; > 0x1a5e no título) com o nome do
/// clube (+0x38) e o gênero (+0x52), põe na `saida` (0x4ae660) e liga o nome ao clube (0x4ae8a0, tipo 0x7d5).
unsafe extern "thiscall" fn write_financial_trouble_news_text(_this: usize, noticia: usize, corpo: u32, saida: usize) {
    let campo: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x785830usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let por_texto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ae660usize);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x4ae8a0usize);
    crate::comparison::branch("cpn_3 texto da crise financeira");
    let id = campo(noticia, 0);
    let gravidade = campo(noticia, 1);
    campo(noticia, 2);
    let clube = cpn3_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245));
    let nome = clube + 0x38;
    let genero = cpn3_b(clube + 0x52) as i32;
    let modelo = if corpo != 0 {
        if gravidade > 0x1a5e {
            0xa436fc
        } else if gravidade > 0xcb2 {
            0xa43644
        } else {
            0xa43594
        }
    } else if gravidade > 0x1a5e {
        0xa43560 // "... financial mess"
    } else {
        0xa43528 // "Financial trouble at ..."
    };
    formatar(0xde1f64, modelo, genero, genero, nome);
    por_texto(saida, 0xde1f64);
    ligar(saida, nome, 0x7d5, clube);
}

/// { 0x786960 } (thiscall, ret 0xc, news.cpp): `News::get_inbox_items(tecnico, conta, categoria)`: um vetor novo de 100
/// notícias (malloc 0x944e46 de 0x571c, na primeira achada; `*conta` = 0) com as notícias do anel do humano, da mais nova
/// para a mais antiga até a vaga vazia, só as da `categoria` (0x785670) se ela não for 0, copiadas (0x786010); `*conta`
/// soma uma por notícia. Devolve o vetor (0 sem nenhuma; `*conta` intocada). Sem técnico ou técnico da máquina: a caixa
/// "Error" e 0.
unsafe extern "thiscall" fn get_inbox_items(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    if tecnico == 0 {
        show_error_at(0xa693ec, 0x21fa);
        return 0;
    }
    let idx = cpn3_i(tecnico).wrapping_sub(cpn3_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        show_error_at(0xa693ec, 0x2205);
        return 0;
    }
    let e = cpn3_d(this + 8) + idx as usize * 12;
    if cpn3_i(e) == -1 {
        return 0;
    }
    let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x786010usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let mut vetor = 0usize;
    let mut k = cpn3_i(e + 4) - 1;
    loop {
        if k < cpn3_i(e) {
            k = cpn3_i(e) + 0x63;
        }
        let vaga = cpn3_d(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE));
        if cpn3_d(vaga + 0xd6) == 0 {
            return vetor;
        }
        if categoria as u8 == 0 || tipo(vaga) as u8 == categoria as u8 {
            if vetor == 0 {
                vetor = alocar(0x571c);
                if vetor == 0 {
                    show_error_at(0xa693ec, 0x222a);
                    return 0;
                }
                *conta = 0;
            }
            let n = *conta;
            *conta = n + 1;
            copiar(vetor.wrapping_add((n as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE)), cpn3_d(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE)));
        }
        k -= 1;
        if k == cpn3_i(e + 4) - 1 {
            return vetor;
        }
    }
}

/// Comparador: a nota é pura (as estatísticas, as datas e o contrato só leem; nada sorteia): compara a devolução bit a
/// bit (a original e a cópia na mesma precisão dupla do x87).
unsafe extern "thiscall" fn comparar_rate_player_form_for_story(this: usize, jogador: usize, clube: usize, contrato: usize, tipo: u32, modo: u32, peso: f32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64>(%%I0%%)(this, jogador, clube, contrato, tipo, modo, peso);
    let n = rate_player_form_for_story(this, jogador, clube, contrato, tipo, modo, peso);
    PLACAR.record(o.to_bits() == n.to_bits() || (o.is_nan() && n.is_nan()),
        || format!("rate_player_form_for_story(tipo {}, modo {}, peso {}) {} x {}", tipo as u16 as i16, modo as u8, peso, o, n));
    o
}

/// Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original (a cópia
/// prova no modo Rust, pelo ramo).
unsafe extern "thiscall" fn comparar_write_financial_trouble_news_text(this: usize, noticia: usize, corpo: u32, saida: usize) {
    // ponytail: o objeto de texto não pode receber o texto duas vezes; só a original roda aqui
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%)(this, noticia, corpo, saida);
}

/// Comparador: a original aloca o vetor: roda só ela e confere o plano feito antes (a conta e os ids, na ordem).
unsafe extern "thiscall" fn comparar_get_inbox_items(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(%%I2%%);
    if tecnico == 0 || cpn3_i(tecnico).wrapping_sub(cpn3_i(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, conta, categoria);
    }
    let e = cpn3_d(this + 8) + (cpn3_i(tecnico).wrapping_sub(cpn3_i(0xae2364)).wrapping_add(0x10)) as usize * 12;
    let mut ids = Vec::new();
    if cpn3_i(e) != -1 {
        let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
        let mut k = cpn3_i(e + 4) - 1;
        loop {
            if k < cpn3_i(e) {
                k = cpn3_i(e) + 0x63;
            }
            let vaga = cpn3_d(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE));
            if cpn3_d(vaga + 0xd6) == 0 {
                break;
            }
            if categoria as u8 == 0 || tipo(vaga) as u8 == categoria as u8 {
                ids.push(cpn3_d(vaga + 0xd6));
            }
            k -= 1;
            if k == cpn3_i(e + 4) - 1 {
                break;
            }
        }
    }
    let r = o(this, tecnico, conta, categoria);
    let ok = if ids.is_empty() {
        r == 0
    } else {
        r == 0 || (*conta as usize == ids.len() && ids.iter().enumerate().all(|(i, &id)| cpn3_d(r + i * MEDIA_NEWS_ITEM_SIZE + 0xd6) == id))
    };
    PLACAR.record(ok, || format!("get_inbox_items(categoria {}) {} notícias, esperava {:?}", categoria as u8, if r == 0 { 0 } else { *conta }, ids));
    r
}
