use super::*;

// ---------- 260a leva: as estatísticas do jogo e os recordes do clube: o jogo nas estatísticas dos jogadores, os recordes de posição na liga e as notícias dos recordes (player_stats.cpp, club_records.cpp) ----------




pub(crate) unsafe fn cpx12_mais_b(a: usize, v: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(v);
}

pub(crate) unsafe fn cpx12_mais_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, core::ptr::read_unaligned(a as *const u16).wrapping_add(v));
}

pub(crate) unsafe fn cpx12_mais_f(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v + core::ptr::read_unaligned(a as *const f64));
}

pub(crate) unsafe fn cpx12_registro(this: usize, jogador: usize, cat: u8) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    f(this, jogador, cat as u32)
}


/// A categoria das estatísticas do lado `lado` da partida (o bl da 0x7c0d70, partida com competição ≠ −2): amistoso
/// (0x5c6710 de [0xae2c74]), 5 se o mandante é seleção (0x53b900), senão 1; reservas, sub-21 e B
/// ([0x9cf684], [0x9cf798], [0x9cf794]) ou o clube do lado sem a marca +0x40 (mandante) / +0x41 (visitante): 1. Pelo
/// tipo (+0x42) da competição ([0xadadfc] pelo id +0): 4 → 5, 2 → 4, 1 → 2 se ela conta (0x4b0700), senão 3. Sem a
/// competição ou outro tipo: a caixa (linhas 0x103a, 0x105d; só com `avisar`), `erro` e 2.
pub(crate) unsafe fn cpx12_categoria(p: usize, lado: i32, avisar: bool, erro: &mut bool) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let clube = read_usize(p + if lado == 0 { 0x1c } else { 0x20 });
    if amistoso(read_usize(G_FRIENDLY), read_usize(p + 0x14)) != 0 {
        return if selecao(read_usize(p + 0x1c)) != 0 { 5 } else { 1 };
    }
    let id = read_usize(read_usize(p + 0x14));
    if id == read_usize(KEY_CLUB_COMP_RESERVE) || id == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) || id == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) {
        return 1;
    }
    if clube == read_usize(p + 0x1c) && read_u8(p + 0x40) == 0 {
        return 1;
    }
    if clube == read_usize(p + 0x20) && read_u8(p + 0x41) == 0 {
        return 1;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(p).wrapping_mul(4)));
    if comp == 0 {
        *erro = true;
        if avisar {
            aviso_em(0xa78310, 0x103a);
        }
        return 2;
    }
    match read_u8(comp + 0x42) {
        4 => 5,
        2 => 4,
        1 => {
            if avisar {
                crate::comparacao::ramo("cpx_12 categoria da liga");
            }
            if conta(read_usize(p) as i32) != 0 { 2 } else { 3 } // cpx12: a categoria da liga
        }
        _ => {
            *erro = true;
            if avisar {
                aviso_em(0xa78310, 0x105d);
            }
            2
        }
    }
}

/// A linha `k` (0..20, 0x49 bytes; 11.. reservas) do lado `lado` do jogo conta: reserva que entrou (+0x32 ≠ 0xff),
/// pessoa (+0) e jogador (+4) ≠ −1 e a pessoa na tabela ([0xae23bc], 0x6e; id abaixo de [0xae2364]). Devolve
/// (linha, pessoa, dados do jogador em [0xae23c4], 0x46).
pub(crate) unsafe fn cpx12_linha(jogo: usize, lado: usize, k: usize) -> Option<(usize, usize, usize)> {
    let e = jogo + 0x625 * lado + 0x190 + 0x49 * k;
    if k > 10 && read_u8(e + 0x32) == 0xff {
        return None;
    }
    let (sid, pid) = (read_usize(e) as i32, read_usize(e + 4) as i32);
    if sid == -1 || pid == -1 {
        return None;
    }
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((sid as usize).wrapping_mul(0x6e));
    if pessoa != 0 && read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return None;
    }
    Some((e, pessoa, read_usize(G_PLAYER_DATA_TABLE).wrapping_add((pid as usize).wrapping_mul(0x46))))
}

/// As categorias de cada passada: a do jogo e a geral (6), que fica de fora no amistoso de clube (1) e no de seleção
/// (5).
pub(crate) fn cpx12_categorias(cat: u8) -> [Option<u8>; 2] {
    [Some(cat), if cat == 5 || cat == 1 { None } else { Some(6) }] // cpx12: a categoria geral
}



/// O ensaio do comparador da 0x447430: a cópia roda sem ordenar as tabelas (a original, que roda antes, já ordenou).
pub(crate) static mut CPX12_ENSAIO: bool = false;

pub(crate) unsafe fn cpx12_temporada(this: usize, clube: usize, criar: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    f(this, clube, criar)
}

pub(crate) unsafe fn cpx12_de_sempre(this: usize, clube: usize, criar: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    f(this, clube, criar)
}

/// A reputação (+0x69, palavra com sinal) da competição `id` (registros de 0x6b em [0xae23d0]).
pub(crate) unsafe fn cpx12_reputacao(id: i32) -> i16 {
    read_i16(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)) + 0x69)
}

/// O recorde de sempre mudou: clube humano (0x600a50) ou seleção (0x53b900) com 5 temporadas ou mais (+0x68 do
/// resumo) e o recorde (id, tipo) ainda fora da fila (+0x24, 6 bytes, conta +0x2c): entra na fila (0x452cb0).
pub(crate) unsafe fn cpx12_fila_do_recorde(this: usize, clube: usize, resumo: usize, tipo: u8) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let guardar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_QUEUE_RECORD_NEWS);
    if humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 {
        return;
    }
    if read_i16(resumo + 0x68) < 5 {
        return;
    }
    let n = read_usize(this + 0x2c) as i32;
    let mut e = read_usize(this + 0x24);
    for _ in 0..n.max(0) {
        if read_usize(e) == read_usize(clube) && read_u8(e + 4) == tipo {
            return;
        }
        e += 6;
    }
    guardar(this, read_usize(clube), tipo as u32);
}

/// As ligas da cadeia da 0x447430: a liga e, enquanto houver, a fase seguinte (0x48d200 da primeira, índice +0x44 + 1
/// até +0x30) do tipo 2 (+0x43) com as mesmas +0x3e linhas.
pub(crate) unsafe fn cpx12_cadeia(liga0: usize) -> Vec<usize> {
    let fase: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_COMP_GET_STAGE);
    let mut v = vec![liga0];
    let mut liga = liga0;
    loop {
        let k = read_u8(liga + 0x44) as i8 as i32;
        if k >= read_usize(liga0 + 0x30) as i32 {
            return v;
        }
        let f = fase(liga0, k + 1);
        if read_u8(f + 0x43) != 2 || read_i16(f + 0x3e) != read_i16(liga + 0x3e) {
            return v;
        }
        v.push(f);
        liga = f;
    }
}

