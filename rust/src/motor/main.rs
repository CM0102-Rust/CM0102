use super::*;

/// The list node (14 bytes: +0 the item, +4 the size as a signed word, +6 the next, +0xa the previous) whose item has
/// the same bytes as `dado`, starting from the head (+2); 0 if none.
///
/// The size is used as in the original routine (movsx into the ecx of repe cmpsb): 0 counts as equal (the repe does not
/// run and ZF comes from the earlier xor).
///
/// [pt] O nó da lista (14 bytes: +0 o item, +4 o tamanho em palavra com sinal, +6 o próximo, +0xa o anterior) cujo item tem
/// os mesmos bytes que `dado`, a partir da cabeça (+2). O tamanho vai como na original (movsx para o ecx do repe
/// cmpsb): 0 é igual (o repe não roda e o ZF vem do xor de antes).
pub(crate) unsafe fn find_list_node_by_bytes(this: usize, given: usize) -> usize {
    let mut no = read_usize(this + 2); // cqb10: the search starts at the head
    while no != 0 {
        let n = core::ptr::read_unaligned((no + 4) as *const i16) as i32 as u32;
        let item = read_usize(no);
        let mut k = 0u32;
        while k < n && *((item + k as usize) as *const u8) == *((given + k as usize) as *const u8) {
            k += 1;
        }
        if k == n {
            return no;
        }
        no = read_usize(no + 6);
    }
    0
}

