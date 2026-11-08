// ---------- %%LEVA%%a leva: as estatísticas do jogo e os recordes do clube: o jogo nas estatísticas dos jogadores, os recordes de posição na liga e as notícias dos recordes (player_stats.cpp, club_records.cpp) ----------

unsafe fn cpx12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn add_u8_at(a: usize, v: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(v);
}

unsafe fn add_u16_at(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, core::ptr::read_unaligned(a as *const u16).wrapping_add(v));
}

unsafe fn add_f64_at(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v + core::ptr::read_unaligned(a as *const f64));
}

unsafe fn player_category_record(this: usize, jogador: usize, cat: u8) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    f(this, jogador, cat as u32)
}

unsafe fn cpx12_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx12_d(0xae1050), pessoa)
}

/// A categoria das estatísticas do lado `lado` da partida (o bl da 0x7c0d70, partida com competição ≠ −2): amistoso
/// (0x5c6710 de [0xae2c74]), 5 se o mandante é seleção (0x53b900), senão 1; reservas, sub-21 e B
/// ([0x9cf684], [0x9cf798], [0x9cf794]) ou o clube do lado sem a marca +0x40 (mandante) / +0x41 (visitante): 1. Pelo
/// tipo (+0x42) da competição ([0xadadfc] pelo id +0): 4 → 5, 2 → 4, 1 → 2 se ela conta (0x4b0700), senão 3. Sem a
/// competição ou outro tipo: a caixa (linhas 0x103a, 0x105d; só com `avisar`), `erro` e 2.
unsafe fn match_stats_category(p: usize, lado: i32, avisar: bool, erro: &mut bool) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let clube = cpx12_d(p + if lado == 0 { 0x1c } else { 0x20 });
    if amistoso(cpx12_d(0xae2c74), cpx12_d(p + 0x14)) != 0 {
        return if selecao(cpx12_d(p + 0x1c)) != 0 { 5 } else { 1 };
    }
    let id = cpx12_d(cpx12_d(p + 0x14));
    if id == cpx12_d(0x9cf684) || id == cpx12_d(0x9cf798) || id == cpx12_d(0x9cf794) {
        return 1;
    }
    if clube == cpx12_d(p + 0x1c) && cpx12_b(p + 0x40) == 0 {
        return 1;
    }
    if clube == cpx12_d(p + 0x20) && cpx12_b(p + 0x41) == 0 {
        return 1;
    }
    let comp = cpx12_d(cpx12_d(0xadadfc).wrapping_add(cpx12_d(p).wrapping_mul(4)));
    if comp == 0 {
        *erro = true;
        if avisar {
            show_error_at(0xa78310, 0x103a);
        }
        return 2;
    }
    match cpx12_b(comp + 0x42) {
        4 => 5,
        2 => 4,
        1 => {
            if avisar {
                crate::comparison::branch("cpx_12 league category");
            }
            if conta(cpx12_d(p) as i32) != 0 { 2 } else { 3 } // cpx12: a categoria da liga
        }
        _ => {
            *erro = true;
            if avisar {
                show_error_at(0xa78310, 0x105d);
            }
            2
        }
    }
}

/// A linha `k` (0..20, 0x49 bytes; 11.. reservas) do lado `lado` do jogo conta: reserva que entrou (+0x32 ≠ 0xff),
/// pessoa (+0) e jogador (+4) ≠ −1 e a pessoa na tabela ([0xae23bc], 0x6e; id abaixo de [0xae2364]). Devolve
/// (linha, pessoa, dados do jogador em [0xae23c4], 0x46).
unsafe fn match_lineup_row(jogo: usize, lado: usize, k: usize) -> Option<(usize, usize, usize)> {
    let e = jogo + 0x625 * lado + 0x190 + 0x49 * k;
    if k > 10 && cpx12_b(e + 0x32) == 0xff {
        return None;
    }
    let (sid, pid) = (cpx12_d(e) as i32, cpx12_d(e + 4) as i32);
    if sid == -1 || pid == -1 {
        return None;
    }
    let pessoa = cpx12_d(0xae23bc).wrapping_add((sid as usize).wrapping_mul(0x6e));
    if pessoa != 0 && cpx12_d(pessoa) as i32 >= cpx12_d(0xae2364) as i32 {
        return None;
    }
    Some((e, pessoa, cpx12_d(0xae23c4).wrapping_add((pid as usize).wrapping_mul(0x46))))
}

