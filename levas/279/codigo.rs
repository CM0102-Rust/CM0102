// ---------- 279a leva: a escolha do estádio de um jogo: o estádio da partida e a busca do estádio neutro (stadium.cpp) ----------

unsafe fn cqo1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A nação `id` (registro de 0x122 bytes em [0xae23a8]).
unsafe fn cqo1_nacao(id: usize) -> usize {
    cqo1_d(0xae23a8).wrapping_add(id.wrapping_mul(0x122))
}

/// O clube é uma seleção? (0x53b900, cdecl, já em Rust.)
unsafe fn cqo1_selecao(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(clube) != 0
}

/// `Stadium::add_booking` (0x85de20, thiscall, ret 8, já em Rust): a reserva do estádio na data.
unsafe fn cqo1_reservar(this: usize, estadio: usize, data: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85de20usize);
    f(this, estadio, data)
}

/// `Stadium::is_stadium_free_on_date` (0x85ddb0, thiscall, ret 8, já em Rust).
unsafe fn cqo1_livre(this: usize, estadio: usize, data: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85ddb0usize);
    f(this, estadio, data) != 0
}

/// Os comparadores das listas (cdecl, (a, b) -> int): 0x85e2f0 pela nação (+0), 0x85e320 pelo continente (+4), os
/// dois desempatando pelos lugares do estádio (+0xc → +0x3c, quando os dois têm estádio); 0x85e350 só pelos lugares.
unsafe fn cqo1_cmp(f: usize, a: usize, b: usize) -> i32 {
    let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(f);
    f(a, b)
}

/// { 0x85c800 } (thiscall, ret 0x1c, stadium.cpp, `Stadium::find_available_venue(marcas, continente, nação, data, casa,
/// fora, partida)`): a busca do estádio neutro na lista dos estádios (this +0xc, conta +0x10; vagas de 16 bytes: +0 a
/// nação, +4 o continente, +8 1 se é estádio de clube [inferência], +0xc o estádio), ordenada (qsort 0x945333) pela
/// nação (com nação; marca +0x18 = 1), pelo continente (com continente; 2) ou só pelos lugares (3). Com nação ou
/// continente, a busca binária (0x946464) acha a primeira vaga da chave; sem nenhum, começa da primeira. Olha até 30
/// vagas (a última da lista nunca), pulando o estádio guardado ([0xa835a4]) e, em jogo de clubes com marcas < 0x40,
/// os estádios dos dois clubes; para na primeira vaga de outra chave. Marcas sem 0x40/0x80/0x7f00 (o neutro comum):
/// com 4 só vagas de clube, com 2 só as outras; entre as livres na data (0x85ddb0), a de maior nota (0x85da00), que
/// é reservada (0x85de20, com partida). Com 0x40..0x4000 (o rodízio das sedes de um torneio): a vaga que não é de
/// clube cuja posição (contada) módulo 9 é a da marca (0x40 → 0, 0x80 → 1, 0x100 → 2 ... 0x4000 → 8), livre na data
/// (ou na competição [0x9cf76c] sem conferir) é reservada e devolvida. Sem nada: com nação, marcas sem 2 e com
/// 4/8/0x20, o estádio nacional dela (+0x80); senão 0.
unsafe extern "thiscall" fn estadio_neutro(this: usize, marcas: u32, cont: usize, nacao: usize, data: usize, casa: usize, fora: usize, partida: usize) -> usize {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let estado = (this + 0x18) as *mut u8;
    // a chave: só o campo da ordem e o +0xc (0) contam para os comparadores
    let mut chave = [0u32; 4];
    let mut cmp = 0usize;
    let mut e;
    if nacao != 0 {
        if *estado != 1 {
            ordenar(cqo1_d(this + 0xc), cqo1_d(this + 0x10) as u32, 0x10, 0x85e2f0);
            *estado = 1;
        }
        chave[0] = cqo1_d(nacao) as u32;
        cmp = 0x85e2f0;
    } else if cont != 0 {
        if *estado != 2 {
            ordenar(cqo1_d(this + 0xc), cqo1_d(this + 0x10) as u32, 0x10, 0x85e320);
            *estado = 2;
        }
        chave[1] = cqo1_d(cont) as u32;
        cmp = 0x85e320;
    } else {
        if *estado != 3 {
            ordenar(cqo1_d(this + 0xc), cqo1_d(this + 0x10) as u32, 0x10, 0x85e350);
            *estado = 3;
        }
    }
    let k = chave.as_ptr() as usize;
    if cmp != 0 {
        e = buscar(k, cqo1_d(this + 0xc), cqo1_d(this + 0x10) as u32, 0x10, cmp);
        if e != 0 {
            // volta até a primeira vaga da mesma chave
            loop {
                if cqo1_d(this + 0xc) >= e {
                    break;
                }
                e -= 0x10;
                if cqo1_cmp(cmp, e, k) != 0 {
                    e += 0x10;
                    break;
                }
                if e == 0 {
                    break;
                }
            }
        }
    } else {
        e = cqo1_d(this + 0xc);
    }
    if e != 0 {
        let mut i: i16 = 0;
        let mut melhor = 0i32;
        let mut escolhida = 0usize;
        loop {
            if i >= 0x1e {
                break;
            }
            let ultima = (cqo1_d(this + 0x10) as u32).wrapping_add(0x0fff_ffff).wrapping_shl(4).wrapping_add(cqo1_d(this + 0xc) as u32);
            if e as u32 == ultima {
                break;
            }
            let mut contar = true;
            'vaga: {
                if cqo1_d(cqo1_d(e + 0xc)) == cqo1_d(0xa835a4) {
                    contar = false;
                    break 'vaga;
                }
                if casa != 0 && fora != 0 && !cqo1_selecao(casa) && (marcas as i32) < 0x40 {
                    let s = cqo1_d(e + 0xc);
                    if s == cqo1_d(casa + 0x69) || s == cqo1_d(fora + 0x69) {
                        break 'vaga;
                    }
                }
                if (nacao != 0 || cont != 0) && cqo1_cmp(cmp, e, k) != 0 {
                    // outra chave: fim da busca
                    i = 0x1e;
                    contar = false;
                    break 'vaga;
                }
                if marcas & 0x40 == 0 && marcas & 0x80 == 0 && marcas & 0x7f00 == 0 {
                    if marcas & 4 != 0 && cqo1_d(e + 8) == 0 {
                        contar = false;
                        break 'vaga;
                    }
                    if marcas & 2 != 0 && cqo1_d(e + 8) != 0 {
                        contar = false;
                        break 'vaga;
                    }
                    if !cqo1_livre(this, cqo1_d(e + 0xc), data) {
                        break 'vaga;
                    }
                    let nota: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32 = core::mem::transmute(0x85da00usize);
                    let n = nota(this, cqo1_d(e + 0xc), marcas, casa, fora, partida);
                    if n > melhor {
                        melhor = n; // cqo1: a melhor nota
                        escolhida = e;
                    }
                } else {
                    if cqo1_d(e + 8) != 0 {
                        contar = false;
                        break 'vaga;
                    }
                    let p = i as i32 % 9;
                    let vez = (marcas & 0x40 != 0 && p == 0)
                        || (marcas & 0x80 != 0 && p == 1)
                        || (marcas & 0x100 != 0 && p == 2)
                        || (marcas & 0x200 != 0 && p == 3)
                        || (marcas & 0x400 != 0 && p == 4)
                        || (marcas & 0x800 != 0 && p == 5)
                        || (marcas & 0x1000 != 0 && p == 6)
                        || (marcas & 0x2000 != 0 && p == 7)
                        || (marcas & 0x4000 != 0 && p == 8);
                    if vez && ((partida != 0 && cqo1_d(partida) == cqo1_d(0x9cf76c)) || cqo1_livre(this, cqo1_d(e + 0xc), data)) {
                        crate::comparacao::ramo("cqo_1 rodizio");
                        cqo1_reservar(this, cqo1_d(e + 0xc), data);
                        return cqo1_d(e + 0xc);
                    }
                }
            }
            if i >= 0x1e {
                break;
            }
            if contar {
                i += 1;
            }
            e = e.wrapping_add(0x10);
            if e == 0 {
                break;
            }
        }
        if melhor > 0 {
            crate::comparacao::ramo("cqo_1 neutro pela nota");
            if partida != 0 {
                cqo1_reservar(this, cqo1_d(escolhida + 0xc), data);
            }
            return cqo1_d(escolhida + 0xc);
        }
    }
    if nacao != 0 && marcas & 2 == 0 && marcas & 0x2c != 0 {
        return cqo1_d(nacao + 0x80);
    }
    0
}

/// As regiões da escolha do estádio: o estado do sorteador, o cabeçalho (0x1c bytes), a lista dos estádios (a ordem muda) e as reservas com
/// a vaga da nova. `None` quando uma reserva pode alocar (sem lista ou cheia): então só a original.
unsafe fn cqo1_regioes(this: usize, extra: Option<(usize, usize)>) -> Option<([(usize, usize); 6], usize)> {
    let nr = cqo1_d(this + 4);
    if cqo1_d(this) == 0 || nr == cqo1_d(this + 8) {
        return None;
    }
    let mut r = [(0usize, 0usize); 6];
    let mut m = 0;
    // a nota do estádio neutro (0x85da00) sorteia: o estado do sorteador entra (e a cópia roda sem contar)
    for x in crate::aleatorio::REGIOES_DO_ESTADO {
        r[m] = x;
        m += 1;
    }
    r[m] = (this, 0x1c);
    m += 1;
    r[m] = (cqo1_d(this), (nr + 1) * 12);
    m += 1;
    let nv = cqo1_d(this + 0x10);
    if cqo1_d(this + 0xc) != 0 && nv > 0 && nv < 0x10000 {
        r[m] = (cqo1_d(this + 0xc), nv * 16);
        m += 1;
    }
    if let Some(x) = extra {
        r[m] = x;
        m += 1;
    }
    Some((r, m))
}

/// Lado a lado (a cópia sem contar os sorteios) pelas regiões de `cqo1_regioes` e pela devolução; com reserva que pode alocar, só a original.
unsafe extern "thiscall" fn comparar_estadio_neutro(this: usize, marcas: u32, cont: usize, nacao: usize, data: usize, casa: usize, fora: usize, partida: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, usize, usize, usize) -> usize>(1888);
    let Some((r, m)) = cqo1_regioes(this, None) else {
        // ponytail: a reserva pode alocar a lista; só a original
        return o(this, marcas, cont, nacao, data, casa, fora, partida);
    };
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&r[..m], || o(this, marcas, cont, nacao, data, casa, fora, partida), || {
        crate::aleatorio::sem_contar(|| estadio_neutro(this, marcas, cont, nacao, data, casa, fora, partida))
    });
    PLACAR.registrar(igual && ro == rr, || format!("estadio_neutro(marcas {:x}, nacao {:x}, cont {:x}): original {:x} rust {:x} {:x?} x {:x?}", marcas, nacao, cont, ro, rr, mo, mr));
    ro
}

/// { 0x85c260 } (thiscall, ret 0xc, stadium.cpp, `Stadium::set_fixture_venue(marcas, partida, estadio)`; this =
/// [0xdd7ef4]): o estádio da partida (+0x18 o ponteiro, +4 o id, -1 sem; +0x3c 1 em campo neutro). Partida nula: a
/// caixa (linha 0x1f0). A data é a da partida (+0x2a o dia, +0x28 o ano). As regras, pelas marcas:
/// - 0x8000: o estádio do Monaco ([0x9d0264]), se nenhum dos dois é o Monaco; senão como 4;
/// - 4: na competição inglesa (nação da competição +0x5d = [0x9cf2e4]) de clubes, com o estádio guardado
///   ([0xa835a4] != -1), o estádio nacional do País de Gales ([0x9cf510]: o Millennium, enquanto Wembley está em obras);
///   senão como as outras;
/// - 0 (o jogo comum): seleção da Inglaterra antes de 2003 vai a campo neutro (como 0x20); na Liga dos Campeões
///   ([0x9cf6e8]) com time da Finlândia ([0x9cf2fc]) fora da fase 0xff, se o estádio nacional da Finlândia está livre
///   na data (sem a reserva, 0x946464 com 0x85e380) e o estádio do clube tem menos de 40 mil lugares, joga lá
///   (reservado); senão o estádio do mandante (+0x69), salvo clube da Itália ou da França em amistoso (0x5c6710,
///   vai a campo neutro como 0x20); sem estádio: seleção como 0x20, clube nenhum;
/// - 0x10: o estádio `estadio` (tabela [0xae23b0], 0x4e bytes), sem ele nenhum; 1: nenhum;
/// - as outras (campo neutro): as sedes da competição no ano +0x2c (0x5fa730): com duas, a primeira e a segunda
///   trocam de papel conforme as marcas de rodízio (0x40/0x100/0x400/0x1000/0x4000 → 4/0x40/0x80/0x100/0x200,
///   alternativa a segunda; 0x80/0x200/0x800/0x2000 → 0x40/0x80/0x100/0x200, alternativa a primeira); com uma, ela;
///   sem nenhuma, a nação da competição (+0x5d). Com 0x20, a seleção joga no próprio país. Com 4, o estádio nacional
///   da nação (reservado); sem ele, 0x20 (seleção) ou 0x40. Depois a busca do estádio neutro (0x85c800) na nação e, sem
///   achar, na alternativa (marcas 0x40..0x200 → 0x400..0x2000).
unsafe extern "thiscall" fn estadio_da_partida(this: usize, marcas: u32, partida: usize, estadio: u32) {
    if partida == 0 {
        return aviso_em(0xa835b0, 0x1f0);
    }
    let fx = partida;
    cqo1_pd(fx + 0x18, 0);
    cqo1_pd(fx + 4, u32::MAX as usize);
    let ano = *((fx + 0x28) as *const u16);
    let d = crate::data::Data { dia: *((fx + 0x2a) as *const i16), ano, bissexto: crate::data::bissexto(ano) };
    let pd = &d as *const crate::data::Data as usize;
    let fixar = |st: usize| {
        cqo1_pd(fx + 0x18, st);
        cqo1_pd(fx + 4, cqo1_d(st));
    };
    let neutro = (fx + 0x3c) as *mut u8;
    let mut f = marcas;
    // 0 = as regras gerais (0x85c5a7); 1 = a regra inglesa de 4 (0x85c38e)
    let mut passo = 0;
    if f == 0x8000 {
        let monaco = cqo1_d(0x9d0264);
        if cqo1_d(cqo1_d(fx + 0x1c)) != monaco && cqo1_d(cqo1_d(fx + 0x20)) != monaco {
            fixar(cqo1_d(cqo1_d(0xae23b4) + monaco * 0x245 + 0x69));
            return;
        }
        f = 4;
        passo = 1;
    } else if f == 4 {
        passo = 1;
    } else if f == 0 {
        let casa = cqo1_d(fx + 0x1c);
        if d.ano < 0x7d3 && cqo1_selecao(casa) && cqo1_d(cqo1_d(casa + 0x53)) == cqo1_d(0x9cf2e4) {
            f = 0x20;
        }
        let casa = cqo1_d(fx + 0x1c);
        let n = cqo1_d(casa + 0x53);
        let fin = cqo1_d(0x9cf2fc);
        if n != 0 && cqo1_d(n) == fin && *((fx + 0x42) as *const u8) != 0xff && cqo1_d(fx) == cqo1_d(0x9cf6e8) && cqo1_d(casa + 0x69) != 0 {
            let st = cqo1_d(cqo1_nacao(fin) + 0x80);
            let mut livre = true;
            if cqo1_d(this) != 0 {
                let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
                let mut chave = [0u32; 3];
                chave[0] = cqo1_d(st) as u32;
                chave[1] = (d.dia as u16 as u32) | ((d.ano as u32) << 16);
                chave[2] = d.bissexto as u32;
                livre = buscar(chave.as_ptr() as usize, cqo1_d(this), cqo1_d(this + 4) as u32, 0xc, 0x85e380) == 0;
            }
            if livre && (cqo1_d(cqo1_d(cqo1_d(fx + 0x1c) + 0x69) + 0x3c) as i32) < 40000 {
                *neutro = 1;
                fixar(cqo1_d(cqo1_nacao(fin) + 0x80));
                cqo1_reservar(this, cqo1_d(fx + 0x18), pd);
                return;
            }
        }
        *neutro = 0;
        let casa = cqo1_d(fx + 0x1c);
        if f != 0x20 && casa != 0 && cqo1_d(casa + 0x69) != 0 {
            let n = cqo1_d(casa + 0x53);
            let mut amistoso = false;
            if n != 0 && (cqo1_d(n) == cqo1_d(0x9cf360) || cqo1_d(n) == cqo1_d(0x9cf300)) {
                let e: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
                amistoso = e(cqo1_d(0xae2c74), cqo1_d(fx + 0x14)) != 0;
            }
            if !amistoso {
                crate::comparacao::ramo("cqo_1 estadio do mandante");
                fixar(cqo1_d(cqo1_d(fx + 0x1c) + 0x69)); // cqo1: o estádio do mandante
                return;
            }
        } else if !cqo1_selecao(cqo1_d(fx + 0x1c)) {
            cqo1_pd(fx + 0x18, 0);
            cqo1_pd(fx + 4, u32::MAX as usize);
            return;
        }
        f = 0x20;
    }
    if passo == 1 {
        let comp = cqo1_d(fx + 0x14);
        if comp != 0 {
            let n = cqo1_d(comp + 0x5d);
            if n != 0 && cqo1_d(n) == cqo1_d(0x9cf2e4) && !cqo1_selecao(cqo1_d(fx + 0x1c)) && cqo1_d(0xa835a4) as i32 != -1 {
                *neutro = 1;
                fixar(cqo1_d(cqo1_nacao(cqo1_d(0x9cf510)) + 0x80));
                return;
            }
        }
    }
    // as regras gerais
    *neutro = 1;
    if f & 0x10 != 0 {
        if estadio as i32 == -1 {
            cqo1_pd(fx + 0x18, 0);
            cqo1_pd(fx + 4, u32::MAX as usize);
            return;
        }
        cqo1_pd(fx + 4, estadio as usize);
        cqo1_pd(fx + 0x18, cqo1_d(0xae23b0).wrapping_add((estadio as usize).wrapping_mul(0x4e)));
        return;
    }
    if f == 1 {
        cqo1_pd(fx + 0x18, 0);
        cqo1_pd(fx + 4, u32::MAX as usize);
        return;
    }
    let comp = cqo1_d(fx + 0x14);
    let sede: unsafe extern "thiscall" fn(usize, u32, u32, *mut u32, *mut u32, u32) -> u32 = core::mem::transmute(0x5fa730usize);
    let (mut a, mut b) = (0u32, 0u32);
    // ponytail: a original empurra o eax com o ano (word +0x2c) nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let r = sede(cqo1_d(0xb5e134), cqo1_d(comp) as u32, *((fx + 0x2c) as *const u16) as u32, &mut a, &mut b, 1) as u8;
    let mut alvo;
    let mut alt = 0usize;
    if r == 2 {
        let (na, nb) = (cqo1_nacao(a as usize), cqo1_nacao(b as usize));
        alvo = nb;
        if f & 0x80 == 0 && f & 0x2a00 == 0 {
            if f & 0x40 != 0 || f & 0x5500 != 0 {
                if f & 0x40 != 0 {
                    f = 4;
                } else if f & 0x100 != 0 {
                    f = 0x40;
                } else if f & 0x400 != 0 {
                    f = 0x80;
                } else if f & 0x1000 != 0 {
                    f = 0x100;
                } else if f & 0x4000 != 0 {
                    f = 0x200;
                }
                alt = nb;
                alvo = na;
            }
        } else {
            if f & 0x80 != 0 {
                f = 0x40;
            } else if f & 0x200 != 0 {
                f = 0x80;
            } else if f & 0x800 != 0 {
                f = 0x100;
            } else if f & 0x2000 != 0 {
                f = 0x200;
            }
            alt = na;
        }
    } else if r == 1 {
        crate::comparacao::ramo("cqo_1 uma sede");
        alvo = cqo1_nacao(a as usize); // cqo1: a sede única
    } else {
        alvo = cqo1_d(comp + 0x5d);
    }
    if f & 0x20 != 0 && cqo1_selecao(cqo1_d(fx + 0x1c)) {
        alvo = cqo1_d(cqo1_d(fx + 0x1c) + 0x53);
    }
    if f & 4 != 0 && alvo != 0 {
        let st = cqo1_d(alvo + 0x80);
        if st != 0 {
            crate::comparacao::ramo("cqo_1 estadio nacional");
            fixar(st);
            cqo1_reservar(this, cqo1_d(fx + 0x18), pd);
            return;
        }
        f = if cqo1_selecao(cqo1_d(fx + 0x1c)) { 0x20 } else { 0x40 };
    }
    let buscar: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x85c800usize);
    let r = buscar(this, f, cqo1_d(comp + 0x59), alvo, pd, cqo1_d(fx + 0x1c), cqo1_d(fx + 0x20), fx);
    cqo1_pd(fx + 0x18, r);
    if r == 0 && alt != 0 {
        f = match f {
            0x40 => 0x400,
            0x80 => 0x800,
            0x100 => 0x1000,
            0x200 => 0x2000,
            x => x,
        };
        let r = buscar(this, f, cqo1_d(comp + 0x59), alt, pd, cqo1_d(fx + 0x1c), cqo1_d(fx + 0x20), fx);
        cqo1_pd(fx + 0x18, r);
    }
    let st = cqo1_d(fx + 0x18);
    if st == 0 {
        cqo1_pd(fx + 4, u32::MAX as usize);
    } else {
        cqo1_pd(fx + 4, cqo1_d(st));
    }
}

