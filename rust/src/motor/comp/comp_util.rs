use super::*;

/// Does club competition `id` count (for deciding a group/league early)? Returns 1 for three fixed lower divisions
/// (Spanish lower division, French CFA, French lower division); 0 if the competition record (107 bytes, +0x5d) is not
/// flagged; otherwise 1 only if its stage object is at stage 2 (+0x43), has rounds (+0x3e) and the first table row's
/// club (+0x57 or +0x60) is the stage's competition (+4).
///
/// [pt] { 0x4b0700 } (comp_util.cpp): a competicao `id` conta? As tres de 0x9cf6b0, 0x9cf5d8 e 0x9cf548
/// sim; sem o +0x5d do registro (0xae23d0, 107 bytes) nao; senao so a do objeto em 0xadadfc com
/// fase 2 (+0x43), rodadas (+0x3e) e a primeira rodada (+0xb1) com o clube +4 em +0x57 ou +0x60.
pub(crate) unsafe extern "C" fn competition_counts(id: i32) -> u32 {
    let g = |a: usize| *(a as *const i32);
    if id == g(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) || id == g(KEY_CLUB_COMP_FRENCH_CFA) || id == g(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) {
        return 1;
    }
    if *((*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(107)).wrapping_add(0x5d) as *const u32) == 0 {
        return 0;
    }
    let c = *((*(G_COMP_OBJECT_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(4)) as *const usize);
    if c == 0 || byte(c, 0x43) != 2 || *((c + 0x3e) as *const u16) == 0 {
        return 0;
    }
    let r = *((c + 0xb1) as *const usize);
    if r == 0 {
        return 0;
    }
    let primeira = *(r as *const usize);
    let alvo = *((c + 4) as *const i32);
    (*((primeira + 0x57) as *const i32) == alvo || *((primeira + 0x60) as *const i32) == alvo) as u32
}

/// qsort comparator: orders records by the signed byte at +0x5f, descending (second minus first; a null pointer counts
/// as 0).
///
/// [pt] { 0x4b0780 } (comparacao do qsort): byte +0x5f do segundo menos o do primeiro (nulo vale 0).
pub(crate) unsafe extern "C" fn order_by_5f(a: *const usize, b: *const usize) -> i32 {
    let v = |p: usize| if p == 0 { 0 } else { *((p + 0x5f) as *const i8) as i32 };
    v(*b) - v(*a)
}

/// qsort comparator: sorts records in descending order of their signed 16-bit word at +0x80 (b minus a).
///
/// [pt] { 0x4b07b0 } (comparacao do qsort): palavra +0x80 do segundo menos a do primeiro.
pub(crate) unsafe extern "C" fn order_by_80(a: *const usize, b: *const usize) -> i32 {
    *((*b + 0x80) as *const i16) as i32 - *((*a + 0x80) as *const i16) as i32
}

/// qsort comparator for club pointers: sorts by the club's word at +0x80 (reputation) in descending order, with null
/// entries placed last.
///
/// [pt] { 0x4b07d0 } (comparacao do qsort): como 0x4b07b0, com os nulos no fim (primeiro nulo: 1 se o
/// segundo nao for; segundo nulo: -1).
pub(crate) unsafe extern "C" fn order_by_80_desc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    *((y + 0x80) as *const i16) as i32 - *((x + 0x80) as *const i16) as i32
}

/// qsort comparator: orders records by their 16-bit word at +0x80 (the club reputation) in ascending order, null
/// entries last.
///
/// [pt] { 0x4b0810 } (comparacao do qsort): como 0x4b07d0, em ordem crescente.
pub(crate) unsafe extern "C" fn order_by_80_asc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    *((x + 0x80) as *const i16) as i32 - *((y + 0x80) as *const i16) as i32
}

/// Returns the units digit of round code `n` (n % 10) when `n` is below 1000, else 0; used to read the round variant
/// from a fixture's round code.
///
/// [pt] { 0x4b0850 } (comp_util.cpp): algarismo das unidades de `n` (abaixo de 1000; senao 0).
pub(crate) unsafe extern "C" fn units_digit(n: i32) -> u8 {
    if n < 1000 {
        (n % 10) as u8
    } else {
        0
    }
}

/// Tells whether competition `comp` is a friendly: it is the one stored at +4 of the friendly manager `this`, or its id
/// (+0) is among the last 127 club competitions (count at 0xae2378). Returns 1/0.
///
/// [pt] { 0x5c6710 } (friendly.cpp): a competicao e amistosa? E a guardada em +4 do objeto, ou o id dela
/// (+0) esta entre as 127 ultimas competicoes de clubes (quantidade em 0xae2378).
pub(crate) unsafe extern "thiscall" fn is_friendly(this: usize, comp: usize) -> u32 {
    if comp == *((this + 4) as *const usize) {
        return 1;
    }
    (*(comp as *const i32) >= (*(G_CLUB_COMP_COUNT as *const i32)).wrapping_sub(0x7f)) as u32
}

