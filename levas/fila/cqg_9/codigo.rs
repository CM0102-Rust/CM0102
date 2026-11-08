// ---------- %%LEVA%%a leva: os árbitros do jogo novo: os que ficam por nação, as vagas das competições e a parte de cada uma; a ordem das vagas pelo nível da competição (officials_manager.cpp) ----------

unsafe fn cqg9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqg9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg9_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cqg9_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqg9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro do árbitro `i` (0x2b bytes em [0xae23e8], [0xae2390] deles).
unsafe fn official_record(i: i32) -> usize {
    cqg9_d(0xae23e8).wrapping_add((i as isize * 0x2b) as usize)
}

/// O fim fatal da original sem memória: a caixa (linha `linha` de officials_manager.cpp, 0xa6a22c), 0x5cc7d0, 0x5a8f60
/// e exit(−1) (0x945934).
unsafe fn officials_out_of_memory(linha: u32) -> ! {
    show_error_at(0xa6a22c, linha);
    let f1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let f2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    f1();
    f2();
    sair(-1)
}

/// O qsort do jogo (0x945333) pela rotina de comparação `cmp`.
unsafe fn officials_manager_crt_qsort(base: usize, n: u32, tam: u32, cmp: usize) {
    let q: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    q(base, n, tam, cmp);
}

/// calloc (0x945501, cdecl).
unsafe fn crt_calloc(n: u32, tam: u32) -> usize {
    let f: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    f(n, tam)
}

/// free (0x9452ca, cdecl).
unsafe fn crt_free(p: usize) {
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    f(p);
}

