// ---------- %%LEVA%%a leva: a nota-alvo da busca de jogadores do clube e o histórico de jogos do jogador acompanhado: o jogo da vez e a soma dos gols (player_search.cpp) ----------

unsafe fn cpu13_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x7bca90 } (cdecl, player_search.cpp): `player_search_get_club_target_rating(clube, *base, *forcada)`: a nota que
/// o clube procura nos jogadores. A base (sem `forcada`): a reputação (+0x80), a do dinheiro (+0x77 × 0,43 − 430, até
/// 10000, se maior que a reputação), a da nação (+0x57: +0x69 × 500, se maior que reputação / 500) e a da liga (+0x1db
/// em [0xae23d0], 0x6b bytes: +0x69 × 600, idem), pesadas e / 16; metade no clube não profissional (+0x64 ≠ 1). A
/// nota sai da base por faixas (até 3000, 5000, 7000, 9000 e acima). Sem `forcada`, pelo elenco (0x53c590) e pela
/// faixa de riqueza das finanças (0x5a0590 de [0xae2a38], 0x5986b0) nos clubes com nação em [0xadadfc]: elenco curto
/// (< 16) metade; < 20 metade ou 2/3 nos pobres; senão encolhe pela falta de jogadores frente à nação (+0x69 × 1,35 +
/// 20) e a riqueza puxa (−2..2: × 0,909 ou − 1000, × 0,952 ou − 500, nada, × 1,0125 ou + 125, × 1,025 ou + 250).
/// No mínimo 500. A base vai em `*base` (se não nulo). As contas de ponto flutuante em f64 (o x87 do jogo em 53 bits).
unsafe extern "C" fn player_search_get_club_target_rating(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let contar: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let li = cpu13_d(clube + 0x1db);
    let liga = if li > -1 { (cpu13_d(0xae23d0) as usize).wrapping_add((li as usize).wrapping_mul(0x6b)) } else { 0 };
    let nacao = cpu13_d(clube + 0x57) as usize;
    let mut esi: i32;
    if !forcada.is_null() {
        esi = *forcada as i32;
    } else {
        let rep = cpu13_w(clube + 0x80) as i32;
        let t = match_pl_ftol(cpu13_d(clube + 0x77) as f64 * read_f64(0x96e2b0) - read_f64(0x96e2a8)) as i16 as i32;
        let ebx = (if rep > t { rep } else if t > 10000 { 10000 } else { t }) as i16 as i32;
        let mut n = rep;
        let mut l = rep;
        if nacao != 0 {
            let c = cpu13_w(nacao + 0x69) as i32;
            if c > rep / 500 {
                n = c * 500;
                l = n;
            }
        }
        if liga != 0 {
            let c = cpu13_w(liga + 0x69) as i32;
            if c > rep / 500 {
                l = c * 600;
            }
        }
        let soma = if n < 8000 {
            crate::comparison::branch("cpu_13 base by reputation");
            n.wrapping_mul(3).wrapping_add(ebx).wrapping_add(l.wrapping_add(rep.wrapping_mul(2)).wrapping_mul(4))
        } else {
            ebx.wrapping_add(rep.wrapping_mul(7)).wrapping_add(n.wrapping_add(l).wrapping_mul(4))
        };
        esi = soma / 16; // cpu13: a base
    }
    if *((clube + 0x64) as *const u8) != 1 {
        esi /= 2;
    }
    let mut edi: i32 = if esi > 9000 {
        esi.wrapping_mul(3).wrapping_add(7500)
    } else if esi > 7000 {
        match_pl_ftol(esi as f64 * read_f64(0x96d688) - read_f64(0x96d760))
    } else if esi > 5000 {
        match_pl_ftol(esi as f64 * read_f64(0x968e18) + read_f64(0x9690b8))
    } else if esi > 3000 {
        crate::comparison::branch("cpu_13 band from 3000 to 5000");
        esi.wrapping_mul(5).wrapping_sub(5000) // cpu13: a faixa do meio
    } else {
        match_pl_ftol(esi as f64 * read_f64(0x969018) + read_f64(0x9688e8))
    };
    if forcada.is_null() {
        let elenco = contar(clube, 0, 0);
        let com_nacao = |c: usize| c != 0 && cpu13_d((cpu13_d(0xadadfc) as usize).wrapping_add(cpu13_d(c) as usize * 4)) != 0;
        if elenco < 16 && com_nacao(nacao) {
            edi /= 2;
        } else if elenco < 20 && com_nacao(nacao) {
            let fin = financas(cpu13_d(0xae2a38) as usize, clube);
            if fin != 0 {
                match riqueza(fin) as i8 {
                    -2 => edi /= 2,
                    -1 => edi = match_pl_ftol(edi as f64 * read_f64(0x96a670)),
                    _ => {}
                }
            }
        } else {
            let nacao = cpu13_d(clube + 0x57) as usize;
            if com_nacao(nacao) {
                let t2 = match_pl_ftol(cpu13_w(nacao + 0x69) as i32 as f64 * read_f64(0x969588) + read_f64(0x968ea8)) as u8;
                let falta = (elenco as u8).wrapping_sub(t2) as i8;
                if falta < 0 {
                    let v = if falta > -10 { falta as i32 } else { -10 };
                    edi = match_pl_ftol((v as f64 * read_f64(0x967888) + read_f64(0x967890)) * edi as f64);
                }
            }
            let fin = financas(cpu13_d(0xae2a38) as usize, clube);
            if fin != 0 {
                let v = edi as f64;
                match riqueza(fin) as i8 {
                    -2 => {
                        let (x, y) = (v * read_f64(0x96d718), edi.wrapping_sub(1000) as f64);
                        edi = match_pl_ftol(if x < y { x } else { y });
                    }
                    -1 => {
                        let (x, y) = (v * read_f64(0x96e2a0), edi.wrapping_sub(500) as f64);
                        edi = match_pl_ftol(if x < y { x } else { y });
                    }
                    1 => {
                        let (x, y) = (v * read_f64(0x96d6b8), edi.wrapping_add(125) as f64);
                        edi = match_pl_ftol(if x > y { x } else { y });
                    }
                    2 => {
                        crate::comparison::branch("cpu_13 rich club");
                        let (x, y) = (v * read_f64(0x968fa0), edi.wrapping_add(250) as f64); // cpu13: o rico
                        edi = match_pl_ftol(if x > y { x } else { y });
                    }
                    _ => {}
                }
            }
        }
    }
    if edi < 500 {
        edi = 500;
    }
    if !base.is_null() {
        *base = esi;
    }
    edi
}

