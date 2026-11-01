// ---------- 263a leva: a rodada da copa: o prêmio da rodada, os mandantes da rodada e o progresso do time (Cup.cpp) ----------

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
unsafe fn cqa1_rodada(copa: usize, r: i32) -> usize {
    cqa1_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// A entrada de 6 bytes do time `t` (+0xa7: +0 o clube, +4 o nível, +5 o desfecho).
unsafe fn cqa1_time(copa: usize, t: i32) -> usize {
    cqa1_d(copa + 0xa7).wrapping_add((t as isize * 6) as usize)
}

/// O confronto `k` (0x35 bytes) da rodada `r` (+0xad: um vetor por rodada).
unsafe fn cqa1_confronto(copa: usize, r: i32, k: i32) -> usize {
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
static mut CQA1_ENSAIO: bool = false;

/// A notícia do prêmio (Comp 0x48e3e0: 7 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, valor, alto do
/// valor, −1, rodada seguinte, partida, desfecho), ret 0x1c); no ensaio, nada.
unsafe fn cqa1_noticia(copa: usize, clube: usize, valor: i32, seguinte: u32, partida: usize, desfecho: u8) {
    if CQA1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, u32) = core::mem::transmute(0x48e3e0usize);
    // ponytail: a original empurra o dword de um local de que só grava o byte baixo (os bits altos são sobra de pilha);
    // a notícia só lê o byte (movsx ecx, byte [esp+0x114]): aqui, o byte limpo.
    f(copa, clube, valor, valor >> 31, -1, seguinte, partida, desfecho as u32);
}

/// Os prêmios da rodada `r`: (ao vencedor, ao perdedor), do registro (+0x60/+0x64) ou do fundo (−1); None quando a
/// divisão da original cairia (divisor zero ou estouro). `contar`: o contador do ramo do fundo (não no comparador).
unsafe fn cqa1_premios(copa: usize, r: i32, contar: bool) -> Option<(i32, i32)> {
    let e = cqa1_rodada(copa, r);
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
unsafe extern "thiscall" fn copa_premio_da_rodada(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let r = rodada as i16 as i32;
    let (ao_vencedor, ao_perdedor) = match cqa1_premios(copa, r, true) {
        Some(p) => p,
        None => return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(1419)(copa, vencedor, perdedor, rodada, partida), // ponytail: a original cai na divisão; que caia nela
    };
    let mut desfecho = cqa1_b(vencedor + 5);
    if cqa1_w(partida + 0x30) == 0x136 && desfecho == 1 {
        desfecho = 3;
    }
    let e = cqa1_rodada(copa, r);
    let mut seguinte = 0u32;
    if r + 1 < cqa1_w(copa + 0x3c) as i32 {
        let w = core::ptr::read_unaligned((e + 0x6f) as *const u16);
        if w != 0x8c {
            seguinte = (e as u32 & 0xffff_0000) | w as u32; // mov bp, [ebp+0x6f]: só a palavra baixa do ebp muda
        }
    }
    if ao_vencedor != 0 {
        cqa1_noticia(copa, cqa1_d(vencedor), ao_vencedor, seguinte, partida, desfecho);
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
unsafe extern "thiscall" fn comparar_copa_premio_da_rodada(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(1419);
    let (cv, cp) = (cqa1_d(vencedor), cqa1_d(perdedor));
    if cv == 0 || cp == 0 || cv == cp || cqa1_premios(copa, rodada as i16 as i32, false).is_none() {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem clube ou com a divisão que cai, só a original
    }
    let (fv, fp) = (cqa1_financas(cv), cqa1_financas(cp));
    if fv == 0 || fp == 0 {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem finanças, só a original
    }
    let regioes = [(copa + 0x28, 4), (fv, 0x167), (fp, 0x167)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQA1_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| copa_premio_da_rodada(copa, vencedor, perdedor, rodada, partida)));
    CQA1_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(copa, vencedor, perdedor, rodada, partida);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("copa_premio_da_rodada(rodada {}, vencedor {:x}, perdedor {:x}) o fundo e as finanças diferem", rodada as i16, cv, cp));
}

/// Os confrontos [início, fim) que a 0x51e870 percorre: um só (`laco`; o fim é a palavra baixa de laco + 1, como o
/// `lea edx, [eax+1]` da original) ou, com −1, todos os da rodada (+0x1a).
unsafe fn cqa1_faixa(copa: usize, rodada: u32, laco: u32) -> (i16, i16) {
    if laco as u16 != 0xffff {
        (laco as i16, laco.wrapping_add(1) as i16)
    } else {
        (0, cqa1_w(cqa1_rodada(copa, rodada as i16 as i32) + 0x1a))
    }
}

/// Troca mandante e visitante no confronto `t`: os pares de palavras +0/+2, +0xa/+0xc e +0xe/+0x10.
unsafe fn cqa1_inverter(t: usize) {
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
unsafe extern "thiscall" fn copa_mandantes_da_rodada(copa: usize, rodada: u32, laco: u32) {
    let r = rodada as i16 as i32;
    let (ini, fim) = cqa1_faixa(copa, rodada, laco);
    let regra = cqa1_b(cqa1_rodada(copa, r) + 0xb);
    if regra & 1 == 0 && regra & 8 == 0 && cqa1_i(copa + 0x24) == -1 {
        return;
    }
    let mut k = ini as i32;
    while k < fim as i32 {
        let t = cqa1_confronto(copa, r, k);
        let (a, b) = (cqa1_w(t), cqa1_w(t + 2));
        k += 1;
        if a == -1 || b == -1 {
            continue;
        }
        let (ta, tb) = (cqa1_time(copa, a as i32), cqa1_time(copa, b as i32));
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
            cqa1_inverter(t);
        }
    }
}

/// Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
unsafe extern "thiscall" fn comparar_copa_mandantes_da_rodada(copa: usize, rodada: u32, laco: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1420);
    let (ini, fim) = cqa1_faixa(copa, rodada, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((cqa1_confronto(copa, rodada as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada, laco), || copa_mandantes_da_rodada(copa, rodada, laco));
    PLACAR.registrar(igual, || format!("copa_mandantes_da_rodada(rodada {}, laco {}) original={:x?} rust={:x?}", rodada as i16, laco as i16, mo, mr));
}

/// Os argumentos da virtual +0x68 que a 0x521b30 chama para o time `t` na rodada `r`: (p1, p2, p3) ou None.
unsafe fn cqa1_progresso(copa: usize, t: i32, r: i32) -> Option<(u8, u8, u8)> {
    let n = cqa1_w(copa + 0x3c) as i32;
    let d = cqa1_b(cqa1_time(copa, t) + 5); // o desfecho
    let byte_da_rodada = |k: i32, desl: usize| cqa1_b(cqa1_rodada(copa, k) + desl);
    if n > 0 && cqa1_w(cqa1_rodada(copa, n - 1) + 7) == 0x8c {
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
unsafe extern "thiscall" fn copa_progresso_do_time(copa: usize, time: u32, rodada: u32) {
    let (t, r) = (time as i16 as i32, rodada as i16 as i32);
    if let Some((p1, p2, p3)) = cqa1_progresso(copa, t, r) {
        // a virtual +0x68: 5 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, comp, p1, p2, p3), ret 0x14
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(cqa1_d(cqa1_d(copa) + 0x68));
        // ponytail: a original empurra bytes sobre registradores inteiros (o ebx de quem chamou, o dword do argumento
        // `time` com o byte baixo trocado, 13 × rodada): os bits altos são sobra; as 70 versões da +0x68 só leem o byte
        // baixo de cada um (ou repassam os dwords a outra que lê): aqui, os bytes limpos.
        f(copa, cqa1_d(cqa1_time(copa, t)), cqa1_b(copa + 0x44) as u32, p1 as u32, p2 as u32, p3 as u32);
    }
}

/// Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
unsafe extern "thiscall" fn comparar_copa_progresso_do_time(copa: usize, time: u32, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1421);
    let stats = cqa1_d(copa + 8);
    let clube = cqa1_d(cqa1_time(copa, time as i16 as i32));
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let reg = if stats != 0 && clube != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(copa, time, rodada); // ponytail: sem registro para olhar, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(copa, time, rodada), || copa_progresso_do_time(copa, time, rodada));
    PLACAR.registrar(igual, || format!("copa_progresso_do_time(time {}, rodada {}) original={:x?} rust={:x?}", time as i16, rodada as i16, mo, mr));
}

// ---------- 263a leva: os amistosos dos clubes: a resposta ao pedido de amistoso, o pedido enviado e a lista das nações para os amistosos internacionais (friendly.cpp) ----------

unsafe fn cqa2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqa2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v)
}

/// O outro clube do pedido de amistoso (0x19 bytes: +0 dia, +2 ano, +0xc quem pede, +0x10 e +0x14 os dois clubes do
/// jogo, +0x18 a situação): o +0x10, ou o +0x14 quando o +0x10 é o que pede.
unsafe fn cqa2_outro(m: usize) -> usize {
    if cqa2_d(m + 0xc) == cqa2_d(m + 0x10) {
        cqa2_d(m + 0x14)
    } else {
        cqa2_d(m + 0x10)
    }
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4; a cópia Rust é stdcall(clube): o this não é lido).
unsafe fn cqa2_humano(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(clube) != 0
}

/// A notícia do amistoso (`tipo` 6 ou 7) numa local de 0xe0 bytes: 0x77bbe0 (push 0; push tipo; ecx = a local; sem add
/// esp -> thiscall, ret 8) com os dados 0x785790 (push valor; push k; ecx = a local -> thiscall, ret 8) 0 e 1 = os ids
/// dos clubes do jogo (+0x10, +0x14), 2 = o id de quem pede (+0xc), 3 = o dia (com sinal), 4 = o ano (sem sinal), 5 =
/// a situação; entregue ao técnico do `para` (0x7861e0: push para; push &local; ecx = [0xb7431c]; sem add esp ->
/// thiscall, ret 8).
unsafe fn cqa2_noticia(m: usize, tipo: u32, situacao: u32, para: usize) {
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    construir(n, tipo, 0);
    gravar(n, 0, cqa2_d(cqa2_d(m + 0x10)) as u32);
    gravar(n, 1, cqa2_d(cqa2_d(m + 0x14)) as u32);
    gravar(n, 2, cqa2_d(cqa2_d(m + 0xc)) as u32);
    gravar(n, 3, cqa2_w(m) as i32 as u32);
    gravar(n, 4, core::ptr::read_unaligned((m + 2) as *const u16) as u32);
    gravar(n, 5, situacao);
    entregar(cqa2_d(0xb7431c), n, para);
}

/// A 0x5c5f40 decide: o outro clube existe, a situação é 1 e ele não é de técnico humano (0x600a50: push 0; push 0;
/// push 1; push 0; push o; add esp, 0x14 -> cdecl(o, 0, 1, 0, 0)), salvo com `forcar`.
unsafe fn cqa2_decide(m: usize, forcar: u32) -> bool {
    let o = cqa2_outro(m);
    if o == 0 || cqa2_b(m + 0x18) != 1 {
        return false;
    }
    let clube_humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    clube_humano(o, 0, 1, 0, 0) == 0 || forcar != 0
}

/// A decisão do convidado (depois da cqa2_decide): 3 aceita, 4 recusa. O fator: 1,65 (1,32 quando o
/// outro é o +0x10), × 0,925 com nações diferentes (+0x53), × 0,85 com regiões (+0x76) diferentes, × 0,85 com
/// continentes (+0x71) diferentes; quando o outro é o +0x10 e os dois têm cidade do estádio (+0x69 → +0x38), × (1 +
/// +0x77 de quem pede / ((distância das coordenadas +0x23/+0x2b × 0,55 + 1) × 40000)). Aceita se a reputação (+0x80)
/// de quem pede é ao menos a do outro / fator, ou se a diferença das reputações não passa de fator × 600.
unsafe fn cqa2_decisao(m: usize, contar: bool) -> u8 {
    let a = cqa2_d(m + 0xc);
    let o = cqa2_outro(m);
    let (na, no) = (cqa2_d(a + 0x53), cqa2_d(o + 0x53));
    let segundo = o == cqa2_d(m + 0x10);
    let mut f = if segundo { f64_em(0x96af38) } else { f64_em(0x96af20) };
    if no != na {
        f *= f64_em(0x96af30);
    }
    if cqa2_b(no + 0x76) != cqa2_b(na + 0x76) {
        f *= f64_em(0x969520);
    }
    if cqa2_i(no + 0x71) != cqa2_i(na + 0x71) {
        f *= f64_em(0x969520);
    }
    if segundo {
        let (fa, fo) = (cqa2_d(a + 0x69), cqa2_d(o + 0x69));
        let sa = if fa != 0 { cqa2_d(fa + 0x38) } else { 0 };
        let so = if sa != 0 && fo != 0 { cqa2_d(fo + 0x38) } else { 0 };
        if sa != 0 && so != 0 {
            if contar {
                crate::comparacao::ramo("cqa_2 distancia do amistoso");
            }
            let dx = f64_em(sa + 0x2b) - f64_em(so + 0x2b);
            let dy = f64_em(sa + 0x23) - f64_em(so + 0x23);
            let i = cqa2_i(a + 0x77) as f64;
            let v = i / (((dy * dy + dx * dx).sqrt() * f64_em(0x96a5d8) + f64_em(0x967890)) * f64_em(0x96af28)) + f64_em(0x967890);
            f *= v; // cqa2: o fator da distância
        }
    }
    let (ro, ra) = (cqa2_w(o + 0x80) as i32, cqa2_w(a + 0x80) as i32);
    if contar {
        crate::comparacao::ramo("cqa_2 decisao do amistoso");
    }
    // fcompp/test ah, 1: o C0 vale para "menor" e para "sem ordem" (NaN): os >= abaixo dão o mesmo caminho
    let aceita = ra as f64 >= ro as f64 / f || f * f64_em(0x96a690) >= ro.wrapping_sub(ra) as f64; // cqa2: aceita pela reputação
    if aceita { 3 } else { 4 }
}

/// { 0x5c5f40 } (thiscall, ret 8, friendly.cpp, quadro SEH sem destrutor no caminho normal):
/// `Friendly::process_friendly_reply(pedido, forcar)`. Sem quem pede (+0xc): nada. Com o outro clube (+0x10, ou +0x14
/// se o +0x10 é quem pede) na situação 1 e sem técnico humano (0x600a50) ou com `forcar`: sem nação num dos dois, a
/// situação 4 (sobrescrita logo depois); a decisão (cqa2_decisao) grava 3 ou 4 em +0x18 e a notícia 7 vai a quem pede.
/// Aceito (3): a partida do amistoso numa local de 0x4f bytes (a competição [+4] e o id dela, o estádio do clube +0x10 (+0x69) e o
/// id dele, os clubes +0x10 e +0x14 e os ids, a data do pedido, o ano de hoje, 9 para seleção (0x53b900) ou 1, o horário 1
/// no sábado e no domingo (0x54a3f0 = 5 ou 6) ou 2, as marcas, −1 nos placares) vai ao calendário (0x5aa680: push 1;
/// push &local; ecx = [0xae2a58] -> thiscall, ret 8). Com 3 ou 4: a vaga do amistoso marcado (0x5c6740) fica com +0x18
/// = 3 ou com +0x10 = 0 e +0x18 = 0, e o pedido sai (0x5c87b0 com 0). Noutra situação, o pedido sai da lista das
/// respostas (+0x34d4; 0x68d1f0, o cursor +0xa volta ao começo antes, depois de cada um e no fim).
unsafe extern "thiscall" fn amistoso_responder_pedido(this: usize, m: usize, forcar: u32) {
    let a = cqa2_d(m + 0xc);
    if a == 0 {
        return;
    }
    let o = cqa2_outro(m);
    if cqa2_decide(m, forcar) {
        if cqa2_d(a + 0x53) == 0 || cqa2_d(o + 0x53) == 0 {
            *((m + 0x18) as *mut u8) = 4; // como a original (que cai logo depois, lendo a nação nula)
        }
        let s = cqa2_decisao(m, true);
        *((m + 0x18) as *mut u8) = s;
        cqa2_noticia(m, 7, s as u32, a);
    }
    let s = cqa2_b(m + 0x18);
    if s == 3 {
        crate::comparacao::ramo("cqa_2 amistoso aceito");
        let mut p = [0u8; 0x50];
        let pp = p.as_mut_ptr() as usize;
        let comp = cqa2_d(this + 4);
        let (c2, c3) = (cqa2_d(m + 0x10), cqa2_d(m + 0x14));
        let estadio = cqa2_d(c2 + 0x69);
        cqa2_pd(pp, cqa2_d(comp));
        cqa2_pd(pp + 0x14, comp);
        cqa2_pd(pp + 0x18, estadio);
        cqa2_pd(pp + 4, if estadio != 0 { cqa2_d(estadio) } else { usize::MAX });
        let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        let sel = e_selecao(c2); // push c2; add esp, 4 -> cdecl
        p[0x2c..0x2e].copy_from_slice(&core::ptr::read_unaligned(0xae2c92 as *const u16).to_le_bytes());
        p[0x3a] = if sel != 0 { 9 } else { 1 };
        p[0x40] = 1;
        p[0x41] = 1;
        for k in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x3b, 0x42] {
            p[k] = 0xff;
        }
        cqa2_pd(pp + 8, usize::MAX);
        cqa2_pd(pp + 0x1c, c2); // cqa2: o mandante do amistoso
        cqa2_pd(pp + 0xc, cqa2_d(c2));
        cqa2_pd(pp + 0x20, c3);
        cqa2_pd(pp + 0x10, cqa2_d(c3));
        p[0x2a..0x2c].copy_from_slice(&core::ptr::read_unaligned(m as *const u16).to_le_bytes());
        p[0x28..0x2a].copy_from_slice(&core::ptr::read_unaligned((m + 2) as *const u16).to_le_bytes());
        let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
        p[0x3f] = if semana(m) as i16 == 5 || semana(m) as i16 == 6 { 1 } else { 2 };
        // ponytail: a original faz and 0xf8ff e depois and 0xff99 / or 0x99 na palavra +0x4d de um local sem valor (os
        // bits 11..15 ficam com a sobra de pilha) e não grava +0x24..+0x27, +0x2e, +0x2f, +0x3c: aqui, zero.
        p[0x4d] = 0x99;
        p[0x3d] = 0;
        p[0x3e] = 0;
        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5aa680usize);
        juntar(cqa2_d(0xae2a58), pp, 1);
    }
    let s = cqa2_b(m + 0x18);
    if s == 3 || s == 4 {
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
        // push +0x10; push +0x14; push ano; push dia; ecx = this; sem add esp -> thiscall, ret 0x10
        // ponytail: o dia e o ano vão em registradores com lixo nos 16 bits altos; a 0x5c6740 só lê as palavras
        let v = vaga(this, cqa2_w(m) as u16 as u32, cqa2_w(m + 2) as u16 as u32, cqa2_d(m + 0x14), cqa2_d(m + 0x10));
        if v != 0 {
            if s == 3 {
                *((v + 0x18) as *mut u8) = 3;
            } else {
                cqa2_pd(v + 0x10, 0);
                *((v + 0x18) as *mut u8) = 0;
            }
        }
        let tirar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c87b0usize);
        tirar(this, m, 0); // push 0; push m; ecx = this -> thiscall, ret 8
        return;
    }
    let l = cqa2_d(this + 0x34d4);
    if l == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    cqa2_pd(l + 0xa, cqa2_d(l + 2));
    let mut item = proximo(cqa2_d(this + 0x34d4));
    while item != 0 {
        if cqa2_d(item) == m {
            tirar(cqa2_d(this + 0x34d4), item); // push item; ecx = a lista -> thiscall, ret 4
            let l = cqa2_d(this + 0x34d4);
            cqa2_pd(l + 0xa, cqa2_d(l + 2));
        }
        item = proximo(cqa2_d(this + 0x34d4));
    }
    let l = cqa2_d(this + 0x34d4);
    cqa2_pd(l + 0xa, cqa2_d(l + 2));
}

/// Algum nó da lista (+2 a cabeça, +6 o próximo de cada nó, +0 o dado) guarda `m`, sem mexer no cursor.
unsafe fn cqa2_na_lista(l: usize, m: usize) -> bool {
    if l == 0 {
        return false;
    }
    let mut no = cqa2_d(l + 2);
    let mut guarda = 0;
    while no != 0 && guarda < 100_000 {
        if cqa2_d(no) != 0 && cqa2_d(cqa2_d(no)) == m {
            return true;
        }
        no = cqa2_d(no + 6);
        guarda += 1;
    }
    false
}

/// Comparador da 0x5c5f40: só a original (a partida vai ao calendário e a notícia ao técnico, que alocam, e o pedido é
/// liberado pela 0x5c87b0). O plano, feito antes: a decisão (cqa2_decisao) e a vaga do amistoso (0x5c6740). Com 3 ou 4,
/// a vaga fica com +0x18 = 3, ou com +0x10 = 0 e +0x18 = 0; sem decisão, o pedido continua com a mesma situação e sai
/// da lista das respostas (+0x34d4). Sem quem pede ou sem nação num dos dois: só a original.
unsafe extern "thiscall" fn comparar_amistoso_responder_pedido(this: usize, m: usize, forcar: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1422);
    let a = if m != 0 { cqa2_d(m + 0xc) } else { 0 };
    let outro = if m != 0 { cqa2_outro(m) } else { 0 };
    if a == 0 || (outro != 0 && (cqa2_d(a + 0x53) == 0 || cqa2_d(outro + 0x53) == 0)) {
        return o(this, m, forcar); // ponytail: sem quem pede ou sem nação, só a original
    }
    let s = if cqa2_decide(m, forcar) { cqa2_decisao(m, false) } else { cqa2_b(m + 0x18) };
    let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
    let v = if s == 3 || s == 4 { vaga(this, cqa2_w(m) as u16 as u32, cqa2_w(m + 2) as u16 as u32, cqa2_d(m + 0x14), cqa2_d(m + 0x10)) } else { 0 };
    o(this, m, forcar);
    let ok = if s == 3 || s == 4 {
        // o pedido foi liberado: só a vaga
        v == 0 || if s == 3 { cqa2_b(v + 0x18) == 3 } else { cqa2_d(v + 0x10) == 0 && cqa2_b(v + 0x18) == 0 }
    } else {
        cqa2_b(m + 0x18) == s && !cqa2_na_lista(cqa2_d(this + 0x34d4), m)
    };
    PLACAR.registrar(ok, || format!("amistoso_responder_pedido(pedido {:x}) situação prevista {} vaga {:x}", m, s, v));
}

/// { 0x5c5570 } (thiscall, ret 4, friendly.cpp, quadro SEH sem destrutor no caminho normal):
/// `Friendly::send_friendly_request(pedido)`: sem a lista das respostas (+0x34d4), cria-a (0x944cf1(0xe): push 0xe;
/// add esp, 4 -> cdecl; 0x68d020: ecx = o bloco -> thiscall, ret 0); sem memória, a caixa (linha 0x982) e nada mais.
/// Guarda o ponteiro do pedido na lista (0x68d0d0: push 4; push &pedido; ecx = a lista -> thiscall, ret 8). Com o outro
/// clube na situação 1 e de técnico humano: situação 2 e, se o +0x10 ou o +0x14 é de técnico humano, a notícia 6 ao
/// outro.
unsafe extern "thiscall" fn amistoso_enviar_pedido(this: usize, m: usize) {
    let o = cqa2_outro(m);
    if cqa2_d(this + 0x34d4) == 0 {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let bloco = novo(0xe);
        let l = if bloco != 0 {
            let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            construir(bloco)
        } else {
            0
        };
        cqa2_pd(this + 0x34d4, l);
        if l == 0 {
            return orig::<unsafe extern "thiscall" fn(usize, usize)>(1423)(this, m); // ponytail: sem memória, a caixa da original
        }
    }
    let guardar: unsafe extern "thiscall" fn(usize, *const usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let copia = m;
    guardar(cqa2_d(this + 0x34d4), &copia, 4);
    if o == 0 || cqa2_b(m + 0x18) != 1 || !cqa2_humano(o) {
        return;
    }
    crate::comparacao::ramo("cqa_2 pedido ao humano");
    *((m + 0x18) as *mut u8) = 2;
    if !cqa2_humano(cqa2_d(m + 0x10)) && !cqa2_humano(cqa2_d(m + 0x14)) {
        return;
    }
    cqa2_noticia(m, 6, 2, o);
}

/// Quantos nós a lista tem, sem mexer no cursor.
unsafe fn cqa2_conta(l: usize) -> usize {
    let mut n = 0;
    let mut no = if l != 0 { cqa2_d(l + 2) } else { 0 };
    while no != 0 && n < 100_000 {
        n += 1;
        no = cqa2_d(no + 6);
    }
    n
}

/// Comparador da 0x5c5570: só a original (o nó da lista e a notícia alocam). O plano, feito antes: a lista das
/// respostas fica com um nó a mais e guarda o pedido; a situação vai a 2 quando o outro clube, na situação 1, é de
/// técnico humano; senão não muda.
unsafe extern "thiscall" fn comparar_amistoso_enviar_pedido(this: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1423);
    let outro = cqa2_outro(m);
    let antes = cqa2_conta(cqa2_d(this + 0x34d4));
    let s = if outro != 0 && cqa2_b(m + 0x18) == 1 && cqa2_humano(outro) { 2 } else { cqa2_b(m + 0x18) };
    o(this, m);
    let l = cqa2_d(this + 0x34d4);
    let ok = l == 0 || (cqa2_conta(l) == antes + 1 && cqa2_na_lista(l, m) && cqa2_b(m + 0x18) == s);
    PLACAR.registrar(ok, || format!("amistoso_enviar_pedido(pedido {:x}) nós {} -> {} situação {} prevista {}", m, antes, cqa2_conta(l), cqa2_b(m + 0x18), s));
}

/// As nações com continente (+0x71) da tabela [0xae23a8] (0x122 bytes cada, [0xae2350] delas), na ordem da tabela.
unsafe fn cqa2_nacoes_com_continente() -> Vec<usize> {
    let mut v = Vec::new();
    let t = cqa2_d(0xae23a8);
    let mut i = 0;
    while i < cqa2_i(0xae2350) {
        let n = t.wrapping_add(i as usize * 0x122);
        if cqa2_d(n + 0x71) != 0 {
            v.push(n);
        }
        i += 1;
    }
    v
}

/// O qsort da CRT (0x945333: 4 pushes; add esp, 0x10 -> cdecl(base, n, 4, ordem)) com a ordem 0x5cc5f0 (reputação,
/// a maior primeiro).
unsafe fn cqa2_ordenar(base: usize, n: u32) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(base, n, 4, 0x5cc5f0);
}

