use super::*;

/// The items (node +0 pointers) of an LLIST, without touching its cursor.
///
/// [pt] Os itens da lista (o ponteiro em +0 de cada nó, da cabeça +2 seguindo o +6), sem mexer no cursor.
pub(crate) unsafe fn llist_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut no = read_usize(list + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

/// Shared body of { `clear_awol` } and { `clear_missed_training` }: removes the person's first entry from a list when the person has the
/// mark bit.
///
/// With the person's mark `bit` ([this+0xc] + id), the mark is flipped (the original's `or bit` never yields zero) and,
/// with the list ([this + `lista`]), the cursor goes back to the head and, among the first `conta` (+0, signed word)
/// items, the first one with the person's id at +0 is removed (`manager_manager_remove`, which frees the node). A list that ends before
/// the count is reached stops the loop.
///
/// [pt] O corpo comum de { 0x4196a0 } e { 0x4197c0 }: com a marca `bit` da pessoa ([this+0xc] + id), a marca troca (o
/// `or bit` da original nunca dá zero) e, com a lista ([this + `lista`]), o cursor volta à cabeça e, entre os primeiros
/// `conta` (+0, palavra com sinal) itens, o primeiro com o id da pessoa em +0 sai (0x68d1f0, que libera o nó). Uma lista
/// que acaba antes da conta encerra.
pub(crate) unsafe fn toggle_mark_and_unlist(this: usize, person: usize, bit: u8, list: usize, foul: bool) {
    if person == 0 {
        return;
    }
    let marks = read_usize(this + 0xc);
    if marks == 0 {
        return;
    }
    let p = marks.wrapping_add(read_usize(person));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return;
    }
    if foul {
        crate::comparison::branch("cqd_11 absence removed");
    }
    *(p as *mut u8) = m ^ bit;
    let l = read_usize(this + list);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    // 0x68d1b0: mov ecx, [edi + lista] -> thiscall; 0x68d1f0: push item; mov ecx, [edi + lista]; ret 4 -> thiscall
    let mut i: i32 = 0;
    if read_i16(l) as i32 <= 0 {
        return;
    }
    loop {
        let item = exe::llist_get_next(read_usize(this + list));
        if item == 0 {
            return;
        }
        if read_usize(item) == read_usize(person) {
            if foul {
                crate::comparison::branch("cqd_11 absence record removed");
            }
            exe::llist_remove(read_usize(this + list), item);
            return;
        }
        i += 1;
        if i >= read_i16(read_usize(this + list)) as i32 {
            return;
        }
    }
}

/// `AWOL::clear_awol(person)`: ends the absence. Called by the end of the absence without a contract (`is_player_awol`), by
/// retirements (`process_retirements`) and by transfers (`free_transfer_staff`, `handle_player_accepted_terms`, `exe::transfer_manager_move_player_to_club`).
///
/// With the person and the flag table ([this+0xc]) not null and bit 1 of the person's flag set: flips the bit and
/// removes the person's record from the absence list ([this]).
/// Original: `AWOL::clear_awol` (0x4196a0, awol.cpp, thiscall).
///
/// [pt] { 0x4196a0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_awol(pessoa)`, pelo fim da ausência sem contrato (0x418f60),
/// pelas aposentadorias (0x7a59e0) e pelas transferências (0x8c1af0, 0x8c2180, 0x8c9720). Pessoa e tabela de marcas
/// ([this+0xc]) não nulas e o bit 1 da marca: troca o bit e tira o registro da pessoa da lista das ausências ([this]).
pub(crate) unsafe extern "thiscall" fn clear_awol(this: usize, person: usize) {
    toggle_mark_and_unlist(this, person, 1, 0, false);
}

/// `AWOL::clear_missed_training(person)`: ends the missed training. Called by the missed-training check for someone
/// left without a contract (`has_missed_training`, which the daily player update consults) and by retirements (`process_retirements`).
///
/// With the person and the flag table not null and bit 2 of the flag set: flips the bit and removes the person's record
/// from the missed training list ([this+4]).
/// Original: `AWOL::clear_missed_training` (0x4197c0, awol.cpp, thiscall).
///
/// [pt] { 0x4197c0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_missed_training(pessoa)`, pela falta ao treino de quem ficou
/// sem contrato (0x418ff0, que os jogadores do dia consultam) e pelas aposentadorias (0x7a59e0). Pessoa e tabela de
/// marcas não nulas e o bit 2 da marca: troca o bit e tira o registro da pessoa da lista das faltas ([this+4]).
pub(crate) unsafe extern "thiscall" fn clear_missed_training(this: usize, person: usize) {
    toggle_mark_and_unlist(this, person, 2, 4, true);
}

/// Plans toggle_mark_and_unlist: the mark value after and the list items that remain.
///
/// [pt] O plano das duas: a marca depois e os itens que ficam na lista (o primeiro com o id da pessoa entre os `conta`
/// primeiros sai). Sem nada a fazer: o estado de antes.
pub(crate) unsafe fn plan_toggle_mark_and_unlist(this: usize, person: usize, bit: u8, list: usize) -> (Option<(usize, u8)>, usize, Vec<usize>) {
    let l = read_usize(this + list);
    let items = llist_items(l);
    if person == 0 || read_usize(this + 0xc) == 0 {
        return (None, l, items);
    }
    let p = read_usize(this + 0xc).wrapping_add(read_usize(person));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return (Some((p, m)), l, items);
    }
    if l == 0 {
        return (Some((p, m ^ bit)), l, items);
    }
    let count = (read_i16(l) as i32).max(0) as usize;
    let mut stays = items.clone();
    if let Some(k) = items.iter().take(count).position(|&x| read_usize(x) == read_usize(person)) {
        stays.remove(k);
    }
    (Some((p, m ^ bit)), l, stays)
}

