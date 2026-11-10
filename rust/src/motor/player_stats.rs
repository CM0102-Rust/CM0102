use super::*;

/// Returns statistic `qual` 1..22 of the record (bytes, percentages ×100.0f, per-match averages, the two ratings
/// completed with under 20 matches ×0.05f), in x87 like the original.
///
/// Statistic `qual` of record `r` (matches at +0, bytes +2..+0xd, words +0xe..+0x1a, +0x1c integer), as the original
/// leaves it in x87: 1..12 the bytes; 13 and 16 the percentages (+0x12/+0x10 and +0x1a/+0x18, times 100.0f); 14, 15, 17
/// and 21 the per-match averages; 18 the average of +0xe with fewer than 20 matches completed by `peso` (weight; times
/// 0.05f); 20 the rating (2×+9 − +0xa + matches, completed by 1.5 with fewer than 20 matches; 1.5f with no matches); 22
/// the integer. No divisor: 0.0. Null `r`, 19 or outside 1..22: the original (error box).
/// Original: `player_statistic` (0x7c2150, player_stats.cpp, stdcall).
///
/// [pt] { 0x7c2150 } (stdcall, ret 0xc, player_stats.cpp): a estatística `qual` do registro `r` (jogos
/// em +0, bytes +2..+0xd, palavras +0xe..+0x1a, +0x1c inteiro), como a original a deixa no x87:
/// 1..12 os bytes; 13 e 16 as porcentagens (+0x12/+0x10 e +0x1a/+0x18, vezes 100.0f); 14, 15, 17 e
/// 21 as médias por jogo; 18 a média de +0xe com menos de 20 jogos completada por `peso` (vezes
/// 0.05f); 20 a nota (2×+9 − +0xa + jogos, completada por 1.5 com menos de 20 jogos; 1.5f sem
/// jogos); 22 o inteiro. Sem divisor: 0.0. `r` nulo, 19 ou fora de 1..22: a original (caixa de erro).
pub(crate) unsafe extern "stdcall" fn player_statistic(r: *const u8, which: i8, weight: f32) -> f64 {
    if r.is_null() || !(1..=22).contains(&which) || which == 19 {
        return original_fn::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(IDX_PLAYER_STATISTIC)(r, which, weight);
    }
    let b = |o: usize| *r.add(o) as f64;
    let w = |o: usize| core::ptr::read_unaligned(r.add(o) as *const i16) as f64;
    let games = *r;
    let cem = read_f32(rdata::F32_100) as f64;
    let vinte_avos = read_f32(rdata::F32_0_05000000075_AT_96D61C) as f64;
    match which {
        1 => b(0),
        2..=4 => b(which as usize),
        5..=12 => b(which as usize + 1),
        13 | 16 => {
            let (n, d) = if which == 13 { (0x12, 0x10) } else { (0x1a, 0x18) };
            if core::ptr::read_unaligned(r.add(d) as *const i16) > 0 {
                w(n) / w(d) * cem
            } else {
                0.0
            }
        }
        14 | 15 | 17 | 21 => {
            let o = match which { 14 => 0x14, 15 => 0x16, 17 => 0xe, _ => 0x18 };
            if games != 0 { w(o) / games as f64 } else { 0.0 }
        }
        18 => {
            if games < 20 {
                ((20 - games as i32) as f64 * weight as f64 + w(0xe)) * vinte_avos
            } else {
                w(0xe) / games as f64
            }
        }
        20 => {
            if games == 0 {
                return read_f32(rdata::F32_1_5) as f64;
            }
            let rating = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + games as i32) as f64;
            if games < 20 {
                (rating + (20 - games as i32) as f64 * read_f64(rdata::F64_1_5)) * vinte_avos
            } else {
                rating / games as f64
            }
        }
        _ => core::ptr::read_unaligned(r.add(0x1c) as *const i32) as f64,
    }
}

/// Returns the average of the record's five 4-bit ratings (zero counts as 27), 16-bit, divided by 4.
///
/// The average of the five 4-bit ratings of record `*indice` (0x27 bytes each, starting at `[this+0x30]`): the two
/// halves of +0x24 and of +0x25 and the low half of +0x26, each ×4 (zero counts as 27), summed in 16 bits and divided
/// by 4 (truncating). Null `indice`: the original (error box).
/// Original: `ratings_average` (0x7c1950, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c1950 } (thiscall, ret 4, player_stats.cpp): a média das cinco notas de 4 bits do registro
/// `*indice` (0x27 bytes cada, a partir de [this+0x30]): os dois meios de +0x24 e de +0x25 e o meio
/// baixo de +0x26, cada um ×4 (zero conta 27), somados em 16 bits e divididos por 4 (truncando).
/// `indice` nulo: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn ratings_average(this: usize, index: *const i32) -> i32 {
    if index.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> i32>(IDX_RATINGS_AVERAGE)(this, index);
    }
    let reg = (*index).wrapping_mul(0x27).wrapping_add(*((this + player_stats_fields::PLAYER_ENTRIES) as *const i32)) as usize;
    let rating = |n: u8| if n == 0 { 0x1b } else { n as u16 * 4 };
    let (a, c, e) = (*((reg + player_stats_entry::RECENT_RATINGS) as *const u8), *((reg + 0x25) as *const u8), *((reg + player_stats_entry::LAST_RATING) as *const u8));
    let sum = rating(a & 0xf)
        .wrapping_add(rating(a >> 4))
        .wrapping_add(rating(c & 0xf))
        .wrapping_add(rating(c >> 4))
        .wrapping_add(rating(e & 0xf));
    sum as i16 as i32 / 4
}

compare_value!(compare_ratings_average, ratings_average, IDX_RATINGS_AVERAGE, (indice: *const i32) -> i32);
compare_value!(compare_centre_backs_without_conflict, centre_backs_without_conflict, IDX_CENTRE_BACKS_WITHOUT_CONFLICT, () -> u8);

/// The comparer's rehearsal: the copy runs without writing to the history file (FILE_LLIST::add_node 0x58e340, which
/// writes to the file and moves the player sheet's positions); the nodes it would write go straight into the sum of
/// matches and goals per club.
///
/// [pt] O ensaio do comparador: a cópia roda sem gravar no arquivo de histórico (FILE_LLIST::add_node 0x58e340, que escreve
/// no arquivo e move as posições da ficha); os nós que ela gravaria entram direto na soma dos jogos e gols pelo clube.
pub(crate) static mut ARCHIVE_STATS_DRY_RUN: bool = false;
pub(crate) static mut ARCHIVE_STATS_PENDING_TOTALS: (u16, u16) = (0, 0);
pub(crate) static mut ARCHIVE_STATS_CLUB_ID: i32 = -1;

/// Builds the 0x2b-byte career history node: club season year, club id, category bytes, value, type and category bits.
///
/// [pt] O nó do histórico (0x2b bytes montados na pilha): +0 o ano da temporada do clube (0x669950), +2 o clube (−1 sem),
/// +6 os 32 bytes da categoria, +0x26 o valor, +0x2a o tipo (3 bits) e a categoria × 8.
pub(crate) unsafe fn career_history_node(club: Club, data: Option<usize>, value: u32, mark: u8) -> [u8; 0x2b] {
    let mut r = [0u8; 0x2b];
    r[0..2].copy_from_slice(&exe::key_nation_get_club_season_year(club).to_le_bytes());
    let id = if club.is_null() { 0xffff_ffffu32 } else { read_usize(club.addr()) as u32 };
    r[2..6].copy_from_slice(&id.to_le_bytes());
    if let Some(d) = data {
        core::ptr::copy_nonoverlapping(d as *const u8, r.as_mut_ptr().add(6), 32);
    }
    r[0x26..0x2a].copy_from_slice(&value.to_le_bytes());
    r[0x2a] = mark;
    r
}

/// Appends the node to the end of the player's history list in the archive file [this] (record +0x18/+0x1c); in
/// dry-run, only counts.
///
/// [pt] Grava o nó no fim da lista do histórico da ficha (0x58e340 no arquivo [this], posições +0x18/+0x1c do registro);
/// no ensaio, só soma.
pub(crate) unsafe fn append_history_node(this: usize, reg: usize, no: &[u8; 0x2b]) {
    if ARCHIVE_STATS_DRY_RUN {
        if i32::from_le_bytes([no[2], no[3], no[4], no[5]]) == ARCHIVE_STATS_CLUB_ID && no[0x2a] & 0xf8 == 0x10 {
            ARCHIVE_STATS_PENDING_TOTALS.0 = ARCHIVE_STATS_PENDING_TOTALS.0.wrapping_add(no[6] as u16);
            ARCHIVE_STATS_PENDING_TOTALS.1 = ARCHIVE_STATS_PENDING_TOTALS.1.wrapping_add(no[8] as u16);
        }
        return;
    }
    exe::file_llist_add_node(read_usize(this), reg + player_stats_entry::HISTORY_START, reg + player_stats_entry::HISTORY_LAST, no.as_ptr());
}

