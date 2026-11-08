// ---------- %%LEVA%%a leva: os árbitros do jogo novo: os árbitros sorteados de uma nação, o conserto das fichas dos árbitros e a cópia de uma ficha (officials_manager.cpp) ----------

unsafe fn cpy9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy9_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpy9_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpy9_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// Um byte sorteado: sorteio(n) + k no al.
unsafe fn random_byte_plus(n: i32, k: u8) -> u8 {
    (sortear(n) as u8).wrapping_add(k)
}

/// Uma ficha de árbitro nova em `r` (0x2b bytes) da nação `nacao`, na ordem da 0x79c6aa: primeiro e segundo nome pelo
/// peso (0x539b90, 0x53a0e0), a nação (+0x16), as palavras +0x22 sorteio(80) + 1, +0x1e e +0x20 sorteio(80) + 40 (o
/// byte estendido), os bytes +0x24..+0x26 sorteio(10) + 5 e +0x27..+0x2a sorteio(10) + 7, +0x1a zero, o ano (+0xe)
/// 1960 + sorteio(10) e o dia (+0xc) sorteio(300) + 1. Não grava +0, +0x10..+0x15 (a original deixa o que o realloc
/// trouxe).
unsafe fn generate_official(r: usize, nacao: usize) {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539b90usize);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a0e0usize);
    cpy9_pd(r + 4, primeiro(nacao));
    cpy9_pd(r + 8, segundo(nacao));
    cpy9_pd(r + 0x16, nacao);
    cpy9_pw(r + 0x22, random_byte_plus(0x50, 1) as i8 as i16); // cpy9: a nota +0x22
    cpy9_pw(r + 0x1e, random_byte_plus(0x50, 0x28) as i8 as i16);
    cpy9_pw(r + 0x20, random_byte_plus(0x50, 0x28) as i8 as i16);
    for o in [0x24usize, 0x25, 0x26] {
        cpy9_pb(r + o, random_byte_plus(10, 5));
    }
    cpy9_pb(r + 0x27, random_byte_plus(10, 7));
    cpy9_pb(r + 0x28, random_byte_plus(10, 7)); // cpy9: o atributo +0x28
    cpy9_pb(r + 0x29, random_byte_plus(10, 7));
    cpy9_pb(r + 0x2a, random_byte_plus(10, 7));
    cpy9_pd(r + 0x1a, 0);
    cpy9_pw(r + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
    cpy9_pw(r + 0xc, sortear(0x12c).wrapping_add(1) as i16);
}

/// { 0x79c5c0 } (thiscall com o this sem uso, ret 8, officials_manager.cpp): `officials_manager_add_random_officials(
/// conta, nacao)`, pela montagem dos árbitros do jogo novo (0x79bf60, nas nações com poucos árbitros). Nação sem
/// continente (+0x71): nada. Realoca a tabela ([0xae23e8], 0x945667) para [0xae2390] + conta fichas (sem memória: erro
/// e o jogo para), gera `conta` fichas novas no fim (generate_official), soma a conta em [0xae2390] e ordena a tabela inteira
/// (qsort 0x945333 com 0x79d480: nação, +0x28 × 50 + +0x22, id).
unsafe extern "thiscall" fn officials_manager_add_random_officials(_this: usize, conta: i32, nacao: usize) {
    if nacao != 0 && cpy9_d(nacao + 0x71) == 0 {
        return;
    }
    let realocar: unsafe extern "C" fn(usize, i32) -> usize = core::mem::transmute(0x945667usize);
    let antes = cpy9_i(0xae2390);
    let t = realocar(cpy9_d(0xae23e8), antes.wrapping_add(conta).wrapping_mul(0x2b));
    *(0xae23e8 as *mut usize) = t;
    if t == 0 {
        fatal_error_at(0xa6a22c, 0x342);
    }
    let mut r = t.wrapping_add(antes.wrapping_mul(0x2b) as usize);
    for _ in 0..conta.max(0) {
        crate::comparison::branch("cpy_9 arbitro novo");
        generate_official(r, nacao);
        r += 0x2b;
    }
    let n = cpy9_i(0xae2390).wrapping_add(conta);
    *(0xae2390 as *mut i32) = n;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparison::branch("cpy_9 ordenar");
    ordenar(cpy9_d(0xae23e8), n, 0x2b, 0x79d480); // cpy9: ordenar a tabela
}

/// A ficha `g` (gerada no ensaio) bate com a `f` da tabela nos campos que a 0x79c5c0 grava (+4..+0x10, +0x16..+0x2b)?
unsafe fn same_official_record(g: &[u8; 0x2b], f: usize) -> bool {
    let t = core::slice::from_raw_parts(f as *const u8, 0x2b);
    g[4..0x10] == t[4..0x10] && g[0x16..] == t[0x16..]
}

