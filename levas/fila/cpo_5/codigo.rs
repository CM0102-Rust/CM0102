// ---------- %%LEVA%%a leva: a notícia nova e a confiança da torcida e da diretoria depois de uma história da mídia (news.cpp, media.cpp) ----------

unsafe fn cpo5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo5_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpo5_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A linha do clube no gerente dos técnicos: [[0xb67a44]] + id × 0x49 (+6 confiança da diretoria, +8 da torcida, as duas
/// entre 1 e 10000; +0xe outra palavra).
unsafe fn manager_manager_row(clube: usize) -> usize {
    cpo5_d(cpo5_d(0xb67a44)).wrapping_add((cpo5_i(clube) as usize).wrapping_mul(0x49))
}

/// Os amortecedores comuns das duas confianças: com o preferido (`gosta`) a queda vale metade e a alta o dobro; com o
/// rejeitado (`nao_gosta`) o contrário; fora de ±1000, o excesso vale 0,2. Devolve a palavra baixa (o original usa `di`).
unsafe fn damp_confidence_change(v: i32, gosta: bool, nao_gosta: bool) -> i16 {
    let mut v = v;
    if gosta {
        let x = v as i16 as f64;
        v = if (v as i16) < 0 { match_pl_ftol(x * read_f64(0x968968)) } else { match_pl_ftol(x + x) };
    } else if nao_gosta {
        let x = v as i16 as f64;
        v = if (v as i16) > 0 { match_pl_ftol(x * read_f64(0x968968)) } else { match_pl_ftol(x + x) };
    }
    let x = v as i16 as i32;
    if x < -1000 {
        v = match_pl_ftol((x + 1000) as f64 * read_f64(0x968918) - read_f64(0x968908));
    } else if x > 1000 {
        v = match_pl_ftol((x - 1000) as f64 * read_f64(0x968918) + read_f64(0x968908));
    }
    v as i16
}

/// A variação da confiança da torcida (0x738340): conf = +8 da linha (0 sem clube); d = alvo − trunc((conf − 5000) ×
/// −0,05) (16 bits); v = trunc(d × 0,75); o técnico (+0xcf) favorito (0x547790) ou rejeitado (0x5477d0) pelo clube
/// amortece. As duas consultas são puras.
unsafe fn fan_confidence_change(clube: usize, alvo: u32) -> i16 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let conf = if clube != 0 { cpo5_w(manager_manager_row(clube) + 8) as i32 } else { 0 };
    let t = match_pl_ftol((conf - 5000) as f64 * read_f64(0x96d280));
    let d = (alvo as i32).wrapping_sub(t) as i16;
    let v = match_pl_ftol(d as f64 * read_f64(0x969040));
    let tecnico = cpo5_d(clube + 0xcf);
    let fav = favorito(tecnico, clube) != 0;
    let rej = !fav && rejeitado(tecnico, clube) != 0;
    damp_confidence_change(v, fav, rej)
}

/// A variação da confiança da diretoria (0x7386f0): conf = +6 da linha (0 sem clube); s = (conf − 6000) / 20 + alvo
/// (16 bits); v = trunc(s × 0,75); o presidente (+0xbf) gosta (0x547810) ou não gosta (0x547840) do técnico (+0xcf).
unsafe fn board_confidence_change(clube: usize, alvo: u32) -> i16 {
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let conf = if clube != 0 { cpo5_w(manager_manager_row(clube) + 6) as i32 } else { 0 };
    let s = ((conf - 6000) / 20).wrapping_add(alvo as i32) as i16;
    let v = match_pl_ftol(s as f64 * read_f64(0x969040));
    let (tecnico, presidente) = (cpo5_d(clube + 0xcf), cpo5_d(clube + 0xbf));
    let g = gosta(tecnico, presidente) != 0;
    let n = !g && nao_gosta(tecnico, presidente) != 0;
    damp_confidence_change(v, g, n)
}

