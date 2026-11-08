use super::*;

/// Adds v to the byte at address a (wrapping).
pub(crate) unsafe fn add_u8_at(a: usize, v: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(v);
}

/// Adds v to the unaligned u16 at address a (wrapping).
pub(crate) unsafe fn add_u16_at(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, core::ptr::read_unaligned(a as *const u16).wrapping_add(v));
}

/// Adds v to the unaligned f64 at address a.
pub(crate) unsafe fn add_f64_at(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v + core::ptr::read_unaligned(a as *const f64));
}

/// PLAYERSTATS get_category_record(player, category).
pub(crate) unsafe fn player_category_record(this: usize, player: usize, cat: u8) -> usize {
    exe::playerstats_get_category_record(this, player, cat as u32)
}

/// The statistics category of side `lado` of a match (the `bl` of `0x7c0d70`, a match whose competition ≠ −2).
///
/// Friendly (`0x5c6710` of [`0xae2c74`]): 5 if the home side is a national team (`0x53b900`), otherwise 1. Reserves,
/// under-21 and B ([`0x9cf684`], [`0x9cf798`], [`0x9cf794`]), or the side's club without the mark `+0x40` (home) /
/// `+0x41` (away): 1.
///
/// By the type (`+0x42`) of the competition ([`0xadadfc`] by id `+0`): 4 → 5, 2 → 4, 1 → 2 if it counts (`0x4b0700`),
/// otherwise 3.
///
/// With no competition or any other type: the box (lines `0x103a`, `0x105d`; only with `avisar`), sets `erro` and
/// returns 2.
///
/// [pt] A categoria das estatísticas do lado `lado` da partida (o bl da 0x7c0d70, partida com competição ≠ −2): amistoso
/// (0x5c6710 de [0xae2c74]), 5 se o mandante é seleção (0x53b900), senão 1; reservas, sub-21 e B
/// ([0x9cf684], [0x9cf798], [0x9cf794]) ou o clube do lado sem a marca +0x40 (mandante) / +0x41 (visitante): 1. Pelo
/// tipo (+0x42) da competição ([0xadadfc] pelo id +0): 4 → 5, 2 → 4, 1 → 2 se ela conta (0x4b0700), senão 3. Sem a
/// competição ou outro tipo: a caixa (linhas 0x103a, 0x105d; só com `avisar`), `erro` e 2.
pub(crate) unsafe fn match_stats_category(p: usize, side: i32, warn: bool, error: &mut bool) -> u8 {
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let count: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let club = read_usize(p + if side == 0 { 0x1c } else { 0x20 });
    if friendly(read_usize(G_FRIENDLY), read_usize(p + fixture::COMPETITION)) != 0 {
        return if national_team(read_usize(p + fixture::HOME_CLUB)) != 0 { 5 } else { 1 };
    }
    let id = read_usize(read_usize(p + fixture::COMPETITION));
    if id == read_usize(KEY_CLUB_COMP_RESERVE) || id == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) || id == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) {
        return 1;
    }
    if club == read_usize(p + fixture::HOME_CLUB) && read_u8(p + fixture::HOME_SIDE_MARK) == 0 {
        return 1;
    }
    if club == read_usize(p + fixture::AWAY_CLUB) && read_u8(p + fixture::AWAY_SIDE_MARK) == 0 {
        return 1;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(p).wrapping_mul(4)));
    if comp == 0 {
        *error = true;
        if warn {
            show_error_at(0xa78310, 0x103a);
        }
        return 2;
    }
    match read_u8(comp + cup::CLUB_SIDES) {
        4 => 5,
        2 => 4,
        1 => {
            if warn {
                crate::comparison::branch("cpx_12 categoria da liga");
            }
            if count(read_usize(p) as i32) != 0 { 2 } else { 3 } // cpx12: the league category
        }
        _ => {
            *error = true;
            if warn {
                show_error_at(0xa78310, 0x105d);
            }
            2
        }
    }
}

/// Tells whether lineup row `k` of side `lado` of the match counts, and returns it.
///
/// Rows are `k` = 0..20, 0x49 bytes each; 11 and up are substitutes. A row counts when: a substitute actually came on
/// (+0x32 ≠ 0xff), the person (+0) and the player (+4) are ≠ −1, and the person is in the table ([0xae23bc], 0x6e
/// bytes; id below [0xae2364]). Returns (row, person, player data in [0xae23c4], 0x46 bytes).
///
/// [pt] A linha `k` (0..20, 0x49 bytes; 11.. reservas) do lado `lado` do jogo conta: reserva que entrou (+0x32 ≠ 0xff),
/// pessoa (+0) e jogador (+4) ≠ −1 e a pessoa na tabela ([0xae23bc], 0x6e; id abaixo de [0xae2364]). Devolve
/// (linha, pessoa, dados do jogador em [0xae23c4], 0x46).
pub(crate) unsafe fn match_lineup_row(game: usize, side: usize, k: usize) -> Option<(usize, usize, usize)> {
    let e = game + match_stats_record::SIDE_SIZE * side + 0x190 + 0x49 * k;
    if k > 10 && read_u8(e + lineup_row::ENTRY_MINUTE) == 0xff {
        return None;
    }
    let (sid, pid) = (read_usize(e) as i32, read_usize(e + lineup_row::PLAYER_ID) as i32);
    if sid == -1 || pid == -1 {
        return None;
    }
    let person = read_usize(G_STAFF_TABLE).wrapping_add((sid as usize).wrapping_mul(staff::SIZE));
    if person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return None;
    }
    Some((e, person, read_usize(G_PLAYER_DATA_TABLE).wrapping_add((pid as usize).wrapping_mul(0x46))))
}

/// Categories updated per pass: the match's and the overall 6 (skipped for club and national friendlies).
///
/// [pt] As categorias de cada passada: a do jogo e a geral (6), que fica de fora no amistoso de clube (1) e no de seleção
/// (5).
pub(crate) fn stats_categories_for(cat: u8) -> [Option<u8>; 2] {
    [Some(cat), if cat == 5 || cat == 1 { None } else { Some(6) }] // cpx12: a categoria geral
}

/// The comparer's dry run of 0x447430: the copy runs without sorting the tables (the original routine, which runs
/// first, has already sorted them).
///
/// [pt] O ensaio do comparador da 0x447430: a cópia roda sem ordenar as tabelas (a original, que roda antes, já ordenou).
pub(crate) static mut POSITION_RECORDS_DRY_RUN: bool = false;

/// CLUB_RECORDS get_season_record(club, create).
pub(crate) unsafe fn club_season_record(this: usize, club: usize, create: u32) -> usize {
    exe::club_records_get_season_record(this, club, create)
}

/// CLUB_RECORDS get_all_time_record(club, create).
pub(crate) unsafe fn club_all_time_record(this: usize, club: usize, create: u32) -> usize {
    exe::club_records_get_all_time_record(this, club, create)
}

/// Reputation (+0x69, i16) of competition id (0x6b-byte records in G_CLUB_COMP_TABLE).
///
/// [pt] A reputação (+0x69, palavra com sinal) da competição `id` (registros de 0x6b em [0xae23d0]).
pub(crate) unsafe fn competition_reputation(id: i32) -> i16 {
    read_i16(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)) + 0x69)
}

/// For a human club or national team with 5+ seasons, queues the all-time record news (club id, type) unless already
/// queued at +0x24.
///
/// [pt] O recorde de sempre mudou: clube humano (0x600a50) ou seleção (0x53b900) com 5 temporadas ou mais (+0x68 do
/// resumo) e o recorde (id, tipo) ainda fora da fila (+0x24, 6 bytes, conta +0x2c): entra na fila (0x452cb0).
pub(crate) unsafe fn club_records_queue_record_news(this: usize, club: usize, summary: usize, kind: u8) {
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let store: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_QUEUE_RECORD_NEWS);
    if human(club, 0, 1, 0, 0) == 0 && national_team(club) == 0 {
        return;
    }
    if read_i16(summary + club_record_entry::SEASONS) < 5 {
        return;
    }
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32;
    let mut e = read_usize(this + club_records_fields::NEWS_QUEUE);
    for _ in 0..n.max(0) {
        if read_usize(e) == read_usize(club) && read_u8(e + record_news_item::RECORD_TYPE) == kind {
            return;
        }
        e += 6;
    }
    store(this, read_usize(club), kind as u32);
}

/// The league and its following type-2 stages with the same number of rows (+0x3e).
///
/// [pt] As ligas da cadeia da 0x447430: a liga e, enquanto houver, a fase seguinte (0x48d200 da primeira, índice +0x44 + 1
/// até +0x30) do tipo 2 (+0x43) com as mesmas +0x3e linhas.
pub(crate) unsafe fn league_stage_chain(liga0: usize) -> Vec<usize> {
    let mut v = vec![liga0];
    let mut league = liga0;
    loop {
        let k = read_u8(league + league::STAGE_INDEX) as i8 as i32;
        if k >= read_usize(liga0 + league::CURRENT_STAGE) as i32 {
            return v;
        }
        let f = exe::comp_get_stage(liga0, k + 1);
        if read_u8(f + league::COMP_TYPE) != 2 || read_i16(f + league::CLUB_COUNT) != read_i16(league + league::CLUB_COUNT) {
            return v;
        }
        v.push(f);
        league = f;
    }
}

