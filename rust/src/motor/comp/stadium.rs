use super::*;

// ---------- 279a leva: a escolha do estádio de um jogo: o estádio da partida e a busca do estádio neutro (stadium.cpp) ----------



/// A nação `id` (registro de 0x122 bytes em [0xae23a8]).
pub(crate) unsafe fn cqo1_nacao(id: usize) -> usize {
    read_u32_as_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(0x122))
}

/// O clube é uma seleção? (0x53b900, cdecl, já em Rust.)
pub(crate) unsafe fn cqo1_selecao(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    f(clube) != 0
}

/// `Stadium::add_booking` (0x85de20, thiscall, ret 8, já em Rust): a reserva do estádio na data.
pub(crate) unsafe fn cqo1_reservar(this: usize, estadio: usize, data: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STADIUM_ADD_BOOKING);
    f(this, estadio, data)
}

/// `Stadium::is_stadium_free_on_date` (0x85ddb0, thiscall, ret 8, já em Rust).
pub(crate) unsafe fn cqo1_livre(this: usize, estadio: usize, data: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_IS_STADIUM_FREE_ON_DATE);
    f(this, estadio, data) != 0
}

/// Os comparadores das listas (cdecl, (a, b) -> int): 0x85e2f0 pela nação (+0), 0x85e320 pelo continente (+4), os
/// dois desempatando pelos lugares do estádio (+0xc → +0x3c, quando os dois têm estádio); 0x85e350 só pelos lugares.
pub(crate) unsafe fn cqo1_cmp(f: usize, a: usize, b: usize) -> i32 {
    let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(f);
    f(a, b)
}

