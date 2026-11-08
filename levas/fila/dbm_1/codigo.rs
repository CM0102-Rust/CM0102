// ---------- %%LEVA%%a leva: os nomes em inglês da sigla da nação, da competição e da competição de pessoal (Database.cpp) ----------

/// Comparador dos nomes em inglês (0x5454c0, 0x5455a0, 0x5456a0): com o idioma dos nomes ([0x9b5984]) ≠ 0 e ≠ 1 e
/// a tabela do eng.lng ([0xae2408]) ainda nula, o primeiro uso aloca e carrega sob o quadro SEH: só a original.
/// Senão as duas, comparando o ponteiro devolvido (sem efeito na memória).
unsafe fn compare_nome_em_ingles(nome: &str, x: usize, o: unsafe extern "C" fn(usize) -> usize, r: unsafe extern "C" fn(usize) -> usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(x);
    }
    let a = o(x);
    let b = r(x);
    SCOREBOARD.record(a == b, || format!("{}({:x}) idioma {} original={:x} rust={:x}", nome, x, idioma, a, b));
    a
}

/// { 0x5454c0 } (cdecl, Database.cpp): `nation_get_english_three_letter_name(nação)`: idioma ([0x9b5984]) 0 ou
/// 1: a própria sigla (+0x53). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de
/// +0x2c, pelo id (sem conferir a quantidade), a sigla em +0x4f da linha. Quadro SEH só no primeiro uso.
unsafe extern "C" fn nation_get_english_three_letter_name(nacao: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return nacao.wrapping_add(0x53); // sigla da nação
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I0%%)(nacao);
    }
    let id = *(nacao as *const i32);
    (*((t + 0x2c) as *const usize)).wrapping_add((id as usize).wrapping_mul(109)).wrapping_add(0x4f) // sigla na linha
}

unsafe extern "C" fn compare_nation_get_english_three_letter_name(nacao: usize) -> usize {
    compare_nome_em_ingles("nation_get_english_three_letter_name", nacao, original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I0%%), nation_get_english_three_letter_name)
}

/// { 0x5455a0 } (cdecl, Database.cpp): `comp_get_english_name(competição)`: idioma 0 ou 1: o próprio nome (+4).
/// Senão a tabela do eng.lng, linhas de 83 bytes: id ≥ [t+0x14] (quantas de clube): a de nação `id − [t+0x14]`
/// em [t+0x28]; senão a de clube `id` em [t+0x38] (comparação com sinal). Quadro SEH só no primeiro uso.
unsafe extern "C" fn comp_get_english_name(comp: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return comp.wrapping_add(4); // nome da competição
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%)(comp);
    }
    let id = *(comp as *const i32);
    let de_clube = *((t + 0x14) as *const i32);
    if id >= de_clube {
        (*((t + 0x28) as *const usize)).wrapping_add((id.wrapping_sub(de_clube) as usize).wrapping_mul(83)) // competição de nação
    } else {
        (*((t + 0x38) as *const usize)).wrapping_add((id as usize).wrapping_mul(83)) // competição de clube
    }
}

unsafe extern "C" fn compare_comp_get_english_name(comp: usize) -> usize {
    compare_nome_em_ingles("comp_get_english_name", comp, original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%), comp_get_english_name)
}

/// { 0x5456a0 } (cdecl, Database.cpp): `staff_comp_get_english_name(competição de pessoal)`: idioma 0 ou 1: o
/// próprio nome (+4). Senão a tabela do eng.lng, linhas de 79 bytes em [t+0x3c], pelo id. SEH só no primeiro uso.
unsafe extern "C" fn staff_comp_get_english_name(comp: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return comp.wrapping_add(4); // nome da competição de pessoal
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I2%%)(comp);
    }
    let id = *(comp as *const i32);
    (*((t + 0x3c) as *const usize)).wrapping_add((id as usize).wrapping_mul(79)) // competição de pessoal na tabela
}

unsafe extern "C" fn compare_staff_comp_get_english_name(comp: usize) -> usize {
    compare_nome_em_ingles("staff_comp_get_english_name", comp, original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I2%%), staff_comp_get_english_name)
}

/// Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação): nem a carreira nem o save passam pelo caminho do eng.lng
/// (o jogo dos testes está em inglês). 512 casos com o idioma de 0 a 13 e uma tabela do eng.lng falsa (nunca
/// nula: o primeiro uso carregaria o arquivo de verdade), quantas de clube de 0 a 9, ids de −2 a 13; as globais
/// voltam no fim.
pub unsafe fn autoteste_dbm_1() {
    let (t0, d0) = SCOREBOARD.counts();
    let mut s = TestRng(0x4442_4d31_6964_696f);
    let globais = [0x9b5984usize, 0xae2408];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // a tabela do eng.lng (new 0x40): +0 .. +0x1c quantos, +0x20 .. +0x3c linhas
    let mut registro = [0u8; 0x60];
    for _ in 0..512 {
        let z = s.next_u64();
        *(0x9b5984 as *mut u32) = (z % 14) as u32;
        tabela[5] = ((z >> 4) % 10) as usize;
        for k in 8..16 {
            tabela[k] = 0x1000_0000 * (k - 7) + ((z >> (8 + k)) as usize & 0xffff0);
        }
        *(0xae2408 as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        let r = registro.as_ptr() as usize;
        compare_nation_get_english_three_letter_name(r);
        compare_comp_get_english_name(r);
        compare_staff_comp_get_english_name(r);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = SCOREBOARD.counts();
    linha(&format!("banco idioma (nomes em inglês 1): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}
