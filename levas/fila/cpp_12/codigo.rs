// ---------- %%LEVA%%a leva: as seleções e o calendário dos clubes: se o jogo do clube pode ser marcado com os convocados, a ordem das convocações pela data e a nação classificada para a fase final (national_teams.cpp) ----------

unsafe fn cpp12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Dia (palavra com sinal) e ano (palavra sem sinal) de uma data de 8 bytes.
unsafe fn national_teams_date_day_year(p: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p + 2) as *const u16))
}

/// Os convocados do clube (id < [0xae235c] − 2 × [0xae2350]: clube comum, não seleção) entre as datas `a` e `b`: a
/// lista do clube em [this + 0x20] + 6 × id (+0 os registros de 9 bytes em ordem de data, +5 a conta em byte com sinal);
/// pula os antes de `a`, para no primeiro depois de `b` e soma os bytes +8 (em byte, com a volta). Mais de 2: demais.
unsafe fn has_too_many_called_up(this: usize, clube: usize, a: usize, b: usize) -> bool {
    if clube == 0 {
        return false;
    }
    let limit = cpp12_i(0xae235c).wrapping_sub(cpp12_i(0xae2350).wrapping_mul(2));
    let id = cpp12_i(clube);
    if id >= limit {
        return false;
    }
    let rec = cpp12_d(this + 0x20).wrapping_add((id as usize).wrapping_mul(6));
    let n = *((rec + 5) as *const i8);
    let lista = cpp12_d(rec);
    let (a_dia, a_ano) = national_teams_date_day_year(a);
    let (b_dia, b_ano) = national_teams_date_day_year(b);
    let mut soma: i8 = 0;
    let mut k: i8 = 0;
    while k < n {
        let e = lista.wrapping_add(k as isize as usize * 9);
        let (dia, ano) = national_teams_date_day_year(e);
        k = k.wrapping_add(1);
        if a_ano > ano || (a_ano == ano && a_dia > dia) {
            continue; // antes da janela
        }
        if b_ano < ano || (b_ano == ano && b_dia < dia) {
            break; // depois da janela: a lista está em ordem
        }
        crate::comparison::branch("cpp_12 convocado na janela");
        soma = soma.wrapping_add(*((e + 8) as *const i8));
    }
    soma > 2 // mais de dois convocados
}

/// { 0x776fc0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::can_schedule_club_fixture(jogo)`: com a data
/// do jogo (0x549ef0 com +0x2a e +0x28), a janela de um dia antes (0x54c8e0) e um depois (0x54c5b0); jogo da competição
/// amistosa (0x5c6710 em [0xae2c74] com +0x14): 1. Senão 0 quando o mandante (+0x1c) ou o visitante (+0x20) tem mais
/// de 2 convocados na janela; senão 1. O quadro SEH só guarda o estado (as datas não têm destrutor).
unsafe extern "thiscall" fn can_schedule_club_fixture(this: usize, jogo: usize) -> u32 {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(0x549ef0usize);
    let menos_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let (mut d, mut a, mut b) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // ponytail: a original empurra as palavras +0x2a e +0x28 com o alto do registro (lixo); a 0x549ef0 só lê as
    // palavras baixas: a cópia passa as palavras limpas.
    data_nova(d.as_mut_ptr(), *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    menos_dias(d.as_mut_ptr(), a.as_mut_ptr(), 1);
    mais_dias(d.as_mut_ptr(), b.as_mut_ptr(), 1);
    if amistosa(cpp12_d(0xae2c74), cpp12_d(jogo + 0x14)) != 0 {
        return 1;
    }
    let (pa, pb) = (a.as_ptr() as usize, b.as_ptr() as usize);
    if has_too_many_called_up(this, cpp12_d(jogo + 0x1c), pa, pb) {
        return 0;
    }
    if has_too_many_called_up(this, cpp12_d(jogo + 0x20), pa, pb) {
        crate::comparison::branch("cpp_12 visitante convocado");
        return 0; // o visitante com convocados demais
    }
    1
}

/// { 0x778e90 } (cdecl, national_teams.cpp): `compare_call_up_records_by_date(a, b)`: o qsort dos registros com a data
/// em +0 (as convocações de 9 bytes, as lesões, as transferências): −1 se a de `a` é antes (0x456770), 1 se é depois
/// (0x456740), senão 0. Quente (qsort): sem contador.
unsafe extern "C" fn compare_call_up_records_by_date(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpp12_d(b) as u32, cpp12_d(b + 4) as u32);
    let r = if antes(a, d0, d1) != 0 { -1 } else { (depois(a, d0, d1) != 0) as i32 };
    r // a convocação mais cedo primeiro
}

/// { 0x7779b0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::add_qualified_nation(nacao, fase_final)`: com
/// as duas não nulas, acrescenta o registro de 8 bytes (nação, fase final) à lista das novas classificadas em
/// this + 0x28 (LLIST, 0x68d0d0).
unsafe extern "thiscall" fn add_qualified_nation(this: usize, nacao: usize, fase_final: usize) {
    if nacao == 0 || fase_final == 0 {
        return;
    }
    let mut reg = [0u8; 8];
    reg[..4].copy_from_slice(&(nacao as u32).to_le_bytes());
    reg[4..].copy_from_slice(&(fase_final as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    acrescentar(this + 0x28, reg.as_mut_ptr() as usize, 8);
}

/// Comparador: pelo valor (só lê; as datas são locais).
unsafe extern "thiscall" fn compare_can_schedule_club_fixture(this: usize, jogo: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, jogo);
    let n = can_schedule_club_fixture(this, jogo);
    SCOREBOARD.record(r == n, || format!("can_schedule_club_fixture({:x}) original={} rust={}", jogo, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn compare_compare_call_up_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_call_up_records_by_date(a, b);
    SCOREBOARD.record(r == n, || format!("compare_call_up_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: a original aloca o nó (não roda duas vezes): só ela, e confere o plano: a conta da lista (+0x28, palavra)
/// sobe 1 com as duas não nulas e fica igual senão.
unsafe extern "thiscall" fn compare_add_qualified_nation(this: usize, nacao: usize, fase_final: usize) {
    let antes = *((this + 0x28) as *const i16) as i32;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, nacao, fase_final);
    let depois = *((this + 0x28) as *const i16) as i32;
    let esperado = antes + (nacao != 0 && fase_final != 0) as i32;
    SCOREBOARD.record(depois == esperado, || format!("add_qualified_nation({:x}, {:x}) conta {} -> {}", nacao, fase_final, antes, depois));
}