/// sorts the table of each league stage and stores each club's best and worst position for the season and, with the
/// second argument, all time (queueing the news item for a human club or national side)
/// Original: `Club_Records::update_league_position_records` (0x447430, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x447430 } (thiscall, ret 8, club_records.cpp): `Club_Records::update_league_position_records(liga, sempre)`:
/// liga nula ou com +0x44 ≠ 0xff, as caixas e 0. Em cada liga da cadeia (cpx12_cadeia), ordena a tabela (0x6827d0) e,
/// em cada linha com jogos (+0x11) e clube, pelos registros da temporada (0x4528d0) e de sempre (0x452710, criando):
/// melhor posição (tipo 1, 0x449590) e pior (tipo 2): sem registro (−1), competição (o id da primeira liga) de
/// reputação maior (melhor) ou menor (pior), ou a mesma com posição melhor ou pior: grava a posição (+0/+7), a
/// competição (+1/+8) e o ano (+5/+0xc; 0 na temporada, 0x669950 em sempre). Em sempre (com `sempre`; id acima de
/// [0xae2378] também é "sem"), a fila da notícia (cpx12_fila_do_recorde). Devolve 1.
pub(crate) unsafe extern "thiscall" fn recordes_de_posicao_na_liga(this: usize, liga0: usize, sempre: u32) -> u32 {
    if liga0 == 0 || read_u8(liga0 + 0x44) != 0xff {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1339)(this, liga0, sempre); // as caixas
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let aplica: unsafe extern "stdcall" fn(u32, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_RECORD_TYPE_APPLIES);
    let ano_da_temporada: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_KEY_NATION_GET_CLUB_SEASON_YEAR);
    let comp = read_usize(read_usize(liga0 + 4)) as i32;
    for liga in cpx12_cadeia(liga0) {
        if !CPX12_ENSAIO {
            ordenar(liga, 0);
        }
        let mut i = 0i32;
        while i < read_i16(liga + 0x3e) as i32 {
            let linha = read_usize(liga + 0xb1).wrapping_add(0x3b * i as usize);
            let clube = read_usize(linha);
            i += 1;
            if read_u8(linha + 0x11) == 0 || clube == 0 {
                continue;
            }
            let t = cpx12_temporada(this, clube, 1);
            let s = cpx12_de_sempre(this, clube, 1);
            let resumo = read_usize(this + 0x1c).wrapping_add(read_usize(clube).wrapping_mul(0x6a));
            let ano = ano_da_temporada(clube) as u16;
            let pos = read_u8(linha + 4);
            let fora = |v: i32| v < 0 || v > read_usize(G_CLUB_COMP_COUNT) as i32;
            if aplica(1, clube, 1) != 0 {
                let v = read_usize(t + 1) as i32;
                if v < 0 || cpx12_reputacao(comp) > cpx12_reputacao(v) || (comp == v && pos < read_u8(t)) {
                    *(t as *mut u8) = pos;
                    core::ptr::write_unaligned((t + 1) as *mut i32, comp);
                    core::ptr::write_unaligned((t + 5) as *mut u16, 0);
                }
                let v = read_usize(s + 1) as i32;
                if sempre != 0 && (fora(v) || cpx12_reputacao(comp) > cpx12_reputacao(v) || (comp == v && pos < read_u8(s))) {
                    crate::comparacao::ramo("cpx_12 melhor posicao de sempre");
                    *(s as *mut u8) = pos;
                    core::ptr::write_unaligned((s + 1) as *mut i32, comp);
                    core::ptr::write_unaligned((s + 5) as *mut u16, ano);
                    cpx12_fila_do_recorde(this, clube, resumo, 1);
                }
            }
            if aplica(2, clube, 1) != 0 {
                let v = read_usize(t + 8) as i32;
                if v < 0 || cpx12_reputacao(comp) < cpx12_reputacao(v) || (comp == v && pos > read_u8(t + 7)) {
                    *((t + 7) as *mut u8) = pos;
                    core::ptr::write_unaligned((t + 8) as *mut i32, comp);
                    core::ptr::write_unaligned((t + 0xc) as *mut u16, 0);
                }
                let v = read_usize(s + 8) as i32;
                if sempre != 0 && (fora(v) || cpx12_reputacao(comp) < cpx12_reputacao(v) || (comp == v && pos > read_u8(s + 7))) {
                    *((s + 7) as *mut u8) = pos;
                    core::ptr::write_unaligned((s + 8) as *mut i32, comp);
                    core::ptr::write_unaligned((s + 0xc) as *mut u16, ano);
                    cpx12_fila_do_recorde(this, clube, resumo, 2);
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
pub(crate) unsafe extern "thiscall" fn comparar_recordes_de_posicao_na_liga(this: usize, liga0: usize, sempre: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1339);
    if liga0 == 0 || read_u8(liga0 + 0x44) != 0xff {
        return o(this, liga0, sempre); // ponytail: as caixas, só a original
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let mut regioes = vec![(this + 0x24, 0xc)];
    let mut podem_entrar = 0i32;
    for liga in cpx12_cadeia(liga0) {
        for i in 0..read_i16(liga + 0x3e).max(0) as usize {
            let linha = read_usize(liga + 0xb1) + 0x3b * i;
            let clube = read_usize(linha);
            if read_u8(linha + 0x11) == 0 || clube == 0 {
                continue;
            }
            let (t, s) = (cpx12_temporada(this, clube, 0), cpx12_de_sempre(this, clube, 0));
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
    let fila = read_usize(this + 0x24);
    if (read_usize(this + 0x2c) as i32).wrapping_add(podem_entrar) > read_usize(this + 0x28) as i32 || (fila == 0 && podem_entrar > 0) {
        return o(this, liga0, sempre); // ponytail: a fila pode crescer (realoca), só a original
    }
    if fila != 0 {
        regioes.push((fila, 6 * read_usize(this + 0x28)));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let ro = o(this, liga0, sempre);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    CPX12_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(|| recordes_de_posicao_na_liga(this, liga0, sempre));
    CPX12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    mo.escrever();
    PLACAR.registrar(ro == rr && mo == mr, || format!("recordes_de_posicao_na_liga({:x}, {:x}, {}) original={} {:x?} rust={} {:x?}", this, liga0, sempre, ro, mo, rr, mr));
    ro
}

/// O ensaio do comparador da 0x448e30: a cópia roda sem preencher nem entregar as notícias e sem soltar a fila.
pub(crate) static mut CPX12_ENSAIO_FILA: bool = false;

/// delivers the news item of each queued record (human club or national side) and empties the queue (freed at capacity
/// 100 or more)
/// Original: `Club_Records::send_record_news` (0x448e30, club_records.cpp, thiscall, ret 0).
///
/// [pt] { 0x448e30 } (thiscall, ret 0, club_records.cpp, quadro SEH sem destrutor): `Club_Records::send_record_news()`: sem
/// fila (+0x24), 1. Cada recorde da fila (6 bytes: id do clube, tipo, período em +5) de clube humano (0x7875e0) ou
/// seleção (0x53b900): a notícia 0x3e8 (local) preenchida pela 0x453120 com o registro da temporada (período 2,
/// 0x4528d0) ou de sempre (0x452710), sem criar; entregue à nação (0x7863f0) da seleção ou ao técnico do clube
/// (0x7861e0). Fila com capacidade (+0x28) de 100 ou mais: solta (0x9452ca) e zera +0x24/+0x28. A conta (+0x2c) volta
/// a 0. Devolve 1.
pub(crate) unsafe extern "thiscall" fn noticias_de_recordes(this: usize) -> u32 {
    if read_usize(this + 0x24) == 0 {
        return 1;
    }
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let preencher: unsafe extern "C" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_WRITE_RECORD_NEWS_FIELDS);
    let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut i = 0i32;
    while i < read_usize(this + 0x2c) as i32 {
        let e = read_usize(this + 0x24) + 6 * i as usize;
        i += 1;
        let id = read_usize(e);
        let tipo = read_u8(e + 4);
        let clube = read_usize(G_CLUB_TABLE).wrapping_add(id).wrapping_add(id.wrapping_mul(0x244));
        if humano(clube) == 0 && selecao(clube) == 0 {
            continue;
        }
        crate::comparacao::ramo("cpx_12 noticia de recorde");
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construir(n, 0x3e8, 0);
        let reg = if read_u8(e + 5) == 2 { cpx12_temporada(this, clube, 0) } else { cpx12_de_sempre(this, clube, 0) };
        if CPX12_ENSAIO_FILA {
            continue;
        }
        // ponytail: a original empurra o dword da local com o tipo no byte baixo (o resto é sobra); a 0x453120 lê o byte
        preencher(id, reg, tipo as u32, n);
        if selecao(clube) != 0 {
            a_nacao(read_usize(G_NEWS), n, read_usize(clube + 0x53));
        } else {
            ao_clube(read_usize(G_NEWS), n, clube);
        }
    }
    if read_usize(this + 0x28) as i32 >= 100 {
        if !CPX12_ENSAIO_FILA {
            liberar(read_usize(this + 0x24));
        }
        core::ptr::write_unaligned((this + 0x24) as *mut usize, 0);
        core::ptr::write_unaligned((this + 0x28) as *mut usize, 0);
    }
    core::ptr::write_unaligned((this + 0x2c) as *mut usize, 0);
    1
}

/// Comparador: a cópia em ensaio (sem preencher nem entregar, sem soltar) roda antes, enquanto a fila está viva, e a
/// original depois a partir do mesmo cabeçalho (+0x24..+0x30): o cabeçalho e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_noticias_de_recordes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1340);
    let regioes = [(this + 0x24, 0xc)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX12_ENSAIO_FILA = true;
    let rr = crate::aleatorio::pausar_rastro(|| noticias_de_recordes(this));
    CPX12_ENSAIO_FILA = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let ro = o(this);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(ro == rr && mo == mr, || format!("noticias_de_recordes({:x}) original={} {:x?} rust={} {:x?}", this, ro, mo, rr, mr));
    ro
}

// ---------- 252a leva: o fim da temporada e as transferências nos registros: o jogador do ano da torcida, arquivar as estatísticas da nação e os recordes de valor de transferência (club_records.cpp, player_stats.cpp) ----------








/// A nota da torcida para o jogador `p` do clube pela estatística `st` (0x452da0): média (+0xe com sinal / jogos +0)
/// + 0,025 × +2 + 1/60 × +6; o clube gosta dele (0x547790) +0,2, não gosta (0x5477d0) −2 (guardada); ele gosta do
/// clube (0x547d60) +0,2, não gosta (0x547d30) −1,5 (só na comparação).
pub(crate) unsafe fn cpp10_nota_da_torcida(p: usize, clube: usize, st: usize) -> f64 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let desafeto: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let media = read_i16(st + 0xe) as i32 as f64 / read_u8(st) as i32 as f64;
    let mut nota = media + read_u8(st + 2) as i32 as f64 * read_f64(0x967878);
    nota = nota + read_u8(st + 6) as i32 as f64 * read_f64(0x968958);
    if favorito(p, clube) != 0 {
        crate::comparacao::ramo("cpp_10 favorito do clube");
        nota = nota + read_f64(0x968918); // o clube gosta dele
    } else if desafeto(p, clube) != 0 {
        nota = nota - read_f64(0x968950);
    }
    if gosta(p, clube) != 0 {
        nota + read_f64(0x968918)
    } else if nao_gosta(p, clube) != 0 {
        nota - read_f64(0x968940)
    } else {
        nota
    }
}

/// from the club's squad, with games above the minimum (factor 0.6 falling by 0.1 down to 0.3), the best score (average
/// + 0.025 × +2 + 1/60 × +6, club and player likes); his reference at record +0x371 and 1, or 0
/// Original: `Club_Records::pick_fans_player_of_year` (0x452da0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x452da0 } (thiscall, ret 8, club_records.cpp): `Club_Records::pick_fans_player_of_year(clube, registro)`: sem
/// jogos (+0x10 com sinal do registro de 106 bytes do clube em [this+0x1c]) e reputação (+0x80) abaixo de 3000, 0.
/// Com o fator 0,6 caindo de 0,1 enquanto ninguém foi escolhido e passa de 0,2: o mínimo de jogos é +0x10 × fator
/// (_ftol); dos 50 do elenco (+0xd7) jogadores (+0x61, id abaixo de [0xae2364]) do clube (+0x39) com a estatística 6
/// (0x7c3920 sobre [0xb74350]) de jogos acima de 0 e do mínimo (16 bits com sinal), o de maior nota
/// (`cpp10_nota_da_torcida`, acima de −1000). Ninguém: 0; senão a referência dele (0x7e4920) em registro +0x371 e 1.
pub(crate) unsafe extern "thiscall" fn jogador_do_ano_da_torcida(this: usize, clube: usize, registro: usize) -> u32 {
    let jogos_do_clube = || read_u8(read_usize(this + 0x1c).wrapping_add((read_i32(clube) as usize).wrapping_mul(106)) + 0x10) as i8;
    if jogos_do_clube() == 0 && read_i16(clube + 0x80) < 0xbb8 {
        return 0;
    }
    let estatistica: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    let mut fator = 0.6f64;
    let mut melhor = -1000.0f64;
    let mut escolhido = 0usize;
    while escolhido == 0 {
        let minimo = ftol(jogos_do_clube() as i32 as f64 * fator) as i16;
        for s in 0..50usize {
            let p = read_usize(clube + 0xd7 + 4 * s);
            if p == 0 || read_i32(p) >= read_i32(0xae2364) {
                continue;
            }
            let jogador = read_usize(p + 0x61);
            if jogador == 0 || read_usize(p + 0x39) != clube {
                continue;
            }
            let st = estatistica(read_usize(G_PLAYER_STATS), jogador, 6);
            if st == 0 {
                continue;
            }
            let jogos = read_u8(st);
            if (jogos as i16) < minimo || jogos == 0 {
                continue;
            }
            let nota = cpp10_nota_da_torcida(p, clube, st);
            if nota > melhor {
                melhor = nota;
                escolhido = p;
            }
        }
        fator -= read_f64(0x967880);
        if !(fator > read_f64(0x968918)) {
            break;
        }
    }
    if escolhido == 0 {
        return 0;
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
    referencia(escolhido, registro + 0x371);
    1
}


/// O recorde de transferência já está na fila de notícias (6 bytes: id do clube, tipo) de [this+0x24], conta +0x2c?
pub(crate) unsafe fn cpp10_na_fila(this: usize, id: usize, tipo: u8) -> bool {
    let fila = read_usize(this + 0x24);
    (0..read_i32(this + 0x2c).max(0) as usize).any(|k| read_usize(fila + 6 * k) == id && read_u8(fila + 6 * k + 4) == tipo)
}

/// A transferência de recorde do clube passa de 1000 × (reputação (+0x80) + 300)?
pub(crate) unsafe fn cpp10_grande(valor: i32, clube: usize) -> bool {
    valor >= (read_i16(clube + 0x80) as i32 + 300).wrapping_mul(1000)
}

/// Copia o bloco de recorde de `o` (+`d`: valor, a referência de 0x18 bytes, o clube, a data, a marca) para `a`.
pub(crate) unsafe fn cpp10_copiar_recorde(a: usize, o: usize, d: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut rascunho = [0u8; 8];
    for k in 0..5 {
        core::ptr::write_unaligned((a + d + 4 * k) as *mut usize, read_usize(o + d + 4 * k));
    }
    copiar(a + d + 0x14, rascunho.as_mut_ptr(), o + d + 0x14);
    core::ptr::write_unaligned((a + d + 0x1c) as *mut usize, read_usize(o + d + 0x1c));
    copiar(a + d + 0x20, rascunho.as_mut_ptr(), o + d + 0x20);
    *((a + d + 0x28) as *mut u8) = read_u8(o + d + 0x28);
}

/// the buyer's paid record and the seller's received record, for the season and all time (fee, reference, other club,
/// date, loan); an all-time one above 1000 × (reputation + 300) goes to the news queue and, on the purchase, the
/// manager comes to favour the player
/// Original: `Club_Records::update_transfer_fee_records` (0x448760, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448760 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_transfer_fee_records(transferencia)`: a
/// pessoa (+0xc) e o clube comprador (+8); o contrato dela (0x4cf5a0 sobre [0xae1050], guardado em [0xae1080]) dá o
/// vendedor (+4). Os registros da temporada (0x4528d0) e de sempre (0x452710) do comprador (criados); sem vendedor,
/// 1. Valor (+0x1b) acima do recorde pago da temporada (+0x313): ele, a referência da pessoa (0x7e4920), o vendedor,
/// a data de hoje e o empréstimo (+4 ≠ 0); acima do de sempre, copiado para ele, e se passa de 1000 × (reputação +
/// 300): o recorde na fila de notícias (0x452cb0, 0x1c, se ainda não está) e o técnico (+0xbf) passa a gostar da pessoa
/// (0x547a10, forçado). O mesmo para o vendedor no recorde recebido (+0x33c; a fila com 0x1d, sem o gostar). 1.
pub(crate) unsafe extern "thiscall" fn recordes_de_valor_de_transferencia(this: usize, t: usize) -> u32 {
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(t + 0xc) as usize).wrapping_mul(0x6e));
    let comprador = cpp8_clube(read_i32(t + 8));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contrato_de(read_usize(G_CONTRACT_MANAGER), pessoa);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, contrato);
    let vendedor = if contrato != 0 { cpp8_clube(read_i32(contrato + 4)) } else { 0 };
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let noticia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_QUEUE_RECORD_NEWS);
    let mut rascunho = [0u8; 8];
    let s = temporada(this, comprador, 1);
    let a = sempre(this, comprador, 1);
    if vendedor == 0 {
        return 1;
    }
    let valor = read_i32(t + 0x1b);
    if valor != 0 && valor > read_i32(s + 0x313) {
        crate::comparacao::ramo("cpp_10 recorde pago");
        core::ptr::write_unaligned((s + 0x313) as *mut i32, valor);
        referencia(pessoa, s + 0x317);
        core::ptr::write_unaligned((s + 0x32f) as *mut usize, read_usize(vendedor));
        copiar(s + 0x333, rascunho.as_mut_ptr(), 0xae2c90);
        *((s + 0x33b) as *mut u8) = (read_usize(t + 4) != 0) as u8;
        if read_i32(t + 0x1b) > read_i32(a + 0x313) {
            cpp10_copiar_recorde(a, s, 0x313);
            if cpp10_grande(read_i32(a + 0x313), comprador) {
                crate::comparacao::ramo("cpp_10 transferencia grande");
                if !cpp10_na_fila(this, read_usize(comprador), 0x1c) {
                    noticia(this, read_usize(comprador) as u32, 0x1c);
                }
                let gostar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_STAFF);
                gostar(read_usize(comprador + 0xbf), pessoa, 1); // forçado
            }
        }
    }
    let s = temporada(this, vendedor, 1);
    let a = sempre(this, vendedor, 1);
    let valor = read_i32(t + 0x1b);
    if valor == 0 || valor <= read_i32(s + 0x33c) {
        return 1;
    }
    core::ptr::write_unaligned((s + 0x33c) as *mut i32, valor);
    referencia(pessoa, s + 0x340);
    core::ptr::write_unaligned((s + 0x358) as *mut usize, read_usize(comprador));
    copiar(s + 0x35c, rascunho.as_mut_ptr(), 0xae2c90);
    *((s + 0x364) as *mut u8) = (read_usize(t + 4) != 0) as u8;
    if read_i32(t + 0x1b) > read_i32(a + 0x33c) {
        cpp10_copiar_recorde(a, s, 0x33c);
        if cpp10_grande(read_i32(a + 0x33c), vendedor) && !cpp10_na_fila(this, read_usize(vendedor), 0x1d) {
            noticia(this, read_usize(vendedor) as u32, 0x1d);
        }
    }
    1
}

/// Comparador: lado a lado pela referência do escolhido (registro +0x371, 0x18 bytes) e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_jogador_do_ano_da_torcida(this: usize, clube: usize, registro: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1033);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(registro + 0x371, 0x18)], || o(this, clube, registro), || jogador_do_ano_da_torcida(this, clube, registro));
    PLACAR.registrar(igual && r == n, || format!("jogador_do_ano_da_torcida(clube {:x}) original={} {:x?} rust={} {:x?}", read_usize(clube), r, mo, n, mr));
    r
}


