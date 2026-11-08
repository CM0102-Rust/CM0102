use super::*;

/// Has `clube` already clinched first place in this league stage (`this`) after match `partida`? Only for counting
/// competitions, with bit 1 of +0xd9 and round +0xa9 >= 24: on a copy of the table updated with the match and sorted,
/// the club must be top and no other club can reach its points (+0x19) with its remaining games (+0xa7 - +0x11) times
/// points per win (+0xc2). Returns 1 if decided, else 0.
///
/// [pt] { 0x6854a0 } (ret 8, league.cpp): a fase (this) com a partida (`partida`, +0x42 = 0xff) ja decide
/// o clube? So nas competicoes que contam (0x4b0700), com o bit 1 de +0xd9 e a rodada +0xa9 >= 24:
/// a linha do clube (+0xb1, 0x3b bytes, +0x3e linhas) sem situacao +0x37 de 5, 2, 3, -2, -4 ou -5;
/// numa copia da tabela (0x687710) preenchida (0x685f60) e ordenada (0x6827d0), o clube tem de estar
/// na frente (+4 zero) e nenhum outro alcanca-lo com os jogos que faltam (+0xa7 - +0x11 vezes os
/// pontos por vitoria +0xc2, contra os pontos +0x19). A copia volta (0x687970).
pub(crate) unsafe extern "thiscall" fn group_decided(this: usize, club: usize, partida: usize) -> u32 {
    if *((partida + 0x42) as *const u8) != 0xff {
        return 0;
    }
    let count: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    if count(*(*((this + 4) as *const usize) as *const i32)) == 0 {
        return 0;
    }
    if *((this + league::LEAGUE_FLAGS) as *const u8) & 1 == 0 || *((this + league::ROUND_COUNT) as *const i16) < 24 {
        return 0;
    }
    let rows = *((this + league::CLUB_COUNT) as *const i16);
    let table = *((this + league::TABLE) as *const usize);
    let row = |base: usize, k: i16| base.wrapping_add(k as usize * 0x3b);
    let Some(k) = (0..rows).find(|&k| *(row(table, k) as *const usize) == club) else {
        return 0;
    };
    let r = row(table, k);
    if r == 0 || matches!(*((r + 0x37) as *const i8), 5 | 2 | 3 | -2 | -4 | -5) {
        return 0;
    }
    let copy: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_COPY_TABLE);
    let fill: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32) = core::mem::transmute(FN_LEAGUE_APPLY_RESULT_TO_TABLE);
    let sort: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let t = copy(this);
    if t == 0 {
        return 0;
    }
    fill(this, partida, t, *((this + league::CLUB_COUNT) as *const u16) as i32, 0, 0, 0);
    sort(this, t);
    let n = *((this + league::CLUB_COUNT) as *const i16);
    // the original reads +4 from address 0 if the club is not in the copy (never happens in the game)
    let achada = (0..n).find(|&k| *(row(t, k) as *const usize) == club).map_or(0, |k| row(t, k));
    if *((achada + 4) as *const u8) != 0 {
        soltar(this, t);
        return 0;
    }
    let rounds = *((this + league::GAMES_PER_CLUB) as *const i16);
    let points = *((this + league::WIN_POINTS) as *const i8) as i16;
    for k in 1..*((this + league::CLUB_COUNT) as *const i16) {
        let o = row(t, k);
        let games = *((o + 0x11) as *const i8) as i16;
        let reach = rounds.wrapping_sub(games).wrapping_mul(points).wrapping_add(*((o + 0x19) as *const i16));
        if reach >= *((t + 0x19) as *const i16) && (*((t + 0x11) as *const i8) as i16 != rounds || games != rounds) {
            soltar(this, t);
            return 0;
        }
    }
    soltar(this, t);
    1
}

/// 0x6be120 with the commentary callback injected: commentates the side's table/tie situation (group decided, knockout
/// resolved, ahead on aggregate, undecided).
///
/// [pt] { 0x6be120 } (ret 4, match_eng.cpp) com o comentario injetado: narra a situacao da
/// classificacao do lado. Grupo decidido (0x6854a0) ou mata-mata ja resolvido (0x6ce310 na rodada
/// 0x82 ou 0x96): 0x2171/0x2170. Com +0x8ebb: 0x2120 se o lado esta na frente do agregado, senao
/// 0x2121 quando a media do +0x1b dos titulares e baixa e a forca (+0x80) do outro clube pesa. No fim,
/// para o lado 1 sem +0x1a8: 0x2001 se o jogo ainda nao estiver decidido.
pub(crate) unsafe fn comment_standing_with(this: usize, side: u32, call: AddEventCall) {
    let t = || *((this + 0x4792) as *const usize);
    let w = |o: usize| *((t() + o) as *const i16);
    let count: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let count_t = || count(*(t() as *const i32));
    let objeto = || *((*(G_COMP_OBJECT_TABLE as *const usize)).wrapping_add((*(t() as *const i32) as usize).wrapping_mul(4)) as *const usize);
    let decidido: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_WOULD_RESULT_CLINCH_TITLE);
    let agregado: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> i8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOAL_MARGIN);
    let commentator = || *((this + 0x8f7a) as *const usize);
    let fala = |c: &mut dyn FnMut(usize, [u32; 7]), cod: u32, l: u32| c(commentator(), [cod, this as u32, u32::MAX, l, u32::MAX, u32::MAX, 0]);
    // the aggregate score with the 5th output (fase3): in the original it lands on the argument itself
    let resolvido = |l: i8| -> bool {
        let (mut a, mut b, mut c, mut d, mut e) = (0i8, 0i8, 0u8, 0u8, 0u8);
        agregado(this, l, &mut a, &mut b, &mut c, &mut d, &mut e, 0) > 0 && e == 0
    };
    let bl = side as u8 as i8;
    if w(0x32) == 0 && count_t() == 0 && w(0x30) == 0x82 && resolvido(bl) {
        fala(call, 0x2171, side);
        return;
    }
    if count_t() != 0 {
        let club = *((t() + if bl != 0 { 0x20 } else { 0x1c }) as *const usize);
        if decidido(objeto(), club, t()) != 0 {
            fala(call, 0x2170, side);
            return;
        }
    } else if w(0x30) == 0x96 && resolvido(bl) {
        fala(call, 0x2170, side);
        return;
    }
    if byte(this, 0x8ebb) != 0 {
        let (mut casa, mut fora, mut classificado, mut x, mut y) = (0i8, 0i8, 0u8, 0u8, 0u8);
        agregado(this, bl, &mut casa, &mut fora, &mut classificado, &mut x, &mut y, 0);
        if casa > fora || (casa == fora && classificado != 0) {
            fala(call, 0x2120, side);
        } else {
            let mut sum = 0.0f64;
            let mut n = 0i8;
            for k in 0..20usize {
                let j = side_player_slot(this, bl, k);
                if k <= 10 || *((j + 0x1f) as *const u8) != 0xff {
                    sum += byte(j, 0x1b) as f64;
                    n = n.wrapping_add(1);
                }
            }
            // with no starter the average would be whatever was in the argument (does not happen)
            let average = if n != 0 { (sum / n as f64) as f32 as f64 } else { 0.0 };
            let strength = |o: usize| *((*((t() + o) as *const usize) + 0x80) as *const i16) as f64;
            let x = average * read_f64_at(0x968968) - read_f64_at(0x968950); // 0x968968 = 0.5, 0x968950 = 2
            if bl == 0 {
                if average < read_f64_at(0x968988) && x * strength(0x20) < strength(0x1c) { // 0x968988 = 7
                    fala(call, 0x2121, 0);
                }
                return;
            }
            if average < read_f64_at(0x96d2e8) && x * strength(0x1c) < strength(0x20) * read_f64_at(0x9689b0) { // 0x96d2e8 = 6.67, 0x9689b0 = 0.8
                fala(call, 0x2121, side);
            }
        }
    }
    if bl != 1 || byte(this, 0x1a8) != 0 {
        return;
    }
    if w(0x32) == 0 && count_t() == 0 && w(0x30) == 0x82 && resolvido(0) {
        return;
    }
    if count_t() != 0 {
        if decidido(objeto(), *((t() + 0x1c) as *const usize), t()) != 0 {
            return;
        }
    } else if w(0x30) == 0x96 && resolvido(0) {
        return;
    }
    fala(call, 0x2001, 0);
}

/// Adds the match commentary about side `lado`'s qualification situation (group decided, tie already resolved, ahead on
/// aggregate, game still open); thin wrapper over comment_standing_with with the real commentary call.
///
/// [pt] { 0x6be120 } (ret 4, match_eng.cpp): ver `comment_standing_with`.
pub(crate) unsafe extern "thiscall" fn narrate_qualification(this: usize, side: u32) {
    comment_standing_with(this, side, &mut |t, a| add_match_event(t, a))
}

