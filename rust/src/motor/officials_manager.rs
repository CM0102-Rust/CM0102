use super::*;

// ---------- centesima trigesima setima leva: a nota do arbitro (0x6d4a20, 0x79ce10) ----------

/// adds the grade to the referee's record (17 bytes at [this] + id × 17, id in 0..[0xae2390]): +0xf the cards, words +8
/// and +0xa the fouls and the grade, +0x10 the last grade, +0xc the matches and +4 the average (+0xa / +0xc, float).
/// Compared side by side (the record)
/// Original: `record_referee_grade` (0x79ce10, officials_manager.cpp (208 bytes), thiscall, ret 4).
///
/// [pt] { 0x79ce10 } (ret 4, officials_manager.cpp): soma a nota `r` (dword id, bytes: cartoes, faltas,
/// nota) na ficha do arbitro (17 bytes em [this] + id x 17, id em 0..[0xae2390]): +0xf os cartoes,
/// as palavras +8 e +0xa as faltas e a nota, +0x10 a ultima nota, +0xc os jogos e +4 a media
/// (+0xa / +0xc, float).
pub(crate) unsafe extern "thiscall" fn registrar_nota_do_arbitro(this: usize, r: *const u8) {
    let base = *(this as *const usize);
    if base == 0 || r.is_null() {
        return;
    }
    let id = *(r as *const i32);
    if id < 0 || id >= *(G_OFFICIAL_COUNT as *const i32) {
        return;
    }
    let f = base + id as usize * 17;
    let b = |o: usize| (f + o) as *mut u8;
    let w = |o: usize| (f + o) as *mut i16;
    *b(0xf) = (*b(0xf)).wrapping_add(*r.add(4));
    *w(8) = (*w(8)).wrapping_add(*r.add(5) as i8 as i16);
    *w(0xa) = (*w(0xa)).wrapping_add(*r.add(6) as i8 as i16);
    *b(0x10) = *r.add(6);
    *w(0xc) = (*w(0xc)).wrapping_add(1);
    if *w(0xc) != 0 {
        *((f + 4) as *mut f32) = (*w(0xa) as f64 / *w(0xc) as f64) as f32;
    }
}


/// A ficha do arbitro que a 0x79ce10 mexe (vazia se o id nao vale).
pub(crate) unsafe fn ficha_do_arbitro(gerente: usize, id: i32) -> Vec<(usize, usize)> {
    let base = *(gerente as *const usize);
    if base == 0 || id < 0 || id >= *(G_OFFICIAL_COUNT as *const i32) {
        return Vec::new();
    }
    vec![(base + id as usize * 17, 17)]
}

pub(crate) unsafe extern "thiscall" fn comparar_registrar_nota_do_arbitro(this: usize, r: *const u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const u8)>(IDX_REGISTRAR_NOTA_DO_ARBITRO);
    if gravando_no_topo() || r.is_null() {
        return o(this, r);
    }
    let reg = ficha_do_arbitro(this, *(r as *const i32));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, r), || registrar_nota_do_arbitro(this, r));
    PLACAR.registrar(igual, || format!("registrar_nota_do_arbitro(nota {}) {:x?} x {:x?}", *r.add(6), mo, mr));
}


// ---------- 271a leva: os árbitros do jogo novo: os que ficam por nação, as vagas das competições e a parte de cada uma; a ordem das vagas pelo nível da competição (officials_manager.cpp) ----------







/// O registro do árbitro `i` (0x2b bytes em [0xae23e8], [0xae2390] deles).
pub(crate) unsafe fn cqg9_arbitro(i: i32) -> usize {
    read_u32_as_usize(G_OFFICIAL_TABLE).wrapping_add((i as isize * 0x2b) as usize)
}

/// O fim fatal da original sem memória: a caixa (linha `linha` de officials_manager.cpp, 0xa6a22c), 0x5cc7d0, 0x5a8f60
/// e exit(−1) (0x945934).
pub(crate) unsafe fn cqg9_fatal(linha: u32) -> ! {
    aviso_em(0xa6a22c, linha);
    let f1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let f2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    f1();
    f2();
    sair(-1)
}

/// O qsort do jogo (0x945333) pela rotina de comparação `cmp`.
pub(crate) unsafe fn cqg9_ordenar(base: usize, n: u32, tam: u32, cmp: usize) {
    let q: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    q(base, n, tam, cmp);
}

/// calloc (0x945501, cdecl).
pub(crate) unsafe fn cqg9_calloc(n: u32, tam: u32) -> usize {
    let f: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    f(n, tam)
}

/// free (0x9452ca, cdecl).
pub(crate) unsafe fn cqg9_liberar(p: usize) {
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    f(p);
}

