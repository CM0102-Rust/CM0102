use super::*;

/// Returns the first row of the 35-row key nation table holding the given nation, or 4.
///
/// `key_nation_index_of_nation(nação)` (nation): the first row of the table of the 35 key nations (`0xb63d60`, 72
/// bytes, nation at `+0`) with the given nation (without checking for null), or 4. Called by the match morale
/// `0x4cb250`, the retirements `0x7a7310`, the free friendly `0x5c6b70`, the prizes of the nations' day and the UEFA
/// places. TapeCall for every player: no counter.
/// Original: `key_nation_index_of_nation` (0x669340, key_nation.cpp, cdecl).
///
/// [pt] { 0x669340 } (cdecl, key_nation.cpp): `key_nation_index_of_nation(nação)`: a primeira linha da tabela das 35
/// nações-chave (0xb63d60, 72 bytes, nação em +0) com a nação dada (sem conferir a nula), ou 4. Pela moral do jogo
/// 0x4cb250, pelas aposentadorias 0x7a7310, pelo amistoso livre 0x5c6b70, pelos prêmios do dia das nações e pelas vagas
/// da UEFA. TapeCall a cada jogador: sem contador.
pub(crate) unsafe extern "C" fn key_nation_index_of_nation(nation: usize) -> u8 {
    for k in 0..0x23usize {
        if read_usize(G_LEAGUE_TABLE + 72 * k) == nation {
            return k as u8; // cpu1: the row of the given nation
        }
    }
    4
}

/// Returns the key nation row of the club: the row of its chosen nation; for a national team, 1; for a nation not
/// chosen, the row of its continent, or 3.
///
/// `key_nation_index_of_club(clube)` (club). A club that is not a national team (`0x53b900`), with a nation (`+0x53`)
/// chosen to be played (`+0x11c` of the nation): the nation's row in the table, or the "Error" box (line `0x26d`) and
/// 0. Otherwise (the original calls `0x53b900` again): national team, 1; the first row whose continent (`+4`) is that
/// of the club's nation (`+0x71`), or 3 (with no nation, 3). Called by the retirements `0x7a7310`, the weather
/// postponement `0x923630`, the days until the season `0x59b4c0`, the desired contract `0x85fc50`, the season year
/// `0x669950` and others. No counter.
/// Original: `key_nation_index_of_club` (0x669360, key_nation.cpp, cdecl).
///
/// [pt] { 0x669360 } (cdecl, key_nation.cpp): `key_nation_index_of_club(clube)`. Clube que não é seleção (0x53b900), com
/// nação (+0x53) escolhida para jogar (+0x11c da nação): a linha da nação na tabela, ou a caixa "Error" (linha 0x26d) e
/// 0. Senão (a original chama 0x53b900 de novo): seleção, 1; a primeira linha cujo continente (+4) é o da nação do clube
/// (+0x71), ou 3 (sem nação, 3). Pelas aposentadorias 0x7a7310, pelo adiamento pelo tempo 0x923630, pelos dias até a
/// temporada 0x59b4c0, pelo contrato desejado 0x85fc50, pelo ano da temporada 0x669950 e outras. Sem contador.
pub(crate) unsafe extern "C" fn key_nation_index_of_club(club: usize) -> u8 {
    let club = Club(club);
    if exe::database_is_national_team(club) == 0 {
        let nation = club.nation().addr();
        if nation != 0 && *((nation + nation::LEAGUE_SELECTED) as *const u8) != 0 {
            for k in 0..0x23usize {
                if read_usize(G_LEAGUE_TABLE + 72 * k) == nation {
                    return k as u8; // cpu1: the row of the chosen nation
                }
            }
            show_error_at(0x9eaff4, 0x26d);
            return 0;
        }
    }
    if exe::database_is_national_team(club) != 0 {
        return 1;
    }
    let nation = club.nation().addr(); // cpu1: the nation of the continent
    for k in 0..0x23usize {
        if nation != 0 && read_usize(G_LEAGUE_WORLD_CUP_CONTINENT + 72 * k) == read_usize(nation + nation::CONTINENT) {
            return k as u8; // cpu1: the continent row
        }
    }
    3
}

