use super::*;








/// Calls human_manager is_human_club(club, a, b, 0, 0).
pub(crate) unsafe fn media_is_human_club(c: usize, a: u32, b: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(c, a, b, 0, 0)
}

/// Collects the human clubs that receive news about a player (TRANSFER_MANAGER collect_player_news_recipients,
/// 0x8d1da0).
///
/// [pt] Os clubes humanos que recebem notícias do jogador (0x8d1da0 em [0xde1f54]: 4 na pilha, ret 0x10; o ecx da original é
/// o gerenciador das transferências).
pub(crate) unsafe fn player_news_recipients(player: usize, marks: u32, out: *mut usize, max: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    f(read_usize(G_TRANSFER_MANAGER), player, marks, out, max)
}

/// Writes a person's id and three name keys (+0x33 of +4, +8, +0xc) into news slots k..k+3.
///
/// [pt] O id e os três nomes (+0x33 de +4, +8, +0xc) de uma pessoa, nos índices `k`..`k+3` da notícia.
pub(crate) unsafe fn set_person_name_slots(write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    write(n, k, read_usize(p) as u32);
    write(n, k + 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    write(n, k + 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    write(n, k + 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// Appends club c to the recipients array (max 30) if not already there.
///
/// [pt] Acrescenta `c` aos destinos se ainda não estiver lá (até 30).
pub(crate) fn add_unique_recipient(dest: &mut [usize; 30], n: &mut u8, c: usize) {
    if !dest[..(*n as i8).max(0) as usize].contains(&c) {
        dest[*n as i8 as usize] = c;
        *n = n.wrapping_add(1);
    }
}

/// drops transfer interest older than 31 days and, by chance, publishes the rumour linking the player to the interested
/// clubs, with the transfer request, the AI manager response, the squad reaction and the rival warning; the AI owner
/// may say he will not sell
/// Original: `Media::process_transfer_interest` (0x754a30, media.cpp, thiscall).
///
/// [pt] { 0x754a30 } (thiscall, ret 0, media.cpp, quadro SEH sem destrutor no caminho normal):
/// `Media::process_transfer_interest()`, pela rotina diária (0x5cdd8f). Para cada registro de interesse (vetor de 25
/// bytes em [this+0xc], conta [this+0x14]: +0 jogador, +4 clube, +8 data, +0x10 estado, +0x11, +0x15): vencido (mais de
/// 31 dias, 0x54c5b0), sem técnico no clube dono (o +0xcf do clube do contrato do jogador, guardado da volta anterior
/// quando ele não tem contrato) ou sem ficha: sai (o último toma o lugar). Senão, se o clube pode chegar ao jogador
/// (0x8ce990): estado 0 e sorteio(25) zero dão o boato: os até 5 clubes interessados nele (registros do mesmo jogador
/// ainda em 0, marcados 1; um já marcado cancela), a notícia 0xfbc (0x77bbe0) com o jogador, o dono, o pedido de
/// transferência (abaixo de 15 em +0x5b e +0x59, reputação acima do sorteio(12500) e o +0x15 acima de 12,5 × +0x59 + 500:
/// 0x24, senão sorteio(3)), +0x11, a resposta do técnico da IA (0x735870), as forças (0x546ab0), as posições (0x5b7d20),
/// o técnico do dono, a lista de transferências (0x544520) e os interessados por reputação; aos humanos (0x8d1da0, os
/// interessados e os humanos da mesma divisão ou da mesma nação e 3000 abaixo) (0x7861e0); com o pedido, a reação do
/// elenco (0x755340); com a resposta, o aviso ao rival (0x758010) e a resposta à história (0x741020). No estado 1, o
/// dono da IA, a data do registro depois de +0x3e do jogador, o vínculo sem +0x49 e fora da lista, felicidade acima de
/// sorteio(50) + 40 e sorteio(30) zero: a notícia de que não vende (0x587600) e os registros dele vão a 2. No fim, o
/// vetor é ordenado (0x945333 com 0x757ea0).
pub(crate) unsafe extern "thiscall" fn process_transfer_interest(this: usize) {
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let reply: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_MEDIA_PICK_AI_MANAGER_RESPONSE);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_GET_PLAYER_STRENGTH_FLAGS);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let to_manager: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    if read_i32(this + 0x14) == 0 {
        return;
    }
    // simplification: o técnico do dono fica da volta anterior quando o jogador não tem contrato (a original não o regrava);
    // na primeira volta a original lê a pilha sem valor: aqui 0
    let mut manager_owner: usize = 0;
    let mut i = 0i32;
    while i < read_i32(this + 0x14) {
        let r = read_usize(this + 0xc).wrapping_add((i as usize).wrapping_mul(25));
        let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE));
        let clube = month_award_club_by_id(read_i32(r + 4));
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
        core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
        let (mut dono, mut vinc) = (0usize, 0usize);
        if c != 0 {
            dono = month_award_club_by_id(read_i32(c + 4));
            if dono != 0 {
                vinc = contract_bond(read_usize(G_CONTRACT_MANAGER), person, dono);
                manager_owner = read_usize(dono + 0xcf);
            }
        }
        let mut today = [0u8; 8];
        core::ptr::copy_nonoverlapping(G_CURRENT_DATE as *const u8, today.as_mut_ptr(), 8);
        let mut end = [0u8; 8];
        let p = mais(r + 8, end.as_mut_ptr(), 0x1f); // cqc10: o interesse vale 31 dias
        crate::comparacao::ramo("cqc_10 interesse");
        let (hd, ha) = (i16::from_le_bytes([today[0], today[1]]), u16::from_le_bytes([today[2], today[3]]));
        let vencido = ha > read_i16(p + 2) as u16 || (ha == read_i16(p + 2) as u16 && hd > read_i16(p));
        if vencido || manager_owner == 0 || read_usize(person + staff::PLAYER) == 0 {
            let n = read_i32(this + 0x14);
            let last = read_usize(this + 0xc).wrapping_add((n as usize).wrapping_mul(25)).wrapping_sub(25);
            core::ptr::copy(last as *const u8, r as *mut u8, 25);
            core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1);
            continue; // o último veio para o lugar i: a original volta um e avança um
        }
        if chegar(read_usize(G_TRANSFER_MANAGER), person, clube, 0, 0) == 0 {
            i += 1;
            continue;
        }
        if read_u8(r + 0x10) == 0 && {
            crate::comparacao::ramo("cqc_10 sorteio do boato");
            friendly_random(0x19) == 0 // cqc10: o sorteio do boato
        } {
            // os clubes interessados no jogador
            let mut ids = [-1i32; 5];
            ids[0] = read_i32(r + 4);
            *((r + 0x10) as *mut u8) = 1;
            let mut cnt: i8 = 1;
            let mut cancelar = false;
            let mut j = 0i32;
            while j < read_i32(this + 0x14) && cnt < 5 {
                let rj = read_usize(this + 0xc).wrapping_add((j as usize).wrapping_mul(25));
                if j != i && read_i32(rj) == read_i32(r) {
                    if read_u8(rj + 0x10) != 0 {
                        cancelar = true;
                        break;
                    }
                    ids[cnt as usize] = read_i32(rj + 4);
                    cnt += 1;
                    *((rj + 0x10) as *mut u8) = 1;
                }
                j += 1;
            }
            if cancelar {
                i += 1;
                continue;
            }
            crate::comparacao::ramo("cqc_10 boato");
            let mut noticia = [0u32; 0x3a]; // a notícia local
            let n = noticia.as_mut_ptr() as usize;
            construct(n, 0xfbc, 0);
            set_person_name_slots(write, n, 0, person);
            write(n, 4, if dono != 0 { read_usize(dono) as u32 } else { 0xffff_ffff });
            let mut pede = false;
            let mut sai = false;
            if read_i8(person + staff::PROFESSIONALISM) < 0xf && read_i8(person + staff::LOYALITY) < 0xf {
                crate::comparacao::ramo("cqc_10 pedido de transferencia");
                let rr = friendly_random(0x30d4); // cqc10: o sorteio da reputação para o pedido
                if (read_i16(read_usize(person + staff::PLAYER) + 0xd) as i32) > rr {
                    let lim = read_i8(person + staff::LOYALITY) as i32 as f64 * 12.5 + 500.0;
                    if lim < read_i32(r + 0x15) as f64 {
                        write(n, 5, 0x24);
                        pede = true;
                        sai = true;
                    }
                }
            }
            if !sai {
                let s = friendly_random(3);
                write(n, 5, s as u32);
            }
            write(n, 6, read_usize(r + 0x11) as u32);
            let resp = if dono != 0 && media_is_human_club(dono, 0, 1) == 0 {
                reply(this, read_usize(dono + 0xcf), person, 0x1f)
            } else {
                0
            };
            write(n, 7, resp);
            write(n, 8, forcas(person, dono));
            write(n, 9, posicoes(read_usize(person + staff::PLAYER)) as i16 as i32 as u32);
            if manager_owner != 0 {
                set_person_name_slots(write, n, 0xa, manager_owner);
            } else {
                for k in 0xa..=0xd {
                    write(n, k, 0xffff_ffff);
                }
            }
            write(n, 0xe, (listada(person, 0) != 0) as u32);
            write(n, 0xf, cnt as i32 as u32);
            // os interessados por reputação (do maior para o menor)
            let rep = |id: i32| read_i16(month_award_club_by_id(id) + 0x80);
            let c = cnt as usize;
            for a in 0..c.saturating_sub(1) {
                for b in a + 1..c {
                    if rep(ids[b]) > rep(ids[a]) {
                        ids.swap(a, b);
                    }
                }
            }
            for (k, &id) in ids.iter().take(c).enumerate() {
                write(n, 0x10 + k as i8, id as u32);
            }
            // os destinos
            let mut dest = [0usize; 30];
            let mut nd: u8 = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(person, 0xda, dest.as_mut_ptr(), 0x1e) } else { 0 };
            for &id in ids.iter().take(c) {
                if (nd as i8) >= 0x1e {
                    break;
                }
                add_unique_recipient(&mut dest, &mut nd, month_award_club_by_id(id));
            }
            let total = read_i32(G_STAFF_COUNT);
            for q in 0..16i32 {
                let h = read_usize(G_STAFF_TABLE).wrapping_add(((total - 1 - q) as usize).wrapping_mul(staff::SIZE));
                let hc = read_usize(h + 0x39);
                if read_u8(h + 0x5f) == 0 || hc == 0 {
                    continue;
                }
                for &id in ids.iter().take(c) {
                    if (nd as i8) >= 0x1e {
                        break;
                    }
                    let ci = month_award_club_by_id(id);
                    let hc = read_usize(h + 0x39);
                    let junto = read_usize(ci + 0x57) == read_usize(hc + 0x57)
                        || (read_usize(ci + 0x53) == read_usize(hc + 0x53) && (read_i16(ci + 0x80) as i32 - 0xbb8) > read_i16(hc + 0x80) as i32);
                    if junto {
                        add_unique_recipient(&mut dest, &mut nd, hc);
                    }
                }
            }
            for &d in dest.iter().take((nd as i8).max(0) as usize) {
                to_manager(read_usize(G_NEWS), n, d);
            }
            if pede {
                let reacao: unsafe extern "thiscall" fn(usize, u32, usize, usize, *mut i32, u32) = core::mem::transmute(FN_MEDIA_APPLY_SQUAD_REACTION_TO_PLAYER_STORY);
                reacao(this, 0x1f, person, dono, ids.as_mut_ptr(), cnt as u8 as u32);
            }
            if dono != 0 && media_is_human_club(dono, 0, 1) == 0 && resp != 0 {
                if clube != 0 {
                    let rival: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_WARN_SQUAD_OF_POSITION_RIVAL);
                    rival(this, person, clube, dono, resp);
                }
                let historia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_APPLY_PLAYER_STORY_RESPONSE);
                historia(this, person, dono, 0x1f, resp);
            }
            i += 1;
            continue;
        }
        // estado 1: o dono da IA pode avisar que não vende
        if read_u8(r + 0x10) == 1 && dono != 0 && media_is_human_club(dono, 1, 1) == 0 {
            let (rd, ra) = (read_i16(r + 8), read_i16(r + 0xa) as u16);
            let (pd, pa) = (read_i16(person + staff::DATE_JOINED_CLUB), read_i16(person + 0x40) as u16);
            let after = ra > pa || (ra == pa && rd > pd);
            if after && vinc != 0 && read_u8(vinc + 0x49) == 0 && listada(person, 0) == 0 {
                let feliz = read_i8(vinc + 0x3d) as i32;
                let s = friendly_random(0x32);
                if feliz > s + 0x28 && friendly_random(0x1e) == 0 {
                    crate::comparacao::ramo("cqc_10 nao vende");
                    let nao_vende: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(FN_NEWS_CREATE_NOT_FOR_SALE_STORY);
                    nao_vende(person, dono, read_usize(dono + 0xbf), 0);
                    let mut j = 0i32;
                    while j < read_i32(this + 0x14) {
                        let rj = read_usize(this + 0xc).wrapping_add((j as usize).wrapping_mul(25));
                        if read_i32(rj) == read_i32(read_usize(this + 0xc).wrapping_add((i as usize).wrapping_mul(25))) {
                            *((rj + 0x10) as *mut u8) = 2;
                        }
                        j += 1;
                    }
                }
            }
        }
        i += 1;
    }
    let n = read_i32(this + 0x14);
    if n != 0 {
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        qsort(read_usize(this + 0xc), n, 0x19, 0x757ea0);
    }
}

// simplification: tira registros (o vetor), sorteia, constrói e entrega notícias e chama as reações e respostas (0x755340,
// 0x758010, 0x741020, 0x587600): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.

/// news item 0xfbe that the club will not sell the player, with the chairman when he likes the player; to every manager
/// for a famous player, else to the interested human managers
/// Original: `news_create_not_for_sale_story` (0x587600, media.cpp, cdecl).
///
/// [pt] { 0x587600 } (cdecl, media.cpp no fluxo; european_cup.cpp no mapa, sem outro chamador; quadro SEH sem destrutor no
/// caminho normal): `news_create_not_for_sale_story(jogador, clube, presidente, outro)`: com ficha e clube, a notícia
/// 0xfbe (0x77bbe0) com o jogador, o outro clube (ou −1), o clube e um sorteio(3); o presidente, se gosta do jogador
/// (0x547810) ou com sorteio(+0x16 da ficha dele) acima de sorteio(20), senão −1 no 7; reputação (+0xd da ficha) acima de
/// 8000: a todos os técnicos (0x786780), senão aos humanos interessados (0x8d1da0 com 0xd8, até 30, 0x7861e0).
pub(crate) unsafe extern "C" fn news_create_not_for_sale_story(player: usize, clube: usize, presidente: usize, other: usize) {
    if player == 0 || read_usize(player + 0x61) == 0 || clube == 0 {
        return;
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let mut dest = [0usize; 30];
    let nd: u8 = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0xd8, dest.as_mut_ptr(), 0x1e) } else { 0 };
    crate::comparacao::ramo("cqc_10 noticia nao vende");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construct(n, 0xfbe, 0);
    set_person_name_slots(write, n, 0, player);
    write(n, 4, if other != 0 { read_usize(other) as u32 } else { 0xffff_ffff });
    write(n, 5, read_usize(clube) as u32);
    let s = friendly_random(3);
    write(n, 6, s as u32);
    let cita = gosta(player, presidente) != 0 || (presidente != 0 && read_usize(presidente + 0x69) != 0 && {
        let a = friendly_random(read_i8(read_usize(presidente + 0x69) + 0x16) as i32);
        a > friendly_random(0x14)
    });
    if cita {
        set_person_name_slots(write, n, 7, presidente);
    } else {
        write(n, 7, 0xffff_ffff);
    }
    let f = read_usize(player + 0x61);
    if f != 0 && read_i16(f + 0xd) > 0x1f40 {
        let all: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        all(read_usize(G_NEWS), n);
    } else {
        let to_manager: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        for &d in dest.iter().take((nd as i8).max(0) as usize) {
            to_manager(read_usize(G_NEWS), n, d);
        }
    }
}

// simplification: a notícia vai aos técnicos (aloca): só a original; a cópia leva o ramo.

/// news item 0x2e with the board and fans praising the manager for the title, with the outlet and the pundit, for a
/// manager with 180 days or more at the club
/// Original: `Media::queue_trophy_praise_story` (0x75c390, media.cpp, thiscall).
///
/// [pt] { 0x75c390 } (thiscall, ret 0xc, media.cpp, quadro SEH sem destrutor no caminho normal):
/// `Media::queue_trophy_praise_story(clube, competição, marca)`, pelos registros do título (0x8693e2, 0x86b000). Clube
/// com técnico, competição e fora das seleções: a notícia 0x2e (0x77bbe0) com o técnico, o clube, a competição, um
/// sorteio(2), o veículo (0x757f30), o comentarista (0x7dab90 com a nação ou −1), a marca, a confiança da diretoria
/// (+6 da linha em [0xb67a44], ou 5000), a liga do clube e os dias do técnico (0x54cdb0); com 180 dias ou mais, a
/// reputação do clube e a entrega ao técnico (0x7861e0).
pub(crate) unsafe extern "thiscall" fn queue_trophy_praise_story(_this: usize, clube: usize, comp: usize, mark: u32) {
    if clube == 0 || read_usize(clube + club::MANAGER) == 0 || comp == 0 {
        return;
    }
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if national_team(clube) != 0 {
        return;
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MEDIA_PICK_OUTLET_TYPE_FOR_CLUB);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(FN_PUNDITLIST_PICK_RANDOM_PUNDIT);
    let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    crate::comparacao::ramo("cqc_10 noticia do titulo");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construct(n, 0x2e, 0);
    set_person_name_slots(write, n, 0, read_usize(clube + club::MANAGER));
    write(n, 4, read_usize(clube) as u32);
    write(n, 5, read_usize(comp) as u32);
    let s = friendly_random(2);
    write(n, 6, s as u32);
    let v = veiculo(clube);
    write(n, 7, v);
    let na = read_usize(clube + club::NATION);
    let c = comentarista(read_usize(G_PUNDIT_LIST), clube, v, if na != 0 { read_i32(na) } else { -1 });
    write(n, 8, c as u32);
    write(n, 9, mark as i8 as i32 as u32);
    let mm = read_usize(G_MANAGER_MANAGER);
    let conf = if mm != 0 {
        read_i16(read_usize(mm).wrapping_add((read_i32(clube) as usize).wrapping_mul(0x49)) + 6) as i32
    } else {
        0x1388
    };
    write(n, 0xa, conf as u32);
    write(n, 0xb, (read_usize(clube + club::DIVISION) == comp) as u32);
    let manager = read_usize(clube + club::MANAGER);
    let days = ate(G_CURRENT_DATE, read_usize(manager + 0x3e) as u32, read_usize(manager + 0x42) as u32);
    write(n, 0xc, days as u32);
    if days < 0xb4 {
        return;
    }
    write(n, 0xd, read_i16(clube + club::REPUTATION) as i32 as u32);
    let to_manager: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    to_manager(read_usize(G_NEWS), n, clube);
}

// simplification: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.





/// CONTRACT_MANAGER get_contract_for_club (0x4cf5f0) for a person and club.
///
/// [pt] O vínculo da `pessoa` com o `clube` (0x4cf5f0: thiscall em [0xae1050], 2 na pilha; grava [0xae1080]).
pub(crate) unsafe fn media_contract_for_club(person: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    f(read_usize(G_CONTRACT_MANAGER), person, clube)
}

/// The person's whole contract at a club (+0..+0x4f) as a comparer region, or None.
///
/// [pt] O contrato inteiro (+0 a +0x4f) da `pessoa` no `clube`, como região do comparador.
pub(crate) unsafe fn contract_region(person: usize, clube: usize) -> Option<(usize, usize)> {
    let c = media_contract_for_club(person, clube);
    (c != 0).then_some((c, 0x50))
}