/// As categorias de cada passada: a do jogo e a geral (6), que fica de fora no amistoso de clube (1) e no de seleção
/// (5).
fn stats_categories_for(cat: u8) -> [Option<u8>; 2] {
    [Some(cat), if cat == 5 || cat == 1 { None } else { Some(6) }] // cpx12: a categoria geral
}

/// { 0x7c0d70 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::record_match(partida, jogo)`: partida ou jogo nulos,
/// as caixas e 0. Competição −2 (jogo simulado): a categoria é sorteio(9) ≠ 0 ? 2 : 3 para os dois lados; senão a de
/// cada lado (match_stats_category). Em cada linha que conta (match_lineup_row): a nota (+0x35) e um jogo nas somas em double
/// do this (+0x20/+0x28 com a marca +0x4c do jogo, senão +0x10/+0x18); nos registros de 0x20 bytes das categorias
/// (0x7c3920, ou criado pela 0x7c5d80): jogos, entradas, gols/assistências e o resto da linha (+0x21..+0x2d), gols a
/// favor (+0x11f do lado) e contra (+0x744 − lado), vitória e derrota, melhor em campo (+0x3c), as palavras
/// (+0x35, +0x3e, +0x3f, +0x42, +0x28, +0x25, +0x26) e os minutos (+0x34, +0x30 ou a duração +0x4d do jogo; o
/// reserva desconta a entrada +0x32). Na categoria 2 ou com a competição contando (0x4b0700): +1 jogo e os gols no
/// registro do jogador ([this+0x30], 0x27 bytes) e, sem ficha (0x4cf6d0), +1 no +0x3b do contrato (0x4cf5a0). Por
/// fim a forma (+0x24..+0x26) anda meio byte e recebe a nota. Devolve 1.
unsafe extern "thiscall" fn player_stats_record_match(this: usize, p: usize, jogo: usize) -> u32 {
    if p == 0 || jogo == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, p, jogo); // as caixas
    }
    let criar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c5d80usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let simulado = cpx12_d(p) as i32 == -2;
    let mut cat: u8 = 0;
    if simulado {
        cat = if sorteio(9) != 0 { 2 } else { 3 };
    }
    let mut contou = false;
    for lado in 0..2usize {
        if !simulado {
            let mut erro = false;
            cat = match_stats_category(p, lado as i32, true, &mut erro);
        }
        let gf = jogo + 0x625 * lado + 0x11f;
        let gc = jogo + 0x744 - 0x625 * lado;
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = match_lineup_row(jogo, lado, k) else { continue };
            let nota = cpx12_b(e + 0x35);
            if cpx12_b(jogo + 0x4c) != 0 {
                add_f64_at(this + 0x20, nota as i32 as f64);
                add_f64_at(this + 0x28, 1.0);
            } else {
                add_f64_at(this + 0x10, nota as i32 as f64);
                add_f64_at(this + 0x18, 1.0);
            }
            if !contou {
                contou = true;
                crate::comparison::branch("cpx_12 match recorded");
            }
            for c in stats_categories_for(cat).into_iter().flatten() {
                let mut r = player_category_record(this, dados, c);
                if r == 0 {
                    r = criar(this, dados, c as u32);
                }
                add_u8_at(r, 1); // cpx12: o jogo
                if k > 10 {
                    add_u8_at(r + 1, 1);
                }
                add_u8_at(r + 2, cpx12_b(e + 0x21));
                add_u8_at(r + 3, cpx12_b(e + 0x22));
                add_u8_at(r + 4, cpx12_b(e + 0x23));
                add_u8_at(r + 5, cpx12_b(e + 0x24));
                add_u8_at(r + 6, cpx12_b(e + 0x27));
                let (f, a) = (cpx12_b(gf), cpx12_b(gc));
                add_u8_at(r + 7, f);
                add_u8_at(r + 8, a);
                if f > a {
                    add_u8_at(r + 9, 1);
                }
                if f < a {
                    add_u8_at(r + 0xa, 1);
                }
                add_u8_at(r + 0xb, cpx12_b(e + 0x2c));
                add_u8_at(r + 0xc, cpx12_b(e + 0x2d));
                if cpx12_d(e + 4) == cpx12_d(jogo + 0x3c) {
                    add_u8_at(r + 0xd, 1);
                }
                add_u16_at(r + 0xe, nota as u16); // cpx12: a nota
                add_u16_at(r + 0x10, cpx12_b(e + 0x3e) as u16);
                add_u16_at(r + 0x12, cpx12_b(e + 0x3f) as u16);
                add_u16_at(r + 0x14, cpx12_b(e + 0x42) as u16);
                add_u16_at(r + 0x16, cpx12_b(e + 0x28) as u16);
                add_u16_at(r + 0x18, cpx12_b(e + 0x25) as u16);
                add_u16_at(r + 0x1a, cpx12_b(e + 0x26) as u16);
                let saiu = |e: usize| {
                    let (s34, s30) = (cpx12_b(e + 0x34) as i8, cpx12_b(e + 0x30) as i8);
                    if s34 > -1 {
                        s34 as i32
                    } else if s30 > -1 {
                        s30 as i32
                    } else {
                        cpx12_w(jogo + 0x4d) as i32
                    }
                };
                let min = r + 0x1c;
                if k < 11 {
                    core::ptr::write_unaligned(min as *mut i32, (cpx12_d(min) as i32).wrapping_add(saiu(e)));
                } else if cpx12_b(e + 0x32) != 0xff {
                    let v = saiu(e).wrapping_sub(cpx12_b(e + 0x32) as i8 as i32);
                    core::ptr::write_unaligned(min as *mut i32, (cpx12_d(min) as i32).wrapping_add(v));
                }
            }
            let reg = cpx12_d(this + 0x30).wrapping_add((cpx12_d(e + 4)).wrapping_mul(0x27));
            if cat == 2 || (cpx12_d(p) as i32 > -1 && conta(cpx12_d(p) as i32) != 0) {
                add_u16_at(reg + 0x20, 1);
                add_u16_at(reg + 0x22, cpx12_b(e + 0x21) as u16);
                if ficha(cpx12_d(0xae1050), pessoa) == 0 && cpx12_contrato(pessoa) != 0 {
                    add_u16_at(cpx12_contrato(pessoa) + 0x3b, 1);
                }
            }
            let (f24, f25, f26) = (cpx12_b(reg + 0x24), cpx12_b(reg + 0x25), cpx12_b(reg + 0x26));
            *((reg + 0x24) as *mut u8) = (f25 << 4) | (f24 >> 4);
            *((reg + 0x25) as *mut u8) = (f26 << 4) | (f25 >> 4);
            *((reg + 0x26) as *mut u8) = ((nota ^ f26) & 0xf) ^ f26;
        }
    }
    1
}

