// ---------- %%LEVA%%a leva: a eliminatória da Oceania: as dez seleções e a final entre os campeões dos grupos; o segundo grupo da Copa das Nações da Oceania (wc_oceania_league.cpp, oceania_nations.cpp) ----------

unsafe fn cqw11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqw11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqw11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A nação de chave `id` é da Oceania (registro [0xae23a8] + 0x122 × id, +0x71 o continente, chave [0x9cfa18])?
unsafe fn cqw11_da_oceania(id: usize) -> bool {
    let c = cqw11_d(cqw11_d(0xae23a8).wrapping_add(id.wrapping_mul(0x122)) + 0x71);
    c != 0 && cqw11_d(c) == cqw11_d(0x9cfa18)
}

/// { 0x91fa80 } (thiscall, ret 0, wc_oceania_league.cpp): `WcOceaniaLeague::build_entrant_pool()`: +0xf3 = 10, +0xee = 0
/// (as isentas da Oceania), +0xa7 (word) = −1. As sedes da Copa do Mundo de dois anos depois (HostCountry::find_hosts
/// 0x5fa730([0xb5e134], [0x9cf79c], +0x40 + 2, &sede0, &sede1, 1): 5 pushes; mov ecx -> thiscall, ret 0x14; sem
/// [0xb5e134], a caixa da linha 0x452 e sem sedes): cada sede da Oceania é isenta (+0xee + 1), até a primeira −1. O
/// campeão da última Copa do Mundo (0x4afd80(comp, 0): 2 pushes; add esp, 8 -> cdecl), da Oceania e fora das sedes,
/// também. +0xf3 = 10 − isentas e a lista +0xef (0x944e46(4 × n): push; add esp, 4 -> cdecl; sem memória, a caixa da
/// linha 0x481 e 0). Das nações da Oceania fora a Nova Caledônia e a Papua-Nova Guiné, enquanto houver vaga: a seleção
/// principal (0x53b3d0: push; add esp, 4 -> cdecl) que não é sede nem o campeão. Sem completar as vagas devolve 0 (sem
/// ordenar); completas, a lista ordenada pela reputação (qsort 0x945333(lista, n, 4, 0x4b07d0): 4 pushes -> cdecl) e 1.
unsafe extern "thiscall" fn cqw11_eliminatoria_oceania_selecoes(this: usize) -> u32 {
    let sedes: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x5fa730usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let mut sede = [-1i32; 2];
    cqw11_pd(this + 0xf3, 10);
    *((this + 0xee) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0xa7) as *mut i16, -1);
    let hc = cqw11_d(0xb5e134);
    if hc == 0 {
        aviso_em(0xad4afc, 0x452);
    } else {
        let ano = (cqw11_w(this + 0x40) as u16).wrapping_add(2);
        let (a, b) = sede.split_at_mut(1);
        let n = sedes(hc, cqw11_d(0x9cf79c) as u32, ano as u32, &mut a[0], &mut b[0], 1) as u8 as i8;
        let mut k = 0i8;
        while k < n {
            let s = sede[k as usize];
            if s == -1 {
                break;
            }
            if cqw11_da_oceania(s as usize) {
                let p = (this + 0xee) as *mut u8;
                *p = (*p).wrapping_add(1);
            }
            k += 1;
        }
    }
    let copa = cqw11_d(0xae23d0).wrapping_add(cqw11_d(0x9cf79c).wrapping_mul(0x6b));
    let campeao = colocado(copa, 0);
    if campeao != 0 {
        let nacao = cqw11_d(campeao + 0x53);
        if nacao != 0 {
            let c = cqw11_d(nacao + 0x71);
            let id = cqw11_d(nacao) as i32;
            if c != 0 && cqw11_d(c) == cqw11_d(0x9cfa18) && id != sede[0] && id != sede[1] {
                let p = (this + 0xee) as *mut u8;
                *p = (*p).wrapping_add(1);
            }
        }
    }
    let n = 10 - *((this + 0xee) as *const i8) as i32;
    cqw11_pd(this + 0xf3, n as usize);
    let lista = alocar((n as u32).wrapping_mul(4));
    cqw11_pd(this + 0xef, lista);
    if lista == 0 {
        aviso_em(0xad4afc, 0x481);
        return 0;
    }
    let mut k = 0i32;
    let mut i = 0i16;
    while (i as i32) < cqw11_d(0xae2350) as i32 {
        let rec = cqw11_d(0xae23a8).wrapping_add((i as isize as usize).wrapping_mul(0x122));
        let cont = cqw11_d(rec + 0x71);
        let id = cqw11_d(rec) as i32;
        let fora = id == cqw11_d(0x9cf530) as i32 || id == cqw11_d(0x9cf41c) as i32; // cqw11: sem a Nova Caledônia e a Papua-Nova Guiné
        if cont != 0 && cqw11_d(cont) == cqw11_d(0x9cfa18) && !fora && k < cqw11_d(this + 0xf3) as i32 {
            let s = selecao(rec);
            let id = cqw11_d(rec) as i32;
            if id != sede[0] && id != sede[1] && s != campeao {
                crate::comparacao::ramo("cqw_11 selecao da eliminatoria");
                cqw11_pd(cqw11_d(this + 0xef) + k as usize * 4, s);
                k += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    if k != cqw11_d(this + 0xf3) as i32 {
        return 0;
    }
    ordenar(cqw11_d(this + 0xef), k as u32, 4, 0x4b07d0); // cqw11: as dez pela reputação
    1
}

// ponytail: a lista é alocada: não roda duas vezes. Só a original; o plano: com 1, +0xf3 = 10 − +0xee, a lista com
// seleções da Oceania (sem a Nova Caledônia e a Papua-Nova Guiné), sem repetição e pela reputação decrescente.
unsafe extern "thiscall" fn comparar_cqw11_eliminatoria_oceania_selecoes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let n = cqw11_d(this + 0xf3) as i32;
        let lista = cqw11_d(this + 0xef);
        let v: Vec<usize> = (0..n.max(0) as usize).map(|k| cqw11_d(lista + k * 4)).collect();
        let mut u = v.clone();
        u.sort_unstable();
        u.dedup();
        let ok = n == 10 - *((this + 0xee) as *const i8) as i32
            && u.len() == v.len()
            && v.iter().all(|&c| {
                let nacao = cqw11_d(c + 0x53);
                let id = cqw11_d(nacao);
                cqw11_da_oceania(id) && id != cqw11_d(0x9cf530) && id != cqw11_d(0x9cf41c)
            })
            && v.windows(2).all(|w| cqw11_w(w[0] + 0x80) >= cqw11_w(w[1] + 0x80));
        PLACAR.registrar(ok, || format!("cqw11_eliminatoria_oceania_selecoes({:#x}): {} seleções {:x?}", this, n, v));
    }
    r
}

