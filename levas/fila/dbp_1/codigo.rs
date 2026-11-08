// ---------- %%LEVA%%a leva: a reputação do clube para a pessoa e quantos favoritos e rejeitados dela estão no clube (Database.cpp) ----------

unsafe fn dbp_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbp_i16(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube do contrato da `pessoa` como a 0x545e90 o acha: o contrato pela 0x4cf5a0 (this = [0xae1050]), gravado
/// em [0xae1080] como na original; com contrato, o clube de id +4 na tabela [0xae23b4] (581 bytes); sem: 0.
unsafe fn database_club_of_contract(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(dbp_u32(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    dbp_u32(0xae23b4).wrapping_add(dbp_u32(c + 4).wrapping_mul(581))
}

/// +500 se o `clube` está entre os três clubes favoritos da pessoa (preferências +4/+8/+0xc), −500 se entre os três
/// rejeitados (+0x10/+0x14/+0x18), senão 0; sem preferências: 0.
unsafe fn club_preference_bonus(clube: usize, pessoa: usize) -> i32 {
    let pref = dbp_u32(pessoa + 0x65);
    if pref == 0 {
        return 0;
    }
    if (0..3).any(|k| dbp_u32(pref + 4 + 4 * k) == clube) {
        500
    } else if (0..3).any(|k| dbp_u32(pref + 0x10 + 4 * k) == clube) {
        -500
    } else {
        0
    }
}

/// { 0x545e90 } (cdecl, Database.cpp): `club_reputation_for_staff(clube, pessoa)`: a reputação do clube (+0x80) vista
/// pela pessoa. Sem divisão (+0x57): a reputação. Com a liga da divisão ([0xadadfc][id]) e a reputação da divisão
/// (+0x69) × 500 acima da do clube: a posição (0x4a4850 na tabela +8 da liga; sem linha: 0) escalada a 20 clubes
/// (+0x3e) dá o peso da tabela 0x9b59c0; com menos de 20 jogos (+4 da linha) mistura com o peso da posição anterior
/// (+0x5f, se +0x5b = +0x57; senão 1,3) × 0,05; se a pessoa joga (+0x61) e o contrato é com o clube, soma a média
/// 0x7c1e30(dados, 6, 20, peso) e divide por 2, e ganha ±500 pelas preferências; o peso vira x = 14p − 18 entre 0 e
/// 10 e o resultado é ((10 − x) × reputação + x × divisão × 500 (16 bits)) / 10 truncado, mais o ±500. Senão: a
/// reputação ±500 (pessoa que joga, com contrato no clube). Os chamadores leem só `ax`; o resto do eax segue a
/// original (o registro que sobrou em cada caminho). Grava [0xae1080] (o contrato achado). No diário das pessoas
/// (game_daily_staff_update 0x5ce2d0, duas chamadas).
unsafe extern "C" fn club_reputation_for_staff(clube: usize, pessoa: usize) -> u32 {
    let rep = dbp_i16(clube + 0x80);
    let comp = dbp_u32(clube + 0x57);
    if comp == 0 {
        return rep as u16 as u32;
    }
    let liga = dbp_u32(dbp_u32(0xadadfc).wrapping_add(dbp_u32(comp).wrapping_mul(4)));
    let divisao = dbp_i16(comp + 0x69);
    let alto = (divisao as i32).wrapping_mul(500);
    if liga != 0 && alto > rep as i32 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let e = busca(dbp_u32(liga + 8), clube);
        if e == 0 {
            return 0;
        }
        let n = dbp_i16(liga + 0x3e) as i32;
        let peso = |k: i32| read_f64_at(0x9b59c0 + 8 * k.clamp(0, 19) as usize);
        let mut posicao = *((e + 0x73) as *const u8) as i32 - 1;
        if n != 0 && n != 20 {
            posicao = posicao * 20 / n;
        }
        let mut v = peso(posicao);
        let jogos = *((e + 4) as *const u8) as i32;
        if jogos < 20 {
            let antes = *((clube + 0x5f) as *const i8) as i32;
            let w = if dbp_u32(clube + 0x5b) != comp {
                read_f64_at(0x968de8)
            } else if n != 0 && n != 20 {
                peso((antes * 20 - 20) / n)
            } else if (1..21).contains(&antes) {
                peso(antes - 1)
            } else {
                read_f64_at(0x968de8)
            };
            v = ((20 - jogos) as f64 * w + jogos as f64 * v) * read_f64_at(0x967888);
        }
        let mut bonus = 0;
        if pessoa != 0 && dbp_u32(pessoa + 0x61) != 0 && database_club_of_contract(pessoa) == clube {
            let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
            let m = media(dbp_u32(0xb74350), dbp_u32(pessoa + 0x61), 6, 0x14, v as f32);
            v = (m + v) * read_f64_at(0x968968);
            bonus = club_preference_bonus(clube, pessoa);
        }
        let mut x = v * read_f64_at(0x969558) - read_f64_at(0x969550); // o peso entre 0 e 10
        let (baixo, teto) = (read_f64_at(0x9689a0), read_f64_at(0x9688b0));
        if baixo > x {
            x = baixo;
        } else if !(teto >= x) {
            x = teto; // fcomp: acima do teto ou NaN
        }
        let cx = (divisao as i32).wrapping_mul(500) as i16 as f64; // imul cx, cx, 0x1f4: 16 bits
        let r = ((teto - x) * rep as f64 + cx * x) * read_f64_at(0x967880);
        return (match_pl_ftol(r) as u32).wrapping_add(bonus as u32);
    }
    // sem liga, ou o clube já à altura da divisão: a reputação ±500; a parte alta do eax é o que sobrou
    let mut eax = if liga == 0 { comp } else { alto as usize };
    let mut bonus = 0;
    if pessoa != 0 {
        eax = dbp_u32(pessoa + 0x61);
        if eax != 0 {
            eax = database_club_of_contract(pessoa);
            if eax == clube {
                eax = dbp_u32(pessoa + 0x65);
                if eax != 0 {
                    bonus = club_preference_bonus(clube, pessoa); // os clubes da pessoa
                }
            }
        }
    }
    ((eax as u32 & 0xffff_0000) | rep as u16 as u32).wrapping_add(bonus as u32)
}

unsafe extern "C" fn comparar_club_reputation_for_staff(clube: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(clube, pessoa),
        || club_reputation_for_staff(clube, pessoa));
    // os chamadores leem só o ax (movsx)
    SCOREBOARD.record(igual && r as u16 == n as u16, || format!("club_reputation_for_staff({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, pessoa, r, n, mo, mr));
    r
}

/// Quantas das três pessoas da lista de preferências em `base` (+0x65 da pessoa) têm o clube (+0x39) igual a `clube`;
/// pessoa, preferências ou clube nulos: 0. Só `al`.
unsafe fn count_preferred_people_at_club(pessoa: usize, clube: usize, base: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let pref = dbp_u32(pessoa + 0x65);
    if pref == 0 || clube == 0 {
        return 0;
    }
    let mut n = 0u8;
    for k in 0..3 {
        let p = dbp_u32(pref + base + 4 * k);
        if p != 0 && dbp_u32(p + 0x39) == clube {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// { 0x547c90 } (cdecl, Database.cpp): `staff_count_disliked_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// que a pessoa rejeita (preferências +0x28/+0x2c/+0x30) estão no clube. No diário das pessoas (0x5ce2d0).
unsafe extern "C" fn staff_count_disliked_staff_at_club(pessoa: usize, clube: usize) -> u8 {
    count_preferred_people_at_club(pessoa, clube, 0x28) // os rejeitados
}

unsafe extern "C" fn comparar_staff_count_disliked_staff_at_club(pessoa: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I1%%)(pessoa, clube);
    let n = staff_count_disliked_staff_at_club(pessoa, clube);
    SCOREBOARD.record(r == n, || format!("staff_count_disliked_staff_at_club({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

/// { 0x547ce0 } (cdecl, Database.cpp): `staff_count_favoured_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24) estão no clube. No diário das pessoas (0x5ce2d0).
unsafe extern "C" fn staff_count_favoured_staff_at_club(pessoa: usize, clube: usize) -> u8 {
    count_preferred_people_at_club(pessoa, clube, 0x1c) // os favoritos
}

unsafe extern "C" fn comparar_staff_count_favoured_staff_at_club(pessoa: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I2%%)(pessoa, clube);
    let n = staff_count_favoured_staff_at_club(pessoa, clube);
    SCOREBOARD.record(r == n, || format!("staff_count_favoured_staff_at_club({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}
