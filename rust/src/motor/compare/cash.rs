use super::*;

// The comparers of the routines in ../cash.rs.
// [pt] Os comparadores das rotinas de ../cash.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `swap_bytes_16_array`.
///
/// [pt] Comparador: lado a lado no vetor (até 0x420 palavras conferidas).
pub(crate) unsafe extern "C" fn compare_swap_bytes_16_array(p: usize, count: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_SWAP_BYTES_16_ARRAY);
    if p == 0 || count == 0 || count > 0x1000 {
        return o(p, count);
    }
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&[(p, 2 * count as usize)], || o(p, count), || swap_bytes_16_array(p, count));
    SCOREBOARD.record(igual, || format!("swap_bytes_16_array({:x}, {})", p, count));
}

/// Comparador: lado a lado no vetor.
pub(crate) unsafe extern "C" fn compare_swap_bytes_32_array(p: usize, count: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_SWAP_BYTES_32_ARRAY);
    if p == 0 || count == 0 || count > 0x1000 {
        return o(p, count);
    }
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&[(p, 4 * count as usize)], || o(p, count), || swap_bytes_32_array(p, count));
    SCOREBOARD.record(igual, || format!("swap_bytes_32_array({:x}, {})", p, count));
}