/// Comparer of the two: runs only the original and checks the mark and remaining list against the plan.
///
/// [pt] Comparador das duas: a original tira e libera o nó da lista (não pode rodar duas vezes): só ela roda, com o plano
/// montado antes (a marca e os itens que ficam, sem ler o item tirado depois).
pub(crate) unsafe fn compare_toggle_mark_and_unlist(this: usize, person: usize, bit: u8, list: usize, o: unsafe extern "thiscall" fn(usize, usize), name: &str) {
    let (marca, l, fica) = plan_toggle_mark_and_unlist(this, person, bit, list);
    o(this, person);
    let after = llist_items(l);
    let mark_ok = marca.map_or(true, |(p, m)| *(p as *const u8) == m);
    SCOREBOARD.record(mark_ok && after == fica, || format!("{}(pessoa {:#x}) marca plano {:x?} agora {:x?}; lista plano {:x?} agora {:x?}",
        name, person, marca, marca.map(|(p, _)| *(p as *const u8)), fica, after));
}

/// The rehearsal of the news item. The random draws come from the original routine's tape (n and the answer) and,
/// instead of delivering the news, the copy records the destination (1 the player's club, 2 the club's nation, 0 none).
/// The tape keeps the next position and whether any n did not match.
///
/// [pt] O ensaio da notícia: os sorteios vêm da fita da original (n e resposta) e, em vez de entregar, a cópia anota o
/// destino (1 o clube do jogador, 2 a nação do clube, 0 nenhum). A fita fica com o próximo e se algum n não bateu.
pub(crate) static mut AWOL_NEWS_TAPE: Option<(Vec<(i32, i32)>, usize, bool)> = None;
pub(crate) static mut AWOL_NEWS_DESTINATION: (u32, usize) = (0, 0);

/// Random draw: replays from AWOL_NEWS_TAPE when set (marking failure on mismatch), else the game's random.
pub(crate) unsafe fn taped_random(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(AWOL_NEWS_TAPE);
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
            exe::utils_random(n)
        }
    }
}

