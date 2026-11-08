use super::*;

// ---------- 284a leva: a Liga dos Campeões asiática: a fase seguinte, os dois grupos e as semifinais (asia_club_champ.cpp) ----------




/// { 0x40c070 } (thiscall, ret 0, asia_club_champ.cpp; a virtual +0x28): `AsiaClubChamp::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x355; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), os dois grupos (0x40c110), chegando a 3 (depois dos grupos), as semifinais (0x40c3d0); as duas mov
/// ecx, this -> thiscall, ret 0.
pub(crate) unsafe extern "thiscall" fn asia_club_champ_advance_stage(this: usize) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x987a40, 0x355);
    }
    let f = f + 1;
    write_u32_from_usize(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_12 grupos da asia");
        let grupos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_ASIACLUBCHAMP_CREATE_GROUP_STAGES);
        grupos(this);
    } else if f == 3 {
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_ASIACLUBCHAMP_CREATE_SEMI_FINAL_STAGE);
        semifinais(this);
    }
}

// ponytail: os grupos e as semifinais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um
// (os grupos gravam 2, o mesmo valor) fora da última fase, que dá a caixa e não muda nada.
pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_advance_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    let esperado = if ultima { f } else if f + 1 == 1 { 2 } else { f + 1 };
    PLACAR.registrar(depois == esperado, || format!("asia_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 8 dos grupos: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da copa, +0xa7) até 4
/// e depois os do mata-mata da segunda zona (a tabela +0xa7 da fase [+0xc][0]) até 8; as duas buscas vão até o +0x3e da
/// copa (palavra com sinal). Devolve os 8 (as vagas que sobrarem ficam 0: a original as deixa com sobra da pilha).
pub(crate) unsafe fn cqt12_vivos(this: usize) -> [usize; 8] {
    let mut v = [0usize; 8];
    let mut n = 0usize;
    let times = read_i16(this + 0x3e);
    let mut i = 0i16;
    while i < times && n < 4 {
        let e = read_u32_as_usize(this + 0xa7) + i as usize * 6;
        if *((e + 5) as *const u8) == 1 {
            v[n] = read_u32_as_usize(e); // cqt12: um vivo da primeira zona
            n += 1;
        }
        i += 1;
    }
    let mut i = 0i16;
    while i < times && n < 8 {
        let e = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc)) + 0xa7) + i as usize * 6;
        if *((e + 5) as *const u8) == 1 {
            v[n] = read_u32_as_usize(e);
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
pub(crate) unsafe fn cqt12_grupo(this: usize, g: u32, times: *const usize, a17: u32) -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let regras_a: [u8; 4] = [1, 2, 0, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
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
            read_u32_as_usize(this + 4),
            tabela,
            rodadas as u16 as u32,
            3,
            1,
            8,
            regras_a.as_ptr(),
            regras_b.as_ptr(),
            read_i16(this + 0x40) as u16 as u32,
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
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + g as usize * 4, fase);
    soltar(tabela);
    read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4)
}

/// { 0x40c110 } (thiscall, ret 0, asia_club_champ.cpp; quadro SEH só para o new das fases: sem destrutor no caminho
/// normal): `AsiaClubChamp::create_group_stages()`: os 8 vivos das duas zonas (cqt12_vivos); o grupo 1 (fase 1, os 4
/// primeiros, o 17º argumento 2; sem fase, a caixa da linha 0x39e e volta) e a notícia dele (0x684230 na fase, thiscall,
/// ret 0); o grupo 2 (fase 2, os 4 seguintes, o 17º argumento 1; sem fase, a caixa da linha 0x3b4 e volta), a notícia e
/// +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn create_group_stages(this: usize) {
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
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
    write_u32_from_usize(this + 0x30, 2);
}