/// { 0x5ca3e0 } (thiscall, ret 0, friendly.cpp): `Friendly::build_nation_list()`: +0x34fe = 0; aloca [0xae2350] × 4
/// bytes (0x944e46: push n; add esp, 4 -> cdecl) em +0x34fa (sem memória, a caixa, linha 0x15af); põe nele as nações
/// com continente (+0x71), contando em +0x34fe, e ordena pela reputação (0x945333 com 0x5cc5f0).
unsafe extern "thiscall" fn amistoso_lista_das_nacoes(this: usize) {
    core::ptr::write_unaligned((this + 0x34fe) as *mut i32, 0);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let p = malloc((cqa2_i(0xae2350) as u32).wrapping_mul(4));
    cqa2_pd(this + 0x34fa, p);
    if p == 0 {
        return orig::<unsafe extern "thiscall" fn(usize)>(1424)(this); // ponytail: sem memória, a caixa da original
    }
    for n in cqa2_nacoes_com_continente() {
        let k = cqa2_i(this + 0x34fe);
        cqa2_pd(cqa2_d(this + 0x34fa).wrapping_add((k as isize * 4) as usize), n);
        core::ptr::write_unaligned((this + 0x34fe) as *mut i32, k + 1);
    }
    cqa2_ordenar(cqa2_d(this + 0x34fa), cqa2_i(this + 0x34fe) as u32);
}

/// Comparador da 0x5ca3e0: só a original (aloca a lista). O plano, feito antes numa lista local com o mesmo qsort e a
/// mesma ordem: a conta (+0x34fe) e as nações, na ordem.
unsafe extern "thiscall" fn comparar_amistoso_lista_das_nacoes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1424);
    let mut plano = cqa2_nacoes_com_continente();
    cqa2_ordenar(plano.as_mut_ptr() as usize, plano.len() as u32);
    o(this);
    let p = cqa2_d(this + 0x34fa);
    let n = cqa2_i(this + 0x34fe);
    let ok = p == 0 || (n as usize == plano.len() && (0..plano.len()).all(|k| cqa2_d(p + 4 * k) == plano[k]));
    PLACAR.registrar(ok, || format!("amistoso_lista_das_nacoes: conta {} prevista {}", n, plano.len()));
}

// ---------- 263a leva: o sorteio da rodada da copa, o nome da fase e da rodada e o clube da divisão mais alta (Cup.cpp, comp_util.cpp) ----------

unsafe fn cqa3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3); `r` como a original o usa (o dword inteiro).
unsafe fn cqa3_rodada(copa: usize, r: u32) -> usize {
    cqa3_d(copa + 0xa3).wrapping_add((r as usize).wrapping_mul(0x68))
}

/// Os confrontos da rodada `r` (+0xad: um vetor de confrontos de 0x35 bytes por rodada).
unsafe fn cqa3_confrontos(copa: usize, r: u32) -> usize {
    cqa3_d(cqa3_d(copa + 0xad).wrapping_add((r as usize).wrapping_mul(4)))
}

/// Algum time definido (≠ −1) de algum confronto da rodada ainda sem clube (o vencedor de um confronto anterior em
/// aberto): a rodada não pode ser sorteada.
unsafe fn cqa3_time_em_aberto(copa: usize, r: u32) -> bool {
    let reg = cqa3_rodada(copa, r);
    let mut t = cqa3_confrontos(copa, r);
    let mut k = 0i32;
    while k < cqa3_w(reg + 0x1a) as i32 {
        for lado in [0usize, 2] {
            let x = cqa3_w(t + lado);
            if x != -1 && cqa3_d(cqa3_d(copa + 0xa7).wrapping_add((x as isize * 6) as usize)) == 0 {
                return true;
            }
        }
        t += 0x35;
        k += 1;
    }
    false
}

/// A data do sorteio da rodada (+0 o dia, +2 o ano em byte a partir de +0x40 da copa), numa Data local (0x549ef0:
/// push ano; push dia; ecx = a local; sem add esp -> thiscall, ret 8; os dois em registradores com lixo nos bits altos,
/// a Rust só lê as palavras).
unsafe fn cqa3_data_da_rodada(copa: usize, reg: usize, d: *mut [u8; 8]) {
    let nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let ano = (cqa3_b(reg + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40));
    nova(d, cqa3_w(reg) as u16 as u32, ano as u16 as u32);
}

/// Grava a data `d` como a do sorteio da rodada: a palavra do dia e o byte do ano menos o byte baixo de +0x40.
unsafe fn cqa3_gravar_data(copa: usize, reg: usize, d: &[u8; 8]) {
    core::ptr::write_unaligned(reg as *mut [u8; 2], [d[0], d[1]]);
    *((reg + 2) as *mut u8) = d[2].wrapping_sub(cqa3_b(copa + 0x40));
}

/// A data `a` vem depois da `b` (o ano sem sinal, depois o dia com sinal), como a comparação da 0x51c410.
fn cqa3_depois(a: &[u8; 8], b: &[u8; 8]) -> bool {
    let (aa, ab) = (u16::from_le_bytes([a[2], a[3]]), u16::from_le_bytes([b[2], b[3]]));
    aa > ab || (aa == ab && i16::from_le_bytes([a[0], a[1]]) > i16::from_le_bytes([b[0], b[1]]))
}

/// A competição não leva a notícia do sorteio: menos de 2 confrontos, regra 0x200 (+0xd), competição especial das
/// seleções (0x76d940 em [0xb74318]: push comp; ecx = as seleções; sem add esp -> ret 4; a cópia Rust é stdcall(comp))
/// ou amistosa (0x5c6710 em [0xae2c74]: push comp; ecx = o Friendly -> thiscall, ret 4).
unsafe fn cqa3_sem_noticia(copa: usize, r: u32) -> bool {
    let reg = cqa3_rodada(copa, r);
    if cqa3_w(reg + 0x1a) < 2 || cqa3_w(reg + 0xd) & 0x200 != 0 {
        return true;
    }
    if cqa3_d(0xb74318) != 0 {
        let especial: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x76d940usize);
        if especial(cqa3_d(copa + 4)) != 0 {
            return true;
        }
    }
    if cqa3_d(0xae2c74) != 0 {
        let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistosa(cqa3_d(0xae2c74), cqa3_d(copa + 4)) != 0 {
            return true;
        }
    }
    false
}

/// { 0x51c410 } (thiscall, ret 4, Cup.cpp, quadro SEH sem destrutor no caminho normal): `Cup::draw_round(rodada)`;
/// devolve 1. Sem +0x45 ou com a rodada já sorteada (+0x34 do primeiro confronto): nada. Com um time de algum confronto
/// ainda sem clube: a data do sorteio anda 1 dia (0x54c5b0, 0x417c10) e as das rodadas seguintes passam a ser, no
/// mínimo, essa. Senão sorteia: regra 0x400 (+0xd) pela semente (0x51d2d0), regra 4 (+0xb) separando as nações
/// (0x51d9f0), sem 0x200 o sorteio livre (0x51e220); os mandantes (0x51e870 com −1); estádio compartilhado (+0x21 = 2,
/// 0x522610); marca os confrontos sorteados (+0x34 = 1). Com notícia (cqa3_sem_noticia): rodada que não é a do
/// terceiro lugar (+7 ≠ 0x8c) soma 1 em +0xac; a notícia 0x27 (a competição, +0x3a, o tipo, +0x44, a rodada, 0, e 1
/// se +0xab + 1 < +0xac) vai a cada um dos times ainda vivos (desfecho 0) entre os primeiros (soma das entradas +0x1c
/// das rodadas até esta) que é de técnico humano, com o id dele no dado 3.
unsafe extern "thiscall" fn copa_sortear_rodada(copa: usize, rodada: u32) -> u32 {
    if cqa3_d(copa + 0x45) == 0 || cqa3_b(cqa3_confrontos(copa, rodada) + 0x34) != 0 {
        return 1;
    }
    let reg = cqa3_rodada(copa, rodada);
    if cqa3_time_em_aberto(copa, rodada) {
        let mais: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> *mut [u8; 8] = core::mem::transmute(0x54c5b0usize);
        let copia: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(0x417c10usize);
        let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        cqa3_data_da_rodada(copa, reg, &mut a);
        crate::comparacao::ramo("cqa_3 sorteio adiado");
        let r = mais(&mut a, &mut b, 1); // cqa3: o sorteio adiado
        copia(&mut a, &mut c, r); // push &b; push &c; ecx = &a -> thiscall, ret 8
        cqa3_gravar_data(copa, reg, &a);
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < cqa3_w(copa + 0x3c) {
            let rk = cqa3_rodada(copa, k as i16 as i32 as u32);
            cqa3_data_da_rodada(copa, rk, &mut b);
            if cqa3_depois(&a, &b) {
                cqa3_gravar_data(copa, rk, &a);
            }
            k = k.wrapping_add(1);
        }
        return 1;
    }
    // os sorteios: push rodada; ecx = a copa; sem add esp -> thiscall, ret 4 (as três)
    let sorteio = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
        g(copa, rodada)
    };
    let regra = cqa3_w(reg + 0xd);
    if regra & 0x400 != 0 {
        crate::comparacao::ramo("cqa_3 sorteio pela semente");
        sorteio(0x51d2d0); // cqa3: pela semente
    } else if cqa3_b(reg + 0xb) & 4 != 0 {
        sorteio(0x51d9f0);
    } else if regra & 0x200 == 0 {
        crate::comparacao::ramo("cqa_3 sorteio livre");
        sorteio(0x51e220); // cqa3: o sorteio livre
    }
    // push −1; push rodada; ecx = a copa -> thiscall, ret 8
    let mandantes: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x51e870usize);
    mandantes(copa, rodada, u32::MAX);
    if cqa3_b(cqa3_rodada(copa, rodada) + 0x21) == 2 {
        sorteio(0x522610);
    }
    let mut k = 0i32;
    while k < cqa3_w(cqa3_rodada(copa, rodada) + 0x1a) as i32 {
        *((cqa3_confrontos(copa, rodada) + k as usize * 0x35 + 0x34) as *mut u8) = 1;
        k += 1;
    }
    if cqa3_sem_noticia(copa, rodada) {
        return 1;
    }
    let reg = cqa3_rodada(copa, rodada);
    if cqa3_w(reg + 7) != 0x8c {
        *((copa + 0xac) as *mut u8) = cqa3_b(copa + 0xac).wrapping_add(1);
    }
    let mut soma: i16 = 0;
    if (rodada as i32) >= 0 {
        for i in 0..=rodada {
            soma = soma.wrapping_add(cqa3_w(cqa3_rodada(copa, i) + 0x1c));
        }
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    construir(n, 0x27, 0); // push 0; push 0x27; ecx = a local -> thiscall, ret 8
    gravar(n, 0, cqa3_d(cqa3_d(copa + 4)) as u32); // push valor; push k; ecx = a local -> thiscall, ret 8
    gravar(n, 1, cqa3_w(copa + 0x3a) as i32 as u32);
    gravar(n, 2, cqa3_w(cqa3_rodada(copa, rodada) + 7) as i32 as u32);
    gravar(n, 4, cqa3_b(copa + 0x44) as i8 as i32 as u32);
    gravar(n, 5, rodada);
    gravar(n, 6, 0);
    gravar(n, 7, ((cqa3_b(copa + 0xab) as i8 as i32 + 1) < cqa3_b(copa + 0xac) as i8 as i32) as u32);
    for i in 0..soma.max(0) as usize {
        let e = cqa3_d(copa + 0xa7) + 6 * i;
        if cqa3_b(e + 5) != 0 || cqa3_d(e) == 0 {
            continue;
        }
        // 0x7875e0: push clube; ecx = [0xb7431c] -> ret 4 (a cópia Rust é stdcall(clube))
        if humano(cqa3_d(e)) == 0 {
            continue;
        }
        gravar(n, 3, cqa3_d(cqa3_d(cqa3_d(copa + 0xa7) + 6 * i)) as u32);
        // push clube; push &local; ecx = [0xb7431c]; sem add esp -> thiscall, ret 8
        entregar(cqa3_d(0xb7431c), n, cqa3_d(cqa3_d(copa + 0xa7) + 6 * i));
    }
    1
}

/// Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
unsafe extern "thiscall" fn comparar_copa_sortear_rodada(copa: usize, rodada: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1425);
    if cqa3_d(copa + 0x45) == 0 || cqa3_b(cqa3_confrontos(copa, rodada) + 0x34) != 0 {
        return o(copa, rodada); // ponytail: nada a sortear, a original não faz nada
    }
    let n = cqa3_w(copa + 0x3c);
    if cqa3_time_em_aberto(copa, rodada) {
        // o plano das datas: (registro, palavra do dia, byte do ano) de cada rodada daqui em diante
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (cqa3_w(cqa3_rodada(copa, rodada)), (cqa3_b(cqa3_rodada(copa, rodada) + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40)) as u16);
        // a 0x54c5b0 (mais 1): normalizar para a frente, 365 dias no bissexto (crate::data::bissexto)
        dia = dia.wrapping_add(1);
        let tam = |y: u16| if crate::data::bissexto(y) == 1 { 365i16 } else { 364 };
        while dia > tam(ano) {
            dia = dia.wrapping_sub(tam(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plano = vec![(cqa3_rodada(copa, rodada), dia, a[2].wrapping_sub(cqa3_b(copa + 0x40)))];
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < n {
            let rk = cqa3_rodada(copa, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (cqa3_b(rk + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40)) as u16;
            b[0..2].copy_from_slice(&cqa3_w(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plano.push(if cqa3_depois(&a, &b) { (rk, dia, a[2].wrapping_sub(cqa3_b(copa + 0x40))) } else { (rk, cqa3_w(rk), cqa3_b(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(copa, rodada);
        let ok = plano.iter().all(|&(rk, d, y)| cqa3_w(rk) == d && cqa3_b(rk + 2) == y);
        PLACAR.registrar(ok, || format!("copa_sortear_rodada(rodada {}) as datas adiadas diferem do plano", rodada as i32));
        return r;
    }
    let antes_ac = cqa3_b(copa + 0xac);
    let r = o(copa, rodada);
    let mut ok = r == 1;
    let reg = cqa3_rodada(copa, rodada);
    for k in 0..cqa3_w(reg + 0x1a).max(0) as usize {
        ok &= cqa3_b(cqa3_confrontos(copa, rodada) + k * 0x35 + 0x34) == 1;
    }
    let soma = if !cqa3_sem_noticia(copa, rodada) && cqa3_w(reg + 7) != 0x8c { 1 } else { 0 };
    ok &= cqa3_b(copa + 0xac) == antes_ac.wrapping_add(soma);
    PLACAR.registrar(ok, || format!("copa_sortear_rodada(rodada {}) marcas dos confrontos ou +0xac ({} -> {}) diferem do plano", rodada as i32, antes_ac, cqa3_b(copa + 0xac)));
    r
}

/// { 0x4bffc0 } (cdecl, comp_util.cpp): `comp_util_format_stage_round_name(fase, rodada, saida, tamanho, estilo)`:
/// sem saída ou com menos de 200 bytes, a caixa (linha 0x26a1) e 0. Zera a saída; com fase e rodada, "{fase} {rodada}"
/// (0x66f4e0 com o modelo 0x9a7cfc, os nomes de 100 bytes da 0x4b4850 e os gêneros da 0x4ba8f0); só uma delas, o nome
/// dela direto na saída (0x4b4850 com o tamanho); nenhuma, só o zero. Devolve 1 (0 na caixa).
unsafe extern "C" fn nome_da_fase_e_rodada(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    if saida.is_null() || tamanho < 0xc8 {
        return orig::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(1426)(fase, rodada, saida, tamanho, estilo); // a caixa
    }
    // 0x4b4850: 4 pushes (estilo, id, tamanho, saída); add esp -> cdecl; 0x4ba8f0: push estilo; push id -> cdecl
    let nome: unsafe extern "C" fn(*mut u8, i32, u32, u32) -> usize = core::mem::transmute(0x4b4850usize);
    let genero: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x4ba8f0usize);
    *saida = 0;
    if fase != 0 && rodada != 0 {
        let (mut a, mut b) = ([0u8; 0x64], [0u8; 0x64]);
        nome(a.as_mut_ptr(), 0x64, fase, estilo);
        let ga = genero(fase, estilo) as i8 as i32 as u32;
        nome(b.as_mut_ptr(), 0x64, rodada, estilo);
        let gb = genero(rodada, estilo) as i8 as i32 as u32;
        // 0x66f4e0: 8 pushes; add esp, 0x20 -> cdecl (saída, modelo, ga, ga, gb, gb, &a, &b)
        let texto: unsafe extern "C" fn(*mut u8, usize, u32, u32, u32, u32, *const u8, *const u8) -> usize = core::mem::transmute(0x66f4e0usize);
        texto(saida, 0x9a7cfc, ga, ga, gb, gb, a.as_ptr(), b.as_ptr());
    } else if fase != 0 {
        nome(saida, tamanho, fase, estilo);
    } else if rodada != 0 {
        nome(saida, tamanho, rodada, estilo);
    } else {
        return 0; // nenhuma das duas: 0 (0x4c0136)
    }
    1
}

/// Comparador da 0x4bffc0: lado a lado na saída (até o tamanho, no máximo 0x400 bytes) e na devolução.
unsafe extern "C" fn comparar_nome_da_fase_e_rodada(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    let o = orig::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(1426);
    if saida.is_null() || tamanho < 0xc8 {
        return o(fase, rodada, saida, tamanho, estilo); // a caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida as usize, (tamanho as usize).min(0x400))],
        || o(fase, rodada, saida, tamanho, estilo), || nome_da_fase_e_rodada(fase, rodada, saida, tamanho, estilo));
    PLACAR.registrar(igual && r == n, || format!("nome_da_fase_e_rodada({}, {}) {} x {} {:x?} x {:x?}", fase as i16, rodada as i16, r, n, mo, mr));
    r
}

/// O nível da divisão do clube: a palavra +0x69 da divisão (+0x57), ou 0 sem divisão.
unsafe fn cqa3_nivel(clube: usize) -> i16 {
    let d = cqa3_d(clube + 0x57);
    if d != 0 {
        cqa3_w(d + 0x69)
    } else {
        0
    }
}

/// { 0x522360 } (thiscall, ret 8, Cup.cpp; o this não é lido; a virtual +0x90 da copa base):
/// `Cup::club_in_higher_division(a, b)`: o clube de nível menor (+0x57 → +0x69, com sinal; 0 sem divisão); empate, 0.
unsafe extern "thiscall" fn clube_da_divisao_mais_alta(_this: usize, a: usize, b: usize) -> usize {
    let (na, nb) = (cqa3_nivel(a), cqa3_nivel(b));
    if na < nb {
        a
    } else if na == nb {
        0
    } else {
        b
    }
}

/// Comparador da 0x522360: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_clube_da_divisao_mais_alta(this: usize, a: usize, b: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(1427)(this, a, b);
    let n = clube_da_divisao_mais_alta(this, a, b);
    PLACAR.registrar(r == n, || format!("clube_da_divisao_mais_alta({:x}, {:x}) original={:x} rust={:x}", a, b, r, n));
    r
}

// ---------- 263a leva: a copa pelo desfecho e no fim da temporada: os registros pelo desfecho, as honras da temporada e os semifinalistas para a UEFA (Cup.cpp) ----------

unsafe fn cqa4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3).
unsafe fn cqa4_rodada(copa: usize, r: i32) -> usize {
    cqa4_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// O clube do time `t` (+0xa7, 6 bytes por time, +0 o clube).
unsafe fn cqa4_clube(copa: usize, t: i32) -> usize {
    cqa4_d(cqa4_d(copa + 0xa7).wrapping_add((t as isize * 6) as usize))
}

/// O registro de 6 bytes do estado do clube ([0xae2d14] + id × 6; +0 a competição europeia da temporada).
unsafe fn cqa4_estado(clube: usize) -> usize {
    cqa4_d(0xae2d14).wrapping_add((cqa4_i(clube) as isize * 6) as usize)
}

/// A competição é a amistosa (0x5c6710: push comp; ecx = [0xae2c74]; sem add esp -> thiscall, ret 4).
unsafe fn cqa4_amistosa(comp: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    f(cqa4_d(0xae2c74), comp) != 0
}

/// O índice da nação dona da competição na tabela das chaves (0x669470: push comp; add esp, 4 -> cdecl; devolve o byte).
unsafe fn cqa4_indice(comp: usize) -> u8 {
    let f: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    f(comp)
}

/// A competição é a copa principal guardada da nação dela ([0xb63d7e + 72 × índice]).
unsafe fn cqa4_copa_principal(comp: usize) -> bool {
    cqa4_d(0xb63d7e_usize.wrapping_add((cqa4_indice(comp) as i8 as isize * 72) as usize)) == comp
}

/// A vaga na Copa da UEFA que o campeão da copa (`copa_registros_pelo_desfecho`, desfecho 1) ganha: a Copa da Liga
/// inglesa ([0x9cf750]) sempre; senão, a copa principal de uma nação europeia ([0x9cfa10] o continente, pela nação
/// +0x5d e o continente +0x71), não amistosa, com o clube ainda sem competição europeia (−1) e a nação com índice.
unsafe fn cqa4_vaga_na_uefa(comp: usize, clube: usize) -> bool {
    if cqa4_i(comp) == cqa4_i(0x9cf750) {
        return true;
    }
    let nacao = cqa4_d(comp + 0x5d);
    if nacao == 0 {
        return false;
    }
    let continente = cqa4_d(nacao + 0x71);
    if continente == 0 || cqa4_i(continente) != cqa4_i(0x9cfa10) || cqa4_amistosa(comp) {
        return false;
    }
    if cqa4_i(cqa4_estado(clube)) != -1 || cqa4_indice(comp) == 0xff {
        return false;
    }
    cqa4_copa_principal(comp)
}

/// Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push final; push jogo; push clube;
/// ecx = [0xdd7f00]; sem add esp -> thiscall(registros, clube, jogo, final), ret 0xc).
unsafe fn cqa4_registro_de_final(f: usize, clube: usize, jogo: usize, final_: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(cqa4_d(0xdd7f00), clube, jogo, final_);
}

/// { 0x519890 } (thiscall, ret 0x18, Cup.cpp; a virtual +0x44 das copas inglesas e da base):
/// `Cup::update_staff_records_for_status(clube, desfecho, _, _, jogo, final)`; devolve 0. Nada com o jogo +0x3a = 1,
/// com +0x2c ou com +0x44 ≠ 0xff. Desfecho 1: o registro de campeão e, com a vaga (cqa4_vaga_na_uefa), o estado do
/// clube recebe a Copa da UEFA ([0x9cf6f0]); 2: o de vice; 3: o de terceiro; 0: se a rodada seguinte (+0x34 + 1) existe
/// e não é a do terceiro lugar (+7 ≠ 0x8c), a classificação (0x86bdd0) com o peso rodada × 99 / (rodadas − 1) + 1;
/// 0xff: a eliminação (0x86c000) com o peso rodada × 99 / rodadas + 1. Os registros de fase levam os dwords que a
/// original empurra: a palavra do jogo (+0x32, +0x30) ou da rodada (+7) sobre os 16 bits altos do registrador.
unsafe extern "thiscall" fn copa_registros_pelo_desfecho(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    if cqa4_b(jogo + 0x3a) == 1 || cqa4_d(copa + 0x2c) != 0 || cqa4_b(copa + 0x44) != 0xff {
        return 0;
    }
    let comp = cqa4_d(copa + 4);
    match desfecho as u8 {
        1 => {
            crate::comparacao::ramo("cqa_4 campeao");
            cqa4_registro_de_final(0x86a800, clube, jogo, final_); // cqa4: o registro de campeão
            if cqa4_vaga_na_uefa(comp, clube) {
                core::ptr::write_unaligned(cqa4_estado(clube) as *mut i32, cqa4_i(0x9cf6f0));
            }
        }
        2 => {
            crate::comparacao::ramo("cqa_4 vice");
            cqa4_registro_de_final(0x86b0b0, clube, jogo, final_); // cqa4: o registro de vice
        }
        3 => cqa4_registro_de_final(0x86b710, clube, jogo, final_),
        0 => {
            let r = cqa4_w(jogo + 0x34);
            let n = cqa4_w(copa + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let w7 = core::ptr::read_unaligned((cqa4_rodada(copa, seguinte as i32) + 7) as *const u16);
            if w7 == 0x8c {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1428)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let fase = (jogo as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov cx, [ecx+0x32]
            let rodada = (cqa4_d(copa + 0xa3) as u32 & 0xffff_0000) | w7 as u32; // mov di, [edi+...+7]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
            f(cqa4_d(0xdd7f00), clube, comp, fase, rodada, peso);
        }
        0xff => {
            let r = cqa4_w(jogo + 0x34) as i32;
            let n = cqa4_w(copa + 0x3c) as i32;
            if n == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1428)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let (q, resto) = ((r * 99) / n, (r * 99) % n);
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32;
            let fase = (peso & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov ax, [ecx+0x32]
            let rodada = (resto as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32; // mov dx, [ecx+0x30]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
            f(cqa4_d(0xdd7f00), clube, comp, fase, rodada, peso);
        }
        _ => {}
    }
    0
}

/// Comparador da 0x519890: só a original (os registros vão ao histórico das pessoas e mexem nas reputações e no moral,
/// não podem rodar duas vezes). O plano, feito antes: o estado do clube (+0 do registro de 6 bytes) passa à Copa da
/// UEFA quando o campeão ganha a vaga; senão não muda. Sem clube: só a original.
unsafe extern "thiscall" fn comparar_copa_registros_pelo_desfecho(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1428);
    if clube == 0 || jogo == 0 {
        return o(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: sem clube ou jogo, só a original
    }
    let estado = cqa4_estado(clube);
    let antes = cqa4_i(estado);
    let previsto = if cqa4_b(jogo + 0x3a) != 1 && cqa4_d(copa + 0x2c) == 0 && cqa4_b(copa + 0x44) == 0xff && desfecho as u8 == 1
        && cqa4_vaga_na_uefa(cqa4_d(copa + 4), clube) { cqa4_i(0x9cf6f0) } else { antes };
    let r = o(copa, clube, desfecho, a3, a4, jogo, final_);
    let depois = cqa4_i(estado);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("copa_registros_pelo_desfecho(desfecho {}) estado {:x} -> {:x}, previsto {:x}, devolveu {}", desfecho as u8, antes, depois, previsto, r));
    r
}

/// Os quatro da linha do histórico: (campeão, vice, terceiro, seleção anfitriã). O campeão, o vice e o terceiro pelo
/// desfecho dos times (1, 2, 3; até achar os três); na copa galesa ([0x9cf958]) o terceiro é o primeiro time com o
/// progresso +0x73 = 3 no Comp_Stats (0x4a4850 de +8 do objeto da competição em [0xadadfc]: push clube; ecx = as
/// estatísticas -> thiscall, ret 4). A anfitriã: com a virtual +0x88 do objeto da competição (ecx = o objeto; sem push
/// -> thiscall, ret 0) e as anfitriãs do ano (+0x40) da HostCountry ([0xb5e134], 0x5fa730: push 0; push &b; push &a;
/// push ano; push id; ecx = [0xb5e134] -> thiscall, ret 0x14): duas, a seleção principal (0x53b3d0: push nação; add
/// esp, 4 -> cdecl) da de +0x8e maior (empate, a primeira); uma, a da primeira; nenhuma, 0.
unsafe fn cqa4_honras(copa: usize) -> (usize, usize, usize, usize) {
    let n = cqa4_w(copa + 0x3e);
    let (mut campeao, mut vice, mut terceiro) = (0usize, 0usize, 0usize);
    let mut i = 0i16;
    while i < n {
        let e = cqa4_d(copa + 0xa7).wrapping_add((i as isize * 6) as usize);
        match cqa4_b(e + 5) {
            1 => campeao = cqa4_d(e),
            2 => vice = cqa4_d(e),
            3 => terceiro = cqa4_d(e),
            _ => {}
        }
        if campeao != 0 && vice != 0 && terceiro != 0 {
            break;
        }
        i += 1;
    }
    let comp = cqa4_d(copa + 4);
    let objeto = || cqa4_d(cqa4_d(0xadadfc).wrapping_add((cqa4_i(cqa4_d(copa + 4)) as isize * 4) as usize));
    if comp != 0 && cqa4_i(comp) == cqa4_i(0x9cf958) {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let mut j = 0i16;
        while j < cqa4_w(copa + 0x3e) {
            let c = cqa4_clube(copa, j as i32);
            if cqa4_b(busca(cqa4_d(objeto() + 8), c) + 0x73) == 3 {
                terceiro = c;
                break;
            }
            j += 1;
        }
    }
    let o = objeto();
    let tem: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqa4_d(cqa4_d(o) + 0x88));
    let mut anfitria = 0usize;
    if tem(o) != 0 {
        let (mut a, mut b) = (0i32, 0i32);
        let achar: unsafe extern "thiscall" fn(usize, i32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
        // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x5fa730 recebe um short
        let k = achar(cqa4_d(0xb5e134), cqa4_i(cqa4_d(copa + 4)), cqa4_w(copa + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let nacao = |x: i32| cqa4_d(0xae23a8).wrapping_add((x as isize * 0x122) as usize);
        if k == 2 {
            let (na, nb) = (nacao(a), nacao(b));
            anfitria = if cqa4_w(na + 0x8e) < cqa4_w(nb + 0x8e) { selecao(nb) } else { selecao(na) };
        } else if k != 0 {
            anfitria = selecao(nacao(a));
        }
    }
    (campeao, vice, terceiro, anfitria)
}

/// { 0x519a90 } (thiscall, ret 0, Cup.cpp; a virtual +0x30 das copas inglesas e da base): `Cup::record_season_honours()`:
/// a linha do histórico (0x4afce0: 5 pushes; add esp, 0x14 -> cdecl(copa, campeão, vice, terceiro, anfitriã)) com os
/// quatro de cqa4_honras; fora da competição amistosa, na copa principal da nação (cqa4_copa_principal), os
/// semifinalistas para a UEFA (0x519fc0: ecx = a copa -> thiscall, ret 0).
unsafe extern "thiscall" fn copa_honras_da_temporada(copa: usize) {
    let (campeao, vice, terceiro, anfitria) = cqa4_honras(copa);
    let historico: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4afce0usize);
    historico(copa, campeao, vice, terceiro, anfitria);
    let comp = cqa4_d(copa + 4);
    if !cqa4_amistosa(comp) && cqa4_copa_principal(comp) {
        let semi: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x519fc0usize);
        semi(copa);
    }
}

/// Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (cpa2_linha_livre) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de cqa4_honras. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
unsafe extern "thiscall" fn comparar_copa_honras_da_temporada(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1429);
    let linha = cpa2_linha_livre();
    if linha == 0 || cqa4_d(copa + 4) == 0 {
        return o(copa); // ponytail: o histórico cresce (realloc) ou não há competição: só a original
    }
    let (campeao, vice, terceiro, anfitria) = cqa4_honras(copa);
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    o(copa);
    let lido = (cqa4_d(linha + 4), core::ptr::read_unaligned((linha + 8) as *const u16), cqa4_d(linha + 0xa), cqa4_d(linha + 0xe), cqa4_d(linha + 0x12), cqa4_d(linha + 0x16));
    let previsto = (cqa4_d(copa + 4), ano, campeao, vice, terceiro, anfitria);
    PLACAR.registrar(lido == previsto, || format!("copa_honras_da_temporada: linha {:x?} prevista {:x?}", lido, previsto));
}

/// Os dois clubes (ids) que a 0x519fc0 manda à semeadura da UEFA, ou None quando ela sai sem mandar: a rodada 0x82
/// (as semifinais) e a 0x96 (a final), as duas achadas; percorre os confrontos da 0x82 e os times que não estão no
/// primeiro confronto da 0x96: o primeiro vira a, o segundo b (aí para); um time −1 (ainda sem confronto) cancela;
/// faltando, −1.
unsafe fn cqa4_semifinalistas(copa: usize) -> Option<(i32, i32)> {
    let (mut semi, mut final_) = (-1i16, -1i16);
    let mut i = 0i16;
    while i < cqa4_w(copa + 0x3c) {
        let t = cqa4_w(cqa4_rodada(copa, i as i32) + 7);
        if t == 0x82 {
            semi = i;
        } else if t == 0x96 {
            final_ = i;
        }
        if semi != -1 && final_ != -1 {
            break;
        }
        i += 1;
    }
    if semi == -1 || final_ == -1 {
        return None;
    }
    let confrontos = |r: i16| cqa4_d(cqa4_d(copa + 0xad).wrapping_add((r as isize * 4) as usize));
    let f0 = confrontos(final_);
    let (fa, fb) = (cqa4_w(f0), cqa4_w(f0 + 2));
    let n = cqa4_w(cqa4_rodada(copa, semi as i32) + 0x1a);
    let base = confrontos(semi);
    let (mut a, mut b) = (-1i32, -1i32);
    let mut k = 0i16;
    'lacos: while k < n {
        let t = base.wrapping_add((k as isize * 0x35) as usize);
        for lado in [0usize, 2] {
            let x = cqa4_w(t + lado);
            if x == fa || x == fb {
                continue;
            }
            if x == -1 {
                return None;
            }
            let id = cqa4_i(cqa4_clube(copa, x as i32));
            if a == -1 {
                a = id;
            } else {
                b = id;
                break 'lacos;
            }
        }
        k += 1;
    }
    Some((a, b))
}

/// { 0x519fc0 } (thiscall, ret 0, Cup.cpp): `Cup::register_semi_final_losers_for_uefa()`: os semifinalistas fora da
/// final (cqa4_semifinalistas) vão à semeadura da UEFA (0x905790: push b; push a; push id da competição; ecx =
/// [0xde1f58]; sem add esp -> thiscall, ret 0xc).
unsafe extern "thiscall" fn copa_semifinalistas_para_a_uefa(copa: usize) {
    if let Some((a, b)) = cqa4_semifinalistas(copa) {
        let f: unsafe extern "thiscall" fn(usize, i32, i32, i32) = core::mem::transmute(0x905790usize);
        f(cqa4_d(0xde1f58), cqa4_i(cqa4_d(copa + 4)), a, b);
    }
}

/// Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
unsafe extern "thiscall" fn comparar_copa_semifinalistas_para_a_uefa(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1430);
    let s = cqa4_d(0xde1f58);
    if s == 0 {
        return o(copa); // ponytail: sem a semeadura, só a original
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(s + 0x440, 50 * 12)], || o(copa), || copa_semifinalistas_para_a_uefa(copa));
    PLACAR.registrar(igual, || format!("copa_semifinalistas_para_a_uefa(copa {:x}) a semeadura difere ({:?})", copa, cqa4_semifinalistas(copa)));
}

// ---------- 263a leva: a vaga no elenco para a oferta (o limite de 50 jogadores e das vagas da comissão, com a liberação do mais barato), a oferta de empréstimo recusada pelo clube e a cláusula de percentual da próxima venda achada (transfer_manager.cpp) ----------

unsafe fn cqa5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x8bfa30 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_loan_offer(vaga, marcar, motivo)`,
/// pelo vendedor do computador que recusa todo lance de empréstimo (0x8c8090, oferta com +0x1f = 1) e pela oferta
/// agendada (0x8c63a0). Com `marcar`, +0x2d = 1; estado 2 (recusada) já de cara; o contrato da pessoa vai para
/// [0xae1080] e dá o clube do contrato; os lances do comprador pelo jogador e os do clube do contrato pelos da troca
/// saem (0x84aeb0); a notícia (0x8d5170); a oferta volta em 10 dias (0x8c6220) para sair; sem rival em aberto, a marca
/// de oferta da pessoa sai (+0x86e); as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a 0x4ccb30 de cada jogador
/// da troca, cuja marca +4 zera), com +0x10 = 0 entre elas; por fim a situação na lista de observação do comprador
/// (0x849ce0; achada, 0x843e10 com evento 1 e tipo 0). É a recusa da transferência (0x8bf680) sem o desassossego, sem
/// a transferência futura, sem fechar as ofertas de contrato ligadas e sem a parte dos jogadores da troca no fim: os
/// ajudantes `cpx1_*` (com o ensaio CPX1_ENSAIO) servem igual.
unsafe extern "thiscall" fn recusar_oferta_de_emprestimo(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    if marcar != 0 {
        *((o + 0x2d) as *mut u8) = 1;
    }
    *((o + 0x2c) as *mut u8) = 2;
    crate::comparacao::ramo("cqa_5 emprestimo recusado");
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    cpx1_desmarcar_lances(this, o, vendedor);
    cpx1_noticia(this, o, motivo);
    cpx1_agendar(this, o, 0xa);
    cpx1_tirar_marca(this, o);
    // vendedor 0: sem a parte dos jogadores da troca na lista do comprador, que esta rotina não tem
    cpx1_ofertas_de_contrato(this, o, 0, 0);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let comprador = cpx1_clube(cpx1_d(o + 8));
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    if entrada(cpx1_listas(this), comprador, p, (cpx1_b(o + 0x11) != 0xb) as u32, 0) != 0 {
        cpx1_situacao_na_lista(this, p, comprador, 1, 0);
    }
}

/// Comparador da 0x8bfa30: o da recusa e da retirada (`cpx1_comparar`: a cópia em ensaio e a original do mesmo estado;
/// iguais o estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa).
unsafe extern "thiscall" fn comparar_recusar_oferta_de_emprestimo(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(1431);
    cpx1_comparar("recusar_oferta_de_emprestimo", this, vaga, || o(this, vaga, marcar, motivo), || recusar_oferta_de_emprestimo(this, vaga, marcar, motivo))
}

/// O ensaio do comparador da 0x8cd450: a cópia roda sem liberar o jogador escolhido (0x86d800 e 0x8c1af0 mexem no
/// resto do jogo) e guarda quem seria em CQA5_LIBERADO.
static mut CQA5_ENSAIO: bool = false;
static mut CQA5_LIBERADO: usize = 0;

/// Os tipos 0x53b620 (11 a 15: jogador) e 0x53b610 (não 11), cdecl com o byte.
unsafe fn cqa5_jogador(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    f(t as i8) != 0
}

unsafe fn cqa5_nao_11(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b610usize);
    f(t as i8) != 0
}

/// As quatro contas de vagas da comissão pelo tipo da pessoa: [0] treinadores (8 e 0xf, +0x19f), [1] olheiros (9,
/// +0x1b3), [2] fisioterapeutas (0xa, +0x1cf), [3] auxiliar (6 e 0xd, +0xd3).
unsafe fn cqa5_comissao(t: u8) -> Option<usize> {
    match t {
        8 | 0xf => Some(0),
        9 => Some(1),
        0xa => Some(2),
        6 | 0xd => Some(3),
        _ => None,
    }
}

/// O id da pessoa no vetor (dword com sinal) abaixo de [0xae2364]: a pessoa ainda existe.
unsafe fn cqa5_viva(p: usize) -> bool {
    p != 0 && (cqa5_d(p) as i32) < cqa5_d(0xae2364) as i32
}

/// A falha com o motivo: (clube ≠ comprador) + `base` na palavra do motivo, se houver; 0.
unsafe fn cqa5_falha(clube: usize, o: usize, motivo: *mut u16, base: u16) -> u32 {
    if !motivo.is_null() {
        *motivo = (cqa5_d(clube) != cqa5_d(o + 8)) as u16 + base;
    }
    0
}

/// A pessoa está na fila da transferência livre ([0xde1f54] + 0x92c, LLIST de ids; o cursor +0xa volta a +2 e a busca
/// para no achado, como na original).
unsafe fn cqa5_na_fila_livre(p: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cqa5_d(0xde1f54) + 0x92c;
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqa5_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if cqa5_d(p) == cqa5_d(e) {
            return true;
        }
        e = proximo(l);
    }
    false
}

/// { 0x8cd450 } (thiscall, ret 0x10, transfer_manager.cpp; o mapa a dá como stdcall, mas os dois pontos de chamada na
/// 0x8cfc90 põem a gerência em ecx e ela o usa para a 0x8c1780 e a 0x8c1af0): `TRANSFER_MANAGER::check_squad_limits
/// (clube, oferta, contar_fila, motivo)`, das restrições da oferta (0x8cfc90), pelo comprador e pelo vendedor. Clube
/// nulo: a caixa da linha 0x2c3b e 0. O parceiro do clube (0x540a50 com 0 e 1). Pelo comprador (+8 da oferta = id do
/// clube): a pessoa da oferta já é dele (contrato de tipo 5, de novo pela 0x4cf5a0, com o clube; ou a ficha de
/// empréstimo 0x4cf6d0 com +4 = o clube) conta −1; jogador pelo tipo +0x11 da oferta conta +1; o tipo da comissão
/// pelo mesmo byte marca 1; cada jogador da troca (+4, LLIST de ids) que sai desconta (jogador −1, comissão −1 pelo
/// +0x3d da pessoa). Pelo vendedor: sem troca, 1; o jogador vendido conta −1 e o tipo da comissão −1 (pelo +0x3d); cada
/// nó da troca que chega soma pelo byte +6. Conta positiva: soma os 50 do elenco (+0xd7, ponteiros com id vivo) do clube
/// e do parceiro; acima de 50, menos a fila livre do clube (0x8c1780) se `contar_fila`; ainda acima de 50: fora do
/// comprador do computador sem troca, o motivo (clube ≠ comprador) + 4 e 0; senão o mais barato do elenco ((+0x3d do
/// contrato + 100) × valor +0x52, com +0x3d ≤ 0, sem ficha, sem contrato de tipo 5 em [0xae1080], sem ofertas 0x8c5cf0,
/// trocas 0x8c5e40 ou transferência futura 0x8cca10, fora da fila livre e de tipo 0xb, 0xf ou 0xd) sai de graça
/// (0x86d800 em [0xdd7f00], 0x8c1af0 com −1); nenhum, o motivo + 4 e 0. Depois as vagas da comissão marcadas: 5
/// treinadores (+0x19f), 3 fisioterapeutas (+0x1cf), 7 olheiros (+0x1b3) — os do clube contam por ponteiro, os do
/// parceiro só com id vivo — e o auxiliar (+0xd3 do clube ou do parceiro ocupado); passou do limite, o motivo + 0x22 e 0.
/// Devolve 1.
unsafe extern "thiscall" fn vaga_no_elenco_para_a_oferta(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    if clube == 0 {
        aviso_em(0xa8d514, 0x2c3b);
        return 0;
    }
    let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let parceiro = parceiro_de(clube, core::ptr::null_mut(), 1);
    let mut n: i8 = 0;
    let mut com = [0i8; 4];
    if cqa5_d(clube) == cqa5_d(o + 8) {
        let p = cpx1_pessoa(cqa5_d(o + 0xc));
        let c = cpx1_contrato(p);
        let mut proprio = false;
        if c != 0 && cqa5_b(c + 0x35) & 0x3f == 5 {
            proprio = cpx1_clube_do_contrato(p) == clube;
        }
        if !proprio && ficha(cqa5_d(0xae1050), p) != 0 && cqa5_d(ficha(cqa5_d(0xae1050), p) + 4) == cqa5_d(clube) {
            proprio = true;
        }
        if proprio {
            n = -1;
        }
        let t = cqa5_b(o + 0x11);
        if cqa5_jogador(t) {
            n = n.wrapping_add(1); // cqa5: o jogador da oferta conta
        }
        if cqa5_nao_11(t) {
            if let Some(k) = cqa5_comissao(t) {
                com[k] = 1;
            }
        }
        let l = cqa5_d(o + 4);
        if l != 0 {
            for e in cpx1_itens(l) {
                let t = cqa5_b(cpx1_pessoa(cqa5_d(e)) + 0x3d);
                if cqa5_jogador(t) {
                    n = n.wrapping_sub(1);
                }
                if cqa5_nao_11(t) {
                    if let Some(k) = cqa5_comissao(t) {
                        com[k] = com[k].wrapping_sub(1);
                    }
                }
            }
        }
    } else {
        if cqa5_d(o + 4) == 0 {
            crate::comparacao::ramo("cqa_5 vendedor sem troca");
            return 1; // cqa5: o vendedor sem troca tem vaga
        }
        let t = cqa5_b(cpx1_pessoa(cqa5_d(o + 0xc)) + 0x3d);
        if cqa5_jogador(t) {
            n = -1;
        }
        if cqa5_nao_11(t) {
            if let Some(k) = cqa5_comissao(t) {
                com[k] = -1;
            }
        }
        for e in cpx1_itens(cqa5_d(o + 4)) {
            let t = cqa5_b(e + 6);
            if cqa5_jogador(t) {
                n = n.wrapping_add(1);
            }
            if cqa5_nao_11(t) {
                if let Some(k) = cqa5_comissao(t) {
                    com[k] = com[k].wrapping_add(1);
                }
            }
        }
    }
    if n > 0 {
        let contar = |base: usize, n: &mut i8| {
            for k in 0..0x32 {
                if cqa5_viva(cqa5_d(base + 0xd7 + 4 * k)) {
                    *n = n.wrapping_add(1); // cqa5: cada um do elenco conta
                }
            }
        };
        crate::comparacao::ramo("cqa_5 conta o elenco");
        contar(clube, &mut n);
        if parceiro != 0 {
            contar(parceiro, &mut n);
        }
        if n > 0x32 { // cqa5: o limite de 50 no elenco
            if contar_fila != 0 {
                let fila: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x8c1780usize);
                n = n.wrapping_sub(fila(this, clube) as i8);
            }
            if n > 0x32 {
                let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
                if cqa5_d(clube) != cqa5_d(o + 8) || humano(clube, 1, 1, 0, 0) != 0 || cqa5_d(o + 4) != 0 {
                    crate::comparacao::ramo("cqa_5 elenco cheio");
                    return cqa5_falha(clube, o, motivo, 4);
                }
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
                let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
                let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
                let mut melhor = 1000000.0f64;
                let mut escolhido = 0usize;
                for k in 0..0x32 {
                    let p = cqa5_d(clube + 0xd7 + 4 * k);
                    if !cqa5_viva(p) || ficha(cqa5_d(0xae1050), p) != 0 {
                        continue;
                    }
                    // a primeira consulta do contrato não vai para [0xae1080]
                    let c = contrato(cqa5_d(0xae1050), p);
                    if c == 0 {
                        continue;
                    }
                    let s = cqa5_b(c + 0x3d) as i8;
                    let v = (s as f64 + 100.0) * (cqa5_d(p + 0x52) as i32 as f64);
                    if !(v < melhor) || s > 0 {
                        continue;
                    }
                    let c = cpx1_contrato(p);
                    if c != 0 && cqa5_b(c + 0x35) & 0x3f == 5 {
                        continue;
                    }
                    if ficha(cqa5_d(0xae1050), p) != 0 {
                        continue;
                    }
                    let g = cqa5_d(0xde1f54);
                    if ofertas(g, p, 0, 0) != 0 || trocas(g, p, 0) != 0 || futura(g, p, 0) != 0 {
                        continue;
                    }
                    if cqa5_na_fila_livre(p) {
                        continue;
                    }
                    if matches!(cqa5_b(p + 0x3d), 0xb | 0xf | 0xd) {
                        escolhido = p;
                        melhor = v;
                    }
                }
                if escolhido == 0 {
                    return cqa5_falha(clube, o, motivo, 4);
                }
                crate::comparacao::ramo("cqa_5 libera o mais barato");
                if CQA5_ENSAIO {
                    CQA5_LIBERADO = escolhido;
                } else {
                    let registrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86d800usize);
                    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) = core::mem::transmute(0x8c1af0usize);
                    registrar(cqa5_d(0xdd7f00), escolhido);
                    liberar(this, escolhido, clube, 0, -1, 0);
                }
            }
        }
    }
    // as vagas da comissão: (conta, deslocamento, vagas)
    for (k, base, vagas) in [(0usize, 0x19fusize, 5usize), (2, 0x1cf, 3), (1, 0x1b3, 7)] {
        let mut m = com[k];
        if m <= 0 {
            continue;
        }
        for j in 0..vagas {
            if cqa5_d(clube + base + 4 * j) != 0 {
                m = m.wrapping_add(1);
            }
        }
        if parceiro != 0 {
            for j in 0..vagas {
                if cqa5_viva(cqa5_d(parceiro + base + 4 * j)) {
                    m = m.wrapping_add(1);
                }
            }
        }
        if m > vagas as i8 {
            return cqa5_falha(clube, o, motivo, 0x22);
        }
    }
    if com[3] > 0 && (cqa5_d(clube + 0xd3) != 0 || (parceiro != 0 && cqa5_d(parceiro + 0xd3) != 0)) {
        return cqa5_falha(clube, o, motivo, 0x22);
    }
    1
}

/// Comparador da 0x8cd450: a cópia em ensaio (sem liberar) e depois a original, do mesmo estado ([0xae1080], o motivo,
/// os cursores da fila livre e da troca). Sem liberação: iguais a devolução e essas regiões. Com liberação (a cópia
/// guardou o escolhido): iguais a devolução e o escolhido ter saído do clube (o contrato dele, pela 0x4cf5a0 que só
/// lê, sem o clube); o resto a original refaz ao liberar. Clube nulo: a caixa, só a original.
unsafe extern "thiscall" fn comparar_vaga_no_elenco_para_a_oferta(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32>(1432);
    if clube == 0 {
        return f(this, clube, o, contar_fila, motivo);
    }
    let mut regioes = vec![(0xae1080usize, 4usize), (cqa5_d(0xde1f54) + 0x936, 4)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    if cqa5_d(o + 4) != 0 {
        regioes.push((cqa5_d(o + 4) + 0xa, 4));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQA5_ENSAIO = true;
    CQA5_LIBERADO = 0;
    let n = crate::aleatorio::pausar_rastro(|| vaga_no_elenco_para_a_oferta(this, clube, o, contar_fila, motivo));
    CQA5_ENSAIO = false;
    let liberado = CQA5_LIBERADO;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = f(this, clube, o, contar_fila, motivo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if liberado == 0 {
        mo.0 == mr.0
    } else {
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato(cqa5_d(0xae1050), liberado);
        c == 0 || cpx1_clube(cqa5_d(c + 4)) != clube
    };
    PLACAR.registrar(igual && r == n, || {
        format!("vaga_no_elenco_para_a_oferta(clube {:x}, oferta {:x}, fila {}) original={} rust={} liberado={:x} {:x?} x {:x?}", clube, o, contar_fila, r, n, liberado, mo, mr)
    });
    r
}

/// { 0x8d0dd0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_sale_percentage(pessoa)`, das
/// telas da pessoa (0x884ea2): sem a marca 8 da pessoa ([this+0x86e] + id × 8), 0. O contrato dela (0x4cf5a0, em
/// [0xae1080]) e o clube; sem os dois, 0. A busca binária (0x946464: cdecl(chave, vetor [this+0x880], conta
/// [this+0x888], 0xd, 0x8e54d0)) pela chave com o id do clube (+1, de novo pelo contrato) e o da pessoa (+9), os dois
/// únicos campos que a comparação lê; achada, devolve o registro; senão a marca 8 sai e 0.
unsafe extern "thiscall" fn percentual_da_proxima_venda_achado(this: usize, p: usize) -> usize {
    let m = (cqa5_d(this + 0x86e).wrapping_add(cqa5_d(p).wrapping_mul(8))) as *mut u8;
    if *m & 8 == 0 {
        return 0;
    }
    if cpx1_clube_do_contrato(p) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0x10];
    chave[9..13].copy_from_slice(&(cqa5_d(p) as u32).to_le_bytes());
    // ponytail: a original lê o id do clube sem conferir o contrato (acabou de achá-lo); sem ele, aqui 0
    let c = cpx1_clube_do_contrato(p);
    let id = if c != 0 { cqa5_d(c) as u32 } else { 0 };
    chave[1..5].copy_from_slice(&id.to_le_bytes());
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(chave.as_ptr(), cqa5_d(this + 0x880), cqa5_d(this + 0x888), 0xd, 0x8e54d0);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(8);
    0
}

/// Comparador da 0x8d0dd0: lado a lado na marca da pessoa, em [0xae1080] e na devolução (só lê o resto).
unsafe extern "thiscall" fn comparar_percentual_da_proxima_venda_achado(this: usize, p: usize) -> usize {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1433);
    let m = cqa5_d(this + 0x86e).wrapping_add(cqa5_d(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1), (0xae1080, 4)], || f(this, p), || percentual_da_proxima_venda_achado(this, p));
    PLACAR.registrar(igual && r == n, || format!("percentual_da_proxima_venda_achado(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

// ---------- 263a leva: a lista de observação do técnico humano: o jogador na busca, as listas dos olheiros acertadas com os olheiros do clube e a vaga do jogador no registro do clube observado (player_search.cpp, scout_manager.cpp) ----------

unsafe fn cqa6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x7af600 } (thiscall, ret 4, player_search.cpp, quadro SEH só pelas duas datas locais, sem destrutor):
/// `PLAYER_SEARCH::contains_player(pessoa)`, da lista do olheiro do técnico humano (0x7f7490, que a avaliação dos
/// jogadores e os clubes interessados consultam a cada dia) e de telas. A chave local de 0x2d bytes começa com as duas
/// datas padrão (+0x10 e +0x18, 0x54a110); pessoa nula: a caixa da linha 0x33e e 0. Sem resultados (+0x1f6 = conta):
/// 0. A referência ao nome da pessoa na chave (0x7e4920: +0xc o id, o único campo que a comparação 0x7bce50 lê) e a
/// busca binária nos resultados (0x946464 com o vetor +0x1ee, a conta, 0x2d e 0x7bce50); achada e sem a marca de
/// removida (bit 0 de +0x20), 1; senão 0.
unsafe extern "thiscall" fn jogador_na_busca(this: usize, pessoa: usize) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut chave = [0u8; 0x30];
    let k = chave.as_mut_ptr() as usize;
    data_padrao(k + 0x10);
    data_padrao(k + 0x18);
    if pessoa == 0 {
        aviso_em(0xa701b0, 0x33e);
        return 0;
    }
    if cqa6_d(this + 0x1f6) == 0 {
        crate::comparacao::ramo("cqa_6 busca vazia");
        return 0; // cqa6: sem resultados, não está
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    referencia(pessoa, k);
    let bsearch: unsafe extern "C" fn(usize, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(k, cqa6_d(this + 0x1ee), cqa6_d(this + 0x1f6), 0x2d, 0x7bce50);
    if r == 0 {
        return 0;
    }
    if cqa6_b(r + 0x20) & 1 != 0 { // cqa6: a removida não conta
        crate::comparacao::ramo("cqa_6 removido");
        return 0;
    }
    crate::comparacao::ramo("cqa_6 achado");
    1 // cqa6: achado
}

/// Comparador da 0x7af600: pela devolução (só lê; as datas e a chave são locais). Pessoa nula: a caixa, só a original.
unsafe extern "thiscall" fn comparar_jogador_na_busca(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1434);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = o(this, pessoa);
    let n = jogador_na_busca(this, pessoa);
    PLACAR.registrar(r == n, || format!("jogador_na_busca(busca {:x}, pessoa {:x}) original={} rust={}", this, pessoa, r, n));
    r
}

/// A busca `i` (0..9) do técnico `m` (0x7f6cd0: thiscall(gerência, i, técnico), ret 8; lê só o byte do índice).
unsafe fn cqa6_busca(this: usize, i: u8, m: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    f(this, i as u32, m)
}

/// O olheiro da busca (0x7ae950: thiscall(busca)).
unsafe fn cqa6_olheiro(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7ae950usize);
    f(l)
}

/// Põe o olheiro na busca (0x7ae8a0: thiscall(busca, olheiro), ret 4).
unsafe fn cqa6_trocar(l: usize, olheiro: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7ae8a0usize);
    f(l, olheiro);
}

/// { 0x7f7270 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::sync_scout_lists(técnico)`, quando a comissão do
/// clube humano muda (a saída do cargo 0x690890 e a da vaga dos olheiros, as transferências e contratações de pessoal).
/// Sem técnico, o humano da vez; sem ele, a caixa da linha 0x1ba e 0. Das 10 buscas do técnico, as listas de olheiro
/// (+0xa = 4) cujo olheiro (0x7ae950) não é mais olheiro (+0x3d = 9) do clube do técnico (+0x39) ficam sem olheiro
/// (0x7ae8a0 com 0; sem clube, todas; com clube, as sem olheiro ficam como estão). Com clube, cada um dos 7 olheiros do
/// clube (0x53cd60) que não tem lista vai para a primeira lista de olheiro sem olheiro. Depois a vaga do técnico (byte
/// +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x212 e 0): com a edição aberta ([this + vaga × 4 +
/// 0xc]), ela refaz os olheiros das listas (0x8043c0). Devolve 1.
unsafe extern "thiscall" fn sincronizar_listas_dos_olheiros(this: usize, tecnico: usize) -> u32 {
    let m = if tecnico != 0 { tecnico } else { cps8_gerente_da_vez() };
    if m == 0 {
        aviso_em(0xa7b094, 0x1ba);
        return 0;
    }
    let clube = cqa6_d(m + 0x39);
    for i in 0..10u8 {
        let l = cqa6_busca(this, i, m);
        if l == 0 || cqa6_b(l + 0xa) != 4 {
            continue;
        }
        let o = cqa6_olheiro(l);
        if clube != 0 {
            if o == 0 || (cqa6_d(o + 0x39) == clube && cqa6_b(o + 0x3d) == 9) {
                continue;
            }
        }
        crate::comparacao::ramo("cqa_6 lista sem olheiro");
        cqa6_trocar(l, 0);
    }
    if clube != 0 {
        let olheiros: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x53cd60usize);
        let mut v = [0usize; 7];
        olheiros(clube, v.as_mut_ptr());
        for &s in v.iter() {
            if s == 0 {
                continue;
            }
            let mut livre: i8 = -1;
            let mut tem = false;
            for i in 0..10u8 {
                let l = cqa6_busca(this, i, m);
                if l == 0 || cqa6_b(l + 0xa) != 4 {
                    continue;
                }
                if cqa6_olheiro(l) == s {
                    tem = true;
                    break;
                }
                if cqa6_olheiro(l) == 0 && livre == -1 {
                    livre = i as i8;
                }
            }
            if !tem && livre > -1 {
                crate::comparacao::ramo("cqa_6 olheiro na lista livre");
                cqa6_trocar(cqa6_busca(this, livre as u8, m), s);
            }
        }
    }
    let vaga = cqa6_b(m).wrapping_sub(cqa6_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        aviso_em(0xa7b094, 0x212);
        return 0;
    }
    let s = cqa6_d(this + vaga as usize * 4 + 0xc);
    if s != 0 {
        let refazer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8043c0usize);
        refazer(s);
    }
    1
}

/// Comparador da 0x7f7270: lado a lado nas 10 buscas do técnico (0x2b0 bytes cada, achadas antes pela 0x7f6cd0, que
/// só lê) e na devolução. Sem técnico ou com a edição aberta (a 0x8043c0 mexe na edição): só a original.
unsafe extern "thiscall" fn comparar_sincronizar_listas_dos_olheiros(this: usize, tecnico: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1435);
    let m = if tecnico != 0 { tecnico } else { cps8_gerente_da_vez() };
    if m == 0 {
        return o(this, tecnico);
    }
    let vaga = cqa6_b(m).wrapping_sub(cqa6_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) || cqa6_d(this + vaga as usize * 4 + 0xc) != 0 {
        return o(this, tecnico);
    }
    let mut regioes = Vec::new();
    for i in 0..10u8 {
        let l = cqa6_busca(this, i, m);
        if l != 0 {
            regioes.push((l, 0x2b0usize));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico), || sincronizar_listas_dos_olheiros(this, tecnico));
    PLACAR.registrar(igual && r == n, || format!("sincronizar_listas_dos_olheiros(técnico {:x}) original={} rust={} {:x?} x {:x?}", m, r, n, mo, mr));
    r
}

