// ---------- %%LEVA%%a leva: o nome da posição, a descrição do jogador e o melhor candidato a favorito (Database.cpp) ----------

unsafe fn dbs2_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbs2_i8(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn dbs2_i16(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

/// { 0x5461b0 } (cdecl, Database.cpp): `write_position_name(mascara, texto, tamanho)`: o nome da posição (0x66f4e0;
/// cada um escreve por cima do anterior) contando as posições: 1 goleiro, 2 líbero, 4 zagueiro central (com 0x200 sem
/// 0x80/0x800), defensor (com 0x200) ou lateral, 8 lateral ofensivo (sem 4 e 0x200) ou volante (com 0x200), 0x10 meia,
/// 0x20 sem nenhuma antes (conta 1) meia-atacante (0x200), ponta (0x80 e 0x800), ponta esquerda (0x80) ou direita, 0x40
/// com 0x200 atacante; mais de 3: "utility player"; nenhuma: "player". Devolve o byte do gênero do último pelo idioma
/// (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4; os outros não mudam (começa em 0). Tamanho < 30: a original (caixa).
/// Na mídia, nos contratos, na busca de jogadores, nos olheiros e nas transferências (0x8dc370).
unsafe extern "C" fn write_position_name(mascara: u32, texto: usize, tamanho: i32) -> u8 {
    if tamanho < 0x1e {
        return original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u8>(%%I0%%)(mascara, texto, tamanho);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let i = idioma();
    let m = mascara;
    let mut g = 0u8;
    // o texto e o gênero com o idioma 2 e com o 4 (o 5 dá sempre 1)
    let mut escrever = |s: usize, a: u8, c: u8| {
        formatar(texto, s);
        match i {
            2 => g = a,
            5 => g = 1,
            4 => g = c,
            _ => {}
        }
    };
    let mut n = 0i8;
    if m & 1 != 0 {
        n = 1;
        escrever(0x9b7d5c, 2, 1); // goalkeeper
    }
    if m & 2 != 0 {
        n += 1;
        escrever(0x9b7d54, 2, 1); // sweeper
    }
    if m & 4 != 0 {
        n += 1;
        if m & 0x200 != 0 {
            if m & 0x80 == 0 && m & 0x800 == 0 {
                escrever(0x9b7d40, 2, 1); // central defender
            } else {
                escrever(0x9b7d28, 2, 1); // defender
            }
        } else {
            escrever(0x9b7d34, 5, 1); // full back
        }
    }
    if m & 8 != 0 {
        n += 1;
        if m & 4 == 0 && m & 0x200 == 0 {
            escrever(0x9b7d14, 5, 1); // attacking full back
        } else if m & 0x200 != 0 {
            escrever(0x9b7cfc, 2, 1); // defensive midfielder
        }
    }
    if m & 0x10 != 0 {
        n += 1;
        escrever(0x9b7cf0, 2, 1); // midfielder
    }
    if m & 0x20 != 0 && n == 0 {
        n = 1;
        if m & 0x200 != 0 {
            escrever(0x9b7cd8, 2, 1); // attacking midfielder
        } else if m & 0x80 != 0 {
            escrever(if m & 0x800 != 0 { 0x9b7cd0 } else { 0x9b7cc4 }, 5, 4); // winger, left winger
        } else {
            escrever(0x9b7cb4, 5, 4); // right winger
        }
    }
    if m & 0x40 != 0 && m & 0x200 != 0 {
        n += 1;
        escrever(0x9b7cac, 5, 4); // forward
    }
    if n > 3 {
        escrever(0x9b7c9c, 2, 1); // utility player
    }
    if n == 0 {
        escrever(0x9b7c94, 2, 1); // player
    }
    g
}

unsafe extern "C" fn comparar_write_position_name(mascara: u32, texto: usize, tamanho: i32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u8>(%%I0%%);
    if tamanho < 0x1e {
        return o(mascara, texto, tamanho); // caixa: só a original
    }
    let n = (tamanho as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(texto, n)], || o(mascara, texto, tamanho),
        || write_position_name(mascara, texto, tamanho));
    PLACAR.record(igual && r == x, || format!("write_position_name({:x}) original={} rust={} {:x?} x {:x?}", mascara, r, x, mo, mr));
    r
}

/// { 0x5465a0 } (cdecl, Database.cpp): `write_player_description(pessoa, posicao, qualidades, texto, tamanho,
/// genero)`: a descrição do jogador (0x66f4e0) pelas qualidades (as da 0x546ab0) e pela máscara da posição. Atacante
/// (qualidades 0x40): matador letal, matador ou goleador veloz (pela soma índice + qualidades % 3), oportunista,
/// matador ou artilheiro (% 4), atacante dinâmico, de força, forte, criativo, empolgante, habilidoso. Meia (posição
/// 0x10): com 0x40000 "inspirational" e por cima "free-scoring" (devolve 0); armador, ladrão de bola, dinâmico,
/// esforçado, confiável, veloz. Qualidades 2 (sem 4): lateral veloz ou empolgante; 4: defensor veloz ou técnico. Senão,
/// pela idade (+0x18 da pessoa [0xae23bc] + 110 × índice): veterano acima de índice % 3 + 33 (sem a qualidade 1),
/// revelação abaixo de índice % 3 + 18 com +0x22 nulo e a soma % 3 = 0. Devolve o gênero: com o idioma 4, 1, 3 ou 4
/// conforme o texto; senão o byte `genero` (1 com o idioma 5, que a original grava também no próprio argumento; a
/// cópia não grava). Tamanho < 30: a original (caixa).
unsafe extern "C" fn write_player_description(pessoa: i32, posicao: u32, qualidades: u32, texto: usize, tamanho: i32, genero: u32) -> u8 {
    if tamanho < 0x1e {
        return original_fn::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(%%I1%%)(pessoa, posicao, qualidades, texto, tamanho, genero);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let i = idioma();
    let g = if i == 5 { 1 } else { genero as u8 };
    let q = qualidades;
    let tem = |m: u32| q & m == m;
    // o texto e o gênero com o idioma 4
    let escrever = |s: usize, com_4: u8| {
        formatar(texto, s);
        if i == 4 { com_4 } else { g }
    };
    let soma = pessoa.wrapping_add(q as i32);
    if q & 0x40 != 0 {
        if tem(0x2c0000) {
            return escrever(0x9b7f20, 1); // deadly attacker
        }
        if tem(0x240000) {
            return escrever(if soma % 3 == 1 { 0x9b7f10 } else { 0x9b7efc }, 1); // pacey hitman, pacey goalscorer
        }
        if tem(0xc0000) {
            return escrever(0x9b7ef0, 4); // goalpoacher
        }
        if tem(0x40000) {
            if soma % 4 == 1 {
                return escrever(0x9b7ee8, 1); // hitman
            }
            return escrever(0x9b7ed4, 3); // prolific striker
        }
        for (m, s) in [(0x102000, 0x9b7ec4), (0x200800, 0x9b7eb0), (0x800, 0x9b7e9c), (0x1200, 0x9b7e88), (0x10000, 0x9b7e74)] {
            if tem(m) {
                return escrever(s, 4); // dynamic, powerhouse, powerful, creative, exciting forward
            }
        }
        if tem(0x1000) {
            return escrever(0x9b7e60, 1); // skilful attacker
        }
        return g;
    }
    if posicao & 0x10 != 0 {
        if tem(0x40000) {
            if tem(0x800200) {
                formatar(texto, 0x9b7e44); // inspirational midfielder
            }
            formatar(texto, 0x9b7e2c); // free-scoring midfielder
            return 0;
        }
        if q & 0x200 != 0 && posicao & 0x800 == 0 && posicao & 0x80 == 0 {
            return escrever(0x9b7e20, 1); // playmaker
        }
        for (m, s) in [(0x800000, 0x9b7e14), (0x102100, 0x9b7e00), (0x2000, 0x9b7de8), (0x8000, 0x9b7dd4), (0x200000, 0x9b7dc0)] {
            if tem(m) {
                return escrever(s, 1); // ball-winner, dynamic, hard working, reliable, pacey midfielder
            }
        }
        return g;
    }
    if q & 2 != 0 && q & 4 == 0 {
        if tem(0x200000) {
            return escrever(0x9b7db0, 1); // pacey full-back
        }
        if tem(0x10000) {
            return escrever(0x9b7d9c, 1); // exciting full-back
        }
        return g;
    }
    if q & 4 != 0 {
        if tem(0x200000) {
            return escrever(0x9b7d8c, 1); // pacey defender
        }
        if tem(0x1200) {
            return escrever(0x9b7d78, 1); // cultured defender
        }
        return g;
    }
    if pessoa <= -1 {
        return g;
    }
    let p = dbs2_u32(0xae23bc).wrapping_add((pessoa as usize).wrapping_mul(110));
    let idade = dbs2_i8(p + 0x18);
    if q & 1 == 0 && idade > pessoa % 3 + 0x21 {
        return escrever(0x9b7d70, 4); // veteran
    }
    if soma % 3 != 0 || q & 1 != 0 {
        return g;
    }
    if idade < pessoa % 3 + 0x12 && *((p + 0x22) as *const u8) == 0 {
        return escrever(0x9b7d68, 4); // rookie
    }
    g
}

unsafe extern "C" fn comparar_write_player_description(pessoa: i32, posicao: u32, qualidades: u32, texto: usize, tamanho: i32, genero: u32) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(%%I1%%);
    if tamanho < 0x1e {
        return o(pessoa, posicao, qualidades, texto, tamanho, genero); // caixa: só a original
    }
    let n = (tamanho as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(texto, n)], || o(pessoa, posicao, qualidades, texto, tamanho, genero),
        || write_player_description(pessoa, posicao, qualidades, texto, tamanho, genero));
    PLACAR.record(igual && r == x, || format!("write_player_description({}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, posicao, qualidades, r, x, mo, mr));
    r
}

/// A nota do colega `o` como favorito da `pessoa` no `clube` (os filtros e a nota da 0x548810, iguais aos da 0x548da0),
/// ou `None` se ele não entra. As chamadas pelo endereço do jogo, na ordem da original.
unsafe fn favourite_colleague_score(pessoa: usize, clube: usize, o: usize) -> Option<i32> {
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    if dbs2_u32(o + 0x61) == 0 {
        return None;
    }
    let id = dbs2_u32(pessoa) as i32;
    let oid = dbs2_u32(o) as i32;
    if oid >= dbs2_u32(0xae2364) as i32 || o == pessoa || oid.wrapping_add(id) % 3 == 0 {
        return None;
    }
    let pref = dbs2_u32(pessoa + 0x65);
    if pref != 0 && (0..3).any(|j| dbs2_u32(pref + 0x28 + 4 * j) == o) {
        return None; // rejeitado pela pessoa
    }
    let contrato = contract_of(dbs2_u32(0xae1050), o, clube);
    if contrato == 0 {
        return None;
    }
    let rep = dbs2_i16(dbs2_u32(o + 0x61) + 0xb);
    if rep <= 0x109a || rep <= dbs2_i16(clube + 0x80) - 0x2ee {
        return None;
    }
    if fora(dbs2_u32(0xde1f54), o, 0) != 0 || dbs2_i16(dbs2_u32(o + 0x61) + 5) <= 100 {
        return None;
    }
    let l0 = media(dbs2_u32(0xb74350), dbs2_u32(o + 0x61), 6, 0x11, 6.5) as f32 as f64; // fstp dword
    let m1 = media(dbs2_u32(0xb74350), dbs2_u32(o + 0x61), 6, 1, 6.5);
    let dados = dbs2_u32(o + 0x61);
    let mut n = dbs2_i16(dados + 0xb);
    if m1 > read_f32_at(0x968990) as f64 {
        let g = dbs2_u32(0xb74350);
        let razao = || {
            let d = read_f64_at(g + 0x18);
            if d == 0.0 || d.is_nan() { read_f64_at(0x968998) } else { read_f64_at(g + 0x10) / d }
        };
        // fcompp: C0 (abaixo ou sem ordem) soma
        if !(razao() + read_f64_at(0x968918) >= l0) {
            n += 500;
        } else if !(razao() + read_f64_at(0x967880) >= l0) {
            n += 250;
        }
    }
    if pref != 0 && (0..3).any(|j| dbs2_u32(pref + 0x1c + 4 * j) == o) {
        n += 1250; // já favorito
    }
    let c = dbs2_i8(dados + 0x21);
    if c == 20 {
        n += 75;
    } else if c > 15 {
        n += 50;
    }
    let a = dbs2_i8(o + 0x5b);
    if a > 15 {
        n += 250;
    } else if a < 6 {
        n -= 100;
    }
    n += id.wrapping_add(oid) % 500; // desempate pelos ids (dbs)
    match *((contrato + 0x4f) as *const u8) >> 4 {
        1 => n += 750,
        2 => n += 300,
        3 => n += 150,
        _ => {}
    }
    let nacao = dbs2_u32(pessoa + 0x1a);
    if nacao != dbs2_u32(clube + 0x53) && dbs2_u32(o + 0x1a) == nacao {
        if dbs2_u32(contrato + 0x45) & 0x20020 != 0 {
            n += 1750;
        } else if idade(0xae2c90, dbs2_u32(pessoa + 0x3e), dbs2_u32(pessoa + 0x42)) < 350 && dbs2_i8(pessoa + 0x18) < 24 {
            n += 250;
        } else {
            n += 125;
        }
    }
    Some(n)
}

/// { 0x548810 } (cdecl, Database.cpp): `staff_is_best_favoured_candidate(pessoa, clube, candidato)`: 1 se o candidato
/// passa nos filtros da escolha do companheiro favorito (os da 0x548da0 sem o da máscara da posição) e nenhum dos 50 do
/// elenco do clube (+0xd7) tem nota estritamente maior; 0 se não. Antes, os da pessoa: [0xb74350], id % 6 = r: +0x59 ≥
/// r + 6, +0x57 ≤ 15 − r, idade (0x54cdb0) ≥ id % 28 + 14. A original guarda a nota do candidato no próprio argumento
/// `pessoa` e um float no `candidato` (gravados antes de lidos); a cópia não. Só `al`. Na mídia (0x758200).
unsafe extern "C" fn staff_is_best_favoured_candidate(pessoa: usize, clube: usize, candidato: usize) -> u8 {
    if pessoa == 0 || clube == 0 || candidato == 0 || dbs2_u32(0xb74350) == 0 {
        return 0;
    }
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let id = dbs2_u32(pessoa) as i32;
    let r6 = id % 6;
    if dbs2_i8(pessoa + 0x59) < r6 + 6 || dbs2_i8(pessoa + 0x57) > 15 - r6 {
        return 0;
    }
    if idade(0xae2c90, dbs2_u32(pessoa + 0x3e), dbs2_u32(pessoa + 0x42)) < id % 28 + 14 {
        return 0;
    }
    let alvo = match favourite_colleague_score(pessoa, clube, candidato) {
        Some(n) => n,
        None => return 0,
    };
    for k in 0..50 {
        let o = dbs2_u32(clube + 0xd7 + 4 * k);
        if o == 0 {
            continue;
        }
        if let Some(n) = favourite_colleague_score(pessoa, clube, o) {
            if n > alvo {
                return 0; // há um melhor
            }
        }
    }
    1
}

unsafe extern "C" fn comparar_staff_is_best_favoured_candidate(pessoa: usize, clube: usize, candidato: usize) -> u8 {
    // as chamadas (contrato, média, 0x8cca10) vão pelo endereço do jogo: o estado do sorteio volta entre os lados e a
    // cópia não conta, se alguma delas sortear
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u8>(%%I2%%);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(pessoa, clube, candidato),
        || crate::rng::without_counting(|| staff_is_best_favoured_candidate(pessoa, clube, candidato)));
    PLACAR.record(igual && r == x, || format!("staff_is_best_favoured_candidate({:x}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, candidato, r, x, mo, mr));
    r
}
