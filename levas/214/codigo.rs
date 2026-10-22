// ---------- 214a leva: a ordem das linhas da tabela, a grade de turnos da liga e a margem do último dia (league.cpp) ----------

/// { 0x683210 } (cdecl, league.cpp): comparação do qsort das linhas da tabela (`a` e `b`: [+0] clube).
/// Liga corrente em [0xb679fc], a coluna da tabela em +0xad (14 bytes por coluna nas linhas). Linha
/// sem clube: 0. Com a comparação particular em [0xb67a00] e resposta dela não nula: ela de novo. Senão
/// pontos (+0x19) decrescentes; sem jogos (+0x11) dos dois, pelo id do clube. Depois os quatro
/// critérios da liga (+0xc5): 1 saldo, 2 gols pró, 3 byte +0x12, 4 byte +4, 5 palavra +0x31 (na
/// coluna 0, critério 6 = byte +4 e fim); na coluna 0 com os dois no número de jogos da liga (+0xa7), o
/// id em +0x69 dos clubes; por fim o id do clube.
unsafe extern "C" fn ordem_das_linhas_da_tabela(a: usize, b: usize) -> i32 {
    let liga = *(0xb679fc as *const usize);
    let coluna = core::ptr::read_unaligned((liga + 0xad) as *const i32);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return 0;
    }
    let propria = *(0xb67a00 as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(0xb67a00 as *const usize));
            return f(a, b);
        }
    }
    let c = (coluna as usize).wrapping_mul(14);
    let w = |r: usize, o: usize| core::ptr::read_unaligned(r.wrapping_add(c).wrapping_add(o) as *const i16) as i32;
    let y = |r: usize, o: usize| *(r.wrapping_add(c).wrapping_add(o) as *const i8) as i32;
    let (pa, pb) = (w(a, 0x19), w(b, 0x19));
    if pa != pb {
        return pb - pa; // pontos
    }
    let id = |r: usize| *(*(r as *const usize) as *const i32);
    if y(a, 0x11) == 0 && y(b, 0x11) == 0 {
        return id(a).wrapping_sub(id(b));
    }
    for k in 0..4usize {
        let criterio = *((liga + 0xc5 + k) as *const i8);
        if coluna == 0 && criterio == 6 {
            return *((a + 4) as *const i8) as i32 - *((b + 4) as *const i8) as i32;
        }
        if criterio == 0 {
            break;
        }
        let (va, vb) = match criterio {
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (y(a, 0x12), y(b, 0x12)),
            4 => (*((a + 4) as *const i8) as i32, *((b + 4) as *const i8) as i32),
            5 => (core::ptr::read_unaligned((a + 0x31) as *const i16) as i32, core::ptr::read_unaligned((b + 0x31) as *const i16) as i32),
            _ => continue,
        };
        if va != vb {
            return vb - va;
        }
    }
    if coluna == 0 {
        let jogos = core::ptr::read_unaligned((liga + 0xa7) as *const i16);
        if *((a + 0x11) as *const i8) as i16 == jogos && *((b + 0x11) as *const i8) as i16 == jogos {
            let xb = core::ptr::read_unaligned((*(b as *const usize) + 0x69) as *const usize);
            let xa = core::ptr::read_unaligned((*(a as *const usize) + 0x69) as *const usize);
            if xb != 0 && xa != 0 {
                let d = (*(xb as *const i32)).wrapping_sub(*(xa as *const i32));
                if d != 0 {
                    return d;
                }
            }
        }
    }
    id(a).wrapping_sub(id(b))
}

