use super::*;

// The comparers of the routines in ../record_utils.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../record_utils.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fill_match_record_from_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7e4750: lado a lado (leve) no registro de saída (0x1f bytes) e na devolução; ponteiro nulo: a caixa,
/// só a original.
pub(crate) unsafe extern "C" fn comparar_fill_match_record_from_fixture(jogo: usize, clube: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_FILL_MATCH_RECORD_FROM_FIXTURE);
    if jogo == 0 || clube == 0 || out == 0 {
        return o(jogo, clube, out);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out, 0x1f)], || o(jogo, clube, out), || fill_match_record_from_fixture(jogo, clube, out));
    PLACAR.record(igual && r == n, || format!("fill_match_record_from_fixture(jogo {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}", jogo, clube, r, n, mo, mr));
    r
}

/// Comparer for `fill_staff_name_ref` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7e4920: lado a lado (leve) na referência (0x18 bytes) e na devolução; ponteiro nulo: só a original.
pub(crate) unsafe extern "C" fn comparar_fill_staff_name_ref(person: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_FILL_STAFF_NAME_REF);
    if person == 0 || out == 0 {
        return o(person, out);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out, 0x18)], || o(person, out), || fill_staff_name_ref(person, out));
    PLACAR.record(igual && r == n, || format!("fill_staff_name_ref(pessoa {:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `match_record_result_code` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7e56d0: pelo byte devolvido (só lê); registro nulo: só a original.
pub(crate) unsafe extern "C" fn comparar_match_record_result_code(record: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_MATCH_RECORD_RESULT_CODE);
    if record == 0 {
        return o(record);
    }
    let r = o(record);
    let n = match_record_result_code(record);
    PLACAR.record(r == n, || format!("match_record_result_code(registro {:x}) original={} rust={}", record, r as i8, n as i8));
    r
}

/// Comparer for `match_record_is_bigger_win` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura; registro nulo (a caixa), só a original.
pub(crate) unsafe extern "C" fn comparar_match_record_is_bigger_win(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_WIN)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = match_record_is_bigger_win(a, b);
    PLACAR.record(r == n, || format!("match_record_is_bigger_win({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `match_record_is_bigger_defeat` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura; registro nulo (a caixa), só a original.
pub(crate) unsafe extern "C" fn comparar_match_record_is_bigger_defeat(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_DEFEAT)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = match_record_is_bigger_defeat(a, b);
    PLACAR.record(r == n, || format!("match_record_is_bigger_defeat({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
