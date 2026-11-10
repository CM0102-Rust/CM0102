use super::*;

/// Does club competition `id` count (for deciding a group/league early)? Returns 1 for three fixed lower divisions
/// (Spanish lower division, French CFA, French lower division); 0 if the competition record (107 bytes, +0x5d) is not
/// flagged; otherwise 1 only if its stage object is at stage 2 (+0x43), has rounds (+0x3e) and the first table row's
/// club (+0x57 or +0x60) is the stage's competition (+4).
///
/// [pt] { 0x4b0700 } (comp_util.cpp): a competicao `id` conta? As tres de 0x9cf6b0, 0x9cf5d8 e 0x9cf548
/// sim; sem o +0x5d do registro (0xae23d0, 107 bytes) nao; senao so a do objeto em 0xadadfc com
/// fase 2 (+0x43), rodadas (+0x3e) e a primeira rodada (+0xb1) com o clube +4 em +0x57 ou +0x60.
pub(crate) unsafe extern "C" fn competition_counts(id: i32) -> u32 {
    let g = |a: usize| read_i32(a);
    if id == g(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) || id == g(KEY_CLUB_COMP_FRENCH_CFA) || id == g(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) {
        return 1;
    }
    if read_u32(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(107)).wrapping_add(0x5d)) == 0 {
        return 0;
    }
    let c = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((id as usize).wrapping_mul(4))));
    if c.is_null() || (c.comp_type() as i8) != 2 || (c.club_count() as u16) == 0 {
        return 0;
    }
    let r = c.table();
    if r == 0 {
        return 0;
    }
    let first = Club(read_usize(r));
    let target = (c.competition().addr() as i32);
    ((first.division().addr() as i32) == target || (first.reserve_division().addr() as i32) == target) as u32
}

/// qsort comparator: orders records by the signed byte at +0x5f, descending (second minus first; a null pointer counts
/// as 0).
///
/// [pt] { 0x4b0780 } (comparacao do qsort): byte +0x5f do segundo menos o do primeiro (nulo vale 0).
pub(crate) unsafe extern "C" fn order_by_5f(a: *const usize, b: *const usize) -> i32 {
    let v = |p: usize| if p == 0 { 0 } else { read_i8(p + 0x5f) as i32 };
    v(*b) - v(*a)
}

/// qsort comparator: sorts records in descending order of their signed 16-bit word at +0x80 (b minus a).
///
/// [pt] { 0x4b07b0 } (comparacao do qsort): palavra +0x80 do segundo menos a do primeiro.
pub(crate) unsafe extern "C" fn order_by_80(a: *const usize, b: *const usize) -> i32 {
    read_i16(*b + 0x80) as i32 - read_i16(*a + 0x80) as i32
}

/// qsort comparator for club pointers: sorts by the club's word at +0x80 (reputation) in descending order, with null
/// entries placed last.
///
/// [pt] { 0x4b07d0 } (comparacao do qsort): como 0x4b07b0, com os nulos no fim (primeiro nulo: 1 se o
/// segundo nao for; segundo nulo: -1).
pub(crate) unsafe extern "C" fn order_by_80_desc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    read_i16(y + 0x80) as i32 - read_i16(x + 0x80) as i32
}

/// qsort comparator: orders records by their 16-bit word at +0x80 (the club reputation) in ascending order, null
/// entries last.
///
/// [pt] { 0x4b0810 } (comparacao do qsort): como 0x4b07d0, em ordem crescente.
pub(crate) unsafe extern "C" fn order_by_80_asc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    read_i16(x + 0x80) as i32 - read_i16(y + 0x80) as i32
}

/// Returns the units digit of round code `n` (n % 10) when `n` is below 1000, else 0; used to read the round variant
/// from a fixture's round code.
///
/// [pt] { 0x4b0850 } (comp_util.cpp): algarismo das unidades de `n` (abaixo de 1000; senao 0).
pub(crate) unsafe extern "C" fn units_digit(n: i32) -> u8 {
    if n < 1000 {
        (n % 10) as u8
    } else {
        0
    }
}

/// Tells whether competition `comp` is a friendly: it is the one stored at +4 of the friendly manager `this`, or its id
/// (+0) is among the last 127 club competitions (count at 0xae2378). Returns 1/0.
///
/// [pt] { 0x5c6710 } (friendly.cpp): a competicao e amistosa? E a guardada em +4 do objeto, ou o id dela
/// (+0) esta entre as 127 ultimas competicoes de clubes (quantidade em 0xae2378).
pub(crate) unsafe extern "thiscall" fn is_friendly(this: usize, comp: usize) -> u32 {
    if comp == read_usize(this + 4) {
        return 1;
    }
    (read_i32(comp) >= read_i32(G_CLUB_COMP_COUNT).wrapping_sub(0x7f)) as u32
}

/// Returns the person's contract inside a contract list: `this` points to an array of 80-byte contract records, and the
/// index comes from the person's 79-byte staff-state record (table 0xae2d0c, indexed by person id). Returns 0 when
/// there is no person, the id is out of range (person count 0xae2364) or the index is negative.
///
/// [pt] { 0x4cf5a0 } (contract_manager.cpp): contrato da pessoa (registros de 80 bytes a partir de +0
/// do objeto), pelo indice na tabela 0xae2d0c (79 bytes por pessoa; quantidade em 0xae2364).
/// Sem pessoa, id fora da tabela ou indice negativo: 0.
pub(crate) unsafe extern "thiscall" fn contract_of(this: usize, person: usize) -> usize {
    if person == 0 {
        return 0;
    }
    let id = *(person as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let table = *(G_STAFF_STATE_TABLE as *const usize);
    let i = *((table as isize + 79 * id as isize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(this as *const usize)).wrapping_add((80 * i) as usize)
}

/// Returns 1 if the club is a national team: national-team ids come after the clubs, so the id must be at least the
/// club count minus twice the nation count. A null club returns 0 and clears the global at 0xb67a34 (the original also
/// shows an error box, skipped here).
///
/// [pt] { 0x53b900 } (Database.cpp): o clube e uma selecao? Os ids das selecoes vem depois dos clubes
/// (total de clubes em 0xae235c menos 2 x nacoes em 0xae2350). Clube nulo: o original mostra uma
/// caixa de erro, zera 0xb67a34 e devolve 0 (aqui, sem a caixa).
pub(crate) unsafe extern "C" fn is_national_team(club: usize) -> u32 {
    if club == 0 {
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        return 0;
    }
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2));
    (read_i32(club) >= limit) as u32
}

