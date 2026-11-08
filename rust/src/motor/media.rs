use super::*;

// ---------- 266a leva: os boatos de transferência, a notícia de que o clube não vende o jogador e a notícia do título (media.cpp) ----------








pub(crate) unsafe fn cqc10_humano(c: usize, a: u32, b: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(c, a, b, 0, 0)
}

/// Os clubes humanos que recebem notícias do jogador (0x8d1da0 em [0xde1f54]: 4 na pilha, ret 0x10; o ecx da original é
/// o gerenciador das transferências).
pub(crate) unsafe fn cqc10_destinos(jogador: usize, marcas: u32, saida: *mut usize, max: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    f(read_usize(G_TRANSFER_MANAGER), jogador, marcas, saida, max)
}

/// O id e os três nomes (+0x33 de +4, +8, +0xc) de uma pessoa, nos índices `k`..`k+3` da notícia.
pub(crate) unsafe fn cqc10_nomes(gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    gravar(n, k, read_usize(p) as u32);
    gravar(n, k + 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, k + 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(n, k + 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// Acrescenta `c` aos destinos se ainda não estiver lá (até 30).
pub(crate) fn cqc10_acrescentar(dest: &mut [usize; 30], n: &mut u8, c: usize) {
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
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let resposta: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_MEDIA_PICK_AI_MANAGER_RESPONSE);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_GET_PLAYER_STRENGTH_FLAGS);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    if read_i32(this + 0x14) == 0 {
        return;
    }
    // ponytail: o técnico do dono fica da volta anterior quando o jogador não tem contrato (a original não o regrava);
    // na primeira volta a original lê a pilha sem valor: aqui 0
    let mut tec_dono: usize = 0;
    let mut i = 0i32;
    while i < read_i32(this + 0x14) {
        let r = read_usize(this + 0xc).wrapping_add((i as usize).wrapping_mul(25));
        let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(0x6e));
        let clube = cpw6_clube(read_i32(r + 4));
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
        core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
        let (mut dono, mut vinc) = (0usize, 0usize);
        if c != 0 {
            dono = cpw6_clube(read_i32(c + 4));
            if dono != 0 {
                vinc = vinculo(read_usize(G_CONTRACT_MANAGER), pessoa, dono);
                tec_dono = read_usize(dono + 0xcf);
            }
        }
        let mut hoje = [0u8; 8];
        core::ptr::copy_nonoverlapping(G_CURRENT_DATE as *const u8, hoje.as_mut_ptr(), 8);
        let mut fim = [0u8; 8];
        let p = mais(r + 8, fim.as_mut_ptr(), 0x1f); // cqc10: o interesse vale 31 dias
        crate::comparacao::ramo("cqc_10 interesse");
        let (hd, ha) = (i16::from_le_bytes([hoje[0], hoje[1]]), u16::from_le_bytes([hoje[2], hoje[3]]));
        let vencido = ha > read_i16(p + 2) as u16 || (ha == read_i16(p + 2) as u16 && hd > read_i16(p));
        if vencido || tec_dono == 0 || read_usize(pessoa + staff::PLAYER) == 0 {
            let n = read_i32(this + 0x14);
            let ultimo = read_usize(this + 0xc).wrapping_add((n as usize).wrapping_mul(25)).wrapping_sub(25);
            core::ptr::copy(ultimo as *const u8, r as *mut u8, 25);
            core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1);
            continue; // o último veio para o lugar i: a original volta um e avança um
        }
        if chegar(read_usize(G_TRANSFER_MANAGER), pessoa, clube, 0, 0) == 0 {
            i += 1;
            continue;
        }
        if read_u8(r + 0x10) == 0 && {
            crate::comparacao::ramo("cqc_10 sorteio do boato");
            cpe1_sorteio(0x19) == 0 // cqc10: o sorteio do boato
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
            construir(n, 0xfbc, 0);
            cqc10_nomes(gravar, n, 0, pessoa);
            gravar(n, 4, if dono != 0 { read_usize(dono) as u32 } else { 0xffff_ffff });
            let mut pede = false;
            let mut sai = false;
            if read_i8(pessoa + staff::PROFESSIONALISM) < 0xf && read_i8(pessoa + staff::LOYALITY) < 0xf {
                crate::comparacao::ramo("cqc_10 pedido de transferencia");
                let rr = cpe1_sorteio(0x30d4); // cqc10: o sorteio da reputação para o pedido
                if (read_i16(read_usize(pessoa + staff::PLAYER) + 0xd) as i32) > rr {
                    let lim = read_i8(pessoa + staff::LOYALITY) as i32 as f64 * 12.5 + 500.0;
                    if lim < read_i32(r + 0x15) as f64 {
                        gravar(n, 5, 0x24);
                        pede = true;
                        sai = true;
                    }
                }
            }
            if !sai {
                let s = cpe1_sorteio(3);
                gravar(n, 5, s as u32);
            }
            gravar(n, 6, read_usize(r + 0x11) as u32);
            let resp = if dono != 0 && cqc10_humano(dono, 0, 1) == 0 {
                resposta(this, read_usize(dono + 0xcf), pessoa, 0x1f)
            } else {
                0
            };
            gravar(n, 7, resp);
            gravar(n, 8, forcas(pessoa, dono));
            gravar(n, 9, posicoes(read_usize(pessoa + staff::PLAYER)) as i16 as i32 as u32);
            if tec_dono != 0 {
                cqc10_nomes(gravar, n, 0xa, tec_dono);
            } else {
                for k in 0xa..=0xd {
                    gravar(n, k, 0xffff_ffff);
                }
            }
            gravar(n, 0xe, (listada(pessoa, 0) != 0) as u32);
            gravar(n, 0xf, cnt as i32 as u32);
            // os interessados por reputação (do maior para o menor)
            let rep = |id: i32| read_i16(cpw6_clube(id) + 0x80);
            let c = cnt as usize;
            for a in 0..c.saturating_sub(1) {
                for b in a + 1..c {
                    if rep(ids[b]) > rep(ids[a]) {
                        ids.swap(a, b);
                    }
                }
            }
            for (k, &id) in ids.iter().take(c).enumerate() {
                gravar(n, 0x10 + k as i8, id as u32);
            }
            // os destinos
            let mut dest = [0usize; 30];
            let mut nd: u8 = if read_usize(G_TRANSFER_MANAGER) != 0 { cqc10_destinos(pessoa, 0xda, dest.as_mut_ptr(), 0x1e) } else { 0 };
            for &id in ids.iter().take(c) {
                if (nd as i8) >= 0x1e {
                    break;
                }
                cqc10_acrescentar(&mut dest, &mut nd, cpw6_clube(id));
            }
            let total = read_i32(0xae2364);
            for q in 0..16i32 {
                let h = read_usize(G_STAFF_TABLE).wrapping_add(((total - 1 - q) as usize).wrapping_mul(0x6e));
                let hc = read_usize(h + 0x39);
                if read_u8(h + 0x5f) == 0 || hc == 0 {
                    continue;
                }
                for &id in ids.iter().take(c) {
                    if (nd as i8) >= 0x1e {
                        break;
                    }
                    let ci = cpw6_clube(id);
                    let hc = read_usize(h + 0x39);
                    let junto = read_usize(ci + 0x57) == read_usize(hc + 0x57)
                        || (read_usize(ci + 0x53) == read_usize(hc + 0x53) && (read_i16(ci + 0x80) as i32 - 0xbb8) > read_i16(hc + 0x80) as i32);
                    if junto {
                        cqc10_acrescentar(&mut dest, &mut nd, hc);
                    }
                }
            }
            for &d in dest.iter().take((nd as i8).max(0) as usize) {
                ao_tecnico(read_usize(G_NEWS), n, d);
            }
            if pede {
                let reacao: unsafe extern "thiscall" fn(usize, u32, usize, usize, *mut i32, u32) = core::mem::transmute(FN_MEDIA_APPLY_SQUAD_REACTION_TO_PLAYER_STORY);
                reacao(this, 0x1f, pessoa, dono, ids.as_mut_ptr(), cnt as u8 as u32);
            }
            if dono != 0 && cqc10_humano(dono, 0, 1) == 0 && resp != 0 {
                if clube != 0 {
                    let rival: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_WARN_SQUAD_OF_POSITION_RIVAL);
                    rival(this, pessoa, clube, dono, resp);
                }
                let historia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_APPLY_PLAYER_STORY_RESPONSE);
                historia(this, pessoa, dono, 0x1f, resp);
            }
            i += 1;
            continue;
        }
        // estado 1: o dono da IA pode avisar que não vende
        if read_u8(r + 0x10) == 1 && dono != 0 && cqc10_humano(dono, 1, 1) == 0 {
            let (rd, ra) = (read_i16(r + 8), read_i16(r + 0xa) as u16);
            let (pd, pa) = (read_i16(pessoa + staff::DATE_JOINED_CLUB), read_i16(pessoa + 0x40) as u16);
            let depois = ra > pa || (ra == pa && rd > pd);
            if depois && vinc != 0 && read_u8(vinc + 0x49) == 0 && listada(pessoa, 0) == 0 {
                let feliz = read_i8(vinc + 0x3d) as i32;
                let s = cpe1_sorteio(0x32);
                if feliz > s + 0x28 && cpe1_sorteio(0x1e) == 0 {
                    crate::comparacao::ramo("cqc_10 nao vende");
                    let nao_vende: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(FN_NEWS_CREATE_NOT_FOR_SALE_STORY);
                    nao_vende(pessoa, dono, read_usize(dono + 0xbf), 0);
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

// ponytail: tira registros (o vetor), sorteia, constrói e entrega notícias e chama as reações e respostas (0x755340,
// 0x758010, 0x741020, 0x587600): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_process_transfer_interest(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_TRANSFER_INTEREST)(this)
}

/// news item 0xfbe that the club will not sell the player, with the chairman when he likes the player; to every manager
/// for a famous player, else to the interested human managers
/// Original: `news_create_not_for_sale_story` (0x587600, media.cpp, cdecl).
///
/// [pt] { 0x587600 } (cdecl, media.cpp no fluxo; european_cup.cpp no mapa, sem outro chamador; quadro SEH sem destrutor no
/// caminho normal): `news_create_not_for_sale_story(jogador, clube, presidente, outro)`: com ficha e clube, a notícia
/// 0xfbe (0x77bbe0) com o jogador, o outro clube (ou −1), o clube e um sorteio(3); o presidente, se gosta do jogador
/// (0x547810) ou com sorteio(+0x16 da ficha dele) acima de sorteio(20), senão −1 no 7; reputação (+0xd da ficha) acima de
/// 8000: a todos os técnicos (0x786780), senão aos humanos interessados (0x8d1da0 com 0xd8, até 30, 0x7861e0).
pub(crate) unsafe extern "C" fn news_create_not_for_sale_story(jogador: usize, clube: usize, presidente: usize, outro: usize) {
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let mut dest = [0usize; 30];
    let nd: u8 = if read_usize(G_TRANSFER_MANAGER) != 0 { cqc10_destinos(jogador, 0xd8, dest.as_mut_ptr(), 0x1e) } else { 0 };
    crate::comparacao::ramo("cqc_10 noticia nao vende");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0xfbe, 0);
    cqc10_nomes(gravar, n, 0, jogador);
    gravar(n, 4, if outro != 0 { read_usize(outro) as u32 } else { 0xffff_ffff });
    gravar(n, 5, read_usize(clube) as u32);
    let s = cpe1_sorteio(3);
    gravar(n, 6, s as u32);
    let cita = gosta(jogador, presidente) != 0 || (presidente != 0 && read_usize(presidente + 0x69) != 0 && {
        let a = cpe1_sorteio(read_i8(read_usize(presidente + 0x69) + 0x16) as i32);
        a > cpe1_sorteio(0x14)
    });
    if cita {
        cqc10_nomes(gravar, n, 7, presidente);
    } else {
        gravar(n, 7, 0xffff_ffff);
    }
    let f = read_usize(jogador + 0x61);
    if f != 0 && read_i16(f + 0xd) > 0x1f40 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        todos(read_usize(G_NEWS), n);
    } else {
        let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        for &d in dest.iter().take((nd as i8).max(0) as usize) {
            ao_tecnico(read_usize(G_NEWS), n, d);
        }
    }
}

// ponytail: a notícia vai aos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "C" fn comparar_news_create_not_for_sale_story(jogador: usize, clube: usize, presidente: usize, outro: usize) {
    orig::<unsafe extern "C" fn(usize, usize, usize, usize)>(IDX_NEWS_CREATE_NOT_FOR_SALE_STORY)(jogador, clube, presidente, outro)
}

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
pub(crate) unsafe extern "thiscall" fn queue_trophy_praise_story(_this: usize, clube: usize, comp: usize, marca: u32) {
    if clube == 0 || read_usize(clube + club::MANAGER) == 0 || comp == 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MEDIA_PICK_OUTLET_TYPE_FOR_CLUB);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(FN_PUNDITLIST_PICK_RANDOM_PUNDIT);
    let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    crate::comparacao::ramo("cqc_10 noticia do titulo");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x2e, 0);
    cqc10_nomes(gravar, n, 0, read_usize(clube + club::MANAGER));
    gravar(n, 4, read_usize(clube) as u32);
    gravar(n, 5, read_usize(comp) as u32);
    let s = cpe1_sorteio(2);
    gravar(n, 6, s as u32);
    let v = veiculo(clube);
    gravar(n, 7, v);
    let na = read_usize(clube + club::NATION);
    let c = comentarista(read_usize(G_PUNDIT_LIST), clube, v, if na != 0 { read_i32(na) } else { -1 });
    gravar(n, 8, c as u32);
    gravar(n, 9, marca as i8 as i32 as u32);
    let mm = read_usize(G_MANAGER_MANAGER);
    let conf = if mm != 0 {
        read_i16(read_usize(mm).wrapping_add((read_i32(clube) as usize).wrapping_mul(0x49)) + 6) as i32
    } else {
        0x1388
    };
    gravar(n, 0xa, conf as u32);
    gravar(n, 0xb, (read_usize(clube + club::DIVISION) == comp) as u32);
    let tec = read_usize(clube + club::MANAGER);
    let dias = ate(0xae2c90, read_usize(tec + 0x3e) as u32, read_usize(tec + 0x42) as u32);
    gravar(n, 0xc, dias as u32);
    if dias < 0xb4 {
        return;
    }
    gravar(n, 0xd, read_i16(clube + club::REPUTATION) as i32 as u32);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    ao_tecnico(read_usize(G_NEWS), n, clube);
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_queue_trophy_praise_story(this: usize, clube: usize, comp: usize, marca: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_QUEUE_TROPHY_PRAISE_STORY)(this, clube, comp, marca)
}

// ---------- 264a leva: a reação do elenco à história de um jogador, o aviso aos rivais de posição e o rótulo das respostas à mídia (media.cpp) ----------





/// O vínculo da `pessoa` com o `clube` (0x4cf5f0: thiscall em [0xae1050], 2 na pilha; grava [0xae1080]).
pub(crate) unsafe fn cqb8_vinculo(pessoa: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    f(read_usize(G_CONTRACT_MANAGER), pessoa, clube)
}

/// O contrato inteiro (+0 a +0x4f) da `pessoa` no `clube`, como região do comparador.
pub(crate) unsafe fn cqb8_contrato(pessoa: usize, clube: usize) -> Option<(usize, usize)> {
    let c = cqb8_vinculo(pessoa, clube);
    (c != 0).then_some((c, 0x50))
}

/// Uma rotina sem porta que por dentro chama units_digit da fita (0x544520 e 0x544600 chamam a 0x4cf5f0; a 0x62c400
/// alcança a 0x600a50 e portas): na comparação pela fita, a original grava essas chamadas internas no nível de cima
/// (com o comparador da rotina, a original e a cópia dele gravam cada uma a sua). A cópia roda a rotina numa fita de
/// rascunho e pede as mesmas chamadas por `chamar`, na ordem. Fora da comparação (sem GRAVAR_POS) só chama.
pub(crate) unsafe fn cqb8_pela_fita<R>(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, f: impl FnOnce() -> R) -> R {
    if !GRAVAR_POS.load(std::sync::atomic::Ordering::Relaxed) {
        return f();
    }
    let externa = fita().replace((Vec::new(), Vec::new()));
    let r = f();
    let gravadas = core::mem::replace(&mut *fita(), externa).map_or(Vec::new(), |g| g.1);
    for c in gravadas {
        chamar(c.0, c.1, c.2);
    }
    r
}

