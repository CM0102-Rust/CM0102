use super::*;

/// Club record i in the club table (0x245 bytes each).
///
/// [pt] O registro do clube `i` (0x245 bytes em [0xae23b4]; a original calcula base + i + 580·i).
pub(crate) unsafe fn fix_man_club_by_index(i: usize) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add(i).wrapping_add(i.wrapping_mul(0x244)))
}

/// True if the club has a human manager (`exe::human_manager_is_human_club` with 1,1,0,0).
///
/// [pt] O clube tem técnico humano (0x600a50: push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl).
pub(crate) unsafe fn fix_man_is_human_club(club: usize) -> bool {
    exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0
}

/// Daily fixture manager update: reserve fixtures, queued fixture news, removal of AI-only friendlies and cup round
/// labels.
///
/// `FIXTURE_MANAGER::daily_update()`, from the day loop (0x5cc850); SEH frame with no destructor on the normal path
/// (the local dates). By today's weekday (`exe::date_get_weekday` on 0xae2c90):
///
/// 1. days 2 and 3: the reserve fixtures for 3 days ahead (`exe::date_add_days` + `exe::reservecomp_schedule_reserve_fixtures`: push &date; mov ecx, [0xb74434] ->
/// thiscall, ret 4) and, on day 3, also for tomorrow;
///
/// 2. the fixture news queue ([+0x1128], a list): each news item goes to the manager of the club in slot 0 if slot 0xe
/// asks for it and to the manager of the club in slot 1 if slot 0xf asks for it (`get_slot`: push k; mov ecx, news ->
/// thiscall, ret 4; `send_to_club_manager`: push club; push news; mov ecx, [0xb7431c] -> thiscall, ret 8); the list is destroyed
/// (`exe::drop_llist` + `exe::crt_operator_delete`) and [+0x1128] becomes 0;
///
/// 3. days 5 and 6, if yesterday is in the same year: in yesterday's lists (the 3 slices, the key nations up to
/// [0xb64738]), from back to front, the friendlies ([0x9cf684], type +0x3a = 1) between two clubs without a human
/// manager leave the list (`exe::llist_get_prev` + `exe::llist_remove_node`); the first fixture that is not a friendly of that type ends the list;
///
/// 4. the leg check list ([0xae2a48]): each entry whose cup (or its stage +4) has +0x43 = 1 labels the following
/// fixtures of the round (`exe::cup_label_round_extra_fixtures`: push +0xb; push +7; push round; ecx = the cup -> thiscall, ret 0xc); the list is
/// emptied (`manager_manager_clear`).
/// Original: `FIXTURE_MANAGER::daily_update` (0x5aaf00, fix_man.cpp, thiscall, ret 0).
///
/// [pt] { 0x5aaf00 } (thiscall, ret 0, fix_man.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `FIXTURE_MANAGER::daily_update()`, do laço do dia (0x5cc850). Pelo dia da semana de hoje (0x54a3f0 em 0xae2c90):
/// 1. dias 2 e 3: os jogos de reservas para daqui a 3 dias (0x54c5b0 + 0x7e6380: push &data; mov ecx, [0xb74434] ->
///    thiscall, ret 4) e, no dia 3, também para amanhã;
/// 2. a fila das notícias de jogo ([+0x1128], uma lista): cada notícia vai ao técnico do clube da vaga 0 se a vaga 0xe
///    pede e ao do clube da vaga 1 se a vaga 0xf pede (0x785830: push k; mov ecx, notícia -> thiscall, ret 4;
///    0x7861e0: push clube; push notícia; mov ecx, [0xb7431c] -> thiscall, ret 8); a lista é destruída (0x68d040 +
///    0x944c94) e [+0x1128] fica 0;
/// 3. dias 5 e 6, se ontem é do mesmo ano: nas listas de ontem (as 3 fatias, as nações-chave até [0xb64738]), de trás
///    para a frente, os amistosos ([0x9cf684], tipo +0x3a = 1) de dois clubes sem técnico humano saem da lista (0x68d1d0 +
///    0x68d280); o primeiro jogo que não é amistoso desse tipo encerra a lista;
/// 4. a lista das conferências das pernas ([0xae2a48]): cada entrada cuja copa (ou fase +4 dela) tem +0x43 = 1 rotula
///    os jogos seguintes da rodada (0x522c70: push +0xb; push +7; push rodada; ecx = a copa -> thiscall, ret 0xc); a
///    lista é esvaziada (0x68d080).
pub(crate) unsafe extern "thiscall" fn fix_man_daily_update(this: usize) {
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    let day = exe::date_get_weekday(G_CURRENT_DATE) as i16;
    if day == 2 || day == 3 {
        exe::date_add_days(G_CURRENT_DATE, pa, 3);
        exe::reservecomp_schedule_reserve_fixtures(read_usize(G_RESERVE_COMP), pa);
        if day == 3 {
            exe::date_add_days(G_CURRENT_DATE, pb, 1);
            crate::comparison::branch("cqd_6 tomorrow reserves");
            exe::reservecomp_schedule_reserve_fixtures(read_usize(G_RESERVE_COMP), pb); // cqd6: tomorrow’s reservations
        }
    }
    let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    if l != 0 {
        write_usize(l + 0xa, read_usize(l + 2));
        let mut n = exe::llist_get_next(l);
        while n != 0 {
            let ca = fix_man_club_by_index(exe::newsitem_get_slot(n, 0) as usize);
            let cb = fix_man_club_by_index(exe::newsitem_get_slot(n, 1) as usize);
            if exe::newsitem_get_slot(n, 0xe) != 0 {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, ca);
            }
            if exe::newsitem_get_slot(n, 0xf) != 0 {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, cb);
            }
            n = exe::llist_get_next(read_usize(this + fixture_manager::MOVED_NEWS_QUEUE));
        }
        let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
        if l != 0 {
            exe::drop_llist(l);
            exe::crt_operator_delete(l);
        }
        write_usize(this + fixture_manager::MOVED_NEWS_QUEUE, 0);
    }
    if day == 5 || day == 6 {
        let t = exe::date_sub_days(G_CURRENT_DATE, pa, 1);
        if read_i16(t + 2) == read_i16(G_CURRENT_DATE_YEAR) {
            let ontem = read_i16(exe::date_sub_days(G_CURRENT_DATE, pc, 1)) as u16 as usize;
            for fatia in 0..3usize {
                let lists = read_usize(this + (ontem * 3 + fatia) * 4);
                let mut k: i8 = 0;
                while k < read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8 {
                    let list = read_usize(lists + k as isize as usize * 4);
                    let mut in_the = read_usize(list + 6);
                    write_usize(list + 0xa, in_the);
                    while in_the != 0 {
                        let f = read_usize(in_the);
                        if read_usize(f) != read_usize(KEY_CLUB_COMP_RESERVE) || read_u8(f + 0x3a) != 1 {
                            break;
                        }
                        if !fix_man_is_human_club(read_usize(f + 0x1c)) && !fix_man_is_human_club(read_usize(f + 0x20)) {
                            exe::llist_get_prev(list);
                            crate::comparison::branch("cqd_6 friendly discarded");
                            exe::llist_remove_node(list, in_the); // cqd6: yesterday’s friendly leaves the list
                        } else {
                            exe::llist_get_prev(list);
                        }
                        in_the = read_usize(list + 0xa);
                    }
                    k = k.wrapping_add(1);
                }
            }
        }
    }
    if read_i16(G_FIXTURE_LEGS_LIST) != 0 {
        write_usize(G_FIXTURE_LEGS_LIST_CURSOR, read_usize(G_FIXTURE_LEGS_LIST_HEAD));
        let mut e = exe::llist_get_next(G_FIXTURE_LEGS_LIST);
        while e != 0 {
            let i = read_usize(e) as i32;
            let table = read_usize(G_COMP_OBJECT_TABLE);
            if i >= 0 && table != 0 {
                let mut cp = read_usize(table + i as usize * 4);
                if cp != 0 {
                    let stage = read_u8(e + 4) as i8;
                    if stage as i32 <= read_usize(cp + 0x30) as i32 {
                        if stage != -1 {
                            cp = read_usize(read_usize(cp + 0xc) + stage as isize as usize * 4);
                        }
                        if read_u8(cp + 0x43) == 1 {
                            // simplification: the original pushes eax with the round word and the high bytes of the pointer of the
                            // entry; 0x522c70 only reads the word
                            exe::cup_label_round_extra_fixtures(cp, read_i16(e + 5) as u16 as u32, read_usize(e + 7) as u32, read_usize(e + 0xb) as u32);
                        }
                    }
                }
            }
            e = exe::llist_get_next(G_FIXTURE_LEGS_LIST);
        }
        exe::llist_clear(G_FIXTURE_LEGS_LIST);
    }
}

/// Returns the fixture list for a year, day, slice and key nation, switching the other year block to the requested year
/// or creating it.
///
/// `FIXTURE_MANAGER::get_fixture_list(year, day, slice, type)`; SEH frame with no destructor on the normal path. The
/// list of fixtures of the year, day (word), slice (byte) and key nation (signed byte) in the chain of year blocks. By
/// the block's year (+0x1130): if the other block (+0x112c) is not of that year, it is switched to the year (`set_year`:
/// push year; ecx = the other -> thiscall, ret 4); without the other block, a new block (`exe::crt_operator_new`, 0x1184 bytes;
/// `exe::new_fixture_manager`: push 0; push this; push year; ecx = the new one -> thiscall, ret 0xc) goes into +0x112c; out of memory,
/// the box for line 0x681 and 0. The list: [[block + (day·3 + slice)·4] + type·4].
/// Original: `FIXTURE_MANAGER::get_fixture_list` (0x5abf10, fix_man.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5abf10 } (thiscall, ret 0x10, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::get_fixture_list(ano, dia, fatia, tipo)`: a lista dos jogos do ano, dia (palavra), fatia (byte) e
/// nação-chave (byte com sinal) na corrente dos blocos dos anos. Pelo bloco do ano (+0x1130): se o outro (+0x112c) não
/// é do ano, ele é trocado para o ano (0x5aa2d0: push ano; ecx = o outro -> thiscall, ret 4); sem o outro, um bloco
/// novo (0x944cf1, 0x1184 bytes; 0x5a9cf0: push 0; push this; push ano; ecx = o novo -> thiscall, ret 0xc) entra em
/// +0x112c; sem memória, a caixa da linha 0x681 e 0. A lista: [[bloco + (dia·3 + fatia)·4] + tipo·4].
pub(crate) unsafe extern "thiscall" fn get_fixture_list(this: usize, year: u32, day: u32, slice: u32, kind: u32) -> usize {
    let mut b = this;
    while read_i16(b + 0x1130) != year as i16 {
        let o = read_usize(b + 0x112c);
        if o == 0 {
            let m = exe::crt_operator_new(0x1184);
            let r = if m == 0 { 0 } else { exe::new_fixture_manager(m, year, b, 0) };
            write_usize(b + 0x112c, r);
            if r == 0 {
                show_error_at(0x9cc208, 0x681);
                return 0;
            }
        } else if read_i16(o + 0x1130) != year as i16 {
            exe::fixture_manager_set_year(o, year);
        }
        b = read_usize(b + 0x112c);
    }
    let i = (day & 0xffff) as usize * 3 + (slice & 0xff) as usize;
    read_usize(read_usize(b + i * 4) + (kind as u8 as i8) as isize as usize * 4)
}

/// Reads fixtures_<year>.tmp: each 0x4f-byte fixture gets its club, competition and stadium pointers and goes into the
/// list of its day, slice and key nation.
///
/// `FIXTURE_MANAGER::load_fixtures()`. The name `fixtures_<year>.tmp` at +0x1133 (0x944c9f) and, with a year (+0x1130,
/// unsigned) from 1980 on and the file in CM3_TEMP (`exe::utils_build_file_path`: push name; push "CM3_TEMP"; add esp, 8 -> cdecl; `exe::crt_fopen`
/// fopen with "rb"), the count (dword) and the 0x4f-byte fixtures (`exe::crt_fread` fread; on failure, the box for line 0x730
/// or 0x73e and the file is closed): the club ids (+0xc, +0x10 → +0x1c, +0x20), the competition id (+0 → +0x14, 0x6b
/// bytes in [0xae23d0]) and the stadium id (+4 → +0x18, 0x4e bytes in [0xae23b0], 0 outside [0xae2358]) become pointers
/// (id -1: the game stops, lines 0x74b, 0x751, 0x758), and the fixture goes (0x68d0d0: push 0x4f; push &fixture; ecx =
/// the list -> thiscall, ret 8; on failure, the game stops, 0x785) into the list of the fixture's day (+0x2a), slice
/// (+0x3f) and key exe::comp_active_key_nation_of_fixture(`comp_active_key_nation_of_fixture`: push &fixture; push competition; add esp, 8 -> cdecl). At the end, fclose
/// (`exe::crt_fclose`).
/// Original: `FIXTURE_MANAGER::load_fixtures` (0x5ac3b0, fix_man.cpp, thiscall, ret 0).
///
/// [pt] { 0x5ac3b0 } (thiscall, ret 0, fix_man.cpp): `FIXTURE_MANAGER::load_fixtures()`: o nome `fixtures_<ano>.tmp` em
/// +0x1133 (0x944c9f) e, com ano (+0x1130, sem sinal) desde 1980 e o arquivo em CM3_TEMP (0x90bde0: push nome; push
/// "CM3_TEMP"; add esp, 8 -> cdecl; 0x945b00 fopen com "rb"), a conta (dword) e os jogos de 0x4f bytes (0x945bfe
/// fread; falhando, a caixa da linha 0x730 ou 0x73e e o arquivo fechado): os ids dos clubes (+0xc, +0x10 → +0x1c,
/// +0x20), da competição (+0 → +0x14, 0x6b bytes em [0xae23d0]) e do estádio (+4 → +0x18, 0x4e bytes em [0xae23b0], 0
/// fora de [0xae2358]) viram ponteiros (id -1: o jogo para, linhas 0x74b, 0x751, 0x758), e o jogo entra (0x68d0d0: push
/// 0x4f; push &jogo; ecx = a lista -> thiscall, ret 8; falhando, o jogo para, 0x785) na lista do dia (+0x2a), fatia
/// (+0x3f) e nação-chave do jogo (0x48dd70: push &jogo; push competição; add esp, 8 -> cdecl). No fim, fclose
/// (0x945a09).
pub(crate) unsafe extern "thiscall" fn load_fixtures(this: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let name = this + fixture_manager::FILE_NAME;
    let year = read_i16(this + fixture_manager::YEAR) as u16;
    format(name, 0x9cc238, year as u32);
    if year < 0x7bc {
        return;
    }
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), 0x98f928);
    if f == 0 {
        return;
    }
    let mut count = 0i32;
    if exe::crt_fread(&mut count as *mut i32 as usize, 4, 1, f) != 1 {
        show_error_at(0x9cc208, 0x730);
        exe::crt_fclose(f);
        return;
    }
    let mut game = [0u8; 0x4f];
    let j = Fixture(game.as_mut_ptr() as usize);
    let mut i = 0i32;
    while i < count {
        if exe::crt_fread(j, 0x4f, 1, f) != 1 {
            show_error_at(0x9cc208, 0x73e);
            exe::crt_fclose(f);
            return;
        }
        let a = j.home_club_id();
        if a as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x74b);
        }
        j.set_home_club(fix_man_club_by_index(a));
        let b = j.away_club_id();
        if b as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x751);
        }
        j.set_away_club(fix_man_club_by_index(b));
        let c = read_usize(j.addr());
        if c as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x758);
        }
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add(c.wrapping_mul(0x6b));
        j.set_competition(Competition(comp));
        let e = j.stadium_id() as i32;
        let stadium = if e != -1 && e < read_usize(G_STADIUM_COUNT) as i32 {
            read_usize(G_STADIUM_TABLE).wrapping_add((e as usize).wrapping_mul(0x4e))
        } else {
            0
        };
        j.set_stadium(stadium);
        let k = exe::comp_active_key_nation_of_fixture(comp, j) as i8;
        let day = (j.day() as i16) as u16 as usize; // cqd6: the day of the loaded match
        let list = read_usize(read_usize(this + (day * 3 + j.time_slice() as usize) * 4) + k as isize as usize * 4);
        crate::comparison::branch("cqd_6 match loaded");
        if (exe::llist_add_tail(list, j, 0x4f) as u32) == 0 {
            fatal_error_at(0x9cc208, 0x785);
        }
        i += 1;
    }
    exe::crt_fclose(f);
}

/// The rehearsal of the comparer for one day's conflicts. The copy does not add anything (0x68d0d0 allocates the node
/// and the copy of the match) and keeps, in order, the matches it would add.
///
/// [pt] O ensaio do comparador dos conflitos de um dia: a cópia não acrescenta (0x68d0d0 aloca o nó e a cópia do jogo) e
/// guarda os jogos que acrescentaria, na ordem.
pub(crate) static mut DAY_CLASHES_DRY_RUN: bool = false;
pub(crate) static mut DAY_CLASHES_APPENDED: Vec<usize> = Vec::new();

/// Appends a copy of the 0x4f-byte fixture to the list (0x68d0d0); in dry-run mode only records it.
///
/// [pt] Acrescenta a cópia dos 0x4f bytes do jogo no fim da lista (0x68d0d0: ecx = lista; push 0x4f; push jogo -> thiscall,
/// ret 8); no ensaio, só guarda o jogo.
pub(crate) unsafe fn append_fixture(list: usize, game: usize) {
    if DAY_CLASHES_DRY_RUN {
        DAY_CLASHES_APPENDED.push(game);
        return;
    }
    exe::llist_add_tail(list, game, 0x4f);
}

/// Data pointers of a LLIST's nodes in order (head +2, node next +6, data +0).
///
/// [pt] Os dados dos nós da lista (cabeça em +2, o próximo em +6 do nó, o dado em +0), na ordem.
pub(crate) unsafe fn fix_man_list_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut in_the = read_usize(list + 2);
    while in_the != 0 {
        v.push(read_usize(in_the));
        in_the = read_usize(in_the + 6);
    }
    v
}

/// Concatenated 0x4f bytes of each fixture in a sequence.
///
/// [pt] O conteúdo (0x4f bytes por jogo) de uma sequência de jogos.
pub(crate) unsafe fn fixtures_bytes(games: &[usize]) -> Vec<u8> {
    let mut v = Vec::new();
    for &j in games {
        v.extend_from_slice(core::slice::from_raw_parts(j as *const u8, 0x4f));
    }
    v
}

/// Builds a date from (day, month, year) via `exe::date_date_549f70` and returns (day of year, year).
///
/// [pt] A data (dia do mês, mês 0..11) do ano dado (0x549f70: lea ecx, local; push −1; push ano; push mês; push dia ->
/// thiscall, ret 0x10): (dia do ano, ano).
pub(crate) unsafe fn day_of_year(day: u32, month: u32, year: u32) -> (u16, u16) {
    let f: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut t = [0u8; 8];
    let r = f(&mut t, day, month, year, 0xffff_ffff);
    (read_u16(r as usize), read_u16(r as usize + 2))
}

