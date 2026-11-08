// ---------- %%LEVA%%a leva: a Mercosul: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_merc.cpp) ----------

unsafe fn cqr11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
unsafe fn conmebol_merc_drawn_club(this: usize, k: usize) -> usize {
    cqr11_d(cqr11_d(this + 0xee) + k * 4) // cqr11: o sorteado k
}

/// { 0x4c5c90 } (thiscall, ret 8, conmebol_merc.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Mercosul.
/// Primeiro a virada: no dia 1º de junho do ano de hoje (a data de 0x549f70(1, 5, ano, −1): lea ecx, local; push −1;
/// push ano; push 5; push 1 -> thiscall, ret 0x10, pura: o dia do ano pelas tabelas dos meses, o mês contado de 0), se o ano da copa (+0x40,
/// com sinal) não é o de hoje, a virtual +8 (a temporada nova: mov ecx, this -> thiscall, ret 0).
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c5850:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c4e60, thiscall, ret 0; 0: a caixa da linha 0x396),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x39b), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x3a1), as fases dos grupos 0..3 (0x4c51d0: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e segue. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn conmebol_merc_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c5850usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4c4e60usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x4c51d0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    let mut junho = [0u8; 8];
    data(junho.as_mut_ptr(), 1, 5, ano_de_hoje as u32, -1);
    if core::ptr::read_unaligned(junho.as_ptr().add(2) as *const u16) == ano_de_hoje
        && core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(hoje as *const u16)
        && cqr11_w(this + 0x40) as i32 != ano_de_hoje as i32
    {
        let virada: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr11_d(cqr11_d(this) + 8));
        virada(this);
    }
    if cqr11_d(this + 0x45) == 0 && completo != 0 {
        let dia = cqr11_w(this + 0xb6) as u16; // cqr11: o dia do sorteio
        let ano = (cqr11_w(this + 0xb8) as u16).wrapping_add(cqr11_w(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparison::branch("cqr_11 mercosul draw");
            clubes(this);
            if primeiro(this) == 0 {
                show_error_at(0x9a7e5c, 0x396);
            }
            if jogos(this) == 0 {
                show_error_at(0x9a7e5c, 0x39b);
            }
            if ordenar(this, 0) == 0 {
                show_error_at(0x9a7e5c, 0x3a1);
            }
            for g in 0..4i32 {
                fase(this, g);
            }
            let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr11_d(cqr11_d(this) + 0x5c));
            previstas(this);
            soltar(cqr11_d(this + 0xee));
            cqr11_pd(this + 0xee, 0);
            cqr11_pd(this + 0x45, 1);
        }
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes (embaralhados), as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 4 fases
// criadas e a fase atual (+0x30) a última delas.
unsafe extern "thiscall" fn compare_conmebol_merc_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let sorteia = cqr11_d(this + 0x45) == 0
        && completo != 0
        && (cqr11_w(this + 0xb8) as u16).wrapping_add(cqr11_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqr11_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16) && !is_rollover_day(this, hoje);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqr11_d(this + 0xc);
        let ok = cqr11_d(this + 0x45) == 1
            && cqr11_d(this + 0xee) == 0
            && cqr11_d(this + 0xb1) != 0
            && fases != 0
            && (0..4).all(|g| cqr11_d(fases + g * 4) != 0)
            && cqr11_d(this + 0x30) == 3;
        SCOREBOARD.record(ok, || format!("conmebol_merc_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqr11_d(this + 0x45), cqr11_d(this + 0x30) as i32));
    }
    r
}

/// O dia da virada (1º de junho do ano de hoje, com a copa de outro ano): o comparador não confere o sorteio nesse dia.
unsafe fn is_rollover_day(this: usize, hoje: usize) -> bool {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    let mut junho = [0u8; 8];
    data(junho.as_mut_ptr(), 1, 5, ano_de_hoje as u32, -1);
    core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(hoje as *const u16) && cqr11_w(this + 0x40) as i32 != ano_de_hoje as i32
}

/// { 0x4c4e60 } (thiscall, ret 0, conmebol_merc.cpp): a tabela do primeiro grupo da Mercosul: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x155 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn conmebol_merc_create_first_group_table(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqr11_pd(this + 0xb1, t);
    if t == 0 {
        show_error_at(0x9a7e5c, 0x155);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqr11_w(this + 0x3e) {
        crate::comparison::branch("cqr_11 first group row");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, conmebol_merc_drawn_club(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
unsafe extern "thiscall" fn compare_conmebol_merc_create_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let lista = cqr11_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqr11_d(lista + i * 4));
    let r = o(this);
    let t = cqr11_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqr11_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqr11_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    SCOREBOARD.record(igual, || format!("conmebol_merc_create_first_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x4c51d0 } (thiscall, ret 4, conmebol_merc.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 3, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x1fb e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x211 e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
unsafe extern "thiscall" fn conmebol_merc_create_group_stage(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(((cqr11_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        show_error_at(0x9a7e5c, 0x1fb);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparison::branch("cqr_11 group team");
        cqr11_pd(times + n * 4, conmebol_merc_drawn_club(this, k as usize)); // cqr11: o time na vaga do grupo
        k += 1;
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr11_d(cqr11_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            cqr11_w(this + 0x3e) as u16 as u32,
            times,
            cqr11_w(this + 0x3c) as u16 as u32,
            cqr11_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqr11_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            cqr11_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr11_pd(cqr11_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr11_d(cqr11_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        show_error_at(0x9a7e5c, 0x211);
        return 0;
    }
    noticia(f);
    cqr11_pd(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn compare_conmebol_merc_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I2%%);
    let lista = cqr11_d(this + 0xee);
    if lista == 0 || !(0..4).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| cqr11_d(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqr11_d(cqr11_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqr11_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqr11_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqr11_d(this + 0x30) as i32 == g && cqr11_w(f + 0x3e) == 4 && cqr11_w(f + 0x40) == cqr11_w(this + 0x40) && a == plano;
        SCOREBOARD.record(ok, || format!("conmebol_merc_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}
