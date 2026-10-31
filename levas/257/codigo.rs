// ---------- 257a leva: as linhas da nação-chave: a linha de uma nação, a linha do clube e o ano da temporada do clube (key_nation.cpp) ----------

unsafe fn cpu1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x669340 } (cdecl, key_nation.cpp): `key_nation_index_of_nation(nação)`: a primeira linha da tabela das 35
/// nações-chave (0xb63d60, 72 bytes, nação em +0) com a nação dada (sem conferir a nula), ou 4. Pela moral do jogo
/// 0x4cb250, pelas aposentadorias 0x7a7310, pelo amistoso livre 0x5c6b70, pelos prêmios do dia das nações e pelas vagas
/// da UEFA. Chamada a cada jogador: sem contador.
unsafe extern "C" fn linha_da_nacao(nacao: usize) -> u8 {
    for k in 0..0x23usize {
        if cpu1_d(0xb63d60 + 72 * k) == nacao {
            return k as u8; // cpu1: a linha da nacao dada
        }
    }
    4
}

unsafe extern "C" fn comparar_linha_da_nacao(nacao: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1194)(nacao);
    let n = linha_da_nacao(nacao);
    PLACAR.registrar(r == n, || format!("linha_da_nacao(nação {:x}) original={} rust={}", nacao, r, n));
    r
}

/// { 0x669360 } (cdecl, key_nation.cpp): `key_nation_index_of_club(clube)`. Clube que não é seleção (0x53b900), com
/// nação (+0x53) escolhida para jogar (+0x11c da nação): a linha da nação na tabela, ou a caixa "Error" (linha 0x26d) e
/// 0. Senão (a original chama 0x53b900 de novo): seleção, 1; a primeira linha cujo continente (+4) é o da nação do clube
/// (+0x71), ou 3 (sem nação, 3). Pelas aposentadorias 0x7a7310, pelo adiamento pelo tempo 0x923630, pelos dias até a
/// temporada 0x59b4c0, pelo contrato desejado 0x85fc50, pelo ano da temporada 0x669950 e outras. Sem contador.
unsafe extern "C" fn linha_da_nacao_do_clube(clube: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) == 0 {
        let nacao = cpu1_d(clube + 0x53);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) != 0 {
            for k in 0..0x23usize {
                if cpu1_d(0xb63d60 + 72 * k) == nacao {
                    return k as u8; // cpu1: a linha da nacao escolhida
                }
            }
            aviso_em(0x9eaff4, 0x26d);
            return 0;
        }
    }
    if selecao(clube) != 0 {
        return 1;
    }
    let nacao = cpu1_d(clube + 0x53); // cpu1: a nacao do continente
    for k in 0..0x23usize {
        if nacao != 0 && cpu1_d(0xb63d64 + 72 * k) == cpu1_d(nacao + 0x71) {
            return k as u8; // cpu1: a linha do continente
        }
    }
    3
}

/// Se o clube cai na caixa "Error" da 0x669360 (não é seleção, nação escolhida fora da tabela).
unsafe fn cpu1_caixa(clube: usize) -> bool {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nacao = cpu1_d(clube + 0x53);
    selecao(clube) == 0
        && nacao != 0
        && *((nacao + 0x11c) as *const u8) != 0
        && !(0..0x23usize).any(|k| cpu1_d(0xb63d60 + 72 * k) == nacao)
}

