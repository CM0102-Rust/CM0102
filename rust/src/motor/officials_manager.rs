// ---------- centesima trigesima setima leva: a nota do arbitro (0x6d4a20, 0x79ce10) ----------

/// { 0x79ce10 } (ret 4, officials_manager.cpp): soma a nota `r` (dword id, bytes: cartoes, faltas,
/// nota) na ficha do arbitro (17 bytes em [this] + id x 17, id em 0..[0xae2390]): +0xf os cartoes,
/// as palavras +8 e +0xa as faltas e a nota, +0x10 a ultima nota, +0xc os jogos e +4 a media
/// (+0xa / +0xc, float).
unsafe extern "thiscall" fn registrar_nota_do_arbitro(this: usize, r: *const u8) {
    let base = *(this as *const usize);
    if base == 0 || r.is_null() {
        return;
    }
    let id = *(r as *const i32);
    if id < 0 || id >= *(0xae2390 as *const i32) {
        return;
    }
    let f = base + id as usize * 17;
    let b = |o: usize| (f + o) as *mut u8;
    let w = |o: usize| (f + o) as *mut i16;
    *b(0xf) = (*b(0xf)).wrapping_add(*r.add(4));
    *w(8) = (*w(8)).wrapping_add(*r.add(5) as i8 as i16);
    *w(0xa) = (*w(0xa)).wrapping_add(*r.add(6) as i8 as i16);
    *b(0x10) = *r.add(6);
    *w(0xc) = (*w(0xc)).wrapping_add(1);
    if *w(0xc) != 0 {
        *((f + 4) as *mut f32) = (*w(0xa) as f64 / *w(0xc) as f64) as f32;
    }
}

/// { 0x6d4a20 } (match_eng.cpp, chamada pela 0x6b98c0): a nota do arbitro da partida (+0xf5b6, id
/// != -1), fora das competicoes amistosas (0x5c6710 em [0xae2c74]). Os cartoes (+0xf5cb e +0xf601 dos
/// dois lados) e as faltas (+0xf5cc e +0xf602); a nota parte de +0xf5ba / 1000, perde 2 com mais de 2
/// cartoes, 1 com 2, ganha 1 sem cartao; perde 1 com mais de 5 faltas, ganha 1 com menos de 3. Abaixo
/// de 4 vira 4 (salvo 1 em 500), acima de 8 vira 8 (salvo 1 em 25), e fica entre 1 e 10. Grava pela
/// 0x79ce10 em [0xb74340].
unsafe extern "thiscall" fn nota_do_arbitro(m: usize) {
    let arbitro = *((m + 0xf5b6) as *const usize);
    if arbitro == 0 || *(arbitro as *const i32) == -1 {
        return;
    }
    let partida = *((m + 0x4792) as *const usize);
    if partida == 0 {
        return;
    }
    let comp = *((partida + 0x14) as *const usize);
    if comp == 0 {
        return;
    }
    let amistosos = *(0xae2c74 as *const usize);
    if amistosos != 0 {
        let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistosa(amistosos, comp) != 0 {
            crate::comparacao::ramo("nota do arbitro amistoso");
            return;
        }
    }
    let cartoes = (*((m + 0xf601) as *const u8)).wrapping_add(*((m + 0xf5cb) as *const u8)) as i8;
    let faltas = (*((m + 0xf602) as *const u8)).wrapping_add(*((m + 0xf5cc) as *const u8)) as i8;
    let mut nota = (*((m + 0xf5ba) as *const i16) as i32 / 1000) as i8;
    if cartoes > 2 {
        nota = nota.wrapping_sub(2);
    } else if cartoes > 1 {
        nota = nota.wrapping_sub(1);
    } else if cartoes == 0 {
        nota = nota.wrapping_add(1);
    }
    if faltas > 5 {
        nota = nota.wrapping_sub(1);
    } else if faltas < 3 {
        nota = nota.wrapping_add(1);
    }
    if nota < 4 && sortear(500) != 0 {
        nota = 4;
    } else if nota > 8 && sortear(0x19) != 0 {
        nota = 8;
    } else if nota < 1 {
        crate::comparacao::ramo("nota do arbitro minima");
        nota = 1;
    } else if nota > 10 {
        crate::comparacao::ramo("nota do arbitro maxima");
        nota = 10;
    }
    let mut r = [0u8; 8];
    r[..4].copy_from_slice(&(*(arbitro as *const i32)).to_le_bytes());
    r[4] = cartoes as u8;
    r[5] = faltas as u8;
    r[6] = nota as u8;
    let registrar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x79ce10usize);
    registrar(*(0xb74340 as *const usize), r.as_ptr());
}

