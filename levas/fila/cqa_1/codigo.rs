// ---------- %%LEVA%%a leva: a rodada da copa: o prêmio da rodada, os mandantes da rodada e o progresso do time (Cup.cpp) ----------

unsafe fn cqa1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3).
unsafe fn round_record(copa: usize, r: i32) -> usize {
    cqa1_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// A entrada de 6 bytes do time `t` (+0xa7: +0 o clube, +4 o nível, +5 o desfecho).
unsafe fn team_entry(copa: usize, t: i32) -> usize {
    cqa1_d(copa + 0xa7).wrapping_add((t as isize * 6) as usize)
}

/// O confronto `k` (0x35 bytes) da rodada `r` (+0xad: um vetor por rodada).
unsafe fn round_tie(copa: usize, r: i32, k: i32) -> usize {
    cqa1_d(cqa1_d(copa + 0xad).wrapping_add((r as isize * 4) as usize)).wrapping_add((k as isize * 0x35) as usize)
}

/// O registro de finanças do clube (0x5a0590: push clube; ecx = [0xae2a38]; sem add esp -> thiscall(tabela, clube),
/// ret 4).
unsafe fn cqa1_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cqa1_d(0xae2a38), clube)
}

/// O dinheiro no saldo (0x5999a0: o valor fica na pilha desde antes da 0x5a0590; ecx = as finanças; sem add esp ->
/// thiscall(finanças, valor), ret 4).
unsafe fn cqa1_somar(fin: usize, valor: i32) {
    let f: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    f(fin, valor)
}

/// O ensaio do comparador da 0x522240: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca).
static mut ROUND_PRIZE_MONEY_DRY_RUN: bool = false;

/// A notícia do prêmio (Comp 0x48e3e0: 7 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, valor, alto do
/// valor, −1, rodada seguinte, partida, desfecho), ret 0x1c); no ensaio, nada.
unsafe fn send_prize_news(copa: usize, clube: usize, valor: i32, seguinte: u32, partida: usize, desfecho: u8) {
    if ROUND_PRIZE_MONEY_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, u32) = core::mem::transmute(0x48e3e0usize);
    // ponytail: a original empurra o dword de um local de que só grava o byte baixo (os bits altos são sobra de pilha);
    // a notícia só lê o byte (movsx ecx, byte [esp+0x114]): aqui, o byte limpo.
    f(copa, clube, valor, valor >> 31, -1, seguinte, partida, desfecho as u32);
}

/// Os prêmios da rodada `r`: (ao vencedor, ao perdedor), do registro (+0x60/+0x64) ou do fundo (−1); None quando a
/// divisão da original cairia (divisor zero ou estouro). `contar`: o contador do ramo do fundo (não no comparador).
unsafe fn round_prizes(copa: usize, r: i32, contar: bool) -> Option<(i32, i32)> {
    let e = round_record(copa, r);
    let (mut ao_vencedor, mut ao_perdedor) = (cqa1_i(e + 0x60), cqa1_i(e + 0x64));
    if ao_vencedor == -1 {
        let (fundo, resta, confrontos) = (cqa1_i(copa + 0x28), cqa1_w(copa + 0x3c) as i32 - r, cqa1_w(e + 0x1a) as i32);
        if resta == 0 || confrontos == 0 || (fundo == i32::MIN && resta == -1) {
            return None;
        }
        let q0 = fundo / resta;
        if q0 == i32::MIN && confrontos == -1 {
            return None;
        }
        if contar {
            crate::comparacao::ramo("cqa_1 premio pelo fundo");
        }
        let q = q0 / confrontos; // cqa1: o fundo pelas rodadas que faltam e pelos confrontos
        if ao_perdedor == -1 {
            ao_vencedor = q.wrapping_mul(3) / 4; // idiv por 4 com o arredondamento para zero (cdq; and 3; sar 2)
            ao_perdedor = q / 4;
        } else {
            ao_vencedor = q;
        }
    }
    Some((ao_vencedor, ao_perdedor))
}

