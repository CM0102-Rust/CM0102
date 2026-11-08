use super::*;

/// The keys of the 32 nations of the real 2002 draw (the g_key_nation_* globals, index into the nation table
/// [0xae23a8], 0x122 bytes each), in the order in which the original routine writes them to the list: groups A to H,
/// four per group.
///
/// [pt] As chaves das 32 nações do sorteio real de 2002 (os globais g_key_nation_*, índice na tabela de nações [0xae23a8],
/// 0x122 bytes cada), na ordem em que a original as grava na lista: grupos A a H, quatro por grupo.
pub(crate) const WORLD_CUP_2002_DRAW: [usize; 32] = [
    KEY_NATION_FRANCE, KEY_NATION_SENEGAL, KEY_NATION_URUGUAY, KEY_NATION_DENMARK, // A: France, Senegal, Uruguay, Denmark
    KEY_NATION_SPAIN, KEY_NATION_SLOVENIA, KEY_NATION_PARAGUAY, KEY_NATION_SOUTH_AFRICA, // B: Spain, Slovenia, Paraguay, South Africa
    KEY_NATION_BRAZIL, KEY_NATION_TURKEY, KEY_NATION_CHINA, KEY_NATION_COSTA_RICA, // C: Brasil, Turquia, China, Costa Rica
    KEY_NATION_SOUTH_KOREA, KEY_NATION_POLAND, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, // D: South Korea, Poland, United States, Portugal
    KEY_NATION_GERMANY, KEY_NATION_SAUDI_ARABIA, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_CAMEROON, // E: Germany, Saudi Arabia, Ireland, Cameroon
    KEY_NATION_ARGENTINA, KEY_NATION_NIGERIA, KEY_NATION_ENGLAND, KEY_NATION_SWEDEN, // F: Argentina, Nigeria, England, Sweden
    KEY_NATION_ITALY, KEY_NATION_ECUADOR, KEY_NATION_CROATIA, KEY_NATION_MEXICO, // G: Italy, Ecuador, Croatia, Mexico
    KEY_NATION_JAPAN, KEY_NATION_BELGIUM, KEY_NATION_RUSSIA, KEY_NATION_TUNISIA, // H: Japan, Belgium, Russia, Tunisia
];

