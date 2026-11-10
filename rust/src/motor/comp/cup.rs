use super::*;

/// True when the club is run by a human manager (News::club_has_human_manager on G_NEWS).
///
/// [pt] O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4).
pub(crate) unsafe fn cup_has_human_manager(club: usize) -> bool {
    exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0
}

/// Moves a fixture's date to tomorrow if it is not after today, then adds the fixture to the fixture calendar.
///
/// The fixture date (+0x2a day, +0x28 year, from the dwords the original routine pushes: 0x549ef0 push year; push day;
/// lea ecx, local -> thiscall, ret 8) that is not after today ([0xae2c90]: unsigned year, signed day) becomes tomorrow
/// (`exe::date_add_days`: push 1; push output; mov ecx, 0xae2c90 -> thiscall(date, output, n), ret 8); and the fixture goes to the
/// calendar (`add_fixture`: push 1; push &fixture; ecx = [0xae2a58] -> thiscall, ret 8).
///
/// [pt] A data da partida (+0x2a dia, +0x28 ano, pelos dwords que a original empurra: 0x549ef0 push ano; push dia; lea ecx,
/// local -> thiscall, ret 8) que não estiver depois de hoje ([0xae2c90]: ano sem sinal, dia com sinal) passa a amanhã
/// (0x54c5b0: push 1; push saída; mov ecx, 0xae2c90 -> thiscall(data, saída, n), ret 8); e a partida vai ao calendário
/// (0x5aa680: push 1; push &partida; ecx = [0xae2a58] -> thiscall, ret 8).
pub(crate) unsafe fn add_fixture_to_calendar(p: Fixture) {
    let (mut t, mut saida) = ([0u8; 8], [0u8; 8]);
    let f = exe::new_date(t.as_mut_ptr() as usize, read_usize(p.addr() + fixture::DAY) as u32, read_usize(p.addr() + fixture::YEAR) as u32);
    let (hd, ha) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let fa = read_i16(f + 2) as u16;
    let before = ha < fa || (ha == fa && hd < read_i16(f));
    if !before {
        let s = saida.as_mut_ptr() as usize;
        exe::date_add_days(G_CURRENT_DATE, s, 1);
        p.set_day(read_i16(s) as u16);
        p.set_year(read_i16(s + 2) as u16);
    }
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), p, 1);
}

/// Creates the fixture of a tie (or only marks it as deferred): competition, round, rules, clubs, date (tomorrow if
/// already past), slot, stadium and calendar; the news to human managers; in a two-legged round, the second leg `+0x22`
/// days later with home and away swapped.
///
/// Convention: thiscall, ret `0xc`, `Cup.cpp`, SEH frame with no destructor on the normal path (the local dates and
/// news item). `rodada.create_tie_fixtures(confronto, adiar)` (round, tie, defer), called from the round set-up
/// (`0x51c800`) and from the competition screen.
///
/// Tie −1: the first tie of the round with flag `+0x34` = 1 (defer) or 2 (create); none: 0. Defer: flag 2. Create: flag
/// 3; with a team missing (`+0/+2` = −1), the news of the pending tie (`0x522950`: push tie; push round; mov ecx, esi
/// -> thiscall, ret 8).
///
/// Otherwise the fixture is built in a local of `0x4f` bytes: the competition (`+0x14`) and the id (`+0`, −1 if none),
/// −1 at `+8`, the cup year (`+0x40`) at `+0x2c`, `+0x3a` at `+0x32`; from the round (`0x68` bytes at `+0xa3`) the code
/// `+7` (`+3` in a two-legged round, `+0x21` = 2) at `+0x30`, `+9` at `+0x34`, `+0xb` at `+0x36`, the rules `+0xd` at
/// `+0x38` (with no second leg and with `0x800`: `0x1000` if the level (`+4` of the team) of the home team is not lower
/// than the away team's, otherwise `0x800`), `+0x17` at `+0x3a`; the league (`+0x44`) at `+0x42`, the flags (`0xff`, 1,
/// 0), the cup's virtual `+0x7c` (push &fixture; mov ecx, esi), bit `0x400` of `+0x4d` (no second leg and `+0x20` > 0),
/// the clubs (`+0x1c`, `+0x20`) and ids (`+0xc`, `+0x10`; −1 with no club), the date (day `+3`, year `+5` + the cup's),
/// the slot `+6`, the date of the next round (`+0x6b`/`+0x6d`; 0 in the last one), the stadium (`0x85c260`: push
/// `+0x24` of the cup; push &fixture; push `+0x13`; mov ecx, `[0xdd7ef4]` -> thiscall, ret `0xc`) and the calendar
/// (add_fixture_to_calendar).
///
/// With a human manager at either club and without bit `0x100` in `+0xd`, news item `0x14` goes to both. In a
/// two-legged round, with the cup still in `[0xadadfc]`: the second leg `+0x22` days later (`0x549ef0`, `0x54c5b0`,
/// `0x417c10`), code `+4`, the slot by the day of the week (`0x54a3f0`: Saturday or Sunday 1, otherwise 2), home and
/// away swapped, the stadium and the calendar. Returns 1.
/// Original: `Cup::create_tie_fixtures` (0x51cb90, Cup.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x51cb90 } (thiscall, ret 0xc, Cup.cpp, quadro SEH sem destrutor no caminho normal: as datas e a notícia locais):
/// `rodada.create_tie_fixtures(confronto, adiar)`, da montagem da rodada (0x51c800) e da tela da competição.
/// Confronto −1: o primeiro da rodada com a marca +0x34 = 1 (adiar) ou 2 (criar); nenhum: 0. Adiar: marca 2. Criar:
/// marca 3; com time faltando (+0/+2 = −1), a notícia do confronto pendente (0x522950: push confronto; push rodada;
/// mov ecx, esi -> thiscall, ret 8). Senão a partida numa local de 0x4f bytes: a competição (+0x14) e o id (+0, −1
/// sem), −1 em +8, o ano da copa (+0x40) em +0x2c, +0x3a em +0x32, da rodada (0x68 bytes em +0xa3) o código +7 (+3
/// na de duas mãos, +0x21 = 2) em +0x30, +9 em +0x34, +0xb em +0x36, as regras +0xd em +0x38 (sem mão de volta e com
/// 0x800: 0x1000 se o nível (+4 do time) do mandante não é menor que o do visitante, senão 0x800), +0x17 em +0x3a, a
/// liga (+0x44) em +0x42, as marcas (0xff, 1, 0), o virtual +0x7c da copa (push &partida; mov ecx, esi), o bit 0x400
/// de +0x4d (sem mão de volta e +0x20 > 0), os clubes (+0x1c, +0x20) e ids (+0xc, +0x10; −1 sem clube), a data (dia
/// +3, ano +5 + o da copa), a fatia +6, a data da rodada seguinte (+0x6b/+0x6d; na última, 0), o estádio (0x85c260:
/// push +0x24 da copa; push &partida; push +0x13; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc) e o calendário
/// (add_fixture_to_calendar). Com técnico humano num dos dois e sem o bit 0x100 em +0xd, a notícia 0x14 aos dois. Na de duas
/// mãos, com a copa ainda em [0xadadfc]: a volta +0x22 dias depois (0x549ef0, 0x54c5b0, 0x417c10), o código +4, a
/// fatia pelo dia da semana (0x54a3f0: sábado ou domingo 1, senão 2), mando invertido, o estádio e o calendário.
/// Devolve 1.
pub(crate) unsafe extern "thiscall" fn create_tie_fixtures(this: usize, round: u32, tie: u32, adiar: u32) -> u32 {
    let this = Cup(this);
    let id = read_usize(read_usize(this.addr() + 4));
    let ri = round as i16 as i32;
    let (mut k, mut adiar) = (tie, adiar);
    if tie as i16 == -1 {
        let n = read_i16(round_entry(this, ri) + 0x1a);
        let mut i: u32 = 0;
        loop {
            if (i as i16) >= n {
                return 0;
            }
            let m = read_u8(tie_of_round(this, ri, i as i16 as i32) + 0x34);
            if m == 1 {
                adiar = 1;
                break;
            }
            if m == 2 {
                adiar = 0;
                break;
            }
            i = i.wrapping_add(1);
        }
        k = i;
        if k as i16 == -1 {
            return 0;
        }
    }
    let t = tie_of_round(this, ri, k as i16 as i32);
    if adiar != 0 {
        write_u8(t + cup_tie::FIXTURE_STATE, 2);
        return 1;
    }
    write_u8(t + cup_tie::FIXTURE_STATE, 3);
    if read_i16(t) == -1 || read_i16(t + cup_tie::AWAY_TEAM) == -1 {
        exe::cup_post_draw_news(this, round, k);
        return 1;
    }
    let mut buf = [0u8; 0x50];
    let p = Fixture(buf.as_mut_ptr() as usize);
    let comp = read_usize(this.addr() + 4);
    if comp != 0 {
        p.set_competition(Competition(comp));
        write_usize(p.addr(), read_usize(comp));
    } else {
        write_usize(p.addr(), usize::MAX);
        p.set_competition(Competition(0));
    }
    p.set_season_year(this.year() as u16);
    p.set_round_prefix((this.stage_label() as u16) as i16);
    let e = round_entry(this, ri);
    let mut code = read_i16(e + cup_round::ROUND_CODE);
    if read_u8(e + cup_round::LEGS) == 2 {
        code = code.wrapping_add(3);
    }
    p.set_round_name((code as u16) as i16);
    p.set_round_number((read_i16(e + cup_round::ROUND_NUMBER) as u16) as i16);
    p.set_category(read_u8(e + cup_round::MATCH_CATEGORY));
    p.set_round_draw_rules(read_i16(e + cup_round::DRAW_RULES) as u16);
    // simplification: the original does and 0xf4ff on the word +0x4d of a meaningless local (the +0x4d and bits 2, 4..7 of
    // +0x4e keep the stack leftovers) and does not write +4, +0x18, +0x2e, +0x2f, +0x3c before the stadium: here, zero
    p.set_tie_rules(read_i16(e + cup_round::TIE_RULES) as u16);
    for x in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a] {
        write_u8(p.addr() + x, 0xff);
    }
    p.set_match_stats_slot(usize::MAX);
    p.set_stage_index(this.stage_index());
    p.set_home_result_mark((0) as i8);
    p.set_away_result_mark((0) as i8);
    p.set_stage(0xff);
    p.set_home_side_mark(1);
    p.set_away_side_mark((1) as i8);
    if (read_u8(e + cup_round::LEGS) as i8) < 2 && (p.tie_rules() as i16) as u16 & 0x800 != 0 {
        let nc = read_u8(cup_team_entry(this, read_i16(t)) + 4) as i8;
        let nf = read_u8(cup_team_entry(this, read_i16(t + cup_tie::AWAY_TEAM)) + 4) as i8;
        p.set_tie_rules(if nc >= nf { 0x1000 } else { 0x800 });
    }
    let virtual_7c: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x7c));
    virtual_7c(this.addr(), p.addr());
    let e = round_entry(this, ri);
    if (read_u8(e + cup_round::LEGS) as i8) < 2 && (read_u8(e + cup_round::SINGLE_LEG_FLAG) as i8) > 0 {
        p.set_flags(p.flags() | 4);
    } else {
        p.set_flags(p.flags() & !4);
    }
    let t = tie_of_round(this, ri, k as i16 as i32);
    let (lc, lf) = (read_i16(t), read_i16(t + cup_tie::AWAY_TEAM)); // cqc5: the first-leg sides
    let home = read_usize(cup_team_entry(this, lc));
    p.set_home_club(Club(home));
    p.set_home_club_id(if home != 0 { read_usize(home) } else { usize::MAX });
    let away = read_usize(cup_team_entry(this, lf));
    p.set_away_club(Club(away));
    p.set_away_club_id(if away != 0 { read_usize(away) } else { usize::MAX });
    let year_cup = this.year();
    p.set_year((read_u8(e + cup_round::YEAR_OFFSET) as i8 as i16).wrapping_add(year_cup) as u16);
    p.set_day(read_i16(e + cup_round::DAY) as u16); // cqc5: the first-leg day
    p.set_time_slice(read_u8(e + cup_round::TIME_SLICE));
    if ri < this.round_count() as i32 - 1 {
        p.set_next_round_day(read_i16(e + 0x6b) as u16);
        p.set_next_round_year((read_u8(e + 0x6d) as i8 as i16).wrapping_add(this.year()) as u16);
    } else {
        p.set_next_round_day(0);
        p.set_next_round_year(0);
    }
    let stadium: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
    stadium(read_usize(G_STADIUM), read_usize(e + cup_round::VENUE_VALUE), p.addr(), this.venue_stadium_id());
    crate::comparison::branch("cqc_5 tie match");
    add_fixture_to_calendar(p);
    let (home, away) = (p.home_club(), p.away_club());
    if (cup_has_human_manager(home.addr()) || cup_has_human_manager(away.addr())) && read_i16(round_entry(this, ri) + 0xd) as u16 & 0x100 == 0 {
        let mut news = [0u32; 0x38];
        let n = news.as_mut_ptr() as usize;
        exe::new_newsitem(n, 0x14, 0);
        exe::newsitem_set_slot(n, 0, read_usize(read_usize(this.addr() + 4)) as u32);
        exe::newsitem_set_slot(n, 1, this.stage_label() as i32 as u32);
        exe::newsitem_set_slot(n, 2, read_i16(round_entry(this, ri) + 7) as i32 as u32);
        exe::newsitem_set_slot(n, 5, (read_u8(round_entry(this, ri) + 0x21) == 2) as u32);
        exe::newsitem_set_slot(n, 6, p.stadium_id() as u32);
        exe::newsitem_set_slot(n, 7, (p.day() as i16) as u16 as u32);
        exe::newsitem_set_slot(n, 8, (p.year() as i16) as u16 as u32);
        if cup_has_human_manager(home.addr()) {
            exe::newsitem_set_slot(n, 3, p.home_club_id() as u32);
            exe::newsitem_set_slot(n, 4, p.away_club_id() as u32);
        } else {
            exe::newsitem_set_slot(n, 3, p.away_club_id() as u32);
            exe::newsitem_set_slot(n, 4, p.home_club_id() as u32);
        }
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, home);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, away);
    }
    let e = round_entry(this, ri);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 || read_u8(e + cup_round::LEGS) != 2 {
        return 1;
    }
    crate::comparison::branch("cqc_5 return leg");
    if (read_u8(e + cup_round::SINGLE_LEG_FLAG) as i8) < 1 {
        p.set_flags(p.flags() & !4);
    } else {
        p.set_flags(p.flags() | 4);
    }
    p.set_round_name((read_i16(e + cup_round::ROUND_CODE).wrapping_add(4) as u16) as i16);
    let week: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    let (mut a, mut s, mut c, mut x) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let pa = a.as_mut_ptr() as usize;
    // simplification: the original pushes the day and the year in registers with leftovers in the high 16 bits; 0x549ef0 only reads the
    // palavras
    exe::new_date(pa, read_i16(e + cup_round::DAY) as u16 as u32, (read_u8(e + cup_round::YEAR_OFFSET) as i8 as i16).wrapping_add(this.year()) as u16 as u32);
    let r = exe::date_add_days(pa, s.as_mut_ptr() as usize, read_u8(e + cup_round::LEG_GAP_DAYS) as u32);
    exe::date_operator_3d(pa, c.as_mut_ptr() as usize, r);
    p.set_year(read_i16(pa + 2) as u16);
    p.set_day(read_i16(pa) as u16);
    let day = exe::new_date(x.as_mut_ptr() as usize, read_usize(pa) as u32, read_usize(pa + 2) as u32);
    let sd = week(day) as i16;
    p.set_time_slice(if sd == 5 || sd == 6 { 1 } else { 2 });
    let (c1, c2, i1, i2) = (p.home_club(), p.away_club(), p.home_club_id(), p.away_club_id()); // cqc5: the second-leg sides
    p.set_flags(p.flags() & 0xf4);
    p.set_away_club(c1);
    p.set_home_club_id(i2);
    p.set_away_club_id(i1);
    p.set_home_club(c2);
    let e = round_entry(this, ri);
    stadium(read_usize(G_STADIUM), read_usize(e + cup_round::VENUE_VALUE), p.addr(), this.venue_stadium_id());
    add_fixture_to_calendar(p);
    1
}

/// (year, day of year) + n days, rolling over 365/366-day years.
///
/// [pt] (ano, dia) + n dias (o dia vai de 0 a 364, ou 365 no bissexto da 0x549ef0).
pub(crate) fn add_days_to_day_year((ano, dia): (i32, i32), n: i32) -> (i32, i32) {
    let (mut a, mut d) = (ano, dia + n);
    loop {
        let t = if crate::date::is_leap_year(a as u16) == 1 { 366 } else { 365 };
        if d < t {
            return (a, d);
        }
        d -= t;
        a += 1;
    }
}

/// Counts the club's (id) fixtures in the competition from today up to (year, day), for the comparer.
///
/// Uses `0x5acbe0` (FixtureManager::find_fixtures) with a null list and any type `0xfe` and side `0xff`. The window
/// must not go past where `0x5aa680` can place new fixtures + 14 days: the search creates the block of any missing year
/// (`0x5abf10`).
///
/// [pt] Os jogos do clube (id) na competição de hoje até (ano, dia) (0x5acbe0: lista nula, tipo 0xfe e lado 0xff
/// quaisquer), para o comparador. A janela não passa de onde a 0x5aa680 pode pôr os jogos novos + 14 dias: a busca cria
/// o bloco do ano que faltar (0x5abf10).
pub(crate) unsafe fn club_fixtures_until(club: u32, comp: u32, (ano, dia): (i32, i32)) -> u32 {
    let find: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    find(read_usize(G_FIXTURE_MANAGER), read_i16(G_CURRENT_DATE) as u16 as u32, read_i16(G_CURRENT_DATE_YEAR) as u16 as u32, dia as u32, ano as u32, club, 0,
        comp, 0xfe, 0xff) & 0xffff
}

