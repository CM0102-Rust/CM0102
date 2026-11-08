// ---------- %%LEVA%%a leva: o mapa de casas da formação, a nota do clube para o jovem e a partida assistida pelo humano (formation.cpp, player_regen.cpp, human_manager.cpp) ----------

/// { 0x5b7160 } (thiscall, formation.cpp): o mapa de casas da formação. Para cada uma das 22 linhas
/// k (11 casas × 2 fases) e bloco 3 × 4 (a18, a14) com a posição (i = 12k + 4·a18 + a14) em +0x164
/// (coluna, palavra) e +0x166 (linha), as 3 × 3 células do bloco ganham a coluna × 9 / 27 no nibble
/// baixo e a linha × 12 / 36 no alto da tabela +0x5eb (12 por fila, 9 filas por k); o espelho em +0xf9e
/// (fila e coluna invertidas) recebe (0xb0 − linha × 16) com (0xf8 − coluna) & 0xf. Devolve 1.
unsafe extern "thiscall" fn build_position_lookup(this: usize) -> u32 {
    for k in 0..22i32 {
        for a18 in 0..3i32 {
            for a14 in 0..4i32 {
                let i = (12 * k + 4 * a18 + a14) as usize;
                for y in 3 * a18..3 * a18 + 3 {
                    for x in 3 * a14..3 * a14 + 3 {
                        let coluna = core::ptr::read_unaligned((this + i * 4 + 0x164) as *const i16) as i32;
                        let c = (coluna * 9 / 27) as u8;
                        let p = (this as isize + 0x5eb + ((9 * k + y) * 12 + x) as isize) as *mut u8;
                        *p = (*p & 0xf0) | (c & 0xf);
                        let linha = core::ptr::read_unaligned((this + i * 4 + 0x166) as *const i16) as i32;
                        let d = ((linha * 12 / 36) as u8) << 4;
                        *p = (c & 0xf) | d;
                        let q = (this as isize + 0xf9e + ((9 * k - y) * 12 - x) as isize) as *mut u8;
                        *q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho
                    }
                }
            }
        }
    }
    1
}

/// { 0x7ad220 } (stdcall, ret 0xc, player_regen.cpp): a nota do `clube` para o jovem `jogador`
/// (sorteio(nota)): 5000 + reputação (+0x80) menos as distâncias dela à habilidade (ficha +0x61: +0xb,
/// e +7, +5 × 50); com +7 acima de sorteio(20) + 100, o bônus de juniores (+0x7f, 20 = garantido,
/// senão sorteio(+0x7f) > sorteio(20)); clube de competição ativa (0xadadfc) com +7 > 50: o estado do
/// clube humano (0xae2d14) e a reputação contra o elenco (`elenco`); o treinador de juniores (+0xcf
/// → +0x69) do clube do computador (0x600a50) ou o bônus do humano; a mesma nação (+0x1a/+0x53) e os
/// clubes galeses (Swansea, Cardiff, Wrexham) e a Escócia. Só os 16 bits baixos da nota contam.
unsafe extern "stdcall" fn player_regen_score_club_for_player(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha = u(jogador, 0x61);
    let rep = w(clube, 0x80) as i32;
    let mut s = rep
        .wrapping_sub((w(ficha, 0xb) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 7).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 5).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_add(0x1388);
    let juniores = *((clube + 0x7f) as *const i8);
    if w(ficha, 7) as i32 > sorteio(0x14) + 0x64 {
        if juniores == 0x14 {
            s = s.wrapping_add(rep / 2 + 0x1b58);
        } else if sorteio(juniores as i32) > sorteio(0x14) {
            s = s.wrapping_add(rep / 2 + (juniores as i32 + 0x64) * 50);
        }
    }
    let comp = u(clube, 0x57);
    if w(ficha, 7) > 0x32 && comp != 0 && *((*(0xadadfc as *const usize) + 4 * *(comp as *const u32) as usize) as *const usize) != 0 {
        let estados = *(0xae2d14 as *const usize);
        if humano(clube, 1, 1, 0, 0) != 0 && estados != 0 && w(ficha, 7) > 0x78 {
            let e = *((estados + 6 * *(clube as *const u32) as usize + 4) as *const i8);
            if e < 0 {
                s = s.wrapping_add(0x1f4);
            } else if e == 0 {
                s = s.wrapping_add(0xfa);
            }
        }
        let r = w(clube, 0x80);
        s = s.wrapping_add(if r > 0x1676 {
            if elenco < 0x28 { 0x3e8 } else if elenco < 0x2d { 0x2ee } else { 0xfa }
        } else if r > 0x109a {
            if elenco < 0x23 { 0x2ee } else if elenco < 0x28 { 0x1f4 } else { 0xfa }
        } else {
            0xfa
        });
    }
    let tecnico = u(clube, 0xcf);
    let x = if tecnico != 0 { u(tecnico, 0x69) } else { 0 };
    if x != 0 && humano(clube, 1, 1, 0, 0) == 0 {
        s = s.wrapping_add((*((x + 0x22) as *const i8) as i32 - 10) * 30);
        if w(ficha, 7) as i32 > sorteio(0x4b) + 0x64 {
            let v = chance(x, *((x + 0x18) as *const i8)) as i8;
            s = s.wrapping_add((v as i32 - 10) * 60);
        }
        let v = chance(x, *((x + 0x18) as *const i8)) as i8;
        let t = v as i32 + *((x + 0x22) as *const i8) as i32 + juniores as i32;
        if t > 0x28 && w(clube, 0x80) > 0x109a && w(ficha, 7) > 0x50 {
            if (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            } else {
                s = s.wrapping_add(0xfa);
            }
        }
    } else if humano(clube, 1, 1, 0, 0) != 0 {
        s = s.wrapping_add(0xc8);
        if w(ficha, 7) > 0x78 {
            if juniores == 0x14 && (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            }
            s = s.wrapping_add(juniores as i32 * 25);
        }
    }
    let nacao = u(jogador, 0x1a);
    let nacao_do_clube = u(clube, 0x53);
    if nacao == nacao_do_clube {
        s = s.wrapping_add(0x3e8); // mesma nação
    }
    let g = |a: usize| *(a as *const u32);
    let (gales, inglaterra, escocia) = (g(0x9cf510), g(0x9cf2e4), g(0x9cf45c));
    let id = g(clube);
    if id == g(0x9d0430) || id == g(0x9d0434) || (id == g(0x9d043c) && nacao != 0) {
        let n = g(nacao);
        if n == gales {
            s = s.wrapping_add(0x3e8);
        } else if n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        }
    }
    if g(nacao_do_clube) == escocia {
        let n = g(nacao);
        if n == gales || n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        } else if n == escocia {
            s = s.wrapping_add(0x3e8);
        }
    }
    sorteio(s as i16 as i32)
}