/// { 0x7bd700 } (thiscall, ret 8, player_search.cpp): `player_search_find_or_claim_player_slot(pessoa, reservar)`, do
/// registro de 50 vagas (0x1a2 bytes a partir de +4: +0 o id, −1 livre) do clube observado, pela partida dele
/// (0x7bd310) e pelas telas (0x7f8610). A vaga com o id da pessoa; senão, com `reservar`, a primeira livre recebe o id
/// (pessoa nula: −1), zera a palavra +4 e os bytes +0x1a0 e +0x1a1 e é devolvida (nenhuma livre: a caixa da linha
/// 0x210d e 0); sem `reservar`, 0.
unsafe extern "thiscall" fn vaga_do_jogador_observado(this: usize, pessoa: usize, reservar: u32) -> usize {
    if pessoa == 0 {
        // ponytail: a original lê o id da pessoa antes de conferir o ponteiro (cairia); fica com ela
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1436)(this, pessoa, reservar);
    }
    let id = cqa6_d(pessoa) as i32;
    let vaga = |i: i8| this + 4 + i as usize * 0x1a2;
    let mut livre: i8 = -1;
    for i in 0..0x32i8 {
        let v = cqa6_d(vaga(i)) as i32;
        if v == id {
            return vaga(i);
        }
        if reservar != 0 && livre == -1 && v == -1 {
            livre = i;
        }
    }
    if reservar == 0 {
        return 0;
    }
    if livre == -1 {
        aviso_em(0xa701b0, 0x210d);
        return 0;
    }
    let a = vaga(livre);
    core::ptr::write_unaligned(a as *mut i32, id);
    core::ptr::write_unaligned((a + 4) as *mut u16, 0);
    *((a + 0x1a0) as *mut u8) = 0;
    *((a + 0x1a1) as *mut u8) = 0;
    a
}

/// Comparador da 0x7bd700: sem `reservar`, pela devolução (só lê); com ele, lado a lado nas 50 vagas e na devolução.
unsafe extern "thiscall" fn comparar_vaga_do_jogador_observado(this: usize, pessoa: usize, reservar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1436);
    if pessoa == 0 {
        return o(this, pessoa, reservar);
    }
    if reservar == 0 {
        let r = o(this, pessoa, reservar);
        let n = vaga_do_jogador_observado(this, pessoa, reservar);
        PLACAR.registrar(r == n, || format!("vaga_do_jogador_observado(registro {:x}, pessoa {:x}) original={:x} rust={:x}", this, pessoa, r, n));
        return r;
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this + 4, 0x32 * 0x1a2)], || o(this, pessoa, reservar), || vaga_do_jogador_observado(this, pessoa, reservar));
    PLACAR.registrar(igual && r == n, || format!("vaga_do_jogador_observado(registro {:x}, pessoa {:x}, reservar) original={:x} rust={:x} iguais={}", this, pessoa, r, n, igual));
    r
}

// ---------- 263a leva: o valor do jogador para o clube (a nota pela vaga da lista de observação e o preço que o clube pagaria), a entrada do jogador na busca do clube observado e o prêmio por jogos na seleção achado (shortlist_manager.cpp, scout_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cqa7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqa7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A constante double do exe em `a` (lida do próprio exe para ser a mesma, bit a bit, que o x87 da original carrega).
unsafe fn cqa7_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// O fim sem valor da 0x840d80 (0x841f62): fora da lista de empréstimos da vaga (bit 0 de +0x24), 0; com ela e a pessoa
/// listada para empréstimo (0x544590 com 0), a nota vira max(nota + 5000, nota × 1,2) (_ftol) e 0.
unsafe fn cqa7_sem_valor(p: usize, vaga: usize, saida: *mut i32) -> i32 {
    if cqa7_b(vaga + 0x24) & 1 == 0 {
        return 0;
    }
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    if emprestimo(p, 0) != 0 {
        let r = *saida;
        let a = r.wrapping_add(5000) as f64;
        let q = r as f64 * cqa7_k(0x968978);
        *saida = ftol(if a > q { a } else { q });
    }
    0
}

/// O ajuste pela habilidade vista dos jogadores jovens (as vagas de tipo 1, 2 e 3 em +0x24 >> 1): com menos de 27 anos
/// (+0x18 da pessoa) e nota acima de 20000, a habilidade atual vista pelo técnico do clube (0x6e86d0 com 1 e nota/4)
/// × 220 / nota, limitada a `frac` + 0,6 e puxada para 1 aos 24, 25 e 26 anos; com 1,05 × mínimo abaixo da nota ou a
/// vaga de nota alta (`alta`), acima de 1 ela ainda vai pela reputação do clube na tabela [this+0xdc] (9 bytes por
/// clube, palavra +5) sobre a reputação do clube (+0x80, pelo menos 1000), entre 1 e 1,2. Acima de 1: `frac` × ela, e o
/// valor base e a nota × (ela + 1) / 2.
unsafe fn cqa7_jovem(this: usize, clube: usize, p: usize, saida: *mut i32, minimo: i32, alta: bool, idade: i8, r: i32, frac: &mut f64, e: &mut u32) {
    if idade >= 0x1b || r <= 20000 {
        return;
    }
    let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
    let sv = vista(cqa7_d(clube + 0xcf), p, clube, 1, (r / 4) as u32) as i32;
    let o = *saida as f64;
    let mut d1 = sv as f64 * cqa7_k(0x968f48) / o;
    let teto = *frac + cqa7_k(0x968e48);
    if !(d1 < teto) {
        d1 = teto;
    }
    match idade {
        0x18 => d1 = d1 * cqa7_k(0x968e40) + cqa7_k(0x968e80),
        0x19 => d1 = (d1 + cqa7_k(0x967890)) * cqa7_k(0x968968),
        0x1a => d1 = d1 * cqa7_k(0x968e80) + cqa7_k(0x968e40),
        _ => {}
    }
    if minimo as f64 * cqa7_k(0x9689b8) < o || alta {
        if !(d1 > cqa7_k(0x967890)) {
            return;
        }
        let a = cqa7_w(clube + 0x80);
        let ra = if a < 1000 { 1000 } else { a as i32 };
        let nr = cqa7_w(cqa7_d(this + 0xdc).wrapping_add(cqa7_d(clube).wrapping_mul(9)) + 5) as i32;
        let mut x = nr as f64 / ra as f64;
        if !(cqa7_k(0x967890) <= x) {
            x = cqa7_k(0x967890);
        } else if cqa7_k(0x968978) < x {
            x = cqa7_k(0x968978);
        }
        d1 = d1 * x;
    }
    if d1 > cqa7_k(0x967890) {
        crate::comparacao::ramo("cqa_7 jovem valorizado");
        *frac = d1 * *frac;
        let h = (d1 + cqa7_k(0x967890)) * cqa7_k(0x968968);
        *e = ftol(*e as f64 * h) as u32;
        *saida = ftol(o * h);
    }
}

/// { 0x840d80 } (thiscall, ret 0x18, shortlist_manager.cpp, 4,6 KB de x87; o mapa dá 1332 bytes):
/// `Shortlist_Manager::value_player_for_club(clube, pessoa, vaga, saida, mínimo, teto)`, das listas de observação e das
/// ofertas do computador (0x83f3a0, 0x842e10, 0x843800, 0x8c0810, 0x8c8090, 0x8c8860, 0x8e5d60). O observador é o
/// técnico do clube (+0xcf; sem a ficha +0x69, uma sorteada, 0x90cfc0 sobre [0xae2368], em [0xae23c0] + 0x44 × i). As
/// marcas da nota pelo tipo da vaga (+0x24 & 0x1e: 4 e 6 com 0x5882, ou 0x815882 para o próprio jogador do clube
/// humano, e o escopo da reputação 0x5408c0; 8 com 0x5982 e o escopo; senão 0x5902); a vaga da lista de empréstimos (bit
/// 0) muda o divisor do mínimo para 16 ou 6 (0x544590). Clube humano (0x600a50 com 1, 1): o técnico dele (0x53ca90) e,
/// com o jogador na lista do olheiro (0x7f7490), a nota (0x6e6340 com −1 e o substituto 0x600e90) vira max(nota +
/// 15000, nota × 1,5); senão a nota com o mínimo² × 0,000025 / divisor. Computador: com a marca 0x100 (ou 0x80) o melhor
/// observador entre o técnico e os 7 olheiros (+0x1b3) pelo conhecimento (0x544410 com 6 ou 5) menos 5 ou 10 pelo
/// escopo; a nota com o mínimo e esse observador, × 1,05 para o listado para transferência de outro clube (0x544520).
/// Nota negativa ou 10000 abaixo do mínimo, ou vaga da lista de empréstimos: o fim sem valor (`cqa7_sem_valor`). A névoa
/// (+0xb do registro de 79 bytes em [0xae2d0c], 4 bits) limita a nota a nota + 5000 por (1 + n²/100) × nota. Pela
/// posição (0x83bd00) o teto do preço e a fração; o valor base (alto com nota ≥ 35000, +0xb da vaga ≥ nota/8 e +1 ≥
/// 0x80); o ajuste pelo tipo da vaga (1, 2, 3: os jovens, `cqa7_jovem`; 3, 4, 5, 6: frações); a idade acima do divisor
/// (28, 32 sem posição, 19 na vaga 5) e o estrangeiro (0x53d980); o preço pela consistência (+0x2d da ficha +0x61) e o
/// teto pelo tipo da vaga, pela reputação, pela ficha (+0xd) e pela idade; acima de 1,5 milhão, a habilidade potencial
/// vista (0x6e86d0 com 2) abaixo de 145 puxa para 1,5 milhão; o limite da divisão do clube (+0x57, +0x69 < 16); a nação
/// do clube (+0x53) [0x9cf434] ou [0x9cf300]; com `teto`, no máximo +7 da vaga (pelo menos 1). Devolve o preço (_ftol).
unsafe extern "thiscall" fn valor_do_jogador_para_o_clube(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x5408c0usize);
    let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    let mut obs = cqa7_d(clube + 0xcf);
    let t7 = cqa7_d(vaga + 7) as i32;
    let teto_i = if t7 > 1 { t7 } else { 1 };
    let mut div: i8 = 4;
    if obs != 0 && cqa7_d(obs + 0x69) == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let i = sorteio(cqa7_d(0xae2368) as i32);
        core::ptr::write_unaligned((obs + 0x69) as *mut usize, cqa7_d(0xae23c0).wrapping_add((i as usize).wrapping_mul(0x44)));
    }
    // ponytail: a original empurra a palavra da posição com a palavra alta de ecx (lixo); quem recebe lê só a palavra
    let pos = cqa7_w(vaga + 1) as u16 as u32;
    let tipo = cqa7_b(vaga + 0x24) & 0x1e;
    let flags: u32 = if tipo == 4 || tipo == 6 {
        let mut f = 0x5882;
        if cqa7_d(vaga + 0xf) == cqa7_d(p) && cqa7_d(p + 0x39) == clube && humano(clube, 1, 1, 0, 0) != 0 {
            f = 0x815882;
        }
        match escopo(p, cqa7_d(clube + 0xcf)) as i8 {
            0 => f | 0x40,
            1 => f | 0x50,
            2 => f | 0x10,
            3 => f | 0x30,
            4 => f | 0x20,
            _ => f,
        }
    } else if tipo == 8 {
        match escopo(p, cqa7_d(clube + 0xcf)) as i8 {
            0 => 0x59c2,
            1 => 0x59d2,
            2 => 0x5992,
            3 => 0x59b2,
            4 => 0x59a2,
            _ => 0x5982,
        }
    } else {
        0x5902
    };
    if cqa7_b(vaga + 0x24) & 1 != 0 {
        div = if emprestimo(p, 0) != 0 { 0x10 } else { 6 };
    }
    // o mínimo² × 0,000025 / divisor (_ftol)
    let corte = |m: i32, div: i8| {
        let q = m as f64;
        ftol(q * cqa7_k(0x96d240) * q / div as f64)
    };
    if humano(clube, 1, 1, 0, 0) != 0 {
        let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
        let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
        let substituto: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x600e90usize);
        let m = tecnico(clube);
        if na_lista(cqa7_d(0xb74448), p, m) != 0 {
            let s = substituto(clube);
            let r = nota(p, pos, flags, clube, -1, s);
            *saida = r;
            let a = r.wrapping_add(15000) as f64;
            let q = r as f64 * cqa7_k(0x968940);
            *saida = ftol(if a > q { a } else { q });
        } else {
            let s = substituto(clube);
            let v = corte(minimo, div);
            *saida = nota(p, pos, flags, clube, v, s);
        }
    } else {
        if flags & 0x100 != 0 || flags & 0x80 != 0 {
            let nivel: i8 = if flags & 0x100 != 0 { 6 } else { 5 };
            let mut melhor: i8 = 0;
            if flags & 0x100 != 0 {
                if obs != 0 && cqa7_d(obs + 0x69) != 0 {
                    melhor = conhecer(cqa7_d(obs + 0x69), 6) as i8;
                    match escopo(p, obs) as i8 {
                        3 => melhor = melhor.wrapping_sub(5),
                        4 => melhor = melhor.wrapping_sub(10),
                        _ => {}
                    }
                }
            } else if obs != 0 {
                // a marca 0x80: o técnico entra com +5 (−5 e +0 pelo escopo 4 e 3)
                melhor = (conhecer(cqa7_d(obs + 0x69), 5) as i8).wrapping_add(5);
                match escopo(p, obs) as i8 {
                    3 => melhor = melhor.wrapping_sub(5),
                    4 => melhor = melhor.wrapping_sub(10),
                    _ => {}
                }
            }
            for k in 0..7 {
                let s = cqa7_d(clube + 0x1b3 + 4 * k);
                if s == 0 || cqa7_d(s + 0x69) == 0 {
                    continue;
                }
                let mut v = conhecer(cqa7_d(s + 0x69), nivel) as i8;
                match escopo(p, s) as i8 {
                    3 => {
                        if nivel == 6 && cqa7_w(clube + 0x80) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(5);
                    }
                    4 => {
                        if nivel == 6 && cqa7_w(clube + 0x80) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(10);
                    }
                    _ => {}
                }
                if v > melhor {
                    melhor = v;
                    obs = s;
                }
            }
        }
        let v = if minimo > -1 { corte(minimo, div) } else { -1 };
        *saida = nota(p, pos, flags, clube, v, obs);
        let transferencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
        if clube != cqa7_d(p + 0x39) && transferencia(p, 0) != 0 {
            crate::comparacao::ramo("cqa_7 listado para transferencia");
            *saida = ftol(*saida as f64 * cqa7_k(0x9689b8)); // cqa7: o listado de outro clube × 1,05
        }
    }
    let r = *saida;
    if r < 0 || r < minimo.wrapping_sub(10000) || cqa7_b(vaga + 0x24) & 1 != 0 {
        return cqa7_sem_valor(p, vaga, saida);
    }
    let nevoa = cqa7_b(cqa7_d(0xae2d0c).wrapping_add(cqa7_d(p).wrapping_mul(0x4f)) + 0xb) & 0xf;
    if nevoa != 0 {
        let a = r.wrapping_add(5000) as f64;
        let w = (pow_do_jogo(nevoa as f64, cqa7_k(0x968950)) * cqa7_k(0x967898) + cqa7_k(0x967890)) * r as f64;
        *saida = ftol(if a < w { a } else { w });
    }
    div = 0x1c;
    let posicao: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x83bd00usize);
    let c = posicao(this, pos) as i8 as i32;
    let (teto_pos, mut frac) = match c {
        1..=4 | 17 => (8000000.0, 0.6),
        5..=7 | 19 => (9000000.0, 0.65),
        8..=10 | 18 => (10000000.0, 0.75),
        11..=13 | 21 => (11000000.0, 0.8),
        14 | 20 => (13000000.0, 1.0),
        _ => {
            div = 0x20;
            (6000000.0, 0.575)
        }
    };
    let r = *saida;
    let bv = cqa7_d(vaga + 0xb) as i32;
    let alta = r >= 35000 && bv >= r / 8 && cqa7_w(vaga + 1) >= 0x80;
    let mut e = if alta {
        let v = r as f64;
        ftol(((v * cqa7_k(0x96d238) - cqa7_k(0x967890)) * v + bv as f64 * cqa7_k(0x9689e0)) * cqa7_k(0x96f918)) as u32
    } else {
        crate::comparacao::ramo("cqa_7 valor base");
        ftol((bv as f64 * cqa7_k(0x9689e0) + r as f64 * cqa7_k(0x969040)) * cqa7_k(0x96f918)) as u32 // cqa7: o valor base
    };
    if e < 1 {
        e = 1;
    }
    let idade = cqa7_b(p + 0x18) as i8;
    match (cqa7_b(vaga + 0x24) >> 1) & 0xf {
        1 | 2 => cqa7_jovem(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e),
        3 => {
            let k = if cqa7_w(vaga + 1) < 0x80 { cqa7_k(0x96a670) } else { cqa7_k(0x96d718) };
            e = ftol(e as f64 * k) as u32;
            cqa7_jovem(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e);
        }
        4 => e = ftol(e as f64 * cqa7_k(0x968968)) as u32,
        5 => {
            e = ftol(e as f64 * cqa7_k(0x968a80)) as u32;
            div = div.wrapping_sub(9);
        }
        6 => e = ftol(e as f64 * cqa7_k(0x968918)) as u32,
        _ => {}
    }
    let tipo = cqa7_b(vaga + 0x24) & 0x1e;
    if idade > div {
        let d = idade as i32 - div as i32;
        let dd = d as f64;
        let ficha = cqa7_d(p + 0x61);
        let f = cqa7_k(0x967890) / (dd * cqa7_k(0x968e80) + cqa7_k(0x9689b0)); // cqa7: a idade acima do divisor
        let ts = cqa7_w(ficha + 0xd);
        let g = if ts > 0x1770 { ts as i32 as f64 * cqa7_k(0x96a680) - cqa7_k(0x968940) } else { 0.0 };
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        let y = if estrangeira(p, cqa7_d(clube + 0x53), 1) != 0 {
            f / (dd * cqa7_k(0x968f20) + cqa7_k(0x9689d8))
        } else if idade as i32 > div as i32 + 2 && (tipo == 8 || tipo == 6) && cqa7_w(ficha + 0xb) < cqa7_w(clube + 0x80) {
            (cqa7_k(0x967890) - (d - 2) as f64 * cqa7_k(0x96d438)) * f
        } else {
            f
        };
        crate::comparacao::ramo("cqa_7 idade");
        let um = cqa7_k(0x967890);
        if tipo == 0xa {
            *saida = ftol(((um - g) * y + g) * *saida as f64);
        }
        e = ftol(((um - g) * y + g) * e as f64) as u32;
    }
    let ficha = cqa7_d(p + 0x61);
    let cons = cqa7_b(ficha + 0x2d) as i8 as f64;
    let c = ftol((cqa7_k(0x967890) - pow_do_jogo(cons, cqa7_k(0x968950)) * cqa7_k(0x968f40)) * e as f64);
    let mut kk = (c.wrapping_mul(c) as u32) as f64 * teto_pos * cqa7_k(0x968eb8);
    let ra = cqa7_w(clube + 0x80);
    if tipo == 0xc {
        if ra > 5000 {
            let lim = (ra as i32 - 5000) as f64 * cqa7_k(0x968ea8) + cqa7_k(0x968880);
            if !(kk <= lim) {
                kk = (ra as i32 - 100000) as f64 + cqa7_k(0x968880);
            }
        } else {
            let lim = (ra as i32 * 10) as f64;
            if !(kk <= lim) {
                kk = ra as f64 * cqa7_k(0x9688b0);
            }
        }
    } else if tipo == 0xa {
        let lim = if ra > 5000 {
            let z = (ra as i32 - 5000) as f64 * cqa7_k(0x968918) + cqa7_k(0x968878);
            z * z
        } else {
            let z = ra as f64;
            z * z * cqa7_k(0x96f910)
        };
        if !(kk <= lim) {
            kk = lim;
        }
    }
    let ts = cqa7_w(ficha + 0xd);
    if ts <= 5000 {
        let lim = frac * cqa7_k(0x96f908);
        if lim < kk {
            kk = lim;
        }
    } else if ts <= 8000 {
        let lim = ts as f64 * frac * cqa7_k(0x96f900) + cqa7_k(0x96f8f8);
        if lim < kk {
            kk = lim;
        }
    } else if ts < 9000 {
        let lim = (ts as f64 * cqa7_k(0x9690b8) - cqa7_k(0x96f8f0)) * frac;
        if lim < kk {
            kk = lim;
        }
        frac = frac * cqa7_k(0x968940);
    } else {
        kk = cqa7_k(0x96d568);
        frac = frac * cqa7_k(0x968e18);
    }
    let pela_idade = match idade {
        0x10 => Some(0x96f8d8),
        0x11 => Some(0x968e90),
        0x12 | 0x1f => Some(0x96d850),
        0x13 | 0x1d => Some(0x96f8e0),
        0x1c => Some(0x96a630),
        0x1e => Some(0x96f908),
        0x20 => Some(0x9688c8),
        0x21 => Some(0x96a650),
        0x22 => Some(0x9688d8),
        0x23 => Some(0x96f8e8),
        0x24..=0x28 => Some(0x968870),
        _ => None,
    };
    if let Some(a) = pela_idade {
        let t = frac * cqa7_k(a);
        if t < kk {
            kk = t;
        }
    }
    let base = cqa7_k(0x96f8d0);
    let mut x = kk;
    if kk > base {
        let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
        let n2 = vista(cqa7_d(clube + 0xcf), p, clube, 2, (*saida / 4) as u32) as i32 as f64;
        if n2 < cqa7_k(0x96f8c8) {
            let mut a = n2 * cqa7_k(0x967878) - cqa7_k(0x96f8c0);
            if !(cqa7_k(0x9689a0) <= a) {
                a = cqa7_k(0x9689a0);
            }
            x = (kk - base) * a + base;
        }
    }
    let bc = cqa7_d(clube + 0x57);
    if bc != 0 && cqa7_w(bc + 0x69) < 0x10 {
        let b69 = cqa7_w(bc + 0x69) as i32;
        let ri = cqa7_w(clube + 0x80) as i32 / 500;
        let m = if ri > b69 { ri } else { b69 };
        let mut lim = m.wrapping_mul(670000);
        if lim <= 1000000 {
            lim = 1000000;
        }
        let ts = cqa7_w(ficha + 0xd) as i32;
        if ts > 8000 {
            lim = lim.wrapping_add((ts - 8000) * 10000);
        }
        let l = lim as f64;
        if !(x <= l) {
            let m2 = if cqa7_k(0x967890) < frac { cqa7_k(0x967890) } else { frac };
            x = ftol(l * m2) as f64;
        }
    }
    // a nação do clube (+0x53 do clube: a original usa edi, que aqui já é o clube)
    let nacao = cqa7_d(cqa7_d(clube + 0x53));
    if nacao == cqa7_d(0x9cf434) {
        if x < cqa7_k(0x968e90) {
            x = x * (x * cqa7_k(0x968e88) * cqa7_k(0x968e48) + cqa7_k(0x968e80));
        } else {
            x = x * cqa7_k(0x9689d8);
        }
    } else if nacao == cqa7_d(0x9cf300) {
        x = ftol(x * cqa7_k(0x9689b0)) as f64;
    }
    if teto != 0 {
        let c = teto_i as f64;
        if c < x {
            x = c;
        }
        x = ftol(x) as f64;
    }
    ftol(x)
}

