// ---------- rx03_2: the T2K memory context: the error exit, the scratch buffers and the guarded blocks (library_without_asserts, T2K tsimem.c) ----------

/// The T2K memory context (`tsiMemObject`, 0xa4 bytes): +0 a magic word, +4 the tracked block count, +8 the table's
/// capacity, +0xc the table of tracked blocks, +0x10 the jump buffer of the error exit, +0x50 the six scratch buffers,
/// +0x68 their sizes, +0x80 their "free to reuse" flags, +0x98 where the next free table slot is searched from,
/// +0x9c and +0xa0 two more magic words (+0x9c becomes `CTX_IN_ERROR` on an error).
pub(crate) mod t2k_ctx {
    pub(crate) const SIZE: u32 = 0xa4;
    pub(crate) const MAGIC_0: u32 = 0xab1500ff;
    pub(crate) const COUNT: usize = 0x4;
    pub(crate) const CAPACITY: usize = 0x8;
    pub(crate) const TABLE: usize = 0xc;
    pub(crate) const JUMP_BUFFER: usize = 0x10;
    pub(crate) const SCRATCH: usize = 0x50;
    pub(crate) const SCRATCH_SIZE: usize = 0x68;
    pub(crate) const SCRATCH_FREE: usize = 0x80;
    pub(crate) const NEXT_SLOT: usize = 0x98;
    pub(crate) const STATE: usize = 0x9c;
    pub(crate) const MAGIC_2: usize = 0xa0;
    pub(crate) const STATE_OK: u32 = 0xaa005501;
    pub(crate) const STATE_IN_ERROR: u32 = 0x5500aaff;
    pub(crate) const MAGIC_2_VALUE: u32 = 0xa5a55a5a;
    pub(crate) const TABLE_CAPACITY: u32 = 0x1f5;
    pub(crate) const SCRATCH_SLOTS: usize = 6;
}

/// A guarded block: a 16-byte header (+0 `BLOCK_MAGIC`, +4 the size asked for, +8 its slot in the context's table,
/// +0xc not written), the caller's bytes, then the two guard bytes 0x5a 0xf0.
const BLOCK_MAGIC: u32 = 0xaa53c5aa;
const BLOCK_HEADER: usize = 0x10;
/// The T2K error codes of the memory context.
const T2K_ERR_OUT_OF_MEMORY: i32 = 0x2718;
const T2K_ERR_BAD_BLOCK: i32 = 0x2719;
const T2K_ERR_NULL_CONTEXT: i32 = 0x271b;
const T2K_ERR_TOO_MANY_BLOCKS: i32 = 0x271c;
const T2K_ERR_BAD_COUNT: i32 = 0x271d;
const T2K_ERR_REALLOC_FAILED: i32 = 0x271e;
const T2K_ERR_BAD_SLOT: i32 = 0x271f;
const T2K_ERR_TABLE_FULL: i32 = 0x2721;

/// Marks the context as in error (`STATE` = `STATE_IN_ERROR`) and longjmps to its jump buffer with `code`; never
/// returns. The longjmp may cross Rust frames (`panic = "abort"` leaves nothing to unwind).
/// Original: `lib_ctx_longjmp_error` (0x944180, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_longjmp_error(ctx: usize, code: i32) -> ! {
    write_u32(ctx.wrapping_add(t2k_ctx::STATE), t2k_ctx::STATE_IN_ERROR);
    let longjmp: unsafe extern "C" fn(usize, i32) -> ! = core::mem::transmute(FN_CRT_LONGJMP);
    longjmp(ctx.wrapping_add(t2k_ctx::JUMP_BUFFER), code)
}