/// For a routine that internally makes tape calls: runs it on a scratch tape and replays the recorded inner calls
/// through `chamar` in order; outside post-match comparison just calls it.
///
/// [pt] Uma rotina sem porta que por dentro chama unidades da fita (0x544520 e 0x544600 chamam a 0x4cf5f0; a 0x62c400
/// alcança a 0x600a50 e portas): na comparação pela fita, a original grava essas chamadas internas no nível de cima
/// (com o comparador da rotina, a original e a cópia dele gravam cada uma a sua). A cópia roda a rotina numa fita de
/// rascunho e pede as mesmas chamadas por `chamar`, na ordem. Fora da comparação (sem RECORD_POST_MATCH) só chama.
pub(crate) unsafe fn replay_inner_tape_calls<R>(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, f: impl FnOnce() -> R) -> R {
    if !RECORD_POST_MATCH.load(std::sync::atomic::Ordering::Relaxed) {
        return f();
    }
    let externa = tape_lock().replace((Vec::new(), Vec::new()));
    let r = f();
    let gravadas = core::mem::replace(&mut *tape_lock(), externa).map_or(Vec::new(), |g| g.1);
    for c in gravadas {
        call(c.0, c.1, c.2);
    }
    r
}

/// Builds the 58-entry vector 0x755340 uses: the 50 squad players, chairman, manager (unless human or player job),
/// assistant and 5 coaches.
///
/// [pt] O vetor que a 0x755340 monta na pilha (58 dwords, ela percorre 57): os 50 do elenco (0x541f90 com 0, cdecl), o
/// +0xbf do clube, o técnico (+0xcf) só sem humano no clube (0x600a50 com 0, 1, 0, 0, cdecl, por `chamar`: unidade da
/// fita) e fora dos tipos 11 a 15 (0x53b620, cdecl, o byte +0x3d), o +0xd3 e os cinco dwords de +0x19f.
pub(crate) unsafe fn club_staff_vector(clube: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> [usize; 58] {
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let kind: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let mut v = [0usize; 58];
    reunir(clube, v.as_mut_ptr(), 0);
    v[50] = read_usize(clube + club::CHAIRMAN);
    let manager = read_usize(clube + club::MANAGER);
    v[51] = if call(FN_HUMAN_MANAGER_IS_HUMAN_CLUB, 0, [clube as u32, 0, 1, 0, 0]) != 0 || (manager != 0 && kind(*((manager + 0x3d) as *const u8) as u32) != 0) {
        0
    } else {
        manager
    };
    v[52] = read_usize(clube + club::ASSISTANT_MANAGER);
    for k in 0..5 {
        v[53 + k] = read_usize(clube + club::COACHES + 4 * k);
    }
    v
}

/// 0x755340 Media::apply_squad_reaction_to_player_story: after a story about a player, interested clubs and the club's
/// staff may come to dislike him, setting contract flags and grievances.
///
/// [pt] { 0x755340 } (thiscall, ret 0x14, media.cpp): `Media::apply_squad_reaction_to_player_story(tópico, jogador, clube,
/// ids, n)`, depois de uma história sobre o jogador (o boato de transferência 0x754a30 com o tópico 0x1f e os clubes
/// interessados; 0x72c1f0 e 0x733a60 sem ids). A chance começa em 10 (5 se o jogador é favorito do clube, 0x547790; 15
/// se rejeitado, 0x5477d0) e cresce por clube interessado: 100 no clássico de Glasgow, 10 se a liga tem o confronto
/// (0x4b02e0), 15 se o clube é rival dele (+0xb3/+0xb7/+0xbb), 5 se é da mesma divisão (fora da divisão inferior).
/// Sorteio(chance) acima de sorteio(10): o clube rejeita o jogador (0x547940 com 1). Listado (0x544520): −25; sem
/// vínculo: −15; senão −10 no fim de contrato (0x867d30) e mais a felicidade (+0x3d) / 10; no mínimo 5. Para cada uma
/// das 57 pessoas do vetor (club_staff_vector) com preferências (+0x65): a nota (10, o +0x1d da ficha de técnico + 10, ou pelo
/// +0x5d, idade, +0x5b e lista a pedido do jogador do elenco), ajustada pelo tópico 0x20/0x1e, pelo +0x59/+0x5a/+0x5b,
/// por gostar ou não do jogador (0x547810/0x547840) e pelo jogador ser favorito ou rejeitado do clube, no mínimo 5;
/// sorteio(chance) acima de sorteio(nota): a pessoa passa a não gostar do jogador (0x547af0 com 1) e, com vínculo e
/// sorteio(nota) abaixo de sorteio(10), a marca 1 do contrato (0x867560) ou, nos tópicos 0x1e/0x20 ou com sorteio(nota)
/// zero, as queixas 0x800 (0x867700). As três com efeito vão por `chamar` (as portas do pós-jogo), e também as unidades
/// da fita que a original chama: o vínculo 0x4cf5f0 (0x75551e, 0x7557dc), o humano 0x600a50 (0x755598) e as 0x4cf5f0
/// de dentro da 0x544520 (0x7554e8) e da 0x544600 (0x75567f, 0x7556ad), estas por `replay_inner_tape_calls`.
pub(crate) unsafe fn squad_reaction_to_story_with(topico: u32, player: usize, clube: usize, ids: usize, n: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let tie: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_CLUB_ADD_DISLIKED_STAFF);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_EXPIRY_BAND);
    let request: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LISTED_BY_REQUEST);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    if clube == 0 || player == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_8 reacao do elenco");
    let mut chance: i32 = 0xa; // cqb8: a chance inicial
    if favorito(player, clube) != 0 {
        chance = 5;
    } else if rejeitado(player, clube) != 0 {
        chance = 0xf;
    }
    if ids != 0 {
        let n = n as i8 as i32;
        let mut k = 0i32;
        while k < n {
            let id = read_usize(ids + 4 * k as usize);
            let other = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE));
            k += 1;
            let (celtic, rangers) = (read_usize(G_KEY_CLUB_GLASGOW_CELTIC), read_usize(G_KEY_CLUB_GLASGOW_RANGERS));
            let meu = read_usize(clube);
            if (meu == celtic || meu == rangers) && (read_usize(other) == celtic || read_usize(other) == rangers) {
                chance += 100;
                continue;
            }
            if tie(clube, other) != 0 {
                chance += 10;
                continue;
            }
            if clube == read_usize(other + 0xb3) || clube == read_usize(other + 0xb7) || clube == read_usize(other + 0xbb) {
                chance += 0xf;
                continue;
            }
            let div = read_usize(clube + club::DIVISION);
            if div != 0 && read_usize(div) != read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && div == read_usize(other + 0x57) {
                chance += 5;
            }
        }
    }
    let r1 = friendly_random(chance);
    if r1 > friendly_random(10) {
        rejeitar(clube, player, 1);
    }
    // cqb8: a 0x4cf5f0 (de dentro da 0x544520 e direta) é unidade da fita: vai por `chamar`
    if replay_inner_tape_calls(call, || listada(player, 0)) != 0 {
        chance -= 0x19;
    } else {
        let c = call(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, read_usize(G_CONTRACT_MANAGER), [player as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 {
            chance -= 0xf;
        } else {
            if band(c) != 0 {
                chance -= 10;
            }
            chance += (read_i8(c + 0x3d) as i32 / 10) as i8 as i32;
        }
    }
    if chance < 5 {
        chance = 5;
    }
    let v = club_staff_vector(clube, call);
    let topico = topico as u16;
    for &p in v.iter().take(57) {
        if p == 0 || p == player || read_usize(p + 0x65) == 0 {
            continue;
        }
        let mut rating: i8 = 10;
        let manager = read_usize(p + 0x69);
        if manager != 0 {
            rating = read_i8(manager + 0x1d).wrapping_add(10);
        } else if read_usize(p + 0x61) != 0 {
            if read_i8(player + 0x18) > 0x14 && read_i8(p + 0x18) < 0x17 && read_i16(read_usize(p + 0x61) + 0xb) < 0x109a {
                continue;
            }
            crate::comparacao::ramo("cqb_8 nota do jogador");
            rating = read_i8(p + 0x5d).wrapping_add(10); // cqb8: a nota do jogador do elenco
            if read_i8(p + 0x18) < 0x14 {
                rating = rating.wrapping_sub(5);
            }
            let m = read_i8(p + 0x5b);
            if m == 0x14 {
                rating = rating.wrapping_add(1);
            } else if m == 1 {
                rating = rating.wrapping_sub(if replay_inner_tape_calls(call, || request(p, 0)) != 0 { 0xf } else { 10 });
                if read_i16(read_usize(p + 0x61) + 0xb) > 0x1676 {
                    rating = rating.wrapping_sub(10);
                }
            } else if m < 6 {
                rating = rating.wrapping_sub(if replay_inner_tape_calls(call, || request(p, 0)) != 0 { 10 } else { 5 });
                if read_i16(read_usize(p + 0x61) + 0xb) > 0x1676 {
                    rating = rating.wrapping_sub(5);
                }
            }
        }
        if topico == 0x20 && read_i8(p + 0x59) > 0xf && read_i8(p + 0x5d) < 6 {
            rating = rating.wrapping_sub(5);
        }
        let a59 = read_i8(p + 0x59);
        if a59 > 0xf {
            if read_i8(p + 0x5b) != 0x14 {
                rating = rating.wrapping_sub(2);
            }
        } else if a59 < 5 {
            rating = rating.wrapping_add(2);
        }
        if topico == 0x1e {
            let (a5b, a5a) = (read_i8(p + 0x5b), read_i8(p + 0x5a));
            if a5b > 0xf {
                if a5a < 0xf {
                    rating = rating.wrapping_sub(1);
                }
            } else if a5b >= 6 {
                rating = rating.wrapping_sub(if a5a >= 0xf { 1 } else { 3 });
            } else {
                rating = rating.wrapping_sub(if a5a < 6 { 10 } else { 5 });
            }
        }
        if gosta(player, p) != 0 {
            rating = rating.wrapping_add(0x19);
        } else if not_likes(player, p) != 0 {
            rating = rating.wrapping_add(read_i8(player + 0x5b).wrapping_sub(0x19));
            if rating > 0xf {
                rating = 0xf;
            }
        }
        if favorito(player, clube) != 0 {
            rating = rating.wrapping_add(5);
        } else if rejeitado(player, clube) != 0 {
            rating = rating.wrapping_sub(5);
        }
        if rating < 5 {
            rating = 5;
        }
        let r1 = friendly_random(chance);
        if r1 > friendly_random(rating as i32) { // cqb8: o desafeto sorteado
            crate::comparacao::ramo("cqb_8 desafeto sorteado");
            call(FN_STAFF_ADD_DISLIKED_STAFF, 0, [p as u32, player as u32, 1, 0, 0]);
            let c = call(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, read_usize(G_CONTRACT_MANAGER), [p as u32, clube as u32, 0, 0, 0]) as usize;
            if c != 0 {
                let r = friendly_random(rating as i32);
                if r < friendly_random(10) {
                    if topico != 0x1e && topico != 0x20 && friendly_random(rating as i32) != 0 {
                        call(FN_STAFF_CONTRACT_SET_FLAGS, c, [1, 0, 0, 0, 0]);
                    } else {
                        call(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS, c, [0x800, 0, 0, 0, 0]);
                    }
                }
            }
        }
    }
}

/// after a story about the player (the transfer rumour): the dislike chance from the interested clubs (Old Firm, league
/// fixture, rival, same division), the club rejecting him and, for each squad, staff and board member, a score from
/// squad status, ambition, loyalty and preferences; when drawn, the person comes to dislike the player and his contract
/// gets flag 1 or the 0x800 complaints
/// Original: `Media::apply_squad_reaction_to_player_story` (0x755340, media.cpp, thiscall).
///
/// [pt] depois de uma história sobre o jogador (o boato de transferência): a chance de desafeição pelos clubes interessados
/// (clássico de Glasgow, confronto na liga, rival, mesma divisão), a rejeição pelo clube e, para cada pessoa do elenco,
/// da comissão e da diretoria, a nota pela posição no elenco, ambição, lealdade e preferências; sorteada, a pessoa
/// passa a não gostar do jogador e o contrato dela ganha a marca 1 ou as queixas 0x800
pub(crate) unsafe extern "thiscall" fn apply_squad_reaction_to_player_story(_this: usize, topico: u32, player: usize, clube: usize, ids: usize, n: u32) {
    squad_reaction_to_story_with(topico, player, clube, ids, n, &mut |r, t, a| dispatch_post_match_call(r, t, a))
}


/// 0x758010 Media::warn_squad_of_position_rival: squad players in the same position may react to a reputable newcomer
/// with a story (0x25) and contract flag 0x800.
///
/// [pt] { 0x758010 } (thiscall, ret 0x10, media.cpp): `Media::warn_squad_of_position_rival(jogador, clube, _, _)`, pelo
/// boato de transferência (0x754a30) e pela resposta à história do jogador (0x741020). Com ficha, clube com mais de 2
/// do elenco na máscara da posição dele (0x548150 com 0x5481a0), divisão com competição ([0xadadfc]) e reputação
/// (+0xb da ficha) acima de 4750: cada um dos 50 do elenco (+0xd7) com ficha, vínculo (0x4cf5f0) de felicidade (+0x3d)
/// positiva, o jogador sem lesão de estado 2 (0x62c400 em [0xb63d28]), sem as marcas 0x101 (+0x45), fora da lista
/// (0x544520), acima de 20 anos, do tipo 0x10/0x20/0x30 (+0x4f & 0xf0), com +0x5a abaixo de n + 5 e +0x5b abaixo de
/// n + 10, sorteio(+0x5a) abaixo de sorteio(n), a mesma máscara e reputação acima de 3750: a primeira vez, a história
/// 0x25 dele (0x72c1f0 em [0xb6b7c8]); sem a marca 0x800 (0x8676f0), a marca 0x800 (0x867560). As duas com efeito vão
/// por `chamar`, e também o vínculo 0x4cf5f0 (0x7580c2, unidade da fita) e, por `replay_inner_tape_calls`, o que a 0x62c400
/// (0x7580ee) e a 0x544520 (0x75810a) chamam de unidades por dentro.
pub(crate) unsafe fn warn_position_rivals_with(player: usize, clube: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mask: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    let count: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_CLUB_COUNT_SQUAD_WITH_POSITION_MASK);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let bandeiras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    if player == 0 || read_usize(player + 0x61) == 0 || clube == 0 {
        return;
    }
    let n = count(clube, mask(player)) as i8;
    if n <= 2 {
        return;
    }
    let div = read_usize(clube + club::DIVISION);
    if div == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(div)) == 0 {
        return;
    }
    if read_i16(read_usize(player + 0x61) + 0xb) <= 0x128e {
        return;
    }
    crate::comparacao::ramo("cqb_8 rival de posicao");
    let n = n as i32;
    let mut postou = false;
    for i in 0..50 {
        let p = read_usize(clube + club::SQUAD + 4 * i);
        if p == 0 || read_usize(p + 0x61) == 0 || p == player {
            continue;
        }
        // cqb8: a 0x4cf5f0 é unidade da fita (direta e de dentro da 0x544520; a 0x62c400 alcança a 0x600a50)
        let c = call(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, read_usize(G_CONTRACT_MANAGER), [p as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 || read_usize(p + 0x61) == 0 || read_i8(c + 0x3d) <= 0 {
            continue;
        }
        if replay_inner_tape_calls(call, || lesao(read_usize(G_INJURY), player)) == 2 || read_usize(c + 0x45) & 0x101 != 0
            || replay_inner_tape_calls(call, || listada(p, clube)) != 0
        {
            continue;
        }
        if read_i8(p + 0x18) <= 0x14 {
            continue;
        }
        let t = *((c + 0x4f) as *const u8) & 0xf0;
        if t != 0x10 && t != 0x20 && t != 0x30 {
            continue;
        }
        let a5a = read_i8(p + 0x5a) as i32;
        if a5a >= n + 5 || read_i8(p + 0x5b) as i32 >= n + 10 {
            continue;
        }
        let r1 = friendly_random(n);
        if friendly_random(a5a) >= r1 {
            continue;
        }
        let m = mask(player);
        if m != mask(p) || read_i16(read_usize(p + 0x61) + 0xb) <= 0xea6 {
            continue;
        }
        if !postou {
            call(FN_MEDIA_CREATE_PLAYER_MEDIA_STORY, read_usize(G_MEDIA), [p as u32, clube as u32, 0x25, 0, player as u32]);
            postou = true;
        }
        if bandeiras(c) & 0x800 != 0 {
            continue;
        }
        call(FN_STAFF_CONTRACT_SET_FLAGS, c, [0x800, 0, 0, 0, 0]);
    }
}

/// with a high-reputation player linked to the club and more than two squad players in his position: the unsettled
/// position rivals (by ambition and loyalty, by a draw) get contract flag 0x800, and the first one gives story 0x25
/// Original: `Media::warn_squad_of_position_rival` (0x758010, media.cpp, thiscall).
///
/// [pt] com o jogador de reputação alta ligado ao clube e mais de dois do elenco na posição dele: os rivais de posição
/// insatisfeitos (pela ambição e pela lealdade, por sorteio) ganham a marca 0x800 no contrato, e o primeiro dá a
/// história 0x25
pub(crate) unsafe extern "thiscall" fn warn_squad_of_position_rival(_this: usize, player: usize, clube: usize, _a: u32, _b: u32) {
    warn_position_rivals_with(player, clube, &mut |r, t, a| dispatch_post_match_call(r, t, a))
}


/// As respostas 1 e 2 de cada caso da tabela de saltos da 0x735160 (a 3ª, de todos, é "No Comment", 0xa27808); o caso
/// vem da tabela de bytes do exe em 0x7356f4 (tipo − 3, até 0x7f; acima, o 21).
pub(crate) const PRESS_ANSWERS: [(usize, usize); 22] = [
    (0xa27700, 0xa2771c), (0xa27670, 0xa2767c), (0xa27688, 0xa276a4), (0xa276e4, 0xa276f0), (0xa2773c, 0xa27770),
    (0xa2764c, 0xa2765c), (0xa27430, 0xa2743c), (0xa2762c, 0xa2763c), (0xa2744c, 0xa27460), (0xa275d0, 0xa27534),
    (0xa27554, 0xa27574), (0xa27510, 0xa27534), (0xa275b4, 0xa275c0), (0xa274e0, 0xa274f8), (0xa27588, 0xa275a0),
    (0xa274b4, 0xa274c4), (0xa2744c, 0xa27494), (0xa2744c, 0xa27610), (0xa275e8, 0xa275f8), (0xa2744c, 0xa2747c),
    (0xa27798, 0xa277c8), (0xa2740c, 0xa2741c),
];

/// the label of a media story response option ("Deny rumour", "Defend Player", "No Comment"...), from the 22-case table
/// by story kind
/// Original: `media_format_response_option_label` (0x735160, media.cpp, stdcall).
///
/// [pt] { 0x735160 } (stdcall, ret 0x10, media.cpp): `media_format_response_option_label(tipo, opção, saída, tamanho)`: o
/// rótulo da opção de resposta a uma história ("Deny rumour", "Defend Player", "No Comment"...) na saída (0x66f4e0),
/// pela tela das histórias (0x742b3b, com 0xde1f64 e 0x7d0). Saída nula ou tamanho abaixo de 15: nada.
pub(crate) unsafe extern "stdcall" fn media_format_response_option_label(kind: u32, opcao: u32, out: usize, size: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if out == 0 || (size as i16) < 0xf {
        return;
    }
    let k = (kind as i16 as i32).wrapping_sub(3) as u32;
    let caso = if k > 0x7f { 21 } else { *((0x7356f4 + k as usize) as *const u8) as usize };
    let (um, dois) = PRESS_ANSWERS[caso];
    let text = match opcao as i8 {
        1 => um,
        2 => dois,
        _ => 0xa27808,
    };
    format(out, text);
}







/// Staff record of id (0x6e bytes each in G_STAFF_TABLE).
///
/// [pt] A pessoa do id (0x6e bytes em [0xae23bc]).
pub(crate) unsafe fn media_staff_by_id(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize)
}