// ponytail: as fases são alocadas e as notícias vão aos técnicos: não roda duas vezes. Só a original; o plano: os vivos
// lidos antes; depois as fases [+0xc][1] e [+0xc][2] com 4 times, o ano da copa e os 4 vivos de cada (em qualquer ordem:
// o construtor ordena a tabela) e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_create_group_stages(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_GROUP_STAGES);
    if read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let v = cqt12_vivos(this);
    o(this);
    let confere = |g: usize, de: &[usize]| {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        if t == 0 || read_i16(f + 0x3e) != 4 || read_i16(f + 0x40) != read_i16(this + 0x40) {
            return false;
        }
        let mut a: Vec<usize> = (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect();
        let mut b = de.to_vec();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    let ok = confere(1, &v[..4]) && confere(2, &v[4..]) && read_u32_as_usize(this + 0x30) == 2;
    PLACAR.registrar(ok, || format!("create_group_stages({:#x}): os grupos não batem com os vivos {:x?}", this, v));
}

/// Os 4 das semifinais, na ordem da lista: o 1º do grupo 1 ([+0xc][1], tabela +0xb1), o 2º do grupo 2 ([+0xc][2]), o
/// 1º do grupo 2 e o 2º do grupo 1 (os cruzamentos: as chaves são os pares da lista).
pub(crate) unsafe fn cqt12_semifinalistas(this: usize) -> [usize; 4] {
    let l = |g: usize, p: usize| read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc) + g * 4) + 0xb1) + p * 0x3b);
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
pub(crate) unsafe extern "thiscall" fn asia_club_champ_create_semi_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(0x987a40, 0x3d1);
    }
    crate::comparacao::ramo("cqt_12 cruzamentos da asia");
    for (k, &c) in cqt12_semifinalistas(this).iter().enumerate() {
        write_u32_from_usize(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 3, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 4, lista, rodadas as u16 as u32, read_u32_as_usize(this + 4), tabela, read_i16(this + 0x40) as u16 as u32, 3, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + 0xc, fase);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if f == 0 {
        return aviso_em(0x987a40, 0x3e6);
    }
    sortear(f, 0);
    soltar(lista);
}