/// Creates a memory context: `*status` = 0, the 0xa4-byte context (malloc) with its magic words, an empty table of
/// 0x1f5 tracked blocks (malloc) and six empty scratch slots (null, size 0, free to reuse). Without memory for the
/// context or the table, `*status` = 0x2718 and the context is null.
/// Defect of the original: the six scratch slots are cleared even when the context is null, a write at address 0x50
/// (an access violation instead of the status return).
/// Original: `lib_ctx_create` (0x9441a0, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_create(status: usize) -> usize {
    write_i32(status, 0);
    let mut ctx = exe::crt_malloc(t2k_ctx::SIZE);
    if ctx != 0 {
        write_u32(ctx, t2k_ctx::MAGIC_0);
        write_u32(ctx + t2k_ctx::STATE, t2k_ctx::STATE_OK);
        write_u32(ctx + t2k_ctx::MAGIC_2, t2k_ctx::MAGIC_2_VALUE);
        write_i32(ctx + t2k_ctx::COUNT, 0);
        write_u32(ctx + t2k_ctx::CAPACITY, t2k_ctx::TABLE_CAPACITY);
        write_u32(ctx + t2k_ctx::NEXT_SLOT, 0);
        let table = exe::crt_malloc(t2k_ctx::TABLE_CAPACITY * 4);
        write_usize(ctx + t2k_ctx::TABLE, table);
        if table != 0 {
            let mut k = 0;
            while k < read_i32(ctx + t2k_ctx::CAPACITY) {
                write_u32(read_usize(ctx + t2k_ctx::TABLE) + 4 * k as usize, 0);
                k += 1;
            }
        } else {
            exe::crt_free(ctx);
            ctx = 0;
            write_i32(status, T2K_ERR_OUT_OF_MEMORY);
        }
    } else {
        write_i32(status, T2K_ERR_OUT_OF_MEMORY);
    }
    clear_scratch_slots(ctx);
    ctx
}

/// The six scratch slots emptied: pointer null, size 0, free to reuse.
unsafe fn clear_scratch_slots(ctx: usize) {
    for k in 0..t2k_ctx::SCRATCH_SLOTS {
        write_u32(ctx.wrapping_add(t2k_ctx::SCRATCH + 4 * k), 0);
        write_u32(ctx.wrapping_add(t2k_ctx::SCRATCH_SIZE + 4 * k), 0);
        write_u32(ctx.wrapping_add(t2k_ctx::SCRATCH_FREE + 4 * k), 1);
    }
}

/// A scratch buffer of at least `size` bytes: when slot `slot` is free to reuse it is taken (marked busy) and returned,
/// first replaced by a new block of `size` bytes when smaller (unsigned); a busy slot gives a plain new block instead.
/// Original: `lib_ctx_acquire_scratch_buffer` (0x944250, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_acquire_scratch_buffer(ctx: usize, size: u32, slot: i32) -> usize {
    let k = slot as usize * 4;
    if read_u32(ctx + t2k_ctx::SCRATCH_FREE + k) == 0 {
        return lib_ctx_alloc(ctx, size);
    }
    write_u32(ctx + t2k_ctx::SCRATCH_FREE + k, 0);
    if size > read_u32(ctx + t2k_ctx::SCRATCH_SIZE + k) {
        lib_ctx_free(ctx, read_usize(ctx + t2k_ctx::SCRATCH + k));
        write_usize(ctx + t2k_ctx::SCRATCH + k, lib_ctx_alloc(ctx, size));
        write_u32(ctx + t2k_ctx::SCRATCH_SIZE + k, size);
    }
    read_usize(ctx + t2k_ctx::SCRATCH + k)
}

/// Frees the six scratch buffers and empties their slots (null, size 0, free to reuse).
/// Original: `lib_ctx_reset_scratch_buffers` (0x9442c0, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_reset_scratch_buffers(ctx: usize) {
    for k in 0..t2k_ctx::SCRATCH_SLOTS {
        lib_ctx_free(ctx, read_usize(ctx + t2k_ctx::SCRATCH + 4 * k));
        write_u32(ctx + t2k_ctx::SCRATCH + 4 * k, 0);
        write_u32(ctx + t2k_ctx::SCRATCH_SIZE + 4 * k, 0);
        write_u32(ctx + t2k_ctx::SCRATCH_FREE + 4 * k, 1);
    }
}

/// Frees the six scratch buffers, then the block table and the context (free); the other tracked blocks are not freed.
/// Original: `lib_ctx_destroy` (0x944300, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_destroy(ctx: usize) {
    lib_ctx_reset_scratch_buffers(ctx);
    exe::crt_free(read_usize(ctx + t2k_ctx::TABLE));
    exe::crt_free(ctx);
}

/// The emergency release after an error: every block still in the table (free), then the table and the context.
/// Nothing for a null context.
/// Original: `lib_ctx_free_all_blocks` (0x944350, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_free_all_blocks(ctx: usize) {
    if ctx == 0 {
        return;
    }
    let (n, table) = (read_i32(ctx + t2k_ctx::CAPACITY), read_usize(ctx + t2k_ctx::TABLE));
    for k in 0..n.max(0) as usize {
        let block = read_usize(table + 4 * k);
        if block != 0 {
            exe::crt_free(block);
        }
    }
    exe::crt_free(table);
    exe::crt_free(ctx);
}