/// Comparador: a original põe na fila de notícias (realoca) e o gostar sorteia: o plano antes (os quatro registros
/// criados na mesma ordem da original, e os recordes pago e recebido esperados); a original roda; pega os registros
/// de novo (o vetor pode ter sido realocado) e confere os valores e o clube de cada recorde e o [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_recordes_de_valor_de_transferencia(this: usize, t: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1035);
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(t + 0xc) as usize).wrapping_mul(0x6e));
    let comprador = cpp8_clube(read_i32(t + 8));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contrato_de(read_usize(G_CONTRACT_MANAGER), pessoa);
    let vendedor = if contrato != 0 { cpp8_clube(read_i32(contrato + 4)) } else { 0 };
    if vendedor == 0 {
        let r = o(this, t);
        PLACAR.registrar(r == 1 && read_usize(G_CONTRACT_LOOKUP_RESULT) == 0, || format!("recordes_de_valor_de_transferencia sem vendedor: {} [0xae1080]={:x}", r, read_usize(G_CONTRACT_LOOKUP_RESULT)));
        return r;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let registros = || [temporada(this, comprador, 1), sempre(this, comprador, 1), temporada(this, vendedor, 1), sempre(this, vendedor, 1)];
    let valor = read_i32(t + 0x1b);
    let rg = registros();
    // (valor, clube) de cada recorde: pago da temporada e de sempre do comprador, recebido do vendedor
    let mut plano = [(read_i32(rg[0] + 0x313), read_usize(rg[0] + 0x32f)), (read_i32(rg[1] + 0x313), read_usize(rg[1] + 0x32f)),
                     (read_i32(rg[2] + 0x33c), read_usize(rg[2] + 0x358)), (read_i32(rg[3] + 0x33c), read_usize(rg[3] + 0x358))];
    let mut fila = read_i32(this + 0x2c);
    if valor != 0 {
        if valor > plano[0].0 {
            plano[0] = (valor, read_usize(vendedor));
            if valor > plano[1].0 {
                plano[1] = plano[0];
                if cpp10_grande(valor, comprador) && !cpp10_na_fila(this, read_usize(comprador), 0x1c) {
                    fila += 1;
                }
            }
        }
        if valor > plano[2].0 {
            plano[2] = (valor, read_usize(comprador));
            if valor > plano[3].0 {
                plano[3] = plano[2];
                if cpp10_grande(valor, vendedor) && !cpp10_na_fila(this, read_usize(vendedor), 0x1d) {
                    fila += 1;
                }
            }
        }
    }
    let r = o(this, t);
    let rg = registros();
    let agora = [(read_i32(rg[0] + 0x313), read_usize(rg[0] + 0x32f)), (read_i32(rg[1] + 0x313), read_usize(rg[1] + 0x32f)),
                 (read_i32(rg[2] + 0x33c), read_usize(rg[2] + 0x358)), (read_i32(rg[3] + 0x33c), read_usize(rg[3] + 0x358))];
    let fila_agora = read_i32(this + 0x2c);
    PLACAR.registrar(r == 1 && agora == plano && fila_agora == fila && read_usize(G_CONTRACT_LOOKUP_RESULT) == contrato, || {
        format!("recordes_de_valor_de_transferencia(valor {}) plano {:x?} fila {} x {:x?} fila {}", valor, plano, fila, agora, fila_agora)
    });
    r
}

