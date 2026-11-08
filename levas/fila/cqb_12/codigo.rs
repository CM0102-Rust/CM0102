// ---------- %%LEVA%%a leva: as nações-chave das competições: a do clube, a do estádio e a entrada da tabela pelo índice ativo (comp.cpp) ----------

/// A entrada `k` (72 bytes) da tabela das 35 nações-chave em 0xb63d60: +0 a nação, +0x14 o índice ativo.
fn cqb12_entrada(k: i8) -> usize {
    0xb63d60 + k as i32 as usize * 72
}

/// O índice `k` da nação `nacao` na tabela das nações-chave (entrada com nação ativa, +0x11c da nação), ou None.
unsafe fn cqb12_achar(nacao: usize) -> Option<i8> {
    let mut k: i8 = 0;
    while k < 0x23 {
        let e = *(cqb12_entrada(k) as *const usize);
        if e != 0 && *((e + 0x11c) as *const u8) != 0 && e == nacao {
            return Some(k);
        }
        k += 1;
    }
    None
}

/// { 0x48dee0 } (cdecl, comp.cpp): `comp_active_key_nation_of_club(clube)`: o índice ativo (+0x14 da entrada) da
/// nação do clube (+0x53) entre as nações-chave ativas; sem nação ou fora da tabela, 0 para uma seleção (0x53b900) e 3
/// para um clube. Devolve só `al` (os chamadores gravam o byte).
unsafe extern "C" fn comp_active_key_nation_of_club(clube: usize) -> u8 {
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao != 0 {
        if let Some(k) = cqb12_achar(nacao) {
            crate::comparacao::ramo("cqb_12 nacao chave do clube");
            return *((cqb12_entrada(k) + 0x14) as *const u8); // cqb12: o índice ativo da nação do clube
        }
    }
    // 0x53b900: push clube; add esp, 4 -> cdecl
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqb_12 clube sem nacao chave");
    3 // cqb12: clube fora das nações-chave
}

/// Comparador: rotina pura; compara o byte devolvido.
unsafe extern "C" fn comparar_comp_active_key_nation_of_club(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    let n = comp_active_key_nation_of_club(clube);
    PLACAR.registrar(r == n, || format!("comp_active_key_nation_of_club({:#x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x48df40 } (cdecl, comp.cpp): `comp_active_key_nation_of_stadium(estadio)`: o índice ativo da nação da cidade
/// do estádio (+0x38 a cidade, +0x1f a nação) entre as nações-chave ativas; sem estádio, cidade, nação ou fora da
/// tabela, 3. Devolve só `al`.
unsafe extern "C" fn comp_active_key_nation_of_stadium(estadio: usize) -> u8 {
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    if estadio == 0 || u(estadio + 0x38) == 0 || u(u(estadio + 0x38) + 0x1f) == 0 {
        return 3;
    }
    match cqb12_achar(u(u(estadio + 0x38) + 0x1f)) {
        Some(k) => {
            crate::comparacao::ramo("cqb_12 nacao chave do estadio");
            *((cqb12_entrada(k) + 0x14) as *const u8) // cqb12: o índice ativo da nação do estádio
        }
        None => 3,
    }
}

/// Comparador: rotina pura; compara o byte devolvido.
unsafe extern "C" fn comparar_comp_active_key_nation_of_stadium(estadio: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(%%I1%%)(estadio);
    let n = comp_active_key_nation_of_stadium(estadio);
    PLACAR.registrar(r == n, || format!("comp_active_key_nation_of_stadium({:#x}) original={} rust={}", estadio, r, n));
    r
}

/// A entrada da tabela das nações-chave cujo índice ativo (+0x14) é `ativo`, se houver.
unsafe fn cqb12_pelo_ativo(ativo: i8) -> Option<i8> {
    (0..0x23i8).find(|&k| *((cqb12_entrada(k) + 0x14) as *const i8) == ativo)
}

/// { 0x48dcd0 } (cdecl, comp.cpp): `comp_key_nation_entry_from_active_index(ativo)`: a primeira entrada da tabela das
/// nações-chave com aquele índice ativo; nenhuma, a caixa da linha 0x543 de comp.cpp (o jogo segue) e 3.
unsafe extern "C" fn comp_key_nation_entry_from_active_index(ativo: i8) -> i32 {
    match cqb12_pelo_ativo(ativo) {
        Some(k) => k as i32,
        None => {
            aviso_em(0x99b80c, 0x543); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp.cpp"
            3
        }
    }
}

/// Comparador: rotina pura; compara a entrada devolvida. Sem entrada (a caixa): só a original.
unsafe extern "C" fn comparar_comp_key_nation_entry_from_active_index(ativo: i8) -> i32 {
    let o = orig::<unsafe extern "C" fn(i8) -> i32>(%%I2%%);
    if cqb12_pelo_ativo(ativo).is_none() {
        return o(ativo); // ponytail: a caixa não roda duas vezes
    }
    let r = o(ativo);
    let n = comp_key_nation_entry_from_active_index(ativo);
    PLACAR.registrar(r == n, || format!("comp_key_nation_entry_from_active_index({}) original={} rust={}", ativo, r, n));
    r
}
