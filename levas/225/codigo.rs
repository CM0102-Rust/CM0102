// ---------- 225a leva: a contagem e a lista do elenco do clube e se o clube pode ser dirigido (Database.cpp) ----------

/// Um dos 50 lugares do elenco (+0xd7, ponteiros de pessoa) do `clube`.
unsafe fn lugar_do_elenco(clube: usize, i: usize) -> usize {
    core::ptr::read_unaligned((clube + 0xd7 + 4 * i) as *const usize)
}

/// A pessoa do elenco conta: não nula e com id abaixo de [0xae2364] (com sinal).
unsafe fn pessoa_valida_do_elenco(p: usize) -> bool {
    p != 0 && *(p as *const i32) < *(0xae2364 as *const i32)
}

/// { 0x53c590 } (cdecl, Database.cpp): quantas pessoas válidas há no elenco do `clube` (byte). Com
/// parceiro A/B (0x540a50 com 1): `modo` 2 e clube A conta só o parceiro; senão conta o clube e, fora
/// `modo` 1 com clube B, também o parceiro. Sem parceiro e com time reserva (0x542740): só as pessoas
/// com bits `modo` nas marcas de elenco (+0x5e), se `modo` não for 0. Com `filtro`, quem tem ficha
/// de contrato (0x4cf6d0) e clube (+0x39) diferente do `clube` só conta se o clube do contrato
/// (0x4cf5a0, guardado em [0xae1080]; +4 = id) também for outro. Clube nulo: a original (caixa).
unsafe extern "C" fn contar_elenco_do_clube(clube: usize, modo: u32, filtro: u32) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, u32, u32) -> u8>(604)(clube, modo, filtro);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let tem_reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x542740usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let modo = modo as u8;
    let contar = |c: usize, marcas: u8| -> u8 {
        let mut n = 0u8;
        for i in 0..50 {
            let p = lugar_do_elenco(c, i);
            if !pessoa_valida_do_elenco(p) || (marcas != 0 && *((p + 0x5e) as *const u8) & marcas == 0) {
                continue;
            }
            if filtro != 0 && ficha(*(0xae1050 as *const usize), p) != 0
                && core::ptr::read_unaligned((p + 0x39) as *const usize) != clube {
                let k = contrato(*(0xae1050 as *const usize), p);
                *(0xae1080 as *mut usize) = k;
                let dono = if k == 0 {
                    0
                } else {
                    (*(0xae23b4 as *const usize)).wrapping_add((core::ptr::read_unaligned((k + 4) as *const u32) as usize).wrapping_mul(581))
                };
                if dono == clube {
                    continue;
                }
            }
            n = n.wrapping_add(1);
        }
        n
    };
    // ponytail: o original passa o endereço de um local; 0x540a50 só devolve parceiro depois de gravá-lo
    let mut eh_a = 0u32;
    let p = parceiro(clube, &mut eh_a, 1);
    if p == 0 {
        if tem_reservas(clube) != 0 {
            return contar(clube, modo);
        }
        return contar(clube, 0);
    }
    if modo == 2 && eh_a != 0 {
        return contar(p, 0);
    }
    let n = contar(clube, 0);
    if modo == 1 && eh_a == 0 {
        return n;
    }
    n.wrapping_add(contar(p, 0))
}

/// { 0x541f90 } (cdecl, Database.cpp): copia o elenco do `clube` (50 ponteiros em +0xd7) para o
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): os 50 como estão e, com `dobro`, mais 50 zeros.
/// Com parceiro: só as pessoas válidas do clube e depois as do parceiro até o limite (100 com
/// `dobro`, senão 50), o resto até o limite zerado. Devolve 1. Clube nulo: a original (caixa e 0).
unsafe extern "C" fn reunir_elenco_do_clube(clube: usize, vetor: *mut usize, dobro: u32) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(605)(clube, vetor, dobro);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let limite: i8 = if dobro != 0 { 100 } else { 50 };
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p == 0 {
        for i in 0..50 {
            core::ptr::write_unaligned(vetor.add(i), lugar_do_elenco(clube, i));
        }
        if dobro != 0 {
            for i in 50..100 {
                core::ptr::write_unaligned(vetor.add(i), 0);
            }
        }
        return 1;
    }
    let mut n: i8 = 0;
    for i in 0..50 {
        let q = lugar_do_elenco(clube, i);
        if pessoa_valida_do_elenco(q) {
            core::ptr::write_unaligned(vetor.add(n as usize), q);
            n += 1;
        }
    }
    for i in 0..50 {
        if n >= limite {
            break;
        }
        let q = lugar_do_elenco(p, i);
        if pessoa_valida_do_elenco(q) {
            core::ptr::write_unaligned(vetor.add(n as usize), q);
            n += 1;
        }
    }
    for k in n..limite {
        core::ptr::write_unaligned(vetor.add(k as usize), 0);
    }
    1
}

