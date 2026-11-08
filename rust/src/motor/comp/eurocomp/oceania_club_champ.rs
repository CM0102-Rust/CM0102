use super::*;

/// Club k of the 8 selected (+0xee: group A slots 0..3, B 4..7).
///
/// [pt] O clube k dos 8 escolhidos (+0xee, ponteiros: o grupo A nas vagas 0..3, o B nas 4..7), a que as duas tabelas leem.
pub(crate) unsafe fn selected_club(this: usize, k: usize) -> usize {
    read_usize(read_usize(this + 0xee) + k * 4) // cqt10: o escolhido k
}

/// Rule: the Oceania Club Championship is drawn on the start day (+0xb6/+0xb8) and only on the call without `completo`:
/// the 8 clubs, the predicted positions, group A in the cup's own table, the fixtures and group B as a stage; before
/// that day the cup does not even run the league day; an error while building returns 1 without the league day.
///
/// `OceaniaClubChamp::update_for_date(today, completo)` (virtual +0x14; SEH frame only for the local dates, no
/// destructor): the Oceania Club Championship's day. The draw date is that of 0x549ef0(+0xb6, +0xb8 + +0x40) (lea ecx,
/// local; push year; push day -> thiscall, ret 8, pure). Before it (today's year lower, unsigned; or the same year and
/// today's day lower, signed) the routine returns 0 without the league day. On the draw day, if the draw is not done
/// (+0x45 = 0) and WITHOUT `completo`: the 8 clubs (`oceania_club_champ_select_qualified_clubs`: mov ecx -> thiscall, ret 0), the predicted positions
/// (virtual +0x5c, thiscall, ret 0), the group A table (`create_group_a_table`, thiscall, ret 0; 0: the error box at line 0x21d), the
/// fixtures (`create_fixtures` League::create_fixtures, thiscall, ret 0; 0: the error box at line 0x224), the sorted table
/// (`sort_table` push 0; mov ecx -> thiscall, ret 4; 0: the error box at line 0x22b) (an error returns 1, without the
/// league day), the group B stage (`create_group_b_stage`, thiscall, ret 0), the list +0xee freed (`exe::crt_free`: push; add esp, 4 ->
/// cdecl) and zeroed, +0xf2 = 0 and +0x45 = 1. Then the league day (`league_update_for_date`: push completo; push today; mov ecx, this
/// -> thiscall, ret 8), whose return value it passes on.
/// Original: `OceaniaClubChamp::update_for_date` (0x797b40, oceania_club_champ.cpp, thiscall, ret 8).
///
/// [pt] { 0x797b40 } (thiscall, ret 8, oceania_club_champ.cpp; a virtual +0x14; quadro SEH só das datas locais, sem
/// destrutor): `OceaniaClubChamp::update_for_date(hoje, completo)`, o dia do Campeonato de Clubes da Oceania. A data do
/// sorteio é a de 0x549ef0(+0xb6, +0xb8 + +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura). Antes
/// dela (o ano de hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de
/// liga. No dia do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo`: os 8 clubes (0x798220: mov ecx ->
/// thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall, ret 0), a tabela do grupo A (0x7978c0, thiscall,
/// ret 0; 0: a caixa da linha 0x21d), os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha
/// 0x224), a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x22b) (um erro volta 1,
/// sem o dia de liga), a fase do grupo B (0x797e80, thiscall, ret 0), a lista +0xee liberada (0x9452ca: push; add esp,
/// 4 -> cdecl) e zerada, +0xf2 = 0 e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this
/// -> thiscall, ret 8), cuja devolução ela repassa.
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let league: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let day = read_i16(this + 0xb6) as u16;
    let year = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + league::YEAR) as u16);
    let day_of_today = core::ptr::read_unaligned(today as *const u16);
    let year_of_today = core::ptr::read_unaligned((today + 2) as *const u16);
    // simplification: in the loops before the draw and on errors the original returns only al (0 or 1; the rest of eax is leftover from the
    // earlier calls); the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        return 0;
    }
    if year_of_today == year && day_of_today == day && read_usize(this + league::ACTIVE) == 0 && full == 0 {
        crate::comparison::branch("cqt_10 oceania draw");
        exe::oceaniaclubchamp_select_qualified_clubs(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
        predicted(this);
        if exe::oceaniaclubchamp_create_group_a_table(this) == 0 {
            show_error_at(0xa6a068, 0x21d);
            return 1;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0xa6a068, 0x224);
            return 1;
        }
        if sort(this, 0) == 0 {
            show_error_at(0xa6a068, 0x22b);
            return 1;
        }
        exe::oceaniaclubchamp_create_group_b_stage(this);
        exe::crt_free(read_usize(this + 0xee));
        write_usize(this + 0xee, 0);
        write_usize(this + 0xf2, 0);
        write_usize(this + league::ACTIVE, 1);
    }
    league(this, today, full)
}