/// Returns the person's contract inside a contract list: `this` points to an array of 80-byte contract records, and the
/// index comes from the person's 79-byte staff-state record (table 0xae2d0c, indexed by person id). Returns 0 when
/// there is no person, the id is out of range (person count 0xae2364) or the index is negative.
///
/// [pt] { 0x4cf5a0 } (contract_manager.cpp): contrato da pessoa (registros de 80 bytes a partir de +0
/// do objeto), pelo indice na tabela 0xae2d0c (79 bytes por pessoa; quantidade em 0xae2364).
/// Sem pessoa, id fora da tabela ou indice negativo: 0.
pub(crate) unsafe extern "thiscall" fn contract_of(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let tabela = *(G_STAFF_STATE_TABLE as *const usize);
    let i = *((tabela as isize + 79 * id as isize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(this as *const usize)).wrapping_add((80 * i) as usize)
}

/// Returns 1 if the club is a national team: national-team ids come after the clubs, so the id must be at least the
/// club count minus twice the nation count. A null club returns 0 and clears the global at 0xb67a34 (the original also
/// shows an error box, skipped here).
///
/// [pt] { 0x53b900 } (Database.cpp): o clube e uma selecao? Os ids das selecoes vem depois dos clubes
/// (total de clubes em 0xae235c menos 2 x nacoes em 0xae2350). Clube nulo: o original mostra uma
/// caixa de erro, zera 0xb67a34 e devolve 0 (aqui, sem a caixa).
pub(crate) unsafe extern "C" fn is_national_team(clube: usize) -> u32 {
    if clube == 0 {
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        return 0;
    }
    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
    (*(clube as *const i32) >= limit) as u32
}



/// Computes each club's strength key (0x4c0720) and sorts clubs and keys descending by swapping, as 0x4c08f0.
///
/// [pt] O núcleo da 0x4c08f0 sobre os `n` clubes em `v` e as chaves em `k`: a chave de cada clube é a força
/// (0x4c0720; clube nulo, 0); depois, para cada i, todo j > i com chave maior troca de lugar com i (chave e clube),
/// com sinal.
pub(crate) unsafe fn sort_clubs_by_strength(v: *mut u32, k: *mut i32, n: i32) {
    let forca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_COMP_UTIL_GET_CLUB_STRENGTH_SCORE);
    for i in 0..n.max(0) as usize {
        let c = *v.add(i);
        *k.add(i) = if c != 0 { forca(c as usize) } else { 0 };
    }
    let mut i = 0usize;
    while (i as i32) < n - 1 {
        for j in i + 1..n as usize {
            if *k.add(i) < *k.add(j) { // cpt13: a força maior sobe
                core::ptr::swap(k.add(i), k.add(j));
                core::ptr::swap(v.add(i), v.add(j));
            }
        }
        i += 1;
    }
}

/// sorts an array of clubs by strength (0x4c0720), strongest first
/// Original: `comp_util_sort_clubs_by_strength` (0x4c08f0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4c08f0 } (cdecl, comp_util.cpp): `comp_util_sort_clubs_by_strength(clubes, n)`: ordena o vetor de `n` clubes
/// pela força (0x4c0720), a maior primeiro, com as chaves num vetor temporário (0x944e46 / 0x9452ca; a original o
/// enche de 0x60 antes, tudo sobrescrito). Sem memória: a caixa (linha 0x292d) e nada.
pub(crate) unsafe extern "C" fn comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let k = alocar((n as u32).wrapping_mul(4));
    if k == 0 {
        return show_error_at(0x99ec20, 0x292d);
    }
    sort_clubs_by_strength(v as *mut u32, k as *mut i32, n);
    if n > 1 {
        crate::comparacao::ramo("cpt_13 clubes ordenados");
    }
    liberar(k);
}

/// finds or adds the club's record in the competition statistics and stores the position at +0x72
/// Original: `Comp_Stats::set_club_final_position` (0x4a2540, comp_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x4a2540 } (thiscall, ret 8, comp_stats.cpp): `Comp_Stats::set_club_final_position(clube, posição)`: acha
/// (0x4a4850) ou acrescenta (0x4aceb0) o registro do clube na estatística da competição e grava a posição (byte) em
/// +0x72; devolve o registro. Clube nulo: a caixa (linha 0x680).
pub(crate) unsafe extern "thiscall" fn set_club_final_position(this: usize, clube: usize, pos: u32) -> usize {
    if clube == 0 {
        show_error_at(0x99ca94, 0x680);
        return 0; // simplification: a original devolve o eax da caixa; quem chama não lê
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_ADD_CLUB_ENTRY);
    let mut e = buscar(this, clube);
    if e == 0 {
        e = acrescentar(this, clube);
    }
    crate::comparacao::ramo("cpt_13 posição gravada");
    core::ptr::write_volatile((e + 0x72) as *mut u8, pos as u8); // cpt13: a posição
    e
}