/// O vetor que a 0x755340 monta na pilha (58 dwords, ela percorre 57): os 50 do elenco (0x541f90 com 0, cdecl), o
/// +0xbf do clube, o técnico (+0xcf) só sem humano no clube (0x600a50 com 0, 1, 0, 0, cdecl, por `chamar`: unidade da
/// fita) e fora dos tipos 11 a 15 (0x53b620, cdecl, o byte +0x3d), o +0xd3 e os cinco dwords de +0x19f.
pub(crate) unsafe fn cqb8_vetor(clube: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> [usize; 58] {
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let tipo: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let mut v = [0usize; 58];
    reunir(clube, v.as_mut_ptr(), 0);
    v[50] = read_usize(clube + club::CHAIRMAN);
    let tecnico = read_usize(clube + club::MANAGER);
    v[51] = if chamar(0x600a50, 0, [clube as u32, 0, 1, 0, 0]) != 0 || (tecnico != 0 && tipo(*((tecnico + 0x3d) as *const u8) as u32) != 0) {
        0
    } else {
        tecnico
    };
    v[52] = read_usize(clube + club::ASSISTANT_MANAGER);
    for k in 0..5 {
        v[53 + k] = read_usize(clube + club::COACHES + 4 * k);
    }
    v
}

/// { 0x755340 } (thiscall, ret 0x14, media.cpp): `Media::apply_squad_reaction_to_player_story(tópico, jogador, clube,
/// ids, n)`, depois de uma história sobre o jogador (o boato de transferência 0x754a30 com o tópico 0x1f e os clubes
/// interessados; 0x72c1f0 e 0x733a60 sem ids). A chance começa em 10 (5 se o jogador é favorito do clube, 0x547790; 15
/// se rejeitado, 0x5477d0) e cresce por clube interessado: 100 no clássico de Glasgow, 10 se a liga tem o confronto
/// (0x4b02e0), 15 se o clube é rival dele (+0xb3/+0xb7/+0xbb), 5 se é da mesma divisão (fora da divisão inferior).
/// Sorteio(chance) acima de sorteio(10): o clube rejeita o jogador (0x547940 com 1). Listado (0x544520): −25; sem
/// vínculo: −15; senão −10 no fim de contrato (0x867d30) e mais a felicidade (+0x3d) / 10; no mínimo 5. Para cada uma
/// das 57 pessoas do vetor (cqb8_vetor) com preferências (+0x65): a nota (10, o +0x1d da ficha de técnico + 10, ou pelo
/// +0x5d, idade, +0x5b e lista a pedido do jogador do elenco), ajustada pelo tópico 0x20/0x1e, pelo +0x59/+0x5a/+0x5b,
/// por gostar ou não do jogador (0x547810/0x547840) e pelo jogador ser favorito ou rejeitado do clube, no mínimo 5;
/// sorteio(chance) acima de sorteio(nota): a pessoa passa a não gostar do jogador (0x547af0 com 1) e, com vínculo e
/// sorteio(nota) abaixo de sorteio(10), a marca 1 do contrato (0x867560) ou, nos tópicos 0x1e/0x20 ou com sorteio(nota)
/// zero, as queixas 0x800 (0x867700). As três com efeito vão por `chamar` (as portas do pós-jogo), e também as units_digit
/// da fita que a original chama: o vínculo 0x4cf5f0 (0x75551e, 0x7557dc), o humano 0x600a50 (0x755598) e as 0x4cf5f0
/// de dentro da 0x544520 (0x7554e8) e da 0x544600 (0x75567f, 0x7556ad), estas por `cqb8_pela_fita`.
pub(crate) unsafe fn cqb8_reacao_com(topico: u32, jogador: usize, clube: usize, ids: usize, n: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_CLUB_ADD_DISLIKED_STAFF);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_EXPIRY_BAND);
    let a_pedido: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LISTED_BY_REQUEST);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    if clube == 0 || jogador == 0 {
        return;
    }
    crate::comparacao::ramo("cqb_8 reacao do elenco");
    let mut chance: i32 = 0xa; // cqb8: a chance inicial
    if favorito(jogador, clube) != 0 {
        chance = 5;
    } else if rejeitado(jogador, clube) != 0 {
        chance = 0xf;
    }
    if ids != 0 {
        let n = n as i8 as i32;
        let mut k = 0i32;
        while k < n {
            let id = read_usize(ids + 4 * k as usize);
            let outro = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(0x245));
            k += 1;
            let (celtic, rangers) = (read_usize(G_KEY_CLUB_GLASGOW_CELTIC), read_usize(G_KEY_CLUB_GLASGOW_RANGERS));
            let meu = read_usize(clube);
            if (meu == celtic || meu == rangers) && (read_usize(outro) == celtic || read_usize(outro) == rangers) {
                chance += 100;
                continue;
            }
            if confronto(clube, outro) != 0 {
                chance += 10;
                continue;
            }
            if clube == read_usize(outro + 0xb3) || clube == read_usize(outro + 0xb7) || clube == read_usize(outro + 0xbb) {
                chance += 0xf;
                continue;
            }
            let div = read_usize(clube + club::DIVISION);
            if div != 0 && read_usize(div) != read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && div == read_usize(outro + 0x57) {
                chance += 5;
            }
        }
    }
    let r1 = cpe1_sorteio(chance);
    if r1 > cpe1_sorteio(10) {
        rejeitar(clube, jogador, 1);
    }
    // cqb8: a 0x4cf5f0 (de dentro da 0x544520 e direta) é unidade da fita: vai por `chamar`
    if cqb8_pela_fita(chamar, || listada(jogador, 0)) != 0 {
        chance -= 0x19;
    } else {
        let c = chamar(0x4cf5f0, read_usize(G_CONTRACT_MANAGER), [jogador as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 {
            chance -= 0xf;
        } else {
            if faixa(c) != 0 {
                chance -= 10;
            }
            chance += (read_i8(c + 0x3d) as i32 / 10) as i8 as i32;
        }
    }
    if chance < 5 {
        chance = 5;
    }
    let v = cqb8_vetor(clube, chamar);
    let topico = topico as u16;
    for &p in v.iter().take(57) {
        if p == 0 || p == jogador || read_usize(p + 0x65) == 0 {
            continue;
        }
        let mut nota: i8 = 10;
        let tecnico = read_usize(p + 0x69);
        if tecnico != 0 {
            nota = read_i8(tecnico + 0x1d).wrapping_add(10);
        } else if read_usize(p + 0x61) != 0 {
            if read_i8(jogador + 0x18) > 0x14 && read_i8(p + 0x18) < 0x17 && read_i16(read_usize(p + 0x61) + 0xb) < 0x109a {
                continue;
            }
            crate::comparacao::ramo("cqb_8 nota do jogador");
            nota = read_i8(p + 0x5d).wrapping_add(10); // cqb8: a nota do jogador do elenco
            if read_i8(p + 0x18) < 0x14 {
                nota = nota.wrapping_sub(5);
            }
            let m = read_i8(p + 0x5b);
            if m == 0x14 {
                nota = nota.wrapping_add(1);
            } else if m == 1 {
                nota = nota.wrapping_sub(if cqb8_pela_fita(chamar, || a_pedido(p, 0)) != 0 { 0xf } else { 10 });
                if read_i16(read_usize(p + 0x61) + 0xb) > 0x1676 {
                    nota = nota.wrapping_sub(10);
                }
            } else if m < 6 {
                nota = nota.wrapping_sub(if cqb8_pela_fita(chamar, || a_pedido(p, 0)) != 0 { 10 } else { 5 });
                if read_i16(read_usize(p + 0x61) + 0xb) > 0x1676 {
                    nota = nota.wrapping_sub(5);
                }
            }
        }
        if topico == 0x20 && read_i8(p + 0x59) > 0xf && read_i8(p + 0x5d) < 6 {
            nota = nota.wrapping_sub(5);
        }
        let a59 = read_i8(p + 0x59);
        if a59 > 0xf {
            if read_i8(p + 0x5b) != 0x14 {
                nota = nota.wrapping_sub(2);
            }
        } else if a59 < 5 {
            nota = nota.wrapping_add(2);
        }
        if topico == 0x1e {
            let (a5b, a5a) = (read_i8(p + 0x5b), read_i8(p + 0x5a));
            if a5b > 0xf {
                if a5a < 0xf {
                    nota = nota.wrapping_sub(1);
                }
            } else if a5b >= 6 {
                nota = nota.wrapping_sub(if a5a >= 0xf { 1 } else { 3 });
            } else {
                nota = nota.wrapping_sub(if a5a < 6 { 10 } else { 5 });
            }
        }
        if gosta(jogador, p) != 0 {
            nota = nota.wrapping_add(0x19);
        } else if nao_gosta(jogador, p) != 0 {
            nota = nota.wrapping_add(read_i8(jogador + 0x5b).wrapping_sub(0x19));
            if nota > 0xf {
                nota = 0xf;
            }
        }
        if favorito(jogador, clube) != 0 {
            nota = nota.wrapping_add(5);
        } else if rejeitado(jogador, clube) != 0 {
            nota = nota.wrapping_sub(5);
        }
        if nota < 5 {
            nota = 5;
        }
        let r1 = cpe1_sorteio(chance);
        if r1 > cpe1_sorteio(nota as i32) { // cqb8: o desafeto sorteado
            crate::comparacao::ramo("cqb_8 desafeto sorteado");
            chamar(0x547af0, 0, [p as u32, jogador as u32, 1, 0, 0]);
            let c = chamar(0x4cf5f0, read_usize(G_CONTRACT_MANAGER), [p as u32, clube as u32, 0, 0, 0]) as usize;
            if c != 0 {
                let r = cpe1_sorteio(nota as i32);
                if r < cpe1_sorteio(10) {
                    if topico != 0x1e && topico != 0x20 && cpe1_sorteio(nota as i32) != 0 {
                        chamar(0x867560, c, [1, 0, 0, 0, 0]);
                    } else {
                        chamar(0x867700, c, [0x800, 0, 0, 0, 0]);
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
pub(crate) unsafe extern "thiscall" fn apply_squad_reaction_to_player_story(_this: usize, topico: u32, jogador: usize, clube: usize, ids: usize, n: u32) {
    cqb8_reacao_com(topico, jogador, clube, ids, n, &mut |r, t, a| despachar_pos(r, t, a))
}

/// Comparador: pela fita das portas do pós-jogo (comparar_pos): as desafeições (0x547af0), a marca (0x867560) e as
/// queixas (0x867700) rodam uma vez, na original, e a cópia confere a chamada e recebe o estado de depois. Regiões: o
/// sorteador, [0xae1080], as seis pessoas do clube (+0x9b, a 0x547940), o contrato do jogador e, de cada pessoa do
/// vetor, as preferências (+0x1c..+0x33), o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
pub(crate) unsafe extern "thiscall" fn comparar_apply_squad_reaction_to_player_story(this: usize, topico: u32, jogador: usize, clube: usize, ids: usize, n: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, u32)>(IDX_APPLY_SQUAD_REACTION_TO_PLAYER_STORY);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if clube != 0 && jogador != 0 {
        reg.push((clube + club::FAV_STAFF1, 24));
        reg.extend(cqb8_contrato(jogador, clube));
        for &p in cqb8_vetor(clube, &mut |r, t, a| despachar_pos(r, t, a)).iter().take(57) {
            if p == 0 {
                continue;
            }
            let pref = read_usize(p + 0x65);
            if pref != 0 {
                reg.push((pref + 0x1c, 24));
            }
            reg.push((p + 0x58, 6));
            reg.extend(cqb8_contrato(p, clube));
        }
    }
    comparar_pos("apply_squad_reaction_to_player_story", reg, || o(this, topico, jogador, clube, ids, n),
        |chamar| cqb8_reacao_com(topico, jogador, clube, ids, n, chamar));
}

/// { 0x758010 } (thiscall, ret 0x10, media.cpp): `Media::warn_squad_of_position_rival(jogador, clube, _, _)`, pelo
/// boato de transferência (0x754a30) e pela resposta à história do jogador (0x741020). Com ficha, clube com mais de 2
/// do elenco na máscara da posição dele (0x548150 com 0x5481a0), divisão com competição ([0xadadfc]) e reputação
/// (+0xb da ficha) acima de 4750: cada um dos 50 do elenco (+0xd7) com ficha, vínculo (0x4cf5f0) de felicidade (+0x3d)
/// positiva, o jogador sem lesão de estado 2 (0x62c400 em [0xb63d28]), sem as marcas 0x101 (+0x45), fora da lista
/// (0x544520), acima de 20 anos, do tipo 0x10/0x20/0x30 (+0x4f & 0xf0), com +0x5a abaixo de n + 5 e +0x5b abaixo de
/// n + 10, sorteio(+0x5a) abaixo de sorteio(n), a mesma máscara e reputação acima de 3750: a primeira vez, a história
/// 0x25 dele (0x72c1f0 em [0xb6b7c8]); sem a marca 0x800 (0x8676f0), a marca 0x800 (0x867560). As duas com efeito vão
/// por `chamar`, e também o vínculo 0x4cf5f0 (0x7580c2, unidade da fita) e, por `cqb8_pela_fita`, o que a 0x62c400
/// (0x7580ee) e a 0x544520 (0x75810a) chamam de units_digit por dentro.
pub(crate) unsafe fn cqb8_rival_com(jogador: usize, clube: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    let contar: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_CLUB_COUNT_SQUAD_WITH_POSITION_MASK);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let bandeiras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let n = contar(clube, mascara(jogador)) as i8;
    if n <= 2 {
        return;
    }
    let div = read_usize(clube + club::DIVISION);
    if div == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(div)) == 0 {
        return;
    }
    if read_i16(read_usize(jogador + 0x61) + 0xb) <= 0x128e {
        return;
    }
    crate::comparacao::ramo("cqb_8 rival de posicao");
    let n = n as i32;
    let mut postou = false;
    for i in 0..50 {
        let p = read_usize(clube + club::SQUAD + 4 * i);
        if p == 0 || read_usize(p + 0x61) == 0 || p == jogador {
            continue;
        }
        // cqb8: a 0x4cf5f0 é unidade da fita (direta e de dentro da 0x544520; a 0x62c400 alcança a 0x600a50)
        let c = chamar(0x4cf5f0, read_usize(G_CONTRACT_MANAGER), [p as u32, clube as u32, 0, 0, 0]) as usize;
        if c == 0 || read_usize(p + 0x61) == 0 || read_i8(c + 0x3d) <= 0 {
            continue;
        }
        if cqb8_pela_fita(chamar, || lesao(read_usize(G_INJURY), jogador)) == 2 || read_usize(c + 0x45) & 0x101 != 0
            || cqb8_pela_fita(chamar, || listada(p, clube)) != 0
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
        let r1 = cpe1_sorteio(n);
        if cpe1_sorteio(a5a) >= r1 {
            continue;
        }
        let m = mascara(jogador);
        if m != mascara(p) || read_i16(read_usize(p + 0x61) + 0xb) <= 0xea6 {
            continue;
        }
        if !postou {
            chamar(0x72c1f0, read_usize(G_MEDIA), [p as u32, clube as u32, 0x25, 0, jogador as u32]);
            postou = true;
        }
        if bandeiras(c) & 0x800 != 0 {
            continue;
        }
        chamar(0x867560, c, [0x800, 0, 0, 0, 0]);
    }
}

/// with a high-reputation player linked to the club and more than two squad players in his position: the unsettled
/// position rivals (by ambition and loyalty, by a draw) get contract flag 0x800, and the first one gives story 0x25
/// Original: `Media::warn_squad_of_position_rival` (0x758010, media.cpp, thiscall).
///
/// [pt] com o jogador de reputação alta ligado ao clube e mais de dois do elenco na posição dele: os rivais de posição
/// insatisfeitos (pela ambição e pela lealdade, por sorteio) ganham a marca 0x800 no contrato, e o primeiro dá a
/// história 0x25
pub(crate) unsafe extern "thiscall" fn warn_squad_of_position_rival(_this: usize, jogador: usize, clube: usize, _a: u32, _b: u32) {
    cqb8_rival_com(jogador, clube, &mut |r, t, a| despachar_pos(r, t, a))
}

/// Comparador: pela fita das portas do pós-jogo (a história 0x72c1f0 e a marca 0x867560 rodam só na original). Regiões:
/// o sorteador, [0xae1080] e, de cada um dos 50 do elenco, o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
pub(crate) unsafe extern "thiscall" fn comparar_warn_squad_of_position_rival(this: usize, jogador: usize, clube: usize, a: u32, b: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_WARN_SQUAD_OF_POSITION_RIVAL);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if clube != 0 {
        for i in 0..50 {
            let p = read_usize(clube + club::SQUAD + 4 * i);
            if p != 0 {
                reg.push((p + 0x58, 6));
                reg.extend(cqb8_contrato(p, clube));
            }
        }
    }
    comparar_pos("warn_squad_of_position_rival", reg, || o(this, jogador, clube, a, b), |chamar| cqb8_rival_com(jogador, clube, chamar));
}

/// As respostas 1 e 2 de cada caso da tabela de saltos da 0x735160 (a 3ª, de todos, é "No Comment", 0xa27808); o caso
/// vem da tabela de bytes do exe em 0x7356f4 (tipo − 3, até 0x7f; acima, o 21).
pub(crate) const CQB8_RESPOSTAS: [(usize, usize); 22] = [
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
pub(crate) unsafe extern "stdcall" fn media_format_response_option_label(tipo: u32, opcao: u32, saida: usize, tamanho: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if saida == 0 || (tamanho as i16) < 0xf {
        return;
    }
    let k = (tipo as i16 as i32).wrapping_sub(3) as u32;
    let caso = if k > 0x7f { 21 } else { *((0x7356f4 + k as usize) as *const u8) as usize };
    let (um, dois) = CQB8_RESPOSTAS[caso];
    let texto = match opcao as i8 {
        1 => um,
        2 => dois,
        _ => 0xa27808,
    };
    formatar(saida, texto);
}

/// Comparador: lado a lado nos primeiros bytes da saída (até 0x50, o maior rótulo tem 61).
pub(crate) unsafe extern "stdcall" fn comparar_media_format_response_option_label(tipo: u32, opcao: u32, saida: usize, tamanho: u32) {
    let o = orig::<unsafe extern "stdcall" fn(u32, u32, usize, u32)>(IDX_MEDIA_FORMAT_RESPONSE_OPTION_LABEL);
    let reg = if saida != 0 && (tamanho as i16) >= 0xf { vec![(saida, (tamanho as u16 as usize).min(0x50))] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(tipo, opcao, saida, tamanho), || media_format_response_option_label(tipo, opcao, saida, tamanho));
    PLACAR.registrar(igual, || format!("media_format_response_option_label({}, {}) {:x?} x {:x?}", tipo as i16, opcao as i8, mo, mr));
}

// ---------- 263a leva: as reações dos elencos à transferência: os contratos dos dois elencos, a comemoração da contratação e o companheiro desassossegado pela venda (media.cpp) ----------






/// A pessoa do id (0x6e bytes em [0xae23bc]).
pub(crate) unsafe fn cqa13_pessoa(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(0x6e) as usize)
}


/// A máscara da posição natural da pessoa (0x5481a0).
pub(crate) unsafe fn cqa13_mascara(pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    f(pessoa)
}

/// As marcas de evento do contrato (+0x40, 0x8676f0).
pub(crate) unsafe fn cqa13_bandeiras(c: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    f(c)
}

/// Apaga marcas de evento do contrato (0x8676d0).
pub(crate) unsafe fn cqa13_desmarcar(c: usize, marcas: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_FLAGS);
    f(c, marcas);
}

/// A reputação atual do jogador (+0xb da ficha +0x61).
pub(crate) unsafe fn cqa13_rep(pessoa: usize) -> i16 {
    read_i16(read_usize(pessoa + staff::PLAYER) + 0xb)
}

/// O ensaio do comparador da 0x758200: a cópia roda sem nada que aloque, dê notícia ou peça transferência; quando chega
/// a um desses, `CQA13_PAROU` e ela para (a comparação fica só com a original).
pub(crate) static mut CQA13_ENSAIO: bool = false;
pub(crate) static mut CQA13_PAROU: bool = false;

/// Liga marcas no contrato (STAFF_CONTRACT::set_flags 0x867560). No ensaio, pela cópia em ensaio dela (CPW7_ENSAIO): se
/// ela pediria transferência, para. Devolve se parou.
pub(crate) unsafe fn cqa13_marcas(c: usize, marcas: u32) -> bool {
    if CQA13_ENSAIO {
        CPW7_ENSAIO = true;
        CPW7_DECISAO = 0;
        set_flags(c, marcas);
        CPW7_ENSAIO = false;
        if CPW7_DECISAO != 0 {
            CQA13_PAROU = true;
        }
        return CQA13_PAROU;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_SET_FLAGS);
    f(c, marcas);
    false
}

/// A decisão de pedir transferência da 0x867700 com as `marcas` (sem efeito: o +0x45 volta): funções 0xb, 0xf e 0xd,
/// fora o técnico e o auxiliar do clube do contrato, moral (+0x44) não negativa, sem +0x49, quer sair (0x866f60, com as
/// marcas já ligadas), sem transferência futura (0x8cca10), sem os bits 1 e 8 de +0x4e, fora da fila da transferência
/// livre (0x8c1870) e com mais de 20 anos ou reputação acima de 4250.
pub(crate) unsafe fn cqa13_pediria(c: usize, marcas: u32) -> bool {
    let pessoa = cqa13_pessoa(read_i32(c));
    let funcao = *((pessoa + staff::JOB_FOR_CLUB) as *const u8);
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
    let antes = read_usize(c + 0x45);
    core::ptr::write_unaligned((c + 0x45) as *mut usize, antes | marcas as usize);
    let quer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    let sair = quer(c) != 0;
    core::ptr::write_unaligned((c + 0x45) as *mut usize, antes);
    if !sair {
        return false;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if futura(read_usize(G_TRANSFER_MANAGER), pessoa, 0) != 0 || *((c + 0x4e) as *const u8) & 9 != 0 {
        return false;
    }
    let na_fila: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_QUEUED_FOR_FREE_TRANSFER);
    if na_fila(read_usize(G_TRANSFER_MANAGER), pessoa) != 0 {
        return false;
    }
    !(read_i8_as_i32(pessoa + staff::YEAR_OF_BIRTH) <= 0x14 && cqa13_rep(pessoa) <= 0x109a)
}

/// As queixas do contrato (STAFF_CONTRACT::apply_event_flags 0x867700 com a pessoa). No ensaio, para antes se ela
/// pediria transferência. Devolve se parou.
pub(crate) unsafe fn cqa13_eventos(c: usize, marcas: u32, pessoa: usize) -> bool {
    if CQA13_ENSAIO && cqa13_pediria(c, marcas) {
        CQA13_PAROU = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    f(c, marcas, pessoa);
    false
}

/// Tira queixas do contrato (STAFF_CONTRACT::clear_grievances 0x8674e0). No ensaio, para antes se o pedido de
/// transferência seria retirado (a decisão `cph1_retira` com as queixas já tiradas). Devolve se parou.
pub(crate) unsafe fn cqa13_limpar(c: usize, queixas: u32) -> bool {
    if CQA13_ENSAIO {
        let antes = read_usize(c + 0x45);
        core::ptr::write_unaligned((c + 0x45) as *mut usize, antes & !(queixas as usize));
        let retira = cph1_retira(c).is_some();
        core::ptr::write_unaligned((c + 0x45) as *mut usize, antes);
        if retira {
            CQA13_PAROU = true;
            return true;
        }
    }
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
    f(c, queixas);
    false
}

/// A comemoração da contratação (0x75b6e0); no ensaio, para.
pub(crate) unsafe fn cqa13_comemorar(this: usize, p: usize, clube: usize, tipo: u32, porta_voz: usize) -> bool {
    if CQA13_ENSAIO {
        CQA13_PAROU = true;
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) = core::mem::transmute(FN_MEDIA_QUEUE_SIGNING_CELEBRATION_STORY);
    f(this, p, clube, tipo, 1, porta_voz);
    false
}

/// O porta-voz do elenco: entre os 50 do clube com ficha, fora `p`, com reputação acima de 4250, o maior sorteio(reputação
/// + 20 × (+0x5b + +0x59) ± pela idade: abaixo de 19 −1500, de 23 −750, de 28 −250, de 34 +250), acima de 0.
pub(crate) unsafe fn cqa13_porta_voz(clube: usize, p: usize) -> usize {
    let (mut melhor, mut quem) = (0i32, 0usize);
    for j in 0..50usize {
        let m = read_usize(clube + club::SQUAD + 4 * j);
        if m == 0 || read_usize(m + 0x61) == 0 || m == p {
            continue;
        }
        let r = cqa13_rep(m);
        if r <= 0x109a {
            continue;
        }
        let mut x = (r as i32).wrapping_add((read_i8_as_i32(m + 0x5b) + read_i8_as_i32(m + 0x59)) * 20);
        let idade = read_i8_as_i32(m + 0x18);
        x += if idade < 0x13 {
            -0x5dc
        } else if idade < 0x17 {
            -0x2ee
        } else if idade < 0x1c {
            -0xfa
        } else if idade < 0x22 {
            0xfa
        } else {
            0
        };
        let s = cpe1_sorteio(x);
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
pub(crate) unsafe extern "thiscall" fn process_transfer_squad_reactions(this: usize, p: usize, novo: usize, velho: usize, flag: u32) {
    if p == 0 || novo == 0 {
        return;
    }
    let ficha = read_usize(p + 0x61);
    if ficha == 0 || read_i8_as_i32(p + 0x18) < 0x13 || read_i16(ficha + 0xb) < 0x109a {
        return;
    }
    let mut mudou = false;
    // (A) a história da mídia na frente da fila
    if read_i32(this + 8) > 0 {
        let h = read_usize(this);
        if read_i32(h + 0x13) == 0xbd0 && read_i16(h + 0x11) == 0x25 && read_i32(novo) == 0x25 {
            let tipo = *((h + 0x10) as *const u8);
            let id = read_i32(h + 8);
            if id != -1 {
                let q = cqa13_pessoa(id);
                let rp = cqa13_rep(p);
                if cqa13_mascara(p) == cqa13_mascara(q) && (rp > 0x128e || rp as i32 - 0x5dc > cqa13_rep(q) as i32) {
                    let c = cqb8_vinculo(q, novo);
                    if c != 0 {
                        crate::comparacao::ramo("cqa_13 historia da midia");
                        mudou = true;
                        if tipo == 1 {
                            if cqa13_eventos(c, 0x8000000, 0) || cqa13_eventos(c, 0x80000000, p) {
                                return;
                            }
                        } else if tipo == 2 {
                            if cqa13_bandeiras(c) & 0x800 != 0
                                && (cqa13_marcas(c, 0x200) || cqa13_eventos(c, 0x80000000, p) || cqa13_marcas(c, 0x80))
                            {
                                return;
                            }
                        } else if cpe1_sorteio(read_i8_as_i32(q + 0x5b)) < cpe1_sorteio(10) {
                            if cqa13_marcas(c, 0x80) {
                                return;
                            }
                            if cpe1_sorteio(read_i8_as_i32(q + 0x5a)) == 0 && cqa13_eventos(c, 0x80000000, p) {
                                return;
                            }
                        }
                        if cqa13_bandeiras(c) & 0x800 != 0 {
                            cqa13_desmarcar(c, 0x800);
                        }
                    }
                }
            }
        }
    }
    // (B) o elenco do novo clube
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let mesma_posicao: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_CLUB_COUNT_SQUAD_WITH_POSITION_MASK);
    let forca = contar(novo, 0, 0) as i8 as i32;
    let rn = read_i16(novo + 0x80);
    let limite = if rn < 0x9c4 { 0x14 } else if rn < 0x1482 { 0x19 } else if rn >= 0x1c52 { 0x20 } else { 0x1e }; // cqa13: o limite do elenco forte
    let mesma = mesma_posicao(novo, cqa13_mascara(p)) as i8 as i32;
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let categoria: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    for j in 0..50usize {
        let m = read_usize(novo + 0xd7 + 4 * j);
        if m == 0 || read_usize(m + 0x61) == 0 || m == p {
            continue;
        }
        if dias(0xae2c90, read_usize(m + 0x3e) as u32, read_usize(m + 0x42) as u32) <= read_i8_as_i32(m + 0x5a) + 0xf {
            continue;
        }
        let c = cqb8_vinculo(m, novo);
        if c == 0 {
            continue;
        }
        if cqa13_bandeiras(c) & 0x800 != 0 && cqa13_mascara(p) == cqa13_mascara(m) {
            cqa13_desmarcar(c, 0x800);
            if cqa13_eventos(c, 0x80000000, p) {
                return;
            }
            if cpe1_sorteio(read_i8_as_i32(m + 0x5a)) == 0 && cqa13_marcas(c, 0x200) {
                return;
            }
            if cpe1_sorteio(read_i8_as_i32(m + 0x5b)) == 0 && cqa13_marcas(c, 0x80) {
                return;
            }
        }
        if cqa13_bandeiras(c) & 0x400 != 0 && forca >= limite {
            crate::comparacao::ramo("cqa_13 elenco forte");
            cqa13_desmarcar(c, 0x400);
        }
        if mesma > 2 && cqa13_bandeiras(c) & 0x400 == 0 && cqa13_rep(m) > 0x109a && cqa13_mascara(p) == cqa13_mascara(m) {
            let rp = cqa13_rep(p);
            if rp > 0x128e || rp as i32 - 0x5dc > read_i16(novo + 0x80) as i32 {
                let a = read_i8_as_i32(m + 0x5a);
                if a < 0xf && read_i8_as_i32(m + 0x5b) < 0xf && a < mesma + 4 && read_i8_as_i32(m + 0x57) < mesma + 8 && cpe1_sorteio(a) < cpe1_sorteio(6) {
                    let parou = if categoria(m, c) == 1 || categoria(m, c) == 2 {
                        cqa13_eventos(c, 0x80000000, p)
                    } else {
                        cqa13_marcas(c, 0x200)
                    };
                    if parou {
                        return;
                    }
                }
            }
        }
        if read_i32(c + 0x45) & 0x20000000 != 0 {
            let rp = cqa13_rep(p);
            if rp > 0x1482 || rp as i32 - 0x4e2 > read_i16(novo + 0x80) as i32 {
                mudou = true;
                if forca > 0x28 || cpe1_sorteio(read_i8_as_i32(m + 0x5a)) != 0 {
                    crate::comparacao::ramo("cqa_13 queixa limpa");
                    if cqa13_limpar(c, 0x20000000) { // cqa13: chega alguém maior
                        return;
                    }
                }
            }
        }
    }
    let ficha = read_usize(p + 0x61);
    if read_i16(ficha + 0xd) > 0x186a {
        let rp = read_i16(ficha + 0xb);
        if rp as i32 + 0x2ee > read_i16(novo + 0x80) as i32 || rp > 0x222e {
            let linha = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(novo) as u32).wrapping_mul(0x49) as usize);
            let x = read_i16(linha + 8).wrapping_add(0x1f4); // cqa13: a torcida comemora
            let x = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            crate::comparacao::ramo("cqa_13 torcida comemora");
            core::ptr::write_unaligned((linha + 8) as *mut i16, x);
            if cqa13_comemorar(this, p, novo, 1, 0) {
                return;
            }
        }
    }
    if mudou {
        let ficha = read_usize(p + 0x61);
        if read_i16(ficha + 0xd) > 0x186a
            && read_i16(ficha + 0xb) as i32 + 0x2ee > read_i16(novo + 0x80) as i32
            && read_i8_as_i32(p + 0x5b) > 0xa
            && read_i8_as_i32(p + 0x59) > 6
            && cpe1_sorteio(3) != 0
        {
            let quem = cqa13_porta_voz(novo, p);
            if quem != 0 && cqa13_comemorar(this, p, novo, 2, quem) {
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
    let forca = contar(velho, 0, 0) as i8 as i32;
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
    let mesma = mesma_posicao(velho, cqa13_mascara(p)) as i8 as i32;
    let candidato: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_STAFF_IS_BEST_FAVOURED_CANDIDATE);
    for j in 0..50usize {
        let m = read_usize(velho + 0xd7 + 4 * j);
        if m == 0 || read_usize(m + 0x61) == 0 || m == p {
            continue;
        }
        let c = cqb8_vinculo(m, velho);
        if c == 0 {
            continue;
        }
        let mut q: i32 = if read_i32(c + 0x45) != 0 { 3 } else { 0 };
        if cqa13_bandeiras(c) != 0 {
            q += 1;
        }
        if read_i8_as_i32(c + 0x44) < -0x19 {
            q += 1;
        }
        let q = q as i8 as i32;
        let mut queixa = 0u32; // 0x40000000 ou 0x20000000 pela saída do companheiro
        let mut segundo = true;
        if flag as u8 != 0
            && cqa13_bandeiras(c) & 0x200 == 0
            && cqa13_bandeiras(c) & 0x800 == 0
            && read_i32(c + 0x45) as u32 & 0x80000000 == 0
            && (forca < 0x28 || mesma < 6)
            && (cqa13_mascara(p) != cqa13_mascara(m) || mesma < 2)
        {
            let rp = cqa13_rep(p);
            if rp > 0x1e46 || rp > read_i16(novo + 0x80).wrapping_sub(0x2ee) {
                let r = cpe1_sorteio(6);
                if read_i8_as_i32(m + 0x57) + q > r + 10 && read_i8_as_i32(c + 0x3d) > 0x19 && cqa13_rep(m) > 0x109a
                    && cpe1_sorteio(read_i8_as_i32(m + 0x5b)) < q + 5 && cpe1_sorteio(read_i8_as_i32(m + 0x5a)) == 0
                {
                    segundo = false;
                    queixa = if read_i8_as_i32(p + 0x18) < 0x18 && read_i16(read_usize(p + 0x61) + 0xd) < 0x128e { 0x40000000 } else { 0x20000000 };
                }
            }
        }
        if segundo
            && read_i8_as_i32(p + 0x18) > 0x17
            && flag as u8 != 0
            && candidato(m, novo, p) != 0
            && cqa13_mascara(p) != cqa13_mascara(m)
            && cpe1_sorteio(read_i8_as_i32(m + 0x5b)) < q + 5
        {
            let rp = cqa13_rep(p);
            if rp > 0x1e46 || rp > read_i16(novo + 0x80).wrapping_sub(0x2ee) {
                queixa = 0x20000000;
            } else {
                crate::comparacao::ramo("cqa_13 moral cai");
                let x = read_i8_as_i32(c + 0x44) - 0x19; // cqa13: a moral do companheiro cai
                *((c + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { x as i8 };
            }
        }
        if queixa != 0 {
            if cqa13_eventos(c, queixa, p) {
                return;
            }
            mudou = true;
        }
        if read_i8_as_i32(m + 0x18) > 0x17 && cqa13_bandeiras(c) & 0x400 == 0 && read_i8_as_i32(m + 0x57) > 0xf {
            let r = (cpe1_sorteio(10) as u8).wrapping_add(10) as i8;
            if *((m + 0x58) as *const i8) > r && read_i8_as_i32(m + 0x5b) > 0xa && cqa13_rep(p) > 0x109a && cqa13_rep(m) > 0x109a && forca < lim1 && cqa13_marcas(c, 0x400) {
                return;
            }
        }
        let iguais = || cqa13_mascara(p) == cqa13_mascara(m);
        if (forca < lim2 || mesma < 2) && iguais() && cqa13_bandeiras(c) & 0x200 != 0 {
            cqa13_desmarcar(c, 0x200);
        }
        let tirar_800 = if forca < lim2 {
            let r1 = cpe1_sorteio(read_i8_as_i32(p + 0x5a));
            (r1 > cpe1_sorteio(10) && iguais()) || mesma < 2
        } else {
            mesma < 2
        };
        if tirar_800 && cqa13_bandeiras(c) & 0x800 != 0 {
            cqa13_desmarcar(c, 0x800);
        }
        if ((forca < lim2 && iguais()) || mesma < 2) && read_i32(c + 0x45) as u32 & 0x80000000 != 0 && cqa13_limpar(c, 0x80000000) {
            return;
        }
        if cqa13_bandeiras(c) & 0x400 != 0 {
            if read_i32(c + 0x45) as u32 & 0x80000000 != 0 && cqa13_limpar(c, 0x80000000) {
                return;
            }
            if cqa13_bandeiras(c) & 0x800 != 0 {
                cqa13_desmarcar(c, 0x800);
            }
            if cqa13_bandeiras(c) & 0x200 != 0 {
                cqa13_desmarcar(c, 0x200);
            }
        }
    }
    if !mudou || read_i8_as_i32(p + 0x18) >= 0x23 {
        return;
    }
    let ficha = read_usize(p + 0x61);
    if read_i16(ficha + 5) > 0x64
        && read_i16(ficha + 0xd) > 0x186a
        && read_i16(ficha + 0xb) as i32 + 0x2ee > read_i16(velho + 0x80) as i32
        && read_i8_as_i32(p + 0x5b) > 0xa
        && read_i8_as_i32(p + 0x59) > 6
        && cpe1_sorteio(3) != 0
    {
        let quem = cqa13_porta_voz(velho, p);
        if quem != 0 {
            if CQA13_ENSAIO {
                CQA13_PAROU = true;
                return;
            }
            let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) = core::mem::transmute(FN_MEDIA_QUEUE_SALE_UNREST_STORY);
            f(this, velho, novo, p, quem);
        }
    }
}

/// Comparador em ensaio: a cópia sem notícia nem pedido de transferência (para quando chega a um) e a original a partir
/// do mesmo estado: o sorteio, [0xae1080] (a ficha do último contrato), a data estática da 0x54cdb0, a linha da torcida
/// do novo clube em [[0xb67a44]] e, de cada um dos elencos dos dois clubes (e da pessoa da história da mídia), o contrato
/// com o clube (0x50 bytes) e a linha de opiniões em [0xae2d0c] (0x4f bytes). Quando a cópia para: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_transfer_squad_reactions(this: usize, p: usize, novo: usize, velho: usize, flag: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(IDX_PROCESS_TRANSFER_SQUAD_REACTIONS);
    if p == 0 || novo == 0 || read_usize(p + 0x61) == 0 || read_i8_as_i32(p + 0x18) < 0x13 || cqa13_rep(p) < 0x109a {
        return o(this, p, novo, velho, flag);
    }
    let ficha = read_usize(G_CONTRACT_LOOKUP_RESULT);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    reg.push((0xae2470, 8));
    if read_usize(G_MANAGER_MANAGER) != 0 {
        reg.push((read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(novo) as u32).wrapping_mul(0x49) as usize), 0x49));
    }
    let mut juntar = |m: usize, clube: usize| {
        if m == 0 {
            return;
        }
        let c = cqb8_vinculo(m, clube);
        if c != 0 {
            reg.push((c, 0x50));
        }
        reg.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(m) as u32).wrapping_mul(0x4f) as usize), 0x4f));
    };
    for clube in [novo, velho] {
        if clube != 0 {
            for j in 0..50usize {
                juntar(read_usize(clube + club::SQUAD + 4 * j), clube);
            }
        }
    }
    if read_i32(this + 8) > 0 && read_i32(read_usize(this) + 8) != -1 {
        juntar(cqa13_pessoa(read_i32(read_usize(this) + 8)), novo);
    }
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, ficha); // os contratos lidos acima não contam
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA13_ENSAIO = true;
    CQA13_PAROU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| process_transfer_squad_reactions(this, p, novo, velho, flag)));
    CQA13_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    if CQA13_PAROU {
        crate::comparacao::ramo("cqa_13 reacoes so na original");
        return o(this, p, novo, velho, flag);
    }
    o(this, p, novo, velho, flag);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("process_transfer_squad_reactions(jogador {:#x}, novo {:#x}, velho {:#x}) diferencas em {:x?}", p, novo, velho, dif)
    });
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
pub(crate) unsafe extern "thiscall" fn queue_signing_celebration_story(this: usize, p: usize, clube: usize, tipo: u32, a: u32, porta_voz: usize) {
    if p == 0 || read_usize(p + 0x1a) == 0 {
        return;
    }
    let atual = read_usize(p + 0x39);
    if atual == 0 || read_usize(atual + 0xcf) == 0 || read_usize(atual + 0x69) == 0 || read_usize(p + 0x61) == 0 {
        return;
    }
    if porta_voz == 0 && tipo as u8 == 2 {
        return;
    }
    crate::comparacao::ramo("cqa_13 noticia da comemoracao");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MEDIA_PICK_OUTLET_TYPE_FOR_PERSON);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(FN_PUNDITLIST_PICK_RANDOM_PUNDIT);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let resposta: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_MEDIA_PICK_AI_MANAGER_RESPONSE);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbe5, 0);
    gravar(nt, 0, read_usize(p) as u32);
    for k in 1..4usize {
        gravar(nt, k as i8, read_usize(read_usize(p + 4 * k) + 0x33) as u32);
    }
    gravar(nt, 4, read_usize(read_usize(p + 0x39)) as u32);
    gravar(nt, 5, cpe1_sorteio(2) as u32);
    let v = veiculo(p);
    gravar(nt, 6, v);
    let nacao = read_usize(clube + club::NATION);
    gravar(nt, 7, comentarista(read_usize(G_PUNDIT_LIST), clube, v, if nacao != 0 { read_i32(nacao) } else { -1 }) as u32);
    gravar(nt, 8, posicoes(read_usize(p + 0x61)) as i16 as i32 as u32);
    gravar(nt, 9, tipo as u8 as i8 as i32 as u32);
    let mut r = 0u32;
    if humano(read_usize(p + 0x39), 0, 1, 0, 0) == 0 {
        r = resposta(this, read_usize(read_usize(p + 0x39) + 0xcf), p, 0x82);
    }
    gravar(nt, 10, r);
    let mut k: i8 = 0xb;
    if tipo as u8 == 2 {
        gravar(nt, 0xb, read_usize(porta_voz) as u32);
        for j in 1..4usize {
            gravar(nt, 0xb + j as i8, read_usize(read_usize(porta_voz + 4 * j) + 0x33) as u32);
        }
        k = 0xf;
    }
    gravar(nt, k, (read_i16(clube + club::REPUTATION) > 0x1c52) as u32);
    let t = read_usize(read_usize(p + 0x39) + 0xcf);
    gravar(nt, k + 1, read_usize(t) as u32);
    for j in 1..4usize {
        gravar(nt, k + 1 + j as i8, read_usize(read_usize(t + 4 * j) + 0x33) as u32);
    }
    // ponytail: a original empurra o índice num dword de bytes de cima sem valor; a 0x785790 lê o char
    gravar(nt, k + 5, a as u8 as i8 as i32 as u32);
    entregar(read_usize(G_NEWS), nt, read_usize(p + 0x39));
}