/// Lado a lado pela partida (0x44 bytes) e pelas regiões de `cqo1_regioes`; partida nula ou reserva que pode alocar:
/// só a original.
unsafe extern "thiscall" fn comparar_estadio_da_partida(this: usize, marcas: u32, partida: usize, estadio: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, u32)>(1887);
    if partida == 0 {
        return o(this, marcas, partida, estadio);
    }
    let Some((r, m)) = cqo1_regioes(this, Some((partida, 0x44))) else {
        // ponytail: a reserva pode alocar a lista; só a original
        return o(this, marcas, partida, estadio);
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&r[..m], || o(this, marcas, partida, estadio), || crate::aleatorio::sem_contar(|| estadio_da_partida(this, marcas, partida, estadio)));
    PLACAR.registrar(igual, || format!("estadio_da_partida(marcas {:x}, comp {:x}): {:x?} x {:x?}", marcas, cqo1_d(partida), mo, mr));
}

// ---------- 279a leva: o público de um jogo (stadium.cpp) ----------

unsafe fn cqo2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqo2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqo2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O sorteio do jogo (0x90cfc0, cdecl): 0..n-1.
unsafe fn cqo2_sortear(n: u32) -> i32 {
    let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A cidade (registro de 0x38 bytes em [0xae23ac]) do estádio `est` (+0x38); sem estádio ou sem cidade, uma sorteada
/// entre as [0xae2354].
unsafe fn cqo2_cidade(est: usize) -> usize {
    if est != 0 && cqo2_d(est + 0x38) != 0 {
        return cqo2_d(est + 0x38);
    }
    let r = cqo2_sortear(cqo2_d(0xae2354) as u32);
    cqo2_d(0xae23ac).wrapping_add((r as usize).wrapping_mul(0x38))
}

/// O piso do fator de cada lado na competição de tipo `t` (+0x3a da partida), pelos limites +0x30/+0x32 da partida e
/// pela reputação (/500) `rep` do clube do lado (0x109a = 4250, 0x1676 = 5750).
unsafe fn cqo2_piso(jogo: usize, lado: u8, f: &mut [f64; 2]) {
    let t = cqo2_b(jogo + 0x3a);
    let rep = |l: u8| cqo2_w(cqo2_d(jogo + if l == 0 { 0x1c } else { 0x20 }) + 0x80);
    let k = lado as usize;
    let piso = |f: &mut f64, m: f64| {
        if *f < m {
            *f = m;
        }
    };
    let alto = |lim: i16| -> bool {
        // os lados além do 1 contam como alto (como a original)
        lado > 1 || rep(lado) >= lim
    };
    if t == 8 {
        let a = cqo2_w(jogo + 0x30) as u16;
        if a == 0x96 {
            if alto(0x1676) {
                piso(&mut f[k], f64_em(0x9690a8));
            } else {
                piso(&mut f[k], f64_em(0x968e38));
            }
        } else if a == 0x82 && f[k] < f64_em(0x968e18) {
            if alto(0x1676) {
                piso(&mut f[k], f64_em(0x968ec8));
            } else {
                piso(&mut f[k], f64_em(0x968930));
            }
        } else if lado > 1 || rep(lado) >= 0x1676 {
            piso(&mut f[k], f64_em(0x9689b8));
        } else if rep(lado) >= 0x109a {
            piso(&mut f[k], f64_em(0x968928));
        } else {
            piso(&mut f[k], f64_em(0x968f98));
        }
    }
    if t == 6 {
        let a = cqo2_w(jogo + 0x30) as u16;
        if a == 0x96 {
            if alto(0x109a) {
                piso(&mut f[k], f64_em(0x9690a8));
            } else {
                piso(&mut f[k], f64_em(0x968e38));
            }
        } else if a == 0x82 && f[k] < f64_em(0x968e18) {
            f[k] = f64_em(0x968e18);
        } else {
            piso(&mut f[k], f64_em(0x968928));
        }
    } else if cqo2_w(jogo + 0x30) as u16 == 0x96 && t == 5 {
        if alto(0x109a) {
            piso(&mut f[k], f64_em(0x9690a8));
        } else {
            piso(&mut f[k], f64_em(0x968e38));
        }
    } else if cqo2_w(jogo + 0x32) as u16 == 0xa0 && t == 5 {
        if lado == 0 {
            if rep(0) < 0x109a && f[0] < f64_em(0x96fe20) {
                f[0] = f64_em(0x96fe20);
            } else {
                piso(&mut f[0], f64_em(0x968e18));
            }
        } else {
            piso(&mut f[k], f64_em(0x968e18));
        }
    } else if t == 10 && f[k] < f64_em(0x968928) {
        f[k] = f64_em(0x968928);
    } else if t == 9 && f[k] < f64_em(0x9689b0) {
        f[k] = f64_em(0x9689b0);
    } else {
        piso(&mut f[k], f64_em(0x967880));
    }
}

/// { 0x85cba0 } (thiscall, ret 8, stadium.cpp, `Stadium::calculate_match_attendance(partida, saida)`; this =
/// [0xdd7ef4]; pelo motor 0x6b7f20): o público do jogo. Sem partida, [0xa835ac] (15 mil). R = reputação do clube /500
/// (no mínimo 1). Para cada lado, o fator começa no número de clubes da competição (+0x69, no mínimo 1; sem competição
/// 1), +2 nas competições de tipo 6, 8 e 10; nas ligas (tabela [0xadadfc], fora de amistoso 0x5c6710 e das nações de
/// 0x7e6f20, sem +0x43 = 2) mais a diferença (+0x34 da partida − +0x3c da liga, entre −6 e 5). O fator do mandante vezes
/// R do visitante / R dele (se ele tem R < 15 ou abaixo do outro), ×1,1 com o outro acima de 17, ×1,25 com o outro
/// acima de 12 e ele abaixo de 14; dividido pelo número de clubes da liga do clube (+0x57 → +0x69), sem ela ×0,2 (tipo
/// 10) ou ×0,05. O do visitante, o mesmo com os papéis trocados. Pisos por tipo de competição (cqo2_piso: copas 8, 6 e
/// 5 com os limites +0x30/+0x32 e a reputação; 10 1,25; 9 0,8; senão 0,1). Com estádio fora de campo neutro: as
/// cidades do estádio e dos dois clubes (sorteadas sem cidade), o fator da distância (0x85d8a0, no mínimo 0,15); o
/// mandante leva a média (+0x73) × fator × distância, o visitante (+0x73 + +0x77) × fator × distância × 0,5; na
/// liga do mandante, pela posição (0x4a4850 na tabela da liga): +50/100/150 por clube nos primeiros, +25/50 na
/// briga, −25/−10 nos de baixo, e pela sequência (+0x72/+0x73/+0x74); no estádio da cidade de um clube, o público dele
/// entre o mínimo (+0x77) e o máximo (+0x7b). Sem estádio ou neutro: média × fator. Clássico (+0xb3/+0xb7/+0xbb):
/// ×1,5 (mesma cidade ×1,35); visitante rival ×1,35; fora de campo neutro com +0x3b != 0xff ×1,05 e ×0,85. Na liga
/// com +0x6d: o mínimo pelo número de clubes da liga (e o máximo com até 4 clubes). Estádio com menos de 1.500
/// lugares passa a R × 1.000 (no mínimo 1.500). Limites pela lotação (o visitante até max(5%, min(lotação − média
/// do visitante × fator do mandante, 15%)) e o mandante o resto; neutro metade), sem estádio 40 mil e 25 mil; sem
/// público, 1 a 50 e 1 a 15. Total = mandante + visitante − sorteio(50) (negativo: 45 a 194). Grava em `saida` o
/// total, o mandante e o visitante e devolve o total.
unsafe extern "thiscall" fn publico_do_jogo(this: usize, jogo: usize, saida: *mut i32) -> i32 {
    if jogo == 0 {
        return cqo2_i(0xa835ac);
    }
    let casa = cqo2_d(jogo + 0x1c);
    let fora = cqo2_d(jogo + 0x20);
    let r500 = |c: usize| -> i8 { (cqo2_w(c + 0x80) as i32 / 500) as i8 };
    let mut rc = r500(casa);
    let mut rf = r500(fora);
    if rc < 1 {
        rc = 1;
    }
    if rf < 1 {
        rf = 1;
    }
    let mut f = [0f64; 2];
    let mut lado: u8 = 0;
    while lado < 2 {
        let k = lado as usize;
        let comp = cqo2_d(jogo + 0x14);
        if comp != 0 {
            f[k] = cqo2_w(comp + 0x69) as i32 as f64;
            if f[k] < f64_em(0x967890) {
                f[k] = 1.0;
            }
        } else {
            f[k] = 1.0;
        }
        let t = cqo2_b(jogo + 0x3a);
        if t == 8 || t == 10 || t == 6 {
            f[k] += f64_em(0x968950);
        }
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        let mut ajuste = f64_em(0x9689a0);
        if amistoso(cqo2_d(0xae2c74), comp) == 0 && cqo2_d(0xadadfc) != 0 && cqo2_d(cqo2_d(0xadadfc) + cqo2_d(comp) * 4) != 0 && cqo2_d(0xb74434) != 0 {
            let grupo: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7e6f20usize);
            if grupo(cqo2_d(jogo + 0x14)) == 0 {
                let liga = cqo2_d(cqo2_d(0xadadfc) + cqo2_d(cqo2_d(jogo + 0x14)) * 4);
                if cqo2_b(liga + 0x43) != 2 {
                    let x = (cqo2_b(jogo + 0x34).wrapping_sub(cqo2_b(liga + 0x3c)) as i8) as i32 as f64;
                    ajuste = if x < f64_em(0x96fe28) {
                        f64_em(0x96fe28)
                    } else if x <= f64_em(0x9688a0) {
                        x
                    } else {
                        f64_em(0x9688a0)
                    };
                }
            }
        }
        let v = ajuste + f[k];
        f[k] = v;
        if lado <= 1 {
            // o mandante (lado 0) contra o visitante; o visitante com os papéis trocados
            let (eu, outro, clube) = if lado == 0 { (rc, rf, casa) } else { (rf, rc, fora) };
            if eu < 0xf || eu < outro {
                f[k] = outro as i32 as f64 / eu as i32 as f64 * v;
            }
            if outro > 0x11 {
                f[k] *= f64_em(0x9689c0);
            }
            if outro > 0xc && eu < 0xe {
                f[k] *= f64_em(0x968928);
            }
            let _ = clube;
            let lg = cqo2_d(cqo2_d(jogo + if lado == 0 { 0x1c } else { 0x20 }) + 0x57);
            if lg != 0 && cqo2_w(lg + 0x69) != 0 {
                f[k] /= cqo2_w(lg + 0x69) as i32 as f64;
            } else if cqo2_b(jogo + 0x3a) == 0xa {
                f[k] *= f64_em(0x968918);
            } else {
                f[k] *= f64_em(0x967888); // cqo2: o fator sem a liga do clube
            }
        }
        cqo2_piso(jogo, lado, &mut f);
        lado += 1;
    }
    let mut pc: i32; // o público do mandante ([esp+0x14] / esi)
    let mut pf: i32; // o do visitante ([esp+0x18])
    let est = cqo2_d(jogo + 0x18);
    if est != 0 && cqo2_b(jogo + 0x3c) == 0 {
        let c_est = cqo2_cidade(est);
        let c_casa = cqo2_cidade(cqo2_d(casa + 0x69));
        let c_fora = cqo2_cidade(cqo2_d(fora + 0x69));
        let fator: unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64 = core::mem::transmute(0x85d8a0usize);
        let mut d = fator(this, jogo, c_est, c_casa, f[0]);
        if d < f64_em(0x968f20) {
            d = f64_em(0x968f20);
        }
        pc = ftol(cqo2_i(casa + 0x73) as f64 * f[0] * d);
        let mut d = fator(this, jogo, c_est, c_fora, f[1]);
        if d < f64_em(0x968f20) {
            d = f64_em(0x968f20);
        }
        pf = ftol(cqo2_i(fora + 0x77).wrapping_add(cqo2_i(fora + 0x73)) as f64 * f[1] * d * f64_em(0x968968));
        let comp = cqo2_d(jogo + 0x14);
        if comp != 0 && casa != 0 && cqo2_d(casa + 0x57) == comp {
            let liga = cqo2_d(cqo2_d(0xadadfc) + cqo2_d(comp) * 4);
            if liga != 0 && cqo2_d(liga + 8) != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                let p = busca(cqo2_d(liga + 8), casa);
                if p != 0 {
                    let n = cqo2_w(comp + 0x69) as i32;
                    let v = cqo2_i(p + 0x19);
                    if v > 1 {
                        crate::comparacao::ramo("cqo_2 posicao de cima");
                        pc = pc.wrapping_add(if v < 4 || n < 0xc {
                            n * 50
                        } else if v < 7 {
                            n * 100 // cqo2: os primeiros
                        } else {
                            n * 150
                        });
                    } else if cqo2_i(p + 0x41) > 2 {
                        pc = pc.wrapping_add(if cqo2_i(p + 0x41) >= 4 && n >= 0xc { n * 50 } else { n * 25 });
                    } else if cqo2_i(p + 0x2d) > 2 {
                        pc = pc.wrapping_sub(n * 25);
                    } else if cqo2_i(p + 0x55) > 3 {
                        pc = pc.wrapping_sub(n * 10);
                    }
                    let dl = cqo2_b(p + 0x74);
                    if dl != cqo2_b(p + 0x75) && cqo2_b(p + 4) > 4 {
                        let cl = cqo2_b(p + 0x73);
                        let bl = cqo2_b(p + 0x72);
                        let mut ax = (bl as i16).wrapping_sub(cl as i16);
                        if bl < 4 && ax <= 0 {
                            ax = if cl == 1 {
                                4
                            } else if cl < 4 {
                                2
                            } else {
                                ftol(ax as i32 as f64 * f64_em(0x9689e0)) as i16
                            };
                        }
                        let m = if bl < 4 && dl == 1 { 25 } else { 15 };
                        pc = pc.wrapping_add(n.wrapping_mul(ax as i32).wrapping_mul(m));
                    }
                }
            }
        }
        if c_est == c_casa {
            let (mx, mn) = (cqo2_i(casa + 0x7b), cqo2_i(casa + 0x77));
            if pc > mx {
                pc = mx;
            } else if pc < mn {
                pc = mn;
            }
        }
        if c_est == c_fora {
            let (mx, mn) = (cqo2_i(fora + 0x7b), cqo2_i(fora + 0x77));
            if pf > mx {
                pf = mx;
            } else if pf < mn {
                pf = mn;
            }
        }
    } else {
        pc = ftol(cqo2_i(casa + 0x73) as f64 * f[0]);
        pf = ftol(cqo2_i(fora + 0x73) as f64 * f[1]);
    }
    // os clássicos
    let (casa, fora) = (cqo2_d(jogo + 0x1c), cqo2_d(jogo + 0x20));
    let rival = |a: usize, b: usize| cqo2_d(a + 0xb3) == b || cqo2_d(a + 0xb7) == b || cqo2_d(a + 0xbb) == b;
    if rival(casa, fora) {
        crate::comparacao::ramo("cqo_2 classico");
        pc = ftol(pc as f64 * f64_em(0x968940)); // cqo2: o clássico
    } else if cqo2_d(casa + 0x69) != 0 && cqo2_d(fora + 0x69) != 0 && cqo2_d(cqo2_d(casa + 0x69) + 0x38) == cqo2_d(cqo2_d(fora + 0x69) + 0x38) {
        pc = ftol(pc as f64 * f64_em(0x969588));
    }
    if rival(fora, casa) {
        pf = ftol(pf as f64 * f64_em(0x969588));
    }
    if cqo2_b(jogo + 0x3c) == 0 && cqo2_b(jogo + 0x3b) != 0xff {
        pc = ftol(pc as f64 * f64_em(0x9689b8));
        pf = ftol(pf as f64 * f64_em(0x969520));
    }
    let comp = cqo2_d(jogo + 0x14);
    if comp != 0 && casa != 0 && cqo2_d(casa + 0x57) == comp && cqo2_b(casa + 0x6d) != 0 {
        let n = cqo2_w(comp + 0x69);
        if n > 0xc {
            let m = (n.wrapping_sub(0xc) as i32 + 3) * 2000;
            if pc < m {
                pc = m;
            }
        } else if n > 8 {
            let m = (n.wrapping_sub(8) as i32 + 4) * 750;
            if pc < m {
                pc = m;
            }
        } else if n > 4 {
            let m = (n.wrapping_sub(4) as i32 + 2) * 500;
            if pc < m {
                pc = m;
            }
        } else {
            let n = n as i32;
            if pc < n * 250 {
                pc = n * 250;
            } else if pc > n * 1500 {
                pc = n * 1500 - ftol((pc - n * 1500) as f64 * f64_em(0x968f30));
            }
        }
    }
    let est = cqo2_d(jogo + 0x18);
    if est != 0 && cqo2_i(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, rc as i32 * 1000);
        if cqo2_i(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    if pf.wrapping_add(pc) != 0 {
        let est = cqo2_d(jogo + 0x18);
        if est != 0 {
            let lot = cqo2_i(est + 0x3c);
            let vis = if cqo2_b(jogo + 0x3c) == 0 {
                let a = lot as f64;
                let x = a - cqo2_i(cqo2_d(jogo + 0x20) + 0x73) as f64 * f[0];
                let y = a * f64_em(0x968f20);
                let z = if x < y { x } else { y };
                let w = a * f64_em(0x967888);
                ftol(if w > z { w } else { z })
            } else {
                lot / 2
            };
            let m = lot.wrapping_sub(vis);
            if pc > m {
                pc = m;
            }
            if pf > vis {
                pf = vis;
            }
        } else {
            if pc > 40000 {
                pc = 40000;
            }
            if pf > 25000 {
                pf = 25000;
            }
        }
    } else {
        pc = cqo2_sortear(0x32) + 1;
        pf = cqo2_sortear(0xf) + 1;
    }
    let r = cqo2_sortear(0x32); // cqo2: o sorteio do total
    let mut total = pf.wrapping_add(pc).wrapping_sub(r);
    if total < 0 {
        total = cqo2_sortear(0x96) + 0x2d;
    }
    if !saida.is_null() {
        *saida = total;
        *saida.add(1) = pc;
        *saida.add(2) = pf;
    }
    total
}

