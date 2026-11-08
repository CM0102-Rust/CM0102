// ---------- %%LEVA%%a leva: a tabela da liga: o resultado na tabela, a cópia da tabela e as posições previstas (league.cpp) ----------

unsafe fn cpw11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O ensaio do comparador da 0x685d30: a cópia roda sem a estatística da competição (0x49f570, que acha ou cria os
/// registros dos jogadores e não pode rodar duas vezes).
static mut LEAGUE_ADD_RESULT_DRY_RUN: bool = false;

/// { 0x685d30 } (thiscall, ret 8, league.cpp): `League::add_result(partida, jogo)`: grava a data do dia em +0xc9
/// (0x417c10 de [0xae2c90]); liga com fases (+0x44 = 0xff) e partida de uma fase (+0x42 ≠ 0xff): a fase
/// ([+0xc] + 4 × fase, até +0x30) recebe a partida pela virtual +4 e o que ela devolve sai; fase fora da faixa: as
/// caixas e 0. Partida de outra fase (+0x42 ≠ +0x44, que não é 0xfe): 0. Senão soma a partida na tabela (0x685f60 na
/// tabela +0xb1 com as +0x3e linhas, marcando) e, fora das fases 0xfe, com a competição ([0xadadfc] pelo id de +4) e a
/// estatística dela (+8) e com o jogo: 0x49f570 com os pontos dos dois; devolve 1. Partida nula: a caixa e −1.
unsafe extern "thiscall" fn league_add_result(liga: usize, p: usize, jogo: usize) -> i32 {
    if p == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I0%%)(liga, p, jogo);
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copia(liga + 0xc9, local.as_mut_ptr(), 0xae2c90);
    let fase_da_liga = *((liga + 0x44) as *const u8);
    let fase = *((p + 0x42) as *const u8);
    if fase_da_liga == 0xff && fase != 0xff {
        let k = fase as i8 as i32;
        if k > cpw11_d(liga + 0x30) as i32 || k < 0 {
            // as caixas da fase fora da faixa (linhas 0xc8e e 0xc8f): a original (a data já está gravada, igual)
            return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I0%%)(liga, p, jogo);
        }
        let filha = cpw11_d(cpw11_d(liga + 0xc).wrapping_add(4 * k as usize));
        let somar: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(cpw11_d(cpw11_d(filha) + 4));
        return somar(filha, p, jogo);
    }
    if fase_da_liga != fase && fase_da_liga != 0xfe {
        return 0;
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16) = core::mem::transmute(0x685f60usize);
    let (mut pts_casa, mut pts_fora) = (0i16, 0i16);
    // ponytail: a original empurra o +0x3e com o alto do ecx (sobra da chamada anterior); a 0x685f60 lê só o curto
    crate::comparacao::ramo("cpw_11 resultado aplicado");
    aplicar(liga, p, cpw11_d(liga + 0xb1), cpw11_w(liga + 0x3e) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela
    if *((liga + 0x44) as *const u8) == 0xfe || cpw11_d(0xadadfc) == 0 {
        return 1;
    }
    let comp = cpw11_d(cpw11_d(0xadadfc).wrapping_add(cpw11_d(cpw11_d(liga + 4)).wrapping_mul(4)));
    if comp == 0 || cpw11_d(comp + 8) == 0 || jogo == 0 {
        return 1;
    }
    if !LEAGUE_ADD_RESULT_DRY_RUN {
        // ponytail: a original empurra os pontos como dwords das locais com o alto sem valor; a 0x49f570 lê os curtos
        let estatistica: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32) -> u32 = core::mem::transmute(0x49f570usize);
        estatistica(cpw11_d(comp + 8), p, jogo, pts_casa as i32, pts_fora as i32);
    }
    1
}