/// Comparador lado a lado: as somas em double (+0x10..+0x30), os registros das categorias de cada linha que conta, o
/// trecho +0x20..+0x27 do registro do jogador e o +0x3b do contrato; no jogo simulado (competição −2), também o estado
/// do sorteio (e as duas categorias possíveis), com o sorteio da cópia fora da contagem. Partida ou jogo nulos, a caixa
/// de uma competição sem tipo conhecido ou registro de categoria a criar (0x7c5d80 cresce o vetor): só a original.
unsafe extern "thiscall" fn compare_player_stats_record_match(this: usize, p: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    if p == 0 || jogo == 0 {
        return o(this, p, jogo); // ponytail: as caixas, só a original
    }
    let simulado = cpx12_d(p) as i32 == -2;
    let mut regioes = vec![(this + 0x10, 0x20)];
    for lado in 0..2usize {
        let cats: Vec<u8> = if simulado {
            vec![2, 3, 6]
        } else {
            let mut erro = false;
            let c = match_stats_category(p, lado as i32, false, &mut erro);
            if erro {
                return o(this, p, jogo); // ponytail: a caixa, só a original
            }
            stats_categories_for(c).into_iter().flatten().collect()
        };
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = match_lineup_row(jogo, lado, k) else { continue };
            for &c in &cats {
                let r = player_category_record(this, dados, c);
                if r == 0 {
                    return o(this, p, jogo); // ponytail: o registro seria criado, só a original
                }
                regioes.push((r, 0x20));
            }
            regioes.push((cpx12_d(this + 0x30).wrapping_add(cpx12_d(e + 4).wrapping_mul(0x27)) + 0x20, 7));
            let ct = cpx12_contrato(pessoa);
            if ct != 0 {
                regioes.push((ct + 0x3b, 2));
            }
        }
    }
    if simulado {
        regioes.extend(crate::rng::STATE_REGIONS);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this, p, jogo), || {
        crate::rng::without_counting(|| player_stats_record_match(this, p, jogo))
    });
    SCOREBOARD.record(igual && ro == rr, || format!("player_stats_record_match({:x}, {:x}, {:x}) original={} {:x?} rust={} {:x?}", this, p, jogo, ro, mo, rr, mr));
    ro
}