/// Computes each club's strength key (`comp_util_get_club_strength_score`) and sorts clubs and keys descending by swapping, as `comp_util_sort_clubs_by_strength`.
///
/// [pt] O núcleo da 0x4c08f0 sobre os `n` clubes em `v` e as chaves em `k`: a chave de cada clube é a força
/// (0x4c0720; clube nulo, 0); depois, para cada i, todo j > i com chave maior troca de lugar com i (chave e clube),
/// com sinal.
pub(crate) unsafe fn sort_clubs_by_strength(v: *mut u32, k: *mut i32, n: i32) {
    for i in 0..n.max(0) as usize {
        let c = *v.add(i);
        *k.add(i) = if c != 0 { exe::comp_util_get_club_strength_score(c as usize) } else { 0 };
    }
    let mut i = 0usize;
    while (i as i32) < n - 1 {
        for j in i + 1..n as usize {
            if *k.add(i) < *k.add(j) { // cpt13: a força maior sobe
                core::ptr::swap(k.add(i), k.add(j));
                core::ptr::swap(v.add(i), v.add(j));
            }
        }
        i += 1;
    }
}

/// Sorts an array of clubs by strength (`comp_util_get_club_strength_score`), strongest first.
///
/// `comp_util_sort_clubs_by_strength(clubs, n)`: sorts the array of `n` clubs by strength (`comp_util_get_club_strength_score`), highest first,
/// keeping the keys in a temporary array (`exe::crt_malloc` / `exe::crt_free`; the original fills it with 0x60 beforehand, all of which
/// is overwritten). Out of memory: the error box (line 0x292d) and nothing else.
/// Original: `comp_util_sort_clubs_by_strength` (0x4c08f0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4c08f0 } (cdecl, comp_util.cpp): `comp_util_sort_clubs_by_strength(clubes, n)`: ordena o vetor de `n` clubes
/// pela força (0x4c0720), a maior primeiro, com as chaves num vetor temporário (0x944e46 / 0x9452ca; a original o
/// enche de 0x60 antes, tudo sobrescrito). Sem memória: a caixa (linha 0x292d) e nada.
pub(crate) unsafe extern "C" fn comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let k = exe::crt_malloc((n as u32).wrapping_mul(4));
    if k == 0 {
        return show_error_at(0x99ec20, 0x292d);
    }
    sort_clubs_by_strength(v as *mut u32, k as *mut i32, n);
    if n > 1 {
        crate::comparison::branch("cpt_13 clubs sorted");
    }
    exe::crt_free(k);
}

/// Finds or adds the club's record in the competition statistics and stores the position at +0x72.
///
/// `Comp_Stats::set_club_final_position(club, position)`: finds (`exe::comp_stats_find_club_entry`) or adds (`exe::comp_stats_add_club_entry`) the club's record in the
/// competition statistics and stores the position (a byte) at +0x72; returns the record. Null club: the error box (line
/// 0x680).
/// Original: `Comp_Stats::set_club_final_position` (0x4a2540, comp_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x4a2540 } (thiscall, ret 8, comp_stats.cpp): `Comp_Stats::set_club_final_position(clube, posição)`: acha
/// (0x4a4850) ou acrescenta (0x4aceb0) o registro do clube na estatística da competição e grava a posição (byte) em
/// +0x72; devolve o registro. Clube nulo: a caixa (linha 0x680).
pub(crate) unsafe extern "thiscall" fn set_club_final_position(this: usize, club: usize, pos: u32) -> usize {
    if club == 0 {
        show_error_at(0x99ca94, 0x680);
        return 0; // simplification: the original returns the box’s eax; the caller does not read it
    }
    let mut e = exe::comp_stats_find_club_entry(this, club);
    if e == 0 {
        e = exe::comp_stats_add_club_entry(this, club);
    }
    crate::comparison::branch("cpt_13 position recorded");
    core::ptr::write_volatile((e + 0x72) as *mut u8, pos as u8); // cpt13: the position
    e
}

/// The calls `cup_rank_teams` makes to `set_club_final_position` (Comp_Stats set_club_final_position: club, position), in order, with the vector
/// `v` already sorted.
///
/// From the second-to-last round (+0x3c − 2) backward ([+0xa3] + r × 0x68, the count of who goes out in +0x1a), until a
/// null club at the start of the round. In the first round seen, those who go out in it all get position 1 (without
/// checking for nulls); in each round, the following ones get the counter's position (which goes up with every slot,
/// null or not), skipping the nulls.
///
/// `gravar` = 0 only builds the list. Returns the (club, position) list.
///
/// [pt] As chamadas da 0x5223a0 à 0x4a2540 (clube, posição), na ordem, com o vetor `v` já ordenado: da penúltima rodada
/// (+0x3c − 2) para trás ([+0xa3] + r × 0x68, a conta de quem sai em +0x1a), até um clube nulo no começo da rodada.
/// Na primeira rodada vista, os que saem nela ganham todos a posição 1 (sem olhar nulos); em cada rodada, os seguintes
/// ganham a posição do contador (que sobe a cada vaga, nula ou não), pulando os nulos. `gravar` = 0 só monta a lista.
pub(crate) unsafe fn set_final_positions_by_round(this: Cup, v: *const u32, write: usize) -> Vec<(usize, u8)> {
    let mut chamadas: Vec<(usize, u8)> = Vec::new();
    let mut s: i16 = 0; // the index into the vector (bp signed)
    let mut b: u8 = 1; // the position counter
    let mut a: u8 = 1; // the pushed position (only the low byte counts; the original pushes the dword with leftovers on top)
    let mut r = this.round_count().wrapping_sub(2) as i32;
    while r >= 0 {
        if *v.offset(s as isize) == 0 {
            break;
        }
        let count = |r: i32| read_i16(this.round_table() + r as usize * 0x68 + 0x1a) as i32;
        if r == this.round_count() as i32 - 2 {
            let mut j = 0;
            while j < count(r) {
                let c = *v.offset(s as isize) as usize;
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if write != 0 {
                    exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), c, a as u32);
                }
                b = b.wrapping_add(1);
                j += 1;
            }
        }
        a = b; // cpt13: the round’s position
        let mut j = 0;
        while j < count(r) {
            let c = *v.offset(s as isize) as usize;
            if c != 0 {
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if write != 0 {
                    crate::comparison::branch("cpt_13 position by round");
                    exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), c, a as u32);
                }
            }
            b = b.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    chamadas
}