/// A ficha do arbitro que a 0x79ce10 mexe (vazia se o id nao vale).
unsafe fn ficha_do_arbitro(gerente: usize, id: i32) -> Vec<(usize, usize)> {
    let base = *(gerente as *const usize);
    if base == 0 || id < 0 || id >= *(0xae2390 as *const i32) {
        return Vec::new();
    }
    vec![(base + id as usize * 17, 17)]
}

unsafe extern "thiscall" fn comparar_registrar_nota_do_arbitro(this: usize, r: *const u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const u8)>(303);
    if gravando_no_topo() || r.is_null() {
        return o(this, r);
    }
    let reg = ficha_do_arbitro(this, *(r as *const i32));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, r), || registrar_nota_do_arbitro(this, r));
    PLACAR.registrar(igual, || format!("registrar_nota_do_arbitro(nota {}) {:x?} x {:x?}", *r.add(6), mo, mr));
}

unsafe extern "thiscall" fn comparar_nota_do_arbitro(m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(302);
    if gravando_no_topo() {
        return o(m);
    }
    // o sorteio e a ficha do arbitro
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let arbitro = *((m + 0xf5b6) as *const usize);
    if arbitro != 0 && *(0xb74340 as *const usize) != 0 {
        reg.extend(ficha_do_arbitro(*(0xb74340 as *const usize), *(arbitro as *const i32)));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m), || nota_do_arbitro(m));
    PLACAR.registrar(igual, || format!("nota_do_arbitro {:x?} x {:x?}", mo, mr));
}

// ---------- 271a leva: os árbitros do jogo novo: os que ficam por nação, as vagas das competições e a parte de cada uma; a ordem das vagas pelo nível da competição (officials_manager.cpp) ----------

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
unsafe fn cqg9_arbitro(i: i32) -> usize {
    cqg9_d(0xae23e8).wrapping_add((i as isize * 0x2b) as usize)
}

/// O fim fatal da original sem memória: a caixa (linha `linha` de officials_manager.cpp, 0xa6a22c), 0x5cc7d0, 0x5a8f60
/// e exit(−1) (0x945934).
unsafe fn cqg9_fatal(linha: u32) -> ! {
    aviso_em(0xa6a22c, linha);
    let f1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let f2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    f1();
    f2();
    sair(-1)
}

/// O qsort do jogo (0x945333) pela rotina de comparação `cmp`.
unsafe fn cqg9_ordenar(base: usize, n: u32, tam: u32, cmp: usize) {
    let q: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    q(base, n, tam, cmp);
}

/// calloc (0x945501, cdecl).
unsafe fn cqg9_calloc(n: u32, tam: u32) -> usize {
    let f: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    f(n, tam)
}

