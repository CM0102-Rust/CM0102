use super::*;




/// AfricanNations::update_for_date(today, full): the daily update of the African Nations Cup. With `full`, before the
/// draw date (day +0xb6, year +0xb8 + +0x40) it returns 0; on the draw date, if not drawn yet (+0x45 = 0), it draws the
/// 16 finalists, sets predicted positions, builds group A's table, fixtures and sorted table (error box and 0 on
/// failure), creates groups B to D, frees the list at +0xee and marks the draw done. Then runs the normal league day
/// (League::update_for_date) and returns its result.
///
/// [pt] { 0x4025a0 } (thiscall, ret 8, african_nations.cpp; a virtual +0x14; quadro SEH só das datas locais, sem destrutor):
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
pub(crate) unsafe extern "thiscall" fn african_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AFRICANNATIONS_DRAW_FINALISTS);
    let group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AFRICANNATIONS_INIT_GROUP_A_TABLE);
    let games: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let stage: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_AFRICANNATIONS_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let league: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if full == 0 {
        return league(this, today, 0);
    }
    let day = read_i16(this + 0xb6) as u16;
    let year = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + league::YEAR) as u16);
    let day_of_today = core::ptr::read_unaligned(today as *const u16);
    let year_of_today = core::ptr::read_unaligned((today + 2) as *const u16);
    // simplification: in the passes before the draw and on errors the original returns only al = 0 (the rest of eax is leftover from the
    // earlier calls); the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        return 0;
    }
    if year_of_today == year && day_of_today == day && read_u32_as_usize(this + league::ACTIVE) == 0 {
        crate::comparison::branch("cqs_13 dia do sorteio da copa africana");
        sortear(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
        predicted(this);
        if group(this) == 0 {
            show_error_at(0x9870f8, 0x3c2);
            return 0;
        }
        if games(this) == 0 {
            show_error_at(0x9870f8, 0x3c8);
            return 0;
        }
        if sort(this, 0) == 0 {
            show_error_at(0x9870f8, 0x3cf);
            return 0;
        }
        for g in 0..3i32 {
            stage(this, g);
        }
        soltar(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, 0);
        core::ptr::write_unaligned((this + 0xf2) as *mut u16, 0);
        write_u32_from_usize(this + league::ACTIVE, 1);
    }
    league(this, today, full)
}

// simplification: the draw, the predictions, the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee and +0xf2 zeroed, the group A table and the three stages
// created, and the current stage (+0x30) is the last of them (2).

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
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
    core::ptr::write_unaligned((this + 0xf2) as *mut u16, 0x10);
    let list = alloc(0xc8);
    write_u32_from_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0x9870f8, 0x349);
        return 0;
    }
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // simplification: the original pushes eax with the year in the low 16 bits and leftovers in the high ones; 0x5fa730 only reads the word
    let how_many = sedes(read_u32_as_usize(G_HOST_COUNTRY), read_u32_as_usize(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS), read_i16(this + league::YEAR) as u16 as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut n = 0usize;
    if how_many > 0 {
        for &id in ids.iter().take(how_many as usize) {
            write_u32_from_usize(read_u32_as_usize(this + 0xee) + n * 4, national_team(read_u32_as_usize(G_NATION_TABLE) + id as usize * nation::SIZE));
            n += 1;
        }
    }
    let mut i = 0i16;
    let mut nation = read_u32_as_usize(G_NATION_TABLE);
    while (i as i32) < read_u32_as_usize(G_NATION_COUNT) as i32 {
        let continent = read_u32_as_usize(nation + nation::CONTINENT);
        if continent != 0 && read_u32_as_usize(continent) == read_u32_as_usize(KEY_CONTINENT_AFRICA) && n < 0x32 { // cqs13: até 50 africanas
            let l = read_u32_as_usize(this + 0xee);
            let sede = (0..how_many.max(0) as usize).any(|j| read_u32_as_usize(read_u32_as_usize(l + j * 4) + 0x53) == nation);
            if !sede {
                write_u32_from_usize(read_u32_as_usize(this + 0xee) + n * 4, national_team(nation));
                n += 1;
            }
        }
        i = i.wrapping_add(1);
        nation += 0x122;
    }
    if n < 0x10 {
        return 0;
    }
    let h = how_many as i32;
    sort(read_u32_as_usize(this + 0xee) + (h as isize * 4) as usize, (n as i32 - h) as u32, 4, 0x4b07d0); // cqs13: as não sedes pela reputação
    let groups = alloc(0x40);
    if groups == 0 {
        show_error_at(0x9870f8, 0x380);
        return 0;
    }
    crate::comparison::branch("cqs_13 sorteio da copa africana");
    for j in 0..4usize {
        for p in 0..4usize {
            let mut r = sorteio(4) as u8 as i8 as usize;
            while read_u32_as_usize(read_u32_as_usize(this + 0xee) + (r + 4 * p) * 4) == 0 {
                r = sorteio(4) as u8 as i8 as usize;
            }
            let e = read_u32_as_usize(this + 0xee) + (r + 4 * p) * 4;
            write_u32_from_usize(groups + (4 * j + p) * 4, read_u32_as_usize(e)); // cqs13: o do pote p na vaga 4j + p
            write_u32_from_usize(e, 0);
        }
    }
    soltar(read_u32_as_usize(this + 0xee));
    write_u32_from_usize(this + 0xee, groups);
    1
}