/// `awol_post_missed_training_news(player, club)`: posts the 0xbc9 missed training news. Called by the missed training
/// (`miss_training`). SEH frame for the local news, no destructor.
///
/// With a player who has a player sheet (+0x61) and a club with a manager (+0xcf): the news 0xbc9 (`construct_news_item`) with the
/// player (id and the +0x33 keys of +4, +8, +0xc), the club id, draw(3), whether the club is human-run (`exe::human_manager_is_human_club` with
/// 0, 1) and who reports it: the manager, if the club has a manager and is not human-run (`exe::human_manager_is_human_club` with 1, 1; id and
/// keys); otherwise, with draw(2), one of the 5 coaches (+0x19f, draw(5) until one is found or a thousand tries), and
/// without the draw the assistant (+0xd3); with nobody (or id −1), −1 in all four.
///
/// Human-run club (`news_club_has_human_manager`): sent to the player's club (+0x39, `send_to_club_manager`); otherwise, if the player has a nation
/// (+0x53) and his reputation (+0x61, +0xb) is above 7250, to the nation's inboxes (`send_to_nation_managers`).
/// Original: `awol_post_missed_training_news` (0x41a630, awol.cpp, thiscall).
///
/// [pt] { 0x41a630 } (thiscall, ret 8, awol.cpp, SEH da notícia local, sem destrutor): `awol_post_missed_training_news(jogador,
/// clube)`, pela falta ao treino (0x41a360). Jogador com ficha (+0x61) e clube com técnico (+0xcf): a notícia 0xbc9
/// (0x77bbe0) com o jogador (id e as chaves +0x33 de +4, +8, +0xc), o id do clube, sorteio(3), o clube de humano que
/// decide (0x600a50 com 0, 1) e quem relata: o técnico, se o clube tem técnico e não é de humano (0x600a50 com 1, 1;
/// id e chaves); senão, com sorteio(2), um dos 5 treinadores (+0x19f, sorteio(5) até achar ou mil voltas), sem
/// sorteio o auxiliar (+0xd3); sem ninguém (ou id −1), −1 nos quatro. Clube de humano (0x7875e0): ao clube do jogador
/// (+0x39, 0x7861e0); senão, com nação (+0x53) e reputação do jogador (+0x61, +0xb) acima de 7250, às caixas da nação
/// (0x7863f0).
pub(crate) unsafe extern "thiscall" fn awol_post_missed_training_news(this: usize, player: usize, club: usize) {
    let _ = this;
    if player == 0 || read_usize(player + staff::PLAYER) == 0 || club == 0 || read_usize(club + club::MANAGER) == 0 { // cqd11: no player or manager
        return;
    }
    // 0x77bbe0: push 0; push 0xbc9; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall; 0x600a50: 5
    // pushes; add esp, 0x14 -> cdecl (club, 0, 1, 0, 0) and (club, 1, 1, 0, 0)
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbc9, 0);
    exe::newsitem_set_slot(nt, 0, read_usize(player) as u32);
    exe::newsitem_set_slot(nt, 1, read_usize(read_usize(player + staff::FIRST_NAME) + 0x33) as u32);
    exe::newsitem_set_slot(nt, 2, read_usize(read_usize(player + staff::SECOND_NAME) + 0x33) as u32);
    exe::newsitem_set_slot(nt, 3, read_usize(read_usize(player + staff::COMMON_NAME) + 0x33) as u32);
    exe::newsitem_set_slot(nt, 4, read_usize(club) as u32);
    crate::comparison::branch("cqd_11 absence news");
    let r = taped_random(3); // cqd11: the news random draw
    exe::newsitem_set_slot(nt, 5, r as u32);
    exe::newsitem_set_slot(nt, 6, exe::human_manager_is_human_club(club, 0, 1, 0, 0));
    let manager = read_usize(club + club::MANAGER);
    if manager != 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 { // cqd11: the AI manager reports
        crate::comparison::branch("cqd_11 technical reporter");
        let t = read_usize(club + club::MANAGER);
        exe::newsitem_set_slot(nt, 7, read_usize(t) as u32);
        exe::newsitem_set_slot(nt, 8, read_usize(read_usize(t + staff::FIRST_NAME) + 0x33) as u32);
        exe::newsitem_set_slot(nt, 9, read_usize(read_usize(t + staff::SECOND_NAME) + 0x33) as u32);
        exe::newsitem_set_slot(nt, 0xa, read_usize(read_usize(t + staff::COMMON_NAME) + 0x33) as u32);
    } else {
        let who = if taped_random(2) != 0 {
            let mut k;
            let mut laps = 0;
            loop {
                k = taped_random(5);
                laps += 1;
                if laps >= 1000 || read_usize(club.wrapping_add(k as usize * 4) + 0x19f) != 0 {
                    break;
                }
            }
            read_usize(club.wrapping_add(k as usize * 4) + 0x19f)
        } else {
            read_usize(club + club::ASSISTANT_MANAGER)
        };
        let id = if who != 0 { read_usize(who) as i32 } else { -1 };
        if id == -1 {
            for k in 7..=0xa {
                exe::newsitem_set_slot(nt, k, u32::MAX);
            }
        } else {
            let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            exe::newsitem_set_slot(nt, 7, id as u32);
            exe::newsitem_set_slot(nt, 8, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
            exe::newsitem_set_slot(nt, 9, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
            exe::newsitem_set_slot(nt, 0xa, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
        }
    }
    let news = read_usize(G_NEWS);
    let of_human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    // 0x7875e0: mov ecx, [0xb7431c]; push club (ret 4); 0x7861e0: push club +0x39; push news; mov ecx, [0xb7431c];
    // 0x7863f0: push nation; push news; mov ecx, [0xb7431c] -> thiscall in all three
    let dry_run = (*core::ptr::addr_of!(AWOL_NEWS_TAPE)).is_some();
    if of_human(news, club) != 0 {
        if dry_run {
            AWOL_NEWS_DESTINATION = (1, read_usize(player + staff::CLUB_JOB));
        } else {
            exe::news_send_to_club_manager(news, nt, read_usize(player + staff::CLUB_JOB));
        }
        return;
    }
    let nation = read_usize(club + club::NATION);
    if nation == 0 || read_i16(read_usize(player + staff::PLAYER) + 0xb) <= 0x1c52 {
        return;
    }
    if dry_run {
        AWOL_NEWS_DESTINATION = (2, nation);
    } else {
        exe::news_send_to_nation_managers(news, nt, nation);
    }
}

/// Decrements a signed attribute byte (above 1) depending on whether random(attribute) is zero.
///
/// [pt] O byte com sinal `a` (atributo de 1 a 20) cai um com o sorteio dele: zero (`zero`) ou não zero, se for maior que 1.
pub(crate) unsafe fn maybe_drop_attribute(a: usize, zero: bool) {
    let v = read_u8(a) as i8;
    if v > 1 && (friendly_random(v as i32) == 0) == zero {
        *(a as *mut i8) = v - 1;
    }
}

/// LLIST::get_next for a list.
pub(crate) unsafe fn awol_llist_next(list: usize) -> usize {
    exe::llist_get_next(list)
}

/// Resets an LLIST cursor (+0xa) to the first node (+2).
///
/// [pt] O cursor da lista volta ao primeiro nó (+0xa = +2).
pub(crate) unsafe fn awol_llist_rewind(list: usize) {
    write_u32(list + 0xa, read_usize(list + 2) as u32);
}

/// True if today is after date d (day signed +0, year unsigned +2).
///
/// [pt] A data `d` (dia com sinal em +0, ano sem sinal em +2) já passou: hoje ([0xae2c90]) é depois dela.
pub(crate) unsafe fn date_has_passed(d: usize) -> bool {
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let a = read_i16(d + 2) as u16;
    ano > a || (ano == a && dia > read_i16(d))
}

/// True if date d plus `semanas` weeks (Date add_weeks) has passed.
///
/// [pt] Uma semana depois da data `d` (0x54c770), já passou?
pub(crate) unsafe fn weeks_after_have_passed(d: usize, weeks: u32) -> bool {
    let mut t = [0u8; 8];
    date_has_passed(exe::date_add_weeks(d, &mut t, weeks))
}

/// Ends an AWOL case: flips bit 2 of the person's mark byte and removes the record from the AWOL list (this + 4).
///
/// [pt] O fim do caso de falta ao treino `r` (pessoa em +0): com o byte da pessoa na tabela de marcas ([this+0xc]) não zero,
/// troca o bit 2 dele (o `or 2` da original nunca dá zero) e tira o registro da lista das faltas ([this+4], 0x68d1f0).
pub(crate) unsafe fn close_awol_case(this: usize, r: usize) {
    let m = read_usize(this + 0xc).wrapping_add(read_i32(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m ^= 2;
    crate::comparison::branch("cpw_7 absence removed from the list");
    exe::llist_remove(read_usize(this + 4), r); // cpw7: removes the case from the list
}

/// Calls AWOL miss_training for a person.
pub(crate) unsafe fn awol_miss_training(this: usize, person: usize) {
    exe::awol_miss_training(this, person);
}

/// `AWOL::daily_update()`: resolves the day's AWOL cases, missed trainings and late arrivals. Called every day by the
/// game loop (0x5cccd0). Three lists, each with its cursor at +0xa.
///
/// (1) Absences ([this]): the first one whose return date (+4) has passed ends (`end_absence`).
///
/// (2) Missed trainings ([this+4]): the first one a week past its date (+4) is resolved. Repeat case (byte +0xc), with
/// two draws coming out zero, by the person's professionalism (+0x5b) and temperament (+0x5d): both drop by a draw and
/// the person misses training again (`miss_training`). New case: the teammates at the club (50 slots at +0xd7) under the
/// offender's contract may ask to leave (`set_flags` with 2) or lose professionalism, morale (+0x44 of the player) and
/// temperament; and the offender, when draw(professionalism) is below draw(15) + 1, loses professionalism and morale,
/// misses training again and falls out with the club (`exe::media_create_player_media_story` with 8).
///
/// (3) Late arrivals ([this+8]): the first one a week past (+8) that wins the draws loses professionalism and
/// temperament or morale.
///
/// Each resolved case leaves its list (`manager_manager_remove`) and bit 2 of the flag flips. A list that ends before the count ends
/// the day.
/// Original: `AWOL::daily_update` (0x4190d0, awol.cpp, thiscall).
///
/// [pt] { 0x4190d0 } (thiscall, awol.cpp): `AWOL::daily_update()`, todo dia pelo laço do jogo (0x5cccd0). Três listas com o
/// cursor em +0xa: (1) ausências ([this]): a primeira cuja volta (data em +4) já passou acaba (0x419710); (2) faltas ao
/// treino ([this+4]): a primeira com uma semana passada da data (+4) é resolvida: caso repetido (byte +0xc) com dois
/// sorteios zero pelo profissionalismo (+0x5b) e pelo temperamento (+0x5d) da pessoa, os dois caem pelo sorteio e ela
/// falta de novo (0x41a360); caso novo: os colegas do clube (50 vagas em +0xd7) com o contrato do faltoso podem pedir a
/// saída (0x867560 com 2) ou perder profissionalismo, moral (+0x44 do jogador) e temperamento; e o faltoso, por
/// sorteio(profissionalismo) abaixo de sorteio(15) + 1, perde profissionalismo e moral, falta de novo e briga com o
/// clube (0x72c1f0 com 8); (3) atrasos ([this+8]): o primeiro com uma semana passada (+8) que vença os sorteios perde
/// profissionalismo e temperamento ou moral. Cada caso resolvido sai da lista (0x68d1f0) e a marca do bit 2 troca. Uma
/// lista que acaba antes da conta encerra o dia.
pub(crate) unsafe extern "thiscall" fn awol_daily_update(this: usize) {
    // (1) the absences
    let l = read_usize(this);
    if l != 0 {
        awol_llist_rewind(l);
        if read_i16(l) > 0 {
            let mut i = 0;
            loop {
                let r = awol_llist_next(read_usize(this));
                if r == 0 {
                    return;
                }
                if date_has_passed(r + 4) {
                    exe::awol_end_absence(this, r);
                    break;
                }
                i += 1;
                if i >= read_i16(read_usize(this)) as i32 {
                    break;
                }
            }
        }
    }
    // (2) missed training sessions
    let l = read_usize(this + 4);
    if l != 0 {
        awol_llist_rewind(l);
        if read_i16(l) > 0 {
            let mut i = 0;
            loop {
                let r = awol_llist_next(read_usize(this + 4));
                if r == 0 {
                    return;
                }
                if weeks_after_have_passed(r + 4, 1) { // cpw7: one week later
                    crate::comparison::branch("cpw_7 absence expired");
                    resolve_awol_case(this, r);
                    break;
                }
                i += 1;
                if i >= read_i16(read_usize(this + 4)) as i32 {
                    break;
                }
            }
        }
    }
    // (3) the delays
    let l = read_usize(this + 8);
    if l == 0 {
        return;
    }
    awol_llist_rewind(l);
    if read_i16(l) <= 0 {
        return;
    }
    let mut i = 0;
    loop {
        let r = awol_llist_next(read_usize(this + 8));
        if r == 0 {
            return;
        }
        if weeks_after_have_passed(r + 8, 1) {
            let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE));
            let resolveu = if read_u8(r + 0x10) != 0 {
                if read_usize(person + staff::PLAYER) == 0 {
                    true
                } else if friendly_random(read_u8(person + staff::PROFESSIONALISM) as i8 as i32) == 0 && friendly_random(read_u8(person + staff::TEMPERAMENT) as i8 as i32) == 0 {
                    maybe_drop_attribute(person + staff::PROFESSIONALISM, true);
                    maybe_drop_attribute(person + staff::TEMPERAMENT, true);
                    true
                } else {
                    false
                }
            } else {
                let a = friendly_random(read_u8(person + staff::PROFESSIONALISM) as i8 as i32);
                let b = friendly_random(5);
                if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
                    maybe_drop_attribute(person + staff::PROFESSIONALISM, true);
                    let j = read_usize(person + staff::PLAYER);
                    if j != 0 {
                        maybe_drop_attribute(j + player_sheet::WORK_RATE, true);
                    }
                    true
                } else {
                    false
                }
            };
            if resolveu {
                exe::llist_remove(read_usize(this + 8), r);
                return;
            }
        }
        i += 1;
        if i >= read_i16(read_usize(this + 8)) as i32 {
            return;
        }
    }
}

/// Resolves an AWOL case one week on: a repeat case may drop professionalism and temperament; a new one involves
/// teammates' reactions; then closes it.
///
/// [pt] A falta ao treino `r` com uma semana passada (o meio da lista (2) de `awol_daily_update`).
pub(crate) unsafe fn resolve_awol_case(this: usize, r: usize) {
    let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE));
    if read_u8(r + 0xc) != 0 {
        // o caso repetido
        if read_usize(person + staff::PLAYER) != 0 {
            crate::comparison::branch("cpw_7 repeated case");
            if friendly_random(read_u8(person + staff::PROFESSIONALISM) as i8 as i32) == 0 && friendly_random(read_u8(person + staff::TEMPERAMENT) as i8 as i32) == 0 { // cpw7: the random draws of the repeated case
                maybe_drop_attribute(person + staff::PROFESSIONALISM, true);
                maybe_drop_attribute(person + staff::TEMPERAMENT, true);
                close_awol_case(this, r);
                awol_miss_training(this, person);
                return;
            }
        }
        return close_awol_case(this, r);
    }
    // the new case: the club’s teammates
    crate::comparison::branch("cpw_7 new case");
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
    let club = if k != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + contract::CLUB).wrapping_mul(club::SIZE)) } else { 0 };
    if club != 0 {
        for v in 0..0x32usize {
            let s = read_usize(club + club::SQUAD + v * 4);
            if s == 0 || s == person || read_usize(s + staff::PLAYER) == 0 {
                continue;
            }
            let ct = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club);
            if ct == 0 {
                continue;
            }
            if exe::staff_contract_get_flags(ct) & 2 == 0 && read_u8(s + staff::PROFESSIONALISM) as i8 > 0xf && read_u8(s + staff::AMBITION) as i8 > 0xa {
                let a = friendly_random(read_u8(s + staff::PROFESSIONALISM) as i8 as i32);
                let b = friendly_random(0x1e);
                let f = read_u8(ct + contract::SQUAD_STATUS) & 0xf0;
                if a > b && (f == 0x10 || f == 0x20) && read_u8(s + staff::AGE) as i8 > 0x17 && read_i16(read_usize(s + staff::PLAYER) + 0xb) > 0x1482 {
                    exe::staff_contract_set_flags(ct, 2);
                }
            }
            if read_u8(s + staff::AGE) as i8 >= 0x15 || read_i16(read_usize(s + staff::PLAYER) + 0xb) <= 0x1482 {
                continue;
            }
            maybe_drop_attribute(s + staff::PROFESSIONALISM, true);
            let j = read_usize(s + staff::PLAYER);
            if read_u8(j + player_sheet::WORK_RATE) as i8 > 1 && friendly_random(read_u8(s + staff::PROFESSIONALISM) as i8 as i32) == 0 {
                *((j + player_sheet::WORK_RATE) as *mut i8) -= 1;
            }
            maybe_drop_attribute(s + staff::TEMPERAMENT, true);
        }
    }
    // o faltoso
    let j = read_usize(person + staff::PLAYER);
    if j != 0 {
        let a = friendly_random(read_u8(person + staff::PROFESSIONALISM) as i8 as i32);
        let b = friendly_random(0xf);
        if (a as u8 as i8) < ((b as u8).wrapping_add(1) as i8) {
            maybe_drop_attribute(person + staff::PROFESSIONALISM, false);
            maybe_drop_attribute(read_usize(person + staff::PLAYER) + 0x44, false);
            close_awol_case(this, r);
            awol_miss_training(this, person);
            if club != 0 && read_usize(person + staff::PLAYER) != 0 {
                exe::media_create_player_media_story(read_usize(G_MEDIA), person, club, 8, 0, 0);
            }
            return;
        }
    }
    close_awol_case(this, r);
}