/// free (0x9452ca, cdecl).
unsafe fn cqg9_liberar(p: usize) {
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
unsafe extern "thiscall" fn montar_os_arbitros(this: usize) {
    let rever: unsafe extern "C" fn() = core::mem::transmute(0x79caa0usize);
    rever();
    cqg9_ordenar(cqg9_d(0xae23e8), cqg9_i(0xae2390) as u32, 0x2b, 0x79d4e0);
    let n = cqg9_i(0xae2390);
    let marcas = cqg9_calloc(n as u32, 1);
    let mut ficam = 0i32;
    if n > 0 {
        let mut atual = cqg9_d(cqg9_arbitro(0) + 0x16);
        let mut grupo = 0i32;
        for i in 0..n {
            let nac = cqg9_d(cqg9_arbitro(i) + 0x16);
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
            let limite = if *((nac + 0x11c) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: os árbitros que ficam por nação
            if grupo <= limite {
                ficam += 1;
                *m = 1;
                crate::comparacao::ramo("cqg_9 arbitro mantido");
            } else {
                *m = 0;
            }
            grupo += 1;
        }
    }
    let nova = cqg9_calloc(ficam as u32, 0x2b);
    let mut dst = nova;
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x79c530usize);
    let mut i = 0i32;
    while i < cqg9_i(0xae2390) {
        if *((marcas + i as usize) as *const u8) != 0 {
            copiar(dst, cqg9_arbitro(i));
            dst += 0x2b;
        }
        i += 1;
    }
    cqg9_liberar(cqg9_d(0xae23e8));
    cqg9_pd(0xae23e8, nova as u32);
    cqg9_pd(0xae2390, ficam as u32);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let nacoes = cqg9_i(0xae2350);
    let blocos = malloc((nacoes as u32).wrapping_mul(0x102));
    cqg9_pd(this + 0xc, blocos as u32);
    if blocos == 0 {
        cqg9_fatal(0x1ff);
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
        crate::comparacao::ramo("cqg_9 vaga de competicao");
        cqg9_pd(v + 0x1a, pedido as u32);
        if cqg9_i(v + 0x1a) > cqg9_i(b + 0xc) {
            cqg9_pd(b + 0xc, cqg9_d(v + 0x1a) as u32);
        }
        cqg9_pw(b + 0x10, cqg9_w(b + 0x10).wrapping_add(1));
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        let b = blocos.wrapping_add((cqg9_i(cqg9_d(cqg9_arbitro(i) + 0x16)) as isize * 0x102) as usize);
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
        cqg9_pd(cqg9_arbitro(i), i as u32);
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        let r = cqg9_arbitro(i);
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
        cqg9_ordenar(b + 0x12, c as i32 as u32, 0x10, 0x79d410);
        let mut j = 0i32;
        while j < cqg9_w(b + 0x10) as i32 {
            let c = cqg9_w(b + 0x10) as i32;
            crate::comparacao::ramo("cqg_9 parte da vaga");
            cqg9_pd(b + 0x16 + 16 * j as usize, (cqg9_i(b + 4) / c) as u32); // cqg9: a parte de cada vaga
            let c = cqg9_w(b + 0x10) as i32;
            cqg9_pd(b + 0x12 + 16 * j as usize, (cqg9_i(b + 4).wrapping_mul(j) / c).wrapping_add(cqg9_i(b + 8)) as u32);
            j += 1;
        }
    }
    let total = cqg9_i(0xae2390);
    cqg9_pd(this + 4, total as u32);
    let m = cqg9_calloc(total as u32, 1);
    cqg9_pd(this + 8, m as u32);
    if m == 0 {
        cqg9_fatal(0x2dd);
    }
    cqg9_liberar(marcas);
}

/// Comparador: só a original (ponytail: aloca, libera e troca a tabela dos árbitros, e as chamadas sorteiam: não pode
/// rodar duas vezes), com o plano: os blocos por nação (this+0xc), a conta (this+4) igual à da tabela e as marcas (this+8).
unsafe extern "thiscall" fn comparar_montar_os_arbitros(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1635)(this);
    PLACAR.registrar(cqg9_d(this + 0xc) != 0 && cqg9_i(this + 4) == cqg9_i(0xae2390) && cqg9_d(this + 8) != 0,
        || format!("montar_os_arbitros: blocos {:x}, conta {} x {}", cqg9_d(this + 0xc), cqg9_i(this + 4), cqg9_i(0xae2390)));
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
unsafe extern "C" fn ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
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
    let sb = cqg9v_w(base.wrapping_add((cb as isize * 0x6b + 0x69) as usize));
    if sa == sb {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - sb as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1636)(a, b);
    let n = ordem_das_vagas_de_competicao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_vagas_de_competicao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 271a leva: a ordem dos árbitros por nação e qualidade, nos dois sentidos (officials_manager.cpp) ----------

unsafe fn cqh7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqh7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A qualidade do árbitro na ordem: o byte +0x28 × 50 + a palavra +0x22, em 16 bits.
unsafe fn cqh7_qualidade(r: usize) -> i16 {
    (*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(cqh7_w(r + 0x22)) // cqh7: o peso da qualidade
}

/// O id da nação do árbitro ([+0x16] + 0).
unsafe fn cqh7_nacao(r: usize) -> i32 {
    cqh7_i(cqh7_i(r + 0x16) as u32 as usize)
}

/// { 0x79d4e0 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_desc(a, b)`, a ordem do
/// qsort da 0x79bf60 (os que ficam por nação): pela nação (id) e, na mesma, pela qualidade da maior para a menor; empate:
/// o índice +0 de `b` menos o de `a`.
unsafe extern "C" fn ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem decrescente");
    if ka == kb {
        return cqh7_i(b).wrapping_sub(cqh7_i(a)); // cqh7: o empate pelo índice
    }
    kb as i32 - ka as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1640)(a, b);
    let n = ordem_dos_arbitros_decrescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_decrescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x79d480 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_asc(a, b)`, a ordem do
/// qsort da 0x79c5c0 (depois dos árbitros novos): pela nação e pela qualidade da menor para a maior; empate: o índice +0
/// de `a` menos o de `b`.
unsafe extern "C" fn ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem crescente");
    if ka == kb {
        return cqh7_i(a).wrapping_sub(cqh7_i(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1641)(a, b);
    let n = ordem_dos_arbitros_crescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_crescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 261a leva: o árbitro de cada jogo: a escolha do árbitro, as marcas de ocupado do dia e a renovação dos árbitros da liga (officials_manager.cpp) ----------

unsafe fn cpy8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O bloco de árbitros da nação `nacao` no gerenciador (0x102 bytes em [this+0xc]: +4 conta, +8 primeiro índice, +0x10
/// palavra com o número de faixas de liga, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição).
unsafe fn cpy8_bloco(this: usize, nacao: i32) -> usize {
    cpy8_d(this + 0xc).wrapping_add(nacao.wrapping_mul(0x102) as usize)
}

/// O id da nação do clube (+0x53, ponteiro) ou, sem nação, `sem`.
unsafe fn cpy8_nacao_do_clube(clube: usize) -> Option<i32> {
    let n = cpy8_d(clube + 0x53);
    if n != 0 { Some(cpy8_i(n)) } else { None }
}

/// { 0x79c7b0 } (thiscall, ret 8, officials_manager.cpp): `OfficialsManager::choose_match_official(jogo, saida)`, pela
/// montagem do dia (0x6b3c10, saída em +0x2e do jogo). Jogo tirado (−2): 0 (eax 0xffffff00, como a original). A faixa
/// de árbitros: sem competição, a nação do mandante (+0x1c, +0x53) ou uma sorteada; amistoso (0x5c6710 em [0xae2c74]),
/// a do mandante; competição sem objeto ([0xadadfc]), como sem competição; competição dos tipos 2 e 4 (+0x42), o quarto
/// de cima (ftol de 0,75 e 0,25 da conta) da primeira nação, a partir de uma sorteada, do continente (+0x59 da
/// competição, ou +0x71 da nação do mandante, do visitante, ou um sorteado) que não seja a de nenhum dos dois clubes;
/// nas outras, a faixa da competição no bloco da nação dela (+0x5d; ou do mandante, do visitante, ou sorteada), a última
/// que bater, ou o bloco inteiro. Conta 1: o primeiro; senão primeiro + sorteio(conta − 1). Ocupado (o byte do id do
/// árbitro em [this+8]): o seguinte da faixa, dando a volta; todos ocupados, o sorteado. Grava o índice (palavra) e
/// devolve 1 no al (o resto do eax com o índice, como a original).
unsafe extern "thiscall" fn escolher_arbitro_da_partida(this: usize, jogo: usize, saida: usize) -> u32 {
    if cpy8_i(jogo) == -2 {
        return 0xffff_ff00;
    }
    let nacoes = cpy8_i(0xae2350);
    let mandante = cpy8_d(jogo + 0x1c);
    let visitante = cpy8_d(jogo + 0x20);
    let comp = cpy8_d(jogo + 0x14);
    let faixa_da_nacao = |nacao: i32| {
        let b = cpy8_bloco(this, nacao);
        (cpy8_i(b + 8), cpy8_i(b + 4))
    };
    let (ini, n) = if comp == 0 {
        faixa_da_nacao(cpy8_nacao_do_clube(mandante).unwrap_or_else(|| sortear(nacoes)))
    } else {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistoso(cpy8_d(0xae2c74), comp) != 0 {
            faixa_da_nacao(cpy8_i(cpy8_d(mandante + 0x53))) // como a original, sem conferir a nação
        } else {
            let c = cpy8_d(cpy8_d(0xadadfc).wrapping_add(cpy8_i(comp).wrapping_mul(4) as usize));
            if c == 0 {
                faixa_da_nacao(cpy8_nacao_do_clube(mandante).unwrap_or_else(|| sortear(nacoes)))
            } else {
                let tipo = cpy8_b(c + 0x42) as i8;
                if tipo == 2 || tipo == 4 {
                    let continente_do = |clube: usize| {
                        let n = cpy8_d(clube + 0x53);
                        if n != 0 && cpy8_d(n + 0x71) != 0 { Some(cpy8_i(cpy8_d(n + 0x71))) } else { None }
                    };
                    let continente = if cpy8_d(comp + 0x59) != 0 {
                        cpy8_i(cpy8_d(comp + 0x59))
                    } else if let Some(k) = continente_do(mandante) {
                        k
                    } else if let Some(k) = continente_do(visitante) {
                        k
                    } else {
                        sortear(cpy8_i(0xae234c))
                    };
                    let inicio = sortear(nacoes);
                    let mut k = inicio;
                    loop {
                        let nac = cpy8_d(0xae23a8).wrapping_add(k.wrapping_mul(0x122) as usize);
                        let cont = cpy8_d(nac + 0x71);
                        if cont != 0 && cpy8_i(cont) == continente
                            && cpy8_nacao_do_clube(mandante) != Some(cpy8_i(nac))
                            && cpy8_nacao_do_clube(visitante) != Some(cpy8_i(nac))
                        {
                            break;
                        }
                        k += 1;
                        if k >= cpy8_i(0xae2350) {
                            k = 0;
                        }
                        if k == inicio {
                            break;
                        }
                    }
                    let b = cpy8_bloco(this, k);
                    let conta = cpy8_i(b + 4) as f64;
                    crate::comparacao::ramo("cpy_8 arbitro neutro");
                    (cpy8_i(b + 8).wrapping_sub(ftol(conta * -0.75)), ftol(conta * 0.25)) // cpy8: o quarto de cima
                } else {
                    let nacao = if cpy8_d(comp + 0x5d) != 0 {
                        cpy8_i(cpy8_d(comp + 0x5d))
                    } else if let Some(k) = cpy8_nacao_do_clube(mandante) {
                        k
                    } else if let Some(k) = cpy8_nacao_do_clube(visitante) {
                        k
                    } else {
                        sortear(nacoes)
                    };
                    let b = cpy8_bloco(this, nacao);
                    let id = cpy8_i(comp);
                    let mut achou = None;
                    for f in 0..cpy8_w(b + 0x10).max(0) as usize {
                        let e = b + 0x12 + f * 0x10;
                        if cpy8_i(e + 0xc) == id {
                            achou = Some((cpy8_i(e), cpy8_i(e + 4)));
                        }
                    }
                    match achou {
                        Some(faixa) => {
                            crate::comparacao::ramo("cpy_8 faixa da liga");
                            faixa // cpy8: a faixa da liga
                        }
                        None => (cpy8_i(b + 8), cpy8_i(b + 4)),
                    }
                }
            }
        }
    };
    let sorteado = if n == 1 {
        ini
    } else {
        crate::comparacao::ramo("cpy_8 arbitro sorteado");
        ini.wrapping_add(sortear(n.wrapping_sub(1))) // cpy8: o sorteio na faixa
    };
    let ocupado = |k: i32| {
        let ficha = cpy8_d(0xae23e8).wrapping_add(k.wrapping_mul(0x2b) as usize);
        cpy8_b(cpy8_d(this + 8).wrapping_add(cpy8_d(ficha))) != 0
    };
    let mut k = sorteado;
    if ocupado(k) {
        let fim = n.wrapping_add(ini);
        loop {
            k += 1;
            if k >= fim {
                k = ini;
            }
            if k == sorteado || !ocupado(k) {
                break;
            }
        }
    }
    core::ptr::write_unaligned(saida as *mut i16, k as i16);
    (k as u32 & 0xffff_ff00) | 1
}

/// Comparador: lado a lado pela palavra de saída e pelo estado do sorteio, e pela devolução.
unsafe extern "thiscall" fn comparar_escolher_arbitro_da_partida(this: usize, jogo: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1362);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 2));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida),
        || crate::aleatorio::sem_contar(|| escolher_arbitro_da_partida(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("escolher_arbitro_da_partida(jogo {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", jogo, r, mo, n, mr));
    r
}

/// { 0x79bf40 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::clear_busy_flags`, no laço do dia
/// (0x5cccd0) e na corrida até a data de início (0x5cc850): zera os [this+4] bytes de ocupado em [this+8] (a conta e o
/// vetor relidos a cada passo). Devolve a conta (0 se não positiva), como o eax da original.
unsafe extern "thiscall" fn limpar_arbitros_ocupados(this: usize) -> u32 {
    let mut i = 0i32;
    while i < cpy8_i(this + 4) {
        *(cpy8_d(this + 8).wrapping_add(i as usize) as *mut u8) = 0;
        i += 1;
    }
    i as u32
}

/// Comparador: lado a lado pelos bytes de ocupado e pela devolução.
unsafe extern "thiscall" fn comparar_limpar_arbitros_ocupados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1363);
    let n = cpy8_i(this + 4);
    let reg = if n > 0 && cpy8_d(this + 8) != 0 { vec![(cpy8_d(this + 8), n as usize)] } else { Vec::new() };
    let (r, m, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || limpar_arbitros_ocupados(this));
    PLACAR.registrar(igual && r == m, || format!("limpar_arbitros_ocupados(conta {}) original={} rust={} memoria igual {}", n, r, m, igual));
    r
}