/// Posts news item `0x26` for a tie with an undecided team: the cup, the round and the possible teams, to the managers
/// of each club.
///
/// Convention: thiscall, ret 8, `Cup.cpp`, SEH frame with no destructor on the normal path (the local news item).
/// `rodada.post_draw_news(confronto)` (round, tie), called only from `0x51cb90`.
///
/// With both teams decided, or a round (`0x68` bytes at `+0xa3`) with fewer than 2 ties (`+0x1a`): nothing. Each side
/// with no team (`+0/+2` = −1) comes from its source tie (round `+0xe/+0x10`, tie `+0xa/+0xc`; with no source or with a
/// team missing there: nothing): the two possible clubs; with a team, its club (`+0xa7`, 6 bytes).
///
/// News item `0x26`: 0 = the cup id, 1 = `+0x3a`, 2 = the round code (`+7`), 3/4 = the home team and −1 or the two
/// possible ones, 5/6 = the away team and −1 or the two possible ones; sent to the managers of each of its clubs
/// (`0x7861e0`: push club; push &news; mov ecx, `[0xb7431c]` -> thiscall, ret 8; the possible ones only if not null).
/// Original: `Cup::post_draw_news` (0x522950, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x522950 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: a notícia local):
/// `rodada.post_draw_news(confronto)`, só da 0x51cb90. Com os dois times definidos, ou rodada (0x68 bytes em
/// +0xa3) com menos de 2 confrontos (+0x1a): nada. Cada lado sem time (+0/+2 = −1) vem do confronto de origem (rodada
/// +0xe/+0x10, confronto +0xa/+0xc; sem origem ou com time faltando lá: nada): os dois clubes possíveis; com time, o
/// clube dele (+0xa7, 6 bytes). A notícia 0x26: 0 = o id da copa, 1 = +0x3a, 2 = o código da rodada (+7), 3/4 = o
/// mandante e −1 ou os dois possíveis, 5/6 = o visitante e −1 ou os dois possíveis; aos técnicos de cada clube dela
/// (0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; os possíveis só se não nulos).
pub(crate) unsafe extern "thiscall" fn post_draw_news(this: usize, round: u32, tie: u32) {
    let this = Cup(this);
    let ri = round as i16 as i32;
    let t = tie_of_round(this, ri, tie as i16 as i32);
    let (mut a1, mut a2, mut b1, mut b2, mut casa) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let h = read_i16(t);
    if h != -1 && read_i16(t + cup_tie::AWAY_TEAM) != -1 {
        return;
    }
    if read_i16(round_entry(this, ri) + 0x1a) < 2 {
        return;
    }
    if h == -1 {
        let r2 = read_i16(t + cup_tie::HOME_LINK_ROUND);
        if r2 == -1 {
            return;
        }
        let o = tie_of_round(this, r2 as i32, read_i16(t + cup_tie::HOME_LINK_TIE) as i32);
        let (x, y) = (read_i16(o), read_i16(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        a1 = read_usize(cup_team_entry(this, x));
        a2 = read_usize(cup_team_entry(this, y));
    } else {
        casa = read_usize(cup_team_entry(this, h));
    }
    let away;
    let v = read_i16(t + cup_tie::AWAY_TEAM);
    if v == -1 {
        let r2 = read_i16(t + cup_tie::AWAY_LINK_ROUND);
        if r2 == -1 {
            return;
        }
        let o = tie_of_round(this, r2 as i32, read_i16(t + cup_tie::AWAY_LINK_TIE) as i32);
        let (x, y) = (read_i16(o), read_i16(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        b1 = read_usize(cup_team_entry(this, x));
        b2 = read_usize(cup_team_entry(this, y));
        away = 0;
    } else {
        away = read_usize(cup_team_entry(this, v));
    }
    crate::comparison::branch("cqc_5 pending tie");
    let mut noticia = [0u32; 0x38];
    let n = noticia.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x26, 0);
    exe::newsitem_set_slot(n, 0, read_usize(read_usize(this.addr() + 4)) as u32);
    exe::newsitem_set_slot(n, 1, this.stage_label() as i32 as u32);
    exe::newsitem_set_slot(n, 2, read_i16(round_entry(this, ri) + 7) as i32 as u32);
    if casa != 0 {
        exe::newsitem_set_slot(n, 3, read_usize(casa) as u32);
        exe::newsitem_set_slot(n, 4, u32::MAX);
    } else {
        exe::newsitem_set_slot(n, 3, read_usize(a1) as u32);
        exe::newsitem_set_slot(n, 4, read_usize(a2) as u32);
    }
    if away != 0 {
        exe::newsitem_set_slot(n, 5, read_usize(away) as u32);
        exe::newsitem_set_slot(n, 6, u32::MAX);
    } else {
        exe::newsitem_set_slot(n, 5, read_usize(b1) as u32);
        exe::newsitem_set_slot(n, 6, read_usize(b2) as u32);
    }
    let news = read_usize(G_NEWS);
    if casa != 0 {
        exe::news_send_to_club_manager(news, n, casa);
    } else {
        if a1 != 0 {
            exe::news_send_to_club_manager(news, n, a1);
        }
        if a2 != 0 {
            exe::news_send_to_club_manager(news, n, a2);
        }
    }
    if away != 0 {
        exe::news_send_to_club_manager(news, n, away);
    } else {
        if b1 != 0 {
            exe::news_send_to_club_manager(news, n, b1);
        }
        if b2 != 0 {
            exe::news_send_to_club_manager(news, n, b2);
        }
    }
}

/// Takes the teams out of round `r`'s ties into the 7-byte draw array (team at +0, link at +2/+4, level at +6), as in
/// `sort_round_teams` and `check_round_teams_for_nation`.
///
/// The home team (+0, with +0xe/+0xa) and the away team (+2, with +0x10/+0xc) are taken. With a team, the entry gets
/// the team's level (+4 of the team; the seed), `com_time(vaga)` runs while the slot is still intact, and the slot
/// becomes −2 (free); with no team (−1), the entry is −1 with 0xff as the level (the rest of the entry stays as it
/// was).
///
/// Returns how many entries were written (the 32-bit count, used as 16 bits).
///
/// [pt] Tira os times dos confrontos da rodada `r` para o vetor de 7 bytes do sorteio (o time +0, o vínculo +2/+4, o
/// nível +6), como nas 0x51d2d0 e 0x51d9f0: o mandante (+0, com +0xe/+0xa) e o visitante (+2, com +0x10/+0xc); com
/// time, a entrada leva o nível do time (+4 do time; a semente), `com_time(vaga)` roda com a vaga ainda intacta e a
/// vaga vira −2 (livre); sem time (−1), a entrada fica −1 com 0xff no nível (o resto da entrada fica como estava).
/// Devolve quantas entradas (a conta de 32 bits, usada em 16).
pub(crate) unsafe fn take_round_teams(cup: usize, r: i32, array: usize, mut with_team: impl FnMut(usize)) -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    while (k as i16) < read_i16(round_entry(Cup(cup), r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = tie_of_round(Cup(cup), r, k as i16 as i32);
            let e = index_addr(array, n as i16 as i32, 7);
            n = n.wrapping_add(1);
            let team = read_i16(t + lado);
            if team != -1 {
                write_u16(e, team as u16);
                write_u16(e + 2, read_i16(t + ligado) as u16);
                write_u16(e + 4, read_i16(t + jogo) as u16);
                write_u8(e + 6, read_u8(cup_team_entry(Cup(cup), team) + 4)); // the team’s seed
                with_team(t + lado);
                write_u16(t + lado, 0xfffe);
            } else {
                write_u16(e, 0xffff);
                write_u8(e + 6, 0xff);
            }
        }
        k += 1;
    }
    n
}

/// Puts entry `e` of the draw vector into tie `cur` of round `r`.
///
/// It goes to the home side (`+0`, `+0xe`, `+0xa`) or the away side (`+2`, `+0x10`, `+0xc`). With a link (`+2` of the
/// entry ≠ −1), the linked tie in the next round receives the round (`+0x12`), the drawn tie (`+6`) and the side (`+8`:
/// 1 home, 0 away).
///
/// [pt] A entrada `e` do vetor vai ao confronto `cur` da rodada `r`: no mandante (+0, +0xe, +0xa) ou no visitante (+2,
/// +0x10, +0xc); com vínculo (+2 da entrada ≠ −1), o confronto ligado na rodada seguinte ganha a rodada (+0x12), o
/// confronto sorteado (+6) e o lado (+8: 1 mandante, 0 visitante).
pub(crate) unsafe fn place_team_in_tie(cup: usize, r: i32, round: u32, e: usize, cur: i32, home: bool) {
    let t = tie_of_round(Cup(cup), r, cur as i16 as i32);
    let (lado, ligado, jogo) = if home { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
    write_u16(t + lado, read_i16(e) as u16);
    write_u16(t + ligado, read_i16(e + 2) as u16);
    write_u16(t + jogo, read_i16(e + 4) as u16);
    let next = read_i16(e + 2);
    if next != -1 {
        let v = tie_of_round(Cup(cup), next as i32, read_i16(e + 4) as i32);
        write_u16(v + cup_tie::FEEDER_ROUND, round as u16);
        write_u16(v + cup_tie::FEEDER_TIE, cur as u16);
        write_u16(v + cup_tie::FEEDER_SIDE, home as u16);
    }
}

/// True when cup teams a and b both have clubs of the same nation (+0x53).
///
/// [pt] Os times `a` e `b` (índices em [copa + 0xa7]) têm clube e são da mesma nação (+0x53).
pub(crate) unsafe fn teams_same_nation(cup: usize, a: i16, b: i16) -> bool {
    let c1 = Club(read_ptr(cup_team_entry(Cup(cup), a)));
    if c1.is_null() {
        return false;
    }
    let c2 = Club(read_ptr(cup_team_entry(Cup(cup), b)));
    !c2.is_null() && (c2.nation().addr() as u32) == (c1.nation().addr() as u32)
}

/// Advances the circular walk over a round's ties, switching to the second pass and then the rule fallback; false when
/// exhausted (error box).
///
/// [pt] O passo da volta pelos confrontos: o próximo (em círculo, 16 bits); de volta ao sorteado, a primeira volta vira a
/// segunda (`segunda`), na segunda cai a regra (`regra`: a nação, ou o "sem a mesma nação" da 0x51d9f0) e, sem ela,
/// devolve false (a caixa).
pub(crate) unsafe fn next_tie_step(cup: usize, r: i32, cur: &mut i32, start: i16, second: &mut bool, rule: &mut bool) -> bool {
    *cur = cur.wrapping_add(1);
    if (*cur as i16) >= read_i16(round_entry(Cup(cup), r) + 0x1a) {
        *cur = 0;
    }
    if *cur as i16 == start {
        if !*second {
            *second = true;
        } else if *rule {
            *rule = false;
        } else {
            return false;
        }
    }
    true
}

/// The round draw by seeding (round rule `0x400`): the teams leave the ties, are sorted by seed (highest first), and
/// each goes to a drawn tie.
///
/// `rodada.sort_round_teams()`.
///
/// 1) It takes the teams out of the ties into an array of 7-byte entries (malloc `0x944e46` of 7 × slots `+0x18`; out
/// of memory, the box of line `0xbf8` and the game stops), each with its seed (`+4` of the team). 2) Sorts by seed,
/// highest first (qsort `0x945333` with `0x51ebc0`). 3) Each entry with a team (up to the first without one, level
/// `0xff`) goes to a drawn tie (`0x90cfc0` over the ties), moving round in a circle: on the first pass, only ties with
/// both slots empty (< 0): with both free (−2), the drawn side (`0x90cfc0(2)` ≠ 0: home); only home free: home; only
/// away free: away. From the second pass on (the flag is never reset to zero), any free slot, but with round rule 4
/// (`+0xb`) a free home slot is not allowed when the away team of the tie is from the same nation, and a free away slot
/// is not allowed when the home team is; after going all the way round again, the nation rule is dropped (also for the
/// following entries) and, without it, the box of line `0xcff` and it exits.
///
/// Frees the array (`0x9452ca`).
/// Original: `Cup::sort_round_teams` (0x51d2d0, Cup.cpp, thiscall).
///
/// [pt] { 0x51d2d0 } (thiscall, ret 4, Cup.cpp): `rodada.sort_round_teams()`: o sorteio pela semente (regra 0x400 da
/// rodada). 1) Tira os times dos confrontos para o vetor de 7 bytes (malloc 0x944e46 de 7 × vagas +0x18; sem memória,
/// a caixa da linha 0xbf8 e o jogo para), cada um com a semente (+4 do time). 2) Ordena pela semente, a maior primeiro
/// (qsort 0x945333 com 0x51ebc0). 3) Cada entrada com time (até a primeira sem, nível 0xff) vai a um confronto
/// sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira volta, só confrontos com as duas vagas sem
/// time (< 0): com as duas livres (−2), o lado sorteado (0x90cfc0(2) ≠ 0: mandante); só o mandante livre: mandante;
/// só o visitante: visitante. Da segunda volta em diante (a marca não volta a zero), qualquer vaga livre, mas com a
/// regra 4 da rodada (+0xb) o mandante livre não serve quando o visitante do confronto é da mesma nação, e o
/// visitante livre não serve quando o mandante é; dando a volta inteira de novo, cai a regra das nações (também para
/// as entradas seguintes) e, sem ela, a caixa da linha 0xcff e sai. Libera o vetor (0x9452ca).
pub(crate) unsafe extern "thiscall" fn sort_round_teams(cup: usize, round: u32) {
    let r = round as i16 as i32;
    // the four: push n; add esp, 4 (malloc and random roll) / push p; add esp, 4 (free) / 4 pushes; add esp, 0x10 (qsort) -> cdecl
    let array = exe::crt_malloc((read_i16(round_entry(Cup(cup), r) + 0x18) as i32).wrapping_mul(7) as u32);
    if array == 0 {
        fatal_memory_error_box(0xbf8, 0x9b56dc); // "CUP::sort_round_teams() : Cannot allocate memory"
    }
    let mut nation = read_u8(round_entry(Cup(cup), r) + 0xb) & 4 != 0; // rule 4 (+0xb) of the round
    let n = take_round_teams(cup, r, array, |_| {}) as i16;
    crate::comparison::branch("cqb_11 sorts by seed");
    exe::crt_qsort(array, (n as i32) as u32, 7, 0x51ebc0); // cqb11: by seed, highest first
    let mut second = false;
    let mut j: i16 = 0;
    while j < n {
        let e = index_addr(array, j as i32, 7);
        if read_u8(e + 6) == 0xff {
            break;
        }
        let s = exe::utils_random(read_i16(round_entry(Cup(cup), r) + 0x1a) as i32);
        let start = s as i16;
        let mut cur = s;
        let home = loop {
            let t = tie_of_round(Cup(cup), r, cur as i16 as i32);
            let (casa, fora) = (read_i16(t), read_i16(t + 2));
            let mut side: Option<bool> = None;
            if second {
                let w0 = read_i16(e);
                let free = casa == -2 && !(nation && w0 >= 0 && fora >= 0 && teams_same_nation(cup, w0, fora));
                if fora == -2 {
                    if free && exe::utils_random(2) != 0 {
                        side = Some(true);
                    } else if free || !(nation && w0 >= 0 && casa >= 0 && teams_same_nation(cup, w0, casa)) {
                        side = Some(false);
                    }
                } else if free {
                    side = Some(true);
                }
            } else if casa < 0 && fora < 0 {
                if fora == -2 {
                    if casa == -2 && exe::utils_random(2) != 0 {
                        crate::comparison::branch("cqb_11 seed drawn side");
                        side = Some(true); // cqb11: the drawn side
                    } else {
                        side = Some(false);
                    }
                } else if casa == -2 {
                    side = Some(true);
                }
            }
            if let Some(m) = side {
                break m;
            }
            if !next_tie_step(cup, r, &mut cur, start, &mut second, &mut nation) {
                show_error_at(0x9b55ac, 0xcff);
                exe::crt_free(array);
                return;
            }
        };
        place_team_in_tie(cup, r, round, e, cur, home);
        j = j.wrapping_add(1);
    }
    exe::crt_free(array);
}

/// The round draw that keeps nations apart (round rule 4 without `0x400`): each team's level becomes how many teams of
/// its nation are in the round (the bigger nations go first) and, from the second pass on, a team does not land in a
/// tie with another of its nation while possible.
///
/// `rodada.check_round_teams_for_nation()`.
///
/// It counts the teams of each nation in a zeroed array of `[0xae2350]` bytes (`0x945501`; out of memory, the box of
/// line `0xd27` and the game stops) while taking the teams out of the ties (malloc of 7 × slots, the box of line
/// `0xd34`), using `+0` of the club's nation (`+0x53`); the level of each entry with a team becomes its nation's count
/// (with no nation, the seed stays); it frees the counts and sorts, highest first (the nations with more teams go
/// first).
///
/// Each entry (up to the first with `0xff`) goes to a drawn tie, in a circle: on the first pass, only ties with no team
/// in either slot (with both free, the drawn side; with only one free, that one); from the second on (the flag stays),
/// any free slot, except (while the rule holds, and it is restored for each entry) a tie with a team of the same nation
/// as the entry's club; after going all the way round again, the rule is dropped and, without it, the box of line
/// `0xe5c` and it exits. Frees the array.
/// Original: `Cup::check_round_teams_for_nation` (0x51d9f0, Cup.cpp, thiscall).
///
/// [pt] { 0x51d9f0 } (thiscall, ret 4, Cup.cpp): `rodada.check_round_teams_for_nation()`: o sorteio que separa as nações
/// (regra 4 da rodada sem a 0x400). Conta os times de cada nação num vetor zerado de [0xae2350] bytes (0x945501;
/// sem memória, a caixa da linha 0xd27 e o jogo para) enquanto tira os times dos confrontos (malloc de 7 × vagas, a
/// caixa da linha 0xd34), pelo +0 da nação (+0x53) do clube; o nível de cada entrada com time vira a conta da nação
/// dele (sem nação, fica a semente); libera as contas e ordena, a maior primeiro (as nações com mais times saem
/// antes). Cada entrada (até a primeira com 0xff) vai a um confronto sorteado, em círculo: na primeira volta, só
/// confrontos sem time nas duas vagas (com as duas livres, o lado sorteado; só uma livre, ela); da segunda em diante
/// (a marca fica), qualquer vaga livre, menos (enquanto valer a regra, que volta a valer a cada entrada) um confronto
/// com um time da mesma nação do clube da entrada; dando a volta inteira de novo, cai a regra e, sem ela, a caixa da
/// linha 0xe5c e sai. Libera o vetor.
pub(crate) unsafe extern "thiscall" fn check_round_teams_for_nation(cup: usize, round: u32) {
    let r = round as i16 as i32;
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free and qsort: add esp, 0x14 covers both -> cdecl
    let contas = exe::crt_calloc(read_u32(G_NATION_COUNT), 1);
    if contas == 0 {
        fatal_memory_error_box(0xd27, 0x9b5710); // "CUP::check_round_teams_for_nation() : cannot malloc array"
    }
    let array = exe::crt_malloc((read_i16(round_entry(Cup(cup), r) + 0x18) as i32).wrapping_mul(7) as u32);
    if array == 0 {
        fatal_memory_error_box(0xd34, 0x9b56dc);
    }
    let n = take_round_teams(cup, r, array, |vaga| {
        let club = read_ptr(cup_team_entry(Cup(cup), read_i16(vaga)));
        if club != 0 {
            let nation = read_ptr(club + club::NATION);
            if nation != 0 {
                let c = read_ptr(nation).wrapping_add(contas);
                write_u8(c, read_u8(c).wrapping_add(1));
            }
        }
    }) as i16;
    if n > 0 {
        let mut e = array;
        for _ in 0..n as i32 {
            if read_i16(e) != -1 {
                // simplification: without checking the null club, like the original routine (which would read its +0x53)
                let nation = read_ptr(read_ptr(cup_team_entry(Cup(cup), read_i16(e))) + 0x53);
                if nation != 0 {
                    crate::comparison::branch("cqb_11 nation count");
                    write_u8(e + 6, read_u8(read_ptr(nation).wrapping_add(contas))); // cqb11: the level becomes the nation’s count
                }
            }
            e += 7;
        }
    }
    exe::crt_free(contas);
    exe::crt_qsort(array, (n as i32) as u32, 7, 0x51ebc0);
    let mut second = false;
    let mut j: i16 = 0;
    while j < n {
        let e = index_addr(array, j as i32, 7);
        let mut rule = true; // becomes valid again on each entry
        let club = if read_i16(e) != -1 { read_ptr(cup_team_entry(Cup(cup), read_i16(e))) } else { 0 };
        if read_u8(e + 6) == 0xff {
            break;
        }
        let s = exe::utils_random(read_i16(round_entry(Cup(cup), r) + 0x1a) as i32);
        let start = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = tie_of_round(Cup(cup), r, cur as i16 as i32);
            let mut side: Option<bool> = None;
            if second {
                let mut free = false;
                let mut m = false;
                let home = read_i16(t);
                let c0 = Club(if home == -2 {
                    m = true;
                    free = true;
                    0
                } else if home == -1 {
                    0
                } else {
                    read_ptr(cup_team_entry(Cup(cup), home))
                });
                let away = read_i16(t + 2);
                let c1 = Club(if away == -2 {
                    if !(free && exe::utils_random(2) != 0) {
                        m = false;
                        free = true;
                    }
                    0
                } else if away == -1 {
                    0
                } else {
                    read_ptr(cup_team_entry(Cup(cup), away))
                });
                let same = club != 0 && rule
                    && ((!c0.is_null() && read_u32(club + club::NATION) == (c0.nation().addr() as u32)) || (!c1.is_null() && read_u32(club + club::NATION) == (c1.nation().addr() as u32)));
                if !same && free {
                    side = Some(m);
                }
            } else {
                let (home, away) = (read_i16(t), read_i16(t + 2));
                if home < 0 && away < 0 {
                    if away == -2 {
                        side = Some(home == -2 && exe::utils_random(2) != 0);
                    } else if home == -2 {
                        side = Some(true);
                    }
                }
            }
            if let Some(m) = side {
                break m;
            }
            if !next_tie_step(cup, r, &mut cur, start, &mut second, &mut rule) {
                show_error_at(0x9b55ac, 0xe5c);
                exe::crt_free(array);
                return;
            }
        };
        place_team_in_tie(cup, r, round, e, cur, mandante); // cqb11: a entrada vai ao confronto sorteado
        j = j.wrapping_add(1);
    }
    exe::crt_free(array);
}

/// the base cup season start: clears byte +0xb1
/// Original: `Cup::start_new_season` (0x518780, Cup.cpp, thiscall).
///
/// [pt] { 0x518780 } (thiscall, ret 0, Cup.cpp): `Cup::start_new_season()` da classe base: só zera o byte +0xb1 da copa
/// (as copas derivadas liberam as tabelas delas).
pub(crate) unsafe extern "thiscall" fn cup_start_new_season(cup: usize) {
    let cup = Cup(cup);
    cup.set_run_by_stages(0);
}

/// Address of the 0x68-byte record of round r (array at cup +0xa3).
///
/// [pt] O registro de 0x68 bytes da rodada `r` (+0xa3).
pub(crate) unsafe fn round_record(cup: Cup, r: i32) -> usize {
    cup.round_table().wrapping_add((r as isize * 0x68) as usize)
}

/// Address of the 6-byte entry of cup team t (+0xa7: +0 club, +4 level, +5 outcome).
///
/// [pt] A entrada de 6 bytes do time `t` (+0xa7: +0 o clube, +4 o nível, +5 o desfecho).
pub(crate) unsafe fn team_entry(cup: Cup, t: i32) -> usize {
    cup.team_table().wrapping_add((t as isize * 6) as usize)
}

/// Address of tie k (0x35 bytes) of round r (per-round arrays at cup +0xad).
///
/// [pt] O confronto `k` (0x35 bytes) da rodada `r` (+0xad: um vetor por rodada).
pub(crate) unsafe fn round_tie(cup: Cup, r: i32, k: i32) -> usize {
    read_usize(cup.tie_tables().wrapping_add((r as isize * 4) as usize)).wrapping_add((k as isize * 0x35) as usize)
}

/// O ensaio do comparador da `pay_round_prize_money`: a cópia roda sem a notícia do prêmio (`send_prize_money_news`, que aloca).
pub(crate) static ROUND_PRIZE_MONEY_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Sends the prize money news (Comp `send_prize_money_news`) for a club, amount, next round, fixture and outcome; nothing in dry-run
/// mode.
///
/// [pt] A notícia do prêmio (Comp 0x48e3e0: 7 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, valor, alto do
/// valor, −1, rodada seguinte, partida, desfecho), ret 0x1c); no ensaio, nada.
pub(crate) unsafe fn send_prize_news(cup: Cup, club: usize, value: i32, next: u32, fixture: Fixture, outcome: u8) {
    if ROUND_PRIZE_MONEY_DRY_RUN.load(Relaxed) {
        return;
    }
    // simplification: the original pushes the dword of a local of which it writes only the low byte (the high bits are stack leftover);
    // the news item only reads the read_i8(movsx ecx + (byte [esp+0x114])): here, the clean byte.
    exe::comp_send_prize_money_news(cup, club, value, value >> 31, -1, next, fixture, (outcome as u32) as i32);
}

/// Round r's prize (to winner, to loser), from the round record (+0x60/+0x64) or from the prize fund; None where the
/// original division would fault.
///
/// [pt] Os prêmios da rodada `r`: (ao vencedor, ao perdedor), do registro (+0x60/+0x64) ou do fundo (−1); None quando a
/// divisão da original cairia (divisor zero ou estouro). `contar`: o contador do ramo do fundo (não no comparador).
pub(crate) unsafe fn round_prizes(cup: Cup, r: i32, count: bool) -> Option<(i32, i32)> {
    let e = round_record(cup, r);
    let (mut ao_vencedor, mut ao_perdedor) = (read_i32(e + cup_round::WINNER_PRIZE), read_i32(e + cup_round::LOSER_PRIZE));
    if ao_vencedor == -1 {
        let (fundo, resta, confrontos) = (cup.prize_fund(), cup.round_count() as i32 - r, read_i16(e + cup_round::TIES) as i32);
        if resta == 0 || confrontos == 0 || (fundo == i32::MIN && resta == -1) {
            return None;
        }
        let q0 = fundo / resta;
        if q0 == i32::MIN && confrontos == -1 {
            return None;
        }
        if count {
            crate::comparison::branch("cqa_1 prize from the fund");
        }
        let q = q0 / confrontos; // cqa1: the fund by the remaining rounds and by the ties
        if ao_perdedor == -1 {
            ao_vencedor = q.wrapping_mul(3) / 4; // idiv by 4 rounding toward zero (cdq; and 3; sar 2)
            ao_perdedor = q / 4;
        } else {
            ao_vencedor = q;
        }
    }
    Some((ao_vencedor, ao_perdedor))
}

/// Pays the round prize money: the winner gets the news item and the money, the loser only the money; the fund loses
/// both.
///
/// `vencedor.pay_round_prize_money(perdedor, rodada, partida)` (winner, loser, round, fixture; 6-byte entries of
/// the two teams).
///
/// The round prizes are `+0x60` (winner) and `+0x64` (loser); with −1 for the winner, the fund (`+0x28`) / (rounds
/// `+0x3c` − round) / ties (`+0x1a`), and with −1 for the loser as well, 3/4 to the winner and 1/4 to the loser.
///
/// The winner (if the prize is not zero) gets the news item (`0x48e3e0`: the team's outcome `+5`, 3 if it was 1 in
/// fixture `0x136`, and the next round: its word `+7` if it is neither the last nor `0x8c`, over the high 16 bits of
/// the round pointer, as the original leaves in ebp) and the money (`0x5999a0` on the finances `0x5a0590`); the loser
/// only the money; the fund loses both.
/// Original: `Cup::pay_round_prize_money` (0x522240, Cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x522240 } (thiscall, ret 0x10, Cup.cpp): `vencedor.pay_round_prize_money(perdedor, rodada, partida)`
/// (entradas de 6 bytes dos dois times). Os prêmios da rodada (+0x60 o do vencedor, +0x64 o do perdedor); com −1 no
/// do vencedor, o fundo (+0x28) / (rodadas +0x3c − rodada) / confrontos (+0x1a), e com −1 também no do perdedor, 3/4
/// para o vencedor e 1/4 para o perdedor. O vencedor (se o prêmio não é zero) recebe a notícia (0x48e3e0: o
/// desfecho +5 do time, 3 se era 1 na partida 0x136, e a rodada seguinte: a palavra +7 dela se não é a última nem
/// 0x8c, sobre os 16 bits altos do ponteiro da rodada, como a original deixa no ebp) e o dinheiro (0x5999a0 nas
/// finanças 0x5a0590); o perdedor só o dinheiro; o fundo perde os dois.
pub(crate) unsafe extern "thiscall" fn pay_round_prize_money(cup: usize, winner: usize, loser: usize, round: u32, fixture: usize) {
    let cup = Cup(cup);
    let fixture = Fixture(fixture);
    let r = round as i16 as i32;
    let (ao_vencedor, ao_perdedor) = match round_prizes(cup, r, true) {
        Some(p) => p,
        None => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_PAY_ROUND_PRIZE_MONEY)(cup.addr(), winner, loser, round, fixture.addr()), // simplification: the original crashes on the division; let it crash there
    };
    let mut outcome = read_u8(winner + 5);
    if fixture.round_name() == 0x136 && outcome == 1 {
        outcome = 3;
    }
    let e = round_record(cup, r);
    let mut next = 0u32;
    if r + 1 < cup.round_count() as i32 {
        let w = read_u16(e + 0x6f);
        if w != 0x8c {
            next = (e as u32 & 0xffff_0000) | w as u32; // mov bp, [ebp+0x6f]: only the low word of ebp changes
        }
    }
    if ao_vencedor != 0 {
        send_prize_news(cup, read_usize(winner), ao_vencedor, next, fixture, outcome);
        crate::comparison::branch("cqa_1 winner prize");
        european_cup_add_prize_money(european_cup_club_finance(read_usize(winner)), ao_vencedor); // cqa1: the winner’s prize
        cup.set_prize_fund(cup.prize_fund().wrapping_sub(ao_vencedor));
    }
    if ao_perdedor != 0 {
        european_cup_add_prize_money(european_cup_club_finance(read_usize(loser)), ao_perdedor);
        cup.set_prize_fund(cup.prize_fund().wrapping_sub(ao_perdedor));
    }
}

/// Range [start, end) of ties `set_home_teams_for_round` walks: a single tie or, with -1, all ties of the round (+0x1a).
///
/// [pt] Os confrontos [início, fim) que a 0x51e870 percorre: um só (`laco`; o fim é a palavra baixa de laco + 1, como o
/// `lea edx, [eax+1]` da original) ou, com −1, todos os da rodada (+0x1a).
pub(crate) unsafe fn ties_range(cup: usize, round: u32, laco: u32) -> (i16, i16) {
    if laco as u16 != 0xffff {
        (laco as i16, laco.wrapping_add(1) as i16)
    } else {
        (0, read_i16(round_record(Cup(cup), round as i16 as i32) + 0x1a))
    }
}

/// Swaps home and away in tie t: the word pairs +0/+2, +0xa/+0xc and +0xe/+0x10.
///
/// [pt] Troca mandante e visitante no confronto `t`: os pares de palavras +0/+2, +0xa/+0xc e +0xe/+0x10.
pub(crate) unsafe fn swap_home_away(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xa, 0xc), (0xe, 0x10)] {
        let (x, y) = (read_i16(t + a), read_i16(t + b));
        write_i16(t + a, y);
        write_i16(t + b, x);
    }
}

