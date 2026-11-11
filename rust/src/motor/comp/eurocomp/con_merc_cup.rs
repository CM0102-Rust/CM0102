use super::*;

/// Qualifiers per group (+0xbe, signed byte).
///
/// [pt] Os classificados por grupo (+0xbe, byte com sinal).
pub(crate) unsafe fn qualifiers_per_group(this: League) -> i32 {
    (this.promoted_count() as i8) as i32
}

/// Club in row i of group 1's table (the cup's own +0xb1, 0x3b-byte rows).
///
/// [pt] O clube da linha i da tabela do grupo 1 (a da própria copa: +0xb1, linhas de 0x3b bytes).
pub(crate) unsafe fn group_one_club(this: League, i: usize) -> usize {
    read_usize(this.table() + i * league_table_row::SIZE) // cqs10: the group 1 row
}

/// Club at position j of stage f (League::get_table_row).
///
/// [pt] O clube da posição j da fase f ([+0xc][f], um League_Stage): a linha pela 0x6826d0 (League::get_table_row: push j;
/// mov ecx, fase -> thiscall, ret 4) e o clube no +0 dela.
pub(crate) unsafe fn stage_club_at(this: League, f: usize, j: i32) -> usize {
    let stage = read_usize(this.stages() + f * 4);
    read_usize(exe::league_get_table_row(stage, j)) // cqs10: position j of the stage
}

/// Builds the knockout stage after the groups (qualifier list, round table, Cup_Stage) and draws round 0.
///
/// The same template is used in the Libertadores and the Merconorte, checked instruction by instruction: only the list
/// size, the group stages, the knockout stage, the source file and the error-box lines change.
///
/// The list (`exe::crt_malloc`(4 × times): push; add esp, 4 -> cdecl; when out of memory, the error box `linha_sem_lista` and
/// return) gets the first +0xbe of group 1 (in row order) and, from each group stage 0..`fases` − 1, the first +0xbe
/// (positions 0, 1...). The round table comes from virtual +0x3c (push 0; push &id; push &rodadas; push `mata`; mov
/// ecx, this -> thiscall, ret 0x10). The stage (`exe::crt_operator_new`(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage
/// `construct_cup_stage`, thiscall, ret 0x3c: the cup, `times`, the list, the rounds, +4, the table, the year +0x40, `mata`, 2, the
/// id, 0x14, 0, 0, 0 and no seeds; the original routine pushes the rounds, the id and the year with garbage in the high
/// 16 bits, which the constructor does not read) goes into [+0xc][`mata`]; the list and the table are freed (`exe::crt_free`).
///
/// Without a stage: the error box `linha_sem_fase`. With one: the draw of round 0 (`draw_round` Cup::draw_round: push 0;
/// mov ecx, stage -> thiscall, ret 4).
///
/// The original routine's SEH frame only covers the constructor (no destructor).
///
/// [pt] O mata-mata depois dos grupos (o mesmo molde na Libertadores e na Merconorte, conferido instrução a instrução: mudam
/// o tamanho da lista, as fases de grupo, a fase do mata-mata, o fonte e as linhas das caixas). A lista (0x944e46(4 ×
/// times): push; add esp, 4 -> cdecl; sem memória, a caixa `linha_sem_lista` e volta) com os +0xbe primeiros do grupo 1
/// (na ordem das linhas) e, de cada fase de grupo 0..`fases` − 1, os +0xbe primeiros (posições 0, 1...); a tabela das
/// rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push `mata`; mov ecx, this -> thiscall, ret 0x10); a
/// fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, `times`,
/// a lista, as rodadas, +4, a tabela, o ano +0x40, `mata`, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra
/// as rodadas, o id e o ano com lixo nos 16 bits altos, que o construtor não lê) em [+0xc][`mata`]; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa `linha_sem_fase`. Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round:
/// push 0; mov ecx, fase -> thiscall, ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
pub(crate) unsafe fn build_knockout_stage(this: League, teams: u32, stages: usize, knockout: u32, file: usize, row_without_list: u32, row_without_stage: u32, ramo: &'static str) {
    let list = exe::crt_malloc(teams * 4);
    if list == 0 {
        return show_error_at(file, row_without_list);
    }
    let mut n = 0usize;
    let mut i = 0i32;
    while i < qualifiers_per_group(this) {
        write_usize(list + n * 4, group_one_club(this, i as usize));
        n += 1;
        i += 1;
    }
    for f in 0..stages {
        let mut j = 0i32;
        while j < qualifiers_per_group(this) {
            covered!(ramo);
            write_usize(list + n * 4, stage_club_at(this, f, j)); // cqs10: the qualifiers of each stage
            n += 1;
            j += 1;
        }
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), knockout, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), teams, list, rodadas as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, knockout, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + knockout as usize * 4, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let f = read_usize(this.stages() + knockout as usize * 4);
    if f == 0 {
        return show_error_at(file, row_without_stage);
    }
    exe::cup_draw_round(f, 0);
}