/// { 0x79cee0 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::renew_officials(comp)`, no começo da
/// temporada de cada liga (83 chamadores). Sem competição, sem objeto ([0xadadfc]), dos tipos 4 e 2 (+0x42) ou sem
/// conta (0x4b0700): 0. A faixa da competição no bloco da nação dela (+0x5d, ou uma sorteada), a primeira que bater,
/// ou o bloco inteiro. Cada árbitro da faixa (fichas de 0x2b bytes em [0xae23e8]; ficha nula: a caixa "Error"): com
/// mais de 45 anos (o ano em byte, menos 1 antes do dia dele), ou mais de 40 e sorteio(3) zero, vira outro: dois nomes
/// da nação dele (0x539c60, +4 e +8), +0x25 sorteio(16) + 5 (ou, com sorteio(10) zero, sorteio(11) + sorteio(11)), o
/// ano (+0xe) ano do jogo − 27 + sorteio(5) e o dia (+0xc) sorteio(340). Depois zera as linhas de estatísticas (17
/// bytes em [this]) de cada árbitro da faixa (+4, +8, +0xa, +0xc, +0xf; o `add eax, 0x11` em 0x79d149 fica dentro do
/// laço). Devolve 1 (só o al vale).
unsafe extern "thiscall" fn renovar_arbitros(this: usize, comp: usize) -> u8 {
    if comp == 0 {
        return 0;
    }
    let c = cpy8_d(cpy8_d(0xadadfc).wrapping_add(cpy8_i(comp).wrapping_mul(4) as usize));
    if c == 0 || cpy8_b(c + 0x42) == 4 || cpy8_b(c + 0x42) == 2 {
        return 0;
    }
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(cpy8_i(comp)) == 0 {
        return 0;
    }
    let nacao = if cpy8_d(comp + 0x5d) != 0 { cpy8_i(cpy8_d(comp + 0x5d)) } else { sortear(cpy8_i(0xae2350)) };
    let b = cpy8_bloco(this, nacao);
    let mut faixa = None;
    for f in 0..cpy8_w(b + 0x10).max(0) as usize {
        let e = b + 0x12 + f * 0x10;
        if cpy8_i(e + 0xc) == cpy8_i(comp) {
            faixa = Some((cpy8_i(e), cpy8_i(e + 4)));
            break;
        }
    }
    let (ini, n) = faixa.unwrap_or((cpy8_i(b + 8), cpy8_i(b + 4)));
    let nome: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    for k in 0..n.max(0) {
        let ficha = cpy8_d(0xae23e8).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x2b) as usize);
        if ficha == 0 {
            aviso_em(0xa6a22c, 0xa61);
            continue;
        }
        let ano = cpy8_b(0xae2c92);
        let dif = (cpy8_w(ficha + 0xc) as i32).wrapping_sub(cpy8_w(0xae2c90) as i32);
        let idade = if dif > 0 { ano.wrapping_sub(cpy8_b(ficha + 0xe)).wrapping_sub(1) } else { ano.wrapping_sub(cpy8_b(ficha + 0xe)) } as i8;
        if idade <= 0x28 || (idade <= 0x2d && sortear(3) != 0) {
            continue;
        }
        crate::comparacao::ramo("cpy_8 arbitro renovado");
        let nac = cpy8_d(ficha + 0x16);
        let n1 = nome(nac);
        core::ptr::write_unaligned((ficha + 4) as *mut usize, n1);
        let n2 = nome(cpy8_d(ficha + 0x16));
        core::ptr::write_unaligned((ficha + 8) as *mut usize, n2);
        *((ficha + 0x25) as *mut u8) = if sortear(10) != 0 {
            (sortear(0x10) as u8).wrapping_add(5)
        } else {
            let a = sortear(0xb) as u8;
            a.wrapping_add(sortear(0xb) as u8)
        };
        let r5 = sortear(5);
        let base = (cpy8_w(0xae2c92) as u16).wrapping_sub(0x1b);
        core::ptr::write_unaligned((ficha + 0xe) as *mut u16, (r5 as u16).wrapping_add(base));
        core::ptr::write_unaligned((ficha + 0xc) as *mut u16, sortear(0x154) as u16);
    }
    for k in 0..n.max(0) {
        let linha = cpy8_d(this).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x11) as usize);
        *((linha + 0xf) as *mut u8) = 0;
        core::ptr::write_unaligned((linha + 8) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xa) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 4) as *mut u32, 0);
    }
    1
}

