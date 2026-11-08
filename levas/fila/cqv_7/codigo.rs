// ---------- %%LEVA%%a leva: a Copa da Inglaterra (FA Cup): os 152 inscritos (eng_fa_cup.cpp) ----------

unsafe fn cqv7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqv7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A divisão do clube (+0x57, ponteiro para o registro da competição; +0 o id) é a de [`g`]?
unsafe fn cqv7_na(c: usize, g: usize) -> bool {
    let l = cqv7_d(c + 0x57);
    l != 0 && cqv7_d(l) == cqv7_d(g)
}

/// A divisão do clube é uma das cinco da liga inglesa (a genérica [0x9cf764], a Premier [0x9cf5bc] e as três da
/// Football League [0x9cf5c0], [0x9cf5c4], [0x9cf5c8]) ou a Conference [0x9cf69c] [inferência pelos 20 + 24 × 3 + 22]?
unsafe fn cqv7_da_liga(id: usize) -> bool {
    [0x9cf764usize, 0x9cf5bc, 0x9cf5c0, 0x9cf5c4, 0x9cf5c8, 0x9cf69c].iter().any(|&g| id == cqv7_d(g))
}

/// As duas listas da original ficam lado a lado na pilha (a da Conference, 22 lugares, e logo depois a dos de fora da
/// liga, 300): a cópia usa um vetor só, com a segunda a partir de 22, para estourar como a original (nunca acontece).
const CQV7_FORA: i32 = 22;

/// Tira o clube `c` da lista que começa em `ini` (`n` usados): troca pelo último e desconta; devolve se achou.
fn cqv7_tirar(v: &mut [usize; 322], ini: i32, n: &mut i32, c: usize) -> bool {
    let mut achou = false;
    let mut j = 0i32;
    while (j as i16) < *n as i16 {
        if v[(ini + j) as usize] == c {
            *n -= 1;
            v[(ini + j) as usize] = v[(ini + *n as i16 as i32) as usize];
            achou = true;
        }
        j += 1;
    }
    achou
}

/// Grava o clube na vaga `k` da tabela (+0xa7, 6 bytes: o clube, +4 e +5 zerados) e conta.
unsafe fn cqv7_por(this: usize, k: &mut i32, c: usize) {
    let t = cqv7_d(this + 0xa7) + *k as usize * 6;
    cqv7_pd(t, c);
    *((t + 4) as *mut u8) = 0;
    *((t + 5) as *mut u8) = 0;
    *k += 1;
}