/// `Club_Records::update_league_position_records(league, all_time)`: sorts the table of each league stage and stores
/// each club's best and worst league position for the season and, with the second argument, all time (queueing the news
/// item for a human club or national side).
///
/// A null league, or one whose +0x44 is not 0xff, gives the error boxes and 0. In each league of the chain
/// (league_stage_chain) it sorts the table (0x6827d0) and, for each row that has games (+0x11) and a club, through the
/// season records (0x4528d0) and the all-time records (0x452710, created when missing): best position (type 1,
/// 0x449590) and worst (type 2). The position is written when there is no record yet (−1), when the competition (the id
/// of the first league) has a higher reputation (best) or a lower one (worst), or when it is the same competition with
/// a better or worse position: it stores the position (+0/+7), the competition (+1/+8) and the year (+5/+0xc; 0 for the
/// season, 0x669950 for all time). For all time (with `all_time`; an id above [0xae2378] also counts as "no record"),
/// the news queue (club_records_queue_record_news). Returns 1.
/// Original: `Club_Records::update_league_position_records` (0x447430, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x447430 } (thiscall, ret 8, club_records.cpp): `Club_Records::update_league_position_records(liga, sempre)`:
/// liga nula ou com +0x44 ≠ 0xff, as caixas e 0. Em cada liga da cadeia (league_stage_chain), ordena a tabela (0x6827d0) e,
/// em cada linha com jogos (+0x11) e clube, pelos registros da temporada (0x4528d0) e de sempre (0x452710, criando):
/// melhor posição (tipo 1, 0x449590) e pior (tipo 2): sem registro (−1), competição (o id da primeira liga) de
/// reputação maior (melhor) ou menor (pior), ou a mesma com posição melhor ou pior: grava a posição (+0/+7), a
/// competição (+1/+8) e o ano (+5/+0xc; 0 na temporada, 0x669950 em sempre). Em sempre (com `sempre`; id acima de
/// [0xae2378] também é "sem"), a fila da notícia (club_records_queue_record_news). Devolve 1.
pub(crate) unsafe extern "thiscall" fn update_league_position_records(this: usize, liga0: usize, always: u32) -> u32 {
    if liga0 == 0 || read_u8(liga0 + league::STAGE_INDEX) != 0xff {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_UPDATE_LEAGUE_POSITION_RECORDS)(this, liga0, always); // the boxes
    }
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let year_of_season: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_KEY_NATION_GET_CLUB_SEASON_YEAR);
    let comp = read_usize(read_usize(liga0 + 4)) as i32;
    for liga in league_stage_chain(liga0) {
        if !POSITION_RECORDS_DRY_RUN {
            sort(liga, 0);
        }
        let mut i = 0i32;
        while i < read_i16(liga + league::CLUB_COUNT) as i32 {
            let row = read_usize(liga + league::TABLE).wrapping_add(0x3b * i as usize);
            let club = read_usize(row);
            i += 1;
            if read_u8(row + league_table_row::PLAYED) == 0 || club == 0 {
                continue;
            }
            let t = club_season_record(this, club, 1);
            let s = club_all_time_record(this, club, 1);
            let summary = read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add(read_usize(club).wrapping_mul(0x6a));
            let year = year_of_season(club) as u16;
            let pos = read_u8(row + league_table_row::POSITION);
            let away = |v: i32| v < 0 || v > read_usize(G_CLUB_COMP_COUNT) as i32;
            if exe::club_records_record_type_applies(1, club, 1) != 0 {
                let v = read_usize(t + club_record::BEST_POSITION_COMP) as i32;
                if v < 0 || competition_reputation(comp) > competition_reputation(v) || (comp == v && pos < read_u8(t)) {
                    *(t as *mut u8) = pos;
                    core::ptr::write_unaligned((t + club_record::BEST_POSITION_COMP) as *mut i32, comp);
                    core::ptr::write_unaligned((t + club_record::BEST_POSITION_YEAR) as *mut u16, 0);
                }
                let v = read_usize(s + club_record::BEST_POSITION_COMP) as i32;
                if always != 0 && (away(v) || competition_reputation(comp) > competition_reputation(v) || (comp == v && pos < read_u8(s))) {
                    crate::comparison::branch("cpx_12 melhor posicao de sempre");
                    *(s as *mut u8) = pos;
                    core::ptr::write_unaligned((s + club_record::BEST_POSITION_COMP) as *mut i32, comp);
                    core::ptr::write_unaligned((s + club_record::BEST_POSITION_YEAR) as *mut u16, year);
                    club_records_queue_record_news(this, club, summary, 1);
                }
            }
            if exe::club_records_record_type_applies(2, club, 1) != 0 {
                let v = read_usize(t + club_record::WORST_POSITION_COMP) as i32;
                if v < 0 || competition_reputation(comp) < competition_reputation(v) || (comp == v && pos > read_u8(t + club_record::WORST_POSITION)) {
                    *((t + club_record::WORST_POSITION) as *mut u8) = pos;
                    core::ptr::write_unaligned((t + club_record::WORST_POSITION_COMP) as *mut i32, comp);
                    core::ptr::write_unaligned((t + club_record::WORST_POSITION_YEAR) as *mut u16, 0);
                }
                let v = read_usize(s + club_record::WORST_POSITION_COMP) as i32;
                if always != 0 && (away(v) || competition_reputation(comp) < competition_reputation(v) || (comp == v && pos > read_u8(s + club_record::WORST_POSITION))) {
                    *((s + club_record::WORST_POSITION) as *mut u8) = pos;
                    core::ptr::write_unaligned((s + club_record::WORST_POSITION_COMP) as *mut i32, comp);
                    core::ptr::write_unaligned((s + club_record::WORST_POSITION_YEAR) as *mut u16, year);
                    club_records_queue_record_news(this, club, summary, 2);
                }
            }
        }
    }
    1
}

/// O ensaio do comparador da 0x448e30: a cópia roda sem preencher nem entregar as notícias e sem soltar a fila.
pub(crate) static mut RECORD_NEWS_DRY_RUN: bool = false;

/// `Club_Records::send_record_news()` (SEH frame without a destructor): delivers the news item of each queued record
/// (human club or national side) and empties the queue.
///
/// Without a queue (+0x24), returns 1. For each record in the queue (6 bytes: club id, type, period at +5) of a human
/// club (0x7875e0) or a national side (0x53b900): the news item 0x3e8 (local) is filled by 0x453120 with the season
/// record (period 2, 0x4528d0) or the all-time record (0x452710), without creating them; it is delivered to the nation
/// (0x7863f0) of the national side or to the club's manager (0x7861e0). A queue with capacity (+0x28) of 100 or more is
/// freed (0x9452ca) and +0x24/+0x28 are zeroed. The count (+0x2c) goes back to 0. Returns 1.
/// Original: `Club_Records::send_record_news` (0x448e30, club_records.cpp, thiscall, ret 0).
///
/// [pt] { 0x448e30 } (thiscall, ret 0, club_records.cpp, quadro SEH sem destrutor): `Club_Records::send_record_news()`: sem
/// fila (+0x24), 1. Cada recorde da fila (6 bytes: id do clube, tipo, período em +5) de clube humano (0x7875e0) ou
/// seleção (0x53b900): a notícia 0x3e8 (local) preenchida pela 0x453120 com o registro da temporada (período 2,
/// 0x4528d0) ou de sempre (0x452710), sem criar; entregue à nação (0x7863f0) da seleção ou ao técnico do clube
/// (0x7861e0). Fila com capacidade (+0x28) de 100 ou mais: solta (0x9452ca) e zera +0x24/+0x28. A conta (+0x2c) volta
/// a 0. Devolve 1.
pub(crate) unsafe extern "thiscall" fn send_record_news(this: usize) -> u32 {
    if read_usize(this + club_records_fields::NEWS_QUEUE) == 0 {
        return 1;
    }
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let mut i = 0i32;
    while i < read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32 {
        let e = read_usize(this + club_records_fields::NEWS_QUEUE) + 6 * i as usize;
        i += 1;
        let id = read_usize(e);
        let kind = read_u8(e + record_news_item::RECORD_TYPE);
        let club = read_usize(G_CLUB_TABLE).wrapping_add(id).wrapping_add(id.wrapping_mul(0x244));
        if human(club) == 0 && national_team(club) == 0 {
            continue;
        }
        crate::comparison::branch("cpx_12 noticia de recorde");
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        exe::newsitem_newsitem(n, 0x3e8, 0);
        let reg = if read_u8(e + record_news_item::PERIOD) == 2 { club_season_record(this, club, 0) } else { club_all_time_record(this, club, 0) };
        if RECORD_NEWS_DRY_RUN {
            continue;
        }
        // simplification: the original pushes the local’s dword with the type in the low byte (the rest is leftover); 0x453120 reads the byte
        exe::club_records_write_record_news_fields(id, reg, kind as u32, n);
        if national_team(club) != 0 {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), n, read_usize(club + club::NATION));
        } else {
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
        }
    }
    if read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY) as i32 >= 100 {
        if !RECORD_NEWS_DRY_RUN {
            exe::crt_free(read_usize(this + club_records_fields::NEWS_QUEUE));
        }
        core::ptr::write_unaligned((this + club_records_fields::NEWS_QUEUE) as *mut usize, 0);
        core::ptr::write_unaligned((this + club_records_fields::NEWS_QUEUE_CAPACITY) as *mut usize, 0);
    }
    core::ptr::write_unaligned((this + club_records_fields::NEWS_QUEUE_COUNT) as *mut usize, 0);
    1
}

