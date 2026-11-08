// ---------- %%LEVA%%a leva: os contratos depois do jogo (os bônus, a felicidade, a moral e a evolução de quem ficou de fora), a moral da ficha e o texto dela (contract_manager.cpp) ----------

unsafe fn cqa8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqa8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Soma `delta` ao byte (felicidade +0x3d, moral +0x44 do contrato) entre −100 e 100, como a original: a soma em 32
/// bits decide o teto e o piso; dentro deles, a soma do byte.
unsafe fn add_clamped_100(a: usize, delta: i32) {
    let v = cqa8_b(a);
    let s = v as i32 + delta;
    *(a as *mut i8) = if s > 100 { 100 } else if s < -100 { -100 } else { v.wrapping_add(delta as i8) };
}

/// Os dois clubes do jogo (+0x1c mandante, +0x20 visitante): o do lado e o outro.
unsafe fn match_clubs_by_side(jogo: usize, lado: usize) -> (usize, usize) {
    if lado == 0 { (cqa8_d(jogo + 0x1c), cqa8_d(jogo + 0x20)) } else { (cqa8_d(jogo + 0x20), cqa8_d(jogo + 0x1c)) }
}

/// Com o byte +0x3a do jogo = 1, o lado fica de fora quando a reputação do outro clube (+0x80) fica abaixo do maior
/// entre a dele × [0x968928] e a dele + 1500 (x87 em dupla, como a original).
unsafe fn skip_side_by_reputation(jogo: usize, lado: usize) -> bool {
    if cqa8_b(jogo + 0x3a) != 1 {
        return false;
    }
    let (meu, outro) = match_clubs_by_side(jogo, lado);
    let r = cqa8_w(meu + 0x80) as i32;
    let x = r as f64 * read_f64_at(0x968928);
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > cqa8_w(outro + 0x80) as f64
}

/// A pessoa da vaga do resumo (id em +0 e +4 diferente de −1; registro de 0x6e bytes em [0xae23bc]), ou None quando a
/// original pula a vaga. A pessoa nula (base 0) passa, como na original.
unsafe fn staff_of_summary_slot(vaga: usize) -> Option<usize> {
    let id = cqa8_i(vaga);
    if id == -1 || cqa8_i(vaga + 4) == -1 {
        return None;
    }
    let p = cqa8_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if p != 0 && cqa8_i(p) >= cqa8_i(0xae2364) {
        return None;
    }
    Some(p)
}

/// Os três bônus do contrato pela vaga (Finance::pay_bonuses 0x599960): +0x21 × +0x10, +0x27 × +0x14 e, com a palavra
/// do resumo = 1 e +0x22, +0x27 × +0x18 (a original usa o +0x27 de novo); cada um só com o byte e o valor positivos.
/// Sem bônus no amistoso (0x5c6710), no jogo da competição [0x9cf684] e nos contratos de tipo 5 e 6.
unsafe fn match_bonuses_for_slot(jogo: usize, contrato: usize, vaga: usize, palavra: i16) -> [Option<i32>; 3] {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let comp = cqa8_d(jogo + 0x14);
    if comp != 0 && (amistoso(cqa8_d(0xae2c74), comp) != 0 || cqa8_d(jogo) == cqa8_d(0x9cf684)) {
        return [None; 3];
    }
    let tipo = *((contrato + 0x35) as *const u8) & 0x3f;
    if tipo == 6 || tipo == 5 {
        return [None; 3];
    }
    let g = *((vaga + 0x21) as *const u8) as i32;
    let a = *((vaga + 0x27) as *const u8) as i32;
    let um = (g != 0 && cqa8_i(contrato + 0x10) > 0).then(|| g.wrapping_mul(cqa8_i(contrato + 0x10)));
    let dois = (a != 0 && cqa8_i(contrato + 0x14) > 0).then(|| a.wrapping_mul(cqa8_i(contrato + 0x14)));
    let tres = (palavra == 1 && cqa8_b(vaga + 0x22) != 0 && cqa8_i(contrato + 0x18) > 0).then(|| a.wrapping_mul(cqa8_i(contrato + 0x18)));
    [um, dois, tres]
}