/// Comparador: realoca e ordena (não pode rodar duas vezes): só a original, depois de um ensaio. O ensaio gera as
/// `conta` fichas da cópia (generate_official, sem contar os sorteios) numa área própria a partir do mesmo estado do sorteio,
/// que é reposto; depois roda a original. Confere o estado final do sorteio, a conta nova ([0xae2390] + conta) e que
/// cada ficha do ensaio esteja na tabela ordenada (campos que a original grava). Conta não positiva ou nação sem
/// continente: só a original.
unsafe extern "thiscall" fn comparar_officials_manager_add_random_officials(this: usize, conta: i32, nacao: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(%%I0%%);
    if conta <= 0 || (nacao != 0 && cpy9_d(nacao + 0x71) == 0) {
        return o(this, conta, nacao);
    }
    let estado = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let mut novas = vec![[0u8; 0x2b]; conta as usize];
    crate::rng::pause_trace(|| crate::rng::without_counting(|| {
        for g in novas.iter_mut() {
            generate_official(g.as_mut_ptr() as usize, nacao);
        }
    }));
    let estado_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    estado.write_back();
    let antes = cpy9_i(0xae2390);
    o(this, conta, nacao);
    let estado_orig = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let total = cpy9_i(0xae2390);
    let t = cpy9_d(0xae23e8);
    let mut usada = vec![false; total.max(0) as usize];
    let mut faltam = Vec::new();
    for (k, g) in novas.iter().enumerate() {
        match (0..usada.len()).find(|&j| !usada[j] && same_official_record(g, t + j * 0x2b)) {
            Some(j) => usada[j] = true,
            None => faltam.push(k),
        }
    }
    let igual = estado_orig == estado_rust && total == antes.wrapping_add(conta) && faltam.is_empty();
    PLACAR.record(igual, || format!("officials_manager_add_random_officials(conta {}, nacao {:#x}) total {} (antes {}), sorteio igual {}, fichas da copia ausentes {:?}",
        conta, nacao, total, antes, estado_orig == estado_rust, faltam));
}

/// O ajuste de um atributo em byte (+0x24..+0x2a) fora de 1..20: o valor sorteado por `f` (lê a ficha antes).
unsafe fn fix_official_attribute(e: usize, o: usize, f: impl FnOnce() -> u8) {
    let v = cpy9_b(e + o) as i8;
    if v <= 0 || v > 0x14 {
        cpy9_pb(e + o, f());
    }
}

