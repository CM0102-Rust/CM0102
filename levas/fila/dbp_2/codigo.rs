// ---------- %%LEVA%%a leva: tirar das preferências: o clube favorito da pessoa, a pessoa favorita do clube e a pessoa favorita da pessoa (Database.cpp) ----------

/// Zera a primeira das três posições (`reg`, +4, +8) que tem `alvo` (a ordem da original: achou, zera e sai).
unsafe fn remove_from_list3(reg: usize, alvo: usize) {
    for k in 0..3 {
        let a = (reg + 4 * k) as *mut usize;
        if core::ptr::read_unaligned(a) == alvo {
            core::ptr::write_unaligned(a, 0);
            return;
        }
    }
}

/// As preferências (+0x65) da pessoa não nula; pessoa nula: 0 (o eax que as 0x547c10/0x547c50/0x547f50 devolvem).
unsafe fn staff_preferences(pessoa: usize) -> usize {
    if pessoa == 0 {
        0
    } else {
        core::ptr::read_unaligned((pessoa + 0x65) as *const usize)
    }
}

/// { 0x547f50 } (cdecl, Database.cpp): `staff_remove_favoured_club(pessoa, clube)`: tira o clube dos três clubes
/// favoritos da pessoa (preferências +4/+8/+0xc, o primeiro que bate). Pessoa, preferências ou clube nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). No diário das pessoas (0x5ce2d0), quando o
/// apego ao clube acaba.
unsafe extern "C" fn staff_remove_favoured_club(pessoa: usize, clube: usize) -> usize {
    let pref = staff_preferences(pessoa);
    if pref != 0 && clube != 0 {
        remove_from_list3(pref + 4, clube); // os clubes favoritos
    }
    pref
}

unsafe extern "C" fn comparar_staff_remove_favoured_club(pessoa: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I0%%);
    let pref = staff_preferences(pessoa);
    if pref == 0 {
        return o(pessoa, clube); // não escreve nada
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(pref + 4, 12)], || o(pessoa, clube), || staff_remove_favoured_club(pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("staff_remove_favoured_club({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// { 0x547bd0 } (cdecl, Database.cpp): `club_remove_favoured_staff(clube, pessoa)`: tira a pessoa das três pessoas
/// favoritas do clube (+0x9b/+0x9f/+0xa3, o primeiro que bate). Clube ou pessoa nulos: nada. Devolve o clube (o eax
/// da original). Nas audiências das disputas (DISPUTE::process_hearings 0x569ed0).
unsafe extern "C" fn club_remove_favoured_staff(clube: usize, pessoa: usize) -> usize {
    if clube != 0 && pessoa != 0 {
        remove_from_list3(clube + 0x9b, pessoa); // os favoritos do clube
    }
    clube
}

unsafe extern "C" fn comparar_club_remove_favoured_staff(clube: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I1%%);
    if clube == 0 {
        return o(clube, pessoa);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(clube + 0x9b, 12)], || o(clube, pessoa), || club_remove_favoured_staff(clube, pessoa));
    SCOREBOARD.record(igual && r == n, || format!("club_remove_favoured_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", clube, pessoa, r, n, mo, mr));
    r
}

/// { 0x547c10 } (cdecl, Database.cpp): `staff_remove_favoured_staff(pessoa, alvo)`: tira o alvo das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24, o primeiro que bate). Pessoa, preferências ou alvo nulos:
/// nada. Devolve o eax da original (as preferências; pessoa nula: 0). Nas disputas (0x569ed0), nas relações do
/// elenco (game_update_squad_relationships 0x5d6d10, diária), na volta da lesão (0x630ee0), no MANAGER_MANAGER
/// (0x697a7a) e na mídia (0x75a340).
unsafe extern "C" fn staff_remove_favoured_staff(pessoa: usize, alvo: usize) -> usize {
    let pref = staff_preferences(pessoa);
    if pref != 0 && alvo != 0 {
        remove_from_list3(pref + 0x1c, alvo); // as pessoas favoritas
    }
    pref
}

unsafe extern "C" fn comparar_staff_remove_favoured_staff(pessoa: usize, alvo: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I2%%);
    let pref = staff_preferences(pessoa);
    if pref == 0 {
        return o(pessoa, alvo);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(pref + 0x1c, 12)], || o(pessoa, alvo), || staff_remove_favoured_staff(pessoa, alvo));
    SCOREBOARD.record(igual && r == n, || format!("staff_remove_favoured_staff({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", pessoa, alvo, r, n, mo, mr));
    r
}
