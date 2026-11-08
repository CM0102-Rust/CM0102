// ---------- %%LEVA%%a leva: acrescentar às preferências: a pessoa favorita e a rejeitada do clube e a pessoa favorita da pessoa (Database.cpp) ----------

unsafe fn dbq1_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A lista de três dwords em `lista` tem `quem`?
unsafe fn list3_contains(lista: usize, quem: usize) -> bool {
    (0..3).any(|k| dbq1_u32(lista + 4 * k) == quem)
}

/// O miolo das rotinas de acrescentar às preferências: já na lista (`ja`), nada; na lista `outra` (o oposto: favoritos
/// x rejeitados), tira de lá a primeira ocorrência e para; senão entra na primeira vaga de `lista`; sem vaga, só com
/// `forcar`: sorteia (0x90cfc0, 3) a vaga que troca (0, 1 ou 2).
unsafe fn database_add_to_preference_list(lista: usize, outra: usize, quem: usize, ja: bool, forcar: u32) {
    if ja {
        return;
    }
    if let Some(k) = (0..3).find(|&k| dbq1_u32(outra + 4 * k) == quem) {
        *((outra + 4 * k) as *mut usize) = 0;
        return;
    }
    if let Some(k) = (0..3).find(|&k| dbq1_u32(lista + 4 * k) == 0) {
        *((lista + 4 * k) as *mut usize) = quem;
        return;
    }
    if forcar == 0 {
        return;
    }
    let k = random(3);
    if (0..3).contains(&k) {
        *((lista + 4 * k as usize) as *mut usize) = quem;
    }
}

/// O estado do sorteio e as 6 pessoas (favoritas e rejeitadas) a partir de `base` (o clube +0x9b ou as preferências
/// +0x1c); `base` nulo: só o sorteio.
unsafe fn preference_regions(base: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if base != 0 {
        reg.push((base, 24));
    }
    reg
}

/// { 0x547870 } (cdecl, Database.cpp): `club_add_favoured_staff(clube, pessoa, forcar)`: põe a pessoa entre as três
/// favoritas do clube (+0x9b/+0x9f/+0xa3): já favorita, nada; rejeitada (+0xa7..+0xaf), deixa de ser e para; senão a
/// primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Clube ou pessoa nulos: nada. Nos registros de clubes
/// (0x445f45) e de pessoas (0x86dc30).
unsafe extern "C" fn club_add_favoured_staff(clube: usize, pessoa: usize, forcar: u32) {
    if clube == 0 || pessoa == 0 {
        return;
    }
    database_add_to_preference_list(clube + 0x9b, clube + 0xa7, pessoa, list3_contains(clube + 0x9b, pessoa), forcar) // as favoritas do clube
}

unsafe extern "C" fn compare_club_add_favoured_staff(clube: usize, pessoa: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I0%%);
    let reg = preference_regions(if clube != 0 { clube + 0x9b } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(clube, pessoa, forcar),
        || crate::rng::without_counting(|| club_add_favoured_staff(clube, pessoa, forcar)));
    SCOREBOARD.record(igual, || format!("club_add_favoured_staff({:x}, {:x}, {}) {:x?} x {:x?}", clube, pessoa, forcar, mo, mr));
}

/// { 0x547940 } (cdecl, Database.cpp): `club_add_disliked_staff(clube, pessoa, forcar)`: põe a pessoa entre as três
/// rejeitadas do clube (+0xa7/+0xab/+0xaf): já rejeitada, nada; favorita (+0x9b..+0xa3), deixa de ser e para; senão a
/// primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Nas audiências das disputas (0x569ed0), na mídia
/// (0x755340) e em 0x7daf50 (do motor de partidas).
unsafe extern "C" fn club_add_disliked_staff(clube: usize, pessoa: usize, forcar: u32) {
    if clube == 0 || pessoa == 0 {
        return;
    }
    database_add_to_preference_list(clube + 0xa7, clube + 0x9b, pessoa, list3_contains(clube + 0xa7, pessoa), forcar) // as rejeitadas do clube
}

unsafe extern "C" fn compare_club_add_disliked_staff(clube: usize, pessoa: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I1%%);
    let reg = preference_regions(if clube != 0 { clube + 0x9b } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(clube, pessoa, forcar),
        || crate::rng::without_counting(|| club_add_disliked_staff(clube, pessoa, forcar)));
    SCOREBOARD.record(igual, || format!("club_add_disliked_staff({:x}, {:x}, {}) {:x?} x {:x?}", clube, pessoa, forcar, mo, mr));
}

/// { 0x547a10 } (cdecl, Database.cpp): `staff_add_favoured_staff(pessoa, outra, forcar)`: põe `outra` entre as três
/// pessoas favoritas da pessoa (preferências +0x65: +0x1c/+0x20/+0x24): já favorita, nada; rejeitada (+0x28..+0x30),
/// deixa de ser e para; senão a primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Pessoa, preferências ou
/// outra nulas: nada. Nas relações do elenco (0x5d6d10, diária), registros de clubes (0x445f45), volta da lesão
/// (0x630ee0), mídia e outras.
unsafe extern "C" fn staff_add_favoured_staff(pessoa: usize, outra: usize, forcar: u32) {
    if pessoa == 0 {
        return;
    }
    let pref = dbq1_u32(pessoa + 0x65);
    if pref == 0 || outra == 0 {
        return;
    }
    database_add_to_preference_list(pref + 0x1c, pref + 0x28, outra, list3_contains(pref + 0x1c, outra), forcar) // as favoritas da pessoa
}

unsafe extern "C" fn compare_staff_add_favoured_staff(pessoa: usize, outra: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I2%%);
    let pref = if pessoa != 0 { dbq1_u32(pessoa + 0x65) } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + 0x1c } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(pessoa, outra, forcar),
        || crate::rng::without_counting(|| staff_add_favoured_staff(pessoa, outra, forcar)));
    SCOREBOARD.record(igual, || format!("staff_add_favoured_staff({:x}, {:x}, {}) {:x?} x {:x?}", pessoa, outra, forcar, mo, mr));
}