/// { 0x91f640 } (thiscall, ret 0, wc_oceania_league.cpp; quadro SEH só para o new da fase): `WcOceaniaLeague::create_
/// final_stage()`: a final da Oceania entre os campeões dos dois grupos: um vetor de 2 (0x944e46(8); sem memória, a
/// caixa da linha 0x3ce e volta) com o primeiro da tabela da eliminatória (o primeiro grupo; 0x6826d0(this, 0): push;
/// mov ecx -> thiscall, ret 4) e o da fase 0 (o segundo grupo); a tabela das rodadas da fase 1 (a virtual +0x3c: push
/// 0; push &extra; push &rodadas; push 1; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) e o construtor
/// Cup_Stage 0x522e00, ret 0x3c: a eliminatória, 2, o vetor, as rodadas, +4, a tabela, o ano, 1, 4, o extra, 0x14, 0,
/// 0, 0, 0; words limpos) em [+0xc][1]; o vetor e a tabela liberados; sem fase, a caixa da linha 0x3e3; com ela, a
/// primeira rodada montada (Cup::set_up_round 0x51c800(fase, 0): push 0; mov ecx -> thiscall, ret 4).
unsafe extern "thiscall" fn cqw11_eliminatoria_oceania_final(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let rodada: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c800usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x522e00usize);
    let v = alocar(8);
    if v == 0 {
        return aviso_em(0xad4afc, 0x3ce);
    }
    crate::comparacao::ramo("cqw_11 final da oceania");
    cqw11_pd(v, cqw11_d(linha(this, 0))); // cqw11: o campeão do primeiro grupo manda primeiro
    cqw11_pd(v + 4, cqw11_d(linha(cqw11_d(cqw11_d(this + 0xc)), 0)));
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqw11_d(cqw11_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, 1, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, v, rodadas as u16 as u32, cqw11_d(this + 4), tabela, cqw11_w(this + 0x40) as u16 as u32, 1, 4, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    cqw11_pd(cqw11_d(this + 0xc) + 4, fase);
    soltar(v);
    soltar(tabela);
    let f = cqw11_d(cqw11_d(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0xad4afc, 0x3e3);
    }
    rodada(f, 0);
}

