// ---------- %%LEVA%%a leva: os boatos de transferência, a notícia de que o clube não vende o jogador e a notícia do título (media.cpp) ----------

unsafe fn cqc10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc10_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqc10_u(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn cqc10_clube(id: i32) -> usize {
    cqc10_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245))
}

unsafe fn media_is_human_club(c: usize, a: u32, b: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(c, a, b, 0, 0)
}

/// Os clubes humanos que recebem notícias do jogador (0x8d1da0 em [0xde1f54]: 4 na pilha, ret 0x10; o ecx da original é
/// o gerenciador das transferências).
unsafe fn player_news_recipients(jogador: usize, marcas: u32, saida: *mut usize, max: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    f(cqc10_d(0xde1f54), jogador, marcas, saida, max)
}

/// O id e os três nomes (+0x33 de +4, +8, +0xc) de uma pessoa, nos índices `k`..`k+3` da notícia.
unsafe fn set_person_name_slots(gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    gravar(n, k, cqc10_d(p) as u32);
    gravar(n, k + 1, cqc10_d(cqc10_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cqc10_d(cqc10_d(p + 8) + 0x33) as u32);
    gravar(n, k + 3, cqc10_d(cqc10_d(p + 0xc) + 0x33) as u32);
}

/// Acrescenta `c` aos destinos se ainda não estiver lá (até 30).
fn add_unique_recipient(dest: &mut [usize; 30], n: &mut u8, c: usize) {
    if !dest[..(*n as i8).max(0) as usize].contains(&c) {
        dest[*n as i8 as usize] = c;
        *n = n.wrapping_add(1);
    }
}

/// { 0x754a30 } (thiscall, ret 0, media.cpp, quadro SEH sem destrutor no caminho normal):
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
unsafe extern "thiscall" fn process_transfer_interest(this: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let resposta: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x735870usize);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    if cqc10_i(this + 0x14) == 0 {
        return;
    }
    // ponytail: o técnico do dono fica da volta anterior quando o jogador não tem contrato (a original não o regrava);
    // na primeira volta a original lê a pilha sem valor: aqui 0
    let mut tec_dono: usize = 0;
    let mut i = 0i32;
    while i < cqc10_i(this + 0x14) {
        let r = cqc10_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(25));
        let pessoa = cqc10_d(0xae23bc).wrapping_add((cqc10_i(r) as usize).wrapping_mul(0x6e));
        let clube = cqc10_clube(cqc10_i(r + 4));
        let c = contract_of(cqc10_d(0xae1050), pessoa);
        core::ptr::write_unaligned(0xae1080 as *mut usize, c);
        let (mut dono, mut vinc) = (0usize, 0usize);
        if c != 0 {
            dono = cqc10_clube(cqc10_i(c + 4));
            if dono != 0 {
                vinc = contract_bond(cqc10_d(0xae1050), pessoa, dono);
                tec_dono = cqc10_d(dono + 0xcf);
            }
        }
        let mut hoje = [0u8; 8];
        core::ptr::copy_nonoverlapping(0xae2c90 as *const u8, hoje.as_mut_ptr(), 8);
        let mut fim = [0u8; 8];
        let p = mais(r + 8, fim.as_mut_ptr(), 0x1f); // cqc10: o interesse vale 31 dias
        crate::comparison::branch("cqc_10 interesse");
        let (hd, ha) = (i16::from_le_bytes([hoje[0], hoje[1]]), u16::from_le_bytes([hoje[2], hoje[3]]));
        let vencido = ha > cqc10_w(p + 2) as u16 || (ha == cqc10_w(p + 2) as u16 && hd > cqc10_w(p));
        if vencido || tec_dono == 0 || cqc10_d(pessoa + 0x61) == 0 {
            let n = cqc10_i(this + 0x14);
            let ultimo = cqc10_d(this + 0xc).wrapping_add((n as usize).wrapping_mul(25)).wrapping_sub(25);
            core::ptr::copy(ultimo as *const u8, r as *mut u8, 25);
            core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1);
            continue; // o último veio para o lugar i: a original volta um e avança um
        }
        if chegar(cqc10_d(0xde1f54), pessoa, clube, 0, 0) == 0 {
            i += 1;
            continue;
        }
        if cqc10_u(r + 0x10) == 0 && {
            crate::comparison::branch("cqc_10 sorteio do boato");
            cqc10_sortear(0x19) == 0 // cqc10: o sorteio do boato
        } {
            // os clubes interessados no jogador
            let mut ids = [-1i32; 5];
            ids[0] = cqc10_i(r + 4);
            *((r + 0x10) as *mut u8) = 1;
            let mut cnt: i8 = 1;
            let mut cancelar = false;
            let mut j = 0i32;
            while j < cqc10_i(this + 0x14) && cnt < 5 {
                let rj = cqc10_d(this + 0xc).wrapping_add((j as usize).wrapping_mul(25));
                if j != i && cqc10_i(rj) == cqc10_i(r) {
                    if cqc10_u(rj + 0x10) != 0 {
                        cancelar = true;
                        break;
                    }
                    ids[cnt as usize] = cqc10_i(rj + 4);
                    cnt += 1;
                    *((rj + 0x10) as *mut u8) = 1;
                }
                j += 1;
            }
            if cancelar {
                i += 1;
                continue;
            }
            crate::comparison::branch("cqc_10 boato");
            let mut noticia = [0u32; 0x3a]; // a notícia local
            let n = noticia.as_mut_ptr() as usize;
            construir(n, 0xfbc, 0);
            set_person_name_slots(gravar, n, 0, pessoa);
            gravar(n, 4, if dono != 0 { cqc10_d(dono) as u32 } else { 0xffff_ffff });
            let mut pede = false;
            let mut sai = false;
            if cqc10_b(pessoa + 0x5b) < 0xf && cqc10_b(pessoa + 0x59) < 0xf {
                crate::comparison::branch("cqc_10 pedido de transferencia");
                let rr = cqc10_sortear(0x30d4); // cqc10: o sorteio da reputação para o pedido
                if (cqc10_w(cqc10_d(pessoa + 0x61) + 0xd) as i32) > rr {
                    let lim = cqc10_b(pessoa + 0x59) as i32 as f64 * 12.5 + 500.0;
                    if lim < cqc10_i(r + 0x15) as f64 {
                        gravar(n, 5, 0x24);
                        pede = true;
                        sai = true;
                    }
                }
            }
            if !sai {
                let s = cqc10_sortear(3);
                gravar(n, 5, s as u32);
            }
            gravar(n, 6, cqc10_d(r + 0x11) as u32);
            let resp = if dono != 0 && media_is_human_club(dono, 0, 1) == 0 {
                resposta(this, cqc10_d(dono + 0xcf), pessoa, 0x1f)
            } else {
                0
            };
            gravar(n, 7, resp);
            gravar(n, 8, forcas(pessoa, dono));
            gravar(n, 9, posicoes(cqc10_d(pessoa + 0x61)) as i16 as i32 as u32);
            if tec_dono != 0 {
                set_person_name_slots(gravar, n, 0xa, tec_dono);
            } else {
                for k in 0xa..=0xd {
                    gravar(n, k, 0xffff_ffff);
                }
            }
            gravar(n, 0xe, (listada(pessoa, 0) != 0) as u32);
            gravar(n, 0xf, cnt as i32 as u32);
            // os interessados por reputação (do maior para o menor)
            let rep = |id: i32| cqc10_w(cqc10_clube(id) + 0x80);
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
            let mut nd: u8 = if cqc10_d(0xde1f54) != 0 { player_news_recipients(pessoa, 0xda, dest.as_mut_ptr(), 0x1e) } else { 0 };
            for &id in ids.iter().take(c) {
                if (nd as i8) >= 0x1e {
                    break;
                }
                add_unique_recipient(&mut dest, &mut nd, cqc10_clube(id));
            }
            let total = cqc10_i(0xae2364);
            for q in 0..16i32 {
                let h = cqc10_d(0xae23bc).wrapping_add(((total - 1 - q) as usize).wrapping_mul(0x6e));
                let hc = cqc10_d(h + 0x39);
                if cqc10_u(h + 0x5f) == 0 || hc == 0 {
                    continue;
                }
                for &id in ids.iter().take(c) {
                    if (nd as i8) >= 0x1e {
                        break;
                    }
                    let ci = cqc10_clube(id);
                    let hc = cqc10_d(h + 0x39);
                    let junto = cqc10_d(ci + 0x57) == cqc10_d(hc + 0x57)
                        || (cqc10_d(ci + 0x53) == cqc10_d(hc + 0x53) && (cqc10_w(ci + 0x80) as i32 - 0xbb8) > cqc10_w(hc + 0x80) as i32);
                    if junto {
                        add_unique_recipient(&mut dest, &mut nd, hc);
                    }
                }
            }
            for &d in dest.iter().take((nd as i8).max(0) as usize) {
                ao_tecnico(cqc10_d(0xb7431c), n, d);
            }
            if pede {
                let reacao: unsafe extern "thiscall" fn(usize, u32, usize, usize, *mut i32, u32) = core::mem::transmute(0x755340usize);
                reacao(this, 0x1f, pessoa, dono, ids.as_mut_ptr(), cnt as u8 as u32);
            }
            if dono != 0 && media_is_human_club(dono, 0, 1) == 0 && resp != 0 {
                if clube != 0 {
                    let rival: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x758010usize);
                    rival(this, pessoa, clube, dono, resp);
                }
                let historia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x741020usize);
                historia(this, pessoa, dono, 0x1f, resp);
            }
            i += 1;
            continue;
        }
        // estado 1: o dono da IA pode avisar que não vende
        if cqc10_u(r + 0x10) == 1 && dono != 0 && media_is_human_club(dono, 1, 1) == 0 {
            let (rd, ra) = (cqc10_w(r + 8), cqc10_w(r + 0xa) as u16);
            let (pd, pa) = (cqc10_w(pessoa + 0x3e), cqc10_w(pessoa + 0x40) as u16);
            let depois = ra > pa || (ra == pa && rd > pd);
            if depois && vinc != 0 && cqc10_u(vinc + 0x49) == 0 && listada(pessoa, 0) == 0 {
                let feliz = cqc10_b(vinc + 0x3d) as i32;
                let s = cqc10_sortear(0x32);
                if feliz > s + 0x28 && cqc10_sortear(0x1e) == 0 {
                    crate::comparison::branch("cqc_10 nao vende");
                    let nao_vende: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(0x587600usize);
                    nao_vende(pessoa, dono, cqc10_d(dono + 0xbf), 0);
                    let mut j = 0i32;
                    while j < cqc10_i(this + 0x14) {
                        let rj = cqc10_d(this + 0xc).wrapping_add((j as usize).wrapping_mul(25));
                        if cqc10_i(rj) == cqc10_i(cqc10_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(25))) {
                            *((rj + 0x10) as *mut u8) = 2;
                        }
                        j += 1;
                    }
                }
            }
        }
        i += 1;
    }
    let n = cqc10_i(this + 0x14);
    if n != 0 {
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cqc10_d(this + 0xc), n, 0x19, 0x757ea0);
    }
}

