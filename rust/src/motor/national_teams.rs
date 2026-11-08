use super::*;

/// Returns the competition (107-byte records) indexed by +4 of the person's 110-byte record in the national-teams table
/// ([this+4]); 0 without a person or with a negative index.
///
/// [pt] { 0x7751e0 } (ret 4, national_teams.cpp): a competicao (0xae23d0, 107 bytes) do indice +4 do
/// registro da pessoa em [this+4] (110 bytes); sem pessoa ou indice negativo, 0.
pub(crate) unsafe extern "thiscall" fn person_competition(this: usize, person: usize) -> usize {
    if person == 0 {
        return 0;
    }
    let reg = (*((this + 4) as *const usize)).wrapping_add((*(person as *const i32) as usize).wrapping_mul(110));
    let i = *((reg + 4) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(107))
}

/// Byte o of the national-team block (class 0/1/2 by flag and second team via 0x5409b0) of the club's nation in
/// [this+4] records (110 bytes each).
///
/// [pt] O byte `o` do bloco da classe (0x76d780: 1 com `flag`, 2 para a segunda selecao, senao 0) da
/// nacao do clube (+0x53) nos registros de [this+4]. A original chama 0x5409b0 duas vezes.
pub(crate) unsafe fn national_block_byte(this: usize, club: usize, flag: u32, o: usize) -> u8 {
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let classe: usize = if second(club) == 0 {
        if flag == 0 { 0 } else { 1 }
    } else if flag == 0 {
        2
    } else {
        1
    };
    second(club);
    let nation = *(*((club + club::NATION) as *const usize) as *const i32);
    *((*((this + 4) as *const usize)).wrapping_add((nation as usize).wrapping_mul(110)).wrapping_add(classe * 20 + o) as *const u8)
}

/// Returns byte +0x42 of the national-team block of the club's nation (class by `flag` and second team).
///
/// [pt] { 0x7754c0 } (ret 8): o byte +0x42 do bloco da selecao do clube.
pub(crate) unsafe extern "thiscall" fn national_side_byte_42(this: usize, club: usize, flag: u32) -> u8 {
    national_block_byte(this, club, flag, 0x42)
}

/// Returns byte +0x43 of the national-team block for the club's nation and class (see national_block_byte).
///
/// [pt] { 0x775520 } (ret 8): o byte +0x43 do bloco da selecao do clube.
pub(crate) unsafe extern "thiscall" fn national_side_byte_43(this: usize, club: usize, flag: u32) -> u8 {
    national_block_byte(this, club, flag, 0x43)
}

/// Returns 1 if the club is a national team (0x53b900) or one of the two MLS All-Stars clubs (East/West), else 0.
///
/// [pt] { 0x775790 } (stdcall): selecao (0x53b900) ou um dos clubes de 0x9d00a4 e 0x9d00a8?
pub(crate) unsafe extern "stdcall" fn national_or_special(club: usize) -> u32 {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if is_national_team(club) != 0 {
        return 1;
    }
    let id = *(club as *const u32);
    (id == *(G_KEY_CLUB_MLS_ALL_STARS_EAST as *const u32) || id == *(G_KEY_CLUB_MLS_ALL_STARS_WEST as *const u32)) as u32
}

/// Tells whether person `pessoa` dislikes the main national team of his nation (+0x1a). Returns 1/0.
///
/// [pt] { 0x776380 } (stdcall): a pessoa nao gosta (0x547d30) da selecao principal (0x53b3d0) da nacao +0x1a?
pub(crate) unsafe extern "stdcall" fn dislikes_national_side(person: usize) -> u32 {
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let s = main(*((person + staff::NATION) as *const usize));
    (not_likes(person, s) != 0) as u32
}

/// Copies the date and the byte +8 from `origem` into this record. Returns `this`.
///
/// [pt] { 0x777590 } (ret 4): copia a data (0x417c10) e o byte +8 de `origem`; devolve `this`.
pub(crate) unsafe extern "thiscall" fn copy_date_and_mark(this: usize, source: usize) -> usize {
    let mut tmp = [0u8; 8];
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    copy(this, tmp.as_mut_ptr() as usize, source);
    *((this + 8) as *mut u8) = *((source + 8) as *const u8);
    this
}

/// Address of block k (20 bytes) of the person's 110-byte record in [this+4].
///
/// [pt] O bloco `k` da pessoa nos registros de [this+4]: id x 110 + k x 20.
pub(crate) unsafe fn person_block(this: usize, person: usize, k: i8) -> usize {
    (*((this + 4) as *const usize))
        .wrapping_add((*(person as *const i32) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
}

/// Sets byte +0x44 of block `k` of the person's national-teams record to 1.
///
/// [pt] { 0x76c0f0 } (ret 8): liga (1) o byte +0x44 do bloco `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn set_person_44(this: usize, person: usize, k: i8) {
    *((person_block(this, person, k) + 0x44) as *mut u8) = 1;
}

/// Returns signed byte +0x44 of block `k` (20 bytes) of person `pessoa`'s 110-byte record in [this+4].
///
/// [pt] { 0x76e010 } (ret 8): o byte +0x44 do bloco `k` da pessoa, com sinal.
pub(crate) unsafe extern "thiscall" fn person_byte_44(this: usize, person: usize, k: i8) -> i32 {
    *((person_block(this, person, k) + 0x44) as *const i8) as i32
}

/// Returns the address of field +0x3a of block `k` of the person's 110-byte record.
///
/// [pt] { 0x76e040 } (ret 8): o endereco do campo +0x3a do bloco `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn person_field_3a(this: usize, person: usize, k: i8) -> usize {
    person_block(this, person, k) + 0x3a
}

/// Classifies a person by national side: 0xff with no national job (+0x24), 2 if it is a second national team, else bit
/// 3 of the PlayingSquad byte (+0x5e). Returns only the low byte.
///
/// [pt] { 0x76e070 } (stdcall): sem selecao (+0x24), 0xff; segunda selecao (0x5409b0), 2; senao o bit 3
/// do byte +0x5e. Devolve so `al`.
pub(crate) unsafe extern "stdcall" fn class_by_national_side(person: usize) -> u8 {
    let national_team = *((person + staff::NATIONAL_JOB) as *const usize);
    if national_team == 0 {
        return 0xff;
    }
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if second(national_team) != 0 {
        return 2;
    }
    (*((person + staff::PLAYING_SQUAD) as *const u8) >> 3) & 1
}




/// Values (dword pointed to by each node's data) of a linked list (head +2, node data +0, next +6) without touching the
/// cursor.
///
/// [pt] Os valores de 4 bytes dos itens de uma lista (nó: +0 o dado, +6 o próximo; cabeça em +2), sem mexer no cursor.
pub(crate) unsafe fn list_item_values(list: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut no = read_usize(list + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(read_usize(read_usize(no)) as u32);
        no = read_usize(no + 6);
    }
    v
}

/// O ensaio do relatório (0x7706c0): os sorteios vêm da fita da original e a notícia não é entregue.
pub(crate) static mut INTERNATIONAL_REPORT_TAPE: Option<(Vec<(i32, i32)>, usize, bool)> = None;

/// Random draw: replays from the CQF11 comparison tape when set (marks failure on mismatch), else calls the game's
/// random.
pub(crate) unsafe fn random_or_tape(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(INTERNATIONAL_REPORT_TAPE);
    match f {
        Some((v, k, falhou)) => match v.get(*k) {
            Some(&(m, r)) if m == n && !*falhou => {
                *k += 1;
                r
            }
            _ => {
                *falhou = true;
                0
            }
        },
        None => {
            let s: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
            s(n)
        }
    }
}

/// Code of how a player did in the match from his slot in the result (+0x190 + 0x49 x k): appearance type, rated, man
/// of the match, extra flag; 0 if absent.
///
/// [pt] O código de como o jogador foi no jogo (do registro dele no lado do clube, +0x190 + 0x49 × k): pelo tipo +0x21
/// (1 → 2, 2 → 8, 3 → 0x20, 4 a 8 → 0x40), +4 com nota (+0x34), +0x10 se é o melhor da partida (+0x3c = o id da ficha
/// +0x61), +1 se +0x23 − +0x24 é 1; 0 se ele não está no registro.
pub(crate) unsafe fn match_performance_code(result: usize, side: usize, player: usize) -> u32 {
    let id = read_usize(player);
    for k in 0..0x14usize {
        let e = result + 0x190 + side * 0x625 + k * 0x49;
        if read_usize(e) != id {
            continue;
        }
        let mut c: u32 = match read_u8(e + 0x21) {
            1 => 2,
            2 => 8,
            3 => 0x20,
            4..=8 => 0x40,
            _ => 0,
        };
        if read_u8(e + 0x34) != 0xff {
            c |= 4;
        }
        if read_usize(result + 0x3c) == read_usize(read_usize(player + staff::PLAYER)) {
            c |= 0x10;
        }
        if (read_u8(e + 0x23) as i32) - (read_u8(e + 0x24) as i32) == 1 {
            c |= 1;
        }
        return c;
    }
    0
}