/// { 0x6844b0 } (stdcall, ret 8, league.cpp): a grade de turnos da liga de `n` clubes: `grade[1..=n]`
/// são linhas de inteiros (colunas 1..n−1 = rodadas) com o adversário de cada clube na rodada (positivo
/// em casa, negativo fora). Zera tudo; o clube `n` enfrenta 1..n−1 numa rodada que anda de 2 em 2
/// (casa para a primeira metade); depois cada clube k de n−1 até 2 preenche as vagas vazias com os
/// clubes 1..n, alternando o mando a partir da paridade de n−1−k. Devolve 1.
unsafe extern "stdcall" fn montar_grade_de_turnos(grade: *const *mut i32, n: i32) -> u32 {
    let celula = |i: i32, j: i32| (*grade.offset(i as isize)).offset(j as isize);
    for i in 1..=n {
        for j in 1..n {
            *celula(i, j) = 0;
        }
    }
    let metade = n / 2;
    let mut col = n - 1;
    for i in 1..n {
        if i <= metade {
            *celula(i, col) = n;
            *celula(n, col) = -i;
        } else {
            *celula(i, col) = -n;
            *celula(n, col) = i;
        }
        col = if col == n - 2 { 1 } else if col == n - 1 { 2 } else { col + 2 };
    }
    let mut k = n - 1;
    while k >= 2 {
        let menos_k = -k;
        let mut fora = (n - 1 + menos_k) % 2;
        let mut col = k - 1;
        let mut contra = -1;
        for j in 1..=n {
            let c = celula(j, col);
            if *c == 0 {
                if fora != 0 {
                    *c = k; // vaga preenchida
                    *celula(k, col) = contra;
                } else {
                    *c = menos_k;
                    *celula(k, col) = j;
                }
                fora = 1 - fora;
            }
            col = if col == n - 1 { 1 } else { col + 1 };
            contra -= 1;
        }
        k -= 1;
    }
    1
}

/// A linha `i` (59 bytes) da tabela [+0xb1] da liga.
unsafe fn linha_da_liga(liga: usize, i: i32) -> usize {
    (*((liga + 0xb1) as *const usize)).wrapping_add((i as isize * 59) as usize)
}

/// A margem de gols (+1, mínimo 1, senão 0xff) que falta para alcançar a linha `alvo` pelo critério da
/// liga (+0xc5): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15).
unsafe fn margem_para(liga: usize, linha: usize, alvo: usize) -> u8 {
    let b = |r: usize, o: usize| *((r + o) as *const u8);
    let c = if *((liga + 0xc5) as *const u8) == 1 {
        b(alvo, 0x15).wrapping_sub(b(alvo, 0x17)).wrapping_sub(b(linha, 0x15)).wrapping_add(b(linha, 0x17))
    } else {
        b(alvo, 0x15).wrapping_sub(b(linha, 0x15))
    }
    .wrapping_add(1);
    if (c as i8) < 1 { 0xff } else { c }
}

