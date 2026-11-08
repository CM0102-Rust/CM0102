// ---------- %%LEVA%%a leva: o conversor das competições da versão 1 e dois nomes em inglês (Db_files.cpp, Database.cpp) ----------

/// Competição (107 bytes) no club_comp.dat / nation_comp.dat versão 1 (104 no arquivo): `(deslocamento,
/// tamanho, quantos)` na ordem do arquivo; buracos em +0x37 e +0x52; tamanho 0 = a reputação num byte, u16 em +0x69.
const COMPETICAO_V1: [(usize, u32, usize); 7] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 1), (0x57, 1, 2), (0x59, 4, 4), (0x69, 0, 1)];

/// { 0x54ffd0 } (cdecl, Db_files.cpp): `db_files_read_comps_v1(stream, use_zip, registros, n)`: para cada um dos
/// `n` registros de 107 bytes, os campos de `COMPETICAO_V1`, cada um lido sozinho (tamanho, 1) por `zread`
/// 0x932c30 (use_zip ≠ 0) ou `fread` 0x945bfe; a reputação lida num byte (a original o lê num argumento da
/// própria pilha, já usado) vira u16 sem conta. A primeira leitura que não vem dá 0 (o que já veio fica); tudo,
/// 1. Chamado por `db_files_read_comps` 0x54feb0 com a versão da entrada igual a 1; a base de fábrica e o save
/// são versão 2. Sem SEH e sem variável local lida sem valor.
unsafe extern "C" fn db_files_read_comps_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler competições v1");
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = registros + i * 107;
        for &(o, t, q) in &COMPETICAO_V1 {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if ler(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o) as *mut u16).write_unaligned(b as u16); // reputação da competição
                } else if ler(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

/// Comparador: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à posição do `ftell`
/// 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo parou.
// ponytail: com zread (o contêiner) só a original; o save é versão 2 e não chega aqui.
unsafe extern "C" fn comparar_db_files_read_comps_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%);
    if use_zip != 0 {
        return o(stream, use_zip, registros, n);
    }
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x946aaeusize);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(0x946a22usize);
    let pos = ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(registros, n as usize * 107)],
        || {
            let r = o(stream, use_zip, registros, n);
            fim_o = ftell(stream);
            r
        },
        || {
            fseek(stream, pos, 0);
            let r = db_files_read_comps_v1(stream, use_zip, registros, n);
            fim_r = ftell(stream);
            r
        });
    PLACAR.registrar(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("db_files_read_comps_v1(n {}) original={} rust={} arquivo em {} x {} (de {}), 1º byte diferente {:?}", n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}

/// { 0x5453f0 } (cdecl, Database.cpp): `nation_get_english_name(nação)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de +0x2c, pelo id
/// (sem conferir a quantidade).
unsafe extern "C" fn nation_get_english_name(nacao: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return nacao.wrapping_add(4); // idioma 0 ou 1 (nação)
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(%%I1%%)(nacao);
    }
    let id = *(nacao as *const i32);
    (*((t + 0x2c) as *const usize)).wrapping_add((id as usize).wrapping_mul(109))
}

unsafe extern "C" fn comparar_nation_get_english_name(nacao: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(%%I1%%);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(nacao); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(nacao);
    let n = nation_get_english_name(nacao);
    PLACAR.registrar(r == n, || format!("nation_get_english_name({:x}) idioma {} original={:x} rust={:x}", nacao, idioma, r, n));
    r
}

/// { 0x545770 } (cdecl, Database.cpp): `club_get_english_short_name(clube)`: idioma ([0x9b5984]) 0 ou 1: o
/// próprio nome curto (+0x38). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20, o
/// nome curto em +0x34 da linha): pelo mapa de ids [0xae23fc] se houver, senão pelo id se < quantos (+0),
/// senão 0.
unsafe extern "C" fn club_get_english_short_name(clube: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return clube.wrapping_add(0x38);
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca e carrega o eng.lng sob o quadro SEH: a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(%%I2%%)(clube);
    }
    let linhas = *((t + 0x20) as *const usize);
    let id = *(clube as *const i32);
    let mapa = *(0xae23fc as *const usize);
    let i = if mapa != 0 {
        *((mapa.wrapping_add((id as usize).wrapping_mul(4))) as *const i32)
    } else if id < *(t as *const i32) {
        id
    } else {
        return 0;
    };
    linhas.wrapping_add((i as usize).wrapping_mul(79)).wrapping_add(0x34) // nome curto na linha
}

