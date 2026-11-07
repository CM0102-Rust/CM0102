// ---------- 284a leva: a Copa Africana de Nações: o dia do sorteio, o sorteio dos 16 e as fases dos grupos B a D (african_nations.cpp) ----------

unsafe fn cqs13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// { 0x4025a0 } (thiscall, ret 8, african_nations.cpp; a virtual +0x14; quadro SEH só das datas locais, sem destrutor):
/// `AfricanNations::update_for_date(hoje, completo)`, o dia da Copa Africana. Sem `completo`, só o dia de liga. Com ele:
/// a data do sorteio é a de 0x549ef0(+0xb6, +0xb8 + +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8,
/// pura); antes dela (o ano de hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0
/// sem o dia de liga. No dia do sorteio, fora do sorteio feito (+0x45 = 0): o sorteio dos 16 (0x4022c0: mov ecx ->
/// thiscall, ret 0; a devolução não é lida), as posições previstas (a virtual +0x5c, thiscall, ret 0), a tabela do grupo
/// A (0x4019c0, thiscall, ret 0; 0: a caixa da linha 0x3c2), os jogos (0x6835c0 League::create_fixtures, thiscall, ret
/// 0; 0: a caixa da linha 0x3c8), a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha
/// 0x3cf) (um erro volta 0 sem o dia de liga), as fases dos grupos B a D (0x401ea0: push g; mov ecx -> thiscall, ret
/// 4, g = 0..2), a lista +0xee liberada (0x9452ca: push; add esp, 4 -> cdecl) e zerada, +0xf2 (palavra) = 0 e +0x45 =
/// 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela
/// repassa.
unsafe extern "thiscall" fn cqs13_africana_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4022c0usize);
    let grupo_a: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4019c0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x401ea0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    if completo == 0 {
        return liga(this, hoje, 0);
    }
    let dia = cqs13_w(this + 0xb6) as u16;
    let ano = (cqs13_w(this + 0xb8) as u16).wrapping_add(cqs13_w(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do sorteio e nos erros a original devolve só o al = 0 (o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && cqs13_d(this + 0x45) == 0 {
        crate::comparacao::ramo("cqs_13 dia do sorteio da copa africana");
        sortear(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqs13_d(cqs13_d(this) + 0x5c));
        previstas(this);
        if grupo_a(this) == 0 {
            aviso_em(0x9870f8, 0x3c2);
            return 0;
        }
        if jogos(this) == 0 {
            aviso_em(0x9870f8, 0x3c8);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            aviso_em(0x9870f8, 0x3cf);
            return 0;
        }
        for g in 0..3i32 {
            fase(this, g);
        }
        soltar(cqs13_d(this + 0xee));
        cqs13_pd(this + 0xee, 0);
        core::ptr::write_unaligned((this + 0xf2) as *mut u16, 0);
        cqs13_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: o sorteio, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do grupo A e as três fases
// criadas e a fase atual (+0x30) a última delas (2).
unsafe extern "thiscall" fn comparar_cqs13_africana_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2068);
    let sorteia = completo != 0
        && cqs13_d(this + 0x45) == 0
        && (cqs13_w(this + 0xb8) as u16).wrapping_add(cqs13_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqs13_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqs13_d(this + 0xc);
        let ok = cqs13_d(this + 0x45) == 1
            && cqs13_d(this + 0xee) == 0
            && cqs13_w(this + 0xf2) == 0
            && cqs13_d(this + 0xb1) != 0
            && fases != 0
            && (0..3).all(|g| cqs13_d(fases + g * 4) != 0)
            && cqs13_d(this + 0x30) == 2;
        PLACAR.registrar(ok, || format!("cqs13_africana_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqs13_d(this + 0x45), cqs13_d(this + 0x30) as i32));
    }
    r
}

/// { 0x4022c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::draw_finalists()`: +0xf2 (palavra) = 16 e a
/// lista +0xee de 50 (0x944e46(0xc8): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x349 e devolve 0). Na
/// frente as sedes da Copa Africana ([0x9cf7ac]) no ano +0x40 (0x5fa730 HostCountry::find_hosts em [0xb5e134]: push 1;
/// push &segunda; push &primeira; push ano; push competição -> thiscall, ret 0x14; devolve quantas, byte com sinal; as
/// saídas só são lidas até a conta), cada uma pela seleção (0x53b3d0 database_get_national_team: push nação; add esp ->
/// cdecl; as nações em [0xae23a8], 0x122 bytes). Depois, pela ordem das nações ([0xae2350], i como palavra), as do
/// continente africano ([0x9cfa08]; +0x71 da nação, o +0 dele) enquanto há menos de 50, menos as sedes (+0x53 da seleção).
/// Menos de 16: devolve 0 (sem caixa; a lista fica em +0xee). As de depois das sedes ordenadas pela reputação (qsort
/// 0x945333 com 0x4b07d0: 4 pushes; add esp -> cdecl). Os grupos (0x944e46(0x40); sem memória, a caixa da linha 0x380 e
/// devolve 0): para cada grupo j (0..3) e pote p (0..3), um sorteado (0x90cfc0(4): push 4; add esp -> cdecl; o byte com
/// sinal; de novo enquanto a vaga do pote já saiu) entre os 4 do pote (a lista nas vagas 4p a 4p + 3) vai para a vaga 4j
/// + p dos grupos e sai do pote (zerado). A lista liberada (0x9452ca) e +0xee = os grupos. Devolve 1.
unsafe fn cqs13_sorteio(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x5fa730usize);
    core::ptr::write_unaligned((this + 0xf2) as *mut u16, 0x10);
    let lista = alocar(0xc8);
    cqs13_pd(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0x9870f8, 0x349);
        return 0;
    }
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // ponytail: a original empurra o eax com o ano nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let quantas = sedes(cqs13_d(0xb5e134), cqs13_d(0x9cf7ac), cqs13_w(this + 0x40) as u16 as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut n = 0usize;
    if quantas > 0 {
        for &id in ids.iter().take(quantas as usize) {
            cqs13_pd(cqs13_d(this + 0xee) + n * 4, selecao(cqs13_d(0xae23a8) + id as usize * 0x122));
            n += 1;
        }
    }
    let mut i = 0i16;
    let mut nacao = cqs13_d(0xae23a8);
    while (i as i32) < cqs13_d(0xae2350) as i32 {
        let continente = cqs13_d(nacao + 0x71);
        if continente != 0 && cqs13_d(continente) == cqs13_d(0x9cfa08) && n < 0x32 { // cqs13: até 50 africanas
            let l = cqs13_d(this + 0xee);
            let sede = (0..quantas.max(0) as usize).any(|j| cqs13_d(cqs13_d(l + j * 4) + 0x53) == nacao);
            if !sede {
                cqs13_pd(cqs13_d(this + 0xee) + n * 4, selecao(nacao));
                n += 1;
            }
        }
        i = i.wrapping_add(1);
        nacao += 0x122;
    }
    if n < 0x10 {
        return 0;
    }
    let h = quantas as i32;
    ordenar(cqs13_d(this + 0xee) + (h as isize * 4) as usize, (n as i32 - h) as u32, 4, 0x4b07d0); // cqs13: as não sedes pela reputação
    let grupos = alocar(0x40);
    if grupos == 0 {
        aviso_em(0x9870f8, 0x380);
        return 0;
    }
    crate::comparacao::ramo("cqs_13 sorteio da copa africana");
    for j in 0..4usize {
        for p in 0..4usize {
            let mut r = sorteio(4) as u8 as i8 as usize;
            while cqs13_d(cqs13_d(this + 0xee) + (r + 4 * p) * 4) == 0 {
                r = sorteio(4) as u8 as i8 as usize;
            }
            let e = cqs13_d(this + 0xee) + (r + 4 * p) * 4;
            cqs13_pd(grupos + (4 * j + p) * 4, cqs13_d(e)); // cqs13: o do pote p na vaga 4j + p
            cqs13_pd(e, 0);
        }
    }
    soltar(cqs13_d(this + 0xee));
    cqs13_pd(this + 0xee, grupos);
    1
}

unsafe extern "thiscall" fn cqs13_africana_sorteio(this: usize) -> u32 {
    cqs13_sorteio(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa os grupos dela em +0xee, a cópia faz os seus, comparados nas 16 vagas e liberados (sem os 16, as listas
/// de 50 comparadas nas 16 primeiras vagas).
unsafe extern "thiscall" fn comparar_cqs13_africana_sorteio(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2069);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqs13_d(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqs13_sorteio(this));
        lr.set(cqs13_d(this + 0xee));
        cqs13_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqs13_africana_sorteio({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x401ea0 } (thiscall, ret 4, african_nations.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AfricanNations::create_group_stage(g)`, a fase do grupo g + 1 (B a D, g = 0..2, do dia do sorteio). Os
/// times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x2dc e 0):
/// os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push
/// g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add esp, 4 -> cdecl; o construtor League_Stage
/// 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a tabela das rodadas, as rodadas (word),
/// +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), g, o id (word), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a
/// original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores limpos)
/// em [+0xc][g]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x2f2 e 0. Com ela:
/// +0x30 = g, a notícia do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqs13_africana_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0x9870f8, 0x2dc);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqs_13 time do grupo");
        cqs13_pd(times + n * 4, cqs13_d(cqs13_d(this + 0xee) + ((g as usize + 1) * 4 + n) * 4)); // cqs13: o time na vaga do grupo
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqs13_d(cqs13_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            1,
            cqs13_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqs13_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            4,
            0,
            cqs13_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqs13_pd(cqs13_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqs13_d(cqs13_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9870f8, 0x2f2);
        return 0;
    }
    cqs13_pd(this + 0x30, g as usize);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqs13_africana_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2070);
    let lista = cqs13_d(this + 0xee);
    if lista == 0 || !(0..3).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| cqs13_d(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqs13_d(cqs13_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqs13_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqs13_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqs13_d(this + 0x30) as i32 == g && cqs13_w(f + 0x3e) == 4 && cqs13_w(f + 0x40) == cqs13_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqs13_africana_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 274a leva: a Copa Africana de Nações: a fase seguinte, a tabela do grupo A e o histórico da temporada (african_nations.cpp) ----------

unsafe fn cqk12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x401e10 } (thiscall, ret 0, african_nations.cpp; a virtual +0x28): `AfricanNations::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x2ba; senão +0x30 sobe e, chegando a 3 (depois dos grupos), o mata-mata
/// (0x402090: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqk12_africana_fase_seguinte(this: usize) {
    let f = cqk12_d(this + 0x30) as i32;
    if f >= (cqk12_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(0x9870f8, 0x2ba);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 3 { // cqk12: o mata-mata da Copa Africana depois dos grupos
        crate::comparacao::ramo("cqk_12 mata-mata da copa africana");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x402090usize);
        mata(this);
    }
}

// ponytail: o mata-mata (0x402090) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
unsafe extern "thiscall" fn comparar_cqk12_africana_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1709);
    let f = cqk12_d(this + 0x30) as i32;
    if f >= (cqk12_d(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = cqk12_d(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("cqk12_africana_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// { 0x4019c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::init_group_a_table()`: o grupo A com 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa
/// da linha 0x1f1 e devolve 0), cada linha i com a seleção i da lista dos sorteados (+0xee) (0x687430: push 0; push 0;
/// push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this
/// -> thiscall, ret 0) e devolve 1.
unsafe extern "thiscall" fn cqk12_africana_grupo_a(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    let times = 4i16; // cqk12: os quatro do grupo A
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, times);
    let t = novo(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        aviso_em(0x9870f8, 0x1f1);
        return 0;
    }
    crate::comparacao::ramo("cqk_12 grupo a da copa africana");
    let mut i = 0i8;
    while (i as i16) < cqk12_w(this + 0x3e) {
        let selecao = cqk12_d(cqk12_d(this + 0xee).wrapping_add((i as isize * 4) as usize)); // cqk12: a seleção da linha
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, selecao, 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico de posições) e a notícia vai aos técnicos: não roda duas
// vezes. Só a original; o plano: 4 times, a tabela, e em cada linha i a seleção i dos sorteados (+0xee) com a posição i
// (+4 da linha, 0x3b bytes).
unsafe extern "thiscall" fn comparar_cqk12_africana_grupo_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1710);
    let lista = cqk12_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqk12_d(lista + i * 4));
    let r = o(this);
    let t = cqk12_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqk12_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqk12_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqk12_africana_grupo_a({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x401b90 } (thiscall, ret 0, african_nations.cpp; a virtual +0x30): `AfricanNations::record_season_honours()`: o
/// histórico vem do mata-mata: salta para a virtual +0x30 da fase 3 ([+0xc] + 0xc; jmp, sem quadro).
unsafe extern "thiscall" fn cqk12_africana_historico(this: usize) {
    let fase = cqk12_d(cqk12_d(this + 0xc) + 0xc);
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqk12_d(cqk12_d(fase) + 0x30));
    h(fase);
}

// ponytail: o histórico (a virtual +0x30 do mata-mata, a 0x519a90 já em Rust com o seu comparador) não roda duas vezes;
// a rotina não grava nada além do salto. Só a original.
unsafe extern "thiscall" fn comparar_cqk12_africana_historico(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1711)(this)
}

// ---------- 273a leva: o destino das seleções nos torneios: a Copa Africana de Nações, a Copa América e a eliminatória da Oceania (african_nations.cpp, goldcup.cpp, wc_oceania_league.cpp) ----------

unsafe fn cqj10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqj10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push a6; push jogo; push clube; mov ecx,
/// [0xdd7f00] -> thiscall, ret 0xc).
unsafe fn cqj10_final(f: usize, clube: usize, jogo: usize, a6: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(cpz4_registros(), clube, jogo, a6);
}

/// A virtual +0x44 de um torneio de seleções com grupos e mata-mata (a Copa Africana e a Copa América): `fase` (byte com
/// sinal) abaixo de `grupos` é a fase de grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x78, peso 0x19); 0xfe,
/// a não classificação (0x86c1d0: fase 0x41c, peso 0x19), que encerra as convocações. Na fase `grupos` (o mata-mata,
/// a fase [+0xc][grupos]): 0, a classificação com a fase +0x32 do jogo, o +0x6f da rodada do jogo (+0x34) na tabela de
/// rodadas da fase (+0xa3, 0x68 bytes) e o peso 0x4bf850(0x28, 0x64, rodada, 3); 0xff, a eliminação (0x86c000) com
/// +0x32, +0x30 e o mesmo peso; 1, 2 e 3, os registros de campeão, vice e terceiro. Devolve 0.
unsafe fn cqj10_selecao(nome: u8, this: usize, clube: usize, desfecho: u32, fase: u32, jogo: usize, a6: u32, grupos: i8) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqj10_d(this + 4);
    if f < grupos {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            if nome == 0 {
                crate::comparacao::ramo("cqj_10 africana fora nos grupos");
                cpz4_nao_classificado(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa Africana
            } else {
                crate::comparacao::ramo("cqj_10 copa america fora nos grupos");
                cpz4_nao_classificado(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa América
            }
        }
    } else if f == grupos {
        match d {
            0 => {
                let r = cqj10_w(jogo + 0x34);
                let peso = cpz4_escala(0x28, 0x64, r, 3);
                let mata = cqj10_d(cqj10_d(this + 0xc) + grupos as usize * 4);
                let rodada = cqj10_w(cqj10_d(mata + 0xa3).wrapping_add((r as isize * 0x68) as usize) + 0x6f) as u16 as u32;
                cpz4_classificado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, rodada, peso);
            }
            0xff => {
                let peso = cpz4_escala(0x28, 0x64, cqj10_w(jogo + 0x34), 3);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
                if nome == 0 {
                    crate::comparacao::ramo("cqj_10 africana fora no mata-mata");
                    cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso); // cqj10: quem cai no mata-mata da Copa Africana
                } else {
                    cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso);
                }
            }
            1 => cqj10_final(0x86a800, clube, jogo, a6),
            2 => cqj10_final(0x86b0b0, clube, jogo, a6),
            3 => cqj10_final(0x86b710, clube, jogo, a6),
            _ => {}
        }
    }
    0
}

/// { 0x401ba0 } (thiscall, ret 0x18, african_nations.cpp; a virtual +0x44): `AfricanNations::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 3 fases de grupos (o
/// mata-mata na fase 3); o desfecho 4 não faz nada.
unsafe extern "thiscall" fn cqj10_copa_africana_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(0, this, clube, desfecho, fase, jogo, a6, 3)
}

// ponytail: os registros vão para o histórico dos técnicos e encerram as convocações: não rodam duas vezes, e a rotina
// não grava nada além deles. Só a original; o plano confere a devolução 0.
unsafe extern "thiscall" fn comparar_cqj10_copa_africana_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1694)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("cqj10_copa_africana_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// { 0x5e13b0 } (thiscall, ret 0x18, goldcup.cpp; a virtual +0x44 da Copa América): `GoldCup::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 2 fases de grupos (o
/// mata-mata na fase 2).
unsafe extern "thiscall" fn cqj10_copa_america_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(1, this, clube, desfecho, fase, jogo, a6, 2)
}

// ponytail: como a da Copa Africana: só a original, a devolução 0.
unsafe extern "thiscall" fn comparar_cqj10_copa_america_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1695)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("cqj10_copa_america_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// A competição atual da seleção acaba (0x775000 em [0xb74318]: push nação; mov ecx -> thiscall, ret 4).
unsafe fn cqj10_fim_da_competicao(clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x775000usize);
    f(cqj10_d(0xb74318), cqj10_d(clube + 0x53));
}