/// True if the club's division (+0x57) has the given competition id.
///
/// [pt] O jogo `x` está na liga de id [0x9cf590] por um dos clubes (+0x57 → +0).
pub(crate) unsafe fn is_in_league(club: Club, id: i32) -> bool {
    let l = club.division().addr();
    l != 0 && read_i32(l) == id
}

/// Calls the competition's clash-rule method (vtable +0x60) for fixtures a and b; 0 without the table or the
/// competition.
///
/// [pt] A regra de conflito da competição (método +0x60 da competição [0xadadfc][id]: push b; push a; ecx = comp ->
/// thiscall): 0 sem a tabela ou sem a competição.
pub(crate) unsafe fn comp_clash_rule(id_of: usize, a: usize, b: usize) -> u32 {
    let table = read_usize(G_COMP_OBJECT_TABLE);
    if table == 0 {
        return 0;
    }
    let c = read_usize(table.wrapping_add((read_i32(id_of) as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + 0x60));
    f(c, a, b)
}

/// If checking played and the fixture was played (+0x43 != 0xff) sets severity 0x16; otherwise appends it and raises
/// severity to its type (+0x3a).
///
/// [pt] O jogo `x` entra (0x16 se já foi jogado, +0x43 diferente de 0xff) e a gravidade sobe ao tipo dele (+0x3a).
pub(crate) unsafe fn add_clash_by_type(grav: *mut i32, list: usize, x: Fixture, check_played: bool) {
    if check_played && (x.home_goals() as u8) != 0xff {
        *grav = 0x16;
        return;
    }
    append_fixture(list, x.addr());
    let t = x.category() as i8 as i32;
    if t > *grav {
        *grav = t;
    }
}

/// If severity is still 0 sets it to 0x14 or 1 (common tail of the same-club/same-stadium branches).
///
/// [pt] O fim comum dos ramos do mesmo clube e do mesmo estádio (0x5adf91/0x5ae060): sem gravidade, 0x14 (`vinte`) ou 1.
pub(crate) unsafe fn default_clash_severity(grav: *mut i32, twenty: bool) {
    if *grav == 0 {
        *grav = if twenty { 0x14 } else { 1 };
    }
}

/// Collects the clashes of a fixture on its day plus an offset: forbidden dates, fixtures of the same club or stadium,
/// national teams and big clubs, and fixtures already played.
///
/// `FIXTURE_MANAGER::collect_day_clashes(fixture, severity, list, nations, n, date, offset)`; SEH frame only for the
/// dates. Used for a fixture's clashes (`find_fixture_clashes`) on day `date` + offset (`exe::date_add_days`: push 16-bit n; push &local; ecx =
/// date -> thiscall, ret 8). A cup (+0x3a = 1) only on the day itself. On the day itself, the forbidden dates give 19
/// and return: 25 December (except in the competitions of the nation of [0x9cf36c]), Sunday in those of the nation of
/// [0x9cf3e4] (`exe::date_get_weekday`: ecx = date -> thiscall), 10 to 20 April in those of [0x9cf31c] (the dates through `exe::date_date_549f70`: 4
/// pushes -> thiscall); 17 May with a club of nation [0x9cf408] (also with an offset, without returning, if it is the
/// day itself). Then, for each nation in `nations` (except the 0xff ones) and each of the 3 slices, the day list
/// (`get_fixture_list`: ecx = manager; 4 pushes -> thiscall, ret 0x10) with the cursor at the start: each fixture in it
/// (`next_list_item`) that shares a club and slice (+0x40/+0x41) goes in by type; a club in another slice goes in with 0x14 or
/// 1 by +0x3f and by type 1; the same stadium (+0x18) outside the competition of [0x9cf76c], likewise; the national
/// teams of the clubs' nation (`exe::database_get_national_team`: push; add esp, 4 -> cdecl) and the clubs with reputation above 5000 by the
/// competition rule (+0x60); fixtures of the competition of [0x9cf690] with a club of the league of [0x9cf590] give 19;
/// a fixture already played (+0x43) gives 0x16. Missing club: the box and the game ends (as in the original).
/// Original: `FIXTURE_MANAGER::collect_day_clashes` (0x5adba0, fix_man.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x5adba0 } (thiscall, ret 0x1c, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::collect_day_clashes(jogo,
/// gravidade, lista, nações, n, data, deslocamento)`, pelos conflitos de um jogo (0x5ad780) no dia `data` + deslocamento
/// (0x54c5b0: push n de 16 bits; push &local; ecx = data -> thiscall, ret 8). Copa (+0x3a = 1) só no próprio dia. No
/// próprio dia, as datas proibidas dão 19 e saem: 25 de dezembro (fora das competições da nação de [0x9cf36c]), o
/// domingo nas da nação de [0x9cf3e4] (0x54a3f0: ecx = data -> thiscall), entre 10 e 20 de abril nas de [0x9cf31c] (as
/// datas pela 0x549f70: 4 pushes -> thiscall); 17 de maio com clube da nação [0x9cf408] (também com deslocamento,
/// sem sair, se for o próprio dia). Depois, para cada nação de `nações` (fora as 0xff) e cada uma das 3 fatias, a lista
/// do dia (0x5abf10: ecx = gerenciador; 4 pushes -> thiscall, ret 0x10) com o cursor do começo: cada jogo dela (0x68d1b0)
/// que divide clube e fatia (+0x40/+0x41) entra pelo tipo; clube em outra fatia, entra com 0x14 ou 1 pelo +0x3f e pelo
/// tipo 1; o mesmo estádio (+0x18) fora da competição de [0x9cf76c], igual; as seleções da nação dos clubes (0x53b3d0:
/// push; add esp, 4 -> cdecl) e os clubes de reputação acima de 5000 pela regra da competição (+0x60); os jogos da
/// competição de [0x9cf690] com clube da liga de [0x9cf590] dão 19; jogo já jogado (+0x43) dá 0x16. Clube ausente: a
/// caixa e o fim do jogo (a original).
pub(crate) unsafe extern "thiscall" fn collect_day_clashes(this: usize, game: usize, grav: *mut i32, list: usize, nations: usize, n: u32, data: usize, desloc: u32) {
    let game = Fixture(game);
    // 0x53b900 (push; add esp, 4 -> cdecl), 0x53b3d0 (push; add esp, 4 -> cdecl)
    // 0x76d940: ecx = [0xb74318]; push comp -> thiscall, ret 4 (the Rust copy ignores ecx)
    let e_sel = exe::database_is_national_team(game.home_club());
    let n1 = game.home_club().nation().addr();
    // simplification: without a nation the original leaves the national team uninitialized; every club has a nation, so 0 does not happen in the game
    let s1 = if n1 != 0 { exe::database_get_national_team(n1) } else { 0 };
    let n2 = game.away_club().nation().addr();
    let s2 = if n2 != 0 { exe::database_get_national_team(n2) } else { 0 };
    let flag = desloc as u8 as i8;
    let mut day = [0u8; 8];
    exe::date_add_days(data, (&mut day) as *mut [u8; 8] as usize, flag as i16 as u16 as u32);
    let d = day.as_ptr() as usize;
    let year = || read_usize(d + 2) as u32; // the year dword as the original pushes it (0x549f70 and 0x5abf10 read the word)
    let same = |t: (u16, u16)| t.1 == read_u16(d + 2) && t.0 == read_u16(d);
    let dezenove = || {
        if *grav < 0x13 {
            *grav = 0x13;
        }
    };
    let kind = game.category();
    if kind == 1 && flag != 0 {
        return;
    }
    if flag == 0 {
        let comp = game.competition();
        let nc = if !comp.is_null() { comp.nation() } else { Nation(0) };
        let and_of_the = |g: usize| !nc.is_null() && read_i32(nc.addr()) == read_i32(g);
        if same(day_of_year(0x19, 0xb, year())) && !and_of_the(KEY_NATION_JAPAN) {
            crate::comparison::branch("cqc_3 Christmas");
            return dezenove();
        }
        if and_of_the(KEY_NATION_NORTHERN_IRELAND) && exe::date_get_weekday((&day) as *const [u8; 8] as usize) as i16 == 6 {
            return dezenove();
        }
        if and_of_the(KEY_NATION_GREECE) {
            let t = day_of_year(10, 3, year());
            if t.1 < read_u16(d + 2) || (t.1 == read_u16(d + 2) && (t.0 as i16) < read_u16(d) as i16) {
                let t = day_of_year(0x14, 3, year());
                if t.1 > read_u16(d + 2) || (t.1 == read_u16(d + 2) && (t.0 as i16) > read_u16(d) as i16) {
                    return dezenove();
                }
            }
        }
    }
    let nation = read_usize(G_NATION_TABLE).wrapping_add((read_i32(KEY_NATION_NORWAY) as usize).wrapping_mul(nation::SIZE));
    if game.home_club().nation().addr() == nation || game.away_club().nation().addr() == nation {
        if same(day_of_year(0x11, 4, year())) && flag == 0 {
            return dezenove();
        }
    }
    let mut k: i8 = 0;
    while k < n as u8 as i8 {
        let p = nations.wrapping_add(k as isize as usize);
        if read_u8(p) != 0xff {
            let mut slice: i8 = 0;
            while slice <= 2 {
                let l = exe::fixture_manager_get_fixture_list(this, year(), read_usize(d) as u32, slice as u8 as u32, read_u8(p) as u32);
                if l != 0 {
                    write_usize(l + 0xa, read_usize(l + 2));
                    let mut x = exe::llist_get_next(l);
                    while x != 0 {
                        let c1 = Club(read_usize(x + 0x1c));
                        let h = game.home_club();
                        let a = game.away_club();
                        if c1.is_null() || h.is_null() || a.is_null() {
                            // the box (line 0xcc8) and the end of the game: the original
                            return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(IDX_COLLECT_DAY_CLASHES)(this, game.addr(), grav, list, nations, n, data, desloc);
                        }
                        let c2 = Club(read_usize(x + 0x20));
                        let (sx0, sx1, s0, s1b) = (read_u8(x + 0x40), read_u8(x + 0x41), game.home_side_mark(), (game.away_side_mark() as u8));
                        if (c1 == h && sx0 == s0) || (c1 == a && sx0 == s1b) || (c2 == h && sx1 == s0) || (c2 == a && sx1 == s1b) {
                            crate::comparison::branch("cqc_3 same club in the same pot");
                            add_clash_by_type(grav, list, Fixture(x), true); // cqc3: the same club in the same slice
                        } else if (flag == 0 && c1 == h && sx0 != s0) || (c1 == a && sx0 != s1b) || (c2 == h && sx1 != s0) || (c2 == a && sx1 != s1b) {
                            if read_u8(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                append_fixture(list, x);
                                if game.time_slice() == read_u8(x + 0x3f) || read_u8(x + 0x3a) != 1 {
                                    if *grav == 0 {
                                        crate::comparison::branch("cqc_3 same club in another pot");
                                    }
                                    default_clash_severity(grav, true); // cqc3: the same club in another pot
                                } else if game.category() != 1 {
                                    default_clash_severity(grav, false);
                                }
                            }
                        } else if flag == 0 && read_usize(x + 0x18) != 0 && game.stadium() == read_usize(x + 0x18) && read_i32(x) != read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) && read_i32(game.addr()) != read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
                            if read_u8(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                append_fixture(list, x);
                                let t = read_u8(x + 0x3a);
                                if game.time_slice() == read_u8(x + 0x3f) {
                                    if t != 1 {
                                        default_clash_severity(grav, true);
                                    } else if *grav < 1 {
                                        *grav = 1;
                                    }
                                } else if t != 1 {
                                    default_clash_severity(grav, true);
                                } else if game.category() != 1 {
                                    default_clash_severity(grav, false);
                                }
                            }
                        } else {
                            let abs_flag = (flag as i32).abs();
                            let national_teams = (e_sel == 0
                                && exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), read_usize(x + 0x14)) == 0
                                && abs_flag < 2
                                && (read_u8(p) == 0 || read_u8(p) == 1)
                                && (game.category() as i8) < 5
                                && c1.addr() == s1)
                                || c1.addr() == s2
                                || c2.addr() == s1
                                || c2.addr() == s2;
                            if national_teams {
                                if ((h.reputation() as u16) as i16 > 0x1388 || (a.reputation() as u16) as i16 > 0x1388) && comp_clash_rule(game.addr(), game.addr(), x) != 0 {
                                    add_clash_by_type(grav, list, Fixture(x), true);
                                }
                            } else if read_i32(x) == read_i32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) && (is_in_league(h, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) || is_in_league(a, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE))) {
                                *grav = 0x13;
                            } else if read_i32(game.addr()) == read_i32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) && (is_in_league(c1, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) || is_in_league(c2, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE))) {
                                add_clash_by_type(grav, list, Fixture(x), false);
                            } else if e_sel != 0
                                && exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), game.competition()) == 0
                                && abs_flag < 2
                                && read_u8(p) != 0
                                && read_u8(p) != 1
                            {
                                let (nx1, nx2) = (c1.nation().addr(), c2.nation().addr());
                                if (nx1 == n1 || nx2 == n1 || nx1 == n2 || nx2 == n2)
                                    && ((c1.reputation() as u16) as i16 > 0x1388 || (c2.reputation() as u16) as i16 > 0x1388)
                                    && (read_u8(x + 0x3a) as i8) < 5
                                    && comp_clash_rule(x, x, game.addr()) != 0
                                {
                                    add_clash_by_type(grav, list, Fixture(x), true);
                                }
                            }
                        }
                        x = exe::llist_get_next(l);
                    }
                }
                slice += 1;
            }
        }
        k += 1;
    }
}

/// Finds a fixture's clashes: builds the nations to check, creates the clash list and collects the clashes of the
/// surrounding days; returns the severity and the list, or 0 with no clash.
///
/// `FIXTURE_MANAGER::find_fixture_clashes(fixture, severity, two_days)`, used by the calendar (`add_fixture`, `find_new_fixture_date`,
/// `resolve_same_day_clash`, `move_nearby_club_fixtures`); SEH frame for the new list. Without a fixture or without an output: the box (as in the
/// original). It zeroes the output; the nations to check, without repeats: 0 to 6, the fixture's key nation (`comp_active_key_nation_of_fixture`:
/// 2 pushes; add esp, 8 -> cdecl), those of the two clubs' leagues, the stadium's (`comp_active_key_nation_of_stadium`: push; add esp, 4 ->
/// cdecl), the partners (`fix_man_get_partner_key_nation_index`: push; add esp, 4 -> cdecl) and, with a home national team (`exe::database_is_national_team`), those of the
/// two national teams (`comp_active_key_nation_of_club`: push; add esp, 4 -> cdecl). Key 3 or 5 outside the competition of [0x9cf7bc] turns off
/// the two days before. The new list (new 0xe: `exe::crt_operator_new`, push; add esp, 4 -> cdecl; `exe::new_llist`: ecx = block -> thiscall;
/// out of memory, the box and 0) and the clashes of each day (`collect_day_clashes`: ecx = this; 7 pushes -> thiscall, ret 0x1c)
/// from the fixture's date (0x549ef0: push +0x28; push +0x2a; ecx = local -> thiscall): below type 0x17, the two days
/// before (with the flag) and the day before; the day itself and the day after; again the two days before with the
/// flag. With no clash: the list is destroyed and freed (`exe::drop_llist`: ecx = list -> thiscall; `exe::crt_operator_delete`: push; add esp, 4
/// -> cdecl), 0x15 in the output if the national team forbids it (`can_schedule_club_fixture`: ecx = [0xb74318]; push fixture -> thiscall,
/// ret 4) and 0. Otherwise the severity in the output and the list.
/// Original: `FIXTURE_MANAGER::find_fixture_clashes` (0x5ad780, fix_man.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5ad780 } (thiscall, ret 0xc, fix_man.cpp; quadro SEH da lista nova): `FIXTURE_MANAGER::find_fixture_clashes(jogo,
/// gravidade, dois_dias)`, pelo calendário (0x5aa680, 0x5ab510, 0x5ae380, 0x5ae9d0). Sem jogo ou sem saída: a caixa (a
/// original). Zera a saída; as nações a olhar: 0 a 6, a nação-chave do jogo (0x48dd70: 2 pushes; add esp, 8 -> cdecl),
/// as das ligas dos dois clubes, a do estádio (0x48df40: push; add esp, 4 -> cdecl), as parceiras (0x5af5b0: push;
/// add esp, 4 -> cdecl) e, com seleção mandante (0x53b900), as das duas seleções (0x48dee0: push; add esp, 4 -> cdecl),
/// sem repetir. A chave 3 ou 5 fora da competição de [0x9cf7bc] desliga os dois dias antes. A lista nova (new 0xe:
/// 0x944cf1, push; add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e os conflitos
/// de cada dia (0x5adba0: ecx = this; 7 pushes -> thiscall, ret 0x1c) pela data do jogo (0x549ef0: push +0x28; push
/// +0x2a; ecx = local -> thiscall): abaixo do tipo 0x17, os dois dias antes (com a marca) e o anterior; o próprio dia e
/// o seguinte; de novo os dois antes com a marca. Sem conflito: a lista destruída e liberada (0x68d040: ecx = lista ->
/// thiscall; 0x944c94: push; add esp, 4 -> cdecl), 0x15 na saída se a seleção não deixa (0x776fc0: ecx = [0xb74318];
/// push jogo -> thiscall, ret 4) e 0. Senão a gravidade na saída e a lista.
pub(crate) unsafe extern "thiscall" fn find_fixture_clashes(this: usize, game: usize, out: *mut i32, two_days: u32) -> usize {
    let game = Fixture(game);
    if game.is_null() || out.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_FIND_FIXTURE_CLASHES)(this, game.addr(), out, two_days); // a caixa
    }
    let mut two = two_days;
    *out = 0;
    let mut a = [0xffu8; 16];
    for k in 0..7 {
        a[k] = k as u8;
    }
    let key = exe::comp_active_key_nation_of_fixture(game.competition(), game) as i8;
    a[7] = key as u8;
    let mut n: usize = 8;
    let home = game.home_club();
    if !home.is_null() && home.division().addr() != 0 {
        a[8] = exe::comp_active_key_nation_of_fixture(home.division().addr(), 0);
        n = 9;
    }
    let away = game.away_club();
    if !away.is_null() && away.division().addr() != 0 {
        a[n] = exe::comp_active_key_nation_of_fixture(away.division().addr(), 0);
        n += 1;
    }
    if game.stadium() != 0 {
        a[n] = exe::comp_active_key_nation_of_stadium(game.stadium());
        n += 1;
    }
    a[n] = exe::fix_man_get_partner_key_nation_index(game.home_club());
    n += 1;
    a[n] = exe::fix_man_get_partner_key_nation_index(game.away_club());
    n += 1;
    if read_i32(game.addr()) != read_i32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) && (key == 3 || key == 5) {
        two = 0;
    }
    if exe::database_is_national_team(game.home_club()) != 0 {
        a[n] = exe::comp_active_key_nation_of_club(game.home_club());
        n += 1;
        a[n] = exe::comp_active_key_nation_of_club(game.away_club());
        n += 1;
    }
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if (a[i] as i8) > -1 && a[i] == a[j] {
                a[j] = 0xff;
            }
        }
    }
    let block = exe::crt_operator_new(0xe);
    let list = if block != 0 { exe::new_llist(block) } else { 0 };
    if list == 0 {
        show_error_at(0x9cc208, 0xc1d); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        return 0;
    }
    let mut data = [0u8; 8];
    exe::new_date((&mut data) as *mut [u8; 8] as usize, game.day() as u32, game.year() as u32);
    let mut g: i32 = 0;
    let (pa, pd) = (a.as_ptr() as usize, data.as_ptr() as usize);
    if (game.category() as i8) < 0x17 {
        if two != 0 {
            exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, -2i32 as u32);
        }
        exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, -1i32 as u32);
    }
    exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, 0);
    crate::comparison::branch("cqc_3 next day");
    exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, 1); // cqc3: o dia seguinte
    if two != 0 {
        exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, -2i32 as u32);
    }
    if g == 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        if exe::nationalteams_can_schedule_club_fixture(read_usize(G_NATIONAL_TEAMS), game) == 0 {
            *out = 0x15;
        }
        return 0;
    }
    *out = g;
    list
}

