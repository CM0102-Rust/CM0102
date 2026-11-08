// ---------- %%LEVA%%a leva: a nação-chave do jogo: a nação-chave ativa de uma competição ou jogo, se a competição é a Copa do Mundo e se é um torneio internacional (comp.cpp, key_nation.cpp) ----------

unsafe fn cpt3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x48dd70 } (cdecl, comp.cpp): `comp_active_key_nation_of_fixture(competição, partida)`, a chave das listas de jogos
/// do FIXTURE_MANAGER (add_fixture 0x5aa680, find_fixtures 0x5acbe0, ...), da simulação rápida (0x6b66a0) e do peso do
/// ranking da FIFA (0x58cf70). Amistoso (0x5c6710 sobre [0xae2c74]) ou reservas ([0x9cf684]) com a partida dada e a
/// competição dela amistosa ([0xae2c74]+4) ou de reservas: com uma das duas seleções (0x53b900 dos clubes +0x1c/+0x20),
/// 1; nações (+0x53) diferentes ou fora da escolha (+0x11c), 3; a nação na tabela das 35 nações-chave (0xb63d60, 72
/// bytes cada, escolhida em +0x11c), o byte +0x14 da linha; senão 0 com o mandante seleção e 3 sem. Senão, competição de
/// nação (+0x5d): o byte +0x14 da linha dela, ou 3. Sem nação: Copa do Mundo e eliminatórias (0x669a10), 0; torneio
/// internacional (0x669ae0), 1; senão o índice da nação-chave (0x669470). TapeCall a cada jogo procurado: sem contador.
unsafe extern "C" fn comp_active_key_nation_of_fixture(comp: usize, partida: usize) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let linha = |nacao: usize| (0..0x23usize).find(|&k| {
        let n = cpt3_d(0xb63d60 + 72 * k);
        n != 0 && *((n + 0x11c) as *const u8) != 0 && n == nacao
    });
    let nacao = cpt3_d(comp + 0x5d);
    let reservas = cpt3_d(0x9cf684);
    if (amistoso(cpt3_d(0xae2c74), comp) != 0 || cpt3_d(comp) == reservas) && partida != 0 {
        let pc = cpt3_d(partida + 0x14);
        if pc == cpt3_d(cpt3_d(0xae2c74) + 4) || cpt3_d(pc) == reservas {
            if selecao(cpt3_d(partida + 0x1c)) != 0 || selecao(cpt3_d(partida + 0x20)) != 0 {
                return 1;
            }
            let casa = cpt3_d(partida + 0x1c);
            let a = cpt3_d(casa + 0x53);
            if a != cpt3_d(cpt3_d(partida + 0x20) + 0x53) || *((a + 0x11c) as *const u8) == 0 {
                return 3;
            }
            if let Some(k) = linha(a) {
                return *((0xb63d74 + 72 * k) as *const u8); // cpt3: a nacao do amistoso
            }
            return if selecao(casa) != 0 { 0 } else { 3 };
        }
    }
    if cpt3_d(comp + 0x5d) != 0 {
        return match linha(nacao) {
            Some(k) => *((0xb63d74 + 72 * k) as *const u8), // cpt3: a nacao-chave da competicao
            None => 3,
        };
    }
    let copa: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669a10usize);
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669ae0usize);
    let indice: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    if copa(comp) != 0 {
        return 0; // cpt3: Copa do Mundo e eliminatorias
    }
    if internacional(comp) != 0 {
        return 1; // cpt3: torneio internacional
    }
    indice(comp)
}

unsafe extern "C" fn comparar_comp_active_key_nation_of_fixture(comp: usize, partida: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I0%%)(comp, partida);
    let n = comp_active_key_nation_of_fixture(comp, partida);
    PLACAR.record(r == n, || format!("comp_active_key_nation_of_fixture(competição {}, partida {:x}) original={} rust={}", cpt3_d(comp) as i32, partida, r, n));
    r
}

/// Os ids das competições da Copa do Mundo: a final e as seis eliminatórias (Europa [0x9cf780], África, Ásia,
/// CONCACAF, Oceania, América do Sul).
const WORLD_CUP_COMPS: [usize; 7] = [0x9cf79c, 0x9cf77c, 0x9cf774, 0x9cf770, 0x9cf780, 0x9cf76c, 0x9cf778];

/// Os ids dos torneios de seleções fora da Copa do Mundo: Olimpíada, Eurocopa, eliminatórias da Eurocopa, Copa
/// Africana [0x9cf7ac], Copa América, seleção B, sub-21, Copa da Oceania, Copa das Confederações e Copa da Ásia.
const INTERNATIONAL_TOURNAMENTS: [usize; 10] = [0x9cf7a0, 0x9cf7a4, 0x9cf784, 0x9cf7ac, 0x9cf7a8, 0x9cf794, 0x9cf798, 0x9cf8a0, 0x9cf964, 0x9cf790];

/// { 0x669a10 } (cdecl, key_nation.cpp): `key_nation_is_world_cup_comp(competição)`: o id (+0) é a Copa do Mundo ou uma
/// das eliminatórias, 1; senão 0. Competição nula: a caixa "Error" (linha 0x3d7) e 0.
unsafe extern "C" fn key_nation_is_world_cup_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3d7);
        return 0;
    }
    let id = cpt3_d(comp);
    WORLD_CUP_COMPS.iter().any(|&g| cpt3_d(g) == id) as u32
}

unsafe extern "C" fn comparar_key_nation_is_world_cup_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_world_cup_comp(comp);
    PLACAR.record(r == n, || format!("key_nation_is_world_cup_comp(competição {}) original={} rust={}", cpt3_d(comp) as i32, r, n));
    r
}

/// { 0x669ae0 } (cdecl, key_nation.cpp): `key_nation_is_international_comp(competição)`: o id é um dos dez torneios de
/// seleções fora da Copa do Mundo, 1; senão 0. Competição nula: a caixa (linha 0x3ef) e 0.
unsafe extern "C" fn key_nation_is_international_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3ef);
        return 0;
    }
    let id = cpt3_d(comp);
    INTERNATIONAL_TOURNAMENTS.iter().any(|&g| cpt3_d(g) == id) as u32
}

unsafe extern "C" fn comparar_key_nation_is_international_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_international_comp(comp);
    PLACAR.record(r == n, || format!("key_nation_is_international_comp(competição {}) original={} rust={}", cpt3_d(comp) as i32, r, n));
    r
}
