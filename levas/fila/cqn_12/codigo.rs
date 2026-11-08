// ---------- %%LEVA%%a leva: os inscritos das copas do Japão e dos EUA: a Copa da Liga japonesa, a Copa dos EUA e a Copa do Imperador (jap_j_cup.cpp, usa_open_cup.cpp, jap_emp_cup.cpp) ----------

unsafe fn cqn12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn jap_j_cup_club_by_index(i: i32) -> usize {
    cqn12_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize)
}

/// O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
unsafe fn cqn12_comp(c: usize, o: usize) -> Option<usize> {
    let d = cqn12_d(c + o);
    if d == 0 { None } else { Some(cqn12_d(d)) }
}

/// Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação 0).
unsafe fn cqn12_inscrever(this: usize, k: i32, clube: usize, classe: u8) {
    let e = cqn12_d(this + 0xa7) + k as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = classe;
    *((e + 5) as *mut u8) = 0;
}

/// O sorteio de 0 a n − 1 (0x90cfc0: push n; add esp, 4 -> cdecl).
unsafe fn jap_j_cup_random(n: i32) -> i32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    sortear(n)
}

/// Ordena `n` clubes do vetor pela reputação e embaralha os melhores (0x4afbd0: push escolhas; push n; push vetor; add
/// esp, 0xc -> cdecl; já em Rust).
unsafe fn sort_and_shuffle_top_clubs(v: *mut usize, n: i32, escolhas: u32) {
    let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    // ponytail: a original empurra a conta com lixo nos 16 bits altos; a 0x4afbd0 lê a palavra
    f(v as usize, n as i16 as u16 as u32, escolhas);
}

/// O qsort de `n` inscritos a partir de `base` (0x945333 com 0x522190: push 0x522190; push 6; push n; push base; add
/// esp, 0x10 -> cdecl; 0x522190 já em Rust).
unsafe fn sort_entrants(base: usize, n: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(base, n, 6, 0x522190);
}