/// Comparador da 0x840d80: lado a lado na nota de saída e na devolução (o resto ela só lê). O técnico do clube sem a
/// ficha +0x69 (a original sorteia uma): só a original.
unsafe extern "thiscall" fn comparar_valor_do_jogador_para_o_clube(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32>(1437);
    let obs = cqa7_d(clube + 0xcf);
    if obs != 0 && cqa7_d(obs + 0x69) == 0 {
        return o(this, clube, p, vaga, saida, minimo, teto);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida as usize, 4)], || o(this, clube, p, vaga, saida, minimo, teto), || valor_do_jogador_para_o_clube(this, clube, p, vaga, saida, minimo, teto));
    PLACAR.registrar(igual && r == n, || {
        format!("valor_do_jogador_para_o_clube(clube {:x}, pessoa {:x}, vaga {:x}, mínimo {}, teto {}) original={} rust={} {:x?} x {:x?}", clube, p, vaga, minimo, teto, r, n, mo, mr)
    });
    r
}

/// { 0x7f8610 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::find_club_search_entry(pessoa)`, das telas da
/// pessoa e da busca (0x808540, 0x813210, 0x870650, 0x87a510): sem o humano da vez ou sem o clube da pessoa (+0x39), 0.
/// A vaga do humano (byte +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x4b5 e 0). A primeira das 10
/// buscas (0x7f6cd0) com o clube observado (+0x190) igual ao da pessoa e o registro dele (+0x170) dá a vaga da pessoa
/// nele (0x7bd700 sem reservar); nenhuma, 0.
unsafe extern "thiscall" fn entrada_do_jogador_na_busca_do_clube(this: usize, p: usize) -> usize {
    let m = cps8_gerente_da_vez();
    if m == 0 || cqa7_d(p + 0x39) == 0 {
        return 0;
    }
    let vaga = cqa7_b(m).wrapping_sub(cqa7_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        aviso_em(0xa7b094, 0x4b5);
        return 0;
    }
    let busca: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    for i in 0..10u32 {
        let l = busca(this, i, m);
        // ponytail: a original lê +0x190 sem conferir a busca (cairia com 0); aqui a busca nula não conta
        if l == 0 || cqa7_d(l + 0x190) != cqa7_d(cqa7_d(p + 0x39)) || cqa7_d(busca(this, i, m) + 0x170) == 0 {
            continue;
        }
        let vaga_do_jogador: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7bd700usize);
        return vaga_do_jogador(cqa7_d(busca(this, i, m) + 0x170), p, 0);
    }
    0
}

/// Comparador da 0x7f8610: pela devolução (só lê).
unsafe extern "thiscall" fn comparar_entrada_do_jogador_na_busca_do_clube(this: usize, p: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1438);
    let r = o(this, p);
    let n = entrada_do_jogador_na_busca_do_clube(this, p);
    PLACAR.registrar(r == n, || format!("entrada_do_jogador_na_busca_do_clube(pessoa {:x}) original={:x} rust={:x}", p, r, n));
    r
}