/// { 0x91e840 } (thiscall, ret 0x18, wc_oceania_league.cpp; a virtual +0x44): `WcOceaniaLeague::
/// update_staff_records_for_status(seleção, desfecho, fase, saida, jogo, _)`; devolve 0. Fase (byte com sinal) 0 ou
/// menos, os grupos: 6, a classificação (fase 0, rodada 0x82, peso 0x14); 0xfe, a não classificação (0x41c, 0x14) e a
/// competição atual da seleção acaba. Fase 1 (a final da Oceania): 1, a classificação (0xa0, 0, 0x1e); 2 ou 0xff, a
/// eliminação (+0x32 e +0x30 do jogo, peso 0x1e) e a competição acaba. Fase 2 (a repescagem): 1, a classificação para
/// a Copa do Mundo (a competição [0xae23d0] + 0x6b × [0x9cf79c], 0, 0, 0x64), a vaga nela (0x92e740 no objeto
/// [[0xadadfc] + 4 × [0x9cf79c]]: push seleção -> thiscall, ret 4) e *saida = 4; 2, a eliminação (peso 0x64) e a
/// competição acaba.
unsafe extern "thiscall" fn cqj10_eliminatoria_oceania_registros(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, _a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqj10_d(this + 4);
    let eliminar = |peso: u32| {
        // ponytail: +0x30 e +0x32 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
        cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso);
        cqj10_fim_da_competicao(clube);
    };
    if f < 1 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x41c, 0x14);
            cqj10_fim_da_competicao(clube);
        }
    } else if f == 1 {
        if d == 1 {
            cpz4_classificado(clube, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            crate::comparacao::ramo("cqj_10 oceania na copa do mundo");
            let id = cqj10_d(0x9cf79c);
            cpz4_classificado(clube, cqj10_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            let vaga: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x92e740usize);
            vaga(cqj10_d(cqj10_d(0xadadfc).wrapping_add(id.wrapping_mul(4))), clube);
            *(saida as *mut u8) = 4;
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

// ponytail: os registros, o fim da competição da seleção e a vaga na Copa do Mundo não rodam duas vezes. Só a original;
// o plano: a devolução 0 e, na repescagem ganha, *saida = 4 e a lista da Copa do Mundo (+0x36) um a mais (se cabia).
unsafe extern "thiscall" fn comparar_cqj10_eliminatoria_oceania_registros(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(1696);
    let vaga = fase as u8 == 2 && desfecho as u8 == 1 && saida != 0;
    let mundial = cqj10_d(cqj10_d(0xadadfc).wrapping_add(cqj10_d(0x9cf79c).wrapping_mul(4)));
    if vaga && mundial == 0 {
        return o(this, clube, desfecho, fase, saida, jogo, a6);
    }
    let n = if vaga { cqj10_w(mundial + 0x36) } else { 0 };
    let cabe = vaga && n < cqj10_w(mundial + 0x38);
    let r = o(this, clube, desfecho, fase, saida, jogo, a6);
    let igual = r == 0 && (!vaga || (*(saida as *const u8) == 4 && cqj10_w(mundial + 0x36) == if cabe { n + 1 } else { n }));
    PLACAR.registrar(igual, || format!("cqj10_eliminatoria_oceania_registros({:#x}, {}, {}) devolveu {} vaga={} lista {}", clube, desfecho as u8, fase as u8, r, vaga, n));
    r
}