/// When the player leaves a club: the season's stats of each category go to the history file (with the year, the club,
/// the fee and the kind of exit) and are cleared; an empty node when the last category is missing; and the games and
/// goals for the club are recounted from the history.
///
/// `PlayerStats::archive_player_stats(person, fee, loan, end, club, other)`, called from club exits (contracts
/// 0x4c7ac0/`toggle_squad_placement`/`sign_loan_contract`/`end_loan`, `manager_manager_remove_player_from_squad` the squad, `end_season_for_key_nation` the national teams). The node type: 2 with `loan`,
/// 1 with `end`, 3 with `other`, otherwise 0. Null person: the error box (line 0xf28) and 0; without a record: 0. Each
/// of the six categories with a record (the record's 39-byte entry in `[this+0x30]`) goes to the history (the current
/// year and club, the record's 32 bytes in `[this+4]`, `fee`, the category and the type) and the record is cleared.
/// Without the sixth category, an empty category 6 node with `fee` when: `fee` ≥ 0 or `end`; or `club` ≥ 0 and: outside
/// the season of the club's nation (table 0xb63d8a); or, without the nation or in its season, less than 365 days since
/// joining the club (+0x3e/+0x42, `exe::date_days_since`). With `club` ≥ 0 or `fee` ≥ 0: the games (+0x20) and goals (+0x22) for
/// `club` are recounted from the category 2 nodes of the history (`exe::file_llist_get_nodes`). Returns 1.
/// Original: `PlayerStats::archive_player_stats` (0x7c5fe0, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c5fe0 } (thiscall, ret 0x18, player_stats.cpp): `PlayerStats::archive_player_stats(pessoa, valor, empréstimo,
/// fim, clube, outro)`, pelas saídas de clube (contratos 0x4c7ac0/0x4cbd20/0x4d01e0/0x4d07e0, 0x690730 o elenco, 0x7c1350
/// as seleções). O tipo do nó: 2 com `empréstimo`, 1 com `fim`, 3 com `outro`, senão 0. Pessoa nula: a caixa (linha
/// 0xf28) e 0; sem ficha: 0. Cada uma das seis categorias com registro (o registro de 39 bytes da ficha em [this+0x30])
/// vai para o histórico (o ano e o clube atuais, os 32 bytes do registro em [this+4], `valor`, a categoria e o tipo) e
/// o registro é zerado. Sem a sexta categoria, um nó vazio da categoria 6 com `valor` quando: `valor` ≥ 0 ou `fim`; ou
/// `clube` ≥ 0 e: fora da temporada da nação do clube (a tabela 0xb63d8a); ou, sem a nação ou na temporada dela, menos de
/// 365 dias desde a entrada no clube (+0x3e/+0x42, 0x54cdb0). Com `clube` ≥ 0 ou `valor` ≥ 0: recontam-se os jogos (+0x20) e os gols (+0x22) pelo
/// `clube` nos nós da categoria 2 do histórico (0x58f120). Devolve 1.
pub(crate) unsafe extern "thiscall" fn archive_player_stats(this: usize, p: usize, value: u32, loan: u32, end: u32, club_id: u32, other: u32) -> u32 {
    let p = Staff(p);
    let kind: u8 = if loan as u8 != 0 {
        2
    } else if end as u8 != 0 {
        1
    } else if other != 0 {
        3
    } else {
        0
    };
    if p.is_null() {
        show_error_at(0xa78310, 0xf28);
        return 0;
    }
    let club = Club(p.club_job().addr());
    let f = p.player();
    if f == 0 {
        return 0;
    }
    let reg = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add((read_i32(f) as isize * 0x27) as usize);
    let mut sexta = false;
    for cat in 0..6usize {
        let id = read_i32(reg + 4 * cat);
        if id == -1 {
            continue;
        }
        let data = read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add((id as isize * 0x20) as usize);
        let no = career_history_node(club, Some(data), value, (((cat + 1) as u8) << 3) | kind); // cqh8: the category and the node type
        crate::comparison::branch("cqh_8 archived category");
        append_history_node(this, reg, &no);
        if cat == 5 {
            sexta = true;
        }
        core::ptr::write_bytes(data as *mut u8, 0, 32); // cqh8: the category record zeroed
    }
    // no club nation (or outside the table) or within its season: the flag that asks for the 365 days
    let mark = if club.is_null() || club.nation().addr() == 0 {
        true
    } else {
        let k = ((exe::key_nation_index_of_nation(club.nation().addr()) as u32) as i8);
        if !(0..0x23).contains(&k) {
            true
        } else {
            let t = G_LEAGUE_WORLD_CUP_START_DATE + k as usize * 0x48;
            let (dia, ano) = (core::ptr::read_unaligned(G_CURRENT_DATE as *const i16), core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16));
            let (d0, a0) = (core::ptr::read_unaligned(t as *const i16), core::ptr::read_unaligned((t + 2) as *const u16));
            let (d1, a1) = (core::ptr::read_unaligned((t + 0xf) as *const i16), core::ptr::read_unaligned((t + 0x11) as *const u16));
            let after_of_start = (dia > d0 && ano == a0) || ano > a0;
            let before_of_end = (dia < d1 && ano == a1) || ano < a1;
            after_of_start && before_of_end
        }
    };
    if !sexta {
        let write = if (value as i32) > -1 || end as u8 != 0 {
            true
        } else if (club_id as i32) <= -1 {
            false
        } else if !mark {
            true
        } else {
            exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32) < 0x16d
        };
        if write {
            let no = career_history_node(club, None, value, 0x30 | kind); // cqh8: the empty node of the sixth category
            crate::comparison::branch("cqh_8 on empty");
            append_history_node(this, reg, &no);
        }
    }
    if (club_id as i32) > -1 || (value as i32) > -1 {
        core::ptr::write_unaligned((reg + player_stats_entry::CLUB_LEAGUE_GAMES) as *mut u16, 0);
        core::ptr::write_unaligned((reg + player_stats_entry::CLUB_LEAGUE_GOALS) as *mut u16, 0);
        let list = Llist(exe::file_llist_get_nodes(read_usize(this), read_usize(reg + player_stats_entry::HISTORY_START) as u32, read_usize(reg + player_stats_entry::HISTORY_LAST) as u32));
        if !list.is_null() {
            list.set_cursor((list.head() as u32) as usize);
            let mut no = exe::llist_get_next(list);
            while no != 0 {
                if read_usize(no + career_entry::CLUB_ID) as u32 == club_id && *((no + career_entry::KIND) as *const u8) & 0xf8 == 0x10 {
                    crate::comparison::branch("cqh_8 matches by club");
                    let a = (reg + player_stats_entry::CLUB_LEAGUE_GAMES) as *mut u16;
                    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(*((no + career_entry::STATS) as *const u8) as u16)); // cqh8: the matches for the club
                    let g = (reg + player_stats_entry::CLUB_LEAGUE_GOALS) as *mut u16;
                    core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(*((no + career_entry::STATS_GOALS) as *const u8) as u16));
                }
                no = exe::llist_get_next(list);
            }
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        if ARCHIVE_STATS_DRY_RUN {
            let a = (reg + player_stats_entry::CLUB_LEAGUE_GAMES) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(ARCHIVE_STATS_PENDING_TOTALS.0));
            let g = (reg + player_stats_entry::CLUB_LEAGUE_GOALS) as *mut u16;
            core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(ARCHIVE_STATS_PENDING_TOTALS.1));
        }
    }
    1
}

/// The size of one table entry (one category of one season at one club).
///
/// +0 the year, +2 the club id (−1 none), +6..+0x26 the 32 bytes of the category (+6 matches, +7 ..., +0x14 the word),
/// +0x26 the transfer fee (−1 none), +0x2a category × 8 with the type in the low 3 bits. Six entries (categories 1..6)
/// per season.
///
/// [pt] O tamanho de uma entrada da tabela (uma categoria de uma temporada num clube): +0 o ano, +2 o id do clube (−1
/// nenhum), +6..+0x26 os 32 bytes da categoria (+6 jogos, +7 ..., +0x14 a palavra), +0x26 o valor da transferência
/// (−1 nenhum), +0x2a categoria × 8 e o tipo nos 3 bits de baixo. Seis entradas (categorias 1..6) por temporada.
pub(crate) const HISTORY_ENTRY_SIZE: usize = 0x2b;

/// The entry built on the stack by the original routine (0x2b bytes, zeroed before each one).
///
/// [pt] A entrada montada na pilha pela original (0x2b bytes, zerada antes de cada uma).
pub(crate) struct PlayerStatsEntry(pub(crate) [u8; HISTORY_ENTRY_SIZE]);

impl PlayerStatsEntry {
    fn nova(ano: u16, clube: u32, cat: u8, transf: u32) -> PlayerStatsEntry {
        let mut r = [0u8; HISTORY_ENTRY_SIZE];
        r[0..2].copy_from_slice(&ano.to_le_bytes());
        r[2..6].copy_from_slice(&clube.to_le_bytes());
        r[0x26..0x2a].copy_from_slice(&transf.to_le_bytes());
        r[0x2a] = cat << 3;
        PlayerStatsEntry(r)
    }
    unsafe fn gravar(&self, a: usize) {
        core::ptr::copy_nonoverlapping(self.0.as_ptr(), a as *mut u8, HISTORY_ENTRY_SIZE);
    }
}

/// Shows player_stats.cpp's "Error" box at linha.
///
/// [pt] A caixa "Error" de player_stats.cpp (0xa78310) na linha `linha`.
pub(crate) unsafe fn show_player_stats_error(row: u32) {
    show_error_at(0xa78310, row);
}

/// Entry k (from 1) after count n in the history buffer: buf + (n + k - 1) × 0x2b.
///
/// [pt] A entrada `k` (contando de 1) a partir da conta `n`: buf + (n + k) × 0x2b − 0x2b.
pub(crate) fn history_entry_at(buf: usize, n: i32, k: i32) -> usize {
    buf.wrapping_add(((n + k) as isize * HISTORY_ENTRY_SIZE as isize - HISTORY_ENTRY_SIZE as isize) as usize)
}