/// Would the club hit the `key_nation_index_of_club` "Error" box (not a national team, nation selected but not in the league table)?
///
/// [pt] Se o clube cai na caixa "Error" da 0x669360 (não é seleção, nação escolhida fora da tabela).
pub(crate) unsafe fn hits_key_nation_error(club: Club) -> bool {
    let nation = club.nation().addr();
    exe::database_is_national_team(club) == 0
        && nation != 0
        && *((nation + nation::LEAGUE_SELECTED) as *const u8) != 0
        && !(0..0x23usize).any(|k| read_usize(G_LEAGUE_TABLE + 72 * k) == nation)
}

/// Returns the club's season year: the previous year when today is before the season start day of its row.
///
/// `key_nation_get_club_season_year(clube)` (club): today's year (`[0xae2c92]`); for a club that is not a national
/// team, by the club's exe::key_nation_index_of_club(`0x669360`): with the row's dword `+0x17` not zero or the start day (`+0x15`, unsigned)
/// below 60, and today's day (`[0xae2c90]`, signed) before the start day, the previous year. The original returns only
/// the low 16 bits meaningfully (the high ones are left over from the pushed slot). Called by the club records
/// `0x444930`/`0x447430`/`0x448ad0` and the player history `0x7c3e40`/`0x7c5fe0`.
/// Original: `key_nation_get_club_season_year` (0x669950, key_nation.cpp, cdecl).
///
/// [pt] { 0x669950 } (cdecl, key_nation.cpp): `key_nation_get_club_season_year(clube)`: o ano de hoje ([0xae2c92]); para
/// clube que não é seleção, pela linha do clube (0x669360): com o dword +0x17 da linha não zero ou o dia de início
/// (+0x15, sem sinal) abaixo de 60, e o dia de hoje ([0xae2c90], com sinal) antes do dia de início, o ano anterior. A
/// original devolve só os 16 bits baixos com sentido (os altos são a sobra da vaga empurrada). Pelos registros do clube
/// 0x444930/0x447430/0x448ad0 e pela história do jogador 0x7c3e40/0x7c5fe0.
pub(crate) unsafe extern "C" fn key_nation_get_club_season_year(club: usize) -> u16 {
    let year = *(G_CURRENT_DATE_YEAR as *const u16);
    if club != 0 && exe::database_is_national_team(club) == 0 {
        let k = exe::key_nation_index_of_club(club) as i8 as isize;
        let e = (0xb63d60isize + 72 * k) as usize;
        let start = core::ptr::read_unaligned((e + 0x15) as *const u16);
        if (read_usize(e + 0x17) != 0 || start < 0x3c) && (*(G_CURRENT_DATE as *const i16) as i32) < start as i32 {
            return year.wrapping_sub(1); // cpu1: o ano anterior
        }
    }
    year
}

