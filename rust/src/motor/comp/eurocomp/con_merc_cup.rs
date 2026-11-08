use super::*;

// ---------- 283a leva: os mata-matas da Libertadores e da Merconorte: os classificados dos grupos e a fase seguinte (conmebol_liber.cpp, con_merc_cup.cpp) ----------




/// Os classificados por grupo (+0xbe, byte com sinal).
pub(crate) unsafe fn cqs10_vagas(this: usize) -> i32 {
    *((this + 0xbe) as *const i8) as i32
}

/// O clube da linha i da tabela do grupo 1 (a da própria copa: +0xb1, linhas de 0x3b bytes).
pub(crate) unsafe fn cqs10_do_grupo_um(this: usize, i: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xb1) + i * 0x3b) // cqs10: a linha do grupo 1
}

/// O clube da posição j da fase f ([+0xc][f], um League_Stage): a linha pela 0x6826d0 (League::get_table_row: push j;
/// mov ecx, fase -> thiscall, ret 4) e o clube no +0 dela.
pub(crate) unsafe fn cqs10_da_fase(this: usize, f: usize, j: i32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + f * 4);
    read_u32_as_usize(linha(fase, j)) // cqs10: a posição j da fase
}

/// O mata-mata depois dos grupos (o mesmo molde na Libertadores e na Merconorte, conferido instrução a instrução: mudam
/// o tamanho da lista, as fases de grupo, a fase do mata-mata, o fonte e as linhas das caixas). A lista (0x944e46(4 ×
/// times): push; add esp, 4 -> cdecl; sem memória, a caixa `linha_sem_lista` e volta) com os +0xbe primeiros do grupo 1
/// (na ordem das linhas) e, de cada fase de grupo 0..`fases` − 1, os +0xbe primeiros (posições 0, 1...); a tabela das
/// rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push `mata`; mov ecx, this -> thiscall, ret 0x10); a
/// fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, `times`,
/// a lista, as rodadas, +4, a tabela, o ano +0x40, `mata`, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra
/// as rodadas, o id e o ano com lixo nos 16 bits altos, que o construtor não lê) em [+0xc][`mata`]; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa `linha_sem_fase`. Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round:
/// push 0; mov ecx, fase -> thiscall, ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe fn cqs10_mata_mata(this: usize, times: u32, fases: usize, mata: u32, arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    let lista = alocar(times * 4);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs10_vagas(this) {
        write_u32_from_usize(lista + n * 4, cqs10_do_grupo_um(this, i as usize));
        n += 1;
        i += 1;
    }
    for f in 0..fases {
        let mut j = 0i32;
        while j < cqs10_vagas(this) {
            crate::comparacao::ramo(ramo);
            write_u32_from_usize(lista + n * 4, cqs10_da_fase(this, f, j)); // cqs10: os classificados de cada fase
            n += 1;
            j += 1;
        }
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, mata, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, times, lista, rodadas as u16 as u32, read_u32_as_usize(this + 4), tabela, read_i16(this + 0x40) as u16 as u32, mata, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + mata as usize * 4, fase);
    soltar(lista);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + mata as usize * 4);
    if f == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    sortear(f, 0);
}

/// O plano do mata-mata (o comparador roda só a original: a lista, a tabela e a fase são alocadas e a rodada é
/// sorteada): os classificados lidos antes (o grupo 1 nas linhas, as fases pelas linhas da tabela +0xb1 delas, como a
/// 0x6826d0); depois a fase em [+0xc][`mata`] com +0x3e = `times`, o +4 da copa e os clubes da lista dela (+0xa7, 6
/// bytes) iguais aos do plano como conjunto (o sorteio pode reordenar).
pub(crate) unsafe fn cqs10_comparar_mata_mata(nome: &str, this: usize, times: usize, fases: usize, mata: usize, original: impl FnOnce()) {
    let q = cqs10_vagas(this).max(0) as usize;
    let mut plano: Vec<usize> = (0..q).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0xb1) + i * 0x3b)).collect();
    for f in 0..fases {
        let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + f * 4);
        for j in 0..q {
            plano.push(read_u32_as_usize(read_u32_as_usize(fase + 0xb1) + j * 0x3b));
        }
    }
    original();
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + mata * 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..times).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) as usize == times && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, read_i16(fase + 0x3e)));
}