/// Comparador: a partida nula e a da fase (a virtual da fase recebe a partida e compara por conta própria): só a
/// original. Nos outros casos, o ensaio: a cópia sem a estatística (0x49f570 acha ou cria registros) e a original a
/// partir do mesmo estado: a data em +0xc9, as linhas da tabela com os confrontos (as regiões da 0x685f60), o +0x3d/+0x3e
/// da partida e a devolução.
unsafe extern "thiscall" fn comparar_league_add_result(liga: usize, p: usize, jogo: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I0%%);
    if p == 0 || (*((liga + 0x44) as *const u8) == 0xff && *((p + 0x42) as *const u8) != 0xff) {
        return o(liga, p, jogo); // ponytail: a caixa ou a fase, só a original
    }
    let n = cpw11_w(liga + 0x3e) as u16 as u32;
    let mut regioes = vec![(liga + 0xc9, 8)];
    regioes.extend(result_update_regions(p, cpw11_d(liga + 0xb1), n, core::ptr::null_mut(), core::ptr::null_mut()));
    let antes = crate::comparacao::Memoria::ler(&regioes);
    LEAGUE_ADD_RESULT_DRY_RUN = true;
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| league_add_result(liga, p, jogo)));
    LEAGUE_ADD_RESULT_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let ro = o(liga, p, jogo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(ro == rr && mo == mr, || format!("league_add_result({:x}, partida {:x}) original={} {:x?} rust={} {:x?}", liga, p, ro, mo, rr, mr));
    ro
}

/// { 0x687710 } (thiscall, ret 0, league.cpp): `League::copy_table()`: sem tabela (+0xb1) ou sem linhas (+0x3e): 0.
/// Com +0xa7 = −1, grava nele (linhas − 1) × +0x3c (16 bits; mais de 50 linhas: a caixa da linha 0x1059 antes, pela
/// original). Aloca as linhas (0x944e46, 0x3b cada; sem memória: a caixa 0x1061 e 0) e copia; com os confrontos na
/// primeira linha (+5), cada linha ganha um vetor novo de (linhas − 1) × 14 bytes com a cópia dos dela; com a forma
/// (+9), um de +0xa7 bytes. Devolve a cópia. As alocações dos vetores não são conferidas (como na original).
unsafe extern "thiscall" fn copy_table(liga: usize) -> usize {
    let tab = cpw11_d(liga + 0xb1);
    let n = cpw11_w(liga + 0x3e);
    if tab == 0 || n == 0 {
        return 0;
    }
    if cpw11_w(liga + 0xa7) == -1 && n > 0 {
        if n > 0x32 {
            return original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%)(liga); // ponytail: a caixa, só a original
        }
        let v = n.wrapping_sub(1).wrapping_mul(cpw11_w(liga + 0x3c));
        core::ptr::write_unaligned((liga + 0xa7) as *mut i16, v);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha = |i: i32| (i as usize).wrapping_mul(0x3b);
    let c = alocar((n as i32 * 0x3b) as u32);
    if c == 0 {
        show_error_at(0x9f15f4, 0x1061);
        return 0;
    }
    core::ptr::copy_nonoverlapping(tab as *const u8, c as *mut u8, n as i32 as usize * 0x3b);
    if cpw11_d(tab + 5) != 0 && n > 0 {
        let mut i: i16 = 0;
        while i < cpw11_w(liga + 0x3e) {
            let m = cpw11_w(liga + 0x3e) as i32;
            let h = alocar((14 * m - 14) as u32);
            core::ptr::write_unaligned((c + linha(i as i32) + 5) as *mut usize, h);
            let de = cpw11_d(cpw11_d(liga + 0xb1) + linha(i as i32) + 5);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, (14 * cpw11_w(liga + 0x3e) as i32 - 14) as usize);
            i += 1;
        }
    }
    if cpw11_d(cpw11_d(liga + 0xb1) + 9) != 0 && cpw11_w(liga + 0x3e) > 0 {
        let mut i: i16 = 0;
        while i < cpw11_w(liga + 0x3e) {
            let t = cpw11_w(liga + 0xa7) as i32;
            let h = alocar(t as u32);
            core::ptr::write_unaligned((c + linha(i as i32) + 9) as *mut usize, h);
            let de = cpw11_d(cpw11_d(liga + 0xb1) + linha(i as i32) + 9);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, cpw11_w(liga + 0xa7) as i32 as usize);
            i += 1;
        }
    }
    c
}