/// Clears the leg check list at 0xae2a48 and refills it with the 15-byte records of check_fixture_legs.tmp.
///
/// `fix_man_load_fixture_leg_checks()` (cdecl), from the construction of the main calendar (`exe::new_fixture_manager`, on load and on
/// game creation): empties the leg check list [0xae2a48] (`manager_manager_clear`: ecx = list -> thiscall) and rebuilds it with the
/// 15-byte records of check_fixture_legs.tmp in the temporary folder (`exe::utils_build_file_path`: 2 pushes; add esp, 8 -> cdecl; fopen
/// `exe::crt_fopen` "rb", fread `exe::crt_fread`, fclose `exe::crt_fclose`: cdecl), using the 16-bit count at the start; a short record: the box
/// (line 0x10b9) and whatever was left in the record goes in (0x68d0d0).
/// Original: `fix_man_load_fixture_leg_checks` (0x5af390, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af390 } (cdecl, fix_man.cpp): `fix_man_load_fixture_leg_checks()`, pela construção do calendário principal
/// (0x5a9cf0, na carga e na criação do jogo): esvazia a lista das conferências das pernas [0xae2a48] (0x68d080: ecx =
/// lista -> thiscall) e a refaz com os registros de 15 bytes do check_fixture_legs.tmp da pasta temporária (0x90bde0:
/// 2 pushes; add esp, 8 -> cdecl; fopen 0x945b00 "rb", fread 0x945bfe, fclose 0x945a09: cdecl), pela conta de 16 bits
/// do começo; registro curto: a caixa (linha 0x10b9) e entra o que ficou no registro (0x68d0d0).
pub(crate) unsafe extern "C" fn fix_man_load_fixture_leg_checks() {
    exe::llist_clear(G_FIXTURE_LEGS_LIST);
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0x9cc258), 0x98f928); // "CM3_TEMP", "check_fixture_legs.tmp", "rb"
    if f == 0 {
        return;
    }
    // simplification: when reading the count fails the original uses stack garbage; here 0 (the game’s own file)
    let mut count = [0u8; 2];
    exe::crt_fread((count.as_mut_ptr()) as usize, 2, 1, f);
    let count = i16::from_le_bytes(count);
    let mut reg = [0u8; 15];
    let mut k: i16 = 0;
    while k < count {
        if exe::crt_fread((reg.as_mut_ptr()) as usize, 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10b9);
        }
        exe::llist_add_tail(G_FIXTURE_LEGS_LIST, (reg.as_ptr()) as *const u8 as usize, 0xf);
        k = k.wrapping_add(1);
    }
    exe::crt_fclose(f);
}

pub(crate) type RawDate = [u8; 8];

/// True if date a is after date b (year +2 unsigned, then day signed).
///
/// [pt] `a` depois de `b` como a original compara as datas: o ano (+2) sem sinal, depois o dia (+0) com sinal.
pub(crate) unsafe fn fix_man_date_is_after(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya > yb || (ya == yb && (read_u16(a) as i16) > (read_u16(b) as i16))
}

/// The date operations (thiscall; the +n ones return their static date or `ret`).
///
/// `exe::date_date_54a110` the default date, 0x549ef0 the date (day, year) (push year; push day), `exe::date_operator_3d` the assignment (push
/// source; push ret), `exe::date_add_days` + days, 0x54c8e0 − days, `exe::date_add_weeks` + weeks (push n; push ret), `exe::date_get_nearest_weekday` the target
/// weekday (push dest; push target), `exe::date_get_weekday` the weekday (no pushes).
///
/// [pt] As operações de data (thiscall; as de +n devolvem a data estática delas ou `ret`): 0x54a110 a data padrão,
/// 0x549ef0 a data (dia, ano) (push ano; push dia), 0x417c10 a atribuição (push fonte; push ret), 0x54c5b0 + dias,
/// 0x54c8e0 − dias, 0x54c770 + semanas (push n; push ret), 0x54a2d0 o dia da semana alvo (push dest; push alvo),
/// 0x54a3f0 o dia da semana (sem pushes).
pub(crate) unsafe fn date_default(d: &mut RawDate) {
    exe::date_date_54a110((d) as *mut RawDate as usize);
}
/// Builds a date from (day, year) (Date::Date).
pub(crate) unsafe fn date_from_day_year(d: &mut RawDate, day: u16, year: u16) -> usize {
    exe::new_date((d) as *mut RawDate as usize, day as u32, year as u32)
}
/// Assigns date `fonte` to d (Date operator=).
pub(crate) unsafe fn date_assign(d: &mut RawDate, font: usize) {
    let mut ret = [0u8; 8];
    exe::date_operator_3d((d) as *mut RawDate as usize, (&mut ret) as *mut RawDate as usize, font);
}
/// Calls a date arithmetic routine (add/sub days or weeks) with a local return buffer and returns a pointer to the
/// result.
pub(crate) unsafe fn date_op(routine: usize, d: usize, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut RawDate, u32) -> usize = core::mem::transmute(routine);
    let mut ret = [0u8; 8];
    // simplification: the original passes a ret on its stack; all three write into it or into the static and return the pointer,
    // which is read right after (before the next call), as in the original
    let r = f(d, &mut ret, n);
    if r == ret.as_ptr() as usize {
        DATE_OP_RESULT = ret;
        return core::ptr::addr_of!(DATE_OP_RESULT) as usize;
    }
    r
}
pub(crate) static mut DATE_OP_RESULT: RawDate = [0; 8];
/// Weekday of a date (`exe::date_get_weekday`).
pub(crate) unsafe fn date_weekday(d: usize) -> i32 {
    exe::date_get_weekday(d)
}
/// Writes into dest the nearest date with the target weekday (`exe::date_get_nearest_weekday`).
pub(crate) unsafe fn nearest_weekday(d: usize, target: u32, dest: &mut RawDate) {
    exe::date_get_nearest_weekday(d, target, (dest) as *mut RawDate as usize);
}

/// Time slice for a competition on a day (FIXTURE_MANAGER get_time_slice `get_time_slice`).
///
/// [pt] A faixa de horário (0x5af2f0: mov ecx, gerenciador; push dia; push comp -> thiscall, ret 8).
pub(crate) unsafe fn fixture_time_slice(this: usize, comp: usize, day: u32) -> u8 {
    exe::fixture_manager_get_time_slice(this, comp, day)
}

/// Clashes of a fixture (`find_fixture_clashes`): returns (clash list or 0, severity).
///
/// [pt] Os conflitos do jogo (0x5ad780: mov ecx, gerenciador; push marca; push &gravidade; push jogo -> thiscall, ret
/// 0xc): a lista (ou 0) e a gravidade.
pub(crate) unsafe fn fix_man_find_fixture_clashes(this: usize, game: usize, two: u32) -> (usize, i32) {
    let mut g: i32 = 0;
    let l = exe::fixture_manager_find_fixture_clashes(this, game, &mut g, two);
    (l, g)
}

/// Destroys and deletes a LLIST.
///
/// [pt] Destrói e libera a lista (0x68d040: mov ecx, lista -> thiscall; 0x944c94: push; add esp, 4 -> cdecl).
pub(crate) unsafe fn free_list(l: usize) {
    exe::drop_llist(l);
    exe::crt_operator_delete(l);
}

/// Finds a new date for a fixture: within a window by fixture type, it tries weekdays in a fixed order and keeps the
/// first date with no (or tolerable) clash; otherwise 2 days later.
///
/// `FIXTURE_MANAGER::find_new_fixture_date(fixture)`, used by calendar entry (`add_fixture`), by same-day clashes (`resolve_same_day_clash`)
/// and by rescheduling (`move_nearby_club_fixtures`); SEH frame only for the dates. Without a fixture: the box (as in the original). If
/// the units digit of the round (+0x30, `exe::comp_util_get_round_variant`: push; add esp, 4 -> cdecl) is 3 or 4, the fixture's 15-byte record
/// goes into the leg check list [0xae2a48] (0x68d0d0). The window by type (+0x3a): 2, up to the date at +0x24/+0x26
/// (without it, the box and 2 weeks), starting 4 weeks later (if the end is more than 20 weeks away) or 3 days later; 4
/// and 6, from 3 days to 2 weeks; 5, from 0 to 1 week; 7, 8 and 10, from 3 days before to 1 week; the others, none. In
/// two passes (the second checking the two days before), through the weekdays in the order 2, 1, 5, 6, 0, 3, 4, every 4
/// days of the window: the target day (`exe::date_get_nearest_weekday`) from 2 days after today and inside the window becomes the date (+0x28,
/// +0x2a) and the time slot (+0x3f, `get_time_slice`); with no clash (`find_fixture_clashes`), or with severity 1 or below the type, it
/// stays. Not found: 2 days after the fixture's date (or after today, if it has already passed).
/// Original: `FIXTURE_MANAGER::find_new_fixture_date` (0x5ab510, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ab510 } (thiscall, ret 4, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::find_new_fixture_date(jogo)`,
/// pela entrada no calendário (0x5aa680), pelos conflitos do mesmo dia (0x5ae380) e pela remarcação (0x5ae9d0). Sem
/// jogo: a caixa (a original). Pela casa das unidades da rodada (+0x30, 0x4b0850: push; add esp, 4 -> cdecl) 3 ou 4,
/// o registro de 15 bytes do jogo na lista de conferência das pernas [0xae2a48] (0x68d0d0). A janela pelo tipo (+0x3a):
/// 2, até a data de +0x24/+0x26 (sem ela, a caixa e 2 semanas), começando 4 semanas depois (se o fim passa de 20
/// semanas) ou 3 dias depois; 4 e 6, de 3 dias a 2 semanas; 5, de 0 a 1 semana; 7, 8 e 10, de 3 dias antes a 1 semana;
/// os outros, nenhuma. Em duas passadas (a segunda olhando os dois dias antes), pelos dias da semana na ordem 2, 1, 5,
/// 6, 0, 3, 4, de 4 em 4 dias da janela: o dia alvo (0x54a2d0) a partir de 2 dias depois de hoje e dentro da janela
/// vira a data (+0x28, +0x2a) e a faixa (+0x3f, 0x5af2f0); sem conflito (0x5ad780), com gravidade 1 ou abaixo do tipo,
/// fica. Sem achar: 2 dias depois da data do jogo (ou de hoje, se já passou).
pub(crate) unsafe extern "thiscall" fn find_new_fixture_date(this: usize, game: usize) {
    let game = Fixture(game);
    if game.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIND_NEW_FIXTURE_DATE)(this, game.addr()); // a caixa
    }
    let (mut s, mut e, mut j, mut h, mut k) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    date_default(&mut s);
    date_default(&mut e);
    date_default(&mut j);
    date_default(&mut h);
    date_default(&mut k);
    let days: [i16; 9] = [2, 1, -1, 5, 6, -1, 0, 3, 4]; // cqc4: the order of the weekdays
    let mut tmp = [0u8; 8];
    let r = date_from_day_year(&mut tmp, game.day(), game.year());
    date_assign(&mut j, r);
    let round = (game.round_name() as u16) as i16 as i32;
    if exe::comp_util_get_round_variant(round) == 3 || exe::comp_util_get_round_variant(round) == 4 {
        let mut reg = [0u8; 15];
        reg[0..4].copy_from_slice(&core::slice::from_raw_parts(game.addr() as *const u8, 4));
        reg[4] = game.stage_index();
        reg[5..7].copy_from_slice(&(game.round_number() as u16).to_le_bytes());
        reg[7..11].copy_from_slice(&core::slice::from_raw_parts((game.addr() + fixture::HOME_CLUB_ID) as *const u8, 4));
        reg[11..15].copy_from_slice(&core::slice::from_raw_parts((game.addr() + fixture::AWAY_CLUB_ID) as *const u8, 4));
        exe::llist_add_tail(G_FIXTURE_LEGS_LIST, (reg.as_ptr()) as *const u8 as usize, 0xf);
    }
    let pj = j.as_ptr() as usize;
    match (game.category() as i8 as i32).wrapping_sub(2) {
        0 => {
            if game.next_round_year() != 0 {
                let mut t = [0u8; 8];
                let r = date_from_day_year(&mut t, game.next_round_day(), game.next_round_year());
                date_assign(&mut e, r);
            } else {
                show_error_at(0x9cc208, 0x4f9); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
                let r = date_op(0x54c770, pj, 2);
                date_assign(&mut e, r);
            }
            let end = e;
            let r = date_op(0x54c770, pj, 0x14);
            let r = if fix_man_date_is_after(end.as_ptr() as usize, r) { date_op(0x54c770, pj, 4) } else { date_op(0x54c5b0, pj, 3) };
            date_assign(&mut s, r);
        }
        2 | 4 => {
            let r = date_op(0x54c770, pj, 2);
            date_assign(&mut e, r);
            let r = date_op(0x54c5b0, pj, 3);
            date_assign(&mut s, r);
        }
        3 => {
            let r = date_op(0x54c770, pj, 1);
            date_assign(&mut e, r);
            date_assign(&mut s, pj);
        }
        5 | 6 | 8 => {
            let r = date_op(0x54c770, pj, 1);
            date_assign(&mut e, r);
            let r = date_op(0x54c8e0, pj, 3);
            date_assign(&mut s, r);
        }
        _ => {}
    }
    let (ps, pe, ph, pk) = (s.as_ptr() as usize, e.as_ptr() as usize, h.as_ptr() as usize, k.as_ptr() as usize);
    for passada in 0..2u8 {
        for i in 0..3usize {
            for jj in 0..3usize {
                let target = days[i * 3 + jj];
                if target == -1 {
                    continue;
                }
                date_assign(&mut k, ps);
                while fix_man_date_is_after(pe, pk) {
                    nearest_weekday(pk, target as u16 as u32, &mut h);
                    let t = date_op(0x54c5b0, G_CURRENT_DATE, 2);
                    let t = [read_usize(t), read_usize(t + 4)];
                    let pt = t.as_ptr() as usize;
                    if !fix_man_date_is_after(pt, ph) && !fix_man_date_is_after(ph, pe) {
                        crate::comparison::branch("cqc_4 new date");
                        game.set_year(read_u16(ph + 2));
                        game.set_day(read_u16(ph));
                        game.set_time_slice(fixture_time_slice(this, game.competition().addr(), target as u16 as u32));
                        let (l, g) = fix_man_find_fixture_clashes(this, game.addr(), (passada == 1) as u32);
                        if l == 0 {
                            if g == 0 {
                                return;
                            }
                        } else {
                            free_list(l);
                        }
                        if g == 1 || (game.category() as i8 as i32) > g {
                            return;
                        }
                    }
                    let r = date_op(0x54c5b0, pk, 4); // cqc4: the search step
                    date_assign(&mut k, r);
                }
            }
        }
    }
    if fix_man_date_is_after(G_CURRENT_DATE, pj) {
        date_assign(&mut j, G_CURRENT_DATE);
    }
    let r = date_op(0x54c5b0, pj, 2);
    date_assign(&mut j, r);
    game.set_day(read_u16(pj));
    game.set_year(read_u16(pj + 2));
}