/// The fans' rating of player `p` at the club from stats `st` (0x452da0).
///
/// Average (+0xe signed / games +0) + 0.025 × +2 + 1/60 × +6. The club likes him (0x547790): +0.2; does not like him
/// (0x5477d0): −2 (stored); he likes the club (0x547d60): +0.2; does not like it (0x547d30): −1.5 (only in the
/// comparison).
///
/// [pt] A nota da torcida para o jogador `p` do clube pela estatística `st` (0x452da0): média (+0xe com sinal / jogos +0)
/// + 0,025 × +2 + 1/60 × +6; o clube gosta dele (0x547790) +0,2, não gosta (0x5477d0) −2 (guardada); ele gosta do
/// clube (0x547d60) +0,2, não gosta (0x547d30) −1,5 (só na comparação).
pub(crate) unsafe fn fan_rating(p: usize, club: usize, st: usize) -> f64 {
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let average = read_i16(st + player_category_stats::RATING_TOTAL) as i32 as f64 / read_u8(st) as i32 as f64;
    let mut rating = average + read_u8(st + player_category_stats::GOALS) as i32 as f64 * read_f64(0x967878);
    rating = rating + read_u8(st + 6) as i32 as f64 * read_f64(0x968958);
    if exe::club_favours_staff(p, club) != 0 {
        crate::comparison::branch("cpp_10 favorito do clube");
        rating = rating + read_f64(0x968918); // o clube gosta dele
    } else if exe::club_dislikes_staff(p, club) != 0 {
        rating = rating - read_f64(0x968950);
    }
    if gosta(p, club) != 0 {
        rating + read_f64(0x968918)
    } else if exe::staff_dislikes_club(p, club) != 0 {
        rating - read_f64(0x968940)
    } else {
        rating
    }
}

/// `Club_Records::pick_fans_player_of_year(club, record)`: picks the fans' player of the year from the club's squad:
/// the best score among the players with games above the minimum; stores his reference at record +0x371 and returns 1,
/// or 0.
///
/// Without games (signed +0x10 of the club's 106-byte entry in [this+0x1c]) and with reputation (+0x80) below 3000,
/// returns 0. With the factor 0.6, falling by 0.1 while nobody has been chosen and it is above 0.2: the minimum number
/// of games is +0x10 × factor (_ftol); among the 50 squad slots (+0xd7), the players (+0x61, id below [0xae2364]) of
/// the club (+0x39) with statistic 6 (0x7c3920 over [0xb74350]) whose games are above 0 and above the minimum (signed
/// 16 bits), the one with the highest score (`fan_rating`: average + 0.025 × +2 + 1/60 × +6, club and player likes; it
/// must be above −1000). Nobody: 0; otherwise his reference (0x7e4920) at record +0x371 and 1.
/// Original: `Club_Records::pick_fans_player_of_year` (0x452da0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x452da0 } (thiscall, ret 8, club_records.cpp): `Club_Records::pick_fans_player_of_year(clube, registro)`: sem
/// jogos (+0x10 com sinal do registro de 106 bytes do clube em [this+0x1c]) e reputação (+0x80) abaixo de 3000, 0.
/// Com o fator 0,6 caindo de 0,1 enquanto ninguém foi escolhido e passa de 0,2: o mínimo de jogos é +0x10 × fator
/// (_ftol); dos 50 do elenco (+0xd7) jogadores (+0x61, id abaixo de [0xae2364]) do clube (+0x39) com a estatística 6
/// (0x7c3920 sobre [0xb74350]) de jogos acima de 0 e do mínimo (16 bits com sinal), o de maior nota
/// (`fan_rating`, acima de −1000). Ninguém: 0; senão a referência dele (0x7e4920) em registro +0x371 e 1.
pub(crate) unsafe extern "thiscall" fn pick_fans_player_of_year(this: usize, club: usize, record: usize) -> u32 {
    let games_of_club = || read_u8(read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add((read_i32(club) as usize).wrapping_mul(106)) + 0x10) as i8;
    if games_of_club() == 0 && read_i16(club + club::REPUTATION) < 0xbb8 {
        return 0;
    }
    let mut factor = 0.6f64;
    let mut best = -1000.0f64;
    let mut chosen = 0usize;
    while chosen == 0 {
        let minimum = ftol(games_of_club() as i32 as f64 * factor) as i16;
        for s in 0..50usize {
            let p = read_usize(club + club::SQUAD + 4 * s);
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            let player = read_usize(p + staff::PLAYER);
            if player == 0 || read_usize(p + staff::CLUB_JOB) != club {
                continue;
            }
            let st = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), player, 6);
            if st == 0 {
                continue;
            }
            let games = read_u8(st);
            if (games as i16) < minimum || games == 0 {
                continue;
            }
            let rating = fan_rating(p, club, st);
            if rating > best {
                best = rating;
                chosen = p;
            }
        }
        factor -= read_f64(0x967880);
        if !(factor > read_f64(0x968918)) {
            break;
        }
    }
    if chosen == 0 {
        return 0;
    }
    exe::fill_staff_name_ref(chosen, record + club_record::FANS_PLAYER);
    1
}

/// Transfer record (club id, type) is already in the news queue at [this+0x24] (count +0x2c).
///
/// [pt] O recorde de transferência já está na fila de notícias (6 bytes: id do clube, tipo) de [this+0x24], conta +0x2c?
pub(crate) unsafe fn record_news_queued(this: usize, id: usize, kind: u8) -> bool {
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    (0..read_i32(this + club_records_fields::NEWS_QUEUE_COUNT).max(0) as usize).any(|k| read_usize(queue + 6 * k) == id && read_u8(queue + 6 * k + 4) == kind)
}

/// Transfer value >= 1000 x (club reputation + 300).
///
/// [pt] A transferência de recorde do clube passa de 1000 × (reputação (+0x80) + 300)?
pub(crate) unsafe fn is_big_transfer(value: i32, club: usize) -> bool {
    value >= (read_i16(club + club::REPUTATION) as i32 + 300).wrapping_mul(1000)
}

/// Copies the record block at offset d from o to a (value, 0x18-byte reference, club, dates, flag).
///
/// [pt] Copia o bloco de recorde de `o` (+`d`: valor, a referência de 0x18 bytes, o clube, a data, a marca) para `a`.
pub(crate) unsafe fn copy_transfer_record(a: usize, o: usize, d: usize) {
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut rascunho = [0u8; 8];
    for k in 0..5 {
        core::ptr::write_unaligned((a + d + transfer_fee_record::HOLDER * k) as *mut usize, read_usize(o + d + transfer_fee_record::HOLDER * k));
    }
    copy(a + d + transfer_fee_record::HOLDER_DATE, rascunho.as_mut_ptr(), o + d + transfer_fee_record::HOLDER_DATE);
    core::ptr::write_unaligned((a + d + transfer_fee_record::OTHER_CLUB_ID) as *mut usize, read_usize(o + d + transfer_fee_record::OTHER_CLUB_ID));
    copy(a + d + transfer_fee_record::DATE, rascunho.as_mut_ptr(), o + d + transfer_fee_record::DATE);
    *((a + d + transfer_fee_record::FLAG) as *mut u8) = read_u8(o + d + transfer_fee_record::FLAG);
}