// simplification: rolls random numbers, removes and frees list nodes (0x68d1f0), gives return news (0x419710) and absence news (0x41a360): only
// the original; the copy takes the branches.

/// `AWOL::has_missed_training(person)`: returns bit 2 of the person's flag; without a contract, clears the miss first.
/// Called by the daily player update (0x5a69b0) and by the screens.
///
/// Null person or beyond the count [0xae2364]: false. With bit 2 of her flag ([this+0xc] + id) set and no contract
/// (`exe::contract_manager_get_contract`, stored in [0xae1080]), clears the miss (`clear_missed_training`) first. Returns bit 2 of the flag.
/// Original: `AWOL::has_missed_training` (0x418ff0, awol.cpp, thiscall, ret 4).
///
/// [pt] { 0x418ff0 } (thiscall, ret 4, awol.cpp): `AWOL::has_missed_training(pessoa)`, pelos jogadores do dia (0x5a69b0) e
/// pelas telas. Pessoa nula ou fora da conta [0xae2364]: falso. Com o bit 2 da marca dela ([this+0xc] + id) e sem
/// contrato (0x4cf5a0, guardado em [0xae1080]), tira a falta (0x4197c0) antes. Devolve o bit 2 da marca.
pub(crate) unsafe extern "thiscall" fn has_missed_training(this: usize, person: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return id as u32 & 0xffffff00;
    }
    if read_u8(read_usize(this + 0xc).wrapping_add(id as usize)) & 2 != 0 {
        let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
        if k == 0 || read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + contract::CLUB).wrapping_mul(club::SIZE)) == 0 {
            exe::awol_clear_missed_training(this, person);
        }
    }
    (read_u8(read_usize(this + 0xc).wrapping_add(read_i32(person) as usize)) & 2) as u32
}