/// Sends the club international report news: the match squad is sorted by club; each group of the same club becomes
/// news 0x24 to its manager: the match data, the star player, how many, up to 4 players (the star first, the others by
/// how they did), random(2) and the codes; the group players leave the array.
///
/// Ret 0x18; SEH frame for the local news item: `NationalTeams::send_club_international_report_news(club, match,
/// result, star, squad, count)`, from the result between national sides (0x76b600), for each side. The squad (a vector
/// of `count` persons, the low byte) is sorted by club (0x945333 with 0x778ef0); each group of players of the same club
/// becomes news 0x24 to the club's manager (0x7861e0): the match record for the club (0x7e4750, only once) and its data
/// (0x7e4de0), the star player (id and keys, if he is from the club; otherwise −1 in the four), how many there are, the
/// club, up to 4 players (the star first, the others by the code of how they did, from highest to lowest), 0x420e9,
/// random(2) and the codes. The group's players leave the vector (zeroed).
/// Original: `NationalTeams::send_club_international_report_news` (0x7706c0, national_teams.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x7706c0 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH da notícia local): `NationalTeams::
/// send_club_international_report_news(clube, jogo, resultado, destaque, elenco, conta)`, pelo resultado entre seleções
/// (0x76b600), de cada lado. O elenco (vetor de `conta` pessoas, o byte baixo) é ordenado pelo clube (0x945333 com
/// 0x778ef0); cada grupo de jogadores do mesmo clube vira uma notícia 0x24 ao técnico do clube (0x7861e0): o registro
/// da partida pelo clube (0x7e4750, uma vez só) e os dados dele (0x7e4de0), o destaque (id e chaves, se é do clube;
/// senão −1 nos quatro), quantos são, o clube, até 4 jogadores (o destaque primeiro, os outros pelo código de como
/// foram, do maior para o menor), 0x420e9, sorteio(2) e os códigos. Os jogadores do grupo saem do vetor (zerados).
pub(crate) unsafe extern "thiscall" fn send_club_international_report_news(this: usize, club: usize, game: usize, result: usize, destaque: usize, squad: *mut usize, count: u32) {
    let _ = this;
    let q = count as u8 as i8;
    let sort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let record_of_match: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(FN_FILL_MATCH_RECORD_FROM_FIXTURE);
    let data_of_match: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(FN_WRITE_MATCH_RECORD_TO_NEWS);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    // 0x945333 (qsort): push 0x778ef0; push 4; push conta; push elenco; add esp, 0x10 -> cdecl; 0x77bbe0: push 0; push
    // 0x24; lea ecx, news -> thiscall; 0x785790: push value; push index; lea ecx, news -> thiscall, ret 8;
    // 0x7e4750: push &record; push club; push match; add esp, 0xc -> cdecl; 0x7e4de0: push &index; push &news;
    // push &record; add esp, 0xc -> cdecl; 0x7861e0: push club; push &news; mov ecx, [0xb7431c] -> thiscall, ret 8
    sort(squad as usize, q as i32, 4, 0x778ef0);
    if q <= 0 {
        return;
    }
    let side = (club != read_usize(game + 0x1c)) as usize;
    let mut record = [0u8; 0x20]; // the Date at +0 (0x54a110 on entry), the rest is filled by 0x7e4750
    let reg = record.as_mut_ptr() as usize;
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    data_default(reg);
    let mut montado = false;
    let mut news = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = news.as_mut_ptr() as usize;
    let mut i: i8 = 1;
    loop {
        let slot = squad.add(i as usize - 1);
        let first = *slot;
        if first != 0 && read_usize(first + staff::CLUB_JOB) != 0 {
            let mut codigos = [0u32; 4];
            let mut players = [0usize; 4];
            codigos[0] = match_performance_code(result, side, first);
            players[0] = first;
            let mut how_many: i8 = 1;
            let mut j = i;
            while j < q {
                let pj = squad.add(j as usize);
                let other = *pj;
                if other != 0 {
                    if read_usize(other + staff::CLUB_JOB) != read_usize(first + staff::CLUB_JOB) {
                        break;
                    }
                    if how_many < 4 {
                        let k = how_many as usize;
                        codigos[k] = match_performance_code(result, side, other);
                        players[k] = other;
                        if other == destaque {
                            // the highlight goes to the front (swap with the first, code together)
                            players[k] = players[0];
                            players[0] = other;
                            codigos.swap(0, k);
                        }
                    }
                    how_many += 1;
                    *pj = 0;
                }
                j += 1;
            }
            construct(nt, 0x24, 0);
            if !montado {
                montado = true;
                record_of_match(game, club, reg);
            }
            let mut idx: u8 = 0;
            data_of_match(reg, nt, &mut idx as *mut u8);
            let mut pr = |v: u32| {
                write(nt, idx as u32, v);
                idx = idx.wrapping_add(1);
            };
            if read_usize(destaque + staff::CLUB_JOB) == read_usize(first + staff::CLUB_JOB) {
                pr(read_usize(destaque) as u32);
                pr(read_usize(read_usize(destaque + staff::FIRST_NAME) + 0x33) as u32);
                pr(read_usize(read_usize(destaque + staff::SECOND_NAME) + 0x33) as u32);
                pr(read_usize(read_usize(destaque + staff::COMMON_NAME) + 0x33) as u32);
            } else {
                for _ in 0..4 {
                    pr(u32::MAX);
                }
            }
            pr(how_many as i32 as u32);
            pr(read_usize(read_usize(first + staff::CLUB_JOB)) as u32);
            // the ones after the first, by code, from largest to smallest (up to the 3rd position and 4 players)
            let end = how_many as i32 - 1;
            let mut a: i8 = 1;
            if end > 1 {
                while a < 3 {
                    let mut b = a + 1;
                    while b < how_many && b < 4 {
                        if (codigos[a as usize] as i32) < (codigos[b as usize] as i32) { // cqf11: a ordem dos jogadores
                            codigos.swap(a as usize, b as usize);
                            players.swap(a as usize, b as usize);
                        }
                        b += 1;
                    }
                    a += 1;
                    if a as i32 >= end {
                        break;
                    }
                }
            }
            let n = how_many.min(4);
            for k in 0..n.max(0) as usize {
                let p = players[k];
                pr(read_usize(p) as u32);
                pr(read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
                pr(read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
                pr(read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
            }
            pr(0x420e9);
            crate::comparison::branch("cqf_11 noticia do clube");
            let k_draw = idx;
            idx = idx.wrapping_add(1);
            let r = random_or_tape(2); // cqf11: o sorteio da notícia
            write(nt, k_draw as u32, r as u32);
            for k in 0..n.max(0) as usize {
                write(nt, idx as u32, codigos[k]);
                idx = idx.wrapping_add(1);
            }
            if (*core::ptr::addr_of!(INTERNATIONAL_REPORT_TAPE)).is_none() {
                deliver(read_usize(G_NEWS), nt, read_usize(first + staff::CLUB_JOB));
            }
        }
        let previous = i;
        i = i.wrapping_add(1);
        if previous >= q {
            break;
        }
    }
}


/// Posts the call-up news: the called-up players (flag 4, or 8 for the second team) of human-managed clubs, grouped by
/// club, become news 0x1d to the manager: national side, club, type, how many, the opponents (given list or built by
/// 0x76d560) and up to 3 players.
///
/// Ret 0x1c; SEH frame for the local news item: `national_teams_post_call_up_news(national_side, second, match, type,
/// no_opponents, opponents, count)`, from the call-up (0x76c120) and from the screen. Of the 50 slots of the national
/// side's squad (+0xd7, pointers), each player with the flag (+0x5e: 8 for the second team, otherwise 4) and a club
/// with a human manager (0x7875e0) joins his club's group; when the next one in the slot is not from the same club (or
/// lacks the flag, or the list ended), news 0x1d goes to the club's manager (0x7861e0): the national side, the club,
/// `no_opponents`, the type, how many, `second`, how many opponents and, with fewer than 4, their ids (from the given
/// list or built once by 0x76d560 with the match, when there is no list and no `no_opponents`), and up to 3 players (id
/// and keys). The list built here is destroyed and freed at the end (0x68d040, 0x944c94).
/// Original: `national_teams_post_call_up_news` (0x76e0b0, national_teams.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x76e0b0 } (thiscall, ret 0x1c, national_teams.cpp, quadro SEH da notícia local): `national_teams_post_call_up_news(
/// seleção, segunda, jogo, tipo, sem_adversarios, adversarios, conta)`, pela convocação (0x76c120) e pela tela. Das 50
/// vagas do elenco da seleção (+0xd7, ponteiros), cada jogador com a marca (+0x5e: 8 na segunda seleção, senão 4) e
/// clube de técnico humano (0x7875e0) entra no grupo do clube dele; quando o próximo da vaga não é do mesmo clube (ou
/// não tem a marca, ou acabou), sai a notícia 0x1d ao técnico do clube (0x7861e0): a seleção, o clube, `sem_adversarios`,
/// o tipo, quantos, `segunda`, quantos adversários e, com menos de 4, os ids deles (da lista dada ou montada uma vez pela
/// 0x76d560 com o jogo, sem lista e sem `sem_adversarios`), e até 3 jogadores (id e chaves). A lista montada aqui é
/// destruída e liberada no fim (0x68d040, 0x944c94).
pub(crate) unsafe extern "thiscall" fn national_teams_post_call_up_news(this: usize, national_team: usize, second: u32, game: usize, kind: u32, sem_adversarios: u32, adversarios: usize, count: u32) {
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let build: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_NATIONAL_TEAMS_COLLECT_FIXTURE_OPPONENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let erase: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    // 0x7875e0: push club; mov ecx, [0xb7431c] -> thiscall, ret 4; 0x76d560: push match; push national team; mov ecx, this ->
    // thiscall, ret 8; 0x77bbe0 / 0x785790 / 0x68d1b0 / 0x68d040: lea ecx (ou mov ecx, lista) -> thiscall; 0x7861e0:
    // push club; push &news; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x944c94 (delete): push; add esp, 4 -> cdecl
    let mut list = 0usize;
    let mut how_many_adv: i16 = 0;
    if adversarios != 0 {
        list = adversarios;
        how_many_adv = count as u8 as i8 as i16;
    }
    let mark: u8 = if second != 0 { 8 } else { 4 };
    let mut n: i8 = 0;
    let mut tres = [0usize; 3];
    let mut news = [0u32; 0x38];
    let nt = news.as_mut_ptr() as usize;
    for idx in 1..=0x32usize {
        let p = read_usize(national_team + club::SQUAD + (idx - 1) * 4);
        if p == 0 || read_u8(p + staff::PLAYING_SQUAD) & mark == 0 || read_usize(p + staff::CLUB_JOB) == 0 || human(read_usize(G_NEWS), read_usize(p + staff::CLUB_JOB)) == 0 {
            continue;
        }
        if list == 0 && sem_adversarios == 0 && game != 0 {
            list = build(this, national_team, game);
            how_many_adv = *(list as *const i16);
        }
        if n < 3 {
            tres[n as usize] = p;
        }
        let club = read_usize(p + staff::CLUB_JOB);
        n = n.wrapping_add(1);
        if idx < 0x32 {
            let next = read_usize(national_team + club::SQUAD + idx * 4);
            if next != 0 && read_usize(next + staff::CLUB_JOB) == club && read_u8(next + staff::PLAYING_SQUAD) & mark != 0 {
                continue;
            }
        }
        crate::comparison::branch("cqf_11 noticia da convocacao");
        construct(nt, 0x1d, 0);
        write(nt, 0, read_usize(national_team) as u32);
        write(nt, 1, read_usize(club) as u32);
        write(nt, 2, sem_adversarios);
        write(nt, 3, kind);
        write(nt, 4, n as i32 as u32);
        write(nt, 5, second);
        write(nt, 6, how_many_adv as i32 as u32);
        if how_many_adv < 4 && list != 0 {
            core::ptr::write_unaligned((list + 0xa) as *mut usize, read_usize(list + 2));
            let mut k: u8 = 7;
            let mut item = proximo(list);
            while item != 0 {
                write(nt, k as u32, read_usize(read_usize(item)) as u32);
                k = k.wrapping_add(1);
                item = proximo(list);
            }
        }
        if n <= 3 && n > 0 {
            let mut k: u8 = 0xa;
            for t in 0..n as usize {
                let j = tres[t];
                write(nt, k as u32, read_usize(j) as u32);
                write(nt, k.wrapping_add(1) as u32, read_usize(read_usize(j + staff::FIRST_NAME) + 0x33) as u32);
                write(nt, k.wrapping_add(2) as u32, read_usize(read_usize(j + staff::SECOND_NAME) + 0x33) as u32);
                write(nt, k.wrapping_add(3) as u32, read_usize(read_usize(j + staff::COMMON_NAME) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
        }
        deliver(read_usize(G_NEWS), nt, club);
        n = 0;
    }
    if list != 0 && adversarios == 0 {
        soltar(list);
        erase(list);
    }
}


/// O ensaio da 0x76d560: os valores que iriam para a lista nova, sem alocar.
pub(crate) static mut FIXTURE_OPPONENTS_PLAN: Option<Vec<u32>> = None;

/// Returns a new list with the match opponent (its nation's first team, for the second team) and the opponents of the
/// fixtures from tomorrow to 13 days later that are not finals.
///
/// Ret 8; SEH frame for the local list: `national_teams_collect_fixture_opponents(national_side, match)`, from the
/// call-up news (0x76e0b0) and from the request to the manager (0x76f1a0). A new list (0x944cf1 with 0xe, 0x68d020)
/// with the match opponent (the away side if the home side is from the national side's nation, otherwise the home side;
/// for the second team, the main national side of its nation, 0x53b4f0) and the opponents of the national side's other
/// matches from tomorrow to 13 days later (0x5acbe0, with byte +0x40 of the match) that are not final-stage matches
/// (the nine national side competitions). Returns the list.
/// Original: `national_teams_collect_fixture_opponents` (0x76d560, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x76d560 } (thiscall, ret 8, national_teams.cpp, quadro SEH da lista local): `national_teams_collect_fixture_
/// opponents(seleção, jogo)`, pela notícia da convocação (0x76e0b0) e pelo pedido ao técnico (0x76f1a0). Uma lista
/// nova (0x944cf1 com 0xe, 0x68d020) com o adversário do jogo (o visitante se o mandante é da nação da seleção, senão o
/// mandante; na segunda seleção, a seleção principal da nação dele, 0x53b4f0) e os adversários dos outros jogos da
/// seleção de amanhã até 13 dias depois (0x5acbe0, com o byte +0x40 do jogo) que não são de fase final (as nove
/// competições de seleções). Devolve a lista.
pub(crate) unsafe extern "thiscall" fn national_teams_collect_fixture_opponents(this: usize, national_team: usize, game: usize) -> usize {
    let _ = this;
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let data_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let find: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    // 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx (ou mov ecx, bloco) -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 ->
    // cdecl; 0x68d0d0: push 4; push &valor; mov ecx, lista -> thiscall, ret 8; 0x5409b0 / 0x53b4f0: push; add esp, 4 ->
    // cdecl; 0x549ef0: push +0x28; push +0x2a; lea ecx -> thiscall, ret 8; 0x54c5b0: push n; push &output; mov ecx, date
    // -> thiscall, ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
    let ensaio = (*core::ptr::addr_of!(FIXTURE_OPPONENTS_PLAN)).is_some();
    let mut local = [0u8; 16];
    let l = local.as_mut_ptr() as usize;
    new(l);
    let mut nl = 0usize;
    if !ensaio {
        let m = novo(0xe);
        if m != 0 {
            nl = new(m);
        }
    }
    let mut por = |v: usize| {
        if ensaio {
            if let Some(p) = &mut *core::ptr::addr_of_mut!(FIXTURE_OPPONENTS_PLAN) {
                p.push(read_usize(v) as u32);
            }
        } else {
            append(nl, v, 4);
        }
    };
    // the 9th argument of 0x5acbe0: the dword of the `jogo` argument on the stack (or of 0x53b4f0 written over it) with the low byte
    // replaced by the match’s +0x40
    let mut cell: usize = game;
    let home = game + 0x1c;
    if read_usize(read_usize(home) + 0x53) == read_usize(national_team + club::NATION) {
        if second(national_team) != 0 {
            cell = main(read_usize(read_usize(game + 0x20) + 0x53));
            por(&cell as *const usize as usize);
        } else {
            por(game + 0x20);
        }
    } else if second(national_team) != 0 {
        cell = main(read_usize(read_usize(home) + 0x53));
        por(&cell as *const usize as usize);
    } else {
        por(home);
    }
    cell = (cell & !0xff) | read_u8(game + 0x40) as usize;
    let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    let day_year = |p: usize| (read_usize(p) as u32, core::ptr::read_unaligned((p + 2) as *const u32));
    data_new(p0, *((game + 0x2a) as *const u16) as u32, *((game + 0x28) as *const u16) as u32);
    more(p0, p1, 1);
    more(p1, p2, 0xd);
    let (a1, a2) = day_year(p1);
    let (a3, a4) = day_year(p2);
    find(read_usize(G_FIXTURE_MANAGER), a1, a2, a3, a4, read_usize(national_team) as u32, l, -1, -2, cell as u32);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let finais = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
    let mut item = next(l);
    while item != 0 {
        let c = read_usize(item + 0x14);
        if c == 0 || !finais.iter().any(|&k| read_usize(c) == read_usize(k)) {
            if read_usize(read_usize(item + 0x1c) + 0x53) == read_usize(national_team + club::NATION) {
                por(item + 0x20);
            } else {
                por(item + 0x1c);
            }
        }
        item = next(l);
    }
    soltar(l);
    nl
}




/// Os códigos de posição das vagas da original: elenco de 26 (0x76c771..0x76c84c), de 24 (0x76c870..0x76c92c) e o de
/// qualquer outro tamanho (0x76c939..0x76c9de, 22 vagas), com os comuns gravados depois (0x76c860/0x76c865 e as vagas
/// 0, 1, 3, 5, 6 e 21). O código 0xc da vaga 12/11/10 vem do ebp (0xc desde 0x76c2e4/0x76c6a9).
pub(crate) const SQUAD_26_POSITIONS: [u16; 26] = [1, 1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240, 0x240];
pub(crate) const SQUAD_24_POSITIONS: [u16; 24] = [1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
pub(crate) const SQUAD_SLOTS_22: [u16; 22] = [1, 1, 1, 0x202, 0x204, 0x204, 0x204, 0x204, 0x84, 0x804, 0xc, 0x210, 0x210, 0x220, 0x90, 0x810,
    0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
/// As vagas que a tabela de 24 tem a mais que a de 22, na ordem em que aparecem nela: 0x202 (líbero/centro, a vaga
/// 3), 0x88 e 0x808 (volantes pelos lados, 9 e 10) e 0x210 (o 3º meia central, 14).
pub(crate) const SQUAD_SLOTS_24_EXTRAS: [u16; 3] = [0x202, 0x88, 0x808];

/// Position codes of the tam squad slots (tables for 22/24/26; approved fix fills slots 23 and 25 from the 24 table
/// extras).
///
/// [pt] Os códigos de posição das `tam` vagas (o byte +0x42 do registro do tipo). Fiel à original em 26 e 24 e nas 22
/// primeiras vagas dos outros tamanhos (a tabela de 22). Correção do original aprovada (01/11/2026): a original só
/// preenche as vagas de 22, 24 ou 26 jogadores; nos elencos de 23 (as fases finais, 0x76d430: a Copa de 2002, em
/// 29/05/2002) e 25, as vagas a partir da 22ª ficam com a sobra da pilha (local_86c), que vai à escala (0x6e2000) e
/// ao filtro dos escalados (0x5b7d20), e a sobra muda a cada execução (metades de ponteiros da heap). Aqui as vagas
/// além das 22 da tabela da original recebem, em ordem, as que a tabela de 24 acrescenta à de 22 (SQUAD_SLOTS_24_EXTRAS):
/// o elenco de 23 = o de 22 + 0x202 (a primeira vaga que o de 24 acrescenta); o de 25 = o de 22 + 0x202, 0x88 e
/// 0x808. Por que assim: as 22 primeiras vagas continuam as que a original grava nesses tamanhos (a comparação
/// confere todas), e cada vaga a mais vai na direção da tabela maior seguinte. O elenco de 25 parte da tabela de 22
/// e não da de 24 porque é a que a original usa para ele (só 24 e 26 têm tabela própria). Outros tamanhos: como a
/// original nas 22 primeiras; as demais com código 0 (ninguém passa no filtro). simplification: o jogo só pede 23, 24 e 26.
pub(crate) unsafe fn squad_position_codes(size: u8) -> [u16; 50] {
    let mut c = [0u16; 50];
    match size {
        0x1a => c[..26].copy_from_slice(&SQUAD_26_POSITIONS),
        0x18 => c[..24].copy_from_slice(&SQUAD_24_POSITIONS),
        _ => c[..22].copy_from_slice(&SQUAD_SLOTS_22),
    }
    if size == 23 || size == 25 {
        crate::comparison::branch(if size == 23 { "cqc_12 correção do elenco de 23" } else { "cqc_12 correção do elenco de 25" });
        for i in 22..size as usize {
            c[i] = SQUAD_SLOTS_24_EXTRAS[i - 22]; // cqc12: as vagas da correção
        }
    }
    c
}

/// Nation's record in the national-teams table: [this+4] + 0x6e x nation id.
///
/// [pt] O registro da nação nas seleções (+4 da tabela, 0x6e bytes pelo id da nação).
pub(crate) unsafe fn squad_nation_record(this: usize, nation: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(nation).wrapping_mul(0x6e))
}

/// Accepted reputation range (exclusive min, max) for selection; special Australia bands by match competition, else
/// (-1, 10001).
///
/// [pt] As faixas da reputação aceita (+9 da ficha +0x61): (mínimo exclusive, máximo), −1 e 0x2711 fora da Austrália
/// ([0x9cf224]); para ela, pela competição da partida (amistoso 0x5c6710 de [0xae2c74], ou a da Oceania: 0x8ca/0x2134;
/// Copa das Confederações 0xabe/0x222e; eliminatórias da Oceania 0x8ca e 9000, 8000 ou 10001 pelo byte +0x42; Copa
/// do Mundo 0xabe/10000).
pub(crate) unsafe fn reputation_limits(nation: usize, p: usize) -> (i16, i16) {
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    // 0x5c6710: push [p + 0x14]; mov ecx, [0xae2c74] -> thiscall ret 4
    if read_usize(nation) != read_usize(KEY_NATION_AUSTRALIA) {
        return (-1, 0x2711);
    }
    if p == 0 || (read_usize(G_FRIENDLY) != 0 && friendly(read_usize(G_FRIENDLY), read_usize(p + 0x14)) != 0) {
        return (0x8ca, 0x2134);
    }
    let k = read_usize(read_usize(p + 0x14));
    if k == read_usize(KEY_CLUB_COMP_OCEANIA_NATIONS_CUP) {
        (0x8ca, 0x2134)
    } else if k == read_usize(KEY_CLUB_COMP_CONFEDERATIONS_CUP) {
        (0xabe, 0x222e)
    } else if k == read_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
        (0x8ca, match read_u8(p + 0x42) { 1 => 0x1f40, 2 => 0x2711, _ => 0x2328 })
    } else if k == read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) {
        (0xabe, 0x2710)
    } else {
        (-1, 0x2711)
    }
}

/// For selection type: national team (types 0/1) or U21, the squad mark (+0x5e: 8 or 4) and tactics team flag.
///
/// [pt] O time do tipo (0x53b3d0 a principal nos tipos 0 e 1, 0x53b4f0 o sub-21), a marca de elenco (+0x5e: 8 no tipo
/// 1, 4 nos outros) e o time da tática (1 no tipo 1).
pub(crate) unsafe fn squad_team_for_type(nation: usize, t: i8) -> (usize, u8, u32) {
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    // 0x53b3d0 e 0x53b4f0: push nacao; add esp, 4 -> cdecl
    match t {
        1 => (main(nation), 8, 1),
        0 => (main(nation), 4, 0),
        _ => (sub21(nation), 4, 0),
    }
}

/// Range of reserve types scanned: only type itself, except U21 (2) which scans 0..1 at the Olympics and 1 otherwise.
///
/// [pt] Os tipos de reserva percorridos: só o do tipo, fora o sub-21 (2) dos Jogos Olímpicos ([0x9cf7a0]), que olha as
/// reservas 0 e 1; o sub-21 fora deles olha a 1.
pub(crate) unsafe fn reserve_type_range(t: i8, p: usize) -> (usize, usize) {
    if t != 2 {
        (0, 0)
    } else if p != 0 && read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
        (0, 1)
    } else {
        (1, 1)
    }
}

/// Reserve entries for type k, group g: (pointer into [this+0xc+4k], start, end) from the record's start dword and
/// group byte counts.
///
/// [pt] As entradas das reservas do tipo `k`, grupo `g`: (vetor [this + 0xc + 4k] a partir do início, quantas). O início é
/// o dword +8 + 0x15k do registro mais os bytes +0xe + 0x15k + 4j dos grupos anteriores; a quantidade, o byte +0xd +
/// 0x15k + 4g (como a original lê: a soma e a quantidade saem de bytes vizinhos).
pub(crate) unsafe fn reserve_group(this: usize, reg: usize, k: usize, g: usize) -> (usize, i32, i32) {
    let base = reg + 0xd + 0x15 * k;
    let mut ini = read_usize(base - 5) as i32;
    for j in 0..g {
        ini = ini.wrapping_add(read_u8(base + 1 + 4 * j) as i32);
    }
    let end = ini.wrapping_add(read_u8(base + 4 * g) as i32);
    (read_usize(this + 0xc + 4 * k).wrapping_add((ini as usize).wrapping_mul(4)), ini, end)
}

/// Does reserve player j qualify for the squad list (job, injury, retirement, dislikes, club/nation, competition rules,
/// reputation band)? (0x76cb84..0x76cf6c)
///
/// [pt] O jogador `j` das reservas entra na lista da escala? (0x76cb84..0x76cf6c)
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn is_squad_candidate(this: usize, nation: usize, t: i8, data: usize, p: usize, squad: usize, reg: usize, lim: (i16, i16), j: usize) -> bool {
    let kind_ok: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let injury: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let bits: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_CAN_CHANGE_SQUAD);
    // 0x53b620: push byte +0x3d; add esp, 4 -> cdecl; 0x62c400: push j; mov ecx, [0xb63d28] -> thiscall ret 4 (al);
    // 0x7a7eb0: push j; mov ecx, [0xb7434c] -> stdcall ret 4 (al); 0x547d30: push clube; push j; add esp, 0xc (junto
    // with that of 0x53b3d0) -> cdecl; 0x4cf5a0: push j; mov ecx, [0xae1050] -> thiscall ret 4; 0x55d270: push 0; push 1;
    // push [p + 0x14]; push j; mov ecx, [0xae28f0] -> thiscall ret 0x10; 0x76db00: 6 pushes (nation, 0, j, 0, 0, 0);
    // mov ecx, this -> thiscall ret 0x18
    if j == 0 || kind_ok(read_u8(j + staff::JOB_FOR_CLUB) as u32) == 0 || injury(read_usize(G_INJURY), j) != 0 {
        return false;
    }
    if read_usize(G_PLAYER_REGEN) != 0 && bits(j) != 0 {
        return false;
    }
    if read_usize(j + staff::CLUB_JOB) == 0 || not_likes(j, main(read_usize(j + staff::NATION))) != 0 {
        return false;
    }
    let tj = read_usize(j + staff::NATIONAL_JOB);
    if tj != 0 && (read_u8(j + staff::JOB_FOR_NATION) != 0xb || read_usize(tj + club::NATION) != nation) {
        return false;
    }
    if p != 0 {
        let k = read_usize(p);
        let c = read_usize(j + staff::CLUB_JOB);
        let other_nation = |c: usize| c != 0 && read_usize(c + club::NATION) != 0 && read_usize(read_usize(c + club::NATION) + 0x71) != read_usize(nation + nation::CONTINENT);
        let mut contract_ok = true;
        if k == read_usize(KEY_CLUB_COMP_FRIENDLY) || k == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) || k == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
            if other_nation(c) {
                // international friendlies, B and under-21: a club from another nation only with reputation (+0x80) up to 0.8 of the
                // opponent (the home team +0x1c is the team itself: the away team +0x20); outside friendlies, no
                if k != read_usize(KEY_CLUB_COMP_FRIENDLY) {
                    return false;
                }
                let s = read_usize(p + 0x1c);
                let other = if s == squad { read_usize(p + 0x20) } else { s };
                if read_i16(c + club::REPUTATION) as f64 * 0.8 > read_i16(other + club::REPUTATION) as f64 {
                    return false;
                }
            }
            // club with a match (table +0x24 by club id) and contract with +0x3d above −10: no
            if read_u8(read_usize(this + 0x24).wrapping_add(read_usize(c))) != 0 {
                let x = contract(read_usize(G_CONTRACT_MANAGER), j);
                if x != 0 && (read_u8(contract(read_usize(G_CONTRACT_MANAGER), j) + 0x3d) as i8) > -10 {
                    contract_ok = false;
                }
            }
            if !contract_ok {
                return false;
            }
        } else if other_nation(c) && read_usize(c + club::DIVISION) != 0
            && (read_i16(read_usize(p + 0x14) + 0x69) as f64) < read_i16(read_usize(c + club::DIVISION) + 0x69) as f64 * 0.7
        {
            // in the other competitions, a club from another nation whose league (+0x57) has +0x69 above the competition’s / 0.7
            return false;
        }
        if suspenso(read_usize(G_DISCIPLINE), j, read_usize(p + 0x14), 1, 0) != 0 {
            return false;
        }
        if t == 0 {
            let m = read_u16(p + 0x4d) & 0x300;
            let tj = read_usize(j + staff::NATIONAL_JOB);
            if m != 0x100 && tj == squad && read_u8(j + staff::PLAYING_SQUAD) & 8 != 0 {
                return false;
            }
            if m != 0x200 && tj != 0 && tj != squad {
                return false;
            }
        }
    }
    if read_usize(j + staff::NATIONAL_JOB) == main(nation) && read_u8(j + staff::PLAYING_SQUAD) & 4 != 0 {
        if pode_mudar(this, nation, 0, j, 0, 0, 0) == 0 {
            return false;
        }
        if !(p != 0 && read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES)) && t != 0 {
            return false;
        }
    }
    // with the flag, only after the squad date of the type (year +2 unsigned, day +0 signed)
    let after = |a: usize| {
        let (ya, da) = (read_u16(data + 2), read_i16(data));
        let (yr, dr) = (read_u16(a + 2), read_i16(a));
        ya > yr || (ya == yr && da > dr)
    };
    if read_u8(j + staff::PLAYING_SQUAD) & 8 != 0 && !after(reg + 0x4e) {
        return false;
    }
    let tj = read_usize(j + staff::NATIONAL_JOB);
    if tj == sub21(nation) && read_u8(j + staff::PLAYING_SQUAD) & 4 != 0 && !after(reg + 0x62) {
        return false;
    }
    if t == 2 && read_u8(j + staff::AGE) as i8 > 0x17 { // cqc12: a idade do sub-21
        return false;
    }
    let nat = read_usize(j + staff::NATION);
    if nat != nation {
        // from another nation: no team, no +0x22 mark, no bit 2 of +0xc in the 79-byte record, with the second
        // nation (+0x1e) in this one; it gets in if this one has a higher +0x8e, or (club in the same confederation +0x71 or no club)
        // with +0x85 of 5 or more
        if tj != 0 || read_u8(j + staff::INT_APPS) != 0 || read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j).wrapping_mul(0x4f)) + 0xc) & 2 != 0
            || read_usize(j + staff::SECOND_NATION) != nation || nat == 0
        {
            return false;
        }
        let c = read_usize(j + staff::CLUB_JOB);
        let n2 = if c != 0 { read_usize(c + club::NATION) } else { 0 };
        if read_i16(nation + nation::REPUTATION) <= read_i16(nat + nation::REPUTATION) {
            if n2 != 0 && read_usize(nation + nation::CONTINENT) != read_usize(n2 + nation::CONTINENT) {
                return false;
            }
            if (read_u8(nation + nation::LEAGUE_STANDARD) as i8) < 5 {
                return false;
            }
        }
    }
    let v = read_i16(read_usize(j + staff::PLAYER) + 9);
    v > lim.0 && v <= lim.1
}

/// Builds the candidate list from reserves of the type range group by group; returns the count (byte) and leaves the
/// last cursor + 4.
///
/// [pt] A lista da escala (0x76cae5..0x76cfe6): as reservas dos tipos da faixa, grupo a grupo; devolve a quantidade (byte,
/// como a original) e deixa em `cursor` o ponteiro da última entrada percorrida mais 4 (a original o lê depois).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn collect_squad_candidates(this: usize, nation: usize, t: i8, data: usize, p: usize, squad: usize, reg: usize, lim: (i16, i16), cand: &mut [usize; 200], cursor: &mut usize) -> u8 {
    let mut n = 0u8;
    let (lo, hi) = reserve_type_range(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (v, ini, fim) = reserve_group(this, reg, k, g);
            *cursor = v;
            for _ in ini..fim {
                let j = read_usize(*cursor);
                if is_squad_candidate(this, nation, t, data, p, squad, reg, lim, j) {
                    // simplification: the original writes to local_32c[(char) n]; from 128 on it would write before the vector
                    if (n as i8) >= 0 {
                        cand[n as usize] = j;
                    }
                    n = n.wrapping_add(1);
                }
                *cursor += 4;
            }
        }
    }
    n
}

