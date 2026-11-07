// ---------- 284a leva: a Liga dos Campeões asiática: a fase seguinte, os dois grupos e as semifinais (asia_club_champ.cpp) ----------

unsafe fn cqt12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// { 0x40c070 } (thiscall, ret 0, asia_club_champ.cpp; a virtual +0x28): `AsiaClubChamp::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x355; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), os dois grupos (0x40c110), chegando a 3 (depois dos grupos), as semifinais (0x40c3d0); as duas mov
/// ecx, this -> thiscall, ret 0.
unsafe extern "thiscall" fn cqt12_asia_fase_seguinte(this: usize) {
    let f = cqt12_d(this + 0x30) as i32;
    if f >= (cqt12_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x987a40, 0x355);
    }
    let f = f + 1;
    cqt12_pd(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_12 grupos da asia");
        let grupos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x40c110usize);
        grupos(this);
    } else if f == 3 {
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x40c3d0usize);
        semifinais(this);
    }
}

// ponytail: os grupos e as semifinais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um
// (os grupos gravam 2, o mesmo valor) fora da última fase, que dá a caixa e não muda nada.
unsafe extern "thiscall" fn comparar_cqt12_asia_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2065);
    let f = cqt12_d(this + 0x30) as i32;
    let ultima = f >= (cqt12_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt12_d(this + 0x30) as i32;
    let esperado = if ultima { f } else if f + 1 == 1 { 2 } else { f + 1 };
    PLACAR.registrar(depois == esperado, || format!("cqt12_asia_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 8 dos grupos: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da copa, +0xa7) até 4
/// e depois os do mata-mata da segunda zona (a tabela +0xa7 da fase [+0xc][0]) até 8; as duas buscas vão até o +0x3e da
/// copa (palavra com sinal). Devolve os 8 (as vagas que sobrarem ficam 0: a original as deixa com sobra da pilha).
unsafe fn cqt12_vivos(this: usize) -> [usize; 8] {
    let mut v = [0usize; 8];
    let mut n = 0usize;
    let times = cqt12_w(this + 0x3e);
    let mut i = 0i16;
    while i < times && n < 4 {
        let e = cqt12_d(this + 0xa7) + i as usize * 6;
        if *((e + 5) as *const u8) == 1 {
            v[n] = cqt12_d(e); // cqt12: um vivo da primeira zona
            n += 1;
        }
        i += 1;
    }
    let mut i = 0i16;
    while i < times && n < 8 {
        let e = cqt12_d(cqt12_d(cqt12_d(this + 0xc)) + 0xa7) + i as usize * 6;
        if *((e + 5) as *const u8) == 1 {
            v[n] = cqt12_d(e);
            n += 1;
        }
        i += 1;
    }
    v
}

/// Um grupo: a tabela das rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push `g`; mov ecx, this ->
/// thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall,
/// ret 0x58, 22 argumentos: a copa, 4, os 4 times, 1, +4, a tabela das rodadas, as rodadas (word), 3, 1, 8, &{1, 2, 0,
/// 0}, &{2, 0, 0, 0}, o ano +0x40 (word), `g`, o id (word), +0x51, `a17`, 0, 0x28, −1, 0, 2; a original empurra os words
/// e bytes com lixo nos bytes altos, que o construtor não lê) em [+0xc][`g`]; a tabela liberada (0x9452ca). Devolve a fase.
unsafe fn cqt12_grupo(this: usize, g: u32, times: *const usize, a17: u32) -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let regras_a: [u8; 4] = [1, 2, 0, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt12_d(cqt12_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            4,
            times,
            1,
            cqt12_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            3,
            1,
            8,
            regras_a.as_ptr(),
            regras_b.as_ptr(),
            cqt12_w(this + 0x40) as u16 as u32,
            g,
            id as u16 as u32,
            *((this + 0x51) as *const u8) as u32,
            a17,
            0,
            0x28,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqt12_pd(cqt12_d(this + 0xc) + g as usize * 4, fase);
    soltar(tabela);
    cqt12_d(cqt12_d(this + 0xc) + g as usize * 4)
}

/// { 0x40c110 } (thiscall, ret 0, asia_club_champ.cpp; quadro SEH só para o new das fases: sem destrutor no caminho
/// normal): `AsiaClubChamp::create_group_stages()`: os 8 vivos das duas zonas (cqt12_vivos); o grupo 1 (fase 1, os 4
/// primeiros, o 17º argumento 2; sem fase, a caixa da linha 0x39e e volta) e a notícia dele (0x684230 na fase, thiscall,
/// ret 0); o grupo 2 (fase 2, os 4 seguintes, o 17º argumento 1; sem fase, a caixa da linha 0x3b4 e volta), a notícia e
/// +0x30 = 2.
unsafe extern "thiscall" fn cqt12_asia_grupos(this: usize) {
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    let v = cqt12_vivos(this);
    let grupo_1 = [v[0], v[1], v[2], v[3]]; // cqt12: o grupo 1
    let f = cqt12_grupo(this, 1, grupo_1.as_ptr(), 2);
    if f == 0 {
        return aviso_em(0x987a40, 0x39e);
    }
    noticia(f);
    crate::comparacao::ramo("cqt_12 dois grupos da asia");
    let grupo_2 = [v[4], v[5], v[6], v[7]]; // cqt12: o grupo 2
    let f = cqt12_grupo(this, 2, grupo_2.as_ptr(), 1);
    if f == 0 {
        return aviso_em(0x987a40, 0x3b4);
    }
    noticia(f);
    cqt12_pd(this + 0x30, 2);
}

// ponytail: as fases são alocadas e as notícias vão aos técnicos: não roda duas vezes. Só a original; o plano: os vivos
// lidos antes; depois as fases [+0xc][1] e [+0xc][2] com 4 times, o ano da copa e os 4 vivos de cada (em qualquer ordem:
// o construtor ordena a tabela) e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqt12_asia_grupos(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2066);
    if cqt12_d(this + 0xc) == 0 || cqt12_d(cqt12_d(this + 0xc)) == 0 {
        return o(this);
    }
    let v = cqt12_vivos(this);
    o(this);
    let confere = |g: usize, de: &[usize]| {
        let f = cqt12_d(cqt12_d(this + 0xc) + g * 4);
        let t = if f != 0 { cqt12_d(f + 0xb1) } else { 0 };
        if t == 0 || cqt12_w(f + 0x3e) != 4 || cqt12_w(f + 0x40) != cqt12_w(this + 0x40) {
            return false;
        }
        let mut a: Vec<usize> = (0..4).map(|k| cqt12_d(t + k * 0x3b)).collect();
        let mut b = de.to_vec();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    let ok = confere(1, &v[..4]) && confere(2, &v[4..]) && cqt12_d(this + 0x30) == 2;
    PLACAR.registrar(ok, || format!("cqt12_asia_grupos({:#x}): os grupos não batem com os vivos {:x?}", this, v));
}

/// Os 4 das semifinais, na ordem da lista: o 1º do grupo 1 ([+0xc][1], tabela +0xb1), o 2º do grupo 2 ([+0xc][2]), o
/// 1º do grupo 2 e o 2º do grupo 1 (os cruzamentos: as chaves são os pares da lista).
unsafe fn cqt12_semifinalistas(this: usize) -> [usize; 4] {
    let l = |g: usize, p: usize| cqt12_d(cqt12_d(cqt12_d(cqt12_d(this + 0xc) + g * 4) + 0xb1) + p * 0x3b);
    [l(1, 0), l(2, 1), l(2, 0), l(1, 1)] // cqt12: 1º do 1 × 2º do 2, 1º do 2 × 2º do 1
}

/// { 0x40c3d0 } (thiscall, ret 0, asia_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal):
/// `AsiaClubChamp::create_semi_final_stage()`: a lista dos 4 (0x944e46(0x10): push; add esp, 4 -> cdecl; sem memória, a
/// caixa da linha 0x3d1 e volta), os cruzamentos de cqt12_semifinalistas; a tabela das rodadas pela virtual +0x3c (push
/// 0; push &id; push &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 ->
/// cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano +0x40,
/// 3, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos,
/// que o construtor não lê) em [+0xc][3]; a tabela liberada (0x9452ca). Sem fase: a caixa da linha 0x3e6 e volta (a lista
/// não é liberada, como na original). Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round: push 0; mov ecx, fase ->
/// thiscall, ret 4) e a lista liberada.
unsafe extern "thiscall" fn cqt12_asia_semifinais(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(0x987a40, 0x3d1);
    }
    crate::comparacao::ramo("cqt_12 cruzamentos da asia");
    for (k, &c) in cqt12_semifinalistas(this).iter().enumerate() {
        cqt12_pd(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt12_d(cqt12_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 3, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqt12_d(this + 4), tabela, cqt12_w(this + 0x40) as u16 as u32, 3, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt12_pd(cqt12_d(this + 0xc) + 0xc, fase);
    soltar(tabela);
    let f = cqt12_d(cqt12_d(this + 0xc) + 0xc);
    if f == 0 {
        return aviso_em(0x987a40, 0x3e6);
    }
    sortear(f, 0);
    soltar(lista);
}

// ponytail: aloca a fase e sorteia a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][3] com 4
// times, o +4 da copa e os 4 cruzados (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio pode
// reordenar).
unsafe extern "thiscall" fn comparar_cqt12_asia_semifinais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2067);
    let fases = cqt12_d(this + 0xc);
    if fases == 0 || cqt12_d(fases + 4) == 0 || cqt12_d(fases + 8) == 0 {
        return o(this);
    }
    let mut plano = cqt12_semifinalistas(this).to_vec();
    o(this);
    let fase = cqt12_d(cqt12_d(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt12_asia_semifinais({:#x}) sem fase", this));
        return;
    }
    let lista = cqt12_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqt12_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt12_w(fase + 0x3e) == 4 && cqt12_d(fase + 4) == cqt12_d(this + 4);
    PLACAR.registrar(igual, || format!("cqt12_asia_semifinais({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

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

// ---------- 284a leva: os mata-matas da Copa Africana de Nações e da Recopa asiática: as quartas, a fase seguinte e as finais (african_nations.cpp, asia_cup_winner.cpp) ----------

unsafe fn cqt13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O grupo de onde sai cada vaga das quartas da Copa Africana (−1 = o grupo A, a tabela da própria copa; 0..2 = as fases
/// dos grupos B a D) e a posição: as chaves são os pares da lista.
const CQT13_QUARTAS: [(i32, i32); 8] = [
    (-1, 0), (0, 1), // cqt13: 1º A × 2º B
    (1, 0), (2, 1), // cqt13: 1º C × 2º D
    (1, 1), (2, 0), // cqt13: 2º C × 1º D
    (0, 0), (-1, 1), // cqt13: 1º B × 2º A
];

/// O clube da posição `p` do grupo `g` (pela 0x6826d0 League::get_table_row: push p; mov ecx, grupo -> thiscall, ret 4).
unsafe fn cqt13_do_grupo(this: usize, g: i32, p: i32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let grupo = if g < 0 { this } else { cqt13_d(cqt13_d(this + 0xc) + g as usize * 4) };
    cqt13_d(linha(grupo, p))
}

/// { 0x402090 } (thiscall, ret 0, african_nations.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AfricanNations::create_knockout_stage()`, as quartas da Copa Africana (da fase seguinte 0x401e10, ao chegar
/// a 3): a lista dos 8 (0x944e46(0x20): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x311 e volta) pelos
/// cruzamentos de CQT13_QUARTAS, na ordem das chamadas; a tabela das rodadas pela virtual +0x3c (push 0; push &id; push
/// &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 8, a lista, as rodadas, +4, a tabela, o ano +0x40, 3, 4, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][3]; a lista e a tabela liberadas (0x9452ca). Sem fase: a caixa da linha 0x32c e volta.
/// Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall, ret 4) e +0x30 = 3.
unsafe extern "thiscall" fn cqt13_africana_quartas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    let lista = alocar(0x20);
    if lista == 0 {
        return aviso_em(0x9870f8, 0x311);
    }
    crate::comparacao::ramo("cqt_13 quartas da copa africana");
    for (k, &(g, p)) in CQT13_QUARTAS.iter().enumerate() {
        cqt13_pd(lista + k * 4, cqt13_do_grupo(this, g, p));
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt13_d(cqt13_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 3, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 8, lista, rodadas as u16 as u32, cqt13_d(this + 4), tabela, cqt13_w(this + 0x40) as u16 as u32, 3, 4, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt13_pd(cqt13_d(this + 0xc) + 0xc, fase);
    soltar(lista);
    soltar(tabela);
    let f = cqt13_d(cqt13_d(this + 0xc) + 0xc);
    if f == 0 {
        return aviso_em(0x9870f8, 0x32c);
    }
    montar(f, 0);
    cqt13_pd(this + 0x30, 3);
}

// ponytail: aloca a fase e monta a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][3] com 8 times,
// o +4 da copa e os 8 cruzados (lidos antes pelas tabelas, +0xb1 de cada grupo) na lista dela (+0xa7, 6 bytes; como
// conjunto) e +0x30 = 3.
unsafe extern "thiscall" fn comparar_cqt13_africana_quartas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2071);
    let fases = cqt13_d(this + 0xc);
    if fases == 0 || (0..3).any(|g| cqt13_d(fases + g * 4) == 0) || cqt13_d(this + 0xb1) == 0 {
        return o(this);
    }
    let tabela = |g: i32| if g < 0 { cqt13_d(this + 0xb1) } else { cqt13_d(cqt13_d(fases + g as usize * 4) + 0xb1) };
    let mut plano: Vec<usize> = CQT13_QUARTAS.iter().map(|&(g, p)| cqt13_d(tabela(g) + p as usize * 0x3b)).collect();
    o(this);
    let fase = cqt13_d(cqt13_d(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt13_africana_quartas({:#x}) sem fase", this));
        return;
    }
    let lista = cqt13_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| cqt13_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt13_w(fase + 0x3e) == 8 && cqt13_d(fase + 4) == cqt13_d(this + 4) && cqt13_d(this + 0x30) == 3;
    PLACAR.registrar(igual, || format!("cqt13_africana_quartas({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// { 0x40dfa0 } (thiscall, ret 0, asia_cup_winner.cpp; a virtual +0x28): `AsiaCupWinner::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x328; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), as finais (0x40e030: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqt13_recopa_asiatica_fase_seguinte(this: usize) {
    let f = cqt13_d(this + 0x30) as i32;
    if f >= (cqt13_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x987b60, 0x328);
    }
    let f = f + 1;
    cqt13_pd(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_13 finais da recopa asiatica");
        let finais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x40e030usize);
        finais(this);
    }
}

// ponytail: as finais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um (fora da última
// fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn comparar_cqt13_recopa_asiatica_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2072);
    let f = cqt13_d(this + 0x30) as i32;
    let ultima = f >= (cqt13_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt13_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqt13_recopa_asiatica_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 4 finalistas da Recopa asiática: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da
/// copa, +0xa7) e depois os da segunda (a tabela +0xa7 da fase [+0xc][0]), até 4 ao todo, as duas buscas até o +0x3e da
/// copa (palavra com sinal); o 2º e o 3º trocam de lugar (os cruzamentos: as chaves são os pares da lista). As vagas
/// que sobrarem ficam 0 (a original as deixa com o que o malloc deu).
unsafe fn cqt13_finalistas(this: usize) -> [usize; 4] {
    let mut v = [0usize; 4];
    let mut n = 0i16;
    let times = cqt13_w(this + 0x3e);
    for tabela in [cqt13_d(this + 0xa7), cqt13_d(cqt13_d(cqt13_d(this + 0xc)) + 0xa7)] {
        let mut i = 0i16;
        while i < times && n < 4 {
            let e = tabela + i as usize * 6;
            if *((e + 5) as *const u8) == 1 {
                v[n as usize] = cqt13_d(e);
                n += 1;
            }
            i += 1;
        }
    }
    v.swap(1, 2); // cqt13: cruza o par do meio
    v
}

/// { 0x40e030 } (thiscall, ret 0, asia_cup_winner.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AsiaCupWinner::create_finals_stage()`: a lista dos 4 (0x944e46(0x10): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x34b e volta) de cqt13_finalistas; a tabela das rodadas pela virtual +0x3c (push 0; push
/// &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 1, 2, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][1]; a tabela liberada (0x9452ca). Sem fase: a caixa da linha 0x375 e volta (a lista não
/// é liberada, como na original). Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round: push 0; mov ecx, fase ->
/// thiscall, ret 4) e a lista liberada.
unsafe extern "thiscall" fn cqt13_recopa_asiatica_finais(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(0x987b60, 0x34b);
    }
    crate::comparacao::ramo("cqt_13 finalistas da recopa asiatica");
    // ponytail: com menos de 4 vivos a original deixa nas vagas o que o malloc deu; a cópia grava 0 (nesta carreira são 4)
    for (k, &c) in cqt13_finalistas(this).iter().enumerate() {
        cqt13_pd(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt13_d(cqt13_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqt13_d(this + 4), tabela, cqt13_w(this + 0x40) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt13_pd(cqt13_d(this + 0xc) + 4, fase);
    soltar(tabela);
    let f = cqt13_d(cqt13_d(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0x987b60, 0x375);
    }
    sortear(f, 0);
    soltar(lista);
}

// ponytail: aloca a fase e sorteia a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com 4
// times, o +4 da copa e os 4 finalistas (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto).
unsafe extern "thiscall" fn comparar_cqt13_recopa_asiatica_finais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2073);
    if cqt13_d(this + 0xa7) == 0 || cqt13_d(this + 0xc) == 0 || cqt13_d(cqt13_d(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = cqt13_finalistas(this).to_vec();
    o(this);
    let fase = cqt13_d(cqt13_d(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt13_recopa_asiatica_finais({:#x}) sem fase", this));
        return;
    }
    let lista = cqt13_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqt13_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt13_w(fase + 0x3e) == 4 && cqt13_d(fase + 4) == cqt13_d(this + 4);
    PLACAR.registrar(igual, || format!("cqt13_recopa_asiatica_finais({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

// ---------- 284a leva: a Copa da Itália e a Copa da Itália da Série C: a tabela do grupo 1 (o próprio objeto da copa) e a fase seguinte (o mata-mata depois do último grupo) (ita_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqr7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A tabela do grupo 1 de uma copa italiana (`create_first_group_table`; a copa é ela mesma a liga do grupo 1): +0x3e =
/// `n` times; a tabela (0x944e46 de n × 0x3b, cdecl) em +0xb1; sem memória, a caixa da linha `linha` e 0. Cada um dos n
/// primeiros inscritos (+0xee, vetor de ponteiros) vai à linha i da tabela (League::init_table_row 0x687430: push 0; push
/// 0; push clube; push i (a original empurra o dword do byte local, com lixo da pilha em cima; a 0x687430 só lê o byte);
/// mov ecx, copa -> thiscall, ret 0x10) e a vaga dele em +0xee é zerada (sai da lista dos que esperam a próxima fase); o
/// índice é um byte com sinal comparado à palavra +0x3e. Depois a notícia do sorteio do grupo (League::send_group_draw_news
/// 0x684230, mov ecx -> thiscall, ret 0). Devolve 1.
unsafe fn cqr7_grupo_um(this: usize, n: i16, arquivo: usize, linha: u32, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(n as i32 as u32 * 0x3b);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(ramo);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut i: i8 = 0;
    while (i as i16) < cqr7_w(this + 0x3e) {
        let vaga = cqr7_d(this + 0xee) + i as isize as usize * 4;
        iniciar(this, i as u8 as u32, cqr7_d(vaga), 0, 0);
        core::ptr::write_unaligned((cqr7_d(this + 0xee) + i as isize as usize * 4) as *mut u32, 0); // cqr7: a vaga do grupo 1 sai da lista
        i = i.wrapping_add(1);
    }
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    noticia(this);
    1
}

/// Só a original (ponytail: a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos; não
/// pode rodar duas vezes), conferindo o plano feito antes: devolve 1, +0x3e = n, a tabela em +0xb1 com os n primeiros
/// inscritos nas linhas (de 0x3b bytes), na ordem, e as n primeiras vagas de +0xee zeradas.
unsafe fn cqr7_comparar_grupo_um(nome: &str, this: usize, n: i16, original: impl FnOnce() -> u32) -> u32 {
    let plano: Vec<usize> = (0..n as usize).map(|k| cqr7_d(cqr7_d(this + 0xee) + k * 4)).collect();
    let r = original();
    let t = cqr7_d(this + 0xb1);
    let ok = r == 1 && cqr7_w(this + 0x3e) == n && t != 0
        && plano.iter().enumerate().all(|(k, &c)| cqr7_d(t + k * 0x3b) == c && cqr7_d(cqr7_d(this + 0xee) + k * 4) == 0);
    PLACAR.registrar(ok, || format!("{}({:#x}) devolveu {}; plano {:x?}", nome, this, r, plano));
    r
}

/// { 0x63f250 } (thiscall, ret 0, ita_cup.cpp; do construtor): `ItaCup::create_first_group_table()`: o grupo 1 da Copa
/// da Itália com 4 times (`cqr7_grupo_um`; a caixa do ita_cup.cpp na linha 0x19e).
unsafe extern "thiscall" fn cqr7_copa_da_italia_grupo_um(this: usize) -> u32 {
    cqr7_grupo_um(this, 4, 0x9ea23c, 0x19e, "cqr_7 grupo 1 da copa da italia")
}

unsafe extern "thiscall" fn comparar_cqr7_copa_da_italia_grupo_um(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2074);
    cqr7_comparar_grupo_um("cqr7_copa_da_italia_grupo_um", this, 4, || o(this))
}

/// { 0x63d5e0 } (thiscall, ret 0, ita_c_cup.cpp; do construtor): `ItaCCup::init_first_group()`: o grupo 1 da Copa da
/// Itália da Série C com 5 times (`cqr7_grupo_um`; a caixa do ita_c_cup.cpp na linha 0x1a0).
unsafe extern "thiscall" fn cqr7_copa_da_serie_c_grupo_um(this: usize) -> u32 {
    cqr7_grupo_um(this, 5, 0x9ea204, 0x1a0, "cqr_7 grupo 1 da copa da serie c") // cqr7: 5 times no grupo 1 da Série C
}

unsafe extern "thiscall" fn comparar_cqr7_copa_da_serie_c_grupo_um(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2075);
    cqr7_comparar_grupo_um("cqr7_copa_da_serie_c_grupo_um", this, 5, || o(this))
}

/// A fase seguinte de uma copa italiana (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal), a caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a `ultimo_grupo` + 1 (o fim dos grupos), monta o
/// mata-mata (`mata_mata`, call direto com o ecx da copa -> thiscall, ret 0).
unsafe fn cqr7_fase_seguinte(this: usize, arquivo: usize, linha: u32, fim_dos_grupos: i32, mata_mata: usize) {
    let fase = cqr7_d(this + 0x30) as i32;
    if fase >= (cqr7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == fim_dos_grupos {
        crate::comparacao::ramo("cqr_7 mata-mata das copas italianas");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this); // cqr7: monta o mata-mata
    }
}

/// O comparador da fase seguinte: só a original (ponytail: o mata-mata aloca a fase e sorteia; não pode rodar duas vezes),
/// conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
unsafe fn cqr7_comparar_fase(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqr7_d(this + 0x30) as i32;
    let ultima = antes >= (cqr7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqr7_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) fase {} -> {}", nome, this, antes, depois));
}

/// { 0x63f5b0 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x28): `ItaCup::advance_stage()`: a fase seguinte
/// (`cqr7_fase_seguinte`; a caixa do ita_cup.cpp na linha 0x23e); depois da fase 6 (os 8 grupos: o grupo 1 e as fases
/// 0..6), o mata-mata da Copa da Itália (ItaCup::create_knockout_stage 0x63f870).
unsafe extern "thiscall" fn cqr7_copa_da_italia_fase_seguinte(this: usize) {
    cqr7_fase_seguinte(this, 0x9ea23c, 0x23e, 7, 0x63f870) // cqr7: o mata-mata da Copa da Itália na fase 7
}

unsafe extern "thiscall" fn comparar_cqr7_copa_da_italia_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2076);
    cqr7_comparar_fase("cqr7_copa_da_italia_fase_seguinte", this, || o(this))
}

/// { 0x63d950 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x28): `ItaCCup::advance_stage()`: a fase seguinte
/// (`cqr7_fase_seguinte`; a caixa do ita_c_cup.cpp na linha 0x240); depois da fase 14 (os 16 grupos), o mata-mata da Copa
/// da Série C (ItaCCup::create_knockout_stage 0x63dc10).
unsafe extern "thiscall" fn cqr7_copa_da_serie_c_fase_seguinte(this: usize) {
    cqr7_fase_seguinte(this, 0x9ea204, 0x240, 0xf, 0x63dc10)
}

unsafe extern "thiscall" fn comparar_cqr7_copa_da_serie_c_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2077);
    cqr7_comparar_fase("cqr7_copa_da_serie_c_fase_seguinte", this, || o(this))
}

// ---------- 284a leva: a Copa da Itália e a Copa da Itália da Série C: a fase de cada grupo (os times tirados da lista dos inscritos, as rodadas e a liga do grupo) (ita_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqr8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqr8_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A fase do grupo `g` de uma copa italiana (`create_group_stage(g)`, quadro SEH só para o new: sem destrutor no caminho
/// normal; o molde da Copa da Itália e da Série C, conferido instrução a instrução: só mudam o fonte e as linhas das
/// caixas). A tabela das rodadas pelo método +0x3c da copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall,
/// ret 0x10); um vetor de +0x3e ponteiros (0x944e46, cdecl; sem memória, a caixa da `linhas[0]` e 0) com os primeiros
/// +0x3e inscritos ainda na lista (+0xee, vetor de ponteiros: os não nulos, na ordem, cada um zerado ao sair); a fase
/// (0x944cf1(0xee), cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (word
/// +0x3e), o vetor, 1, +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40
/// (word), g, o id (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da `linhas[1]` e 0. Com ela: a notícia do sorteio do grupo (0x684230
/// na fase), +0x30 = g e 1.
unsafe fn cqr8_fase_do_grupo(this: usize, g: u32, arquivo: usize, linhas: [u32; 2], ramo: &'static str) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr8_d(cqr8_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let times = alocar(((cqr8_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    if cqr8_w(this + 0x3e) > 0 {
        let mut k = 0i32;
        let mut o = 0usize;
        loop {
            let c = cqr8_d(cqr8_d(this + 0xee) + o);
            if c != 0 {
                crate::comparacao::ramo(ramo);
                cqr8_pd(times + k as usize * 4, c); // cqr8: o time na vaga k do grupo
                cqr8_pd(cqr8_d(this + 0xee) + o, 0);
                k += 1;
            }
            o += 4;
            if k >= cqr8_w(this + 0x3e) as i32 {
                break;
            }
        }
    }
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            cqr8_w(this + 0x3e) as u16 as u32,
            times,
            1,
            cqr8_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            cqr8_b(this + 0xc2) as u32,
            cqr8_b(this + 0xc3) as u32,
            cqr8_b(this + 0xc4) as u32,
            this + 0xc5,
            this + 0xbe,
            cqr8_w(this + 0x40) as u16 as u32,
            g,
            id as u16 as u32,
            cqr8_b(this + 0x51) as u32,
            1,
            0,
            cqr8_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr8_pd(cqr8_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr8_d(cqr8_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    noticia(f);
    cqr8_pd(this + 0x30, g as usize);
    1
}

/// Só a original (aloca a fase, que aloca a tabela dela, e manda a notícia; não pode rodar duas vezes), conferindo o plano
/// feito antes: devolve 1 com a fase em [+0xc][g] e +0x30 = g; a fase com os times (+0x3e) e o ano (+0x40) da copa; os
/// primeiros +0x3e inscritos não nulos de +0xee zerados e, na tabela da fase (+0xb1, linhas de 0x3b bytes), esses clubes
/// (em qualquer ordem: a fase ordena a tabela dela [inferência, como na fase dos grupos da Copa do Mundo]).
unsafe fn cqr8_comparar(nome: &str, this: usize, g: u32, original: impl FnOnce() -> u32) -> u32 {
    let n = cqr8_w(this + 0x3e).max(0) as usize;
    let mut plano = Vec::new();
    let mut vagas = Vec::new();
    let mut o = 0usize;
    while plano.len() < n && o < 0x400 {
        let c = cqr8_d(cqr8_d(this + 0xee) + o);
        if c != 0 {
            plano.push(c);
            vagas.push(o);
        }
        o += 4;
    }
    let r = original();
    let mut ok = r == 1;
    if ok {
        let f = cqr8_d(cqr8_d(this + 0xc) + g as usize * 4);
        let t = cqr8_d(f + 0xb1);
        ok = f != 0 && cqr8_d(this + 0x30) == g as usize && cqr8_w(f + 0x3e) == cqr8_w(this + 0x3e) && cqr8_w(f + 0x40) == cqr8_w(this + 0x40)
            && vagas.iter().all(|&o| cqr8_d(cqr8_d(this + 0xee) + o) == 0)
            && (t == 0 || {
                let mut a: Vec<usize> = (0..plano.len()).map(|k| cqr8_d(t + k * 0x3b)).collect();
                let mut b = plano.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
    }
    PLACAR.registrar(ok, || format!("{}({:#x}, {}) devolveu {}; plano {:x?}", nome, this, g, r, plano));
    r
}

/// { 0x63f640 } (thiscall, ret 4, ita_cup.cpp): `ItaCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// (`cqr8_fase_do_grupo`; as caixas do ita_cup.cpp nas linhas 0x263 e 0x27c).
unsafe extern "thiscall" fn cqr8_copa_da_italia_grupo(this: usize, g: u32) -> u32 {
    cqr8_fase_do_grupo(this, g, 0x9ea23c, [0x263, 0x27c], "cqr_8 time do grupo da copa da italia")
}

unsafe extern "thiscall" fn comparar_cqr8_copa_da_italia_grupo(this: usize, g: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(2078);
    cqr8_comparar("cqr8_copa_da_italia_grupo", this, g, || o(this, g))
}

/// { 0x63d9e0 } (thiscall, ret 4, ita_c_cup.cpp): `ItaCCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// da Série C (`cqr8_fase_do_grupo`; as caixas do ita_c_cup.cpp nas linhas 0x265 e 0x27e).
unsafe extern "thiscall" fn cqr8_copa_da_serie_c_grupo(this: usize, g: u32) -> u32 {
    cqr8_fase_do_grupo(this, g, 0x9ea204, [0x265, 0x27e], "cqr_8 time do grupo da copa da serie c")
}

unsafe extern "thiscall" fn comparar_cqr8_copa_da_serie_c_grupo(this: usize, g: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(2079);
    cqr8_comparar("cqr8_copa_da_serie_c_grupo", this, g, || o(this, g))
}

// ---------- 284a leva: a Copa da Itália e a Copa da Itália da Série C: o mata-mata depois dos grupos (os campeões dos grupos, os melhores segundos e os cabeças de chave que entram direto) (ita_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqr9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr9_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A tabela (+0xb1, linhas de 0x3b bytes) do grupo `i` de uma copa italiana: o grupo 0 é a própria copa; o grupo i ≥ 1 é
/// a fase [+0xc][i − 1].
unsafe fn cqr9_tabela(this: usize, i: usize) -> usize {
    if i == 0 {
        cqr9_d(this + 0xb1)
    } else {
        cqr9_d(cqr9_d(cqr9_d(this + 0xc) + (i - 1) * 4) + 0xb1)
    }
}

/// Os que entram direto no mata-mata: a partir da vaga `k` do vetor `times`, até `fim`, os inscritos ainda na lista (+0xee,
/// até +0xf2 vagas: os não nulos, na ordem), cada um zerado ao sair.
unsafe fn cqr9_cabecas(this: usize, times: usize, mut k: usize, fim: usize) {
    let mut j = 0i32;
    if (cqr9_d(this + 0xf2) as i32) <= 0 {
        return;
    }
    loop {
        if k >= fim {
            break;
        }
        let c = cqr9_d(cqr9_d(this + 0xee) + j as usize * 4);
        if c != 0 {
            cqr9_pd(times + k * 4, c);
            cqr9_pd(cqr9_d(this + 0xee) + j as usize * 4, 0);
            k += 1;
        }
        j += 1;
        if j >= cqr9_d(this + 0xf2) as i32 {
            break;
        }
    }
}

/// O mata-mata (Cup_Stage) com os `n` times de `times`: as rodadas pelo método +0x3c da copa (push 0; push &extra; push
/// &rodadas; push fase; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2), cdecl; Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, n, o vetor, as rodadas (word), +4, a tabela das rodadas, o ano +0x40 (word), a fase, 1, o
/// extra (word), 0x14, 0, 0, 0 e `sementes`; a original empurra os words com lixo nos 16 bits altos, que o construtor não
/// lê) em [+0xc][fase]; o vetor e a tabela das rodadas liberados (0x9452ca, cdecl; nesta ordem). Sem fase, a caixa da
/// `linha`; com ela, o sorteio da primeira rodada (Cup::draw_round 0x51c410: push 0; mov ecx, fase -> thiscall, ret 4).
unsafe fn cqr9_mata_mata(this: usize, n: u32, times: usize, fase: u32, sementes: *const u8, arquivo: usize, linha: u32) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr9_d(cqr9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, fase, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n, times, rodadas as u16 as u32, cqr9_d(this + 4), tabela, cqr9_w(this + 0x40) as u16 as u32, fase, 1, extra as u16 as u32, 0x14, 0, 0, 0, sementes)
    } else {
        0
    };
    cqr9_pd(cqr9_d(this + 0xc) + fase as usize * 4, f);
    soltar(times);
    soltar(tabela);
    let f = cqr9_d(cqr9_d(this + 0xc) + fase as usize * 4);
    if f == 0 {
        return aviso_em(arquivo, linha);
    }
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    sortear(f, 0);
}

/// { 0x63f870 } (thiscall, ret 0, ita_cup.cpp; da fase seguinte, depois da fase 6; quadro SEH só para o new, sem
/// destrutor): `ItaCup::create_knockout_stage()`: o mata-mata da Copa da Itália com 24 times: um vetor de 24 ponteiros
/// (0x944e46 de 0x60; sem memória, a caixa da linha 0x29c) com os 8 campeões dos grupos (a primeira linha da tabela de
/// cada grupo, `cqr9_tabela`) e, nas vagas 8 a 23, os 16 cabeças de chave que entram direto (`cqr9_cabecas`); a fase 7
/// (`cqr9_mata_mata`) com as sementes {3 × 8, 2 × 8, 1 × 8} (os campeões dos grupos, depois os cabeças de chave
/// [inferência: o pote de cada vaga no sorteio]) e a caixa da linha 0x2c0.
unsafe extern "thiscall" fn cqr9_copa_da_italia_mata_mata(this: usize) {
    let mut sementes = [0u8; 24];
    for (k, s) in sementes.iter_mut().enumerate() {
        *s = 3 - (k / 8) as u8;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let times = alocar(0x60);
    if times == 0 {
        return aviso_em(0x9ea23c, 0x29c);
    }
    crate::comparacao::ramo("cqr_9 mata-mata da copa da italia");
    for i in 0..8usize {
        cqr9_pd(times + i * 4, cqr9_d(cqr9_tabela(this, i))); // cqr9: o campeão do grupo i
    }
    cqr9_cabecas(this, times, 8, 0x18);
    cqr9_mata_mata(this, 0x18, times, 7, sementes.as_ptr(), 0x9ea23c, 0x2c0);
}

/// O plano dos dois mata-matas (feito antes): os campeões dos `grupos` grupos e os primeiros inscritos não nulos de +0xee
/// que cabem; o comparador confere, depois da original, a fase em [+0xc][fase] e esses inscritos zerados em +0xee.
unsafe fn cqr9_comparar(nome: &str, this: usize, fase: usize, cabem: usize, original: impl FnOnce()) {
    let mut vagas = Vec::new();
    let mut j = 0usize;
    while vagas.len() < cabem && (j as i32) < cqr9_d(this + 0xf2) as i32 {
        if cqr9_d(cqr9_d(this + 0xee) + j * 4) != 0 {
            vagas.push(j);
        }
        j += 1;
    }
    original();
    let f = cqr9_d(cqr9_d(this + 0xc) + fase * 4);
    let ok = f != 0 && vagas.iter().all(|&j| cqr9_d(cqr9_d(this + 0xee) + j * 4) == 0);
    PLACAR.registrar(ok, || format!("{}({:#x}) fase {:#x}, vagas tiradas {:?}", nome, this, f, vagas));
}

unsafe extern "thiscall" fn comparar_cqr9_copa_da_italia_mata_mata(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2080);
    // ponytail: aloca a fase e sorteia a primeira rodada; não pode rodar duas vezes: só a original, com o plano
    cqr9_comparar("cqr9_copa_da_italia_mata_mata", this, 7, 16, || o(this))
}

/// { 0x63dc10 } (thiscall, ret 0, ita_c_cup.cpp; da fase seguinte, depois da fase 14; quadro SEH só para os dois new, sem
/// destrutor no caminho normal): `ItaCCup::create_knockout_stage()`: o mata-mata da Copa da Itália da Série C com 32
/// times. 1) A classificação dos segundos colocados: uma liga provisória (0x944cf1(0xee); League_Stage 0x68b100,
/// thiscall, ret 0x58: a copa, 16, 0, 0, +4, 0, 0, +0xc2, +0xc3, 0, &{1, 2, 3, 0} (os critérios de desempate), &+0xbe, o
/// ano +0x40 (word), −2, 0, 0, 1, 0, 2, −1, 0, 2) cuja tabela recebe, na linha i, a segunda linha (+0x3b) da tabela do
/// grupo i (os 0x3b bytes copiados) com +5 e +9 zerados, ordenada (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall,
/// ret 4). 2) Um vetor de 32 ponteiros (0x944e46 de 0x80; sem memória, a caixa da linha 0x2ba) com os 16 campeões dos
/// grupos (a primeira linha de cada tabela) e, nas vagas 16 a 21, os 6 melhores segundos (as 6 primeiras linhas da
/// classificação): cada um, achado na tabela do seu grupo (o primeiro grupo em que aparece), marca o grupo (+0xbe = 2) e a
/// linha dele (+0x37 = 6). 3) A liga provisória destruída (a virtual 0 com 1). 4) Nas vagas 22 a 31, os 10 cabeças de chave
/// que entram direto (`cqr9_cabecas`). 5) A fase 15 (`cqr9_mata_mata`), sem sementes, e a caixa da linha 0x304.
unsafe extern "thiscall" fn cqr9_copa_da_serie_c_mata_mata(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let desempate: [u8; 4] = [1, 2, 3, 0];
    let p = novo(0xee);
    let liga = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(0x68b100usize);
        construir(p, this, 0x10, 0, 0, cqr9_d(this + 4), 0, 0, *((this + 0xc2) as *const u8) as u32, *((this + 0xc3) as *const u8) as u32, 0,
            desempate.as_ptr() as usize, this + 0xbe, cqr9_w(this + 0x40) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for i in 0..16usize {
        let de = cqr9_tabela(this, i) + 0x3b;
        let para = cqr9_d(liga + 0xb1) + i * 0x3b;
        core::ptr::copy_nonoverlapping(de as *const u8, para as *mut u8, 0x3b); // cqr9: o segundo do grupo i
        cqr9_pd(cqr9_d(liga + 0xb1) + i * 0x3b + 9, 0);
        cqr9_pd(cqr9_d(liga + 0xb1) + i * 0x3b + 5, 0);
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    ordenar(liga, 0); // cqr9: a classificação dos segundos
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let times = alocar(0x80);
    if times == 0 {
        return aviso_em(0x9ea204, 0x2ba);
    }
    crate::comparacao::ramo("cqr_9 mata-mata da copa da serie c");
    for i in 0..16usize {
        cqr9_pd(times + i * 4, cqr9_d(cqr9_tabela(this, i)));
    }
    let mut l = 0usize;
    while l < 0x162 {
        let c = cqr9_d(cqr9_d(liga + 0xb1) + l);
        cqr9_pd(times + 0x40 + (l / 0x3b) * 4, c); // cqr9: os 6 melhores segundos
        let mut g = 0usize;
        while g < 0x10 {
            let grupo = if g == 0 { this } else { cqr9_d(cqr9_d(this + 0xc) + (g - 1) * 4) };
            let n = cqr9_w(grupo + 0x3e) as i32;
            let mut r = 0i32;
            while r < n {
                if cqr9_d(cqr9_d(grupo + 0xb1) + r as usize * 0x3b) == cqr9_d(cqr9_d(liga + 0xb1) + l) {
                    *((grupo + 0xbe) as *mut u8) = 2;
                    *((cqr9_d(grupo + 0xb1) + r as usize * 0x3b + 0x37) as *mut u8) = 6;
                    g = 0x10;
                    break;
                }
                r += 1;
            }
            g += 1;
        }
        l += 0x3b;
    }
    if liga != 0 {
        let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqr9_d(cqr9_d(liga)));
        destruir(liga, 1);
    }
    cqr9_cabecas(this, times, 0x16, 0x20);
    cqr9_mata_mata(this, 0x20, times, 0xf, core::ptr::null(), 0x9ea204, 0x304);
}

unsafe extern "thiscall" fn comparar_cqr9_copa_da_serie_c_mata_mata(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2081);
    // ponytail: monta e destrói a liga provisória, aloca a fase e sorteia; não pode rodar duas vezes: só a original
    cqr9_comparar("cqr9_copa_da_serie_c_mata_mata", this, 0xf, 10, || o(this))
}

// ---------- 284a leva: as eliminatórias da Eurocopa: as regras e a data do sorteio, o tamanho de cada grupo, a situação de um clube e a tabela do grupo 1 (euro_champ_qual.cpp) ----------

unsafe fn cqs8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs8_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// { 0x57e8f0 } (thiscall, ret 0, euro_champ_qual.cpp; a virtual +0x8c, do construtor em 2001-000; quadro SEH pela Date
/// local, sem destrutor): `EuroChampQual::set_up_rules()`: 3 pontos pela vitória (+0xc2) e 1 pelo empate (+0xc3), +0x4a =
/// 3, dois turnos (+0x3c = 2, word), a categoria 0xa dos jogos (+0xc4), +0x43 = 2, os desempates 1, 2, 4, 5 (+0xc5..+0xc8),
/// uma vaga direta (+0xbe = 1) e uma de play-off (+0xbf = 1), sem rebaixamento (+0xc0 = +0xc1 = 0), +0x49 = 7, +0xa7 =
/// 0xffff (word); a data do sorteio (+0xb6, o dia do ano) = Date(10, 2, ano +0x40, 5) (0x549f70: push 5; push ano; push 2;
/// push 0xa; lea ecx, local -> thiscall, ret 0x10: 10 de março levado à sexta-feira [inferência]); +0xb8 = 0 (word); a
/// tabela das rodadas pela virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) em
/// +0xba. Devolve 1.
unsafe fn cqs8_regras(this: usize) -> u32 {
    cqs8_pb(this + 0xc2, 3);
    cqs8_pb(this + 0x4a, 3);
    core::ptr::write_unaligned((this + 0x3c) as *mut i16, 2); // cqs8: dois turnos
    cqs8_pb(this + 0xc3, 1);
    cqs8_pb(this + 0xc4, 0xa);
    cqs8_pb(this + 0x43, 2);
    for (k, v) in [1u8, 2, 4, 5].into_iter().enumerate() {
        cqs8_pb(this + 0xc5 + k, v);
    }
    cqs8_pb(this + 0xbe, 1);
    cqs8_pb(this + 0xbf, 1);
    cqs8_pb(this + 0xc0, 0);
    cqs8_pb(this + 0xc1, 0);
    cqs8_pb(this + 0x49, 7);
    core::ptr::write_unaligned((this + 0xa7) as *mut u16, 0xffff);
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut data = [0u32; 2];
    // ponytail: a original empurra o ano no ecx com os 16 bits altos do ponteiro da copa; a 0x549f70 lê a palavra
    construir(&mut data, 0xa, 2, cqs8_w(this + 0x40) as u16 as u32, 5); // cqs8: a data do sorteio
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, data[0] as u16);
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, 0);
    crate::comparacao::ramo("cqs_8 regras da eurocopa");
    let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(cqs8_d(cqs8_d(this) + 0x3c));
    let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

unsafe extern "thiscall" fn cqs8_eurocopa_regras(this: usize) -> u32 {
    cqs8_regras(this)
}

/// Lado a lado a partir do mesmo estado (os campos +0x3a..+0xc9 da copa, fora o ponteiro +0xba); a tabela das rodadas que
/// cada lado aloca em +0xba (pela virtual +0x3c, original) só é conferida como montada, e a da cópia é liberada.
unsafe extern "thiscall" fn comparar_cqs8_eurocopa_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2082);
    let antes = cqs8_d(this + 0xba);
    let reg = [(this + 0x3a, 0x80usize), (this + 0xbe, 0xb)];
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        to.set(cqs8_d(this + 0xba));
        core::ptr::write_unaligned((this + 0xba) as *mut u32, antes as u32);
        r
    }, || {
        let r = cqs8_regras(this);
        tr.set(cqs8_d(this + 0xba));
        core::ptr::write_unaligned((this + 0xba) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    // a tabela é montada pela virtual +0x3c (original nos dois lados) e tem bytes que ela não grava (sobra do malloc):
    // confere só que os dois lados a montaram
    let tabela = to != 0 && tr != 0;
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(tr);
    }
    PLACAR.registrar(igual && a == b && tabela, || format!("cqs8_eurocopa_regras({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}

/// { 0x57d9d0 } (thiscall, ret 4, euro_champ_qual.cpp): `EuroChampQual::get_group_team_count(g)`: com o formato +0xf6 = 1
/// (9 grupos [inferência]), os grupos 0, 2 e 5 têm 6 seleções e os outros 5; com o formato 2 (10 grupos [inferência]), os
/// grupos 0, 2, 5 e 7 têm 6 e os outros 5; outro formato, a caixa da linha 0x245 e 5. `g` é o byte do argumento.
unsafe fn cqs8_tamanho(this: usize, g: u32) -> i16 {
    let g = g as u8;
    match *((this + 0xf6) as *const i8) {
        1 => if matches!(g, 0 | 2 | 5) { 6 } else { 5 }, // cqs8: os grupos de 6 no formato 1
        2 => if matches!(g, 0 | 2 | 5 | 7) { 6 } else { 5 },
        _ => {
            aviso_em(0x9c4674, 0x245);
            5
        }
    }
}

unsafe extern "thiscall" fn cqs8_eurocopa_tamanho_do_grupo(this: usize, g: u32) -> i16 {
    cqs8_tamanho(this, g)
}

unsafe extern "thiscall" fn comparar_cqs8_eurocopa_tamanho_do_grupo(this: usize, g: u32) -> i16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> i16>(2083);
    let r = o(this, g);
    if !matches!(*((this + 0xf6) as *const i8), 1 | 2) {
        return r; // ponytail: o formato desconhecido mostra a caixa: só a original
    }
    let n = cqs8_tamanho(this, g);
    PLACAR.registrar(r == n, || format!("cqs8_eurocopa_tamanho_do_grupo(formato {}, grupo {}) original={} rust={}", *((this + 0xf6) as *const i8), g as u8, r, n));
    r
}

/// { 0x580b50 } (thiscall, ret 4, euro_champ_qual.cpp; a virtual da situação de um clube): `EuroChampQual::
/// get_qualification_status(clube)`: a estatística 0x12 do clube nas estatísticas da copa (Comp_Stats::get_club_stat
/// 0x4a2e10: push 0x12; push clube; mov ecx, [this+8] -> thiscall, ret 8, devolve float), truncada (_ftol 0x945640): com o
/// byte de baixo menor que 14 (com sinal), −2; senão −3 (word).
unsafe fn cqs8_situacao(this: usize, clube: usize) -> i16 {
    let estatistica: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(0x4a2e10usize);
    let x = ftol(estatistica(cqs8_d(this + 8), clube, 0x12));
    ((((x as u8 as i8) < 0xe) as i32) - 3) as i16 // cqs8: a situação pela estatística 0x12
}

unsafe extern "thiscall" fn cqs8_eurocopa_situacao(this: usize, clube: usize) -> i16 {
    cqs8_situacao(this, clube)
}

unsafe extern "thiscall" fn comparar_cqs8_eurocopa_situacao(this: usize, clube: usize) -> i16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> i16>(2084);
    let r = o(this, clube);
    let n = cqs8_situacao(this, clube);
    PLACAR.registrar(r == n, || format!("cqs8_eurocopa_situacao({:#x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x57e7e0 } (thiscall, ret 0, euro_champ_qual.cpp; do sorteio, 2002-067): `EuroChampQual::init_group_one_table()`: o
/// grupo 1 é a própria copa: +0x3e = o tamanho do grupo −1 (0x57d9d0: push −1; mov ecx -> thiscall, ret 4; o byte 0xff
/// não é grupo de 6: 5); a tabela (0x944e46 de +0x3e × 0x3b em +0xb1; sem memória, a caixa da linha 0x2d2 e 0); +0xa7 =
/// (+0x3e − 1) × +0x3c (words: as rodadas, ida e volta); cada seleção da lista +0xf7 (na ordem) na linha i
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push o dword do byte i com lixo em cima (só o byte é lido);
/// mov ecx -> thiscall, ret 0x10); a notícia do grupo (0x684230, thiscall). Devolve 1.
unsafe extern "thiscall" fn cqs8_eurocopa_grupo_um(this: usize) -> u32 {
    let tamanho: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(0x57d9d0usize);
    let n = tamanho(this, 0xffff_ffff);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 * 0x3b) as u32);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        aviso_em(0x9c4674, 0x2d2);
        return 0;
    }
    let n = cqs8_w(this + 0x3e);
    core::ptr::write_unaligned((this + 0xa7) as *mut i16, n.wrapping_sub(1).wrapping_mul(cqs8_w(this + 0x3c))); // cqs8: as rodadas do grupo 1
    crate::comparacao::ramo("cqs_8 grupo 1 da eurocopa");
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut i: i8 = 0;
    if n > 0 {
        loop {
            let c = cqs8_d(cqs8_d(this + 0xf7) + i as isize as usize * 4);
            iniciar(this, i as u8 as u32, c, 0, 0);
            i = i.wrapping_add(1);
            if (i as i16) >= cqs8_w(this + 0x3e) {
                break;
            }
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    noticia(this);
    1
}

/// Só a original (a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos), conferindo o
/// plano feito antes: devolve 1, +0x3e = o tamanho do grupo −1, +0xa7 = (+0x3e − 1) × +0x3c e as seleções de +0xf7 nas
/// linhas, na ordem.
unsafe extern "thiscall" fn comparar_cqs8_eurocopa_grupo_um(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2085);
    let n = if matches!(*((this + 0xf6) as *const i8), 1 | 2) { 5i16 } else { return o(this) };
    let plano: Vec<usize> = (0..n as usize).map(|k| cqs8_d(cqs8_d(this + 0xf7) + k * 4)).collect();
    let r = o(this);
    let t = cqs8_d(this + 0xb1);
    let ok = r == 1 && cqs8_w(this + 0x3e) == n && cqs8_w(this + 0xa7) == (n - 1).wrapping_mul(cqs8_w(this + 0x3c)) && t != 0
        && plano.iter().enumerate().all(|(k, &c)| cqs8_d(t + k * 0x3b) == c);
    PLACAR.registrar(ok, || format!("cqs8_eurocopa_grupo_um({:#x}) devolveu {}; plano {:x?}", this, r, plano));
    r
}

// ---------- 284a leva: a Copa da UEFA: os inscritos (os clubes das vagas da classificação da UEFA, completados até 118 com clubes de nações fracas, e a lista dos que vêm da Intertoto) (uefa_cup.cpp) ----------

unsafe fn cqs9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs9_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os inscritos menos a metade (truncada, com sinal) dos clubes da preliminar (+0xb2, byte com sinal).
unsafe fn cqs9_conta(this: usize, k: i16) -> i32 {
    k as i32 - (*((this + 0xb2) as *const i8) as i32) / 2
}

/// { 0x902b90 } (thiscall, ret 0, uefa_cup.cpp; do construtor, 2001-000): `UefaCup::build_entrants()`: +0x3e = 145; a
/// lista (calloc 0x945501(161, 6) em +0xa7; sem memória, a caixa da linha 0x1bf e −1); +0xb2 = 0.
/// 1) Pela ordem da tabela dos clubes, os que as regras europeias puseram na Copa da UEFA (+0x1db = [0x9cf6f0]), cada um
///    com a classe = o seu coeficiente (+0x1df) e a situação 0; os de coeficiente 10 ou mais (com sinal) contam em +0xb2
///    (os da preliminar [inferência]).
/// 2) Se os inscritos menos a metade de +0xb2 não chegam a 118, completa (entre os [0xae235c] − 2 × [0xae2350] primeiros
///    clubes, até chegar a 118) com os clubes sem copa europeia (+0x1db = −1) de nação (+0x53) sem a marca +0x11c e de
///    coeficiente da nação (+0x8e, word com sinal) abaixo de 2000: cada um com a classe 3, a situação 0 e +0x1db = o id da
///    copa ([+4] +0).
/// 3) Passando de 118, tira os últimos (a vaga fica zerada) até 118.
/// A lista vai ao qsort (0x945333, 6 bytes, 0x522190: pela classe). Com +0xb2 par (e não negativo): +0x36 = 0, +0x38 =
/// os inscritos + 27, a lista dos que vêm da Intertoto [inferência] (0x944e46 de +0x38 × 6 em +0x14; sem memória, a
/// caixa da linha 0x21a e 0) com os inscritos de classe 1, cada um com a classe 3, contados em +0x36; +0x3e = +0x38;
/// devolve 1. Com +0xb2 ímpar ou negativo, 0.
unsafe fn cqs9_inscritos(this: usize) -> i32 {
    const F: usize = 0xaa19e0;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x91);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let l = zerada(0xa1, 6);
    cqs9_pd(this + 0xa7, l);
    if l == 0 {
        aviso_em(F, 0x1bf);
        return -1;
    }
    *((this + 0xb2) as *mut u8) = 0;
    let base = cqs9_d(0xae23b4);
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    if (cqs9_d(0xae235c) as i32) > 0 {
        loop {
            let c = base + i as u16 as usize * 0x245;
            if cqs9_d(c + 0x1db) == cqs9_d(0x9cf6f0) {
                let e = k as i32 as usize * 6;
                k = k.wrapping_add(1);
                cqs9_pd(cqs9_d(this + 0xa7) + e, c);
                *((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = *((c + 0x1df) as *const u8);
                *((cqs9_d(this + 0xa7) + e + 5) as *mut u8) = 0;
                if *((c + 0x1df) as *const i8) >= 0xa { // cqs9: os da preliminar
                    *((this + 0xb2) as *mut u8) = (*((this + 0xb2) as *const u8)).wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
            if i as i32 >= cqs9_d(0xae235c) as i32 {
                break;
            }
        }
    }
    if cqs9_conta(this, k) < 0x76 {
        let lim = |_: ()| (cqs9_d(0xae235c) as i32).wrapping_sub((cqs9_d(0xae2350) as i32).wrapping_mul(2));
        let mut j: i16 = 0;
        if lim(()) > 0 {
            loop {
                if cqs9_conta(this, k) >= 0x76 { // cqs9: completa até 118
                    break;
                }
                let c = base + j as u16 as usize * 0x245;
                let nac = cqs9_d(c + 0x53);
                if nac != 0 && *((nac + 0x11c) as *const u8) == 0 && cqs9_w(nac + 0x8e) < 0x7d0 && cqs9_d(c + 0x1db) as i32 == -1 {
                    crate::comparacao::ramo("cqs_9 clube completando a uefa");
                    let e = k as i32 as usize * 6;
                    k = k.wrapping_add(1);
                    cqs9_pd(cqs9_d(this + 0xa7) + e, c);
                    *((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = 3; // cqs9: a classe de quem completa
                    *((cqs9_d(this + 0xa7) + e + 5) as *mut u8) = 0;
                    cqs9_pd(c + 0x1db, cqs9_d(cqs9_d(this + 4)));
                }
                j = j.wrapping_add(1);
                if j as i32 >= lim(()) {
                    break;
                }
            }
        }
    }
    while cqs9_conta(this, k) > 0x76 {
        cqs9_pd(cqs9_d(this + 0xa7) + k as i32 as usize * 6 - 6, 0);
        k = k.wrapping_sub(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqs9_d(this + 0xa7), k as i32 as u32, 6, 0x522190);
    let b2 = *((this + 0xb2) as *const i8);
    if b2 < 0 || b2 % 2 != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqs_9 inscritos da uefa");
    core::ptr::write_unaligned((this + 0x36) as *mut i16, 0);
    let n = k.wrapping_add(0x1b);
    core::ptr::write_unaligned((this + 0x38) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 * 6) as u32);
    cqs9_pd(this + 0x14, t);
    if t == 0 {
        aviso_em(F, 0x21a);
        return 0;
    }
    if k > 0 {
        for q in 0..k as usize {
            let e = cqs9_d(this + 0xa7) + q * 6;
            let c = cqs9_d(e);
            if *((e + 4) as *const u8) == 1 && c != 0 {
                let m = cqs9_w(this + 0x36) as i32 as usize * 6;
                cqs9_pd(cqs9_d(this + 0x14) + m, c);
                *((cqs9_d(this + 0x14) + cqs9_w(this + 0x36) as i32 as usize * 6 + 4) as *mut u8) = 3;
                core::ptr::write_unaligned((this + 0x36) as *mut i16, cqs9_w(this + 0x36).wrapping_add(1));
            }
        }
    }
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    1
}

unsafe extern "thiscall" fn cqs9_uefa_inscritos(this: usize) -> i32 {
    cqs9_inscritos(this)
}

/// Lado a lado a partir do mesmo estado: +0x14, +0x36..+0x39, +0x3e, +0xb2, [0xb67a34] e o +0x1db de cada clube; as duas
/// listas (+0xa7 com 161 vagas e +0x14 com +0x38 vagas) de cada lado comparadas e as da cópia liberadas.
unsafe extern "thiscall" fn comparar_cqs9_uefa_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(2086);
    let (a0, t0) = (cqs9_d(this + 0xa7), cqs9_d(this + 0x14));
    let base = cqs9_d(0xae23b4);
    let mut reg: Vec<(usize, usize)> = vec![(this + 0x36, 4), (this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)];
    reg.extend((0..cqs9_d(0xae235c)).map(|i| (base + i * 0x245 + 0x1db, 4)));
    let lo = core::cell::Cell::new((0usize, 0usize));
    let lr = core::cell::Cell::new((0usize, 0usize));
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set((cqs9_d(this + 0xa7), cqs9_d(this + 0x14)));
        cqs9_pd(this + 0xa7, a0);
        cqs9_pd(this + 0x14, t0);
        r
    }, || {
        let r = cqs9_inscritos(this);
        lr.set((cqs9_d(this + 0xa7), cqs9_d(this + 0x14)));
        cqs9_pd(this + 0xa7, lo.get().0);
        cqs9_pd(this + 0x14, lo.get().1);
        r
    });
    let ((ao, to), (ar, tr)) = (lo.get(), lr.get());
    let igual_em = |x: usize, y: usize, n: usize| (x == 0 && y == 0) || (x != 0 && y != 0 && core::slice::from_raw_parts(x as *const u8, n) == core::slice::from_raw_parts(y as *const u8, n));
    let listas = igual_em(ao, ar, 0xa1 * 6) && (a != 1 || igual_em(to, tr, cqs9_w(this + 0x36).max(0) as usize * 6));
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if ar != 0 && ar != ao {
        soltar(ar);
    }
    if tr != 0 && tr != to && tr != t0 {
        soltar(tr);
    }
    PLACAR.registrar(igual && a == b && listas, || format!("cqs9_uefa_inscritos({:#x}) devolveu {} x {} memória {} listas {}", this, a, b, igual, listas));
    a
}

// ---------- 284a leva: o dia da Copa do Rei (a copa refeita uma vez no ano dela) e o histórico da temporada de quatro copas (o da última fase): a Liga dos Campeões, a Copa da Grécia, a Copa da Liga irlandesa e a Copa da Série C italiana (spa_cup.cpp, european_cup.cpp, gre_cup.cpp, ire_lge_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqt5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A virtual `o` de `this` (thiscall, sem argumentos).
unsafe fn cqt5_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqt5_d(cqt5_d(this) + o));
    f(this)
}

/// { 0x84d740 } (thiscall, ret 8, spa_cup.cpp; a virtual +0x14 da Copa do Rei, todo dia): `SpaCup::update_for_date(hoje,
/// fase)`. Uma vez (+0x45 = 0), quando o ano da classificação da UEFA ([0xde1f58] +0x439, palavra sem sinal) é o da copa
/// (+0x40, com sinal) e a fase não é 0: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos de cada
/// rodada (+0xad, +0x3c rodadas) liberados (0x9452ca; os ponteiros ficam), +0x45 = 1, os inscritos (SpaCup::
/// collect_entrants 0x84d910, thiscall), as rodadas (virtual +0x8c) e a árvore (virtual +0x94) — uma devolução negativa
/// dá a caixa (linhas 0x161, 0x168, 0x16f) e 0 — e as posições previstas (virtual +0x5c). Sempre, no fim, a atualização da
/// copa (0x51f890: push fase; push hoje; mov ecx -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn cqt5_copa_do_rei_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqt5_d(this + 0x45) == 0 && cqt5_w(cqt5_d(0xde1f58) + 0x439) as u16 as i32 == cqt5_w(this + 0x40) as i32 && fase != 0 { // cqt5: o ano da Copa do Rei
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        for o in [0xa7usize, 0xa3] {
            if cqt5_d(this + o) != 0 {
                soltar(cqt5_d(this + o));
            }
        }
        if cqt5_d(this + 0xad) != 0 {
            let mut i = 0i32;
            while i < cqt5_w(this + 0x3c) as i32 {
                let p = cqt5_d(cqt5_d(this + 0xad) + i as usize * 4);
                if p != 0 {
                    soltar(p);
                }
                i += 1;
            }
            soltar(cqt5_d(this + 0xad));
        }
        crate::comparacao::ramo("cqt_5 copa do rei refeita");
        core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqt5: a Copa do Rei refeita uma vez
        let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x84d910usize);
        // ponytail: nas caixas a original zera só o al (o resto do eax vem da caixa); quem chama lê o byte
        if inscritos(this) < 0 {
            aviso_em(0xa82810, 0x161);
            return 0;
        }
        if cqt5_virtual(this, 0x8c) < 0 {
            aviso_em(0xa82810, 0x168);
            return 0;
        }
        if cqt5_virtual(this, 0x94) < 0 {
            aviso_em(0xa82810, 0x16f);
            return 0;
        }
        cqt5_virtual(this, 0x5c);
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

// ponytail: a virada libera e refaz as tabelas e a atualização da copa joga e sorteia: não rodam duas vezes. Só a
// original; o plano: +0x45 vai a 1 quando a condição vale, senão fica.
unsafe extern "thiscall" fn comparar_cqt5_copa_do_rei_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2087);
    let antes = cqt5_d(this + 0x45);
    let vira = antes == 0 && cqt5_w(cqt5_d(0xde1f58) + 0x439) as u16 as i32 == cqt5_w(this + 0x40) as i32 && fase != 0;
    let r = o(this, hoje, fase);
    let depois = cqt5_d(this + 0x45);
    PLACAR.registrar(depois == if vira { 1 } else { antes }, || format!("cqt5_copa_do_rei_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// O histórico da temporada de uma copa (`record_season_honours`, 11 bytes: mov eax, [ecx+0xc]; mov ecx, [eax+o]; mov
/// edx, [ecx]; jmp [edx+0x30]): a virtual +0x30 da fase [+0xc][o/4] (a última fase da copa: a final), com o eax dela.
unsafe fn cqt5_historico(this: usize, o: usize, ramo: &'static str) -> i32 {
    crate::comparacao::ramo(ramo);
    let fase = cqt5_d(cqt5_d(this + 0xc) + o);
    cqt5_virtual(fase, 0x30)
}

/// { 0x584ad0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::record_season_honours()`: o histórico da fase 12
/// (+0x30: a final da Liga dos Campeões).
unsafe extern "thiscall" fn cqt5_historico_liga_dos_campeoes(this: usize) -> i32 {
    cqt5_historico(this, 0x30, "cqt_5 historico da liga dos campeoes")
}

/// { 0x5e93e0 } (thiscall, ret 0, gre_cup.cpp): `GreCup::record_season_honours()`: o histórico da fase 10 (+0x28).
unsafe extern "thiscall" fn cqt5_historico_copa_da_grecia(this: usize) -> i32 {
    cqt5_historico(this, 0x28, "cqt_5 historico da copa da grecia")
}

/// { 0x637750 } (thiscall, ret 0, ire_lge_cup.cpp): `IreLgeCup::record_season_honours()`: o histórico da fase 7 (+0x1c).
unsafe extern "thiscall" fn cqt5_historico_copa_da_liga_irlandesa(this: usize) -> i32 {
    cqt5_historico(this, 0x1c, "cqt_5 historico da copa da liga irlandesa")
}

/// { 0x63d770 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::record_season_honours()`: o histórico da fase 15 (+0x3c: o
/// mata-mata da Copa da Série C).
unsafe extern "thiscall" fn cqt5_historico_copa_da_serie_c(this: usize) -> i32 {
    cqt5_historico(this, 0x3c, "cqt_5 historico da copa da serie c")
}

// ponytail: o histórico grava nas estatísticas das competições (acha ou acrescenta entradas: aloca); só a original
unsafe extern "thiscall" fn comparar_cqt5_historico_liga_dos_campeoes(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2088)(this)
}

unsafe extern "thiscall" fn comparar_cqt5_historico_copa_da_grecia(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2089)(this)
}

unsafe extern "thiscall" fn comparar_cqt5_historico_copa_da_liga_irlandesa(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2090)(this)
}

unsafe extern "thiscall" fn comparar_cqt5_historico_copa_da_serie_c(this: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize) -> i32>(2091)(this)
}