/// O ensaio do comparador da 0x447430: a cópia roda sem ordenar as tabelas (a original, que roda antes, já ordenou).
static mut POSITION_RECORDS_DRY_RUN: bool = false;

unsafe fn club_season_record(this: usize, clube: usize, criar: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    f(this, clube, criar)
}

unsafe fn club_all_time_record(this: usize, clube: usize, criar: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    f(this, clube, criar)
}

/// A reputação (+0x69, palavra com sinal) da competição `id` (registros de 0x6b em [0xae23d0]).
unsafe fn competition_reputation(id: i32) -> i16 {
    cpx12_w(cpx12_d(0xae23d0).wrapping_add((id as usize).wrapping_mul(0x6b)) + 0x69)
}

/// O recorde de sempre mudou: clube humano (0x600a50) ou seleção (0x53b900) com 5 temporadas ou mais (+0x68 do
/// resumo) e o recorde (id, tipo) ainda fora da fila (+0x24, 6 bytes, conta +0x2c): entra na fila (0x452cb0).
unsafe fn club_records_queue_record_news(this: usize, clube: usize, resumo: usize, tipo: u8) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let guardar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x452cb0usize);
    if humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 {
        return;
    }
    if cpx12_w(resumo + 0x68) < 5 {
        return;
    }
    let n = cpx12_d(this + 0x2c) as i32;
    let mut e = cpx12_d(this + 0x24);
    for _ in 0..n.max(0) {
        if cpx12_d(e) == cpx12_d(clube) && cpx12_b(e + 4) == tipo {
            return;
        }
        e += 6;
    }
    guardar(this, cpx12_d(clube), tipo as u32);
}

/// As ligas da cadeia da 0x447430: a liga e, enquanto houver, a fase seguinte (0x48d200 da primeira, índice +0x44 + 1
/// até +0x30) do tipo 2 (+0x43) com as mesmas +0x3e linhas.
unsafe fn league_stage_chain(liga0: usize) -> Vec<usize> {
    let fase: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x48d200usize);
    let mut v = vec![liga0];
    let mut liga = liga0;
    loop {
        let k = cpx12_b(liga + 0x44) as i8 as i32;
        if k >= cpx12_d(liga0 + 0x30) as i32 {
            return v;
        }
        let f = fase(liga0, k + 1);
        if cpx12_b(f + 0x43) != 2 || cpx12_w(f + 0x3e) != cpx12_w(liga + 0x3e) {
            return v;
        }
        v.push(f);
        liga = f;
    }
}