/// The national squad selection (senior, B/Olympic or under-21): in the finals the list item; the squad dates, the ages
/// and the size; with a human manager, the request to him; otherwise those from another side and the flagged ones
/// leave, the pool list goes to the slot assignment and each picked player who fits the slot joins the squad (leaving
/// the previous side and the tactic slots); the sorting, the news and the tactic slots. Approved fix of the original
/// (01/11/2026): position codes also for 23- and 25-man squads (the original left stack leftovers in the slots from the
/// 22nd on): the original 22 plus, in order, 0x202, 0x88 and 0x808, the slots the 24 table has beyond the 22 table.
///
/// Ret 0x14, about 4.9 KB; SEH frame only for the local dates, no destructor: `NationalTeams::select_squad(nation,
/// type, date, fixture, forced)`, from the call-up (0x76bab0, in Rust: nation +0x53 of the side, type, 0xae2c90,
/// fixture, 0), from building the tables (0x76a840: fixture 0) and from the news (0x784100: forced 1). Type 0 senior, 1
/// B/Olympic (flag 8, tactic team 1), 2 under-21 (0x53b4f0). Without `forced`: in the final stages (+0x14 of the
/// fixture in one of the 9 keys) an item (team, competition, match date − 10, 1000) goes into the list at +0 (0x68d0d0,
/// created with 0x68d020 if missing); the squad date for the type (+0x3a of the record: today − 1 without a fixture,
/// the match date in type 0 and above 2, the match date − 1 in types 1 and 2, which first mark both nations in vector
/// `[this + 0x36]` at the Olympic Games or release the flagged under-21 player, 0x76d880) and today's (+0x32); the ages
/// and the size (0x76d430 into +0x42/+0x43). With a human manager who decides (0x600a50, 0x5fd6b0; or through the main
/// national side for a B team without a manager): players injured for more than 10 days (0x62c400 = 2, 0x62ed70 +0x2e)
/// leave the squad; if it is still his, the request to the manager (0x76f1a0) and stop. Otherwise: the slot codes
/// (squad_position_codes, with the 23 and 25 fix); those from another team leave the squad, those with the flag and, in
/// type 0 with bit 1 of +0x4e of the fixture, those without it (0x896c60 and +0x5e/+0x24/+0x28); the pool list
/// (collect_squad_candidates) goes to the slot assignment (0x6e2000, mode 0x2100914 in type 1, 0x2000814 in 0 and 0x814
/// in 2); each picked player who fits the slot (0x5b7d20) joins the squad: he leaves the previous team (slot, 0x76db00,
/// 0x896c60) or his own tactic slots (0x8968f0/0x896e90, 0x76db00), gets the flag (0x540980), the team and +0x28 (0xc
/// from 5 or 0xc, otherwise 0xb) and +9 of the 79-byte record goes to 0..100. Finally the squad is sorted (0x945333
/// with 0x778ef0), the news (0x76e0b0) and the tactic slots with those not in the team are cleared. The error boxes
/// (lines 0x748 and 0x76b) as the original.
/// Original: `NationalTeams::select_squad` (0x76c120, national_teams.cpp, thiscall).
///
/// [pt] { 0x76c120 } (thiscall, ret 0x14, national_teams.cpp, ~4,9 KB, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::select_squad(nação, tipo, data, partida, forçado)`, pela convocação (0x76bab0, em Rust: nação +0x53
/// do lado, tipo, 0xae2c90, partida, 0), pela montagem das tabelas (0x76a840: partida 0) e pelas notícias (0x784100:
/// forçado 1). Tipo 0 principal, 1 B/olímpico (marca 8, time da tática 1), 2 sub-21 (0x53b4f0). Sem `forçado`: nas
/// fases finais (+0x14 da partida numa das 9 chaves) um item (time, competição, data do jogo − 10, 1000) entra na lista
/// em +0 (0x68d0d0, criada com 0x68d020 se faltar); a data do elenco do tipo (+0x3a do registro: hoje − 1 sem partida,
/// a do jogo nos tipos 0 e acima de 2, a do jogo − 1 nos tipos 1 e 2, que antes marcam as duas nações no vetor
/// [this + 0x36] nos Jogos Olímpicos ou dispensam o sub-21 marcado, 0x76d880) e a de hoje (+0x32); as idades e o
/// tamanho (0x76d430 em +0x42/+0x43). Com técnico humano que decide (0x600a50, 0x5fd6b0; ou pela seleção principal
/// de uma seleção B sem técnico): os machucados por mais de 10 dias (0x62c400 = 2, 0x62ed70 +0x2e) saem do elenco; se
/// ainda for dele, o pedido ao técnico (0x76f1a0) e fim. Senão: os códigos das vagas (squad_position_codes, com a correção
/// de 23 e 25); saem do elenco os de outro time, os com a marca e, no tipo 0 com o bit 1 de +0x4e da partida, os sem
/// ela (0x896c60 e +0x5e/+0x24/+0x28); a lista das reservas (collect_squad_candidates) vai à escala (0x6e2000, modo 0x2100914
/// no tipo 1, 0x2000814 no 0 e 0x814 no 2); cada escalado que serve na vaga (0x5b7d20) entra no elenco: sai do time
/// anterior (vaga, 0x76db00, 0x896c60) ou das vagas da tática do próprio (0x8968f0/0x896e90, 0x76db00), ganha a marca
/// (0x540980), o time e +0x28 (0xc de 5 ou 0xc, senão 0xb) e o +9 do registro de 79 bytes vai a 0..100. Por fim o
/// elenco é ordenado (0x945333 com 0x778ef0), a notícia (0x76e0b0) e as vagas da tática com quem não é do time saem.
/// As caixas de erro (linhas 0x748 e 0x76b) como a original.
pub(crate) unsafe extern "thiscall" fn select_squad(this: usize, nacao: usize, kind: u32, data: usize, p: usize, forcado: u32) {
    let data_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let inserir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_RELEASE_OVERAGE_U21_PLAYERS);
    let idades: unsafe extern "stdcall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_NATIONAL_TEAMS_GET_SQUAD_SIZE_LIMITS);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let injury: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let days_of_injury: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let remove: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    let pedir: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_SQUAD_REQUEST_NEWS);
    let pick: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let mask: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let pode_mudar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_CAN_CHANGE_SQUAD);
    let of_slot: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let clear_slot: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_CLEAR_SELECTION_SLOT);
    let mark: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let news: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_CALL_UP_NEWS);
    // 0x549ef0: push +0x28; push +0x2a; lea ecx, saida -> thiscall ret 8 (palavras); 0x54c8e0: push n; push saida; mov
    // ecx, data -> thiscall ret 8; 0x417c10: push origem; push saida; lea ecx, destino -> thiscall ret 8; 0x54a110: lea
    // ecx -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 -> cdecl; 0x68d020: mov ecx, bloco -> thiscall ret 0;
    // 0x68d0d0: push 0x12; push item; mov ecx, [this] -> thiscall ret 8; 0x76d880: push nation; mov ecx, this ->
    // thiscall ret 4; 0x76d430: 4 pushes (partida, tipo, +0x42, +0x43) -> stdcall ret 0x10; 0x600a50: 5 pushes (time,
    // 0, 1, 0, 0); add esp -> cdecl; 0x5fd6b0: push 0; push manager; mov ecx, [0xb63c98] -> thiscall ret 8 (al);
    // 0x5409b0: push; add esp, 4 -> cdecl; 0x62ed70: push j; mov ecx, [0xb63d28] -> thiscall ret 4; 0x896c60: push 1;
    // push time; push j; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0x10; 0x76f1a0: 3 pushes (elenco, time,
    // partida); mov ecx, this -> ret 0xc; 0x6e2000: 15 pushes; add esp, 0x3c -> cdecl; 0x5b7d20: push +0x61; add esp,
    // 4 -> cdecl (ax); 0x8968f0 e 0x896e90: push time; push vaga; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0xc;
    // 0x540980: push marca; push j; add esp, 8 -> cdecl; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x76e0b0:
    // 7 pushes; mov ecx, this -> ret 0x1c
    let t = kind as u8 as i8;
    let reg = squad_nation_record(this, nacao);
    let lim = reputation_limits(nacao, p);
    let (elenco, marca, time) = squad_team_for_type(nacao, t);
    let chaves = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
    let finais = p != 0 && read_usize(p + 0x14) != 0 && chaves.iter().any(|&k| read_usize(read_usize(p + 0x14)) == read_usize(k));
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tactic = || read_usize(G_TACTICS);
    if forcado == 0 {
        let (mut d88c, mut d87c, mut d8a0) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p88c, p87c, p8a0) = (d88c.as_mut_ptr() as usize, d87c.as_mut_ptr() as usize, d8a0.as_mut_ptr() as usize);
        if finais {
            crate::comparison::branch("cqc_12 elenco das fases finais");
            let d = data_new(p87c, read_u16(p + 0x2a) as u32, read_u16(p + 0x28) as u32);
            minus(d, p88c, 10);
            let comp = read_usize(p + 0x14);
            if read_usize(this) == 0 {
                let m = new(0xe);
                write_usize(this, if m != 0 { list_new(m) } else { 0 });
            }
            let mut item = [0u8; 0x12];
            let pi = item.as_mut_ptr() as usize;
            default(pi + 8);
            write_usize(pi + 4, read_usize(comp));
            write_usize(pi, read_usize(elenco));
            copy(pi + 8, p87c, p88c);
            core::ptr::write_unaligned((pi + 0x10) as *mut u16, 1000);
            inserir(read_usize(this), pi, 0x12);
        }
        if p == 0 {
            let x = minus(data, p88c, 1);
            copy(r + 0x3a, p87c, x);
        } else if t != 1 && t != 2 {
            let x = data_new(p88c, read_u16(p + 0x2a) as u32, read_u16(p + 0x28) as u32);
            copy(r + 0x3a, p87c, x);
        } else {
            // each side’s nation (+0x53 of the team) and its cell in the vector [this + 0x36]
            let nation = |o: usize| read_usize(read_usize(p + o) + 0x53);
            let home = |o: usize| read_usize(nation(o)).wrapping_add(read_usize(this + 0x36));
            if read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
                write_u8(home(0x1c), 1);
                write_u8(home(0x20), 1);
            } else {
                if read_u8(home(0x1c)) != 0 {
                    dispensar(this, nation(0x1c));
                }
                if read_u8(home(0x20)) != 0 {
                    dispensar(this, nation(0x20));
                }
                write_u8(home(0x1c), 0);
                write_u8(home(0x20), 0);
            }
            let x = data_new(p88c, read_u16(p + 0x2a) as u32, read_u16(p + 0x28) as u32);
            let y = minus(x, p87c, 1);
            copy(r + 0x3a, p8a0, y);
        }
        copy(r + 0x32, p88c, data);
        idades(p, kind, r + 0x42, r + 0x43);
        let manager = read_usize(main(nacao) + 0xcf);
        let decide = || {
            (human(elenco, 0, 1, 0, 0) != 0 && (t == 0 || t9a06(read_usize(G_HUMAN_MANAGER), manager, 0) != 0))
                || (second(elenco) != 0 && read_usize(elenco + club::MANAGER) == 0
                    && human(main(read_usize(elenco + club::NATION)), 0, 1, 0, 0) != 0
                    && t9a06(read_usize(G_HUMAN_MANAGER), manager, 0) != 0)
        };
        if decide() {
            for k in 0..50 {
                let s = elenco + club::SQUAD + 4 * k;
                let j = read_usize(s);
                if j == 0 || injury(read_usize(G_INJURY), j) != 2 || read_u8(j + staff::PLAYING_SQUAD) & marca == 0 {
                    continue;
                }
                let l = days_of_injury(read_usize(G_INJURY), j);
                if l != 0 && read_i16(l + 0x2e) > 10 {
                    remove(tactic(), elenco, j, time, 1);
                    write_u8(j + staff::PLAYING_SQUAD, read_u8(j + staff::PLAYING_SQUAD) & 3);
                    write_usize(j + staff::NATIONAL_JOB, 0);
                    write_u8(j + staff::JOB_FOR_NATION, if read_u8(j + staff::JOB_FOR_NATION) == 0xc { 5 } else { 0 });
                    write_usize(s, 0);
                }
            }
        }
        if decide() {
            crate::comparison::branch("cqc_12 elenco pelo técnico humano");
            write_u8(r + 0x44, 0);
            pedir(this, elenco, time, p);
            return;
        }
    }
    let size = r + 0x42;
    let codigos = squad_position_codes(read_u8(size));
    // leave the squad: those from another team; those with the mark; in type 0 with bit 1 of the match’s +0x4e, those without it
    for k in 0..50 {
        let s = elenco + club::SQUAD + 4 * k;
        let j = read_usize(s);
        if j == 0 {
            continue;
        }
        if read_usize(j + staff::NATIONAL_JOB) != elenco {
            remove(tactic(), elenco, j, time, 1);
        } else {
            let old = if read_u8(j + staff::PLAYING_SQUAD) & marca != 0 {
                5
            } else if t == 0 && read_u8(p + 0x4e) & 1 != 0 {
                0xb
            } else {
                continue;
            };
            remove(tactic(), elenco, j, time, 1);
            write_u8(j + staff::PLAYING_SQUAD, read_u8(j + staff::PLAYING_SQUAD) & 3);
            write_usize(j + staff::NATIONAL_JOB, 0);
            write_u8(j + staff::JOB_FOR_NATION, if read_u8(j + staff::JOB_FOR_NATION) == 0xc { old } else { 0 });
        }
        write_usize(s, 0); // cqc12: a vaga do elenco fica livre
    }
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let n = collect_squad_candidates(this, nacao, t, data, p, elenco, reg, lim, &mut cand, &mut cursor);
    let mode: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut out = [0usize; 50];
    crate::comparison::branch("cqc_12 escala do elenco");
    pick(cand.as_ptr() as usize, n as u32, codigos.as_ptr() as usize, (mode & !0xff) | read_u8(size) as u32, 0,
        out.as_mut_ptr() as usize, mode, elenco, 0, 0, 0, 0, 0, 0, 0);
    let mut i = 0usize;
    // simplification: the original does not limit to 50 (the game asks for up to 26)
    while (i as i32) < read_u8(size) as i8 as i32 && i < 50 {
        let j = out[i];
        if j != 0 && codigos[i] & mask(read_usize(j + staff::PLAYER)) != 0 {
            if let Some(k) = (0..50).find(|&k| read_usize(elenco + club::SQUAD + 4 * k) == 0) {
                write_usize(elenco + club::SQUAD + 4 * k, j);
            }
            let tj = read_usize(j + staff::NATIONAL_JOB);
            if tj != elenco && tj != 0 {
                // from another team: leaves the slot there (with permission, the call-up mark of its type is zeroed) and the tactic
                let v = ((read_u8(j + staff::PLAYING_SQUAD) as i8 as i32 & 8) >> 3) as u32;
                let c3: u8 = if second(tj) == 0 { (v != 0) as u8 } else if v != 0 { 1 } else { 2 };
                let mut c = read_usize(j + staff::NATIONAL_JOB) + 0xd7;
                for _ in 0..50 {
                    if read_usize(c) == j {
                        write_usize(c, 0);
                        if pode_mudar(this, read_usize(read_usize(j + staff::NATIONAL_JOB) + 0x53), c3 as u32, 0, 0, 0, 0) != 0 {
                            write_u8(reg + 0x43 + c3 as usize * 0x14, 0);
                            break;
                        }
                        show_error_at(0xa5b2a4, 0x748);
                    }
                    c += 4;
                }
                cursor = c;
                remove(tactic(), read_usize(j + staff::NATIONAL_JOB), j, v, 1);
            } else if tj == elenco && read_u8(j + staff::PLAYING_SQUAD) & marca == 0 {
                // already of the team without the mark: leaves the team’s tactic slots (tactic team reversed)
                let bz = (time == 0) as u32;
                let c3: u8 = if second(elenco) == 0 { (bz != 0) as u8 } else if bz != 0 { 1 } else { 2 };
                for v in 0..20 {
                    if of_slot(tactic(), read_usize(j + staff::NATIONAL_JOB), v, bz) != j {
                        continue;
                    }
                    clear_slot(tactic(), read_usize(j + staff::NATIONAL_JOB), v, bz);
                    // the original passes as the player the dword of the last traversal pointer (cursor): reproduced
                    if c3 != 0 && pode_mudar(this, read_usize(read_usize(j + staff::NATIONAL_JOB) + 0x53), c3 as u32, read_usize(cursor), 0, 0, 0) != 0 {
                        write_u8(reg + 0x43 + c3 as usize * 0x14, 0);
                        break;
                    }
                    show_error_at(0xa5b2a4, 0x76b);
                }
            }
            mark(j, marca as u32);
            write_usize(j + staff::NATIONAL_JOB, elenco); // cqc12: o convocado ganha o time
            let x = read_u8(j + staff::JOB_FOR_NATION);
            write_u8(j + staff::JOB_FOR_NATION, if x == 5 || x == 0xc { 0xc } else { 0xb });
            let e = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j).wrapping_mul(0x4f)) + 9;
            write_u8(e, (read_u8(e) as i8).clamp(0, 100) as u8);
        }
        i += 1;
    }
    sort(elenco + club::SQUAD, 0x32, 4, 0x778ef0);
    let key = if p == 0 { u32::MAX } else { read_usize(read_usize(p + 0x14)) as u32 };
    news(this, elenco, time, p, key, finais as u32, 0, 0);
    for v in 0..20 {
        let x = of_slot(tactic(), elenco, v, time);
        if x != 0 && read_usize(x + 0x24) != elenco {
            clear_slot(tactic(), elenco, v, time);
        }
    }
}

/// Memory regions the squad selection (0x76c120) touches: nation record and each scanned reserve player's +0x24..+0x28
/// and +0x5e.
///
/// [pt] O que a escala da 0x76c120 lê e a original muda depois dela: o registro da nação (0x6e bytes) e, de cada jogador
/// das reservas percorridas, +0x24..+0x28 (time, estado) e +0x5e (marcas de elenco).
pub(crate) unsafe fn squad_selection_regions(this: usize, nation: usize, t: i8, p: usize) -> Vec<(usize, usize)> {
    let reg = squad_nation_record(this, nation);
    let mut v = vec![(reg, 0x6e)];
    let (lo, hi) = reserve_type_range(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (a, ini, fim) = reserve_group(this, reg, k, g);
            for i in 0..(fim - ini).max(0) as usize {
                let j = read_usize(a + 4 * i);
                if j != 0 {
                    v.push((j + staff::NATIONAL_JOB, 5));
                    v.push((j + staff::PLAYING_SQUAD, 1));
                }
            }
        }
    }
    v
}





/// Club record of all-star team g ([0x9d00a4] or [0x9d00a8]) in the club table (0x245 bytes).
///
/// [pt] O clube do time das estrelas `g` ([0x9d00a4] ou [0x9d00a8]): o registro de 0x245 bytes em [0xae23b4].
pub(crate) unsafe fn all_star_club(g: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(g).wrapping_mul(club::SIZE))
}

/// Returns the national side holding the player from the day before to the day after (a national side fixture in the
/// window, against the given opponent if any, or a call-up already started) or the MLS all-star squad on the game date;
/// writes the comment; otherwise 0 (and 0 for a national side or all-star club).
///
/// Ret 0x14: `NationalTeams::get_international_duty_club(player, date, club, comment, opponent)`, from the line-up
/// (0x6dbb80, 0x6dbc50), the scouts, the transfers and the screens. With a temporary list (0x68d020, destroyed at the
/// end by 0x68d040). Club is a national side (0x53b900) or one of the all-star teams (id in `[0x9d00a4]`/`[0x9d00a8]`):
/// 0. The window goes from the day before (0x54c8e0) to the day after (0x54c5b0). Called-up player (+0x24, the national
/// side): the national side's matches in the window (0x5acbe0, side by bit 4 of +0x5e: set 1, otherwise 0; the days and
/// years as the original's dwords); with a match and no opponent, or with the opponent (+0x14 of the match) among them,
/// he is on duty (text 0xa62ff4 into `comment`, 0x66f4e0) and returns the national side. With bit 4 set, the call-ups
/// of list `[this]` (+0 the national side, +4 the opponent, +8 the date) with his national side (and the opponent, if
/// given) and a date before the requested one: he is in the team (text 0xa62fc8) and returns the national side.
/// Otherwise the all-star team (competition `[[0xadadfc] + 4·[0x9cf690]]`: the match date through record +0xa3, day +3
/// and year +5 + year +0x40, 0x549ef0) for a club player (+0x39) of league `[0x9cf590]` (+0x57), with the date in the
/// window: the 50 names (+0xd7) of both teams (first `[0x9d00a8]`; text 0xa62fa4 into 0xde1f64 if there is a
/// `comment`). Otherwise 0.
/// Original: `NationalTeams::get_international_duty_club` (0x7757d0, national_teams.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7757d0 } (thiscall, ret 0x14, national_teams.cpp): `NationalTeams::get_international_duty_club(jogador, data,
/// clube, comentario, adversario)`, da escalação (0x6dbb80, 0x6dbc50), dos olheiros, das transferências e das telas.
/// Com uma lista provisória (0x68d020, destruída no fim por 0x68d040). Clube seleção (0x53b900) ou um dos times das
/// estrelas (id em [0x9d00a4]/[0x9d00a8]): 0. A janela vai da véspera (0x54c8e0) ao dia seguinte (0x54c5b0). Jogador
/// convocado (+0x24, a seleção): os jogos da seleção na janela (0x5acbe0, lado pelo bit 4 de +0x5e: ligado 1, senão 0;
/// os dias e anos como os dwords da original); com jogo e sem adversário, ou com o adversário (+0x14 do jogo) entre
/// eles, ele está a serviço (o texto 0xa62ff4 em `comentario`, 0x66f4e0) e devolve a seleção. Com o bit 4 ligado, as
/// convocações da lista [this] (+0 a seleção, +4 o adversário, +8 a data) com a seleção dele (e o adversário, se dado)
/// e data antes da pedida: está no time (o texto 0xa62fc8) e devolve a seleção. Senão o time das estrelas (a
/// competição [[0xadadfc] + 4·[0x9cf690]]: a data do jogo pelo registro +0xa3, dia +3 e ano +5 + o ano +0x40, 0x549ef0)
/// para jogador de clube (+0x39) da liga [0x9cf590] (+0x57), com a data na janela: os 50 nomes (+0xd7) dos dois times
/// (primeiro [0x9d00a8]; o texto 0xa62fa4 em 0xde1f64 se há `comentario`). Senão 0.
pub(crate) unsafe extern "thiscall" fn get_international_duty_club(this: usize, player: usize, data: usize, club: usize, comentario: usize,
                                                      opponent: usize) -> usize {
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut list = [0u8; 0x10];
    let pl = list.as_mut_ptr() as usize;
    create(pl);
    let sai = |r: usize| -> usize {
        destroy(pl);
        r
    };
    if club != 0 {
        let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        if national_team(club) != 0 {
            return sai(0);
        }
        let id = read_usize(club);
        if id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) || id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) {
            return sai(0);
        }
    }
    let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let (mut lo, mut hi) = ([0u8; 8], [0u8; 8]);
    let (plo, phi) = (lo.as_mut_ptr() as usize, hi.as_mut_ptr() as usize);
    minus(data, plo, 1); // cpz6: a véspera
    more(data, phi, 1); // cpz6: o dia seguinte
    let nacional = read_usize(player + staff::NATIONAL_JOB);
    if nacional != 0 {
        let sem_bit = (read_u8(player + staff::PLAYING_SQUAD) & 4 == 0) as u32;
        // simplification: the original pushes the side with the high 24 bits of ecx left over; 0x5acbe0 only reads the byte
        let games: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        let n = games(read_usize(G_FIXTURE_MANAGER), read_usize(plo) as u32, read_usize(plo + 2) as u32, read_usize(phi) as u32, read_usize(phi + 2) as u32,
            read_usize(nacional) as u32, pl, u32::MAX, 0xffff_fffe, (sem_bit == 0) as u32);
        let mut servico = false;
        if n as u16 != 0 {
            if opponent == 0 {
                servico = true;
            } else {
                core::ptr::write_unaligned((pl + 0xa) as *mut usize, read_usize(pl + 2));
                loop {
                    let j = next(pl);
                    if j == 0 {
                        break;
                    }
                    if read_usize(j + 0x14) == opponent {
                        servico = true;
                        break;
                    }
                }
            }
        }
        if servico {
            if comentario != 0 {
                text(comentario, 0xa62ff4, 0xad9c64usize);
            }
            crate::comparison::branch("cpz_6 a servico da selecao");
            return sai(read_usize(player + staff::NATIONAL_JOB)); // cpz6: a serviço da seleção
        }
        let conv = read_usize(this);
        if conv != 0 && sem_bit == 0 {
            core::ptr::write_unaligned((conv + 0xa) as *mut usize, read_usize(conv + 2));
            loop {
                let e = next(read_usize(this));
                if e == 0 {
                    break;
                }
                if read_usize(e) != read_usize(read_usize(player + staff::NATIONAL_JOB)) {
                    continue;
                }
                if opponent != 0 && read_usize(e + 4) != read_usize(opponent) {
                    continue;
                }
                let (dia, year) = (read_u16(e + 8) as i16, read_u16(e + 0xa));
                let today = read_u16(data + 2);
                if year < today || (year == today && dia < read_u16(data) as i16) {
                    if comentario != 0 {
                        text(comentario, 0xa62fc8, 0xad9c64usize);
                    }
                    crate::comparison::branch("cpz_6 no time da selecao");
                    return sai(read_usize(player + staff::NATIONAL_JOB)); // cpz6: já no time da seleção
                }
            }
        }
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS).wrapping_mul(4)));
    if comp == 0 {
        return sai(0);
    }
    let reg = read_usize(comp + cup::ROUND_TABLE);
    let year = (read_u8(reg + 5) as i8 as i16).wrapping_add(read_u16(comp + cup::YEAR) as i16) as u16;
    let data_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mut game = [0u8; 8];
    let pj = game.as_mut_ptr() as usize;
    data_new(pj, read_u16(reg + 3) as u32, year as u32);
    let cj = read_usize(player + staff::CLUB_JOB);
    if cj == 0 {
        return sai(0);
    }
    let league = read_usize(cj + club::DIVISION);
    if league == 0 || read_usize(league) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return sai(0);
    }
    let (jd, ja) = (read_u16(pj) as i16, read_u16(pj + 2));
    let (ld, la) = (read_u16(plo) as i16, read_u16(plo + 2));
    if la > ja || (la == ja && ld > jd) {
        return sai(0);
    }
    let (hd, ha) = (read_u16(phi) as i16, read_u16(phi + 2));
    if ha < ja || (ha == ja && hd < jd) {
        return sai(0);
    }
    for i in 0..0x32usize {
        for g in [0x9d00a8usize, 0x9d00a4] {
            if read_usize(all_star_club(g) + 0xd7 + 4 * i) == player {
                if comentario != 0 {
                    text(0xde1f64, 0xa62fa4, 0xad9c64usize);
                }
                crate::comparison::branch("cpz_6 time das estrelas");
                return sai(all_star_club(g));
            }
        }
    }
    sai(0)
}


/// Tells whether the player may play for the manager's national side: nationality, under-21 age, current call-up and
/// the second-nationality rule.
///
/// Stdcall, ret 8: `national_teams_player_is_eligible(player, manager)`, from the player search (0x7afa70, 0x807450)
/// and from the comparison screen. No player sheet (+0x61), no national side for the manager (+0x24) or it not being a
/// national side (0x53b900): 0. Already called up for it: 1. A national side of the second set (0x5409b0, the under-21)
/// and age (+0x18) above 21: 0. With +0x22 or bit 2 of +0xc of the person's 79-byte record (`[0xae2d0c]`), the
/// nationality (+0x1a) must be that of the national side (+0x53); called up for another national side of another
/// nation: 0. The national side's nation must be the first or the second (+0x1e) nationality; if the nation is
/// `[0x9cf434]` and so is the second: 0. Otherwise 1.
/// Original: `national_teams_player_is_eligible` (0x76d9b0, national_teams.cpp, stdcall, ret 8).
///
/// [pt] { 0x76d9b0 } (stdcall, ret 8, national_teams.cpp): `national_teams_player_is_eligible(jogador, tecnico)`, da busca de
/// jogadores (0x7afa70, 0x807450) e da tela de comparação. Sem ficha de jogador (+0x61), sem a seleção do técnico (+0x24)
/// ou ela não sendo seleção (0x53b900): 0. Já convocado para ela: 1. Seleção do segundo conjunto (0x5409b0, a sub-21)
/// e idade (+0x18) acima de 21: 0. Com +0x22 ou o bit 2 de +0xc do registro de 79 bytes da pessoa ([0xae2d0c]), a
/// nacionalidade (+0x1a) tem de ser a da seleção (+0x53); convocado para outra seleção de outra nação: 0. A nação da
/// seleção tem de ser a primeira ou a segunda (+0x1e) nacionalidade; sendo a nação [0x9cf434] e a segunda também: 0.
/// Senão 1.
pub(crate) unsafe extern "stdcall" fn national_teams_player_is_eligible(player: usize, manager: usize) -> u32 {
    let sel = read_usize(manager + staff::NATIONAL_JOB);
    if read_usize(player + staff::PLAYER) == 0 || sel == 0 {
        return 0;
    }
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if national_team(sel) == 0 {
        return 0;
    }
    if read_usize(player + staff::NATIONAL_JOB) == sel {
        return 1;
    }
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if second(sel) != 0 && read_u8(player + staff::AGE) as i8 > 0x15 {
        return 0;
    }
    let nation = read_usize(sel + club::NATION);
    let first = read_usize(player + staff::NATION);
    if read_u8(player + staff::INT_APPS) != 0 && first != nation {
        return 0;
    }
    let p = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(player).wrapping_mul(79));
    if read_u8(p + 0xc) & 2 != 0 && read_usize(player + staff::NATION) != nation {
        return 0;
    }
    let other = read_usize(player + staff::NATIONAL_JOB);
    if other != 0 && read_usize(other + club::NATION) != nation {
        return 0;
    }
    let first = read_usize(player + staff::NATION);
    if first != nation && read_usize(player + staff::SECOND_NATION) != nation {
        return 0;
    }
    let x = read_usize(KEY_NATION_PORTUGAL);
    if read_usize(nation) == x {
        let s = read_usize(player + staff::SECOND_NATION);
        if s != 0 && read_usize(s) == x {
            return 0;
        }
    }
    1
}