unsafe extern "C" fn comparar_linha_da_nacao_do_clube(clube: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(1195);
    if cpu1_caixa(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = linha_da_nacao_do_clube(clube);
    PLACAR.registrar(r == n, || format!("linha_da_nacao_do_clube(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x669950 } (cdecl, key_nation.cpp): `key_nation_get_club_season_year(clube)`: o ano de hoje ([0xae2c92]); para
/// clube que não é seleção, pela linha do clube (0x669360): com o dword +0x17 da linha não zero ou o dia de início
/// (+0x15, sem sinal) abaixo de 60, e o dia de hoje ([0xae2c90], com sinal) antes do dia de início, o ano anterior. A
/// original devolve só os 16 bits baixos com sentido (os altos são a sobra da vaga empurrada). Pelos registros do clube
/// 0x444930/0x447430/0x448ad0 e pela história do jogador 0x7c3e40/0x7c5fe0.
unsafe extern "C" fn ano_da_temporada_do_clube(clube: usize) -> u16 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    let ano = *(0xae2c92 as *const u16);
    if clube != 0 && selecao(clube) == 0 {
        let k = linha(clube) as i8 as isize;
        let e = (0xb63d60isize + 72 * k) as usize;
        let inicio = core::ptr::read_unaligned((e + 0x15) as *const u16);
        if (cpu1_d(e + 0x17) != 0 || inicio < 0x3c) && (*(0xae2c90 as *const i16) as i32) < inicio as i32 {
            return ano.wrapping_sub(1); // cpu1: o ano anterior
        }
    }
    ano
}

unsafe extern "C" fn comparar_ano_da_temporada_do_clube(clube: usize) -> u16 {
    let o = orig::<unsafe extern "C" fn(usize) -> u16>(1196);
    if clube != 0 && cpu1_caixa(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = ano_da_temporada_do_clube(clube);
    PLACAR.registrar(r == n, || format!("ano_da_temporada_do_clube(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 257a leva: a temporada do clube e a nação principal: se o clube está na temporada e as datas dela, o continente principal e a nação principal (key_nation.cpp) ----------

unsafe fn cpu2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Grava em `dst` (se não nulo) a data (dia, ano) montada pela 0x549ef0 (com o bissexto), como a cópia 0x417c10.
unsafe fn cpu2_data(dst: usize, dia: i16, ano: u16) {
    if dst == 0 {
        return;
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut b = [0u8; 8];
    data(b.as_mut_ptr(), dia as u16 as u32, ano as u32);
    core::ptr::copy_nonoverlapping(b.as_ptr(), dst as *mut u8, 8);
}

/// { 0x669bd0 } (cdecl, key_nation.cpp, quadro SEH só das datas locais): `key_nation_club_in_season(clube, &início,
/// &fim)`. A linha da nação do clube (+0x53) na tabela das 35 nações-chave (0xb63d60, 72 bytes; sem nação ou fora da
/// tabela, a linha 3 0xb63e38 e o dia de fim da data padrão 0x54a110, 40). `a` = o dia +0x2a da linha (início), `c` = o
/// dia da data +0x39 (fim; a original também copia os 7 bytes seguintes e, com o clube +0x1db ≠ −1 e o byte +0x43 3, 4
/// ou 5, troca dois deles, mas não os usa), `t` o dia de hoje ([0xae2c90]) e `y` o ano ([0xae2c92]); comparações de 16
/// bits com sinal. Com a > c (temporada que vira o ano): t > a, (a, y) e (c, y+1), 1; t < c, (a, y−1) e (c, y), 1; senão
/// (a, y) e (c, y), 0. Com a ≤ c: t < a, (a, y) e (c, y), 0; t > c, (a, y+1) e (c, y+1), 1; senão (a, y) e (c, y), 1. As
/// datas vão para `início` e `fim` quando não nulos. Pelo clube na temporada 0x547770, pela forma física 0x62a920,
/// 0x62c680 e 0x62ce30, pelo mês do fim da temporada 0x4d45f0 e pela data da transferência 0x8cee70. Chamada a cada
/// jogador: sem contador.
unsafe extern "C" fn clube_na_temporada(clube: usize, inicio: usize, fim: usize) -> u32 {
    let mut linha = 0xb63e38usize;
    let mut c: i16 = 0x28;
    let nacao = cpu2_d(clube + 0x53);
    if nacao != 0 {
        if let Some(k) = (0..0x23usize).find(|&k| cpu2_d(0xb63d60 + 72 * k) == nacao) {
            linha = 0xb63d60 + 72 * k;
            c = cpu2_w(linha + 0x39);
        }
    }
    let a = cpu2_w(linha + 0x2a);
    let t = *(0xae2c90 as *const i16);
    let y = *(0xae2c92 as *const u16);
    if a > c {
        if t > a {
            cpu2_data(inicio, a, y);
            if fim != 0 {
                cpu2_data(fim, c, y.wrapping_add(1)); // cpu2: o fim no ano seguinte
            }
            return 1;
        }
        if t < c {
            cpu2_data(inicio, a, y.wrapping_sub(1));
            cpu2_data(fim, c, y);
            return 1; // cpu2: antes do fim, na temporada
        }
        cpu2_data(inicio, a, y);
        cpu2_data(fim, c, y);
        return 0; // cpu2: entre o fim e o inicio, fora
    }
    if t < a {
        cpu2_data(inicio, a, y);
        cpu2_data(fim, c, y);
        return 0;
    }
    if t > c {
        cpu2_data(inicio, a, y.wrapping_add(1));
        cpu2_data(fim, c, y.wrapping_add(1));
        return 1;
    }
    cpu2_data(inicio, a, y);
    cpu2_data(fim, c, y);
    1 // cpu2: entre o inicio e o fim do ano
}

unsafe extern "C" fn comparar_clube_na_temporada(clube: usize, inicio: usize, fim: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(1197)(clube, inicio, fim);
    let (mut i, mut f) = ([0u8; 8], [0u8; 8]);
    let n = clube_na_temporada(
        clube,
        if inicio != 0 { i.as_mut_ptr() as usize } else { 0 },
        if fim != 0 { f.as_mut_ptr() as usize } else { 0 },
    );
    let ler = |p: usize| if p != 0 { *(p as *const [u8; 8]) } else { [0u8; 8] };
    let (oi, of) = (ler(inicio), ler(fim));
    let ok = r == n && (inicio == 0 || oi == i) && (fim == 0 || of == f);
    PLACAR.registrar(ok, || {
        format!("clube_na_temporada(clube {:x}) original={} {:02x?} {:02x?} rust={} {:02x?} {:02x?}", clube, r, oi, of, n, i, f)
    });
    r
}

/// { 0x6695c0 } (cdecl, key_nation.cpp): `key_nation_get_main_continent()`: o continente (+0x71) da nação do humano
/// ([0xae2d08]); sem ela ou sem continente, o da primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
unsafe extern "C" fn continente_principal() -> usize {
    let h = cpu2_d(0xae2d08);
    if h != 0 && cpu2_d(h + 0x71) != 0 {
        return cpu2_d(h + 0x71);
    }
    for k in 0..0x23usize {
        let n = cpu2_d(0xb63d60 + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return cpu2_d(n + 0x71);
        }
    }
    0
}

unsafe extern "C" fn comparar_continente_principal() -> usize {
    let r = orig::<unsafe extern "C" fn() -> usize>(1198)();
    let n = continente_principal();
    PLACAR.registrar(r == n, || format!("continente_principal() original={:x} rust={:x}", r, n));
    r
}

/// { 0x669600 } (cdecl, key_nation.cpp): `key_nation_get_main_nation()`: a nação do humano ([0xae2d08]); sem ela, a
/// primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
unsafe extern "C" fn nacao_principal() -> usize {
    let h = cpu2_d(0xae2d08);
    if h != 0 {
        return h;
    }
    for k in 0..0x23usize {
        let n = cpu2_d(0xb63d60 + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return n;
        }
    }
    0
}

unsafe extern "C" fn comparar_nacao_principal() -> usize {
    let r = orig::<unsafe extern "C" fn() -> usize>(1199)();
    let n = nacao_principal();
    PLACAR.registrar(r == n, || format!("nacao_principal() original={:x} rust={:x}", r, n));
    r
}

// ---------- 257a leva: as estatísticas do jogador: a estatística de uma categoria e as notas combinadas, a ordem dos idiomas e a ordem da história por clube (player_stats.cpp) ----------

unsafe fn cpu3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x7c1e30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_stat(jogador, categoria, qual, padrão)`, como
/// a original a deixa no x87. Sem jogador: a caixa "Error" (linha 0x659) e 0.0; id (+0) fora da tabela ([0xae236c]):
/// 0.0. Com `qual` ≠ 0x13 e a categoria (byte com sinal) até 6: o registro da categoria (0x7c3920) dá a estatística
/// (0x7c2150 com `qual` e `padrão`); sem registro, `padrão` para 0x12 e 0x14 e 0.0 para as outras. Senão as notas
/// combinadas pelas categorias 2+3 (jogos n e total t de +0xe: n < 20, ((20 − n)·5 + t)·0.05f; senão t/n; guardado em
/// f32), 4 (n < 8, ((8 − n)·3 + t)·0.125; senão t/n; f32) e 5 (n < 5, ((5 − n) + t)·0.2f; senão t/n; no x87): categoria
/// 100, (c5·1.5 + c4·2.5 + c23·6)·0.1; 101, (c5·3 + c4·5 + 2·c23)·0.1; 102, (c5·5 + c4·4 + c23)·0.1; as outras 0.0. Sem
/// o registro 4 ou 5 a original lê locais sem valor (a cópia usa zero; o comparador deixa esses casos com a original).
/// Pela escolha da escalação 0x6e2f20, pelo MATCH_PLAYER::setup 0x6eaea0, pela reputação do clube 0x545e90, pela força
/// do jogador 0x546ab0, pelo dia da comissão 0x5ce2d0 e por outras. Quente: sem contador.
unsafe extern "thiscall" fn estatistica_da_categoria(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    if jogador == 0 {
        aviso_em(0xa78310, 0x659);
        return 0.0;
    }
    if cpu3_d(jogador) as i32 >= cpu3_d(0xae236c) as i32 {
        return 0.0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    if qual as u8 != 0x13 && categoria as u8 as i8 <= 6 {
        let r = registro(this, jogador, categoria);
        if r != 0 {
            let valor: unsafe extern "stdcall" fn(usize, u32, u32) -> f64 = core::mem::transmute(0x7c2150usize);
            return valor(r, qual, padrao.to_bits()); // cpu3: o valor do registro
        }
        if qual as u8 == 0x12 || qual as u8 == 0x14 {
            return padrao as f64; // cpu3: sem registro, o padrao
        }
        return 0.0; // cpu3: sem registro, zero
    }
    let (mut n, mut t) = (0u8, 0i16);
    let r2 = registro(this, jogador, 2);
    if r2 != 0 {
        n = *(r2 as *const u8);
        t = cpu3_w(r2 + 0xe);
    }
    let r3 = registro(this, jogador, 3);
    if r3 != 0 {
        n = n.wrapping_add(*(r3 as *const u8));
        t = t.wrapping_add(cpu3_w(r3 + 0xe));
    }
    let c23 = if n < 0x14 {
        (((0x14 - n as i32) as f64 * 5.0 + t as f64) * (0.05f32 as f64)) as f32
    } else {
        (t as f64 / n as f64) as f32
    };
    // ponytail: sem o registro 4 a original lê a vaga do byte n com os 3 bytes de cima sem valor; a cópia usa zero
    let mut c4 = f32::from_bits(n as u32);
    let r4 = registro(this, jogador, 4);
    if r4 != 0 {
        let k = *(r4 as *const u8);
        let v = cpu3_w(r4 + 0xe) as f64;
        c4 = if k < 8 { (((8 - k as i32) as f64 * 3.0 + v) * 0.125) as f32 } else { (v / k as f64) as f32 };
    }
    let mut c5 = 0.0f64; // ponytail: sem o registro 5, a original carrega uma vaga sem valor; a cópia usa zero
    let r5 = registro(this, jogador, 5);
    if r5 != 0 {
        let k = *(r5 as *const u8);
        let v = cpu3_w(r5 + 0xe) as f64;
        c5 = if k < 5 { ((5 - k as i32) as f64 + v) * (0.2f32 as f64) } else { v / k as f64 };
    }
    let (c23, c4) = (c23 as f64, c4 as f64);
    match categoria as u8 as i8 as i32 {
        100 => ((c5 * 1.5 + c4 * 2.5) + c23 * 6.0) * 0.1,
        101 => ((c5 * 3.0 + c4 * 5.0) + (c23 + c23)) * 0.1,
        102 => ((c5 * 5.0 + c4 * 4.0) + c23) * 0.1,
        _ => 0.0,
    }
}

unsafe extern "thiscall" fn comparar_estatistica_da_categoria(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64>(1200);
    if jogador == 0 || (cpu3_d(jogador) as i32) >= cpu3_d(0xae236c) as i32 {
        return o(this, jogador, categoria, qual, padrao);
    }
    let c = categoria as u8 as i8;
    let simples = qual as u8 != 0x13 && c <= 6;
    // ponytail: categoria fora de 1..6 cai na caixa da 0x7c3920; nas combinadas sem o registro 4 ou 5, locais sem valor
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    if (simples && !(1..=6).contains(&c))
        || (!simples && (100..=102).contains(&(c as i32)) && (registro(this, jogador, 4) == 0 || registro(this, jogador, 5) == 0))
    {
        return o(this, jogador, categoria, qual, padrao);
    }
    let r = o(this, jogador, categoria, qual, padrao);
    let n = estatistica_da_categoria(this, jogador, categoria, qual, padrao);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || {
        format!("estatistica_da_categoria(jogador {}, categoria {}, qual {}, padrão {}) original={} rust={}", cpu3_d(jogador) as i32, c, qual as u8, padrao, r, n)
    });
    r
}

/// { 0x7c6d40 } (cdecl, player_stats.cpp): `player_stats_compare_language_entries(a, b)` (o qsort e o bsearch dos
/// idiomas, registros de 6 bytes {id, idioma, fluência}): a nulo, −1; b nulo, 1; ids (+0) diferentes, a diferença;
/// idioma 0xff em qualquer um, 0; senão a diferença dos idiomas (+4, com sinal). Pela fluência no idioma 0x7c6830 e pelos
/// idiomas acrescentados, tirados e melhorados.
unsafe extern "C" fn ordem_dos_idiomas(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (ia, ib) = (cpu3_d(a) as i32, cpu3_d(b) as i32);
    if ia != ib {
        return ia.wrapping_sub(ib);
    }
    let (la, lb) = (*((a + 4) as *const u8), *((b + 4) as *const u8));
    if la == 0xff || lb == 0xff {
        return 0;
    }
    la as i8 as i32 - lb as i8 as i32
}

unsafe extern "C" fn comparar_ordem_dos_idiomas(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1201)(a, b);
    let n = ordem_dos_idiomas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_idiomas({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x7c6d90 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_club(a, b)` (o qsort da história na carga
/// das estatísticas 0x7bfd50): a nula, 1; b nula, −1; pelo dword +0xa e depois pelo +4, sem sinal: maior 1, menor −1,
/// iguais 0.
unsafe extern "C" fn ordem_da_historia_por_clube(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ua, ub) = (cpu3_d(a + 0xa) as u32, cpu3_d(b + 0xa) as u32);
    if ua != ub {
        return if ua > ub { 1 } else { -1 };
    }
    let (va, vb) = (cpu3_d(a + 4) as u32, cpu3_d(b + 4) as u32);
    if va > vb {
        1
    } else if va < vb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn comparar_ordem_da_historia_por_clube(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1202)(a, b);
    let n = ordem_da_historia_por_clube(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_historia_por_clube({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 257a leva: as cláusulas do contrato: a cláusula de rescisão ativa, o período protegido da FIFA e o tipo de bônus permitido (staff_contracts.cpp) ----------

unsafe fn cpu4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x867b50 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::has_active_release_clause()`: 1 com a cláusula +0x1c
/// = 2, a +0x1d = 1 ou a +0x1f = 2; senão 0. Pelo dia dos contratos 0x4c7ac0, pelo valor do jogador 0x4d0c60, pelo preço
/// pedido 0x4d2070, pela resposta prevista do vendedor 0x84af00, pelo vendedor que tem de aceitar 0x8cd240 e pelo lance
/// 0x8e5d60. Chamada a cada contrato: sem contador.
unsafe extern "thiscall" fn clausula_de_rescisao_ativa(c: usize) -> u32 {
    if cpu4_b(c + 0x1c) == 2 || cpu4_b(c + 0x1d) == 1 || cpu4_b(c + 0x1f) == 2 {
        return 1; // cpu4: com a clausula ativa
    }
    0
}

unsafe extern "thiscall" fn comparar_clausula_de_rescisao_ativa(c: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1203)(c);
    let n = clausula_de_rescisao_ativa(c);
    PLACAR.registrar(r == n, || format!("clausula_de_rescisao_ativa({:x}) original={} rust={}", c, r, n));
    r
}

/// { 0x868210 } (thiscall, ret 0xc, staff_contracts.cpp, quadro SEH só da data local):
/// `STAFF_CONTRACT::check_fifa_protected_period(_, &no_limite, &anos)`: zera `no_limite` (se não nulo). Sem as regras
/// novas da FIFA ([0xae233c] ≠ 1), 1. Contrato (data +0x25) anterior à data das regras ([0xae2330]: ano sem sinal, dia
/// com sinal), 1. Senão o período protegido: 3 anos com a idade da pessoa ([0xae23bc] + 110 × [+0]) na data do contrato
/// (0x53ab90) abaixo de 28, senão 2, em `anos` (se não nulo); a duração de hoje até a data (0x54cc30, duas vezes, como a
/// original) com menos anos que o período, 1; com os anos exatos e 0 dias, `no_limite` = 1; 0. Pelo dia dos contratos
/// 0x4c7ac0, pelo lance 0x8e5d60, pela troca com compensação 0x84b6f0 e por telas. Sem contador.
unsafe extern "thiscall" fn periodo_protegido(c: usize, _x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    if !no_limite.is_null() {
        *no_limite = 0;
    }
    if cpu4_b(0xae233c) != 1 {
        return 1; // cpu4: sem as regras novas
    }
    let (rdia, rano) = (*(0xae2330 as *const i16), *(0xae2332 as *const u16));
    let d = c + 0x25;
    let (cdia, cano) = (core::ptr::read_unaligned(d as *const i16), core::ptr::read_unaligned((d + 2) as *const u16));
    if rano > cano || (rano == cano && rdia > cdia) {
        return 1;
    }
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let idade: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53ab90usize);
    let (d0, d1) = (cpu4_d(d) as u32, cpu4_d(d + 4) as u32);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    let pessoa = cpu4_d(0xae23bc).wrapping_add(cpu4_d(c).wrapping_mul(110));
    let limite: u8 = if (idade(pessoa, d) as u8 as i8) < 0x1c { 3 } else { 2 }; // cpu4: o periodo pela idade
    if !anos.is_null() {
        *anos = limite;
    }
    let mut l = [0u8; 8];
    let p = diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    core::ptr::copy_nonoverlapping(p as *const u8, l.as_mut_ptr(), 8);
    let a = u16::from_le_bytes([l[2], l[3]]) as i32;
    if a < limite as i32 {
        return 1; // cpu4: dentro do periodo
    }
    if !no_limite.is_null() && a == limite as i32 && u16::from_le_bytes([l[0], l[1]]) == 0 {
        *no_limite = 1;
    }
    0
}

unsafe extern "thiscall" fn comparar_periodo_protegido(c: usize, x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    // as saídas como estavam antes: a cópia parte delas (a original só grava `anos` no período)
    let (mut nl, mut na) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) -> u32>(1204)(c, x, no_limite, anos);
    let (ol, oa) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let n = periodo_protegido(
        c,
        x,
        if no_limite.is_null() { core::ptr::null_mut() } else { &mut nl },
        if anos.is_null() { core::ptr::null_mut() } else { &mut na },
    );
    let ok = r == n && (no_limite.is_null() || ol == nl) && (anos.is_null() || oa == na);
    PLACAR.registrar(ok, || format!("periodo_protegido({:x}) original={} {} {} rust={} {} {}", c, r, ol, oa, n, nl, na));
    r
}

/// { 0x867aa0 } (cdecl, staff_contracts.cpp): `staff_contract_is_bonus_type_allowed(pessoa, tipo)`: sem a ficha de
/// jogador (+0x61), 0; goleiro (+0xf da ficha = 0x14), 1 só para o tipo 2; senão 1 para os tipos que não são 2. Pela
/// tela do contrato.
unsafe extern "C" fn tipo_de_bonus_permitido(pessoa: usize, tipo: u32) -> u32 {
    let f = cpu4_d(pessoa + 0x61);
    if f == 0 {
        return 0;
    }
    if cpu4_b(f + 0xf) == 0x14 {
        return (tipo as u8 == 2) as u32;
    }
    (tipo as u8 != 2) as u32
}

unsafe extern "C" fn comparar_tipo_de_bonus_permitido(pessoa: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u32>(1205)(pessoa, tipo);
    let n = tipo_de_bonus_permitido(pessoa, tipo);
    PLACAR.registrar(r == n, || format!("tipo_de_bonus_permitido({:x}, {}) original={} rust={}", pessoa, tipo as u8, r, n));
    r
}

// ---------- 257a leva: o adiamento pelo clima: a decisão de adiar ou abandonar o jogo, as finanças do clube e a ordem das reservas do estádio (weather.cpp, finance.cpp, stadium.cpp) ----------

unsafe fn cpu5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpu5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O fim do adiamento: sorteio(n) diferente de zero não adia (0), zero abandona (6).
unsafe fn cpu5_abandonar_se_zero(n: i32) -> u8 {
    if cpu5_sortear(n) != 0 {
        0
    } else {
        6
    }
}

/// O fim do adiamento: sorteio(n) zero não adia (0), diferente de zero adia (5).
unsafe fn cpu5_adiar_se_nao_zero(n: i32) -> u8 {
    if cpu5_sortear(n) == 0 {
        0
    } else {
        5
    }
}

/// { 0x923630 } (thiscall, ret 4, weather.cpp): `Weather::check_postponement(jogo)`, pelo dia dos jogos (0x6b3c10) e
/// pela preparação da partida (0x6b7f20). Sem estádio (+0x18) com cidade (+0x38) e lugares (+0x3c), sem competição
/// (+0x14), com o mandante (+0x1c) fora de liga (+0x57) ou o jogo fora da liga dele, ou estádio coberto (+0x4c): 0. Na
/// nação-chave do mandante (0x669360, tabela 0xb63d60 de 0x48 bytes), perto do fim da temporada do ano (+0x39/+0x3b): 0.
/// Na fatia da noite (+0x3f = 2), pelas finanças do mandante (0x5a0590, faixa 0x5986b0): sorteio(1000/2500/5000) zero
/// devolve 0x12. Depois o clima do jogo (0x923010, gravado na vaga do argumento como na original): chuva 5 com 1 a 13
/// graus, estádio descoberto (+0x4d) de até 20000 lugares: sorteio por lugares e mandante (+0x64), zero = 9; chuva 8
/// (até 15000 lugares) e 9 (pelo vento 4 e por 10000 lugares): uma cadeia de sorteios que devolve 0, 5 ou 6.
unsafe extern "thiscall" fn adiar_pelo_clima(this: usize, jogo: usize) -> u8 {
    let chave: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(0x669360usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x923010usize);
    let est = cpu5_d(jogo + 0x18);
    if est == 0 || cpu5_d(est + 0x38) == 0 || cpu5_d(est + 0x3c) == 0 || cpu5_d(jogo + 0x14) == 0 {
        return 0;
    }
    let casa = cpu5_d(jogo + 0x1c);
    if cpu5_d(casa + 0x57) == 0 {
        return 0;
    }
    let k: i8 = if casa != 0 { chave(casa) } else { -1 };
    if cpu5_d(jogo + 0x14) != cpu5_d(casa + 0x57) || cpu5_b(est + 0x4c) != 0 {
        return 0;
    }
    if (0..0x23).contains(&k) {
        let e = k as usize * 0x48;
        let fim = core::ptr::read_unaligned((0xb63d99 + e) as *const i16);
        let ano = core::ptr::read_unaligned((0xb63d9b + e) as *const u16);
        let dia = core::ptr::read_unaligned(0xae2c90 as *const i16);
        let ano_atual = core::ptr::read_unaligned(0xae2c92 as *const u16);
        if fim > 0x3c {
            if ano == ano_atual && (fim as i32 - 0x3c) < dia as i32 {
                return 0;
            }
        } else if ano == ano_atual && dia > 0x154 {
            return 0;
        }
    }
    if cpu5_b(jogo + 0x3f) == 2 { // cpu5: a fatia da noite
        let fin = financas(cpu5_d(0xae2a38), casa);
        if fin != 0 {
            let f = faixa(fin) as i8;
            let n = if f == -2 {
                1000
            } else if f == -1 && (cpu5_b(fin + 0x166) as i8) > 0xf {
                0x9c4
            } else {
                5000
            };
            crate::comparacao::ramo("cpu_5 rolagem das financas");
            if cpu5_sortear(n) == 0 { // cpu5: a rolagem das finanças
                return 0x12;
            }
        }
    }
    // ponytail: a original grava o clima na vaga do próprio argumento; se o clima não grava (sem regiões), ficam os
    // bytes do ponteiro do jogo, como aqui.
    let mut cl = jogo as u32;
    clima(this, &mut cl as *mut u32 as usize, jogo);
    let [vento, chuva, temp, _] = cl.to_le_bytes();
    let temp = temp as i8;
    let est = cpu5_d(jogo + 0x18);
    let casa = cpu5_d(jogo + 0x1c);
    match chuva {
        5 => {
            if !(1..0xe).contains(&temp) || cpu5_b(est + 0x4d) != 0 {
                return 0;
            }
            let cap = cpu5_i(est + 0x3c);
            if cap > 0x4e20 {
                return 0;
            }
            let mandante = cpu5_b(casa + 0x64) == 1;
            let n = if cap > 0x30d4 {
                0xfa
            } else if cap > 0x1388 {
                if mandante { 0x96 } else { 0x1e }
            } else if mandante {
                0x32
            } else {
                0xa
            };
            crate::comparacao::ramo("cpu_5 chuva forte");
            if cpu5_sortear(n) != 0 { // cpu5: o sorteio da chuva forte
                0
            } else {
                9
            }
        }
        8 => {
            if cpu5_b(est + 0x4d) != 0 {
                return 0;
            }
            let cap = cpu5_i(est + 0x3c);
            if cap > 0x3a98 {
                return 0;
            }
            let mandante = cpu5_b(casa + 0x64) == 1;
            if cap > 0x1d4c && mandante {
                if cpu5_sortear(4) != 0 {
                    return 5;
                }
                return cpu5_abandonar_se_zero(4);
            }
            if cap <= 0x1d4c && !mandante {
                if cpu5_sortear(0x19) != 0 {
                    return 5;
                }
                return if cpu5_sortear(3) == 0 { 0 } else { 6 };
            }
            if cpu5_sortear(0x1e) == 0 {
                return 6;
            }
            cpu5_adiar_se_nao_zero(0xf)
        }
        9 => {
            let (d1, d2, d3) = if vento == 4 { (10, 0x14, 10) } else { (0x14, 10, 4) };
            if cpu5_b(est + 0x4d) != 0 && cpu5_sortear(d1) != 0 {
                return 0;
            }
            if cpu5_i(est + 0x3c) > 0x2710 {
                if vento == 4 {
                    if cpu5_sortear(d2) != 0 {
                        return 5;
                    }
                    return cpu5_abandonar_se_zero(d3);
                }
                if cpu5_sortear(d2) == 0 {
                    return cpu5_abandonar_se_zero(d3);
                }
                return 5;
            }
            if cpu5_sortear(0x32) == 0 {
                return 6;
            }
            cpu5_adiar_se_nao_zero(if vento == 4 { 8 } else { 4 })
        }
        _ => 0,
    }
}

// ponytail: sorteia: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_adiar_pelo_clima(this: usize, jogo: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1206)(this, jogo)
}

/// { 0x5a0590 } (thiscall, ret 4, finance.cpp): `FinanceTable::get_club_finance(clube)`: o registro de finanças
/// (0x167 bytes em [this]) do clube pelo id (+0); clube B com parceiro A (0x540a50 com o byte devolvido 0): o do A.
/// Clube nulo: a caixa "Error" de finance.CPP (a original).
unsafe extern "thiscall" fn financas_do_clube(this: usize, clube: usize) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1207)(this, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let mut c = clube;
    let p = parceiro(clube, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        c = p;
    }
    cpu5_d(this).wrapping_add((cpu5_d(c) as u32).wrapping_mul(0x167) as usize)
}

/// Comparador: pelo valor (clube nulo: só a original, com a caixa de erro).
unsafe extern "thiscall" fn comparar_financas_do_clube(this: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1207);
    if clube == 0 {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = financas_do_clube(this, clube);
    PLACAR.registrar(r == n, || format!("financas_do_clube(clube {:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

/// { 0x85e380 } (cdecl, stadium.cpp): `stadium_compare_bookings(a, b)`, a ordem das reservas do estádio (qsort em
/// 0x85ba1b, 0x85c4ba, 0x85ddf3, 0x85df26): pela data (+4, Date::operator< 0x456770 e > 0x456740), depois pelo id do
/// estádio (+0).
unsafe extern "C" fn ordem_das_reservas_do_estadio(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpu5_d(b + 4) as u32, cpu5_d(b + 8) as u32);
    if antes(a + 4, d0, d1) != 0 {
        return -1;
    }
    if depois(a + 4, d0, d1) != 0 {
        return 1;
    }
    cpu5_i(a).wrapping_sub(cpu5_i(b))
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_ordem_das_reservas_do_estadio(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1208)(a, b);
    let n = ordem_das_reservas_do_estadio(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_reservas_do_estadio({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 257a leva: o clima da partida: o vento, a chuva e a temperatura do jogo, a temperatura pela faixa e o estado do gramado (weather.cpp) ----------

unsafe fn cpu6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu6_s(a: usize) -> i8 {
    *(a as *const i8)
}

/// A região do clima de uma cidade (+0x34) se for válida (0 ≤ r < [this + 8]): [this] + r × 0x6e.
unsafe fn cpu6_regiao(this: usize, cidade: usize) -> Option<usize> {
    let r = cpu6_i(cidade + 0x34);
    if r >= 0 && r < cpu6_i(this + 8) {
        Some(cpu6_d(this).wrapping_add(r as usize * 0x6e))
    } else {
        None
    }
}

/// { 0x923010 } (thiscall, ret 8, weather.cpp): `Weather::get_match_weather(saida, jogo)`, na preparação da partida
/// (0x6b7f20, em +0x4756 do motor: o vento pesa nos chutes de longe), no adiamento (0x923630) e nas telas. Sem regiões
/// ([this], [this + 8]): nada. A cidade: a do estádio (+0x18 → +0x38), senão a do estádio do mandante (+0x1c → +0x69 →
/// +0x38), senão a capital da nação dele (+0x53 → +0x7a). Com cidade: saida[3] = o vento da cidade (tabela [this + 4], 4
/// bytes por cidade, byte 3) e a região dela (senão a da capital da nação +0x1f, senão a primeira). O período pelo dia do
/// jogo (+0x2a; sem jogo o dia do jogo) contra os limites +0x1e/+0x31/+0x44/+0x57 da região (3, 0, 1, 2, 3); no período
/// (19 bytes a partir de +0x20): saida[0] = vento pela leitura 2 da cidade (+0x20..+0x24: 0..4), saida[1] = chuva pela
/// leitura 1 (+0x25..+0x29: 0, 1, 2, 3, 5), e a faixa da temperatura pela leitura 2 (+0x2a..+0x30: 1, 0, 2..6, senão 3).
/// Sem cidade: saida[3], [0] e [1] = 0 e a faixa 2. saida[2] = a temperatura (0x9232c0); abaixo de zero, a chuva vira
/// neve/gelo: 1 → 6, 2 e 3 → 6/7/8 e 5 → 8/7/9 pelo vento da cidade 0/1/outro.
unsafe extern "thiscall" fn clima_da_partida(this: usize, s: *mut u8, jogo: usize) {
    let temperatura: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x9232c0usize);
    if cpu6_d(this) == 0 || cpu6_d(this + 8) == 0 {
        return;
    }
    let mut cid = 0usize;
    if jogo != 0 {
        let est = cpu6_d(jogo + 0x18);
        if est != 0 {
            cid = cpu6_d(est + 0x38);
        }
    }
    if cid == 0 {
        let casa = cpu6_d(jogo + 0x1c);
        if casa != 0 {
            let c = cpu6_d(casa + 0x69);
            if c != 0 && cpu6_d(c + 0x38) != 0 {
                cid = cpu6_d(c + 0x38);
            } else {
                let n = cpu6_d(casa + 0x53);
                if n != 0 && cpu6_d(n + 0x7a) != 0 {
                    cid = cpu6_d(n + 0x7a);
                }
            }
        }
    }
    let cidades = cpu6_d(this + 4);
    let mut reg = cpu6_d(this);
    if cid != 0 {
        let leituras = cidades.wrapping_add(cpu6_d(cid).wrapping_mul(4));
        *s.add(3) = *((leituras + 3) as *const u8);
        if let Some(r) = cpu6_regiao(this, cid) {
            reg = r;
        } else {
            let n = cpu6_d(cid + 0x1f);
            if n != 0 {
                let capital = cpu6_d(n + 0x7a);
                if capital != 0 {
                    if let Some(r) = cpu6_regiao(this, capital) {
                        reg = r;
                    }
                }
            }
        }
    } else {
        *s.add(3) = 0;
    }
    let dia = if jogo != 0 {
        core::ptr::read_unaligned((jogo + 0x2a) as *const i16)
    } else {
        core::ptr::read_unaligned(0xae2c90 as *const i16)
    };
    let limite = |o: usize| core::ptr::read_unaligned((reg + o) as *const i16);
    let p: usize = if dia < limite(0x1e) {
        3
    } else if dia < limite(0x31) {
        0
    } else if dia < limite(0x44) {
        1
    } else if dia < limite(0x57) {
        2
    } else {
        3
    };
    let faixa: u32 = if cid == 0 {
        *s = 0;
        *s.add(1) = 0;
        2
    } else {
        let per = reg + 19 * p;
        let lim = |o: usize| cpu6_s(per + o);
        let leitura = |k: usize| cpu6_s(cidades.wrapping_add(cpu6_d(cid).wrapping_mul(4)) + k);
        let a = leitura(2);
        *s = if a < lim(0x20) {
            0
        } else if a < lim(0x21) {
            crate::comparacao::ramo("cpu_6 vento fraco");
            1 // cpu6: o vento fraco
        } else if a < lim(0x22) {
            crate::comparacao::ramo("cpu_6 vento moderado");
            2 // cpu6: o vento moderado
        } else if a < lim(0x23) {
            crate::comparacao::ramo("cpu_6 vento forte");
            3 // cpu6: o vento forte
        } else if a >= lim(0x24) {
            0
        } else {
            4
        };
        let c = leitura(1);
        *s.add(1) = if c < lim(0x25) {
            0
        } else if c < lim(0x26) {
            1
        } else if c < lim(0x27) {
            2
        } else if c < lim(0x28) {
            3
        } else if c < lim(0x29) {
            5
        } else {
            0
        };
        let t = leitura(2);
        if t < lim(0x2a) {
            1
        } else if t < lim(0x2b) {
            0
        } else if t < lim(0x2c) {
            2
        } else if t < lim(0x2d) {
            3
        } else if t < lim(0x2e) {
            4
        } else if t < lim(0x2f) {
            5
        } else if t < lim(0x30) {
            6
        } else {
            3
        }
    };
    let t = temperatura(this, faixa, jogo) as i8;
    *s.add(2) = t as u8;
    if t >= 0 {
        return;
    }
    let vento = *s.add(3) as i8;
    match *s.add(1) {
        1 => *s.add(1) = 6,
        2 | 3 => *s.add(1) = match vento {
            0 => 6,
            1 => 7,
            _ => 8,
        },
        5 => *s.add(1) = match vento {
            0 => 8,
            1 => 7,
            _ => 9,
        },
        _ => {}
    }
}

/// Comparador: lado a lado nos 4 bytes da saída.
unsafe extern "thiscall" fn comparar_clima_da_partida(this: usize, s: *mut u8, jogo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(1209);
    let regioes = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, s, jogo), || clima_da_partida(this, s, jogo));
    PLACAR.registrar(igual, || format!("clima_da_partida(jogo {:#x}) {:x?} x {:x?}", jogo, mo.0, mr.0));
}

/// { 0x9232c0 } (thiscall, ret 8, weather.cpp): `Weather::get_temperature(faixa, jogo)`, pelo clima da partida. Com o
/// dia d (do jogo +0x2a, sem sinal; sem jogo o dia do jogo) e x = d × faixa: faixa 0: x % 6 + 1; 1: x % 6 − 5; 2:
/// x % 7 + 7; 3: x % 8 + 14; 4: x % 7 + 22; 5: x % 6 + 29; 6: x % 6 + 35; outra: −100. Jogo da fatia da noite (+0x3f = 2):
/// abaixo de 10, − 2; senão × 0,8 truncado. (Lê +0x3f do jogo mesmo sem jogo, como a original.)
unsafe extern "thiscall" fn temperatura_do_jogo(_this: usize, faixa: u32, jogo: usize) -> u8 {
    let f = faixa as u8 as i8 as i32;
    let d: i32 = if jogo != 0 {
        core::ptr::read_unaligned((jogo + 0x2a) as *const u16) as i32
    } else {
        core::ptr::read_unaligned(0xae2c90 as *const i16) as i32
    };
    let x = d.wrapping_mul(f);
    let t: i8 = match f {
        0 => (x % 6 + 1) as i8,
        1 => (x % 6 - 5) as i8,
        2 => (x % 7 + 7) as i8,
        3 => (x % 8 + 14) as i8,
        4 => (x % 7 + 0x16) as i8,
        5 => (x % 6 + 0x1d) as i8,
        6 => (x % 6 + 0x23) as i8,
        _ => -100,
    };
    if *((jogo + 0x3f) as *const u8) == 2 {
        if t < 10 {
            return (t as u8).wrapping_sub(2);
        }
        return ftol(t as f64 * f64_em(0x9689b0)) as u8;
    }
    t as u8
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_temperatura_do_jogo(this: usize, faixa: u32, jogo: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(1210)(this, faixa, jogo);
    let n = temperatura_do_jogo(this, faixa, jogo);
    PLACAR.registrar(r == n, || format!("temperatura_do_jogo(faixa {}, jogo {:#x}) original={} rust={}", faixa as u8 as i8, jogo, r as i8, n as i8));
    r
}

/// { 0x923970 } (thiscall, ret 8, weather.cpp): `Weather::get_pitch_condition(clima, jogo)`, na preparação da partida
/// (0x6b7f20, em +1 do motor: o gramado 2 e 0x20 pesam nos chutes). Sem jogo, sem estádio (+0x18) ou estádio coberto
/// (+0x4c): 0. h = (dia +0x2a + +0x10 + +0xc + +0) % 20. Pela chuva clima[1] e a temperatura clima[2] (1 a 28 graus),
/// com o estádio de marca +0x4d e os lugares +0x3c: chuva 5: 2/1 ou 8/1; chuva 1: 2 ou 0 por h < 6/4; chuva 2 e 3: por
/// h < 4/1/3; chuva 4: 2 ou 1/2; abaixo de 1 grau, chuva 1 e 8: 2, 0x10 ou 1; chuva 9: 2, 1 ou 0x20/1; sem chuva a 29
/// graus ou mais: 0x40 ou 4; senão 0.
unsafe extern "thiscall" fn gramado_do_jogo(_this: usize, clima: *const u8, jogo: usize) -> u8 {
    if jogo == 0 {
        return 0;
    }
    let est = cpu6_d(jogo + 0x18);
    if est == 0 || *((est + 0x4c) as *const u8) != 0 {
        return 0;
    }
    let soma = (core::ptr::read_unaligned((jogo + 0x2a) as *const u16) as i32)
        .wrapping_add(cpu6_i(jogo + 0x10))
        .wrapping_add(cpu6_i(jogo + 0xc))
        .wrapping_add(cpu6_i(jogo));
    let h = soma % 20;
    let chuva = *clima.add(1);
    let t = *clima.add(2) as i8;
    let marca = *((est + 0x4d) as *const u8) != 0;
    let lugares = cpu6_i(est + 0x3c);
    let morno = (1..0x1d).contains(&t);
    if chuva == 5 && morno {
        if marca {
            return if lugares > 0x6b6c { 2 } else { 1 };
        }
        return if lugares <= 0x445c { 8 } else { 1 };
    }
    if chuva == 1 && morno {
        if !marca {
            return 2;
        }
        let corte = if lugares > 0x6b6c { 4 } else { 6 };
        return if h >= corte { 0 } else { 2 };
    }
    if (chuva == 2 || chuva == 3) && morno {
        if marca {
            if lugares <= 0x6b6c {
                return if h >= 4 { 0 } else { 2 };
            }
            return if h != 0 { 0 } else { 2 };
        }
        return if h >= 3 { 0 } else { 2 };
    }
    if chuva == 4 && morno {
        if marca {
            if lugares <= 0x6b6c {
                return 2;
            }
            return if h >= 3 { 0 } else { 2 };
        }
        return if lugares > 0x445c { 2 } else { 1 };
    }
    if t < 1 && (chuva == 1 || chuva == 8) {
        if marca || lugares > 0xafc8 {
            return 2;
        }
        return if lugares <= 0x30d4 { 0x10 } else { 1 };
    }
    if chuva == 9 {
        if marca {
            return if lugares > 0x7530 { 2 } else { 1 };
        }
        return if lugares <= 0x4e20 { 0x20 } else { 1 };
    }
    if chuva == 0 && t >= 0x1d {
        return if lugares <= 0x88b8 { 0x40 } else { 4 };
    }
    0
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_gramado_do_jogo(this: usize, clima: *const u8, jogo: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(1211)(this, clima, jogo);
    let n = gramado_do_jogo(this, clima, jogo);
    PLACAR.registrar(r == n, || format!("gramado_do_jogo(clima {:x?}, jogo {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), jogo, r, n));
    r
}

// ---------- 257a leva: a riqueza do clube e duas ordens: a faixa das finanças, a ordem dos jogadores pela habilidade e a dos estádios pela capacidade (finance.cpp, game.cpp, stadium.cpp) ----------

/// { 0x5986b0 } (thiscall, finance.cpp): `Finance::get_wealth_band()`, a faixa do saldo (64 bits em +0/+4) do clube (+8,
/// vetor 0xae23b4) pela reputação R (+0x80), usada pelas transferências, pela diretoria, pelos técnicos, pelas finanças e
/// pelo adiamento. Seleção (0x53b900): 0. Acima de 2500·R + 5.000.000: 2; acima de 1000·R + 3.500.000: 1; abaixo de
/// −2.500.000 − 1500·R: −2; abaixo de −250·R (R < 5000) ou de (3750 − R)·1000 (R ≥ 5000): −1; senão 0.
unsafe extern "thiscall" fn faixa_de_riqueza(this: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let id = core::ptr::read_unaligned((this + 8) as *const u32);
    let clube = core::ptr::read_unaligned(0xae23b4 as *const usize).wrapping_add(id.wrapping_mul(0x245) as usize);
    if selecao(clube) != 0 {
        return 0;
    }
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    let r = rep as i32;
    let saldo = core::ptr::read_unaligned(this as *const i64);
    if saldo > r.wrapping_mul(2500).wrapping_add(5_000_000) as i64 {
        crate::comparacao::ramo("cpu_7 rica");
        return 2; // cpu7: a faixa rica
    }
    if saldo > r.wrapping_mul(1000).wrapping_add(3_500_000) as i64 {
        crate::comparacao::ramo("cpu_7 confortavel");
        return 1; // cpu7: a faixa confortável
    }
    if saldo < (-2_500_000i32).wrapping_sub(r.wrapping_mul(1500)) as i64 {
        return 0xfe;
    }
    let devendo = if rep < 0x1388 { r.wrapping_mul(-250) } else { (0xea6 - r).wrapping_mul(1000) };
    if saldo < devendo as i64 {
        crate::comparacao::ramo("cpu_7 devendo");
        return 0xff; // cpu7: a faixa devendo
    }
    0
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_faixa_de_riqueza(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(1212)(this);
    let n = faixa_de_riqueza(this);
    PLACAR.registrar(r == n, || format!("faixa_de_riqueza(financas {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}

/// { 0x5d6220 } (cdecl, game.cpp): `compare_players_by_ability(a, b)`, o qsort dos dados de jogador em
/// game_daily_staff_update (0x5d588f): pela habilidade atual (+5), depois pela potencial (+7), crescente.
unsafe extern "C" fn ordem_por_habilidade(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    let h = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16) as i32;
    if h(pa, 5) == h(pb, 5) {
        return h(pa, 7) - h(pb, 7);
    }
    h(pa, 5) - h(pb, 5)
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_ordem_por_habilidade(a: *const usize, b: *const usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(1213)(a, b);
    let n = ordem_por_habilidade(a, b);
    PLACAR.registrar(r == n, || format!("ordem_por_habilidade({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// { 0x85e350 } (cdecl, stadium.cpp): `stadium_compare_venues_by_capacity(a, b)`, o qsort dos locais na busca do
/// estádio neutro (0x85c8de): pelos lugares (+0x3c) do estádio (+0xc), do maior para o menor; sem um dos estádios, 0.
unsafe extern "C" fn ordem_pela_capacidade(a: usize, b: usize) -> i32 {
    let ea = core::ptr::read_unaligned((a + 0xc) as *const usize);
    if ea == 0 {
        return 0;
    }
    let eb = core::ptr::read_unaligned((b + 0xc) as *const usize);
    if eb == 0 {
        return 0;
    }
    core::ptr::read_unaligned((eb + 0x3c) as *const i32).wrapping_sub(core::ptr::read_unaligned((ea + 0x3c) as *const i32))
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_ordem_pela_capacidade(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1214)(a, b);
    let n = ordem_pela_capacidade(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_capacidade({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 257a leva: os programas de treino padrão: os sete do computador e os dez de cada técnico humano no começo do jogo, o tipo do programa e o programa do humano por vaga (training_manager.cpp) ----------

unsafe fn cpu8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Os cinco níveis (+4..+8) de um programa de treino.
unsafe fn cpu8_niveis(this: usize, n: [u8; 5]) {
    for (k, v) in n.iter().enumerate() {
        *((this + 4 + k) as *mut u8) = *v;
    }
}

/// { 0x8b4e50 } (thiscall, ret 8, training_manager.cpp): `TRAINING_SCHEDULE::set_type(vaga, tipo)`: a vaga (com sinal,
/// em dword) em +0, o tipo em +0x15 e os cinco níveis das categorias (+4..+8) pela mistura do tipo (tabela de saltos
/// 0x8b4f08): 1 geral (25 nas quatro primeiras), 2 a 5 uma categoria com 50 e as outras com 10, 6 goleiros (50 na quinta,
/// 0 na terceira), 0 e 7 sem treino (tudo 0); fora de 0..=7 (o −1 da primeira vaga do humano) só a vaga e o tipo. Pela
/// set_default_schedules (0x8b3d40, 167 vezes no começo do jogo) e pelas telas do treino.
unsafe extern "thiscall" fn tipo_do_programa(this: usize, vaga: i8, tipo: i8) {
    *(this as *mut i32) = vaga as i32;
    *((this + 0x15) as *mut i8) = tipo;
    match tipo {
        1 => {
            crate::comparacao::ramo("cpu_8 geral");
            cpu8_niveis(this, [0x19, 0x19, 0x19, 0x19, 0]); // cpu8: o treino geral
        }
        2 => cpu8_niveis(this, [0x32, 0xa, 0xa, 0xa, 0]),
        3 => cpu8_niveis(this, [0xa, 0xa, 0x32, 0xa, 0]),
        4 => cpu8_niveis(this, [0xa, 0xa, 0xa, 0x32, 0]),
        5 => cpu8_niveis(this, [0xa, 0x32, 0xa, 0xa, 0]),
        6 => {
            crate::comparacao::ramo("cpu_8 goleiros");
            cpu8_niveis(this, [0xa, 0xa, 0, 0xa, 0x32]); // cpu8: o treino dos goleiros
        }
        0 | 7 => cpu8_niveis(this, [0; 5]),
        _ => {}
    }
}

/// Comparador: lado a lado nos 0x16 bytes do programa (a vaga, os níveis e o tipo).
unsafe extern "thiscall" fn comparar_tipo_do_programa(this: usize, vaga: i8, tipo: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8)>(1216);
    let regioes = [(this, 0x16)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, vaga, tipo), || tipo_do_programa(this, vaga, tipo));
    PLACAR.registrar(igual, || format!("tipo_do_programa({:x}, {}, {}) {:x?} x {:x?}", this, vaga, tipo, mo, mr));
}

/// O programa `vaga` (0..=9) do técnico humano `pessoa` como a 0x8b3d40 o calcula sem chamar a 0x8b3930 (vagas 0 a
/// 5): this + 0x43d + 0x9b × vaga + 0x60e × humano; fora dos 16 humanos, a caixa (linha 0x68f) e 0.
unsafe fn cpu8_vaga_do_humano(this: usize, pessoa: usize, vaga: usize) -> usize {
    let v = cpu8_i(0xae2364).wrapping_sub(cpu8_i(pessoa)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        aviso_em(0xa8ccd0, 0x68f);
        return 0;
    }
    this + 0x43d + 0x9b * vaga + 0x60e * v as usize
}

/// { 0x8b3d40 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::set_default_schedules()`, pelo construtor
/// (0x8b0ec0) num jogo novo (sem training.dat): os sete programas do computador (0x9b bytes cada a partir de this; o
/// tipo k na vaga k: o computador treina pela vaga 1, geral, e pela 6, goleiros, conforme a ficha de treino do
/// jogador) e, para cada um dos 16 técnicos humanos (pessoa [0xae2364] − h − 1, registro de 0x6e bytes em
/// [0xae23bc]), os dez programas: vagas 0 a 5 calculadas aqui (tipos −1, 0, 1, 2, 3, 4), 6 a 9 pela 0x8b3930 (tipos
/// 5, 6, 7, 7). No fim os nomes padrão (0x8b2230). Chamadas pelo endereço (0x8b4e50 e 0x8b3930 nesta leva).
unsafe extern "thiscall" fn programas_padrao(this: usize) {
    let tipo: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(0x8b4e50usize);
    let humano: unsafe extern "thiscall" fn(usize, usize, i8) -> usize = core::mem::transmute(0x8b3930usize);
    tipo(this, 0, 0);
    crate::comparacao::ramo("cpu_8 programas padrao");
    tipo(this + 0x9b, 1, 1); // cpu8: o programa geral do computador
    for k in 2..7i8 {
        tipo(this + 0x9b * k as usize, k, k);
    }
    for h in 0..0x10i32 {
        let id = cpu8_i(0xae2364).wrapping_sub(h).wrapping_sub(1);
        let pessoa = (*(0xae23bc as *const usize)).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        for (vaga, t) in [(0usize, -1i8), (1, 0), (2, 1), (3, 2), (4, 3), (5, 4)] {
            tipo(cpu8_vaga_do_humano(this, pessoa, vaga), vaga as i8, t);
        }
        // ponytail: a original empurra a vaga 8 e 9 num dword só com o byte baixo gravado; as duas chamadas leem o byte.
        for (vaga, t) in [(6i8, 5i8), (7, 6), (8, 7), (9, 7)] {
            tipo(humano(this, pessoa, vaga), vaga, t);
        }
    }
    let nomes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b2230usize);
    nomes(this);
}

/// Comparador: lado a lado nos 167 programas (0x651d bytes a partir de this; nomes incluídos) e no marcador de erro.
unsafe extern "thiscall" fn comparar_programas_padrao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1215);
    let regioes = [(this, 0x651d), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || programas_padrao(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("programas_padrao({:x}) diferencas em {:x?}", this, dif)
    });
}

/// { 0x8b3930 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_manager_schedule(tecnico, vaga)`: o
/// programa da vaga (0..=9; fora disso, 1) do técnico humano: this + 0x9b × (vaga + 10 × humano + 7), humano =
/// [0xae2364] − id − 1. Fora dos 16 humanos: a caixa (linha 0x68f) e 0. Pela 0x8b3d40 (vagas 6 a 9) e pelas telas.
unsafe extern "thiscall" fn programa_do_humano(this: usize, tecnico: usize, vaga: i8) -> usize {
    let v = cpu8_i(0xae2364).wrapping_sub(cpu8_i(tecnico)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        aviso_em(0xa8ccd0, 0x68f);
        return 0;
    }
    let vaga = if (0..0xa).contains(&vaga) { vaga } else { 1 };
    this.wrapping_add(0x9b * (vaga as i32 + 10 * v + 7) as usize)
}

/// Comparador: a devolução, sem os casos de erro (só a original).
unsafe extern "thiscall" fn comparar_programa_do_humano(this: usize, tecnico: usize, vaga: i8) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8) -> usize>(1217);
    if !(0..0x10).contains(&cpu8_i(0xae2364).wrapping_sub(cpu8_i(tecnico)).wrapping_sub(1)) {
        return o(this, tecnico, vaga);
    }
    let r = o(this, tecnico, vaga);
    let n = programa_do_humano(this, tecnico, vaga);
    PLACAR.registrar(r == n, || format!("programa_do_humano(tecnico {:x}, vaga {}) original={:x} rust={:x}", tecnico, vaga, r, n));
    r
}

// ---------- 257a leva: as fichas de treino iniciais dos jogadores, o banco de jogadores livres do regen e se há reserva de não jogador (training_manager.cpp, player_regen.cpp) ----------

unsafe fn cpu9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpu9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x8b3c50 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::init_player_entries()`, pelo construtor
/// (0x8b0ec0) num jogo novo: aloca (calloc 0x945501) as fichas de treino de 6 bytes, uma por ficha de jogador
/// ([0xae236c] fichas de 0x46 bytes em [0xae23c4], a ficha pelo id +0), em this + 0x651d; sem memória, a caixa (linha
/// 0x6e3) e volta. Em cada ficha apaga o bit 0x20 do +5 e põe a vaga do programa (+0): 6 (goleiros) com a nota de
/// goleiro (+0xf, com sinal) de 15 para cima, senão 1 (geral). A primeira atualização do treino de cada jogador
/// (0x8b1750, que só depois refaz a vaga dos jogadores do computador) e todo o treino do clube humano usam essa vaga.
unsafe extern "thiscall" fn fichas_de_treino_iniciais(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let p = calloc(cpu9_d(0xae236c) as u32, 6);
    *((this + 0x651d) as *mut u32) = p as u32;
    if p == 0 {
        aviso_em(0xa8ccd0, 0x6e3);
        return;
    }
    let mut f = cpu9_d(0xae23c4);
    let mut i = 0i32;
    while i < cpu9_i(0xae236c) {
        let ficha = || cpu9_d(this + 0x651d).wrapping_add((cpu9_i(f) as usize).wrapping_mul(6));
        *((ficha() + 5) as *mut u8) &= 0xdf;
        let gol = *((f + 0xf) as *const i8);
        if gol >= 0xf { // cpu9: goleiro de 15 para cima
            crate::comparacao::ramo("cpu_9 goleiro");
            *(ficha() as *mut u8) = 6; // cpu9: o programa dos goleiros
        } else {
            crate::comparacao::ramo("cpu_9 linha");
            *(ficha() as *mut u8) = 1; // cpu9: o programa geral
        }
        i += 1;
        f += 0x46;
    }
}

/// Comparador: a original aloca (não pode rodar duas vezes): roda só ela e confere cada ficha pela regra (a vaga 6
/// ou 1 pela nota de goleiro e o bit 0x20 do +5 apagado).
unsafe extern "thiscall" fn comparar_fichas_de_treino_iniciais(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1218)(this);
    let base = cpu9_d(this + 0x651d);
    if base == 0 {
        return;
    }
    let mut erradas = 0usize;
    let mut primeira = None;
    for k in 0..cpu9_i(0xae236c).max(0) as usize {
        let f = cpu9_d(0xae23c4) + 0x46 * k;
        let e = base.wrapping_add((cpu9_i(f) as usize).wrapping_mul(6));
        let vaga = if *((f + 0xf) as *const i8) >= 0xf { 6 } else { 1 };
        if *(e as *const u8) != vaga || *((e + 5) as *const u8) & 0x20 != 0 {
            erradas += 1;
            primeira.get_or_insert(k);
        }
    }
    PLACAR.registrar(erradas == 0, || format!("fichas_de_treino_iniciais: {} fichas fora da regra (a primeira {:?})", erradas, primeira));
}

/// { 0x7a8350 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_player_data_to_pool(pessoa, nacao,
/// segunda)`: acrescenta ao banco de jogadores livres (16 bytes em [this+0x3c], conta +0x40, capacidade +0x44) o id da
/// ficha de jogador (+0x61) da pessoa de índice `pessoa` (registro de 0x6e bytes em [0xae23bc]) em +0, a nação em +4 e
/// a segunda nação em +8 (+0xc fica como está). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória,
/// caixa 0xa0a e fim). Pela process_retirements (0x7a59e0), quando um jogador que também tem ficha de não jogador
/// deixa de jogar.
unsafe extern "thiscall" fn guardar_jogador_no_banco(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    if cpk2_d(this + 0x40) == cpk2_d(this + 0x44).wrapping_sub(1) {
        let p = cpk2_realocar(cpk2_d(this + 0x3c), cpk2_d(this + 0x44).wrapping_add(250) << 4);
        cpk2_por(this + 0x3c, p);
        cpk2_por(this + 0x44, cpk2_d(this + 0x44).wrapping_add(250));
        if p == 0 {
            cpk2_sem_memoria(0xa0a);
        }
    }
    crate::comparacao::ramo("cpu_9 banco de jogadores");
    let registro = cpk2_d(0xae23bc).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize);
    let item = cpk2_d(this + 0x3c).wrapping_add(cpk2_d(this + 0x40) << 4);
    cpk2_por(item, cpk2_d(cpk2_d(registro + 0x61)));
    cpk2_por(item + 4, nacao as usize);
    cpk2_por(item + 8, segunda as usize);
    cpk2_por(this + 0x40, cpk2_d(this + 0x40).wrapping_add(1));
}

/// Comparador: o id da ficha antes; a original acrescenta (pode realocar); confere a conta e o item novo.
unsafe extern "thiscall" fn comparar_guardar_jogador_no_banco(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    let n = cpk2_d(this + 0x40);
    let id = cpk2_d(cpk2_d(cpk2_d(0xae23bc).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize) + 0x61)) as u32;
    orig::<unsafe extern "thiscall" fn(usize, i32, u32, u32)>(1219)(this, pessoa, nacao, segunda);
    let item = cpk2_d(this + 0x3c).wrapping_add(n << 4);
    let g = [cpk2_d(item) as u32, cpk2_d(item + 4) as u32, cpk2_d(item + 8) as u32];
    PLACAR.registrar(cpk2_d(this + 0x40) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("guardar_jogador_no_banco({}, {}, {}) conta {} -> {}, item {:?} (id {})", pessoa, nacao, segunda, n, cpk2_d(this + 0x40), g, id)
    });
}

/// { 0x7a6e20 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::has_spare_non_player_record()`: se a reserva de
/// fichas de não jogador (conta em +0x20) tem alguma. O eax leva os 24 bits altos da conta (setne no al), como na
/// original.
unsafe extern "thiscall" fn tem_reserva_de_tecnico(this: usize) -> u32 {
    let c = cpu9_d(this + 0x20) as u32;
    (c & 0xffff_ff00) | (c != 0) as u32
}

/// Comparador: a devolução.
unsafe extern "thiscall" fn comparar_tem_reserva_de_tecnico(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1220)(this);
    let n = tem_reserva_de_tecnico(this);
    PLACAR.registrar(r == n, || format!("tem_reserva_de_tecnico({:x}) original={:x} rust={:x}", this, r, n));
    r
}

// ---------- 257a leva: as suspensões dos órgãos disciplinares: a suspensão do cartão vermelho, a virada da temporada do órgão e a compactação das suas entradas (ruling_body.cpp) ----------

unsafe fn cpu10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpu10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Os jogos de suspensão pelo código da falta (9 a 12) e pela escala do órgão (+6), na tabela de saltos 0x7e9028;
/// None quando não há suspensão (escala abaixo de 3 nos códigos 9 e 10, código fora de 9..=12).
fn cpu10_jogos(codigo: i32, e: i8) -> Option<u8> {
    let x = e as i32;
    Some(match codigo {
        9 if e < 3 => return None,
        9 if e < 8 => 1,
        9 => 2 + (e >= 0xf) as u8,
        10 if e < 3 => return None,
        10 if e < 5 => 1,
        10 if e < 9 => 2,
        10 if e < 0xd => 3,
        10 => 4 + (e >= 0x11) as u8,
        11 => ((x * 8) / 20 + 4) as u8,
        12 => ((x * 28) / 20 + 12) as u8,
        _ => return None,
    })
}

/// Os dias de suspensão (tabela 0x7e9038): os mesmos degraus em semanas.
fn cpu10_dias(codigo: i32, e: i8) -> Option<u16> {
    let x = e as i32;
    Some(match codigo {
        9 if e < 3 => return None,
        9 if e < 8 => 7,
        9 => if e >= 0xf { 21 } else { 14 },
        10 if e < 3 => return None,
        10 if e < 5 => 7,
        10 if e < 9 => 14,
        10 if e < 0xd => 21,
        10 => if e >= 0x11 { 35 } else { 28 },
        11 => (((x * 8) / 20 + 4) * 7) as u16,
        12 => (((x * 28) / 20 + 12) * 7) as u16,
        _ => return None,
    })
}

/// { 0x7e8e10 } (thiscall, ret 0xc, ruling_body.cpp): `RulingBody::make_points_ban(registro, jogador, por_jogos)`, pelos
/// órgãos de cada país (rb_*.cpp, ao processar o cartão vermelho de uma partida): o código da falta (+0x2e com +0x2c =
/// 0, +0x2f com +0x2c = 1; outro tipo: 0) dá, pela escala do órgão (+6), os jogos (por_jogos ≠ 0; os dias ficam
/// 0xffff) ou os dias (os jogos ficam 0xff) da suspensão. No registro: +0 jogos, +1 = 0, +2 dias, +4 a data de hoje
/// ([0xae2c90], 0x417c10), +0x10 = 3, +0x1c o id do jogador (+0), +0x6f = (+0x48 do jogador == 0). Devolve 1; sem
/// suspensão, 0 sem gravar nada.
unsafe extern "thiscall" fn suspensao_do_vermelho(this: usize, r: usize, j: usize, por_jogos: u32) -> u32 {
    let codigo = match *((j + 0x2c) as *const u8) {
        0 => *((j + 0x2e) as *const i8) as i32, // cpu10: a falta do primeiro tipo
        1 => *((j + 0x2f) as *const i8) as i32,
        _ => return 0,
    };
    let escala = *((this + 6) as *const i8);
    let (jogos, dias) = if por_jogos != 0 {
        match cpu10_jogos(codigo, escala) {
            Some(n) => (n, 0xffffu16),
            None => return 0,
        }
    } else {
        match cpu10_dias(codigo, escala) {
            Some(n) => (0xffu8, n),
            None => return 0,
        }
    };
    crate::comparacao::ramo("cpu_10 punicao");
    *(r as *mut u8) = jogos; // cpu10: os jogos da suspensão
    *((r + 1) as *mut u8) = 0;
    *((r + 2) as *mut u16) = dias;
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut hoje = [0u32; 2];
    copiar(r + 4, hoje.as_mut_ptr() as usize, 0xae2c90);
    *((r + 0x10) as *mut u8) = 3;
    *((r + 0x1c) as *mut u32) = cpu10_d(j) as u32;
    *((r + 0x6f) as *mut u8) = (*((j + 0x48) as *const u8) == 0) as u8;
    1 // cpu10: a punição feita
}

/// Comparador: lado a lado nos 0x70 bytes do registro (a rotina só grava nele).
unsafe extern "thiscall" fn comparar_suspensao_do_vermelho(this: usize, r: usize, j: usize, por_jogos: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1221);
    let regioes = [(r, 0x70)];
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, r, j, por_jogos), || suspensao_do_vermelho(this, r, j, por_jogos));
    PLACAR.registrar(igual && a == b, || format!("suspensao_do_vermelho(jogador {:x}, por_jogos {}) original={} rust={} {:x?} x {:x?}", cpu10_d(j), por_jogos, a, b, mo, mr));
    a
}