/// { 0x665a00 } (thiscall, ret 0, jap_j_cup.cpp): `JapJCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de 0xa8 em
/// +0xa7; sem memória, a caixa da linha 0x11a e −1). Os clubes da J2 ([0x9cf768]) enquanto cabem, classe 0; os da J1
/// ([0x9cf65c]) enquanto cabem, cada um com a classe sorteada de 0 a 19 (0x90cfc0(20), depois de gravar o clube); o
/// qsort (0x522190) das 16 inscrições a partir da 11ª (+0x42). Devolve 1 com a lista cheia, senão 0.
unsafe fn collect_j_cup_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x1c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xa8);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9eaf14, 0x11a);
        return (-1, 0);
    }
    let mut n = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf768)) && n < cqn12_w(this + 0x3e) as i32 {
            cqn12_inscrever(this, n, c, 0);
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf65c)) && n < cqn12_w(this + 0x3e) as i32 {
            let e = cqn12_d(this + 0xa7) + n as usize * 6;
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            crate::comparacao::ramo("cqn_12 semente da j1");
            let s = jap_j_cup_random(0x14); // cqn12: a classe sorteada dos clubes da J1
            *((e + 4) as *mut u8) = s as u8;
            *((e + 5) as *mut u8) = 0;
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    sort_entrants(cqn12_d(this + 0xa7) + 0x42, 0x10);
    ((n == cqn12_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn jap_j_cup_collect_entrants(this: usize) -> i32 {
    collect_j_cup_entrants(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 e sorteiam: o +0x3e, o [0xb67a34] e o estado do sorteador a
/// partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xa7, a cópia faz a sua,
/// comparada nos inscritos que ela gravou e liberada.
unsafe fn jap_j_cup_compare_entrant_list(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqn12_d(this + 0xa7));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(c);
        lr.set(cqn12_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 6;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", nome, this, a, b, igual, lista, k));
    a
}

unsafe extern "thiscall" fn comparar_jap_j_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    jap_j_cup_compare_entrant_list("jap_j_cup_collect_entrants", this, || o(this), || collect_j_cup_entrants(this))
}

/// { 0x90b530 } (thiscall, ret 0, usa_open_cup.cpp): `UsaOpenCup::collect_entrants()`: +0x3e = 32. Os clubes cuja
/// divisão (+0x57, o ponteiro) é a competição do banco de [0x9cf594] ou a de [0x9cf598] vão para dois vetores locais de
/// até 50 (a passada para quando os dois enchem); cada vetor é ordenado e embaralhado (0x4afbd0, 10 escolhas). A lista
/// (0x945501, o calloc de +0x3e × 6, em +0xa7; sem memória, a caixa da linha 0x138 e −1) leva os 10 primeiros de cada
/// vetor com a classe 2 e depois todos os clubes da MLS ([0x9cf590]) com a classe 1 (sem conferir o espaço). Devolve 1.
unsafe fn collect_us_open_cup_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x20);
    let comp = |id: usize| cqn12_d(0xae23d0).wrapping_add(cqn12_d(id).wrapping_mul(0x6b));
    let (a, b) = (comp(0x9cf594), comp(0x9cf598));
    // ponytail: os vetores locais da original começam com o que estava na pilha; com menos de 10 clubes numa das
    // ligas ela copiaria essa sobra (e a 0x4afbd0 já daria a caixa): a cópia começa com zero
    let mut va = [0usize; 50];
    let mut vb = [0usize; 50];
    let (mut na, mut nb) = (0i8, 0i8);
    let mut i = 0i32;
    while i < cqn12_d(0xae235c) as i32 {
        if na == 0x32 && nb == 0x32 {
            break;
        }
        let c = jap_j_cup_club_by_index(i);
        let d = cqn12_d(c + 0x57);
        if d == a && na < 0x32 {
            va[na as usize] = c;
            na += 1;
        } else if d == b && nb < 0x32 {
            vb[nb as usize] = c;
            nb += 1;
        }
        i += 1;
    }
    sort_and_shuffle_top_clubs(va.as_mut_ptr(), na as i32, 10);
    sort_and_shuffle_top_clubs(vb.as_mut_ptr(), nb as i32, 10);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let l = calloc(cqn12_w(this + 0x3e) as i32 as u32, 6);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0xaa20e0, 0x138);
        return (-1, 0);
    }
    let mut k = 0i8;
    for &c in va.iter().take(10).chain(vb.iter().take(10)) {
        cqn12_inscrever(this, k as i32, c, 2); // cqn12: os 10 de cada liga de baixo
        k = k.wrapping_add(1);
    }
    crate::comparacao::ramo("cqn_12 inscritos da copa dos eua");
    let mut i = 0i32;
    while i < cqn12_d(0xae235c) as i32 {
        let c = jap_j_cup_club_by_index(i);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf590)) {
            cqn12_inscrever(this, k as i32, c, 1);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    (1, k as i32)
}

unsafe extern "thiscall" fn jap_j_cup_collect_entrants_90b530(this: usize) -> i32 {
    collect_us_open_cup_entrants(this).0
}

unsafe extern "thiscall" fn comparar_jap_j_cup_collect_entrants_90b530(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    jap_j_cup_compare_entrant_list("jap_j_cup_collect_entrants_90b530", this, || o(this), || collect_us_open_cup_entrants(this))
}