/// As datas das 6 rodadas das fases de grupos (0x68a160: rodada, dia, mês 0..11, anos a somar, dia da semana; código 2,
/// valor 0), pela metade da fase: [0] primeira fase, grupos 0..3; [1] primeira fase, 4..7; [2] segunda fase, 8..9; [3]
/// segunda fase, 10..11.
pub(crate) const GROUP_STAGE_DATES: [[(i32, i32, i32, i32, i32); 6]; 4] = [
    [(0, 0xc, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // cqh4: a primeira fase, grupos A a D
    [(0, 0xd, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // cqh4: a primeira fase, grupos E a H
    [(0, 0x15, 0xa, 0, 1), (1, 6, 0xb, 0, 2), (2, 0xe, 1, 1, 2), (3, 0x14, 1, 1, 1), (4, 7, 2, 1, 2), (5, 0xd, 2, 1, 1)],
    [(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0xe, 2, 1, 2)], // cqh4: a segunda fase, grupos C e D
];


/// Collects the bytes the fixture table actually writes: per round +0..+0xb and +0x3d..+0x40, for 6 rounds of 0x41
/// bytes.
///
/// [pt] Os bytes que a tabela tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4, +0x3d..+0x40
/// e o jogo 0 (+5..+0xb).
pub(crate) unsafe fn league_written_table_bytes(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..6usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}




/// Sets the season expectations of each club in the table, through the manager manager, with the news.
///
/// Ret 0, a league method: `League::set_club_season_expectations()`: with the manager manager (`[0xb67a44]`), the
/// season expectations of each club in the table (+0xb1, rows of 0x3b bytes, +0x3e clubs) through 0x6a3770 (push 1;
/// push club; mov ecx, `[0xb67a44]` -> thiscall, ret 8: with the news).
/// Original: `League::set_club_season_expectations` (0x68aa80, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x68aa80 } (thiscall, ret 0, league.cpp, método da liga): `League::set_club_season_expectations()`: com o
/// gerenciador dos técnicos ([0xb67a44]), as expectativas da temporada de cada clube da tabela (+0xb1, linhas de 0x3b
/// bytes, +0x3e clubes) pela 0x6a3770 (push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 8: com a notícia).
pub(crate) unsafe extern "thiscall" fn set_club_season_expectations(this: usize) {
    if read_u32_as_usize(G_MANAGER_MANAGER) == 0 {
        return;
    }
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
    let mut i = 0i16;
    while i < read_i16(this + league::CLUB_COUNT) {
        crate::comparison::branch("cqh_4 expectativa do clube");
        let club = read_u32_as_usize(read_u32_as_usize(this + league::TABLE) + i as usize * 0x3b);
        expectativas(read_u32_as_usize(G_MANAGER_MANAGER), club, 1); // cqh4: com a notícia
        i += 1; // cqh4: o próximo clube
    }
}


/// Moves the club down to the division (keeping the previous one), applies the relegation clauses and makes the human
/// manager leave when the division is not played.
///
/// Ret 0xc; this is not read: `League::relegate_club_to_division(club, division, _)`: the club's competition (+0x57)
/// becomes the division and the previous one goes to +0x5b; the relegation clauses (0x4cd030 push old; push club; mov
/// ecx, `[0xae1050]` -> thiscall, ret 8); if the division is not played (no competition table `[0xadadfc]`, no division
/// or no competition with its id) and the club has a human manager who decides (0x600a50 push 0; push 0; push 1; push
/// 1; push club; add esp, 0x14 -> cdecl), the manager leaves (0x69b790 push 0; push 7; push 1; push club; mov ecx,
/// `[0xb67a44]` -> thiscall, ret 0x10).
/// Original: `League::relegate_club_to_division` (0x6831a0, league.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x6831a0 } (thiscall, ret 0xc, league.cpp; this não é lido): `League::relegate_club_to_division(clube, divisão,
/// _)`: a competição do clube (+0x57) vira a divisão e a anterior vai para +0x5b; as cláusulas de rebaixamento (0x4cd030
/// push antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8); se a divisão não é jogada (sem a tabela das
/// competições [0xadadfc], sem divisão ou sem a competição do id dela) e o clube tem técnico humano que decide
/// (0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl), o técnico sai (0x69b790 push 0; push
/// 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn relegate_club_to_division(_this: usize, club: usize, division: usize, _x: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_TRIGGER_RELEGATION_CLAUSES);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let remove: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_CLUB_MANAGER);
    let antiga = read_u32_as_usize(club + club::DIVISION);
    core::ptr::write_unaligned((club + club::DIVISION) as *mut u32, division as u32);
    core::ptr::write_unaligned((club + club::LAST_DIVISION) as *mut u32, antiga as u32);
    clausulas(read_u32_as_usize(G_CONTRACT_MANAGER), club, antiga);
    let comps = read_u32_as_usize(G_COMP_OBJECT_TABLE);
    if comps != 0 && division != 0 && read_u32_as_usize(comps + read_u32_as_usize(division) * 4) != 0 {
        return;
    }
    if human(club, 1, 1, 0, 0) != 0 {
        crate::comparison::branch("cqh_4 técnico sai");
        remove(read_u32_as_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
    }
}









/// League TV date table entry (0x41 bytes at [+0xba], count +0xa9) for a date via bsearch, as 0x685640; None if out of
/// range or the first slot is empty.
///
/// [pt] A entrada da tabela das datas da TV da liga (0x41 bytes em [+0xba], conta +0xa9) para a data, como a 0x685640: sem
/// entradas, data depois da última (ano +2 + o da liga +0x40, sem sinal; dia +0 com sinal) ou sem a entrada pela busca
/// (0x946464: push 0x685cf0; push 0x41; push conta; push tabela; push &chave; add esp, 0x14 -> cdecl bsearch; a chave
/// é o dia e o ano menos o da liga) ou com a primeira vaga vazia (+0xb = 0xff): nenhuma.
pub(crate) unsafe fn tv_date_entry(this: usize, data: usize) -> Option<usize> {
    let n = read_i16(this + league::ROUND_COUNT);
    if n == 0 {
        return None;
    }
    let table = read_u32_as_usize(this + league::ROUND_DATES);
    let base = read_i16(this + league::YEAR);
    let ult = (table as isize + n as isize * 0x41 - 0x41) as usize;
    let ult_year = read_i16(ult + 2) as u16 as i32 + base as i32;
    let year = read_i16(data + 2) as u16 as i32;
    if year > ult_year || (year == ult_year && read_i16(data) > read_i16(ult)) {
        return None;
    }
    let key = [read_i16(data), read_i16(data + 2).wrapping_sub(base)];
    let search: unsafe extern "C" fn(*const i16, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = search(key.as_ptr(), table, n as i32, 0x41, 0x685cf0);
    if e == 0 || read_u8(e + 0xb) == 0xff {
        return None;
    }
    Some(e)
}


/// Moves fixtures for TV: on the TV table dates, each slot takes the highest-rated fixture (a random draw by
/// reputation, zeroed when either club plays 2..5 days before or after; 100/50 in national team slots) to the weekday
/// and time slot of the slot, with the news, the stadium and the calendar.
///
/// Ret 4; SEH frame with no destructor on the normal path (the local dates and list):
/// `League::move_fixtures_for_tv(date)`, from the league day (0x6847c0) and from building the calendar (0x683e30).
/// Through the date's entry (tv_date_entry), the league matches on the entry's day and time slot (+4) go to a local
/// list (0x68d020; 0x5abd90: push list; push league +0x44; push -1; push id; push slot; push day; push year; mov ecx,
/// `[0xae2a58]` -> thiscall, ret 0x1c). With matches: the entry's slots (7 bytes from +5, up to the first with +0xb =
/// 0xff) are sorted (0x945333 with 0x8b4e00) if any has bits 0x7fc0 in dword +5. For each slot with time slot (+0xa)
/// 0..2: the date moved to the slot's weekday (+9; 0x54a2d0); the clash window: 2..5 days after the requested date if
/// the TV day falls after it, otherwise 5..2 days before (0x54c5b0/0x54c8e0 + 0x417c10). Among the matches in the list
/// (of the league's type, +0x42), the one with the highest rating (signed word; the first on a tie): slot of type 3,
/// 100 for a national side match (0x53b900 on the home side) in the stadium (+0x18 → +0x38 → +0x1f) of the nation of
/// either side, otherwise 50; otherwise the random draw (0x90cfc0) of the away side's reputation (+0x80) + 1500 plus
/// that of the home side's, zeroed if either has a match in the window (0x5acbe0). The chosen one: outside type 3, news
/// 3 to both clubs; it leaves the calendar (0x5abc30), takes the TV date, the time slot and the slot's type, the
/// stadium (0x85c260) and goes back into the calendar (0x5aa680); the slot becomes 0xff and the match leaves the list
/// (0x68d1f0).
/// Original: `League::move_fixtures_for_tv` (0x685640, league.cpp, thiscall, ret 4).
///
/// [pt] { 0x685640 } (thiscall, ret 4, league.cpp, quadro SEH sem destrutor no caminho normal: as datas e a lista locais):
/// `League::move_fixtures_for_tv(data)`, do dia da liga (0x6847c0) e da montagem do calendário (0x683e30). Pela
/// entrada da data (tv_date_entry), os jogos da liga no dia e fatia da entrada (+4) vão a uma lista local (0x68d020;
/// 0x5abd90: push lista; push liga +0x44; push -1; push id; push fatia; push dia; push ano; mov ecx, [0xae2a58] ->
/// thiscall, ret 0x1c). Com jogos: as vagas da entrada (7 bytes a partir de +5, até a primeira com +0xb = 0xff) são
/// ordenadas (0x945333 com 0x8b4e00) se alguma tem bits 0x7fc0 no dword +5. Para cada vaga com fatia (+0xa) 0..2: a data
/// levada ao dia da semana da vaga (+9; 0x54a2d0); a janela de conflito: 2..5 dias depois da data pedida se o dia da TV
/// cai depois dela, senão 5..2 dias antes (0x54c5b0/0x54c8e0 + 0x417c10). Entre os jogos da lista (do tipo da liga,
/// +0x42), o de maior nota (palavra com sinal; o primeiro no empate): vaga do tipo 3, 100 para jogo de seleção (0x53b900
/// no mandante) no estádio (+0x18 → +0x38 → +0x1f) da nação de um dos dois, senão 50; senão o sorteio (0x90cfc0) da
/// reputação (+0x80) do visitante + 1500 mais o da do mandante, zerada se um dos dois tem jogo na janela (0x5acbe0). O
/// escolhido: fora do tipo 3, a notícia 3 aos dois clubes; sai do calendário (0x5abc30), ganha a data da TV, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta ao calendário (0x5aa680); a vaga fica 0xff e o jogo sai da lista
/// (0x68d1f0).
pub(crate) unsafe extern "thiscall" fn move_fixtures_for_tv(this: usize, data: usize) {
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // the dates from the dwords the original reads from the stack (day|year, year|leap)
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    default(pa);
    default(pb);
    default(pc);
    let e = match tv_date_entry(this, data) {
        Some(e) => e,
        None => return,
    };
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    create(l);
    let games: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_COLLECT_COMP_FIXTURES_ON_DAY);
    // simplification: the original pushes the year, the day, the slice and the type in registers with leftover in the high bytes; the
    // 0x5abd90 only reads the words and the bytes
    let n = games(read_u32_as_usize(G_FIXTURE_MANAGER), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, u32::MAX, read_u8(this + league::STAGE_INDEX) as u32, l) & 0xffff;
    if n != 0 {
        let mut k = 0usize;
        let mut sort = false;
        while k < 8 {
            if read_u8(e + k * 7 + 0xb) == 0xff {
                break;
            }
            if read_u32_as_usize(e + k * 7 + 5) & 0x7fc0 != 0 {
                sort = true;
            }
            k += 1;
        }
        if sort {
            let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
            qsort(e + 5, k as u32, 7, 0x8b4e00);
        }
        let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
        let adjust: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATE_GET_NEAREST_WEEKDAY);
        let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
        let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        let find: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        for k in 0..8usize {
            let v = e + k * 7;
            if read_u8(v + 0xb) == 0xff {
                break;
            }
            let slice = read_u8(v + 0xa) as i8;
            if !(0..=2).contains(&slice) {
                continue;
            }
            let mut x = [0u8; 8];
            let px = x.as_mut_ptr() as usize;
            new(px, read_i16(data) as u16 as u32, read_i16(data + 2) as u16 as u32);
            adjust(px, read_u8(v + 9) as i8 as i16 as u16 as u32, pa);
            let (xa, aa) = (read_i16(px + 2) as u16, read_i16(pa + 2) as u16);
            let (mut t1, mut t2, mut t3, mut t4) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
            if xa < aa || (xa == aa && read_i16(px) < read_i16(pa)) {
                let r = more(px, t1.as_mut_ptr() as usize, 2);
                copy(pb, t2.as_mut_ptr() as usize, r);
                let r = more(px, t3.as_mut_ptr() as usize, 5);
                copy(pc, t4.as_mut_ptr() as usize, r);
            } else {
                let r = minus(px, t1.as_mut_ptr() as usize, 5);
                copy(pb, t2.as_mut_ptr() as usize, r);
                let r = minus(px, t3.as_mut_ptr() as usize, 2);
                copy(pc, t4.as_mut_ptr() as usize, r);
            }
            write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
            let mut best = 0usize;
            let mut rating: i16 = -1;
            let mut f = next(l);
            while f != 0 {
                if read_u8(this + league::STAGE_INDEX) == read_u8(f + 0x42) {
                    let mut s: i32;
                    if read_u8(v + 0xb) == 3 {
                        let est = read_u32_as_usize(f + 0x18);
                        s = 50;
                        if national_team(read_u32_as_usize(f + 0x1c)) != 0 && est != 0 && read_u32_as_usize(est + stadium::STADIUM_CITY) != 0 {
                            let g = read_u32_as_usize(read_u32_as_usize(est + stadium::STADIUM_CITY) + 0x1f);
                            if read_u32_as_usize(read_u32_as_usize(f + 0x1c) + 0x53) == g || read_u32_as_usize(read_u32_as_usize(f + 0x20) + 0x53) == g {
                                s = 100;
                            }
                        }
                    } else {
                        s = sorteio(read_i16(read_u32_as_usize(f + 0x20) + 0x80) as i32 + 0x5dc); // cqc6: o visitante com o bônus
                        s = s.wrapping_add(sorteio(read_i16(read_u32_as_usize(f + 0x1c) + 0x80) as i32));
                    }
                    if read_u8(v + 0xb) != 3 {
                        let window = |club: usize| {
                            find(read_u32_as_usize(G_FIXTURE_MANAGER), read_u32_as_usize(pb) as u32, read_u32_as_usize(pb + 2) as u32, read_u32_as_usize(pc) as u32, read_u32_as_usize(pc + 2) as u32,
                                read_u32_as_usize(club) as u32, 0, u32::MAX, 0xffff_fffe, u32::MAX) as u16 != 0
                        };
                        if window(read_u32_as_usize(f + 0x1c)) || window(read_u32_as_usize(f + 0x20)) {
                            crate::comparison::branch("cqc_6 conflito na janela");
                            s = 0; // cqc6: conflito na janela
                        }
                    }
                    if s as i16 > rating {
                        best = f;
                        rating = s as i16;
                    }
                }
                f = next(l);
            }
            if best == 0 {
                continue;
            }
            let mut buf = [0u8; 0x50];
            let r = buf.as_mut_ptr() as usize;
            core::ptr::copy_nonoverlapping(best as *const u8, r as *mut u8, 0x4f);
            if read_u8(v + 0xb) != 3 {
                let mut news = [0u32; 0x38];
                let n = news.as_mut_ptr() as usize;
                let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
                let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
                let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
                construct(n, 3, 0);
                write(n, 0, read_u32_as_usize(r + 0xc) as u32);
                write(n, 1, read_u32_as_usize(r + 0x10) as u32);
                write(n, 2, read_u32_as_usize(r) as u32);
                write(n, 3, read_i16(r + 0x32) as i32 as u32);
                write(n, 4, read_i16(r + 0x30) as i32 as u32);
                write(n, 5, read_i16(r + 0x2a) as u16 as u32);
                write(n, 6, read_i16(r + 0x28) as u16 as u32);
                write(n, 7, read_u8(r + 0x3f) as u32);
                write(n, 8, read_i16(pa) as i32 as u32);
                write(n, 9, read_i16(pa + 2) as u16 as u32);
                write(n, 10, read_u8(v + 0xa) as i8 as i32 as u32);
                write(n, 11, read_u8(v + 0xb) as i8 as i32 as u32);
                deliver(read_u32_as_usize(G_NEWS), n, read_u32_as_usize(r + 0x1c));
                deliver(read_u32_as_usize(G_NEWS), n, read_u32_as_usize(r + 0x20));
            }
            let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
            remove(read_u32_as_usize(G_FIXTURE_MANAGER), r);
            crate::comparison::branch("cqc_6 jogo para a TV");
            write_u16(r + 0x28, read_i16(pa + 2) as u16);
            write_u16(r + 0x2a, read_i16(pa) as u16); // cqc6: o dia da TV
            write_u8(r + 0x3f, read_u8(v + 0xa));
            write_u8(r + 0x3b, read_u8(v + 0xb));
            let stadium: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
            stadium(read_u32_as_usize(G_STADIUM), read_u32_as_usize(v + 5), r, read_u32_as_usize(this + league::FIXED_STADIUM_ID));
            let join: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
            join(read_u32_as_usize(G_FIXTURE_MANAGER), r, 1);
            write_u8(v + 0xb, 0xff);
            let exit: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
            exit(l, best);
        }
    }
    destroy(l);
}


/// Sends the group draw news 0x15 (the competition, the first date, the opponents) to each human-managed club in the
/// group.
///
/// Ret 0; SEH frame with no destructor on the normal path: `League::send_group_draw_news()`, from the competitions'
/// group draw. A group of up to 8 teams (+0x3e; the 0x3b-byte teams in `[+0xb1]`, +0 the club): for each club with a
/// human manager, news 0x15 (allocated once, 0x944cf1(0xdf), built by 0x77bbe0, freed at the end by 0x944c94 without a
/// destructor) with the competition id, +0x3a, the first date (that of the first entry of table `[+0xba]` moved to the
/// weekday of each slot, 0x54a2d0, the earliest), the number of opponents; per club, its own id (2) and those of the
/// others (6 onwards), delivered to it (0x7861e0).
/// Original: `League::send_group_draw_news` (0x684230, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x684230 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal): `League::send_group_draw_news()`,
/// do sorteio dos grupos das competições. Grupo de até 8 times (+0x3e; os times de 0x3b bytes em [+0xb1], +0 o clube):
/// para cada clube de técnico humano, a notícia 0x15 (alocada uma vez, 0x944cf1(0xdf), montada por 0x77bbe0, liberada
/// no fim por 0x944c94 sem destrutor) com o id da competição, +0x3a, a primeira data (a da primeira entrada da tabela
/// [+0xba] levada ao dia da semana de cada vaga, 0x54a2d0, a menor), o número de adversários; por clube, o id dele (2)
/// e os dos outros (6 em diante), entregue a ele (0x7861e0).
pub(crate) unsafe extern "thiscall" fn send_group_draw_news(this: usize) {
    let n = read_i16(this + league::CLUB_COUNT);
    if n > 8 || n <= 0 {
        return;
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let team = |i: u8| read_u32_as_usize(read_u32_as_usize(this + league::TABLE) + (i as i8 as isize * 0x3b) as usize);
    let mut news = 0usize;
    let mut i: u8 = 0;
    loop {
        if cup_has_human_manager(team(i)) {
            if news == 0 {
                let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
                let p = alloc(0xdf);
                news = if p != 0 { construct(p, 0x15, 0) } else { 0 };
                let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
                let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
                let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
                let adjust: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATE_GET_NEAREST_WEEKDAY);
                let (mut d1, mut d2, mut d3, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let (p1, p2, p3) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize, d3.as_mut_ptr() as usize);
                let table = read_u32_as_usize(this + league::ROUND_DATES);
                // simplification: the day and the year go in registers with leftovers in the high 16 bits; 0x549ef0 only reads the words
                new(p1, read_i16(table) as u16 as u32, read_i16(table + 2).wrapping_add(read_i16(this + league::YEAR)) as u16 as u32);
                default(p2);
                default(p3);
                copy(p2, t.as_mut_ptr() as usize, p1);
                for j in 0..8usize {
                    let v = read_u32_as_usize(this + league::ROUND_DATES) + j * 7;
                    if read_u8(v + 0xb) == 0xff {
                        break;
                    }
                    adjust(p1, read_u8(v + 9) as i8 as i16 as u16 as u32, p3);
                    let (y2, y3) = (read_i16(p2 + 2) as u16, read_i16(p3 + 2) as u16);
                    if y2 > y3 || (y2 == y3 && read_i16(p2) > read_i16(p3)) {
                        let mut t2 = [0u8; 8];
                        copy(p2, t2.as_mut_ptr() as usize, p3);
                    }
                }
                write(news, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
                write(news, 1, read_i16(this + league::STAGE_ID) as i32 as u32);
                write(news, 3, read_i16(p2) as i32 as u32);
                write(news, 4, read_i16(p2 + 2) as u16 as u32);
                write(news, 5, (read_i16(this + league::CLUB_COUNT) as i32 - 1) as u32);
            }
            write(news, 2, read_u32_as_usize(team(i)) as u32);
            let mut k: u8 = 6;
            let mut j: u8 = 0;
            while (j as i8 as i16) < read_i16(this + league::CLUB_COUNT) {
                if j != i {
                    write(news, k as i8, read_u32_as_usize(team(j)) as u32);
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            crate::comparison::branch("cqc_6 noticia do grupo");
            deliver(read_u32_as_usize(G_NEWS), news, team(i));
        }
        i = i.wrapping_add(1);
        if (i as i8 as i16) >= read_i16(this + league::CLUB_COUNT) {
            break;
        }
    }
    if news != 0 {
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        free(news);
    }
}


/// Sends news 0xd with the first three fixtures (date, opponent, venue) to each human-managed club not yet told, and
/// does the same in the following stages.
///
/// Ret 4; SEH frame with no destructor on the normal path (the local lists): `League::send_fixture_list_news(list)`,
/// from creating the calendar (0x5cd1e0) and from the league day in its year. Uses the local list (0x68d020) if the
/// league is a first stage (+0x44 = 0xff) or there is no list. With the competition counting (0x4b0700: push id; add
/// esp, 4 -> cdecl) and a first stage: the date of the first entry of table `[+0xba]` minus 5 days (0x54c8e0,
/// 0x417c10); for each club with a human manager (0x3b-byte teams in `[+0xb1]`) that is not in the list (4-byte ids):
/// its first 3 matches from that date (0x5acee0 into a local list, 18 arguments: push 1; push -1; push +0x44; push 0;
/// push list; push 3; push -1 ×4; push club id; push -1; push -1; push competition id; push 1; push &slot; push &day;
/// push &year; mov ecx, `[0xae2a58]` -> thiscall, ret 0x48); finding 3: news 0xd (competition id, club id, the league
/// year and, per match, the day, the year, the opponent and 1 at home or 0 away) to the club, which enters the list
/// (0x68d0d0: push 4; push club; mov ecx, list -> thiscall, ret 8). Then, in the stages (`[+0xc]`, up to +0x30) with
/// +0x43 = 2, the same routine with the list.
/// Original: `League::send_fixture_list_news` (0x683ee0, league.cpp, thiscall, ret 4).
///
/// [pt] { 0x683ee0 } (thiscall, ret 4, league.cpp, quadro SEH sem destrutor no caminho normal: as listas locais):
/// `League::send_fixture_list_news(lista)`, da criação do calendário (0x5cd1e0) e do dia da liga no ano dela. Usa a
/// lista local (0x68d020) se a liga é de primeira fase (+0x44 = 0xff) ou sem lista. Com a competição contando
/// (0x4b0700: push id; add esp, 4 -> cdecl) e de primeira fase: a data da primeira entrada da tabela [+0xba] menos 5 dias
/// (0x54c8e0, 0x417c10); para cada clube de técnico humano (times de 0x3b bytes em [+0xb1]) que não esteja na lista (ids
/// de 4 bytes): os primeiros 3 jogos dele a partir da data (0x5acee0 numa lista local, 18 argumentos: push 1; push -1;
/// push +0x44; push 0; push lista; push 3; push -1 ×4; push id do clube; push -1; push -1; push id da competição; push 1;
/// push &fatia; push &dia; push &ano; mov ecx, [0xae2a58] -> thiscall, ret 0x48); achando 3: a notícia 0xd (id da
/// competição, do clube, o ano da liga e, por jogo, o dia, o ano, o adversário e 1 em casa ou 0 fora) ao clube, que
/// entra na lista (0x68d0d0: push 4; push clube; mov ecx, lista -> thiscall, ret 8). Depois, nas fases ([+0xc], até
/// +0x30) com +0x43 = 2, a mesma rotina com a lista.
pub(crate) unsafe extern "thiscall" fn send_fixture_list_news(this: usize, list: usize) {
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut l0 = [0u8; 0x10];
    let p0 = l0.as_mut_ptr() as usize;
    create(p0);
    let list = if read_u8(this + league::STAGE_INDEX) == 0xff || list == 0 { p0 } else { list };
    let count: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    if count(read_u32_as_usize(read_u32_as_usize(this + 4)) as u32) == 0 || read_u8(this + league::STAGE_INDEX) != 0xff {
        destroy(p0);
        return;
    }
    let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let (mut d1, mut t, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p1 = d1.as_mut_ptr() as usize;
    let table = read_u32_as_usize(this + league::ROUND_DATES);
    // simplification: the day and the year go in registers with leftovers in the high 16 bits; 0x549ef0 only reads the words
    new(p1, read_i16(table) as u16 as u32, read_i16(table + 2).wrapping_add(read_i16(this + league::YEAR)) as u16 as u32);
    let r = minus(p1, t.as_mut_ptr() as usize, 5);
    copy(p1, t2.as_mut_ptr() as usize, r);
    let team = |i: u32| read_u32_as_usize(read_u32_as_usize(this + league::TABLE) + (i as i16 as isize * 0x3b) as usize);
    let mut i: u32 = 0;
    if read_i16(this + league::CLUB_COUNT) > 0 {
        loop {
            if cup_has_human_manager(team(i)) {
                write_u32_from_usize(list + 0xa, read_u32_as_usize(list + 2));
                let mut ja = false;
                let mut it = next(list);
                while it != 0 {
                    if read_u32_as_usize(it) == read_u32_as_usize(team(i)) {
                        ja = true;
                        break;
                    }
                    it = next(list);
                }
                if !ja {
                    let mut v_year = read_u32_as_usize(p1 + 2) as u32;
                    let mut v_day = read_u32_as_usize(p1) as u32;
                    let mut slice: u8 = 0;
                    let mut l2 = [0u8; 0x10];
                    let p2 = l2.as_mut_ptr() as usize;
                    create(p2);
                    let search: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, u32, i32, i32, i32,
                                                            i32, u32, usize, u32, u32, i32, u32) -> u32 =
                        core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
                    let found = search(read_u32_as_usize(G_FIXTURE_MANAGER), &mut v_year, &mut v_day, &mut slice, 1, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, -1, -1,
                        read_u32_as_usize(team(i)) as u32, -1, -1, -1, -1, 3, p2, 0, read_u8(this + league::STAGE_INDEX) as u32, -1, 1) as u16;
                    if found == 3 {
                        crate::comparison::branch("cqc_6 noticia da tabela");
                        let mut news = [0u32; 0x38];
                        let n = news.as_mut_ptr() as usize;
                        let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
                        let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
                        let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
                        let store: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
                        construct(n, 0xd, 0);
                        let mut k: u8 = 3;
                        write(n, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
                        write(n, 1, read_u32_as_usize(team(i)) as u32);
                        write(n, 2, read_i16(this + league::YEAR) as i32 as u32);
                        write_u32_from_usize(p2 + 0xa, read_u32_as_usize(p2 + 2));
                        let mut f = next(p2);
                        while f != 0 {
                            write(n, k as i8, read_i16(f + 0x2a) as u16 as u32);
                            k = k.wrapping_add(1);
                            write(n, k as i8, read_i16(f + 0x28) as u16 as u32);
                            k = k.wrapping_add(1);
                            if team(i) == read_u32_as_usize(f + 0x1c) {
                                write(n, k as i8, read_u32_as_usize(read_u32_as_usize(f + 0x20)) as u32);
                                k = k.wrapping_add(1);
                                write(n, k as i8, 1);
                            } else {
                                write(n, k as i8, read_u32_as_usize(read_u32_as_usize(f + 0x1c)) as u32);
                                k = k.wrapping_add(1);
                                write(n, k as i8, 0);
                            }
                            k = k.wrapping_add(1);
                            f = next(p2);
                        }
                        deliver(read_u32_as_usize(G_NEWS), n, team(i));
                        store(list, team(i), 4);
                    }
                    destroy(p2);
                }
            }
            i = i.wrapping_add(1);
            if (i as i16) >= read_i16(this + league::CLUB_COUNT) {
                break;
            }
        }
    }
    if read_u8(this + league::STAGE_INDEX) == 0xff && (read_u32_as_usize(this + league::CURRENT_STAGE) as i32) >= 0 {
        let mut j: u8 = 0;
        loop {
            let stage = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + (j as i8 as isize * 4) as usize);
            if read_u8(stage + 0x43) == 2 {
                let same: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_SEND_FIXTURE_LIST_NEWS);
                same(stage, list);
            }
            j = j.wrapping_add(1);
            if j as i8 as i32 > read_u32_as_usize(this + league::CURRENT_STAGE) as i32 {
                break;
            }
        }
    }
    destroy(p0);
}








/// Returns the method pointer at offset k of object o's vtable.
///
/// [pt] O método `k` (deslocamento na tabela virtual) do objeto `o`.
pub(crate) unsafe fn vtable_method(o: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(o) + k)
}

/// True if two dates have the same day and year words.
///
/// [pt] As duas datas (dia, ano) são a mesma (as palavras, como a original compara).
pub(crate) unsafe fn same_date(a: usize, b: usize) -> bool {
    read_i16(a + 2) == read_i16(b + 2) && read_i16(a) == read_i16(b)
}

/// The league day: the fixture list news on its day, each stage's day, the +0xc9 date event, the season end (next
/// stage, history, records), the TV fixtures four weeks ahead, the registration deadline 5/20/14 days before the first
/// date and the +0xd1 date.
///
/// Ret 8; SEH frame with no destructor on the normal path (the local dates): `League::update_for_date(date, full)`, the
/// day of each league (through the competitions, 46522 calls up to 2002-06-01). League stopped (+0xb5) or without clubs
/// (+0x45 = 0): 0. Otherwise (returns 1, in the byte):
///
/// - +0xa7 = -1 with teams (+0x3e) > 0: +0xa7 = (teams − 1) · legs (+0x3c), in the word;
///
/// - in the first stage (+0x44 = 0xff) with a table (+0xb1, +0x11 = 0), full and a new date (≠ +0xd1): on the day after
/// the key nation's Cup day (0x669470; `[0xb63d75 + 72·k]` + 1, 0 after 364) and in the league year (+0x40), the table
/// news (0x683ee0: push 0; mov ecx, this -> thiscall, ret 4);
///
/// - in the first stage, each stage's day (+0xc, up to +0x30; method 0x14 with the date and the full flag);
///
/// - on date +0xc9 (without +0xdd): +0xad = 0 and method 0x94;
///
/// - in the first stage with a table and a new date: method 0x10 (0, 0); season end (+0xdd = 1) with all stages
/// finished (method 0x10 with (0, 1) ≠ 0): the next stage (method 0x28) if +0x30 < +0x2c − 1, otherwise (once, +0x4c)
/// the history (0x442e80: push league; mov ecx, `[0xad9da8]` -> thiscall, ret 4), the records with 1 (0x447430: push 1;
/// push league; mov ecx, `[0xad9dac]` -> thiscall, ret 8) if the competition counts (0x4b0700: push id; add esp, 4 ->
/// cdecl), the box at line 0x74c if +0xe6, method 0x30 and the end of attachment +8 (0x4a19f0: mov ecx, `[+8]` ->
/// thiscall, ret 0); then the records with 0 and the attachment (0x4a2450: mov ecx, `[+8]` -> thiscall, ret 0);
///
/// - with a table and full, without +0xdd: the TV fixtures four weeks ahead (0x54c770 + 0x685640: push &date; mov ecx,
/// this -> thiscall, ret 4);
///
/// - full: in the first stage with registrations (+0x52 bit 2), a new date, without +0xdd and the table in progress: if
/// the first date of the TV table (+0xba; day, year − the league's) falls 5 days from now (Argentina `[0x9cf218]`), 20
/// (Italy `[0x9cf360]`) or 14 days, the registration deadline (0x859e20: push 0; push 0; push league; mov ecx,
/// `[0xdd7ef0]` -> thiscall, ret 0xc); and +0xd1 takes the date (0x417c10).
/// Original: `League::update_for_date` (0x6847c0, league.cpp, thiscall, ret 8).
///
/// [pt] { 0x6847c0 } (thiscall, ret 8, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::update_for_date(data, completo)`, o dia de cada liga (pelas competições, 46522 chamadas até 2002-06-01).
/// Liga parada (+0xb5) ou sem clubes (+0x45 = 0): 0. Senão (devolve 1, no byte):
/// - +0xa7 = -1 com times (+0x3e) > 0: +0xa7 = (times − 1) · turnos (+0x3c), na palavra;
/// - na primeira fase (+0x44 = 0xff) com tabela (+0xb1, +0x11 = 0), completo e a data nova (≠ +0xd1): no dia seguinte ao
///   do dia da Copa da nação-chave (0x669470; [0xb63d75 + 72·k] + 1, 0 depois de 364) e no ano da liga (+0x40), a
///   notícia da tabela (0x683ee0: push 0; mov ecx, this -> thiscall, ret 4);
/// - na primeira fase, o dia de cada fase (+0xc, até +0x30; método 0x14 com a data e o completo);
/// - na data +0xc9 (sem +0xdd): +0xad = 0 e o método 0x94;
/// - na primeira fase com tabela e a data nova: o método 0x10 (0, 0); fim da temporada (+0xdd = 1) com todas as fases
///   acabadas (método 0x10 com (0, 1) ≠ 0): a fase seguinte (método 0x28) se +0x30 < +0x2c − 1, senão (uma vez, +0x4c)
///   o histórico (0x442e80: push liga; mov ecx, [0xad9da8] -> thiscall, ret 4), os recordes com 1 (0x447430: push 1;
///   push liga; mov ecx, [0xad9dac] -> thiscall, ret 8) se a competição conta (0x4b0700: push id; add esp, 4 -> cdecl),
///   a caixa da linha 0x74c se +0xe6, o método 0x30 e o fim do anexo +8 (0x4a19f0: mov ecx, [+8] -> thiscall, ret 0);
///   depois os recordes com 0 e o anexo (0x4a2450: mov ecx, [+8] -> thiscall, ret 0);
/// - com tabela e completo, sem +0xdd: os jogos para a TV quatro semanas adiante (0x54c770 + 0x685640: push &data; mov
///   ecx, this -> thiscall, ret 4);
/// - completo: na primeira fase com inscrições (+0x52 bit 2), a data nova, sem +0xdd e tabela em andamento: se a primeira
///   data da tabela da TV (+0xba; dia, ano − o da liga) cai daqui a 5 dias (Argentina [0x9cf218]), 20 (Itália
///   [0x9cf360]) ou 14 dias, o prazo das inscrições (0x859e20: push 0; push 0; push liga; mov ecx, [0xdd7ef0] ->
///   thiscall, ret 0xc); e +0xd1 fica com a data (0x417c10).
pub(crate) unsafe extern "thiscall" fn league_update_for_date(this: usize, data: usize, full: u32) -> u32 {
    let weeks: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let key: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_COMP);
    let news: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_SEND_FIXTURE_LIST_NEWS);
    let history: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_HISTORY_RECORD_COMP_SEASON);
    let count: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_UPDATE_LEAGUE_POSITION_RECORDS);
    let fim_anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_STATS_UPDATE_SEASON_END_RECORDS);
    let anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_STATS_SEND_RECORD_NEWS);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_MOVE_FIXTURES_FOR_TV);
    let prazo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ASSIGN_LEAGUE_SQUAD_NUMBERS);
    // the requested date copied at the start (the original keeps the day|year dword and the leap-year one)
    let d0 = [read_u32_as_usize(data) as u32, read_u32_as_usize(data + 4) as u32];
    let pd0 = d0.as_ptr() as usize;
    let (mut t, mut x, mut w) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pt, px, pw) = (t.as_mut_ptr() as usize, x.as_mut_ptr() as usize, w.as_mut_ptr() as usize);
    if read_u8(this + league::STOPPED) != 0 || read_u32_as_usize(this + league::ACTIVE) == 0 {
        return 0;
    }
    if read_i16(this + league::GAMES_PER_CLUB) == -1 {
        let n = read_i16(this + league::CLUB_COUNT);
        if n > 0 {
            write_u16(this + league::GAMES_PER_CLUB, n.wrapping_sub(1).wrapping_mul(read_i16(this + league::LEGS)) as u16);
        }
    }
    let table = read_u32_as_usize(this + league::TABLE);
    if table != 0 && read_u8(table + 0x11) == 0 && read_u8(this + league::STAGE_INDEX) == 0xff && full != 0 && !same_date(this + league::LAST_UPDATE_DATE, data) {
        let c = read_u32_as_usize(this + 4);
        if read_u32_as_usize(c + 0x5d) != 0 {
            let k = key(c) as i8 as isize;
            let mut day = (read_i16((0xb63d75 + k * 72) as usize) as u16).wrapping_add(1);
            if day > 0x16c {
                day = 0;
            }
            if day as i32 == read_i16(G_CURRENT_DATE) as i32 && read_i16(this + league::YEAR) as i32 == read_i16(G_CURRENT_DATE_YEAR) as u16 as i32 {
                news(this, 0);
            }
        }
    }
    if read_u8(this + league::STAGE_INDEX) == 0xff {
        let mut i = 0i32;
        while i <= read_u32_as_usize(this + league::CURRENT_STAGE) as i32 {
            let stage = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + i as usize * 4);
            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(vtable_method(stage, 0x14));
            f(stage, data, full);
            i += 1;
        }
    }
    if read_i16(this + league::LAST_RESULT_DATE + 2) == read_i16(G_CURRENT_DATE_YEAR) && read_i16(this + league::LAST_RESULT_DATE) == read_i16(G_CURRENT_DATE) && read_u32_as_usize(this + league::COMPLETE) == 0 {
        write_u32_from_usize(this + league::TABLE_COLUMN, 0);
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, 0x94));
        f(this);
    }
    let mut seguir_tv = true;
    if read_u8(this + league::STAGE_INDEX) == 0xff {
        if read_u32_as_usize(this + league::TABLE) == 0 {
            seguir_tv = false;
        } else if !same_date(this + league::LAST_UPDATE_DATE, data) {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(vtable_method(this, 0x10));
            f(this, 0, 0);
            if read_u32_as_usize(this + league::COMPLETE) == 1 {
                let mut acabadas = true;
                let mut i = 0i32;
                while i <= read_u32_as_usize(this + league::CURRENT_STAGE) as i32 {
                    let stage = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + i as usize * 4);
                    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(vtable_method(stage, 0x10));
                    if g(stage, 0, 1) as u8 == 0 {
                        acabadas = false;
                        break;
                    }
                    i += 1;
                }
                if acabadas {
                    if (read_u32_as_usize(this + league::CURRENT_STAGE) as i32) < (read_u32_as_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, 0x28));
                        h(this);
                    } else if read_u32_as_usize(this + league::SEASON_CLOSED) == 0 {
                        history(read_u32_as_usize(G_CLUB_HISTORY), this);
                        if count(read_u32_as_usize(read_u32_as_usize(this + 4)) as u32) != 0 {
                            recordes(read_u32_as_usize(G_CLUB_RECORDS), this, 1);
                        }
                        if read_u32_as_usize(this + league::TIEBREAK_MATCHES) != 0 {
                            show_error_at(0x9f15f4, 0x74c);
                        }
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, 0x30));
                        h(this);
                        if read_u32_as_usize(this + 8) != 0 {
                            fim_anexo(read_u32_as_usize(this + 8));
                        }
                        write_u32_from_usize(this + league::SEASON_CLOSED, 1);
                    }
                }
            }
            if count(read_u32_as_usize(read_u32_as_usize(this + 4)) as u32) != 0 {
                recordes(read_u32_as_usize(G_CLUB_RECORDS), this, 0);
            }
            if read_u32_as_usize(this + 8) != 0 {
                anexo(read_u32_as_usize(this + 8));
            }
        }
    }
    if seguir_tv && read_u32_as_usize(this + league::TABLE) != 0 {
        if full == 0 {
            return 1;
        }
        if read_u32_as_usize(this + league::COMPLETE) == 0 {
            weeks(pd0, px, 4); // cqd5: a TV quatro semanas adiante
            crate::comparison::branch("cqd_5 jogos para a tv");
            tv(this, px);
        }
    }
    if full == 0 {
        return 1;
    }
    if read_u8(this + league::SQUAD_FLAGS) & 2 != 0 && read_u8(this + league::STAGE_INDEX) == 0xff && !same_date(this + league::LAST_UPDATE_DATE, data)
        && read_u32_as_usize(this + league::COMPLETE) == 0 && read_u8(read_u32_as_usize(this + league::TABLE) + 0x11) == 0 {
        let e = read_u32_as_usize(this + league::ROUND_DATES);
        // simplification: the original pushes the day and the year in registers with leftovers in the high bytes; 0x549ef0 only reads the words
        new(pt, read_i16(e) as u16 as u32, read_i16(e + 2).wrapping_add(read_i16(this + league::YEAR)) as u16 as u32);
        let c = read_u32_as_usize(this + 4);
        let nation = if c != 0 && read_u32_as_usize(c + 0x5d) != 0 { read_u32_as_usize(read_u32_as_usize(c + 0x5d)) } else { 0 };
        let has = c != 0 && read_u32_as_usize(c + 0x5d) != 0;
        let days = if has && nation == read_u32_as_usize(KEY_NATION_ARGENTINA) {
            5
        } else if has && nation == read_u32_as_usize(KEY_NATION_ITALY) {
            0x14
        } else {
            0xe
        };
        let r = more(pd0, pw, days);
        if same_date(pt, r) {
            prazo(read_u32_as_usize(G_SQUAD_MANAGER), this, 0, 0);
        }
    }
    copy(this + league::LAST_UPDATE_DATE, pw, data);
    1
}


/// Fills a league fixture record (clubs, flags, ids, venue, date, slice, last-round bit) from table entries and the TV
/// date entry, and adds it to the fixture manager.
///
/// [pt] Um jogo da tabela da liga no registro `r` (0x4f bytes) e no calendário: os clubes das entradas `ea` (mandante) e
/// `eb` (0x3b bytes na tabela +0xb1: +0 clube, +0x38 marca), as marcas +0x40/+0x41 (marca 0), os ids +0xc/+0x10 (-1 sem
/// clube), o estádio (0x85c260: push +0x24; push &jogo; push [+0x3d da data]; mov ecx, [0xdd7ef4] -> thiscall, ret
/// 0xc), a data (+0x28 ano + o da liga, +0x2a dia) e a fatia (+0x3f) da entrada `off` da tabela das datas (+0xba), os
/// bits 0..1 de +0x4e zerados e o bit 3 na última rodada (+0x34 = +0xa9 − 1; ou com `tipo4`, data do tipo 4), e o jogo
/// vai ao calendário (0x5aa680: push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 8).
pub(crate) unsafe fn add_league_fixture(this: usize, r: usize, ea: usize, eb: usize, off: usize, tipo4: bool) {
    let stadium: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
    let calendario: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
    let (ea, eb) = (ea, eb); // cqd5: o mandante e o visitante
    let (ca, cb) = (read_u32_as_usize(ea), read_u32_as_usize(eb));
    write_u32_from_usize(r + 0x1c, ca);
    write_u32_from_usize(r + 0x20, cb);
    write_u8(r + 0x40, (read_u8(ea + 0x38) == 0) as u8);
    write_u8(r + 0x41, (read_u8(eb + 0x38) == 0) as u8);
    write_u32_from_usize(r + 0xc, if ca != 0 { read_u32_as_usize(ca) } else { u32::MAX as usize });
    write_u32_from_usize(r + 0x10, if cb != 0 { read_u32_as_usize(cb) } else { u32::MAX as usize });
    stadium(read_u32_as_usize(G_STADIUM), read_u32_as_usize(read_u32_as_usize(this + league::ROUND_DATES) + off + 0x3d) as u32, r, read_u32_as_usize(this + league::FIXED_STADIUM_ID) as u32);
    let e = read_u32_as_usize(this + league::ROUND_DATES) + off;
    write_u16(r + 0x28, read_i16(e + 2).wrapping_add(read_i16(this + league::YEAR)) as u16);
    write_u16(r + 0x2a, read_i16(e) as u16);
    write_u8(r + 0x3f, read_u8(e + 4));
    write_u16(r + 0x4d, read_i16(r + 0x4d) as u16 & 0xfcff);
    if read_i16(r + 0x34) as i32 == read_i16(this + league::ROUND_COUNT) as i32 - 1 || (tipo4 && read_u8(e + 0xb) == 4) {
        write_u8(r + 0x4e, read_u8(r + 0x4e) | 8);
    } else {
        write_u16(r + 0x4d, read_i16(r + 0x4d) as u16 & 0xf7ff);
    }
    crate::comparison::branch("cqd_5 jogo da tabela");
    calendario(read_u32_as_usize(G_FIXTURE_MANAGER), r, 1);
}