/// `AWOL::has_fitness_incident(person)`: true when the person has a record in the late arrival list. Called by the
/// daily player update (0x5a69b0).
///
/// Null person, beyond the count [0xae2364] or no late arrival list ([this+8]): false; otherwise walks the list (cursor
/// at +0xa) looking for the person's record (id at +0). True in the low byte; the high bytes as the original leaves
/// them in the register.
/// Original: `AWOL::has_fitness_incident` (0x419070, awol.cpp, thiscall, ret 4).
///
/// [pt] { 0x419070 } (thiscall, ret 4, awol.cpp): `AWOL::has_fitness_incident(pessoa)`, pelos jogadores do dia (0x5a69b0).
/// Pessoa nula, fora da conta [0xae2364] ou sem a lista dos atrasos ([this+8]): falso; senão percorre a lista (cursor em
/// +0xa) atrás do registro da pessoa (id em +0). Verdadeiro no byte baixo; os altos como a original deixa no registrador.
pub(crate) unsafe extern "thiscall" fn has_fitness_incident(this: usize, person: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return id as u32 & 0xffffff00;
    }
    let l = read_usize(this + 8);
    if l == 0 {
        return 0;
    }
    awol_llist_rewind(l);
    if read_i16(l) <= 0 {
        return l as u32 & 0xffffff00;
    }
    let mut i = 0;
    loop {
        let r = awol_llist_next(read_usize(this + 8));
        if r == 0 {
            return 0;
        }
        if read_i32(r) == read_i32(person) {
            return (read_i32(person) as u32 & 0xffffff00) | 1;
        }
        i += 1;
        let n = read_i16(read_usize(this + 8)) as i32;
        if i >= n {
            return n as u32 & 0xffffff00;
        }
    }
}