/// Os clubes da divisão [`g`] pela ordem da tabela dos clubes, fora `fora`, até a tabela encher (k == +0x3e).
unsafe fn cqv7_divisao(this: usize, k: &mut i32, g: usize, fora: &[usize]) {
    let base = cqv7_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < cqv7_d(0xae235c) as i32 {
        let c = base + i as u16 as usize * 0x245;
        if cqv7_na(c, g) && !fora.contains(&c) {
            cqv7_por(this, k, c);
            if *k == cqv7_w(this + 0x3e) as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// { 0x56fab0 } (thiscall, ret 0, eng_fa_cup.cpp; chamada pelo dia da copa, 0x570550, todo 3 de setembro; n = 1 até
/// 2002-06-01): `EngFaCup::collect_entrants()`: os 152 inscritos da Copa da Inglaterra.
/// 1) Os ingleses (+0x53 = [0x9cf2e4]) da competição [0x9cf7bc] do mesmo ano (+0x40), procurada entre as [0xb63dfc]
///    competições de [0xb63e00] pelo id do registro (+4): a lista dela pelo método +0x58 (push 0; push &lista; mov ecx ->
///    thiscall, ret 8; devolve quantos; a lista é liberada com 0x9452ca), até 2 (o Mundial de Clubes [inferência]: a
///    dispensa do Manchester United em 1999/2000). Em 1999, com 1 achado e +0xb2 ≠ 2, +0xb4 = 1 e +0xb3 = 0 (o clube sai
///    da copa); senão +0xb4 = 0 e +0xb3 = quantos.
/// 2) +0x3e = 152 e a tabela (malloc 0x944e46 de 0x390, sem zerar) em +0xa7 (sem memória: a caixa da linha 0x1d2 e −1).
/// 3) Com a regra da nação ([+4] +0x5d, bit 4 de +0x11c) ou em 2001 (+0x40 = 0x7d1): o campeão e o vice da última
///    temporada da competição [0x9cf6a0] (comp_util_get_latest_placed_club 0x4afd80: push lugar; push registro; add esp,
///    0x10 nas duas -> cdecl; registro = [0xae23d0] + 0x6b × [0x9cf6a0]; o FA Trophy [inferência]) ganham vaga; senão só
///    o primeiro clube da tabela, se for da Conference (a original testa sempre o mesmo clube, sem avançar no laço).
/// 4) Pela ordem da tabela dos clubes: os da Conference ([0x9cf69c]) numa lista; os ingleses (+0x53 = o registro
///    [0xae23a8] + 0x122 × [0x9cf2e4]) fora das divisões da liga, até 300, noutra (os de fora da liga).
/// 5) Os dois premiados saem da lista deles: da Conference, com 21 vagas no lugar de 22 para cada um; de fora da liga,
///    com 37 no lugar de 38 (não achado, o primeiro da lista é trocado pelo último).
/// 6) Dois da Conference sorteados (0x90cfc0(n − 2), cdecl, duas vezes) vão para as duas últimas posições; os de fora
///    da liga ordenados pela reputação com os melhores embaralhados (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push
///    vagas; push n; push lista; add esp, 0x14 com os dois sorteios -> cdecl).
/// 7) A tabela: as vagas de fora da liga (38), as da Conference (22), os dois premiados, a terceira divisão
///    ([0x9cf5c8]), a segunda ([0x9cf5c4]) e a primeira ([0x9cf5c0]) da Football League (fora os do passo 1 e, na
///    terceira, os premiados), cada uma até a tabela encher; as 24 últimas (a primeira divisão) embaralhadas por 60
///    trocas de duas posições sorteadas (0x90cfc0(0x18) duas vezes); a Premier ([0x9cf5bc]); por fim os do passo 1. Com
///    +0xb4 = 1, +0x3e desconta um. Devolve 1 se a tabela encheu (k == +0x3e), senão 0.
unsafe fn cqv7_inscritos(this: usize) -> i32 {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // 1) os ingleses da competição [0x9cf7bc] do mesmo ano
    let mut fora_do_ano = [0usize; 2];
    let mut nf: i16 = 0;
    let n_comp = cqv7_d(0xb63dfc) as i32;
    if n_comp > 0 {
        let v = cqv7_d(0xb63e00);
        let mut i: i16 = 0;
        let mut achada = 0usize;
        loop {
            let c = cqv7_d(v + i as i32 as usize * 4);
            if cqv7_d(cqv7_d(c + 4)) == cqv7_d(0x9cf7bc) {
                achada = c;
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n_comp {
                break;
            }
        }
        if achada != 0 && cqv7_w(achada + 0x40) == cqv7_w(this + 0x40) {
            let lista_da: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> i32 = core::mem::transmute(cqv7_d(cqv7_d(achada) + 0x58));
            let mut lista = 0usize;
            let n = lista_da(achada, &mut lista, 0) as i16;
            let mut j: i16 = 0;
            while j < n && nf < 2 {
                let c = cqv7_d(lista + j as i32 as usize * 4);
                let nac = cqv7_d(c + 0x53);
                if nac != 0 && cqv7_d(nac) == cqv7_d(0x9cf2e4) {
                    fora_do_ano[nf as usize] = c;
                    nf += 1;
                }
                j += 1;
            }
            if lista != 0 {
                soltar(lista);
            }
        }
    }
    if cqv7_w(this + 0x40) == 0x7cf && nf == 1 && *((this + 0xb2) as *const u8) != 2 {
        *((this + 0xb4) as *mut u8) = nf as u8;
        *((this + 0xb3) as *mut u8) = 0;
    } else {
        *((this + 0xb4) as *mut u8) = 0;
        *((this + 0xb3) as *mut u8) = nf as u8;
    }
    // 2) a tabela
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x98);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x390);
    cqv7_pd(this + 0xa7, t);
    if t == 0 {
        show_error_at(0x9c42cc, 0x1d2);
        return -1;
    }
    // 3) os premiados
    let base = cqv7_d(0xae23b4);
    let mut a = 0usize;
    let mut b = 0usize;
    let nacao = cqv7_d(cqv7_d(this + 4) + 0x5d);
    if *((nacao + 0x11c) as *const u8) & 4 != 0 || cqv7_w(this + 0x40) == 0x7d1 {
        crate::comparison::branch("cqv_7 campeao e vice do trofeu");
        let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
        let reg = cqv7_d(0xae23d0) + cqv7_d(0x9cf6a0).wrapping_mul(0x6b);
        a = colocado(reg, 0); // cqv7: o campeão
        let reg = cqv7_d(0xae23d0) + cqv7_d(0x9cf6a0).wrapping_mul(0x6b);
        b = colocado(reg, 1);
    } else if cqv7_d(0xae235c) as i32 > 0 && cqv7_na(base, 0x9cf69c) {
        // ponytail: a original repete o teste do primeiro clube sem avançar no laço; um teste basta
        a = base;
    }
    // 4) a Conference e os de fora da liga
    // ponytail: a original não zera as listas (sobra da pilha); só lê além do que preencheu se faltarem clubes (menos de
    // 38 de fora da liga), o que a carreira não tem; a cópia usa 0, que a original também pula
    let mut v = [0usize; 322];
    let mut nconf = 0i32;
    let mut nout = 0i32;
    let inglaterra = cqv7_d(0xae23a8) + cqv7_d(0x9cf2e4).wrapping_mul(0x122);
    let mut i: i16 = 0;
    while (i as i32) < cqv7_d(0xae235c) as i32 {
        let c = base + i as u16 as usize * 0x245;
        let l = cqv7_d(c + 0x57);
        if l != 0 {
            let id = cqv7_d(l);
            if id == cqv7_d(0x9cf69c) {
                // ponytail: a original não confere o tamanho da lista da Conference (22 lugares; a Conference tem 22)
                v[nconf as i16 as usize] = c;
                nconf += 1;
            } else if !cqv7_da_liga(id) && cqv7_d(c + 0x53) == inglaterra && (nout as i16) < 0x12c {
                v[(CQV7_FORA + nout as i16 as i32) as usize] = c;
                nout += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    // 5) os premiados saem da lista deles
    let mut vagas_conf: i8 = 0x16;
    let mut vagas_fora: i8 = 0x26;
    for (k, &p) in [a, b].iter().enumerate() {
        if p == 0 || cqv7_d(p + 0x57) == 0 {
            continue;
        }
        let id = cqv7_d(cqv7_d(p + 0x57));
        if id == cqv7_d(0x9cf69c) {
            cqv7_tirar(&mut v, 0, &mut nconf, p);
            if k == 0 {
                vagas_conf = 0x15;
            } else {
                vagas_conf -= 1;
            }
        } else if !cqv7_da_liga(id) {
            if !cqv7_tirar(&mut v, CQV7_FORA, &mut nout, p) {
                nout -= 1;
                v[CQV7_FORA as usize + k] = v[(CQV7_FORA + nout as i16 as i32) as usize];
            }
            if k == 0 {
                vagas_fora = 0x25;
            } else {
                vagas_fora -= 1;
            }
        }
    }
    // 6) dois da Conference sorteados para o fim; os de fora da liga pela reputação
    let n = nconf as i16 as i32;
    let r1 = sorteio(n - 2) as i8 as i32;
    let r2 = sorteio(n - 2) as i8 as i32;
    v.swap(r1 as usize, (n - 1) as usize);
    v.swap(r2 as usize, (n - 2) as usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    // ponytail: a original empurra as vagas num edi com lixo nos 16 bits altos; a 0x4afbd0 lê a palavra
    embaralhar(v.as_mut_ptr() as usize + CQV7_FORA as usize * 4, nout as u32, vagas_fora as i16 as u16 as u32); // cqv7: os de fora da liga pela reputação
    // 7) a tabela
    let mut k = 0i32;
    for j in 0..(vagas_fora as i32).max(0) {
        let c = v[(CQV7_FORA + j) as usize];
        if c != 0 {
            cqv7_por(this, &mut k, c);
        }
    }
    for j in 0..(vagas_conf as i32).max(0) {
        let c = v[j as usize];
        if c != 0 {
            cqv7_por(this, &mut k, c);
        }
    }
    if a != 0 {
        cqv7_por(this, &mut k, a);
    }
    if b != 0 {
        cqv7_por(this, &mut k, b);
    }
    let (x1, x2) = (fora_do_ano[0], fora_do_ano[1]);
    cqv7_divisao(this, &mut k, 0x9cf5c8, &[a, b, x1, x2]); cqv7_divisao(this, &mut k, 0x9cf5c4, &[x1, x2]); // cqv7: a terceira e a segunda
    cqv7_divisao(this, &mut k, 0x9cf5c0, &[x1, x2]);
    crate::comparison::branch("cqv_7 primeira divisao embaralhada");
    for _ in 0..0x3c { // cqv7: 60 trocas na primeira divisão
        let r1 = sorteio(0x18) as i8 as i32;
        let r2 = sorteio(0x18) as i8 as i32;
        let t = cqv7_d(this + 0xa7);
        let p1 = t + (k - r1 - 1) as usize * 6;
        let p2 = t + (k - r2 - 1) as usize * 6;
        let v1 = cqv7_d(p1);
        cqv7_pd(p1, cqv7_d(p2));
        cqv7_pd(p2, v1);
    }
    cqv7_divisao(this, &mut k, 0x9cf5bc, &[x1, x2]);
    let nb = *((this + 0xb3) as *const i8);
    let mut j: i16 = 0;
    while j < nb as i16 {
        cqv7_por(this, &mut k, fora_do_ano[j as usize]);
        j += 1;
    }
    if *((this + 0xb4) as *const u8) == 1 {
        core::ptr::write_unaligned((this + 0x3e) as *mut i16, cqv7_w(this + 0x3e).wrapping_sub(1));
    }
    (k == cqv7_w(this + 0x3e) as i32) as i32
}

unsafe extern "thiscall" fn cqv7_inscritos_da_copa_da_inglaterra(this: usize) -> i32 {
    cqv7_inscritos(this)
}

/// Lado a lado a partir do mesmo estado (o sorteador, +0x3e, +0xb3, +0xb4, [0xb67a34]): a original deixa a tabela dela em
/// +0xa7, a cópia faz a sua (sem contar os sorteios), comparada nas vagas preenchidas (o clube de cada uma; a tabela
/// não é zerada) e liberada; a devolução também.
unsafe extern "thiscall" fn comparar_cqv7_inscritos_da_copa_da_inglaterra(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    let antes = cqv7_d(this + 0xa7);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + 0x3e, 2usize), (this + 0xb3, 2), (0xb67a34, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(cqv7_d(this + 0xa7));
        cqv7_pd(this + 0xa7, antes);
        r
    }, || {
        let r = crate::rng::without_counting(|| cqv7_inscritos(this));
        lr.set(cqv7_d(this + 0xa7));
        cqv7_pd(this + 0xa7, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = (cqv7_w(this + 0x3e).max(0) as usize).min(0x98);
    let tabela = lo == 0 || lr == 0 || a != 1 || (0..n).all(|j| cqv7_d(lo + j * 6) == cqv7_d(lr + j * 6) && *((lo + j * 6 + 4) as *const u16) == *((lr + j * 6 + 4) as *const u16));
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.record(igual && a == b && tabela, || format!("cqv7_inscritos_da_copa_da_inglaterra({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}