/// on a new game: reviews the officials, sorts them by nation and quality, keeps 6 per nation (61 in the youth club
/// nations), builds the per-nation blocks with one slot per league (the demand from the competition), draws the missing
/// ones and shares the nation officials among the slots
/// Original: `OfficialsManager::set_up_officials` (0x79bf60, officials_manager.cpp, thiscall).
///
/// [pt] { 0x79bf60 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::set_up_officials()`, pelo construtor
/// (0x79b580) no jogo novo. Revê os registros (0x79caa0), ordena por nação e pela qualidade (qsort 0x79d4e0) e, por
/// nação, fica com os 6 primeiros (nação com +0x11c zero) ou 61 (o primeiro de cada nação depois da primeira não fica:
/// a conta recomeça nele), copiados (0x79c530) numa tabela nova; os blocos por nação (this+0xc, 0x102 bytes: +0 a nação,
/// +4 os árbitros, +8 o primeiro, +0xc o pedido, +0x10 as competições e 15 vagas de 16 bytes em +0x12): cada liga
/// (0x4b0700, não amistosa 0x5c6710, com nação +0x5d) ganha uma vaga com o pedido (+0x3e da competição) / 2 + 10; a nação
/// com competições pede o maior × a quantidade e, faltando, sorteia os que faltam (0x79c5c0); sem competições e menos de
/// 5, sorteia até o +0x8e da nação (1..5). Os índices (+0), o primeiro de cada nação e, por nação, as vagas ordenadas
/// (qsort 0x79d410) com a parte de cada uma; this+4 a conta e this+8 as marcas (calloc).
pub(crate) unsafe extern "thiscall" fn montar_os_arbitros(this: usize) {
    let rever: unsafe extern "C" fn() = core::mem::transmute(FN_OFFICIALS_MANAGER_VALIDATE_OFFICIALS);
    rever();
    cqg9_ordenar(read_u32_as_usize(G_OFFICIAL_TABLE), read_i32(0xae2390) as u32, 0x2b, 0x79d4e0);
    let n = read_i32(0xae2390);
    let marcas = cqg9_calloc(n as u32, 1);
    let mut ficam = 0i32;
    if n > 0 {
        let mut atual = read_u32_as_usize(cqg9_arbitro(0) + 0x16);
        let mut grupo = 0i32;
        for i in 0..n {
            let nac = read_u32_as_usize(cqg9_arbitro(i) + 0x16);
            if nac != atual {
                grupo = 0;
                atual = nac;
                continue;
            }
            let m = (marcas + i as usize) as *mut u8;
            if nac == 0 {
                *m = 0;
                continue;
            }
            let limite = if *((nac + 0x11c) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: os árbitros que ficam por nação
            if grupo <= limite {
                ficam += 1;
                *m = 1;
                crate::comparacao::ramo("cqg_9 arbitro mantido");
            } else {
                *m = 0;
            }
            grupo += 1;
        }
    }
    let nova = cqg9_calloc(ficam as u32, 0x2b);
    let mut dst = nova;
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_OFFICIAL_OPERATOR_3D);
    let mut i = 0i32;
    while i < read_i32(0xae2390) {
        if *((marcas + i as usize) as *const u8) != 0 {
            copiar(dst, cqg9_arbitro(i));
            dst += 0x2b;
        }
        i += 1;
    }
    cqg9_liberar(read_u32_as_usize(G_OFFICIAL_TABLE));
    write_u32(G_OFFICIAL_TABLE, nova as u32);
    write_u32(G_OFFICIAL_COUNT, ficam as u32);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let nacoes = read_i32(0xae2350);
    let blocos = malloc((nacoes as u32).wrapping_mul(0x102));
    write_u32(this + 0xc, blocos as u32);
    if blocos == 0 {
        cqg9_fatal(0x1ff);
    }
    for k in 0..nacoes.max(0) {
        let b = blocos + k as usize * 0x102;
        write_i16(b + 0x10, 0);
        write_u32(b + 4, 0);
        write_u32(b + 8, 1_000_000);
        write_u32(b + 0xc, 0);
        write_u32(b, k as u32);
        for j in 0..0xf {
            let v = b + 0x12 + 16 * j;
            write_u32(v, 1_000_000);
            write_u32(v + 4, 0);
            write_u32(v + 8, 0);
            write_u32(v + 0xc, 0xffff_ffff);
        }
    }
    let ativa: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    for k in 0..read_i32(0xadadf8).max(0) {
        if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + 4 * k as usize) == 0 || ativa(k) == 0 {
            continue;
        }
        let comp = read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(k as usize * 0x6b);
        if amistoso(read_u32_as_usize(G_FRIENDLY), comp) != 0 {
            continue;
        }
        let nac = read_u32_as_usize(comp + competition::CLUB_COMP_NATION);
        if nac == 0 {
            continue;
        }
        let b = blocos.wrapping_add((read_i32(nac) as isize * 0x102) as usize);
        let v = b.wrapping_add((read_i16(b + 0x10) as isize * 0x10) as usize);
        write_u32(v + 0x1e, k as u32);
        let pedido = read_i16(read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xa; // cqg9: o pedido da vaga
        crate::comparacao::ramo("cqg_9 vaga de competicao");
        write_u32(v + 0x1a, pedido as u32);
        if read_i32(v + 0x1a) > read_i32(b + 0xc) {
            write_u32(b + 0xc, read_u32_as_usize(v + 0x1a) as u32);
        }
        write_i16(b + 0x10, read_i16(b + 0x10).wrapping_add(1));
    }
    for i in 0..read_i32(0xae2390).max(0) {
        let b = blocos.wrapping_add((read_i32(read_u32_as_usize(cqg9_arbitro(i) + 0x16)) as isize * 0x102) as usize);
        write_u32(b + 4, read_u32_as_usize(b + 4).wrapping_add(1) as u32);
    }
    let novos: unsafe extern "stdcall" fn(i32, usize) = core::mem::transmute(FN_OFFICIALS_MANAGER_ADD_RANDOM_OFFICIALS);
    for k in 0..read_i32(0xae2350).max(0) {
        let b = blocos + k as usize * 0x102;
        let nac = read_u32_as_usize(G_NATION_TABLE).wrapping_add(k as usize * 0x122);
        let c = read_i16(b + 0x10);
        if c > 0 {
            write_u32(b + 0xc, (c as i32).wrapping_mul(read_i32(b + 0xc)) as u32);
            if read_i32(b + 0xc) > read_i32(b + 4) {
                novos(read_i32(b + 0xc) - read_i32(b + 4), nac);
                write_u32(b + 4, read_u32_as_usize(b + 0xc) as u32);
            }
        } else if read_i32(b + 4) < 5 {
            if read_i16(nac + 0x8e) == 0 {
                write_i16(nac + 0x8e, 1);
            }
            novos((read_i16(nac + 0x8e) as i32).min(5), nac);
            write_u32(b + 4, (read_i16(read_u32_as_usize(G_NATION_TABLE).wrapping_add(k as usize * 0x122) + 0x8e) as i32).min(5) as u32);
        }
    }
    for i in 0..read_i32(0xae2390).max(0) {
        write_u32(cqg9_arbitro(i), i as u32);
    }
    for i in 0..read_i32(0xae2390).max(0) {
        let r = cqg9_arbitro(i);
        let b = blocos.wrapping_add((read_i32(read_u32_as_usize(r + 0x16)) as isize * 0x102) as usize);
        if read_i32(r) < read_i32(b + 8) {
            write_u32(b + 8, read_u32_as_usize(r) as u32);
        }
    }
    for k in 0..read_i32(0xae2350).max(0) {
        let b = read_u32_as_usize(this + 0xc) + k as usize * 0x102;
        let c = read_i16(b + 0x10);
        if c == 0 {
            continue;
        }
        cqg9_ordenar(b + 0x12, c as i32 as u32, 0x10, 0x79d410);
        let mut j = 0i32;
        while j < read_i16(b + 0x10) as i32 {
            let c = read_i16(b + 0x10) as i32;
            crate::comparacao::ramo("cqg_9 parte da vaga");
            write_u32(b + 0x16 + 16 * j as usize, (read_i32(b + 4) / c) as u32); // cqg9: a parte de cada vaga
            let c = read_i16(b + 0x10) as i32;
            write_u32(b + 0x12 + 16 * j as usize, (read_i32(b + 4).wrapping_mul(j) / c).wrapping_add(read_i32(b + 8)) as u32);
            j += 1;
        }
    }
    let total = read_i32(0xae2390);
    write_u32(this + 4, total as u32);
    let m = cqg9_calloc(total as u32, 1);
    write_u32(this + 8, m as u32);
    if m == 0 {
        cqg9_fatal(0x2dd);
    }
    cqg9_liberar(marcas);
}

