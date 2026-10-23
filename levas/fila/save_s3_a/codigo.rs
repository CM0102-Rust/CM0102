// ---------- %%LEVA%%a leva: o save: criar o jogo pelo arquivo de configuração (o início rápido e a carreira automática) (Setup.cpp) ----------

/// A caixa "Error" de Setup.cpp (`aviso_do_setup`) e o jogo para, como a original: 0x5cc7d0,
/// 0x5a8f60 e `exit(-1)` (0x945934).
unsafe fn erro_fatal_do_setup(linha: u32) -> ! {
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    aviso_do_setup(linha);
    fechar1();
    fechar2();
    sair(-1)
}

/// { 0x827110 } (cdecl, Setup.cpp, ~3 KB): `create_game_from_setup_file(cfg, qst)`: lê o arquivo de
/// configuração `cfg` (`auto_setup.cfg` ou `quick_start.cfg`), cria o jogo e, com `qst`, grava o
/// início rápido `CM3_QSTART\<qst>.qst`; sem `qst`, põe o técnico humano de férias num clube
/// sorteado. Devolve 1 se criou.
/// - Pastas CM3_TEMP (0x90bde0, só copiada, como a original) e CM3_DATA. `cfg` nulo ou que não abre
///   (0x945b00 "r"): a caixa "Error" (0x1c13, 0x1c1b) e 0.
/// - Cada linha (0x90cba0, buffer de 1000 bytes) passa por 0x90bb70 e perde os espaços (um a um,
///   como o `strcpy` da original); o valor é procurado por `strstr` (0x945a60) depois do prefixo:
///   `DATABASE=` NORMAL/MINIMUM/MAXIMUM → 0x5d7c60(1/2/4) em [0xae3138]; `MATCH_STATS=`
///   NORMAL/SMALL/LARGE → 0x200/0x400/0x800; `FOREGROUND_LEAGUES=` e `BACKGROUND_LEAGUES=`: na
///   primeira, sem banco ([0xae23a8] = 0), carrega o banco (0x524d50, CM3_DATA; falha para o jogo),
///   aplica (0x5d8480), zera o byte +0x11c das nações, [0xae2d00], [0xae2d04] (e [0xae2d08] na
///   de frente) e monta as nações-chave (0x667150; falha para o jogo); depois, para cada liga da
///   tabela 0xb63d60 (0x23 de 0x48) com nação e +8, se o nome (0x5454c0) está na linha ou 0x839340
///   aceita: de frente, marca 0/1 → ((~opções & 2) | 1) << 1 e conta (+0xae2d00 só do 0,
///   +0xae2d04) e a 1ª que casa pelo `strncmp` (0x946fa0) ou 0x839340 vira [0xae2d08]; de fundo,
///   0 → ((~opções & 2) << 1) | 1 (+[0xae2d00]) e 2 → o mesmo (−[0xae2d04]); `LANGUAGE=`: o 1º
///   idioma i (tabela 0x9eb244, 19) que existe (0x66ea50), é o inglês, não tem banco ou o banco
///   tem (0x535930) e está na linha: 0x5d8020(i) e 0x5d8480; `ENABLE_MATCH_ENGINE=` YES/NO →
///   byte [0xa5a390]; `ENABLE_COMPUTER_BIDS=` → [0x9cd1f8]; `ON_HOLIDAY=` → as férias do técnico.
/// - Fecha; sem liga marcada, sem liga de frente, sem nação padrão ou com ela sem o bit 2: 0.
/// - [0xdd7124] = [0xdd713c] = 1; cria o jogo (0x828110(qst ≠ 0); falha para o jogo).
/// - Com `qst`: 0x945b13 (CM3_QSTART, `qst`, ".qst"), 0x82e430(caminho, 0x17), 0x82ff40(1) e 1.
/// - Sem: "auto_setup_game" em [0xae3138]+0x113, [0xdd7130] = 1, data em 0xdd70a0; se o humano da
///   vez (0xb74c26 + 0x300 × (i16)[0xb77c7a]) já tem pessoa, 0. Senão, a 1ª das 16 últimas pessoas
///   (0xae23bc, 0x6e) com +0x5f = 0 (sem tabela, a caixa 0x39b3 e pessoa 0) vira o humano: texto
///   +0xc zerado, data +0x10 (0x54a130: 1, 0, 1966, −1), +0x18 = 1966, +0x24 e +0x39 = 0, +0x5f = 1,
///   0x5fb810 e 0x5fd2f0(férias, 0) em [0xb63c98]; sorteia (0x90cfc0) clubes (0xae23b4, 0x245) até
///   um fora de seleção (0x53b900) com nação de liga de frente (bit 2) e aceito por 0x544820: a
///   nação dele em +0x1a, os nomes (0x539c60, 0x53a1b0) em +4 e +8, e 0x826f20(pessoa, clube). 1.
unsafe extern "C" fn criar_jogo_pela_configuracao(cfg: *const u8, qst: *const u8) -> u32 {
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(*const u8, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let ler_linha: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(0x90cba0usize);
    let ajeitar: unsafe extern "C" fn(*mut u8) = core::mem::transmute(0x90bb70usize);
    let comeca: unsafe extern "C" fn(*const u8, usize, u32) -> i32 = core::mem::transmute(0x947120usize);
    let procurar: unsafe extern "C" fn(*const u8, usize) -> usize = core::mem::transmute(0x945a60usize);
    let comparar_n: unsafe extern "C" fn(*const u8, *const u8, u32) -> i32 = core::mem::transmute(0x946fa0usize);
    let opcao: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5d7c60usize);
    let aplicar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5d8480usize);
    let trocar_idioma: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5d8020usize);
    let carregar_banco: unsafe extern "C" fn(*const u8, u32) -> i32 = core::mem::transmute(0x524d50usize);
    let nacoes_chave: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x667150usize);
    let nome_da_nacao: unsafe extern "C" fn(usize) -> *const u8 = core::mem::transmute(0x5454c0usize);
    let nacao_casa: unsafe extern "C" fn(usize, *const u8) -> i32 = core::mem::transmute(0x839340usize);
    let idioma_existe: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x66ea50usize);
    let idioma_no_banco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x535930usize);
    let criar_jogo: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x828110usize);
    let juntar: unsafe extern "C" fn(*mut u8, usize, *const u8, *const u8, usize) = core::mem::transmute(0x945b13usize);
    let gravar_jogo: unsafe extern "C" fn(*const u8, u32) -> u32 = core::mem::transmute(0x82e430usize);
    let liberar: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x82ff40usize);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
    let novo_humano: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5fb810usize);
    let ferias_do_humano: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fd2f0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let aceita: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let primeiro_nome: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x539c60usize);
    let segundo_nome: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53a1b0usize);
    let pos_humano: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x826f20usize);
    crate::comparacao::ramo("save criar jogo pela configuracao");
    let opcoes = || *(0xae3138 as *const usize);
    let u = |a: usize| *(a as *const u32);
    let i = |a: usize| *(a as *const i32);
    let mut temporaria = [0u8; 0x104];
    copiar_texto(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    let mut dados = [0u8; 0x104];
    copiar_texto(dados.as_mut_ptr(), pasta(0x9b5348, 0xad9c64));
    if cfg.is_null() {
        aviso_do_setup(0x1c13);
        return 0;
    }
    let f = abrir(cfg, 0x9899fc);
    if f == 0 {
        aviso_do_setup(0x1c1b);
        return 0;
    }
    let mut ferias = 0u32;
    let mut linha = [0u8; 0x3e8];
    let b = linha.as_mut_ptr();
    // sem banco: carrega, aplica, zera as marcas das nações e as contagens, monta as nações-chave
    let preparar_banco = |de_frente: bool, erro_banco: u32, erro_chave: u32| {
        if carregar_banco(dados.as_ptr(), 1) == 0 {
            erro_fatal_do_setup(erro_banco);
        }
        aplicar(opcoes());
        let mut k = 0i32;
        let mut d = 0usize;
        while k < i(0xae2350) {
            k += 1;
            *((u(0xae23a8) as usize + d + 0x11c) as *mut u8) = 0;
            d += 0x122;
        }
        *(0xae2d00 as *mut u32) = 0;
        *(0xae2d04 as *mut u32) = 0;
        if de_frente {
            *(0xae2d08 as *mut u32) = 0;
        }
        if nacoes_chave() == 0 {
            erro_fatal_do_setup(erro_chave);
        }
    };
    while ler_linha(f, b) != 0 {
        ajeitar(b);
        if *b != 0 {
            let mut p = b;
            loop {
                if *p == b' ' {
                    copiar_texto(p, p.add(1));
                }
                p = p.add(1);
                if *p == 0 {
                    break;
                }
            }
        }
        let tem = |o: usize, valor: usize| procurar(b.add(o), valor) != 0;
        if comeca(b, 0xa81348, 9) == 0 {
            // DATABASE=
            if tem(9, 0xa81340) {
                opcao(opcoes(), 1);
            } else if tem(9, 0xa81338) {
                opcao(opcoes(), 2);
            } else if tem(9, 0xa81330) {
                opcao(opcoes(), 4);
            }
        } else if comeca(b, 0xa81320, 0xc) == 0 {
            // MATCH_STATS=
            if tem(0xc, 0xa81340) {
                opcao(opcoes(), 0x200);
            } else if tem(0xc, 0xa81318) {
                opcao(opcoes(), 0x400);
            } else if tem(0xc, 0xa81310) {
                opcao(opcoes(), 0x800);
            }
        } else if comeca(b, 0xa812fc, 0x13) == 0 {
            // FOREGROUND_LEAGUES=
            if u(0xae23a8) == 0 {
                preparar_banco(true, 0x1c49, 0x1c5d);
            }
            let v = b.add(0x13);
            let mut liga = 0xb63d68usize;
            while liga < 0xb64740 {
                let n = u(liga - 8) as usize;
                if n != 0 && u(liga) != 0 && (procurar(v, nome_da_nacao(n) as usize) != 0 || nacao_casa(n, v) != 0) {
                    let m = (n + 0x11c) as *mut u8;
                    let marca = ((!(u(opcoes() + 4) as u8) & 2) | 1) << 1;
                    if *m == 0 {
                        *m = marca;
                        *(0xae2d00 as *mut i32) += 1;
                        *(0xae2d04 as *mut i32) += 1;
                    } else if *m == 1 {
                        *m = marca;
                        *(0xae2d04 as *mut i32) += 1;
                    }
                    let tam = core::ffi::CStr::from_ptr(nome_da_nacao(n) as *const core::ffi::c_char).to_bytes().len() as u32;
                    if (comparar_n(v, nome_da_nacao(n), tam) == 0 || nacao_casa(n, v) != 0) && u(0xae2d08) == 0 {
                        *(0xae2d08 as *mut u32) = n as u32;
                    }
                }
                liga += 0x48;
            }
        } else if comeca(b, 0xa812e8, 0x13) == 0 {
            // BACKGROUND_LEAGUES=
            if u(0xae23a8) == 0 {
                preparar_banco(false, 0x1c92, 0x1ca3);
            }
            let v = b.add(0x13);
            let mut liga = 0xb63d68usize;
            while liga < 0xb64740 {
                let n = u(liga - 8) as usize;
                if n != 0 && u(liga) != 0 && (procurar(v, nome_da_nacao(n) as usize) != 0 || nacao_casa(n, v) != 0) {
                    let m = (n + 0x11c) as *mut u8;
                    if *m == 0 {
                        *m = ((!(u(opcoes() + 4) as u8) & 2) << 1) | 1;
                        *(0xae2d00 as *mut i32) += 1;
                    } else if *m == 2 {
                        *m = ((!(u(opcoes() + 4) as u8) & 2) << 1) | 1;
                        *(0xae2d04 as *mut i32) -= 1;
                    }
                }
                liga += 0x48;
            }
        } else if comeca(b, 0xa812dc, 9) == 0 {
            // LANGUAGE=
            let mut k = 0u32;
            let mut t = 0x9eb244usize;
            while t < 0x9eb290 {
                if idioma_existe(k) != 0
                    && (t == 0x9eb244 || u(0xae23a8) == 0 || idioma_no_banco(k) != 0)
                    && tem(9, u(t) as usize)
                {
                    trocar_idioma(opcoes(), k);
                    aplicar(opcoes());
                    break;
                }
                t += 4;
                k += 1;
            }
        } else if comeca(b, 0xa812c4, 0x14) == 0 {
            // ENABLE_MATCH_ENGINE=
            if tem(0x14, 0xa812c0) {
                *(0xa5a390 as *mut u8) = 1;
            } else if tem(0x14, 0xa812bc) {
                *(0xa5a390 as *mut u8) = 0;
            }
        } else if comeca(b, 0xa812a4, 0x15) == 0 {
            // ENABLE_COMPUTER_BIDS=
            if tem(0x15, 0xa812c0) {
                *(0x9cd1f8 as *mut u32) = 1;
            } else if tem(0x15, 0xa812bc) {
                *(0x9cd1f8 as *mut u32) = 0;
            }
        } else if comeca(b, 0xa81298, 0xb) == 0 {
            // ON_HOLIDAY=
            if tem(0xb, 0xa812c0) {
                ferias = 1;
            } else if tem(0xb, 0xa812bc) {
                ferias = 0;
            }
        }
    }
    fechar(f);
    if i(0xae2d00) <= 0 || i(0xae2d04) <= 0 {
        return 0;
    }
    let padrao = u(0xae2d08) as usize;
    if padrao == 0 || *((padrao + 0x11c) as *const u8) & 2 == 0 {
        return 0;
    }
    *(0xdd7124 as *mut u32) = 1;
    *(0xdd713c as *mut u32) = 1;
    if criar_jogo(!qst.is_null() as u32) == 0 {
        erro_fatal_do_setup(0x1d11);
    }
    if !qst.is_null() {
        let mut arquivo = [0u8; 0x104];
        juntar(arquivo.as_mut_ptr(), 0, pasta(0xa80204, 0xad9c64), qst, 0xa803b4);
        gravar_jogo(arquivo.as_ptr(), 0x17);
        liberar(1);
        return 1;
    }
    copiar_texto((opcoes() + 0x113) as *mut u8, 0xa81288 as *const u8);
    *(0xdd7130 as *mut u32) = 1;
    let mut data = [0u8; 8];
    copia_data(0xdd70a0, data.as_mut_ptr(), 0xae2c90);
    let vaga = || 0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as isize as usize).wrapping_mul(0x300));
    if core::ptr::read_unaligned(vaga() as *const u32) != 0 {
        return 0;
    }
    let tabela = u(0xae23bc) as usize;
    let mut p = 0usize;
    if tabela == 0 {
        aviso_do_setup(0x39b3);
    } else {
        let mut q = tabela.wrapping_add((i(0xae2364) as usize).wrapping_mul(0x6e)).wrapping_sub(0x6e);
        for _ in 0..0x10 {
            if *((q + 0x5f) as *const u8) == 0 {
                p = q;
                break;
            }
            q = q.wrapping_sub(0x6e);
        }
    }
    core::ptr::write_unaligned(vaga() as *mut u32, p as u32);
    *core::ptr::read_unaligned((p + 0xc) as *const *mut u8) = 0;
    acertar_data(p + 0x10, 1, 0, 0x7ae, -1);
    core::ptr::write_unaligned((p + 0x18) as *mut u16, 0x7ae);
    core::ptr::write_unaligned((p + 0x24) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x39) as *mut u32, 0);
    *((p + 0x5f) as *mut u8) = 1;
    novo_humano(u(0xb63c98) as usize, p);
    ferias_do_humano(u(0xb63c98) as usize, ferias, 0);
    let clube = loop {
        let r = sorteio(i(0xae235c));
        let c = (u(0xae23b4) as usize).wrapping_add((r as usize).wrapping_mul(0x245));
        if selecao(c) != 0 {
            continue;
        }
        let nacao = core::ptr::read_unaligned((c + 0x53) as *const usize);
        if nacao == 0 || *((nacao + 0x11c) as *const u8) & 2 == 0 || aceita(c) == 0 {
            continue;
        }
        break c;
    };
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    core::ptr::write_unaligned((p + 0x1a) as *mut usize, nacao);
    core::ptr::write_unaligned((p + 4) as *mut u32, primeiro_nome(nacao));
    let n2 = segundo_nome(core::ptr::read_unaligned((p + 0x1a) as *const usize));
    core::ptr::write_unaligned((p + 8) as *mut u32, n2);
    pos_humano(p, clube);
    1
}

// ponytail: cria o jogo inteiro (carrega o banco, sorteia, grava o .qst e libera tudo); não roda
// duas vezes. Só a original; a cópia leva o ramo e é provada pela prova do save (o .qst) e pelas
// carreiras (o auto_setup.cfg).
unsafe extern "C" fn comparar_criar_jogo_pela_configuracao(cfg: *const u8, qst: *const u8) -> u32 {
    orig::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(%%I0%%)(cfg, qst)
}