/// Writes the header and the guard bytes of a block of `size` bytes.
unsafe fn write_block_guards(block: usize, size: u32) {
    write_u32(block + 4, size);
    write_u8(block + size as usize + BLOCK_HEADER, 0x5a);
    write_u8(block + size as usize + BLOCK_HEADER + 1, 0xf0);
}

/// The guard checks of a block (its magic word and the two bytes after its size): a broken guard raises 0x2719.
unsafe fn check_block_guards(ctx: usize, block: usize) {
    if read_u32(block) != BLOCK_MAGIC {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_BLOCK);
    }
    let end = block + read_u32(block + 4) as usize + BLOCK_HEADER;
    if read_u8(end) != 0x5a {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_BLOCK);
    }
    if read_u8(end + 1) != 0xf0 {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_BLOCK);
    }
}

/// A guarded block of `size` bytes (malloc of `size` + 0x12) recorded in the first free table slot from the search
/// point on (wrapping); returns the caller's bytes (block + 0x10). Errors (longjmp): a null context 0x271b, no memory
/// 0x2718, the count at the capacity 0x271c, no free slot found 0x2721.
/// Defect of the original: with a null context the error marks address 0x9c (an access violation instead of the
/// longjmp).
/// Original: `lib_mem_alloc` (0x9443a0, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_alloc(ctx: usize, size: u32) -> usize {
    if ctx == 0 {
        lib_ctx_longjmp_error(0, T2K_ERR_NULL_CONTEXT);
    }
    let block = exe::crt_malloc(size.wrapping_add(0x12));
    if block == 0 {
        lib_ctx_longjmp_error(ctx, T2K_ERR_OUT_OF_MEMORY);
    }
    write_u32(block, BLOCK_MAGIC);
    write_block_guards(block, size);
    let capacity = read_i32(ctx + t2k_ctx::CAPACITY);
    if read_i32(ctx + t2k_ctx::COUNT) >= capacity {
        lib_ctx_longjmp_error(ctx, T2K_ERR_TOO_MANY_BLOCKS);
    }
    let (table, mut slot) = (read_usize(ctx + t2k_ctx::TABLE), read_u32(ctx + t2k_ctx::NEXT_SLOT));
    let mut tried = 0;
    while tried < capacity {
        if read_u32(table + 4 * slot as usize) == 0 {
            write_usize(table + 4 * slot as usize, block);
            write_u32(block + 8, slot);
            write_i32(ctx + t2k_ctx::COUNT, read_i32(ctx + t2k_ctx::COUNT).wrapping_add(1));
            break;
        }
        slot = slot.wrapping_add(1) % capacity as u32;
        tried += 1;
    }
    write_u32(ctx + t2k_ctx::NEXT_SLOT, slot);
    if tried >= capacity {
        lib_ctx_longjmp_error(ctx, T2K_ERR_TABLE_FULL);
    }
    block + BLOCK_HEADER
}

/// Resizes a guarded block (realloc of `size` + 0x12), keeping its table slot, and rewrites its size and guards; a null
/// block returns null. Errors (longjmp): a broken guard 0x2719, a count outside 1..capacity 0x271d, no memory 0x271e
/// (the slot is left null), the slot outside the table 0x271f.
/// Original: `lib_mem_realloc` (0x944480, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_realloc(ctx: usize, p: usize, size: u32) -> usize {
    if p == 0 {
        return 0;
    }
    let block = p - BLOCK_HEADER;
    check_block_guards(ctx, block);
    let (count, table) = (read_i32(ctx + t2k_ctx::COUNT), read_usize(ctx + t2k_ctx::TABLE));
    if count <= 0 || count > read_i32(ctx + t2k_ctx::CAPACITY) {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_COUNT);
    }
    let slot = read_i32(block + 8);
    let moved = exe::crt_realloc(block, size.wrapping_add(0x12));
    write_usize(table.wrapping_add((slot as usize).wrapping_mul(4)), moved);
    if moved == 0 {
        lib_ctx_longjmp_error(ctx, T2K_ERR_REALLOC_FAILED);
    }
    if read_u32(moved) != BLOCK_MAGIC {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_BLOCK);
    }
    write_block_guards(moved, size);
    if slot >= read_i32(ctx + t2k_ctx::CAPACITY) {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_SLOT);
    }
    moved + BLOCK_HEADER
}

