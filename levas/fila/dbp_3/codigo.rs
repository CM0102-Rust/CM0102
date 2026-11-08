// ---------- %%LEVA%%a leva: a pessoa rejeitada que sai das preferências e as pessoas favoritas num vetor (Database.cpp) ----------

/// { 0x547c50 } (cdecl, Database.cpp): `staff_remove_disliked_staff(pessoa, alvo)`: zera a primeira das três pessoas
/// rejeitadas da pessoa (preferências +0x28/+0x2c/+0x30) que é o alvo. Pessoa, preferências ou alvo nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). Nas relações do elenco (game_update_squad_relationships
/// 0x5d6d10, diária), na volta da lesão (0x630ee0) e na mídia (0x7414ac, 0x75a340).
unsafe extern "C" fn staff_remove_disliked_staff(pessoa: usize, alvo: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let pref = core::ptr::read_unaligned((pessoa + 0x65) as *const usize);
    if pref != 0 && alvo != 0 {
        let reg = pref + 0x28; // as pessoas rejeitadas
        for k in 0..3 {
            let a = (reg + 4 * k) as *mut usize;
            if core::ptr::read_unaligned(a) == alvo {
                core::ptr::write_unaligned(a, 0);
                break;
            }
        }
    }
    pref
}

unsafe extern "C" fn comparar_staff_remove_disliked_staff(pessoa: usize, alvo: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I0%%);
    let pref = if pessoa == 0 { 0 } else { core::ptr::read_unaligned((pessoa + 0x65) as *const usize) };
    if pref == 0 {
        return o(pessoa, alvo); // não escreve nada
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(pref + 0x28, 12)], || o(pessoa, alvo), || staff_remove_disliked_staff(pessoa, alvo));
    PLACAR.record(igual && r == n, || format!("staff_remove_disliked_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", pessoa, alvo, r, n, mo, mr));
    r
}

/// { 0x548090 } (cdecl, Database.cpp): `staff_get_favoured_staff(pessoa, vetor, n)`: zera o byte `n` e, com pessoa,
/// preferências e vetor não nulos, copia as pessoas favoritas não nulas (preferências +0x1c, +0x20, +0x24, nessa
/// ordem) para `vetor[n]`, somando 1 a `n` a cada uma (a primeira vai direto em `vetor[0]`). Devolve `n` (o eax da
/// original). No MANAGER_MANAGER: a opinião da diretoria sobre o elenco (0x693850) e a reação dela à partida
/// (0x6aaee0).
unsafe extern "C" fn staff_get_favoured_staff(pessoa: usize, vetor: usize, n: *mut i8) -> *mut i8 {
    *n = 0;
    if pessoa == 0 {
        return n;
    }
    let pref = |p: usize| core::ptr::read_unaligned((p + 0x65) as *const usize);
    if pref(pessoa) == 0 || vetor == 0 {
        return n;
    }
    for (k, d) in [0x1cusize, 0x20, 0x24].into_iter().enumerate() { // as três favoritas
        let p = core::ptr::read_unaligned((pref(pessoa) + d) as *const usize);
        if p != 0 {
            let i = if k == 0 { 0 } else { *n as isize };
            core::ptr::write_unaligned((vetor as isize + 4 * i) as *mut usize, p);
            *n = (*n).wrapping_add(1);
        }
    }
    n
}

unsafe extern "C" fn comparar_staff_get_favoured_staff(pessoa: usize, vetor: usize, n: *mut i8) -> *mut i8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut i8) -> *mut i8>(%%I1%%);
    let com_vetor = [(vetor, 12), (n as usize, 1)];
    let regioes: &[(usize, usize)] = if vetor == 0 { &com_vetor[1..] } else { &com_vetor };
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(regioes, || o(pessoa, vetor, n), || staff_get_favoured_staff(pessoa, vetor, n));
    PLACAR.record(igual && r == x, || format!("staff_get_favoured_staff({:x}, {:x}) {:x?} x {:x?}", pessoa, vetor, mo, mr));
    r
}