/// Swaps home and away in one tie or in every tie of the round by the round rule: the club in the higher division
/// (virtual `+0x90`), the team level or the cup stadium.
///
/// `rodada.set_home_teams_for_round(laco)` (round, tie): in one tie (`laco`) or in all those of the round (−1),
/// with both teams decided (≠ −1), it swaps home and away by the round rule (low byte of `+0xb`): bit 1, if virtual
/// `+0x90` (club_in_higher_division(club a, club b)) returns club b; otherwise bit 8, if the level (`+4` of the team,
/// signed) of a is lower than b's; otherwise, with the cup stadium (`+0x24`) ≠ −1, if club b exists, has a stadium
/// (`+0x69`) and its id (`+0`) is the cup's.
/// Original: `Cup::set_home_teams_for_round` (0x51e870, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51e870 } (thiscall, ret 8, Cup.cpp): `rodada.set_home_teams_for_round(laco)`: num confronto (`laco`) ou
/// em todos os da rodada (−1), com os dois times definidos (≠ −1), inverte mandante e visitante pela regra da rodada
/// (byte baixo de +0xb): bit 1, se a virtual +0x90 (club_in_higher_division(clube a, clube b)) devolve o clube b;
/// senão bit 8, se o nível (+4 do time, com sinal) de a é menor que o de b; senão, com o estádio da copa (+0x24) ≠
/// −1, se o clube b existe, tem estádio (+0x69) e o id dele (+0) é o da copa.
pub(crate) unsafe extern "thiscall" fn set_home_teams_for_round(cup: usize, round: u32, laco: u32) {
    let cup = Cup(cup);
    let r = round as i16 as i32;
    let (ini, fim) = ties_range(cup.addr(), round, laco);
    let rule = read_u8(round_record(cup, r) + 0xb);
    if rule & 1 == 0 && rule & 8 == 0 && (cup.venue_stadium_id() as i32) == -1 {
        return;
    }
    let mut k = ini as i32;
    while k < fim as i32 {
        let t = round_tie(cup, r, k);
        let (a, b) = (read_i16(t), read_i16(t + 2));
        k += 1;
        if a == -1 || b == -1 {
            continue;
        }
        let (ta, tb) = (team_entry(cup, a as i32), team_entry(cup, b as i32));
        let inverter = if rule & 1 != 0 {
            let (ca, cb) = (read_usize(ta), read_usize(tb));
            // the virtual +0x90: push cb; push ca; ecx = the cup; no add esp -> thiscall(copa, ca, cb), ret 8
            let greater: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + 0x90));
            let m = greater(cup.addr(), ca, cb);
            m != 0 && m == cb
        } else if rule & 8 != 0 {
            (read_u8(ta + 4) as i8) < (read_u8(tb + 4) as i8)
        } else {
            let cb = read_usize(tb);
            cb != 0 && read_usize(cb + club::STADIUM) != 0 && read_i32(read_usize(cb + club::STADIUM)) == (cup.venue_stadium_id() as i32) // the cup stadium
        };
        if inverter {
            swap_home_away(t);
        }
    }
}

/// Arguments (p1, p2, p3) of virtual +0x68 that `record_team_progress` calls for team t in round r, by its outcome, or None.
///
/// [pt] Os argumentos da virtual +0x68 que a 0x521b30 chama para o time `t` na rodada `r`: (p1, p2, p3) ou None.
pub(crate) unsafe fn team_progress_args(cup: Cup, t: i32, r: i32) -> Option<(u8, u8, u8)> {
    let n = cup.round_count() as i32;
    let d = read_u8(team_entry(cup, t) + 5); // o desfecho
    let byte_of_round = |k: i32, desl: usize| read_u8(round_record(cup, k) + desl);
    if n > 0 && read_i16(round_record(cup, n - 1) + 7) == 0x8c {
        // the last round is the third-place one
        if r == n - 2 {
            return match d {
                1 => Some((1, 1, 1)),
                2 => Some((2, 2, 2)),
                _ => None,
            };
        }
        if r == n - 1 {
            return match d {
                3 => Some((3, 3, 3)),
                4 => Some((4, 4, 4)),
                _ => None,
            };
        }
        if d != 0 {
            let v = byte_of_round(r, 0x5b);
            return Some((v, v, v));
        }
        if r + 1 == n - 2 {
            return Some((4, 1, 4));
        }
        let v = byte_of_round(r + 1, 0x5b);
        return Some((v, 1, v));
    }
    if r == n - 1 {
        return match d {
            1 => Some((1, 1, 1)),
            2 => {
                let v = byte_of_round(r, 0x1a).wrapping_add(1);
                Some((v, v, v))
            }
            _ => None,
        };
    }
    if d != 0 {
        let v = byte_of_round(r, 0x5b);
        return Some((v, v, v));
    }
    if r + 1 == n - 1 {
        let v = byte_of_round(r + 1, 0x1a).wrapping_add(1);
        return Some((v, 1, v));
    }
    let v = byte_of_round(r + 1, 0x5b);
    Some((v, 1, v))
}

/// Records how far the team went in the cup, by outcome and round (with or without a third-place round), through
/// virtual `+0x68` (`set_club_progress(clube, competição +0x44, p1, p2, p3)`, i.e. club, competition `+0x44`, p1, p2,
/// p3).
///
/// `time.record_team_progress(rodada)` (team, round).
///
/// With a last round of type `0x8c` (third place): in the semi-final (rounds − 2), outcome 1 or 2 → (d, d, d); in the
/// last one, 3 or 4 → (d, d, d); in another round, outcome ≠ 0 → the round's `+0x5b` three times; 0 → (4, 1, 4) before
/// the semi-final, otherwise (x, 1, x) with `+0x5b` of the next round.
///
/// Without it: in the final, 1 → (1, 1, 1), 2 → the ties (`+0x1a`) of the final + 1 three times; in another round,
/// outcome ≠ 0 → `+0x5b` three times; 0 → (x, 1, x) with the ties + 1 of the final before it, otherwise `+0x5b` of the
/// next round. Outcomes other than these call nothing.
/// Original: `Cup::record_team_progress` (0x521b30, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x521b30 } (thiscall, ret 8, Cup.cpp): `time.record_team_progress(rodada)`: quanto o time foi na copa,
/// pela virtual +0x68 (`set_club_progress(clube, competição +0x44, p1, p2, p3)`). Com a última rodada do tipo 0x8c
/// (terceiro lugar): na semifinal (rodadas − 2), desfecho 1 ou 2 → (d, d, d); na última, 3 ou 4 → (d, d, d); noutra
/// rodada, desfecho ≠ 0 → o +0x5b da rodada três vezes; 0 → (4, 1, 4) antes da semifinal, senão (x, 1, x) com o +0x5b
/// da rodada seguinte. Sem ela: na final, 1 → (1, 1, 1), 2 → os confrontos (+0x1a) da final + 1 três vezes; noutra,
/// desfecho ≠ 0 → o +0x5b três vezes; 0 → (x, 1, x) com os confrontos + 1 da final antes dela, senão o +0x5b da
/// seguinte. Os desfechos fora desses não chamam nada.
pub(crate) unsafe extern "thiscall" fn record_team_progress(cup: usize, team: u32, round: u32) {
    let cup = Cup(cup);
    let (t, r) = (team as i16 as i32, round as i16 as i32);
    if let Some((p1, p2, p3)) = team_progress_args(cup, t, r) {
        // the virtual +0x68: 5 pushes; ecx = the cup; no add esp -> thiscall(copa, clube, comp, p1, p2, p3), ret 0x14
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(cup.addr()) + 0x68));
        // simplification: the original pushes bytes over whole registers (the caller’s ebx, the argument’s dword
        // `time` with the low byte replaced, 13 × round): the high bits are leftovers; the 70 versions of +0x68 only read the byte
        // low byte of each (or pass the dwords on to another that reads): here, the clean bytes.
        f(cup.addr(), read_usize(team_entry(cup, t)), cup.stage_index() as u32, p1 as u32, p2 as u32, p3 as u32);
    }
}

/// Address of the 0x68-byte record of round r (+0xa3), r used as the full dword like the original.
///
/// [pt] O registro de 0x68 bytes da rodada `r` (+0xa3); `r` como a original o usa (o dword inteiro).
pub(crate) unsafe fn round_record_by_dword(cup: Cup, r: u32) -> usize {
    cup.round_table().wrapping_add((r as usize).wrapping_mul(0x68))
}

/// Start of round r's tie array (0x35-byte ties, per-round pointers at cup +0xad).
///
/// [pt] Os confrontos da rodada `r` (+0xad: um vetor de confrontos de 0x35 bytes por rodada).
pub(crate) unsafe fn round_ties_array(cup: Cup, r: u32) -> usize {
    read_usize(cup.tie_tables().wrapping_add((r as usize).wrapping_mul(4)))
}

/// True when some defined team of a round's ties still has no club (an earlier tie undecided): the round cannot be
/// drawn.
///
/// [pt] Algum time definido (≠ −1) de algum confronto da rodada ainda sem clube (o vencedor de um confronto anterior em
/// aberto): a rodada não pode ser sorteada.
pub(crate) unsafe fn round_has_open_team(cup: Cup, r: u32) -> bool {
    let reg = round_record_by_dword(cup, r);
    let mut t = round_ties_array(cup, r);
    let mut k = 0i32;
    while k < read_i16(reg + cup_round::TIES) as i32 {
        for lado in [0usize, 2] {
            let x = read_i16(t + lado);
            if x != -1 && read_usize(cup.team_table().wrapping_add((x as isize * 6) as usize)) == 0 {
                return true;
            }
        }
        t += 0x35;
        k += 1;
    }
    false
}

/// Builds the round's draw date (+0 the day, +2 the year as a byte relative to the cup's +0x40) into a local Date.
///
/// 0x549ef0: push year; push day; ecx = the local; no add esp -> thiscall, ret 8; both are in registers with garbage in
/// the high bits, and the Rust only reads the words.
///
/// [pt] A data do sorteio da rodada (+0 o dia, +2 o ano em byte a partir de +0x40 da copa), numa Data local (0x549ef0:
/// push ano; push dia; ecx = a local; sem add esp -> thiscall, ret 8; os dois em registradores com lixo nos bits altos,
/// a Rust só lê as palavras).
pub(crate) unsafe fn round_draw_date(cup: Cup, reg: usize, d: *mut [u8; 8]) {
    let year = (read_u8(reg + 2) as i8 as i16).wrapping_add(cup.year());
    exe::new_date((d) as *mut [u8; 8] as usize, read_i16(reg) as u16 as u32, year as u16 as u32);
}

/// Stores date d as the round's draw date: the day word and the year byte minus the low byte of cup +0x40.
///
/// [pt] Grava a data `d` como a do sorteio da rodada: a palavra do dia e o byte do ano menos o byte baixo de +0x40.
pub(crate) unsafe fn set_round_draw_date(cup: Cup, reg: usize, d: &[u8; 8]) {
    core::ptr::write_unaligned(reg as *mut [u8; 2], [d[0], d[1]]);
    write_u8(reg + 2, d[2].wrapping_sub((cup.year() as u8)));
}

/// True when date a is after date b (unsigned year, then signed day), as `draw_round` compares.
///
/// [pt] A data `a` vem depois da `b` (o ano sem sinal, depois o dia com sinal), como a comparação da 0x51c410.
pub(crate) fn cup_date_is_after(a: &[u8; 8], b: &[u8; 8]) -> bool {
    let (aa, ab) = (u16::from_le_bytes([a[2], a[3]]), u16::from_le_bytes([b[2], b[3]]));
    aa > ab || (aa == ab && i16::from_le_bytes([a[0], a[1]]) > i16::from_le_bytes([b[0], b[1]]))
}

/// True when the competition gets no draw news item.
///
/// That is the case with fewer than 2 ties, with rule 0x200 (+0xd), for the special national-team competition (0x76d940
/// on [0xb74318]: push comp; ecx = the national teams; no add esp -> ret 4; the Rust copy is stdcall(comp)) or for a
/// friendly (`exe::friendly_is_friendly_competition` on [0xae2c74]: push comp; ecx = the Friendly -> thiscall, ret 4).
///
/// [pt] A competição não leva a notícia do sorteio: menos de 2 confrontos, regra 0x200 (+0xd), competição especial das
/// seleções (0x76d940 em [0xb74318]: push comp; ecx = as seleções; sem add esp -> ret 4; a cópia Rust é stdcall(comp))
/// ou exe::friendly_is_friendly_competition(0x5c6710 em [0xae2c74]: push comp; ecx = o Friendly -> thiscall, ret 4).
pub(crate) unsafe fn skips_draw_news(cup: Cup, r: u32) -> bool {
    let reg = round_record_by_dword(cup, r);
    if read_i16(reg + cup_round::TIES) < 2 || read_i16(reg + cup_round::TIE_RULES) & 0x200 != 0 {
        return true;
    }
    if read_usize(G_NATIONAL_TEAMS) != 0 {
        let special: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
        if special(read_usize(cup.addr() + 4)) != 0 {
            return true;
        }
    }
    if read_usize(G_FRIENDLY) != 0 {
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(cup.addr() + 4)) != 0 {
            return true;
        }
    }
    false
}

/// Draws a cup round: with some team still open it puts the draw off by 1 day; otherwise it draws by the round rule,
/// sets the home teams and the shared stadium, marks the drawn ties and sends the draw news to human-managed teams.
/// Returns 1.
///
/// Convention: thiscall, ret 4, `Cup.cpp`, SEH frame with no destructor on the normal path. `rodada.draw_round()`.
///
/// Without `+0x45` or with the round already drawn (`+0x34` of the first tie): nothing. With a team of some tie still
/// without a club: the draw date moves 1 day (`0x54c5b0`, `0x417c10`) and those of the following rounds become at least
/// that date.
///
/// Otherwise it draws: rule `0x400` (`+0xd`) by seeding (`0x51d2d0`), rule 4 (`+0xb`) keeping nations apart
/// (`0x51d9f0`), without `0x200` the open draw (`0x51e220`); the home teams (`0x51e870` with −1); shared stadium
/// (`+0x21` = 2, `0x522610`); marks the drawn ties (`+0x34` = 1).
///
/// With news (skips_draw_news): a round that is not the third-place one (`+7` ≠ `0x8c`) adds 1 to `+0xac`; news item
/// `0x27` (the competition, `+0x3a`, the type, `+0x44`, the round, 0, and 1 if `+0xab` + 1 < `+0xac`) goes to each of
/// the teams still alive (outcome 0) among the first ones (sum of the entries `+0x1c` of the rounds up to this one)
/// that has a human manager, with the manager's id in data item 3.
/// Original: `Cup::draw_round` (0x51c410, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51c410 } (thiscall, ret 4, Cup.cpp, quadro SEH sem destrutor no caminho normal): `rodada.draw_round()`;
/// devolve 1. Sem +0x45 ou com a rodada já sorteada (+0x34 do primeiro confronto): nada. Com um time de algum confronto
/// ainda sem clube: a data do sorteio anda 1 dia (0x54c5b0, 0x417c10) e as das rodadas seguintes passam a ser, no
/// mínimo, essa. Senão sorteia: regra 0x400 (+0xd) pela semente (0x51d2d0), regra 4 (+0xb) separando as nações
/// (0x51d9f0), sem 0x200 o sorteio livre (0x51e220); os mandantes (0x51e870 com −1); estádio compartilhado (+0x21 = 2,
/// 0x522610); marca os confrontos sorteados (+0x34 = 1). Com notícia (skips_draw_news): rodada que não é a do
/// terceiro lugar (+7 ≠ 0x8c) soma 1 em +0xac; a notícia 0x27 (a competição, +0x3a, o tipo, +0x44, a rodada, 0, e 1
/// se +0xab + 1 < +0xac) vai a cada um dos times ainda vivos (desfecho 0) entre os primeiros (soma das entradas +0x1c
/// das rodadas até esta) que é de técnico humano, com o id dele no dado 3.
pub(crate) unsafe extern "thiscall" fn draw_round(cup: usize, round: u32) -> u32 {
    let cup = Cup(cup);
    if cup.season_set_up() == 0 || read_u8(round_ties_array(cup, round) + 0x34) != 0 {
        return 1;
    }
    let reg = round_record_by_dword(cup, round);
    if round_has_open_team(cup, round) {
        let more: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_ADD_DAYS);
        let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        round_draw_date(cup, reg, &mut a);
        crate::comparison::branch("cqa_3 draw postponed");
        let r = more(&mut a, &mut b, 1); // cqa3: the deferred random draw
        exe::date_operator_3d((&mut a) as *mut [u8; 8] as usize, (&mut c) as *mut [u8; 8] as usize, (r) as *mut [u8; 8] as usize); // push &b; push &c; ecx = &a -> thiscall, ret 8
        set_round_draw_date(cup, reg, &a);
        let mut k = round.wrapping_add(1);
        while (k as i16) < cup.round_count() {
            let rk = round_record_by_dword(cup, k as i16 as i32 as u32);
            round_draw_date(cup, rk, &mut b);
            if cup_date_is_after(&a, &b) {
                set_round_draw_date(cup, rk, &a);
            }
            k = k.wrapping_add(1);
        }
        return 1;
    }
    // the draws: push round; ecx = the cup; no add esp -> thiscall, ret 4 (all three)
    let sorteio = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
        g(cup.addr(), round)
    };
    let rule = read_i16(reg + cup_round::TIE_RULES);
    if rule & 0x400 != 0 {
        crate::comparison::branch("cqa_3 seeded draw");
        sorteio(0x51d2d0); // cqa3: by the seed
    } else if read_u8(reg + cup_round::DRAW_RULES) & 4 != 0 {
        sorteio(0x51d9f0);
    } else if rule & 0x200 == 0 {
        crate::comparison::branch("cqa_3 open draw");
        sorteio(0x51e220); // cqa3: the free random draw
    }
    // push −1; push rodada; ecx = a copa -> thiscall, ret 8
    exe::cup_set_home_teams_for_round(cup, round, u32::MAX);
    if read_u8(round_record_by_dword(cup, round) + 0x21) == 2 {
        sorteio(0x522610);
    }
    let mut k = 0i32;
    while k < read_i16(round_record_by_dword(cup, round) + 0x1a) as i32 {
        write_u8(round_ties_array(cup, round) + k as usize * 0x35 + 0x34, 1);
        k += 1;
    }
    if skips_draw_news(cup, round) {
        return 1;
    }
    let reg = round_record_by_dword(cup, round);
    if read_i16(reg + cup_round::ROUND_CODE) != 0x8c {
        cup.set_next_draw_round(cup.next_draw_round().wrapping_add(1));
    }
    let mut sum: i16 = 0;
    if (round as i32) >= 0 {
        for i in 0..=round {
            sum = sum.wrapping_add(read_i16(round_record_by_dword(cup, i) + 0x1c));
        }
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    exe::new_newsitem(n, 0x27, 0); // push 0; push 0x27; ecx = a local -> thiscall, ret 8
    exe::newsitem_set_slot(n, 0, read_usize(read_usize(cup.addr() + 4)) as u32); // push valor; push k; ecx = a local -> thiscall, ret 8
    exe::newsitem_set_slot(n, 1, cup.stage_label() as i32 as u32);
    exe::newsitem_set_slot(n, 2, read_i16(round_record_by_dword(cup, round) + 7) as i32 as u32);
    exe::newsitem_set_slot(n, 4, cup.stage_index() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 5, round);
    exe::newsitem_set_slot(n, 6, 0);
    exe::newsitem_set_slot(n, 7, ((cup.current_round() as i8 as i32 + 1) < cup.next_draw_round() as i8 as i32) as u32);
    for i in 0..sum.max(0) as usize {
        let e = cup.team_table() + 6 * i;
        if read_u8(e + 5) != 0 || read_usize(e) == 0 {
            continue;
        }
        // 0x7875e0: push club; ecx = [0xb7431c] -> ret 4 (the Rust copy is stdcall(club))
        if human(read_usize(e)) == 0 {
            continue;
        }
        exe::newsitem_set_slot(n, 3, read_usize(read_usize(cup.team_table() + 6 * i)) as u32);
        // push club; push &local; ecx = [0xb7431c]; no add esp -> thiscall, ret 8
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, read_usize(cup.team_table() + 6 * i));
    }
    1
}

/// Club's division level (word +0x69 of its division +0x57), or 0 without division.
///
/// [pt] O nível da divisão do clube: a palavra +0x69 da divisão (+0x57), ou 0 sem divisão.
pub(crate) unsafe fn division_level(club: Club) -> i16 {
    let d = club.division();
    if !d.is_null() {
        d.reputation()
    } else {
        0
    }
}

/// returns the club whose division level (+0x57 -> +0x69) is lower, or 0 when level
/// Original: `Cup::club_in_higher_division` (0x522360, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x522360 } (thiscall, ret 8, Cup.cpp; o this não é lido; a virtual +0x90 da copa base):
/// `a.club_in_higher_division(b)`: o clube de nível menor (+0x57 → +0x69, com sinal; 0 sem divisão); empate, 0.
pub(crate) unsafe extern "thiscall" fn club_in_higher_division(_this: usize, a: usize, b: usize) -> usize {
    let (na, nb) = (division_level(Club(a)), division_level(Club(b)));
    if na < nb {
        a
    } else if na == nb {
        0
    } else {
        b
    }
}

/// Club of cup team t (+0xa7, 6 bytes per team, +0 club).
///
/// [pt] O clube do time `t` (+0xa7, 6 bytes por time, +0 o clube).
pub(crate) unsafe fn club_of_team(cup: Cup, t: i32) -> usize {
    read_usize(cup.team_table().wrapping_add((t as isize * 6) as usize))
}

/// Address of the club's 6-byte state record in G_CLUB_STATE_TABLE (+0 = season's European competition).
///
/// [pt] O registro de 6 bytes do estado do clube ([0xae2d14] + id × 6; +0 a competição europeia da temporada).
pub(crate) unsafe fn club_state_record(club: usize) -> usize {
    read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(club) as isize * 6) as usize)
}