/// `Club_Records::update_transfer_fee_records(transfer)`: updates the buyer's paid record and the seller's received
/// record, for the season and all time (fee, reference, other club, date, loan).
///
/// The person (+0xc) and the buying club (+8); the person's contract (0x4cf5a0 over [0xae1050], kept in [0xae1080])
/// gives the seller (+4). The buyer's season (0x4528d0) and all-time (0x452710) records are fetched (created when
/// missing); without a seller, returns 1. A fee (+0x1b) above the season paid record (+0x313) becomes the record: the
/// fee, the person's reference (0x7e4920), the seller, today's date and the loan flag (+4 ≠ 0); if it is above the
/// all-time record it is copied there, and if it passes 1000 × (reputation + 300): the record goes to the news queue
/// (0x452cb0, 0x1c, if it is not there yet) and the manager (+0xbf) comes to favour the person (0x547a10, forced). The
/// same for the seller in the received record (+0x33c; the queue with 0x1d, without the favouring). Returns 1.
/// Original: `Club_Records::update_transfer_fee_records` (0x448760, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448760 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_transfer_fee_records(transferencia)`: a
/// pessoa (+0xc) e o clube comprador (+8); o contrato dela (0x4cf5a0 sobre [0xae1050], guardado em [0xae1080]) dá o
/// vendedor (+4). Os registros da temporada (0x4528d0) e de sempre (0x452710) do comprador (criados); sem vendedor,
/// 1. Valor (+0x1b) acima do recorde pago da temporada (+0x313): ele, a referência da pessoa (0x7e4920), o vendedor,
/// a data de hoje e o empréstimo (+4 ≠ 0); acima do de sempre, copiado para ele, e se passa de 1000 × (reputação +
/// 300): o recorde na fila de notícias (0x452cb0, 0x1c, se ainda não está) e o técnico (+0xbf) passa a gostar da pessoa
/// (0x547a10, forçado). O mesmo para o vendedor no recorde recebido (+0x33c; a fila com 0x1d, sem o gostar). 1.
pub(crate) unsafe extern "thiscall" fn update_transfer_fee_records(this: usize, t: usize) -> u32 {
    let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(t + offer::TARGET_STAFF_ID) as usize).wrapping_mul(staff::SIZE));
    let buyer = injury_club_by_index(read_i32(t + offer::BUYER_CLUB_ID));
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, contract);
    let seller = if contract != 0 { injury_club_by_index(read_i32(contract + contract::CLUB)) } else { 0 };
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let news: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_QUEUE_RECORD_NEWS);
    let mut rascunho = [0u8; 8];
    let s = exe::club_records_get_season_record(this, buyer, 1);
    let a = exe::club_records_get_all_time_record(this, buyer, 1);
    if seller == 0 {
        return 1;
    }
    let value = read_i32(t + offer::FEE);
    if value != 0 && value > read_i32(s + club_record::FEE_PAID) {
        crate::comparison::branch("cpp_10 recorde pago");
        core::ptr::write_unaligned((s + club_record::FEE_PAID) as *mut i32, value);
        exe::fill_staff_name_ref(person, s + club_record::FEE_PAID_HOLDER);
        core::ptr::write_unaligned((s + club_record::FEE_PAID_OTHER_CLUB_ID) as *mut usize, read_usize(seller));
        copy(s + club_record::FEE_PAID_DATE, rascunho.as_mut_ptr(), G_CURRENT_DATE);
        *((s + club_record::FEE_PAID_FLAG) as *mut u8) = (read_usize(t + offer::EXCHANGE_PLAYERS) != 0) as u8;
        if read_i32(t + offer::FEE) > read_i32(a + club_record::FEE_PAID) {
            copy_transfer_record(a, s, 0x313);
            if is_big_transfer(read_i32(a + club_record::FEE_PAID), buyer) {
                crate::comparison::branch("cpp_10 transferencia grande");
                if !record_news_queued(this, read_usize(buyer), 0x1c) {
                    news(this, read_usize(buyer) as u32, 0x1c);
                }
                exe::staff_add_favoured_staff(read_usize(buyer + club::CHAIRMAN), person, 1); // forced
            }
        }
    }
    let s = exe::club_records_get_season_record(this, seller, 1);
    let a = exe::club_records_get_all_time_record(this, seller, 1);
    let value = read_i32(t + offer::FEE);
    if value == 0 || value <= read_i32(s + club_record::FEE_RECEIVED) {
        return 1;
    }
    core::ptr::write_unaligned((s + club_record::FEE_RECEIVED) as *mut i32, value);
    exe::fill_staff_name_ref(person, s + club_record::FEE_RECEIVED_HOLDER);
    core::ptr::write_unaligned((s + club_record::FEE_RECEIVED_OTHER_CLUB_ID) as *mut usize, read_usize(buyer));
    copy(s + club_record::FEE_RECEIVED_DATE, rascunho.as_mut_ptr(), G_CURRENT_DATE);
    *((s + club_record::FEE_RECEIVED_FLAG) as *mut u8) = (read_usize(t + offer::EXCHANGE_PLAYERS) != 0) as u8;
    if read_i32(t + offer::FEE) > read_i32(a + club_record::FEE_RECEIVED) {
        copy_transfer_record(a, s, 0x33c);
        if is_big_transfer(read_i32(a + club_record::FEE_RECEIVED), seller) && !record_news_queued(this, read_usize(seller), 0x1d) {
            news(this, read_usize(seller) as u32, 0x1d);
        }
    }
    1
}

/// Record news (club id, type) is already in the 6-byte vector [this+0x24] (count +0x2c).
///
/// [pt] A notícia de recorde (id do clube, tipo) já está no vetor de 6 bytes [this+0x24] (conta +0x2c)?
pub(crate) unsafe fn is_record_news_queued(this: usize, id: usize, kind: u8) -> bool {
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32;
    let v = read_usize(this + club_records_fields::NEWS_QUEUE);
    (0..n.max(0) as usize).any(|k| read_usize(v + 6 * k) == id && *((v + 6 * k + 4) as *const u8) == kind)
}

/// `Club_Records::add_transfer_totals(fee, buyer, seller)`: adds the fee to the seller's season income and the buyer's
/// season spending, raises the all-time records and queues the record news (0x1f, 0x1e) above (+0x80 + 500) × 1000.
///
/// The seller adds `fee` to the income for the season (+0x36b of the season record 0x4528d0; −1 = nothing yet: the fee
/// is stored) and zeroes the year (+0x36f); above the all-time record (0x452710, same field), the record becomes the
/// season one with the club's season year (0x669950), and with the total at least (club +0x80 + 500) × 1000 and the
/// year different from the record's, the record news item 0x1f (0x452cb0) if it is not in the queue yet. The buyer does
/// the same for spending (+0x365, year +0x369, news 0x1e), but the minimum is checked against the OLD record and
/// without the year. The records are created when missing (second argument 1). A null buyer or seller: nothing, 1.
/// Returns 1.
/// Original: `Club_Records::add_transfer_totals` (0x448ad0, club_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x448ad0 } (thiscall, ret 0xc, club_records.cpp): `Club_Records::add_transfer_totals(valor, comprador,
/// vendedor)`: o vendedor soma o `valor` ao recebido na temporada (+0x36b do registro da temporada 0x4528d0; −1 =
/// ainda nada: fica o valor) e zera o ano (+0x36f); acima do recorde de sempre (0x452710, mesmo campo), o recorde passa
/// a ser o da temporada com o ano da temporada do clube (0x669950), e com o total no mínimo (+0x80 do clube + 500) ×
/// 1000 e o ano diferente do recorde, a notícia de recorde 0x1f (0x452cb0) se ainda não está na fila. O comprador, o
/// mesmo pelo gasto (+0x365, ano +0x369, notícia 0x1e), mas o mínimo é conferido com o recorde ANTIGO e sem o ano. Os
/// registros são criados se faltarem (o segundo argumento 1). Comprador ou vendedor nulo: nada, 1. Devolve 1.
pub(crate) unsafe extern "thiscall" fn add_transfer_totals(this: usize, value: i32, buyer: usize, seller: usize) -> u32 {
    if buyer == 0 || seller == 0 {
        return 1;
    }
    let year_of_club: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_GET_CLUB_SEASON_YEAR);
    let news: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_QUEUE_RECORD_NEWS);
    let add = |field: usize| {
        let v = read_usize(field) as i32;
        write_usize(field, if v == -1 { value } else { v.wrapping_add(value) } as usize);
    };
    // the seller: the amount received
    let t = exe::club_records_get_season_record(this, seller, 1);
    let s = exe::club_records_get_all_time_record(this, seller, 1);
    add(t + club_record::TRANSFER_INCOME);
    write_i16(t + club_record::TRANSFER_INCOME_YEAR, 0);
    let year = year_of_club(seller) as i16;
    if (read_usize(s + club_record::TRANSFER_INCOME) as i32) < read_usize(t + club_record::TRANSFER_INCOME) as i32 {
        let minimum = (read_i16(seller + club::REPUTATION) as i32 + 500).wrapping_mul(1000);
        if read_usize(t + club_record::TRANSFER_INCOME) as i32 >= minimum && year != read_i16(s + club_record::TRANSFER_INCOME_YEAR) && !is_record_news_queued(this, read_usize(seller), 0x1f) {
            crate::comparison::branch("cpm_6 noticia de recorde recebido");
            news(this, read_usize(seller) as u32, 0x1f); // record fee received
        }
        write_usize(s + club_record::TRANSFER_INCOME, read_usize(t + club_record::TRANSFER_INCOME));
        write_i16(s + club_record::TRANSFER_INCOME_YEAR, year);
    }
    // the buyer: the amount spent
    let t = exe::club_records_get_season_record(this, buyer, 1);
    let s = exe::club_records_get_all_time_record(this, buyer, 1);
    add(t + club_record::TRANSFER_SPENDING);
    write_i16(t + club_record::TRANSFER_SPENDING_YEAR, 0);
    let year = year_of_club(buyer) as i16;
    if (read_usize(t + club_record::TRANSFER_SPENDING) as i32) > read_usize(s + club_record::TRANSFER_SPENDING) as i32 {
        let minimum = (read_i16(buyer + club::REPUTATION) as i32 + 500).wrapping_mul(1000);
        // the original checks the minimum against the old record (the ecx that still holds the all-time one)
        if read_usize(s + club_record::TRANSFER_SPENDING) as i32 >= minimum && !is_record_news_queued(this, read_usize(buyer), 0x1e) {
            crate::comparison::branch("cpm_6 noticia de recorde pago");
            news(this, read_usize(buyer) as u32, 0x1e); // record fee paid
        }
        write_usize(s + club_record::TRANSFER_SPENDING, read_usize(t + club_record::TRANSFER_SPENDING));
        write_i16(s + club_record::TRANSFER_SPENDING_YEAR, year);
    }
    1
}