/// The calls 0x5223a0 makes to Comp_Stats set_club_final_position, round by round backward from the second-to-last;
/// returns the (club, position) list.
///
/// [pt] As chamadas da 0x5223a0 à 0x4a2540 (clube, posição), na ordem, com o vetor `v` já ordenado: da penúltima rodada
/// (+0x3c − 2) para trás ([+0xa3] + r × 0x68, a conta de quem sai em +0x1a), até um clube nulo no começo da rodada.
/// Na primeira rodada vista, os que saem nela ganham todos a posição 1 (sem olhar nulos); em cada rodada, os seguintes
/// ganham a posição do contador (que sobe a cada vaga, nula ou não), pulando os nulos. `gravar` = 0 só monta a lista.
pub(crate) unsafe fn set_final_positions_by_round(this: usize, v: *const u32, gravar: usize) -> Vec<(usize, u8)> {
    let gravar_posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let mut chamadas: Vec<(usize, u8)> = Vec::new();
    let mut s: i16 = 0; // o índice no vetor (bp com sinal)
    let mut b: u8 = 1; // o contador de posições
    let mut a: u8 = 1; // a posição empurrada (só o byte de baixo conta; a original empurra a dword com sobra em cima)
    let mut r = read_i16(this + 0x3c).wrapping_sub(2) as i32;
    while r >= 0 {
        if *v.offset(s as isize) == 0 {
            break;
        }
        let conta = |r: i32| read_i16(read_u32_as_usize(this + 0xa3) + r as usize * 0x68 + 0x1a) as i32;
        if r == read_i16(this + 0x3c) as i32 - 2 {
            let mut j = 0;
            while j < conta(r) {
                let c = *v.offset(s as isize) as usize;
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if gravar != 0 {
                    gravar_posicao(read_u32_as_usize(this + 8), c, a as u32);
                }
                b = b.wrapping_add(1);
                j += 1;
            }
        }
        a = b; // cpt13: a posição da rodada
        let mut j = 0;
        while j < conta(r) {
            let c = *v.offset(s as isize) as usize;
            if c != 0 {
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if gravar != 0 {
                    crate::comparacao::ramo("cpt_13 posição pela rodada");
                    gravar_posicao(read_u32_as_usize(this + 8), c, a as u32);
                }
            }
            b = b.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    chamadas
}







/// the club side in the fixture: 1 home, 2 away, 3 neutral, by the fixture stadium and the two clubs' stadiums
/// (national sides: by the stadium nation)
/// Original: `comp_util_get_fixture_venue_side` (0x4afa80, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afa80 } (cdecl, comp_util.cpp): `comp_util_get_fixture_venue_side(clube, jogo)`: o lado do clube no jogo:
/// 1 em casa, 2 fora, 3 neutro. Sem campo no jogo (+0x18): pelo mandante (+0x1c), 3 se ele não tem estádio (+0x69),
/// senão 1 para o mandante e 2 para o outro. Seleção (0x53b900): 1 se a nação do campo (+0x38, +0x1f) é a do clube
/// (+0x53), 2 se é a do adversário, senão 3. Clube: o campo é o estádio do clube e não o do adversário, 1; dos dois,
/// 1 para o mandante e 2 para o outro; só do adversário, 2; de nenhum, 3. Quem chama lê só o al.
pub(crate) unsafe extern "C" fn comp_util_get_fixture_venue_side(clube: usize, jogo: usize) -> u8 {
    let campo = read_u32(jogo + 0x18);
    let casa = read_ptr(jogo + 0x1c);
    if campo == 0 {
        if read_u32(casa + 0x69) == 0 {
            return 3;
        }
        return if clube != casa { 2 } else { 1 };
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        let nacao = read_ptr(campo as usize + 0x38);
        if nacao != 0 && read_u32(nacao + 0x1f) == read_u32(clube + club::NATION) {
            return 1;
        }
        let casa = read_ptr(jogo + 0x1c);
        let outro = if casa == clube { read_ptr(jogo + 0x20) } else { casa };
        if nacao != 0 && read_u32(nacao + 0x1f) == read_u32(outro + 0x53) {
            return 2;
        }
        return 3;
    }
    let casa = read_ptr(jogo + 0x1c);
    let outro = if casa == clube { read_ptr(jogo + 0x20) } else { casa };
    let campo = read_u32(jogo + 0x18);
    if campo != read_u32(clube + club::STADIUM) {
        return if campo != read_u32(outro + 0x69) { 3 } else { 2 };
    }
    if campo == read_u32(outro + 0x69) {
        return if clube != casa { 2 } else { 1 }; // o mesmo estádio dos dois
    }
    crate::comparacao::ramo("cpq_12 em casa");
    1 // o estádio do clube
}


/// the qsort order of the nation coefficients: highest first
/// Original: `compare_nation_coefficients_desc` (0x4afba0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afba0 } (cdecl, comp_util.cpp): `compare_nation_coefficients_desc(a, b)`: a ordem do qsort (12 bytes por
/// entrada, o coeficiente em double em +4) da game_update_nation_coefficients 0x5d6187 (no dia do mês da tabela
/// 0x9cd200): maior antes (−1); menor ou sem ordem (NaN, a flag C0 do fcomp), 1; igual, 0. Quente (o qsort): sem
/// contador. A original ordena e libera o vetor logo depois: a ordem não chega ao jogo (só o comparando acusa).
pub(crate) unsafe extern "C" fn compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let x = core::ptr::read_unaligned((a + 4) as *const f64);
    let y = core::ptr::read_unaligned((b + 4) as *const f64);
    if x > y {
        return -1; // o maior antes
    }
    if !(x >= y) {
        return 1;
    }
    0
}








/// Swaps two randomly drawn entries of a pointer array.
///
/// [pt] As trocas de `vezes` pares sorteados (0x90cfc0(n) duas vezes; a primeira dá o índice que recebe o item do segundo)
/// no vetor de ponteiros `itens`, como as duas originais (índices de 16 bits com sinal).
pub(crate) unsafe fn swap_random_pair(itens: usize, n: i32) {
    let a = friendly_random(n) as i16 as isize;
    let b = friendly_random(n) as i16 as isize;
    let pa = (itens as isize + 4 * a) as usize;
    let pb = (itens as isize + 4 * b) as usize;
    let (va, vb) = (read_usize(pa), read_usize(pb));
    write_usize(pa, vb);
    write_usize(pb, va);
}

/// shuffles an array of n pointers with 3 × n swaps of drawn pairs
/// Original: `comp_util_shuffle_pointer_array` (0x4afb40, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afb40 } (cdecl, comp_util.cpp): `comp_util_shuffle_pointer_array(itens, n)`: embaralha o vetor de `n` (curto)
/// ponteiros com 3 × n trocas de pares sorteados (0x90cfc0(n) duas vezes cada).
pub(crate) unsafe extern "C" fn comp_util_shuffle_pointer_array(itens: usize, n: u32) {
    let n = n as u16 as i16 as i32;
    if 3 * n <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar ponteiros");
    let mut k = 0i16;
    loop {
        swap_random_pair(itens, n);
        k = k.wrapping_add(1); // mais uma troca
        if k as i32 >= 3 * n {
            break;
        }
    }
}

/// sorts the clubs by reputation (nulls last) and shuffles the first min(3 × picks, n)
/// Original: `comp_util_sort_and_shuffle_top_clubs` (0x4afbd0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afbd0 } (cdecl, comp_util.cpp): `comp_util_sort_and_shuffle_top_clubs(clubes, n, escolhas)`: ordena os `n`
/// clubes pela reputação, maior primeiro e nulos no fim (qsort 0x945333 com 0x4b07d0), e embaralha os m = min(3 ×
/// escolhas, n) primeiros com m trocas de pares sorteados entre eles. `n` negativo ou menor que `escolhas`: caixa
/// "Error" (a original); `escolhas` < 1: nada.
pub(crate) unsafe extern "C" fn comp_util_sort_and_shuffle_top_clubs(clubes: usize, n: u32, escolhas: u32) {
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS)(clubes, n, escolhas); // a caixa
    }
    if e < 1 {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(clubes, nn as i32 as u32, 4, 0x4b07d0);
    let mut m = e.wrapping_mul(3);
    if m > nn {
        m = nn;
    }
    if m <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar os melhores");
    for _ in 0..m as i32 { // m trocas
        swap_random_pair(clubes, m as i32);
    }
}