/// True when the competition is the friendly one (Friendly::is_friendly_competition on G_FRIENDLY).
///
/// [pt] A competição é a amistosa (0x5c6710: push comp; ecx = [0xae2c74]; sem add esp -> thiscall, ret 4).
pub(crate) unsafe fn cup_is_friendly_comp(comp: usize) -> bool {
    exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0
}

/// Key-table index of the nation that owns the competition (`key_nation_index_of_comp`, cdecl).
///
/// [pt] O índice da nação dona da competição na tabela das chaves (0x669470: push comp; add esp, 4 -> cdecl; devolve o byte).
pub(crate) unsafe fn cup_key_nation_index_of_comp(comp: usize) -> u8 {
    exe::key_nation_index_of_comp(comp)
}

/// True when the competition is its nation's stored main cup ([0xb63d7e + 72 × index]).
///
/// [pt] A competição é a copa principal guardada da nação dela ([0xb63d7e + 72 × índice]).
pub(crate) unsafe fn is_main_national_cup(comp: usize) -> bool {
    read_usize(0xb63d7e_usize.wrapping_add((cup_key_nation_index_of_comp(comp) as i8 as isize * 72) as usize)) == comp
}

/// Whether the cup winner (`cup_update_staff_records_for_status`, outcome 1) earns a UEFA Cup place.
///
/// Always for the English League Cup ([`0x9cf750`]); otherwise for the main cup of a European nation ([`0x9cfa10`] is
/// the continent, via nation `+0x5d` and continent `+0x71`), when it is not a friendly, the club has no European
/// competition yet (−1) and the nation has an index.
///
/// [pt] A vaga na Copa da UEFA que o campeão da copa (`cup_update_staff_records_for_status`, desfecho 1) ganha: a Copa da Liga
/// inglesa ([0x9cf750]) sempre; senão, a copa principal de uma nação europeia ([0x9cfa10] o continente, pela nação
/// +0x5d e o continente +0x71), não amistosa, com o clube ainda sem competição europeia (−1) e a nação com índice.
pub(crate) unsafe fn wins_uefa_cup_place(comp: Competition, club: usize) -> bool {
    if read_i32(comp.addr()) == read_i32(KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP) {
        return true;
    }
    let nation = comp.nation();
    if nation.is_null() {
        return false;
    }
    let continent = nation.continent();
    if continent == 0 || read_i32(continent) != read_i32(KEY_CONTINENT_EUROPE) || cup_is_friendly_comp(comp.addr()) {
        return false;
    }
    if read_i32(club_state_record(club)) != -1 || cup_key_nation_index_of_comp(comp.addr()) == 0xff {
        return false;
    }
    is_main_national_cup(comp.addr())
}

/// Calls a staff-records final routine f (winner/runner-up/third) on G_STAFF_RECORDS with club, match and final.
///
/// [pt] Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push final; push jogo; push clube;
/// ecx = [0xdd7f00]; sem add esp -> thiscall(registros, clube, jogo, final), ret 0xc).
pub(crate) unsafe fn cup_record_final_result(f: usize, club: usize, game: Fixture, final_: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(read_usize(G_STAFF_RECORDS), club, game.addr(), final_);
}

/// Updates the staff records by the club's outcome in the cup: the winners record (and the UEFA Cup place through the
/// English League Cup or a European nation's main cup), runners-up, third place, qualified for the next round or
/// knocked out, weighted by round. Returns 0.
///
/// Convention: thiscall, ret `0x18`, `Cup.cpp`; virtual `+0x44` of the English cups and of the base class.
/// `clube.update_staff_records_for_status(desfecho, _, _, jogo, final)` (club, outcome, _, _, fixture, final).
///
/// Nothing with fixture `+0x3a` = 1, with `+0x2c` or with `+0x44` ≠ `0xff`. Outcome 1: the winners record and, with the
/// place (wins_uefa_cup_place), the club's state receives the UEFA Cup (`[0x9cf6f0]`); 2: runners-up; 3: third place;
/// 0: if the next round (`+0x34` + 1) exists and is not the third-place one (`+7` ≠ `0x8c`), the qualification
/// (`0x86bdd0`) with weight round × 99 / (rounds − 1) + 1; `0xff`: the knock-out (`0x86c000`) with weight round × 99 /
/// rounds + 1.
///
/// The stage records take the dwords the original pushes: the word of the fixture (`+0x32`, `+0x30`) or of the round
/// (`+7`) over the high 16 bits of the register.
/// Original: `Cup::update_staff_records_for_status` (0x519890, Cup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x519890 } (thiscall, ret 0x18, Cup.cpp; a virtual +0x44 das copas inglesas e da base):
/// `clube.update_staff_records_for_status(desfecho, _, _, jogo, final)`; devolve 0. Nada com o jogo +0x3a = 1,
/// com +0x2c ou com +0x44 ≠ 0xff. Desfecho 1: o registro de campeão e, com a vaga (wins_uefa_cup_place), o estado do
/// clube recebe a Copa da UEFA ([0x9cf6f0]); 2: o de vice; 3: o de terceiro; 0: se a rodada seguinte (+0x34 + 1) existe
/// e não é a do terceiro lugar (+7 ≠ 0x8c), a classificação (0x86bdd0) com o peso rodada × 99 / (rodadas − 1) + 1;
/// 0xff: a eliminação (0x86c000) com o peso rodada × 99 / rodadas + 1. Os registros de fase levam os dwords que a
/// original empurra: a palavra do jogo (+0x32, +0x30) ou da rodada (+7) sobre os 16 bits altos do registrador.
pub(crate) unsafe extern "thiscall" fn cup_update_staff_records_for_status(cup: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let cup = Cup(cup);
    let game = Fixture(game);
    if game.category() == 1 || cup.stage_count() != 0 || cup.stage_index() != 0xff {
        return 0;
    }
    let comp = read_usize(cup.addr() + 4);
    match outcome as u8 {
        1 => {
            crate::comparison::branch("cqa_4 champion");
            cup_record_final_result(0x86a800, club, game, final_); // cqa4: the champion record
            if wins_uefa_cup_place(Competition(comp), club) {
                write_i32(club_state_record(club), read_i32(KEY_CLUB_COMP_UEFA_CUP));
            }
        }
        2 => {
            crate::comparison::branch("cqa_4 runner-up");
            cup_record_final_result(0x86b0b0, club, game, final_); // cqa4: the runner-up record
        }
        3 => cup_record_final_result(0x86b710, club, game, final_),
        0 => {
            let r = game.round_number();
            let n = cup.round_count();
            let next = r.wrapping_add(1);
            if next >= n {
                return 0;
            }
            let w7 = read_u16(round_record(cup, next as i32) + 7);
            if w7 == 0x8c {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(cup.addr(), club, outcome, a3, a4, game.addr(), final_); // simplification: the original falls into the division
            }
            let q = (r as i32 * 99) / div;
            let weight = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let stage = (game.addr() as u32 & 0xffff_0000) | (game.round_prefix() as u16) as u32; // mov cx, [ecx+0x32]
            let round = (cup.round_table() as u32 & 0xffff_0000) | w7 as u32; // mov di, [edi+...+7]
            // push weight; push round; push stage; push comp; push club; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
            f(read_usize(G_STAFF_RECORDS), club, comp, stage, round, weight);
        }
        0xff => {
            let r = game.round_number() as i32;
            let n = cup.round_count() as i32;
            if n == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(cup.addr(), club, outcome, a3, a4, game.addr(), final_); // simplification: the original falls into the division
            }
            let (q, resto) = ((r * 99) / n, (r * 99) % n);
            let weight = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32;
            let stage = (weight & 0xffff_0000) | (game.round_prefix() as u16) as u32; // mov ax, [ecx+0x32]
            let round = (resto as u32 & 0xffff_0000) | (game.round_name() as u16) as u32; // mov dx, [ecx+0x30]
            // push weight; push round; push stage; push comp; push club; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
            f(read_usize(G_STAFF_RECORDS), club, comp, stage, round, weight);
        }
        _ => {}
    }
    0
}

/// The four names of the cup's history line: (winner, runner-up, third, host national team), with special cases for the
/// Welsh cup and the host country.
///
/// The winner, runner-up and third come from the teams' outcome (1, 2, 3; until all three are found). In the Welsh cup
/// ([0x9cf958]) the third is the first team with progress +0x73 = 3 in Comp_Stats (`exe::comp_stats_find_club_entry` on +8 of the competition
/// object in [0xadadfc]: push club; ecx = the stats -> thiscall, ret 4).
///
/// The host: using virtual +0x88 of the competition object (ecx = the object; no push -> thiscall, ret 0) and the
/// year's hosts (+0x40) from HostCountry ([0xb5e134], `find_hosts`: push 0; push &b; push &a; push year; push id; ecx =
/// [0xb5e134] -> thiscall, ret 0x14). With two hosts, the main national team (`exe::database_get_national_team`: push nation; add esp, 4 ->
/// cdecl) of the one with the larger +0x8e (on a tie, the first); with one host, that of the first; with none, 0.
///
/// [pt] Os quatro da linha do histórico: (campeão, vice, terceiro, seleção anfitriã). O campeão, o vice e o terceiro pelo
/// desfecho dos times (1, 2, 3; até achar os três); na copa galesa ([0x9cf958]) o terceiro é o primeiro time com o
/// progresso +0x73 = 3 no Comp_Stats (0x4a4850 de +8 do objeto da competição em [0xadadfc]: push clube; ecx = as
/// estatísticas -> thiscall, ret 4). A anfitriã: com a virtual +0x88 do objeto da competição (ecx = o objeto; sem push
/// -> thiscall, ret 0) e as anfitriãs do ano (+0x40) da HostCountry ([0xb5e134], 0x5fa730: push 0; push &b; push &a;
/// push ano; push id; ecx = [0xb5e134] -> thiscall, ret 0x14): duas, a seleção principal (0x53b3d0: push nação; add
/// esp, 4 -> cdecl) da de +0x8e maior (empate, a primeira); uma, a da primeira; nenhuma, 0.
pub(crate) unsafe fn season_honours(cup: Cup) -> (usize, usize, usize, usize) {
    let n = cup.team_count();
    let (mut campeao, mut vice, mut terceiro) = (0usize, 0usize, 0usize);
    let mut i = 0i16;
    while i < n {
        let e = cup.team_table().wrapping_add((i as isize * 6) as usize);
        match read_u8(e + 5) {
            1 => campeao = read_usize(e),
            2 => vice = read_usize(e),
            3 => terceiro = read_usize(e),
            _ => {}
        }
        if campeao != 0 && vice != 0 && terceiro != 0 {
            break;
        }
        i += 1;
    }
    let comp = read_usize(cup.addr() + 4);
    let object = || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(read_usize(cup.addr() + 4)) as isize * 4) as usize));
    if comp != 0 && read_i32(comp) == read_i32(KEY_CLUB_COMP_WELSH_PREMIER_CUP) {
        let mut j = 0i16;
        while j < cup.team_count() {
            let c = club_of_team(cup, j as i32);
            if read_u8(exe::comp_stats_find_club_entry(read_usize(object() + 8), c) + 0x73) == 3 {
                terceiro = c;
                break;
            }
            j += 1;
        }
    }
    let o = object();
    let has: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + 0x88));
    let mut anfitria = 0usize;
    if has(o) != 0 {
        let (mut a, mut b) = (0i32, 0i32);
        // simplification: the year goes in a register with garbage in the high 16 bits; 0x5fa730 receives a short
        let k = (exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_i32(read_usize(cup.addr() + 4)) as usize, cup.year() as u16 as u32, &mut a, &mut b, 0) as u8);
        let nation = |x: i32| read_usize(G_NATION_TABLE).wrapping_add((x as isize * 0x122) as usize);
        if k == 2 {
            let (na, nb) = (nation(a), nation(b));
            anfitria = if read_i16(na + nation::REPUTATION) < read_i16(nb + nation::REPUTATION) { exe::database_get_national_team(nb) } else { exe::database_get_national_team(na) };
        } else if k != 0 {
            anfitria = exe::database_get_national_team(nation(a));
        }
    }
    (campeao, vice, terceiro, anfitria)
}

/// Records the season honours: finds the winner, runner-up, third (in the Welsh cup, through the statistics) and the
/// host national team, writes the history row and, in the nation's main cup, passes the semi-final losers to UEFA.
///
/// Convention: thiscall, ret 0, `Cup.cpp`; virtual `+0x30` of the English cups and of the base class.
/// `Cup::record_season_honours()`: the history row (`0x4afce0`: 5 pushes; add esp, `0x14` -> cdecl(cup, winner,
/// runner-up, third, host)) with the four from season_honours; outside the friendly competition, in the nation's main
/// cup (is_main_national_cup), the semi-finalists go to UEFA (`0x519fc0`: ecx = the cup -> thiscall, ret 0).
/// Original: `Cup::record_season_honours` (0x519a90, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x519a90 } (thiscall, ret 0, Cup.cpp; a virtual +0x30 das copas inglesas e da base): `Cup::record_season_honours()`:
/// a linha do histórico (0x4afce0: 5 pushes; add esp, 0x14 -> cdecl(copa, campeão, vice, terceiro, anfitriã)) com os
/// quatro de season_honours; fora da competição amistosa, na copa principal da nação (is_main_national_cup), os
/// semifinalistas para a UEFA (0x519fc0: ecx = a copa -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cup_record_season_honours(cup: usize) {
    let (campeao, vice, terceiro, anfitria) = season_honours(Cup(cup));
    exe::comp_util_add_history_record(cup, campeao, vice, terceiro, anfitria);
    let comp = read_usize(cup + 4);
    if !cup_is_friendly_comp(comp) && is_main_national_cup(comp) {
        exe::cup_register_semi_final_losers_for_uefa(cup);
    }
}

/// The two clubs (ids) that `register_semi_final_losers_for_uefa` sends to the UEFA seeding (semi-final losers not in the final), or None when it
/// exits without sending anything.
///
/// Round 0x82 (the semi-finals) and 0x96 (the final), both found; it walks the fixtures of 0x82 and the teams that are
/// not in the first fixture of 0x96: the first becomes a, the second b (then it stops); a team of −1 (still without a
/// fixture) cancels; if one is missing, −1.
///
/// [pt] Os dois clubes (ids) que a 0x519fc0 manda à semeadura da UEFA, ou None quando ela sai sem mandar: a rodada 0x82
/// (as semifinais) e a 0x96 (a final), as duas achadas; percorre os confrontos da 0x82 e os times que não estão no
/// primeiro confronto da 0x96: o primeiro vira a, o segundo b (aí para); um time −1 (ainda sem confronto) cancela;
/// faltando, −1.
pub(crate) unsafe fn semi_final_losers(cup: Cup) -> Option<(i32, i32)> {
    let (mut semi, mut final_) = (-1i16, -1i16);
    let mut i = 0i16;
    while i < cup.round_count() {
        let t = read_i16(round_record(cup, i as i32) + 7);
        if t == 0x82 {
            semi = i;
        } else if t == 0x96 {
            final_ = i;
        }
        if semi != -1 && final_ != -1 {
            break;
        }
        i += 1;
    }
    if semi == -1 || final_ == -1 {
        return None;
    }
    let confrontos = |r: i16| read_usize(cup.tie_tables().wrapping_add((r as isize * 4) as usize));
    let f0 = confrontos(final_);
    let (fa, fb) = (read_i16(f0), read_i16(f0 + 2));
    let n = read_i16(round_record(cup, semi as i32) + 0x1a);
    let base = confrontos(semi);
    let (mut a, mut b) = (-1i32, -1i32);
    let mut k = 0i16;
    'lacos: while k < n {
        let t = base.wrapping_add((k as isize * 0x35) as usize);
        for lado in [0usize, 2] {
            let x = read_i16(t + lado);
            if x == fa || x == fb {
                continue;
            }
            if x == -1 {
                return None;
            }
            let id = read_i32(club_of_team(cup, x as i32));
            if a == -1 {
                a = id;
            } else {
                b = id;
                break 'lacos;
            }
        }
        k += 1;
    }
    Some((a, b))
}

/// Finds the semi-finals (`0x82`) and the final (`0x96`) and sends the two semi-finalists not in the final to the UEFA
/// seeding.
///
/// `Cup::register_semi_final_losers_for_uefa()`: the semi-finalists outside the final (semi_final_losers) go to the
/// UEFA seeding (`0x905790`: push b; push a; push competition id; ecx = `[0xde1f58]`; no add esp -> thiscall, ret
/// `0xc`).
/// Original: `Cup::register_semi_final_losers_for_uefa` (0x519fc0, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x519fc0 } (thiscall, ret 0, Cup.cpp): `Cup::register_semi_final_losers_for_uefa()`: os semifinalistas fora da
/// final (semi_final_losers) vão à semeadura da UEFA (0x905790: push b; push a; push id da competição; ecx =
/// [0xde1f58]; sem add esp -> thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn register_semi_final_losers_for_uefa(cup: usize) {
    if let Some((a, b)) = semi_final_losers(Cup(cup)) {
        exe::uefaseeding_record_cup_semi_final_losers(read_usize(G_UEFA_SEEDING), read_i32(read_usize(cup + 4)), a, b);
    }
}

/// O registro de 59 bytes da tabela da liga.
pub(crate) const CUP_LEAGUE_TABLE_ROW_SIZE: usize = 0x3b;

/// Copies one 59-byte row record (CUP_LEAGUE_TABLE_ROW_SIZE) from origem to destino.
///
/// [pt] Copia um registro de 59 bytes (o `rep movsd` de 14 dwords, a palavra e o byte da original).
pub(crate) unsafe fn copy_row_59(target: usize, source: usize) {
    core::ptr::copy(source as *const u8, target as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE);
}

/// Puts table pair (i, j) in grid slots k and half + k, marks both slots and both clubs used, increments k.
///
/// [pt] Põe o par (i, j) da tabela nas vagas k e metade + k da grade, marca as vagas e os dois clubes.
pub(crate) unsafe fn place_pair(grid: usize, posto: *mut u32, used: *mut u32, table: usize, i: usize, j: usize, k: &mut i32, half: i32) {
    copy_row_59(grid + *k as usize * CUP_LEAGUE_TABLE_ROW_SIZE, table + i * CUP_LEAGUE_TABLE_ROW_SIZE);
    let m = half + *k;
    copy_row_59(grid + m as usize * CUP_LEAGUE_TABLE_ROW_SIZE, table + j * CUP_LEAGUE_TABLE_ROW_SIZE);
    *posto.add(m as usize) = 1;
    *posto.add(*k as usize) = 1;
    *used.add(j) = 1;
    *used.add(i) = 1;
    *k += 1;
}

/// Core of `collect_possible_teams`: collects the clubs that can play one side of a fixture, descending to feeder ties for empty sides;
/// returns the count.
///
/// The fixture is match `jogo` of round `rodada` ([[+0xad] + rodada × 4] + jogo × 0x35). Each side (+0 and +2) that has
/// a club goes to `saida[*conta]` (and the count goes up); an empty side (−1) descends to the match it comes from
/// (+0xe/+0xa and +0x10/+0xc).
///
/// A negative round or match −1 (also during the descents): the error box (line 0x1592, if `caixa`) and the count.
///
/// Returns the count.
///
/// [pt] O núcleo da 0x5219b0: o jogo `jogo` da rodada `rodada` ([[+0xad] + rodada × 4] + jogo × 0x35): cada lado (+0 e
/// +2) com clube vai a `saida[*conta]` (e conta sobe); lado vazio (−1) desce ao jogo de onde ele vem (+0xe/+0xa e
/// +0x10/+0xc). Rodada negativa ou jogo −1 (também nas descidas): a caixa (linha 0x1592, se `caixa`) e a conta.
/// Devolve a conta.
pub(crate) unsafe fn collect_fixture_teams(this: Cup, round: i32, game: i32, out: *mut i16, count: *mut i32, caixa: bool) -> i32 {
    if round < 0 || game == -1 {
        if caixa {
            show_error_at(0x9b55ac, 0x1592);
        }
        return *count;
    }
    let reg = read_usize(this.tie_tables() + round as usize * 4) + (game as usize).wrapping_mul(0x35);
    let a = read_i16(reg);
    if a == -1 {
        collect_fixture_teams(this, read_i16(reg + cup_tie::HOME_LINK_ROUND) as i32, read_i16(reg + cup_tie::HOME_LINK_TIE) as i32, out, count, caixa);
    } else {
        *out.offset(*count as isize) = a;
        *count += 1;
    }
    let reg = read_usize(this.tie_tables() + round as usize * 4) + (game as usize).wrapping_mul(0x35);
    let b = read_i16(reg + cup_tie::AWAY_TEAM);
    if b == -1 {
        collect_fixture_teams(this, read_i16(reg + cup_tie::AWAY_LINK_ROUND) as i32, read_i16(reg + cup_tie::AWAY_LINK_TIE) as i32, out, count, caixa);
        return *count;
    }
    *out.offset(*count as isize) = b;
    *count += 1;
    *count
}

/// Appends to the output the clubs that can still play the fixture, walking down the fixtures the empty sides come
/// from.
///
/// `rodada.collect_possible_teams(jogo, saida, *conta)` (round, fixture, output, *count).
///
/// Null output or count: the original routine runs (the error box).
/// Original: `Cup::collect_possible_teams` (0x5219b0, Cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5219b0 } (thiscall, ret 0x10, Cup.cpp): `rodada.collect_possible_teams(jogo, saida, *conta)`: acrescenta
/// a `saida` os clubes que ainda podem jogar esse jogo, descendo pelos jogos de onde vêm os lados vazios. Saída ou
/// conta nula: a original (a caixa).
pub(crate) unsafe extern "thiscall" fn collect_possible_teams(this: usize, round: i32, game: i32, out: *mut i16, count: *mut i32) -> i32 {
    if out.is_null() || count.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(IDX_COLLECT_POSSIBLE_TEAMS)(this, round, game, out, count);
    }
    collect_fixture_teams(Cup(this), round, game, out, count, true)
}

/// Returns the club on one side of the fixture or, if that side is empty, the clubs that can still reach it.
///
/// `rodada.get_fixture_side_teams(jogo, lado, saida)` (round, fixture, side, output): the club on one side of the
/// fixture (side ≠ 0: `+0`; 0: `+2`) or, if empty, the clubs that can reach it (`0x5219b0` with the count in global
/// `0xae1880`). Returns the count (the error box if none).
///
/// Round outside 0..`+0x3c`, a cup with no clubs (`+0xa7`) or fixture −1: the original routine runs (the error box and
/// 0).
/// Original: `Cup::get_fixture_side_teams` (0x521790, Cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x521790 } (thiscall, ret 0x10, Cup.cpp): `rodada.get_fixture_side_teams(jogo, lado, saida)`: o clube de um
/// lado do jogo (lado ≠ 0: +0; 0: +2) ou, vazio, os clubes que podem chegar a ele (0x5219b0 com a conta no global
/// 0xae1880). Devolve a conta (a caixa se nenhum). Rodada fora de 0..+0x3c, copa sem clubes (+0xa7) ou jogo −1: a
/// original (a caixa e 0).
pub(crate) unsafe extern "thiscall" fn get_fixture_side_teams(this: usize, round: i32, game: i32, side: u32, out: *mut i16) -> i32 {
    let this = Cup(this);
    if round < 0 || round > this.round_count() as i32 || this.team_table() == 0 || game == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(IDX_GET_FIXTURE_SIDE_TEAMS)(this.addr(), round, game, side, out);
    }
    let count = G_CUP_POSSIBLE_TEAM_COUNT as *mut i32;
    *count = 0;
    let reg = read_usize(this.tie_tables() + round as usize * 4) + (game as usize).wrapping_mul(0x35);
    let (c, r, j) = if side as u8 != 0 { (read_i16(reg), reg + cup_tie::HOME_LINK_ROUND, reg + cup_tie::HOME_LINK_TIE) } else { (read_i16(reg + cup_tie::AWAY_TEAM), reg + cup_tie::AWAY_LINK_ROUND, reg + cup_tie::AWAY_LINK_TIE) };
    if c != -1 {
        *out = c;
        *count = 1;
        return 1;
    }
    exe::cup_collect_possible_teams(this, read_i16(r) as i32, read_i16(j) as i32, out, count);
    if *count == 0 {
        show_error_at(0x9b55ac, 0x157f);
    }
    *count
}

