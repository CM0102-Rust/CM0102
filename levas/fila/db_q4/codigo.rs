// ---------- %%LEVA%%a leva: a ordem do histórico de competições, o vetor de atributos do jogador e se a nação é britânica ou a Irlanda (Database.cpp) ----------

/// { 0x540780 } (cdecl, Database.cpp): comparação do qsort das linhas do histórico de competições.
/// Linha sem competição (+4) vai para o fim (a sem e b com: 1; a com e b sem: −1; as duas sem: 0);
/// depois pelo id da competição (a − b); na mesma, pela temporada (+8, palavra) da mais nova para a
/// mais velha (b − a), a não ser que uma delas seja −1 (0).
unsafe extern "C" fn database_compare_comp_history_rows(a: usize, b: usize) -> i32 {
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let ca = ler(a, 4);
    if ca == 0 {
        return (ler(b, 4) != 0) as i32;
    }
    let cb = ler(b, 4);
    if cb == 0 {
        return -1;
    }
    let d = (*(ca as *const i32)).wrapping_sub(*(cb as *const i32));
    if d != 0 {
        return d; // pelo id da competição
    }
    let ta = core::ptr::read_unaligned((a + 8) as *const i16);
    let tb = core::ptr::read_unaligned((b + 8) as *const i16);
    if ta == -1 || tb == -1 {
        return 0;
    }
    tb as i32 - ta as i32 // a temporada mais nova primeiro
}

/// As 54 primeiras células do vetor de atributos (0x53a610), na ordem em que a original grava:
/// (deslocamento no registro de jogador, peso). Peso 0 = o zero do começo.
const ATTRIBUTE_VECTOR_CELLS: [(u8, u8); 54] = [
    (0x0f, 0), (0x10, 0), (0x11, 0), (0x12, 0), (0x13, 0), (0x14, 0), (0x15, 0), (0x16, 0), (0x1a, 0),
    (0x17, 0), (0x18, 0), (0x19, 0),
    (0x30, 0), (0x3b, 0), (0x1e, 0), (0x23, 2), (0x24, 0), (0x26, 2), (0x27, 2), (0x2a, 1), (0x2b, 2),
    (0x31, 2), (0x32, 2), (0x33, 2), (0x35, 0), (0x37, 2), (0x38, 2), (0x39, 0), (0x3a, 0), (0x3e, 2),
    (0x41, 0), (0x43, 2),
    (0x28, 2), (0x22, 2), (0x29, 0), (0x40, 2), (0x3d, 0), (0x3c, 0), (0x36, 0), (0x2e, 0), (0x2f, 0),
    (0x25, 0), (0x1f, 0), (0x20, 0), (0x21, 0), (0x1b, 2), (0x1c, 0), (0x1d, 0), (0x2c, 0), (0x2d, 0),
    (0x42, 0), (0x3f, 2), (0x44, 2), (0x34, 0),
];

/// { 0x53a610 } (cdecl, Database.cpp): estende os bytes do registro de `jogador` num vetor de 63
/// células de 3 bytes (palavra com sinal + peso): zera os 63 pesos; células 0..53 pela tabela
/// `ATTRIBUTE_VECTOR_CELLS` (posições 0..8, lados 9..11, depois os atributos, peso 2 nos que contam e 1 no
/// +0x2a); a célula 54 é o byte +0x58 da `pessoa`. Os fins de grupo vão às saídas não nulas:
/// posições 9, lados 0xc, ponderados 0x20. Devolve o vetor (eax).
unsafe extern "C" fn database_player_to_attribute_vector(jogador: usize, pessoa: usize, vetor: usize, fim_dos_lados: *mut u8,
                                                   fim_das_posicoes: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    for i in 0..0x3f {
        *((vetor + 3 * i + 2) as *mut u8) = 0;
    }
    for (i, &(d, peso)) in ATTRIBUTE_VECTOR_CELLS.iter().enumerate() {
        core::ptr::write_unaligned((vetor + 3 * i) as *mut i16, *((jogador + d as usize) as *const i8) as i16);
        *((vetor + 3 * i + 2) as *mut u8) = peso;
        match i {
            8 if !fim_das_posicoes.is_null() => *fim_das_posicoes = 9,
            11 if !fim_dos_lados.is_null() => *fim_dos_lados = 0xc,
            31 if !fim_dos_ponderados.is_null() => *fim_dos_ponderados = 0x20,
            _ => {}
        }
    }
    core::ptr::write_unaligned((vetor + 3 * 54) as *mut i16, *((pessoa + 0x58) as *const i8) as i16);
    vetor
}

/// { 0x5446d0 } (cdecl, Database.cpp): 1 se a `nacao` (não nula) é Inglaterra, Escócia, Irlanda, País
/// de Gales ou Irlanda do Norte (a regra das nações da casa).
unsafe extern "C" fn nation_is_uk_or_ireland(nacao: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    let id = g(nacao);
    (id == g(0x9cf2e4) || id == g(0x9cf45c) || id == g(0x9cf2dc) || id == g(0x9cf510) || id == g(0x9cf3e4)) as u32
}

unsafe extern "C" fn comparar_database_compare_comp_history_rows(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = database_compare_comp_history_rows(a, b);
    PLACAR.registrar(r == n, || format!("database_compare_comp_history_rows({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

unsafe extern "C" fn comparar_database_player_to_attribute_vector(jogador: usize, pessoa: usize, vetor: usize, fim_dos_lados: *mut u8,
                                                            fim_das_posicoes: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize>(%%I1%%);
    let mut regioes = vec![(vetor, 0x3f * 3)];
    for p in [fim_dos_lados, fim_das_posicoes, fim_dos_ponderados] {
        if !p.is_null() {
            regioes.push((p as usize, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes,
        || o(jogador, pessoa, vetor, fim_dos_lados, fim_das_posicoes, fim_dos_ponderados),
        || database_player_to_attribute_vector(jogador, pessoa, vetor, fim_dos_lados, fim_das_posicoes, fim_dos_ponderados));
    PLACAR.registrar(igual && r == n, || format!("database_player_to_attribute_vector({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", jogador, pessoa, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_nation_is_uk_or_ireland(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(nacao);
    let n = nation_is_uk_or_ireland(nacao);
    PLACAR.registrar(r == n, || format!("nation_is_uk_or_ireland({:x}) original={} rust={}", nacao, r, n));
    r
}