/// Comparador: lado a lado pelas fichas de todos os árbitros ([0xae2390] × 0x2b), pelas linhas de estatísticas
/// ([0xae2390] × 17), pelo estado do sorteio e por [0xb67a34], e pela devolução. Sem tabela de árbitros (a caixa a cada
/// ficha): só a original.
unsafe extern "thiscall" fn comparar_renovar_arbitros(this: usize, comp: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1364);
    let total = cpy8_i(0xae2390).max(0) as usize;
    if cpy8_d(0xae23e8) == 0 || cpy8_d(this) == 0 {
        return o(this, comp);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(cpy8_d(0xae23e8), total * 0x2b), (cpy8_d(this), total * 0x11), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, comp),
        || crate::aleatorio::sem_contar(|| renovar_arbitros(this, comp)));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("renovar_arbitros(comp {:#x}) original={} rust={}, diferencas em {:x?}", comp, r, n, dif)
    });
    r
}

// ---------- 261a leva: os árbitros do jogo novo: os árbitros sorteados de uma nação, o conserto das fichas dos árbitros e a cópia de uma ficha (officials_manager.cpp) ----------

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
unsafe fn cpy9_s(n: i32, k: u8) -> u8 {
    (sortear(n) as u8).wrapping_add(k)
}

/// Uma ficha de árbitro nova em `r` (0x2b bytes) da nação `nacao`, na ordem da 0x79c6aa: primeiro e segundo nome pelo
/// peso (0x539b90, 0x53a0e0), a nação (+0x16), as palavras +0x22 sorteio(80) + 1, +0x1e e +0x20 sorteio(80) + 40 (o
/// byte estendido), os bytes +0x24..+0x26 sorteio(10) + 5 e +0x27..+0x2a sorteio(10) + 7, +0x1a zero, o ano (+0xe)
/// 1960 + sorteio(10) e o dia (+0xc) sorteio(300) + 1. Não grava +0, +0x10..+0x15 (a original deixa o que o realloc
/// trouxe).
unsafe fn cpy9_gerar(r: usize, nacao: usize) {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539b90usize);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a0e0usize);
    cpy9_pd(r + 4, primeiro(nacao));
    cpy9_pd(r + 8, segundo(nacao));
    cpy9_pd(r + 0x16, nacao);
    cpy9_pw(r + 0x22, cpy9_s(0x50, 1) as i8 as i16); // cpy9: a nota +0x22
    cpy9_pw(r + 0x1e, cpy9_s(0x50, 0x28) as i8 as i16);
    cpy9_pw(r + 0x20, cpy9_s(0x50, 0x28) as i8 as i16);
    for o in [0x24usize, 0x25, 0x26] {
        cpy9_pb(r + o, cpy9_s(10, 5));
    }
    cpy9_pb(r + 0x27, cpy9_s(10, 7));
    cpy9_pb(r + 0x28, cpy9_s(10, 7)); // cpy9: o atributo +0x28
    cpy9_pb(r + 0x29, cpy9_s(10, 7));
    cpy9_pb(r + 0x2a, cpy9_s(10, 7));
    cpy9_pd(r + 0x1a, 0);
    cpy9_pw(r + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
    cpy9_pw(r + 0xc, sortear(0x12c).wrapping_add(1) as i16);
}