// simplification: the clubs, the predictions, the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee and +0xf2 zeroed, the group A table and the stage of
// grupo B criadas e a fase atual (+0x30) = 0.

/// Rule: group A of the Oceania Club Championship has the first 4 chosen clubs, each in the row of its order; human
/// managers get the group draw news.
///
/// `OceaniaClubChamp::create_group_a_table()`: 4 teams (+0x3e), the table +0xb1 (`exe::crt_malloc`(0xec): push 0xec; add esp, 4
/// -> cdecl; 4 rows of 0x3b bytes; out of memory, the error box at line 0x182 and returns 0), each row i with chosen
/// club i (+0xee) (`init_table_row` League::init_table_row: push 0; push 0; push club; push i; mov ecx, this -> thiscall, ret
/// 0x10), the group draw news (`send_group_draw_news`: mov ecx, this -> thiscall, ret 0) and returns 1. The same body as `init_group_a_table` of
/// the African Cup (only the file and the error box line change).
/// Original: `OceaniaClubChamp::create_group_a_table` (0x7978c0, oceania_club_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x7978c0 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::create_group_a_table()`: 4 times (+0x3e), a
/// tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha
/// 0x182 e devolve 0), cada linha i com o escolhido i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push
/// clube; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this ->
/// thiscall, ret 0) e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
pub(crate) unsafe extern "thiscall" fn create_group_a_table(this: usize) -> u32 {
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, 4);
    let t = exe::crt_malloc(0xec);
    write_usize(this + league::TABLE, t);
    if t == 0 {
        show_error_at(0xa6a068, 0x182);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        crate::comparison::branch("cqt_10 group a row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, selected_club(this, i as usize), 0, 0); // cqt10: row i of group A
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the chosen i with position i (+4).

/// Rule: group B of the Oceania Club Championship is a league stage with chosen clubs 4 to 7, the cup's rules and year
/// and the cup's round table for stage 0 (10, 12 and 14 January); human managers get the news.
///
/// `OceaniaClubChamp::create_group_b_stage()` (SEH frame only for the stage's new: no destructor on the normal path):
/// the teams in a vector of 4 pointers (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the error box at line
/// 0x296 and 0): chosen clubs 4 to 7 (+0xee); the round table through virtual +0x3c (push 0; push &id; push &rounds;
/// push 0; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xee) push; add esp, 4 -> cdecl; the League_Stage
/// constructor 0x68b100, thiscall, ret 0x58, 22 arguments: the cup, 4, the vector, 1, +4, the round table, the rounds
/// (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, the year +0x40 (word), 0, the id (word), +0x51, 2, 0, +0xd9 (word), −1,
/// 0, 2; the original pushes the words and bytes with garbage in the high bytes, which the constructor does not read:
/// the copy passes clean values) into [+0xc][0]; the vector and the round table freed (`exe::crt_free`). No stage: the error
/// box at line 0x2ad and 0. With it: +0x30 = 0, the group news (`send_group_draw_news` on the stage) and 1.
/// Original: `OceaniaClubChamp::create_group_b_stage` (0x797e80, oceania_club_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x797e80 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_group_b_stage()`: os times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp,
/// 4 -> cdecl; sem memória, a caixa da linha 0x296 e 0): os escolhidos 4 a 7 (+0xee); a tabela das rodadas pela virtual
/// +0x3c (push 0; push &id; push &rodadas; push 0; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a
/// tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), 0, o id (word),
/// +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos, que o construtor
/// não lê: a cópia passa os valores limpos) em [+0xc][0]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase:
/// a caixa da linha 0x2ad e 0. Com ela: +0x30 = 0, a notícia do grupo (0x684230 na fase) e 1.
pub(crate) unsafe extern "thiscall" fn create_group_b_stage(this: usize) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0xa6a068, 0x296);
        return 0;
    }
    for n in 0..4usize {
        crate::comparison::branch("cqt_10 group b team");
        write_usize(teams + n * 4, selected_club(this, 4 + n)); // cqt10: the team in the group B slot
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, 0, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this,
            4,
            teams,
            1,
            read_usize(this + 4),
            table,
            rodadas as u16 as u32,
            (read_u8(this + 0xc2) as u32),
            (read_u8(this + 0xc3) as u32),
            (read_u8(this + 0xc4) as u32),
            this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT,
            read_i16(this + league::YEAR) as u16 as u32,
            0,
            id as u16 as u32,
            (read_u8(this + 0x51) as u32),
            2,
            0,
            read_i16(this + league::LEAGUE_FLAGS) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES));
    if f == 0 {
        show_error_at(0xa6a068, 0x2ad);
        return 0;
    }
    write_usize(this + league::CURRENT_STAGE, 0);
    exe::league_send_group_draw_news(f);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][0], +0x30 = 0, the stage with 4 teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// chosen 4 to 7 (in any order: the constructor sorts the table).

