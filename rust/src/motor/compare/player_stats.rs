use super::*;

// The comparers of the routines in ../player_stats.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../player_stats.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `player_statistic` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_player_statistic(r: *const u8, qual: i8, peso: f32) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(IDX_PLAYER_STATISTIC)(r, qual, peso);
    let n = player_statistic(r, qual, peso);
    PLACAR.registrar(o.to_bits() == n.to_bits(), || format!("player_statistic({:p}, {}, {}) original={} rust={}", r, qual, peso, o, n));
    o
}

/// Comparer for `archive_player_stats` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem gravar no arquivo (os nós que gravaria entram na
/// soma dos jogos e gols), depois a original a partir do mesmo estado; compara a devolução, os seis registros de 32 bytes
/// das categorias e os jogos e gols pelo clube (+0x20/+0x22 do registro da ficha). As posições do histórico (+0x18,
/// +0x1c), que só a gravação da original move, ficam fora. Pessoa nula ou sem ficha: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_archive_player_stats(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_ARCHIVE_PLAYER_STATS);
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        return o(this, p, valor, emprestimo, fim, clube_id, outro);
    }
    let reg = read_u32_as_usize(this + 0x30).wrapping_add((read_i32(read_u32_as_usize(p + 0x61)) as isize * 0x27) as usize);
    let mut regioes = vec![(reg + 0x20, 4)];
    for cat in 0..6usize {
        let id = read_i32(reg + 4 * cat);
        if id != -1 {
            regioes.push((read_u32_as_usize(this + 4).wrapping_add((id as isize * 0x20) as usize), 32));
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    ARCHIVE_STATS_DRY_RUN = true;
    ARCHIVE_STATS_PENDING_TOTALS = (0, 0);
    ARCHIVE_STATS_CLUB_ID = clube_id as i32;
    let n = crate::aleatorio::pausar_rastro(|| archive_player_stats(this, p, valor, emprestimo, fim, clube_id, outro));
    ARCHIVE_STATS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = o(this, p, valor, emprestimo, fim, clube_id, outro);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(r == n && mo == mr, || format!("archive_player_stats(pessoa {:x}, valor {}, clube {}) original={} rust={} {:x?} x {:x?}",
        p, valor as i32, clube_id as i32, r, n, mo.0, mr.0));
    r
}

/// Comparer for `build_career_history` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as duas montam a tabela (a original e a cópia, a partir do mesmo estado; o arquivo de histórico só é
/// lido) e comparam a conta e as entradas; a da cópia é liberada (0x9452ca) e o jogo segue com a da original. Os casos
/// de erro (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_build_career_history(this: usize, p: usize, conta: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize>(IDX_BUILD_CAREER_HISTORY);
    if p == 0 || (read_i32(p) < read_i32(G_STAFF_COUNT) && read_u32_as_usize(p + 0x61) == 0) {
        return o(this, p, conta);
    }
    let r = o(this, p, conta);
    let mut n = 0i32;
    let t = build_career_history(this, p, &mut n);
    let igual = if r == 0 || t == 0 {
        r == t && *conta == n
    } else {
        *conta == n
            && core::slice::from_raw_parts(r as *const u8, n.max(0) as usize * HISTORY_ENTRY_SIZE)
                == core::slice::from_raw_parts(t as *const u8, n.max(0) as usize * HISTORY_ENTRY_SIZE)
    };
    PLACAR.registrar(igual, || {
        let k = (*conta).max(n).max(0) as usize;
        let dif = (0..k).find(|&i| {
            r == 0 || t == 0 || core::slice::from_raw_parts((r + i * HISTORY_ENTRY_SIZE) as *const u8, HISTORY_ENTRY_SIZE) != core::slice::from_raw_parts((t + i * HISTORY_ENTRY_SIZE) as *const u8, HISTORY_ENTRY_SIZE)
        });
        format!("build_career_history(pessoa {:x}) conta original={} rust={}, primeira entrada diferente {:?}", p, *conta, n, dif)
    });
    if t != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        liberar(t);
    }
    r
}