/// { 0x4ca6d0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::update_after_match(jogo, resumo)`, pelo
/// fechamento da partida do dia (0x6b5f00). Com +0x40 e +0x41 do jogo, o vencedor pelo placar (+0x47/+0x48; com +0x47 =
/// −1, +0x43/+0x44) e uma cópia dos 50 do elenco (+0xd7) de cada clube. Primeiro, por lado (clube B do parceiro
/// 0x540a50 e o lado pulado pela reputação ficam de fora), as 20 vagas do resumo (+0x190 + lado × 0x625, 0x49 bytes):
/// fora das seleções, o contrato com o clube (0x4cf5f0) paga os bônus (match_bonuses_for_slot); os minutos (+0x30 dos titulares,
/// 90 sem valor; 90 − +0x32 dos reservas) mudam a felicidade (+0x3d, minutos × 10 / 90, presa em 50 abaixo de 50) ou,
/// nas seleções, o +9 do registro de 79 bytes; fora das seleções e dos amistosos, os reservas e os titulares de pouco
/// jogo sorteiam (2 × sorteio(30)) contra os minutos: a felicidade cai 4, +0x3e conta e a moral cai (0x4cb250); senão
/// +0x3f conta (com 5 ou mais, fora do clube humano, a marca de elenco 1 em 0x540980) e a moral sobe (0x4cb0a0). O
/// vencedor (e o perdedor) sorteia(21 − +0x57) abaixo de 4 para +3 (−3) na moral +0x44; a pessoa sai da cópia do
/// elenco. Depois, quem ficou na cópia (com ficha, contrato, sem suspensão 0x55d270, lesão 0x62c400, seleção 0x7757d0
/// nem empréstimo) evolui (0x6d26b0 com a nota média [0xb74350] − 1), +0x3e conta e, a partir de 8 (14 com parceiro),
/// fora do humano e do parceiro, sai do principal (0x4cbb60 2 ou 4 → 0x4cbd20); a felicidade cai 8 e a moral cai.
unsafe extern "thiscall" fn update_after_match(this: usize, jogo: usize, resumo: usize) {
    if cqa8_b(jogo + 0x40) == 0 || cqa8_b(jogo + 0x41) == 0 {
        return;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let pagar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x599960usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let marcar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    // a original põe ecx = this antes da 0x4cb0a0 (stdcall na cópia Rust): thiscall com 3 na pilha serve aos dois
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x4cb0a0usize);
    let baixar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x4cb250usize);
    let (c47, c48) = (cqa8_b(jogo + 0x47), cqa8_b(jogo + 0x48));
    let (a, b) = if c47 == -1 { (cqa8_b(jogo + 0x43), cqa8_b(jogo + 0x44)) } else { (c47, c48) };
    let venceu = [(a > b) as u32, (a < b) as u32];
    let mut lista = [[0usize; 50]; 2];
    for (lado, l) in lista.iter_mut().enumerate() {
        let c = match_clubs_by_side(jogo, lado).0;
        core::ptr::copy_nonoverlapping((c + 0xd7) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let clube = match_clubs_by_side(jogo, lado).0;
        if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if skip_side_by_reputation(jogo, lado) {
            continue;
        }
        let fin = financas(cqa8_d(0xae2a38), clube);
        let base = resumo + lado * 0x625;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = staff_of_summary_slot(vaga) else { continue };
            let contrato = if selecao(clube) != 0 {
                0
            } else {
                let c = com_clube(this, pessoa, clube);
                if c == 0 {
                    continue;
                }
                for v in match_bonuses_for_slot(jogo, c, vaga, cqa8_w(base + 0x154 + 2 * k)).into_iter().flatten() {
                    pagar(fin, v);
                }
                c
            };
            let minutos: i8 = if k < 11 {
                let v = cqa8_b(vaga + 0x30);
                if v > -1 { v } else { 0x5a }
            } else {
                let v = cqa8_b(vaga + 0x32);
                if v <= -1 {
                    0
                } else {
                    let e = 0x5a - v as i32;
                    if e <= 0 { 0 } else { e as i8 }
                }
            };
            if k < 11 || cqa8_b(vaga + 0x32) != -1 {
                let q = ((minutos as i32 * 10) / 90) as i8;
                if selecao(clube) != 0 {
                    let r = cqa8_d(0xae2d0c).wrapping_add(cqa8_d(pessoa).wrapping_mul(0x4f)) + 9;
                    *(r as *mut i8) = cqa8_b(r).wrapping_add(q);
                    if cqa8_b(r) > 100 {
                        *(r as *mut i8) = 100;
                    }
                } else {
                    let al = cqa8_b(contrato + 0x3d);
                    let s = al as i32 + q as i32;
                    *((contrato + 0x3d) as *mut i8) = if al > 0x32 {
                        if s > 100 { 100 } else if s < -100 { -100 } else { al.wrapping_add(q) }
                    } else if s > 0x32 {
                        0x32
                    } else {
                        let v = al.wrapping_add(q);
                        if v > 100 { 100 } else if v < -100 { -100 } else { v }
                    };
                }
            }
            if selecao(clube) == 0 && (cqa8_i(jogo) < 0 || amistoso(cqa8_d(0xae2c74), cqa8_d(jogo + 0x14)) == 0) {
                let testa = k >= 11 || !(minutos > 0x1e || cqa8_b(vaga + 0x1f) != 0 || cqa8_b(vaga + 0x20) < 0x46);
                let mal = testa && {
                    crate::comparison::branch("cqa_8 testa a moral");
                    let m = cqa8_b(contrato + 0x44) as i32;
                    let r1 = sorteio(0x1e); // cqa8: o primeiro sorteio da moral
                    let v = r1 as f64 + (read_f64_at(0x968d68) - m as f64 * read_f64_at(0x968d70));
                    let r2 = sorteio(0x1e);
                    v - r2 as f64 > minutos as f64
                };
                let p = contrato as *mut u8;
                if mal {
                    *p.add(0x3f) = 0;
                    if *p.add(0x3e) != 100 {
                        *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
                    }
                    add_clamped_100(contrato + 0x3d, -4);
                    baixar(this, pessoa, clube, contrato);
                } else {
                    *p.add(0x3e) = 0;
                    if *p.add(0x3f) != 100 {
                        *p.add(0x3f) = (*p.add(0x3f)).wrapping_add(1);
                    }
                    if cqa8_b(contrato + 0x3f) >= 5 && humano(clube, 0, 1, 0, 0) == 0 {
                        marcar(pessoa, 1);
                    }
                    subir(this, pessoa, clube, contrato);
                }
            }
            if venceu[lado] != 0 {
                crate::comparison::branch("cqa_8 moral do vencedor");
                let r = sorteio(0x15 - cqa8_b(pessoa + 0x57) as i32); // cqa8: o sorteio do vencedor
                if r < 4 && contrato != 0 {
                    add_clamped_100(contrato + 0x44, 3);
                }
            } else if venceu[1 - lado] != 0 {
                let r = sorteio(0x15 - cqa8_b(pessoa + 0x57) as i32);
                if r < 4 && contrato != 0 {
                    add_clamped_100(contrato + 0x44, -3);
                }
            }
            if let Some(x) = lista[lado].iter_mut().find(|x| **x == pessoa) {
                *x = 0;
            }
        }
    }
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7757d0usize);
    let evoluir: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(0x6d26b0usize);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x4cbd20usize);
    for lado in 0..2usize {
        let (clube, outro) = match_clubs_by_side(jogo, lado);
        let limit: i8 = if parceiro(clube, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if skip_side_by_reputation(jogo, lado) {
            continue;
        }
        for i in 0..50usize {
            let pessoa = lista[lado][i];
            if pessoa == 0 || cqa8_i(pessoa) >= cqa8_i(0xae2364) || cqa8_d(pessoa + 0x61) == 0 || selecao(clube) != 0 {
                continue;
            }
            let contrato = com_clube(this, pessoa, clube);
            if contrato == 0 {
                continue;
            }
            if cqa8_i(jogo) != -2 && suspenso(cqa8_d(0xae28f0), pessoa, cqa8_d(jogo + 0x14), 1, 0) != 0 {
                continue;
            }
            if lesao(cqa8_d(0xb63d28), pessoa) != 0 || impedido(cqa8_d(0xb74318), pessoa, 0xae2c90, clube, 0, 0) != 0 {
                continue;
            }
            // emprestado: o segundo contrato da pessoa (+4 do registro de 79 bytes)
            let idx = cqa8_i(cqa8_d(0xae2d0c).wrapping_add(cqa8_d(pessoa).wrapping_mul(0x4f)) + 4);
            if idx >= 0 && (idx as usize).wrapping_mul(0x50).wrapping_add(cqa8_d(this)) != 0 {
                continue;
            }
            let m = cqa8_d(0xb74350);
            let x = core::ptr::read_unaligned((m + 0x18) as *const f64);
            let c = read_f64_at(0x9689a0);
            // fcomp + C3: igual ou desordenado vai para a constante
            let nota = if !(x < c || x > c) { read_f64_at(0x968998) } else { core::ptr::read_unaligned((m + 0x10) as *const f64) / x };
            crate::comparison::branch("cqa_8 evolui quem ficou de fora");
            evoluir(0, pessoa, clube, outro, jogo, (nota - read_f64_at(0x967888)) as f32, 0, 0); // cqa8: a evolução de quem ficou de fora
            let p = contrato as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if cqa8_b(contrato + 0x3e) >= limit && humano(clube, 0, 1, 0, 0) == 0 && parceiro(clube, core::ptr::null_mut(), 1) == 0 {
                let l = lugar(this, pessoa, clube);
                if l == 2 || l == 4 {
                    trocar(this, pessoa, clube, 0);
                }
            }
            add_clamped_100(contrato + 0x3d, -8);
            baixar(this, pessoa, clube, contrato);
        }
    }
}