/// The knockout plan (the comparer runs only the original routine: the list, the table and the stage are allocated and
/// the round is drawn).
///
/// The qualifiers are read first (group 1 in the rows, the stages through the rows of their table +0xb1, like
/// `league_record`); then the stage in [+0xc][`mata`] is checked: +0x3e = `times`, the cup's +4, and the clubs of its list
/// (+0xa7, 6 bytes) equal to the plan's as a set (the draw may reorder them).
///
/// [pt] O plano do mata-mata (o comparador roda só a original: a lista, a tabela e a fase são alocadas e a rodada é
/// sorteada): os classificados lidos antes (o grupo 1 nas linhas, as fases pelas linhas da tabela +0xb1 delas, como a
/// 0x6826d0); depois a fase em [+0xc][`mata`] com +0x3e = `times`, o +4 da copa e os clubes da lista dela (+0xa7, 6
/// bytes) iguais aos do plano como conjunto (o sorteio pode reordenar).
pub(crate) unsafe fn con_merc_cup_compare_knockout_stage(name: &str, this: League, teams: usize, stages: usize, knockout: usize, original: impl FnOnce()) {
    let q = qualifiers_per_group(this).max(0) as usize;
    let mut plan: Vec<usize> = (0..q).map(|i| read_usize(this.table() + i * league_table_row::SIZE)).collect();
    for f in 0..stages {
        let stage = read_usize(this.stages() + f * 4);
        for j in 0..q {
            plan.push(read_usize(read_usize(stage + league::TABLE) + j * league_table_row::SIZE));
        }
    }
    original();
    let stage = read_usize(this.stages() + knockout * 4);
    if stage == 0 {
        SCOREBOARD.record(false, || format!("{}({:#x}) without stage", name, this.addr()));
        return;
    }
    let list = read_usize(stage + cup::TEAM_TABLE);
    let mut found: Vec<usize> = (0..teams).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && read_i16(stage + cup::TEAM_COUNT) as usize == teams && read_usize(stage + 4) == this.competition().addr();
    SCOREBOARD.record(same, || format!("{}({:#x}) plan={:x?} stage={:#x} list={:x?} +0x3e={}", name, this.addr(), pl, stage, ac, read_i16(stage + 0x3e)));
}

/// rule: the Merconorte semi-finals have 4 clubs: the winner (+0xbe = 1) of each of the 4 groups, in a cup stage (stage
/// 3); the first round is drawn at once
/// Original: `ConMercCup::create_semi_final_stage` (0x4c2330, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c2330 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::create_semi_final_stage()`, as semifinais da
/// Merconorte: os 4 (o primeiro do grupo 1 e das 3 fases de grupo) na fase 3; caixas nas linhas 0x21d (sem lista) e
/// 0x23b (sem fase).
pub(crate) unsafe extern "thiscall" fn con_merc_cup_create_semi_final_stage(this: usize) {
    build_knockout_stage(League(this), 4, 3, 3, 0x9a7d6c, 0x21d, 0x23b, "cqs_10 classificados da merconorte");
}

/// rule: the Merconorte moves on one stage when the current one ends; after the 3 group stages (on reaching 3) it
/// builds the semi-finals; going past the last stage is an error
/// Original: `ConMercCup::advance_stage` (0x4c20b0, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c20b0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x28): `ConMercCup::advance_stage()`, a fase seguinte da
/// Merconorte: na última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x1c7; senão +0x30 sobe e, chegando a 3
/// (depois dos grupos), as semifinais (0x4c2330: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn con_merc_cup_advance_stage(this: usize) {
    let this = League(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9a7d6c, 0x1c7);
    }
    let f = f + 1;
    this.set_current_stage(f as usize);
    if f == 3 { // cqs10: the Merconorte semifinals after the groups
        covered!("cqs_10 merconorte semifinals");
        exe::conmerccup_create_semi_final_stage(this);
    }
}