/// Comparador: só a original (ponytail: aloca, libera e troca a tabela dos árbitros, e as chamadas sorteiam: não pode
/// rodar duas vezes), com o plano: os blocos por nação (this+0xc), a conta (this+4) igual à da tabela e as marcas (this+8).
pub(crate) unsafe extern "thiscall" fn comparar_montar_os_arbitros(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(IDX_MONTAR_OS_ARBITROS)(this);
    PLACAR.registrar(read_u32_as_usize(this + 0xc) != 0 && read_i32(this + 4) == read_i32(0xae2390) && read_u32_as_usize(this + 8) != 0,
        || format!("montar_os_arbitros: blocos {:x}, conta {} x {}", read_u32_as_usize(this + 0xc), read_i32(this + 4), read_i32(0xae2390)));
}



/// orders the official slots of a nation by competition level (the invalid ones last or first), tie by id
/// Original: `officials_manager_compare_comp_level` (0x79d410, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79d410 } (cdecl, officials_manager.cpp): `officials_manager_compare_comp_level(a, b)`, a ordem das vagas de
/// competição de uma nação (qsort da 0x79bf60, entradas de 16 bytes com a competição em +0xc): competição de `a` fora de
/// 0..[0xae2378]: 1; a de `b` fora: −1; pelo nível (+0x69, palavra, da competição de 0x6b bytes em [0xae23d0]); mesmo
/// nível: a diferença dos ids.
pub(crate) unsafe extern "C" fn ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_i32(a + 0xc), read_i32(b + 0xc));
    let max = read_i32(0xae2378);
    if ca < 0 || ca > max {
        return 1;
    }
    if cb < 0 || cb > max {
        return -1;
    }
    let base = read_i32(0xae23d0) as u32 as usize;
    let sa = read_i16(base.wrapping_add((ca as isize * 0x6b + 0x69) as usize));
    let sb = read_i16(base.wrapping_add((cb as isize * 0x6b + 0x69) as usize));
    if sa == sb {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - sb as i32
}

/// Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_ORDEM_DAS_VAGAS_DE_COMPETICAO)(a, b);
    let n = ordem_das_vagas_de_competicao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_vagas_de_competicao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 271a leva: a ordem dos árbitros por nação e qualidade, nos dois sentidos (officials_manager.cpp) ----------



/// A qualidade do árbitro na ordem: o byte +0x28 × 50 + a palavra +0x22, em 16 bits.
pub(crate) unsafe fn cqh7_qualidade(r: usize) -> i16 {
    (*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(read_i16(r + 0x22)) // cqh7: o peso da qualidade
}

/// O id da nação do árbitro ([+0x16] + 0).
pub(crate) unsafe fn cqh7_nacao(r: usize) -> i32 {
    read_i32(read_i32(r + 0x16) as u32 as usize)
}

/// orders the officials by nation and, within it, by quality (+0x28 × 50 + +0x22) from highest to lowest; tie by index,
/// descending
/// Original: `officials_manager_compare_nation_then_quality_desc` (0x79d4e0, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79d4e0 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_desc(a, b)`, a ordem do
/// qsort da 0x79bf60 (os que ficam por nação): pela nação (id) e, na mesma, pela qualidade da maior para a menor; empate:
/// o índice +0 de `b` menos o de `a`.
pub(crate) unsafe extern "C" fn ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem decrescente");
    if ka == kb {
        return read_i32(b).wrapping_sub(read_i32(a)); // cqh7: o empate pelo índice
    }
    kb as i32 - ka as i32
}

/// Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_ORDEM_DOS_ARBITROS_DECRESCENTE)(a, b);
    let n = ordem_dos_arbitros_decrescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_decrescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// orders the officials by nation and by quality from lowest to highest; tie by index, ascending