/// Adds a fixture to the calendar: fixes the stage, reschedules late fixtures, moves weekend league fixtures to the
/// home club's preferred day, resolves clashes, handles national team fixtures and neighbours, and puts the fixture in
/// the day list or in the other year's block.
///
/// `FIXTURE_MANAGER::add_fixture(fixture, reset)`, used by the cups, leagues, friendlies and rescheduling (20 callers);
/// SEH frame for the dates and the new manager. `reset` zeroes the count of chained reschedulings (word [0xae2a40]). It
/// fixes the stage (`mark_calendar_year`: push; add esp, 4 -> cdecl), +0x2e = −1; a fixture whose date has already passed and is
/// not played: new date (`find_new_fixture_date`). A league fixture of the home club's league (+0x57 or +0x60) with no round (+0x30,
/// +0x32) and no stage (+0x3b), once (bit 0x800 of +0x4d): on Saturday or Sunday, it moves to the home club's preferred
/// day (+0x72: 4, 5 or 6) if there is no clash there. With fewer than 10 chained reschedulings, the clashes (`find_fixture_clashes`):
/// severity 0x14, the same-day clash (`resolve_same_day_clash`) and 1; a cup or type 9 refused by the competition (+0x64): 0; severity
/// at least the type: 0 if the fixture's national team has a call-up ([0xb74434], 0x7e6f20), otherwise a new date and
/// it goes in again; otherwise it goes in (`insert_fixture`) and the clashing fixtures leave: the friendlies (`exe::friendly_is_friendly_competition`/`on_friendly_fixture_cancelled`
/// on [0xae2c74]), type 9 (`remove_fixture`), the others rescheduled (`remove_fixture`, `find_new_fixture_date`, `add_fixture`) with the news (`queue_fixture_moved_news` on
/// [0xae2a58]). In the block's year (+0x1130): the key nation (`comp_active_key_nation_of_fixture`), the order (`add_away_trip_illness_risk`), the national team
/// fixtures (`exe::reservecomp_add_b_or_under21_fixture` on [0xb74434], `select_squads_for_fixture` on [0xb74318]), the neighbours (`move_nearby_club_fixtures`, key 3 or 5); if it did not go
/// in, into the list of the day, slice and key (0x68d0d0). Outside the year, into the other year's block (+0x112c:
/// `set_year` for the year, or a new one of 0x1184 bytes, `exe::new_fixture_manager`; out of memory, the box and 0).
/// Original: `FIXTURE_MANAGER::add_fixture` (0x5aa680, fix_man.cpp, thiscall, ret 8).
///
/// [pt] { 0x5aa680 } (thiscall, ret 8, fix_man.cpp; quadro SEH das datas e do gerenciador novo): `FIXTURE_MANAGER::add_fixture(jogo,
/// zerar)`, pelas copas, ligas, amistosos e remarcações (20 chamadores). `zerar` zera a conta das remarcações em
/// cadeia (palavra [0xae2a40]). Acerta a fase (0x5af180: push; add esp, 4 -> cdecl), +0x2e = −1; jogo de data já
/// passada e não jogado: nova data (0x5ab510). Jogo da liga do mandante (+0x57 ou +0x60) sem rodada (+0x30, +0x32) nem
/// fase (+0x3b), uma vez (bit 0x800 de +0x4d): no sábado ou domingo, vai ao dia preferido do mandante (+0x72: 4, 5 ou
/// 6) se lá não houver conflito. Com menos de 10 remarcações em cadeia, os conflitos (0x5ad780): gravidade 0x14, o
/// conflito do mesmo dia (0x5ae380) e 1; copa ou tipo 9 recusado pela competição (+0x64): 0; gravidade de pelo menos o
/// tipo: 0 se a seleção do jogo tem convocação ([0xb74434], 0x7e6f20), senão nova data e entra de novo; senão entra
/// (0x5ab230) e os jogos do conflito saem: os amistosos (0x5c6710/0x5c6570 em [0xae2c74]), o tipo 9 (0x5abc30), os
/// outros remarcados (0x5abc30, 0x5ab510, 0x5aa680) com a notícia (0x5aac70 em [0xae2a58]). No ano do bloco (+0x1130):
/// a nação-chave (0x48dd70), a ordem (0x5ab3c0), os jogos de seleções (0x7e6720 em [0xb74434], 0x76bab0 em [0xb74318]),
/// os vizinhos (0x5ae9d0, chave 3 ou 5); se não entrou, na lista do dia, fatia e chave (0x68d0d0). Fora do ano, no
/// bloco do outro ano (+0x112c: 0x5aa2d0 para o ano, ou novo de 0x1184 bytes, 0x5a9cf0; sem memória, a caixa e 0).
pub(crate) unsafe extern "thiscall" fn add_fixture(this: usize, game: usize, clear: u32) -> u32 {
    let game = Fixture(game);
    let count = G_ADD_FIXTURE_RETRY_COUNT as *mut i16;
    let mut entrou = false;
    if clear != 0 {
        *count = 0;
    }
    exe::fix_man_record_club_fixture_years(game);
    game.set_referee(0xffffu16 as i16);
    let today = [read_usize(G_CURRENT_DATE), read_usize(G_CURRENT_DATE_LEAP)];
    let mut dj = [0u8; 8];
    let r = date_from_day_year(&mut dj, game.day(), game.year());
    if !fix_man_date_is_after(r, today.as_ptr() as usize) && (game.home_goals() as u8) == 0xff {
        exe::fixture_manager_find_new_fixture_date(this, game);
    }
    let marks = game.squad_rules();
    let home = game.home_club();
    let comp = game.competition().addr();
    if marks & 0x800 == 0
        && (comp == home.division().addr() || comp == home.reserve_division().addr())
        && (game.round_prefix() as u16) == 0
        && (game.round_name() as u16) == 0
        && game.stage() == 0xff
    {
        game.set_squad_rules(marks | 0x800);
        let preferred = home.match_day();
        if preferred != 0xff && (preferred == 5 || preferred == 6 || preferred == 4) {
            let mut d = [0u8; 8];
            date_from_day_year(&mut d, game.day(), game.year());
            let pd = d.as_ptr() as usize;
            let day = date_weekday(pd) as u8;
            if preferred != day && (day == 5 || day == 6) {
                let mut t = [0u8; 8];
                date_default(&mut t);
                crate::comparison::branch("cqc_4 home side preferred day");
                nearest_weekday(pd, preferred as i8 as i16 as u16 as u32, &mut t); // cqc4: the home side’s preferred day
                let pt = t.as_ptr() as usize;
                game.set_year(read_u16(pt + 2));
                game.set_day(read_u16(pt));
                game.set_time_slice(fixture_time_slice(this, game.competition().addr(), date_weekday(pt) as u32));
                let (l, g) = fix_man_find_fixture_clashes(this, game.addr(), 0);
                if l != 0 || g != 0 {
                    game.set_day(read_u16(pd));
                    game.set_year(read_u16(pd + 2));
                    game.set_time_slice(fixture_time_slice(this, game.competition().addr(), date_weekday(pd) as u32));
                    if l != 0 {
                        free_list(l);
                    }
                }
            }
        }
    }
    if *count < 10 {
        let (l, g) = fix_man_find_fixture_clashes(this, game.addr(), 0);
        if l != 0 || g != 0 {
            *count = (*count).wrapping_add(1);
            if g == 0x14 {
                crate::comparison::branch("cqc_4 same day");
                exe::fixture_manager_resolve_same_day_clash(this, game, l); // cqc4: the same-day conflict
                if l != 0 {
                    free_list(l);
                }
                return 1;
            }
            let kind = game.category();
            if (kind == 1 || kind == 9) && game.competition().addr() != 0 && read_usize(G_COMP_OBJECT_TABLE) != 0 {
                let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_usize(game.competition().addr()) as usize).wrapping_mul(4)));
                if c != 0 {
                    let accepted: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + 0x64));
                    if accepted(c, game.addr()) == 0 {
                        if l != 0 {
                            free_list(l);
                        }
                        return 0;
                    }
                }
            }
            // 0x7e6f20: mov ecx, [0xb74434]; push comp -> thiscall, ret 4
            if g >= game.category() as i8 as i32 {
                if l != 0 {
                    free_list(l);
                }
                if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), game.competition().addr()) != 0 {
                    return 0;
                }
                exe::fixture_manager_find_new_fixture_date(this, game);
                return (exe::fixture_manager_add_fixture(this, game, 0) != 0) as u32;
            }
            exe::fixture_manager_insert_fixture(this, game);
            entrou = true;
            if l != 0 {
                let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                let news: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_QUEUE_FIXTURE_MOVED_NEWS);
                write_usize(l + 0xa, read_usize(l + 2));
                let mut x = exe::llist_get_next(l);
                while x != 0 {
                    if friendly(read_usize(G_FRIENDLY), read_usize(x + 0x14)) != 0 {
                        exe::friendly_on_friendly_fixture_cancelled(read_usize(G_FRIENDLY), x, 0);
                    } else if read_u8(x + 0x3a) == 9 {
                        exe::fixture_manager_remove_fixture(this, x);
                    } else {
                        if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), game.competition().addr()) != 0 {
                            return 0; // as in the original, without freeing the list
                        }
                        let mut copy = [0u8; 0x4f];
                        core::ptr::copy_nonoverlapping(x as *const u8, copy.as_mut_ptr(), 0x4f);
                        let pc = copy.as_mut_ptr() as usize;
                        if exe::fixture_manager_remove_fixture(this, x) != 0 {
                            exe::fixture_manager_find_new_fixture_date(this, pc);
                            exe::fixture_manager_add_fixture(this, pc, 0);
                        }
                        news(read_usize(G_FIXTURE_MANAGER), pc, game.addr(), 0);
                    }
                    x = exe::llist_get_next(l);
                }
                free_list(l);
            }
        }
    }
    let year = game.year();
    let mut key: i8 = 0;
    if year == read_u16(this + fixture_manager::YEAR) {
        key = exe::comp_active_key_nation_of_fixture(game.competition().addr(), game) as i8;
        exe::fixture_manager_add_away_trip_illness_risk(this, game);
        if exe::database_is_national_team(game.home_club().addr()) != 0 && exe::database_is_national_team(game.away_club().addr()) != 0 && (game.home_goals() as u8) == 0xff {
            let c = game.competition().addr();
            let id = read_usize(c) as i32;
            if id != read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) as i32 && id != read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) as i32 && game.flags() & 3 == 0 {
                let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), c) == 0 || friendly(read_usize(G_FRIENDLY), game.competition().addr()) != 0 {
                    exe::reservecomp_add_b_or_under21_fixture(read_usize(G_RESERVE_COMP), game);
                }
            }
            exe::nationalteams_select_squads_for_fixture(read_usize(G_NATIONAL_TEAMS), game);
        }
        if key == 3 || key == 5 {
            exe::fixture_manager_move_nearby_club_fixtures(this, game);
        }
    }
    if entrou {
        return 1;
    }
    let year = game.year();
    if year == read_u16(this + fixture_manager::YEAR) {
        let day = game.day() as usize;
        let slice = game.time_slice() as usize;
        let row = read_usize(this + (day * 3 + slice) * 4);
        let list = read_usize(row.wrapping_add((key as isize as usize).wrapping_mul(4)));
        return ((exe::llist_add_tail(list, game, 0x4f) as u32) != 0) as u32;
    }
    let mut other = read_usize(this + fixture_manager::OTHER_YEAR);
    if other != 0 {
        if read_u16(other + 0x1130) != year {
            exe::fixture_manager_set_year(other, year as u32);
        }
    } else {
        let b = exe::crt_operator_new(0x1184);
        other = if b != 0 { exe::new_fixture_manager(b, year as u32, this, 0) } else { 0 };
        write_usize(this + fixture_manager::OTHER_YEAR, other);
        if other == 0 {
            show_error_at(0x9cc208, 0x2e6);
            return 0;
        }
    }
    exe::fixture_manager_add_fixture(read_usize(this + fixture_manager::OTHER_YEAR), game, 0);
    1
}