/// { 0x7e7ed0 } (thiscall, ret 0, ruling_body.cpp): `RulingBody::start_new_season()`: para cada entrada disciplinar
/// ocupada (14 bytes em [this+0xb], conta +0x13; id −1 é vaga livre): com suspensão (+6, índice com sinal na tabela de
/// 20 bytes da disciplina [[0xae28f0]+4]) de 2 anos ou mais (4 se o órgão não tem +0x17 nem +0x1b; o ano da suspensão
/// em +6 contra o de hoje [0xae2c92]), solta a suspensão (0x556d00) e libera a entrada; sem suspensão, libera a
/// entrada; senão zera os cartões (+4, +5, +8 e os 6 bits baixos de +0xd). Liberar: a vaga guardada do jogador no
/// órgão (0x55cb60 pela vaga +4) com o mesmo +5 fica com −1 em +6; sem ela, grava a vaga (0x55cb20, tipo +5, −1); o id
/// da entrada vira −1. No fim compacta as entradas (0x7e9050, nesta leva).
unsafe extern "thiscall" fn nova_temporada_do_orgao(this: usize) {
    let guardada: unsafe extern "thiscall" fn(usize, i32, i8) -> usize = core::mem::transmute(0x55cb60usize);
    let gravar: unsafe extern "thiscall" fn(usize, i32, i8, u8, u32) = core::mem::transmute(0x55cb20usize);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
    crate::comparacao::ramo("cpu_10 nova temporada");
    let mut e = cpu10_d(this + 0xb);
    let mut i = 0i32;
    while i < cpu10_i(this + 0x13) {
        let id = cpu10_i(e);
        if id != -1 {
            let s = *((e + 6) as *const i16);
            let liberar = if s == -1 {
                true
            } else {
                let ban = cpu10_d(cpu10_d(0xae28f0) + 4).wrapping_add((s as i32 as usize).wrapping_mul(20));
                let anos = if cpu10_i(this + 0x17) == -1 && cpu10_i(this + 0x1b) == -1 { 4 } else { 2 };
                let ano = *((ban + 6) as *const u16) as i32;
                if *(0xae2c92 as *const u16) as i32 - ano >= anos {
                    crate::comparacao::ramo("cpu_10 suspensao velha");
                    // ponytail: a original empurra o índice com os 16 bits altos do edx de antes; a 0x556d00 lê só a palavra.
                    soltar(cpu10_d(0xae28f0), id as u32, s as u16 as u32);
                    true
                } else {
                    false
                }
            };
            if liberar {
                let id = cpu10_i(e);
                let g = guardada(cpu10_d(0xae28f0), id, *((this + 4) as *const i8));
                if g != 0 && *((g + 5) as *const u8) == *((this + 5) as *const u8) {
                    *((g + 6) as *mut u32) = 0xffff_ffff;
                } else {
                    gravar(cpu10_d(0xae28f0), cpu10_i(e), *((this + 4) as *const i8), *((this + 5) as *const u8), 0xffff_ffff);
                }
                *(e as *mut u32) = 0xffff_ffff;
            } else {
                let f = *((e + 0xd) as *const u8);
                *((e + 4) as *mut u8) = 0;
                *((e + 5) as *mut u8) = 0;
                *((e + 8) as *mut u16) = 0;
                *((e + 0xd) as *mut u8) = f & 0xc0;
            }
        }
        i += 1;
        e += 0xe;
    }
    let compactar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7e9050usize);
    compactar(this);
}