/// Builds the player's career table, six entries (categories) per season and club: the current season, then those from
/// the history file from newest to oldest with the empty years filled, the repeated entries of the same season and club
/// summed and the empty groups removed.
///
/// `PlayerStats::build_career_history(person, &count)`: the table (malloc `exe::crt_malloc` of 500 entries, realloc `exe::crt_realloc` in
/// steps of 500; the caller frees it with `exe::crt_free`) with six entries per season and club, and the count in `*count`.
/// Null person (line 0xb98) or without a record (0xbaa): the error box and 0; id from `[0xae2364]` on: 0; out of
/// memory: the error box (0xbb2, 0xc00, 0xc26, 0xc41) and 0.
///
/// (1) The current season: the six categories with the year of the club's season (`key_nation_get_club_season_year`), the club (+0x39; −1
/// without one), transfer −1 and the category's 32 bytes (`get_category_record`) when present. (2) The history from the file
/// (FILE_LLIST::get_nodes `exe::file_llist_get_nodes` of `[this]` by the positions +0x18/+0x1c of the record's 0x27-byte entry in
/// `[this+0x30]`), from end to start (`exe::llist_get_prev`): each node of another season, club or type opens a group: the count
/// moves on by 6; empty years between the two (until the count exceeds 500) become empty groups (club −1, transfer −1);
/// the node's group takes the year, the club, the transfer and the node's type; the node goes into the entry for its
/// category (+0x2a / 8) in the group. The list is destroyed (`exe::drop_llist` and `exe::crt_operator_delete`). (3) The count moves on by 6; each
/// entry takes the transfer and the type of the entry six ahead (those of the last group, −1 and 0). (4) A group
/// followed by one of the same year and club, with no transfer in either, the following one's type other than 1 and the
/// word +0x14 other than −1 in both, adds bytes +6..+0x13 and the word +0x14 into the following one and disappears
/// (year 0). (5) A group without matches (+6, +7 of the first and the last entry), with a year, without a transfer and
/// of type 0 disappears when it has no club or when another group has its year and the next group is of the same club;
/// one with matches whose last entry has no club (+0xd9) also disappears. (6) The year-0 groups leave the table (the
/// following ones move down) and the count drops by 6 per group.
/// Original: `PlayerStats::build_career_history` (0x7c3e40, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c3e40 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::build_career_history(pessoa, &conta)`: a tabela
/// (malloc 0x944e46 de 500 entradas, realloc 0x945667 de 500 em 500; quem chama libera com 0x9452ca) com seis entradas
/// por temporada e clube, e a conta em `*conta`. Pessoa nula (linha 0xb98) ou sem ficha (0xbaa): a caixa e 0; id a
/// partir de [0xae2364]: 0; sem memória: a caixa (0xbb2, 0xc00, 0xc26, 0xc41) e 0.
/// (1) A temporada atual: as seis categorias com o ano da temporada do clube (0x669950), o clube (+0x39; −1 sem ele), a
/// transferência −1 e os 32 bytes da categoria (0x7c3920) quando há. (2) O histórico do arquivo (FILE_LLIST::get_nodes
/// 0x58f120 do [this] pelas posições +0x18/+0x1c do registro de 0x27 bytes da ficha em [this+0x30]), do fim para o
/// começo (0x68d1d0): cada nó de outra temporada, clube ou tipo abre um grupo: a conta anda 6; anos vazios entre os dois
/// (até a conta passar de 500) viram grupos vazios (clube −1, transferência −1); o grupo do nó leva o ano, o clube, a
/// transferência e o tipo do nó; o nó vai para a entrada da sua categoria (+0x2a / 8) no grupo. A lista é destruída
/// (0x68d040 e 0x944c94). (3) A conta anda 6; cada entrada leva a transferência e o tipo da entrada seis adiante (as
/// do último grupo, −1 e 0). (4) Um grupo com um seguinte do mesmo ano e clube, sem transferência nos dois, tipo do
/// seguinte diferente de 1 e a palavra +0x14 diferente de −1 nos dois, soma os bytes +6..+0x13 e a palavra +0x14 no
/// seguinte e some (ano 0). (5) O grupo sem jogos (+6, +7 da primeira e da última entrada), com ano, sem transferência
/// e de tipo 0 some quando sem clube ou quando outro grupo tem o ano dele e o grupo seguinte é do mesmo clube; o de
/// jogos com a última entrada sem clube (+0xd9) também some. (6) Os grupos de ano 0 saem da tabela (os seguintes
/// descem) e a conta cai 6 por grupo.
pub(crate) unsafe extern "thiscall" fn build_career_history(this: usize, p: usize, count: *mut i32) -> usize {
    let p = Staff(p);
    *count = 0;
    if p.is_null() {
        show_player_stats_error(0xb98);
        return 0;
    }
    if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let f = p.player();
    if f == 0 {
        show_player_stats_error(0xbaa);
        return 0;
    }
    let mut buf = exe::crt_malloc((0x53fc) as u32);
    if buf == 0 {
        show_player_stats_error(0xbb2);
        return 0;
    }
    let mut cap: i32 = 500;
    let club = p.club_job();
    let mut last = PlayerStatsEntry::nova(0, 0, 0, 0);
    for cat in 1..=6u8 {
        let year = exe::key_nation_get_club_season_year(club);
        let id = if club.is_null() { 0xffff_ffff } else { read_usize(club.addr()) as u32 }; // cqf8: the club of the current season
        crate::comparison::branch("cqf_8 current season");
        let mut r = PlayerStatsEntry::nova(year, id, cat, 0xffff_ffff);
        let c = exe::playerstats_get_category_record(this, f, cat as u32);
        if c != 0 {
            core::ptr::copy_nonoverlapping(c as *const u8, r.0.as_mut_ptr().add(6), 32);
        }
        r.gravar(history_entry_at(buf, *count, cat as i32));
        last = r;
    }
    // the year, the club and the type of the open group (the type starts at 0xff: the first node always opens a group)
    let mut year_current = u16::from_le_bytes([last.0[0], last.0[1]]);
    let mut club_current = u32::from_le_bytes([last.0[2], last.0[3], last.0[4], last.0[5]]);
    let mut kind_current: u8 = 0xff;
    let reg_sheet = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add((read_i32(f) as isize * 0x27) as usize);
    let list = Llist(exe::file_llist_get_nodes(read_usize(this), read_usize(reg_sheet + player_stats_entry::HISTORY_START) as u32, read_usize(reg_sheet + player_stats_entry::HISTORY_LAST) as u32));
    if !list.is_null() {
        list.set_cursor((list.tail() as u32) as usize);
        let mut no = exe::llist_get_prev(list);
        while no != 0 {
            let kind = read_u8(no + career_entry::KIND);
            if !(read_u16(no) == year_current && read_usize(no + career_entry::CLUB_ID) as u32 == club_current && kind & 7 == kind_current) {
                *count += 6;
                year_current = year_current.wrapping_sub(1);
                if year_current > read_u16(no) {
                    loop {
                        let mut size = cap as usize * HISTORY_ENTRY_SIZE;
                        for cat in 1..=6u8 {
                            let r = PlayerStatsEntry::nova(year_current, 0xffff_ffff, cat, 0xffff_ffff);
                            if *count + cat as i32 - 1 >= cap {
                                size += 0x53fc;
                                cap += 500;
                                buf = exe::crt_realloc(buf, (size) as u32);
                                if buf == 0 {
                                    show_player_stats_error(0xc00);
                                    return 0;
                                }
                            }
                            r.gravar(history_entry_at(buf, *count, cat as i32));
                        }
                        *count += 6;
                        if *count > 500 {
                            break;
                        }
                        year_current = year_current.wrapping_sub(1);
                        if year_current <= read_u16(no) {
                            break;
                        }
                    }
                }
                let mut size = cap as usize * HISTORY_ENTRY_SIZE;
                let year_in = read_u16(no); // cqf8: the year of the group of the node
                let club_in = read_usize(no + career_entry::CLUB_ID) as u32; // cqf8: the club of the group of the node
                for cat in 1..=6u8 {
                    let mut r = PlayerStatsEntry::nova(year_in, club_in, cat, read_usize(no + career_entry::TRANSFER_FEE) as u32);
                    r.0[0x2a] = (cat << 3) | (kind & 7);
                    crate::comparison::branch("cqf_8 history season");
                    if *count + cat as i32 - 1 >= cap {
                        size += 0x53fc;
                        cap += 500;
                        buf = exe::crt_realloc(buf, (size) as u32);
                        if buf == 0 {
                            show_player_stats_error(0xc26);
                            return 0;
                        }
                    }
                    r.gravar(history_entry_at(buf, *count, cat as i32));
                }
                year_current = read_u16(no);
                club_current = read_usize(no + career_entry::CLUB_ID) as u32;
                kind_current = kind & 7;
            }
            if (kind >> 3) as i32 - 1 + *count >= cap {
                cap += 500;
                buf = exe::crt_realloc(buf, (cap as usize * HISTORY_ENTRY_SIZE) as u32);
                if buf == 0 {
                    show_player_stats_error(0xc41);
                    return 0;
                }
            }
            core::ptr::copy_nonoverlapping(no as *const u8, history_entry_at(buf, *count, (kind >> 3) as i32) as *mut u8, HISTORY_ENTRY_SIZE);
            no = exe::llist_get_prev(list);
        }
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    *count += 6;
    // (3) the transfer and the type move down from the next group
    let n = *count;
    for i in 0..n {
        let e = buf + i as usize * HISTORY_ENTRY_SIZE;
        if i + 6 < n {
            write_u32(e + career_entry::TRANSFER_FEE, read_usize(e + career_entry::STATS * HISTORY_ENTRY_SIZE + 0x26) as u32);
            *((e + career_entry::KIND) as *mut u8) = (read_u8(e + career_entry::KIND) & 0xf8) | (read_u8(e + career_entry::STATS * HISTORY_ENTRY_SIZE + 0x2a) & 7);
        } else {
            write_u32(e + career_entry::TRANSFER_FEE, 0xffff_ffff);
            *((e + career_entry::KIND) as *mut u8) = read_u8(e + career_entry::KIND) & 0xf8;
        }
    }
    // (4) equal entries of the next group add up and the one above disappears
    let mut k = 6i32;
    while k - 6 < *count {
        let ei = buf + (k - 6) as usize * HISTORY_ENTRY_SIZE;
        let mut j = k;
        while j < *count {
            let ej = buf + j as usize * HISTORY_ENTRY_SIZE;
            if read_u16(ei) == read_u16(ej)
                && read_u8(ej + career_entry::KIND) & 7 != 1
                && read_i32(ej + career_entry::TRANSFER_FEE) == -1
                && read_i32(ei + career_entry::TRANSFER_FEE) == -1
                && read_usize(ei + career_entry::CLUB_ID) == read_usize(ej + career_entry::CLUB_ID)
                && read_u16(ei + career_entry::STATS_RATING_TOTAL) != 0xffff
                && read_u16(ej + career_entry::STATS_RATING_TOTAL) != 0xffff
            {
                for o in 6..0x14 {
                    *((ej + o) as *mut u8) = read_u8(ej + o).wrapping_add(read_u8(ei + o)); // cqf8: sum of the repeated entry
                }
                write_u16(ej + career_entry::STATS_RATING_TOTAL, read_u16(ej + career_entry::STATS_RATING_TOTAL).wrapping_add(read_u16(ei + career_entry::STATS_RATING_TOTAL)));
                if read_i32(ei + career_entry::TRANSFER_FEE) != -1 || (read_u8(ei + career_entry::KIND) & 7 == 1 && read_i32(ej + career_entry::TRANSFER_FEE) == -1) {
                    write_u32(ej + career_entry::TRANSFER_FEE, read_usize(ei + career_entry::TRANSFER_FEE) as u32);
                    *((ej + career_entry::KIND) as *mut u8) = (read_u8(ej + career_entry::KIND) & 0xf8) | (read_u8(ei + career_entry::KIND) & 7);
                }
                write_u16(ei, 0);
                break;
            }
            j += 6;
        }
        k += 1;
    }
    // (5) empty groups disappear (year 0)
    let mut g = 0i32;
    while g < *count {
        let e = buf + g as usize * HISTORY_ENTRY_SIZE;
        let clear = if read_u8(e + career_entry::STATS) == 0
            && read_u8(e + career_entry::STATS_SUB_APPEARANCES) == 0
            && read_u8(e + 0xdd) == 0
            && read_u8(e + 0xde) == 0
            && read_u16(e) != 0
            && read_i32(e + career_entry::TRANSFER_FEE) == -1
            && read_u8(e + career_entry::KIND) & 7 == 0
        {
            let mut z = false;
            let mut j = 0i32;
            while j < *count {
                if g != j {
                    let c = read_i32(e + career_entry::CLUB_ID);
                    if c == -1 {
                        z = true;
                        break;
                    }
                    let ej = buf + (j / 6) as usize * 0x102;
                    if read_u16(ej) == read_u16(e) && g + 6 < *count && read_i32(e + 0x104) == c {
                        z = true;
                        break;
                    }
                }
                j += 6;
            }
            z
        } else {
            read_i32(e + 0xd9) == -1
        };
        if clear {
            for t in 0..6 {
                write_u16(e + t * HISTORY_ENTRY_SIZE, 0); // cqf8: o grupo some
            }
        }
        g += 6;
    }
    // (6) the year-0 groups leave and the following ones move down
    let mut i = 0i32;
    while i < *count {
        let e = buf + (i / 6) as usize * 0x102;
        if read_u16(e) == 0 {
            let mut d = i + 6;
            let mut a = e + 0x102;
            while d < *count {
                core::ptr::copy(a as *const u8, (a - 0x102) as *mut u8, 0x102);
                d += 6;
                a += 0x102;
            }
            *count -= 6;
            i -= 6;
        }
        i += 6;
    }
    buf
}

/// Returns how many years since the person played for the club: 0 when they are there, −1 when never.
///
/// `player_stats_years_since_at_club(person, club)`: how many years since the person played for the club: 0 if they are
/// in it (+0x39), −1 if never; otherwise today's year (0xae2c92) minus the highest year among the history groups
/// (`build_career_history` on `[0xb74350]`; the last entry of each group, +0xd7 the year, +0xd9 the club) with the club. Null person
/// (line 0x133f), null club (0x1347) or without a record (0x135f): the error box and −1; id from `[0xae2364]` on: −1.
/// The caller calls it with the PlayerStats in ecx, which it does not use. Called from 0x5b03e0 (does not run in the
/// automatic career: it passes through untested).
/// Original: `player_stats_years_since_at_club` (0x7c6e30, player_stats.cpp, stdcall).
///
/// [pt] { 0x7c6e30 } (stdcall, ret 8, player_stats.cpp): `player_stats_years_since_at_club(pessoa, clube)`: quantos anos
/// desde que a pessoa jogou pelo clube: 0 se está nele (+0x39), −1 se nunca; senão o ano de hoje (0xae2c92) menos o
/// maior ano dos grupos do histórico (0x7c3e40 em [0xb74350]; a última entrada de cada grupo, +0xd7 o ano, +0xd9 o
/// clube) com o clube. Pessoa nula (linha 0x133f), clube nulo (0x1347) ou sem ficha (0x135f): a caixa e −1; id a partir
/// de [0xae2364]: −1. Quem chama a põe com o ecx do PlayerStats, que ela não usa. Pela 0x5b03e0 (não roda na carreira
/// automática: passageira).
pub(crate) unsafe extern "stdcall" fn player_stats_years_since_at_club(p: usize, club: usize) -> i16 {
    let p = Staff(p);
    if p.is_null() {
        show_player_stats_error(0x133f);
        return -1;
    }
    if club == 0 {
        show_player_stats_error(0x1347);
        return -1;
    }
    if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
        return -1;
    }
    if p.club_job() == Club(club) {
        return 0;
    }
    if p.player() == 0 {
        show_player_stats_error(0x135f);
        return -1;
    }
    let mut n = 0i32;
    let t = exe::playerstats_build_career_history(read_usize(G_PLAYER_STATS), p, &mut n);
    let mut greater = 0x7b3i32;
    if t != 0 {
        let mut i = 0i32;
        let mut e = t + 0xd7;
        while i < n {
            let c = read_i32(e + 2);
            if c >= 0 && c == read_i32(club) && read_u16(e) as i32 > greater {
                greater = read_u16(e) as i32;
            }
            i += 6;
            e += 0x102;
        }
    }
    exe::crt_free(t);
    if greater == 0x7b3 {
        return -1;
    }
    read_u16(G_CURRENT_DATE_YEAR).wrapping_sub(greater as u16) as i16
}