/// `club_records_update_career_records(player, club, games, goals)`: updates the club's all-time league appearances
/// (+0x247) and goals (+0x261) records, with the player reference (0x7e4920), when the record type applies to the club
/// (0x449590).
///
/// In the club's all-time records (0x452710, created when missing), if type 0x17 applies to the club (0x449590) and
/// `games` beats the record (+0x247; −1 = no record), the record = `games` and the player's reference goes to +0x249
/// (0x7e4920); the same with type 0x18 and `goals` (+0x261, reference +0x263). Returns 1. A null player or club, or no
/// record: the "Error" box (lines 0x6fa and 0x702) and 0.
/// Original: `club_records_update_career_records` (0x448cb0, club_records.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x448cb0 } (thiscall, ret 0x10, club_records.cpp): `club_records_update_career_records(jogador, clube, jogos,
/// gols)`: nos recordes de sempre do clube (0x452710, criados se faltarem), se o tipo 0x17 vale para o clube (0x449590)
/// e os `jogos` passam o recorde (+0x247; −1 = sem recorde), o recorde = `jogos` e a referência do jogador em +0x249
/// (0x7e4920); o mesmo com o tipo 0x18 e os `gols` (+0x261, referência +0x263). Devolve 1. Jogador ou clube nulo, ou
/// sem registro: a caixa "Error" (linhas 0x6fa e 0x702) e 0.
pub(crate) unsafe extern "thiscall" fn club_records_update_career_records(this: usize, player: usize, club: usize, games: u32, goals: u32) -> u32 {
    if player == 0 || club == 0 {
        show_error_at(0x98b6a0, 0x6fa);
        return 0;
    }
    let r = exe::club_records_get_all_time_record(this, club, 1);
    if r == 0 {
        show_error_at(0x98b6a0, 0x702);
        return 0;
    }
    for (tipo, campo, n) in [(0x17u32, 0x247usize, games as i16), (0x18, 0x261, goals as i16)] {
        if exe::club_records_record_type_applies(tipo, club, 1) != 0 {
            let v = read_i16(r + campo);
            if v == -1 || n > v {
                write_i16(r + campo, n);
                exe::fill_staff_name_ref(player, r + campo + 2);
            }
        }
    }
    1
}

/// `Club_Records::update_season_rating_record(club)`: the squad player with the best average rating (statistic 0x11)
/// among those who played at least half the games becomes the season rating record.
///
/// With games in the season (byte +0x10 of the club's entry, [this+0x1c] + id × 0x6a) and the season record (0x4528d0,
/// without creating), the record rating (+0x1f1) goes back to −1.0 and, over the 50 squad slots (+0xd7), the player
/// (not a human manager: id < [0xae2364]) with statistics (+0x61) at the club (+0x39 = the club or the one linked to
/// it, 0x540a50) and a season statistics record (0x7c3920, type 6) who played at least half the games (statistic 1,
/// truncated, 0x7c2150 with 6.5) and has the highest average (statistic 0x11) becomes the record: the rating (+0x1f1,
/// float), the games (+0x20d), the reference (+0x1f5, 0x7e4920) and +0x20e = 0. Returns 1 if someone became the record.
/// Without games or without a record: 0. A null club: the original routine (box 0x16f2, 0).
/// Original: `Club_Records::update_season_rating_record` (0x452a90, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x452a90 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_season_rating_record(clube)`: com jogos
/// na temporada (o byte +0x10 da entrada do clube, [this+0x1c] + id × 0x6a) e o registro da temporada (0x4528d0, sem
/// criar), a nota do recorde (+0x1f1) volta a −1.0 e, pelas 50 vagas do elenco (+0xd7), o jogador (não técnico humano:
/// id < [0xae2364]) com estatísticas (+0x61) no clube (+0x39 = clube ou o ligado a ele, 0x540a50) e registro de
/// estatísticas da temporada (0x7c3920, tipo 6) que jogou ao menos metade dos jogos (estatística 1, truncada, 0x7c2150 com
/// 6.5) e tem a maior média (estatística 0x11) vira o recorde: a nota (+0x1f1, float), os jogos (+0x20d), a referência
/// (+0x1f5, 0x7e4920) e +0x20e = 0. Devolve 1 se alguém virou recorde. Sem jogos ou sem registro: 0. Clube nulo: a
/// original (caixa 0x16f2, 0).
pub(crate) unsafe extern "thiscall" fn update_season_rating_record(this: usize, club: usize) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_SEASON_RATING_RECORD)(this, club); // message box: original only
    }
    let entry = read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add(read_usize(club).wrapping_mul(0x6a));
    if *((entry + club_record_entry::SEASON_GAMES) as *const u8) == 0 {
        return 0;
    }
    let r = exe::club_records_get_season_record(this, club, 0);
    if r == 0 {
        return 0;
    }
    let ligado: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let estatistica: unsafe extern "stdcall" fn(usize, u32, f32) -> f64 = core::mem::transmute(FN_PLAYER_STATS_GET_RECORD_VALUE);
    let rating = (r + club_record::SEASON_RATING) as *mut f32;
    write_usize(r + club_record::SEASON_RATING, 0xbf800000); // −1.0f
    let mut found = 0;
    for k in 0..50usize {
        let p = read_usize(club + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        let est = read_usize(p + staff::PLAYER);
        if est == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        if read_usize(p + staff::CLUB_JOB) != club && read_usize(p + staff::CLUB_JOB) != ligado(club, 0, 1) {
            continue;
        }
        let reg = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), est, 6);
        if reg == 0 {
            continue;
        }
        let jogou = ftol(estatistica(reg, 1, 6.5)) as u8;
        if (jogou as i32) < *((entry + club_record_entry::SEASON_GAMES) as *const i8) as i32 / 2 { // fewer than half the matches
            continue;
        }
        let average = estatistica(reg, 0x11, 6.5);
        // fcomp with −1.0 (double) and fcom with the stored float: passes only with the higher average
        if *rating as f64 == read_f64_at(0x968948) || average > *rating as f64 { // 0x968948 = -1
            crate::comparison::branch("cpm_6 recorde de nota da temporada");
            *rating = average as f32;
            *((r + club_record::SEASON_RATING_GAMES) as *mut u8) = jogou;
            exe::fill_staff_name_ref(p, r + 0x1f5);
            write_i16(r + club_record::SEASON_RATING_YEAR, 0);
            found = 1;
        }
    }
    found
}

/// Club's 0x6a-byte entry in [this+0x1c] (+0 all-time index, +4 season index, -1 none).
///
/// [pt] A entrada do clube (0x6a bytes, pelo id +0 do clube) na tabela [this+0x1c]: +0 o índice dos recordes de sempre,
/// +4 o dos da temporada (−1 = ainda sem).
pub(crate) unsafe fn club_records_entry(this: usize, club: usize) -> usize {
    read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add(read_usize(club).wrapping_mul(0x6a))
}

/// club_records.cpp out-of-memory: shows the box at `linha` and exits fatally.
///
/// [pt] A falta de memória das tabelas dos recordes (club_records.cpp, linha `linha`): caixa e fim, como a original.
pub(crate) unsafe fn records_out_of_memory(row: u32) -> ! {
    show_error_at(0x98b6a0, row);
    fatal_exit()
}