/// Builds the World Cup draw list: the seeds into groups 1..8, the other 24 into drawn groups with a limit; in 2002,
/// the real draw and the nation table with the holder and the hosts.
///
/// `WorldCup::draw_groups()`, from the draw made on the day (0x92e7f0); no SEH. The draw list: 32 entries of 5 bytes
/// (+0 the id of the national team's club, +4 the group 1..8), zeroed (0x945501(0x20, 5): push 5; push 0x20; add esp, 8
/// -> cdecl; out of memory, the box for line 0x4b5 and it returns 0).
///
/// 1) The seeds (8 slots of 5 bytes at +0xee: +0 the id, +4 the pot; 0xff = already drawn): slot 0 goes to group 1 and
/// slot 1 (+0xf3) to group 3; groups 2 and 4..8 draw one of slots 2..7 (0x90cfc0(6) + 2), drawing again while the drawn
/// slot has already gone. THIS IS THE LOOP OF THE 27/12/2001 HANG (the game stops without an error; worked around by
/// Alan_v2 by changing the jump at 0x92ea55): if fewer than six of slots 2..7 are still standing (in a career started
/// before 2001, seed_teams 0x92f230 does not seed the 2002 World Cup and the slots stay 0xff), the draw never finds a
/// slot and the loop never ends. The port repeats the loop like the original, with no limit.
///
/// 2) The other 24 (the slots at +0x116, in order) fall into drawn groups (0x90cfc0(8)), each group with at most (k /
/// 8) + 1 teams when the kth goes in (k from 8 to 31): it draws again while the group already has the limit.
///
/// 3) In the 2002 World Cup (+0x40 = 2002), after the draw, the list becomes the real draw (the order of
/// `WORLD_CUP_2002_DRAW`: the id of the main national team's club, 0x53b3d0 push nation; add esp -> cdecl), groups i/4
/// + 1, and the table of the 32 World Cup nations ([+0x14], 6 bytes: +0 the club, +4 the status) receives the clubs
/// with status 6, France (holder) 2, Korea and Japan (hosts) 1; +0x36 (how many) = 32.
///
/// Returns the list.
/// Original: `WorldCup::draw_groups` (0x92e940, world_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x92e940 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::draw_groups()`, do sorteio feito no dia
/// (0x92e7f0). A lista do sorteio: 32 entradas de 5 bytes (+0 o id do clube da seleção, +4 o grupo 1..8), zerada
/// (0x945501(0x20, 5): push 5; push 0x20; add esp, 8 -> cdecl; sem memória, a caixa da linha 0x4b5 e devolve 0).
/// 1) Os cabeças de chave (8 vagas de 5 bytes em +0xee: +0 o id, +4 o pote; 0xff = já tirada): a vaga 0 vai para o
///    grupo 1 e a vaga 1 (+0xf3) para o grupo 3; os grupos 2 e 4..8 tiram uma das vagas 2..7 (0x90cfc0(6) + 2),
///    sorteando de novo enquanto a vaga sorteada já saiu. ESTE É O LAÇO DA QUEDA DE 27/12/2001 (o jogo para sem erro,
///    contornado pelo Alan_v2 trocando o desvio em 0x92ea55): se menos de seis das vagas 2..7 estão de pé (numa carreira
///    começada antes de 2001, a seed_teams 0x92f230 não semeia a Copa de 2002 e as vagas ficam 0xff), o sorteio nunca
///    acha vaga e o laço não termina. A cópia repete o laço como a original, sem limite.
/// 2) Os outros 24 (as vagas de +0x116, na ordem) caem em grupos sorteados (0x90cfc0(8)), cada grupo com no máximo
///    (k / 8) + 1 times quando o k-ésimo entra (k de 8 a 31): sorteia de novo enquanto o grupo já tem o limite.
/// 3) Na Copa de 2002 (+0x40 = 2002), depois do sorteio, a lista vira o sorteio real (a ordem de `WORLD_CUP_2002_DRAW`:
///    o id do clube da seleção principal, 0x53b3d0 push nação; add esp -> cdecl), os grupos i/4 + 1, e a tabela das 32
///    nações da Copa ([+0x14], 6 bytes: +0 o clube, +4 a situação) recebe os clubes com a situação 6, a França (detentora)
///    2, a Coreia e o Japão (sedes) 1; +0x36 (quantas) = 32.
/// Devolve a lista.
pub(crate) unsafe extern "thiscall" fn draw_groups(this: usize) -> usize {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let mut count = [0i8; 8];
    let list = zerada(0x20, 5);
    if list == 0 {
        show_error_at(0xad5888, 0x4b5);
        return 0;
    }
    let mut s = 0i32;
    while s < 8 {
        count[s as usize] = count[s as usize].wrapping_add(1);
        if s == 0 {
            write_u32_from_usize(list, read_u32_as_usize(this + 0xee));
            write_u8(list + 4, 1);
            write_u8(this + 0xf2, 0xff);
        } else if s == 2 {
            write_u32_from_usize(list + 0xa, read_u32_as_usize(this + 0xf3));
            write_u8(list + 0xe, 3);
            write_u8(this + 0xf7, 0xff);
        } else {
            // the loop of the 27/12/2001 crash (0x92ea55): with no standing slot among 2..7, it never ends
            let mut k = sorteio(6).wrapping_add(2) as i16 as i32;
            while read_u8(this.wrapping_add((k * 5) as usize) + 0xf2) == 0xff {
                k = sorteio(6).wrapping_add(2) as i16 as i32;
            }
            let c = this.wrapping_add((k * 5) as usize);
            let e = list + s as usize * 5;
            write_u32_from_usize(e, read_u32_as_usize(c + 0xee));
            write_u8(e + 4, (s as u8).wrapping_add(1));
            write_u8(c + 0xf2, 0xff);
        }
        s += 1;
    }
    let mut src = this + 0x116;
    let mut dst = list + 0x28;
    let mut k = 8i32;
    for _ in 0..0x18 {
        let lim = ((k / 8) as i8).wrapping_add(1); // cqf4: the limit of teams per group
        let mut r = sorteio(8);
        while count[r as i16 as usize] >= lim {
            r = sorteio(8);
        }
        crate::comparison::branch("cqf_4 vaga no grupo");
        write_u32_from_usize(dst, read_u32_as_usize(src));
        src += 5;
        write_u8(dst + 4, (r as u8).wrapping_add(1));
        dst += 5;
        let g = r as i16 as usize;
        count[g] = count[g].wrapping_add(1);
        k += 1;
    }
    if read_i16(this + league::YEAR) != 0x7d2 {
        return list;
    }
    crate::comparison::branch("cqf_4 sorteio de 2002");
    for (i, &chave) in WORLD_CUP_2002_DRAW.iter().enumerate() { // cqf4: the nations of the real random draw
        let nation = read_u32_as_usize(G_NATION_TABLE).wrapping_add((read_u32_as_usize(chave) as i32).wrapping_mul(0x122) as usize);
        write_u32_from_usize(list + i * 5, read_u32_as_usize(main(nation)));
    }
    for i in 0..32usize {
        let e = list + i * 5;
        let club = read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(e).wrapping_mul(club::SIZE));
        write_u32_from_usize(read_u32_as_usize(this + 0x14) + i * 6, club);
        write_u8(read_u32_as_usize(this + 0x14) + i * 6 + 4, 6);
        write_u8(e + 4, (i / 4) as u8 + 1);
    }
    write_u8(read_u32_as_usize(this + 0x14) + 4, 2);
    write_u8(read_u32_as_usize(this + 0x14) + 0x4c, 1);
    write_u8(read_u32_as_usize(this + 0x14) + 0xac, 1);
    write_u16(this + 0x36, 0x20);
    list
}