/// The clubs of one source for the Oceania competition: up to 20, in the order of the club table.
///
/// [pt] Os clubes de uma origem para a Oceania: até 20, na ordem da tabela de clubes.
pub(crate) struct SourceClubList {
    pub(crate) clubs: [usize; 20],
    pub(crate) n: i16,
}

impl SourceClubList {
    fn novo() -> Self {
        SourceClubList { clubs: [0; 20], n: 0 }
    }

    /// Adds the club if it fits (the count is a signed word, < 20).
    ///
    /// [pt] Entra se cabe (a conta é palavra com sinal, < 20).
    fn cabe(&self) -> bool {
        self.n < 0x14
    }

    fn por(&mut self, c: usize) {
        self.clubs[self.n as usize] = c;
        self.n += 1;
    }

    /// Sorts by reputation and shuffles the top band.
    ///
    /// Uses `comp_util_sort_and_shuffle_top_clubs` comp_util_sort_and_shuffle_top_clubs (push how many; push the count; push list; add esp -> cdecl;
    /// draws at random).
    ///
    /// [pt] Ordena pela reputação e embaralha a faixa de cima (0x4afbd0 comp_util_sort_and_shuffle_top_clubs: push quantos;
    /// push a conta; push lista; add esp -> cdecl; sorteia).
    unsafe fn ordenar(&mut self, quantos: u32) {
        let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
        f(self.clubs.as_mut_ptr() as usize, self.n as u16 as u32, quantos);
    }

    /// The first club of the list and the first one after it from another nation (+0x53), as in the original.
    ///
    /// It walks the `n` slots; an empty slot does not count as the first.
    ///
    /// [pt] O primeiro da lista e o primeiro depois dele de outra nação (+0x53), como a original: percorre as `n` vagas; vaga
    /// vazia não conta como primeiro.
    unsafe fn dois_de_nacoes_diferentes(&self) -> (usize, usize) {
        let (mut a, mut b) = (0usize, 0usize);
        for &c in self.clubs.iter().take(self.n.max(0) as usize) {
            if a == 0 {
                a = c;
            } else if b == 0 && read_usize(c + 0x53) != read_usize(a + 0x53) {
                b = c;
            }
        }
        (a, b)
    }
}

/// Places a pot pair into groups A and B by the draw: first to slot k and second to k+4, or reversed.
///
/// [pt] O par de um pote vai para os grupos A e B: com o sorteio de 2 (0x90cfc0(2): push 2; add esp -> cdecl) diferente de
/// zero, o primeiro na vaga `k` (grupo A) e o segundo na `k` + 4 (grupo B); com zero, ao contrário.
pub(crate) unsafe fn place_pot_pair(this: usize, k: usize, first: usize, second: usize, sorteio: i32) {
    let l = read_usize(this + 0xee);
    if sorteio != 0 {
        write_usize(l + k * 4, first);
        write_usize(read_usize(this + 0xee) + (k + 4) * 4, second);
    } else {
        write_usize(l + (k + 4) * 4, first);
        write_usize(read_usize(this + 0xee) + k * 4, second);
    }
}