/// Staff natural position mask (0x5481a0).
///
/// [pt] A máscara da posição natural da pessoa (0x5481a0).
pub(crate) unsafe fn natural_position_mask(person: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    f(person)
}

/// STAFF_CONTRACT get_flags (0x8676f0, contract +0x40).
///
/// [pt] As marcas de evento do contrato (+0x40, 0x8676f0).
pub(crate) unsafe fn media_contract_flags(c: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    f(c)
}

/// STAFF_CONTRACT clear_flags (0x8676d0) on a contract.
///
/// [pt] Apaga marcas de evento do contrato (0x8676d0).
pub(crate) unsafe fn clear_contract_flags(c: usize, marks: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_FLAGS);
    f(c, marks);
}

/// Player's current reputation (+0xb of the player record at staff +0x61).
///
/// [pt] A reputação atual do jogador (+0xb da ficha +0x61).
pub(crate) unsafe fn current_reputation(person: usize) -> i16 {
    read_i16(read_usize(person + staff::PLAYER) + 0xb)
}

/// O ensaio do comparador da 0x758200: a cópia roda sem nada que aloque, dê notícia ou peça transferência; quando chega
/// a um desses, `MEDIA_REACTION_STOPPED` e ela para (a comparação fica só com a original).
pub(crate) static mut MEDIA_REACTION_DRY_RUN: bool = false;
pub(crate) static mut MEDIA_REACTION_STOPPED: bool = false;

/// STAFF_CONTRACT set_flags; in the dry run, checks whether it would trigger a transfer request and returns true (stop)
/// if so.
///
/// [pt] Liga marcas no contrato (STAFF_CONTRACT::set_flags 0x867560). No ensaio, pela cópia em ensaio dela (MISS_TRAINING_DRY_RUN): se
/// ela pediria transferência, para. Devolve se parou.
pub(crate) unsafe fn set_contract_flags_or_stop(c: usize, marks: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN {
        MISS_TRAINING_DRY_RUN = true;
        MISS_TRAINING_DECISION = 0;
        set_flags(c, marks);
        MISS_TRAINING_DRY_RUN = false;
        if MISS_TRAINING_DECISION != 0 {
            MEDIA_REACTION_STOPPED = true;
        }
        return MEDIA_REACTION_STOPPED;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_SET_FLAGS);
    f(c, marks);
    false
}

/// Side-effect-free copy of 0x867700's transfer-request decision with the given flags (job, not manager/assistant,
/// morale, wants to leave, no future transfer, not queued, age or reputation).
///
/// [pt] A decisão de pedir transferência da 0x867700 com as `marcas` (sem efeito: o +0x45 volta): funções 0xb, 0xf e 0xd,
/// fora o técnico e o auxiliar do clube do contrato, moral (+0x44) não negativa, sem +0x49, quer sair (0x866f60, com as
/// marcas já ligadas), sem transferência futura (0x8cca10), sem os bits 1 e 8 de +0x4e, fora da fila da transferência
/// livre (0x8c1870) e com mais de 20 anos ou reputação acima de 4250.
pub(crate) unsafe fn would_request_transfer(c: usize, marks: u32) -> bool {
    let person = media_staff_by_id(read_i32(c));
    let funcao = *((person + staff::JOB_FOR_CLUB) as *const u8);
    if funcao != 0xb && funcao != 0xf && funcao != 0xd {
        return false;
    }
    let clube = read_usize(G_CLUB_TABLE).wrapping_add((read_i32(c + 4) as u32).wrapping_mul(0x245) as usize);
    for o in [0xcf, 0xd3] {
        let t = read_usize(clube + o);
        if t != 0 && read_usize(c) == read_usize(t) {
            return false;
        }
    }
    if read_i8_as_i32(c + 0x44) < 0 || *((c + 0x49) as *const u8) != 0 {
        return false;
    }
    let before = read_usize(c + 0x45);
    core::ptr::write_unaligned((c + 0x45) as *mut usize, before | marks as usize);
    let quer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    let exit = quer(c) != 0;
    core::ptr::write_unaligned((c + 0x45) as *mut usize, before);
    if !exit {
        return false;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if futura(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 || *((c + 0x4e) as *const u8) & 9 != 0 {
        return false;
    }
    let in_queue: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_QUEUED_FOR_FREE_TRANSFER);
    if in_queue(read_usize(G_TRANSFER_MANAGER), person) != 0 {
        return false;
    }
    !(read_i8_as_i32(person + staff::YEAR_OF_BIRTH) <= 0x14 && current_reputation(person) <= 0x109a)
}

/// STAFF_CONTRACT apply_event_flags (0x867700); in the dry run stops first if the person would request a transfer.
///
/// [pt] As queixas do contrato (STAFF_CONTRACT::apply_event_flags 0x867700 com a pessoa). No ensaio, para antes se ela
/// pediria transferência. Devolve se parou.
pub(crate) unsafe fn apply_event_flags_or_stop(c: usize, marks: u32, person: usize) -> bool {
    if MEDIA_REACTION_DRY_RUN && would_request_transfer(c, marks) {
        MEDIA_REACTION_STOPPED = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    f(c, marks, person);
    false
}

/// STAFF_CONTRACT clear_grievances (0x8674e0); in the dry run stops first if the transfer request would be withdrawn.
///
/// [pt] Tira queixas do contrato (STAFF_CONTRACT::clear_grievances 0x8674e0). No ensaio, para antes se o pedido de
/// transferência seria retirado (a decisão `should_withdraw_transfer_request` com as queixas já tiradas). Devolve se parou.
pub(crate) unsafe fn clear_grievances_or_stop(c: usize, queixas: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN {
        let before = read_usize(c + 0x45);
        core::ptr::write_unaligned((c + 0x45) as *mut usize, before & !(queixas as usize));
        let retira = should_withdraw_transfer_request(c).is_some();
        core::ptr::write_unaligned((c + 0x45) as *mut usize, before);
        if retira {
            MEDIA_REACTION_STOPPED = true;
            return true;
        }
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
    f(c, queixas);
    false
}

/// Queues the signing celebration story (0x75b6e0); in the dry run stops instead.
///
/// [pt] A comemoração da contratação (0x75b6e0); no ensaio, para.
pub(crate) unsafe fn signing_celebration_or_stop(this: usize, p: usize, clube: usize, kind: u32, porta_voz: usize) -> bool {
    if MEDIA_REACTION_DRY_RUN {
        MEDIA_REACTION_STOPPED = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) = core::mem::transmute(FN_MEDIA_QUEUE_SIGNING_CELEBRATION_STORY);
    f(this, p, clube, kind, 1, porta_voz);
    false
}

/// Picks the squad spokesman: among the club's 50 players other than p with reputation above 4250, the highest
/// random(reputation + 20 x (+0x5b + +0x59) +- age adjustment).
///
/// [pt] O porta-voz do elenco: entre os 50 do clube com ficha, fora `p`, com reputação acima de 4250, o maior sorteio(reputação
/// + 20 × (+0x5b + +0x59) ± pela idade: abaixo de 19 −1500, de 23 −750, de 28 −250, de 34 +250), acima de 0.
pub(crate) unsafe fn squad_spokesman(clube: usize, p: usize) -> usize {
    let (mut melhor, mut quem) = (0i32, 0usize);
    for j in 0..50usize {
        let m = read_usize(clube + club::SQUAD + 4 * j);
        if m == 0 || read_usize(m + 0x61) == 0 || m == p {
            continue;
        }
        let r = current_reputation(m);
        if r <= 0x109a {
            continue;
        }
        let mut x = (r as i32).wrapping_add((read_i8_as_i32(m + 0x5b) + read_i8_as_i32(m + 0x59)) * 20);
        let age = read_i8_as_i32(m + 0x18);
        x += if age < 0x13 {
            -0x5dc
        } else if age < 0x17 {
            -0x2ee
        } else if age < 0x1c {
            -0xfa
        } else if age < 0x22 {
            0xfa
        } else {
            0
        };
        let s = friendly_random(x);
        if s > melhor {
            melhor = s;
            quem = m;
        }
    }
    quem
}

/// after the transfer of a player aged 19 or more with reputation 4250: in the new squad, the contract flags and
/// grievances by position and squad strength, the supporters (+500) and the celebration of a star signing; in the old
/// squad, the grievances at the team mate leaving, morale −25, cleared flags and the unsettled team mate news
/// Original: `Media::process_transfer_squad_reactions` (0x758200, media.cpp, thiscall).
///
/// [pt] { 0x758200 } (thiscall, ret 0x10, media.cpp): `Media::process_transfer_squad_reactions(jogador, novo, velho, flag)`,
/// no fim de cada transferência (TRANSFER_MANAGER::move_player_to_club, 0x8ca879). Só para o jogador (ficha +0x61) de 19
/// anos ou mais com reputação de 4250. (A) A história da mídia na frente da fila (0xbd0, +0x11 = 0x25, novo clube de id
/// 0x25; nunca na prática): queixas no contrato da pessoa dela. (B) No elenco do novo clube (50 vagas +0xd7), quem está
/// lá há mais de +0x5a + 15 dias (0x54cdb0 de +0x3e) e tem contrato: a marca 0x800 com a mesma posição troca por queixa
/// (0x867700 0x80000000) e sorteios; a 0x400 sai com o elenco forte (0x53c590 até o limite pela reputação); com mais de 2
/// da posição, o jogador forte pode virar queixa ou a marca 0x200; a queixa 0x20000000 sai (0x8674e0) quando chega
/// alguém maior. Promessa de craque (+0xd do jogador acima de 6250): a torcida (+8 da linha do clube em [[0xb67a44]],
/// +500 até 10000) e a comemoração (0x75b6e0); se algo mudou, sorteio(3) e o porta-voz comemoram. (C) No elenco do velho
/// clube (com liga), com a força e a reputação do clube (ou a nação entre as sete de 0x9cf31c..): as queixas 0x40000000 /
/// 0x20000000 pela saída do companheiro, a moral −25, a marca 0x400, as 0x200/0x800 e a queixa 0x80000000 tiradas; se
/// algo mudou e o jogador era craque, o companheiro desassossegado (0x75cfc0).
pub(crate) unsafe extern "thiscall" fn process_transfer_squad_reactions(this: usize, p: usize, new: usize, velho: usize, flag: u32) {
    if p == 0 || new == 0 {
        return;
    }
    let sheet = read_usize(p + 0x61);
    if sheet == 0 || read_i8_as_i32(p + 0x18) < 0x13 || read_i16(sheet + 0xb) < 0x109a {
        return;
    }
    let mut mudou = false;
    // (A) a história da mídia na frente da fila
    if read_i32(this + 8) > 0 {
        let h = read_usize(this);
        if read_i32(h + 0x13) == 0xbd0 && read_i16(h + 0x11) == 0x25 && read_i32(new) == 0x25 {
            let kind = *((h + 0x10) as *const u8);
            let id = read_i32(h + 8);
            if id != -1 {
                let q = media_staff_by_id(id);
                let rp = current_reputation(p);
                if natural_position_mask(p) == natural_position_mask(q) && (rp > 0x128e || rp as i32 - 0x5dc > current_reputation(q) as i32) {
                    let c = media_contract_for_club(q, new);
                    if c != 0 {
                        crate::comparacao::ramo("cqa_13 historia da midia");
                        mudou = true;
                        if kind == 1 {
                            if apply_event_flags_or_stop(c, 0x8000000, 0) || apply_event_flags_or_stop(c, 0x80000000, p) {
                                return;
                            }
                        } else if kind == 2 {
                            if media_contract_flags(c) & 0x800 != 0
                                && (set_contract_flags_or_stop(c, 0x200) || apply_event_flags_or_stop(c, 0x80000000, p) || set_contract_flags_or_stop(c, 0x80))
                            {
                                return;
                            }
                        } else if friendly_random(read_i8_as_i32(q + 0x5b)) < friendly_random(10) {
                            if set_contract_flags_or_stop(c, 0x80) {
                                return;
                            }
                            if friendly_random(read_i8_as_i32(q + 0x5a)) == 0 && apply_event_flags_or_stop(c, 0x80000000, p) {
                                return;
                            }
                        }
                        if media_contract_flags(c) & 0x800 != 0 {
                            clear_contract_flags(c, 0x800);
                        }
                    }
                }
            }
        }
    }
    // (B) o elenco do novo clube
    let count: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let same_position: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_CLUB_COUNT_SQUAD_WITH_POSITION_MASK);
    let strength = count(new, 0, 0) as i8 as i32;
    let rn = read_i16(new + 0x80);
    let limit = if rn < 0x9c4 { 0x14 } else if rn < 0x1482 { 0x19 } else if rn >= 0x1c52 { 0x20 } else { 0x1e }; // cqa13: o limite do elenco forte
    let same = same_position(new, natural_position_mask(p)) as i8 as i32;
    let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let category: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    for j in 0..50usize {
        let m = read_usize(new + 0xd7 + 4 * j);
        if m == 0 || read_usize(m + 0x61) == 0 || m == p {
            continue;
        }
        if days(G_CURRENT_DATE, read_usize(m + 0x3e) as u32, read_usize(m + 0x42) as u32) <= read_i8_as_i32(m + 0x5a) + 0xf {
            continue;
        }
        let c = media_contract_for_club(m, new);
        if c == 0 {
            continue;
        }
        if media_contract_flags(c) & 0x800 != 0 && natural_position_mask(p) == natural_position_mask(m) {
            clear_contract_flags(c, 0x800);
            if apply_event_flags_or_stop(c, 0x80000000, p) {
                return;
            }
            if friendly_random(read_i8_as_i32(m + 0x5a)) == 0 && set_contract_flags_or_stop(c, 0x200) {
                return;
            }
            if friendly_random(read_i8_as_i32(m + 0x5b)) == 0 && set_contract_flags_or_stop(c, 0x80) {
                return;
            }
        }
        if media_contract_flags(c) & 0x400 != 0 && strength >= limit {
            crate::comparacao::ramo("cqa_13 elenco forte");
            clear_contract_flags(c, 0x400);
        }
        if same > 2 && media_contract_flags(c) & 0x400 == 0 && current_reputation(m) > 0x109a && natural_position_mask(p) == natural_position_mask(m) {
            let rp = current_reputation(p);
            if rp > 0x128e || rp as i32 - 0x5dc > read_i16(new + 0x80) as i32 {
                let a = read_i8_as_i32(m + 0x5a);
                if a < 0xf && read_i8_as_i32(m + 0x5b) < 0xf && a < same + 4 && read_i8_as_i32(m + 0x57) < same + 8 && friendly_random(a) < friendly_random(6) {
                    let parou = if category(m, c) == 1 || category(m, c) == 2 {
                        apply_event_flags_or_stop(c, 0x80000000, p)
                    } else {
                        set_contract_flags_or_stop(c, 0x200)
                    };
                    if parou {
                        return;
                    }
                }
            }
        }
        if read_i32(c + 0x45) & 0x20000000 != 0 {
            let rp = current_reputation(p);
            if rp > 0x1482 || rp as i32 - 0x4e2 > read_i16(new + 0x80) as i32 {
                mudou = true;
                if strength > 0x28 || friendly_random(read_i8_as_i32(m + 0x5a)) != 0 {
                    crate::comparacao::ramo("cqa_13 queixa limpa");
                    if clear_grievances_or_stop(c, 0x20000000) { // cqa13: chega alguém maior
                        return;
                    }
                }
            }
        }
    }
    let sheet = read_usize(p + 0x61);
    if read_i16(sheet + 0xd) > 0x186a {
        let rp = read_i16(sheet + 0xb);
        if rp as i32 + 0x2ee > read_i16(new + 0x80) as i32 || rp > 0x222e {
            let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(new) as u32).wrapping_mul(0x49) as usize);
            let x = read_i16(row + 8).wrapping_add(0x1f4); // cqa13: a torcida comemora
            let x = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            crate::comparacao::ramo("cqa_13 goal_crowd_reaction comemora");
            core::ptr::write_unaligned((row + 8) as *mut i16, x);
            if signing_celebration_or_stop(this, p, new, 1, 0) {
                return;
            }
        }
    }
    if mudou {
        let sheet = read_usize(p + 0x61);
        if read_i16(sheet + 0xd) > 0x186a
            && read_i16(sheet + 0xb) as i32 + 0x2ee > read_i16(new + 0x80) as i32
            && read_i8_as_i32(p + 0x5b) > 0xa
            && read_i8_as_i32(p + 0x59) > 6
            && friendly_random(3) != 0
        {
            let who = squad_spokesman(new, p);
            if who != 0 && signing_celebration_or_stop(this, p, new, 2, who) {
                return;
            }
        }
    }
    // (C) o elenco do velho clube
    let mut mudou = false;
    if velho == 0 || read_usize(velho + 0x57) == 0 {
        return;
    }
    if read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(read_usize(velho + 0x57)))) == 0 {
        return;
    }
    let strength = count(velho, 0, 0) as i8 as i32;
    let nacao = read_usize(velho + 0x53);
    let sete = nacao != 0 && [KEY_NATION_GREECE, KEY_NATION_SWEDEN, KEY_NATION_WALES, KEY_NATION_FINLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_AUSTRALIA].iter().any(|&k| read_usize(nacao) == read_usize(k));
    let rv = read_i16(velho + 0x80);
    let (lim1, lim2) = if sete || rv < 0x1482 {
        (0x19, 0x2d)
    } else if rv < 0x1c52 {
        (0x1e, 0x2d)
    } else {
        (0x23, 0x2e)
    };
    let same = same_position(velho, natural_position_mask(p)) as i8 as i32;
    let candidato: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_STAFF_IS_BEST_FAVOURED_CANDIDATE);
    for j in 0..50usize {
        let m = read_usize(velho + 0xd7 + 4 * j);
        if m == 0 || read_usize(m + 0x61) == 0 || m == p {
            continue;
        }
        let c = media_contract_for_club(m, velho);
        if c == 0 {
            continue;
        }
        let mut q: i32 = if read_i32(c + 0x45) != 0 { 3 } else { 0 };
        if media_contract_flags(c) != 0 {
            q += 1;
        }
        if read_i8_as_i32(c + 0x44) < -0x19 {
            q += 1;
        }
        let q = q as i8 as i32;
        let mut queixa = 0u32; // 0x40000000 ou 0x20000000 pela saída do companheiro
        let mut second = true;
        if flag as u8 != 0
            && media_contract_flags(c) & 0x200 == 0
            && media_contract_flags(c) & 0x800 == 0
            && read_i32(c + 0x45) as u32 & 0x80000000 == 0
            && (strength < 0x28 || same < 6)
            && (natural_position_mask(p) != natural_position_mask(m) || same < 2)
        {
            let rp = current_reputation(p);
            if rp > 0x1e46 || rp > read_i16(new + 0x80).wrapping_sub(0x2ee) {
                let r = friendly_random(6);
                if read_i8_as_i32(m + 0x57) + q > r + 10 && read_i8_as_i32(c + 0x3d) > 0x19 && current_reputation(m) > 0x109a
                    && friendly_random(read_i8_as_i32(m + 0x5b)) < q + 5 && friendly_random(read_i8_as_i32(m + 0x5a)) == 0
                {
                    second = false;
                    queixa = if read_i8_as_i32(p + 0x18) < 0x18 && read_i16(read_usize(p + 0x61) + 0xd) < 0x128e { 0x40000000 } else { 0x20000000 };
                }
            }
        }
        if second
            && read_i8_as_i32(p + 0x18) > 0x17
            && flag as u8 != 0
            && candidato(m, new, p) != 0
            && natural_position_mask(p) != natural_position_mask(m)
            && friendly_random(read_i8_as_i32(m + 0x5b)) < q + 5
        {
            let rp = current_reputation(p);
            if rp > 0x1e46 || rp > read_i16(new + 0x80).wrapping_sub(0x2ee) {
                queixa = 0x20000000;
            } else {
                crate::comparacao::ramo("cqa_13 moral cai");
                let x = read_i8_as_i32(c + 0x44) - 0x19; // cqa13: a moral do companheiro cai
                *((c + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { x as i8 };
            }
        }
        if queixa != 0 {
            if apply_event_flags_or_stop(c, queixa, p) {
                return;
            }
            mudou = true;
        }
        if read_i8_as_i32(m + 0x18) > 0x17 && media_contract_flags(c) & 0x400 == 0 && read_i8_as_i32(m + 0x57) > 0xf {
            let r = (friendly_random(10) as u8).wrapping_add(10) as i8;
            if *((m + 0x58) as *const i8) > r && read_i8_as_i32(m + 0x5b) > 0xa && current_reputation(p) > 0x109a && current_reputation(m) > 0x109a && strength < lim1 && set_contract_flags_or_stop(c, 0x400) {
                return;
            }
        }
        let iguais = || natural_position_mask(p) == natural_position_mask(m);
        if (strength < lim2 || same < 2) && iguais() && media_contract_flags(c) & 0x200 != 0 {
            clear_contract_flags(c, 0x200);
        }
        let remove_800 = if strength < lim2 {
            let r1 = friendly_random(read_i8_as_i32(p + 0x5a));
            (r1 > friendly_random(10) && iguais()) || same < 2
        } else {
            same < 2
        };
        if remove_800 && media_contract_flags(c) & 0x800 != 0 {
            clear_contract_flags(c, 0x800);
        }
        if ((strength < lim2 && iguais()) || same < 2) && read_i32(c + 0x45) as u32 & 0x80000000 != 0 && clear_grievances_or_stop(c, 0x80000000) {
            return;
        }
        if media_contract_flags(c) & 0x400 != 0 {
            if read_i32(c + 0x45) as u32 & 0x80000000 != 0 && clear_grievances_or_stop(c, 0x80000000) {
                return;
            }
            if media_contract_flags(c) & 0x800 != 0 {
                clear_contract_flags(c, 0x800);
            }
            if media_contract_flags(c) & 0x200 != 0 {
                clear_contract_flags(c, 0x200);
            }
        }
    }
    if !mudou || read_i8_as_i32(p + 0x18) >= 0x23 {
        return;
    }
    let sheet = read_usize(p + 0x61);
    if read_i16(sheet + 5) > 0x64
        && read_i16(sheet + 0xd) > 0x186a
        && read_i16(sheet + 0xb) as i32 + 0x2ee > read_i16(velho + 0x80) as i32
        && read_i8_as_i32(p + 0x5b) > 0xa
        && read_i8_as_i32(p + 0x59) > 6
        && friendly_random(3) != 0
    {
        let who = squad_spokesman(velho, p);
        if who != 0 {
            if MEDIA_REACTION_DRY_RUN {
                MEDIA_REACTION_STOPPED = true;
                return;
            }
            let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) = core::mem::transmute(FN_MEDIA_QUEUE_SALE_UNREST_STORY);
            f(this, velho, new, p, who);
        }
    }
}


