// ---------- %%LEVA%%a leva: os árbitros do jogo novo: a revisão dos registros, os que ficam por nação, as vagas das competições e os árbitros novos sorteados (officials_manager.cpp) ----------

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

/// Um valor de 1..200 sorteado de novo (palavra): com nação, sorteio(80) + `extra` + 1 em 8 bits com sinal; sem,
/// sorteio(80) + 1.
unsafe fn cqg9_valor(a: usize, nacao: usize, mult: u8) {
    let v = cqg9_w(a);
    if v > 0 && v <= 0xc8 {
        return;
    }
    let r = cqg9_sortear(0x50) as u8;
    let b = if nacao != 0 { r.wrapping_add((*((nacao + 0x8e) as *const u8)).wrapping_mul(mult)).wrapping_add(1) } else { r.wrapping_add(1) };
    cqg9_pw(a, b as i8 as i16);
}

/// Um atributo de 1..20 sorteado de novo pela faixa de +0x1e (acima de 120, acima de 80, o resto): sorteio(n) + soma.
unsafe fn cqg9_atributo(a: usize, base: i16, faixas: [(i32, i8); 3]) {
    let v = *(a as *const i8);
    if v > 0 && v <= 0x14 {
        return;
    }
    let (n, s) = if base > 0x78 { faixas[0] } else if base > 0x50 { faixas[1] } else { faixas[2] };
    *(a as *mut i8) = (cqg9_sortear(n) as i8).wrapping_add(s);
}

/// { 0x79caa0 } (cdecl, officials_manager.cpp): `officials_manager_validate_officials()`: em cada árbitro, sem nação
/// (+0x16) a do primeiro nome (+4, +0x37), senão a do segundo (+8), senão uma sorteada; as notas +0x22, +0x1e, +0x20
/// fora de 1..200 sorteadas (sorteio(80) + 1, mais o +0x8e da nação; vezes 4 nas duas últimas) e +0x20 pelo menos +0x1e;
/// os atributos +0x24..+0x2a fora de 1..20 sorteados pela faixa de +0x1e; nascido antes de 1/1/1910 (0x549f70), um
/// nascimento nos anos 60 (+0xe ano 1960..1969, +0xc dia 1..300).
unsafe extern "C" fn rever_os_arbitros() {
    let mut i = 0i32;
    while i < cqg9_i(0xae2390) {
        let r = cqg9_arbitro(i);
        if cqg9_d(r + 0x16) == 0 {
            let n1 = cqg9_d(r + 4);
            let id = if cqg9_i(n1 + 0x37) >= 0 {
                cqg9_i(n1 + 0x37)
            } else if cqg9_i(cqg9_d(r + 8) + 0x37) >= 0 {
                cqg9_i(cqg9_d(r + 8) + 0x37)
            } else {
                cqg9_sortear(cqg9_i(0xae2350))
            };
            cqg9_pd(r + 0x16, cqg9_d(0xae23a8).wrapping_add((id as isize * 0x122) as usize) as u32);
        }
        cqg9_valor(r + 0x22, cqg9_d(r + 0x16), 1);
        cqg9_valor(r + 0x1e, cqg9_d(r + 0x16), 4);
        let w20 = cqg9_w(r + 0x20);
        if !(w20 > 0 && w20 <= 0xc8) {
            cqg9_valor(r + 0x20, cqg9_d(r + 0x16), 4);
            if cqg9_w(r + 0x20) < cqg9_w(r + 0x1e) {
                cqg9_pw(r + 0x20, cqg9_w(r + 0x1e));
            }
        }
        let b = cqg9_w(r + 0x1e);
        let v = *((r + 0x24) as *const i8);
        if !(v > 0 && v <= 0x14) {
            let x = if b > 0x64 { (cqg9_sortear(0xc) as i8).wrapping_add(7) } else { (cqg9_sortear(0xa) as i8).wrapping_add(5) };
            *((r + 0x24) as *mut i8) = x;
        }
        cqg9_atributo(r + 0x25, cqg9_w(r + 0x1e), [(0xe, 7), (0xf, 5), (0xa, 5)]);
        cqg9_atributo(r + 0x26, cqg9_w(r + 0x1e), [(0xb, 0xa), (0xf, 5), (0xa, 5)]);
        let v = *((r + 0x27) as *const i8);
        if !(v > 0 && v <= 0x14) {
            let x = if cqg9_w(r + 0x1e) > 0x50 { (cqg9_sortear(0xf) as i8).wrapping_add(6) } else { (cqg9_sortear(0x14) as i8).wrapping_add(1) };
            *((r + 0x27) as *mut i8) = x;
        }
        cqg9_atributo(r + 0x28, cqg9_w(r + 0x1e), [(0xa, 0xb), (0xf, 5), (0xa, 7)]);
        cqg9_atributo(r + 0x29, cqg9_w(r + 0x1e), [(0xd, 8), (0xe, 7), (0xf, 6)]);
        let v = *((r + 0x2a) as *const i8);
        if !(v > 0 && v <= 0x14) {
            let x = if cqg9_sortear(0x64) != 0 { (cqg9_sortear(0xa) as i8).wrapping_add(0xb) } else { (cqg9_sortear(0xa) as i8).wrapping_add(5) };
            *((r + 0x2a) as *mut i8) = x;
        }
        let mut d = [0u8; 8];
        let data: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
        let p = data(d.as_mut_ptr() as usize, 1, 1, 0x776, 0xffff_ffff);
        let (dia, ano) = (cqg9_w(p), cqg9_w(p + 2) as u16);
        let ano_r = cqg9_w(r + 0xe) as u16;
        if ano > ano_r || (ano == ano_r && dia > cqg9_w(r + 0xc)) {
            let a = cqg9_sortear(0xa);
            cqg9_pw(r + 0xe, a.wrapping_add(0x7a8) as i16);
            let dd = cqg9_sortear(0x12c);
            cqg9_pw(r + 0xc, dd.wrapping_add(1) as i16);
        }
        i += 1;
    }
}