/// Writes to dst (if not null) the date built by 0x549ef0 from day and year (with leap flag).
///
/// [pt] Grava em `dst` (se não nulo) a data (dia, ano) montada pela 0x549ef0 (com o bissexto), como a cópia 0x417c10.
pub(crate) unsafe fn write_date(dst: usize, day: i16, year: u16) {
    if dst == 0 {
        return;
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mut b = [0u8; 8];
    data(b.as_mut_ptr(), day as u16 as u32, year as u32);
    core::ptr::copy_nonoverlapping(b.as_ptr(), dst as *mut u8, 8);
}

/// By the start and end days of the club's nation row and today, tells whether the club is in season and gives the
/// start and end dates with the right year.
///
/// Convention: cdecl, `key_nation.cpp`, SEH frame only for the local dates. `key_nation_club_in_season(clube, &início,
/// &fim)` (club, &start, &end). The row of the club's nation (`+0x53`) in the table of the 35 key nations (`0xb63d60`,
/// 72 bytes; with no nation or outside the table, row 3 `0xb63e38` and the end day of the default date `0x54a110`, 40).
/// `a` = day `+0x2a` of the row (start), `c` = the day of date `+0x39` (end; the original also copies the next 7 bytes
/// and, with club `+0x1db` ≠ −1 and byte `+0x43` 3, 4 or 5, swaps two of them, but does not use them), `t` today's day
/// (`[0xae2c90]`) and `y` the year (`[0xae2c92]`); signed 16-bit comparisons.
///
/// With a > c (a season that crosses the year): t > a, (a, y) and (c, y+1), 1; t < c, (a, y−1) and (c, y), 1; otherwise
/// (a, y) and (c, y), 0. With a ≤ c: t < a, (a, y) and (c, y), 0; t > c, (a, y+1) and (c, y+1), 1; otherwise (a, y) and
/// (c, y), 1. The dates go to `início` and `fim` when not null.
///
/// Called by the club in season `0x547770`, the fitness `0x62a920`, `0x62c680` and `0x62ce30`, the month of the season
/// end `0x4d45f0` and the transfer date `0x8cee70`. TapeCall for every player: no counter.
/// Original: `key_nation_club_in_season` (0x669bd0, key_nation.cpp, cdecl).
///
/// [pt] { 0x669bd0 } (cdecl, key_nation.cpp, quadro SEH só das datas locais): `key_nation_club_in_season(clube, &início,
/// &fim)`. A linha da nação do clube (+0x53) na tabela das 35 nações-chave (0xb63d60, 72 bytes; sem nação ou fora da
/// tabela, a linha 3 0xb63e38 e o dia de fim da data padrão 0x54a110, 40). `a` = o dia +0x2a da linha (início), `c` = o
/// dia da data +0x39 (fim; a original também copia os 7 bytes seguintes e, com o clube +0x1db ≠ −1 e o byte +0x43 3, 4
/// ou 5, troca dois deles, mas não os usa), `t` o dia de hoje ([0xae2c90]) e `y` o ano ([0xae2c92]); comparações de 16
/// bits com sinal. Com a > c (temporada que vira o ano): t > a, (a, y) e (c, y+1), 1; t < c, (a, y−1) e (c, y), 1; senão
/// (a, y) e (c, y), 0. Com a ≤ c: t < a, (a, y) e (c, y), 0; t > c, (a, y+1) e (c, y+1), 1; senão (a, y) e (c, y), 1. As
/// datas vão para `início` e `fim` quando não nulos. Pelo clube na temporada 0x547770, pela forma física 0x62a920,
/// 0x62c680 e 0x62ce30, pelo mês do fim da temporada 0x4d45f0 e pela data da transferência 0x8cee70. TapeCall a cada
/// jogador: sem contador.
pub(crate) unsafe extern "C" fn key_nation_club_in_season(club: usize, start: usize, end: usize) -> u32 {
    let club = Club(club);
    let mut row = G_LEAGUE_EUROPE_NATION;
    let mut c: i16 = 0x28;
    let nation = club.nation().addr();
    if nation != 0 {
        if let Some(k) = (0..0x23usize).find(|&k| read_usize(G_LEAGUE_TABLE + 72 * k) == nation) {
            row = G_LEAGUE_TABLE + 72 * k;
            c = read_i16(row + 0x39);
        }
    }
    let a = read_i16(row + 0x2a);
    let t = *(G_CURRENT_DATE as *const i16);
    let y = *(G_CURRENT_DATE_YEAR as *const u16);
    if a > c {
        if t > a {
            write_date(start, a, y);
            if end != 0 {
                write_date(end, c, y.wrapping_add(1)); // cpu2: the end in the following year
            }
            return 1;
        }
        if t < c {
            write_date(start, a, y.wrapping_sub(1));
            write_date(end, c, y);
            return 1; // cpu2: before the end, within the season
        }
        write_date(start, a, y);
        write_date(end, c, y);
        return 0; // cpu2: between the end and the start, outside
    }
    if t < a {
        write_date(start, a, y);
        write_date(end, c, y);
        return 0;
    }
    if t > c {
        write_date(start, a, y.wrapping_add(1));
        write_date(end, c, y.wrapping_add(1));
        return 1;
    }
    write_date(start, a, y);
    write_date(end, c, y);
    1 // cpu2: between the start and the end of the year
}

/// the continent of the human's nation, or that of the first main nation of the table
/// Original: `key_nation_get_main_continent` (0x6695c0, key_nation.cpp, cdecl).
///
/// [pt] { 0x6695c0 } (cdecl, key_nation.cpp): `key_nation_get_main_continent()`: o continente (+0x71) da nação do humano
/// ([0xae2d08]); sem ela ou sem continente, o da primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
pub(crate) unsafe extern "C" fn key_nation_get_main_continent() -> usize {
    let h = read_usize(G_MAIN_NATION);
    if h != 0 && read_usize(h + nation::CONTINENT) != 0 {
        return read_usize(h + nation::CONTINENT);
    }
    for k in 0..0x23usize {
        let n = read_usize(G_LEAGUE_TABLE + 72 * k);
        if n != 0 && *((n + nation::LEAGUE_SELECTED) as *const u8) & 2 != 0 {
            return read_usize(n + nation::CONTINENT);
        }
    }
    0
}

/// the human's nation, or the first main nation of the table
/// Original: `key_nation_get_main_nation` (0x669600, key_nation.cpp, cdecl).
///
/// [pt] { 0x669600 } (cdecl, key_nation.cpp): `key_nation_get_main_nation()`: a nação do humano ([0xae2d08]); sem ela, a
/// primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
pub(crate) unsafe extern "C" fn key_nation_get_main_nation() -> usize {
    let h = read_usize(G_MAIN_NATION);
    if h != 0 {
        return h;
    }
    for k in 0..0x23usize {
        let n = read_usize(G_LEAGUE_TABLE + 72 * k);
        if n != 0 && *((n + nation::LEAGUE_SELECTED) as *const u8) & 2 != 0 {
            return n;
        }
    }
    0
}

/// The World Cup competition ids: the finals and the six qualifying competitions (Europe [0x9cf780], Africa, Asia,
/// CONCACAF, Oceania, South America).
///
/// [pt] Os ids das competições da Copa do Mundo: a final e as seis eliminatórias (Europa [0x9cf780], África, Ásia,
/// CONCACAF, Oceania, América do Sul).
pub(crate) const WORLD_CUP_COMPS: [usize; 7] = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT];