/// { 0x544820 } (cdecl, Database.cpp): o `clube` pode ser dirigido? Precisa de nação (+0x53). Seleções
/// (id ≥ [0xae235c] − 2 × [0xae2350], com sinal): só as principais (id < [0xae235c] − [0xae2350]) de
/// nação com +0x88 > 150. Clubes: com competição (+0x57), que não sejam o time B de um par (0x540a50
/// com 1), com a competição no vetor [[0xadadfc] + id × 4] e a nação com o bit 2 de +0x11c.
unsafe extern "C" fn clube_pode_ser_dirigido(clube: usize) -> u8 {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let u = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    if clube == 0 || u(0x53) == 0 {
        return 0;
    }
    let id = *(clube as *const i32);
    let (n, total) = (*(0xae2350 as *const i32), *(0xae235c as *const i32));
    if id >= total.wrapping_sub(n.wrapping_add(n)) {
        return (id < total.wrapping_sub(n) && core::ptr::read_unaligned((u(0x53) + 0x88) as *const i32) > 0x96) as u8;
    }
    if u(0x57) == 0 {
        return 0;
    }
    // ponytail: o original usa o lugar do próprio argumento como saída; só é lido com parceiro, e aí já foi gravado
    let mut eh_a = 0u32;
    if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
        return 0; // time B
    }
    let tabela = *(0xadadfc as *const usize);
    if tabela == 0 {
        return 0;
    }
    let comp = *(u(0x57) as *const i32);
    if *((tabela as isize).wrapping_add((comp as isize).wrapping_mul(4)) as *const u32) == 0 {
        return 0;
    }
    (*((u(0x53) + 0x11c) as *const u8) & 2 != 0) as u8
}