/// Switches a fixture block to another year: saves the year's fixtures, moves the other block to the neighbouring year
/// if needed, destroys the 366 days' lists, switches the year and leap flag, creates new lists and loads the new year's
/// fixtures.
///
/// `FIXTURE_MANAGER::set_year(year)`, used by calendar entry (`add_fixture`, the other year's block), by saving and removing
/// fixtures and by the friendlies; SEH frame for the new blocks. If the same year (word, +0x1130): returns 1.
/// Otherwise: it writes the year's fixtures (`exe::fixture_manager_save_fixtures`: ecx = block; push 0 -> thiscall; on failure, the box and 0); if
/// the other block (+0x112c) is in the requested year, it moves to the year neighbouring this one's (`set_year` on it);
/// it destroys and frees the lists of the 366 days × 3 slices × key nations ([0xb64738], signed byte; `exe::drop_llist`,
/// `exe::crt_operator_delete`); the other block in the requested year, if this is the main one (+0x1183), is destroyed and freed
/// (`exe::drop_fixture_manager`: ecx = block -> thiscall; `exe::crt_operator_delete`); the new year, leap flag at +0x1132 (`exe::date_is_leap_year`: push year; add esp, 4
/// -> cdecl), the new lists (new 0xe: `exe::crt_operator_new`, push; add esp, 4 -> cdecl; `exe::new_llist`: ecx = block -> thiscall; out of
/// memory, the box and 0) and the loading of the year's fixtures (`load_fixtures`: ecx = block -> thiscall). Returns 1.
/// Original: `FIXTURE_MANAGER::set_year` (0x5aa2d0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5aa2d0 } (thiscall, ret 4, fix_man.cpp; quadro SEH dos blocos novos): `FIXTURE_MANAGER::set_year(ano)`, pela entrada
/// no calendário (0x5aa680, o bloco do outro ano), pela guarda e pela retirada de jogos e pelos amistosos. O mesmo ano
/// (palavra, +0x1130): 1. Grava os jogos do ano (0x5ac060: ecx = bloco; push 0 -> thiscall; falhando, a caixa e 0); se
/// o outro bloco (+0x112c) está no ano pedido, ele vai para o ano vizinho do deste (0x5aa2d0 nele); destrói e libera as
/// listas dos 366 dias × 3 fatias × nações-chave ([0xb64738], byte com sinal; 0x68d040, 0x944c94); o outro bloco no
/// ano pedido, se este é o principal (+0x1183), é destruído e liberado (0x5aa0d0: ecx = bloco -> thiscall; 0x944c94); o
/// ano novo, bissexto em +0x1132 (0x54cfe0: push ano; add esp, 4 -> cdecl), as listas novas (new 0xe: 0x944cf1, push;
/// add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e a carga dos jogos do ano
/// (0x5ac3b0: ecx = bloco -> thiscall). Devolve 1.
pub(crate) unsafe extern "thiscall" fn set_year(this: usize, year: u32) -> u32 {
    let a = year as u16;
    if a == read_u16(this + fixture_manager::YEAR) {
        return 1;
    }
    if exe::fixture_manager_save_fixtures(this, 0) == 0 {
        show_error_at(0x9cc208, 0x126);
        return 0;
    }
    let other = read_usize(this + fixture_manager::OTHER_YEAR);
    if other != 0 {
        let oa = read_u16(other + 0x1130);
        if oa == a {
            let vizinho = if read_u16(this + fixture_manager::YEAR) as u32 == oa as u32 + 1 { oa.wrapping_sub(1) } else { oa.wrapping_add(1) };
            exe::fixture_manager_set_year(other, vizinho as u32);
        }
    }
    let n = || read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8;
    for dia in 0..0x16eusize {
        if n() > 0 {
            let base = this + 8 + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                for p in [base - 8, base - 4, base] {
                    let slot = read_usize(p).wrapping_add((k as i32 as usize).wrapping_mul(4));
                    let l = read_usize(slot);
                    if l != 0 {
                        free_list(l);
                        write_usize(read_usize(p).wrapping_add((k as i32 as usize).wrapping_mul(4)), 0);
                    }
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let other = read_usize(this + fixture_manager::OTHER_YEAR);
    if other != 0 && read_u16(other + 0x1130) == a && read_u8(this + fixture_manager::IS_PRIMARY) != 0 {
        exe::drop_fixture_manager(other);
        exe::crt_operator_delete(other);
        write_usize(this + fixture_manager::OTHER_YEAR, 0);
    }
    write_u16(this + fixture_manager::YEAR, a);
    write_u8(this + fixture_manager::LEAP_YEAR, (exe::date_is_leap_year(year) != 0) as u8);
    crate::comparison::branch("cqc_4 calendar new year");
    for dia in 0..0x16eusize {
        if n() > 0 {
            let e = this + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                let desloc = (k as i32 as usize).wrapping_mul(4);
                for j in 0..3usize {
                    let b = exe::crt_operator_new(0xe);
                    let l = if b != 0 { exe::new_llist(b) } else { 0 };
                    write_usize(read_usize(e + j * 4).wrapping_add(desloc), l);
                }
                if (0..3usize).any(|j| read_usize(read_usize(e + j * 4).wrapping_add(desloc)) == 0) {
                    show_error_at(0x9cc208, 0x170);
                    return 0;
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    exe::fixture_manager_load_fixtures(this);
    1
}

/// Opens a file by name in the CM3_TEMP folder with fopen and the given mode.
///
/// [pt] Abre `nome` em CM3_TEMP (0x90bde0: cdecl(pasta, nome); fopen 0x945b00: cdecl(caminho, modo)).
pub(crate) unsafe fn open_temp_file(name: usize, mode: usize) -> usize {
    exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), mode)
}

/// Writes the club dates table [0xae2a5c], one word per club ([0xae235c]), into CM3_TEMP\club_dates.tmp.
///
/// `fix_man_save_club_dates()` (cdecl), from the calendar save (FIXTURE_MANAGER::save `exe::fixture_manager_save_fixtures`, 0x5ac394): without the
/// club dates table ([0xae2a5c]), the "Error" box for line 0x1011 (fix_man.cpp). It writes into CM3_TEMP\club_dates.tmp
/// ("wb") one word per club ([0xae235c] of them, fwrite `exe::crt_fwrite`); a short write: the box for line 0x101b; then it
/// closes the file. If the file does not open: nothing.
/// Original: `fix_man_save_club_dates` (0x5af040, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af040 } (cdecl, fix_man.cpp): `fix_man_save_club_dates()`, do save do calendário (FIXTURE_MANAGER::save
/// 0x5ac060, 0x5ac394): sem a tabela das datas dos clubes ([0xae2a5c]), a caixa da linha 0x1011. Grava em
/// CM3_TEMP\club_dates.tmp ("wb") uma palavra por clube ([0xae235c] delas, fwrite 0x945ce6); gravação curta: a caixa
/// da linha 0x101b; fecha. Sem abrir: nada.
pub(crate) unsafe extern "C" fn fix_man_save_club_dates() {
    if read_usize(G_CLUB_FIXTURE_YEAR_TABLE) == 0 {
        return show_error_at(0x9cc208, 0x1011);
    }
    let f = open_temp_file(0x9cc248, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparison::branch("cpy_7 save club dates");
    if exe::crt_fwrite(read_usize(G_CLUB_FIXTURE_YEAR_TABLE), 2, club_count(), f) != club_count() { // cpy7: the club dates
        show_error_at(0x9cc208, 0x101b);
    }
    uefa_seeding_crt_fclose(f);
}

/// Allocates the club dates table [0xae2a5c] (one word per club) and reads it from CM3_TEMP\club_dates.tmp, or zeroes
/// it without the file.
///
/// `fix_man_load_club_dates()` (cdecl), from the calendar constructor (`exe::new_fixture_manager`, 0x5a9e38: on a new career and on
/// loading the save). With the table already allocated ([0xae2a5c]), the "Error" box for line 0xfe4. It allocates one
/// word per club (malloc `exe::crt_malloc` of 2 × [0xae235c]; out of memory, the box for line 0xfec); reads from
/// CM3_TEMP\club_dates.tmp ("rb", fread `exe::crt_fread`; a short read: the box for line 0xff6) and closes it; without the file
/// (a new career), it zeroes the table.
/// Original: `fix_man_load_club_dates` (0x5aee90, fix_man.cpp, cdecl).
///
/// [pt] { 0x5aee90 } (cdecl, fix_man.cpp): `fix_man_load_club_dates()`, do construtor do calendário (0x5a9cf0, 0x5a9e38:
/// na carreira nova e na carga do save). Com a tabela já alocada ([0xae2a5c]), a caixa da linha 0xfe4. Aloca uma
/// palavra por clube (malloc 0x944e46 de 2 × [0xae235c]; sem memória, a caixa da linha 0xfec); lê de
/// CM3_TEMP\club_dates.tmp ("rb", fread 0x945bfe; leitura curta: a caixa da linha 0xff6) e fecha; sem o arquivo (a
/// carreira nova), zera a tabela.
pub(crate) unsafe extern "C" fn fix_man_load_club_dates() {
    if read_usize(G_CLUB_FIXTURE_YEAR_TABLE) != 0 {
        return show_error_at(0x9cc208, 0xfe4);
    }
    let t = exe::crt_malloc(club_count().wrapping_mul(2));
    write_usize(G_CLUB_FIXTURE_YEAR_TABLE, t);
    if t == 0 {
        return show_error_at(0x9cc208, 0xfec);
    }
    let f = open_temp_file(0x9cc248, 0x98f928);
    if f == 0 {
        crate::comparison::branch("cpy_7 club dates zeroed");
        core::ptr::write_bytes(read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as *mut u8, 0, club_count().wrapping_mul(2) as usize);
        return;
    }
    crate::comparison::branch("cpy_7 club dates read");
    if exe::crt_fread(read_usize(G_CLUB_FIXTURE_YEAR_TABLE), 2, club_count(), f) != club_count() { // cpy7: the reading of the dates
        show_error_at(0x9cc208, 0xff6);
    }
    uefa_seeding_crt_fclose(f);
}

/// Writes the leg check list [0xae2a48] into CM3_TEMP\check_fixture_legs.tmp: the count (2 bytes) and the 15-byte
/// items.
///
/// `fix_man_save_fixture_leg_checks()` (cdecl), from the calendar save (0x5ac399): with the leg check list ([0xae2a48],
/// LLIST) empty (the count word zero), nothing. It writes into CM3_TEMP\check_fixture_legs.tmp ("wb") the count (2
/// bytes) and, from the start of the list (cursor +0xa ← head +2), each 15-byte item (`next_list_item`; a short write: the
/// "Error" box for line 0x10e0, and it carries on); then it closes the file. If the file does not open: nothing.
/// Original: `fix_man_save_fixture_leg_checks` (0x5af4a0, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af4a0 } (cdecl, fix_man.cpp): `fix_man_save_fixture_leg_checks()`, do save do calendário (0x5ac399): com a
/// lista de conferência das pernas ([0xae2a48], LLIST) vazia (a palavra da conta zero), nada. Grava em
/// CM3_TEMP\check_fixture_legs.tmp ("wb") a conta (2 bytes) e, do começo da lista (o cursor +0xa ← a cabeça +2), cada
/// item de 15 bytes (0x68d1b0; gravação curta: a caixa da linha 0x10e0, e segue); fecha. Sem abrir: nada.
pub(crate) unsafe extern "C" fn fix_man_save_fixture_leg_checks() {
    let count = read_usize(G_FIXTURE_LEGS_LIST);
    if count as u16 == 0 {
        return;
    }
    let f = open_temp_file(0x9cc258, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparison::branch("cpy_7 save leg check");
    exe::crt_fwrite(&count as *const usize as usize, 2, 1, f);
    write_usize(G_FIXTURE_LEGS_LIST_CURSOR, read_usize(G_FIXTURE_LEGS_LIST_HEAD));
    loop {
        let item = exe::llist_get_next(G_FIXTURE_LEGS_LIST);
        if item == 0 {
            break;
        }
        if exe::crt_fwrite(item, 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10e0);
        }
    }
    uefa_seeding_crt_fclose(f);
}

/// Rehearsal mode of `resolve_same_day_clash` (the comparer). The copy stops at the decision, before touching the calendar.
///
/// [pt] Modo de ensaio da 0x5ae380 (o comparador): a cópia para na decisão, antes de mexer no calendário.
pub(crate) static mut SAME_DAY_CLASH_DRY_RUN: bool = false;

/// Resolves a new fixture falling on the same day as another: the one with higher priority stays and the other is moved
/// to a new date, with the rescheduling news.
///
/// `FIXTURE_MANAGER::resolve_same_day_clash(fixture, clashes)`; SEH frame with no destructor on the normal path. The
/// new fixture `fixture` falls on the same day as another, the first of the `clashes` list (cursor +0xa goes back to
/// the start +2 and `next_list_item` reads it); without either of them, the box for line 0xde6 and it returns 1. It copies both
/// (0x4f bytes) and finds the target weekday from the date of the list's fixture (+0x2a, +0x28; 0x549ef0, `exe::date_operator_3d`,
/// `exe::date_get_weekday`): days 0 and 5 of `exe::date_get_weekday` go to 6, 1 and 3 to 2, 2 to 1, 4 and 6 to 5 (`exe::date_get_nearest_weekday` gives the date). Each
/// one's priority is its competition's (+0x69 of +0x14), +0x15 (21) with a national team at home (+0x1c, `exe::database_is_national_team`); the
/// list's fixture leaves the day when the new one's priority is higher (or equal, with the new one's home club
/// reputation +0x80 higher) and it has not been played yet (+0x43 = 0xff): it is removed from the calendar (`remove_fixture`)
/// and the new one goes in (`add_fixture`); otherwise the new one leaves. The one that leaves gets the new date; with a
/// clash there (`find_fixture_clashes`, the list destroyed and freed: `exe::drop_llist`, `exe::crt_operator_delete`), a fixed fixture (+0x3a = 1) returns 0,
/// otherwise `find_new_fixture_date` finds another date; it goes into the calendar (`add_fixture`) with the rescheduling news (`queue_fixture_moved_news`,
/// caused by the other one). Returns 1.
/// Original: `FIXTURE_MANAGER::resolve_same_day_clash` (0x5ae380, fix_man.cpp, thiscall).
///
/// [pt] { 0x5ae380 } (thiscall, ret 8, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::resolve_same_day_clash(jogo, conflitos)`: o jogo novo `jogo` cai no mesmo dia de outro, o primeiro
/// da lista `conflitos` (o cursor +0xa volta ao começo +2 e 0x68d1b0 o lê); sem um dos dois, a caixa da linha 0xde6
/// e devolve 1. Copia os dois (0x4f bytes) e acha o dia da semana alvo pela data do da lista (+0x2a, +0x28; 0x549ef0,
/// 0x417c10, 0x54a3f0): os dias 0 e 5 da 0x54a3f0 vão a 6, 1 e 3 a 2, o 2 a 1, 4 e 6 a 5 (0x54a2d0 dá a
/// data). A prioridade de cada um é a da competição (+0x69 da +0x14), +0x15 com seleção
/// como mandante (+0x1c, 0x53b900); o da lista sai do dia quando a do novo é maior (ou igual, com a reputação +0x80
/// do mandante do novo maior) e ele ainda não foi jogado (+0x43 = 0xff): tirado do calendário (0x5abc30), o novo entra
/// (0x5aa680); senão sai o novo. O que sai ganha a data nova; com conflito nela (0x5ad780, a lista destruída e
/// liberada: 0x68d040, 0x944c94), jogo fixo (+0x3a = 1) devolve 0, senão 0x5ab510 acha outra data; entra no calendário
/// (0x5aa680) com a notícia da remarcação (0x5aac70, causada pelo outro). Devolve 1.
pub(crate) unsafe extern "thiscall" fn resolve_same_day_clash(this: usize, game: usize, conflitos: usize) -> u32 {
    // the dates: ecx = the date, no pushes -> thiscall (0x54a110, 0x54a3f0); 0x549ef0: push [+0x28]; push [+0x2a];
    // 0x417c10: push ret; push &tmp; 0x54a2d0: push &b; push alvo -> thiscall
    let new: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_DATE);
    // 0x68d1b0: ecx = the list, no pushes -> thiscall; 0x53b900: push clube; add esp, 4 -> cdecl
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110((&mut a) as *mut [u8; 8] as usize);
    exe::date_date_54a110((&mut b) as *mut [u8; 8] as usize);
    let mut severity: u32 = 0; // [esp+0x10]: the uncased target of the table and the severity from 0x5ad780
    write_usize(conflitos + 0xa, read_usize(conflitos + 2));
    let other = exe::llist_get_next(conflitos);
    if game == 0 || other == 0 {
        if !SAME_DAY_CLASH_DRY_RUN {
            show_error_at(0x9cc208, 0xde6); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        }
        return 1;
    }
    let (mut novo, mut velho) = ([0u8; 0x50], [0u8; 0x50]);
    core::ptr::copy_nonoverlapping(game as *const u8, novo.as_mut_ptr(), 0x4f);
    core::ptr::copy_nonoverlapping(other as *const u8, velho.as_mut_ptr(), 0x4f);
    let (mut tmp, mut tmp2) = ([0u8; 8], [0u8; 8]);
    // simplification: the high bytes of the two 16-bit pushes do not count (0x549ef0 reads the word)
    let r = new(&mut tmp, read_i16(other + 0x2a) as u16 as u32, read_i16(other + 0x28) as u16 as u32);
    exe::date_operator_3d((&mut a) as *mut [u8; 8] as usize, (&mut tmp2) as *mut [u8; 8] as usize, (r) as *mut [u8; 8] as usize);
    let target = match exe::date_get_weekday((&mut a) as *mut [u8; 8] as usize) as i16 {
        0 | 5 => 6,
        1 | 3 => 2,
        2 => 1,
        4 | 6 => 5,
        _ => severity,
    };
    crate::comparison::branch("cqb_13 target day");
    exe::date_get_nearest_weekday((&mut a) as *mut [u8; 8] as usize, target, (&mut b) as *mut [u8; 8] as usize); // cqb13: the target weekday
    let n = novo.as_mut_ptr() as usize;
    let v = Fixture(velho.as_mut_ptr() as usize);
    let mut pn = read_i16(read_usize(n + 0x14) + 0x69);
    let mut pv = v.competition().reputation();
    if exe::database_is_national_team(read_usize(n + 0x1c)) != 0 {
        pn = pn.wrapping_add(0x15);
    }
    if exe::database_is_national_team(v.home_club()) != 0 {
        pv = pv.wrapping_add(0x15);
    }
    let new_wins = pn > pv // cqb13: a prioridade maior fica
        || (pn == pv && read_i16(read_usize(n + 0x1c) + 0x80) > v.home_club().reputation());
    if new_wins {
        crate::comparison::branch("cqb_13 new one wins");
    }
    let sai_old = new_wins && (v.home_goals() as u8) == 0xff; // cqb13: the one in the list not yet played
    if SAME_DAY_CLASH_DRY_RUN {
        return 1;
    }
    // the calendar ones: ecx = this; push the match (0x5abc30, 0x5ab510; ret 4), push 0; push the match (0x5aa680; ret 8),
    // push 0; push &gravidade; push the match (0x5ad780; ret 0xc), push 0; push causa; push the match (0x5aac70; ret 0xc);
    // 0x68d040: ecx = a lista; 0x944c94: push lista; add esp, 4 -> cdecl
    let (sai, causa) = if sai_old {
        crate::comparison::branch("cqb_13 match leaves the list");
        exe::fixture_manager_remove_fixture(this, other);
        exe::fixture_manager_add_fixture(this, game, 0);
        (v.addr(), game)
    } else {
        (n, other)
    };
    write_u16(sai + 0x2a, u16::from_le_bytes([b[0], b[1]]));
    write_u16(sai + 0x28, u16::from_le_bytes([b[2], b[3]]));
    let list = exe::fixture_manager_find_fixture_clashes(this, sai, (&mut severity) as *mut u32 as *mut i32, 0);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        if read_u8(sai + 0x3a) == 1 {
            return 0;
        }
        exe::fixture_manager_find_new_fixture_date(this, sai);
    }
    exe::fixture_manager_add_fixture(this, sai, 0);
    exe::fixture_manager_queue_fixture_moved_news(this, sai, causa, 0);
    1
}

/// Returns the club's first year with fixtures: 2001, or 2002 by bit 0 of its word in [0xae2a5c].
///
/// `FIXTURE_MANAGER::get_club_first_fixture_year(&id)`; this is not read. The first year with fixtures of club `[id]`:
/// 2001, or 2002 with bit 0 of its word in [0xae2a5c]. Returns `ax` (the screen caller compares 16 bits).
/// Original: `FIXTURE_MANAGER::get_club_first_fixture_year` (0x5aee40, fix_man.cpp, thiscall).
///
/// [pt] { 0x5aee40 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_first_fixture_year(&id)`:
/// o primeiro ano com jogos do clube `[id]`: 2001, ou 2002 com o bit 0 da palavra dele em [0xae2a5c]. Devolve `ax`
/// (o chamador da tela compara 16 bits).
pub(crate) unsafe extern "thiscall" fn get_club_first_fixture_year(_this: usize, id: usize) -> u16 {
    let b = *((read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u8);
    0x7d1 + (b & 1) as u16
}

/// Returns the club's last year with fixtures: its word in [0xae2a5c] without bit 0, or the current year.
///
/// `FIXTURE_MANAGER::get_club_last_fixture_year(&id)`; this is not read. The club's last year with fixtures: its word
/// in [0xae2a5c] without bit 0 (halved); with no year, the current year [0xae2c92]. Returns `ax` (the screen caller
/// keeps 16 bits).
/// Original: `FIXTURE_MANAGER::get_club_last_fixture_year` (0x5aee60, fix_man.cpp, thiscall).
///
/// [pt] { 0x5aee60 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_last_fixture_year(&id)`:
/// o último ano com jogos do clube: a palavra dele em [0xae2a5c] sem o bit 0 (metade); sem ano, o ano corrente
/// [0xae2c92]. Devolve `ax` (o chamador da tela guarda 16 bits).
pub(crate) unsafe extern "thiscall" fn get_club_last_fixture_year(_this: usize, id: usize) -> u16 {
    let w = core::ptr::read_unaligned((read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u16);
    if w & 0xfffe != 0 { w >> 1 } else { *(G_CURRENT_DATE_YEAR as *const u16) }
}

compare_value!(compare_get_club_first_fixture_year, get_club_first_fixture_year, IDX_GET_CLUB_FIRST_FIXTURE_YEAR, (id: usize) -> u16);
compare_value!(compare_get_club_last_fixture_year, get_club_last_fixture_year, IDX_GET_CLUB_LAST_FIXTURE_YEAR, (id: usize) -> u16);

/// LLIST constructor (`exe::new_llist`).
///
/// [pt] LLIST (0xe bytes: +0 conta i16, +2 cabeça, +6 cauda, +0xa cursor; nó: +0 dados, +6 próximo): o construtor
/// (0x68d020), o destrutor (0x68d040), o esvaziar (0x68d080), o cursor na cabeça e o próximo (0x68d1b0).
pub(crate) unsafe fn list_init(l: usize) {
    exe::new_llist(l);
}

/// LLIST destructor (`exe::drop_llist`).
pub(crate) unsafe fn list_destroy(l: usize) {
    exe::drop_llist(l);
}

/// Empties a LLIST (`manager_manager_clear`).
pub(crate) unsafe fn list_clear(l: usize) {
    exe::llist_clear(l);
}

/// Sets the LLIST cursor (+0xa) to the head (+2).
pub(crate) unsafe fn list_rewind(l: usize) {
    write_u32(l + 0xa, read_u32(l + 2));
}

/// True if the competition is a friendly (`exe::friendly_is_friendly_competition` on G_FRIENDLY).
///
/// [pt] A competição do jogo é amistosa? (0x5c6710: thiscall em [0xae2c74], 1 argumento.)
pub(crate) unsafe fn fix_man_is_friendly_comp(comp: u32) -> bool {
    exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), (comp) as usize) != 0
}

/// Builds a date from (day, year) at d (0x549ef0) and returns it.
///
/// [pt] As datas (Date, 8 bytes): construir por (dia, ano) (0x549ef0), menos e mais n dias (0x54c8e0, 0x54c5b0: thiscall
/// (data, saída, n), devolvem a saída) e a atribuição (0x417c10: thiscall(destino, temporária, origem)).
pub(crate) unsafe fn date_from_day_year_ptr(d: *mut u8, day: u32, year: u32) -> usize {
    exe::new_date(d as usize, day, year)
}

/// Date minus n days into `saida` (0x54c8e0).
pub(crate) unsafe fn date_sub_days(d: usize, out: *mut u8, n: u32) -> usize {
    exe::date_sub_days(d, out as usize, n)
}

/// Date plus n days into `saida` (`exe::date_add_days`).
pub(crate) unsafe fn fix_man_date_add_days(d: usize, out: *mut u8, n: u32) -> usize {
    exe::date_add_days(d, out as usize, n)
}

/// Assigns date `origem` to `destino` (`exe::date_operator_3d`).
pub(crate) unsafe fn date_assign_ptr(target: *mut u8, source: usize) {
    let mut temporary = [0u8; 8];
    exe::date_operator_3d(target as usize, temporary.as_mut_ptr() as usize, source);
}

/// Finds the club's fixtures between two dates into a list (FIXTURE_MANAGER find_fixtures `find_fixtures`, any comp/type, side
/// 1).
///
/// [pt] Os jogos do clube entre duas datas (0x5acbe0, thiscall, 9 argumentos): a original empurra os dwords em +0 e +2 de
/// cada data (só a palavra baixa conta), o clube, a lista, comp −1, tipo −2 e lado 1.
pub(crate) unsafe fn find_club_fixtures_between(this: usize, a: *const u8, b: *const u8, club: u32, list: usize) -> u32 {
    let (a, b) = (a as usize, b as usize);
    (exe::fixture_manager_find_fixtures(this, read_u32(a), read_u32(a + 2), read_u32(b), read_u32(b + 2), club, list, u32::MAX, 0xffff_fffe, 1) as u32)
}

/// Removes a fixture from the calendar (`remove_fixture`).
///
/// [pt] Tirar o jogo do calendário (0x5abc30), gravar a data (0x5ab510), juntar (0x5aa680) e a notícia (0x5aac70).
pub(crate) unsafe fn fix_man_remove_fixture(this: usize, game: usize) -> u32 {
    exe::fixture_manager_remove_fixture(this, game)
}

/// Adds a fixture to the calendar (`add_fixture`).
pub(crate) unsafe fn fix_man_add_fixture(this: usize, game: usize, clear: u32) {
    exe::fixture_manager_add_fixture(this, game, clear);
}

/// Queues the "fixture moved" news item (`queue_fixture_moved_news`) for the fixture, cause and old date.
pub(crate) unsafe fn fix_man_queue_fixture_moved_news(this: usize, game: usize, causa: Fixture, old: usize) {
    exe::fixture_manager_queue_fixture_moved_news(this, game, causa, (old) as u32);
}

/// True if the fixture's new date clashes with a fixture other than the original f1 (via `find_fixture_clashes`); frees the clash
/// list.
///
/// [pt] A data nova do jogo `f2` tem outro jogo? (0x5ad780: thiscall(jogo, &gravidade, 0) devolve uma LLIST alocada;
/// percorre e acha um item com bytes diferentes do jogo original `f1`; depois destrói e libera, 0x944c94.)
pub(crate) unsafe fn new_date_has_clash(this: usize, f2: *mut u8, f1: *const u8) -> bool {
    let mut severity = 0i32;
    let l = exe::fixture_manager_find_fixture_clashes(this, f2 as usize, &mut severity, 0);
    if l == 0 {
        return false;
    }
    list_rewind(l);
    let mut other = false;
    loop {
        let q = contract_manager_list_next(l);
        if q == 0 {
            break;
        }
        if core::slice::from_raw_parts(f1, 0x4f) != core::slice::from_raw_parts(q as *const u8, 0x4f) {
            other = true;
            break;
        }
    }
    list_destroy(l);
    exe::crt_operator_delete(l);
    other
}

/// Copies the first fixture of the list into f1 and f2 (0x4f bytes).
///
/// [pt] O primeiro jogo da lista copiado para `f1` e `f2` (0x4f bytes).
pub(crate) unsafe fn copy_first_fixture(l: usize, f1: &mut [u8; 0x4f], f2: &mut [u8; 0x4f]) {
    list_rewind(l);
    let p = contract_manager_list_next(l);
    core::ptr::copy_nonoverlapping(p as *const u8, f1.as_mut_ptr(), 0x4f);
    *f2 = *f1;
}

/// For a non-friendly fixture, moves the clubs' fixtures two days before one day earlier and those two days after one
/// day later, if the new date is free, with the news.
///
/// `FIXTURE_MANAGER::move_nearby_club_fixtures(fixture)`, from adding the fixture (`add_fixture`); SEH for the local list
/// and the dates. A fixture of a friendly competition (+0x14, `exe::friendly_is_friendly_competition`): nothing. For both clubs (+0x1c, +0x20) of a
/// nation (+0x53) with the flag +0x11c: the club's first fixture two days before (`find_fixtures`, side 1), not a friendly and
/// with +0x43 = 0xff, moves to one day before it; the first fixture two days after, not a friendly, moves to one day
/// after it. Only if the new date has no other fixture (`find_fixture_clashes`): it removes (`remove_fixture`), adds (`add_fixture`, 0) and posts
/// the rescheduled fixture news (`queue_fixture_moved_news`, with the fixture as the cause).
/// Original: `FIXTURE_MANAGER::move_nearby_club_fixtures` (0x5ae9d0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ae9d0 } (thiscall, ret 4, fix_man.cpp, SEH da lista local e das datas): `FIXTURE_MANAGER::
/// move_nearby_club_fixtures(jogo)`, pela junção do jogo (0x5aa680). Jogo de competição amistosa (+0x14, 0x5c6710):
/// nada. Para os dois clubes (+0x1c, +0x20) de nação (+0x53) com a marca +0x11c: o primeiro jogo do clube dois dias
/// antes (0x5acbe0, lado 1), não amistoso e com +0x43 = 0xff, vai para um dia antes dele; o primeiro jogo dois dias
/// depois, não amistoso, vai para um dia depois. Só se a data nova não tiver outro jogo (0x5ad780): tira (0x5abc30),
/// junta (0x5aa680, 0) e dá a notícia do jogo remarcado (0x5aac70, com o jogo como causa).
pub(crate) unsafe extern "thiscall" fn move_nearby_club_fixtures(this: usize, game: usize) {
    let game = Fixture(game);
    let mut d = [0u8; 8];
    exe::date_date_54a110(d.as_mut_ptr() as usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    if fix_man_is_friendly_comp((game.competition().addr() as u32)) {
        list_destroy(lp);
        return;
    }
    let mut f1 = [0u8; 0x4f];
    let mut f2 = [0u8; 0x4f];
    for k in 0..2 {
        let club = Club(read_usize(game.addr() + if k == 0 { 0x1c } else { 0x20 }));
        let nation = club.nation();
        if nation.is_null() || nation.league_selected() == 0 {
            continue;
        }
        // dois dias antes
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), game.day() as u32, game.year() as u32);
        let r = date_sub_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), read_u32(club.addr()), lp) as u16 != 0 {
            copy_first_fixture(lp, &mut f1, &mut f2);
            if !fix_man_is_friendly_comp(read_u32(f1.as_ptr() as usize + 0x14)) {
                crate::comparison::branch("cpz_8 match two days before");
                if f2[0x43] == 0xff { // cpz8: the match from two days before without the mark
                    let mut v = [0u8; 8];
                    let r = date_sub_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia antes
                    date_assign_ptr(d.as_mut_ptr(), r);
                    f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
                    f2[0x28..0x2a].copy_from_slice(&d[2..4]);
                    if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
                        crate::comparison::branch("cpz_8 match brought forward");
                        fix_man_remove_fixture(this, f1.as_ptr() as usize);
                        fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
                        fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, game, 0);
                    }
                }
            }
        }
        // dois dias depois
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), game.day() as u32, game.year() as u32);
        let r = fix_man_date_add_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), read_u32(club.addr()), lp) as u16 == 0 {
            continue;
        }
        copy_first_fixture(lp, &mut f1, &mut f2);
        if fix_man_is_friendly_comp(read_u32(f1.as_ptr() as usize + 0x14)) {
            continue;
        }
        let mut v = [0u8; 8];
        let r = fix_man_date_add_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia depois
        date_assign_ptr(d.as_mut_ptr(), r);
        f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
        f2[0x28..0x2a].copy_from_slice(&d[2..4]);
        if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
            crate::comparison::branch("cpz_8 match postponed");
            fix_man_remove_fixture(this, f1.as_ptr() as usize);
            fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
            fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, game, 0);
        }
    }
    list_destroy(lp);
}