// ---- the originals that awol_daily_update calls and that read leftovers from the private stack (259th batch bench) ----

/// The dry run of the comparers of `miss_training`, `end_absence` and `set_flags`.
///
/// The copy stops before the calls that allocate, send news or cannot run twice, and leaves in `MISS_TRAINING_DECISION`
/// what it would have done.
///
/// [pt] O ensaio dos comparadores de `miss_training`, `end_absence` e `set_flags`: a cópia para antes
/// das chamadas que alocam, dão notícia ou não podem rodar duas vezes e deixa em MISS_TRAINING_DECISION o que faria.
pub(crate) static mut MISS_TRAINING_DRY_RUN: bool = false;
pub(crate) static mut MISS_TRAINING_DECISION: u32 = 0;

/// Club record of contract k (id +4, G_CLUB_TABLE), or 0.
///
/// [pt] O clube do contrato `k` (+4 id, [0xae23b4], 0x245 bytes), ou 0.
pub(crate) unsafe fn awol_club_of_contract(k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + contract::CLUB).wrapping_mul(club::SIZE))
}

/// Byte +0xc (repeated case) of the absence record.
///
/// `miss_training` does not write it: the 13-byte record is built in a local and +0xc keeps whatever the previous call left in
/// the dword E−0x10 of the stack (E: esp on entry). On the original routine's normal path, through
/// game_check_players_missing_training (`game_check_players_missing_training`), the previous call that reaches that depth is `exe::date_days_since` (days since
/// arrival), which stores its `this` there, the game date 0xae2c90: the low byte is 0x90.
///
/// The copy uses that value (a repeated case); the comparer does not compare it (the original's value is genuine
/// residue).
///
/// [pt] O byte +0xc (caso repetido) do registro da falta. A 0x41a360 não o grava: o registro de 13 bytes é montado num local
/// e o +0xc fica com o que a chamada anterior deixou no dword E−0x10 da pilha (E: o esp na entrada). No caminho normal
/// da original, pela game_check_players_missing_training (0x5d6aa0), a chamada anterior que alcança essa altura é a 0x54cdb0 (dias desde a
/// chegada), que guarda ali o seu `this`, a data do jogo 0xae2c90: o byte baixo é 0x90. A cópia usa esse valor (um caso
/// repetido); o comparador não o compara (o da original é resíduo de verdade).
pub(crate) const AWOL_RESIDUE_BYTE: u8 = 0x90;

/// `AWOL::miss_training(person)`: the player misses training (or, if very unhappy, may go AWOL). Called by the day's
/// missed trainings (`game_check_players_missing_training`) and by awol_daily_update. SEH frame only for the local date, no destructor.
///
/// Player (+0x61) within the count, with a contract (`exe::contract_manager_get_contract`, in [0xae1080]) at a club and without flag 2: if the
/// contract has morale (+0x44) at or below −100 and bit 0x2000 (then 0x800) in +0x45, and draw(professionalism + 10)
/// and draw(+0x59 + 10) both come out zero, he goes AWOL (`exe::awol_start_absence` with 0, 0).
///
/// Otherwise: sets bit 2, posts the missed training news (`awol_post_missed_training_news`, with the contract's club), byte +0xa of the 79-byte
/// row ([0xae2d0c]) adds draw(10), the condition (`get_condition`) drops by 25 × (21 − the player's +0x34) + draw + 100
/// (`recover_injury`), and the record (person, today, case) joins the missed training list ([this+4], 0x68d0d0); the case byte,
/// unset in the original, goes 0x90. AI club (`exe::human_manager_is_human_club`) with a manager (+0xcf, +0x69) and draw(his +0x14) not zero: the
/// fine (`exe::fine_fine_player`, period 1 if draw(manager rating `exe::non_player_attribute_rating` with +0x19) beats draw(5), else 2; reason 1).
/// Original: `AWOL::miss_training` (0x41a360, awol.cpp, thiscall, ret 4).
///
/// [pt] { 0x41a360 } (thiscall, ret 4, awol.cpp, SEH só com a data local, sem destrutor): `AWOL::miss_training(pessoa)`,
/// pelas faltas ao treino do dia (0x5d6aa0) e pela awol_daily_update. Jogador (+0x61) dentro da conta, com contrato
/// (0x4cf5a0, em [0xae1080]) num clube e sem a marca 2: contrato com moral (+0x44) até −100 e o bit 0x2000 (depois o
/// 0x800) em +0x45, com sorteio(profissionalismo + 10) e sorteio(+0x59 + 10) zero, some (0x419840 com 0, 0). Senão
/// marca o bit 2, notícia da falta (0x41a630, com o clube do contrato), o byte +0xa da linha de 79 bytes ([0xae2d0c])
/// soma sorteio(10), a condição (0x62c590) cai 25 × (21 − o +0x34 do jogador) + sorteio + 100 (0x62bd10), o registro
/// (pessoa, hoje, caso) entra na lista das faltas ([this+4], 0x68d0d0); clube da IA (0x600a50) com técnico (+0xcf, +0x69)
/// e sorteio(+0x14 dele) não zero: a multa (0x5a5980, prazo 1 se sorteio(nota do técnico 0x544410 com o +0x19) passar
/// de sorteio(5), senão 2; motivo 1).
pub(crate) unsafe extern "thiscall" fn miss_training(this: usize, person: usize) {
    if person == 0 || read_usize(person + staff::PLAYER) == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
    let club = awol_club_of_contract(k);
    if k == 0 || club == 0 {
        return;
    }
    let id = read_i32(person);
    let m = read_usize(this + 0xc).wrapping_add(id as usize) as *mut u8;
    if *m & 2 != 0 {
        return;
    }
    let k2 = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    if k2 != 0 && read_u8(k2 + 0x44) as i8 <= -100 {
        for bit in [0x2000u32, 0x800] {
            if read_usize(k2 + 0x45) as u32 & bit != 0
                && friendly_random(read_u8(person + staff::PROFESSIONALISM) as i8 as i32 + 10) == 0
                && friendly_random(read_u8(person + staff::LOYALTY) as i8 as i32 + 10) == 0
            {
                crate::comparison::branch("cpw_7 vanished instead of missing");
                if MISS_TRAINING_DRY_RUN {
                    MISS_TRAINING_DECISION = 1;
                    return;
                }
                exe::awol_start_absence(this, person, (0) as usize, 0);
                return;
            }
        }
    }
    crate::comparison::branch("cpw_7 marked the absence");
    if MISS_TRAINING_DRY_RUN {
        MISS_TRAINING_DECISION = 2;
        return;
    }
    *m |= 2;
    let k3 = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k3 as u32);
    exe::awol_post_missed_training_news(person, awol_club_of_contract(k3));
    let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(person) as usize).wrapping_mul(79)).wrapping_add(0xa) as *mut u8;
    let r = friendly_random(10);
    *row = (*row).wrapping_add(r as u8);
    let c = exe::injury_get_condition(read_usize(G_INJURY), person, 0);
    // the original’s calculations are 32-bit with high bytes from other values in the register, but only the low word
    // arrives (the condition is short in 0x62bd10)
    let queda = 0x15i16.wrapping_sub(read_u8(read_usize(person + staff::PLAYER) + 0x34) as i8 as i16).wrapping_mul(25);
    let r = friendly_random(queda as i32);
    let new = (c as u16).wrapping_sub((queda as u16).wrapping_add(r as u16).wrapping_add(0x64));
    exe::injury_set_match_condition(read_usize(G_INJURY), person, new as i16, 0);
    let mut reg = [0u8; 13];
    reg[..4].copy_from_slice(&read_i32(person).to_le_bytes());
    reg[4..12].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    reg[12] = AWOL_RESIDUE_BYTE;
    exe::llist_add_tail(read_usize(this + 4), reg.as_ptr() as usize, 0xd);
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    let manager = read_usize(club + club::MANAGER);
    if manager == 0 {
        return;
    }
    let np = read_usize(manager + staff::NON_PLAYER);
    if np == 0 || friendly_random(read_u8(np + non_player::DISCIPLINE) as i8 as i32) == 0 {
        return;
    }
    let a = friendly_random((exe::non_player_attribute_rating(np, (read_u8(np + non_player::MAN_HANDLING) as u32) as i8) as i8) as i32);
    let b = friendly_random(5);
    crate::comparison::branch("cpw_7 fine for absence");
    exe::fine_fine_player(read_usize(G_FINE), person, if a > b { 1 } else { 2 }, 1);
}