/// A club's rating in the league stats (0x4b02e0): weighted average of 4 attributes, weights scaled by games under 25.
///
/// [pt] A nota de um clube na estatística da liga (0x4b02e0): pesos 1 nos 4 atributos (+0x72..+0x75; +0x75 e +0x74 com
/// o peso [0x968968]) ou, com menos de 25 jogos (+4), proporcionais aos jogos; a média ponderada, na ordem do x87.
pub(crate) unsafe fn league_stats_rating(e: usize, b73: f64) -> f64 {
    let (mut p, mut k73, mut k72) = (read_f64_at(0x968968), 1.0f64, 1.0f64); // 0x968968 = 0.5
    let jogos = read_u8(e + 4);
    if jogos < 0x19 {
        let v = jogos as f64;
        p = v * read_f64_at(0x968a98); // 0x968a98 = 0.02
        k73 = v * read_f64_at(0x968a90); // 0x968a90 = 0.04
        k72 = read_f64_at(0x968938) - v * read_f64_at(0x968a88); // 0x968938 = 4, 0x968a88 = 0.1200480192
    }
    let s = read_u8(e + 0x75) as f64 * p + read_u8(e + 0x72) as f64 * k72 + b73 * k73 + read_u8(e + 0x74) as f64 * p;
    s / (p + p + k72 + k73)
}

/// 1 = title clash, 3 = zone clash, 0 = none, between two clubs of the same league, from the weighted statistics
/// ratings (0x4a4850) against a third of the clubs or 0.2 × clubs + +0xbe (+ +0xbf)
/// Original: `comp_util_get_league_clash_type` (0x4b02e0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b02e0 } (cdecl, comp_util.cpp): `comp_util_get_league_clash_type(a, b)`: os dois clubes na mesma divisão
/// (+0x57, não reserva nem menor: 0x7e6f20), a liga dela ([0xadadfc][id]) com estatística (+8, 0x4a4850 dos dois),
/// sem fases (+0x30 = −1) e com +0x43 = 2, e a diferença de +0x73 até um quarto dos clubes (+0x3e). Com as notas
/// (league_stats_rating): 1 = briga pelo título (virtual +0x98 = 0, as duas abaixo de um terço dos clubes e cada uma não abaixo
/// do +0x74 do outro); 3 = briga na zona (+0x98 ≠ 0, as duas até 0,2 × clubes + +0xbe, ou com +0xbf até 0,2 × clubes
/// + +0xbe + +0xbf, e o mesmo cruzamento do +0x74); senão 0. Devolve só `al`. Sem SEH, sem sorteio; a virtual +0x98
/// é chamada até três vezes, como na original.
pub(crate) unsafe extern "C" fn comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let comp = read_usize(a + 0x57);
    if comp == 0 || comp != read_usize(b + 0x57) {
        return 0;
    }
    let menor: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    if menor(comp) != 0 {
        return 0;
    }
    let liga = read_usize((*(G_COMP_OBJECT_TABLE as *const usize)).wrapping_add(4 * *(read_usize(a + 0x57) as *const u32) as usize));
    if liga == 0 || read_usize(liga + 8) == 0 {
        return 0;
    }
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let ea = achar(read_usize(liga + 8), a);
    let eb = achar(read_usize(liga + 8), b);
    if ea == 0 || eb == 0 || read_usize(liga + 0x30) as i32 != -1 || read_u8(liga + 0x43) != 2 {
        return 0;
    }
    let (a73, b73) = (read_u8(ea + 0x73) as i32, read_u8(eb + 0x73) as i32);
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    if (a73 - b73).abs() > match_pl_ftol(clubes as f64 * read_f64_at(0x9689e0)) as i16 as i32 { // 0x9689e0 = 0.25
        return 0;
    }
    let (na, nb) = (league_stats_rating(ea, a73 as f64), league_stats_rating(eb, b73 as f64));
    let (a74, b74) = (read_u8(ea + 0x74) as f64, read_u8(eb + 0x74) as f64);
    let virtual98 = || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(liga) + 0x98));
        f(liga)
    };
    // as comparações do x87: NaN conta como "menor" (C0) e "menor ou igual" (C0|C3)
    let cruzadas = !(a74 > nb) && !(b74 > na);
    if virtual98() == 0 {
        let t = clubes as f64 * read_f64_at(0x968a80); // 0x968a80 = 0.3333333333
        if !(na >= t) && !(nb >= t) && cruzadas {
            return 1; // briga pelo título
        }
    }
    let be = read_u8(liga + 0xbe) as i8 as f64;
    if virtual98() != 0 {
        let t = clubes as f64 * read_f64_at(0x968918) + be; // 0x968918 = 0.2
        if !(na > t) && !(nb > t) && cruzadas {
            return 3;
        }
    }
    if virtual98() == 0 {
        return 0;
    }
    let bf = read_u8(liga + 0xbf) as i8;
    if bf == 0 {
        return 0;
    }
    let t = clubes as f64 * read_f64_at(0x968918) + be + bf as f64; // 0x968918 = 0.2
    if !(na > t) && !(nb > t) && cruzadas {
        3
    } else {
        0
    }
}