/// { 0x79c5c0 } (thiscall com o this sem uso, ret 8, officials_manager.cpp): `officials_manager_add_random_officials(
/// conta, nacao)`, pela montagem dos árbitros do jogo novo (0x79bf60, nas nações com poucos árbitros). Nação sem
/// continente (+0x71): nada. Realoca a tabela ([0xae23e8], 0x945667) para [0xae2390] + conta fichas (sem memória: erro
/// e o jogo para), gera `conta` fichas novas no fim (cpy9_gerar), soma a conta em [0xae2390] e ordena a tabela inteira
/// (qsort 0x945333 com 0x79d480: nação, +0x28 × 50 + +0x22, id).
unsafe extern "thiscall" fn arbitros_sorteados(_this: usize, conta: i32, nacao: usize) {
    if nacao != 0 && cpy9_d(nacao + 0x71) == 0 {
        return;
    }
    let realocar: unsafe extern "C" fn(usize, i32) -> usize = core::mem::transmute(0x945667usize);
    let antes = cpy9_i(0xae2390);
    let t = realocar(cpy9_d(0xae23e8), antes.wrapping_add(conta).wrapping_mul(0x2b));
    *(0xae23e8 as *mut usize) = t;
    if t == 0 {
        erro_em(0xa6a22c, 0x342);
    }
    let mut r = t.wrapping_add(antes.wrapping_mul(0x2b) as usize);
    for _ in 0..conta.max(0) {
        crate::comparacao::ramo("cpy_9 arbitro novo");
        cpy9_gerar(r, nacao);
        r += 0x2b;
    }
    let n = cpy9_i(0xae2390).wrapping_add(conta);
    *(0xae2390 as *mut i32) = n;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparacao::ramo("cpy_9 ordenar");
    ordenar(cpy9_d(0xae23e8), n, 0x2b, 0x79d480); // cpy9: ordenar a tabela
}