/// AfricanNations::draw_finalists(): hook entry that delegates to draw_african_nations_finalists, which takes the hosts
/// plus the best-reputed African nations and draws the 16 finalists into 4 groups of 4 by pots; returns 1 on success, 0
/// with fewer than 16 nations or out of memory.
pub(crate) unsafe extern "thiscall" fn draw_finalists(this: usize) -> u32 {
    draw_african_nations_finalists(this)
}


/// AfricanNations::create_group_stage(g): on draw day, creates the league stage of group g + 1 (B to D) of the African
/// Nations Cup with the 4 drawn teams (draw slots 4g+4 to 4g+7), using the round table from the cup's virtual +0x3c,
/// stores it in [+0xc][g], sets the current stage (+0x30) to g and sends the group news. Returns 1, or 0 after the
/// error box when allocation fails.
///
/// [pt] { 0x401ea0 } (thiscall, ret 4, african_nations.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
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
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = alloc(0x10);
    if teams == 0 {
        show_error_at(0x9870f8, 0x2dc);
        return 0;
    }
    for n in 0..4usize {
        crate::comparison::branch("cqs_13 time do grupo");
        write_u32_from_usize(teams + n * 4, read_u32_as_usize(read_u32_as_usize(this + 0xee) + ((g as usize + 1) * 4 + n) * 4)); // cqs13: o time na vaga do grupo
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, g, &mut rodadas, &mut id, 0);
    let p = new(0xee);
    let stage = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construct(
            p,
            this,
            4,
            teams,
            1,
            read_u32_as_usize(this + 4),
            table,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT,
            read_i16(this + league::YEAR) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            4,
            0,
            read_i16(this + league::LEAGUE_FLAGS) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4, stage);
    soltar(teams);
    soltar(table);
    let f = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4);
    if f == 0 {
        show_error_at(0x9870f8, 0x2f2);
        return 0;
    }
    write_u32_from_usize(this + league::CURRENT_STAGE, g as usize);
    news(f);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage at [+0xc][g], +0x30 = g, the stage with 4 teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).