/// Original: `officials_manager_compare_nation_then_quality_asc` (0x79d480, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79d480 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_asc(a, b)`, a ordem do
/// qsort da 0x79c5c0 (depois dos árbitros novos): pela nação e pela qualidade da menor para a maior; empate: o índice +0
/// de `a` menos o de `b`.
pub(crate) unsafe extern "C" fn ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem crescente");
    if ka == kb {
        return read_i32(a).wrapping_sub(read_i32(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn comparar_ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_ORDEM_DOS_ARBITROS_CRESCENTE)(a, b);
    let n = ordem_dos_arbitros_crescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_crescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 261a leva: o árbitro de cada jogo: a escolha do árbitro, as marcas de ocupado do dia e a renovação dos árbitros da liga (officials_manager.cpp) ----------





/// O bloco de árbitros da nação `nacao` no gerenciador (0x102 bytes em [this+0xc]: +4 conta, +8 primeiro índice, +0x10
/// palavra com o número de faixas de liga, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição).
pub(crate) unsafe fn cpy8_bloco(this: usize, nacao: i32) -> usize {
    read_usize(this + 0xc).wrapping_add(nacao.wrapping_mul(0x102) as usize)
}

/// O id da nação do clube (+0x53, ponteiro) ou, sem nação, `sem`.
pub(crate) unsafe fn cpy8_nacao_do_clube(clube: usize) -> Option<i32> {
    let n = read_usize(clube + club::NATION);
    if n != 0 { Some(read_i32(n)) } else { None }
}

/// the referee range for the competition (the league range in the nation block, the top quarter of a neutral nation of
/// the continent in type 2 and 4 competitions, the home nation for a friendly), a random one in it and the next free
/// one
/// Original: `OfficialsManager::choose_match_official` (0x79c7b0, officials_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x79c7b0 } (thiscall, ret 8, officials_manager.cpp): `OfficialsManager::choose_match_official(jogo, saida)`, pela
/// montagem do dia (0x6b3c10, saída em +0x2e do jogo). Jogo tirado (−2): 0 (eax 0xffffff00, como a original). A faixa
/// de árbitros: sem competição, a nação do mandante (+0x1c, +0x53) ou uma sorteada; amistoso (0x5c6710 em [0xae2c74]),
/// a do mandante; competição sem objeto ([0xadadfc]), como sem competição; competição dos tipos 2 e 4 (+0x42), o quarto
/// de cima (ftol de 0,75 e 0,25 da conta) da primeira nação, a partir de uma sorteada, do continente (+0x59 da
/// competição, ou +0x71 da nação do mandante, do visitante, ou um sorteado) que não seja a de nenhum dos dois clubes;
/// nas outras, a faixa da competição no bloco da nação dela (+0x5d; ou do mandante, do visitante, ou sorteada), a última
/// que bater, ou o bloco inteiro. Conta 1: o primeiro; senão primeiro + sorteio(conta − 1). Ocupado (o byte do id do
/// árbitro em [this+8]): o seguinte da faixa, dando a volta; todos ocupados, o sorteado. Grava o índice (palavra) e
/// devolve 1 no al (o resto do eax com o índice, como a original).
pub(crate) unsafe extern "thiscall" fn escolher_arbitro_da_partida(this: usize, jogo: usize, saida: usize) -> u32 {
    if read_i32(jogo) == -2 {
        return 0xffff_ff00;
    }
    let nacoes = read_i32(0xae2350);
    let mandante = read_usize(jogo + 0x1c);
    let visitante = read_usize(jogo + 0x20);
    let comp = read_usize(jogo + 0x14);
    let faixa_da_nacao = |nacao: i32| {
        let b = cpy8_bloco(this, nacao);
        (read_i32(b + 8), read_i32(b + 4))
    };
    let (ini, n) = if comp == 0 {
        faixa_da_nacao(cpy8_nacao_do_clube(mandante).unwrap_or_else(|| sortear(nacoes)))
    } else {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
        if amistoso(read_usize(G_FRIENDLY), comp) != 0 {
            faixa_da_nacao(read_i32(read_usize(mandante + 0x53))) // como a original, sem conferir a nação
        } else {
            let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(comp).wrapping_mul(4) as usize));
            if c == 0 {
                faixa_da_nacao(cpy8_nacao_do_clube(mandante).unwrap_or_else(|| sortear(nacoes)))
            } else {
                let tipo = read_u8(c + 0x42) as i8;
                if tipo == 2 || tipo == 4 {
                    let continente_do = |clube: usize| {
                        let n = read_usize(clube + club::NATION);
                        if n != 0 && read_usize(n + 0x71) != 0 { Some(read_i32(read_usize(n + 0x71))) } else { None }
                    };
                    let continente = if read_usize(comp + competition::CLUB_COMP_CONTINENT) != 0 {
                        read_i32(read_usize(comp + competition::CLUB_COMP_CONTINENT))
                    } else if let Some(k) = continente_do(mandante) {
                        k
                    } else if let Some(k) = continente_do(visitante) {
                        k
                    } else {
                        sortear(read_i32(0xae234c))
                    };
                    let inicio = sortear(nacoes);
                    let mut k = inicio;
                    loop {
                        let nac = read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize);
                        let cont = read_usize(nac + 0x71);
                        if cont != 0 && read_i32(cont) == continente
                            && cpy8_nacao_do_clube(mandante) != Some(read_i32(nac))
                            && cpy8_nacao_do_clube(visitante) != Some(read_i32(nac))
                        {
                            break;
                        }
                        k += 1;
                        if k >= read_i32(0xae2350) {
                            k = 0;
                        }
                        if k == inicio {
                            break;
                        }
                    }
                    let b = cpy8_bloco(this, k);
                    let conta = read_i32(b + 4) as f64;
                    crate::comparacao::ramo("cpy_8 arbitro neutro");
                    (read_i32(b + 8).wrapping_sub(ftol(conta * -0.75)), ftol(conta * 0.25)) // cpy8: o quarto de cima
                } else {
                    let nacao = if read_usize(comp + competition::CLUB_COMP_NATION) != 0 {
                        read_i32(read_usize(comp + competition::CLUB_COMP_NATION))
                    } else if let Some(k) = cpy8_nacao_do_clube(mandante) {
                        k
                    } else if let Some(k) = cpy8_nacao_do_clube(visitante) {
                        k
                    } else {
                        sortear(nacoes)
                    };
                    let b = cpy8_bloco(this, nacao);
                    let id = read_i32(comp);
                    let mut achou = None;
                    for f in 0..read_i16(b + 0x10).max(0) as usize {
                        let e = b + 0x12 + f * 0x10;
                        if read_i32(e + 0xc) == id {
                            achou = Some((read_i32(e), read_i32(e + 4)));
                        }
                    }
                    match achou {
                        Some(faixa) => {
                            crate::comparacao::ramo("cpy_8 faixa da liga");
                            faixa // cpy8: a faixa da liga
                        }
                        None => (read_i32(b + 8), read_i32(b + 4)),
                    }
                }
            }
        }
    };
    let sorteado = if n == 1 {
        ini
    } else {
        crate::comparacao::ramo("cpy_8 arbitro sorteado");
        ini.wrapping_add(sortear(n.wrapping_sub(1))) // cpy8: o sorteio na faixa
    };
    let ocupado = |k: i32| {
        let ficha = read_usize(G_OFFICIAL_TABLE).wrapping_add(k.wrapping_mul(0x2b) as usize);
        read_u8(read_usize(this + 8).wrapping_add(read_usize(ficha))) != 0
    };
    let mut k = sorteado;
    if ocupado(k) {
        let fim = n.wrapping_add(ini);
        loop {
            k += 1;
            if k >= fim {
                k = ini;
            }
            if k == sorteado || !ocupado(k) {
                break;
            }
        }
    }
    core::ptr::write_unaligned(saida as *mut i16, k as i16);
    (k as u32 & 0xffff_ff00) | 1
}

