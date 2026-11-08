// ---------- %%LEVA%%a leva: o físico depois da partida: o ritmo de recuperação de cada jogador e as batidas que viram lesão, a batida do jogador na partida e o jogador de férias (injury.cpp) ----------

unsafe fn cpv8_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpv8_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// A pessoa da vaga de 0x49 bytes da partida (id em −0x30): sem id (−1) ou com id fora de [0xae2364], nenhuma. A
/// original confere o ponteiro nulo antes do id (ponteiro nulo passa).
unsafe fn staff_of_match_slot(vaga: usize) -> Option<(i32, usize)> {
    let id = cpv8_d(vaga - 0x30);
    if id == -1 {
        return None;
    }
    let pessoa = (cpv8_d(0xae23bc) as u32).wrapping_add((id as u32).wrapping_mul(0x6e)) as usize;
    if pessoa != 0 && cpv8_d(pessoa) >= cpv8_d(0xae2364) {
        return None;
    }
    Some((id, pessoa))
}

/// O nível da vaga para o ritmo: +4 se não for negativo, senão +0, senão o padrão da partida (+0x4d).
unsafe fn slot_rhythm_level(partida: usize, vaga: usize) -> i8 {
    let b = cpv8_b(vaga + 4);
    if b >= 0 {
        return b;
    }
    let b = cpv8_b(vaga);
    if b >= 0 {
        b
    } else {
        cpv8_b(partida + 0x4d)
    }
}

/// { 0x62c080 } (thiscall, ret 8, injury.cpp): `Injury::apply_match_effects(jogo, partida)`, pela close_day_match
/// (0x6b5f00) e pelas estatísticas simuladas (0x84be20), em toda partida: para os dois lados da partida (0x625 bytes a
/// partir de +0x1c0), as 20 vagas de 0x49 bytes com pessoa (id em −0x30). O ritmo: o nível (+0x5a nas onze primeiras
/// com a marca +0x4c da partida; senão o da vaga, menos o +2 nas reservas, que com 0xff dá 0) mais (10000 − condição
/// +6) / 100, em 32 bits (o nível entra só nos 16 de baixo). Jogo sem a marca +0x40: a metade (16 bits). Com ela: acima
/// de 33 + sorteio(3) anos (+0x18), × 1,25 (0x968928, _ftol); senão abaixo de 23 − sorteio(5), o dobro. Antes do começo
/// do jogo ([0xdd6ec8] depois de hoje, 0x456740): 0. A recuperação (0x62bd10) com a condição e o ritmo. Com a batida
/// (−0x11) e a lista das batidas da partida (+0x28): cada batida da pessoa (o tipo 7 só com a ficha +0xf de 15 ou
/// mais) tem a gravidade por +5 (acima de 95: 2; de 66: 1; de 51: 0; senão a da batida anterior) e o tipo da lesão
/// (0x62d140); tipo válido: fim da lesão atual (0x62e700) e a lesão nova (0x62d250 com o jogo, ou 0 no jogo −2).
unsafe extern "thiscall" fn apply_match_effects(this: usize, jogo: usize, partida: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let recuperar: unsafe extern "thiscall" fn(usize, usize, i16, i16) = core::mem::transmute(0x62bd10usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tipo_de_lesao: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8 = core::mem::transmute(0x62d140usize);
    let end_player_injury: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62e700usize);
    let nova_lesao: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, i32, u32, u32) = core::mem::transmute(0x62d250usize);
    // ponytail: a gravidade é uma local da original que passa de uma batida para a outra; sem batida acima de 51 antes,
    // a original lê sobra de pilha e a cópia usa 0 (o comparador deixa toda partida com batida para a original).
    let mut gravidade: i8 = 0;
    for lado in 0..2usize {
        let base = partida + 0x1c0 + 0x625 * lado;
        for k in 0..0x14i8 {
            let vaga = base + 0x49 * k as usize;
            let Some((id, pessoa)) = staff_of_match_slot(vaga) else { continue };
            let marca = cpv8_b(partida + 0x4c) != 0;
            let nivel: i8 = if k < 0xb {
                if marca { 0x5a } else { slot_rhythm_level(partida, vaga) }
            } else if *((vaga + 2) as *const u8) == 0xff {
                0
            } else {
                slot_rhythm_level(partida, vaga).wrapping_sub(cpv8_b(vaga + 2))
            };
            let c = 0x2710i32.wrapping_sub(core::ptr::read_unaligned((vaga + 6) as *const i16) as i32);
            // ponytail: a original soma o ecx inteiro: o nível nos 16 de baixo, os 16 de cima de 10000 − condição
            let mut v = (c / 100).wrapping_add(((c as u32 & 0xffff_0000) | nivel as i16 as u16 as u32) as i32); // cpv8: o ritmo
            crate::comparacao::ramo("cpv_8 ritmo");
            if cpv8_b(jogo + 0x40) == 0 {
                crate::comparacao::ramo("cpv_8 metade");
                v = (v as i16 as i32) / 2; // cpv8: sem a marca do jogo, a metade
            } else if cpv8_b(pessoa + 0x18) as i32 > sorteio(3) + 0x21 {
                v = ((v as i16 as i32) as f64 * read_f64_at(0x968928)) as i32;
            } else if (cpv8_b(pessoa + 0x18) as i32) < 0x17 - sorteio(5) {
                crate::comparacao::ramo("cpv_8 jovem");
                v = v.wrapping_add(v); // cpv8: o jovem em dobro
            }
            if depois(0xdd6ec8, cpv8_d(0xae2c90) as u32, cpv8_d(0xae2c94) as u32) != 0 {
                v = 0;
            }
            recuperar(this, pessoa, core::ptr::read_unaligned((vaga + 6) as *const i16), v as i16);
            if *((vaga - 0x11) as *const u8) == 0 || cpv8_d(this + 0x28) == 0 {
                continue;
            }
            let lista = cpv8_d(this + 0x28) as usize;
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
            let mut e = proximo(cpv8_d(this + 0x28) as usize);
            while e != 0 {
                let tipo = cpv8_b(e + 4);
                if cpv8_d(e) == id && (tipo != 7 || cpv8_b(cpv8_d(pessoa + 0x61) as usize + 0xf) >= 0xf) {
                    let g = cpv8_b(e + 5);
                    if g > 0x5f {
                        gravidade = 2;
                    } else if g > 0x41 {
                        gravidade = 1;
                    } else if g > 0x32 {
                        gravidade = 0;
                    }
                    let t = tipo_de_lesao(this, tipo, gravidade, pessoa);
                    if (t as i8) >= 0 {
                        crate::comparacao::ramo("cpv_8 batida vira lesao");
                        end_player_injury(this, pessoa, 0);
                        let j = if cpv8_d(jogo) == -2 { 0 } else { jogo };
                        nova_lesao(this, t as u32, 0, pessoa, j, 1, -1, 0, 0);
                    }
                }
                e = proximo(cpv8_d(this + 0x28) as usize);
            }
        }
    }
}