/// The ids of the national-team tournaments outside the World Cup: Olympics, European Championship, European
/// Championship qualifiers, African Cup of Nations [0x9cf7ac], Copa América, B team, under-21, Oceania Nations Cup,
/// Confederations Cup and Asian Cup.
///
/// [pt] Os ids dos torneios de seleções fora da Copa do Mundo: Olimpíada, Eurocopa, eliminatórias da Eurocopa, Copa
/// Africana [0x9cf7ac], Copa América, seleção B, sub-21, Copa da Oceania, Copa das Confederações e Copa da Ásia.
pub(crate) const INTERNATIONAL_TOURNAMENTS: [usize; 10] = [KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_CONFEDERATIONS_CUP, KEY_CLUB_COMP_ASIAN_CUP];

/// 1 when the competition id is the World Cup or one of its six qualifying sections
/// Original: `key_nation_is_world_cup_comp` (0x669a10, key_nation.cpp, cdecl).
///
/// [pt] { 0x669a10 } (cdecl, key_nation.cpp): `key_nation_is_world_cup_comp(competição)`: o id (+0) é a Copa do Mundo ou uma
/// das eliminatórias, 1; senão 0. Competição nula: a caixa "Error" (linha 0x3d7) e 0.
pub(crate) unsafe extern "C" fn key_nation_is_world_cup_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3d7);
        return 0;
    }
    let id = read_usize(comp);
    WORLD_CUP_COMPS.iter().any(|&g| read_usize(g) == id) as u32
}