/// The qsort order of the history records: by person, by year and by the first field.
///
/// `player_stats_compare_history_by_staff(a, b)`, the qsort order of the staff_history.dat records when the statistics
/// are created (0x7bfd50, pointer at 0x7c05ad): null `a` 1, null `b` −1; by person (+4, unsigned), then by year (+8,
/// signed word), then the difference of the first dword.
/// Original: `player_stats_compare_history_by_staff` (0x7c6de0, player_stats.cpp, cdecl).
///
/// [pt] { 0x7c6de0 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_staff(a, b)`, a ordem do qsort dos
/// registros do staff_history.dat na criação das estatísticas (0x7bfd50, ponteiro em 0x7c05ad): `a` nulo 1, `b` nulo −1;
/// pela pessoa (+4, sem sinal), depois pelo ano (+8, palavra com sinal), depois a diferença do primeiro dword.
pub(crate) unsafe extern "C" fn player_stats_compare_history_by_staff(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (pa, pb) = (read_usize(a + 4) as u32, read_usize(b + 4) as u32);
    if pa > pb {
        return 1;
    }
    if pa < pb {
        return -1;
    }
    let (aa, ab) = (read_u16(a + 8) as i16, read_u16(b + 8) as i16);
    if aa > ab {
        return 1;
    }
    if aa != ab {
        return -1;
    }
    read_i32(a).wrapping_sub(read_i32(b))
}

/// Returns the requested statistic of the category record (without a record, the default for the rating and the
/// average), or the combined ratings of categories 2 to 5.
///
/// `PlayerStats::get_stat(player, category, qual, default)`, as the original leaves it in x87. No player: the "Error"
/// box (line 0x659) and 0.0; id (+0) outside the table (`[0xae236c]`): 0.0. With `qual` ≠ 0x13 and the category (signed
/// byte) up to 6: the category record (`get_category_record`) gives the statistic (`player_statistic` with `qual` and `default`); without a
/// record, `default` for 0x12 and 0x14 and 0.0 for the others. Otherwise the combined ratings from categories 2+3
/// (matches n and total t of +0xe: n < 20, ((20 − n)·5 + t)·0.05f; otherwise t/n; stored as f32), 4 (n < 8, ((8 − n)·3
/// + t)·0.125; otherwise t/n; f32) and 5 (n < 5, ((5 − n) + t)·0.2f; otherwise t/n; in x87): category 100, (c5·1.5 +
/// c4·2.5 + c23·6)·0.1; 101, (c5·3 + c4·5 + 2·c23)·0.1; 102, (c5·5 + c4·4 + c23)·0.1; the others 0.0. Without record 4
/// or 5 the original reads uninitialised locals (the copy uses zero; the comparer leaves those cases to the original).
/// Called from the line-up choice `lineup_slot_score`, MATCH_PLAYER::setup `exe::match_player_setup`, the club reputation `club_reputation_for_staff`, the player
/// strength `staff_get_player_strength_flags`, the staff's daily pass 0x5ce2d0 and others. Hot: no counter.
/// Original: `PlayerStats::get_stat` (0x7c1e30, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c1e30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_stat(jogador, categoria, qual, padrão)`, como
/// a original a deixa no x87. Sem jogador: a caixa "Error" (linha 0x659) e 0.0; id (+0) fora da tabela ([0xae236c]):
/// 0.0. Com `qual` ≠ 0x13 e a categoria (byte com sinal) até 6: o registro da categoria (0x7c3920) dá a estatística
/// (0x7c2150 com `qual` e `padrão`); sem registro, `padrão` para 0x12 e 0x14 e 0.0 para as outras. Senão as notas
/// combinadas pelas categorias 2+3 (jogos n e total t de +0xe: n < 20, ((20 − n)·5 + t)·0.05f; senão t/n; guardado em
/// f32), 4 (n < 8, ((8 − n)·3 + t)·0.125; senão t/n; f32) e 5 (n < 5, ((5 − n) + t)·0.2f; senão t/n; no x87): categoria
/// 100, (c5·1.5 + c4·2.5 + c23·6)·0.1; 101, (c5·3 + c4·5 + 2·c23)·0.1; 102, (c5·5 + c4·4 + c23)·0.1; as outras 0.0. Sem
/// o registro 4 ou 5 a original lê locais sem valor (a cópia usa zero; o comparador deixa esses casos com a original).
/// Pela escolha da escalação 0x6e2f20, pelo MATCH_PLAYER::setup 0x6eaea0, pela reputação do clube 0x545e90, pela força
/// do jogador 0x546ab0, pelo dia da comissão 0x5ce2d0 e por outras. Quente: sem contador.
pub(crate) unsafe extern "thiscall" fn get_stat(this: usize, player: usize, category: u32, which: u32, default: f32) -> f64 {
    if player == 0 {
        show_error_at(0xa78310, 0x659);
        return 0.0;
    }
    if read_usize(player) as i32 >= read_usize(G_PLAYER_DATA_COUNT) as i32 {
        return 0.0;
    }
    if which as u8 != 0x13 && category as u8 as i8 <= 6 {
        let r = exe::playerstats_get_category_record(this, player, category);
        if r != 0 {
            let value: unsafe extern "stdcall" fn(usize, u32, u32) -> f64 = core::mem::transmute(FN_PLAYER_STATS_GET_RECORD_VALUE);
            return value(r, which, default.to_bits()); // cpu3: the record value
        }
        if which as u8 == 0x12 || which as u8 == 0x14 {
            return default as f64; // cpu3: without a record, the default
        }
        return 0.0; // cpu3: no record, zero
    }
    let (mut n, mut t) = (0u8, 0i16);
    let r2 = exe::playerstats_get_category_record(this, player, 2);
    if r2 != 0 {
        n = *(r2 as *const u8);
        t = read_i16(r2 + player_category_stats::RATING_TOTAL);
    }
    let r3 = exe::playerstats_get_category_record(this, player, 3);
    if r3 != 0 {
        n = n.wrapping_add(*(r3 as *const u8));
        t = t.wrapping_add(read_i16(r3 + player_category_stats::RATING_TOTAL));
    }
    let c23 = if n < 0x14 {
        (((0x14 - n as i32) as f64 * 5.0 + t as f64) * (0.05f32 as f64)) as f32
    } else {
        (t as f64 / n as f64) as f32
    };
    // simplification: without record 4 the original reads the slot of byte n with the 3 upper bytes without a value; the copy uses zero
    let mut c4 = f32::from_bits(n as u32);
    let r4 = exe::playerstats_get_category_record(this, player, 4);
    if r4 != 0 {
        let k = *(r4 as *const u8);
        let v = read_i16(r4 + player_category_stats::RATING_TOTAL) as f64;
        c4 = if k < 8 { (((8 - k as i32) as f64 * 3.0 + v) * 0.125) as f32 } else { (v / k as f64) as f32 };
    }
    let mut c5 = 0.0f64; // simplification: without record 5, the original loads an uninitialized slot; the copy uses zero
    let r5 = exe::playerstats_get_category_record(this, player, 5);
    if r5 != 0 {
        let k = *(r5 as *const u8);
        let v = read_i16(r5 + player_category_stats::RATING_TOTAL) as f64;
        c5 = if k < 5 { ((5 - k as i32) as f64 + v) * (0.2f32 as f64) } else { v / k as f64 };
    }
    let (c23, c4) = (c23 as f64, c4 as f64);
    match category as u8 as i8 as i32 {
        100 => ((c5 * 1.5 + c4 * 2.5) + c23 * 6.0) * 0.1,
        101 => ((c5 * 3.0 + c4 * 5.0) + (c23 + c23)) * 0.1,
        102 => ((c5 * 5.0 + c4 * 4.0) + c23) * 0.1,
        _ => 0.0,
    }
}