/// A pessoa tem batida na lista da partida que chega à lesão (a mesma conferência da 0x62c080)? O cursor (+0xa) volta
/// ao que era.
unsafe fn has_injury_hit(this: usize, id: i32, pessoa: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let lista = cpv8_d(this + 0x28) as usize;
    let cursor = core::ptr::read_unaligned((lista + 0xa) as *const u32);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut achou = false;
    let mut e = proximo(lista);
    while e != 0 && !achou {
        achou = cpv8_d(e) == id && (cpv8_b(e + 4) != 7 || cpv8_b(cpv8_d(pessoa + 0x61) as usize + 0xf) >= 0xf);
        e = proximo(lista);
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor);
    achou
}

/// Comparador: lado a lado no estado do sorteio, no cursor da lista das batidas e na recuperação (+0xa, 4 bytes, do
/// registro de 31 bytes em [this]) de cada pessoa das vagas. Partida com batida que chega ao tipo da lesão: só a original (a lesão nova aloca e manda a
/// notícia; a gravidade da primeira batida pode ser sobra de pilha).
unsafe extern "thiscall" fn comparar_apply_match_effects(this: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let mut batida = false;
    for lado in 0..2usize {
        for k in 0..0x14usize {
            let vaga = partida + 0x1c0 + 0x625 * lado + 0x49 * k;
            let Some((id, pessoa)) = staff_of_match_slot(vaga) else { continue };
            if pessoa == 0 {
                return o(this, jogo, partida);
            }
            reg.push(((cpv8_d(this) as u32).wrapping_add((id as u32).wrapping_mul(31)).wrapping_add(0xa) as usize, 4));
            if *((vaga - 0x11) as *const u8) != 0 && cpv8_d(this + 0x28) != 0 && has_injury_hit(this, id, pessoa) {
                batida = true;
            }
        }
    }
    if batida {
        crate::comparacao::ramo("apply_match_effects com batida");
        return o(this, jogo, partida);
    }
    if cpv8_d(this + 0x28) != 0 {
        reg.push((cpv8_d(this + 0x28) as usize + 0xa, 4)); // o cursor da lista das batidas
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, partida), || crate::aleatorio::sem_contar(|| apply_match_effects(this, jogo, partida)));
    PLACAR.registrar(igual, || format!("apply_match_effects({:x}, {:x}) {:x?} x {:x?}", jogo, partida, mo, mr));
}

/// { 0x62bf60 } (thiscall, ret 4, injury.cpp): `Injury::find_match_injury(pessoa)`: o item da lista das batidas da
/// partida (+0x28; cursor +0xa recomeça de +2, 0x68d1b0) com o id da pessoa, ou 0 (sem lista também). Pelas telas do
/// pessoal (0x870650, 0x885820).
unsafe extern "thiscall" fn find_match_injury(this: usize, pessoa: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let lista = cpv8_d(this + 0x28) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut e = proximo(cpv8_d(this + 0x28) as usize);
    while e != 0 {
        if cpv8_d(e) == cpv8_d(pessoa) {
            return e;
        }
        e = proximo(cpv8_d(this + 0x28) as usize);
    }
    0
}

/// Comparador: lado a lado no cursor da lista e na devolução.
unsafe extern "thiscall" fn comparar_find_match_injury(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let lista = cpv8_d(this + 0x28) as usize;
    let reg: &[(usize, usize)] = if lista == 0 { &[] } else { &[(lista + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, pessoa), || find_match_injury(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("find_match_injury({:x}) original={:x} {:x?} rust={:x} {:x?}", pessoa, r, mo, n, mr));
    r
}

/// { 0x631550 } (ret 4, injury.cpp; `this` não é lido): `Injury::is_player_on_holiday(pessoa)`: 1 se a lesão atual
/// da pessoa (0x62ed70 no [0xb63d28]) é de férias (tipo 0x51). Pela tela do clube (0x4849f0).
unsafe extern "thiscall" fn is_player_on_holiday(_this: usize, pessoa: usize) -> u8 {
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let e = atual(cpv8_d(0xb63d28) as usize, pessoa);
    (e != 0 && *(e as *const u8) == 0x51) as u8
}

/// Comparador: a devolução (só o al).
unsafe extern "thiscall" fn comparar_is_player_on_holiday(this: usize, pessoa: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%);
    let r = o(this, pessoa);
    let n = is_player_on_holiday(this, pessoa);
    PLACAR.registrar(r == n, || format!("is_player_on_holiday({:x}) original={} rust={}", pessoa, r, n));
    r
}