/// news 0xbe5 of the fans or a squad spokesman celebrating the signing, to the club manager
/// Original: `Media::queue_signing_celebration_story` (0x75b6e0, media.cpp, thiscall).
///
/// [pt] { 0x75b6e0 } (thiscall, ret 0x14, media.cpp, quadro SEH da notícia local): `Media::queue_signing_celebration_story(
/// jogador, clube, tipo, a, porta_voz)`, pelas reações à transferência (0x758200). Jogador com +0x1a, clube atual
/// (+0x39) com técnico (+0xcf) e +0x69, ficha (+0x61); tipo 2 sem porta-voz: nada. A notícia 0xbe5: o jogador (id e as
/// três chaves), o id do clube dele, sorteio(2), o veículo (0x757fa0), o comentarista (0x7dab90 com o clube, o veículo e
/// a nação +0x53 ou −1), as posições (0x5b7d20), o tipo, a resposta do técnico da máquina (0x735870 com 0x82; 0 com
/// técnico humano), no tipo 2 o porta-voz (id e chaves), a reputação do clube acima de 7250, o técnico (id e chaves) e
/// `a`; vai ao técnico do clube do jogador (0x7861e0).
pub(crate) unsafe extern "thiscall" fn queue_signing_celebration_story(this: usize, p: usize, clube: usize, kind: u32, a: u32, porta_voz: usize) {
    if p == 0 || read_usize(p + 0x1a) == 0 {
        return;
    }
    let current = read_usize(p + 0x39);
    if current == 0 || read_usize(current + 0xcf) == 0 || read_usize(current + 0x69) == 0 || read_usize(p + 0x61) == 0 {
        return;
    }
    if porta_voz == 0 && kind as u8 == 2 {
        return;
    }
    crate::comparacao::ramo("cqa_13 noticia da comemoracao");
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MEDIA_PICK_OUTLET_TYPE_FOR_PERSON);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(FN_PUNDITLIST_PICK_RANDOM_PUNDIT);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let reply: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_MEDIA_PICK_AI_MANAGER_RESPONSE);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xbe5, 0);
    write(nt, 0, read_usize(p) as u32);
    for k in 1..4usize {
        write(nt, k as i8, read_usize(read_usize(p + 4 * k) + 0x33) as u32);
    }
    write(nt, 4, read_usize(read_usize(p + 0x39)) as u32);
    write(nt, 5, friendly_random(2) as u32);
    let v = veiculo(p);
    write(nt, 6, v);
    let nacao = read_usize(clube + club::NATION);
    write(nt, 7, comentarista(read_usize(G_PUNDIT_LIST), clube, v, if nacao != 0 { read_i32(nacao) } else { -1 }) as u32);
    write(nt, 8, posicoes(read_usize(p + 0x61)) as i16 as i32 as u32);
    write(nt, 9, kind as u8 as i8 as i32 as u32);
    let mut r = 0u32;
    if human(read_usize(p + 0x39), 0, 1, 0, 0) == 0 {
        r = reply(this, read_usize(read_usize(p + 0x39) + 0xcf), p, 0x82);
    }
    write(nt, 10, r);
    let mut k: i8 = 0xb;
    if kind as u8 == 2 {
        write(nt, 0xb, read_usize(porta_voz) as u32);
        for j in 1..4usize {
            write(nt, 0xb + j as i8, read_usize(read_usize(porta_voz + 4 * j) + 0x33) as u32);
        }
        k = 0xf;
    }
    write(nt, k, (read_i16(clube + club::REPUTATION) > 0x1c52) as u32);
    let t = read_usize(read_usize(p + 0x39) + 0xcf);
    write(nt, k + 1, read_usize(t) as u32);
    for j in 1..4usize {
        write(nt, k + 1 + j as i8, read_usize(read_usize(t + 4 * j) + 0x33) as u32);
    }
    // simplification: a original empurra o índice num dword de bytes de cima sem valor; a 0x785790 lê o char
    write(nt, k + 5, a as u8 as i8 as i32 as u32);
    deliver(read_usize(G_NEWS), nt, read_usize(p + 0x39));
}

// simplification: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.

/// news 0xbe4 of the team mate unhappy at the sale, to the old club's manager
/// Original: `Media::queue_sale_unrest_story` (0x75cfc0, media.cpp, thiscall).
///
/// [pt] { 0x75cfc0 } (thiscall, ret 0x10, media.cpp, quadro SEH da notícia local): `Media::queue_sale_unrest_story(velho,
/// novo, jogador, companheiro)`, pelas reações à transferência (0x758200). Velho clube com técnico (+0xcf), fora da
/// seleção (0x53b900), companheiro e jogador com ficha (+0x61). A notícia 0xbe4: o técnico, o companheiro e o jogador
/// (id e as três chaves de cada), os ids dos dois clubes, sorteio(2) (+2 se +0x5b do companheiro não passar de
/// sorteio(5) + 10), o veículo (0x757f30), o comentarista (0x7dab90 com a nação +0x53 ou −1) e 2; vai ao técnico do velho
/// clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn queue_sale_unrest_story(_this: usize, velho: usize, new: usize, p: usize, comp: usize) {
    if velho == 0 || read_usize(velho + 0xcf) == 0 || is_national_team(velho) != 0 {
        return;
    }
    if comp == 0 || read_usize(comp + competition::CLUB_COMP_FOREGROUND_COLOUR) == 0 || p == 0 || read_usize(p + 0x61) == 0 {
        return;
    }
    crate::comparacao::ramo("cqa_13 noticia do desassossego");
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MEDIA_PICK_OUTLET_TYPE_FOR_CLUB);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(FN_PUNDITLIST_PICK_RANDOM_PUNDIT);
    let mut local = [0u32; 0x3a];
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xbe4, 0);
    let mut k: i8 = 0;
    for quem in [read_usize(velho + 0xcf), comp, p] {
        write(nt, k, read_usize(quem) as u32);
        for j in 1..4usize {
            write(nt, k + j as i8, read_usize(read_usize(quem + 4 * j) + 0x33) as u32);
        }
        k += 4;
    }
    write(nt, 0xc, read_usize(velho) as u32);
    write(nt, 0xd, read_usize(new) as u32);
    let r = friendly_random(5);
    let s = if read_i8_as_i32(comp + 0x5b) > r + 10 { friendly_random(2) } else { friendly_random(2) + 2 };
    write(nt, 0xe, s as u32);
    let v = veiculo(velho);
    write(nt, 0xf, v);
    let nacao = read_usize(velho + 0x53);
    write(nt, 0x10, comentarista(read_usize(G_PUNDIT_LIST), velho, v, if nacao != 0 { read_i32(nacao) } else { -1 }) as u32);
    write(nt, 0x11, 2);
    deliver(read_usize(G_NEWS), nt, velho);
}

// simplification: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.






/// True if the date (+4) of a regen queue entry is before today.
///
/// [pt] A data da entrada `e` da fila dos regens (+4) é antes de hoje (0x456770)?
pub(crate) unsafe fn is_regen_entry_due(e: usize) -> bool {
    let previous: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    previous(e + 4, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 // cpw10: a entrada vencida
}

/// One rating of the rebuilt coach (0x7a65d0): max(pow(+0x22, 0x968950) x factor, record value v) x k, capped and
/// truncated.
///
/// [pt] Uma nota do técnico refeito (um dos blocos da 0x7a65d0): P = pow(+0x22 da pessoa, 0x968950) (vezes `fator`, quando
/// há), o maior entre P e o valor `v` da ficha, vezes `k`, no máximo `teto` (f64 em 0x...), truncado.
pub(crate) unsafe fn rebuilt_coach_rating(person: usize, v: i16, factor: Option<usize>, k: usize, ceiling: usize) -> u16 {
    let mut p = game_pow(*((person + staff::INT_APPS) as *const u8) as f64, read_f64_at(0x968950)); // 0x968950 = 2
    if let Some(f) = factor {
        p *= read_f64_at(f);
    }
    let v = v as f64;
    let m = if p > v { p } else { v };
    let mk = m * read_f64_at(k);
    let t = read_f64_at(ceiling);
    match_pl_ftol(if !(mk >= t) { mk } else { t }) as u16
}

/// Places the rebuilt coach at his AI club (0x7a6757..0x7a68d8) as coach in empty coach slots or as assistant, by
/// reputation against the club's.
///
/// [pt] O cargo do técnico refeito no clube da máquina (0x7a6757..0x7a68d8): reputação do jogador (+0xb da ficha) abaixo da
/// do clube (+0x80) vezes 0x968968 e menos de 5 treinadores (0x53c8d0): as vagas de treinador vazias (+0x19f, 5) ficam
/// com ele (cargo 0xf); sem vaga, auxiliar (+0xd3, cargo 0xd) se o clube não tiver (0x53cb80). Senão, abaixo da do clube
/// vezes 0x969040, auxiliar do mesmo jeito.
pub(crate) unsafe fn place_coach_at_ai_club(s: usize) {
    let clube = read_usize(s + 0x39);
    if clube == 0 {
        return;
    }
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let treinadores: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_COACHES);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_ASSISTANT);
    let rep = |k: usize| {
        let f = read_i16(read_usize(s + 0x61) + 0xb) as f64;
        let c = read_i16(read_usize(s + 0x39) + 0x80) as f64 * read_f64_at(k);
        c > f
    };
    if rep(0x968968) && (treinadores(read_usize(s + 0x39)) as i8) < 5 {
        for k in 0..5usize {
            let v = read_usize(s + 0x39) + 0x19f + 4 * k;
            if read_usize(v) == 0 {
                core::ptr::write_unaligned(v as *mut usize, s);
                *((s + 0x3d) as *mut u8) = 0xf;
            }
        }
        if *((s + 0x3d) as *const u8) == 0xf || auxiliar(read_usize(s + 0x39)) != 0 {
            return;
        }
        core::ptr::write_unaligned((read_usize(s + 0x39) + 0xd3) as *mut usize, s);
        *((s + 0x3d) as *mut u8) = 0xd;
        return;
    }
    if !rep(0x969040) || auxiliar(read_usize(s + 0x39)) != 0 {
        return;
    }
    core::ptr::write_unaligned((read_usize(s + 0x39) + 0xd3) as *mut usize, s);
    *((s + 0x3d) as *mut u8) = 0xd;
}



/// Media outlet code for big clubs from a random(100) value (6, 3, 5, 2, 0 or 1) [inference: "veiculo" = media outlet].
///
/// [pt] O veículo pelo sorteio(100) dos grandes (0x757f30/0x757fa0): 0 → 6; até 7 → 3; até 14 → 5; até 39 → 2; até 59 → 0;
/// senão 1 (comparações do al com sinal).
pub(crate) fn big_club_outlet(al: i8) -> u32 {
    if al == 0 {
        6
    } else if al < 8 {
        3
    } else if al < 0xf {
        5
    } else if al < 0x28 {
        2
    } else {
        (al >= 0x3c) as u32
    }
}

/// Media outlet code for smaller clubs: random(4) 0..2 gives 2, else 4 [inference].
///
/// [pt] O veículo dos menores: sorteio(4) de 0 a 2 → 2, senão 4.
pub(crate) unsafe fn small_club_outlet() -> u32 {
    let r = friendly_random(4);
    if (0..=2).contains(&r) {
        2
    } else {
        4
    }
}

/// club reputation above 6250: outlet 0 to 6 from random(100); otherwise 2 or 4 from random(4)
/// Original: `media_pick_outlet_type_for_club` (0x757f30, media.cpp, cdecl).
///
/// [pt] { 0x757f30 } (cdecl, media.cpp): `media_pick_outlet_type_for_club(clube)`: reputação (+0x80) acima de 6250: o
/// veículo dos grandes pelo sorteio(100); senão o dos menores (sorteio(4)). Pelas notícias do troféu (0x75c390) e da venda
/// (0x75cfc0).
pub(crate) unsafe extern "C" fn media_pick_outlet_type_for_club(clube: usize) -> u32 {
    if read_i16(clube + club::REPUTATION) > 0x186a {
        big_club_outlet(friendly_random(100) as u8 as i8)
    } else {
        small_club_outlet()
    }
}


/// the same from the player's reputation (player data +0xb)
/// Original: `media_pick_outlet_type_for_person` (0x757fa0, media.cpp, cdecl).
///
/// [pt] { 0x757fa0 } (cdecl, media.cpp): `media_pick_outlet_type_for_person(pessoa)`: como o do clube, pela reputação do
/// jogador (+0xb da ficha +0x61). Pelas histórias do jogador (0x72c1f0), do desabafo, do elogio, de querer jogar e da
/// comemoração da contratação (0x75b6e0, nas transferências).
pub(crate) unsafe extern "C" fn media_pick_outlet_type_for_person(person: usize) -> u32 {
    if read_i16(read_usize(person + staff::PLAYER) + 0xb) > 0x186a {
        big_club_outlet(friendly_random(100) as u8 as i8)
    } else {
        small_club_outlet()
    }
}







/// The club's row in the manager manager table ([[G_MANAGER_MANAGER]] + id x 0x49; +6 board confidence, +8 fan
/// confidence).
///
/// [pt] A linha do clube no gerente dos técnicos: [[0xb67a44]] + id × 0x49 (+6 confiança da diretoria, +8 da torcida, as duas
/// entre 1 e 10000; +0xe outra palavra).
pub(crate) unsafe fn manager_manager_row(clube: usize) -> usize {
    read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as usize).wrapping_mul(0x49))
}

/// Dampens a confidence change: halved drop / doubled rise for a favoured manager, the reverse for a disliked one;
/// beyond +-1000 the excess counts 0.2.
///
/// [pt] Os amortecedores comuns das duas confianças: com o preferido (`gosta`) a queda vale metade e a alta o dobro; com o
/// rejeitado (`nao_gosta`) o contrário; fora de ±1000, o excesso vale 0,2. Devolve a palavra baixa (o original usa `di`).
pub(crate) unsafe fn damp_confidence_change(v: i32, gosta: bool, not_likes: bool) -> i16 {
    let mut v = v;
    if gosta {
        let x = v as i16 as f64;
        v = if (v as i16) < 0 { match_pl_ftol(x * read_f64_at(0x968968)) } else { match_pl_ftol(x + x) }; // 0x968968 = 0.5
    } else if not_likes {
        let x = v as i16 as f64;
        v = if (v as i16) > 0 { match_pl_ftol(x * read_f64_at(0x968968)) } else { match_pl_ftol(x + x) }; // 0x968968 = 0.5
    }
    let x = v as i16 as i32;
    if x < -1000 {
        v = match_pl_ftol((x + 1000) as f64 * read_f64_at(0x968918) - read_f64_at(0x968908)); // 0x968918 = 0.2, 0x968908 = 1000
    } else if x > 1000 {
        v = match_pl_ftol((x - 1000) as f64 * read_f64_at(0x968918) + read_f64_at(0x968908)); // 0x968918 = 0.2, 0x968908 = 1000
    }
    v as i16
}

/// Fan confidence change (0x738340) toward a target, from the row's +8 and the manager being favoured or disliked by
/// the club.
///
/// [pt] A variação da confiança da torcida (0x738340): conf = +8 da linha (0 sem clube); d = alvo − trunc((conf − 5000) ×
/// −0,05) (16 bits); v = trunc(d × 0,75); o técnico (+0xcf) favorito (0x547790) ou rejeitado (0x5477d0) pelo clube
/// amortece. As duas consultas são puras.
pub(crate) unsafe fn fan_confidence_change(clube: usize, target: u32) -> i16 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let conf = if clube != 0 { read_i16(manager_manager_row(clube) + 8) as i32 } else { 0 };
    let t = match_pl_ftol((conf - 5000) as f64 * read_f64_at(0x96d280)); // 0x96d280 = -0.05
    let d = (target as i32).wrapping_sub(t) as i16;
    let v = match_pl_ftol(d as f64 * read_f64_at(0x969040)); // 0x969040 = 0.75
    let manager = read_usize(clube + club::MANAGER);
    let fav = favorito(manager, clube) != 0;
    let rej = !fav && rejeitado(manager, clube) != 0;
    damp_confidence_change(v, fav, rej)
}

/// Board confidence change (0x7386f0) toward a target, from the row's +6 and the chairman liking or disliking the
/// manager.
///
/// [pt] A variação da confiança da diretoria (0x7386f0): conf = +6 da linha (0 sem clube); s = (conf − 6000) / 20 + alvo
/// (16 bits); v = trunc(s × 0,75); o presidente (+0xbf) gosta (0x547810) ou não gosta (0x547840) do técnico (+0xcf).
pub(crate) unsafe fn board_confidence_change(clube: usize, target: u32) -> i16 {
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let conf = if clube != 0 { read_i16(manager_manager_row(clube) + 6) as i32 } else { 0 };
    let s = ((conf - 6000) / 20).wrapping_add(target as i32) as i16;
    let v = match_pl_ftol(s as f64 * read_f64_at(0x969040)); // 0x969040 = 0.75
    let (tecnico, presidente) = (read_usize(clube + club::MANAGER), read_usize(clube + club::CHAIRMAN));
    let g = gosta(tecnico, presidente) != 0;
    let n = !g && not_likes(tecnico, presidente) != 0;
    damp_confidence_change(v, g, n)
}