/// The order of the language records: by person id and by language, with 0xff matching any language.
///
/// `player_stats_compare_language_entries(a, b)` (the qsort and bsearch of the languages, 6-byte records {id, language,
/// fluency}): null a, −1; null b, 1; different ids (+0), the difference; language 0xff in either one, 0; otherwise the
/// difference of the languages (+4, signed). Used by the language fluency `get_language_fluency` and by the languages added, removed
/// and improved.
/// Original: `player_stats_compare_language_entries` (0x7c6d40, player_stats.cpp, cdecl).
///
/// [pt] { 0x7c6d40 } (cdecl, player_stats.cpp): `player_stats_compare_language_entries(a, b)` (o qsort e o bsearch dos
/// idiomas, registros de 6 bytes {id, idioma, fluência}): a nulo, −1; b nulo, 1; ids (+0) diferentes, a diferença;
/// idioma 0xff em qualquer um, 0; senão a diferença dos idiomas (+4, com sinal). Pela fluência no idioma 0x7c6830 e pelos
/// idiomas acrescentados, tirados e melhorados.
pub(crate) unsafe extern "C" fn player_stats_compare_language_entries(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (ia, ib) = (read_usize(a) as i32, read_usize(b) as i32);
    if ia != ib {
        return ia.wrapping_sub(ib);
    }
    let (la, lb) = (*((a + language_entry::LANGUAGE) as *const u8), *((b + language_entry::LANGUAGE) as *const u8));
    if la == 0xff || lb == 0xff {
        return 0;
    }
    la as i8 as i32 - lb as i8 as i32
}

/// the order of the player history by the dword +0xa and then by +4
/// Original: `player_stats_compare_history_by_club` (0x7c6d90, player_stats.cpp, cdecl).
///
/// [pt] { 0x7c6d90 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_club(a, b)` (o qsort da história na carga
/// das estatísticas 0x7bfd50): a nula, 1; b nula, −1; pelo dword +0xa e depois pelo +4, sem sinal: maior 1, menor −1,
/// iguais 0.
pub(crate) unsafe extern "C" fn player_stats_compare_history_by_club(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ua, ub) = (read_usize(a + 0xa) as u32, read_usize(b + 0xa) as u32);
    if ua != ub {
        return if ua > ub { 1 } else { -1 };
    }
    let (va, vb) = (read_usize(a + 4) as u32, read_usize(b + 4) as u32);
    if va > vb {
        1
    } else if va < vb {
        -1
    } else {
        0
    }
}

/// Sums the goals of every season and club of the player's career table (`build_career_history`), in 16 bits.
///
/// The original receives the PlayerStats in ecx and does not read it. `player_stats_get_career_goals(person)` (called
/// from MATCH_PLAYER::setup `exe::match_player_setup`, which writes it into +0x43 of the match player): null person, the "Error" box
/// (line 0xb4b) and 0; id from `[0xae2364]` on, 0; without the player record (+0x61), the error box (line 0xb5d) and 0.
/// Otherwise the career table (`build_career_history` of `[0xb74350]`: 0x2b-byte entries, six per season and club) and the sum, in 16
/// bits, of byte +8 of the sixth entry of each group (+0xdf); the table is freed (`exe::crt_free`, also when null). Hot (every
/// player of every match): no counter.
/// Original: `player_stats_get_career_goals` (0x7c3d20, player_stats.cpp, stdcall, ret 4).
///
/// [pt] { 0x7c3d20 } (stdcall, ret 4, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_career_goals(pessoa)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x43 do jogador da
/// partida): pessoa nula, a caixa "Error" (linha 0xb4b) e 0; id a partir de [0xae2364], 0; sem a ficha de jogador
/// (+0x61), a caixa (linha 0xb5d) e 0. Senão a tabela da carreira (0x7c3e40 do [0xb74350]: entradas de 0x2b bytes,
/// seis por temporada e clube) e a soma, em 16 bits, do byte +8 da sexta entrada de cada grupo (+0xdf); a tabela é
/// liberada (0x9452ca, também quando nula). Quente (cada jogador de cada partida): sem contador.
pub(crate) unsafe extern "stdcall" fn player_stats_get_career_goals(person: usize) -> u16 {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0xa78310, 0xb4b);
        return 0;
    }
    if read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    if person.player() == 0 {
        show_error_at(0xa78310, 0xb5d);
        return 0;
    }
    let mut n = 0i32;
    let t = exe::playerstats_build_career_history(read_usize(G_PLAYER_STATS), person, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32; // cps9: from the first group
        while i < n {
            s = s.wrapping_add(*((t + 0xdf + (i / 6) as usize * 0x102) as *const u8) as u16);
            i += 6;
        }
    }
    exe::crt_free(t);
    s // cps9: the career goals
}

// simplification: the history goes to the file (does not run twice) and the copy writes nothing else: original only; the
// copy takes the branch.

/// Address of the category (1..6) slot in the player's 39-byte stats index record in [this+0x30]; None without player
/// or out of range.
///
/// [pt] A vaga da categoria `categoria` (1..6, byte com sinal) no índice do jogador: o registro de 39 bytes do id dele
/// (+0) em [this+0x30], dword 4 × (categoria − 1). None sem jogador ou fora de 1..6.
pub(crate) unsafe fn category_slot(this: usize, player: usize, category: u32) -> Option<usize> {
    let k = (category as u8 as i8 as i32).wrapping_sub(1);
    if player == 0 || k as u32 > 5 {
        return None;
    }
    Some(read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add(read_usize(player).wrapping_mul(39)) + 4 * k as usize)
}

/// Shows player_stats.cpp's (0xa78310) "Memory Allocation Error" box for a line and text; the game continues.
///
/// Text "v%s %s.%s %d (%s)" (version 0x90d130, name and extension, `linha`, `texto`) at 0xae24d0, the translated title
/// (0x66f4e0) at 0xae28bc, the body (0x9a7ff8 with 0xae24d0) at 0xae26c4, and the box `exe::gui_show_message_box`.
///
/// [pt] A caixa "Memory Allocation Error" de player_stats.cpp (0xa78310): "v%s %s.%s %d (%s)" (versão 0x90d130, nome e
/// extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
pub(crate) unsafe fn show_stats_memory_error(row: u32, text: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa78310, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// Returns the 0x20-byte record of the player's category (1..6), through the index in the 39-byte entry of his id; 0
/// without an index.
///
/// `PlayerStats::get_category_record(player, category)`: the 0x20-byte record (`[this+4]` + 32 × index) of the player's
/// category (category_slot), or 0 when the index is −1 or lower. No player or outside 1..6: the "Error" box and 0: the
/// original. Called from PlayerStats::record_match 0x7c10e6 and PlayerStats::get_stat `get_stat` (the match engine's
/// averages). Hot: no counter.
/// Original: `PlayerStats::get_category_record` (0x7c3920, player_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x7c3920 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::get_category_record(jogador, categoria)`: o
/// registro de 0x20 bytes ([this+4] + 32 × índice) da categoria do jogador (category_slot), ou 0 quando o índice é −1
/// ou menor. Sem jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10e6 e
/// pela PlayerStats::get_stat 0x7c1e30 (as médias do motor de partida). Quente: sem contador.
pub(crate) unsafe extern "thiscall" fn get_category_record(this: usize, player: usize, category: u32) -> usize {
    let slot = match category_slot(this, player, category) {
        Some(v) => v,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CATEGORY_RECORD)(this, player, category),
    };
    let i = read_usize(slot) as i32; // the category index
    if i <= -1 {
        return 0;
    }
    read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add((i as usize) << 5)
}

/// Returns the player's category record; without an index, the first free record (+0xe = 0xffff) or a new one at the
/// end (grows by 100), zeroed.
///
/// `PlayerStats::create_stats(player, category)`: the player's category record (category_slot); without an index (−1),
/// with loose records (`[this+0x38]`), the first record of `[this+4]` (count +0xc) with +0xe = 0xffff becomes his
/// (cleared; the loose count goes down); otherwise one at the end (cleared; the count goes up): when full (capacity
/// +8), it grows in steps of 100 (realloc `exe::crt_realloc`; out of memory, the error box 0xef4 and 0). No player or outside
/// 1..6: the "Error" box and 0: the original. Called from PlayerStats::record_match 0x7c10f9.
/// Original: `PlayerStats::create_stats` (0x7c5d80, player_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x7c5d80 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::create_stats(jogador, categoria)`: o registro da
/// categoria do jogador (category_slot); sem índice (−1), com registros soltos ([this+0x38]), o primeiro registro de
/// [this+4] (conta +0xc) com +0xe = 0xffff vira o dele (zerado; os soltos descem); senão um no fim (zerado; a conta
/// sobe): cheio (capacidade +8), cresce de 100 em 100 (realloc 0x945667; sem memória, a caixa 0xef4 e 0). Sem
/// jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10f9.
pub(crate) unsafe extern "thiscall" fn create_stats(this: usize, player: usize, category: u32) -> usize {
    let slot = match category_slot(this, player, category) {
        Some(v) => v,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_CREATE_STATS)(this, player, category),
    };
    if read_usize(slot) as i32 != -1 {
        return read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add(read_usize(slot) << 5);
    }
    if read_usize(this + player_stats_fields::FREE_CATEGORY_COUNT) != 0 {
        let n = read_usize(this + player_stats_fields::CATEGORY_COUNT) as i32;
        let mut i = 0i32;
        while i < n {
            let r = read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add((i as usize) << 5);
            if core::ptr::read_unaligned((r + player_category_stats::RATING_TOTAL) as *const u16) == 0xffff {
                crate::comparison::branch("cpq_10 record freed");
                core::ptr::write_bytes(r as *mut u8, 0, 0x20);
                write_usize(slot, i as usize);
                write_usize(this + player_stats_fields::FREE_CATEGORY_COUNT, read_usize(this + player_stats_fields::FREE_CATEGORY_COUNT).wrapping_sub(1));
                return r;
            }
            i += 1;
        }
    }
    let n = read_usize(this + player_stats_fields::CATEGORY_COUNT) as i32;
    if n >= read_usize(this + player_stats_fields::CATEGORY_CAPACITY) as i32 {
        let cap = read_usize(this + player_stats_fields::CATEGORY_CAPACITY).wrapping_add(100);
        write_usize(this + player_stats_fields::CATEGORY_CAPACITY, cap);
        let p = exe::crt_realloc(read_usize(this + player_stats_fields::CATEGORY_RECORDS), (cap << 5) as u32);
        write_usize(this + player_stats_fields::CATEGORY_RECORDS, p);
        if p == 0 {
            show_stats_memory_error(0xef4, 0xa784fc); // "PLAYER_STATS::create_stats() : Unable to allocate enough memory"
            return 0;
        }
    }
    crate::comparison::branch("cpq_10 new record");
    write_usize(slot, read_usize(this + player_stats_fields::CATEGORY_COUNT)); // the new index
    write_usize(this + player_stats_fields::CATEGORY_COUNT, read_usize(this + player_stats_fields::CATEGORY_COUNT).wrapping_add(1));
    let r = read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add(read_usize(slot) << 5);
    core::ptr::write_bytes(r as *mut u8, 0, 0x20);
    r
}

compare_value!(compare_is_retirement_pending, is_retirement_pending, IDX_IS_RETIREMENT_PENDING, (pessoa: usize) -> u8);

