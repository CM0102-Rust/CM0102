// ---------- %%LEVA%%a leva: a diretoria e os jogadores depois do jogo: os jogadores que ela cita (com a confiança), o veredito da nota na partida e o texto dela (manager_manager.cpp) ----------

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
unsafe extern "thiscall" fn board_named_players_after_match(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
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
    crate::comparison::branch("cqa_10 the board draws");
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
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
        let c = contract_of(cqa10_d(0xae1050), p);
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
        crate::comparison::branch("cqa_10 board praises");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_add(x.wrapping_mul(10))); // cqa10: a confiança pelos preferidos
    } else if d9 != 0 {
        crate::comparison::branch("cqa_10 board blames");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_sub(x.wrapping_mul(10))); // cqa10: a confiança pelos rejeitados
    }
    (d9 as u8).wrapping_add(da as u8)
}

/// Comparador: lado a lado a partir do mesmo estado do sorteador, do [0xae1080], do registro da diretoria do clube, dos
/// 3 ids e das 5 saídas; a devolução também (a lista do técnico, a tabela da liga e o registro da partida só leem). A
/// cópia sem contar os sorteios.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_board_named_players_after_match(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                                  a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, u32, *const u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8;
    let o = original_fn::<F>(%%I0%%);
    let mut regioes = crate::rng::STATE_REGIONS.to_vec();
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
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10),
        || crate::rng::without_counting(|| board_named_players_after_match(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10)));
    SCOREBOARD.record(igual && r == n, || format!("board_named_players_after_match(clube {:x}, elogio {}) original={} rust={} {:x?} x {:x?}", clube, *elogio, r, n, mo, mr));
    r
}

/// { 0x6ab670 } (thiscall, ret 8, manager_manager.cpp, quadro SEH do vetor local): `get_player_match_rating_verdict(
/// pessoa, partida)`: lê o registro da partida (0x729600 em [0xb6b7c0], com os dois lados de 0x625 bytes construídos e
/// destruídos pelos iteradores 0x9463af/0x9462bb) e acha a pessoa entre os 11 primeiros de cada lado (ids em +0x71, 0x49
/// bytes): a nota (+0x35) abaixo de 7 dá −1, acima de 7 dá 1, 7 dá 0; não achada (ou sem pessoa), 0. O ecx não conta.
unsafe extern "thiscall" fn get_player_match_rating_verdict(_this: usize, pessoa: usize, partida: u32) -> u8 {
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
unsafe extern "thiscall" fn compare_get_player_match_rating_verdict(this: usize, pessoa: usize, partida: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I1%%)(this, pessoa, partida);
    let n = get_player_match_rating_verdict(this, pessoa, partida);
    SCOREBOARD.record(r == n, || format!("get_player_match_rating_verdict({:x}, partida {}) original={} rust={}", pessoa, partida as i32, r as i8, n as i8));
    r
}

/// { 0x6ab2d0 } (stdcall, ret 0x10, manager_manager.cpp): `build_board_player_performance_text(ids, vitória, saída,
/// variante)`, pelo texto da diretoria (0x6a69c0). A original guarda só o primeiro id que não é −1 (no lugar dele, 0, 1
/// ou 2), conta 1 e os textos leem sempre o lugar 0. Vitória: com o primeiro, o nome (0x53dba0, 200, estilo 1) num dos
/// três elogios pela variante (0, 1, outra); depois, como nenhum jogador conta na derrota, a saída fica vazia (a original
/// zera o primeiro byte). Derrota: a saída vazia e, com o primeiro, uma das quatro culpas pela variante (0, 1, 2,
/// outra), com 100 e o nome. Sem ids, nada.
unsafe extern "stdcall" fn build_board_player_performance_text(ids: *const i32, vitoria: u32, saida: usize, variante: u32) {
    if ids.is_null() {
        return;
    }
    let (i0, i1, i2) = (*ids, *ids.add(1), *ids.add(2));
    if i0 == -1 && (i1 != -1 || i2 != -1) {
        // ponytail: a original lê o lugar 0 sem valor (a pilha): esse caso fica com ela
        return original_fn::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(%%I2%%)(ids, vitoria, saida, variante);
    }
    let nome: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut buf = [0u8; 0xc8];
    let b = buf.as_mut_ptr() as usize;
    crate::comparison::branch("cqa_10 board text");
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
unsafe extern "stdcall" fn compare_build_board_player_performance_text(ids: *const i32, vitoria: u32, saida: usize, variante: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(%%I2%%);
    if ids.is_null() || (*ids == -1 && (*ids.add(1) != -1 || *ids.add(2) != -1)) {
        return o(ids, vitoria, saida, variante);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(saida, 0x200)], || o(ids, vitoria, saida, variante),
        || build_board_player_performance_text(ids, vitoria, saida, variante));
    SCOREBOARD.record(igual, || format!("build_board_player_performance_text(vitória {}, variante {}) {:?} x {:?}", vitoria as u8, variante,
        String::from_utf8_lossy(&mo.0[0].1[..0x40]), String::from_utf8_lossy(&mr.0[0].1[..0x40])));
}
