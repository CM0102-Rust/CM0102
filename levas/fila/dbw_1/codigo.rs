// ---------- %%LEVA%%a leva: as chaves fixas do jogo: os ids pela base, as cores e os continentes, e o relato das que faltam (index.cpp) ----------

// As listas da set_key_ids, geradas do exe por `python engenharia-reversa/chaves_fixas.py` (rust/src/chaves_fixas.rs).
include!("chaves_fixas.rs");

/// Uma cadeia de _stricmp (0x951c50) em linha (cores, continentes): o primeiro texto igual ao `nome` grava `linha` na
/// global dele e a cadeia acaba.
unsafe fn cadeia_das_chaves_dbw(nome: usize, pares: &[(u32, u32)], linha: u32) {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    for &(texto, global) in pares {
        if stricmp(nome, texto as usize) == 0 {
            *(global as *mut u32) = linha;
            return;
        }
    }
}

/// Um laço da set_key_ids: `i` de 0 enquanto `i < limite()` (relido a cada volta, com sinal). Devolve se rodou (a
/// original só põe o -1 de volta no edi depois de um laço que rodou).
unsafe fn laco_das_chaves_dbw(limite: impl Fn() -> i32, mut f: impl FnMut(i32)) -> bool {
    let mut i = 0i32;
    while i < limite() {
        f(i);
        i += 1;
    }
    i > 0
}