/// Soma `v` na palavra da linha, entre 1 e 10000.
unsafe fn add_clamped_confidence(p: usize, v: i16) {
    let x = cpo5_w(p).wrapping_add(v);
    core::ptr::write_unaligned(p as *mut i16, x.clamp(1, 10000));
}

/// { 0x77bbe0 } (thiscall, ret 8, news.cpp, quadro SEH só da data, destrutor vazio 0x5a53c0): `NewsItem::NewsItem(tipo,
/// importante)`: inicia a data (+0xcd, 0x54a110), grava o tipo (+0), o byte importante (+4) e zera os 50 dados (+5);
/// a data de hoje ([0xae2c90], pela atribuição 0x417c10 com retorno numa temporária), o byte [0xae2c88] em +0xd5, o id
/// (+0xd6) e +0xda zerados. Devolve a notícia.
unsafe extern "thiscall" fn construct_news_item(this: usize, tipo: u32, importante: u32) -> usize {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    iniciar_data(this + 0xcd);
    cpo5_por(this, tipo as usize);
    *((this + 4) as *mut u8) = importante as u8;
    for k in 0..50 {
        cpo5_por(this + 5 + 4 * k, 0);
    }
    let mut temporaria = [0u32; 2];
    atribuir_data(this + 0xcd, temporaria.as_mut_ptr() as usize, 0xae2c90);
    *((this + 0xd5) as *mut u8) = *(0xae2c88 as *const u8);
    cpo5_por(this + 0xd6, 0);
    cpo5_por(this + 0xda, 0);
    this
}