/// Comparador: lado a lado pela palavra de saída e pelo estado do sorteio, e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_escolher_arbitro_da_partida(this: usize, jogo: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_ESCOLHER_ARBITRO_DA_PARTIDA);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 2));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida),
        || crate::aleatorio::sem_contar(|| escolher_arbitro_da_partida(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("escolher_arbitro_da_partida(jogo {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", jogo, r, mo, n, mr));
    r
}

/// clears the daily referee busy bytes
/// Original: `OfficialsManager::clear_busy_flags` (0x79bf40, officials_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x79bf40 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::clear_busy_flags`, no laço do dia
/// (0x5cccd0) e na corrida até a data de início (0x5cc850): zera os [this+4] bytes de ocupado em [this+8] (a conta e o
/// vetor relidos a cada passo). Devolve a conta (0 se não positiva), como o eax da original.
pub(crate) unsafe extern "thiscall" fn limpar_arbitros_ocupados(this: usize) -> u32 {
    let mut i = 0i32;
    while i < read_i32(this + 4) {
        *(read_usize(this + 8).wrapping_add(i as usize) as *mut u8) = 0;
        i += 1;
    }
    i as u32
}

/// Comparador: lado a lado pelos bytes de ocupado e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_limpar_arbitros_ocupados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_LIMPAR_ARBITROS_OCUPADOS);
    let n = read_i32(this + 4);
    let reg = if n > 0 && read_usize(this + 8) != 0 { vec![(read_usize(this + 8), n as usize)] } else { Vec::new() };
    let (r, m, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || limpar_arbitros_ocupados(this));
    PLACAR.registrar(igual && r == m, || format!("limpar_arbitros_ocupados(conta {}) original={} rust={} memoria igual {}", n, r, m, igual));
    r
}

/// at the league season start, replaces the referees of the range over 45 (or over 40, one in three) with new random
/// ones and clears the stats row of the first one
/// Original: `OfficialsManager::renew_officials` (0x79cee0, officials_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x79cee0 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::renew_officials(comp)`, no começo da
/// temporada de cada liga (83 chamadores). Sem competição, sem objeto ([0xadadfc]), dos tipos 4 e 2 (+0x42) ou sem
/// conta (0x4b0700): 0. A faixa da competição no bloco da nação dela (+0x5d, ou uma sorteada), a primeira que bater,
/// ou o bloco inteiro. Cada árbitro da faixa (fichas de 0x2b bytes em [0xae23e8]; ficha nula: a caixa "Error"): com
/// mais de 45 anos (o ano em byte, menos 1 antes do dia dele), ou mais de 40 e sorteio(3) zero, vira outro: dois nomes
/// da nação dele (0x539c60, +4 e +8), +0x25 sorteio(16) + 5 (ou, com sorteio(10) zero, sorteio(11) + sorteio(11)), o
/// ano (+0xe) ano do jogo − 27 + sorteio(5) e o dia (+0xc) sorteio(340). Depois zera as linhas de estatísticas (17
/// bytes em [this]) de cada árbitro da faixa (+4, +8, +0xa, +0xc, +0xf; o `add eax, 0x11` em 0x79d149 fica dentro do
/// laço). Devolve 1 (só o al vale).
pub(crate) unsafe extern "thiscall" fn renovar_arbitros(this: usize, comp: usize) -> u8 {
    if comp == 0 {
        return 0;
    }
    let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(comp).wrapping_mul(4) as usize));
    if c == 0 || read_u8(c + 0x42) == 4 || read_u8(c + 0x42) == 2 {
        return 0;
    }
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    if conta(read_i32(comp)) == 0 {
        return 0;
    }
    let nacao = if read_usize(comp + competition::CLUB_COMP_NATION) != 0 { read_i32(read_usize(comp + competition::CLUB_COMP_NATION)) } else { sortear(read_i32(0xae2350)) };
    let b = cpy8_bloco(this, nacao);
    let mut faixa = None;
    for f in 0..read_i16(b + 0x10).max(0) as usize {
        let e = b + 0x12 + f * 0x10;
        if read_i32(e + 0xc) == read_i32(comp) {
            faixa = Some((read_i32(e), read_i32(e + 4)));
            break;
        }
    }
    let (ini, n) = faixa.unwrap_or((read_i32(b + 8), read_i32(b + 4)));
    let nome: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    for k in 0..n.max(0) {
        let ficha = read_usize(G_OFFICIAL_TABLE).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x2b) as usize);
        if ficha == 0 {
            aviso_em(0xa6a22c, 0xa61);
            continue;
        }
        let ano = read_u8(G_CURRENT_DATE_YEAR);
        let dif = (read_i16(ficha + 0xc) as i32).wrapping_sub(read_i16(G_CURRENT_DATE) as i32);
        let idade = if dif > 0 { ano.wrapping_sub(read_u8(ficha + 0xe)).wrapping_sub(1) } else { ano.wrapping_sub(read_u8(ficha + 0xe)) } as i8;
        if idade <= 0x28 || (idade <= 0x2d && sortear(3) != 0) {
            continue;
        }
        crate::comparacao::ramo("cpy_8 arbitro renovado");
        let nac = read_usize(ficha + 0x16);
        let n1 = nome(nac);
        core::ptr::write_unaligned((ficha + 4) as *mut usize, n1);
        let n2 = nome(read_usize(ficha + 0x16));
        core::ptr::write_unaligned((ficha + 8) as *mut usize, n2);
        *((ficha + 0x25) as *mut u8) = if sortear(10) != 0 {
            (sortear(0x10) as u8).wrapping_add(5)
        } else {
            let a = sortear(0xb) as u8;
            a.wrapping_add(sortear(0xb) as u8)
        };
        let r5 = sortear(5);
        let base = (read_i16(G_CURRENT_DATE_YEAR) as u16).wrapping_sub(0x1b);
        core::ptr::write_unaligned((ficha + 0xe) as *mut u16, (r5 as u16).wrapping_add(base));
        core::ptr::write_unaligned((ficha + 0xc) as *mut u16, sortear(0x154) as u16);
    }
    for k in 0..n.max(0) {
        let linha = read_usize(this).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x11) as usize);
        *((linha + 0xf) as *mut u8) = 0;
        core::ptr::write_unaligned((linha + 8) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xa) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 4) as *mut u32, 0);
    }
    1
}

