// ---------- %%LEVA%%a leva: as posições previstas da copa: as posições pela rodada de saída, ordenar os clubes pela força e gravar a posição do clube na estatística (Cup.cpp, comp_util.cpp, comp_stats.cpp) ----------

unsafe fn cpt13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O núcleo da 0x4c08f0 sobre os `n` clubes em `v` e as chaves em `k`: a chave de cada clube é a força
/// (0x4c0720; clube nulo, 0); depois, para cada i, todo j > i com chave maior troca de lugar com i (chave e clube),
/// com sinal.
unsafe fn cpt13_ordenar_com(v: *mut u32, k: *mut i32, n: i32) {
    let forca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x4c0720usize);
    for i in 0..n.max(0) as usize {
        let c = *v.add(i);
        *k.add(i) = if c != 0 { forca(c as usize) } else { 0 };
    }
    let mut i = 0usize;
    while (i as i32) < n - 1 {
        for j in i + 1..n as usize {
            if *k.add(i) < *k.add(j) { // cpt13: a força maior sobe
                core::ptr::swap(k.add(i), k.add(j));
                core::ptr::swap(v.add(i), v.add(j));
            }
        }
        i += 1;
    }
}

/// { 0x4c08f0 } (cdecl, comp_util.cpp): `comp_util_sort_clubs_by_strength(clubes, n)`: ordena o vetor de `n` clubes
/// pela força (0x4c0720), a maior primeiro, com as chaves num vetor temporário (0x944e46 / 0x9452ca; a original o
/// enche de 0x60 antes, tudo sobrescrito). Sem memória: a caixa (linha 0x292d) e nada.
unsafe extern "C" fn comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let k = alocar((n as u32).wrapping_mul(4));
    if k == 0 {
        return aviso_em(0x99ec20, 0x292d);
    }
    cpt13_ordenar_com(v as *mut u32, k as *mut i32, n);
    if n > 1 {
        crate::comparacao::ramo("cpt_13 clubes ordenados");
    }
    liberar(k);
}

/// { 0x4a2540 } (thiscall, ret 8, comp_stats.cpp): `Comp_Stats::set_club_final_position(clube, posição)`: acha
/// (0x4a4850) ou acrescenta (0x4aceb0) o registro do clube na estatística da competição e grava a posição (byte) em
/// +0x72; devolve o registro. Clube nulo: a caixa (linha 0x680).
unsafe extern "thiscall" fn set_club_final_position(this: usize, clube: usize, pos: u32) -> usize {
    if clube == 0 {
        aviso_em(0x99ca94, 0x680);
        return 0; // ponytail: a original devolve o eax da caixa; quem chama não lê
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4aceb0usize);
    let mut e = buscar(this, clube);
    if e == 0 {
        e = acrescentar(this, clube);
    }
    crate::comparacao::ramo("cpt_13 posição gravada");
    core::ptr::write_volatile((e + 0x72) as *mut u8, pos as u8); // cpt13: a posição
    e
}