/// `AWOL::end_absence(record)`: ends an absence. Called by awol_daily_update.
///
/// With the person's flag (+0 id; [this+0xc] + id) not zero: if she has a contract (`exe::contract_manager_get_contract`, in [0xae1080]) at a club,
/// the return news (`exe::awol_post_news` with the person's club +0x39, 0 and −1); flips bit 1 of the flag (the original's `or 1`
/// never gives zero) and removes the record from the absence list ([this], `manager_manager_remove`). Does not read an unset local.
/// Original: `AWOL::end_absence` (0x419710, awol.cpp, thiscall, ret 4).
///
/// [pt] { 0x419710 } (thiscall, ret 4, awol.cpp): `AWOL::end_absence(registro)`, pela awol_daily_update. Registro com a marca
/// da pessoa (+0 id; [this+0xc] + id) não zero: com contrato (0x4cf5a0, em [0xae1080]) num clube, a notícia da volta
/// (0x419ad0 com o clube +0x39 da pessoa, 0 e −1); troca o bit 1 da marca (o `or 1` da original nunca dá zero) e tira o
/// registro da lista das ausências ([this], 0x68d1f0). Não lê local sem valor.
pub(crate) unsafe extern "thiscall" fn end_absence(this: usize, r: usize) {
    if r == 0 {
        return;
    }
    let m = read_usize(this + 0xc).wrapping_add(read_i32(r) as usize) as *mut u8;
    if *m == 0 {
        return;
    }
    crate::comparison::branch("cpw_7 end of absence");
    let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE));
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
    if k != 0 && awol_club_of_contract(k) != 0 {
        if MISS_TRAINING_DRY_RUN {
            MISS_TRAINING_DECISION = 1;
        } else {
            let p = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE));
            exe::awol_post_news(p, read_usize(p + staff::CLUB_JOB), 0, 0xffffffff);
        }
    }
    let m = read_usize(this + 0xc).wrapping_add(read_i32(r) as usize) as *mut u8;
    *m ^= 1;
    if MISS_TRAINING_DRY_RUN {
        return;
    }
    exe::llist_remove(read_usize(this), r);
}