// simplification: moves matches in the calendar (removes, merges, allocates and frees lists, news): original only; the copy carries the branches.

/// Reschedules the club's fixtures from one day before to one day after a national team date: each removed fixture gets
/// a new date, is added back and posts the news with the old date.
///
/// `FIXTURE_MANAGER::reschedule_club_fixtures_near_date(club, date)`, from the national team call-ups (0x777940: a club
/// with more than two players called up); SEH for the local list. The club's fixtures (the id in `*club`) from one day
/// before to one day after the date (`find_fixtures`, side 1); each one that leaves the calendar (`remove_fixture`) gets a new date
/// (`find_new_fixture_date`), goes back (`add_fixture`, 1) and posts the news with the old date (`queue_fixture_moved_news`, no cause).
/// Original: `FIXTURE_MANAGER::reschedule_club_fixtures_near_date` (0x5abb10, fix_man.cpp, thiscall, ret 8).
///
/// [pt] { 0x5abb10 } (thiscall, ret 8, fix_man.cpp, SEH da lista local): `FIXTURE_MANAGER::
/// reschedule_club_fixtures_near_date(clube, data)`, pelas convocações das seleções (0x777940: clube com mais de dois
/// convocados). Os jogos do clube (o id em `*clube`) de um dia antes a um dia depois da data (0x5acbe0, lado 1); cada
/// um que sai do calendário (0x5abc30) ganha data nova (0x5ab510), volta (0x5aa680, 1) e dá a notícia com a data velha
/// (0x5aac70, sem causa).
pub(crate) unsafe extern "thiscall" fn reschedule_club_fixtures_near_date(this: usize, club: usize, data: usize) {
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    date_sub_days(data, a.as_mut_ptr(), 1);
    fix_man_date_add_days(data, b.as_mut_ptr(), 1);
    find_club_fixtures_between(this, a.as_ptr(), b.as_ptr(), read_u32(club), lp);
    list_rewind(lp);
    loop {
        let p = contract_manager_list_next(lp);
        if p == 0 {
            break;
        }
        let mut old = [0u8; 8];
        date_from_day_year_ptr(old.as_mut_ptr(), read_u16(p + 0x2a) as u32, read_u16(p + 0x28) as u32);
        if fix_man_remove_fixture(this, p) != 0 {
            crate::comparison::branch("cpz_8 match rescheduled by national team");
            exe::fixture_manager_find_new_fixture_date(this, p);
            fix_man_add_fixture(this, p, 1);
            fix_man_queue_fixture_moved_news(this, p, Fixture(0), old.as_ptr() as usize);
        }
    }
    list_destroy(lp);
}

// simplification: moves matches in the calendar and gives the news (allocates): original only; the copy takes the branch.

/// The rehearsal of the news item. The copy builds the new news item here instead of putting it in the queue (and does
/// not create the queue).
///
/// [pt] O ensaio da notícia: a cópia monta a notícia nova aqui em vez de pô-la na fila (e não cria a fila).
pub(crate) static mut FIXTURE_MOVED_NEWS_DRY_RUN: bool = false;
pub(crate) static mut FIXTURE_MOVED_NEWS_BUILT: [u8; 0xe0] = [0; 0xe0];
pub(crate) static mut FIXTURE_MOVED_NEWS_WAS_BUILT: bool = false;

/// Data slot k of a news item (NEWSITEM get_slot).
pub(crate) unsafe fn news_slot(n: usize, k: u32) -> u32 {
    exe::newsitem_get_slot(n, k)
}

/// News item already in the list for the same fixture (slots 0,1,2,0xe,0xf vs fixture +0xc,+0x10,+0,+0x40,+0x41), or 0.
///
/// [pt] A notícia já na fila para o mesmo jogo (dados 0, 1, 2, 0xe e 0xf: +0xc, +0x10, +0, +0x40 e +0x41 do jogo).
pub(crate) unsafe fn find_queued_fixture_news(l: usize, f: usize) -> usize {
    list_rewind(l);
    loop {
        let it = contract_manager_list_next(l);
        if it == 0 {
            return 0;
        }
        if news_slot(it, 0) == read_u32(f + 0xc)
            && news_slot(it, 1) == read_u32(f + 0x10)
            && news_slot(it, 2) == read_u32(f)
            && news_slot(it, 0xe) as i32 == read_i8_as_i32(f + 0x40)
            && news_slot(it, 0xf) as i32 == read_i8_as_i32(f + 0x41)
        {
            return it;
        }
    }
}

