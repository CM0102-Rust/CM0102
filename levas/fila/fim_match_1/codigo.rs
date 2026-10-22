// ---------- %%LEVA%%a leva: o fim das telas da partida: a partida que o humano deixa e a ficha do jogo de um registro de estatística (match_day.cpp, match_screens.cpp) ----------

/// { 0x6b6970 } (cdecl, match_day.cpp): o humano da vez ([0xb77c7a]) deixa a partida que assistia
/// (chamada só quando um jogador da rede sai, 0x838c00). A partida dele ([0xb74c26 + 0x300 × vez],
/// 0x765480 → +0xe2): se outro dos 16 humanos (as últimas 16 fichas de [0xae23bc], 110 bytes, +0x5f
/// ativo) assiste a mesma e é a vez dele (0x7fffa0 no gerente), sai calado; senão tira a vez dos
/// quatro vetores do resumo (+0x5b: +5/+0x45/+0x85/+0xc5, os contadores +0x105..+0x108). Ninguém mais
/// assistindo: cria o motor (+0x57, 0xf63b bytes, 0x6b7e50 + 0x6b7f20; sem ele erro 0xa4b), anda até
/// o fim (0x6b98c0 com −1; estado +0x67, 2 ou 8 contam em [[0xb67a50]+0x25]), fora do 8 cria o resumo
/// (+0x5f, 0xd7a bytes com 2 objetos de 0x625 em +0x11f; sem ele erro 0xa68) e registra (0x7293f0),
/// libera o motor e tira a partida da fila [0xde2f70] (0x90d6b0).
unsafe extern "C" fn largar_partida_assistida() {
    crate::comparacao::ramo("telas largar a partida assistida");
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x7fffa0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x6b7e50usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x6b7f20usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x6b98c0usize);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7293f0usize);
    let destruir_motor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6b97a0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d6b0usize);
    let vez = || *(0xb77c7a as *const i16) as isize;

    let p = u((0xb74c26 + vez() * 0x300) as usize, 0);
    if p == 0 {
        return;
    }
    let a = ficha(p);
    if a == 0 {
        return;
    }
    let jogo = u(a, 0xe2);
    if jogo == 0 {
        return;
    }
    let mut sozinho = true;
    for i in 0..0x10usize {
        let idx = (*(0xae2364 as *const usize)).wrapping_add(i).wrapping_sub(0x10);
        let q = u(0xae23bc, 0).wrapping_add(idx.wrapping_mul(110));
        // como a original, sem conferir a ficha nula do outro humano
        if *((q + 0x5f) as *const u8) != 0 && q != p && u(ficha(q), 0xe2) == jogo {
            let minha = *(0xb77c7a as *const u16);
            if indice(0xb74450, q) == minha {
                return;
            }
            sozinho = false;
        }
    }
    let resumo = u(jogo, 0x5b);
    if resumo == 0 {
        return;
    }
    for (k, o) in [5isize, 0x45, 0x85, 0xc5].into_iter().enumerate() {
        let c = (resumo as isize + vez() * 4 + o) as *mut u32;
        if core::ptr::read_unaligned(c) != 0 {
            core::ptr::write_unaligned(c, 0);
            let n = (resumo + 0x105 + k) as *mut u8;
            *n = (*n).wrapping_sub(1);
        }
    }
    if !sozinho {
        return;
    }
    let motor = (jogo + 0x57) as *mut usize;
    if core::ptr::read_unaligned(motor) == 0 {
        let mem = novo(0xf63b);
        let m = if mem != 0 { construir(mem) } else { 0 };
        core::ptr::write_unaligned(motor, m);
        if m == 0 {
            aviso_em(0xa15708, 0xa4b);
            return;
        }
        preparar(m, jogo + 4, 0, 0, 0);
    }
    let estado = (jogo + 0x67) as *mut u8;
    while *estado != 2 && *estado != 8 {
        let e = laco(core::ptr::read_unaligned(motor), -1) as u8;
        *estado = e;
        if e == 2 || e == 8 {
            *((u(0xb67a50, 0) + 0x25) as *mut i32) += 1;
        }
    }
    if *estado != 8 {
        let mem = novo(0xd7a);
        if mem != 0 {
            vetor(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        core::ptr::write_unaligned((jogo + 0x5f) as *mut usize, mem);
        if mem == 0 {
            aviso_em(0xa15708, 0xa68);
            return;
        }
        registrar(u(0xb6b7c0, 0), core::ptr::read_unaligned(motor), mem);
    }
    let m = core::ptr::read_unaligned(motor);
    if m != 0 {
        destruir_motor(m);
        liberar(m);
    }
    core::ptr::write_unaligned(motor, 0);
    tirar(u(0xde2f70, 0), jogo + 4);
}

/// { 0x729e30 } (thiscall, ret 8, match_stats.cpp): a ficha do jogo (0x4f bytes) do registro `k` do
/// índice de estatísticas ([this], 12 bytes: +0 competição, +4 um dos clubes, +8/+0xa a data): pede
/// à agenda [0xae2a58] (0x5acbe0) os jogos daquela data e competição numa lista local (0x68d020) e copia
/// para `saida` o primeiro com o clube +4 na casa (+0xc) ou fora (+0x10). Devolve 1 se achou. Índice
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro, 0).
unsafe extern "thiscall" fn ficha_do_registro(estat: usize, k: i32, saida: usize) -> u32 {
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(%%I1%%)(estat, k, saida);
    }
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    // ponytail: a lista nasce zerada aqui, que é tudo o que o construtor 0x68d020 faz
    let mut lista = [0u8; 14];
    let l = lista.as_mut_ptr() as usize;
    let tab = *(estat as *const usize);
    let e = tab.wrapping_add((k as usize).wrapping_mul(12));
    // a original empurra os registradores inteiros: em cima, os 16 bits de [this] e de k × 3
    let d1 = (tab as u32 & 0xffff_0000) | *((e + 8) as *const u16) as u32;
    let d2 = ((k as u32).wrapping_mul(3) & 0xffff_0000) | *((e + 0xa) as *const u16) as u32;
    jogos(*(0xae2a58 as *const usize), d1, d2, d1, d2, *(e as *const u32), l, u32::MAX, 0xffff_fffe, u32::MAX);
    // o cursor (+0xa) começa na cabeça (+2)
    core::ptr::copy(lista.as_ptr().add(2), lista.as_mut_ptr().add(0xa), 4);
    let clube = *((e + 4) as *const u32);
    let mut f = proximo(l);
    while f != 0 {
        if clube == *((f + 0xc) as *const u32) || clube == *((f + 0x10) as *const u32) {
            core::ptr::copy_nonoverlapping(f as *const u8, saida as *mut u8, 0x4f);
            destruir_lista(l);
            return 1;
        }
        f = proximo(l);
    }
    destruir_lista(l);
    0
}

// ponytail: a partida deixada cria e libera o motor, anda a partida até o fim e a registra no arquivo
// temporário: rodar duas vezes não dá, então o comparador só chama a original; a cópia se prova no
// modo Rust (o ramo), na sessão do Lucas com rede (só um jogador da rede saindo a chama).
unsafe extern "C" fn comparar_largar_partida_assistida() {
    orig::<unsafe extern "C" fn()>(%%I0%%)()
}

unsafe extern "thiscall" fn comparar_ficha_do_registro(estat: usize, k: i32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(%%I1%%);
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return o(estat, k, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 0x4f)], || o(estat, k, saida), || ficha_do_registro(estat, k, saida));
    PLACAR.registrar(igual && r == n, || format!("ficha_do_registro(k {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}