// simplification: ~4 milhões de chamadas na carreira: comparação só do valor, sem regiões (a rotina só lê; a 0x4a4850 pode
// reordenar a estatística, e a original já deixa ordenada para a cópia).

/// money (+0x65 / 400 or balance / 300) + 16 × reputation + the off-match ratings (0x6e6340) of the 15 picked
/// (0x6e2000) from the squad; null club: −10,000,000
/// Original: `comp_util_get_club_strength_score` (0x4c0720, comp_util.cpp, cdecl).
///
/// [pt] { 0x4c0720 } (cdecl, comp_util.cpp): `comp_util_get_club_strength_score(clube)`: o dinheiro (sem tabela de
/// finanças [0xae2a38]: +0x65 / 400; com ela, o saldo de 64 bits / 300 pela 0x5a0590), mais 16 × a reputação (+0x80),
/// mais a nota fora da partida (0x6e6340, 0x840, limite −1) dos 15 escalados (0x6e2000, códigos 1, 0x804, 0x84,
/// 3 × 0x204, 0x810, 0x90, 3 × 0x210, 4 × 0x40) entre os jogadores (+0x61) do elenco (0x541f90). Clube nulo:
/// −10.000.000. Sem SEH, sem sorteio próprio. A original empurra o número de jogadores como dword com só o byte de
/// baixo gravado (o resto é sobra de pilha); a 0x6e2000 só lê o byte, e a cópia passa o byte estendido.
pub(crate) unsafe extern "C" fn comp_util_get_club_strength_score(clube: usize) -> i32 {
    if clube == 0 {
        return -10_000_000;
    }
    let financas = *(G_FINANCE_TABLE as *const usize);
    let mut forca: i32 = 0;
    if financas == 0 {
        forca = core::ptr::read_unaligned((clube + club::CASH) as *const i32) / 400;
    } else {
        let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
        let f = get_club_finance(financas, clube);
        if f != 0 {
            forca = (core::ptr::read_unaligned(f as *const i64) / 300) as i32;
        }
    }
    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32) << 4); // a reputação
    let mut elenco = [0usize; 50];
    let mut jogadores = [0usize; 50];
    let mut escalados = [0usize; 15];
    let codigos: [u16; 15] = [1, 0x804, 0x84, 0x204, 0x204, 0x204, 0x810, 0x90, 0x210, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    reunir(clube, elenco.as_mut_ptr(), 0);
    let mut k: i8 = 0;
    for &p in elenco.iter() {
        if p != 0 && read_usize(p + 0x61) != 0 {
            jogadores[k as usize] = p;
            k = k.wrapping_add(1);
        }
    }
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    pick(jogadores.as_ptr() as usize, k as u8 as u32, codigos.as_ptr() as usize, 0xf, 0, escalados.as_mut_ptr() as usize, 0x840, clube, 0, 0, 0, 0, 0, 0, 0);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_SELECTION);
    for (j, &p) in escalados.iter().enumerate() {
        if p != 0 {
            forca = forca.wrapping_add(nota(p, codigos[j] as u32, 0x840, clube, -1, 0));
        }
    }
    forca
}

// simplification: 1.117 chamadas; só o valor. A escalação e as notas têm comparadores próprios, e o físico que a nota
// atualiza uma vez (0x62ed70, com sorteio) a original já deixa pronto para a cópia.

/// sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) of the first row of the
/// competition; not found: 0
/// Original: `comp_util_get_latest_placed_club` (0x4afd80, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afd80 } (cdecl, comp_util.cpp): `comp_util_get_latest_placed_club(comp, lugar)`: ordena o histórico de
/// competições (qsort 0x945333 com 0x540780), busca (bsearch 0x946464) a competição com ano −1 (qualquer), volta até a
/// primeira linha dela e devolve o clube do `lugar` (0 campeão +0xa, 1 vice +0xe, 2 terceiro +0x12); não achou: 0.
/// Outro lugar: a original (a caixa do comp_util.cpp). A chave na pilha da original só tem +4 e +8 gravados, e a
/// 0x540780 só lê esses dois; a cópia zera o resto.
pub(crate) unsafe extern "C" fn comp_util_get_latest_placed_club(comp: usize, lugar: u32) -> usize {
    let lugar = lugar as u8;
    if lugar > 2 {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_COMP_UTIL_GET_LATEST_PLACED_CLUB)(comp, lugar as u32); // caixa: só a original
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let buscar: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let comparar: unsafe extern "C" fn(*const u8, usize) -> i32 = core::mem::transmute(FN_DATABASE_COMPARE_COMP_HISTORY_ROWS);
    qsort(*(G_CLUB_COMP_HISTORY_TABLE as *const usize), *(G_CLUB_COMP_HISTORY_COUNT as *const i32), 0x1a, 0x540780);
    let mut chave = [0u8; 0x1a];
    chave[4..8].copy_from_slice(&comp.to_le_bytes());
    chave[8..10].copy_from_slice(&0xffffu16.to_le_bytes()); // o ano: qualquer
    let mut p = buscar(chave.as_ptr(), *(G_CLUB_COMP_HISTORY_TABLE as *const usize), *(G_CLUB_COMP_HISTORY_COUNT as *const i32), 0x1a, 0x540780);
    if p == 0 {
        return 0;
    }
    let t = *(G_CLUB_COMP_HISTORY_TABLE as *const usize);
    while p > t {
        p -= 0x1a;
        if comparar(chave.as_ptr(), p) != 0 {
            p += 0x1a;
            break;
        }
    }
    read_usize(p + [0xa, 0xe, 0x12][lugar as usize]) // campeão, vice, terceiro
}


/// rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition
/// Original: `comp_util_read_fixture_from_news_slots` (0x4b0010, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b0010 } (cdecl, comp_util.cpp): a partida (`partida`) refeita de 18 vagas seguidas da
/// `noticia` (0x785830) a partir de `*cursor` (que avança 18): +0xc e +0x10 os clubes, +0x3d, +0x3e,
/// +4 o estádio, +0 a competição, +0x42, +0x32, +0x30, +0x43/+0x44, +0x4b/+0x4c e +0x47/+0x48 os
/// placares, +0x2a, +0x28 e +8. Depois os ponteiros: clubes (0xae23b4, 0x245 bytes) em +0x1c/+0x20,
/// estádio (0xae23b0, 0x4e) em +0x18 e competição (0xae23d0, 0x6b) em +0x14; id −1: 0.
pub(crate) unsafe extern "C" fn comp_util_read_fixture_from_news_slots(noticia: usize, partida: usize, cursor: *mut u8) {
    // simplification: a original empilha o eax inteiro (resto da vaga anterior nos bits altos); a 0x785830 só lê o byte
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let mut v = *cursor;
    let mut ler = || {
        let r = vaga(noticia, v as u32);
        v = v.wrapping_add(1);
        r
    };
    let d = |o: usize, x: u32| core::ptr::write_unaligned((partida + o) as *mut u32, x);
    let w = |o: usize, x: u32| core::ptr::write_unaligned((partida + o) as *mut u16, x as u16);
    let b = |o: usize, x: u32| *((partida + o) as *mut u8) = x as u8;
    d(0xc, ler());
    d(0x10, ler());
    b(0x3d, ler());
    b(0x3e, ler());
    d(4, ler());
    d(0, ler());
    b(0x42, ler());
    w(0x32, ler());
    w(0x30, ler());
    b(0x43, ler());
    b(0x44, ler());
    b(0x4b, ler());
    b(0x4c, ler());
    b(0x47, ler());
    b(0x48, ler());
    w(0x2a, ler());
    w(0x28, ler());
    d(8, ler());
    *cursor = v;
    let id = |o: usize| core::ptr::read_unaligned((partida + o) as *const u32);
    let ponteiro = |o: usize, tabela: usize, tamanho: u32| {
        let i = id(o);
        if i == 0xffff_ffff { 0 } else { (*(tabela as *const u32)).wrapping_add(i.wrapping_mul(tamanho)) }
    };
    d(0x1c, ponteiro(0xc, G_CLUB_TABLE, 0x245)); // clube da casa
    d(0x20, ponteiro(0x10, G_CLUB_TABLE, 0x245));
    d(0x18, ponteiro(4, G_STADIUM_TABLE, 0x4e));
    d(0x14, ponteiro(0, G_CLUB_COMP_TABLE, 0x6b));
}

/// the category of the cup tie result for the club, from the final margin, the side and the reputation gap
/// Original: `comp_util_classify_tie_result_for_news` (0x4b0890, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b0890 } (cdecl, comp_util.cpp): a categoria (1 a 7) do resultado da eliminatória para o
/// `clube` na `partida`: a margem final (placar +0x4b/+0x4c, ou +0x47/+0x48, ou +0x43/+0x44, o
/// primeiro sem 0xff) do lado do clube (da casa = +0x1c) e as reputações (+0x80 / 500) dos clubes.
/// 1 goleada, 2 e 3 vitórias contra mais fortes, 4 comum, 5 e 7 derrotas para mais fracos, 6
/// zebra. Só o al é fiel (quem chama usa al).
pub(crate) unsafe extern "C" fn comp_util_classify_tie_result_for_news(clube: usize, partida: usize) -> u8 {
    let casa = *((partida + 0x1c) as *const usize);
    let fora = *((partida + 0x20) as *const usize);
    let h = (core::ptr::read_unaligned((casa + 0x80) as *const i16) as i32 / 500) as i8 as i32;
    let a = (core::ptr::read_unaligned((fora + 0x80) as *const i16) as i32 / 500) as i8 as i32;
    let p = |o: usize| *((partida + o) as *const u8);
    let (g1, g2) = if p(0x4b) != 0xff {
        (p(0x4b), p(0x4c))
    } else if p(0x47) != 0xff {
        (p(0x47), p(0x48))
    } else {
        (p(0x43), p(0x44))
    };
    let m = g1.wrapping_sub(g2) as i8;
    // do ponto de vista do clube: `eu` e `ele` as reputações, `vitoria` a margem a favor
    let (eu, ele) = if clube == casa { (h, a) } else { (a, h) };
    let vitoria = if clube == casa { m } else { m.wrapping_neg() };
    if vitoria == 0 {
        if eu < ele - 16 || eu - 15 > ele {
            return 6;
        }
        if eu < ele - 11 {
            return 2;
        }
        if eu < ele - 3 {
            return 3;
        }
        if eu - 5 > ele {
            return 5;
        }
        return if eu - 11 <= ele { 4 } else { 7 };
    }
    // a zebra: a casa tolera 8 / 7, o visitante 13 / 7 de diferença na vitória por 1 ou 2
    let (z1, z2) = if clube == casa { (8, 2) } else { (13, 7) };
    if vitoria > 0 {
        if (vitoria < 3 && eu < ele - z1) || (vitoria > 2 && eu < ele - z2) {
            return 6;
        }
        if vitoria > 4 {
            return 1;
        }
        if vitoria < 2 {
            if eu < ele - 3 {
                return 2;
            }
            if eu < ele {
                return 3;
            }
        } else if vitoria > 2 && eu < ele {
            return 2;
        }
        return 4;
    }
    let d = vitoria.wrapping_neg();
    if (d < 3 && ele < eu - 7) || (d > 2 && ele < eu - 2) {
        return 6;
    }
    if d < 2 {
        if eu - 6 > ele {
            return 7;
        }
        if eu - 2 > ele {
            return 5;
        }
    }
    4 // sem destaque
}