// ponytail: aloca a fase e sorteia a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][3] com 4
// times, o +4 da copa e os 4 cruzados (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio pode
// reordenar).
pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_create_semi_final_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_CREATE_SEMI_FINAL_STAGE);
    let fases = read_u32_as_usize(this + 0xc);
    if fases == 0 || read_u32_as_usize(fases + 4) == 0 || read_u32_as_usize(fases + 8) == 0 {
        return o(this);
    }
    let mut plano = cqt12_semifinalistas(this).to_vec();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("asia_club_champ_create_semi_final_stage({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("asia_club_champ_create_semi_final_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

// ---------- 281a leva: o calendário das copas de clubes da Ásia e do Troféu da Liga inglesa: a Liga dos Campeões e a Recopa asiáticas e o LDV Vans Trophy (asia_club_champ.cpp, asia_cup_winner.cpp, eng_auto_cup.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
pub(crate) enum Cqp11Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
pub(crate) use Cqp11Op::{B as Cqp11B, D as Cqp11D, E as Cqp11E, J as Cqp11J, L as Cqp11L, S as Cqp11S, W as Cqp11W, X as Cqp11X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações e o nome do contador. `None`: a fase devolve 0 sem gravar nada.
#[derive(Clone, Copy)]
pub(crate) struct Cqp11Fase {
    pub(crate) liga: i32,
    pub(crate) rodadas: i16,
    pub(crate) extra: i16,
    pub(crate) extra_checa: bool,
    pub(crate) tamanho: u32,
    pub(crate) linha: u32,
    pub(crate) ops: &'static [Cqp11Op],
    pub(crate) nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
pub(crate) unsafe fn cqp11_tabela(this: usize, f: Option<Cqp11Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(f.tamanho);
    if t == 0 {
        aviso_em(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DAY);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops {
        match *op {
            Cqp11D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp11L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqp11S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqp11J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp11E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp11B(o, v) => *((t + o) as *mut u8) = v,
            Cqp11W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqp11X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
pub(crate) unsafe fn cqp11_gravados(ops: &[Cqp11Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops {
        match *op {
            Cqp11D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp11L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp11S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp11J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp11E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp11B(o, _) => pega(o, 1),
            Cqp11W(o, _) => pega(o, 2),
            Cqp11X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
pub(crate) unsafe fn cqp11_comparar(nome: &str, fase: u32, f: Option<Cqp11Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqp11_gravados(f.ops, a) == cqp11_gravados(f.ops, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Liga dos Campeões asiática, eliminatórias de cada zona: 1ª rodada sorteada em 1/8 e play em 23/8, 2ª em 14/9 e 20/9, ida e volta.
pub(crate) const CQP11_ASIA_CLUBES_MATA: [Cqp11Op; 36] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0),
];
/// A Liga dos Campeões asiática, grupos finais: 3 rodadas (21, 23 e 25 de março do ano seguinte, valor 1) com os jogos de cada rodada.
pub(crate) const CQP11_ASIA_CLUBES_GRUPOS: [Cqp11Op; 12] = [
    Cqp11D(0, 0x15, 2, 1, 2, 1, 1), Cqp11L(0, 0, 2, 2, 3, 0x10), Cqp11L(0, 1, 2, 1, 3, 0x10),
    Cqp11L(0, 2, -1, -1, -1, 0), Cqp11D(1, 0x17, 2, 1, 4, 1, 1), Cqp11L(1, 0, 4, 2, 3, 0x10),
    Cqp11L(1, 1, 4, 1, 3, 0x10), Cqp11L(1, 2, -1, -1, -1, 0), Cqp11D(2, 0x19, 2, 1, 6, 1, 1),
    Cqp11L(2, 0, 6, 2, 3, 0x10), Cqp11L(2, 1, 6, 1, 3, 0x10), Cqp11L(2, 2, -1, -1, -1, 0),
];
/// A Liga dos Campeões asiática, fase final em maio do ano seguinte: semifinal (com os confrontos 0x521f10), terceiro lugar e final.
pub(crate) const CQP11_ASIA_CLUBES_FINAL: [Cqp11Op; 57] = [
    Cqp11S(0, 0x1a, 2, 1, 0), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10),
    Cqp11E(0, 1, 3, 2, 3, 0x10), Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x383), Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2),
    Cqp11W(0xea, 1), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, eliminatórias: 1ª rodada em 23/8, 2ª em 20/9, quartas em 24 de março do ano seguinte, ida e volta.
pub(crate) const CQP11_ASIA_RECOPA_MATA: [Cqp11Op; 54] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x1c, 8, 0, 3), Cqp11J(2, 0x18, 2, 1, 2, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11W(0xdf, 0x83), Cqp11B(0xe7, 4), Cqp11W(0xe8, 4),
    Cqp11W(0xea, 2), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, fase final em maio do ano seguinte: semifinal, terceiro lugar e final (e a cauda comum +0x12c..+0x137).
pub(crate) const CQP11_ASIA_RECOPA_FINAL: [Cqp11Op; 57] = [
    Cqp11S(0, 0xf, 3, 1, 6), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10), Cqp11E(0, 1, 3, 2, 3, 0x10),
    Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x383),
    Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0), Cqp11W(0x1e, 0),
    Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0), Cqp11X(0x64, 0),
    Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1), Cqp11W(0x73, 0),
    Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1), Cqp11W(0x84, 0),
    Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0),
    Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c), Cqp11W(0xd9, 2),
    Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0), Cqp11X(0x12c, 0),
    Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// O LDV Vans Trophy, cada chave: 5 rodadas de dezembro a março do ano seguinte, a última (a final da chave) em ida e volta.
pub(crate) const CQP11_TROFEU_CHAVES: [Cqp11Op; 90] = [
    Cqp11S(0, 7, 0xa, 0, 5), Cqp11J(0, 0xa, 0xb, 0, 1, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 4), Cqp11W(0x18, 0x18), Cqp11W(0x1a, 0xc), Cqp11W(0x1c, 0x18), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xb, 0xb, 0, 2), Cqp11J(1, 0xe, 0, 1, 1, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 4), Cqp11W(0x80, 0x10), Cqp11W(0x82, 8),
    Cqp11W(0x84, 4), Cqp11W(0x86, 0x18), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0xf, 0, 1, 2), Cqp11J(2, 4, 1, 1, 1, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 4), Cqp11W(0xe8, 8),
    Cqp11W(0xea, 4), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0), Cqp11S(3, 5, 1, 1, 2), Cqp11J(3, 0x12, 1, 1, 1, 2, 0),
    Cqp11W(0x13f, 0x82), Cqp11W(0x141, 3), Cqp11W(0x143, 0), Cqp11W(0x145, 0x83), Cqp11W(0x147, 0), Cqp11B(0x14f, 4),
    Cqp11W(0x150, 4), Cqp11W(0x152, 2), Cqp11W(0x154, 0), Cqp11W(0x156, 0), Cqp11B(0x158, 0), Cqp11B(0x159, 1),
    Cqp11B(0x15a, 0), Cqp11X(0x194, 0), Cqp11X(0x198, 0), Cqp11X(0x19c, 0), Cqp11S(4, 0x13, 1, 1, 2),
    Cqp11J(4, 4, 2, 1, 1, 2, 0), Cqp11W(0x1a7, 0x96), Cqp11W(0x1a9, 4), Cqp11W(0x1ab, 0), Cqp11W(0x1ad, 0),
    Cqp11W(0x1af, 0x83), Cqp11B(0x1b7, 6), Cqp11W(0x1b8, 2), Cqp11W(0x1ba, 1), Cqp11W(0x1bc, 0), Cqp11W(0x1be, 0),
    Cqp11B(0x1c0, 0), Cqp11B(0x1c1, 2), Cqp11B(0x1c2, 7), Cqp11X(0x1fc, 0), Cqp11X(0x200, 0), Cqp11X(0x204, 0),
];
/// O LDV Vans Trophy, a final: sorteio em 12/3 e jogo em 20/4 do ano seguinte, mando 4 (estádio neutro [inferência]).
pub(crate) const CQP11_TROFEU_FINAL: [Cqp11Op; 18] = [
    Cqp11S(0, 0xc, 2, 1, 3), Cqp11J(0, 0x14, 3, 1, 6, 1, 4), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 6), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0xffff_ffff),
    Cqp11X(0x64, 0),
];