/// Lado a lado (a cópia sem contar os sorteios) pelo estado do sorteador, pela saída, pelos lugares do estádio (que
/// ela pode subir para 1.500) e pela devolução.
unsafe extern "thiscall" fn comparar_publico_do_jogo(this: usize, jogo: usize, saida: *mut i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32>(1889);
    if jogo == 0 {
        return o(this, jogo, saida);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !saida.is_null() {
        reg.push((saida as usize, 12));
    }
    if cqo2_d(jogo + 0x18) != 0 {
        reg.push((cqo2_d(jogo + 0x18) + 0x3c, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida), || crate::aleatorio::sem_contar(|| publico_do_jogo(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("publico_do_jogo(comp {:x}, tipo {}): original {} rust {} {:x?} x {:x?}", cqo2_d(jogo), cqo2_b(jogo + 0x3a), r, n, mo, mr));
    r
}

// ---------- 279a leva: o calendário das ligas francesas de baixo: a terceira divisão (National), a segunda e a divisão de baixo: as 38 rodadas da temporada (fra_third.cpp, fra_second.cpp, fra_lower.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqo4Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqo4_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqo4Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
unsafe fn cqo4_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// O que muda entre as ligas de fase única.
struct Cqo4Liga {
    rodadas: i16,
    arquivo: usize,
    linha: u32,
    tabela: &'static [Cqo4Rodada],
    ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
unsafe fn cqo4_calendario(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo4Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rodadas;
    *extra = 0;
    let t = alocar(p.rodadas as u32 * 0x41);
    if t == 0 {
        aviso_em(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    cqo4_rodadas(t, ano, 0, p.tabela);
    t
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
unsafe fn cqo4_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        cqo4_gravados(a, a, n) == cqo4_gravados(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a terceira divisão (o National), 38 rodadas de 7/ago a 20/mai, sem faixas de TV
const CQO4_TERCEIRA: [Cqo4Rodada; 38] = [
    (7, 7, 0, 5, 2, &[(-1, -1, -1)]), // 0: 7/ago
    (13, 7, 0, 4, 2, &[(-1, -1, -1)]), // 1: 13/ago
    (17, 7, 0, 1, 2, &[(-1, -1, -1)]), // 2: 17/ago
    (21, 7, 0, 5, 2, &[(-1, -1, -1)]), // 3: 21/ago
    (28, 7, 0, 5, 2, &[(-1, -1, -1)]), // 4: 28/ago
    (3, 8, 0, 4, 2, &[(-1, -1, -1)]), // 5: 3/set
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 6: 7/set
    (11, 8, 0, 5, 2, &[(-1, -1, -1)]), // 7: 11/set
    (18, 8, 0, 5, 2, &[(-1, -1, -1)]), // 8: 18/set
    (25, 8, 0, 5, 2, &[(-1, -1, -1)]), // 9: 25/set
    (2, 9, 0, 5, 2, &[(-1, -1, -1)]), // 10: 2/out
    (8, 9, 0, 4, 2, &[(-1, -1, -1)]), // 11: 8/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 12: 12/out
    (16, 9, 0, 5, 2, &[(-1, -1, -1)]), // 13: 16/out
    (30, 9, 0, 5, 2, &[(-1, -1, -1)]), // 14: 30/out
    (12, 10, 0, 4, 2, &[(-1, -1, -1)]), // 15: 12/nov
    (20, 10, 0, 5, 2, &[(-1, -1, -1)]), // 16: 20/nov
    (1, 11, 0, 2, 2, &[(-1, -1, -1)]), // 17: 1/dez
    (4, 11, 0, 5, 2, &[(-1, -1, -1)]), // 18: 4/dez
    (11, 11, 0, 5, 2, &[(-1, -1, -1)]), // 19: 11/dez
    (7, 0, 1, 4, 2, &[(-1, -1, -1)]), // 20: 7/jan (+1 ano)
    (15, 0, 1, 5, 2, &[(-1, -1, -1)]), // 21: 15/jan (+1 ano)
    (28, 0, 1, 4, 2, &[(-1, -1, -1)]), // 22: 28/jan (+1 ano)
    (5, 1, 1, 5, 2, &[(-1, -1, -1)]), // 23: 5/fev (+1 ano)
    (18, 1, 1, 4, 2, &[(-1, -1, -1)]), // 24: 18/fev (+1 ano)
    (26, 1, 1, 5, 2, &[(-1, -1, -1)]), // 25: 26/fev (+1 ano)
    (3, 2, 1, 4, 2, &[(-1, -1, -1)]), // 26: 3/mar (+1 ano)
    (11, 2, 1, 5, 2, &[(-1, -1, -1)]), // 27: 11/mar (+1 ano)
    (18, 2, 1, 5, 2, &[(-1, -1, -1)]), // 28: 18/mar (+1 ano)
    (25, 2, 1, 5, 2, &[(-1, -1, -1)]), // 29: 25/mar (+1 ano)
    (31, 2, 1, 4, 2, &[(-1, -1, -1)]), // 30: 31/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(-1, -1, -1)]), // 31: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(-1, -1, -1)]), // 32: 15/abr (+1 ano)
    (21, 3, 1, 4, 2, &[(-1, -1, -1)]), // 33: 21/abr (+1 ano)
    (29, 3, 1, 5, 2, &[(-1, -1, -1)]), // 34: 29/abr (+1 ano)
    (5, 4, 1, 4, 2, &[(-1, -1, -1)]), // 35: 5/mai (+1 ano)
    (13, 4, 1, 5, 2, &[(-1, -1, -1)]), // 36: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

/// a segunda divisão, 38 rodadas de 31/jul a 20/mai; 18 rodadas com uma faixa de TV (dia 5, turno 1, 1 jogo)
const CQO4_SEGUNDA: [Cqo4Rodada; 38] = [
    (31, 6, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 0: 31/jul
    (7, 7, 0, 5, 2, &[(-1, -1, -1)]), // 1: 7/ago
    (13, 7, 0, 4, 2, &[(-1, -1, -1)]), // 2: 13/ago
    (18, 7, 0, 1, 2, &[(-1, -1, -1)]), // 3: 18/ago
    (21, 7, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 4: 21/ago
    (27, 7, 0, 4, 2, &[(-1, -1, -1)]), // 5: 27/ago
    (3, 8, 0, 4, 2, &[(-1, -1, -1)]), // 6: 3/set
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 7: 7/set
    (11, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 8: 11/set
    (18, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 9: 18/set
    (25, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 10: 25/set
    (1, 9, 0, 4, 2, &[(-1, -1, -1)]), // 11: 1/out
    (8, 9, 0, 4, 2, &[(-1, -1, -1)]), // 12: 8/out
    (16, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 13: 16/out
    (23, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 14: 23/out
    (30, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/out
    (6, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 16: 6/nov
    (9, 10, 0, 1, 2, &[(-1, -1, -1)]), // 17: 9/nov
    (12, 10, 0, 4, 2, &[(-1, -1, -1)]), // 18: 12/nov
    (20, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 19: 20/nov
    (4, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 20: 4/dez
    (11, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 21: 11/dez
    (15, 0, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 22: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(-1, -1, -1)]), // 23: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 25: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(-1, -1, -1)]), // 26: 16/fev (+1 ano)
    (26, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 27: 26/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 28: 11/mar (+1 ano)
    (24, 2, 1, 4, 2, &[(-1, -1, -1)]), // 29: 24/mar (+1 ano)
    (28, 2, 1, 1, 2, &[(-1, -1, -1)]), // 30: 28/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 31: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 32: 15/abr (+1 ano)
    (21, 3, 1, 4, 2, &[(-1, -1, -1)]), // 33: 21/abr (+1 ano)
    (28, 3, 1, 4, 2, &[(-1, -1, -1)]), // 34: 28/abr (+1 ano)
    (5, 4, 1, 4, 2, &[(-1, -1, -1)]), // 35: 5/mai (+1 ano)
    (13, 4, 1, 5, 2, &[(-1, -1, -1)]), // 36: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

/// a divisão de baixo (de fundo), a mesma tabela da segunda, com a faixa também nas duas últimas rodadas
const CQO4_BAIXO: [Cqo4Rodada; 38] = [
    (31, 6, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 0: 31/jul
    (7, 7, 0, 5, 2, &[(-1, -1, -1)]), // 1: 7/ago
    (13, 7, 0, 4, 2, &[(-1, -1, -1)]), // 2: 13/ago
    (18, 7, 0, 1, 2, &[(-1, -1, -1)]), // 3: 18/ago
    (21, 7, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 4: 21/ago
    (27, 7, 0, 4, 2, &[(-1, -1, -1)]), // 5: 27/ago
    (3, 8, 0, 4, 2, &[(-1, -1, -1)]), // 6: 3/set
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 7: 7/set
    (11, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 8: 11/set
    (18, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 9: 18/set
    (25, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 10: 25/set
    (1, 9, 0, 4, 2, &[(-1, -1, -1)]), // 11: 1/out
    (8, 9, 0, 4, 2, &[(-1, -1, -1)]), // 12: 8/out
    (16, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 13: 16/out
    (23, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 14: 23/out
    (30, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/out
    (6, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 16: 6/nov
    (9, 10, 0, 1, 2, &[(-1, -1, -1)]), // 17: 9/nov
    (12, 10, 0, 4, 2, &[(-1, -1, -1)]), // 18: 12/nov
    (20, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 19: 20/nov
    (4, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 20: 4/dez
    (11, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 21: 11/dez
    (15, 0, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 22: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(-1, -1, -1)]), // 23: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 25: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(-1, -1, -1)]), // 26: 16/fev (+1 ano)
    (26, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 27: 26/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 28: 11/mar (+1 ano)
    (24, 2, 1, 4, 2, &[(-1, -1, -1)]), // 29: 24/mar (+1 ano)
    (28, 2, 1, 1, 2, &[(-1, -1, -1)]), // 30: 28/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 31: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 32: 15/abr (+1 ano)
    (21, 3, 1, 4, 2, &[(-1, -1, -1)]), // 33: 21/abr (+1 ano)
    (28, 3, 1, 4, 2, &[(-1, -1, -1)]), // 34: 28/abr (+1 ano)
    (5, 4, 1, 4, 2, &[(-1, -1, -1)]), // 35: 5/mai (+1 ano)
    (13, 4, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 36: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

const CQO4_LIGA_TERCEIRA: Cqo4Liga = Cqo4Liga { rodadas: 0x26, arquivo: 0x9ccc80, linha: 0xa6, tabela: &CQO4_TERCEIRA, ramo: "cqo_4 calendario da terceira francesa" };
const CQO4_LIGA_SEGUNDA: Cqo4Liga = Cqo4Liga { rodadas: 0x26, arquivo: 0x9ccc0c, linha: 0xa7, tabela: &CQO4_SEGUNDA, ramo: "cqo_4 calendario da segunda francesa" };
const CQO4_LIGA_BAIXO: Cqo4Liga = Cqo4Liga { rodadas: 0x26, arquivo: 0x9ccbd0, linha: 0xa9, tabela: &CQO4_BAIXO, ramo: "cqo_4 calendario da divisao de baixo francesa" };

/// { 0x5bfe30 } (thiscall, ret 0x10, fra_third.cpp, 1.948 bytes, método +0x3c): `FraThird::create_schedule(fase, &rodadas, &extra,
/// &é_liga)`: o National, 38 rodadas (`CQO4_TERCEIRA`). Fonte 0x9ccc80, linha 0xa6.
unsafe extern "thiscall" fn calendario_da_terceira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo4_calendario(this, fase, rodadas, extra, liga, &CQO4_LIGA_TERCEIRA)
}

unsafe extern "thiscall" fn comparar_calendario_da_terceira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1890);
    cqo4_comparar("calendario_da_terceira_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_terceira_francesa(this, fase, r, e, l))
}

/// { 0x5be620 } (thiscall, ret 0x10, fra_second.cpp, 2.275 bytes, método +0x3c): `FraSecond::create_schedule`: a segunda divisão,
/// 38 rodadas (`CQO4_SEGUNDA`). Fonte 0x9ccc0c, linha 0xa7.
unsafe extern "thiscall" fn calendario_da_segunda_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo4_calendario(this, fase, rodadas, extra, liga, &CQO4_LIGA_SEGUNDA)
}

unsafe extern "thiscall" fn comparar_calendario_da_segunda_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1891);
    cqo4_comparar("calendario_da_segunda_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_segunda_francesa(this, fase, r, e, l))
}

/// { 0x5bd920 } (thiscall, ret 0x10, fra_lower.cpp, 2.311 bytes, método +0x3c): `FraLower::create_schedule`: a divisão de
/// baixo, 38 rodadas (`CQO4_BAIXO`). Fonte 0x9ccbd0, linha 0xa9.
unsafe extern "thiscall" fn calendario_da_divisao_de_baixo_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo4_calendario(this, fase, rodadas, extra, liga, &CQO4_LIGA_BAIXO)
}

unsafe extern "thiscall" fn comparar_calendario_da_divisao_de_baixo_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1892);
    cqo4_comparar("calendario_da_divisao_de_baixo_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_divisao_de_baixo_francesa(this, fase, r, e, l))
}

// ---------- 279a leva: o calendário da CFA e da Série C1 italiana (as duas chaves): as rodadas da temporada e os play-offs de acesso (fra_cfa.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqo5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqo5_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqo5Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
unsafe fn cqo5_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// O que muda entre as ligas de fase única.
struct Cqo5Liga {
    rodadas: i16,
    arquivo: usize,
    linha: u32,
    tabela: &'static [Cqo5Rodada],
    ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
unsafe fn cqo5_calendario(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo5Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rodadas;
    *extra = 0;
    let t = alocar(p.rodadas as u32 * 0x41);
    if t == 0 {
        aviso_em(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    cqo5_rodadas(t, ano, 0, p.tabela);
    t
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
unsafe fn cqo5_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        cqo5_gravados(a, a, n) == cqo5_gravados(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqo5_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqo5_comparar_fases(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                             original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqo5_gravados(a, a, n) == cqo5_gravados(b, a, n)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, *rodadas)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a CFA (de fundo, 4 grupos de 18), 34 rodadas de 31/jul a 20/mai; quase todas com duas faixas de TV
const CQO5_CFA: [Cqo5Rodada; 34] = [
    (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 0: 31/jul
    (6, 7, 0, 4, 2, &[(-1, -1, -1)]), // 1: 6/ago
    (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 2: 14/ago
    (20, 7, 0, 4, 2, &[(-1, -1, -1)]), // 3: 20/ago
    (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 4: 28/ago
    (10, 8, 0, 4, 2, &[(-1, -1, -1)]), // 5: 10/set
    (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 4, 2, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 8: 2/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 9: 12/out
    (15, 9, 0, 4, 2, &[(-1, -1, -1)]), // 10: 15/out
    (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 11: 23/out
    (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 12: 30/out
    (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 13: 6/nov
    (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 14: 10/nov
    (19, 10, 0, 4, 2, &[(-1, -1, -1)]), // 15: 19/nov
    (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 16: 27/nov
    (3, 11, 0, 4, 2, &[(-1, -1, -1)]), // 17: 3/dez
    (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 18: 11/dez
    (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 19: 15/dez
    (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 20: 18/dez
    (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)]), // 21: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 22: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 23: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 24: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)]), // 25: 16/fev (+1 ano)
    (27, 1, 1, 4, 2, &[(-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 27: 11/mar (+1 ano)
    (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 28: 25/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)]), // 29: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 30: 15/abr (+1 ano)
    (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)]), // 31: 29/abr (+1 ano)
    (13, 4, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)]), // 32: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(4, 2, 1), (4, 1, 1), (-1, -1, -1)]), // 33: 20/mai (+1 ano)
];

/// a Série C1 (as duas chaves, a mesma tabela), 34 rodadas de domingo de 5/set a 7/mai, sem faixas de TV
const CQO5_C1: [Cqo5Rodada; 34] = [
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 0: 5/set
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 12/set
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 2: 19/set
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 26/set
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 4: 3/out
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 5: 10/out
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 6: 17/out
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 7: 24/out
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 8: 31/out
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 7/nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 10: 14/nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 11: 21/nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 12: 28/nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 13: 5/dez
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 14: 12/dez
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 15: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 16: 6/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]), // 17: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]), // 18: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]), // 19: 30/jan (+1 ano)
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 21: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 22: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(-1, -1, -1)]), // 23: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 24: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 25: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 27: 2/abr (+1 ano)
    (5, 3, 1, 2, 2, &[(-1, -1, -1)]), // 28: 5/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 29: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 32: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 7/mai (+1 ano)
];

const CQO5_LIGA_CFA: Cqo5Liga = Cqo5Liga { rodadas: 0x22, arquivo: 0x9ccaf0, linha: 0xab, tabela: &CQO5_CFA, ramo: "cqo_5 calendario da cfa" };

/// { 0x5b9510 } (thiscall, ret 0x10, fra_cfa.cpp, 2.703 bytes, método +0x3c): `FraCfa::create_schedule(fase, &rodadas, &extra,
/// &é_liga)`: a CFA, 34 rodadas (`CQO5_CFA`). Fonte 0x9ccaf0, linha 0xab.
unsafe extern "thiscall" fn calendario_da_cfa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_calendario(this, fase, rodadas, extra, liga, &CQO5_LIGA_CFA)
}

unsafe extern "thiscall" fn comparar_calendario_da_cfa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1893);
    cqo5_comparar("calendario_da_cfa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_cfa(this, fase, r, e, l))
}

/// As fases de copa da C1 (rodadas de 0x68 bytes, `cql10_rodada`; categoria 5, sem prêmios, o ano +0x40 com sinal):
/// a fase 0, os play-offs de acesso: as semifinais (sorteio em 8/mai, ida em 21/mai, ida e volta com 7 dias entre os
/// jogos, 4 times em 2 confrontos, todos novos; 0xa00 no jogo único = sem sorteio (0x200)
/// mais 0x800, 0x802 no confronto de dois jogos) e a final (sorteio em 29/mai, ida em 4/jun, 2 times); a fase 1, os
/// play-outs [inferência: o rebaixamento]: uma rodada de 2 confrontos de ida e volta nas datas das semifinais, sem
/// sorteio (0x200), nome 0xbe.
const CQO5_SEMIFINAIS: Cql10R = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0x82, (0, 0xa00, 0x802), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0)); // cqo5: as semifinais da C1
const CQO5_FINAL: Cql10R = cql10r((0x1d, 4, 1, 0), (4, 5, 1, 6, 1, 0), 0x96, (0, 0x800, 0x802), 5, (2, 1, 0, 0), (0, 2, 7), (0, 0, 0));
const CQO5_PLAYOUT: Cql10R = cql10r((8, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0xbe, (0, 0x200, 0x800), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));

/// O que muda entre as duas chaves da C1: a fonte e as linhas das caixas (fase −1, 0 e 1).
struct Cqo5C1 {
    arquivo: usize,
    linhas: [u32; 3],
}

/// O corpo comum da C1 A e da C1 B (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo), pela
/// fase (o byte): −1 a fase principal (é_liga = 1, 34 rodadas, extra 0, a tabela de 0x8a2 bytes com `CQO5_C1`, o ano
/// +0x40 como palavra); 0 os play-offs (é_liga = 0, 2 rodadas, extra 0xa0, a tabela de 0xd0 bytes com as semifinais e a
/// final); 1 os play-outs (é_liga = 0, 1 rodada, extra 0, 0x68 bytes); as outras devolvem 0 sem gravar nada. Sem memória
/// (malloc 0x944e46: push; add esp, 4 -> cdecl): a caixa da linha da fase e 0. Devolve a tabela.
unsafe fn cqo5_c1(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo5C1) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let (k, l, n, x, tamanho) = match fase as u8 {
        0xff => (0, 1, 0x22, 0, 0x8a2),
        0 => (1, 0, 2, 0xa0, 0xd0),
        1 => (2, 0, 1, 0, 0x68),
        _ => return 0,
    };
    if !liga.is_null() {
        *liga = l;
    }
    *rodadas = n;
    *extra = x;
    let t = alocar(tamanho);
    if t == 0 {
        aviso_em(p.arquivo, p.linhas[k]);
        return 0;
    }
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16);
    if k == 0 {
        crate::comparacao::ramo("cqo_5 calendario da c1");
        cqo5_rodadas(t, ano as u16 as u32, 0, &CQO5_C1);
    } else {
        crate::comparacao::ramo("cqo_5 play-offs da c1");
        let ano = ano as i32 as u32;
        if k == 1 {
            cql10_rodada(t, 0, &CQO5_SEMIFINAIS, ano);
            cql10_rodada(t, 1, &CQO5_FINAL, ano);
        } else {
            cql10_rodada(t, 0, &CQO5_PLAYOUT, ano);
        }
    }
    t
}

/// { 0x64ad80 } (thiscall, ret 0x10, ita_ser_c1a.cpp, 2.279 bytes, método +0x3c): `ItaSerC1a::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a chave A da Série C1. Fonte 0x9ea474, linhas 0xab, 0x155, 0x18f.
unsafe extern "thiscall" fn calendario_da_c1_a(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_c1(this, fase, rodadas, extra, liga, &Cqo5C1 { arquivo: 0x9ea474, linhas: [0xab, 0x155, 0x18f] })
}

unsafe extern "thiscall" fn comparar_calendario_da_c1_a(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1894);
    cqo5_comparar_fases("calendario_da_c1_a", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_c1_a(this, fase, r, e, l))
}

/// { 0x64ef30 } (thiscall, ret 0x10, ita_ser_c1b.cpp, 2.279 bytes, método +0x3c): `ItaSerC1b::create_schedule`: a
/// chave B, o mesmo código. Fonte 0x9ea558, linhas 0xac, 0x156, 0x190.
unsafe extern "thiscall" fn calendario_da_c1_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo5_c1(this, fase, rodadas, extra, liga, &Cqo5C1 { arquivo: 0x9ea558, linhas: [0xac, 0x156, 0x190] })
}

unsafe extern "thiscall" fn comparar_calendario_da_c1_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1895);
    cqo5_comparar_fases("calendario_da_c1_b", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_c1_b(this, fase, r, e, l))
}

// ---------- 279a leva: o calendário das primeiras divisões da Itália e da França e da Série B: as rodadas da temporada, o jogo-desempate italiano e as 34 ou 38 rodadas da francesa (ita_ser_a.cpp, ita_ser_b.cpp, fra_first.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqo6Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqo6_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqo6Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
unsafe fn cqo6_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// O que muda entre as ligas de fase única.
struct Cqo6Liga {
    rodadas: i16,
    arquivo: usize,
    linha: u32,
    tabela: &'static [Cqo6Rodada],
    ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
unsafe fn cqo6_calendario(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo6Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rodadas;
    *extra = 0;
    let t = alocar(p.rodadas as u32 * 0x41);
    if t == 0 {
        aviso_em(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    cqo6_rodadas(t, ano, 0, p.tabela);
    t
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
unsafe fn cqo6_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        cqo6_gravados(a, a, n) == cqo6_gravados(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqo6_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqo6_comparar_fases(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                             original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqo6_gravados(a, a, n) == cqo6_gravados(b, a, n)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, *rodadas)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a Série A, 34 rodadas de 28/ago a 12/mai; quase todas com duas faixas de TV (dias 6 e 5, turno 2, 2 jogos cada)
const CQO6_SERIE_A: [Cqo6Rodada; 34] = [
    (28, 7, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 0: 28/ago
    (9, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 1: 9/set
    (16, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 2: 16/set
    (23, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 3: 23/set
    (30, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 4: 30/set
    (10, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 5: 10/out
    (14, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 6: 14/out
    (21, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 7: 21/out
    (28, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 8: 28/out
    (4, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 9: 4/nov
    (18, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 10: 18/nov
    (25, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 11: 25/nov
    (2, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 12: 2/dez
    (9, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 13: 9/dez
    (16, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 14: 16/dez
    (6, 0, 1, 2, 2, &[(-1, -1, -1)]), // 15: 6/jan (+1 ano)
    (13, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 16: 13/jan (+1 ano)
    (20, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 17: 20/jan (+1 ano)
    (27, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 18: 27/jan (+1 ano)
    (3, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 19: 3/fev (+1 ano)
    (10, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 20: 10/fev (+1 ano)
    (17, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 21: 17/fev (+1 ano)
    (24, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 22: 24/fev (+1 ano)
    (3, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 23: 3/mar (+1 ano)
    (10, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 24: 10/mar (+1 ano)
    (17, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 25: 17/mar (+1 ano)
    (24, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 26: 24/mar (+1 ano)
    (30, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 30/mar (+1 ano)
    (7, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 28: 7/abr (+1 ano)
    (14, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 29: 14/abr (+1 ano)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 21/abr (+1 ano)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 28/abr (+1 ano)
    (5, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 5/mai (+1 ano)
    (12, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 12/mai (+1 ano)
];

/// a Série B, 38 rodadas de 29/ago a 14/mai; quase todas com duas faixas de TV (dias 4 e 0, turno 2, 2 jogos cada)
const CQO6_SERIE_B: [Cqo6Rodada; 38] = [
    (29, 7, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 0: 29/ago
    (5, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 1: 5/set
    (12, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 5: 3/out
    (10, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 6: 10/out
    (17, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 7: 17/out
    (24, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 8: 24/out
    (31, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 9: 31/out
    (7, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 10: 7/nov
    (14, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 11: 14/nov
    (21, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 12: 21/nov
    (28, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 13: 28/nov
    (5, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 14: 5/dez
    (12, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 15: 12/dez
    (19, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 16: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 17: 6/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 18: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 19: 16/jan (+1 ano)
    (19, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 19/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 21: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 22: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 23: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 24: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 25: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 27: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 28: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 29: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 30: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 31: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 32: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 33: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 34: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 36: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 14/mai (+1 ano)
];

/// A primeira divisão francesa: as rodadas em ordem, cada uma marcada `true` se só entra com 38 rodadas (20 clubes; com
/// 34, as de `false`, em sequência). O índice no comentário é o da temporada de 38.
const CQO6_PRIMEIRA_FRANCESA: [(bool, Cqo6Rodada); 38] = [
    (false, (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 0: 31/jul
    (false, (6, 7, 0, 4, 2, &[(-1, -1, -1)])), // 1: 6/ago
    (false, (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 2: 14/ago
    (false, (20, 7, 0, 4, 2, &[(-1, -1, -1)])), // 3: 20/ago
    (false, (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 4: 28/ago
    (false, (10, 8, 0, 4, 2, &[(-1, -1, -1)])), // 5: 10/set
    (false, (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 6: 18/set
    (false, (25, 8, 0, 4, 2, &[(-1, -1, -1)])), // 7: 25/set
    (false, (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 8: 2/out
    (false, (12, 9, 0, 1, 2, &[(-1, -1, -1)])), // 9: 12/out
    (false, (15, 9, 0, 4, 2, &[(-1, -1, -1)])), // 10: 15/out
    (false, (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 11: 23/out
    (false, (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 12: 30/out
    (false, (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 13: 6/nov
    (false, (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 14: 10/nov
    (false, (19, 10, 0, 4, 2, &[(-1, -1, -1)])), // 15: 19/nov
    (false, (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 16: 27/nov
    (false, (3, 11, 0, 4, 2, &[(-1, -1, -1)])), // 17: 3/dez
    (false, (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 18: 11/dez
    (false, (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 19: 15/dez
    (false, (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 20: 18/dez
    (true, (8, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 21: 8/jan (+1 ano)
    (false, (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 22: 15/jan (+1 ano)
    (true, (22, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 23: 22/jan (+1 ano)
    (false, (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 24: 26/jan (+1 ano)
    (false, (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 25: 2/fev (+1 ano)
    (false, (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 26: 5/fev (+1 ano)
    (false, (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 27: 16/fev (+1 ano)
    (false, (27, 1, 1, 4, 2, &[(-1, -1, -1)])), // 28: 27/fev (+1 ano)
    (true, (4, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 29: 4/mar (+1 ano)
    (false, (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 30: 11/mar (+1 ano)
    (true, (18, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 31: 18/mar (+1 ano)
    (false, (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 32: 25/mar (+1 ano)
    (false, (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 33: 8/abr (+1 ano)
    (false, (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 34: 15/abr (+1 ano)
    (false, (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)])), // 35: 29/abr (+1 ano)
    (false, (13, 4, 1, 5, 2, &[(-1, -1, -1)])), // 36: 13/mai (+1 ano)
    (false, (20, 4, 1, 5, 2, &[(-1, -1, -1)])), // 37: 20/mai (+1 ano)
];

const CQO6_LIGA_SERIE_A: Cqo6Liga = Cqo6Liga { rodadas: 0x22, arquivo: 0x9ea2a8, linha: 0xb0, tabela: &CQO6_SERIE_A, ramo: "cqo_6 calendario da serie a" };
const CQO6_LIGA_SERIE_B: Cqo6Liga = Cqo6Liga { rodadas: 0x26, arquivo: 0x9ea384, linha: 0xa8, tabela: &CQO6_SERIE_B, ramo: "cqo_6 calendario da serie b" };

/// O jogo-desempate das ligas italianas (fase 0, uma rodada de copa de 0x68 bytes, `cql10_rodada`; o ano +0x40 com
/// sinal): o sorteio e o jogo em maio do ano seguinte (a Série A: sorteio em 13/mai, jogo em 19/mai; a Série B: 15/mai e
/// 21/mai), nome 0xa0, sem sorteio (0x200), desempate do confronto 3, categoria 5, ida e volta (+0x21 = 2) com 3 dias
/// entre os jogos, sem prêmios; os times (2n, n jogos, 2n novos) pelo byte +0xee da liga (n, com sinal; as contas em 16
/// bits como a original) [inferência: os confrontos de desempate pedidos].
fn cqo6_desempate(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), n: i8) -> Cql10R {
    let n = n as i16;
    let dois = n.wrapping_shl(1) as u16;
    cql10r(sorteio, jogo, 0xa0, (0, 0x200, 3), 5, (dois, n as u16, dois, 0), (0, 2, 3), (0, 0, 0))
}

/// O que muda entre a Série A e a Série B.
struct Cqo6Italia {
    liga: &'static Cqo6Liga,
    sorteio: (u32, u32, u32, u32),
    jogo: (u32, u32, u32, u32, u32, u32),
    linha_desempate: u32,
}

/// O corpo comum das duas (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): a fase −1 é a
/// principal (`cqo6_calendario`); a 0, o jogo-desempate: é_liga (se pedido) = 0, 1 rodada, extra 0, a tabela de 0x68
/// bytes (malloc 0x944e46; sem memória: a caixa e 0); as outras devolvem 0 sem gravar nada. Devolve a tabela.
unsafe fn cqo6_italia(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo6Italia) -> usize {
    match fase as u8 {
        0xff => cqo6_calendario(this, fase, rodadas, extra, liga, p.liga),
        0 => {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 1;
            *extra = 0;
            let t = alocar(0x68);
            if t == 0 {
                aviso_em(p.liga.arquivo, p.linha_desempate);
                return 0;
            }
            crate::comparacao::ramo("cqo_6 jogo-desempate italiano");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            cql10_rodada(t, 0, &cqo6_desempate(p.sorteio, p.jogo, *((this + 0xee) as *const i8)), ano);
            t
        }
        _ => 0,
    }
}

const CQO6_SERIE_A_E_DESEMPATE: Cqo6Italia = Cqo6Italia { liga: &CQO6_LIGA_SERIE_A, sorteio: (0xd, 4, 1, 0), jogo: (0x13, 4, 1, 6, 1, 0), linha_desempate: 0x226 };
const CQO6_SERIE_B_E_DESEMPATE: Cqo6Italia = Cqo6Italia { liga: &CQO6_LIGA_SERIE_B, sorteio: (0xf, 4, 1, 0), jogo: (0x15, 4, 1, 6, 1, 0), linha_desempate: 0x170 };

/// { 0x6409b0 } (thiscall, ret 0x10, ita_ser_a.cpp, 2.901 bytes, método +0x3c): `ItaSerA::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a Série A, 34 rodadas (`CQO6_SERIE_A`), e o jogo-desempate. Fonte 0x9ea2a8, linhas 0xb0
/// e 0x226.
unsafe extern "thiscall" fn calendario_da_serie_a(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo6_italia(this, fase, rodadas, extra, liga, &CQO6_SERIE_A_E_DESEMPATE)
}

unsafe extern "thiscall" fn comparar_calendario_da_serie_a(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1896);
    cqo6_comparar_fases("calendario_da_serie_a", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_serie_a(this, fase, r, e, l))
}

/// { 0x645af0 } (thiscall, ret 0x10, ita_ser_b.cpp, 3.215 bytes, método +0x3c): `ItaSerB::create_schedule`: a Série B,
/// 38 rodadas (`CQO6_SERIE_B`), e o jogo-desempate. Fonte 0x9ea384, linhas 0xa8 e 0x170.
unsafe extern "thiscall" fn calendario_da_serie_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo6_italia(this, fase, rodadas, extra, liga, &CQO6_SERIE_B_E_DESEMPATE)
}

unsafe extern "thiscall" fn comparar_calendario_da_serie_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1897);
    cqo6_comparar_fases("calendario_da_serie_b", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_serie_b(this, fase, r, e, l))
}

/// { 0x5bb690 } (thiscall, ret 0x10, fra_first.cpp, 3.090 bytes, método +0x3c): `FraFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1 (as outras devolvem 0). Em 2001 (o ano +0x40 = 0x7d1) 34 rodadas (18
/// clubes); nos outros anos 38 (20 clubes). é_liga (se pedido) = 1, as rodadas, extra 0, a tabela de rodadas × 0x41
/// bytes (sem memória: a caixa da linha 0xd1 e 0); as rodadas de `CQO6_PRIMEIRA_FRANCESA` em sequência (as marcadas só
/// entram com 38); no fim, se a conta não bate com as rodadas, a caixa da linha 0x1b2 (e devolve a tabela assim mesmo).
/// Fonte 0x9ccb5c.
unsafe extern "thiscall" fn calendario_da_primeira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16);
    let n: i16 = if ano == 0x7d1 { 0x22 } else { 0x26 }; // cqo6: 34 rodadas em 2001, 38 depois
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = n;
    *extra = 0;
    let t = alocar((n as i32 * 0x41) as u32);
    if t == 0 {
        aviso_em(0x9ccb5c, 0xd1);
        return 0;
    }
    crate::comparacao::ramo("cqo_6 calendario da primeira francesa");
    let mut i: i16 = 0;
    for (so_38, r) in CQO6_PRIMEIRA_FRANCESA.iter() {
        if *so_38 && n != 0x26 {
            continue;
        }
        cqo6_rodadas(t, ano as u32, i as i32, core::slice::from_ref(r));
        i += 1;
    }
    if i != n {
        aviso_em(0x9ccb5c, 0x1b2);
    }
    t
}

unsafe extern "thiscall" fn comparar_calendario_da_primeira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1898);
    cqo6_comparar("calendario_da_primeira_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_primeira_francesa(this, fase, r, e, l))
}

// ---------- 279a leva: o Mineiro: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_minas_gerais.cpp) ----------

unsafe fn cqo7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqo7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqo7_campeao_do_turno(this: usize, k: usize) -> usize {
    cqo7_d(cqo7_d(cqo7_d(cqo7_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x434ae0 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::advance_stage()`, a fase seguinte do Mineiro (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x434b70, call direto com o ecx da liga).
unsafe extern "thiscall" fn cqo7_fase_seguinte_do_mineiro(this: usize) {
    let fase = cqo7_d(this + 0x30) as i32;
    if fase >= (cqo7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x98aed0, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x434b70usize);
        final_(this); // cqo7: a final depois do segundo turno
    }
}

/// { 0x434b70 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe extern "thiscall" fn cqo7_final_do_mineiro(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x98aed0, 0x226);
    }
    let a = cqo7_campeao_do_turno(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = cqo7_campeao_do_turno(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_7 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((cqo7_d(cqo7_d(cqo7_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo7: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < cqo7_w(this + 0x3e) {
            let linha = cqo7_d(this + 0xb1) + i as usize * 0x3b;
            if cqo7_d(linha) == cqo7_d(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqo7_d(cqo7_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqo7_d(this + 4), t, cqo7_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((cqo7_d(this + 0xc) + 8) as *mut usize, fase);
    let fase = cqo7_d(cqo7_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0x98aed0, 0x250);
    }
    crate::comparacao::ramo("cqo_7 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x434ed0 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::record_season_honours()` (slot 13), o fim da temporada do Mineiro.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 350.000 ao campeão (primeira linha da tabela +0xb1) e 100.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
unsafe extern "thiscall" fn cqo7_fim_da_temporada_do_mineiro(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    if cqo7_d(this + 0x30) == 2 {
        let f = cqo7_d(cqo7_d(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqo7_d(cqo7_d(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_7 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684640usize);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    for (linha, valor, posicao) in [(0usize, 350000i32, 1i32), (0x3b, 100000, 2)] {
        let c = cqo7_d(cqo7_d(this + 0xb1) + linha);
        premio(financas(cqo7_d(0xae2a38), c), valor);
        noticia(this, cqo7_d(cqo7_d(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_7 premios");
    campeoes()
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
unsafe extern "thiscall" fn comparar_cqo7_fase_seguinte_do_mineiro(this: usize) {
    let antes = cqo7_d(this + 0x30) as i32;
    let ultima = antes >= (cqo7_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1899)(this);
    let depois = cqo7_d(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("cqo7_fase_seguinte_do_mineiro({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqo7_final_do_mineiro(this: usize) {
    let (a, b) = (cqo7_campeao_do_turno(this, 0), cqo7_campeao_do_turno(this, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(1900)(this);
    let igual = if a == b {
        let marca = |k: usize| *((cqo7_d(cqo7_d(cqo7_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..cqo7_w(this + 0x3e).max(0) as usize).all(|i| {
            let linha = cqo7_d(this + 0xb1) + i * 0x3b;
            cqo7_d(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        cqo7_d(this + 0x30) == 1 && cqo7_d(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = cqo7_d(cqo7_d(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = cqo7_d(fase + 0xa7);
            let (x, y) = (cqo7_d(l), cqo7_d(l + 6));
            cqo7_w(fase + 0x3e) == 2 && cqo7_d(fase + 4) == cqo7_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqo7_d(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("cqo7_final_do_mineiro({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, cqo7_d(this + 0x30), cqo7_d(this + 0x2c)));
}

/// Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_7 historico da final" e "cqo_7 premios") e é provada pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqo7_fim_da_temporada_do_mineiro(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1901)(this)
}

// ---------- 279a leva: o Paranaense: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_parana.cpp) ----------

unsafe fn cqo8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqo8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqo8_campeao_do_turno(this: usize, k: usize) -> usize {
    cqo8_d(cqo8_d(cqo8_d(cqo8_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x438230 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::advance_stage()`, a fase seguinte do Paranaense (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x4382c0, call direto com o ecx da liga).
unsafe extern "thiscall" fn cqo8_fase_seguinte_do_paranaense(this: usize) {
    let fase = cqo8_d(this + 0x30) as i32;
    if fase >= (cqo8_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x98af98, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4382c0usize);
        final_(this); // cqo8: a final depois do segundo turno
    }
}

/// { 0x4382c0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe extern "thiscall" fn cqo8_final_do_paranaense(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x98af98, 0x226);
    }
    let a = cqo8_campeao_do_turno(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = cqo8_campeao_do_turno(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_8 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((cqo8_d(cqo8_d(cqo8_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo8: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < cqo8_w(this + 0x3e) {
            let linha = cqo8_d(this + 0xb1) + i as usize * 0x3b;
            if cqo8_d(linha) == cqo8_d(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqo8_d(cqo8_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqo8_d(this + 4), t, cqo8_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((cqo8_d(this + 0xc) + 8) as *mut usize, fase);
    let fase = cqo8_d(cqo8_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0x98af98, 0x250);
    }
    crate::comparacao::ramo("cqo_8 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x4384e0 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::record_season_honours()` (slot 13), o fim da temporada do Paranaense.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 175.000 ao campeão (primeira linha da tabela +0xb1) e 100.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
unsafe extern "thiscall" fn cqo8_fim_da_temporada_do_paranaense(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    if cqo8_d(this + 0x30) == 2 {
        let f = cqo8_d(cqo8_d(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqo8_d(cqo8_d(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_8 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684640usize);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    for (linha, valor, posicao) in [(0usize, 175000i32, 1i32), (0x3b, 100000, 2)] {
        let c = cqo8_d(cqo8_d(this + 0xb1) + linha);
        premio(financas(cqo8_d(0xae2a38), c), valor);
        noticia(this, cqo8_d(cqo8_d(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_8 premios");
    campeoes()
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
unsafe extern "thiscall" fn comparar_cqo8_fase_seguinte_do_paranaense(this: usize) {
    let antes = cqo8_d(this + 0x30) as i32;
    let ultima = antes >= (cqo8_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1902)(this);
    let depois = cqo8_d(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("cqo8_fase_seguinte_do_paranaense({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqo8_final_do_paranaense(this: usize) {
    let (a, b) = (cqo8_campeao_do_turno(this, 0), cqo8_campeao_do_turno(this, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(1903)(this);
    let igual = if a == b {
        let marca = |k: usize| *((cqo8_d(cqo8_d(cqo8_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..cqo8_w(this + 0x3e).max(0) as usize).all(|i| {
            let linha = cqo8_d(this + 0xb1) + i * 0x3b;
            cqo8_d(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        cqo8_d(this + 0x30) == 1 && cqo8_d(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = cqo8_d(cqo8_d(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = cqo8_d(fase + 0xa7);
            let (x, y) = (cqo8_d(l), cqo8_d(l + 6));
            cqo8_w(fase + 0x3e) == 2 && cqo8_d(fase + 4) == cqo8_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqo8_d(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("cqo8_final_do_paranaense({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, cqo8_d(this + 0x30), cqo8_d(this + 0x2c)));
}

/// Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_8 historico da final" e "cqo_8 premios") e é provada pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqo8_fim_da_temporada_do_paranaense(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1904)(this)
}

// ---------- 279a leva: o Carioca: a fase seguinte, a final entre os campeões dos dois turnos e o fim da temporada (bra_reg_rio.cpp) ----------

unsafe fn cqo9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqo9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqo9_campeao_do_turno(this: usize, k: usize) -> usize {
    cqo9_d(cqo9_d(cqo9_d(cqo9_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x43a870 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::advance_stage()`, a fase seguinte do Carioca (dois turnos, as fases 0
/// e 1, cada um um League_Stage, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa de
/// erro da linha 0x202 e nada mais. Senão +0x30 sobe um e, ao chegar a 2 (o fim do segundo turno), monta a final
/// (0x43a900, call direto com o ecx da liga).
unsafe extern "thiscall" fn cqo9_fase_seguinte_do_carioca(this: usize) {
    let fase = cqo9_d(this + 0x30) as i32;
    if fase >= (cqo9_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x98b014, 0x202);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x43a900usize);
        final_(this); // cqo9: a final depois do segundo turno
    }
}

/// { 0x43a900 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_final_stage()`, da fase seguinte quando o segundo turno
/// acaba. Os dois campeões dos turnos (o clube da primeira linha da tabela de cada League_Stage) vão para uma lista
/// de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x226 e volta).
/// - O mesmo clube nos dois turnos: é o campeão sem final. +0x30 = 1, +0x2c = 2 (a temporada acaba com as duas fases),
///   a situação 5 (+0x37, campeão) na primeira linha da tabela de cada turno e na linha dele da tabela da liga
///   (+0xb1, as +0x3e linhas). A lista não é liberada (a original a deixa vazar; a cópia também).
/// - Campeões diferentes: a final, em dois jogos [inferência: ida e volta]. A virtual +0x3c (create_round_table:
///   thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new
///   0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela,
///   o ano +0x40, 2, 1, o extra, 0x14, 0, 1, 0 e as sementes {0, 1}: o campeão do primeiro turno é a semente 0) vai
///   para a terceira vaga das fases ([+0xc]+8; sem fase: a caixa da linha 0x250 e volta, sem liberar). Com ela:
///   Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela e a lista liberadas (free 0x9452ca, nessa
///   ordem) e +0x30 = 2. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe extern "thiscall" fn cqo9_final_do_carioca(this: usize) {
    let sementes: [u8; 4] = [0, 1, 0, 0];
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x98b014, 0x226);
    }
    let a = cqo9_campeao_do_turno(this, 0);
    core::ptr::write_unaligned(lista as *mut usize, a);
    let b = cqo9_campeao_do_turno(this, 1);
    core::ptr::write_unaligned((lista + 4) as *mut usize, b);
    if a == b {
        crate::comparacao::ramo("cqo_9 mesmo campeao");
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 1);
        core::ptr::write_unaligned((this + 0x2c) as *mut u32, 2);
        for k in 0..2 {
            *((cqo9_d(cqo9_d(cqo9_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *mut u8) = 5; // cqo9: campeão dos dois turnos
        }
        let mut i: i16 = 0;
        while i < cqo9_w(this + 0x3e) {
            let linha = cqo9_d(this + 0xb1) + i as usize * 0x3b;
            if cqo9_d(linha) == cqo9_d(lista) {
                *((linha + 0x37) as *mut u8) = 5;
            }
            i += 1;
        }
        // ponytail: a lista de 8 bytes fica sem liberar, como na original.
        return;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqo9_d(cqo9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqo9_d(this + 4), t, cqo9_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 1, 0, sementes.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned((cqo9_d(this + 0xc) + 8) as *mut usize, fase);
    let fase = cqo9_d(cqo9_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0x98b014, 0x250);
    }
    crate::comparacao::ramo("cqo_9 final");
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x43ad20 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::record_season_honours()` (slot 13), o fim da temporada do Carioca.
/// - Com final (+0x30 = 2): o histórico da final (o slot 13, +0x30, da fase [+0xc]+8) e, em salto final, a 0x42bba0
///   (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze estaduais terminam, põe os
///   campeões na Copa dos Campeões); devolve o eax dela.
/// - Sem final (o mesmo campeão nos dois turnos): o histórico da liga (League::record_season_honours 0x684640,
///   thiscall, ret 0); os prêmios, 675.000 ao campeão (primeira linha da tabela +0xb1) e 250.000 ao vice (segunda linha),
///   cada um no saldo (0x5a0590 em [0xae2a38] -> as finanças do clube, thiscall, ret 4; 0x5999a0 Finance::add_prize_money,
///   thiscall, ret 4) e na notícia do prêmio (0x48e3e0 Comp::send_prize_money_news: thiscall(this, clube, valor em 64
///   bits, posição 1 ou 2, rodada 0, partida 0, tipo −2), ret 0x1c); depois a 0x42bba0, como acima.
unsafe extern "thiscall" fn cqo9_fim_da_temporada_do_carioca(this: usize) -> u32 {
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    if cqo9_d(this + 0x30) == 2 {
        let f = cqo9_d(cqo9_d(this + 0xc) + 8);
        let historico: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqo9_d(cqo9_d(f) + 0x30));
        historico(f);
        crate::comparacao::ramo("cqo_9 historico da final");
        return campeoes();
    }
    let historico: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684640usize);
    historico(this);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let premio: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    for (linha, valor, posicao) in [(0usize, 675000i32, 1i32), (0x3b, 250000, 2)] {
        let c = cqo9_d(cqo9_d(this + 0xb1) + linha);
        premio(financas(cqo9_d(0xae2a38), c), valor);
        noticia(this, cqo9_d(cqo9_d(this + 0xb1) + linha), valor, 0, posicao, 0, 0, -2);
    }
    crate::comparacao::ramo("cqo_9 premios");
    campeoes()
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
unsafe extern "thiscall" fn comparar_cqo9_fase_seguinte_do_carioca(this: usize) {
    let antes = cqo9_d(this + 0x30) as i32;
    let ultima = antes >= (cqo9_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1905)(this);
    let depois = cqo9_d(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("cqo9_fase_seguinte_do_carioca({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqo9_final_do_carioca(this: usize) {
    let (a, b) = (cqo9_campeao_do_turno(this, 0), cqo9_campeao_do_turno(this, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(1906)(this);
    let igual = if a == b {
        let marca = |k: usize| *((cqo9_d(cqo9_d(cqo9_d(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..cqo9_w(this + 0x3e).max(0) as usize).all(|i| {
            let linha = cqo9_d(this + 0xb1) + i * 0x3b;
            cqo9_d(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        cqo9_d(this + 0x30) == 1 && cqo9_d(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = cqo9_d(cqo9_d(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = cqo9_d(fase + 0xa7);
            let (x, y) = (cqo9_d(l), cqo9_d(l + 6));
            cqo9_w(fase + 0x3e) == 2 && cqo9_d(fase + 4) == cqo9_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqo9_d(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("cqo9_final_do_carioca({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, cqo9_d(this + 0x30), cqo9_d(this + 0x2c)));
}

/// Comparador do fim da temporada: só a original (ponytail: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_9 historico da final" e "cqo_9 premios") e é provada pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqo9_fim_da_temporada_do_carioca(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1907)(this)
}

// ---------- 279a leva: o calendário das copas italianas: as rodadas da Copa da Itália, da Copa da Itália da Série C e das duas Supercopas (ita_cup.cpp, ita_c_cup.cpp, ita_super.cpp, ita_c1_super.cpp) ----------

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
type Cqo10G = (u32, u32, u32, u32);

/// As duas fases das copas italianas (a virtual +0x3c, `create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret
/// 0x10; a fase lida como byte com sinal). Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1, rodadas
/// = o número de rodadas de grupo, extra = fase + `id` (16 bits); a tabela de 0x41 bytes por rodada (0x944e46; sem
/// memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0, rodadas =
/// o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as rodadas pela
/// `cql10_rodada` com o ano +0x40 da copa (com sinal). Acima de `limite`: devolve 0 sem gravar nada.
unsafe fn cqo10_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, id: i16, grupos: &[Cqo10G],
    mata: &[Cql10R], arquivo: usize, linha_g: u32, linha_m: u32, nome_g: &'static str, nome: &'static str) -> usize {
    let f = fase as u8 as i8;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if f < limite {
        let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
        let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
        if !liga.is_null() {
            *liga = 1;
        }
        *rodadas = grupos.len() as i16;
        *extra = (f as i16).wrapping_add(id);
        let t = alocar(grupos.len() as u32 * 0x41);
        if t == 0 {
            aviso_em(arquivo, linha_g);
            return 0;
        }
        crate::comparacao::ramo(nome_g);
        // o ano vai com lixo nos bytes altos na original (o registrador de antes com a palavra +0x40 por cima); a
        // 0x68a160 só usa os 16 bits de baixo: a cópia passa o ano limpo
        let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
            data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, ano, 0);
            jogo(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limite {
        return 0;
    }
    if !liga.is_null() {
        *liga = 0;
    }
    *rodadas = mata.len() as i16;
    *extra = 0;
    let t = alocar(mata.len() as u32 * 0x68);
    if t == 0 {
        aviso_em(arquivo, linha_m);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in mata.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
unsafe fn cqo10_gravados_grupos(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (de grupos ou de mata-mata, pelo é_liga
/// da cópia) têm de ser iguais; a tabela da cópia é liberada (0x9452ca).
unsafe fn cqo10_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize, limite: i8) -> usize {
    if fase as u8 as i8 > limite {
        // ponytail: acima do mata-mata nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let grupos = (fase as u8 as i8) < limite;
    let tabela = if a != 0 && b != 0 {
        if grupos { cqo10_gravados_grupos(a, *rodadas) == cqo10_gravados_grupos(b, r2) } else { cql10_gravados(a, *rodadas) == cql10_gravados(b, r2) }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Itália, grupos: três rodadas em agosto (12 e 19, sábado [inferência: dia da semana 6], e 29, terça [2]).
const CQO10_COPA_DA_ITALIA_GRUPOS: [Cqo10G; 3] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x1d, 7, 2, 2)];

/// A Copa da Itália, mata-mata: cinco rodadas em ida e volta, de setembro a março do ano seguinte.
const CQO10_COPA_DA_ITALIA: [Cql10R; 5] = [
    cql10r((0x1e, 7, 0, 3), (0x13, 8, 0, 2, 2, 0), 0xa, (0, 0x400, 3), 4, (0x10, 8, 0x10, 0), (0, 2, 0x23), (0, 0xffff_ffff, 0)), // cqo10: a 1ª rodada do mata-mata da Copa da Itália
    cql10r((0x19, 9, 0, 3), (0xb, 0xa, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 4, (0x10, 8, 8, 0x10), (0, 2, 0x11), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 0xa, 0, 3), (0xc, 0xb, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0x1c), (0, 0xffff_ffff, 0)),
    cql10r((0xa, 0, 1, 3), (0x17, 0, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((7, 1, 1, 3), (6, 2, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 0x39), (0, 0xffff_ffff, 0)),
];

/// { 0x63ec50 } (thiscall, ret 0x10, ita_cup.cpp; a virtual +0x3c da Copa da Itália): `ItaCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (extra = fase + 0x3e9; a caixa da linha 0xc1), o mata-mata na
/// fase 7 (a caixa da linha 0xe1).
unsafe extern "thiscall" fn cqo10_copa_da_italia_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo10_tabela(this, fase, rodadas, extra, liga, 7, 0x3e9, &CQO10_COPA_DA_ITALIA_GRUPOS, &CQO10_COPA_DA_ITALIA, 0x9ea23c, 0xc1, 0xe1,
        "cqo_10 grupos da copa da italia", "cqo_10 mata-mata da copa da italia")
}

unsafe extern "thiscall" fn comparar_cqo10_copa_da_italia_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1908);
    cqo10_comparar("cqo10_copa_da_italia_tabela", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo10_copa_da_italia_tabela(this, fase, r, e, l), 7)
}

/// A Copa da Itália da Série C, grupos: cinco rodadas em agosto (12, 19, 22, 26 e 29).
const CQO10_COPA_C_GRUPOS: [Cqo10G; 5] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x16, 7, 2, 2), (0x1a, 7, 6, 1), (0x1d, 7, 2, 2)]; // cqo10: os grupos da Copa da Série C

/// A Copa da Itália da Série C, mata-mata: cinco rodadas em ida e volta, de setembro a maio do ano seguinte (as
/// semifinais em abril).
const CQO10_COPA_C: [Cql10R; 5] = [
    cql10r((0x1e, 7, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x14, (0, 0, 3), 4, (0x20, 0x10, 0x20, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 9, 0, 3), (1, 0xa, 0, 2, 2, 0), 0x1e, (0, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0xf, 0xa, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0xa, 0, 3), (0xb, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
];

/// { 0x63cf90 } (thiscall, ret 0x10, ita_c_cup.cpp; a virtual +0x3c da Copa da Série C): `ItaCCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 0xf (extra = fase + 0x420; a caixa da linha 0xbc), o mata-mata na
/// fase 0xf (a caixa da linha 0xe3).
unsafe extern "thiscall" fn cqo10_copa_c_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo10_tabela(this, fase, rodadas, extra, liga, 0xf, 0x420, &CQO10_COPA_C_GRUPOS, &CQO10_COPA_C, 0x9ea204, 0xbc, 0xe3,
        "cqo_10 grupos da copa da serie c", "cqo_10 mata-mata da copa da serie c")
}

unsafe extern "thiscall" fn comparar_cqo10_copa_c_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1909);
    cqo10_comparar("cqo10_copa_c_tabela", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo10_copa_c_tabela(this, fase, r, e, l), 0xf)
}

/// A Supercopa da Itália: um jogo único em 19 de agosto (sorteio em 5 de julho), mando neutro [inferência: o +0x13 da
/// data fica 0], sem prêmios.
const CQO10_SUPERCOPA: [Cql10R; 1] = [
    cql10r((5, 6, 0, 5), (0x13, 7, 0, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqo10: a Supercopa da Itália
];

/// { 0x65f920 } (thiscall, ret 0x10, ita_super.cpp; a virtual +0x3c da Supercopa da Itália): `ItaSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
unsafe extern "thiscall" fn cqo10_supercopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQO10_SUPERCOPA, 0x9eac34, 0x99, "cqo_10 tabela da supercopa")
}

unsafe extern "thiscall" fn comparar_cqo10_supercopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1910);
    cql10_comparar("cqo10_supercopa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqo10_supercopa_tabela(this, fase, r, e, s))
}

/// A Supercopa da Série C1: um jogo único em 21 de maio do ano seguinte (sorteio em 15 de maio), sem prêmios.
const CQO10_SUPERCOPA_C1: [Cql10R; 1] = [
    cql10r((0xf, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)),
];

/// { 0x63c890 } (thiscall, ret 0x10, ita_c1_super.cpp; a virtual +0x3c da Supercopa da Série C1): `ItaC1Super::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
unsafe extern "thiscall" fn cqo10_supercopa_c1_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQO10_SUPERCOPA_C1, 0x9ea1c8, 0x99, "cqo_10 tabela da supercopa c1")
}

unsafe extern "thiscall" fn comparar_cqo10_supercopa_c1_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1911);
    cql10_comparar("cqo10_supercopa_c1_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqo10_supercopa_c1_tabela(this, fase, r, e, s))
}

// ---------- 279a leva: os inscritos das copas italianas: a Copa da Itália, a Copa da Série C e os dois clubes da Supercopa (ita_cup.cpp, ita_c_cup.cpp, ita_super.cpp) ----------

unsafe fn cqo11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// Os clubes da tabela de clubes ([0xae23b4], 0x245 bytes) que a passada olha: [0xae235c] − 2 × [0xae2350] (sem as
/// seleções [inferência]).
unsafe fn cqo11_clubes() -> i32 {
    (cqo11_d(0xae235c) as i32).wrapping_sub((cqo11_d(0xae2350) as i32).wrapping_mul(2))
}

unsafe fn cqo11_clube(i: i32) -> usize {
    cqo11_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize)
}

/// O id da competição apontada por `c + o` (+0x57 a divisão, +0x5b a da temporada anterior [inferência]), ou None.
unsafe fn cqo11_comp(c: usize, o: usize) -> Option<usize> {
    let d = cqo11_d(c + o);
    if d == 0 { None } else { Some(cqo11_d(d)) }
}

/// O clube é da Itália (+0x53 a nação; [0x9cf360] o id da Itália).
unsafe fn cqo11_italiano(c: usize) -> bool {
    cqo11_comp(c, 0x53) == Some(cqo11_d(0x9cf360))
}

/// A posição na temporada anterior (+0x5f, byte com sinal; 0 o campeão [inferência]).
unsafe fn cqo11_posicao(c: usize) -> i8 {
    *((c + 0x5f) as *const i8)
}

/// O sorteio de 0 a n − 1 (0x90cfc0: push n; add esp, 4 -> cdecl).
unsafe fn cqo11_sortear(n: i32) -> i32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    sortear(n)
}

/// Embaralha `n` ponteiros a partir de `v` (0x4afb40: push n; push v; add esp junto -> cdecl; já em Rust).
unsafe fn cqo11_embaralhar(v: usize, n: u32) {
    let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    f(v, n);
}

/// O qsort de `n` ponteiros a partir de `v` com a comparação `cmp` (0x945333: 4 pushes; add esp, 0x10 -> cdecl).
unsafe fn cqo11_ordenar(v: usize, n: u32, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(v, n, 4, cmp);
}

/// A lista de ponteiros (+0xee) com o máximo `max` em +0xf2 (dword): 0x944e46 de 4 × max; sem memória, a caixa `linha`.
unsafe fn cqo11_lista(this: usize, max: u32, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, max);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(max * 4);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    Some(l)
}

/// Põe `c` no fim da lista (+0xee) e devolve verdadeiro quando a conta chega ao máximo +0xf2 (dword).
unsafe fn cqo11_por(this: usize, n: &mut i32, c: usize) -> bool {
    *n += 1;
    core::ptr::write_unaligned((cqo11_d(this + 0xee) + (*n as usize - 1) * 4) as *mut u32, c as u32);
    *n == cqo11_d(this + 0xf2) as i32
}

/// { 0x63faf0 } (thiscall, ret 0, ita_cup.cpp): `ItaCup::select_entrants()`: os 48 inscritos da Copa da Itália (+0xf2 =
/// 0x30; a lista de 0xc0 bytes em +0xee; sem memória, a caixa da linha 0x2e1 e 0). Os 32 dos grupos num vetor local:
/// os clubes italianos da temporada (passada única) que não vieram da Série B ([0x9cf574]) entre os 3 primeiros nem da
/// Série A ([0x9cf570]) entre os 15 primeiros: os da Série B atual, os que vieram da Série B em 4º, 5º ou abaixo do 17º,
/// e os que vieram da C1 ([0x9cf578]/[0x9cf57c]) em 4º, 5º ou 6º; faltando, os da C1 fora da Série B pela posição (6º,
/// 7º...). O vetor vai ao qsort (0x640590) e é repartido em 4 potes de 8: 8 voltas, cada uma tira um clube sorteado de
/// cada pote (0x90cfc0(8), seguindo para o próximo do pote se já saiu). Depois os 3 primeiros da Série B anterior e os
/// da Série A anterior do 10º ao 15º (até completar 48) e, sem conferir antes, os 9 primeiros da Série A anterior.
/// Devolve 1.
unsafe fn cqo11_copa_da_italia(this: usize) -> (i32, i32) {
    let l = match cqo11_lista(this, 0x30, 0x9ea23c, 0x2e1) {
        Some(l) => l,
        None => return (0, 0),
    };
    let (serie_a, serie_b, c1a, c1b) = (cqo11_d(0x9cf570), cqo11_d(0x9cf574), cqo11_d(0x9cf578), cqo11_d(0x9cf57c));
    let mut v = [0usize; 32];
    let mut k = 0usize;
    let n = cqo11_clubes();
    'grupos: {
        let mut i = 0;
        while i < n {
            let c = cqo11_clube(i);
            i += 1;
            if !cqo11_italiano(c) {
                continue;
            }
            let antes = cqo11_comp(c, 0x5b);
            if antes == Some(serie_b) && cqo11_posicao(c) < 3 || antes == Some(serie_a) && cqo11_posicao(c) < 0xf {
                continue;
            }
            if cqo11_comp(c, 0x57) == Some(serie_b) {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
                continue;
            }
            let p = cqo11_posicao(c);
            let entra = match antes {
                None => false,
                Some(x) if x == serie_b => p == 3 || p == 4 || p > 0x10,
                Some(x) => (x == c1a || x == c1b) && (p == 3 || p == 4 || p == 5),
            };
            if entra {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
            }
        }
        let mut lugar = 5i8;
        while k < 32 {
            let mut i = 0;
            while i < n {
                let c = cqo11_clube(i);
                i += 1;
                if !cqo11_italiano(c) {
                    continue;
                }
                let agora = cqo11_comp(c, 0x57);
                if agora.is_none() || agora == Some(serie_b) {
                    continue;
                }
                let antes = cqo11_comp(c, 0x5b);
                if (antes == Some(c1a) || antes == Some(c1b)) && cqo11_posicao(c) == lugar {
                    crate::comparacao::ramo("cqo_11 completar com a c1");
                    v[k] = c;
                    k += 1;
                    if k == 32 {
                        break;
                    }
                }
            }
            lugar = lugar.wrapping_add(1);
        }
    }
    cqo11_ordenar(v.as_mut_ptr() as usize, k as u32, 0x640590);
    let mut m = 0i32;
    for _ in 0..8 {
        let mut pote = 0usize;
        while pote < 32 {
            crate::comparacao::ramo("cqo_11 potes da copa da italia");
            let mut r = cqo11_sortear(8) as i8 as i32; // cqo11: o clube sorteado de cada pote
            while v[pote + r as usize] == 0 {
                r += 1;
                if r == 8 {
                    r = 0;
                }
            }
            m += 1;
            core::ptr::write_unaligned((l + (m as usize - 1) * 4) as *mut u32, v[pote + r as usize] as u32);
            v[pote + r as usize] = 0;
            pote += 8;
        }
    }
    'b: {
        let mut i = 0;
        while i < cqo11_clubes() {
            let c = cqo11_clube(i);
            i += 1;
            if !cqo11_italiano(c) {
                continue;
            }
            let antes = cqo11_comp(c, 0x5b);
            let p = cqo11_posicao(c);
            if antes == Some(serie_b) && p < 3 || antes == Some(serie_a) && p > 8 && p < 0xf { // cqo11: os que entram na 2ª rodada
                if cqo11_por(this, &mut m, c) {
                    break 'b;
                }
            }
        }
    }
    let mut i = 0;
    while i < cqo11_clubes() {
        let c = cqo11_clube(i);
        i += 1;
        if cqo11_italiano(c) && cqo11_comp(c, 0x5b) == Some(serie_a) && cqo11_posicao(c) < 9 {
            if cqo11_por(this, &mut m, c) {
                break;
            }
        }
    }
    (1, m)
}

unsafe extern "thiscall" fn cqo11_copa_da_italia_inscritos(this: usize) -> i32 {
    cqo11_copa_da_italia(this).0
}

/// Lado a lado para as rotinas que alocam a lista de ponteiros +0xee e sorteiam: o +0xf2, o [0xb67a34] e o estado do
/// sorteador a partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xee, a cópia faz
/// a sua, comparada nos inscritos que ela gravou e liberada.
unsafe fn cqo11_comparar(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqo11_d(this + 0xee));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(c);
        lr.set(cqo11_d(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    let ver = |l: usize| if l == 0 { Vec::new() } else { core::slice::from_raw_parts(l as *const u32, k / 4).to_vec() };
    let (vo, vr) = (ver(lo), ver(lr));
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} {:x?} x {:x?} lista {} ({} bytes) {:x?} x {:x?}", nome, this, a, b, igual, mo, mr, lista, k, vo, vr));
    a
}

unsafe extern "thiscall" fn comparar_cqo11_copa_da_italia_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1912);
    cqo11_comparar("cqo11_copa_da_italia_inscritos", this, || o(this), || cqo11_copa_da_italia(this))
}

/// { 0x63dfe0 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::select_entrants()`: os 90 inscritos da Copa da Série C (+0xf2
/// = 0x5a; a lista de 0x168 bytes em +0xee; sem memória, a caixa da linha 0x322 e 0): os clubes italianos da C1 e da C2
/// ([0x9cf578]..[0x9cf588]) pela ordem da tabela, até 90. Dos 80 primeiros, os que vieram da Série B ([0x9cf574]) e os
/// que vieram da C1 em 4º, 5º ou 6º vão, um a um, para as vagas 80, 81... (trocados com o que estava lá). Os 80 primeiros
/// são embaralhados (0x4afb40), ordenados pela longitude (qsort com 0x53fe50) e cada uma das 8 dezenas deles
/// embaralhada (os grupos por região [inferência]); as 10 últimas vagas ficam como estão. Devolve 1.
unsafe fn cqo11_copa_c(this: usize) -> (i32, i32) {
    let l = match cqo11_lista(this, 0x5a, 0x9ea204, 0x322) {
        Some(l) => l,
        None => return (0, 0),
    };
    let divisoes = [cqo11_d(0x9cf578), cqo11_d(0x9cf57c), cqo11_d(0x9cf580), cqo11_d(0x9cf584), cqo11_d(0x9cf588)];
    let mut m = 0i32;
    let mut i = 0;
    while i < cqo11_clubes() {
        let c = cqo11_clube(i);
        i += 1;
        if !cqo11_italiano(c) {
            continue;
        }
        match cqo11_comp(c, 0x57) {
            Some(d) if divisoes.contains(&d) => {
                if cqo11_por(this, &mut m, c) {
                    break;
                }
            }
            _ => {}
        }
    }
    let (serie_b, c1a, c1b) = (cqo11_d(0x9cf574), cqo11_d(0x9cf578), cqo11_d(0x9cf57c));
    let mut j = 0x50i32;
    for idx in 0..0x50usize {
        let c = cqo11_d(l + idx * 4);
        let antes = match cqo11_comp(c, 0x5b) {
            Some(x) => x,
            None => continue,
        };
        let p = cqo11_posicao(c);
        if antes == serie_b || (antes == c1a || antes == c1b) && (p == 3 || p == 4 || p == 5) {
            crate::comparacao::ramo("cqo_11 vagas do fim da copa da serie c");
            let e = l + (j as i16 as isize * 4) as usize;
            let t = cqo11_d(e);
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            j += 1;
            core::ptr::write_unaligned((l + idx * 4) as *mut u32, t as u32);
            if j as i16 as i32 >= cqo11_d(this + 0xf2) as i32 {
                break;
            }
        }
    }
    cqo11_embaralhar(l, 0x50);
    cqo11_ordenar(l, 0x50, 0x53fe50);
    crate::comparacao::ramo("cqo_11 dezenas da copa da serie c");
    for k in 0..8usize {
        cqo11_embaralhar(l + k * 0x28, 0xa); // cqo11: as dezenas da Copa da Série C
    }
    (1, m)
}

unsafe extern "thiscall" fn cqo11_copa_c_inscritos(this: usize) -> i32 {
    cqo11_copa_c(this).0
}

unsafe extern "thiscall" fn comparar_cqo11_copa_c_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1913);
    cqo11_comparar("cqo11_copa_c_inscritos", this, || o(this), || cqo11_copa_c(this))
}

/// { 0x65fa60 } (thiscall, ret 0, ita_super.cpp): `ItaSuper::init_team_list()`: a lista de dois (+0x3e = 2, 0xc bytes
/// em +0xa7; sem memória, a caixa da linha 0xc2 e −1). O campeão da Série A ([0x9cf570]; sem ele, a caixa 0xcb e 0) e o
/// da Copa da Itália ([0x9cf71c]; sem ele, 0xd4 e 0); se é o mesmo clube, o segundo vira o vice da Copa (sem ele, 0xe0
/// e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn cqo11_supercopa(this: usize) -> (i32, i32) {
    const F: usize = 0x9ea934;
    let l = match cqm12_lista(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf570, 0); // cqo11: o campeão da Série A
    if !cqm12_vaga(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf71c, 0);
    if !cqm12_vaga(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqo_11 dobradinha italiana");
        if !cqm12_vaga(l, 1, cqm12_colocado(0x9cf71c, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqo_11 clubes da supercopa da italia");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqo11_supercopa_clubes(this: usize) -> i32 {
    cqo11_supercopa(this).0
}

unsafe extern "thiscall" fn comparar_cqo11_supercopa_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1914);
    cqm12_comparar("cqo11_supercopa_clubes", this, || o(this), || cqo11_supercopa(this))
}

// ---------- 279a leva: a Copa do Brasil: as rodadas, a tabela dos times e as regras dos jogos (bra_cup.cpp) ----------

/// A Copa do Brasil: sete rodadas em ida e volta de fevereiro a julho do ano da copa (o jogo de volta 14 dias depois do
/// de ida até as oitavas, 21 nas quartas e 7 na semifinal; 4 dias nas três últimas, ao meio da semana [inferência]).
const CQO12_COPA_DO_BRASIL: [Cql10R; 7] = [
    cql10r((0x12, 0, 0, 6), (0x17, 1, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x36, 0x1b, 0x36, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo12: a 1ª rodada da Copa do Brasil
    cql10r((9, 2, 0, 3), (0x16, 2, 0, 2, 2, 0), 0x14, (0, 0, 1), 4, (0x22, 0x11, 7, 0x36), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((6, 3, 0, 3), (0x1a, 3, 0, 2, 2, 0), 0x1e, (0, 0, 1), 6, (0x16, 0xb, 5, 0x3d), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
    cql10r((0x12, 4, 0, 3), (7, 5, 0, 2, 2, 0), 0x28, (0, 0, 1), 6, (0x10, 8, 5, 0x42), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xf, 5, 0, 3), (0x11, 5, 0, 5, 1, 0), 0x78, (0, 0, 1), 6, (8, 4, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x16, 5, 0, 3), (0x18, 5, 0, 5, 1, 0), 0x82, (0, 0, 1), 6, (4, 2, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 5, 0, 3), (1, 6, 0, 5, 1, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 4), (0, 0xf4240, 0xc3500)),
];

/// { 0x427700 } (thiscall, ret 0x10, bra_cup.cpp; a virtual +0x3c da Copa do Brasil): `BraCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa4 sem memória.
unsafe extern "thiscall" fn cqo12_copa_do_brasil_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQO12_COPA_DO_BRASIL, 0x98a3bc, 0xa4, "cqo_12 tabela da copa do brasil")
}

unsafe extern "thiscall" fn comparar_cqo12_copa_do_brasil_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1915);
    cql10_comparar("cqo12_copa_do_brasil_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqo12_copa_do_brasil_tabela(this, fase, r, e, s))
}

unsafe fn cqo12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x427c20 } (thiscall, ret 0, bra_cup.cpp): `BraCup::build_team_table()`: +0x3e = 71 e a tabela dos times (0x944e46
/// de 0x1aa em +0xa7; sem memória, a caixa da linha 0x143 e −1); cada vaga i (6 bytes) recebe o clube da inscrição i da
/// lista +0x14 (6 bytes), a situação +5 = 0xff e a classe +4 = 0. Devolve 1.
unsafe fn cqo12_times(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x47);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x1aa);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x98a3bc, 0x143);
        return -1;
    }
    crate::comparacao::ramo("cqo_12 times da copa do brasil");
    let mut i = 0i16;
    while i < cqo12_w(this + 0x3e) {
        let o = i as usize * 6;
        let fonte = o; // cqo12: a inscrição de cada vaga
        let c = cqo12_d(cqo12_d(this + 0x14) + fonte);
        let e = cqo12_d(this + 0xa7) + o;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 5) as *mut u8) = 0xff;
        *((e + 4) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

unsafe extern "thiscall" fn cqo12_times_da_copa(this: usize) -> i32 {
    cqo12_times(this)
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 71 vagas (com a devolução 1) e liberada.
unsafe extern "thiscall" fn comparar_cqo12_times_da_copa(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1916);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqo12_d(this + 0xa7));
        r
    }, || {
        let r = cqo12_times(this);
        lr.set(cqo12_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { cqo12_w(this + 0x3e).max(0) as usize * 6 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqo12_times_da_copa({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x427ec0 } (thiscall, ret 0, bra_cup.cpp; uma virtual da copa): `BraCup::set_up_rounds()`: 7 reservas (+0x49), 3
/// substituições (+0x4a), o fundo de prêmios de 500 mil (+0x28) e a tabela das rodadas (a virtual +0x3c com a fase −1,
/// as rodadas em +0x3c e o extra em +0x3a, sem saída) em +0xa3. Devolve 1.
unsafe fn cqo12_regras(this: usize) -> i32 {
    crate::comparacao::ramo("cqo_12 regras da copa do brasil");
    *((this + 0x49) as *mut u8) = 7;
    *((this + 0x4a) as *mut u8) = 3; // cqo12: as substituições da Copa do Brasil
    core::ptr::write_unaligned((this + 0x28) as *mut u32, 0x7a120);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(cqo12_d(cqo12_d(this) + 0x3c));
    let t = tabela(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xa3) as *mut u32, t as u32);
    1
}

unsafe extern "thiscall" fn cqo12_regras_da_copa(this: usize) -> i32 {
    cqo12_regras(this)
}

/// Lado a lado: +0x28, +0x3a..+0x3d, +0x49/+0x4a e o [0xb67a34] a partir do mesmo estado; as tabelas das rodadas das
/// duas (+0xa3) comparadas nos bytes gravados (cql10_gravados) e a da cópia liberada.
unsafe extern "thiscall" fn comparar_cqo12_regras_da_copa(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1917);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let antes = cqo12_d(this + 0xa3);
    let reg = [(this + 0x28, 4usize), (this + 0x3a, 4), (this + 0x49, 2), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        to.set(cqo12_d(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, antes as u32);
        r
    }, || {
        let r = cqo12_regras(this);
        tr.set(cqo12_d(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = cqo12_w(this + 0x3c);
    let tabela = if to != 0 && tr != 0 { cql10_gravados(to, n) == cql10_gravados(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(tr);
    }
    PLACAR.registrar(igual && a == b && tabela, || format!("cqo12_regras_da_copa({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}

// ---------- 279a leva: as copas sul-americanas de clubes: o calendário da Libertadores e da Mercosul e os convidados da Mercosul (conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
type Cqo13G = (u32, u32, u32, u32);

/// As duas fases da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10; a fase lida
/// como byte com sinal) das copas sul-americanas. Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1,
/// rodadas = o número de rodadas de grupo, extra = fase + 0x3e9 (16 bits); a tabela de 0x41 bytes por rodada (0x944e46;
/// sem memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0,
/// rodadas = o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as
/// rodadas pela `cql10_rodada` com o ano +0x40 da copa (com sinal) e as palavras de `a_mais` (rodada, deslocamento,
/// valor). Acima de `limite`: devolve 0 sem gravar nada.
unsafe fn cqo13_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, grupos: &[Cqo13G],
    mata: &[Cql10R], a_mais: &[(usize, usize, u16)], arquivo: usize, linha_g: u32, linha_m: u32, nome_g: &'static str, nome: &'static str) -> usize {
    let f = fase as u8 as i8;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if f < limite {
        let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
        let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
        if !liga.is_null() {
            *liga = 1;
        }
        *rodadas = grupos.len() as i16;
        *extra = (f as i16).wrapping_add(0x3e9);
        let t = alocar(grupos.len() as u32 * 0x41);
        if t == 0 {
            aviso_em(arquivo, linha_g);
            return 0;
        }
        crate::comparacao::ramo(nome_g);
        // o ano vai com lixo nos bytes altos na original (o registrador de antes com a palavra +0x40 por cima); a
        // 0x68a160 só usa os 16 bits de baixo: a cópia passa o ano limpo
        let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
            data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, ano, 0);
            jogo(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limite {
        return 0;
    }
    if !liga.is_null() {
        *liga = 0;
    }
    *rodadas = mata.len() as i16;
    *extra = 0;
    let t = alocar(mata.len() as u32 * 0x68);
    if t == 0 {
        aviso_em(arquivo, linha_m);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in mata.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    for &(r, o, v) in a_mais {
        core::ptr::write_unaligned((t + r * 0x68 + o) as *mut u16, v);
    }
    t
}

/// Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
unsafe fn cqo13_gravados_grupos(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (grupos ou mata-mata, este com as
/// palavras de `a_mais`) têm de ser iguais; a tabela da cópia é liberada (0x9452ca). Acima do mata-mata só a original.
unsafe fn cqo13_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, a_mais: &[(usize, usize, u16)],
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = fase as u8 as i8;
    if f > limite {
        // ponytail: acima do mata-mata nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, f, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let mais = |t: usize| a_mais.iter().flat_map(|&(r, o, _)| core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2).to_vec()).collect::<Vec<u8>>();
    let tabela = if a != 0 && b != 0 {
        if f < limite {
            cqo13_gravados_grupos(a, *rodadas) == cqo13_gravados_grupos(b, r2)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, r2) && mais(a) == mais(b)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, f, saidas, *rodadas, r2, tabela));
    a
}

/// A Libertadores, grupos: seis rodadas às terças [inferência: dia da semana 2], de 25 de fevereiro a 1º de abril.
const CQO13_LIBERTADORES_GRUPOS: [Cqo13G; 6] = [(0x19, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // cqo13: os grupos da Libertadores

/// A Libertadores, mata-mata: quatro rodadas em ida e volta, de abril a junho (o jogo de volta 14 dias depois nas
/// oitavas e 7 nas outras).
const CQO13_LIBERTADORES: [Cql10R; 4] = [
    cql10r((2, 3, 0, 3), (0xf, 3, 0, 2, 2, 0), 0x14, (0, 4, 1), 7, (0x10, 8, 0x10, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo13: as oitavas da Libertadores
    cql10r((0x1e, 3, 0, 3), (0x14, 4, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x1c, 4, 0, 3), (3, 5, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 5, 0, 3), (0x11, 5, 0, 2, 2, 0), 0x96, (0, 4, 1), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0xc3500)),
];

/// { 0x4c31c0 } (thiscall, ret 0x10, conmebol_liber.cpp; a virtual +0x3c da Libertadores): `ConmebolLiber::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (a caixa da linha 0x9f), o mata-mata na
/// fase 7 (a caixa da linha 0xc5).
unsafe extern "thiscall" fn cqo13_libertadores_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo13_tabela(this, fase, rodadas, extra, liga, 7, &CQO13_LIBERTADORES_GRUPOS, &CQO13_LIBERTADORES, &[], 0x9a7de4, 0x9f, 0xc5,
        "cqo_13 grupos da libertadores", "cqo_13 mata-mata da libertadores")
}

unsafe extern "thiscall" fn comparar_cqo13_libertadores_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1918);
    cqo13_comparar("cqo13_libertadores_tabela", fase, rodadas, extra, liga, 7, &[], || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo13_libertadores_tabela(this, fase, r, e, l))
}

/// A Mercosul, grupos: seis rodadas às terças, de 29 de julho a 14 de outubro.
const CQO13_MERCOSUL_GRUPOS: [Cqo13G; 6] = [(0x1d, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // cqo13: os grupos da Mercosul

/// A Mercosul, mata-mata: quartas, semifinal e final em ida e volta, de outubro a dezembro (a volta 3 dias depois nas
/// quartas e na semifinal, 7 na final).
const CQO13_MERCOSUL: [Cql10R; 3] = [
    cql10r((0xf, 9, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 8, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((5, 0xa, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x96, (0, 0x2000, 0x2000), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0)),
];

/// A final da Mercosul grava ainda a palavra +0x11 = 3 [inferência: o desempate da final, pênaltis].
const CQO13_MERCOSUL_A_MAIS: [(usize, usize, u16); 1] = [(2, 0x11, 3)];

/// { 0x4c4930 } (thiscall, ret 0x10, conmebol_merc.cpp; a virtual +0x3c da Mercosul): `ConmebolMerc::create_schedule(
/// fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 4 (a caixa da linha 0x97), o mata-mata na fase 4 (a caixa
/// da linha 0xbd).
unsafe extern "thiscall" fn cqo13_mercosul_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo13_tabela(this, fase, rodadas, extra, liga, 4, &CQO13_MERCOSUL_GRUPOS, &CQO13_MERCOSUL, &CQO13_MERCOSUL_A_MAIS, 0x9a7e5c, 0x97, 0xbd,
        "cqo_13 grupos da mercosul", "cqo_13 mata-mata da mercosul")
}

unsafe extern "thiscall" fn comparar_cqo13_mercosul_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1919);
    cqo13_comparar("cqo13_mercosul_tabela", fase, rodadas, extra, liga, 4, &CQO13_MERCOSUL_A_MAIS, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo13_mercosul_tabela(this, fase, r, e, l))
}

unsafe fn cqo13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// { 0x4c5850 } (thiscall, ret 0, conmebol_merc.cpp): `ConmebolMerc::select_invited_clubs()`: +0xf2 = 20 e a lista de
/// ponteiros (0x944e46 de 0x50 em +0xee; sem memória, a caixa da linha 0x2c6 e 0); cada clube da tabela ([0xae23b4],
/// [0xae235c] − 2 × [0xae2350]) que as regras ([0xae1000], 0x4c6890: thiscall com o clube) convidam entra na lista, sem
/// conferir o espaço; a lista é embaralhada (0x4afb40 com +0xf2). Devolve 1.
unsafe fn cqo13_convidados(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x14);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x50);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9a7e5c, 0x2c6);
        return (0, 0);
    }
    let convidado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4c6890usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    let clubes = || (cqo13_d(0xae235c) as i32).wrapping_sub((cqo13_d(0xae2350) as i32).wrapping_mul(2));
    let mut k = 0i16;
    let mut i = 0i32;
    while i < clubes() {
        let c = cqo13_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize);
        if convidado(cqo13_d(0xae1000), c) != 0 {
            core::ptr::write_unaligned((cqo13_d(this + 0xee) + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    crate::comparacao::ramo("cqo_13 convidados da mercosul");
    // ponytail: a original empurra a palavra +0xf2 com lixo nos 16 bits altos (o eax de antes); a 0x4afb40 lê a palavra
    embaralhar(cqo13_d(this + 0xee), core::ptr::read_unaligned((this + 0xf2) as *const u16) as u32); // cqo13: embaralha os convidados
    (1, k as i32)
}

unsafe extern "thiscall" fn cqo13_mercosul_convidados(this: usize) -> i32 {
    cqo13_convidados(this).0
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nos convidados que ela gravou e liberada.
unsafe extern "thiscall" fn comparar_cqo13_mercosul_convidados(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1920);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqo13_d(this + 0xee));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(|| cqo13_convidados(this));
        lr.set(cqo13_d(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqo13_mercosul_convidados({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", this, a, b, igual, lista, k));
    a
}

// ---------- 279a leva: o calendário da segunda e da terceira divisões inglesas: as 46 rodadas com o Natal conforme o dia da semana e os play-offs de acesso (eng_second.cpp, eng_third.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqp4Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqp4_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqp4Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
unsafe fn cqp4_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
struct Cqp4Natal {
    comeco: &'static [Cqp4Rodada],
    referencia: usize,
    ajuste: i32,
    casos: [&'static [Cqp4Rodada]; 8],
    fim: &'static [Cqp4Rodada],
    linha: u32,
}

unsafe fn cqp4_natal(t: usize, ano: u32, n: &Cqp4Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    cqp4_rodadas(t, ano, 0, n.comeco);
    let mut d = [0u16; 4];
    construir(&mut d, 1, 0xb, ano, 0xffff_ffff);
    let dia = core::ptr::read_unaligned((t + n.referencia * 0x41) as *const u16);
    let k = (dia.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.ajuste);
    let caso = if k as u32 > 6 {
        aviso_em(arquivo, n.linha);
        7
    } else {
        k as usize
    };
    let meio = n.casos[caso];
    cqp4_rodadas(t, ano, n.comeco.len() as i32, meio);
    cqp4_rodadas(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqp4_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqp4_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqp4_gravados(a, a, n) == cqp4_gravados(b, a, n)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, *rodadas)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a segunda e a terceira divisões inglesas (as mesmas datas): as rodadas antes do Natal
const CQP4_SEGUNDA_COMECO: [Cqp4Rodada; 23] = [
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 12/ago
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 19/ago
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 26/ago
    (28, 7, 0, 0, 2, &[(-1, -1, -1)]), // 3: 28/ago
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 2/set
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 9/set
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 6: 13/set
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 8: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30/set
    (7, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 7/out
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 14/out
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 12: 17/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 13: 21/out
    (24, 9, 0, 1, 2, &[(-1, -1, -1)]), // 14: 24/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 15: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 17: 11/nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 18: 18/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 19: 25/nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 20: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 21: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 22: 16/dez
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
const CQP4_SEGUNDA_NATAL_0: [Cqp4Rodada; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
const CQP4_SEGUNDA_NATAL_1: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
const CQP4_SEGUNDA_NATAL_2: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
const CQP4_SEGUNDA_NATAL_3: [Cqp4Rodada; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
const CQP4_SEGUNDA_NATAL_4: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
const CQP4_SEGUNDA_NATAL_5: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
const CQP4_SEGUNDA_NATAL_6: [Cqp4Rodada; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
const CQP4_SEGUNDA_NATAL_PADRAO: [Cqp4Rodada; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// a segunda e a terceira divisões inglesas (as mesmas datas): as rodadas depois do Natal
const CQP4_SEGUNDA_FIM: [Cqp4Rodada; 19] = [
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 27: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 28: 20/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 29: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 30: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 31: 17/fev (+1 ano)
    (20, 1, 1, 1, 2, &[(-1, -1, -1)]), // 32: 20/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 33: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 34: 3/mar (+1 ano)
    (7, 2, 1, 2, 2, &[(-1, -1, -1)]), // 35: 7/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 36: 10/mar (+1 ano)
    (17, 2, 1, 1, 2, &[(-1, -1, -1)]), // 37: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 38: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 39: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(-1, -1, -1)]), // 40: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 41: 14/abr (+1 ano)
    (16, 3, 1, 0, 2, &[(-1, -1, -1)]), // 42: 16/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 43: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 44: 28/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 45: 6/mai (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
const CQP4_TERCEIRA_NATAL_0: [Cqp4Rodada; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
const CQP4_TERCEIRA_NATAL_1: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
const CQP4_TERCEIRA_NATAL_2: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
const CQP4_TERCEIRA_NATAL_3: [Cqp4Rodada; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
const CQP4_TERCEIRA_NATAL_4: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
const CQP4_TERCEIRA_NATAL_5: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
const CQP4_TERCEIRA_NATAL_6: [Cqp4Rodada; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
const CQP4_TERCEIRA_NATAL_PADRAO: [Cqp4Rodada; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

const CQP4_SEGUNDA_NATAL: Cqp4Natal = Cqp4Natal { comeco: &CQP4_SEGUNDA_COMECO, referencia: 22, ajuste: 13, casos: [&CQP4_SEGUNDA_NATAL_0, &CQP4_SEGUNDA_NATAL_1, &CQP4_SEGUNDA_NATAL_2, &CQP4_SEGUNDA_NATAL_3, &CQP4_SEGUNDA_NATAL_4, &CQP4_SEGUNDA_NATAL_5, &CQP4_SEGUNDA_NATAL_6, &CQP4_SEGUNDA_NATAL_PADRAO], fim: &CQP4_SEGUNDA_FIM, linha: 0x15c };
const CQP4_SEGUNDA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
const CQP4_SEGUNDA_PLAYOFF_1: Cql10R = cql10r((0x11, 4, 1, 3), (0x1b, 4, 1, 6, 1, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));
const CQP4_TERCEIRA_NATAL: Cqp4Natal = Cqp4Natal { comeco: &CQP4_SEGUNDA_COMECO, referencia: 22, ajuste: 13, casos: [&CQP4_TERCEIRA_NATAL_0, &CQP4_TERCEIRA_NATAL_1, &CQP4_TERCEIRA_NATAL_2, &CQP4_TERCEIRA_NATAL_3, &CQP4_TERCEIRA_NATAL_4, &CQP4_TERCEIRA_NATAL_5, &CQP4_TERCEIRA_NATAL_6, &CQP4_TERCEIRA_NATAL_PADRAO], fim: &CQP4_SEGUNDA_FIM, linha: 0x161 };
const CQP4_TERCEIRA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
const CQP4_TERCEIRA_PLAYOFF_1: Cql10R = cql10r((0x11, 4, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));

/// { 0x5757a0 } (thiscall, ret 0x10, eng_second.cpp, 3.692 bytes, método +0x3c): `EngSecond::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase −1, 46 rodadas (`CQP4_SEGUNDA_NATAL`); a fase 0, os play-offs (semifinais e
/// final); as outras devolvem 0. Fonte 0x9c43e8.
unsafe extern "thiscall" fn calendario_da_segunda_inglesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 46;
            *extra = 0;
            let t = alocar(0xbae);
            if t == 0 {
                aviso_em(0x9c43e8, 0xae);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 calendario da segunda inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            cqp4_natal(t, ano, &CQP4_SEGUNDA_NATAL, 0x9c43e8);
            t
        }
        0 => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 2;
            *extra = 0xa0;
            let t = alocar(0xd0);
            if t == 0 {
                aviso_em(0x9c43e8, 0x1b5);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 play-offs da segunda inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            cql10_rodada(t, 0, &CQP4_SEGUNDA_PLAYOFF_0, ano);
            cql10_rodada(t, 1, &CQP4_SEGUNDA_PLAYOFF_1, ano);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_calendario_da_segunda_inglesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1921);
    cqp4_comparar("calendario_da_segunda_inglesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_segunda_inglesa(this, fase, r, e, l))
}

/// { 0x5770e0 } (thiscall, ret 0x10, eng_third.cpp, 3.692 bytes, método +0x3c): `EngThird::create_schedule`: o mesmo
/// código da segunda (as mesmas rodadas), com a data da final dos play-offs um dia antes. Fonte 0x9c4424.
unsafe extern "thiscall" fn calendario_da_terceira_inglesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 46;
            *extra = 0;
            let t = alocar(0xbae);
            if t == 0 {
                aviso_em(0x9c4424, 0xb3);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 calendario da terceira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            cqp4_natal(t, ano, &CQP4_TERCEIRA_NATAL, 0x9c4424);
            t
        }
        0 => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 2;
            *extra = 0xa0;
            let t = alocar(0xd0);
            if t == 0 {
                aviso_em(0x9c4424, 0x1ba);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 play-offs da terceira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            cql10_rodada(t, 0, &CQP4_TERCEIRA_PLAYOFF_0, ano);
            cql10_rodada(t, 1, &CQP4_TERCEIRA_PLAYOFF_1, ano);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_calendario_da_terceira_inglesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1922);
    cqp4_comparar("calendario_da_terceira_inglesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_terceira_inglesa(this, fase, r, e, l))
}