// ---------- 249a leva: os recordes do clube: somar o valor da transferência aos totais de compra e venda, os recordes de jogos e gols na carreira pelo clube e o recorde de nota da temporada (club_records.cpp) ----------





/// A notícia de recorde (id do clube, tipo) já está no vetor de 6 bytes [this+0x24] (conta +0x2c)?
pub(crate) unsafe fn cpm6_noticia_na_fila(this: usize, id: usize, tipo: u8) -> bool {
    let n = read_usize(this + 0x2c) as i32;
    let v = read_usize(this + 0x24);
    (0..n.max(0) as usize).any(|k| read_usize(v + 6 * k) == id && *((v + 6 * k + 4) as *const u8) == tipo)
}

/// adds the fee to the seller's season income and the buyer's season spending, raises the all-time records and queues
/// the record news (0x1f, 0x1e) above (+0x80 + 500) × 1000
/// Original: `Club_Records::add_transfer_totals` (0x448ad0, club_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x448ad0 } (thiscall, ret 0xc, club_records.cpp): `Club_Records::add_transfer_totals(valor, comprador,
/// vendedor)`: o vendedor soma o `valor` ao recebido na temporada (+0x36b do registro da temporada 0x4528d0; −1 =
/// ainda nada: fica o valor) e zera o ano (+0x36f); acima do recorde de sempre (0x452710, mesmo campo), o recorde passa
/// a ser o da temporada com o ano da temporada do clube (0x669950), e com o total no mínimo (+0x80 do clube + 500) ×
/// 1000 e o ano diferente do recorde, a notícia de recorde 0x1f (0x452cb0) se ainda não está na fila. O comprador, o
/// mesmo pelo gasto (+0x365, ano +0x369, notícia 0x1e), mas o mínimo é conferido com o recorde ANTIGO e sem o ano. Os
/// registros são criados se faltarem (o segundo argumento 1). Comprador ou vendedor nulo: nada, 1. Devolve 1.
pub(crate) unsafe extern "thiscall" fn somar_totais_de_transferencia(this: usize, valor: i32, comprador: usize, vendedor: usize) -> u32 {
    if comprador == 0 || vendedor == 0 {
        return 1;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let ano_do_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_GET_CLUB_SEASON_YEAR);
    let noticia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_QUEUE_RECORD_NEWS);
    let somar = |campo: usize| {
        let v = read_usize(campo) as i32;
        write_usize(campo, if v == -1 { valor } else { v.wrapping_add(valor) } as usize);
    };
    // o vendedor: o recebido
    let t = temporada(this, vendedor, 1);
    let s = sempre(this, vendedor, 1);
    somar(t + 0x36b);
    write_i16(t + 0x36f, 0);
    let ano = ano_do_clube(vendedor) as i16;
    if (read_usize(s + 0x36b) as i32) < read_usize(t + 0x36b) as i32 {
        let minimo = (read_i16(vendedor + 0x80) as i32 + 500).wrapping_mul(1000);
        if read_usize(t + 0x36b) as i32 >= minimo && ano != read_i16(s + 0x36f) && !cpm6_noticia_na_fila(this, read_usize(vendedor), 0x1f) {
            crate::comparacao::ramo("cpm_6 noticia de recorde recebido");
            noticia(this, read_usize(vendedor) as u32, 0x1f); // recorde de valor recebido
        }
        write_usize(s + 0x36b, read_usize(t + 0x36b));
        write_i16(s + 0x36f, ano);
    }
    // o comprador: o gasto
    let t = temporada(this, comprador, 1);
    let s = sempre(this, comprador, 1);
    somar(t + 0x365);
    write_i16(t + 0x369, 0);
    let ano = ano_do_clube(comprador) as i16;
    if (read_usize(t + 0x365) as i32) > read_usize(s + 0x365) as i32 {
        let minimo = (read_i16(comprador + 0x80) as i32 + 500).wrapping_mul(1000);
        // a original confere o mínimo com o recorde antigo (o ecx que ainda tem o de sempre)
        if read_usize(s + 0x365) as i32 >= minimo && !cpm6_noticia_na_fila(this, read_usize(comprador), 0x1e) {
            crate::comparacao::ramo("cpm_6 noticia de recorde pago");
            noticia(this, read_usize(comprador) as u32, 0x1e); // recorde de valor pago
        }
        write_usize(s + 0x365, read_usize(t + 0x365));
        write_i16(s + 0x369, ano);
    }
    1
}

/// the club's all-time league appearances (+0x247) and goals (+0x261) records, with the player reference (0x7e4920),
/// when the record type applies to the club (0x449590)
/// Original: `club_records_update_career_records` (0x448cb0, club_records.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x448cb0 } (thiscall, ret 0x10, club_records.cpp): `club_records_update_career_records(jogador, clube, jogos,
/// gols)`: nos recordes de sempre do clube (0x452710, criados se faltarem), se o tipo 0x17 vale para o clube (0x449590)
/// e os `jogos` passam o recorde (+0x247; −1 = sem recorde), o recorde = `jogos` e a referência do jogador em +0x249
/// (0x7e4920); o mesmo com o tipo 0x18 e os `gols` (+0x261, referência +0x263). Devolve 1. Jogador ou clube nulo, ou
/// sem registro: a caixa "Error" (linhas 0x6fa e 0x702) e 0.
pub(crate) unsafe extern "thiscall" fn recordes_de_carreira_no_clube(this: usize, jogador: usize, clube: usize, jogos: u32, gols: u32) -> u32 {
    if jogador == 0 || clube == 0 {
        aviso_em(0x98b6a0, 0x6fa);
        return 0;
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let r = sempre(this, clube, 1);
    if r == 0 {
        aviso_em(0x98b6a0, 0x702);
        return 0;
    }
    let vale: unsafe extern "stdcall" fn(u32, usize, u32) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_RECORD_TYPE_APPLIES);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
    for (tipo, campo, n) in [(0x17u32, 0x247usize, jogos as i16), (0x18, 0x261, gols as i16)] {
        if vale(tipo, clube, 1) != 0 {
            let v = read_i16(r + campo);
            if v == -1 || n > v {
                write_i16(r + campo, n);
                referencia(jogador, r + campo + 2);
            }
        }
    }
    1
}

