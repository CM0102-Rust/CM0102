// ---------- %%LEVA%%a leva: o jogador a serviço da seleção: a seleção ou o time das estrelas que o tem na data, o jogador que serve à seleção do técnico e as competições das convocações (national_teams.cpp) ----------

unsafe fn cpz6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz6_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do time das estrelas `g` ([0x9d00a4] ou [0x9d00a8]): o registro de 0x245 bytes em [0xae23b4].
unsafe fn all_star_club(g: usize) -> usize {
    cpz6_d(0xae23b4).wrapping_add(cpz6_d(g).wrapping_mul(0x245))
}

/// { 0x7757d0 } (thiscall, ret 0x14, national_teams.cpp): `NationalTeams::get_international_duty_club(jogador, data,
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
unsafe extern "thiscall" fn get_international_duty_club(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                      adversario: usize) -> usize {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut lista = [0u8; 0x10];
    let pl = lista.as_mut_ptr() as usize;
    criar(pl);
    let sai = |r: usize| -> usize {
        destruir(pl);
        r
    };
    if clube != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(clube) != 0 {
            return sai(0);
        }
        let id = cpz6_d(clube);
        if id == cpz6_d(0x9d00a4) || id == cpz6_d(0x9d00a8) {
            return sai(0);
        }
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (mut lo, mut hi) = ([0u8; 8], [0u8; 8]);
    let (plo, phi) = (lo.as_mut_ptr() as usize, hi.as_mut_ptr() as usize);
    menos(data, plo, 1); // cpz6: a véspera
    mais(data, phi, 1); // cpz6: o dia seguinte
    let nacional = cpz6_d(jogador + 0x24);
    if nacional != 0 {
        let sem_bit = (cpz6_b(jogador + 0x5e) & 4 == 0) as u32;
        // ponytail: a original empurra o lado com os 24 bits altos de ecx sobrando; a 0x5acbe0 só lê o byte
        let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(0x5acbe0usize);
        let n = jogos(cpz6_d(0xae2a58), cpz6_d(plo) as u32, cpz6_d(plo + 2) as u32, cpz6_d(phi) as u32, cpz6_d(phi + 2) as u32,
            cpz6_d(nacional) as u32, pl, u32::MAX, 0xffff_fffe, (sem_bit == 0) as u32);
        let mut servico = false;
        if n as u16 != 0 {
            if adversario == 0 {
                servico = true;
            } else {
                core::ptr::write_unaligned((pl + 0xa) as *mut usize, cpz6_d(pl + 2));
                loop {
                    let j = proximo(pl);
                    if j == 0 {
                        break;
                    }
                    if cpz6_d(j + 0x14) == adversario {
                        servico = true;
                        break;
                    }
                }
            }
        }
        if servico {
            if comentario != 0 {
                texto(comentario, 0xa62ff4, 0xad9c64usize);
            }
            crate::comparison::branch("cpz_6 a servico da selecao");
            return sai(cpz6_d(jogador + 0x24)); // cpz6: a serviço da seleção
        }
        let conv = cpz6_d(this);
        if conv != 0 && sem_bit == 0 {
            core::ptr::write_unaligned((conv + 0xa) as *mut usize, cpz6_d(conv + 2));
            loop {
                let e = proximo(cpz6_d(this));
                if e == 0 {
                    break;
                }
                if cpz6_d(e) != cpz6_d(cpz6_d(jogador + 0x24)) {
                    continue;
                }
                if adversario != 0 && cpz6_d(e + 4) != cpz6_d(adversario) {
                    continue;
                }
                let (dia, ano) = (cpz6_w(e + 8) as i16, cpz6_w(e + 0xa));
                let hoje = cpz6_w(data + 2);
                if ano < hoje || (ano == hoje && dia < cpz6_w(data) as i16) {
                    if comentario != 0 {
                        texto(comentario, 0xa62fc8, 0xad9c64usize);
                    }
                    crate::comparison::branch("cpz_6 no time da selecao");
                    return sai(cpz6_d(jogador + 0x24)); // cpz6: já no time da seleção
                }
            }
        }
    }
    let comp = cpz6_d(cpz6_d(0xadadfc).wrapping_add(cpz6_d(0x9cf690).wrapping_mul(4)));
    if comp == 0 {
        return sai(0);
    }
    let reg = cpz6_d(comp + 0xa3);
    let ano = (cpz6_b(reg + 5) as i8 as i16).wrapping_add(cpz6_w(comp + 0x40) as i16) as u16;
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut jogo = [0u8; 8];
    let pj = jogo.as_mut_ptr() as usize;
    data_nova(pj, cpz6_w(reg + 3) as u32, ano as u32);
    let cj = cpz6_d(jogador + 0x39);
    if cj == 0 {
        return sai(0);
    }
    let liga = cpz6_d(cj + 0x57);
    if liga == 0 || cpz6_d(liga) != cpz6_d(0x9cf590) {
        return sai(0);
    }
    let (jd, ja) = (cpz6_w(pj) as i16, cpz6_w(pj + 2));
    let (ld, la) = (cpz6_w(plo) as i16, cpz6_w(plo + 2));
    if la > ja || (la == ja && ld > jd) {
        return sai(0);
    }
    let (hd, ha) = (cpz6_w(phi) as i16, cpz6_w(phi + 2));
    if ha < ja || (ha == ja && hd < jd) {
        return sai(0);
    }
    for i in 0..0x32usize {
        for g in [0x9d00a8usize, 0x9d00a4] {
            if cpz6_d(all_star_club(g) + 0xd7 + 4 * i) == jogador {
                if comentario != 0 {
                    texto(0xde1f64, 0xa62fa4, 0xad9c64usize);
                }
                crate::comparison::branch("cpz_6 time das estrelas");
                return sai(all_star_club(g));
            }
        }
    }
    sai(0)
}