/// `Club_Records::get_all_time_record(club, create)`: returns the club's all-time records: index +0 of its entry
/// ([this+0x1c] + id × 0x6a) into the vector [this+4] of 0x38b-byte records.
///
/// Without an index (−1) and with `create`: the new index is the count (+0xc, which goes up), the vector grows by 50 at
/// a time (capacity +8, realloc 0x945667; without memory, box 0x1694 and the end) and the record is reset (0x452fe0).
/// Returns the record, or 0 without an index. A null club: the original routine (box 0x1680, 0).
/// Original: `Club_Records::get_all_time_record` (0x452710, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x452710 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_all_time_record(clube, criar)`: os recordes
/// de sempre do clube: o índice +0 da entrada dele ([this+0x1c] + id × 0x6a) no vetor [this+4] de 0x38b bytes. Sem
/// índice (−1) e com `criar`: o índice novo é a conta (+0xc, que sobe), o vetor cresce de 50 em 50 (capacidade +8,
/// realloc 0x945667; sem memória, caixa 0x1694 e fim) e o registro é reiniciado (0x452fe0). Devolve o registro, ou 0
/// sem índice. Clube nulo: a original (caixa 0x1680, 0).
pub(crate) unsafe extern "thiscall" fn get_all_time_record(this: usize, club: usize, create: u32) -> usize {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_ALL_TIME_RECORD)(this, club, create);
    }
    let e = club_records_entry(this, club);
    if read_usize(e) as i32 == -1 && create != 0 {
        crate::comparison::branch("cpk_1 recordes de sempre novos");
        if read_usize(this + club_records_fields::ALL_TIME_COUNT) == read_usize(this + club_records_fields::ALL_TIME_CAPACITY) {
            let cap = read_usize(this + club_records_fields::ALL_TIME_CAPACITY).wrapping_add(50);
            write_usize(this + club_records_fields::ALL_TIME_CAPACITY, cap);
            let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            let p = realloc(read_usize(this + club_records_fields::ALL_TIME_RECORDS), cap.wrapping_mul(0x38b) as u32);
            write_usize(this + club_records_fields::ALL_TIME_RECORDS, p);
            if p == 0 {
                records_out_of_memory(0x1694);
            }
        }
        let n = read_usize(this + club_records_fields::ALL_TIME_COUNT);
        write_usize(e, n);
        write_usize(this + club_records_fields::ALL_TIME_COUNT, n.wrapping_add(1)); // the all-time records calculation
        exe::season_history_entry_clear(read_usize(this + club_records_fields::ALL_TIME_RECORDS).wrapping_add(read_usize(e).wrapping_mul(0x38b)));
    }
    let i = read_usize(e) as i32;
    if i <= -1 {
        return 0;
    }
    read_usize(this + club_records_fields::ALL_TIME_RECORDS).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// `Club_Records::get_season_record(club, create)`: the same as `get_all_time_record` for the season records: index +4
/// of the entry, vector [this+0x10], capacity +0x14, count +0x18 (without memory, box 0x16c9 and the end). A null club:
/// the original routine (box 0x16b5, 0).
/// Original: `Club_Records::get_season_record` (0x4528d0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x4528d0 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_season_record(clube, criar)`: o mesmo para os
/// recordes da temporada: índice +4 da entrada, vetor [this+0x10], capacidade +0x14, conta +0x18 (sem memória, caixa
/// 0x16c9 e fim). Clube nulo: a original (caixa 0x16b5, 0).
pub(crate) unsafe extern "thiscall" fn get_season_record(this: usize, club: usize, create: u32) -> usize {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_SEASON_RECORD)(this, club, create);
    }
    let e = club_records_entry(this, club) + 4;
    if read_usize(e) as i32 == -1 && create != 0 {
        crate::comparison::branch("cpk_1 recordes da temporada novos");
        if read_usize(this + club_records_fields::SEASON_COUNT) == read_usize(this + club_records_fields::SEASON_CAPACITY) {
            let cap = read_usize(this + club_records_fields::SEASON_CAPACITY).wrapping_add(50);
            write_usize(this + club_records_fields::SEASON_CAPACITY, cap);
            let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            let p = realloc(read_usize(this + club_records_fields::SEASON_RECORDS), cap.wrapping_mul(0x38b) as u32);
            write_usize(this + club_records_fields::SEASON_RECORDS, p);
            if p == 0 {
                records_out_of_memory(0x16c9);
            }
        }
        let n = read_usize(this + club_records_fields::SEASON_COUNT);
        write_usize(e, n);
        write_usize(this + club_records_fields::SEASON_COUNT, n.wrapping_add(1)); // the season records count
        exe::season_history_entry_clear(read_usize(this + club_records_fields::SEASON_RECORDS).wrapping_add(read_usize(e).wrapping_mul(0x38b)));
    }
    let i = read_usize(e) as i32;
    if i <= -1 {
        return 0;
    }
    read_usize(this + club_records_fields::SEASON_RECORDS).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// `Club_Records::queue_record_news(id, type)`: appends (club id at +0, record type at +4; byte +5 is left as it is) to
/// the 6-byte vector [this+0x24] of record news, read by 0x448e30.
///
/// When full (count +0x2c = capacity +0x28) it grows by 5 at a time (realloc 0x945667); without memory, box 0x174f and
/// returns 0 (the game goes on, without the vector). Returns 1.
/// Original: `Club_Records::queue_record_news` (0x452cb0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x452cb0 } (thiscall, ret 8, club_records.cpp): `Club_Records::queue_record_news(id, tipo)`: acrescenta (id do
/// clube +0, tipo do recorde +4; o byte +5 fica como está) ao vetor de 6 bytes [this+0x24] das notícias de recorde,
/// lidas pela 0x448e30. Cheio (conta +0x2c = capacidade +0x28): cresce de 5 em 5 (realloc 0x945667); sem memória,
/// caixa 0x174f e devolve 0 (o jogo segue, sem o vetor). Devolve 1.
pub(crate) unsafe extern "thiscall" fn queue_record_news(this: usize, id: u32, kind: u32) -> u32 {
    crate::comparison::branch("cpk_1 noticia de recorde");
    if read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) == read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY) {
        let cap = read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY).wrapping_add(5);
        write_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY, cap);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_usize(this + club_records_fields::NEWS_QUEUE), cap.wrapping_mul(6) as u32);
        write_usize(this + club_records_fields::NEWS_QUEUE, p);
        if p == 0 {
            show_error_at(0x98b6a0, 0x174f);
            return 0;
        }
    }
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT);
    let item = read_usize(this + club_records_fields::NEWS_QUEUE).wrapping_add(n.wrapping_mul(6));
    core::ptr::write_unaligned(item as *mut u32, id);
    *((item + record_news_item::RECORD_TYPE) as *mut u8) = kind as u8;
    write_usize(this + club_records_fields::NEWS_QUEUE_COUNT, n.wrapping_add(1)); // one more news item
    1
}

/// Plan before the original: expected index, expected count and whether a new record is created.
///
/// [pt] O plano dos recordes (de sempre ou da temporada) antes da original: o índice esperado (o novo é a conta), a conta
/// esperada e se o registro é novo.
pub(crate) unsafe fn record_index_plan(e: usize, count: usize, create: u32) -> (i32, usize, bool) {
    let i = read_usize(e) as i32;
    let n = read_usize(count);
    if i == -1 && create != 0 { (n as i32, n.wrapping_add(1), true) } else { (i, n, false) }
}

/// Checks the original's result against the plan (index, count, returned record, reset markers of a new one) and
/// records it in PLACAR.
///
/// [pt] Confere o resultado da original com o plano: o índice da entrada, a conta, o registro devolvido e, se novo, as
/// marcas da 0x452fe0 (−1.0f em +0x1f1, −1 em +0x313).
pub(crate) unsafe fn check_record_plan(name: &str, club: usize, e: usize, count: usize, array: usize, plan: (i32, usize, bool), r: usize) {
    let (i, n, novo) = plan;
    let expected = if i <= -1 { 0 } else { read_usize(array).wrapping_add((i as usize).wrapping_mul(0x38b)) };
    let reiniciado = !novo || (r != 0 && read_usize(r + club_record::SEASON_RATING) == 0xbf800000 && read_usize(r + club_record::FEE_PAID) as i32 == -1);
    SCOREBOARD.record(r == expected && read_usize(e) as i32 == i && read_usize(count) == n && reiniciado, || {
        format!("{}(clube {}) índice {} (esperava {}), conta {} (esperava {}), registro {:x} (esperava {:x}), novo={} reiniciado={}",
            name, read_usize(club), read_usize(e) as i32, i, read_usize(count), n, r, expected, novo, reiniciado)
    });
}

/// `Club_Records::offer_breaks_fee_record(offer, require)`: tells whether the offer fee (+0x1b) beats the fee-paid
/// record of the buying club (+8; the all-time records through 0x452710, creating them if needed).
///
/// It must be at least 50 × reputation (+0x80) + 500,000; with `require`, also at least reputation² × the constant
/// 0x967880 (_ftol); and above zero and above the record (+0x313). A null offer: 0.
/// Original: `Club_Records::offer_breaks_fee_record` (0x4486c0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x4486c0 } (thiscall, ret 8, club_records.cpp): `Club_Records::offer_breaks_fee_record(oferta, exigir)`: o valor
/// da oferta (+0x1b) bate o recorde de valor pago do clube comprador (+8; os recordes de sempre pela 0x452710, criando
/// se preciso)? Tem de ser ao menos 50 × reputação (+0x80) + 500.000; com `exigir`, também ao menos reputação² × a
/// constante 0x967880 (_ftol); e acima de zero e do recorde (+0x313). Oferta nula: 0.
pub(crate) unsafe extern "thiscall" fn offer_breaks_fee_record(this: usize, offer: usize, exigir: u32) -> u8 {
    if offer == 0 {
        return 0;
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + offer::BUYER_CLUB_ID).wrapping_mul(club::SIZE));
    let rec = exe::club_records_get_all_time_record(this, club, 1);
    if rec == 0 || club == 0 {
        return 0;
    }
    let rep = *((club + club::REPUTATION) as *const i16) as i32;
    let value = read_usize(offer + offer::FEE) as i32;
    if value < rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {
        return 0; // below the minimum for the reputation
    }
    if exigir != 0 && value < ftol(rep as f64 * rep as f64 * read_f64_at(0x967880)) { // 0x967880 = 0.1
        return 0;
    }
    (value != 0 && value > read_usize(rec + club_record::FEE_PAID) as i32) as u8
}