/// { 0x6624f0 } (thiscall, ret 0, jap_emp_cup.cpp): `JapEmpCup::collect_entrants()`: +0x3e = 80; a lista (0x944e46 de
/// 0x1e0 em +0xa7; sem memória, a caixa da linha 0x147 e −1). Dos clubes japoneses (+0x53 = [0x9cf36c]) com divisão, em
/// vetores locais: os da [0x9cf678] (até 100), os da [0x9cf674] (até 30), os da [0x9cf670] (até 30) e os de outras
/// divisões que não a J2 ([0x9cf768]) nem a J1 ([0x9cf65c]) (até 6); sem nenhum destes, o último da [0x9cf678] sai de lá
/// para cá. Com menos de 47, 2, 3 ou 1 em cada um, a caixa da linha 0x178 e 0. Cada vetor é ordenado e embaralhado
/// (0x4afbd0 com 47, 2, 3 e 1 escolhas); a lista leva os 47, os 2 e os 3 primeiros (o último vetor não entra), classe 0;
/// depois os da J2 enquanto cabem (classe 0) e os da J1 enquanto cabem, com a classe = a posição da temporada anterior
/// (+0x5f), até encher; o qsort (0x522190) das 16 últimas inscrições. Devolve 1 com a lista cheia, senão 0.
unsafe fn collect_emperors_cup_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x50);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x1e0);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9eab90, 0x147);
        return (-1, 0);
    }
    let mut va = [0usize; 100];
    let mut vb = [0usize; 30];
    let mut vc = [0usize; 30];
    let mut vd = [0usize; 6];
    let (mut na, mut nb, mut nc, mut nd) = (0i8, 0i8, 0i8, 0i8);
    if cqn12_d(0xae235c) as i32 > 0 {
        let mut i: i16 = 0;
        while (i as i32) < cqn12_d(0xae235c) as i32 {
            let c = jap_j_cup_club_by_index(i as i32);
            if cqn12_d(c + 0x57) != 0 && cqn12_comp(c, 0x53) == Some(cqn12_d(0x9cf36c)) {
                let d = cqn12_d(cqn12_d(c + 0x57));
                if d == cqn12_d(0x9cf678) {
                    if na < 0x64 {
                        va[na as usize] = c;
                        na += 1;
                    }
                } else if d == cqn12_d(0x9cf674) {
                    if nb < 0x1e {
                        vb[nb as usize] = c;
                        nb += 1;
                    }
                } else if d == cqn12_d(0x9cf670) {
                    if nc < 0x1e {
                        vc[nc as usize] = c;
                        nc += 1;
                    }
                } else if d != cqn12_d(0x9cf768) && d != cqn12_d(0x9cf65c) && nd < 6 {
                    vd[nd as usize] = c;
                    nd += 1;
                }
            }
            i = i.wrapping_add(1);
        }
        if nd == 0 {
            na = na.wrapping_sub(1);
            nd = 1;
            vd[0] = if na >= 0 { va[na as usize] } else { 0 };
        }
    } else {
        // ponytail: sem clubes a original lê o vetor fora do lugar (índice −1); a cópia usa zero e cai na caixa abaixo
        na = na.wrapping_sub(1);
        nd = 1;
        vd[0] = 0;
    }
    if na < 0x2f || nb < 2 || nc < 3 || nd < 1 {
        show_error_at(0x9eab90, 0x178);
        return (0, 0);
    }
    sort_and_shuffle_top_clubs(va.as_mut_ptr(), na as i32, 0x2f);
    sort_and_shuffle_top_clubs(vb.as_mut_ptr(), nb as i32, 2);
    sort_and_shuffle_top_clubs(vc.as_mut_ptr(), nc as i32, 3);
    sort_and_shuffle_top_clubs(vd.as_mut_ptr(), nd as i32, 1);
    crate::comparacao::ramo("cqn_12 inscritos da copa do imperador");
    let mut n = 0i32;
    for &c in va.iter().take(0x2f).chain(vb.iter().take(2)).chain(vc.iter().take(3)) {
        cqn12_inscrever(this, n, c, 0); // cqn12: os sorteados das divisões de baixo
        n += 1;
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf768)) && n < cqn12_w(this + 0x3e) as i32 {
            cqn12_inscrever(this, n, c, 0);
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf65c)) && n < cqn12_w(this + 0x3e) as i32 {
            crate::comparacao::ramo("cqn_12 classe da j1 na copa do imperador");
            cqn12_inscrever(this, n, c, *((c + 0x5f) as *const u8)); // cqn12: a classe dos clubes da J1 na Copa do Imperador
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    sort_entrants(cqn12_d(this + 0xa7).wrapping_add(((n * 3 - 0x30) * 2) as isize as usize), 0x10);
    ((n == cqn12_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn jap_j_cup_collect_entrants_6624f0(this: usize) -> i32 {
    collect_emperors_cup_entrants(this).0
}

unsafe extern "thiscall" fn comparar_jap_j_cup_collect_entrants_6624f0(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    jap_j_cup_compare_entrant_list("jap_j_cup_collect_entrants_6624f0", this, || o(this), || collect_emperors_cup_entrants(this))
}