/// { 0x447430 } (thiscall, ret 8, club_records.cpp): `Club_Records::update_league_position_records(liga, sempre)`:
/// liga nula ou com +0x44 ≠ 0xff, as caixas e 0. Em cada liga da cadeia (league_stage_chain), ordena a tabela (0x6827d0) e,
/// em cada linha com jogos (+0x11) e clube, pelos registros da temporada (0x4528d0) e de sempre (0x452710, criando):
/// melhor posição (tipo 1, 0x449590) e pior (tipo 2): sem registro (−1), competição (o id da primeira liga) de
/// reputação maior (melhor) ou menor (pior), ou a mesma com posição melhor ou pior: grava a posição (+0/+7), a
/// competição (+1/+8) e o ano (+5/+0xc; 0 na temporada, 0x669950 em sempre). Em sempre (com `sempre`; id acima de
/// [0xae2378] também é "sem"), a fila da notícia (club_records_queue_record_news). Devolve 1.
unsafe extern "thiscall" fn update_league_position_records(this: usize, liga0: usize, sempre: u32) -> u32 {
    if liga0 == 0 || cpx12_b(liga0 + 0x44) != 0xff {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, liga0, sempre); // as caixas
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let aplica: unsafe extern "stdcall" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x449590usize);
    let ano_da_temporada: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x669950usize);
    let comp = cpx12_d(cpx12_d(liga0 + 4)) as i32;
    for liga in league_stage_chain(liga0) {
        if !POSITION_RECORDS_DRY_RUN {
            ordenar(liga, 0);
        }
        let mut i = 0i32;
        while i < cpx12_w(liga + 0x3e) as i32 {
            let linha = cpx12_d(liga + 0xb1).wrapping_add(0x3b * i as usize);
            let clube = cpx12_d(linha);
            i += 1;
            if cpx12_b(linha + 0x11) == 0 || clube == 0 {
                continue;
            }
            let t = club_season_record(this, clube, 1);
            let s = club_all_time_record(this, clube, 1);
            let resumo = cpx12_d(this + 0x1c).wrapping_add(cpx12_d(clube).wrapping_mul(0x6a));
            let ano = ano_da_temporada(clube) as u16;
            let pos = cpx12_b(linha + 4);
            let fora = |v: i32| v < 0 || v > cpx12_d(0xae2378) as i32;
            if aplica(1, clube, 1) != 0 {
                let v = cpx12_d(t + 1) as i32;
                if v < 0 || competition_reputation(comp) > competition_reputation(v) || (comp == v && pos < cpx12_b(t)) {
                    *(t as *mut u8) = pos;
                    core::ptr::write_unaligned((t + 1) as *mut i32, comp);
                    core::ptr::write_unaligned((t + 5) as *mut u16, 0);
                }
                let v = cpx12_d(s + 1) as i32;
                if sempre != 0 && (fora(v) || competition_reputation(comp) > competition_reputation(v) || (comp == v && pos < cpx12_b(s))) {
                    crate::comparison::branch("cpx_12 best position ever");
                    *(s as *mut u8) = pos;
                    core::ptr::write_unaligned((s + 1) as *mut i32, comp);
                    core::ptr::write_unaligned((s + 5) as *mut u16, ano);
                    club_records_queue_record_news(this, clube, resumo, 1);
                }
            }
            if aplica(2, clube, 1) != 0 {
                let v = cpx12_d(t + 8) as i32;
                if v < 0 || competition_reputation(comp) < competition_reputation(v) || (comp == v && pos > cpx12_b(t + 7)) {
                    *((t + 7) as *mut u8) = pos;
                    core::ptr::write_unaligned((t + 8) as *mut i32, comp);
                    core::ptr::write_unaligned((t + 0xc) as *mut u16, 0);
                }
                let v = cpx12_d(s + 8) as i32;
                if sempre != 0 && (fora(v) || competition_reputation(comp) < competition_reputation(v) || (comp == v && pos > cpx12_b(s + 7))) {
                    *((s + 7) as *mut u8) = pos;
                    core::ptr::write_unaligned((s + 8) as *mut i32, comp);
                    core::ptr::write_unaligned((s + 0xc) as *mut u16, ano);
                    club_records_queue_record_news(this, clube, resumo, 2);
                }
            }
        }
    }
    1
}

