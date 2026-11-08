// ---------- %%LEVA%%a leva: a competição: todas as fases completas, a lista dos clubes e o progresso do clube (comp.cpp) ----------

unsafe fn cqh6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqh6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x48ce10 } (thiscall, ret 0, comp.cpp, método +0xc das competições): `Comp::all_stages_complete()`: só na
/// competição do topo (+0x1c ≤ −1), completa (método +0x10 com 1, 1: push 1; push 1; mov ecx -> thiscall, ret 8) ela e
/// cada uma da cadeia de baixo (+0x20, o índice em [0xadadfc]); devolve 1 ou 0 (no al).
unsafe extern "thiscall" fn all_stages_complete(this: usize) -> u8 {
    if (cqh6_d(this + 0x1c) as i32) > -1 {
        return 0;
    }
    let completa = |c: usize| -> u8 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cqh6_d(cqh6_d(c) + 0x10));
        f(c, 1, 1)
    };
    if completa(this) == 0 {
        return 0;
    }
    let mut i = cqh6_d(this + 0x20) as i32;
    while i >= 0 {
        let c = cqh6_d(cqh6_d(0xadadfc) + i as usize * 4);
        if completa(c) == 0 {
            return 0;
        }
        crate::comparison::branch("cqh_6 liga de baixo completa");
        i = cqh6_d(c + 0x20) as i32; // cqh6: a liga de baixo
    }
    1
}

/// Só a original (o método +0x10 marca a liga completa, grava recordes e paga prêmios: não roda duas vezes): a cópia
/// leva os ramos.
unsafe extern "thiscall" fn compare_all_stages_complete(this: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(%%I0%%)(this)
}