/// rule: the African Nations Cup has three group stages (stages 0 to 2, one per group round [inference]) and, at stage
/// 3, the knockout; going past the last stage is an error (the box)
/// Original: `AfricanNations::advance_stage` (0x401e10, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x401e10 } (thiscall, ret 0, african_nations.cpp; a virtual +0x28): `AfricanNations::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x2ba; senão +0x30 sobe e, chegando a 3 (depois dos grupos), o mata-mata
/// (0x402090: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn african_nations_advance_stage(this: usize) {
    let f = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    if f >= (read_u32_as_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        show_error_at(0x9870f8, 0x2ba);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, f);
    if f == 3 { // cqk12: o mata-mata da Copa Africana depois dos grupos
        crate::comparison::branch("cqk_12 mata-mata da copa africana");
        let knockout: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AFRICANNATIONS_CREATE_KNOCKOUT_STAGE);
        knockout(this);
    }
}

// simplification: the knockout (0x402090) allocates the stage and draws: does not run twice. Original only; the plan: +0x30 goes up one
// (outside the last stage, which gives the message box).

/// Builds the group A table of the African Nations Cup. Rule: group A has four national teams, the first four of the
/// draw list, in order; the managers get the draw news.
///
/// Ret 0: `AfricanNations::init_group_a_table()`: group A with 4 teams (+0x3e), table +0xb1 (0x944e46(0xec): push 0xec;
/// add esp, 4 -> cdecl; 4 rows of 0x3b bytes; out of memory: the box at line 0x1f1 and returns 0), each row i with
/// national side i of the draw list (+0xee) (0x687430: push 0; push 0; push national side; push i; mov ecx, this ->
/// thiscall, ret 0x10), the group draw news (0x684230: mov ecx, this -> thiscall, ret 0) and returns 1.
/// Original: `AfricanNations::init_group_a_table` (0x4019c0, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x4019c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::init_group_a_table()`: o grupo A com 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa
/// da linha 0x1f1 e devolve 0), cada linha i com a seleção i da lista dos sorteados (+0xee) (0x687430: push 0; push 0;
/// push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this
/// -> thiscall, ret 0) e devolve 1.
pub(crate) unsafe extern "thiscall" fn init_group_a_table(this: usize) -> u32 {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let row: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    let teams = 4i16; // cqk12: os quatro do grupo A
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, teams);
    let t = new(0xec);
    core::ptr::write_unaligned((this + league::TABLE) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(0x9870f8, 0x1f1);
        return 0;
    }
    crate::comparison::branch("cqk_12 grupo a da copa africana");
    let mut i = 0i8;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        let national_team = read_u32_as_usize(read_u32_as_usize(this + 0xee).wrapping_add((i as isize * 4) as usize)); // cqk12: a seleção da linha
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        row(this, i as u8 as u32, national_team, 0, 0);
        i = i.wrapping_add(1);
    }
    news(this);
    1
}

// simplification: the table is allocated (and each row allocates the position history) and the news goes to the managers: does not run
// times. Original only; the plan: 4 teams, the table, and in each row i the national team i of those drawn (+0xee) with position i
// (+4 of the row, 0x3b bytes).

/// rule: the season honours (winner, runner-up, third) are the knockout ones
/// Original: `AfricanNations::record_season_honours` (0x401b90, african_nations.cpp, thiscall, ret 0).
///
/// [pt] { 0x401b90 } (thiscall, ret 0, african_nations.cpp; a virtual +0x30): `AfricanNations::record_season_honours()`: o
/// histórico vem do mata-mata: salta para a virtual +0x30 da fase 3 ([+0xc] + 0xc; jmp, sem quadro).
pub(crate) unsafe extern "thiscall" fn african_nations_record_season_honours(this: usize) {
    let stage = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + 0xc);
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(stage) + 0x30));
    h(stage);
}

// simplification: the history (the knockout’s virtual +0x30, 0x519a90 already in Rust with its own comparer) does not run twice;
// the routine writes nothing beyond the jump. Original only.



