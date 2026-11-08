// ---------- %%LEVA%%a leva: o treino do jogador posto em dia: o progresso das semanas, o programa do jogador e o nível da categoria (training_manager.cpp) ----------

unsafe fn cqd8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqd8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqd8_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqd8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
unsafe fn training_entry_of_staff(this: usize, p: usize) -> usize {
    cqd8_d(this + 0x651d).wrapping_add((cqd8_i(cqd8_d(p + 0x61)) as isize * 6) as usize)
}

/// O contrato da pessoa (CONTRACT_MANAGER 0x4cf5a0, thiscall no gerente [0xae1050]), guardado em [0xae1080] como a
/// original faz a cada consulta.
unsafe fn training_manager_get_contract_stored(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqd8_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (+4 o id; registros de 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
unsafe fn training_manager_club_of_contract(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cqd8_d(0xae23b4).wrapping_add((cqd8_i(c + 4) as isize * 0x245) as usize)
    }
}

/// O técnico humano é um dos 16 últimos ids de pessoa ([0xae2364] − 16 para cima).
unsafe fn is_human_manager_id(tec: usize) -> bool {
    cqd8_i(tec) >= cqd8_i(0xae2364).wrapping_sub(0x10)
}

/// O programa `vaga` (0..9; fora disso a 1) do técnico humano `tec`: this + (vaga + 7 + 10 × ([0xae2364] − id − 1)) ×
/// 0x9b. Fora dos 16 humanos, a caixa "Error" (linha 0x68f) e 0.
unsafe fn cqd8_programa_do_humano(this: usize, tec: usize, vaga: i8) -> usize {
    let k = cqd8_i(0xae2364).wrapping_sub(cqd8_i(tec)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&vaga) { vaga as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// O meio byte com sinal da categoria `k` na entrada do treino: 0 o baixo de +3, 1 o alto de +3, 2 o baixo de +4, 3 o
/// alto de +4, 4 o baixo de +5.
unsafe fn cqd8_meio_byte(e: usize, k: usize) -> i8 {
    let b = *((e + 3 + k / 2) as *const u8);
    if k % 2 == 0 {
        ((b << 4) as i8) >> 4
    } else {
        (b as i8) >> 4
    }
}

/// { 0x8b1750 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::update_player_training(pessoa, pular)`,
/// pela iniciação do jogador na partida (0x6eaea0, match_man.cpp, com 0: todo jogador de toda partida), pela lesão nova
/// (0x62d250) e pelas telas. Pessoa nula (linha 0x18a), sem a ficha (0x195) ou sem programa (0x8b39f0 nulo, 0x1a3): a
/// caixa "Error"; id a partir de [0xae2364]: nada. Jogador de clube de técnico humano sem a marca 0x20 em +5 da entrada
/// do treino: a marca, a vaga do programa (o de goleiro, tipo 6, para goleiro de 15 para cima; senão o geral, tipo 1;
/// senão 1) e a carga dele (0x8b4f60) na ficha de forma (0x62c660). Semanas = semana do treino (+0x688d) − a da entrada
/// (+1); nenhuma e sem `pular`: nada. Com a marca e o clube já sem técnico humano: tira a marca, carga 50, vaga 6 ou 1 e
/// os treinos de posição param (0x8b4450 com 0). Até 30 semanas: põe a entrada em dia; cada categoria (0..4) tem o
/// peso nível × 2 + 60 + (fator do treinador − 100) / 5 (0..200) e, por semana, sorteios do progresso: sobe 1 com peso
/// acima de 100, sorteio(210 − peso) < 20, valor abaixo de (+0x58) / 3 e sorteio(|valor| × 2 + 14) < 10; senão desce 1
/// com peso abaixo de 110, sorteio(peso + 60) < 20, valor acima de −7 e de (+0x5b) / 3 − 6 e o mesmo último sorteio
/// (goleiro de 19 para cima: só com sorteio(4), ou sorteio(2) na categoria 4, nulo). Os treinos de posição (0x8b4b20,
/// itens de 8 bytes: +4 a máscara, +6 a nota, +7 a taxa) andam por semana (ou uma vez, com `pular` na máscara): taxa
/// positiva, sorteio(taxa) < 5 dá nota + 2 (até 20) e o jogador pode reclamar (contrato do clube 0x4cf5f0 sem 0x8000
/// em +0x45: pela taxa acima de 50 ou pela aptidão 0x8b4770 abaixo de 12, com +0x5b abaixo de 15, sorteio(+0x5b) < 10
/// e sem lesão 0x62c400: a marca 0x8000 (0x867700), −5 em +0x44 (−100..100) e 0x5885c0); taxa negativa, sorteio(−taxa)
/// < 5 dá nota − 2 e, abaixo de 1, o item acaba (taxa 0, nota 1). Aplica o progresso (0x8b2be0) e, com itens acabados,
/// tira-os da lista (o último no lugar) e reordena (qsort 0x945333 com 0x8b4e00).
unsafe extern "thiscall" fn update_player_training(this: usize, p: usize, pular: u32) {
    if p == 0 {
        show_error_at(0xa8ccd0, 0x18a);
        return;
    }
    if cqd8_i(p) >= cqd8_i(0xae2364) {
        return;
    }
    if cqd8_d(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x195);
        return;
    }
    let mut d = [0i8; 8]; // o progresso das cinco categorias (a original zera um dword e um byte)
    let e = training_entry_of_staff(this, p);
    let programa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b39f0usize);
    let prog = programa(this, p);
    if prog == 0 {
        show_error_at(0xa8ccd0, 0x1a3);
        return;
    }
    let gravar_carga: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62c660usize);
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let marca = (e + 5) as *mut u8;
    if *marca & 0x20 == 0 {
        let c = training_manager_get_contract_stored(p);
        if c != 0 && training_manager_club_of_contract(c) != 0 && cqd8_d(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf) != 0
            && is_human_manager_id(cqd8_d(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf)) {
            let tec = cqd8_d(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf);
            *marca |= 0x20;
            let (mut goleiro, mut geral) = (-1i8, -1i8);
            for k in 0..10i8 {
                if *((cqd8_programa_do_humano(this, tec, k) + 0x15) as *const u8) == 6 {
                    goleiro = k;
                } else if *((cqd8_programa_do_humano(this, tec, k) + 0x15) as *const u8) == 1 {
                    geral = k;
                }
            }
            let vaga = if cqd8_b(cqd8_d(p + 0x61) + 0xf) >= 0xf && goleiro != -1 {
                goleiro
            } else if geral != -1 {
                geral
            } else {
                1
            };
            *(e as *mut i8) = vaga;
            let carga: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x8b4f60usize);
            let v = carga(cqd8_programa_do_humano(this, tec, vaga));
            crate::comparison::branch("cqd_8 programa do humano");
            gravar_carga(cqd8_d(0xb63d28), p, v as u32);
        }
    }
    let mut semanas = cqd8_w(this + 0x688d).wrapping_sub(cqd8_w(e + 1));
    let pular = pular as u16;
    if semanas == 0 && pular == 0 {
        return;
    }
    if *marca & 0x20 != 0 {
        let c = training_manager_get_contract_stored(p);
        let ainda = c != 0 && training_manager_club_of_contract(c) != 0 && cqd8_d(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf) != 0
            && is_human_manager_id(cqd8_d(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf));
        if !ainda {
            crate::comparison::branch("cqd_8 saiu do clube humano");
            *marca &= 0xdf;
            gravar_carga(cqd8_d(0xb63d28), p, 0x32);
            let f = cqd8_d(p + 0x61);
            *(training_entry_of_staff(this, p) as *mut u8) = if cqd8_b(f + 0xf) >= 0xf { 6 } else { 1 };
            let mut q = 0usize;
            let n = achar(this, p, &mut q);
            let parar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b4450usize);
            let mut k = 0i8;
            if n > 0 {
                loop {
                    parar(this, q.wrapping_add((k as isize * 8) as usize), 0);
                    k = k.wrapping_add(1);
                    if k as i32 >= n {
                        break;
                    }
                }
            }
        }
    }
    if semanas > 0x1e {
        semanas = 0x1e;
    }
    core::ptr::write_unaligned((e + 1) as *mut u16, cqd8_w(this + 0x688d));
    let mut q = 0usize;
    let npos = achar(this, p, &mut q);
    if *marca & 0x10 != 0 && npos == 0 {
        *marca &= 0xef;
    }
    let nivel: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x8b4f50usize);
    let fator: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8b41d0usize);
    for k in 0..5usize {
        // ponytail: a original empurra a categoria num dword cujos bytes de cima sobram da pilha; a 0x8b4f50 e a
        // 0x8b41d0 leem o byte
        let lv = nivel(prog, k as u32) as u8 as i8;
        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x3c); // cqd8: o peso do nível
        let clube = training_manager_club_of_contract(training_manager_get_contract_stored(p));
        let fa = fator(this, clube, k as u32) as u16 as i16 as i32;
        let x = base.wrapping_add(((fa - 100) / 5) as i16);
        let peso: i16 = if x < 0 { 0 } else if x > 200 { 200 } else { x };
        let piso = ((cqd8_b(p + 0x5b) as i32 / 3) as i8).wrapping_sub(6);
        let teto = (cqd8_b(p + 0x58) as i32 / 3) as i8;
        let meio = cqd8_meio_byte(e, k);
        for _ in 0..semanas {
            let v = meio.wrapping_add(d[k]);
            if cqd8_b(cqd8_d(p + 0x61) + 0xf) > 0x12 {
                let r = if k == 4 { cqd8_sortear(2) } else { cqd8_sortear(4) };
                if r != 0 {
                    continue; // goleiro: a categoria quase não anda
                }
            }
            if peso > 0x64 {
                if cqd8_sortear(0xd2 - peso as i32) < 0x14 && v < teto { // cqd8: o sorteio da subida
                    if cqd8_sortear((v as i32).abs() * 2 + 0xe) < 10 {
                        crate::comparison::branch("cqd_8 progresso sobe");
                        d[k] = d[k].wrapping_add(1); // cqd8: o progresso sobe
                    }
                    continue;
                }
            }
            if peso >= 0x6e {
                continue;
            }
            if cqd8_sortear(peso as i32 + 0x3c) >= 0x14 || v <= -7 || v <= piso {
                continue;
            }
            if cqd8_sortear((v as i32).abs() * 2 + 0xe) < 10 {
                crate::comparison::branch("cqd_8 progresso desce");
                d[k] = d[k].wrapping_sub(1); // cqd8: o progresso desce
            }
        }
    }
    let mut voltas = semanas;
    let mut contrato_do_clube = 0usize;
    if pular != 0 || voltas != 0 {
        if pular != 0 {
            voltas = 1;
        }
        let ficha: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4cf5f0usize);
        contrato_do_clube = ficha(cqd8_d(0xae1050), p, cqd8_d(p + 0x39) as u32);
    }
    let mut acabou = false;
    if npos > 0 {
        let aptidao: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8b4770usize);
        let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
        let reclamar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
        let infeliz: unsafe extern "C" fn(usize) = core::mem::transmute(0x5885c0usize);
        let mut k = 0i8;
        let mut it = q + 7; // a taxa do item (+7); a nota em +6, a máscara em +4
        loop {
            for _ in 0..voltas {
                let taxa = cqd8_b(it);
                if (pular != 0 && cqd8_w(it - 3) == pular) || taxa > 0 {
                    if cqd8_sortear(cqd8_b(it) as i32) < 5 {
                        let n = cqd8_b(it - 1).wrapping_add(2);
                        *((it - 1) as *mut i8) = if n > 0x14 { 0x14 } else { n };
                    }
                    let c = contrato_do_clube;
                    if c != 0 && cqd8_d(c + 0x45) & 0x8000 == 0 {
                        // ponytail: a original empurra a máscara no ecx com os 16 bits de cima que sobram; a 0x8b4770
                        // lê a palavra
                        let apt = aptidao(this, p, cqd8_w(it - 3) as u32) as u8 as i8;
                        let t = cqd8_b(it);
                        let quer = (t > 0x32 && cqd8_sortear(100 - t as i32) < 10) || (apt < 0xc && cqd8_sortear(apt as i32) < 2);
                        let a = cqd8_b(p + 0x5b);
                        if quer && a < 0xf && cqd8_sortear(a as i32) < 10 && cqd8_d(0xb63d28) != 0
                            && estado(cqd8_d(0xb63d28), p) == 0 {
                            crate::comparison::branch("cqd_8 reclama do treino de posicao");
                            reclamar(c, 0x8000, 0);
                            let m = (c + 0x44) as *mut i8;
                            let v = *m as i32 - 5;
                            *m = if v > 100 { 100 } else if v < -100 { -100 } else { (*m).wrapping_sub(5) };
                            infeliz(p);
                            contrato_do_clube = 0;
                        }
                    }
                } else if taxa < 0 && cqd8_sortear((taxa as i32).abs()) < 5 {
                    let n = cqd8_b(it - 1).wrapping_sub(2);
                    *((it - 1) as *mut i8) = n;
                    if n < 1 {
                        *(it as *mut i8) = 0;
                        *((it - 1) as *mut i8) = 1;
                        acabou = true;
                    }
                }
            }
            k = k.wrapping_add(1);
            it += 8;
            if k as i32 >= npos {
                break;
            }
        }
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, *const i8) = core::mem::transmute(0x8b2be0usize);
    aplicar(this, p, d.as_ptr());
    if acabou {
        crate::comparison::branch("cqd_8 treino de posicao acabou");
        let n0 = cqd8_i(this + 0x6525);
        let mut i = 0i32;
        if n0 > 0 {
            loop {
                let base = cqd8_d(this + 0x6521);
                let n = cqd8_i(this + 0x6525);
                let item = base.wrapping_add((i as isize * 8) as usize);
                if *((item + 7) as *const u8) == 0 {
                    let ultimo = base.wrapping_add((n as isize * 8 - 8) as usize);
                    core::ptr::write_unaligned(item as *mut u32, cqd8_d(ultimo) as u32);
                    core::ptr::write_unaligned((item + 4) as *mut u32, cqd8_d(ultimo + 4) as u32);
                    core::ptr::write_unaligned((this + 0x6525) as *mut i32, cqd8_i(this + 0x6525) - 1);
                }
                i += 1;
                if i >= cqd8_i(this + 0x6525) {
                    break;
                }
            }
        }
        if n0 != cqd8_i(this + 0x6525) {
            let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
            qsort(cqd8_d(this + 0x6521), cqd8_d(this + 0x6525) as u32, 8, 0x8b4e00);
        }
    }
}