// simplification: the semi-finals (0x4c2330) allocate and roll: they do not run twice. Original only; the plan: +0x30 goes up one
// (outside the last stage, which shows the box and changes nothing).

/// A nation's clubs for the Merconorte: up to 30, in club table order.
///
/// [pt] Os clubes de uma nação para a Merconorte: até 30, na ordem da tabela de clubes.
pub(crate) struct NationClubList {
    pub(crate) clubs: [usize; 30],
    pub(crate) n: u8,
}

impl NationClubList {
    fn new() -> Self {
        NationClubList { clubs: [0; 30], n: 0 }
    }

    fn por(&mut self, c: usize) {
        if self.n < 0x1e {
            self.clubs[self.n as usize] = c;
            self.n += 1;
        }
    }
}

/// `ConMercCup::select_invited_clubs()` (`con_merc_cup_select_invited_clubs`, thiscall, ret 0, con_merc_cup.cpp): builds the 16 invited clubs from
/// Mexico, Colombia, Peru, Bolivia and the MLS, shuffled. +0xf2 = 16 and the list +0xee (`exe::crt_malloc`(0x40): push; add esp,
/// 4 -> cdecl; out of memory, the box for line 0x262 and returns 0).
///
/// The clubs of the table ([0xae23b4], 0x245 bytes, [0xae235c] of them; the first national team, `exe::database_is_national_team`
/// database_is_national_team: push club; add esp, 4 -> cdecl, ends the search) with a nation (+0x53) go, by the
/// nation's id (its +0), to the lists of Mexico ([0x9cf3d0]), Colombia ([0x9cf29c]), Peru ([0x9cf428]) and Bolivia
/// ([0x9cf254]), up to 30 in each; those of another nation whose league (+0x57) is [0x9cf590] (the MLS) go to the
/// American league list, up to 12.
///
/// Each list is sorted by reputation and has its top band shuffled (`comp_util_sort_and_shuffle_top_clubs` comp_util_sort_and_shuffle_top_clubs: push
/// how many; push the count (word); push list; add esp, 0xc -> cdecl; uses random draws): Mexico 4, Colombia 5, Peru 3,
/// Bolivia 2, the American league 2.
///
/// The list: in slots 2..5 the top 4 of Mexico, 6..10 the 5 of Colombia, 11..13 the 3 of Peru, 14..15 the 2 of Bolivia;
/// in slots 0 and 1, without the American league competition ([[0xadadfc] + 4 × [0x9cf728]] = 0) the top 2 of its list;
/// with it, the champion of the last record of competition [0x9cf590] (0xae23d0 + 0x6b × id) and that of its
/// competition +4 (`comp_util_get_latest_placed_club` comp_util_get_latest_placed_club: push place; push competition; add esp -> cdecl), and, if
/// they are the same club, the runner-up of the first in slot 0.
///
/// The list is shuffled (`comp_util_shuffle_pointer_array`(list, 16): push; push; add esp, 8 -> cdecl; 3 × 16 random swaps). Returns 1.
///
/// The original keeps the lists on the stack without zeroing them: with fewer clubs than slots, it reads leftovers; the
/// copy reads zero [inference: in this career the four nations and the MLS have enough clubs; the comparer checks the
/// list].
///
/// [pt] { 0x4c2560 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::select_invited_clubs()`: +0xf2 = 16 e a lista +0xee
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
pub(crate) unsafe fn select_invited_clubs(this: usize) -> u32 {
    write_usize(this + con_merc_cup::DRAWN_CLUB_COUNT, 0x10);
    let americana = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_US_OPEN_CUP) * 4);
    let list = exe::crt_malloc(0x40);
    write_usize(this + con_merc_cup::DRAWN_CLUBS, list);
    if list == 0 {
        show_error_at(0x9a7d6c, 0x262);
        return 0;
    }
    let (mut mexico, mut colombia, mut peru, mut bolivia) = (NationClubList::new(), NationClubList::new(), NationClubList::new(), NationClubList::new());
    let mut mls = [0usize; 12];
    let mut n_mls = 0i8;
    let mut i = 0i32;
    let mut c = read_usize(G_CLUB_TABLE);
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if exe::database_is_national_team(c) != 0 {
            break;
        }
        let nation = read_usize(c + club::NATION);
        if nation != 0 {
            let id = read_usize(nation);
            if id == read_usize(KEY_NATION_MEXICO) {
                mexico.por(c);
            } else if id == read_usize(KEY_NATION_COLOMBIA) {
                colombia.por(c);
            } else if id == read_usize(KEY_NATION_PERU) {
                peru.por(c);
            } else if id == read_usize(KEY_NATION_BOLIVIA) {
                bolivia.por(c);
            } else {
                let league = read_usize(c + club::DIVISION);
                if league != 0 && read_usize(league) == read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && n_mls < 0xc {
                    mls[n_mls as usize] = c;
                    n_mls += 1;
                }
            }
        }
        i += 1;
        c += club::SIZE;
    }
    covered!("cqs_11 merconorte guests");
    exe::comp_util_sort_and_shuffle_top_clubs(mexico.clubs.as_mut_ptr() as usize, mexico.n as u32, 4); // cqs11: the 4 from Mexico
    exe::comp_util_sort_and_shuffle_top_clubs(colombia.clubs.as_mut_ptr() as usize, colombia.n as u32, 5);
    exe::comp_util_sort_and_shuffle_top_clubs(peru.clubs.as_mut_ptr() as usize, peru.n as u32, 3);
    exe::comp_util_sort_and_shuffle_top_clubs(bolivia.clubs.as_mut_ptr() as usize, bolivia.n as u32, 2);
    exe::comp_util_sort_and_shuffle_top_clubs(mls.as_mut_ptr() as usize, n_mls as i16 as u16 as u32, 2);
    let slot = |k: usize, v: usize| write_usize(read_usize(this + con_merc_cup::DRAWN_CLUBS) + k * 4, v);
    slot(2, mexico.clubs[0]);
    slot(3, mexico.clubs[1]);
    slot(4, mexico.clubs[2]);
    slot(5, mexico.clubs[3]);
    for k in 0..5 {
        slot(6 + k, colombia.clubs[k]);
    }
    for k in 0..3 {
        slot(11 + k, peru.clubs[k]);
    }
    slot(14, bolivia.clubs[0]);
    slot(15, bolivia.clubs[1]);
    if americana == 0 {
        slot(0, mls[0]);
        slot(1, mls[1]);
    } else {
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) * 0x6b;
        covered!("cqs_11 american league champion");
        slot(0, exe::comp_util_get_latest_placed_club(record, 0)); // cqs11: the American league champion
        slot(1, exe::comp_util_get_latest_placed_club(read_usize(americana + 4), 0));
        let l = read_usize(this + con_merc_cup::DRAWN_CLUBS);
        if read_usize(l) == read_usize(l + 4) {
            slot(0, exe::comp_util_get_latest_placed_club(record, 1));
        }
    }
    exe::comp_util_shuffle_pointer_array(read_usize(this + con_merc_cup::DRAWN_CLUBS), 0x10);
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
pub(crate) unsafe extern "thiscall" fn con_merc_cup_select_invited_clubs(this: usize) -> u32 {
    select_invited_clubs(this)
}

