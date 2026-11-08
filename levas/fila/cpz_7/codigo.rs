// ---------- %%LEVA%%a leva: os registros de partida e de nome: a partida vista por um clube, a referência ao nome da pessoa e o código do resultado (record_utils.cpp) ----------

unsafe fn cpz7_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz7_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x7e4750 } (cdecl, record_utils.cpp): `fill_match_record_from_fixture(jogo, clube, saida)`, da estatística da tática
/// (0x896290, no fim de cada partida sem humano) e dos recordes. Jogo, clube ou saída nulos: a caixa (linhas 0x29, 0x31,
/// 0x39) e 0. Senão a data (+0, 0x417c10 da Date(dia +0x2a, ano +0x28) da 0x549ef0), +8 o id do jogo (+0), +0xc a
/// palavra +0x32, +0xe a +0x30, +0x10 o lado (0x4afa80, cdecl(clube, jogo)), +0x11 o id do clube, +0x1b o dword +8 do
/// jogo; mandante (+0x1c): +0x15 o +0x10 do jogo e o placar (+0x19 pró, +0x1a contra) pelo da prorrogação (+0x47/+0x48)
/// se ele não for negativo, senão o normal (+0x43/+0x44); visitante: +0x15 o +0xc e o placar ao contrário. Devolve 1.
unsafe extern "C" fn fill_match_record_from_fixture(jogo: usize, clube: usize, saida: usize) -> u32 {
    if jogo == 0 {
        show_error_at(0xa7a088, 0x29);
        return 0;
    }
    if clube == 0 {
        show_error_at(0xa7a088, 0x31);
        return 0;
    }
    if saida == 0 {
        show_error_at(0xa7a088, 0x39);
        return 0;
    }
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let lado: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4afa80usize);
    let (mut t0, mut t1) = ([0u8; 8], [0u8; 8]);
    let (p0, p1) = (t0.as_mut_ptr() as usize, t1.as_mut_ptr() as usize);
    let d = data_nova(p0, cpz7_w(jogo + 0x2a) as u32, cpz7_w(jogo + 0x28) as u32);
    copiar(saida, p1, d);
    let s = |o: usize| (saida + o) as *mut u8;
    core::ptr::write_unaligned(s(8) as *mut u32, cpz7_d(jogo));
    core::ptr::write_unaligned(s(0xc) as *mut u16, cpz7_w(jogo + 0x32));
    core::ptr::write_unaligned(s(0xe) as *mut u16, cpz7_w(jogo + 0x30));
    *s(0x10) = lado(clube, jogo);
    core::ptr::write_unaligned(s(0x11) as *mut u32, cpz7_d(clube));
    core::ptr::write_unaligned(s(0x1b) as *mut u32, cpz7_d(jogo + 8));
    if clube as u32 == cpz7_d(jogo + 0x1c) { // cpz7: o clube é o mandante
        core::ptr::write_unaligned(s(0x15) as *mut u32, cpz7_d(jogo + 0x10));
        if cpz7_b(jogo + 0x47) as i8 > -1 {
            *s(0x19) = cpz7_b(jogo + 0x47);
            *s(0x1a) = cpz7_b(jogo + 0x48);
        } else {
            crate::comparison::branch("cpz_7 placar do mandante");
            *s(0x19) = cpz7_b(jogo + 0x43); // cpz7: os gols do mandante
            *s(0x1a) = cpz7_b(jogo + 0x44);
        }
    } else {
        core::ptr::write_unaligned(s(0x15) as *mut u32, cpz7_d(jogo + 0xc));
        if cpz7_b(jogo + 0x48) as i8 > -1 {
            *s(0x19) = cpz7_b(jogo + 0x48);
            *s(0x1a) = cpz7_b(jogo + 0x47);
        } else {
            crate::comparison::branch("cpz_7 placar do visitante");
            *s(0x19) = cpz7_b(jogo + 0x44);
            *s(0x1a) = cpz7_b(jogo + 0x43); // cpz7: os gols sofridos pelo visitante
        }
    }
    1
}