/// the squad player with the best average rating (statistic 0x11) among those who played at least half the games
/// becomes the season rating record
/// Original: `Club_Records::update_season_rating_record` (0x452a90, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x452a90 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_season_rating_record(clube)`: com jogos
/// na temporada (o byte +0x10 da entrada do clube, [this+0x1c] + id × 0x6a) e o registro da temporada (0x4528d0, sem
/// criar), a nota do recorde (+0x1f1) volta a −1.0 e, pelas 50 vagas do elenco (+0xd7), o jogador (não técnico humano:
/// id < [0xae2364]) com estatísticas (+0x61) no clube (+0x39 = clube ou o ligado a ele, 0x540a50) e registro de
/// estatísticas da temporada (0x7c3920, tipo 6) que jogou ao menos metade dos jogos (estatística 1, truncada, 0x7c2150 com
/// 6.5) e tem a maior média (estatística 0x11) vira o recorde: a nota (+0x1f1, float), os jogos (+0x20d), a referência
/// (+0x1f5, 0x7e4920) e +0x20e = 0. Devolve 1 se alguém virou recorde. Sem jogos ou sem registro: 0. Clube nulo: a
/// original (caixa 0x16f2, 0).
pub(crate) unsafe extern "thiscall" fn recorde_de_nota_da_temporada(this: usize, clube: usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(942)(this, clube); // caixa: só a original
    }
    let entrada = read_usize(this + 0x1c).wrapping_add(read_usize(clube).wrapping_mul(0x6a));
    if *((entrada + 0x10) as *const u8) == 0 {
        return 0;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let r = temporada(this, clube, 0);
    if r == 0 {
        return 0;
    }
    let ligado: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    let estatistica: unsafe extern "stdcall" fn(usize, u32, f32) -> f64 = core::mem::transmute(FN_PLAYER_STATS_GET_RECORD_VALUE);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
    let nota = (r + 0x1f1) as *mut f32;
    write_usize(r + 0x1f1, 0xbf800000); // −1.0f
    let mut achou = 0;
    for k in 0..50usize {
        let p = read_usize(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let est = read_usize(p + 0x61);
        if est == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        if read_usize(p + 0x39) != clube && read_usize(p + 0x39) != ligado(clube, 0, 1) {
            continue;
        }
        let reg = registro(read_usize(G_PLAYER_STATS), est, 6);
        if reg == 0 {
            continue;
        }
        let jogou = ftol(estatistica(reg, 1, 6.5)) as u8;
        if (jogou as i32) < *((entrada + 0x10) as *const i8) as i32 / 2 { // menos da metade dos jogos
            continue;
        }
        let media = estatistica(reg, 0x11, 6.5);
        // fcomp com o −1.0 (double) e fcom com o float guardado: passa só com a média maior
        if *nota as f64 == f64_em(0x968948) || media > *nota as f64 {
            crate::comparacao::ramo("cpm_6 recorde de nota da temporada");
            *nota = media as f32;
            *((r + 0x20d) as *mut u8) = jogou;
            referencia(p, r + 0x1f5);
            write_i16(r + 0x20e, 0);
            achou = 1;
        }
    }
    achou
}

/// Comparador: os registros podem ser criados (realloc) e a notícia realoca: os quatro registros são pedidos antes, na
/// ordem da original (vendedor: temporada, sempre; comprador: temporada, sempre; a segunda vez só acha), e com a fila
/// de notícias sem folga para duas, só a original. Lado a lado pelos quatro registros (+0x365, 12 bytes), a conta das
/// notícias e as duas vagas seguintes.
pub(crate) unsafe extern "thiscall" fn comparar_somar_totais_de_transferencia(this: usize, valor: i32, comprador: usize, vendedor: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32>(940);
    if comprador == 0 || vendedor == 0 || (read_usize(this + 0x28).wrapping_sub(read_usize(this + 0x2c)) as i32) < 2 {
        return o(this, valor, comprador, vendedor); // ponytail: com realloc da fila, só a original
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let _ = [temporada(this, vendedor, 1), sempre(this, vendedor, 1), temporada(this, comprador, 1), sempre(this, comprador, 1)];
    // criar o registro do comprador pode realocar o vetor e mover o do vendedor: pega os quatro de novo (agora só acha)
    let regs = [temporada(this, vendedor, 1), sempre(this, vendedor, 1), temporada(this, comprador, 1), sempre(this, comprador, 1)];
    let fila = read_usize(this + 0x24).wrapping_add(read_usize(this + 0x2c).wrapping_mul(6));
    let reg = [(regs[0] + 0x365, 12), (regs[1] + 0x365, 12), (regs[2] + 0x365, 12), (regs[3] + 0x365, 12), (this + 0x2c, 4), (fila, 12)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, valor, comprador, vendedor), || somar_totais_de_transferencia(this, valor, comprador, vendedor));
    PLACAR.registrar(igual && r == n, || format!("somar_totais_de_transferencia({}, {}, {}) {:x?} x {:x?}", valor, read_usize(comprador), read_usize(vendedor), mo, mr));
    r
}

/// Comparador: o registro de sempre é pedido antes (pode ser criado; a segunda vez só acha); lado a lado pelos dois
/// recordes e as referências (+0x247 a +0x27b). Nulos ou sem registro: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_recordes_de_carreira_no_clube(this: usize, jogador: usize, clube: usize, jogos: u32, gols: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(941);
    if jogador == 0 || clube == 0 {
        return o(this, jogador, clube, jogos, gols);
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let reg = sempre(this, clube, 1);
    if reg == 0 {
        return o(this, jogador, clube, jogos, gols);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(reg + 0x247, 0x34)], || o(this, jogador, clube, jogos, gols), || recordes_de_carreira_no_clube(this, jogador, clube, jogos, gols));
    PLACAR.registrar(igual && r == n, || format!("recordes_de_carreira_no_clube({}, {}, {}, {}) {:x?} x {:x?}", read_usize(jogador), read_usize(clube), jogos as i16, gols as i16, mo, mr));
    r
}