/// Adds v to a confidence word, clamped to 1..10000.
///
/// [pt] Soma `v` na palavra da linha, entre 1 e 10000.
pub(crate) unsafe fn add_clamped_confidence(p: usize, v: i16) {
    let x = read_i16(p).wrapping_add(v);
    core::ptr::write_unaligned(p as *mut i16, x.clamp(1, 10000));
}



/// adds to the supporters' confidence (+8 of the club row in [[0xb67a44]], 1..10000) the target minus the current
/// confidence, × 0.75, damped by a favoured or rejected manager and beyond ±1000; beyond ±500, news 0xbe0 to the
/// manager
/// Original: `Media::adjust_supporter_confidence` (0x738340, media.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x738340 } (thiscall, ret 0x14, media.cpp, quadro SEH sem destrutor): `Media::adjust_supporter_confidence(clube,
/// sujeito, tópico, alvo, variante)`: soma na confiança da torcida (+8 da linha do clube, entre 1 e 10000) a variação
/// (ver `fan_confidence_change`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0): 0 = 0, 1 =
/// tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750, 4 negativa, 3 até 750, 2
/// acima), 6..9 o técnico; com sujeito, 10..13 o sujeito e 14 = 6 favorito, 7 rejeitado, 8 nenhum; depois −1, a
/// confiança nova e 0.
pub(crate) unsafe extern "thiscall" fn adjust_supporter_confidence(_this: usize, clube: usize, sujeito: usize, topico: u32, target: u32, variante: u32) {
    let v = fan_confidence_change(clube, target);
    crate::comparacao::ramo("cpo_5 goal_crowd_reaction");
    if clube != 0 {
        add_clamped_confidence(manager_manager_row(clube) + 8, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbe0, 0);
    write(n, 0, 0);
    write(n, 1, topico as i16 as i32 as u32);
    write(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    write(n, 3, s as u32);
    write(n, 4, read_usize(clube) as u32);
    let grau = if v < -750 { 5 } else if v < 0 { 4 } else if v > 750 { 2 } else { 3 };
    write(n, 5, grau);
    set_person_name_slots(write, n, 6, read_usize(clube + club::MANAGER));
    let b: i8 = if sujeito != 0 {
        set_person_name_slots(write, n, 10, sujeito);
        let r = if favorito(sujeito, clube) != 0 {
            6
        } else if rejeitado(sujeito, clube) != 0 {
            7
        } else {
            8
        };
        write(n, 14, r);
        0xf
    } else {
        write(n, 10, u32::MAX);
        0xb
    };
    write(n, b, u32::MAX);
    let conf = if clube != 0 { read_i16(manager_manager_row(clube) + 8) as i32 } else { 0 };
    write(n, b + 1, conf as u32);
    write(n, b + 2, 0);
    deliver(read_usize(G_NEWS), n, clube);
}

/// adds to the board's confidence (+6 of the club row) (confidence − 6000)/20 + target, × 0.75, damped by a chairman
/// who likes or dislikes the manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager with the subject's flags
/// Original: `Media::adjust_board_confidence` (0x7386f0, media.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x7386f0 } (thiscall, ret 0x18, media.cpp, quadro SEH sem destrutor): `Media::adjust_board_confidence(clube,
/// sujeito, tópico, alvo, variante, sexto)`: soma na confiança da diretoria (+6 da linha do clube, entre 1 e 10000) a
/// variação (ver `board_confidence_change`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0):
/// 0 = 1, 1 = tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750 com confiança abaixo
/// de 4500; 9 negativa com confiança acima de 6250, senão 4; 2 acima de 750 com confiança acima de 6250, senão 3), 6..9
/// o técnico; com sujeito 10..13 e 14 = 6/7/8 (o presidente gosta, não gosta, nenhum); depois o presidente (ou −1), a
/// confiança nova, o sexto (com ele 1 e a palavra +0xe), e as marcas do sujeito: contrato 0x10/0x20 acima de 25 anos,
/// média das notas acima da média da liga × 5, +0x59 e +0x5b acima de 14, mais de 1500 dias no clube, valor (0x4d0c60)
/// acima de 4.500.000 e reputação (+5 da ficha) acima de 140.
pub(crate) unsafe extern "thiscall" fn adjust_board_confidence(_this: usize, clube: usize, sujeito: usize, topico: u32, target: u32, variante: u32, sexto: u32) {
    let v = board_confidence_change(clube, target);
    crate::comparacao::ramo("cpo_5 diretoria");
    if clube != 0 {
        add_clamped_confidence(manager_manager_row(clube) + 6, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    crate::comparacao::ramo("cpo_5 diretoria noticia");
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let ratings_average: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_PLAYERSTATS_GET_RECENT_FORM);
    let days_until: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let conf = || if clube != 0 { read_i16(manager_manager_row(clube) + 6) } else { 0 };
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbe0, 0);
    write(n, 0, 1);
    write(n, 1, topico as i16 as i32 as u32);
    write(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    write(n, 3, s as u32);
    write(n, 4, read_usize(clube) as u32);
    let grau = if v < -750 && (clube == 0 || conf() < 4500) {
        5
    } else if v < 0 {
        if clube != 0 && conf() > 6250 { 9 } else { 4 }
    } else if v > 750 && clube != 0 && conf() > 6250 {
        2
    } else {
        3
    };
    write(n, 5, grau);
    set_person_name_slots(write, n, 6, read_usize(clube + club::MANAGER));
    let mut b: i8 = if sujeito != 0 {
        set_person_name_slots(write, n, 10, sujeito);
        let presidente = read_usize(clube + club::CHAIRMAN);
        let r = if gosta(sujeito, presidente) != 0 {
            6
        } else if not_likes(sujeito, presidente) != 0 {
            7
        } else {
            8
        };
        write(n, 14, r);
        0xf
    } else {
        write(n, 10, u32::MAX);
        0xb
    };
    let presidente = read_usize(clube + club::CHAIRMAN);
    if presidente != 0 {
        set_person_name_slots(write, n, b, presidente);
        b += 4;
    } else {
        write(n, b, u32::MAX);
        b += 1;
    }
    write(n, b, conf() as i32 as u32);
    b += 1;
    if sexto != 0 {
        write(n, b, 1);
        let w = if clube != 0 { read_i16(manager_manager_row(clube) + 0xe) } else { 0 };
        write(n, b + 1, w as i32 as u32);
        b += 2;
    } else {
        write(n, b, 0);
        b += 1;
    }
    // as marcas do sujeito
    let mut m = 0u32;
    if sujeito != 0 && read_usize(sujeito + 0x61) != 0 {
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), sujeito);
        if c != 0 && read_i8_as_i32(sujeito + 0x18) > 0x19 {
            let t = *((c + 0x4f) as *const u8) & 0xf0;
            if t == 0x20 || t == 0x10 {
                m = 1;
            }
        }
    }
    write(n, b, m);
    b += 1;
    let mut m = 0u32;
    if sujeito != 0 && read_usize(sujeito + 0x61) != 0 {
        let base = read_usize(G_PLAYER_STATS);
        let notas = ratings_average(base, read_usize(sujeito + 0x61)) as i8 as f64;
        let base = read_usize(G_PLAYER_STATS);
        let y = read_f64_at(base + 0x18);
        let c0 = read_f64_at(0x9689a0); // 0x9689a0 = 0
        let average = if !(y < c0 || y > c0) { read_f64_at(0x968998) } else { read_f64_at(base + 0x10) / y }; // sem jogos: 6,8; 0x968998 = 6.8
        if notas > average * read_f64_at(0x9688a0) { // 0x9688a0 = 5
            m = 1;
        }
    }
    write(n, b, m);
    b += 1;
    write(n, b, (sujeito != 0 && read_i8_as_i32(sujeito + 0x59) > 0xe) as u32);
    b += 1;
    write(n, b, (sujeito != 0 && read_i8_as_i32(sujeito + 0x5b) > 0xe) as u32);
    b += 1;
    let days = sujeito != 0 && days_until(G_CURRENT_DATE, read_usize(sujeito + 0x3e) as u32, read_usize(sujeito + 0x42) as u32) > 0x5dc;
    write(n, b, days as u32);
    b += 1;
    let caro = sujeito != 0 && value(read_usize(G_CONTRACT_MANAGER), sujeito, clube, 0, u32::MAX, 0) > 0x44aa20;
    write(n, b, caro as u32);
    b += 1;
    let injury_word_0c = sujeito != 0 && read_usize(sujeito + 0x61) != 0 && read_i16(read_usize(sujeito + 0x61) + 5) > 0x8c;
    write(n, b, injury_word_0c as u32);
    deliver(read_usize(G_NEWS), n, clube);
}







pub(crate) type NewsWriteFn = unsafe extern "thiscall" fn(usize, i8, u32) -> u32;

/// Writes a person's id and three keys (+0x33 of +4, +8, +0xc) into news slots from k; the +0xc one may be missing (-1)
/// when optional.
///
/// [pt] Os dados da pessoa a partir de `k`: id e as três chaves +0x33 de +4, +8 e +0xc (a de +0xc pode faltar: −1, só
/// quando `opcional`).
pub(crate) unsafe fn set_person_slots_optional(write: NewsWriteFn, n: usize, k: i8, p: usize, opcional: bool) {
    write(n, k, read_usize(p) as u32);
    write(n, k + 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    write(n, k + 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    let q = read_usize(p + 0xc);
    write(n, k + 3, if opcional && q == 0 { u32::MAX } else { read_usize(q + 0x33) as u32 });
}

/// Returns the NEWSITEM constructor (0x77bbe0) and set_slot (0x785790) function pointers.
///
/// [pt] A dupla de transmutes comum: construir a notícia (0x77bbe0) e gravar um dado (0x785790).
pub(crate) unsafe fn news_item_fns() -> (unsafe extern "thiscall" fn(usize, u32, u32) -> usize, NewsWriteFn) {
    (core::mem::transmute(FN_NEWSITEM_NEWSITEM), core::mem::transmute(FN_NEWSITEM_SET_SLOT))
}

/// Days from the person's date at +0x3e/+0x42 to today (Date days_since).
///
/// [pt] Os dias desde a chegada da pessoa (+0x3e/+0x42) até hoje (0x54cdb0 em 0xae2c90).
pub(crate) unsafe fn days_since_arrival(p: usize) -> i32 {
    let days_until: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    days_until(G_CURRENT_DATE, read_usize(p + 0x3e) as u32, read_usize(p + 0x42) as u32)
}

/// O sorteio da gratidão pelo tipo (índices da tabela de bytes 0x732fcc, saltos 0x732fb4): 0 os tipos 0, 6, 14 e 17; 1
/// os 2 e 30 (sorteio(2)); 2 os 4, 9, 33, 34 e 37 (sorteio(3)); 3 os 5 e 7 (sorteio(4)); o resto 0.
pub(crate) const GRATITUDE_DRAW_CASE: [u8; 0x26] = [0, 5, 1, 5, 2, 3, 0, 3, 5, 2, 5, 5, 5, 4, 0, 5, 5, 0, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 2, 2, 4, 5, 2];

/// news 0xbda to the manager: the player, the club, the type (0x12/0x21 by a draw for a player on the media list or
/// with a flagged contract, 0xc when injured), the variant, the type's draw (which may raise +0x44 of the player
/// record), the manager and the assistant
/// Original: `Media::queue_player_gratitude_news` (0x732c80, media.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x732c80 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::queue_player_gratitude_news(jogador,
/// clube, tipo, variante)`: com jogador com ficha (+0x61), clube com técnico (+0xcf) e o contrato dele com o clube
/// (0x4cf5f0), a notícia 0xbda (local) ao técnico (0x7861e0): 0..3 o jogador (3 = −1 sem +0xc), 4 = id do clube, 5 =
/// o tipo, 6 = a variante, 7 = o sorteio do tipo, 8..0xb o técnico, 0xc = 0x420e9, 0xd..0x10 o auxiliar (+0xd3) ou −1.
/// O tipo: se o jogador está na lista da mídia (this +0/+8, 0x17 bytes, id em +8 e marca 2 em +0x10) ou o contrato tem
/// 0x2000 ou 8 em +0x45, os tipos 0, 2, 4, 7, 9, 0xe e 0x23 viram 0x12 (+0x5d ≥ sorteio(10)) ou 0x21; lesionado
/// (0x62ed70): 0xc. Nos tipos 0, 6, 14 e 17: com +0x44 da ficha abaixo de 10, sorteio(3) + 2; senão sorteio(5), e 0 ou 1
/// sobe +0x44 da ficha (até 20).
pub(crate) unsafe extern "thiscall" fn queue_player_gratitude_news(this: usize, player: usize, clube: usize, kind: u32, variante: u32) {
    if player == 0 || read_usize(player + 0x61) == 0 || clube == 0 || read_usize(clube + club::MANAGER) == 0 {
        return;
    }
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = contract_with_club(read_usize(G_CONTRACT_MANAGER), player, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 gratidao");
    let (construir, gravar) = news_item_fns();
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let lesionado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbda, 0);
    set_person_slots_optional(gravar, n, 0, player, true);
    gravar(n, 4, read_usize(clube) as u32);
    // a lista da mídia (this +0, conta em +8)
    let mut in_list = false;
    for i in 0..core::ptr::read_unaligned((this + 8) as *const i32).max(0) as usize {
        let e = read_usize(this) + i * 0x17;
        if read_usize(e + 8) == read_usize(player) && *((e + 0x10) as *const u8) == 2 {
            in_list = true;
            break;
        }
    }
    let f45 = read_usize(contrato + 0x45);
    let mut t = kind;
    if in_list || f45 & 0x2000 != 0 || f45 & 8 != 0 {
        if [0xe, 0, 2, 9, 0x23, 4, 7].contains(&(kind as i16)) {
            crate::comparacao::ramo("cpo_7 gratidao tipo");
            let s = sorteio(10);
            t = if read_i8_as_i32(player + 0x5d) >= s { 0x12 } else { 0x21 };
        }
    }
    if lesionado(read_usize(G_INJURY), player) != 0 {
        t = 0xc;
    }
    let t = t as i16 as i32;
    gravar(n, 5, t as u32);
    gravar(n, 6, variante as i8 as i32 as u32);
    let v = match if (t as u32) <= 0x25 { GRATITUDE_DRAW_CASE[t as usize] } else { 5 } {
        0 => {
            let sheet = read_usize(player + 0x61);
            if sheet != 0 && read_i8_as_i32(sheet + 0x44) < 10 {
                (sorteio(3) as u8).wrapping_add(2) as i8 as i32
            } else {
                let s = sorteio(5);
                if s == 0 || s == 1 {
                    let sheet = read_usize(player + 0x61);
                    if sheet != 0 && read_i8_as_i32(sheet + 0x44) < 0x14 {
                        crate::comparacao::ramo("cpo_7 gratidao sobe");
                        *((sheet + 0x44) as *mut u8) += 1;
                    }
                }
                s
            }
        }
        1 => sorteio(2),
        2 => {
            crate::comparacao::ramo("cpo_7 gratidao sorteio 3");
            sorteio(3)
        }
        3 => sorteio(4),
        _ => 0,
    };
    gravar(n, 7, v as u32);
    let manager = read_usize(clube + club::MANAGER);
    set_person_slots_optional(gravar, n, 8, manager, false);
    gravar(n, 0xc, 0x420e9);
    let auxiliar = read_usize(clube + club::ASSISTANT_MANAGER);
    if auxiliar != 0 {
        set_person_slots_optional(gravar, n, 0xd, auxiliar, false);
    } else {
        gravar(n, 0xd, u32::MAX);
    }
    deliver(read_usize(G_NEWS), n, clube);
}

/// for a human club, news 0xbcf to the manager: the player, the club, the reputation band and the request level by
/// attributes, the type, the fourth argument and whether he is transfer-listed
/// Original: `Media::post_player_backing_request_news` (0x743370, media.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x743370 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::post_player_backing_request_news(
/// jogador, clube, tipo, quarto)`: com jogador com ficha (+0x61) e clube humano (0x600a50), a notícia 0xbcf (local) ao
/// técnico do clube (0x7861e0): 0..3 o jogador (3 = −1 sem +0xc), 4 = id do clube, 5/6 = (1, ·) com reputação (+0xb da
/// ficha) acima de 4250, senão (2, ·): o segundo 1 se (+0x57 > 15 e +0x5d < 6, ou +0x5b < 10 e +0x59 < 6 na faixa de
/// baixo), 3 se o tipo é 0x20, 1 ou 0x1e com +0x59 > 15, +0x5b > 10 e mais de 210 dias no clube, senão 2; 7 = tipo, 8 =
/// quarto, 9 = listado para transferência (0x544520).
pub(crate) unsafe extern "thiscall" fn post_player_backing_request_news(_this: usize, player: usize, clube: usize, kind: u32, quarto: u32) {
    if player == 0 || read_usize(player + 0x61) == 0 || clube == 0 {
        return;
    }
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human_club(clube, 0, 1, 0, 0) == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 pedido de apoio");
    let (construir, gravar) = news_item_fns();
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbcf, 0);
    set_person_slots_optional(gravar, n, 0, player, true);
    gravar(n, 4, read_usize(clube) as u32);
    let t = kind as u8;
    let target = t == 0x20 || t == 1 || t == 0x1e;
    let (a, b): (i32, i32) = if read_i16(read_usize(player + 0x61) + 0xb) > 0x109a {
        if read_i8_as_i32(player + 0x57) > 0xf && read_i8_as_i32(player + 0x5d) < 6 {
            (1, 1)
        } else if target && read_i8_as_i32(player + 0x59) > 0xf && read_i8_as_i32(player + 0x5b) > 0xa && days_since_arrival(player) > 0xd2 {
            (1, 3)
        } else {
            (1, 2)
        }
    } else if read_i8_as_i32(player + 0x5b) < 0xa && read_i8_as_i32(player + 0x59) < 6 {
        (2, 1)
    } else if target && read_i8_as_i32(player + 0x59) > 0xf && read_i8_as_i32(player + 0x5b) > 0xa && days_since_arrival(player) > 0xd2 {
        (2, 3)
    } else {
        (2, 2)
    };
    gravar(n, 5, a as u32);
    gravar(n, 6, b as u32);
    gravar(n, 7, t as i8 as i32 as u32);
    gravar(n, 8, quarto as i16 as i32 as u32);
    gravar(n, 9, (listado(player, 0) != 0) as u32);
    deliver(read_usize(G_NEWS), n, clube);
}

/// news 0xbdc: the player, the club, the reaction (0..4 by reputation, contract category, age, attributes and the
/// response), the response, board confidence below 4000, the topic and the manager, delivered to the managers of the
/// interested clubs (0x8d1da0)
/// Original: `Media::post_player_criticism_reaction_news` (0x743a50, media.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x743a50 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::post_player_criticism_reaction_news(
/// jogador, clube, tópico, resposta)`: com jogador com ficha, clube com técnico e o contrato (0x4cf5f0), a notícia 0xbdc
/// (local): 0..3 o jogador, 4 = id do clube, 5 = a reação (com reputação acima de 4250 e da do clube, categoria
/// 0x4ca470 1 ou 2 e mais de 22 anos: 1 ou 3 se +0x59 < 10, +0x57 > 15 e +0x5d < 6; 2 ou 4 com mais de 1000 dias, +0x59
/// > 15 e sem 0x181 em +0x45 do contrato; o maior quando o contrato tem 0x2000 ou 8, +0x5b < 10, +0x5d > 10, +0x59 < 15,
/// mais de 23 anos e a resposta é 2; senão 0), 6 = resposta, 7 = confiança da diretoria abaixo de 4000, 8 = tópico,
/// 9..0xc o técnico. Vai aos técnicos dos clubes que a 0x8d1da0 ([0xde1f54], 0x21a, até 30) devolve.
pub(crate) unsafe extern "thiscall" fn post_player_criticism_reaction_news(_this: usize, player: usize, clube: usize, topico: u32, reply: u32) {
    if player == 0 || read_usize(player + 0x61) == 0 || clube == 0 || read_usize(clube + club::MANAGER) == 0 {
        return;
    }
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = contract_with_club(read_usize(G_CONTRACT_MANAGER), player, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 reacao a critica");
    let (construir, gravar) = news_item_fns();
    let category: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    let clubes_interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdc, 0);
    let f45 = read_usize(contrato + 0x45);
    let mark = (f45 & 0x2000 != 0 || f45 & 8 != 0)
        && read_i8_as_i32(player + 0x5b) < 0xa
        && read_i8_as_i32(player + 0x5d) > 0xa
        && read_i8_as_i32(player + 0x59) < 0xf
        && read_i8_as_i32(player + 0x18) > 0x17;
    set_person_slots_optional(gravar, n, 0, player, true);
    gravar(n, 4, read_usize(clube) as u32);
    let r = reply as u8;
    let rep = read_i16(read_usize(player + 0x61) + 0xb);
    let mut reacao = 0i8;
    if rep > 0x109a && rep > read_i16(clube + club::REPUTATION) {
        let manager = read_usize(G_CONTRACT_MANAGER);
        let cat_ok = category(manager, player, contrato) == 1 || category(manager, player, contrato) == 2;
        if cat_ok && read_i8_as_i32(player + 0x18) > 0x16 {
            if read_i8_as_i32(player + 0x59) < 0xa && read_i8_as_i32(player + 0x57) > 0xf && read_i8_as_i32(player + 0x5d) < 6 {
                reacao = if mark && r == 2 { 3 } else { 1 };
            } else if days_since_arrival(player) > 0x3e8 && read_i8_as_i32(player + 0x59) > 0xf && read_usize(contrato + 0x45) & 0x181 == 0 {
                reacao = if mark && r == 2 { 4 } else { 2 };
            }
        }
    }
    gravar(n, 5, reacao as i32 as u32);
    gravar(n, 6, r as i8 as i32 as u32);
    let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_usize(clube)).wrapping_mul(0x49));
    gravar(n, 7, (read_i16(row + 6) < 0xfa0) as u32); // confiança da diretoria abaixo de 4000
    gravar(n, 8, topico);
    set_person_slots_optional(gravar, n, 9, read_usize(clube + club::MANAGER), false);
    let mut clubs = [0usize; 0x1e];
    let tm = read_usize(G_TRANSFER_MANAGER);
    let k = if tm != 0 { clubes_interessados(tm, player, 0x21a, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for i in 0..k.max(0) as usize {
        deliver(read_usize(G_NEWS), n, clubs[i]);
    }
}








/// A notícia guardada na caixa: 0xdf bytes (+0xd6 id).
pub(crate) const MEDIA_NEWS_ITEM_SIZE: usize = 0xdf;

/// Player reputation (min 2000) over club reputation (clamped 2000..7500).
///
/// [pt] A razão entre a reputação do jogador (+0xb do registro +0x61, no mínimo 2000) e a do clube (+0x80, entre 2000 e
/// 7500), como o fild/fidiv do original.
pub(crate) unsafe fn player_club_reputation_ratio(player: usize, clube: usize) -> f64 {
    let a = read_i16(read_usize(player + 0x61) + 0xb);
    let a = if a < 2000 { 2000 } else { a as i32 };
    let b = read_i16(clube + club::REPUTATION);
    let b = if b < 2000 { 2000 } else if b > 7500 { 7500 } else { b as i32 };
    a as f64 / b as f64
}

/// Maps a story type (0..0x52) to the original's switch case (byte table at 0x736b8c; 12 = rating unchanged).
///
/// [pt] O tipo de história (0..0x52) → o caso do original (a tabela de bytes de 0x736b8c; 12 = a nota como está).
pub(crate) fn story_type_case(kind: i16) -> u8 {
    match kind {
        0 => 0,
        1 | 2 | 9 => 1,
        3 => 2,
        4 => 3,
        6 => 4,
        7 => 5,
        12 => 6,
        13 => 7,
        17 => 8,
        30 => 9,
        35 => 10,
        82 => 11,
        _ => 12,
    }
}

/// the player's rating for a story kind: stats and the average of ratings against the league average, the weight, the
/// games and the reputation; then, by kind, the contract, age, days at the club and attributes
/// Original: `Media::rate_player_form_for_story` (0x736720, media.cpp, thiscall, ret 0x18, st0).
///
/// [pt] { 0x736720 } (thiscall, ret 0x18, media.cpp): `Media::rate_player_form_for_story(jogador, clube, contrato, tipo,
/// modo, peso)`: a nota (x87, em st0) do jogador para uma história da mídia. Sem contrato, o do jogador com o clube
/// (0x4cf5f0); sem nenhum, 0. Base: a nota média da liga x ([0xb74350] +0x10 / +0x18, sem jogos 6,8), as estatísticas
/// do jogador (0x7c1e30, categoria 6: s1 campo 1, s2 campo 0x11, s3 campo 2, padrão 6,5) e a média das notas
/// (0x7c1950) × 0,2: r = (s2 − x) + (média − x) − peso/60; com s1 < 12, r × s1/12; negativa, × a razão das
/// reputações. Depois, pelo tipo (0..0x52), somas do tipo de contrato (+0x4f >> 4), da idade, dos dias de casa
/// (0x54cdb0), de atributos do registro +0x61, do peso; o tipo 0x52 dá r × 0,75 + s3/max(s1, 1) − 0,4.
pub(crate) unsafe extern "thiscall" fn rate_player_form_for_story(_this: usize, player: usize, clube: usize, contrato: usize, kind: u32, mode: u32, weight: f32) -> f64 {
    let mut contrato = contrato;
    if contrato == 0 {
        let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contrato = with_club(read_usize(G_CONTRACT_MANAGER), player, clube);
        if contrato == 0 {
            return 0.0;
        }
    }
    let stats: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let media_notas: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYERSTATS_GET_RECENT_FORM);
    let g = read_usize(G_PLAYER_STATS);
    let games = core::ptr::read_unaligned((g + 0x18) as *const f64);
    let x = if games == 0.0 || games.is_nan() { 6.8 } else { core::ptr::read_unaligned((g + 0x10) as *const f64) / games };
    let xf = x as f32;
    let s1f = stats(read_usize(G_PLAYER_STATS), read_usize(player + 0x61), 6, 1, 6.5) as f32;
    let s2f = stats(read_usize(G_PLAYER_STATS), read_usize(player + 0x61), 6, 0x11, 6.5) as f32;
    let average = media_notas(read_usize(G_PLAYER_STATS), read_usize(player + 0x61)) as u8 as i8 as i32;
    let mut l0 = (average as f64 * 0.2) as f32;
    let y = if s1f as f64 > 1.0 { s1f as f64 } else { 1.0 }; // as partidas, no mínimo 1
    let s3 = stats(read_usize(G_PLAYER_STATS), read_usize(player + 0x61), 6, 2, 6.5);
    let l2 = (s3 / y) as f32;
    l0 = (l0 as f64 - xf as f64) as f32;
    let dif = s2f as f64 - xf as f64;
    let dif_f = dif as f32;
    crate::comparacao::ramo("cpn_3 nota do jogador");
    let mut r = dif + l0 as f64 - weight as f64 * (1.0f32 / 60.0) as f64; // o peso da história
    if !(s1f as f64 >= 12.0) {
        crate::comparacao::ramo("cpn_3 poucas partidas");
        r = r * s1f as f64 * (1.0f32 / 12.0) as f64; // menos de 12 partidas
    }
    if !(r >= 0.0) {
        crate::comparacao::ramo("cpn_3 nota negativa");
        r = r * player_club_reputation_ratio(player, clube); // a negativa pela reputação
    }
    let rf = r as f32;
    let kind = kind as u16 as i16;
    if !(0..=0x52).contains(&kind) {
        return r;
    }
    let c = *((contrato + contract::SQUAD_STATUS) as *const u8) >> 4; // o tipo de contrato
    match story_type_case(kind) {
        0 => match c {
            1 => r - 0.5,
            2 => r - 0.35f32 as f64,
            3 => r - 0.2f32 as f64,
            _ => r + 0.25,
        },
        1 => {
            let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            let d = days(G_CURRENT_DATE, read_usize(player + 0x3e) as u32, read_usize(player + 0x42) as u32);
            (200i32.wrapping_sub(d)) as f64 * 0.1f32 as f64 + rf as f64
        }
        2 => {
            let r2 = r + dif_f as f64;
            match c {
                1 => r2 + 2.0,
                2 => r2 + 1.5,
                3 => r2 + 1.0,
                _ => r2,
            }
        }
        3 => r + (0x23 - read_i8(player + 0x18) as i32) as f64 * (1.0f32 / 3.0) as f64,
        4 => r + (10 - read_i8(read_usize(player + 0x61) + 0x25) as i32) as f64 * 0.2f32 as f64,
        5 => r + (read_i8(player + 0x18) as i32 - 0x13) as f64 * (1.0f32 / 3.0) as f64,
        6 => r + (read_i8(read_usize(player + 0x61) + 0x2f) as i32 - 10) as f64 * (1.0f32 / 3.0) as f64,
        7 => r + l0 as f64,
        8 => {
            let r2 = if r > 0.0 { r / player_club_reputation_ratio(player, clube) } else { r };
            match c {
                1 => r2 - 0.5,
                2 => r2 - 0.35f32 as f64,
                3 => r2 - 0.2f32 as f64,
                _ => r2,
            }
        }
        9 => r - weight as f64 * 0.1f32 as f64,
        10 => match c {
            0 => r - 1.0,
            1 => r + 2.0,
            2 => r + 1.5,
            3 => r + 1.0,
            7 => {
                if mode as u8 == 3 { 0.0 } else { r - 3.0 }
            }
            _ => r,
        },
        11 => r * 0.75 + l2 as f64 - 0.4f32 as f64,
        _ => r,
    }
}

/// the headline or body of the club in financial trouble story, by severity, with the club name linked
/// Original: `Media::write_financial_trouble_news_text` (0x74af60, media.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x74af60 } (thiscall, ret 0xc, media.cpp): `Media::write_financial_trouble_news_text(noticia, corpo, saida)`: o
/// título ou o corpo da notícia do clube em crise: lê os campos 0 (o clube), 1 (a gravidade) e 2 da notícia (0x785830),
/// monta em 0xde1f64 (0x66f4e0) o texto pela gravidade (> 0x1a5e, > 0xcb2 no corpo; > 0x1a5e no título) com o nome do
/// clube (+0x38) e o gênero (+0x52), põe na `saida` (0x4ae660) e liga o nome ao clube (0x4ae8a0, tipo 0x7d5).
pub(crate) unsafe extern "thiscall" fn write_financial_trouble_news_text(_this: usize, noticia: usize, corpo: u32, out: usize) {
    let field: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let by_text: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TEXT_SET_STRING);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TEXT_ADD_LINK);
    crate::comparacao::ramo("cpn_3 texto da crise financeira");
    let id = field(noticia, 0);
    let gravidade = field(noticia, 1);
    field(noticia, 2);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
    let name = clube + club::SHORT_NAME;
    let genero = read_i8(clube + club::SHORT_GENDER_NAME) as i32;
    let modelo = if corpo != 0 {
        if gravidade > 0x1a5e {
            0xa436fc
        } else if gravidade > 0xcb2 {
            0xa43644
        } else {
            0xa43594
        }
    } else if gravidade > 0x1a5e {
        0xa43560 // "... financial mess"
    } else {
        0xa43528 // "Financial trouble at ..."
    };
    format(0xde1f64, modelo, genero, genero, name);
    by_text(out, 0xde1f64);
    ligar(out, name, 0x7d5, clube);
}