/// Comparador da 0x7757d0 (2,1 milhões de chamadas até 2002-06-01: o lado a lado leve, sem alocar até 256 bytes): cada
/// uma com a sua lista provisória (como a original); no texto de 0xde1f64 (0x80) e no do comentário (0x40) e na devolução.
unsafe extern "thiscall" fn comparar_get_international_duty_club(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                               adversario: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize>(%%I0%%);
    let regioes = [(0xde1f64usize, 0x80usize), (comentario, 0x40)];
    let reg = if comentario != 0 { &regioes[..] } else { &regioes[..1] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(reg,|| o(this, jogador, data, clube, comentario, adversario),
        || get_international_duty_club(this, jogador, data, clube, comentario, adversario));
    SCOREBOARD.record(igual && r == n, || format!("get_international_duty_club(jogador {:x}, data {:x}, clube {:x}, adversario {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        jogador, data, clube, adversario, r, n, mo, mr));
    r
}

/// { 0x76d9b0 } (stdcall, ret 8, national_teams.cpp): `national_teams_player_is_eligible(jogador, tecnico)`, da busca de
/// jogadores (0x7afa70, 0x807450) e da tela de comparação. Sem ficha de jogador (+0x61), sem a seleção do técnico (+0x24)
/// ou ela não sendo seleção (0x53b900): 0. Já convocado para ela: 1. Seleção do segundo conjunto (0x5409b0, a sub-21)
/// e idade (+0x18) acima de 21: 0. Com +0x22 ou o bit 2 de +0xc do registro de 79 bytes da pessoa ([0xae2d0c]), a
/// nacionalidade (+0x1a) tem de ser a da seleção (+0x53); convocado para outra seleção de outra nação: 0. A nação da
/// seleção tem de ser a primeira ou a segunda (+0x1e) nacionalidade; sendo a nação [0x9cf434] e a segunda também: 0.
/// Senão 1.
unsafe extern "stdcall" fn national_teams_player_is_eligible(jogador: usize, tecnico: usize) -> u32 {
    let sel = cpz6_d(tecnico + 0x24);
    if cpz6_d(jogador + 0x61) == 0 || sel == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(sel) == 0 {
        return 0;
    }
    if cpz6_d(jogador + 0x24) == sel {
        return 1;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(sel) != 0 && cpz6_b(jogador + 0x18) as i8 > 0x15 {
        return 0;
    }
    let nacao = cpz6_d(sel + 0x53);
    let primeira = cpz6_d(jogador + 0x1a);
    if cpz6_b(jogador + 0x22) != 0 && primeira != nacao {
        return 0;
    }
    let p = cpz6_d(0xae2d0c).wrapping_add(cpz6_d(jogador).wrapping_mul(79));
    if cpz6_b(p + 0xc) & 2 != 0 && cpz6_d(jogador + 0x1a) != nacao {
        return 0;
    }
    let outra = cpz6_d(jogador + 0x24);
    if outra != 0 && cpz6_d(outra + 0x53) != nacao {
        return 0;
    }
    let primeira = cpz6_d(jogador + 0x1a);
    if primeira != nacao && cpz6_d(jogador + 0x1e) != nacao {
        return 0;
    }
    let x = cpz6_d(0x9cf434);
    if cpz6_d(nacao) == x {
        let s = cpz6_d(jogador + 0x1e);
        if s != 0 && cpz6_d(s) == x {
            return 0;
        }
    }
    1
}

/// Comparador da 0x76d9b0: pela devolução (só lê).
unsafe extern "stdcall" fn comparar_national_teams_player_is_eligible(jogador: usize, tecnico: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u32>(%%I1%%)(jogador, tecnico);
    let n = national_teams_player_is_eligible(jogador, tecnico);
    SCOREBOARD.record(r == n, || format!("national_teams_player_is_eligible(jogador {:x}, tecnico {:x}) original={} rust={}", jogador, tecnico, r, n));
    r
}

/// { 0x775580 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::collect_duty_competitions(saida)`, dos olheiros
/// (0x7f7010). Sem a lista de convocações [this] ou sem `saida`: 0. Para cada convocação, a competição dela (+4) entra
/// em `saida` (0x68d0d0, 4 bytes) se ainda não estiver lá; devolve quantas entraram (byte).
unsafe extern "thiscall" fn collect_duty_competitions(this: usize, saida: usize) -> u8 {
    let conv = cpz6_d(this);
    if conv == 0 || saida == 0 {
        return 0;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let mut n: u8 = 0;
    core::ptr::write_unaligned((conv + 0xa) as *mut usize, cpz6_d(conv + 2));
    let mut e = proximo(cpz6_d(this));
    while e != 0 {
        core::ptr::write_unaligned((saida + 0xa) as *mut usize, cpz6_d(saida + 2));
        let mut ja = false;
        loop {
            let x = proximo(saida);
            if x == 0 {
                break;
            }
            if cpz6_d(x) == cpz6_d(e + 4) {
                ja = true;
                break;
            }
        }
        if !ja {
            acrescentar(saida, e + 4, 4);
            n = n.wrapping_add(1);
        }
        e = proximo(cpz6_d(this));
    }
    n
}

/// Os valores dos nós de uma lista (cabeça em +2, nó: +0 o item, +6 o próximo), sem mexer no cursor (+0xa).
unsafe fn national_teams_list_items(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpz6_d(lista + 2);
    while no != 0 {
        v.push(cpz6_d(no));
        no = cpz6_d(no + 6);
    }
    v
}

// ponytail: a 0x775580 acrescenta nós (aloca): só a original roda. O plano, feito antes pelos nós (sem mexer nos
// cursores): as competições das convocações que ainda não estão em `saida`, na ordem, sem repetir; depois, a devolução
// é quantas e os últimos itens de `saida` (4 bytes cada) são elas.
unsafe extern "thiscall" fn comparar_collect_duty_competitions(this: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%);
    let conv = cpz6_d(this);
    if conv == 0 || saida == 0 {
        let r = o(this, saida);
        SCOREBOARD.record(r == 0, || format!("collect_duty_competitions(saida {:x}) sem lista original={}", saida, r));
        return r;
    }
    let mut tem: Vec<usize> = national_teams_list_items(saida).iter().map(|&x| cpz6_d(x)).collect();
    let antes = tem.len();
    for e in national_teams_list_items(conv) {
        let c = cpz6_d(e + 4);
        if !tem.contains(&c) {
            tem.push(c);
        }
    }
    let r = o(this, saida);
    let depois: Vec<usize> = national_teams_list_items(saida).iter().map(|&x| cpz6_d(x)).collect();
    let igual = r == (tem.len() - antes) as u8 && depois == tem;
    SCOREBOARD.record(igual, || format!("collect_duty_competitions(saida {:x}) original={} plano={:x?} depois={:x?}", saida, r, tem, depois));
    r
}
