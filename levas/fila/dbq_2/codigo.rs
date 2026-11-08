// ---------- %%LEVA%%a leva: o clube rejeitado da pessoa, o esfriar das relações pessoais e a lista das pessoas rejeitadas (Database.cpp) ----------

unsafe fn dbq2_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O estado do sorteio e 24 bytes a partir de `base` (nulo: só o sorteio).
unsafe fn dbq2_regioes(base: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if base != 0 {
        reg.push((base, 24));
    }
    reg
}

/// { 0x547e70 } (cdecl, Database.cpp): `staff_add_disliked_club(pessoa, clube, forcar)`: põe o clube entre os três
/// rejeitados da pessoa (preferências +0x65: +0x10/+0x14/+0x18). O clube já rejeita a pessoa (+0xa7..+0xaf): nada;
/// o clube é favorito (+4..+0xc): deixa de ser e para; senão a primeira vaga (sem conferir se já está entre os
/// rejeitados, como a original); sem vaga, com `forcar`, a vaga sorteada (0x90cfc0, 3). Pessoa, preferências ou clube
/// nulos: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0, duas chamadas), 0x5a5980, MANAGER_MANAGER
/// (0x69c030) e outras.
unsafe extern "C" fn staff_add_disliked_club(pessoa: usize, clube: usize, forcar: u32) {
    if pessoa == 0 {
        return;
    }
    let pref = dbq2_u32(pessoa + 0x65);
    if pref == 0 || clube == 0 {
        return;
    }
    if (0..3).any(|k| dbq2_u32(clube + 0xa7 + 4 * k) == pessoa) {
        return;
    }
    if let Some(k) = (0..3).find(|&k| dbq2_u32(pref + 4 + 4 * k) == clube) {
        *((pref + 4 + 4 * k) as *mut usize) = 0;
        return;
    }
    let lista = pref + 0x10; // os clubes rejeitados
    if let Some(k) = (0..3).find(|&k| dbq2_u32(lista + 4 * k) == 0) {
        *((lista + 4 * k) as *mut usize) = clube;
        return;
    }
    if forcar == 0 {
        return;
    }
    let k = sortear(3);
    if (0..3).contains(&k) {
        *((lista + 4 * k as usize) as *mut usize) = clube;
    }
}

unsafe extern "C" fn comparar_staff_add_disliked_club(pessoa: usize, clube: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I0%%);
    let pref = if pessoa != 0 { dbq2_u32(pessoa + 0x65) } else { 0 };
    let reg = dbq2_regioes(if pref != 0 { pref + 4 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(pessoa, clube, forcar),
        || crate::rng::without_counting(|| staff_add_disliked_club(pessoa, clube, forcar)));
    SCOREBOARD.record(igual, || format!("staff_add_disliked_club({:x}, {:x}, {}) {:x?} x {:x?}", pessoa, clube, forcar, mo, mr));
}

/// { 0x547f90 } (cdecl, Database.cpp): `staff_fade_staff_relationships(pessoa)`: cada uma das três pessoas favoritas
/// (preferências +0x1c..+0x24) presente sai com o sorteio (0x90cfc0) de (+0x59 da pessoa, com sinal) + 15 dando 0;
/// cada uma das três rejeitadas (+0x28..+0x30), com o de 35 − (+0x5d). Um sorteio por vaga ocupada, na ordem.
/// Pessoa ou preferências nulas: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0).
unsafe extern "C" fn staff_fade_staff_relationships(pessoa: usize) {
    if pessoa == 0 || dbq2_u32(pessoa + 0x65) == 0 {
        return;
    }
    let pref = dbq2_u32(pessoa + 0x65);
    for k in 0..3 {
        let v = pref + 0x1c + 4 * k;
        if dbq2_u32(v) != 0 && sortear(*((pessoa + 0x59) as *const i8) as i32 + 15) == 0 { // as favoritas
            *(v as *mut usize) = 0;
        }
    }
    for k in 0..3 {
        let v = pref + 0x28 + 4 * k;
        if dbq2_u32(v) != 0 && sortear(35 - *((pessoa + 0x5d) as *const i8) as i32) == 0 { // as rejeitadas
            *(v as *mut usize) = 0;
        }
    }
}

unsafe extern "C" fn comparar_staff_fade_staff_relationships(pessoa: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I1%%);
    let pref = if pessoa != 0 { dbq2_u32(pessoa + 0x65) } else { 0 };
    let reg = dbq2_regioes(if pref != 0 { pref + 0x1c } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(pessoa),
        || crate::rng::without_counting(|| staff_fade_staff_relationships(pessoa)));
    SCOREBOARD.record(igual, || format!("staff_fade_staff_relationships({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// { 0x5480f0 } (cdecl, Database.cpp): `staff_get_disliked_staff(pessoa, saida, n)`: zera o byte `*n` (sempre, antes
/// de conferir a pessoa) e copia para `saida` as pessoas rejeitadas presentes (preferências +0x28/+0x2c/+0x30), na
/// ordem, contando em `*n`. Pessoa, preferências ou saída nulas: só o zero. Devolve `n` (o eax da original); os
/// chamadores leem só o byte. No diário das pessoas (0x5ce2d0, duas chamadas), MANAGER_MANAGER (0x693850), mídia e
/// telas de pessoas.
unsafe extern "C" fn staff_get_disliked_staff(pessoa: usize, saida: usize, n: usize) -> usize {
    let cont = n as *mut i8;
    *cont = 0;
    if pessoa == 0 || dbq2_u32(pessoa + 0x65) == 0 || saida == 0 {
        return n;
    }
    let pref = dbq2_u32(pessoa + 0x65);
    for k in 0..3 {
        let p = dbq2_u32(pref + 0x28 + 4 * k); // as rejeitadas
        if p != 0 {
            *((saida as isize + 4 * *cont as isize) as *mut usize) = p;
            *cont = (*cont).wrapping_add(1);
        }
    }
    n
}

unsafe extern "C" fn comparar_staff_get_disliked_staff(pessoa: usize, saida: usize, n: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> usize>(%%I2%%);
    let mut reg = vec![(n, 1)];
    if saida != 0 {
        reg.push((saida, 12));
    }
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(pessoa, saida, n), || staff_get_disliked_staff(pessoa, saida, n));
    SCOREBOARD.record(igual && r == m, || format!("staff_get_disliked_staff({:x}, {:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, saida, n, r, m, mo, mr));
    r
}