/// Comparador: lado a lado pelas fichas de todos os árbitros ([0xae2390] × 0x2b), pelas linhas de estatísticas
/// ([0xae2390] × 17), pelo estado do sorteio e por [0xb67a34], e pela devolução. Sem tabela de árbitros (a caixa a cada
/// ficha): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_renovar_arbitros(this: usize, comp: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_RENOVAR_ARBITROS);
    let total = read_i32(0xae2390).max(0) as usize;
    if read_usize(G_OFFICIAL_TABLE) == 0 || read_usize(this) == 0 {
        return o(this, comp);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(read_usize(G_OFFICIAL_TABLE), total * 0x2b), (read_usize(this), total * 0x11), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, comp),
        || crate::aleatorio::sem_contar(|| renovar_arbitros(this, comp)));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("renovar_arbitros(comp {:#x}) original={} rust={}, diferencas em {:x?}", comp, r, n, dif)
    });
    r
}

// ---------- 261a leva: os árbitros do jogo novo: os árbitros sorteados de uma nação, o conserto das fichas dos árbitros e a cópia de uma ficha (officials_manager.cpp) ----------








/// Um byte sorteado: sorteio(n) + k no al.
pub(crate) unsafe fn cpy9_s(n: i32, k: u8) -> u8 {
    (sortear(n) as u8).wrapping_add(k)
}

/// Uma ficha de árbitro nova em `r` (0x2b bytes) da nação `nacao`, na ordem da 0x79c6aa: primeiro e segundo nome pelo
/// peso (0x539b90, 0x53a0e0), a nação (+0x16), as palavras +0x22 sorteio(80) + 1, +0x1e e +0x20 sorteio(80) + 40 (o
/// byte estendido), os bytes +0x24..+0x26 sorteio(10) + 5 e +0x27..+0x2a sorteio(10) + 7, +0x1a zero, o ano (+0xe)
/// 1960 + sorteio(10) e o dia (+0xc) sorteio(300) + 1. Não grava +0, +0x10..+0x15 (a original deixa o que o realloc
/// trouxe).
pub(crate) unsafe fn cpy9_gerar(r: usize, nacao: usize) {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_BY_WEIGHT);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_BY_WEIGHT);
    write_usize(r + 4, primeiro(nacao));
    write_usize(r + 8, segundo(nacao));
    write_usize(r + 0x16, nacao);
    write_i16(r + 0x22, cpy9_s(0x50, 1) as i8 as i16); // cpy9: a nota +0x22
    write_i16(r + 0x1e, cpy9_s(0x50, 0x28) as i8 as i16);
    write_i16(r + 0x20, cpy9_s(0x50, 0x28) as i8 as i16);
    for o in [0x24usize, 0x25, 0x26] {
        write_u8(r + o, cpy9_s(10, 5));
    }
    write_u8(r + 0x27, cpy9_s(10, 7));
    write_u8(r + 0x28, cpy9_s(10, 7)); // cpy9: o atributo +0x28
    write_u8(r + 0x29, cpy9_s(10, 7));
    write_u8(r + 0x2a, cpy9_s(10, 7));
    write_usize(r + 0x1a, 0);
    write_i16(r + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
    write_i16(r + 0xc, sortear(0x12c).wrapping_add(1) as i16);
}

/// reallocates the referee table, generates `count` new referees of the nation (names by weight, random ratings and
/// attributes, born in the 1960s) and sorts the table by nation and quality
/// Original: `officials_manager_add_random_officials` (0x79c5c0, officials_manager.cpp, thiscall (this sem uso), ret 8).
///
/// [pt] { 0x79c5c0 } (thiscall com o this sem uso, ret 8, officials_manager.cpp): `officials_manager_add_random_officials(
/// conta, nacao)`, pela montagem dos árbitros do jogo novo (0x79bf60, nas nações com poucos árbitros). Nação sem
/// continente (+0x71): nada. Realoca a tabela ([0xae23e8], 0x945667) para [0xae2390] + conta fichas (sem memória: erro
/// e o jogo para), gera `conta` fichas novas no fim (cpy9_gerar), soma a conta em [0xae2390] e ordena a tabela inteira
/// (qsort 0x945333 com 0x79d480: nação, +0x28 × 50 + +0x22, id).
pub(crate) unsafe extern "thiscall" fn arbitros_sorteados(_this: usize, conta: i32, nacao: usize) {
    if nacao != 0 && read_usize(nacao + nation::CONTINENT) == 0 {
        return;
    }
    let realocar: unsafe extern "C" fn(usize, i32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let antes = read_i32(0xae2390);
    let t = realocar(read_usize(G_OFFICIAL_TABLE), antes.wrapping_add(conta).wrapping_mul(0x2b));
    *(G_OFFICIAL_TABLE as *mut usize) = t;
    if t == 0 {
        erro_em(0xa6a22c, 0x342);
    }
    let mut r = t.wrapping_add(antes.wrapping_mul(0x2b) as usize);
    for _ in 0..conta.max(0) {
        crate::comparacao::ramo("cpy_9 arbitro novo");
        cpy9_gerar(r, nacao);
        r += 0x2b;
    }
    let n = read_i32(0xae2390).wrapping_add(conta);
    *(G_OFFICIAL_COUNT as *mut i32) = n;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    crate::comparacao::ramo("cpy_9 ordenar");
    ordenar(read_usize(G_OFFICIAL_TABLE), n, 0x2b, 0x79d480); // cpy9: ordenar a tabela
}

/// A ficha `g` (gerada no ensaio) bate com a `f` da tabela nos campos que a 0x79c5c0 grava (+4..+0x10, +0x16..+0x2b)?
pub(crate) unsafe fn cpy9_mesma_ficha(g: &[u8; 0x2b], f: usize) -> bool {
    let t = core::slice::from_raw_parts(f as *const u8, 0x2b);
    g[4..0x10] == t[4..0x10] && g[0x16..] == t[0x16..]
}

/// Comparador: realoca e ordena (não pode rodar duas vezes): só a original, depois de um ensaio. O ensaio gera as
/// `conta` fichas da cópia (cpy9_gerar, sem contar os sorteios) numa área própria a partir do mesmo estado do sorteio,
/// que é reposto; depois roda a original. Confere o estado final do sorteio, a conta nova ([0xae2390] + conta) e que
/// cada ficha do ensaio esteja na tabela ordenada (campos que a original grava). Conta não positiva ou nação sem
/// continente: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_arbitros_sorteados(this: usize, conta: i32, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize)>(IDX_ARBITROS_SORTEADOS);
    if conta <= 0 || (nacao != 0 && read_usize(nacao + nation::CONTINENT) == 0) {
        return o(this, conta, nacao);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let mut novas = vec![[0u8; 0x2b]; conta as usize];
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        for g in novas.iter_mut() {
            cpy9_gerar(g.as_mut_ptr() as usize, nacao);
        }
    }));
    let estado_rust = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    let antes = read_i32(0xae2390);
    o(this, conta, nacao);
    let estado_orig = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let total = read_i32(0xae2390);
    let t = read_usize(G_OFFICIAL_TABLE);
    let mut usada = vec![false; total.max(0) as usize];
    let mut faltam = Vec::new();
    for (k, g) in novas.iter().enumerate() {
        match (0..usada.len()).find(|&j| !usada[j] && cpy9_mesma_ficha(g, t + j * 0x2b)) {
            Some(j) => usada[j] = true,
            None => faltam.push(k),
        }
    }
    let igual = estado_orig == estado_rust && total == antes.wrapping_add(conta) && faltam.is_empty();
    PLACAR.registrar(igual, || format!("arbitros_sorteados(conta {}, nacao {:#x}) total {} (antes {}), sorteio igual {}, fichas da copia ausentes {:?}",
        conta, nacao, total, antes, estado_orig == estado_rust, faltam));
}