/// Os dados do técnico (id e as três chaves +0x33 de +4, +8, +0xc) a partir de `k`.
unsafe fn cpo5_dados_da_pessoa(gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    gravar(n, k, cpo5_d(p) as u32);
    gravar(n, k + 1, cpo5_d(cpo5_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cpo5_d(cpo5_d(p + 8) + 0x33) as u32);
    gravar(n, k + 3, cpo5_d(cpo5_d(p + 0xc) + 0x33) as u32);
}

/// { 0x738340 } (thiscall, ret 0x14, media.cpp, quadro SEH sem destrutor): `Media::adjust_supporter_confidence(clube,
/// sujeito, tópico, alvo, variante)`: soma na confiança da torcida (+8 da linha do clube, entre 1 e 10000) a variação
/// (ver `fan_confidence_change`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0): 0 = 0, 1 =
/// tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750, 4 negativa, 3 até 750, 2
/// acima), 6..9 o técnico; com sujeito, 10..13 o sujeito e 14 = 6 favorito, 7 rejeitado, 8 nenhum; depois −1, a
/// confiança nova e 0.
unsafe extern "thiscall" fn adjust_supporter_confidence(_this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32) {
    let v = fan_confidence_change(clube, alvo);
    crate::comparison::branch("cpo_5 goal_crowd_reaction");
    if clube != 0 {
        add_clamped_confidence(manager_manager_row(clube) + 8, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbe0, 0);
    gravar(n, 0, 0);
    gravar(n, 1, topico as i16 as i32 as u32);
    gravar(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    gravar(n, 3, s as u32);
    gravar(n, 4, cpo5_d(clube) as u32);
    let grau = if v < -750 { 5 } else if v < 0 { 4 } else if v > 750 { 2 } else { 3 };
    gravar(n, 5, grau);
    cpo5_dados_da_pessoa(gravar, n, 6, cpo5_d(clube + 0xcf));
    let b: i8 = if sujeito != 0 {
        cpo5_dados_da_pessoa(gravar, n, 10, sujeito);
        let r = if favorito(sujeito, clube) != 0 {
            6
        } else if rejeitado(sujeito, clube) != 0 {
            7
        } else {
            8
        };
        gravar(n, 14, r);
        0xf
    } else {
        gravar(n, 10, u32::MAX);
        0xb
    };
    gravar(n, b, u32::MAX);
    let conf = if clube != 0 { cpo5_w(manager_manager_row(clube) + 8) as i32 } else { 0 };
    gravar(n, b + 1, conf as u32);
    gravar(n, b + 2, 0);
    entregar(cpo5_d(0xb7431c), n, clube);
}

/// { 0x7386f0 } (thiscall, ret 0x18, media.cpp, quadro SEH sem destrutor): `Media::adjust_board_confidence(clube,
/// sujeito, tópico, alvo, variante, sexto)`: soma na confiança da diretoria (+6 da linha do clube, entre 1 e 10000) a
/// variação (ver `board_confidence_change`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0):
/// 0 = 1, 1 = tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750 com confiança abaixo
/// de 4500; 9 negativa com confiança acima de 6250, senão 4; 2 acima de 750 com confiança acima de 6250, senão 3), 6..9
/// o técnico; com sujeito 10..13 e 14 = 6/7/8 (o presidente gosta, não gosta, nenhum); depois o presidente (ou −1), a
/// confiança nova, o sexto (com ele 1 e a palavra +0xe), e as marcas do sujeito: contrato 0x10/0x20 acima de 25 anos,
/// média das notas acima da média da liga × 5, +0x59 e +0x5b acima de 14, mais de 1500 dias no clube, valor (0x4d0c60)
/// acima de 4.500.000 e reputação (+5 da ficha) acima de 140.
unsafe extern "thiscall" fn adjust_board_confidence(_this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32, sexto: u32) {
    let v = board_confidence_change(clube, alvo);
    crate::comparison::branch("cpo_5 board");
    if clube != 0 {
        add_clamped_confidence(manager_manager_row(clube) + 6, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    crate::comparison::branch("cpo_5 board news");
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let ratings_average: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x7c1950usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let conf = || if clube != 0 { cpo5_w(manager_manager_row(clube) + 6) } else { 0 };
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbe0, 0);
    gravar(n, 0, 1);
    gravar(n, 1, topico as i16 as i32 as u32);
    gravar(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    gravar(n, 3, s as u32);
    gravar(n, 4, cpo5_d(clube) as u32);
    let grau = if v < -750 && (clube == 0 || conf() < 4500) {
        5
    } else if v < 0 {
        if clube != 0 && conf() > 6250 { 9 } else { 4 }
    } else if v > 750 && clube != 0 && conf() > 6250 {
        2
    } else {
        3
    };
    gravar(n, 5, grau);
    cpo5_dados_da_pessoa(gravar, n, 6, cpo5_d(clube + 0xcf));
    let mut b: i8 = if sujeito != 0 {
        cpo5_dados_da_pessoa(gravar, n, 10, sujeito);
        let presidente = cpo5_d(clube + 0xbf);
        let r = if gosta(sujeito, presidente) != 0 {
            6
        } else if nao_gosta(sujeito, presidente) != 0 {
            7
        } else {
            8
        };
        gravar(n, 14, r);
        0xf
    } else {
        gravar(n, 10, u32::MAX);
        0xb
    };
    let presidente = cpo5_d(clube + 0xbf);
    if presidente != 0 {
        cpo5_dados_da_pessoa(gravar, n, b, presidente);
        b += 4;
    } else {
        gravar(n, b, u32::MAX);
        b += 1;
    }
    gravar(n, b, conf() as i32 as u32);
    b += 1;
    if sexto != 0 {
        gravar(n, b, 1);
        let w = if clube != 0 { cpo5_w(manager_manager_row(clube) + 0xe) } else { 0 };
        gravar(n, b + 1, w as i32 as u32);
        b += 2;
    } else {
        gravar(n, b, 0);
        b += 1;
    }
    // as marcas do sujeito
    let mut m = 0u32;
    if sujeito != 0 && cpo5_d(sujeito + 0x61) != 0 {
        let c = contract_of(cpo5_d(0xae1050), sujeito);
        if c != 0 && cpo5_c(sujeito + 0x18) > 0x19 {
            let t = *((c + 0x4f) as *const u8) & 0xf0;
            if t == 0x20 || t == 0x10 {
                m = 1;
            }
        }
    }
    gravar(n, b, m);
    b += 1;
    let mut m = 0u32;
    if sujeito != 0 && cpo5_d(sujeito + 0x61) != 0 {
        let base = cpo5_d(0xb74350);
        let notas = ratings_average(base, cpo5_d(sujeito + 0x61)) as i8 as f64;
        let base = cpo5_d(0xb74350);
        let y = read_f64(base + 0x18);
        let c0 = read_f64(0x9689a0);
        let media = if !(y < c0 || y > c0) { read_f64(0x968998) } else { read_f64(base + 0x10) / y }; // sem jogos: 6,8
        if notas > media * read_f64(0x9688a0) {
            m = 1;
        }
    }
    gravar(n, b, m);
    b += 1;
    gravar(n, b, (sujeito != 0 && cpo5_c(sujeito + 0x59) > 0xe) as u32);
    b += 1;
    gravar(n, b, (sujeito != 0 && cpo5_c(sujeito + 0x5b) > 0xe) as u32);
    b += 1;
    let dias = sujeito != 0 && dias_ate(0xae2c90, cpo5_d(sujeito + 0x3e) as u32, cpo5_d(sujeito + 0x42) as u32) > 0x5dc;
    gravar(n, b, dias as u32);
    b += 1;
    let caro = sujeito != 0 && valor(cpo5_d(0xae1050), sujeito, clube, 0, u32::MAX, 0) > 0x44aa20;
    gravar(n, b, caro as u32);
    b += 1;
    let injury_word_0c = sujeito != 0 && cpo5_d(sujeito + 0x61) != 0 && cpo5_w(cpo5_d(sujeito + 0x61) + 5) > 0x8c;
    gravar(n, b, injury_word_0c as u32);
    entregar(cpo5_d(0xb7431c), n, clube);
}

/// Comparador: pela memória da notícia (0xde bytes; a original usa só uma temporária na pilha) e pela devolução.
unsafe extern "thiscall" fn compare_construct_news_item(this: usize, tipo: u32, importante: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0xde)], || o(this, tipo, importante), || construct_news_item(this, tipo, importante));
    SCOREBOARD.record(igual && r == n, || format!("construct_news_item(tipo {:#x}, {}) {:x?} x {:x?}", tipo, importante as u8, mo.0[0].1, mr.0[0].1));
    r
}

/// Comparador: a original sorteia e entrega a notícia (não pode rodar duas vezes): roda só ela e confere o plano feito
/// antes: a confiança da torcida nova (a variação é pura).
unsafe extern "thiscall" fn compare_adjust_supporter_confidence(this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I1%%);
    if clube == 0 {
        return o(this, clube, sujeito, topico, alvo, variante);
    }
    let p = manager_manager_row(clube) + 8;
    let antes = cpo5_w(p);
    let v = fan_confidence_change(clube, alvo);
    let esperado = antes.wrapping_add(v).clamp(1, 10000);
    o(this, clube, sujeito, topico, alvo, variante);
    let depois = cpo5_w(manager_manager_row(clube) + 8);
    SCOREBOARD.record(depois == esperado, || format!("adjust_supporter_confidence(clube {:x}, alvo {}) {} + {} = {} x original {}", clube, alvo as i16, antes, v, esperado, depois));
}

/// Comparador: como o da torcida, pela confiança da diretoria nova.
unsafe extern "thiscall" fn compare_adjust_board_confidence(this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32, sexto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(%%I2%%);
    if clube == 0 {
        return o(this, clube, sujeito, topico, alvo, variante, sexto);
    }
    let p = manager_manager_row(clube) + 6;
    let antes = cpo5_w(p);
    let v = board_confidence_change(clube, alvo);
    let esperado = antes.wrapping_add(v).clamp(1, 10000);
    o(this, clube, sujeito, topico, alvo, variante, sexto);
    let depois = cpo5_w(manager_manager_row(clube) + 6);
    SCOREBOARD.record(depois == esperado, || format!("adjust_board_confidence(clube {:x}, alvo {}) {} + {} = {} x original {}", clube, alvo as i32, antes, v, esperado, depois));
}