/// Comparador: aloca (não roda duas vezes): só a original, com o plano completo conferido depois: o +0xa7 esperado
/// (calculado antes) e a cópia devolvida igual à tabela, linha a linha (fora os ponteiros dos vetores novos), com os
/// vetores de confrontos e de forma iguais aos da tabela.
unsafe extern "thiscall" fn comparar_copy_table(liga: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let tab = cpw11_d(liga + 0xb1);
    let n = cpw11_w(liga + 0x3e);
    if tab == 0 || n <= 0 || n > 0x32 {
        return o(liga); // ponytail: nada a copiar ou a caixa: só a original
    }
    let a7 = if cpw11_w(liga + 0xa7) == -1 { n.wrapping_sub(1).wrapping_mul(cpw11_w(liga + 0x3c)) } else { cpw11_w(liga + 0xa7) };
    let c = o(liga);
    let mut erros = Vec::new();
    if cpw11_w(liga + 0xa7) != a7 {
        erros.push(format!("+0xa7 {} esperado {}", cpw11_w(liga + 0xa7), a7));
    }
    if c != 0 {
        let (h2h, forma) = (cpw11_d(tab + 5) != 0, cpw11_d(tab + 9) != 0);
        let n = n as usize;
        for i in 0..n {
            let (lt, lc) = (tab + 0x3b * i, c + 0x3b * i);
            for b in 0..0x3b {
                if (h2h && (5..9).contains(&b)) || (forma && (9..13).contains(&b)) {
                    continue;
                }
                if *((lt + b) as *const u8) != *((lc + b) as *const u8) {
                    erros.push(format!("linha {} +{:#x}", i, b));
                    break;
                }
            }
            let iguais = |o_: usize, t: usize| {
                let (a, b) = (cpw11_d(lt + o_), cpw11_d(lc + o_));
                b != 0 && core::slice::from_raw_parts(a as *const u8, t) == core::slice::from_raw_parts(b as *const u8, t)
            };
            if h2h && n > 1 && !iguais(5, 14 * (n - 1)) {
                erros.push(format!("linha {} confrontos", i));
            }
            if forma && a7 > 0 && !iguais(9, a7 as usize) {
                erros.push(format!("linha {} forma", i));
            }
        }
    }
    PLACAR.registrar(c != 0 && erros.is_empty(), || format!("copy_table({:x}) n {} copia {:x}: {}", liga, n, c, erros.join("; ")));
    c
}

/// { 0x68a850 } (thiscall, ret 0, league.cpp): `League::set_predicted_positions()`: sem estatística (+8): a caixa
/// (linha 0x189e). Copia os clubes das +0x3e linhas da tabela (+0xb1, 0x3b cada) para um vetor (0x944e46; sem memória:
/// a caixa 0x18a6, sem soltar), ordena pela força (0x4c08f0) e grava a posição i + 1 de cada clube não nulo na
/// estatística (0x4a2540); solta o vetor (0x9452ca).
unsafe extern "thiscall" fn league_set_predicted_positions(liga: usize) {
    if cpw11_d(liga + 8) == 0 {
        return show_error_at(0x9f15f4, 0x189e);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let v = alocar(((cpw11_w(liga + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x9f15f4, 0x18a6);
    }
    let mut i = 0i32;
    while i < cpw11_w(liga + 0x3e) as i32 {
        *((v + 4 * i as usize) as *mut u32) = cpw11_d(cpw11_d(liga + 0xb1) + 0x3b * i as usize) as u32;
        i += 1;
    }
    ordenar(v, cpw11_w(liga + 0x3e) as i32);
    let mut i = 0i32;
    while i < cpw11_w(liga + 0x3e) as i32 {
        let c = cpw11_d(v + 4 * i as usize);
        if c != 0 {
            // ponytail: a original empurra o ecx com a posição no byte de baixo (o resto é sobra); a 0x4a2540 lê o byte
            gravar(cpw11_d(liga + 8), c, (i as u8).wrapping_add(1) as u32);
        }
        i += 1;
    }
    liberar(v);
}

/// Comparador: grava e pode criar registros na estatística (não roda duas vezes): o plano sai antes, sobre uma cópia
/// do vetor ordenada pelo núcleo da 0x4c08f0 (chaves num Vec); depois, o registro de cada clube (a última vez que ele
/// aparece) tem de ter em +0x72 a posição planejada.
unsafe extern "thiscall" fn comparar_league_set_predicted_positions(liga: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let n = cpw11_w(liga + 0x3e) as i32;
    if cpw11_d(liga + 8) == 0 || n <= 0 {
        return o(liga); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| cpw11_d(cpw11_d(liga + 0xb1) + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    o(liga);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = (i as u8).wrapping_add(1);
        let e = buscar(cpw11_d(liga + 8), c as usize);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("league_set_predicted_positions({:x}) {} clubes: {}", liga, n, erros.join("; ")));
}