/// O ajuste de um atributo em byte (+0x24..+0x2a) fora de 1..20: o valor sorteado por `f` (lê a ficha antes).
pub(crate) unsafe fn cpy9_atributo(e: usize, o: usize, f: impl FnOnce() -> u8) {
    let v = read_u8(e + o) as i8;
    if v <= 0 || v > 0x14 {
        write_u8(e + o, f());
    }
}

/// repairs each referee record: the nation from the names, out-of-range ratings and attributes drawn again and a too
/// old birth date
/// Original: `officials_manager_validate_officials` (0x79caa0, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79caa0 } (cdecl, officials_manager.cpp): `officials_manager_validate_officials()`, no começo da montagem dos
/// árbitros do jogo novo (0x79bf60). Para cada uma das [0xae2390] fichas (0x2b bytes em [0xae23e8]): sem nação (+0x16),
/// a da tabela das nações pelo índice (+0x37) do primeiro nome (+4), do segundo (+8) ou sorteado; as palavras +0x22,
/// +0x1e e +0x20 fora de 1..200: sorteio(80) + 1 mais o byte +0x8e da nação (+0x22) ou 4 vezes ele (+0x1e, +0x20; a
/// +0x20 refeita fica no mínimo na +0x1e); os bytes +0x24..+0x2a fora de 1..20 sorteados pelas faixas de +0x1e (acima de
/// 120, de 80); nascido antes da data 0x549f70(1, 1, 1910, −1) (ano sem sinal, depois o dia com sinal), ano 1960 +
/// sorteio(10) e dia sorteio(300) + 1.
pub(crate) unsafe extern "C" fn validar_arbitros() {
    let base = read_usize(G_OFFICIAL_TABLE);
    let mut i = 0i32;
    while i < read_i32(0xae2390) {
        let e = base.wrapping_add((i as usize).wrapping_mul(0x2b));
        if read_usize(e + 0x16) == 0 {
            let (n1, n2) = (read_usize(e + 4), read_usize(e + 8));
            let k = if read_i32(n1 + 0x37) >= 0 {
                read_i32(n1 + 0x37)
            } else if read_i32(n2 + 0x37) >= 0 {
                read_i32(n2 + 0x37)
            } else {
                sortear(read_i32(0xae2350))
            };
            write_usize(e + 0x16, read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize));
        }
        let fora = |o: usize| {
            let v = read_i16(e + o);
            v <= 0 || v > 0xc8
        };
        if fora(0x22) {
            let nac = read_usize(e + 0x16);
            let v = if nac != 0 {
                let s = sortear(0x50) as u8;
                s.wrapping_add(read_u8(read_usize(e + 0x16) + 0x8e)).wrapping_add(1)
            } else {
                (sortear(0x50) as u8).wrapping_add(1)
            };
            write_i16(e + 0x22, v as i8 as i16);
        }
        for o in [0x1eusize, 0x20] {
            if fora(o) {
                crate::comparacao::ramo("cpy_9 nota consertada");
                let nac = read_usize(e + 0x16);
                let v = if nac != 0 {
                    let s = sortear(0x50) as u8;
                    s.wrapping_add(read_u8(read_usize(e + 0x16) + 0x8e) << 2).wrapping_add(1)
                } else {
                    (sortear(0x50) as u8).wrapping_add(1)
                };
                write_i16(e + o, v as i8 as i16);
                if o == 0x20 && read_i16(e + 0x20) < read_i16(e + 0x1e) {
                    write_i16(e + 0x20, read_i16(e + 0x1e));
                }
            }
        }
        let faixa = |alto: (i32, u8), medio: (i32, u8), baixo: (i32, u8)| {
            let w = read_i16(e + 0x1e);
            let (n, k) = if w > 0x78 { alto } else if w > 0x50 { medio } else { baixo };
            cpy9_s(n, k)
        };
        cpy9_atributo(e, 0x24, || if read_i16(e + 0x1e) > 0x64 { cpy9_s(0xc, 7) } else { cpy9_s(0xa, 5) });
        cpy9_atributo(e, 0x25, || faixa((0xe, 7), (0xf, 5), (0xa, 5)));
        cpy9_atributo(e, 0x26, || faixa((0xb, 0xa), (0xf, 5), (0xa, 5)));
        cpy9_atributo(e, 0x27, || if read_i16(e + 0x1e) > 0x50 { cpy9_s(0xf, 6) } else { cpy9_s(0x14, 1) });
        cpy9_atributo(e, 0x28, || faixa((0xa, 0xb), (0xf, 5), (0xa, 7)));
        cpy9_atributo(e, 0x29, || faixa((0xd, 8), (0xe, 7), (0xf, 6)));
        cpy9_atributo(e, 0x2a, || if sortear(0x64) != 0 { cpy9_s(0xa, 0xb) } else { cpy9_s(0xa, 5) });
        let mut limite = [0u8; 8];
        let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> *const u8 = core::mem::transmute(FN_DATE_DATE_549F70);
        let d = data(limite.as_mut_ptr(), 1, 1, 0x776, -1);
        let (dia, ano) = (read_i16(d as usize), read_i16(d as usize + 2) as u16);
        let ano_e = read_i16(e + 0xe) as u16;
        if ano > ano_e || (ano == ano_e && dia > read_i16(e + 0xc)) {
            write_i16(e + 0xe, sortear(10).wrapping_add(0x7a8) as i16);
            write_i16(e + 0xc, sortear(0x12c).wrapping_add(1) as i16);
        }
        i += 1;
    }
}