/// { 0x68a4d0 } (thiscall, ret 8, league.cpp): no último dia (jogos +0x11 ≥ rodadas da liga +0xa9 − 1)
/// e linha sem marca (+0x37 = 0xff), a margem de gols que o clube precisa para o título, as vagas de
/// cima (+0xbe/+0xbf) ou fugir das de baixo (+0xc0/+0xc1); 0xff se nenhuma. Com pontos iguais a quem
/// está à frente sem margem possível, grava 0x14 em `saida` (decisivo) e devolve 0 ou 0xff. Só o al é
/// devolvido (quem chama usa al).
unsafe extern "thiscall" fn margem_do_ultimo_dia(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let p = |r: usize| core::ptr::read_unaligned((r + 0x19) as *const i16) as i32;
    let extra = *((liga + 0xc2) as *const i8) as i32;
    let pos = *((linha + 4) as *const i8);
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let tabela = *((liga + 0xb1) as *const usize);
    let criterio = *((liga + 0xc5) as *const u8);
    if (*((linha + 0x11) as *const i8) as i32) < core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32 - 1 || *((linha + 0x37) as *const u8) != 0xff {
        return 0xff; // fora do último dia
    }
    // a linha `alvo` à frente, sem marca: pontos + extra acima dela, 0xff; iguais (extra 0), a margem
    let ver = |alvo: usize| -> Option<u8> {
        if p(linha) + extra > p(alvo) {
            return Some(0xff);
        }
        if extra == 0 && (criterio == 1 || criterio == 2) {
            return Some(margem_para(liga, linha, alvo));
        }
        None
    };
    // a primeira linha sem marca de `de` até `ate` (exclusive), com pontos + extra contra os da `linha`
    let decidir = |de: i16, ate: i16| -> Option<u8> {
        let mut i = de;
        while i < ate {
            let r = linha_da_liga(liga, i as i32);
            if *((r + 0x37) as *const u8) == 0xff {
                let v = p(r) + extra;
                if v > p(linha) {
                    *saida = 0x14;
                    return Some(0xff);
                }
                if v != p(linha) {
                    return Some(0xff);
                }
                *saida = 0x14;
                return Some(0);
            }
            i += 1;
        }
        None
    };
    if *((liga + 0xd9) as *const u8) & 1 != 0 && *((tabela + 0x37) as *const u8) != 5 {
        if pos > 0 {
            let v = p(linha) + extra;
            if v > p(tabela) {
                return 0xff;
            }
            if v == p(tabela) && (criterio == 1 || criterio == 2) {
                return margem_para(liga, linha, tabela);
            }
        } else if clubes > 1 {
            let v = core::ptr::read_unaligned((tabela + 0x54) as *const i16) as i32 + extra;
            if v > p(tabela) {
                *saida = 0x14;
                return 0xff;
            }
            if v != p(tabela) {
                return 0xff;
            }
            *saida = 0x14;
            return 0;
        }
    }
    let (s1, s2) = (*((liga + 0xbe) as *const u8), *((liga + 0xbf) as *const u8));
    if s1 != 0 || s2 != 0 {
        let vagas = s2.wrapping_add(s1) as i8;
        if pos < vagas {
            if let Some(r) = decidir(vagas as i16, clubes) {
                return r;
            }
        } else {
            let mut i = 0i16;
            while i < vagas as i16 {
                let r = linha_da_liga(liga, i as i32);
                if *((r + 0x37) as *const u8) == 0xff {
                    if let Some(m) = ver(r) {
                        return m;
                    }
                }
                i += 1;
            }
        }
    }
    let (d1, d2) = (*((liga + 0xc1) as *const u8), *((liga + 0xc0) as *const u8));
    if d1 == 0 && d2 == 0 {
        return 0xff;
    }
    let descem = d2.wrapping_add(d1) as i8;
    let limite = clubes as i32 - descem as i32;
    if (pos as i32) < limite {
        return decidir(clubes.wrapping_sub(descem as i16), clubes).unwrap_or(0xff);
    }
    let mut i = 0i16;
    while (i as i32) < limite {
        let r = linha_da_liga(liga, i as i32);
        if *((r + 0x37) as *const u8) == 0xff {
            if let Some(m) = ver(r) {
                return m;
            }
        }
        i += 1;
    }
    0xff
}