/// Comparer for `player_stats_years_since_at_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (16 bits); a tabela é montada e liberada pelas duas. Os casos de erro: só a original.
pub(crate) unsafe extern "stdcall" fn comparar_player_stats_years_since_at_club(p: usize, clube: usize) -> i16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> i16>(IDX_PLAYER_STATS_YEARS_SINCE_AT_CLUB);
    if p == 0 || clube == 0 || (read_i32(p) < read_i32(G_STAFF_COUNT) && read_u32_as_usize(p + 0x39) != clube && read_u32_as_usize(p + 0x61) == 0) {
        return o(p, clube);
    }
    let r = o(p, clube);
    let n = player_stats_years_since_at_club(p, clube);
    PLACAR.registrar(r == n, || format!("player_stats_years_since_at_club(pessoa {:x}, clube {:x}) original={} rust={}", p, clube, r, n));
    r
}

/// Comparer for `player_stats_compare_history_by_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_player_stats_compare_history_by_staff(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_STATS_COMPARE_HISTORY_BY_STAFF)(a, b);
    let n = player_stats_compare_history_by_staff(a, b);
    PLACAR.registrar(r == n, || format!("player_stats_compare_history_by_staff({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `get_stat` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_stat(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64>(IDX_GET_STAT);
    if jogador == 0 || (read_usize(jogador) as i32) >= read_usize(G_PLAYER_DATA_COUNT) as i32 {
        return o(this, jogador, categoria, qual, padrao);
    }
    let c = categoria as u8 as i8;
    let simples = qual as u8 != 0x13 && c <= 6;
    // simplification: categoria fora de 1..6 cai na caixa da 0x7c3920; nas combinadas sem o registro 4 ou 5, locais sem valor
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    if (simples && !(1..=6).contains(&c))
        || (!simples && (100..=102).contains(&(c as i32)) && (registro(this, jogador, 4) == 0 || registro(this, jogador, 5) == 0))
    {
        return o(this, jogador, categoria, qual, padrao);
    }
    let r = o(this, jogador, categoria, qual, padrao);
    let n = get_stat(this, jogador, categoria, qual, padrao);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || {
        format!("get_stat(jogador {}, categoria {}, qual {}, padrão {}) original={} rust={}", read_usize(jogador) as i32, c, qual as u8, padrao, r, n)
    });
    r
}