/// Adds to a list each duty competition it does not yet hold and returns how many were added.
///
/// Ret 4: `NationalTeams::collect_duty_competitions(out)`, from the scouts (0x7f7010). Without the call-up list
/// `[this]` or without `out`: 0. For each call-up, its competition (+4) goes into `out` (0x68d0d0, 4 bytes) if it is
/// not there yet; returns how many were added (byte).
/// Original: `NationalTeams::collect_duty_competitions` (0x775580, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x775580 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::collect_duty_competitions(saida)`, dos olheiros
/// (0x7f7010). Sem a lista de convocações [this] ou sem `saida`: 0. Para cada convocação, a competição dela (+4) entra
/// em `saida` (0x68d0d0, 4 bytes) se ainda não estiver lá; devolve quantas entraram (byte).
pub(crate) unsafe extern "thiscall" fn collect_duty_competitions(this: usize, out: usize) -> u8 {
    let conv = read_usize(this);
    if conv == 0 || out == 0 {
        return 0;
    }
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let mut n: u8 = 0;
    core::ptr::write_unaligned((conv + 0xa) as *mut usize, read_usize(conv + 2));
    let mut e = next(read_usize(this));
    while e != 0 {
        core::ptr::write_unaligned((out + 0xa) as *mut usize, read_usize(out + 2));
        let mut ja = false;
        loop {
            let x = next(out);
            if x == 0 {
                break;
            }
            if read_usize(x) == read_usize(e + 4) {
                ja = true;
                break;
            }
        }
        if !ja {
            append(out, e + 4, 4);
            n = n.wrapping_add(1);
        }
        e = next(read_usize(this));
    }
    n
}

/// Node values of a linked list (head +2, item +0, next +6) without touching the cursor (+0xa).
///
/// [pt] Os valores dos nós de uma lista (cabeça em +2, nó: +0 o item, +6 o próximo), sem mexer no cursor (+0xa).
pub(crate) unsafe fn national_teams_list_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = read_usize(list + 2);
    while no != 0 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

// simplification: 0x775580 appends nodes (allocates): only the original runs. The plan, made earlier through the nodes (without touching the
// cursors): the call-up competitions not yet in `saida`, in order, without repeats; then, the return value
// is how many and the last items of `saida` (4 bytes each) are them.




/// O ensaio do comparador da 0x76bab0: a cópia roda sem escolher elenco (0x76c120), sem dispensar os acima da idade do
/// sub-21 (0x76d880: no ensaio, só o byte +0x6b que ela zera depois da data) e sem apagar repetidos (0x76c060).
pub(crate) static mut SELECT_SQUADS_DRY_RUN: bool = false;
/// No ensaio: a cópia pediu um elenco (a comparação não vale).
pub(crate) static mut SELECT_SQUADS_REQUESTED_SQUAD: bool = false;


/// For squad type (main, B/Olympic, U21) of a nation record: if picked within 14 days and marked, stamps today and
/// moves the squad date to the match date; else calls select_squad (0x76c120).
///
/// [pt] Um elenco do registro `reg` (tipo 0 principal, 1 B/olímpico, 2 sub-21: a marca de hoje em +0x32/+0x46/+0x5a, a
/// data do elenco em +0x3a/+0x4e/+0x62, a marca de convocado em +0x43/+0x57/+0x6b): com o elenco de até 14 dias (0x54cdb0
/// sobre hoje) e a marca (e `extra`), grava hoje (0x417c10) e leva a data do elenco até a data do jogo (+0x2a/+0x28)
/// se ela for anterior; senão pede o elenco (0x76c120 com a nação, o tipo, hoje, a partida e 0).
pub(crate) unsafe fn refresh_or_select_squad(this: usize, p: usize, club: usize, reg: usize, kind: u32, extra: impl FnOnce() -> bool) {
    let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let data_new: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let base = reg + [0x32usize, 0x46, 0x5a][kind as usize];
    let data = base + 8;
    let d = days(G_CURRENT_DATE, read_usize(data) as u32, read_usize(data + 4) as u32);
    if d <= 0xe && read_u8(base + 0x11) != 0 && extra() {
        let mut r = [0u8; 8];
        copy(base, r.as_mut_ptr(), G_CURRENT_DATE);
        let mut game = [0u8; 8];
        data_new(game.as_mut_ptr(), read_i16(p + 0x2a) as u16 as u32, read_i16(p + 0x28) as u16 as u32);
        let (ra, rd) = (read_i16(data + 2) as u16, read_i16(data) as i16);
        let (ja, jd) = (u16::from_le_bytes([game[2], game[3]]), i16::from_le_bytes([game[0], game[1]]));
        if ra < ja || (ra == ja && rd < jd) {
            core::ptr::copy_nonoverlapping(data as *const u8, r.as_mut_ptr(), 8);
            copy(data, r.as_mut_ptr(), game.as_ptr() as usize);
        }
        return;
    }
    if SELECT_SQUADS_DRY_RUN {
        SELECT_SQUADS_REQUESTED_SQUAD = true;
        return;
    }
    let escolher: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32) = core::mem::transmute(FN_NATIONALTEAMS_SELECT_SQUAD);
    crate::comparison::branch("cpx_13 elenco pedido");
    escolher(this, read_usize(club + club::NATION), kind, G_CURRENT_DATE, p, 0); // cpx13: o elenco
}

/// Refreshes the B/Olympic (match +0x4e bit 0x100) or U21 (bit 0x200 or B national team) squad for the club side.
///
/// [pt] Os elencos B/olímpico (+0x4e da partida, bit 0x100) ou sub-21 (bit 0x200, ou o clube é seleção B 0x5409b0) do lado
/// `clube`; o sub-21 só sem convocado: competição [0x9cf7a0] ou a nação fora do vetor [this+0x36].
pub(crate) unsafe fn refresh_minor_squads(this: usize, p: usize, club: usize, reg: usize) {
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let m = read_i16(p + 0x4d) as u16;
    if m & 0x100 != 0 {
        refresh_or_select_squad(this, p, club, reg, 1, || true);
    } else if m & 0x200 != 0 || second(club) != 0 {
        refresh_or_select_squad(this, p, club, reg, 2, || {
            read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) || read_u8(read_usize(read_usize(club + club::NATION)).wrapping_add(read_usize(this + 0x36))) == 0
        });
    }
}

/// Calls release_overage_u21_players, or in dry-run clears the U21 call-up mark (+0x6b) when today is past the U21
/// date.
///
/// [pt] O que a 0x76d880 grava no registro (fora das pessoas): depois da data do sub-21 (+0x62/+0x66, 0x456740 sobre
/// hoje), zera a marca +0x6b.
pub(crate) unsafe fn release_overage_u21(this: usize, club: usize) {
    if !SELECT_SQUADS_DRY_RUN {
        let dispensar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_RELEASE_OVERAGE_U21_PLAYERS);
        return dispensar(this, read_usize(club + club::NATION));
    }
    let after: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let reg = club_nation_record(this, club);
    if after(G_CURRENT_DATE, read_usize(reg + 0x62) as u32, read_usize(reg + 0x66) as u32) != 0 {
        *((reg + 0x6b) as *mut u8) = 0;
    }
}

/// Selects the squads for a fixture: from two weeks before the match, for each side, a senior, B/Olympic or under-21
/// squad at most 14 days old and flagged only moves its date up to the match; otherwise asks for a new one (0x76c120);
/// releases the over-age under-21 players and clears duplicates.
///
/// Ret 4; SEH frame without a destructor: `NationalTeams::select_squads_for_fixture(fixture)`: without flag +0x40 with
/// bit 1 of +0x4e, or with a B national side at home (0x5409b0) with bit 2, nothing. Only from two weeks before the
/// match (0x54cab0 over date +0x2a/+0x28). Home side: outside a B national side, the senior squad
/// (refresh_or_select_squad type 0) and, without bit 2, the release of the under-21 players (0x76d880) of both nations;
/// the smaller squads (refresh_minor_squads). Away side: the same without the release. Finally deletes the duplicates
/// (0x76c060) of both.
/// Original: `NationalTeams::select_squads_for_fixture` (0x76bab0, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x76bab0 } (thiscall, ret 4, national_teams.cpp, quadro SEH sem destrutor): `NationalTeams::select_squads_for_fixture
/// (partida)`: sem a marca +0x40 com o bit 1 de +0x4e, ou mandante seleção B (0x5409b0) com o bit 2, nada. Só a
/// partir de duas semanas antes do jogo (0x54cab0 sobre a data +0x2a/+0x28). Mandante: fora de seleção B, o elenco
/// principal (refresh_or_select_squad tipo 0) e, sem o bit 2, a dispensa do sub-21 (0x76d880) das duas nações; os elencos
/// menores (refresh_minor_squads). Visitante: o mesmo sem a dispensa. Por fim apaga os repetidos (0x76c060) das duas.
pub(crate) unsafe extern "thiscall" fn select_squads_for_fixture(this: usize, p: usize) {
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let data_new: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let weeks_before: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(FN_NATIONAL_TEAMS_REMOVE_DUPLICATE_SQUAD_ENTRIES);
    if read_u8(p + 0x40) == 0 && read_u8(p + 0x4e) & 1 != 0 {
        return;
    }
    if second(read_usize(p + 0x1c)) != 0 && read_u8(p + 0x4e) & 2 != 0 {
        return;
    }
    let mut game = [0u8; 8];
    data_new(game.as_mut_ptr(), read_i16(p + 0x2a) as u16 as u32, read_i16(p + 0x28) as u16 as u32);
    let (hd, ha) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let mut r = [0u8; 8];
    let q = weeks_before(game.as_mut_ptr(), r.as_mut_ptr(), 2); // cpx13: a janela
    let (qa, qd) = (read_i16(q as usize + 2) as u16, read_i16(q as usize));
    if ha < qa || (ha == qa && hd < qd) {
        return;
    }
    let (casa, fora) = (read_usize(p + 0x1c), read_usize(p + 0x20));
    let reg = club_nation_record(this, casa);
    if second(casa) == 0 {
        refresh_or_select_squad(this, p, casa, reg, 0, || true);
        if read_u8(p + 0x4e) & 2 == 0 {
            release_overage_u21(this, read_usize(p + 0x1c));
            release_overage_u21(this, read_usize(p + 0x20));
        }
    }
    refresh_minor_squads(this, p, casa, reg);
    let reg = club_nation_record(this, fora);
    if second(fora) == 0 {
        refresh_or_select_squad(this, p, fora, reg, 0, || true);
    }
    refresh_minor_squads(this, p, fora, reg);
    if !SELECT_SQUADS_DRY_RUN {
        repetidos(read_usize(read_usize(p + 0x1c) + 0x53));
        repetidos(read_usize(read_usize(p + 0x20) + 0x53));
    }
}


/// Club of the person's contract (0x4cf5a0, stored in [0xae1080]; club = table + 0x245 x contract +4), or 0.
///
/// [pt] O clube do contrato da pessoa (0x4cf5a0 de [0xae1050], gravado em [0xae1080]; clube +4 em [0xae23b4], 0x245
/// bytes), ou 0.
pub(crate) unsafe fn club_of_person_contract(person: usize) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
}

/// O ensaio do comparador da 0x76fd10: a cópia roda sem entregar a notícia.
pub(crate) static mut CALLED_UP_NEWS_DRY_RUN: bool = false;

/// With a human manager at the player's club, sends news item 0x21 (the player, the national side and the slot) to the
/// manager of club +0x39 and of the contract club.
///
/// Stdcall, ret 0xc; SEH frame without a destructor: `national_teams_post_player_called_up_news(player, nation, slot)`:
/// only with a human manager (0x7875e0) at the player's club +0x39 or at the contract club (which must exist). News
/// 0x21 (local): 0 the id, 1..3 the +0x33 of +4, +8, +0xc; slot 0: 4 the senior national side (0x53b3d0), 5 = 0; slot
/// 1: the senior side and 1; other: the B team (0x53b4f0) and 0. Delivered to the manager (0x7861e0) of +0x39, if any,
/// and to that of the contract club when it is another one.
/// Original: `national_teams_post_player_called_up_news` (0x76fd10, national_teams.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x76fd10 } (stdcall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor):
/// `national_teams_post_player_called_up_news(jogador, nação, vaga)`: só com técnico humano (0x7875e0) no clube +0x39
/// do jogador ou no clube do contrato (que precisa existir). A notícia 0x21 (local): 0 o id, 1..3 os +0x33 de +4, +8,
/// +0xc; vaga 0: 4 a seleção principal (0x53b3d0), 5 = 0; vaga 1: a principal e 1; outra: a B (0x53b4f0) e 0.
/// Entregue ao técnico (0x7861e0) do +0x39, se houver, e ao do clube do contrato quando é outro.
pub(crate) unsafe extern "stdcall" fn national_teams_post_player_called_up_news(player: usize, nation: usize, slot: u32) {
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let to_club: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let c39 = read_usize(player + staff::CLUB_JOB);
    if c39 == 0 || human(c39) == 0 {
        if club_of_person_contract(player) == 0 {
            return;
        }
        if human(club_of_person_contract(player)) == 0 {
            return;
        }
    }
    crate::comparison::branch("cpx_13 noticia do convocado");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0x21, 0);
    write(n, 0, read_usize(player) as u32);
    write(n, 1, read_usize(read_usize(player + staff::FIRST_NAME) + 0x33) as u32);
    write(n, 2, read_usize(read_usize(player + staff::SECOND_NAME) + 0x33) as u32);
    write(n, 3, read_usize(read_usize(player + staff::COMMON_NAME) + 0x33) as u32);
    match slot as u8 {
        0 => {
            write(n, 4, read_usize(main(nation)) as u32);
            write(n, 5, 0);
        }
        1 => {
            write(n, 4, read_usize(main(nation)) as u32);
            write(n, 5, 1);
        }
        _ => {
            write(n, 4, read_usize(b(nation)) as u32);
            write(n, 5, 0);
        }
    }
    let news = read_usize(G_NEWS);
    let c39 = read_usize(player + staff::CLUB_JOB);
    if c39 != 0 && !CALLED_UP_NEWS_DRY_RUN {
        to_club(news, n, c39);
    }
    if club_of_person_contract(player) == read_usize(player + staff::CLUB_JOB) {
        return;
    }
    let c = club_of_person_contract(player);
    if !CALLED_UP_NEWS_DRY_RUN {
        to_club(news, n, c);
    }
}



// simplification: the cup’s virtual +0x44 appends records to people and the news goes to the managers’ inboxes (they allocate): only
// the original; the copy takes the branch.


/// O ensaio do comparador da 0x76b600: a cópia roda sem o ranking da FIFA (0x58cf70, acrescenta pontos ao histórico),
/// sem a primeira convocação (0x86cc50, vai para o arquivo), sem o jogo pela seleção no registro de transferências
/// (0x8ce920) e sem a notícia aos clubes (0x7706c0, aloca).
pub(crate) static mut INTERNATIONAL_RESULT_DRY_RUN: bool = false;

/// Lineup slot k (0..20) of side in match summary m: m + 0x625 x side + 0x190 + 0x49 x k.
///
/// [pt] A vaga `k` (0..20) da escalação do lado `lado` no resumo do jogo `m`: m + 0x625 × lado + 0x190 + 0x49 × k.
pub(crate) fn lineup_slot(m: usize, side: usize, k: usize) -> usize {
    m.wrapping_add(0x625 * side + 0x190 + 0x49 * k)
}

/// Person in a lineup slot if it counts (id != -1 and starter, or sub who came on) and is in range of the staff table.
///
/// [pt] A pessoa da vaga, se a vaga conta (id ≠ −1 e titular, ou reserva que entrou: +0x32 ≠ 0xff) e a pessoa está na
/// faixa (id < [0xae2364]; o ponteiro nulo passa sem o teste, como na original).
pub(crate) unsafe fn lineup_slot_person(slot: usize, k: usize) -> Option<usize> {
    let id = read_usize(slot) as i32;
    if id == -1 || (k >= 11 && *((slot + 0x32) as *const u8) == 0xff) {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if p != 0 && read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return None;
    }
    Some(p)
}

/// The person's 79-byte general state record in [0xae2d0c].
///
/// [pt] O registro geral da pessoa (79 bytes em [0xae2d0c]).
pub(crate) unsafe fn national_teams_staff_state_record(p: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f))
}

/// National-teams record of the club's nation: [this+4] + 0x6e x id.
///
/// [pt] O registro da nação do clube nas seleções: [this + 4] + 0x6e × id.
pub(crate) unsafe fn club_nation_record(this: usize, club: usize) -> usize {
    read_usize(this + 4).wrapping_add(read_usize(read_usize(club + club::NATION)).wrapping_mul(0x6e))
}

/// If the person's second nation (+0x1e) is the club's nation, makes it the first (+0x1a) and the old first the second.
///
/// [pt] Jogou pela nação do clube: se a segunda nação (+0x1e) é a dele, ela vira a primeira (+0x1a) e a antiga a segunda.
pub(crate) unsafe fn swap_to_club_nation(p: usize, club: usize) {
    if read_usize(club + club::NATION) == read_usize(p + staff::SECOND_NATION) {
        core::ptr::write_unaligned((p + staff::SECOND_NATION) as *mut usize, read_usize(p + staff::NATION));
        core::ptr::write_unaligned((p + staff::NATION) as *mut usize, read_usize(club + club::NATION));
    }
}

/// For both sides: increments the nation's match count (+2 of its record) and calls f(club, slot, k, person) for each
/// counting lineup person.
///
/// [pt] O jogo de nação por lado (o seu código de competição em [p + 0x14]): conta o jogo da nação (+2 do registro dela,
/// curto) e cada pessoa que conta na escalação.
pub(crate) unsafe fn for_each_lineup_person(this: usize, p: usize, m: usize, mut f: impl FnMut(usize, usize, usize, usize)) {
    for lado in 0..2usize {
        let club = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
        let games = (club_nation_record(this, club) + 2) as *mut i16;
        core::ptr::write_unaligned(games, core::ptr::read_unaligned(games).wrapping_add(1));
        for k in 0..20usize {
            let slot = lineup_slot(m, lado, k);
            if let Some(pessoa) = lineup_slot_person(slot, k) {
                f(club, slot, k, pessoa);
            }
        }
    }
}

/// Records a result between two national sides: counts each nation's match; by competition, marks the general record,
/// swaps the nationality or, in real internationals, scores the FIFA ranking and adds caps and goals to everyone who
/// played; clears the type's +0x44 on its date and sends the news to the clubs.
///
/// Ret 8: `NationalTeams::record_international_result(fixture, summary)`: only between two national sides (0x53b900).
/// By the fixture's competition (`[+0x14]`): that of `[0x9cf794]` marks +0xd = 1 in the general record of those who
/// played; that of `[0x9cf798]` sets bit 2 of +0xc and swaps the nation (+0x1a/+0x1e); that of `[0x9cf7a0]` nothing;
/// the others: the FIFA ranking (0x58cf70 on `[this+0x1c]`) and, for those who played, the first call-up (0x86cc50 with
/// the opponent, if +0x22 = 0, after the nation swap), a cap (+0x22) and the goals (+0x23 += +0x21 of the slot) for the
/// national side and the match in the record (0x8ce920 with the nation's number of matches). In all of them, the
/// nation's match count (+2) goes up. Then, per side: the type (1 if +0x40 = 0; otherwise 2 for the second team
/// 0x5409b0, otherwise 0); on the type's date (+0x3a + 20 × type, 0x4a7f20 with today's) zeroes +0x44 + 20 × type; the
/// news to the clubs (0x7706c0) with the star player (slot +0x153) and the list of those who played. No SEH, no box, no
/// random draw.
/// Original: `NationalTeams::record_international_result` (0x76b600, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x76b600 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::record_international_result(partida, resumo)`:
/// só entre duas seleções (0x53b900). Pela competição da partida ([+0x14]): a de [0x9cf794] marca +0xd = 1 no registro
/// geral de quem jogou; a de [0x9cf798] liga o bit 2 do +0xc e troca a nação (+0x1a/+0x1e); a de [0x9cf7a0] nada; as
/// outras: o ranking da FIFA (0x58cf70 em [this+0x1c]) e, de quem jogou, a primeira convocação (0x86cc50 com o
/// adversário, se +0x22 = 0, depois da troca da nação), um jogo (+0x22) e os gols (+0x23 += +0x21 da vaga) pela
/// seleção e o jogo no registro (0x8ce920 com o número de jogos da nação). Em todas, o jogo da nação (+2) sobe. Depois,
/// por lado: o tipo (1 se +0x40 = 0; senão 2 na segunda seleção 0x5409b0, senão 0); na data do tipo (+0x3a + 20 ×
/// tipo, 0x4a7f20 com a de hoje) zera +0x44 + 20 × tipo; a notícia aos clubes (0x7706c0) com o destaque (vaga +0x153) e
/// a lista de quem jogou. Sem SEH, sem caixa, sem sorteio.
pub(crate) unsafe extern "thiscall" fn record_international_result(this: usize, p: usize, m: usize) {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if is_national_team(read_usize(p + 0x1c)) == 0 || is_national_team(read_usize(p + 0x20)) == 0 {
        return;
    }
    let comp = read_usize(read_usize(p + 0x14));
    if comp == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) {
        for_each_lineup_person(this, p, m, |_, _, _, person| *((national_teams_staff_state_record(person) + 0xd) as *mut u8) = 1);
    } else if comp == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
        for_each_lineup_person(this, p, m, |club, _, _, person| {
            *((national_teams_staff_state_record(person) + 0xc) as *mut u8) |= 2;
            swap_to_club_nation(person, club);
        });
    } else if comp != read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
        let fifa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIFA_RANKINGS_RECORD_MATCH_RESULT);
        let first: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FIRST_CAP_RECORD);
        let game_by_national_team: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_RECORD_INTERNATIONAL_APPEARANCE);
        if !INTERNATIONAL_RESULT_DRY_RUN {
            fifa(read_usize(this + 0x1c), p);
        }
        for lado in 0..2usize {
            let club = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
            let games = (club_nation_record(this, club) + 2) as *mut i16;
            core::ptr::write_unaligned(games, core::ptr::read_unaligned(games).wrapping_add(1));
            for k in 0..20usize {
                let slot = lineup_slot(m, lado, k);
                let id = read_usize(slot) as i32;
                if id == -1 {
                    continue;
                }
                if k >= 11 && *((slot + 0x32) as *const u8) == 0xff { // cpw12: a reserva que não entrou
                    crate::comparison::branch("cpw_12 reserva sem entrar");
                    continue;
                }
                let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
                if person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
                    continue;
                }
                if *((person + staff::INT_APPS) as *const u8) == 0 {
                    swap_to_club_nation(person, club);
                    let opponent = if club == read_usize(p + 0x1c) { read_usize(p + 0x20) } else { read_usize(p + 0x1c) };
                    if !INTERNATIONAL_RESULT_DRY_RUN {
                        first(read_usize(G_STAFF_RECORDS), person, opponent);
                    }
                }
                crate::comparison::branch("cpw_12 jogo pela seleção");
                *((person + staff::INT_APPS) as *mut u8) = (*((person + staff::INT_APPS) as *const u8)).wrapping_add(1); // cpw12: um jogo
                *((person + staff::INT_GOALS) as *mut u8) = (*((person + staff::INT_GOALS) as *const u8)).wrapping_add(*((slot + 0x21) as *const u8)); // cpw12: os gols
                if !INTERNATIONAL_RESULT_DRY_RUN {
                    // the original pushes edx with the short of the matches below and the high part of 55 × nation id
                    let id_nation = read_usize(read_usize(club + club::NATION)) as u32;
                    let n = core::ptr::read_unaligned(games) as u16 as u32;
                    game_by_national_team(read_usize(G_TRANSFER_MANAGER), person, (id_nation.wrapping_mul(0x37) & 0xffff_0000) | n);
                }
            }
        }
    }
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let same_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3D_3D);
    let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32) = core::mem::transmute(FN_NATIONALTEAMS_SEND_CLUB_INTERNATIONAL_REPORT_NEWS);
    for lado in 0..2usize {
        let club = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
        let b = *((p + 0x40) as *const u8) == 0;
        let kind: usize = if second(club) == 0 { if b { 1 } else { 0 } } else if b { 1 } else { 2 };
        let reg = club_nation_record(this, club) + 20 * kind;
        if same_data(reg + 0x3a, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 {
            *((reg + 0x44) as *mut u8) = 0;
        }
        let mut list = [0usize; 20];
        let mut q = 0usize;
        for k in 0..20usize {
            if let Some(person) = lineup_slot_person(lineup_slot(m, lado, k), k) {
                list[q] = person;
                q += 1;
            }
        }
        let destaque_k = *((m + 0x625 * lado + 0x153) as *const i8) as isize;
        let destaque_id = read_usize((m + 0x625 * lado + 0x190) .wrapping_add((0x49 * destaque_k) as usize));
        let destaque = read_usize(G_STAFF_TABLE).wrapping_add(destaque_id.wrapping_mul(staff::SIZE));
        if !INTERNATIONAL_RESULT_DRY_RUN {
            // simplification: the original pushes the count in a dword of the local with only the low byte written; 0x7706c0 reads the byte
            news(this, club, p, m, destaque, list.as_mut_ptr(), q as u32);
        }
    }
}