/// rule: the Merconorte semi-finals have 4 clubs: the winner (+0xbe = 1) of each of the 4 groups, in a cup stage (stage
/// 3); the first round is drawn at once
/// Original: `ConMercCup::create_semi_final_stage` (0x4c2330, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c2330 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::create_semi_final_stage()`, as semifinais da
/// Merconorte: os 4 (o primeiro do grupo 1 e das 3 fases de grupo) na fase 3; caixas nas linhas 0x21d (sem lista) e
/// 0x23b (sem fase).
pub(crate) unsafe extern "thiscall" fn cqs10_merconorte_semifinais(this: usize) {
    cqs10_mata_mata(this, 4, 3, 3, 0x9a7d6c, 0x21d, 0x23b, "cqs_10 classificados da merconorte");
}

pub(crate) unsafe extern "thiscall" fn comparar_cqs10_merconorte_semifinais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQS10_MERCONORTE_SEMIFINAIS);
    cqs10_comparar_mata_mata("cqs10_merconorte_semifinais", this, 4, 3, 3, || o(this))
}

/// rule: the Merconorte moves on one stage when the current one ends; after the 3 group stages (on reaching 3) it
/// builds the semi-finals; going past the last stage is an error
/// Original: `ConMercCup::advance_stage` (0x4c20b0, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c20b0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x28): `ConMercCup::advance_stage()`, a fase seguinte da
/// Merconorte: na última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x1c7; senão +0x30 sobe e, chegando a 3
/// (depois dos grupos), as semifinais (0x4c2330: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cqs10_merconorte_fase_seguinte(this: usize) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x9a7d6c, 0x1c7);
    }
    let f = f + 1;
    write_u32_from_usize(this + 0x30, f as usize);
    if f == 3 { // cqs10: as semifinais da Merconorte depois dos grupos
        crate::comparacao::ramo("cqs_10 semifinais da merconorte");
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONMERCCUP_CREATE_SEMI_FINAL_STAGE);
        semifinais(this);
    }
}

// ponytail: as semifinais (0x4c2330) alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa e não muda nada).
pub(crate) unsafe extern "thiscall" fn comparar_cqs10_merconorte_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQS10_MERCONORTE_FASE_SEGUINTE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqs10_merconorte_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

// ---------- 283a leva: os convidados da Merconorte e a fase seguinte da Mercosul e da Copa América (con_merc_cup.cpp, conmebol_merc.cpp, goldcup.cpp) ----------



/// Os clubes de uma nação para a Merconorte: até 30, na ordem da tabela de clubes.
pub(crate) struct Cqs11Nacao {
    pub(crate) clubes: [usize; 30],
    pub(crate) n: u8,
}

impl Cqs11Nacao {
    fn novo() -> Self {
        Cqs11Nacao { clubes: [0; 30], n: 0 }
    }

    fn por(&mut self, c: usize) {
        if self.n < 0x1e {
            self.clubes[self.n as usize] = c;
            self.n += 1;
        }
    }
}