/// `AWOL::report_poor_fitness(player, club)`: records a fitness incident and handles the reaction. Called by the daily
/// person update (0x5ce2d0: the day's big drop in condition). SEH frame for the date and the local news, no destructor.
///
/// The record (player, club, today and a byte that in the original is stack leftover) joins the incident list
/// ([this+8], 0x68d0d0, 0x11 bytes).
///
/// Human-run club (`news_club_has_human_manager`): one coach from the 5 slots (+0x19f, draw(5), until one is found or draw(20) comes out
/// zero) and the news 0x138f to the club (the player, draw(2), his keys and the coach's, or −1).
///
/// AI club (`exe::human_manager_is_human_club`) with a manager (+0xcf, +0x69): draw(his +0x14) above draw(10) leads to the fine (`exe::fine_fine_player`, period
/// 1 if draw(rating `exe::non_player_attribute_rating` with +0x19) beats draw(5), else 2; reason 1).
/// Original: `AWOL::report_poor_fitness` (0x41add0, awol.cpp, thiscall, ret 8).
///
/// [pt] { 0x41add0 } (thiscall, ret 8, awol.cpp, SEH da data e da notícia local, sem destrutor): `AWOL::
/// report_poor_fitness(jogador, clube)`, pelo diário das pessoas (0x5ce2d0: a queda de condição grande do dia). O
/// registro (jogador, clube, hoje e um byte que na original é sobra de pilha) entra na lista dos incidentes ([this+8],
/// 0x68d0d0, 0x11 bytes). Clube de humano (0x7875e0): um treinador das 5 vagas (+0x19f, sorteio(5), até achar um ou
/// sorteio(20) dar zero) e a notícia 0x138f ao clube (o jogador, sorteio(2), as chaves dele e do treinador ou −1).
/// Clube da IA (0x600a50) com técnico (+0xcf, +0x69): sorteio(+0x14 dele) acima de sorteio(10) leva à multa (0x5a5980,
/// prazo 1 se sorteio(nota 0x544410 com o +0x19) passar de sorteio(5), senão 2; motivo 1).
pub(crate) unsafe extern "thiscall" fn report_poor_fitness(this: usize, player: usize, club: usize) {
    let mut reg = [0u8; 0x11];
    exe::date_date_54a110(reg.as_mut_ptr() as usize + 8);
    if player == 0 || club == 0 {
        return;
    }
    reg[..4].copy_from_slice(&read_u32(player).to_le_bytes());
    reg[4..8].copy_from_slice(&read_u32(club).to_le_bytes());
    let mut temporary = [0u8; 8];
    exe::date_operator_3d(reg.as_mut_ptr() as usize + 8, temporary.as_mut_ptr() as usize, G_CURRENT_DATE);
    // simplification: byte +0x10 of the record is stack leftover in the original; the copy writes 0 (nobody reads it)
    exe::llist_add_tail(read_usize(this + 8), reg.as_ptr() as usize, 0x11);
    if club_has_human_manager(club) {
        let mut coach;
        loop {
            let r = friendly_random(5);
            coach = read_usize(club.wrapping_add((r as usize).wrapping_mul(4)) + 0x19f);
            if friendly_random(0x14) == 0 || coach != 0 {
                break;
            }
        }
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        exe::new_newsitem(nt, 0x138f, 0);
        set_news_slot(nt, 0, read_u32(player));
        set_news_slot(nt, 1, read_u32(club));
        let r = friendly_random(2);
        set_news_slot(nt, 2, r as u32);
        set_news_slot(nt, 3, read_u32(read_usize(player + staff::FIRST_NAME) + 0x33));
        set_news_slot(nt, 4, read_u32(read_usize(player + staff::SECOND_NAME) + 0x33));
        set_news_slot(nt, 5, read_u32(read_usize(player + staff::COMMON_NAME) + 0x33));
        if coach != 0 {
            set_news_person_slots(nt, 6, coach);
        } else {
            for k in 6..10 {
                set_news_slot(nt, k, u32::MAX);
            }
        }
        exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    let manager = read_usize(club + club::MANAGER);
    if manager == 0 {
        return;
    }
    let np = read_usize(manager + staff::NON_PLAYER);
    if np == 0 {
        return;
    }
    crate::comparison::branch("cpz_9 fine evaluated");
    let a = friendly_random(read_i8_as_i32(np + non_player::DISCIPLINE));
    let b = friendly_random(10); // cpz9: the fine random draw
    if a <= b {
        return;
    }
    let np = read_usize(read_usize(club + club::MANAGER) + 0x69);
    crate::comparison::branch("cpz_9 fine by form");
    // simplification: the original pushes ecx with byte +0x19 only in cl (the high ones are leftover); 0x544410 only reads the byte
    let c =friendly_random((exe::non_player_attribute_rating(np, (*((np + non_player::MAN_HANDLING) as *const u8) as u32) as i8) as i8) as i32);
    let e = friendly_random(5); // cpz9: the deadline random draw
    let deadline = if c > e { 1 } else { 2 }; // cpz9: the fine deadline
    exe::fine_fine_player(read_usize(G_FINE), player, deadline, 1);
}

/// `AWOL::is_player_awol(person, read_only)`: returns bit 0 of the person's flag; without `read_only` and without a
/// contract or club, the absence ends first. Called by the squad selection, training, asking price, the staff day and
/// transfers.
///
/// Null person or id beyond [0xae2364]: 0 (only al). With `read_only`, bit 0 of the person's flag ([this+0xc] + id).
/// Otherwise, with the bit set and no contract (`exe::contract_manager_get_contract`, stored in [0xae1080]) or no club on the contract, the absence
/// ends (`clear_awol`); returns the bit read again.
/// Original: `AWOL::is_player_awol` (0x418f60, awol.cpp, thiscall, ret 8).
///
/// [pt] { 0x418f60 } (thiscall, ret 8, awol.cpp): `AWOL::is_player_awol(pessoa, só_ler)`, pela seleção, pelo treino, pelo
/// preço pedido, pelo dia da comissão e pelas transferências. Pessoa nula ou id além de [0xae2364]: 0 (só o al). Com
/// `só_ler`, o bit 0 da marca da pessoa ([this+0xc] + id). Senão, com o bit e sem contrato (0x4cf5a0, guardado em
/// [0xae1080]) ou sem clube do contrato, a ausência acaba (0x4196a0); devolve o bit relido.
pub(crate) unsafe extern "thiscall" fn is_player_awol(this: usize, p: usize, so_read: u32) -> u32 {
    if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let mark = || read_u8(read_usize(this + 0xc) + read_usize(p)) as u32 & 1;
    if so_read == 0 && mark() != 0 {
        let c = injury_main_contract_stored(p);
        if c == 0 || injury_club_of_contract(c) == 0 {
            crate::comparison::branch("cqb_5 absence ends without club");
            exe::awol_clear_awol(this, p);
        }
    }
    mark()
}