// ponytail: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_queue_signing_celebration_story(this: usize, p: usize, clube: usize, tipo: u32, a: u32, porta_voz: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize)>(IDX_QUEUE_SIGNING_CELEBRATION_STORY)(this, p, clube, tipo, a, porta_voz)
}

/// news 0xbe4 of the team mate unhappy at the sale, to the old club's manager
/// Original: `Media::queue_sale_unrest_story` (0x75cfc0, media.cpp, thiscall).
///
/// [pt] { 0x75cfc0 } (thiscall, ret 0x10, media.cpp, quadro SEH da notícia local): `Media::queue_sale_unrest_story(velho,
/// novo, jogador, companheiro)`, pelas reações à transferência (0x758200). Velho clube com técnico (+0xcf), fora da
/// seleção (0x53b900), companheiro e jogador com ficha (+0x61). A notícia 0xbe4: o técnico, o companheiro e o jogador
/// (id e as três chaves de cada), os ids dos dois clubes, sorteio(2) (+2 se +0x5b do companheiro não pass_ball de
/// sorteio(5) + 10), o veículo (0x757f30), o comentarista (0x7dab90 com a nação +0x53 ou −1) e 2; vai ao técnico do velho
/// clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn queue_sale_unrest_story(_this: usize, velho: usize, novo: usize, p: usize, comp: usize) {
    if velho == 0 || read_usize(velho + 0xcf) == 0 || is_national_team(velho) != 0 {
        return;
    }
    if comp == 0 || read_usize(comp + competition::CLUB_COMP_FOREGROUND_COLOUR) == 0 || p == 0 || read_usize(p + 0x61) == 0 {
        return;
    }
    crate::comparacao::ramo("cqa_13 noticia do desassossego");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MEDIA_PICK_OUTLET_TYPE_FOR_CLUB);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(FN_PUNDITLIST_PICK_RANDOM_PUNDIT);
    let mut local = [0u32; 0x3a];
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbe4, 0);
    let mut k: i8 = 0;
    for quem in [read_usize(velho + 0xcf), comp, p] {
        gravar(nt, k, read_usize(quem) as u32);
        for j in 1..4usize {
            gravar(nt, k + j as i8, read_usize(read_usize(quem + 4 * j) + 0x33) as u32);
        }
        k += 4;
    }
    gravar(nt, 0xc, read_usize(velho) as u32);
    gravar(nt, 0xd, read_usize(novo) as u32);
    let r = cpe1_sorteio(5);
    let s = if read_i8_as_i32(comp + 0x5b) > r + 10 { cpe1_sorteio(2) } else { cpe1_sorteio(2) + 2 };
    gravar(nt, 0xe, s as u32);
    let v = veiculo(velho);
    gravar(nt, 0xf, v);
    let nacao = read_usize(velho + 0x53);
    gravar(nt, 0x10, comentarista(read_usize(G_PUNDIT_LIST), velho, v, if nacao != 0 { read_i32(nacao) } else { -1 }) as u32);
    gravar(nt, 0x11, 2);
    entregar(read_usize(G_NEWS), nt, velho);
}