/// { 0x522240 } (thiscall, ret 0x10, Cup.cpp): `Cup::pay_round_prize_money(vencedor, perdedor, rodada, partida)`
/// (entradas de 6 bytes dos dois times). Os prêmios da rodada (+0x60 o do vencedor, +0x64 o do perdedor); com −1 no
/// do vencedor, o fundo (+0x28) / (rodadas +0x3c − rodada) / confrontos (+0x1a), e com −1 também no do perdedor, 3/4
/// para o vencedor e 1/4 para o perdedor. O vencedor (se o prêmio não é zero) recebe a notícia (0x48e3e0: o
/// desfecho +5 do time, 3 se era 1 na partida 0x136, e a rodada seguinte: a palavra +7 dela se não é a última nem
/// 0x8c, sobre os 16 bits altos do ponteiro da rodada, como a original deixa no ebp) e o dinheiro (0x5999a0 nas
/// finanças 0x5a0590); o perdedor só o dinheiro; o fundo perde os dois.
unsafe extern "thiscall" fn pay_round_prize_money(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let r = rodada as i16 as i32;
    let (ao_vencedor, ao_perdedor) = match round_prizes(copa, r, true) {
        Some(p) => p,
        None => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(%%I0%%)(copa, vencedor, perdedor, rodada, partida), // ponytail: a original cai na divisão; que caia nela
    };
    let mut desfecho = cqa1_b(vencedor + 5);
    if cqa1_w(partida + 0x30) == 0x136 && desfecho == 1 {
        desfecho = 3;
    }
    let e = round_record(copa, r);
    let mut seguinte = 0u32;
    if r + 1 < cqa1_w(copa + 0x3c) as i32 {
        let w = core::ptr::read_unaligned((e + 0x6f) as *const u16);
        if w != 0x8c {
            seguinte = (e as u32 & 0xffff_0000) | w as u32; // mov bp, [ebp+0x6f]: só a palavra baixa do ebp muda
        }
    }
    if ao_vencedor != 0 {
        send_prize_news(copa, cqa1_d(vencedor), ao_vencedor, seguinte, partida, desfecho);
        crate::comparacao::ramo("cqa_1 premio do vencedor");
        cqa1_somar(cqa1_financas(cqa1_d(vencedor)), ao_vencedor); // cqa1: o prêmio do vencedor
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, cqa1_i(copa + 0x28).wrapping_sub(ao_vencedor));
    }
    if ao_perdedor != 0 {
        cqa1_somar(cqa1_financas(cqa1_d(perdedor)), ao_perdedor);
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, cqa1_i(copa + 0x28).wrapping_sub(ao_perdedor));
    }
}

/// Comparador da 0x522240, em ensaio: a cópia sem a notícia e a original inteira, a partir do mesmo estado (o fundo e
/// os registros de finanças dos dois clubes, restaurados depois do ensaio); confere o fundo e as finanças. Sem clube,
/// sem finanças ou com a divisão que cai: só a original.
unsafe extern "thiscall" fn comparar_pay_round_prize_money(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(%%I0%%);
    let (cv, cp) = (cqa1_d(vencedor), cqa1_d(perdedor));
    if cv == 0 || cp == 0 || cv == cp || round_prizes(copa, rodada as i16 as i32, false).is_none() {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem clube ou com a divisão que cai, só a original
    }
    let (fv, fp) = (cqa1_financas(cv), cqa1_financas(cp));
    if fv == 0 || fp == 0 {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem finanças, só a original
    }
    let regioes = [(copa + 0x28, 4), (fv, 0x167), (fp, 0x167)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    ROUND_PRIZE_MONEY_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_round_prize_money(copa, vencedor, perdedor, rodada, partida)));
    ROUND_PRIZE_MONEY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(copa, vencedor, perdedor, rodada, partida);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pay_round_prize_money(rodada {}, vencedor {:x}, perdedor {:x}) o fundo e as finanças diferem", rodada as i16, cv, cp));
}