/// Creates the season fixtures: the round-robin grid (with the club order drawn) or the year's own list, home and away
/// by the parity of the half and the year, the stadium, date and slot of each round, and the calendar; then the TV
/// dates.
///
/// Ret 0; SEH frame with no destructor on the normal path (the local dates): `League::create_fixtures()`, from building
/// the competitions (44 calls up to 2002-06-01). With n = teams (+0x3e) + teams % 2: +0xdd = 0; the grid of n rows of n
/// dwords (0x944cf1; out of memory: the box at line 0x3fc or 0x405 and 0), built by 0x6844b0 (push n; push grid ->
/// stdcall, ret 8: it does not read the mov ecx, this before it) and, outside competitions `[0x9cf79c]` and
/// `[0x9cf7a4]`, the club order drawn (0x686a70: push n; mov ecx, this -> thiscall, ret 4). The match record (0x4f
/// bytes on the stack): +0 competition id, +8 = -1, +0x14 the competition, +0x24/+0x26 the last date of the table
/// (+0xba, entry +0xa9 − 1), +0x2c year (+0x40), +0x30 = 0, +0x32 (+0x3a), +0x36 (+0xdb), +0x38 (+0xab), +0x3a (+0xc4),
/// +0x3b = 0xff, +0x3d/+0x3e = 0, +0x42 stage (+0x44), +0x43..+0x4c = 0xff, and the league's method 0x7c with it. With
/// its own list (+0xea, 0x10-byte entries: +8/+0xc the club ids) for the league year: in each round k (+0xa9) the
/// teams/2 matches of the list (the indices through table +0xb1), and the list is freed (0x9462bb + 0x944c94).
/// Otherwise the round robin: for j = 1..legs (+0x3c)·(n − 1), the round through 0x68a030 (push +0xd9; push +0xa9; push
/// +0x3c; push +0x3e; push id; push &step; push round; add esp, 0x1c -> cdecl) and column j % (n − 1) (n − 1 at zero):
/// each team i with opponent v ≠ 0 in the grid clears the opponent's slot; home advantage by the parity of v > 0, of
/// the half ((j − 1)/(n − 1)) and of year + stage; with nation +0x24, the club of that nation (+0x69) stays at home.
/// Each match goes to the calendar (add_league_fixture). At the end, the grid is freed and the type 3 dates of the
/// table take the matches to TV (0x685640). Returns 1.
/// Original: `League::create_fixtures` (0x6835c0, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x6835c0 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::create_fixtures()`, da montagem das competições (44 chamadas até 2002-06-01). Com n = times (+0x3e) +
/// times % 2: +0xdd = 0; a grade de n linhas de n dwords (0x944cf1; sem memória, a caixa da linha 0x3fc ou 0x405 e 0),
/// montada pela 0x6844b0 (push n; push grade -> stdcall, ret 8: o mov ecx, this antes ela não lê) e, fora das
/// competições [0x9cf79c] e [0x9cf7a4], a ordem dos clubes sorteada (0x686a70: push n; mov ecx, this -> thiscall, ret
/// 4). O registro do jogo (0x4f bytes na pilha): +0 id da competição, +8 = -1, +0x14 a competição, +0x24/+0x26 a última
/// data da tabela (+0xba, entrada +0xa9 − 1), +0x2c ano (+0x40), +0x30 = 0, +0x32 (+0x3a), +0x36 (+0xdb), +0x38
/// (+0xab), +0x3a (+0xc4), +0x3b = 0xff, +0x3d/+0x3e = 0, +0x42 fase (+0x44), +0x43..+0x4c = 0xff, e o método 0x7c da
/// liga com ele. Com a lista própria (+0xea, entradas de 0x10 bytes: +8/+0xc os ids dos clubes) do ano da liga: em cada
/// rodada k (+0xa9) os times/2 jogos da lista (os índices pela tabela +0xb1), e a lista é liberada (0x9462bb +
/// 0x944c94). Senão o rodízio: para j = 1..turnos (+0x3c)·(n − 1), a rodada pela 0x68a030 (push +0xd9; push +0xa9;
/// push +0x3c; push +0x3e; push id; push &passo; push rodada; add esp, 0x1c -> cdecl) e a coluna j % (n − 1) (n − 1 no
/// zero): cada time i com adversário v ≠ 0 na grade apaga a vaga do adversário; o mando pela paridade de v > 0, do turno
/// ((j − 1)/(n − 1)) e do ano + fase; com a nação +0x24, o clube dessa nação (+0x69) fica como mandante. Cada jogo vai
/// ao calendário (add_league_fixture). No fim, a grade é liberada e as datas do tipo 3 da tabela levam os jogos para a TV
/// (0x685640). Devolve 1.
pub(crate) unsafe extern "thiscall" fn create_fixtures(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let build: unsafe extern "stdcall" fn(usize, i32) -> u32 = core::mem::transmute(FN_LEAGUE_BUILD_ROUND_ROBIN_GRID);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_LEAGUE_DRAW_TEAM_ORDER);
    let girar: unsafe extern "C" fn(i32, *mut i8, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_LEAGUE_NEXT_SCHEDULE_ROUND);
    let destroy_list: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_MOVE_FIXTURES_FOR_TV);
    let mut round: i32 = -1;
    let teams = read_i16(this + league::CLUB_COUNT) as i32;
    let n = teams + teams % 2;
    let mut step_towards: i8 = 0;
    let (mut x, mut y, mut z) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pz) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, z.as_mut_ptr() as usize);
    default(px);
    write_u32_from_usize(this + league::COMPLETE, 0);
    let grade = novo((n * 4 + 4) as u32);
    if grade == 0 {
        show_error_at(0x9f15f4, 0x3fc);
        return 0;
    }
    let mut i = 1i32;
    while i <= n {
        let l = novo((n * 4) as u32);
        if l == 0 {
            show_error_at(0x9f15f4, 0x405);
            free(grade);
            return 0;
        }
        write_u32_from_usize(grade + i as usize * 4, l);
        i += 1;
    }
    build(grade, n);
    let c = read_u32_as_usize(this + 4);
    if c == 0 || (read_u32_as_usize(c) != read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) && read_u32_as_usize(c) != read_u32_as_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP)) {
        crate::comparison::branch("cqd_5 ordem sorteada");
        sortear(this, n as u32); // cqd5: a ordem dos clubes
    }
    // the match record; the bytes the original does not write (+4..+7, +0x18, +0x2e, +0x39, +0x4d and the high bits of
    // +0x4e: leftover from its stack, which method 0x7c and 0x85c260 may fill) stay 0, a value the game produces
    let mut reg = [0u8; 0x60];
    let r = reg.as_mut_ptr() as usize;
    let c = read_u32_as_usize(this + 4);
    write_u32_from_usize(r + 0x14, c);
    write_u16(r + 0x32, read_i16(this + league::STAGE_ID) as u16);
    write_u32_from_usize(r, read_u32_as_usize(c));
    write_u16(r + 0x2c, read_i16(this + league::YEAR) as u16);
    write_u16(r + 0x38, read_i16(this + league::MATCH_RULES) as u16);
    write_u16(r + 0x4d, read_i16(r + 0x4d) as u16 & 0xfbff);
    write_u8(r + 0x3a, read_u8(this + league::MATCH_CATEGORY));
    write_u16(r + 0x36, read_i16(this + 0xdb) as u16);
    write_u16(r + 0x30, 0);
    for k in 0x43..=0x4cusize {
        write_u8(r + k, 0xff);
    }
    write_u32_from_usize(r + 8, u32::MAX as usize);
    write_u8(r + 0x42, read_u8(this + league::STAGE_INDEX));
    write_u8(r + 0x3d, 0);
    write_u8(r + 0x3e, 0);
    write_u8(r + 0x3b, 0xff);
    let prepare: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(vtable_method(this, 0x7c));
    prepare(this, r);
    let ult = read_u32_as_usize(this + league::ROUND_DATES).wrapping_add((read_i16(this + league::ROUND_COUNT) as i32 * 0x41) as usize);
    // simplification: the original pushes the day and the year with leftover in the high bytes; 0x549ef0 only reads the words
    let yy = new(py, read_i16(ult - 0x41) as u16 as u32, read_i16(ult - 0x3f).wrapping_add(read_i16(this + league::YEAR)) as u16 as u32);
    copy(px, pz, yy);
    write_u16(r + 0x26, read_i16(px) as u16);
    write_u16(r + 0x24, read_i16(px + 2) as u16);
    let list = read_u32_as_usize(this + league::FIXED_FIXTURES);
    if list != 0 && read_i16(this + league::YEAR) as i32 == read_i16(list + 2) as u16 as i32 {
        let mut k = 0i32;
        let mut off = 0usize;
        while k < read_i16(this + league::ROUND_COUNT) as i32 {
            let mut i = 0i32;
            loop {
                let teams = read_i16(this + league::CLUB_COUNT) as i32;
                let half = teams / 2;
                if i >= half {
                    break;
                }
                write_u16(r + 0x34, k as u16);
                let e = read_u32_as_usize(this + league::FIXED_FIXTURES) + ((half * k + i) as usize) * 0x10;
                let (mut ia, mut ib) = (-1i32, -1i32);
                if teams > 0 {
                    let (a, b) = (read_u32_as_usize(e + 8), read_u32_as_usize(e + 0xc));
                    let tb = read_u32_as_usize(this + league::TABLE);
                    for p in 0..teams {
                        let id = read_u32_as_usize(read_u32_as_usize(tb + p as usize * 0x3b));
                        if a == id {
                            ia = p;
                        }
                        if b == id {
                            ib = p;
                        }
                    }
                }
                let tb = read_u32_as_usize(this + league::TABLE);
                add_league_fixture(this, r, tb.wrapping_add((ia as isize * 0x3b) as usize), tb.wrapping_add((ib as isize * 0x3b) as usize), off, true);
                i += 1;
            }
            k += 1;
            off += 0x41;
        }
        let list = read_u32_as_usize(this + league::FIXED_FIXTURES);
        if list != 0 {
            destroy_list(list, 0x10, read_u32_as_usize(list - 4) as u32, 0x5a53c0);
            free(list - 4);
        }
        write_u32_from_usize(this + league::FIXED_FIXTURES, 0);
    } else {
        let mut j = 1i32;
        while j <= read_i16(this + league::LEGS) as i32 * (n - 1) {
            // simplification: the original pushes the words +0x3e, +0x3c, +0xa9 and +0xd9 with leftovers in the high bytes; 0x68a030
            // only reads the words and the byte
            round = girar(round, &mut step_towards, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32, read_i16(this + league::CLUB_COUNT) as u16 as u32,
                read_i16(this + league::LEGS) as u16 as u32, read_i16(this + league::ROUND_COUNT) as u16 as u32, read_i16(this + league::LEAGUE_FLAGS) as u16 as u32);
            let mut col = j % (n - 1);
            if col == 0 {
                col = n - 1;
            }
            write_u16(r + 0x34, round as u16);
            if read_i16(this + league::CLUB_COUNT) >= 1 {
                let mut i = 0i32;
                loop {
                    let v = read_u32_as_usize(read_u32_as_usize(grade + 4 + i as usize * 4) + col as usize * 4) as i32;
                    if v != 0 {
                        let turno = ((j - 1) / (n - 1)) % 2 != 0;
                        let paridade = (read_i16(this + league::YEAR) as i32 + read_u8(this + league::STAGE_INDEX) as i8 as i32) % 2 != 0;
                        let idx = if v > 0 { v - 1 } else { -1 - v };
                        let row = read_u32_as_usize(grade + 4 + (idx as isize * 4) as usize);
                        let (a, b) = if (v > 0) ^ turno ^ paridade { (idx, i) } else { (i, idx) };
                        write_u32_from_usize(row + col as usize * 4, 0);
                        let ult = read_i16(this + league::CLUB_COUNT) as i32 - 1;
                        if a >= 0 && a <= ult && b >= 0 && b <= ult {
                            let tb = read_u32_as_usize(this + league::TABLE);
                            let (ea, eb) = (tb + a as usize * 0x3b, tb + b as usize * 0x3b);
                            let sel = read_u32_as_usize(this + league::FIXED_STADIUM_ID) as u32;
                            let (m, f) = if sel != u32::MAX {
                                let nation = read_u32_as_usize(read_u32_as_usize(eb) + 0x69);
                                if nation != 0 && read_u32_as_usize(nation) as u32 == sel { (eb, ea) } else { (ea, eb) }
                            } else {
                                (ea, eb)
                            };
                            add_league_fixture(this, r, m, f, (round as isize * 0x41) as usize, false);
                        }
                    }
                    i += 1;
                    if i + 1 > read_i16(this + league::CLUB_COUNT) as i32 {
                        break;
                    }
                }
            }
            j += 1;
        }
    }
    let mut i = 0i32;
    while i < n {
        free(read_u32_as_usize(grade + 4 + i as usize * 4));
        i += 1;
    }
    free(grade);
    let mut k = 0i32;
    while k < read_i16(this + league::ROUND_COUNT) as i32 {
        let e = read_u32_as_usize(this + league::ROUND_DATES) + k as usize * 0x41;
        if read_u8(e + 0xb) == 3 {
            new(py, read_i16(e) as u16 as u32, read_i16(e + 2).wrapping_add(read_i16(this + league::YEAR)) as u16 as u32);
            tv(this, py);
        }
        k += 1;
    }
    1
}


/// Promotes and relegates between two leagues: the non-promotion clauses, the relegated clubs go down (relegation
/// clauses, human manager sacked in a competition with no active record) and the promoted clubs go up, in pairs or all;
/// the box if the counts differ from the expected ones.
///
/// Ret 0x18: `league_promote_and_relegate(upper, lower, contracts, in_pairs, going_up, going_down)`, from the end of
/// the leagues (after 2002-06-01). It receives the caller's ecx and passes it to 0x6830b0, which does not read it:
/// thiscall here to keep it. With `contracts`, the clubs of the lower league that do not go up (+0x37 ≠ 0 and ≠ 5)
/// trigger the non-promotion clauses (0x4cced0: push league; push club; mov ecx, `[0xae1050]` -> thiscall, ret 8). In
/// pairs: each relegated club from above (+0x37 = 3) swaps with the first one below that goes up (+0x37 = 0 or 5): that
/// one goes up (0x6830b0: push 0; push upper competition; push club; caller's ecx -> thiscall, ret 0xc), the relegated
/// one keeps the old competition (+0x5b) and takes the lower one (+0x57), triggers the relegation clauses (0x4cd030:
/// push old; push club; mov ecx, `[0xae1050]` -> thiscall, ret 8) and, in a competition with no active record and with
/// a human manager (0x600a50), loses the manager (0x69b790: push 0; push 7; push 1; push club; mov ecx, `[0xb67a44]` ->
/// thiscall, ret 0x10); both end with +0x37 = 0xff. Otherwise, all relegated clubs go down this way and then all the
/// lower ones that go up go up. Counts that differ from the expected ones (0xff = any) give the box at line 0x15dc
/// (going up) or 0x15e2 (going down).
/// Original: `league_promote_and_relegate` (0x689c80, league.cpp, thiscall (ecx repassado), ret 0x18).
///
/// [pt] { 0x689c80 } (ret 0x18, league.cpp): `league_promote_and_relegate(acima, abaixo, contratos, aos_pares, sobem,
/// descem)`, do fim das ligas (depois de 2002-06-01). Recebe o ecx de quem chama e o repassa à 0x6830b0, que não o lê:
/// thiscall aqui para o guardar. Com `contratos`, os clubes da liga de baixo que não sobem (+0x37 ≠ 0 e ≠ 5) acionam
/// as cláusulas de não promoção (0x4cced0: push liga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8). Aos pares:
/// cada rebaixado de cima (+0x37 = 3) troca com o primeiro de baixo que sobe (+0x37 = 0 ou 5): esse sobe (0x6830b0:
/// push 0; push competição de cima; push clube; ecx de quem chama -> thiscall, ret 0xc), o rebaixado guarda a
/// competição antiga (+0x5b) e fica com a de baixo (+0x57), aciona as cláusulas de rebaixamento (0x4cd030: push
/// antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8) e, numa competição sem registro ativo e com técnico
/// humano (0x600a50), perde o técnico (0x69b790: push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall,
/// ret 0x10); os dois ficam com +0x37 = 0xff. Senão, todos os rebaixados descem assim e depois todos os de baixo que
/// sobem sobem. Os contados que diferem dos esperados (0xff = qualquer) dão a caixa da linha 0x15dc (sobem) ou 0x15e2
/// (descem).
pub(crate) unsafe extern "thiscall" fn league_promote_and_relegate(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pairs: u32, sobem: u32, descem: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_TRIGGER_NON_PROMOTION_CLAUSES);
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_LEAGUE_MOVE_CLUB_TO_DIVISION);
    let relegation: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_TRIGGER_RELEGATION_CLAUSES);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let without_manager: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_CLUB_MANAGER);
    let (mut desceram, mut subiram) = (0u8, 0u8);
    if contratos != 0 {
        let mut i = 0i32;
        while i < read_i16(abaixo + league::CLUB_COUNT) as i32 {
            let e = read_u32_as_usize(abaixo + league::TABLE) + i as usize * 0x3b;
            let s = read_u8(e + 0x37);
            if s != 5 && s != 0 {
                clausulas(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(e), read_u32_as_usize(abaixo + 4));
            }
            i += 1;
        }
    }
    // the relegated club goes down to the lower competition and loses the human manager in a competition with no active registration
    let descer = |club: usize| {
        let comp = read_u32_as_usize(abaixo + 4);
        let antiga = read_u32_as_usize(club + club::DIVISION);
        write_u32_from_usize(club + club::LAST_DIVISION, antiga);
        write_u32_from_usize(club + club::DIVISION, comp);
        relegation(read_u32_as_usize(G_CONTRACT_MANAGER), club, antiga);
        let table = read_u32_as_usize(G_COMP_OBJECT_TABLE);
        if table == 0 || comp == 0 || read_u32_as_usize(table + read_u32_as_usize(comp) * 4) == 0 {
            if human(club, 1, 1, 0, 0) != 0 {
                without_manager(read_u32_as_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
            }
        }
    };
    if aos_pairs != 0 {
        let mut i = 0i32;
        while i < read_i16(acima + league::CLUB_COUNT) as i32 {
            let ue = read_u32_as_usize(acima + league::TABLE) + i as usize * 0x3b;
            if read_u8(ue + 0x37) == 3 {
                let n = read_i16(abaixo + league::CLUB_COUNT) as i32;
                let mut j = 0i32;
                while j < n {
                    let s = read_u8(read_u32_as_usize(abaixo + league::TABLE) + j as usize * 0x3b + 0x37);
                    if s == 0 || s == 5 {
                        break;
                    }
                    j += 1;
                }
                if j < n {
                    let le = read_u32_as_usize(abaixo + league::TABLE) + j as usize * 0x3b;
                    subir(ecx, read_u32_as_usize(le), read_u32_as_usize(acima + 4), 0);
                    descer(read_u32_as_usize(ue));
                    desceram = desceram.wrapping_add(1);
                    subiram = subiram.wrapping_add(1);
                    write_u8(le + 0x37, 0xff);
                    write_u8(ue + 0x37, 0xff);
                }
            }
            i += 1;
        }
    } else {
        let mut i = 0i32;
        while i < read_i16(acima + league::CLUB_COUNT) as i32 {
            let ue = read_u32_as_usize(acima + league::TABLE) + i as usize * 0x3b;
            if read_u8(ue + 0x37) == 3 {
                descer(read_u32_as_usize(ue));
                desceram = desceram.wrapping_add(1);
                write_u8(ue + 0x37, 0xff);
            }
            i += 1;
        }
        let mut i = 0i32;
        while i < read_i16(abaixo + league::CLUB_COUNT) as i32 {
            let le = read_u32_as_usize(abaixo + league::TABLE) + i as usize * 0x3b;
            let s = read_u8(le + 0x37);
            if s == 0 || s == 5 {
                subir(ecx, read_u32_as_usize(le), read_u32_as_usize(acima + 4), 0);
                subiram = subiram.wrapping_add(1);
                write_u8(le + 0x37, 0xff);
            }
            i += 1;
        }
    }
    if sobem as u8 != 0xff && subiram != sobem as u8 {
        show_error_at(0x9f15f4, 0x15dc);
    }
    if descem as u8 != 0xff && desceram != descem as u8 {
        show_error_at(0x9f15f4, 0x15e2);
    }
}







/// No ensaio (o comparador da 0x684cd0) a cópia não chama as virtuais +0x40 (o anúncio da situação, que manda notícia
/// e grava os registros) e +0x68 (a situação nas estatísticas) nem as cláusulas de rebaixamento (0x4cd030).
pub(crate) static mut POSITIONS_STATUS_DRY_RUN: bool = false;

/// A fita do comparador da 0x684cd0: a original roda primeiro e cada anúncio dela (+0x40 = 0x686670, pelo comparador
/// dela) guarda o clube, a situação e os bytes que a virtual +0x44 da competição gravou nas regiões (a BraNatFirst
/// 0x42abc0, a BraNatSecond 0x42d040, a BelThird 0x4230d0 e a FinFirst 0x594810 põem 5 ou 0xfe no +0x37 da linha).
/// No ensaio, o anúncio da cópia que bater com o próximo da fita repõe esses bytes.
pub(crate) static mut STATUS_ANNOUNCE_RECORD: Option<Vec<(usize, usize)>> = None;
pub(crate) static mut STATUS_ANNOUNCE_TAPE: Vec<(usize, u8, Vec<(usize, u8)>)> = Vec::new();
pub(crate) static mut STATUS_ANNOUNCE_TAPE_POS: usize = 0;

/// Returns row i of the league table (+0xb1, 0x3b bytes).
///
/// [pt] A linha `i` da tabela (+0xb1, 0x3b bytes).
pub(crate) unsafe fn league_table_row(league: usize, i: i32) -> usize {
    read_usize(league + league::TABLE).wrapping_add((i as usize).wrapping_mul(0x3b))
}

/// Calls the league's virtual +0x40 (announce_team_status) for the club of row i with the situation and stage; in
/// rehearsal replays the recorded tape.
///
/// [pt] A virtual +0x40 da liga (League::announce_team_status: thiscall(liga, clube, situação, fase, 0, 0), ret 0x14), com
/// o clube da linha `i`. `alto` = os bits de cima do registrador que a original empurra com a fase (+0x44) no byte de
/// baixo (o ponteiro da linha ou a devolução das cláusulas: reproduzidos).
pub(crate) unsafe fn league_announce_team_status(league: usize, i: i32, status: u32, high: u32) {
    if POSITIONS_STATUS_DRY_RUN {
        let tape_lock = &*core::ptr::addr_of!(STATUS_ANNOUNCE_TAPE);
        if let Some((clube, s, bytes)) = tape_lock.get(STATUS_ANNOUNCE_TAPE_POS) {
            STATUS_ANNOUNCE_TAPE_POS += 1;
            if *clube == read_usize(league_table_row(league, i)) && *s == status as u8 {
                for &(a, v) in bytes {
                    *(a as *mut u8) = v;
                }
            }
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(league) + 0x40));
    f(league, read_usize(league_table_row(league, i)), status, high | read_u8(league + league::STAGE_INDEX) as u32, 0, 0);
}

/// Writes status v at +0x37 of table row i and announces it.
///
/// [pt] Grava a situação `v` no +0x37 da linha `i` e a anuncia.
pub(crate) unsafe fn set_row_status(league: usize, i: i32, v: u8, high: u32) {
    *((league_table_row(league, i) + 0x37) as *mut u8) = v;
    league_announce_team_status(league, i, v as i8 as i32 as u32, high);
}

/// Maximum reachable points of a table row: (season games +0xa7 − played +0x11) × win points (+0xc2) + points (+0x19).
///
/// [pt] O máximo de pontos da linha: (jogos da temporada +0xa7 − jogos feitos +0x11) × pontos da vitória (+0xc2) + pontos
/// (+0x19).
pub(crate) unsafe fn max_points(league: usize, l: usize) -> i32 {
    ((read_i16(league + league::GAMES_PER_CLUB) as i32) - (read_i8(l + 0x11) as i32)).wrapping_mul(read_i8(league + league::WIN_POINTS) as i32).wrapping_add(read_i16(l + 0x19) as i32)
}

/// Sorts the table (0x6827d0), marks the bands (promotion, play-offs, 2 and relegation) from the top and the bottom,
/// works out the best (+0x39) and worst (+0x3a) possible position from the maximum points and decides the status
/// (+0x37): title (5), promotion (0, 7 or 6), play-offs (1), relegation (3, with the clauses 0x4cd030), 2 and 0xfe,
/// each announced (+0x40); the European place (`[0xae2d14]`) from the nation places (0x905c60); the stats progress
/// (+0x68).
///
/// Ret 0: `League::update_positions_and_status()`, virtual +0x94 of the league (from the day pass 0x684946). +0xa7 (the
/// season's matches) = (+0x3e − 1) × +0x3c (16 bits) if still −1, with rows and +0x45. With +0xdd, nothing more.
/// Otherwise sorts the table (League::sort_table 0x6827d0, thiscall(league, 0), ret 4) and builds the band map (malloc
/// 0x944e46 of +0x3e bytes set to 0xff; out of memory: the box at line 0x7c9): from the top, counts +0xbe (band 0,
/// promotion) and +0xbf (band 1, play-offs) on the rows not in 0xfe, 0xfc or 3; from the bottom, +0xc1 (band 3,
/// relegation) and +0xc0 (band 2) on those not in 0, 1 or 5. With +0xd9 bit 0, the first in the table outside 0xfe,
/// 0xfc and 3. The best position (+0x39) and the worst (+0x3a) of each row not yet decided, from the maximum points and
/// the matches played (with +0xe6 on a tie among those that finished). Then, row by row: worst = first + 1, the title
/// (5); band 0 at the worst, promotion (0 with +0xd9 bit 1; 7 with bit 2; 6 with bit 3) for those in 0xff or 1; band 1,
/// play-offs (1) for those in 0xff; band 3 at the best, the relegation clauses
/// (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, thiscall(`[0xae1050]`, club, +4), ret 8) and relegation (3)
/// for those in 0xff, 2 or 0xfc; band 2 at the best, 2 for those in 0xff or 0xfe. Each new status is announced (+0x40).
/// With +0xd9 bit 0x20, those between the bands with no band in the middle and in 0xff become 0xfe (announced with −2).
/// Best = worst below 9, top league (+0x44 = 0xff) that counts (0x4b0700, cdecl(id)), without +0x1c and a club with no
/// European place (`[0xae2d14]` + 6 × id = −1) from a European nation (`[0x9cfa10]`) outside `[0x9cf45c]`: the nation's
/// places (UefaSeeding::get_nation_places 0x905c60, thiscall(`[0xde1f58]`, nation, &cl, &uefa), ret 0xc) give the
/// Champions League (`[0x9cf6e8]`) or the UEFA Cup (`[0x9cf6f0]`). Finally virtual +0x68 (the status in the stats:
/// club, +0x44, position, best, worst) and the free (0x9452ca) of the map.
/// Original: `League::update_positions_and_status` (0x684cd0, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x684cd0 } (thiscall, ret 0, league.cpp): `League::update_positions_and_status()`, a virtual +0x94 da liga (do
/// passe do dia 0x684946). +0xa7 (os jogos da temporada) = (+0x3e − 1) × +0x3c (16 bits) se ainda −1, com linhas e
/// +0x45. Com +0xdd nada mais. Senão ordena a tabela (League::sort_table 0x6827d0, thiscall(liga, 0), ret 4) e monta
/// o mapa das faixas (malloc 0x944e46 de +0x3e bytes em 0xff; sem memória: a caixa da linha 0x7c9): de cima, as
/// contas +0xbe (faixa 0, acesso) e +0xbf (faixa 1, mata-mata) nas linhas que não estão em 0xfe, 0xfc ou 3; de baixo,
/// +0xc1 (faixa 3, rebaixamento) e +0xc0 (faixa 2) nas que não estão em 0, 1 ou 5. Com +0xd9 bit 0, o primeiro da
/// tabela fora de 0xfe, 0xfc e 3. A melhor posição (+0x39) e a pior (+0x3a) de cada linha que ainda não estão
/// decididas, pelos máximos de pontos e pelos jogos feitos (com +0xe6 no empate dos que terminaram). Depois, linha a
/// linha: a pior = o primeiro + 1, o título (5); faixa 0 na pior, o acesso (0 com +0xd9 bit 1; 7 com bit 2; 6 com bit
/// 3) em quem está em 0xff ou 1; faixa 1, o mata-mata (1) em quem está em 0xff; faixa 3 na melhor, as cláusulas de
/// rebaixamento (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, thiscall([0xae1050], clube, +4), ret 8) e o
/// rebaixamento (3) em quem está em 0xff, 2 ou 0xfc; faixa 2 na melhor, 2 em quem está em 0xff ou 0xfe. Cada situação
/// nova é anunciada (+0x40). Com +0xd9 bit 0x20, quem fica entre as faixas sem faixa no meio e está em 0xff vira 0xfe
/// (anunciado com −2). Melhor = pior abaixo de 9, liga de cima (+0x44 = 0xff) que conta (0x4b0700, cdecl(id)), sem
/// +0x1c e clube sem vaga europeia ([0xae2d14] + 6 × id = −1) de nação europeia ([0x9cfa10]) fora de [0x9cf45c]: as
/// vagas da nação (UefaSeeding::get_nation_places 0x905c60, thiscall([0xde1f58], nação, &cl, &uefa), ret 0xc) dão a
/// Liga dos Campeões ([0x9cf6e8]) ou a Copa da UEFA ([0x9cf6f0]). Por fim a virtual +0x68 (a situação nas
/// estatísticas: clube, +0x44, posição, melhor, pior) e o free (0x9452ca) do mapa.
pub(crate) unsafe extern "thiscall" fn update_positions_and_status(this: usize) {
    let n0 = read_i16(this + league::CLUB_COUNT);
    if read_i16(this + league::GAMES_PER_CLUB) == -1 && n0 > 0 && read_usize(this + league::ACTIVE) != 0 {
        *((this + league::GAMES_PER_CLUB) as *mut i16) = (n0 - 1).wrapping_mul(read_i16(this + league::LEGS));
    }
    if read_usize(this + league::COMPLETE) != 0 {
        return;
    }
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    sort(this, 0);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let mapa = alloc(read_i16(this + league::CLUB_COUNT) as i32 as u32);
    if mapa == 0 {
        return show_error_at(0x9f15f4, 0x7c9);
    }
    let n = read_i16(this + league::CLUB_COUNT);
    core::ptr::write_bytes(mapa as *mut u8, 0xff, n as i32 as usize);
    let m = |i: i32| (mapa as isize + i as isize) as *mut u8;
    let mut contas = [read_u8(this + league::PROMOTED_COUNT), read_u8(this + league::PROMOTION_PLAYOFF_COUNT), read_u8(this + league::RELEGATION_PLAYOFF_COUNT), read_u8(this + league::RELEGATED_COUNT)];
    let mut first: i8 = -1;
    if read_u8(this + league::LEAGUE_FLAGS) & 1 != 0 {
        let mut i: i16 = 0;
        while i < n {
            let s = read_u8(league_table_row(this, i as i32) + 0x37);
            if s != 0xfe && s != 0xfc && s != 3 {
                first = i as i8;
                break;
            }
            i += 1;
        }
    }
    // upper: bands 0 and 1
    let mut i: i32 = 0;
    if n > 0 {
        let mut k: i8 = 0;
        while k <= 1 {
            if contas[k as usize] != 0 {
                while (i as i16) < n {
                    let s = read_u8(league_table_row(this, i as i16 as i32) + 0x37);
                    if s != 0xfe && s != 0xfc && s != 3 {
                        *m(i as i16 as i32) = k as u8;
                        contas[k as usize] = contas[k as usize].wrapping_sub(1);
                    }
                    i += 1;
                    if contas[k as usize] == 0 {
                        break;
                    }
                }
            }
            k += 1;
            if (i as i16) >= n {
                break;
            }
        }
    }
    // from below: bands 3 and 2
    let mut i: i32 = (n as i32) - 1;
    let mut k: i8 = 3;
    while (i as i16) >= 0 && k >= 2 {
        if contas[k as usize] != 0 {
            while (i as i16) >= 0 {
                let s = read_u8(league_table_row(this, i as i16 as i32) + 0x37);
                if s != 0 && s != 1 && s != 5 {
                    *m(i as i16 as i32) = k as u8;
                    contas[k as usize] = contas[k as usize].wrapping_sub(1);
                }
                i -= 1;
                if contas[k as usize] == 0 {
                    break;
                }
            }
        }
        k -= 1;
    }
    // the best (+0x39) and the worst (+0x3a) possible position
    let mut i: i32 = 0;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        let li = league_table_row(this, i as i16 as i32);
        if read_u8(li + 0x39) != read_u8(li + 0x3a) {
            let pos = (i as u8).wrapping_add(1);
            *((li + 0x39) as *mut u8) = pos;
            // the upper ones (j < i): the best drops one for each one that i’s maximum reaches
            let max_i = max_points(this, li);
            let mut j: i32 = 0;
            while (j as i16) < (i as i16) {
                let lj = league_table_row(this, j);
                let games = read_i16(this + league::GAMES_PER_CLUB);
                let end_i = (read_i8(li + 0x11) as i16) >= games;
                let end_j = (read_i8(lj + 0x11) as i16) >= games;
                let pj = read_i16(lj + 0x19) as i32;
                let cai = if !end_i && !end_j {
                    max_i >= pj
                } else if !end_j {
                    max_i == pj
                } else if !end_i {
                    max_i >= pj
                } else {
                    max_i == pj && read_usize(this + league::TIEBREAK_MATCHES) != 0
                };
                if cai {
                    *((li + 0x39) as *mut u8) = read_u8(li + 0x39).wrapping_sub(1);
                }
                j += 1;
            }
            // the lower ones (j > i): the worst moves up one for each whose maximum reaches i
            *((li + 0x3a) as *mut u8) = pos;
            let mut j: i32 = pos as i8 as i32;
            while (j as i16) < read_i16(this + league::CLUB_COUNT) {
                let lj = league_table_row(this, j as i16 as i32);
                let games = read_i16(this + league::GAMES_PER_CLUB);
                let pj = read_i16(lj + 0x19) as i32;
                let max_j = max_points(this, lj);
                let end_i = (read_i8(li + 0x11) as i16) >= games;
                let end_j = (read_i8(lj + 0x11) as i16) >= games;
                let pi = read_i16(li + 0x19) as i32;
                let sobe = if !end_i && !end_j {
                    max_j >= pi
                } else if !end_j {
                    max_j >= pi
                } else if !end_i {
                    max_j == pi
                } else {
                    max_j == pj && read_usize(this + league::TIEBREAK_MATCHES) != 0
                };
                if sobe {
                    *((li + 0x3a) as *mut u8) = read_u8(li + 0x3a).wrapping_add(1);
                }
                j += 1;
            }
        }
        i += 1;
    }
    // the situation of each row
    let one_after = (first as i32) + 1;
    let mut i: i32 = 0;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        let ii = i as i16 as i32;
        let li = league_table_row(this, ii);
        let high_row = (li as u32) & !0xff;
        let worst = read_i8(li + 0x3a) as i32;
        let s = read_u8(li + 0x37);
        if worst == one_after {
            if s != 3 && s != 0xfe && s != 0xfc && s != 5 {
                crate::comparison::branch("cpz_3 titulo");
                set_row_status(this, ii, 5, high_row); // cpz3: o título na pior posição = o primeiro + 1
            }
        } else {
            let f = *m(worst - 1);
            if f == 0 {
                if s == 0xff || s == 1 {
                    let d9 = read_u8(this + league::LEAGUE_FLAGS);
                    if d9 & 2 != 0 {
                        if s != 0 {
                            set_row_status(this, ii, 0, high_row);
                        }
                    } else if d9 & 4 != 0 {
                        if s != 7 {
                            set_row_status(this, ii, 7, high_row);
                        }
                    } else if d9 & 8 != 0 {
                        if s != 6 {
                            set_row_status(this, ii, 6, high_row);
                        }
                    }
                }
            } else if f == 1 {
                if s == 0xff {
                    crate::comparison::branch("cpz_3 vaga no mata-mata");
                    set_row_status(this, ii, 1, high_row); // cpz3: a vaga no mata-mata (situação 1)
                }
            } else {
                let g = *m(read_i8(li + 0x39) as i32 - 1);
                if g == 3 {
                    let mut high = high_row;
                    if !POSITIONS_STATUS_DRY_RUN {
                        crate::comparison::branch("cpz_3 clausulas de rebaixamento");
                        let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TRIGGER_RELEGATION_CLAUSES);
                        high = clausulas(read_usize(G_CONTRACT_MANAGER), read_usize(li), read_usize(this + 4)) & !0xff; // cpz3: as cláusulas de rebaixamento
                    }
                    let li = league_table_row(this, ii);
                    let s = read_u8(li + 0x37);
                    if s == 0xff || s == 2 || s == 0xfc {
                        set_row_status(this, ii, 3, high);
                    }
                } else if g == 2 {
                    if s == 0xff || s == 0xfe {
                        set_row_status(this, ii, 2, high_row);
                    }
                }
            }
        }
        // between bands, no band in the middle: 0xfe
        if read_u8(this + league::LEAGUE_FLAGS) & 0x20 != 0 {
            let li = league_table_row(this, ii);
            let c0 = read_i8(this + league::PROMOTED_COUNT) as i32;
            let c2 = read_i8(this + league::RELEGATION_PLAYOFF_COUNT) as i32;
            let best = read_i8(li + 0x39);
            if (best as i32) > c0 + c2 {
                let c3 = read_i8(this + league::RELEGATED_COUNT) as i32;
                let worst = read_i8(li + 0x3a);
                if (worst as i32) <= (read_i16(this + league::CLUB_COUNT) as i32) - c3 - c2 {
                    let mut livre = true;
                    let mut a: i16 = (best as i16) - 1;
                    while a < worst as i16 {
                        if !livre {
                            break;
                        }
                        if *m(a as i32) != 0xff {
                            livre = false;
                        }
                        a += 1;
                    }
                    if livre && read_u8(li + 0x37) == 0xff {
                        *((li + 0x37) as *mut u8) = 0xfe;
                        league_announce_team_status(this, ii, 0xffff_fffe, (li as u32) & 0xffff_0000);
                    }
                }
            }
        }
        // the European place of those whose position is already decided
        let li = league_table_row(this, ii);
        let best = read_i8(li + 0x39);
        if best == read_i8(li + 0x3a) && best < 9 && read_u8(this + league::STAGE_INDEX) == 0xff {
            let count: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
            if count(read_i32(read_usize(this + 4))) != 0 && read_i32(this + league::UPPER_DIVISION_ID) == -1 {
                let slot = read_usize(G_CLUB_STATE_TABLE) + (read_i32(read_usize(li)) as usize).wrapping_mul(6);
                let reg = read_usize(this + 4);
                let nation = read_usize(reg + 0x5d);
                if read_i32(slot) == -1 && nation != 0 && read_i32(nation) != read_i32(KEY_NATION_SCOTLAND) {
                    let cont = read_usize(nation + nation::CONTINENT);
                    if cont != 0 && read_i32(cont) == read_i32(KEY_CONTINENT_EUROPE) {
                        let (mut cl, mut uefa) = (0i8, 0i8);
                        let slots: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(FN_UEFASEEDING_GET_NATION_PLACES);
                        slots(read_usize(G_UEFA_SEEDING), nation, &mut cl, &mut uefa);
                        let li = league_table_row(this, ii);
                        let best = read_i8(li + 0x39) as i32;
                        let slot = read_usize(G_CLUB_STATE_TABLE) + (read_i32(read_usize(li)) as usize).wrapping_mul(6);
                        if best <= cl as i32 {
                            core::ptr::write_unaligned(slot as *mut i32, read_i32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE));
                        } else if best <= (cl as i32) + (uefa as i32) - 1 {
                            core::ptr::write_unaligned(slot as *mut i32, read_i32(KEY_CLUB_COMP_UEFA_CUP));
                        }
                    }
                }
            }
        }
        // the situation in the statistics (virtual +0x68)
        if !POSITIONS_STATUS_DRY_RUN {
            let li = league_table_row(this, ii);
            // simplification: the original pushes the worst, the best, the position and the stage in registers with garbage in the bits of
            // top; the +0x68 ones (0x573260, 0x578660, Comp::set_club_progress 0x48e380) only read the byte.
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + 0x68));
            f(this, read_usize(li), read_u8(this + league::STAGE_INDEX) as u32, (i as u8).wrapping_add(1) as u32, read_u8(li + 0x39) as u32, read_u8(li + 0x3a) as u32);
        }
        i += 1;
    }
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(mapa);
}