/// True if date a is before date b (year word unsigned, then day word signed).
///
/// [pt] A data `a` é anterior à `b`? Como a original: a palavra +2 sem sinal e, igual, a palavra +0 com sinal.
pub(crate) unsafe fn media_date_before(a: usize, b: usize) -> bool {
    let (x, y) = (read_u16(a + 2), read_u16(b + 2));
    if x != y {
        return x < y;
    }
    (read_u16(a) as i16) < (read_u16(b) as i16)
}

/// The record's id (+0), or -1 when null.
///
/// [pt] O id (+0) do sujeito, ou −1 sem ele.
pub(crate) unsafe fn media_id_or_minus_one(p: usize) -> usize {
    if p != 0 { read_usize(p) } else { usize::MAX }
}

/// Builds the 23-byte record 0x757a50 stores (today's date, two ids, response, subtype, type).
///
/// [pt] Os 23 bytes que a 0x757a50 grava no registro (data de hoje [0xae2c90], ids, resposta, subtipo, tipo).
pub(crate) unsafe fn build_news_log_record(a: usize, b: usize, kind: u32, subtipo: u32, reply: u32) -> [u8; 0x17] {
    let mut r = [0u8; 0x17];
    r[..8].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    r[8..0xc].copy_from_slice(&(media_id_or_minus_one(a) as u32).to_le_bytes());
    r[0xc..0x10].copy_from_slice(&(media_id_or_minus_one(b) as u32).to_le_bytes());
    r[0x10] = reply as u8;
    r[0x11..0x13].copy_from_slice(&(subtipo as u8 as i8 as i16).to_le_bytes());
    r[0x13..].copy_from_slice(&kind.to_le_bytes());
    r
}

/// qsorts the 23-byte records with the comparer 0x757ee0.
pub(crate) unsafe fn sort_news_log(base: usize, n: usize) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(base, n as u32, 0x17, 0x757ee0);
}

/// appends the story (today, subject ids or −1, type, subtype, response) and sorts again (0x757ee0); grows by 20
/// Original: `Media::log_story` (0x757a50, media.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x757a50 } (thiscall, ret 0x14, media.cpp): `Media::log_story(a, b, tipo, subtipo, resposta)`: acrescenta ao
/// registro de histórias ([this], capacidade +4, conta +8; registros de 23 bytes) a data de hoje (+0, 0x417c10 de
/// [0xae2c90]), o id de `a` (+8) e o de `b` (+0xc) (−1 sem eles), o `tipo` (+0x13), o `subtipo` (byte com sinal, em
/// palavra +0x11) e a `resposta` (+0x10); a conta sobe e o registro é reordenado (qsort 0x945333 com 0x757ee0). Sem
/// registro, aloca 20 (0x944e46; sem memória, caixa 0x6e11 e fim); cheio, cresce 20 (realloc 0x945667; caixa 0x6e1c).
pub(crate) unsafe extern "thiscall" fn log_story(this: usize, a: usize, b: usize, kind: u32, subtipo: u32, reply: u32) {
    crate::comparacao::ramo("cpl_6 registrar historia");
    if read_usize(this) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let p = alocar(0x1cc);
        write_usize(this, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x6e11);
            fatal_exit();
        }
        write_usize(this + 4, 0x14);
    }
    let cap = read_usize(this + 4);
    if cap == read_usize(this + 8) {
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_usize(this), cap.wrapping_add(0x14).wrapping_mul(0x17) as u32);
        write_usize(this, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x6e1c);
            fatal_exit();
        }
        write_usize(this + 4, read_usize(this + 4).wrapping_add(0x14));
    }
    let item = || read_usize(this).wrapping_add(read_usize(this + 8).wrapping_mul(0x17));
    write_usize(item() + 8, media_id_or_minus_one(a));
    write_usize(item() + 0xc, media_id_or_minus_one(b));
    copy_record_date(item(), G_CURRENT_DATE);
    write_usize(item() + 0x13, kind as usize);
    core::ptr::write_unaligned((item() + 0x11) as *mut i16, subtipo as u8 as i8 as i16);
    *((item() + 0x10) as *mut u8) = reply as u8;
    let n = read_usize(this + 8).wrapping_add(1);
    write_usize(this + 8, n); // mais uma história
    sort_news_log(read_usize(this), n);
}

/// removes stories older than 100 days (team ones, 0xbdf, older than 40 + random(40)) moving the last one into the gap;
/// sorts again
/// Original: `Media::expire_story_log` (0x757c70, media.cpp, thiscall, ret 0).
///
/// [pt] { 0x757c70 } (thiscall, ret 0, media.cpp): `Media::expire_story_log()`: cada história do registro anterior ao
/// limite (hoje − 100 dias; a do time, tipo 0xbdf, hoje − (40 + sorteio(40)), um sorteio 0x90cfc0 por história dessas
/// a cada passada; datas da 0x54c8e0 sobre [0xae2c90]) recebe por cima a última (23 bytes), a conta desce e o mesmo
/// lugar é visto de novo. Sobrando alguma, reordena (qsort com 0x757ee0). Sem registro, nada.
pub(crate) unsafe extern "thiscall" fn expire_story_log(this: usize) {
    if read_usize(this) == 0 {
        return;
    }
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mut i = 0i32;
    while i < read_usize(this + 8) as i32 {
        let r = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x17));
        let days = if read_usize(r + 0x13) == 0xbdf {
            crate::comparacao::ramo("cpl_6 expirar historia do time");
            let s = sorteio(0x28); // o sorteio da história do time
            (s + 0x28) as u32
        } else {
            100 // os dias das outras histórias
        };
        let mut local = [0u8; 8];
        let limit = datar(G_CURRENT_DATE, &mut local, days);
        if media_date_before(r, limit) {
            let n = read_usize(this + 8);
            let last = read_usize(this).wrapping_add(n.wrapping_mul(0x17)).wrapping_sub(0x17);
            core::ptr::copy(last as *const u8, r as *mut u8, 0x17);
            write_usize(this + 8, n.wrapping_sub(1));
            i -= 1; // o mesmo lugar de novo
        }
        i += 1;
    }
    if read_usize(this + 8) != 0 {
        sort_news_log(read_usize(this), read_usize(this + 8));
    }
}

/// team story: linear search by second subject id and subtype; the others: binary search by the key and, with a date,
/// whether it is not older
/// Original: `Media::has_logged_story` (0x757da0, media.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x757da0 } (thiscall, ret 0x14, media.cpp): `Media::has_logged_story(a, b, tipo, subtipo, desde)`: a história do
/// time (tipo 0xbdf): procura em ordem a de tipo 0xbdf com o id de `b` (+0xc) e o `subtipo` (palavra +0x11), sem olhar
/// `a` nem a data: 1 se achou. As outras: busca binária (bsearch 0x946464 com 0x757ee0) pela chave {id de `a` ou −1,
/// id de `b`, tipo, subtipo} (a data padrão 0x54a110 na chave, que a comparação não lê); achou e sem `desde`, 1; com
/// `desde`, 1 se a história não é anterior a ela. Senão 0. `b` não pode ser nulo (a original lê sem conferir).
pub(crate) unsafe extern "thiscall" fn has_logged_story(this: usize, a: usize, b: usize, kind: u32, subtipo: u32, desde: usize) -> u32 {
    let mut key = [0u8; 0x17];
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    default(key.as_mut_ptr() as usize);
    let s = subtipo as u16;
    if kind == 0xbdf {
        crate::comparacao::ramo("cpl_6 historia do time ja contada");
        let n = read_usize(this + 8) as i32;
        for k in 0..n.max(0) as usize {
            let r = read_usize(this).wrapping_add(k * 0x17);
            if read_usize(r + 0x13) == 0xbdf && read_usize(r + 0xc) == read_usize(b) && read_u16(r + 0x11) == s {
                return 1; // a história do time já contada
            }
        }
        return 0;
    }
    key[8..0xc].copy_from_slice(&(media_id_or_minus_one(a) as u32).to_le_bytes());
    key[0x13..].copy_from_slice(&kind.to_le_bytes());
    key[0xc..0x10].copy_from_slice(&(read_usize(b) as u32).to_le_bytes());
    key[0x11..0x13].copy_from_slice(&s.to_le_bytes());
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = bsearch(key.as_ptr() as usize, read_usize(this), read_usize(this + 8) as u32, 0x17, 0x757ee0);
    if e == 0 {
        return 0;
    }
    if desde == 0 {
        return 1;
    }
    if media_date_before(e, desde) { 0 } else { 1 }
}