// ponytail: a notícia vai à caixa do técnico (aloca) e sorteia: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_queue_sale_unrest_story(this: usize, velho: usize, novo: usize, p: usize, comp: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_QUEUE_SALE_UNREST_STORY)(this, velho, novo, p, comp)
}

// ---------- 259a leva: a fila dos regens: os regens jovens que vencem e os técnicos de reserva refeitos; o veículo da notícia pelo clube e pela pessoa (player_regen.cpp, media.cpp) ----------






/// A data da entrada `e` da fila dos regens (+4) é antes de hoje (0x456770)?
pub(crate) unsafe fn cpw10_vencida(e: usize) -> bool {
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    anterior(e + 4, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 // cpw10: a entrada vencida
}

/// Uma nota do técnico refeito (um dos blocos da 0x7a65d0): P = pow(+0x22 da pessoa, 0x968950) (vezes `fator`, quando
/// há), o maior entre P e o valor `v` da ficha, vezes `k`, no máximo `teto` (f64 em 0x...), truncado.
pub(crate) unsafe fn cpw10_nota(pessoa: usize, v: i16, fator: Option<usize>, k: usize, teto: usize) -> u16 {
    let mut p = pow_do_jogo(*((pessoa + staff::INT_APPS) as *const u8) as f64, f64_em(0x968950));
    if let Some(f) = fator {
        p *= f64_em(f);
    }
    let v = v as f64;
    let m = if p > v { p } else { v };
    let mk = m * f64_em(k);
    let t = f64_em(teto);
    ftol(if !(mk >= t) { mk } else { t }) as u16
}

/// O cargo do técnico refeito no clube da máquina (0x7a6757..0x7a68d8): reputação do jogador (+0xb da ficha) abaixo da
/// do clube (+0x80) vezes 0x968968 e menos de 5 treinadores (0x53c8d0): as vagas de treinador vazias (+0x19f, 5) ficam
/// com ele (cargo 0xf); sem vaga, auxiliar (+0xd3, cargo 0xd) se o clube não tiver (0x53cb80). Senão, abaixo da do clube
/// vezes 0x969040, auxiliar do mesmo jeito.
pub(crate) unsafe fn cpw10_cargo_no_clube(s: usize) {
    let clube = read_usize(s + 0x39);
    if clube == 0 {
        return;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let treinadores: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_COACHES);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_ASSISTANT);
    let rep = |k: usize| {
        let f = read_i16(read_usize(s + 0x61) + 0xb) as f64;
        let c = read_i16(read_usize(s + 0x39) + 0x80) as f64 * f64_em(k);
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



/// O veículo pelo sorteio(100) dos grandes (0x757f30/0x757fa0): 0 → 6; até 7 → 3; até 14 → 5; até 39 → 2; até 59 → 0;
/// senão 1 (comparações do al com sinal).
pub(crate) fn cpw10_veiculo_grande(al: i8) -> u32 {
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

/// O veículo dos menores: sorteio(4) de 0 a 2 → 2, senão 4.
pub(crate) unsafe fn cpw10_veiculo_menor() -> u32 {
    let r = cpe1_sorteio(4);
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
        cpw10_veiculo_grande(cpe1_sorteio(100) as u8 as i8)
    } else {
        cpw10_veiculo_menor()
    }
}

pub(crate) unsafe extern "C" fn comparar_media_pick_outlet_type_for_club(clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(IDX_MEDIA_PICK_OUTLET_TYPE_FOR_CLUB);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(clube), || crate::aleatorio::sem_contar(|| media_pick_outlet_type_for_club(clube)));
    PLACAR.registrar(igual && r == n, || format!("media_pick_outlet_type_for_club({:x}) original={} rust={} sorteio igual={}", clube, r, n, igual));
    r
}

/// the same from the player's reputation (player data +0xb)
/// Original: `media_pick_outlet_type_for_person` (0x757fa0, media.cpp, cdecl).
///
/// [pt] { 0x757fa0 } (cdecl, media.cpp): `media_pick_outlet_type_for_person(pessoa)`: como o do clube, pela reputação do
/// jogador (+0xb da ficha +0x61). Pelas histórias do jogador (0x72c1f0), do desabafo, do elogio, de querer jogar e da
/// comemoração da contratação (0x75b6e0, nas transferências).
pub(crate) unsafe extern "C" fn media_pick_outlet_type_for_person(pessoa: usize) -> u32 {
    if read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) > 0x186a {
        cpw10_veiculo_grande(cpe1_sorteio(100) as u8 as i8)
    } else {
        cpw10_veiculo_menor()
    }
}

pub(crate) unsafe extern "C" fn comparar_media_pick_outlet_type_for_person(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(IDX_MEDIA_PICK_OUTLET_TYPE_FOR_PERSON);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(pessoa), || crate::aleatorio::sem_contar(|| media_pick_outlet_type_for_person(pessoa)));
    PLACAR.registrar(igual && r == n, || format!("media_pick_outlet_type_for_person({:x}) original={} rust={} sorteio igual={}", pessoa, r, n, igual));
    r
}

// ---------- 251a leva: a notícia nova e a confiança da torcida e da diretoria depois de uma história da mídia (news.cpp, media.cpp) ----------






/// A linha do clube no gerente dos técnicos: [[0xb67a44]] + id × 0x49 (+6 confiança da diretoria, +8 da torcida, as duas
/// entre 1 e 10000; +0xe outra palavra).
pub(crate) unsafe fn cpo5_linha(clube: usize) -> usize {
    read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as usize).wrapping_mul(0x49))
}