/// Memory regions 0x684cd0 writes: the whole table, +0xa7 and each club's European slot in G_CLUB_STATE_TABLE.
///
/// [pt] As regiões que a 0x684cd0 grava: a tabela inteira (+0xb1, +0x3e linhas de 0x3b bytes), +0xa7 e a vaga europeia
/// ([0xae2d14] + 6 × id, 4 bytes) de cada clube da tabela.
pub(crate) unsafe fn table_status_regions(this: usize) -> Vec<(usize, usize)> {
    let n = read_i16(this + league::CLUB_COUNT).max(0) as usize;
    let mut v = vec![(this + league::GAMES_PER_CLUB, 2)];
    let table = read_usize(this + league::TABLE);
    if n > 0 && table != 0 {
        v.push((table, n * 0x3b));
        let slots = read_usize(G_CLUB_STATE_TABLE);
        if slots != 0 {
            for i in 0..n {
                let c = read_usize(table + i * 0x3b);
                if c != 0 {
                    let a = slots + (read_i32(c) as usize).wrapping_mul(6);
                    if !v.iter().any(|r| r.0 == a) {
                        v.push((a, 4));
                    }
                }
            }
        }
    }
    v
}


/// Advances a league with knockouts to the next stage (+0x30 + 1), calling the knockout routine when it reaches 0;
/// warns at the last stage.
///
/// [pt] A fase seguinte das ligas com mata-mata (o corpo de 0x572ff0 e 0x5769e0): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal) a caixa da linha `linha`; senão +0x30 + 1 e, se chegou a 0, o mata-mata (`mata_mata`, thiscall(this)).
pub(crate) unsafe fn advance_league_stage(this: usize, file: usize, row: u32, knockout_knockout: usize, name: &'static str) {
    let f = read_i32(this + league::CURRENT_STAGE);
    if f >= read_i32(this + league::STAGE_COUNT).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, f);
    if f == 0 {
        crate::comparison::branch(name);
        let create: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout_knockout);
        create(this);
    }
}



/// Comparer for the stage-advance routines: side by side on +0x30 when the stage does not reach 0; only the original at
/// the last stage or when the knockout is created.
///
/// [pt] O comparador das duas: na fase que não chega a 0 (só +0x30 muda) a original e a cópia lado a lado em +0x30; na
/// última fase (a caixa) e na que chega a 0 (o mata-mata aloca a fase): só a original.
pub(crate) unsafe fn compare_stage_advance(name: &str, this: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    let f = read_i32(this + league::CURRENT_STAGE);
    if f >= read_i32(this + league::STAGE_COUNT).wrapping_sub(1) || f.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + league::CURRENT_STAGE, 4)], original, copy);
    SCOREBOARD.record(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this, mo, mr));
}







/// O ensaio do comparador da 0x682e10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca e não pode rodar duas
/// vezes).
pub(crate) static mut LEAGUE_PRIZE_MONEY_DRY_RUN: bool = false;

/// Prize divisor 1 + 2 + … + n, truncated to 16 bits like the original.
///
/// [pt] O divisor dos prêmios: 1 + 2 + … + n (16 bits, como o ax da original).
pub(crate) unsafe fn sum_of_positions(n: i16) -> i32 {
    let (mut s, mut k) = (0i32, 0i32);
    if n > 0 {
        loop {
            s = s.wrapping_add(k).wrapping_add(1);
            k += 1;
            if k as i16 >= n {
                break;
            }
        }
    }
    s as i16 as i32
}

/// Pays the prize money: divides the prize fund (+0x28) by 1 + 2 + ... + n and pays (n − i) shares to the club in table
/// row i (0x5999a0 on the finance record), with the prize news item above 1000; with +0xe1 = 3 the competition pays
/// through virtual +0x6c; with +0xe1 = 2, nothing.
///
/// Ret 0: `League::pay_prize_money()`: with +0xe1 = 2, nothing. With +0xe1 = 3, the competition (`[0xadadfc]` by the id
/// of +4) pays through virtual +0x6c (without the competition table: the box at line 0x1df). Otherwise the fund (+0x28)
/// divided by 1 + 2 + … + n (n = +0x3e table rows) is the share; the club of row i (+0xb1, 0x3b each) receives (n − i)
/// shares in the finance record (0x5a0590; 0x5999a0 adds to the balance, in +0xa8 and +0x148) and, above 1000, the
/// prize news (0x48e3e0 with position i + 1).
/// Original: `League::pay_prize_money` (0x682e10, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x682e10 } (thiscall, ret 0, league.cpp): `League::pay_prize_money()`: com +0xe1 = 2, nada. Com +0xe1 = 3, a
/// competição ([0xadadfc] pelo id de +4) paga pela virtual +0x6c (sem a tabela das competições: a caixa da linha
/// 0x1df). Senão o fundo (+0x28) dividido por 1 + 2 + … + n (n = +0x3e linhas da tabela) é a parte; o clube da linha i
/// (+0xb1, 0x3b cada) recebe (n − i) partes no registro de finanças (0x5a0590; 0x5999a0 soma no saldo, em +0xa8 e
/// +0x148) e, acima de 1000, a notícia do prêmio (0x48e3e0 com a posição i + 1).
pub(crate) unsafe extern "thiscall" fn pay_prize_money(league: usize) {
    let e1 = read_u8(league + league::PRIZE_MODE);
    if e1 == 2 {
        return;
    }
    if e1 == 3 {
        let t = read_usize(G_COMP_OBJECT_TABLE);
        if t == 0 {
            return show_error_at(0x9f15f4, 0x1df);
        }
        let comp = read_usize(t.wrapping_add(read_usize(read_usize(league + 4)).wrapping_mul(4)));
        let pay: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(comp) + 0x6c));
        return pay(comp, league);
    }
    let s = sum_of_positions(read_i16(league + league::CLUB_COUNT));
    let bottom = read_usize(league + league::PRIZE_FUND) as i32;
    if s == 0 || (bottom == i32::MIN && s == -1) {
        // simplification: the original divides by zero (or overflows) and crashes; original only, to crash the same way
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_PRIZE_MONEY)(league);
    }
    let parte = bottom / s;
    if read_i16(league + league::CLUB_COUNT) <= 0 {
        return;
    }
    let add: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_PRIZE_MONEY);
    let news: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    let mut i: i16 = 0;
    loop {
        let value = (read_i16(league + league::CLUB_COUNT) as i32).wrapping_sub(i as i32).wrapping_mul(parte);
        let club = read_usize(read_usize(league + league::TABLE).wrapping_add(0x3b * i as usize));
        let fin = european_cup_club_finance(club);
        crate::comparison::branch("cpx_11 premio pago");
        add(fin, value); // cpx11: o prêmio
        if value > 1000 && !LEAGUE_PRIZE_MONEY_DRY_RUN {
            // simplification: the original pushes the position in dl and the rest of edx (29 × i, leftover from the row calculation); the
            // 0x48e3e0 reads only the byte
            let pos = ((29 * i as i32 as u32) & !0xff) | (i as u8).wrapping_add(1) as u32;
            news(league, club, value as u32, (value >> 31) as u32, pos, 0, 0, -2);
        }
        i = i.wrapping_add(1);
        if i >= read_i16(league + league::CLUB_COUNT) {
            break;
        }
    }
}


/// Updates the staff records by status: 5 the title (0x868c50), 0 promotion (0x869480, weight 100), 3 relegation
/// (0x86a1c0); returns 0.
///
/// Ret 0x18: `League::update_staff_records_for_status(club, status, stage, out, _, _)` (virtual +0x44 of the league):
/// status 5, the title (Staff_Records 0x868c50 of `[0xdd7f00]` with the club and competition +4); 0, promotion
/// (0x869480 with weight 100); 3, relegation (0x86a1c0). Returns 0 and does not write the output.
/// Original: `League::update_staff_records_for_status` (0x686940, league.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x686940 } (thiscall, ret 0x18, league.cpp): `League::update_staff_records_for_status(clube, situação, fase,
/// saída, _, _)` (a virtual +0x44 da liga): situação 5, o título (Staff_Records 0x868c50 de [0xdd7f00] com o clube e
/// a competição +4); 0, o acesso (0x869480 com peso 100); 3, o rebaixamento (0x86a1c0). Devolve 0 e não grava a saída.
pub(crate) unsafe extern "thiscall" fn league_update_staff_records_for_status(league: usize, club: usize, status: u32, stage: u32, out: usize, _a: u32, _b: u32) -> u32 {
    let records = read_usize(G_STAFF_RECORDS);
    let comp = read_usize(league + 4);
    match status as u8 {
        5 => {
            crate::comparison::branch("cpx_11 registro do titulo");
            let title: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
            title(records, club, comp);
        }
        0 => {
            crate::comparison::branch("cpx_11 registro do acesso");
            let promotion: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
            promotion(records, club, comp, 0x64);
        }
        3 => {
            crate::comparison::branch("cpx_11 registro do rebaixamento");
            let relegation: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
            relegation(records, club, comp);
        }
        _ => {}
    }
    0
}

// simplification: the three records append to the people’s history and change reputation (they allocate): original only; the
// the copy takes the branches.

/// Announces a team's status: outside friendlies, calls the competition's virtual +0x44 and builds news item 0xc (club,
/// competition, stage, status, round, what the virtual returned), delivered by scope or, by status, to the nation, the
/// division (and the promotion or relegation one) or the club's manager.
///
/// Ret 0x14; SEH frame with no destructor: `League::announce_team_status(club, status, stage, _, _)`: friendly
/// (0x5c6710), 0. Virtual +0x44 of the competition (`[0xadadfc]` by the id of +4) with the club, the status, the stage
/// and the output (0xff); −1 from it: 0. The round (+0x3a; with stages, +0x44 = 0xff, and stage ≠ 0xff: that of the
/// stage). News 0xc (local): 0 the club, 1 the competition, 2 the stage, 3 the status, 4 the round, 5 what the virtual
/// returned. Output ≠ 0xff: delivery by scope (0x786090 with the club's nation and the continent). Otherwise by status:
/// 5, to the competition's nation (0x7863f0) without +0x1c, to the division (0x7862d0) with it; 0 and 3, to the
/// division and to the competition's promotion (+0x1c) or relegation (+0x20) division, if any; 1 and 2, to the
/// division; 6, 7 and 0xfe, to the club's manager (0x7861e0). Returns 1.
/// Original: `League::announce_team_status` (0x686670, league.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x686670 } (thiscall, ret 0x14, league.cpp, quadro SEH sem destrutor): `League::announce_team_status(clube,
/// situação, fase, _, _)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o
/// clube, a situação, a fase e a saída (0xff); −1 dela: 0. A rodada (+0x3a; com fases, +0x44 = 0xff, e fase ≠ 0xff:
/// a da fase). A notícia 0xc (local): 0 o clube, 1 a competição, 2 a fase, 3 a situação, 4 a rodada, 5 o que a
/// virtual devolveu. Saída ≠ 0xff: a entrega pelo alcance (0x786090 com a nação do clube e o continente). Senão pela
/// situação: 5, à nação da competição (0x7863f0) sem o +0x1c, à divisão (0x7862d0) com ele; 0 e 3, à divisão e à
/// divisão de acesso (+0x1c) ou de rebaixamento (+0x20) da competição, se houver; 1 e 2, à divisão; 6, 7 e 0xfe, ao
/// técnico do clube (0x7861e0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn announce_team_status(league: usize, club: usize, status: u32, stage: u32, _a: u32, _b: u32) -> u32 {
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let to_club: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let division: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
    let nation: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    let by_reach: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    // simplification: the original writes only the output byte in a stack dword and pushes the whole dword to 0x786090, which
    // reads the byte; the copy passes the high part zeroed
    let mut out: u32 = 0xff;
    let mut round = read_i16(league + league::STAGE_ID);
    if friendly(read_usize(G_FRIENDLY), read_usize(league + 4)) != 0 {
        return 0;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(read_usize(league + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, u32, u32) -> i32 = core::mem::transmute(read_usize(read_usize(comp) + 0x44));
    let r = virtual44(comp, club, status, stage, &mut out, 0, 0);
    if r == -1 {
        return 0;
    }
    if read_u8(league + league::STAGE_INDEX) == 0xff && stage as u8 != 0xff {
        let f = read_usize(read_usize(league + league::STAGES).wrapping_add((stage as u8 as i8 as isize as usize).wrapping_mul(4)));
        round = read_i16(f + 0x3a);
    }
    crate::comparison::branch("cpx_11 noticia da situacao");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xc, 0);
    write(n, 0, read_usize(club) as u32);
    write(n, 1, read_usize(read_usize(league + 4)) as u32);
    write(n, 2, stage as u8 as i8 as i32 as u32);
    write(n, 3, status as u8 as i8 as i32 as u32);
    write(n, 4, round as i32 as u32);
    write(n, 5, r as u32);
    let news = read_usize(G_NEWS);
    if out as u8 != 0xff {
        let nacao = read_usize(club + club::NATION);
        let continent = if nacao != 0 { read_usize(nacao + nation::CONTINENT) } else { 0 };
        by_reach(news, n, club, read_usize(league + 4), nacao, continent, out);
        return 1;
    }
    // the competition’s promotion (+0x1c) or relegation (+0x20) division, 0x6b records in [0xae23d0]
    let other = |desl: usize| {
        let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(read_usize(league + 4)).wrapping_mul(4)));
        let e = read_usize(c + desl) as i32;
        if e != -1 {
            division(news, n, read_usize(G_CLUB_COMP_TABLE).wrapping_add((e as usize).wrapping_mul(0x6b)));
        }
    };
    match status as u8 {
        5 => {
            if read_usize(league + league::UPPER_DIVISION_ID) as i32 == -1 {
                nation(news, n, read_usize(read_usize(league + 4) + 0x5d));
            } else {
                division(news, n, read_usize(league + 4));
            }
        }
        0 => {
            division(news, n, read_usize(league + 4));
            other(0x1c);
        }
        7 | 6 | 0xfe => {
            to_club(news, n, club);
        }
        1 | 2 => {
            division(news, n, read_usize(league + 4));
        }
        3 => {
            division(news, n, read_usize(league + 4));
            other(0x20);
        }
        _ => {}
    }
    1
}

// simplification: virtual +0x44 appends records and the news goes to the managers’ inboxes (they allocate): original only; the
// copy takes the branch.
// Inside the comparer of 0x684cd0 (STATUS_ANNOUNCE_RECORD), stores on its tape what the announcement wrote to the regions.



/// O ensaio do comparador da 0x685d30: a cópia roda sem a estatística da competição (0x49f570, que acha ou cria os
/// registros dos jogadores e não pode rodar duas vezes).
pub(crate) static mut LEAGUE_ADD_RESULT_DRY_RUN: bool = false;

/// Adds a result to the league: stamps today's date on the league; a stage fixture goes to its stage; otherwise adds
/// the fixture to the table (0x685f60) and to the competition stats (0x49f570) and returns 1.
///
/// Ret 8: `League::add_result(fixture, match)`: stores the day's date in +0xc9 (0x417c10 of `[0xae2c90]`); league with
/// stages (+0x44 = 0xff) and a fixture of a stage (+0x42 ≠ 0xff): the stage (`[+0xc]` + 4 × stage, up to +0x30)
/// receives the fixture through virtual +4 and what it returns is returned; stage out of range: the boxes and 0.
/// Fixture of another stage (+0x42 ≠ +0x44, which is not 0xfe): 0. Otherwise adds the fixture to the table (0x685f60 on
/// table +0xb1 with the +0x3e rows, marking) and, outside stages 0xfe, with the competition (`[0xadadfc]` by the id of
/// +4) and its stats (+8) and with the match: 0x49f570 with the points of both; returns 1. Null fixture: the box and
/// −1.
/// Original: `League::add_result` (0x685d30, league.cpp, thiscall, ret 8).
///
/// [pt] { 0x685d30 } (thiscall, ret 8, league.cpp): `League::add_result(partida, jogo)`: grava a data do dia em +0xc9
/// (0x417c10 de [0xae2c90]); liga com fases (+0x44 = 0xff) e partida de uma fase (+0x42 ≠ 0xff): a fase
/// ([+0xc] + 4 × fase, até +0x30) recebe a partida pela virtual +4 e o que ela devolve sai; fase fora da faixa: as
/// caixas e 0. Partida de outra fase (+0x42 ≠ +0x44, que não é 0xfe): 0. Senão soma a partida na tabela (0x685f60 na
/// tabela +0xb1 com as +0x3e linhas, marcando) e, fora das fases 0xfe, com a competição ([0xadadfc] pelo id de +4) e a
/// estatística dela (+8) e com o jogo: 0x49f570 com os pontos dos dois; devolve 1. Partida nula: a caixa e −1.
pub(crate) unsafe extern "thiscall" fn league_add_result(league: usize, p: usize, game: usize) -> i32 {
    if p == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT)(league, p, game);
    }
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copy(league + league::LAST_RESULT_DATE, local.as_mut_ptr(), G_CURRENT_DATE);
    let stage_of_league = *((league + league::STAGE_INDEX) as *const u8);
    let stage = *((p + 0x42) as *const u8);
    if stage_of_league == 0xff && stage != 0xff {
        let k = stage as i8 as i32;
        if k > read_usize(league + league::CURRENT_STAGE) as i32 || k < 0 {
            // the stage’s message boxes outside the range (lines 0xc8e and 0xc8f): the original (the date is already written, the same)
            return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT)(league, p, game);
        }
        let filha = read_usize(read_usize(league + league::STAGES).wrapping_add(4 * k as usize));
        let add: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(read_usize(read_usize(filha) + 4));
        return add(filha, p, game);
    }
    if stage_of_league != stage && stage_of_league != 0xfe {
        return 0;
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16) = core::mem::transmute(FN_LEAGUE_APPLY_RESULT_TO_TABLE);
    let (mut pts_casa, mut pts_fora) = (0i16, 0i16);
    // simplification: the original pushes +0x3e with the high part of ecx (leftover from the previous call); 0x685f60 reads only the short
    crate::comparison::branch("cpw_11 resultado aplicado");
    aplicar(league, p, read_usize(league + league::TABLE), read_i16(league + league::CLUB_COUNT) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela
    if *((league + league::STAGE_INDEX) as *const u8) == 0xfe || read_usize(G_COMP_OBJECT_TABLE) == 0 {
        return 1;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(read_usize(league + 4)).wrapping_mul(4)));
    if comp == 0 || read_usize(comp + 8) == 0 || game == 0 {
        return 1;
    }
    if !LEAGUE_ADD_RESULT_DRY_RUN {
        // simplification: the original pushes the points as dwords of the locals with an uninitialized high part; 0x49f570 reads the shorts
        let estatistica: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32) -> u32 = core::mem::transmute(FN_COMP_STATS_RECORD_MATCH_RESULT);
        estatistica(read_usize(comp + 8), p, game, pts_casa as i32, pts_fora as i32);
    }
    1
}


/// Allocates and returns a copy of the table rows, with new head-to-head and form buffers; sets +0xa7 when it is −1.
///
/// Ret 0: `League::copy_table()`: no table (+0xb1) or no rows (+0x3e): 0. With +0xa7 = −1, writes (rows − 1) × +0x3c
/// into it (16 bits; more than 50 rows: the box at line 0x1059 first, as in the original). Allocates the rows
/// (0x944e46, 0x3b each; out of memory: box 0x1061 and 0) and copies them; with head-to-head on the first row (+5),
/// each row gets a new vector of (rows − 1) × 14 bytes with a copy of its own; with form (+9), one of +0xa7 bytes.
/// Returns the copy. The vector allocations are not checked (as in the original).
/// Original: `League::copy_table` (0x687710, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x687710 } (thiscall, ret 0, league.cpp): `League::copy_table()`: sem tabela (+0xb1) ou sem linhas (+0x3e): 0.
/// Com +0xa7 = −1, grava nele (linhas − 1) × +0x3c (16 bits; mais de 50 linhas: a caixa da linha 0x1059 antes, pela
/// original). Aloca as linhas (0x944e46, 0x3b cada; sem memória: a caixa 0x1061 e 0) e copia; com os confrontos na
/// primeira linha (+5), cada linha ganha um vetor novo de (linhas − 1) × 14 bytes com a cópia dos dela; com a forma
/// (+9), um de +0xa7 bytes. Devolve a cópia. As alocações dos vetores não são conferidas (como na original).
pub(crate) unsafe extern "thiscall" fn copy_table(league: usize) -> usize {
    let table = read_usize(league + league::TABLE);
    let n = read_i16(league + league::CLUB_COUNT);
    if table == 0 || n == 0 {
        return 0;
    }
    if read_i16(league + league::GAMES_PER_CLUB) == -1 && n > 0 {
        if n > 0x32 {
            return original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COPY_TABLE)(league); // simplification: the box, original only
        }
        let v = n.wrapping_sub(1).wrapping_mul(read_i16(league + league::LEGS));
        core::ptr::write_unaligned((league + league::GAMES_PER_CLUB) as *mut i16, v);
    }
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let row = |i: i32| (i as usize).wrapping_mul(0x3b);
    let c = alloc((n as i32 * 0x3b) as u32);
    if c == 0 {
        show_error_at(0x9f15f4, 0x1061);
        return 0;
    }
    core::ptr::copy_nonoverlapping(table as *const u8, c as *mut u8, n as i32 as usize * 0x3b);
    if read_usize(table + 5) != 0 && n > 0 {
        let mut i: i16 = 0;
        while i < read_i16(league + league::CLUB_COUNT) {
            let m = read_i16(league + league::CLUB_COUNT) as i32;
            let h = alloc((14 * m - 14) as u32);
            core::ptr::write_unaligned((c + row(i as i32) + 5) as *mut usize, h);
            let de = read_usize(read_usize(league + league::TABLE) + row(i as i32) + 5);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, (14 * read_i16(league + league::CLUB_COUNT) as i32 - 14) as usize);
            i += 1;
        }
    }
    if read_usize(read_usize(league + league::TABLE) + 9) != 0 && read_i16(league + league::CLUB_COUNT) > 0 {
        let mut i: i16 = 0;
        while i < read_i16(league + league::CLUB_COUNT) {
            let t = read_i16(league + league::GAMES_PER_CLUB) as i32;
            let h = alloc(t as u32);
            core::ptr::write_unaligned((c + row(i as i32) + 9) as *mut usize, h);
            let de = read_usize(read_usize(league + league::TABLE) + row(i as i32) + 9);
            core::ptr::copy_nonoverlapping(de as *const u8, h as *mut u8, read_i16(league + league::GAMES_PER_CLUB) as i32 as usize);
            i += 1;
        }
    }
    c
}


