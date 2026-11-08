// ---------- %%LEVA%%a leva: o Campeonato de Clubes da Oceania: o dia do sorteio e as tabelas dos grupos A e B (oceania_club_champ.cpp) ----------

unsafe fn cqt10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube k dos 8 escolhidos (+0xee, ponteiros: o grupo A nas vagas 0..3, o B nas 4..7), a que as duas tabelas leem.
unsafe fn selected_club(this: usize, k: usize) -> usize {
    cqt10_d(cqt10_d(this + 0xee) + k * 4) // cqt10: o escolhido k
}

/// { 0x797b40 } (thiscall, ret 8, oceania_club_champ.cpp; a virtual +0x14; quadro SEH só das datas locais, sem
/// destrutor): `OceaniaClubChamp::update_for_date(hoje, completo)`, o dia do Campeonato de Clubes da Oceania. A data do
/// sorteio é a de 0x549ef0(+0xb6, +0xb8 + +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura). Antes
/// dela (o ano de hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de
/// liga. No dia do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo`: os 8 clubes (0x798220: mov ecx ->
/// thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall, ret 0), a tabela do grupo A (0x7978c0, thiscall,
/// ret 0; 0: a caixa da linha 0x21d), os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha
/// 0x224), a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x22b) (um erro volta 1,
/// sem o dia de liga), a fase do grupo B (0x797e80, thiscall, ret 0), a lista +0xee liberada (0x9452ca: push; add esp,
/// 4 -> cdecl) e zerada, +0xf2 = 0 e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this
/// -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn oceania_club_champ_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x798220usize);
    let grupo_a: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7978c0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let grupo_b: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x797e80usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let dia = cqt10_w(this + 0xb6) as u16;
    let ano = (cqt10_w(this + 0xb8) as u16).wrapping_add(cqt10_w(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do sorteio e nos erros a original devolve só o al (0 ou 1; o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && cqt10_d(this + 0x45) == 0 && completo == 0 {
        crate::comparison::branch("cqt_10 sorteio da oceania");
        clubes(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqt10_d(cqt10_d(this) + 0x5c));
        previstas(this);
        if grupo_a(this) == 0 {
            show_error_at(0xa6a068, 0x21d);
            return 1;
        }
        if jogos(this) == 0 {
            show_error_at(0xa6a068, 0x224);
            return 1;
        }
        if ordenar(this, 0) == 0 {
            show_error_at(0xa6a068, 0x22b);
            return 1;
        }
        grupo_b(this);
        soltar(cqt10_d(this + 0xee));
        cqt10_pd(this + 0xee, 0);
        cqt10_pd(this + 0xf2, 0);
        cqt10_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do grupo A e a fase do
// grupo B criadas e a fase atual (+0x30) = 0.
unsafe extern "thiscall" fn compare_oceania_club_champ_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let sorteia = cqt10_d(this + 0x45) == 0
        && completo == 0
        && (cqt10_w(this + 0xb8) as u16).wrapping_add(cqt10_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqt10_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqt10_d(this + 0xc);
        let ok = cqt10_d(this + 0x45) == 1
            && cqt10_d(this + 0xee) == 0
            && cqt10_d(this + 0xf2) == 0
            && cqt10_d(this + 0xb1) != 0
            && fases != 0
            && cqt10_d(fases) != 0
            && cqt10_d(this + 0x30) == 0;
        SCOREBOARD.record(ok, || format!("oceania_club_champ_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqt10_d(this + 0x45), cqt10_d(this + 0x30) as i32));
    }
    r
}

/// { 0x7978c0 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::create_group_a_table()`: 4 times (+0x3e), a
/// tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha
/// 0x182 e devolve 0), cada linha i com o escolhido i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push
/// clube; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this ->
/// thiscall, ret 0) e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn create_group_a_table(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqt10_pd(this + 0xb1, t);
    if t == 0 {
        show_error_at(0xa6a068, 0x182);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqt10_w(this + 0x3e) {
        crate::comparison::branch("cqt_10 linha do grupo a");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, selected_club(this, i as usize), 0, 0); // cqt10: a linha i do grupo A
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o escolhido i com a posição i (+4).
unsafe extern "thiscall" fn compare_create_group_a_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let lista = cqt10_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqt10_d(lista + i * 4));
    let r = o(this);
    let t = cqt10_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqt10_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqt10_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    SCOREBOARD.record(igual, || format!("create_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x797e80 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_group_b_stage()`: os times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp,
/// 4 -> cdecl; sem memória, a caixa da linha 0x296 e 0): os escolhidos 4 a 7 (+0xee); a tabela das rodadas pela virtual
/// +0x3c (push 0; push &id; push &rodadas; push 0; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a
/// tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), 0, o id (word),
/// +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos, que o construtor
/// não lê: a cópia passa os valores limpos) em [+0xc][0]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase:
/// a caixa da linha 0x2ad e 0. Com ela: +0x30 = 0, a notícia do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn create_group_b_stage(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0x10);
    if times == 0 {
        show_error_at(0xa6a068, 0x296);
        return 0;
    }
    for n in 0..4usize {
        crate::comparison::branch("cqt_10 time do grupo b");
        cqt10_pd(times + n * 4, selected_club(this, 4 + n)); // cqt10: o time na vaga do grupo B
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt10_d(cqt10_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 0, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            1,
            cqt10_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqt10_w(this + 0x40) as u16 as u32,
            0,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            cqt10_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqt10_pd(cqt10_d(this + 0xc), fase);
    soltar(times);
    soltar(tabela);
    let f = cqt10_d(cqt10_d(this + 0xc));
    if f == 0 {
        show_error_at(0xa6a068, 0x2ad);
        return 0;
    }
    cqt10_pd(this + 0x30, 0);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][0], +0x30 = 0, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// escolhidos 4 a 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn compare_create_group_b_stage(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let lista = cqt10_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let mut plano: Vec<usize> = (4..8).map(|k| cqt10_d(lista + k * 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = cqt10_d(cqt10_d(this + 0xc));
        let t = if f != 0 { cqt10_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqt10_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqt10_d(this + 0x30) == 0 && cqt10_w(f + 0x3e) == 4 && cqt10_w(f + 0x40) == cqt10_w(this + 0x40) && a == plano;
        SCOREBOARD.record(ok, || format!("create_group_b_stage: a fase não bate com o plano ({:x?} lidos {:x?})", plano, a));
    }
    r
}