/// A caixa fatal do relato (index.cpp, `linha`): "v%s %s.%s %d" em 0xae24d0 (0x90d130 com o nome e a extensão do
/// index.cpp), a caixa "Error" (0x5e8290), [0xb67a34] = 0, 0x5cc7d0, 0x5a8f60 e exit(marca). Não volta.
unsafe fn caixa_fatal_das_chaves_dbw(linha: u32, marca: u32) -> ! {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let antes_de_sair1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let antes_de_sair2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(u32) -> ! = core::mem::transmute(0x945934usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9d93cc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    // a original não desempilha os argumentos da 0x90d130: são o resto do "v%s %s.%s %d"
    formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
    antes_de_sair1();
    antes_de_sair2();
    sair(marca)
}

/// O miolo da set_key_ids depois do -1 (o -1 é de cada entrada). `marca` = o edi na entrada (-1 na original;
/// MARCA_DAS_CHAVES_1080 no 1080, que só vale se nenhum laço rodar); `relatar` = o relato das chaves sem valor (no
/// modo comparando a cópia roda sem ele, para a caixa não aparecer duas vezes). Devolve 1.
/// 1) As cores ([0xae2394] de [0xae23ec] + 4, passo 0x3a): CORES_DAS_CHAVES. 2) Com [0x9b5984] ≠ 0 e
/// CM3_TEMP\eng.lng aberto: 8 contagens (bytes invertidos se 0x90c140(f) = 0xf1fffff1) e as tabelas na ordem do
/// arquivo: clubes c0 × 0x4f (0x6146b0; pela lista [0xae23fc] até [0xae235c], se houver), continentes c1 × 0xba,
/// competições c2 × 0x53 (0x60efd0 com a linha c5 + i), nações c3 × 0x6d (0x60d810), pula c6 × 0x34 e c4 × 0x34,
/// prêmios c7 × 0x4f (0x6115e0), competições c5 × 0x53 (0x60efd0); tabela sem memória: fseek por cima. 3) Senão, pela
/// base: nações (0x60d810), competições de clube até [0xae2378] − 0x7f (0x60efd0), continentes, prêmios (0x6115e0) e
/// clubes (0x6146b0). 4) O relato (RELATO_DAS_CHAVES): a 0x60d780 (a caixa "Unable to find the %s index"); as 7
/// fatais saem do jogo se a nação da chave tiver o byte +0x11c.
unsafe fn chaves_fixas(marca_entrada: u32, relatar: bool) -> u32 {
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const u32);
    let em = |base: u32, i: i32, passo: u32| base.wrapping_add((i as u32).wrapping_mul(passo)) as usize;
    let clube: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x6146b0usize);
    let nacao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x60d810usize);
    let competicao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x60efd0usize);
    let premio: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x6115e0usize);
    let mut marca = marca_entrada;
    if laco_das_chaves_dbw(|| gi(0xae2394), |i| cadeia_das_chaves_dbw(em(rd(0xae23ec), i, 0x3a) + 4, &CORES_DAS_CHAVES, i as u32)) {
        marca = u32::MAX;
    }
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let f = if gi(0x9b5984) != 0 { abrir(caminho(0x987da0, 0x9b632c), 0x98f928) } else { 0 }; // CM3_TEMP\eng.lng, "rb"
    if f != 0 {
        let versao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x90c140usize);
        let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        let pular: unsafe extern "C" fn(usize, u32, i32) -> i32 = core::mem::transmute(0x946a22usize);
        let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
        let inverter = versao(f) == 0xf1fffff1;
        let mut c = [0i32; 8];
        ler(c.as_mut_ptr() as usize, 0x20, 1, f);
        if inverter {
            for x in c.iter_mut() {
                *x = x.swap_bytes();
            }
        }
        // a tabela de `n` linhas de `passo` bytes: lida e entregue a `f` (o fseek por cima se a memória faltar)
        let tabela = |n: i32, passo: u32, f_: &mut dyn FnMut(u32)| {
            let b = alocar((n as u32).wrapping_mul(passo));
            if b != 0 {
                ler(b, passo, n as u32, f);
                f_(b as u32);
                liberar(b);
            } else {
                pular(f, (n as u32).wrapping_mul(passo), 1);
            }
        };
        tabela(c[0], 0x4f, &mut |b| {
            let mut i = 0i32;
            loop {
                let lista = rd(0xae23fc); // a lista de clubes da base
                if i >= if lista != 0 { gi(0xae235c) } else { c[0] } {
                    break;
                }
                let linha = if lista != 0 { em(b, gi(em(lista, i, 4)), 0x4f) } else { em(b, i, 0x4f) };
                clube(linha, i as u32);
                i += 1;
            }
        });
        tabela(c[1], 0xba, &mut |b| {
            laco_das_chaves_dbw(|| c[1], |i| cadeia_das_chaves_dbw(em(b, i, 0xba), &CONTINENTES_DAS_CHAVES, i as u32)); // continentes do eng.lng
        });
        tabela(c[2], 0x53, &mut |b| {
            laco_das_chaves_dbw(|| c[2], |i| { competicao(em(b, i, 0x53), c[5].wrapping_add(i) as u32); }); // competições do eng.lng depois das c5
        });
        tabela(c[3], 0x6d, &mut |b| {
            laco_das_chaves_dbw(|| c[3], |i| { nacao(em(b, i, 0x6d), i as u32); }); // nações do eng.lng
        });
        // os prêmios: a memória antes de pular as duas tabelas de 0x34
        let b = alocar((c[7] as u32).wrapping_mul(0x4f));
        pular(f, (c[6] as u32).wrapping_mul(0x34), 1);
        pular(f, (c[4] as u32).wrapping_mul(0x34), 1);
        if b != 0 {
            ler(b, 0x4f, c[7] as u32, f);
            laco_das_chaves_dbw(|| c[7], |i| { premio(em(b as u32, i, 0x4f), i as u32); }); // prêmios do eng.lng
            liberar(b);
        } else {
            pular(f, (c[7] as u32).wrapping_mul(0x4f), 1);
        }
        tabela(c[5], 0x53, &mut |b| {
            laco_das_chaves_dbw(|| c[5], |i| { competicao(em(b, i, 0x53), i as u32); }); // competições do eng.lng
        });
        fechar(f);
        marca = u32::MAX;
    } else {
        let mut rodou = laco_das_chaves_dbw(|| gi(0xae2350), |i| { nacao(em(rd(0xae23a8), i, 0x122) + 4, i as u32); });
        rodou |= laco_das_chaves_dbw(|| gi(0xae2378).wrapping_sub(0x7f), |i| { competicao(em(rd(0xae23d0), i, 0x6b) + 4, i as u32); });
        rodou |= laco_das_chaves_dbw(|| gi(0xae234c), |i| {
            let nome = em(rd(0xae23a4), i, 0xc6) + 4;
            cadeia_das_chaves_dbw(nome, &CONTINENTES_DAS_CHAVES, i as u32); // continentes da base
        });
        rodou |= laco_das_chaves_dbw(|| gi(0xae2388), |i| { premio(em(rd(0xae23e0), i, 0x65) + 4, i as u32); });
        rodou |= laco_das_chaves_dbw(|| gi(0xae235c), |i| { clube(em(rd(0xae23b4), i, 0x245) + 4, i as u32); }); // clubes da base
        if rodou {
            marca = u32::MAX;
        }
    }
    if relatar {
        let relato: unsafe extern "C" fn(usize) = core::mem::transmute(0x60d780usize);
        for &(chave, nome, tipo, nacao_da_chave, linha) in RELATO_DAS_CHAVES.iter() {
            if rd(chave as usize) != marca {
                continue;
            }
            if tipo == 2 {
                let n = em(rd(0xae23a8), gi(nacao_da_chave as usize), 0x122);
                if *((n as u32).wrapping_add(0x11c) as *const u8) != 0 {
                    caixa_fatal_das_chaves_dbw(linha as u32, marca);
                }
            }
            relato(nome as usize); // tipos 0 e 1 (o 1 é o corpo da 0x60d780 em linha)
        }
    }
    1
}