/// { 0x7bd830 } (thiscall, ret 4, player_search.cpp): `player_search_slot_get_match_entry(i)`: o jogo `i` (0x29
/// bytes, a partir de +6) do anel de 10 que começa em +0x1a0 (byte com sinal), ou 0 se `i` ≥ a conta (+0x1a1).
unsafe extern "thiscall" fn player_search_slot_get_match_entry(this: usize, i: u32) -> usize {
    let i = i as u8 as i8;
    if i >= *((this + 0x1a1) as *const i8) {
        return 0;
    }
    let mut k = *((this + 0x1a0) as *const i8) as i32 + i as i32;
    if k >= 10 {
        k -= 10;
    }
    (this as isize + k as isize * 0x29 + 6) as usize
}

/// { 0x7bd870 } (thiscall, ret 0, player_search.cpp): `player_search_slot_sum_goals()`: a soma (16 bits) do byte com
/// sinal +0x16 dos jogos do anel (+0x1a1 deles, a partir de +0x1a0, dando a volta em 10).
unsafe extern "thiscall" fn player_search_slot_sum_goals(this: usize) -> u16 {
    let n = *((this + 0x1a1) as *const i8);
    let mut k = *((this + 0x1a0) as *const u8);
    let mut s: u16 = 0;
    for _ in 0..n.max(0) {
        let g = *((this as isize + (k as i8) as isize * 0x29 + 0x16) as *const i8);
        s = s.wrapping_add(g as i16 as u16);
        k = k.wrapping_add(1);
        if k == 10 {
            k = 0;
        }
    }
    s
}

/// Comparador: pura (lê o clube, o elenco e as finanças; grava só `*base`): as duas sobre bases próprias.
unsafe extern "C" fn compare_player_search_get_club_target_rating(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32>(%%I0%%);
    let (mut bo, mut br) = (0x7fff_0001_i32, 0x7fff_0002_i32);
    let r = o(clube, &mut bo, forcada);
    let n = player_search_get_club_target_rating(clube, &mut br, forcada);
    SCOREBOARD.record(r == n && bo == br, || format!("player_search_get_club_target_rating({:x}, forcada {}) {}/{} base {}/{}", clube, if forcada.is_null() { -1 } else { *forcada as i32 }, r, n, bo, br));
    if !base.is_null() {
        *base = bo;
    }
    r
}

/// Comparador: pura: compara a devolução.
unsafe extern "thiscall" fn compare_player_search_slot_get_match_entry(this: usize, i: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, i);
    let n = player_search_slot_get_match_entry(this, i);
    SCOREBOARD.record(r == n, || format!("player_search_slot_get_match_entry({:x}, {}) {:x}/{:x}", this, i as u8 as i8, r, n));
    r
}

/// Comparador: pura: compara a devolução.
unsafe extern "thiscall" fn compare_player_search_slot_sum_goals(this: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(%%I2%%)(this);
    let n = player_search_slot_sum_goals(this);
    SCOREBOARD.record(r == n, || format!("player_search_slot_sum_goals({:x}) {}/{}", this, r, n));
    r
}