/// Returns the club's side in the fixture: 1 home, 2 away, 3 neutral, from the fixture's stadium and the two clubs'
/// stadiums (national sides: by the stadium's nation).
///
/// `comp_util_get_fixture_venue_side(club, fixture)`. Without a venue in the fixture (+0x18): by the home club (+0x1c),
/// 3 if it has no stadium (+0x69), otherwise 1 for the home club and 2 for the other. National side (`exe::database_is_national_team`): 1 if
/// the venue's nation (+0x38, +0x1f) is the club's (+0x53), 2 if it is the opponent's, otherwise 3. Club: if the venue
/// is the club's stadium and not the opponent's, 1; if it is both clubs' stadium, 1 for the home club and 2 for the
/// other; if only the opponent's, 2; if neither's, 3. The caller reads only `al`.
/// Original: `comp_util_get_fixture_venue_side` (0x4afa80, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afa80 } (cdecl, comp_util.cpp): `comp_util_get_fixture_venue_side(clube, jogo)`: o lado do clube no jogo:
/// 1 em casa, 2 fora, 3 neutro. Sem campo no jogo (+0x18): pelo mandante (+0x1c), 3 se ele não tem estádio (+0x69),
/// senão 1 para o mandante e 2 para o outro. Seleção (0x53b900): 1 se a nação do campo (+0x38, +0x1f) é a do clube
/// (+0x53), 2 se é a do adversário, senão 3. Clube: o campo é o estádio do clube e não o do adversário, 1; dos dois,
/// 1 para o mandante e 2 para o outro; só do adversário, 2; de nenhum, 3. Quem chama lê só o al.
pub(crate) unsafe extern "C" fn comp_util_get_fixture_venue_side(club: usize, game: usize) -> u8 {
    let game = Fixture(game);
    let club = Club(club);
    let field = (game.stadium() as u32);
    let home = read_ptr(game.addr() + fixture::HOME_CLUB);
    if field == 0 {
        if read_u32(home + club::STADIUM) == 0 {
            return 3;
        }
        return if club.addr() != home { 2 } else { 1 };
    }
    if exe::database_is_national_team(club) != 0 {
        let nation = read_ptr(field as usize + 0x38);
        if nation != 0 && read_u32(nation + city::NATION) == (club.nation().addr() as u32) {
            return 1;
        }
        let home = read_ptr(game.addr() + fixture::HOME_CLUB);
        let other = if home == club.addr() { read_ptr(game.addr() + fixture::AWAY_CLUB) } else { home };
        if nation != 0 && read_u32(nation + city::NATION) == read_u32(other + club::NATION) {
            return 2;
        }
        return 3;
    }
    let home = read_ptr(game.addr() + fixture::HOME_CLUB);
    let other = if home == club.addr() { read_ptr(game.addr() + fixture::AWAY_CLUB) } else { home };
    let field = (game.stadium() as u32);
    if field != (club.stadium().addr() as u32) {
        return if field != read_u32(other + club::STADIUM) { 3 } else { 2 };
    }
    if field == read_u32(other + club::STADIUM) {
        return if club.addr() != home { 2 } else { 1 }; // the same stadium for both
    }
    crate::comparison::branch("cpq_12 at home");
    1 // the club’s stadium
}

/// The qsort order of the nation coefficients: highest first.
///
/// `compare_nation_coefficients_desc(a, b)`: the qsort comparison (12 bytes per entry, the coefficient as a double at
/// +4) used by game_update_nation_coefficients 0x5d6187 (on the day of the month from table 0x9cd200): higher first
/// (−1); lower or unordered (NaN, the C0 flag of fcomp), 1; equal, 0. Hot (inside qsort): no counter. The original
/// sorts the array and frees it right afterwards, so the order never reaches the game (only the comparison run notices
/// it).
/// Original: `compare_nation_coefficients_desc` (0x4afba0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afba0 } (cdecl, comp_util.cpp): `compare_nation_coefficients_desc(a, b)`: a ordem do qsort (12 bytes por
/// entrada, o coeficiente em double em +4) da game_update_nation_coefficients 0x5d6187 (no dia do mês da tabela
/// 0x9cd200): maior antes (−1); menor ou sem ordem (NaN, a flag C0 do fcomp), 1; igual, 0. Quente (o qsort): sem
/// contador. A original ordena e libera o vetor logo depois: a ordem não chega ao jogo (só o comparando acusa).
pub(crate) unsafe extern "C" fn compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let x = read_f64(a + 4);
    let y = read_f64(b + 4);
    if x > y {
        return -1; // o maior antes
    }
    if !(x >= y) {
        return 1;
    }
    0
}

/// Swaps two randomly drawn entries of a pointer array.
///
/// [pt] As trocas de `vezes` pares sorteados (0x90cfc0(n) duas vezes; a primeira dá o índice que recebe o item do segundo)
/// no vetor de ponteiros `itens`, como as duas originais (índices de 16 bits com sinal).
pub(crate) unsafe fn swap_random_pair(items: usize, n: i32) {
    let a = friendly_random(n) as i16 as isize;
    let b = friendly_random(n) as i16 as isize;
    let pa = (items as isize + 4 * a) as usize;
    let pb = (items as isize + 4 * b) as usize;
    let (va, vb) = (read_usize(pa), read_usize(pb));
    write_usize(pa, vb);
    write_usize(pb, va);
}

/// shuffles an array of n pointers with 3 × n swaps of drawn pairs
/// Original: `comp_util_shuffle_pointer_array` (0x4afb40, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afb40 } (cdecl, comp_util.cpp): `comp_util_shuffle_pointer_array(itens, n)`: embaralha o vetor de `n` (curto)
/// ponteiros com 3 × n trocas de pares sorteados (0x90cfc0(n) duas vezes cada).
pub(crate) unsafe extern "C" fn comp_util_shuffle_pointer_array(items: usize, n: u32) {
    let n = n as u16 as i16 as i32;
    if 3 * n <= 0 {
        return;
    }
    crate::comparison::branch("cpm_3 shuffle pointers");
    let mut k = 0i16;
    loop {
        swap_random_pair(items, n);
        k = k.wrapping_add(1); // one more swap
        if k as i32 >= 3 * n {
            break;
        }
    }
}