/// Comparador: a moral (0x4cb0a0, 0x4cb250: queixas, pedido de transferência), a evolução (0x6d26b0) e a troca de
/// elenco (0x4cbd20) sorteiam e alocam (não podem rodar duas vezes): só a original roda. Confere o que a rotina garante
/// antes de qualquer sorteio: os bônus que cada clube paga (+0x138 das finanças soma o total previsto pelas mesmas
/// regras da cópia, calculado antes). A cópia é provada pelos ramos e pelas mutações na carreira.
unsafe extern "thiscall" fn compare_update_after_match(this: usize, jogo: usize, resumo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if cqa8_b(jogo + 0x40) == 0 || cqa8_b(jogo + 0x41) == 0 {
        return o(this, jogo, resumo);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let mut plano: [(usize, i32, i32); 2] = [(0, 0, 0); 2];
    let mut eh_a = 0u32;
    for (lado, pl) in plano.iter_mut().enumerate() {
        let clube = match_clubs_by_side(jogo, lado).0;
        if (parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0) || skip_side_by_reputation(jogo, lado) || selecao(clube) != 0 {
            continue;
        }
        let fin = financas(cqa8_d(0xae2a38), clube);
        let base = resumo + lado * 0x625;
        let mut total = 0i32;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = staff_of_summary_slot(vaga) else { continue };
            let c = com_clube(this, pessoa, clube);
            if c != 0 {
                for v in match_bonuses_for_slot(jogo, c, vaga, cqa8_w(base + 0x154 + 2 * k)).into_iter().flatten() {
                    total = total.wrapping_add(v);
                }
            }
        }
        if fin != 0 {
            *pl = (fin, cqa8_i(fin + 0x138), total);
        }
    }
    o(this, jogo, resumo);
    for &(fin, antes, total) in &plano {
        if fin != 0 {
            let pago = cqa8_i(fin + 0x138).wrapping_sub(antes);
            SCOREBOARD.record(pago == total, || format!("update_after_match(jogo {:x}) bônus de fin {:x}: original {} previsto {}", jogo, fin, pago, total));
        }
    }
}