/// Regions 0x76b600 writes: each side's nation record fields and, per lineup person in range, staff +0x1a..+0x24 and
/// state record +0xc/+0xd.
///
/// [pt] As regiões que a 0x76b600 grava por conta própria: de cada lado, o registro da nação (o +2 e os +0x44 dos tipos) e,
/// de cada vaga com pessoa na faixa, os campos +0x1a..+0x24 dela e o +0xc/+0xd do registro geral.
pub(crate) unsafe fn caps_update_regions(this: usize, p: usize, m: usize) -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    for lado in 0..2usize {
        let club = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
        let r = club_nation_record(this, club);
        reg.extend([(r + 2, 2), (r + 0x44, 1), (r + 0x58, 1), (r + 0x6c, 1)]);
        for k in 0..20usize {
            let slot = lineup_slot(m, lado, k);
            let id = read_usize(slot) as i32;
            if id == -1 {
                continue;
            }
            let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            if person != 0 && (read_usize(person) as i32) < read_usize(G_STAFF_COUNT) as i32 {
                reg.push((person + staff::NATION, 10));
                reg.push((national_teams_staff_state_record(person) + 0xc, 2));
            }
        }
    }
    reg
}


/// Writes the squad name (Main, B, Under 21 or Under 23), in lower case when asked.
///
/// Ret 0x14; `this` is not read: `NationalTeams::format_squad_label(type, under23, out, size, lowercase)`: size below
/// 20: the box (line 0x1c98). Writes (0x66f4e0) the squad name: type 0 "Main", 1 "'B'", others "Under 23" (with
/// under23) or "Under 21"; with lowercase = 1, in lower case.
/// Original: `NationalTeams::format_squad_label` (0x775610, national_teams.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x775610 } (thiscall, ret 0x14, national_teams.cpp; `this` não é lido): `NationalTeams::format_squad_label(tipo,
/// sub23, saida, tamanho, minusculo)`: tamanho abaixo de 20: a caixa (linha 0x1c98). Escreve (0x66f4e0) o nome do
/// elenco: tipo 0 "Main", 1 "'B'", outros "Under 23" (com sub23) ou "Under 21"; com minúsculo = 1, em minúsculas.
pub(crate) unsafe extern "thiscall" fn format_squad_label(this: usize, kind: u32, sub23: u32, out: usize, size: i32, minusculo: u32) -> u32 {
    if size < 0x14 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(IDX_FORMAT_SQUAD_LABEL)(this, kind, sub23, out, size, minusculo);
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = match (minusculo as u8 == 1, kind as u8, sub23 != 0) {
        (true, 0, _) => 0x9cd9b4,
        (true, 1, _) => 0xa62fa0,
        (true, _, true) => 0xa62f94,
        (true, _, false) => 0xa62f88,
        (false, 0, _) => 0xa62f80,
        (false, 1, _) => 0xa62fa0,
        (false, _, true) => 0xa62f74,
        (false, _, false) => 0xa62f68,
    };
    text(out, s)
}






/// Nation's record in the national-teams manager: [this+4] + 0x6e x id (signed).
///
/// [pt] O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id.
pub(crate) unsafe fn nation_record(this: usize, nation: usize) -> usize {
    read_usize(this + 4).wrapping_add((read_usize(nation) as i32).wrapping_mul(0x6e) as usize)
}

/// Call-up type of the player in the national side: 2 for second national teams (0x5409b0), else bit 3 of squad marks
/// (+0x5e).
///
/// [pt] O tipo de convocação do jogador no clube da seleção `clube`: 2 no segundo conjunto de seleções (0x5409b0), senão o
/// bit 3 das marcas de elenco (+0x5e).
pub(crate) unsafe fn call_up_type(club: usize, p: usize) -> u8 {
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if second(club) != 0 {
        2
    } else {
        (read_u8(p + staff::PLAYING_SQUAD) >> 3) & 1 // cpv11: o tipo pelas marcas
    }
}

/// Writes news slots 0..3: the player id and the +0x33 keys of his +4, +8 and +0xc pointers.
///
/// [pt] Os dados 0..3 da notícia: o id do jogador e os de +4, +8 e +0xc dele (+0x33 de cada).
pub(crate) unsafe fn set_news_player_slots(n: usize, p: usize) {
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    write(n, 0, read_usize(p) as u32);
    for k in 0..3usize {
        write(n, 1 + k as i8, read_usize(read_usize(p + 4 + 4 * k) + 0x33) as u32);
    }
}

/// Writes news slot 6: severity category of the player's injury (0x62ec00 over the injury type), or 0.
///
/// [pt] O dado 6 da notícia: a gravidade da lesão do jogador (0x62ec00 sobre o tipo do registro 0x62ed70), ou 0.
pub(crate) unsafe fn set_news_injury_severity(n: usize, p: usize) {
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let injury: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let severity: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_INJURY_GET_TYPE_CATEGORY);
    let v = if injury(read_usize(G_INJURY), p) != 0 {
        let t = read_u8(injury(read_usize(G_INJURY), p)) as i8;
        severity(read_usize(G_INJURY), t) as i8 as i32
    } else {
        0
    };
    write(n, 6, v as u32);
}

/// Club of the player's contract (0x4cf5a0, cached in [0xae1080]; 0x245-byte records), or 0.
///
/// [pt] O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0.
pub(crate) unsafe fn club_of_player_contract(p: usize) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
    }
}

/// Withdraws an injured player: with 14 or more injury days, removes the call-up from the national tactics and squad;
/// before the squad date, tells his club, opens the slot and calls up a replacement or asks the human manager.
///
/// Ret 8: `NationalTeams::withdraw_injured_player(p, days)`: with 14 or more days of injury, the called-up player
/// (+0x28 = 0xb) of the national side's club (+0x24) leaves: with an injury of type 0x51 he only leaves the tactic
/// (0x896c60, twice) and the squad (0x7762f0). Otherwise, only before the type's date (+0x3a of the type record:
/// `[this+4]` + 0x6e × nation id + 20 × type; 0x456740 over `[0xae2c90]`) and if list `[this]` does not have the club
/// with a date already past: the news to his club (0x76f7d0), the removal, the type's open slots (+0x45) go up 1, the
/// level (0x76d810) and, if the national side's manager is human and decides (0x600a50, 0x5fd6b0), the request for a
/// replacement (0x7701f0); otherwise the replacement (0x775f90) and the repeated call-ups deleted (0x76c060). Without a
/// club: the original routine (box and 0).
/// Original: `NationalTeams::withdraw_injured_player` (0x775cd0, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x775cd0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::withdraw_injured_player(p, dias)`: com 14 dias
/// de lesão ou mais, o convocado (+0x28 = 0xb) do clube da seleção (+0x24) sai: com a lesão do tipo 0x51 só sai da
/// tática (0x896c60, duas vezes) e do elenco (0x7762f0). Senão, só antes da data do tipo (+0x3a do registro do tipo:
/// [this+4] + 0x6e × id da nação + 20 × tipo; 0x456740 sobre [0xae2c90]) e se a lista [this] não tem o clube com uma
/// data já passada: a notícia ao clube dele (0x76f7d0), a saída, a vaga do tipo (+0x45) sobe 1, o nível (0x76d810) e,
/// se o técnico da seleção é humano e decide (0x600a50, 0x5fd6b0), o pedido de substituto (0x7701f0); senão o
/// substituto (0x775f90) e as convocações repetidas apagadas (0x76c060). Sem clube: a original (caixa e 0).
pub(crate) unsafe extern "thiscall" fn withdraw_injured_player(this: usize, p: usize, days_arg: u32) {
    let days = days_arg as u16 as i16;
    if days < 0xe { // cpv11: o limite de dias
        return;
    }
    let club = read_usize(p + staff::NATIONAL_JOB);
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_WITHDRAW_INJURED_PLAYER)(this, p, days_arg);
    }
    if read_u8(p + staff::JOB_FOR_NATION) != 0xb {
        return;
    }
    let injury: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let mark: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    let remove: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_REMOVE_PLAYER_FROM_SQUAD);
    let greater: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let news: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_PLAYER_WITHDRAWN_NEWS);
    let level_of: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    let request: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_REPLACEMENT_NEEDED_NEWS);
    let substituto: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_CALL_UP_REPLACEMENT);
    let repetidos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(FN_NATIONAL_TEAMS_REMOVE_DUPLICATE_SQUAD_ENTRIES);
    let nation = read_usize(club + club::NATION);
    let rec = nation_record(this, nation);
    // simplification: the original pushes the local’s dword with the type only in the low byte (the others are stack leftovers); the
    // calls read only the byte
    let kind = call_up_type(club, p);
    let inj = injury(read_usize(G_INJURY), p);
    if inj != 0 && read_u8(inj) == 0x51 {
        crate::comparison::branch("cpv_11 doença");
        mark(read_usize(G_TACTICS), club, p, 1, 1);
        mark(read_usize(G_TACTICS), club, p, 0, 1);
        remove(this, nation, kind as u32, p);
        return;
    }
    let slot = rec.wrapping_add((kind as i8 as i32 * 20) as usize);
    if greater(slot + 0x3a, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) == 0 {
        return;
    }
    let list = read_usize(this);
    if list != 0 {
        core::ptr::write_unaligned((list + 0xa) as *mut u32, read_usize(list + 2) as u32);
        let mut e = next(read_usize(this));
        while e != 0 {
            if read_usize(e) == read_usize(club) && greater(G_CURRENT_DATE, read_usize(e + 8) as u32, read_usize(e + 0xc) as u32) != 0 {
                return;
            }
            e = next(read_usize(this));
        }
    }
    if days < 0x1c {
        crate::comparison::branch("cpv_11 lesão de 14 a 27 dias");
    }
    news(this, p, nation, kind as u32);
    mark(read_usize(G_TACTICS), club, p, 1, 1);
    mark(read_usize(G_TACTICS), club, p, 0, 1);
    if kind != 2 {
        crate::comparison::branch("cpv_11 tipo pelas marcas");
    }
    remove(this, nation, kind as u32, p);
    let a = (slot + 0x45) as *mut u8;
    *a = (*a).wrapping_add(1);
    // the base goes in the original’s eax: the slot address with the low byte replaced by (tipo == 2)
    let level = level_of(p, rec, (slot as u32 & !0xff) | (kind == 2) as u32);
    // the level’s local is the record’s with the low byte replaced (only the byte is read)
    let level = (rec as u32 & !0xff) | level as u32;
    let pede = 'decide: {
        if human(club, 0, 1, 0, 0) == 0 {
            if read_usize(club + club::MANAGER) != 0 || human(main(nation), 0, 1, 0, 0) == 0 {
                break 'decide false;
            }
        }
        let t = read_usize(main(nation) + 0xcf);
        kind == 0 || t9a06(read_usize(G_HUMAN_MANAGER), t, core::ptr::null_mut()) != 0
    };
    if pede {
        crate::comparison::branch("cpv_11 pedido de substituto");
        request(this, p, nation, kind as u32, level);
        return;
    }
    crate::comparison::branch("cpv_11 substituto");
    substituto(this, nation, kind as u32, level, p); // cpv11: o substituto
    repetidos(nation);
}

/// Read-only plan of what 0x775cd0 will do for the player: 0 nothing, 1 illness (just leaves), 2 leaves with news.
///
/// [pt] O caminho que a 0x775cd0 vai tomar, só lendo (a lista [this] percorrida pelos nós, sem mexer no cursor): 0 nada, 1
/// doença (só sai), 2 sai com notícia. O clube (+0x24) já conferido.
pub(crate) unsafe fn release_plan(this: usize, p: usize, days: u32) -> u8 {
    if (days as u16 as i16) < 0xe || read_u8(p + staff::JOB_FOR_NATION) != 0xb {
        return 0;
    }
    let injury: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let greater: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let club = read_usize(p + staff::NATIONAL_JOB);
    let kind = call_up_type(club, p);
    let inj = injury(read_usize(G_INJURY), p);
    if inj != 0 && read_u8(inj) == 0x51 {
        return 1;
    }
    let slot = nation_record(this, read_usize(club + club::NATION)).wrapping_add((kind as i8 as i32 * 20) as usize);
    if greater(slot + 0x3a, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) == 0 {
        return 0;
    }
    let list = read_usize(this);
    if list != 0 {
        let mut no = read_usize(list + 2);
        while no != 0 {
            let e = read_usize(no);
            if e == 0 {
                break;
            }
            if read_usize(e) == read_usize(club) && greater(G_CURRENT_DATE, read_usize(e + 8) as u32, read_usize(e + 0xc) as u32) != 0 {
                return 0;
            }
            no = read_usize(no + 6);
        }
    }
    2
}

/// Builds news item 0x20 (the player left the national squad injured) and delivers it to his club and his contract club
/// when human-managed.
///
/// Ret 0xc; SEH frame without a destructor; `this` is not read: `national_teams_post_player_withdrawn_news(p, nation,
/// type)`: if the player's club (+0x39) or the contract club (0x4cf5a0) has a human manager (0x7875e0), builds news
/// 0x20 in a local (0x77bbe0 with 0): data 0..3 the player and those of +4/+8/+0xc, 4 the national side's id (senior
/// 0x53b3d0 in types 0 and 1, otherwise 0x53b4f0), 5 = (type == 1), 6 the injury severity; delivers it to club +0x39
/// and, if it is another one, to the contract club (0x7861e0).
/// Original: `national_teams_post_player_withdrawn_news` (0x76f7d0, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x76f7d0 } (thiscall, ret 0xc, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_player_withdrawn_news(p, nacao, tipo)`: se o clube do jogador (+0x39) ou o do contrato
/// (0x4cf5a0) é de técnico humano (0x7875e0), monta a notícia 0x20 numa local (0x77bbe0 com 0): dados 0..3 o jogador
/// e os de +4/+8/+0xc, 4 o id da seleção (principal 0x53b3d0 nos tipos 0 e 1, senão 0x53b4f0), 5 = (tipo == 1), 6 a
/// gravidade da lesão; entrega ao clube +0x39 e, se for outro, ao do contrato (0x7861e0).
pub(crate) unsafe extern "thiscall" fn national_teams_post_player_withdrawn_news(_this: usize, p: usize, nation: usize, kind: u32) {
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    if read_usize(p + staff::CLUB_JOB) == 0 || human(read_usize(p + staff::CLUB_JOB)) == 0 {
        if club_of_player_contract(p) == 0 {
            return;
        }
        if human(club_of_player_contract(p)) == 0 {
            return;
        }
    }
    crate::comparison::branch("cpv_11 notícia da saída");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construct(n, 0x20, 0);
    set_news_player_slots(n, p);
    let (sel, um) = match kind as u8 {
        0 => (main(nation), 0),
        1 => (main(nation), 1),
        _ => (b_national_team(nation), 0),
    };
    write(n, 4, read_usize(sel) as u32);
    write(n, 5, um);
    set_news_injury_severity(n, p);
    let news = read_usize(G_NEWS);
    if read_usize(p + staff::CLUB_JOB) != 0 {
        deliver(news, n, read_usize(p + staff::CLUB_JOB));
    }
    if club_of_player_contract(p) != read_usize(p + staff::CLUB_JOB) {
        deliver(news, n, club_of_player_contract(p));
    }
}

/// Builds news item 0x22 (a replacement is needed) with the team, squad count, level and type, and delivers it to the
/// national side's human manager.
///
/// Ret 0x10; SEH frame without a destructor; `this` is not read: `national_teams_post_replacement_needed_news(p,
/// nation, type, level)`: news 0x22 to the national side's manager: data 0..3 the player, 4 the team id (type 1: senior
/// with flag 8 and 5 = 1; type 0: senior, otherwise the under-21 0x53b4f0, flag 4 and 5 = 0), 6 the injury severity, 7
/// = 0, 8 those in the squad with the flag (0x53c590), 9 the level, 0xa the type and 0xb the nation id. Delivered to
/// the team if its manager is human (0x600a50), otherwise to the senior side if its manager is.
/// Original: `national_teams_post_replacement_needed_news` (0x7701f0, national_teams.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7701f0 } (thiscall, ret 0x10, national_teams.cpp, quadro SEH sem destrutor; `this` não é lido):
/// `national_teams_post_replacement_needed_news(p, nacao, tipo, nivel)`: a notícia 0x22 ao técnico da seleção: dados
/// 0..3 o jogador, 4 o id do time (tipo 1: principal com marca 8 e 5 = 1; tipo 0: principal, senão o sub-21 0x53b4f0,
/// marca 4 e 5 = 0), 6 a gravidade da lesão, 7 = 0, 8 os do elenco com a marca (0x53c590), 9 o nível, 0xa o tipo e 0xb
/// o id da nação. Entrega ao time se o técnico dele é humano (0x600a50), senão ao principal se o dele for.
pub(crate) unsafe extern "thiscall" fn national_teams_post_replacement_needed_news(_this: usize, p: usize, nation: usize, kind: u32, level: u32) {
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let count: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    crate::comparison::branch("cpv_11 notícia do pedido");
    let (time, um, marca) = match kind as u8 {
        0 => (main(nation), 0u32, 4u32),
        1 => (main(nation), 1, 8),
        _ => (b_national_team(nation), 0, 4),
    };
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construct(n, 0x22, 0);
    set_news_player_slots(n, p);
    write(n, 4, read_usize(time) as u32);
    write(n, 5, um);
    set_news_injury_severity(n, p);
    write(n, 7, 0);
    // simplification: the mark goes in a dword of the local with only the low byte written; 0x53c590 reads only the byte
    write(n, 8, count(time, marca, 0) as i8 as i32 as u32);
    write(n, 9, level as u8 as i8 as i32 as u32);
    write(n, 0xa, kind as u8 as i8 as i32 as u32);
    write(n, 0xb, read_usize(nation) as u32);
    let news = read_usize(G_NEWS);
    if human(time, 0, 1, 0, 0) != 0 {
        deliver(news, n, time);
    } else if human(main(nation), 0, 1, 0, 0) != 0 {
        deliver(news, n, main(nation));
    }
}






/// Writes the national team name (0x542100) into buf and returns the grammar byte (+0x52).
///
/// [pt] O nome do time (0x542100: time, marca de segundo time, destino de 0x47 bytes, 2, 1) e o byte de gramática (+0x52).
pub(crate) unsafe fn national_team_name(team: usize, second: u32, buf: &mut [u8; 0x48]) -> i32 {
    let name: unsafe extern "C" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_GET_NATIONAL_TEAM_NAME);
    name(team, second, buf.as_mut_ptr(), 0x47, 2, 1);
    read_u8(team + club::SHORT_GENDER_NAME) as i8 as i32
}

/// Tells whether the player may join or leave the squad now (retired, other nation, full squad, injured, squad window,
/// date and match day, human manager) and writes the reason.
///
/// Ret 0x18; SEH frame with the local list of the match search: `national_teams_can_change_squad(nation, type, p,
/// joining, msg, size)`: a player (if any) retired from the national side (0x547d30) or from a club of another nation
/// cannot change (0). The team of the type (0 and 1 the senior side, type 1 with flag 8 and the second team; otherwise
/// the under-21 0x53b4f0, flag 4). Joining: a full squad (0x53c590 against +0x42 of the type record) or an injured
/// player (0x62c400 = 2) cannot change. Can change (1): joining with open slots (+0x45), the type with no window (+0x44
/// = 0), the type's date (+0x3a) already past or, on the day itself, a team match today (0x5acee0 into a local list);
/// in types 1 and 2 with a human manager who decides (0x600a50), also when list `[this]` does not have the team with a
/// past date and he does not allow it (0x5fd6b0). The reasons go to `msg` (0x66f4e0). Without a nation, or `msg` with
/// fewer than 0x32 bytes: the original routine (box and 1).
/// Original: `national_teams_can_change_squad` (0x76db00, national_teams.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x76db00 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH com a lista local da busca de jogos):
/// `national_teams_can_change_squad(nacao, tipo, p, entrando, msg, tam)`: o jogador (se houver) aposentado da seleção
/// (0x547d30) ou de clube de outra nação não muda (0). O time do tipo (0 e 1 a principal, o 1 com a marca 8 e o segundo
/// time; senão o sub-21 0x53b4f0, marca 4). Entrando: elenco cheio (0x53c590 contra +0x42 do registro do tipo) ou
/// jogador machucado (0x62c400 = 2) não muda. Pode mudar (1): entrando com vagas abertas (+0x45), o tipo sem janela
/// (+0x44 = 0), a data do tipo (+0x3a) já passada ou, no próprio dia, um jogo do time hoje (0x5acee0 numa lista local);
/// nos tipos 1 e 2 com técnico humano que decide (0x600a50), também quando a lista [this] não tem o time com data
/// passada e ele não deixa (0x5fd6b0). Os motivos vão para `msg` (0x66f4e0). Sem nação, ou `msg` com menos de 0x32
/// bytes: a original (caixa e 1).
pub(crate) unsafe extern "thiscall" fn national_teams_can_change_squad(this: usize, nation: usize, kind: u32, p: usize, entrando: u32, msg: usize, size: i32) -> u32 {
    if nation == 0 || (msg != 0 && size < 0x32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(IDX_NATIONAL_TEAMS_CAN_CHANGE_SQUAD)(this, nation, kind, p, entrando, msg, size);
    }
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let aposentado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let name_of_player: unsafe extern "C" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let count: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let state: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_ALL_NATIONAL_TEAMS);
    if p != 0 {
        if aposentado(p, main(read_usize(p + staff::NATION))) != 0 { // cpv12: aposentado
            crate::comparison::branch("cpv_12 aposentado");
            if msg != 0 {
                let mut name = [0u8; 0x12c];
                name_of_player(p, name.as_mut_ptr(), 0x12c, 1);
                text(msg, 0xa5b4c8, name.as_ptr());
            }
            return 0;
        }
        let c = read_usize(p + staff::NATIONAL_JOB);
        if c != 0 && read_usize(c + club::NATION) != nation {
            if msg != 0 {
                let mut name = [0u8; 0x12c];
                name_of_player(p, name.as_mut_ptr(), 0x12c, 1);
                let other = read_usize(read_usize(p + staff::NATIONAL_JOB) + 0x53);
                let g = read_u8(other + nation::SHORT_GENDER_NAME) as i8 as i32;
                text(msg, 0xa5b460, g, g, name.as_ptr(), other + nation::SHORT_NAME);
            }
            return 0;
        }
    }
    let s = kind as u8;
    // the flag goes in the local [esp+0x1c] only in the low byte (the rest is stack leftover); 0x53c590 reads only the byte
    let (time, segundo, marca) = match s {
        0 => (main(nation), 0u32, 4u32),
        1 => (main(nation), 1, 8),
        _ => (b_national_team(nation), 0, 4),
    };
    let mut manager = read_usize(main(nation) + 0xcf);
    if human(main(nation), 0, 1, 0, 0) == 0 {
        manager = 0;
    }
    let rec = read_usize(this + 4).wrapping_add((read_usize(nation) as i32).wrapping_mul(0x6e) as usize);
    let e = rec.wrapping_add((s as i8 as i32 * 20) as usize);
    if entrando != 0 {
        let ceiling = read_u8(e + 0x42) as i8;
        if (count(time, marca, 0) as i8) >= ceiling {
            if msg != 0 {
                let mut buf = [0u8; 0x48];
                let g = national_team_name(time, segundo, &mut buf);
                text(msg, 0xa5b3e0, g, g, ceiling as i32, buf.as_ptr());
            }
            return 0;
        }
        if state(read_usize(G_INJURY), p) == 2 {
            if msg != 0 {
                text(msg, 0xa5b3bc);
            }
            return 0;
        }
    }
    if read_u8(e + 0x45) != 0 && entrando != 0 {
        return 1;
    }
    if read_u8(e + 0x44) == 0 {
        crate::comparison::branch("cpv_12 sem janela");
        return 1; // cpv12: sem janela
    }
    let today = read_usize(G_CURRENT_DATE) as u32;
    let (ano, dia) = ((today >> 16) as u16, today as u16 as i16);
    if s != 0 && manager != 0 {
        let mut passou = false;
        let list = read_usize(this);
        if list != 0 {
            core::ptr::write_unaligned((list + 0xa) as *mut u32, read_usize(list + 2) as u32);
            let mut it = next(read_usize(this));
            while it != 0 {
                if read_usize(it) == read_usize(time) {
                    let d = read_usize(it + 8) as u32;
                    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
                    if a < ano || (a == ano && dd < dia) {
                        passou = true;
                        break;
                    }
                }
                it = next(read_usize(this));
            }
        }
        if !passou && t9a06(read_usize(G_HUMAN_MANAGER), manager, core::ptr::null_mut()) == 0 {
            return 1;
        }
    }
    let d = read_usize(e + 0x3a) as u32;
    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
    if a < ano || (a == ano && dd < dia) {
        crate::comparison::branch("cpv_12 data passada");
        return 1; // cpv12: data passada
    }
    if a == ano && dd as u16 == dia as u16 {
        crate::comparison::branch("cpv_12 jogo hoje");
        let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let search: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, u32, u32, u32, u32, usize, u32, i32, u32, u32) -> u16 =
            core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        // the date slots the search reads (the year word, the day dword, the slice byte), as the original builds them
        let mut v_year: u32 = ano as u32;
        let mut v_day: u32 = today;
        let mut slice: u8 = 0;
        let mut l = [0u32; 4];
        let lp = l.as_mut_ptr() as usize;
        new(lp);
        let found = search(read_usize(G_FIXTURE_MANAGER), &mut v_year, &mut v_day, &mut slice, 1, -1, -1, -1, read_usize(time) as u32, -1, 0, 0, 0, 1, lp, 1, -2,
            (segundo == 0) as u32, 0);
        destroy(lp);
        if found != 0 {
            return 1;
        }
    }
    if msg != 0 {
        let mut buf = [0u8; 0x48];
        let g = national_team_name(time, segundo, &mut buf);
        text(msg, if entrando != 0 { 0xa5b34c } else { 0xa5b2d8 }, g, g, buf.as_ptr());
    }
    0
}

