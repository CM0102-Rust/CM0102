// ---------- %%LEVA%%a leva: a classificação da UEFA: as vagas da Copa da UEFA, o voluntário inglês da Intertoto e a virada do ano (uefa_seeding.cpp) ----------

unsafe fn cqi3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqi3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqi3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes; +0 id da nação, +4 a nação, +0x11
/// clubes da nação nas copas).
fn cqi3_e(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// O clube `c` (com nação e ainda sem vaga) entra na Copa da UEFA na fase `fase`; um clube a mais da nação `r`. Devolve
/// se entrou. (O caso da nação −1, que a original procura pela nação do clube, não acontece: `r` vem da volta.)
unsafe fn enter_uefa_cup(this: usize, c: usize, fase: u8, r: i32, colocados: &mut i32) -> bool {
    if cqi3_d(c + 0x53) == 0 || cqi3_d(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = cqi3_d(0x9cf6f0);
    *((c + 0x1df) as *mut u8) = fase;
    *colocados += 1;
    let p = cqi3_e(this, r) + 0x11;
    *(p as *mut u8) = cqi3_b(p).wrapping_add(1);
    true
}

/// { 0x904580 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_uefa_cup_places`): as vagas da Copa da UEFA
/// ([0x9cf6f0]). O campeão da última edição (0x4afd80 com a linha da competição e o lugar 0) na fase 1, e a nação dele
/// com uma vaga a menos na fase 1. Três nações sorteadas (0x90cfc0 com o número de nações; a 2ª e a 3ª de novo até as
/// três serem distintas) ganham uma vaga a mais na preliminar (o fair play). Por posição da nação no coeficiente, as
/// vagas na fase 1 e na preliminar (fase 2): 1ª a 6ª 3 e 0, 7ª e 8ª 4 e 0, 9ª a 15ª 1 e 1, 16ª e 17ª 1 e 2, 18ª a 21ª 0
/// e 3, 22ª a 48ª 0 e 2, da 49ª em diante 0 e 1. Os clubes de cada nação, na ordem: 1º o campeão da copa nacional (a
/// copa da nação em 0xb63d7e + linha × 0x48, linha = 0x669340), ou o vice se o campeão já tem vaga (a Bélgica e o vice
/// sem nação: o seguinte da tabela), e, com os dois já com vaga, a final guardada em +0x440 (50 registros: copa,
/// campeão, vice) quando a copa da temporada ainda não foi disputada (o ano +0x40 da copa ≠ o da nação em 0xb63d7b;
/// fora Espanha e Bélgica); 2º o campeão da copa da liga (0xb63d82), se ainda sem vaga; os outros, o primeiro sem vaga da
/// tabela (0x9050d0(.., 1, 1, 0)); sem copa nacional na linha, a caixa da linha 0x418 e a vaga é tentada de novo.
/// Com a final guardada e os dois finalistas: uma vaga a mais para quem dos dois ainda não tem (os dois sem vaga entram
/// na fase 10 × a conta + 3; os dois com vaga, o primeiro sem vaga da tabela). Para quando os colocados menos a conta
/// passam de 116.
unsafe extern "thiscall" fn assign_uefa_cup_places(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x9050d0usize);
    let linha: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(0x669340usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n8 = || *((this + 0x438) as *const i8);
    let uc = cqi3_d(0x9cf6f0);
    let mut conta_extra = 1i8;
    let mut colocados = 0i32;
    let mut nacao_do_campeao = 0usize;
    let h = campeao((cqi3_d(0xae23d0) as usize).wrapping_add(uc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && cqi3_d(h + 0x53) != 0 && cqi3_d(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = uc;
        colocados = 1;
        let id = cqi3_d(cqi3_d(h + 0x53) as usize);
        let mut i = -1i32;
        for k in 0..n8() as i32 {
            if cqi3_d(cqi3_e(this, k)) == id {
                i = k;
                break;
            }
        }
        let p = cqi3_e(this, i) + 0x11;
        *(p as *mut u8) = cqi3_b(p).wrapping_add(1);
        nacao_do_campeao = cqi3_d(h + 0x53) as usize;
    }
    let s1 = sorteio(n8() as i32) as u8 as i8;
    let mut s2 = sorteio(n8() as i32) as u8 as i8;
    let mut s3 = sorteio(n8() as i32) as u8 as i8;
    while s1 == s2 || s1 == s3 || s2 == s3 {
        s2 = sorteio(n8() as i32) as u8 as i8;
        s3 = sorteio(n8() as i32) as u8 as i8;
    }
    let mut ultima_fase = 0u8;
    let mut r = 0i8;
    while r < n8() {
        let (f1, f2): (u8, u8) = if r < 6 {
            crate::comparison::branch("cqi_3 tres na primeira fase");
            (3, 0) // cqi3: 1ª a 6ª nação
        } else if r < 8 {
            (4, 0)
        } else if r < 0xf {
            (1, 1)
        } else if r < 0x11 {
            (1, 2)
        } else if r < 0x15 {
            (0, 3)
        } else if r < 0x30 {
            (0, 2)
        } else {
            (0, 1)
        };
        let mut vagas = [0u8, f1, f2];
        let e = cqi3_e(this, r as i32);
        let nacao = cqi3_d(e + 4) as usize;
        if nacao == nacao_do_campeao {
            vagas[1] = vagas[1].wrapping_sub(1);
        }
        if r == s1 || r == s2 || r == s3 {
            crate::comparison::branch("cqi_3 fair play");
            vagas[2] = vagas[2].wrapping_add(1); // cqi3: a vaga do fair play
        }
        let um = || enesimo(this, clubes, conta, cqi3_d(e), 1, 1, 0);
        let mut escolhidos = 0u8;
        let (mut vencedor, mut vice) = (0usize, 0usize);
        let mut f = 1u8;
        while f < 3 {
            let lim = vagas[f as usize] as i8;
            if lim > 0 {
                ultima_fase = f;
                let mut j = 0i8;
                loop {
                    let mut tentar_de_novo = false;
                    let c = if escolhidos == 0 {
                        if cqi3_b(nacao + 0x11c) == 0 {
                            um()
                        } else {
                            let copa = cqi3_d(0xb63d7e + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                show_error_at(0xaa1a1c, 0x418);
                                tentar_de_novo = true;
                                0
                            } else {
                                let mut c = campeao(copa, 0);
                                if c == 0 || cqi3_d(c + 0x53) == 0 {
                                    c = um();
                                }
                                if cqi3_d(c + 0x1db) == u32::MAX {
                                    c
                                } else {
                                    let mut c = campeao(copa, 1);
                                    if c == 0 || cqi3_d(c + 0x53) == 0 || cqi3_d(nacao) == cqi3_d(0x9cf244) {
                                        c = um();
                                    }
                                    if cqi3_d(c + 0x1db) == u32::MAX {
                                        c
                                    } else if cqi3_d(0xadadfc) == 0 {
                                        um()
                                    } else {
                                        let id = cqi3_d(copa);
                                        let ano = core::ptr::read_unaligned((0xb63d7b + (linha(nacao) as i32 * 0x48) as usize) as *const u16) as i32;
                                        let comp = cqi3_d((cqi3_d(0xadadfc) as usize).wrapping_add(id as usize * 4)) as usize;
                                        if cqi3_w(comp + 0x40) as i32 == ano || cqi3_d(nacao) == cqi3_d(0x9cf488) || cqi3_d(nacao) == cqi3_d(0x9cf244) {
                                            um()
                                        } else {
                                            let (mut v1, mut v2) = (-1i32, -1i32);
                                            for k in 0..0x32usize {
                                                if cqi3_d(this + 0x440 + k * 12) == id {
                                                    v1 = cqi3_d(this + 0x444 + k * 12) as i32;
                                                    v2 = cqi3_d(this + 0x448 + k * 12) as i32;
                                                    break;
                                                }
                                            }
                                            let base = cqi3_d(0xae23b4) as usize;
                                            vencedor = if v1 != -1 { base.wrapping_add((v1 as usize).wrapping_mul(0x245)) } else { 0 };
                                            vice = if v2 != -1 { base.wrapping_add((v2 as usize).wrapping_mul(0x245)) } else { 0 };
                                            if vencedor == 0 {
                                                um()
                                            } else if vice != 0 {
                                                0
                                            } else {
                                                c
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if escolhidos == 1 {
                        if cqi3_b(nacao + 0x11c) == 0 {
                            um()
                        } else {
                            let copa = cqi3_d(0xb63d82 + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                um()
                            } else {
                                let c = campeao(copa, 0);
                                if c == 0 || cqi3_d(c + 0x53) == 0 || cqi3_d(c + 0x1db) != u32::MAX {
                                    um()
                                } else {
                                    c
                                }
                            }
                        }
                    } else {
                        um()
                    };
                    if tentar_de_novo {
                        j -= 1;
                    } else {
                        escolhidos = escolhidos.wrapping_add(1);
                        if c != 0 {
                            enter_uefa_cup(this, c, f, r as i32, &mut colocados);
                            if colocados as i16 as i32 - conta_extra as i32 + 1 >= 0x76 {
                                return;
                            }
                        }
                    }
                    j += 1;
                    if j >= lim {
                        break;
                    }
                }
            }
            f += 1;
        }
        if vencedor != 0 && vice != 0 {
            conta_extra += 1;
            let (a, b) = (cqi3_d(vencedor + 0x1db) == u32::MAX, cqi3_d(vice + 0x1db) == u32::MAX);
            if !a && !b {
                let c = um();
                if c != 0 {
                    enter_uefa_cup(this, c, ultima_fase, r as i32, &mut colocados);
                }
            } else if a && b {
                let fase = (conta_extra as u8).wrapping_mul(10).wrapping_add(3);
                ultima_fase = fase;
                if cqi3_d(vencedor + 0x53) != 0 {
                    enter_uefa_cup(this, vencedor, fase, r as i32, &mut colocados);
                }
                enter_uefa_cup(this, vice, fase, r as i32, &mut colocados);
            } else {
                enter_uefa_cup(this, if a { vencedor } else { vice }, ultima_fase, r as i32, &mut colocados);
            }
        }
        r += 1;
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqi3_dif(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Lado a lado: a classificação (com a tabela das finais em +0x440), os clubes, o sorteio (a cópia sem contar) e a marca
/// da caixa.
unsafe extern "thiscall" fn compare_assign_uefa_cup_places(this: usize, clubes: usize, conta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let n = (cqi3_d(0xae235c) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let mut reg = vec![(this, 0x698), (cqi3_d(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (cqi3_d(0xae23d4) as usize, (cqi3_d(0xae237c) as i32).max(0) as usize * 0x1a)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubes, conta), || crate::rng::without_counting(|| assign_uefa_cup_places(this, clubes, conta)));
    SCOREBOARD.record(igual, || format!("assign_uefa_cup_places: diferenças em {:x?}", cqi3_dif(&mo, &mr)));
}

/// { 0x903890 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::pick_english_intertoto_club`; quadro SEH pela notícia
/// local, sem destrutor no caminho normal): o clube inglês da Intertoto. Em 2001, ou sem a Premier ([0xadadfc] +
/// [0x9cf5bc] × 4), devolve 0. Senão completa a lista dos voluntários (+0x43b, 30 ids, alocada com 0x78 bytes; +0x43f a
/// conta) até 6, sorteando clubes da Premier (0x90cfc0 com +0x3e, até 80 sorteios) sem técnico humano e sem vaga
/// europeia; escolhe o voluntário sem vaga da divisão mais alta (a da Premier +4 conta o +0x5f; as outras +30); sem
/// nenhum, sorteia clubes da Premier até um sem vaga. Entrega a notícia 0x17 com o clube (0x77bbe0, 0x785790, 0x7862d0
/// à divisão da Premier) e devolve o clube.
unsafe extern "thiscall" fn pick_english_intertoto_club(this: usize) -> usize {
    if cqi3_w(0xae2c92) == 0x7d1 {
        return 0;
    }
    let tabela = cqi3_d(0xadadfc) as usize;
    if tabela == 0 {
        return 0;
    }
    let liga = cqi3_d(tabela.wrapping_add(cqi3_d(0x9cf5bc) as usize * 4)) as usize;
    if liga == 0 {
        return 0;
    }
    crate::comparison::branch("cqi_3 voluntario ingles");
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let clube_sorteado = || {
        let s = sorteio(cqi3_w(liga + 0x3e) as i32) as i16 as i32;
        cqi3_d((cqi3_d(liga + 0xb1) as usize).wrapping_add((s * 0x3b) as usize)) as usize
    };
    let mut tentativas = 0i8;
    while (cqi3_b(this + 0x43f) as i8) < 6 && tentativas < 0x50 {
        tentativas += 1;
        let c = clube_sorteado();
        if humano(c, 0, 1, 0, 0) == 0 && cqi3_d(c + 0x1db) == u32::MAX {
            if cqi3_d(this + 0x43b) == 0 {
                *((this + 0x43f) as *mut u8) = 0;
                let l = alocar(0x78);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, l as u32);
                if l == 0 {
                    show_error_at(0xaa1a1c, 0xf9);
                    continue;
                }
            }
            let k = cqi3_b(this + 0x43f) as i8;
            if k < 0x1e {
                core::ptr::write_unaligned((cqi3_d(this + 0x43b) as usize + k as usize * 4) as *mut u32, cqi3_d(c));
                *((this + 0x43f) as *mut u8) = (k as u8).wrapping_add(1);
            }
        }
    }
    let mut melhor = 0usize;
    let mut nivel = 0x64i8;
    let k = cqi3_b(this + 0x43f) as i8 as i32;
    if k > 0 {
        let lista = cqi3_d(this + 0x43b) as usize;
        let base = cqi3_d(0xae23b4) as usize;
        for q in 0..k as usize {
            let c = base.wrapping_add((cqi3_d(lista + q * 4) as usize).wrapping_mul(0x245));
            if cqi3_d(c + 0x1db) == u32::MAX {
                let v = cqi3_b(c + 0x5f) as i8;
                if cqi3_d(c + 0x5b) == cqi3_d(liga + 4) {
                    if v < nivel {
                        melhor = c;
                        nivel = v;
                    }
                } else if v as i32 + 0x1e < nivel as i32 {
                    melhor = c;
                    nivel = v.wrapping_add(0x1e);
                }
            }
        }
    }
    while melhor == 0 {
        let c = clube_sorteado();
        if cqi3_d(c + 0x1db) == u32::MAX {
            melhor = c;
        }
    }
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x17, 0);
    gravar(n, 0, cqi3_d(melhor));
    a_divisao(cqi3_d(0xb7431c) as usize, n, cqi3_d(liga + 4) as usize);
    melhor
}

/// Em 2001 (devolve 0 sem efeito): a devolução das duas. Fora de 2001 só a original (sorteia, aloca a lista e entrega a
/// notícia).
unsafe extern "thiscall" fn compare_pick_english_intertoto_club(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    if cqi3_w(0xae2c92) != 0x7d1 {
        return o(this);
    }
    let r = o(this);
    let c = pick_english_intertoto_club(this);
    SCOREBOARD.record(r == c, || format!("pick_english_intertoto_club: original {:x} rust {:x}", r, c));
    r
}

/// { 0x9035a0 } (thiscall, ret 4, uefa_seeding.cpp, `UefaSeeding::update_season`; quadro SEH pela notícia local, sem
/// destrutor no caminho normal): chamada pelas copas europeias. Sem `diario`: quando o ano do jogo ([0xae2c92]) é outro
/// que o +0x439, grava o ano e refaz a classificação: os coeficientes (0x903b50), as vagas por nação (0x903cc0), a tabela
/// dos clubes (0x904040; vazia, a caixa da linha 0x2b7), as vagas da Liga dos Campeões (0x904210), da Copa da UEFA
/// (0x904580) e da Intertoto (0x904e30), solta a tabela e entrega a notícia dos classificados (0x905f00). Com `diario`:
/// no dia 100 do ano, com a Premier e a Intertoto no jogo, a notícia 0x16 do pedido de voluntários (com o ano) a cada
/// técnico humano de clube da Premier (sem a Premier, volta sem mais nada); e o prazo das inscrições europeias (0x9058e0).
unsafe extern "thiscall" fn update_season(this: usize, diario: u32) {
    if diario == 0 {
        let ano = cqi3_w(0xae2c92);
        if ano == cqi3_w(this + 0x439) {
            return;
        }
        crate::comparison::branch("cqi_3 virada da classificacao");
        core::ptr::write_unaligned((this + 0x439) as *mut i16, ano);
        let coeficientes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x903b50usize);
        let vagas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x903cc0usize);
        let juntar: unsafe extern "thiscall" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x904040usize);
        let campeoes: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x904210usize);
        let copa_uefa: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x904580usize);
        let intertoto: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x904e30usize);
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x905f00usize);
        coeficientes(this);
        vagas(this);
        let mut t = 0usize;
        let n = juntar(this, &mut t);
        if n as u16 == 0 {
            show_error_at(0xaa1a1c, 0x2b7);
        } else {
            campeoes(this, t, n);
            copa_uefa(this, t, n);
            intertoto(this, t, n); // cqi3: as vagas da Intertoto
            soltar(t);
        }
        classificados(this);
        return;
    }
    if *(0xae2c90 as *const i16) == 100 {
        let tabela = cqi3_d(0xadadfc) as usize;
        let liga = cqi3_d(tabela.wrapping_add(cqi3_d(0x9cf5bc) as usize * 4)) as usize;
        if liga == 0 {
            return;
        }
        if cqi3_d(tabela.wrapping_add(cqi3_d(0x9cf6f8) as usize * 4)) != 0 {
            let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
            let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
            let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
            let mut noticia = [0u8; 0xe8];
            let p = noticia.as_mut_ptr() as usize;
            construir(p, 0x16, 0);
            gravar(p, 0, 0);
            gravar(p, 2, cqi3_w(this + 0x439) as u16 as u32);
            let mut i = 0i16;
            while i < cqi3_w(liga + 0x3e) {
                let c = cqi3_d((cqi3_d(liga + 0xb1) as usize).wrapping_add(i as i32 as usize * 0x3b)) as usize;
                if humano(cqi3_d(0xb7431c) as usize, c) != 0 {
                    gravar(p, 1, cqi3_d(c));
                    entregar(cqi3_d(0xb7431c) as usize, p, c);
                }
                i += 1;
            }
        }
    }
    let prazos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x9058e0usize);
    prazos(this);
}

/// Só a original (refaz as vagas pelas rotinas de dentro, solta a tabela, entrega notícias); a cópia leva o contador.
unsafe extern "thiscall" fn compare_update_season(this: usize, diario: u32) {
    // ponytail: refaz a classificação inteira e entrega notícias: não dá para rodar duas vezes
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%)(this, diario)
}
