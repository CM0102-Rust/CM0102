// ---------- %%LEVA%%a leva: os auxiliares de competição: o tipo de confronto na liga, a força do clube e o colocado mais recente (comp_util.cpp) ----------

unsafe fn cpa3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpa3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// A nota de um clube na estatística da liga (0x4b02e0): pesos 1 nos 4 atributos (+0x72..+0x75; +0x75 e +0x74 com
/// o peso [0x968968]) ou, com menos de 25 jogos (+4), proporcionais aos jogos; a média ponderada, na ordem do x87.
unsafe fn league_stats_rating(e: usize, b73: f64) -> f64 {
    let (mut p, mut k73, mut k72) = (read_f64_at(0x968968), 1.0f64, 1.0f64);
    let jogos = cpa3_b(e + 4);
    if jogos < 0x19 {
        let v = jogos as f64;
        p = v * read_f64_at(0x968a98);
        k73 = v * read_f64_at(0x968a90);
        k72 = read_f64_at(0x968938) - v * read_f64_at(0x968a88);
    }
    let s = cpa3_b(e + 0x75) as f64 * p + cpa3_b(e + 0x72) as f64 * k72 + b73 * k73 + cpa3_b(e + 0x74) as f64 * p;
    s / (p + p + k72 + k73)
}

/// { 0x4b02e0 } (cdecl, comp_util.cpp): `comp_util_get_league_clash_type(a, b)`: os dois clubes na mesma divisão
/// (+0x57, não reserva nem menor: 0x7e6f20), a liga dela ([0xadadfc][id]) com estatística (+8, 0x4a4850 dos dois),
/// sem fases (+0x30 = −1) e com +0x43 = 2, e a diferença de +0x73 até um quarto dos clubes (+0x3e). Com as notas
/// (league_stats_rating): 1 = briga pelo título (virtual +0x98 = 0, as duas abaixo de um terço dos clubes e cada uma não abaixo
/// do +0x74 do outro); 3 = briga na zona (+0x98 ≠ 0, as duas até 0,2 × clubes + +0xbe, ou com +0xbf até 0,2 × clubes
/// + +0xbe + +0xbf, e o mesmo cruzamento do +0x74); senão 0. Devolve só `al`. Sem SEH, sem sorteio; a virtual +0x98
/// é chamada até três vezes, como na original.
unsafe extern "C" fn comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let comp = cpa3_d(a + 0x57);
    if comp == 0 || comp != cpa3_d(b + 0x57) {
        return 0;
    }
    let menor: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7e6f20usize);
    if menor(comp) != 0 {
        return 0;
    }
    let liga = cpa3_d((*(0xadadfc as *const usize)).wrapping_add(4 * *(cpa3_d(a + 0x57) as *const u32) as usize));
    if liga == 0 || cpa3_d(liga + 8) == 0 {
        return 0;
    }
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let ea = achar(cpa3_d(liga + 8), a);
    let eb = achar(cpa3_d(liga + 8), b);
    if ea == 0 || eb == 0 || cpa3_d(liga + 0x30) as i32 != -1 || cpa3_b(liga + 0x43) != 2 {
        return 0;
    }
    let (a73, b73) = (cpa3_b(ea + 0x73) as i32, cpa3_b(eb + 0x73) as i32);
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    if (a73 - b73).abs() > match_pl_ftol(clubes as f64 * read_f64_at(0x9689e0)) as i16 as i32 {
        return 0;
    }
    let (na, nb) = (league_stats_rating(ea, a73 as f64), league_stats_rating(eb, b73 as f64));
    let (a74, b74) = (cpa3_b(ea + 0x74) as f64, cpa3_b(eb + 0x74) as f64);
    let virtual98 = || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpa3_d(cpa3_d(liga) + 0x98));
        f(liga)
    };
    // as comparações do x87: NaN conta como "menor" (C0) e "menor ou igual" (C0|C3)
    let cruzadas = !(a74 > nb) && !(b74 > na);
    if virtual98() == 0 {
        let t = clubes as f64 * read_f64_at(0x968a80);
        if !(na >= t) && !(nb >= t) && cruzadas {
            return 1; // briga pelo título
        }
    }
    let be = cpa3_b(liga + 0xbe) as i8 as f64;
    if virtual98() != 0 {
        let t = clubes as f64 * read_f64_at(0x968918) + be;
        if !(na > t) && !(nb > t) && cruzadas {
            return 3;
        }
    }
    if virtual98() == 0 {
        return 0;
    }
    let bf = cpa3_b(liga + 0xbf) as i8;
    if bf == 0 {
        return 0;
    }
    let t = clubes as f64 * read_f64_at(0x968918) + be + bf as f64;
    if !(na > t) && !(nb > t) && cruzadas {
        3
    } else {
        0
    }
}