/// Comparador: a original solta suspensões e realoca (não pode rodar duas vezes): só ela roda.
// ponytail: a cópia leva o contador "cpu_10 nova temporada" para provar no modo Rust; a compactação (comparada) confere o fim.
unsafe extern "thiscall" fn comparar_nova_temporada_do_orgao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1222)(this);
}

/// { 0x7e9050 } (thiscall, ret 0, ruling_body.cpp): `RulingBody::compact_entries()`: tira as entradas livres (id −1)
/// da tabela de 14 bytes [this+0xb], trazendo as ocupadas para a frente; cada uma que muda de lugar acerta o índice
/// guardado do jogador (0x55cb60 pela vaga +4: com o mesmo +5, o novo índice em +6; sem ela, 0x55cb20 com o tipo +5 e
/// o índice). Conta +0x13 = n, capacidade +0xf = n + 100 e realoca (0x945667) com (n + 100) × 14 bytes; sem memória, a
/// caixa (linha 0x594) e segue.
unsafe extern "thiscall" fn compactar_disciplina_do_orgao(this: usize) {
    let guardada: unsafe extern "thiscall" fn(usize, i32, i8) -> usize = core::mem::transmute(0x55cb60usize);
    let gravar: unsafe extern "thiscall" fn(usize, i32, i8, u8, u32) = core::mem::transmute(0x55cb20usize);
    let mut destino = cpu10_d(this + 0xb);
    let mut origem = destino;
    let mut n = 0u32;
    let mut i = 0i32;
    while i < cpu10_i(this + 0x13) {
        if cpu10_i(origem) != -1 {
            if origem != destino {
                core::ptr::copy(origem as *const u8, destino as *mut u8, 0xe);
                let g = guardada(cpu10_d(0xae28f0), cpu10_i(origem), *((this + 4) as *const i8));
                if g != 0 && *((g + 5) as *const u8) == *((this + 5) as *const u8) {
                    *((g + 6) as *mut u32) = n;
                } else {
                    gravar(cpu10_d(0xae28f0), cpu10_i(origem), *((this + 4) as *const i8), *((this + 5) as *const u8), n);
                }
            }
            destino += 0xe;
            n += 1;
        }
        i += 1;
        origem += 0xe;
    }
    *((this + 0x13) as *mut u32) = n;
    *((this + 0xf) as *mut u32) = n + 100;
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let p = realocar(cpu10_d(this + 0xb), (n + 100) * 14);
    *((this + 0xb) as *mut u32) = p as u32;
    if p == 0 {
        aviso_em(0xa7a858, 0x594);
    }
}

