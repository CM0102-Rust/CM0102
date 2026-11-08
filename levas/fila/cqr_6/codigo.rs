// ---------- %%LEVA%%a leva: a virada das copas: a Supercopa japonesa, a Copa dos EUA e o Jogo das Estrelas da MLS (com os dois times das estrelas e o estádio sorteado) (jap_super.cpp, usa_open_cup.cpp, usa_mls_all_stars.cpp) ----------

unsafe fn cqr6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov edx, [esi]; mov ecx, esi; call [edx + o]).
unsafe fn cqr6_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqr6_d(cqr6_d(this) + o));
    f(this)
}

/// A virada da temporada de uma copa (`start_new_season`, slot 3, +0x08; o molde das copas de mata-mata, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c) e a árvore dos confrontos
/// (virtual +0x94); por fim, com `previstas`, as posições previstas (virtual +0x5c), cujo eax ela devolve (o Jogo das
/// Estrelas não as tem: devolve o eax da árvore).
unsafe fn cqr6_virada(this: usize, inscritos: usize, arquivo: usize, linhas: [u32; 3], ano: i16, previstas: bool, ramo: &'static str) -> u32 {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    cqr6_pd(this + 0x4c, 0);
    for o in [0xa7usize, 0xa3] {
        let l = cqr6_d(this + o);
        if l != 0 {
            soltar(l);
            cqr6_pd(this + o, 0);
        }
    }
    if cqr6_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqr6_w(this + 0x3c) as i32 {
            let p = cqr6_d(cqr6_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
                cqr6_pd(cqr6_d(this + 0xad) + i as usize * 4, 0);
            }
            i += 1;
        }
        soltar(cqr6_d(this + 0xad));
        cqr6_pd(this + 0xad, 0);
    }
    let e = cqr6_d(this + 8);
    if e != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(e, 1);
    }
    let a = cqr6_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    *((this + 0xab) as *mut u8) = 0;
    *((this + 0xb1) as *mut u8) = 0;
    let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(inscritos);
    // ponytail: nas caixas a original sai com o eax da caixa (0x5e8290); quem chama a virada não lê a devolução
    if inscritos(this) < 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    if cqr6_virtual(this, 0x8c) < 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let r = cqr6_virtual(this, 0x94);
    if r < 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    if !previstas {
        return r as u32;
    }
    cqr6_virtual(this, 0x5c) as u32
}

/// O comparador da virada: só a original (ponytail: libera as tabelas e refaz os inscritos, as rodadas e a árvore, que
/// alocam e sorteiam; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqr6_comparar(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = cqr6_w(this + 0x40);
    let r = original();
    let depois = cqr6_w(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// { 0x6664e0 } (thiscall, ret 0, jap_super.cpp; a virtual +0x08 da Supercopa japonesa): `JapSuper::start_new_season()`,
/// a virada (`cqr6_virada`) com os dois clubes de JapSuper::select_teams (0x666160) e as caixas do jap_super.cpp nas
/// linhas 0x14b, 0x152 e 0x159.
unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season(this: usize) -> u32 {
    cqr6_virada(this, 0x666160, 0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa") // cqr6: a Supercopa japonesa
}

unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqr6_comparar("usa_mls_all_stars_start_new_season", this, || o(this))
}

/// { 0x90b790 } (thiscall, ret 0, usa_open_cup.cpp; a virtual +0x08 da Copa dos EUA): `UsaOpenCup::start_new_season()`,
/// a virada (`cqr6_virada`) com os inscritos de UsaOpenCup::collect_entrants (0x90b530) e as caixas do usa_open_cup.cpp
/// nas linhas 0x198, 0x19f e 0x1a6.
unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90b790(this: usize) -> u32 {
    cqr6_virada(this, 0x90b530, 0xaa20e0, [0x198, 0x19f, 0x1a6], 1, true, "cqr_6 virada da copa dos eua") // cqr6: a Copa dos EUA
}

unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season_90b790(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqr6_comparar("usa_mls_all_stars_start_new_season_90b790", this, || o(this))
}

/// { 0x90a760 } (thiscall, ret 0, usa_mls_all_stars.cpp; a virtual +0x08 do Jogo das Estrelas): `UsaMlsAllStars::
/// start_new_season()`, a virada (`cqr6_virada`) com os dois times de UsaMlsAllStars::set_up_teams (0x90a540) e as caixas
/// do usa_mls_all_stars.cpp nas linhas 0x133, 0x13a e 0x141, sem as posições previstas.
unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    cqr6_virada(this, 0x90a540, 0xaa20a0, [0x133, 0x13a, 0x141], 1, false, "cqr_6 virada do jogo das estrelas") // cqr6: o Jogo das Estrelas
}

unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqr6_comparar("usa_mls_all_stars_start_new_season_90a760", this, || o(this))
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqr6_clube(i: usize) -> usize {
    cqr6_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// { 0x90a540 } (thiscall, ret 0, usa_mls_all_stars.cpp): `UsaMlsAllStars::set_up_teams()`, os dois times do Jogo das
/// Estrelas: +0x3e = 2; a lista (0x944e46 de 0xc, cdecl; sem memória, a caixa da linha 0xc6 e −1) com os dois clubes das
/// estrelas ([0x9d00a4] e [0x9d00a8], índices na tabela dos clubes): no ano ímpar (+0x40, com sinal) o de [0x9d00a4] é o
/// primeiro (o mandante) e a conferência dos estádios é 0x413; no ano par o de [0x9d00a8] é o primeiro e a conferência é
/// 0x414. Um sorteio de 0 a 5 (0x90cfc0(6), cdecl) escolhe o k-ésimo clube da MLS ([0x9cf590] na divisão do clube, +0x57)
/// dessa conferência (usa_mls_get_club_division 0x909ec0, push clube; add esp -> cdecl, palavra com sinal), pela ordem
/// da tabela: o primeiro time das estrelas fica com o estádio dele (+0x69) e +0x6d = 0 (sem o k-ésimo, nada muda). Os
/// dois ficam com a nação [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, os EUA [inferência]) e os quatro bytes da lista (+4,
/// +5, +0xa, +0xb) zerados. Devolve 1.
unsafe fn cqr6_times(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    cqr6_pd(this + 0xa7, l);
    if l == 0 {
        aviso_em(0xaa20a0, 0xc6);
        return -1;
    }
    let (a, b) = (cqr6_d(0x9d00a4), cqr6_d(0x9d00a8));
    let conferencia = if cqr6_w(this + 0x40) % 2 != 0 {
        cqr6_pd(l, cqr6_clube(a));
        cqr6_pd(cqr6_d(this + 0xa7) + 6, cqr6_clube(b));
        0x413
    } else {
        cqr6_pd(l + 6, cqr6_clube(a));
        cqr6_pd(cqr6_d(this + 0xa7), cqr6_clube(b));
        0x414
    };
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut k = sortear(6) as u8; // cqr6: o estádio sorteado entre 6
    let divisao: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x909ec0usize);
    let mut i = 0usize;
    while (i as i32) < cqr6_d(0xae235c) as i32 {
        let c = cqr6_clube(i);
        let liga = cqr6_d(c + 0x57);
        if liga != 0 && cqr6_d(liga) == cqr6_d(0x9cf590) && divisao(c) as i32 == conferencia {
            if k == 0 {
                crate::comparacao::ramo("cqr_6 estadio do jogo das estrelas");
                let primeiro = cqr6_d(cqr6_d(this + 0xa7));
                cqr6_pd(primeiro + 0x69, cqr6_d(c + 0x69)); // cqr6: o estádio do clube sorteado
                *((cqr6_d(cqr6_d(this + 0xa7)) + 0x6d) as *mut u8) = 0;
                break;
            }
            k = k.wrapping_sub(1);
        }
        i += 1;
    }
    let nacao = cqr6_d(0xae23a8).wrapping_add(cqr6_d(0x9cf4f8).wrapping_mul(0x122));
    let l = cqr6_d(this + 0xa7);
    cqr6_pd(cqr6_d(l + 6) + 0x53, nacao);
    cqr6_pd(cqr6_d(l) + 0x53, cqr6_d(cqr6_d(l + 6) + 0x53));
    for o in [4usize, 5, 0xa, 0xb] {
        *((cqr6_d(this + 0xa7) + o) as *mut u8) = 0;
    }
    1
}

unsafe extern "thiscall" fn set_up_teams(this: usize) -> i32 {
    cqr6_times(this)
}

/// Lado a lado a partir do mesmo estado (o sorteador, +0x3e, os dois clubes das estrelas e [0xb67a34]); a original deixa
/// a lista dela em +0xa7, a cópia faz a sua (sem contar o sorteio), comparada nos 12 bytes e liberada. A lista antiga
/// (+0xa7 na entrada) não é liberada por nenhuma das duas (a virada já a liberou).
unsafe extern "thiscall" fn comparar_set_up_teams(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I3%%);
    let antes = cqr6_d(this + 0xa7);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x3e, 2usize), (0xb67a34, 4), (cqr6_clube(cqr6_d(0x9d00a4)), 0x245), (cqr6_clube(cqr6_d(0x9d00a8)), 0x245)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqr6_d(this + 0xa7));
        cqr6_pd(this + 0xa7, antes);
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqr6_times(this));
        lr.set(cqr6_d(this + 0xa7));
        cqr6_pd(this + 0xa7, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0xc) == core::slice::from_raw_parts(lr as *const u8, 0xc)
    } else {
        lo == lr
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("set_up_teams({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}
