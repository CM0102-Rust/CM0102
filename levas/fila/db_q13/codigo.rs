// ---------- %%LEVA%%a leva: a forma do adjetivo por idioma, o atributo de treinador da pessoa e a nota com a habilidade reduzida (Database.cpp) ----------

/// As formas do adjetivo de 0x545a60 (tabelas de salto 0x545b2c/0x545b34, 0x545b58/0x545b60, 0x545b84,
/// 0x545c10, 0x545c9c e 0x545d28), índice 1..35. Os idiomas 3, 6, 7, 8 e 10 dão a mesma linha.
const FORMAS_DO_ADJETIVO_COMUM: [u8; 35] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const FORMAS_DO_ADJETIVO_4: [u8; 35] = [
    2, 3, 0, 2, 2, 3, 0, 0, 2, 0, 0, 0, 1, 1, 0, 1, 2, 3, 0, 3, 0, 0, 0, 2, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0,
];
const FORMAS_DO_ADJETIVO_5: [u8; 35] = [
    2, 3, 2, 2, 1, 3, 2, 1, 2, 2, 3, 1, 3, 3, 1, 3, 2, 3, 2, 4, 1, 1, 1, 2, 2, 1, 3, 2, 1, 1, 1, 1, 3, 2, 2,
];
const FORMAS_DO_ADJETIVO_9: [u8; 35] = [
    1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2, 0, 2, 3, 1, 2, 2, 3, 1, 3, 0, 0, 0, 1, 1, 0, 2, 1, 0, 0, 0, 0, 1, 1, 0,
];

/// { 0x545a60 } (cdecl, Database.cpp): a forma (0..4) do adjetivo para o `indice` (byte com sinal,
/// 1..35) no `idioma` (0x66f4d0; os chamadores só chamam nos idiomas que flexionam). Idioma sem tabela
/// ou índice fora: 0xff. Sem chamadas e sem caixa.
unsafe extern "C" fn forma_do_adjetivo(indice: u32, idioma: u32) -> u8 {
    let linha = match idioma {
        3 | 6 | 7 | 8 | 10 => &FORMAS_DO_ADJETIVO_COMUM,
        4 => &FORMAS_DO_ADJETIVO_4,
        5 => &FORMAS_DO_ADJETIVO_5,
        9 => &FORMAS_DO_ADJETIVO_9,
        _ => return 0xff,
    };
    let i = (indice as u8 as i8 as i32).wrapping_sub(1) as u32; // o ja da original compara sem sinal
    if i > 0x22 {
        return 0xff;
    }
    linha[i as usize]
}

/// { 0x543f30 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`: 0x543fd0 com o
/// não jogador dela (+0x69) e o `atributo` inteiro como veio. Pessoa nula ou atributo (byte com sinal)
/// fora de 1..12: a original (caixa "Error" e −1).
unsafe extern "C" fn atributo_do_nao_jogador(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) {
        return orig::<unsafe extern "C" fn(usize, u32) -> u8>(%%I1%%)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543fd0usize);
    let nao_jogador = core::ptr::read_unaligned((pessoa + 0x69) as *const usize);
    calcular(pessoa, nao_jogador, atributo)
}

/// { 0x5442f0 } (cdecl, Database.cpp): a nota de 1 a 20 (`nota_do_atributo`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador` × 0,1 ([0x967880]); a de 0x544380 com a habilidade
/// reduzida (os goleiros de 0x543540). Termina no `_ftol`.
unsafe extern "C" fn nota_relativa_com_habilidade_reduzida(jogador: usize, valor: u32) -> u32 {
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as f64;
    let v = (valor as u8 as i8) as f64;
    nota_do_atributo(habilidade * f64_em(0x967880) + (v + v)) as u32
}

unsafe extern "C" fn comparar_forma_do_adjetivo(indice: u32, idioma: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(u32, u32) -> u8>(%%I0%%);
    let r = o(indice, idioma);
    let n = forma_do_adjetivo(indice, idioma);
    PLACAR.registrar(r == n, || format!("forma_do_adjetivo({}, idioma {}) original={} rust={}", indice as u8 as i8, idioma, r, n));
    r
}

unsafe extern "C" fn comparar_atributo_do_nao_jogador(pessoa: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u8>(%%I1%%);
    let r = o(pessoa, atributo);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) {
        return r; // caixa: só a original
    }
    let n = atributo_do_nao_jogador(pessoa, atributo);
    PLACAR.registrar(r == n, || format!("atributo_do_nao_jogador({:x}, {}) original={} rust={}", pessoa, a, r, n));
    r
}

unsafe extern "C" fn comparar_nota_relativa_com_habilidade_reduzida(jogador: usize, valor: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(%%I2%%);
    let r = o(jogador, valor);
    let n = nota_relativa_com_habilidade_reduzida(jogador, valor);
    PLACAR.registrar(r == n, || format!("nota_relativa_com_habilidade_reduzida({:x}, {}) original={} rust={}", jogador, valor as u8 as i8, r, n));
    r
}