/// Sorts the table's clubs by strength and stores each one's predicted position in the competition stats.
///
/// Ret 0: `League::set_predicted_positions()`: without stats (+8): the box (line 0x189e). Copies the clubs of the +0x3e
/// table rows (+0xb1, 0x3b each) into a vector (0x944e46; out of memory: box 0x18a6, without freeing), sorts by
/// strength (0x4c08f0) and stores position i + 1 of each non-null club in the stats (0x4a2540); frees the vector
/// (0x9452ca).
/// Original: `League::set_predicted_positions` (0x68a850, league.cpp, thiscall, ret 0).
///
/// [pt] { 0x68a850 } (thiscall, ret 0, league.cpp): `League::set_predicted_positions()`: sem estatística (+8): a caixa
/// (linha 0x189e). Copia os clubes das +0x3e linhas da tabela (+0xb1, 0x3b cada) para um vetor (0x944e46; sem memória:
/// a caixa 0x18a6, sem soltar), ordena pela força (0x4c08f0) e grava a posição i + 1 de cada clube não nulo na
/// estatística (0x4a2540); solta o vetor (0x9452ca).
pub(crate) unsafe extern "thiscall" fn league_set_predicted_positions(league: usize) {
    if read_usize(league + 8) == 0 {
        return show_error_at(0x9f15f4, 0x189e);
    }
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sort: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    let write: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let v = alloc(((read_i16(league + league::CLUB_COUNT) as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x9f15f4, 0x18a6);
    }
    let mut i = 0i32;
    while i < read_i16(league + league::CLUB_COUNT) as i32 {
        *((v + 4 * i as usize) as *mut u32) = read_usize(read_usize(league + league::TABLE) + 0x3b * i as usize) as u32;
        i += 1;
    }
    sort(v, read_i16(league + league::CLUB_COUNT) as i32);
    let mut i = 0i32;
    while i < read_i16(league + league::CLUB_COUNT) as i32 {
        let c = read_usize(v + 4 * i as usize);
        if c != 0 {
            // simplification: the original pushes ecx with the position in the low byte (the rest is leftover); 0x4a2540 reads the byte
            write(read_usize(league + 8), c, (i as u8).wrapping_add(1) as u32);
        }
        i += 1;
    }
    free(v);
}




/// Returns entry i (0xe bytes: club, points per game, points, projected points, games) of the table projection buffer.
///
/// [pt] A entrada da projeção (0xe bytes, no bloco da 0x944cf1): +0 o clube, +4 os pontos por jogo (float), +8 os pontos,
/// +0xa os pontos projetados, +0xc os jogos.
pub(crate) unsafe fn projection_entry(buf: usize, i: i32) -> usize {
    buf.wrapping_add((i as isize * 0xe) as usize)
}

/// qsort comparator of projection entries by club reputation (+0x80), highest first
/// Original: `compare_projection_by_reputation_desc` (0x68afb0, league.cpp, cdecl).
///
/// [pt] { 0x68afb0 } (cdecl, league.cpp): `compare_projection_by_reputation_desc(a, b)`: o qsort das entradas da projeção
/// pela reputação do clube (+0 → +0x80, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_projection_by_reputation_desc(a: usize, b: usize) -> i32 {
    let ra = read_i16(read_usize(a) + 0x80) as i32;
    let rb = read_i16(read_usize(b) + 0x80) as i32;
    rb - ra // the club with the highest reputation first
}

/// qsort comparator of projection entries by projected points (+0xa), highest first
/// Original: `compare_projection_by_points_desc` (0x68afd0, league.cpp, cdecl).
///
/// [pt] { 0x68afd0 } (cdecl, league.cpp): `compare_projection_by_points_desc(a, b)`: o qsort das entradas da projeção pelos
/// pontos projetados (+0xa, palavra com sinal), o maior primeiro. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_projection_by_points_desc(a: usize, b: usize) -> i32 {
    read_i16(b + 0xa) as i32 - read_i16(a + 0xa) as i32 // most points first
}

/// Index of the club's first entry in the sorted projection, or None.
///
/// [pt] A posição (1..) do clube na projeção já ordenada (a primeira entrada dele; `None` sem ela).
pub(crate) unsafe fn projection_position(buf: usize, n: i16, club: usize) -> Option<i32> {
    (0..n as i32).find(|&i| read_usize(projection_entry(buf, i)) == club)
}

/// Multiplies the entry's points per game (+4, float) by factor and recomputes its projected points (+0xa); returns the
/// remaining games.
///
/// [pt] O clube com outra forma: os pontos por jogo (+4) vezes `fator` (o float guardado; o x87 segue com o de 53 bits) e
/// os pontos projetados de novo com ele.
pub(crate) unsafe fn apply_form_factor(this: usize, e: usize, factor: f64) -> i32 {
    let r = *((e + 4) as *const f32) as f64 * factor;
    *((e + 4) as *mut f32) = r as f32; // fst: stores the float and continues with the 53-bit one
    let resta = read_i16(this + league::GAMES_PER_CLUB) as i32 - read_i16(e + 0xc) as i32;
    *((e + 0xa) as *mut i16) = (match_pl_ftol(resta as f64 * r) as i16).wrapping_add(read_i16(e + 8));
    resta
}

/// Projects each table club's end-of-season points (in the first third of the season, by a reputation scale), sorts
/// them and hands the club's positions in normal, better (× 1.5) and worse (× 0.444) form to virtual +0x68.
///
/// Ret 4; virtual +0x84 of the leagues, called only by Comp_Stats 0x4ae300, which the board uses in the weekly
/// confidence 0x697c40 and in the reason for the manager leaving 0x69b350: `League::update_projected_positions(club)`:
/// with the club in the table (+0xb1, 0x3b bytes, count +0x3e: +0 the club, +0x11 the matches, +0x19 the points),
/// builds in a block (0x944cf1) one 0xe-byte entry per row with the points per match; in the first third of the season
/// (club's matches × 3 < +0xa7), replaces them with a scale by reputation (0.83 × +0xc2 for the first down to 0.23 ×
/// +0xc2 below the last). Projects the end-of-season points, sorts and finds the club's position in normal, better (×
/// 1.5) and worse (× 0.444) form; frees the block and hands the three to virtual +0x68 (`set_club_progress(club,
/// competition +0x44, normal, better, worse)`, which writes bytes +0x73..+0x75 of the club's record in Comp_Stats).
/// Returns 1; outside the table, 0.
/// Original: `League::update_projected_positions` (0x68ac40, league.cpp, thiscall, ret 4).
///
/// [pt] { 0x68ac40 } (thiscall, ret 4, league.cpp; virtual +0x84 das ligas, chamada só por Comp_Stats 0x4ae300, que a
/// diretoria usa na confiança semanal 0x697c40 e no motivo da saída do técnico 0x69b350):
/// `League::update_projected_positions(clube)`: com o clube na tabela (+0xb1, 0x3b bytes, conta +0x3e: +0 o clube,
/// +0x11 os jogos, +0x19 os pontos), monta num bloco (0x944cf1) uma entrada de 0xe bytes por linha com os pontos por
/// jogo; no primeiro terço da temporada (jogos do clube × 3 < +0xa7), troca-os por uma escala pela reputação
/// (0,83 × +0xc2 do primeiro ao 0,23 × +0xc2 abaixo do último). Projeta os pontos no fim da temporada, ordena e acha a
/// posição do clube na forma normal, melhor (× 1,5) e pior (× 0,444); libera o bloco e entrega as três à virtual
/// +0x68 (`set_club_progress(clube, competição +0x44, normal, melhor, pior)`, que grava os bytes +0x73..+0x75 do
/// registro do clube no Comp_Stats). Devolve 1; fora da tabela, 0.
pub(crate) unsafe extern "thiscall" fn update_projected_positions(this: usize, club: usize) -> u32 {
    let n = read_i16(this + league::CLUB_COUNT);
    let table = read_usize(this + league::TABLE);
    let row = match (0..n as i32).map(|i| table.wrapping_add((i as isize * 0x3b) as usize)).find(|&l| read_usize(l) == club) {
        Some(l) => l,
        None => return 0,
    };
    if row == 0 || n == 0 {
        return 0;
    }
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let buf = new((n as i32 * 0xe) as u32);
    let mut i: i16 = 0;
    while i < read_i16(this + league::CLUB_COUNT) {
        let l = read_usize(this + league::TABLE).wrapping_add((i as isize * 0x3b) as usize);
        let e = projection_entry(buf, i as i32);
        core::ptr::write_unaligned(e as *mut usize, read_usize(l));
        *((e + 8) as *mut i16) = read_i16(l + 0x19);
        let games = *((l + 0x11) as *const i8);
        *((e + 0xc) as *mut i16) = games as i16;
        *((e + 4) as *mut f32) = if games > 0 { (read_i16(l + 0x19) as f64 / games as f64) as f32 } else { 0.0 };
        i += 1;
    }
    // the original’s three locals (dwords): the low byte of each becomes the position; the high bytes are leftovers
    // in them (float A and the row count on the reputation scale, the club’s remaining matches in the third)
    let (mut normal, mut melhor, mut pior) = (0u32, 0u32, 0u32);
    if (*((row + 0x11) as *const i8) as i32) * 3 < read_i16(this + league::GAMES_PER_CLUB) as i32 {
        crate::comparison::branch("cpr_11 projecao pela injury_word_0c");
        let n = read_i16(this + league::CLUB_COUNT);
        qsort(buf, n as i32 as u32, 0xe, 0x68afb0);
        let c = *((this + league::WIN_POINTS) as *const i8) as f64;
        let a = (c * read_f64_at(0x96d0e8)) as f32; // fstp into the local: the first rounds to float; 0x96d0e8 = 0.83
        normal = a.to_bits();
        let b = c * read_f64_at(0x96d0e0); // 0x96d0e0 = 0.23
        if n > 0 {
            let d = a as f64 - b;
            let step_towards = d / n as f64;
            for k in 0..n as i32 {
                *((projection_entry(buf, k) + 4) as *mut f32) = (a as f64 - step_towards * k as f64) as f32; // from largest to smallest
            }
            melhor = n as i32 as u32;
        }
    }
    let mut i: i16 = 0;
    while i < read_i16(this + league::CLUB_COUNT) {
        let e = projection_entry(buf, i as i32);
        let resta = read_i16(this + league::GAMES_PER_CLUB) as i32 - read_i16(e + 0xc) as i32;
        pior = resta as u32;
        *((e + 0xa) as *mut i16) = (match_pl_ftol(resta as f64 * *((e + 4) as *const f32) as f64) as i16).wrapping_add(read_i16(e + 8));
        i += 1;
    }
    qsort(buf, read_i16(this + league::CLUB_COUNT) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = projection_position(buf, read_i16(this + league::CLUB_COUNT), club) {
        normal = (normal & !0xff) | (k as u8).wrapping_add(1) as u32; // the position in normal form
        pior = apply_form_factor(this, projection_entry(buf, k), read_f64_at(0x968940)) as u32; // 0x968940 = 1.5
    }
    qsort(buf, read_i16(this + league::CLUB_COUNT) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = projection_position(buf, read_i16(this + league::CLUB_COUNT), club) {
        melhor = (melhor & !0xff) | (k as u8).wrapping_add(1) as u32;
        pior = apply_form_factor(this, projection_entry(buf, k), read_f64_at(0x96d0d8)) as u32; // 0x96d0d8 = 0.4444444444
    }
    qsort(buf, read_i16(this + league::CLUB_COUNT) as i32 as u32, 0xe, 0x68afd0);
    for k in 0..read_i16(this + league::CLUB_COUNT) as i32 {
        if read_usize(projection_entry(buf, k)) == club {
            pior = (pior & !0xff) | (k as u8).wrapping_add(1) as u32; // the last one (the club appears only once)
        }
    }
    free(buf);
    crate::comparison::branch("cpr_11 projecao");
    // simplification: the high bytes the original leaves in the three locals outside the reputation scale are stack leftovers
    // (here, zero); the 69 versions of virtual +0x68 only read the low byte of each argument (or pass the dwords on to
    // another one that reads)
    let progresso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) =
        core::mem::transmute(read_usize(read_usize(this) + 0x68));
    let competition = (melhor & !0xff) | *((this + league::STAGE_INDEX) as *const u8) as u32; // dl over the edx of the second local
    progresso(this, club, competition, normal, melhor, pior);
    1
}








/// Linhas da tabela: 0x3b bytes (+0 clube, +4 posição, +5 confrontos diretos de 14 bytes, +9 histórico das posições,
/// +0x11/+0x1f/+0x2d os totais, em casa e fora, +0x37 situação, +0x38 marca, +0x39, +0x3a clubes).
pub(crate) const TABLE_ROW_SIZE: usize = 0x3b;

/// True if tiebreak rule 4 (head-to-head) appears among the league's 4 tiebreak rules (+0xc5) before the first 0.
///
/// [pt] A regra 4 (confrontos diretos) entre as 4 regras de desempate (+0xc5, até a primeira 0).
pub(crate) unsafe fn uses_head_to_head_rule(this: usize) -> bool {
    for k in 0..4 {
        match read_u8(this + league::TIEBREAKS + k) {
            0 => return false,
            4 => return true,
            _ => {}
        }
    }
    false
}

/// Puts the club in table row i: totals cleared, status 0xff, flag, total rounds (+0xa7) if still −1, position history
/// and, under rule 4, the head-to-head entries (the last row fills every row's opponents).
///
/// Ret 0x10: `League::init_table_row(i, club, flag, rows)`: puts the club in row `i` (char) of table `rows` (null: the
/// league's, +0xb1): a club with status +0x64 = 3 changes to 2; total rounds +0xa7 still −1 becomes (clubs +0x3e − 1) ×
/// legs +0x3c (more than 50 clubs: box and continues); the row gets the club, position `i`, the three totals blocks
/// zeroed, the flag (+0x38), status 0xff, 1 in +0x39 and the clubs (+0x3a); with legs, the position history (+9, +0xa7
/// bytes; out of memory, box and end); with rule 4, the head-to-head entries (+5, 14 bytes per opponent, zeroed). On
/// the last row with head-to-head, each row receives the opponents in table order.
/// Original: `League::init_table_row` (0x687430, league.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x687430 } (thiscall, ret 0x10, league.cpp): `League::init_table_row(i, clube, marca, linhas)`: põe o clube na
/// linha `i` (char) da tabela `linhas` (nula: a da liga, +0xb1): o clube com situação +0x64 = 3 passa a 2; o total de
/// rodadas +0xa7 ainda −1 vira (clubes +0x3e − 1) × turnos +0x3c (mais de 50 clubes: caixa e segue); a linha recebe o
/// clube, a posição `i`, os três blocos de totais zerados, a marca (+0x38), situação 0xff, 1 em +0x39 e os clubes
/// (+0x3a); com turnos, o histórico das posições (+9, +0xa7 bytes; sem memória, caixa e fim); com a regra 4, os
/// confrontos diretos (+5, 14 bytes por adversário, zerados). Na última linha com confrontos, cada linha recebe os
/// adversários na ordem da tabela.
pub(crate) unsafe extern "thiscall" fn init_table_row(this: usize, i: u32, club: usize, mark: u32, rows: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    if club != 0 && read_u8(club + club::PROFESSIONAL_STATUS) == 3 {
        *((club + club::PROFESSIONAL_STATUS) as *mut u8) = 2;
    }
    if read_i16(this + league::GAMES_PER_CLUB) == -1 && read_i16(this + league::CLUB_COUNT) > 0 {
        if read_i16(this + league::CLUB_COUNT) > 0x32 {
            show_error_at(0x9f15f4, 0xfe9); // more than 50 clubs: the box, and it continues
        }
        let total = (read_i16(this + league::CLUB_COUNT) - 1).wrapping_mul(read_i16(this + league::LEGS));
        core::ptr::write_unaligned((this + league::GAMES_PER_CLUB) as *mut i16, total);
    }
    let table = if rows == 0 { read_usize(this + league::TABLE) } else { rows };
    let idx = i as u8 as i8 as isize;
    let l = table.wrapping_add((idx * TABLE_ROW_SIZE as isize) as usize);
    write_usize(l, club);
    *((l + 4) as *mut u8) = i as u8;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        write_usize(e - 4, 0);
        core::ptr::write_bytes(e as *mut u8, 0, 4);
        core::ptr::write_bytes((e + 4) as *mut u8, 0, 6);
    }
    *((l + 0x38) as *mut u8) = mark as u8;
    *((l + 0x37) as *mut u8) = 0xff;
    *((l + 0x39) as *mut u8) = 1;
    *((l + 0x3a) as *mut u8) = read_u8(this + league::CLUB_COUNT);
    write_usize(l + 5, 0);
    if read_i16(this + league::LEGS) > 0 {
        let p = alloc(read_i16(this + league::GAMES_PER_CLUB) as i32 as u32);
        write_usize(l + 9, p);
        if p == 0 {
            show_error_at(0x9f15f4, 0x100c);
            return;
        }
    } else {
        write_usize(l + 9, 0);
    }
    if uses_head_to_head_rule(this) {
        let n = read_i16(this + league::CLUB_COUNT) as i32;
        write_usize(l + 5, alloc((n * 14 - 14) as u32));
        // the fixtures zeroed (+4..+0xd of each; the original writes +4 twice, without checking memory)
        let mut k = 0i32;
        while k < read_i16(this + league::CLUB_COUNT) as i32 - 1 {
            let e = read_usize(l + 5) + 14 * k as usize;
            core::ptr::write_bytes((e + 4) as *mut u8, 0, 10);
            k += 1;
        }
    }
    let n = read_i16(this + league::CLUB_COUNT) as isize;
    if idx == n - 1 && read_usize(l + 5) != 0 {
        crate::comparison::branch("cpm_2 confrontos diretos");
        let mut a = 0isize;
        while a < read_i16(this + league::CLUB_COUNT) as isize {
            let mut k = 0usize;
            let mut j = 0isize;
            while j < read_i16(this + league::CLUB_COUNT) as isize {
                if a != j {
                    let p = read_usize(table + a as usize * TABLE_ROW_SIZE + 5);
                    write_usize(p + 14 * k, read_usize(table + j as usize * TABLE_ROW_SIZE)); // the opposing club
                    k += 1;
                }
                j += 1;
            }
            a += 1;
        }
    }
}

/// a new array with the table clubs and the count; without memory, error box and count 0
/// Original: `League::get_club_array` (0x687a20, league.cpp, thiscall, ret 8).
///
/// [pt] { 0x687a20 } (thiscall, ret 8, league.cpp): `League::get_club_array(saida, n)`: um vetor novo (malloc 0x944e46) com
/// os clubes das linhas da tabela (+0xb1) em `*saida` e a conta (+0x3e) em `*n`; sem memória, caixa e `*n` = 0.
pub(crate) unsafe extern "thiscall" fn league_get_club_array(this: usize, out: *mut usize, n: *mut i16) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let p = alloc((read_i16(this + league::CLUB_COUNT) as i32 * 4) as u32);
    *out = p;
    if p == 0 {
        show_error_at(0x9f15f4, 0x10cc);
        *n = 0;
        return;
    }
    crate::comparison::branch("cpm_2 vetor dos clubes");
    let mut i = 0i16;
    while i < read_i16(this + league::CLUB_COUNT) {
        write_usize(p + 4 * i as usize, read_usize(read_usize(this + league::TABLE) + i as usize * TABLE_ROW_SIZE));
        i += 1;
    }
    *n = read_i16(this + league::CLUB_COUNT); // a conta
}

/// each club still in the league division stores last season's division (+0x5b) and the final position (+0x5f)
/// Original: `League::record_final_positions` (0x68aa30, league.cpp, thiscall).
///
/// [pt] { 0x68aa30 } (thiscall, league.cpp): `League::record_final_positions()`: cada clube da tabela (+0xb1, na ordem) que
/// ainda está na divisão da liga (+0x57 = +4 dela) guarda a divisão como a da última temporada (+0x5b) e a posição
/// (linha + 1) em +0x5f.
pub(crate) unsafe extern "thiscall" fn record_final_positions(this: usize) {
    let mut i = 0i16;
    while i < read_i16(this + league::CLUB_COUNT) {
        let division = read_usize(this + 4);
        let c = read_usize(read_usize(this + league::TABLE) + i as usize * TABLE_ROW_SIZE);
        if read_usize(c + club::DIVISION) == division {
            crate::comparison::branch("cpm_2 posicao final");
            write_usize(c + club::LAST_DIVISION, division);
            *((c + club::LAST_POSITION) as *mut u8) = (i as u8).wrapping_add(1); // the final position
        }
        i += 1;
    }
}





/// Adds 1 to the byte at address a.
pub(crate) unsafe fn increment_u8(a: usize) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(1);
}

/// Adds v to the i16 at address a.
pub(crate) unsafe fn league_add_i16(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, core::ptr::read_unaligned(a as *const i16).wrapping_add(v));
}

/// Linhas da tabela: 0x3b bytes; +0 clube, +5 confrontos (entradas de 14), +0x11..+0x1a os totais, +0x1f..+0x28 em
/// casa (jogos, vitórias, empates, derrotas, gols pró, contra, pontos), +0x2d..+0x36 fora.
pub(crate) const RESULT_TABLE_ROW_SIZE: usize = 0x3b;

/// Applies a result to the table: finds the home and away rows and adds the game at home and away (played, goals,
/// win/draw/loss, by extra time and penalties, marking +0x3d/+0x3e), the win points by the +0x90 virtual and the draw
/// points +0xc3; redoes the totals, returns both rows' points and adds the head-to-head (0x6869a0).
///
/// Ret 0x18: `League::apply_result_to_table(fixture, table, n, mark, home_pts*, away_pts*)`: finds the rows of the home
/// side (+0x1c) and the away side (+0x20) among the `n` (short) of the table (stops at the first club that is neither
/// after finding both) and adds the game: matches (+0x1f/+0x2d), goals (+0x23/+0x25 and +0x31/+0x33: the score of
/// +0x47/+0x48 when ≠ 0xff, otherwise that of +0x43/+0x44). The result from score +0x43/+0x44; level: by extra time
/// (+0x38 bit 2, +0x47/+0x48; `mark` writes 3 in +0x3d/+0x3e) and by penalties (bits 0x20/1, +0x49/+0x4a; a home win
/// counts as a draw for it; 5 with 0x20, otherwise 4); a pure draw with the league's draw points (+0xc3). The win
/// points through virtual +0x90 (0x68aad0) of the type 2 competition (`[0xadadfc]`, +0x43) or of the league itself,
/// read on each call. Redoes the totals of both rows and returns the points (+0x19) in `home_pts`/`away_pts` (0 without
/// the row). With head-to-head on the first row (+5), finds the opponent in each one and adds the tie (0x6869a0). No
/// SEH, no random draw, no box, all locals written before being read.
/// Original: `League::apply_result_to_table` (0x685f60, league.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x685f60 } (thiscall, ret 0x18, league.cpp): `League::apply_result_to_table(partida, tabela, n, marcar,
/// pts_casa*, pts_fora*)`: acha as linhas do mandante (+0x1c) e do visitante (+0x20) entre as `n` (curto) da tabela
/// (para no primeiro clube que não é nenhum dos dois depois de achar ambos) e soma o jogo: jogos (+0x1f/+0x2d), gols
/// (+0x23/+0x25 e +0x31/+0x33: o placar de +0x47/+0x48 quando ≠ 0xff, senão o de +0x43/+0x44). O resultado pelo placar
/// +0x43/+0x44; empatado: pela prorrogação (+0x38 bit 2, +0x47/+0x48; `marcar` grava 3 em +0x3d/+0x3e) e pelos pênaltis
/// (bits 0x20/1, +0x49/+0x4a; vitória do mandante conta como empate dele; 5 com o 0x20, senão 4); empate puro com os
/// pontos de empate da liga (+0xc3). Os pontos de vitória pela virtual +0x90 (0x68aad0) da competição de tipo 2
/// ([0xadadfc], +0x43) ou da própria liga, lida a cada chamada. Refaz os totais das duas linhas e devolve os pontos
/// (+0x19) em `pts_casa`/`pts_fora` (0 sem a linha). Com confrontos na primeira linha (+5), acha o adversário em cada
/// uma e soma o confronto (0x6869a0). Sem SEH, sem sorteio, sem caixa, locais todas gravadas antes de lidas.
pub(crate) unsafe extern "thiscall" fn apply_result_to_table(league: usize, p: usize, table: usize, n: u32, mark: u32, pts_home: *mut i16, pts_away: *mut i16) {
    let mut pontuador = league;
    let id = read_usize(league + 4);
    if id != 0 && read_usize(G_COMP_OBJECT_TABLE) != 0 {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(id).wrapping_mul(4)));
        if *((comp + 0x43) as *const u8) == 2 {
            pontuador = comp;
        }
    }
    let n = n as i16;
    let row = |k: i16| table.wrapping_add(RESULT_TABLE_ROW_SIZE.wrapping_mul(k as i32 as usize));
    let (mut casa, mut fora): (i16, i16) = (-1, -1);
    let mut k: i16 = 0;
    if n > 0 {
        loop {
            let c = read_usize(row(k));
            if c == read_usize(p + 0x1c) {
                casa = k;
            } else if c == read_usize(p + 0x20) {
                fora = k;
            } else if casa != -1 && fora != -1 {
                break;
            }
            k = k.wrapping_add(1);
            if k >= n {
                break;
            }
        }
    }
    let pb = |o: usize| *((p + o) as *const i8);
    let noventa = || *((p + 0x47) as *const u8) != 0xff;
    if casa > -1 {
        let r = row(casa);
        increment_u8(r + 0x1f);
        let (pro, contra) = if noventa() { (pb(0x47), pb(0x48)) } else { (pb(0x43), pb(0x44)) };
        league_add_i16(r + 0x23, pro as i16);
        league_add_i16(r + 0x25, contra as i16);
    }
    if fora > -1 {
        let r = row(fora);
        increment_u8(r + 0x2d);
        let (pro, contra) = if noventa() { (pb(0x48), pb(0x47)) } else { (pb(0x44), pb(0x43)) };
        league_add_i16(r + 0x31, pro as i16);
        league_add_i16(r + 0x33, contra as i16);
    }
    let points = |club: usize| -> i16 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(pontuador) + 0x90));
        f(pontuador, club, p) as u8 as i8 as i16
    };
    // home team wins: its +0x20 and points (+0x27), defeat (+0x30) for the away team; loses: +0x22 and +0x2e with the points (+0x35)
    let vence_casa = |win: usize| {
        if casa > -1 {
            let r = row(casa);
            increment_u8(r + win);
            league_add_i16(r + 0x27, points(read_usize(p + 0x1c))); // the home team’s points
        }
        if fora > -1 {
            increment_u8(row(fora) + 0x30);
        }
    };
    let vence_fora = |win: usize| {
        if casa > -1 {
            increment_u8(row(casa) + 0x22);
        }
        if fora > -1 {
            let r = row(fora);
            increment_u8(r + win);
            league_add_i16(r + 0x35, points(read_usize(p + 0x20)));
        }
    };
    let (gc, gf) = (pb(0x43), pb(0x44));
    let flags = core::ptr::read_unaligned((p + 0x38) as *const u16);
    let penalties = if *((p + 0x38) as *const u8) & 0x20 != 0 { 5u8 } else { 4 };
    if gc > gf {
        vence_casa(0x20);
    } else if gc < gf {
        vence_fora(0x2e);
    } else if flags & 2 != 0 && pb(0x47) > pb(0x48) {
        vence_casa(0x20);
        if mark != 0 {
            *((p + 0x3d) as *mut u8) = 3;
        }
    } else if flags & 2 != 0 && pb(0x47) < pb(0x48) {
        vence_fora(0x2e);
        if mark != 0 {
            *((p + 0x3e) as *mut u8) = 3;
        }
    } else if flags & 0x21 != 0 && pb(0x49) > pb(0x4a) {
        vence_casa(0x21);
        if mark != 0 {
            *((p + 0x3d) as *mut u8) = penalties;
        }
    } else if flags & 0x21 != 0 && pb(0x49) < pb(0x4a) {
        vence_fora(0x2f);
        if mark != 0 {
            *((p + 0x3e) as *mut u8) = penalties;
        }
    } else if flags & 0x21 == 0 {
        let empate = *((league + league::DRAW_POINTS) as *const i8) as i16; // o empate puro
        if casa > -1 {
            increment_u8(row(casa) + 0x21);
            league_add_i16(row(casa) + 0x27, empate);
        }
        if fora > -1 {
            increment_u8(row(fora) + 0x2f);
            league_add_i16(row(fora) + 0x35, empate);
        }
    }
    let totais = |r: usize| -> i16 {
        for (t, f, c) in [(0x11usize, 0x2d, 0x1f), (0x12, 0x2e, 0x20), (0x13, 0x2f, 0x21), (0x14, 0x30, 0x22)] {
            *((r + t) as *mut u8) = (*((r + f) as *const u8)).wrapping_add(*((r + c) as *const u8));
        }
        for (t, f, c) in [(0x15usize, 0x31, 0x23), (0x17, 0x33, 0x25), (0x19, 0x35, 0x27)] {
            let v = core::ptr::read_unaligned((r + f) as *const i16).wrapping_add(core::ptr::read_unaligned((r + c) as *const i16));
            core::ptr::write_unaligned((r + t) as *mut i16, v);
        }
        core::ptr::read_unaligned((r + 0x19) as *const i16)
    };
    for (k, pts) in [(casa, pts_home), (fora, pts_away)] {
        let v = if k > -1 { totais(row(k)) } else { 0 };
        if !pts.is_null() {
            core::ptr::write_unaligned(pts, v);
        }
    }
    if read_usize(table + 5) != 0 {
        let m = n as i32 - 1;
        let (mut ia, mut ib) = (-1i32, -1i32);
        let entry = |linha_: i16, k: i16| read_usize(read_usize(row(linha_) + 5).wrapping_add(14 * k as i32 as usize));
        let mut k: i16 = 0;
        if m > 0 {
            loop {
                if casa > -1 && entry(casa, k) == read_usize(p + 0x20) {
                    ia = k as i32;
                    if ib > -1 {
                        break;
                    }
                }
                if fora > -1 && entry(fora, k) == read_usize(p + 0x1c) {
                    ib = k as i32;
                    if ia > -1 {
                        break;
                    }
                }
                k = k.wrapping_add(1);
                if k as i32 >= m {
                    break;
                }
            }
        }
        let tie: unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8) = core::mem::transmute(FN_LEAGUE_UPDATE_HEAD_TO_HEAD);
        tie(league, table, casa as i32, ia, gc, gf);
        tie(league, table, fora as i32, ib, gf, gc);
    }
}