/// Os amortecedores comuns das duas confianças: com o preferido (`gosta`) a queda vale metade e a alta o dobro; com o
/// rejeitado (`nao_gosta`) o contrário; fora de ±1000, o excesso vale 0,2. Devolve a palavra baixa (o original usa `di`).
pub(crate) unsafe fn cpo5_amortecer(v: i32, gosta: bool, nao_gosta: bool) -> i16 {
    let mut v = v;
    if gosta {
        let x = v as i16 as f64;
        v = if (v as i16) < 0 { ftol(x * f64_em(0x968968)) } else { ftol(x + x) };
    } else if nao_gosta {
        let x = v as i16 as f64;
        v = if (v as i16) > 0 { ftol(x * f64_em(0x968968)) } else { ftol(x + x) };
    }
    let x = v as i16 as i32;
    if x < -1000 {
        v = ftol((x + 1000) as f64 * f64_em(0x968918) - f64_em(0x968908));
    } else if x > 1000 {
        v = ftol((x - 1000) as f64 * f64_em(0x968918) + f64_em(0x968908));
    }
    v as i16
}

/// A variação da confiança da torcida (0x738340): conf = +8 da linha (0 sem clube); d = alvo − trunc((conf − 5000) ×
/// −0,05) (16 bits); v = trunc(d × 0,75); o técnico (+0xcf) favorito (0x547790) ou rejeitado (0x5477d0) pelo clube
/// amortece. As duas consultas são puras.
pub(crate) unsafe fn cpo5_variacao_da_torcida(clube: usize, alvo: u32) -> i16 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let conf = if clube != 0 { read_i16(cpo5_linha(clube) + 8) as i32 } else { 0 };
    let t = ftol((conf - 5000) as f64 * f64_em(0x96d280));
    let d = (alvo as i32).wrapping_sub(t) as i16;
    let v = ftol(d as f64 * f64_em(0x969040));
    let tecnico = read_usize(clube + club::MANAGER);
    let fav = favorito(tecnico, clube) != 0;
    let rej = !fav && rejeitado(tecnico, clube) != 0;
    cpo5_amortecer(v, fav, rej)
}

/// A variação da confiança da diretoria (0x7386f0): conf = +6 da linha (0 sem clube); s = (conf − 6000) / 20 + alvo
/// (16 bits); v = trunc(s × 0,75); o presidente (+0xbf) gosta (0x547810) ou não gosta (0x547840) do técnico (+0xcf).
pub(crate) unsafe fn cpo5_variacao_da_diretoria(clube: usize, alvo: u32) -> i16 {
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let conf = if clube != 0 { read_i16(cpo5_linha(clube) + 6) as i32 } else { 0 };
    let s = ((conf - 6000) / 20).wrapping_add(alvo as i32) as i16;
    let v = ftol(s as f64 * f64_em(0x969040));
    let (tecnico, presidente) = (read_usize(clube + club::MANAGER), read_usize(clube + club::CHAIRMAN));
    let g = gosta(tecnico, presidente) != 0;
    let n = !g && nao_gosta(tecnico, presidente) != 0;
    cpo5_amortecer(v, g, n)
}