/// { 0x79caa0 } (cdecl, officials_manager.cpp): `officials_manager_validate_officials()`, no começo da montagem dos
/// árbitros do jogo novo (0x79bf60). Para cada uma das [0xae2390] fichas (0x2b bytes em [0xae23e8]): sem nação (+0x16),
/// a da tabela das nações pelo índice (+0x37) do primeiro nome (+4), do segundo (+8) ou sorteado; as palavras +0x22,
/// +0x1e e +0x20 fora de 1..200: sorteio(80) + 1 mais o byte +0x8e da nação (+0x22) ou 4 vezes ele (+0x1e, +0x20; a
/// +0x20 refeita fica no mínimo na +0x1e); os bytes +0x24..+0x2a fora de 1..20 sorteados pelas faixas de +0x1e (acima de
/// 120, de 80); nascido antes da data 0x549f70(1, 1, 1910, −1) (ano sem sinal, depois o dia com sinal), ano 1960 +
/// sorteio(10) e dia sorteio(300) + 1.
unsafe extern "C" fn officials_manager_validate_officials() {
    let base = cpy9_d(0xae23e8);
    let mut i = 0i32;
    while i < cpy9_i(0xae2390) {
        let e = base.wrapping_add((i as usize).wrapping_mul(0x2b));
        if cpy9_d(e + 0x16) == 0 {
            let (n1, n2) = (cpy9_d(e + 4), cpy9_d(e + 8));
            let k = if cpy9_i(n1 + 0x37) >= 0 {
                cpy9_i(n1 + 0x37)
            } else if cpy9_i(n2 + 0x37) >= 0 {
                cpy9_i(n2 + 0x37)
            } else {
                sortear(cpy9_i(0xae2350))
            };
            cpy9_pd(e + 0x16, cpy9_d(0xae23a8).wrapping_add(k.wrapping_mul(0x122) as usize));
        }
        let fora = |o: usize| {
            let v = cpy9_w(e + o);
            v <= 0 || v > 0xc8
        };
        if fora(0x22) {
            let nac = cpy9_d(e + 0x16);
            let v = if nac != 0 {
                let s = sortear(0x50) as u8;
                s.wrapping_add(cpy9_b(cpy9_d(e + 0x16) + 0x8e)).wrapping_add(1)
            } else {
                (sortear(0x50) as u8).wrapping_add(1)
            };
            cpy9_pw(e + 0x22, v as i8 as i16);
        }
        for o in [0x1eusize, 0x20] {
            if fora(o) {
                crate::comparison::branch("cpy_9 nota consertada");
                let nac = cpy9_d(e + 0x16);
                let v = if nac != 0 {
                    let s = sortear(0x50) as u8;
                    s.wrapping_add(cpy9_b(cpy9_d(e + 0x16) + 0x8e) << 2).wrapping_add(1)
                } else {
                    (sortear(0x50) as u8).wrapping_add(1)
                };
                cpy9_pw(e + o, v as i8 as i16);
                if o == 0x20 && cpy9_w(e + 0x20) < cpy9_w(e + 0x1e) {
                    cpy9_pw(e + 0x20, cpy9_w(e + 0x1e));
                }
            }
        }
        let band = |alto: (i32, u8), medio: (i32, u8), baixo: (i32, u8)| {
            let w = cpy9_w(e + 0x1e);
            let (n, k) = if w > 0x78 { alto } else if w > 0x50 { medio } else { baixo };
            random_byte_plus(n, k)
        };
        fix_official_attribute(e, 0x24, || if cpy9_w(e + 0x1e) > 0x64 { random_byte_plus(0xc, 7) } else { random_byte_plus(0xa, 5) });
        fix_official_attribute(e, 0x25, || band((0xe, 7), (0xf, 5), (0xa, 5)));
        fix_official_attribute(e, 0x26, || band((0xb, 0xa), (0xf, 5), (0xa, 5)));
        fix_official_attribute(e, 0x27, || if cpy9_w(e + 0x1e) > 0x50 { random_byte_plus(0xf, 6) } else { random_byte_plus(0x14, 1) });
        fix_official_attribute(e, 0x28, || band((0xa, 0xb), (0xf, 5), (0xa, 7)));
        fix_official_attribute(e, 0x29, || band((0xd, 8), (0xe, 7), (0xf, 6)));
        fix_official_attribute(e, 0x2a, || if sortear(0x64) != 0 { random_byte_plus(0xa, 0xb) } else { random_byte_plus(0xa, 5) });
        let mut limit = [0u8; 8];
        let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> *const u8 = core::mem::transmute(0x549f70usize);
        let d = data(limit.as_mut_ptr(), 1, 1, 0x776, -1);
        let (dia, ano) = (cpy9_w(d as usize), cpy9_w(d as usize + 2) as u16);
        let ano_e = cpy9_w(e + 0xe) as u16;
        if ano > ano_e || (ano == ano_e && dia > cpy9_w(e + 0xc)) {
            cpy9_pw(e + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
            cpy9_pw(e + 0xc, sortear(0x12c).wrapping_add(1) as i16);
        }
        i += 1;
    }
}

/// Comparador: lado a lado pelas fichas de todos os árbitros e pelo estado do sorteio.
unsafe extern "C" fn comparar_officials_manager_validate_officials() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let total = cpy9_i(0xae2390).max(0) as usize;
    if cpy9_d(0xae23e8) == 0 {
        return o();
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((cpy9_d(0xae23e8), total * 0x2b));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| officials_manager_validate_officials()));
    PLACAR.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("officials_manager_validate_officials ({} fichas) diferencas em {:x?}", total, dif)
    });
}

/// { 0x79c530 } (thiscall, ret 4, officials_manager.cpp): `Official::operator=(src)`, pela montagem dos árbitros do
/// jogo novo (0x79bf60, as fichas que ficam). Copia a ficha de 0x2b bytes campo a campo (+0, +4, +8; a data de +0xc
/// pela 0x417c10; +0x14, +0x16, +0x1a, +0x1e, +0x20, +0x22, os bytes +0x24..+0x2a). Devolve `this`.
unsafe extern "thiscall" fn officials_manager_operator(this: usize, src: usize) -> usize {
    for o in [0usize, 4, 8] {
        cpy9_pd(this + o, cpy9_d(src + o));
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(this + 0xc, t.as_mut_ptr(), src + 0xc);
    cpy9_pw(this + 0x14, cpy9_w(src + 0x14));
    cpy9_pd(this + 0x16, cpy9_d(src + 0x16));
    cpy9_pd(this + 0x1a, cpy9_d(src + 0x1a));
    for o in [0x1eusize, 0x20, 0x22] {
        cpy9_pw(this + o, cpy9_w(src + o));
    }
    for o in 0x24usize..0x2b {
        cpy9_pb(this + o, cpy9_b(src + o));
    }
    this
}

/// Comparador: lado a lado pela ficha de destino e pela devolução.
unsafe extern "thiscall" fn comparar_officials_manager_operator(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x2b)], || o(this, src), || officials_manager_operator(this, src));
    PLACAR.record(igual && r == n, || format!("officials_manager_operator(src {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", src, r, mo, n, mr));
    r
}