/// Comparer for `player_stats_compare_language_entries` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_player_stats_compare_language_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_STATS_COMPARE_LANGUAGE_ENTRIES)(a, b);
    let n = player_stats_compare_language_entries(a, b);
    PLACAR.registrar(r == n, || format!("player_stats_compare_language_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `player_stats_compare_history_by_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_player_stats_compare_history_by_club(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_STATS_COMPARE_HISTORY_BY_CLUB)(a, b);
    let n = player_stats_compare_history_by_club(a, b);
    PLACAR.registrar(r == n, || format!("player_stats_compare_history_by_club({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `player_stats_get_career_goals` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "stdcall" fn comparar_player_stats_get_career_goals(pessoa: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(IDX_PLAYER_STATS_GET_CAREER_GOALS);
    if pessoa == 0 || (read_i32(pessoa) < read_i32(G_STAFF_COUNT) && read_u32_as_usize(pessoa + staff::PLAYER) == 0) {
        return o(pessoa);
    }
    let r = o(pessoa);
    let n = player_stats_get_career_goals(pessoa);
    PLACAR.registrar(r == n, || format!("player_stats_get_career_goals(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}

/// Comparer for `get_category_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê); os casos de erro só na original.
pub(crate) unsafe extern "thiscall" fn comparar_get_category_record(this: usize, jogador: usize, categoria: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CATEGORY_RECORD)(this, jogador, categoria);
    if category_slot(this, jogador, categoria).is_none() {
        return r;
    }
    let n = get_category_record(this, jogador, categoria);
    PLACAR.registrar(r == n, || format!("get_category_record({:x}, {:x}, {}) original={:x} rust={:x}", this, jogador, categoria as u8 as i8, r, n));
    r
}

/// Comparer for `create_stats` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original realoca o vetor, então só ela roda; antes, o plano (o índice que o jogador vai ter, a
/// conta e os soltos depois); depois, o registro devolvido, o índice na vaga, a conta, os soltos e o registro
/// zerado quando é novo. simplification: os casos de erro e a falta de memória só na original, sem conferir.
pub(crate) unsafe extern "thiscall" fn comparar_create_stats(this: usize, jogador: usize, categoria: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_CREATE_STATS);
    let vaga = match category_slot(this, jogador, categoria) {
        Some(v) => v,
        _ => return o(this, jogador, categoria),
    };
    let atual = read_usize(vaga) as i32;
    let (n, soltos) = (read_usize(this + 0xc), read_usize(this + 0x38));
    let solto = if atual == -1 && soltos != 0 {
        (0..n as i32).find(|&i| core::ptr::read_unaligned((read_usize(this + 4) + ((i as usize) << 5) + 0xe) as *const u16) == 0xffff)
    } else {
        None
    };
    let (indice, n2, soltos2) = match (atual, solto) {
        (-1, Some(i)) => (i, n, soltos.wrapping_sub(1)),
        (-1, None) => (n as i32, n.wrapping_add(1), soltos),
        (i, _) => (i, n, soltos),
    };
    let r = o(this, jogador, categoria);
    if r == 0 {
        return r;
    }
    let esperado = read_usize(this + 4).wrapping_add((indice as usize) << 5);
    let zerado = atual != -1 || (0..0x20).all(|k| *((r + k) as *const u8) == 0);
    let igual = r == esperado && read_usize(vaga) as i32 == indice && read_usize(this + 0xc) == n2 && read_usize(this + 0x38) == soltos2 && zerado;
    PLACAR.registrar(igual, || format!("create_stats({:x}, {:x}, {}) original={:x} plano={:x} índice {} x {} conta {} x {} soltos {} x {} zerado={}",
        this, jogador, categoria as u8 as i8, r, esperado, read_usize(vaga) as i32, indice, read_usize(this + 0xc), n2, read_usize(this + 0x38), soltos2, zerado));
    r
}

/// Comparer for `add_language` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x7c6a30 aloca e realoca a tabela (não pode rodar duas vezes): o plano antes (a conta e a
/// capacidade depois, a fluência), a original roda; confere a conta, a capacidade e o registro novo (no fim sem ordenar;
/// com `ordenar`, em algum lugar da tabela).
pub(crate) unsafe extern "thiscall" fn comparar_add_language(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    let (n, cap) = if read_u32_as_usize(this + 0x3c) == 0 { (read_u32_as_usize(this + 0x44), 100) } else { (read_u32_as_usize(this + 0x44), read_u32_as_usize(this + 0x40)) };
    let cap = if cap == n { cap.wrapping_add(100) } else { cap };
    let f = fluencia as u8;
    let novo = (read_u32_as_usize(pessoa) as u32, idioma as u8, if (f as i8) > 10 { 10 } else { f });
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_ADD_LANGUAGE)(this, pessoa, idioma, fluencia, ordenar);
    let base = read_u32_as_usize(this + 0x3c);
    let e = |i: usize| {
        let r = base.wrapping_add(i * 6);
        (read_u32_as_usize(r) as u32, *((r + 4) as *const u8), *((r + 5) as *const u8))
    };
    let conta = read_u32_as_usize(this + 0x44);
    let mut ok = conta == n.wrapping_add(1) && read_u32_as_usize(this + 0x40) == cap;
    if ok {
        ok = if ordenar == 0 { e(n) == novo } else { (0..conta).any(|i| e(i) == novo) };
    }
    PLACAR.registrar(ok, || format!("add_language({:?}, ordenar {}) conta {} (esperava {}) capacidade {} (esperava {})",
        novo, ordenar, conta, n.wrapping_add(1), read_u32_as_usize(this + 0x40), cap));
}

/// Comparer for `improve_language` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem tabela ou sem o registro, só a original (o acréscimo 0x7c6a30 tem o comparador dele); com o
/// registro, lado a lado nos 6 bytes dele.
pub(crate) unsafe extern "thiscall" fn comparar_improve_language(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_IMPROVE_LANGUAGE);
    let r = if read_u32_as_usize(this + 0x3c) == 0 { 0 } else { find_language_record(this, pessoa, idioma as u8) };
    if r == 0 {
        return o(this, pessoa, idioma, aumento, ordenar);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 6)],
        || o(this, pessoa, idioma, aumento, ordenar), || improve_language(this, pessoa, idioma, aumento, ordenar));
    PLACAR.registrar(igual, || format!("improve_language({}, idioma {}, +{}) {:x?} x {:x?}", read_u32_as_usize(pessoa), idioma as u8, aumento as u8, mo, mr));
}