/// Comparador: lado a lado pelo recorde de nota (+0x1f1 a +0x210 do registro da temporada, pedido antes sem criar).
pub(crate) unsafe extern "thiscall" fn comparar_recorde_de_nota_da_temporada(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(942);
    if clube == 0 {
        return o(this, clube);
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let t = temporada(this, clube, 0);
    let reg: &[(usize, usize)] = if t == 0 { &[] } else { &[(t + 0x1f1, 0x1f)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube), || recorde_de_nota_da_temporada(this, clube));
    PLACAR.registrar(igual && r == n, || format!("recorde_de_nota_da_temporada({}) original={} rust={} {:x?} x {:x?}", read_usize(clube), r, n, mo, mr));
    r
}

// ---------- 247a leva: os recordes do clube: achar ou criar os recordes de sempre e os da temporada e guardar a notícia de recorde batido ----------



/// A entrada do clube (0x6a bytes, pelo id +0 do clube) na tabela [this+0x1c]: +0 o índice dos recordes de sempre,
/// +4 o dos da temporada (−1 = ainda sem).
pub(crate) unsafe fn cpk1_entrada(this: usize, clube: usize) -> usize {
    read_usize(this + 0x1c).wrapping_add(read_usize(clube).wrapping_mul(0x6a))
}

/// A falta de memória das tabelas dos recordes (club_records.cpp, linha `linha`): caixa e fim, como a original.
pub(crate) unsafe fn cpk1_sem_memoria(linha: u32) -> ! {
    aviso_em(0x98b6a0, linha);
    erro_fatal()
}

/// the all-time records of the club (index +0 of the 0x6a-byte entry, 0x38b-byte vector at +4); with create, the new
/// index is the count (+0xc), the vector grows by 50 and the record is reset (0x452fe0)
/// Original: `Club_Records::get_all_time_record` (0x452710, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x452710 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_all_time_record(clube, criar)`: os recordes
/// de sempre do clube: o índice +0 da entrada dele ([this+0x1c] + id × 0x6a) no vetor [this+4] de 0x38b bytes. Sem
/// índice (−1) e com `criar`: o índice novo é a conta (+0xc, que sobe), o vetor cresce de 50 em 50 (capacidade +8,
/// realloc 0x945667; sem memória, caixa 0x1694 e fim) e o registro é reiniciado (0x452fe0). Devolve o registro, ou 0
/// sem índice. Clube nulo: a original (caixa 0x1680, 0).
pub(crate) unsafe extern "thiscall" fn recordes_de_sempre(this: usize, clube: usize, criar: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(883)(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube);
    if read_usize(e) as i32 == -1 && criar != 0 {
        crate::comparacao::ramo("cpk_1 recordes de sempre novos");
        if read_usize(this + 0xc) == read_usize(this + 8) {
            let cap = read_usize(this + 8).wrapping_add(50);
            write_usize(this + 8, cap);
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            let p = realocar(read_usize(this + 4), cap.wrapping_mul(0x38b) as u32);
            write_usize(this + 4, p);
            if p == 0 {
                cpk1_sem_memoria(0x1694);
            }
        }
        let n = read_usize(this + 0xc);
        write_usize(e, n);
        write_usize(this + 0xc, n.wrapping_add(1)); // a conta dos recordes de sempre
        let reiniciar: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_SEASON_HISTORY_ENTRY_CLEAR);
        reiniciar(read_usize(this + 4).wrapping_add(read_usize(e).wrapping_mul(0x38b)));
    }
    let i = read_usize(e) as i32;
    if i <= -1 {
        return 0;
    }
    read_usize(this + 4).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// the same for the season records (index +4, vector +0x10, capacity +0x14, count +0x18)
/// Original: `Club_Records::get_season_record` (0x4528d0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x4528d0 } (thiscall, ret 8, club_records.cpp): `Club_Records::get_season_record(clube, criar)`: o mesmo para os
/// recordes da temporada: índice +4 da entrada, vetor [this+0x10], capacidade +0x14, conta +0x18 (sem memória, caixa
/// 0x16c9 e fim). Clube nulo: a original (caixa 0x16b5, 0).
pub(crate) unsafe extern "thiscall" fn recordes_da_temporada(this: usize, clube: usize, criar: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(884)(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube) + 4;
    if read_usize(e) as i32 == -1 && criar != 0 {
        crate::comparacao::ramo("cpk_1 recordes da temporada novos");
        if read_usize(this + 0x18) == read_usize(this + 0x14) {
            let cap = read_usize(this + 0x14).wrapping_add(50);
            write_usize(this + 0x14, cap);
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            let p = realocar(read_usize(this + 0x10), cap.wrapping_mul(0x38b) as u32);
            write_usize(this + 0x10, p);
            if p == 0 {
                cpk1_sem_memoria(0x16c9);
            }
        }
        let n = read_usize(this + 0x18);
        write_usize(e, n);
        write_usize(this + 0x18, n.wrapping_add(1)); // a conta dos recordes da temporada
        let reiniciar: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_SEASON_HISTORY_ENTRY_CLEAR);
        reiniciar(read_usize(this + 0x10).wrapping_add(read_usize(e).wrapping_mul(0x38b)));
    }
    let i = read_usize(e) as i32;
    if i <= -1 {
        return 0;
    }
    read_usize(this + 0x10).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// appends (club id, record type) to the 6-byte record news vector (+0x24), growing by 5; without memory, error box and
/// 0
/// Original: `Club_Records::queue_record_news` (0x452cb0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x452cb0 } (thiscall, ret 8, club_records.cpp): `Club_Records::queue_record_news(id, tipo)`: acrescenta (id do
/// clube +0, tipo do recorde +4; o byte +5 fica como está) ao vetor de 6 bytes [this+0x24] das notícias de recorde,
/// lidas pela 0x448e30. Cheio (conta +0x2c = capacidade +0x28): cresce de 5 em 5 (realloc 0x945667); sem memória,
/// caixa 0x174f e devolve 0 (o jogo segue, sem o vetor). Devolve 1.
pub(crate) unsafe extern "thiscall" fn guardar_noticia_de_recorde(this: usize, id: u32, tipo: u32) -> u32 {
    crate::comparacao::ramo("cpk_1 noticia de recorde");
    if read_usize(this + 0x2c) == read_usize(this + 0x28) {
        let cap = read_usize(this + 0x28).wrapping_add(5);
        write_usize(this + 0x28, cap);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realocar(read_usize(this + 0x24), cap.wrapping_mul(6) as u32);
        write_usize(this + 0x24, p);
        if p == 0 {
            aviso_em(0x98b6a0, 0x174f);
            return 0;
        }
    }
    let n = read_usize(this + 0x2c);
    let item = read_usize(this + 0x24).wrapping_add(n.wrapping_mul(6));
    core::ptr::write_unaligned(item as *mut u32, id);
    *((item + 4) as *mut u8) = tipo as u8;
    write_usize(this + 0x2c, n.wrapping_add(1)); // mais uma notícia
    1
}

/// O plano dos recordes (de sempre ou da temporada) antes da original: o índice esperado (o novo é a conta), a conta
/// esperada e se o registro é novo.
pub(crate) unsafe fn cpk1_plano(e: usize, conta: usize, criar: u32) -> (i32, usize, bool) {
    let i = read_usize(e) as i32;
    let n = read_usize(conta);
    if i == -1 && criar != 0 { (n as i32, n.wrapping_add(1), true) } else { (i, n, false) }
}

/// Confere o resultado da original com o plano: o índice da entrada, a conta, o registro devolvido e, se novo, as
/// marcas da 0x452fe0 (−1.0f em +0x1f1, −1 em +0x313).
pub(crate) unsafe fn cpk1_conferir(nome: &str, clube: usize, e: usize, conta: usize, vetor: usize, plano: (i32, usize, bool), r: usize) {
    let (i, n, novo) = plano;
    let esperado = if i <= -1 { 0 } else { read_usize(vetor).wrapping_add((i as usize).wrapping_mul(0x38b)) };
    let reiniciado = !novo || (r != 0 && read_usize(r + 0x1f1) == 0xbf800000 && read_usize(r + 0x313) as i32 == -1);
    PLACAR.registrar(r == esperado && read_usize(e) as i32 == i && read_usize(conta) == n && reiniciado, || {
        format!("{}(clube {}) índice {} (esperava {}), conta {} (esperava {}), registro {:x} (esperava {:x}), novo={} reiniciado={}",
            nome, read_usize(clube), read_usize(e) as i32, i, read_usize(conta), n, r, esperado, novo, reiniciado)
    });
}

/// Comparador: a criação realoca e não pode rodar duas vezes: o plano da cópia antes, a original roda, confere.
pub(crate) unsafe extern "thiscall" fn comparar_recordes_de_sempre(this: usize, clube: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(883);
    if clube == 0 {
        return o(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube);
    let plano = cpk1_plano(e, this + 0xc, criar);
    let r = o(this, clube, criar);
    cpk1_conferir("recordes_de_sempre", clube, e, this + 0xc, this + 4, plano, r);
    r
}

/// Comparador: o mesmo pelos recordes da temporada.
pub(crate) unsafe extern "thiscall" fn comparar_recordes_da_temporada(this: usize, clube: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(884);
    if clube == 0 {
        return o(this, clube, criar);
    }
    let e = cpk1_entrada(this, clube) + 4;
    let plano = cpk1_plano(e, this + 0x18, criar);
    let r = o(this, clube, criar);
    cpk1_conferir("recordes_da_temporada", clube, e, this + 0x18, this + 0x10, plano, r);
    r
}

/// Comparador: a original acrescenta (pode realocar); confere a conta, o item novo (id, tipo) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_guardar_noticia_de_recorde(this: usize, id: u32, tipo: u32) -> u32 {
    let n = read_usize(this + 0x2c);
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(885)(this, id, tipo);
    if r == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let item = read_usize(this + 0x24).wrapping_add(n.wrapping_mul(6));
    let (gid, gtipo) = (core::ptr::read_unaligned(item as *const u32), *((item + 4) as *const u8));
    PLACAR.registrar(r == 1 && read_usize(this + 0x2c) == n.wrapping_add(1) && gid == id && gtipo == tipo as u8, || {
        format!("guardar_noticia_de_recorde({}, {}) devolveu {}, conta {} -> {}, item ({}, {})", id, tipo as u8, r, n, read_usize(this + 0x2c), gid, gtipo)
    });
    r
}

// ---------- 246a leva: a oferta que bate o recorde de valor pago, a condição do jogador e o limite de escalação da partida ----------


/// the offer fee passes 50 × reputation + 500,000 (and reputation² × 0x967880 when required) and the record (+0x313) of
/// the buyer all-time records
/// Original: `Club_Records::offer_breaks_fee_record` (0x4486c0, club_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x4486c0 } (thiscall, ret 8, club_records.cpp): `Club_Records::offer_breaks_fee_record(oferta, exigir)`: o valor
/// da oferta (+0x1b) bate o recorde de valor pago do clube comprador (+8; os recordes de sempre pela 0x452710, criando
/// se preciso)? Tem de ser ao menos 50 × reputação (+0x80) + 500.000; com `exigir`, também ao menos reputação² × a
/// constante 0x967880 (_ftol); e acima de zero e do recorde (+0x313). Oferta nula: 0.
pub(crate) unsafe extern "thiscall" fn oferta_bate_o_recorde(this: usize, oferta: usize, exigir: u32) -> u8 {
    if oferta == 0 {
        return 0;
    }
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(oferta + 8).wrapping_mul(0x245));
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let rec = recordes(this, clube, 1);
    if rec == 0 || clube == 0 {
        return 0;
    }
    let rep = *((clube + 0x80) as *const i16) as i32;
    let valor = read_usize(oferta + 0x1b) as i32;
    if valor < rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {
        return 0; // abaixo do mínimo pela reputação
    }
    if exigir != 0 && valor < ftol(rep as f64 * rep as f64 * f64_em(0x967880)) {
        return 0;
    }
    (valor != 0 && valor > read_usize(rec + 0x313) as i32) as u8
}



/// Comparador: a 0x452710 cria os recordes do clube na primeira vez (a original primeiro; a cópia acha os mesmos):
/// pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_oferta_bate_o_recorde(this: usize, oferta: usize, exigir: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(873)(this, oferta, exigir);
    let n = oferta_bate_o_recorde(this, oferta, exigir);
    PLACAR.registrar(r == n, || format!("oferta_bate_o_recorde({:x}, {}) original={} rust={}", oferta, exigir, r, n));
    r
}



// ---------- 245a leva: os recordes do clube: a construção dos recordes de valor de transferência pago e recebido e do recorde de sequência ----------

/// constructs the highest fee paid record: the dates at +0x14 and +0x20 (0x54a110)
/// Original: `transfer_fee_record_construct` (0x4485d0, club_records.cpp, thiscall).
///
/// [pt] { 0x4485d0 } (thiscall, club_records.cpp): `transfer_fee_record_construct`: constrói o recorde de maior valor de
/// transferência pago: as datas em +0x14 e +0x20 (0x54a110). O quadro SEH do original só cobre a 2ª data (a 0x54a110
/// não falha). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_valor_pago(this: usize) -> usize {
    cpf2_data_padrao(this + 0x14); // a data do valor pago
    cpf2_data_padrao(this + 0x20);
    this
}