/// The next stage of a tournament with groups and a knockout (`advance_stage`, virtual `+0x28`).
///
/// At the last stage (`+0x30` ≥ `+0x2c` − 1, signed), the box of the line and return; otherwise `+0x30` is incremented
/// and, on reaching `mata`, the knockout routine runs (direct call with the cup's ecx -> thiscall, ret 0).
///
/// [pt] A fase seguinte de um torneio com grupos e mata-mata (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥
/// +0x2c − 1, com sinal), a caixa da linha e volta; senão +0x30 sobe e, chegando a `mata`, o mata-mata (call direto com
/// o ecx da copa -> thiscall, ret 0).
pub(crate) unsafe fn con_merc_cup_advance_stage_cqs11_next_stage(this: League, file: usize, row: u32, knockout: i32, routine: usize, ramo: &'static str) {
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let f = f + 1;
    this.set_current_stage(f as usize);
    if f == knockout {
        covered!(ramo);
        let m: unsafe extern "thiscall" fn(usize) = core::mem::transmute(routine);
        m(this.addr());
    }
}

// simplification: the knockout allocates and draws: it does not run twice. Original only; the plan: +0x30 goes up one (outside the last
// stage, which shows the box and changes nothing).
/// Comparer: runs only the original and checks +0x30 increased by one (unchanged on the last stage).
pub(crate) unsafe fn con_merc_cup_compare_advance_stage(name: &str, this: League, original: impl FnOnce()) {
    let f = this.current_stage() as i32;
    let last = f >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { f } else { f + 1 }, || format!("{}({:#x}) stage {} -> {}", name, this.addr(), f, after));
}