/// The World Cup draw day: the draw list, group 1 in the World Cup itself with its fixtures and sorted table, the group
/// 2..8 stages, the predicted positions and the bookings reset.
///
/// `WorldCup::run_group_draw()`, from the draw day (0x92f1a0); no SEH. The draw list (0x92e940, mov ecx, this ->
/// thiscall); group 1 in the World Cup's own table (0x92d0f0 push list; mov ecx -> thiscall, ret 4; 0: the box for line
/// 0x46d and end); its fixtures (0x6835c0 mov ecx -> thiscall, ret 0; 0: line 0x473); the sorted table (0x6827d0 push
/// 0; mov ecx -> thiscall, ret 4; 0: line 0x47a). With everything done: +0x45 = 1, the stages of groups 2..8 (0x92d540
/// push list; push g; mov ecx -> thiscall, ret 8, g from 0 to 6), the list freed (0x9452ca push; add esp -> cdecl), the
/// predicted positions (method +0x5c, 0x92f680) and the bookings reset in the competition's body (0x7e9180 push 0; ecx
/// = [[0xae28f0] + 4 × (char)+0x50] -> thiscall, ret 4). On an error the list is not freed (as in the original).
/// Original: `WorldCup::run_group_draw` (0x92e7f0, world_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x92e7f0 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::run_group_draw()`, do dia do sorteio (0x92f1a0).
/// A lista do sorteio (0x92e940, mov ecx, this -> thiscall); o grupo 1 na tabela da própria Copa (0x92d0f0 push lista;
/// mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x46d e fim); os jogos dele (0x6835c0 mov ecx -> thiscall, ret 0;
/// 0: a linha 0x473); a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a linha 0x47a). Com tudo
/// feito: +0x45 = 1, as fases dos grupos 2..8 (0x92d540 push lista; push g; mov ecx -> thiscall, ret 8, g de 0 a 6),
/// a lista liberada (0x9452ca push; add esp -> cdecl), as posições previstas (método +0x5c, 0x92f680) e os cartões
/// zerados no órgão da competição (0x7e9180 push 0; ecx = [[0xae28f0] + 4 × (char)+0x50] -> thiscall, ret 4).
/// Num erro a lista não é liberada (como a original).
pub(crate) unsafe extern "thiscall" fn run_group_draw(this: usize) {
    let sortear: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_WORLDCUP_DRAW_GROUPS);
    let group_one: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_WORLDCUP_CREATE_GROUP_TABLE);
    let create: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let stage: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(FN_WORLDCUP_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let cards: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_RULINGBODY_RESET_BOOKINGS_FOR_CLUB);
    let list = sortear(this);
    if group_one(this, list) == 0 {
        show_error_at(0xad5888, 0x46d);
        return;
    }
    if create(this) == 0 {
        show_error_at(0xad5888, 0x473);
        return;
    }
    if sort(this, 0) == 0 {
        show_error_at(0xad5888, 0x47a);
        return;
    }
    write_u32_from_usize(this + league::ACTIVE, 1);
    for g in 0..7 {
        stage(this, g, list);
    }
    soltar(list);
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
    predicted(this);
    let body = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(G_DISCIPLINE)).wrapping_add((read_u8(this + league::DISCIPLINE_RULE) as i8 as i32 * 4) as usize));
    cards(body, 0);
}

/// The World Cup day: the seeding on the seeding date, the draw on its date (no table yet) and the league day.
///
/// `WorldCup::update_for_date(today, full)`, method +0x14 of the World Cup, the World Cup's day from the competitions'
/// day. On the seeding date (+0x18e the day of the year, +0x190 the year relative to +0x40: 15/12 of the previous year,
/// written by 0x92d200) and with `full`, it seeds (0x92f230 mov ecx -> thiscall, ret 0); on the draw date (+0xb6,
/// +0xb8: 27/12 of the previous year), with `full` and no table yet (+0xb1 = 0), the draw (0x92e7f0 mov ecx ->
/// thiscall, ret 0). Then the league day (0x6847c0 push full; push today; mov ecx -> thiscall, ret 8), whose return
/// value (eax) it passes on. The date: +0 the day of the year, +2 the year (unsigned).
/// Original: `WorldCup::update_for_date` (0x92f1a0, world_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x92f1a0 } (thiscall, ret 8, world_cup.cpp, método +0x14 da Copa do Mundo): `WorldCup::update_for_date(hoje,
/// completo)`, o dia da Copa pelo dia das competições. Na data das cabeças de chave (+0x18e o dia do ano, +0x190 o ano
/// relativo a +0x40: 15/12 do ano anterior, gravados pela 0x92d200) e com `completo`, semeia (0x92f230 mov ecx ->
/// thiscall, ret 0); na data do sorteio (+0xb6, +0xb8: 27/12 do ano anterior), com `completo` e sem tabela ainda (+0xb1
/// = 0), o sorteio (0x92e7f0 mov ecx -> thiscall, ret 0). Depois o dia de liga (0x6847c0 push completo; push hoje; mov
/// ecx -> thiscall, ret 8), cuja devolução (eax) ela repassa. A data: +0 o dia do ano, +2 o ano (sem sinal).
pub(crate) unsafe extern "thiscall" fn world_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let semear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_WORLDCUP_SEED_TEAMS);
    let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_WORLDCUP_RUN_GROUP_DRAW);
    let league: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let year = core::ptr::read_unaligned((today + 2) as *const u16) as i32;
    let day = core::ptr::read_unaligned(today as *const u16);
    if read_i16(this + 0x18e) as u16 == day && read_i16(this + 0x190) as i32 + read_i16(this + league::YEAR) as i32 == year && full != 0 {
        semear(this);
    }
    let day_of_draw = read_i16(this + 0xb6) as u16; // cqf4: the random draw day
    if day_of_draw == day && read_i16(this + 0xb8) as i32 + read_i16(this + league::YEAR) as i32 == year && full != 0 && read_u32_as_usize(this + league::TABLE) == 0 {
        crate::comparison::branch("cqf_4 dia do sorteio");
        sortear(this);
    }
    league(this, today, full)
}

/// Club of entry i (5 bytes, id at +0) of the draw list: [G_CLUB_TABLE] + id × 0x245.
///
/// [pt] O clube da entrada `i` da lista do sorteio (5 bytes: +0 o id): 0xae23b4 + id × 0x245.
pub(crate) unsafe fn draw_list_club(list: usize, i: usize) -> usize {
    read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(list + i * 5).wrapping_mul(club::SIZE))
}