/// { 0x4cc2d0 } (stdcall, ret 4, contract_manager.cpp): `contract_manager_get_player_morale(pessoa)`: o byte +0x45 da
/// ficha (+0x61), pelas telas. O eax leva os bytes altos da pessoa, como na original.
unsafe extern "stdcall" fn contract_manager_get_player_morale(pessoa: usize) -> u32 {
    (pessoa as u32 & 0xffff_ff00) | *((cqa8_d(pessoa + 0x61) + 0x45) as *const u8) as u32
}

unsafe extern "stdcall" fn compare_contract_manager_get_player_morale(pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I1%%)(pessoa);
    let n = contract_manager_get_player_morale(pessoa);
    SCOREBOARD.record(r == n, || format!("contract_manager_get_player_morale({:x}) original={:#x} rust={:#x}", pessoa, r, n));
    r
}

/// O texto terminado em zero em `a` (até 0x400 bytes), para os comparadores dos textos.
unsafe fn c_string_bytes(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// { 0x4cc2e0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_write_morale_text(pessoa, saída, _)`: a
/// palavra da moral (+0x45 da ficha: abaixo de 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", senão
/// "Superb") na saída (0x66f4e0 com 0xad9c64), pelas telas.
unsafe extern "stdcall" fn contract_manager_write_morale_text(pessoa: usize, saida: usize, _x: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let m = cqa8_b(cqa8_d(pessoa + 0x61) + 0x45);
    let f = if m < 4 {
        0x9a8320usize
    } else if m < 8 {
        0x9a82fc
    } else if m < 0xc {
        0x9a82dc
    } else if m < 0xf {
        0x9a82b8
    } else if m < 0x12 {
        0x9a8290
    } else {
        0x9a826c
    };
    formatar(saida, f, 0xad9c64usize);
}

/// Comparador: a original escreve a palavra; a cópia escreve de novo (o texto se sobrescreve) e as duas têm de bater;
/// fica o texto da original.
unsafe extern "stdcall" fn compare_contract_manager_write_morale_text(pessoa: usize, saida: usize, x: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(%%I2%%)(pessoa, saida, x);
    let r = c_string_bytes(saida);
    contract_manager_write_morale_text(pessoa, saida, x);
    let n = c_string_bytes(saida);
    core::ptr::copy_nonoverlapping(r.as_ptr(), saida as *mut u8, r.len());
    *((saida + r.len()) as *mut u8) = 0;
    SCOREBOARD.record(r == n, || format!("contract_manager_write_morale_text({:x}) original={:?} rust={:?}", pessoa, String::from_utf8_lossy(&r), String::from_utf8_lossy(&n)));
}