/// Comparador da 0x7e4750: lado a lado (leve) no registro de saída (0x1f bytes) e na devolução; ponteiro nulo: a caixa,
/// só a original.
unsafe extern "C" fn comparar_fill_match_record_from_fixture(jogo: usize, clube: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I0%%);
    if jogo == 0 || clube == 0 || saida == 0 {
        return o(jogo, clube, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(saida, 0x1f)], || o(jogo, clube, saida), || fill_match_record_from_fixture(jogo, clube, saida));
    SCOREBOARD.record(igual && r == n, || format!("fill_match_record_from_fixture(jogo {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}", jogo, clube, r, n, mo, mr));
    r
}

/// O id do nome (+0x33 do registro de nome) ou −1 sem nome.
unsafe fn name_id(p: u32) -> u32 {
    if p != 0 { cpz7_d(p as usize + 0x33) } else { u32::MAX }
}

/// { 0x7e4920 } (cdecl, record_utils.cpp): `fill_staff_name_ref(pessoa, saida)`, dos recordes do clube e da competição,
/// da notícia da suspensão, do prêmio e da busca de jogadores. Pessoa ou saída nulas: a caixa (linhas 0x80, 0x88) e 0.
/// Senão +0, +4, +8 os ids dos nomes (+4, +8, +0xc da pessoa; −1 sem nome), +0xc o id da pessoa e +0x10 a data de
/// nascimento (+0x10, 0x417c10); devolve 1.
unsafe extern "C" fn fill_staff_name_ref(pessoa: usize, saida: usize) -> u32 {
    if pessoa == 0 {
        show_error_at(0xa7a088, 0x80);
        return 0;
    }
    if saida == 0 {
        show_error_at(0xa7a088, 0x88);
        return 0;
    }
    for k in 0..3 {
        core::ptr::write_unaligned((saida + 4 * k) as *mut u32, name_id(cpz7_d(pessoa + 4 + 4 * k)));
    }
    core::ptr::write_unaligned((saida + 0xc) as *mut u32, cpz7_d(pessoa));
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copiar(saida + 0x10, t.as_mut_ptr() as usize, pessoa + 0x10);
    1
}

/// Comparador da 0x7e4920: lado a lado (leve) na referência (0x18 bytes) e na devolução; ponteiro nulo: só a original.
unsafe extern "C" fn comparar_fill_staff_name_ref(pessoa: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%);
    if pessoa == 0 || saida == 0 {
        return o(pessoa, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(saida, 0x18)], || o(pessoa, saida), || fill_staff_name_ref(pessoa, saida));
    SCOREBOARD.record(igual && r == n, || format!("fill_staff_name_ref(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x7e56d0 } (cdecl, record_utils.cpp): `match_record_result_code(registro)`, da estatística da competição
/// (0x49f570, a cada partida). Registro nulo: a caixa (linha 0x25a) e 0. Pró (+0x19) > contra (+0x1a), com sinal: 3;
/// empate com gols: 2; empate sem gols: 1; derrota: −3 (só o byte conta: quem chama lê al).
unsafe extern "C" fn match_record_result_code(registro: usize) -> u8 {
    if registro == 0 {
        show_error_at(0xa7a088, 0x25a);
        return 0;
    }
    let (p, c) = (cpz7_b(registro + 0x19) as i8, cpz7_b(registro + 0x1a) as i8);
    if p > c {
        3
    } else if p == c && p > 0 {
        2
    } else if p == c {
        1
    } else {
        (-3i8) as u8
    }
}

/// Comparador da 0x7e56d0: pelo byte devolvido (só lê); registro nulo: só a original.
unsafe extern "C" fn comparar_match_record_result_code(registro: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%);
    if registro == 0 {
        return o(registro);
    }
    let r = o(registro);
    let n = match_record_result_code(registro);
    SCOREBOARD.record(r == n, || format!("match_record_result_code(registro {:x}) original={} rust={}", registro, r as i8, n as i8));
    r
}