/// the neutral venue search: in the venue list sorted by nation, continent or capacity, up to 30 candidates of the key
/// (skipping the stashed venue and, in club matches, both clubs' grounds); the ordinary neutral venue is the free one
/// with the best score (0x85da00); the host rotation of a tournament (flags 0x40..0x4000) takes the candidate whose
/// position modulo 9 matches the flag; failing that, the national stadium (flags 4/8/0x20) or none
/// Original: `Stadium::find_available_venue` (0x85c800, stadium.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x85c800 } (thiscall, ret 0x1c, stadium.cpp, `Stadium::find_available_venue(marcas, continente, nação, data, casa,
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
pub(crate) unsafe extern "thiscall" fn estadio_neutro(this: usize, marcas: u32, cont: usize, nacao: usize, data: usize, casa: usize, fora: usize, partida: usize) -> usize {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let estado = (this + 0x18) as *mut u8;
    // a chave: só o campo da ordem e o +0xc (0) contam para os comparadores
    let mut chave = [0u32; 4];
    let mut cmp = 0usize;
    let mut e;
    if nacao != 0 {
        if *estado != 1 {
            ordenar(read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 0x10) as u32, 0x10, 0x85e2f0);
            *estado = 1;
        }
        chave[0] = read_u32_as_usize(nacao) as u32;
        cmp = 0x85e2f0;
    } else if cont != 0 {
        if *estado != 2 {
            ordenar(read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 0x10) as u32, 0x10, 0x85e320);
            *estado = 2;
        }
        chave[1] = read_u32_as_usize(cont) as u32;
        cmp = 0x85e320;
    } else {
        if *estado != 3 {
            ordenar(read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 0x10) as u32, 0x10, 0x85e350);
            *estado = 3;
        }
    }
    let k = chave.as_ptr() as usize;
    if cmp != 0 {
        e = buscar(k, read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 0x10) as u32, 0x10, cmp);
        if e != 0 {
            // volta até a primeira vaga da mesma chave
            loop {
                if read_u32_as_usize(this + 0xc) >= e {
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
        e = read_u32_as_usize(this + 0xc);
    }
    if e != 0 {
        let mut i: i16 = 0;
        let mut melhor = 0i32;
        let mut escolhida = 0usize;
        loop {
            if i >= 0x1e {
                break;
            }
            let ultima = (read_u32_as_usize(this + 0x10) as u32).wrapping_add(0x0fff_ffff).wrapping_shl(4).wrapping_add(read_u32_as_usize(this + 0xc) as u32);
            if e as u32 == ultima {
                break;
            }
            let mut contar = true;
            'vaga: {
                if read_u32_as_usize(read_u32_as_usize(e + 0xc)) == read_u32_as_usize(G_STADIUM_STASHED_VENUE_ID) {
                    contar = false;
                    break 'vaga;
                }
                if casa != 0 && fora != 0 && !cqo1_selecao(casa) && (marcas as i32) < 0x40 {
                    let s = read_u32_as_usize(e + 0xc);
                    if s == read_u32_as_usize(casa + 0x69) || s == read_u32_as_usize(fora + 0x69) {
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
                    if marcas & 4 != 0 && read_u32_as_usize(e + 8) == 0 {
                        contar = false;
                        break 'vaga;
                    }
                    if marcas & 2 != 0 && read_u32_as_usize(e + 8) != 0 {
                        contar = false;
                        break 'vaga;
                    }
                    if !cqo1_livre(this, read_u32_as_usize(e + 0xc), data) {
                        break 'vaga;
                    }
                    let nota: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32 = core::mem::transmute(FN_STADIUM_SCORE_VENUE_FOR_FIXTURE);
                    let n = nota(this, read_u32_as_usize(e + 0xc), marcas, casa, fora, partida);
                    if n > melhor {
                        melhor = n; // cqo1: a melhor nota
                        escolhida = e;
                    }
                } else {
                    if read_u32_as_usize(e + 8) != 0 {
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
                    if vez && ((partida != 0 && read_u32_as_usize(partida) == read_u32_as_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION)) || cqo1_livre(this, read_u32_as_usize(e + 0xc), data)) {
                        crate::comparacao::ramo("cqo_1 rodizio");
                        cqo1_reservar(this, read_u32_as_usize(e + 0xc), data);
                        return read_u32_as_usize(e + 0xc);
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
                cqo1_reservar(this, read_u32_as_usize(escolhida + 0xc), data);
            }
            return read_u32_as_usize(escolhida + 0xc);
        }
    }
    if nacao != 0 && marcas & 2 == 0 && marcas & 0x2c != 0 {
        return read_u32_as_usize(nacao + 0x80);
    }
    0
}

/// As regiões da escolha do estádio: o estado do sorteador, o cabeçalho (0x1c bytes), a lista dos estádios (a ordem muda) e as reservas com
/// a vaga da nova. `None` quando uma reserva pode alocar (sem lista ou cheia): então só a original.
pub(crate) unsafe fn cqo1_regioes(this: usize, extra: Option<(usize, usize)>) -> Option<([(usize, usize); 6], usize)> {
    let nr = read_u32_as_usize(this + 4);
    if read_u32_as_usize(this) == 0 || nr == read_u32_as_usize(this + 8) {
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
    r[m] = (read_u32_as_usize(this), (nr + 1) * 12);
    m += 1;
    let nv = read_u32_as_usize(this + 0x10);
    if read_u32_as_usize(this + 0xc) != 0 && nv > 0 && nv < 0x10000 {
        r[m] = (read_u32_as_usize(this + 0xc), nv * 16);
        m += 1;
    }
    if let Some(x) = extra {
        r[m] = x;
        m += 1;
    }
    Some((r, m))
}

/// Lado a lado (a cópia sem contar os sorteios) pelas regiões de `cqo1_regioes` e pela devolução; com reserva que pode alocar, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_estadio_neutro(this: usize, marcas: u32, cont: usize, nacao: usize, data: usize, casa: usize, fora: usize, partida: usize) -> usize {
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

/// the fixture venue (+0x18, id in +4, +0x3c neutral): an ordinary match is at the home club ground; the England
/// national team until 2002 and Italian and French clubs in friendlies go to a neutral ground; a Finnish side in the
/// Champions League with a ground under 40,000 plays at the national stadium if free; flag 0x8000 the Monaco ground;
/// the English competition (flag 4) with the stashed venue goes to the Welsh national stadium [inference: Cardiff, with
/// Wembley being rebuilt]; other flags the national stadium or the neutral venue search in the competition hosts
/// (0x5fa730), rotating between the two hosts of a tournament
/// Original: `Stadium::set_fixture_venue` (0x85c260, stadium.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x85c260 } (thiscall, ret 0xc, stadium.cpp, `Stadium::set_fixture_venue(marcas, partida, estadio)`; this =
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
pub(crate) unsafe extern "thiscall" fn estadio_da_partida(this: usize, marcas: u32, partida: usize, estadio: u32) {
    if partida == 0 {
        return aviso_em(0xa835b0, 0x1f0);
    }
    let fx = partida;
    write_u32_from_usize(fx + 0x18, 0);
    write_u32_from_usize(fx + 4, u32::MAX as usize);
    let ano = *((fx + 0x28) as *const u16);
    let d = crate::data::Data { dia: *((fx + 0x2a) as *const i16), ano, bissexto: crate::data::bissexto(ano) };
    let pd = &d as *const crate::data::Data as usize;
    let fixar = |st: usize| {
        write_u32_from_usize(fx + 0x18, st);
        write_u32_from_usize(fx + 4, read_u32_as_usize(st));
    };
    let neutro = (fx + 0x3c) as *mut u8;
    let mut f = marcas;
    // 0 = as regras gerais (0x85c5a7); 1 = a regra inglesa de 4 (0x85c38e)
    let mut passo = 0;
    if f == 0x8000 {
        let monaco = read_u32_as_usize(G_KEY_CLUB_AS_MONACO);
        if read_u32_as_usize(read_u32_as_usize(fx + 0x1c)) != monaco && read_u32_as_usize(read_u32_as_usize(fx + 0x20)) != monaco {
            fixar(read_u32_as_usize(read_u32_as_usize(G_CLUB_TABLE) + monaco * 0x245 + 0x69));
            return;
        }
        f = 4;
        passo = 1;
    } else if f == 4 {
        passo = 1;
    } else if f == 0 {
        let casa = read_u32_as_usize(fx + 0x1c);
        if d.ano < 0x7d3 && cqo1_selecao(casa) && read_u32_as_usize(read_u32_as_usize(casa + 0x53)) == read_u32_as_usize(KEY_NATION_ENGLAND) {
            f = 0x20;
        }
        let casa = read_u32_as_usize(fx + 0x1c);
        let n = read_u32_as_usize(casa + 0x53);
        let fin = read_u32_as_usize(KEY_NATION_FINLAND);
        if n != 0 && read_u32_as_usize(n) == fin && *((fx + 0x42) as *const u8) != 0xff && read_u32_as_usize(fx) == read_u32_as_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) && read_u32_as_usize(casa + 0x69) != 0 {
            let st = read_u32_as_usize(cqo1_nacao(fin) + 0x80);
            let mut livre = true;
            if read_u32_as_usize(this) != 0 {
                let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
                let mut chave = [0u32; 3];
                chave[0] = read_u32_as_usize(st) as u32;
                chave[1] = (d.dia as u16 as u32) | ((d.ano as u32) << 16);
                chave[2] = d.bissexto as u32;
                livre = buscar(chave.as_ptr() as usize, read_u32_as_usize(this), read_u32_as_usize(this + 4) as u32, 0xc, 0x85e380) == 0;
            }
            if livre && (read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(fx + 0x1c) + 0x69) + 0x3c) as i32) < 40000 {
                *neutro = 1;
                fixar(read_u32_as_usize(cqo1_nacao(fin) + 0x80));
                cqo1_reservar(this, read_u32_as_usize(fx + 0x18), pd);
                return;
            }
        }
        *neutro = 0;
        let casa = read_u32_as_usize(fx + 0x1c);
        if f != 0x20 && casa != 0 && read_u32_as_usize(casa + 0x69) != 0 {
            let n = read_u32_as_usize(casa + 0x53);
            let mut amistoso = false;
            if n != 0 && (read_u32_as_usize(n) == read_u32_as_usize(KEY_NATION_ITALY) || read_u32_as_usize(n) == read_u32_as_usize(KEY_NATION_FRANCE)) {
                let e: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                amistoso = e(read_u32_as_usize(G_FRIENDLY), read_u32_as_usize(fx + 0x14)) != 0;
            }
            if !amistoso {
                crate::comparacao::ramo("cqo_1 estadio do mandante");
                fixar(read_u32_as_usize(read_u32_as_usize(fx + 0x1c) + 0x69)); // cqo1: o estádio do mandante
                return;
            }
        } else if !cqo1_selecao(read_u32_as_usize(fx + 0x1c)) {
            write_u32_from_usize(fx + 0x18, 0);
            write_u32_from_usize(fx + 4, u32::MAX as usize);
            return;
        }
        f = 0x20;
    }
    if passo == 1 {
        let comp = read_u32_as_usize(fx + 0x14);
        if comp != 0 {
            let n = read_u32_as_usize(comp + 0x5d);
            if n != 0 && read_u32_as_usize(n) == read_u32_as_usize(KEY_NATION_ENGLAND) && !cqo1_selecao(read_u32_as_usize(fx + 0x1c)) && read_u32_as_usize(G_STADIUM_STASHED_VENUE_ID) as i32 != -1 {
                *neutro = 1;
                fixar(read_u32_as_usize(cqo1_nacao(read_u32_as_usize(KEY_NATION_WALES)) + 0x80));
                return;
            }
        }
    }
    // as regras gerais
    *neutro = 1;
    if f & 0x10 != 0 {
        if estadio as i32 == -1 {
            write_u32_from_usize(fx + 0x18, 0);
            write_u32_from_usize(fx + 4, u32::MAX as usize);
            return;
        }
        write_u32_from_usize(fx + 4, estadio as usize);
        write_u32_from_usize(fx + 0x18, read_u32_as_usize(G_STADIUM_TABLE).wrapping_add((estadio as usize).wrapping_mul(0x4e)));
        return;
    }
    if f == 1 {
        write_u32_from_usize(fx + 0x18, 0);
        write_u32_from_usize(fx + 4, u32::MAX as usize);
        return;
    }
    let comp = read_u32_as_usize(fx + 0x14);
    let sede: unsafe extern "thiscall" fn(usize, u32, u32, *mut u32, *mut u32, u32) -> u32 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
    let (mut a, mut b) = (0u32, 0u32);
    // ponytail: a original empurra o eax com o ano (word +0x2c) nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let r = sede(read_u32_as_usize(G_HOST_COUNTRY), read_u32_as_usize(comp) as u32, *((fx + 0x2c) as *const u16) as u32, &mut a, &mut b, 1) as u8;
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
        alvo = read_u32_as_usize(comp + 0x5d);
    }
    if f & 0x20 != 0 && cqo1_selecao(read_u32_as_usize(fx + 0x1c)) {
        alvo = read_u32_as_usize(read_u32_as_usize(fx + 0x1c) + 0x53);
    }
    if f & 4 != 0 && alvo != 0 {
        let st = read_u32_as_usize(alvo + 0x80);
        if st != 0 {
            crate::comparacao::ramo("cqo_1 estadio nacional");
            fixar(st);
            cqo1_reservar(this, read_u32_as_usize(fx + 0x18), pd);
            return;
        }
        f = if cqo1_selecao(read_u32_as_usize(fx + 0x1c)) { 0x20 } else { 0x40 };
    }
    let buscar: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_STADIUM_FIND_AVAILABLE_VENUE);
    let r = buscar(this, f, read_u32_as_usize(comp + 0x59), alvo, pd, read_u32_as_usize(fx + 0x1c), read_u32_as_usize(fx + 0x20), fx);
    write_u32_from_usize(fx + 0x18, r);
    if r == 0 && alt != 0 {
        f = match f {
            0x40 => 0x400,
            0x80 => 0x800,
            0x100 => 0x1000,
            0x200 => 0x2000,
            x => x,
        };
        let r = buscar(this, f, read_u32_as_usize(comp + 0x59), alt, pd, read_u32_as_usize(fx + 0x1c), read_u32_as_usize(fx + 0x20), fx);
        write_u32_from_usize(fx + 0x18, r);
    }
    let st = read_u32_as_usize(fx + 0x18);
    if st == 0 {
        write_u32_from_usize(fx + 4, u32::MAX as usize);
    } else {
        write_u32_from_usize(fx + 4, read_u32_as_usize(st));
    }
}