/// { 0x79bf60 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::set_up_officials()`, pelo construtor
/// (0x79b580) no jogo novo. Revê os registros (0x79caa0), ordena por nação e pela qualidade (qsort 0x79d4e0) e, por
/// nação, fica com os 6 primeiros (nação com +0x11c zero) ou 61 (o primeiro de cada nação depois da primeira não fica:
/// a conta recomeça nele), copiados (0x79c530) numa tabela nova; os blocos por nação (this+0xc, 0x102 bytes: +0 a nação,
/// +4 os árbitros, +8 o primeiro, +0xc o pedido, +0x10 as competições e 15 vagas de 16 bytes em +0x12): cada liga
/// (0x4b0700, não amistosa 0x5c6710, com nação +0x5d) ganha uma vaga com o pedido (+0x3e da competição) / 2 + 10; a nação
/// com competições pede o maior × a quantidade e, faltando, sorteia os que faltam (0x79c5c0); sem competições e menos de
/// 5, sorteia até o +0x8e da nação (1..5). Os índices (+0), o primeiro de cada nação e, por nação, as vagas ordenadas
/// (qsort 0x79d410) com a parte de cada uma; this+4 a conta e this+8 as marcas (calloc).
unsafe extern "thiscall" fn set_up_officials(this: usize) {
    let rever: unsafe extern "C" fn() = core::mem::transmute(0x79caa0usize);
    rever();
    officials_manager_crt_qsort(cqg9_d(0xae23e8), cqg9_i(0xae2390) as u32, 0x2b, 0x79d4e0);
    let n = cqg9_i(0xae2390);
    let marcas = crt_calloc(n as u32, 1);
    let mut ficam = 0i32;
    if n > 0 {
        let mut atual = cqg9_d(official_record(0) + 0x16);
        let mut grupo = 0i32;
        for i in 0..n {
            let nac = cqg9_d(official_record(i) + 0x16);
            if nac != atual {
                grupo = 0;
                atual = nac;
                continue;
            }
            let m = (marcas + i as usize) as *mut u8;
            if nac == 0 {
                *m = 0;
                continue;
            }
            let limit = if *((nac + 0x11c) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: os árbitros que ficam por nação
            if grupo <= limit {
                ficam += 1;
                *m = 1;
                crate::comparison::branch("cqg_9 arbitro mantido");
            } else {
                *m = 0;
            }
            grupo += 1;
        }
    }
    let nova = crt_calloc(ficam as u32, 0x2b);
    let mut dst = nova;
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x79c530usize);
    let mut i = 0i32;
    while i < cqg9_i(0xae2390) {
        if *((marcas + i as usize) as *const u8) != 0 {
            copiar(dst, official_record(i));
            dst += 0x2b;
        }
        i += 1;
    }
    crt_free(cqg9_d(0xae23e8));
    cqg9_pd(0xae23e8, nova as u32);
    cqg9_pd(0xae2390, ficam as u32);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let nacoes = cqg9_i(0xae2350);
    let blocos = malloc((nacoes as u32).wrapping_mul(0x102));
    cqg9_pd(this + 0xc, blocos as u32);
    if blocos == 0 {
        officials_out_of_memory(0x1ff);
    }
    for k in 0..nacoes.max(0) {
        let b = blocos + k as usize * 0x102;
        cqg9_pw(b + 0x10, 0);
        cqg9_pd(b + 4, 0);
        cqg9_pd(b + 8, 1_000_000);
        cqg9_pd(b + 0xc, 0);
        cqg9_pd(b, k as u32);
        for j in 0..0xf {
            let v = b + 0x12 + 16 * j;
            cqg9_pd(v, 1_000_000);
            cqg9_pd(v + 4, 0);
            cqg9_pd(v + 8, 0);
            cqg9_pd(v + 0xc, 0xffff_ffff);
        }
    }
    let ativa: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    for k in 0..cqg9_i(0xadadf8).max(0) {
        if cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) == 0 || ativa(k) == 0 {
            continue;
        }
        let comp = cqg9_d(0xae23d0).wrapping_add(k as usize * 0x6b);
        if amistoso(cqg9_d(0xae2c74), comp) != 0 {
            continue;
        }
        let nac = cqg9_d(comp + 0x5d);
        if nac == 0 {
            continue;
        }
        let b = blocos.wrapping_add((cqg9_i(nac) as isize * 0x102) as usize);
        let v = b.wrapping_add((cqg9_w(b + 0x10) as isize * 0x10) as usize);
        cqg9_pd(v + 0x1e, k as u32);
        let pedido = cqg9_w(cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xa; // cqg9: o pedido da vaga
        crate::comparison::branch("cqg_9 vaga de competicao");
        cqg9_pd(v + 0x1a, pedido as u32);
        if cqg9_i(v + 0x1a) > cqg9_i(b + 0xc) {
            cqg9_pd(b + 0xc, cqg9_d(v + 0x1a) as u32);
        }
        cqg9_pw(b + 0x10, cqg9_w(b + 0x10).wrapping_add(1));
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        let b = blocos.wrapping_add((cqg9_i(cqg9_d(official_record(i) + 0x16)) as isize * 0x102) as usize);
        cqg9_pd(b + 4, cqg9_d(b + 4).wrapping_add(1) as u32);
    }
    let novos: unsafe extern "stdcall" fn(i32, usize) = core::mem::transmute(0x79c5c0usize);
    for k in 0..cqg9_i(0xae2350).max(0) {
        let b = blocos + k as usize * 0x102;
        let nac = cqg9_d(0xae23a8).wrapping_add(k as usize * 0x122);
        let c = cqg9_w(b + 0x10);
        if c > 0 {
            cqg9_pd(b + 0xc, (c as i32).wrapping_mul(cqg9_i(b + 0xc)) as u32);
            if cqg9_i(b + 0xc) > cqg9_i(b + 4) {
                novos(cqg9_i(b + 0xc) - cqg9_i(b + 4), nac);
                cqg9_pd(b + 4, cqg9_d(b + 0xc) as u32);
            }
        } else if cqg9_i(b + 4) < 5 {
            if cqg9_w(nac + 0x8e) == 0 {
                cqg9_pw(nac + 0x8e, 1);
            }
            novos((cqg9_w(nac + 0x8e) as i32).min(5), nac);
            cqg9_pd(b + 4, (cqg9_w(cqg9_d(0xae23a8).wrapping_add(k as usize * 0x122) + 0x8e) as i32).min(5) as u32);
        }
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        cqg9_pd(official_record(i), i as u32);
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        let r = official_record(i);
        let b = blocos.wrapping_add((cqg9_i(cqg9_d(r + 0x16)) as isize * 0x102) as usize);
        if cqg9_i(r) < cqg9_i(b + 8) {
            cqg9_pd(b + 8, cqg9_d(r) as u32);
        }
    }
    for k in 0..cqg9_i(0xae2350).max(0) {
        let b = cqg9_d(this + 0xc) + k as usize * 0x102;
        let c = cqg9_w(b + 0x10);
        if c == 0 {
            continue;
        }
        officials_manager_crt_qsort(b + 0x12, c as i32 as u32, 0x10, 0x79d410);
        let mut j = 0i32;
        while j < cqg9_w(b + 0x10) as i32 {
            let c = cqg9_w(b + 0x10) as i32;
            crate::comparison::branch("cqg_9 parte da vaga");
            cqg9_pd(b + 0x16 + 16 * j as usize, (cqg9_i(b + 4) / c) as u32); // cqg9: a parte de cada vaga
            let c = cqg9_w(b + 0x10) as i32;
            cqg9_pd(b + 0x12 + 16 * j as usize, (cqg9_i(b + 4).wrapping_mul(j) / c).wrapping_add(cqg9_i(b + 8)) as u32);
            j += 1;
        }
    }
    let total = cqg9_i(0xae2390);
    cqg9_pd(this + 4, total as u32);
    let m = crt_calloc(total as u32, 1);
    cqg9_pd(this + 8, m as u32);
    if m == 0 {
        officials_out_of_memory(0x2dd);
    }
    crt_free(marcas);
}