/// Tells whether the player is in the squad of the type (senior, second or under-21) with the type's flag.
///
/// Stdcall, ret 0xc: `national_teams_player_in_squad(p, nation, type)`: 1 if the club of the player's national side
/// (+0x24) is the team of the type (0 and 1 the senior side 0x53b3d0, 2 the under-21 0x53b4f0) and he has the type's
/// flag (+0x5e: 4 in types 0 and 2, 8 in 1). No club or another type: 0.
/// Original: `national_teams_player_in_squad` (0x76da80, national_teams.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x76da80 } (stdcall, ret 0xc, national_teams.cpp): `national_teams_player_in_squad(p, nacao, tipo)`: 1 se o
/// clube da seleção do jogador (+0x24) é o time do tipo (0 e 1 a principal 0x53b3d0, 2 o sub-21 0x53b4f0) e ele tem a
/// marca do tipo (+0x5e: 4 nos tipos 0 e 2, 8 no 1). Sem clube ou outro tipo: 0.
pub(crate) unsafe extern "stdcall" fn national_teams_player_in_squad(p: usize, nation: usize, kind: u32) -> u32 {
    if read_usize(p + staff::NATIONAL_JOB) == 0 {
        return 0;
    }
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b_national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let (time, marca) = match kind as u8 {
        0 => (main(nation), 4u8),
        1 => (main(nation), 8),
        2 => (b_national_team(nation), 4),
        _ => return 0,
    };
    (read_usize(p + staff::NATIONAL_JOB) == time && read_u8(p + staff::PLAYING_SQUAD) & marca != 0) as u32
}







/// Day (signed word) and year (unsigned word) of an 8-byte date.
///
/// [pt] Dia (palavra com sinal) e ano (palavra sem sinal) de uma data de 8 bytes.
pub(crate) unsafe fn national_teams_date_day_year(p: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p + 2) as *const u16))
}

/// Does a regular club have more than 2 players called up between dates a and b (sums +8 of the club's dated call-up
/// list)?
///
/// [pt] Os convocados do clube (id < [0xae235c] − 2 × [0xae2350]: clube comum, não seleção) entre as datas `a` e `b`: a
/// lista do clube em [this + 0x20] + 6 × id (+0 os registros de 9 bytes em ordem de data, +5 a conta em byte com sinal);
/// pula os antes de `a`, para no primeiro depois de `b` e soma os bytes +8 (em byte, com a volta). Mais de 2: demais.
pub(crate) unsafe fn has_too_many_called_up(this: usize, club: usize, a: usize, b: usize) -> bool {
    if club == 0 {
        return false;
    }
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2));
    let id = read_i32(club);
    if id >= limit {
        return false;
    }
    let rec = read_usize(this + 0x20).wrapping_add((id as usize).wrapping_mul(6));
    let n = *((rec + 5) as *const i8);
    let list = read_usize(rec);
    let (a_dia, a_ano) = national_teams_date_day_year(a);
    let (b_dia, b_ano) = national_teams_date_day_year(b);
    let mut sum: i8 = 0;
    let mut k: i8 = 0;
    while k < n {
        let e = list.wrapping_add(k as isize as usize * 9);
        let (dia, ano) = national_teams_date_day_year(e);
        k = k.wrapping_add(1);
        if a_ano > ano || (a_ano == ano && a_dia > dia) {
            continue; // before the window
        }
        if b_ano < ano || (b_ano == ano && b_dia < dia) {
            break; // after the window: the list is in order
        }
        crate::comparison::branch("cpp_12 convocado na janela");
        sum = sum.wrapping_add(*((e + 8) as *const i8));
    }
    sum > 2 // more than two called up
}

/// friendly fixture: 1; otherwise 0 when the home or away club (a plain club) has more than 2 players on call-up from a
/// day before to a day after the fixture date, else 1; turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: the detour alone (without
/// running the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of
/// batch 252): the detour changes the stack residue a later original reads; the copy stays in the code and does not run
/// Original: `NationalTeams::can_schedule_club_fixture` (0x776fc0, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x776fc0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::can_schedule_club_fixture(jogo)`: com a data
/// do jogo (0x549ef0 com +0x2a e +0x28), a janela de um dia antes (0x54c8e0) e um depois (0x54c5b0); jogo da competição
/// amistosa (0x5c6710 em [0xae2c74] com +0x14): 1. Senão 0 quando o mandante (+0x1c) ou o visitante (+0x20) tem mais
/// de 2 convocados na janela; senão 1. O quadro SEH só guarda o estado (as datas não têm destrutor).
pub(crate) unsafe extern "thiscall" fn can_schedule_club_fixture(this: usize, game: usize) -> u32 {
    let data_new: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(FN_DATE_DATE);
    let minus_days: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let more_days: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let (mut d, mut a, mut b) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // simplification: the original pushes the words +0x2a and +0x28 with the high part of the register (garbage); 0x549ef0 only reads the
    // low words: the copy passes the clean words.
    data_new(d.as_mut_ptr(), *((game + 0x2a) as *const u16) as u32, *((game + 0x28) as *const u16) as u32);
    minus_days(d.as_mut_ptr(), a.as_mut_ptr(), 1);
    more_days(d.as_mut_ptr(), b.as_mut_ptr(), 1);
    if amistosa(read_usize(G_FRIENDLY), read_usize(game + 0x14)) != 0 {
        return 1;
    }
    let (pa, pb) = (a.as_ptr() as usize, b.as_ptr() as usize);
    if has_too_many_called_up(this, read_usize(game + 0x1c), pa, pb) {
        return 0;
    }
    if has_too_many_called_up(this, read_usize(game + 0x20), pa, pb) {
        crate::comparison::branch("cpp_12 visitante convocado");
        return 0; // the away club with too many called up
    }
    1
}

/// qsort comparer of records by the date at +0 (0x456770/0x456740), earliest first.
///
/// `compare_call_up_records_by_date(a, b)`: the qsort of records with the date at +0 (the 9-byte call-ups, the
/// injuries, the transfers): −1 if that of `a` is earlier (0x456770), 1 if later (0x456740), otherwise 0. Hot (qsort):
/// no counter.
/// Original: `compare_call_up_records_by_date` (0x778e90, national_teams.cpp, cdecl).
///
/// [pt] { 0x778e90 } (cdecl, national_teams.cpp): `compare_call_up_records_by_date(a, b)`: o qsort dos registros com a data
/// em +0 (as convocações de 9 bytes, as lesões, as transferências): −1 se a de `a` é antes (0x456770), 1 se é depois
/// (0x456740), senão 0. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_call_up_records_by_date(a: usize, b: usize) -> i32 {
    let before: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let after: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let (d0, d1) = (read_usize(b) as u32, read_usize(b + 4) as u32);
    let r = if before(a, d0, d1) != 0 { -1 } else { (after(a, d0, d1) != 0) as i32 };
    r // the earliest call-up first
}

/// With nation and finals both non-null, appends the pair to the new qualifiers list (+0x28).
///
/// Ret 8: `NationalTeams::add_qualified_nation(nation, finals)`: with both non-null, appends the 8-byte record (nation,
/// finals) to the list of new qualifiers at this + 0x28 (LLIST, 0x68d0d0).
/// Original: `NationalTeams::add_qualified_nation` (0x7779b0, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x7779b0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::add_qualified_nation(nacao, fase_final)`: com
/// as duas não nulas, acrescenta o registro de 8 bytes (nação, fase final) à lista das novas classificadas em
/// this + 0x28 (LLIST, 0x68d0d0).
pub(crate) unsafe extern "thiscall" fn add_qualified_nation(this: usize, nation: usize, stage_final: usize) {
    if nation == 0 || stage_final == 0 {
        return;
    }
    let mut reg = [0u8; 8];
    reg[..4].copy_from_slice(&(nation as u32).to_le_bytes());
    reg[4..].copy_from_slice(&(stage_final as u32).to_le_bytes());
    let append: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    append(this + 0x28, reg.as_mut_ptr() as usize, 8);
}