/// The person's 6-byte language record (id, language, fluency) via bsearch in [this+0x3c]; 0 if not found.
///
/// [pt] O registro de idioma (6 bytes: +0 id da pessoa, +4 idioma, +5 fluência) da `pessoa` no `idioma`: bsearch
/// (0x946464, comparação 0x7c6d40) na tabela [this+0x3c], [this+0x44] registros. 0 sem achar.
pub(crate) unsafe fn find_language_record(this: usize, person: usize, language: u8) -> usize {
    let mut key = [0u8; 8];
    core::ptr::write_unaligned(key.as_mut_ptr() as *mut u32, read_usize(person) as u32);
    key[4] = language; // the comparison only reads byte +4 of the key
    exe::crt_bsearch((key.as_ptr()) as usize, read_usize(this + player_stats_fields::LANGUAGES), read_usize(this + player_stats_fields::LANGUAGE_COUNT) as u32, 6, 0x7c6d40)
}

/// Appends the new entry to the language table (6 bytes: id, language, fluency up to 10; starts at 100, grows by 100)
/// and sorts it when asked (qsort).
///
/// `PlayerStats::add_language(person, language, fluency, sort)`: the language table (6 bytes per record in
/// `[this+0x3c]`, capacity +0x40, count +0x44) starts with 100 (malloc 0x258) and grows by 100 (realloc) when full; the
/// new record at the end: the person's id, the language and the fluency (at most 10, signed); with `sort`, qsort with
/// the comparison `player_stats_compare_language_entries`. Out of memory: fatal error box.
/// Original: `PlayerStats::add_language` (0x7c6a30, player_stats.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7c6a30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::add_language(pessoa, idioma, fluencia,
/// ordenar)`: a tabela dos idiomas (6 bytes por registro em [this+0x3c], capacidade +0x40, conta +0x44) nasce com 100
/// (malloc 0x258) e cresce 100 (realloc) quando cheia; o registro novo no fim: o id da pessoa, o idioma e a fluência
/// (no máximo 10, com sinal); com `ordenar`, qsort pela comparação 0x7c6d40. Sem memória: caixa fatal.
pub(crate) unsafe extern "thiscall" fn add_language(this: usize, person: usize, language: u32, fluencia: u32, sort: u32) {
    if read_usize(this + player_stats_fields::LANGUAGES) == 0 {
        let p = exe::crt_malloc(0x258);
        write_u32(this + player_stats_fields::LANGUAGES, p as u32);
        if p == 0 {
            show_error_at(0xa78310, 0x11fd);
            fatal_exit();
        }
        write_u32(this + player_stats_fields::LANGUAGE_CAPACITY, 0x64);
    }
    let cap = read_usize(this + player_stats_fields::LANGUAGE_CAPACITY);
    if cap == read_usize(this + player_stats_fields::LANGUAGE_COUNT) {
        let p = exe::crt_realloc(read_usize(this + player_stats_fields::LANGUAGES), (cap as u32).wrapping_mul(3).wrapping_add(0x12c).wrapping_mul(2));
        write_u32(this + player_stats_fields::LANGUAGES, p as u32);
        if p == 0 {
            show_error_at(0xa78310, 0x1208);
            fatal_exit();
        }
        write_u32(this + player_stats_fields::LANGUAGE_CAPACITY, (read_usize(this + player_stats_fields::LANGUAGE_CAPACITY) as u32).wrapping_add(0x64));
    }
    crate::comparison::branch("cpo_12 new language");
    let n = read_usize(this + player_stats_fields::LANGUAGE_COUNT);
    let r = read_usize(this + player_stats_fields::LANGUAGES).wrapping_add(n.wrapping_mul(6));
    write_u32(r, read_usize(person) as u32);
    *((r + language_entry::LANGUAGE) as *mut u8) = language as u8;
    let f = fluencia as u8;
    *((r + language_entry::FLUENCY) as *mut u8) = if (f as i8) > 10 { 10 } else { f };
    write_u32(this + player_stats_fields::LANGUAGE_COUNT, (n as u32).wrapping_add(1));
    if sort != 0 {
        crate::comparison::branch("cpo_12 languages sorted");
        exe::crt_qsort(read_usize(this + player_stats_fields::LANGUAGES), read_usize(this + player_stats_fields::LANGUAGE_COUNT) as u32, 6, 0x7c6d40);
    }
}

/// finds the person's entry for the language (bsearch) and adds the increase to the fluency, up to 10; without one,
/// adds the language with the increase as fluency
/// Original: `PlayerStats::improve_language` (0x7c6ca0, player_stats.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7c6ca0 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::improve_language(pessoa, idioma, aumento,
/// ordenar)`: sem tabela, ou sem o registro da pessoa no idioma (`find_language_record`), acrescenta-o com a fluência
/// `aumento` (0x7c6a30); com ele, soma o aumento à fluência (byte), no máximo 10 (com sinal).
pub(crate) unsafe extern "thiscall" fn improve_language(this: usize, person: usize, language: u32, aumento: u32, sort: u32) {
    if read_usize(this + player_stats_fields::LANGUAGES) == 0 {
        exe::playerstats_add_language(this, person, language, aumento, sort);
        return;
    }
    let r = find_language_record(this, person, language as u8);
    if r == 0 {
        exe::playerstats_add_language(this, person, language, aumento, sort);
        return;
    }
    crate::comparison::branch("cpo_12 language improved");
    let f = (*((r + language_entry::FLUENCY) as *const u8)).wrapping_add(aumento as u8);
    *((r + language_entry::FLUENCY) as *mut u8) = f;
    if (f as i8) > 10 {
        *((r + language_entry::FLUENCY) as *mut u8) = 10;
    }
}

/// frees the player's 6 category records (+0xe = −1, +1 to the free count), clears the league totals and recent ratings
/// and empties the history; turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied: with the detour the career in Rust mode left the reference
/// be54a138 (bisection of batch 251's 39 routines; with `construct_news_item`, the news constructor, the game later crashed in
/// `player_stats_reset_player` on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy
/// stays in the code and does not run
/// Original: `PlayerStats::reset_player` (0x7c6650, player_stats.cpp, thiscall, ret 4).
///
/// [pt] { 0x7c6650 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::reset_player(pessoa)`: no registro de 39 bytes da
/// pessoa ([this+0x30] + id da ficha de jogador [pessoa+0x61] × 39; corrigido em 31/10/2026, antes usava o id da pessoa), cada uma das 6 categorias (+0..+0x14) com registro (≠ −1) o libera (+0xe = −1 no
/// registro de 32 bytes de [this+4]; +1 nos livres [this+0x38]) e fica −1; zera +0x20, +0x22 (palavras) e +0x24..+0x26
/// (as notas recentes) e esvazia o histórico (0x58f110 sobre [this] com +0x18/+0x1c). Pessoa nula ou não jogadora
/// (+0x61): a original (caixa fatal). Devolve o que a 0x58f110 devolve. Do regen (0x7a59e0).
pub(crate) unsafe extern "thiscall" fn player_stats_reset_player(this: usize, person: usize) -> usize {
    let person = Staff(person);
    if person.is_null() || person.player() == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_STATS_RESET_PLAYER)(this, person.addr()); // fatal box: original only
    }
    let r = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add(read_usize(person.player()).wrapping_mul(39)); // the id of the player profile (+0x61), not the person’s
    for k in 0..6usize {
        let i = read_usize(r + 4 * k);
        if i as u32 == 0xffff_ffff {
            continue;
        }
        core::ptr::write_unaligned((read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add(i << 5) + 0xe) as *mut u16, 0xffff);
        write_u32(this + player_stats_fields::FREE_CATEGORY_COUNT, (read_usize(this + player_stats_fields::FREE_CATEGORY_COUNT) as u32).wrapping_add(1));
        write_u32(r + 4 * k, 0xffff_ffff);
    }
    core::ptr::write_unaligned((r + player_stats_entry::CLUB_LEAGUE_GAMES) as *mut u16, 0);
    core::ptr::write_unaligned((r + player_stats_entry::CLUB_LEAGUE_GOALS) as *mut u16, 0);
    *((r + player_stats_entry::LAST_RATING) as *mut u8) = 0;
    *((r + 0x25) as *mut u8) = 0;
    *((r + player_stats_entry::RECENT_RATINGS) as *mut u8) = 0;
    exe::file_llist_reset_last_offset(read_usize(this), r + player_stats_entry::HISTORY_START, r + player_stats_entry::HISTORY_LAST)
}

/// Returns from 0 to 10 how well the person speaks the language: the contract level, 10 for a nation (or the second
/// one) speaking the language, or the learned language (binary search).
///
/// `PlayerStats::get_language_fluency(person, club, language, by_contract)`: how well the person speaks `language` (0
/// to 10). Human manager (id ≥ `[0xae2364]`): 10 if it is the language (+0x77) of their nation (+0x1a), otherwise 0.
/// With `by_contract`, the contract with the club (`exe::contract_manager_get_contract_for_club`) of someone playing at a club (+0x39) of the same nation as
/// `club` (+0x53): the contract level (+0x4f & 0xf). 10 if the nation (+0x1a) or the second one (+0x1e) speaks the
/// language of `club`'s nation or `language`. Otherwise the learned language: the binary search (bsearch `exe::crt_bsearch`,
/// comparison `player_stats_compare_language_entries`, 6-byte records at `[+0x3c]`, `[+0x44]` of them) for the key {id, language}, byte +5; without
/// the table or the record, 0. The callers read only `al`.
/// Original: `PlayerStats::get_language_fluency` (0x7c6830, player_stats.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7c6830 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_language_fluency(pessoa, clube, idioma,
/// pelo_contrato)`: quanto a pessoa fala o `idioma` (0 a 10). Técnico humano (id ≥ [0xae2364]): 10 se é o idioma
/// (+0x77) da nação dela (+0x1a), senão 0. Com `pelo_contrato`, o contrato com o clube (0x4cf5f0) de quem joga num
/// clube (+0x39) da mesma nação do `clube` (+0x53): o nível do contrato (+0x4f & 0xf). 10 se a nação (+0x1a) ou a
/// segunda (+0x1e) fala o idioma da nação do `clube` ou o `idioma`. Senão o idioma aprendido: a busca binária (bsearch
/// 0x946464, comparação 0x7c6d40, registros de 6 bytes em [+0x3c], [+0x44] deles) pela chave {id, idioma}, o byte +5;
/// sem a tabela ou sem o registro, 0. Os chamadores só leem o al.
pub(crate) unsafe extern "thiscall" fn get_language_fluency(this: usize, person: usize, club: usize, language: u32, by_contract: u32) -> u8 {
    let person = Staff(person);
    let club = Club(club);
    let language = language as u8;
    if !person.is_null() && read_usize(person.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return if read_u8(person.nation().addr() + nation::FIRST_LANGUAGE) == language { 10 } else { 0 }; // the human manager
    }
    if by_contract != 0 {
        let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
        if !c.is_null() {
            let cp = Club(person.club_job().addr());
            if !cp.is_null() && club.nation() == cp.nation() {
                return c.squad_status() & 0xf;
            }
        }
    }
    let fala = |l: u8| {
        if read_u8(person.nation().addr() + nation::FIRST_LANGUAGE) == l {
            return true;
        }
        let s = Nation(person.second_nation().addr());
        !s.is_null() && (s.first_language() as u8) == l // the second nation
    };
    if !club.is_null() {
        let n = Nation(club.nation().addr());
        if !n.is_null() && fala((n.first_language() as u8)) {
            return 10;
        }
    }
    if fala(language) {
        return 10;
    }
    let base = read_usize(this + player_stats_fields::LANGUAGES);
    if base == 0 {
        return 0;
    }
    let mut key = [0u8; 8];
    key[..4].copy_from_slice(&(read_usize(person.addr()) as u32).to_le_bytes());
    key[4] = language;
    let e = exe::crt_bsearch((key.as_ptr()) as usize, base, (read_usize(this + player_stats_fields::LANGUAGE_COUNT)) as u32, (6) as u32, 0x7c6d40);
    if e == 0 {
        return 0;
    }
    read_u8(e + language_entry::FLUENCY)
}

