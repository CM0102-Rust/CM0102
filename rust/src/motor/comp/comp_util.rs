use super::*;

// ---------- septuagesima oitava leva: competicao com rodada a disputar, comparacoes de ordenacao (comp_util.cpp) ----------

/// { 0x4b0700 } (comp_util.cpp): a competicao `id` conta? As tres de 0x9cf6b0, 0x9cf5d8 e 0x9cf548
/// sim; sem o +0x5d do registro (0xae23d0, 107 bytes) nao; senao so a do objeto em 0xadadfc com
/// fase 2 (+0x43), rodadas (+0x3e) e a primeira rodada (+0xb1) com o clube +4 em +0x57 ou +0x60.
pub(crate) unsafe extern "C" fn competicao_conta(id: i32) -> u32 {
    let g = |a: usize| *(a as *const i32);
    if id == g(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) || id == g(KEY_CLUB_COMP_FRENCH_CFA) || id == g(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) {
        return 1;
    }
    if *((*(0xae23d0 as *const usize)).wrapping_add((id as usize).wrapping_mul(107)).wrapping_add(0x5d) as *const u32) == 0 {
        return 0;
    }
    let c = *((*(0xadadfc as *const usize)).wrapping_add((id as usize).wrapping_mul(4)) as *const usize);
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

/// { 0x4b0780 } (comparacao do qsort): byte +0x5f do segundo menos o do primeiro (nulo vale 0).
pub(crate) unsafe extern "C" fn ordem_por_5f(a: *const usize, b: *const usize) -> i32 {
    let v = |p: usize| if p == 0 { 0 } else { *((p + 0x5f) as *const i8) as i32 };
    v(*b) - v(*a)
}

/// { 0x4b07b0 } (comparacao do qsort): palavra +0x80 do segundo menos a do primeiro.
pub(crate) unsafe extern "C" fn ordem_por_80(a: *const usize, b: *const usize) -> i32 {
    *((*b + 0x80) as *const i16) as i32 - *((*a + 0x80) as *const i16) as i32
}

/// { 0x4b07d0 } (comparacao do qsort): como 0x4b07b0, com os nulos no fim (primeiro nulo: 1 se o
/// segundo nao for; segundo nulo: -1).
pub(crate) unsafe extern "C" fn ordem_por_80_desc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    *((y + 0x80) as *const i16) as i32 - *((x + 0x80) as *const i16) as i32
}

/// { 0x4b0810 } (comparacao do qsort): como 0x4b07d0, em ordem crescente.
pub(crate) unsafe extern "C" fn ordem_por_80_cresc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    *((x + 0x80) as *const i16) as i32 - *((y + 0x80) as *const i16) as i32
}

// ---------- quadragesima oitava leva: folhas de outros modulos ----------

/// { 0x4b0850 } (comp_util.cpp): algarismo das unidades de `n` (abaixo de 1000; senao 0).
pub(crate) unsafe extern "C" fn unidades(n: i32) -> u8 {
    if n < 1000 {
        (n % 10) as u8
    } else {
        0
    }
}

/// { 0x5c6710 } (friendly.cpp): a competicao e amistosa? E a guardada em +4 do objeto, ou o id dela
/// (+0) esta entre as 127 ultimas competicoes de clubes (quantidade em 0xae2378).
pub(crate) unsafe extern "thiscall" fn e_amistoso(this: usize, comp: usize) -> u32 {
    if comp == *((this + 4) as *const usize) {
        return 1;
    }
    (*(comp as *const i32) >= (*(0xae2378 as *const i32)).wrapping_sub(0x7f)) as u32
}

/// { 0x4cf5a0 } (contract_manager.cpp): contrato da pessoa (registros de 80 bytes a partir de +0
/// do objeto), pelo indice na tabela 0xae2d0c (79 bytes por pessoa; quantidade em 0xae2364).
/// Sem pessoa, id fora da tabela ou indice negativo: 0.
pub(crate) unsafe extern "thiscall" fn contrato_de(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let tabela = *(0xae2d0c as *const usize);
    let i = *((tabela as isize + 79 * id as isize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(this as *const usize)).wrapping_add((80 * i) as usize)
}

/// { 0x53b900 } (Database.cpp): o clube e uma selecao? Os ids das selecoes vem depois dos clubes
/// (total de clubes em 0xae235c menos 2 x nacoes em 0xae2350). Clube nulo: o original mostra uma
/// caixa de erro, zera 0xb67a34 e devolve 0 (aqui, sem a caixa).
pub(crate) unsafe extern "C" fn e_selecao(clube: usize) -> u32 {
    if clube == 0 {
        *(0xb67a34 as *mut u32) = 0;
        return 0;
    }
    let limite = (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2));
    (*(clube as *const i32) >= limite) as u32
}