/// The World Cup itself becomes group 1: four teams, the allocated table, the group 1 teams of the draw list in the
/// rows and the group draw news.
///
/// `WorldCup::create_group_table(list)`, from the draw (0x92e7f0); no SEH. The World Cup itself becomes group 1: +0x3e
/// (teams) = 4, the table [+0xb1] allocated with 0xec bytes (0x944e46 push; add esp -> cdecl; out of memory, the box
/// for line 0x2b1 and it returns 0); the list entries of group 1 (+4 = 1), in order, go into rows 0..3 (0x687430 push
/// 0; push 0; push club; push row; mov ecx, this -> thiscall, ret 0x10; the row goes in cl and the original leaves
/// garbage in the high bytes of ecx, which 0x687430 does not read: the port passes the clean byte), up to 4 rows or the
/// end of the 32 entries; then the group news (0x684230 mov ecx, this -> thiscall). Returns 1.
/// Original: `WorldCup::create_group_table` (0x92d0f0, world_cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x92d0f0 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::create_group_table(lista)`, do sorteio
/// (0x92e7f0). A própria Copa vira o grupo 1: +0x3e (times) = 4, a tabela [+0xb1] alocada com 0xec bytes (0x944e46 push;
/// add esp -> cdecl; sem memória, a caixa da linha 0x2b1 e devolve 0); as entradas da lista do grupo 1 (+4 = 1), na
/// ordem, entram nas linhas 0..3 (0x687430 push 0; push 0; push clube; push linha; mov ecx, this -> thiscall, ret 0x10;
/// a linha vai no cl e a original deixa lixo nos bytes altos do ecx, que a 0x687430 não lê: a cópia passa o byte
/// limpo), até 4 linhas ou o fim das 32 entradas; depois a notícia dos grupos (0x684230 mov ecx, this -> thiscall).
/// Devolve 1.
pub(crate) unsafe extern "thiscall" fn world_cup_create_group_table(this: usize, list: usize) -> u32 {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let row_of_table: unsafe extern "thiscall" fn(usize, u32, usize, u32, usize) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    write_u16(this + league::CLUB_COUNT, 4);
    let t = alloc(0xec);
    write_u32_from_usize(this + league::TABLE, t);
    if t == 0 {
        show_error_at(0xad5888, 0x2b1);
        return 0;
    }
    let mut row = 0i8;
    let mut i = 0i8;
    while row < 4 {
        let e = i as usize; // cqf5: the list entry
        if read_u8(list + e * 5 + 4) == 1 {
            crate::comparison::branch("cqf_5 time do grupo um");
            let club = draw_list_club(list, e);
            row_of_table(this, row as u8 as u32, club, 0, 0);
            row += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    news(this);
    1
}

/// Clubs of group g (+4 = g) in the draw list, in order, up to 4.
///
/// [pt] Os clubes do grupo `g` (+4 = g) da lista, na ordem, até 4 (o que a 0x92d0f0 e a 0x92d540 põem nas linhas).
pub(crate) unsafe fn group_clubs(list: usize, g: i32) -> Vec<usize> {
    (0..32usize).filter(|&i| read_u8(list + i * 5 + 4) as i8 as i32 == g).map(|i| draw_list_club(list, i)).take(4).collect()
}

/// Creates the stage of one of groups 2 to 8: its teams, the round schedule, the stage constructor, the current stage
/// and the group news.
///
/// `WorldCup::create_group_stage(g, list)`, from the draw (0x92e7f0, g from 0 to 6: groups 2 to 8); SEH frame only for
/// the new: no destructor on the normal path. The teams of group g+2 (up to 4, in list order) in a vector of +0x3e
/// pointers (0x944e46; out of memory, the box for line 0x366 and 0); the round table through the World Cup's method
/// +0x3c (push 0; push &id; push &rounds; push g; mov ecx -> thiscall, ret 0x10: 0x92c290 writes both outputs for
/// stages 0..6); the stage (0x944cf1(0xee) push; add esp -> cdecl; the League_Stage constructor 0x68b100, thiscall, ret
/// 0x58: 22 arguments, checked against its reading: the World Cup, the teams (word), the vector, 1, +4, the round
/// table, the rounds (word: +0xa9 of the stage, the date count), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, the year +0x40
/// (word), g (byte), the id (word: +0x3a of the stage), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; the original pushes the
/// words and bytes with garbage in the high bytes, which the constructor does not read: the port passes the clean
/// values) stored in [+0xc][g]; the vector and the round table freed (0x9452ca). Without a stage: the box for line
/// 0x380 and 0. With it: +0x30 = g and the group news (0x684230 on the stage). Returns 1.
/// Original: `WorldCup::create_group_stage` (0x92d540, world_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x92d540 } (thiscall, ret 8, world_cup.cpp, quadro SEH só para o new: sem destrutor no caminho normal):
/// `WorldCup::create_group_stage(g, lista)`, do sorteio (0x92e7f0, g de 0 a 6: os grupos 2 a 8). Os times do grupo g+2
/// (até 4, na ordem da lista) num vetor de +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x366 e 0); a tabela
/// das rodadas pelo método +0x3c da Copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall, ret 0x10: a
/// 0x92c290 grava as duas saídas para as fases 0..6); a fase (0x944cf1(0xee) push; add esp -> cdecl; o construtor
/// League_Stage 0x68b100, thiscall, ret 0x58: 22 argumentos, conferidos na leitura dele: a Copa, os times (word),
/// o vetor, 1, +4, a tabela das rodadas, as rodadas (word: +0xa9 da fase, a conta das datas), +0xc2, +0xc3, +0xc4,
/// &+0xc5, &+0xbe, o ano +0x40 (word), g (byte), o id (word: +0x3a da fase), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x380 e 0. Com ela: +0x30 = g e a notícia do grupo
/// (0x684230 na fase). Devolve 1.
pub(crate) unsafe extern "thiscall" fn world_cup_create_group_stage(this: usize, g: i32, list: usize) -> u32 {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = alloc(((read_i16(this + league::CLUB_COUNT) as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0xad5888, 0x366);
        return 0;
    }
    let mut n = 0i16;
    let mut i = 0i16;
    while n < 4 {
        let e = list + i as usize * 5;
        if read_u8(e + 4) as i8 as i32 == g + 2 {
            crate::comparison::branch("cqf_5 time do grupo");
            write_u32_from_usize(teams + n as usize * 4, draw_list_club(list, i as usize)); // cqf5: the team in the group slot
            n += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this, g, &mut rodadas, &mut id, 0);
    let p = new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this,
            read_i16(this + league::CLUB_COUNT) as u16 as u32,
            teams,
            1,
            read_u32_as_usize(this + 4),
            table,
            rodadas as u16 as u32, // cqf5: the rounds (the dates, +0xa9 of the stage)
            read_u8(this + league::WIN_POINTS) as u32,
            read_u8(this + league::DRAW_POINTS) as u32,
            read_u8(this + league::MATCH_CATEGORY) as u32,
            this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT,
            read_i16(this + league::YEAR) as u16 as u32,
            g as u8 as u32,
            id as u16 as u32,
            read_u8(this + league::IMPORTANCE) as u32,
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
    if read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4) == 0 {
        show_error_at(0xad5888, 0x380);
        return 0;
    }
    write_u32_from_usize(this + league::CURRENT_STAGE, g as usize);
    news(read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4));
    1
}

/// Sorts the 32 nations by reputation, sets each one's predicted position (1, 2, 3, 4, 5, 9, 17, 25 by the stage bands)
/// and makes the World Cup the nation's current competition.
///
/// `WorldCup::set_predicted_positions()`, method +0x5c of the World Cup, from the draw (0x92e7f0). Without statistics
/// (+8): the box for line 0x671 and it returns. Fewer than 32 nations (+0x36): the box for line 0x678 and +0x36 = 32.
/// The clubs of the nation table ([+0x14], 6 bytes) in a vector (0x944e46; out of memory, the box for line 0x680 and it
/// returns), sorted by reputation (qsort 0x945333 with 0x4b07d0: push; add esp, 0x10 -> cdecl), and each one's
/// predicted position in the statistics (0x4a2540 push position; push club; mov ecx, [+8] -> thiscall, ret 8): 1, 2, 3,
/// 4, four 5s, eight 9s, eight 17s and eight 25s (the bands of the final stages); then the current competition of each
/// club's nation (+0x53) becomes the World Cup (0x775220 push [+4]; push nation; mov ecx, [0xb74318] -> thiscall, ret
/// 8) and the vector is freed (0x9452ca).
/// Original: `WorldCup::set_predicted_positions` (0x92f680, world_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x92f680 } (thiscall, ret 0, world_cup.cpp, método +0x5c da Copa do Mundo): `WorldCup::set_predicted_positions()`,
/// do sorteio (0x92e7f0). Sem estatística (+8): a caixa da linha 0x671 e volta. Menos de 32 nações (+0x36): a caixa da
/// linha 0x678 e +0x36 = 32. Os clubes da tabela das nações ([+0x14], 6 bytes) num vetor (0x944e46; sem memória, a
/// caixa da linha 0x680 e volta), ordenado pela reputação (qsort 0x945333 com 0x4b07d0: push; add esp, 0x10 -> cdecl),
/// e a posição prevista de cada um na estatística (0x4a2540 push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 4, quatro 5, oito 9, oito 17 e oito 25 (as faixas das fases finais); depois a competição atual
/// da nação de cada clube (+0x53) vira a Copa (0x775220 push [+4]; push nação; mov ecx, [0xb74318] -> thiscall, ret 8)
/// e o vetor é liberado (0x9452ca).
pub(crate) unsafe extern "thiscall" fn world_cup_set_predicted_positions(this: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let position: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let competition: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_SET_CURRENT_COMPETITION);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u32_as_usize(this + 8) == 0 {
        show_error_at(0xad5888, 0x671);
        return;
    }
    if read_i16(this + 0x36) < 0x20 {
        show_error_at(0xad5888, 0x678);
        write_u16(this + 0x36, 0x20);
    }
    let v = alloc(((read_i16(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0xad5888, 0x680);
        return;
    }
    let mut i = 0i16;
    while i < read_i16(this + 0x36) {
        write_u32_from_usize(v + i as usize * 4, read_u32_as_usize(read_u32_as_usize(this + 0x14) + i as usize * 6));
        i += 1;
    }
    sort(v, read_i16(this + 0x36) as i32 as u32, 4, 0x4b07d0);
    const FAIXAS: [u32; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    for (k, &p) in FAIXAS.iter().enumerate() {
        position(read_u32_as_usize(this + 8), read_u32_as_usize(v + k * 4), p);
    }
    let mut i = 0i16;
    while i < read_i16(this + 0x36) {
        crate::comparison::branch("cqf_5 competição da nação");
        competition(read_u32_as_usize(G_NATIONAL_TEAMS), read_u32_as_usize(read_u32_as_usize(v + i as usize * 4) + 0x53), read_u32_as_usize(this + 4));
        i += 1; // cqf5: the next nation
    }
    soltar(v);
}

/// The matches of each group stage in the round table (0x68a1c0: round, match, a, b, c, value), in the original's
/// order, by stage + 1 (jump table 0x92ccd0: −1 is the Cup itself, group 1; 0..6 groups 2..8).
///
/// [pt] Os jogos de cada fase de grupos na tabela das rodadas (0x68a1c0: rodada, jogo, a, b, c, valor), na ordem da
/// original, pela fase + 1 (a tabela de saltos 0x92ccd0: −1 é a própria Copa, o grupo 1; 0..6 os grupos 2..8).
pub(crate) const WORLD_CUP_GROUP_GAMES: [[(i32, i32, i32, i32, i32, u32); 6]; 8] = [
    [(0, 0, 2, 1, 3, 4), (0, 1, 2, 2, 3, 0x4000), (1, 0, 1, 1, 3, 0x1000), (1, 1, 1, 2, 3, 0x400), (2, 0, 1, 2, 3, 0x40), (2, 1, 1, 2, 3, 0x2000)],
    [(0, 0, 3, 1, 3, 0x1000), (0, 1, 3, 2, 3, 0x800), (1, 0, 2, 1, 3, 0x2000), (1, 1, 2, 2, 3, 0x4000), (2, 0, 1, 1, 3, 4), (2, 1, 1, 1, 3, 0x400)],
    [(0, 0, 4, 1, 3, 0x200), (0, 1, 4, 2, 3, 0x40), (1, 0, 3, 2, 3, 0x800), (1, 1, 3, 1, 3, 4), (2, 0, 2, 1, 3, 0x100), (2, 1, 2, 1, 3, 0x1000)],
    [(0, 0, 4, 1, 3, 0x4000), (0, 1, 5, 1, 3, 0x400), (1, 0, 5, 1, 3, 0x2000), (1, 1, 5, 2, 3, 0x80), (2, 0, 2, 2, 3, 0x200), (2, 1, 2, 2, 3, 0x800)],
    [(0, 0, 5, 1, 3, 0x100), (0, 1, 5, 2, 3, 4), (1, 0, 5, 1, 3, 0x1000), (1, 1, 5, 2, 3, 0x40), (2, 0, 3, 1, 3, 0x2000), (2, 1, 3, 1, 3, 0x80)],
    [(0, 0, 6, 1, 3, 0x2000), (0, 1, 0, 2, 3, 0x80), (1, 0, 6, 1, 3, 0x200), (1, 1, 6, 2, 3, 0x100), (2, 0, 3, 2, 3, 0x4000), (2, 1, 3, 2, 3, 0x400)],
    [(0, 0, 0, 1, 3, 0x40), (0, 1, 0, 2, 3, 0x100), (1, 0, 0, 1, 3, 0x4000), (1, 1, 0, 2, 3, 0x800), (2, 0, 4, 2, 3, 4), (2, 1, 4, 2, 3, 0x200)],
    [(0, 0, 6, 1, 3, 0x800), (0, 1, 6, 2, 3, 0x200), (1, 0, 5, 1, 3, 0x400), (1, 1, 6, 1, 3, 0x80), (2, 0, 4, 1, 3, 0x1000), (2, 1, 4, 1, 3, 0x100)],
];

/// Creates the three-round schedule of a World Cup group stage (12, 19 and 24 June, moved to the weekday) with the two
/// fixtures of each round; the knockout stays with the original.
///
/// `WorldCup::create_schedule(stage, &rounds, &id, &is_league)`, method +0x3c of the World Cup, from the creation of
/// the rules (0x92d200, stage −1) and from the group stages (0x92d540, 0..6). In the group stages (char < 7): is_league
/// (if requested) = 1, rounds = 3, id = 0x420 + stage; the table of 3 rounds of 0x41 bytes (0x944e46(0xc3); out of
/// memory, the box for line 0xcc and it returns 0), the dates (0x68a160: 12, 19 and 24 June of the World Cup year,
/// moved to weekdays 4, 4 and 2, code 1, value 1), game 2 of each round empty (−1) and the two fixtures of each round
/// of the stage (`WORLD_CUP_GROUP_GAMES`). The knockout (stage 7, with the cup routines) and the stages above: the
/// original. Returns the table.
/// Original: `WorldCup::create_schedule` (0x92c290, world_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x92c290 } (thiscall, ret 0x10, world_cup.cpp, método +0x3c da Copa do Mundo): `WorldCup::create_schedule(fase,
/// &rodadas, &id, &é_liga)`, da criação das regras (0x92d200, fase −1) e das fases dos grupos (0x92d540, 0..6). Nas
/// fases de grupos (char < 7): é_liga (se pedido) = 1, rodadas = 3, id = 0x420 + fase; a tabela de 3 rodadas de 0x41
/// bytes (0x944e46(0xc3); sem memória, a caixa da linha 0xcc e devolve 0), as datas (0x68a160: 12, 19 e 24 de junho
/// do ano da Copa, levadas aos dias da semana 4, 4 e 2, código 1, valor 1), o jogo 2 de cada rodada vazio (−1) e os
/// dois jogos de cada rodada da fase (`WORLD_CUP_GROUP_GAMES`). O mata-mata (fase 7, com as rotinas da copa) e as fases acima:
/// a original. Devolve a tabela.
pub(crate) unsafe extern "thiscall" fn world_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, id: *mut i16, league: *mut i32) -> usize {
    let f = stage as u8 as i8;
    if f >= 7 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WORLD_CUP_CREATE_SCHEDULE)(this, stage, rounds, id, league);
    }
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let game: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 3; // cqf6: três rodadas
    *id = (f as i16).wrapping_add(0x420);
    let t = alloc(0xc3);
    if t == 0 {
        show_error_at(0xad5888, 0xcc);
        return 0;
    }
    crate::comparison::branch("cqf_6 tabela das rodadas");
    // the year carries garbage in the high bytes in the original (the year’s cx over the previous ecx); 0x68a160 only uses the 16 bits
    // low (0x549f70 reads the year as a word and the word at +2 holds year − base): the copy passes the clean year
    let year = read_i16(this + league::YEAR) as u16 as u32;
    data(t, 0, 0xc, 5, 0, 4, 1, year, 1); // cqf6: a primeira rodada
    game(t, 0, 2, -1, -1, -1, 0);
    data(t, 1, 0x13, 5, 0, 4, 1, year, 1); // cqf6: a segunda rodada
    game(t, 1, 2, -1, -1, -1, 0);
    data(t, 2, 0x18, 5, 0, 2, 1, year, 1); // cqf6: a terceira rodada
    game(t, 2, 2, -1, -1, -1, 0);
    let k = (f as i32 + 1) as u32;
    if k <= 7 {
        for &(r, j, a, b, c, v) in WORLD_CUP_GROUP_GAMES[k as usize].iter() {
            game(t, r, j, a, b, c, v);
        }
    }
    t
}

/// Collects the written bytes of a 3-round table: per round +0..+4 and +0x3d..+0x40, and each of the 3 matches
/// +5..+0xb.
///
/// [pt] Os bytes que a tabela das rodadas tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4 e
/// +0x3d..+0x40; de cada um dos 3 jogos, +5..+0xb.
pub(crate) unsafe fn written_group_round_bytes(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..3usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..3usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + j * 7 + 5) as *const u8, 7));
        }
    }
    v
}