/// Records a match in the statistics: by each side's category (friendly, reserves, competition type; a random draw in a
/// simulated match), adds the rating and the match to the double sums and each player who played to the category
/// records (games, goals, score, rating, minutes...), to the player's games and goals and to contract +0x3b; shifts the
/// form by half a byte.
///
/// `PlayerStats::record_match(fixture, match)`: null fixture or match, the error boxes and 0. Competition −2 (simulated
/// match): the category is random(9) ≠ 0 ? 2 : 3 for both sides; otherwise each side's own (match_stats_category). In
/// each row that counts (match_lineup_row): the rating (+0x35) and one match into the double sums of `this`
/// (+0x20/+0x28 with the match's flag +0x4c, otherwise +0x10/+0x18); into the 0x20-byte category records (`get_category_record`, or
/// created by `create_stats`): matches, appearances, goals/assists and the rest of the row (+0x21..+0x2d), goals for (the
/// side's +0x11f) and against (+0x744 − side), win and defeat, man of the match (+0x3c), the words (+0x35, +0x3e,
/// +0x3f, +0x42, +0x28, +0x25, +0x26) and the minutes (+0x34, +0x30 or the match duration +0x4d; a substitute subtracts
/// the entry time +0x32). In category 2 or with the competition counting (`exe::comp_util_is_league_division`): +1 match and the goals in the
/// player's record (`[this+0x30]`, 0x27 bytes) and, without a record (`contract_record`), +1 on the contract's +0x3b (`exe::contract_manager_get_contract`).
/// Finally the form (+0x24..+0x26) shifts by half a byte and receives the rating. Returns 1.
/// Original: `PlayerStats::record_match` (0x7c0d70, player_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x7c0d70 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::record_match(partida, jogo)`: partida ou jogo nulos,
/// as caixas e 0. Competição −2 (jogo simulado): a categoria é exe::utils_random(9) ≠ 0 ? 2 : 3 para os dois lados; senão a de
/// cada lado (match_stats_category). Em cada linha que conta (match_lineup_row): a nota (+0x35) e um jogo nas somas em double
/// do this (+0x20/+0x28 com a marca +0x4c do jogo, senão +0x10/+0x18); nos registros de 0x20 bytes das categorias
/// (0x7c3920, ou criado pela 0x7c5d80): jogos, entradas, gols/assistências e o resto da linha (+0x21..+0x2d), gols a
/// favor (+0x11f do lado) e contra (+0x744 − lado), vitória e derrota, melhor em campo (+0x3c), as palavras
/// (+0x35, +0x3e, +0x3f, +0x42, +0x28, +0x25, +0x26) e os minutos (+0x34, +0x30 ou a duração +0x4d do jogo; o
/// reserva desconta a entrada +0x32). Na categoria 2 ou com a competição contando (0x4b0700): +1 jogo e os gols no
/// registro do jogador ([this+0x30], 0x27 bytes) e, sem ficha (0x4cf6d0), +1 no +0x3b do contrato (0x4cf5a0). Por
/// fim a forma (+0x24..+0x26) anda meio byte e recebe a nota. Devolve 1.
pub(crate) unsafe extern "thiscall" fn player_stats_record_match(this: usize, p: usize, game: usize) -> u32 {
    if p == 0 || game == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PLAYER_STATS_RECORD_MATCH)(this, p, game); // the boxes
    }
    let simulado = read_usize(p) as i32 == -2;
    let mut cat: u8 = 0;
    if simulado {
        cat = if exe::utils_random(9) != 0 { 2 } else { 3 };
    }
    let mut contou = false;
    for lado in 0..2usize {
        if !simulado {
            let mut error = false;
            cat = match_stats_category(Fixture(p), lado as i32, true, &mut error);
        }
        let gf = game + match_stats_record::SIDE_SIZE * lado + 0x11f;
        let gc = game + match_stats_record::AWAY_GOALS - 0x625 * lado;
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = match_lineup_row(game, lado, k) else { continue };
            let rating = read_u8(e + lineup_row::RATING);
            if read_u8(game + 0x4c) != 0 {
                add_f64_at(this + player_stats_fields::FLAGGED_RATING_SUM, rating as i32 as f64);
                add_f64_at(this + player_stats_fields::FLAGGED_RATED_COUNT, 1.0);
            } else {
                add_f64_at(this + player_stats_fields::RATING_SUM, rating as i32 as f64);
                add_f64_at(this + player_stats_fields::RATED_COUNT, 1.0);
            }
            if !contou {
                contou = true;
                crate::comparison::branch("cpx_12 match recorded");
            }
            for c in stats_categories_for(cat).into_iter().flatten() {
                let mut r = player_category_record(this, dados, c);
                if r == 0 {
                    r = exe::playerstats_create_stats(this, dados, c as u32);
                }
                add_u8_at(r, 1); // cpx12: the match
                if k > 10 {
                    add_u8_at(r + player_category_stats::SUB_APPEARANCES, 1);
                }
                add_u8_at(r + player_category_stats::GOALS, read_u8(e + lineup_row::GOALS));
                add_u8_at(r + 3, read_u8(e + 0x22));
                add_u8_at(r + 4, read_u8(e + 0x23));
                add_u8_at(r + 5, read_u8(e + 0x24));
                add_u8_at(r + 6, read_u8(e + 0x27));
                let (f, a) = (read_u8(gf), read_u8(gc));
                add_u8_at(r + player_category_stats::TEAM_GOALS_FOR, f);
                add_u8_at(r + player_category_stats::TEAM_GOALS_AGAINST, a);
                if f > a {
                    add_u8_at(r + player_category_stats::WINS, 1);
                }
                if f < a {
                    add_u8_at(r + player_category_stats::DEFEATS, 1);
                }
                add_u8_at(r + 0xb, read_u8(e + 0x2c));
                add_u8_at(r + 0xc, read_u8(e + 0x2d));
                if read_usize(e + lineup_row::PLAYER_ID) == read_usize(game + match_stats_record::MAN_OF_THE_MATCH_ID) {
                    add_u8_at(r + player_category_stats::MAN_OF_THE_MATCH, 1);
                }
                add_u16_at(r + player_category_stats::RATING_TOTAL, rating as u16); // cpx12: a nota
                add_u16_at(r + 0x10, read_u8(e + 0x3e) as u16);
                add_u16_at(r + 0x12, read_u8(e + 0x3f) as u16);
                add_u16_at(r + 0x14, read_u8(e + 0x42) as u16);
                add_u16_at(r + 0x16, read_u8(e + 0x28) as u16);
                add_u16_at(r + 0x18, read_u8(e + 0x25) as u16);
                add_u16_at(r + 0x1a, read_u8(e + 0x26) as u16);
                let left = |e: usize| {
                    let (s34, s30) = (read_u8(e + 0x34) as i8, read_u8(e + 0x30) as i8);
                    if s34 > -1 {
                        s34 as i32
                    } else if s30 > -1 {
                        s30 as i32
                    } else {
                        read_i16(game + match_stats_record::DURATION) as i32
                    }
                };
                let min = r + player_category_stats::MINUTES;
                if k < 11 {
                    core::ptr::write_unaligned(min as *mut i32, (read_usize(min) as i32).wrapping_add(left(e)));
                } else if read_u8(e + lineup_row::ENTRY_MINUTE) != 0xff {
                    let v = left(e).wrapping_sub(read_u8(e + lineup_row::ENTRY_MINUTE) as i8 as i32);
                    core::ptr::write_unaligned(min as *mut i32, (read_usize(min) as i32).wrapping_add(v));
                }
            }
            let reg = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add((read_usize(e + lineup_row::PLAYER_ID)).wrapping_mul(0x27));
            if cat == 2 || (read_usize(p) as i32 > -1 && exe::comp_util_is_league_division(read_usize(p) as i32) != 0) {
                add_u16_at(reg + player_stats_entry::CLUB_LEAGUE_GAMES, 1);
                add_u16_at(reg + player_stats_entry::CLUB_LEAGUE_GOALS, read_u8(e + lineup_row::GOALS) as u16);
                if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), pessoa) == 0 && contract_of_staff(pessoa) != 0 {
                    add_u16_at(contract_of_staff(pessoa) + 0x3b, 1);
                }
            }
            let (f24, f25, f26) = (read_u8(reg + player_stats_entry::RECENT_RATINGS), read_u8(reg + 0x25), read_u8(reg + player_stats_entry::LAST_RATING));
            *((reg + player_stats_entry::RECENT_RATINGS) as *mut u8) = (f25 << 4) | (f24 >> 4);
            *((reg + 0x25) as *mut u8) = (f26 << 4) | (f25 >> 4);
            *((reg + player_stats_entry::LAST_RATING) as *mut u8) = ((rating ^ f26) & 0xf) ^ f26;
        }
    }
    1
}

/// For the nation's clubs (up to the first national team), archives (`archive_player_stats`) the statistics of every squad player not
/// on loan elsewhere; 1 (0 and the "Error" box without a nation).
///
/// `PlayerStats::end_season_for_key_nation(nation)`: without a nation, the "Error" box at line 0x396 and 0. Walks the
/// clubs of `[0xae235c]` up to the first national team (`exe::database_is_national_team`: it stops there); a club of the nation (`key_nation_index_of_club`,
/// 72-byte record at 0xb63d60): each player (+0x61, id below `[0xae2364]`) of the 50 slots (+0xd7) without a loan
/// (`contract_record` on `[0xae1050]`) or on loan to the club itself (+0x39) has his statistics archived (`archive_player_stats` with −1, 0,
/// 0, −1, 0). Returns 1.
/// Original: `PlayerStats::end_season_for_key_nation` (0x7c1350, player_stats.cpp, thiscall, ret 4).
///
/// [pt] { 0x7c1350 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::end_season_for_key_nation(nacao)`: sem nação, a
/// caixa "Error" da linha 0x396 e 0. Percorre os clubes de [0xae235c] até a primeira seleção (0x53b900: para ali);
/// clube da nação (0x669360, registro de 72 bytes em 0xb63d60): cada jogador (+0x61, id abaixo de [0xae2364]) das 50
/// vagas (+0xd7) sem empréstimo (0x4cf6d0 sobre [0xae1050]) ou emprestado ao próprio clube (+0x39) tem as estatísticas
/// arquivadas (0x7c5fe0 com −1, 0, 0, −1, 0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn end_season_for_key_nation(this: usize, nation: usize) -> u32 {
    if nation == 0 {
        show_error_at(0xa78310, 0x396);
        return 0;
    }
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let club = Club(injury_club_by_index(i));
        if injury_is_national_team(club) {
            break; // the first national team ends it
        }
        let k = (exe::key_nation_index_of_club(club) as i8) as isize; // only al
        if (0xb63d60usize).wrapping_add((k * 72) as usize) == nation {
            for s in 0..50usize {
                let p = Staff(club.squad(s));
                if p.is_null() || p.player() == 0 || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
                    continue;
                }
                if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0 || p.club_job() == club {
                    crate::comparison::branch("cpp_10 archive");
                    exe::playerstats_archive_player_stats(this, p, -1, 0, 0, -1, 0);
                }
            }
        }
        i += 1;
    }
    1
}