/// Lado a lado pela partida (0x44 bytes) e pelas regiões de `cqo1_regioes`; partida nula ou reserva que pode alocar:
/// só a original.
pub(crate) unsafe extern "thiscall" fn comparar_estadio_da_partida(this: usize, marcas: u32, partida: usize, estadio: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, u32)>(1887);
    if partida == 0 {
        return o(this, marcas, partida, estadio);
    }
    let Some((r, m)) = cqo1_regioes(this, Some((partida, 0x44))) else {
        // ponytail: a reserva pode alocar a lista; só a original
        return o(this, marcas, partida, estadio);
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&r[..m], || o(this, marcas, partida, estadio), || crate::aleatorio::sem_contar(|| estadio_da_partida(this, marcas, partida, estadio)));
    PLACAR.registrar(igual, || format!("estadio_da_partida(marcas {:x}, comp {:x}): {:x?} x {:x?}", marcas, read_u32_as_usize(partida), mo, mr));
}

// ---------- 279a leva: o público de um jogo (stadium.cpp) ----------


pub(crate) unsafe fn cqo2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



/// O sorteio do jogo (0x90cfc0, cdecl): 0..n-1.
pub(crate) unsafe fn cqo2_sortear(n: u32) -> i32 {
    let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A cidade (registro de 0x38 bytes em [0xae23ac]) do estádio `est` (+0x38); sem estádio ou sem cidade, uma sorteada
/// entre as [0xae2354].
pub(crate) unsafe fn cqo2_cidade(est: usize) -> usize {
    if est != 0 && read_u32_as_usize(est + 0x38) != 0 {
        return read_u32_as_usize(est + 0x38);
    }
    let r = cqo2_sortear(read_u32_as_usize(G_CITY_COUNT) as u32);
    read_u32_as_usize(G_CITY_TABLE).wrapping_add((r as usize).wrapping_mul(0x38))
}

/// O piso do fator de cada lado na competição de tipo `t` (+0x3a da partida), pelos limites +0x30/+0x32 da partida e
/// pela reputação (/500) `rep` do clube do lado (0x109a = 4250, 0x1676 = 5750).
pub(crate) unsafe fn cqo2_piso(jogo: usize, lado: u8, f: &mut [f64; 2]) {
    let t = read_u8(jogo + 0x3a);
    let rep = |l: u8| read_i16(read_u32_as_usize(jogo + if l == 0 { 0x1c } else { 0x20 }) + 0x80);
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
        let a = read_i16(jogo + 0x30) as u16;
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
        let a = read_i16(jogo + 0x30) as u16;
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
    } else if read_i16(jogo + 0x30) as u16 == 0x96 && t == 5 {
        if alto(0x109a) {
            piso(&mut f[k], f64_em(0x9690a8));
        } else {
            piso(&mut f[k], f64_em(0x968e38));
        }
    } else if read_i16(jogo + 0x32) as u16 == 0xa0 && t == 5 {
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

/// the match attendance: one factor per side (number of clubs in the competition, +2 for cup types 6/8/10, the league
/// round difference between −6 and 5; by the reputation/500 of both clubs: the bigger one draws more, ×1.1 against
/// sides above 17, ×1.25 against sides above 12; divided by the number of clubs in the club's league), with floors by
/// cup type (finals and semi-finals, +0x30/+0x32 = 150/130/160); the home side gets the average gate × factor ×
/// distance (0x85d8a0, at least 0.15), the away side (average + minimum) × factor × distance × 0.5; in the league, a
/// bonus by position (+50/100/150 per club at the top, +25/50 in the race, −25/−10 at the bottom) and by form; derby
/// ×1.5 (same city ×1.35), off neutral ground ×1.05/×0.85; capped by the capacity (away fans between 5% and 15%) and a
/// ground under 1,500 becomes reputation/500 × 1,000; total = both − random(50)
/// Original: `Stadium::calculate_match_attendance` (0x85cba0, stadium.cpp, thiscall, ret 8).
///
/// [pt] { 0x85cba0 } (thiscall, ret 8, stadium.cpp, `Stadium::calculate_match_attendance(partida, saida)`; this =
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
pub(crate) unsafe extern "thiscall" fn publico_do_jogo(this: usize, jogo: usize, saida: *mut i32) -> i32 {
    if jogo == 0 {
        return cqo2_i(0xa835ac);
    }
    let casa = read_u32_as_usize(jogo + 0x1c);
    let fora = read_u32_as_usize(jogo + 0x20);
    let r500 = |c: usize| -> i8 { (read_i16(c + 0x80) as i32 / 500) as i8 };
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
        let comp = read_u32_as_usize(jogo + 0x14);
        if comp != 0 {
            f[k] = read_i16(comp + 0x69) as i32 as f64;
            if f[k] < f64_em(0x967890) {
                f[k] = 1.0;
            }
        } else {
            f[k] = 1.0;
        }
        let t = read_u8(jogo + 0x3a);
        if t == 8 || t == 10 || t == 6 {
            f[k] += f64_em(0x968950);
        }
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
        let mut ajuste = f64_em(0x9689a0);
        if amistoso(read_u32_as_usize(G_FRIENDLY), comp) == 0 && read_u32_as_usize(G_COMP_OBJECT_TABLE) != 0 && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(comp) * 4) != 0 && read_u32_as_usize(G_RESERVE_COMP) != 0 {
            let grupo: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
            if grupo(read_u32_as_usize(jogo + 0x14)) == 0 {
                let liga = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(read_u32_as_usize(jogo + 0x14)) * 4);
                if read_u8(liga + 0x43) != 2 {
                    let x = (read_u8(jogo + 0x34).wrapping_sub(read_u8(liga + 0x3c)) as i8) as i32 as f64;
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
            let lg = read_u32_as_usize(read_u32_as_usize(jogo + if lado == 0 { 0x1c } else { 0x20 }) + 0x57);
            if lg != 0 && read_i16(lg + 0x69) != 0 {
                f[k] /= read_i16(lg + 0x69) as i32 as f64;
            } else if read_u8(jogo + 0x3a) == 0xa {
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
    let est = read_u32_as_usize(jogo + 0x18);
    if est != 0 && read_u8(jogo + 0x3c) == 0 {
        let c_est = cqo2_cidade(est);
        let c_casa = cqo2_cidade(read_u32_as_usize(casa + 0x69));
        let c_fora = cqo2_cidade(read_u32_as_usize(fora + 0x69));
        let fator: unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64 = core::mem::transmute(FN_STADIUM_GET_TRAVEL_DISTANCE_FACTOR);
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
        let comp = read_u32_as_usize(jogo + 0x14);
        if comp != 0 && casa != 0 && read_u32_as_usize(casa + 0x57) == comp {
            let liga = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(comp) * 4);
            if liga != 0 && read_u32_as_usize(liga + 8) != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
                let p = busca(read_u32_as_usize(liga + 8), casa);
                if p != 0 {
                    let n = read_i16(comp + 0x69) as i32;
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
                    let dl = read_u8(p + 0x74);
                    if dl != read_u8(p + 0x75) && read_u8(p + 4) > 4 {
                        let cl = read_u8(p + 0x73);
                        let bl = read_u8(p + 0x72);
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
    let (casa, fora) = (read_u32_as_usize(jogo + 0x1c), read_u32_as_usize(jogo + 0x20));
    let rival = |a: usize, b: usize| read_u32_as_usize(a + 0xb3) == b || read_u32_as_usize(a + 0xb7) == b || read_u32_as_usize(a + 0xbb) == b;
    if rival(casa, fora) {
        crate::comparacao::ramo("cqo_2 classico");
        pc = ftol(pc as f64 * f64_em(0x968940)); // cqo2: o clássico
    } else if read_u32_as_usize(casa + 0x69) != 0 && read_u32_as_usize(fora + 0x69) != 0 && read_u32_as_usize(read_u32_as_usize(casa + 0x69) + 0x38) == read_u32_as_usize(read_u32_as_usize(fora + 0x69) + 0x38) {
        pc = ftol(pc as f64 * f64_em(0x969588));
    }
    if rival(fora, casa) {
        pf = ftol(pf as f64 * f64_em(0x969588));
    }
    if read_u8(jogo + 0x3c) == 0 && read_u8(jogo + 0x3b) != 0xff {
        pc = ftol(pc as f64 * f64_em(0x9689b8));
        pf = ftol(pf as f64 * f64_em(0x969520));
    }
    let comp = read_u32_as_usize(jogo + 0x14);
    if comp != 0 && casa != 0 && read_u32_as_usize(casa + 0x57) == comp && read_u8(casa + 0x6d) != 0 {
        let n = read_i16(comp + 0x69);
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
    let est = read_u32_as_usize(jogo + 0x18);
    if est != 0 && cqo2_i(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, rc as i32 * 1000);
        if cqo2_i(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    if pf.wrapping_add(pc) != 0 {
        let est = read_u32_as_usize(jogo + 0x18);
        if est != 0 {
            let lot = cqo2_i(est + 0x3c);
            let vis = if read_u8(jogo + 0x3c) == 0 {
                let a = lot as f64;
                let x = a - cqo2_i(read_u32_as_usize(jogo + 0x20) + 0x73) as f64 * f[0];
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
pub(crate) unsafe extern "thiscall" fn comparar_publico_do_jogo(this: usize, jogo: usize, saida: *mut i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32>(1889);
    if jogo == 0 {
        return o(this, jogo, saida);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !saida.is_null() {
        reg.push((saida as usize, 12));
    }
    if read_u32_as_usize(jogo + 0x18) != 0 {
        reg.push((read_u32_as_usize(jogo + 0x18) + 0x3c, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida), || crate::aleatorio::sem_contar(|| publico_do_jogo(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("publico_do_jogo(comp {:x}, tipo {}): original {} rust {} {:x?} x {:x?}", read_u32_as_usize(jogo), read_u8(jogo + 0x3a), r, n, mo, mr));
    r
}

// ---------- 256a leva: o estádio neutro e o clima inicial: a nota do estádio para o jogo, o fator da distância no público e a leitura do weather.cfg (stadium.cpp, weather.cpp) ----------


pub(crate) unsafe fn cpt7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


pub(crate) unsafe fn cpt7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A distância entre duas cidades (x em +0x23, y em +0x2b, doubles) × 44,44, truncada (_ftol).
pub(crate) unsafe fn cpt7_distancia(a: usize, b: usize) -> i32 {
    let dx = (read_f64(a + 0x23) - read_f64(b + 0x23)).abs();
    let dy = (read_f64(a + 0x2b) - read_f64(b + 0x2b)).abs();
    ftol((dy * dy + dx * dx).sqrt() * f64_em(0x96fe40))
}

/// the English national stadium before 2004 scores 0; the score starts at 100000, changes with the city distances (flag
/// 8) or with the places against the expected crowd (flag 0x20: national team 60000, otherwise the clubs crowd sum) or
/// becomes random(places) (flag 4), and ends × (random(100) × 0.004 + 0.8)
/// Original: `Stadium::score_venue_for_fixture` (0x85da00, stadium.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x85da00 } (thiscall, ret 0x14, stadium.cpp): `Stadium::score_venue_for_fixture(estadio, marcas, casa, fora,
/// jogo)`, pela busca do estádio neutro 0x85c800 (com as marcas 0x20 nos jogos da carreira). O estádio nacional da
/// Inglaterra (nação [0x9cf2e4] +0x80) antes de 2004: 0. A nota parte de 100000:
/// - marcas & 8, com as cidades dos dois clubes (+0x69 → +0x38) e a do estádio (+0x38) e mais de 10000 lugares (+0x3c):
///   pelas distâncias d1 (casa-fora), d2 (casa-estádio), d3 (fora-estádio): com d1 < 30, 200000/180000/140000 se d2 e
///   d3 < 15/30/50 e 50000 se uma passa de 100; senão 200000 se as duas < 0,588·d1 e 50000 se uma ≥ 0,8333·d1;
/// - marcas & 0x20: o público esperado (10000; seleção da casa 60000; senão a soma dos públicos +0x73 dos dois clubes,
///   no mínimo 5000 e, acima de 40000, a metade sem ficar abaixo de nenhum dos dois); lugares acima dele: × 1,2; acima
///   de 1,5× dele: / ((lugares − 1,5·esperado)·8,33e-5 + 1); abaixo de 0,3/0,5/0,7/0,9 dele: × 0,1/0,3/0,7/0,9;
/// - senão, com marcas & 4: a nota é sorteio(lugares).
/// No fim, nota × (sorteio(100) × 0,004 + 0,8), truncada.
pub(crate) unsafe extern "thiscall" fn nota_do_estadio_neutro(_this: usize, est: usize, marcas: u32, casa: usize, fora: usize, jogo: usize) -> i32 {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let inglaterra = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_ENGLAND).wrapping_mul(0x122));
    if est == read_usize(inglaterra + 0x80) && core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16) < 0x7d4 {
        return 0;
    }
    let m = marcas as u8;
    let mut nota: i32 = 100000;
    if m & 8 != 0 {
        let cc = read_usize(casa + 0x69);
        let cf = read_usize(fora + 0x69);
        let (cidc, cidf) = (if cc != 0 { read_usize(cc + 0x38) } else { 0 }, if cf != 0 { read_usize(cf + 0x38) } else { 0 });
        let cide = read_usize(est + 0x38);
        if cc != 0 && cidc != 0 && cf != 0 && cidf != 0 && cide != 0 && cpt7_i(est + 0x3c) > 0x2710 {
            let d1 = cpt7_distancia(cidc, cidf);
            let d2 = cpt7_distancia(cidc, cide);
            let d3 = cpt7_distancia(cidf, cide);
            if d1 < 0x1e {
                if d2 < 0xf && d3 < 0xf {
                    nota = 200000;
                } else if d2 < 0x1e && d3 < 0x1e {
                    nota = 180000;
                } else if d2 < 0x32 && d3 < 0x32 {
                    nota = 140000;
                } else if d2 >= 0x64 || d3 >= 0x64 {
                    nota = 50000;
                }
            } else {
                let (f1, f2, f3) = (d1 as f64, d2 as f64, d3 as f64);
                let perto = f1 * f64_em(0x96fe38);
                let longe = f1 * f64_em(0x969088);
                if f2 < perto && f3 < perto {
                    nota = 200000;
                } else if !(f2 < longe && f3 < longe) {
                    nota = 50000;
                }
            }
        }
    }
    let lugares = cpt7_i(est + 0x3c);
    if m & 0x20 != 0 {
        let mut esperado: i32 = 0x2710;
        if jogo != 0 {
            if e_selecao(read_usize(jogo + 0x1c)) != 0 {
                esperado = 0xea60;
            } else {
                let a = cpt7_i(read_usize(jogo + 0x20) + 0x73);
                let c = cpt7_i(read_usize(jogo + 0x1c) + 0x73);
                let s = c.wrapping_add(a);
                if s < 0x1388 {
                    esperado = 0x1388;
                } else if s > 0x9c40 {
                    let mut h = s / 2;
                    if h <= c {
                        h = c;
                    }
                    if h <= a {
                        h = a;
                    }
                    esperado = h;
                } else {
                    esperado = s;
                }
            }
        }
        if lugares > esperado {
            nota = ftol(nota as f64 * f64_em(0x968978));
        }
        let (l, e) = (lugares as f64, esperado as f64);
        let t = e * f64_em(0x968940);
        if l > t {
            nota = ftol(nota as f64 / ((l - t) * f64_em(0x96fe30) + f64_em(0x967890)));
        } else if l < e * f64_em(0x968e80) {
            nota = ftol(nota as f64 * f64_em(0x967880));
        } else if l < e * f64_em(0x968968) {
            nota = ftol(nota as f64 * f64_em(0x968e80));
        } else if l < e * f64_em(0x968e40) {
            nota = ftol(nota as f64 * f64_em(0x968e40));
        } else if l < e * f64_em(0x9689d8) {
            nota = ftol(nota as f64 * f64_em(0x9689d8));
        }
    } else if m & 4 != 0 {
        nota = cpt7_sortear(lugares);
    }
    crate::comparacao::ramo("cpt_7 nota do estadio");
    let r = cpt7_sortear(100); // cpt7: o sorteio da nota
    ftol((r as f64 * f64_em(0x968d60) + f64_em(0x9689b0)) * nota as f64)
}

// ponytail: sorteia (e, com a marca 4, sorteia os lugares): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_nota_do_estadio_neutro(this: usize, est: usize, marcas: u32, casa: usize, fora: usize, jogo: usize) -> i32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, usize) -> i32>(1173)(this, est, marcas, casa, fora, jogo)
}

/// 1 − distance × k (squared without +0x3c), divided by (reputation + 1)² when far and with reputation; with minimums
/// by the base, by type 10 (0.2), by both clubs of the same nation (0.05) and a general one
/// Original: `Stadium::get_travel_distance_factor` (0x85d8a0, stadium.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x85d8a0 } (thiscall, ret 0x14, stadium.cpp): `Stadium::get_travel_distance_factor(jogo, cidade_do_estadio,
/// cidade_do_clube, base)`, no público do jogo (0x85cba0, pelo motor 0x6b7f20). D = distância entre as cidades
/// (pela metade com o jogo +0x30 = 0xa0 e +0x3a = 5); F = 1 − D·[0x96afe0], ao quadrado sem +0x3c; com D e a
/// reputação R (byte +0x7e da nação +0x53 do clube de fora +0x20) acima de [0x9689a0]: F / (R + 1)². Com base acima de
/// [0x969558], o mínimo 1 − ([0x968ea8] − base)·[0x968f20]; com +0x3a = 10, o mínimo 0,2; com os dois clubes da mesma
/// nação, 0,05; e sempre [0x96d2c8]. Devolve o double no st0.
pub(crate) unsafe extern "thiscall" fn fator_da_distancia(_this: usize, jogo: usize, cid_est: usize, cid_clube: usize, base: f64) -> f64 {
    let nacao = read_usize(read_usize(jogo + 0x20) + 0x53);
    let r = *((nacao + 0x7e) as *const i8) as i32 as f64;
    let dy = read_f64(cid_est + 0x2b) - read_f64(cid_clube + 0x2b);
    let dx = read_f64(cid_est + 0x23) - read_f64(cid_clube + 0x23);
    let mut d = (dx * dx + dy * dy).sqrt();
    let tipo = *((jogo + 0x3a) as *const u8);
    if core::ptr::read_unaligned((jogo + 0x30) as *const u16) == 0xa0 && tipo == 5 {
        d *= f64_em(0x968968);
    }
    let mut f = f64_em(0x967890) - d * f64_em(0x96afe0);
    if *((jogo + 0x3c) as *const u8) == 0 {
        f *= f;
    }
    if d > f64_em(0x9689a0) && r > f64_em(0x9689a0) {
        let q = r + f64_em(0x967890);
        f /= q * q;
    }
    if base > f64_em(0x969558) {
        let minimo = f64_em(0x967890) - (f64_em(0x968ea8) - base) * f64_em(0x968f20);
        if f < minimo {
            return minimo;
        }
    }
    if tipo == 0xa && f < f64_em(0x968918) {
        return f64_em(0x968918);
    }
    if f < f64_em(0x967888) && read_usize(read_usize(jogo + 0x1c) + 0x53) == nacao {
        return f64_em(0x967888);
    }
    if f < f64_em(0x96d2c8) {
        return f64_em(0x96d2c8);
    }
    f
}

/// Comparador: pelo valor (os bits do double).
pub(crate) unsafe extern "thiscall" fn comparar_fator_da_distancia(this: usize, jogo: usize, cid_est: usize, cid_clube: usize, base: f64) -> f64 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64>(1174)(this, jogo, cid_est, cid_clube, base);
    let n = fator_da_distancia(this, jogo, cid_est, cid_clube, base);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("fator_da_distancia(jogo {:x}, base {}) {} x {}", jogo, base, r, n));
    r
}


// ponytail: abre o arquivo, aloca e sorteia: só a original; a cópia leva o ramo.

// ---------- 255a leva: os estádios: o dia dos estádios (o estádio nacional fechado e as reservas vencidas), reservar um estádio numa data e se ele está livre (stadium.cpp) ----------




/// A nação da Inglaterra ([0x9cf2e4]; registros de 0x122 bytes em [0xae23a8]).
pub(crate) unsafe fn cps7_inglaterra() -> usize {
    read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_ENGLAND).wrapping_mul(0x122))
}

/// A seleção principal da nação (0x53b3d0).
pub(crate) unsafe fn cps7_selecao(nacao: usize) -> usize {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    f(nacao)
}

/// O estádio de id `id` (registros de 0x4e bytes em [0xae23b0]).
pub(crate) unsafe fn cps7_estadio(id: usize) -> usize {
    read_usize(G_STADIUM_TABLE).wrapping_add(id.wrapping_mul(0x4e))
}

pub(crate) unsafe fn cps7_ordenar(lista: usize, n: u32) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(lista, n, 0xc, 0x85e380);
}

/// on day 0 rebuilds the venue list; before 2003 closes the England national stadium (id in [0xa835a4], nation and
/// national team without a stadium) and reopens it on 1/6/2003 with 90000 places; every 30 days removes the expired
/// bookings and re-sorts
/// Original: `Stadium::daily_update` (0x85b820, stadium.cpp, thiscall, ret 4).
///
/// [pt] { 0x85b820 } (thiscall, ret 4, stadium.cpp): `Stadium::daily_update(hoje)`, todo dia pelo laço do dia do jogo
/// ([0xdd7ef4]). No dia 0 do ano refaz a lista dos estádios (0x85dfc0). Antes de 2003, o estádio nacional da Inglaterra
/// (nação +0x80) fecha: o id vai para [0xa835a4], a nação e a seleção principal (+0x69) ficam sem estádio. Em
/// 1/6/2003 (0x549f70) ele volta com 90000 lugares (+0x3c, +0x40). A cada 30 dias, as reservas (lista em +0, conta em
/// +4, 12 bytes: id e data) antes de hoje saem: a original conta as k primeiras (ano sem sinal, dia com sinal), copia
/// por cima delas, para a frente, as k que começam em n − k − 1 (não em n − k) e reordena as n − k (0x945333 com
/// 0x85e380).
pub(crate) unsafe extern "thiscall" fn estadios_do_dia(this: usize, hoje: usize) {
    if read_u16(hoje) == 0 {
        let refazer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_STADIUM_BUILD_VENUE_LIST);
        refazer(this);
    }
    if read_u16(hoje + 2) < 0x7d3 {
        let nacao = cps7_inglaterra();
        let e = read_usize(nacao + 0x80);
        if e != 0 {
            crate::comparacao::ramo("cps_7 fecha o estadio nacional");
            write_u32(G_STADIUM_STASHED_VENUE_ID, read_usize(e) as u32); // cps7: guarda o id do estádio fechado
            write_u32(nacao + 0x80, 0);
            write_u32(cps7_selecao(cps7_inglaterra()) + 0x69, 0); // cps7: a seleção sem casa
        }
    }
    let nova: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut t = [0u8; 8];
    let r = nova(t.as_mut_ptr() as usize, 1, 6, 0x7d3, 0xffff_ffff);
    let reabre = read_usize(r) as u32;
    if (reabre >> 16) as u16 == read_u16(hoje + 2) && reabre as u16 == read_u16(hoje) && read_usize(G_STADIUM_STASHED_VENUE_ID) as i32 != -1 {
        crate::comparacao::ramo("cps_7 reabre o estadio nacional");
        write_u32(cps7_inglaterra() + 0x80, cps7_estadio(read_usize(G_STADIUM_STASHED_VENUE_ID)) as u32);
        let nacao = cps7_inglaterra();
        write_u32(cps7_selecao(nacao) + 0x69, read_usize(nacao + 0x80) as u32);
        write_u32(cps7_estadio(read_usize(G_STADIUM_STASHED_VENUE_ID)) + 0x3c, 0x15f90);
        write_u32(cps7_estadio(read_usize(G_STADIUM_STASHED_VENUE_ID)) + 0x40, 0x15f90);
        write_u32(G_STADIUM_STASHED_VENUE_ID, 0xffff_ffff);
    }
    if (read_u16(hoje) as i16 as i32) % 0x1e != 0 {
        return;
    }
    let lista = read_usize(this);
    if lista == 0 || (read_usize(this + 4) as i32) <= 0 {
        return;
    }
    let (dia, ano) = (read_u16(hoje) as i16, read_u16(hoje + 2));
    // k em 32 bits, o índice em 16 com sinal (como a original: [esp+0x10] e si)
    let mut k = 0u32;
    let mut i = 0i16;
    loop {
        let e = lista.wrapping_add((i as i32 as usize).wrapping_mul(12));
        let ano_e = read_u16(e + 6);
        let antes = ano > ano_e || (ano == ano_e && dia > read_u16(e + 4) as i16);
        if !antes {
            break;
        }
        k = k.wrapping_add(1);
        i = i.wrapping_add(1);
        if i as i32 >= read_usize(this + 4) as i32 {
            break;
        }
    }
    if k as u16 == 0 {
        return;
    }
    crate::comparacao::ramo("cps_7 reservas vencidas");
    let k = k as i16 as i32;
    let n = read_usize(this + 4) as i32;
    if k < n {
        let de = lista.wrapping_add(((n - k - 1) as usize).wrapping_mul(12));
        // rep movsd para a frente: as regiões podem se sobrepor
        for j in 0..(k as usize * 3) {
            write_u32(lista + j * 4, read_usize(de + j * 4) as u32);
        }
    }
    write_u32(this + 4, (n - k) as u32);
    cps7_ordenar(read_usize(this), (n - k) as u32);
}

/// Comparador: no dia 0 do ano (refaz a lista dos estádios, aloca) só a original. Nos outros, lado a lado pelo cabeçalho
/// (0x1c bytes), pelas reservas, por [0xa835a4], pelo estádio nacional da Inglaterra, pela casa da seleção e pelos
/// lugares do estádio fechado.
pub(crate) unsafe extern "thiscall" fn comparar_estadios_do_dia(this: usize, hoje: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1134);
    if read_u16(hoje) == 0 {
        return o(this, hoje);
    }
    let mut regioes = [(0usize, 0usize); 6];
    let mut m = 0;
    regioes[m] = (this, 0x1c);
    m += 1;
    let n = read_usize(this + 4) as i32;
    if read_usize(this) != 0 && n > 0 {
        regioes[m] = (read_usize(this), n as usize * 12);
        m += 1;
    }
    regioes[m] = (0xa835a4, 4);
    m += 1;
    let nacao = cps7_inglaterra();
    regioes[m] = (nacao + 0x80, 4);
    m += 1;
    let s = cps7_selecao(nacao);
    if s != 0 {
        regioes[m] = (s + 0x69, 4);
        m += 1;
    }
    if read_usize(G_STADIUM_STASHED_VENUE_ID) as i32 != -1 {
        regioes[m] = (cps7_estadio(read_usize(G_STADIUM_STASHED_VENUE_ID)) + 0x3c, 8);
        m += 1;
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes[..m], || o(this, hoje), || estadios_do_dia(this, hoje));
    PLACAR.registrar(igual, || format!("estadios_do_dia(dia {}, ano {}) {:x?} x {:x?}", read_u16(hoje) as i16, read_u16(hoje + 2), mo, mr));
}

/// appends the booking (stadium id and date) to the list, which grows 30 at a time, and re-sorts by date and id
/// Original: `Stadium::add_booking` (0x85de20, stadium.cpp, thiscall, ret 8).
///
/// [pt] { 0x85de20 } (thiscall, ret 8, stadium.cpp): `Stadium::add_booking(estadio, data)`: acrescenta a reserva (id do
/// estádio, cópia da data por 0x417c10) à lista (+0; conta +4, capacidade +8), que nasce com 30 (0x944e46, 0x168
/// bytes) e cresce de 30 em 30 (0x945667), e reordena (0x945333 com 0x85e380). Estádio ou data nulos, ou sem memória: a
/// caixa "Error" (linhas 0x810, 0x81b, 0x82b). Pela escolha do estádio (0x85c260, 0x85c800).
pub(crate) unsafe extern "thiscall" fn reservar_estadio(this: usize, estadio: usize, data: usize) {
    if estadio == 0 || data == 0 {
        return aviso_em(0xa835b0, 0x810);
    }
    if read_usize(this) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let l = alocar(0x168);
        write_u32(this, l as u32);
        if l == 0 {
            return aviso_em(0xa835b0, 0x81b);
        }
        write_u32(this + 8, 0x1e);
    } else if read_usize(this + 4) == read_usize(this + 8) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let l = realocar(read_usize(this), (read_usize(this + 8) as u32).wrapping_mul(3).wrapping_add(0x5a).wrapping_mul(4));
        write_u32(this, l as u32);
        if l == 0 {
            return aviso_em(0xa835b0, 0x82b);
        }
        write_u32(this + 8, (read_usize(this + 8) as u32).wrapping_add(0x1e));
    }
    let e = read_usize(this).wrapping_add(read_usize(this + 4).wrapping_mul(12));
    write_u32(e, read_usize(estadio) as u32);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(e + 4, t.as_mut_ptr() as usize, data);
    let n = (read_usize(this + 4) as u32).wrapping_add(1);
    write_u32(this + 4, n);
    cps7_ordenar(read_usize(this), n);
}

/// Comparador: com lista e vaga (não aloca), lado a lado pelo cabeçalho (0xc bytes) e pelas reservas com a nova; senão
/// (aloca, realoca ou a caixa) só a original.
pub(crate) unsafe extern "thiscall" fn comparar_reservar_estadio(this: usize, estadio: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1135);
    let n = read_usize(this + 4);
    if estadio == 0 || data == 0 || read_usize(this) == 0 || n == read_usize(this + 8) {
        return o(this, estadio, data);
    }
    let regioes = [(this, 0xc), (read_usize(this), (n + 1) * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, estadio, data), || reservar_estadio(this, estadio, data));
    PLACAR.registrar(igual, || format!("reservar_estadio(estadio {:x}, reservas {}) {:x?} x {:x?}", estadio, n, mo, mr));
}

/// 1 when there is no booking of the stadium on the date (binary search in the list; no list, 1)
/// Original: `Stadium::is_stadium_free_on_date` (0x85ddb0, stadium.cpp, thiscall, ret 8).
///
/// [pt] { 0x85ddb0 } (thiscall, ret 8, stadium.cpp): `Stadium::is_stadium_free_on_date(estadio, data)`: 1 sem lista de
/// reservas; senão 1 se a busca binária (0x946464 com 0x85e380) não acha a reserva (id do estádio, data copiada por
/// 0x417c10). Pela escolha do estádio neutro (0x85c800).
pub(crate) unsafe extern "thiscall" fn estadio_livre_na_data(this: usize, estadio: usize, data: usize) -> u32 {
    if read_usize(this) == 0 {
        return 1;
    }
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let mut chave = [0u8; 12];
    let mut t = [0u8; 8];
    core::ptr::write_unaligned(chave.as_mut_ptr() as *mut u32, read_usize(estadio) as u32);
    copia(chave.as_mut_ptr() as usize + 4, t.as_mut_ptr() as usize, data);
    let r = buscar(chave.as_ptr() as usize, read_usize(this), read_usize(this + 4) as u32, 0xc, 0x85e380);
    crate::comparacao::ramo("cps_7 reserva conferida");
    (r == 0) as u32 // cps7: livre sem a reserva
}

/// Comparador: a original e a cópia (as duas só leem), pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_estadio_livre_na_data(this: usize, estadio: usize, data: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1136)(this, estadio, data);
    let n = estadio_livre_na_data(this, estadio, data);
    PLACAR.registrar(r == n, || format!("estadio_livre_na_data(estadio {:x}) original={} rust={}", estadio, r, n));
    r
}

/// bookings qsort: by date (+4), then by stadium id (+0)
/// Original: `stadium_compare_bookings` (0x85e380, stadium.cpp, cdecl).
///
/// [pt] { 0x85e380 } (cdecl, stadium.cpp): `stadium_compare_bookings(a, b)`, a ordem das reservas do estádio (qsort em
/// 0x85ba1b, 0x85c4ba, 0x85ddf3, 0x85df26): pela data (+4, Date::operator< 0x456770 e > 0x456740), depois pelo id do
/// estádio (+0).
pub(crate) unsafe extern "C" fn ordem_das_reservas_do_estadio(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let (d0, d1) = (read_usize(b + 4) as u32, read_usize(b + 8) as u32);
    if antes(a + 4, d0, d1) != 0 {
        return -1;
    }
    if depois(a + 4, d0, d1) != 0 {
        return 1;
    }
    cpu5_i(a).wrapping_sub(cpu5_i(b))
}

/// Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_ordem_das_reservas_do_estadio(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1208)(a, b);
    let n = ordem_das_reservas_do_estadio(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_reservas_do_estadio({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// the neutral venue rule (finals and semi-finals): grounds with a city, nation and continent qualify, except the City
/// of Manchester Stadium (under construction) and, in nations with competitions, those under 5000 seats; each nation's
/// national stadium is flagged
/// Original: `Stadium::build_venue_list` (0x85dfc0, stadium.cpp, thiscall, ret 0).
///
/// [pt] { 0x85dfc0 } (thiscall, ret 0, stadium.cpp, `Stadium::build_venue_list`): refaz a lista dos estádios para jogos
/// em campo neutro (+0xc a lista de registros de 16 bytes, +0x10 a conta, +0x14 a capacidade; a velha é solta). As
/// sedes das próximas edições de três competições (as chaves 0x9cf79c, 0x9cf7a4, 0x9cf7ac; 0x5fa7d0 no HostCountry
/// [0xb5e134], sem ele a caixa da linha 0x858) são achadas, mas não mudam nada (o laço delas termina igual nos dois
/// caminhos). Entram os estádios (registros de 0x4e bytes em [0xae23b0], conta [0xae2358]) com cidade (+0x38), nação da
/// cidade (+0x1f) e continente da nação (+0x71), fora o "City of Manchester Stadium" (ainda em obras em 2001) e, nas
/// nações com competições (+0x11c), os de menos de 5000 lugares (+0x3c). Cada registro: +0 o id da nação, +4 o id do
/// continente, +8 se é o estádio nacional (+0x80 da nação), +0xc o estádio. A lista cresce de 30 em 30 (0x944e46,
/// realloc 0x945667; sem memória, as caixas das linhas 0x89c e 0x8ac, e para). No fim é ordenada (qsort com a
/// 0x85e2f0) e +0x18 = 1.
pub(crate) unsafe extern "thiscall" fn lista_de_estadios_neutros(this: usize) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sedes: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u8 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_NEXT_HOSTS);
    let (lista, conta, cap) = ((this + 0xc) as *mut usize, (this + 0x10) as *mut i32, (this + 0x14) as *mut i32);
    if *lista != 0 {
        soltar(*lista);
        *lista = 0;
        *cap = 0;
        *conta = 0;
    }
    let hc = read_u32(G_HOST_COUNTRY) as usize;
    if hc == 0 {
        aviso_em(0xa835b0, 0x858);
    } else {
        // as sedes: só achadas (a original não as usa depois); as saídas se sobrepõem como na original
        let mut s = [0u32; 8];
        let mut b = sedes(hc, read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP), s.as_mut_ptr(), s.as_mut_ptr().add(1)) as i8;
        b = b.wrapping_add(sedes(hc, read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP), s.as_mut_ptr().add(b as usize), s.as_mut_ptr().add(b as usize + 1)) as i8);
        sedes(hc, read_u32(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS), s.as_mut_ptr().add(b as usize), s.as_mut_ptr().add(b as usize + 1));
    }
    let mut k = 0i32;
    while k < read_u32(G_STADIUM_COUNT) as i32 {
        let e = read_u32(G_STADIUM_TABLE) as usize + 0x4e * k as usize;
        k = (k as i16).wrapping_add(1) as i32;
        let cidade = read_u32(e + 0x38) as usize;
        if cidade == 0 {
            continue;
        }
        let nacao = read_u32(cidade + 0x1f) as usize;
        if nacao == 0 || read_u32(nacao + 0x71) == 0 {
            continue;
        }
        if core::ffi::CStr::from_ptr((e + 4) as *const core::ffi::c_char).to_bytes() == b"City of Manchester Stadium" {
            continue;
        }
        if *((nacao + 0x11c) as *const u8) != 0 && (read_u32(e + 0x3c) as i32) < 0x1388 { // cqn3: menos de 5000 lugares
            crate::comparacao::ramo("cqn_3 estadio pequeno");
            continue; // cqn3: o estádio pequeno fica de fora
        }
        if *lista == 0 {
            *lista = alocar(0x1e0);
            if *lista == 0 {
                aviso_em(0xa835b0, 0x89c);
                return;
            }
            *cap = 0x1e;
        } else if *conta == *cap {
            *lista = realocar(*lista, ((*cap + 0x1e) << 4) as u32);
            if *lista == 0 {
                aviso_em(0xa835b0, 0x8ac);
                return;
            }
            *cap += 0x1e;
        }
        let r = *lista + 16 * *conta as usize;
        *(r as *mut u32) = read_u32(nacao);
        *((r + 4) as *mut u32) = read_u32(read_u32(nacao + 0x71) as usize);
        *((r + 0xc) as *mut usize) = e;
        crate::comparacao::ramo("cqn_3 entra na lista");
        *((r + 8) as *mut u32) = (read_u32(nacao + 0x80) as usize == e) as u32; // cqn3: o estádio nacional
        *conta += 1;
    }
    ordenar(*lista, *conta as u32, 0x10, 0x85e2f0);
    *((this + 0x18) as *mut u8) = 1;
}

/// Lado a lado à mão: a original no objeto de quem chama, a cópia num rascunho com os mesmos 0x20 bytes de antes e a
/// lista velha zerada (a original já a soltou); iguais os campos (fora o ponteiro +0xc) e as listas (a do rascunho é
/// solta). Sem o HostCountry (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_lista_de_estadios_neutros(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1824);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u32(G_HOST_COUNTRY) == 0 {
        return o(this);
    }
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x20);
    s[0xc..0x18].fill(0);
    o(this);
    let r = s.as_mut_ptr() as usize;
    lista_de_estadios_neutros(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x20);
    let campos = a[..0xc] == s[..0xc] && a[0x10..0x20] == s[0x10..0x20];
    let (la, lb) = (read_u32(this + 0xc) as usize, read_u32(r + 0xc) as usize);
    let n = (read_u32(this + 0x10) as i32).max(0) as usize * 16;
    let listas = if la != 0 && lb != 0 {
        core::slice::from_raw_parts(la as *const u8, n) == core::slice::from_raw_parts(lb as *const u8, n)
    } else {
        la == lb
    };
    if lb != 0 {
        soltar(lb);
    }
    PLACAR.registrar(campos && listas, || format!("lista_de_estadios_neutros: campos {} listas {} (contas {} e {})", campos, listas, read_u32(this + 0x10) as i32, read_u32(r + 0x10) as i32));
}

/// venue qsort: places (+0x3c) of the stadium (+0xc), largest first; without a stadium, 0
/// Original: `stadium_compare_venues_by_capacity` (0x85e350, stadium.cpp, cdecl).
///
/// [pt] { 0x85e350 } (cdecl, stadium.cpp): `stadium_compare_venues_by_capacity(a, b)`, o qsort dos locais na busca do
/// estádio neutro (0x85c8de): pelos lugares (+0x3c) do estádio (+0xc), do maior para o menor; sem um dos estádios, 0.
pub(crate) unsafe extern "C" fn ordem_pela_capacidade(a: usize, b: usize) -> i32 {
    let ea = core::ptr::read_unaligned((a + 0xc) as *const usize);
    if ea == 0 {
        return 0;
    }
    let eb = core::ptr::read_unaligned((b + 0xc) as *const usize);
    if eb == 0 {
        return 0;
    }
    core::ptr::read_unaligned((eb + 0x3c) as *const i32).wrapping_sub(core::ptr::read_unaligned((ea + 0x3c) as *const i32))
}

/// Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_ordem_pela_capacidade(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1214)(a, b);
    let n = ordem_pela_capacidade(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_capacidade({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
