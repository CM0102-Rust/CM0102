// ---------- %%LEVA%%a leva: a Copa América: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros dois grupos (goldcup.cpp) ----------

unsafe fn cqr13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A seleção k da lista dos sorteados (+0xee, ponteiros: os 12 em grupos intercalados, o grupo j nas vagas j, j + 3,
/// j + 6 e j + 9), a que o primeiro grupo e as fases dos outros leem.
unsafe fn cqr13_sorteado(this: usize, k: usize) -> usize {
    cqr13_d(cqr13_d(this + 0xee) + k * 4) // cqr13: o sorteado k
}

/// { 0x5e2120 } (thiscall, ret 8, goldcup.cpp; a virtual +0x14; quadro SEH só das datas locais, sem destrutor):
/// `GoldCup::update_for_date(hoje, completo)`, o dia da Copa América. A data do sorteio é a de 0x549ef0(+0xb6, +0xb8 +
/// +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura: o dia do ano e o ano). Antes dela (o ano de
/// hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de liga. No dia
/// do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo` (a original sorteia só na chamada com completo = 0):
/// a lista das 12 seleções (0x5e1cd0: mov ecx -> thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall,
/// ret 0), a tabela do primeiro grupo (0x5e11e0, thiscall, ret 0; 0: a caixa da linha 0x3e7), os jogos (0x6835c0
/// League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x3ed), a tabela ordenada (0x6827d0 push 0; mov ecx ->
/// thiscall, ret 4; 0: a caixa da linha 0x3f4) (um erro volta 0 sem o dia de liga), as fases dos grupos 0 e 1 (0x5e16b0:
/// push g; mov ecx -> thiscall, ret 4), a lista +0xee liberada (0x9452ca: push; add esp, 4 -> cdecl) e zerada, +0xf2 = 0
/// e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa.
unsafe extern "thiscall" fn cqr13_copa_america_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let selecoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5e1cd0usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5e11e0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x5e16b0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let dia = cqr13_w(this + 0xb6) as u16; // cqr13: o dia do sorteio
    let ano = (cqr13_w(this + 0xb8) as u16).wrapping_add(cqr13_w(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas duas voltas antes do sorteio e nos erros a original devolve só o al = 0 (o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && cqr13_d(this + 0x45) == 0 && completo == 0 {
        crate::comparacao::ramo("cqr_13 sorteio da copa america");
        selecoes(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr13_d(cqr13_d(this) + 0x5c));
        previstas(this);
        if primeiro(this) == 0 {
            aviso_em(0x9cd640, 0x3e7);
            return 0;
        }
        if jogos(this) == 0 {
            aviso_em(0x9cd640, 0x3ed);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            aviso_em(0x9cd640, 0x3f4);
            return 0;
        }
        for g in 0..2i32 {
            fase(this, g);
        }
        soltar(cqr13_d(this + 0xee));
        cqr13_pd(this + 0xee, 0);
        cqr13_pd(this + 0xf2, 0);
        cqr13_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: a lista, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano no
// dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do primeiro grupo e as duas
// fases criadas e a fase atual (+0x30) a última delas (1).
unsafe extern "thiscall" fn comparar_cqr13_copa_america_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let sorteia = cqr13_d(this + 0x45) == 0
        && completo == 0
        && (cqr13_w(this + 0xb8) as u16).wrapping_add(cqr13_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqr13_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqr13_d(this + 0xc);
        let ok = cqr13_d(this + 0x45) == 1
            && cqr13_d(this + 0xee) == 0
            && cqr13_d(this + 0xf2) == 0
            && cqr13_d(this + 0xb1) != 0
            && fases != 0
            && (0..2).all(|g| cqr13_d(fases + g * 4) != 0)
            && cqr13_d(this + 0x30) == 1;
        PLACAR.registrar(ok, || format!("cqr13_copa_america_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqr13_d(this + 0x45), cqr13_d(this + 0x30) as i32));
    }
    r
}

/// { 0x5e11e0 } (thiscall, ret 0, goldcup.cpp): `GoldCup::create_group_table()`, o grupo A da Copa América: 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a
/// caixa da linha 0x1d4 e devolve 0), cada linha i com o sorteado 3i (+0xee: os grupos intercalados) (0x687430
/// League::init_table_row: push 0; push 0; push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do
/// sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0) e devolve 1.
unsafe extern "thiscall" fn cqr13_copa_america_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqr13_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9cd640, 0x1d4);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqr13_w(this + 0x3e) {
        crate::comparacao::ramo("cqr_13 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr13_sorteado(this, i as usize * 3), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado 3i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqr13_copa_america_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let lista = cqr13_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqr13_d(lista + i * 12));
    let r = o(this);
    let t = cqr13_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqr13_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqr13_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqr13_copa_america_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x5e16b0 } (thiscall, ret 4, goldcup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal):
/// `GoldCup::create_group_stage(g)`, a fase do grupo g + 1 (g = 0 e 1, do dia do sorteio). Os times num vetor de 4
/// ponteiros (0x944e46(0x10): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x2bf e 0): os sorteados g + 1,
/// g + 4, g + 7 e g + 10 (+0xee); a tabela das rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push g; mov
/// ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add esp, 4 -> cdecl; o construtor League_Stage
/// 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a tabela das rodadas, as rodadas (word),
/// +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), g, o id (word), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a
/// original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores limpos)
/// em [+0xc][g]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x2d6 e 0. Com ela:
/// +0x30 = g, a notícia do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqr13_copa_america_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0x9cd640, 0x2bf);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqr_13 time do grupo");
        cqr13_pd(times + n * 4, cqr13_sorteado(this, (g + 1) as usize + n * 3)); // cqr13: o time na vaga do grupo
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr13_d(cqr13_d(this) + 0x3c));
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
            cqr13_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqr13_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            4,
            0,
            cqr13_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr13_pd(cqr13_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr13_d(cqr13_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9cd640, 0x2d6);
        return 0;
    }
    cqr13_pd(this + 0x30, g as usize);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados g + 1, g + 4, g + 7 e g + 10 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqr13_copa_america_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I2%%);
    let lista = cqr13_d(this + 0xee);
    if lista == 0 || !(0..2).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|n| cqr13_d(lista + ((g + 1) as usize + n * 3) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqr13_d(cqr13_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqr13_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqr13_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqr13_d(this + 0x30) as i32 == g && cqr13_w(f + 0x3e) == 4 && cqr13_w(f + 0x40) == cqr13_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqr13_copa_america_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}