/// Adds a qualified nation to the World Cup nation table (status 6), or shows the box when the list is full.
///
/// `WorldCup::add_qualified_team(club)`, from the qualifiers (0x90f4f0, 0x911e10, 0x91538e, 0x9184c0, 0x91e200,
/// 0x920500); no SEH. With the nation list (+0x36) below the maximum (+0x38), slot +0x36 of table [+0x14] (6 bytes)
/// receives status 6 and the club, and +0x36 goes up; when full, the box for line 0x456.
/// Original: `WorldCup::add_qualified_team` (0x92e740, world_cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x92e740 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::add_qualified_team(clube)`, das eliminatórias
/// (0x90f4f0, 0x911e10, 0x91538e, 0x9184c0, 0x91e200, 0x920500): com a lista das nações (+0x36) abaixo do máximo
/// (+0x38), a vaga +0x36 da tabela [+0x14] (6 bytes) recebe a situação 6 e o clube, e +0x36 sobe; cheia, a caixa da
/// linha 0x456.
pub(crate) unsafe extern "thiscall" fn add_qualified_team(this: usize, club: usize) {
    let n = read_i16(this + 0x36);
    if n >= read_i16(this + 0x38) {
        show_error_at(0xad5888, 0x456);
        return;
    }
    crate::comparison::branch("cqf_6 nação classificada");
    let e = read_u32_as_usize(this + 0x14).wrapping_add((n as i32 * 6) as usize);
    write_u8(e + 4, 6);
    write_u32_from_usize(read_u32_as_usize(this + 0x14).wrapping_add((read_i16(this + 0x36) as i32 * 6) as usize), club);
    write_u16(this + 0x36, (read_i16(this + 0x36) as u16).wrapping_add(1));
}