/// writes the fixture clubs, venue, competition, score and round into 18 consecutive news slots and advances the cursor
/// Original: `comp_util_write_fixture_to_news_slots` (0x4afeb0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afeb0 } (cdecl, comp_util.cpp; a convenção do mapa dá o 2º argumento como sem uso, mas é a
/// notícia): grava o jogo em 18 dados seguidos da notícia (0x785790), a partir do dado [cursor], e
/// avança o cursor 18. Ordem: +0xc, +0x10, +0x3d, +0x3e, +4, +0, +0x42, +0x32, +0x30, +0x43, +0x44,
/// +0x4b, +0x4c, +0x47, +0x48, +0x2a, +0x28, +8 (bytes e palavras +0x30/+0x32 com sinal; +0x28 e
/// +0x2a sem). Devolve o que a última gravação devolveu. A original guarda o índice do 18º dado no
/// byte baixo do próprio argumento `cursor` (só a parte baixa chega à 0x785790).
pub(crate) unsafe extern "C" fn comp_util_write_fixture_to_news_slots(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let d = |o: usize| core::ptr::read_unaligned((jogo + o) as *const u32);
    let b = |o: usize| byte(jogo, o) as i32 as u32;
    let s = |o: usize| core::ptr::read_unaligned((jogo + o) as *const i16) as i32 as u32;
    let z = |o: usize| core::ptr::read_unaligned((jogo + o) as *const u16) as u32;
    let valores = [
        d(0xc), d(0x10), b(0x3d), b(0x3e), d(4), d(0), b(0x42), s(0x32), s(0x30),
        b(0x43), b(0x44), b(0x4b), b(0x4c), b(0x47), b(0x48), z(0x2a), z(0x28), d(8),
    ];
    let mut k = *cursor;
    let mut r = 0;
    for v in valores {
        r = gravar(noticia, k as i8, v);
        k = k.wrapping_add(1);
    }
    *cursor = k;
    r
}


/// writes the stage and round names (with their genders, through template 0x9a7cfc), or just one of them; without an
/// output or under 200 bytes, the error box and 0
/// Original: `comp_util_format_stage_round_name` (0x4bffc0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4bffc0 } (cdecl, comp_util.cpp): `comp_util_format_stage_round_name(fase, rodada, saida, tamanho, estilo)`:
/// sem saída ou com menos de 200 bytes, a caixa (linha 0x26a1) e 0. Zera a saída; com fase e rodada, "{fase} {rodada}"
/// (0x66f4e0 com o modelo 0x9a7cfc, os nomes de 100 bytes da 0x4b4850 e os gêneros da 0x4ba8f0); só uma delas, o nome
/// dela direto na saída (0x4b4850 com o tamanho); nenhuma, só o zero. Devolve 1 (0 na caixa).
pub(crate) unsafe extern "C" fn comp_util_format_stage_round_name(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    if saida.is_null() || tamanho < 0xc8 {
        return original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(IDX_COMP_UTIL_FORMAT_STAGE_ROUND_NAME)(fase, rodada, saida, tamanho, estilo); // a caixa
    }
    // 0x4b4850: 4 pushes (estilo, id, tamanho, saída); add esp -> cdecl; 0x4ba8f0: push estilo; push id -> cdecl
    let nome: unsafe extern "C" fn(*mut u8, i32, u32, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_STAGE_NAME);
    let genero: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_STAGE_NAME_GENDER);
    *saida = 0;
    if fase != 0 && rodada != 0 {
        let (mut a, mut b) = ([0u8; 0x64], [0u8; 0x64]);
        nome(a.as_mut_ptr(), 0x64, fase, estilo);
        let ga = genero(fase, estilo) as i8 as i32 as u32;
        nome(b.as_mut_ptr(), 0x64, rodada, estilo);
        let gb = genero(rodada, estilo) as i8 as i32 as u32;
        // 0x66f4e0: 8 pushes; add esp, 0x20 -> cdecl (saída, modelo, ga, ga, gb, gb, &a, &b)
        let texto: unsafe extern "C" fn(*mut u8, usize, u32, u32, u32, u32, *const u8, *const u8) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        texto(saida, 0x9a7cfc, ga, ga, gb, gb, a.as_ptr(), b.as_ptr());
    } else if fase != 0 {
        nome(saida, tamanho, fase, estilo);
    } else if rodada != 0 {
        nome(saida, tamanho, rodada, estilo);
    } else {
        return 0; // nenhuma das duas: 0 (0x4c0136)
    }
    1
}