/// { 0x8d0f00 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_international_appearance_money(pessoa)`,
/// das telas da pessoa (0x884ea2): sem a marca 4 da pessoa ([this+0x86e] + id × 8), 0. A busca binária (0x946464:
/// cdecl(chave, vetor [this+0x898], conta [this+0x8a0], 0xd, 0x8b4e00)) pela chave com o id da pessoa em +0, o único
/// campo que a comparação lê; achada, o registro; senão a marca 4 sai e 0.
unsafe extern "thiscall" fn premio_de_jogos_na_selecao_achado(this: usize, p: usize) -> usize {
    let m = (cqa7_d(this + 0x86e).wrapping_add(cqa7_d(p).wrapping_mul(8))) as *mut u8;
    if *m & 4 == 0 {
        return 0;
    }
    let chave = [cqa7_d(p) as u32, 0, 0, 0];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(chave.as_ptr(), cqa7_d(this + 0x898), cqa7_d(this + 0x8a0), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(4);
    0
}

/// Comparador da 0x8d0f00: lado a lado na marca da pessoa e na devolução.
unsafe extern "thiscall" fn comparar_premio_de_jogos_na_selecao_achado(this: usize, p: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1439);
    let m = cqa7_d(this + 0x86e).wrapping_add(cqa7_d(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, p), || premio_de_jogos_na_selecao_achado(this, p));
    PLACAR.registrar(igual && r == n, || format!("premio_de_jogos_na_selecao_achado(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

// ---------- 263a leva: os contratos depois do jogo (os bônus, a felicidade, a moral e a evolução de quem ficou de fora), a moral da ficha e o texto dela (contract_manager.cpp) ----------

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
unsafe fn cqa8_somar(a: usize, delta: i32) {
    let v = cqa8_b(a);
    let s = v as i32 + delta;
    *(a as *mut i8) = if s > 100 { 100 } else if s < -100 { -100 } else { v.wrapping_add(delta as i8) };
}

/// Os dois clubes do jogo (+0x1c mandante, +0x20 visitante): o do lado e o outro.
unsafe fn cqa8_clubes(jogo: usize, lado: usize) -> (usize, usize) {
    if lado == 0 { (cqa8_d(jogo + 0x1c), cqa8_d(jogo + 0x20)) } else { (cqa8_d(jogo + 0x20), cqa8_d(jogo + 0x1c)) }
}

/// Com o byte +0x3a do jogo = 1, o lado fica de fora quando a reputação do outro clube (+0x80) fica abaixo do maior
/// entre a dele × [0x968928] e a dele + 1500 (x87 em dupla, como a original).
unsafe fn cqa8_pula_lado(jogo: usize, lado: usize) -> bool {
    if cqa8_b(jogo + 0x3a) != 1 {
        return false;
    }
    let (meu, outro) = cqa8_clubes(jogo, lado);
    let r = cqa8_w(meu + 0x80) as i32;
    let x = r as f64 * f64_em(0x968928);
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > cqa8_w(outro + 0x80) as f64
}

/// A pessoa da vaga do resumo (id em +0 e +4 diferente de −1; registro de 0x6e bytes em [0xae23bc]), ou None quando a
/// original pula a vaga. A pessoa nula (base 0) passa, como na original.
unsafe fn cqa8_pessoa_da_vaga(vaga: usize) -> Option<usize> {
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
unsafe fn cqa8_bonus(jogo: usize, contrato: usize, vaga: usize, palavra: i16) -> [Option<i32>; 3] {
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
/// fora das seleções, o contrato com o clube (0x4cf5f0) paga os bônus (cqa8_bonus); os minutos (+0x30 dos titulares,
/// 90 sem valor; 90 − +0x32 dos reservas) mudam a felicidade (+0x3d, minutos × 10 / 90, presa em 50 abaixo de 50) ou,
/// nas seleções, o +9 do registro de 79 bytes; fora das seleções e dos amistosos, os reservas e os titulares de pouco
/// jogo sorteiam (2 × sorteio(30)) contra os minutos: a felicidade cai 4, +0x3e conta e a moral cai (0x4cb250); senão
/// +0x3f conta (com 5 ou mais, fora do clube humano, a marca de elenco 1 em 0x540980) e a moral sobe (0x4cb0a0). O
/// vencedor (e o perdedor) sorteia(21 − +0x57) abaixo de 4 para +3 (−3) na moral +0x44; a pessoa sai da cópia do
/// elenco. Depois, quem ficou na cópia (com ficha, contrato, sem suspensão 0x55d270, lesão 0x62c400, seleção 0x7757d0
/// nem empréstimo) evolui (0x6d26b0 com a nota média [0xb74350] − 1), +0x3e conta e, a partir de 8 (14 com parceiro),
/// fora do humano e do parceiro, sai do principal (0x4cbb60 2 ou 4 → 0x4cbd20); a felicidade cai 8 e a moral cai.
unsafe extern "thiscall" fn contratos_depois_do_jogo(this: usize, jogo: usize, resumo: usize) {
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
        let c = cqa8_clubes(jogo, lado).0;
        core::ptr::copy_nonoverlapping((c + 0xd7) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let clube = cqa8_clubes(jogo, lado).0;
        if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if cqa8_pula_lado(jogo, lado) {
            continue;
        }
        let fin = financas(cqa8_d(0xae2a38), clube);
        let base = resumo + lado * 0x625;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = cqa8_pessoa_da_vaga(vaga) else { continue };
            let contrato = if selecao(clube) != 0 {
                0
            } else {
                let c = com_clube(this, pessoa, clube);
                if c == 0 {
                    continue;
                }
                for v in cqa8_bonus(jogo, c, vaga, cqa8_w(base + 0x154 + 2 * k)).into_iter().flatten() {
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
                    crate::comparacao::ramo("cqa_8 testa a moral");
                    let m = cqa8_b(contrato + 0x44) as i32;
                    let r1 = sorteio(0x1e); // cqa8: o primeiro sorteio da moral
                    let v = r1 as f64 + (f64_em(0x968d68) - m as f64 * f64_em(0x968d70));
                    let r2 = sorteio(0x1e);
                    v - r2 as f64 > minutos as f64
                };
                let p = contrato as *mut u8;
                if mal {
                    *p.add(0x3f) = 0;
                    if *p.add(0x3e) != 100 {
                        *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
                    }
                    cqa8_somar(contrato + 0x3d, -4);
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
                crate::comparacao::ramo("cqa_8 moral do vencedor");
                let r = sorteio(0x15 - cqa8_b(pessoa + 0x57) as i32); // cqa8: o sorteio do vencedor
                if r < 4 && contrato != 0 {
                    cqa8_somar(contrato + 0x44, 3);
                }
            } else if venceu[1 - lado] != 0 {
                let r = sorteio(0x15 - cqa8_b(pessoa + 0x57) as i32);
                if r < 4 && contrato != 0 {
                    cqa8_somar(contrato + 0x44, -3);
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
        let (clube, outro) = cqa8_clubes(jogo, lado);
        let limite: i8 = if parceiro(clube, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if cqa8_pula_lado(jogo, lado) {
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
            let c = f64_em(0x9689a0);
            // fcomp + C3: igual ou desordenado vai para a constante
            let nota = if !(x < c || x > c) { f64_em(0x968998) } else { core::ptr::read_unaligned((m + 0x10) as *const f64) / x };
            crate::comparacao::ramo("cqa_8 evolui quem ficou de fora");
            evoluir(0, pessoa, clube, outro, jogo, (nota - f64_em(0x967888)) as f32, 0, 0); // cqa8: a evolução de quem ficou de fora
            let p = contrato as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if cqa8_b(contrato + 0x3e) >= limite && humano(clube, 0, 1, 0, 0) == 0 && parceiro(clube, core::ptr::null_mut(), 1) == 0 {
                let l = lugar(this, pessoa, clube);
                if l == 2 || l == 4 {
                    trocar(this, pessoa, clube, 0);
                }
            }
            cqa8_somar(contrato + 0x3d, -8);
            baixar(this, pessoa, clube, contrato);
        }
    }
}

/// Comparador: a moral (0x4cb0a0, 0x4cb250: queixas, pedido de transferência), a evolução (0x6d26b0) e a troca de
/// elenco (0x4cbd20) sorteiam e alocam (não podem rodar duas vezes): só a original roda. Confere o que a rotina garante
/// antes de qualquer sorteio: os bônus que cada clube paga (+0x138 das finanças soma o total previsto pelas mesmas
/// regras da cópia, calculado antes). A cópia é provada pelos ramos e pelas mutações na carreira.
unsafe extern "thiscall" fn comparar_contratos_depois_do_jogo(this: usize, jogo: usize, resumo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1440);
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
        let clube = cqa8_clubes(jogo, lado).0;
        if (parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0) || cqa8_pula_lado(jogo, lado) || selecao(clube) != 0 {
            continue;
        }
        let fin = financas(cqa8_d(0xae2a38), clube);
        let base = resumo + lado * 0x625;
        let mut total = 0i32;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = cqa8_pessoa_da_vaga(vaga) else { continue };
            let c = com_clube(this, pessoa, clube);
            if c != 0 {
                for v in cqa8_bonus(jogo, c, vaga, cqa8_w(base + 0x154 + 2 * k)).into_iter().flatten() {
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
            PLACAR.registrar(pago == total, || format!("contratos_depois_do_jogo(jogo {:x}) bônus de fin {:x}: original {} previsto {}", jogo, fin, pago, total));
        }
    }
}

/// { 0x4cc2d0 } (stdcall, ret 4, contract_manager.cpp): `contract_manager_get_player_morale(pessoa)`: o byte +0x45 da
/// ficha (+0x61), pelas telas. O eax leva os bytes altos da pessoa, como na original.
unsafe extern "stdcall" fn moral_da_ficha(pessoa: usize) -> u32 {
    (pessoa as u32 & 0xffff_ff00) | *((cqa8_d(pessoa + 0x61) + 0x45) as *const u8) as u32
}

unsafe extern "stdcall" fn comparar_moral_da_ficha(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1441)(pessoa);
    let n = moral_da_ficha(pessoa);
    PLACAR.registrar(r == n, || format!("moral_da_ficha({:x}) original={:#x} rust={:#x}", pessoa, r, n));
    r
}

/// O texto terminado em zero em `a` (até 0x400 bytes), para os comparadores dos textos.
unsafe fn cqa8_texto(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// { 0x4cc2e0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_write_morale_text(pessoa, saída, _)`: a
/// palavra da moral (+0x45 da ficha: abaixo de 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", senão
/// "Superb") na saída (0x66f4e0 com 0xad9c64), pelas telas.
unsafe extern "stdcall" fn texto_da_moral(pessoa: usize, saida: usize, _x: u32) {
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
unsafe extern "stdcall" fn comparar_texto_da_moral(pessoa: usize, saida: usize, x: u32) {
    orig::<unsafe extern "stdcall" fn(usize, usize, u32)>(1442)(pessoa, saida, x);
    let r = cqa8_texto(saida);
    texto_da_moral(pessoa, saida, x);
    let n = cqa8_texto(saida);
    core::ptr::copy_nonoverlapping(r.as_ptr(), saida as *mut u8, r.len());
    *((saida + r.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n, || format!("texto_da_moral({:x}) original={:?} rust={:?}", pessoa, String::from_utf8_lossy(&r), String::from_utf8_lossy(&n)));
}

// ---------- 263a leva: o valor do jogador, a comparação da moral de dois jogadores e as marcas de elenco da tela do clube (contract_manager.cpp) ----------

unsafe fn cqa9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqa9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cqa9_clube(id: usize) -> usize {
    cqa9_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato principal da pessoa (índice +0 do registro de 79 bytes em [0xae2d0c], registros de 0x50 bytes em
/// [[0xae1050]]), ou 0: sem pessoa, fora das pessoas ([0xae2364]) ou sem índice.
unsafe fn cqa9_contrato_principal(pessoa: usize) -> usize {
    if pessoa == 0 || cqa9_d(pessoa) as i32 >= cqa9_d(0xae2364) as i32 {
        return 0;
    }
    let i = cqa9_d(cqa9_d(0xae2d0c).wrapping_add(cqa9_d(pessoa).wrapping_mul(0x4f))) as i32;
    if i < 0 { 0 } else { (i as usize).wrapping_mul(0x50).wrapping_add(cqa9_d(cqa9_d(0xae1050))) }
}

/// O mínimo com a cláusula de rescisão (+0x21 do contrato, se 0x867b50 a diz ativa) e o truncamento (_ftol).
unsafe fn cqa9_fim(contrato: usize, w: f64) -> i32 {
    let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
    if contrato != 0 && clausula(contrato) != 0 {
        let cl = core::ptr::read_unaligned((contrato + 0x21) as *const i32) as f64;
        return ftol(if !(w >= cl) { w } else { cl });
    }
    ftol(w)
}

/// { 0x4d0c60 } (thiscall, ret 0x14, contract_manager.cpp): `CONTRACT_MANAGER::calc_player_value(pessoa, clube, modo,
/// habilidade, _)`. O contrato principal vai para [0xae1080] e o contrato com o clube dele (0x4cf5f0, no this de
/// entrada). Sem ficha (+0x61): a compensação (0x8d0780 com 1) pelo clube do contrato, ou 0. A base e a idade do auge
/// pelas posições da ficha (+0x15/+0x14/+0x13/+0x12 ≥ 15: 13, 11, 10, 9 milhões; goleiro +0xf = 20: 6 milhões, 33; senão
/// 8 milhões; auge 28 ou 29). O fator da idade (auge − idade, com a correção acima de [0x9688a0]) × as notas da ficha
/// (+0x28, +0x27, +0x43, +0x36, +0x2f, fora dos goleiros) e +0x2d²; a reputação (+9) limita o fator. A habilidade: sem
/// clube, a atual (+0xb); com clube, a vista pelo técnico (0x6e86d0 se `habilidade` = −1, com a reputação vista 0x5407e0)
/// × 50 + a reputação + até 5000 × fator, × [0x968f08], entre 1 e 12000; o potencial (+0xd) acima de 5000 (e 8000) antes
/// do auge a puxa. Valor = habilidade² × base × a tabela 0x9a7f30 (pela idade, com a experiência +0x22/+0x1a) × a nação
/// do clube (+0x53: +0x7e e +0x85 ou a divisão +0x69; sem clube, sorteio(3) + 1), com piso. `modo`: a nação [0x9cf434]
/// e a cláusula. Senão: os dias de contrato (com a lei Bosman 0x8d0a70) e as nações [0x9cf434], [0x9cf300], [0x9cf484];
/// no fim a cláusula de rescisão ativa (0x867b50) limita.
unsafe extern "thiscall" fn valor_do_jogador(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, _x: u32) -> i32 {
    let c1080 = cqa9_contrato_principal(pessoa);
    *(0xae1080 as *mut usize) = c1080;
    let clube_do_contrato = if c1080 == 0 { 0 } else { cqa9_clube(cqa9_d(c1080 + 4)) };
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = com_clube(this, pessoa, clube_do_contrato);
    let ficha = cqa9_d(pessoa + 0x61);
    if ficha == 0 {
        if clube_do_contrato == 0 {
            return 0;
        }
        // a original põe ecx = [0xde1f54] (a cópia Rust é stdcall): thiscall com 3 na pilha serve aos dois
        let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(0x8d0780usize);
        return compensacao(cqa9_d(0xde1f54), pessoa, clube_do_contrato, 1);
    }
    let (base, auge): (f64, i8) = if cqa9_b(ficha + 0x15) >= 0xf {
        (13_000_000.0, 0x1c) // cqa9: a base de quem joga na frente
    } else if cqa9_b(ficha + 0x14) >= 0xf {
        (11_000_000.0, 0x1d)
    } else if cqa9_b(ficha + 0x13) >= 0xf {
        (10_000_000.0, 0x1d)
    } else if cqa9_b(ficha + 0x12) >= 0xf {
        (9_000_000.0, 0x1d)
    } else if cqa9_b(ficha + 0xf) == 0x14 {
        (6_000_000.0, 0x21)
    } else {
        (8_000_000.0, 0x1d)
    };
    let idade = cqa9_b(pessoa + 0x18);
    let dif = auge as i32 - idade as i32;
    let mut t = dif as f64;
    if t > f64_em(0x9688a0) {
        t = f64_em(0x968f10) - t * f32_em(0x968f18) as f64;
    }
    let a = ftol(t).wrapping_abs();
    let mut f = t * a as f64 * f32_em(0x968f08) as f64 * f64_em(0x968f00) + f64_em(0x967890);
    if cqa9_b(ficha + 0xf) < 0xf {
        let nota = |o: usize, c: usize| (cqa9_b(ficha + o) as i32 - 0xf) as f64 * f64_em(c) + f64_em(0x967890);
        let fa = nota(0x28, 0x968ef8);
        let fb = cqa9_b(ficha + 0x27) as f64 * f64_em(0x968ef0) + f64_em(0x967890);
        let fc = cqa9_b(ficha + 0x43) as f64 * f64_em(0x968e60) + f64_em(0x967890);
        let fd = nota(0x36, 0x968ef8);
        let fe = nota(0x2f, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = cqa9_b(ficha + 0x2d) as i32;
    let fator = ((f64_em(0x968ee0) - (q * q) as f64 * f64_em(0x967888) * f64_em(0x968ee8)) * f) as f32;
    let mut r = cqa9_w(ficha + 9) as f64 * f64_em(0x968ed8) + f64_em(0x968ed0);
    if !(r >= f64_em(0x9689c0)) {
        r = f64_em(0x9689c0);
    }
    let x: f32 = if !(fator as f64 >= f64_em(0x967898)) {
        f64_em(0x967898) as f32
    } else if !(r >= fator as f64) {
        r as f32
    } else {
        fator
    };
    let mut v: u32 = if clube == 0 {
        cqa9_w(ficha + 0xb) as i32 as u32
    } else {
        let vista: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(0x5407e0usize);
        let tecnico = cqa9_d(clube + 0xcf);
        let rep = vista(pessoa, tecnico, 0) as i16 as i32;
        let h = if habilidade as u16 == 0xffff {
            let hab: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(0x6e86d0usize);
            hab(tecnico, pessoa, clube, 1, rep as i16) as u16
        } else {
            habilidade as u16
        };
        // só o ax de h × 50 conta (movsx depois); negativo vira 0
        let h50 = (h.wrapping_mul(50) as i16).max(0);
        let teto = if (rep as u32) < 0x1388 { rep as u32 } else { 0x1388 };
        let s = (h50 as i32).wrapping_add(rep) as u32;
        let y = ftol((s as f64 + teto as f64 * x as f64) * f32_em(0x968f08) as f64) as u32;
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = cqa9_w(ficha + 0xd);
    if pot > 0x1388 && (idade as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && idade <= auge {
            let z = pot as f64 * f64_em(0x968ec8) + (v >> 1) as f64 - f64_em(0x968ec0);
            let e = v as f64;
            v = ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * f64_em(0x968eb8);
    let mut idade_ef = idade;
    let exp = *((pessoa + 0x22) as *const u8);
    if exp != 0 && cqa9_d(pessoa + 0x1a) != 0 {
        let n = (cqa9_w(cqa9_d(pessoa + 0x1a) + 0x8e) as i32).wrapping_mul(exp as i32) / 18;
        let n = n as i16 as i32;
        let m = if n < dif { n } else { dif };
        let soma = if m < 0 { 0 } else if n >= dif { dif } else { n };
        idade_ef = idade_ef.wrapping_add(soma as i8);
    }
    let mut i = auge.wrapping_sub(idade_ef).wrapping_add(10); // cqa9: o índice da tabela pela idade
    if i < 0 {
        i = 0;
    } else if i >= 0x14 {
        i = 0x13;
    }
    let tab = *((0x9a7f30 + i as usize) as *const i8) as i32;
    let mut w = tab as f64 * vv * f64_em(0x967898);
    let c890 = f64_em(0x967890);
    let k = if clube == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        (sorteio(3) + 1) as f64
    } else {
        let div = cqa9_d(clube + 0x57);
        let nat = cqa9_d(clube + 0x53);
        if nat == 0 {
            c890
        } else {
            let mut t = cqa9_b(nat + 0x7e) as f64;
            if !(t >= c890) || t > f64_em(0x968930) {
                t = f64_em(0x968930);
            }
            if div != 0 {
                t = (t + c890) * f64_em(0x968968);
            }
            match cqa9_b(clube + 0x64) {
                2 => t = t + t,
                3 => t *= f64_em(0x968eb0),
                _ => {}
            }
            let n = if div != 0 { cqa9_w(div + 0x69) as i32 } else { cqa9_b(nat + 0x85) as i32 };
            let e = n.wrapping_mul(n).wrapping_mul(25).wrapping_add(cqa9_w(clube + 0x80) as i32);
            e as f64 / (t * f64_em(0x968908))
        }
    };
    let dd = f64_em(0x968ea8) - k;
    let mut l = c890 - dd * dd * f64_em(0x968918) * f64_em(0x968a98);
    if !(l >= f64_em(0x967880)) {
        l = f64_em(0x967880);
    } else if l > f64_em(0x9689c0) {
        l = f64_em(0x9689c0);
    }
    w = l * w;
    let k2 = k * k;
    if !(k2 * f64_em(0x968ea0) >= w) {
        w = w * f64_em(0x9689e0) + k2 * f64_em(0x968e98);
    }
    let piso = k * f64_em(0x9688e8);
    if !(w >= piso) {
        w = piso;
    }
    let nacao_e = |g: usize| -> bool {
        let nat = cqa9_d(clube + 0x53);
        clube != 0 && nat != 0 && cqa9_d(nat) == cqa9_d(g)
    };
    // a nação [0x9cf434]: abaixo de [0x968e90] a curva com piso, senão × [0x9689d8]
    let curva = |w: f64, piso: f64| -> f64 {
        if !(w >= f64_em(0x968e90)) {
            let v = (w * f64_em(0x968e88) * f64_em(0x968e48) + f64_em(0x968e80)) * w;
            if !(f64_em(0x9688e8) > v) { v } else { piso }
        } else {
            w * f64_em(0x9689d8)
        }
    };
    if modo as u8 != 0 {
        if nacao_e(0x9cf434) {
            w = curva(w, 5000.0);
        }
        return cqa9_fim(contrato, w);
    }
    let mut nacoes = true; // 0x4d1480 (as outras nações) ou 0x4d14ad (a [0x9cf434] antes)
    if contrato == 0 {
        w = f64_em(0x9689a0);
    } else {
        let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8d0a70usize);
        if bosman(cqa9_d(0xde1f54), pessoa) != 0 {
            let ano = |a: usize| core::ptr::read_unaligned(a as *const u16) as i32;
            let mut dias = (ano(contrato + 0x2f) - ano(0xae2c92)).wrapping_mul(365) + cqa9_w(contrato + 0x2d) as i32 - cqa9_w(0xae2c90) as i32;
            if dias < 1 {
                return 0;
            }
            let mut w2 = w;
            if clube == clube_do_contrato && cqa9_b(contrato + 0x3d) > 0 {
                if *((contrato + 0x35) as *const u8) & 0x3f == 2 {
                    dias = dias + 7 * cqa9_b(contrato + 0x3d) as i32 + 0xc8;
                } else {
                    w2 = w * f64_em(0x968968);
                }
            }
            let x = dias as f64 * f64_em(0x968e78);
            if dias < 0x15e { // cqa9: o contrato curto
                w = w2 * x;
            } else {
                let mut dv = f64_em(0x968e18) - x;
                if !(dv >= c890) {
                    dv = f32_em(0x968e70) as f64;
                }
                if dv > f64_em(0x968950) {
                    dv = f32_em(0x9689a8) as f64;
                }
                w = w2 / dv;
            }
        }
        nacoes = false;
    }
    if !nacoes {
        // 0x4d14ad: valor zero (ou desordenado) vai às outras nações; sem clube, ao fim
        if !(w < f64_em(0x9689a0) || w > f64_em(0x9689a0)) {
            nacoes = true;
        } else if clube == 0 {
            return cqa9_fim(contrato, w);
        } else if nacao_e(0x9cf434) {
            w = curva(w, f64_em(0x9688e8));
            return cqa9_fim(contrato, w);
        } else {
            nacoes = true;
        }
    }
    if nacoes && clube != 0 {
        if nacao_e(0x9cf300) {
            w *= f64_em(0x9689b0);
        } else if nacao_e(0x9cf484) {
            w *= f64_em(0x9689e0);
            let c = f64_em(0x968e68);
            if w > c {
                w = ftol((w - c) * f64_em(0x968968) + c) as f64;
            }
            let c = f64_em(0x9688e0);
            if w > c {
                w = ftol((w - c) * f64_em(0x968968) + c) as f64;
            }
        }
    }
    cqa9_fim(contrato, w)
}

/// As estáticas das datas (CPQ6_ESTATICAS) e o contrato guardado em [0xae1080]: o que a original e as chamadas dela
/// gravam, com o estado do sorteador (na pilha: a rotina é quente).
fn cqa9_regioes() -> [(usize, usize); 11] {
    let (e, c) = (crate::aleatorio::REGIOES_DO_ESTADO, CPQ6_ESTATICAS);
    [e[0], e[1], (0xae1080, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}

/// Comparador: lado a lado (sem alocar) com o estado do sorteador (sem clube, sorteio(3)), o [0xae1080] e as estáticas
/// das datas da compensação; a cópia sem contar os sorteios. Quente (5,6 milhões de chamadas): sem contador.
unsafe extern "thiscall" fn comparar_valor_do_jogador(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, x: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32>(1443);
    let regioes = cqa9_regioes();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, clube, modo, habilidade, x),
        || crate::aleatorio::sem_contar(|| valor_do_jogador(this, pessoa, clube, modo, habilidade, x)));
    PLACAR.registrar(igual && r == n, || format!("valor_do_jogador(pessoa {:x}, clube {:x}, modo {}, hab {}) original={} rust={} {:x?} x {:x?}", pessoa, clube, modo as u8, habilidade as i16, r, n, mo, mr));
    r
}

/// { 0x4cc3a0 } (stdcall, ret 0x10, contract_manager.cpp): `contract_manager_write_morale_comparison_text(a, b, saída,
/// _)`: a diferença das morais (+0x45 das fichas, a − b, em byte) vira "Superior" (até −14 e desde 13), "Better" (−13 a
/// −8 e 7 a 12), "Marginal" (−7 a −4 e 3 a 6) ou "Same" (−3 a 2) na saída (0x66f4e0 com 0xad9c64); devolve a diferença
/// (o jogo lê o al), pela tela de comparação.
unsafe extern "stdcall" fn diferenca_de_moral_em_texto(a: usize, b: usize, saida: usize, _x: u32) -> u8 {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let dif = cqa9_b(cqa9_d(a + 0x61) + 0x45).wrapping_sub(cqa9_b(cqa9_d(b + 0x61) + 0x45));
    let f = if dif < -13 {
        0x9a8348usize
    } else if dif < -7 {
        0x9a83bc
    } else if dif < -3 {
        0x9a8394
    } else if dif < 3 {
        0x9a8370
    } else if dif < 7 {
        0x9a8394
    } else if dif < 13 {
        0x9a83bc
    } else {
        0x9a8348
    };
    formatar(saida, f, 0xad9c64usize);
    dif as u8
}

/// O texto terminado em zero em `a` (até 0x400 bytes).
unsafe fn cqa9_texto(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// Comparador: a original escreve; a cópia escreve de novo (sobrescreve) e o texto e a diferença têm de bater; fica o
/// texto da original.
unsafe extern "stdcall" fn comparar_diferenca_de_moral_em_texto(a: usize, b: usize, saida: usize, x: u32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, usize, u32) -> u8>(1444)(a, b, saida, x);
    let tr = cqa9_texto(saida);
    let n = diferenca_de_moral_em_texto(a, b, saida, x);
    let tn = cqa9_texto(saida);
    core::ptr::copy_nonoverlapping(tr.as_ptr(), saida as *mut u8, tr.len());
    *((saida + tr.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n && tr == tn, || format!("diferenca_de_moral_em_texto original={} {:?} rust={} {:?}", r as i8, String::from_utf8_lossy(&tr), n as i8, String::from_utf8_lossy(&tn)));
    r
}

/// { 0x4cc130 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_flags_for_club(pessoa, clube)`,
/// pelas telas do clube: as marcas de elenco da pessoa (+0x5e sem os bits de seleção, & 0xf3), a não ser que ela esteja
/// emprestada (segundo contrato, +4 do registro de 79 bytes) a outro clube que não o `clube` nem o parceiro dele
/// (0x540a50): com parceiro, 1; senão 2 com o bit 0x40 de +0x35 do contrato principal, 1 sem. Sem clube, a caixa
/// "Error" (a original). O jogo lê o al.
unsafe extern "thiscall" fn marcas_de_elenco_para_a_tela(this: usize, pessoa: usize, clube: usize) -> u8 {
    if clube == 0 || pessoa == 0 {
        // ponytail: sem clube a caixa de erro; sem pessoa a original lê [0x5e] e cai: as duas ficam com a original
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1445)(this, pessoa, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let geral = || *((pessoa + 0x5e) as *const u8) & 0xf3;
    if cqa9_d(pessoa) as i32 >= cqa9_d(0xae2364) as i32 {
        return geral();
    }
    let reg = cqa9_d(0xae2d0c).wrapping_add(cqa9_d(pessoa).wrapping_mul(0x4f));
    let i = cqa9_d(reg + 4) as i32;
    if i < 0 {
        return geral();
    }
    let c = (i as usize).wrapping_mul(0x50).wrapping_add(cqa9_d(this));
    if c == 0 || cqa9_d(c + 4) == cqa9_d(clube) {
        return geral();
    }
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p != 0 && cqa9_d(c + 4) == cqa9_d(p) {
        return geral();
    }
    if parceiro(clube, core::ptr::null_mut(), 1) != 0 {
        return 1;
    }
    let i0 = cqa9_d(reg) as i32;
    if i0 < 0 {
        // a original lê [0x35] do contrato nulo e cai
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1445)(this, pessoa, clube);
    }
    let c0 = (i0 as usize).wrapping_mul(0x50).wrapping_add(cqa9_d(this));
    (*((c0 + 0x35) as *const u8) & 0x40 != 0) as u8 + 1
}

unsafe extern "thiscall" fn comparar_marcas_de_elenco_para_a_tela(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1445);
    if clube == 0 || pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let r = o(this, pessoa, clube);
    let n = marcas_de_elenco_para_a_tela(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("marcas_de_elenco_para_a_tela({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

// ---------- 263a leva: a diretoria e os jogadores depois do jogo: os jogadores que ela cita (com a confiança), o veredito da nota na partida e o texto dela (manager_manager.cpp) ----------

unsafe fn cqa10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa10_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqa10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cqa10_pessoa(id: i32) -> usize {
    cqa10_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e))
}

/// O registro da diretoria do clube na gerência dos técnicos ([this] + id × 0x49).
unsafe fn cqa10_diretoria(this: usize, clube: usize) -> usize {
    cqa10_d(this).wrapping_add(cqa10_d(clube).wrapping_mul(0x49))
}

/// { 0x6aaee0 } (thiscall, ret 0x28, manager_manager.cpp): pela notícia da diretoria depois do jogo (0x6a5fa0):
/// `(jogo, clube, ids, partida, elogio, a6..a10)`. Zera os 3 ids (−1). Clube fora das seleções com técnico (+0xbf) de
/// ficha (+0x69): sorteio(+0x16 da ficha) contra sorteio(10); passando, zera as 5 saídas e, no jogo da divisão do clube,
/// a10 = 1 e, pela tabela da liga (0x4a4850, com mais de 8 jogos), a9/a8 (+0x74/+0x73 da linha), a7 (+0x2c da diretoria)
/// e a6 (2 ou 0xfe pelos +0x19/+0x2d da linha). Os jogadores (cargo 11, com ficha) que o técnico rejeita (0x5480f0, se
/// `elogio` = 0) ou prefere (0x548090), do clube pelo contrato (0x4cf5a0, em [0xae1080]), abaixo da reputação do clube
/// (rejeitados) ou até 35 anos e acima dela − 750 (preferidos), com nota abaixo de 7 na partida (0x6ab670 = −1): os ids
/// vão em `ids` e a confiança da diretoria (+6) sobe (preferidos) ou cai (rejeitados) 10 × o +0x57 do técnico. Devolve
/// a soma das duas contas (o chamador não lê).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn jogadores_que_a_diretoria_cita(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                         a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if !ids.is_null() {
        for k in 0..3 {
            *ids.add(k) = -1;
        }
    }
    if clube == 0 {
        return 0;
    }
    let t = cqa10_d(clube + 0xbf);
    if t == 0 || cqa10_d(t + 0x69) == 0 || selecao(clube) != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqa_10 a diretoria sorteia");
    let r1 = sorteio(cqa10_b(cqa10_d(t + 0x69) + 0x16) as i32); // cqa10: o sorteio pela ficha do técnico
    let r2 = sorteio(10); // cqa10: o sorteio da diretoria
    if r1 < r2 {
        return 0;
    }
    *a10 = 0;
    *a6 = 0;
    *a7 = 0;
    *a8 = 0;
    *a9 = 0;
    let comp = cqa10_d(jogo + 0x14);
    if comp != 0 && cqa10_d(clube + 0x57) == comp {
        *a10 = 1;
        let div = cqa10_d(clube + 0x57);
        let c = cqa10_d(cqa10_d(0xadadfc).wrapping_add(cqa10_d(div).wrapping_mul(4)));
        if c != 0 && cqa10_d(c + 8) != 0 {
            let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
            let r = busca(cqa10_d(c + 8), clube);
            if r != 0 && *((r + 4) as *const u8) > 8 {
                *a9 = *((r + 0x74) as *const u8);
                *a8 = *((r + 0x73) as *const u8);
                *a7 = *((cqa10_diretoria(this, clube) + 0x2c) as *const u8);
                if cqa10_d(r + 0x19) == 2 {
                    *a6 = 2;
                } else if cqa10_d(r + 0x2d) == 2 {
                    *a6 = 0xfe;
                }
            }
        }
    }
    // a original guarda os escolhidos em 3 dwords logo antes da lista: m[0..3] são os escolhidos, m[3..] a lista
    let mut m = [0usize; 53];
    let mut n: i8 = 0;
    let (mut da, mut d9) = (0i8, 0i8);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let veredito: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x6ab670usize);
    let preferidos = *elogio != 0;
    if preferidos {
        let f: unsafe extern "C" fn(usize, usize, *mut i8) -> usize = core::mem::transmute(0x548090usize);
        f(t, m.as_mut_ptr().add(3) as usize, &mut n);
    } else {
        let f: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(0x5480f0usize);
        f(t, m.as_mut_ptr().add(3) as usize, &mut n as *mut i8 as usize);
    }
    let mut k = 0i8;
    while k < n {
        let p = m[3 + k as usize];
        k += 1;
        if p == 0 || cqa10_d(p + 0x61) == 0 || cqa10_b(p + 0x3d) != 0xb {
            continue;
        }
        let c = contrato_de(cqa10_d(0xae1050), p);
        *(0xae1080 as *mut usize) = c;
        if c == 0 {
            continue;
        }
        let cl = cqa10_d(0xae23b4).wrapping_add(cqa10_d(c + 4).wrapping_mul(0x245));
        if cl == 0 || cl != clube {
            continue;
        }
        let nota = cqa10_w(cqa10_d(p + 0x61) + 0xb);
        let rep = cqa10_w(clube + 0x80);
        let passa = if preferidos { cqa10_b(p + 0x18) < 0x24 && nota as i32 > rep as i32 - 0x2ee } else { nota < rep };
        if passa && veredito(this, p, partida) == 0xff {
            let i = if preferidos { &mut da } else { &mut d9 };
            m[*i as usize] = p;
            *i += 1;
        }
    }
    if !ids.is_null() {
        for i in 0..da.max(0) as usize {
            *ids.add(i) = cqa10_d(m[i]) as i32;
        }
        let base = da.max(0) as usize;
        for j in 0..d9.max(0) as usize {
            *ids.add(base + j) = cqa10_d(m[j]) as i32;
        }
    }
    let x = cqa10_b(cqa10_d(clube + 0xbf) + 0x57) as i16;
    let conf = (cqa10_diretoria(this, clube) + 6) as *mut i16;
    if da != 0 {
        crate::comparacao::ramo("cqa_10 diretoria elogia");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_add(x.wrapping_mul(10))); // cqa10: a confiança pelos preferidos
    } else if d9 != 0 {
        crate::comparacao::ramo("cqa_10 diretoria culpa");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_sub(x.wrapping_mul(10))); // cqa10: a confiança pelos rejeitados
    }
    (d9 as u8).wrapping_add(da as u8)
}

/// Comparador: lado a lado a partir do mesmo estado do sorteador, do [0xae1080], do registro da diretoria do clube, dos
/// 3 ids e das 5 saídas; a devolução também (a lista do técnico, a tabela da liga e o registro da partida só leem). A
/// cópia sem contar os sorteios.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_jogadores_que_a_diretoria_cita(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                                  a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, u32, *const u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8;
    let o = orig::<F>(1446);
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regioes.push((0xae1080, 4));
    if clube != 0 {
        regioes.push((cqa10_diretoria(this, clube), 0x49));
    }
    if !ids.is_null() {
        regioes.push((ids as usize, 12));
    }
    for s in [a6, a7, a8, a9, a10] {
        regioes.push((s as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10),
        || crate::aleatorio::sem_contar(|| jogadores_que_a_diretoria_cita(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10)));
    PLACAR.registrar(igual && r == n, || format!("jogadores_que_a_diretoria_cita(clube {:x}, elogio {}) original={} rust={} {:x?} x {:x?}", clube, *elogio, r, n, mo, mr));
    r
}

/// { 0x6ab670 } (thiscall, ret 8, manager_manager.cpp, quadro SEH do vetor local): `get_player_match_rating_verdict(
/// pessoa, partida)`: lê o registro da partida (0x729600 em [0xb6b7c0], com os dois lados de 0x625 bytes construídos e
/// destruídos pelos iteradores 0x9463af/0x9462bb) e acha a pessoa entre os 11 primeiros de cada lado (ids em +0x71, 0x49
/// bytes): a nota (+0x35) abaixo de 7 dá −1, acima de 7 dá 1, 7 dá 0; não achada (ou sem pessoa), 0. O ecx não conta.
unsafe extern "thiscall" fn veredito_da_nota_na_partida(_this: usize, pessoa: usize, partida: u32) -> u8 {
    let construir: unsafe extern "stdcall" fn(usize, usize, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let destruir: unsafe extern "stdcall" fn(usize, usize, u32, usize) = core::mem::transmute(0x9462bbusize);
    let ler: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x729600usize);
    let mut registro = [0u32; 0x360]; // 0xd7a bytes do registro da partida
    let r = registro.as_mut_ptr() as usize;
    let lados = r + 0x11f;
    construir(lados, 0x625, 2, 0x453c30, 0x453c10);
    let mut v = 0u8;
    if pessoa != 0 && ler(cqa10_d(0xb6b7c0), partida, r) != 0 {
        'busca: for lado in 0..2usize {
            for k in 0..20usize {
                let s = lados + lado * 0x625 + 0x71 + k * 0x49;
                let id = cqa10_d(s) as i32;
                if id != -1 && pessoa == cqa10_pessoa(id) && k < 11 {
                    let nota = *((s + 0x35) as *const u8);
                    v = if nota < 7 { 0xff } else if nota > 7 { 1 } else { 0 };
                    break 'busca;
                }
            }
        }
    }
    destruir(lados, 0x625, 2, 0x453c10);
    v
}

/// Comparador: pelo valor (o registro é lido do arquivo temporário e o vetor local se constrói e destrói nos dois).
unsafe extern "thiscall" fn comparar_veredito_da_nota_na_partida(this: usize, pessoa: usize, partida: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(1447)(this, pessoa, partida);
    let n = veredito_da_nota_na_partida(this, pessoa, partida);
    PLACAR.registrar(r == n, || format!("veredito_da_nota_na_partida({:x}, partida {}) original={} rust={}", pessoa, partida as i32, r as i8, n as i8));
    r
}

/// { 0x6ab2d0 } (stdcall, ret 0x10, manager_manager.cpp): `build_board_player_performance_text(ids, vitória, saída,
/// variante)`, pelo texto da diretoria (0x6a69c0). A original guarda só o primeiro id que não é −1 (no lugar dele, 0, 1
/// ou 2), conta 1 e os textos leem sempre o lugar 0. Vitória: com o primeiro, o nome (0x53dba0, 200, estilo 1) num dos
/// três elogios pela variante (0, 1, outra); depois, como nenhum jogador conta na derrota, a saída fica vazia (a original
/// zera o primeiro byte). Derrota: a saída vazia e, com o primeiro, uma das quatro culpas pela variante (0, 1, 2,
/// outra), com 100 e o nome. Sem ids, nada.
unsafe extern "stdcall" fn texto_da_diretoria_sobre_os_jogadores(ids: *const i32, vitoria: u32, saida: usize, variante: u32) {
    if ids.is_null() {
        return;
    }
    let (i0, i1, i2) = (*ids, *ids.add(1), *ids.add(2));
    if i0 == -1 && (i1 != -1 || i2 != -1) {
        // ponytail: a original lê o lugar 0 sem valor (a pilha): esse caso fica com ela
        return orig::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(1448)(ids, vitoria, saida, variante);
    }
    let nome: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut buf = [0u8; 0xc8];
    let b = buf.as_mut_ptr() as usize;
    crate::comparacao::ramo("cqa_10 texto da diretoria");
    if vitoria as u8 != 0 {
        if i0 != -1 {
            nome(cqa10_pessoa(i0), b, 0xc8, 1);
            let f = match variante {
                0 => 0xa12558usize,
                1 => 0xa125ac,
                _ => 0xa125f8,
            };
            formatar(saida, f, b);
        }
        *(saida as *mut u8) = 0;
    } else {
        *(saida as *mut u8) = 0;
        if i0 != -1 {
            nome(cqa10_pessoa(i0), b, 0xc8, 1);
            let f = match variante {
                0 => 0xa12294usize,
                1 => 0xa12310,
                2 => 0xa12384,
                _ => 0xa123ec,
            };
            formatar(saida, f, 0x64u32, b);
        }
    }
}

/// Comparador: lado a lado nos primeiros 0x200 bytes da saída (o único chamador, 0x6a69c0, passa um texto de 1500).
unsafe extern "stdcall" fn comparar_texto_da_diretoria_sobre_os_jogadores(ids: *const i32, vitoria: u32, saida: usize, variante: u32) {
    let o = orig::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(1448);
    if ids.is_null() || (*ids == -1 && (*ids.add(1) != -1 || *ids.add(2) != -1)) {
        return o(ids, vitoria, saida, variante);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 0x200)], || o(ids, vitoria, saida, variante),
        || texto_da_diretoria_sobre_os_jogadores(ids, vitoria, saida, variante));
    PLACAR.registrar(igual, || format!("texto_da_diretoria_sobre_os_jogadores(vitória {}, variante {}) {:?} x {:?}", vitoria as u8, variante,
        String::from_utf8_lossy(&mo.0[0].1[..0x40]), String::from_utf8_lossy(&mr.0[0].1[..0x40])));
}

// ---------- 263a leva: os registros dos técnicos: o resultado de cada jogo, a posição no fim da liga e o jogador do ano da torcida (staff_records.cpp) ----------

unsafe fn cqa11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal em `a`.
unsafe fn cqa11_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cqa11_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cqa11_i(a).wrapping_add(v));
}

unsafe fn cqa11_mais1(a: usize) {
    core::ptr::write_unaligned(a as *mut i16, cqa11_w(a).wrapping_add(1));
}

unsafe fn cqa11_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de pontos do técnico do clube (+0xcf, lido sem conferir o clube; 0x86f5b0 com criar), ou 0 sem técnico.
unsafe fn cqa11_registro(this: usize, clube: usize) -> usize {
    let t = cqa11_d(clube + 0xcf);
    if t == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    f(this, t, 1)
}

/// Os pontos de um lado na partida `m`: ((a + b) × 5 + (c − b, em 8 bits)) × 2 + d, vezes 5 (bytes com sinal).
unsafe fn cqa11_pontos(m: usize, a: usize, b: usize, c: usize, d: usize) -> i32 {
    let x = cqa11_c(m + a).wrapping_add(cqa11_c(m + b));
    let y = (*((m + c) as *const u8)).wrapping_sub(*((m + b) as *const u8)) as i8 as i32;
    let x = x.wrapping_mul(5).wrapping_add(y);
    cqa11_c(m + d).wrapping_add(x.wrapping_mul(2)).wrapping_mul(5)
}

/// A nota do jogo para o técnico: a importância (0x48e190 da competição do jogo, com `lado` e a saída em `saida`; só o
/// byte, com sinal) × (reputação do adversário / 500) × o peso da competição (+0x69 de [+0x14]) × (pontos + bônus) × 50,
/// dividido por 100000 (divisões com sinal truncadas, como as multiplicações mágicas da original; produtos em 32 bits).
unsafe fn cqa11_nota(p: usize, lado: u32, saida: &mut u32, adversario: usize, pontos: i32) -> i32 {
    let comp = cqa11_d(cqa11_d(0xadadfc).wrapping_add(4 * cqa11_d(p)));
    let importancia: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(0x48e190usize);
    let i = importancia(comp, p, lado, saida as *mut u32) as u8 as i8 as i32;
    let x = i
        .wrapping_mul(cqa11_w(adversario + 0x80) as i32 / 500)
        .wrapping_mul(cqa11_w(cqa11_d(p + 0x14) + 0x69) as i32)
        .wrapping_mul(pontos)
        .wrapping_mul(50);
    x / 100000
}

/// { 0x868830 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::record_match_result(jogo, partida)`, pelo
/// fechamento da partida do dia (0x6b5f00), a cada jogo. Jogo não disputado (+0x3a = 1): nada. Os registros dos dois
/// técnicos (+0xcf do mandante +0x1c e do visitante +0x20; 0x86f5b0 com criar): +1 jogo (+0x8e); os gols pró (+0x96) e
/// contra (+0x9a) pelo placar (+0x47/+0x48, ou +0x43/+0x44 com +0x47 = −1); vitória (+0x90, bônus 900), empate (+0x92,
/// 300 para os dois) ou derrota (+0x94). A nota de cada um (0x48e190 com 0 / 1, a reputação do adversário, os pontos da
/// partida em +0x11f.. / +0x744..) vai para +0xb6 e +0xca; na liga do clube (+0x57 ou +0x60 = a competição +0x14) também
/// para os pontos do mês (+0xce); na seleção (0x53b900) para +0xbe, senão +0xba. Defeito da original, reproduzido: no
/// visitante, +0xbe/+0xba recebem o [esp+0x10] da rotina, que é a nota do mandante (ou o bônus dele, sem registro).
unsafe extern "thiscall" fn resultado_nos_registros_dos_tecnicos(this: usize, p: usize, m: usize) {
    if *((p + 0x3a) as *const u8) == 1 {
        return;
    }
    let (casa, fora) = (cqa11_d(p + 0x1c), cqa11_d(p + 0x20));
    let a = cqa11_registro(this, casa);
    let v = cqa11_registro(this, fora);
    if a != 0 {
        cqa11_mais1(a + 0x8e);
    }
    if v != 0 {
        cqa11_mais1(v + 0x8e);
    }
    let (ga, gv) = if *((p + 0x47) as *const u8) == 0xff {
        (cqa11_c(p + 0x43), cqa11_c(p + 0x44))
    } else {
        (cqa11_c(p + 0x47), cqa11_c(p + 0x48))
    };
    if a != 0 {
        cqa11_somar(a + 0x96, ga);
        cqa11_somar(a + 0x9a, gv);
    }
    if v != 0 {
        cqa11_somar(v + 0x96, gv);
        cqa11_somar(v + 0x9a, ga);
    }
    let (mut ba, mut bv) = (0i32, 0i32);
    if ga > gv {
        ba = 900; // cqa11: o bônus da vitória do mandante
        if a != 0 {
            crate::comparacao::ramo("cqa_11 vitoria do mandante");
            cqa11_mais1(a + 0x90);
        }
        if v != 0 {
            cqa11_mais1(v + 0x94);
        }
    } else if ga < gv {
        bv = 900;
        if v != 0 {
            cqa11_mais1(v + 0x90);
        }
        if a != 0 {
            cqa11_mais1(a + 0x94);
        }
    } else {
        ba = 300;
        bv = 300;
        if a != 0 {
            cqa11_mais1(a + 0x92);
        }
        if v != 0 {
            cqa11_mais1(v + 0x92);
        }
    }
    // a saída da 0x48e190 vai na vaga do argumento `jogo` da original, que começa com o próprio jogo
    let mut saida = p as u32;
    let mut guardado = ba; // o [esp+0x10] da original: o bônus do mandante, depois a nota dele
    let comp = cqa11_d(p + 0x14);
    if a != 0 {
        crate::comparacao::ramo("cqa_11 nota do mandante");
        let q = cqa11_nota(p, 0, &mut saida, fora, cqa11_pontos(m, 0x11f, 0x128, 0x127, 0x129).wrapping_add(ba));
        cqa11_somar(a + 0xb6, q);
        cqa11_somar(a + 0xca, q);
        guardado = q;
        if comp == cqa11_d(casa + 0x57) || comp == cqa11_d(casa + 0x60) {
            crate::comparacao::ramo("cqa_11 pontos do mes");
            cqa11_somar(a + 0xce, q); // cqa11: os pontos do mês do mandante
        }
        if e_selecao(casa) != 0 {
            cqa11_somar(a + 0xbe, q);
        } else {
            cqa11_somar(a + 0xba, q);
        }
    }
    if v != 0 {
        let q = cqa11_nota(p, 1, &mut saida, casa, cqa11_pontos(m, 0x744, 0x74d, 0x74c, 0x74e).wrapping_add(bv));
        cqa11_somar(v + 0xb6, q);
        cqa11_somar(v + 0xca, q);
        if comp == cqa11_d(fora + 0x57) || comp == cqa11_d(fora + 0x60) {
            cqa11_somar(v + 0xce, q);
        }
        if e_selecao(fora) != 0 {
            cqa11_somar(v + 0xbe, guardado);
        } else {
            cqa11_somar(v + 0xba, guardado);
        }
    }
}

/// Comparador lado a lado nos dois registros (0xd2 bytes), criados antes pela 0x86f5b0 como a original faria (a segunda
/// chamada acha o mesmo registro). Jogo não disputado: só a original (não cria nada).
unsafe extern "thiscall" fn comparar_resultado_nos_registros_dos_tecnicos(this: usize, p: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1449);
    if *((p + 0x3a) as *const u8) == 1 {
        return o(this, p, m);
    }
    let a = cqa11_registro(this, cqa11_d(p + 0x1c));
    let v = cqa11_registro(this, cqa11_d(p + 0x20));
    let mut reg = Vec::new();
    for r in [a, v] {
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, m), || resultado_nos_registros_dos_tecnicos(this, p, m));
    PLACAR.registrar(igual, || format!("resultado_nos_registros_dos_tecnicos(jogo {:#x}) original={:x?} rust={:x?}", p, mo, mr));
}

/// { 0x86a070 } (thiscall, ret 4, staff_records.cpp, SEH só da data local): `Staff_Records::add_league_finish_records(
/// liga)`, pela liga completa (0x689ad0), no fim da liga. Para cada linha da tabela (0x6826d0, até +0x3e), o histórico do
/// técnico do clube (+0xcf; tipo 0xa: o id do clube, a competição de +4, o ano +0x3a e a posição k + 1), no arquivo
/// (0x58e340).
unsafe extern "thiscall" fn posicoes_no_fim_da_liga(this: usize, liga: usize) {
    crate::comparacao::ramo("cqa_11 fim da liga");
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0xa;
    cpq4_por(&mut e, 0x11, cqa11_w(liga + 0x3a) as i32);
    cpq4_por(&mut e, 0xd, cqa11_i(cqa11_d(liga + 4)));
    let mut k: i16 = 0;
    while k < cqa11_w(liga + 0x3e) {
        let clube = cqa11_d(linha(liga, k as i32));
        cpq4_por(&mut e, 0x15, k as i32 + 1);
        cpq4_por(&mut e, 8, cqa11_i(clube));
        let t = cqa11_d(clube + 0xcf);
        if t != 0 {
            cpq4_historico(true, this, cqa11_d(t), &e);
        }
        k += 1;
    }
}

// ponytail: só grava o histórico no arquivo (não roda duas vezes): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_posicoes_no_fim_da_liga(this: usize, liga: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1450)(this, liga)
}

/// O ensaio do comparador da 0x86dc30: a cópia sem o histórico (arquivo).
static mut CQA11_ENSAIO: bool = false;

/// { 0x86dc30 } (thiscall, ret 0x10, staff_records.cpp, SEH só da data local): `Staff_Records::
/// add_supporters_player_of_year_record(pessoa, clube, a, b)`, pelo fim da temporada dos recordes (0x447880). O histórico
/// da pessoa (tipo 0x35: o id do clube, `a` em 16 bits sem sinal e `b`), no arquivo (0x58e340); depois, se sorteio(21 −
/// +0x59 da pessoa) < sorteio(10), a pessoa gosta do clube (0x547d90 com sorteio(2)) e o clube dela (0x547870 com
/// sorteio(2)). Pessoa nula: a original (a caixa e a leitura de +0x59 da pessoa nula).
unsafe extern "thiscall" fn jogador_do_ano_da_torcida_no_registro(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1451)(this, pessoa, clube, a, b);
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x35;
    cpq4_por(&mut e, 8, cqa11_i(clube));
    cpq4_por(&mut e, 0xd, (a & 0xffff) as i32);
    cpq4_por(&mut e, 0x11, b as i32);
    cpq4_historico(!CQA11_ENSAIO, this, cqa11_d(pessoa), &e);
    let r1 = cqa11_sortear(0x15 - cqa11_c(pessoa + 0x59));
    if r1 < cqa11_sortear(10) {
        crate::comparacao::ramo("cqa_11 torcida favorita");
        let gosta: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547d90usize);
        let favorita: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547870usize);
        gosta(pessoa, clube, cqa11_sortear(2) as u32);
        favorita(clube, pessoa, cqa11_sortear(2) as u32);
    }
}