/// Sorts the clubs by reputation (nulls last) and shuffles the first min(3 × picks, n).
///
/// `comp_util_sort_and_shuffle_top_clubs(clubs, n, picks)`: sorts the `n` clubs by reputation, highest first and nulls
/// at the end (qsort `exe::crt_qsort` with 0x4b07d0), and shuffles the first m = min(3 × picks, n) with m swaps of randomly
/// drawn pairs among them. `n` negative or smaller than `picks`: the "Error" box (the original); `picks` < 1: nothing.
/// Original: `comp_util_sort_and_shuffle_top_clubs` (0x4afbd0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afbd0 } (cdecl, comp_util.cpp): `comp_util_sort_and_shuffle_top_clubs(clubes, n, escolhas)`: ordena os `n`
/// clubes pela reputação, maior primeiro e nulos no fim (qsort 0x945333 com 0x4b07d0), e embaralha os m = min(3 ×
/// escolhas, n) primeiros com m trocas de pares sorteados entre eles. `n` negativo ou menor que `escolhas`: caixa
/// "Error" (a original); `escolhas` < 1: nada.
pub(crate) unsafe extern "C" fn comp_util_sort_and_shuffle_top_clubs(clubs: usize, n: u32, escolhas: u32) {
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS)(clubs, n, escolhas); // a caixa
    }
    if e < 1 {
        return;
    }
    exe::crt_qsort(clubs, nn as i32 as u32, 4, 0x4b07d0);
    let mut m = e.wrapping_mul(3);
    if m > nn {
        m = nn;
    }
    if m <= 0 {
        return;
    }
    crate::comparison::branch("cpm_3 shuffle the best");
    for _ in 0..m as i32 { // m trocas
        swap_random_pair(clubs, m as i32);
    }
}

/// A club's rating in the league stats (`comp_util_get_league_clash_type`): weighted average of 4 attributes, weights scaled by games under 25.
///
/// [pt] A nota de um clube na estatística da liga (0x4b02e0): pesos 1 nos 4 atributos (+0x72..+0x75; +0x75 e +0x74 com
/// o peso [0x968968]) ou, com menos de 25 jogos (+4), proporcionais aos jogos; a média ponderada, na ordem do x87.
pub(crate) unsafe fn league_stats_rating(e: usize, b73: f64) -> f64 {
    let (mut p, mut k73, mut k72) = (read_f64(rdata::F64_0_5), 1.0f64, 1.0f64);
    let games = read_u8(e + 4);
    if games < 0x19 {
        let v = games as f64;
        p = v * read_f64(rdata::F64_0_02);
        k73 = v * read_f64(rdata::F64_0_04);
        k72 = read_f64(rdata::F64_4) - v * read_f64(rdata::F64_0_1200480192);
    }
    let s = read_u8(e + 0x75) as f64 * p + read_u8(e + 0x72) as f64 * k72 + b73 * k73 + read_u8(e + 0x74) as f64 * p;
    s / (p + p + k72 + k73)
}

/// Classifies a league match between two clubs: 1 = title clash, 3 = zone clash, 0 = none, from the weighted statistics
/// ratings (`exe::comp_stats_find_club_entry`) against a third of the clubs or 0.2 × clubs + `+0xbe` (+ `+0xbf`).
///
/// `comp_util_get_league_clash_type(a, b)`: both clubs must be in the same division (+0x57, neither reserve nor minor:
/// 0x7e6f20), its league (`[0xadadfc][id]`) must have statistics (+8, `exe::comp_stats_find_club_entry` for both), no stages (+0x30 = −1) and
/// +0x43 = 2, and the difference in +0x73 must be at most a quarter of the clubs (+0x3e). With the ratings
/// (league_stats_rating): 1 = title clash (virtual +0x98 = 0, both below a third of the clubs and each not below the
/// other's +0x74); 3 = zone clash (+0x98 ≠ 0, both up to 0.2 × clubs + +0xbe, or, with +0xbf, up to 0.2 × clubs + +0xbe
/// + +0xbf, and the same cross-check of +0x74); otherwise 0. Returns only `al`. No SEH, no random draw; the virtual
/// +0x98 is called up to three times, as in the original.
/// Original: `comp_util_get_league_clash_type` (0x4b02e0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b02e0 } (cdecl, comp_util.cpp): `comp_util_get_league_clash_type(a, b)`: os dois clubes na mesma divisão
/// (+0x57, não reserva nem menor: 0x7e6f20), a liga dela ([0xadadfc][id]) com estatística (+8, 0x4a4850 dos dois),
/// sem fases (+0x30 = −1) e com +0x43 = 2, e a diferença de +0x73 até um quarto dos clubes (+0x3e). Com as notas
/// (league_stats_rating): 1 = briga pelo título (virtual +0x98 = 0, as duas abaixo de um terço dos clubes e cada uma não abaixo
/// do +0x74 do outro); 3 = briga na zona (+0x98 ≠ 0, as duas até 0,2 × clubes + +0xbe, ou com +0xbf até 0,2 × clubes
/// + +0xbe + +0xbf, e o mesmo cruzamento do +0x74); senão 0. Devolve só `al`. Sem SEH, sem sorteio; a virtual +0x98
/// é chamada até três vezes, como na original.
pub(crate) unsafe extern "C" fn comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let b = Club(b);
    let a = Club(a);
    let comp = a.division().addr();
    if comp == 0 || comp != b.division().addr() {
        return 0;
    }
    let smaller: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    if smaller(comp) != 0 {
        return 0;
    }
    let league = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_u32(a.division().addr()) as usize)));
    if league.is_null() || league.comp_stats() == 0 {
        return 0;
    }
    let ea = exe::comp_stats_find_club_entry(league.comp_stats(), a);
    let eb = exe::comp_stats_find_club_entry(league.comp_stats(), b);
    if ea == 0 || eb == 0 || league.current_stage() as i32 != -1 || league.comp_type() != 2 {
        return 0;
    }
    let (a73, b73) = (read_u8(ea + 0x73) as i32, read_u8(eb + 0x73) as i32);
    let clubs = league.club_count() as i32;
    if (a73 - b73).abs() > ftol(clubs as f64 * read_f64(rdata::F64_0_25)) as i16 as i32 {
        return 0;
    }
    let (na, nb) = (league_stats_rating(ea, a73 as f64), league_stats_rating(eb, b73 as f64));
    let (a74, b74) = (read_u8(ea + 0x74) as f64, read_u8(eb + 0x74) as f64);
    let virtual98 = || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(league.addr()) + 0x98));
        f(league.addr())
    };
    // x87 comparisons: NaN counts as ”less” (C0) and ”less or equal” (C0|C3)
    let cruzadas = !(a74 > nb) && !(b74 > na);
    if virtual98() == 0 {
        let t = clubs as f64 * read_f64(rdata::F64_0_3333333333);
        if !(na >= t) && !(nb >= t) && cruzadas {
            return 1; // title race
        }
    }
    let be = league.promoted_count() as i8 as f64;
    if virtual98() != 0 {
        let t = clubs as f64 * read_f64(rdata::F64_0_2) + be;
        if !(na > t) && !(nb > t) && cruzadas {
            return 3;
        }
    }
    if virtual98() == 0 {
        return 0;
    }
    let bf = league.promotion_playoff_count() as i8;
    if bf == 0 {
        return 0;
    }
    let t = clubs as f64 * read_f64(rdata::F64_0_2) + be + bf as f64;
    if !(na > t) && !(nb > t) && cruzadas {
        3
    } else {
        0
    }
}