unsafe extern "C" fn comparar_ordem_das_linhas_da_tabela(a: usize, b: usize) -> i32 {
    // a comparação particular de [0xb67a00] (se houver) roda nos dois lados: só lê as linhas
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(534)(a, b);
    let n = ordem_das_linhas_da_tabela(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_linhas_da_tabela original={} rust={}", r, n));
    r
}

unsafe extern "stdcall" fn comparar_montar_grade_de_turnos(grade: *const *mut i32, n: i32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(*const *mut i32, i32) -> u32>(535);
    let regioes: Vec<(usize, usize)> = (1..=n.max(0)).map(|i| (*grade.offset(i as isize) as usize + 4, (n.max(1) as usize - 1) * 4)).collect();
    let (r, s, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(grade, n), || montar_grade_de_turnos(grade, n));
    PLACAR.registrar(igual && r == s, || format!("montar_grade_de_turnos({}) {:x?} x {:x?}", n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_margem_do_ultimo_dia(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8>(536);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 1)], || o(liga, linha, saida),
        || margem_do_ultimo_dia(liga, linha, saida));
    PLACAR.registrar(igual && r == n, || format!("margem_do_ultimo_dia original={:#x} rust={:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 214a leva: a partida guardada na notícia, o resultado da eliminatória para a notícia e o dia da rodada da copa (comp_util.cpp, Cup.cpp) ----------

/// { 0x4b0010 } (cdecl, comp_util.cpp): a partida (`partida`) refeita de 18 vagas seguidas da
/// `noticia` (0x785830) a partir de `*cursor` (que avança 18): +0xc e +0x10 os clubes, +0x3d, +0x3e,
/// +4 o estádio, +0 a competição, +0x42, +0x32, +0x30, +0x43/+0x44, +0x4b/+0x4c e +0x47/+0x48 os
/// placares, +0x2a, +0x28 e +8. Depois os ponteiros: clubes (0xae23b4, 0x245 bytes) em +0x1c/+0x20,
/// estádio (0xae23b0, 0x4e) em +0x18 e competição (0xae23d0, 0x6b) em +0x14; id −1: 0.
unsafe extern "C" fn partida_das_vagas_da_noticia(noticia: usize, partida: usize, cursor: *mut u8) {
    // ponytail: a original empilha o eax inteiro (resto da vaga anterior nos bits altos); a 0x785830 só lê o byte
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x785830usize);
    let mut v = *cursor;
    let mut ler = || {
        let r = vaga(noticia, v as u32);
        v = v.wrapping_add(1);
        r
    };
    let d = |o: usize, x: u32| core::ptr::write_unaligned((partida + o) as *mut u32, x);
    let w = |o: usize, x: u32| core::ptr::write_unaligned((partida + o) as *mut u16, x as u16);
    let b = |o: usize, x: u32| *((partida + o) as *mut u8) = x as u8;
    d(0xc, ler());
    d(0x10, ler());
    b(0x3d, ler());
    b(0x3e, ler());
    d(4, ler());
    d(0, ler());
    b(0x42, ler());
    w(0x32, ler());
    w(0x30, ler());
    b(0x43, ler());
    b(0x44, ler());
    b(0x4b, ler());
    b(0x4c, ler());
    b(0x47, ler());
    b(0x48, ler());
    w(0x2a, ler());
    w(0x28, ler());
    d(8, ler());
    *cursor = v;
    let id = |o: usize| core::ptr::read_unaligned((partida + o) as *const u32);
    let ponteiro = |o: usize, tabela: usize, tamanho: u32| {
        let i = id(o);
        if i == 0xffff_ffff { 0 } else { (*(tabela as *const u32)).wrapping_add(i.wrapping_mul(tamanho)) }
    };
    d(0x1c, ponteiro(0xc, 0xae23b4, 0x245)); // clube da casa
    d(0x20, ponteiro(0x10, 0xae23b4, 0x245));
    d(0x18, ponteiro(4, 0xae23b0, 0x4e));
    d(0x14, ponteiro(0, 0xae23d0, 0x6b));
}

/// { 0x4b0890 } (cdecl, comp_util.cpp): a categoria (1 a 7) do resultado da eliminatória para o
/// `clube` na `partida`: a margem final (placar +0x4b/+0x4c, ou +0x47/+0x48, ou +0x43/+0x44, o
/// primeiro sem 0xff) do lado do clube (da casa = +0x1c) e as reputações (+0x80 / 500) dos clubes.
/// 1 goleada, 2 e 3 vitórias contra mais fortes, 4 comum, 5 e 7 derrotas para mais fracos, 6
/// zebra. Só o al é fiel (quem chama usa al).
unsafe extern "C" fn resultado_da_eliminatoria_na_noticia(clube: usize, partida: usize) -> u8 {
    let casa = *((partida + 0x1c) as *const usize);
    let fora = *((partida + 0x20) as *const usize);
    let h = (core::ptr::read_unaligned((casa + 0x80) as *const i16) as i32 / 500) as i8 as i32;
    let a = (core::ptr::read_unaligned((fora + 0x80) as *const i16) as i32 / 500) as i8 as i32;
    let p = |o: usize| *((partida + o) as *const u8);
    let (g1, g2) = if p(0x4b) != 0xff {
        (p(0x4b), p(0x4c))
    } else if p(0x47) != 0xff {
        (p(0x47), p(0x48))
    } else {
        (p(0x43), p(0x44))
    };
    let m = g1.wrapping_sub(g2) as i8;
    // do ponto de vista do clube: `eu` e `ele` as reputações, `vitoria` a margem a favor
    let (eu, ele) = if clube == casa { (h, a) } else { (a, h) };
    let vitoria = if clube == casa { m } else { m.wrapping_neg() };
    if vitoria == 0 {
        if eu < ele - 16 || eu - 15 > ele {
            return 6;
        }
        if eu < ele - 11 {
            return 2;
        }
        if eu < ele - 3 {
            return 3;
        }
        if eu - 5 > ele {
            return 5;
        }
        return if eu - 11 <= ele { 4 } else { 7 };
    }
    // a zebra: a casa tolera 8 / 7, o visitante 13 / 7 de diferença na vitória por 1 ou 2
    let (z1, z2) = if clube == casa { (8, 2) } else { (13, 7) };
    if vitoria > 0 {
        if (vitoria < 3 && eu < ele - z1) || (vitoria > 2 && eu < ele - z2) {
            return 6;
        }
        if vitoria > 4 {
            return 1;
        }
        if vitoria < 2 {
            if eu < ele - 3 {
                return 2;
            }
            if eu < ele {
                return 3;
            }
        } else if vitoria > 2 && eu < ele {
            return 2;
        }
        return 4;
    }
    let d = vitoria.wrapping_neg();
    if (d < 3 && ele < eu - 7) || (d > 2 && ele < eu - 2) {
        return 6;
    }
    if d < 2 {
        if eu - 6 > ele {
            return 7;
        }
        if eu - 2 > ele {
            return 5;
        }
    }
    4 // sem destaque
}

/// { 0x521f10 } (cdecl, Cup.cpp): o dia `vaga` (0 a 7) da rodada `rodada` (registros de 104 bytes):
/// em 7 bytes a partir de +0x23, o mando (4 bytes), o dia da semana (−1 a 6), o horário (−1 a 2) e o
/// código (−1 a 3). Fora da faixa: a original (caixa de erro e fim do jogo). Devolve o mando.
unsafe extern "C" fn dia_da_rodada_da_copa(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(539)(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    *((e + 0x27) as *mut u8) = semana as u8; // dia da semana
    *((e + 0x28) as *mut u8) = hora as u8;
    *((e + 0x29) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((e + 0x23) as *mut u32, mando);
    mando
}

unsafe extern "C" fn comparar_partida_das_vagas_da_noticia(noticia: usize, partida: usize, cursor: *mut u8) {
    let o = orig::<unsafe extern "C" fn(usize, usize, *mut u8)>(537);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(partida, 0x4d), (cursor as usize, 1)],
        || o(noticia, partida, cursor), || partida_das_vagas_da_noticia(noticia, partida, cursor));
    PLACAR.registrar(igual, || format!("partida_das_vagas_da_noticia {:x?} x {:x?}", mo, mr));
}

unsafe extern "C" fn comparar_resultado_da_eliminatoria_na_noticia(clube: usize, partida: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(538)(clube, partida);
    let n = resultado_da_eliminatoria_na_noticia(clube, partida);
    PLACAR.registrar(r == n, || format!("resultado_da_eliminatoria_na_noticia original={} rust={}", r, n));
    r
}

unsafe extern "C" fn comparar_dia_da_rodada_da_copa(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(539);
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return o(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x23, 7)], || o(rodadas, rodada, vaga, semana, hora, codigo, mando),
        || dia_da_rodada_da_copa(rodadas, rodada, vaga, semana, hora, codigo, mando));
    PLACAR.registrar(igual && r == n, || format!("dia_da_rodada_da_copa({}, {}) {:x?} x {:x?}", rodada, vaga, mo, mr));
    r
}

// ---------- 214a leva: o mapa de casas da formação, a nota do clube para o jovem e a partida assistida pelo humano (formation.cpp, player_regen.cpp, human_manager.cpp) ----------

/// { 0x5b7160 } (thiscall, formation.cpp): o mapa de casas da formação. Para cada uma das 22 linhas
/// k (11 casas × 2 fases) e bloco 3 × 4 (a18, a14) com a posição (i = 12k + 4·a18 + a14) em +0x164
/// (coluna, palavra) e +0x166 (linha), as 3 × 3 células do bloco ganham a coluna × 9 / 27 no nibble
/// baixo e a linha × 12 / 36 no alto da tabela +0x5eb (12 por fila, 9 filas por k); o espelho em +0xf9e
/// (fila e coluna invertidas) recebe (0xb0 − linha × 16) com (0xf8 − coluna) & 0xf. Devolve 1.
unsafe extern "thiscall" fn mapa_de_casas_da_formacao(this: usize) -> u32 {
    for k in 0..22i32 {
        for a18 in 0..3i32 {
            for a14 in 0..4i32 {
                let i = (12 * k + 4 * a18 + a14) as usize;
                for y in 3 * a18..3 * a18 + 3 {
                    for x in 3 * a14..3 * a14 + 3 {
                        let coluna = core::ptr::read_unaligned((this + i * 4 + 0x164) as *const i16) as i32;
                        let c = (coluna * 9 / 27) as u8;
                        let p = (this as isize + 0x5eb + ((9 * k + y) * 12 + x) as isize) as *mut u8;
                        *p = (*p & 0xf0) | (c & 0xf);
                        let linha = core::ptr::read_unaligned((this + i * 4 + 0x166) as *const i16) as i32;
                        let d = ((linha * 12 / 36) as u8) << 4;
                        *p = (c & 0xf) | d;
                        let q = (this as isize + 0xf9e + ((9 * k - y) * 12 - x) as isize) as *mut u8;
                        *q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho
                    }
                }
            }
        }
    }
    1
}

/// { 0x7ad220 } (stdcall, ret 0xc, player_regen.cpp): a nota do `clube` para o jovem `jogador`
/// (sorteio(nota)): 5000 + reputação (+0x80) menos as distâncias dela à habilidade (ficha +0x61: +0xb,
/// e +7, +5 × 50); com +7 acima de sorteio(20) + 100, o bônus de juniores (+0x7f, 20 = garantido,
/// senão sorteio(+0x7f) > sorteio(20)); clube de competição ativa (0xadadfc) com +7 > 50: o estado do
/// clube humano (0xae2d14) e a reputação contra o elenco (`elenco`); o treinador de juniores (+0xcf
/// → +0x69) do clube do computador (0x600a50) ou o bônus do humano; a mesma nação (+0x1a/+0x53) e os
/// clubes galeses (Swansea, Cardiff, Wrexham) e a Escócia. Só os 16 bits baixos da nota contam.
unsafe extern "stdcall" fn nota_do_clube_para_o_jovem(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha = u(jogador, 0x61);
    let rep = w(clube, 0x80) as i32;
    let mut s = rep
        .wrapping_sub((w(ficha, 0xb) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 7).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 5).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_add(0x1388);
    let juniores = *((clube + 0x7f) as *const i8);
    if w(ficha, 7) as i32 > sorteio(0x14) + 0x64 {
        if juniores == 0x14 {
            s = s.wrapping_add(rep / 2 + 0x1b58);
        } else if sorteio(juniores as i32) > sorteio(0x14) {
            s = s.wrapping_add(rep / 2 + (juniores as i32 + 0x64) * 50);
        }
    }
    let comp = u(clube, 0x57);
    if w(ficha, 7) > 0x32 && comp != 0 && *((*(0xadadfc as *const usize) + 4 * *(comp as *const u32) as usize) as *const usize) != 0 {
        let estados = *(0xae2d14 as *const usize);
        if humano(clube, 1, 1, 0, 0) != 0 && estados != 0 && w(ficha, 7) > 0x78 {
            let e = *((estados + 6 * *(clube as *const u32) as usize + 4) as *const i8);
            if e < 0 {
                s = s.wrapping_add(0x1f4);
            } else if e == 0 {
                s = s.wrapping_add(0xfa);
            }
        }
        let r = w(clube, 0x80);
        s = s.wrapping_add(if r > 0x1676 {
            if elenco < 0x28 { 0x3e8 } else if elenco < 0x2d { 0x2ee } else { 0xfa }
        } else if r > 0x109a {
            if elenco < 0x23 { 0x2ee } else if elenco < 0x28 { 0x1f4 } else { 0xfa }
        } else {
            0xfa
        });
    }
    let tecnico = u(clube, 0xcf);
    let x = if tecnico != 0 { u(tecnico, 0x69) } else { 0 };
    if x != 0 && humano(clube, 1, 1, 0, 0) == 0 {
        s = s.wrapping_add((*((x + 0x22) as *const i8) as i32 - 10) * 30);
        if w(ficha, 7) as i32 > sorteio(0x4b) + 0x64 {
            let v = chance(x, *((x + 0x18) as *const i8)) as i8;
            s = s.wrapping_add((v as i32 - 10) * 60);
        }
        let v = chance(x, *((x + 0x18) as *const i8)) as i8;
        let t = v as i32 + *((x + 0x22) as *const i8) as i32 + juniores as i32;
        if t > 0x28 && w(clube, 0x80) > 0x109a && w(ficha, 7) > 0x50 {
            if (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            } else {
                s = s.wrapping_add(0xfa);
            }
        }
    } else if humano(clube, 1, 1, 0, 0) != 0 {
        s = s.wrapping_add(0xc8);
        if w(ficha, 7) > 0x78 {
            if juniores == 0x14 && (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            }
            s = s.wrapping_add(juniores as i32 * 25);
        }
    }
    let nacao = u(jogador, 0x1a);
    let nacao_do_clube = u(clube, 0x53);
    if nacao == nacao_do_clube {
        s = s.wrapping_add(0x3e8); // mesma nação
    }
    let g = |a: usize| *(a as *const u32);
    let (gales, inglaterra, escocia) = (g(0x9cf510), g(0x9cf2e4), g(0x9cf45c));
    let id = g(clube);
    if id == g(0x9d0430) || id == g(0x9d0434) || (id == g(0x9d043c) && nacao != 0) {
        let n = g(nacao);
        if n == gales {
            s = s.wrapping_add(0x3e8);
        } else if n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        }
    }
    if g(nacao_do_clube) == escocia {
        let n = g(nacao);
        if n == gales || n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        } else if n == escocia {
            s = s.wrapping_add(0x3e8);
        }
    }
    sorteio(s as i16 as i32)
}