/// 1 when the competition id is one of the ten national side tournaments outside the World Cup
/// Original: `key_nation_is_international_comp` (0x669ae0, key_nation.cpp, cdecl).
///
/// [pt] { 0x669ae0 } (cdecl, key_nation.cpp): `key_nation_is_international_comp(competição)`: o id é um dos dez torneios de
/// seleções fora da Copa do Mundo, 1; senão 0. Competição nula: a caixa (linha 0x3ef) e 0.
pub(crate) unsafe extern "C" fn key_nation_is_international_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3ef);
        return 0;
    }
    let id = read_usize(comp);
    INTERNATIONAL_TOURNAMENTS.iter().any(|&g| read_usize(g) == id) as u32
}

/// Returns the competition's key nation row: the nation's row, 0 for the World Cup, 1 for national team tournaments, 2
/// for the club world cups, the continent's row or 3.
///
/// `key_nation_index_of_comp(competição)` (competition), called by the scope of the match rules `0x8d2fa0`, the league
/// day `0x6847c0`, the cup records `0x519890` and the match key nation `0x48dd70` of cpt_3. A nation competition
/// (`+0x5d`): its row in the table of the 35 key nations (`0xb63d60`, 72 bytes), or 4. With no nation: the World Cup
/// and its qualifiers, 0; national team tournament (`0x669ae0`), 1; the Club World Cup and the FIFA Club World
/// Championship, 2; otherwise the continent row (nation 0) with the competition's continent (`+0x59`), or 3.
///
/// The original reads `+0x5d` before checking for a null competition (the error case never reaches the box). TapeCall
/// for every match: no counter.
/// Original: `key_nation_index_of_comp` (0x669470, key_nation.cpp, cdecl).
///
/// [pt] { 0x669470 } (cdecl, key_nation.cpp): `key_nation_index_of_comp(competição)` (pelo alcance das regras da partida
/// 0x8d2fa0, pela liga do dia 0x6847c0, pelos registros da copa 0x519890 e pela nação-chave do jogo 0x48dd70 da cpt_3):
/// competição de nação (+0x5d): a linha dela na tabela das 35 nações-chave (0xb63d60, 72 bytes), ou 4. Sem nação: Copa
/// do Mundo e eliminatórias, 0; torneio de seleções (0x669ae0), 1; Copa do Mundo de Clubes e Mundial de Clubes da FIFA,
/// 2; senão a linha de continente (nação 0) com o continente da competição (+0x59), ou 3. A original lê +0x5d antes de
/// conferir a competição nula (o caso de erro nunca chega à caixa). TapeCall a cada partida: sem contador.
pub(crate) unsafe extern "C" fn key_nation_index_of_comp(comp: usize) -> u8 {
    let nation = read_usize(comp + competition::NATION);
    if nation != 0 {
        let row = (0..0x23usize).find(|&k| read_usize(G_LEAGUE_TABLE + 72 * k) == nation); // cpt4: the nation search
        return match row {
            Some(k) => k as u8, // cpt4: the nation row
            None => 4,
        };
    }
    let id = read_usize(comp);
    let cup = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT]; // cpt4: the World Cup and the qualifiers
    if cup.iter().any(|&g| read_usize(g) == id) {
        return 0;
    }
    if exe::key_nation_is_international_comp(comp) != 0 {
        return 1;
    }
    if id == read_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP) || id == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
        return 2;
    }
    for k in 0..0x23usize {
        let c = read_usize(comp + competition::CONTINENT);
        if read_usize(G_LEAGUE_TABLE + 72 * k) == 0 && c != 0 && c == read_usize(G_LEAGUE_WORLD_CUP_CONTINENT + 72 * k) {
            return k as u8; // cpt4: the continent row
        }
    }
    3
}