/// Comparador: a original realoca (não pode rodar duas vezes): as entradas ocupadas copiadas antes; roda só a
/// original e confere a conta, a capacidade e as entradas na nova ordem.
unsafe extern "thiscall" fn comparar_compactar_disciplina_do_orgao(this: usize) {
    let mut esperado: Vec<u8> = Vec::new();
    let base = cpu10_d(this + 0xb);
    for k in 0..cpu10_i(this + 0x13).max(0) as usize {
        let e = base + 0xe * k;
        if cpu10_i(e) != -1 {
            esperado.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xe));
        }
    }
    orig::<unsafe extern "thiscall" fn(usize)>(1223)(this);
    let n = (esperado.len() / 0xe) as u32;
    let p = cpu10_d(this + 0xb);
    let iguais = p != 0 && core::slice::from_raw_parts(p as *const u8, esperado.len()) == &esperado[..];
    PLACAR.registrar(cpu10_d(this + 0x13) as u32 == n && cpu10_d(this + 0xf) as u32 == n + 100 && iguais, || {
        format!("compactar_disciplina_do_orgao: conta {} (esperava {}), capacidade {}, entradas iguais {}", cpu10_d(this + 0x13), n, cpu10_d(this + 0xf), iguais)
    });
}

// ---------- 257a leva: as casas da formação: gravar a coordenada da casa e escolher as posições, a posição repetida e o tipo de marcação (formation.cpp) ----------