/// { 0x48e1c0 } (thiscall, ret 8, comp.cpp, método +0x58 das competições): `Comp::collect_club_list(&lista, _)`: um
/// vetor de 200 ponteiros (0x944e46(0x320); sem memória, a caixa da linha 0x681, a lista nula e 0) com os clubes
/// distintos de todas as fases (−1 a própria até +0x30; as outras em [+0xc]): numa liga (+0x43 = 2) os da tabela +0xb1
/// (linhas de 0x3b bytes), noutra os de +0xa7 (6 bytes), sem os nulos e os repetidos, até 200; grava o vetor em
/// `lista` e devolve a conta (word, no ax).
unsafe extern "thiscall" fn comp_collect_club_list(this: usize, lista: *mut usize, _x: u32) -> u16 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0x320);
    if v == 0 {
        show_error_at(0x99b80c, 0x681);
        *lista = 0;
        return 0;
    }
    let mut n = 0i16;
    let mut s = -1i8;
    while s as i32 <= cqh6_d(this + 0x30) as i32 {
        let f = if s == -1 { this } else { cqh6_d(cqh6_d(this + 0xc) + (s as i32 * 4) as usize) };
        let liga = cqh6_b(f + 0x43) == 2;
        let mut i = 0i16;
        while i < cqh6_w(f + 0x3e) {
            let clube = if liga { cqh6_d(cqh6_d(f + 0xb1) + i as usize * 0x3b) } else { cqh6_d(cqh6_d(f + 0xa7) + i as usize * 6) };
            if clube != 0 && !(0..n).any(|k| cqh6_d(v + k as usize * 4) == clube) && n < 0xc8 { // cqh6: até 200 clubes
                crate::comparison::branch("cqh_6 clube na lista");
                core::ptr::write_unaligned((v + n as usize * 4) as *mut u32, clube as u32); // cqh6: o clube na lista
                n += 1;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    *lista = v;
    n as u16
}

/// Lado a lado: a original com a saída de quem chama, a cópia com saída própria, a partir do mesmo [0xb67a34]; a conta
/// e os clubes das duas listas iguais (a da cópia é liberada).
unsafe extern "thiscall" fn compare_comp_collect_club_list(this: usize, lista: *mut usize, x: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(%%I1%%);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut l2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, lista, x), || comp_collect_club_list(this, &mut l2, x));
    let mesmas = a == b && (*lista == 0) == (l2 == 0) && (*lista == 0 || core::slice::from_raw_parts(*lista as *const u32, a as usize) == core::slice::from_raw_parts(l2 as *const u32, b as usize));
    if l2 != 0 {
        soltar(l2);
    }
    SCOREBOARD.record(igual && mesmas, || format!("comp_collect_club_list: conta original={} rust={}", a, b));
    a
}
/// { 0x48cc70 } (thiscall, ret 0x10, comp.cpp): `Comp::find_club_progress(clube, &fase, &rodada, &situação)`: das
/// fases da última (+0x30) para a primeira (−1 a própria; as outras em [+0xc]), pulando as de id 0x3fc: numa liga
/// (+0x43 = 2), a linha da tabela do clube (0x6826d0 push i; mov ecx, fase -> thiscall, ret 4): fase = o id (+0x3a) e
/// situação = +0x37 da linha, e volta; numa copa, o último time da tabela +0xa7 (6 bytes) que é o clube dá fase = o id
/// e situação = +5, e a rodada é a da primeira rodada (de +0xab − 1 para trás, +0xa3 com 0x68 bytes, os confrontos de
/// +0xad com 0x35 bytes) com um confronto em que o time joga (+0 ou +2): rodada = +7 da rodada, e volta.
unsafe extern "thiscall" fn find_club_progress(this: usize, clube: usize, fase_id: *mut i16, rodada: *mut i16, situacao: *mut u8) {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let mut f = cqh6_d(this + 0x30) as i32;
    while f >= -1 {
        let s = if f > -1 { cqh6_d(cqh6_d(this + 0xc) + f as usize * 4) } else { this };
        if cqh6_w(s + 0x3a) != 0x3fc {
            if cqh6_b(s + 0x43) == 2 {
                let mut i = 0i8;
                while (i as i16) < cqh6_w(s + 0x3e) {
                    if cqh6_d(linha(s, i as i32)) == clube {
                        *fase_id = cqh6_w(s + 0x3a);
                        *situacao = cqh6_b(linha(s, i as i32) + 0x37);
                        return;
                    }
                    i = i.wrapping_add(1);
                }
            } else {
                let mut time: i16 = -1;
                let mut k = 0i16;
                while k < cqh6_w(s + 0x3e) {
                    let t = cqh6_d(s + 0xa7) + k as usize * 6;
                    if cqh6_d(t) == clube {
                        *fase_id = cqh6_w(s + 0x3a);
                        crate::comparison::branch("cqh_6 progresso na copa");
                        *situacao = cqh6_b(t + 5); // cqh6: a situação do time
                        time = k;
                    }
                    k += 1;
                }
                if time != -1 {
                    let mut r = (cqh6_b(s + 0xab) as i8 as i16).wrapping_sub(1);
                    while r >= 0 {
                        let n = cqh6_w(cqh6_d(s + 0xa3) + r as usize * 0x68 + 0x1a);
                        let confrontos = cqh6_d(cqh6_d(s + 0xad) + r as usize * 4);
                        let mut j = 0i16;
                        while j < n {
                            let c = confrontos.wrapping_add((j as i32 * 0x35) as usize);
                            if cqh6_w(c) == time || cqh6_w(c + 2) == time {
                                *rodada = cqh6_w(cqh6_d(s + 0xa3) + r as usize * 0x68 + 7);
                                return;
                            }
                            j += 1;
                        }
                        r -= 1;
                    }
                }
            }
        }
        f -= 1;
    }
}

/// Lado a lado nas três saídas (a original e a cópia com saídas próprias, começando dos valores de quem chama).
unsafe extern "thiscall" fn compare_find_club_progress(this: usize, clube: usize, fase_id: *mut i16, rodada: *mut i16, situacao: *mut u8) {
    let (mut f2, mut r2, mut s2) = (*fase_id, *rodada, *situacao);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i16, *mut i16, *mut u8)>(%%I2%%)(this, clube, fase_id, rodada, situacao);
    find_club_progress(this, clube, &mut f2, &mut r2, &mut s2);
    let ok = f2 == *fase_id && r2 == *rodada && s2 == *situacao;
    SCOREBOARD.record(ok, || format!("find_club_progress: original ({}, {}, {}) rust ({}, {}, {})", *fase_id, *rodada, *situacao, f2, r2, s2));
}
