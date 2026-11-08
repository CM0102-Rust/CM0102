use super::*;

// ---------- 264a leva: a lista encadeada do jogo: tirar o item pelo conteúdo, esvaziar e o item do cursor (LLIST, vizinhança de manager_manager.cpp) ----------


pub(crate) unsafe fn cqb10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O nó da lista (14 bytes: +0 o item, +4 o tamanho em palavra com sinal, +6 o próximo, +0xa o anterior) cujo item tem
/// os mesmos bytes que `dado`, a partir da cabeça (+2). O tamanho vai como na original (movsx para o ecx do repe
/// cmpsb): 0 é igual (o repe não roda e o ZF vem do xor de antes).
pub(crate) unsafe fn cqb10_achar(this: usize, dado: usize) -> usize {
    let mut no = read_usize(this + 2); // cqb10: a busca começa na cabeça
    while no != 0 {
        let n = core::ptr::read_unaligned((no + 4) as *const i16) as i32 as u32;
        let item = read_usize(no);
        let mut k = 0u32;
        while k < n && *((item + k as usize) as *const u8) == *((dado + k as usize) as *const u8) {
            k += 1;
        }
        if k == n {
            return no;
        }
        no = read_usize(no + 6);
    }
    0
}