/// Soma `v` na palavra da linha, entre 1 e 10000.
pub(crate) unsafe fn cpo5_somar(p: usize, v: i16) {
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
/// (ver `cpo5_variacao_da_torcida`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0): 0 = 0, 1 =
/// tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750, 4 negativa, 3 até 750, 2
/// acima), 6..9 o técnico; com sujeito, 10..13 o sujeito e 14 = 6 favorito, 7 rejeitado, 8 nenhum; depois −1, a
/// confiança nova e 0.
pub(crate) unsafe extern "thiscall" fn adjust_supporter_confidence(_this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32) {
    let v = cpo5_variacao_da_torcida(clube, alvo);
    crate::comparacao::ramo("cpo_5 torcida");
    if clube != 0 {
        cpo5_somar(cpo5_linha(clube) + 8, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbe0, 0);
    gravar(n, 0, 0);
    gravar(n, 1, topico as i16 as i32 as u32);
    gravar(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    gravar(n, 3, s as u32);
    gravar(n, 4, read_usize(clube) as u32);
    let grau = if v < -750 { 5 } else if v < 0 { 4 } else if v > 750 { 2 } else { 3 };
    gravar(n, 5, grau);
    cqc10_nomes(gravar, n, 6, read_usize(clube + club::MANAGER));
    let b: i8 = if sujeito != 0 {
        cqc10_nomes(gravar, n, 10, sujeito);
        let r = if favorito(sujeito, clube) != 0 {
            6
        } else if rejeitado(sujeito, clube) != 0 {
            7
        } else {
            8
        };
        gravar(n, 14, r);
        0xf
    } else {
        gravar(n, 10, u32::MAX);
        0xb
    };
    gravar(n, b, u32::MAX);
    let conf = if clube != 0 { read_i16(cpo5_linha(clube) + 8) as i32 } else { 0 };
    gravar(n, b + 1, conf as u32);
    gravar(n, b + 2, 0);
    entregar(read_usize(G_NEWS), n, clube);
}

/// adds to the board's confidence (+6 of the club row) (confidence − 6000)/20 + target, × 0.75, damped by a chairman
/// who likes or dislikes the manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager with the subject's flags
/// Original: `Media::adjust_board_confidence` (0x7386f0, media.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x7386f0 } (thiscall, ret 0x18, media.cpp, quadro SEH sem destrutor): `Media::adjust_board_confidence(clube,
/// sujeito, tópico, alvo, variante, sexto)`: soma na confiança da diretoria (+6 da linha do clube, entre 1 e 10000) a
/// variação (ver `cpo5_variacao_da_diretoria`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0):
/// 0 = 1, 1 = tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750 com confiança abaixo
/// de 4500; 9 negativa com confiança acima de 6250, senão 4; 2 acima de 750 com confiança acima de 6250, senão 3), 6..9
/// o técnico; com sujeito 10..13 e 14 = 6/7/8 (o presidente gosta, não gosta, nenhum); depois o presidente (ou −1), a
/// confiança nova, o sexto (com ele 1 e a palavra +0xe), e as marcas do sujeito: contrato 0x10/0x20 acima de 25 anos,
/// média das notas acima da média da liga × 5, +0x59 e +0x5b acima de 14, mais de 1500 dias no clube, valor (0x4d0c60)
/// acima de 4.500.000 e reputação (+5 da ficha) acima de 140.
pub(crate) unsafe extern "thiscall" fn adjust_board_confidence(_this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32, sexto: u32) {
    let v = cpo5_variacao_da_diretoria(clube, alvo);
    crate::comparacao::ramo("cpo_5 diretoria");
    if clube != 0 {
        cpo5_somar(cpo5_linha(clube) + 6, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    crate::comparacao::ramo("cpo_5 diretoria noticia");
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let ratings_average: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_PLAYERSTATS_GET_RECENT_FORM);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let conf = || if clube != 0 { read_i16(cpo5_linha(clube) + 6) } else { 0 };
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbe0, 0);
    gravar(n, 0, 1);
    gravar(n, 1, topico as i16 as i32 as u32);
    gravar(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    gravar(n, 3, s as u32);
    gravar(n, 4, read_usize(clube) as u32);
    let grau = if v < -750 && (clube == 0 || conf() < 4500) {
        5
    } else if v < 0 {
        if clube != 0 && conf() > 6250 { 9 } else { 4 }
    } else if v > 750 && clube != 0 && conf() > 6250 {
        2
    } else {
        3
    };
    gravar(n, 5, grau);
    cqc10_nomes(gravar, n, 6, read_usize(clube + club::MANAGER));
    let mut b: i8 = if sujeito != 0 {
        cqc10_nomes(gravar, n, 10, sujeito);
        let presidente = read_usize(clube + club::CHAIRMAN);
        let r = if gosta(sujeito, presidente) != 0 {
            6
        } else if nao_gosta(sujeito, presidente) != 0 {
            7
        } else {
            8
        };
        gravar(n, 14, r);
        0xf
    } else {
        gravar(n, 10, u32::MAX);
        0xb
    };
    let presidente = read_usize(clube + club::CHAIRMAN);
    if presidente != 0 {
        cqc10_nomes(gravar, n, b, presidente);
        b += 4;
    } else {
        gravar(n, b, u32::MAX);
        b += 1;
    }
    gravar(n, b, conf() as i32 as u32);
    b += 1;
    if sexto != 0 {
        gravar(n, b, 1);
        let w = if clube != 0 { read_i16(cpo5_linha(clube) + 0xe) } else { 0 };
        gravar(n, b + 1, w as i32 as u32);
        b += 2;
    } else {
        gravar(n, b, 0);
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
    gravar(n, b, m);
    b += 1;
    let mut m = 0u32;
    if sujeito != 0 && read_usize(sujeito + 0x61) != 0 {
        let base = read_usize(G_PLAYER_STATS);
        let notas = ratings_average(base, read_usize(sujeito + 0x61)) as i8 as f64;
        let base = read_usize(G_PLAYER_STATS);
        let y = f64_em(base + 0x18);
        let c0 = f64_em(0x9689a0);
        let media = if !(y < c0 || y > c0) { f64_em(0x968998) } else { f64_em(base + 0x10) / y }; // sem jogos: 6,8
        if notas > media * f64_em(0x9688a0) {
            m = 1;
        }
    }
    gravar(n, b, m);
    b += 1;
    gravar(n, b, (sujeito != 0 && read_i8_as_i32(sujeito + 0x59) > 0xe) as u32);
    b += 1;
    gravar(n, b, (sujeito != 0 && read_i8_as_i32(sujeito + 0x5b) > 0xe) as u32);
    b += 1;
    let dias = sujeito != 0 && dias_ate(0xae2c90, read_usize(sujeito + 0x3e) as u32, read_usize(sujeito + 0x42) as u32) > 0x5dc;
    gravar(n, b, dias as u32);
    b += 1;
    let caro = sujeito != 0 && valor(read_usize(G_CONTRACT_MANAGER), sujeito, clube, 0, u32::MAX, 0) > 0x44aa20;
    gravar(n, b, caro as u32);
    b += 1;
    let reputacao = sujeito != 0 && read_usize(sujeito + 0x61) != 0 && read_i16(read_usize(sujeito + 0x61) + 5) > 0x8c;
    gravar(n, b, reputacao as u32);
    entregar(read_usize(G_NEWS), n, clube);
}


/// Comparador: a original sorteia e entrega a notícia (não pode rodar duas vezes): roda só ela e confere o plano feito
/// antes: a confiança da torcida nova (a variação é pura).
pub(crate) unsafe extern "thiscall" fn comparar_adjust_supporter_confidence(this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADJUST_SUPPORTER_CONFIDENCE);
    if clube == 0 {
        return o(this, clube, sujeito, topico, alvo, variante);
    }
    let p = cpo5_linha(clube) + 8;
    let antes = read_i16(p);
    let v = cpo5_variacao_da_torcida(clube, alvo);
    let esperado = antes.wrapping_add(v).clamp(1, 10000);
    o(this, clube, sujeito, topico, alvo, variante);
    let depois = read_i16(cpo5_linha(clube) + 8);
    PLACAR.registrar(depois == esperado, || format!("adjust_supporter_confidence(clube {:x}, alvo {}) {} + {} = {} x original {}", clube, alvo as i16, antes, v, esperado, depois));
}

/// Comparador: como o da torcida, pela confiança da diretoria nova.
pub(crate) unsafe extern "thiscall" fn comparar_adjust_board_confidence(this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32, sexto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_ADJUST_BOARD_CONFIDENCE);
    if clube == 0 {
        return o(this, clube, sujeito, topico, alvo, variante, sexto);
    }
    let p = cpo5_linha(clube) + 6;
    let antes = read_i16(p);
    let v = cpo5_variacao_da_diretoria(clube, alvo);
    let esperado = antes.wrapping_add(v).clamp(1, 10000);
    o(this, clube, sujeito, topico, alvo, variante, sexto);
    let depois = read_i16(cpo5_linha(clube) + 6);
    PLACAR.registrar(depois == esperado, || format!("adjust_board_confidence(clube {:x}, alvo {}) {} + {} = {} x original {}", clube, alvo as i32, antes, v, esperado, depois));
}

// ---------- 251a leva: as notícias do jogador depois de uma história da mídia: gratidão ao técnico, pedido de apoio e reação à crítica (media.cpp) ----------




pub(crate) type Cpo7Gravar = unsafe extern "thiscall" fn(usize, i8, u32) -> u32;

/// Os dados da pessoa a partir de `k`: id e as três chaves +0x33 de +4, +8 e +0xc (a de +0xc pode faltar: −1, só
/// quando `opcional`).
pub(crate) unsafe fn cpo7_dados(gravar: Cpo7Gravar, n: usize, k: i8, p: usize, opcional: bool) {
    gravar(n, k, read_usize(p) as u32);
    gravar(n, k + 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, k + 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    let q = read_usize(p + 0xc);
    gravar(n, k + 3, if opcional && q == 0 { u32::MAX } else { read_usize(q + 0x33) as u32 });
}

/// A dupla de transmutes comum: construir a notícia (0x77bbe0) e gravar um dado (0x785790).
pub(crate) unsafe fn cpo7_noticia() -> (unsafe extern "thiscall" fn(usize, u32, u32) -> usize, Cpo7Gravar) {
    (core::mem::transmute(FN_NEWSITEM_NEWSITEM), core::mem::transmute(FN_NEWSITEM_SET_SLOT))
}

/// Os dias desde a chegada da pessoa (+0x3e/+0x42) até hoje (0x54cdb0 em 0xae2c90).
pub(crate) unsafe fn cpo7_dias(p: usize) -> i32 {
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    dias_ate(0xae2c90, read_usize(p + 0x3e) as u32, read_usize(p + 0x42) as u32)
}

/// O sorteio da gratidão pelo tipo (índices da tabela de bytes 0x732fcc, saltos 0x732fb4): 0 os tipos 0, 6, 14 e 17; 1
/// os 2 e 30 (sorteio(2)); 2 os 4, 9, 33, 34 e 37 (sorteio(3)); 3 os 5 e 7 (sorteio(4)); o resto 0.
pub(crate) const CPO7_CASO: [u8; 0x26] = [0, 5, 1, 5, 2, 3, 0, 3, 5, 2, 5, 5, 5, 4, 0, 5, 5, 0, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 2, 2, 4, 5, 2];

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
pub(crate) unsafe extern "thiscall" fn queue_player_gratitude_news(this: usize, jogador: usize, clube: usize, tipo: u32, variante: u32) {
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || clube == 0 || read_usize(clube + club::MANAGER) == 0 {
        return;
    }
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = contract_with_club(read_usize(G_CONTRACT_MANAGER), jogador, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 gratidao");
    let (construir, gravar) = cpo7_noticia();
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let lesionado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbda, 0);
    cpo7_dados(gravar, n, 0, jogador, true);
    gravar(n, 4, read_usize(clube) as u32);
    // a lista da mídia (this +0, conta em +8)
    let mut in_list = false;
    for i in 0..core::ptr::read_unaligned((this + 8) as *const i32).max(0) as usize {
        let e = read_usize(this) + i * 0x17;
        if read_usize(e + 8) == read_usize(jogador) && *((e + 0x10) as *const u8) == 2 {
            in_list = true;
            break;
        }
    }
    let f45 = read_usize(contrato + 0x45);
    let mut t = tipo;
    if in_list || f45 & 0x2000 != 0 || f45 & 8 != 0 {
        if [0xe, 0, 2, 9, 0x23, 4, 7].contains(&(tipo as i16)) {
            crate::comparacao::ramo("cpo_7 gratidao tipo");
            let s = sorteio(10);
            t = if read_i8_as_i32(jogador + 0x5d) >= s { 0x12 } else { 0x21 };
        }
    }
    if lesionado(read_usize(G_INJURY), jogador) != 0 {
        t = 0xc;
    }
    let t = t as i16 as i32;
    gravar(n, 5, t as u32);
    gravar(n, 6, variante as i8 as i32 as u32);
    let v = match if (t as u32) <= 0x25 { CPO7_CASO[t as usize] } else { 5 } {
        0 => {
            let ficha = read_usize(jogador + 0x61);
            if ficha != 0 && read_i8_as_i32(ficha + 0x44) < 10 {
                (sorteio(3) as u8).wrapping_add(2) as i8 as i32
            } else {
                let s = sorteio(5);
                if s == 0 || s == 1 {
                    let ficha = read_usize(jogador + 0x61);
                    if ficha != 0 && read_i8_as_i32(ficha + 0x44) < 0x14 {
                        crate::comparacao::ramo("cpo_7 gratidao sobe");
                        *((ficha + 0x44) as *mut u8) += 1;
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
    let tecnico = read_usize(clube + club::MANAGER);
    cpo7_dados(gravar, n, 8, tecnico, false);
    gravar(n, 0xc, 0x420e9);
    let auxiliar = read_usize(clube + club::ASSISTANT_MANAGER);
    if auxiliar != 0 {
        cpo7_dados(gravar, n, 0xd, auxiliar, false);
    } else {
        gravar(n, 0xd, u32::MAX);
    }
    entregar(read_usize(G_NEWS), n, clube);
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
pub(crate) unsafe extern "thiscall" fn post_player_backing_request_news(_this: usize, jogador: usize, clube: usize, tipo: u32, quarto: u32) {
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human_club(clube, 0, 1, 0, 0) == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 pedido de apoio");
    let (construir, gravar) = cpo7_noticia();
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbcf, 0);
    cpo7_dados(gravar, n, 0, jogador, true);
    gravar(n, 4, read_usize(clube) as u32);
    let t = tipo as u8;
    let alvo = t == 0x20 || t == 1 || t == 0x1e;
    let (a, b): (i32, i32) = if read_i16(read_usize(jogador + 0x61) + 0xb) > 0x109a {
        if read_i8_as_i32(jogador + 0x57) > 0xf && read_i8_as_i32(jogador + 0x5d) < 6 {
            (1, 1)
        } else if alvo && read_i8_as_i32(jogador + 0x59) > 0xf && read_i8_as_i32(jogador + 0x5b) > 0xa && cpo7_dias(jogador) > 0xd2 {
            (1, 3)
        } else {
            (1, 2)
        }
    } else if read_i8_as_i32(jogador + 0x5b) < 0xa && read_i8_as_i32(jogador + 0x59) < 6 {
        (2, 1)
    } else if alvo && read_i8_as_i32(jogador + 0x59) > 0xf && read_i8_as_i32(jogador + 0x5b) > 0xa && cpo7_dias(jogador) > 0xd2 {
        (2, 3)
    } else {
        (2, 2)
    };
    gravar(n, 5, a as u32);
    gravar(n, 6, b as u32);
    gravar(n, 7, t as i8 as i32 as u32);
    gravar(n, 8, quarto as i16 as i32 as u32);
    gravar(n, 9, (listado(jogador, 0) != 0) as u32);
    entregar(read_usize(G_NEWS), n, clube);
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
pub(crate) unsafe extern "thiscall" fn post_player_criticism_reaction_news(_this: usize, jogador: usize, clube: usize, topico: u32, resposta: u32) {
    if jogador == 0 || read_usize(jogador + 0x61) == 0 || clube == 0 || read_usize(clube + club::MANAGER) == 0 {
        return;
    }
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = contract_with_club(read_usize(G_CONTRACT_MANAGER), jogador, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 reacao a critica");
    let (construir, gravar) = cpo7_noticia();
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    let clubes_interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdc, 0);
    let f45 = read_usize(contrato + 0x45);
    let marca = (f45 & 0x2000 != 0 || f45 & 8 != 0)
        && read_i8_as_i32(jogador + 0x5b) < 0xa
        && read_i8_as_i32(jogador + 0x5d) > 0xa
        && read_i8_as_i32(jogador + 0x59) < 0xf
        && read_i8_as_i32(jogador + 0x18) > 0x17;
    cpo7_dados(gravar, n, 0, jogador, true);
    gravar(n, 4, read_usize(clube) as u32);
    let r = resposta as u8;
    let rep = read_i16(read_usize(jogador + 0x61) + 0xb);
    let mut reacao = 0i8;
    if rep > 0x109a && rep > read_i16(clube + club::REPUTATION) {
        let gerente = read_usize(G_CONTRACT_MANAGER);
        let cat_ok = categoria(gerente, jogador, contrato) == 1 || categoria(gerente, jogador, contrato) == 2;
        if cat_ok && read_i8_as_i32(jogador + 0x18) > 0x16 {
            if read_i8_as_i32(jogador + 0x59) < 0xa && read_i8_as_i32(jogador + 0x57) > 0xf && read_i8_as_i32(jogador + 0x5d) < 6 {
                reacao = if marca && r == 2 { 3 } else { 1 };
            } else if cpo7_dias(jogador) > 0x3e8 && read_i8_as_i32(jogador + 0x59) > 0xf && read_usize(contrato + 0x45) & 0x181 == 0 {
                reacao = if marca && r == 2 { 4 } else { 2 };
            }
        }
    }
    gravar(n, 5, reacao as i32 as u32);
    gravar(n, 6, r as i8 as i32 as u32);
    let linha = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_usize(clube)).wrapping_mul(0x49));
    gravar(n, 7, (read_i16(linha + 6) < 0xfa0) as u32); // confiança da diretoria abaixo de 4000
    gravar(n, 8, topico);
    cpo7_dados(gravar, n, 9, read_usize(clube + club::MANAGER), false);
    let mut clubes = [0usize; 0x1e];
    let tm = read_usize(G_TRANSFER_MANAGER);
    let k = if tm != 0 { clubes_interessados(tm, jogador, 0x21a, clubes.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for i in 0..k.max(0) as usize {
        entregar(read_usize(G_NEWS), n, clubes[i]);
    }
}

/// Comparador: a gratidão sorteia, sobe +0x44 da ficha e entrega a notícia: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_queue_player_gratitude_news(this: usize, jogador: usize, clube: usize, tipo: u32, variante: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_QUEUE_PLAYER_GRATITUDE_NEWS)(this, jogador, clube, tipo, variante)
}

/// Comparador: a notícia vai à caixa do técnico (não pode rodar duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_post_player_backing_request_news(this: usize, jogador: usize, clube: usize, tipo: u32, quarto: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_POST_PLAYER_BACKING_REQUEST_NEWS)(this, jogador, clube, tipo, quarto)
}

/// Comparador: a notícia vai às caixas dos técnicos (não pode rodar duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_post_player_criticism_reaction_news(this: usize, jogador: usize, clube: usize, topico: u32, resposta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_POST_PLAYER_CRITICISM_REACTION_NEWS)(this, jogador, clube, topico, resposta)
}

// ---------- 250a leva: a mídia e a caixa: a nota do jogador para uma história, o texto da crise financeira e as notícias da caixa do humano (media.cpp, news.cpp) ----------





/// A notícia guardada na caixa: 0xdf bytes (+0xd6 id).
pub(crate) const CPN3_NOTICIA: usize = 0xdf;

/// A razão entre a reputação do jogador (+0xb do registro +0x61, no mínimo 2000) e a do clube (+0x80, entre 2000 e
/// 7500), como o fild/fidiv do original.
pub(crate) unsafe fn cpn3_razao_das_reputacoes(jogador: usize, clube: usize) -> f64 {
    let a = read_i16(read_usize(jogador + 0x61) + 0xb);
    let a = if a < 2000 { 2000 } else { a as i32 };
    let b = read_i16(clube + club::REPUTATION);
    let b = if b < 2000 { 2000 } else if b > 7500 { 7500 } else { b as i32 };
    a as f64 / b as f64
}

/// O tipo de história (0..0x52) → o caso do original (a tabela de bytes de 0x736b8c; 12 = a nota como está).
pub(crate) fn cpn3_caso(tipo: i16) -> u8 {
    match tipo {
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
pub(crate) unsafe extern "thiscall" fn rate_player_form_for_story(_this: usize, jogador: usize, clube: usize, contrato: usize, tipo: u32, modo: u32, peso: f32) -> f64 {
    let mut contrato = contrato;
    if contrato == 0 {
        let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contrato = com_clube(read_usize(G_CONTRACT_MANAGER), jogador, clube);
        if contrato == 0 {
            return 0.0;
        }
    }
    let estat: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let media_notas: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYERSTATS_GET_RECENT_FORM);
    let g = read_usize(G_PLAYER_STATS);
    let jogos = core::ptr::read_unaligned((g + 0x18) as *const f64);
    let x = if jogos == 0.0 || jogos.is_nan() { 6.8 } else { core::ptr::read_unaligned((g + 0x10) as *const f64) / jogos };
    let xf = x as f32;
    let s1f = estat(read_usize(G_PLAYER_STATS), read_usize(jogador + 0x61), 6, 1, 6.5) as f32;
    let s2f = estat(read_usize(G_PLAYER_STATS), read_usize(jogador + 0x61), 6, 0x11, 6.5) as f32;
    let media = media_notas(read_usize(G_PLAYER_STATS), read_usize(jogador + 0x61)) as u8 as i8 as i32;
    let mut l0 = (media as f64 * 0.2) as f32;
    let y = if s1f as f64 > 1.0 { s1f as f64 } else { 1.0 }; // as partidas, no mínimo 1
    let s3 = estat(read_usize(G_PLAYER_STATS), read_usize(jogador + 0x61), 6, 2, 6.5);
    let l2 = (s3 / y) as f32;
    l0 = (l0 as f64 - xf as f64) as f32;
    let dif = s2f as f64 - xf as f64;
    let dif_f = dif as f32;
    crate::comparacao::ramo("cpn_3 nota do jogador");
    let mut r = dif + l0 as f64 - peso as f64 * (1.0f32 / 60.0) as f64; // o peso da história
    if !(s1f as f64 >= 12.0) {
        crate::comparacao::ramo("cpn_3 poucas partidas");
        r = r * s1f as f64 * (1.0f32 / 12.0) as f64; // menos de 12 partidas
    }
    if !(r >= 0.0) {
        crate::comparacao::ramo("cpn_3 nota negativa");
        r = r * cpn3_razao_das_reputacoes(jogador, clube); // a negativa pela reputação
    }
    let rf = r as f32;
    let tipo = tipo as u16 as i16;
    if !(0..=0x52).contains(&tipo) {
        return r;
    }
    let c = *((contrato + contract::SQUAD_STATUS) as *const u8) >> 4; // o tipo de contrato
    match cpn3_caso(tipo) {
        0 => match c {
            1 => r - 0.5,
            2 => r - 0.35f32 as f64,
            3 => r - 0.2f32 as f64,
            _ => r + 0.25,
        },
        1 => {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            let d = dias(0xae2c90, read_usize(jogador + 0x3e) as u32, read_usize(jogador + 0x42) as u32);
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
        3 => r + (0x23 - read_i8(jogador + 0x18) as i32) as f64 * (1.0f32 / 3.0) as f64,
        4 => r + (10 - read_i8(read_usize(jogador + 0x61) + 0x25) as i32) as f64 * 0.2f32 as f64,
        5 => r + (read_i8(jogador + 0x18) as i32 - 0x13) as f64 * (1.0f32 / 3.0) as f64,
        6 => r + (read_i8(read_usize(jogador + 0x61) + 0x2f) as i32 - 10) as f64 * (1.0f32 / 3.0) as f64,
        7 => r + l0 as f64,
        8 => {
            let r2 = if r > 0.0 { r / cpn3_razao_das_reputacoes(jogador, clube) } else { r };
            match c {
                1 => r2 - 0.5,
                2 => r2 - 0.35f32 as f64,
                3 => r2 - 0.2f32 as f64,
                _ => r2,
            }
        }
        9 => r - peso as f64 * 0.1f32 as f64,
        10 => match c {
            0 => r - 1.0,
            1 => r + 2.0,
            2 => r + 1.5,
            3 => r + 1.0,
            7 => {
                if modo as u8 == 3 { 0.0 } else { r - 3.0 }
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
pub(crate) unsafe extern "thiscall" fn write_financial_trouble_news_text(_this: usize, noticia: usize, corpo: u32, saida: usize) {
    let campo: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let por_texto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TEXT_SET_STRING);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TEXT_ADD_LINK);
    crate::comparacao::ramo("cpn_3 texto da crise financeira");
    let id = campo(noticia, 0);
    let gravidade = campo(noticia, 1);
    campo(noticia, 2);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(0x245));
    let nome = clube + club::SHORT_NAME;
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
    formatar(0xde1f64, modelo, genero, genero, nome);
    por_texto(saida, 0xde1f64);
    ligar(saida, nome, 0x7d5, clube);
}


/// Comparador: a nota é pura (as estatísticas, as datas e o contrato só leem; nada sorteia): compara a devolução bit a
/// bit (a original e a cópia na mesma precisão dupla do x87).
pub(crate) unsafe extern "thiscall" fn comparar_rate_player_form_for_story(this: usize, jogador: usize, clube: usize, contrato: usize, tipo: u32, modo: u32, peso: f32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64>(IDX_RATE_PLAYER_FORM_FOR_STORY)(this, jogador, clube, contrato, tipo, modo, peso);
    let n = rate_player_form_for_story(this, jogador, clube, contrato, tipo, modo, peso);
    PLACAR.registrar(o.to_bits() == n.to_bits() || (o.is_nan() && n.is_nan()),
        || format!("rate_player_form_for_story(tipo {}, modo {}, peso {}) {} x {}", tipo as u16 as i16, modo as u8, peso, o, n));
    o
}

/// Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original (a cópia
/// prova no modo Rust, pelo ramo).
pub(crate) unsafe extern "thiscall" fn comparar_write_financial_trouble_news_text(this: usize, noticia: usize, corpo: u32, saida: usize) {
    // ponytail: o objeto de texto não pode receber o texto duas vezes; só a original roda aqui
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_FINANCIAL_TROUBLE_NEWS_TEXT)(this, noticia, corpo, saida);
}


// ---------- 248a leva: o registro de histórias da mídia: registrar uma história, expirar as antigas e se a história já foi contada ----------




/// A data `a` é anterior à `b`? Como a original: a palavra +2 sem sinal e, igual, a palavra +0 com sinal.
pub(crate) unsafe fn cpl6_antes(a: usize, b: usize) -> bool {
    let (x, y) = (read_u16(a + 2), read_u16(b + 2));
    if x != y {
        return x < y;
    }
    (read_u16(a) as i16) < (read_u16(b) as i16)
}

/// O id (+0) do sujeito, ou −1 sem ele.
pub(crate) unsafe fn cpl6_id(p: usize) -> usize {
    if p != 0 { read_usize(p) } else { usize::MAX }
}

/// Os 23 bytes que a 0x757a50 grava no registro (data de hoje [0xae2c90], ids, resposta, subtipo, tipo).
pub(crate) unsafe fn cpl6_registro(a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) -> [u8; 0x17] {
    let mut r = [0u8; 0x17];
    r[..8].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    r[8..0xc].copy_from_slice(&(cpl6_id(a) as u32).to_le_bytes());
    r[0xc..0x10].copy_from_slice(&(cpl6_id(b) as u32).to_le_bytes());
    r[0x10] = resposta as u8;
    r[0x11..0x13].copy_from_slice(&(subtipo as u8 as i8 as i16).to_le_bytes());
    r[0x13..].copy_from_slice(&tipo.to_le_bytes());
    r
}

pub(crate) unsafe fn cpl6_ordenar(base: usize, n: usize) {
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
pub(crate) unsafe extern "thiscall" fn log_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) {
    crate::comparacao::ramo("cpl_6 registrar historia");
    if read_usize(this) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let p = alocar(0x1cc);
        write_usize(this, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6e11);
            erro_fatal();
        }
        write_usize(this + 4, 0x14);
    }
    let cap = read_usize(this + 4);
    if cap == read_usize(this + 8) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realocar(read_usize(this), cap.wrapping_add(0x14).wrapping_mul(0x17) as u32);
        write_usize(this, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6e1c);
            erro_fatal();
        }
        write_usize(this + 4, read_usize(this + 4).wrapping_add(0x14));
    }
    let item = || read_usize(this).wrapping_add(read_usize(this + 8).wrapping_mul(0x17));
    write_usize(item() + 8, cpl6_id(a));
    write_usize(item() + 0xc, cpl6_id(b));
    copiar_data_do_recorde(item(), 0xae2c90);
    write_usize(item() + 0x13, tipo as usize);
    core::ptr::write_unaligned((item() + 0x11) as *mut i16, subtipo as u8 as i8 as i16);
    *((item() + 0x10) as *mut u8) = resposta as u8;
    let n = read_usize(this + 8).wrapping_add(1);
    write_usize(this + 8, n); // mais uma história
    cpl6_ordenar(read_usize(this), n);
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
        let dias = if read_usize(r + 0x13) == 0xbdf {
            crate::comparacao::ramo("cpl_6 expirar historia do time");
            let s = sorteio(0x28); // o sorteio da história do time
            (s + 0x28) as u32
        } else {
            100 // os dias das outras histórias
        };
        let mut local = [0u8; 8];
        let limite = datar(0xae2c90, &mut local, dias);
        if cpl6_antes(r, limite) {
            let n = read_usize(this + 8);
            let ultimo = read_usize(this).wrapping_add(n.wrapping_mul(0x17)).wrapping_sub(0x17);
            core::ptr::copy(ultimo as *const u8, r as *mut u8, 0x17);
            write_usize(this + 8, n.wrapping_sub(1));
            i -= 1; // o mesmo lugar de novo
        }
        i += 1;
    }
    if read_usize(this + 8) != 0 {
        cpl6_ordenar(read_usize(this), read_usize(this + 8));
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
pub(crate) unsafe extern "thiscall" fn has_logged_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, desde: usize) -> u32 {
    let mut chave = [0u8; 0x17];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    padrao(chave.as_mut_ptr() as usize);
    let s = subtipo as u16;
    if tipo == 0xbdf {
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
    chave[8..0xc].copy_from_slice(&(cpl6_id(a) as u32).to_le_bytes());
    chave[0x13..].copy_from_slice(&tipo.to_le_bytes());
    chave[0xc..0x10].copy_from_slice(&(read_usize(b) as u32).to_le_bytes());
    chave[0x11..0x13].copy_from_slice(&s.to_le_bytes());
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = bsearch(chave.as_ptr() as usize, read_usize(this), read_usize(this + 8) as u32, 0x17, 0x757ee0);
    if e == 0 {
        return 0;
    }
    if desde == 0 {
        return 1;
    }
    if cpl6_antes(e, desde) { 0 } else { 1 }
}

/// Comparador: a original pode alocar e realocar (não roda duas vezes): o registro esperado antes; confere a conta e
/// que o registro esperado está na lista (a ordem é a do qsort).
pub(crate) unsafe extern "thiscall" fn comparar_log_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) {
    let n = read_usize(this + 8);
    let esperado = cpl6_registro(a, b, tipo, subtipo, resposta);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_LOG_STORY)(this, a, b, tipo, subtipo, resposta);
    let m = read_usize(this + 8);
    let achou = (0..m).any(|k| {
        core::slice::from_raw_parts(read_usize(this).wrapping_add(k * 0x17) as *const u8, 0x17) == &esperado[..]
    });
    PLACAR.registrar(m == n.wrapping_add(1) && achou, || {
        format!("log_story(tipo {:#x}, subtipo {}) conta {} -> {}, esperado {:x?} achou {}", tipo, subtipo as u8 as i8, n, m, esperado, achou)
    });
}

/// Comparador: lado a lado pelo registro (ponteiro, capacidade e conta; as histórias) e o estado do sorteio (a cópia
/// sem contar os sorteios).
pub(crate) unsafe extern "thiscall" fn comparar_expire_story_log(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_EXPIRE_STORY_LOG);
    if read_usize(this) == 0 {
        return o(this);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0xc));
    reg.push((read_usize(this), (read_usize(this + 8) as i32).max(0) as usize * 0x17));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| expire_story_log(this)));
    // mo.0[2] e mr.0[2]: ponteiro, capacidade e conta depois de cada lado
    PLACAR.registrar(igual, || format!("expire_story_log {:x?} x {:x?}", mo.0[2].1, mr.0[2].1));
}