/// Memory regions 0x685f60 writes: the n table rows, each row's head-to-head records, match +0x3d/+0x3e and the two
/// returned points.
///
/// [pt] As regiões da 0x685f60: as `n` linhas, os confrontos de cada linha ((n − 1) entradas de 14, se a primeira os tem),
/// +0x3d/+0x3e da partida e os dois pontos devolvidos.
pub(crate) unsafe fn result_update_regions(p: usize, table: usize, n: u32, pts_home: *mut i16, pts_away: *mut i16) -> Vec<(usize, usize)> {
    let n = (n as i16).max(0) as usize;
    let mut reg = vec![(p + 0x3d, 2)];
    if table != 0 && n > 0 {
        reg.push((table, RESULT_TABLE_ROW_SIZE * n));
        if read_usize(table + 5) != 0 && n > 1 {
            for i in 0..n {
                let h = read_usize(table + RESULT_TABLE_ROW_SIZE * i + 5);
                if h != 0 {
                    reg.push((h, 14 * (n - 1)));
                }
            }
        }
    }
    for pts in [pts_home, pts_away] {
        if !pts.is_null() {
            reg.push((pts as usize, 2));
        }
    }
    reg
}


/// Returns the points for a result: the win points (+0xc2) to the winner, 0 to the loser; when level, by extra time
/// (+0xab bit 2), by penalties (bits 0x21: the draw points +0xc3 to the winner) or the draw points.
///
/// Ret 8: `League::points_for_result(club, fixture)`: the win points (+0xc2) if the club won (home side +0x1c with
/// +0x43 > +0x44, away side +0x20 with fewer), otherwise 0; level: by extra time (+0xab bit 2, +0x47/+0x48, the win
/// points), by penalties (bits 0x21, +0x49/+0x4a: the draw points +0xc3 to the winner, 0 to the loser) or the draw
/// points. Only al is written: the 3 high bytes of eax are those of the fixture.
/// Original: `League::points_for_result` (0x68aad0, league.cpp, thiscall, ret 8).
///
/// [pt] { 0x68aad0 } (thiscall, ret 8, league.cpp): `League::points_for_result(clube, partida)`: os pontos de vitória
/// (+0xc2) se o clube venceu (mandante +0x1c com +0x43 > +0x44, visitante +0x20 com menos), senão 0; empatado: pela
/// prorrogação (+0xab bit 2, +0x47/+0x48, os de vitória), pelos pênaltis (bits 0x21, +0x49/+0x4a: os de empate +0xc3
/// a quem venceu, 0 a quem perdeu) ou os de empate. Só o al é escrito: os 3 bytes altos do eax são os da partida.
pub(crate) unsafe extern "thiscall" fn points_for_result(league: usize, club: usize, p: usize) -> u32 {
    let pb = |o: usize| *((p + o) as *const i8);
    let (casa, fora) = (read_usize(p + 0x1c), read_usize(p + 0x20));
    let venceu = |a: i8, b: i8| (club == casa && a > b) || (club == fora && a < b);
    let gain = |o: usize| *((league + o) as *const u8);
    let flags = *((league + league::MATCH_RULES) as *const u8);
    let v = if pb(0x43) != pb(0x44) {
        if venceu(pb(0x43), pb(0x44)) { gain(0xc2) } else { 0 } // the points for a win
    } else if flags & 2 != 0 && pb(0x47) != pb(0x48) {
        if venceu(pb(0x47), pb(0x48)) { gain(0xc2) } else { 0 }
    } else if flags & 0x21 != 0 && pb(0x49) != pb(0x4a) {
        if venceu(pb(0x49), pb(0x4a)) { gain(0xc3) } else { 0 }
    } else {
        gain(0xc3)
    };
    (p as u32 & 0xffff_ff00) | v as u32
}

comparar_valor!(comparar_points_for_result, points_for_result, IDX_POINTS_FOR_RESULT, (clube: usize, p: usize) -> u32);



/// qsort callback comparing table rows with head-to-head: the own compare, the points, the club id with no games, the
/// stored head-to-head position (+4) and the name.
///
/// `league_compare_table_rows_h2h(a, b)` (qsort callback of 0x6827d0 under rule 4 and of 0x6471a0 of the Italian Serie
/// B): an empty row ties; the own comparison `[0xb67a00]` when it does not give 0 (called again); the points (+0x19 of
/// column `[[0xb679fc]+0xad]`, b − a); both without matches (+0x11): the club id (a − b); the head-to-head position
/// stored in +4 (b − a); finally the clubs' name (+4, strcmp: −1, 0 or 1).
/// Original: `league_compare_table_rows_h2h` (0x683410, league.cpp, cdecl).
///
/// [pt] { 0x683410 } (cdecl, league.cpp): `league_compare_table_rows_h2h(a, b)` (callback do qsort da 0x6827d0 na regra
/// 4 e da 0x6471a0 da Série B italiana): linha vazia empata; a comparação própria [0xb67a00] quando não dá 0 (chamada
/// de novo); os pontos (+0x19 da coluna [[0xb679fc]+0xad], b − a); os dois sem jogos (+0x11): o id dos clubes (a − b);
/// a posição do confronto guardada em +4 (b − a); por fim o nome dos clubes (+4, strcmp: −1, 0 ou 1).
pub(crate) unsafe extern "C" fn league_compare_table_rows_h2h(a: usize, b: usize) -> i32 {
    let col = read_usize(read_usize(G_LEAGUE_SORT_CURRENT_LEAGUE) + 0xad);
    if read_usize(a) == 0 || read_usize(b) == 0 {
        return 0;
    }
    if read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC) != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC));
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC));
            return f(a, b);
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (read_i16(a + o + 0x19), read_i16(b + o + 0x19));
    if pa != pb {
        return pb as i32 - pa as i32;
    }
    if *((a + o + 0x11) as *const u8) == 0 && *((b + o + 0x11) as *const u8) == 0 {
        return (read_usize(read_usize(a)) as i32).wrapping_sub(read_usize(read_usize(b)) as i32);
    }
    let dif = *((b + 4) as *const i8) as i32 - *((a + 4) as *const i8) as i32; // the tie’s position
    if dif != 0 {
        return dif;
    }
    let (mut x, mut y) = (read_usize(a) + 4, read_usize(b) + 4);
    loop {
        let (cx, cy) = (*(x as *const u8), *(y as *const u8));
        if cx != cy {
            return if cx < cy { -1 } else { 1 };
        }
        if cx == 0 {
            return 0;
        }
        x += 1;
        y += 1;
    }
}


/// bsearch callback: the year (+2, unsigned) and the day (+0): −1, 0 or 1
/// Original: `league_compare_dates` (0x685cf0, league.cpp, cdecl).
///
/// [pt] { 0x685cf0 } (cdecl, league.cpp): `league_compare_dates(a, b)` (callback do bsearch da 0x685640 na tabela de
/// datas da liga): o ano (+2, sem sinal) e depois o dia (+0, com sinal): −1, 0 ou 1.
pub(crate) unsafe extern "C" fn league_compare_dates(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_i16(a + 2) as u16, read_i16(b + 2) as u16);
    if ya < yb { // o ano antes
        return -1;
    }
    if ya > yb {
        return 1;
    }
    let (da, db) = (read_i16(a), read_i16(b));
    if da < db {
        -1
    } else {
        (da > db) as i32
    }
}


/// the club row in the given rows (null: the league table), or 0
/// Original: `League::find_club_row` (0x68ab70, league.cpp, thiscall, ret 8).
///
/// [pt] { 0x68ab70 } (thiscall, ret 8, league.cpp): `League::find_club_row(clube, linhas)`: a linha (0x3b bytes) do
/// clube entre as +0x3e (curto) de `linhas` (nula: a tabela da liga, +0xb1), ou 0.
pub(crate) unsafe extern "thiscall" fn find_club_row(league: usize, club: usize, rows: usize) -> usize {
    let t = if rows == 0 { read_usize(league + league::TABLE) } else { rows };
    let n = read_i16(league + league::CLUB_COUNT);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if read_usize(r) == club { // the club’s row
                return r;
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_find_club_row, find_club_row, IDX_FIND_CLUB_ROW, (clube: usize, linhas: usize) -> usize);

/// 1 when the club has a row in the league table
/// Original: `League::contains_club` (0x68abd0, league.cpp, thiscall, ret 4).
///
/// [pt] { 0x68abd0 } (thiscall, ret 4, league.cpp; virtual +0x80 das ligas): `League::contains_club(clube)`: 1 se o
/// clube tem linha na tabela da liga (+0xb1, +0x3e linhas; a linha no endereço 0 conta como não), senão 0.
pub(crate) unsafe extern "thiscall" fn contains_club(league: usize, club: usize) -> u32 {
    let t = read_usize(league + league::TABLE);
    let n = read_i16(league + league::CLUB_COUNT);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if read_usize(r) == club {
                return (r != 0) as u32; // a liga tem o clube
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_contains_club, contains_club, IDX_CONTAINS_CLUB, (clube: usize) -> u32);




/// Linhas da tabela: 0x3b bytes; +0 clube, +4 posição, +5 confrontos diretos (entradas de 14 bytes), +9 histórico
/// das posições (um byte por rodada), +0x11 jogos e, a cada 14 bytes por coluna (+0xad), os pontos em +0x19.
pub(crate) const LEAGUE_TABLE_ROW_SIZE: usize = 0x3b;

/// Sorts the table by the rules (+0xc5): rule 6 only sorts (0x683210); rule 4 builds the mini table and, when first,
/// sorts by head-to-head (0x683410) and bubbles the ties; stores the position (+4) and the position history (+9);
/// returns 1.
///
/// `League::sort_table(rows)`: sorts the table (`rows` null: the league's, +0xb1). First fixes total rounds +0xa7 (−1:
/// (+0x3e − 1) × +0x3c, with clubs and +0x45). Rule 6 (+0xc5, without column +0xad) only sorts (qsort 0x945333 with
/// 0x683210); rule 4 with head-to-head builds the mini table (0x682b30) and, being the first rule, sorts with 0x683410
/// and runs a bubble pass over the league rows (+0xb1) while there are swaps: a tie on points (or on the own comparison
/// +0xa3) decided by 0x683210; otherwise qsort with 0x683210. Finally (+0x44 ≠ 0xfe, without column) stores the
/// position in +4 and in history +9 (round +0x11 up to +0xa7). Keeps the league in `[0xb679fc]` and the own comparison
/// in `[0xb67a00]`. Returns 1. No SEH, no random draw.
/// Original: `League::sort_table` (0x6827d0, league.cpp, thiscall).
///
/// [pt] { 0x6827d0 } (thiscall, league.cpp): `League::sort_table(linhas)`: ordena a tabela (`linhas` nula: a da liga,
/// +0xb1). Antes acerta o total de rodadas +0xa7 (−1: (+0x3e − 1) × +0x3c, com clubes e +0x45). A regra 6 (+0xc5,
/// sem coluna +0xad) só ordena (qsort 0x945333 com 0x683210); a regra 4 com confrontos monta a minitabela
/// (0x682b30) e, sendo a primeira regra, ordena com 0x683410 e passa bolha nas linhas da liga (+0xb1) enquanto
/// trocar: empate em pontos (ou na comparação própria +0xa3) decidido pela 0x683210; senão qsort com 0x683210.
/// Por fim (+0x44 ≠ 0xfe, sem coluna) grava a posição em +4 e no histórico +9 (rodada +0x11 até +0xa7). Guarda a
/// liga em [0xb679fc] e a comparação própria em [0xb67a00]. Devolve 1. Sem SEH, sem sorteio.
pub(crate) unsafe extern "thiscall" fn sort_table(league: usize, rows: usize) -> u32 {
    let n = || read_i16(league + league::CLUB_COUNT);
    let rows = if rows == 0 { read_usize(league + league::TABLE) } else { rows };
    if read_i16(league + league::GAMES_PER_CLUB) == -1 && n() > 0 && read_usize(league + league::ACTIVE) != 0 {
        core::ptr::write_unaligned((league + league::GAMES_PER_CLUB) as *mut i16, (n() - 1).wrapping_mul(read_i16(league + league::LEGS)));
    }
    if rows == 0 {
        return 1;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let order: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_LEAGUE_COMPARE_TABLE_ROWS);
    *(G_LEAGUE_SORT_CURRENT_LEAGUE as *mut usize) = league;
    let column = read_usize(league + league::TABLE_COLUMN);
    let mut so_sort = false;
    let mut tie = false;
    for k in 0..4usize {
        let rule = read_u8(league + league::TIEBREAKS + k);
        if column == 0 && rule == 6 {
            so_sort = true;
            break;
        }
        if rule == 0 {
            break;
        }
        if rule == 4 && n() != 0 && read_usize(rows + 5) != 0 {
            tie = k == 0;
            let mini: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_RANK_BY_HEAD_TO_HEAD);
            mini(league, rows);
            break;
        }
    }
    if so_sort {
        qsort(rows, n() as i32, LEAGUE_TABLE_ROW_SIZE as u32, 0x683210);
    } else {
        *(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC as *mut usize) = read_usize(league + league::CUSTOM_COMPARE);
        if tie {
            qsort(rows, n() as i32, LEAGUE_TABLE_ROW_SIZE as u32, 0x683410);
            loop {
                let mut trocou = false;
                if n() as i32 - 1 <= 0 {
                    break;
                }
                let mut i: i8 = 0;
                loop {
                    let (oa, ob) = (LEAGUE_TABLE_ROW_SIZE * i as i32 as usize, LEAGUE_TABLE_ROW_SIZE * (i as i32 + 1) as usize);
                    let (a, b) = (read_usize(league + league::TABLE) + oa, read_usize(league + league::TABLE) + ob);
                    let col = read_usize(*(G_LEAGUE_SORT_CURRENT_LEAGUE as *const usize) + 0xad);
                    let decidir = read_usize(a) == 0 || read_usize(b) == 0 || is_sort_tie(a, b, col);
                    if decidir && order(read_usize(league + league::TABLE) + oa, read_usize(league + league::TABLE) + ob) > 0 {
                        let t = read_usize(league + league::TABLE);
                        core::ptr::swap_nonoverlapping((t + oa) as *mut u8, (t + ob) as *mut u8, LEAGUE_TABLE_ROW_SIZE); // the row swap
                        trocou = true;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n() as i32 - 1 {
                        break;
                    }
                }
                if !trocou {
                    break;
                }
            }
        } else {
            qsort(rows, n() as i32, LEAGUE_TABLE_ROW_SIZE as u32, 0x683210);
        }
    }
    if read_u8(league + league::STAGE_INDEX) != 0xfe && read_usize(league + league::TABLE_COLUMN) == 0 && n() > 0 {
        let mut c: i8 = 0;
        loop {
            let r = rows + LEAGUE_TABLE_ROW_SIZE * c as i32 as usize;
            *((r + 4) as *mut i8) = c; // the position
            let hist = read_usize(r + 9);
            let round = read_u8(r + 0x11) as i8;
            if hist != 0 && round > 0 && round as i16 <= read_i16(league + league::GAMES_PER_CLUB) {
                *((hist + round as usize - 1) as *mut i8) = c;
            }
            c = c.wrapping_add(1);
            if c as i16 >= n() {
                break;
            }
        }
    }
    1
}

/// Whether the 0x6827d0 bubble sort treats rows a and b as tied in 0x683210: custom compare func, else points (+0x19),
/// club ids without games, or head-to-head goals.
///
/// [pt] A bolha da 0x6827d0 decide o par (a, b) pela 0x683210? Só quando empatam: na comparação própria [0xb67a00]
/// (chamada de novo quando dá diferente de 0) ou, sem ela, nos pontos (+0x19 da coluna) e, com os dois sem jogos
/// (+0x11), no id dos clubes; com jogos, no confronto direto (gols +5/+7 e +8/+0xa da entrada do clube b).
pub(crate) unsafe fn is_sort_tie(a: usize, b: usize, col: usize) -> bool {
    let propria = *(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC as *const usize));
            return f(a, b) == 0;
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (read_i16(a + o + 0x19), read_i16(b + o + 0x19));
    if pa != pb {
        return false;
    }
    if read_u8(a + o + 0x11) == 0 && read_u8(b + o + 0x11) == 0 {
        let (ca, cb) = (read_usize(a), read_usize(b));
        if ca == 0 || cb == 0 {
            return true;
        }
        return *(ca as *const i32) == *(cb as *const i32);
    }
    let m = read_i16(*(G_LEAGUE_SORT_CURRENT_LEAGUE as *const usize) + 0x3e) as i32 - 1;
    if m <= 0 {
        return true;
    }
    let (h, cb) = (read_usize(a + 5), read_usize(b));
    let mut k: i16 = 0;
    loop {
        let e = h.wrapping_add(14 * k as i32 as usize);
        if read_usize(e) == cb {
            if e == 0 {
                return true;
            }
            return read_u8(e + 5) == read_u8(e + 7) && read_i16(e + 8) == read_i16(e + 0xa);
        }
        k = k.wrapping_add(1);
        if k as i32 >= m {
            return true;
        }
    }
}

/// Memory regions of the table sort: the rows (requested and league's), total rounds, the two sort globals and each
/// row's position history.
///
/// [pt] As regiões da ordenação: as linhas pedidas e as da liga, o total de rodadas, as duas globais e o histórico de
/// posições de cada linha (o byte gravado vai até a rodada +0x11, um byte com sinal: no máximo 127).
pub(crate) unsafe fn sort_regions(league: usize, rows: usize) -> Vec<(usize, usize)> {
    let n = read_i16(league + league::CLUB_COUNT).max(0) as usize;
    let rows = if rows == 0 { read_usize(league + league::TABLE) } else { rows };
    let mut reg = vec![(league + league::GAMES_PER_CLUB, 2), (G_LEAGUE_SORT_CURRENT_LEAGUE, 4), (G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC, 4)];
    if rows == 0 {
        return reg;
    }
    reg.push((rows, LEAGUE_TABLE_ROW_SIZE * n));
    let propria = read_usize(league + league::TABLE);
    if propria != 0 && propria != rows {
        reg.push((propria, LEAGUE_TABLE_ROW_SIZE * n));
    }
    let mut total = read_i16(league + league::GAMES_PER_CLUB);
    if total == -1 && n > 0 && read_usize(league + league::ACTIVE) != 0 {
        total = (n as i16 - 1).wrapping_mul(read_i16(league + league::LEGS));
    }
    let size = total.clamp(0, 127) as usize;
    if size > 0 {
        for i in 0..n {
            let hist = read_usize(rows + LEAGUE_TABLE_ROW_SIZE * i + 9);
            if hist != 0 {
                reg.push((hist, size));
            }
        }
    }
    reg
}


/// Builds the head-to-head mini table of the clubs level on points with their summed head-to-head results, sorted by
/// 0x6834f0; the position (+4) starts at the club count and drops at each separated pair.
///
/// `League::build_h2h_table(rows)` (unnamed in the community map): a mini table (calloc 0x945501 of +0x3e rows) with
/// the club of each row and, for each other club with the same points (+0x19), the sum of the head-to-head against it
/// (entry of +5: bytes +4..+7 into +0x11..+0x14, words +8/+0xa/+0xc into +0x15/+0x17/+0x19); sorts with 0x6834f0 and
/// writes in +4 of each row the position, which starts at the number of clubs and drops at each pair of the mini table
/// that 0x6834f0 separates. Frees the mini table (0x9452ca). Out of memory: the original routine (the league.cpp box).
/// TapeCall through 0x6827d0 under rule 4.
/// Original: `League::build_h2h_table` (0x682b30, league.cpp, thiscall).
///
/// [pt] { 0x682b30 } (thiscall, league.cpp): `League::build_h2h_table(linhas)` (sem nome no mapa da comunidade): uma
/// minitabela (calloc 0x945501 de +0x3e linhas) com o clube de cada linha e, para cada outro clube com os mesmos
/// pontos (+0x19), a soma do confronto direto contra ele (entrada de +5: bytes +4..+7 em +0x11..+0x14, palavras
/// +8/+0xa/+0xc em +0x15/+0x17/+0x19); ordena com 0x6834f0 e grava em +4 de cada linha a posição, que começa no
/// número de clubes e desce a cada par da minitabela que a 0x6834f0 separa. Solta a minitabela (0x9452ca). Sem
/// memória: a original (a caixa do league.cpp). TapeCall pela 0x6827d0 na regra 4.
pub(crate) unsafe extern "thiscall" fn build_h2h_table(league: usize, rows: usize) {
    let n = || read_i16(league + league::CLUB_COUNT);
    let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let t = calloc(n() as i32, LEAGUE_TABLE_ROW_SIZE as u32);
    if t == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_BUILD_H2H_TABLE)(league, rows); // message box: original only
    }
    let row = |base: usize, i: i16| base + LEAGUE_TABLE_ROW_SIZE * i as i32 as usize;
    let mut i: i16 = 0;
    if n() > 0 {
        loop {
            let (ri, mi) = (row(rows, i), row(t, i));
            *(mi as *mut usize) = read_usize(ri);
            let mut j: i16 = 0;
            if n() > 0 {
                loop {
                    let rj = row(rows, j);
                    if i != j && read_i16(ri + 0x19) == read_i16(rj + 0x19) {
                        let m = n() as i32 - 1;
                        let (h, cj) = (read_usize(ri + 5), read_usize(rj));
                        let mut k: i16 = 0;
                        while (k as i32) < m {
                            let e = h + 14 * k as i32 as usize;
                            if read_usize(e) == cj {
                                for (de, para) in [(4, 0x11), (5, 0x12), (6, 0x13), (7, 0x14)] {
                                    *((mi + para) as *mut u8) = read_u8(mi + para).wrapping_add(read_u8(e + de)); // o confronto somado
                                }
                                for (de, para) in [(8, 0x15), (0xa, 0x17), (0xc, 0x19)] {
                                    let v = read_i16(mi + para).wrapping_add(read_i16(e + de));
                                    core::ptr::write_unaligned((mi + para) as *mut i16, v);
                                }
                                break;
                            }
                            k = k.wrapping_add(1);
                        }
                    }
                    j = j.wrapping_add(1);
                    if j >= n() {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
            if i >= n() {
                break;
            }
        }
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(t, n() as i32, LEAGUE_TABLE_ROW_SIZE as u32, 0x6834f0);
    let separa: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_LEAGUE_COMPARE_H2H_ROWS);
    let mut pos = n() as u8;
    let mut a: i16 = 0;
    if n() > 0 {
        loop {
            let club = read_usize(row(t, a));
            let mut c: i16 = 0;
            while c < n() {
                if club == read_usize(row(rows, c)) {
                    *((row(rows, c) + 4) as *mut u8) = pos;
                    break;
                }
                c = c.wrapping_add(1);
            }
            if (a as i32 + 1) < n() as i32 && separa(row(t, a), row(t, a + 1)) != 0 {
                pos = pos.wrapping_sub(1); // the position goes down
            }
            a = a.wrapping_add(1);
            if a >= n() {
                break;
            }
        }
    }
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(t);
}


/// Frees the +5 and +9 row buffers and the table (null: the league's, then clears +0x3e and +0xb1).
///
/// `League::free_table(table)`: a null `table` = the league's (+0xb1). With clubs (+0x3e ≥ 1) and a table: frees
/// (0x9452ca) the +5 vectors of each row (if the first row's is not null), then the +9 ones (likewise) and the table;
/// if it is the league's, zeroes +0x3e and +0xb1. No SEH, no random draw.
/// Original: `League::free_table` (0x687970, league.cpp, thiscall).
///
/// [pt] { 0x687970 } (thiscall, league.cpp): `League::free_table(tabela)`: `tabela` nula = a da liga (+0xb1). Com clubes
/// (+0x3e ≥ 1) e tabela: solta (0x9452ca) os vetores +5 de cada linha (se o da primeira não é nulo), depois os +9
/// (idem) e a tabela; sendo a da liga, zera +0x3e e +0xb1. Sem SEH, sem sorteio.
pub(crate) unsafe extern "thiscall" fn free_table(league: usize, table: usize) {
    let (t, propria) = if table == 0 { (read_usize(league + league::TABLE), true) } else { (table, false) };
    if read_i16(league + league::CLUB_COUNT) < 1 || t == 0 {
        return;
    }
    crate::comparison::branch("liga liberar tabela");
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    for campo in [5usize, 9] {
        if read_usize(t + campo) != 0 {
            let mut i = 0usize;
            while (i as i32) < read_i16(league + league::CLUB_COUNT) as i32 {
                let p = read_usize(t + campo + LEAGUE_TABLE_ROW_SIZE * i);
                if p != 0 {
                    free(p);
                }
                i += 1;
            }
        }
    }
    free(t);
    if propria { // the league one: clears the count and the table
        core::ptr::write_unaligned((league + league::CLUB_COUNT) as *mut i16, 0);
        core::ptr::write_unaligned((league + league::TABLE) as *mut usize, 0);
    }
}

/// Comparador: liberar duas vezes soltaria os mesmos blocos duas vezes.
// simplification: original only (like the releases of the 232nd); the proof is the career and the copy’s branch in Rust mode.

/// Returns 1 when every club has played the +0xa7 rounds (and the stages, if asked); at the end of the league, the
/// records (0x86a070), the prize money (0x682e10), the notice (+0x94) and +0xdd.
///
/// `League::check_complete(stages, notify)`: 0 without a table, closed (+0xb5) or without +0x45. Outside the single
/// stage (+0x44 ≠ 0xff) already complete (+0xdd): 1. Not complete: fixes total rounds +0xa7 (−1) and, if some club has
/// not yet played them all (+0x11 < +0xa7), 0; otherwise, in league divisions (0x4b0700) with +0xa9 > 8 and without
/// +0xe6, the end-of-league records (0x86a070), and without +0xe6 the prizes (0x682e10). With `stages` = 1 in the
/// single stage: each stage of +0xc (0..=+0x30) must be complete (virtual +0x10 with 0, 1) and +0x30 ≥ +0x2c − 1,
/// otherwise 0. Marks +0xdd (before that, with `notify`, virtual +0x94) and returns 1. No SEH, no random draw.
/// Original: `League::check_complete` (0x689ad0, league.cpp, thiscall).
///
/// [pt] { 0x689ad0 } (thiscall, league.cpp): `League::check_complete(fases, avisar)`: 0 sem tabela, encerrada (+0xb5) ou
/// sem +0x45. Fora da fase única (+0x44 ≠ 0xff) já completa (+0xdd): 1. Não completa: acerta o total de rodadas
/// +0xa7 (−1) e, se algum clube ainda não jogou todas (+0x11 < +0xa7), 0; senão, nas divisões de liga (0x4b0700)
/// com +0xa9 > 8 e sem +0xe6, os registros de fim de liga (0x86a070), e sem +0xe6 os prêmios (0x682e10). Com
/// `fases` = 1 na fase única: cada fase de +0xc (0..=+0x30) tem de estar completa (virtual +0x10 com 0, 1) e
/// +0x30 ≥ +0x2c − 1, senão 0. Marca +0xdd (antes, com `avisar`, a virtual +0x94) e devolve 1. Sem SEH, sem sorteio.
pub(crate) unsafe extern "thiscall" fn league_check_complete(league: usize, stages: u32, warn: u32) -> u8 {
    let table = read_usize(league + league::TABLE);
    if table == 0 || read_u8(league + league::STOPPED) != 0 || read_usize(league + league::ACTIVE) == 0 {
        return 0;
    }
    let unica = read_u8(league + league::STAGE_INDEX) == 0xff;
    if read_usize(league + league::COMPLETE) == 0 {
        if read_i16(league + league::GAMES_PER_CLUB) == -1 && read_i16(league + league::CLUB_COUNT) > 0 {
            core::ptr::write_unaligned((league + league::GAMES_PER_CLUB) as *mut i16, (read_i16(league + league::CLUB_COUNT) - 1).wrapping_mul(read_i16(league + league::LEGS)));
        }
        let rounds = read_i16(league + league::GAMES_PER_CLUB);
        for i in 0..read_i16(league + league::CLUB_COUNT).max(0) as usize {
            if ((read_u8(table + LEAGUE_TABLE_ROW_SIZE * i + 0x11) as i8) as i16) < rounds { // ainda falta jogar
                return 0;
            }
        }
        crate::comparison::branch("liga completa: fim da liga");
        let division: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
        let id = *(read_usize(league + 4) as *const u32);
        let mut premios = true;
        if division(id) != 0 && read_i16(league + league::ROUND_COUNT) > 8 {
            if read_usize(league + league::TIEBREAK_MATCHES) != 0 {
                premios = false;
            } else {
                let records: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_FINISH_RECORDS);
                records(*(G_STAFF_RECORDS as *const usize), league);
            }
        }
        if premios && read_usize(league + league::TIEBREAK_MATCHES) == 0 {
            let pay: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_PAY_PRIZE_MONEY);
            pay(league);
        }
    } else if !unica {
        return 1;
    }
    if stages == 1 && read_u8(league + league::STAGE_INDEX) == 0xff {
        crate::comparison::branch("liga completa: fases");
        let mut k = 0usize;
        if read_usize(league + league::CURRENT_STAGE) as i32 >= 0 {
            loop {
                let stage = read_usize(read_usize(league + league::STAGES) + 4 * k);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(stage) + 0x10));
                if completa(stage, 0, 1) == 0 {
                    return 0;
                }
                k += 1;
                if k as i32 > read_usize(league + league::CURRENT_STAGE) as i32 {
                    break;
                }
            }
        }
        if (read_usize(league + league::CURRENT_STAGE) as i32) < (read_usize(league + league::STAGE_COUNT) as i32).wrapping_sub(1) {
            return 0;
        }
    }
    if read_usize(league + league::COMPLETE) == 0 {
        if warn != 0 {
            let end: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league) + 0x94));
            end(league);
        }
        core::ptr::write_unaligned((league + league::COMPLETE) as *mut u32, 1);
    }
    1
}