/// Is competition id one of the major finals (World Cup, Olympics, Euro, Copa America, ACN, Asian Cup, OFC Nations Cup,
/// Asian Games, Confederations Cup)?
///
/// [pt] A competição (id) é uma das fases finais grandes ([0x9cf79c], [0x9cf7a0], [0x9cf7a4], [0x9cf7a8], [0x9cf7ac],
/// [0x9cf790], [0x9cf8a0], [0x9cf78c], [0x9cf964])?
pub(crate) unsafe fn is_major_finals(id: u32) -> bool {
    [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
        .iter()
        .any(|&g| read_u32_as_usize(g) as u32 == id)
}

/// O que a 0x775220 grava: a competição atual da nação ([this+4] + id × 110 + 4), a da fila ([this+8] + id × 4) e se
/// avisa o gerenciador dos técnicos.
pub(crate) struct NationCurrentCompUpdate {
    pub(crate) vaga: usize,
    pub(crate) nova: Option<u32>,
    pub(crate) fila: usize,
    pub(crate) na_fila: Option<u32>,
    pub(crate) avisa: bool,
}

/// Read-only plan of 0x775220: whether the new competition becomes current or is queued, what goes to the queue, and
/// whether the manager manager is notified.
///
/// [pt] O plano da 0x775220 (só lê): com competição atual (≠ −1) que é fase final grande, a nova vai para a fila e a atual
/// fica (sem aviso); senão, se a nova é fase final grande e há atual, a atual vai para a fila; a nova vira a atual e o
/// gerenciador dos técnicos ([0xb67a44], se existe) é avisado.
pub(crate) unsafe fn current_comp_plan(this: usize, nacao: usize, comp: usize) -> NationCurrentCompUpdate {
    let id = read_u32_as_usize(nacao);
    let vaga = read_u32_as_usize(this + 4).wrapping_add(id.wrapping_mul(110)) + 4;
    let fila = read_u32_as_usize(this + 8).wrapping_add(id.wrapping_mul(4));
    let atual = read_u32_as_usize(vaga) as u32;
    let nova = read_u32_as_usize(comp) as u32;
    let mut p = NationCurrentCompUpdate { vaga, nova: Some(nova), fila, na_fila: None, avisa: read_u32_as_usize(G_MANAGER_MANAGER) != 0 };
    if atual != 0xffff_ffff {
        let r = read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add((atual as usize).wrapping_mul(107)); // 107-byte records
        if r != 0 && is_major_finals(read_u32_as_usize(r) as u32) {
            return NationCurrentCompUpdate { vaga, nova: None, fila, na_fila: Some(nova), avisa: false };
        }
        if is_major_finals(nova) {
            p.na_fila = Some(atual);
        }
    }
    p
}

/// makes the competition the nation's current one; with a major finals under way, the new one goes to the queue; when
/// the new one is a major finals, the current one goes to the queue; notifies the manager handler (0x6a3770)
/// Original: `NationalTeams::set_current_competition` (0x775220, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x775220 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::set_current_competition(nacao, comp)`: a
/// competição atual da nação vira `comp` (plano em `current_comp_plan`: a fase final grande em curso fica na frente e a
/// outra vai para a fila) e o gerenciador dos técnicos é avisado (0x6a3770 com o clube da seleção principal, 0x53b3d0,
/// e 1). Nação ou competição nula: a caixa "Error" (e volta).
pub(crate) unsafe extern "thiscall" fn set_current_competition(this: usize, nation: usize, comp: usize) {
    if nation == 0 || comp == 0 {
        show_error_at(0xa5b2a4, 0x1bb2);
        return;
    }
    let p = current_comp_plan(this, nation, comp);
    if let Some(f) = p.na_fila {
        write_u32(p.fila, f);
    }
    let Some(nova) = p.nova else {
        crate::comparison::branch("cpo_13 fase final fica");
        return;
    };
    crate::comparison::branch("cpo_13 competicao atual");
    write_u32(p.vaga, nova);
    let g = read_u32_as_usize(G_MANAGER_MANAGER);
    if g != 0 {
        let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
        let warn: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
        warn(g, main(nation), 1);
    }
}


/// Writes code for club c (if present and id < n) into the table at [this+0x24].
///
/// [pt] Marca o clube `c` (se existe e o id < n) com o código na tabela [this+0x24].
pub(crate) unsafe fn mark_club_code(this: usize, c: usize, n: i32, code: u8) {
    if c != 0 && (read_u32_as_usize(c) as i32) < n {
        *(read_u32_as_usize(this + 0x24).wrapping_add(read_u32_as_usize(c)) as *mut u8) = code;
    }
}

/// Does the club have a nation (+0x53) with byte +0x11c set?
///
/// [pt] O clube tem nação marcada (+0x53, +0x11c ≠ 0)?
pub(crate) unsafe fn has_flagged_nation(c: usize) -> bool {
    let n = read_u32_as_usize(c + club::NATION);
    n != 0 && *((n + nation::NATION_LEAGUE_SELECTED) as *const u8) != 0
}

/// in the one-byte-per-club table, marks the clubs of the non-friendly fixtures on the date: 1 the club's league game,
/// 2 another, 3 a club of a flagged nation (entry with no nation); turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied: with the detour the
/// career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news
/// constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the
/// copy) is not isolated yet; the copy stays in the code and does not run
/// Original: `NationalTeams::mark_clubs_with_fixtures` (0x7775c0, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x7775c0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::mark_clubs_with_fixtures(data)`: a tabela de um
/// byte por clube ([this+0x24], n = clubes [0xae235c] − 2 × nações [0xae2350]; calloc na primeira, sem memória a
/// caixa "Error") recebe, para cada jogo não amistoso (0x5c6710) da data nas 3 fatias (0x5ae6a0) de cada uma das 0x21
/// entradas de 0x48 bytes de 0xb63df0 (a nação +0 ativa em +0x11c, ou sem nação; o grupo +0x14): sem nação, 3 para os
/// clubes (+0x1c, +0x20) de nação ativa; com nação, 1 quando a competição do jogo (+0x14) é a da liga do mandante (+0x57)
/// ou a +0x60 do visitante, senão 2, para os dois. Lida pela remarcação dos jogos dos clubes com convocados (0x7777e0).
pub(crate) unsafe extern "thiscall" fn mark_clubs_with_fixtures(this: usize, data: usize) {
    let n = (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    if read_u32_as_usize(this + 0x24) == 0 {
        let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
        let p = calloc(n, 1);
        write_u32(this + 0x24, p as u32);
        if p == 0 {
            show_error_at(0xa5b2a4, 0x2229);
            return;
        }
    }
    let get_fixture_list_for_date: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    for i in 0..0x21usize {
        let e = G_LEAGUE_WORLD_CLUB_NATION + i * 0x48;
        let nation = read_u32_as_usize(e);
        if nation != 0 && *((nation + nation::NATION_LEAGUE_SELECTED) as *const u8) == 0 {
            continue;
        }
        for fatia in 0..3u32 {
            // the original pushes the slice in a stack dword with a garbage high part and the group in al: 0x5abf10 only reads the bytes
            let list = get_fixture_list_for_date(data, fatia, *((e + 0x14) as *const u8) as u32);
            if list == 0 {
                continue;
            }
            write_u32(list + 0xa, read_u32_as_usize(list + 2) as u32);
            let mut game = next(list);
            while game != 0 {
                if friendly(read_u32_as_usize(G_FRIENDLY), read_u32_as_usize(game + 0x14)) == 0 {
                    let (casa, fora) = (read_u32_as_usize(game + 0x1c), read_u32_as_usize(game + 0x20));
                    if read_u32_as_usize(e) == 0 {
                        if casa != 0 && has_flagged_nation(casa) {
                            mark_club_code(this, casa, n, 3);
                        }
                        if fora != 0 && has_flagged_nation(fora) {
                            mark_club_code(this, fora, n, 3);
                        }
                    } else {
                        let comp = read_u32_as_usize(game + 0x14);
                        // the away side +0x60 is read without checking the pointer (like the original)
                        let code = if casa != 0 && (read_u32_as_usize(casa + club::DIVISION) == comp || read_u32_as_usize(fora + club::RESERVE_DIVISION) == comp) { 1 } else { 2 };
                        crate::comparison::branch("cpo_13 clubes com jogo");
                        mark_club_code(this, casa, n, code); // o mandante
                        mark_club_code(this, fora, n, code); // o visitante
                    }
                }
                game = next(list);
            }
        }
    }
}







/// Items of list l (head +2, item +0, next +6, capped at 100000) without touching the cursor.
///
/// [pt] Os itens da lista `l` (0x68d0d0: começo em +2, nó = item +0 e seguinte +6), sem mexer no cursor (+0xa).
pub(crate) unsafe fn list_items_capped(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = read_usize(l + 2);
    while no != 0 && v.len() < 100_000 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

/// Ends call-ups in list [this]: resets the cursor and sets +0x10 to tomorrow's day for each record passing the filter.
///
/// [pt] O fim das convocações da lista [this] (as duas rotinas da 0x774f30/0x774f90): do começo (cursor +0xa = +2),
/// cada registro que `passa` recebe em +0x10 a palavra +0 (o dia) de amanhã (0x54c5b0 com 1 sobre [0xae2c90]).
pub(crate) unsafe fn end_call_ups(this: usize, passa: impl Fn(usize) -> bool) {
    let l = read_usize(this);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let more_days: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *const u16 = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mut r = next(read_usize(this));
    while r != 0 {
        if passa(r) {
            let mut data = [0u8; 8];
            let day = *more_days(G_CURRENT_DATE, data.as_mut_ptr(), 1);
            core::ptr::write_unaligned((r + 0x10) as *mut u16, day); // tomorrow’s day
        }
        r = next(read_usize(this));
    }
}

/// In the call-up record list, those of the competition (+4) end tomorrow (+0x10, the day from 0x54c5b0).
///
/// Ret 4: `end_duty_for_competition(comp)`: in the call-up record list `[this]`, those of the competition (+4 equal to
/// *comp) end tomorrow (+0x10). No SEH, no box, no random draw; the local is the date that 0x54c5b0 writes.
/// Original: `end_duty_for_competition` (0x774f30, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x774f30 } (thiscall, ret 4, national_teams.cpp): `end_duty_for_competition(comp)`: na lista de registros
/// de convocação [this], os da competição (+4 igual a *comp) terminam amanhã (+0x10). Sem SEH, sem caixa, sem
/// sorteio; a local é a data que a 0x54c5b0 grava.
pub(crate) unsafe extern "thiscall" fn end_duty_for_competition(this: usize, comp: usize) {
    end_call_ups(this, |r| read_usize(r + 4) == read_usize(comp) /* a competição */)
}

/// like 0x774f30, only the records of the competition (+4) and the nation (+0)
/// Original: `end_duty_for_nation_in_competition` (0x774f90, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x774f90 } (thiscall, ret 8, national_teams.cpp): `end_duty_for_nation_in_competition(nacao, comp)`: como a
/// 0x774f30, só os registros da competição (+4) e da nação (+0 igual a *nacao).
pub(crate) unsafe extern "thiscall" fn end_duty_for_nation_in_competition(this: usize, nation: usize, comp: usize) {
    end_call_ups(this, |r| read_usize(r + 4) == read_usize(comp) && read_usize(r) == read_usize(nation) /* a nação */)
}

/// Regions touched by end_call_ups: the list cursor and +0x10 of each record.
///
/// [pt] As regiões do fim das convocações: o cursor da lista e o +0x10 de cada registro.
pub(crate) unsafe fn end_call_ups_regions(this: usize) -> Vec<(usize, usize)> {
    let l = read_usize(this);
    if l == 0 {
        return Vec::new();
    }
    let mut reg = vec![(l + 0xa, 4)];
    reg.extend(list_items_capped(l).into_iter().filter(|&r| r != 0).map(|r| (r + 0x10, 2)));
    reg
}



/// Number of per-club call-up records of 0x7774b0: club count - 2 x nation count.
///
/// [pt] O número de registros de convocação por clube da 0x7774b0: [0xae235c] − 2 × [0xae2350].
pub(crate) unsafe fn club_call_up_record_count() -> i32 {
    (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2))
}

/// In each club record (+0x20, 6 bytes), drops the expired entries (0x456770) at the start, moves the rest down
/// (0x777590) and frees the emptied buffer.
///
/// `prune_club_call_up_records()`: each club record (+0x20, 6 bytes: +0 vector of 9-byte entries, +4 flag, +5 count,
/// signed byte) counts the entries already expired (0x456770: their date before `[0xae2c90]`; the routine assumes they
/// are at the start), moves the others down over them (0x777590) and decreases the count; zeroed with a vector, frees
/// the vector (0x9452ca) and zeroes +4 and +0. No SEH, no box, no random draw; the local (the expired count) is zeroed
/// before being read.
/// Original: `prune_club_call_up_records` (0x7774b0, national_teams.cpp, thiscall).
///
/// [pt] { 0x7774b0 } (thiscall, national_teams.cpp): `prune_club_call_up_records()`: cada registro de clube (+0x20,
/// 6 bytes: +0 vetor de entradas de 9 bytes, +4 marca, +5 quantas, byte com sinal) conta as entradas já vencidas
/// (0x456770: a data delas antes de [0xae2c90]; a rotina supõe que ficam no começo), desce as outras por cima
/// (0x777590) e diminui a contagem; zerada com vetor, solta o vetor (0x9452ca) e zera +4 e +0. Sem SEH, sem caixa,
/// sem sorteio; a local (o vencido) é zerada antes de ler.
pub(crate) unsafe extern "thiscall" fn prune_club_call_up_records(this: usize) {
    let n = club_call_up_record_count();
    let mut rec = read_usize(this + 0x20);
    if n <= 0 {
        return;
    }
    let before: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CALL_UP_RECORD_ASSIGN);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let count = |rec: usize| *((rec + 5) as *const i8);
    for _ in 0..n {
        let mut vencidas: i8 = 0;
        if count(rec) > 0 {
            let mut k: i8 = 0;
            loop {
                let e = read_usize(rec).wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                if before(e, *(G_CURRENT_DATE as *const u32), *(G_CURRENT_DATE_LEAP as *const u32)) != 0 {
                    vencidas = vencidas.wrapping_add(1); // one won
                }
                k = k.wrapping_add(1);
                if k >= count(rec) {
                    break;
                }
            }
        }
        if vencidas < count(rec) {
            let mut k = vencidas;
            loop {
                crate::comparison::branch("convocações descer as que ficam");
                let v = read_usize(rec);
                let de = v.wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                let para = v.wrapping_add(9usize.wrapping_mul((k as i32 - vencidas as i32) as usize));
                copy(para, de);
                k = k.wrapping_add(1);
                if k >= count(rec) {
                    break;
                }
            }
        }
        let rest = count(rec).wrapping_sub(vencidas);
        *((rec + 5) as *mut i8) = rest;
        if rest == 0 && read_usize(rec) != 0 {
            crate::comparison::branch("convocações soltar o vetor");
            free(read_usize(rec));
            *((rec + 4) as *mut u8) = 0;
            core::ptr::write_unaligned(rec as *mut usize, 0);
        }
        rec += 6;
    }
}

/// Would record rec of 0x7774b0 end with no entries but a vector (so the original frees it)? Counts entries before
/// today.
///
/// [pt] O registro `rec` da 0x7774b0 terminaria sem entradas e com vetor (a original solta o vetor)? Escrito à parte da
/// cópia, para o comparador.
pub(crate) unsafe fn would_free_call_up_record(rec: usize) -> bool {
    let before: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let n = *((rec + 5) as *const i8);
    let v = read_usize(rec);
    let vencidas = (0..n.max(0) as usize).filter(|&k| before(v + 9 * k, *(G_CURRENT_DATE as *const u32), *(G_CURRENT_DATE_LEAP as *const u32)) != 0).count();
    v != 0 && n.wrapping_sub(vencidas as i8) == 0
}



/// National team for call-up type: main team (0x53b3d0) for 0 and 1, U21 (0x53b4f0) otherwise.
///
/// [pt] O time da nação para o tipo de convocação: 0 e 1 a seleção principal (0x53b3d0), os outros o sub-21 (0x53b4f0).
pub(crate) unsafe fn national_team_for_type(nation: usize, kind: u8) -> usize {
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    if kind == 0 || kind == 1 { main(nation) } else { sub21(nation) }
}

/// Removes person p from the team's tactics selection (0x896c60 with 1 then 0), as the originals do before clearing the
/// slot.
///
/// [pt] Tira a pessoa `p` da tática do time (0x896c60 de [0xde1d08], com 1 e com 0 no 3º argumento), como as originais
/// fazem antes de esvaziar a vaga.
pub(crate) unsafe fn remove_from_tactics(team: usize, p: usize) {
    let mark: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    mark(*(G_TACTICS as *const usize), team, p, 1, 1);
    mark(*(G_TACTICS as *const usize), team, p, 0, 1);
}

/// Person leaves the national team: +0x24 = 0, call-up mark +0x28 = 0, +0x5e keeps only club bits (& 3).
///
/// [pt] A pessoa sai da seleção: sem time (+0x24), sem a marca +0x28 e só com os bits de clube (& 3) em +0x5e.
pub(crate) unsafe fn clear_national_call_up(p: usize) {
    core::ptr::write_unaligned((p + staff::NATIONAL_JOB) as *mut usize, 0);
    *((p + staff::JOB_FOR_NATION) as *mut u8) = 0; // the call-up mark
    *((p + staff::PLAYING_SQUAD) as *mut u8) &= 3;
}

/// the first of the 50 free squad slots of the team (+0xd7; type 0/1 the senior team, else the under-21) takes the
/// person, who gets the team (+0x24), today's date (+0x29), mark 0xb (+0x28) and squad bit 4 or 8 (0x540980)
/// Original: `add_player_to_squad` (0x776250, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x776250 } (thiscall, ret 0xc, national_teams.cpp): `add_player_to_squad(nacao, tipo, p)`: a primeira vaga
/// livre das 50 do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe `p`; sem vaga, nada. A pessoa ganha o
/// time (+0x24), a data de hoje em +0x29 (0x417c10 de [0xae2c90]), a marca 0xb em +0x28 e o elenco (0x540980) 4
/// (tipo 0 e 2) ou 8. Sem SEH, sem caixa, sem sorteio; a local é a cópia da data que a 0x417c10 devolve.
pub(crate) unsafe extern "thiscall" fn add_player_to_squad(_this: usize, nation: usize, kind: u32, p: usize) {
    let kind = kind as u8;
    let team = national_team_for_type(nation, kind);
    let squad: u32 = if kind == 0 || kind == 2 { 4 } else { 8 };
    let slot = match (0..50usize).find(|&c| read_usize(team + club::SQUAD + 4 * c) == 0) {
        Some(c) => team + club::SQUAD + 4 * c,
        None => return,
    };
    core::ptr::write_unaligned(slot as *mut usize, p); // a vaga
    core::ptr::write_unaligned((p + staff::NATIONAL_JOB) as *mut usize, team);
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut data = [0u8; 8];
    copy(p + staff::DATE_JOINED_NATION, data.as_mut_ptr(), G_CURRENT_DATE);
    *((p + staff::JOB_FOR_NATION) as *mut u8) = 0xb;
    let mark_squad: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    mark_squad(p, squad);
}

/// Regions of add/remove from squad: the team's 50 slots (+0xd7) and the person's +0x24..+0x5e.
///
/// [pt] As regiões de pôr e tirar do elenco: as 50 vagas do time e a pessoa (+0x24 a +0x5e).
pub(crate) unsafe fn squad_change_regions(team: usize, p: usize) -> [(usize, usize); 2] {
    [(team + club::SQUAD, 200), (p + staff::NATIONAL_JOB, 0x3b)]
}


/// Removes the person from the team tactics (0x896c60) and, if found in a slot, empties it and clears +0x24, +0x28 and
/// the national bits of +0x5e.
///
/// Ret 0xc: `remove_player_from_squad(nation, type, p)`: takes `p` out of the team's tactic (0x896c60, twice) and, if
/// he is in one of the 50 slots (+0xd7), empties the slot and clears the person (+0x24, +0x28, +0x5e & 3). No SEH, no
/// random draw, no locals; a box only with a null team or person (0x896c60).
/// Original: `remove_player_from_squad` (0x7762f0, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7762f0 } (thiscall, ret 0xc, national_teams.cpp): `remove_player_from_squad(nacao, tipo, p)`: tira `p` da
/// tática do time (0x896c60, duas vezes) e, se ele está numa das 50 vagas (+0xd7), esvazia a vaga e limpa a
/// pessoa (+0x24, +0x28, +0x5e & 3). Sem SEH, sem sorteio, sem locais; caixa só com time ou pessoa nulos (0x896c60).
pub(crate) unsafe extern "thiscall" fn national_teams_remove_player_from_squad(_this: usize, nation: usize, kind: u32, p: usize) {
    let team = national_team_for_type(nation, kind as u8);
    remove_from_tactics(team, p);
    if let Some(c) = (0..50usize).find(|&c| read_usize(team + club::SQUAD + 4 * c) == p) {
        core::ptr::write_unaligned((team + club::SQUAD + 4 * c) as *mut usize, 0);
        clear_national_call_up(p);
    }
}


/// Nation's record in the national-teams manager: [this+4] + 0x6e x id (+0 of the nation).
///
/// [pt] O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id (+0 da nação).
pub(crate) unsafe fn nation_record_by_id(this: usize, nation: usize) -> usize {
    read_usize(this + 4).wrapping_add((*(nation as *const i32)).wrapping_mul(0x6e) as usize)
}

/// Once the nation record date (+0x62) has passed, removes from the under-21 squad everyone older than 21 (+0x18) and
/// clears record +0x6b.
///
/// Ret 4: `release_overage_u21_players(nation)`: after the date of the nation record (+0x62/+0x66; 0x456740 over
/// `[0xae2c90]`), each of the 50 slots of the under-21 side (0x53b4f0) with a person aged (+0x18, signed byte) above 21
/// leaves the tactic (0x896c60, twice), is cleared (+0x24, +0x28, +0x5e & 3) and the slot is emptied; finally zeroes
/// byte +0x6b of the record. No SEH, no random draw; the local is `this`.
/// Original: `release_overage_u21_players` (0x76d880, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x76d880 } (thiscall, ret 4, national_teams.cpp): `release_overage_u21_players(nacao)`: depois da data do
/// registro da nação (+0x62/+0x66; 0x456740 sobre [0xae2c90]), cada uma das 50 vagas do sub-21 (0x53b4f0) com
/// pessoa de idade (+0x18, byte com sinal) acima de 21 sai da tática (0x896c60, duas vezes), é limpa (+0x24, +0x28,
/// +0x5e & 3) e a vaga esvazia; por fim zera o byte +0x6b do registro. Sem SEH, sem sorteio; a local é o `this`.
pub(crate) unsafe extern "thiscall" fn release_overage_u21_players(this: usize, nation: usize) {
    let rec = nation_record_by_id(this, nation);
    let after: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    if after(G_CURRENT_DATE, read_usize(rec + 0x62) as u32, read_usize(rec + 0x66) as u32) == 0 {
        return;
    }
    crate::comparison::branch("sub-21 dispensar acima da idade");
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    let team = sub21(nation);
    for c in 0..50usize {
        let slot = team + club::SQUAD + 4 * c;
        let p = read_usize(slot);
        if p != 0 && *((p + staff::AGE) as *const i8) > 0x15 { // over 21 years old
            remove_from_tactics(team, p);
            clear_national_call_up(p);
            core::ptr::write_unaligned(slot as *mut usize, 0);
        }
    }
    *((nation_record_by_id(this, nation) + 0x6b) as *mut u8) = 0;
}




/// Notifies the manager manager ([0xb67a44], if any) about the nation's main team (set_season_expectations with 1).
///
/// [pt] Avisa o gerenciador dos técnicos ([0xb67a44], se existe) da seleção principal da nação (0x6a3770 com 1).
pub(crate) unsafe fn notify_manager_manager(nation: usize) {
    let g = *(G_MANAGER_MANAGER as *const usize);
    if g != 0 {
        let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
        let warn: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
        warn(g, main(nation), 1);
    }
}

/// The nation's current competition (record +4) becomes −1 and, if there is a next one (`[this+8][id]`), that one takes
/// its place, notifying the managers (0x6a3770).
///
/// Ret 4: `clear_current_competition(nation)`: the current competition of the nation record (`[this+4]` + 0x6e × id,
/// +4) becomes −1; if there is a next one (`[this+8][id]` ≠ −1), it becomes the current one, the next becomes −1 and
/// the manager manager is notified (0x6a3770). No SEH, no box, no locals.
/// Original: `clear_current_competition` (0x775000, national_teams.cpp, thiscall, ret 4).
///
/// [pt] { 0x775000 } (thiscall, ret 4, national_teams.cpp): `clear_current_competition(nacao)`: a competição atual do
/// registro da nação ([this+4] + 0x6e × id, +4) vira −1; havendo a seguinte ([this+8][id] ≠ −1), ela passa a ser a
/// atual, a seguinte vira −1 e o gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
pub(crate) unsafe extern "thiscall" fn clear_current_competition(this: usize, nation: usize) {
    let id = *(nation as *const i32);
    let current = read_usize(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4;
    core::ptr::write_unaligned(current as *mut i32, -1);
    let next = read_usize(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    let v = *(next as *const i32);
    if v != -1 {
        crate::comparison::branch("seleção a competição seguinte");
        core::ptr::write_unaligned(current as *mut i32, v); // a seguinte vira a atual
        *(next as *mut i32) = -1;
        notify_manager_manager(nation); // the notice of the next competition
    }
}


/// Has competition v finished for 0x775070? Without end-of-stage: comp +0x59 is null; with it: it equals the European
/// continent stage.
///
/// [pt] A competição `v` (−1: nenhuma) acabou para a 0x775070? Sem `fim_de_fase`, quando +0x59 da competição
/// ([0xae23d0] + 107 × v) é nulo; com ele, quando é a fase [0xae23a4] + 0xc6 × [0x9cfa10].
pub(crate) unsafe fn is_comp_finished(v: i32, end_of_stage: bool) -> bool {
    let c = read_usize((*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add(v.wrapping_mul(107) as usize) + 0x59);
    if end_of_stage {
        c == (*(G_CONTINENT_TABLE as *const usize)).wrapping_add((*(KEY_CONTINENT_EUROPE as *const i32)).wrapping_mul(0xc6) as usize)
    } else {
        c == 0
    }
}

/// in the nations of the continent (+0x71), the current and next competitions that ended (competition +0x59 null, or
/// stage [0x9cfa10]) become −1; empty current and a next one: the next moves up, with the notice (0x6a3770)
/// Original: `release_competitions_of_continent` (0x775070, national_teams.cpp, thiscall, ret 8).
///
/// [pt] { 0x775070 } (thiscall, ret 8, national_teams.cpp): `release_competitions_of_continent(continente, fim_de_fase)`:
/// a cada nação ([0xae23a8], 0x122 bytes) do continente (+0x71), a competição atual (registro +4) e a seguinte
/// ([this+8][i]) que acabaram (is_comp_finished) viram −1; se a atual ficou −1 e há seguinte, a seguinte passa a atual e o
/// gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
pub(crate) unsafe extern "thiscall" fn release_competitions_of_continent(this: usize, continent: usize, end_of_stage: u32) {
    let end = end_of_stage as u8 != 0;
    let mut i = 0i32;
    while i < *(G_NATION_COUNT as *const i32) {
        let nation = (*(G_NATION_TABLE as *const usize)).wrapping_add(0x122 * i as usize);
        if read_usize(nation + nation::CONTINENT) == continent {
            let current = read_usize(this + 4) + 0x6e * i as usize + 4;
            let v = *(current as *const i32);
            if v != -1 && is_comp_finished(v, end) {
                *(current as *mut i32) = -1; // a atual acabou
            }
            let next = read_usize(this + 8) + 4 * i as usize;
            let v = *(next as *const i32);
            if v != -1 && is_comp_finished(v, end) {
                *(next as *mut i32) = -1;
            }
            let v = *((read_usize(this + 8) + 4 * i as usize) as *const i32);
            if v != -1 && *((read_usize(this + 4) + 0x6e * i as usize + 4) as *const i32) == -1 {
                crate::comparison::branch("seleção continente a seguinte vira a atual");
                *((read_usize(this + 4) + 0x6e * i as usize + 4) as *mut i32) = v;
                *((read_usize(this + 8) + 4 * i as usize) as *mut i32) = -1;
                notify_manager_manager(nation); // the continent warning
            }
        }
        i += 1;
    }
}

/// Comparador: o aviso 0x6a3770 sorteia e mexe nos técnicos (não pode rodar duas vezes): só a original; a prova é a
/// carreira e o ramo da cópia no modo Rust.
// simplification: original only (4 calls in the career); comparing would require separating the decision from the warning.

/// Calls up a replacement: in the pool of the level, the first eligible player of the nation who plays a position of
/// the one leaving (0x5b7d20) is called up (0x776250, leaving another squad first through 0x7762f0) with the news
/// (0x76fd10); nobody: clears the type's slot count (+0x45).
///
/// Ret 0x10: `call_up_replacement(nation, type, level, leaving)`: in the section of the level (0x76d810) in the type's
/// pool (`[this+0xc]`, under-21 `[this+0x10]`; start +8/+0x1d of the record plus the slots of the previous levels,
/// count +0xd/+0x22), the first one who is not `leaving`, has a player sheet (+0x61), likes the club (0x547d30), is fit
/// (0x62c400), is not a flagged regen (0x7a7eb0) outside the senior side, is from the nation (team +0x24, nationalities
/// +0x1a/+0x1e, +0x22, bit 2 of the 79-byte record), is not suspended (0x55d270) and plays one of the positions of
/// `leaving` (0x5b7d20). Without a team he goes straight in; called up (+0x28 = 0xb) for another type (2 with 0x5409b0,
/// otherwise bit 3 of +0x5e) only after the type's date (+0x3a; 0x456770) and, for type 0, if the club allows
/// (0x600a50): he leaves there (0x7762f0). He joins (0x776250), the type's slot (+0x45) goes down 1 and the news goes
/// out (0x76fd10). Nobody: zeroes +0x45. No SEH, no random draw; the local for the type of the other squad is written
/// before being read (only the byte counts).
/// Original: `call_up_replacement` (0x775f90, national_teams.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x775f90 } (thiscall, ret 0x10, national_teams.cpp): `call_up_replacement(nacao, tipo, nivel, saindo)`: no
/// trecho do nível (0x76d810) na reserva do tipo ([this+0xc], sub-21 [this+0x10]; começo +8/+0x1d do registro mais
/// as vagas dos níveis anteriores, quantos +0xd/+0x22), o primeiro que não é `saindo`, tem ficha (+0x61), gosta do
/// clube (0x547d30), está bem (0x62c400), não é regen marcado (0x7a7eb0) fora da principal, é da nação (time +0x24,
/// nacionalidades +0x1a/+0x1e, +0x22, bit 2 do registro de 79 bytes), não está suspenso (0x55d270) e joga numa das
/// posições de `saindo` (0x5b7d20). Sem time vai direto; convocado (+0x28 = 0xb) para outro tipo (2 com 0x5409b0,
/// senão o bit 3 de +0x5e) só depois da data do tipo (+0x3a; 0x456770) e, sendo o 0, se o clube deixa (0x600a50):
/// sai de lá (0x7762f0). Entra (0x776250), a vaga do tipo (+0x45) desce 1 e sai a notícia (0x76fd10). Ninguém: zera
/// +0x45. Sem SEH, sem sorteio; a local do tipo do outro elenco é gravada antes de ler (só o byte vale).
pub(crate) unsafe extern "thiscall" fn call_up_replacement(this: usize, nation: usize, kind: u32, level: u32, saindo: usize) {
    let positions: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let state: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let regen: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let before: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let remove: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_REMOVE_PLAYER_FROM_SQUAD);
    let por: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_PLAYER_TO_SQUAD);
    let news: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NATIONAL_TEAMS_POST_PLAYER_CALLED_UP_NEWS);
    let mask = positions(read_usize(saindo + staff::PLAYER)) as u8;
    let t = kind as u8 as i8;
    let level = level as u8 as i8;
    let rec = read_usize(this + 4).wrapping_add((*(nation as *const i32)).wrapping_mul(0x6e) as usize);
    let sub = (t == 2) as usize;
    let o = 0x15 * sub;
    let mut start = read_usize(rec + o + 8) as i32;
    for j in 0..level.max(0) as usize {
        start = start.wrapping_add(read_u8(rec + o + 0xe + 4 * j) as i32);
    }
    let end = start.wrapping_add(read_u8(rec.wrapping_add(o).wrapping_add((4 * level as i32) as usize) + 0xd) as i32);
    let reserve = read_usize(this + 4 * sub + 0xc);
    let slot = rec.wrapping_add((20 * t as i32) as usize) + 0x45;
    let mut k = start;
    while k < end {
        let q = read_usize(reserve.wrapping_add((4 * k) as usize));
        k += 1;
        if q == saindo || read_usize(q + staff::PLAYER) == 0 || not_likes(q, main(read_usize(q + staff::NATION))) != 0 {
            continue;
        }
        if state(*(G_INJURY as *const usize), q) != 0 {
            continue;
        }
        if *(G_PLAYER_REGEN as *const usize) != 0 && regen(q) != 0 && t != 0 {
            continue;
        }
        let team = read_usize(q + staff::NATIONAL_JOB);
        if team != 0 && read_usize(team + club::NATION) != nation {
            continue;
        }
        if read_usize(q + staff::NATION) != nation {
            let record = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((*(q as *const i32)).wrapping_mul(0x4f) as usize);
            if team != 0 || read_u8(q + staff::INT_APPS) != 0 || read_u8(record + 0xc) & 2 != 0 || read_usize(q + staff::SECOND_NATION) != nation {
                continue;
            }
        }
        let comp = (*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((*(KEY_CLUB_COMP_FIFA_WORLD_CUP as *const i32)).wrapping_mul(107) as usize);
        if suspenso(*(G_DISCIPLINE as *const usize), q, comp, 1, 0) != 0 {
            continue;
        }
        if positions(read_usize(q + staff::PLAYER)) as u8 & mask & 0x7f == 0 { // the positions of those leaving
            continue;
        }
        let team = read_usize(q + staff::NATIONAL_JOB);
        if team != 0 {
            if read_u8(q + staff::JOB_FOR_NATION) != 0xb {
                continue;
            }
            let other: i8 = if second(team) != 0 { 2 } else { ((read_u8(q + staff::PLAYING_SQUAD) >> 3) & 1) as i8 };
            if other == t {
                continue;
            }
            let data = rec.wrapping_add((20 * other as i32) as usize) + 0x3a;
            if before(data, *(G_CURRENT_DATE as *const u32), *(G_CURRENT_DATE_LEAP as *const u32)) == 0 {
                continue;
            }
            if other == 0 && human(team, 0, 1, 0, 0) == 0 {
                continue;
            }
            crate::comparison::branch("substituto sai do outro elenco");
            remove(this, nation, other as u8 as u32, q);
        }
        crate::comparison::branch("substituto convocado");
        por(this, nation, kind, q);
        *(slot as *mut u8) = read_u8(slot).wrapping_sub(1); // one slot fewer
        news(this, q, nation, kind);
        return;
    }
    *(slot as *mut u8) = 0;
}

/// Comparador: a entrada no elenco sai com a notícia (0x76fd10) e a saída do outro mexe na tática: só a original
/// (a 0x776250 e a 0x7762f0 que ela chama passam pelos comparadores delas).
// simplification: original only; the proof is the career (140 calls) and the copy’s branch in Rust mode.




/// Rating of the person in reserve slot q: word +9 of his player record (+0x61).
///
/// [pt] A nota da pessoa da vaga `q` (ponteiro para a pessoa): o curto +9 da ficha (+0x61).
pub(crate) unsafe fn slot_person_rating(q: usize) -> i16 {
    read_i16(read_usize(read_usize(q) + 0x61) + 9)
}

/// Start index of level `nivel` in the reserve (main or U21 fields): record +8 plus the slot counts of previous levels.
///
/// [pt] O começo do trecho do nível `nivel` na reserva (`o` 0: a principal, 0x15: o sub-21): o +8 do registro mais as
/// vagas (+0xe) dos níveis anteriores.
pub(crate) unsafe fn reserve_level_start(rec: usize, o: usize, level: i8) -> i32 {
    let mut v = read_usize(rec + o + 8) as i32;
    for j in 0..level.max(0) as usize {
        v = v.wrapping_add(read_u8(rec + o + 0xe + 4 * j) as i32);
    }
    v
}

/// Adds person p to the reserve of record rec (main or U21) in his level's block, keeping it sorted by rating (insert,
/// evict last, qsort when full).
///
/// [pt] Põe a pessoa `p` na reserva do registro `rec` (`sub` 0: a principal em [this+0xc]; 1: o sub-21 em [this+0x10],
/// campos 0x15 adiante), no trecho do nível dela (0x76d810). Com vaga (+0xd < +0xe), no fim do trecho, e a menor nota
/// (+0xf) desce se for o caso; cheio, só com nota acima da menor: entra na posição pela nota (decrescente, depois das
/// iguais), empurrando o último para fora, e a menor nota vira a do último. Trecho cheio: ordena (qsort 0x945333 com
/// 0x778ed0).
pub(crate) unsafe fn add_to_reserves(this: usize, p: usize, rec: usize, sub: u32) {
    let level_of: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let o = 0x15 * sub as usize;
    let level = level_of(p, rec, sub) as i8;
    let c = rec.wrapping_add(o).wrapping_add((4 * level as i32) as usize);
    let reserve = || read_usize(this + 0xc + 4 * sub as usize);
    let slot = |k: i32| reserve().wrapping_add((4 * k) as usize);
    let rating = read_i16(read_usize(p + staff::PLAYER) + 9);
    if read_u8(c + 0xd) < read_u8(c + 0xe) {
        let ini = reserve_level_start(rec, o, level);
        core::ptr::write_unaligned(slot(ini.wrapping_add(read_u8(c + 0xd) as i32)) as *mut usize, p);
        *((c + 0xd) as *mut u8) = read_u8(c + 0xd).wrapping_add(1);
        if rating < read_i16(c + 0xf) {
            core::ptr::write_unaligned((c + 0xf) as *mut i16, rating); // a menor nota
        }
    } else if rating > read_i16(c + 0xf) {
        crate::comparison::branch("reservas cheias entra pela nota");
        let ini = reserve_level_start(rec, o, level);
        let last = (read_u8(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(2);
        let mut pos = ini;
        let mut j = last;
        while j >= ini {
            if rating < slot_person_rating(slot(j)) {
                pos = j + 1; // after the ones with a higher rating
                break;
            }
            j -= 1;
        }
        let mut j = last;
        while j >= pos {
            core::ptr::write_unaligned(slot(j + 1) as *mut usize, read_usize(slot(j)));
            j -= 1;
        }
        core::ptr::write_unaligned(slot(pos) as *mut usize, p);
        let ult = (read_u8(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(1);
        core::ptr::write_unaligned((c + 0xf) as *mut i16, slot_person_rating(slot(ult)));
    }
    let slots = read_u8(c + 0xe);
    if read_u8(c + 0xd) == slots {
        qsort(slot(reserve_level_start(rec, o, level)), slots as u32, 4, 0x778ed0);
    }
}

/// Builds the selection pools: clears the 4 levels of each nation record and puts every eligible person in the nation
/// pool (and the under-21 one, up to 21 years and under 5 caps), also by the second nationality; full section: enters
/// by rating, and sorts (qsort with 0x778ed0).
///
/// `build_selection_pools()`: zeroes the counts (+0xd, +0x22) and puts 10000 in the lowest ratings (+0xf, +0x24) of the
/// 4 levels of each nation record (`[this+4]`, 0x6e bytes); then each person (`[0xae23bc]`, 0x6e bytes) with a nation
/// (+0x1a), a player sheet (+0x61), over 16 years old (+0x18), without a national side or already called up (+0x24,
/// +0x28 = 0xb), in a player position (0x53b620 of +0x3d) and with a club (+0x39) joins the nation's pool
/// (add_to_reserves) and, up to 21 years with fewer than 5 caps (+0x22), the under-21 one; and again by the second
/// nationality (+0x1e: with no caps for the first, a different one and not `[0x9cf434]`). No SEH, no box, no random
/// draw; the locals are written before being read (of the rating stored in 32 bits only the short counts).
/// Original: `build_selection_pools` (0x76abf0, national_teams.cpp, thiscall).
///
/// [pt] { 0x76abf0 } (thiscall, national_teams.cpp): `build_selection_pools()`: zera as contagens (+0xd, +0x22) e põe
/// 10000 nas menores notas (+0xf, +0x24) dos 4 níveis de cada registro de nação ([this+4], 0x6e bytes); depois, cada
/// pessoa ([0xae23bc], 0x6e bytes) com nação (+0x1a), ficha (+0x61), mais de 16 anos (+0x18), sem seleção ou já
/// convocada (+0x24, +0x28 = 0xb), na posição de jogador (0x53b620 de +0x3d) e com clube (+0x39) entra na reserva
/// da nação (add_to_reserves) e, até 21 anos com menos de 5 jogos (+0x22), na do sub-21; e de novo pela segunda
/// nacionalidade (+0x1e: sem jogos pela primeira, outra e não a [0x9cf434]). Sem SEH, sem caixa, sem sorteio; as
/// locais são gravadas antes de ler (da nota guardada em 32 bits só vale o curto).
pub(crate) unsafe extern "thiscall" fn build_selection_pools(this: usize) {
    let mut i = 0i32;
    while i < *(G_NATION_COUNT as *const i32) {
        for k in 0..4usize {
            let r = read_usize(this + 4) + 0x6e * i as usize + 4 * k;
            *((r + 0xd) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0xf) as *mut i16, 10000);
            *((r + 0x22) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0x24) as *mut i16, 10000);
        }
        i += 1;
    }
    let player: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let mut p = *(G_STAFF_TABLE as *const usize);
    let mut n = 0i32;
    while n < *(G_STAFF_COUNT as *const i32) {
        let can = read_usize(p + staff::NATION) != 0
            && read_usize(p + staff::PLAYER) != 0
            && *((p + staff::AGE) as *const i8) > 0x10 // over 16 years old
            && (read_usize(p + staff::NATIONAL_JOB) == 0 || read_u8(p + staff::JOB_FOR_NATION) == 0xb)
            && player(read_u8(p + staff::JOB_FOR_CLUB) as u32) != 0
            && read_usize(p + staff::CLUB_JOB) != 0;
        if can {
            for k in 0..2 {
                let id = if k == 0 {
                    *(read_usize(p + staff::NATION) as *const i32)
                } else {
                    let n2 = read_usize(p + staff::SECOND_NATION);
                    if n2 == 0 || read_u8(p + staff::INT_APPS) != 0 || n2 == read_usize(p + staff::NATION) || *(n2 as *const i32) == *(KEY_NATION_PORTUGAL as *const i32) {
                        continue;
                    }
                    crate::comparison::branch("reservas segunda nacionalidade");
                    *(n2 as *const i32)
                };
                let rec = read_usize(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize);
                add_to_reserves(this, p, rec, 0);
                if *((p + staff::AGE) as *const i8) > 0x15 || read_u8(p + staff::INT_APPS) >= 5 { // o sub-21
                    continue;
                }
                add_to_reserves(this, p, rec, 1);
            }
        }
        n += 1;
        p += 0x6e;
    }
}


/// the pool qsort order: rating (+9 of the player data) descending
/// Original: `compare_by_rating` (0x778ed0, national_teams.cpp, cdecl).
///
/// [pt] { 0x778ed0 } (cdecl, national_teams.cpp): a ordem das reservas para o qsort da 0x76abf0: pela nota (+9 da ficha
/// +0x61) decrescente (nota de b menos a de a).
pub(crate) unsafe extern "C" fn compare_by_rating(a: usize, b: usize) -> i32 {
    (slot_person_rating(b) as i32) - slot_person_rating(a) as i32 // decrescente
}


/// the order by club (+0x39): no club last, clubs by id, within a club squad bit 4 before 8 (+0x5e)
/// Original: `compare_by_club` (0x778ef0, national_teams.cpp, cdecl).
///
/// [pt] { 0x778ef0 } (cdecl, national_teams.cpp): a ordem das pessoas pelo clube (+0x39) para os qsort das seleções
/// (0x76c120, 0x7706c0, 0x7771c0): sem clube depois; clubes diferentes pelo id; no mesmo, o de elenco 4 (+0x5e)
/// antes do de elenco 8.
pub(crate) unsafe extern "C" fn compare_by_club(a: usize, b: usize) -> i32 {
    let (x, y) = (read_usize(a), read_usize(b));
    let cx = if x != 0 { read_usize(x + staff::CLUB_JOB) } else { 0 };
    let cy = if y != 0 { read_usize(y + staff::CLUB_JOB) } else { 0 };
    if cx == 0 {
        return (cy != 0) as i32;
    }
    if cy == 0 {
        return -1;
    }
    if cx != cy {
        return (*(cx as *const i32)).wrapping_sub(*(cy as *const i32)); // the club id
    }
    let (mx, my) = (read_u8(x + staff::PLAYING_SQUAD), read_u8(y + staff::PLAYING_SQUAD));
    if mx & 4 != 0 && my & 8 != 0 {
        return -1;
    }
    if mx & 8 != 0 && my & 4 != 0 {
        return 1;
    }
    0
}


/// Takes the player out of his level pool in the nation and out of the squad, clearing his national team fields.
///
/// Ret 4: takes `player` out of the national side. With a nation (+0x1a) and a player sheet (+0x61): in the nation's
/// 110-byte record in `[this+4]`, the level (0x76d810 with the record and 0); if the level's rating (word +0xf + level
/// × 4) is lower than the player sheet's (+9), looks for the player in the level's section of list `[this+0xc]` (start
/// = dword +8 + bytes +0xe of the previous levels, size in byte +0xd + level × 4): found, the last of the section goes
/// into his slot and the size decreases. Then, if called up (+0x24): his slot among the 50 (+0xd7) is zeroed, +0x24 is
/// zeroed and byte +0x5e keeps only bits 0..1.
/// Original: `NationalTeams::remove_player` (0x775ba0, national_teams.cpp, thiscall).
///
/// [pt] { 0x775ba0 } (thiscall, ret 4, national_teams.cpp): tira o `jogador` da seleção. Com nação (+0x1a)
/// e ficha (+0x61): no registro de 110 bytes da nação em [this+4], o nível (0x76d810 com o registro e
/// 0); se a nota do nível (palavra +0xf + nível x 4) for menor que a da ficha (+9), procura o jogador
/// na faixa do nível da lista [this+0xc] (início = dword +8 + bytes +0xe dos níveis anteriores,
/// tamanho no byte +0xd + nível x 4): achou, o último da faixa vai para a vaga dele e o tamanho
/// diminui. Depois, convocado (+0x24): a vaga dele entre as 50 (+0xd7) zera, +0x24 zera e o byte +0x5e
/// fica só com os bits 0..1.
pub(crate) unsafe extern "thiscall" fn national_teams_remove_player(this: usize, player: usize) {
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nation = u(player + staff::NATION);
    if nation == 0 || u(player + staff::PLAYER) == 0 {
        return;
    }
    let level: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let reg = u(this + 4).wrapping_add((*(nation as *const u32) as usize).wrapping_mul(110));
    let n = level(player, reg as *const i16, 0) as i8 as isize;
    let field = |o: isize| (reg as isize).wrapping_add(n * 4 + o) as usize;
    if core::ptr::read_unaligned(field(0xf) as *const i16) < core::ptr::read_unaligned((u(player + staff::PLAYER) + 9) as *const i16) {
        let mut start = u(reg + 8) as i32;
        for k in 0..n.max(0) as usize {
            start = start.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32);
        }
        let size = field(0xd) as *mut u8;
        let end = start.wrapping_add(*size as i32);
        let list = |i: i32| (u(this + 0xc) as isize).wrapping_add(i as isize * 4) as *mut usize;
        let mut i = start;
        while i < end {
            if core::ptr::read_unaligned(list(i)) == player {
                *size = (*size).wrapping_sub(1);
                core::ptr::write_unaligned(list(i), core::ptr::read_unaligned(list(end - 1)));
                break;
            }
            i += 1;
        }
    }
    let national_team = u(player + staff::NATIONAL_JOB);
    if national_team == 0 {
        return;
    }
    if let Some(k) = (0..50).find(|&k| u(national_team + club::SQUAD + 4 * k) == player) {
        core::ptr::write_unaligned((national_team + club::SQUAD + 4 * k) as *mut usize, 0);
    }
    core::ptr::write_unaligned((player + staff::NATIONAL_JOB) as *mut usize, 0);
    *((player + staff::PLAYING_SQUAD) as *mut u8) &= 3;
}






/// Addresses of the 50 call-up slots (+0xd7) of the nation's main team and U21/B team.
///
/// [pt] As 50 vagas de convocação (dwords desalinhados em +0xd7) das duas seleções da nação: a principal
/// (0x53b3d0) e a B (0x53b4f0), nessa ordem.
pub(crate) unsafe fn national_squad_slots(nation: usize) -> [usize; 2] {
    let main: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
    [main(nation).wrapping_add(0xd7), b(nation).wrapping_add(0xd7)]
}

/// clears repeated squad entries from the senior and B national sides of a nation
/// Original: `national_teams_remove_duplicate_squad_entries` (0x76c060, national_teams.cpp, stdcall).
///
/// [pt] { 0x76c060 } (stdcall, ret 4, national_teams.cpp): apaga as convocações repetidas das duas
/// seleções da nação: em cada uma, todo id não nulo zera as vagas seguintes com o mesmo id.
pub(crate) unsafe extern "stdcall" fn national_teams_remove_duplicate_squad_entries(nation: usize) {
    for vagas in national_squad_slots(nation) {
        let slot = |i: usize| (vagas + 4 * i) as *mut u32;
        for i in 0..50 {
            let v = core::ptr::read_unaligned(slot(i));
            if v == 0 {
                continue;
            }
            for j in i + 1..50 {
                if core::ptr::read_unaligned(slot(j)) == v {
                    core::ptr::write_unaligned(slot(j), 0);
                }
            }
        }
    }
}




/// Returns the player's level 1..3 for the national side by reputation and position ratings.
///
/// Stdcall, ret 0xc: the level 1..3 of player `j` for the national side (0 with reputation `*rep` below 100, or 50 with
/// `base`, or a goalkeeper rated 15): 1 for sweeper or central defender, 2 for defensive midfielder, midfielder or
/// attacking midfielder (rating 15 in +0x10..+0x14 of the player sheet), otherwise 3. Returns only `al`.
/// Original: `national_team_level` (0x76d810, national_teams.cpp, stdcall).
///
/// [pt] { 0x76d810 } (stdcall, ret 0xc, national_teams.cpp): o nível 1..3 do jogador `j` para a seleção
/// (0 com reputação `*rep` abaixo de 100, ou 50 com `base`, ou goleiro de nota 15): 1 para líbero ou
/// zagueiro, 2 para volante, meia ou meia-atacante (nota 15 em +0x10..+0x14 da ficha), senão 3.
/// Devolve só `al`.
pub(crate) unsafe extern "stdcall" fn national_team_level(j: usize, rep: *const i16, base: u8) -> u8 {
    if *rep < if base != 0 { 0x32 } else { 0x64 } {
        return 0;
    }
    let sheet = core::ptr::read_unaligned((j + staff::PLAYER) as *const usize);
    let n = |o: usize| *((sheet + o) as *const i8) >= 0xf;
    if n(0xf) {
        0
    } else if n(0x10) || n(0x11) {
        1
    } else if n(0x12) || n(0x13) || n(0x14) {
        2
    } else {
        3
    }
}


/// the national teams day: the table of clubs with a match is freed; on 1 January, 15 April and 1 August the pools are
/// rebuilt; for the matches two weeks ahead, the clubs playing around the date are marked, the national teams call up
/// and the call-ups per club are recorded (outside the finals); with the table, the matches of clubs with called-up
/// players are rescheduled; the day's items leave, the FIFA ranking moves, every 70 days the records are pruned and the
/// qualified nations news goes out
/// Original: `NationalTeams::daily_update` (0x76b100, national_teams.cpp, thiscall).
///
/// [pt] { 0x76b100 } (thiscall, ret 4, national_teams.cpp, quadro SEH só das datas locais, sem destrutor):
/// `NationalTeams::daily_update(hoje)`, pelo dia do jogo ([0xb74318], 0x5ccb07). A data daqui a duas semanas
/// (0x54c770); a tabela dos clubes com jogo (+0x24) sai (0x9452ca) e zera. Em 1º de janeiro, 15 de abril e 1º de agosto
/// (0x549f70 no ano de hoje), as reservas das seleções se refazem (0x76abf0). Nas 3 fatias do dia daqui a duas
/// semanas, as listas dos grupos 0 e 1 (0x5abf10): com jogo em alguma e sem a tabela, os clubes com jogo em volta da
/// data são marcados (mark_clubs_three_days); os jogos das duas listas (select_squads_for_fixtures). Com a tabela, os jogos dos clubes
/// com convocados se remarcam (0x7777e0). Da lista em +0 saem (0x68d1f0) os itens do dia de hoje (+0x10). Com o ranking
/// (+0x1c), a atualização dele (0x58d460); no dia múltiplo de 70, a poda dos registros de convocação (0x7774b0); por fim
/// as notícias dos classificados (0x7779e0).
pub(crate) unsafe extern "thiscall" fn national_teams_daily_update(this: usize, today: usize) {
    let weeks: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let redo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_BUILD_SELECTION_POOLS);
    let list: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_GET_FIXTURE_LIST);
    let remarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_RESCHEDULE_CLASHING_CLUB_FIXTURES);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let ranking: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIFA_RANKINGS_UPDATE_RANKINGS);
    let podar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_PRUNE_CLUB_CALL_UP_RECORDS);
    let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_POST_QUALIFIED_NATIONS_NEWS);
    // 0x54c770: push 2; push saida; mov ecx, hoje -> thiscall ret 8; free: add esp, 4 -> cdecl; 0x549f70: 4 pushes;
    // lea ecx, saida -> thiscall ret 0x10; 0x76abf0, 0x7774b0, 0x7779e0: mov ecx, edi sem pushes -> thiscall;
    // 0x5abf10: 4 pushes; mov ecx, [0xae2a58] -> thiscall ret 0x10; 0x7777e0: push &data; mov ecx, edi -> thiscall
    // ret 4; 0x68d1f0: push item; mov ecx, [edi] -> thiscall ret 4; 0x58d460: push hoje; mov ecx, [edi + 0x1c] ->
    // thiscall ret 4
    let mut dd = [0u8; 8];
    let d = dd.as_mut_ptr() as usize;
    weeks(today, d, 2);
    if read_usize(this + 0x24) != 0 {
        free(read_usize(this + 0x24));
        write_usize(this + 0x24, 0);
    }
    // simplification: the original pushes the year (word) and the week with register leftovers on top; 0x549f70 reads the words
    let year = read_u16(today + 2) as u32;
    let mut t = [0u8; 8];
    let pt = t.as_mut_ptr() as usize;
    if is_today(dma(pt, 1, 0, year, u32::MAX), today) // cqc12: 1º de janeiro
        || is_today(dma(pt, 0xf, 3, year, u32::MAX), today)
        || is_today(dma(pt, 1, 7, year, u32::MAX), today)
    {
        crate::comparison::branch("cqc_12 refaz as reservas");
        redo(this);
    }
    let fixman = read_usize(G_FIXTURE_MANAGER);
    for fatia in 0u32..3 {
        // the original’s dwords: the year (with the next 2 bytes of the date) and the day (with the year on top); simplification: the
        // slice goes without the stack leftover in the upper bytes (0x5abf10 does and 0xff)
        let l0 = list(fixman, read_usize(d + 2) as u32, read_usize(d) as u32, fatia, 0);
        let l1 = list(fixman, read_usize(d + 2) as u32, read_usize(d) as u32, fatia, 1);
        if ((l0 != 0 && read_i16(l0) != 0) || (l1 != 0 && read_i16(l1) != 0)) && read_usize(this + 0x24) == 0 {
            mark_clubs_three_days(this, d); // cqc12: os clubes com jogo em volta da data
        }
        if l0 != 0 {
            select_squads_for_fixtures(this, l0);
        }
        if l1 != 0 {
            select_squads_for_fixtures(this, l1);
        }
    }
    if read_usize(this + 0x24) != 0 {
        remarcar(this, d);
    }
    if read_usize(this) != 0 {
        let day = read_i16(today);
        let l = read_usize(this);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut p = next(read_usize(this));
        while p != 0 {
            if read_i16(p + 0x10) == day {
                remove(read_usize(this), p);
            }
            p = next(read_usize(this));
        }
    }
    if read_usize(this + 0x1c) != 0 {
        ranking(read_usize(this + 0x1c), today);
    }
    if read_i16(today) as i32 % 0x46 == 0 {
        podar(this);
    }
    classificados(this);
}