/// Comparador: pelo valor (a busca e a data padrão da chave só leem).
pub(crate) unsafe extern "thiscall" fn comparar_has_logged_story(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, desde: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> u32>(IDX_HAS_LOGGED_STORY)(this, a, b, tipo, subtipo, desde);
    let n = has_logged_story(this, a, b, tipo, subtipo, desde);
    PLACAR.registrar(r == n, || format!("has_logged_story(tipo {:#x}, subtipo {}, desde {}) original={} rust={}", tipo, subtipo as u16 as i16, desde != 0, r, n));
    r
}

// ---------- 248a leva: os interesses de transferência da mídia: acrescentar um interesse e as comparações das chaves de ids e das histórias ----------



/// A chave da 0x754740: {id do jogador, id do clube ou −1} (os 8 bytes que a 0x757ea0 lê).
pub(crate) unsafe fn cpl7_chave(jogador: usize, clube: usize) -> [u32; 2] {
    [read_usize(jogador) as u32, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }]
}

/// O interesse já está no vetor? (bsearch 0x946464 com 0x757ea0; sem vetor, não).
pub(crate) unsafe fn cpl7_ja_esta(this: usize, chave: &[u32; 2]) -> bool {
    let base = read_usize(this + 0xc);
    if base == 0 {
        return false;
    }
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(chave.as_ptr() as usize, base, read_usize(this + 0x14) as u32, 0x19, 0x757ea0) != 0
}

/// O valor que a 0x754740 grava: o `valor` limitado a 50.000.000.
pub(crate) fn cpl7_valor(valor: i32) -> i32 {
    if valor < 0x2faf080 { valor } else { 0x2faf080 }
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
pub(crate) unsafe extern "thiscall" fn add_transfer_interest(this: usize, jogador: usize, clube: usize, valor: i32, avaliacao: f64) {
    let mut data = [0u8; 8];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    padrao(data.as_mut_ptr() as usize);
    let chave = cpl7_chave(jogador, clube);
    if cpl7_ja_esta(this, &chave) {
        crate::comparacao::ramo("cpl_7 interesse repetido");
        return; // o interesse já está
    }
    crate::comparacao::ramo("cpl_7 interesse novo");
    if read_usize(this + 0xc) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let p = alocar(0x1f4);
        write_usize(this + 0xc, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x665d);
            erro_fatal();
        }
        write_usize(this + 0x10, 0x14);
    }
    let cap = read_usize(this + 0x10);
    if cap == read_usize(this + 0x14) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realocar(read_usize(this + 0xc), cap.wrapping_add(0x14).wrapping_mul(0x19) as u32);
        write_usize(this + 0xc, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6668);
            erro_fatal();
        }
        write_usize(this + 0x10, read_usize(this + 0x10).wrapping_add(0x14));
    }
    let item = || read_usize(this + 0xc).wrapping_add(read_usize(this + 0x14).wrapping_mul(0x19));
    write_usize(item(), read_usize(jogador));
    write_usize(item() + 4, read_usize(clube));
    copiar_data_do_recorde(item() + 8, 0xae2c90);
    *((item() + 0x10) as *mut u8) = 0; // o boato ainda não saiu
    write_usize(item() + 0x11, cpl7_valor(valor) as usize);
    write_usize(item() + 0x15, ftol(avaliacao * 100.0) as usize); // [0x9688a8] = 100.0
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

/// Os 25 bytes que a 0x754740 acrescenta.
pub(crate) unsafe fn cpl7_interesse(jogador: usize, clube: usize, valor: i32, avaliacao: f64) -> [u8; 0x19] {
    let mut r = [0u8; 0x19];
    r[..4].copy_from_slice(&(read_usize(jogador) as u32).to_le_bytes());
    r[4..8].copy_from_slice(&(read_usize(clube) as u32).to_le_bytes());
    r[8..0x10].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    r[0x11..0x15].copy_from_slice(&cpl7_valor(valor).to_le_bytes());
    r[0x15..].copy_from_slice(&ftol(avaliacao * 100.0).to_le_bytes());
    r
}

