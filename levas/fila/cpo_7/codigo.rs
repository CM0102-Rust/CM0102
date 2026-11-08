// ---------- %%LEVA%%a leva: as notícias do jogador depois de uma história da mídia: gratidão ao técnico, pedido de apoio e reação à crítica (media.cpp) ----------

unsafe fn cpo7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal (os atributos de 1 a 20, a idade).
unsafe fn cpo7_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

type Cpo7Gravar = unsafe extern "thiscall" fn(usize, i8, u32) -> u32;

/// Os dados da pessoa a partir de `k`: id e as três chaves +0x33 de +4, +8 e +0xc (a de +0xc pode faltar: −1, só
/// quando `opcional`).
unsafe fn set_person_slots_optional(gravar: Cpo7Gravar, n: usize, k: i8, p: usize, opcional: bool) {
    gravar(n, k, cpo7_d(p) as u32);
    gravar(n, k + 1, cpo7_d(cpo7_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cpo7_d(cpo7_d(p + 8) + 0x33) as u32);
    let q = cpo7_d(p + 0xc);
    gravar(n, k + 3, if opcional && q == 0 { u32::MAX } else { cpo7_d(q + 0x33) as u32 });
}

/// A dupla de transmutes comum: construir a notícia (0x77bbe0) e gravar um dado (0x785790).
unsafe fn news_item_fns() -> (unsafe extern "thiscall" fn(usize, u32, u32) -> usize, Cpo7Gravar) {
    (core::mem::transmute(0x77bbe0usize), core::mem::transmute(0x785790usize))
}

/// Os dias desde a chegada da pessoa (+0x3e/+0x42) até hoje (0x54cdb0 em 0xae2c90).
unsafe fn days_since_arrival(p: usize) -> i32 {
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    dias_ate(0xae2c90, cpo7_d(p + 0x3e) as u32, cpo7_d(p + 0x42) as u32)
}

/// O sorteio da gratidão pelo tipo (índices da tabela de bytes 0x732fcc, saltos 0x732fb4): 0 os tipos 0, 6, 14 e 17; 1
/// os 2 e 30 (sorteio(2)); 2 os 4, 9, 33, 34 e 37 (sorteio(3)); 3 os 5 e 7 (sorteio(4)); o resto 0.
const CPO7_CASO: [u8; 0x26] = [0, 5, 1, 5, 2, 3, 0, 3, 5, 2, 5, 5, 5, 4, 0, 5, 5, 0, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 2, 2, 4, 5, 2];

/// { 0x732c80 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::queue_player_gratitude_news(jogador,
/// clube, tipo, variante)`: com jogador com ficha (+0x61), clube com técnico (+0xcf) e o contrato dele com o clube
/// (0x4cf5f0), a notícia 0xbda (local) ao técnico (0x7861e0): 0..3 o jogador (3 = −1 sem +0xc), 4 = id do clube, 5 =
/// o tipo, 6 = a variante, 7 = o sorteio do tipo, 8..0xb o técnico, 0xc = 0x420e9, 0xd..0x10 o auxiliar (+0xd3) ou −1.
/// O tipo: se o jogador está na lista da mídia (this +0/+8, 0x17 bytes, id em +8 e marca 2 em +0x10) ou o contrato tem
/// 0x2000 ou 8 em +0x45, os tipos 0, 2, 4, 7, 9, 0xe e 0x23 viram 0x12 (+0x5d ≥ sorteio(10)) ou 0x21; lesionado
/// (0x62ed70): 0xc. Nos tipos 0, 6, 14 e 17: com +0x44 da ficha abaixo de 10, sorteio(3) + 2; senão sorteio(5), e 0 ou 1
/// sobe +0x44 da ficha (até 20).
unsafe extern "thiscall" fn queue_player_gratitude_news(this: usize, jogador: usize, clube: usize, tipo: u32, variante: u32) {
    if jogador == 0 || cpo7_d(jogador + 0x61) == 0 || clube == 0 || cpo7_d(clube + 0xcf) == 0 {
        return;
    }
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = contract_with_club(cpo7_d(0xae1050), jogador, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 gratidao");
    let (construir, gravar) = news_item_fns();
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let lesionado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x62ed70usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbda, 0);
    set_person_slots_optional(gravar, n, 0, jogador, true);
    gravar(n, 4, cpo7_d(clube) as u32);
    // a lista da mídia (this +0, conta em +8)
    let mut in_list = false;
    for i in 0..core::ptr::read_unaligned((this + 8) as *const i32).max(0) as usize {
        let e = cpo7_d(this) + i * 0x17;
        if cpo7_d(e + 8) == cpo7_d(jogador) && *((e + 0x10) as *const u8) == 2 {
            in_list = true;
            break;
        }
    }
    let f45 = cpo7_d(contrato + 0x45);
    let mut t = tipo;
    if in_list || f45 & 0x2000 != 0 || f45 & 8 != 0 {
        if [0xe, 0, 2, 9, 0x23, 4, 7].contains(&(tipo as i16)) {
            crate::comparacao::ramo("cpo_7 gratidao tipo");
            let s = sorteio(10);
            t = if cpo7_c(jogador + 0x5d) >= s { 0x12 } else { 0x21 };
        }
    }
    if lesionado(cpo7_d(0xb63d28), jogador) != 0 {
        t = 0xc;
    }
    let t = t as i16 as i32;
    gravar(n, 5, t as u32);
    gravar(n, 6, variante as i8 as i32 as u32);
    let v = match if (t as u32) <= 0x25 { CPO7_CASO[t as usize] } else { 5 } {
        0 => {
            let ficha = cpo7_d(jogador + 0x61);
            if ficha != 0 && cpo7_c(ficha + 0x44) < 10 {
                (sorteio(3) as u8).wrapping_add(2) as i8 as i32
            } else {
                let s = sorteio(5);
                if s == 0 || s == 1 {
                    let ficha = cpo7_d(jogador + 0x61);
                    if ficha != 0 && cpo7_c(ficha + 0x44) < 0x14 {
                        crate::comparacao::ramo("cpo_7 gratidao sobe");
                        *((ficha + 0x44) as *mut u8) += 1;
                    }
                }
                s
            }
        }
        1 => sorteio(2),
        2 => {
            crate::comparacao::ramo("cpo_7 gratidao sorteio 3");
            sorteio(3)
        }
        3 => sorteio(4),
        _ => 0,
    };
    gravar(n, 7, v as u32);
    let tecnico = cpo7_d(clube + 0xcf);
    set_person_slots_optional(gravar, n, 8, tecnico, false);
    gravar(n, 0xc, 0x420e9);
    let auxiliar = cpo7_d(clube + 0xd3);
    if auxiliar != 0 {
        set_person_slots_optional(gravar, n, 0xd, auxiliar, false);
    } else {
        gravar(n, 0xd, u32::MAX);
    }
    entregar(cpo7_d(0xb7431c), n, clube);
}

/// { 0x743370 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::post_player_backing_request_news(
/// jogador, clube, tipo, quarto)`: com jogador com ficha (+0x61) e clube humano (0x600a50), a notícia 0xbcf (local) ao
/// técnico do clube (0x7861e0): 0..3 o jogador (3 = −1 sem +0xc), 4 = id do clube, 5/6 = (1, ·) com reputação (+0xb da
/// ficha) acima de 4250, senão (2, ·): o segundo 1 se (+0x57 > 15 e +0x5d < 6, ou +0x5b < 10 e +0x59 < 6 na faixa de
/// baixo), 3 se o tipo é 0x20, 1 ou 0x1e com +0x59 > 15, +0x5b > 10 e mais de 210 dias no clube, senão 2; 7 = tipo, 8 =
/// quarto, 9 = listado para transferência (0x544520).
unsafe extern "thiscall" fn post_player_backing_request_news(_this: usize, jogador: usize, clube: usize, tipo: u32, quarto: u32) {
    if jogador == 0 || cpo7_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if human_club(clube, 0, 1, 0, 0) == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 pedido de apoio");
    let (construir, gravar) = news_item_fns();
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbcf, 0);
    set_person_slots_optional(gravar, n, 0, jogador, true);
    gravar(n, 4, cpo7_d(clube) as u32);
    let t = tipo as u8;
    let alvo = t == 0x20 || t == 1 || t == 0x1e;
    let (a, b): (i32, i32) = if cpo7_w(cpo7_d(jogador + 0x61) + 0xb) > 0x109a {
        if cpo7_c(jogador + 0x57) > 0xf && cpo7_c(jogador + 0x5d) < 6 {
            (1, 1)
        } else if alvo && cpo7_c(jogador + 0x59) > 0xf && cpo7_c(jogador + 0x5b) > 0xa && days_since_arrival(jogador) > 0xd2 {
            (1, 3)
        } else {
            (1, 2)
        }
    } else if cpo7_c(jogador + 0x5b) < 0xa && cpo7_c(jogador + 0x59) < 6 {
        (2, 1)
    } else if alvo && cpo7_c(jogador + 0x59) > 0xf && cpo7_c(jogador + 0x5b) > 0xa && days_since_arrival(jogador) > 0xd2 {
        (2, 3)
    } else {
        (2, 2)
    };
    gravar(n, 5, a as u32);
    gravar(n, 6, b as u32);
    gravar(n, 7, t as i8 as i32 as u32);
    gravar(n, 8, quarto as i16 as i32 as u32);
    gravar(n, 9, (listado(jogador, 0) != 0) as u32);
    entregar(cpo7_d(0xb7431c), n, clube);
}