/// Returns the age limits (23/23 in youth competitions, 24/22, 26/22).
///
/// Stdcall, ret 0x10: the age limits (`*minimum`, `*maximum`) of a national side in competition `comp`: 23/23 in the
/// Olympic and youth competitions (`[comp+0x14]` with an id among the 9 constants 0x9cf78c…0x9cf964); 24/22 with `type`
/// 1 or above 2; otherwise 26/22. Null pointers: the original routine (error box).
/// Original: `national_team_ages` (0x76d430, national_teams.cpp, stdcall).
///
/// [pt] { 0x76d430 } (stdcall, ret 0x10, national_teams.cpp): as idades limite (`*minima`, `*maxima`) de
/// uma seleção na competição `comp`: 23/23 nas competições olímpicas e de base ([comp+0x14] com id
/// numa das 9 constantes 0x9cf78c…0x9cf964); 24/22 com `tipo` 1 ou acima de 2; senão 26/22.
/// Ponteiros nulos: a original (caixa de erro).
pub(crate) unsafe extern "stdcall" fn national_team_ages(comp: usize, kind: i8, minima: *mut u8, maxima: *mut u8) {
    if minima.is_null() || maxima.is_null() {
        return original_fn::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(IDX_NATIONAL_TEAM_AGES)(comp, kind, minima, maxima);
    }
    if comp != 0 {
        let c = *((comp + 0x14) as *const usize);
        if c != 0 {
            let id = *(c as *const i32);
            if [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
                .iter()
                .any(|&a| *(a as *const i32) == id)
            {
                *minima = 0x17;
                *maxima = 0x17;
                return;
            }
        }
    }
    if kind != 0 && kind != 2 {
        *minima = 0x18;
        *maxima = 0x16;
    } else {
        *minima = 0x1a;
        *maxima = 0x16;
    }
}


/// Sorts the national squad by club and adds, on the match date, how many players each club gives up (the record that
/// stops club fixtures near that date).
///
/// Ret 0xc; SEH frame only for the state, the dates have no destructor: `NationalTeams::record_club_call_ups(team, b,
/// fixture)`: sorts the team's squad (+0xd7, 50 slots) by club (qsort with 0x778ef0). With the fixture: its date
/// (+0x2a, +0x28; a day earlier for the second team 0x5409b0 or with `b`); counts the consecutive called-up players of
/// each club (+0x39), skipping the empty slots, the persons out of range (`[0xae2364]`) and those of another squad
/// (+0x5e bit 3 without `b`, bit 2 with it), and when the club changes adds the previous club's count to its record
/// (add_club_callups). The last club of the loop gets no record (as in the original).
/// Original: `NationalTeams::record_club_call_ups` (0x7771c0, national_teams.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7771c0 } (thiscall, ret 0xc, national_teams.cpp; quadro SEH só do estado, as datas não têm destrutor):
/// `NationalTeams::record_club_call_ups(time, b, partida)`: ordena o elenco do time (+0xd7, 50 vagas) pelo clube (qsort
/// com 0x778ef0). Com a partida: a data dela (+0x2a, +0x28; um dia antes no segundo time 0x5409b0 ou com `b`); conta os
/// convocados seguidos de cada clube (+0x39), pulando as vagas vazias, as pessoas fora da faixa ([0xae2364]) e as de
/// outro elenco (+0x5e bit 3 sem `b`, bit 2 com), e ao mudar de clube soma a conta do anterior no registro dele
/// (add_club_callups). O último clube da volta fica sem registro (como na original).
pub(crate) unsafe extern "thiscall" fn record_club_call_ups(this: usize, team: usize, b: u32, p: usize) {
    let default: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let mut d = [0u8; 8];
    default(d.as_mut_ptr());
    qsort(team + club::SQUAD, 0x32, 4, 0x778ef0);
    if p == 0 {
        return;
    }
    let new: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copy: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let minus: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    // simplification: the original pushes the shorts +0x2a and +0x28 with the high part of the register; 0x549ef0 only reads the shorts
    let r = new(t1.as_mut_ptr(), *((p + 0x2a) as *const u16) as u32, *((p + 0x28) as *const u16) as u32);
    copy(d.as_mut_ptr(), t2.as_mut_ptr(), r);
    if second(team) != 0 || b != 0 {
        let r = minus(d.as_mut_ptr(), t2.as_mut_ptr(), 1);
        copy(d.as_mut_ptr(), t1.as_mut_ptr(), r);
    }
    let mut previous = 0usize;
    let mut count: u8 = 0;
    for i in 0..0x32usize {
        let s = read_usize(team + club::SQUAD + 4 * i);
        if s == 0 || read_usize(s) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        let marks = *((s + staff::PLAYING_SQUAD) as *const u8);
        if (b == 0 && marks & 8 != 0) || (b != 0 && marks & 4 != 0) {
            continue;
        }
        let c = read_usize(s + staff::CLUB_JOB);
        if c == previous && previous != 0 {
            count = count.wrapping_add(1);
            continue;
        }
        if previous != 0 && !add_club_callups(this, previous, count, d.as_mut_ptr()) {
            return;
        }
        previous = c;
        count = 1;
    }
    if previous != 0 && club_callup_record(this, previous).is_some() {
        crate::comparison::branch("cpw_13 último clube sem registro"); // cpw13: o último clube fica sem registro
    }
}