/// Search key for 0x754740: {player id, club id or -1}.
///
/// [pt] A chave da 0x754740: {id do jogador, id do clube ou −1} (os 8 bytes que a 0x757ea0 lê).
pub(crate) unsafe fn interest_key(player: usize, clube: usize) -> [u32; 2] {
    [read_usize(player) as u32, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }]
}

/// True if the interest key is already in the media's sorted interest array (bsearch with 0x757ea0).
///
/// [pt] O interesse já está no vetor? (bsearch 0x946464 com 0x757ea0; sem vetor, não).
pub(crate) unsafe fn has_interest(this: usize, key: &[u32; 2]) -> bool {
    let base = read_usize(this + 0xc);
    if base == 0 {
        return false;
    }
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(key.as_ptr() as usize, base, read_usize(this + 0x14) as u32, 0x19, 0x757ea0) != 0
}

/// Caps the interest value at 50,000,000.
///
/// [pt] O valor que a 0x754740 grava: o `valor` limitado a 50.000.000.
pub(crate) fn cap_interest_value(value: i32) -> i32 {
    if value < 0x2faf080 { value } else { 0x2faf080 }
}

/// if the {player, club} key is missing, appends the interest (today, fee up to 50 million, valuation × 100) and sorts
/// again; grows by 20
/// Original: `Media::add_transfer_interest` (0x754740, media.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x754740 } (thiscall, ret 0x14, media.cpp): `Media::add_transfer_interest(jogador, clube, valor, avaliacao)`: com
/// o vetor de interesses ([this+0xc], capacidade +0x10, conta +0x14; registros de 25 bytes), procura (bsearch 0x946464
/// com 0x757ea0) a chave {id do jogador, id do clube ou −1}: já está, nada. Senão acrescenta: id do jogador (+0), id do
/// clube (+4, lido sem conferir), a data de hoje (+8, 0x417c10 de [0xae2c90]), 0 (+0x10, o boato ainda não saiu), o
/// `valor` até 50.000.000 (+0x11) e a `avaliacao` × 100 truncada (+0x15, _ftol 0x945640); a conta sobe e reordena (qsort
/// com 0x757ea0). Sem vetor, aloca 20 (0x944e46; sem memória, caixa 0x665d e fim); cheio, cresce 20 (realloc 0x945667;
/// caixa 0x6668). A data padrão (0x54a110) da chave na pilha, como a original.
pub(crate) unsafe extern "thiscall" fn add_transfer_interest(this: usize, player: usize, clube: usize, value: i32, avaliacao: f64) {
    let mut data = [0u8; 8];
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    default(data.as_mut_ptr() as usize);
    let key = interest_key(player, clube);
    if has_interest(this, &key) {
        crate::comparacao::ramo("cpl_7 interesse repetido");
        return; // o interesse já está
    }
    crate::comparacao::ramo("cpl_7 interesse novo");
    if read_usize(this + 0xc) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let p = alocar(0x1f4);
        write_usize(this + 0xc, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x665d);
            fatal_exit();
        }
        write_usize(this + 0x10, 0x14);
    }
    let cap = read_usize(this + 0x10);
    if cap == read_usize(this + 0x14) {
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_usize(this + 0xc), cap.wrapping_add(0x14).wrapping_mul(0x19) as u32);
        write_usize(this + 0xc, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x6668);
            fatal_exit();
        }
        write_usize(this + 0x10, read_usize(this + 0x10).wrapping_add(0x14));
    }
    let item = || read_usize(this + 0xc).wrapping_add(read_usize(this + 0x14).wrapping_mul(0x19));
    write_usize(item(), read_usize(player));
    write_usize(item() + 4, read_usize(clube));
    copy_record_date(item() + 8, G_CURRENT_DATE);
    *((item() + 0x10) as *mut u8) = 0; // o boato ainda não saiu
    write_usize(item() + 0x11, cap_interest_value(value) as usize);
    write_usize(item() + 0x15, match_pl_ftol(avaliacao * 100.0) as usize); // [0x9688a8] = 100.0
    let n = read_usize(this + 0x14).wrapping_add(1);
    write_usize(this + 0x14, n); // mais um interesse
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0xc), n as u32, 0x19, 0x757ea0);
}

/// null first; by the +0 id and, when equal, by +4 (−1 matches anything)
/// Original: `compare_id_pair_keys` (0x757ea0, media.cpp, cdecl).
///
/// [pt] { 0x757ea0 } (cdecl, media.cpp): `compare_id_pair_keys(a, b)` do qsort e do bsearch: nulo antes (`a` nulo −1, `b`
/// nulo 1); ids +0 diferentes, a diferença; iguais, 0 se um dos +4 é −1 (o coringa), senão a diferença dos +4.
pub(crate) unsafe extern "C" fn compare_id_pair_keys(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (x, y) = (read_usize(a) as i32, read_usize(b) as i32);
    if x != y {
        return x.wrapping_sub(y);
    }
    let (x, y) = (read_usize(a + 4) as i32, read_usize(b + 4) as i32);
    if x == -1 || y == -1 {
        return 0;
    }
    x.wrapping_sub(y)
}

/// null first; by subjects +8 and +0xc, type +0x13 and subtype +0x11
/// Original: `compare_story_log_records` (0x757ee0, media.cpp, cdecl).
///
/// [pt] { 0x757ee0 } (cdecl, media.cpp): `compare_story_log_records(a, b)` do qsort e do bsearch do registro de histórias:
/// nulo antes (`a` nulo −1, `b` nulo 1); pelo sujeito a (+8), o sujeito b (+0xc) e o tipo (+0x13), a diferença do
/// primeiro diferente; iguais, a diferença dos subtipos (palavras +0x11 com sinal).
pub(crate) unsafe extern "C" fn compare_story_log_records(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    for o in [8usize, 0xc, 0x13] {
        let (x, y) = (read_usize(a + o) as i32, read_usize(b + o) as i32);
        if x != y {
            return x.wrapping_sub(y);
        }
    }
    let s = |p: usize| core::ptr::read_unaligned((p + 0x11) as *const i16) as i32;
    s(a) - s(b)
}

/// Builds the 25-byte interest record 0x754740 appends (player, club, today, capped value, rating x 100).
///
/// [pt] Os 25 bytes que a 0x754740 acrescenta.
pub(crate) unsafe fn build_interest_record(player: usize, clube: usize, value: i32, avaliacao: f64) -> [u8; 0x19] {
    let mut r = [0u8; 0x19];
    r[..4].copy_from_slice(&(read_usize(player) as u32).to_le_bytes());
    r[4..8].copy_from_slice(&(read_usize(clube) as u32).to_le_bytes());
    r[8..0x10].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    r[0x11..0x15].copy_from_slice(&cap_interest_value(value).to_le_bytes());
    r[0x15..].copy_from_slice(&match_pl_ftol(avaliacao * 100.0).to_le_bytes());
    r
}





/// Calls the exe's CRT qsort.
pub(crate) unsafe fn media_crt_qsort(base: usize, n: i32, size: u32, comparar: usize) {
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(base, n, size, comparar);
}

/// Calls the exe's CRT bsearch.
pub(crate) unsafe fn crt_bsearch(key: usize, base: usize, n: u32, size: u32, comparar: usize) -> usize {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(key, base, n, size, comparar)
}

/// removes the person interests (25 bytes) moving the last one into the gap; sorts again (0x757ea0)
/// Original: `Media::remove_transfer_interest` (0x7549b0, media.cpp, thiscall, ret 4).
///
/// [pt] { 0x7549b0 } (thiscall, ret 4, media.cpp): `Media::remove_transfer_interest(pessoa)`: no vetor de interesses
/// ([this+0xc], [this+0x14] registros de 25 bytes), cada um da pessoa (+0 = id) recebe por cima o último (6 duplas e um
/// byte), a contagem desce e o mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x757ea0).
pub(crate) unsafe extern "thiscall" fn remove_transfer_interest(this: usize, person: usize) {
    let n = (this + 0x14) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + 0xc) as usize;
            let r = base.wrapping_add(off);
            if read_u32(r) == read_u32(person) {
                let last = base.wrapping_add((*n as usize).wrapping_mul(25)).wrapping_sub(25); // o último interesse
                core::ptr::copy(last as *const u8, r as *mut u8, 25);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(25);
            }
            i += 1;
            off = off.wrapping_add(25);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        media_crt_qsort(read_u32(this + 0xc) as usize, *n, 0x19, 0x757ea0);
    }
}






/// { 0x7357c0 } (thiscall, ret 4, media.cpp): o tipo da história guardada na imprensa sobre o jogador
/// quando é de indisciplina (lista de tipos em 0xa17be0, terminada em 0xff), senão 0. Histórias: [+0]
/// vetor de 0x17 bytes, [+8] quantas; em +8 o id do jogador, em +0x11 o tipo (palavra). Jogador nulo ou
/// com +0x39 zerado: 0.
// simplification: só o ax é fiel; os 16 bits altos do eax da original são resto de registrador (quem chama usa ax)
/// the kind of the media story about the player when it is a misbehaviour kind, else 0
/// Original: `Media::get_finable_story_kind` (0x7357c0, media.cpp, thiscall).
///
/// [pt] o tipo da história da imprensa sobre o jogador quando é de indisciplina, senão 0
pub(crate) unsafe extern "thiscall" fn get_finable_story_kind(midia: usize, player: usize) -> u16 {
    if player == 0 || *((player + 0x39) as *const u32) == 0 {
        return 0;
    }
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    let id = *(player as *const u32);
    for i in 0..n.max(0) as usize {
        let e = base.wrapping_add(i * 0x17);
        if core::ptr::read_unaligned((e + 8) as *const u32) != id || *(0xa17be0 as *const u8) == 0xff {
            continue;
        }
        let kind = core::ptr::read_unaligned((e + 0x11) as *const u16);
        let mut p = 0xa17be0usize;
        loop {
            if kind == *(p as *const i8) as i16 as u16 {
                return kind;
            }
            p += 1;
            if *(p as *const u8) == 0xff {
                break;
            }
        }
    }
    0 // nenhuma história de indisciplina
}

/// { 0x62d140 } (thiscall, ret 0xc, injury.cpp): o tipo de lesão da categoria (parte do corpo) e da
/// gravidade pedidas: se o jogador já tem lesão atual (0x62ed70) de tipo fora da lista de leves, devolve
/// esse tipo. Senão sorteia cada tipo da categoria (primeiro/último em 0x9e7f05/06 + 3 × categoria; a
/// tabela de tipos tem 13 bytes a partir de 0x9e7ad8: +1 categoria, +2 peso, +0xc gravidade, 0xff =
/// qualquer) e fica com o menor sorteio (menor que 100); o tipo recorrente do jogador ([this] + 31 × id +
/// 0x12) sorteia só 0..2 − 1. Gravidade 3: nenhum (0xff).
// simplification: só o al é fiel (a original devolve char)





/// True if any 0x17-byte media news record has the player id at +8 and passes the predicate.
///
/// [pt] O registro de notícias da mídia (`this`: +0 os registros de 0x17 bytes, +8 quantos): algum com o
/// jogador em +8 que `aceita`?
pub(crate) unsafe fn any_news_for_player(midia: usize, id: i32, aceita: impl Fn(usize) -> bool) -> bool {
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    (0..n.max(0) as usize).any(|k| {
        let r = base + k * 0x17;
        core::ptr::read_unaligned((r + 8) as *const i32) == id && aceita(r)
    })
}

/// whether the player has a story with response 2 in the media log
/// Original: `story_with_response_two` (0x735780, media.cpp, thiscall).
///
/// [pt] { 0x735780 } (thiscall, ret 4, media.cpp): o jogador `*id` tem uma notícia com a resposta 2 (+0x10)?
pub(crate) unsafe extern "thiscall" fn story_with_response_two(midia: usize, id: *const i32) -> u32 {
    any_news_for_player(midia, *id, |r| *((r + 0x10) as *const u8) == 2) as u32
}

/// whether the player has any story in the media log
/// Original: `has_story` (0x735840, media.cpp, thiscall).
///
/// [pt] { 0x735840 } (thiscall, ret 4, media.cpp): o jogador `*id` tem alguma notícia? Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn has_story(midia: usize, id: *const i32) -> u8 {
    any_news_for_player(midia, *id, |_| true) as u8
}




/// builds news 0xbdb (player, names, club, story and the random variant for some stories) and sends it to the club's
/// manager
/// Original: `media_queue_player_apology_news` (0x72bbf0, media.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x72bbf0 } (stdcall, ret 0xc, media.cpp, quadro SEH sem destrutor): `media_queue_player_apology_news(jogador,
/// clube, historia)`, pelos efeitos da história (0x736c00) e pela resposta à história do jogador (0x741020): com o
/// jogador, a ficha (+0x61) e o clube, a notícia 0xbdb numa local (0x77bbe0 com 0) com os dados 0 = id, 1 a 3 = os
/// nomes (+0x33 de +4, +8 e +0xc; sem o terceiro, −1), 4 = id do clube, 5 = a história, 6 = 0 e 7 = a variante:
/// sorteio(4) nas histórias 0, 2 a 5, 13, 14, 17 e 35 (tabela 0x72bd2c), senão 0; vai para o técnico do clube
/// (0x7861e0).
pub(crate) unsafe extern "stdcall" fn media_queue_player_apology_news(player: usize, clube: usize, historia: i16) {
    if player == 0 || read_i32(player + 0x61) == 0 || clube == 0 {
        return;
    }
    crate::comparacao::ramo("media_queue_player_apology_news");
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let to_manager: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbdb, 0);
    write(n, 0, read_i32(player) as u32);
    write(n, 1, read_i32(read_i32(player + 4) as usize + 0x33) as u32);
    write(n, 2, read_i32(read_i32(player + 8) as usize + 0x33) as u32);
    let terceiro = read_i32(player + 0xc) as usize;
    write(n, 3, if terceiro == 0 { u32::MAX } else { read_i32(terceiro + 0x33) as u32 });
    write(n, 4, read_i32(clube) as u32);
    let h = historia as i32;
    write(n, 5, h as u32);
    write(n, 6, 0);
    let v = if (h as u32) <= 0x23 && *((0x72bd2c + h as usize) as *const u8) == 0 { sorteio(4) } else { 0 };
    write(n, 7, v as u32);
    to_manager(read_i32(G_NEWS) as usize, n, clube);
}


