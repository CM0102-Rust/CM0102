// ---------- 282a leva: a Libertadores: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_liber.cpp) ----------




/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
unsafe fn cqr10_sorteado(this: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) // cqr10: o sorteado k
}

/// { 0x4c4410 } (thiscall, ret 8, conmebol_liber.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Libertadores.
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c3f40:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c37a0, thiscall, ret 0; 0: a caixa da linha 0x365),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x36a), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x370), as fases dos grupos 0..6 (0x4c3b10: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e segue. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqr10_libertadores_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c3f40usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4c37a0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x4c3b10usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    if read_u32_as_usize(this + 0x45) == 0 && completo != 0 {
        let dia = read_i16(this + 0xb6) as u16; // cqr10: o dia do sorteio
        let ano = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparacao::ramo("cqr_10 sorteio da libertadores");
            clubes(this);
            if primeiro(this) == 0 {
                aviso_em(0x9a7de4, 0x365);
            }
            if jogos(this) == 0 {
                aviso_em(0x9a7de4, 0x36a);
            }
            if ordenar(this, 0) == 0 {
                aviso_em(0x9a7de4, 0x370);
            }
            for g in 0..7i32 {
                fase(this, g);
            }
            let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
            previstas(this);
            soltar(read_u32_as_usize(this + 0xee));
            write_u32_from_usize(this + 0xee, 0);
            write_u32_from_usize(this + 0x45, 1);
        }
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes (embaralhados), as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 7 fases
// criadas e a fase atual (+0x30) a última delas.
unsafe extern "thiscall" fn comparar_cqr10_libertadores_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2006);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && completo != 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && (0..7).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 6;
        PLACAR.registrar(ok, || format!("cqr10_libertadores_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// { 0x4c37a0 } (thiscall, ret 0, conmebol_liber.cpp): a tabela do primeiro grupo da Libertadores: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x16f e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn cqr10_libertadores_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9a7de4, 0x16f);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + 0x3e) {
        crate::comparacao::ramo("cqr_10 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr10_sorteado(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqr10_libertadores_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2007);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(lista + i * 4));
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && read_i16(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqr10_libertadores_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x4c3b10 } (thiscall, ret 4, conmebol_liber.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 6, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x218 e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x22e e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
unsafe extern "thiscall" fn cqr10_libertadores_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0x9a7de4, 0x218);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparacao::ramo("cqr_10 time do grupo");
        write_u32_from_usize(times + n * 4, cqr10_sorteado(this, k as usize)); // cqr10: o time na vaga do grupo
        k += 1;
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            read_i16(this + 0x3e) as u16 as u32,
            times,
            read_i16(this + 0x3c) as u16 as u32,
            read_u32_as_usize(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            read_i16(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            read_i16(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9a7de4, 0x22e);
        return 0;
    }
    noticia(f);
    write_u32_from_usize(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqr10_libertadores_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2008);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 || !(0..7).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) as i32 == g && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqr10_libertadores_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 274a leva: as regras dos grupos continentais de clubes: a Libertadores, a Copa Merconorte e o Campeonato de Clubes da Oceania (conmebol_liber.cpp, con_merc_cup.cpp, oceania_club_champ.cpp) ----------

/// As regras de um grupo continental (a virtual +0x8c, set_up_rules, numa liga de grupo): os campos de byte e palavra
/// gravados (deslocamento, tamanho 1 ou 2, valor), a data de começo (0x549f70: dia, mês, ano da competição +0x40, dia da
/// semana; −1 = qualquer) e o fundo de prêmios (+0x28), quando há.
struct Cqj11Regras {
    campos: &'static [(usize, u8, u32)],
    dia: u32,
    mes: u32,
    semana: u32,
    premio: Option<u32>,
}

/// Grava os campos de `g` (os pontos da vitória, +0xc2, vêm de `vitoria`); a data de começo numa Date local (0x549f70: push semana; push ano; push mês; push dia; lea
/// ecx, local -> thiscall, ret 0x10) e a primeira palavra dela em +0xb6, +0xb8 = 0; o prêmio; depois a tabela das
/// rodadas da virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba.
/// Devolve 1.
unsafe fn cqj11_regras(this: usize, g: &Cqj11Regras, dia: u32, vitoria: u8) -> u32 {
    for &(o, n, v) in g.campos {
        let v = if o == 0xc2 { vitoria as u32 } else { v };
        if n == 1 {
            *((this + o) as *mut u8) = v as u8;
        } else if n == 2 {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        } else {
            core::ptr::write_unaligned((this + o) as *mut u32, v);
        }
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    // ponytail: o ano vai num registrador que pode ter lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    data(d.as_mut_ptr(), dia, g.mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, g.semana);
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, core::ptr::read_unaligned(d.as_ptr() as *const u16));
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, 0);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + 0x28) as *mut u32, p);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// O plano: os campos de `g`, a palavra da data (calculada antes numa Date local pela 0x549f70, sem efeito fora dela),
/// +0xb8 = 0, o prêmio e a devolução 1.
unsafe fn cqj11_confere(nome: &str, this: usize, g: &Cqj11Regras, o: impl FnOnce() -> u32) -> u32 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    data(d.as_mut_ptr(), g.dia, g.mes, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32, g.semana);
    let palavra = core::ptr::read_unaligned(d.as_ptr() as *const u16);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in g.campos {
        let lido = match n {
            1 => *((this + off) as *const u8) as u32,
            2 => core::ptr::read_unaligned((this + off) as *const u16) as u32,
            _ => core::ptr::read_unaligned((this + off) as *const u32),
        };
        let v = if n == 1 { v & 0xff } else if n == 2 { v & 0xffff } else { v };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = core::ptr::read_unaligned((this + 0xb6) as *const u16);
    let b8 = core::ptr::read_unaligned((this + 0xb8) as *const u16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + 0x28) as *const u32) == p);
    PLACAR.registrar(r == 1 && k == 0 && b6 == palavra && b8 == 0 && premio, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {} prêmio ok {}", nome, this, r, &errados[..k], b6, palavra, b8, premio));
    r
}