/// Os confrontos [início, fim) que a 0x51e870 percorre: um só (`laco`; o fim é a palavra baixa de laco + 1, como o
/// `lea edx, [eax+1]` da original) ou, com −1, todos os da rodada (+0x1a).
unsafe fn ties_range(copa: usize, rodada: u32, laco: u32) -> (i16, i16) {
    if laco as u16 != 0xffff {
        (laco as i16, laco.wrapping_add(1) as i16)
    } else {
        (0, cqa1_w(round_record(copa, rodada as i16 as i32) + 0x1a))
    }
}

/// Troca mandante e visitante no confronto `t`: os pares de palavras +0/+2, +0xa/+0xc e +0xe/+0x10.
unsafe fn swap_home_away(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xa, 0xc), (0xe, 0x10)] {
        let (x, y) = (cqa1_w(t + a), cqa1_w(t + b));
        core::ptr::write_unaligned((t + a) as *mut i16, y);
        core::ptr::write_unaligned((t + b) as *mut i16, x);
    }
}

/// { 0x51e870 } (thiscall, ret 8, Cup.cpp): `Cup::set_home_teams_for_round(rodada, laco)`: num confronto (`laco`) ou
/// em todos os da rodada (−1), com os dois times definidos (≠ −1), inverte mandante e visitante pela regra da rodada
/// (byte baixo de +0xb): bit 1, se a virtual +0x90 (club_in_higher_division(clube a, clube b)) devolve o clube b;
/// senão bit 8, se o nível (+4 do time, com sinal) de a é menor que o de b; senão, com o estádio da copa (+0x24) ≠
/// −1, se o clube b existe, tem estádio (+0x69) e o id dele (+0) é o da copa.
unsafe extern "thiscall" fn set_home_teams_for_round(copa: usize, rodada: u32, laco: u32) {
    let r = rodada as i16 as i32;
    let (ini, fim) = ties_range(copa, rodada, laco);
    let regra = cqa1_b(round_record(copa, r) + 0xb);
    if regra & 1 == 0 && regra & 8 == 0 && cqa1_i(copa + 0x24) == -1 {
        return;
    }
    let mut k = ini as i32;
    while k < fim as i32 {
        let t = round_tie(copa, r, k);
        let (a, b) = (cqa1_w(t), cqa1_w(t + 2));
        k += 1;
        if a == -1 || b == -1 {
            continue;
        }
        let (ta, tb) = (team_entry(copa, a as i32), team_entry(copa, b as i32));
        let inverter = if regra & 1 != 0 {
            let (ca, cb) = (cqa1_d(ta), cqa1_d(tb));
            // a virtual +0x90: push cb; push ca; ecx = a copa; sem add esp -> thiscall(copa, ca, cb), ret 8
            let maior: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(cqa1_d(cqa1_d(copa) + 0x90));
            let m = maior(copa, ca, cb);
            m != 0 && m == cb
        } else if regra & 8 != 0 {
            (cqa1_b(ta + 4) as i8) < (cqa1_b(tb + 4) as i8)
        } else {
            let cb = cqa1_d(tb);
            cb != 0 && cqa1_d(cb + 0x69) != 0 && cqa1_i(cqa1_d(cb + 0x69)) == cqa1_i(copa + 0x24) // o estádio da copa
        };
        if inverter {
            swap_home_away(t);
        }
    }
}

/// Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
unsafe extern "thiscall" fn comparar_set_home_teams_for_round(copa: usize, rodada: u32, laco: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I1%%);
    let (ini, fim) = ties_range(copa, rodada, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((round_tie(copa, rodada as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada, laco), || set_home_teams_for_round(copa, rodada, laco));
    PLACAR.registrar(igual, || format!("set_home_teams_for_round(rodada {}, laco {}) original={:x?} rust={:x?}", rodada as i16, laco as i16, mo, mr));
}

/// Os argumentos da virtual +0x68 que a 0x521b30 chama para o time `t` na rodada `r`: (p1, p2, p3) ou None.
unsafe fn team_progress_args(copa: usize, t: i32, r: i32) -> Option<(u8, u8, u8)> {
    let n = cqa1_w(copa + 0x3c) as i32;
    let d = cqa1_b(team_entry(copa, t) + 5); // o desfecho
    let byte_da_rodada = |k: i32, desl: usize| cqa1_b(round_record(copa, k) + desl);
    if n > 0 && cqa1_w(round_record(copa, n - 1) + 7) == 0x8c {
        // a última rodada é a do terceiro lugar
        if r == n - 2 {
            return match d {
                1 => Some((1, 1, 1)),
                2 => Some((2, 2, 2)),
                _ => None,
            };
        }
        if r == n - 1 {
            return match d {
                3 => Some((3, 3, 3)),
                4 => Some((4, 4, 4)),
                _ => None,
            };
        }
        if d != 0 {
            let v = byte_da_rodada(r, 0x5b);
            return Some((v, v, v));
        }
        if r + 1 == n - 2 {
            return Some((4, 1, 4));
        }
        let v = byte_da_rodada(r + 1, 0x5b);
        return Some((v, 1, v));
    }
    if r == n - 1 {
        return match d {
            1 => Some((1, 1, 1)),
            2 => {
                let v = byte_da_rodada(r, 0x1a).wrapping_add(1);
                Some((v, v, v))
            }
            _ => None,
        };
    }
    if d != 0 {
        let v = byte_da_rodada(r, 0x5b);
        return Some((v, v, v));
    }
    if r + 1 == n - 1 {
        let v = byte_da_rodada(r + 1, 0x1a).wrapping_add(1);
        return Some((v, 1, v));
    }
    let v = byte_da_rodada(r + 1, 0x5b);
    Some((v, 1, v))
}

/// { 0x521b30 } (thiscall, ret 8, Cup.cpp): `Cup::record_team_progress(time, rodada)`: quanto o time foi na copa,
/// pela virtual +0x68 (`set_club_progress(clube, competição +0x44, p1, p2, p3)`). Com a última rodada do tipo 0x8c
/// (terceiro lugar): na semifinal (rodadas − 2), desfecho 1 ou 2 → (d, d, d); na última, 3 ou 4 → (d, d, d); noutra
/// rodada, desfecho ≠ 0 → o +0x5b da rodada três vezes; 0 → (4, 1, 4) antes da semifinal, senão (x, 1, x) com o +0x5b
/// da rodada seguinte. Sem ela: na final, 1 → (1, 1, 1), 2 → os confrontos (+0x1a) da final + 1 três vezes; noutra,
/// desfecho ≠ 0 → o +0x5b três vezes; 0 → (x, 1, x) com os confrontos + 1 da final antes dela, senão o +0x5b da
/// seguinte. Os desfechos fora desses não chamam nada.
unsafe extern "thiscall" fn record_team_progress(copa: usize, time: u32, rodada: u32) {
    let (t, r) = (time as i16 as i32, rodada as i16 as i32);
    if let Some((p1, p2, p3)) = team_progress_args(copa, t, r) {
        // a virtual +0x68: 5 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, comp, p1, p2, p3), ret 0x14
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(cqa1_d(cqa1_d(copa) + 0x68));
        // ponytail: a original empurra bytes sobre registradores inteiros (o ebx de quem chamou, o dword do argumento
        // `time` com o byte baixo trocado, 13 × rodada): os bits altos são sobra; as 70 versões da +0x68 só leem o byte
        // baixo de cada um (ou repassam os dwords a outra que lê): aqui, os bytes limpos.
        f(copa, cqa1_d(team_entry(copa, t)), cqa1_b(copa + 0x44) as u32, p1 as u32, p2 as u32, p3 as u32);
    }
}

/// Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
unsafe extern "thiscall" fn comparar_record_team_progress(copa: usize, time: u32, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I2%%);
    let stats = cqa1_d(copa + 8);
    let clube = cqa1_d(team_entry(copa, time as i16 as i32));
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let reg = if stats != 0 && clube != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(copa, time, rodada); // ponytail: sem registro para olhar, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(copa, time, rodada), || record_team_progress(copa, time, rodada));
    PLACAR.registrar(igual, || format!("record_team_progress(time {}, rodada {}) original={:x?} rust={:x?}", time as i16, rodada as i16, mo, mr));
}