/// A ficha `g` (gerada no ensaio) bate com a `f` da tabela nos campos que a 0x79c5c0 grava (+4..+0x10, +0x16..+0x2b)?
unsafe fn cpy9_mesma_ficha(g: &[u8; 0x2b], f: usize) -> bool {
    let t = core::slice::from_raw_parts(f as *const u8, 0x2b);
    g[4..0x10] == t[4..0x10] && g[0x16..] == t[0x16..]
}

/// Comparador: realoca e ordena (não pode rodar duas vezes): só a original, depois de um ensaio. O ensaio gera as
/// `conta` fichas da cópia (cpy9_gerar, sem contar os sorteios) numa área própria a partir do mesmo estado do sorteio,
/// que é reposto; depois roda a original. Confere o estado final do sorteio, a conta nova ([0xae2390] + conta) e que
/// cada ficha do ensaio esteja na tabela ordenada (campos que a original grava). Conta não positiva ou nação sem
/// continente: só a original.
unsafe extern "thiscall" fn comparar_arbitros_sorteados(this: usize, conta: i32, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize)>(1365);
    if conta <= 0 || (nacao != 0 && cpy9_d(nacao + 0x71) == 0) {
        return o(this, conta, nacao);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let mut novas = vec![[0u8; 0x2b]; conta as usize];
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        for g in novas.iter_mut() {
            cpy9_gerar(g.as_mut_ptr() as usize, nacao);
        }
    }));
    let estado_rust = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    let antes = cpy9_i(0xae2390);
    o(this, conta, nacao);
    let estado_orig = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let total = cpy9_i(0xae2390);
    let t = cpy9_d(0xae23e8);
    let mut usada = vec![false; total.max(0) as usize];
    let mut faltam = Vec::new();
    for (k, g) in novas.iter().enumerate() {
        match (0..usada.len()).find(|&j| !usada[j] && cpy9_mesma_ficha(g, t + j * 0x2b)) {
            Some(j) => usada[j] = true,
            None => faltam.push(k),
        }
    }
    let igual = estado_orig == estado_rust && total == antes.wrapping_add(conta) && faltam.is_empty();
    PLACAR.registrar(igual, || format!("arbitros_sorteados(conta {}, nacao {:#x}) total {} (antes {}), sorteio igual {}, fichas da copia ausentes {:?}",
        conta, nacao, total, antes, estado_orig == estado_rust, faltam));
}