unsafe fn cpu11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpu11_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// { 0x5b3560 } (thiscall, ret 0x10, formation.cpp): `Formation::set_slot_coordinate(casa, fase, faixa, coord)`:
/// grava a coordenada (dword: x 0..0x1a, y 0..0x23) da casa na fase e na faixa da bola (+0x164, 11 × 2 × 3 × 4
/// dwords) e, para cada fase, acha as posições da tabela 0x9cc368 (palavras até 0xffff) mais perto das 12 coordenadas
/// da casa nessa fase (|Δx·9/27| + |Δy·12/36| contra a casa de cada posição pela 0x5b7e60; dobrada nas posições com o
/// bit 2; os empates entram na lista da fase, até 11 em [esp+0x64] contíguas). A posição geral (+0x12d) fica se
/// estiver numa das listas; senão vira a primeira da fase de menor distância (a 0x5b59a0 refaz a máscara, a casa
/// ganha a marca +0x143 e as outras casas com a mesma posição a perdem). A posição extra (+0x14e) sai da lista da
/// outra fase: a mesma posição, ou a de mesmo lado (0x5b73a0), ou a primeira, ou com duas o meio dos bits das duas.
/// Zera +0x187f. Argumentos fora da faixa: a original (caixa e 0). Devolve 1.
unsafe extern "thiscall" fn coordenada_da_casa(this: usize, casa: u32, fase: u32, faixa: u32, coord: u32) -> u32 {
    let (s, f) = (casa as u8 as i8, fase as u8 as i8);
    let (zx, zy) = (faixa as u16 as i16, (faixa >> 16) as u16 as i16);
    let (cx, cy) = (coord as u16 as i16, (coord >> 16) as u16 as i16);
    if !(0..=10).contains(&s) || !(0..=1).contains(&f) || !(0..=2).contains(&zx) || !(0..=3).contains(&zy)
        || !(0..=0x1a).contains(&cx) || !(0..=0x23).contains(&cy)
    {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(1224)(this, casa, fase, faixa, coord);
    }
    let casa_no: unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32 = core::mem::transmute(0x5b7e60usize);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
    let su = s as usize;
    let cel = this + 0x164 + 4 * (zy as usize + 4 * (zx as usize + 3 * (f as usize + 2 * su)));
    crate::comparacao::ramo("cpu_11 coordenada gravada");
    core::ptr::write_unaligned(cel as *mut u32, coord); // cpu11: a coordenada
    // a saída da 0x5b7e60 é o próprio argumento coord na original (uma vaga só, para a função inteira)
    let mut saida: [i16; 2] = [cx, cy];
    // ponytail: as listas das duas fases ficam contíguas como em [esp+0x64]; os índices além de 0xea não acontecem
    let mut cand = [0u16; 0xea];
    let mut conta = [0i8; 2]; // na original sem valor inicial; a primeira posição sempre a zera
    let mut melhor = [0i32; 2];
    let mut ultimo_dy: i32 = 0; // [esp+0x60], lido só pelos índices fora da tabela no fim
    for p in 0..2usize {
        melhor[p] = 0x7fffffff;
        let mut lista = 0x9cc368usize;
        while cpu11_w(lista) != -1 {
            let pos = cpu11_w(lista) as u16;
            let base = this + 0x164 + su * 0x60 + p * 0x30;
            let mut dist: i32 = 0;
            for zx in 0..3u32 {
                for zy in 0..4u32 {
                    // a máscara vai com a altura da faixa nos 16 bits de cima (o ecx da original); a 0x5b7e60 não os lê
                    casa_no(pos as u32 | zy << 16, p as i8, zx | zy << 16, saida.as_mut_ptr());
                    let c = base + 4 * (zx as usize * 4 + zy as usize);
                    let dy = ((cpu11_w(c + 2) as i32 * 12) / 36 - (saida[1] as i32 * 12) / 36).wrapping_abs();
                    let dx = ((cpu11_w(c) as i32 * 9) / 27 - (saida[0] as i32 * 9) / 27).wrapping_abs();
                    ultimo_dy = dy;
                    dist = dx.wrapping_add(dist).wrapping_add(dy);
                }
            }
            if pos & 2 != 0 {
                crate::comparacao::ramo("cpu_11 distância dobrada");
                dist = dist.wrapping_add(dist); // cpu11: o dobro
            }
            if dist <= melhor[p] { // cpu11: o empate entra na lista
                if dist < melhor[p] {
                    conta[p] = 0;
                } else {
                    crate::comparacao::ramo("cpu_11 empate na lista");
                }
                melhor[p] = dist;
                cand[(conta[p] as i32 + p as i32 * 11) as usize] = pos;
                conta[p] = conta[p].wrapping_add(1);
            }
            lista += 2;
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    let geral = this + 0x12d + su * 2;
    let mut sf: i8 = (melhor[0] > melhor[1]) as i8;
    let mut si: i8 = 0;
    for q in 0..2i8 {
        let mut a: i8 = 0;
        while a < conta[q as usize] {
            if cand[(a as i32 + q as i32 * 11) as usize] == cpu11_w(geral) as u16 {
                sf = q;
                si = a;
                break;
            }
            a += 1;
        }
    }
    let novo = cand[(sf as i32 * 11 + si as i32) as usize];
    if cpu11_w(geral) as u16 != novo {
        cpu11_pw(geral, novo);
        mascara(this);
        *((this + 0x143 + su) as *mut u8) = 1;
        for k in 0..11i8 {
            if k != s && cpu11_w(this + 0x12d + k as usize * 2) as u16 == novo {
                *((this + 0x143 + k as usize) as *mut u8) = 0;
            }
        }
    }
    let extra = this + 0x14e + su * 2;
    let o = (1 - sf as i32) * 11; // o começo da lista da outra fase
    let no = conta[(1 - sf) as usize];
    let atual = cpu11_w(geral) as u16;
    for a in 0..no.max(0) as i32 {
        if cand[(o + a) as usize] == atual {
            cpu11_pw(extra, cand[(o + a) as usize]);
            return 1;
        }
    }
    // ponytail: as saídas da 0x5b73a0 ficam sem valor inicial na original se ela falhar (posição sem lado); aqui 0
    let (mut setor, mut lado_c, mut lado_a) = (0u16, 0u16, 0u16);
    for b in 0..no.max(0) as i32 {
        separar(cand[(o + b) as usize] as u32, &mut setor, &mut lado_c);
        separar(cpu11_w(geral) as u16 as u32, &mut setor, &mut lado_a);
        if lado_c == lado_a {
            cpu11_pw(extra, cand[(o + b) as usize]);
            return 1;
        }
    }
    if no != 2 {
        cpu11_pw(extra, cand[o as usize]);
        return 1;
    }
    // duas na outra fase: o meio dos bits de setor (1..0x40) e de lado (0x80..0x800) das duas. As palavras do quadro a
    // partir de [esp+0x38]: a tabela dos lados e 0xffff, a faixa (3) e 0, o ponteiro da melhor distância (sem valor
    // reproduzível: 0), this, a tabela dos setores e 0xffff, o último |Δy| e as listas.
    let (c0, c1) = (cand[o as usize], cand[(o + 1) as usize]);
    let mut q = [0u16; 0x100];
    q[..6].copy_from_slice(&[0x80, 0x100, 0x200, 0x400, 0x800, 0xffff]);
    q[6] = 3;
    q[10] = this as u16;
    q[11] = (this >> 16) as u16;
    q[12..20].copy_from_slice(&[1, 2, 4, 8, 0x10, 0x20, 0x40, 0xffff]);
    q[20] = ultimo_dy as u16;
    q[21] = (ultimo_dy >> 16) as u16;
    q[22..].copy_from_slice(&cand[..0xea]);
    let mut v1a: i8 = conta[0]; // [esp+0x1a]
    let mut v12: i8 = no; // [esp+0x12]
    for k in 0..7usize {
        if q[12 + k] & c0 != 0 {
            v1a = k as i8;
        }
        if q[12 + k] & c1 != 0 {
            v12 = k as i8;
        }
    }
    let mut v13: i8 = si; // [esp+0x13]
    let mut dl: i8 = v1a;
    for k in 0..5usize {
        if q[k] & c0 != 0 {
            v13 = k as i8;
        }
        if q[k] & c1 != 0 {
            dl = k as i8;
        }
    }
    let ib = (v13 as i32 + dl as i32) / 2;
    let ia = (v12 as i32 + v1a as i32) / 2;
    cpu11_pw(extra, q[ib as usize & 0xff] | q[(12 + ia) as usize & 0xff]);
    1
}

/// { 0x5b3460 } (thiscall, ret 8, formation.cpp): `Formation::check_duplicate_position(casa, *saida)`: zera a saída;
/// a primeira outra casa com a mesma posição geral (+0x12d) decide: com marca (+0x143, byte com sinal) menor que a da
/// casa, saída 1 e devolve 1; senão devolve 0. Sem repetida, 1. Casa fora de 0..10: a original (caixa e 0).
unsafe extern "thiscall" fn posicao_repetida_na_formacao(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let s = casa as u8 as i8;
    if !(0..=10).contains(&s) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(1225)(this, casa, saida);
    }
    *saida = 0;
    let su = s as usize;
    for k in 0..11usize {
        if k != su && cpu11_w(this + 0x12d + k * 2) == cpu11_w(this + 0x12d + su * 2) {
            if *((this + 0x143 + k) as *const i8) < *((this + 0x143 + su) as *const i8) {
                *saida = 1;
                return 1;
            }
            return 0;
        }
    }
    1
}

/// { 0x5b57d0 } (thiscall, ret 8, formation.cpp): `Formation::set_marking_type(casa, marca)`: as marcas 1, 2 e 4
/// trocam os bits 0..2 do byte +0x5b4 da casa; 8 e 0x10, os bits 3..4; zera +0x187f e devolve 1. Outra marca: a
/// original (caixa e 0). A casa não é conferida.
unsafe extern "thiscall" fn tipo_de_marcacao(this: usize, casa: u32, marca: u32) -> u32 {
    let m = marca as u8;
    let limpa = match m {
        1 | 2 | 4 => 0xf8u8,
        8 | 0x10 => 0xe7,
        _ => return orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(1226)(this, casa, marca),
    };
    let b = (this as isize + casa as u8 as i8 as isize + 0x5b4) as *mut u8;
    *b = *b & limpa | m;
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// As regiões da formação que as três gravam: as posições, as marcas, as extras e as coordenadas (+0x12d..+0x584),
/// os tipos de marcação (+0x5b4, 11) e a máscara com +0x187f (+0x187b, 8).
fn cpu11_regioes(this: usize) -> [(usize, usize); 3] {
    [(this + 0x12d, 0x584 - 0x12d), (this + 0x5b4, 11), (this + 0x187b, 8)]
}

/// Comparador: grava só a formação (a 0x5b7e60 e a 0x5b73a0 são puras; a 0x5b59a0 refaz a máscara): lado a lado.
unsafe extern "thiscall" fn comparar_coordenada_da_casa(this: usize, casa: u32, fase: u32, faixa: u32, coord: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(1224);
    let reg = cpu11_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, fase, faixa, coord), || coordenada_da_casa(this, casa, fase, faixa, coord));
    PLACAR.registrar(igual && r == n, || format!("coordenada_da_casa({:x}, {}, {}, {:x}, {:x}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, fase as u8 as i8, faixa, coord, r, n, mo, mr));
    r
}

/// Comparador: pura (grava só a saída): as duas sobre saídas próprias.
unsafe extern "thiscall" fn comparar_posicao_repetida_na_formacao(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(1225);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, saida); // ponytail: a caixa, só a original
    }
    let (mut so, mut sr) = (0xdead_u32, 0xbeef_u32);
    let r = o(this, casa, &mut so);
    let n = posicao_repetida_na_formacao(this, casa, &mut sr);
    PLACAR.registrar(r == n && so == sr, || format!("posicao_repetida_na_formacao({:x}, {}) {}/{} saída {}/{}", this, casa as u8 as i8, r, n, so, sr));
    *saida = so;
    r
}