/// `transfer_fee_record_construct`: constructs the highest transfer fee paid record: the dates at +0x14 and +0x20
/// (0x54a110).
///
/// The original's SEH frame only covers the 2nd date (0x54a110 does not fail). Returns `this`.
/// Original: `transfer_fee_record_construct` (0x4485d0, club_records.cpp, thiscall).
///
/// [pt] { 0x4485d0 } (thiscall, club_records.cpp): `transfer_fee_record_construct`: constrói o recorde de maior valor de
/// transferência pago: as datas em +0x14 e +0x20 (0x54a110). O quadro SEH do original só cobre a 2ª data (a 0x54a110
/// não falha). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn transfer_fee_record_construct(this: usize) -> usize {
    init_default_date(this + transfer_fee_record::HOLDER_DATE); // the date of the amount paid
    init_default_date(this + transfer_fee_record::DATE);
    this
}

/// constructs the highest fee received record: the dates at +0x14 and +0x20 (0x54a110)
/// Original: `transfer_fee_record_construct_2` (0x448620, club_records.cpp, thiscall).
///
/// [pt] { 0x448620 } (thiscall, club_records.cpp): `transfer_fee_record_construct_2`: constrói o recorde de maior valor de
/// transferência recebido: as datas em +0x14 e +0x20 (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn transfer_fee_record_construct_2(this: usize) -> usize {
    init_default_date(this + transfer_fee_record::HOLDER_DATE);
    init_default_date(this + transfer_fee_record::DATE); // the 2nd date of the amount received
    this
}

/// constructs the run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `run_record_construct` (0x448670, club_records.cpp, thiscall).
///
/// [pt] { 0x448670 } (thiscall, club_records.cpp): `run_record_construct`: constrói o recorde de sequência: as datas do
/// começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE); // the start of the sequence
    init_default_date(this + run_record::END_DATE);
    this
}

/// constructs the won run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `won_run_record_construct` (0x450ad0, club_records.cpp, thiscall).
///
/// [pt] { 0x450ad0 } (thiscall, club_records.cpp): `won_run_record_construct`: constrói o recorde de vitórias seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
pub(crate) unsafe extern "thiscall" fn won_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE); // the start of the winning run
    init_default_date(this + run_record::END_DATE);
    this
}

/// constructs the lost run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `lost_run_record_construct` (0x450b20, club_records.cpp, thiscall).
///
/// [pt] { 0x450b20 } (thiscall, club_records.cpp): `lost_run_record_construct`: constrói o recorde de derrotas seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn lost_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE);
    init_default_date(this + run_record::END_DATE); // the end of the losing run
    this
}

/// constructs the unbeaten run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `unbeaten_run_record_construct` (0x450b70, club_records.cpp, thiscall).
///
/// [pt] { 0x450b70 } (thiscall, club_records.cpp): `unbeaten_run_record_construct`: constrói o recorde de jogos sem perder:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn unbeaten_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE); // the start of the unbeaten run
    init_default_date(this + run_record::END_DATE);
    this
}

/// constructs the winless run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `winless_run_record_construct` (0x450bc0, club_records.cpp, thiscall).
///
/// [pt] { 0x450bc0 } (thiscall, club_records.cpp): `winless_run_record_construct`: constrói o recorde de jogos sem vencer:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
pub(crate) unsafe extern "thiscall" fn winless_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE);
    init_default_date(this + run_record::END_DATE); // the end of the winless run
    this
}

/// constructs the goals in a match record: the holder date (+0x11) and the match date (+0x19) (0x54a110)
/// Original: `goals_in_match_record_construct` (0x450c10, club_records.cpp, thiscall).
///
/// [pt] { 0x450c10 } (thiscall, club_records.cpp): `goals_in_match_record_construct`: constrói o recorde de gols numa
/// partida: a data do recordista (+0x11) e a da partida (+0x19) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn goals_in_match_record_construct(this: usize) -> usize {
    init_default_date(this + goals_in_match_record::HOLDER_DATE);
    init_default_date(this + goals_in_match_record::MATCH); // the match date
    this
}

/// constructs the youngest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110)
/// Original: `youngest_player_record_construct` (0x450c70, club_records.cpp, thiscall).
///
/// [pt] { 0x450c70 } (thiscall, club_records.cpp): `youngest_player_record_construct`: constrói o recorde do jogador mais
/// novo: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn youngest_player_record_construct(this: usize) -> usize {
    init_default_date(this); // the age of the youngest
    init_default_date(this + age_record::HOLDER_DATE);
    init_default_date(this + age_record::MATCH_DATE);
    this
}

/// `oldest_player_record_construct`: constructs the oldest player record: the age (+0), the record holder's date
/// (+0x18) and the match date (+0x20) (0x54a110).
///
/// The original's SEH frame is not needed (0x54a110 does not fail). Returns `this`.
/// Original: `oldest_player_record_construct` (0x450cd0, club_records.cpp, thiscall).
///
/// [pt] { 0x450cd0 } (thiscall, club_records.cpp): `oldest_player_record_construct`: constrói o recorde do jogador mais
/// velho: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). O quadro SEH do original não
/// é preciso (a 0x54a110 não falha). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn oldest_player_record_construct(this: usize) -> usize {
    init_default_date(this);
    init_default_date(this + age_record::HOLDER_DATE); // the date of the oldest
    init_default_date(this + age_record::MATCH_DATE);
    this
}

/// Copies the 0x18-byte record holder from `origem` to `destino` (record_holder_assign 0x448490).
///
/// [pt] O recordista (0x18 bytes) de `origem` em `destino`, pela 0x448490 (`record_holder_assign`, já em Rust).
pub(crate) unsafe fn assign_record_holder(target: usize, source: usize) {
    exe::record_holder_assign(target, source);
}

/// copies the rating record (0x1f bytes): the rating (+0), the holder (+4, through 0x448490), +0x1c and +0x1d
/// Original: `rating_record_assign` (0x448230, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448230 } (thiscall, ret 4, club_records.cpp): `rating_record_assign`: copia o recorde de nota (0x1f bytes) de
/// `origem`: a nota (+0, dupla), o recordista em +4, o byte +0x1c e a palavra +0x1d. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn rating_record_assign(this: usize, source: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(source as *const u32)); // a nota
    assign_record_holder(this + rating_record::HOLDER, source + rating_record::HOLDER);
    *((this + rating_record::GAMES) as *mut u8) = *((source + rating_record::GAMES) as *const u8); // the rating record byte
    core::ptr::write_unaligned((this + rating_record::YEAR) as *mut u16, core::ptr::read_unaligned((source + rating_record::YEAR) as *const u16));
    this
}

/// copies the man of the match record (0x1b bytes): the count (+0), the holder (+1) and the year (+0x19)
/// Original: `man_of_match_record_assign` (0x4484d0, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x4484d0 } (thiscall, ret 4, club_records.cpp): `man_of_match_record_assign`: copia o recorde de melhor em campo
/// (0x1b bytes) de `origem`: as vezes (+0, byte), o recordista em +1 e o ano da temporada (+0x19). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn man_of_match_record_assign(this: usize, source: usize) -> usize {
    *(this as *mut u8) = *(source as *const u8); // the man-of-the-match count
    assign_record_holder(this + man_of_match_record::HOLDER, source + man_of_match_record::HOLDER);
    core::ptr::write_unaligned((this + man_of_match_record::YEAR) as *mut u16, core::ptr::read_unaligned((source + man_of_match_record::YEAR) as *const u16));
    this
}

/// copies the discipline record (0x1c bytes): yellows (+0), reds (+1), the holder (+2) and the year (+0x1a)
/// Original: `discipline_record_assign` (0x448500, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448500 } (thiscall, ret 4, club_records.cpp): `discipline_record_assign`: copia o recorde de disciplina (0x1c
/// bytes) de `origem`: os amarelos (+0) e vermelhos (+1), o recordista em +2 e o ano da temporada (+0x1a). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn discipline_record_assign(this: usize, source: usize) -> usize {
    *(this as *mut u8) = *(source as *const u8);
    *((this + club_discipline_record::RED_CARDS) as *mut u8) = *((source + club_discipline_record::RED_CARDS) as *const u8); // the reds
    assign_record_holder(this + club_discipline_record::HOLDER, source + club_discipline_record::HOLDER);
    core::ptr::write_unaligned((this + club_discipline_record::YEAR) as *mut u16, core::ptr::read_unaligned((source + club_discipline_record::YEAR) as *const u16));
    this
}