/// Comparer for `player_stats_reset_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no registro de 39 bytes, na conta dos livres (+0x38) e na marca +0xe dos registros de
/// categoria liberados (a 0x58f110 só grava +0x1c, dentro do registro). Pessoa nula ou não jogadora: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_player_stats_reset_player(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_STATS_RESET_PLAYER);
    if pessoa == 0 || read_u32_as_usize(pessoa + staff::PLAYER) == 0 {
        return o(this, pessoa);
    }
    let r = read_u32_as_usize(this + 0x30).wrapping_add(read_u32_as_usize(read_u32_as_usize(pessoa + staff::PLAYER)).wrapping_mul(39)); // o id da ficha de jogador (+0x61), não o da pessoa
    let mut reg = vec![(r, 39), (this + 0x38, 4)];
    for k in 0..6usize {
        let i = read_u32_as_usize(r + 4 * k);
        if i as u32 != 0xffff_ffff {
            reg.push((read_u32_as_usize(this + 4).wrapping_add(i << 5) + 0xe, 2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || player_stats_reset_player(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("player_stats_reset_player({}) {:x?} x {:x?}", read_u32_as_usize(pessoa), mo, mr));
    a
}

/// Comparer for `get_language_fluency` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x4cf5f0 e a busca só leem).
pub(crate) unsafe extern "thiscall" fn comparar_get_language_fluency(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(IDX_GET_LANGUAGE_FLUENCY)(this, pessoa, clube, idioma, pelo_contrato);
    let n = get_language_fluency(this, pessoa, clube, idioma, pelo_contrato);
    PLACAR.registrar(r == n, || format!("get_language_fluency({:x}, {:x}, {}, {}) original={} rust={}", pessoa, clube, idioma as u8, pelo_contrato, r, n));
    r
}

/// Comparer for `player_stats_record_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador lado a lado: as somas em double (+0x10..+0x30), os registros das categorias de cada linha que conta, o
/// trecho +0x20..+0x27 do registro do jogador e o +0x3b do contrato; no jogo simulado (competição −2), também o estado
/// do sorteio (e as duas categorias possíveis), com o sorteio da cópia fora da contagem. Partida ou jogo nulos, a caixa
/// de uma competição sem tipo conhecido ou registro de categoria a criar (0x7c5d80 cresce o vetor): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_player_stats_record_match(this: usize, p: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PLAYER_STATS_RECORD_MATCH);
    if p == 0 || jogo == 0 {
        return o(this, p, jogo); // simplification: as caixas, só a original
    }
    let simulado = read_usize(p) as i32 == -2;
    let mut regioes = vec![(this + 0x10, 0x20)];
    for lado in 0..2usize {
        let cats: Vec<u8> = if simulado {
            vec![2, 3, 6]
        } else {
            let mut erro = false;
            let c = match_stats_category(p, lado as i32, false, &mut erro);
            if erro {
                return o(this, p, jogo); // simplification: a caixa, só a original
            }
            stats_categories_for(c).into_iter().flatten().collect()
        };
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = match_lineup_row(jogo, lado, k) else { continue };
            for &c in &cats {
                let r = player_category_record(this, dados, c);
                if r == 0 {
                    return o(this, p, jogo); // simplification: o registro seria criado, só a original
                }
                regioes.push((r, 0x20));
            }
            regioes.push((read_usize(this + 0x30).wrapping_add(read_usize(e + 4).wrapping_mul(0x27)) + 0x20, 7));
            let ct = contract_of_staff(pessoa);
            if ct != 0 {
                regioes.push((ct + 0x3b, 2));
            }
        }
    }
    if simulado {
        regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, p, jogo), || {
        crate::aleatorio::sem_contar(|| player_stats_record_match(this, p, jogo))
    });
    PLACAR.registrar(igual && ro == rr, || format!("player_stats_record_match({:x}, {:x}, {:x}) original={} {:x?} rust={} {:x?}", this, p, jogo, ro, mo, rr, mr));
    ro
}