/// Calls a staff-records final routine f (winner/runner-up/third) on the staff records with club, match and a6.
///
/// [pt] Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push a6; push jogo; push clube; mov ecx,
/// [0xdd7f00] -> thiscall, ret 0xc).
pub(crate) unsafe fn african_nations_record_final_result(f: usize, club: usize, game: usize, a6: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(eng_second_staff_records(), club, game, a6);
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
pub(crate) unsafe fn national_tournament_staff_records(name: u8, this: usize, club: usize, outcome: u32, stage: u32, game: usize, a6: u32, groups: i8) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    if f < groups {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            if name == 0 {
                crate::comparison::branch("cqj_10 africana fora nos grupos");
                european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa Africana
            } else {
                crate::comparison::branch("cqj_10 copa america fora nos grupos");
                european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa América
            }
        }
    } else if f == groups {
        match d {
            0 => {
                let r = read_i16(game + 0x34);
                let weight = scale_by_round(0x28, 0x64, r, 3);
                let knockout = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + groups as usize * 4);
                let round = read_i16(read_u32_as_usize(knockout + cup::ROUND_TABLE).wrapping_add((r as isize * 0x68) as usize) + 0x6f) as u16 as u32;
                european_cup_add_qualified_record(club, comp, read_i16(game + 0x32) as u16 as u32, round, weight);
            }
            0xff => {
                let weight = scale_by_round(0x28, 0x64, read_i16(game + 0x34), 3);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
                if name == 0 {
                    crate::comparison::branch("cqj_10 africana fora no mata-mata");
                    european_cup_add_knocked_out_record(club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight); // cqj10: quem cai no mata-mata da Copa Africana
                } else {
                    european_cup_add_knocked_out_record(club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
                }
            }
            1 => african_nations_record_final_result(0x86a800, club, game, a6),
            2 => african_nations_record_final_result(0x86b0b0, club, game, a6),
            3 => african_nations_record_final_result(0x86b710, club, game, a6),
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
pub(crate) unsafe extern "thiscall" fn african_nations_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    national_tournament_staff_records(0, this, club, outcome, stage, game, a6, 3)
}

// simplification: the records go to the managers’ history and end the call-ups: they do not run twice, and the routine
// writes nothing besides them. Original only; the plan checks the return of 0.


// simplification: like the African Cup one: original only, return value 0.

/// Ends the national team's current competition (NationalTeams::clear_current_competition for the club's nation).
///
/// [pt] A competição atual da seleção acaba (0x775000 em [0xb74318]: push nação; mov ecx -> thiscall, ret 4).
pub(crate) unsafe fn clear_national_team_competition(club: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_CLEAR_CURRENT_COMPETITION);
    f(read_u32_as_usize(G_NATIONAL_TEAMS), read_u32_as_usize(club + club::NATION));
}


// simplification: the records, the end of the national team’s competition and the World Cup place do not run twice. Original only;
// the plan: return value 0 and, on a won play-off, *saida = 4 and the World Cup list (+0x36) one more (if it fit).

/// rule: the African Nations Cup has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 8, 12 and 16
/// February, with each group's fixed matches; the knockout (stage 3) in single matches: quarter-finals on 21/2 with
/// fixed pairings, semi-final on 25/2, third place on 27/2 and final on 28/2 (venue 4); category 10; tie-break 0x303 up
/// to the semi-final and 0x203 in the final; no prizes
/// Original: `AfricanNations::create_schedule` (0x4012e0, african_nations.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4012e0 } (thiscall, ret 0x10, african_nations.cpp; a virtual +0x3c da Copa Africana): `AfricanNations::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `african_nations_phase`.
pub(crate) unsafe extern "thiscall" fn african_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(this, african_nations_phase(stage), rounds, extra, league, 0x9870f8)
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
    crate::comparison::branch("cqk_10 regras da copa africana");
    let win = 3; // cqk10: a vitória nos grupos da Copa Africana
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let day = 5; // cqk10: o dia da data da Copa Africana
    let word = competition_date_word(this, day, 9, -1, 5);
    finish_rules_setup(this, word, 0xffff)
}