/// A Liga dos Campeões asiática [inferência: o Campeonato Asiático de Clubes], as fases: −1 e 0 as eliminatórias de
/// cada zona (2 rodadas de ida e volta em agosto e setembro), 1 e 2 os grupos finais (3 rodadas em março), 3 a fase
/// final (semifinal, terceiro lugar e final em maio).
pub(crate) fn cqp11_asia_clubes_fase(fase: u32) -> Option<Cqp11Fase> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(Cqp11Fase { liga: 0, rodadas: 2, extra: 0x480 + (f != -1) as i16, extra_checa: false, tamanho: 0xd0, linha: 0xc5,
            ops: &CQP11_ASIA_CLUBES_MATA, nome: "cqp_11 eliminatorias da liga dos campeoes asiatica" }),
        1 | 2 => Some(Cqp11Fase { liga: 1, rodadas: 3, extra: 0x482 + (f != 1) as i16, extra_checa: false, tamanho: 0xc3, linha: 0x103,
            ops: &CQP11_ASIA_CLUBES_GRUPOS, nome: "cqp_11 grupos da liga dos campeoes asiatica" }),
        3 => Some(Cqp11Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: true, tamanho: 0x138, linha: 0x125,
            ops: &CQP11_ASIA_CLUBES_FINAL, nome: "cqp_11 final da liga dos campeoes asiatica" }),
        _ => None,
    }
}