/// Comparer for `end_season_for_key_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x7c5fe0 move as estatísticas para o histórico (aloca): roda só a original e confere a devolução
/// (1, ou 0 sem nação).
pub(crate) unsafe extern "thiscall" fn comparar_end_season_for_key_nation(this: usize, nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_END_SEASON_FOR_KEY_NATION)(this, nacao);
    PLACAR.registrar(r == (nacao != 0) as u32, || format!("end_season_for_key_nation({:x}) devolveu {}", nacao, r));
    r
}

/// Comparer for `remove_language` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_language(estat: usize, ficha: usize, idioma: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_REMOVE_LANGUAGE);
    let tabela = *((estat + 0x3c) as *const usize);
    let contagem = *((estat + 0x44) as *const i32);
    if tabela == 0 || contagem <= 0 {
        return o(estat, ficha, idioma);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(estat + 0x44, 4), (tabela, contagem as usize * 6)],
        || o(estat, ficha, idioma), || remove_language(estat, ficha, idioma));
    PLACAR.registrar(igual, || format!("remove_language({}) contagem {:x?} x {:x?}", idioma, mo.0[0].1, mr.0[0].1));
}

/// Comparer for `get_club_league_stat` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_club_league_stat(estat: usize, jogador: usize, campo: i8) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f32>(IDX_GET_CLUB_LEAGUE_STAT)(estat, jogador, campo);
    if jogador == 0 || !(1..=2).contains(&campo) {
        return r;
    }
    let n = get_club_league_stat(estat, jogador, campo);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("get_club_league_stat({}) original={} rust={}", campo, r, n));
    r
}

/// Comparer for `get_last_match_rating` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_last_match_rating(estatisticas: usize, id: *const i32) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(IDX_GET_LAST_MATCH_RATING)(estatisticas, id);
    if id.is_null() {
        return r;
    }
    let n = get_last_match_rating(estatisticas, id);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("get_last_match_rating({}) original={} rust={}", *id, r, n));
    r
}

/// Comparer for `player_stats_get_goals_for_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "stdcall" fn comparar_player_stats_get_goals_for_club(pessoa: usize, clube: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u16>(IDX_PLAYER_STATS_GET_GOALS_FOR_CLUB);
    if pessoa == 0 || clube == 0 || (read_i32(pessoa) < read_i32(G_STAFF_COUNT) && read_u32_as_usize(pessoa + staff::PLAYER) == 0) {
        return o(pessoa, clube);
    }
    let r = o(pessoa, clube);
    let n = player_stats_get_goals_for_club(pessoa, clube);
    PLACAR.registrar(r == n, || format!("player_stats_get_goals_for_club(pessoa {:x}, clube {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

/// Comparer for `competition_weight` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_competition_weight(p: usize, cod: i8) -> u16 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, i8) -> u16>(IDX_COMPETITION_WEIGHT)(p, cod);
    let n = competition_weight(p, cod);
    PLACAR.registrar(r == n, || format!("competition_weight({:#x}, {}) original={} rust={}", p, cod, r, n));
    r
}
