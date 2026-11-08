use super::*;

// ---------- 282a leva: a Mercosul: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_merc.cpp) ----------




/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
pub(crate) unsafe fn cqr11_sorteado(this: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) // cqr11: o sorteado k
}

/// rule: the Mercosur has 20 invited clubs (the clubs the South American rules invite, shuffled) in 5 groups of 4
/// [inference: six rounds on Tuesdays, from 29 July to 14 October]; the season turns on 1 June; on the draw day group 1
/// stays in the cup's own table (drawn clubs 0 to 3) and groups 2 to 5 are stages (4g + 4 to 4g + 7); the 5 winners and
/// the 3 best runners-up go to the quarter-finals; on the draw day: the clubs (0x4c5850 (the 20 invited clubs,
/// shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted positions;
/// then the league day
/// Original: `ConmebolMerc::update_for_date` (0x4c5c90, conmebol_merc.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c5c90 } (thiscall, ret 8, conmebol_merc.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Mercosul.
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
pub(crate) unsafe extern "thiscall" fn cqr11_mercosul_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONMEBOLMERC_SELECT_INVITED_CLUBS);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONMEBOLMERC_CREATE_FIRST_GROUP_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CONMEBOLMERC_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    let mut junho = [0u8; 8];
    data(junho.as_mut_ptr(), 1, 5, ano_de_hoje as u32, -1);
    if core::ptr::read_unaligned(junho.as_ptr().add(2) as *const u16) == ano_de_hoje
        && core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(hoje as *const u16)
        && read_i16(this + 0x40) as i32 != ano_de_hoje as i32
    {
        let virada: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 8));
        virada(this);
    }
    if read_u32_as_usize(this + 0x45) == 0 && completo != 0 {
        let dia = read_i16(this + 0xb6) as u16; // cqr11: o dia do sorteio
        let ano = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparacao::ramo("cqr_11 sorteio da mercosul");
            clubes(this);
            if primeiro(this) == 0 {
                aviso_em(0x9a7e5c, 0x396);
            }
            if jogos(this) == 0 {
                aviso_em(0x9a7e5c, 0x39b);
            }
            if ordenar(this, 0) == 0 {
                aviso_em(0x9a7e5c, 0x3a1);
            }
            for g in 0..4i32 {
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
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 4 fases
// criadas e a fase atual (+0x30) a última delas.
pub(crate) unsafe extern "thiscall" fn comparar_cqr11_mercosul_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2009);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && completo != 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16) && !cqr11_virada_hoje(this, hoje);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && (0..4).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 3;
        PLACAR.registrar(ok, || format!("cqr11_mercosul_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// O dia da virada (1º de junho do ano de hoje, com a copa de outro ano): o comparador não confere o sorteio nesse dia.
pub(crate) unsafe fn cqr11_virada_hoje(this: usize, hoje: usize) -> bool {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    let mut junho = [0u8; 8];
    data(junho.as_mut_ptr(), 1, 5, ano_de_hoje as u32, -1);
    core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(hoje as *const u16) && read_i16(this + 0x40) as i32 != ano_de_hoje as i32
}

/// rule: group 1 of the Copa Mercosur has 4 clubs, the first 4 drawn, each in the row of its order; human managers get
/// the group draw news
/// Original: `ConmebolMerc::create_first_group_table` (0x4c4e60, conmebol_merc.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c4e60 } (thiscall, ret 0, conmebol_merc.cpp): a tabela do primeiro grupo da Mercosul: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x155 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
pub(crate) unsafe extern "thiscall" fn cqr11_mercosul_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9a7e5c, 0x155);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + 0x3e) {
        crate::comparacao::ramo("cqr_11 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr11_sorteado(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
pub(crate) unsafe extern "thiscall" fn comparar_cqr11_mercosul_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2010);
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
    PLACAR.registrar(igual, || format!("cqr11_mercosul_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// rule: each of the other groups of the Copa Mercosur is a league stage with the next 4 drawn clubs (4g + 4 to 4g +
/// 7), the cup's rules and year and the cup's round table for the group; human managers get the news
/// Original: `ConmebolMerc::create_group_stage` (0x4c51d0, conmebol_merc.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c51d0 } (thiscall, ret 4, conmebol_merc.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 3, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x1fb e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x211 e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
pub(crate) unsafe extern "thiscall" fn cqr11_mercosul_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let times = alocar(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0x9a7e5c, 0x1fb);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparacao::ramo("cqr_11 time do grupo");
        write_u32_from_usize(times + n * 4, cqr11_sorteado(this, k as usize)); // cqr11: o time na vaga do grupo
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
        aviso_em(0x9a7e5c, 0x211);
        return 0;
    }
    noticia(f);
    write_u32_from_usize(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
pub(crate) unsafe extern "thiscall" fn comparar_cqr11_mercosul_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2011);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 || !(0..4).contains(&g) {
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
        PLACAR.registrar(ok, || format!("cqr11_mercosul_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 279a leva: as copas sul-americanas de clubes: o calendário da Libertadores e da Mercosul e os convidados da Mercosul (conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
pub(crate) type Cqo13G = (u32, u32, u32, u32);

/// As duas fases da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10; a fase lida
/// como byte com sinal) das copas sul-americanas. Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1,
/// rodadas = o número de rodadas de grupo, extra = fase + 0x3e9 (16 bits); a tabela de 0x41 bytes por rodada (0x944e46;
/// sem memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0,
/// rodadas = o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as
/// rodadas pela `cql10_rodada` com o ano +0x40 da copa (com sinal) e as palavras de `a_mais` (rodada, deslocamento,
/// valor). Acima de `limite`: devolve 0 sem gravar nada.
pub(crate) unsafe fn cqo13_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, grupos: &[Cqo13G],
    mata: &[Cql10R], a_mais: &[(usize, usize, u16)], arquivo: usize, linha_g: u32, linha_m: u32, nome_g: &'static str, nome: &'static str) -> usize {
    let f = fase as u8 as i8;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    if f < limite {
        let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
        let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
        if !liga.is_null() {
            *liga = 1;
        }
        *rodadas = grupos.len() as i16;
        *extra = (f as i16).wrapping_add(0x3e9);
        let t = alocar(grupos.len() as u32 * 0x41);
        if t == 0 {
            aviso_em(arquivo, linha_g);
            return 0;
        }
        crate::comparacao::ramo(nome_g);
        // o ano vai com lixo nos bytes altos na original (o registrador de antes com a palavra +0x40 por cima); a
        // 0x68a160 só usa os 16 bits de baixo: a cópia passa o ano limpo
        let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
            data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, ano, 0);
            jogo(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limite {
        return 0;
    }
    if !liga.is_null() {
        *liga = 0;
    }
    *rodadas = mata.len() as i16;
    *extra = 0;
    let t = alocar(mata.len() as u32 * 0x68);
    if t == 0 {
        aviso_em(arquivo, linha_m);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in mata.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    for &(r, o, v) in a_mais {
        core::ptr::write_unaligned((t + r * 0x68 + o) as *mut u16, v);
    }
    t
}

/// Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
pub(crate) unsafe fn cqo13_gravados_grupos(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (grupos ou mata-mata, este com as
/// palavras de `a_mais`) têm de ser iguais; a tabela da cópia é liberada (0x9452ca). Acima do mata-mata só a original.
pub(crate) unsafe fn cqo13_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, a_mais: &[(usize, usize, u16)],
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = fase as u8 as i8;
    if f > limite {
        // ponytail: acima do mata-mata nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, f, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let mais = |t: usize| a_mais.iter().flat_map(|&(r, o, _)| core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2).to_vec()).collect::<Vec<u8>>();
    let tabela = if a != 0 && b != 0 {
        if f < limite {
            cqo13_gravados_grupos(a, *rodadas) == cqo13_gravados_grupos(b, r2)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, r2) && mais(a) == mais(b)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, f, saidas, *rodadas, r2, tabela));
    a
}

/// A Libertadores, grupos: seis rodadas às terças [inferência: dia da semana 2], de 25 de fevereiro a 1º de abril.
pub(crate) const CQO13_LIBERTADORES_GRUPOS: [Cqo13G; 6] = [(0x19, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // cqo13: os grupos da Libertadores

/// A Libertadores, mata-mata: quatro rodadas em ida e volta, de abril a junho (o jogo de volta 14 dias depois nas
/// oitavas e 7 nas outras).
pub(crate) const CQO13_LIBERTADORES: [Cql10R; 4] = [
    cql10r((2, 3, 0, 3), (0xf, 3, 0, 2, 2, 0), 0x14, (0, 4, 1), 7, (0x10, 8, 0x10, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo13: as oitavas da Libertadores
    cql10r((0x1e, 3, 0, 3), (0x14, 4, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x1c, 4, 0, 3), (3, 5, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 5, 0, 3), (0x11, 5, 0, 2, 2, 0), 0x96, (0, 4, 1), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0xc3500)),
];



/// A Mercosul, grupos: seis rodadas às terças, de 29 de julho a 14 de outubro.
pub(crate) const CQO13_MERCOSUL_GRUPOS: [Cqo13G; 6] = [(0x1d, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // cqo13: os grupos da Mercosul

/// A Mercosul, mata-mata: quartas, semifinal e final em ida e volta, de outubro a dezembro (a volta 3 dias depois nas
/// quartas e na semifinal, 7 na final).
pub(crate) const CQO13_MERCOSUL: [Cql10R; 3] = [
    cql10r((0xf, 9, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 8, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((5, 0xa, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x96, (0, 0x2000, 0x2000), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0)),
];

/// A final da Mercosul grava ainda a palavra +0x11 = 3 [inferência: o desempate da final, pênaltis].
pub(crate) const CQO13_MERCOSUL_A_MAIS: [(usize, usize, u16); 1] = [(2, 0x11, 3)];

/// rule: the Copa Mercosur starts with six-round groups on Tuesdays, from 29/7 to 14/10 (id 0x3e9 + stage, up to stage
/// 3: 5 groups [inference]); the knockout (stage 4) over two legs: quarter-finals drawn on 15/10 and played on 28/10 (8
/// teams, return 3 days later), semi-final 5/11 and 11/11 (return 3 days later), final draw 19/11 and match on 2/12
/// (return 7 days later, tie-break 0x2000 and the word +0x11 = 3 [inference: penalties]); category 8; prize of 2
/// million to the winner
/// Original: `ConmebolMerc::create_schedule` (0x4c4930, conmebol_merc.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c4930 } (thiscall, ret 0x10, conmebol_merc.cpp; a virtual +0x3c da Mercosul): `ConmebolMerc::create_schedule(
/// fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 4 (a caixa da linha 0x97), o mata-mata na fase 4 (a caixa
/// da linha 0xbd).
pub(crate) unsafe extern "thiscall" fn cqo13_mercosul_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo13_tabela(this, fase, rodadas, extra, liga, 4, &CQO13_MERCOSUL_GRUPOS, &CQO13_MERCOSUL, &CQO13_MERCOSUL_A_MAIS, 0x9a7e5c, 0x97, 0xbd,
        "cqo_13 grupos da mercosul", "cqo_13 mata-mata da mercosul")
}

pub(crate) unsafe extern "thiscall" fn comparar_cqo13_mercosul_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1919);
    cqo13_comparar("cqo13_mercosul_tabela", fase, rodadas, extra, liga, 4, &CQO13_MERCOSUL_A_MAIS, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo13_mercosul_tabela(this, fase, r, e, l))
}


/// { 0x4c5850 } (thiscall, ret 0, conmebol_merc.cpp): `ConmebolMerc::select_invited_clubs()`: +0xf2 = 20 e a lista de
/// ponteiros (0x944e46 de 0x50 em +0xee; sem memória, a caixa da linha 0x2c6 e 0); cada clube da tabela ([0xae23b4],
/// [0xae235c] − 2 × [0xae2350]) que as regras ([0xae1000], 0x4c6890: thiscall com o clube) convidam entra na lista, sem
/// conferir o espaço; a lista é embaralhada (0x4afb40 com +0xf2). Devolve 1.
pub(crate) unsafe fn cqo13_convidados(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x14);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0x50);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9a7e5c, 0x2c6);
        return (0, 0);
    }
    let convidado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONMEBOLSEEDING_IS_MERCOSUR_INVITED_CLUB);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let clubes = || (read_u32_as_usize(0xae235c) as i32).wrapping_sub((read_u32_as_usize(0xae2350) as i32).wrapping_mul(2));
    let mut k = 0i16;
    let mut i = 0i32;
    while i < clubes() {
        let c = read_u32_as_usize(0xae23b4).wrapping_add((i as isize * 0x245) as usize);
        if convidado(read_u32_as_usize(0xae1000), c) != 0 {
            core::ptr::write_unaligned((read_u32_as_usize(this + 0xee) + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    crate::comparacao::ramo("cqo_13 convidados da mercosul");
    // ponytail: a original empurra a palavra +0xf2 com lixo nos 16 bits altos (o eax de antes); a 0x4afb40 lê a palavra
    embaralhar(read_u32_as_usize(this + 0xee), core::ptr::read_unaligned((this + 0xf2) as *const u16) as u32); // cqo13: embaralha os convidados
    (1, k as i32)
}

/// rule: the Copa Mercosur is invitation-only: the 20 clubs of the South American rules' fixed list (0x4c6890), in club
/// table order, shuffled for the group draw
/// Original: `ConmebolMerc::select_invited_clubs` (0x4c5850, conmebol_merc.cpp, thiscall, ret 0).
///
/// [pt] regra: a Mercosul é só de convidados: os 20 clubes da lista fixa das regras sul-americanas (0x4c6890), na ordem da
/// tabela de clubes, embaralhados para o sorteio dos grupos
pub(crate) unsafe extern "thiscall" fn cqo13_mercosul_convidados(this: usize) -> i32 {
    cqo13_convidados(this).0
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nos convidados que ela gravou e liberada.
pub(crate) unsafe extern "thiscall" fn comparar_cqo13_mercosul_convidados(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1920);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(|| cqo13_convidados(this));
        lr.set(read_u32_as_usize(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqo13_mercosul_convidados({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", this, a, b, igual, lista, k));
    a
}

/// rule: the Mercosur moves on one stage when the current one ends; after the 4 group stages (on reaching 4) it builds
/// the quarter-finals; going past the last stage is an error
/// Original: `ConmebolMerc::advance_stage` (0x4c5140, conmebol_merc.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c5140 } (thiscall, ret 0, conmebol_merc.cpp; a virtual +0x28): `ConmebolMerc::advance_stage()`, a fase seguinte
/// da Mercosul: a caixa da linha 0x1d9 na última; chegando a 4 (depois dos 4 grupos-fase), as quartas (0x4c53d0).
pub(crate) unsafe extern "thiscall" fn cqs11_mercosul_fase_seguinte(this: usize) {
    cqs11_fase_seguinte(this, 0x9a7e5c, 0x1d9, 4, 0x4c53d0, "cqs_11 quartas da mercosul");
}

pub(crate) unsafe extern "thiscall" fn comparar_cqs11_mercosul_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2041);
    cqs11_comparar_fase_seguinte("cqs11_mercosul_fase_seguinte", this, || o(this))
}