/// { 0x743a50 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::post_player_criticism_reaction_news(
/// jogador, clube, tópico, resposta)`: com jogador com ficha, clube com técnico e o contrato (0x4cf5f0), a notícia 0xbdc
/// (local): 0..3 o jogador, 4 = id do clube, 5 = a reação (com reputação acima de 4250 e da do clube, categoria
/// 0x4ca470 1 ou 2 e mais de 22 anos: 1 ou 3 se +0x59 < 10, +0x57 > 15 e +0x5d < 6; 2 ou 4 com mais de 1000 dias, +0x59
/// > 15 e sem 0x181 em +0x45 do contrato; o maior quando o contrato tem 0x2000 ou 8, +0x5b < 10, +0x5d > 10, +0x59 < 15,
/// mais de 23 anos e a resposta é 2; senão 0), 6 = resposta, 7 = confiança da diretoria abaixo de 4000, 8 = tópico,
/// 9..0xc o técnico. Vai aos técnicos dos clubes que a 0x8d1da0 ([0xde1f54], 0x21a, até 30) devolve.
unsafe extern "thiscall" fn post_player_criticism_reaction_news(_this: usize, jogador: usize, clube: usize, topico: u32, resposta: u32) {
    if jogador == 0 || cpo7_d(jogador + 0x61) == 0 || clube == 0 || cpo7_d(clube + 0xcf) == 0 {
        return;
    }
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = contract_with_club(cpo7_d(0xae1050), jogador, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 reacao a critica");
    let (construir, gravar) = news_item_fns();
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    let clubes_interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdc, 0);
    let f45 = cpo7_d(contrato + 0x45);
    let marca = (f45 & 0x2000 != 0 || f45 & 8 != 0)
        && cpo7_c(jogador + 0x5b) < 0xa
        && cpo7_c(jogador + 0x5d) > 0xa
        && cpo7_c(jogador + 0x59) < 0xf
        && cpo7_c(jogador + 0x18) > 0x17;
    set_person_slots_optional(gravar, n, 0, jogador, true);
    gravar(n, 4, cpo7_d(clube) as u32);
    let r = resposta as u8;
    let rep = cpo7_w(cpo7_d(jogador + 0x61) + 0xb);
    let mut reacao = 0i8;
    if rep > 0x109a && rep > cpo7_w(clube + 0x80) {
        let gerente = cpo7_d(0xae1050);
        let cat_ok = categoria(gerente, jogador, contrato) == 1 || categoria(gerente, jogador, contrato) == 2;
        if cat_ok && cpo7_c(jogador + 0x18) > 0x16 {
            if cpo7_c(jogador + 0x59) < 0xa && cpo7_c(jogador + 0x57) > 0xf && cpo7_c(jogador + 0x5d) < 6 {
                reacao = if marca && r == 2 { 3 } else { 1 };
            } else if days_since_arrival(jogador) > 0x3e8 && cpo7_c(jogador + 0x59) > 0xf && cpo7_d(contrato + 0x45) & 0x181 == 0 {
                reacao = if marca && r == 2 { 4 } else { 2 };
            }
        }
    }
    gravar(n, 5, reacao as i32 as u32);
    gravar(n, 6, r as i8 as i32 as u32);
    let linha = cpo7_d(cpo7_d(0xb67a44)).wrapping_add((cpo7_d(clube)).wrapping_mul(0x49));
    gravar(n, 7, (cpo7_w(linha + 6) < 0xfa0) as u32); // confiança da diretoria abaixo de 4000
    gravar(n, 8, topico);
    set_person_slots_optional(gravar, n, 9, cpo7_d(clube + 0xcf), false);
    let mut clubes = [0usize; 0x1e];
    let tm = cpo7_d(0xde1f54);
    let k = if tm != 0 { clubes_interessados(tm, jogador, 0x21a, clubes.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for i in 0..k.max(0) as usize {
        entregar(cpo7_d(0xb7431c), n, clubes[i]);
    }
}

/// Comparador: a gratidão sorteia, sobe +0x44 da ficha e entrega a notícia: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_queue_player_gratitude_news(this: usize, jogador: usize, clube: usize, tipo: u32, variante: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I0%%)(this, jogador, clube, tipo, variante)
}

/// Comparador: a notícia vai à caixa do técnico (não pode rodar duas vezes): só a original.
unsafe extern "thiscall" fn comparar_post_player_backing_request_news(this: usize, jogador: usize, clube: usize, tipo: u32, quarto: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I1%%)(this, jogador, clube, tipo, quarto)
}

/// Comparador: a notícia vai às caixas dos técnicos (não pode rodar duas vezes): só a original.
unsafe extern "thiscall" fn comparar_post_player_criticism_reaction_news(this: usize, jogador: usize, clube: usize, topico: u32, resposta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%)(this, jogador, clube, topico, resposta)
}
