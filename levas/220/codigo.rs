// ---------- 220a leva: o save: criar o jogo pelo arquivo de configuração (o início rápido e a carreira automática) (Setup.cpp) ----------

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
    orig::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(574)(cfg, qst)
}

// ---------- 220a leva: o save: gravar o jogo inteiro (save_game_data) (Setup.cpp) ----------

/// A caixa de erro da gravação: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de Setup.cpp,
/// `linha`, `texto`) em 0xae24d0; o título `titulo` traduzido (0x66f4e0) em 0xae28bc, o corpo
/// `corpo` (com o texto de 0xae24d0) traduzido em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está, ao contrário do `aviso_do_setup`).
unsafe fn caixa_do_save(linha: u32, texto: usize, titulo: usize, corpo: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, titulo);
    traduzir(0xae26c4, corpo, 0xae24d0);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Os passos 2..=0x24 da gravação, na ordem da original: (global que tem de ser ≠ 0, rotina, como
/// chamar). `this` é o próprio global, menos no 0x5afad0 (FOG_OF_WAR), que a original confere por
/// [0xb6b7c8] (o de Media) e chama com [0xae2a60]. O passo 0x19 (0x6692f0, as competições) não
/// tem global e devolve 0 quando falha.
#[derive(Clone, Copy)]
enum PassoDoSave {
    /// thiscall(this, caminho), ret 4
    Caminho(usize, usize),
    /// thiscall(this), sem argumento
    Sozinho(usize, usize),
    /// thiscall(this, 1), ret 4 (FIXTURE_MANAGER::save_fixtures)
    Um(usize, usize),
    /// o método +0x1c da vtable de [global](1) (Friendly)
    Virtual(usize),
    /// confere o 1º global e chama a rotina com o 2º (FOG_OF_WAR)
    Outro(usize, usize, usize),
    /// cdecl() → 0 é erro (key_nation_save_all_comps)
    Competicoes,
}

const PASSOS_DO_SAVE: [PassoDoSave; 0x23] = [
    PassoDoSave::Caminho(0xb63c98, 0x5ff9e0), // 2 HUMAN_MANAGER
    PassoDoSave::Caminho(0xb74448, 0x7f8020), // 3 SCOUT_MANAGER
    PassoDoSave::Caminho(0xb7431c, 0x787c30), // 4 News
    PassoDoSave::Virtual(0xae2c74),           // 5 Friendly
    PassoDoSave::Sozinho(0xde1f58, 0x905510), // 6 UefaSeeding
    PassoDoSave::Sozinho(0xae1000, 0x4c6b70), // 7 ConmebolSeeding
    PassoDoSave::Caminho(0xdd7f00, 0x86ffd0), // 8 Staff_Records
    PassoDoSave::Caminho(0xb6b7c0, 0x72adf0), // 9 Match_Stats
    PassoDoSave::Caminho(0xb74350, 0x7c55a0), // 0xa PlayerStats
    PassoDoSave::Caminho(0xad9da8, 0x443f50), // 0xb Club_History
    PassoDoSave::Caminho(0xad9dac, 0x451ff0), // 0xc Club_Records
    PassoDoSave::Caminho(0xde1d08, 0x8995d0), // 0xd Tactics
    PassoDoSave::Sozinho(0xb5e134, 0x5fb320), // 0xe HostCountry
    PassoDoSave::Caminho(0xae28f0, 0x555dd0), // 0xf Discipline
    PassoDoSave::Caminho(0xad9c60, 0x4141a0), // 0x10 AWARD_MANAGER
    PassoDoSave::Caminho(0xae2a3c, 0x5a53d0), // 0x11 Fine
    PassoDoSave::Caminho(0xad9c6c, 0x418980), // 0x12 AWOL
    PassoDoSave::Caminho(0xae2a30, 0x569ae0), // 0x13 DISPUTE
    PassoDoSave::Caminho(0xb7433c, 0x794f00), // 0x14 Notes
    PassoDoSave::Caminho(0xb63d28, 0x62b2d0), // 0x15 Injury
    PassoDoSave::Caminho(0xb74318, 0x776a20), // 0x16 NationalTeams
    PassoDoSave::Sozinho(0xdd7ef4, 0x85bea0), // 0x17 Stadium
    PassoDoSave::Um(0xae2a58, 0x5ac060),      // 0x18 FIXTURE_MANAGER
    PassoDoSave::Competicoes,                 // 0x19 key_nation_save_all_comps
    PassoDoSave::Caminho(0xae1050, 0x4dc3a0), // 0x1a CONTRACT_MANAGER
    PassoDoSave::Caminho(0xae2a38, 0x59b3b0), // 0x1b FinanceTable
    PassoDoSave::Caminho(0xde1f54, 0x8bd4b0), // 0x1c TRANSFER_MANAGER
    PassoDoSave::Caminho(0xb7434c, 0x7ad7c0), // 0x1d PLAYER_REGEN
    PassoDoSave::Caminho(0xb6b7c8, 0x72b5f0), // 0x1e Media
    PassoDoSave::Outro(0xb6b7c8, 0xae2a60, 0x5afad0), // 0x1f FOG_OF_WAR
    PassoDoSave::Caminho(0xb67a44, 0x68e920), // 0x20 MANAGER_MANAGER
    PassoDoSave::Caminho(0xb74340, 0x79bb80), // 0x21 OfficialsManager
    PassoDoSave::Caminho(0xdd7ee4, 0x84c680), // 0x22 SimulatedStats
    PassoDoSave::Caminho(0xde1f44, 0x8b1450), // 0x23 TRAINING_MANAGER
    PassoDoSave::Caminho(0xdd7ef0, 0x859cc0), // 0x24 SQUAD_MANAGER
];

/// { 0x82e430 } (cdecl, Setup.cpp, ~6,8 KB, SEH): `save_game_data(caminho, a4)`: grava o jogo
/// inteiro no contêiner `caminho` (a4 0x16 `.sav`, 0x17 `.qst`; tipo 3 com o bit 0x8000 do
/// `game.cfg` [[0xae3138]+4], senão 4 = RLE). Devolve 1 se gravou.
/// - [0xdd7110] = 0xfffc1fb9 (a versão do save). Em rede (bit 0x20000000), antes de tudo, disco
///   (0x7a1820(0) < 25) liga [0xdd7134] e memória ([0x9cd1fc] e 0x7a1710() < 1) liga [0xdd7138]:
///   apaga `caminho` e devolve 0.
/// - Pasta temporária (0x90bde0 "CM3_TEMP"); `caminho` nulo dá a caixa "Error" (0x2958). O
///   caminho de trabalho é `caminho` ou, em rede, o mesmo com o nome `temp_save_file`
///   (0x944cff/0x945b13); se já existe (0x90c120) e não sai (0x94643a = −1), devolve 0. Com o bit
///   0x40000000, disco e memória de novo (apagando `caminho`).
/// - 0x524d30 (o banco, passo 1 do save); disco e memória (apagando o caminho de trabalho);
///   "Saving game data" traduzido e a barra 0x802f80 (passo 1 de 0x25).
/// - `general.dat` (0x9326e0): 16 campos (a versão, o resumo 0x591f80 de 0xf74 bytes, o id único,
///   o tamanho do banco, as datas e horas, as ligas, a nação padrão ou −1, os dados de execução de
///   pessoas, clubes e reputação); erro de escrita dá a caixa "Disk Space Error" com a linha do
///   campo e fecha o item.
/// - Os passos 2..=0x24 (`PASSOS_DO_SAVE`): disco e memória (antes do 0x11, sem apagar e com
///   memória < 25), a barra e o `save` do subsistema; 0x25: empacota os `*.tmp` (0x933610).
/// - Hall da fama (0x5f00c0, fora do save), regras FIFA (0x8395b0); em rede, troca o save antigo
///   pelo novo (0x945ba4); [0xdd7130] = 1, [0xa801bc] = 0 e a data do último save (0xdd70a0).
/// A data local da original ([esp+0x1758], 0x54a110, destrutor vazio 0x5a53c0 no SEH) não é usada:
/// fica de fora. O laço da original ([esp+0x11c] de 1 a 2) roda o corpo uma vez, com o total 0x25.
/// O resumo é um buffer zerado aqui; na original, o que o 0x591f80 não escreve (o clube dos 16
/// técnicos não humanos, o resto dos textos) é sobra da pilha dela, gravada no `general.dat`.
unsafe extern "C" fn gravar_jogo(caminho: *const u8, a4: u32) -> u32 {
    let disco: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7a1820usize);
    let memoria: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x7a1710usize);
    let apagar: unsafe extern "C" fn(*const u8) -> i32 = core::mem::transmute(0x94643ausize);
    let pasta: unsafe extern "C" fn(usize, usize) -> *const u8 = core::mem::transmute(0x90bde0usize);
    let existe: unsafe extern "C" fn(*const u8) -> i32 = core::mem::transmute(0x90c120usize);
    let separar: unsafe extern "C" fn(*const u8, *mut u8, *mut u8, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let juntar: unsafe extern "C" fn(*mut u8, *const u8, *const u8, usize, *const u8) = core::mem::transmute(0x945b13usize);
    let renomear: unsafe extern "C" fn(*const u8, *const u8) -> i32 = core::mem::transmute(0x945ba4usize);
    let gravar_banco: unsafe extern "C" fn(*const u8, u32, u32) -> u32 = core::mem::transmute(0x524d30usize);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let barra: unsafe extern "thiscall" fn(usize, *const u8, u32, u32, i32) = core::mem::transmute(0x802f80usize);
    let abrir_item: unsafe extern "C" fn(*const u8, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    let montar_resumo: unsafe extern "C" fn(*mut u8) = core::mem::transmute(0x591f80usize);
    let empacotar: unsafe extern "C" fn(*const u8, *const u8, usize, u32, u32) -> i32 = core::mem::transmute(0x933610usize);
    let competicoes: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x6692f0usize);
    let hall_da_fama: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5f00c0usize);
    let regras_fifa: unsafe extern "C" fn(*const u8) = core::mem::transmute(0x8395b0usize);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    crate::comparacao::ramo("save gravar jogo");
    let opcoes = || *((*(0xae3138 as *const usize) + 4) as *const u32);
    let tipo = || if opcoes() & 0x8000 != 0 { 3u32 } else { 4 };
    let sem_memoria = || *(0x9cd1fc as *const u32) != 0 && memoria() < 1;
    *(0xdd7110 as *mut u32) = 0xfffc1fb9;
    if opcoes() & 0x20000000 != 0 {
        let falta = if disco(0) < 0x19 {
            *(0xdd7134 as *mut u32) = 1;
            true
        } else if sem_memoria() {
            *(0xdd7138 as *mut u32) = 1;
            true
        } else {
            false
        };
        if falta {
            if !caminho.is_null() {
                apagar(caminho);
            }
            return 0;
        }
    }
    let mut temporaria = [0u8; 0x104];
    copiar_texto(temporaria.as_mut_ptr(), pasta(0x987da0, 0xad9c64));
    if caminho.is_null() {
        aviso_do_setup(0x2958);
        return 0;
    }
    let mut local = [0u8; 0x104];
    if opcoes() & 0x20000000 != 0 {
        let (mut unidade, mut dir, mut nome, mut ext) = ([0u8; 0x104], [0u8; 0x104], [0u8; 0x100], [0u8; 0x100]);
        separar(caminho, unidade.as_mut_ptr(), dir.as_mut_ptr(), nome.as_mut_ptr(), ext.as_mut_ptr());
        juntar(local.as_mut_ptr(), unidade.as_ptr(), dir.as_ptr(), 0xa81cf4, ext.as_ptr());
    } else {
        copiar_texto(local.as_mut_ptr(), caminho);
    }
    let l = local.as_ptr();
    if existe(l) != 0 && apagar(l) == -1 {
        return 0;
    }
    if opcoes() & 0x40000000 != 0 {
        if disco(0) < 0x19 {
            *(0xdd7134 as *mut u32) = 1;
            apagar(caminho);
            return 0;
        }
        if sem_memoria() {
            *(0xdd7138 as *mut u32) = 1;
            apagar(caminho);
            return 0;
        }
    }
    gravar_banco(l, a4, tipo());
    // disco e memória entre os passos: liga a marca, apaga o caminho de trabalho e devolve 0
    let falta_espaco = || {
        if disco(0) < 0x19 {
            *(0xdd7134 as *mut u32) = 1;
            apagar(l);
            return true;
        }
        if sem_memoria() {
            *(0xdd7138 as *mut u32) = 1;
            apagar(l);
            return true;
        }
        false
    };
    if falta_espaco() {
        return 0;
    }
    let mut texto = [0u8; 0xc8];
    traduzir(texto.as_mut_ptr(), 0xa81ce0);
    barra(0xb74450, texto.as_ptr(), 1, 0x25, -1);
    let mut z = abrir_item(l, 0x9c7b84, 1, a4, tipo());
    if z == 0 {
        caixa_do_save(0x29a3, 0xa81c88, 0xa81c70, 0xa81b5c);
        return 0;
    }
    // general.dat: (endereço, tamanho, quantidade, linha e texto do erro); o resumo e a nação
    // padrão saem de buffers locais; os três últimos gravam vetores e conferem a quantidade
    let mut resumo = [0u8; 0xf74];
    let mut campo = |buf: usize, tam: u32, n: u32, linha: u32, msg: usize| -> bool {
        if gravar(buf, tam, n, z) == n {
            return true;
        }
        caixa_do_save(linha, msg, 0x9c7774, 0x9c7680);
        fechar_item(&mut z);
        false
    };
    let u = |a: usize| *(a as *const u32);
    if !campo(0xdd7110, 4, 1, 0x29aa, 0xa81b1c) {
        return 0;
    }
    montar_resumo(resumo.as_mut_ptr());
    if !campo(resumo.as_ptr() as usize, 0xf74, 1, 0x29b5, 0xa81ae0)
        || !campo(0xdd7120, 4, 1, 0x29bd, 0xa81aa4)
        || !campo(0xdd7114, 1, 1, 0x29c5, 0xa81a6c)
        || !campo(0xdd6ec8, 8, 1, 0x29cd, 0xa81a3c)
        || !campo(0xae2c90, 8, 1, 0x29d5, 0xa81a3c)
        || !campo(0xae2c88, 1, 1, 0x29dd, 0xa81a04)
        || !campo(0xae2c78, 8, 1, 0x29e5, 0xa819d0)
        || !campo(0xae2cfc, 1, 1, 0x29ed, 0xa81994)
        || !campo(0xae2d00, 4, 1, 0x29f5, 0xa8194c)
        || !campo(0xae2d04, 4, 1, 0x29fd, 0xa818fc)
    {
        return 0;
    }
    let padrao = u(0xae2d08);
    let nacao_padrao: u32 = if padrao != 0 { u(padrao as usize) } else { 0xffffffff };
    if !campo(&nacao_padrao as *const u32 as usize, 4, 1, 0x2a06, 0xa818c0)
        || !campo(0xae2d10, 4, 1, 0x2a0e, 0xa81880)
        || !campo(u(0xae2d0c) as usize, 0x4f, u(0xae2d10), 0x2a14, 0xa81848)
        || !campo(u(0xae2d14) as usize, 6, u(0xae2d18), 0x2a1c, 0xa81814)
        || !campo(u(0xae2d1c) as usize, 4, u(0xae235c), 0x2a24, 0xa817dc)
    {
        return 0;
    }
    fechar_item(&mut z);
    for (i, passo) in PASSOS_DO_SAVE.iter().enumerate() {
        let k = i as u32 + 2;
        // antes do passo 0x11 (depois do AWARD_MANAGER) a original não apaga o caminho e pede
        // memória >= 25 (e não >= 1)
        if k == 0x11 {
            if disco(0) < 0x19 {
                *(0xdd7134 as *mut u32) = 1;
                return 0;
            }
            if *(0x9cd1fc as *const u32) != 0 && memoria() < 0x19 {
                *(0xdd7138 as *mut u32) = 1;
                return 0;
            }
        } else if falta_espaco() {
            return 0;
        }
        barra(0xb74450, texto.as_ptr(), k, 0x25, -1);
        match *passo {
            PassoDoSave::Caminho(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(f);
                    f(u(g) as usize, l);
                }
            }
            PassoDoSave::Sozinho(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
                    f(u(g) as usize);
                }
            }
            PassoDoSave::Um(g, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                    f(u(g) as usize, 1);
                }
            }
            PassoDoSave::Virtual(g) => {
                let o = u(g) as usize;
                if o != 0 {
                    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(*((*(o as *const usize) + 0x1c) as *const usize));
                    f(o, 1);
                }
            }
            PassoDoSave::Outro(g, this, f) => {
                if u(g) != 0 {
                    let f: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(f);
                    f(u(this) as usize, l);
                }
            }
            PassoDoSave::Competicoes => {
                if competicoes() == 0 {
                    aviso_do_setup(0x2baa);
                    return 0;
                }
            }
        }
    }
    if falta_espaco() {
        return 0;
    }
    barra(0xb74450, texto.as_ptr(), 0x25, 0x25, -1);
    let (t, linha) = if opcoes() & 0x8000 != 0 { (3, 0x2c82) } else { (4, 0x2c8a) };
    if empacotar(l, temporaria.as_ptr(), 0xa813ac, a4, t) == 0 {
        caixa_do_save(linha, 0xa817a4, 0x9c7774, 0x9c7680);
        return 0;
    }
    hall_da_fama(u(0xb5ded8) as usize);
    regras_fifa(l);
    if opcoes() & 0x20000000 != 0 {
        if existe(caminho) != 0 && apagar(caminho) == -1 {
            caixa_do_save(0x2ca7, 0xa81cc0, 0x9af90c, 0x9af828);
        }
        renomear(l, caminho);
    }
    *(0xdd7130 as *mut u32) = 1;
    *(0xa801bc as *mut u32) = 0;
    let mut data = [0u8; 8];
    copia_data(0xdd70a0, data.as_mut_ptr(), 0xae2c90);
    1
}

// ponytail: grava o save inteiro (apaga o antigo, cria o contêiner, empacota a pasta temporária);
// não roda duas vezes. Só a original; a cópia leva o ramo e é provada pela prova do save.
unsafe extern "C" fn comparar_gravar_jogo(caminho: *const u8, a4: u32) -> u32 {
    orig::<unsafe extern "C" fn(*const u8, u32) -> u32>(575)(caminho, a4)
}