// ponytail: ~4 milhões de chamadas na carreira: comparação só do valor, sem regiões (a rotina só lê; a 0x4a4850 pode
// reordenar a estatística, e a original já deixa ordenada para a cópia).
unsafe extern "C" fn comparar_comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I0%%)(a, b);
    let n = comp_util_get_league_clash_type(a, b);
    PLACAR.record(r == n, || format!("comp_util_get_league_clash_type({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x4c0720 } (cdecl, comp_util.cpp): `comp_util_get_club_strength_score(clube)`: o dinheiro (sem tabela de
/// finanças [0xae2a38]: +0x65 / 400; com ela, o saldo de 64 bits / 300 pela 0x5a0590), mais 16 × a reputação (+0x80),
/// mais a nota fora da partida (0x6e6340, 0x840, limite −1) dos 15 escalados (0x6e2000, códigos 1, 0x804, 0x84,
/// 3 × 0x204, 0x810, 0x90, 3 × 0x210, 4 × 0x40) entre os jogadores (+0x61) do elenco (0x541f90). Clube nulo:
/// −10.000.000. Sem SEH, sem sorteio próprio. A original empurra o número de jogadores como dword com só o byte de
/// baixo gravado (o resto é sobra de pilha); a 0x6e2000 só lê o byte, e a cópia passa o byte estendido.
unsafe extern "C" fn comp_util_get_club_strength_score(clube: usize) -> i32 {
    if clube == 0 {
        return -10_000_000;
    }
    let financas = *(0xae2a38 as *const usize);
    let mut forca: i32 = 0;
    if financas == 0 {
        forca = core::ptr::read_unaligned((clube + 0x65) as *const i32) / 400;
    } else {
        let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
        let f = get_club_finance(financas, clube);
        if f != 0 {
            forca = (core::ptr::read_unaligned(f as *const i64) / 300) as i32;
        }
    }
    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 4); // a reputação
    let mut elenco = [0usize; 50];
    let mut jogadores = [0usize; 50];
    let mut escalados = [0usize; 15];
    let codigos: [u16; 15] = [1, 0x804, 0x84, 0x204, 0x204, 0x204, 0x810, 0x90, 0x210, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    reunir(clube, elenco.as_mut_ptr(), 0);
    let mut k: i8 = 0;
    for &p in elenco.iter() {
        if p != 0 && cpa3_d(p + 0x61) != 0 {
            jogadores[k as usize] = p;
            k = k.wrapping_add(1);
        }
    }
    let pick: AssignPositionsFn = core::mem::transmute(0x6e2000usize);
    pick(jogadores.as_ptr() as usize, k as u8 as u32, codigos.as_ptr() as usize, 0xf, 0, escalados.as_mut_ptr() as usize, 0x840, clube, 0, 0, 0, 0, 0, 0, 0);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    for (j, &p) in escalados.iter().enumerate() {
        if p != 0 {
            forca = forca.wrapping_add(nota(p, codigos[j] as u32, 0x840, clube, -1, 0));
        }
    }
    forca
}

// ponytail: 1.117 chamadas; só o valor. A escalação e as notas têm comparadores próprios, e o físico que a nota
// atualiza uma vez (0x62ed70, com sorteio) a original já deixa pronto para a cópia.
unsafe extern "C" fn comparar_comp_util_get_club_strength_score(clube: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> i32>(%%I1%%)(clube);
    let n = comp_util_get_club_strength_score(clube);
    PLACAR.record(r == n, || format!("comp_util_get_club_strength_score({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x4afd80 } (cdecl, comp_util.cpp): `comp_util_get_latest_placed_club(comp, lugar)`: ordena o histórico de
/// competições (qsort 0x945333 com 0x540780), busca (bsearch 0x946464) a competição com ano −1 (qualquer), volta até a
/// primeira linha dela e devolve o clube do `lugar` (0 campeão +0xa, 1 vice +0xe, 2 terceiro +0x12); não achou: 0.
/// Outro lugar: a original (a caixa do comp_util.cpp). A chave na pilha da original só tem +4 e +8 gravados, e a
/// 0x540780 só lê esses dois; a cópia zera o resto.
unsafe extern "C" fn comp_util_get_latest_placed_club(comp: usize, lugar: u32) -> usize {
    let lugar = lugar as u8;
    if lugar > 2 {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I2%%)(comp, lugar as u32); // caixa: só a original
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let buscar: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let comparar: unsafe extern "C" fn(*const u8, usize) -> i32 = core::mem::transmute(0x540780usize);
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
    cpa3_d(p + [0xa, 0xe, 0x12][lugar as usize]) // campeão, vice, terceiro
}

unsafe extern "C" fn comparar_comp_util_get_latest_placed_club(comp: usize, lugar: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I2%%);
    if lugar as u8 > 2 {
        return o(comp, lugar);
    }
    let n = (*(0xae237c as *const i32)).max(0) as usize;
    let reg = [(*(0xae23d4 as *const usize), 0x1a * n)];
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(comp, lugar), || comp_util_get_latest_placed_club(comp, lugar));
    PLACAR.record(igual && r == x, || format!("comp_util_get_latest_placed_club({:x}, {}) {:x}/{:x} {:x?} x {:x?}", comp, lugar as u8, r, x, mo, mr));
    r
}
