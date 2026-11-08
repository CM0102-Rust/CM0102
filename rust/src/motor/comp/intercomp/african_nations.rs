use super::*;




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
pub(crate) unsafe extern "thiscall" fn african_nations_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AFRICANNATIONS_DRAW_FINALISTS);
    let grupo_a: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AFRICANNATIONS_INIT_GROUP_A_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_AFRICANNATIONS_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if completo == 0 {
        return liga(this, hoje, 0);
    }
    let dia = read_i16(this + 0xb6) as u16;
    let ano = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do sorteio e nos erros a original devolve só o al = 0 (o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && read_u32_as_usize(this + 0x45) == 0 {
        crate::comparacao::ramo("cqs_13 dia do sorteio da copa africana");
        sortear(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
        previstas(this);
        if grupo_a(this) == 0 {
            show_error_at(0x9870f8, 0x3c2);
            return 0;
        }
        if jogos(this) == 0 {
            show_error_at(0x9870f8, 0x3c8);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            show_error_at(0x9870f8, 0x3cf);
            return 0;
        }
        for g in 0..3i32 {
            fase(this, g);
        }
        soltar(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, 0);
        core::ptr::write_unaligned((this + 0xf2) as *mut u16, 0);
        write_u32_from_usize(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: o sorteio, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do grupo A e as três fases
// criadas e a fase atual (+0x30) a última delas (2).
/// Comparer for `african_nations_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_FOR_DATE);
    let sorteia = completo != 0
        && read_u32_as_usize(this + 0x45) == 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_i16(this + 0xf2) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && (0..3).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 2;
        PLACAR.registrar(ok, || format!("african_nations_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// AfricanNations::draw_finalists: hosts first, then African nations by reputation, drawn from 4 pots into 4 groups of
/// 4 stored in +0xee; returns 1 (0 on failure).
///
/// [pt] { 0x4022c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::draw_finalists()`: +0xf2 (palavra) = 16 e a
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
pub(crate) unsafe fn draw_african_nations_finalists(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
    core::ptr::write_unaligned((this + 0xf2) as *mut u16, 0x10);
    let lista = alocar(0xc8);
    write_u32_from_usize(this + 0xee, lista);
    if lista == 0 {
        show_error_at(0x9870f8, 0x349);
        return 0;
    }
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // ponytail: a original empurra o eax com o ano nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let quantas = sedes(read_u32_as_usize(G_HOST_COUNTRY), read_u32_as_usize(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS), read_i16(this + 0x40) as u16 as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut n = 0usize;
    if quantas > 0 {
        for &id in ids.iter().take(quantas as usize) {
            write_u32_from_usize(read_u32_as_usize(this + 0xee) + n * 4, selecao(read_u32_as_usize(G_NATION_TABLE) + id as usize * nation::SIZE));
            n += 1;
        }
    }
    let mut i = 0i16;
    let mut nacao = read_u32_as_usize(G_NATION_TABLE);
    while (i as i32) < read_u32_as_usize(G_NATION_COUNT) as i32 {
        let continente = read_u32_as_usize(nacao + nation::CONTINENT);
        if continente != 0 && read_u32_as_usize(continente) == read_u32_as_usize(KEY_CONTINENT_AFRICA) && n < 0x32 { // cqs13: até 50 africanas
            let l = read_u32_as_usize(this + 0xee);
            let sede = (0..quantas.max(0) as usize).any(|j| read_u32_as_usize(read_u32_as_usize(l + j * 4) + 0x53) == nacao);
            if !sede {
                write_u32_from_usize(read_u32_as_usize(this + 0xee) + n * 4, selecao(nacao));
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
    ordenar(read_u32_as_usize(this + 0xee) + (h as isize * 4) as usize, (n as i32 - h) as u32, 4, 0x4b07d0); // cqs13: as não sedes pela reputação
    let grupos = alocar(0x40);
    if grupos == 0 {
        show_error_at(0x9870f8, 0x380);
        return 0;
    }
    crate::comparacao::ramo("cqs_13 sorteio da copa africana");
    for j in 0..4usize {
        for p in 0..4usize {
            let mut r = sorteio(4) as u8 as i8 as usize;
            while read_u32_as_usize(read_u32_as_usize(this + 0xee) + (r + 4 * p) * 4) == 0 {
                r = sorteio(4) as u8 as i8 as usize;
            }
            let e = read_u32_as_usize(this + 0xee) + (r + 4 * p) * 4;
            write_u32_from_usize(grupos + (4 * j + p) * 4, read_u32_as_usize(e)); // cqs13: o do pote p na vaga 4j + p
            write_u32_from_usize(e, 0);
        }
    }
    soltar(read_u32_as_usize(this + 0xee));
    write_u32_from_usize(this + 0xee, grupos);
    1
}

pub(crate) unsafe extern "thiscall" fn draw_finalists(this: usize) -> u32 {
    draw_african_nations_finalists(this)
}

/// Comparer for `draw_finalists` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa os grupos dela em +0xee, a cópia faz os seus, comparados nas 16 vagas e liberados (sem os 16, as listas
/// de 50 comparadas nas 16 primeiras vagas).
pub(crate) unsafe extern "thiscall" fn comparar_draw_finalists(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DRAW_FINALISTS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| draw_african_nations_finalists(this));
        lr.set(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("draw_finalists({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
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
pub(crate) unsafe extern "thiscall" fn african_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let times = alocar(0x10);
    if times == 0 {
        show_error_at(0x9870f8, 0x2dc);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqs_13 time do grupo");
        write_u32_from_usize(times + n * 4, read_u32_as_usize(read_u32_as_usize(this + 0xee) + ((g as usize + 1) * 4 + n) * 4)); // cqs13: o time na vaga do grupo
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
            4,
            times,
            1,
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
            4,
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
        show_error_at(0x9870f8, 0x2f2);
        return 0;
    }
    write_u32_from_usize(this + 0x30, g as usize);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
/// Comparer for `african_nations_create_group_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_AFRICAN_NATIONS_CREATE_GROUP_STAGE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 || !(0..3).contains(&g) {
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
        PLACAR.registrar(ok, || format!("african_nations_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}



/// rule: the African Nations Cup has three group stages (stages 0 to 2, one per group round [inference]) and, at stage
/// 3, the knockout; going past the last stage is an error (the box)
/// Original: `AfricanNations::advance_stage` (0x401e10, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x401e10 } (thiscall, ret 0, african_nations.cpp; a virtual +0x28): `AfricanNations::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x2ba; senão +0x30 sobe e, chegando a 3 (depois dos grupos), o mata-mata
/// (0x402090: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn african_nations_advance_stage(this: usize) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        show_error_at(0x9870f8, 0x2ba);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 3 { // cqk12: o mata-mata da Copa Africana depois dos grupos
        crate::comparacao::ramo("cqk_12 mata-mata da copa africana");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AFRICANNATIONS_CREATE_KNOCKOUT_STAGE);
        mata(this);
    }
}

// ponytail: o mata-mata (0x402090) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
/// Comparer for `african_nations_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AFRICAN_NATIONS_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("african_nations_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// rule: group A has four national teams, the first four of the draw list, in order; the managers get the draw news
/// Original: `AfricanNations::init_group_a_table` (0x4019c0, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x4019c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::init_group_a_table()`: o grupo A com 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa
/// da linha 0x1f1 e devolve 0), cada linha i com a seleção i da lista dos sorteados (+0xee) (0x687430: push 0; push 0;
/// push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this
/// -> thiscall, ret 0) e devolve 1.
pub(crate) unsafe extern "thiscall" fn init_group_a_table(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    let times = 4i16; // cqk12: os quatro do grupo A
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, times);
    let t = novo(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(0x9870f8, 0x1f1);
        return 0;
    }
    crate::comparacao::ramo("cqk_12 grupo a da copa africana");
    let mut i = 0i8;
    while (i as i16) < read_i16(this + 0x3e) {
        let selecao = read_u32_as_usize(read_u32_as_usize(this + 0xee).wrapping_add((i as isize * 4) as usize)); // cqk12: a seleção da linha
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
/// Comparer for `init_group_a_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_group_a_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INIT_GROUP_A_TABLE);
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
    PLACAR.registrar(igual, || format!("init_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// rule: the season honours (winner, runner-up, third) are the knockout ones
/// Original: `AfricanNations::record_season_honours` (0x401b90, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x401b90 } (thiscall, ret 0, african_nations.cpp; a virtual +0x30): `AfricanNations::record_season_honours()`: o
/// histórico vem do mata-mata: salta para a virtual +0x30 da fase 3 ([+0xc] + 0xc; jmp, sem quadro).
pub(crate) unsafe extern "thiscall" fn african_nations_record_season_honours(this: usize) {
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(fase) + 0x30));
    h(fase);
}

// ponytail: o histórico (a virtual +0x30 do mata-mata, a 0x519a90 já em Rust com o seu comparador) não roda duas vezes;
// a rotina não grava nada além do salto. Só a original.
/// Comparer for `african_nations_record_season_honours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AFRICAN_NATIONS_RECORD_SEASON_HONOURS)(this)
}



/// Calls a staff-records final routine f (winner/runner-up/third) on the staff records with club, match and a6.
///
/// [pt] Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push a6; push jogo; push clube; mov ecx,
/// [0xdd7f00] -> thiscall, ret 0xc).
pub(crate) unsafe fn african_nations_record_final_result(f: usize, clube: usize, jogo: usize, a6: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(eng_second_staff_records(), clube, jogo, a6);
}

/// Virtual +0x44 of a national-team tournament with groups and knockout: writes qualification/elimination/final records
/// by outcome and stage; returns 0.
///
/// [pt] A virtual +0x44 de um torneio de seleções com grupos e mata-mata (a Copa Africana e a Copa América): `fase` (byte com
/// sinal) abaixo de `grupos` é a fase de grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x78, peso 0x19); 0xfe,
/// a não classificação (0x86c1d0: fase 0x41c, peso 0x19), que encerra as convocações. Na fase `grupos` (o mata-mata,
/// a fase [+0xc][grupos]): 0, a classificação com a fase +0x32 do jogo, o +0x6f da rodada do jogo (+0x34) na tabela de
/// rodadas da fase (+0xa3, 0x68 bytes) e o peso 0x4bf850(0x28, 0x64, rodada, 3); 0xff, a eliminação (0x86c000) com
/// +0x32, +0x30 e o mesmo peso; 1, 2 e 3, os registros de campeão, vice e terceiro. Devolve 0.
pub(crate) unsafe fn national_tournament_staff_records(nome: u8, this: usize, clube: usize, desfecho: u32, fase: u32, jogo: usize, a6: u32, grupos: i8) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    if f < grupos {
        if d == 6 {
            european_cup_add_qualified_record(clube, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            if nome == 0 {
                crate::comparacao::ramo("cqj_10 africana fora nos grupos");
                european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa Africana
            } else {
                crate::comparacao::ramo("cqj_10 copa america fora nos grupos");
                european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa América
            }
        }
    } else if f == grupos {
        match d {
            0 => {
                let r = read_i16(jogo + 0x34);
                let peso = scale_by_round(0x28, 0x64, r, 3);
                let mata = read_u32_as_usize(read_u32_as_usize(this + 0xc) + grupos as usize * 4);
                let rodada = read_i16(read_u32_as_usize(mata + 0xa3).wrapping_add((r as isize * 0x68) as usize) + 0x6f) as u16 as u32;
                european_cup_add_qualified_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, rodada, peso);
            }
            0xff => {
                let peso = scale_by_round(0x28, 0x64, read_i16(jogo + 0x34), 3);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
                if nome == 0 {
                    crate::comparacao::ramo("cqj_10 africana fora no mata-mata");
                    european_cup_add_knocked_out_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso); // cqj10: quem cai no mata-mata da Copa Africana
                } else {
                    european_cup_add_knocked_out_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
                }
            }
            1 => african_nations_record_final_result(0x86a800, clube, jogo, a6),
            2 => african_nations_record_final_result(0x86b0b0, clube, jogo, a6),
            3 => african_nations_record_final_result(0x86b710, clube, jogo, a6),
            _ => {}
        }
    }
    0
}

/// rule: in the three African Nations Cup group stages, a team going through gets the qualification record (weight 25)
/// and one going out the failed-to-qualify record (weight 25), which ends the team's call-ups; in the knockout (stage
/// 3), a team going through gets the qualification by round (weight 40 to 100), one going out the knocked-out record
/// (which also ends the call-ups), and the winner, runner-up and third place the final records
/// Original: `AfricanNations::update_staff_records_for_status` (0x401ba0, african_nations.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x401ba0 } (thiscall, ret 0x18, african_nations.cpp; a virtual +0x44): `AfricanNations::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: national_tournament_staff_records com 3 fases de grupos (o
/// mata-mata na fase 3); o desfecho 4 não faz nada.
pub(crate) unsafe extern "thiscall" fn african_nations_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    national_tournament_staff_records(0, this, clube, desfecho, fase, jogo, a6, 3)
}

// ponytail: os registros vão para o histórico dos técnicos e encerram as convocações: não rodam duas vezes, e a rotina
// não grava nada além deles. Só a original; o plano confere a devolução 0.
/// Comparer for `african_nations_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("african_nations_update_staff_records_for_status({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}


// ponytail: como a da Copa Africana: só a original, a devolução 0.

/// Ends the national team's current competition (NationalTeams::clear_current_competition for the club's nation).
///
/// [pt] A competição atual da seleção acaba (0x775000 em [0xb74318]: push nação; mov ecx -> thiscall, ret 4).
pub(crate) unsafe fn clear_national_team_competition(clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_CLEAR_CURRENT_COMPETITION);
    f(read_u32_as_usize(G_NATIONAL_TEAMS), read_u32_as_usize(clube + club::NATION));
}

/// rule: in the Oceania qualifier groups, a team going through gets the qualification (weight 20) and one going out the
/// failed-to-qualify record and ends its campaign; in the Oceania final, the winner gets the qualification (weight 30)
/// and the loser the knock-out; in the play-off (against South America [inference]), the winner gets the World Cup
/// place (weight 100 record, the entry in its list and status 4) and the loser the knock-out (weight 100) and the end
/// of the campaign
/// Original: `WcOceaniaLeague::update_staff_records_for_status` (0x91e840, wc_oceania_league.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x91e840 } (thiscall, ret 0x18, wc_oceania_league.cpp; a virtual +0x44): `WcOceaniaLeague::
/// update_staff_records_for_status(seleção, desfecho, fase, saida, jogo, _)`; devolve 0. Fase (byte com sinal) 0 ou
/// menos, os grupos: 6, a classificação (fase 0, rodada 0x82, peso 0x14); 0xfe, a não classificação (0x41c, 0x14) e a
/// competição atual da seleção acaba. Fase 1 (a final da Oceania): 1, a classificação (0xa0, 0, 0x1e); 2 ou 0xff, a
/// eliminação (+0x32 e +0x30 do jogo, peso 0x1e) e a competição acaba. Fase 2 (a repescagem): 1, a classificação para
/// a Copa do Mundo (a competição [0xae23d0] + 0x6b × [0x9cf79c], 0, 0, 0x64), a vaga nela (0x92e740 no objeto
/// [[0xadadfc] + 4 × [0x9cf79c]]: push seleção -> thiscall, ret 4) e *saida = 4; 2, a eliminação (peso 0x64) e a
/// competição acaba.
pub(crate) unsafe extern "thiscall" fn african_nations_update_staff_records_for_status_91e840(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, _a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    let eliminar = |peso: u32| {
        // ponytail: +0x30 e +0x32 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
        european_cup_add_knocked_out_record(clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
        clear_national_team_competition(clube);
    };
    if f < 1 {
        if d == 6 {
            european_cup_add_qualified_record(clube, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x14);
            clear_national_team_competition(clube);
        }
    } else if f == 1 {
        if d == 1 {
            european_cup_add_qualified_record(clube, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            crate::comparacao::ramo("cqj_10 oceania na copa do mundo");
            let id = read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
            european_cup_add_qualified_record(clube, read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            let vaga: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_WORLDCUP_ADD_QUALIFIED_TEAM);
            vaga(read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))), clube);
            *(saida as *mut u8) = 4;
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

// ponytail: os registros, o fim da competição da seleção e a vaga na Copa do Mundo não rodam duas vezes. Só a original;
// o plano: a devolução 0 e, na repescagem ganha, *saida = 4 e a lista da Copa do Mundo (+0x36) um a mais (se cabia).
/// Comparer for `african_nations_update_staff_records_for_status_91e840` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_staff_records_for_status_91e840(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_STAFF_RECORDS_FOR_STATUS_91E840);
    let vaga = fase as u8 == 2 && desfecho as u8 == 1 && saida != 0;
    let mundial = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
    if vaga && mundial == 0 {
        return o(this, clube, desfecho, fase, saida, jogo, a6);
    }
    let n = if vaga { read_i16(mundial + 0x36) } else { 0 };
    let cabe = vaga && n < read_i16(mundial + 0x38);
    let r = o(this, clube, desfecho, fase, saida, jogo, a6);
    let igual = r == 0 && (!vaga || (*(saida as *const u8) == 4 && read_i16(mundial + 0x36) == if cabe { n + 1 } else { n }));
    PLACAR.registrar(igual, || format!("african_nations_update_staff_records_for_status_91e840({:#x}, {}, {}) devolveu {} vaga={} lista {}", clube, desfecho as u8, fase as u8, r, vaga, n));
    r
}

/// rule: the African Nations Cup has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 8, 12 and 16
/// February, with each group's fixed matches; the knockout (stage 3) in single matches: quarter-finals on 21/2 with
/// fixed pairings, semi-final on 25/2, third place on 27/2 and final on 28/2 (venue 4); category 10; tie-break 0x303 up
/// to the semi-final and 0x203 in the final; no prizes
/// Original: `AfricanNations::create_schedule` (0x4012e0, african_nations.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4012e0 } (thiscall, ret 0x10, african_nations.cpp; a virtual +0x3c da Copa Africana): `AfricanNations::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `african_nations_phase`.
pub(crate) unsafe extern "thiscall" fn african_nations_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    build_phase_table(this, african_nations_phase(fase), rodadas, extra, liga, 0x9870f8)
}

/// Comparer for `african_nations_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_AFRICAN_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("african_nations_create_schedule", fase, african_nations_phase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| african_nations_create_schedule(this, fase, r, e, l))
}

/// rule: the African Nations Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group,
/// category 10, 7 substitutes and 3 changes; the date of 5 of month 9 of the year before the tournament on weekday 5
/// [inference: the draw in the previous year] and +0xb8 = −1
/// Original: `AfricanNations::set_up_rules` (0x401ab0, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x401ab0 } (thiscall, ret 0, african_nations.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `AfricanNations::set_up_rules()`: os grupos da Copa Africana em turno único, 3/1 pontos, a categoria 0xa, os
/// desempates (1, 2, 3), dois classificados por grupo, seleções, 7 no banco e 3 substituições, a data em 5 do mês 9 do
/// ano anterior ao da competição num dia da semana 5 [inferência: o sorteio] e +0xb8 = −1.
pub(crate) unsafe extern "thiscall" fn african_nations_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da copa africana");
    let vitoria = 3; // cqk10: a vitória nos grupos da Copa Africana
    goldcup_write_rule_fields(this, &CQK10_AFRICANA, vitoria);
    let dia = 5; // cqk10: o dia da data da Copa Africana
    let palavra = competition_date_word(this, dia, 9, -1, 5);
    finish_rules_setup(this, palavra, 0xffff)
}

/// Comparer for `african_nations_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AFRICAN_NATIONS_SET_UP_RULES);
    check_set_up_rules("african_nations_set_up_rules", this, &CQK10_AFRICANA, (5, 9, -1, 5), 0xffff, || o(this))
}