const CQJ11_LIBERTADORES: Cqj11Regras = Cqj11Regras {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7),
        (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    dia: 0xf, mes: 0, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

const CQJ11_MERCONORTE: Cqj11Regras = Cqj11Regras {
    campos: &[(0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1), (0xbe, 1, 1),
        (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff)],
    dia: 1, mes: 7, semana: 0xffff_ffff, premio: Some(0x1e8480),
};

const CQJ11_OCEANIA: Cqj11Regras = Cqj11Regras {
    campos: &[(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x49, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 8),
        (0xc5, 1, 1), (0xc7, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xd9, 2, 0x28)],
    dia: 0x14, mes: 7, semana: 5, premio: None,
};

/// { 0x4c3890 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `ConmebolLiber::set_up_rules()`: os grupos da Libertadores em dois turnos, 3 pontos a vitória e 1 o empate, os
/// desempates (1, 2, 3, 0), dois classificados por grupo, clubes internacionais (+0x42 = 2), categoria 7, 7 no banco e 3
/// substituições, começo no dia 15 do mês 0 do ano da competição, prêmios de 2.000.000; depois a tabela das rodadas.
unsafe extern "thiscall" fn cqj11_libertadores_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da libertadores");
    let dia = 0xf; // cqj11: o dia do começo da Libertadores
    let vitoria = 3; // cqj11: a vitória nos grupos da Libertadores
    cqj11_regras(this, &CQJ11_LIBERTADORES, dia, vitoria)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj11_libertadores_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1697);
    cqj11_confere("cqj11_libertadores_regras", this, &CQJ11_LIBERTADORES, || o(this))
}