/// Builds a date from day and year with Date::Date (0x549ef0).
///
/// [pt] Uma data montada pela 0x549ef0 (dia, ano: ela lê as palavras; os dwords vão como os da original).
pub(crate) unsafe fn make_date(day: u32, year: u32) -> [u8; 8] {
    let mut d = [0u8; 8];
    exe::new_date((&mut d) as *mut [u8; 8] as usize, day, year);
    d
}

/// (day, year) words of an 8-byte date.
///
/// [pt] (dia, ano) de uma data.
pub(crate) fn cup_date_day_year(d: &[u8; 8]) -> (u16, u16) {
    (u16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]))
}

/// Address of cup round k (0x68 bytes each at +0xa3), signed index.
///
/// [pt] A rodada `k` da copa (0x68 bytes cada, em +0xa3).
pub(crate) unsafe fn round_record_at(this: Cup, k: isize) -> usize {
    (this.round_table() as isize).wrapping_add(k.wrapping_mul(0x68)) as usize
}

/// Date of cup round k (day +0, year byte +2 relative to cup +0x40), built like the original's blocks in `cup_update_for_date`.
///
/// [pt] A data da rodada `k` (+0 o dia, +2 o ano a partir do ano-base +0x40 da copa, byte com sinal), como nos blocos da
/// 0x51f890 em que a metade alta do dword do dia é a do ponteiro da rodada (a do ano é zero).
pub(crate) unsafe fn round_date(this: Cup, k: isize) -> [u8; 8] {
    let r = round_record_at(this, k);
    let year = (read_u8(r + 2) as i8 as i16).wrapping_add((this.year() as u16) as i16) as u16 as u32;
    make_date((r as u32 & 0xffff_0000) | read_u16(r) as u32, year)
}

/// Calls the thiscall virtual at offset off of object p with no arguments.
///
/// [pt] A virtual `off` do objeto `p` (thiscall, sem argumentos).
pub(crate) unsafe fn cup_call_virtual_no_args(p: Cup, off: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(p.addr()) + off));
    f(p.addr())
}

/// Stage i of the cup ([+0xc][i]).
///
/// [pt] A fase `i` da copa ([+0xc][i]).
pub(crate) unsafe fn cup_stage(this: Cup, i: usize) -> usize {
    read_usize(this.stages() + i * 4)
}

/// The daily update of each cup: moves on completed stages and ends the cup, gives each stage the day and, on the date,
/// draws (phase 0) or sets up (phase ≠ 0) the rounds until none advances. Returns 0.
///
/// `hoje.update_for_date(fase)` (today, phase).
///
/// A cup with no parent (`+0x44` = `0xff`): with `+0xb1`, if stages 0..=`+0x30` are complete (virtual `+0x10` with 0,
/// 1), it moves to the next stage (virtual `+0x28`, if `+0x30` < `+0x2c` − 1) or ends the cup once (`+0x4c`: the
/// history `0x442e80` of `[0xad9da8]`, virtual `+0x30`, the records `0x4a19f0` of Comp_Stats `+8`); then the records
/// exe::comp_stats_send_record_news(`0x4a2450`). Still with no parent, each stage receives the day (virtual `+0x14` with `0xae2c90` and the
/// phase).
///
/// Phase 0 (with `[0xae2c88]` = 1): the last, final round (`+7` = `0x8c`) is drawn on its date (`0x51c410`); without
/// `+0xb1`, it draws the rounds from `+0xac` whose date has arrived, until none advances or they run out. Phase ≠ 0
/// (with `+0x45`): the same with the round set-up (`0x51c800`) from `+0xab`, which also sets up on the round's pending
/// date (`+0x34` of the round's stage).
/// Original: `Cup::update_for_date` (0x51f890, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51f890 } (thiscall, ret 8, Cup.cpp; a atualização diária de cada copa): `hoje.update_for_date(fase)`. Copa
/// sem pai (+0x44 = 0xff): com +0xb1, se as fases 0..=+0x30 estão completas (virtual +0x10 com 0, 1), passa à fase
/// seguinte (virtual +0x28, se +0x30 < +0x2c − 1) ou encerra uma vez (+0x4c: o histórico 0x442e80 de [0xad9da8], a
/// virtual +0x30, os recordes 0x4a19f0 do Comp_Stats +8); depois as notícias dos exe::comp_stats_update_season_end_records(0x4a2450). Ainda sem pai,
/// cada fase recebe o dia (virtual +0x14 com 0xae2c90 e a fase). Fase 0 (com [0xae2c88] = 1): a última rodada final
/// (+7 = 0x8c) sorteia na data dela (0x51c410); sem +0xb1, sorteia as rodadas a partir de +0xac cuja data chegou, até
/// não avançar ou acabar. Fase ≠ 0 (com +0x45): o mesmo com a montagem das rodadas (0x51c800) a partir de +0xab, que
/// também monta na data pendente da rodada (+0x34 da fase da rodada). Devolve 0.
pub(crate) unsafe extern "thiscall" fn cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    if this.stage_index() == 0xff {
        let mut completas = this.run_by_stages() != 0;
        if completas {
            let mut i = 0usize;
            while (i as i32) <= this.current_stage() as i32 {
                let p = cup_stage(this, i);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + 0x10));
                if completa(p, 0, 1) == 0 {
                    completas = false;
                    break;
                }
                i += 1;
            }
        }
        if completas {
            if (this.current_stage() as i32) < (this.stage_count() as i32).wrapping_sub(1) {
                cup_call_virtual_no_args(this, 0x28); // a fase seguinte
            } else if this.season_closed() == 0 {
                exe::club_history_record_comp_season(read_usize(G_CLUB_HISTORY), this);
                cup_call_virtual_no_args(this, 0x30);
                if read_usize(this.addr() + 8) != 0 {
                    exe::comp_stats_update_season_end_records(read_usize(this.addr() + 8));
                }
                this.set_season_closed((1) as usize);
            }
        }
        if read_usize(this.addr() + 8) != 0 {
            exe::comp_stats_send_record_news(read_usize(this.addr() + 8));
        }
    }
    if this.stage_index() == 0xff {
        let mut i = 0usize;
        while (i as i32) <= this.current_stage() as i32 {
            let p = cup_stage(this, i);
            let day: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(read_usize(read_usize(p) + 0x14));
            day(p, G_CURRENT_DATE, stage);
            i += 1;
        }
    }
    let (hdia, hano) = (read_u16(today), read_u16(today + 2));
    let n = (this.round_count() as u16) as i16;
    if stage == 0 {
        if read_u8(G_CURRENT_DAY_SLICE) != 1 {
            return 0;
        }
        if n > 0 && read_u16(round_record_at(this, n as isize - 1) + 7) == 0x8c {
            let r = round_record_at(this, n as isize - 1);
            let year = (read_u8(r + 2) as i8 as i16).wrapping_add((this.year() as u16) as i16) as u16 as u32;
            let d = make_date(read_u16(r) as u32, year);
            if cup_date_day_year(&d) == (hdia, hano) {
                exe::cup_draw_round(this, (n as i32 - 1) as u32); // the final on its date
            }
        }
        if this.run_by_stages() != 0 {
            return 0;
        }
        let mut k = this.next_draw_round() as i8;
        if k as i16 == (this.round_count() as u16) as i16 {
            return 0;
        }
        loop {
            let (day, year) = cup_date_day_year(&round_date(this, k as isize));
            // the year unsigned, the day signed
            if year > hano || (year == hano && day as i16 > hdia as i16) { // cps13: the draw date
                return 0;
            }
            crate::comparison::branch("cps_13 round draw");
            exe::cup_draw_round(this, this.next_draw_round() as i8 as i32 as u32);
            let new = this.next_draw_round() as i8;
            if new == k || new as i16 == (this.round_count() as u16) as i16 {
                return 0;
            }
            k = new;
        }
    }
    if this.season_set_up() == 0 {
        return 0;
    }
    if n > 0 && read_u16(round_record_at(this, n as isize - 1) + 7) == 0x8c {
        let r = round_record_at(this, n as isize - 1);
        let year = (read_u8(r + 2) as i8 as i16).wrapping_add((this.year() as u16) as i16) as u16 as u32;
        let d = make_date(read_u16(r) as u32, year);
        if cup_date_day_year(&d) == (hdia, hano) {
            exe::cup_set_up_round(this, (n as i32 - 1) as u32); // the final on its date
        }
    }
    if this.run_by_stages() != 0 || this.season_set_up() == 0 {
        return 0;
    }
    let mut k = this.current_round() as i8;
    if k as i16 == (this.round_count() as u16) as i16 {
        return 0;
    }
    loop {
        let (day, year) = cup_date_day_year(&round_date(this, k as isize));
        if (year, day) != (hano, hdia) { // cps13: the setup date
            let current = this.current_round() as i8 as isize;
            let p = read_usize((this.tie_tables() as isize + current * 4) as usize);
            if read_u8(p + cup_tie::FIXTURE_STATE) == 0 {
                return 0; // nem a data nem pendente
            }
        }
        crate::comparison::branch("cps_13 round setup");
        exe::cup_set_up_round(this, this.current_round() as i8 as i32 as u32);
        let new = this.current_round() as i8;
        if new == k || new as i16 == (this.round_count() as u16) as i16 {
            return 0;
        }
        k = new;
    }
}

/// Sets up a cup round: draws the round if needed, creates the fixtures of the marked ties, moves them to the round
/// dates and moves on; a team without a club postpones the rounds by a day.
///
/// `rodada.set_up_round()`: without `+0x45`: 1. Round outside 0..`+0x3c` − 1: the error box (line `0xa64`) and −1.
/// It draws the round if its stage has no `+0x34` (`0x51c410`).
///
/// In the current round (`+0xab`): if a tie (`0x35` bytes in the stage, `+0/+2` the teams or `0xffff`) has a team
/// without a club (`+0xa7`, 6 bytes), it postpones the requested round and the following ones by a day (`0x54c5b0`,
/// `0x417c10`; the following ones only if they ended up earlier) and returns 1.
///
/// Otherwise it creates the fixtures of each marked tie (`+0x34` = 1 deferred, 2 already: `0x51cb90`; after the 2,
/// without the cup in `[0xadadfc][id]`: 1), moves the fixtures to the round dates (date `+3/+5` in an 8-byte block,
/// `0x51ebe0`, freed afterwards) and, with the cup in `[0xadadfc][id]`, moves on to the next round (except the final
/// `0x8c`) and takes `+0xac` along. Returns 1.
/// Original: `Cup::set_up_round` (0x51c800, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51c800 } (thiscall, ret 4, Cup.cpp): `rodada.set_up_round()`: sem +0x45: 1. Rodada fora de 0..+0x3c − 1: a
/// caixa (linha 0xa64) e −1. Sorteia a rodada se a fase dela não tem +0x34 (0x51c410). Na rodada atual (+0xab): se um
/// confronto (0x35 bytes na fase, +0/+2 os times ou 0xffff) tem time sem clube (+0xa7, 6 bytes), adia a rodada pedida e
/// as seguintes um dia (0x54c5b0, 0x417c10; as seguintes só se ficaram antes) e devolve 1. Senão cria os jogos de cada
/// confronto marcado (+0x34 = 1 adiado, 2 já: 0x51cb90; depois do 2, sem a copa em [0xadadfc][id]: 1), leva os jogos às
/// datas da rodada (data +3/+5 num bloco de 8 bytes, 0x51ebe0, liberado depois) e, com a copa em [0xadadfc][id], passa à
/// rodada seguinte (fora da final 0x8c) e leva +0xac junto. Devolve 1.
pub(crate) unsafe extern "thiscall" fn set_up_round(this: usize, round: u32) -> u32 {
    let this = Cup(this);
    let id = read_usize(read_usize(this.addr() + 4));
    let active = || read_usize(read_usize(G_COMP_OBJECT_TABLE) + id * 4) != 0;
    if this.season_set_up() == 0 {
        return 1;
    }
    let ri = round as i32;
    if ri > (this.round_count() as u16) as i16 as i32 - 1 || ri < 0 {
        show_error_at(0x9b55ac, 0xa64);
        return u32::MAX;
    }
    let ad = this.tie_tables();
    if read_u8(read_usize(ad + round as usize * 4) + 0x34) == 0 {
        exe::cup_draw_round(this, round);
    }
    let current = this.current_round() as i8 as isize;
    let base = this.round_table();
    let off = current * 0x68;
    let n = *((base as isize + off + 0x1a) as *const i16) as i32;
    let mut without_club = false;
    for i in 0..n.max(0) as usize {
        let t = read_usize((this.tie_tables() as isize + current * 4) as usize) + i * 0x35;
        for lado in [0usize, 2] {
            let team = read_u16(t + lado);
            if team != 0xffff && read_usize(this.team_table() + (team as i16 as isize * 6) as usize) == 0 {
                without_club = true;
                break;
            }
        }
        if without_club {
            break;
        }
    }
    if without_club {
        // the requested round (not the current one) and the following ones are moved one day later
        let r = round_record_at(this, ri as isize);
        let year = (read_u8(r + 2) as i8 as i16).wrapping_add((this.year() as u16) as i16) as u16 as u32;
        let mut d = make_date((this.team_table() as u32 & 0xffff_0000) | read_u16(r) as u32, year);
        let (mut amanha, mut copia) = ([0u8; 8], [0u8; 8]);
        let p = exe::date_add_days((&mut d) as *mut [u8; 8] as usize, (&mut amanha) as *mut [u8; 8] as usize, 1);
        exe::date_operator_3d((&mut d) as *mut [u8; 8] as usize, (&mut copia) as *mut [u8; 8] as usize, p);
        let (day, year) = cup_date_day_year(&d);
        crate::comparison::branch("cps_13 round postponed");
        let r = round_record_at(this, ri as isize);
        core::ptr::write_unaligned(r as *mut u16, day);
        *((r + 2) as *mut u8) = (year as u8).wrapping_sub((this.year() as u8));
        let mut k = ri + 1;
        while (k as i16) < (this.round_count() as u16) as i16 {
            let r = round_record_at(this, k as i16 as isize);
            let rano = (read_u8(r + 2) as i8 as i16).wrapping_add((this.year() as u16) as i16) as u16 as u32;
            let (odia, oano) = cup_date_day_year(&make_date((this.round_table() as u32 & 0xffff_0000) | read_u16(r) as u32, rano));
            if year > oano || (year == oano && day as i16 > odia as i16) {
                let r = round_record_at(this, k as i16 as isize);
                core::ptr::write_unaligned(r as *mut u16, day);
                *((r + 2) as *mut u8) = (year as u8).wrapping_sub((this.year() as u8));
            }
            k += 1;
        }
        return 1;
    }
    let mut i = 0i32;
    while n > 0 && i < *((this.round_table() as isize + off + 0x1a) as *const i16) as i32 {
        let mark = |i: i32| read_u8(read_usize((this.tie_tables() as isize + current * 4) as usize) + i as usize * 0x35 + 0x34);
        if mark(i) == 1 {
            exe::cup_create_tie_fixtures(this, current as i32 as u32, i as u32, 1);
        }
        if mark(i) == 2 {
            exe::cup_create_tie_fixtures(this, current as i32 as u32, i as u32, 0);
            if !active() {
                return 1;
            }
        }
        i += 1;
    }
    let p = exe::crt_operator_new(8 as u32);
    let data = if p != 0 {
        let base = this.round_table();
        let r = (base as isize + off) as usize;
        let year = (read_u8(r + 5) as i8 as i16).wrapping_add((this.year() as u16) as i16) as u16 as u32;
        let day = read_u16(r + 3) as u32;
        exe::new_date(p, (r as u32 & 0xffff_0000) | day, (base as u32 & 0xffff_0000) | year)
    } else {
        0
    };
    crate::comparison::branch("cps_13 round matches");
    exe::cup_move_round_fixtures_to_set_dates(this, data, current as i32 as u32);
    if data != 0 {
        exe::crt_operator_delete(data);
    }
    if !active() {
        return 1;
    }
    if read_u16((this.round_table() as isize + off + 7) as usize) != 0x8c {
        this.set_current_round(this.current_round().wrapping_add(1));
    }
    if (this.next_draw_round() as i8) < this.current_round() as i8 {
        this.set_next_draw_round(this.current_round());
    }
    1
}

/// Adds an illness outbreak risk on the eve of the date of fixture `f` for club `id`, in the stadium's nation.
///
/// The risk is `0x62e820` of [`0xb63d28`] (Injury), type 3; the date comes from `+0x2a` (day) and `+0x28` (year) via
/// `0x549ef0`, `0x54c8e0`, `0x417c10`.
///
/// `alto_dia`/`alto_ano`: the high halves of the dwords that the original routine pushes to `0x549ef0`.
///
/// [pt] Põe o risco de doença (0x62e820 de [0xb63d28], tipo 3) na véspera da data do jogo `f` (+0x2a o dia, +0x28 o ano;
/// 0x549ef0, 0x54c8e0, 0x417c10) para o clube `id` com a nação do estádio. `alto_dia`/`alto_ano`: as metades altas dos
/// dwords que a original empurra à 0x549ef0.
pub(crate) unsafe fn add_illness_risk_before_match(f: Fixture, id: usize, nation: usize, high_day: u32, high_year: u32) {
    let mut d = make_date((high_day & 0xffff_0000) | f.day() as u32, (high_year & 0xffff_0000) | f.year() as u32);
    let (mut vespera, mut copia) = ([0u8; 8], [0u8; 8]);
    let p = exe::date_sub_days((&mut d) as *mut [u8; 8] as usize, (&mut vespera) as *mut [u8; 8] as usize, 1);
    exe::date_operator_3d((&mut d) as *mut [u8; 8] as usize, (&mut copia) as *mut [u8; 8] as usize, p);
    exe::injury_add_illness_outbreak(read_usize(G_INJURY), &mut d, 3, id, nation);
}

// simplification: draws and builds rounds (allocates matches): does not run twice. Original only; the copy carries the branches (draw and
// setup; the update is daily in each cup, 29,934 calls up to 2002-06-01: no branch on the common path).

// simplification: draws, creates the matches and advances the round: it does not run twice. Original only; the copy takes the branches.

// simplification: adds the illness risk (does not run twice; the common path does nothing). Original only; the copy takes
// the branches.

/// Reads a u32 at a as usize.
pub(crate) unsafe fn read_ptr(a: usize) -> usize {
    read_u32(a) as usize
}

/// base + i × size with signed index (like the original's movsx).
///
/// [pt] `base + i × tam` (o índice com sinal, como o movsx do original).
pub(crate) fn index_addr(base: usize, i: i32, size: usize) -> usize {
    base.wrapping_add((i as isize).wrapping_mul(size as isize) as usize)
}

/// Address of tie k (0x35 bytes) of round r: [[cup + 0xad] + 4r] + 0x35k.
///
/// [pt] O confronto `k` (0x35 bytes) da rodada `r` da copa: [[copa + 0xad] + 4r] + 0x35k.
pub(crate) unsafe fn tie_of_round(cup: Cup, r: i32, k: i32) -> usize {
    index_addr(read_ptr(index_addr(read_ptr(cup.addr() + cup::TIE_TABLES), r, 4)), k, 0x35)
}

/// Address of round r's 0x68-byte entry in [cup + 0xa3] (+0x18 team slots, +0x1a ties).
///
/// [pt] A entrada da rodada `r` (0x68 bytes) em [copa + 0xa3]: +0x18 vagas de times, +0x1a confrontos.
pub(crate) unsafe fn round_entry(cup: Cup, r: i32) -> usize {
    index_addr(read_ptr(cup.addr() + cup::ROUND_TABLE), r, 0x68)
}

/// Address of cup team t's 6-byte entry in [cup + 0xa7] (+0 club, +4 level).
///
/// [pt] O clube do time `t` da copa (6 bytes em [copa + 0xa7]: +0 clube, +4 nível).
pub(crate) unsafe fn cup_team_entry(cup: Cup, t: i16) -> usize {
    index_addr(read_ptr(cup.addr() + cup::TEAM_TABLE), t as i32, 6)
}