/// rule: the Asian Club Championship has, in each zone (stages −1 and 0, ids 0x480 and 0x481), two two-legged
/// qualifying rounds (1st drawn on 1/8 and played on 23/8, 2nd on 14/9 and 20/9; 21 days between legs in the 1st, 7 in
/// the 2nd); the final groups (stages 1 and 2, ids 0x482 and 0x483) of three rounds on 21, 23 and 25 March of the
/// following year with fixed pairings (matches 0 and 1 of each round); the final stage (stage 3) in May: semi-final on
/// 24/5, third place and final on 26/5, single matches (tie-break 0x383 and 0x83); category 4 in the qualifiers and 6
/// after
/// Original: `AsiaClubChamp::create_round_table` (0x40ae90, asia_club_champ.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x40ae90 } (thiscall, ret 0x10, asia_club_champ.cpp; a virtual +0x3c): `AsiaClubChamp::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqp11_asia_clubes_fase`.
pub(crate) unsafe extern "thiscall" fn asia_club_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqp11_asia_clubes_fase(fase), rodadas, extra, liga, 0x987a40)
}

pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_CLUB_CHAMP_CREATE_ROUND_TABLE);
    cqp11_comparar("asia_club_champ_create_round_table", fase, cqp11_asia_clubes_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| asia_club_champ_create_round_table(this, fase, r, e, l))
}

/// A Recopa asiática, as fases: −1 e 0 as eliminatórias de cada zona (3 rodadas de agosto a março), 1 a fase final
/// (semifinal, terceiro lugar e final em maio).
pub(crate) fn cqp11_asia_recopa_fase(fase: u32) -> Option<Cqp11Fase> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(Cqp11Fase { liga: 0, rodadas: 3, extra: 0x480 + (f != -1) as i16, extra_checa: false, tamanho: 0x138, linha: 0xc6,
            ops: &CQP11_ASIA_RECOPA_MATA, nome: "cqp_11 eliminatorias da recopa asiatica" }),
        1 => Some(Cqp11Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: true, tamanho: 0x138, linha: 0x112,
            ops: &CQP11_ASIA_RECOPA_FINAL, nome: "cqp_11 final da recopa asiatica" }),
        _ => None,
    }
}



/// O LDV Vans Trophy (o troféu da liga inglesa de baixo), as fases: −1 e 0 as chaves do Norte e do Sul (5 rodadas em
/// jogo único de dezembro a fevereiro e a final da chave em ida e volta em março), 1 a final (abril, mando 4).
pub(crate) fn cqp11_trofeu_fase(fase: u32) -> Option<Cqp11Fase> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(Cqp11Fase { liga: 0, rodadas: 5, extra: 0x41d + (f != -1) as i16, extra_checa: false, tamanho: 0x208, linha: 0xca,
            ops: &CQP11_TROFEU_CHAVES, nome: "cqp_11 chaves do trofeu" }),
        1 => Some(Cqp11Fase { liga: 0, rodadas: 1, extra: 0, extra_checa: false, tamanho: 0x68, linha: 0x13d,
            ops: &CQP11_TROFEU_FINAL, nome: "cqp_11 final do trofeu" }),
        _ => None,
    }
}

/// rule: the Asian Club Championship has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with
/// class and status 0
/// Original: `AsiaClubChamp::build_team_table` (0x40b620, asia_club_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x40b620 } (thiscall, ret 0, asia_club_champ.cpp): `AsiaClubChamp::build_team_table()`: os 16 times (a caixa da
/// linha 0x1cd sem memória).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_build_team_table(this: usize) -> i32 {
    cqq12_times(this, 0x987a40, 0x1cd, "cqq_12 times da liga dos campeoes asiatica", |i| i) // cqq12: os times da Liga dos Campeões asiática
}

pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_build_team_table(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ASIA_CLUB_CHAMP_BUILD_TEAM_TABLE);
    cqq12_comparar_times("asia_club_champ_build_team_table", this, || o(this), || asia_club_champ_build_team_table(this))
}