/// As globais em -1 na original (MENOS_UM_DAS_CHAVES).
unsafe fn menos_um_das_chaves_dbw() {
    for &g in MENOS_UM_DAS_CHAVES.iter() {
        *(g as *mut i32) = -1;
    }
}

/// As faixas inteiras em -1 no 1080 (MENOS_UM_DAS_CHAVES_1080, o rep stosd).
unsafe fn menos_um_das_chaves_1080_dbw() {
    for &(a, n) in MENOS_UM_DAS_CHAVES_1080.iter() {
        core::ptr::write_bytes(a as *mut u8, 0xff, n as usize * 4);
    }
}

/// { 0x601a40 } (cdecl, index.cpp, set_key_ids): na carga da base (0x528540), as chaves fixas do jogo (cores,
/// continentes, nações, competições, prêmios, clubes) recebem a linha da base pelo nome; as que ficam sem valor vão ao
/// relato. A entrada do cm0102.exe: o -1 nas 1.282 globais de MENOS_UM_DAS_CHAVES e o miolo `chaves_fixas`. Devolve 1.
unsafe extern "C" fn definir_chaves_fixas() -> u32 {
    menos_um_das_chaves_dbw();
    chaves_fixas(u32::MAX, true)
}

/// { 0x603834 } (cdecl, só no cm0102-1080.exe): a mesma rotina pela entrada do 1080 (a chamada de 0x52d8fa vai para
/// ela): o -1 nas faixas inteiras 0x9cf178..0x9d0527 e 0xb63c9c..0xb63d03 (também as 4 globais que a original não toca)
/// e o mesmo miolo. Devolve 1.
unsafe extern "C" fn definir_chaves_fixas_1080() -> u32 {
    menos_um_das_chaves_1080_dbw();
    chaves_fixas(MARCA_DAS_CHAVES_1080, true)
}

/// As regiões das chaves fixas: as globais das chaves, as chaves de clube altas e 0xde279c.
const REGIOES_DAS_CHAVES_DBW: [(usize, usize); 3] = [(0x9cf178, 0x13b0), (0xb63c9c, 0x68), (0xde279c, 0x104)];

unsafe extern "C" fn comparar_definir_chaves_fixas() -> u32 {
    let o = orig::<unsafe extern "C" fn() -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&REGIOES_DAS_CHAVES_DBW, || o(), || {
        menos_um_das_chaves_dbw();
        chaves_fixas(u32::MAX, false)
    });
    PLACAR.registrar(igual && r == n, || format!("definir_chaves_fixas() original={} rust={} {}", r, n, chaves_diferentes(&mo, &mr)));
    r
}

unsafe extern "C" fn comparar_definir_chaves_fixas_1080() -> u32 {
    let o = orig::<unsafe extern "C" fn() -> u32>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&REGIOES_DAS_CHAVES_DBW, || o(), || {
        menos_um_das_chaves_1080_dbw();
        chaves_fixas(MARCA_DAS_CHAVES_1080, false)
    });
    PLACAR.registrar(igual && r == n, || format!("definir_chaves_fixas_1080() original={} rust={} {}", r, n, chaves_diferentes(&mo, &mr)));
    r
}