/// Queues the "fixture moved" news: updates the new date of an existing news item for the same fixture, or queues news
/// 1 with the fixture and the cause or the old date.
///
/// `FIXTURE_MANAGER::queue_fixture_moved_news(fixture, cause, old date)`, from the reschedulings (`add_fixture`, `reschedule_club_fixtures_near_date`,
/// `resolve_same_day_clash`, `move_nearby_club_fixtures`); SEH for the local news. The queue at +0x1128 (created the first time: `exe::crt_operator_new` and `exe::new_llist`):
/// if there is already news for the same fixture, it only changes the new date (data 5 and 6); otherwise news 1: the
/// fixture (+0xc, +0x10, +0, +0x32, +0x30, the new date +0x2a/+0x28, +0x40, +0x41) and the cause (the same fields of it
/// in data 7 to 0xd) or, with no cause, −1 and the old date (data 0xc and 0xd); it goes into the queue (0x68d0d0,
/// 0xdf).
/// Original: `FIXTURE_MANAGER::queue_fixture_moved_news` (0x5aac70, fix_man.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5aac70 } (thiscall, ret 0xc, fix_man.cpp, SEH da notícia local): `FIXTURE_MANAGER::queue_fixture_moved_news(jogo,
/// causa, data velha)`, pelas remarcações (0x5aa680, 0x5abb10, 0x5ae380, 0x5ae9d0). A fila em +0x1128 (criada na
/// primeira vez: 0x944cf1 e 0x68d020): se já há a notícia do mesmo jogo, só troca a data nova (dados 5 e 6); senão a
/// notícia 1: o jogo (+0xc, +0x10, +0, +0x32, +0x30, a data nova +0x2a/+0x28, +0x40, +0x41) e a causa (os mesmos
/// campos dela nos dados 7 a 0xd) ou, sem causa, −1 e a data velha (dados 0xc e 0xd); entra na fila (0x68d0d0, 0xdf).
pub(crate) unsafe extern "thiscall" fn queue_fixture_moved_news(this: usize, f: usize, causa: usize, old: usize) {
    let f = Fixture(f);
    if read_usize(this + fixture_manager::MOVED_NEWS_QUEUE) == 0 && !FIXTURE_MOVED_NEWS_DRY_RUN {
        let m = exe::crt_operator_new(0xe);
        if m != 0 {
            list_init(m);
        }
        write_usize(this + fixture_manager::MOVED_NEWS_QUEUE, m);
    }
    let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    if l != 0 {
        let it = find_queued_fixture_news(l, f.addr());
        if it != 0 {
            exe::newsitem_set_slot(it, 5, f.day() as u32);
            exe::newsitem_set_slot(it, 6, f.year() as u32);
            return;
        }
    }
    crate::comparison::branch("cpz_8 rescheduled match news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 1, 0);
    let fields = |nt: usize, k: i8, j: usize| {
        exe::newsitem_set_slot(nt, k, read_u32(j + 0xc));
        exe::newsitem_set_slot(nt, k + 1, read_u32(j + 0x10));
        exe::newsitem_set_slot(nt, k + 2, read_u32(j));
        exe::newsitem_set_slot(nt, k + 3, read_i16_as_i32(j + 0x32) as u32);
        exe::newsitem_set_slot(nt, k + 4, read_i16_as_i32(j + 0x30) as u32);
        exe::newsitem_set_slot(nt, k + 5, read_u16(j + 0x2a) as u32);
    };
    fields(nt, 0, f.addr());
    exe::newsitem_set_slot(nt, 6, f.year() as u32);
    exe::newsitem_set_slot(nt, 0xe, ((f.home_side_mark() as i8) as i32) as u32);
    exe::newsitem_set_slot(nt, 0xf, (f.away_side_mark() as i32) as u32);
    if causa != 0 {
        fields(nt, 7, causa);
        exe::newsitem_set_slot(nt, 0xd, read_u16(causa + 0x28) as u32);
    } else {
        exe::newsitem_set_slot(nt, 7, u32::MAX);
        exe::newsitem_set_slot(nt, 0xc, read_i16_as_i32(old) as u32);
        exe::newsitem_set_slot(nt, 0xd, read_u16(old + 2) as u32);
    }
    if FIXTURE_MOVED_NEWS_DRY_RUN {
        core::ptr::copy_nonoverlapping(nt as *const u8, core::ptr::addr_of_mut!(FIXTURE_MOVED_NEWS_BUILT) as *mut u8, 0xe0);
        FIXTURE_MOVED_NEWS_WAS_BUILT = true;
        return;
    }
    exe::llist_add_tail(read_usize(this + fixture_manager::MOVED_NEWS_QUEUE), nt, 0xdf);
}

/// The calendar block for the year of match `f` (+0x28, word), walking the calendar chain from `fm`.
///
/// This is the loop shared by `find_stored_fixture`, `insert_fixture` and `remove_fixture`: the next block (+0x112c) of another year is moved to the
/// requested year (`set_year` set_year); with no next block, a new block (`exe::crt_operator_new`, 0x1184 bytes; `exe::new_fixture_manager` with the year,
/// the previous block and 0) joins the chain.
///
/// Returns 0 if memory ran out (the chain keeps 0 at +0x112c, as in the original routine).
///
/// [pt] O bloco do ano do jogo `f` (+0x28, palavra) na corrente do calendário a partir de `fm` (o laço comum das 0x5ac7a0,
/// 0x5ab230 e 0x5abc30): o bloco seguinte (+0x112c) de outro ano é levado ao ano pedido (0x5aa2d0 set_year); sem
/// seguinte, um bloco novo (0x944cf1, 0x1184 bytes; 0x5a9cf0 com o ano, o bloco de antes e 0) entra na corrente. 0 se
/// faltou memória (a corrente fica com 0 em +0x112c, como na original).
pub(crate) unsafe fn calendar_year_block(fm: usize, f: Fixture) -> usize {
    let mut b = fm;
    loop {
        let year = f.year();
        if year == read_u16(b + 0x1130) {
            return b;
        }
        let next = read_usize(b + 0x112c);
        if next != 0 {
            if read_u16(next + 0x1130) != year {
                // simplification: the original pushes eax with the year in the low word and register leftovers on top; the
                // 0x5aa2d0 compares the word (the path only runs at the calendar year turn)
                exe::fixture_manager_set_year(next, year as u32);
            }
        } else {
            let p = exe::crt_operator_new((0x1184) as u32);
            // the original’s ecx was the null +0x112c: the year is passed clean
            let r = if p != 0 { exe::new_fixture_manager(p, year as u32, b, 0) } else { 0 };
            write_u32(b + 0x112c, r as u32);
            if r == 0 {
                return 0;
            }
        }
        b = read_usize(b + 0x112c);
    }
}

/// Fixture list in block b for the fixture's day (+0x2a), time slice (+0x3f) and key nation.
///
/// [pt] A lista do dia, fatia (+0x3f) e nação-chave do jogo `f` no bloco `b`: [[b + (dia +0x2a × 3 + fatia) × 4] + nação × 4],
/// a nação pela 0x48dd70 (competição +0x14, o jogo), com sinal.
pub(crate) unsafe fn fixture_list_in_block(b: usize, f: Fixture) -> usize {
    let n = (exe::comp_active_key_nation_of_fixture(f.competition(), f) as i8) as isize;
    let k = f.day() as usize * 3 + f.time_slice() as usize;
    read_usize((read_usize(b + k * 4) as isize + n * 4) as usize)
}

/// Returns the stored copy in the calendar whose 0x4f bytes match the fixture's, or 0 if none.
///
/// `FIXTURE_MANAGER::find_stored_fixture(fixture)`: the copy stored in the calendar whose 0x4f bytes equal the
/// fixture's, in the list of its day, slice and key nation (the year block through the chain; the list cursor goes back
/// to the start and walks). Null fixture or none equal: 0; out of memory for the year block: the box (line 0x7cf) and
/// 0.
/// Original: `FIXTURE_MANAGER::find_stored_fixture` (0x5ac7a0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ac7a0 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::find_stored_fixture(jogo)`: a cópia guardada no
/// calendário cujos 0x4f bytes são iguais aos do jogo, na lista do dia, fatia e nação-chave dele (o bloco do ano pela
/// corrente; o cursor da lista volta ao começo e anda). Jogo nulo ou nenhum igual: 0; sem memória para o bloco do ano:
/// a caixa (linha 0x7cf) e 0.
pub(crate) unsafe extern "thiscall" fn find_stored_fixture(this: usize, f: usize) -> usize {
    if f == 0 {
        return 0;
    }
    let b = calendar_year_block(this, Fixture(f));
    if b == 0 {
        show_error_at(0x9cc208, 0x7cf);
        return 0;
    }
    let l = fixture_list_in_block(b, Fixture(f));
    write_u32(l + 0xa, read_usize(l + 2) as u32);
    let target = core::slice::from_raw_parts(f as *const u8, 0x4f);
    let mut item = list_next_item(l);
    while item != 0 {
        if core::slice::from_raw_parts(item as *const u8, 0x4f) == target {
            crate::comparison::branch("cps_11 found the stored match");
            return item; // cps11: the saved copy
        }
        item = list_next_item(l);
    }
    0
}

/// Finds (or creates) the fixture's year block and appends a copy of the fixture to its day, slice and key-nation list.
///
/// `FIXTURE_MANAGER::insert_fixture(fixture)`: appends a copy of the fixture's 0x4f bytes (0x68d0d0) to the list of its
/// day, slice and key nation in the year block (through the chain). Out of memory for the block: the box (line 0x3ff);
/// a failed copy: the box (line 0x3eb).
/// Original: `FIXTURE_MANAGER::insert_fixture` (0x5ab230, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ab230 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::insert_fixture(jogo)`: acrescenta uma cópia dos 0x4f
/// bytes do jogo (0x68d0d0) à lista do dia, fatia e nação-chave dele no bloco do ano (pela corrente). Sem memória para o
/// bloco: a caixa (linha 0x3ff); a cópia que falha: a caixa (linha 0x3eb).
pub(crate) unsafe extern "thiscall" fn insert_fixture(this: usize, f: usize) {
    let b = calendar_year_block(this, Fixture(f));
    if b == 0 {
        return show_error_at(0x9cc208, 0x3ff);
    }
    let l = fixture_list_in_block(b, Fixture(f));
    if exe::llist_add_tail(l, f, 0x4f) == 0 { // cps11: guardar
        return show_error_at(0x9cc208, 0x3eb);
    }
    crate::comparison::branch("cps_11 stored match");
}

/// Removes the first item with the same bytes from the fixture's day, slice and key-nation list; returns 1 if removed.
///
/// `FIXTURE_MANAGER::remove_fixture(fixture)`: removes from the list of the fixture's day, slice and key nation (in the
/// year block, through the chain) the first item with the same bytes (`manager_manager_remove`). Returns 1 if removed, 0 if not; out of
/// memory for the block: the box (line 0x5ef) and 0.
/// Original: `FIXTURE_MANAGER::remove_fixture` (0x5abc30, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5abc30 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::remove_fixture(jogo)`: tira da lista do dia, fatia e
/// nação-chave do jogo (no bloco do ano, pela corrente) o primeiro item com os mesmos bytes (0x68d1f0). Devolve 1 se
/// tirou, 0 se não; sem memória para o bloco: a caixa (linha 0x5ef) e 0.
pub(crate) unsafe extern "thiscall" fn remove_fixture(this: usize, f: usize) -> u32 {
    let b = calendar_year_block(this, Fixture(f));
    if b == 0 {
        show_error_at(0x9cc208, 0x5ef);
        return 0;
    }
    let r = exe::llist_remove(fixture_list_in_block(b, Fixture(f)), f); // cps11: tirar
    if r != 0 {
        crate::comparison::branch("cps_11 match removed");
    }
    (r != 0) as u32
}

/// Fixture list of the fixture in the calendar (year block, then list); 0 on out of memory.
///
/// [pt] A lista do jogo no calendário, achada antes da original (o laço da corrente faz o mesmo que ela faria: depois dele
/// a original acha o bloco do ano de primeira); 0 sem memória.
pub(crate) unsafe fn find_fixture_list(fm: usize, f: usize) -> usize {
    let b = calendar_year_block(fm, Fixture(f));
    if b == 0 { 0 } else { fixture_list_in_block(b, Fixture(f)) }
}

/// Fixture list for (year, day, slice, key nation) in calendar fm (`get_fixture_list`, creates the year block if missing).
///
/// [pt] A lista dos jogos de um ano, dia, fatia e nação-chave no calendário `fm` (0x5abf10; cria o bloco do ano se a
/// corrente não o tem). A original compara o ano como palavra e mascara o dia, a fatia e a nação.
pub(crate) unsafe fn fix_man_get_fixture_list(fm: usize, year: u32, day: u32, slice: u32, nation: u32) -> usize {
    exe::fixture_manager_get_fixture_list(fm, year, day, slice, nation)
}

/// Rewinds the LLIST cursor and returns the first item.
///
/// [pt] Põe o cursor da lista no começo (+0xa = +2) e devolve o primeiro item.
pub(crate) unsafe fn list_first(l: usize) -> usize {
    write_u32(l + 0xa, read_u32(l + 2));
    list_next_item(l)
}

/// Appends a copy of the 0x4f-byte fixture to the list (0x68d0d0); 0 on failure.
///
/// [pt] Acrescenta à lista `l` uma cópia dos 0x4f bytes do jogo `f` (0x68d0d0); 0 se falhou.
pub(crate) unsafe fn list_append_fixture(l: usize, f: usize) -> usize {
    exe::llist_add_tail(l, f, 0x4f)
}

/// Active key nation of competition `comp` (`comp_active_key_nation_of_fixture` on its club-comp record, no fixture).
///
/// [pt] A nação-chave ativa da competição `comp` (0x48dd70 sobre o registro [0xae23d0] + 0x6b × comp, sem jogo).
pub(crate) unsafe fn comp_key_nation(comp: u32) -> u8 {
    (exe::comp_active_key_nation_of_fixture((read_u32(G_CLUB_COMP_TABLE).wrapping_add(comp.wrapping_mul(0x6b))) as usize, 0) as u32) as u8
}

/// Fixture passes the `find_fixtures` filters: type (+0x42), club home/away with its side (+0x40/+0x41) and competition (+0);
/// wildcards allowed.
///
/// [pt] O jogo `f` passa nos filtros da 0x5acbe0: tipo (+0x42; 0xfe qualquer), clube em casa (+0xc) ou fora (+0x10; −1
/// qualquer) com o lado dele (+0x40 em casa, +0x41 fora; 0xff qualquer) e a competição (+0; −1 qualquer).
pub(crate) unsafe fn fixture_matches_filter(f: Fixture, club: u32, comp: u32, kind: u8, side: u8) -> bool {
    if kind != 0xfe && kind != f.stage_index() {
        return false;
    }
    if club != u32::MAX {
        let home = (f.home_club_id() as u32);
        if club != home && club != (f.away_club_id() as u32) {
            return false;
        }
        if side != 0xff {
            let o_dele = if club == home { f.home_side_mark() } else { (f.away_side_mark() as u8) };
            if o_dele != side {
                return false;
            }
        }
    }
    comp == u32::MAX || comp == read_u32(f.addr())
}

/// Counts (and copies into the list, if given) the calendar fixtures between two dates in the key-nation lists of the
/// club or competition, with type, club side and competition filters.
///
/// `FIXTURE_MANAGER::find_fixtures(day0, year0, day1, year1, club, list, comp, type, side)`: counts (and, with `list`,
/// copies into it) the calendar fixtures between (day0, year0) and (day1, year1) inclusive, day by day (366 per year),
/// in the three slices of each day and in the key-nation lists: the club's (index into [0xae23b4], 0x245 bytes; a
/// national team `exe::database_is_national_team`: nations 0 and 1; otherwise 2..7, the one from `comp_active_key_nation_of_club` and the partner `fix_man_get_partner_key_nation_index`, without
/// repeats) or only the competition's (`comp_active_key_nation_of_fixture`). Filters: type (0xfe any), club side (0xff any) and competition (−1
/// any). Returns the count (word); a failed copy shows the box (line 0x93b) and stops there. Hot (288 thousand calls up
/// to 2002-06-01): no counter.
/// Original: `FIXTURE_MANAGER::find_fixtures` (0x5acbe0, fix_man.cpp, thiscall, ret 0x24).
///
/// [pt] { 0x5acbe0 } (thiscall, ret 0x24, fix_man.cpp): `FIXTURE_MANAGER::find_fixtures(dia0, ano0, dia1, ano1, clube, lista,
/// comp, tipo, lado)`: conta (e, com `lista`, copia para ela) os jogos do calendário entre (dia0, ano0) e (dia1, ano1)
/// inclusive, dia a dia (366 por ano), nas três fatias de cada dia e nas listas das nações-chave: do clube (índice em
/// [0xae23b4], 0x245 bytes; seleção 0x53b900: as nações 0 e 1; senão 2..7, a da 0x48dee0 e a parceira 0x5af5b0, sem
/// repetir) ou só a da competição (0x48dd70). Filtros: tipo (0xfe qualquer), lado do clube (0xff qualquer) e
/// competição (−1 qualquer). Devolve a conta (palavra); a cópia que falha mostra a caixa (linha 0x93b) e para ali.
/// Quente (288 mil chamadas até 2002-06-01): sem contador.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, club: u32,
                                                      list: usize, comp: u32, kind: u32, side: u32) -> u32 {
    let (kind, side) = (kind as u8, side as u8);
    let mut nations = [0xffu8; 8];
    let mut n = 0usize;
    if club != u32::MAX {
        let reg = club_at(club as isize).addr();
        if exe::database_is_national_team(reg) != 0 {
            nations[0] = 0;
            nations[1] = 1;
            n = 2;
        } else {
            nations[..6].copy_from_slice(&[2, 3, 4, 5, 6, 7]);
            nations[6] = (exe::comp_active_key_nation_of_club(reg) as u32) as u8;
            nations[7] = (exe::fix_man_get_partner_key_nation_index(reg) as u32) as u8;
            n = 8;
            for i in 0..7 {
                if (i + 1..8).any(|j| nations[j] == nations[i]) {
                    nations[i] = 0xff; // repetida adiante
                }
            }
        }
    } else if (comp as i32) > -1 {
        nations[0] = comp_key_nation(comp);
        n = 1;
    }
    let (mut dia, mut ano, mut total) = (dia0, ano0, 0u32);
    loop {
        if dia as u16 >= 0x16e {
            ano = ano.wrapping_add(1); // the next year, from day 0
            dia = 0;
            continue;
        }
        if ano as u16 > ano1 as u16 || (ano as u16 == ano1 as u16 && dia as u16 > dia1 as u16) {
            return total & 0xffff; // past the last day
        }
        for fatia in 0..3u32 {
            for &nacao in &nations[..n] {
                if nacao == 0xff {
                    continue;
                }
                let l = fix_man_get_fixture_list(this, ano, dia, fatia, nacao as u32);
                if l == 0 {
                    continue;
                }
                let mut f = list_first(l);
                while f != 0 {
                    if fixture_matches_filter(Fixture(f), club, comp, kind, side) {
                        if list != 0 && list_append_fixture(list, f) == 0 {
                            show_error_at(0x9cc208, 0x93b);
                            return total & 0xffff;
                        }
                        total = total.wrapping_add(1);
                    }
                    f = list_next_item(l);
                }
            }
        }
        dia = dia.wrapping_add(1);
    }
}

/// Copies into the list the fixtures of one day and slice for the requested competition, round and type, and returns
/// how many.
///
/// `FIXTURE_MANAGER::collect_comp_fixtures_on_day(year, day, slice, comp, round, type, list)` (from the leagues' TV
/// postponement and from moving cup rounds to the fixed dates): copies into `list` the fixtures of the day and slice,
/// in the list of the competition's key nation (`comp_active_key_nation_of_fixture`; −1: nation 0), of the competition (+0; −1 any), of the round
/// (+0x34, word; −1 any) and of the type (+0x42; 0xfe any). Returns how many (word); a failed copy shows the box (line
/// 0x634) and stops there.
/// Original: `FIXTURE_MANAGER::collect_comp_fixtures_on_day` (0x5abd90, fix_man.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x5abd90 } (thiscall, ret 0x1c, fix_man.cpp): `FIXTURE_MANAGER::collect_comp_fixtures_on_day(ano, dia, fatia,
/// comp, rodada, tipo, lista)` (do adiamento pela TV das ligas e da mudança das rodadas das copas para as datas fixas):
/// copia para `lista` os jogos do dia e fatia, na lista da nação-chave da competição (0x48dd70; −1: a nação 0), da
/// competição (+0; −1 qualquer), da rodada (+0x34, palavra; −1 qualquer) e do tipo (+0x42; 0xfe qualquer). Devolve
/// quantos (palavra); a cópia que falha mostra a caixa (linha 0x634) e para ali.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn collect_comp_fixtures_on_day(this: usize, year: u32, day: u32, slice: u32, comp: u32, round: u32,
                                                       kind: u32, list: usize) -> u32 {
    let kind = kind as u8;
    // simplification: the original pushes the nation local with high bytes of stack leftovers; 0x5abf10 only reads the byte
    let nation = if comp != u32::MAX { comp_key_nation(comp) } else { 0 };
    let mut total = 0u32;
    let l = fix_man_get_fixture_list(this, year, day, slice, nation as u32);
    if l == 0 {
        return 0;
    }
    let mut f = list_first(l);
    while f != 0 {
        let mut ok = true;
        if comp != u32::MAX && comp != read_u32(f) {
            ok = false;
        }
        if round != u32::MAX && round as i32 != read_i16(f + 0x34) as i32 {
            ok = false; // another round
        }
        if (kind == 0xfe || read_u8(f + 0x42) == kind) && ok {
            if list_append_fixture(list, f) == 0 {
                show_error_at(0x9cc208, 0x634);
                return total & 0xffff;
            }
            crate::comparison::branch("cpr_12 round match");
            total = total.wrapping_add(1);
        }
        f = list_next_item(l);
    }
    total & 0xffff
}

/// Tells whether the competition has a fixture on the date in any of the three calendar slices.
///
/// `fix_man_comp_has_fixture_on_date(date, comp)` (stdcall, ret 8; the `ecx` = [0xae2a58] set by the caller is not
/// used), from the European Championship qualifiers (0x57fd60): 1 if the competition has a fixture (+0 = comp) on the
/// date in any of the three slices of the calendar [0xae2a58], in its key nation's list (`comp_active_key_nation_of_fixture`). Competition −1 or
/// below: 0; null date: the box (line 0xe89) and 0.
/// Original: `fix_man_comp_has_fixture_on_date` (0x5ae6d0, fix_man.cpp, stdcall, ret 8).
///
/// [pt] { 0x5ae6d0 } (stdcall, ret 8, fix_man.cpp; o `ecx` = [0xae2a58] que o chamador põe não é usado):
/// `fix_man_comp_has_fixture_on_date(data, comp)` (das eliminatórias da Eurocopa 0x57fd60): 1 se a competição tem
/// jogo (+0 = comp) na data em alguma das três fatias do calendário [0xae2a58], na lista da nação-chave dela
/// (0x48dd70). Competição −1 ou abaixo: 0; data nula: a caixa (linha 0xe89) e 0.
pub(crate) unsafe extern "stdcall" fn fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    if (comp as i32) <= -1 {
        return 0;
    }
    let nation = comp_key_nation(comp);
    if data == 0 {
        show_error_at(0x9cc208, 0xe89);
        return 0;
    }
    let fm = read_u32(G_FIXTURE_MANAGER) as usize;
    // simplification: the original pushes the year and the day (words) with leftover high bytes in the registers; 0x5abf10
    // compares the year as a word and masks the day (the year dword only follows if the year block is missing, and the two years of the
    // career are in the chain since the game’s creation)
    let (ano, dia) = (read_u16(data + 2) as u32, read_u16(data) as u32);
    for fatia in 0..3u32 {
        let l = fix_man_get_fixture_list(fm, ano, dia, fatia, nation as u32);
        if l == 0 {
            continue;
        }
        let mut f = list_first(l);
        while f != 0 {
            if read_u32(f) == comp {
                return 1;
            }
            f = list_next_item(l);
        }
    }
    0
}

/// True if the items the original appended to `lista` (after the old tail) equal those of the temporary list `nova`, in
/// order, 0x4f bytes each.
///
/// [pt] Os itens que a original acrescentou à `lista` (depois da cauda de antes) são os da lista provisória `nova`, na
/// mesma ordem, 0x4f bytes cada?
pub(crate) unsafe fn appended_items_match(list: usize, count_before: u16, tail_before: u32, new: usize) -> bool {
    let acrescentados = read_u16(list).wrapping_sub(count_before);
    if acrescentados != read_u16(new) {
        return false;
    }
    let mut a = if count_before == 0 { read_u32(list + 2) } else { read_u32(tail_before as usize + 6) } as usize;
    let mut b = read_u32(new + 2) as usize;
    for _ in 0..acrescentados {
        if a == 0 || b == 0 {
            return false;
        }
        let (da, db) = (read_u32(a) as usize, read_u32(b) as usize);
        if core::slice::from_raw_parts(da as *const u8, 0x4f) != core::slice::from_raw_parts(db as *const u8, 0x4f) {
            return false;
        }
        a = read_u32(a + 6) as usize;
        b = read_u32(b + 6) as usize;
    }
    true
}

/// Runs the port with a temporary LLIST and checks its items against those the original appended to the real list;
/// returns (result, equal).
///
/// [pt] Roda a cópia com uma lista provisória (0x68d020; destruída pela 0x68d040) e confere os itens contra os que a
/// original acrescentou à lista real.
pub(crate) unsafe fn run_with_temp_list(list: usize, count_before: u16, tail_before: u32, copy: impl FnOnce(usize) -> u32) -> (u32, bool) {
    let mut new = [0u8; 0x10];
    let p = new.as_mut_ptr() as usize;
    exe::new_llist(p);
    let n = copy(p);
    let equal = appended_items_match(list, count_before, tail_before, p);
    exe::drop_llist(p);
    (n, equal)
}

/// Fixture list for (year, day, slice, group) via `get_fixture_list` with the cursor rewound; 0 if null.
///
/// [pt] A lista das partidas de (ano, dia, fatia, grupo) no gerenciador `this` (0x5abf10, thiscall ret 0x10, original:
/// carrega o ano se preciso), com o cursor voltado ao começo (+0xa = +2), como as duas originais fazem; nula: 0.
pub(crate) unsafe fn get_fixture_list_rewound(this: usize, year: u32, day: u32, slice: u32, group: u32) -> usize {
    let l = exe::fixture_manager_get_fixture_list(this, year, day, slice, group);
    if l != 0 {
        write_u32(l + 0xa, read_u32(l + 2));
    }
    l
}

/// Next item of a LLIST (`next_list_item`) or 0.
///
/// [pt] O próximo item da lista `l` (0x68d1b0, next_list_item), ou 0.
pub(crate) unsafe fn list_next_item(l: usize) -> usize {
    exe::llist_get_next(l)
}

/// Fixture passes find_next_fixture's filter: competition (+0), stage (+0x34) and both clubs (home +0xc or away +0x10),
/// each -1 = any.
///
/// [pt] A partida `f` passa pelo filtro da find_next_fixture? Cada filtro em -1 deixa passar: a competição (+0), a fase
/// (+0x34, curto com sinal) e os dois clubes (cada um o mandante +0xc ou o visitante +0x10).
pub(crate) unsafe fn fixture_matches_next_filter(f: Fixture, comp: i32, stage: i32, clube_a: i32, club_b: i32) -> bool {
    let club = |c: i32| c == -1 || c == (f.home_club_id() as i32) || c == (f.away_club_id() as i32);
    (comp == -1 || comp == f.comp_id()) // the competition
        && (stage == -1 || stage == f.round_number() as i32)
        && club(clube_a)
        && club(club_b)
}