// ponytail: tira registros (o vetor), sorteia, constrói e entrega notícias e chama as reações e respostas (0x755340,
// 0x758010, 0x741020, 0x587600): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.
unsafe extern "thiscall" fn compare_process_transfer_interest(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x587600 } (cdecl, media.cpp no fluxo; european_cup.cpp no mapa, sem outro chamador; quadro SEH sem destrutor no
/// caminho normal): `news_create_not_for_sale_story(jogador, clube, presidente, outro)`: com ficha e clube, a notícia
/// 0xfbe (0x77bbe0) com o jogador, o outro clube (ou −1), o clube e um sorteio(3); o presidente, se gosta do jogador
/// (0x547810) ou com sorteio(+0x16 da ficha dele) acima de sorteio(20), senão −1 no 7; reputação (+0xd da ficha) acima de
/// 8000: a todos os técnicos (0x786780), senão aos humanos interessados (0x8d1da0 com 0xd8, até 30, 0x7861e0).
unsafe extern "C" fn news_create_not_for_sale_story(jogador: usize, clube: usize, presidente: usize, outro: usize) {
    if jogador == 0 || cqc10_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let mut dest = [0usize; 30];
    let nd: u8 = if cqc10_d(0xde1f54) != 0 { player_news_recipients(jogador, 0xd8, dest.as_mut_ptr(), 0x1e) } else { 0 };
    crate::comparison::branch("cqc_10 noticia nao vende");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0xfbe, 0);
    set_person_name_slots(gravar, n, 0, jogador);
    gravar(n, 4, if outro != 0 { cqc10_d(outro) as u32 } else { 0xffff_ffff });
    gravar(n, 5, cqc10_d(clube) as u32);
    let s = cqc10_sortear(3);
    gravar(n, 6, s as u32);
    let cita = gosta(jogador, presidente) != 0 || (presidente != 0 && cqc10_d(presidente + 0x69) != 0 && {
        let a = cqc10_sortear(cqc10_b(cqc10_d(presidente + 0x69) + 0x16) as i32);
        a > cqc10_sortear(0x14)
    });
    if cita {
        set_person_name_slots(gravar, n, 7, presidente);
    } else {
        gravar(n, 7, 0xffff_ffff);
    }
    let f = cqc10_d(jogador + 0x61);
    if f != 0 && cqc10_w(f + 0xd) > 0x1f40 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(cqc10_d(0xb7431c), n);
    } else {
        let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        for &d in dest.iter().take((nd as i8).max(0) as usize) {
            ao_tecnico(cqc10_d(0xb7431c), n, d);
        }
    }
}