unsafe extern "C" fn comparar_contar_elenco_do_clube(clube: usize, modo: u32, filtro: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32) -> u8>(604);
    if clube == 0 {
        return o(clube, modo, filtro); // caixa: só a original
    }
    // 0x540a50 pode zerar +0x244 do clube; o filtro grava o último contrato em [0xae1080]
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1), (0xae1080, 4)],
        || o(clube, modo, filtro), || contar_elenco_do_clube(clube, modo, filtro));
    PLACAR.registrar(igual && r == n, || format!("contar_elenco_do_clube({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", clube, modo as u8, filtro, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_reunir_elenco_do_clube(clube: usize, vetor: *mut usize, dobro: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(605);
    if clube == 0 {
        return o(clube, vetor, dobro); // caixa: só a original
    }
    let regioes = [(vetor as usize, if dobro != 0 { 400 } else { 200 }), (clube + 0x244, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(clube, vetor, dobro),
        || reunir_elenco_do_clube(clube, vetor, dobro));
    PLACAR.registrar(igual && r == n, || format!("reunir_elenco_do_clube({:x}, {}) original={} rust={} {:x?} x {:x?}", clube, dobro, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_clube_pode_ser_dirigido(clube: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(606);
    let marca = [(clube.wrapping_add(0x244), 1)];
    let regioes = if clube == 0 { &marca[..0] } else { &marca[..] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || o(clube), || clube_pode_ser_dirigido(clube));
    PLACAR.registrar(igual && r == n, || format!("clube_pode_ser_dirigido({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

// ---------- 225a leva: os vetores de atributos de volta ao não jogador e ao jogador e o reequilíbrio dos atributos do não jogador (Database.cpp) ----------

/// { 0x53a980 } (cdecl, Database.cpp): o inverso de 0x53a8b0: o byte de baixo de cada uma das 21
/// células de 3 bytes do `vetor` volta ao `nao_jogador` na ordem `ORDEM_DO_NAO_JOGADOR`. A pessoa não
/// é usada. eax na saída: o vetor com o último byte copiado no byte de baixo (os chamadores não usam).
unsafe extern "C" fn vetor_para_o_nao_jogador(nao_jogador: usize, _pessoa: usize, vetor: usize) -> u32 {
    let mut ultimo = 0u8;
    for (k, &d) in ORDEM_DO_NAO_JOGADOR.iter().enumerate() {
        ultimo = *((vetor + 3 * k) as *const u8);
        *((nao_jogador + d) as *mut u8) = ultimo;
    }
    (vetor as u32 & 0xffff_ff00) | ultimo as u32
}

/// { 0x53aa10 } (cdecl, Database.cpp): o inverso de 0x53a610: o byte de baixo das células 0..53 do
/// `vetor` volta ao `jogador` nos deslocamentos de `CELULAS_DO_VETOR` e o da célula 54 à `pessoa`
/// +0x58. eax na saída: o vetor com o último byte no byte de baixo (os chamadores não usam).
unsafe extern "C" fn vetor_para_o_jogador(jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    for (i, &(d, _)) in CELULAS_DO_VETOR.iter().enumerate() {
        *((jogador + d as usize) as *mut u8) = *((vetor + 3 * i) as *const u8);
    }
    let ultimo = *((vetor + 3 * 54) as *const u8);
    *((pessoa + 0x58) as *mut u8) = ultimo;
    (vetor as u32 & 0xffff_ff00) | ultimo as u32
}

/// { 0x539490 } (cdecl, Database.cpp): reequilibra os atributos do não jogador (pessoa +0x69). Monta
/// o vetor (63 células 0xff81/0x81, preenchidas por 0x53a8b0, que devolve o corte 13) e, duas vezes,
/// a partir do corte: (1) com limite 40, +10 por volta, até 6 células com |valor| ≤ limite (marcadas);
/// a célula 19 fora do limite também é marcada; (2) soma −5 − média às marcadas (presas em −125..125);
/// na 1ª passada as não marcadas positivas entram na soma de excesso, as demais são marcadas; com
/// cargo +0x3d ou +0x28 = 10, as células ficam ≤ −80 e a 19 ≥ 0; (3) na 1ª passada, com soma > 28,
/// tira (soma − 28) / quantas das não marcadas (mínimo 0). No fim, 0x53a980 grava de volta. Pessoa
/// ou não jogador nulos: nada.
unsafe extern "C" fn normalizar_atributos_do_nao_jogador(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let nj = core::ptr::read_unaligned((pessoa + 0x69) as *const usize);
    if nj == 0 {
        return;
    }
    let preencher: unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32 = core::mem::transmute(0x53a8b0usize);
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53a980usize);
    let mut vetor = [0u8; 63 * 3];
    for c in vetor.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = vetor.as_mut_ptr();
    let mut fim = 0u8;
    preencher(nj, pessoa, v as usize, &mut fim);
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let fim = fim as i8;
    let mut marca = [0u32; 64];
    let cargo_10 = *((pessoa + 0x3d) as *const u8) == 10 || *((pessoa + 0x28) as *const u8) == 10;
    for passo in 0..2 {
        let bp = w(fim);
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limite = 0x28i32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if bp != -0x7f {
                let mut c = fim;
                loop {
                    let x = w(c) as i32;
                    if x.abs() <= limite as i16 as i32 {
                        total += x;
                        n += 1;
                        marca[c as usize] = 1;
                    } else {
                        marca[c as usize] = 0;
                        if c == 0x13 && (melhor == -1 || w(0x13) > w(melhor)) {
                            melhor = 0x13;
                        }
                    }
                    c += 1;
                    if w(c) == -0x7f {
                        break;
                    }
                }
                if n >= 6 {
                    break;
                }
            }
            // ponytail: com a célula do corte = −127 o original roda para sempre; a cópia também
            limite += 10;
        }
        if melhor > -1 {
            marca[melhor as usize] = 1;
        }
        let delta = (-5i32).wrapping_sub(total / n as i32) as i16;
        if bp != -0x7f {
            let mut c = fim;
            loop {
                if c != melhor {
                    if marca[c as usize] == 1 {
                        let x = w(c).wrapping_add(delta);
                        pw(c, if x < -0x7d { -0x7d } else if x > 0x7d { 0x7d } else { x });
                    } else if passo == 0 && w(c) > 0 {
                        quantas += 1;
                        soma += w(c) as i32;
                    } else {
                        marca[c as usize] = 1;
                    }
                    if cargo_10 {
                        if c != 0x13 {
                            pw(c, w(c).min(-0x50));
                        } else {
                            pw(0x13, w(0x13).max(0));
                        }
                    }
                }
                c += 1;
                if w(c) == -0x7f {
                    break;
                }
            }
        }
        if w(fim) != -0x7f {
            let mut c = fim;
            while quantas != 0 && soma > 0x1c {
                if marca[c as usize] != 1 {
                    let x = w(c).wrapping_sub(((soma - 0x1c) / quantas) as i16);
                    pw(c, if x < 0 { 0 } else { x });
                }
                c += 1;
                if w(c) == -0x7f {
                    break;
                }
            }
        }
    }
    devolver(nj, pessoa, v as usize);
}

unsafe extern "C" fn comparar_vetor_para_o_nao_jogador(nao_jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(607);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(nao_jogador + 0x0e, 21)],
        || o(nao_jogador, pessoa, vetor), || vetor_para_o_nao_jogador(nao_jogador, pessoa, vetor));
    PLACAR.registrar(igual && r == n, || format!("vetor_para_o_nao_jogador({:x}) original={:x} rust={:x} {:x?} x {:x?}", nao_jogador, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_vetor_para_o_jogador(jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(608);
    // os deslocamentos de CELULAS_DO_VETOR ficam em +0x0f..+0x44
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(jogador + 0x0f, 0x44 - 0x0f + 1), (pessoa + 0x58, 1)],
        || o(jogador, pessoa, vetor), || vetor_para_o_jogador(jogador, pessoa, vetor));
    PLACAR.registrar(igual && r == n, || format!("vetor_para_o_jogador({:x}) original={:x} rust={:x} {:x?} x {:x?}", jogador, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_normalizar_atributos_do_nao_jogador(pessoa: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(609);
    let nj = if pessoa == 0 { 0 } else { core::ptr::read_unaligned((pessoa + 0x69) as *const usize) };
    if nj == 0 {
        return o(pessoa); // não faz nada: só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(nj + 0x0e, 21)], || o(pessoa),
        || normalizar_atributos_do_nao_jogador(pessoa));
    PLACAR.registrar(igual, || format!("normalizar_atributos_do_nao_jogador({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

// ---------- 225a leva: a família da cor, se duas cores se confundem e o atributo calculado do não jogador (Database.cpp) ----------

/// As cores fixas (chaves do index.cpp em 0x9cf178..0x9cf1fc, ids) e a família de cada uma, na ordem
/// das comparações de 0x540520.
const FAMILIAS_DAS_CORES: [(usize, u8); 34] = [
    (0x9cf1e4, 10), (0x9cf1e8, 10), (0x9cf178, 1), (0x9cf1d0, 1), (0x9cf17c, 2), (0x9cf180, 3), (0x9cf184, 3),
    (0x9cf188, 3), (0x9cf18c, 8), (0x9cf190, 8), (0x9cf194, 8), (0x9cf198, 8), (0x9cf19c, 9), (0x9cf1a0, 9),
    (0x9cf1f0, 4), (0x9cf1a4, 4), (0x9cf1a8, 4), (0x9cf1fc, 4), (0x9cf1ac, 5), (0x9cf1b0, 5), (0x9cf1b4, 5),
    (0x9cf1b8, 5), (0x9cf1bc, 5), (0x9cf1c0, 6), (0x9cf1c4, 6), (0x9cf1c8, 6), (0x9cf1cc, 6), (0x9cf1f4, 6),
    (0x9cf1f8, 6), (0x9cf1d4, 7), (0x9cf1d8, 7), (0x9cf1dc, 7), (0x9cf1e0, 7), (0x9cf1ec, 7),
];

/// A família da `cor` pela tabela; None se o id não está em nenhuma chave (caixa na original).
unsafe fn familia_conhecida_da_cor(cor: usize) -> Option<u8> {
    let id = *(cor as *const u32);
    FAMILIAS_DAS_CORES.iter().find(|&&(a, _)| *(a as *const u32) == id).map(|&(_, f)| f)
}

/// { 0x540520 } (cdecl, Database.cpp): a família (1..10) da `cor` pelo id (+0), comparado com as chaves
/// de cor em `FAMILIAS_DAS_CORES` (a primeira que casa). Cor fora da lista: a original (caixa "Error",
/// [0xb67a34] = 0 e 2).
unsafe extern "C" fn familia_da_cor(cor: usize) -> u8 {
    match familia_conhecida_da_cor(cor) {
        Some(f) => f,
        None => orig::<unsafe extern "C" fn(usize) -> u8>(610)(cor),
    }
}

/// { 0x540490 } (cdecl, Database.cpp): as cores `a` e `b` se confundem? Pelas famílias (0x540520, `a`
/// primeiro): iguais; 8 com 9; 8 com 7; 6 com 7, salvo quando a da família 6 está contra a cor com o id
/// da chave [0x9cf1dc] (exceção: não se confundem).
unsafe extern "C" fn cores_se_confundem(a: usize, b: usize) -> u8 {
    let familia: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x540520usize);
    let fa = familia(a);
    let fb = familia(b);
    let par = |x: u8, y: u8| (fa == x && fb == y) || (fb == x && fa == y);
    let chave = *(0x9cf1dc as *const u32);
    if fa == fb || par(8, 9) || par(8, 7) {
        return 1;
    }
    // a família 6 contra a cor da chave é exceção: não se confundem (a original sai com 0 antes do par 6/7)
    if (fa == 6 && *(b as *const u32) == chave) || (fb == 6 && *(a as *const u32) == chave) {
        return 0;
    }
    par(6, 7) as u8
}

/// { 0x543fd0 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`, do `nao_jogador`
/// (+0x69 dela). Diretos: 1 pessoa +0x56, 4 pessoa +0x58, 7 não jogador +0x14, 12 +0x22. Os outros
/// (2 +0x11, 3 +0x10, 5 +0x17, 6 +0x18, 8 +0x19, 9 +0x1b, 10 +0x1e, 11 +0x21) pela fórmula de 0x544410:
/// x = (palavra +4 + 2 × byte) × 0,05 + 10; (x + 0,1 x²) / 3 + 0,5, preso em 1..20 e truncado. Pessoa
/// ou não jogador nulos ou atributo fora de 1..12: a original (caixa e −1).
unsafe extern "C" fn atributo_calculado_do_nao_jogador(pessoa: usize, nao_jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) || nao_jogador == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(612)(pessoa, nao_jogador, atributo);
    }
    let byte = |p: usize, d: usize| *((p + d) as *const u8);
    let formula = |d: usize| {
        let b = *((nao_jogador + d) as *const i8) as f64;
        let x = (*((nao_jogador + 4) as *const i16) as f64 + (b + b)) * 0.05 + 10.0;
        let y = (x * x * 0.1 + x) * f64_em(0x968a80) + 0.5;
        ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y }) as u8
    };
    match a {
        1 => byte(pessoa, 0x56),
        2 => formula(0x11),
        3 => formula(0x10),
        4 => byte(pessoa, 0x58),
        5 => formula(0x17),
        6 => formula(0x18),
        7 => byte(nao_jogador, 0x14),
        8 => formula(0x19),
        9 => formula(0x1b),
        10 => formula(0x1e),
        11 => formula(0x21),
        _ => byte(nao_jogador, 0x22),
    }
}

unsafe extern "C" fn comparar_familia_da_cor(cor: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(610);
    let r = o(cor);
    if familia_conhecida_da_cor(cor).is_none() {
        return r; // caixa: só a original
    }
    let n = familia_da_cor(cor);
    PLACAR.registrar(r == n, || format!("familia_da_cor({:x}: id {}) original={} rust={}", cor, *(cor as *const u32), r, n));
    r
}

unsafe extern "C" fn comparar_cores_se_confundem(a: usize, b: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u8>(611);
    let r = o(a, b);
    if familia_conhecida_da_cor(a).is_none() || familia_conhecida_da_cor(b).is_none() {
        return r; // caixa: só a original
    }
    let n = cores_se_confundem(a, b);
    PLACAR.registrar(r == n, || format!("cores_se_confundem({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

unsafe extern "C" fn comparar_atributo_calculado_do_nao_jogador(pessoa: usize, nao_jogador: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(612);
    let r = o(pessoa, nao_jogador, atributo);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) || nao_jogador == 0 {
        return r; // caixa: só a original
    }
    let n = atributo_calculado_do_nao_jogador(pessoa, nao_jogador, atributo);
    PLACAR.registrar(r == n, || format!("atributo_calculado_do_nao_jogador({:x}, {:x}, {}) original={} rust={}", pessoa, nao_jogador, a, r, n));
    r
}