/// True if 0x689ad0 takes a read-only path (no table, finished, no +0x45, complete outside single stage, or some club
/// short of games).
///
/// [pt] O caminho da 0x689ad0 que só lê (e acerta +0xa7)? Escrito à parte da cópia: sem tabela, encerrada ou sem +0x45;
/// completa fora da fase única, ou na única sem pedir as fases; ou algum clube sem todos os jogos.
pub(crate) unsafe fn is_complete_check_read_only(league: usize, stages: u32) -> bool {
    let table = read_usize(league + league::TABLE);
    if table == 0 || read_u8(league + league::STOPPED) != 0 || read_usize(league + league::ACTIVE) == 0 {
        return true;
    }
    if read_usize(league + league::COMPLETE) != 0 {
        return read_u8(league + league::STAGE_INDEX) != 0xff || stages != 1;
    }
    let n = read_i16(league + league::CLUB_COUNT);
    let mut total = read_i16(league + league::GAMES_PER_CLUB);
    if total == -1 && n > 0 {
        total = (n - 1).wrapping_mul(read_i16(league + league::LEGS));
    }
    (0..n.max(0) as usize).any(|i| ((read_u8(table + 0x3b * i + 0x11) as i8) as i16) < total)
}




/// Pays TV revenue (FN_FINANCE_ADD_TV_REVENUE) to each league table club (optionally filtered by last division), with
/// the TV rights news unless disabled.
///
/// [pt] O pagamento da 0x683010 (`noticia` falso: sem a notícia da 0x48ea40, para o comparador).
pub(crate) unsafe fn pay_tv_money(league: usize, value: i32, filtro: usize, news: bool) {
    let finances = *(G_FINANCE_TABLE as *const usize);
    if finances == 0 || read_i16(league + league::CLUB_COUNT) <= 0 {
        return;
    }
    crate::comparison::branch("liga direitos de tv");
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let add_tv: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_ADD_TV_REVENUE);
    let mut i: i16 = 0;
    loop {
        let row = read_usize(league + league::TABLE) + 0x3b * i as i32 as usize;
        let club = read_usize(row);
        if filtro == 0 || read_usize(club + club::LAST_DIVISION) == filtro {
            add_tv(get_club_finance(*(G_FINANCE_TABLE as *const usize), club), value); // the TV money
            if news {
                let warn: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_COMP_SEND_TV_RIGHTS_NEWS);
                warn(league, read_usize(read_usize(league + league::TABLE) + 0x3b * i as i32 as usize), value);
            }
        }
        i = i.wrapping_add(1);
        if i >= read_i16(league + league::CLUB_COUNT) {
            break;
        }
    }
}

/// Pays TV rights: adds the amount to the TV revenue of every club in the table (with a filter, only those with +0x5b
/// equal to it) and sends the news (0x48ea40).
///
/// `League::pay_tv_rights(amount, filter)`: with the finance table `[0xae2a38]`, for each club in the table (+0xb1;
/// with `filter`, only those whose +0x5b equals it) adds `amount` to the TV revenue (0x5a0590 → 0x5987c0) and sends the
/// news (Comp::send_tv_rights_news 0x48ea40, only for a club with a human manager). No SEH, no random draw, no locals.
/// Original: `League::pay_tv_rights` (0x683010, league.cpp, thiscall).
///
/// [pt] { 0x683010 } (thiscall, league.cpp): `League::pay_tv_rights(valor, filtro)`: com a tabela de finanças
/// [0xae2a38], a cada clube da tabela (+0xb1; com `filtro`, só os de +0x5b igual a ele) soma `valor` à receita de TV
/// (0x5a0590 → 0x5987c0) e manda a notícia (Comp::send_tv_rights_news 0x48ea40, só para clube de técnico humano).
/// Sem SEH, sem sorteio, sem locais.
pub(crate) unsafe extern "thiscall" fn pay_tv_rights(league: usize, value: i32, filtro: usize) {
    pay_tv_money(league, value, filtro, true)
}


/// Records the season honours: champion, runner-up and third into the history (0x4afce0), with the host nation's
/// national side as the featured entry when the competition asks (+0x88, 0x5fa730, 0x53b3d0); a champion of this
/// division clears +0x5b of the clubs pointing at it.
///
/// `League::record_season_honours()`: champion, runner-up and (with more than 2 clubs) third of the table (+0xb1). If
/// the champion's division (+0x57) is this competition (+4), zeroes in the clubs (`[0xae23b4]`, 0x245 bytes,
/// `[0xae235c]` − `[0xae2350]`) the +0x5b that points at it. If virtual +0x88 of the competition's league
/// (`[0xadadfc][id]`) asks, the featured entry is the national side (0x53b3d0) of the host country
/// (HostCountry::find_hosts 0x5fa730 with year +0x40): with two hosts, the one with the higher +0x8e (tie: the first).
/// Writes into the history (0x4afce0) and returns what it returns. No SEH, no random draw; the two host locals are only
/// read when 0x5fa730 writes them.
/// Original: `League::record_season_honours` (0x684640, league.cpp, thiscall).
///
/// [pt] { 0x684640 } (thiscall, league.cpp): `League::record_season_honours()`: campeão, vice e (com mais de 2 clubes)
/// terceiro da tabela (+0xb1). Se a divisão do campeão (+0x57) é esta competição (+4), zera nos clubes ([0xae23b4],
/// 0x245 bytes, [0xae235c] − [0xae2350]) o +0x5b que aponta para ela. Se a virtual +0x88 da liga da competição
/// ([0xadadfc][id]) pede, o destaque é a seleção (0x53b3d0) do país-sede (HostCountry::find_hosts 0x5fa730 com o ano
/// +0x40): com duas sedes, a de maior +0x8e (empate: a primeira). Grava no histórico (0x4afce0) e devolve o que ela
/// devolve. Sem SEH, sem sorteio; as duas locais das sedes só são lidas quando a 0x5fa730 as grava.
pub(crate) unsafe extern "thiscall" fn league_record_season_honours(league: usize) -> usize {
    crate::comparison::branch("liga honras da temporada");
    let table = read_usize(league + league::TABLE);
    let runner_up = read_usize(table + 0x3b);
    let champion = read_usize(table);
    let terceiro = if read_i16(league + league::CLUB_COUNT) > 2 { read_usize(table + 0x76) } else { 0 };
    if read_usize(champion + club::DIVISION) == read_usize(league + 4) {
        let how_many = || (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32));
        let mut k = 0i32;
        while k < how_many() {
            let p = (*(G_CLUB_TABLE as *const usize)).wrapping_add(0x245 * k as usize + 0x5b);
            if read_usize(p) == read_usize(league + 4) {
                core::ptr::write_unaligned(p as *mut usize, 0); // the previous division
            }
            k += 1;
        }
    }
    let comp = read_usize(league + 4);
    let target = *((*(G_COMP_OBJECT_TABLE as *const usize)).wrapping_add(4 * *(comp as *const u32) as usize) as *const usize);
    let pede: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(target) + 0x88));
    let mut destaque = 0usize;
    if pede(target) != 0 {
        let sedes: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
        let national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
        let (mut a, mut b) = (0i32, 0i32);
        let q = sedes(*(G_HOST_COUNTRY as *const usize), *(read_usize(league + 4) as *const u32), read_i16(league + league::YEAR) as u16 as u32, &mut a, &mut b, 0);
        let nation = |k: i32| (*(G_NATION_TABLE as *const usize)).wrapping_add(k.wrapping_mul(0x122) as usize);
        if q == 2 {
            destaque = if read_i16(nation(a) + 0x8e) < read_i16(nation(b) + 0x8e) { national_team(nation(b)) } else { national_team(nation(a)) };
        } else if q != 0 {
            destaque = national_team(nation(a));
        }
    }
    let register: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_COMP_UTIL_ADD_HISTORY_RECORD);
    register(league, champion, runner_up, terceiro, destaque) // champion, runner-up, third
}

/// Memory regions of the club competition history: count, pointer and the 26-byte rows.
///
/// [pt] As regiões do histórico de competições: contagem, ponteiro e as linhas (26 bytes).
pub(crate) unsafe fn comp_history_regions() -> Vec<(usize, usize)> {
    let n = (*(G_CLUB_COMP_HISTORY_COUNT as *const i32)).max(0) as usize;
    vec![(0xae237c, 4), (G_CLUB_COMP_HISTORY_TABLE, 4), (*(G_CLUB_COMP_HISTORY_TABLE as *const usize), 26 * n)]
}


/// Next free row of the club competition history after the last filled one (+4 competition); 0 if none filled or the
/// table is full.
///
/// [pt] A linha livre do histórico de competições ([0xae23d4], [0xae237c] linhas de 26 bytes): a seguinte à última com
/// +4 (competição) preenchido; nenhuma preenchida, ou a última preenchida: 0 (falta crescer).
pub(crate) unsafe fn free_comp_history_row() -> usize {
    let n = *(G_CLUB_COMP_HISTORY_COUNT as *const i32);
    let t = *(G_CLUB_COMP_HISTORY_TABLE as *const usize);
    let mut i = n - 1;
    while i >= 0 {
        if read_usize(t.wrapping_add(26 * i as usize + 4)) != 0 {
            break;
        }
        i -= 1;
    }
    if i < 0 || i >= n - 1 {
        return 0;
    }
    t.wrapping_add(26 * (i + 1) as usize)
}



/// whether the league has relegation places at the bottom of the table
/// Original: `League::has_relegation` (0x6847b0, league.cpp, thiscall).
///
/// [pt] { 0x6847b0 } (thiscall, league.cpp): 1 se a liga tem vagas de rebaixamento (byte +0xc1 não nulo).
pub(crate) unsafe extern "thiscall" fn has_relegation(league: usize) -> u32 {
    (*((league + league::RELEGATED_COUNT) as *const u8) != 0) as u32 // tem rebaixamento
}

/// Stores in a league schedule round record the date with a relative year, the code and a value.
///
/// The date (Date::Date, 0x549f70, into a local Date) from `day`/`month`/(`years` + `base`)/`week` in record `i`
/// (signed word, 0x41 bytes) of `rounds`: day of the year in word +0, year − `base` in word +2, `code` in byte +4 and
/// `value` in +0x3d. Returns the record.
/// Original: `league_round_set_date` (0x68a160, league.cpp, cdecl).
///
/// [pt] { 0x68a160 } (cdecl, league.cpp): a data (Date::Date, 0x549f70, num Date local) de `dia`/`mes`/
/// (`anos` + `base`)/`semana` no registro `i` (palavra com sinal, 0x41 bytes) de `rodadas`: dia do ano
/// na palavra +0, ano − `base` na palavra +2, `codigo` no byte +4 e `valor` em +0x3d. Devolve o registro.
pub(crate) unsafe extern "C" fn league_round_set_date(rounds: usize, i: u32, day: u32, month: u32, years: u32, week: u32, code: u32, base: u32, value: u32) -> usize {
    let construct: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u16; 4];
    construct(&mut d, day, month, years.wrapping_add(base), week);
    let r = rounds.wrapping_add((i as i16 as isize * 0x41) as usize);
    core::ptr::write_unaligned(r as *mut u16, d[0]);
    core::ptr::write_unaligned((r + 2) as *mut u16, d[1].wrapping_sub(base as u16)); // ano relativo
    *((r + 4) as *mut u8) = code as u8;
    core::ptr::write_unaligned((r + 0x3d) as *mut u32, value);
    r
}

/// Returns the league match importance for the club: the league base, +1 early in the season or with the final-day
/// margin.
///
/// Ret 0xc: the importance of the `fixture` for the club on `side` (0 = home, +0x1c; otherwise +0x20): (byte +0x51 of
/// the league − 10) / 4 + 10. Writes 0xff into `out`; with the club in the table (+0xb1, 59-byte rows, +0x3e clubs) and
/// more than 10 rounds (+0xa9): +1 with +0x34 of the fixture below 3; otherwise, a row with no mark (+0x37 = 0xff), +1
/// and the last-day margin (0x68a4d0, which can change the importance) in `out`. Only al is returned (the caller uses
/// al).
/// Original: `League::get_match_importance` (0x68a3e0, league.cpp, thiscall).
///
/// [pt] { 0x68a3e0 } (thiscall, ret 0xc, league.cpp): a importância do jogo da `partida` para o clube do
/// `lado` (0 = casa, +0x1c; senão +0x20): (byte +0x51 da liga − 10) / 4 + 10. Grava 0xff em `saida`;
/// com o clube na tabela (+0xb1, linhas de 59 bytes, +0x3e clubes) e mais de 10 rodadas (+0xa9): +1
/// com +0x34 da partida abaixo de 3; senão, linha sem marca (+0x37 = 0xff), +1 e a margem do último
/// dia (0x68a4d0, que pode trocar a importância) em `saida`. Só o al é devolvido (quem chama usa al).
pub(crate) unsafe extern "thiscall" fn get_match_importance(league: usize, partida: usize, side: u32, out: *mut u8) -> u8 {
    let margem: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(FN_LEAGUE_FINAL_DAY_MARGIN_NEEDED);
    *out = 0xff;
    let club = *((partida + if side as u8 == 0 { 0x1c } else { 0x20 }) as *const usize);
    let mut imp = ((*((league + league::IMPORTANCE) as *const i8) as i32 - 10) / 4) as u8;
    imp = imp.wrapping_add(10); // base importance
    let clubs = core::ptr::read_unaligned((league + league::CLUB_COUNT) as *const i16);
    let row = |i: i16| (*((league + league::TABLE) as *const usize)).wrapping_add((i as isize * 59) as usize);
    // simplification: the original tests i == −1 after the search; i starts at 0 and stops before `clubes`, it is never −1
    let mut i = 0i16;
    loop {
        if i >= clubs {
            return imp;
        }
        if *(row(i) as *const usize) == club {
            break;
        }
        i = i.wrapping_add(1);
    }
    if core::ptr::read_unaligned((league + league::ROUND_COUNT) as *const i16) <= 10 {
        return imp;
    }
    if core::ptr::read_unaligned((partida + 0x34) as *const i16) < 3 {
        return imp.wrapping_add(1);
    }
    let r = row(i);
    if *((r + 0x37) as *const u8) != 0xff {
        return imp;
    }
    let mut local = imp.wrapping_add(1);
    *out = margem(league, r, &mut local);
    local
}




/// The qsort comparison of the table rows: points, the league tie-break rules and the club id.
///
/// Compares table rows `a` and `b` (`[+0]` club). Current league in `[0xb679fc]`, the table column in +0xad (14 bytes
/// per column in the rows). Row without a club: 0. With the private comparison in `[0xb67a00]` and its answer non-zero:
/// it again. Otherwise points (+0x19) descending; both without matches (+0x11): by club id. Then the league's four
/// criteria (+0xc5): 1 goal difference, 2 goals for, 3 byte +0x12, 4 byte +4, 5 word +0x31 (in column 0, criterion 6 =
/// byte +4 and stop); in column 0 with both at the league's number of matches (+0xa7), the id in +0x69 of the clubs;
/// finally the club id.
/// Original: `league_compare_table_rows` (0x683210, league.cpp, cdecl).
///
/// [pt] { 0x683210 } (cdecl, league.cpp): comparação do qsort das linhas da tabela (`a` e `b`: [+0] clube).
/// Liga corrente em [0xb679fc], a coluna da tabela em +0xad (14 bytes por coluna nas linhas). Linha
/// sem clube: 0. Com a comparação particular em [0xb67a00] e resposta dela não nula: ela de novo. Senão
/// pontos (+0x19) decrescentes; sem jogos (+0x11) dos dois, pelo id do clube. Depois os quatro
/// critérios da liga (+0xc5): 1 saldo, 2 gols pró, 3 byte +0x12, 4 byte +4, 5 palavra +0x31 (na
/// coluna 0, critério 6 = byte +4 e fim); na coluna 0 com os dois no número de jogos da liga (+0xa7), o
/// id em +0x69 dos clubes; por fim o id do clube.
pub(crate) unsafe extern "C" fn league_compare_table_rows(a: usize, b: usize) -> i32 {
    let league = *(G_LEAGUE_SORT_CURRENT_LEAGUE as *const usize);
    let column = core::ptr::read_unaligned((league + league::TABLE_COLUMN) as *const i32);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return 0;
    }
    let propria = *(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC as *const usize));
            return f(a, b);
        }
    }
    let c = (column as usize).wrapping_mul(14);
    let w = |r: usize, o: usize| core::ptr::read_unaligned(r.wrapping_add(c).wrapping_add(o) as *const i16) as i32;
    let y = |r: usize, o: usize| *(r.wrapping_add(c).wrapping_add(o) as *const i8) as i32;
    let (pa, pb) = (w(a, 0x19), w(b, 0x19));
    if pa != pb {
        return pb - pa; // pontos
    }
    let id = |r: usize| *(*(r as *const usize) as *const i32);
    if y(a, 0x11) == 0 && y(b, 0x11) == 0 {
        return id(a).wrapping_sub(id(b));
    }
    for k in 0..4usize {
        let criterio = *((league + league::TIEBREAKS + k) as *const i8);
        if column == 0 && criterio == 6 {
            return *((a + 4) as *const i8) as i32 - *((b + 4) as *const i8) as i32;
        }
        if criterio == 0 {
            break;
        }
        let (va, vb) = match criterio {
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (y(a, 0x12), y(b, 0x12)),
            4 => (*((a + 4) as *const i8) as i32, *((b + 4) as *const i8) as i32),
            5 => (core::ptr::read_unaligned((a + 0x31) as *const i16) as i32, core::ptr::read_unaligned((b + 0x31) as *const i16) as i32),
            _ => continue,
        };
        if va != vb {
            return vb - va;
        }
    }
    if column == 0 {
        let games = core::ptr::read_unaligned((league + league::GAMES_PER_CLUB) as *const i16);
        if *((a + 0x11) as *const i8) as i16 == games && *((b + 0x11) as *const i8) as i16 == games {
            let xb = core::ptr::read_unaligned((*(b as *const usize) + 0x69) as *const usize);
            let xa = core::ptr::read_unaligned((*(a as *const usize) + 0x69) as *const usize);
            if xb != 0 && xa != 0 {
                let d = (*(xb as *const i32)).wrapping_sub(*(xa as *const i32));
                if d != 0 {
                    return d;
                }
            }
        }
    }
    id(a).wrapping_sub(id(b))
}

/// Builds the round-robin grid where every club meets each other once, with balanced home and away games.
///
/// Stdcall, ret 8: the league's round grid for `n` clubs: `grid[1..=n]` are rows of integers (columns 1..n−1 = rounds)
/// with each club's opponent in the round (positive at home, negative away). Zeroes everything; club `n` faces 1..n−1
/// in a round that advances 2 at a time (at home for the first half); then each club k from n−1 down to 2 fills the
/// empty slots with clubs 1..n, alternating home advantage starting from the parity of n−1−k. Returns 1.
/// Original: `league_build_round_robin_grid` (0x6844b0, league.cpp, stdcall).
///
/// [pt] { 0x6844b0 } (stdcall, ret 8, league.cpp): a grade de turnos da liga de `n` clubes: `grade[1..=n]`
/// são linhas de inteiros (colunas 1..n−1 = rodadas) com o adversário de cada clube na rodada (positivo
/// em casa, negativo fora). Zera tudo; o clube `n` enfrenta 1..n−1 numa rodada que anda de 2 em 2
/// (casa para a primeira metade); depois cada clube k de n−1 até 2 preenche as vagas vazias com os
/// clubes 1..n, alternando o mando a partir da paridade de n−1−k. Devolve 1.
pub(crate) unsafe extern "stdcall" fn league_build_round_robin_grid(grade: *const *mut i32, n: i32) -> u32 {
    let cell = |i: i32, j: i32| (*grade.offset(i as isize)).offset(j as isize);
    for i in 1..=n {
        for j in 1..n {
            *cell(i, j) = 0;
        }
    }
    let half = n / 2;
    let mut col = n - 1;
    for i in 1..n {
        if i <= half {
            *cell(i, col) = n;
            *cell(n, col) = -i;
        } else {
            *cell(i, col) = -n;
            *cell(n, col) = i;
        }
        col = if col == n - 2 { 1 } else if col == n - 1 { 2 } else { col + 2 };
    }
    let mut k = n - 1;
    while k >= 2 {
        let minus_k = -k;
        let mut away = (n - 1 + minus_k) % 2;
        let mut col = k - 1;
        let mut contra = -1;
        for j in 1..=n {
            let c = cell(j, col);
            if *c == 0 {
                if away != 0 {
                    *c = k; // vaga preenchida
                    *cell(k, col) = contra;
                } else {
                    *c = minus_k;
                    *cell(k, col) = j;
                }
                away = 1 - away;
            }
            col = if col == n - 1 { 1 } else { col + 1 };
            contra -= 1;
        }
        k -= 1;
    }
    1
}

/// Returns row i (59 bytes) of the league table [+0xb1].
///
/// [pt] A linha `i` (59 bytes) da tabela [+0xb1] da liga.
pub(crate) unsafe fn league_row(league: usize, i: i32) -> usize {
    (*((league + league::TABLE) as *const usize)).wrapping_add((i as isize * 59) as usize)
}

/// Goal margin (+1, min 1, else 0xff) a row needs to reach the target row by the league criterion (+0xc5): goal
/// difference or goals scored.
///
/// [pt] A margem de gols (+1, mínimo 1, senão 0xff) que falta para alcançar a linha `alvo` pelo critério da
/// liga (+0xc5): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15).
pub(crate) unsafe fn goal_margin_to_reach(league: usize, row: usize, target: usize) -> u8 {
    let b = |r: usize, o: usize| *((r + o) as *const u8);
    let c = if *((league + league::TIEBREAKS) as *const u8) == 1 {
        b(target, 0x15).wrapping_sub(b(target, 0x17)).wrapping_sub(b(row, 0x15)).wrapping_add(b(row, 0x17))
    } else {
        b(target, 0x15).wrapping_sub(b(row, 0x15))
    }
    .wrapping_add(1);
    if (c as i8) < 1 { 0xff } else { c }
}

/// On the last matchday, returns the goal margin a club needs for the title, the top places or survival, and sets the
/// deciding-match flag.
///
/// Ret 8: on the last day (matches +0x11 ≥ league rounds +0xa9 − 1) and a row with no mark (+0x37 = 0xff), the goal
/// margin the club needs for the title, the top places (+0xbe/+0xbf) or to escape the bottom ones (+0xc0/+0xc1); 0xff
/// if none. With points equal to the club ahead and no possible margin, writes 0x14 into `out` (deciding) and returns 0
/// or 0xff. Only al is returned (the caller uses al).
/// Original: `League::final_day_margin_needed` (0x68a4d0, league.cpp, thiscall).
///
/// [pt] { 0x68a4d0 } (thiscall, ret 8, league.cpp): no último dia (jogos +0x11 ≥ rodadas da liga +0xa9 − 1)
/// e linha sem marca (+0x37 = 0xff), a margem de gols que o clube precisa para o título, as vagas de
/// cima (+0xbe/+0xbf) ou fugir das de baixo (+0xc0/+0xc1); 0xff se nenhuma. Com pontos iguais a quem
/// está à frente sem margem possível, grava 0x14 em `saida` (decisivo) e devolve 0 ou 0xff. Só o al é
/// devolvido (quem chama usa al).
pub(crate) unsafe extern "thiscall" fn final_day_margin_needed(league: usize, row: usize, out: *mut u8) -> u8 {
    let p = |r: usize| core::ptr::read_unaligned((r + 0x19) as *const i16) as i32;
    let extra = *((league + league::WIN_POINTS) as *const i8) as i32;
    let pos = *((row + 4) as *const i8);
    let clubs = core::ptr::read_unaligned((league + league::CLUB_COUNT) as *const i16);
    let table = *((league + league::TABLE) as *const usize);
    let criterio = *((league + league::TIEBREAKS) as *const u8);
    if (*((row + 0x11) as *const i8) as i32) < core::ptr::read_unaligned((league + league::ROUND_COUNT) as *const i16) as i32 - 1 || *((row + 0x37) as *const u8) != 0xff {
        return 0xff; // outside the last day
    }
    // the `alvo` row ahead, unmarked: points + extra above it, 0xff; equal (extra 0), the margin
    let ver = |target: usize| -> Option<u8> {
        if p(row) + extra > p(target) {
            return Some(0xff);
        }
        if extra == 0 && (criterio == 1 || criterio == 2) {
            return Some(goal_margin_to_reach(league, row, target));
        }
        None
    };
    // the first unmarked row from `de` to `ate` (exclusive), with points + extra against those of `linha`
    let decidir = |de: i16, ate: i16| -> Option<u8> {
        let mut i = de;
        while i < ate {
            let r = league_row(league, i as i32);
            if *((r + 0x37) as *const u8) == 0xff {
                let v = p(r) + extra;
                if v > p(row) {
                    *out = 0x14;
                    return Some(0xff);
                }
                if v != p(row) {
                    return Some(0xff);
                }
                *out = 0x14;
                return Some(0);
            }
            i += 1;
        }
        None
    };
    if *((league + league::LEAGUE_FLAGS) as *const u8) & 1 != 0 && *((table + 0x37) as *const u8) != 5 {
        if pos > 0 {
            let v = p(row) + extra;
            if v > p(table) {
                return 0xff;
            }
            if v == p(table) && (criterio == 1 || criterio == 2) {
                return goal_margin_to_reach(league, row, table);
            }
        } else if clubs > 1 {
            let v = core::ptr::read_unaligned((table + 0x54) as *const i16) as i32 + extra;
            if v > p(table) {
                *out = 0x14;
                return 0xff;
            }
            if v != p(table) {
                return 0xff;
            }
            *out = 0x14;
            return 0;
        }
    }
    let (s1, s2) = (*((league + league::PROMOTED_COUNT) as *const u8), *((league + league::PROMOTION_PLAYOFF_COUNT) as *const u8));
    if s1 != 0 || s2 != 0 {
        let slots = s2.wrapping_add(s1) as i8;
        if pos < slots {
            if let Some(r) = decidir(slots as i16, clubs) {
                return r;
            }
        } else {
            let mut i = 0i16;
            while i < slots as i16 {
                let r = league_row(league, i as i32);
                if *((r + 0x37) as *const u8) == 0xff {
                    if let Some(m) = ver(r) {
                        return m;
                    }
                }
                i += 1;
            }
        }
    }
    let (d1, d2) = (*((league + league::RELEGATED_COUNT) as *const u8), *((league + league::RELEGATION_PLAYOFF_COUNT) as *const u8));
    if d1 == 0 && d2 == 0 {
        return 0xff;
    }
    let descem = d2.wrapping_add(d1) as i8;
    let limit = clubs as i32 - descem as i32;
    if (pos as i32) < limit {
        return decidir(clubs.wrapping_sub(descem as i16), clubs).unwrap_or(0xff);
    }
    let mut i = 0i16;
    while (i as i32) < limit {
        let r = league_row(league, i as i32);
        if *((r + 0x37) as *const u8) == 0xff {
            if let Some(m) = ver(r) {
                return m;
            }
        }
        i += 1;
    }
    0xff
}




