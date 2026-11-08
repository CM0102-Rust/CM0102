// ---------- %%LEVA%%a leva: o nome em inglês do estádio, o .lng do idioma existe? e soltar as tabelas em inglês (Database.cpp) ----------

/// { 0x545990 } (cdecl, Database.cpp): `stadium_get_english_name(estádio)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]), linhas de 52 bytes em [t+0x34], pelo id. Quadro SEH só no
/// primeiro uso (new 0x40 e a carga 0x544fc0), que fica com a original.
unsafe extern "C" fn stadium_get_english_name(estadio: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return estadio.wrapping_add(4); // nome do estádio
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I0%%)(estadio);
    }
    let id = *(estadio as *const i32);
    (*((t + 0x34) as *const usize)).wrapping_add((id as usize).wrapping_mul(52)) // estádio na tabela
}

unsafe extern "C" fn comparar_stadium_get_english_name(estadio: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I0%%);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(estadio); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(estadio);
    let n = stadium_get_english_name(estadio);
    PLACAR.registrar(r == n, || format!("stadium_get_english_name({:x}) idioma {} original={:x} rust={:x}", estadio, idioma, r, n));
    r
}

/// Os nomes dos `.lng` no .rdata do exe por idioma: 0, 1, 11 e 12 "eng.lng"; 2 fra, 3 ger, 4 ita, 5 esp, 6 dan,
/// 7 hol, 8 nor, 9 por, 10 swe; os outros (13, negativos...) nenhum.
const LANGUAGE_FILES: [usize; 13] = [0x9b632c, 0x9b632c, 0x9b633c, 0x9b6334, 0x9b6324, 0x9b630c, 0x9b62fc,
    0x9b6344, 0x9b6314, 0x9b631c, 0x9b6304, 0x9b632c, 0x9b632c];

/// { 0x535930 } (cdecl, Database.cpp): `database_language_file_exists(idioma)`: o caminho `CM3_TEMP\<.lng>`
/// (0x90bde0, no buffer 0xde279c) e se existe (0x90c120: `_access(.., 0) == 0`, 1 ou 0); idioma sem `.lng`, 0
/// sem montar o caminho. TapeCall pelo `load_database` (0x5286e4, o idioma das preferências) e na criação.
unsafe extern "C" fn database_language_file_exists(idioma: i32) -> i32 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x90c120usize);
    match LANGUAGE_FILES.get(idioma as u32 as usize) {
        Some(&nome) => existe(caminho(0x987da0, nome)), // "CM3_TEMP"
        None => 0,
    }
}

/// Comparador: as duas a partir do mesmo buffer do caminho (0xde279c, 0x104 bytes); compara a resposta e o caminho
/// montado (acusa o nome trocado mesmo sem o arquivo na pasta).
unsafe extern "C" fn comparar_database_language_file_exists(idioma: i32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(i32) -> i32>(%%I1%%);
    let (r, x, igual, _, _) = crate::comparacao::lado_a_lado(&[(0xde279c, 0x104)], || o(idioma), || database_language_file_exists(idioma));
    PLACAR.registrar(igual && r == x, || format!("database_language_file_exists({}) original={} rust={} caminho igual {}", idioma, r, x, igual));
    r
}

/// As 8 linhas da tabela do eng.lng (+0x20 .. +0x3c), na ordem, cada uma não nula entregue a `soltar`.
unsafe fn for_each_english_table(t: usize, mut soltar: impl FnMut(usize)) {
    for k in 0..8 {
        let p = *((t + 0x20 + 4 * k) as *const usize); // linha da tabela em inglês
        if p != 0 {
            soltar(p);
        }
    }
}

/// { 0x544f30 } (thiscall, Database.cpp): `eng_lng_free_tables(tabela)`: `free` 0x9452ca de cada uma das 8 linhas
/// não nulas, de +0x20 a +0x3c (os ponteiros ficam; quem chama, `load_database` 0x528503 e a troca de idioma
/// 0x533aa6, apaga a tabela com `delete` 0x944c94 e zera [0xae2408]).
unsafe extern "thiscall" fn eng_lng_free_tables(t: usize) {
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    for_each_english_table(t, |p| free(p));
}

/// Comparador: soltar duas vezes soltaria os mesmos blocos duas vezes.
// ponytail: só a original (como as da 232a); a cópia é provada pelo autoteste_dbm_2 (a ordem dos blocos soltos).
unsafe extern "thiscall" fn comparar_eng_lng_free_tables(t: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(t)
}

/// Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação). (1) O estádio: 256 casos com o idioma de 0 a 13 e uma
/// tabela do eng.lng falsa (nunca nula), ids de −2 a 13. (2) O `.lng` do idioma: idiomas de −3 a 16 contra a
/// original (resposta e caminho montado). (3) Soltar as tabelas: 64 tabelas falsas com linhas nulas ou não
/// sorteadas; a cópia entrega a uma lista (sem soltar nada), que tem de ser as não nulas de +0x20 a +0x3c, na
/// ordem da original (a original solta de verdade e não dá para rodar as duas). As globais voltam no fim.
pub unsafe fn autoteste_dbm_2() {
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x4442_4d32_6573_7461);
    let globais = [0x9b5984usize, 0xae2408];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.proximo();
        *(0x9b5984 as *mut u32) = (z % 14) as u32;
        tabela[13] = 0x3000_0000 + ((z >> 8) as usize & 0xffff0);
        *(0xae2408 as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        comparar_stadium_get_english_name(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    for idioma in -3..=16 {
        comparar_database_language_file_exists(idioma);
    }
    for _ in 0..64 {
        let z = s.proximo();
        for k in 0..8 {
            tabela[8 + k] = if (z >> k) & 1 == 0 { 0 } else { 0x1000_0000 * (k + 1) + ((z >> (8 + k)) as usize & 0xffff0) };
        }
        let esperado: Vec<usize> = tabela[8..].iter().copied().filter(|&p| p != 0).collect();
        let mut soltos = Vec::new();
        for_each_english_table(tabela.as_ptr() as usize, |p| soltos.push(p));
        PLACAR.registrar(soltos == esperado, || format!("eng_lng_free_tables: soltos {:x?} esperados {:x?}", soltos, esperado));
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco idioma (estádio, .lng, soltar): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}