/// Frees a guarded block after checking its guards and that its table slot holds it; the slot is cleared and the count
/// lowered. A null block does nothing. Errors (longjmp): a null context 0x271b (with the original's write at 0x9c, as
/// `lib_ctx_alloc`), a broken guard 0x2719, a slot holding another block 0x271f.
/// Original: `lib_mem_free` (0x9445d0, library_without_asserts, T2K tsimem.c, cdecl).
pub(crate) unsafe extern "C" fn lib_ctx_free(ctx: usize, p: usize) {
    if ctx == 0 {
        lib_ctx_longjmp_error(0, T2K_ERR_NULL_CONTEXT);
    }
    if p == 0 {
        return;
    }
    let block = p - BLOCK_HEADER;
    check_block_guards(ctx, block);
    let at = read_usize(ctx + t2k_ctx::TABLE).wrapping_add(read_usize(block + 8).wrapping_mul(4));
    if read_usize(at) != block {
        lib_ctx_longjmp_error(ctx, T2K_ERR_BAD_SLOT);
    }
    write_u32(at, 0);
    write_i32(ctx + t2k_ctx::COUNT, read_i32(ctx + t2k_ctx::COUNT).wrapping_sub(1));
    exe::crt_free(block);
}

/// Comparers: only the original (they allocate, free or longjmp; proved on the emulator), then the plan where it is
/// cheap: the new block's header, guards and table slot, the context's empty slots.
unsafe fn block_is_tracked(ctx: usize, p: usize, size: u32) -> bool {
    let block = p - BLOCK_HEADER;
    read_u32(block) == BLOCK_MAGIC && read_u32(block + 4) == size && read_u8(block + size as usize + BLOCK_HEADER) == 0x5a
        && read_u8(block + size as usize + BLOCK_HEADER + 1) == 0xf0
        && read_usize(read_usize(ctx + t2k_ctx::TABLE) + 4 * read_usize(block + 8)) == block
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_longjmp_error(ctx: usize, code: i32) -> ! {
    original_fn::<unsafe extern "C" fn(usize, i32) -> !>(%%I0%%)(ctx, code)
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_create(status: usize) -> usize {
    let ctx = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%)(status);
    if ctx != 0 {
        let empty = (0..t2k_ctx::SCRATCH_SLOTS).all(|k| read_u32(ctx + t2k_ctx::SCRATCH + 4 * k) == 0
            && read_u32(ctx + t2k_ctx::SCRATCH_SIZE + 4 * k) == 0 && read_u32(ctx + t2k_ctx::SCRATCH_FREE + 4 * k) == 1);
        let ok = empty && read_i32(status) == 0 && read_u32(ctx) == t2k_ctx::MAGIC_0 && read_u32(ctx + t2k_ctx::STATE) == t2k_ctx::STATE_OK
            && read_i32(ctx + t2k_ctx::COUNT) == 0 && read_u32(ctx + t2k_ctx::CAPACITY) == t2k_ctx::TABLE_CAPACITY;
        SCOREBOARD.record(ok, || format!("lib_ctx_create: the new context {:#x} differs from the plan", ctx));
    }
    ctx
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_acquire_scratch_buffer(ctx: usize, size: u32, slot: i32) -> usize {
    let reusable = read_u32(ctx + t2k_ctx::SCRATCH_FREE + 4 * slot as usize) != 0;
    let r = original_fn::<unsafe extern "C" fn(usize, u32, i32) -> usize>(%%I2%%)(ctx, size, slot);
    let k = 4 * slot as usize;
    let ok = if reusable {
        r == read_usize(ctx + t2k_ctx::SCRATCH + k) && read_u32(ctx + t2k_ctx::SCRATCH_FREE + k) == 0 && read_u32(ctx + t2k_ctx::SCRATCH_SIZE + k) >= size
    } else {
        block_is_tracked(ctx, r, size)
    };
    SCOREBOARD.record(ok, || format!("lib_ctx_acquire_scratch_buffer(size {}, slot {}) differs from the plan", size, slot));
    r
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_reset_scratch_buffers(ctx: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I3%%)(ctx)
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_destroy(ctx: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I4%%)(ctx)
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_free_all_blocks(ctx: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I5%%)(ctx)
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_alloc(ctx: usize, size: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(%%I6%%)(ctx, size);
    SCOREBOARD.record(block_is_tracked(ctx, r, size), || format!("lib_mem_alloc({}): the block differs from the plan", size));
    r
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_realloc(ctx: usize, p: usize, size: u32) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> usize>(%%I7%%)(ctx, p, size);
    if p != 0 {
        SCOREBOARD.record(block_is_tracked(ctx, r, size), || format!("lib_mem_realloc({}): the block differs from the plan", size));
    }
    r
}

pub(crate) unsafe extern "C" fn compare_lib_ctx_free(ctx: usize, p: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I8%%)(ctx, p)
}