/// The ordering of the head-to-head mini table: points and the league tie-breakers.
///
/// qsort callback: an empty row ties; otherwise points (+0x19), and then the league's criteria (`[0xb679fc]`+0xc5, up
/// to 4; 0 ends): 1 goal difference (+0x15 − +0x17), 2 goals for (+0x15), 3 byte +0x12, 5 word +0x31; 4 and the others
/// do not decide. Returns b − a at the first criterion that differs.
/// Original: `league_compare_h2h_rows` (0x6834f0, league.cpp, cdecl).
///
/// [pt] { 0x6834f0 } (cdecl, league.cpp): a ordem da minitabela do confronto direto (callback do qsort):
/// linha vazia empata; senão pontos (+0x19), e depois os critérios da liga ([0xb679fc]+0xc5, até 4;
/// 0 encerra): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15), 3 o byte +0x12, 5 a palavra +0x31; 4 e os
/// outros não decidem. Devolve b − a no primeiro critério que difere.
pub(crate) unsafe extern "C" fn league_compare_h2h_rows(a: usize, b: usize) -> i32 {
    if *(a as *const u32) == 0 || *(b as *const u32) == 0 {
        return 0;
    }
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16) as i32;
    if w(a, 0x19) != w(b, 0x19) {
        return w(b, 0x19) - w(a, 0x19);
    }
    let criterios = *(G_LEAGUE_SORT_CURRENT_LEAGUE as *const usize) + 0xc5;
    for k in 0..4 {
        let (x, y) = match *((criterios + k) as *const i8) {
            0 => return 0,
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (*((a + 0x12) as *const i8) as i32, *((b + 0x12) as *const i8) as i32),
            5 => (w(a, 0x31), w(b, 0x31)),
            _ => continue,
        };
        if x != y {
            return y - x;
        }
    }
    0
}

/// Returns the 14-byte head-to-head record of table row `row` against opponent (pointer at +5 of the 59-byte row).
///
/// [pt] O registro de 14 bytes do confronto da linha `linha` contra o adversário `adversario` (a linha da
/// tabela tem 59 bytes; em +5 o ponteiro dos confrontos).
pub(crate) unsafe fn head_to_head_record(table: usize, row: i32, opponent: i32) -> usize {
    let confrontos = core::ptr::read_unaligned((table as isize + row as isize * 59 + 5) as *const usize);
    (confrontos as isize + opponent as isize * 14) as usize
}

/// Adds one match to the head-to-head record (played, result, goals and points).
///
/// Ret 0x14: adds a match to the head-to-head entry: matches (+4), wins (+5, with the league's win points +0xc2), draws
/// (+6, draw points +0xc3) or defeats (+7), goals for (+8) and against (+0xa); the points in +0xc. Negative row or
/// opponent: nothing.
/// Original: `League::update_head_to_head` (0x6869a0, league.cpp, thiscall).
///
/// [pt] { 0x6869a0 } (thiscall, ret 0x14, league.cpp): soma um jogo ao confronto direto: jogos (+4), vitórias
/// (+5, com os pontos por vitória da liga +0xc2), empates (+6, pontos por empate +0xc3) ou derrotas (+7),
/// gols pró (+8) e contra (+0xa); os pontos em +0xc. Linha ou adversário negativo: nada.
pub(crate) unsafe extern "thiscall" fn update_head_to_head(league: usize, table: usize, row: i32, opponent: i32, pro: i8, contra: i8) {
    if row < 0 || opponent < 0 {
        return;
    }
    let e = head_to_head_record(table, row, opponent);
    let byte = |o: usize| (e + o) as *mut u8;
    let word = |o: usize| (e + o) as *mut i16;
    let add = |o: usize, v: i16| core::ptr::write_unaligned(word(o), core::ptr::read_unaligned(word(o)).wrapping_add(v));
    *byte(4) = (*byte(4)).wrapping_add(1);
    if pro > contra {
        *byte(5) = (*byte(5)).wrapping_add(1);
        add(0xc, *((league + league::WIN_POINTS) as *const i8) as i16);
    } else if pro == contra {
        *byte(6) = (*byte(6)).wrapping_add(1);
        add(0xc, *((league + league::DRAW_POINTS) as *const i8) as i16);
    } else {
        *byte(7) = (*byte(7)).wrapping_add(1);
    }
    add(8, pro as i16);
    add(0xa, contra as i16);
}






/// does the competition have +0xbe and flag 2 of +0xd9?
/// Original: `competition_flagged` (0x684790, league.cpp, thiscall).
///
/// [pt] { 0x684790 } (thiscall, league.cpp): a competição tem o byte +0xbe e a marca 2 de +0xd9?
pub(crate) unsafe extern "thiscall" fn competition_flagged(t: usize) -> u32 {
    (*((t + league::PROMOTED_COUNT) as *const u8) != 0 && *((t + league::LEAGUE_FLAGS) as *const u8) & 2 != 0) as u32
}

/// the record (59 bytes) of table +0xb1
/// Original: `league_record` (0x6826d0, league.cpp, thiscall).
///
/// [pt] { 0x6826d0 } (thiscall, ret 4, league.cpp): o registro `i` (59 bytes) da tabela [this+0xb1].
pub(crate) unsafe extern "thiscall" fn league_record(t: usize, i: i32) -> usize {
    core::ptr::read_unaligned((t + league::TABLE) as *const usize).wrapping_add((i as usize).wrapping_mul(59))
}

comparar_valor!(comparar_competition_flagged, competition_flagged, IDX_COMPETITION_FLAGGED, () -> u32);
comparar_valor!(comparar_league_record, league_record, IDX_LEAGUE_RECORD, (i: i32) -> usize);


/// Returns the next position in the calendar rotation (special league, jumps of 3 by random draw, wrapping at n).
///
/// The next position `pos` in the calendar rotation of `n` (word) clubs. In the special league (`league` =
/// `[0x9cf6e8]`): 2 becomes 4, 5 becomes 3, the rest advance 1. With fewer than 2 legs or 8 rounds, or the 0x40 flag,
/// advances 1; with 0x80, advances 1 swapping n/2 for n−1 and n for n/2. Otherwise the state `*step` (0..3) alternates
/// jumps: 1 and 2 go back 1 (below 0, n−1) and move to 2 and 3; 3 jumps 3 and goes back to 0; 0 jumps 3 (moving to 1)
/// if `pos` < n−5 and random(4) ≥ 2, otherwise advances 1; the jumps wrap around at n.
/// Original: `rotate_round` (0x68a030, league.cpp, cdecl).
///
/// [pt] { 0x68a030 } (cdecl, league.cpp): a próxima posição `pos` no rodízio do calendário de `n` (palavra)
/// clubes. Na liga especial (`liga` = [0x9cf6e8]): 2 vira 4, 5 vira 3, o resto avança 1. Com menos
/// de 2 turnos ou de 8 rodadas, ou a marca 0x40, avança 1; com 0x80, avança 1 trocando n/2 por
/// n−1 e n por n/2. Senão o estado `*passo` (0..3) alterna saltos: 1 e 2 voltam 1 (abaixo de 0,
/// n−1) e vão a 2 e 3; 3 salta 3 e volta a 0; 0 salta 3 (indo a 1) se `pos` < n−5 e sorteio(4) ≥ 2,
/// senão avança 1; os saltos dão a volta em n.
pub(crate) unsafe extern "C" fn rotate_round(pos: i32, step_towards: *mut i8, league: i32, rounds: i16, turnos: i16, n: i16, marks: u8) -> i32 {
    if league == *(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as *const i32) {
        return match pos {
            2 => 4,
            5 => 3,
            _ => pos + 1,
        };
    }
    if turnos < 2 || rounds < 8 || marks & 0x40 != 0 {
        return pos + 1;
    }
    let n = n as i32;
    if marks & 0x80 != 0 {
        let p = pos + 1;
        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p };
    }
    let volta = |p: i32| if p < n { p } else { p - n };
    match *step_towards {
        1 | 2 => {
            *step_towards += 1;
            if pos - 1 >= 0 { pos - 1 } else { n - 1 }
        }
        3 => {
            *step_towards = 0;
            volta(pos + 3)
        }
        _ => {
            let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
            if pos < n - 5 && sorteio(4) >= 2 {
                *step_towards = 1;
                volta(pos + 3)
            } else {
                volta(pos + 1)
            }
        }
    }
}

/// Stores a round match (three bytes and a dword).
///
/// Writes match `j` 0..7 of round `round` (0x41 bytes each, 7 per match starting at `base`): +9 `a` (−1..6), +0xa `b`
/// (−1..2), +0xb `c` (−1..4) and +5 the dword `value`. Out of range: the original routine (error boxes, and it writes
/// anyway).
/// Original: `store_round_match` (0x68a1c0, league.cpp, cdecl).
///
/// [pt] { 0x68a1c0 } (cdecl, league.cpp): grava o jogo `j` 0..7 da rodada `rodada` (0x41 bytes cada,
/// 7 por jogo a partir de `base`): +9 `a` (−1..6), +0xa `b` (−1..2), +0xb `c` (−1..4) e +5 o dword
/// `valor`. Fora das faixas: a original (caixas de erro, e grava assim mesmo).
pub(crate) unsafe extern "C" fn store_round_match(base: usize, round: i16, j: i16, a: i16, b: i8, c: i8, value: u32) {
    if !(0..=7).contains(&j) || !(-1..=6).contains(&a) || !(-1..=2).contains(&b) || !(-1..=4).contains(&c) {
        return original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(IDX_STORE_ROUND_MATCH)(base, round, j, a, b, c, value);
    }
    let e = base.wrapping_add((round as isize * 0x41 + j as isize * 7) as usize);
    *((e + 9) as *mut i8) = a as i8;
    *((e + 0xa) as *mut i8) = b;
    *((e + 0xb) as *mut i8) = c;
    core::ptr::write_unaligned((e + 5) as *mut u32, value);
}




/// the league total prize fund (one more share per place above the last)
/// Original: `League::calc_prize_fund` (0x682f70, league.cpp, thiscall).
///
/// [pt] { 0x682f70 } (thiscall, ret 4, league.cpp): o prêmio total da liga: `cota` para o último colocado e
/// uma cota a mais por posição acima (cota × (1 + 2 + … + times), times = +0x3e). Sem times: a
/// original (caixa de erro, devolve 0).
pub(crate) unsafe extern "thiscall" fn calc_prize_fund(league: usize, cota: i32) -> i32 {
    let teams = *((league + league::CLUB_COUNT) as *const i16);
    if teams == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_CALC_PRIZE_FUND)(league, cota);
    }
    let mut total = 0i32;
    let mut parcela = cota;
    for _ in 0..teams.max(0) {
        total = total.wrapping_add(parcela);
        parcela = parcela.wrapping_add(cota);
    }
    total
}


/// at the season end, only the top league of the pyramid (no league above) closes its table and walks down the chain of
/// divisions: for each pair (upper and lower), closes the lower one and swaps the promoted and relegated clubs
/// (0x689c80, with contract clauses, pairwise or as a block, without checking counts)
/// Original: `League::process_promotion_relegation_chain` (0x689c20, league.cpp, thiscall, ret 4).
///
/// [pt] { 0x689c20 } (thiscall, ret 4, league.cpp, `League::process_promotion_relegation_chain(aos_pares)`; pelo slot do
/// fim de temporada da liga): só na liga do topo da pirâmide (+0x1c, a liga de cima, negativo): fecha a temporada
/// dela (slot 0xa4) e, descendo a cadeia (+0x20 o índice da liga de baixo na tabela [0xadadfc], até negativo), fecha a
/// de baixo e troca os que sobem e descem entre as duas (0x689c80 com contratos = 1, `aos_pares`, sem contagem
/// esperada: −1, −1; o ecx é o desta liga).
pub(crate) unsafe extern "thiscall" fn league_process_promotion_relegation_chain(this: usize, aos_pairs: u32) {
    if (read_u32_as_usize(this + league::UPPER_DIVISION_ID) as i32) >= 0 {
        return;
    }
    crate::comparison::branch("cqp_1 cadeia");
    let swap: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_LEAGUE_PROMOTE_AND_RELEGATE);
    let mut acima = this;
    finalize_league_table(this); // cqp1: fecha a liga do topo
    let mut i = read_u32_as_usize(this + league::LOWER_DIVISION_ID) as i32;
    while i >= 0 {
        let abaixo = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + i as usize * 4);
        finalize_league_table(abaixo); // cqp1: fecha a de baixo
        swap(this, acima, abaixo, 1, aos_pairs, u32::MAX, u32::MAX); // cqp1: sobem e descem
        i = read_u32_as_usize(abaixo + league::LOWER_DIVISION_ID) as i32;
        acima = abaixo;
    }
}


/// Moves the club to a division: the current division becomes last season's and the new one is set; clears the division
/// clauses (and the squad numbers when the new one uses them), opens the requested manager vacancy, upgrades the
/// stadium to the league minimum and turns status 3 into 2.
///
/// Ret 0xc; `this` is not read: `League::move_club_to_division(club, division, vacancy)`: the current division (+0x57)
/// becomes last season's (+0x5b) and the new one goes in; the players' division clauses are cleared (0x4cd120 on
/// `[0xae1050]`), also the squad numbers if the old division did not use them and the new one does (0x85add0 on
/// `[0xdd7ef0]`, before and after); with `vacancy` and no manager (+0xcf), the manager vacancy (0x69ba50 on
/// `[0xb67a44]`); in a league division (+0x43 = 2 in competition `[0xadadfc][id]`) with a minimum stadium (+0xe2 ≠ −1)
/// and a stadium record (+0x69), the stadium works (0x599e00 on the finances 0x5a0590 of `[0xae2a38]`, seats +0xe4 and
/// +0xe2); and, with the competition loaded, status +0x64 = 3 changes to 2.
/// Original: `League::move_club_to_division` (0x6830b0, league.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x6830b0 } (thiscall, ret 0xc, league.cpp; `this` não é lido): `League::move_club_to_division(clube, divisao,
/// vaga)`: a divisão atual (+0x57) vira a da última temporada (+0x5b) e a nova entra; as cláusulas de divisão dos
/// jogadores são limpas (0x4cd120 em [0xae1050]), com os números da camisa também se a divisão velha não os usava e a
/// nova usa (0x85add0 em [0xdd7ef0], antes e depois); com `vaga` e sem técnico (+0xcf), a vaga de técnico (0x69ba50 em
/// [0xb67a44]); numa divisão de liga (+0x43 = 2 na competição [0xadadfc][id]) com estádio mínimo (+0xe2 ≠ −1) e ficha
/// do estádio (+0x69), a obra do estádio (0x599e00 nas finanças 0x5a0590 de [0xae2a38], lugares +0xe4 e +0xe2); e,
/// com a competição carregada, a situação +0x64 = 3 passa a 2.
pub(crate) unsafe extern "thiscall" fn move_club_to_division(_this: usize, club: usize, division: usize, slot: u32) {
    crate::comparison::branch("cpm_3 mudar de divisao");
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    let limpar_clausulas: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_RESET_DIVISION_CLAUSES);
    let before = usa_numeros(read_usize(G_SQUAD_MANAGER), club, 0);
    write_usize(club + club::LAST_DIVISION, read_usize(club + club::DIVISION)); // the division of the last season
    write_usize(club + club::DIVISION, division);
    let numeros = (before == 0 && usa_numeros(read_usize(G_SQUAD_MANAGER), club, 0) != 0) as u32;
    limpar_clausulas(read_usize(G_CONTRACT_MANAGER), club, numeros);
    if slot != 0 && read_usize(club + club::MANAGER) == 0 {
        let open_slot: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_ADD_JOB_VACANCY);
        open_slot(read_usize(G_MANAGER_MANAGER), club, 0);
    }
    if division == 0 {
        return;
    }
    let comps = read_usize(G_COMP_OBJECT_TABLE);
    let id = read_usize(division);
    if comps != 0 {
        let comp = read_usize(comps + 4 * id);
        if comp != 0 && read_usize(club + club::STADIUM) != 0 && *((comp + 0x43) as *const u8) == 2 {
            let lugares = core::ptr::read_unaligned((comp + 0xe2) as *const i16);
            if lugares != -1 {
                crate::comparison::branch("cpm_3 estadio da divisao");
                let capacity = core::ptr::read_unaligned((comp + 0xe4) as *const i16);
                let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
                let obra: unsafe extern "thiscall" fn(usize, i32, i32, usize, u32) -> u32 = core::mem::transmute(FN_FINANCE_UPGRADE_STADIUM_FOR_LEAGUE);
                let f = finances(read_usize(G_FINANCE_TABLE), club);
                obra(f, capacity as i32, lugares as i32, id, 0); // the stadium works
            }
        }
    }
    // the original rereads [0xadadfc] after the construction work (without it, the same value; if null, it also reads from address 0)
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * id) != 0 && *((club + club::PROFESSIONAL_STATUS) as *const u8) == 3 {
        *((club + club::PROFESSIONAL_STATUS) as *mut u8) = 2;
    }
}


/// Shuffles the league table with the seeded rand and puts clubs sharing a ground, clubs with linked grounds, and
/// Celtic and Rangers in paired slots.
///
/// Ret 4: `League::draw_team_order(clubs)`: shuffles the league table (`[+0xb1]`, +0x3e records of 59 bytes) for the
/// fixture grid: keeps a random draw (0x90cfc0, 30000), seeds the CRT rand with +0x40 + `[0xdd7120]` and swaps n random
/// pairs (rand × n / 0x8000), then restores the seed (draw + 1). League with flag 0x100 in +0xd9: each club goes to the
/// slot that virtual table +0xa8 says (if free), the others to the first free ones, and +0xac receives each slot.
/// Otherwise: in Scotland (`[+4]` = `[0x9cf5dc]`) Celtic and Rangers (`[0x9d0118]`, `[0x9d011c]`) go to slots 0 and
/// clubs / 2, in random order (0x90cfc0, 2); clubs of the same stadium (+0x69) and then those of linked stadiums (+0x48
/// of the stadium) go in pairs to slots k and clubs / 2 + k (past half of the clubs, the box); the others to the first
/// free slots. The grid (0x944cf1, freed with 0x944c94) goes back to the table. Out of memory: the box (line 0xed3).
/// Original: `League::draw_team_order` (0x686a70, league.cpp, thiscall, ret 4).
///
/// [pt] { 0x686a70 } (thiscall, ret 4, league.cpp): `League::draw_team_order(clubes)`: embaralha a tabela da liga
/// ([+0xb1], +0x3e registros de 59 bytes) para a grade de jogos: guarda um sorteio (0x90cfc0, 30000), semeia o rand
/// da CRT com +0x40 + [0xdd7120] e troca n pares sorteados (rand × n / 0x8000), e volta a semente (sorteio + 1). Liga
/// com a marca 0x100 em +0xd9: cada clube vai à vaga que a tabela virtual +0xa8 diz (se livre), os outros às
/// primeiras livres, e a +0xac recebe cada vaga. Senão: na Escócia ([+4] = [0x9cf5dc]) o Celtic e o Rangers
/// ([0x9d0118], [0x9d011c]) vão às vagas 0 e clubes / 2, em ordem sorteada (0x90cfc0, 2); os clubes do mesmo estádio
/// (+0x69) e depois os de estádios ligados (+0x48 do estádio) vão em pares às vagas k e clubes / 2 + k (passando da
/// metade dos clubes, a caixa); os outros, às primeiras vagas livres. A grade (0x944cf1, livre em 0x944c94) volta à
/// tabela. Sem memória: a caixa (linha 0xed3).
pub(crate) unsafe extern "thiscall" fn draw_team_order(this: usize, clubs: i32) {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(FN_CRT_SRAND);
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_CRT_RAND);
    let n = || read_i16(this + league::CLUB_COUNT) as i32;
    let table = || read_u32_as_usize(this + league::TABLE);
    let half_n = n() / 2;
    let used = new((n() * 4) as u32) as *mut u32;
    let posto = new((n() << 2) as u32) as *mut u32;
    let grade = new((n() * CUP_LEAGUE_TABLE_ROW_SIZE as i32) as u32);
    let mut k: i32 = 0;
    if grade == 0 || used.is_null() || posto.is_null() {
        show_error_at(0x9f15f4, 0xed3);
        free(posto as usize);
        free(used as usize);
        free(grade);
        return;
    }
    let mut i = 0;
    while i < n() {
        *used.add(i as usize) = 0;
        *posto.add(i as usize) = 0;
        i += 1;
    }
    let semente = sorteio(30000).wrapping_add(1);
    semear((read_i16(this + league::YEAR) as i32).wrapping_add(*(G_GAME_RANDOM_SEED as *const i32)) as u32); // cpu12: a semente
    crate::comparison::branch("cpu_12 ordem sorteada");
    let mut i = 0;
    while i < n() {
        let a = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let b = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let mut tmp = [0u8; CUP_LEAGUE_TABLE_ROW_SIZE];
        let t = table();
        core::ptr::copy(( t + a * CUP_LEAGUE_TABLE_ROW_SIZE) as *const u8, tmp.as_mut_ptr(), CUP_LEAGUE_TABLE_ROW_SIZE);
        copy_row_59(t + a * CUP_LEAGUE_TABLE_ROW_SIZE, t + b * CUP_LEAGUE_TABLE_ROW_SIZE);
        core::ptr::copy(tmp.as_ptr(), (table() + b * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE); // cpu12: a troca
        i += 1;
    }
    semear(semente as u32);
    let half = clubs / 2;
    if read_i16(this + league::LEAGUE_FLAGS) as u16 & 0x100 != 0 {
        let mut i = 0;
        while i < n() {
            *((grade + i as usize * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u32) = 0;
            i += 1;
        }
        let vt = read_u32_as_usize(this);
        let slot_of: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(read_u32_as_usize(vt + 0xa8));
        let mut i = 0usize;
        while (i as i32) < n() {
            let s = slot_of(this, read_u32_as_usize(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE)) as i8 as i32;
            if s >= 0 && s < n() && read_u32_as_usize(grade + s as usize * CUP_LEAGUE_TABLE_ROW_SIZE) == 0 {
                copy_row_59(grade + s as usize * CUP_LEAGUE_TABLE_ROW_SIZE, table() + i * CUP_LEAGUE_TABLE_ROW_SIZE);
                *((table() + i * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u32) = 0;
            }
            i += 1;
        }
        let mut i = 0usize;
        while (i as i32) < n() {
            let lin = table() + i * CUP_LEAGUE_TABLE_ROW_SIZE;
            if read_u32_as_usize(lin) != 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| read_u32_as_usize(grade + j * CUP_LEAGUE_TABLE_ROW_SIZE) == 0) {
                    copy_row_59(grade + j * CUP_LEAGUE_TABLE_ROW_SIZE, lin);
                }
            }
            i += 1;
        }
        let write: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(read_u32_as_usize(vt + 0xac));
        let mut i = 0;
        while i < n() {
            write(this, read_u32_as_usize(grade + i as usize * CUP_LEAGUE_TABLE_ROW_SIZE), i);
            i += 1;
        }
    } else {
        if read_u32_as_usize(read_u32_as_usize(this + 4)) == read_u32_as_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
            let (mut rangers, mut celtic): (i8, i8) = (-1, -1); // bl e [esp+0x17]
            let total = read_i16(this + league::CLUB_COUNT);
            let mut c: i8 = 0;
            while total > 0 && (c as i16) < total {
                let id = read_u32_as_usize(read_u32_as_usize(table() + c as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE));
                if id == read_u32_as_usize(G_KEY_CLUB_GLASGOW_CELTIC) {
                    celtic = c;
                }
                if id == read_u32_as_usize(G_KEY_CLUB_GLASGOW_RANGERS) {
                    rangers = c;
                }
                c = c.wrapping_add(1);
            }
            if rangers != -1 && celtic != -1 {
                let (primeiro, segundo) = if sorteio(2) != 0 { (celtic, rangers) } else { (rangers, celtic) };
                copy_row_59(grade, table() + primeiro as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE);
                copy_row_59(grade + half as usize * CUP_LEAGUE_TABLE_ROW_SIZE, table() + segundo as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE);
                k = 1;
                *posto.add(half as usize) = 1;
                *posto = 1;
                *used.add(celtic as isize as usize) = 1;
                *used.add(rangers as isize as usize) = 1;
            }
        }
        // the clubs of the same stadium (+0x69)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *used.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *used.add(j) == 0 {
                        let ei = read_u32_as_usize(read_u32_as_usize(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                        if ei != 0 {
                            let ej = read_u32_as_usize(read_u32_as_usize(table() + j * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                            if ej != 0 && ei == ej {
                                if k >= half_n {
                                    show_error_at(0x9f15f4, 0xf76);
                                } else {
                                    place_pair(grade, posto, used, table(), i, j, &mut k, half);
                                }
                                break;
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // the clubs with linked stadiums (+0x48 of one’s stadium is the other’s)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *used.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *used.add(j) == 0 {
                        let ei = read_u32_as_usize(read_u32_as_usize(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                        if ei == 0 {
                            break;
                        }
                        let ej = read_u32_as_usize(read_u32_as_usize(table() + j * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                        if ej == 0 {
                            break;
                        }
                        if read_u32_as_usize(ei + stadium::STADIUM_NEARBY_STADIUM) == ej || read_u32_as_usize(ej + stadium::STADIUM_NEARBY_STADIUM) == ei {
                            if k >= half_n {
                                show_error_at(0x9f15f4, 0xfa3);
                            } else {
                                place_pair(grade, posto, used, table(), i, j, &mut k, half);
                            }
                            break;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // the others, to the first free slots
        let mut i = 0usize;
        while (i as i32) < n() {
            if *used.add(i) == 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| *posto.add(j) == 0) { // cpu12: a primeira vaga livre
                    crate::comparison::branch("cpu_12 clube na vaga livre");
                    copy_row_59(grade + j * CUP_LEAGUE_TABLE_ROW_SIZE, table() + i * CUP_LEAGUE_TABLE_ROW_SIZE);
                    *posto.add(j) = 1;
                    *used.add(i) = 1;
                }
            }
            i += 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < n() {
        copy_row_59(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE, grade + i * CUP_LEAGUE_TABLE_ROW_SIZE);
        i += 1;
    }
    free(posto as usize);
    free(used as usize);
    free(grade);
}


/// For each table row, moves the club byte +0x64 to the value (down or up) when the row status is not the excluded one
/// and when it is the included one (0xfd = none).
///
/// Ret 0x10: `League::limit_club_division_tier(value, excluded, included, lower)` (called by the season end of each
/// league: Argentina, Belgium, Brazil, Croatia, Denmark, England, France, Germany, Netherlands, Italy, Russia,
/// Scotland, Sweden and Turkey): for each table row (+0xb1, 0x3b bytes, count +0x3e), adjusts byte +0x64 of the club
/// (+0) to `value` when the row status (+0x37) is not `excluded` (0xfd = none) and again when it is `included` (0xfd =
/// none). The first three arguments are bytes pushed as dwords: only the low byte counts.
/// Original: `League::limit_club_division_tier` (0x68a980, league.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x68a980 } (thiscall, ret 0x10, league.cpp): `League::limit_club_division_tier(valor, fora, dentro, baixar)`
/// (chamada pelo fim de temporada de cada liga: Argentina, Bélgica, Brasil, Croácia, Dinamarca, Inglaterra, França,
/// Alemanha, Holanda, Itália, Rússia, Escócia, Suécia e Turquia): para cada linha da tabela (+0xb1, 0x3b bytes, conta
/// +0x3e), ajusta o byte +0x64 do clube (+0) a `valor` quando a situação da linha (+0x37) não é `fora` (0xfd = nenhuma)
/// e de novo quando é `dentro` (0xfd = nenhuma). Os três primeiros argumentos são bytes empurrados como dword: só o
/// byte baixo conta.
pub(crate) unsafe extern "thiscall" fn limit_club_division_tier(this: usize, value: u32, away: u32, inside: u32, baixar: u32) {
    let (value, away, inside) = (value as u8 as i8, away as u8, inside as u8);
    let mut i: i16 = 0;
    while i < *((this + league::CLUB_COUNT) as *const i16) {
        let row = || read_usize(this + league::TABLE).wrapping_add(i as isize as usize * 0x3b);
        if away != 0xfd && *((row() + 0x37) as *const u8) != away {
            cap_professional_status(read_usize(row()), value, baixar);
        }
        if inside != 0xfd && *((row() + 0x37) as *const u8) == inside {
            cap_professional_status(read_usize(row()), value, baixar);
        }
        i = i.wrapping_add(1);
    }
}