/// { 0x4c2560 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::select_invited_clubs()`: +0xf2 = 16 e a lista +0xee
/// (0x944e46(0x40): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x262 e devolve 0). Os clubes da tabela
/// ([0xae23b4], 0x245 bytes, [0xae235c] deles; a primeira seleção, 0x53b900 database_is_national_team: push clube; add
/// esp, 4 -> cdecl, encerra a busca) com nação (+0x53) vão, pelo id da nação (+0 dela), para as listas do México
/// ([0x9cf3d0]), da Colômbia ([0x9cf29c]), do Peru ([0x9cf428]) e da Bolívia ([0x9cf254]), até 30 em cada; os de outra
/// nação cuja liga (+0x57) é a [0x9cf590] (a MLS) vão para a lista da liga americana, até 12. Cada lista é ordenada pela
/// reputação e tem a faixa de cima embaralhada (0x4afbd0 comp_util_sort_and_shuffle_top_clubs: push quantos; push a
/// conta (word); push lista; add esp, 0xc -> cdecl; sorteia): México 4, Colômbia 5, Peru 3, Bolívia 2, a liga americana 2.
/// A lista: nas vagas 2..5 os 4 primeiros do México, 6..10 os 5 da Colômbia, 11..13 os 3 do Peru, 14..15 os 2 da
/// Bolívia; nas vagas 0 e 1, sem a competição da liga americana ([[0xadadfc] + 4 × [0x9cf728]] = 0) os 2 primeiros da
/// lista dela; com ela, o campeão do último registro da competição [0x9cf590] (0xae23d0 + 0x6b × id) e o da competição
/// +4 dela (0x4afd80 comp_util_get_latest_placed_club: push lugar; push competição; add esp -> cdecl), e, se forem o
/// mesmo clube, o vice da primeira na vaga 0. A lista embaralhada (0x4afb40(lista, 16): push; push; add esp, 8 -> cdecl;
/// 3 × 16 trocas sorteadas). Devolve 1.
/// A original guarda as listas na pilha sem zerá-las: com menos clubes que as vagas, ela lê sobra; a cópia lê zero
/// [inferência: nesta carreira as quatro nações e a MLS têm clubes que bastam; o comparador confere a lista].
pub(crate) unsafe fn cqs11_convidados(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let ordenar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    write_u32_from_usize(this + 0xf2, 0x10);
    let americana = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_US_OPEN_CUP) * 4);
    let lista = alocar(0x40);
    write_u32_from_usize(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0x9a7d6c, 0x262);
        return 0;
    }
    let (mut mexico, mut colombia, mut peru, mut bolivia) = (Cqs11Nacao::novo(), Cqs11Nacao::novo(), Cqs11Nacao::novo(), Cqs11Nacao::novo());
    let mut mls = [0usize; 12];
    let mut n_mls = 0i8;
    let mut i = 0i32;
    let mut c = read_u32_as_usize(G_CLUB_TABLE);
    while i < read_u32_as_usize(G_CLUB_COUNT) as i32 {
        if selecao(c) != 0 {
            break;
        }
        let nacao = read_u32_as_usize(c + 0x53);
        if nacao != 0 {
            let id = read_u32_as_usize(nacao);
            if id == read_u32_as_usize(KEY_NATION_MEXICO) {
                mexico.por(c);
            } else if id == read_u32_as_usize(KEY_NATION_COLOMBIA) {
                colombia.por(c);
            } else if id == read_u32_as_usize(KEY_NATION_PERU) {
                peru.por(c);
            } else if id == read_u32_as_usize(KEY_NATION_BOLIVIA) {
                bolivia.por(c);
            } else {
                let liga = read_u32_as_usize(c + 0x57);
                if liga != 0 && read_u32_as_usize(liga) == read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && n_mls < 0xc {
                    mls[n_mls as usize] = c;
                    n_mls += 1;
                }
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparacao::ramo("cqs_11 convidados da merconorte");
    ordenar(mexico.clubes.as_mut_ptr() as usize, mexico.n as u32, 4); // cqs11: os 4 do México
    ordenar(colombia.clubes.as_mut_ptr() as usize, colombia.n as u32, 5);
    ordenar(peru.clubes.as_mut_ptr() as usize, peru.n as u32, 3);
    ordenar(bolivia.clubes.as_mut_ptr() as usize, bolivia.n as u32, 2);
    ordenar(mls.as_mut_ptr() as usize, n_mls as i16 as u16 as u32, 2);
    let vaga = |k: usize, v: usize| write_u32_from_usize(read_u32_as_usize(this + 0xee) + k * 4, v);
    vaga(2, mexico.clubes[0]);
    vaga(3, mexico.clubes[1]);
    vaga(4, mexico.clubes[2]);
    vaga(5, mexico.clubes[3]);
    for k in 0..5 {
        vaga(6 + k, colombia.clubes[k]);
    }
    for k in 0..3 {
        vaga(11 + k, peru.clubes[k]);
    }
    vaga(14, bolivia.clubes[0]);
    vaga(15, bolivia.clubes[1]);
    if americana == 0 {
        vaga(0, mls[0]);
        vaga(1, mls[1]);
    } else {
        let registro = read_u32_as_usize(G_CLUB_COMP_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) * 0x6b;
        crate::comparacao::ramo("cqs_11 campeao da liga americana");
        vaga(0, colocado(registro, 0)); // cqs11: o campeão da liga americana
        vaga(1, colocado(read_u32_as_usize(americana + 4), 0));
        let l = read_u32_as_usize(this + 0xee);
        if read_u32_as_usize(l) == read_u32_as_usize(l + 4) {
            vaga(0, colocado(registro, 1));
        }
    }
    embaralhar(read_u32_as_usize(this + 0xee), 0x10);
    1
}

/// rule: the Merconorte invites 16: the best 4 of Mexico, 5 of Colombia, 3 of Peru and 2 of Bolivia (each list sorted
/// by reputation, with the top band shuffled: 3 times the places) and 2 from the American league (with its competition:
/// the latest MLS champion and that of its other competition, and the MLS runner-up if they are the same club; without
/// it, the 2 best MLS clubs); the final list is shuffled (3 × 16 swaps) before the groups
/// Original: `ConMercCup::select_invited_clubs` (0x4c2560, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: a Merconorte convida 16: os 4 melhores do México, os 5 da Colômbia, os 3 do Peru e os 2 da Bolívia (cada
/// lista ordenada pela reputação, com a faixa de cima embaralhada: 3 vezes as vagas) e 2 da liga americana (com a
/// competição dela: o campeão do último registro da MLS e o da outra competição dela, e o vice da MLS se forem o mesmo;
/// sem ela, os 2 melhores clubes da MLS); a lista final é embaralhada (3 × 16 trocas) antes dos grupos
pub(crate) unsafe extern "thiscall" fn cqs11_merconorte_convidados(this: usize) -> u32 {
    cqs11_convidados(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 16 vagas e liberada. Os sorteios das listas
/// das nações ficam na pilha de cada uma: só a lista final e o sorteador contam.
pub(crate) unsafe extern "thiscall" fn comparar_cqs11_merconorte_convidados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQS11_MERCONORTE_CONVIDADOS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqs11_convidados(this));
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
    PLACAR.registrar(igual && a == b && lista, || format!("cqs11_merconorte_convidados({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// A fase seguinte de um torneio com grupos e mata-mata (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥
/// +0x2c − 1, com sinal), a caixa da linha e volta; senão +0x30 sobe e, chegando a `mata`, o mata-mata (call direto com
/// o ecx da copa -> thiscall, ret 0).
pub(crate) unsafe fn cqs11_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata: i32, rotina: usize, ramo: &'static str) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let f = f + 1;
    write_u32_from_usize(this + 0x30, f as usize);
    if f == mata {
        crate::comparacao::ramo(ramo);
        let m: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
        m(this);
    }
}

// ponytail: o mata-mata aloca e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um (fora da última
// fase, que dá a caixa e não muda nada).
pub(crate) unsafe fn cqs11_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("{}({:#x}) fase {} -> {}", nome, this, f, depois));
}





// ---------- 282a leva: a Merconorte: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (con_merc_cup.cpp) ----------




/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
pub(crate) unsafe fn cqr12_sorteado(this: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) // cqr12: o sorteado k
}

/// rule: the Merconorte has 16 invited clubs (the best of four nations and two clubs of the American league
/// [inference], shuffled) in 4 groups of 4, home and away (3/1 points, one qualifies from each group, to the
/// semi-finals); it starts on 1 August; on the draw day group 1 stays in the cup's own table (drawn clubs 0 to 3) and
/// groups 2 to 4 are stages (4g + 4 to 4g + 7, always 4 teams); on the draw day: the clubs (0x4c2560 (the 16 invited
/// clubs, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted
/// positions; then the league day
/// Original: `ConMercCup::update_for_date` (0x4c2c90, con_merc_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c2c90 } (thiscall, ret 8, con_merc_cup.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Merconorte.
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c2560:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c1de0, thiscall, ret 0; 0: a caixa da linha 0x385),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x38b), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x392), as fases dos grupos 0..2 (0x4c2140: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e volta 0 sem o dia de liga. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
pub(crate) unsafe extern "thiscall" fn cqr12_merconorte_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONMERCCUP_SELECT_INVITED_CLUBS);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONMERCCUP_CREATE_FIRST_GROUP_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CONMERCCUP_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if read_u32_as_usize(this + 0x45) == 0 && completo != 0 {
        let dia = read_i16(this + 0xb6) as u16; // cqr12: o dia do sorteio
        let ano = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparacao::ramo("cqr_12 sorteio da merconorte");
            clubes(this);
            if primeiro(this) == 0 {
                aviso_em(0x9a7d6c, 0x385);
                // ponytail: a original devolve só o al = 0 (o resto do eax é o que a caixa deixou); quem chama lê o byte
                return 0;
            }
            if jogos(this) == 0 {
                aviso_em(0x9a7d6c, 0x38b);
                // ponytail: a original devolve só o al = 0 (o resto do eax é o que a caixa deixou); quem chama lê o byte
                return 0;
            }
            if ordenar(this, 0) == 0 {
                aviso_em(0x9a7d6c, 0x392);
                // ponytail: a original devolve só o al = 0 (o resto do eax é o que a caixa deixou); quem chama lê o byte
                return 0;
            }
            for g in 0..3i32 {
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
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 3 fases
// criadas e a fase atual (+0x30) a última delas.
pub(crate) unsafe extern "thiscall" fn comparar_cqr12_merconorte_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CQR12_MERCONORTE_DIA);
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
            && (0..3).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 2;
        PLACAR.registrar(ok, || format!("cqr12_merconorte_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// rule: group 1 of the Copa Merconorte has 4 clubs, the first 4 drawn, each in the row of its order; human managers
/// get the group draw news
/// Original: `ConMercCup::create_first_group_table` (0x4c1de0, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c1de0 } (thiscall, ret 0, con_merc_cup.cpp): a tabela do primeiro grupo da Merconorte: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x145 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
pub(crate) unsafe extern "thiscall" fn cqr12_merconorte_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9a7d6c, 0x145);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + 0x3e) {
        crate::comparacao::ramo("cqr_12 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr12_sorteado(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
pub(crate) unsafe extern "thiscall" fn comparar_cqr12_merconorte_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQR12_MERCONORTE_PRIMEIRO_GRUPO);
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
    PLACAR.registrar(igual, || format!("cqr12_merconorte_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// rule: each of the other groups of the Copa Merconorte is a league stage with the next 4 drawn clubs (4g + 4 to 4g +
/// 7), the cup's rules and year and the cup's round table for the group; human managers get the news
/// Original: `ConMercCup::create_group_stage` (0x4c2140, con_merc_cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c2140 } (thiscall, ret 4, con_merc_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 2, do dia do sorteio). Os times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x1e9 e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (4), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x1ff e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
pub(crate) unsafe extern "thiscall" fn cqr12_merconorte_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0x9a7d6c, 0x1e9);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparacao::ramo("cqr_12 time do grupo");
        write_u32_from_usize(times + n * 4, cqr12_sorteado(this, k as usize)); // cqr12: o time na vaga do grupo
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
            4,
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
        aviso_em(0x9a7d6c, 0x1ff);
        return 0;
    }
    noticia(f);
    write_u32_from_usize(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
pub(crate) unsafe extern "thiscall" fn comparar_cqr12_merconorte_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CQR12_MERCONORTE_GRUPO);
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
        PLACAR.registrar(ok, || format!("cqr12_merconorte_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

/// rule: the Copa Merconorte starts with groups (stages below 3, id 0x3e9 + stage: 4 groups [inference]) of six rounds
/// on Tuesdays every 15 days, from 9/9 to 18/11; the knockout (stage 3): semi-final drawn on 19/11 and played on 25/11
/// over two legs (7 days), final drawn on 3/12 and a single match on 9/12; category 8; tie-break 3
/// Original: `ConMercCup::create_schedule` (0x4c1960, con_merc_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c1960 } (thiscall, ret 0x10, con_merc_cup.cpp; a virtual +0x3c da Merconorte): `ConMercCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: os grupos ou o mata-mata (`cqp10_merconorte_fase`).
pub(crate) unsafe extern "thiscall" fn cqp10_merconorte_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp10_tabela(this, cqp10_merconorte_fase(fase), rodadas, extra, liga, 0x9a7d6c)
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp10_merconorte_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CQP10_MERCONORTE_TABELA);
    cqp10_comparar("cqp10_merconorte_tabela", fase, cqp10_merconorte_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp10_merconorte_tabela(this, fase, r, e, l))
}

/// rule: the Copa Merconorte groups are home and away, 3/1 points, tiebreak 1, 2, 3, one qualifier per group, category
/// 7, starts on day 1 of month 7 [inference: 1 August], prize fund of 2,000,000
/// Original: `ConMercCup::set_up_rules` (0x4c1ed0, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c1ed0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x8c; quadro SEH só da data local): `ConMercCup::
/// set_up_rules()`: os grupos da Copa Merconorte em dois turnos, 3/1 pontos, desempates (1, 2, 3, 0), um classificado
/// por grupo, clubes internacionais, categoria 7, começo no dia 1 do mês 7, prêmios de 2.000.000.
pub(crate) unsafe extern "thiscall" fn cqj11_merconorte_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_11 regras da merconorte");
    cqj11_regras(this, &CQJ11_MERCONORTE, CQJ11_MERCONORTE.dia, 3)
}

pub(crate) unsafe extern "thiscall" fn comparar_cqj11_merconorte_regras(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQJ11_MERCONORTE_REGRAS);
    cqj11_confere("cqj11_merconorte_regras", this, &CQJ11_MERCONORTE, || o(this))
}