/// As chamadas da 0x5223a0 à 0x4a2540 (clube, posição), na ordem, com o vetor `v` já ordenado: da penúltima rodada
/// (+0x3c − 2) para trás ([+0xa3] + r × 0x68, a conta de quem sai em +0x1a), até um clube nulo no começo da rodada.
/// Na primeira rodada vista, os que saem nela ganham todos a posição 1 (sem olhar nulos); em cada rodada, os seguintes
/// ganham a posição do contador (que sobe a cada vaga, nula ou não), pulando os nulos. `gravar` = 0 só monta a lista.
unsafe fn cpt13_posicoes(this: usize, v: *const u32, gravar: usize) -> Vec<(usize, u8)> {
    let gravar_posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let mut chamadas: Vec<(usize, u8)> = Vec::new();
    let mut s: i16 = 0; // o índice no vetor (bp com sinal)
    let mut b: u8 = 1; // o contador de posições
    let mut a: u8 = 1; // a posição empurrada (só o byte de baixo conta; a original empurra a dword com sobra em cima)
    let mut r = cpt13_w(this + 0x3c).wrapping_sub(2) as i32;
    while r >= 0 {
        if *v.offset(s as isize) == 0 {
            break;
        }
        let conta = |r: i32| cpt13_w(cpt13_d(this + 0xa3) + r as usize * 0x68 + 0x1a) as i32;
        if r == cpt13_w(this + 0x3c) as i32 - 2 {
            let mut j = 0;
            while j < conta(r) {
                let c = *v.offset(s as isize) as usize;
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if gravar != 0 {
                    gravar_posicao(cpt13_d(this + 8), c, a as u32);
                }
                b = b.wrapping_add(1);
                j += 1;
            }
        }
        a = b; // cpt13: a posição da rodada
        let mut j = 0;
        while j < conta(r) {
            let c = *v.offset(s as isize) as usize;
            if c != 0 {
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if gravar != 0 {
                    crate::comparacao::ramo("cpt_13 posição pela rodada");
                    gravar_posicao(cpt13_d(this + 8), c, a as u32);
                }
            }
            b = b.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    chamadas
}

/// { 0x5223a0 } (thiscall, ret 0, Cup.cpp): `Cup::rank_teams()`: copia os clubes da copa ([+0xa7], registros de 6
/// bytes, +0x3e deles) para um vetor (0x944e46), ordena pela força (0x4c08f0) e dá a cada um a posição prevista pela
/// rodada em que sai (0x4a2540 na estatística [+8]); libera o vetor (0x9452ca). Sem estatística: a caixa (linha
/// 0x1827); sem memória: a original (a caixa da falta de memória e o fim do jogo).
unsafe extern "thiscall" fn cup_rank_teams(this: usize) {
    if cpt13_d(this + 8) == 0 {
        return aviso_em(0x9b55ac, 0x1827);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let n = cpt13_w(this + 0x3e) as i32;
    let v = alocar((n << 2) as u32);
    if v == 0 {
        return orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    }
    for i in 0..n.max(0) as usize {
        *((v + i * 4) as *mut u32) = cpt13_d(cpt13_d(this + 0xa7) + i * 6) as u32;
    }
    ordenar(v, n);
    cpt13_posicoes(this, v as *const u32, 1);
    liberar(v);
}

/// Comparador: grava e pode realocar a estatística (não roda duas vezes): antes da original, as posições são
/// planejadas sobre uma cópia ordenada do vetor (as chaves num Vec); depois, o registro de cada clube planejado tem
/// de ter em +0x72 a última posição dele.
unsafe extern "thiscall" fn comparar_cup_rank_teams(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let n = cpt13_w(this + 0x3e) as i32;
    if cpt13_d(this + 8) == 0 || n <= 0 {
        return o(this); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| cpt13_d(cpt13_d(this + 0xa7) + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // ponytail: com mais vagas nas rodadas que clubes, a original lê além do vetor (sobra do bloco); o plano lê 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plano = cpt13_posicoes(this, v.as_ptr(), 0);
    o(this);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erros = Vec::new();
    for (i, &(c, p)) in plano.iter().enumerate() {
        if c == 0 || plano[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = buscar(cpt13_d(this + 8), c);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("cup_rank_teams({:x}) {} posições: {}", this, plano.len(), erros.join("; ")));
}

/// Comparador: ordena no lugar (aloca e libera o temporário): o esperado sai do núcleo sobre uma cópia (chaves num
/// Vec) antes da original; depois, o vetor tem de ser igual.
unsafe extern "C" fn comparar_comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let o = orig::<unsafe extern "C" fn(usize, i32)>(%%I1%%);
    if v == 0 || n <= 1 {
        return o(v, n);
    }
    let mut esperado = core::slice::from_raw_parts(v as *const u32, n as usize).to_vec();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(esperado.as_mut_ptr(), k.as_mut_ptr(), n);
    o(v, n);
    let obtido = core::slice::from_raw_parts(v as *const u32, n as usize);
    PLACAR.registrar(obtido == &esperado[..], || format!("comp_util_sort_clubs_by_strength(n {}) original={:x?} rust={:x?}", n, obtido, esperado));
}

/// Comparador: acha ou acrescenta o registro (não roda duas vezes): o registro visto antes (se havia) tem de ser o
/// devolvido, com a posição em +0x72.
unsafe extern "thiscall" fn comparar_set_club_final_position(this: usize, clube: usize, pos: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I2%%);
    if clube == 0 {
        return o(this, clube, pos); // ponytail: a caixa, só a original
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let antes = buscar(this, clube);
    let r = o(this, clube, pos);
    let igual = r != 0 && (antes == 0 || r == antes) && *((r + 0x72) as *const u8) == pos as u8;
    PLACAR.registrar(igual, || format!("set_club_final_position(clube {:x}, {}) antes={:x} original={:x}", clube, pos as u8, antes, r));
    r
}