// simplification: ~4 million calls in the career: compares only the value, without regions (the routine only reads; 0x4a4850 may
// reorder the statistics, and the original already leaves it sorted for the copy).

/// Computes a club's strength score: money (+0x65 / 400 or balance / 300) + 16 × reputation + the off-match ratings
/// (`off_match_score`) of the 15 picked (0x6e2000) from the squad; null club: −10,000,000.
///
/// `comp_util_get_club_strength_score(club)`: the money (without the finance table `[0xae2a38]`: +0x65 / 400; with it,
/// the 64-bit balance / 300 via `get_club_finance`), plus 16 × the reputation (+0x80), plus the off-match rating (`off_match_score`,
/// 0x840, limit −1) of the 15 picked players (0x6e2000, codes 1, 0x804, 0x84, 3 × 0x204, 0x810, 0x90, 3 × 0x210, 4 ×
/// 0x40) among the players (+0x61) of the squad (`database_collect_club_squad`). Null club: −10,000,000. No SEH, no random draw of its own.
/// The original pushes the number of players as a dword with only the low byte written (the rest is stack leftover);
/// 0x6e2000 reads only the byte, and the copy passes the byte zero-extended.
/// Original: `comp_util_get_club_strength_score` (0x4c0720, comp_util.cpp, cdecl).
///
/// [pt] { 0x4c0720 } (cdecl, comp_util.cpp): `comp_util_get_club_strength_score(clube)`: o dinheiro (sem tabela de
/// finanças [0xae2a38]: +0x65 / 400; com ela, o saldo de 64 bits / 300 pela 0x5a0590), mais 16 × a reputação (+0x80),
/// mais a nota fora da partida (0x6e6340, 0x840, limite −1) dos 15 escalados (0x6e2000, códigos 1, 0x804, 0x84,
/// 3 × 0x204, 0x810, 0x90, 3 × 0x210, 4 × 0x40) entre os jogadores (+0x61) do elenco (0x541f90). Clube nulo:
/// −10.000.000. Sem SEH, sem sorteio próprio. A original empurra o número de jogadores como dword com só o byte de
/// baixo gravado (o resto é sobra de pilha); a 0x6e2000 só lê o byte, e a cópia passa o byte estendido.
pub(crate) unsafe extern "C" fn comp_util_get_club_strength_score(club: usize) -> i32 {
    let club = Club(club);
    if club.is_null() {
        return -10_000_000;
    }
    let finances = read_usize(G_FINANCE_TABLE);
    let mut strength: i32 = 0;
    if finances == 0 {
        strength = club.cash() / 400;
    } else {
        let f = exe::financetable_get_club_finance(finances, club);
        if f != 0 {
            strength = (read_i64(f) / 300) as i32;
        }
    }
    strength = strength.wrapping_add((club.reputation() as i32) << 4); // the reputation
    let mut squad = [0usize; 50];
    let mut players = [0usize; 50];
    let mut escalados = [0usize; 15];
    let codes: [u16; 15] = [1, 0x804, 0x84, 0x204, 0x204, 0x204, 0x810, 0x90, 0x210, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    let mut k: i8 = 0;
    for &p in squad.iter() {
        if p != 0 && read_usize(p + staff::PLAYER) != 0 {
            players[k as usize] = p;
            k = k.wrapping_add(1);
        }
    }
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    pick(players.as_ptr() as usize, k as u8 as u32, codes.as_ptr() as usize, 0xf, 0, escalados.as_mut_ptr() as usize, 0x840, club.addr(), 0, 0, 0, 0, 0, 0, 0);
    for (j, &p) in escalados.iter().enumerate() {
        if p != 0 {
            strength = strength.wrapping_add(exe::match_man_score_player_for_selection(p, codes[j] as u32, 0x840, club, -1, 0));
        }
    }
    strength
}

// simplification: 1,117 calls; only the value. The line-up and the ratings have their own comparers, and the fitness that the rating
// updates once (0x62ed70, with a random roll); the original already leaves it ready for the copy.

/// Sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) from the competition's
/// first row; not found: 0.
///
/// `comp_util_get_latest_placed_club(comp, place)`: sorts the competition history (qsort `exe::crt_qsort` with `database_compare_comp_history_rows`),
/// searches (bsearch `exe::crt_bsearch`) for the competition with year −1 (any), walks back to its first row and returns the club
/// in `place` (0 winner +0xa, 1 runner-up +0xe, 2 third +0x12); not found: 0. Any other place: the original (the
/// comp_util.cpp error box). The key on the original's stack has only +4 and +8 written, and `database_compare_comp_history_rows` reads only those
/// two; the copy zeroes the rest.
/// Original: `comp_util_get_latest_placed_club` (0x4afd80, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afd80 } (cdecl, comp_util.cpp): `comp_util_get_latest_placed_club(comp, lugar)`: ordena o histórico de
/// competições (qsort 0x945333 com 0x540780), busca (bsearch 0x946464) a competição com ano −1 (qualquer), volta até a
/// primeira linha dela e devolve o clube do `lugar` (0 campeão +0xa, 1 vice +0xe, 2 terceiro +0x12); não achou: 0.
/// Outro lugar: a original (a caixa do comp_util.cpp). A chave na pilha da original só tem +4 e +8 gravados, e a
/// 0x540780 só lê esses dois; a cópia zera o resto.
pub(crate) unsafe extern "C" fn comp_util_get_latest_placed_club(comp: usize, place: u32) -> usize {
    let place = place as u8;
    if place > 2 {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_COMP_UTIL_GET_LATEST_PLACED_CLUB)(comp, place as u32); // message box: original only
    }
    exe::crt_qsort(read_usize(G_CLUB_COMP_HISTORY_TABLE), read_i32(G_CLUB_COMP_HISTORY_COUNT) as u32, 0x1a, 0x540780);
    let mut key = [0u8; 0x1a];
    key[4..8].copy_from_slice(&comp.to_le_bytes());
    key[8..10].copy_from_slice(&0xffffu16.to_le_bytes()); // o ano: qualquer
    let mut p = exe::crt_bsearch((key.as_ptr()) as usize, read_usize(G_CLUB_COMP_HISTORY_TABLE), read_i32(G_CLUB_COMP_HISTORY_COUNT) as u32, 0x1a, 0x540780);
    if p == 0 {
        return 0;
    }
    let t = read_usize(G_CLUB_COMP_HISTORY_TABLE);
    while p > t {
        p -= 0x1a;
        if exe::database_compare_comp_history_rows(key.as_ptr(), p) != 0 {
            p += 0x1a;
            break;
        }
    }
    read_usize(p + [0xa, 0xe, 0x12][place as usize]) // champion, runner-up, third
}

/// Rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition.
///
/// The fixture (`partida`) is rebuilt from 18 consecutive slots of the `noticia` (news item, `get_slot`) starting at
/// `*cursor` (which advances by 18): +0xc and +0x10 the clubs, +0x3d, +0x3e, +4 the stadium, +0 the competition, +0x42,
/// +0x32, +0x30, +0x43/+0x44, +0x4b/+0x4c and +0x47/+0x48 the scores, +0x2a, +0x28 and +8. Then the pointers: clubs
/// (0xae23b4, 0x245 bytes) at +0x1c/+0x20, stadium (0xae23b0, 0x4e) at +0x18 and competition (0xae23d0, 0x6b) at +0x14;
/// id −1: 0.
/// Original: `comp_util_read_fixture_from_news_slots` (0x4b0010, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b0010 } (cdecl, comp_util.cpp): a partida (`partida`) refeita de 18 vagas seguidas da
/// `noticia` (0x785830) a partir de `*cursor` (que avança 18): +0xc e +0x10 os clubes, +0x3d, +0x3e,
/// +4 o estádio, +0 a competição, +0x42, +0x32, +0x30, +0x43/+0x44, +0x4b/+0x4c e +0x47/+0x48 os
/// placares, +0x2a, +0x28 e +8. Depois os ponteiros: clubes (0xae23b4, 0x245 bytes) em +0x1c/+0x20,
/// estádio (0xae23b0, 0x4e) em +0x18 e competição (0xae23d0, 0x6b) em +0x14; id −1: 0.
pub(crate) unsafe extern "C" fn comp_util_read_fixture_from_news_slots(news: usize, fixture: usize, cursor: *mut u8) {
    // simplification: the original pushes the whole eax (remainder of the previous slot in the high bits); 0x785830 only reads the byte
    let mut v = *cursor;
    let mut read = || {
        let r = exe::newsitem_get_slot(news, v as u32);
        v = v.wrapping_add(1);
        r
    };
    let d = |o: usize, x: u32| write_u32(fixture + o, x);
    let w = |o: usize, x: u32| write_u16(fixture + o, x as u16);
    let b = |o: usize, x: u32| write_u8(fixture + o, x as u8);
    d(0xc, read());
    d(0x10, read());
    b(0x3d, read());
    b(0x3e, read());
    d(4, read());
    d(0, read());
    b(0x42, read());
    w(0x32, read());
    w(0x30, read());
    b(0x43, read());
    b(0x44, read());
    b(0x4b, read());
    b(0x4c, read());
    b(0x47, read());
    b(0x48, read());
    w(0x2a, read());
    w(0x28, read());
    d(8, read());
    *cursor = v;
    let pointer = |o: usize, table: usize, size: u32| {
        let i = read_u32(fixture + o);
        if i == 0xffff_ffff { 0 } else { read_u32(table).wrapping_add(i.wrapping_mul(size)) }
    };
    d(0x1c, pointer(0xc, G_CLUB_TABLE, 0x245)); // home club
    d(0x20, pointer(0x10, G_CLUB_TABLE, 0x245));
    d(0x18, pointer(4, G_STADIUM_TABLE, 0x4e));
    d(0x14, pointer(0, G_CLUB_COMP_TABLE, 0x6b));
}

/// Classifies the cup tie result for the club, from the final margin, the side and the reputation gap.
///
/// The category (1 to 7) of the tie result for the `clube` (club) in the `partida` (fixture): the final margin (score
/// +0x4b/+0x4c, or +0x47/+0x48, or +0x43/+0x44, the first one without 0xff) from the club's side (home = +0x1c) and the
/// clubs' reputations (+0x80 / 500). 1 thrashing, 2 and 3 wins against stronger sides, 4 ordinary, 5 and 7 defeats to
/// weaker sides, 6 upset. Only `al` is faithful (the caller uses `al`).
/// Original: `comp_util_classify_tie_result_for_news` (0x4b0890, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b0890 } (cdecl, comp_util.cpp): a categoria (1 a 7) do resultado da eliminatória para o
/// `clube` na `partida`: a margem final (placar +0x4b/+0x4c, ou +0x47/+0x48, ou +0x43/+0x44, o
/// primeiro sem 0xff) do lado do clube (da casa = +0x1c) e as reputações (+0x80 / 500) dos clubes.
/// 1 goleada, 2 e 3 vitórias contra mais fortes, 4 comum, 5 e 7 derrotas para mais fracos, 6
/// zebra. Só o al é fiel (quem chama usa al).
pub(crate) unsafe extern "C" fn comp_util_classify_tie_result_for_news(club: usize, fixture: usize) -> u8 {
    let fixture = Fixture(fixture);
    let home = fixture.home_club();
    let away = fixture.away_club();
    let h = (home.reputation() as i32 / 500) as i8 as i32;
    let a = (away.reputation() as i32 / 500) as i8 as i32;
    let (g1, g2) = if (fixture.home_aggregate() as u8) != 0xff {
        ((fixture.home_aggregate() as u8), (fixture.away_aggregate() as u8))
    } else if (fixture.home_goals_extra_time() as u8) != 0xff {
        ((fixture.home_goals_extra_time() as u8), (fixture.away_goals_extra_time() as u8))
    } else {
        ((fixture.home_goals() as u8), (fixture.away_goals() as u8))
    };
    let m = g1.wrapping_sub(g2) as i8;
    // from the club’s point of view: `eu` and `ele` the reputations, `vitoria` the margin in its favour
    let (eu, ele) = if club == home.addr() { (h, a) } else { (a, h) };
    let win = if club == home.addr() { m } else { m.wrapping_neg() };
    if win == 0 {
        if eu < ele - 16 || eu - 15 > ele {
            return 6;
        }
        if eu < ele - 11 {
            return 2;
        }
        if eu < ele - 3 {
            return 3;
        }
        if eu - 5 > ele {
            return 5;
        }
        return if eu - 11 <= ele { 4 } else { 7 };
    }
    // the upset: the home side tolerates 8 / 7, the away side 13 / 7 difference in a win by 1 or 2
    let (z1, z2) = if club == home.addr() { (8, 2) } else { (13, 7) };
    if win > 0 {
        if (win < 3 && eu < ele - z1) || (win > 2 && eu < ele - z2) {
            return 6;
        }
        if win > 4 {
            return 1;
        }
        if win < 2 {
            if eu < ele - 3 {
                return 2;
            }
            if eu < ele {
                return 3;
            }
        } else if win > 2 && eu < ele {
            return 2;
        }
        return 4;
    }
    let d = win.wrapping_neg();
    if (d < 3 && ele < eu - 7) || (d > 2 && ele < eu - 2) {
        return 6;
    }
    if d < 2 {
        if eu - 6 > ele {
            return 7;
        }
        if eu - 2 > ele {
            return 5;
        }
    }
    4 // no highlight
}