/// draws a former international (37+, 50+ caps) as pundit, weighted by nation reputation and caps
/// Original: `Media::pick_former_international_pundit` (0x72b810, media.cpp, thiscall).
///
/// [pt] { 0x72b810 } (thiscall, ret 8, media.cpp; `this` não é lido): sorteia o comentarista
/// ex-internacional para o `clube` (ou para falar de `assunto`). 150 rodadas; em cada uma sorteia
/// pessoas (0x90cfc0 com [0xae2364], 110 bytes em [0xae23bc]) até achar uma com idade (+0x18) ≥ 37,
/// jogos pela seleção (+0x22) ≥ 50, ficha (+0x69), sem clube (+0x39) se tiver contrato (+0x61) e que
/// não seja o técnico do clube (+0xcf); a 100ª tentativa encerra a rodada. O peso é um sorteio de
/// reputação da nação (+0x1a, palavra +0x8e) × jogos, dobrado se for da nação do assunto (sem
/// assunto: tem de ser da nação do clube, +0x53). De outra nação: a reputação tem de vencer 6500 +
/// sorteio de 100, a ficha +0xc ≥ 6000 e o assunto não pode desgostar dele (0x547840). Fica o de
/// maior peso (o primeiro aceito sempre entra). Sem clube e sem assunto: 0.
pub(crate) unsafe extern "thiscall" fn pick_former_international_pundit(_this: usize, clube: usize, assunto: usize) -> usize {
    if assunto == 0 && clube == 0 {
        return 0;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let injury_word_0c = |nacao: usize| core::ptr::read_unaligned((nacao + nation::REPUTATION) as *const i16) as i32;
    let (mut melhor, mut maior) = (0usize, 0i32);
    for _ in 0..150 {
        let mut tentativas = 0;
        let p = loop {
            let r = sorteio(*(G_STAFF_COUNT as *const i32));
            tentativas += 1;
            if tentativas >= 100 {
                break 0;
            }
            let p = (*(G_STAFF_TABLE as *const usize)).wrapping_add((r as usize).wrapping_mul(110));
            if *((p + 0x18) as *const i8) < 0x25 || *((p + 0x22) as *const u8) < 0x32 || u(p + 0x69) == 0 {
                continue;
            }
            if u(p + 0x61) != 0 && u(p + 0x39) != 0 {
                continue;
            }
            if u(clube + club::MANAGER) == p {
                continue;
            }
            break p;
        };
        if p == 0 {
            continue;
        }
        let nacao = u(p + 0x1a);
        let mut weight = sorteio(injury_word_0c(nacao).wrapping_mul(*((p + 0x22) as *const u8) as i32));
        let same_nation = if assunto == 0 {
            if nacao != u(clube + club::NATION) {
                continue;
            }
            true
        } else {
            nacao == u(assunto + 0x1a)
        };
        if same_nation {
            weight = weight.wrapping_add(weight);
        } else {
            let corte = sorteio(100).wrapping_add(0x1964);
            if injury_word_0c(nacao) < corte
                || core::ptr::read_unaligned((u(p + 0x69) + 0xc) as *const i16) < 0x1770
                || not_likes(assunto, p) != 0
            {
                continue;
            }
        }
        if melhor == 0 || weight > maior {
            maior = weight;
            melhor = p;
        }
    }
    melhor
}


/// the AI manager's response 1, 2 or 3 to a story of the given type about the player: by the contract, the player's
/// story rating (× 5), the manager's likes and dislikes and chains of draws per family of types; some types add the
/// player to the manager's rejected list
/// Original: `Media::pick_ai_manager_response` (0x735870, media.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x735870 } (thiscall, ret 0xc, media.cpp, 3,7 KB): `Media::pick_ai_manager_response(tecnico, jogador, tipo)`: a
/// resposta (1, 2 ou 3) do técnico da máquina a uma história do `tipo` sobre o jogador. Sem técnico, sem a ficha de
/// técnico (+0x69), jogador sem ficha (+0x61), o próprio técnico, sem clube (+0x39, senão a seleção +0x24) ou sem
/// contrato do jogador com ele (0x4cf5f0): 3. A nota é trunc(5 × 0x736720(jogador, clube, contrato, tipo, 3, 0)); sem
/// jogador, a palavra do argumento fica no lugar (o ponteiro do técnico, como no original). Depois, pelo tipo: 0x83
/// pelo tipo de contrato e o conhecimento (0x544410); 0x82 pelas preferências do técnico (0x547840/0x547810) e
/// sorteios; 0x1f pela lista de transferências (0x544520) e o contrato; os outros, antes, pelas preferências com a
/// nota, e depois por famílias de tipos, cada uma com a sua cadeia de sorteios (0x90cfc0); 8/0x25 e 3/4 podem pôr o
/// jogador entre os rejeitados do técnico (0x547af0).
pub(crate) unsafe extern "thiscall" fn pick_ai_manager_response(this: usize, manager: usize, player: usize, kind: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let nota_da_historia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_MEDIA_RATE_PLAYER_FORM_FOR_STORY);
    let not_likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let listada_pelo_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    if manager == 0 || read_usize(manager + 0x69) == 0 {
        return 3;
    }
    if player != 0 && read_usize(player + 0x61) == 0 {
        return 3;
    }
    if manager == player {
        return 3;
    }
    let mut clube = read_usize(manager + 0x39);
    if clube == 0 {
        clube = read_usize(manager + 0x24);
        if clube == 0 {
            return 3;
        }
    }
    let contrato = contract_with_club(read_usize(G_CONTRACT_MANAGER), player, clube);
    if contrato == 0 {
        return 3;
    }
    let mut rating = manager as i32; // sem jogador: o argumento que ocupa a vaga
    if player != 0 {
        rating = match_pl_ftol(nota_da_historia(this, player, clube, contrato, kind, 3, 0.0) * *(0x968990 as *const f32) as f64);
    }
    let f = || read_usize(manager + 0x69); // a ficha de técnico
    let ch = |o: usize| chance(f(), *((f() + o) as *const i8)) as i8 as i32; // o conhecimento pelo atributo +o
    let k = kind as i16;
    if k == 0x83 {
        let t = contract_squad_status(contrato);
        if t == 0x10 || t == 0x20 {
            return 1;
        }
        if t == 0x30 && read_i8_as_i32(contrato + 0x3f) > 1 {
            return 1;
        }
        if read_i16(read_usize(player + 0x61) + 0xb) < 0x1482 && ch(0x14) > 0xf {
            return 2;
        }
        if contract_squad_status(contrato) == 0x70 && ch(0x14) > 0xa {
            return 2;
        }
        return if sorteio(ch(0x19)) != 0 { 1 } else { 3 };
    }
    if k == 0x82 {
        if not_likes(player, manager) != 0 {
            let a = read_i8_as_i32(manager + 0x5b);
            if a < 0xf && sorteio(a) == 0 {
                let x = sorteio(ch(0x19));
                let y = sorteio(0x14);
                if x < y {
                    return 2;
                }
            }
            let a = read_i8_as_i32(manager + 0x5b);
            if a <= 0xe || sorteio(a) <= 10 {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0x14);
            return if x <= y { 3 } else { 1 };
        }
        if gosta(player, manager) != 0 {
            let x = sorteio(read_i8_as_i32(manager + 0x5b));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0xf);
            return if x <= y { 3 } else { 1 };
        }
        if read_i8_as_i32(manager + 0x5d) < 6 {
            let y = sorteio(0x14);
            if read_i8_as_i32(manager + 0x5d) < y {
                return 1;
            }
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        if x > y {
            return 1;
        }
        if read_i8_as_i32(manager + 0x5c) <= 0xf {
            return 3;
        }
        let y = sorteio(10);
        return if read_i8_as_i32(manager + 0x5c) <= y { 3 } else { 2 };
    }
    if k == 0x1f {
        if listada(player, clube) != 0 {
            return 2;
        }
        let b = read_i8_as_i32(contrato + 0x3d);
        if b <= sorteio(100) {
            return 3;
        }
        if criterio(contrato) != 0 || *((contrato + contract::LEAVING_ON_BOSMAN) as *const u8) != 0 {
            return 3;
        }
        return if sorteio(3) != 0 { 1 } else { 3 };
    }
    // as preferências do técnico com a nota
    if not_likes(player, manager) != 0 && rating < 5 {
        if sorteio(read_i8_as_i32(manager + 0x5b)) != 0 {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 2 };
    }
    if gosta(player, manager) != 0 && rating > -5 {
        let x = sorteio(read_i8_as_i32(manager + 0x5b));
        let y = sorteio(10);
        if x >= y {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 1 };
    }
    // a soma do conhecimento (+0x19), de +0x1d da ficha e de +0x59 do técnico
    let sum = || ch(0x19) + read_i8_as_i32(f() + 0x1d) + read_i8_as_i32(manager + 0x59);
    match k as i32 {
        0x3c | 0x3d => {
            let x = sorteio(ch(0x1b));
            let y = sorteio(0x17);
            if x > y {
                let x = sorteio(read_i8_as_i32(f() + 0x1d));
                let y = sorteio(10);
                if x < y {
                    return 2;
                }
            }
            if sorteio(read_i8_as_i32(manager + 0x5a)) == 0 {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(manager + 0x59));
            let y = sorteio(0x28);
            if x > y {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(f() + 0x1d));
            let y = sorteio(0x32);
            if x <= y { 3 } else { 1 }
        }
        0x28..=0x3b => {
            let x = sorteio(read_i8_as_i32(player + 0x5b));
            let y = sorteio(0x14);
            if x > y {
                let a = sorteio(ch(0x19));
                let b = sorteio(0x14 - rating);
                if a > b {
                    return 1;
                }
                if sorteio(ch(0x19)) != 0 {
                    return 3;
                }
                return if rating >= 0xa { 3 } else { 2 };
            }
            let a = sorteio(ch(0x19));
            let b = sorteio(10);
            if a > b {
                let mut arg = 0;
                if *((read_usize(player + 0x61) + 0x44) as *const u8) != 0 {
                    // o registro de 0x4f bytes da pessoa em [0xae2d0c] (+0xa, +0xb bit 7)
                    let reg = |o: usize| *((read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(player) as usize).wrapping_mul(0x4f)) + o) as *const u8);
                    let e = sorteio((2 - (reg(0xb) >> 7) as i32) * 20);
                    if sorteio(reg(0xa) as i32) > e {
                        arg = 1;
                    }
                }
                if sorteio(arg) == 0 && rating < 0xa {
                    return 2;
                }
            }
            if sorteio(read_i8_as_i32(read_usize(player + 0x61) + 0x45)) == 0 {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(manager + 0x59));
            let y = sorteio(0x14 - rating);
            if x <= y { 3 } else { 1 }
        }
        0 | 0xd | 0x11 | 0x52 | 0x23 | 0xe | 2 | 0x21 | 0x22 | 9 | 0xc => {
            crate::comparacao::ramo("cpo_6 tipos comuns");
            let t = contract_squad_status(contrato);
            if t == 0x30 || t == 0x10 || t == 0x20 {
                let e = entry(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), clube, player, 0, 0);
                if e != 0 && read_usize(e + 0xf) == read_usize(player) && rating > -10 {
                    crate::comparacao::ramo("cpo_6 sorteio descartado");
                    sorteio(read_i8_as_i32(manager + 0x59)); // o original descarta o resultado
                }
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(10);
            if x < y {
                let x = sorteio(read_i8_as_i32(manager + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(read_i8_as_i32(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y {
                        let x = sorteio(read_i8_as_i32(manager + 0x59));
                        let y = sorteio(10);
                        if x < y {
                            let x = sorteio(rating);
                            let y = sorteio(-5);
                            if x < y {
                                return 2;
                            }
                        }
                    }
                }
            }
            let x = sorteio(sum());
            let y = sorteio(0x24);
            if x <= y {
                return 3;
            }
            let x = sorteio(rating);
            let y = sorteio(-5);
            if x > y { 1 } else { 3 }
        }
        8 | 0x25 => {
            if sorteio(read_i8_as_i32(f() + 0x1d) * 2) == 0 {
                rejeitar(manager, player, 1);
            }
            if ch(0x19) < 0xa {
                let x = sorteio(read_i8_as_i32(manager + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(read_i8_as_i32(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(sum());
            let y = sorteio(0x24);
            if x <= y { 3 } else { 1 }
        }
        1 => {
            if sorteio(ch(0x19)) == 0 {
                let x = sorteio(read_i8_as_i32(manager + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(read_i8_as_i32(f() + 0x1d));
                    let y = sorteio(5);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(sum());
            let y = sorteio(0x1e);
            if x <= y { 3 } else { 1 }
        }
        0x1e | 0x20 => {
            let x = sorteio(read_i8_as_i32(manager + 0x5d));
            let y = sorteio(0x14);
            if x < y {
                let x = sorteio(read_i8_as_i32(f() + 0x1d));
                let y = sorteio(0xf);
                if x < y {
                    return 2;
                }
            }
            let x = sorteio(read_i8_as_i32(f() + 0x1d) + read_i8_as_i32(manager + 0x59));
            let y = sorteio(0x96);
            if x > y {
                if read_i16(read_usize(player + 0x61) + 0xb) as i32 <= read_i16(clube + club::REPUTATION) as i32 + 0x1f4 {
                    return 2;
                }
                let x = sorteio(read_i8_as_i32(f() + 0x14));
                let y = sorteio(5);
                return if x >= y { 3 } else { 1 };
            }
            let x = sorteio(read_i8_as_i32(f() + 0x14));
            let y = sorteio(5);
            if x < y {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(f() + 0x14));
            let y = sorteio(0x14);
            if x <= y { 3 } else { 2 }
        }
        0x63 | 0x62 | 0x5d | 0x3f | 0x5f | 0x60 | 0x43 | 0x5c | 0x5a | 0x44 | 0x5b | 0x42 | 0x61 | 0x40 | 0x41 => {
            let mut base: i32 = match k {
                0x5d => 5,
                0x61 => 10,
                0x62 => -7,
                _ => 0,
            };
            if sorteio(read_i8_as_i32(manager + 0x5a)) == 0 {
                base += sorteio(10);
                base -= sorteio(10);
            }
            let x = sorteio(read_i8_as_i32(f() + 0x14));
            let y = sorteio(0xf - base);
            if x > y {
                return 2;
            }
            let x = sorteio(read_i8_as_i32(manager + 0x57));
            let y = sorteio(base + 0xf);
            if x <= y { 3 } else { 1 }
        }
        3 | 4 => {
            let x = sorteio(read_i8_as_i32(f() + 0x14));
            let y = sorteio(0x32);
            if x > y && sorteio(read_i8_as_i32(f() + 0x1d)) == 0 {
                let x = sorteio(rating);
                let y = sorteio(-5);
                if x < y {
                    rejeitar(manager, player, 1);
                    return 2;
                }
            }
            let t = contract_squad_status(contrato);
            if t != 0 && t != 0x70 && t != 0x80 && listada_pelo_clube(player, 0) == 0 {
                let x = sorteio(read_i8_as_i32(manager + 0x59));
                let y = sorteio(6);
                if x > y {
                    let x = sorteio(ch(0x19));
                    let y = sorteio(6);
                    if x > y {
                        let x = sorteio(rating);
                        let y = sorteio(-5);
                        if x > y {
                            return 1;
                        }
                    }
                }
            }
            let t = contract_squad_status(contrato);
            if t != 0x70 && t != 0x40 {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(manager + 0x59));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(manager + 0x5b));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(rating);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        5 => {
            if sorteio(read_i8_as_i32(manager + 0x59)) != 0 && sorteio(ch(0x19)) != 0 && sorteio(read_i8_as_i32(player + 0x5b)) != 0 {
                let x = sorteio(rating);
                let y = sorteio(-5);
                if x > y {
                    return 1;
                }
            }
            if sorteio(ch(0x19)) != 0 {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(player + 0x5a));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(rating);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        6 => {
            if sorteio(read_i8_as_i32(manager + 0x59)) != 0 {
                let x = sorteio(read_i8_as_i32(f() + 0x1f));
                let y = sorteio(10);
                if x > y {
                    let x = sorteio(read_i8_as_i32(player + 0x5b));
                    let y = sorteio(5);
                    if x > y {
                        let x = sorteio(read_i8_as_i32(read_usize(player + 0x61) + 0x25));
                        let y = sorteio(0x1e);
                        if x < y {
                            let x = sorteio(rating);
                            let y = sorteio(-5);
                            if x > y {
                                return 1;
                            }
                        }
                    }
                }
            }
            if sorteio(ch(0x19)) != 0 || sorteio(read_i8_as_i32(f() + 0x1f)) != 0 {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(read_usize(player + 0x61) + 0x25));
            let y = sorteio(0x32);
            if x <= y {
                return 3;
            }
            let x = sorteio(rating);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        _ => 3,
    }
}


/// news 0x1790 (id, reputation, draw of 2) to the managers of the nation, or to all for a big club (9000+)
/// Original: `Media::post_financial_trouble_news` (0x74ae60, media.cpp, thiscall, ret 4).
///
/// [pt] { 0x74ae60 } (thiscall, ret 4, media.cpp, quadro SEH sem destrutor): `Media::post_financial_trouble_news(clube)`:
/// com o clube, a nação (+0x53), a ficha (+0x69, com +0x38) e a divisão (+0x57), monta a notícia 0x1790 numa local
/// (0x77bbe0 com 0) com os dados 0 = id do clube, 1 = reputação (+0x80) e 2 = sorteio(2) (0x90cfc0); clube de
/// reputação 9000 ou mais numa divisão com +0x69 acima de 12: para todos os técnicos (0x786780); senão para os da
/// nação (0x7863f0). A notícia local tem 0xe0 bytes (a entrega copia só até +0xde).
pub(crate) unsafe extern "thiscall" fn post_financial_trouble_news(_this: usize, clube: usize) {
    if clube == 0 || read_usize(clube + club::NATION) == 0 {
        return;
    }
    let sheet = read_usize(clube + club::STADIUM);
    if sheet == 0 || read_usize(sheet + 0x38) == 0 || read_usize(clube + club::DIVISION) == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_6 apuro financeiro");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    construct(n, 0x1790, 0);
    write(n, 0, read_usize(clube) as u32);
    write(n, 1, read_i16(clube + club::REPUTATION) as i32 as u32);
    let v = sorteio(2); // o sorteio da notícia
    write(n, 2, v as u32);
    let noticias = read_usize(G_NEWS);
    let division = read_usize(clube + club::DIVISION);
    if read_i16(clube + club::REPUTATION) >= 9000 && division != 0 && read_i16(division + 0x69) > 12 {
        let all: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        all(noticias, n);
    } else {
        let pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
        pais(noticias, n, read_usize(clube + club::NATION));
    }
}


/// the manager of a rival (+0xb3/+0xb7/+0xbb) of the same league and much bigger, or of a drawn club in a league clash,
/// who passes the draws; returns the id or −1
/// Original: `Media::pick_rival_manager_id` (0x72b940, media.cpp, thiscall, ret 4).
///
/// [pt] { 0x72b940 } (thiscall, ret 4, media.cpp): `Media::pick_rival_manager_id(clube)`: com técnico (+0xcf) com contrato
/// (+0x69) e liga (+0x57; sem liga, só seleção, 0x53b900): o primeiro dos rejeitados do técnico (0x5480f0) com
/// registro de técnico e nota + 750 < nota (nunca: o sorteio(3) dali não acontece); depois cada rival (+0xb3, +0xb7,
/// +0xbb) com técnico que passa nos sorteios, da mesma liga e de reputação (+0x80) acima da do clube + 750, com
/// sorteio(3) ≠ 0; depois até 50 clubes sorteados (de [0xae235c] − 2 × [0xae2350]) que dão confronto na liga
/// (0x4b02e0, com o limite do sorteio como 3º argumento, resto da pilha do original) com técnico que passa nos
/// sorteios. Devolve o id do técnico, ou −1.
pub(crate) unsafe extern "thiscall" fn pick_rival_manager_id(_this: usize, clube: usize) -> u32 {
    if read_u32(clube + club::MANAGER) == 0 {
        return u32::MAX;
    }
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if read_u32(clube + club::DIVISION) == 0 && national_team(clube) == 0 {
        return u32::MAX;
    }
    let manager = read_u32(clube + club::MANAGER) as usize;
    if read_u32(manager + 0x69) == 0 {
        return u32::MAX;
    }
    let mut rejeitados = [0usize; 20];
    let mut n = clube as u32; // a vaga do argumento: a 0x5480f0 zera o byte antes de contar
    let rejeitadas: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_STAFF_GET_DISLIKED_STAFF);
    rejeitadas(manager, rejeitados.as_mut_ptr() as usize, &mut n as *mut u32 as usize);
    let mut k = 0i8;
    while k < n as u8 as i8 {
        let p = *rejeitados.as_ptr().add(k as u8 as usize); // como o original, sem conferir o tamanho (a 0x5480f0 dá até 3)
        if p != 0 && read_u32(p + 0x69) != 0 {
            let rating = *((read_u32(p + 0x69) as usize + 0xa) as *const i16) as i32;
            if rating.wrapping_add(0x2ee) < rating && friendly_random(3) != 0 {
                return read_u32(p);
            }
        }
        k = k.wrapping_add(1);
    }
    for rival in [0xb3usize, 0xb7, 0xbb] {
        let r = read_u32(clube + rival) as usize;
        if r == 0 || read_u32(r + 0xcf) == 0 || !rival_manager_passes_draws(r) {
            continue;
        }
        if read_u32(r + 0x57) != read_u32(clube + club::DIVISION) {
            continue;
        }
        let injury_word_0c = |c: usize| *((c + 0x80) as *const i16) as i32;
        if injury_word_0c(clube) + 0x2ee >= injury_word_0c(r) {
            continue; // o rival não é bem maior
        }
        if friendly_random(3) != 0 {
            return read_u32(read_u32(r + 0xcf) as usize);
        }
    }
    let tie: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let mut b = 0i8;
    while b < 0x32 {
        let limit = (read_u32(G_CLUB_COUNT) as i32).wrapping_sub((read_u32(G_NATION_COUNT) as i32).wrapping_mul(2));
        let i = friendly_random(limit) as usize;
        let c = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
        if tie(clube, c, limit) != 0 && read_u32(c + 0xcf) != 0 && rival_manager_passes_draws(c) {
            return read_u32(read_u32(c + 0xcf) as usize);
        }
        b += 1;
    }
    u32::MAX
}


/// club with a nation and manager: news 0x178e (club, manager, type, person, a, b, nation, c) to the nation's managers
/// if the reputation is above 4750, else to the club
/// Original: `Media::queue_manager_fa_complaint_story` (0x758de0, media.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x758de0 } (thiscall, ret 0x18, media.cpp, SEH da notícia local, sem destrutor): `Media::
/// queue_manager_fa_complaint_story(clube, pessoa, tipo, a, b, c)`, pela disciplina (0x55e300) e pelo MANAGER_MANAGER
/// (0x6a9e60, 0x6ab800). Clube com nação (+0x53) e técnico (+0xcf), e pessoa: a notícia 0x178e (o clube; o técnico, id
/// e chaves, ou −1; o tipo; a pessoa, id e chaves; a; b; a nação ou −1; c) aos técnicos da nação (0x7863f0) se a
/// reputação (+0x80) passar de 4750, senão ao clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn queue_manager_fa_complaint_story(this: usize, clube: usize, person: usize, kind: u32, a: u32, b: u32, c: u32) {
    let _ = this;
    if clube == 0 || read_usize(clube + club::NATION) == 0 || read_usize(clube + club::MANAGER) == 0 || person == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_10 noticia da queixa a federacao");
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0x178e, 0);
    write(nt, 0, read_u32(clube));
    let manager = read_usize(clube + club::MANAGER);
    if manager != 0 {
        write(nt, 1, read_u32(manager));
        write(nt, 2, read_u32(read_usize(manager + 4) + 0x33));
        write(nt, 3, read_u32(read_usize(manager + 8) + 0x33));
        write(nt, 4, read_u32(read_usize(manager + 0xc) + 0x33));
    } else {
        for k in 1..5 {
            write(nt, k, u32::MAX);
        }
    }
    write(nt, 5, kind as i8 as u32);
    write(nt, 6, read_u32(person));
    write(nt, 7, read_u32(read_usize(person + 4) + 0x33));
    write(nt, 8, read_u32(read_usize(person + 8) + 0x33));
    write(nt, 9, read_u32(read_usize(person + staff::COMMON_NAME) + 0x33));
    write(nt, 0xa, a as i8 as u32);
    write(nt, 0xb, b as i16 as u32);
    let nacao = read_usize(clube + club::NATION);
    write(nt, 0xc, if nacao != 0 { read_u32(nacao) } else { u32::MAX });
    write(nt, 0xd, c as i8 as u32);
    let nacao = read_usize(clube + club::NATION);
    if nacao != 0 && read_i16_as_i32(clube + club::REPUTATION) > 0x128e {
        let aos_of_nation: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
        aos_of_nation(read_usize(G_NEWS), nt, nacao);
    } else {
        let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        deliver(read_usize(G_NEWS), nt, clube);
    }
}