/// Memory regions of all rounds' tie arrays (plus 0xb67a34), for the comparers.
///
/// [pt] Os confrontos de todas as rodadas da copa (rodadas em +0x3c), para os comparadores.
pub(crate) unsafe fn all_ties_regions(cup: Cup) -> Vec<(usize, usize)> {
    let mut v = vec![(G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    for r in 0..(cup.round_count() as i32).max(0) {
        let f = read_ptr(index_addr(read_ptr(cup.addr() + cup::TIE_TABLES), r, 4));
        let n = read_i16(round_entry(cup, r) + 0x1a) as i32;
        if f != 0 && n > 0 {
            v.push((f, n as usize * 0x35));
        }
    }
    v
}

/// Shows Cup.cpp's "Memory Allocation Error" box for a line and text, then exits the game like the original.
///
/// [pt] A caixa "Memory Allocation Error" ("v%s %s.%s %d (%s)" de Cup.cpp na linha `linha`) e o jogo para (como a original).
pub(crate) unsafe fn fatal_memory_error_box(row: u32, text: usize) -> ! {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x9b55ac, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// The open draw of the round's teams: takes the teams out of the round ties, shuffles them and puts each one in a
/// drawn tie (empty ones first), drawing the side when both slots are free, and sets the link to the next round.
///
/// `rodada.randomize_round_teams()`.
///
/// 1) It takes the teams out of the ties (`+0x1a` of the round) into an array of 7 bytes per slot (malloc of 7 × slots
/// `+0x18`; out of memory, box of line `0xe7f` and the game stops): the home team (`+0`, with the link `+0xe/+0xa`) and
/// the away team (`+2`, with `+0x10/+0xc`), each with the team level (`+4` of the team); the tie slot becomes −2
/// (free); with no team (−1), the entry stays −1 (with `0xff` in the level).
///
/// 2) Shuffles: for each position, swaps with the entry drawn among all of them (`0x90cfc0(n)`).
///
/// 3) Each entry with a team goes to a drawn tie (`0x90cfc0` over the ties), moving round in a circle: on the first
/// pass only ties with both slots empty (< 0); then any with a free slot (−2); after going all the way round on the
/// second pass, box of line `0xf4c` and it exits (the second-pass flag is never reset to zero). With both slots free,
/// it draws the side (`0x90cfc0(2)`: ≠ 0 home). The team carries its link (`+0xe/+0xa` or `+0x10/+0xc`) and, with a
/// link, the tie of the next round receives the round (`+0x12`), the drawn tie (`+6`) and the side (`+8`: 1 home, 0
/// away).
///
/// Frees the array (`0x9452ca`).
/// Original: `Cup::randomize_round_teams` (0x51e220, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51e220 } (thiscall, ret 4, Cup.cpp): `rodada.randomize_round_teams()`: o sorteio dos times da rodada.
/// 1) Tira os times dos confrontos (+0x1a da rodada) para um vetor de 7 bytes por vaga (malloc de 7 × vagas +0x18;
///    sem memória, caixa da linha 0xe7f e o jogo para): o mandante (+0, com o vínculo +0xe/+0xa) e o visitante (+2,
///    com +0x10/+0xc), cada um com o nível do time (+4 do time); a vaga do confronto vira −2 (livre); sem time (−1),
///    a entrada fica −1 (com 0xff no nível).
/// 2) Embaralha: para cada posição, troca com a entrada sorteada entre todas (0x90cfc0(n)).
/// 3) Cada entrada com time vai a um confronto sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira
///    volta só confrontos com as duas vagas vazias (< 0); depois qualquer um com vaga livre (−2); dando a volta
///    inteira na segunda, caixa da linha 0xf4c e sai (a marca da segunda volta não volta a zero). Com as duas vagas
///    livres, sorteia o lado (0x90cfc0(2): ≠ 0 mandante). O time leva o vínculo (+0xe/+0xa ou +0x10/+0xc) e, com
///    vínculo, o confronto da rodada seguinte ganha a rodada (+0x12), o confronto sorteado (+6) e o lado (+8: 1
///    mandante, 0 visitante).
/// Libera o vetor (0x9452ca).
pub(crate) unsafe extern "thiscall" fn randomize_round_teams(cup: usize, round: u32) {
    let r = round as i16 as i32;
    let slots = read_i16(round_entry(Cup(cup), r) + 0x18) as i32;
    let array = exe::crt_malloc(slots.wrapping_mul(7) as u32);
    if array == 0 {
        fatal_memory_error_box(0xe7f, 0x9b574c); // "CUP::randomize_round_teams() : Cannot allocate memory"
    }
    let entry = |i: i16| index_addr(array, i as i32, 7);
    // 1) the teams come out of the fixtures
    let mut n: i16 = 0;
    let mut k: i32 = 0;
    while (k as i16) < read_i16(round_entry(Cup(cup), r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = tie_of_round(Cup(cup), r, k as i16 as i32);
            let e = entry(n);
            n = n.wrapping_add(1);
            let team = read_i16(t + lado);
            if team != -1 {
                write_u16(e, team as u16);
                write_u16(e + 2, read_i16(t + ligado) as u16);
                write_u16(e + 4, read_i16(t + jogo) as u16);
                write_u8(e + 6, read_u8(cup_team_entry(Cup(cup), team) + 4)); // the team’s level
                write_u16(t + lado, 0xfffe); // the place stays free
            } else {
                write_u16(e, 0xffff);
                write_u8(e + 6, 0xff);
            }
        }
        k += 1;
    }
    // 2) o embaralhamento
    if n > 0 {
        let mut p = array;
        for _ in 0..n as i32 {
            let q = entry(exe::utils_random(n as i32) as i16);
            let (a, b, c) = (read_u32(q), read_i16(q + 4), read_u8(q + 6));
            write_u32(q, read_u32(p));
            write_u16(q + 4, read_i16(p + 4) as u16);
            write_u8(q + 6, read_u8(p + 6));
            write_u32(p, a);
            write_u16(p + 4, b as u16);
            write_u8(p + 6, c);
            p += 7;
        }
    }
    // 3) each team goes to a drawn tie
    let mut second_return = false;
    let mut i: i16 = 0;
    while i < n {
        let e = entry(i);
        i = i.wrapping_add(1);
        if read_i16(e) == -1 {
            continue;
        }
        let mut s = exe::utils_random(read_i16(round_entry(Cup(cup), r) + 0x1a) as i32) as i16;
        let start = s;
        let home = loop {
            let t = tie_of_round(Cup(cup), r, s as i32);
            let (casa, fora) = (read_i16(t), read_i16(t + 2));
            if second_return || (casa < 0 && fora < 0) {
                let home_free = casa == -2;
                if fora == -2 {
                    if home_free && exe::utils_random(2) != 0 {
                        crate::comparison::branch("cpq_11 home side draw");
                        break true;
                    }
                    break false;
                }
                if home_free {
                    break true;
                }
            }
            s = s.wrapping_add(1);
            if s >= read_i16(round_entry(Cup(cup), r) + 0x1a) {
                s = 0;
            }
            if s == start {
                if second_return {
                    show_error_at(0x9b55ac, 0xf4c);
                    exe::crt_free(array);
                    return;
                }
                second_return = true;
            }
        };
        let t = tie_of_round(Cup(cup), r, s as i32);
        let (lado, ligado, jogo) = if home { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
        write_u16(t + lado, read_i16(e) as u16);
        write_u16(t + ligado, read_i16(e + 2) as u16);
        write_u16(t + jogo, read_i16(e + 4) as u16);
        let next = read_i16(e + 2);
        if next != -1 {
            let v = tie_of_round(Cup(cup), next as i32, read_i16(e + 4) as i32);
            write_u16(v + cup_tie::FEEDER_ROUND, round as u16);
            write_u16(v + cup_tie::FEEDER_TIE, s as u16);
            write_u16(v + cup_tie::FEEDER_SIDE, home as u16);
        }
    }
    exe::crt_free(array);
}

/// Swaps the word pairs `separate_shared_stadium_ties` swaps in tie t: home/away (+0/+2), +0xe/+0x10 and +0xa/+0xc.
///
/// [pt] Os pares de palavras do confronto `t` que a 0x522610 troca: mandante/visitante (+0/+2), +0xa/+0xc e +0xe/+0x10.
pub(crate) unsafe fn swap_tie_sides(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xe, 0x10), (0xa, 0xc)] {
        let (x, y) = (read_i16(t + a), read_i16(t + b));
        write_u16(t + a, y as u16);
        write_u16(t + b, x as u16);
    }
}

/// Swaps home and away in the later tie whose team shares a stadium with a team of an earlier tie of the round, and
/// sets the side in the linked ties.
///
/// `rodada.separate_shared_stadium_ties()`: for each tie `i` of the round, it looks for the first `j > i` whose
/// home team (or, failing that, the away team) shares the stadium (`+0x69` of the club, not null) with the home (or the
/// away) team of `i`; if found, it swaps tie `j` (home and away, `+0xa/+0xc` and `+0xe/+0x10`) and fixes the side
/// (`+8`) of the linked ties in the next round: the one at `+0xe/+0xa` becomes 1 (home), the one at `+0x10/+0xc`
/// becomes 0. The two fixtures in the same stadium no longer fall on the same side.
/// Original: `Cup::separate_shared_stadium_ties` (0x522610, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x522610 } (thiscall, ret 4, Cup.cpp): `rodada.separate_shared_stadium_ties()`: para cada confronto `i` da
/// rodada, procura o primeiro `j > i` cujo mandante (ou, senão, o visitante) divide o estádio (+0x69 do clube, não
/// nulo) com o mandante (ou o visitante) de `i`; achando, inverte o confronto `j` (mandante e visitante, +0xa/+0xc e
/// +0xe/+0x10) e acerta o lado (+8) dos confrontos ligados na rodada seguinte: o de +0xe/+0xa fica 1 (mandante), o de
/// +0x10/+0xc fica 0. Os dois jogos no mesmo estádio deixam de cair no mesmo lado.
pub(crate) unsafe extern "thiscall" fn separate_shared_stadium_ties(cup: usize, round: u32) {
    let r = round as i16 as i32;
    let stadium = |t: i16| read_u32(read_ptr(cup_team_entry(Cup(cup), t)) as usize + 0x69);
    let mut n = read_i16(round_entry(Cup(cup), r) + 0x1a);
    let mut i: i32 = 0;
    while (i as i16) < n {
        let fi = tie_of_round(Cup(cup), r, i as i16 as i32);
        let home_i = read_i16(fi);
        let mut j = i + 1;
        while (j as i16) < n {
            let fj = tie_of_round(Cup(cup), r, j as i16 as i32);
            let mut dividem = false;
            for lado in [0usize, 2] {
                let a = if lado == 0 { home_i } else { read_i16(fi + 2) };
                let b = read_i16(fj + lado);
                if a != -1 && b != -1 {
                    crate::comparison::branch("cpq_11 stadium compared");
                    let (sa, signed_byte_at) = (stadium(a), stadium(b));
                    if sa == signed_byte_at && sa != 0 {
                        dividem = true;
                        break;
                    }
                }
            }
            if dividem {
                swap_tie_sides(fj);
                let next = read_i16(fj + cup_tie::HOME_LINK_ROUND);
                if next != -1 {
                    write_u16(tie_of_round(Cup(cup), next as i32, read_i16(fj + cup_tie::HOME_LINK_TIE) as i32) + 8, 1);
                }
                let next = read_i16(fj + cup_tie::AWAY_LINK_ROUND);
                if next != -1 {
                    write_u16(tie_of_round(Cup(cup), next as i32, read_i16(fj + cup_tie::AWAY_LINK_TIE) as i32) + 8, 0);
                }
                break;
            }
            j += 1;
        }
        n = read_i16(round_entry(Cup(cup), r) + 0x1a);
        i += 1;
    }
}

/// Side-effect-free `comp_util_scale_by_round`: value between low and high byte by round (midpoint with a single round); returns the
/// original eax and the byte.
///
/// [pt] A conta da 0x4bf850 sem efeitos: o eax da original e o byte devolvido. Com uma rodada só, (baixo + alto) / 2
/// (truncada para o zero) sobre a soma; senão o quociente (alto − baixo) × rodada / (rodadas − 1), sem somar o baixo.
pub(crate) fn interpolate_by_round(low: u32, high: u32, round: u32, rounds: u32) -> (u32, i8) {
    let (b, a) = (low as u8 as i8 as i32, high as u8 as i8 as i32);
    let n = rounds as u16 as i16 as i32;
    if n == 1 {
        let s = b.wrapping_add(a);
        let t = s.wrapping_sub(s >> 31); // cdq; sub eax, edx
        let v = (t >> 1) as i8;
        ((t as u32 & 0xffff_ff00) | v as u8 as u32, v)
    } else {
        let q = a.wrapping_sub(b).wrapping_mul(round as u16 as i16 as i32).wrapping_div(n - 1); // the average per round
        (q as u32, q as i8)
    }
}

/// Level read_i8(+4 + signed) of a cup entry.
///
/// [pt] O nível (+4, byte com sinal) e a reputação do clube (+0 → +0x80, palavra com sinal) de uma entrada de copa.
pub(crate) unsafe fn entry_level(e: usize) -> i32 {
    read_i8(e + 4) as i32
}

/// Reputation (+0x80) of the club (+0) of a cup entry.
pub(crate) unsafe fn entry_club_reputation(e: usize) -> i32 {
    read_i16(read_usize(e) + 0x80) as i32
}

/// qsort comparer of the cup entries by level (`+4`), highest first; on a tie, by club reputation (`+0x80`), lowest
/// first.
///
/// `cup_compare_entries_by_level_desc(a, b)`: the qsort of the cup entries (draws of the UEFA Cup, the Intertoto and
/// the cups of Japan and Scotland). Hot (called by qsort): no counter.
/// Original: `cup_compare_entries_by_level_desc` (0x522190, Cup.cpp, cdecl).
///
/// [pt] { 0x522190 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_desc(a, b)`: o qsort das entradas das copas (sorteios da
/// Copa da UEFA, Intertoto, copas do Japão e da Escócia) pelo nível, o maior primeiro; no empate pela reputação do
/// clube, a menor primeiro. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = entry_level(b) - entry_level(a); // the highest level first
    if r != 0 {
        return r;
    }
    entry_club_reputation(a) - entry_club_reputation(b)
}

/// qsort comparator of cup entries by level (+4), lowest first; on a tie by club reputation (+0x80), lowest first
/// Original: `cup_compare_entries_by_level_asc` (0x5221c0, Cup.cpp, cdecl).
///
/// [pt] { 0x5221c0 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_asc(a, b)`: o qsort das entradas da copa da liga
/// inglesa (0x56ccc0) pelo nível, o menor primeiro; no empate pela reputação do clube, a menor primeiro. Quente
/// (qsort): sem contador.
pub(crate) unsafe extern "C" fn cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = entry_level(a) - entry_level(b); // the lowest level first
    if r != 0 {
        return r;
    }
    entry_club_reputation(a) - entry_club_reputation(b)
}

/// Moves the club's professional status byte (+0x64) to valor: down if above (baixar ≠ 0), up if below otherwise.
///
/// [pt] Leva o byte +0x64 do clube (com sinal) até `valor`: para baixo (`baixar` ≠ 0) se está acima, para cima se está
/// abaixo.
pub(crate) unsafe fn cap_professional_status(club: Club, value: i8, baixar: u32) {
    let p = (club.addr() + club::PROFESSIONAL_STATUS) as *mut i8;
    let muda = if baixar != 0 { *p > value } else { *p < value };
    if muda {
        crate::comparison::branch("cpp_13 division ceiling");
        *p = value; // o teto novo
    }
}

/// Body of `decide_fixture_winner`: the winning side of a fixture (goals, extra time, penalties) as `Some(side)`, or `None` on a draw
/// that is also level on penalties under their rule.
///
/// The original routine shows the "Error" box with line 0x97e and goes on to walkover and points: the caller runs the
/// whole original routine, which redoes the only thing already written (+0x3d/+0x3e = 1 with bit 0x10, idempotent).
///
/// [pt] O corpo da 0x51c020: `Some(lado)`, ou `None` no empate também nos pênaltis com a regra deles (a original mostra a
/// caixa "Error" com a linha 0x97e e segue para W.O. e pontos): quem chama roda a original inteira, que refaz o único
/// que já foi gravado (+0x3d/+0x3e = 1 com o bit 0x10, idempotente).
pub(crate) unsafe fn fixture_winner_side(this: Cup, f: Fixture) -> Option<u32> {
    let fl = (f.tie_rules() as i16) as u16;
    if fl & 0x10 != 0 {
        f.set_home_result_mark(1);
        f.set_away_result_mark(1);
    }
    let (g1, g2) = (f.home_goals(), f.away_goals());
    if g1 > g2 {
        if f.home_result_mark() == 1 {
            f.set_home_result_mark(2);
        }
        if fl & 0x2000 != 0 {
            f.set_home_result_mark(f.home_result_mark().wrapping_sub(3));
        }
        return Some(1);
    }
    if g1 < g2 {
        if f.away_result_mark() == 1 {
            f.set_away_result_mark(2);
        }
        if fl & 0x2000 != 0 {
            f.set_away_result_mark(f.away_result_mark().wrapping_sub(3));
        }
        return Some(2);
    }
    if fl & 2 != 0 {
        let (a1, a2) = (f.home_goals_extra_time(), f.away_goals_extra_time());
        if a1 > a2 {
            crate::comparison::branch("cpm_1 match in extra time");
            let c = f.home_result_mark();
            if c == 1 {
                f.set_home_result_mark(2);
            } else if fl & 0x2000 != 0 {
                f.set_home_result_mark(c.wrapping_sub(3));
            } else {
                f.set_home_result_mark(3);
            }
            return Some(1); // the home team in extra time
        }
        if a1 < a2 {
            let c = f.away_result_mark();
            if c == 1 {
                f.set_away_result_mark(2);
            } else if fl & 0x2000 != 0 {
                f.set_away_result_mark(c.wrapping_sub(3));
            } else {
                f.set_away_result_mark(3);
            }
            return Some(2); // the away team in extra time
        }
    }
    if fl & 0x21 != 0 {
        let (p1, p2) = (f.home_penalties(), f.away_penalties());
        // the penalty code: 4, or 5 without bit 1 of the rule (not al; and al, 1; or al, 4)
        let penalties = ((!(fl as u8)) & 1 | 4) as i8;
        if p1 > p2 {
            let c = f.home_result_mark();
            if c == 1 {
                f.set_home_result_mark(2);
            } else if fl & 0x2000 != 0 {
                f.set_home_result_mark(c.wrapping_sub(3));
            } else {
                f.set_home_result_mark(penalties);
            }
            return Some(1);
        }
        if p1 < p2 {
            let c = f.away_result_mark();
            if c == 1 {
                f.set_away_result_mark(2);
            } else if fl & 0x2000 != 0 {
                f.set_away_result_mark(c.wrapping_sub(3));
            } else {
                f.set_away_result_mark(penalties);
            }
            return Some(2);
        }
        return None; // draw in the penalties: the original’s box
    }
    if fl & 0x800 != 0 {
        f.set_home_result_mark(8); // Walkover in favour of the home team
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        f.set_away_result_mark(8);
        return Some(2);
    }
    if fl & 0x2000 == 0 {
        return Some(0);
    }
    let a = f.home_result_mark().wrapping_sub(1);
    let c = f.away_result_mark().wrapping_sub(1);
    f.set_home_result_mark(a);
    f.set_away_result_mark(c);
    if a < c {
        return Some(1);
    }
    if a > c {
        return Some(2);
    }
    // tie on points: the lower seed (byte +4 of the 6-byte entry [this+0xa7], +0x3e entries)
    let n = this.team_count();
    if n <= 0 {
        return Some(2);
    }
    let (c1, c2) = (f.home_club(), f.away_club());
    let (mut s1, mut s2) = (0i8, 0i8);
    let t = this.team_table();
    for k in 0..n as usize {
        let e = t + 6 * k;
        if read_usize(e) == c1.addr() {
            s1 = read_i8(e + 4);
        }
        if read_usize(e) == c2.addr() {
            s2 = read_i8(e + 4);
        }
    }
    Some(if s1 < s2 { 1 } else { 2 })
}

/// Decides who goes through in a cup match (1 home, 2 away, 0 nobody) by the rule (`+0x38`), storing each side's code
/// (`+0x3d`/`+0x3e`).
///
/// `jogo.decide_fixture_winner()` (fixture): by the score (`+0x43`/`+0x44`), extra time (bit 2, `+0x47`/`+0x48`),
/// penalties (bits `0x21`, `+0x49`/`+0x4a`), the walkover (`0x800`/`0x1000`) and the points (`0x2000`: codes −1 and the
/// seeded team). The code of each side (`+0x3d`/`+0x3e`): 2 won, 3 in extra time, 4/5 on penalties, 8 walkover; −3 with
/// `0x2000`.
///
/// Null fixture or a draw on penalties: the original routine runs ("Error" box).
/// Original: `Cup::decide_fixture_winner` (0x51c020, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51c020 } (thiscall, ret 4, Cup.cpp): `jogo.decide_fixture_winner()`: quem passa num jogo de copa (1
/// mandante, 2 visitante, 0 ninguém) pela regra (+0x38): o placar (+0x43/+0x44), a prorrogação (bit 2, +0x47/+0x48), os
/// pênaltis (bits 0x21, +0x49/+0x4a), o W.O. (0x800/0x1000) e os pontos (0x2000: os códigos −1 e o cabeça de chave),
/// gravando o código de cada lado (+0x3d/+0x3e: 2 venceu, 3 na prorrogação, 4/5 nos pênaltis, 8 W.O.; −3 com 0x2000).
/// Jogo nulo ou empate nos pênaltis: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn decide_fixture_winner(this: usize, game: usize) -> u32 {
    if game != 0 {
        if let Some(r) = fixture_winner_side(Cup(this), Fixture(game)) {
            return r;
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DECIDE_FIXTURE_WINNER)(this, game)
}

/// Body of `decide_tie_winner`: the winning side of a tie over its legs, or None on an unresolved penalty draw.
///
/// [pt] O corpo da 0x51f2f0: `None` no empate nos pênaltis com a regra deles (a original mostra a caixa "Error" com a linha
/// 0x11d1 e devolve 4; nada foi gravado antes).
pub(crate) unsafe fn tie_winner_side(conf: usize, j: Fixture) -> Option<u32> {
    let t = |k: usize| read_i8(conf + k);
    let fl = (j.tie_rules() as i16) as u16;
    if t(0x33) as u8 != 1 {
        // first leg (or single match) of the tie: 3
        if fl & 0x10 != 0 {
            let lado1 = if t(0x14) != t(0x15) {
                t(0x14) > t(0x15)
            } else if t(0x18) != t(0x19) {
                t(0x18) > t(0x19)
            } else {
                t(0x1a) > t(0x1b)
            };
            write_i8(j.addr() + if lado1 { 0x3d } else { 0x3e }, 1);
        } else if fl & 0x2000 != 0 {
            let pairs = [(0x14, 0x15), (0x18, 0x19), (0x1a, 0x1b)];
            match pairs.iter().find(|p| t(p.0) != t(p.1)) {
                Some(&(a, b)) => {
                    let side = if t(a) > t(b) { 0x3d } else { 0x3e };
                    write_i8(j.addr() + side, read_i8(j.addr() + side).wrapping_sub(3));
                }
                None => {
                    j.set_home_result_mark(j.home_result_mark().wrapping_sub(1));
                    j.set_away_result_mark(j.away_result_mark().wrapping_sub(1));
                }
            }
        }
        return Some(3);
    }
    // the second leg: the aggregate (goals +0x14 and +0x1f against +0x1e and +0x15, in 16 bits)
    let tot1 = (t(0x14) as i16).wrapping_add(t(0x1f) as i16);
    let tot2 = (t(0x1e) as i16).wrapping_add(t(0x15) as i16);
    let volta2 = t(0x15) as i16;
    if fl & 0x10 != 0 {
        // points: who won each match (score, extra time, penalties); one each: 4
        let side_return = if t(0x14) != t(0x15) {
            t(0x14) > t(0x15)
        } else if t(0x18) != t(0x19) {
            t(0x18) > t(0x19)
        } else {
            t(0x1a) > t(0x1b)
        };
        let side_first_leg = if t(0x1e) != t(0x1f) {
            t(0x1e) > t(0x1f)
        } else if t(0x22) != t(0x23) {
            t(0x22) > t(0x23)
        } else {
            t(0x24) > t(0x25)
        };
        if side_return == side_first_leg {
            j.set_home_result_mark(1);
            j.set_away_result_mark(1);
            return Some(4);
        }
        if side_return {
            j.set_away_result_mark(2);
            return Some(2);
        }
        j.set_home_result_mark(2);
        return Some(1);
    }
    if fl & 0x2000 != 0 {
        let pairs = [(0x43, 0x44), (0x47, 0x48), (0x49, 0x4a)];
        match pairs.iter().find(|p| read_i8(j.addr() + p.0) != read_i8(j.addr() + p.1)) {
            Some(&(a, b)) => {
                let side = if read_i8(j.addr() + a) > read_i8(j.addr() + b) { 0x3d } else { 0x3e };
                write_i8(j.addr() + side, read_i8(j.addr() + side).wrapping_sub(3));
            }
            None => {
                j.set_home_result_mark(j.home_result_mark().wrapping_sub(1));
                j.set_away_result_mark(j.away_result_mark().wrapping_sub(1));
            }
        }
        if j.home_result_mark() < -5 {
            return Some(1);
        }
        return Some(if j.away_result_mark() >= -5 { 4 } else { 2 });
    }
    if tot1 > tot2 {
        return Some(2); // agregado
    }
    if tot1 < tot2 {
        return Some(1);
    }
    crate::comparison::branch("cpm_1 tie level on aggregate");
    if fl & 0x800 != 0 {
        j.set_home_result_mark(8);
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        j.set_away_result_mark(8);
        return Some(2);
    }
    if fl & 2 != 0 && t(0x22) as u8 != 0xff {
        if fl & 0x4000 != 0 && (t(0x22) as i32 + t(0x15) as i32) == (t(0x23) as i32 + t(0x14) as i32) {
            if t(0x23) > t(0x15) {
                j.set_away_result_mark(6);
                return Some(2);
            }
            if t(0x23) < t(0x15) {
                j.set_home_result_mark(6);
                return Some(1);
            }
        }
        let x = t(0x23) as i32 - t(0x1f) as i32;
        let y = t(0x22) as i32 - t(0x1e) as i32;
        if y > x {
            j.set_home_result_mark(3);
            return Some(1);
        }
        if y < x {
            j.set_away_result_mark(3);
            return Some(2);
        }
    }
    if fl & 4 == 0 && fl & 0x4000 == 0 {
        // away goals (the second-leg away team, +0x15, against the first-leg one, +0x1f)
        let away = t(0x1f) as i16;
        if volta2 < away {
            j.set_away_result_mark(6);
            return Some(2);
        }
        if volta2 > away {
            j.set_home_result_mark(6);
            return Some(1);
        }
    }
    if fl & 0x21 == 0 {
        return Some(4);
    }
    let (p1, p2) = (t(0x24), t(0x25));
    let code = if fl & 1 != 0 { 4 } else { 5 };
    if p2 < p1 {
        j.set_home_result_mark(code);
        return Some(1);
    }
    if p2 > p1 {
        j.set_away_result_mark(code);
        return Some(2);
    }
    None
}