/// Comparador: lado a lado no estado do sorteador, em [0xae1080], na entrada do treino (6 bytes), na ficha (+0xf..+0x41,
/// o que a 0x8b2be0 grava) e na carga da ficha de forma (31 bytes por jogador em [[0xb63d28]], +0x1e), com os sorteios
/// da cópia fora da contagem. Só a original: os casos de erro e o jogador com treino de posição (0x8b4b20 antes de
/// rodar): ali a 0x8b4450, a marca do contrato (0x867700) e a 0x5885c0 mexem fora dessas regiões.
unsafe extern "thiscall" fn compare_update_player_training(this: usize, p: usize, pular: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    if p == 0 || cqd8_i(p) >= cqd8_i(0xae2364) || cqd8_d(p + 0x61) == 0 {
        return o(this, p, pular);
    }
    let programa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b39f0usize);
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let mut q = 0usize;
    if programa(this, p) == 0 || achar(this, p, &mut q) > 0 {
        return o(this, p, pular);
    }
    let f = cqd8_d(p + 0x61);
    let mut reg = Vec::from(crate::rng::STATE_REGIONS);
    reg.extend_from_slice(&[(0xae1080, 4), (training_entry_of_staff(this, p), 6), (f + 0xf, 0x33)]);
    let les = cqd8_d(0xb63d28);
    if les != 0 {
        reg.push((cqd8_d(les).wrapping_add((cqd8_i(p) as isize * 31 + 0x1e) as usize), 1));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p, pular),
        || crate::rng::without_counting(|| update_player_training(this, p, pular)));
    SCOREBOARD.record(igual, || format!("update_player_training(pessoa {:x}, pular {:x}) {:x?} x {:x?}", p, pular as u16, mo.0, mr.0));
}