/// Comparador em ensaio: a cópia sem o histórico e a original a partir do mesmo estado (sorteio, as pessoas favoritas e
/// rejeitadas do clube +0x9b e os clubes favoritos e rejeitados da pessoa, preferências +0x65: +4..+0x18). Pessoa nula:
/// só a original.
unsafe extern "thiscall" fn comparar_jogador_do_ano_da_torcida_no_registro(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1451);
    if pessoa == 0 || clube == 0 {
        return o(this, pessoa, clube, a, b);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube + 0x9b, 0x18));
    let pref = cqa11_d(pessoa + 0x65);
    if pref != 0 {
        reg.push((pref + 4, 0x18));
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| jogador_do_ano_da_torcida_no_registro(this, pessoa, clube, a, b)));
    CQA11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, pessoa, clube, a, b);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("jogador_do_ano_da_torcida_no_registro(pessoa {:#x}, clube {:#x}) original={:x?} rust={:x?}", pessoa, clube, mo, mr));
}

// ---------- 263a leva: os registros das copas e da troca de técnico: o vice, o terceiro e a passagem do técnico pelo clube (staff_records.cpp) ----------

unsafe fn cqa12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa12_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqa12_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// O ensaio dos comparadores: as cópias sem o histórico (arquivo) e sem acabar as convocações.
static mut CQA12_ENSAIO: bool = false;

/// A entrada `e` no fim da lista do clube `id` no arquivo do histórico dos clubes (FILE_LLIST [this+4], 0x58e340; o
/// começo e o fim em [this+0x10] + id × 8). No ensaio, nada.
unsafe fn cqa12_historico_do_clube(this: usize, id: usize, e: &[u8; 0x40]) {
    if CQA12_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
    let p = cqa12_d(this + 0x10).wrapping_add(id.wrapping_mul(8));
    f(cqa12_d(this + 4), p, p + 4, e.as_ptr());
}

/// Uma nota do lado do clube na partida (palavra em `a`): soma ftol(peso × k × [0x96ff88]) (produto em 32 bits) e fica
/// no piso peso × k (16 bits) quando está abaixo dele, senão no máximo 10000 (comparações com sinal em 16 bits).
unsafe fn cqa12_subir(a: usize, peso: i16, k: i32) {
    let soma = ftol((peso as i32).wrapping_mul(k) as f64 * f64_em(0x96ff88));
    let x = cqa12_w(a).wrapping_add(soma as i16);
    cqa12_pw(a, x);
    let piso = peso.wrapping_mul(k as i16);
    cqa12_pw(a, if piso > x { piso } else if x > 10000 { 10000 } else { x });
}

/// O vice (tipo 1: limiar 500, teto 350, notas 300/150) e o terceiro (tipo 2: 500, 200, 200/100; o limiar 500 conferido em 0x86b774) da copa: a reputação do
/// clube (+0x80) sobe até peso da competição (+0x69 de [jogo+0x14]) × limiar, no máximo `teto` de uma vez (entre 1 e
/// 10000); com técnico (+0xcf) que tem registro de não jogador (+0x69), as três notas do lado do clube na partida
/// (+0x188/+0x18a/+0x18c, lado 0x625 × visitante): peso × k, e na competição de fase 1 ([0xadadfc] +0x42 = 1) peso ×
/// metade na terceira e na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1). Os históricos (data,
/// tipo, id do clube, competição): do técnico, do presidente (+0xbf) e dos 20 da escalação do lado (+0x190, 0x49 bytes,
/// −1 vazio; pessoas válidas e fora o técnico); no terceiro, as convocações acabam nas competições guardadas (0x76d940,
/// 0x774f90 com os dois clubes); por fim o histórico do clube (tipo no +8, competição no +9).
unsafe fn cqa12_colocado(this: usize, clube: usize, jogo: usize, partida: usize, tipo: u8) {
    let (limiar, teto, k, metade) = if tipo == 1 { (500, 350, 300, 150) } else { (500, 200, 200, 100) };
    let comp = cqa12_d(jogo + 0x14);
    let peso = cqa12_w(comp + 0x69);
    let alvo = (peso as i32).wrapping_mul(limiar); // cqa12: o limiar da reputação
    let rep = cqa12_w(clube + 0x80) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > teto { teto } else { falta }; // cqa12: o teto da subida
        let v = cqa12_w(clube + 0x80).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if tipo == 1 {
            crate::comparacao::ramo("cqa_12 reputacao do vice");
        } else {
            crate::comparacao::ramo("cqa_12 reputacao do terceiro");
        }
        cqa12_pw(clube + 0x80, v); // cqa12: a reputação do colocado
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = tipo;
    cpq4_por(&mut e, 8, cqa12_i(clube));
    cpq4_por(&mut e, 0xd, cqa12_i(comp));
    let t = cqa12_d(clube + 0xcf);
    if t != 0 {
        if cqa12_d(t + 0x69) != 0 {
            let c = cqa12_d(cqa12_d(0xadadfc).wrapping_add(4 * cqa12_d(jogo)));
            let (k1, k2) = if *((c + 0x42) as *const i8) != 1 {
                (k, k)
            } else {
                let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
                (if estrangeiro(t, cqa12_d(clube + 0x53), 1) != 0 { metade } else { k }, metade)
            };
            let lado = partida + 0x188 + 0x625 * (cqa12_d(jogo + 0x20) == clube) as usize;
            cqa12_subir(lado, peso, k);
            cqa12_subir(lado + 2, peso, k1);
            cqa12_subir(lado + 4, peso, k2);
        }
        cpq4_historico(!CQA12_ENSAIO, this, cqa12_d(t), &e);
    }
    let presidente = cqa12_d(clube + 0xbf);
    if presidente != 0 {
        cpq4_historico(!CQA12_ENSAIO, this, cqa12_d(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != cqa12_d(jogo + 0x1c)) as usize;
    for j in 0..0x14usize {
        let id = cqa12_i(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = cqa12_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && cqa12_i(p) >= cqa12_i(0xae2364) {
            continue;
        }
        if p == cqa12_d(clube + 0xcf) {
            continue;
        }
        if p == 0 {
            aviso_em(0xa83b78, 0x1010);
            continue;
        }
        cpq4_historico(!CQA12_ENSAIO, this, cqa12_d(p), &e);
    }
    if tipo == 2 && !CQA12_ENSAIO {
        let guardada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x76d940usize);
        if guardada(cqa12_d(jogo + 0x14)) != 0 {
            let acabar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x774f90usize);
            acabar(cqa12_d(0xb74318), cqa12_d(jogo + 0x1c), cqa12_d(jogo + 0x14));
            acabar(cqa12_d(0xb74318), cqa12_d(jogo + 0x20), cqa12_d(jogo + 0x14));
        }
    }
    let mut e2 = cpq4_entrada();
    cpq4_datar(e2.as_mut_ptr());
    e2[8] = tipo;
    cpq4_por(&mut e2, 9, cqa12_i(comp));
    cqa12_historico_do_clube(this, cqa12_d(clube), &e2);
}

/// { 0x86b0b0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_runners_up_record(clube, jogo, partida)`, pela final das copas (Cup 0x519890, Liga dos Campeões 0x584ae0 e
/// as copas de cada nação): `cqa12_colocado` com o tipo 1. Clube, jogo ou partida nulos: a caixa (linha 0x5f6).
unsafe extern "thiscall" fn vice_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return aviso_em(0xa83b78, 0x5f6);
    }
    cqa12_colocado(this, clube, jogo, partida, 1);
}

/// { 0x86b710 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_third_place_record(clube, jogo, partida)`, pelas copas com disputa de terceiro: `cqa12_colocado` com o tipo
/// 2. Nulos: a caixa (linha 0x67b).
unsafe extern "thiscall" fn terceiro_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return aviso_em(0xa83b78, 0x67b);
    }
    cqa12_colocado(this, clube, jogo, partida, 2);
}

/// Comparador em ensaio dos colocados: a cópia sem os históricos e sem acabar as convocações, e a original a partir do
/// mesmo estado (a reputação do clube e as três notas do lado dele na partida). Nulos: só a original.
unsafe fn cqa12_comparar_colocado(nome: &str, clube: usize, jogo: usize, partida: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return original();
    }
    let reg = [(clube + 0x80, 2), (partida + 0x188 + 0x625 * (cqa12_d(jogo + 0x20) == clube) as usize, 6)];
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA12_ENSAIO = true;
    copia();
    CQA12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("{}(clube {:#x}, jogo {:#x}) original={:x?} rust={:x?}", nome, clube, jogo, mo, mr));
}

unsafe extern "thiscall" fn comparar_vice_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1452);
    cqa12_comparar_colocado("vice_da_copa_nos_registros", clube, jogo, partida, || o(this, clube, jogo, partida),
        || vice_da_copa_nos_registros(this, clube, jogo, partida))
}

unsafe extern "thiscall" fn comparar_terceiro_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1453);
    cqa12_comparar_colocado("terceiro_da_copa_nos_registros", clube, jogo, partida, || o(this, clube, jogo, partida),
        || terceiro_da_copa_nos_registros(this, clube, jogo, partida))
}

/// A data em `src` copiada para `dst` (0x417c10; a cópia de retorno vai para uma data local).
unsafe fn cqa12_copiar(dst: usize, src: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(dst, t.as_mut_ptr(), src);
}

/// Os dias da data `a` menos a `b` (0x54cdb0, `b` por valor; grava a data estática 0xae2470).
unsafe fn cqa12_dias(a: usize, b: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    f(a, cqa12_d(b) as u32, cqa12_d(b + 4) as u32)
}

/// { 0x869aa0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// record_manager_job_change(técnico, clube, contratado)`, pela saída do técnico (0x69b790) e pelo técnico novo
/// (0x694c80). Fora da seleção (0x53b900), no registro do técnico (0x86f5b0 com criar), quando o começo do jogo
/// ([0xdd6ec8]) não é depois de hoje: a passagem atual (+0x4a clube, +0x4e começo) fecha; a mais curta (+0x72, +0x76
/// começo, +0x7e fim) é trocada pela atual se não houver ou se a atual for mais curta (0x54cdb0), ou só tem o fim
/// renovado se for a mesma; a mais longa (+0x5e, +0x62, +0x6a) é trocada se a atual for mais longa (o fim vira o +0x56)
/// ou se não houver (o fim vira hoje). Depois +0x4a = −1 e, contratado = 1, a passagem nova (o id do clube, hoje,
/// +0x56 = 13/11/1975 da 0x549f70) e +1 em +0xb4. Na seleção, contratado = 1: +1 em +0xb5. Os históricos: do técnico
/// (tipo 0x96: clube, contratado), do presidente do clube (+0xbf; tipo 0x97 com o técnico e as três chaves) e do clube
/// (tipo 0x96 no +8, o técnico, as chaves e contratado). Técnico ou clube nulos: a caixa (linha 0x3b1).
unsafe extern "thiscall" fn troca_de_tecnico_nos_registros(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    if clube == 0 || tecnico == 0 {
        return aviso_em(0xa83b78, 0x3b1);
    }
    let contratado = contratado as u8 as i8;
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    if e_selecao(clube) == 0 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            let hoje = 0xae2c90usize;
            let (ai, ah) = (*((0xdd6ec8 + 2) as *const u16), *((hoje + 2) as *const u16));
            let depois = ai > ah || (ai == ah && cqa12_w(0xdd6ec8) > cqa12_w(hoje));
            if !depois {
                crate::comparacao::ramo("cqa_12 passagem fechada");
                if cqa12_i(r + 0x72) != -1 && cqa12_i(r + 0x4a) != -1 {
                    if cqa12_i(r + 0x72) == cqa12_i(r + 0x4a) && cqa12_w(r + 0x50) == cqa12_w(r + 0x78) && cqa12_w(r + 0x4e) == cqa12_w(r + 0x76) {
                        cqa12_copiar(r + 0x7e, hoje);
                    } else {
                        let atual = cqa12_dias(hoje, r + 0x4e);
                        if atual < cqa12_dias(r + 0x7e, r + 0x76) {
                            cqa12_pi(r + 0x72, cqa12_i(r + 0x4a));
                            cqa12_copiar(r + 0x76, r + 0x4e);
                            cqa12_copiar(r + 0x7e, hoje);
                        }
                    }
                } else {
                    cqa12_pi(r + 0x72, cqa12_i(r + 0x4a));
                    cqa12_copiar(r + 0x76, r + 0x4e);
                    cqa12_copiar(r + 0x7e, hoje);
                }
                if cqa12_i(r + 0x5e) != -1 && cqa12_i(r + 0x4a) != -1 {
                    let atual = cqa12_dias(hoje, r + 0x4e);
                    if atual > cqa12_dias(r + 0x6a, r + 0x62) {
                        cqa12_pi(r + 0x5e, cqa12_i(r + 0x4a));
                        cqa12_copiar(r + 0x62, r + 0x4e);
                        cqa12_copiar(r + 0x6a, r + 0x56);
                    }
                } else {
                    cqa12_pi(r + 0x5e, cqa12_i(r + 0x4a));
                    cqa12_copiar(r + 0x62, r + 0x4e);
                    cqa12_copiar(r + 0x6a, hoje);
                }
            }
            cqa12_pi(r + 0x4a, -1);
            if contratado == 1 {
                crate::comparacao::ramo("cqa_12 passagem nova");
                cqa12_pi(r + 0x4a, cqa12_i(clube));
                cqa12_copiar(r + 0x4e, hoje);
                let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
                let mut t = [0u8; 8];
                let d = data(t.as_mut_ptr(), 0xd, 0xb, 0x7b7, u32::MAX);
                cqa12_copiar(r + 0x56, d);
                *((r + 0xb4) as *mut u8) = (*((r + 0xb4) as *const u8)).wrapping_add(1); // cqa12: as passagens
            }
        }
    } else if contratado == 1 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            *((r + 0xb5) as *mut u8) = (*((r + 0xb5) as *const u8)).wrapping_add(1);
        }
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x96;
    cpq4_por(&mut e, 8, cqa12_i(clube));
    cpq4_por(&mut e, 0xd, contratado as i32);
    cpq4_historico(!CQA12_ENSAIO, this, cqa12_d(tecnico), &e);
    if cqa12_d(clube + 0xbf) != 0 {
        let mut e2 = cpq4_entrada();
        cpq4_datar(e2.as_mut_ptr());
        e2[0xc] = 0x97;
        cpq4_por(&mut e2, 8, cqa12_i(clube));
        cpq4_por(&mut e2, 0xd, contratado as i32);
        cpq4_quem(&mut e2, 0x11, tecnico);
        cpq4_historico(!CQA12_ENSAIO, this, cqa12_d(cqa12_d(clube + 0xbf)), &e2);
    }
    let mut e3 = cpq4_entrada();
    cpq4_datar(e3.as_mut_ptr());
    e3[8] = 0x96;
    cpq4_quem(&mut e3, 9, tecnico);
    cpq4_por(&mut e3, 0x19, contratado as i32);
    cqa12_historico_do_clube(this, cqa12_d(clube), &e3);
}

/// Comparador em ensaio: o registro do técnico criado antes (0x86f5b0, como a original faria nos dois caminhos que o
/// pedem), a cópia sem os históricos e a original a partir do mesmo estado (o registro, 0xd2 bytes, e a data estática da
/// 0x54cdb0). Nulos: só a original.
unsafe extern "thiscall" fn comparar_troca_de_tecnico_nos_registros(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1454);
    if clube == 0 || tecnico == 0 {
        return o(this, tecnico, clube, contratado);
    }
    let mut reg = vec![(0xae2470usize, 8usize)];
    if e_selecao(clube) == 0 || contratado as u8 == 1 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, tecnico, 1);
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA12_ENSAIO = true;
    troca_de_tecnico_nos_registros(this, tecnico, clube, contratado);
    CQA12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, tecnico, clube, contratado);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("troca_de_tecnico_nos_registros(tecnico {:#x}, clube {:#x}, {}) original={:x?} rust={:x?}", tecnico, clube, contratado as u8, mo, mr));
}

// ---------- 263a leva: as reações dos elencos à transferência: os contratos dos dois elencos, a comemoração da contratação e o companheiro desassossegado pela venda (media.cpp) ----------

unsafe fn cqa13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa13_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal em `a`.
unsafe fn cqa13_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cqa13_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A pessoa do id (0x6e bytes em [0xae23bc]).
unsafe fn cqa13_pessoa(id: i32) -> usize {
    cqa13_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize)
}

/// O contrato da pessoa com o clube (0x4cf5f0 em [0xae1050]), ou 0.
unsafe fn cqa13_contrato(pessoa: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cqa13_d(0xae1050), pessoa, clube)
}

/// A máscara da posição natural da pessoa (0x5481a0).
unsafe fn cqa13_mascara(pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    f(pessoa)
}

/// As marcas de evento do contrato (+0x40, 0x8676f0).
unsafe fn cqa13_bandeiras(c: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8676f0usize);
    f(c)
}

/// Apaga marcas de evento do contrato (0x8676d0).
unsafe fn cqa13_desmarcar(c: usize, marcas: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8676d0usize);
    f(c, marcas);
}

/// A reputação atual do jogador (+0xb da ficha +0x61).
unsafe fn cqa13_rep(pessoa: usize) -> i16 {
    cqa13_w(cqa13_d(pessoa + 0x61) + 0xb)
}

/// O ensaio do comparador da 0x758200: a cópia roda sem nada que aloque, dê notícia ou peça transferência; quando chega
/// a um desses, `CQA13_PAROU` e ela para (a comparação fica só com a original).
static mut CQA13_ENSAIO: bool = false;
static mut CQA13_PAROU: bool = false;

/// Liga marcas no contrato (STAFF_CONTRACT::set_flags 0x867560). No ensaio, pela cópia em ensaio dela (CPW7_ENSAIO): se
/// ela pediria transferência, para. Devolve se parou.
unsafe fn cqa13_marcas(c: usize, marcas: u32) -> bool {
    if CQA13_ENSAIO {
        CPW7_ENSAIO = true;
        CPW7_DECISAO = 0;
        marcas_do_contrato(c, marcas);
        CPW7_ENSAIO = false;
        if CPW7_DECISAO != 0 {
            CQA13_PAROU = true;
        }
        return CQA13_PAROU;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867560usize);
    f(c, marcas);
    false
}

/// A decisão de pedir transferência da 0x867700 com as `marcas` (sem efeito: o +0x45 volta): funções 0xb, 0xf e 0xd,
/// fora o técnico e o auxiliar do clube do contrato, moral (+0x44) não negativa, sem +0x49, quer sair (0x866f60, com as
/// marcas já ligadas), sem transferência futura (0x8cca10), sem os bits 1 e 8 de +0x4e, fora da fila da transferência
/// livre (0x8c1870) e com mais de 20 anos ou reputação acima de 4250.
unsafe fn cqa13_pediria(c: usize, marcas: u32) -> bool {
    let pessoa = cqa13_pessoa(cqa13_i(c));
    let funcao = *((pessoa + 0x3d) as *const u8);
    if funcao != 0xb && funcao != 0xf && funcao != 0xd {
        return false;
    }
    let clube = cqa13_d(0xae23b4).wrapping_add((cqa13_i(c + 4) as u32).wrapping_mul(0x245) as usize);
    for o in [0xcf, 0xd3] {
        let t = cqa13_d(clube + o);
        if t != 0 && cqa13_d(c) == cqa13_d(t) {
            return false;
        }
    }
    if cqa13_c(c + 0x44) < 0 || *((c + 0x49) as *const u8) != 0 {
        return false;
    }
    let antes = cqa13_d(c + 0x45);
    core::ptr::write_unaligned((c + 0x45) as *mut usize, antes | marcas as usize);
    let quer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    let sair = quer(c) != 0;
    core::ptr::write_unaligned((c + 0x45) as *mut usize, antes);
    if !sair {
        return false;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if futura(cqa13_d(0xde1f54), pessoa, 0) != 0 || *((c + 0x4e) as *const u8) & 9 != 0 {
        return false;
    }
    let na_fila: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c1870usize);
    if na_fila(cqa13_d(0xde1f54), pessoa) != 0 {
        return false;
    }
    !(cqa13_c(pessoa + 0x18) <= 0x14 && cqa13_rep(pessoa) <= 0x109a)
}