/// `OceaniaClubChamp::select_qualified_clubs()` (`0x798220`, thiscall, ret 0, `oceania_club_champ.cpp`): builds the
/// 8-club list of the Oceania Club Championship from the Australian champion and the clubs of the Oceania nations, by
/// drawn pairs.
///
/// Sets `+0xf2` = 8 and allocates list `+0xee` (`0x944e46(0x20)`: push; add esp, 4 -> cdecl; out of memory, the box of
/// line `0x307` and returns 0). The champion of the last record of the Australian league (`0x4afd80`
/// comp_util_get_latest_placed_club: push 0; push competition `0xae23d0` + 0x6b × [`0x9cf89c`]; add esp, 8 -> cdecl).
///
/// The clubs ([`0xae23b4`], 0x245 bytes, [`0xae235c`] − 2 × [`0xae2350`] of them) from an Oceania nation ([`0x9cfa18`]:
/// `+0x71` of the nation, its `+0`) go, by nation id, to the lists of New Zealand ([`0x9cf3f4`]), Australia
/// ([`0x9cf224`]), Tahiti ([`0x9cf4b4`]) and Fiji ([`0x9cf2f8`]), up to 20 in each (the count is a signed word); the
/// others (or when the list is full) go to the Melanesian list (`0x53d900` database_is_melanesian_nation: push nation;
/// add esp -> cdecl; when full, the club is left out) or, otherwise, the Polynesian list (`0x53d940` with the nation
/// again), up to 20. Without a champion, the best Australian (the list with a drawn range of 1) takes its place.
///
/// The pairs, each drawn between groups A (slots 0..3) and B (4..7) by `0x90cfc0(2)`: the champion and the best New
/// Zealander (slots 0/4); the best from Fiji and from Tahiti (1/5); the first two Melanesians from different nations
/// (2/6, the range of 2); the two Polynesians (3/7). Returns 1.
///
/// The original routine keeps the lists on the stack without zeroing them; with fewer clubs than it reads, it reads
/// leftovers; the copy reads zero [inference: in this career the lists have enough clubs; the comparer checks the
/// list].
///
/// [pt] { 0x798220 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::select_qualified_clubs()`: +0xf2 = 8 e a
/// lista +0xee (0x944e46(0x20): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x307 e devolve 0). O campeão
/// do último registro da liga australiana (0x4afd80 comp_util_get_latest_placed_club: push 0; push competição
/// 0xae23d0 + 0x6b × [0x9cf89c]; add esp, 8 -> cdecl). Os clubes ([0xae23b4], 0x245 bytes, [0xae235c] − 2 × [0xae2350]
/// deles) de nação da Oceania ([0x9cfa18]: +0x71 da nação, o +0 dele) vão, pelo id da nação, para as listas da Nova
/// Zelândia ([0x9cf3f4]), da Austrália ([0x9cf224]), do Taiti ([0x9cf4b4]) e de Fiji ([0x9cf2f8]), até 20 em cada (a
/// conta é palavra com sinal); os outros (ou com a lista cheia) para a melanésia (0x53d900 database_is_melanesian_nation:
/// push nação; add esp -> cdecl; cheia, o clube fica de fora) ou, senão, a polinésia (0x53d940 com a nação de novo), até
/// 20. Sem campeão, o melhor
/// australiano (a lista com a faixa de 1 sorteada) no lugar. Os pares, cada um sorteado entre os grupos A (vagas 0..3)
/// e B (4..7) por 0x90cfc0(2): o campeão e o melhor neozelandês (vagas 0/4); o melhor de Fiji e o do Taiti (1/5); os dois
/// primeiros melanésios de nações diferentes (2/6, a faixa de 2); os dois polinésios (3/7). Devolve 1.
/// A original guarda as listas na pilha sem zerá-las; com menos clubes que o lido ela lê sobra; a cópia lê zero
/// [inferência: nesta carreira as listas têm clubes que bastam; o comparador confere a lista].
pub(crate) unsafe fn select_qualified_clubs(this: usize) -> u32 {
    write_usize(this + 0xf2, 8);
    let list = exe::crt_malloc(0x20);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0xa6a068, 0x307);
        return 0;
    }
    let mut champion = exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE) * 0x6b, 0);
    let (mut nz, mut aus, mut tai, mut fij, mut mel, mut pol) =
        (SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo());
    let mut i = 0i32;
    let mut c = read_usize(G_CLUB_TABLE);
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        let nation = read_usize(c + 0x53);
        if nation != 0 && read_usize(nation + nation::CONTINENT) != 0 && read_usize(read_usize(nation + nation::CONTINENT)) == read_usize(KEY_CONTINENT_OCEANIA) {
            let id = read_usize(nation);
            if id == read_usize(KEY_NATION_NEW_ZEALAND) && nz.cabe() {
                nz.por(c);
            } else if id == read_usize(KEY_NATION_AUSTRALIA) && aus.cabe() {
                aus.por(c);
            } else if id == read_usize(KEY_NATION_TAHITI) && tai.cabe() {
                tai.por(c);
            } else if id == read_usize(KEY_NATION_FIJI) && fij.cabe() {
                fij.por(c);
            } else if exe::database_is_melanesian_nation(nation) != 0 {
                if mel.cabe() {
                    mel.por(c);
                }
            } else if exe::database_is_polynesian_nation(read_usize(c + 0x53)) != 0 && pol.cabe() {
                pol.por(c);
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparison::branch("cqt_11 oceania clubs");
    if champion == 0 {
        aus.ordenar(1);
        champion = aus.clubs[0];
    }
    nz.ordenar(1);
    place_pot_pair(this, 0, champion, nz.clubs[0], exe::utils_random(2)); // cqt11: the Australian and New Zealand champions
    fij.ordenar(1);
    tai.ordenar(1);
    place_pot_pair(this, 1, fij.clubs[0], tai.clubs[0], exe::utils_random(2)); // cqt11: Fiji e Taiti
    mel.ordenar(2);
    let (a, b) = mel.dois_de_nacoes_diferentes();
    place_pot_pair(this, 2, a, b, exe::utils_random(2)); // cqt11: os dois melanésios
    pol.ordenar(2);
    let (a, b) = pol.dois_de_nacoes_diferentes();
    place_pot_pair(this, 3, a, b, exe::utils_random(2));
    1
}

/// rule: the Oceania Club Championship has 8 clubs in 4 pairs, each pair split between groups A and B by a draw: the
/// Australian league champion (without one, the best Australian club) and the best New Zealand club; the best of Fiji
/// and of Tahiti; two Melanesian clubs of different nations; two Polynesian clubs of different nations (each list
/// sorted by reputation, with the top band shuffled)
/// Original: `OceaniaClubChamp::select_qualified_clubs` (0x798220, oceania_club_champ.cpp, thiscall, ret 0).
///
/// [pt] regra: o Campeonato de Clubes da Oceania tem 8 clubes em 4 pares, cada par dividido entre os grupos A e B por
/// sorteio: o campeão da liga australiana (sem ele, o melhor australiano) e o melhor neozelandês; o melhor de Fiji e o
/// do Taiti; dois melanésios de nações diferentes; dois polinésios de nações diferentes (cada lista ordenada pela
/// reputação, com a faixa de cima embaralhada)
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_select_qualified_clubs(this: usize) -> u32 {
    select_qualified_clubs(this)
}

/// rule: the Oceania Club Championship moves on one stage when the current one ends; after the groups (on reaching 1)
/// it builds the semi-finals; going past the last stage is an error
/// Original: `OceaniaClubChamp::advance_stage` (0x797df0, oceania_club_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x797df0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x28): `OceaniaClubChamp::advance_stage()`: na
/// última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x274; senão +0x30 sobe e, chegando a 1 (depois dos
/// grupos), as semifinais (0x798050: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_advance_stage(this: usize) {
    let f = read_usize(this + league::CURRENT_STAGE) as i32;
    if f >= (read_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        return show_error_at(0xa6a068, 0x274);
    }
    let f = f + 1;
    write_usize(this + league::CURRENT_STAGE, f as usize);
    if f == 1 {
        crate::comparison::branch("cqt_11 oceania semifinals");
        exe::oceaniaclubchamp_create_knockout_stage(this);
    }
}