// ---------- 256a leva: as posições previstas da copa: as posições pela rodada de saída, ordenar os clubes pela força e gravar a posição do clube na estatística (Cup.cpp, comp_util.cpp, comp_stats.cpp) ----------



/// O núcleo da 0x4c08f0 sobre os `n` clubes em `v` e as chaves em `k`: a chave de cada clube é a força
/// (0x4c0720; clube nulo, 0); depois, para cada i, todo j > i com chave maior troca de lugar com i (chave e clube),
/// com sinal.
pub(crate) unsafe fn cpt13_ordenar_com(v: *mut u32, k: *mut i32, n: i32) {
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
pub(crate) unsafe extern "C" fn ordenar_clubes_pela_forca(v: usize, n: i32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let k = alocar((n as u32).wrapping_mul(4));
    if k == 0 {
        return aviso_em(0x99ec20, 0x292d);
    }
    cpt13_ordenar_com(v as *mut u32, k as *mut i32, n);
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
pub(crate) unsafe extern "thiscall" fn gravar_posicao_do_clube(this: usize, clube: usize, pos: u32) -> usize {
    if clube == 0 {
        aviso_em(0x99ca94, 0x680);
        return 0; // ponytail: a original devolve o eax da caixa; quem chama não lê
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

/// As chamadas da 0x5223a0 à 0x4a2540 (clube, posição), na ordem, com o vetor `v` já ordenado: da penúltima rodada
/// (+0x3c − 2) para trás ([+0xa3] + r × 0x68, a conta de quem sai em +0x1a), até um clube nulo no começo da rodada.
/// Na primeira rodada vista, os que saem nela ganham todos a posição 1 (sem olhar nulos); em cada rodada, os seguintes
/// ganham a posição do contador (que sobe a cada vaga, nula ou não), pulando os nulos. `gravar` = 0 só monta a lista.
pub(crate) unsafe fn cpt13_posicoes(this: usize, v: *const u32, gravar: usize) -> Vec<(usize, u8)> {
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

/// sorts the cup clubs by strength and gives each one the predicted position of the round it goes out in, in the cup
/// statistics
/// Original: `Cup::rank_teams` (0x5223a0, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5223a0 } (thiscall, ret 0, Cup.cpp): `Cup::rank_teams()`: copia os clubes da copa ([+0xa7], registros de 6
/// bytes, +0x3e deles) para um vetor (0x944e46), ordena pela força (0x4c08f0) e dá a cada um a posição prevista pela
/// rodada em que sai (0x4a2540 na estatística [+8]); libera o vetor (0x9452ca). Sem estatística: a caixa (linha
/// 0x1827); sem memória: a original (a caixa da falta de memória e o fim do jogo).
pub(crate) unsafe extern "thiscall" fn posicoes_previstas_da_copa(this: usize) {
    if read_u32_as_usize(this + 8) == 0 {
        return aviso_em(0x9b55ac, 0x1827);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    let n = read_i16(this + 0x3e) as i32;
    let v = alocar((n << 2) as u32);
    if v == 0 {
        return orig::<unsafe extern "thiscall" fn(usize)>(1191)(this);
    }
    for i in 0..n.max(0) as usize {
        *((v + i * 4) as *mut u32) = read_u32_as_usize(read_u32_as_usize(this + 0xa7) + i * 6) as u32;
    }
    ordenar(v, n);
    cpt13_posicoes(this, v as *const u32, 1);
    liberar(v);
}

/// Comparador: grava e pode realocar a estatística (não roda duas vezes): antes da original, as posições são
/// planejadas sobre uma cópia ordenada do vetor (as chaves num Vec); depois, o registro de cada clube planejado tem
/// de ter em +0x72 a última posição dele.
pub(crate) unsafe extern "thiscall" fn comparar_posicoes_previstas_da_copa(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1191);
    let n = read_i16(this + 0x3e) as i32;
    if read_u32_as_usize(this + 8) == 0 || n <= 0 {
        return o(this); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0xa7) + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // ponytail: com mais vagas nas rodadas que clubes, a original lê além do vetor (sobra do bloco); o plano lê 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plano = cpt13_posicoes(this, v.as_ptr(), 0);
    o(this);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut erros = Vec::new();
    for (i, &(c, p)) in plano.iter().enumerate() {
        if c == 0 || plano[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = buscar(read_u32_as_usize(this + 8), c);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("posicoes_previstas_da_copa({:x}) {} posições: {}", this, plano.len(), erros.join("; ")));
}

/// Comparador: ordena no lugar (aloca e libera o temporário): o esperado sai do núcleo sobre uma cópia (chaves num
/// Vec) antes da original; depois, o vetor tem de ser igual.
pub(crate) unsafe extern "C" fn comparar_ordenar_clubes_pela_forca(v: usize, n: i32) {
    let o = orig::<unsafe extern "C" fn(usize, i32)>(1192);
    if v == 0 || n <= 1 {
        return o(v, n);
    }
    let mut esperado = core::slice::from_raw_parts(v as *const u32, n as usize).to_vec();
    let mut k = vec![0i32; n as usize];
    cpt13_ordenar_com(esperado.as_mut_ptr(), k.as_mut_ptr(), n);
    o(v, n);
    let obtido = core::slice::from_raw_parts(v as *const u32, n as usize);
    PLACAR.registrar(obtido == &esperado[..], || format!("ordenar_clubes_pela_forca(n {}) original={:x?} rust={:x?}", n, obtido, esperado));
}

/// Comparador: acha ou acrescenta o registro (não roda duas vezes): o registro visto antes (se havia) tem de ser o
/// devolvido, com a posição em +0x72.
pub(crate) unsafe extern "thiscall" fn comparar_gravar_posicao_do_clube(this: usize, clube: usize, pos: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1193);
    if clube == 0 {
        return o(this, clube, pos); // ponytail: a caixa, só a original
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let antes = buscar(this, clube);
    let r = o(this, clube, pos);
    let igual = r != 0 && (antes == 0 || r == antes) && *((r + 0x72) as *const u8) == pos as u8;
    PLACAR.registrar(igual, || format!("gravar_posicao_do_clube(clube {:x}, {}) antes={:x} original={:x}", clube, pos as u8, antes, r));
    r
}

// ---------- 253a leva: o lado do jogo, a ordem dos coeficientes das nações e o pedido de observação (comp_util.cpp, scout_manager.cpp) ----------


pub(crate) unsafe fn cpq12_us(a: usize) -> usize {
    read_u32(a) as usize
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
pub(crate) unsafe extern "C" fn lado_do_jogo_para_o_clube(clube: usize, jogo: usize) -> u8 {
    let campo = read_u32(jogo + 0x18);
    let casa = cpq12_us(jogo + 0x1c);
    if campo == 0 {
        if read_u32(casa + 0x69) == 0 {
            return 3;
        }
        return if clube != casa { 2 } else { 1 };
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        let nacao = cpq12_us(campo as usize + 0x38);
        if nacao != 0 && read_u32(nacao + 0x1f) == read_u32(clube + 0x53) {
            return 1;
        }
        let casa = cpq12_us(jogo + 0x1c);
        let outro = if casa == clube { cpq12_us(jogo + 0x20) } else { casa };
        if nacao != 0 && read_u32(nacao + 0x1f) == read_u32(outro + 0x53) {
            return 2;
        }
        return 3;
    }
    let casa = cpq12_us(jogo + 0x1c);
    let outro = if casa == clube { cpq12_us(jogo + 0x20) } else { casa };
    let campo = read_u32(jogo + 0x18);
    if campo != read_u32(clube + 0x69) {
        return if campo != read_u32(outro + 0x69) { 3 } else { 2 };
    }
    if campo == read_u32(outro + 0x69) {
        return if clube != casa { 2 } else { 1 }; // o mesmo estádio dos dois
    }
    crate::comparacao::ramo("cpq_12 em casa");
    1 // o estádio do clube
}

/// Comparador: pura (consulta 0x53b900, sem efeitos): pelo lado.
pub(crate) unsafe extern "C" fn comparar_lado_do_jogo_para_o_clube(clube: usize, jogo: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u8>(1078);
    let r = o(clube, jogo);
    let n = lado_do_jogo_para_o_clube(clube, jogo);
    PLACAR.registrar(r == n, || format!("lado_do_jogo_para_o_clube(clube {:x}, jogo {:x}) original={} rust={}", clube, jogo, r, n));
    r
}

/// the qsort order of the nation coefficients: highest first
/// Original: `compare_nation_coefficients_desc` (0x4afba0, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afba0 } (cdecl, comp_util.cpp): `compare_nation_coefficients_desc(a, b)`: a ordem do qsort (12 bytes por
/// entrada, o coeficiente em double em +4) da game_update_nation_coefficients 0x5d6187 (no dia do mês da tabela
/// 0x9cd200): maior antes (−1); menor ou sem ordem (NaN, a flag C0 do fcomp), 1; igual, 0. Quente (o qsort): sem
/// contador. A original ordena e libera o vetor logo depois: a ordem não chega ao jogo (só o comparando acusa).
pub(crate) unsafe extern "C" fn ordem_dos_coeficientes_das_nacoes(a: usize, b: usize) -> i32 {
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

pub(crate) unsafe extern "C" fn comparar_ordem_dos_coeficientes_das_nacoes(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1079);
    let r = o(a, b);
    let n = ordem_dos_coeficientes_das_nacoes(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_coeficientes_das_nacoes original={} rust={}", r, n));
    r
}

/// whether the club (or the current human's) asked to scout the player, with fewer than 3 reports and, if asked, with
/// the date already due
/// Original: `SCOUT_MANAGER::has_scout_request` (0x7f8d70, scout_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7f8d70 } (thiscall, ret 0xc, scout_manager.cpp): `SCOUT_MANAGER::has_scout_request(jogador, clube, so_vencidos)`:
/// se o clube (nulo: o do humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]] +0x39) pediu a observação do jogador: busca
/// binária (0x946464 com 0x7fc270) pelo par (id do clube, id do jogador) nos pedidos (+0x5c, conta +0x64, 17 bytes)
/// com menos de 3 relatórios (+0x10, com sinal); com `so_vencidos`, só se a data do pedido (+8: dia, +0xa: ano) é
/// anterior a hoje ([0xae2c90], ano sem sinal, dia com sinal). Devolve 1 ou 0. Jogador nulo: a original (caixa da
/// linha 0x5da). A data local da original (0x54a110) só ocupa a chave depois do par.
pub(crate) unsafe extern "thiscall" fn pedido_de_observacao(this: usize, jogador: usize, clube: usize, so_vencidos: u32) -> u32 {
    if jogador == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1080)(this, jogador, clube, so_vencidos);
    }
    let mut clube = clube;
    if clube == 0 {
        let humano = cpq12_us(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize));
        if humano == 0 {
            return 0;
        }
        clube = cpq12_us(humano + 0x39);
        if clube == 0 {
            return 0;
        }
    }
    let lista = cpq12_us(this + 0x5c);
    let achado = if lista == 0 {
        0
    } else {
        let data: unsafe extern "thiscall" fn(*mut u8) -> *mut u8 = core::mem::transmute(FN_DATE_DATE_54A110);
        let buscar: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        let mut chave = [0u8; 16];
        data(chave.as_mut_ptr().add(8));
        chave[4..8].copy_from_slice(&read_u32(jogador).to_le_bytes());
        chave[0..4].copy_from_slice(&read_u32(clube).to_le_bytes());
        buscar(chave.as_ptr(), lista, read_u32(this + 0x64), 0x11, 0x7fc270)
    };
    if achado == 0 {
        crate::comparacao::ramo("cpq_12 sem pedido");
        return 0; // sem pedido
    }
    if *((achado + 0x10) as *const i8) >= 3 {
        return 0;
    }
    if so_vencidos != 0 {
        let ano = core::ptr::read_unaligned((achado + 0xa) as *const u16);
        let hoje = *(0xae2c92 as *const u16);
        if ano > hoje {
            return 0;
        }
        if ano == hoje && core::ptr::read_unaligned((achado + 8) as *const i16) >= *(0xae2c90 as *const i16) {
            return 0;
        }
    }
    1
}

/// Comparador: consulta pura (a busca binária e a data local): pelo valor; jogador nulo só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_pedido_de_observacao(this: usize, jogador: usize, clube: usize, so_vencidos: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1080);
    if jogador == 0 {
        return o(this, jogador, clube, so_vencidos);
    }
    let r = o(this, jogador, clube, so_vencidos);
    let n = pedido_de_observacao(this, jogador, clube, so_vencidos);
    PLACAR.registrar(r == n, || format!("pedido_de_observacao(jogador {:x}, clube {:x}, {}) original={} rust={}", jogador, clube, so_vencidos, r, n));
    r
}

// ---------- 249a leva: a mudança de divisão do clube e os embaralhamentos das listas de clubes (league.cpp, comp_util.cpp) ----------


pub(crate) unsafe fn cpm3_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// the current division becomes last season's and the new one is set; clears the division clauses (and the squad
/// numbers when the new one uses them), opens the requested manager vacancy, upgrades the stadium to the league minimum
/// and turns status 3 into 2
/// Original: `League::move_club_to_division` (0x6830b0, league.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x6830b0 } (thiscall, ret 0xc, league.cpp; `this` não é lido): `League::move_club_to_division(clube, divisao,
/// vaga)`: a divisão atual (+0x57) vira a da última temporada (+0x5b) e a nova entra; as cláusulas de divisão dos
/// jogadores são limpas (0x4cd120 em [0xae1050]), com os números da camisa também se a divisão velha não os usava e a
/// nova usa (0x85add0 em [0xdd7ef0], antes e depois); com `vaga` e sem técnico (+0xcf), a vaga de técnico (0x69ba50 em
/// [0xb67a44]); numa divisão de liga (+0x43 = 2 na competição [0xadadfc][id]) com estádio mínimo (+0xe2 ≠ −1) e ficha
/// do estádio (+0x69), a obra do estádio (0x599e00 nas finanças 0x5a0590 de [0xae2a38], lugares +0xe4 e +0xe2); e,
/// com a competição carregada, a situação +0x64 = 3 passa a 2.
pub(crate) unsafe extern "thiscall" fn mudar_clube_de_divisao(_this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("cpm_3 mudar de divisao");
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    let limpar_clausulas: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_RESET_DIVISION_CLAUSES);
    let antes = usa_numeros(read_usize(0xdd7ef0), clube, 0);
    cpm3_por(clube + 0x5b, read_usize(clube + 0x57)); // a divisão da última temporada
    cpm3_por(clube + 0x57, divisao);
    let numeros = (antes == 0 && usa_numeros(read_usize(0xdd7ef0), clube, 0) != 0) as u32;
    limpar_clausulas(read_usize(0xae1050), clube, numeros);
    if vaga != 0 && read_usize(clube + 0xcf) == 0 {
        let abrir_vaga: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_ADD_JOB_VACANCY);
        abrir_vaga(read_usize(0xb67a44), clube, 0);
    }
    if divisao == 0 {
        return;
    }
    let comps = read_usize(0xadadfc);
    let id = read_usize(divisao);
    if comps != 0 {
        let comp = read_usize(comps + 4 * id);
        if comp != 0 && read_usize(clube + 0x69) != 0 && *((comp + 0x43) as *const u8) == 2 {
            let lugares = core::ptr::read_unaligned((comp + 0xe2) as *const i16);
            if lugares != -1 {
                crate::comparacao::ramo("cpm_3 estadio da divisao");
                let capacidade = core::ptr::read_unaligned((comp + 0xe4) as *const i16);
                let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
                let obra: unsafe extern "thiscall" fn(usize, i32, i32, usize, u32) -> u32 = core::mem::transmute(FN_FINANCE_UPGRADE_STADIUM_FOR_LEAGUE);
                let f = financas(read_usize(0xae2a38), clube);
                obra(f, capacidade as i32, lugares as i32, id, 0); // a obra do estádio
            }
        }
    }
    // a original relê [0xadadfc] depois da obra (sem a obra, o mesmo valor; nulo, ela lê do endereço 0 também)
    if read_usize(read_usize(0xadadfc) + 4 * id) != 0 && *((clube + 0x64) as *const u8) == 3 {
        *((clube + 0x64) as *mut u8) = 2;
    }
}

pub(crate) unsafe fn cpm3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// As trocas de `vezes` pares sorteados (0x90cfc0(n) duas vezes; a primeira dá o índice que recebe o item do segundo)
/// no vetor de ponteiros `itens`, como as duas originais (índices de 16 bits com sinal).
pub(crate) unsafe fn cpm3_trocar(itens: usize, n: i32) {
    let a = cpm3_sorteio(n) as i16 as isize;
    let b = cpm3_sorteio(n) as i16 as isize;
    let pa = (itens as isize + 4 * a) as usize;
    let pb = (itens as isize + 4 * b) as usize;
    let (va, vb) = (read_usize(pa), read_usize(pb));
    cpm3_por(pa, vb);
    cpm3_por(pb, va);
}

/// shuffles an array of n pointers with 3 × n swaps of drawn pairs
/// Original: `comp_util_shuffle_pointer_array` (0x4afb40, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afb40 } (cdecl, comp_util.cpp): `comp_util_shuffle_pointer_array(itens, n)`: embaralha o vetor de `n` (curto)
/// ponteiros com 3 × n trocas de pares sorteados (0x90cfc0(n) duas vezes cada).
pub(crate) unsafe extern "C" fn embaralhar_ponteiros(itens: usize, n: u32) {
    let n = n as u16 as i16 as i32;
    if 3 * n <= 0 {
        return;
    }
    crate::comparacao::ramo("cpm_3 embaralhar ponteiros");
    let mut k = 0i16;
    loop {
        cpm3_trocar(itens, n);
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
pub(crate) unsafe extern "C" fn ordenar_e_embaralhar_os_melhores(clubes: usize, n: u32, escolhas: u32) {
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return orig::<unsafe extern "C" fn(usize, u32, u32)>(933)(clubes, n, escolhas); // a caixa
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
        cpm3_trocar(clubes, m as i32);
    }
}

/// Comparador: a mudança chama as notícias, a vaga de técnico e a obra (não podem rodar duas vezes): só a original (a
/// prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_mudar_clube_de_divisao(this: usize, clube: usize, divisao: usize, vaga: u32) {
    crate::comparacao::ramo("mudar_clube_de_divisao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(931)(this, clube, divisao, vaga)
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pelo vetor, a cópia em sem_contar.
pub(crate) unsafe extern "C" fn comparar_embaralhar_ponteiros(itens: usize, n: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(932);
    let k = (n as u16 as i16).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if k > 0 {
        reg.push((itens, 4 * k));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(itens, n),
        || crate::aleatorio::sem_contar(|| embaralhar_ponteiros(itens, n)));
    PLACAR.registrar(igual, || format!("embaralhar_ponteiros({}) {:x?} x {:x?}", k, mo, mr));
}

/// Comparador: ordena (qsort, determinístico) e sorteia: lado a lado pelo estado do sorteio e pelos `n` clubes, a cópia
/// em sem_contar. Caixa (`n` negativo ou menor que `escolhas`): só a original.
pub(crate) unsafe extern "C" fn comparar_ordenar_e_embaralhar_os_melhores(clubes: usize, n: u32, escolhas: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32)>(933);
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return o(clubes, n, escolhas);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if nn > 0 {
        reg.push((clubes, 4 * nn as usize));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clubes, n, escolhas),
        || crate::aleatorio::sem_contar(|| ordenar_e_embaralhar_os_melhores(clubes, n, escolhas)));
    PLACAR.registrar(igual, || format!("ordenar_e_embaralhar_os_melhores({}, {}) {:x?} x {:x?}", nn, e, mo, mr));
}

// ---------- 241a leva: os auxiliares de competição: o tipo de confronto na liga, a força do clube e o colocado mais recente (comp_util.cpp) ----------



/// A nota de um clube na estatística da liga (0x4b02e0): pesos 1 nos 4 atributos (+0x72..+0x75; +0x75 e +0x74 com
/// o peso [0x968968]) ou, com menos de 25 jogos (+4), proporcionais aos jogos; a média ponderada, na ordem do x87.
pub(crate) unsafe fn cpa3_nota(e: usize, b73: f64) -> f64 {
    let (mut p, mut k73, mut k72) = (f64_em(0x968968), 1.0f64, 1.0f64);
    let jogos = read_u8(e + 4);
    if jogos < 0x19 {
        let v = jogos as f64;
        p = v * f64_em(0x968a98);
        k73 = v * f64_em(0x968a90);
        k72 = f64_em(0x968938) - v * f64_em(0x968a88);
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
/// (cpa3_nota): 1 = briga pelo título (virtual +0x98 = 0, as duas abaixo de um terço dos clubes e cada uma não abaixo
/// do +0x74 do outro); 3 = briga na zona (+0x98 ≠ 0, as duas até 0,2 × clubes + +0xbe, ou com +0xbf até 0,2 × clubes
/// + +0xbe + +0xbf, e o mesmo cruzamento do +0x74); senão 0. Devolve só `al`. Sem SEH, sem sorteio; a virtual +0x98
/// é chamada até três vezes, como na original.
pub(crate) unsafe extern "C" fn tipo_de_confronto_na_liga(a: usize, b: usize) -> u8 {
    let comp = read_usize(a + 0x57);
    if comp == 0 || comp != read_usize(b + 0x57) {
        return 0;
    }
    let menor: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    if menor(comp) != 0 {
        return 0;
    }
    let liga = read_usize((*(0xadadfc as *const usize)).wrapping_add(4 * *(read_usize(a + 0x57) as *const u32) as usize));
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
    if (a73 - b73).abs() > ftol(clubes as f64 * f64_em(0x9689e0)) as i16 as i32 {
        return 0;
    }
    let (na, nb) = (cpa3_nota(ea, a73 as f64), cpa3_nota(eb, b73 as f64));
    let (a74, b74) = (read_u8(ea + 0x74) as f64, read_u8(eb + 0x74) as f64);
    let virtual98 = || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(liga) + 0x98));
        f(liga)
    };
    // as comparações do x87: NaN conta como "menor" (C0) e "menor ou igual" (C0|C3)
    let cruzadas = !(a74 > nb) && !(b74 > na);
    if virtual98() == 0 {
        let t = clubes as f64 * f64_em(0x968a80);
        if !(na >= t) && !(nb >= t) && cruzadas {
            return 1; // briga pelo título
        }
    }
    let be = read_u8(liga + 0xbe) as i8 as f64;
    if virtual98() != 0 {
        let t = clubes as f64 * f64_em(0x968918) + be;
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
    let t = clubes as f64 * f64_em(0x968918) + be + bf as f64;
    if !(na > t) && !(nb > t) && cruzadas {
        3
    } else {
        0
    }
}

// ponytail: ~4 milhões de chamadas na carreira: comparação só do valor, sem regiões (a rotina só lê; a 0x4a4850 pode
// reordenar a estatística, e a original já deixa ordenada para a cópia).
pub(crate) unsafe extern "C" fn comparar_tipo_de_confronto_na_liga(a: usize, b: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(780)(a, b);
    let n = tipo_de_confronto_na_liga(a, b);
    PLACAR.registrar(r == n, || format!("tipo_de_confronto_na_liga({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

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
pub(crate) unsafe extern "C" fn forca_do_clube(clube: usize) -> i32 {
    if clube == 0 {
        return -10_000_000;
    }
    let financas = *(0xae2a38 as *const usize);
    let mut forca: i32 = 0;
    if financas == 0 {
        forca = core::ptr::read_unaligned((clube + 0x65) as *const i32) / 400;
    } else {
        let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
        let f = financas_do_clube(financas, clube);
        if f != 0 {
            forca = (core::ptr::read_unaligned(f as *const i64) / 300) as i32;
        }
    }
    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 4); // a reputação
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
    let escalar: Escalar = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    escalar(jogadores.as_ptr() as usize, k as u8 as u32, codigos.as_ptr() as usize, 0xf, 0, escalados.as_mut_ptr() as usize, 0x840, clube, 0, 0, 0, 0, 0, 0, 0);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_SELECTION);
    for (j, &p) in escalados.iter().enumerate() {
        if p != 0 {
            forca = forca.wrapping_add(nota(p, codigos[j] as u32, 0x840, clube, -1, 0));
        }
    }
    forca
}

// ponytail: 1.117 chamadas; só o valor. A escalação e as notas têm comparadores próprios, e o físico que a nota
// atualiza uma vez (0x62ed70, com sorteio) a original já deixa pronto para a cópia.
pub(crate) unsafe extern "C" fn comparar_forca_do_clube(clube: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize) -> i32>(781)(clube);
    let n = forca_do_clube(clube);
    PLACAR.registrar(r == n, || format!("forca_do_clube({:x}) original={} rust={}", clube, r, n));
    r
}

/// sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) of the first row of the
/// competition; not found: 0
/// Original: `comp_util_get_latest_placed_club` (0x4afd80, comp_util.cpp, cdecl).
///
/// [pt] { 0x4afd80 } (cdecl, comp_util.cpp): `comp_util_get_latest_placed_club(comp, lugar)`: ordena o histórico de
/// competições (qsort 0x945333 com 0x540780), busca (bsearch 0x946464) a competição com ano −1 (qualquer), volta até a
/// primeira linha dela e devolve o clube do `lugar` (0 campeão +0xa, 1 vice +0xe, 2 terceiro +0x12); não achou: 0.
/// Outro lugar: a original (a caixa do comp_util.cpp). A chave na pilha da original só tem +4 e +8 gravados, e a
/// 0x540780 só lê esses dois; a cópia zera o resto.
pub(crate) unsafe extern "C" fn clube_colocado_mais_recente(comp: usize, lugar: u32) -> usize {
    let lugar = lugar as u8;
    if lugar > 2 {
        return orig::<unsafe extern "C" fn(usize, u32) -> usize>(782)(comp, lugar as u32); // caixa: só a original
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let buscar: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let comparar: unsafe extern "C" fn(*const u8, usize) -> i32 = core::mem::transmute(FN_DATABASE_COMPARE_COMP_HISTORY_ROWS);
    qsort(*(0xae23d4 as *const usize), *(0xae237c as *const i32), 0x1a, 0x540780);
    let mut chave = [0u8; 0x1a];
    chave[4..8].copy_from_slice(&comp.to_le_bytes());
    chave[8..10].copy_from_slice(&0xffffu16.to_le_bytes()); // o ano: qualquer
    let mut p = buscar(chave.as_ptr(), *(0xae23d4 as *const usize), *(0xae237c as *const i32), 0x1a, 0x540780);
    if p == 0 {
        return 0;
    }
    let t = *(0xae23d4 as *const usize);
    while p > t {
        p -= 0x1a;
        if comparar(chave.as_ptr(), p) != 0 {
            p += 0x1a;
            break;
        }
    }
    read_usize(p + [0xa, 0xe, 0x12][lugar as usize]) // campeão, vice, terceiro
}

pub(crate) unsafe extern "C" fn comparar_clube_colocado_mais_recente(comp: usize, lugar: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> usize>(782);
    if lugar as u8 > 2 {
        return o(comp, lugar);
    }
    let n = (*(0xae237c as *const i32)).max(0) as usize;
    let reg = [(*(0xae23d4 as *const usize), 0x1a * n)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(comp, lugar), || clube_colocado_mais_recente(comp, lugar));
    PLACAR.registrar(igual && r == x, || format!("clube_colocado_mais_recente({:x}, {}) {:x}/{:x} {:x?} x {:x?}", comp, lugar as u8, r, x, mo, mr));
    r
}

// ---------- 214a leva: a partida guardada na notícia, o resultado da eliminatória para a notícia e o dia da rodada da copa (comp_util.cpp, Cup.cpp) ----------

/// rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition
/// Original: `comp_util_read_fixture_from_news_slots` (0x4b0010, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b0010 } (cdecl, comp_util.cpp): a partida (`partida`) refeita de 18 vagas seguidas da
/// `noticia` (0x785830) a partir de `*cursor` (que avança 18): +0xc e +0x10 os clubes, +0x3d, +0x3e,
/// +4 o estádio, +0 a competição, +0x42, +0x32, +0x30, +0x43/+0x44, +0x4b/+0x4c e +0x47/+0x48 os
/// placares, +0x2a, +0x28 e +8. Depois os ponteiros: clubes (0xae23b4, 0x245 bytes) em +0x1c/+0x20,
/// estádio (0xae23b0, 0x4e) em +0x18 e competição (0xae23d0, 0x6b) em +0x14; id −1: 0.
pub(crate) unsafe extern "C" fn partida_das_vagas_da_noticia(noticia: usize, partida: usize, cursor: *mut u8) {
    // ponytail: a original empilha o eax inteiro (resto da vaga anterior nos bits altos); a 0x785830 só lê o byte
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
    d(0x1c, ponteiro(0xc, 0xae23b4, 0x245)); // clube da casa
    d(0x20, ponteiro(0x10, 0xae23b4, 0x245));
    d(0x18, ponteiro(4, 0xae23b0, 0x4e));
    d(0x14, ponteiro(0, 0xae23d0, 0x6b));
}

/// the category of the cup tie result for the club, from the final margin, the side and the reputation gap
/// Original: `comp_util_classify_tie_result_for_news` (0x4b0890, comp_util.cpp, cdecl).
///
/// [pt] { 0x4b0890 } (cdecl, comp_util.cpp): a categoria (1 a 7) do resultado da eliminatória para o
/// `clube` na `partida`: a margem final (placar +0x4b/+0x4c, ou +0x47/+0x48, ou +0x43/+0x44, o
/// primeiro sem 0xff) do lado do clube (da casa = +0x1c) e as reputações (+0x80 / 500) dos clubes.
/// 1 goleada, 2 e 3 vitórias contra mais fortes, 4 comum, 5 e 7 derrotas para mais fracos, 6
/// zebra. Só o al é fiel (quem chama usa al).
pub(crate) unsafe extern "C" fn resultado_da_eliminatoria_na_noticia(clube: usize, partida: usize) -> u8 {
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

/// stores one of the eight cup round match days: the venue flags, the weekday, the time slot and the code
/// Original: `cup_set_round_match_day` (0x521f10, Cup.cpp, cdecl).
///
/// [pt] { 0x521f10 } (cdecl, Cup.cpp): o dia `vaga` (0 a 7) da rodada `rodada` (registros de 104 bytes):
/// em 7 bytes a partir de +0x23, o mando (4 bytes), o dia da semana (−1 a 6), o horário (−1 a 2) e o
/// código (−1 a 3). Fora da faixa: a original (caixa de erro e fim do jogo). Devolve o mando.
pub(crate) unsafe extern "C" fn dia_da_rodada_da_copa(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(539)(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    *((e + 0x27) as *mut u8) = semana as u8; // dia da semana
    *((e + 0x28) as *mut u8) = hora as u8;
    *((e + 0x29) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((e + 0x23) as *mut u32, mando);
    mando
}

pub(crate) unsafe extern "C" fn comparar_partida_das_vagas_da_noticia(noticia: usize, partida: usize, cursor: *mut u8) {
    let o = orig::<unsafe extern "C" fn(usize, usize, *mut u8)>(537);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(partida, 0x4d), (cursor as usize, 1)],
        || o(noticia, partida, cursor), || partida_das_vagas_da_noticia(noticia, partida, cursor));
    PLACAR.registrar(igual, || format!("partida_das_vagas_da_noticia {:x?} x {:x?}", mo, mr));
}

pub(crate) unsafe extern "C" fn comparar_resultado_da_eliminatoria_na_noticia(clube: usize, partida: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(538)(clube, partida);
    let n = resultado_da_eliminatoria_na_noticia(clube, partida);
    PLACAR.registrar(r == n, || format!("resultado_da_eliminatoria_na_noticia original={} rust={}", r, n));
    r
}

pub(crate) unsafe extern "C" fn comparar_dia_da_rodada_da_copa(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(539);
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return o(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x23, 7)], || o(rodadas, rodada, vaga, semana, hora, codigo, mando),
        || dia_da_rodada_da_copa(rodadas, rodada, vaga, semana, hora, codigo, mando));
    PLACAR.registrar(igual && r == n, || format!("dia_da_rodada_da_copa({}, {}) {:x?} x {:x?}", rodada, vaga, mo, mr));
    r
}