/// As queixas do contrato (STAFF_CONTRACT::apply_event_flags 0x867700 com a pessoa). No ensaio, para antes se ela
/// pediria transferência. Devolve se parou.
unsafe fn cqa13_eventos(c: usize, marcas: u32, pessoa: usize) -> bool {
    if CQA13_ENSAIO && cqa13_pediria(c, marcas) {
        CQA13_PAROU = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x867700usize);
    f(c, marcas, pessoa);
    false
}

/// Tira queixas do contrato (STAFF_CONTRACT::clear_grievances 0x8674e0). No ensaio, para antes se o pedido de
/// transferência seria retirado (a decisão `cph1_retira` com as queixas já tiradas). Devolve se parou.
unsafe fn cqa13_limpar(c: usize, queixas: u32) -> bool {
    if CQA13_ENSAIO {
        let antes = cqa13_d(c + 0x45);
        core::ptr::write_unaligned((c + 0x45) as *mut usize, antes & !(queixas as usize));
        let retira = cph1_retira(c).is_some();
        core::ptr::write_unaligned((c + 0x45) as *mut usize, antes);
        if retira {
            CQA13_PAROU = true;
            return true;
        }
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
    f(c, queixas);
    false
}

/// A comemoração da contratação (0x75b6e0); no ensaio, para.
unsafe fn cqa13_comemorar(this: usize, p: usize, clube: usize, tipo: u32, porta_voz: usize) -> bool {
    if CQA13_ENSAIO {
        CQA13_PAROU = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) = core::mem::transmute(0x75b6e0usize);
    f(this, p, clube, tipo, 1, porta_voz);
    false
}

/// O porta-voz do elenco: entre os 50 do clube com ficha, fora `p`, com reputação acima de 4250, o maior sorteio(reputação
/// + 20 × (+0x5b + +0x59) ± pela idade: abaixo de 19 −1500, de 23 −750, de 28 −250, de 34 +250), acima de 0.
unsafe fn cqa13_porta_voz(clube: usize, p: usize) -> usize {
    let (mut melhor, mut quem) = (0i32, 0usize);
    for j in 0..50usize {
        let m = cqa13_d(clube + 0xd7 + 4 * j);
        if m == 0 || cqa13_d(m + 0x61) == 0 || m == p {
            continue;
        }
        let r = cqa13_rep(m);
        if r <= 0x109a {
            continue;
        }
        let mut x = (r as i32).wrapping_add((cqa13_c(m + 0x5b) + cqa13_c(m + 0x59)) * 20);
        let idade = cqa13_c(m + 0x18);
        x += if idade < 0x13 {
            -0x5dc
        } else if idade < 0x17 {
            -0x2ee
        } else if idade < 0x1c {
            -0xfa
        } else if idade < 0x22 {
            0xfa
        } else {
            0
        };
        let s = cqa13_sortear(x);
        if s > melhor {
            melhor = s;
            quem = m;
        }
    }
    quem
}

/// { 0x758200 } (thiscall, ret 0x10, media.cpp): `Media::process_transfer_squad_reactions(jogador, novo, velho, flag)`,
/// no fim de cada transferência (TRANSFER_MANAGER::move_player_to_club, 0x8ca879). Só para o jogador (ficha +0x61) de 19
/// anos ou mais com reputação de 4250. (A) A história da mídia na frente da fila (0xbd0, +0x11 = 0x25, novo clube de id
/// 0x25; nunca na prática): queixas no contrato da pessoa dela. (B) No elenco do novo clube (50 vagas +0xd7), quem está
/// lá há mais de +0x5a + 15 dias (0x54cdb0 de +0x3e) e tem contrato: a marca 0x800 com a mesma posição troca por queixa
/// (0x867700 0x80000000) e sorteios; a 0x400 sai com o elenco forte (0x53c590 até o limite pela reputação); com mais de 2
/// da posição, o jogador forte pode virar queixa ou a marca 0x200; a queixa 0x20000000 sai (0x8674e0) quando chega
/// alguém maior. Promessa de craque (+0xd do jogador acima de 6250): a torcida (+8 da linha do clube em [[0xb67a44]],
/// +500 até 10000) e a comemoração (0x75b6e0); se algo mudou, sorteio(3) e o porta-voz comemoram. (C) No elenco do velho
/// clube (com liga), com a força e a reputação do clube (ou a nação entre as sete de 0x9cf31c..): as queixas 0x40000000 /
/// 0x20000000 pela saída do companheiro, a moral −25, a marca 0x400, as 0x200/0x800 e a queixa 0x80000000 tiradas; se
/// algo mudou e o jogador era craque, o companheiro desassossegado (0x75cfc0).
unsafe extern "thiscall" fn reacoes_dos_elencos_a_transferencia(this: usize, p: usize, novo: usize, velho: usize, flag: u32) {
    if p == 0 || novo == 0 {
        return;
    }
    let ficha = cqa13_d(p + 0x61);
    if ficha == 0 || cqa13_c(p + 0x18) < 0x13 || cqa13_w(ficha + 0xb) < 0x109a {
        return;
    }
    let mut mudou = false;
    // (A) a história da mídia na frente da fila
    if cqa13_i(this + 8) > 0 {
        let h = cqa13_d(this);
        if cqa13_i(h + 0x13) == 0xbd0 && cqa13_w(h + 0x11) == 0x25 && cqa13_i(novo) == 0x25 {
            let tipo = *((h + 0x10) as *const u8);
            let id = cqa13_i(h + 8);
            if id != -1 {
                let q = cqa13_pessoa(id);
                let rp = cqa13_rep(p);
                if cqa13_mascara(p) == cqa13_mascara(q) && (rp > 0x128e || rp as i32 - 0x5dc > cqa13_rep(q) as i32) {
                    let c = cqa13_contrato(q, novo);
                    if c != 0 {
                        crate::comparacao::ramo("cqa_13 historia da midia");
                        mudou = true;
                        if tipo == 1 {
                            if cqa13_eventos(c, 0x8000000, 0) || cqa13_eventos(c, 0x80000000, p) {
                                return;
                            }
                        } else if tipo == 2 {
                            if cqa13_bandeiras(c) & 0x800 != 0
                                && (cqa13_marcas(c, 0x200) || cqa13_eventos(c, 0x80000000, p) || cqa13_marcas(c, 0x80))
                            {
                                return;
                            }
                        } else if cqa13_sortear(cqa13_c(q + 0x5b)) < cqa13_sortear(10) {
                            if cqa13_marcas(c, 0x80) {
                                return;
                            }
                            if cqa13_sortear(cqa13_c(q + 0x5a)) == 0 && cqa13_eventos(c, 0x80000000, p) {
                                return;
                            }
                        }
                        if cqa13_bandeiras(c) & 0x800 != 0 {
                            cqa13_desmarcar(c, 0x800);
                        }
                    }
                }
            }
        }
    }
    // (B) o elenco do novo clube
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let mesma_posicao: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x548150usize);
    let forca = contar(novo, 0, 0) as i8 as i32;
    let rn = cqa13_w(novo + 0x80);
    let limite = if rn < 0x9c4 { 0x14 } else if rn < 0x1482 { 0x19 } else if rn >= 0x1c52 { 0x20 } else { 0x1e }; // cqa13: o limite do elenco forte
    let mesma = mesma_posicao(novo, cqa13_mascara(p)) as i8 as i32;
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let categoria: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    for j in 0..50usize {
        let m = cqa13_d(novo + 0xd7 + 4 * j);
        if m == 0 || cqa13_d(m + 0x61) == 0 || m == p {
            continue;
        }
        if dias(0xae2c90, cqa13_d(m + 0x3e) as u32, cqa13_d(m + 0x42) as u32) <= cqa13_c(m + 0x5a) + 0xf {
            continue;
        }
        let c = cqa13_contrato(m, novo);
        if c == 0 {
            continue;
        }
        if cqa13_bandeiras(c) & 0x800 != 0 && cqa13_mascara(p) == cqa13_mascara(m) {
            cqa13_desmarcar(c, 0x800);
            if cqa13_eventos(c, 0x80000000, p) {
                return;
            }
            if cqa13_sortear(cqa13_c(m + 0x5a)) == 0 && cqa13_marcas(c, 0x200) {
                return;
            }
            if cqa13_sortear(cqa13_c(m + 0x5b)) == 0 && cqa13_marcas(c, 0x80) {
                return;
            }
        }
        if cqa13_bandeiras(c) & 0x400 != 0 && forca >= limite {
            crate::comparacao::ramo("cqa_13 elenco forte");
            cqa13_desmarcar(c, 0x400);
        }
        if mesma > 2 && cqa13_bandeiras(c) & 0x400 == 0 && cqa13_rep(m) > 0x109a && cqa13_mascara(p) == cqa13_mascara(m) {
            let rp = cqa13_rep(p);
            if rp > 0x128e || rp as i32 - 0x5dc > cqa13_w(novo + 0x80) as i32 {
                let a = cqa13_c(m + 0x5a);
                if a < 0xf && cqa13_c(m + 0x5b) < 0xf && a < mesma + 4 && cqa13_c(m + 0x57) < mesma + 8 && cqa13_sortear(a) < cqa13_sortear(6) {
                    let parou = if categoria(m, c) == 1 || categoria(m, c) == 2 {
                        cqa13_eventos(c, 0x80000000, p)
                    } else {
                        cqa13_marcas(c, 0x200)
                    };
                    if parou {
                        return;
                    }
                }
            }
        }
        if cqa13_i(c + 0x45) & 0x20000000 != 0 {
            let rp = cqa13_rep(p);
            if rp > 0x1482 || rp as i32 - 0x4e2 > cqa13_w(novo + 0x80) as i32 {
                mudou = true;
                if forca > 0x28 || cqa13_sortear(cqa13_c(m + 0x5a)) != 0 {
                    crate::comparacao::ramo("cqa_13 queixa limpa");
                    if cqa13_limpar(c, 0x20000000) { // cqa13: chega alguém maior
                        return;
                    }
                }
            }
        }
    }
    let ficha = cqa13_d(p + 0x61);
    if cqa13_w(ficha + 0xd) > 0x186a {
        let rp = cqa13_w(ficha + 0xb);
        if rp as i32 + 0x2ee > cqa13_w(novo + 0x80) as i32 || rp > 0x222e {
            let linha = cqa13_d(cqa13_d(0xb67a44)).wrapping_add((cqa13_i(novo) as u32).wrapping_mul(0x49) as usize);
            let x = cqa13_w(linha + 8).wrapping_add(0x1f4); // cqa13: a torcida comemora
            let x = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            crate::comparacao::ramo("cqa_13 torcida comemora");
            core::ptr::write_unaligned((linha + 8) as *mut i16, x);
            if cqa13_comemorar(this, p, novo, 1, 0) {
                return;
            }
        }
    }
    if mudou {
        let ficha = cqa13_d(p + 0x61);
        if cqa13_w(ficha + 0xd) > 0x186a
            && cqa13_w(ficha + 0xb) as i32 + 0x2ee > cqa13_w(novo + 0x80) as i32
            && cqa13_c(p + 0x5b) > 0xa
            && cqa13_c(p + 0x59) > 6
            && cqa13_sortear(3) != 0
        {
            let quem = cqa13_porta_voz(novo, p);
            if quem != 0 && cqa13_comemorar(this, p, novo, 2, quem) {
                return;
            }
        }
    }
    // (C) o elenco do velho clube
    let mut mudou = false;
    if velho == 0 || cqa13_d(velho + 0x57) == 0 {
        return;
    }
    if cqa13_d(cqa13_d(0xadadfc).wrapping_add(4 * cqa13_d(cqa13_d(velho + 0x57)))) == 0 {
        return;
    }
    let forca = contar(velho, 0, 0) as i8 as i32;
    let nacao = cqa13_d(velho + 0x53);
    let sete = nacao != 0 && [0x9cf31c, 0x9cf4a8, 0x9cf510, 0x9cf2fc, 0x9cf2dc, 0x9cf3e4, 0x9cf224].iter().any(|&k| cqa13_d(nacao) == cqa13_d(k));
    let rv = cqa13_w(velho + 0x80);
    let (lim1, lim2) = if sete || rv < 0x1482 {
        (0x19, 0x2d)
    } else if rv < 0x1c52 {
        (0x1e, 0x2d)
    } else {
        (0x23, 0x2e)
    };
    let mesma = mesma_posicao(velho, cqa13_mascara(p)) as i8 as i32;
    let candidato: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x548810usize);
    for j in 0..50usize {
        let m = cqa13_d(velho + 0xd7 + 4 * j);
        if m == 0 || cqa13_d(m + 0x61) == 0 || m == p {
            continue;
        }
        let c = cqa13_contrato(m, velho);
        if c == 0 {
            continue;
        }
        let mut q: i32 = if cqa13_i(c + 0x45) != 0 { 3 } else { 0 };
        if cqa13_bandeiras(c) != 0 {
            q += 1;
        }
        if cqa13_c(c + 0x44) < -0x19 {
            q += 1;
        }
        let q = q as i8 as i32;
        let mut queixa = 0u32; // 0x40000000 ou 0x20000000 pela saída do companheiro
        let mut segundo = true;
        if flag as u8 != 0
            && cqa13_bandeiras(c) & 0x200 == 0
            && cqa13_bandeiras(c) & 0x800 == 0
            && cqa13_i(c + 0x45) as u32 & 0x80000000 == 0
            && (forca < 0x28 || mesma < 6)
            && (cqa13_mascara(p) != cqa13_mascara(m) || mesma < 2)
        {
            let rp = cqa13_rep(p);
            if rp > 0x1e46 || rp > cqa13_w(novo + 0x80).wrapping_sub(0x2ee) {
                let r = cqa13_sortear(6);
                if cqa13_c(m + 0x57) + q > r + 10 && cqa13_c(c + 0x3d) > 0x19 && cqa13_rep(m) > 0x109a
                    && cqa13_sortear(cqa13_c(m + 0x5b)) < q + 5 && cqa13_sortear(cqa13_c(m + 0x5a)) == 0
                {
                    segundo = false;
                    queixa = if cqa13_c(p + 0x18) < 0x18 && cqa13_w(cqa13_d(p + 0x61) + 0xd) < 0x128e { 0x40000000 } else { 0x20000000 };
                }
            }
        }
        if segundo
            && cqa13_c(p + 0x18) > 0x17
            && flag as u8 != 0
            && candidato(m, novo, p) != 0
            && cqa13_mascara(p) != cqa13_mascara(m)
            && cqa13_sortear(cqa13_c(m + 0x5b)) < q + 5
        {
            let rp = cqa13_rep(p);
            if rp > 0x1e46 || rp > cqa13_w(novo + 0x80).wrapping_sub(0x2ee) {
                queixa = 0x20000000;
            } else {
                crate::comparacao::ramo("cqa_13 moral cai");
                let x = cqa13_c(c + 0x44) - 0x19; // cqa13: a moral do companheiro cai
                *((c + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { x as i8 };
            }
        }
        if queixa != 0 {
            if cqa13_eventos(c, queixa, p) {
                return;
            }
            mudou = true;
        }
        if cqa13_c(m + 0x18) > 0x17 && cqa13_bandeiras(c) & 0x400 == 0 && cqa13_c(m + 0x57) > 0xf {
            let r = (cqa13_sortear(10) as u8).wrapping_add(10) as i8;
            if *((m + 0x58) as *const i8) > r && cqa13_c(m + 0x5b) > 0xa && cqa13_rep(p) > 0x109a && cqa13_rep(m) > 0x109a && forca < lim1 && cqa13_marcas(c, 0x400) {
                return;
            }
        }
        let iguais = || cqa13_mascara(p) == cqa13_mascara(m);
        if (forca < lim2 || mesma < 2) && iguais() && cqa13_bandeiras(c) & 0x200 != 0 {
            cqa13_desmarcar(c, 0x200);
        }
        let tirar_800 = if forca < lim2 {
            let r1 = cqa13_sortear(cqa13_c(p + 0x5a));
            (r1 > cqa13_sortear(10) && iguais()) || mesma < 2
        } else {
            mesma < 2
        };
        if tirar_800 && cqa13_bandeiras(c) & 0x800 != 0 {
            cqa13_desmarcar(c, 0x800);
        }
        if ((forca < lim2 && iguais()) || mesma < 2) && cqa13_i(c + 0x45) as u32 & 0x80000000 != 0 && cqa13_limpar(c, 0x80000000) {
            return;
        }
        if cqa13_bandeiras(c) & 0x400 != 0 {
            if cqa13_i(c + 0x45) as u32 & 0x80000000 != 0 && cqa13_limpar(c, 0x80000000) {
                return;
            }
            if cqa13_bandeiras(c) & 0x800 != 0 {
                cqa13_desmarcar(c, 0x800);
            }
            if cqa13_bandeiras(c) & 0x200 != 0 {
                cqa13_desmarcar(c, 0x200);
            }
        }
    }
    if !mudou || cqa13_c(p + 0x18) >= 0x23 {
        return;
    }
    let ficha = cqa13_d(p + 0x61);
    if cqa13_w(ficha + 5) > 0x64
        && cqa13_w(ficha + 0xd) > 0x186a
        && cqa13_w(ficha + 0xb) as i32 + 0x2ee > cqa13_w(velho + 0x80) as i32
        && cqa13_c(p + 0x5b) > 0xa
        && cqa13_c(p + 0x59) > 6
        && cqa13_sortear(3) != 0
    {
        let quem = cqa13_porta_voz(velho, p);
        if quem != 0 {
            if CQA13_ENSAIO {
                CQA13_PAROU = true;
                return;
            }
            let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) = core::mem::transmute(0x75cfc0usize);
            f(this, velho, novo, p, quem);
        }
    }
}

/// Comparador em ensaio: a cópia sem notícia nem pedido de transferência (para quando chega a um) e a original a partir
/// do mesmo estado: o sorteio, [0xae1080] (a ficha do último contrato), a data estática da 0x54cdb0, a linha da torcida
/// do novo clube em [[0xb67a44]] e, de cada um dos elencos dos dois clubes (e da pessoa da história da mídia), o contrato
/// com o clube (0x50 bytes) e a linha de opiniões em [0xae2d0c] (0x4f bytes). Quando a cópia para: só a original.
unsafe extern "thiscall" fn comparar_reacoes_dos_elencos_a_transferencia(this: usize, p: usize, novo: usize, velho: usize, flag: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(1455);
    if p == 0 || novo == 0 || cqa13_d(p + 0x61) == 0 || cqa13_c(p + 0x18) < 0x13 || cqa13_rep(p) < 0x109a {
        return o(this, p, novo, velho, flag);
    }
    let ficha = cqa13_d(0xae1080);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    reg.push((0xae2470, 8));
    if cqa13_d(0xb67a44) != 0 {
        reg.push((cqa13_d(cqa13_d(0xb67a44)).wrapping_add((cqa13_i(novo) as u32).wrapping_mul(0x49) as usize), 0x49));
    }
    let mut juntar = |m: usize, clube: usize| {
        if m == 0 {
            return;
        }
        let c = cqa13_contrato(m, clube);
        if c != 0 {
            reg.push((c, 0x50));
        }
        reg.push((cqa13_d(0xae2d0c).wrapping_add((cqa13_i(m) as u32).wrapping_mul(0x4f) as usize), 0x4f));
    };
    for clube in [novo, velho] {
        if clube != 0 {
            for j in 0..50usize {
                juntar(cqa13_d(clube + 0xd7 + 4 * j), clube);
            }
        }
    }
    if cqa13_i(this + 8) > 0 && cqa13_i(cqa13_d(this) + 8) != -1 {
        juntar(cqa13_pessoa(cqa13_i(cqa13_d(this) + 8)), novo);
    }
    core::ptr::write_unaligned(0xae1080 as *mut usize, ficha); // os contratos lidos acima não contam
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA13_ENSAIO = true;
    CQA13_PAROU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| reacoes_dos_elencos_a_transferencia(this, p, novo, velho, flag)));
    CQA13_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    if CQA13_PAROU {
        crate::comparacao::ramo("cqa_13 reacoes so na original");
        return o(this, p, novo, velho, flag);
    }
    o(this, p, novo, velho, flag);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("reacoes_dos_elencos_a_transferencia(jogador {:#x}, novo {:#x}, velho {:#x}) diferencas em {:x?}", p, novo, velho, dif)
    });
}

/// { 0x75b6e0 } (thiscall, ret 0x14, media.cpp, quadro SEH da notícia local): `Media::queue_signing_celebration_story(
/// jogador, clube, tipo, a, porta_voz)`, pelas reações à transferência (0x758200). Jogador com +0x1a, clube atual
/// (+0x39) com técnico (+0xcf) e +0x69, ficha (+0x61); tipo 2 sem porta-voz: nada. A notícia 0xbe5: o jogador (id e as
/// três chaves), o id do clube dele, sorteio(2), o veículo (0x757fa0), o comentarista (0x7dab90 com o clube, o veículo e
/// a nação +0x53 ou −1), as posições (0x5b7d20), o tipo, a resposta do técnico da máquina (0x735870 com 0x82; 0 com
/// técnico humano), no tipo 2 o porta-voz (id e chaves), a reputação do clube acima de 7250, o técnico (id e chaves) e
/// `a`; vai ao técnico do clube do jogador (0x7861e0).
unsafe extern "thiscall" fn noticia_da_comemoracao_da_contratacao(this: usize, p: usize, clube: usize, tipo: u32, a: u32, porta_voz: usize) {
    if p == 0 || cqa13_d(p + 0x1a) == 0 {
        return;
    }
    let atual = cqa13_d(p + 0x39);
    if atual == 0 || cqa13_d(atual + 0xcf) == 0 || cqa13_d(atual + 0x69) == 0 || cqa13_d(p + 0x61) == 0 {
        return;
    }
    if porta_voz == 0 && tipo as u8 == 2 {
        return;
    }
    crate::comparacao::ramo("cqa_13 noticia da comemoracao");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x757fa0usize);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(0x7dab90usize);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let resposta: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x735870usize);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbe5, 0);
    gravar(nt, 0, cqa13_d(p) as u32);
    for k in 1..4usize {
        gravar(nt, k as i8, cqa13_d(cqa13_d(p + 4 * k) + 0x33) as u32);
    }
    gravar(nt, 4, cqa13_d(cqa13_d(p + 0x39)) as u32);
    gravar(nt, 5, cqa13_sortear(2) as u32);
    let v = veiculo(p);
    gravar(nt, 6, v);
    let nacao = cqa13_d(clube + 0x53);
    gravar(nt, 7, comentarista(cqa13_d(0xb74430), clube, v, if nacao != 0 { cqa13_i(nacao) } else { -1 }) as u32);
    gravar(nt, 8, posicoes(cqa13_d(p + 0x61)) as i16 as i32 as u32);
    gravar(nt, 9, tipo as u8 as i8 as i32 as u32);
    let mut r = 0u32;
    if humano(cqa13_d(p + 0x39), 0, 1, 0, 0) == 0 {
        r = resposta(this, cqa13_d(cqa13_d(p + 0x39) + 0xcf), p, 0x82);
    }
    gravar(nt, 10, r);
    let mut k: i8 = 0xb;
    if tipo as u8 == 2 {
        gravar(nt, 0xb, cqa13_d(porta_voz) as u32);
        for j in 1..4usize {
            gravar(nt, 0xb + j as i8, cqa13_d(cqa13_d(porta_voz + 4 * j) + 0x33) as u32);
        }
        k = 0xf;
    }
    gravar(nt, k, (cqa13_w(clube + 0x80) > 0x1c52) as u32);
    let t = cqa13_d(cqa13_d(p + 0x39) + 0xcf);
    gravar(nt, k + 1, cqa13_d(t) as u32);
    for j in 1..4usize {
        gravar(nt, k + 1 + j as i8, cqa13_d(cqa13_d(t + 4 * j) + 0x33) as u32);
    }
    // ponytail: a original empurra o índice num dword de bytes de cima sem valor; a 0x785790 lê o char
    gravar(nt, k + 5, a as u8 as i8 as i32 as u32);
    entregar(cqa13_d(0xb7431c), nt, cqa13_d(p + 0x39));
}

// ponytail: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_comemoracao_da_contratacao(this: usize, p: usize, clube: usize, tipo: u32, a: u32, porta_voz: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize)>(1456)(this, p, clube, tipo, a, porta_voz)
}

/// { 0x75cfc0 } (thiscall, ret 0x10, media.cpp, quadro SEH da notícia local): `Media::queue_sale_unrest_story(velho,
/// novo, jogador, companheiro)`, pelas reações à transferência (0x758200). Velho clube com técnico (+0xcf), fora da
/// seleção (0x53b900), companheiro e jogador com ficha (+0x61). A notícia 0xbe4: o técnico, o companheiro e o jogador
/// (id e as três chaves de cada), os ids dos dois clubes, sorteio(2) (+2 se +0x5b do companheiro não passar de
/// sorteio(5) + 10), o veículo (0x757f30), o comentarista (0x7dab90 com a nação +0x53 ou −1) e 2; vai ao técnico do velho
/// clube (0x7861e0).
unsafe extern "thiscall" fn noticia_do_companheiro_desassossegado(_this: usize, velho: usize, novo: usize, p: usize, comp: usize) {
    if velho == 0 || cqa13_d(velho + 0xcf) == 0 || e_selecao(velho) != 0 {
        return;
    }
    if comp == 0 || cqa13_d(comp + 0x61) == 0 || p == 0 || cqa13_d(p + 0x61) == 0 {
        return;
    }
    crate::comparacao::ramo("cqa_13 noticia do desassossego");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x757f30usize);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(0x7dab90usize);
    let mut local = [0u32; 0x3a];
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbe4, 0);
    let mut k: i8 = 0;
    for quem in [cqa13_d(velho + 0xcf), comp, p] {
        gravar(nt, k, cqa13_d(quem) as u32);
        for j in 1..4usize {
            gravar(nt, k + j as i8, cqa13_d(cqa13_d(quem + 4 * j) + 0x33) as u32);
        }
        k += 4;
    }
    gravar(nt, 0xc, cqa13_d(velho) as u32);
    gravar(nt, 0xd, cqa13_d(novo) as u32);
    let r = cqa13_sortear(5);
    let s = if cqa13_c(comp + 0x5b) > r + 10 { cqa13_sortear(2) } else { cqa13_sortear(2) + 2 };
    gravar(nt, 0xe, s as u32);
    let v = veiculo(velho);
    gravar(nt, 0xf, v);
    let nacao = cqa13_d(velho + 0x53);
    gravar(nt, 0x10, comentarista(cqa13_d(0xb74430), velho, v, if nacao != 0 { cqa13_i(nacao) } else { -1 }) as u32);
    gravar(nt, 0x11, 2);
    entregar(cqa13_d(0xb7431c), nt, velho);
}

// ponytail: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_companheiro_desassossegado(this: usize, velho: usize, novo: usize, p: usize, comp: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(1457)(this, velho, novo, p, comp)
}
