// ---------- %%LEVA%%a leva: as seleções: as reservas de convocáveis de cada nação e as duas ordens delas (national_teams.cpp) ----------

unsafe fn cpd4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpd4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpd4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A nota da pessoa da vaga `q` (ponteiro para a pessoa): o curto +9 da ficha (+0x61).
unsafe fn slot_person_rating(q: usize) -> i16 {
    cpd4_w(cpd4_d(cpd4_d(q) + 0x61) + 9)
}

/// O começo do trecho do nível `nivel` na reserva (`o` 0: a principal, 0x15: o sub-21): o +8 do registro mais as
/// vagas (+0xe) dos níveis anteriores.
unsafe fn reserve_level_start(rec: usize, o: usize, nivel: i8) -> i32 {
    let mut v = cpd4_d(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        v = v.wrapping_add(cpd4_b(rec + o + 0xe + 4 * j) as i32);
    }
    v
}

/// Põe a pessoa `p` na reserva do registro `rec` (`sub` 0: a principal em [this+0xc]; 1: o sub-21 em [this+0x10],
/// campos 0x15 adiante), no trecho do nível dela (0x76d810). Com vaga (+0xd < +0xe), no fim do trecho, e a menor nota
/// (+0xf) desce se for o caso; cheio, só com nota acima da menor: entra na posição pela nota (decrescente, depois das
/// iguais), empurrando o último para fora, e a menor nota vira a do último. Trecho cheio: ordena (qsort 0x945333 com
/// 0x778ed0).
unsafe fn add_to_reserves(this: usize, p: usize, rec: usize, sub: u32) {
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x76d810usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let o = 0x15 * sub as usize;
    let nivel = nivel_de(p, rec, sub) as i8;
    let c = rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize);
    let reserva = || cpd4_d(this + 0xc + 4 * sub as usize);
    let vaga = |k: i32| reserva().wrapping_add((4 * k) as usize);
    let nota = cpd4_w(cpd4_d(p + 0x61) + 9);
    if cpd4_b(c + 0xd) < cpd4_b(c + 0xe) {
        let ini = reserve_level_start(rec, o, nivel);
        core::ptr::write_unaligned(vaga(ini.wrapping_add(cpd4_b(c + 0xd) as i32)) as *mut usize, p);
        *((c + 0xd) as *mut u8) = cpd4_b(c + 0xd).wrapping_add(1);
        if nota < cpd4_w(c + 0xf) {
            core::ptr::write_unaligned((c + 0xf) as *mut i16, nota); // a menor nota
        }
    } else if nota > cpd4_w(c + 0xf) {
        crate::comparison::branch("reserves full enters by rating");
        let ini = reserve_level_start(rec, o, nivel);
        let ultimo = (cpd4_b(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(2);
        let mut pos = ini;
        let mut j = ultimo;
        while j >= ini {
            if nota < slot_person_rating(vaga(j)) {
                pos = j + 1; // depois das de nota maior
                break;
            }
            j -= 1;
        }
        let mut j = ultimo;
        while j >= pos {
            core::ptr::write_unaligned(vaga(j + 1) as *mut usize, cpd4_d(vaga(j)));
            j -= 1;
        }
        core::ptr::write_unaligned(vaga(pos) as *mut usize, p);
        let ult = (cpd4_b(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(1);
        core::ptr::write_unaligned((c + 0xf) as *mut i16, slot_person_rating(vaga(ult)));
    }
    let vagas = cpd4_b(c + 0xe);
    if cpd4_b(c + 0xd) == vagas {
        qsort(vaga(reserve_level_start(rec, o, nivel)), vagas as u32, 4, 0x778ed0);
    }
}

/// { 0x76abf0 } (thiscall, national_teams.cpp): `build_selection_pools()`: zera as contagens (+0xd, +0x22) e põe
/// 10000 nas menores notas (+0xf, +0x24) dos 4 níveis de cada registro de nação ([this+4], 0x6e bytes); depois, cada
/// pessoa ([0xae23bc], 0x6e bytes) com nação (+0x1a), ficha (+0x61), mais de 16 anos (+0x18), sem seleção ou já
/// convocada (+0x24, +0x28 = 0xb), na posição de jogador (0x53b620 de +0x3d) e com clube (+0x39) entra na reserva
/// da nação (add_to_reserves) e, até 21 anos com menos de 5 jogos (+0x22), na do sub-21; e de novo pela segunda
/// nacionalidade (+0x1e: sem jogos pela primeira, outra e não a [0x9cf434]). Sem SEH, sem caixa, sem sorteio; as
/// locais são gravadas antes de ler (da nota guardada em 32 bits só vale o curto).
unsafe extern "thiscall" fn build_selection_pools(this: usize) {
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        for k in 0..4usize {
            let r = cpd4_d(this + 4) + 0x6e * i as usize + 4 * k;
            *((r + 0xd) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0xf) as *mut i16, 10000);
            *((r + 0x22) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0x24) as *mut i16, 10000);
        }
        i += 1;
    }
    let jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let mut p = *(0xae23bc as *const usize);
    let mut n = 0i32;
    while n < *(0xae2364 as *const i32) {
        let pode = cpd4_d(p + 0x1a) != 0
            && cpd4_d(p + 0x61) != 0
            && *((p + 0x18) as *const i8) > 0x10 // mais de 16 anos
            && (cpd4_d(p + 0x24) == 0 || cpd4_b(p + 0x28) == 0xb)
            && jogador(cpd4_b(p + 0x3d) as u32) != 0
            && cpd4_d(p + 0x39) != 0;
        if pode {
            for k in 0..2 {
                let id = if k == 0 {
                    *(cpd4_d(p + 0x1a) as *const i32)
                } else {
                    let n2 = cpd4_d(p + 0x1e);
                    if n2 == 0 || cpd4_b(p + 0x22) != 0 || n2 == cpd4_d(p + 0x1a) || *(n2 as *const i32) == *(0x9cf434 as *const i32) {
                        continue;
                    }
                    crate::comparison::branch("reserves second nationality");
                    *(n2 as *const i32)
                };
                let rec = cpd4_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize);
                add_to_reserves(this, p, rec, 0);
                if *((p + 0x18) as *const i8) > 0x15 || cpd4_b(p + 0x22) >= 5 { // o sub-21
                    continue;
                }
                add_to_reserves(this, p, rec, 1);
            }
        }
        n += 1;
        p += 0x6e;
    }
}