/// { 0x8b39f0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_player_schedule(pessoa)`, pelo diário
/// das pessoas (game_daily_staff_update 0x5ce2d0), pela 0x5d6aa0 e pela update_player_training (0x8b1750). Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x6ad) e 0. No clube de técnico humano (contrato 0x4cf5a0, guardado em
/// [0xae1080]; técnico +0xcf entre os 16 últimos ids): lesionado (0x62c400 = 2) o programa 1 do humano, this + 0x4d8 +
/// 0x60e × ([0xae2364] − id − 1); senão o da vaga da entrada do treino (+0, fora de 0..9 a 1); fora dos 16, a caixa
/// (linha 0x68f) e 0. Senão: lesionado, o programa 0 (this); senão this + 0x9b × vaga (com sinal).
unsafe extern "thiscall" fn get_player_schedule(this: usize, p: usize) -> usize {
    if p == 0 || cqd8_d(p + 0x61) == 0 {
        show_error_at(0xa8ccd0, 0x6ad);
        return 0;
    }
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let c = training_manager_get_contract_stored(p);
    if c != 0 && training_manager_club_of_contract(c) != 0 {
        let tec = cqd8_d(training_manager_club_of_contract(training_manager_get_contract_stored(p)) + 0xcf);
        if tec != 0 && is_human_manager_id(tec) {
            if estado(cqd8_d(0xb63d28), p) == 2 {
                let k = cqd8_i(0xae2364).wrapping_sub(cqd8_i(tec)).wrapping_sub(1);
                if !(0..0x10).contains(&k) {
                    show_error_at(0xa8ccd0, 0x68f);
                    return 0;
                }
                return this.wrapping_add(0x4d8 + 0x60e * k as usize);
            }
            return cqd8_programa_do_humano(this, tec, *(training_entry_of_staff(this, p) as *const i8));
        }
    }
    if estado(cqd8_d(0xb63d28), p) == 2 {
        return this;
    }
    this.wrapping_add((*(training_entry_of_staff(this, p) as *const i8) as isize * 0x9b) as usize)
}