/// O registro (42735 bytes em [this]) da partida que o `humano` (ou o da tela, 0xb74c26 + 0x300 ×
/// [0xb77c7a]) assiste: Ok(0) sem humano; Err se o índice (id − [0xae2364] + 0x10) sai de 0..[this+4].
unsafe fn formation_watched_match_record(this: usize, humano: usize) -> Result<usize, ()> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return Ok(0);
    }
    let i = (*(h as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(());
    }
    Ok((*(this as *const usize)).wrapping_add((i as usize).wrapping_mul(42735)))
}

/// { 0x5fe290 } (thiscall, ret 8, human_manager.cpp): se a `partida` é a que o humano escolheu
/// assistir: o registro dele (+0x9ad4..) com a mesma competição (+0x3f), datas (+0x2a, +0x28), clube
/// (+0), +0x34 e os dois lados (+0xc, +0x10). Partida nula ou humano fora da faixa: a original (caixa
/// de erro e 0).
unsafe extern "thiscall" fn is_watching_fixture(this: usize, partida: usize, humano: usize) -> u32 {
    let r = match formation_watched_match_record(this, humano) {
        Err(()) => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, partida, humano),
        Ok(_) if partida == 0 => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, partida, humano),
        Ok(0) => return 0,
        Ok(r) => r,
    };
    let b = |p: usize, o: usize| *((p + o) as *const u8);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let d = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u32);
    let mesma = b(partida, 0x3f) == b(r, 0x9b13)
        && w(partida, 0x2a) == w(r, 0x9afe)
        && w(partida, 0x28) == w(r, 0x9afc)
        && d(partida, 0) == d(r, 0x9ad4)
        && w(partida, 0x34) == w(r, 0x9b08)
        && d(partida, 0xc) == d(r, 0x9ae0)
        && d(partida, 0x10) == d(r, 0x9ae4);
    if mesma { 1 } else { 0 } // outra partida
}

unsafe extern "thiscall" fn comparar_build_position_lookup(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this + 0x5eb, 0x187b - 0x5eb)], || o(this), || build_position_lookup(this));
    SCOREBOARD.record(igual && r == n, || format!("build_position_lookup {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "stdcall" fn comparar_player_regen_score_club_for_player(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i8) -> i32>(%%I1%%);
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS,
        || o(jogador, clube, elenco), || crate::rng::without_counting(|| player_regen_score_club_for_player(jogador, clube, elenco)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_score_club_for_player(elenco {}) original={} rust={} sorteio igual={}", elenco, r, n, igual));
    r
}

unsafe extern "thiscall" fn comparar_is_watching_fixture(this: usize, partida: usize, humano: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%);
    let r = o(this, partida, humano);
    // ponytail: os casos de erro (caixa) só na original
    if partida == 0 || formation_watched_match_record(this, humano).is_err() {
        return r;
    }
    let n = is_watching_fixture(this, partida, humano);
    SCOREBOARD.record(r == n, || format!("is_watching_fixture original={} rust={}", r, n));
    r
}
