// ---------- %%LEVA%%a leva: a eliminatória da Oceania para a Copa do Mundo: o dia do começo, a tabela do primeiro grupo e a fase do segundo (wc_oceania_league.cpp) ----------

unsafe fn cqu12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqu12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A data do começo da eliminatória (0x549ef0 na data local: push ano; push dia; lea ecx -> thiscall, ret 8): o dia
/// +0xb6 e o ano +0xb8 + +0x40 (words), devolvidos como (dia, ano).
unsafe fn cqu12_comeco(this: usize) -> (u16, u16) {
    let montar: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    // a data tem 8 bytes: o dia, o ano e o dword +4 do ano bissexto, que a 0x549ef0 também grava
    let mut data = [0u32; 2];
    let ano = (cqu12_w(this + 0xb8) as u16).wrapping_add(cqu12_w(this + 0x40) as u16);
    montar(data.as_mut_ptr(), cqu12_w(this + 0xb6) as u16 as u32, ano as u32);
    (data[0] as u16, (data[0] >> 16) as u16)
}

/// { 0x91f3d0 } (thiscall, ret 8, wc_oceania_league.cpp; a virtual +0x14; quadro SEH só das datas locais, sem
/// destrutor): `WcOceaniaLeague::update_for_date(hoje, completo)`, o dia da eliminatória da Oceania. Antes do começo (o
/// ano de hoje menor, sem sinal; ou o mesmo ano e o dia menor, com sinal) volta 0 sem o dia de liga. No dia do começo
/// ou em qualquer dia de 2001 (o jogo que começa em 2001 monta a eliminatória já em curso), com +0x45 = 0 e COM
/// `completo`: as seleções da Oceania são liberadas das competições do continente (0x775070([0xb74318], continente
/// [0xae23a4] + 0xc6 × [0x9cfa18], 0): push 0; push; mov ecx -> thiscall, ret 8); cada seleção da tabela do primeiro
/// grupo (+0xb1, linhas de 0x3b bytes; +0x3e linhas) e cada uma das +0xf3 da lista +0xef (as não nulas) entra na
/// eliminatória (0x775220([0xb74318], nação, [+4]): push comp; push nação; mov ecx -> thiscall, ret 8); os jogos
/// (0x6835c0, thiscall, ret 0; 0: a caixa da linha 0x3a4 e 0), a tabela ordenada (0x6827d0(this, 0): push 0; mov ecx
/// -> thiscall, ret 4; 0: a caixa da linha 0x3ab e 0), a fase do segundo grupo (0x91f1a0, thiscall, ret 0) e +0x45 =
/// 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqu12_oceania_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let liberar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775070usize);
    let entrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x775220usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let segundo_grupo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f1a0usize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let (dia, ano) = cqu12_comeco(this);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do começo e nos erros a original devolve só o al (0); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    let no_dia = (ano_de_hoje == ano && dia_de_hoje == dia) || ano_de_hoje == 0x7d1; // cqu12: o jogo de 2001 monta já
    if no_dia && cqu12_d(this + 0x45) == 0 && completo != 0 {
        crate::comparison::branch("cqu_12 montagem da eliminatoria");
        let gerente = cqu12_d(0xb74318);
        liberar(gerente, cqu12_d(0xae23a4).wrapping_add(cqu12_d(0x9cfa18).wrapping_mul(0xc6)), 0);
        let mut i = 0i16;
        while i < cqu12_w(this + 0x3e) {
            let clube = cqu12_d(cqu12_d(this + 0xb1) + i as usize * 0x3b);
            entrar(gerente, cqu12_d(clube + 0x53), cqu12_d(this + 4));
            i += 1;
        }
        let mut k = 0i16;
        while (k as i32) < cqu12_d(this + 0xf3) as i32 {
            let e = cqu12_d(cqu12_d(this + 0xef) + k as usize * 4);
            if e != 0 {
                entrar(gerente, cqu12_d(e + 0x53), cqu12_d(this + 4));
            }
            k = k.wrapping_add(1);
        }
        if jogos(this) == 0 {
            show_error_at(0xad4afc, 0x3a4);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            show_error_at(0xad4afc, 0x3ab);
            return 0;
        }
        segundo_grupo(this);
        cqu12_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: as entradas nas competições, os jogos, a fase e o dia de liga não rodam duas vezes. Só a original; o plano
// na montagem (a condição da cópia): +0x45 = 1, a fase do segundo grupo em [+0xc][0] e a fase atual (+0x30) = 0.
unsafe extern "thiscall" fn comparar_cqu12_oceania_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let monta = cqu12_d(this + 0x45) == 0 && completo != 0 && {
        let (dia, ano) = cqu12_comeco(this);
        let (dh, ah) = (core::ptr::read_unaligned(hoje as *const u16), core::ptr::read_unaligned((hoje + 2) as *const u16));
        !(ah < ano || (ah == ano && (dh as i16) < dia as i16)) && ((ah == ano && dh == dia) || ah == 0x7d1)
    };
    let r = o(this, hoje, completo);
    if monta {
        let fases = cqu12_d(this + 0xc);
        let ok = cqu12_d(this + 0x45) == 1 && fases != 0 && cqu12_d(fases) != 0 && cqu12_d(this + 0x30) == 0;
        PLACAR.record(ok, || format!("cqu12_oceania_dia({:#x}): a montagem não fez a fase (+0x45 {}, +0x30 {})", this, cqu12_d(this + 0x45), cqu12_d(this + 0x30) as i32));
    }
    r
}