/// Comparador: a devolução e [0xae1080] lado a lado; sem pessoa ou sem ficha (a caixa), só a original.
unsafe extern "thiscall" fn compare_get_player_schedule(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if p == 0 || cqd8_d(p + 0x61) == 0 {
        return o(this, p);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(this, p), || get_player_schedule(this, p));
    SCOREBOARD.record(igual && r == n, || format!("get_player_schedule(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo.0, mr.0));
    r
}

/// { 0x8b4f50 } (thiscall, ret 4, training_manager.cpp): `TRAINING_SCHEDULE::get_category_level(categoria)`: o nível da
/// categoria (+4 + categoria, com sinal) do programa, no al; os bytes de cima do eax ficam com a categoria estendida
/// (movsx), como na original. Pela update_player_training (0x8b1750, as cinco categorias de cada atualização).
unsafe extern "thiscall" fn get_category_level(this: usize, categoria: u32) -> u32 {
    let k = categoria as u8 as i8 as i32;
    let v = *((this as isize + k as isize + 4) as *const u8);
    (k as u32 & 0xffff_ff00) | v as u32
}

/// Comparador: a devolução inteira (só lê).
unsafe extern "thiscall" fn compare_get_category_level(this: usize, categoria: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%)(this, categoria);
    let n = get_category_level(this, categoria);
    SCOREBOARD.record(r == n, || format!("get_category_level({:x}, {}) original={:x} rust={:x}", this, categoria as u8 as i8, r, n));
    r
}