/// { 0x4c1ed0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x8c; quadro SEH só da data local): `ConMercCup::
/// set_up_rules()`: os grupos da Copa Merconorte em dois turnos, 3/1 pontos, desempates (1, 2, 3, 0), um classificado
/// por grupo, clubes internacionais, categoria 7, começo no dia 1 do mês 7, prêmios de 2.000.000.
unsafe extern "thiscall" fn cqj11_merconorte_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da merconorte");
    cqj11_regras(this, &CQJ11_MERCONORTE, CQJ11_MERCONORTE.dia, 3)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj11_merconorte_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1698);
    cqj11_confere("cqj11_merconorte_regras", this, &CQJ11_MERCONORTE, || o(this))
}

/// { 0x7979b0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `OceaniaClubChamp::set_up_rules()`: os grupos do Campeonato de Clubes da Oceania em turno único, 3/1 pontos,
/// desempates (1, 2, 4, 0), dois classificados, categoria 8, 3 no banco e 3 substituições, +0x43 = 2, os bits de regra
/// 0x28 (os de cima classificados, os outros eliminados), começo no dia 20 do mês 7 num dia da semana 5; sem prêmio.
unsafe extern "thiscall" fn cqj11_oceania_clubes_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da oceania");
    let dia = 0x14; // cqj11: o dia do começo na Oceania
    cqj11_regras(this, &CQJ11_OCEANIA, dia, 3)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqj11_oceania_clubes_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1699);
    cqj11_confere("cqj11_oceania_clubes_regras", this, &CQJ11_OCEANIA, || o(this))
}

// ---------- 274a leva: a Libertadores: os 32 classificados, o destino de cada clube (o campeão vai ao Mundial Interclubes) e a fase seguinte (conmebol_liber.cpp) ----------



/// Os clubes da competição da Libertadores na ordem do banco ([0xae23b4], 0x245 bytes, [0xae235c] clubes): os que têm
/// a competição europeia/continental da temporada (+0x1db) igual ao id da competição ([+4]), até `max`.
unsafe fn cqk11_inscritos(this: usize, max: i32, saida: &mut [usize; 32]) -> i32 {
    let mut n = 0i32;
    let mut i = 0i16;
    let mut clube = read_u32_as_usize(0xae23b4);
    if (read_u32_as_usize(0xae235c) as i32) > 0 {
        loop {
            if read_u32_as_usize(clube + 0x1db) == read_u32_as_usize(read_u32_as_usize(this + 4)) && n < max {
                if (n as usize) < saida.len() {
                    saida[n as usize] = clube;
                }
                n += 1;
            }
            i = i.wrapping_add(1);
            clube = clube.wrapping_add(0x245);
            if i as i32 >= read_u32_as_usize(0xae235c) as i32 {
                break;
            }
        }
    }
    n
}

/// { 0x4c3f40 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::select_qualified_clubs()`: +0xf2 = 32 e a lista
/// +0xee (0x944e46(0x80): push 0x80; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x285 e devolve 0) com os
/// clubes, na ordem do banco, cuja competição da temporada (+0x1db) é a Libertadores, até 32; menos de 32, a caixa da
/// linha 0x297 e devolve 0; senão a lista embaralhada (0x4afb40(lista, 32): push n; push lista; add esp, 8 -> cdecl; 3 ×
/// 32 trocas sorteadas) e devolve 1.
unsafe extern "thiscall" fn cqk11_libertadores_classificados(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    let lista = novo(0x80);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, lista as u32);
    if lista == 0 {
        aviso_em(0x9a7de4, 0x285);
        return 0;
    }
    let mut v = [0usize; 32];
    let n = cqk11_inscritos(this, read_u32_as_usize(this + 0xf2) as i32, &mut v);
    for (k, &c) in v.iter().enumerate().take(n.min(32) as usize) {
        core::ptr::write_unaligned((lista + k * 4) as *mut u32, c as u32); // cqk11: o clube na ordem do banco
    }
    if n != read_u32_as_usize(this + 0xf2) as i32 {
        aviso_em(0x9a7de4, 0x297);
        return 0;
    }
    crate::comparacao::ramo("cqk_11 classificados da libertadores");
    embaralhar(lista, read_i16(this + 0xf2) as u16 as u32); // cqk11: os 32 da Libertadores embaralhados
    1
}