/// Writes the fixture's clubs, venue, competition, score and round into 18 consecutive news slots and advances the
/// cursor.
///
/// The community map gives the 2nd argument as unused, but it is the news item. Writes the fixture into 18 consecutive
/// data slots of the news item (`set_news_field`), starting at slot `[cursor]`, and advances the cursor by 18. Order: +0xc,
/// +0x10, +0x3d, +0x3e, +4, +0, +0x42, +0x32, +0x30, +0x43, +0x44, +0x4b, +0x4c, +0x47, +0x48, +0x2a, +0x28, +8 (bytes
/// and the words +0x30/+0x32 signed; +0x28 and +0x2a unsigned). Returns what the last write returned. The original
/// keeps the index of the 18th slot in the low byte of the `cursor` argument itself (only the low part reaches
/// `set_news_field`).
/// Original: `comp_util_write_fixture_to_news_slots` (0x4afeb0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afeb0 } (cdecl, comp_util.cpp; a convenção do mapa dá o 2º argumento como sem uso, mas é a
/// notícia): grava o jogo em 18 dados seguidos da notícia (0x785790), a partir do dado [cursor], e
/// avança o cursor 18. Ordem: +0xc, +0x10, +0x3d, +0x3e, +4, +0, +0x42, +0x32, +0x30, +0x43, +0x44,
/// +0x4b, +0x4c, +0x47, +0x48, +0x2a, +0x28, +8 (bytes e palavras +0x30/+0x32 com sinal; +0x28 e
/// +0x2a sem). Devolve o que a última gravação devolveu. A original guarda o índice do 18º dado no
/// byte baixo do próprio argumento `cursor` (só a parte baixa chega à 0x785790).
pub(crate) unsafe extern "C" fn comp_util_write_fixture_to_news_slots(game: usize, news: usize, cursor: *mut u8) -> u32 {
    let b = |o: usize| read_i8(game + o) as i32 as u32;
    let s = |o: usize| read_i16(game + o) as i32 as u32;
    let values = [
        read_u32(game + 0xc), read_u32(game + 0x10), b(0x3d), b(0x3e), read_u32(game + 4), read_u32(game + 0), b(0x42), s(0x32), s(0x30),
        b(0x43), b(0x44), b(0x4b), b(0x4c), b(0x47), b(0x48), (read_u16(game + 0x2a) as u32), (read_u16(game + 0x28) as u32), read_u32(game + 8),
    ];
    let mut k = *cursor;
    let mut r = 0;
    for v in values {
        r = exe::newsitem_set_slot(news, k as i8, v);
        k = k.wrapping_add(1);
    }
    *cursor = k;
    r
}

/// Writes the stage and round names (with their genders, through template 0x9a7cfc), or just one of them; without an
/// output or under 200 bytes, the error box and 0.
///
/// `comp_util_format_stage_round_name(stage, round, output, size, style)`: without an output or with fewer than 200
/// bytes, the error box (line 0x26a1) and 0. Clears the output; with both stage and round, "{stage} {round}" (0x66f4e0
/// with template 0x9a7cfc, the 100-byte names from 0x4b4850 and the genders from `exe::comp_util_get_stage_name_gender`); with only one of them, its
/// name directly in the output (0x4b4850 with the size); with neither, just the zero. Returns 1 (0 on the error box).
/// Original: `comp_util_format_stage_round_name` (0x4bffc0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4bffc0 } (cdecl, comp_util.cpp): `comp_util_format_stage_round_name(fase, rodada, saida, tamanho, estilo)`:
/// sem saída ou com menos de 200 bytes, a caixa (linha 0x26a1) e 0. Zera a saída; com fase e rodada, "{fase} {rodada}"
/// (0x66f4e0 com o modelo 0x9a7cfc, os nomes de 100 bytes da 0x4b4850 e os gêneros da 0x4ba8f0); só uma delas, o nome
/// dela direto na saída (0x4b4850 com o tamanho); nenhuma, só o zero. Devolve 1 (0 na caixa).
pub(crate) unsafe extern "C" fn comp_util_format_stage_round_name(stage: u32, round: u32, out: *mut u8, size: i32, style: u32) -> i32 {
    if out.is_null() || size < 0xc8 {
        return original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(IDX_COMP_UTIL_FORMAT_STAGE_ROUND_NAME)(stage, round, out, size, style); // a caixa
    }
    // 0x4b4850: 4 pushes (style, id, size, output); add esp -> cdecl; 0x4ba8f0: push style; push id -> cdecl
    *out = 0;
    if stage != 0 && round != 0 {
        let (mut a, mut b) = ([0u8; 0x64], [0u8; 0x64]);
        exe::comp_util_get_stage_name(a.as_mut_ptr(), 0x64, stage, style);
        let ga = exe::comp_util_get_stage_name_gender(stage, style) as i8 as i32 as u32;
        exe::comp_util_get_stage_name(b.as_mut_ptr(), 0x64, round, style);
        let gb = exe::comp_util_get_stage_name_gender(round, style) as i8 as i32 as u32;
        // 0x66f4e0: 8 pushes; add esp, 0x20 -> cdecl (output, template, ga, ga, gb, gb, &a, &b)
        let text: unsafe extern "C" fn(*mut u8, usize, u32, u32, u32, u32, *const u8, *const u8) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        text(out, 0x9a7cfc, ga, ga, gb, gb, a.as_ptr(), b.as_ptr());
    } else if stage != 0 {
        exe::comp_util_get_stage_name(out, size, stage, style);
    } else if round != 0 {
        exe::comp_util_get_stage_name(out, size, round, style);
    } else {
        return 0; // neither of the two: 0 (0x4c0136)
    }
    1
}