// simplification: the semi-finals allocate and draw: they do not run twice. Original only; the plan: +0x30 goes up one (outside the
// last stage, which shows the box and changes nothing).

/// The 4 semifinalists: 1st A, 2nd B, 2nd A, 1st B.
///
/// [pt] Os 4 das semifinais, na ordem da lista: o 1º do grupo A (a tabela da copa, +0xb1), o 2º do B (a tabela da fase
/// [+0xc][0]), o 2º do A e o 1º do B (os cruzamentos: as chaves são os pares da lista).
pub(crate) unsafe fn semifinalists(this: usize) -> [usize; 4] {
    let a = |l: usize| read_usize(read_usize(this + league::TABLE) + l * 0x3b);
    let b = |l: usize| read_usize(read_usize(read_usize(read_usize(this + league::STAGES)) + 0xb1) + l * 0x3b);
    [a(0), b(1), a(1), b(0)] // cqt11: 1º A × 2º B, 2º A × 1º B
}

/// Rule: the Oceania Club Championship semi-finals cross the groups: A winner × B runner-up and A runner-up × B winner,
/// in a cup stage (stage 1) with the first round set up at once.
///
/// `OceaniaClubChamp::create_knockout_stage()` (SEH frame only for the stage's new: no destructor on the normal path),
/// the semi-finals: the list of the 4 (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the error box at line
/// 0x2cb and return), the semi-finalist pairings; the round table through virtual +0x3c (push 0; push &id; push
/// &rounds; push 1; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage `construct_cup_stage`, thiscall, ret 0x3c: the cup, 4, the list, the rounds, +4, the table, the year +0x40,
/// 1, 2, the id, 0x14, 0, 0, 0 and no seed; the original pushes the rounds, the id and the year with garbage in the
/// high 16 bits, which the constructor does not read) into [+0xc][1]; the table and the list freed (`exe::crt_free`). No
/// stage: the error box at line 0x2e3 and return. With it: the round set up (`set_up_round` Cup::set_up_round: push 0; mov
/// ecx, stage -> thiscall, ret 4).
/// Original: `OceaniaClubChamp::create_knockout_stage` (0x798050, oceania_club_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x798050 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_knockout_stage()`, as semifinais: a lista dos 4 (0x944e46(0x10): push; add esp, 4
/// -> cdecl; sem memória, a caixa da linha 0x2cb e volta), os cruzamentos de semifinalists; a tabela das rodadas
/// pela virtual +0x3c (push 0; push &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2)
/// push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a
/// tabela, o ano +0x40, 1, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo
/// nos 16 bits altos, que o construtor não lê) em [+0xc][1]; a tabela e a lista liberadas (0x9452ca). Sem fase: a caixa
/// da linha 0x2e3 e volta. Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall,
/// ret 4).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_create_knockout_stage(this: usize) {
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0xa6a068, 0x2cb);
    }
    crate::comparison::branch("cqt_11 oceania pairings");
    for (k, &c) in semifinalists(this).iter().enumerate() {
        write_usize(list + k * 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, 1, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 4, list, rodadas as u16 as u32, read_usize(this + 4), table, read_i16(this + league::YEAR) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES) + 4, stage);
    exe::crt_free(table);
    exe::crt_free(list);
    let f = read_usize(read_usize(this + league::STAGES) + 4);
    if f == 0 {
        return show_error_at(0xa6a068, 0x2e3);
    }
    build(f, 0);
}