/// Finds the next (or previous) fixture from a year/day/slice cursor, inclusive, matching the competition, stage and
/// two club filters; writes the cursor back and returns the fixture, or 0.
///
/// `FIXTURE_MANAGER::find_next_fixture(year*, day*, slice*, forward, comp, stage, club_a, club_b, ...)`: the fixture
/// after (or before, `forward` 0) the cursor (year, day, slice), the cursor itself included, that passes the filter
/// (fixture_matches_next_filter: competition +0, stage +0x34 and the two clubs +0xc/+0x10, each -1 = any). The group of
/// lists comes from `comp_active_key_nation_of_fixture` (the competition of table [0xae23d0] + 107 × comp, 0). Forward: slices up to 2, days up to
/// 365 (0x16e exclusive), following years from day 0 (the slice only goes back to 0 when the day turns); backward:
/// slices down to 0, then 2, days down to 0, previous years from day 0x16d. Five years from the cursor's year (forward:
/// signed less than unsigned; backward: signed), 0. Found: it writes the cursor (day, year and the slice as a byte) and
/// returns the fixture. The last 3 arguments are not read. The original passes `get_fixture_list` the year, day and slice with
/// the high half of locals that were never written (garbage), but it and what it calls (`set_year`, `exe::new_fixture_manager`, `exe::date_is_leap_year`)
/// only use the word/byte: the port passes zero. Used in the cup draws (Cup.cpp 0x519d90, `exe::cup_process_fixture`, `exe::cup_label_round_extra_fixtures`). No SEH,
/// no box (`get_fixture_list`'s only when out of memory).
/// Original: `FIXTURE_MANAGER::find_next_fixture` (0x5ac920, fix_man.cpp, thiscall, ret 0x2c).
///
/// [pt] { 0x5ac920 } (thiscall, ret 0x2c, fix_man.cpp): `FIXTURE_MANAGER::find_next_fixture(ano*, dia*, fatia*, adiante,
/// comp, fase, clube_a, clube_b, ...)`: a partida seguinte (ou anterior, `adiante` 0) ao cursor (ano, dia, fatia),
/// o próprio cursor incluído, que passa pelo filtro (fixture_matches_next_filter). O grupo das listas vem da 0x48dd70 (a competição
/// da tabela [0xae23d0] + 107 × comp, 0). Adiante: fatias até 2, dias até 365 (0x16e exclusivo), anos seguintes no
/// dia 0 (a fatia só volta a 0 na virada do dia); atrás: fatias até 0, depois 2, dias até 0, anos anteriores no dia
/// 0x16d. A cinco anos do ano do cursor (adiante: com sinal menos sem sinal; atrás: com sinal), 0. Achou: grava o
/// cursor (dia, ano e a fatia em byte) e devolve a partida. Os 3 últimos argumentos não são lidos. A original passa
/// à 0x5abf10 ano, dia e fatia com a metade alta de locais nunca escritas (lixo), mas ela e o que chama (0x5aa2d0,
/// 0x5a9cf0, 0x54cfe0) só usam a palavra/o byte: a cópia passa zero. Nos sorteios de copa (Cup.cpp 0x519d90,
/// 0x51a150, 0x522c70). Sem SEH, sem caixa (a da 0x5abf10 só sem memória).
pub(crate) unsafe extern "thiscall" fn find_next_fixture(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, stage: i32, club: i32, club_b: i32, _a9: u32, _a10: u32, _a11: u32) -> usize {
    let table = read_usize(G_CLUB_COMP_TABLE);
    let group = (exe::comp_active_key_nation_of_fixture(table.wrapping_add((comp as usize).wrapping_mul(107)), 0) as u32) as u8 as u32;
    let (mut year, mut day, mut slice) = (*pano, *pdia, *pfatia as u16);
    let find = |year: u16, day: u16, slice: u16| -> usize {
        let l = get_fixture_list_rewound(this, year as u32, day as u32, slice as u32, group);
        if l == 0 {
            return 0;
        }
        loop {
            let f = list_next_item(l);
            if f == 0 || fixture_matches_next_filter(Fixture(f), comp, stage, club, club_b) {
                return f;
            }
        }
    };
    let f = 'busca: {
        if adiante != 0 {
            loop {
                while (day as i16) < 0x16e {
                    while (slice as i16) < 3 {
                        let f = find(year, day, slice);
                        if f != 0 {
                            break 'busca f;
                        }
                        slice = slice.wrapping_add(1);
                    }
                    day = day.wrapping_add(1);
                    slice = 0; // o dia seguinte
                }
                year = year.wrapping_add(1);
                day = 0;
                if (year as i16 as i32) - (*pano as i32) >= 5 {
                    return 0;
                }
            }
        } else {
            loop {
                while (day as i16) >= 0 {
                    while (slice as i16) >= 0 {
                        let f = find(year, day, slice);
                        if f != 0 {
                            break 'busca f;
                        }
                        slice = slice.wrapping_sub(1);
                    }
                    slice = 2; // o dia anterior
                    day = day.wrapping_sub(1);
                }
                year = year.wrapping_sub(1);
                day = 0x16d;
                if (*pano as i16 as i32) - (year as i16 as i32) >= 5 {
                    return 0;
                }
            }
        }
    };
    *pdia = day;
    *pano = year;
    *pfatia = slice as u8;
    f
}

/// Returns 1 when a fixture in the day lists uses the stadium, in the given slice or in slices 0 to 2 (slice -1).
///
/// `FIXTURE_MANAGER::is_stadium_in_use(stadium, day, year, slice)`: 1 if any fixture (+0x18 = the stadium, not null) in
/// the day's lists uses the stadium. The groups: 0 to 7 and the stadium's (`comp_active_key_nation_of_stadium`, byte), the latter skipped if it
/// repeats one of the eight. Slice -1 (byte): slices 0 to 2, otherwise only that one. The slice goes to `get_fixture_list` in the
/// low byte of the year argument (the original keeps the loop there: the 3 high bytes are the year's). Null stadium: 0.
/// Used when scheduling friendlies (friendly.cpp `add_friendly_match`, 0x5c75a0) and in `exe::reservecomp_schedule_reserve_fixtures`, `exe::reservecomp_add_b_or_under21_fixture`. No SEH, no locals
/// without a value.
/// Original: `FIXTURE_MANAGER::is_stadium_in_use` (0x5ae870, fix_man.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5ae870 } (thiscall, ret 0x10, fix_man.cpp): `FIXTURE_MANAGER::is_stadium_in_use(estadio, dia, ano, fatia)`:
/// 1 se alguma partida (+0x18 = o estádio, não nulo) das listas do dia usa o estádio. Os grupos: 0 a 7 e o do estádio
/// (0x48df40, byte), este fora se repete um dos oito. Fatia -1 (byte): as fatias 0 a 2, senão só ela. A fatia vai à
/// 0x5abf10 no byte baixo do argumento do ano (a original guarda o laço lá: os 3 bytes altos são os do ano). Estádio
/// nulo: 0. Na marcação dos amistosos (friendly.cpp 0x5c6f00, 0x5c75a0) e em 0x7e6380, 0x7e6720. Sem SEH, sem locais
/// sem valor.
pub(crate) unsafe extern "thiscall" fn is_stadium_in_use(this: usize, stadium: usize, day: u32, year: u32, slice: u32) -> u32 {
    if stadium == 0 {
        return 0;
    }
    let mut groups = [0u8, 1, 2, 3, 4, 5, 6, 7, (exe::comp_active_key_nation_of_stadium(stadium) as u32) as u8];
    for i in 0..8 {
        for j in i + 1..9 {
            if groups[i] as i8 >= 0 && groups[i] == groups[j] {
                groups[j] = 0xff;
            }
        }
    }
    let (ini, fim) = if slice as u8 == 0xff { (0i8, 2i8) } else { (slice as i8, slice as i8) };
    for &g in &groups {
        if g == 0xff {
            continue;
        }
        let mut s = ini;
        while s <= fim {
            let l = get_fixture_list_rewound(this, year, day, (year & 0xffff_ff00) | s as u8 as u32, g as u32);
            if l != 0 {
                loop {
                    let f = list_next_item(l);
                    if f == 0 {
                        break;
                    }
                    let e = read_usize(f + 0x18);
                    if e != 0 && e == stadium { // the match stadium
                        return 1;
                    }
                }
            }
            s = s.wrapping_add(1);
        }
    }
    0
}

compare_value!(compare_is_stadium_in_use, is_stadium_in_use, IDX_IS_STADIUM_IN_USE, (estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32);

/// Calls the original routine (ret 4) with eax = year and ecx = this, as its only caller does.
///
/// [pt] A original (ret 4) com eax = ano, como no único chamador.
pub(crate) unsafe fn call_original_with_year(o: usize, this: usize, year: u32) -> u32 {
    let r: u32;
    core::arch::asm!(
        "push {a}",
        "call {f}",
        a = in(reg) year,
        f = in(reg) o,
        inout("eax") year => r,
        inout("ecx") this => _,
        clobber_abi("C"),
    );
    r
}

/// Returns the fixture's time slice (1 weekend, 2 otherwise) by the competition's nation and the weekday.
///
/// `this` is not read. The time slice of a fixture of competition `comp` on day `day` (word): the Netherlands, Ireland,
/// Belgium and France (nation id +0x5d in [0x9cf338], [0x9cf2dc], [0x9cf244], [0x9cf300]) only have 1 on day 6; the
/// others, 1 on days 5 and 6; otherwise 2. Returns only `al`.
/// Original: `FIXTURE_MANAGER::get_time_slice` (0x5af2f0, fix_man.cpp, thiscall).
///
/// [pt] { 0x5af2f0 } (thiscall, ret 8, fix_man.cpp; `this` não é lido): a faixa de horário do jogo da
/// competição `comp` no dia `dia` (palavra): Holanda, Irlanda, Bélgica e França (id da nação +0x5d
/// em [0x9cf338], [0x9cf2dc], [0x9cf244], [0x9cf300]) só têm 1 no dia 6; as outras, 1 nos dias 5 e 6;
/// senão 2. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn get_time_slice(_this: usize, comp: usize, day: u32) -> u8 {
    let comp = Competition(comp);
    let day = day as i16;
    if !comp.is_null() {
        let nation = comp.nation();
        if !nation.is_null() {
            let id = read_u32(nation.addr());
            if [KEY_NATION_HOLLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_BELGIUM, KEY_NATION_FRANCE].iter().any(|&g| id == read_u32(g)) {
                return if day != 6 { 2 } else { 1 };
            }
        }
    }
    if day == 6 || day == 5 { 1 } else { 2 }
}

/// With the stadium in a third-world country, gives the visiting clubs the illness risk on the day before the fixture.
///
/// `FIXTURE_MANAGER::add_away_trip_illness_risk(fixture)`; `this` is not used. With the fixture's stadium (+0x18), its
/// city (+0x38) and the city's nation (+0x1f) in a third-world country (+0x7e = 3), each visiting club from another
/// nation (+0x53) — the home club (+0x1c, id +0xc) and the away club (+0x20, id +0x10) — gets the illness risk on the
/// day before the fixture.
/// Original: `FIXTURE_MANAGER::add_away_trip_illness_risk` (0x5ab3c0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ab3c0 } (thiscall, ret 4, fix_man.cpp; o `this` não é usado): `FIXTURE_MANAGER::add_away_trip_illness_risk(jogo)`:
/// com o estádio do jogo (+0x18), a cidade (+0x38) e a nação dela (+0x1f) num país do terceiro mundo (+0x7e = 3), cada
/// clube visitante de outra nação (+0x53) — o mandante (+0x1c, id +0xc) e o visitante (+0x20, id +0x10) — ganha o risco
/// de doença na véspera do jogo.
pub(crate) unsafe extern "thiscall" fn add_away_trip_illness_risk(_this: usize, f: usize) {
    let f = Fixture(f);
    let stadium = f.stadium();
    if stadium == 0 {
        return;
    }
    let city = read_usize(stadium + stadium::CITY);
    if city == 0 {
        return;
    }
    let nation = Nation(read_usize(city + city::NATION));
    if nation.is_null() {
        return;
    }
    if nation.state_of_development() != 3 { // cps13: só o terceiro mundo
        return;
    }
    let home = f.home_club();
    if !home.is_null() && home.nation() != nation {
        crate::comparison::branch("cps_13 home side illness");
        // the original pushes the year in the club’s eax and the day in the nation’s ecx (high halves of the pointers)
        add_illness_risk_before_match(f, f.home_club_id(), read_usize(read_usize(f.stadium() + 0x38) + 0x1f), nation.addr() as u32, home.addr() as u32);
    }
    let away = f.away_club();
    if !away.is_null() && away.nation().addr() != read_usize(read_usize(f.stadium() + 0x38) + 0x1f) {
        crate::comparison::branch("cps_13 away side illness");
        // the year in the stadium’s ecx and the day in the city’s edx
        let (e, c) = (f.stadium(), read_usize(f.stadium() + 0x38));
        add_illness_risk_before_match(f, f.away_club_id(), read_usize(c + city::NATION), c as u32, e as u32);
    }
}

/// For an English or Welsh club, returns the active key-nation index of the other nation; otherwise 0xff.
///
/// `fix_man_get_partner_key_nation_index(club)` (cdecl), from the fixture searches `find_fixtures`, 0x5acee0 and `find_fixture_clashes`: for
/// a club from England ([0x9cf2e4]) or Wales ([0x9cf510]) (nation +0x53, id +0), the index of the other nation's active
/// key nation (the 35 records of 72 bytes at 0xb63d60: +0 the nation, active at +0x11c, +0x14 the index); otherwise
/// 0xff.
/// Original: `fix_man_get_partner_key_nation_index` (0x5af5b0, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af5b0 } (cdecl, fix_man.cpp; das buscas de jogos 0x5acbe0, 0x5acee0 e 0x5ad780): `fix_man_get_partner_key_nation_index(clube)`:
/// para clube da Inglaterra ([0x9cf2e4]) ou do País de Gales ([0x9cf510]) (nação +0x53, id +0), o índice da nação-chave
/// ativa da outra (os 35 registros de 72 bytes em 0xb63d60: +0 a nação, ativa em +0x11c, +0x14 o índice); senão 0xff.
pub(crate) unsafe extern "C" fn fix_man_get_partner_key_nation_index(club: usize) -> u8 {
    let club = Club(club);
    let nation = (club.nation().addr() as u32) as usize;
    if nation == 0 {
        return 0xff;
    }
    let (id, inglaterra, gales) = (read_u32(nation), read_u32(KEY_NATION_ENGLAND), read_u32(KEY_NATION_WALES));
    let other = if id == inglaterra {
        gales
    } else if id == gales {
        inglaterra
    } else {
        return 0xff;
    };
    let target = read_u32(G_NATION_TABLE).wrapping_add(other.wrapping_mul(0x122)) as usize;
    for i in 0..0x23usize {
        let p = Nation(read_u32(G_LEAGUE_TABLE + i * 72) as usize);
        if !p.is_null() && p.league_selected() != 0 && p.addr() == target {
            return read_u8(0xb63d74 + i * 72);
        }
    }
    0xff
}

/// Returns the fixture list (`get_fixture_list` of the manager [0xae2a58]) for the year (+2) and day (+0) of the date, in the
/// given slice and group.
///
/// `get_fixture_list_for_date(date, slice, group)` (stdcall, ret 0xc): the fixture list from `get_fixture_list` of the manager
/// [0xae2a58] for the year (+2) and day (+0) of the date. The year argument carries in its high half that of the date
/// pointer (`mov ax, [eax+2]`), which is reproduced; the day argument carries the caller's ecx high half, which
/// `get_fixture_list` discards (`and 0xffff`): the port passes zero. Used on match days (set_up_the_day `set_up_the_day`, Rust, each list
/// of the day), in the friendlies (0x5cbf3e), in the human manager (0x5fe83a), in the media (0x7776a8) and in 0x7e6498,
/// 0x7e6ac9.
/// Original: `FIXTURE_MANAGER::get_fixture_list_for_date` (0x5ae6a0, fix_man.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x5ae6a0 } (stdcall, ret 0xc, fix_man.cpp): `get_fixture_list_for_date(data, fatia, grupo)`: a lista de partidas
/// da 0x5abf10 do gerenciador [0xae2a58] para o ano (+2) e o dia (+0) da data. O argumento do ano leva na metade alta a
/// do ponteiro da data (`mov ax, [eax+2]`), reproduzida; o do dia leva a do ecx do chamador, que a 0x5abf10 descarta
/// (`and 0xffff`): a cópia passa zero. No dia de jogos (set_up_the_day 0x6b3c10, Rust, cada lista do dia), nos
/// amistosos (0x5cbf3e), no técnico humano (0x5fe83a), na mídia (0x7776a8) e em 0x7e6498, 0x7e6ac9.
pub(crate) unsafe extern "stdcall" fn get_fixture_list_for_date(data: usize, slice: u32, group: u32) -> usize {
    let year = (data as u32 & 0xffff_0000) | read_u16(data + 2) as u32;
    let day = read_u16(data) as u32;
    exe::fixture_manager_get_fixture_list(read_usize(G_FIXTURE_MANAGER), year, day, slice, group) // the match manager
}

/// Marks the fixture's year in both clubs' entries of the club dates table (year × 2 + bit, 2001 rule).
///
/// (cdecl) Marks the year (+0x28, word) of fixture `fixture` in the entries of the two clubs (+0xc and +0x10) of the
/// table [0xae2a5c] (words: year × 2 + bit 0). Empty entry: bit 0 is set, or the entry is zeroed in 2001, and the year
/// goes in; otherwise in 2001 bit 0 is cleared and the year goes up if larger. Null table: as in the original (error
/// box).
/// Original: `mark_calendar_year` (0x5af180, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af180 } (cdecl, fix_man.cpp): marca o ano (+0x28, palavra) do jogo `jogo` nas entradas dos
/// dois clubes (+0xc e +0x10) da tabela [0xae2a5c] (palavras: ano × 2 + bit 0). Entrada vazia: o
/// bit 0 liga, ou a entrada zera em 2001, e o ano entra; senão em 2001 o bit 0 apaga e o ano sobe se
/// for maior. Tabela nula: a original (caixa de erro).
pub(crate) unsafe extern "C" fn mark_calendar_year(game: usize) {
    let game = Fixture(game);
    let table = read_usize(G_CLUB_FIXTURE_YEAR_TABLE);
    if table == 0 {
        return original_fn::<unsafe extern "C" fn(usize)>(IDX_MARK_CALENDAR_YEAR)(game.addr());
    }
    let year = game.year();
    for o in [0xc, 0x10] {
        let e = table.wrapping_add((read_i32(game.addr() + o) as usize).wrapping_mul(2)) as *mut u16;
        let v = core::ptr::read_unaligned(e);
        if v & 0xfffe == 0 {
            let w = if year == 0x7d1 { 0 } else { v | 1 };
            core::ptr::write_unaligned(e, w & 1 | year << 1);
        } else {
            if v & 1 != 0 && year == 0x7d1 {
                core::ptr::write_unaligned(e, v & 0xfffe);
            }
            let v = core::ptr::read_unaligned(e);
            if year > v >> 1 {
                core::ptr::write_unaligned(e, year.wrapping_mul(2) | v & 1);
            }
        }
    }
}