/// Decides a two-legged tie: in the first leg it marks the winner and returns 3; in the second leg it decides by
/// aggregate, walkover, extra time, away goals and penalties (1 or 2 the side through, 4 undecided).
///
/// Convention: thiscall, ret 8, `Cup.cpp`; `this` is not read. `confronto.decide_tie_winner(jogo)` (tie,
/// fixture): in the first leg (`+0x33` ≠ 1), it only marks the winner (points `0x10`: code 1; `0x2000`: −3, or −1 for
/// both) and returns 3.
///
/// In the second leg, by the aggregate (`+0x14` + `+0x1f` against `+0x1e` + `+0x15`): 2 if the first side is ahead, 1
/// the second; level: walkover (`0x800`/`0x1000`, code 8), extra time (bit 2 with `+0x22` ≠ `0xff`: with `0x4000` the
/// away goals in it; code 6, otherwise the extra-time difference, code 3), away goals (without 4 or `0x4000`, code 6)
/// and penalties (`0x21`, `+0x24`/`+0x25`: code 4 with bit 1, otherwise 5); nothing decided: 4. Points (`0x10`) and
/// rule `0x2000` have their own branches.
///
/// A draw on penalties: the original routine runs ("Error" box, 4).
/// Original: `Cup::decide_tie_winner` (0x51f2f0, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51f2f0 } (thiscall, ret 8, Cup.cpp; `this` não é lido): `confronto.decide_tie_winner(jogo)`: no primeiro
/// jogo (+0x33 ≠ 1), só marca o vencedor (pontos 0x10: código 1; 0x2000: −3, ou −1 nos dois) e devolve 3. Na volta,
/// pelo agregado (+0x14 + +0x1f contra +0x1e + +0x15): 2 o primeiro lado à frente, 1 o segundo; empatado: W.O.
/// (0x800/0x1000, código 8), a prorrogação (bit 2 com +0x22 ≠ 0xff: com 0x4000 os gols fora dela; código 6, senão a
/// diferença da prorrogação, código 3), os gols fora (sem 4 nem 0x4000, código 6) e os pênaltis (0x21, +0x24/+0x25:
/// código 4 com o bit 1, senão 5); nada decidido: 4. Pontos (0x10) e regra 0x2000 têm os ramos próprios. Empate nos
/// pênaltis: a original (caixa "Error", 4).
pub(crate) unsafe extern "thiscall" fn decide_tie_winner(this: usize, tie: usize, game: usize) -> u32 {
    match tie_winner_side(tie, Fixture(game)) {
        Some(r) => r,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DECIDE_TIE_WINNER)(this, tie, game),
    }
}

/// the qsort comparison of the 7-byte draw entries: the higher seed (+6) first
/// Original: `cup_compare_draw_entries_by_seed` (0x51ebc0, Cup.cpp, cdecl).
///
/// [pt] { 0x51ebc0 } (cdecl, Cup.cpp): `cup_compare_draw_entries_by_seed(a, b)`: a comparação do qsort das entradas de 7
/// bytes do sorteio (0x51d2d0 e 0x51d9f0): o cabeça de chave (byte com sinal +6) de `b` menos o de `a` (maior primeiro).
pub(crate) unsafe extern "C" fn cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    crate::comparison::branch("cpm_1 seeding order");
    read_i8(b + 6) as i32 - read_i8(a + 6) as i32 // the largest first
}

/// Date from day/month/year moved to weekday semana (-1 any) via Date::Date `exe::date_date_549f70` → (day of year, year low byte).
///
/// [pt] A data (Date::Date, 0x549f70, num Date local de 8 bytes) de `dia`/`mes`/`ano`, levada ao dia da
/// semana `semana` (−1 = qualquer): (palavra +0 = dia do ano, byte baixo do ano +2).
pub(crate) unsafe fn date_on_weekday(day: u32, month: u32, year: u32, week: u32) -> (u16, u8) {
    let mut d = [0u32; 2];
    exe::date_date_549f70((&mut d) as *mut [u32; 2] as *mut [u16; 4], day, month, year, week);
    (d[0] as u16, (d[0] >> 16) as u8)
}

/// Address of the 104-byte record of round rodada (signed word) from rodadas.
///
/// [pt] O registro de 104 bytes da rodada `rodada` (palavra com sinal) a partir de `rodadas`.
pub(crate) fn round_record_from_base(rounds: usize, round: u32) -> usize {
    rounds.wrapping_add((round as i16 as isize * 104) as usize)
}

/// stores the cup round draw date: the day number and the year offset from the base year
/// Original: `cup_set_round_draw_date` (0x521e60, Cup.cpp, cdecl).
///
/// [pt] { 0x521e60 } (cdecl, Cup.cpp): a data do sorteio da rodada: dia do ano na palavra +0 e ano − ano-base
/// (byte) em +2, da data de `dia`/`mes`/ano-base + `anos` levada ao dia da semana. Devolve o registro.
pub(crate) unsafe extern "C" fn cup_set_round_draw_date(rounds: usize, round: u32, day: u32, month: u32, years: u32, week: u32, base: u32) -> usize {
    let (d, a) = date_on_weekday(day, month, base.wrapping_add(years), week);
    let r = round_record_from_base(rounds, round);
    write_u16(r, d);
    write_u8(r + 2, a.wrapping_sub(base as u8)); // year of the draw
    r
}

/// stores the cup round match date, its match time and the venue flags
/// Original: `cup_set_round_match_date` (0x521eb0, Cup.cpp, cdecl).
///
/// [pt] { 0x521eb0 } (cdecl, Cup.cpp): a data do jogo da rodada, como a do sorteio, em +3 (dia) e +5 (ano −
/// ano-base); o horário (byte) em +6 e as marcas do mando em +0x13. Devolve o registro.
pub(crate) unsafe extern "C" fn cup_set_round_match_date(rounds: usize, round: u32, day: u32, month: u32, years: u32, week: u32, hour: u32, base: u32, mando: u32) -> usize {
    let (d, a) = date_on_weekday(day, month, base.wrapping_add(years), week);
    let r = round_record_from_base(rounds, round);
    write_u16(r + cup_round::DAY, d); // match day
    write_u8(r + cup_round::YEAR_OFFSET, a.wrapping_sub(base as u8));
    write_u8(r + cup_round::TIME_SLICE, hour as u8);
    write_u32(r + cup_round::VENUE_VALUE, mando);
    r
}

/// Address of cup round r: 104-byte record in [cup + 0xa3].
///
/// [pt] A rodada `r` da copa: registro de 104 bytes em [copa+0xa3].
pub(crate) unsafe fn cup_cup_round(cup: Cup, r: i32) -> usize {
    (cup.round_table() as isize + r as isize * 104) as usize
}

/// Address of bracket row k of round r: [[cup + 0xad] + 4r] + 53k.
///
/// [pt] A linha da chave `k` da rodada `r` da copa: [[copa+0xad] + 4·r] + 53·k.
pub(crate) unsafe fn bracket_row(cup: Cup, r: i32, k: i32) -> usize {
    (*((cup.tie_tables() as isize + r as isize * 4) as *const usize) as isize + k as isize * 53) as usize
}

/// Address of the cup's 6-byte team slot v: [cup + 0xa7] + 6v.
///
/// [pt] O registro de 6 bytes da vaga `v` da copa: [copa+0xa7] + 6·v.
pub(crate) unsafe fn team_slot(cup: Cup, v: i16) -> usize {
    (cup.team_table() as isize + v as isize * 6) as usize
}

/// Copies the round rules onto the match and, with a decision on a draw, marks who goes through by both sides' levels.
///
/// It passes the rules of the match's round (word `+0xf` of round `[jogo+0x34]`) to the match flags (`+0x38`, OR). With
/// flag `0x800` (decides who goes through on a draw), the level (byte `+4` of the slot, signed) of the two sides of tie
/// `chave` decides: home team lower removes `0x800` and sets `0x1000`; higher keeps `0x800`; equal removes `0x800` and
/// sets 1.
/// Original: `Cup::apply_round_rules_to_match` (0x521d60, Cup.cpp, thiscall).
///
/// [pt] { 0x521d60 } (thiscall, ret 8, Cup.cpp): passa as regras da rodada do jogo (palavra +0xf da rodada
/// [jogo+0x34]) para as marcas do jogo (+0x38, OU). Com a marca 0x800 (decide quem passa no empate), o
/// nível (byte +4 da vaga, com sinal) dos dois lados da chave `chave` decide: mandante menor tira 0x800 e
/// põe 0x1000; maior mantém 0x800; igual tira 0x800 e põe 1.
pub(crate) unsafe extern "thiscall" fn apply_round_rules_to_match(cup: usize, game: usize, key: i32) {
    let game = Fixture(game);
    let r = game.round_number() as i32;
    let marks = (game.addr() + fixture::TIE_RULES) as *mut u16;
    let m = core::ptr::read_unaligned(marks) | read_u16(cup_cup_round(Cup(cup), r) + 0xf);
    core::ptr::write_unaligned(marks, m);
    if m & 0x800 == 0 {
        return;
    }
    let row = bracket_row(Cup(cup), r, key);
    let level = |o: usize| read_i8(team_slot(Cup(cup), read_i16(row + o)) + 4);
    let (a, b) = (level(0), level(2));
    let m = if a < b { (m ^ 0x800) | 0x1000 } else if a > b { m | 0x800 } else { (m ^ 0x800) | 1 };
    core::ptr::write_unaligned(marks, m);
}

/// Returns the tie of the match's round (`+0x34`) that holds the match's home club (`+0xc`), or -1 if none.
///
/// It looks at both sides of each tie (53 bytes; a negative slot is empty). The number of ties is word `+0x1a` of the
/// round. Null match: the original routine runs (error box).
/// Original: `Cup::find_tree_slot_for_fixture` (0x519c90, Cup.cpp, thiscall).
///
/// [pt] { 0x519c90 } (thiscall, ret 4, Cup.cpp): a chave da rodada do jogo (+0x34) em que está o clube
/// mandante do jogo (+0xc), olhando os dois lados de cada chave (53 bytes; vaga negativa é vazia); -1 se
/// nenhuma. O número de chaves é a palavra +0x1a da rodada. Jogo nulo: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn find_tree_slot_for_fixture(cup: usize, game: usize) -> i32 {
    let game = Fixture(game);
    if game.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_FIND_TREE_SLOT_FOR_FIXTURE)(cup, game.addr());
    }
    let r = game.round_number() as i32;
    let n = read_i16(cup_cup_round(Cup(cup), r) + 0x1a) as i32;
    let home = game.home_club_id();
    for i in 0..n {
        let row = bracket_row(Cup(cup), r, i);
        for o in [0, 2] {
            let v = read_i16(row + o);
            if v >= 0 && *(read_usize(team_slot(Cup(cup), v)) as *const usize) == home {
                return i;
            }
        }
    }
    -1
}

/// Returns the cup match importance by round and the first-leg score difference.
///
/// The importance is the match stage `+0x34` minus `+0x3c` plus the cup's `+0x51`, at least 1; in `margem` it puts the
/// first-leg score difference (`+0x4b`/`+0x4c`, `0xff` with no first leg) for side `segundo`, plus 1 (`0xff` below 1).
/// Returns only `al`.
/// Original: `cup_match_importance` (0x5221f0, Cup.cpp, thiscall).
///
/// [pt] { 0x5221f0 } (thiscall, ret 0xc, Cup.cpp): a importância da partida da copa (a fase +0x34 da
/// partida menos +0x3c mais +0x51 da copa, pelo menos 1) e em `margem` a diferença do placar da ida
/// (+0x4b/+0x4c, 0xff sem ida) para o lado `segundo`, mais 1 (0xff abaixo de 1). Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn cup_match_importance(cup: usize, fixture: usize, second: u8, margin: *mut u8) -> u8 {
    let cup = Cup(cup);
    let fixture = Fixture(fixture);
    let mut al = (fixture.round_number() as u8).wrapping_sub((cup.round_count() as u8)).wrapping_add(cup.importance_bias());
    if (al as i8) < 1 {
        al = 1;
    }
    let first_leg = (fixture.home_aggregate() as u8);
    if first_leg != 0xff {
        let volta = (fixture.away_aggregate() as u8);
        *margin = if second == 0 { volta.wrapping_sub(first_leg) } else { first_leg.wrapping_sub(volta) }.wrapping_add(1);
        if (*margin as i8) < 1 {
            *margin = 0xff;
        }
    }
    al
}

/// Outside friendlies, calls the competition's virtual `+0x44` and posts news item `0x10` about the team's result
/// (club, competition, outcome, result category, the match, the next round), delivered by scope, to the competition
/// region (final and third-place match) or to the club manager.
///
/// Convention: thiscall, ret `0x14`, `Cup.cpp`, SEH frame with no destructor. `clube.add_team_result_news(/// desfecho, a3, partida, a5)` (club, outcome, a3, match, a5): friendly (``exe::friendly_is_friendly_competition``), 0. The competition's virtual
/// `+0x44` (`[0xadadfc]` by the id at `+4`) with the club, the outcome, a3, the output (`0xff`), the match and a5; −1
/// from it or no match: 0.
///
/// News item `0x10` (local): 0 the club, 1 the competition, 2 the outcome, 3 the result category (`0x4b0890`), 4 what
/// the virtual returned, the match from data item 5 on (`0x4afeb0`, which advances the cursor) and at the cursor the
/// next round (`+7` of round `+0x34` + 1, up to `+0x3c` − 1; `0x8c` or outside: −1). Output ≠ `0xff`: delivery by scope
/// (`0x786090`). Outcome 1 in round `0x96` or 3 in `0x8c` (`+0x30`): to the competition region (`0x786860`); otherwise
/// to the club manager (`0x7861e0`). Returns 1.
/// Original: `Cup::add_team_result_news` (0x519690, Cup.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x519690 } (thiscall, ret 0x14, Cup.cpp, quadro SEH sem destrutor): `clube.add_team_result_news(desfecho, /// a3, partida, a5)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o clube, o
/// desfecho, a3, a saída (0xff), a partida e a5; −1 dela ou sem partida: 0. A notícia 0x10 (local): 0 o clube, 1 a
/// competição, 2 o desfecho, 3 a categoria do resultado (0x4b0890), 4 o que a virtual devolveu, o jogo a partir do
/// dado 5 (0x4afeb0, que avança o cursor) e no cursor a rodada seguinte (+7 da rodada +0x34 + 1, até +0x3c − 1; 0x8c
/// ou fora: −1). Saída ≠ 0xff: a entrega pelo alcance (0x786090). Desfecho 1 na rodada 0x96 ou 3 na 0x8c (+0x30): à
/// região da competição (0x786860); senão ao técnico do clube (0x7861e0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn add_team_result_news(cup: usize, club: usize, outcome: u32, a3: u32, p: usize, a5: u32) -> u32 {
    let cup = Cup(cup);
    let p = Fixture(p);
    let club = Club(club);
    // simplification: the original writes only the output byte into a stack dword and pushes the dword; 0x786090 reads the byte
    let mut out: u32 = 0xff;
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(cup.addr() + 4)) != 0 {
        return 0;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(read_usize(cup.addr() + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(comp) + 0x44));
    let r = virtual44(comp, club.addr(), outcome, a3, &mut out, p.addr(), a5);
    if r == -1 || p.is_null() {
        return 0;
    }
    crate::comparison::branch("cpx_13 outcome news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x10, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(read_usize(cup.addr() + 4)) as u32);
    exe::newsitem_set_slot(n, 2, outcome as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 3, exe::comp_util_classify_tie_result_for_news(club, p) as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 4, r as u32);
    // simplification: the cursor is a stack dword with 5 in the low byte (the rest is leftover); only the low byte counts
    let mut cursor = [5u8, 0, 0, 0];
    exe::comp_util_write_fixture_to_news_slots(p, n, cursor.as_mut_ptr());
    let k = p.round_number() as i32;
    let proxima = if k < cup.round_count() as i32 - 1 {
        let v = read_i16(cup.round_table().wrapping_add((k as usize).wrapping_mul(0x68)) + 0x6f);
        if v != 0x8c { v as i32 } else { -1 }
    } else {
        -1
    };
    exe::newsitem_set_slot(n, cursor[0] as i8, proxima as u32);
    let news = read_usize(G_NEWS);
    if out as u8 != 0xff {
        let nation = club.nation();
        let continent = if !nation.is_null() { nation.continent() } else { 0 };
        exe::news_send_by_scope(news, n, club, read_usize(cup.addr() + 4), nation, continent, out);
        return 1;
    }
    let round = p.round_name();
    if (outcome as u8 == 1 && round == 0x96) || (outcome as u8 == 3 && round == 0x8c) {
        exe::news_send_to_comp_region(news, n, read_usize(cup.addr() + 4));
    } else {
        exe::news_send_to_club_manager(news, n, club);
    }
    1
}

/// Tells whether the cup is complete: it has started (`+0xb1`) and, when asked for the stages with no stage set
/// (`+0x44`), all stages up to the current one are complete and the current one is the last.
///
/// `fases.check_complete(_)` (stages, _): 0 if the cup has not started (`+0xb1`). With `fases` = 1 and no stage
/// set (`+0x44` = `0xff`): each stage from 0 up to the current one (`+0x30`, array `+0xc`) must be complete (method
/// `+0x10` with 0, 1) and the current one must be the last (`+0x2c` − 1); otherwise 0. Apart from that, 1.
/// Original: `Cup::check_complete` (0x521e00, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x521e00 } (thiscall, ret 8, Cup.cpp): `fases.check_complete(_)`: 0 se a copa não começou (+0xb1). Com
/// `fases` = 1 e sem fase marcada (+0x44 = 0xff): cada fase de 0 até a atual (+0x30, vetor +0xc) tem de estar completa
/// (método +0x10 com 0, 1) e a atual tem de ser a última (+0x2c − 1); senão 0. Fora isso, 1.
pub(crate) unsafe extern "thiscall" fn cup_check_complete(this: usize, stages: u32, _x: u32) -> u8 {
    let this = Cup(this);
    if this.run_by_stages() == 0 {
        return 0;
    }
    if stages != 1 || this.stage_index() != 0xff {
        return 1;
    }
    let mut i = 0i32;
    while i <= this.current_stage() as i32 {
        let stage = read_usize(this.stages().wrapping_add(i as usize * 4));
        let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(stage) + 0x10));
        if completa(stage, 0, 1) == 0 {
            return 0; // an incomplete stage
        }
        i += 1;
    }
    (this.current_stage() as i32 >= (this.stage_count() as i32).wrapping_sub(1)) as u8
}

/// In rounds with set dates, moves each round fixture to the weekday and time slot of its match-day slot, with the
/// news, the stadium and the calendar; the used slot becomes empty.
///
/// Convention: thiscall, ret 8, `Cup.cpp`, SEH frame with no destructor on the normal path (the local news items, date
/// and list). `data.move_round_fixtures_to_set_dates(rodada)` (date, round), called from the round set-up
/// (`0x51c800`, in Rust). The local news item `0x14` is built right away (`0x77bbe0`).
///
/// A round (`0x68` bytes at `+0xa3`) with bit `0x100` in `+0xd` and the first slot set (`+0x29` ≠ `0xff`): the cup
/// fixtures on that date and time slot (collect_round_fixtures). With fixtures, the news carries the cup id, `+0x3a`,
/// the round code (`+7`) and whether the round is two-legged (`+0x21` = 2).
///
/// Each fixture of the league's type (`+0x42`) takes a slot (7 bytes from `+0x23`: `+0` stadium rules, `+4` weekday,
/// `+5` time slot, `+6` type; up to 8, up to the first `0xff`): on the round date, it goes to the slot's weekday
/// (`0x549ef0` in an 8-byte block from `0x944cf1`, `0x54a2d0`, freed afterwards), with news item 3 to the clubs outside
/// type 3, leaves the calendar (`0x5abc30`), takes the slot's date, time slot and type, the stadium (`0x85c260`) and
/// goes back in (`0x5aa680`); without the cup still in `[0xadadfc]`, it stops; otherwise the slot becomes `0xff`.
///
/// With or without the change, news item `0x14` goes to the clubs if one has a human manager. With no slot (end of the
/// slots or an empty slot): only the news, with the fixture data.
/// Original: `Cup::move_round_fixtures_to_set_dates` (0x51ebe0, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51ebe0 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: as notícias, a data e a lista
/// locais): `data.move_round_fixtures_to_set_dates(rodada)`, da montagem da rodada (0x51c800, em Rust). A notícia
/// 0x14 local é montada logo (0x77bbe0). Rodada (0x68 bytes em +0xa3) com o bit 0x100 em +0xd e a primeira vaga
/// marcada (+0x29 ≠ 0xff): os jogos da copa na data e fatia (collect_round_fixtures). Com jogos, a notícia leva o id da
/// copa, +0x3a, o código da rodada (+7) e a rodada de duas mãos (+0x21 = 2). Cada jogo do tipo da liga (+0x42) ocupa
/// uma vaga (7 bytes a partir de +0x23: +0 regras do estádio, +4 dia da semana, +5 fatia, +6 tipo; até 8, até a
/// primeira 0xff): na data da rodada, vai ao dia da semana da vaga (0x549ef0 num bloco de 8 bytes de 0x944cf1, 0x54a2d0,
/// liberado depois), com a notícia 3 aos clubes fora do tipo 3, sai do calendário (0x5abc30), ganha a data, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta (0x5aa680); sem a copa ainda em [0xadadfc], para; senão a vaga fica
/// 0xff. Com ou sem a mudança, a notícia 0x14 vai aos clubes se um é de técnico humano. Sem vaga (fim das vagas ou
/// vaga vazia): só a notícia, com os dados do jogo.
pub(crate) unsafe extern "thiscall" fn move_round_fixtures_to_set_dates(this: usize, data: usize, round: u32) {
    let this = Cup(this);
    let mut news = [0u32; 0x38];
    let n1 = news.as_mut_ptr() as usize;
    exe::new_newsitem(n1, 0x14, 0);
    let id = read_usize(read_usize(this.addr() + 4));
    let e = cup_round_entry(this, round);
    if read_i16(e + cup_round::TIE_RULES) as u16 & 0x100 == 0 || read_u8(e + cup_round::SLOT_CODE) == 0xff {
        return;
    }
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    if collect_round_fixtures(this, data, round, l) != 0 {
        exe::newsitem_set_slot(n1, 0, read_usize(read_usize(this.addr() + 4)) as u32);
        exe::newsitem_set_slot(n1, 1, this.stage_label() as i32 as u32);
        exe::newsitem_set_slot(n1, 2, read_i16(cup_round_entry(this, round) + 7) as i32 as u32);
        exe::newsitem_set_slot(n1, 5, (read_u8(cup_round_entry(this, round) + 0x21) == 2) as u32);
        // match news item 0x14 (data 3, 4, 6, 7, 8) to both clubs, if either has a human manager
        let news_of_game = |c1: usize, c2: usize, i1: usize, i2: usize, est: usize, day: u16, year: u16| {
            if cup_has_human_manager(c1) || cup_has_human_manager(c2) {
                if cup_has_human_manager(c1) {
                    exe::newsitem_set_slot(n1, 3, i1 as u32);
                    exe::newsitem_set_slot(n1, 4, i2 as u32);
                } else {
                    exe::newsitem_set_slot(n1, 3, i2 as u32);
                    exe::newsitem_set_slot(n1, 4, i1 as u32);
                }
                exe::newsitem_set_slot(n1, 6, est as u32);
                exe::newsitem_set_slot(n1, 7, day as u32);
                exe::newsitem_set_slot(n1, 8, year as u32);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n1, c1);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n1, c2);
            }
        };
        l.set_cursor(l.head());
        let mut j = 0usize;
        let mut f = exe::llist_get_next(l);
        while f != 0 {
            if this.stage_index() == read_u8(f + fixture::STAGE_INDEX) {
                let e = cup_round_entry(this, round);
                if j < 0x38 && read_u8(e + j + 0x29) != 0xff {
                    let mut buf = [0u8; 0x50];
                    let r = Fixture(buf.as_mut_ptr() as usize);
                    core::ptr::copy_nonoverlapping(f as *const u8, r.addr() as *mut u8, 0x4f);
                    if fixture_on_round_date(this, e, Fixture(f)) {
                        let p = exe::crt_operator_new(8);
                        let dp = if p != 0 { exe::new_date(p, read_i16(f + fixture::DAY) as u16 as u32, read_i16(f + fixture::YEAR) as u16 as u32) } else { 0 };
                        let mut t = [0u8; 8];
                        let pt = t.as_mut_ptr() as usize;
                        exe::date_date_54a110(pt);
                        let e = cup_round_entry(this, round);
                        exe::date_get_nearest_weekday(dp, read_u8(e + j + 0x27) as i8 as i16 as u16 as u32, pt); // cqc7: the scheduled weekday
                        if read_u8(cup_round_entry(this, round) + j + 0x29) != 3 {
                            let mut noticia2 = [0u32; 0x38];
                            let n2 = noticia2.as_mut_ptr() as usize;
                            exe::new_newsitem(n2, 3, 0);
                            exe::newsitem_set_slot(n2, 0, r.home_club_id() as u32);
                            exe::newsitem_set_slot(n2, 1, r.away_club_id() as u32);
                            exe::newsitem_set_slot(n2, 2, read_usize(r.addr()) as u32);
                            exe::newsitem_set_slot(n2, 3, r.round_prefix() as i32 as u32);
                            exe::newsitem_set_slot(n2, 4, r.round_name() as i32 as u32);
                            exe::newsitem_set_slot(n2, 5, (r.day() as i16) as u16 as u32);
                            exe::newsitem_set_slot(n2, 6, (r.year() as i16) as u16 as u32);
                            exe::newsitem_set_slot(n2, 7, r.time_slice() as u32);
                            exe::newsitem_set_slot(n2, 8, read_i16(pt) as i32 as u32);
                            exe::newsitem_set_slot(n2, 9, read_i16(pt + 2) as u16 as u32);
                            exe::newsitem_set_slot(n2, 10, read_u8(cup_round_entry(this, round) + j + 0x28) as i8 as i32 as u32);
                            exe::newsitem_set_slot(n2, 11, read_u8(cup_round_entry(this, round) + j + 0x29) as i8 as i32 as u32);
                            exe::news_send_to_club_manager(read_usize(G_NEWS), n2, r.home_club());
                            exe::news_send_to_club_manager(read_usize(G_NEWS), n2, r.away_club());
                        }
                        exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), r);
                        crate::comparison::branch("cqc_7 match moved to the date");
                        r.set_day(read_i16(pt) as u16); // cqc7: o dia marcado
                        r.set_year(read_i16(pt + 2) as u16);
                        let v = cup_round_entry(this, round) + j;
                        r.set_time_slice(read_u8(v + 0x28)); // cqc7: a fatia marcada
                        r.set_stage(read_u8(v + 0x29));
                        exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(cup_round_entry(this, round) + j + 0x23), r, this.venue_stadium_id());
                        exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r, 1);
                        if dp != 0 {
                            exe::crt_operator_delete(dp);
                        }
                        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 {
                            exe::drop_llist(l);
                            return;
                        }
                        write_u8(cup_round_entry(this, round) + j + 0x29, 0xff);
                    }
                    news_of_game(r.home_club().addr(), r.away_club().addr(), r.home_club_id(), r.away_club_id(), r.stadium_id(),
                        (r.day() as i16) as u16, (r.year() as i16) as u16);
                    j += 7;
                } else {
                    news_of_game(read_usize(f + fixture::HOME_CLUB), read_usize(f + fixture::AWAY_CLUB), read_usize(f + fixture::HOME_CLUB_ID), read_usize(f + fixture::AWAY_CLUB_ID), read_usize(f + fixture::STADIUM_ID),
                        read_i16(f + fixture::DAY) as u16, read_i16(f + fixture::YEAR) as u16);
                }
            }
            f = exe::llist_get_next(l);
        }
    }
    exe::drop_llist(l);
}

