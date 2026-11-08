// ---------- %%LEVA%%a leva: a partida guardada na notícia, o resultado da eliminatória para a notícia e o dia da rodada da copa (comp_util.cpp, Cup.cpp) ----------

/// { 0x4b0010 } (cdecl, comp_util.cpp): a partida (`partida`) refeita de 18 vagas seguidas da
/// `noticia` (0x785830) a partir de `*cursor` (que avança 18): +0xc e +0x10 os clubes, +0x3d, +0x3e,
/// +4 o estádio, +0 a competição, +0x42, +0x32, +0x30, +0x43/+0x44, +0x4b/+0x4c e +0x47/+0x48 os
/// placares, +0x2a, +0x28 e +8. Depois os ponteiros: clubes (0xae23b4, 0x245 bytes) em +0x1c/+0x20,
/// estádio (0xae23b0, 0x4e) em +0x18 e competição (0xae23d0, 0x6b) em +0x14; id −1: 0.
unsafe extern "C" fn comp_util_read_fixture_from_news_slots(noticia: usize, partida: usize, cursor: *mut u8) {
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
unsafe extern "C" fn comp_util_classify_tie_result_for_news(clube: usize, partida: usize) -> u8 {
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
unsafe extern "C" fn cup_set_round_match_day(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(%%I2%%)(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    *((e + 0x27) as *mut u8) = semana as u8; // dia da semana
    *((e + 0x28) as *mut u8) = hora as u8;
    *((e + 0x29) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((e + 0x23) as *mut u32, mando);
    mando
}

unsafe extern "C" fn comparar_comp_util_read_fixture_from_news_slots(noticia: usize, partida: usize, cursor: *mut u8) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut u8)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(partida, 0x4d), (cursor as usize, 1)],
        || o(noticia, partida, cursor), || comp_util_read_fixture_from_news_slots(noticia, partida, cursor));
    PLACAR.registrar(igual, || format!("comp_util_read_fixture_from_news_slots {:x?} x {:x?}", mo, mr));
}

unsafe extern "C" fn comparar_comp_util_classify_tie_result_for_news(clube: usize, partida: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I1%%)(clube, partida);
    let n = comp_util_classify_tie_result_for_news(clube, partida);
    PLACAR.registrar(r == n, || format!("comp_util_classify_tie_result_for_news original={} rust={}", r, n));
    r
}

unsafe extern "C" fn comparar_cup_set_round_match_day(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(%%I2%%);
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return o(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x23, 7)], || o(rodadas, rodada, vaga, semana, hora, codigo, mando),
        || cup_set_round_match_day(rodadas, rodada, vaga, semana, hora, codigo, mando));
    PLACAR.registrar(igual && r == n, || format!("cup_set_round_match_day({}, {}) {:x?} x {:x?}", rodada, vaga, mo, mr));
    r
}