/// O ajuste de um atributo em byte (+0x24..+0x2a) fora de 1..20: o valor sorteado por `f` (lê a ficha antes).
unsafe fn cpy9_atributo(e: usize, o: usize, f: impl FnOnce() -> u8) {
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
unsafe extern "C" fn validar_arbitros() {
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
                crate::comparacao::ramo("cpy_9 nota consertada");
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
        let faixa = |alto: (i32, u8), medio: (i32, u8), baixo: (i32, u8)| {
            let w = cpy9_w(e + 0x1e);
            let (n, k) = if w > 0x78 { alto } else if w > 0x50 { medio } else { baixo };
            cpy9_s(n, k)
        };
        cpy9_atributo(e, 0x24, || if cpy9_w(e + 0x1e) > 0x64 { cpy9_s(0xc, 7) } else { cpy9_s(0xa, 5) });
        cpy9_atributo(e, 0x25, || faixa((0xe, 7), (0xf, 5), (0xa, 5)));
        cpy9_atributo(e, 0x26, || faixa((0xb, 0xa), (0xf, 5), (0xa, 5)));
        cpy9_atributo(e, 0x27, || if cpy9_w(e + 0x1e) > 0x50 { cpy9_s(0xf, 6) } else { cpy9_s(0x14, 1) });
        cpy9_atributo(e, 0x28, || faixa((0xa, 0xb), (0xf, 5), (0xa, 7)));
        cpy9_atributo(e, 0x29, || faixa((0xd, 8), (0xe, 7), (0xf, 6)));
        cpy9_atributo(e, 0x2a, || if sortear(0x64) != 0 { cpy9_s(0xa, 0xb) } else { cpy9_s(0xa, 5) });
        let mut limite = [0u8; 8];
        let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> *const u8 = core::mem::transmute(0x549f70usize);
        let d = data(limite.as_mut_ptr(), 1, 1, 0x776, -1);
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
unsafe extern "C" fn comparar_validar_arbitros() {
    let o = orig::<unsafe extern "C" fn()>(1366);
    let total = cpy9_i(0xae2390).max(0) as usize;
    if cpy9_d(0xae23e8) == 0 {
        return o();
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((cpy9_d(0xae23e8), total * 0x2b));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| validar_arbitros()));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("validar_arbitros ({} fichas) diferencas em {:x?}", total, dif)
    });
}

/// { 0x79c530 } (thiscall, ret 4, officials_manager.cpp): `Official::operator=(src)`, pela montagem dos árbitros do
/// jogo novo (0x79bf60, as fichas que ficam). Copia a ficha de 0x2b bytes campo a campo (+0, +4, +8; a data de +0xc
/// pela 0x417c10; +0x14, +0x16, +0x1a, +0x1e, +0x20, +0x22, os bytes +0x24..+0x2a). Devolve `this`.
unsafe extern "thiscall" fn copiar_ficha_de_arbitro(this: usize, src: usize) -> usize {
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
unsafe extern "thiscall" fn comparar_copiar_ficha_de_arbitro(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1367);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x2b)], || o(this, src), || copiar_ficha_de_arbitro(this, src));
    PLACAR.registrar(igual && r == n, || format!("copiar_ficha_de_arbitro(src {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", src, r, mo, n, mr));
    r
}