/// Club k of the drawn list (+0xee pointers).
///
/// [pt] O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
pub(crate) unsafe fn con_merc_cup_drawn_club(this: League, k: usize) -> usize {
    read_usize(read_usize(this.addr() + con_merc_cup::DRAWN_CLUBS) + k * 4) // cqr12: o sorteado k
}

/// rule: the Merconorte has 16 invited clubs (the best of four nations and two clubs of the American league
/// [inference], shuffled) in 4 groups of 4, home and away (3/1 points, one qualifies from each group, to the
/// semi-finals); it starts on 1 August; on the draw day group 1 stays in the cup's own table (drawn clubs 0 to 3) and
/// groups 2 to 4 are stages (4g + 4 to 4g + 7, always 4 teams); on the draw day: the clubs (`con_merc_cup_select_invited_clubs` (the 16 invited
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
pub(crate) unsafe extern "thiscall" fn con_merc_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    if this.active() == 0 && full != 0 {
        let day = this.next_draw_day() as u16; // cqr12: the draw day
        let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
        if year == read_u16(today + 2) && day == read_u16(today) {
            covered!("cqr_12 merconorte draw");
            exe::conmerccup_select_invited_clubs(this);
            if exe::conmerccup_create_first_group_table(this) == 0 {
                show_error_at(0x9a7d6c, 0x385);
                // simplification: the original returns only al = 0 (the rest of eax is what the box left); the caller reads the byte
                return 0;
            }
            if exe::league_create_fixtures(this) == 0 {
                show_error_at(0x9a7d6c, 0x38b);
                // simplification: the original returns only al = 0 (the rest of eax is what the box left); the caller reads the byte
                return 0;
            }
            if exe::league_sort_table(this, 0) == 0 {
                show_error_at(0x9a7d6c, 0x392);
                // simplification: the original returns only al = 0 (the rest of eax is what the box left); the caller reads the byte
                return 0;
            }
            for g in 0..3i32 {
                exe::conmerccup_create_group_stage(this, g);
            }
            let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
            predicted(this.addr());
            exe::crt_free(read_usize(this.addr() + con_merc_cup::DRAWN_CLUBS));
            write_usize(this.addr() + con_merc_cup::DRAWN_CLUBS, 0);
            this.set_active(1);
        }
    }
    exe::league_update_for_date(this, today, full) as u32 
}

// simplification: the clubs (shuffled), the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee cleared, the table of the first group and the 3 stages
// created and the current stage (+0x30) the last of them.