// simplification: allocates the stage and builds the round: does not run twice. Original only; the plan: the stage at [+0xc][1] with 4 teams,
// the cup’s +4 and the 4 crossed ones (read earlier) in its list (+0xa7, 6 bytes; as a set: the round may reorder).

/// One operation of building the round table, in the original routine's order.
///
/// `D` the date of a group round (`league_round_set_date`: round, day, month with 0 = January, years to add, weekday, code, value;
/// records of 0x41 bytes), `L` a match of the group round (`store_round_match`: round, match, a, b, c, value), `S` the draw of a
/// cup round (`cup_set_round_draw_date`: round, day, month, years, weekday; records of 0x68 bytes), `J` the match of the cup round
/// (`cup_set_round_match_date`: round, day, month, years, weekday, time, home side), `E` a day of the tie (`cup_set_round_match_day`: round, slot,
/// weekday, time, code, home side) and `B`/`W`/`X` writing a byte/word/dword at the given offset of the table.
///
/// [pt] Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
pub(crate) enum OceaniaClubChampScheduleOp {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
pub(crate) use OceaniaClubChampScheduleOp::{B as Cqq12B, D as Cqq12D, E as Cqq12E, J as Cqq12J, L as Cqq12L, S as Cqq12S, W as Cqq12W, X as Cqq12X};

/// Describes one stage of virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`).
///
/// It holds the é_liga written (if requested), the rounds, the extra (only if the pointer is not null, when
/// `extra_checa`), the table size, the error-box line when out of memory, the common operations and the stage's own
/// operations (the matches of each group) and the counter name. `None`: the stage returns 0 without writing anything.
///
/// [pt] Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
pub(crate) struct OceaniaStage {
    pub(crate) league: i32,
    pub(crate) rounds: i16,
    pub(crate) extra: i16,
    pub(crate) extra_check: bool,
    pub(crate) size: u32,
    pub(crate) row: u32,
    pub(crate) ops: &'static [OceaniaClubChampScheduleOp],
    pub(crate) ops2: &'static [OceaniaClubChampScheduleOp],
    pub(crate) name: &'static str,
}

/// Runs stage `f` (or returns 0 without it): the outputs, the table (`exe::crt_malloc`; out of memory, the box `linha` of source
/// file `arquivo` and 0) and the date/match operations in order.
///
/// The cup's year (+0x40) goes as clean 16 bits to `league_round_set_date` (the original pushes the register with garbage in the high
/// 16 bits; `league_round_set_date` only uses the word) and signed to the cup ones.
///
/// [pt] Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
pub(crate) unsafe fn build_stage_table(this: usize, f: Option<OceaniaStage>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.league;
    }
    *rodadas = f.rounds;
    if !f.extra_check || !extra.is_null() {
        *extra = f.extra;
    }
    let t = exe::crt_malloc(f.size);
    if t == 0 {
        show_error_at(arquivo, f.row);
        return 0;
    }
    crate::comparison::branch(f.name);
    let ano16 = core::ptr::read_unaligned((this + league::YEAR) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + league::YEAR) as *const i16) as i32 as u32;
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqq12D(r, dia, mes, anos, sem, cod, v) => {
                exe::league_round_set_date(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqq12L(r, j, a, b, c, v) => exe::league_round_set_match_slot(t, r, j, a, b, c, v),
            Cqq12S(r, dia, mes, anos, sem) => {
                exe::cup_set_round_draw_date(t, r, dia, mes, anos, sem, ano);
            }
            Cqq12J(r, dia, mes, anos, sem, hora, mando) => {
                exe::cup_set_round_match_date(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqq12E(r, vaga, sem, hora, cod, mando) => {
                exe::cup_set_round_match_day(t, r, vaga, sem, hora, cod, mando);
            }
            Cqq12B(o, v) => *((t + o) as *mut u8) = v,
            Cqq12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqq12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Bytes the stage operations write into table t, in operation order.
///
/// [pt] Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
pub(crate) unsafe fn oceania_club_champ_written_table_bytes(ops: &[OceaniaClubChampScheduleOp], ops2: &[OceaniaClubChampScheduleOp], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqq12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqq12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqq12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqq12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqq12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqq12B(o, _) => pega(o, 1),
            Cqq12W(o, _) => pega(o, 2),
            Cqq12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Side-by-side comparer of the stage table builder.
///
/// The original routine runs with the caller's outputs, the copy with its own outputs that start with the same values,
/// both from the same [0xb67a34]; the outputs and the bytes written to the two tables must be equal; the copy's table
/// is freed (`exe::crt_free`).
///
/// In the stage without a table only the original runs (neither writes anything; the outputs may not even exist).
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
pub(crate) unsafe fn compare_stage_table(name: &str, stage: u32, f: Option<OceaniaStage>, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            SCOREBOARD.record(a == 0, || format!("{}(fase {}) devolveu {:#x}", name, stage as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rounds, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && (extra.is_null() || *extra == e2) && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { oceania_club_champ_written_table_bytes(f.ops, f.ops2, a) == oceania_club_champ_written_table_bytes(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// O Campeonato de Clubes da Oceania, as três rodadas de cada grupo (10, 12 e 14 de janeiro do ano seguinte, valor 1) com o jogo 2 vazio.
pub(crate) const OCEANIA_CLUB_GROUP_OPS: [OceaniaClubChampScheduleOp; 6] = [
    Cqq12D(0, 0xa, 0, 1, 4, 2, 1), Cqq12L(0, 2, -1, -1, -1, 0), Cqq12D(1, 0xc, 0, 1, 6, 1, 1),
    Cqq12L(1, 2, -1, -1, -1, 0), Cqq12D(2, 0xe, 0, 1, 1, 2, 1), Cqq12L(2, 2, -1, -1, -1, 0),
];
/// Os jogos do grupo da fase −1.
pub(crate) const OCEANIA_GROUP_A_MATCHES: [OceaniaClubChampScheduleOp; 6] = [
    Cqq12L(0, 0, 4, 1, 3, 4), Cqq12L(0, 1, 4, 2, 3, 0x40), Cqq12L(1, 0, 6, 1, 3, 4), Cqq12L(1, 1, 6, 2, 3, 0x40),
    Cqq12L(2, 0, 1, 1, 3, 4), Cqq12L(2, 1, 1, 1, 3, 0x40),
];
/// Os jogos do grupo da fase 0.
pub(crate) const OCEANIA_GROUP_B_OPS: [OceaniaClubChampScheduleOp; 6] = [
    Cqq12L(0, 0, 5, 1, 3, 0x40), Cqq12L(0, 1, 5, 2, 3, 4), Cqq12L(1, 0, 0, 1, 3, 0x40), Cqq12L(1, 1, 0, 2, 3, 4),
    Cqq12L(2, 0, 2, 1, 3, 4), Cqq12L(2, 1, 2, 1, 3, 0x40),
];
/// O mata-mata em jogo único: semifinal em 17/1, final e terceiro lugar em 19/1 do ano seguinte.
pub(crate) const OCEANIA_CLUB_KNOCKOUT_OPS: [OceaniaClubChampScheduleOp; 54] = [
    Cqq12S(0, 0x10, 0, 1, 3), Cqq12J(0, 0x11, 0, 1, 4, 2, 0x40), Cqq12W(0x7, 0x82), Cqq12W(0x9, 0), Cqq12W(0xb, 0),
    Cqq12W(0xd, 0x283), Cqq12B(0x17, 8), Cqq12W(0x18, 4), Cqq12W(0x1a, 2), Cqq12W(0x1c, 4), Cqq12W(0xf, 0),
    Cqq12W(0x1e, 0), Cqq12B(0x20, 0), Cqq12B(0x21, 1), Cqq12B(0x22, 0), Cqq12X(0x5c, 0), Cqq12X(0x60, 0),
    Cqq12X(0x64, 0), Cqq12S(1, 0x12, 0, 1, 5), Cqq12J(1, 0x13, 0, 1, 6, 1, 4), Cqq12W(0x6f, 0x96), Cqq12W(0x71, 1),
    Cqq12W(0x73, 0), Cqq12W(0x75, 0x83), Cqq12B(0x7f, 8), Cqq12W(0x80, 2), Cqq12W(0x82, 1), Cqq12W(0x84, 0),
    Cqq12W(0x77, 0), Cqq12W(0x86, 0), Cqq12B(0x88, 0), Cqq12B(0x89, 1), Cqq12B(0x8a, 0), Cqq12X(0xc4, 0),
    Cqq12X(0xc8, 0), Cqq12X(0xcc, 0), Cqq12S(2, 0x12, 0, 1, 5), Cqq12J(2, 0x13, 0, 1, 6, 1, 0x40), Cqq12W(0xd7, 0x8c),
    Cqq12W(0xd9, 2), Cqq12W(0xdb, 0), Cqq12W(0xdd, 0x83), Cqq12B(0xe7, 8), Cqq12W(0xe8, 2), Cqq12W(0xea, 1),
    Cqq12W(0xec, 0), Cqq12W(0xdf, 0), Cqq12W(0xee, 0), Cqq12B(0xf0, 0), Cqq12B(0xf1, 1), Cqq12B(0xf2, 0),
    Cqq12X(0x12c, 0), Cqq12X(0x130, 0), Cqq12X(0x134, 0),
];

/// Stage spec of the Oceania Club Championship: groups below 1, knockout at 1, None above.
///
/// [pt] A fase do Campeonato de Clubes da Oceania: abaixo de 1 os grupos (id 0x420 + fase; os jogos de cada grupo para −1
/// e 0), 1 o mata-mata, acima nada.
pub(crate) fn oceania_stage_spec(fase: u32) -> Option<OceaniaStage> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [OceaniaClubChampScheduleOp] = match f {
            -1 => &OCEANIA_GROUP_A_MATCHES,
            0 => &OCEANIA_GROUP_B_OPS,
            _ => &[],
        };
        Some(OceaniaStage { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x420), extra_check: false, size: 0xc3, row: 0x93,
            ops: &OCEANIA_CLUB_GROUP_OPS, ops2: jogos, name: "cqq_12 grupos da oceania" })
    } else if f == 1 {
        Some(OceaniaStage { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xc5,
            ops: &OCEANIA_CLUB_KNOCKOUT_OPS, ops2: &[], name: "cqq_12 mata-mata da oceania" })
    } else {
        None
    }
}

/// rule: the Oceania Club Championship has two groups (stages −1 and 0, ids 0x41f and 0x420) of three rounds on 10, 12
/// and 14 January of the year after the cup year, with each group's fixed matches; the knockout (stage 1) in single
/// matches: semi-final on 17/1, third place and final on 19/1 (venue 4 in the final); category 8; tie-break 0x283 in
/// the semi-final and 0x83 after; no prizes
/// Original: `OceaniaClubChamp::create_schedule` (0x797230, oceania_club_champ.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x797230 } (thiscall, ret 0x10, oceania_club_champ.cpp; a virtual +0x3c): `OceaniaClubChamp::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `oceania_stage_spec`.
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_stage_table(this, oceania_stage_spec(stage), rounds, extra, league, 0xa6a068)
}

/// The 16-team table of the Asian cups (AsiaClubChamp::build_team_table `asia_club_champ_build_team_table` and AsiaCupWinner::create_team_entries
/// `create_team_entries`, the same body).
///
/// +0x3e = 16 and the table (`exe::crt_malloc` of 0x60 at +0xa7; out of memory, the box `linha` of `arquivo` and −1); slot i
/// gets club i of the qualified list (+0xb2, pointers; `ordem` gives the index) with class and status 0. Returns 1.
///
/// [pt] A tabela dos 16 times das copas asiáticas (AsiaClubChamp::build_team_table 0x40b620 e AsiaCupWinner::
/// create_team_entries 0x40d4d0, o mesmo corpo): +0x3e = 16 e a tabela (0x944e46 de 0x60 em +0xa7; sem memória, a caixa
/// `linha` de `arquivo` e −1); a vaga i recebe o clube i dos classificados (+0xb2, ponteiros; `ordem` dá o índice) com
/// classe e situação 0. Devolve 1.
pub(crate) unsafe fn build_team_table(this: usize, file: usize, row: u32, name: &'static str, order: impl Fn(i16) -> i16) -> i32 {
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 0x10);
    let l = exe::crt_malloc(0x60);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(file, row);
        return -1;
    }
    crate::comparison::branch(name);
    let mut i: i16 = 0;
    while i < core::ptr::read_unaligned((this + cup::TEAM_COUNT) as *const i16) {
        let c = read_usize(read_usize(this + 0xb2) + order(i) as usize * 4);
        let e = read_usize(this + cup::TEAM_TABLE) + i as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = 0;
        *((e + 5) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

/// Side-by-side comparer of the team table: +0x3e, return and the 16 entries must match; frees the copy's table.
///
/// [pt] Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 16 vagas (com a devolução 1) e liberada.
pub(crate) unsafe fn compare_team_table(name: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(read_usize(this + cup::TEAM_TABLE));
        r
    }, || {
        let r = c();
        lr.set(read_usize(this + cup::TEAM_TABLE));
        core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 0x60 } else { 0 };
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", name, this, a, b, igual, list));
    a
}

/// rule: the Oceania Club Championship groups are single round robin, 3/1 points, tiebreak by codes 1, 2 and 4, two
/// qualifiers per group (the top ones go on, the others are out: rule bits 0x28), category 8 (the deciding stages one),
/// 3 substitutes on the bench and 3 changes; starts on day 20 of month 7 moved to weekday 5 [inference]; no prize fund
/// Original: `OceaniaClubChamp::set_up_rules` (0x7979b0, oceania_club_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x7979b0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `OceaniaClubChamp::set_up_rules()`: os grupos do Campeonato de Clubes da Oceania em turno único, 3/1 pontos,
/// desempates (1, 2, 4, 0), dois classificados, categoria 8, 3 no banco e 3 substituições, +0x43 = 2, os bits de regra
/// 0x28 (os de cima classificados, os outros eliminados), começo no dia 20 do mês 7 num dia da semana 5; sem prêmio.
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_11 oceania rules");
    let day = 0x14; // cqj11: the start day in Oceania
    apply_cup_rules(this, &OCEANIA_RULES, day, 3)
}