/// Comparador: a original roda antes (ordena as tabelas, que a 0x6827d0 confere por conta própria) e a cópia depois,
/// em ensaio (sem ordenar), a partir dos mesmos recordes: os 14 primeiros bytes dos registros da temporada e de sempre
/// de cada clube da cadeia, o cabeçalho da fila (+0x24..+0x30) e a fila; e a devolução. Liga nula ou com +0x44 ≠ 0xff,
/// registro a criar (0x4528d0/0x452710 alocam) ou fila que pode crescer: só a original.
unsafe extern "thiscall" fn compare_update_league_position_records(this: usize, liga0: usize, sempre: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    if liga0 == 0 || cpx12_b(liga0 + 0x44) != 0xff {
        return o(this, liga0, sempre); // ponytail: as caixas, só a original
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let mut regioes = vec![(this + 0x24, 0xc)];
    let mut podem_entrar = 0i32;
    for liga in league_stage_chain(liga0) {
        for i in 0..cpx12_w(liga + 0x3e).max(0) as usize {
            let linha = cpx12_d(liga + 0xb1) + 0x3b * i;
            let clube = cpx12_d(linha);
            if cpx12_b(linha + 0x11) == 0 || clube == 0 {
                continue;
            }
            let (t, s) = (club_season_record(this, clube, 0), club_all_time_record(this, clube, 0));
            if t == 0 || s == 0 {
                return o(this, liga0, sempre); // ponytail: o registro seria criado, só a original
            }
            regioes.push((t, 0xe));
            regioes.push((s, 0xe));
            if humano(clube, 0, 1, 0, 0) != 0 || selecao(clube) != 0 {
                podem_entrar += 2;
            }
        }
    }
    let fila = cpx12_d(this + 0x24);
    if (cpx12_d(this + 0x2c) as i32).wrapping_add(podem_entrar) > cpx12_d(this + 0x28) as i32 || (fila == 0 && podem_entrar > 0) {
        return o(this, liga0, sempre); // ponytail: a fila pode crescer (realoca), só a original
    }
    if fila != 0 {
        regioes.push((fila, 6 * cpx12_d(this + 0x28)));
    }
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    let ro = o(this, liga0, sempre);
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    POSITION_RECORDS_DRY_RUN = true;
    let rr = crate::rng::pause_trace(|| update_league_position_records(this, liga0, sempre));
    POSITION_RECORDS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    mo.write_back();
    SCOREBOARD.record(ro == rr && mo == mr, || format!("update_league_position_records({:x}, {:x}, {}) original={} {:x?} rust={} {:x?}", this, liga0, sempre, ro, mo, rr, mr));
    ro
}

/// O ensaio do comparador da 0x448e30: a cópia roda sem preencher nem entregar as notícias e sem soltar a fila.
static mut RECORD_NEWS_DRY_RUN: bool = false;

/// { 0x448e30 } (thiscall, ret 0, club_records.cpp, quadro SEH sem destrutor): `Club_Records::send_record_news()`: sem
/// fila (+0x24), 1. Cada recorde da fila (6 bytes: id do clube, tipo, período em +5) de clube humano (0x7875e0) ou
/// seleção (0x53b900): a notícia 0x3e8 (local) preenchida pela 0x453120 com o registro da temporada (período 2,
/// 0x4528d0) ou de sempre (0x452710), sem criar; entregue à nação (0x7863f0) da seleção ou ao técnico do clube
/// (0x7861e0). Fila com capacidade (+0x28) de 100 ou mais: solta (0x9452ca) e zera +0x24/+0x28. A conta (+0x2c) volta
/// a 0. Devolve 1.
unsafe extern "thiscall" fn send_record_news(this: usize) -> u32 {
    if cpx12_d(this + 0x24) == 0 {
        return 1;
    }
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let preencher: unsafe extern "C" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x453120usize);
    let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut i = 0i32;
    while i < cpx12_d(this + 0x2c) as i32 {
        let e = cpx12_d(this + 0x24) + 6 * i as usize;
        i += 1;
        let id = cpx12_d(e);
        let tipo = cpx12_b(e + 4);
        let clube = cpx12_d(0xae23b4).wrapping_add(id).wrapping_add(id.wrapping_mul(0x244));
        if humano(clube) == 0 && selecao(clube) == 0 {
            continue;
        }
        crate::comparison::branch("cpx_12 record news");
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construir(n, 0x3e8, 0);
        let reg = if cpx12_b(e + 5) == 2 { club_season_record(this, clube, 0) } else { club_all_time_record(this, clube, 0) };
        if RECORD_NEWS_DRY_RUN {
            continue;
        }
        // ponytail: a original empurra o dword da local com o tipo no byte baixo (o resto é sobra); a 0x453120 lê o byte
        preencher(id, reg, tipo as u32, n);
        if selecao(clube) != 0 {
            a_nacao(cpx12_d(0xb7431c), n, cpx12_d(clube + 0x53));
        } else {
            ao_clube(cpx12_d(0xb7431c), n, clube);
        }
    }
    if cpx12_d(this + 0x28) as i32 >= 100 {
        if !RECORD_NEWS_DRY_RUN {
            liberar(cpx12_d(this + 0x24));
        }
        core::ptr::write_unaligned((this + 0x24) as *mut usize, 0);
        core::ptr::write_unaligned((this + 0x28) as *mut usize, 0);
    }
    core::ptr::write_unaligned((this + 0x2c) as *mut usize, 0);
    1
}

/// Comparador: a cópia em ensaio (sem preencher nem entregar, sem soltar) roda antes, enquanto a fila está viva, e a
/// original depois a partir do mesmo cabeçalho (+0x24..+0x30): o cabeçalho e a devolução.
unsafe extern "thiscall" fn compare_send_record_news(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let regioes = [(this + 0x24, 0xc)];
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    RECORD_NEWS_DRY_RUN = true;
    let rr = crate::rng::pause_trace(|| send_record_news(this));
    RECORD_NEWS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    let ro = o(this);
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    SCOREBOARD.record(ro == rr && mo == mr, || format!("send_record_news({:x}) original={} {:x?} rust={} {:x?}", this, ro, mo, rr, mr));
    ro
}