/// copies the count record (0x1a bytes): the count (+0, word) and the holder (+2, through 0x448490)
/// Original: `count_record_assign` (0x448530, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448530 } (thiscall, ret 4, club_records.cpp): `count_record_assign`: copia o recorde de contagem (0x1a bytes) de
/// `origem`: a contagem (+0, palavra) e o recordista em +2 (pela 0x448490, já em Rust). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn count_record_assign(this: usize, source: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u16, core::ptr::read_unaligned(source as *const u16)); // a contagem
    exe::record_holder_assign(this + count_record::HOLDER, source + count_record::HOLDER);
    this
}

/// copies the fans player record (0x1a bytes): the holder (+0) and the year (+0x18)
/// Original: `fans_player_record_assign` (0x4485b0, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x4485b0 } (thiscall, ret 4, club_records.cpp): `fans_player_record_assign`: copia o recorde de jogador da
/// torcida (0x1a bytes) de `origem`: o recordista em +0 (pela 0x448490) e o ano da temporada (+0x18). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn fans_player_record_assign(this: usize, source: usize) -> usize {
    exe::record_holder_assign(this, source);
    core::ptr::write_unaligned((this + fans_player_record::YEAR) as *mut u16, core::ptr::read_unaligned((source + fans_player_record::YEAR) as *const u16)); // the supporter’s year
    this
}

/// Constructs the default record date at `a` (0x54a110).
///
/// [pt] A data do recordista em `a` pela 0x54a110 (o construtor da data: dia 40, ano 1972, +4 = 1).
pub(crate) unsafe fn init_default_date(a: usize) {
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    data_default(a);
}

/// constructs the tally record: the holder date at +0x11 (0x54a110)
/// Original: `tally_record_construct` (0x450c60, club_records.cpp, thiscall).
///
/// [pt] { 0x450c60 } (thiscall, club_records.cpp): `tally_record_construct`: constrói o recorde de ocorrências (contagem em
/// byte + recordista): só a data do recordista em +0x11 (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn tally_record_construct(this: usize) -> usize {
    init_default_date(this + tally_record::HOLDER_DATE); // the date of the occurrences
    this
}

/// constructs the count record: the holder date at +0x12 (0x54a110)
/// Original: `count_record_construct` (0x450d30, club_records.cpp, thiscall).
///
/// [pt] { 0x450d30 } (thiscall, club_records.cpp): `count_record_construct`: constrói o recorde de contagem (palavra +
/// recordista): só a data do recordista em +0x12 (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn count_record_construct(this: usize) -> usize {
    init_default_date(this + count_record::HOLDER_DATE); // the counting date
    this
}

/// `transfer_fee_record_assign`: copies the transfer fee record (0x29 bytes) from `source`: five dwords (+0..+0x10: the
/// fee, the player, the other club...), the date at +0x14, the dword +0x1c, the date at +0x20 and the part-exchange
/// flag (+0x28). Returns `this`.
/// Original: `transfer_fee_record_assign` (0x448550, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448550 } (thiscall, ret 4, club_records.cpp): `transfer_fee_record_assign`: copia o recorde de valor de
/// transferência (0x29 bytes) de `origem`: cinco duplas (+0..+0x10: o valor, o jogador, o outro clube...), a data em
/// +0x14, a dupla +0x1c, a data em +0x20 e a marca de troca (+0x28). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn transfer_fee_record_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((source + o) as *const u32));
    for o in [0, 4, 8, 0xc, 0x10] {
        d(o); // the value and the ids
    }
    copy_record_date(this + transfer_fee_record::HOLDER_DATE, source + transfer_fee_record::HOLDER_DATE);
    d(transfer_fee_record::OTHER_CLUB_ID);
    copy_record_date(this + transfer_fee_record::DATE, source + transfer_fee_record::DATE);
    *((this + transfer_fee_record::FLAG) as *mut u8) = *((source + transfer_fee_record::FLAG) as *const u8); // the swap flag
    this
}

/// `goals_in_match_record_assign`: copies the goals-in-a-match record (0x38 bytes) from `source`: the goals (+0, byte),
/// the record holder at +1 (0x448490) and the match record at +0x19 (0x448110, both already in Rust). Returns `this`.
/// Original: `goals_in_match_record_assign` (0x451570, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x451570 } (thiscall, ret 4, club_records.cpp): `goals_in_match_record_assign`: copia o recorde de gols numa
/// partida (0x38 bytes) de `origem`: os gols (+0, byte), o recordista em +1 (0x448490) e o registro da partida em +0x19
/// (0x448110, ambos já em Rust). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn goals_in_match_record_assign(this: usize, source: usize) -> usize {
    *(this as *mut u8) = *(source as *const u8); // the goals
    exe::record_holder_assign(this + goals_in_match_record::HOLDER, source + goals_in_match_record::HOLDER);
    exe::match_record_assign(this + goals_in_match_record::MATCH, source + goals_in_match_record::MATCH);
    this
}

/// Copies a date from `origem` to `destino` (Date::operator=).
///
/// [pt] Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local.
pub(crate) unsafe fn copy_record_date(target: usize, source: usize) {
    let copy: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copy(target, &mut local, source);
}

/// copies the record holder: four identity fields and the date
/// Original: `record_holder_assign` (0x448490, club_records.cpp, thiscall).
///
/// [pt] { 0x448490 } (thiscall, ret 4, club_records.cpp): copia o recordista (0x18 bytes) de `origem`:
/// quatro palavras duplas de identificação (+0..+0xc) e a data em +0x10. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn record_holder_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((source + o) as *const u32));
    d(0); // identification
    d(4);
    d(8);
    d(0xc);
    copy_record_date(this + record_holder::DATE, source + record_holder::DATE);
    this
}

/// Copies the match record (0x1f bytes) from `source`: the date, the opponent, the score and the fixture fields.
///
/// The date at +0, then +8 (dword), +0xc and +0xe (words), +0x10 (byte), +0x11 and +0x15 (dwords), +0x19 and +0x1a
/// (bytes) and +0x1b (dword). Returns `this`.
/// Original: `match_record_assign` (0x448110, club_records.cpp, thiscall).
///
/// [pt] { 0x448110 } (thiscall, ret 4, club_records.cpp): copia o registro de partida (0x1f bytes) de
/// `origem`: a data em +0, depois +8 (dupla), +0xc e +0xe (palavras), +0x10 (byte), +0x11 e +0x15
/// (duplas), +0x19 e +0x1a (bytes) e +0x1b (dupla). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn match_record_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((source + o) as *const u32));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((source + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = *((source + o) as *const u8);
    copy_record_date(this, source);
    d(8); // opponent
    w(0xc);
    w(0xe);
    b(0x10);
    d(0x11);
    d(0x15);
    b(0x19);
    b(0x1a);
    d(0x1b);
    this
}

/// copies the run record: the game count and the start and end dates
/// Original: `run_record_assign` (0x448170, club_records.cpp, thiscall).
///
/// [pt] { 0x448170 } (thiscall, ret 4, club_records.cpp): copia o registro de sequência (0x14 bytes) de
/// `origem`: o número de jogos (+0) e as datas de início (+4) e fim (+0xc). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn run_record_assign(this: usize, source: usize) -> usize {
    *(this as *mut u32) = *(source as *const u32); // jogos
    copy_record_date(this + run_record::START_DATE, source + run_record::START_DATE);
    copy_record_date(this + run_record::END_DATE, source + run_record::END_DATE);
    this
}

/// −1.0f and −1 in the club record fields
/// Original: `reset_records` (0x452fe0, club_records.cpp, cdecl).
///
/// [pt] { 0x452fe0 } (cdecl, club_records.cpp): reinicia os recordes do clube `r`: −1.0f em +0x1f1 e −1
/// nos campos de id (dwords, palavras e bytes). Devolve 1. `r` nulo: a original (caixa de erro).
pub(crate) unsafe extern "C" fn reset_records(r: usize) -> u32 {
    if r == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_RESET_RECORDS)(r);
    }
    let d = |o: usize| core::ptr::write_unaligned((r + o) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((r + club_record::SEASON_RATING) as *mut u32, 0xbf800000);
    for o in [1, 8, 0xe, 0x31, 0x54, 0x77, 0x85, 0xa4, 0xc3, 0xe2, 0x101, 0x118, 0x12c, 0x140, 0x154] {
        d(o);
    }
    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210, 0x22b] {
        *((r + o) as *mut u8) = 0xff;
    }
    for o in [0x247, 0x261] {
        core::ptr::write_unaligned((r + o) as *mut u16, 0xffff);
    }
    d(0x2a3);
    d(0x2e2);
    core::ptr::write_unaligned((r + 0x2f9) as *mut u16, 0xffff);
    for o in [0x313, 0x33c, 0x365, 0x36b, 0x37d] {
        d(o);
    }
    1
}

/// builds the record holder: only the date at +0x10 (0x54a110)
/// Original: `record_holder_construct` (0x450d40, club_records.cpp, thiscall).
///
/// [pt] { 0x450d40 } (thiscall, club_records.cpp): `record_holder_construct`: constrói o recordista: só a data em +0x10
/// (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn record_holder_construct(this: usize) -> usize {
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    default(this + record_holder::DATE); // the record holder’s date
    this
}