/// Sets the World Cup seeds (holder, hosts, reputation) and the pots by continent; does nothing for the 2002 World Cup.
///
/// `WorldCup::seed_teams()`, from the World Cup day (0x92f1a0, on the seeding date); no SEH. In the 2002 World Cup
/// (+0x40 = 2002) it returns without doing anything: its draw slots come from the database, which is why a 2002 World
/// Cup created by another path hangs the draw (the 0x92e940 loop, cqf_4). In the others: +0x4b = 0; with fewer than 32
/// nations (+0x36), the box for line 0x59e and it completes the list with the main national team (0x53b3d0) of each
/// nation ([0xae2350] nations of 0x122 bytes) not yet in it, up to 32; it sorts table [+0x14] (6 bytes) by reputation
/// (qsort 0x945333 with 0x4b07b0); the hosts (0x5fa730 on [0xb5e134]: thiscall, ret 0x14, with the World Cup id
/// [0x9cf79c], the year as a word, the two outputs and 1; output −1 = no host) and the holder (the most recent
/// champion, 0x4afd80 of the competition [0xae23d0] + id × 0x6b) go to slots 1, 2 and 0 (swapping places), and the
/// others are sorted again from the first free one; the first 8 become seeds (pot 0, status 3) and the other 24 go to
/// their continent's pot (+0x53 nation, +0x71 continent: [0x9cfa10] pot 1, [0x9cfa18]/[0x9cfa0c]/[0x9cfa1c] pot 2,
/// [0x9cfa08]/[0x9cfa14] pot 3; without a nation or continent, the box for line 0x61e) with the club id in the slots at
/// +0xee (5 bytes); finally the 24 of the table (from the 9th on) and the slots at +0x116 are sorted by pot (0x92f660).
/// Original: `WorldCup::seed_teams` (0x92f230, world_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x92f230 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::seed_teams()`, do dia da Copa (0x92f1a0, na
/// data das cabeças de chave). Na Copa de 2002 (+0x40 = 2002) volta sem fazer nada: as vagas do sorteio dela vêm do
/// banco de dados, e é por isso que uma Copa de 2002 criada por outro caminho trava o sorteio (o laço da 0x92e940, cqf_4).
/// Nas outras: +0x4b = 0; com menos de 32 nações (+0x36), a caixa da linha 0x59e e completa a lista com a seleção
/// principal (0x53b3d0) de cada nação ([0xae2350] nações de 0x122 bytes) que ainda não esteja nela, até 32; ordena a
/// tabela [+0x14] (6 bytes) pela reputação (qsort 0x945333 com 0x4b07b0); as sedes (0x5fa730 em [0xb5e134]: thiscall,
/// ret 0x14, com o id da Copa [0x9cf79c], o ano como word, as duas saídas e 1; saída −1 = sem sede) e a detentora (o
/// campeão mais recente, 0x4afd80 da competição [0xae23d0] + id × 0x6b) vão para as vagas 1, 2 e 0 (trocando de lugar),
/// e as outras são ordenadas de novo a partir da primeira livre; as 8 primeiras ficam cabeças de chave (pote 0,
/// situação 3) e as outras 24 vão para o pote do continente (+0x53 nação, +0x71 continente: [0x9cfa10] pote 1,
/// [0x9cfa18]/[0x9cfa0c]/[0x9cfa1c] pote 2, [0x9cfa08]/[0x9cfa14] pote 3; sem nação ou continente, a caixa da linha
/// 0x61e) com o id do clube nas vagas +0xee (5 bytes); por fim as 24 da tabela (a partir da 9ª) e as vagas +0x116 são
/// ordenadas pelo pote (0x92f660).
pub(crate) unsafe extern "thiscall" fn seed_teams(this: usize) {
    if read_i16(this + league::YEAR) == 0x7d2 {
        crate::comparison::branch("cqf_6 sem cabeças de chave em 2002");
        return;
    }
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
    let champion: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    write_u8(this + 0x4b, 0);
    if read_i16(this + 0x36) < 0x20 {
        show_error_at(0xad5888, 0x59e);
        let mut k = 0i32;
        let mut desloc = 0usize;
        if read_u32_as_usize(G_NATION_COUNT) as i32 > 0 {
            while read_i16(this + 0x36) < 0x20 {
                let mut j = 0i16;
                let mut ja = false;
                while j < read_i16(this + 0x36) {
                    let c = main(read_u32_as_usize(G_NATION_TABLE).wrapping_add(desloc));
                    if read_u32_as_usize(read_u32_as_usize(this + 0x14) + j as usize * 6) == c {
                        ja = true;
                        break;
                    }
                    j += 1;
                }
                if !ja {
                    let c = main(read_u32_as_usize(G_NATION_TABLE).wrapping_add(desloc));
                    write_u32_from_usize(read_u32_as_usize(this + 0x14) + read_i16(this + 0x36) as usize * 6, c);
                    write_u16(this + 0x36, (read_i16(this + 0x36) as u16).wrapping_add(1));
                }
                k += 1;
                desloc += 0x122;
                if k >= read_u32_as_usize(G_NATION_COUNT) as i32 {
                    break;
                }
            }
        }
    }
    sort(read_u32_as_usize(this + 0x14), read_i16(this + 0x36) as i32 as u32, 6, 0x4b07b0);
    // simplification: both outputs start at −1 (no venue); the original leaves them with the stack leftover when 0x5fa730 does not
    // finds the Cup (it only writes them when it finds it)
    let (mut a, mut b) = (-1i32, -1i32);
    sedes(read_u32_as_usize(G_HOST_COUNTRY), read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), read_i16(this + league::YEAR) as u16 as u32, &mut a, &mut b, 1);
    let sede1 = if a != -1 { main(read_u32_as_usize(G_NATION_TABLE).wrapping_add(a.wrapping_mul(0x122) as usize)) } else { 0 };
    let sede2 = if b != -1 { main(read_u32_as_usize(G_NATION_TABLE).wrapping_add(b.wrapping_mul(0x122) as usize)) } else { 0 };
    let detentor = champion(read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) as i32).wrapping_mul(0x6b) as usize), 0);
    let mut fixos = 0i8;
    for i in 0..32usize {
        for (vaga, alvo) in [(0usize, detentor), (1, sede1), (2, sede2)] {
            let t = read_u32_as_usize(this + 0x14);
            let e = t + i * 6;
            if read_u32_as_usize(e) == alvo {
                let (c, w) = (read_u32_as_usize(e), read_i16(e + 4) as u16);
                let f = t + vaga * 6;
                write_u32_from_usize(e, read_u32_as_usize(f));
                write_u16(e + 4, read_i16(f + 4) as u16);
                let t = read_u32_as_usize(this + 0x14) + vaga * 6;
                write_u32_from_usize(t, c);
                write_u16(t + 4, w);
                if fixos < vaga as i8 + 1 {
                    fixos = vaga as i8 + 1;
                }
            }
        }
    }
    let mut n = fixos;
    if sede2 == sede1 || sede2 == detentor {
        n = 2;
    }
    if sede1 == detentor {
        n = 1;
    }
    sort(read_u32_as_usize(this + 0x14) + n as i32 as usize * 6, (0x20 - n as i32) as u32, 6, 0x4b07b0);
    for i in 0..0x20usize {
        let club = read_u32_as_usize(read_u32_as_usize(this + 0x14) + i * 6);
        let pote = this + 0xf2 + i * 5;
        write_u32_from_usize(pote - 4, read_u32_as_usize(club));
        if i < 8 {
            write_u8(pote, 0);
            write_u8(read_u32_as_usize(this + 0x14) + i * 6 + 4, 3);
            continue;
        }
        let nation = read_u32_as_usize(club + club::NATION);
        let count = if nation != 0 { read_u32_as_usize(nation + nation::CONTINENT) } else { 0 };
        if count == 0 {
            show_error_at(0xad5888, 0x61e);
            continue;
        }
        let c = read_u32_as_usize(count);
        let (p, s) = if c == read_u32_as_usize(KEY_CONTINENT_EUROPE) {
            (1, 0xa)
        } else if c == read_u32_as_usize(KEY_CONTINENT_OCEANIA) || c == read_u32_as_usize(KEY_CONTINENT_ASIA) || c == read_u32_as_usize(KEY_CONTINENT_SOUTH_AMERICA) {
            (2, 0xb)
        } else if c == read_u32_as_usize(KEY_CONTINENT_AFRICA) || c == read_u32_as_usize(KEY_CONTINENT_NORTH_AMERICA) {
            (3, 0xc)
        } else {
            continue;
        };
        write_u8(pote, p);
        write_u8(read_u32_as_usize(this + 0x14) + i * 6 + 4, s);
    }
    sort(read_u32_as_usize(this + 0x14) + 0x30, 0x18, 6, 0x92f660);
    sort(this + 0x116, 0x18, 5, 0x92f660);
}