/// Comparador: lado a lado pelas fichas de todos os árbitros e pelo estado do sorteio.
pub(crate) unsafe extern "C" fn comparar_validar_arbitros() {
    let o = orig::<unsafe extern "C" fn()>(IDX_VALIDAR_ARBITROS);
    let total = read_i32(0xae2390).max(0) as usize;
    if read_usize(G_OFFICIAL_TABLE) == 0 {
        return o();
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((read_usize(G_OFFICIAL_TABLE), total * 0x2b));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| validar_arbitros()));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("validar_arbitros ({} fichas) diferencas em {:x?}", total, dif)
    });
}

/// copies a 0x2b-byte referee record field by field, the date through the date copy
/// Original: `Official::operator=` (0x79c530, officials_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x79c530 } (thiscall, ret 4, officials_manager.cpp): `Official::operator=(src)`, pela montagem dos árbitros do
/// jogo novo (0x79bf60, as fichas que ficam). Copia a ficha de 0x2b bytes campo a campo (+0, +4, +8; a data de +0xc
/// pela 0x417c10; +0x14, +0x16, +0x1a, +0x1e, +0x20, +0x22, os bytes +0x24..+0x2a). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_ficha_de_arbitro(this: usize, src: usize) -> usize {
    for o in [0usize, 4, 8] {
        write_usize(this + o, read_usize(src + o));
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(this + 0xc, t.as_mut_ptr(), src + 0xc);
    write_i16(this + 0x14, read_i16(src + 0x14));
    write_usize(this + 0x16, read_usize(src + 0x16));
    write_usize(this + 0x1a, read_usize(src + 0x1a));
    for o in [0x1eusize, 0x20, 0x22] {
        write_i16(this + o, read_i16(src + o));
    }
    for o in 0x24usize..0x2b {
        write_u8(this + o, read_u8(src + o));
    }
    this
}

/// Comparador: lado a lado pela ficha de destino e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_copiar_ficha_de_arbitro(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COPIAR_FICHA_DE_ARBITRO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x2b)], || o(this, src), || copiar_ficha_de_arbitro(this, src));
    PLACAR.registrar(igual && r == n, || format!("copiar_ficha_de_arbitro(src {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", src, r, mo, n, mr));
    r
}

/// writes [0xae2390] officials (0x11 bytes from [+0]), [0xae2350] nations (0x102 bytes from [+0xc]) and one byte per
/// official from [+8] into the off.dat item; returns 1 in the low byte; with no officials or on error the "Error" box,
/// closes the item and 0
/// Original: `OfficialsManager::save_official_stats` (0x79bb80, officials_manager.cpp, thiscall).
///
/// [pt] { 0x79bb80 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::save_official_stats(save)`: grava no item
/// `off.dat` a quantidade de árbitros [0xae2390], os registros de 0x11 bytes de [+0] (um por árbitro), a quantidade de
/// nações [0xae2350], os registros de 0x102 bytes de [+0xc] (um por nação) e um byte por árbitro de [+8]; devolve 1
/// (no byte baixo). Sem contêiner ou erro ao abrir: a caixa "Error" (officials_manager.cpp 0x142, 0x14a) e 0; sem
/// árbitros ou erro ao gravar: a caixa (0x150, 0x156, 0x15c, 0x164, 0x16c, 0x174), fecha o item e 0.
pub(crate) unsafe extern "thiscall" fn gravar_estatisticas_dos_arbitros(this: usize, save: usize) -> u8 {
    const FONTE: usize = 0xa6a22c;
    let arbitros = || *(G_OFFICIAL_COUNT as *const u32);
    let nacoes = || *(G_NATION_COUNT as *const u32);
    crate::comparacao::ramo("cpx_7 gravar estatisticas dos arbitros");
    if save == 0 {
        aviso_em(FONTE, 0x142);
        return 0;
    }
    let mut z = cpx7_abrir(save, 0xa6a260);
    if z == 0 {
        aviso_em(FONTE, 0x14a);
        return 0;
    }
    if cpx7_gravar(0xae2390, 4, 1, z) != 1 {
        return cpx7_falha_dos_arbitros(0x150, &mut z);
    }
    if arbitros() == 0 {
        return cpx7_falha_dos_arbitros(0x156, &mut z);
    }
    if cpx7_gravar(read_usize(this), 0x11, arbitros(), z) != arbitros() {
        return cpx7_falha_dos_arbitros(0x15c, &mut z);
    }
    if cpx7_gravar(0xae2350, 4, 1, z) != 1 {
        return cpx7_falha_dos_arbitros(0x164, &mut z);
    }
    if cpx7_gravar(read_usize(this + 0xc), 0x102, nacoes(), z) != nacoes() { // cpx7: os registros das nações
        return cpx7_falha_dos_arbitros(0x16c, &mut z);
    }
    if cpx7_gravar(read_usize(this + 8), 1, arbitros(), z) != arbitros() {
        return cpx7_falha_dos_arbitros(0x174, &mut z);
    }
    cpx7_fechar(&mut z);
    1
}

/// Comparador da 0x79bb80: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_gravar_estatisticas_dos_arbitros(this: usize, save: usize) -> u8 {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GRAVAR_ESTATISTICAS_DOS_ARBITROS)(this, save)
}