/// Comparador: os registros das nações e, em cada reserva (principal e sub-21), o trecho dos 4 níveis (a soma das
/// vagas +0xe a partir do começo +8).
unsafe extern "thiscall" fn compare_build_selection_pools(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let n = (*(0xae2350 as *const i32)).max(0) as usize;
    let recs = cpd4_d(this + 4);
    let mut reg = vec![(recs, 0x6e * n)];
    for i in 0..n {
        let rec = recs + 0x6e * i;
        for sub in 0..2usize {
            let o = 0x15 * sub;
            let total: usize = (0..4).map(|k| cpd4_b(rec + o + 0xe + 4 * k) as usize).sum();
            if total > 0 {
                reg.push((cpd4_d(this + 0xc + 4 * sub).wrapping_add(4 * cpd4_d(rec + o + 8)), 4 * total));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || build_selection_pools(this));
    SCOREBOARD.record(igual, || format!("build_selection_pools({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// { 0x778ed0 } (cdecl, national_teams.cpp): a ordem das reservas para o qsort da 0x76abf0: pela nota (+9 da ficha
/// +0x61) decrescente (nota de b menos a de a).
unsafe extern "C" fn compare_by_rating(a: usize, b: usize) -> i32 {
    (slot_person_rating(b) as i32) - slot_person_rating(a) as i32 // decrescente
}

unsafe extern "C" fn compare_compare_by_rating(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_by_rating(a, b);
    SCOREBOARD.record(r == n, || format!("compare_by_rating({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x778ef0 } (cdecl, national_teams.cpp): a ordem das pessoas pelo clube (+0x39) para os qsort das seleções
/// (0x76c120, 0x7706c0, 0x7771c0): sem clube depois; clubes diferentes pelo id; no mesmo, o de elenco 4 (+0x5e)
/// antes do de elenco 8.
unsafe extern "C" fn compare_by_club(a: usize, b: usize) -> i32 {
    let (x, y) = (cpd4_d(a), cpd4_d(b));
    let cx = if x != 0 { cpd4_d(x + 0x39) } else { 0 };
    let cy = if y != 0 { cpd4_d(y + 0x39) } else { 0 };
    if cx == 0 {
        return (cy != 0) as i32;
    }
    if cy == 0 {
        return -1;
    }
    if cx != cy {
        return (*(cx as *const i32)).wrapping_sub(*(cy as *const i32)); // o id do clube
    }
    let (mx, my) = (cpd4_b(x + 0x5e), cpd4_b(y + 0x5e));
    if mx & 4 != 0 && my & 8 != 0 {
        return -1;
    }
    if mx & 8 != 0 && my & 4 != 0 {
        return 1;
    }
    0
}

unsafe extern "C" fn compare_compare_by_club(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_by_club(a, b);
    SCOREBOARD.record(r == n, || format!("compare_by_club({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