// ponytail: a notícia vai aos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "C" fn compare_news_create_not_for_sale_story(jogador: usize, clube: usize, presidente: usize, outro: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize, usize, usize)>(%%I1%%)(jogador, clube, presidente, outro)
}

/// { 0x75c390 } (thiscall, ret 0xc, media.cpp, quadro SEH sem destrutor no caminho normal):
/// `Media::queue_trophy_praise_story(clube, competição, marca)`, pelos registros do título (0x8693e2, 0x86b000). Clube
/// com técnico, competição e fora das seleções: a notícia 0x2e (0x77bbe0) com o técnico, o clube, a competição, um
/// sorteio(2), o veículo (0x757f30), o comentarista (0x7dab90 com a nação ou −1), a marca, a confiança da diretoria
/// (+6 da linha em [0xb67a44], ou 5000), a liga do clube e os dias do técnico (0x54cdb0); com 180 dias ou mais, a
/// reputação do clube e a entrega ao técnico (0x7861e0).
unsafe extern "thiscall" fn queue_trophy_praise_story(_this: usize, clube: usize, comp: usize, marca: u32) {
    if clube == 0 || cqc10_d(clube + 0xcf) == 0 || comp == 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let veiculo: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x757f30usize);
    let comentarista: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32 = core::mem::transmute(0x7dab90usize);
    let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    crate::comparison::branch("cqc_10 noticia do titulo");
    let mut noticia = [0u32; 0x3a];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x2e, 0);
    set_person_name_slots(gravar, n, 0, cqc10_d(clube + 0xcf));
    gravar(n, 4, cqc10_d(clube) as u32);
    gravar(n, 5, cqc10_d(comp) as u32);
    let s = cqc10_sortear(2);
    gravar(n, 6, s as u32);
    let v = veiculo(clube);
    gravar(n, 7, v);
    let na = cqc10_d(clube + 0x53);
    let c = comentarista(cqc10_d(0xb74430), clube, v, if na != 0 { cqc10_i(na) } else { -1 });
    gravar(n, 8, c as u32);
    gravar(n, 9, marca as i8 as i32 as u32);
    let mm = cqc10_d(0xb67a44);
    let conf = if mm != 0 {
        cqc10_w(cqc10_d(mm).wrapping_add((cqc10_i(clube) as usize).wrapping_mul(0x49)) + 6) as i32
    } else {
        0x1388
    };
    gravar(n, 0xa, conf as u32);
    gravar(n, 0xb, (cqc10_d(clube + 0x57) == comp) as u32);
    let tec = cqc10_d(clube + 0xcf);
    let dias = ate(0xae2c90, cqc10_d(tec + 0x3e) as u32, cqc10_d(tec + 0x42) as u32);
    gravar(n, 0xc, dias as u32);
    if dias < 0xb4 {
        return;
    }
    gravar(n, 0xd, cqc10_w(clube + 0x80) as i32 as u32);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    ao_tecnico(cqc10_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_queue_trophy_praise_story(this: usize, clube: usize, comp: usize, marca: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I2%%)(this, clube, comp, marca)
}