/// the round value between two limits: the average with a single round, otherwise (high − low) × round / (rounds − 1);
/// outside 0..100, the error box
/// Original: `comp_util_scale_by_round` (0x4bf850, comp_util.cpp, cdecl).
///
/// [pt] { 0x4bf850 } (cdecl, comp_util.cpp): `comp_util_scale_by_round(baixo, alto, rodada, rodadas)` (bytes e palavras com
/// sinal; as palavras chegam em registradores com lixo acima, só a palavra é lida): com uma rodada só, a média
/// (baixo + alto) / 2; senão (alto − baixo) × rodada / (rodadas − 1). Fora de 0..=100: a original (caixa da linha
/// 0x256e, devolve 0 ou 100). Devolve o eax inteiro da original (quem chama empurra o registrador todo para
/// 0x86bdd0/0x86c000): o byte em al sobre a soma (rodada única) ou o quociente.
pub(crate) unsafe extern "C" fn comp_util_scale_by_round(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let (eax, v) = interpolate_by_round(baixo, alto, rodada, rodadas);
    if !(0..=100).contains(&v) {
        return original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(IDX_COMP_UTIL_SCALE_BY_ROUND)(baixo, alto, rodada, rodadas);
    }
    if rodadas as u16 != 1 {
        crate::comparacao::ramo("cpq_11 escala pela rodada");
    }
    eax
}


/// stores competition, year and four values in the free row of the competition history (without one, grows 20 rows by
/// 0x527f60); returns the row
/// Original: `comp_util_add_history_record` (0x4afce0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afce0 } (cdecl, comp_util.cpp): `comp_util_add_history_record(liga, primeiro, segundo, terceiro, extra)`:
/// a linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60 e usa a primeira nova; sem
/// memória, 0) recebe a competição (+4 da liga), o ano [0xae2c92] (+8), e os quatro valores em +0xa, +0xe, +0x12,
/// +0x16. Devolve a linha. Sem SEH, sem sorteio, sem locais.
pub(crate) unsafe extern "C" fn comp_util_add_history_record(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let mut r = free_comp_history_row();
    if r == 0 {
        crate::comparacao::ramo("histórico de competições crescer");
        let crescer: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_DATABASE_GROW_COMP_HISTORY_TABLE);
        if crescer(0x14) == 0 {
            return 0;
        }
        let n = *(G_CLUB_COMP_HISTORY_COUNT as *const i32);
        r = (*(G_CLUB_COMP_HISTORY_TABLE as *const usize)).wrapping_add(26usize.wrapping_mul(n.wrapping_sub(20) as usize));
    }
    core::ptr::write_unaligned((r + 4) as *mut usize, read_usize(liga + 4));
    core::ptr::write_unaligned((r + 8) as *mut u16, *(G_CURRENT_DATE_YEAR as *const u16)); // o ano
    for (k, v) in [a, b, c, d].into_iter().enumerate() {
        core::ptr::write_unaligned((r + 0xa + 4 * k) as *mut u32, v);
    }
    r
}


/// the group (the containing stage) of a stage or round by id, by the fixed ranges; −1 without a group
/// Original: `comp_util_get_parent_stage_id` (0x4b01d0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b01d0 } (cdecl, comp_util.cpp): `comp_util_get_parent_stage_id(id)`, o grupo de uma fase pelo id: 0x3e8..0x3fb,
/// 0x41f..0x42e, 0x413, 0x414, 0x47b e 0x3fc -> 0x41c; 0x473..0x474 -> 0x410; 0x475..0x479 -> 0x47a; 0x3fd..0x406 ->
/// 0xa; 0x407..0x40b e 0x472 -> 0x14; 0x47e..0x47f -> 0x1e; 0x40c..0x40e -> 0x82; 0x43d..0x444 -> 0x449; 0x445..0x448
/// -> 0x44a; 0x44d..0x450 -> 0x451; o resto −1 (na ordem dos testes da original).
pub(crate) unsafe extern "C" fn comp_util_get_parent_stage_id(id: i32) -> i32 {
    let em = |a: i32, b: i32| id >= a && id <= b;
    if em(0x3e8, 0x3fb) {
        return 0x41c;
    }
    if em(0x473, 0x474) {
        return 0x410;
    }
    if em(0x475, 0x479) {
        return 0x47a;
    }
    if em(0x3fd, 0x406) {
        crate::comparacao::ramo("cqg_6 grupo 0xa");
        return 0xa; // cqg6: o grupo das fases 0x3fd..0x406
    }
    if em(0x407, 0x40b) || id == 0x472 {
        return 0x14;
    }
    if em(0x47e, 0x47f) {
        return 0x1e;
    }
    if em(0x41f, 0x42e) {
        return 0x41c;
    }
    if em(0x40c, 0x40e) {
        return 0x82;
    }
    if id == 0x413 || id == 0x414 || id == 0x47b || id == 0x3fc {
        return 0x41c;
    }
    if em(0x43d, 0x444) {
        return 0x449;
    }
    if em(0x445, 0x448) {
        return 0x44a;
    }
    if em(0x44d, 0x450) {
        return 0x451;
    }
    -1
}