/// Comparador da 0x4c3f40: lado a lado com as duas listas (a original aloca a dela, a cópia a sua): a original roda e a
/// lista dela fica; o estado do sorteador volta, a cópia roda sem contar e a lista dela é comparada com a da original e
/// liberada (0x9452ca: push; add esp, 4 -> cdecl). Com menos de 32 inscritos (as caixas): só a original.
unsafe extern "thiscall" fn comparar_cqk11_libertadores_classificados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1706);
    let mut v = [0usize; 32];
    if cqk11_inscritos(this, 0x20, &mut v) != 0x20 {
        return o(this); // ponytail: as caixas de erro: só a original
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (ee, f2) = (read_u32_as_usize(this + 0xee), read_u32_as_usize(this + 0xf2));
    let r = o(this);
    let (la, depois) = (read_u32_as_usize(this + 0xee), crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO));
    let a: [u32; 32] = if la != 0 { core::ptr::read_unaligned(la as *const [u32; 32]) } else { [0; 32] };
    estado.escrever();
    core::ptr::write_unaligned((this + 0xee) as *mut u32, ee as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, f2 as u32);
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cqk11_libertadores_classificados(this)));
    let lb = read_u32_as_usize(this + 0xee);
    let b: [u32; 32] = if lb != 0 { core::ptr::read_unaligned(lb as *const [u32; 32]) } else { [0; 32] };
    let igual_estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois;
    if lb != 0 && lb != la {
        soltar(lb);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut u32, la as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    depois.escrever();
    PLACAR.registrar(r == rr && a == b && igual_estado, || format!("cqk11_libertadores_classificados({:#x}) devolveu {} x {} estado igual {} original {:x?} rust {:x?}", this, r, rr, igual_estado, a, b));
    r
}

/// { 0x4c4080 } (thiscall, ret 0x18, conmebol_liber.cpp; a virtual +0x44): `ConmebolLiber::
/// update_staff_records_for_status(clube, desfecho, fase, _, jogo, a6)`; devolve 0. Fase (byte com sinal) abaixo de 7,
/// os grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x14, peso 0x28); 0xfe, a não classificação (0x86c1d0:
/// 0x41c, 0x28). Fase 7, o mata-mata: peso 0x4bf850(0x32, 0x64, rodada do jogo +0x34, 4); 0xff, a eliminação (0x86c000:
/// +0x32, +0x30 do jogo, peso); 0, a classificação com +0x32 e o +0x6f da rodada do jogo na tabela de rodadas da fase 7
/// ([+0xc] + 0x1c, +0xa3, 0x68 bytes; os 16 bits altos que a original empurra são os do ponteiro da fase); 1, o registro
/// de campeão (0x86a800) e o clube vai ao Mundial Interclubes (o objeto [[0xadadfc] + 4 × [0x9cf6e4]]): na vaga 0 da
/// tabela +0xa7 dele se o clube que lá está é do continente [0x9cfa1c] (a nação +0x53, o continente +0x71 da nação, +0
/// o id), senão na vaga 1; depois o arquivo da temporada das estatísticas dele (0x4a1c50 em +8: push 1 -> thiscall, ret
/// 4) e as posições previstas (virtual +0x5c); 2, o registro de vice (0x86b0b0).
unsafe extern "thiscall" fn cqk11_libertadores_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    let final_ = |g: usize| {
        let h: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(g);
        h(cpz4_registros(), clube, jogo, a6);
    };
    if f < 7 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x14, 0x28);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x41c, 0x28);
        }
    } else if f == 7 {
        match d {
            0xff => {
                let peso = cpz4_escala(0x32, 0x64, read_i16(jogo + 0x34), 4);
                // ponytail: +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
                cpz4_eliminado(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
            }
            0 => {
                let peso = cpz4_escala(0x32, 0x64, read_i16(jogo + 0x34), 4);
                let mata = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0x1c);
                let r = read_i16(jogo + 0x34) as isize;
                let rodada = (mata as u32 & 0xffff_0000) | core::ptr::read_unaligned((read_u32_as_usize(mata + 0xa3).wrapping_add((r * 0x68) as usize) + 0x6f) as *const u16) as u32;
                cpz4_classificado(clube, comp, core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, rodada, peso);
            }
            1 => {
                final_(0x86a800);
                let mundial = read_u32_as_usize(read_u32_as_usize(0xadadfc).wrapping_add(read_u32_as_usize(0x9cf6e4).wrapping_mul(4)));
                let t = read_u32_as_usize(mundial + 0xa7);
                let nacao = read_u32_as_usize(read_u32_as_usize(t) + 0x53);
                crate::comparacao::ramo("cqk_11 campeao da libertadores");
                if nacao != 0 && read_u32_as_usize(read_u32_as_usize(nacao + 0x71)) == read_u32_as_usize(0x9cfa1c) {
                    core::ptr::write_unaligned(t as *mut u32, clube as u32); // cqk11: o campeão na vaga sul-americana
                } else {
                    core::ptr::write_unaligned((t + 6) as *mut u32, clube as u32);
                }
                let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
                arquivar(read_u32_as_usize(mundial + 8), 1);
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(mundial) + 0x5c));
                prever(mundial);
            }
            2 => final_(0x86b0b0),
            _ => {}
        }
    }
    0
}