/// Comparador: lado a lado nos registros dos árbitros e no estado do sorteio (os sorteios da cópia fora da contagem).
unsafe extern "C" fn comparar_rever_os_arbitros() {
    let o = orig::<unsafe extern "C" fn()>(%%I1%%);
    let n = cqg9_i(0xae2390);
    if n <= 0 || cqg9_d(0xae23e8) == 0 {
        return o();
    }
    let mut reg = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.push((cqg9_d(0xae23e8), n as usize * 0x2b));
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| rever_os_arbitros()));
    PLACAR.registrar(igual, || format!("rever_os_arbitros({} arbitros)", n));
}

/// { 0x79c5c0 } (stdcall, ret 8, officials_manager.cpp): `officials_manager_add_random_officials(conta, nação)`: nação
/// sem continente (+0x71): nada. Senão cresce a tabela (realloc 0x945667; sem memória, fim fatal 0x342) com `conta`
/// árbitros da nação: os nomes (0x539b90, 0x53a0e0), as notas +0x22 1..80 e +0x1e/+0x20 40..119, os atributos +0x24..
/// +0x26 5..14 e +0x27..+0x2a 7..16, +0x1a = 0 e o nascimento nos anos 60 (+0xe, +0xc); depois ordena (qsort 0x79d480).
/// Os bytes +0..+3 e +0x10..+0x15 dos novos ficam como o realloc deixou (como na original).
unsafe extern "stdcall" fn sortear_arbitros_novos(conta: i32, nacao: usize) {
    if nacao != 0 && cqg9_d(nacao + 0x71) == 0 {
        return;
    }
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let n = cqg9_i(0xae2390);
    let base = realloc(cqg9_d(0xae23e8), (n.wrapping_add(conta) as u32).wrapping_mul(0x2b));
    cqg9_pd(0xae23e8, base as u32);
    if base == 0 {
        cqg9_fatal(0x342);
    }
    let nome1: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539b90usize);
    let nome2: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a0e0usize);
    let mut r = base.wrapping_add((n as isize * 0x2b) as usize);
    for _ in 0..conta.max(0) {
        let a = nome1(nacao);
        cqg9_pd(r + 4, a as u32);
        let b = nome2(nacao);
        cqg9_pd(r + 8, b as u32);
        cqg9_pd(r + 0x16, nacao as u32);
        let x = cqg9_sortear(0x50) as i8;
        cqg9_pw(r + 0x22, x.wrapping_add(1) as i16); // cqg9: a nota +0x22 do árbitro novo
        crate::comparacao::ramo("cqg_9 arbitro novo");
        let x = cqg9_sortear(0x50) as i8;
        cqg9_pw(r + 0x1e, x.wrapping_add(0x28) as i16);
        let x = cqg9_sortear(0x50) as i8;
        cqg9_pw(r + 0x20, x.wrapping_add(0x28) as i16);
        for (k, s) in [(0x24usize, 5i8), (0x25, 5), (0x26, 5), (0x27, 7), (0x28, 7), (0x29, 7), (0x2a, 7)] {
            let x = cqg9_sortear(0xa) as i8;
            *((r + k) as *mut i8) = x.wrapping_add(s);
        }
        cqg9_pd(r + 0x1a, 0);
        let a = cqg9_sortear(0xa);
        cqg9_pw(r + 0xe, a.wrapping_add(0x7a8) as i16);
        let d = cqg9_sortear(0x12c);
        cqg9_pw(r + 0xc, d.wrapping_add(1) as i16);
        r += 0x2b;
    }
    let total = n.wrapping_add(conta);
    cqg9_pd(0xae2390, total as u32);
    cqg9_ordenar(base, total as u32, 0x2b, 0x79d480);
}

/// Comparador: só a original (ponytail: realloc e sorteio: não pode rodar duas vezes) com o plano: a conta cresce
/// `conta` e a tabela existe (a nação sem continente: nada muda).
unsafe extern "stdcall" fn comparar_sortear_arbitros_novos(conta: i32, nacao: usize) {
    let o = orig::<unsafe extern "stdcall" fn(i32, usize)>(%%I2%%);
    let n = cqg9_i(0xae2390);
    let nada = nacao != 0 && cqg9_d(nacao + 0x71) == 0;
    o(conta, nacao);
    let esperado = if nada { n } else { n.wrapping_add(conta) };
    PLACAR.registrar(cqg9_i(0xae2390) == esperado && cqg9_d(0xae23e8) != 0,
        || format!("sortear_arbitros_novos({}, nacao {:x}) conta {} -> {}", conta, nacao, n, cqg9_i(0xae2390)));
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
            cqg9_pd(b + 0x16 + 16 * j as usize, (cqg9_i(b + 4) / c) as u32);
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
    orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    PLACAR.registrar(cqg9_d(this + 0xc) != 0 && cqg9_i(this + 4) == cqg9_i(0xae2390) && cqg9_d(this + 8) != 0,
        || format!("montar_os_arbitros: blocos {:x}, conta {} x {}", cqg9_d(this + 0xc), cqg9_i(this + 4), cqg9_i(0xae2390)));
}