// ponytail: a fase é alocada e a rodada montada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com
// os dois campeões (o do primeiro grupo e o da fase 0) na tabela de times (+0xa7, linhas de 6 bytes), nessa ordem.
unsafe extern "thiscall" fn comparar_cqw11_eliminatoria_oceania_final(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let fases = cqw11_d(this + 0xc);
    if fases == 0 || cqw11_d(fases) == 0 || cqw11_d(this + 0xb1) == 0 || cqw11_d(cqw11_d(fases) + 0xb1) == 0 {
        return o(this);
    }
    let plano = [cqw11_d(cqw11_d(this + 0xb1)), cqw11_d(cqw11_d(cqw11_d(fases) + 0xb1))];
    o(this);
    let f = cqw11_d(cqw11_d(this + 0xc) + 4);
    if f == 0 {
        return;
    }
    let t = cqw11_d(f + 0xa7);
    let lidos = if t != 0 { [cqw11_d(t), cqw11_d(t + 6)] } else { [0, 0] };
    PLACAR.registrar(lidos == plano, || format!("cqw11_eliminatoria_oceania_final({:#x}): {:x?} plano {:x?}", this, lidos, plano));
}

/// { 0x799a60 } (thiscall, ret 4, oceania_nations.cpp; quadro SEH só para o new da fase): `OceaniaNations::create_group_
/// stage(fase)`: o segundo grupo da Copa das Nações da Oceania, as seleções 3, 4 e 5 da lista +0xee num vetor
/// (0x944e46(0xc); sem memória, a caixa da linha 0x331 e 0); a tabela das rodadas da `fase` (a virtual +0x3c); a fase
/// (0x944cf1(0xee) e o construtor League_Stage 0x68b100, ret 0x58: a copa, 3, o vetor, 1, +4, a tabela, as rodadas,
/// +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano, a fase, o id, +0x51, 4, 0, +0xd9, −1, 0, 2; words e bytes limpos) em
/// [+0xc][fase]; o vetor e a tabela liberados; sem fase, a caixa da linha 0x347 e 0; com ela, +0x30 = fase, a notícia
/// do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqw11_oceania_nacoes_segundo_grupo(this: usize, fase: u32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0xc);
    if times == 0 {
        aviso_em(0xa6a0ec, 0x331);
        return 0;
    }
    for i in 0..3usize {
        cqw11_pd(times + i * 4, cqw11_d(cqw11_d(this + 0xee) + 0xc + i * 4));
    }
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqw11_d(cqw11_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, fase, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let f = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(p, this, 3, times, 1, cqw11_d(this + 4), tabela, rodadas as u16 as u32, b(0xc2), b(0xc3), b(0xc4), this + 0xc5, this + 0xbe,
                  cqw11_w(this + 0x40) as u16 as u32, fase, id as u16 as u32, b(0x51), 4, 0, cqw11_w(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    cqw11_pd(cqw11_d(this + 0xc) + fase as usize * 4, f);
    soltar(times);
    soltar(tabela);
    let f = cqw11_d(cqw11_d(this + 0xc) + fase as usize * 4);
    if f == 0 {
        aviso_em(0xa6a0ec, 0x347);
        return 0;
    }
    cqw11_pd(this + 0x30, fase as usize);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1, a
// fase em [+0xc][fase], +0x30 = fase e as seleções 3, 4 e 5 da lista na tabela dela (em qualquer ordem).
unsafe extern "thiscall" fn comparar_cqw11_oceania_nacoes_segundo_grupo(this: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    let lista = cqw11_d(this + 0xee);
    if lista == 0 || fase > 8 {
        return o(this, fase);
    }
    let mut plano: Vec<usize> = (3..6).map(|k| cqw11_d(lista + k * 4)).collect();
    let r = o(this, fase);
    if r == 1 {
        let f = cqw11_d(cqw11_d(this + 0xc) + fase as usize * 4);
        let t = if f != 0 { cqw11_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..3).map(|k| cqw11_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        PLACAR.registrar(f != 0 && cqw11_d(this + 0x30) == fase as usize && a == plano, || format!("cqw11_oceania_nacoes_segundo_grupo: {:x?} plano {:x?}", a, plano));
    }
    r
}
