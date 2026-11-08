// ---------- %%LEVA%%a leva: as estatísticas do jogador arquivadas no histórico (a troca de clube, o empréstimo, a aposentadoria) e os jogos e gols pelo clube recontados (player_stats.cpp) ----------

unsafe fn cqh8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// O ensaio do comparador: a cópia roda sem gravar no arquivo de histórico (FILE_LLIST::add_node 0x58e340, que escreve
/// no arquivo e move as posições da ficha); os nós que ela gravaria entram direto na soma dos jogos e gols pelo clube.
static mut ARCHIVE_STATS_DRY_RUN: bool = false;
static mut ARCHIVE_STATS_PENDING_TOTALS: (u16, u16) = (0, 0);
static mut ARCHIVE_STATS_CLUB_ID: i32 = -1;

/// O nó do histórico (0x2b bytes montados na pilha): +0 o ano da temporada do clube (0x669950), +2 o clube (−1 sem),
/// +6 os 32 bytes da categoria, +0x26 o valor, +0x2a o tipo (3 bits) e a categoria × 8.
unsafe fn career_history_node(clube: usize, dados: Option<usize>, valor: u32, marca: u8) -> [u8; 0x2b] {
    let ano: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x669950usize);
    let mut r = [0u8; 0x2b];
    r[0..2].copy_from_slice(&ano(clube).to_le_bytes());
    let id = if clube == 0 { 0xffff_ffffu32 } else { cqh8_d(clube) as u32 };
    r[2..6].copy_from_slice(&id.to_le_bytes());
    if let Some(d) = dados {
        core::ptr::copy_nonoverlapping(d as *const u8, r.as_mut_ptr().add(6), 32);
    }
    r[0x26..0x2a].copy_from_slice(&valor.to_le_bytes());
    r[0x2a] = marca;
    r
}

/// Grava o nó no fim da lista do histórico da ficha (0x58e340 no arquivo [this], posições +0x18/+0x1c do registro);
/// no ensaio, só soma.
unsafe fn append_history_node(this: usize, reg: usize, no: &[u8; 0x2b]) {
    if ARCHIVE_STATS_DRY_RUN {
        if i32::from_le_bytes([no[2], no[3], no[4], no[5]]) == ARCHIVE_STATS_CLUB_ID && no[0x2a] & 0xf8 == 0x10 {
            ARCHIVE_STATS_PENDING_TOTALS.0 = ARCHIVE_STATS_PENDING_TOTALS.0.wrapping_add(no[6] as u16);
            ARCHIVE_STATS_PENDING_TOTALS.1 = ARCHIVE_STATS_PENDING_TOTALS.1.wrapping_add(no[8] as u16);
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) = core::mem::transmute(0x58e340usize);
    f(cqh8_d(this), reg + 0x18, reg + 0x1c, no.as_ptr());
}

/// { 0x7c5fe0 } (thiscall, ret 0x18, player_stats.cpp): `PlayerStats::archive_player_stats(pessoa, valor, empréstimo,
/// fim, clube, outro)`, pelas saídas de clube (contratos 0x4c7ac0/0x4cbd20/0x4d01e0/0x4d07e0, 0x690730 o elenco, 0x7c1350
/// as seleções). O tipo do nó: 2 com `empréstimo`, 1 com `fim`, 3 com `outro`, senão 0. Pessoa nula: a caixa (linha
/// 0xf28) e 0; sem ficha: 0. Cada uma das seis categorias com registro (o registro de 39 bytes da ficha em [this+0x30])
/// vai para o histórico (o ano e o clube atuais, os 32 bytes do registro em [this+4], `valor`, a categoria e o tipo) e
/// o registro é zerado. Sem a sexta categoria, um nó vazio da categoria 6 com `valor` quando: `valor` ≥ 0 ou `fim`; ou
/// `clube` ≥ 0 e: fora da temporada da nação do clube (a tabela 0xb63d8a); ou, sem a nação ou na temporada dela, menos de
/// 365 dias desde a entrada no clube (+0x3e/+0x42, 0x54cdb0). Com `clube` ≥ 0 ou `valor` ≥ 0: recontam-se os jogos (+0x20) e os gols (+0x22) pelo
/// `clube` nos nós da categoria 2 do histórico (0x58f120). Devolve 1.
unsafe extern "thiscall" fn archive_player_stats(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let tipo: u8 = if emprestimo as u8 != 0 {
        2
    } else if fim as u8 != 0 {
        1
    } else if outro != 0 {
        3
    } else {
        0
    };
    if p == 0 {
        show_error_at(0xa78310, 0xf28);
        return 0;
    }
    let clube = cqh8_d(p + 0x39);
    let f = cqh8_d(p + 0x61);
    if f == 0 {
        return 0;
    }
    let reg = cqh8_d(this + 0x30).wrapping_add((cqh8_i(f) as isize * 0x27) as usize);
    let mut sexta = false;
    for cat in 0..6usize {
        let id = cqh8_i(reg + 4 * cat);
        if id == -1 {
            continue;
        }
        let dados = cqh8_d(this + 4).wrapping_add((id as isize * 0x20) as usize);
        let no = career_history_node(clube, Some(dados), valor, (((cat + 1) as u8) << 3) | tipo); // cqh8: a categoria e o tipo do nó
        crate::comparison::branch("cqh_8 archived category");
        append_history_node(this, reg, &no);
        if cat == 5 {
            sexta = true;
        }
        core::ptr::write_bytes(dados as *mut u8, 0, 32); // cqh8: o registro da categoria zerado
    }
    // sem nação do clube (ou fora da tabela) ou na temporada dela: a marca que pede os 365 dias
    let marca = if clube == 0 || cqh8_d(clube + 0x53) == 0 {
        true
    } else {
        let linha: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(0x669340usize);
        let k = linha(cqh8_d(clube + 0x53));
        if !(0..0x23).contains(&k) {
            true
        } else {
            let t = 0xb63d8a + k as usize * 0x48;
            let (dia, ano) = (core::ptr::read_unaligned(0xae2c90 as *const i16), core::ptr::read_unaligned(0xae2c92 as *const u16));
            let (d0, a0) = (core::ptr::read_unaligned(t as *const i16), core::ptr::read_unaligned((t + 2) as *const u16));
            let (d1, a1) = (core::ptr::read_unaligned((t + 0xf) as *const i16), core::ptr::read_unaligned((t + 0x11) as *const u16));
            let depois_do_inicio = (dia > d0 && ano == a0) || ano > a0;
            let antes_do_fim = (dia < d1 && ano == a1) || ano < a1;
            depois_do_inicio && antes_do_fim
        }
    };
    if !sexta {
        let gravar = if (valor as i32) > -1 || fim as u8 != 0 {
            true
        } else if (clube_id as i32) <= -1 {
            false
        } else if !marca {
            true
        } else {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            dias(0xae2c90, cqh8_d(p + 0x3e) as u32, cqh8_d(p + 0x42) as u32) < 0x16d
        };
        if gravar {
            let no = career_history_node(clube, None, valor, 0x30 | tipo); // cqh8: o nó vazio da sexta categoria
            crate::comparison::branch("cqh_8 on empty");
            append_history_node(this, reg, &no);
        }
    }
    if (clube_id as i32) > -1 || (valor as i32) > -1 {
        core::ptr::write_unaligned((reg + 0x20) as *mut u16, 0);
        core::ptr::write_unaligned((reg + 0x22) as *mut u16, 0);
        let nos: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x58f120usize);
        let lista = nos(cqh8_d(this), cqh8_d(reg + 0x18) as u32, cqh8_d(reg + 0x1c) as u32);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqh8_d(lista + 2) as u32);
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            let mut no = proximo(lista);
            while no != 0 {
                if cqh8_d(no + 2) as u32 == clube_id && *((no + 0x2a) as *const u8) & 0xf8 == 0x10 {
                    crate::comparison::branch("cqh_8 matches by club");
                    let a = (reg + 0x20) as *mut u16;
                    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(*((no + 6) as *const u8) as u16)); // cqh8: os jogos pelo clube
                    let g = (reg + 0x22) as *mut u16;
                    core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(*((no + 8) as *const u8) as u16));
                }
                no = proximo(lista);
            }
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
            destruir(lista);
            apagar(lista);
        }
        if ARCHIVE_STATS_DRY_RUN {
            let a = (reg + 0x20) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(ARCHIVE_STATS_PENDING_TOTALS.0));
            let g = (reg + 0x22) as *mut u16;
            core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(ARCHIVE_STATS_PENDING_TOTALS.1));
        }
    }
    1
}