/// Rule: group 1 of the Copa Merconorte has 4 clubs, the first 4 drawn, each in the row of its order; human managers
/// get the group draw news.
///
/// The table of the Merconorte's first group: 4 teams (+0x3e), the table +0xb1 (`exe::crt_malloc`(0xec): push 0xec; add esp, 4
/// -> cdecl; 4 rows of 0x3b bytes; out of memory, the error box at line 0x145 and returns 0), each row i with drawn
/// club i (+0xee) (`init_table_row` League::init_table_row: push 0; push 0; push club; push i; mov ecx, this -> thiscall, ret
/// 0x10), the group draw news (`send_group_draw_news`: mov ecx, this -> thiscall, ret 0) and returns 1. The same body as `init_group_a_table` of
/// the African Cup (only the file and the error box line change).
/// Original: `ConMercCup::create_first_group_table` (0x4c1de0, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c1de0 } (thiscall, ret 0, con_merc_cup.cpp): a tabela do primeiro grupo da Merconorte: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x145 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
pub(crate) unsafe extern "thiscall" fn con_merc_cup_create_first_group_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9a7d6c, 0x145);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        covered!("cqr_12 first group row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, con_merc_cup_drawn_club(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the drawn team i with position i (+4).

/// Rule: each of the other groups of the Copa Merconorte is a league stage with the next 4 drawn clubs (4g + 4 to 4g +
/// 7), the cup's rules and year and the cup's round table for the group; human managers get the news.
///
/// SEH frame only for the stage's new: no destructor on the normal path. Builds the stage of group g + 1 (g from 0 to
/// 2, on the draw day). The teams in a vector of 4 pointers (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory,
/// the error box at line 0x1e9 and 0): drawn clubs 4g + 4 to 4g + 7 (+0xee); the round table through virtual +0x3c
/// (push 0; push &id; push &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xee) push; add
/// esp, 4 -> cdecl; the League_Stage constructor 0x68b100, thiscall, ret 0x58, 22 arguments: the cup, the teams (4),
/// the vector, +0x3c (word), +4, the round table, the rounds (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, the year
/// +0x40 (word), g, the id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; the original pushes the words and bytes with
/// garbage in the high bytes, which the constructor does not read: the copy passes clean values) into [+0xc][g]; the
/// vector and the round table freed (`exe::crt_free`). No stage: the error box at line 0x1ff and 0. With it: the group news
/// (`send_group_draw_news` on the stage), +0x30 = g and 1.
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
pub(crate) unsafe extern "thiscall" fn con_merc_cup_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0x9a7d6c, 0x1e9);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        covered!("cqr_12 group team");
        write_usize(teams + n * 4, con_merc_cup_drawn_club(this, k as usize)); // cqr12: the team in the group slot
        k += 1;
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), g, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this.addr(),
            4,
            teams,
            this.legs() as u16 as u32,
            this.competition().addr(),
            table,
            rodadas as u16 as u32,
            this.win_points() as u32 ,
            this.draw_points() as u32 ,
            this.match_category() as u32 ,
            this.addr() + league::TIEBREAKS,
            this.addr() + league::PROMOTED_COUNT,
            this.year() as u16 as u32,
            g as u32,
            id as u16 as u32,
            this.importance() as u32 ,
            2,
            0,
            this.league_flags() as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(this.stages() + g as usize * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(this.stages() + g as usize * 4);
    if f == 0 {
        show_error_at(0x9a7d6c, 0x1ff);
        return 0;
    }
    exe::league_send_group_draw_news(f);
    this.set_current_stage(g as usize);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][g], +0x30 = g, the stage with the teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// drawn 4g + 4 to 4g + 7 (in any order: the constructor sorts the table).

/// rule: the Copa Merconorte starts with groups (stages below 3, id 0x3e9 + stage: 4 groups [inference]) of six rounds
/// on Tuesdays every 15 days, from 9/9 to 18/11; the knockout (stage 3): semi-final drawn on 19/11 and played on 25/11
/// over two legs (7 days), final drawn on 3/12 and a single match on 9/12; category 8; tie-break 3
/// Original: `ConMercCup::create_schedule` (0x4c1960, con_merc_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c1960 } (thiscall, ret 0x10, con_merc_cup.cpp; a virtual +0x3c da Merconorte): `fase.create_schedule(/// &rodadas, &extra, &é_liga)`: os grupos ou o mata-mata (`merconorte_stage`).
pub(crate) unsafe extern "thiscall" fn con_merc_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_stage_schedule(Cup(this), merconorte_stage(stage), rounds, extra, league, 0x9a7d6c)
}

/// rule: the Copa Merconorte groups are home and away, 3/1 points, tiebreak 1, 2, 3, one qualifier per group, category
/// 7, starts on day 1 of month 7 [inference: 1 August], prize fund of 2,000,000
/// Original: `ConMercCup::set_up_rules` (0x4c1ed0, con_merc_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c1ed0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x8c; quadro SEH só da data local): `ConMercCup::
/// set_up_rules()`: os grupos da Copa Merconorte em dois turnos, 3/1 pontos, desempates (1, 2, 3, 0), um classificado
/// por grupo, clubes internacionais, categoria 7, começo no dia 1 do mês 7, prêmios de 2.000.000.
pub(crate) unsafe extern "thiscall" fn con_merc_cup_set_up_rules(this: usize) -> u32 {
    covered!("cqj_11 merconorte rules");
    apply_cup_rules(League(this), &MERCONORTE_RULES, MERCONORTE_RULES.day, 3)
}

