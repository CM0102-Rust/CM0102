// ---------- %%LEVA%%a leva: a Copa das Nações da Oceania: o dia do sorteio, o sorteio das seis seleções e a tabela do primeiro grupo (oceania_nations.cpp) ----------

unsafe fn cqv10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqv10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A data do sorteio (0x549ef0 na data local: push ano; push dia; lea ecx -> thiscall, ret 8): o dia +0xb6 e o ano
/// +0xb8 + +0x40 (words), devolvidos como (dia, ano).
unsafe fn cqv10_data_do_sorteio(this: usize) -> (u16, u16) {
    let montar: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    // a data tem 8 bytes: o dia, o ano e o dword +4 do ano bissexto, que a 0x549ef0 também grava
    let mut data = [0u32; 2];
    let ano = (cqv10_w(this + 0xb8) as u16).wrapping_add(cqv10_w(this + 0x40) as u16);
    montar(data.as_mut_ptr(), cqv10_w(this + 0xb6) as u16 as u32, ano as u32);
    (data[0] as u16, (data[0] >> 16) as u16)
}

/// { 0x79a100 } (thiscall, ret 8, oceania_nations.cpp; a virtual +0x14; quadro SEH só das datas locais, sem destrutor):
/// `OceaniaNations::update_for_date(hoje, completo)`, o dia da Copa das Nações da Oceania. Sem `completo`, só o dia de
/// liga (0x6847c0(this, hoje, 0): 2 pushes; mov ecx -> thiscall, ret 8). Antes do sorteio (o ano de hoje menor, sem
/// sinal; ou o mesmo ano e o dia menor, com sinal) volta 0 sem o dia de liga. No dia do sorteio, com +0x45 = 0: as seis
/// seleções (0x799e30, thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall, ret 0), a tabela do
/// primeiro grupo (0x7995a0, thiscall, ret 0; 0: a caixa da linha 0x405 e 0), os jogos (0x6835c0; 0: a caixa da linha
/// 0x40b e 0), a tabela ordenada (0x6827d0(this, 0): push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x412 e
/// 0), a fase do segundo grupo (0x799a60(this, 0): push 0; mov ecx -> thiscall, ret 4), a lista +0xee liberada
/// (0x9452ca) e zerada, +0xf2 (word) = 0 e +0x45 = 1. Depois o dia de liga, cuja devolução ela repassa.
unsafe extern "thiscall" fn cqv10_oceania_nacoes_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x799e30usize);
    let grupo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7995a0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let segundo: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x799a60usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    if completo == 0 {
        return liga(this, hoje, 0);
    }
    let (dia, ano) = cqv10_data_do_sorteio(this);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do sorteio e nos erros a original devolve só o al (0); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && cqv10_d(this + 0x45) == 0 {
        crate::comparison::branch("cqv_10 dia do sorteio");
        sortear(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqv10_d(cqv10_d(this) + 0x5c));
        previstas(this);
        if grupo(this) == 0 {
            show_error_at(0xa6a0ec, 0x405);
            return 0;
        }
        if jogos(this) == 0 {
            show_error_at(0xa6a0ec, 0x40b);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            show_error_at(0xa6a0ec, 0x412);
            return 0;
        }
        segundo(this, 0);
        soltar(cqv10_d(this + 0xee));
        cqv10_pd(this + 0xee, 0);
        core::ptr::write_unaligned((this + 0xf2) as *mut i16, 0);
        cqv10_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: o sorteio, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano no dia do
// sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do primeiro grupo e a fase do
// segundo criadas.
unsafe extern "thiscall" fn comparar_cqv10_oceania_nacoes_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let sorteia = completo != 0 && cqv10_d(this + 0x45) == 0 && {
        let (dia, ano) = cqv10_data_do_sorteio(this);
        core::ptr::read_unaligned(hoje as *const u16) == dia && core::ptr::read_unaligned((hoje + 2) as *const u16) == ano
    };
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqv10_d(this + 0xc);
        let ok = cqv10_d(this + 0x45) == 1 && cqv10_d(this + 0xee) == 0 && cqv10_w(this + 0xf2) == 0 && cqv10_d(this + 0xb1) != 0 && fases != 0 && cqv10_d(fases) != 0;
        PLACAR.record(ok, || format!("cqv10_oceania_nacoes_dia({:#x}): o sorteio não montou os grupos (+0x45 {})", this, cqv10_d(this + 0x45)));
    }
    r
}

/// O clube da seleção principal da nação de registro `rec` (0x53b3d0: push; add esp, 4 -> cdecl).
unsafe fn cqv10_selecao(rec: usize) -> usize {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    f(rec)
}

/// O registro da nação de chave `chave` (global): [0xae23a8] + 0x122 × [chave].
unsafe fn cqv10_nacao(chave: usize) -> usize {
    cqv10_d(0xae23a8).wrapping_add(cqv10_d(chave).wrapping_mul(0x122))
}