/// Comparador: grava um byte e +0x187f: lado a lado.
unsafe extern "thiscall" fn comparar_tipo_de_marcacao(this: usize, casa: u32, marca: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(1226);
    if !matches!(marca as u8, 1 | 2 | 4 | 8 | 0x10) {
        return o(this, casa, marca); // ponytail: a caixa, só a original
    }
    let reg = [(this as isize + casa as u8 as i8 as isize + 0x5b4) as usize, 1usize];
    let reg = [(reg[0], reg[1]), (this + 0x187f, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, marca), || tipo_de_marcacao(this, casa, marca));
    PLACAR.registrar(igual && r == n, || format!("tipo_de_marcacao({:x}, {}, {}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, marca as u8, r, n, mo, mr));
    r
}

// ---------- 257a leva: a ordem dos clubes da liga para a tabela de jogos e os clubes possíveis de cada lado de um jogo da copa (league.cpp, Cup.cpp) ----------

unsafe fn cpu12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpu12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de 59 bytes da tabela da liga.
const CPU12_LINHA: usize = 0x3b;

/// Copia um registro de 59 bytes (o `rep movsd` de 14 dwords, a palavra e o byte da original).
unsafe fn cpu12_copiar(destino: usize, origem: usize) {
    core::ptr::copy(origem as *const u8, destino as *mut u8, CPU12_LINHA);
}

/// Põe o par (i, j) da tabela nas vagas k e metade + k da grade, marca as vagas e os dois clubes.
unsafe fn cpu12_par(grade: usize, posto: *mut u32, usado: *mut u32, tab: usize, i: usize, j: usize, k: &mut i32, metade: i32) {
    cpu12_copiar(grade + *k as usize * CPU12_LINHA, tab + i * CPU12_LINHA);
    let m = metade + *k;
    cpu12_copiar(grade + m as usize * CPU12_LINHA, tab + j * CPU12_LINHA);
    *posto.add(m as usize) = 1;
    *posto.add(*k as usize) = 1;
    *usado.add(j) = 1;
    *usado.add(i) = 1;
    *k += 1;
}

/// { 0x686a70 } (thiscall, ret 4, league.cpp): `League::draw_team_order(clubes)`: embaralha a tabela da liga
/// ([+0xb1], +0x3e registros de 59 bytes) para a grade de jogos: guarda um sorteio (0x90cfc0, 30000), semeia o rand
/// da CRT com +0x40 + [0xdd7120] e troca n pares sorteados (rand × n / 0x8000), e volta a semente (sorteio + 1). Liga
/// com a marca 0x100 em +0xd9: cada clube vai à vaga que a tabela virtual +0xa8 diz (se livre), os outros às
/// primeiras livres, e a +0xac recebe cada vaga. Senão: na Escócia ([+4] = [0x9cf5dc]) o Celtic e o Rangers
/// ([0x9d0118], [0x9d011c]) vão às vagas 0 e clubes / 2, em ordem sorteada (0x90cfc0, 2); os clubes do mesmo estádio
/// (+0x69) e depois os de estádios ligados (+0x48 do estádio) vão em pares às vagas k e clubes / 2 + k (passando da
/// metade dos clubes, a caixa); os outros, às primeiras vagas livres. A grade (0x944cf1, livre em 0x944c94) volta à
/// tabela. Sem memória: a caixa (linha 0xed3).
unsafe extern "thiscall" fn sortear_ordem_dos_clubes(this: usize, clubes: i32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(0x9469fausize);
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    let n = || cpu12_w(this + 0x3e) as i32;
    let tab = || cpu12_d(this + 0xb1);
    let metade_n = n() / 2;
    let usado = novo((n() * 4) as u32) as *mut u32;
    let posto = novo((n() << 2) as u32) as *mut u32;
    let grade = novo((n() * CPU12_LINHA as i32) as u32);
    let mut k: i32 = 0;
    if grade == 0 || usado.is_null() || posto.is_null() {
        aviso_em(0x9f15f4, 0xed3);
        liberar(posto as usize);
        liberar(usado as usize);
        liberar(grade);
        return;
    }
    let mut i = 0;
    while i < n() {
        *usado.add(i as usize) = 0;
        *posto.add(i as usize) = 0;
        i += 1;
    }
    let semente = sorteio(30000).wrapping_add(1);
    semear((cpu12_w(this + 0x40) as i32).wrapping_add(*(0xdd7120 as *const i32)) as u32); // cpu12: a semente
    crate::comparacao::ramo("cpu_12 ordem sorteada");
    let mut i = 0;
    while i < n() {
        let a = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let b = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let mut tmp = [0u8; CPU12_LINHA];
        let t = tab();
        core::ptr::copy(( t + a * CPU12_LINHA) as *const u8, tmp.as_mut_ptr(), CPU12_LINHA);
        cpu12_copiar(t + a * CPU12_LINHA, t + b * CPU12_LINHA);
        core::ptr::copy(tmp.as_ptr(), (tab() + b * CPU12_LINHA) as *mut u8, CPU12_LINHA); // cpu12: a troca
        i += 1;
    }
    semear(semente as u32);
    let metade = clubes / 2;
    if cpu12_w(this + 0xd9) as u16 & 0x100 != 0 {
        let mut i = 0;
        while i < n() {
            *((grade + i as usize * CPU12_LINHA) as *mut u32) = 0;
            i += 1;
        }
        let vt = cpu12_d(this);
        let vaga_do: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(cpu12_d(vt + 0xa8));
        let mut i = 0usize;
        while (i as i32) < n() {
            let s = vaga_do(this, cpu12_d(tab() + i * CPU12_LINHA)) as i8 as i32;
            if s >= 0 && s < n() && cpu12_d(grade + s as usize * CPU12_LINHA) == 0 {
                cpu12_copiar(grade + s as usize * CPU12_LINHA, tab() + i * CPU12_LINHA);
                *((tab() + i * CPU12_LINHA) as *mut u32) = 0;
            }
            i += 1;
        }
        let mut i = 0usize;
        while (i as i32) < n() {
            let lin = tab() + i * CPU12_LINHA;
            if cpu12_d(lin) != 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| cpu12_d(grade + j * CPU12_LINHA) == 0) {
                    cpu12_copiar(grade + j * CPU12_LINHA, lin);
                }
            }
            i += 1;
        }
        let gravar: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cpu12_d(vt + 0xac));
        let mut i = 0;
        while i < n() {
            gravar(this, cpu12_d(grade + i as usize * CPU12_LINHA), i);
            i += 1;
        }
    } else {
        if cpu12_d(cpu12_d(this + 4)) == cpu12_d(0x9cf5dc) {
            let (mut rangers, mut celtic): (i8, i8) = (-1, -1); // bl e [esp+0x17]
            let total = cpu12_w(this + 0x3e);
            let mut c: i8 = 0;
            while total > 0 && (c as i16) < total {
                let id = cpu12_d(cpu12_d(tab() + c as isize as usize * CPU12_LINHA));
                if id == cpu12_d(0x9d0118) {
                    celtic = c;
                }
                if id == cpu12_d(0x9d011c) {
                    rangers = c;
                }
                c = c.wrapping_add(1);
            }
            if rangers != -1 && celtic != -1 {
                let (primeiro, segundo) = if sorteio(2) != 0 { (celtic, rangers) } else { (rangers, celtic) };
                cpu12_copiar(grade, tab() + primeiro as isize as usize * CPU12_LINHA);
                cpu12_copiar(grade + metade as usize * CPU12_LINHA, tab() + segundo as isize as usize * CPU12_LINHA);
                k = 1;
                *posto.add(metade as usize) = 1;
                *posto = 1;
                *usado.add(celtic as isize as usize) = 1;
                *usado.add(rangers as isize as usize) = 1;
            }
        }
        // os clubes do mesmo estádio (+0x69)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *usado.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *usado.add(j) == 0 {
                        let ei = cpu12_d(cpu12_d(tab() + i * CPU12_LINHA) + 0x69);
                        if ei != 0 {
                            let ej = cpu12_d(cpu12_d(tab() + j * CPU12_LINHA) + 0x69);
                            if ej != 0 && ei == ej {
                                if k >= metade_n {
                                    aviso_em(0x9f15f4, 0xf76);
                                } else {
                                    cpu12_par(grade, posto, usado, tab(), i, j, &mut k, metade);
                                }
                                break;
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // os clubes de estádios ligados (+0x48 do estádio de um é o do outro)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *usado.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *usado.add(j) == 0 {
                        let ei = cpu12_d(cpu12_d(tab() + i * CPU12_LINHA) + 0x69);
                        if ei == 0 {
                            break;
                        }
                        let ej = cpu12_d(cpu12_d(tab() + j * CPU12_LINHA) + 0x69);
                        if ej == 0 {
                            break;
                        }
                        if cpu12_d(ei + 0x48) == ej || cpu12_d(ej + 0x48) == ei {
                            if k >= metade_n {
                                aviso_em(0x9f15f4, 0xfa3);
                            } else {
                                cpu12_par(grade, posto, usado, tab(), i, j, &mut k, metade);
                            }
                            break;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // os outros, às primeiras vagas livres
        let mut i = 0usize;
        while (i as i32) < n() {
            if *usado.add(i) == 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| *posto.add(j) == 0) { // cpu12: a primeira vaga livre
                    crate::comparacao::ramo("cpu_12 clube na vaga livre");
                    cpu12_copiar(grade + j * CPU12_LINHA, tab() + i * CPU12_LINHA);
                    *posto.add(j) = 1;
                    *usado.add(i) = 1;
                }
            }
            i += 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < n() {
        cpu12_copiar(tab() + i * CPU12_LINHA, grade + i * CPU12_LINHA);
        i += 1;
    }
    liberar(posto as usize);
    liberar(usado as usize);
    liberar(grade);
}

/// O núcleo da 0x5219b0: o jogo `jogo` da rodada `rodada` ([[+0xad] + rodada × 4] + jogo × 0x35): cada lado (+0 e
/// +2) com clube vai a `saida[*conta]` (e conta sobe); lado vazio (−1) desce ao jogo de onde ele vem (+0xe/+0xa e
/// +0x10/+0xc). Rodada negativa ou jogo −1 (também nas descidas): a caixa (linha 0x1592, se `caixa`) e a conta.
/// Devolve a conta.
unsafe fn cpu12_coletar(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32, caixa: bool) -> i32 {
    if rodada < 0 || jogo == -1 {
        if caixa {
            aviso_em(0x9b55ac, 0x1592);
        }
        return *conta;
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let a = cpu12_w(reg);
    if a == -1 {
        cpu12_coletar(this, cpu12_w(reg + 0xe) as i32, cpu12_w(reg + 0xa) as i32, saida, conta, caixa);
    } else {
        *saida.offset(*conta as isize) = a;
        *conta += 1;
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let b = cpu12_w(reg + 2);
    if b == -1 {
        cpu12_coletar(this, cpu12_w(reg + 0x10) as i32, cpu12_w(reg + 0xc) as i32, saida, conta, caixa);
        return *conta;
    }
    *saida.offset(*conta as isize) = b;
    *conta += 1;
    *conta
}

/// { 0x5219b0 } (thiscall, ret 0x10, Cup.cpp): `Cup::collect_possible_teams(rodada, jogo, saida, *conta)`: acrescenta
/// a `saida` os clubes que ainda podem jogar esse jogo, descendo pelos jogos de onde vêm os lados vazios. Saída ou
/// conta nula: a original (a caixa).
unsafe extern "thiscall" fn clubes_possiveis_do_jogo(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    if saida.is_null() || conta.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(1229)(this, rodada, jogo, saida, conta);
    }
    cpu12_coletar(this, rodada, jogo, saida, conta, true)
}

/// { 0x521790 } (thiscall, ret 0x10, Cup.cpp): `Cup::get_fixture_side_teams(rodada, jogo, lado, saida)`: o clube de um
/// lado do jogo (lado ≠ 0: +0; 0: +2) ou, vazio, os clubes que podem chegar a ele (0x5219b0 com a conta no global
/// 0xae1880). Devolve a conta (a caixa se nenhum). Rodada fora de 0..+0x3c, copa sem clubes (+0xa7) ou jogo −1: a
/// original (a caixa e 0).
unsafe extern "thiscall" fn clubes_do_lado_do_jogo(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    if rodada < 0 || rodada > cpu12_w(this + 0x3c) as i32 || cpu12_d(this + 0xa7) == 0 || jogo == -1 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(1228)(this, rodada, jogo, lado, saida);
    }
    let coletar: unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32 = core::mem::transmute(0x5219b0usize);
    let conta = 0xae1880 as *mut i32;
    *conta = 0;
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (cpu12_w(reg), reg + 0xe, reg + 0xa) } else { (cpu12_w(reg + 2), reg + 0x10, reg + 0xc) };
    if c != -1 {
        *saida = c;
        *conta = 1;
        return 1;
    }
    coletar(this, cpu12_w(r) as i32, cpu12_w(j) as i32, saida, conta);
    if *conta == 0 {
        aviso_em(0x9b55ac, 0x157f);
    }
    *conta
}

/// Comparador: embaralha com o sorteio e o rand, aloca e chama a tabela virtual: só a original.
unsafe extern "thiscall" fn comparar_sortear_ordem_dos_clubes(this: usize, clubes: i32) {
    // ponytail: sorteia e aloca (não roda duas vezes); a cópia se prova pela carreira (a grade de jogos de toda liga)
    orig::<unsafe extern "thiscall" fn(usize, i32)>(1227)(this, clubes)
}

/// Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
unsafe extern "thiscall" fn comparar_clubes_do_lado_do_jogo(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(1228);
    if saida.is_null() || rodada < 0 || rodada > cpu12_w(this + 0x3c) as i32 || cpu12_d(this + 0xa7) == 0 || jogo == -1 {
        return o(this, rodada, jogo, lado, saida); // ponytail: a caixa, só a original
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (cpu12_w(reg), reg + 0xe, reg + 0xa) } else { (cpu12_w(reg + 2), reg + 0x10, reg + 0xc) };
    let mut esperado = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        esperado[0] = c;
        n = 1;
    } else {
        cpu12_coletar(this, cpu12_w(r) as i32, cpu12_w(j) as i32, esperado.as_mut_ptr(), &mut n, false);
    }
    let res = o(this, rodada, jogo, lado, saida);
    let obtido = core::slice::from_raw_parts(saida, res.clamp(0, 0x100) as usize);
    let global = *(0xae1880 as *const i32);
    PLACAR.registrar(res == n && global == n && obtido == &esperado[..n.clamp(0, 0x100) as usize], || format!("clubes_do_lado_do_jogo({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this, rodada, jogo, lado as u8, res, n, global, obtido, &esperado[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
unsafe extern "thiscall" fn comparar_clubes_possiveis_do_jogo(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(1229);
    if saida.is_null() || conta.is_null() || rodada < 0 || jogo == -1 || !(0..0x100).contains(&*conta) {
        return o(this, rodada, jogo, saida, conta); // ponytail: a caixa ou conta fora do esperado, só a original
    }
    let c0 = *conta;
    let mut esperado = vec![0i16; 0x200];
    let mut n = c0;
    cpu12_coletar(this, rodada, jogo, esperado.as_mut_ptr(), &mut n, false);
    let res = o(this, rodada, jogo, saida, conta);
    let fim = (*conta).clamp(c0, 0x200);
    let obtido = core::slice::from_raw_parts(saida.offset(c0 as isize), (fim - c0) as usize);
    let ok = res == n && *conta == n && obtido == &esperado[c0 as usize..n.clamp(c0, 0x200) as usize];
    PLACAR.registrar(ok, || format!("clubes_possiveis_do_jogo({:x}, {}, {}) {}/{} conta {} {:?} x {:?}", this, rodada, jogo, res, n, *conta, obtido, &esperado[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}

// ---------- 257a leva: a nota-alvo da busca de jogadores do clube e o histórico de jogos do jogador acompanhado: o jogo da vez e a soma dos gols (player_search.cpp) ----------

unsafe fn cpu13_d(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x7bca90 } (cdecl, player_search.cpp): `player_search_get_club_target_rating(clube, *base, *forcada)`: a nota que
/// o clube procura nos jogadores. A base (sem `forcada`): a reputação (+0x80), a do dinheiro (+0x77 × 0,43 − 430, até
/// 10000, se maior que a reputação), a da nação (+0x57: +0x69 × 500, se maior que reputação / 500) e a da liga (+0x1db
/// em [0xae23d0], 0x6b bytes: +0x69 × 600, idem), pesadas e / 16; metade no clube não profissional (+0x64 ≠ 1). A
/// nota sai da base por faixas (até 3000, 5000, 7000, 9000 e acima). Sem `forcada`, pelo elenco (0x53c590) e pela
/// faixa de riqueza das finanças (0x5a0590 de [0xae2a38], 0x5986b0) nos clubes com nação em [0xadadfc]: elenco curto
/// (< 16) metade; < 20 metade ou 2/3 nos pobres; senão encolhe pela falta de jogadores frente à nação (+0x69 × 1,35 +
/// 20) e a riqueza puxa (−2..2: × 0,909 ou − 1000, × 0,952 ou − 500, nada, × 1,0125 ou + 125, × 1,025 ou + 250).
/// No mínimo 500. A base vai em `*base` (se não nulo). As contas de ponto flutuante em f64 (o x87 do jogo em 53 bits).
unsafe extern "C" fn nota_alvo_do_clube(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let contar: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let li = cpu13_d(clube + 0x1db);
    let liga = if li > -1 { (cpu13_d(0xae23d0) as usize).wrapping_add((li as usize).wrapping_mul(0x6b)) } else { 0 };
    let nacao = cpu13_d(clube + 0x57) as usize;
    let mut esi: i32;
    if !forcada.is_null() {
        esi = *forcada as i32;
    } else {
        let rep = cpu13_w(clube + 0x80) as i32;
        let t = ftol(cpu13_d(clube + 0x77) as f64 * f64_em(0x96e2b0) - f64_em(0x96e2a8)) as i16 as i32;
        let ebx = (if rep > t { rep } else if t > 10000 { 10000 } else { t }) as i16 as i32;
        let mut n = rep;
        let mut l = rep;
        if nacao != 0 {
            let c = cpu13_w(nacao + 0x69) as i32;
            if c > rep / 500 {
                n = c * 500;
                l = n;
            }
        }
        if liga != 0 {
            let c = cpu13_w(liga + 0x69) as i32;
            if c > rep / 500 {
                l = c * 600;
            }
        }
        let soma = if n < 8000 {
            crate::comparacao::ramo("cpu_13 base pela reputação");
            n.wrapping_mul(3).wrapping_add(ebx).wrapping_add(l.wrapping_add(rep.wrapping_mul(2)).wrapping_mul(4))
        } else {
            ebx.wrapping_add(rep.wrapping_mul(7)).wrapping_add(n.wrapping_add(l).wrapping_mul(4))
        };
        esi = soma / 16; // cpu13: a base
    }
    if *((clube + 0x64) as *const u8) != 1 {
        esi /= 2;
    }
    let mut edi: i32 = if esi > 9000 {
        esi.wrapping_mul(3).wrapping_add(7500)
    } else if esi > 7000 {
        ftol(esi as f64 * f64_em(0x96d688) - f64_em(0x96d760))
    } else if esi > 5000 {
        ftol(esi as f64 * f64_em(0x968e18) + f64_em(0x9690b8))
    } else if esi > 3000 {
        crate::comparacao::ramo("cpu_13 faixa de 3000 a 5000");
        esi.wrapping_mul(5).wrapping_sub(5000) // cpu13: a faixa do meio
    } else {
        ftol(esi as f64 * f64_em(0x969018) + f64_em(0x9688e8))
    };
    if forcada.is_null() {
        let elenco = contar(clube, 0, 0);
        let com_nacao = |c: usize| c != 0 && cpu13_d((cpu13_d(0xadadfc) as usize).wrapping_add(cpu13_d(c) as usize * 4)) != 0;
        if elenco < 16 && com_nacao(nacao) {
            edi /= 2;
        } else if elenco < 20 && com_nacao(nacao) {
            let fin = financas(cpu13_d(0xae2a38) as usize, clube);
            if fin != 0 {
                match riqueza(fin) as i8 {
                    -2 => edi /= 2,
                    -1 => edi = ftol(edi as f64 * f64_em(0x96a670)),
                    _ => {}
                }
            }
        } else {
            let nacao = cpu13_d(clube + 0x57) as usize;
            if com_nacao(nacao) {
                let t2 = ftol(cpu13_w(nacao + 0x69) as i32 as f64 * f64_em(0x969588) + f64_em(0x968ea8)) as u8;
                let falta = (elenco as u8).wrapping_sub(t2) as i8;
                if falta < 0 {
                    let v = if falta > -10 { falta as i32 } else { -10 };
                    edi = ftol((v as f64 * f64_em(0x967888) + f64_em(0x967890)) * edi as f64);
                }
            }
            let fin = financas(cpu13_d(0xae2a38) as usize, clube);
            if fin != 0 {
                let v = edi as f64;
                match riqueza(fin) as i8 {
                    -2 => {
                        let (x, y) = (v * f64_em(0x96d718), edi.wrapping_sub(1000) as f64);
                        edi = ftol(if x < y { x } else { y });
                    }
                    -1 => {
                        let (x, y) = (v * f64_em(0x96e2a0), edi.wrapping_sub(500) as f64);
                        edi = ftol(if x < y { x } else { y });
                    }
                    1 => {
                        let (x, y) = (v * f64_em(0x96d6b8), edi.wrapping_add(125) as f64);
                        edi = ftol(if x > y { x } else { y });
                    }
                    2 => {
                        crate::comparacao::ramo("cpu_13 clube rico");
                        let (x, y) = (v * f64_em(0x968fa0), edi.wrapping_add(250) as f64); // cpu13: o rico
                        edi = ftol(if x > y { x } else { y });
                    }
                    _ => {}
                }
            }
        }
    }
    if edi < 500 {
        edi = 500;
    }
    if !base.is_null() {
        *base = esi;
    }
    edi
}

/// { 0x7bd830 } (thiscall, ret 4, player_search.cpp): `player_search_slot_get_match_entry(i)`: o jogo `i` (0x29
/// bytes, a partir de +6) do anel de 10 que começa em +0x1a0 (byte com sinal), ou 0 se `i` ≥ a conta (+0x1a1).
unsafe extern "thiscall" fn jogo_do_historico(this: usize, i: u32) -> usize {
    let i = i as u8 as i8;
    if i >= *((this + 0x1a1) as *const i8) {
        return 0;
    }
    let mut k = *((this + 0x1a0) as *const i8) as i32 + i as i32;
    if k >= 10 {
        k -= 10;
    }
    (this as isize + k as isize * 0x29 + 6) as usize
}

/// { 0x7bd870 } (thiscall, ret 0, player_search.cpp): `player_search_slot_sum_goals()`: a soma (16 bits) do byte com
/// sinal +0x16 dos jogos do anel (+0x1a1 deles, a partir de +0x1a0, dando a volta em 10).
unsafe extern "thiscall" fn gols_do_historico(this: usize) -> u16 {
    let n = *((this + 0x1a1) as *const i8);
    let mut k = *((this + 0x1a0) as *const u8);
    let mut s: u16 = 0;
    for _ in 0..n.max(0) {
        let g = *((this as isize + (k as i8) as isize * 0x29 + 0x16) as *const i8);
        s = s.wrapping_add(g as i16 as u16);
        k = k.wrapping_add(1);
        if k == 10 {
            k = 0;
        }
    }
    s
}

/// Comparador: pura (lê o clube, o elenco e as finanças; grava só `*base`): as duas sobre bases próprias.
unsafe extern "C" fn comparar_nota_alvo_do_clube(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32>(1230);
    let (mut bo, mut br) = (0x7fff_0001_i32, 0x7fff_0002_i32);
    let r = o(clube, &mut bo, forcada);
    let n = nota_alvo_do_clube(clube, &mut br, forcada);
    PLACAR.registrar(r == n && bo == br, || format!("nota_alvo_do_clube({:x}, forcada {}) {}/{} base {}/{}", clube, if forcada.is_null() { -1 } else { *forcada as i32 }, r, n, bo, br));
    if !base.is_null() {
        *base = bo;
    }
    r
}

/// Comparador: pura: compara a devolução.
unsafe extern "thiscall" fn comparar_jogo_do_historico(this: usize, i: u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1231)(this, i);
    let n = jogo_do_historico(this, i);
    PLACAR.registrar(r == n, || format!("jogo_do_historico({:x}, {}) {:x}/{:x}", this, i as u8 as i8, r, n));
    r
}

/// Comparador: pura: compara a devolução.
unsafe extern "thiscall" fn comparar_gols_do_historico(this: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u16>(1232)(this);
    let n = gols_do_historico(this);
    PLACAR.registrar(r == n, || format!("gols_do_historico({:x}) {}/{}", this, r, n));
    r
}
