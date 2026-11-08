// ---------- %%LEVA%%a leva: o match_wrap_up do motor (5): a narração combinada e o comentário de contexto ----------

/// Os rótulos de desvio da 0x6d60b0 (o fim de cada um devolve ou segue para outro).
#[derive(Clone, Copy)]
enum Rotulo {
    R6cde,
    R6d1d,
    R69a6,
    R69f5,
    R6c95,
    R6cbf,
    Copiar,
}

/// { 0x6d60b0 } (thiscall, ret 8, match_events.cpp): combina o evento novo `e` (14 bytes: palavra do
/// evento; +5, +6 jogador e lado; +7, +8 o outro; +9 inteiro) com o último da fila (posição +8 − 1
/// no anel de 12, registros de 14 bytes em +0x30): devolve o evento combinado (negativo = troca o
/// anterior; 0 = sem combinação) e, nas combinações, copia o jogador, o lado, o outro e o inteiro do
/// anterior para `e`. `m` é o motor (0 fora da partida): dá o lado da posse (+0x8eab) e a bola na
/// área (0x6d58f0). As duas chamadas da 0x6cb960 do original não têm efeito e o valor delas é
/// descartado (ficam de fora).
unsafe extern "thiscall" fn combine_with_previous(t: usize, e: usize, m: usize) -> i16 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let b = |p: usize, o: usize| *((p + o) as *const i8);
    let w = |p: usize| core::ptr::read_unaligned(p as *const i16);
    let s2 = w(e);
    let (posse, area) = if m == 0 { (0, 0) } else { (b(m, 0x8eab), ball_goal_end(m)) };
    if w(t + 8) == w(t + 6) {
        return 0;
    }
    let k = if w(t + 8) == 0 { 0xb } else { b(t, 8).wrapping_sub(1) };
    let q = (t as isize + 0x30 + k as isize * 14) as usize;
    let s3 = w(q);
    let igual = |o: usize| b(q, o) == b(e, o);
    let copiar = || core::ptr::copy((q as *const u8).add(5), (e as *mut u8).add(5), 8);
    // o anterior passa a ser o principal e o novo vira o outro (+7, +8)
    let trocar = |inteiro: bool| {
        *((e + 8) as *mut i8) = b(e, 6);
        *((e + 7) as *mut i8) = b(e, 5);
        *((e + 5) as *mut i8) = b(q, 5);
        *((e + 6) as *mut i8) = b(q, 6);
        if inteiro {
            core::ptr::write_unaligned((e + 9) as *mut u32, core::ptr::read_unaligned((q + 9) as *const u32));
        }
    };
    let contexto = |v: i16| (0x2021..=0x20d0).contains(&v);

    if s2 == 0x1f6f {
        if posse == b(e, 6) {
            return 0x21bb;
        }
    } else if s2 == 0x1f92 && (s3 == 0x1f6c || s3 == 0x1f6e) {
        return -0x1f9c;
    }
    if s3 == 0x1f7d && b(e, 6) != b(q, 6) && s2 == 0x1f46 {
        return -0x219d;
    }
    let shot = (0x1f87..=0x1fbd).contains(&s2);
    if (shot || (s2 == 0x1f5f && area != 0)) && s3 == 0x1f7e {
        core::ptr::write_unaligned(q as *mut i16, 0x219c);
        core::ptr::write_unaligned((q + 2) as *mut i16, sorteio(2) as i16);
        return 0;
    }
    if shot && s3 == 8000 {
        core::ptr::write_unaligned(q as *mut i16, 0x219b);
        return 0;
    }

    let mut r = 'escolha: {
        if s3 == 0x1f44 {
            if s2 == 0x1f43 {
                copiar();
                return 0x213f;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x1f69 {
            if s2 == 0x1f9d {
                return if sorteio(3) != 0 { 0 } else { 0x2198 };
            }
            break 'escolha Rotulo::R6cde;
        }
        if contexto(s3) && matches!(s2, 0x1f46 | 0x213e | 0x1f60 | 0x1f5f) && b(e, 6) != b(q, 6) {
            let lado = b(q, 6);
            if (area == -1 && lado == 1) || (area == 1 && lado == 0) {
                return -0x2178;
            }
        }
        if s3 == 0x1f6f {
            if s2 == 0x1f46 {
                if igual(6) || area == 0 {
                    return 0;
                }
                trocar(false);
                return 0x2177;
            }
            break 'escolha Rotulo::R6cde;
        }
        for (de, para) in [(0x2111, 0x2140), (0x210f, 0x2141), (0x1fe6, 0x2142), (0x1fe7, 0x2142)] {
            if s3 == de {
                if s2 == 0x1f46 {
                    if !igual(5) || !igual(6) {
                        return 0;
                    }
                    copiar();
                    return para;
                }
                break 'escolha Rotulo::R6cde;
            }
        }
        if (s3 == 0x213e || s3 == 0x1f71) && s2 == 0x1f46 && igual(5) && igual(6) {
            copiar();
            return 0x1f46;
        }
        if s3 == 0x213e {
            if s2 == 0x1f41 {
                copiar();
                return 0x213e;
            }
            break 'escolha Rotulo::R69a6;
        }
        if s3 == 0x1f41 {
            if s2 == 0x1f46 {
                copiar();
                return 0x1f46;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x2113 {
            if (0x1f86..=0x1fbe).contains(&s2) && sorteio(3) == 0 && igual(5) && igual(6) {
                copiar();
                return s2;
            }
            break 'escolha Rotulo::R6c95;
        }
        if s3 == 0x1f47 {
            if s2 == 0x1f44 || s2 == 0x1f43 {
                if !igual(5) || !igual(6) {
                    return 0;
                }
                copiar();
                return s2;
            }
            if (s2 == 0x1f45 || s2 == 0x20de) && igual(5) && igual(6) {
                copiar();
                return s2;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x1f4d {
            if s2 == 0x1f4a {
                if sorteio(2) == 0 || !igual(5) {
                    return 0;
                }
                if (!igual(6) || !igual(7) || !igual(8)) && (!igual(6) || (b(q, 7) != -1 && b(e, 7) != -1)) {
                    return 0;
                }
                copiar();
                return 0x1f4a;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x1f43 {
            if s2 == 0x1f9d {
                if sorteio(2) == 0 {
                    return 0;
                }
                copiar();
                return 0x1f43;
            }
            break 'escolha Rotulo::R6cde;
        }
        if matches!(s3, 0x1f62 | 0x1f63 | 0x2150) && s2 == 0x1f46 && igual(5) && igual(6) {
            break 'escolha Rotulo::Copiar;
        }
        if s3 == 0x1f5d {
            if s2 == 0x1f6f {
                copiar();
                return 0x2143;
            }
            break 'escolha Rotulo::R6cde;
        }
        if contexto(s3) && s2 == 0x1f46 && b(q, 7) == b(e, 5) && b(q, 8) == b(e, 6) {
            break 'escolha Rotulo::Copiar;
        }
        match s3 {
            0x1f60 if s2 == 0x1f6f => {
                copiar();
                return 0x1f6f;
            }
            0x1f46 if s2 == 0x1f70 => {
                copiar();
                return 0x1f70;
            }
            0x1f84 | 0x1f85 if s2 == 0x1f75 => {
                if sorteio(3) == 0 {
                    return 0;
                }
                copiar();
                return if s3 == 0x1f84 { 0x2144 } else { 0x2145 };
            }
            0x1f60 | 0x1f46 | 0x1f84 | 0x1f85 => {}
            0x1f80 => {
                if s2 == 0x1f46 {
                    if sorteio(2) != 0 && !igual(6) {
                        trocar(true);
                        return 0x2146;
                    }
                    break 'escolha Rotulo::Copiar;
                }
            }
            0x1f5e => break 'escolha Rotulo::R69a6,
            _ => break 'escolha Rotulo::R69f5,
        }
        Rotulo::R6cde
    };

    loop {
        r = match r {
            Rotulo::R69a6 => {
                if s2 == 0x1f46 && sorteio(5) != 0 && igual(6) && !igual(5) {
                    trocar(true);
                    return 0x2147;
                }
                Rotulo::R69f5
            }
            Rotulo::R69f5 => 'r: {
                for (de, para) in [(0x2148, 0x2149), (0x214a, 0x214b), (0x214c, 0x214d)] {
                    if s3 == de {
                        if s2 == 0x1f60 || s2 == 0x1f5f {
                            copiar();
                            return para;
                        }
                        break 'r Rotulo::R6cde;
                    }
                }
                if contexto(s3) {
                    let (ie, iq) = (core::ptr::read_unaligned((e + 9) as *const i32), core::ptr::read_unaligned((q + 9) as *const i32));
                    if contexto(s2) && (1..0x15).contains(&ie) && (1..0x15).contains(&iq) && b(e, 5) == b(q, 7) && b(e, 7) == b(q, 5) {
                        copiar();
                        return 0x214e;
                    }
                    if s2 == 0x1f47 && (1..0x15).contains(&iq) && sorteio(0x16 - iq) != 0 && b(q, 7) == b(e, 5) && b(q, 8) == b(e, 6) {
                        break 'r Rotulo::Copiar;
                    }
                }
                if s3 == 0x214f {
                    if s2 == 0x1f92 {
                        copiar();
                        return 0x214f;
                    }
                    break 'r Rotulo::R6cde;
                }
                if matches!(s3, 0x213e | 0x1f71 | 0x1f72) && s2 == 0x1f75 {
                    copiar();
                    return 0x2151;
                }
                if s3 == 8000 {
                    if s2 == 0x1f9d {
                        copiar();
                        return 0x1f9d;
                    }
                    break 'r Rotulo::R6cde;
                }
                if s3 != 0x2021 {
                    if s3 == 0x1f55 {
                        if s2 == 8000 {
                            return -0x2152;
                        }
                    } else if s3 == 0x2113 {
                        break 'r Rotulo::R6c95;
                    } else if contexto(s3) {
                        break 'r Rotulo::R6cbf;
                    }
                    break 'r Rotulo::R6cde;
                }
                if matches!(s2, 0x1f62 | 0x1f63 | 0x2150) {
                    return -0x2153;
                }
                Rotulo::R6cbf
            }
            Rotulo::R6cbf => {
                if s2 != 0x1f70 {
                    Rotulo::R6cde
                } else if igual(6) {
                    return -0x2157;
                } else {
                    Rotulo::R6d1d
                }
            }
            Rotulo::R6c95 => {
                if s2 == 0x1f62 || s2 == 0x2150 {
                    return -0x2153;
                }
                Rotulo::R6cde
            }
            Rotulo::R6cde => {
                if (0x2076..=0x20d0).contains(&s2) && s3 == 0x2113 {
                    return if igual(5) && igual(6) { -0x2159 } else { 0 };
                }
                Rotulo::R6d1d
            }
            Rotulo::R6d1d => {
                const GRUPO: [i16; 8] = [0x1f4b, 0x1f4c, 0x1f4e, 0x1f4f, 0x1f50, 0x1f51, 0x1f52, 0x1f53];
                if (0x215a..=0x215f).contains(&s3) && s2 == 0x1f70 {
                    return -0x2157;
                }
                if s3 == 0x1f70 {
                    if s2 == 0x1f76 && !igual(6) {
                        copiar();
                        return 0x2165;
                    }
                    return 0;
                }
                if s3 == 0x2150 {
                    if s2 != 0x1f75 {
                        return 0;
                    }
                    copiar();
                    return 0x2151;
                }
                if GRUPO.contains(&s3) && GRUPO.contains(&s2) && sorteio(2) != 0 {
                    return -0x2169;
                }
                if s3 == 0x216e {
                    return if s2 == 0x1f44 && igual(5) && igual(6) { 0x216e } else { 0 };
                }
                if matches!(s3, 0x1f80 | 0x2131 | 0x2130) && s2 == 0x1f46 {
                    Rotulo::Copiar
                } else if s3 != 0x1f73 && !(0x20b9..=0x20c4).contains(&s3) && s3 != 0x20d0 && s3 != 0x1f80 {
                    return 0;
                } else if s2 != 0x1f46 && s2 != 0x1f47 {
                    return 0;
                } else {
                    Rotulo::Copiar
                }
            }
            Rotulo::Copiar => {
                copiar();
                return s3;
            }
        }
    }
}

/// { 0x6d4b70 } (thiscall, ret 0xc, match_eng.cpp): o comentário de contexto `evento` (0x21a3..=0x21af:
/// a corrida da liga, as notícias recentes do clube) pode tocar agora para o `lado`? Fora da faixa
/// 8000..=0x21e4, já tocado (tabela de +0x21e por lado) ou sem competição (+0x4792 → +0x14), não. Pesa
/// o placar agregado (0x6ce310), a linha do clube na tabela da liga (0x4a4850: +0x19/+0x41 e
/// +0x2d/+0x55), o minuto (+0x8ed0 / 11), as notícias do clube nos últimos 10 ou 30 dias (0x757da0 na
/// mídia [0xb6b7c8]) e a média dos 20 jogadores do lado (+0x47cb, passo 0x1be, os marcados em −0x1c);
/// os da tabela gravam o valor em `saida`. Os eventos da faixa fora dos 13 casos: sim.
unsafe extern "thiscall" fn context_commentary(m: usize, lado: u32, evento: u32, saida: *mut i32) -> u32 {
    let hoje = 0xae2c90usize;
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(0x54c8e0usize);
    let (mut d10, mut d30) = ([0u8; 8], [0u8; 8]);
    datar(hoje, &mut d10, 10);
    datar(hoje, &mut d30, 0x1e);
    let ev = evento as u16 as i16 as i32;
    if !(8000..=0x21e4).contains(&ev) {
        return 0;
    }
    let l = lado as u8 as i8;
    if *((m as isize + ((ev + l as i32 * 1000) * 4) as isize - 0x7ae2) as *const u32) != 0 {
        return 0;
    }
    let c = *((m + 0x4792) as *const usize);
    if *((c + 0x14) as *const u32) == 0 {
        return 0;
    }
    let (mut casa, mut fora, mut classificado, mut ida, mut fase3) = (0i8, 0i8, 0u8, 0u8, 0u8);
    let agregado = aggregate_score(m, l, &mut casa, &mut fora, &mut classificado, &mut ida, &mut fase3, 0) as i8 as i32;
    let competicao = *((*(0xadadfc as *const usize)) as *const usize).add(*(c as *const u32) as usize);
    let clube = *((c + if l != 0 { 0x20 } else { 0x1c }) as *const usize);
    let mut linha = 0usize;
    if competicao != 0 && *((competicao + 8) as *const u32) != 0 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        linha = busca(*((competicao + 8) as *const usize), clube);
    }
    let na_competicao = || {
        let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x48f150usize);
        f(competicao, clube) != 0
    };
    let noticia = |tipo: u32, data: &[u8; 8]| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, *const [u8; 8]) -> u32 = core::mem::transmute(0x757da0usize);
        f(*(0xb6b7c8 as *const usize), 0, clube, -1, tipo, data) != 0
    };
    // a média (16 bits) dos jogadores marcados do lado; None sem nenhum
    let media = || {
        let (mut s, mut n) = (0i32, 0i32);
        for k in 0..20 {
            let p = (m as isize + l as isize * 0x22d8 + 0x47cb + k * 0x1be) as usize;
            if *((p - 0x1c) as *const i8) < 0 {
                s += core::ptr::read_unaligned(p as *const i16) as i32;
                n += 1;
            }
        }
        if n == 0 { None } else { Some((s / n) as i16) }
    };
    let do_clube = || core::ptr::read_unaligned((clube + 0x57) as *const u32) == *((c + 0x14) as *const u32);
    let da_competicao = || core::ptr::read_unaligned((clube + 0x1db) as *const u32) == *(c as *const u32);
    let minuto = core::ptr::read_unaligned((m + 0x8ed0) as *const i16) as i32 / 11;
    // 0x6d587c / 0x6d5704: a linha da tabela (+o1, +o2) contra sorteios; grava +o2 em `saida`
    let tabela = |o1: usize, o2: usize| -> u32 {
        if linha == 0 || core::ptr::read_unaligned((linha + o1) as *const i32) <= sortear(3) + 1 {
            return 0;
        }
        let valor = core::ptr::read_unaligned((linha + o2) as *const i32);
        if valor <= sortear(5) + 8 {
            return 0;
        }
        if !saida.is_null() {
            *saida = valor;
        }
        1
    };
    let ate_media = |limite: i16| media().is_some_and(|v| v >= limite) as u32;
    let fim = || media().map_or(1, |v| (v <= 0x1a2b) as u32);
    let nao_trava = if fase3 as i8 != -3 { 3 } else { 0 };
    match ev {
        0x21a3 => {
            if !do_clube() || agregado <= sortear(3) || !na_competicao() {
                return 0;
            }
            if noticia(0x61, &d10) && media().is_some_and(|v| v > 8000) {
                return 1;
            }
            if noticia(0x62, &d10) && media().is_some_and(|v| v > 0x1e46) {
                return 1;
            }
            if !noticia(99, &d10) {
                return 0;
            }
            ate_media(0x1d4d)
        }
        0x21a4 | 0x21a5 => {
            if ev == 0x21a4 {
                if !da_competicao() || agregado <= sortear(nao_trava) {
                    return 0;
                }
            } else if !do_clube() || agregado <= sortear(3) {
                return 0;
            }
            let (tipo, data) = if ev == 0x21a4 { (0x5c, &d30) } else { (0x5f, &d10) };
            if !na_competicao() || !noticia(tipo, data) {
                return 0;
            }
            ate_media(0x1d4d)
        }
        0x21a6 | 0x21a7 | 0x21a9 => {
            if !do_clube() || sortear(-3) <= agregado || minuto < 0x1f || !na_competicao() {
                return 0;
            }
            let tipos: &[u32] = match ev {
                0x21a6 => &[0x3f],
                0x21a7 => &[0x61, 0x62, 0x41, 99],
                _ => &[0x5f, 0x40],
            };
            if !tipos.iter().any(|&t| noticia(t, &d10)) {
                return 0;
            }
            fim()
        }
        0x21a8 => {
            if !da_competicao() || sortear(-nao_trava) <= agregado || !na_competicao() || !noticia(0x5c, &d30) {
                return 0;
            }
            fim()
        }
        0x21aa => {
            if !do_clube() || agregado <= sortear(3) || minuto < 0x1f || !na_competicao() {
                return 0;
            }
            if !noticia(0x3f, &d10) && !noticia(0x60, &d10) {
                return 0;
            }
            ate_media(0x1b59)
        }
        0x21ab => {
            if !do_clube() || agregado <= sortear(2) + 1 || minuto <= 0x1e {
                return 0;
            }
            tabela(0x19, 0x41)
        }
        0x21ac | 0x21ad => {
            if !do_clube() {
                return 0;
            }
            if ev == 0x21ac {
                if -1 - sortear(2) <= agregado {
                    return 0;
                }
            } else if agregado <= sortear(3) + 1 {
                return 0;
            }
            if minuto < 0x1f {
                return 0;
            }
            tabela(0x2d, 0x55)
        }
        0x21ae => {
            if !do_clube() || sortear(-3) <= agregado || minuto < 0x1f || (minuto > 0x4b && agregado < -1) {
                return 0;
            }
            tabela(0x19, 0x41)
        }
        0x21af => {
            if !do_clube() {
                return 0;
            }
            match agregado {
                -1 if minuto > 0x50 => tabela(0x19, 0x41),
                -2 if minuto > sortear(10) + 0x46 => tabela(0x19, 0x41),
                _ => 0,
            }
        }
        _ => 1,
    }
}

unsafe extern "thiscall" fn comparar_combine_with_previous(t: usize, e: usize, m: usize) -> i16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i16>(%%I0%%);
    if gravando_no_topo() {
        return o(t, e, m);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(e, 14), (t + 0x30, 12 * 14)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, e, m), || crate::aleatorio::sem_contar(|| combine_with_previous(t, e, m)));
    PLACAR.registrar(igual && r == n, || format!("combine_with_previous({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", core::ptr::read_unaligned(e as *const u16), r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_context_commentary(m: usize, lado: u32, evento: u32, saida: *mut i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, *mut i32) -> u32>(%%I1%%);
    if gravando_no_topo() {
        return o(m, lado, evento, saida);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !saida.is_null() {
        reg.push((saida as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m, lado, evento, saida), || crate::aleatorio::sem_contar(|| context_commentary(m, lado, evento, saida)));
    PLACAR.registrar(igual && r == n, || format!("context_commentary({}, {:#x}) original={} rust={} {:x?} x {:x?}", lado as u8, evento as u16, r, n, mo, mr));
    r
}

/// { 0x6eada0 } (thiscall, match_pl.cpp): o construtor do jogador da partida (0x1bc bytes): zera os
/// campos, com −1 em +0, +0x28, +0x2b e +0x5d/+0x61/+0x65, 0xfb em +0x19, 6 em +0x1b e 0x17d4 em +0x35.
/// Devolve `this`.
unsafe extern "thiscall" fn construct_match_player(j: usize) -> usize {
    let b = |o: usize, v: u8| *((j + o) as *mut u8) = v;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((j + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((j + o) as *mut u32, v);
    for o in [0x19e, 0x69, 0x6d, 0x55, 0x1a6, 0x1a2, 0x1aa, 0x1ae, 0x1b2, 0x1b6, 0x4d, 0xf9] {
        d(o, 0);
    }
    for o in [0x5d, 0x61, 0x65] {
        d(o, u32::MAX);
    }
    for o in [0x37, 0x29, 0x19c, 0x1ba] {
        w(o, 0);
    }
    for o in [0x27, 0x102, 0x103, 0x25, 0x26, 0x1c, 0xff, 0x1a, 0x101, 0xfe, 1, 0x24, 0x23, 0xfd] {
        b(o, 0);
    }
    for o in 2..=0x16 {
        b(o, 0);
    }
    b(0x19, 0xfb);
    b(0, 0xff);
    b(0x2b, 0xff);
    b(0x28, 0xff);
    b(0x1b, 6);
    w(0x35, 0x17d4);
    j
}

unsafe extern "thiscall" fn comparar_construct_match_player(j: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(j, 0x1bc)], || o(j), || construct_match_player(j));
    PLACAR.registrar(igual && r == n, || format!("construct_match_player({:#x}) {:x?} x {:x?}", j, mo, mr));
    r
}