/// O registro (42735 bytes em [this]) da partida que o `humano` (ou o da tela, 0xb74c26 + 0x300 ×
/// [0xb77c7a]) assiste: Ok(0) sem humano; Err se o índice (id − [0xae2364] + 0x10) sai de 0..[this+4].
unsafe fn registro_da_partida_assistida(this: usize, humano: usize) -> Result<usize, ()> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return Ok(0);
    }
    let i = (*(h as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(());
    }
    Ok((*(this as *const usize)).wrapping_add((i as usize).wrapping_mul(42735)))
}

/// { 0x5fe290 } (thiscall, ret 8, human_manager.cpp): se a `partida` é a que o humano escolheu
/// assistir: o registro dele (+0x9ad4..) com a mesma competição (+0x3f), datas (+0x2a, +0x28), clube
/// (+0), +0x34 e os dois lados (+0xc, +0x10). Partida nula ou humano fora da faixa: a original (caixa
/// de erro e 0).
unsafe extern "thiscall" fn partida_assistida_pelo_humano(this: usize, partida: usize, humano: usize) -> u32 {
    let r = match registro_da_partida_assistida(this, humano) {
        Err(()) => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542)(this, partida, humano),
        Ok(_) if partida == 0 => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542)(this, partida, humano),
        Ok(0) => return 0,
        Ok(r) => r,
    };
    let b = |p: usize, o: usize| *((p + o) as *const u8);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let d = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u32);
    let mesma = b(partida, 0x3f) == b(r, 0x9b13)
        && w(partida, 0x2a) == w(r, 0x9afe)
        && w(partida, 0x28) == w(r, 0x9afc)
        && d(partida, 0) == d(r, 0x9ad4)
        && w(partida, 0x34) == w(r, 0x9b08)
        && d(partida, 0xc) == d(r, 0x9ae0)
        && d(partida, 0x10) == d(r, 0x9ae4);
    if mesma { 1 } else { 0 } // outra partida
}

unsafe extern "thiscall" fn comparar_mapa_de_casas_da_formacao(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(540);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x5eb, 0x187b - 0x5eb)], || o(this), || mapa_de_casas_da_formacao(this));
    PLACAR.registrar(igual && r == n, || format!("mapa_de_casas_da_formacao {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "stdcall" fn comparar_nota_do_clube_para_o_jovem(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, i8) -> i32>(541);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(jogador, clube, elenco), || crate::aleatorio::sem_contar(|| nota_do_clube_para_o_jovem(jogador, clube, elenco)));
    PLACAR.registrar(igual && r == n, || format!("nota_do_clube_para_o_jovem(elenco {}) original={} rust={} sorteio igual={}", elenco, r, n, igual));
    r
}

unsafe extern "thiscall" fn comparar_partida_assistida_pelo_humano(this: usize, partida: usize, humano: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542);
    let r = o(this, partida, humano);
    // ponytail: os casos de erro (caixa) só na original
    if partida == 0 || registro_da_partida_assistida(this, humano).is_err() {
        return r;
    }
    let n = partida_assistida_pelo_humano(this, partida, humano);
    PLACAR.registrar(r == n, || format!("partida_assistida_pelo_humano original={} rust={}", r, n));
    r
}