// ponytail: os registros (histórico, convocações), o arquivo das estatísticas e as posições previstas do Mundial não
// rodam duas vezes. Só a original; o plano: a devolução 0 e, no título (fase 7, desfecho 1), o campeão na vaga do
// Mundial Interclubes que a regra escolhe (0 se lá está um clube do continente [0x9cfa1c], senão 1).
unsafe extern "thiscall" fn comparar_cqk11_libertadores_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1707);
    let titulo = fase as u8 == 7 && desfecho as u8 == 1;
    let mut vaga = 0usize;
    if titulo {
        let mundial = read_u32_as_usize(read_u32_as_usize(0xadadfc).wrapping_add(read_u32_as_usize(0x9cf6e4).wrapping_mul(4)));
        let t = read_u32_as_usize(mundial + 0xa7);
        let nacao = read_u32_as_usize(read_u32_as_usize(t) + 0x53);
        vaga = if nacao != 0 && read_u32_as_usize(read_u32_as_usize(nacao + 0x71)) == read_u32_as_usize(0x9cfa1c) { t } else { t + 6 };
    }
    let r = o(this, clube, desfecho, fase, a4, jogo, a6);
    let igual = r == 0 && (!titulo || read_u32_as_usize(vaga) == clube);
    PLACAR.registrar(igual, || format!("cqk11_libertadores_registros({:#x}, {}, {}) devolveu {} titulo={} vaga {:#x}", clube, desfecho as u8, fase as u8, r, titulo, vaga));
    r
}

/// { 0x4c3a80 } (thiscall, ret 0, conmebol_liber.cpp; a virtual +0x28): `ConmebolLiber::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x1f6; senão +0x30 sobe e, chegando a 7 (o fim dos sete grupos), o
/// mata-mata (0x4c3d10: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqk11_libertadores_fase_seguinte(this: usize) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(0x9a7de4, 0x1f6);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 7 { // cqk11: o mata-mata depois dos sete grupos
        crate::comparacao::ramo("cqk_11 mata-mata da libertadores");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c3d10usize);
        mata(this);
    }
}

// ponytail: o mata-mata (0x4c3d10) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
unsafe extern "thiscall" fn comparar_cqk11_libertadores_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1708);
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("cqk11_libertadores_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