/// { 0x799e30 } (thiscall, ret 0, oceania_nations.cpp): `OceaniaNations::select_and_draw_teams()`: +0xf2 = 6 e a lista
/// +0xee de 6 seleções (0x944e46(0x18): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x39e e 0). Das
/// nações da Oceania (continente [0x9cfa18]) fora a Austrália e a Nova Zelândia: até 10 melanésias (0x53d900) e até 10
/// polinésias (0x53d940; a melanésia além das 10 cai no teste das polinésias), cada uma pela seleção principal
/// (0x53b3d0). Um sorteio de 2 (utils_random 0x90cfc0(2): push; add esp, 4 -> cdecl) põe a Austrália na vaga 0 e a Nova
/// Zelândia na 3 (com 1) ou o contrário; as melanésias e as polinésias são ordenadas e embaralhadas (0x4afbd0(lista,
/// n, 2): 3 pushes -> cdecl); outro sorteio põe as duas primeiras melanésias nas vagas 1 e 4 (ou 4 e 1) e outro as duas
/// primeiras polinésias nas 2 e 5 (ou 5 e 2). Devolve 1.
unsafe extern "thiscall" fn cqv10_oceania_nacoes_sorteio(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let melanesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d900usize);
    let polinesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d940usize);
    let sorteio: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x90cfc0usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    core::ptr::write_unaligned((this + 0xf2) as *mut i16, 6);
    let lista = alocar(0x18);
    cqv10_pd(this + 0xee, lista);
    if lista == 0 {
        show_error_at(0xa6a0ec, 0x39e);
        return 0;
    }
    let (mut mel, mut pol) = ([0usize; 10], [0usize; 10]);
    let (mut nm, mut np) = (0i8, 0i8);
    let oceania = cqv10_d(0x9cfa18);
    let mut i = 0i16;
    while (i as i32) < cqv10_d(0xae2350) as i32 {
        let rec = cqv10_d(0xae23a8).wrapping_add((i as isize as usize).wrapping_mul(0x122));
        let cont = cqv10_d(rec + 0x71);
        let k = i as i32;
        if cont != 0 && cqv10_d(cont) == oceania && k != cqv10_d(0x9cf224) as i32 && k != cqv10_d(0x9cf3f4) as i32 {
            if melanesia(rec) != 0 && nm < 10 {
                mel[nm as usize] = cqv10_selecao(rec);
                nm += 1;
            } else if polinesia(rec) != 0 && np < 10 {
                // cqv10: até 10 polinésias
                pol[np as usize] = cqv10_selecao(rec);
                np += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparison::branch("cqv_10 sorteio da copa da oceania");
    let (australia, nova_zelandia) = (cqv10_nacao(0x9cf224), cqv10_nacao(0x9cf3f4));
    if sorteio(2) != 0 { // cqv10: o sorteio da Austrália e da Nova Zelândia
        cqv10_pd(cqv10_d(this + 0xee), cqv10_selecao(australia));
        cqv10_pd(cqv10_d(this + 0xee) + 0xc, cqv10_selecao(nova_zelandia));
    } else {
        cqv10_pd(cqv10_d(this + 0xee) + 0xc, cqv10_selecao(australia));
        cqv10_pd(cqv10_d(this + 0xee), cqv10_selecao(nova_zelandia));
    }
    embaralhar(mel.as_mut_ptr() as usize, nm as i16 as u16 as u32, 2); // cqv10: as melanésias com 2 escolhas
    embaralhar(pol.as_mut_ptr() as usize, np as i16 as u16 as u32, 2);
    let l = |k: usize| cqv10_d(this + 0xee) + k * 4;
    if sorteio(2) != 0 {
        cqv10_pd(l(1), mel[0]);
        cqv10_pd(l(4), mel[1]);
    } else {
        cqv10_pd(l(4), mel[0]);
        cqv10_pd(l(1), mel[1]);
    }
    if sorteio(2) != 0 {
        cqv10_pd(l(2), pol[0]);
        cqv10_pd(l(5), pol[1]);
    } else {
        cqv10_pd(l(5), pol[0]);
        cqv10_pd(l(2), pol[1]);
    }
    1
}

// ponytail: a lista é alocada e os sorteios contam: não roda duas vezes. Só a original; o plano: devolve 1, +0xf2 = 6 e,
// na lista, a Austrália e a Nova Zelândia nas vagas 0 e 3 (em qualquer ordem).
unsafe extern "thiscall" fn comparar_cqv10_oceania_nacoes_sorteio(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let r = o(this);
    if r == 1 {
        let lista = cqv10_d(this + 0xee);
        let mut a = [cqv10_d(lista), cqv10_d(lista + 0xc)];
        let mut b = [cqv10_selecao(cqv10_nacao(0x9cf224)), cqv10_selecao(cqv10_nacao(0x9cf3f4))];
        a.sort_unstable();
        b.sort_unstable();
        PLACAR.record(cqv10_w(this + 0xf2) == 6 && a == b, || format!("cqv10_oceania_nacoes_sorteio({:#x}): vagas 0 e 3 {:x?} esperadas {:x?}", this, a, b));
    }
    r
}

/// { 0x7995a0 } (thiscall, ret 0, oceania_nations.cpp): `OceaniaNations::create_group_table()`: o primeiro grupo tem 3
/// seleções (+0x3e), a tabela +0xb1 (0x944e46(0xb1): 3 linhas de 0x3b bytes; sem memória, a caixa da linha 0x246 e 0),
/// cada linha i com a seleção i da lista +0xee (0x687430 League::init_table_row: push 0; push 0; push seleção; push i;
/// mov ecx -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230) e devolve 1. O corpo da 0x91ec00 da
/// eliminatória da Oceania com 3 linhas e a lista de 4 em 4 bytes.
unsafe extern "thiscall" fn cqv10_oceania_nacoes_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 3);
    let t = novo(0xb1);
    cqv10_pd(this + 0xb1, t);
    if t == 0 {
        show_error_at(0xa6a0ec, 0x246);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqv10_w(this + 0x3e) {
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqv10_d(cqv10_d(this + 0xee) + i as isize as usize * 4), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1,
// +0x3e = 3 e na linha i a seleção i da lista com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqv10_oceania_nacoes_primeiro_grupo(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let lista = cqv10_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 3] = core::array::from_fn(|i| cqv10_d(lista + i * 4));
    let r = o(this);
    let t = cqv10_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 3];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqv10_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqv10_w(this + 0x3e) == 3 && t != 0 && (0..3).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.record(igual, || format!("cqv10_oceania_nacoes_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}