/// Comparador em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem gravar no arquivo (os nós que gravaria entram na
/// soma dos jogos e gols), depois a original a partir do mesmo estado; compara a devolução, os seis registros de 32 bytes
/// das categorias e os jogos e gols pelo clube (+0x20/+0x22 do registro da ficha). As posições do histórico (+0x18,
/// +0x1c), que só a gravação da original move, ficam fora. Pessoa nula ou sem ficha: só a original.
unsafe extern "thiscall" fn compare_archive_player_stats(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    if p == 0 || cqh8_d(p + 0x61) == 0 {
        return o(this, p, valor, emprestimo, fim, clube_id, outro);
    }
    let reg = cqh8_d(this + 0x30).wrapping_add((cqh8_i(cqh8_d(p + 0x61)) as isize * 0x27) as usize);
    let mut regioes = vec![(reg + 0x20, 4)];
    for cat in 0..6usize {
        let id = cqh8_i(reg + 4 * cat);
        if id != -1 {
            regioes.push((cqh8_d(this + 4).wrapping_add((id as isize * 0x20) as usize), 32));
        }
    }
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    ARCHIVE_STATS_DRY_RUN = true;
    ARCHIVE_STATS_PENDING_TOTALS = (0, 0);
    ARCHIVE_STATS_CLUB_ID = clube_id as i32;
    let n = crate::rng::pause_trace(|| archive_player_stats(this, p, valor, emprestimo, fim, clube_id, outro));
    ARCHIVE_STATS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    let r = o(this, p, valor, emprestimo, fim, clube_id, outro);
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    SCOREBOARD.record(r == n && mo == mr, || format!("archive_player_stats(pessoa {:x}, valor {}, clube {}) original={} rust={} {:x?} x {:x?}",
        p, valor as i32, clube_id as i32, r, n, mo.0, mr.0));
    r
}