/// constructs the highest fee received record: the dates at +0x14 and +0x20 (0x54a110)
/// Original: `transfer_fee_record_construct_2` (0x448620, club_records.cpp, thiscall).
///
/// [pt] { 0x448620 } (thiscall, club_records.cpp): `transfer_fee_record_construct_2`: constrói o recorde de maior valor de
/// transferência recebido: as datas em +0x14 e +0x20 (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_valor_recebido(this: usize) -> usize {
    cpf2_data_padrao(this + 0x14);
    cpf2_data_padrao(this + 0x20); // a 2ª data do valor recebido
    this
}

/// constructs the run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `run_record_construct` (0x448670, club_records.cpp, thiscall).
///
/// [pt] { 0x448670 } (thiscall, club_records.cpp): `run_record_construct`: constrói o recorde de sequência: as datas do
/// começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_sequencia(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo da sequência
    cpf2_data_padrao(this + 0xc);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_valor_pago(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(846);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_de_valor_pago(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_valor_pago {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_valor_recebido(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(847);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_de_valor_recebido(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_valor_recebido {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_sequencia(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(848);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_sequencia(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_sequencia {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 245a leva: os recordes do clube: a construção dos recordes de vitórias, derrotas e invencibilidade seguidas ----------

/// constructs the won run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `won_run_record_construct` (0x450ad0, club_records.cpp, thiscall).
///
/// [pt] { 0x450ad0 } (thiscall, club_records.cpp): `won_run_record_construct`: constrói o recorde de vitórias seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_vitorias_seguidas(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo das vitórias
    cpf2_data_padrao(this + 0xc);
    this
}

/// constructs the lost run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `lost_run_record_construct` (0x450b20, club_records.cpp, thiscall).
///
/// [pt] { 0x450b20 } (thiscall, club_records.cpp): `lost_run_record_construct`: constrói o recorde de derrotas seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_derrotas_seguidas(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim das derrotas
    this
}

/// constructs the unbeaten run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `unbeaten_run_record_construct` (0x450b70, club_records.cpp, thiscall).
///
/// [pt] { 0x450b70 } (thiscall, club_records.cpp): `unbeaten_run_record_construct`: constrói o recorde de jogos sem perder:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_invencibilidade(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo da invencibilidade
    cpf2_data_padrao(this + 0xc);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_vitorias_seguidas(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(849);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_vitorias_seguidas(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_vitorias_seguidas {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_derrotas_seguidas(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(850);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_derrotas_seguidas(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_derrotas_seguidas {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_invencibilidade(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(851);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_invencibilidade(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_invencibilidade {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 245a leva: os recordes do clube: a construção dos recordes de jogos sem vencer, de gols numa partida e do jogador mais novo ----------

/// constructs the winless run record: the start (+4) and end (+0xc) dates (0x54a110)
/// Original: `winless_run_record_construct` (0x450bc0, club_records.cpp, thiscall).
///
/// [pt] { 0x450bc0 } (thiscall, club_records.cpp): `winless_run_record_construct`: constrói o recorde de jogos sem vencer:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_sem_vencer(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim dos jogos sem vencer
    this
}

/// constructs the goals in a match record: the holder date (+0x11) and the match date (+0x19) (0x54a110)
/// Original: `goals_in_match_record_construct` (0x450c10, club_records.cpp, thiscall).
///
/// [pt] { 0x450c10 } (thiscall, club_records.cpp): `goals_in_match_record_construct`: constrói o recorde de gols numa
/// partida: a data do recordista (+0x11) e a da partida (+0x19) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_gols_na_partida(this: usize) -> usize {
    cpf2_data_padrao(this + 0x11);
    cpf2_data_padrao(this + 0x19); // a data da partida
    this
}

/// constructs the youngest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110)
/// Original: `youngest_player_record_construct` (0x450c70, club_records.cpp, thiscall).
///
/// [pt] { 0x450c70 } (thiscall, club_records.cpp): `youngest_player_record_construct`: constrói o recorde do jogador mais
/// novo: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_do_mais_novo(this: usize) -> usize {
    cpf2_data_padrao(this); // a idade do mais novo
    cpf2_data_padrao(this + 0x18);
    cpf2_data_padrao(this + 0x20);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_sem_vencer(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(852);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_sem_vencer(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_sem_vencer {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_gols_na_partida(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(853);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x21)], || o(this), || construir_recorde_de_gols_na_partida(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_gols_na_partida {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_do_mais_novo(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(854);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_do_mais_novo(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_do_mais_novo {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 245a leva: o recorde do jogador mais velho e as permissões de trabalho: achar a do jogador e se ele passa no limite de estrangeiros ----------


/// constructs the oldest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110)
/// Original: `oldest_player_record_construct` (0x450cd0, club_records.cpp, thiscall).
///
/// [pt] { 0x450cd0 } (thiscall, club_records.cpp): `oldest_player_record_construct`: constrói o recorde do jogador mais
/// velho: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). O quadro SEH do original não
/// é preciso (a 0x54a110 não falha). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_do_mais_velho(this: usize) -> usize {
    cpf2_data_padrao(this);
    cpf2_data_padrao(this + 0x18); // a data do mais velho
    cpf2_data_padrao(this + 0x20);
    this
}



pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_do_mais_velho(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(864);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_do_mais_velho(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_do_mais_velho {:x?} x {:x?}", mo, mr));
    r
}



// ---------- 243a leva: as cópias dos recordes de nota, de melhor em campo e de disciplina dos recordes do clube (club_records.cpp) ----------

/// O recordista (0x18 bytes) de `origem` em `destino`, pela 0x448490 (`record_holder_assign`, já em Rust).
pub(crate) unsafe fn cpf1_recordista(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_RECORD_HOLDER_ASSIGN);
    copiar(destino, origem);
}

/// copies the rating record (0x1f bytes): the rating (+0), the holder (+4, through 0x448490), +0x1c and +0x1d
/// Original: `rating_record_assign` (0x448230, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448230 } (thiscall, ret 4, club_records.cpp): `rating_record_assign`: copia o recorde de nota (0x1f bytes) de
/// `origem`: a nota (+0, dupla), o recordista em +4, o byte +0x1c e a palavra +0x1d. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_de_nota(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32)); // a nota
    cpf1_recordista(this + 4, origem + 4);
    *((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8); // o byte do recorde de nota
    core::ptr::write_unaligned((this + 0x1d) as *mut u16, core::ptr::read_unaligned((origem + 0x1d) as *const u16));
    this
}

/// copies the man of the match record (0x1b bytes): the count (+0), the holder (+1) and the year (+0x19)
/// Original: `man_of_match_record_assign` (0x4484d0, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x4484d0 } (thiscall, ret 4, club_records.cpp): `man_of_match_record_assign`: copia o recorde de melhor em campo
/// (0x1b bytes) de `origem`: as vezes (+0, byte), o recordista em +1 e o ano da temporada (+0x19). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_de_melhor_em_campo(this: usize, origem: usize) -> usize {
    *(this as *mut u8) = *(origem as *const u8); // as vezes de melhor em campo
    cpf1_recordista(this + 1, origem + 1);
    core::ptr::write_unaligned((this + 0x19) as *mut u16, core::ptr::read_unaligned((origem + 0x19) as *const u16));
    this
}

/// copies the discipline record (0x1c bytes): yellows (+0), reds (+1), the holder (+2) and the year (+0x1a)
/// Original: `discipline_record_assign` (0x448500, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448500 } (thiscall, ret 4, club_records.cpp): `discipline_record_assign`: copia o recorde de disciplina (0x1c
/// bytes) de `origem`: os amarelos (+0) e vermelhos (+1), o recordista em +2 e o ano da temporada (+0x1a). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_de_disciplina(this: usize, origem: usize) -> usize {
    *(this as *mut u8) = *(origem as *const u8);
    *((this + 1) as *mut u8) = *((origem + 1) as *const u8); // os vermelhos
    cpf1_recordista(this + 2, origem + 2);
    core::ptr::write_unaligned((this + 0x1a) as *mut u16, core::ptr::read_unaligned((origem + 0x1a) as *const u16));
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_de_nota(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(808);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, origem), || copiar_recorde_de_nota(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_nota {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_de_melhor_em_campo(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(809);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1b)], || o(this, origem), || copiar_recorde_de_melhor_em_campo(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_melhor_em_campo {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_de_disciplina(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(810);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1c)], || o(this, origem), || copiar_recorde_de_disciplina(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_disciplina {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 243a leva: a cópia do recorde de contagem e do torcedor e a construção dos recordes de contagem (club_records.cpp) ----------

/// copies the count record (0x1a bytes): the count (+0, word) and the holder (+2, through 0x448490)
/// Original: `count_record_assign` (0x448530, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448530 } (thiscall, ret 4, club_records.cpp): `count_record_assign`: copia o recorde de contagem (0x1a bytes) de
/// `origem`: a contagem (+0, palavra) e o recordista em +2 (pela 0x448490, já em Rust). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_de_contagem(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u16, core::ptr::read_unaligned(origem as *const u16)); // a contagem
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_RECORD_HOLDER_ASSIGN);
    copiar(this + 2, origem + 2);
    this
}

/// copies the fans player record (0x1a bytes): the holder (+0) and the year (+0x18)
/// Original: `fans_player_record_assign` (0x4485b0, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x4485b0 } (thiscall, ret 4, club_records.cpp): `fans_player_record_assign`: copia o recorde de jogador da
/// torcida (0x1a bytes) de `origem`: o recordista em +0 (pela 0x448490) e o ano da temporada (+0x18). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_do_torcedor(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_RECORD_HOLDER_ASSIGN);
    copiar(this, origem);
    core::ptr::write_unaligned((this + 0x18) as *mut u16, core::ptr::read_unaligned((origem + 0x18) as *const u16)); // o ano do torcedor
    this
}

/// A data do recordista em `a` pela 0x54a110 (o construtor da data: dia 40, ano 1972, +4 = 1).
pub(crate) unsafe fn cpf2_data_padrao(a: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    data_padrao(a);
}

/// constructs the tally record: the holder date at +0x11 (0x54a110)
/// Original: `tally_record_construct` (0x450c60, club_records.cpp, thiscall).
///
/// [pt] { 0x450c60 } (thiscall, club_records.cpp): `tally_record_construct`: constrói o recorde de ocorrências (contagem em
/// byte + recordista): só a data do recordista em +0x11 (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_ocorrencias(this: usize) -> usize {
    cpf2_data_padrao(this + 0x11); // a data das ocorrências
    this
}

/// constructs the count record: the holder date at +0x12 (0x54a110)
/// Original: `count_record_construct` (0x450d30, club_records.cpp, thiscall).
///
/// [pt] { 0x450d30 } (thiscall, club_records.cpp): `count_record_construct`: constrói o recorde de contagem (palavra +
/// recordista): só a data do recordista em +0x12 (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recorde_de_contagem(this: usize) -> usize {
    cpf2_data_padrao(this + 0x12); // a data da contagem
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_de_contagem(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(811);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || copiar_recorde_de_contagem(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_contagem {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_do_torcedor(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(812);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || copiar_recorde_do_torcedor(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_do_torcedor {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_ocorrencias(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(813);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x19)], || o(this), || construir_recorde_de_ocorrencias(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_ocorrencias {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recorde_de_contagem(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(814);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this), || construir_recorde_de_contagem(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_contagem {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 243a leva: as cópias do recorde de valor de transferência e do recorde de gols numa partida (club_records.cpp) ----------


/// copies the transfer fee record (0x29 bytes): five dwords, the dates (+0x14, +0x20), +0x1c and the part-exchange flag
/// (+0x28)
/// Original: `transfer_fee_record_assign` (0x448550, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x448550 } (thiscall, ret 4, club_records.cpp): `transfer_fee_record_assign`: copia o recorde de valor de
/// transferência (0x29 bytes) de `origem`: cinco duplas (+0..+0x10: o valor, o jogador, o outro clube...), a data em
/// +0x14, a dupla +0x1c, a data em +0x20 e a marca de troca (+0x28). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_de_valor(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    for o in [0, 4, 8, 0xc, 0x10] {
        d(o); // o valor e os ids
    }
    copiar_data_do_recorde(this + 0x14, origem + 0x14);
    d(0x1c);
    copiar_data_do_recorde(this + 0x20, origem + 0x20);
    *((this + 0x28) as *mut u8) = *((origem + 0x28) as *const u8); // a marca de troca
    this
}

/// copies the goals in a match record (0x38 bytes): the goals (+0), the holder (+1) and the match (+0x19)
/// Original: `goals_in_match_record_assign` (0x451570, club_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x451570 } (thiscall, ret 4, club_records.cpp): `goals_in_match_record_assign`: copia o recorde de gols numa
/// partida (0x38 bytes) de `origem`: os gols (+0, byte), o recordista em +1 (0x448490) e o registro da partida em +0x19
/// (0x448110, ambos já em Rust). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recorde_de_gols_na_partida(this: usize, origem: usize) -> usize {
    let recordista: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_RECORD_HOLDER_ASSIGN);
    let partida: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MATCH_RECORD_ASSIGN);
    *(this as *mut u8) = *(origem as *const u8); // os gols
    recordista(this + 1, origem + 1);
    partida(this + 0x19, origem + 0x19);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_de_valor(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(815);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x29)], || o(this, origem), || copiar_recorde_de_valor(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_valor {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recorde_de_gols_na_partida(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(816);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x38)], || o(this, origem), || copiar_recorde_de_gols_na_partida(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_gols_na_partida {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 216a leva: as cópias do recordista, do registro de partida e do registro de sequência dos recordes do clube (club_records.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local.
pub(crate) unsafe fn copiar_data_do_recorde(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// copies the record holder: four identity fields and the date
/// Original: `record_holder_assign` (0x448490, club_records.cpp, thiscall).
///
/// [pt] { 0x448490 } (thiscall, ret 4, club_records.cpp): copia o recordista (0x18 bytes) de `origem`:
/// quatro palavras duplas de identificação (+0..+0xc) e a data em +0x10. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_recordista(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    d(0); // identificação
    d(4);
    d(8);
    d(0xc);
    copiar_data_do_recorde(this + 0x10, origem + 0x10);
    this
}

/// copies the match record: the date, the opponent, the score and the fixture fields
/// Original: `match_record_assign` (0x448110, club_records.cpp, thiscall).
///
/// [pt] { 0x448110 } (thiscall, ret 4, club_records.cpp): copia o registro de partida (0x1f bytes) de
/// `origem`: a data em +0, depois +8 (dupla), +0xc e +0xe (palavras), +0x10 (byte), +0x11 e +0x15
/// (duplas), +0x19 e +0x1a (bytes) e +0x1b (dupla). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_registro_de_partida(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    copiar_data_do_recorde(this, origem);
    d(8); // adversário
    w(0xc);
    w(0xe);
    b(0x10);
    d(0x11);
    d(0x15);
    b(0x19);
    b(0x1a);
    d(0x1b);
    this
}

/// copies the run record: the game count and the start and end dates
/// Original: `run_record_assign` (0x448170, club_records.cpp, thiscall).
///
/// [pt] { 0x448170 } (thiscall, ret 4, club_records.cpp): copia o registro de sequência (0x14 bytes) de
/// `origem`: o número de jogos (+0) e as datas de início (+4) e fim (+0xc). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_registro_de_sequencia(this: usize, origem: usize) -> usize {
    *(this as *mut u32) = *(origem as *const u32); // jogos
    copiar_data_do_recorde(this + 4, origem + 4);
    copiar_data_do_recorde(this + 0xc, origem + 0xc);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_recordista(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(548);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x18)], || o(this, origem), || copiar_recordista(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recordista {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_registro_de_partida(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(549);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, origem), || copiar_registro_de_partida(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_registro_de_partida {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_registro_de_sequencia(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(550);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this, origem), || copiar_registro_de_sequencia(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_registro_de_sequencia {:x?} x {:x?}", mo, mr));
    r
}

/// −1.0f and −1 in the club record fields
/// Original: `reset_records` (0x452fe0, club_records.cpp, cdecl).
///
/// [pt] { 0x452fe0 } (cdecl, club_records.cpp): reinicia os recordes do clube `r`: −1.0f em +0x1f1 e −1
/// nos campos de id (dwords, palavras e bytes). Devolve 1. `r` nulo: a original (caixa de erro).
pub(crate) unsafe extern "C" fn reiniciar_recordes(r: usize) -> u32 {
    if r == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(363)(r);
    }
    let d = |o: usize| core::ptr::write_unaligned((r + o) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((r + 0x1f1) as *mut u32, 0xbf800000);
    for o in [1, 8, 0xe, 0x31, 0x54, 0x77, 0x85, 0xa4, 0xc3, 0xe2, 0x101, 0x118, 0x12c, 0x140, 0x154] {
        d(o);
    }
    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210, 0x22b] {
        *((r + o) as *mut u8) = 0xff;
    }
    for o in [0x247, 0x261] {
        core::ptr::write_unaligned((r + o) as *mut u16, 0xffff);
    }
    d(0x2a3);
    d(0x2e2);
    core::ptr::write_unaligned((r + 0x2f9) as *mut u16, 0xffff);
    for o in [0x313, 0x33c, 0x365, 0x36b, 0x37d] {
        d(o);
    }
    1
}

pub(crate) unsafe extern "C" fn comparar_reiniciar_recordes(r: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(363);
    let reg = if r == 0 { vec![] } else { vec![(r, 0x381)] };
    let (a, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(r), || reiniciar_recordes(r));
    PLACAR.registrar(igual && a == v, || format!("reiniciar_recordes({:#x}) original={} {:x?} rust={} {:x?}", r, a, mo, v, mr));
    a
}

/// builds the record holder: only the date at +0x10 (0x54a110)
/// Original: `record_holder_construct` (0x450d40, club_records.cpp, thiscall).
///
/// [pt] { 0x450d40 } (thiscall, club_records.cpp): `record_holder_construct`: constrói o recordista: só a data em +0x10
/// (0x54a110). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_recordista(this: usize) -> usize {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    padrao(this + 0x10); // a data do recordista
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_recordista(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(844);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x10, 8)], || o(this), || construir_recordista(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recordista {:x?} x {:x?}", mo, mr));
    r
}