/// Builds the cup's fixture tree: allocates the ties of each round, clears them, links each tie to a slot in the next
/// round (by pair or drawn, the side by a coin toss; with no slot, two rounds ahead), puts each round's teams into its
/// ties (drawn, the cup nation's club at home) and numbers the first tie of each round. Returns 1.
///
/// `Cup::map_fixture_tree()`, called when the cups are created (22 calls up to 2002-06-01).
///
/// The tie arrays of each round (`0x944e46`: 4 per round and `0x35` per tie; out of memory, the "Memory Allocation
/// Error" box of line `0x8b` or `0x9e` and the game stops; a round with no ties, the box of line `0x95` and -1); flag
/// `+5` of each team (`+0xa7`, 6 bytes) and `+0xab`/`+0xac` cleared. With clubs (`+0x45`): each tie empty (the teams
/// and the slots −1, `+8` = 0, `+0x14..+0x31` = `0xff`, `+0x32..+0x34` = 0).
///
/// In the rounds up to the second-to-last (the third-to-last when the last is of type `0x8c`): the order of the ties
/// (shuffled in drawn pairs if the next round has fewer than half); each one gets its slot in the next round: by pair
/// (round `+0xd` bit `0x200`: p/2) or drawn (with bit `0x400`, the first fully free slot from there), the side by coin
/// toss (`0x90cfc0(2)`; if taken, the next slot and another toss); with no free slot, a slot drawn two rounds ahead
/// (without it, the game stops: lines `0x175` and `0x1a0`). The tie stores the slot (`+6`), the side (`+8`: 1 home) and
/// the destination round (`+0x12`); the slot stores the tie (`+0xa/+0xc`) and the source round (`+0xe/+0x10`).
///
/// Then the teams of each round (`+0x1c`, from `+0x1e`) go into its ties: the first by pair or drawn (`0x400`: the
/// first fully free), the side by coin toss, and in a taken slot the team of nation `+0x24` becomes the home team (no
/// slot: the box of line `0x1c2` or `0x222` and -1). With no ties in the first round, `+0xb1` = 1. Finally the number
/// of the first tie of each round (`+0x5b`), from back to front.
/// Original: `Cup::map_fixture_tree` (0x518790, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x518790 } (thiscall, ret 0, Cup.cpp): `Cup::map_fixture_tree()`, da criação das copas (22 chamadas até
/// 2002-06-01). Os vetores dos confrontos de cada rodada (0x944e46: 4 por rodada e 0x35 por confronto; sem memória, a
/// caixa "Memory Allocation Error" da linha 0x8b ou 0x9e e o jogo para; rodada sem confrontos, a caixa da linha 0x95 e
/// -1); a marca +5 de cada time (+0xa7, 6 bytes) e +0xab/+0xac zeradas. Com clubes (+0x45): cada confronto vazio (os
/// times e as vagas −1, +8 = 0, +0x14..+0x31 = 0xff, +0x32..+0x34 = 0). Nas rodadas até a penúltima (a antepenúltima com
/// a última do tipo 0x8c): a ordem dos confrontos (embaralhada aos pares sorteados se a rodada seguinte tem menos da
/// metade); cada um ganha a vaga na rodada seguinte: pelo par (rodada +0xd bit 0x200: p/2) ou sorteada (com o bit 0x400,
/// a primeira vaga toda livre a partir dela), o lado pela moeda (0x90cfc0(2); ocupada, a vaga seguinte e outra moeda);
/// sem vaga livre, uma vaga sorteada duas rodadas adiante (sem ela, o jogo para: linhas 0x175 e 0x1a0). O confronto
/// guarda a vaga (+6), o lado (+8: 1 mandante) e a rodada de destino (+0x12); a vaga guarda o confronto (+0xa/+0xc) e a
/// rodada de origem (+0xe/+0x10). Depois os times de cada rodada (+0x1c, a partir de +0x1e) entram nos confrontos dela:
/// o primeiro pelo par ou sorteado (0x400: o primeiro todo livre), o lado pela moeda, e numa vaga ocupada o time da
/// nação +0x24 vira mandante (sem vaga, a caixa da linha 0x1c2 ou 0x222 e -1). Sem confrontos na primeira rodada, +0xb1
/// = 1. Por fim o número do primeiro confronto de cada rodada (+0x5b), de trás para a frente. Devolve 1.
pub(crate) unsafe extern "thiscall" fn map_fixture_tree(this: usize) -> i32 {
    let this = Cup(this);
    let t = exe::crt_malloc((this.round_count() as i32 as u32) << 2);
    this.set_tie_tables(t);
    if t == 0 {
        fatal_memory_error_box(0x8b, 0x9b5600);
    }
    let mut i = 0i32;
    while i < this.round_count() as i32 {
        let n = read_i16(comp_cup_round(this, i) + 0x1a);
        if n < 1 {
            show_error_at(0x9b55ac, 0x95);
            return -1;
        }
        let v = exe::crt_malloc((n as i32 * 0x35) as u32);
        write_usize(this.tie_tables() + i as usize * 4, v);
        if read_usize(this.tie_tables() + i as usize * 4) == 0 {
            fatal_memory_error_box(0x9e, 0x9b55d8);
        }
        i += 1;
    }
    if this.team_table() != 0 {
        let mut k = 0i32;
        while k < this.team_count() as i32 {
            write_u8(this.team_table() + k as usize * 6 + 5, 0);
            k += 1;
        }
    }
    this.set_current_round(0);
    this.set_next_draw_round(0);
    if this.season_set_up() == 0 {
        return 1;
    }
    let mut i = 0i32;
    while i < this.round_count() as i32 {
        let mut j = 0i32;
        while j < read_i16(comp_cup_round(this, i) + 0x1a) as i32 {
            let c = cup_tie(this, i, j);
            for k in [0usize, 2, 6, 0xa, 0xc, 0xe, 0x10, 0x12] {
                write_u16(c + k, 0xffff);
            }
            write_u16(c + cup_tie::FEEDER_SIDE, 0);
            for k in 0x14..=0x31usize {
                write_u8(c + k, 0xff);
            }
            write_u8(c + 0x32, 0);
            write_u8(c + 0x33, 0);
            write_u8(c + cup_tie::FIXTURE_STATE, 0);
            j += 1;
        }
        i += 1;
    }
    let nr = this.round_count();
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut r = 0i32;
    while r < m - 1 {
        let n = read_i16(comp_cup_round(this, r) + 0x1a);
        let order = if n != 0 { exe::crt_malloc(n as i32 as u32) } else { 0 };
        let mut k = 0i32;
        while k < read_i16(comp_cup_round(this, r) + 0x1a) as i32 {
            write_u8(order + k as usize, k as u8);
            k += 1;
        }
        let nn = read_i16(comp_cup_round(this, r) + 0x1a) as i32;
        let next = read_i16(comp_cup_round(this, r) + 0x82) as i32;
        if next < nn / 2 && nn > 0 {
            let (mut q, mut conta) = (0i32, nn);
            loop {
                let a = exe::utils_random(conta) as usize;
                let b = exe::utils_random(read_i16(comp_cup_round(this, r) + 0x1a) as i32) as usize;
                let x = read_u8(order + a);
                write_u8(order + a, read_u8(order + b));
                write_u8(order + b, x);
                q += 1;
                conta = read_i16(comp_cup_round(this, r) + 0x1a) as i32;
                if q >= conta {
                    break;
                }
            }
        }
        let mut k = 0i32;
        while k < read_i16(comp_cup_round(this, r) + 0x1a) as i32 {
            let rr = comp_cup_round(this, r);
            let p = read_u8(order + k as usize) as i8 as i32;
            let mut s: i32;
            let mut start: i32;
            if read_i16(rr + 0x75) as u16 & 0x200 != 0 {
                s = p / 2; // cqd7: the slot by the pair
                start = s;
            } else {
                s = exe::utils_random(read_i16(rr + 0x82) as i32);
                start = s;
                if read_i16(rr + 0x75) as u16 & 0x400 != 0 {
                    loop {
                        let v = cup_tie(this, r + 1, s as i16 as i32);
                        if read_i16(v + cup_tie::HOME_LINK_TIE) == -1 && read_i16(v + cup_tie::AWAY_LINK_TIE) == -1 {
                            start = s;
                            break;
                        }
                        s += 1;
                        if s as i16 >= read_i16(rr + 0x82) {
                            s = 0;
                        }
                        if s as i16 == start as i16 {
                            break;
                        }
                    }
                }
            }
            let (mut s, start) = (s, start); // cqd7: the initial slot in the next round
            let c = cup_tie(this, r, p);
            loop {
                let moeda = exe::utils_random(2);
                let v = cup_tie(this, r + 1, s as i16 as i32);
                let first = moeda != 0; // cqd7: the side coin in the next round
                let side = if first {
                    if read_i16(v + cup_tie::HOME_LINK_TIE) == -1 { Some(true) } else if read_i16(v + cup_tie::AWAY_LINK_TIE) == -1 { Some(false) } else { None }
                } else if read_i16(v + cup_tie::AWAY_LINK_TIE) == -1 {
                    Some(false)
                } else if read_i16(v + cup_tie::HOME_LINK_TIE) == -1 {
                    Some(true)
                } else {
                    None
                };
                if let Some(a) = side {
                    crate::comparison::branch("cqd_7 place in the next round");
                    write_u16(v + if a { 0xa } else { 0xc }, p as u16);
                    write_u16(v + if a { 0xe } else { 0x10 }, r as u16);
                    write_u16(c + cup_tie::FEEDER_TIE, s as u16);
                    write_u16(c + cup_tie::FEEDER_SIDE, a as u16);
                    write_u16(c + cup_tie::FEEDER_ROUND, (r + 1) as u16);
                    break;
                }
                if (s as i16 as i32) < read_i16(comp_cup_round(this, r) + 0x82) as i32 - 1 {
                    s += 1;
                } else {
                    s = 0;
                }
                if start as i16 == s as i16 {
                    // no free slot: a random slot two rounds ahead
                    if r >= m - 2 {
                        fatal_error_at(0x9b55ac, 0x175);
                    }
                    let mut t = exe::utils_random(read_i16(comp_cup_round(this, r) + 0xea) as i32);
                    let ini = t;
                    loop {
                        let v = cup_tie(this, r + 2, t as i16 as i32);
                        let a = if read_i16(v + cup_tie::HOME_LINK_TIE) == -1 {
                            true
                        } else if read_i16(v + cup_tie::AWAY_LINK_TIE) == -1 {
                            false
                        } else {
                            if (t as i16 as i32) < read_i16(comp_cup_round(this, r) + 0xea) as i32 - 1 {
                                t += 1;
                            } else {
                                t = 0;
                            }
                            if t as i16 == ini as i16 {
                                fatal_error_at(0x9b55ac, 0x1a0);
                            }
                            continue;
                        };
                        write_u16(v + if a { 0xa } else { 0xc }, p as u16);
                        write_u16(v + if a { 0xe } else { 0x10 }, r as u16);
                        write_u16(c + cup_tie::FEEDER_TIE, t as u16);
                        write_u16(c + cup_tie::FEEDER_SIDE, a as u16);
                        break;
                    }
                    write_u16(c + cup_tie::FEEDER_ROUND, (r + 2) as u16);
                    break;
                }
            }
            k += 1;
        }
        if order != 0 {
            exe::crt_free(order);
        }
        r += 1;
    }
    if read_i16(comp_cup_round(this, 0) + 0x1a) == 0 {
        this.set_run_by_stages(1);
        return 1;
    }
    // the teams of each round in its ties; `inicio` moves from one team to the other, as in the original
    let mut start: i32 = -1;
    let mut r = 0i32;
    while r < m {
        if read_i16(comp_cup_round(this, r) + 0x1c) > 0 {
            let mut q = 0i32;
            loop {
                let rr = comp_cup_round(this, r);
                let mut di: i32;
                if read_i16(rr + cup_round::TIE_RULES) as u16 & 0x200 != 0 {
                    di = 0;
                    if read_i16(rr + cup_round::TIES) <= 0 {
                        show_error_at(0x9b55ac, 0x1c2);
                        return -1;
                    }
                } else {
                    di = exe::utils_random(read_i16(rr + cup_round::TIES) as i32);
                    start = di;
                    let rr = comp_cup_round(this, r);
                    if read_i16(rr + cup_round::TIE_RULES) as u16 & 0x400 != 0 {
                        loop {
                            let c = cup_tie(this, r, di as i16 as i32);
                            if read_i16(c) == -1 && read_i16(c + cup_tie::AWAY_TEAM) == -1 {
                                start = di;
                                break;
                            }
                            di += 1;
                            if di as i16 >= read_i16(rr + cup_round::TIES) {
                                di = 0;
                            }
                            if di as i16 == start as i16 {
                                break;
                            }
                        }
                    }
                }
                loop {
                    let moeda = exe::utils_random(2);
                    let x = di as i16 as i32;
                    let c = cup_tie(this, r, x);
                    let livre2 = || read_i16(c + cup_tie::AWAY_TEAM) == -1 && read_i16(c + cup_tie::AWAY_LINK_TIE) == -1;
                    let first_0 = moeda != 0; // cqd7: the side coin in the first round
                    let v0 = read_i16(c) as u16;
                    let livre0 = v0 == 0xffff && read_i16(c + cup_tie::HOME_LINK_TIE) == -1;
                    let side = if first_0 {
                        if livre0 { Some(0usize) } else if livre2() { Some(2) } else { None }
                    } else if livre2() {
                        Some(2)
                    } else if livre0 {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(o) = side {
                        crate::comparison::branch("cqd_7 team in the first round");
                        write_u16(c + o, (read_i16(comp_cup_round(this, r) + 0x1e) as u16).wrapping_add(q as u16));
                        break;
                    }
                    // the occupied slot: the team of nation +0x24 on the outer side becomes the home team
                    if this.venue_stadium_id() as u32 != u32::MAX && v0 != 0xffff {
                        let away = read_i16(c + cup_tie::AWAY_TEAM);
                        if away != -1 {
                            let club = Club(read_usize(this.team_table() + (away as isize * 6) as usize));
                            if !club.is_null() {
                                let nation = club.stadium().addr();
                                if nation != 0 && read_usize(nation) == this.venue_stadium_id() {
                                    write_u16(c, away as u16);
                                    write_u16(c + cup_tie::AWAY_TEAM, v0);
                                }
                            }
                        }
                    }
                    if x < read_i16(comp_cup_round(this, r) + 0x1a) as i32 - 1 {
                        di += 1;
                    } else {
                        di = 0;
                    }
                    if start as i16 == di as i16 {
                        show_error_at(0x9b55ac, 0x222);
                        return -1;
                    }
                }
                q += 1;
                if q >= read_i16(comp_cup_round(this, r) + 0x1c) as i32 {
                    break;
                }
            }
        }
        r += 1;
    }
    let nr = this.round_count();
    if nr > 0 && read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c {
        let mut k = nr.wrapping_sub(3) as i32;
        let mut n: u8 = 5;
        while k >= 0 {
            write_u8(comp_cup_round(this, k) + 0x5b, n);
            n = n.wrapping_add(read_u8(comp_cup_round(this, k) + 0x1a));
            k -= 1;
        }
    } else {
        let mut n = read_u8(comp_cup_round(this, nr as i32).wrapping_sub(0x50)).wrapping_add(1);
        let mut k = nr.wrapping_sub(2) as i32;
        while k >= 0 {
            write_u8(comp_cup_round(this, k) + 0x5b, n);
            n = n.wrapping_add(read_u8(comp_cup_round(this, k) + 0x1a));
            k -= 1;
        }
    }
    1
}

/// Sorts the cup clubs by strength and gives each one the predicted position of the round it goes out in, in the cup
/// statistics.
///
/// `Cup::rank_teams()`: copies the cup clubs (`[+0xa7]`, 6-byte records, `+0x3e` of them) into an array (`0x944e46`),
/// sorts by strength (`0x4c08f0`) and gives each one the predicted position by the round it goes out in (`0x4a2540` in
/// the statistics `[+8]`); frees the array (`0x9452ca`).
///
/// No statistics: the error box (line `0x1827`); out of memory: the original routine runs (the out-of-memory box and
/// the end of the game).
/// Original: `Cup::rank_teams` (0x5223a0, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5223a0 } (thiscall, ret 0, Cup.cpp): `Cup::rank_teams()`: copia os clubes da copa ([+0xa7], registros de 6
/// bytes, +0x3e deles) para um vetor (0x944e46), ordena pela força (0x4c08f0) e dá a cada um a posição prevista pela
/// rodada em que sai (0x4a2540 na estatística [+8]); libera o vetor (0x9452ca). Sem estatística: a caixa (linha
/// 0x1827); sem memória: a original (a caixa da falta de memória e o fim do jogo).
pub(crate) unsafe extern "thiscall" fn cup_rank_teams(this: usize) {
    let this = Cup(this);
    if read_usize(this.addr() + 8) == 0 {
        return show_error_at(0x9b55ac, 0x1827);
    }
    let n = this.team_count() as i32;
    let v = exe::crt_malloc((n << 2) as u32);
    if v == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RANK_TEAMS)(this.addr());
    }
    for i in 0..n.max(0) as usize {
        write_u32(v + i * 4, read_usize(this.team_table() + i * 6) as u32);
    }
    exe::comp_util_sort_clubs_by_strength(v, n);
    set_final_positions_by_round(this, v as *const u32, 1);
    exe::crt_free(v);
}

/// Stores one of the eight cup round match days: the venue flags, the weekday, the time slot and the code. Returns the
/// venue flags.
///
/// Day `vaga` (0 to 7) of round `rodada` (104-byte records): in 7 bytes from `+0x23`, the venue (4 bytes), the weekday
/// (−1 to 6), the time slot (−1 to 2) and the code (−1 to 3). Out of range: the original routine runs (error box and
/// end of the game).
/// Original: `cup_set_round_match_day` (0x521f10, Cup.cpp, cdecl).
///
/// [pt] { 0x521f10 } (cdecl, Cup.cpp): o dia `vaga` (0 a 7) da rodada `rodada` (registros de 104 bytes):
/// em 7 bytes a partir de +0x23, o mando (4 bytes), o dia da semana (−1 a 6), o horário (−1 a 2) e o
/// código (−1 a 3). Fora da faixa: a original (caixa de erro e fim do jogo). Devolve o mando.
pub(crate) unsafe extern "C" fn cup_set_round_match_day(rounds: usize, round: i16, slot: i16, week: i16, hour: i8, code: i8, mando: u32) -> u32 {
    if !(0..8).contains(&slot) || !(-1..=6).contains(&week) || !(-1..3).contains(&hour) || !(-1..=3).contains(&code) {
        return original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(IDX_CUP_SET_ROUND_MATCH_DAY)(rounds, round, slot, week, hour, code, mando);
    }
    let e = rounds.wrapping_add((round as isize * 104 + slot as isize * 7) as usize);
    write_u8(e + cup_round::SLOT_WEEKDAY, week as u8); // day of the week
    write_u8(e + cup_round::SLOT_TIME_SLICE, hour as u8);
    write_u8(e + cup_round::SLOT_CODE, code as u8);
    write_u32(e + cup_round::SLOT_VENUE, mando);
    mando
}

/// tells whether the club is in the cup's team table
/// Original: `Cup::has_club` (0x522910, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x522910 } (thiscall, ret 4, Cup.cpp; virtual +0x80 das copas, chamada pela 0x4ae300 da projeção da diretoria):
/// `clube.has_club()`: 1 se o clube está na tabela de times da copa (+0xa7, 6 bytes por time, +0 o clube; conta
/// +0x3e); senão 0.
pub(crate) unsafe extern "thiscall" fn has_club(this: usize, club: u32) -> u32 {
    let this = Cup(this);
    let n = this.team_count();
    let teams = (this.team_table() as u32) as usize;
    for i in 0..n.max(0) as usize {
        if read_u32(teams + i * 6) == club {
            crate::comparison::branch("cpr_13 cup has the club");
            return 1; // is in the cup
        }
    }
    0
}