/// Removes a language from the player's language table (binary search) and shifts the later entries down.
///
/// Removes language `idioma` of player `ficha` from the language table (`[this+0x3c]`, `[this+0x44]` entries of 6
/// bytes: id, language, fluency), found by binary search (`exe::crt_bsearch` with `player_stats_compare_language_entries`): the following entries move down one
/// place, the last one becomes id −1 and language 0, and the count drops. Without a table or without the entry:
/// nothing.
/// Original: `PlayerStats::remove_language` (0x7c6bf0, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c6bf0 } (thiscall, ret 8, player_stats.cpp): tira o idioma `idioma` do jogador `ficha` da
/// tabela de idiomas ([this+0x3c], [this+0x44] entradas de 6 bytes: id, idioma, fluência), achado por
/// busca binária (0x946464 com 0x7c6d40): as seguintes descem uma casa, a última vira id −1 e
/// idioma 0 e a contagem cai. Sem tabela ou sem a entrada: nada.
pub(crate) unsafe extern "thiscall" fn remove_language(stats: usize, sheet: usize, language: i8) {
    let table = *((stats + player_stats_fields::LANGUAGES) as *const usize);
    if table == 0 {
        return;
    }
    let search: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    // simplification: byte 5 of the key holds stack garbage in the original; the comparer 0x7c6d40 only reads id and language
    let mut key = [0u8; 6];
    key[..4].copy_from_slice(&(*(sheet as *const u32)).to_le_bytes());
    key[4] = language as u8;
    let count = (stats + player_stats_fields::LANGUAGE_COUNT) as *mut i32;
    let achada = search(key.as_ptr() as usize, table, *count as u32, 6, 0x7c6d40);
    if achada == 0 {
        return;
    }
    let mut i = (achada.wrapping_sub(table) as u32 / 6) as i32;
    while i < *count - 1 {
        let (para, de) = (table + 6 * i as usize, table + 6 * (i + 1) as usize);
        core::ptr::write_unaligned(para as *mut u32, core::ptr::read_unaligned(de as *const u32));
        core::ptr::write_unaligned((para + language_entry::LANGUAGE) as *mut u16, core::ptr::read_unaligned((de + language_entry::LANGUAGE) as *const u16));
        i += 1;
    }
    let last = (table as isize + *count as isize * 6) as usize;
    core::ptr::write_unaligned((last - 6) as *mut u32, u32::MAX);
    *((last - 2) as *mut u8) = 0;
    *count -= 1;
}

/// Returns the player's league appearances (field 1) or league goals (field 2) at the club, as a float.
///
/// The player's league statistic at the club, as a float (fild): field 1 the matches (word +0x20) and field 2 the goals
/// (word +0x22) of the 39-byte record at `[this+0x30]` + id × 39. Null player or any other field: the original (error
/// box).
/// Original: `PlayerStats::get_club_league_stat` (0x7c47b0, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c47b0 } (thiscall, ret 8, player_stats.cpp): a estatística de liga do jogador no clube, como
/// float (fild): campo 1 as partidas (palavra +0x20) e campo 2 os gols (palavra +0x22) do registro de
/// 39 bytes em [this+0x30] + id × 39. Jogador nulo ou outro campo: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn get_club_league_stat(stats: usize, player: usize, field: i8) -> f32 {
    if player == 0 || !(1..=2).contains(&field) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f32>(IDX_GET_CLUB_LEAGUE_STAT)(stats, player, field);
    }
    let record = (*((stats + player_stats_fields::PLAYER_ENTRIES) as *const usize) as isize + *(player as *const i32) as isize * 39) as usize;
    let deslocamento = if field == 1 { 0x20 } else { 0x22 };
    core::ptr::read_unaligned((record + deslocamento) as *const i16) as f32
}

/// Returns the player's last match rating (6.5 with none).
///
/// The last match rating of player `*id` (the low 4 bits of byte +0x26 of the 39-byte record at `[estatisticas+0x30]`,
/// the statistics); with no rating, 6.5 (`[0x96e2b8]`). Null player: the original (error box). Returns in st0.
/// Original: `PlayerStats::get_last_match_rating` (0x7c1a60, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c1a60 } (thiscall, ret 4, player_stats.cpp): a nota da última partida do jogador `*id` (os 4
/// bits baixos do byte +0x26 do registro de 39 bytes em [estatisticas+0x30]); sem nota, 6,5
/// ([0x96e2b8]). Jogador nulo: a original (caixa de erro). Devolve em st0.
pub(crate) unsafe extern "thiscall" fn get_last_match_rating(stats: usize, id: *const i32) -> f32 {
    if id.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(IDX_GET_LAST_MATCH_RATING)(stats, id);
    }
    let record = (*((stats + player_stats_fields::PLAYER_ENTRIES) as *const usize) as isize + *id as isize * 39) as usize;
    let rating = *((record + player_stats_entry::LAST_RATING) as *const u8) & 0xf;
    if rating != 0 { rating as f32 } else { *(0x96e2b8 as *const f32) }
}

/// Sums the goals of the player's seasons for the club in the career table (`build_career_history`), in 16 bits.
///
/// The original receives the PlayerStats in ecx and does not read it. `player_stats_get_goals_for_club(person, club)`
/// (called from MATCH_PLAYER::setup `exe::match_player_setup`, which writes it into +0x3d of the match player): null person, the "Error"
/// box (line 0xb04) and 0; null club, the one at line 0xb0c and 0; id from `[0xae2364]` on, 0; without the player
/// record (+0x61), the one at line 0xb1e and 0. Otherwise the career table (`build_career_history` of `[0xb74350]`: 0x2b-byte
/// entries, six per season and club) and the sum, in 16 bits, of byte +0xdf of the groups whose club (+0xd9, signed,
/// non-negative) is the id of `club`; the table is freed (`exe::crt_free`, also when null). Hot (every player of every match):
/// no counter.
/// Original: `player_stats_get_goals_for_club` (0x7c3bb0, player_stats.cpp, stdcall, ret 8).
///
/// [pt] { 0x7c3bb0 } (stdcall, ret 8, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_goals_for_club(pessoa, clube)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x3d do jogador
/// da partida): pessoa nula, a caixa "Error" (linha 0xb04) e 0; clube nulo, a da linha 0xb0c e 0; id a partir de
/// [0xae2364], 0; sem a ficha de jogador (+0x61), a da linha 0xb1e e 0. Senão a tabela da carreira (0x7c3e40 do
/// [0xb74350]: entradas de 0x2b bytes, seis por temporada e clube) e a soma, em 16 bits, do byte +0xdf dos grupos cujo
/// clube (+0xd9, com sinal, não negativo) é o id do `clube`; a tabela é liberada (0x9452ca, também quando nula).
/// Quente (cada jogador de cada partida): sem contador.
pub(crate) unsafe extern "stdcall" fn player_stats_get_goals_for_club(person: usize, club: usize) -> u16 {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0xa78310, 0xb04);
        return 0;
    }
    if club == 0 {
        show_error_at(0xa78310, 0xb0c);
        return 0;
    }
    if read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    if person.player() == 0 {
        show_error_at(0xa78310, 0xb1e);
        return 0;
    }
    let mut n = 0i32;
    let t = exe::playerstats_build_career_history(read_usize(G_PLAYER_STATS), person, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32;
        while i < n {
            let g = t + 0xd9 + (i / 6) as usize * 0x102;
            let v = read_i32(g);
            if v >= 0 && v == read_i32(club) { // cps10: only the club groups
                s = s.wrapping_add(*((g + 6) as *const u8) as u16);
            }
            i += 6;
        }
    }
    exe::crt_free(t);
    s // cps10: the goals for the club
}

/// Returns the competition weight by the club's two competitions (500/1000/300/100).
///
/// The weight of competition `cod` for player `p` by the club's two competitions (+0x1a and +0x1e): in the four special
/// competitions (`[0x9cf4a8]` with 0x16/0x20, `[0x9cf408]` with 0x1e/0x20, `[0x9cf2c0]` with 0x16/0x1e, `[0x9cf244]`
/// with 7) and in the competition's codes +0x78/+0x79: 500/1000/500/300 by the first, 300/500/300/100 by the second; no
/// competition, 0. Returns only `ax`.
/// Original: `competition_weight` (0x7c6930, player_stats.cpp, stdcall).
///
/// [pt] { 0x7c6930 } (stdcall, ret 8, player_stats.cpp): o peso da competição `cod` para o jogador `p`
/// pelas duas competições do clube (+0x1a e +0x1e): nas quatro competições especiais ([0x9cf4a8]
/// com 0x16/0x20, [0x9cf408] com 0x1e/0x20, [0x9cf2c0] com 0x16/0x1e, [0x9cf244] com 7) e nos códigos
/// +0x78/+0x79 da competição: 500/1000/500/300 pela primeira, 300/500/300/100 pela segunda; sem
/// competição, 0. Devolve só `ax`.
pub(crate) unsafe extern "stdcall" fn competition_weight(p: usize, code: i8) -> u16 {
    let p = Staff(p);
    let k = |a: usize| *(a as *const i32);
    // as in the original: the first constant equal to the id decides (without the right code, goes to the codes)
    let special = |id: i32| -> u8 {
        if id == k(KEY_NATION_SWEDEN) {
            (code == 0x16 || code == 0x20) as u8
        } else if id == k(KEY_NATION_NORWAY) {
            (code == 0x1e || code == 0x20) as u8
        } else if id == k(KEY_NATION_DENMARK) {
            (code == 0x16 || code == 0x1e) as u8
        } else if id == k(KEY_NATION_BELGIUM) && code == 7 {
            2
        } else {
            0
        }
    };
    let c1 = Nation(p.nation().addr());
    if c1.is_null() {
        return 0;
    }
    match special(k(c1.addr())) {
        1 => return 500,
        2 => return 1000,
        _ => {}
    }
    if c1.second_language() == code {
        return 500;
    }
    if c1.third_language() == code {
        return 300;
    }
    let c2 = Nation(p.second_nation().addr());
    if c2.is_null() {
        return 0;
    }
    match special(k(c2.addr())) {
        1 => return 300,
        2 => return 500,
        _ => {}
    }
    if c2.second_language() == code {
        return 300;
    }
    if c2.third_language() == code { 100 } else { 0 }
}