/// Comparador: a original pode alocar e realocar (não roda duas vezes): o plano antes (já está? o registro novo);
/// confere a conta e que o registro novo está no vetor (a ordem é a do qsort).
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_interest(this: usize, jogador: usize, clube: usize, valor: i32, avaliacao: f64) {
    let n = read_usize(this + 0x14);
    let ja_esta = cpl7_ja_esta(this, &cpl7_chave(jogador, clube));
    let esperado = if ja_esta { None } else { Some(cpl7_interesse(jogador, clube, valor, avaliacao)) };
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, i32, f64)>(IDX_ADD_TRANSFER_INTEREST)(this, jogador, clube, valor, avaliacao);
    let m = read_usize(this + 0x14);
    let ok = match &esperado {
        None => m == n,
        Some(e) => m == n.wrapping_add(1) && (0..m).any(|k| {
            core::slice::from_raw_parts(read_usize(this + 0xc).wrapping_add(k * 0x19) as *const u8, 0x19) == &e[..]
        }),
    };
    PLACAR.registrar(ok, || format!("add_transfer_interest(valor {}, avaliacao {}) conta {} -> {}, esperado {:x?}", valor, avaliacao, n, m, esperado));
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_id_pair_keys(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_ID_PAIR_KEYS)(a, b);
    let n = compare_id_pair_keys(a, b);
    PLACAR.registrar(r == n, || format!("compare_id_pair_keys({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_story_log_records(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_STORY_LOG_RECORDS)(a, b);
    let n = compare_story_log_records(a, b);
    PLACAR.registrar(r == n, || format!("compare_story_log_records({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 244a leva: tirar de vetores ordenados: o interesse de transferência da mídia, o pedido de observação e o apelido de todos os técnicos humanos ----------


pub(crate) unsafe fn cph4_qsort(base: usize, n: i32, tamanho: u32, comparar: usize) {
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(base, n, tamanho, comparar);
}

pub(crate) unsafe fn cph4_bsearch(chave: usize, base: usize, n: u32, tamanho: u32, comparar: usize) -> usize {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(chave, base, n, tamanho, comparar)
}

/// removes the person interests (25 bytes) moving the last one into the gap; sorts again (0x757ea0)
/// Original: `Media::remove_transfer_interest` (0x7549b0, media.cpp, thiscall, ret 4).
///
/// [pt] { 0x7549b0 } (thiscall, ret 4, media.cpp): `Media::remove_transfer_interest(pessoa)`: no vetor de interesses
/// ([this+0xc], [this+0x14] registros de 25 bytes), cada um da pessoa (+0 = id) recebe por cima o último (6 duplas e um
/// byte), a contagem desce e o mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x757ea0).
pub(crate) unsafe extern "thiscall" fn remove_transfer_interest(this: usize, pessoa: usize) {
    let n = (this + 0x14) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + 0xc) as usize;
            let r = base.wrapping_add(off);
            if read_u32(r) == read_u32(pessoa) {
                let ultimo = base.wrapping_add((*n as usize).wrapping_mul(25)).wrapping_sub(25); // o último interesse
                core::ptr::copy(ultimo as *const u8, r as *mut u8, 25);
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
        cph4_qsort(read_u32(this + 0xc) as usize, *n, 0x19, 0x757ea0);
    }
}



pub(crate) unsafe extern "thiscall" fn comparar_remove_transfer_interest(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_TRANSFER_INTEREST);
    let n = (*((this + 0x14) as *const i32)).max(0) as usize;
    let reg = [(this + 0xc, 0xc), (read_u32(this + 0xc) as usize, n * 25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || remove_transfer_interest(this, pessoa));
    PLACAR.registrar(igual, || format!("remove_transfer_interest({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}



// ---------- 210a leva: a história punível da imprensa, o tipo de lesão sorteado e a categoria da notícia (media.cpp, injury.cpp, news.cpp) ----------

/// { 0x7357c0 } (thiscall, ret 4, media.cpp): o tipo da história guardada na imprensa sobre o jogador
/// quando é de indisciplina (lista de tipos em 0xa17be0, terminada em 0xff), senão 0. Histórias: [+0]
/// vetor de 0x17 bytes, [+8] quantas; em +8 o id do jogador, em +0x11 o tipo (palavra). Jogador nulo ou
/// com +0x39 zerado: 0.
// ponytail: só o ax é fiel; os 16 bits altos do eax da original são resto de registrador (quem chama usa ax)
/// the kind of the media story about the player when it is a misbehaviour kind, else 0
/// Original: `Media::get_finable_story_kind` (0x7357c0, media.cpp, thiscall).
///
/// [pt] o tipo da história da imprensa sobre o jogador quando é de indisciplina, senão 0
pub(crate) unsafe extern "thiscall" fn get_finable_story_kind(midia: usize, jogador: usize) -> u16 {
    if jogador == 0 || *((jogador + 0x39) as *const u32) == 0 {
        return 0;
    }
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    let id = *(jogador as *const u32);
    for i in 0..n.max(0) as usize {
        let e = base.wrapping_add(i * 0x17);
        if core::ptr::read_unaligned((e + 8) as *const u32) != id || *(0xa17be0 as *const u8) == 0xff {
            continue;
        }
        let tipo = core::ptr::read_unaligned((e + 0x11) as *const u16);
        let mut p = 0xa17be0usize;
        loop {
            if tipo == *(p as *const i8) as i16 as u16 {
                return tipo;
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
// ponytail: só o al é fiel (a original devolve char)


pub(crate) unsafe extern "thiscall" fn comparar_get_finable_story_kind(midia: usize, jogador: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u16>(IDX_GET_FINABLE_STORY_KIND)(midia, jogador);
    let n = get_finable_story_kind(midia, jogador);
    PLACAR.registrar(r == n, || format!("get_finable_story_kind original={} rust={}", r, n));
    r
}



// ---------- 181a leva: as notícias do jogador na mídia e a importância da partida de copa ----------

/// O registro de notícias da mídia (`this`: +0 os registros de 0x17 bytes, +8 quantos): algum com o
/// jogador em +8 que `aceita`?
pub(crate) unsafe fn alguma_noticia(midia: usize, id: i32, aceita: impl Fn(usize) -> bool) -> bool {
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
    alguma_noticia(midia, *id, |r| *((r + 0x10) as *const u8) == 2) as u32
}

/// whether the player has any story in the media log
/// Original: `has_story` (0x735840, media.cpp, thiscall).
///
/// [pt] { 0x735840 } (thiscall, ret 4, media.cpp): o jogador `*id` tem alguma notícia? Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn has_story(midia: usize, id: *const i32) -> u8 {
    alguma_noticia(midia, *id, |_| true) as u8
}


pub(crate) unsafe extern "thiscall" fn comparar_story_with_response_two(midia: usize, id: *const i32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(IDX_STORY_WITH_RESPONSE_TWO)(midia, id);
    let n = story_with_response_two(midia, id);
    PLACAR.registrar(r == n, || format!("story_with_response_two({}) original={} rust={}", *id, r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_has_story(midia: usize, id: *const i32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(IDX_HAS_STORY)(midia, id);
    let n = has_story(midia, id);
    PLACAR.registrar(r == n, || format!("has_story({}) original={} rust={}", *id, r, n));
    r
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
pub(crate) unsafe extern "stdcall" fn media_queue_player_apology_news(jogador: usize, clube: usize, historia: i16) {
    if jogador == 0 || read_i32(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    crate::comparacao::ramo("media_queue_player_apology_news");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdb, 0);
    gravar(n, 0, read_i32(jogador) as u32);
    gravar(n, 1, read_i32(read_i32(jogador + 4) as usize + 0x33) as u32);
    gravar(n, 2, read_i32(read_i32(jogador + 8) as usize + 0x33) as u32);
    let terceiro = read_i32(jogador + 0xc) as usize;
    gravar(n, 3, if terceiro == 0 { u32::MAX } else { read_i32(terceiro + 0x33) as u32 });
    gravar(n, 4, read_i32(clube) as u32);
    let h = historia as i32;
    gravar(n, 5, h as u32);
    gravar(n, 6, 0);
    let v = if (h as u32) <= 0x23 && *((0x72bd2c + h as usize) as *const u8) == 0 { sorteio(4) } else { 0 };
    gravar(n, 7, v as u32);
    ao_tecnico(read_i32(G_NEWS) as usize, n, clube);
}

/// Comparador: a notícia sorteia e vai para a caixa do técnico: só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "stdcall" fn comparar_media_queue_player_apology_news(jogador: usize, clube: usize, historia: i16) {
    orig::<unsafe extern "stdcall" fn(usize, usize, i16)>(IDX_MEDIA_QUEUE_PLAYER_APOLOGY_NEWS)(jogador, clube, historia)
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
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let reputacao = |nacao: usize| core::ptr::read_unaligned((nacao + nation::REPUTATION) as *const i16) as i32;
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
        let mut peso = sorteio(reputacao(nacao).wrapping_mul(*((p + 0x22) as *const u8) as i32));
        let mesma_nacao = if assunto == 0 {
            if nacao != u(clube + club::NATION) {
                continue;
            }
            true
        } else {
            nacao == u(assunto + 0x1a)
        };
        if mesma_nacao {
            peso = peso.wrapping_add(peso);
        } else {
            let corte = sorteio(100).wrapping_add(0x1964);
            if reputacao(nacao) < corte
                || core::ptr::read_unaligned((u(p + 0x69) + 0xc) as *const i16) < 0x1770
                || nao_gosta(assunto, p) != 0
            {
                continue;
            }
        }
        if melhor == 0 || peso > maior {
            maior = peso;
            melhor = p;
        }
    }
    melhor
}

pub(crate) unsafe extern "thiscall" fn comparar_pick_former_international_pundit(this: usize, clube: usize, assunto: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_PICK_FORMER_INTERNATIONAL_PUNDIT);
    if assunto == 0 && clube == 0 {
        return o(this, clube, assunto);
    }
    // o comentarista sai só do sorteio
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube, assunto),
        || crate::aleatorio::sem_contar(|| pick_former_international_pundit(this, clube, assunto)));
    PLACAR.registrar(igual && r == n, || format!("pick_former_international_pundit {:#x} x {:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
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
pub(crate) unsafe extern "thiscall" fn pick_ai_manager_response(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let nota_da_historia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_MEDIA_RATE_PLAYER_FORM_FOR_STORY);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let listada_pelo_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    if tecnico == 0 || read_usize(tecnico + 0x69) == 0 {
        return 3;
    }
    if jogador != 0 && read_usize(jogador + 0x61) == 0 {
        return 3;
    }
    if tecnico == jogador {
        return 3;
    }
    let mut clube = read_usize(tecnico + 0x39);
    if clube == 0 {
        clube = read_usize(tecnico + 0x24);
        if clube == 0 {
            return 3;
        }
    }
    let contrato = contract_with_club(read_usize(G_CONTRACT_MANAGER), jogador, clube);
    if contrato == 0 {
        return 3;
    }
    let mut nota = tecnico as i32; // sem jogador: o argumento que ocupa a vaga
    if jogador != 0 {
        nota = ftol(nota_da_historia(this, jogador, clube, contrato, tipo, 3, 0.0) * *(0x968990 as *const f32) as f64);
    }
    let f = || read_usize(tecnico + 0x69); // a ficha de técnico
    let ch = |o: usize| chance(f(), *((f() + o) as *const i8)) as i8 as i32; // o conhecimento pelo atributo +o
    let k = tipo as i16;
    if k == 0x83 {
        let t = cpo6_tipo(contrato);
        if t == 0x10 || t == 0x20 {
            return 1;
        }
        if t == 0x30 && read_i8_as_i32(contrato + 0x3f) > 1 {
            return 1;
        }
        if read_i16(read_usize(jogador + 0x61) + 0xb) < 0x1482 && ch(0x14) > 0xf {
            return 2;
        }
        if cpo6_tipo(contrato) == 0x70 && ch(0x14) > 0xa {
            return 2;
        }
        return if sorteio(ch(0x19)) != 0 { 1 } else { 3 };
    }
    if k == 0x82 {
        if nao_gosta(jogador, tecnico) != 0 {
            let a = read_i8_as_i32(tecnico + 0x5b);
            if a < 0xf && sorteio(a) == 0 {
                let x = sorteio(ch(0x19));
                let y = sorteio(0x14);
                if x < y {
                    return 2;
                }
            }
            let a = read_i8_as_i32(tecnico + 0x5b);
            if a <= 0xe || sorteio(a) <= 10 {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0x14);
            return if x <= y { 3 } else { 1 };
        }
        if gosta(jogador, tecnico) != 0 {
            let x = sorteio(read_i8_as_i32(tecnico + 0x5b));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0xf);
            return if x <= y { 3 } else { 1 };
        }
        if read_i8_as_i32(tecnico + 0x5d) < 6 {
            let y = sorteio(0x14);
            if read_i8_as_i32(tecnico + 0x5d) < y {
                return 1;
            }
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        if x > y {
            return 1;
        }
        if read_i8_as_i32(tecnico + 0x5c) <= 0xf {
            return 3;
        }
        let y = sorteio(10);
        return if read_i8_as_i32(tecnico + 0x5c) <= y { 3 } else { 2 };
    }
    if k == 0x1f {
        if listada(jogador, clube) != 0 {
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
    if nao_gosta(jogador, tecnico) != 0 && nota < 5 {
        if sorteio(read_i8_as_i32(tecnico + 0x5b)) != 0 {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 2 };
    }
    if gosta(jogador, tecnico) != 0 && nota > -5 {
        let x = sorteio(read_i8_as_i32(tecnico + 0x5b));
        let y = sorteio(10);
        if x >= y {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 1 };
    }
    // a soma do conhecimento (+0x19), de +0x1d da ficha e de +0x59 do técnico
    let soma = || ch(0x19) + read_i8_as_i32(f() + 0x1d) + read_i8_as_i32(tecnico + 0x59);
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
            if sorteio(read_i8_as_i32(tecnico + 0x5a)) == 0 {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(tecnico + 0x59));
            let y = sorteio(0x28);
            if x > y {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(f() + 0x1d));
            let y = sorteio(0x32);
            if x <= y { 3 } else { 1 }
        }
        0x28..=0x3b => {
            let x = sorteio(read_i8_as_i32(jogador + 0x5b));
            let y = sorteio(0x14);
            if x > y {
                let a = sorteio(ch(0x19));
                let b = sorteio(0x14 - nota);
                if a > b {
                    return 1;
                }
                if sorteio(ch(0x19)) != 0 {
                    return 3;
                }
                return if nota >= 0xa { 3 } else { 2 };
            }
            let a = sorteio(ch(0x19));
            let b = sorteio(10);
            if a > b {
                let mut arg = 0;
                if *((read_usize(jogador + 0x61) + 0x44) as *const u8) != 0 {
                    // o registro de 0x4f bytes da pessoa em [0xae2d0c] (+0xa, +0xb bit 7)
                    let reg = |o: usize| *((read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(jogador) as usize).wrapping_mul(0x4f)) + o) as *const u8);
                    let e = sorteio((2 - (reg(0xb) >> 7) as i32) * 20);
                    if sorteio(reg(0xa) as i32) > e {
                        arg = 1;
                    }
                }
                if sorteio(arg) == 0 && nota < 0xa {
                    return 2;
                }
            }
            if sorteio(read_i8_as_i32(read_usize(jogador + 0x61) + 0x45)) == 0 {
                return 1;
            }
            let x = sorteio(read_i8_as_i32(tecnico + 0x59));
            let y = sorteio(0x14 - nota);
            if x <= y { 3 } else { 1 }
        }
        0 | 0xd | 0x11 | 0x52 | 0x23 | 0xe | 2 | 0x21 | 0x22 | 9 | 0xc => {
            crate::comparacao::ramo("cpo_6 tipos comuns");
            let t = cpo6_tipo(contrato);
            if t == 0x30 || t == 0x10 || t == 0x20 {
                let e = entrada(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), clube, jogador, 0, 0);
                if e != 0 && read_usize(e + 0xf) == read_usize(jogador) && nota > -10 {
                    crate::comparacao::ramo("cpo_6 sorteio descartado");
                    sorteio(read_i8_as_i32(tecnico + 0x59)); // o original descarta o resultado
                }
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(10);
            if x < y {
                let x = sorteio(read_i8_as_i32(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(read_i8_as_i32(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y {
                        let x = sorteio(read_i8_as_i32(tecnico + 0x59));
                        let y = sorteio(10);
                        if x < y {
                            let x = sorteio(nota);
                            let y = sorteio(-5);
                            if x < y {
                                return 2;
                            }
                        }
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x24);
            if x <= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x > y { 1 } else { 3 }
        }
        8 | 0x25 => {
            if sorteio(read_i8_as_i32(f() + 0x1d) * 2) == 0 {
                rejeitar(tecnico, jogador, 1);
            }
            if ch(0x19) < 0xa {
                let x = sorteio(read_i8_as_i32(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(read_i8_as_i32(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x24);
            if x <= y { 3 } else { 1 }
        }
        1 => {
            if sorteio(ch(0x19)) == 0 {
                let x = sorteio(read_i8_as_i32(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(read_i8_as_i32(f() + 0x1d));
                    let y = sorteio(5);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x1e);
            if x <= y { 3 } else { 1 }
        }
        0x1e | 0x20 => {
            let x = sorteio(read_i8_as_i32(tecnico + 0x5d));
            let y = sorteio(0x14);
            if x < y {
                let x = sorteio(read_i8_as_i32(f() + 0x1d));
                let y = sorteio(0xf);
                if x < y {
                    return 2;
                }
            }
            let x = sorteio(read_i8_as_i32(f() + 0x1d) + read_i8_as_i32(tecnico + 0x59));
            let y = sorteio(0x96);
            if x > y {
                if read_i16(read_usize(jogador + 0x61) + 0xb) as i32 <= read_i16(clube + club::REPUTATION) as i32 + 0x1f4 {
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
            if sorteio(read_i8_as_i32(tecnico + 0x5a)) == 0 {
                base += sorteio(10);
                base -= sorteio(10);
            }
            let x = sorteio(read_i8_as_i32(f() + 0x14));
            let y = sorteio(0xf - base);
            if x > y {
                return 2;
            }
            let x = sorteio(read_i8_as_i32(tecnico + 0x57));
            let y = sorteio(base + 0xf);
            if x <= y { 3 } else { 1 }
        }
        3 | 4 => {
            let x = sorteio(read_i8_as_i32(f() + 0x14));
            let y = sorteio(0x32);
            if x > y && sorteio(read_i8_as_i32(f() + 0x1d)) == 0 {
                let x = sorteio(nota);
                let y = sorteio(-5);
                if x < y {
                    rejeitar(tecnico, jogador, 1);
                    return 2;
                }
            }
            let t = cpo6_tipo(contrato);
            if t != 0 && t != 0x70 && t != 0x80 && listada_pelo_clube(jogador, 0) == 0 {
                let x = sorteio(read_i8_as_i32(tecnico + 0x59));
                let y = sorteio(6);
                if x > y {
                    let x = sorteio(ch(0x19));
                    let y = sorteio(6);
                    if x > y {
                        let x = sorteio(nota);
                        let y = sorteio(-5);
                        if x > y {
                            return 1;
                        }
                    }
                }
            }
            let t = cpo6_tipo(contrato);
            if t != 0x70 && t != 0x40 {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(tecnico + 0x59));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(tecnico + 0x5b));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        5 => {
            if sorteio(read_i8_as_i32(tecnico + 0x59)) != 0 && sorteio(ch(0x19)) != 0 && sorteio(read_i8_as_i32(jogador + 0x5b)) != 0 {
                let x = sorteio(nota);
                let y = sorteio(-5);
                if x > y {
                    return 1;
                }
            }
            if sorteio(ch(0x19)) != 0 {
                return 3;
            }
            let x = sorteio(read_i8_as_i32(jogador + 0x5a));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        6 => {
            if sorteio(read_i8_as_i32(tecnico + 0x59)) != 0 {
                let x = sorteio(read_i8_as_i32(f() + 0x1f));
                let y = sorteio(10);
                if x > y {
                    let x = sorteio(read_i8_as_i32(jogador + 0x5b));
                    let y = sorteio(5);
                    if x > y {
                        let x = sorteio(read_i8_as_i32(read_usize(jogador + 0x61) + 0x25));
                        let y = sorteio(0x1e);
                        if x < y {
                            let x = sorteio(nota);
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
            let x = sorteio(read_i8_as_i32(read_usize(jogador + 0x61) + 0x25));
            let y = sorteio(0x32);
            if x <= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        _ => 3,
    }
}

/// Comparador: a resposta sorteia e pode pôr o jogador entre os rejeitados do técnico (0x547af0): lado a lado no
/// estado do gerador e no bloco das preferências do técnico (+0x65, +0x1c..+0x34), a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_pick_ai_manager_response(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_PICK_AI_MANAGER_RESPONSE);
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if tecnico != 0 && read_usize(tecnico + 0x65) != 0 {
        regioes.push((read_usize(tecnico + 0x65) + 0x1c, 0x18));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, tecnico, jogador, tipo),
        || crate::aleatorio::sem_contar(|| pick_ai_manager_response(this, tecnico, jogador, tipo)),
    );
    PLACAR.registrar(igual && r == n, || format!("pick_ai_manager_response(tipo {:#x}) original={} rust={} {:x?} x {:x?}", tipo as i16, r, n, mo.0, mr.0));
    r
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
    let ficha = read_usize(clube + club::STADIUM);
    if ficha == 0 || read_usize(ficha + 0x38) == 0 || read_usize(clube + club::DIVISION) == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_6 apuro financeiro");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    construir(n, 0x1790, 0);
    gravar(n, 0, read_usize(clube) as u32);
    gravar(n, 1, read_i16(clube + club::REPUTATION) as i32 as u32);
    let v = sorteio(2); // o sorteio da notícia
    gravar(n, 2, v as u32);
    let noticias = read_usize(G_NEWS);
    let divisao = read_usize(clube + club::DIVISION);
    if read_i16(clube + club::REPUTATION) >= 9000 && divisao != 0 && read_i16(divisao + 0x69) > 12 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        todos(noticias, n);
    } else {
        let pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
        pais(noticias, n, read_usize(clube + club::NATION));
    }
}

/// Comparador: a notícia sorteia (0x90cfc0) e vai para as caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_post_financial_trouble_news(this: usize, clube: usize) {
    crate::comparacao::ramo("post_financial_trouble_news");
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_FINANCIAL_TROUBLE_NEWS)(this, clube)
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
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if read_u32(clube + club::DIVISION) == 0 && selecao(clube) == 0 {
        return u32::MAX;
    }
    let tecnico = read_u32(clube + club::MANAGER) as usize;
    if read_u32(tecnico + 0x69) == 0 {
        return u32::MAX;
    }
    let mut rejeitados = [0usize; 20];
    let mut n = clube as u32; // a vaga do argumento: a 0x5480f0 zera o byte antes de contar
    let rejeitadas: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_STAFF_GET_DISLIKED_STAFF);
    rejeitadas(tecnico, rejeitados.as_mut_ptr() as usize, &mut n as *mut u32 as usize);
    let mut k = 0i8;
    while k < n as u8 as i8 {
        let p = *rejeitados.as_ptr().add(k as u8 as usize); // como o original, sem conferir o tamanho (a 0x5480f0 dá até 3)
        if p != 0 && read_u32(p + 0x69) != 0 {
            let nota = *((read_u32(p + 0x69) as usize + 0xa) as *const i16) as i32;
            if nota.wrapping_add(0x2ee) < nota && cpe1_sorteio(3) != 0 {
                return read_u32(p);
            }
        }
        k = k.wrapping_add(1);
    }
    for rival in [0xb3usize, 0xb7, 0xbb] {
        let r = read_u32(clube + rival) as usize;
        if r == 0 || read_u32(r + 0xcf) == 0 || !cph7_sorteado(r) {
            continue;
        }
        if read_u32(r + 0x57) != read_u32(clube + club::DIVISION) {
            continue;
        }
        let reputacao = |c: usize| *((c + 0x80) as *const i16) as i32;
        if reputacao(clube) + 0x2ee >= reputacao(r) {
            continue; // o rival não é bem maior
        }
        if cpe1_sorteio(3) != 0 {
            return read_u32(read_u32(r + 0xcf) as usize);
        }
    }
    let confronto: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let mut b = 0i8;
    while b < 0x32 {
        let limite = (read_u32(G_CLUB_COUNT) as i32).wrapping_sub((read_u32(G_NATION_COUNT) as i32).wrapping_mul(2));
        let i = cpe1_sorteio(limite) as usize;
        let c = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
        if confronto(clube, c, limite) != 0 && read_u32(c + 0xcf) != 0 && cph7_sorteado(c) {
            return read_u32(read_u32(c + 0xcf) as usize);
        }
        b += 1;
    }
    u32::MAX
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio, a cópia em sem_contar (as escritas são só nas
/// locais da pilha de cada uma).
pub(crate) unsafe extern "thiscall" fn comparar_pick_rival_manager_id(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PICK_RIVAL_MANAGER_ID);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pick_rival_manager_id(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pick_rival_manager_id({:x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// club with a nation and manager: news 0x178e (club, manager, type, person, a, b, nation, c) to the nation's managers
/// if the reputation is above 4750, else to the club
/// Original: `Media::queue_manager_fa_complaint_story` (0x758de0, media.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x758de0 } (thiscall, ret 0x18, media.cpp, SEH da notícia local, sem destrutor): `Media::
/// queue_manager_fa_complaint_story(clube, pessoa, tipo, a, b, c)`, pela disciplina (0x55e300) e pelo MANAGER_MANAGER
/// (0x6a9e60, 0x6ab800). Clube com nação (+0x53) e técnico (+0xcf), e pessoa: a notícia 0x178e (o clube; o técnico, id
/// e chaves, ou −1; o tipo; a pessoa, id e chaves; a; b; a nação ou −1; c) aos técnicos da nação (0x7863f0) se a
/// reputação (+0x80) pass_ball de 4750, senão ao clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn queue_manager_fa_complaint_story(this: usize, clube: usize, pessoa: usize, tipo: u32, a: u32, b: u32, c: u32) {
    let _ = this;
    if clube == 0 || read_usize(clube + club::NATION) == 0 || read_usize(clube + club::MANAGER) == 0 || pessoa == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_10 noticia da queixa a federacao");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x178e, 0);
    gravar(nt, 0, read_u32(clube));
    let tecnico = read_usize(clube + club::MANAGER);
    if tecnico != 0 {
        gravar(nt, 1, read_u32(tecnico));
        gravar(nt, 2, read_u32(read_usize(tecnico + 4) + 0x33));
        gravar(nt, 3, read_u32(read_usize(tecnico + 8) + 0x33));
        gravar(nt, 4, read_u32(read_usize(tecnico + 0xc) + 0x33));
    } else {
        for k in 1..5 {
            gravar(nt, k, u32::MAX);
        }
    }
    gravar(nt, 5, tipo as i8 as u32);
    gravar(nt, 6, read_u32(pessoa));
    gravar(nt, 7, read_u32(read_usize(pessoa + 4) + 0x33));
    gravar(nt, 8, read_u32(read_usize(pessoa + 8) + 0x33));
    gravar(nt, 9, read_u32(read_usize(pessoa + staff::COMMON_NAME) + 0x33));
    gravar(nt, 0xa, a as i8 as u32);
    gravar(nt, 0xb, b as i16 as u32);
    let nacao = read_usize(clube + club::NATION);
    gravar(nt, 0xc, if nacao != 0 { read_u32(nacao) } else { u32::MAX });
    gravar(nt, 0xd, c as i8 as u32);
    let nacao = read_usize(clube + club::NATION);
    if nacao != 0 && read_i16_as_i32(clube + club::REPUTATION) > 0x128e {
        let aos_da_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
        aos_da_nacao(read_usize(G_NEWS), nt, nacao);
    } else {
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        entregar(read_usize(G_NEWS), nt, clube);
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_queue_manager_fa_complaint_story(this: usize, clube: usize, pessoa: usize, tipo: u32, a: u32, b: u32, c: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_QUEUE_MANAGER_FA_COMPLAINT_STORY)(this, clube, pessoa, tipo, a, b, c)
}
