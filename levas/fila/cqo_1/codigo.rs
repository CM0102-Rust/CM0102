// ---------- %%LEVA%%a leva: a escolha do estádio de um jogo: o estádio da partida e a busca do estádio neutro (stadium.cpp) ----------

unsafe fn cqo1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A nação `id` (registro de 0x122 bytes em [0xae23a8]).
unsafe fn nation_by_id(id: usize) -> usize {
    cqo1_d(0xae23a8).wrapping_add(id.wrapping_mul(0x122))
}

/// O clube é uma seleção? (0x53b900, cdecl, já em Rust.)
unsafe fn cqo1_selecao(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(clube) != 0
}

/// `Stadium::add_booking` (0x85de20, thiscall, ret 8, já em Rust): a reserva do estádio na data.
unsafe fn add_stadium_booking(this: usize, estadio: usize, data: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85de20usize);
    f(this, estadio, data)
}

/// `Stadium::is_stadium_free_on_date` (0x85ddb0, thiscall, ret 8, já em Rust).
unsafe fn is_stadium_free(this: usize, estadio: usize, data: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x85ddb0usize);
    f(this, estadio, data) != 0
}

/// Os comparadores das listas (cdecl, (a, b) -> int): 0x85e2f0 pela nação (+0), 0x85e320 pelo continente (+4), os
/// dois desempatando pelos lugares do estádio (+0xc → +0x3c, quando os dois têm estádio); 0x85e350 só pelos lugares.
unsafe fn call_stadium_compare(f: usize, a: usize, b: usize) -> i32 {
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
unsafe extern "thiscall" fn find_available_venue(this: usize, marcas: u32, cont: usize, nacao: usize, data: usize, casa: usize, fora: usize, partida: usize) -> usize {
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
                if call_stadium_compare(cmp, e, k) != 0 {
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
                if (nacao != 0 || cont != 0) && call_stadium_compare(cmp, e, k) != 0 {
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
                    if !is_stadium_free(this, cqo1_d(e + 0xc), data) {
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
                    if vez && ((partida != 0 && cqo1_d(partida) == cqo1_d(0x9cf76c)) || is_stadium_free(this, cqo1_d(e + 0xc), data)) {
                        crate::comparison::branch("cqo_1 rotation");
                        add_stadium_booking(this, cqo1_d(e + 0xc), data);
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
            crate::comparison::branch("cqo_1 neutral by rating");
            if partida != 0 {
                add_stadium_booking(this, cqo1_d(escolhida + 0xc), data);
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
unsafe fn stadium_choice_regions(this: usize, extra: Option<(usize, usize)>) -> Option<([(usize, usize); 6], usize)> {
    let nr = cqo1_d(this + 4);
    if cqo1_d(this) == 0 || nr == cqo1_d(this + 8) {
        return None;
    }
    let mut r = [(0usize, 0usize); 6];
    let mut m = 0;
    // a nota do estádio neutro (0x85da00) sorteia: o estado do sorteador entra (e a cópia roda sem contar)
    for x in crate::rng::STATE_REGIONS {
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

/// Lado a lado (a cópia sem contar os sorteios) pelas regiões de `stadium_choice_regions` e pela devolução; com reserva que pode alocar, só a original.
unsafe extern "thiscall" fn compare_find_available_venue(this: usize, marcas: u32, cont: usize, nacao: usize, data: usize, casa: usize, fora: usize, partida: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, usize, usize, usize) -> usize>(%%I1%%);
    let Some((r, m)) = stadium_choice_regions(this, None) else {
        // ponytail: a reserva pode alocar a lista; só a original
        return o(this, marcas, cont, nacao, data, casa, fora, partida);
    };
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&r[..m], || o(this, marcas, cont, nacao, data, casa, fora, partida), || {
        crate::rng::without_counting(|| find_available_venue(this, marcas, cont, nacao, data, casa, fora, partida))
    });
    SCOREBOARD.record(igual && ro == rr, || format!("find_available_venue(marcas {:x}, nacao {:x}, cont {:x}): original {:x} rust {:x} {:x?} x {:x?}", marcas, nacao, cont, ro, rr, mo, mr));
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
unsafe extern "thiscall" fn set_fixture_venue(this: usize, marcas: u32, partida: usize, estadio: u32) {
    if partida == 0 {
        return show_error_at(0xa835b0, 0x1f0);
    }
    let fx = partida;
    cqo1_pd(fx + 0x18, 0);
    cqo1_pd(fx + 4, u32::MAX as usize);
    let ano = *((fx + 0x28) as *const u16);
    let d = crate::date::Data { dia: *((fx + 0x2a) as *const i16), ano, is_leap_year: crate::date::is_leap_year(ano) };
    let pd = &d as *const crate::date::Data as usize;
    let fixar = |st: usize| {
        cqo1_pd(fx + 0x18, st);
        cqo1_pd(fx + 4, cqo1_d(st));
    };
    let neutro = (fx + 0x3c) as *mut u8;
    let mut f = marcas;
    // 0 = as regras gerais (0x85c5a7); 1 = a regra inglesa de 4 (0x85c38e)
    let mut step_towards = 0;
    if f == 0x8000 {
        let monaco = cqo1_d(0x9d0264);
        if cqo1_d(cqo1_d(fx + 0x1c)) != monaco && cqo1_d(cqo1_d(fx + 0x20)) != monaco {
            fixar(cqo1_d(cqo1_d(0xae23b4) + monaco * 0x245 + 0x69));
            return;
        }
        f = 4;
        step_towards = 1;
    } else if f == 4 {
        step_towards = 1;
    } else if f == 0 {
        let casa = cqo1_d(fx + 0x1c);
        if d.ano < 0x7d3 && cqo1_selecao(casa) && cqo1_d(cqo1_d(casa + 0x53)) == cqo1_d(0x9cf2e4) {
            f = 0x20;
        }
        let casa = cqo1_d(fx + 0x1c);
        let n = cqo1_d(casa + 0x53);
        let fin = cqo1_d(0x9cf2fc);
        if n != 0 && cqo1_d(n) == fin && *((fx + 0x42) as *const u8) != 0xff && cqo1_d(fx) == cqo1_d(0x9cf6e8) && cqo1_d(casa + 0x69) != 0 {
            let st = cqo1_d(nation_by_id(fin) + 0x80);
            let mut livre = true;
            if cqo1_d(this) != 0 {
                let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
                let mut chave = [0u32; 3];
                chave[0] = cqo1_d(st) as u32;
                chave[1] = (d.dia as u16 as u32) | ((d.ano as u32) << 16);
                chave[2] = d.is_leap_year as u32;
                livre = buscar(chave.as_ptr() as usize, cqo1_d(this), cqo1_d(this + 4) as u32, 0xc, 0x85e380) == 0;
            }
            if livre && (cqo1_d(cqo1_d(cqo1_d(fx + 0x1c) + 0x69) + 0x3c) as i32) < 40000 {
                *neutro = 1;
                fixar(cqo1_d(nation_by_id(fin) + 0x80));
                add_stadium_booking(this, cqo1_d(fx + 0x18), pd);
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
                crate::comparison::branch("cqo_1 home team stadium");
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
    if step_towards == 1 {
        let comp = cqo1_d(fx + 0x14);
        if comp != 0 {
            let n = cqo1_d(comp + 0x5d);
            if n != 0 && cqo1_d(n) == cqo1_d(0x9cf2e4) && !cqo1_selecao(cqo1_d(fx + 0x1c)) && cqo1_d(0xa835a4) as i32 != -1 {
                *neutro = 1;
                fixar(cqo1_d(nation_by_id(cqo1_d(0x9cf510)) + 0x80));
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
        let (na, nb) = (nation_by_id(a as usize), nation_by_id(b as usize));
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
        crate::comparison::branch("cqo_1 single host");
        alvo = nation_by_id(a as usize); // cqo1: a sede única
    } else {
        alvo = cqo1_d(comp + 0x5d);
    }
    if f & 0x20 != 0 && cqo1_selecao(cqo1_d(fx + 0x1c)) {
        alvo = cqo1_d(cqo1_d(fx + 0x1c) + 0x53);
    }
    if f & 4 != 0 && alvo != 0 {
        let st = cqo1_d(alvo + 0x80);
        if st != 0 {
            crate::comparison::branch("cqo_1 national stadium");
            fixar(st);
            add_stadium_booking(this, cqo1_d(fx + 0x18), pd);
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

/// Lado a lado pela partida (0x44 bytes) e pelas regiões de `stadium_choice_regions`; partida nula ou reserva que pode alocar:
/// só a original.
unsafe extern "thiscall" fn compare_set_fixture_venue(this: usize, marcas: u32, partida: usize, estadio: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32)>(%%I0%%);
    if partida == 0 {
        return o(this, marcas, partida, estadio);
    }
    let Some((r, m)) = stadium_choice_regions(this, Some((partida, 0x44))) else {
        // ponytail: a reserva pode alocar a lista; só a original
        return o(this, marcas, partida, estadio);
    };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&r[..m], || o(this, marcas, partida, estadio), || crate::rng::without_counting(|| set_fixture_venue(this, marcas, partida, estadio)));
    SCOREBOARD.record(igual, || format!("set_fixture_venue(marcas {:x}, comp {:x}): {:x?} x {:x?}", marcas, cqo1_d(partida), mo, mr));
}