unsafe extern "C" fn comparar_club_get_english_short_name(clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(%%I2%%);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(clube); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(clube);
    let n = club_get_english_short_name(clube);
    PLACAR.registrar(r == n, || format!("club_get_english_short_name({:x}) idioma {} original={:x} rust={:x}", clube, idioma, r, n));
    r
}

/// Autoteste (CM_AUTOTESTE=1, modo comparação): nem a carreira nem o save passam por estas três (a base é
/// versão 2; o jogo dos testes está em inglês). (1) O conversor das competições: 64 casos de sementes fixas,
/// de 1 a 6 registros, arquivo temporário (`autoteste_banco_v1c.tmp` na pasta do jogo, apagado no fim) com
/// bytes sorteados (1 em 4 só zeros), inteiro (1 em 3) ou cortado num ponto sorteado, buffer começando
/// sorteado. (2) Os nomes em inglês: 256 casos com o idioma 0, 1, 2, 3 ou 13 e uma tabela do eng.lng falsa
/// (nunca nula: o primeiro uso carregaria o arquivo de verdade), com e sem o mapa de ids, ids de −2 a 11 (no
/// mapa, de 0 a 7); as globais voltam no fim.
pub unsafe fn autoteste_dbj_2() {
    const ARQ: &str = "autoteste_banco_v1c.tmp";
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x5631_4442_4a32);
    let v1: usize = COMPETICAO_V1.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum();
    for caso in 0..64u64 {
        let n = 1 + (s.proximo() % 6) as usize;
        let total = n * v1;
        let corte = if caso % 3 == 0 { total } else { (s.proximo() % (total as u64 + 1)) as usize };
        let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.proximo() as u8 }).collect();
        if std::fs::write(ARQ, &bytes).is_err() {
            continue;
        }
        let f = fopen(b"autoteste_banco_v1c.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
        if f == 0 {
            continue;
        }
        let mut buf: Vec<u8> = (0..n * 107).map(|_| s.proximo() as u8).collect();
        comparar_db_files_read_comps_v1(f, 0, buf.as_mut_ptr() as usize, n as u32);
        fclose(f);
    }
    let _ = std::fs::remove_file(ARQ);
    let globais = [0x9b5984usize, 0xae2408, 0xae23fc];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // a tabela do eng.lng (new 0x40): +0 quantos clubes, +0x20 clubes, +0x2c nações
    let mut mapa = [0i32; 8];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.proximo();
        *(0x9b5984 as *mut u32) = [0u32, 1, 2, 3, 13][(z % 5) as usize];
        tabela[0] = ((z >> 4) % 10) as usize;
        tabela[8] = 0x1000_0000 + ((z >> 8) as usize & 0xffff0);
        tabela[11] = 0x2000_0000 + ((z >> 24) as usize & 0xffff0);
        *(0xae2408 as *mut usize) = tabela.as_mut_ptr() as usize;
        for (k, m) in mapa.iter_mut().enumerate() {
            *m = ((z >> (40 + k)) % 9) as i32 - 1;
        }
        let com_mapa = (z >> 50) % 2 == 0;
        *(0xae23fc as *mut usize) = if com_mapa { mapa.as_ptr() as usize } else { 0 };
        let id = if com_mapa { ((z >> 52) % 8) as i32 } else { ((z >> 52) % 14) as i32 - 2 };
        *(registro.as_mut_ptr() as *mut i32) = id;
        comparar_nation_get_english_name(registro.as_ptr() as usize);
        comparar_club_get_english_short_name(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco v1 e nomes em inglês: AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}