/// { 0x91ec00 } (thiscall, ret 0, wc_oceania_league.cpp): `WcOceaniaLeague::init_group_table()`: o primeiro grupo da
/// eliminatória da Oceania tem 5 seleções (+0x3e), a tabela +0xb1 (0x944e46(0x127): push; add esp, 4 -> cdecl; 5
/// linhas de 0x3b bytes; sem memória, a caixa da linha 0x24b e 0), cada linha i com a seleção das vagas pares da lista
/// +0xef (2i) (0x687430 League::init_table_row: push 0; push 0; push seleção; push i; mov ecx, this -> thiscall, ret
/// 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0) e devolve 1.
unsafe extern "thiscall" fn cqu12_oceania_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 5); // cqu12: 5 seleções no primeiro grupo
    let t = novo(0x127);
    cqu12_pd(this + 0xb1, t);
    if t == 0 {
        show_error_at(0xad4afc, 0x24b);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqu12_w(this + 0x3e) {
        crate::comparison::branch("cqu_12 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqu12_d(cqu12_d(this + 0xef) + i as isize as usize * 8), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só
// a original; o plano: devolve 1, +0x3e = 5 e na linha i (0x3b bytes) a seleção 2i da lista com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqu12_oceania_primeiro_grupo(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let lista = cqu12_d(this + 0xef);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 5] = core::array::from_fn(|i| cqu12_d(lista + i * 8));
    let r = o(this);
    let t = cqu12_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 5];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqu12_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqu12_w(this + 0x3e) == 5 && t != 0 && (0..5).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.record(igual, || format!("cqu12_oceania_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x91f1a0 } (thiscall, ret 0, wc_oceania_league.cpp; quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `WcOceaniaLeague::create_second_group_stage()`: o segundo grupo tem 4 seleções, ou 5 se o byte +0xee não é
/// 1; os times num vetor (0x944e46(4 × n): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x355 e 0) com as
/// seleções das vagas ímpares da lista +0xef (2i + 1); a tabela das rodadas pela virtual +0x3c (push 0; push &id; push
/// &rodadas; push 0; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee): push; add esp, 4 -> cdecl; o
/// construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a eliminatória, n, o vetor, 1, +4, a tabela
/// das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, um vetor local de 4 bytes {1, 0, 0, 0} (os desempates
/// [inferência]), o ano +0x40 (word), 0, o id (word), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e
/// bytes com sobra nos bytes altos, que o construtor não lê: a cópia passa os valores limpos) em [+0xc][0]; o vetor e
/// a tabela das rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x36b e 0. Com ela: +0x30 = 0, a notícia do
/// grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqu12_oceania_segundo_grupo(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let desempates: [u8; 4] = [1, 0, 0, 0];
    let n = 4 + (*((this + 0xee) as *const u8) != 1) as usize;
    let times = alocar((n * 4) as u32);
    if times == 0 {
        show_error_at(0xad4afc, 0x355);
        return 0;
    }
    for i in 0..n {
        crate::comparison::branch("cqu_12 time do segundo grupo");
        cqu12_pd(times + i * 4, cqu12_d(cqu12_d(this + 0xef) + i * 8 + 4)); // cqu12: a seleção na vaga i do segundo grupo
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqu12_d(cqu12_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 0, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            n as u32,
            times,
            1,
            cqu12_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            desempates.as_ptr() as usize,
            cqu12_w(this + 0x40) as u16 as u32,
            0,
            id as u16 as u32,
            b(0x51),
            4,
            0,
            cqu12_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqu12_pd(cqu12_d(this + 0xc), fase);
    soltar(times);
    soltar(tabela);
    let f = cqu12_d(cqu12_d(this + 0xc));
    if f == 0 {
        show_error_at(0xad4afc, 0x36b);
        return 0;
    }
    cqu12_pd(this + 0x30, 0);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][0], +0x30 = 0, a fase com n times e o ano da eliminatória e, na tabela dela (+0xb1, linhas de 0x3b
// bytes), as seleções das vagas ímpares da lista (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqu12_oceania_segundo_grupo(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let lista = cqu12_d(this + 0xef);
    if lista == 0 {
        return o(this);
    }
    let n = 4 + (*((this + 0xee) as *const u8) != 1) as usize;
    let mut plano: Vec<usize> = (0..n).map(|i| cqu12_d(lista + i * 8 + 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = cqu12_d(cqu12_d(this + 0xc));
        let t = if f != 0 { cqu12_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..n).map(|k| cqu12_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqu12_d(this + 0x30) == 0 && cqu12_w(f + 0x3e) as usize == n && cqu12_w(f + 0x40) == cqu12_w(this + 0x40) && a == plano;
        PLACAR.record(ok, || format!("cqu12_oceania_segundo_grupo: a fase não bate com o plano ({:x?} lidos {:x?})", plano, a));
    }
    r
}