/// Returns the round's value between two limits: the average with a single round, otherwise (high − low) × round /
/// (rounds − 1); outside 0..100, the error box.
///
/// `comp_util_scale_by_round(low, high, round, rounds)` (signed bytes and words; the words arrive in registers with
/// garbage above them and only the word is read): with a single round, the average (low + high) / 2; otherwise (high −
/// low) × round / (rounds − 1). Outside 0..=100: the original (error box at line 0x256e, returns 0 or 100). Returns the
/// original's whole eax (the caller pushes the whole register to `add_qualified_for_round_record`/`add_knocked_out_record`): the byte in `al` over the sum
/// (single round) or the quotient.
/// Original: `comp_util_scale_by_round` (0x4bf850, comp_util.cpp, cdecl).
///
/// [pt] { 0x4bf850 } (cdecl, comp_util.cpp): `comp_util_scale_by_round(baixo, alto, rodada, rodadas)` (bytes e palavras com
/// sinal; as palavras chegam em registradores com lixo acima, só a palavra é lida): com uma rodada só, a média
/// (baixo + alto) / 2; senão (alto − baixo) × rodada / (rodadas − 1). Fora de 0..=100: a original (caixa da linha
/// 0x256e, devolve 0 ou 100). Devolve o eax inteiro da original (quem chama empurra o registrador todo para
/// 0x86bdd0/0x86c000): o byte em al sobre a soma (rodada única) ou o quociente.
pub(crate) unsafe extern "C" fn comp_util_scale_by_round(low: u32, high: u32, round: u32, rounds: u32) -> u32 {
    let (eax, v) = interpolate_by_round(low, high, round, rounds);
    if !(0..=100).contains(&v) {
        return original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(IDX_COMP_UTIL_SCALE_BY_ROUND)(low, high, round, rounds);
    }
    if rounds as u16 != 1 {
        crate::comparison::branch("cpq_11 lineup by round");
    }
    eax
}

/// Stores competition, year and four values in the free row of the competition history (without one, grows by 20 rows
/// through `database_grow_comp_history_table`); returns the row.
///
/// `comp_util_add_history_record(league, first, second, third, extra)`: the free row of the competition history
/// (without one, it grows by 20 rows through `database_grow_comp_history_table` and uses the first new one; out of memory, 0) receives the
/// competition (the league's +4), the year `[0xae2c92]` (+8), and the four values at +0xa, +0xe, +0x12, +0x16. Returns
/// the row. No SEH, no random draw, no locals.
/// Original: `comp_util_add_history_record` (0x4afce0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afce0 } (cdecl, comp_util.cpp): `comp_util_add_history_record(liga, primeiro, segundo, terceiro, extra)`:
/// a linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60 e usa a primeira nova; sem
/// memória, 0) recebe a competição (+4 da liga), o ano [0xae2c92] (+8), e os quatro valores em +0xa, +0xe, +0x12,
/// +0x16. Devolve a linha. Sem SEH, sem sorteio, sem locais.
pub(crate) unsafe extern "C" fn comp_util_add_history_record(league: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let mut r = free_comp_history_row();
    if r == 0 {
        crate::comparison::branch("competition history grow");
        if exe::database_grow_comp_history_table(0x14) == 0 {
            return 0;
        }
        let n = read_i32(G_CLUB_COMP_HISTORY_COUNT);
        r = read_usize(G_CLUB_COMP_HISTORY_TABLE).wrapping_add(26usize.wrapping_mul(n.wrapping_sub(20) as usize));
    }
    write_usize(r + 4, read_usize(league + 4));
    write_u16(r + 8, read_u16(G_CURRENT_DATE_YEAR)); // o ano
    for (k, v) in [a, b, c, d].into_iter().enumerate() {
        write_u32(r + 0xa + 4 * k, v);
    }
    r
}

/// Returns the group (the containing stage) of a stage or round by id, from fixed ranges; −1 without a group.
///
/// `comp_util_get_parent_stage_id(id)`: 0x3e8..0x3fb, 0x41f..0x42e, 0x413, 0x414, 0x47b and 0x3fc -> 0x41c;
/// 0x473..0x474 -> 0x410; 0x475..0x479 -> 0x47a; 0x3fd..0x406 -> 0xa; 0x407..0x40b and 0x472 -> 0x14; 0x47e..0x47f ->
/// 0x1e; 0x40c..0x40e -> 0x82; 0x43d..0x444 -> 0x449; 0x445..0x448 -> 0x44a; 0x44d..0x450 -> 0x451; everything else −1
/// (in the original's test order).
/// Original: `comp_util_get_parent_stage_id` (0x4b01d0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b01d0 } (cdecl, comp_util.cpp): `comp_util_get_parent_stage_id(id)`, o grupo de uma fase pelo id: 0x3e8..0x3fb,
/// 0x41f..0x42e, 0x413, 0x414, 0x47b e 0x3fc -> 0x41c; 0x473..0x474 -> 0x410; 0x475..0x479 -> 0x47a; 0x3fd..0x406 ->
/// 0xa; 0x407..0x40b e 0x472 -> 0x14; 0x47e..0x47f -> 0x1e; 0x40c..0x40e -> 0x82; 0x43d..0x444 -> 0x449; 0x445..0x448
/// -> 0x44a; 0x44d..0x450 -> 0x451; o resto −1 (na ordem dos testes da original).
pub(crate) unsafe extern "C" fn comp_util_get_parent_stage_id(id: i32) -> i32 {
    let em = |a: i32, b: i32| id >= a && id <= b;
    if em(0x3e8, 0x3fb) {
        return 0x41c;
    }
    if em(0x473, 0x474) {
        return 0x410;
    }
    if em(0x475, 0x479) {
        return 0x47a;
    }
    if em(0x3fd, 0x406) {
        crate::comparison::branch("cqg_6 group 0xa");
        return 0xa; // cqg6: the group of stages 0x3fd..0x406
    }
    if em(0x407, 0x40b) || id == 0x472 {
        return 0x14;
    }
    if em(0x47e, 0x47f) {
        return 0x1e;
    }
    if em(0x41f, 0x42e) {
        return 0x41c;
    }
    if em(0x40c, 0x40e) {
        return 0x82;
    }
    if id == 0x413 || id == 0x414 || id == 0x47b || id == 0x3fc {
        return 0x41c;
    }
    if em(0x43d, 0x444) {
        return 0x449;
    }
    if em(0x445, 0x448) {
        return 0x44a;
    }
    if em(0x44d, 0x450) {
        return 0x451;
    }
    -1
}