/// Comparador: só a original (ponytail: aloca, libera e troca a tabela dos árbitros, e as chamadas sorteiam: não pode
/// rodar duas vezes), com o plano: os blocos por nação (this+0xc), a conta (this+4) igual à da tabela e as marcas (this+8).
unsafe extern "thiscall" fn compare_set_up_officials(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    SCOREBOARD.record(cqg9_d(this + 0xc) != 0 && cqg9_i(this + 4) == cqg9_i(0xae2390) && cqg9_d(this + 8) != 0,
        || format!("set_up_officials: blocos {:x}, conta {} x {}", cqg9_d(this + 0xc), cqg9_i(this + 4), cqg9_i(0xae2390)));
}

unsafe fn cqg9v_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqg9v_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x79d410 } (cdecl, officials_manager.cpp): `officials_manager_compare_comp_level(a, b)`, a ordem das vagas de
/// competição de uma nação (qsort da 0x79bf60, entradas de 16 bytes com a competição em +0xc): competição de `a` fora de
/// 0..[0xae2378]: 1; a de `b` fora: −1; pelo nível (+0x69, palavra, da competição de 0x6b bytes em [0xae23d0]); mesmo
/// nível: a diferença dos ids.
unsafe extern "C" fn officials_manager_compare_comp_level(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cqg9v_i(a + 0xc), cqg9v_i(b + 0xc));
    let max = cqg9v_i(0xae2378);
    if ca < 0 || ca > max {
        return 1;
    }
    if cb < 0 || cb > max {
        return -1;
    }
    let base = cqg9v_i(0xae23d0) as u32 as usize;
    let sa = cqg9v_w(base.wrapping_add((ca as isize * 0x6b + 0x69) as usize));
    let signed_byte_at = cqg9v_w(base.wrapping_add((cb as isize * 0x6b + 0x69) as usize));
    if sa == signed_byte_at {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - signed_byte_at as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn compare_officials_manager_compare_comp_level(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = officials_manager_compare_comp_level(a, b);
    SCOREBOARD.record(r == n, || format!("officials_manager_compare_comp_level({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
