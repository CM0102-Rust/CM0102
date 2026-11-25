// ---------- rx03_11: the T2K scan converter: the crossing lists, lines, quadratic curves and the bitmap fill (library_without_asserts, T2K scanconv.c) ----------

/// The T2K scan converter (0x2226 bytes, scratch slot 4): +0..+0x1c the bitmap (x min and max pixels, y min and max
/// pixels, y max + 1 pixel in 26.6, x min in 26.6, row bytes, the bitmap), +0x20 the bounds computed, +0x24..+0x30 the
/// bounds (x min, x max, y min, y max, 26.6), +0x34..+0x40 the outline's bounds, +0x44 large outline (fixed point
/// steps), +0x48 room for 32 row lists, +0xc8 the row lists indexed by row, +0xcc their block, +0xd0 and +0xd4 the
/// first and last row, +0xd8 room for 32 column lists, +0x158..+0x164 the same for the columns, +0x168 the next free
/// crossing node and +0x16c the end of its chunk, +0x170 the first chunk (1024 nodes of 8 bytes: the crossing, 26.6
/// doubled with the direction in bit 0, and the next node), +0x2170 room for 32 chunk pointers, +0x21f0 the chunk
/// pointers, +0x21f4 their capacity, +0x21f8 the last chunk, +0x21fc the curve flatness, +0x2200 grey, +0x2201 dropout
/// control, +0x2202 column dropout control, +0x2203 column lists, +0x2204 and +0x2208 dropout used (rows, columns),
/// +0x220c and +0x2210 the contour starts and ends, +0x2214 the contour count, +0x2216 and +0x221a the x and y
/// coordinates, +0x221e the on curve flags, +0x2222 the memory context.
pub(crate) mod t2k_scan {
    pub(crate) const BOUNDS_DONE: usize = 0x20;
    pub(crate) const X_MIN: usize = 0x34;
    pub(crate) const X_MAX: usize = 0x38;
    pub(crate) const Y_MIN: usize = 0x3c;
    pub(crate) const Y_MAX: usize = 0x40;
    pub(crate) const LARGE: usize = 0x44;
    pub(crate) const ROW_ROOM: usize = 0x48;
    pub(crate) const ROWS: usize = 0xc8;
    pub(crate) const ROW_BLOCK: usize = 0xcc;
    pub(crate) const ROW_FIRST: usize = 0xd0;
    pub(crate) const ROW_LAST: usize = 0xd4;
    pub(crate) const COLUMN_ROOM: usize = 0xd8;
    pub(crate) const COLUMNS: usize = 0x158;
    pub(crate) const COLUMN_BLOCK: usize = 0x15c;
    pub(crate) const COLUMN_FIRST: usize = 0x160;
    pub(crate) const COLUMN_LAST: usize = 0x164;
    pub(crate) const NEXT_NODE: usize = 0x168;
    pub(crate) const CHUNK_END: usize = 0x16c;
    pub(crate) const FIRST_CHUNK: usize = 0x170;
    pub(crate) const CHUNK_ROOM: usize = 0x2170;
    pub(crate) const CHUNKS: usize = 0x21f0;
    pub(crate) const CHUNK_CAPACITY: usize = 0x21f4;
    pub(crate) const LAST_CHUNK: usize = 0x21f8;
    pub(crate) const FLATNESS: usize = 0x21fc;
    pub(crate) const GREY: usize = 0x2200;
    pub(crate) const DROPOUT: usize = 0x2201;
    pub(crate) const COLUMN_DROPOUT: usize = 0x2202;
    pub(crate) const COLUMN_LISTS: usize = 0x2203;
    pub(crate) const ROW_DROPOUT_USED: usize = 0x2204;
    pub(crate) const COLUMN_DROPOUT_USED: usize = 0x2208;
    pub(crate) const STARTS: usize = 0x220c;
    pub(crate) const ENDS: usize = 0x2210;
    pub(crate) const CONTOURS: usize = 0x2214;
    pub(crate) const XS: usize = 0x2216;
    pub(crate) const YS: usize = 0x221a;
    pub(crate) const ON_CURVE: usize = 0x221e;
    pub(crate) const MEM: usize = 0x2222;
    pub(crate) const SIZE: u32 = 0x2226;
}

/// The span limit of exact integer steps (outlines wider than this use `fixed_mul`/`fixed_div` steps).
const LARGE_SPAN: i32 = 0xb503;

/// Creates a scan converter for an outline (26.6 points): the flags, the outline bounds, the column lists (only with
/// column lists, i.e. grey or column dropout) and the row lists, each a row of list heads (inline up to 32, else a
/// block), the first node chunk; then every contour is fed in (not for curve order 3): lines between on curve points,
/// quadratic curves through off curve points (with implied middle points between two off curve points; a contour
/// starting off curve starts at the last point, or at the middle of the last and first when both are off).
/// Original: `t2k_scan_converter_new` (0x93fd10, library_without_asserts, T2K scanconv.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scan_converter_new(mem: usize, contours: i32, starts: usize, ends: usize, xs: usize, ys: usize, on: usize,
    grey: i32, order: i32, dropout: i32, column_dropout: i32, flatness: i32) -> usize {
    let (grey, column_dropout) = (grey as u8, column_dropout as u8);
    let sc = lib_ctx_acquire_scratch_buffer(mem, t2k_scan::SIZE, 4);
    write_usize(sc + t2k_scan::MEM, mem);
    write_u8(sc + t2k_scan::GREY, grey);
    write_i32(sc + t2k_scan::FLATNESS, 4);
    if flatness > 4 {
        write_i32(sc + t2k_scan::FLATNESS, flatness);
    }
    if read_i32(sc + t2k_scan::FLATNESS) > 0x10 {
        write_i32(sc + t2k_scan::FLATNESS, 0x10);
    }
    write_u8(sc + t2k_scan::DROPOUT, dropout as u8);
    write_u8(sc + t2k_scan::COLUMN_DROPOUT, column_dropout);
    write_u8(sc + t2k_scan::COLUMN_LISTS, (column_dropout != 0 || grey != 0) as u8);
    write_usize(sc + t2k_scan::STARTS, starts);
    write_i16(sc + t2k_scan::CONTOURS, contours as i16);
    write_usize(sc + t2k_scan::YS, ys);
    write_u32(sc + t2k_scan::ROW_DROPOUT_USED, 0);
    write_u32(sc + t2k_scan::COLUMN_DROPOUT_USED, 0);
    write_usize(sc + t2k_scan::ENDS, ends);
    write_usize(sc + t2k_scan::XS, xs);
    write_usize(sc + t2k_scan::ON_CURVE, on);
    write_u32(sc + t2k_scan::BOUNDS_DONE, 0);
    let last = read_i16(ends + 2 * (contours as i16 as i32 - 1) as usize) as i32;
    for (values, lo, hi) in [(xs, t2k_scan::X_MIN, t2k_scan::X_MAX), (ys, t2k_scan::Y_MIN, t2k_scan::Y_MAX)] {
        let (mut min, mut max) = (read_i32(values), read_i32(values));
        for k in 1..=last.max(0) as usize {
            let v = read_i32(values + 4 * k);
            if v > max {
                max = v;
            } else if v < min {
                min = v;
            }
        }
        write_i32(sc + lo, min);
        write_i32(sc + hi, max);
    }
    write_u32(sc + t2k_scan::LARGE, 0);
    if read_i32(sc + t2k_scan::X_MAX).wrapping_sub(read_i32(sc + t2k_scan::X_MIN)) > LARGE_SPAN
        || read_i32(sc + t2k_scan::Y_MAX).wrapping_sub(read_i32(sc + t2k_scan::Y_MIN)) > LARGE_SPAN {
        write_u32(sc + t2k_scan::LARGE, 1);
    }
    init_crossing_lists(sc, mem);
    write_u32(sc + t2k_scan::CHUNK_CAPACITY, 0);
    t2k_scan_edge_pool_grow(sc);
    write_u32(sc + 0x1c, 0);
    if order as u8 != 3 {
        for c in 0..read_i16(sc + t2k_scan::CONTOURS) as i32 {
            feed_contour(sc, c as usize);
        }
    }
    sc
}

/// The column lists (x pixels, only with column lists) and the row lists (y pixels): one head per pixel from
/// (min − 1 pixel) >> 6 to (max + 1 pixel) >> 6, zeroed, inline up to 32 heads.
unsafe fn init_crossing_lists(sc: usize, mem: usize) {
    let (mut hi, mut lo) = (read_i32(sc + t2k_scan::X_MAX), read_i32(sc + t2k_scan::X_MIN));
    for axis in 0..2 {
        hi = hi.wrapping_add(0x40) >> 6;
        lo = lo.wrapping_sub(0x40) >> 6;
        let n = hi.wrapping_sub(lo).wrapping_add(1);
        let heads = if axis == 0 && read_u8(sc + t2k_scan::COLUMN_LISTS) == 0 {
            0
        } else {
            let heads = if n < 0x21 { sc + if axis == 0 { t2k_scan::COLUMN_ROOM } else { t2k_scan::ROW_ROOM } } else { lib_ctx_alloc(mem, n.wrapping_mul(4) as u32) };
            for k in 0..n.max(0) as usize {
                write_u32(heads + 4 * k, 0);
            }
            heads
        };
        let base = heads.wrapping_sub((lo as usize).wrapping_mul(4));
        if axis == 0 {
            write_usize(sc + t2k_scan::COLUMN_BLOCK, heads);
            write_i32(sc + t2k_scan::COLUMN_FIRST, lo);
            write_i32(sc + t2k_scan::COLUMN_LAST, hi);
            write_usize(sc + t2k_scan::COLUMNS, base);
            lo = read_i32(sc + t2k_scan::Y_MIN);
            hi = read_i32(sc + t2k_scan::Y_MAX);
        } else {
            write_usize(sc + t2k_scan::ROW_BLOCK, heads);
            write_i32(sc + t2k_scan::ROW_FIRST, lo);
            write_usize(sc + t2k_scan::ROWS, base);
            write_i32(sc + t2k_scan::ROW_LAST, hi);
        }
    }
}

/// One contour into the crossing lists, walking its points as the original does (see `t2k_scan_converter_new`).
unsafe fn feed_contour(sc: usize, c: usize) {
    let (xs, ys, on) = (read_usize(sc + t2k_scan::XS), read_usize(sc + t2k_scan::YS), read_usize(sc + t2k_scan::ON_CURVE));
    let start = read_i16(read_usize(sc + t2k_scan::STARTS) + 2 * c) as i32;
    let end = read_i16(read_usize(sc + t2k_scan::ENDS) + 2 * c) as i32;
    let x = |k: i32| read_i32(xs + 4 * k as usize);
    let y = |k: i32| read_i32(ys + 4 * k as usize);
    let on_curve = |k: i32| read_u8(on + k as usize) != 0;
    let next = |k: i32| if k + 1 > end { start } else { k + 1 };
    let (mut ax, mut ay, mut cx, mut cy, mut pending, mut cur);
    if !on_curve(start) {
        cx = x(start);
        cy = y(start);
        ax = x(end);
        ay = y(end);
        if !on_curve(end) {
            ax = ax.wrapping_add(1).wrapping_add(cx) >> 1;
            ay = cy.wrapping_add(1).wrapping_add(ay) >> 1;
        }
        pending = start;
        cur = end;
    } else {
        ax = x(start);
        ay = y(start);
        cx = 0;
        cy = 0;
        pending = -1;
        cur = start;
    }
    let mut scanning = pending < 0;
    loop {
        if scanning {
            loop {
                let k = next(cur);
                cx = x(k);
                cy = y(k);
                if !on_curve(k) {
                    pending = k;
                    break;
                }
                t2k_scan_add_line(sc, ax, ay, cx, cy);
                ax = cx;
                ay = cy;
                cur = k;
                if k == start {
                    return;
                }
            }
            if pending == start {
                return;
            }
            scanning = false;
        }
        let control = pending;
        let k = next(control);
        let (nx, ny) = (x(k), y(k));
        if on_curve(k) {
            t2k_scan_add_quadratic_curve(sc, ax, ay, cx, cy, nx, ny);
            ax = nx;
            ay = ny;
            cur = k;
            if k == start {
                return;
            }
            scanning = true;
            continue;
        }
        let (mx, my) = (nx.wrapping_add(1).wrapping_add(cx) >> 1, ny.wrapping_add(1).wrapping_add(cy) >> 1);
        t2k_scan_add_quadratic_curve(sc, ax, ay, cx, cy, mx, my);
        ax = mx;
        ay = my;
        cy = ny;
        cx = nx;
        cur = control;
        pending = k;
        if k == start {
            return;
        }
    }
}

/// The crossing node pool: the first call points it at the converter's own chunk (1024 nodes) and its own pointer
/// room (32); later calls add a chunk of ((y span − x min + x max) >> 6) + 0x100 nodes, growing the chunk pointer
/// array (doubled; moved out of the converter the first time) when full, and return the new chunk.
/// Original: `t2k_scan_edge_pool_grow` (0x940160, library_without_asserts, T2K scanconv.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scan_edge_pool_grow(sc: usize) -> usize {
    if read_u32(sc + t2k_scan::CHUNK_CAPACITY) == 0 {
        write_u32(sc + t2k_scan::CHUNK_CAPACITY, 0x20);
        write_usize(sc + t2k_scan::CHUNKS, sc + t2k_scan::CHUNK_ROOM);
        write_u32(sc + t2k_scan::LAST_CHUNK, 0);
        write_usize(sc + t2k_scan::CHUNK_ROOM, sc + t2k_scan::FIRST_CHUNK);
        write_usize(sc + t2k_scan::NEXT_NODE, sc + t2k_scan::FIRST_CHUNK);
        write_usize(sc + t2k_scan::CHUNK_END, sc + t2k_scan::CHUNK_ROOM);
        return sc + t2k_scan::CHUNK_ROOM;
    }
    let mem = read_usize(sc + t2k_scan::MEM);
    let n = (read_i32(sc + t2k_scan::Y_MAX).wrapping_sub(read_i32(sc + t2k_scan::Y_MIN)).wrapping_sub(read_i32(sc + t2k_scan::X_MIN))
        .wrapping_add(read_i32(sc + t2k_scan::X_MAX)) >> 6).wrapping_add(0x100);
    let last = read_i32(sc + t2k_scan::LAST_CHUNK).wrapping_add(1);
    write_i32(sc + t2k_scan::LAST_CHUNK, last);
    if read_i32(sc + t2k_scan::CHUNK_CAPACITY) <= last {
        write_i32(sc + t2k_scan::CHUNK_CAPACITY, last.wrapping_mul(2));
        if read_usize(sc + t2k_scan::CHUNKS) == sc + t2k_scan::CHUNK_ROOM {
            let moved = lib_ctx_alloc(mem, last.wrapping_mul(8) as u32);
            write_usize(sc + t2k_scan::CHUNKS, moved);
            for k in 0..0x20 {
                write_u32(read_usize(sc + t2k_scan::CHUNKS) + 4 * k, read_u32(sc + t2k_scan::CHUNK_ROOM + 4 * k));
            }
        } else {
            let moved = lib_ctx_realloc(read_usize(sc + t2k_scan::MEM), read_usize(sc + t2k_scan::CHUNKS), last.wrapping_mul(8) as u32);
            write_usize(sc + t2k_scan::CHUNKS, moved);
        }
    }
    let chunk = lib_ctx_alloc(read_usize(sc + t2k_scan::MEM), n.wrapping_mul(8) as u32);
    write_usize(read_usize(sc + t2k_scan::CHUNKS) + 4 * read_i32(sc + t2k_scan::LAST_CHUNK) as usize, chunk);
    write_usize(sc + t2k_scan::NEXT_NODE, chunk);
    write_usize(sc + t2k_scan::CHUNK_END, chunk + 8 * n as usize);
    chunk
}

/// A new crossing node in the list at `head_at`, kept sorted by value (after the equal ones).
unsafe fn insert_crossing(sc: usize, head_at: usize, value: u32) {
    let head = read_usize(head_at);
    let mut node = read_usize(sc + t2k_scan::NEXT_NODE);
    if node >= read_usize(sc + t2k_scan::CHUNK_END) {
        node = t2k_scan_edge_pool_grow(sc);
    }
    write_usize(sc + t2k_scan::NEXT_NODE, read_usize(sc + t2k_scan::NEXT_NODE) + 8);
    write_u32(node, value);
    if head == 0 || (value as i32) < read_i32(head) {
        write_usize(node + 4, head);
        write_usize(head_at, node);
        return;
    }
    let mut p = head;
    let mut q = read_usize(p + 4);
    while q != 0 && read_i32(q) <= value as i32 {
        p = q;
        q = read_usize(q + 4);
    }
    write_usize(node + 4, q);
    write_usize(p + 4, node);
}

/// The crossings of one axis: for each pixel centre from `lo` (rounded up to a centre) to `hi`, the other coordinate
/// interpolated from (`at_lo`, `at_hi`) (exact integer steps, or `fixed_mul`/`fixed_div` for a large outline with a
/// long span), doubled with the direction bit, into the list of that pixel (`heads` indexed by pixel).
unsafe fn add_axis_crossings(sc: usize, heads: usize, lo: i32, hi: i32, at_lo: i32, at_hi: i32, up: u32, exact_first: bool) {
    let mut centre = (lo.wrapping_add(0x1f) & !0x3f).wrapping_add(0x20);
    if centre > hi {
        return;
    }
    let span = hi.wrapping_sub(lo);
    let mut head_at = heads.wrapping_add(((centre >> 6) as usize).wrapping_mul(4));
    if span == 0 {
        insert_crossing(sc, head_at, (at_lo.wrapping_mul(2) as u32) | up);
        return;
    }
    let delta = at_hi.wrapping_sub(at_lo);
    let large = read_u32(sc + t2k_scan::LARGE) != 0 && (span > LARGE_SPAN || delta > LARGE_SPAN || delta < -LARGE_SPAN);
    let _ = exact_first;
    if !large {
        let mut acc = centre.wrapping_sub(lo).wrapping_mul(delta);
        loop {
            insert_crossing(sc, head_at, ((acc / span).wrapping_add(at_lo).wrapping_mul(2) as u32) | up);
            acc = acc.wrapping_add(delta.wrapping_mul(0x40));
            centre = centre.wrapping_add(0x40);
            head_at = head_at.wrapping_add(4);
            if centre > hi {
                return;
            }
        }
    }
    loop {
        let step = fixed_mul(delta, fixed_div(centre.wrapping_sub(lo), span));
        insert_crossing(sc, head_at, (step.wrapping_add(at_lo).wrapping_mul(2) as u32) | up);
        centre = centre.wrapping_add(0x40);
        head_at = head_at.wrapping_add(4);
        if centre > hi {
            return;
        }
    }
}

/// Adds a line segment (26.6, each coordinate with its low bit set): its column crossings (with column lists) and its
/// row crossings, the direction bit set when the segment goes towards larger x (columns) or y (rows).
/// Original: `t2k_scan_add_line` (0x940290, library_without_asserts, T2K scanconv.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scan_add_line(sc: usize, x0: i32, y0: i32, x1: i32, y1: i32) {
    let (x0, y0, x1, y1) = (x0 | 1, y0 | 1, x1 | 1, y1 | 1);
    if read_u8(sc + t2k_scan::COLUMN_LISTS) != 0 {
        let up = x0 < x1;
        let (lo, hi, at_hi, at_lo) = if up { (x0, x1, y1, y0) } else { (x1, x0, y0, y1) };
        add_axis_crossings(sc, read_usize(sc + t2k_scan::COLUMNS), lo, hi, at_lo, at_hi, up as u32, true);
    }
    let up = y0 < y1;
    let (lo, hi, at_hi, at_lo) = if up { (y0, y1, x1, x0) } else { (y1, y0, x0, x1) };
    add_axis_crossings(sc, read_usize(sc + t2k_scan::ROWS), lo, hi, at_lo, at_hi, up as u32, false);
}

/// Adds a quadratic curve by halving: the number of halvings grows (each quartering the deviation
/// `|p0 − 2·p1 + p2| / 4`, the larger of x and y) until the deviation is within the flatness; the halves are kept on
/// a stack of at most 16 and each flat piece is added as a line.
/// Original: `t2k_scan_add_quadratic_curve` (0x940820, library_without_asserts, T2K scanconv.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scan_add_quadratic_curve(sc: usize, x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    let (mut x0, mut y0, mut x1, mut y1, mut x2, mut y2) = (x0, y0, x1, y1, x2, y2);
    let mut stack = [[0i32; 7]; 16];
    let mut top = 0usize;
    let dev = |a: i32, b: i32, c: i32| {
        let d = a.wrapping_add(b.wrapping_mul(-2)).wrapping_add(c) >> 2;
        if d < 0 { d.wrapping_neg() } else { d }
    };
    let mut d = dev(x0, x1, x2).max(dev(y0, y1, y2));
    let mut levels = 0;
    while read_i32(sc + t2k_scan::FLATNESS) < d {
        levels += 1;
        d >>= 2;
    }
    loop {
        while levels > 0 {
            let mx = x0.wrapping_add(2).wrapping_add(x1.wrapping_mul(2)).wrapping_add(x2) >> 2;
            let my = y0.wrapping_add(2).wrapping_add(y1.wrapping_mul(2)).wrapping_add(y2) >> 2;
            levels -= 1;
            stack[top] = [mx, my, x2.wrapping_add(1).wrapping_add(x1) >> 1, y1.wrapping_add(1).wrapping_add(y2) >> 1, x2, y2, levels];
            top += 1;
            x1 = x0.wrapping_add(1).wrapping_add(x1) >> 1;
            y1 = y0.wrapping_add(1).wrapping_add(y1) >> 1;
            y2 = my;
            x2 = mx;
        }
        t2k_scan_add_line(sc, x0, y0, x2, y2);
        if top == 0 {
            return;
        }
        top -= 1;
        [x0, y0, x1, y1, x2, y2, levels] = stack[top];
    }
}

/// Fills the bitmap from the crossing lists. The bounds are found once (+0x20): the first and last rows with
/// crossings, the extreme crossings of each row (from the outline's centre), and, with column lists, the same over the
/// columns. The bitmap (scratch slot 5, zeroed; not with `no_bitmap`) is one bit per pixel or one byte (grey, 0..0x7e)
/// per pixel. One-bit: each row's spans by the winding rule, then optional dropout control on rows and columns
/// (a span shorter than a pixel with both ends unlit lights one pixel). Grey: each row's spans with fractional end
/// coverage, then each column's ends blended in.
/// Original: `t2k_scan_convert_to_bitmap` (0x9409b0, library_without_asserts, T2K scanconv.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scan_convert_to_bitmap(sc: usize, _unused: usize, no_bitmap: i32) {
    let grey = read_u8(sc + t2k_scan::GREY) != 0;
    if read_u32(sc + t2k_scan::BOUNDS_DONE) == 0 {
        find_bitmap_bounds(sc);
    }
    let (x_lo, x_hi, y_lo, y_hi) = (read_i32(sc + 0x24), read_i32(sc + 0x28), read_i32(sc + 0x2c), read_i32(sc + 0x30));
    write_i32(sc + 0x14, x_lo);
    let x_max_px = x_hi.wrapping_add(0x40) >> 6;
    let x_min_px = x_lo >> 6;
    write_i32(sc + 4, x_max_px);
    write_i32(sc + 0x10, y_hi.wrapping_add(0x40));
    let mut row_bytes = x_max_px.wrapping_sub(x_min_px);
    let y_max_px = y_hi.wrapping_add(0x40) >> 6;
    let y_min_px = y_lo >> 6;
    write_i32(sc + 0xc, y_max_px);
    let height = y_max_px.wrapping_sub(y_min_px);
    write_i32(sc, x_min_px);
    write_i32(sc + 8, y_min_px);
    if !grey {
        row_bytes = row_bytes.wrapping_add(7) / 8;
    }
    write_i32(sc + 0x18, row_bytes);
    write_u32(sc + 0x1c, 0);
    if no_bitmap as u8 != 0 {
        return;
    }
    let size = row_bytes.wrapping_mul(height);
    let bitmap = lib_ctx_acquire_scratch_buffer(read_usize(sc + t2k_scan::MEM), size as u32, 5);
    write_usize(sc + 0x1c, bitmap);
    if size > 0 {
        core::ptr::write_bytes(bitmap as *mut u8, 0, size as usize);
    }
    let first = read_i32(sc + t2k_scan::ROW_FIRST);
    if first <= read_i32(sc + t2k_scan::ROW_LAST) && y_max_px <= read_i32(sc + t2k_scan::ROW_LAST) {
        write_i32(sc + t2k_scan::ROW_LAST, y_max_px - 1);
    }
    let geometry = BitmapGeometry { bitmap, row_bytes, height, x_min_px, y_min_px };
    if !grey {
        fill_one_bit_rows(sc, &geometry);
        if read_u8(sc + t2k_scan::DROPOUT) != 0 {
            row_dropout(sc, &geometry, x_lo, x_hi);
        }
        if read_u8(sc + t2k_scan::COLUMN_DROPOUT) != 0 {
            column_dropout(sc, &geometry, y_lo, y_hi);
        }
    } else {
        fill_grey_rows(sc, &geometry);
        blend_grey_columns(sc, &geometry);
    }
}

/// Where the bitmap lies: the buffer, its row bytes and height, its first pixel column and row.
struct BitmapGeometry {
    bitmap: usize,
    row_bytes: i32,
    height: i32,
    x_min_px: i32,
    y_min_px: i32,
}

/// The last node of a crossing list.
unsafe fn last_node(mut p: usize) -> usize {
    while read_usize(p + 4) != 0 {
        p = read_usize(p + 4);
    }
    p
}

/// The bounds of the crossings (see `t2k_scan_convert_to_bitmap`), as the original computes them, including its use
/// of the last row list it looked at when the column range is empty.
unsafe fn find_bitmap_bounds(sc: usize) {
    let rows = read_usize(sc + t2k_scan::ROWS);
    let row = |k: i32| read_usize(rows.wrapping_add((k as usize).wrapping_mul(4)));
    let mut x_lo = read_i32(sc + t2k_scan::X_MAX).wrapping_add(read_i32(sc + t2k_scan::X_MIN)) >> 1;
    let mut y_hi = read_i32(sc + t2k_scan::Y_MAX).wrapping_add(read_i32(sc + t2k_scan::Y_MIN)) >> 1;
    let mut x_hi = x_lo;
    let mut y_lo = y_hi;
    let (mut lo, mut hi) = (read_i32(sc + t2k_scan::ROW_FIRST), read_i32(sc + t2k_scan::ROW_LAST));
    let mut seen = 0usize;
    if lo <= hi {
        while row(lo) == 0 {
            lo += 1;
            if lo > hi {
                break;
            }
        }
        if lo <= hi {
            let yc = lo.wrapping_shl(6).wrapping_add(0x20);
            if yc < y_hi {
                y_lo = yc;
            }
            while row(hi) == 0 {
                hi -= 1;
            }
            let yc = hi.wrapping_shl(6).wrapping_add(0x20);
            if yc > y_hi {
                y_hi = yc;
            }
        }
        if lo <= hi {
            for k in lo..=hi {
                let head = row(k);
                seen = head;
                if head != 0 {
                    x_lo = x_lo.min(read_i32(head) >> 1);
                    let last = last_node(head);
                    seen = last;
                    x_hi = x_hi.max(read_i32(last) >> 1);
                }
            }
        }
    }
    write_i32(sc + t2k_scan::ROW_FIRST, lo);
    write_i32(sc + t2k_scan::ROW_LAST, hi);
    if read_u8(sc + t2k_scan::COLUMN_LISTS) != 0 {
        let columns = read_usize(sc + t2k_scan::COLUMNS);
        let column = |k: i32| read_usize(columns.wrapping_add((k as usize).wrapping_mul(4)));
        let (mut lo, mut hi) = (read_i32(sc + t2k_scan::COLUMN_FIRST), read_i32(sc + t2k_scan::COLUMN_LAST));
        let mut found_lo = false;
        if lo <= hi {
            loop {
                if column(lo) != 0 {
                    found_lo = true;
                    break;
                }
                lo += 1;
                if lo > hi {
                    break;
                }
            }
        } else {
            found_lo = seen != 0;
        }
        if lo <= hi || found_lo {
            if found_lo {
                let xc = lo.wrapping_shl(6).wrapping_add(0x20);
                if xc < x_lo {
                    x_lo = xc;
                }
            }
            let mut found_hi = false;
            if lo <= hi {
                loop {
                    if column(hi) != 0 {
                        found_hi = true;
                        break;
                    }
                    hi -= 1;
                    if lo > hi {
                        break;
                    }
                }
            } else {
                found_hi = seen != 0;
            }
            if found_hi {
                let xc = hi.wrapping_shl(6).wrapping_add(0x20);
                if xc > x_hi {
                    x_hi = xc;
                }
            }
        }
        write_i32(sc + t2k_scan::COLUMN_FIRST, lo);
        write_i32(sc + t2k_scan::COLUMN_LAST, hi);
        if lo <= hi {
            for k in lo..=hi {
                let head = column(k);
                if head != 0 {
                    y_lo = y_lo.min(read_i32(head) >> 1);
                    y_hi = y_hi.max(read_i32(last_node(head)) >> 1);
                }
            }
        }
    }
    write_i32(sc + 0x24, x_lo);
    write_i32(sc + 0x28, x_hi);
    write_i32(sc + 0x2c, y_lo);
    write_i32(sc + 0x30, y_hi);
    write_u32(sc + t2k_scan::BOUNDS_DONE, 1);
}

/// The next span of a crossing list by the winding rule: from node `p`, the node `q` where the winding returns to
/// zero (each crossing adds +1 going up, −1 going down); `None` when the list ends.
unsafe fn winding_span(p: usize) -> Option<usize> {
    let mut q = read_usize(p + 4);
    let mut w = ((read_u32(q) & 1) + (read_u32(p) & 1)) as i32 * 2 - 2;
    while w != 0 {
        q = read_usize(q + 4);
        w = w - 1 + (read_u32(q) & 1) as i32 * 2;
    }
    Some(q)
}

/// The original's span fill: `count` (rounded down to even) bytes of `value` from `at`, by dwords then one word.
unsafe fn fill_even(at: usize, count: i32, value: u8) {
    let n = count as u32;
    let bytes = (n >> 2) * 4 + if (n >> 1) & 1 != 0 { 2 } else { 0 };
    core::ptr::write_bytes(at as *mut u8, value, bytes as usize);
}

/// The one-bit rows: each span by the winding rule, pixels from round(start) to round(end) − 1.
unsafe fn fill_one_bit_rows(sc: usize, g: &BitmapGeometry) {
    let rows = read_usize(sc + t2k_scan::ROWS);
    let first = read_i32(sc + t2k_scan::ROW_FIRST);
    let mut r = read_i32(sc + t2k_scan::ROW_LAST);
    let mut dst = g.bitmap.wrapping_add((g.height.wrapping_sub(r) - 1 + g.y_min_px).wrapping_mul(g.row_bytes) as usize);
    while first <= r && r >= read_i32(sc + t2k_scan::ROW_FIRST) {
        let mut p = read_usize(rows.wrapping_add((r as usize).wrapping_mul(4)));
        while p != 0 && read_usize(p + 4) != 0 {
            let q = winding_span(p).unwrap();
            let a = read_i32(p).wrapping_add(0x40) >> 7;
            let b = read_i32(q).wrapping_add(0x40) >> 7;
            if a < b {
                let a = a - g.x_min_px;
                let b = b - 1 - g.x_min_px;
                let (ia, ib) = (a >> 3, b >> 3);
                let left = (0xffu32 >> (a & 7)) as u8;
                let right = (0xff80u32 >> (b & 7)) as u8;
                let at = |k: i32| dst.wrapping_add(k as usize);
                if ia == ib {
                    write_u8(at(ia), read_u8(at(ia)) | (left & right));
                } else {
                    write_u8(at(ia), read_u8(at(ia)) | left);
                    let mut k = ia + 1;
                    if at(k) & 1 != 0 {
                        write_u8(at(k), 0xff);
                        k = ia + 2;
                    }
                    if k < ib {
                        fill_even(at(k), ib - k + 1, 0xff);
                    }
                    write_u8(at(ib), right);
                }
            }
            p = read_usize(q + 4);
        }
        r -= 1;
        dst = dst.wrapping_add(g.row_bytes as usize);
    }
}

/// The one-bit dropout control on rows: a pair of crossings less than a pixel apart whose two end pixels are unlit
/// lights the pixel at their middle (nudged left or right by the side of the bitmap's centre).
unsafe fn row_dropout(sc: usize, g: &BitmapGeometry, x_lo: i32, x_hi: i32) {
    let rows = read_usize(sc + t2k_scan::ROWS);
    let mut used = 0u32;
    let mut r = read_i32(sc + t2k_scan::ROW_FIRST);
    while r <= read_i32(sc + t2k_scan::ROW_LAST) {
        let line = (g.height.wrapping_sub(r) - 1 + g.y_min_px).wrapping_mul(g.row_bytes);
        let bit = |px: i32| (g.bitmap.wrapping_add((px >> 3) as usize).wrapping_add(line as usize), (0x80u32 >> (px & 7)) as u8);
        let mut p = read_usize(rows.wrapping_add((r as usize).wrapping_mul(4)));
        while p != 0 && read_usize(p + 4) != 0 {
            let q = read_usize(p + 4);
            let (a, b) = (read_i32(p) >> 1, read_i32(q) >> 1);
            if b.wrapping_sub(a) < 0x40 {
                let (pa, ma) = bit((read_i32(p) >> 7) - g.x_min_px);
                let (pb, mb) = bit((read_i32(q) >> 7) - g.x_min_px);
                if read_u8(pa) & ma == 0 && read_u8(pb) & mb == 0 {
                    let nudged = if a <= x_hi.wrapping_add(x_lo) >> 1 { a + 1 } else { a - 2 };
                    used = 1;
                    let (pm, mm) = bit((nudged.wrapping_add(b) >> 7) - g.x_min_px);
                    write_u8(pm, read_u8(pm) | mm);
                }
            }
            p = read_usize(q + 4);
        }
        r += 1;
    }
    write_u32(sc + t2k_scan::ROW_DROPOUT_USED, read_u32(sc + t2k_scan::ROW_DROPOUT_USED) | used);
}

/// The one-bit dropout control on columns, as on rows with the y crossings.
unsafe fn column_dropout(sc: usize, g: &BitmapGeometry, y_lo: i32, y_hi: i32) {
    let columns = read_usize(sc + t2k_scan::COLUMNS);
    let mut used = 0u32;
    let mut c = read_i32(sc + t2k_scan::COLUMN_FIRST);
    while c <= read_i32(sc + t2k_scan::COLUMN_LAST) {
        let byte = (c - g.x_min_px) >> 3;
        let mask = (0x80u32 >> ((c - g.x_min_px) & 7)) as u8;
        let at = |row: i32| g.bitmap.wrapping_add((g.height.wrapping_sub(row) - 1 + g.y_min_px).wrapping_mul(g.row_bytes).wrapping_add(byte) as usize);
        let mut p = read_usize(columns.wrapping_add((c as usize).wrapping_mul(4)));
        while p != 0 && read_usize(p + 4) != 0 {
            let q = read_usize(p + 4);
            let (b, a) = (read_i32(q) >> 1, read_i32(p) >> 1);
            if b.wrapping_sub(a) < 0x40 && read_u8(at(read_i32(p) >> 7)) & mask == 0 && read_u8(at(read_i32(q) >> 7)) & mask == 0 {
                let nudged = if y_hi.wrapping_add(y_lo) >> 1 < a { b - 2 } else { b + 1 };
                used = 1;
                let pm = at(nudged.wrapping_add(a) >> 7);
                write_u8(pm, read_u8(pm) | mask);
            }
            p = read_usize(q + 4);
        }
        c += 1;
    }
    write_u32(sc + t2k_scan::COLUMN_DROPOUT_USED, read_u32(sc + t2k_scan::COLUMN_DROPOUT_USED) | used);
}

/// The grey rows: each span by the winding rule; the end pixels get their fractional coverage (2 × the 26.6
/// fraction), the pixels between 0x7e.
unsafe fn fill_grey_rows(sc: usize, g: &BitmapGeometry) {
    let rows = read_usize(sc + t2k_scan::ROWS);
    let mut r = read_i32(sc + t2k_scan::ROW_FIRST);
    let mut dst = g.bitmap.wrapping_add(((g.height - 1).wrapping_sub(r).wrapping_add(g.y_min_px)).wrapping_mul(g.row_bytes).wrapping_sub(g.x_min_px) as usize);
    while r <= read_i32(sc + t2k_scan::ROW_LAST) {
        let mut p = read_usize(rows.wrapping_add((r as usize).wrapping_mul(4)));
        while p != 0 && read_usize(p + 4) != 0 {
            let q = winding_span(p).unwrap();
            let (v0, v1) = (read_i32(p), read_i32(q));
            let (a, b) = (v0 >> 7, v1 >> 7);
            let (c0, c1) = (((v0 >> 1) as u8) & 0x3f, ((v1 >> 1) as u8) & 0x3f);
            let at = |k: i32| dst.wrapping_add(k as usize);
            if a == b {
                write_u8(at(a), read_u8(at(a)).wrapping_add(c0.wrapping_mul(0x7f).wrapping_add(c1).wrapping_mul(2)));
            } else {
                write_u8(at(a), read_u8(at(a)).wrapping_add(c0.wrapping_mul(0xfe)).wrapping_add(0x7e));
                let mut k = a + 1;
                if k < b {
                    if at(k) & 1 != 0 {
                        write_u8(at(k), 0x7e);
                        k = a + 2;
                    }
                    if k < b {
                        fill_even(at(k), b - k + 1, 0x7e);
                    }
                }
                write_u8(at(b), c1 << 1);
            }
            p = read_usize(q + 4);
        }
        r += 1;
        dst = dst.wrapping_sub(g.row_bytes as usize);
    }
}

/// Blends a coverage into a grey pixel: (old·w_old + new·w_new) / (w_old + w_new), each weight 0xf82 − (0x3f − v)².
unsafe fn blend_grey(at: usize, new: u32) {
    let old = read_u8(at) as i32;
    let new = new as i32;
    let w_new = 0xf82 - (0x3f - new) * (0x3f - new);
    let w_old = 0xf82 - (0x3f - old) * (0x3f - old);
    write_u8(at, ((old * w_old + new * w_new) / (w_new + w_old)) as u8);
}

/// The grey columns: each column's spans by the winding rule, their end coverages blended into the row fill.
unsafe fn blend_grey_columns(sc: usize, g: &BitmapGeometry) {
    let columns = read_usize(sc + t2k_scan::COLUMNS);
    let last_row = g.height - 1;
    let mut c = read_i32(sc + t2k_scan::COLUMN_FIRST);
    while c <= read_i32(sc + t2k_scan::COLUMN_LAST) {
        let mut pending = 0usize;
        let mut acc = 0u8;
        let mut pending_value = 0u32;
        let mut p = read_usize(columns.wrapping_add((c as usize).wrapping_mul(4)));
        if p != 0 {
            let offset = |v: i32| (last_row.wrapping_sub(v >> 7).wrapping_add(g.y_min_px)).wrapping_mul(g.row_bytes).wrapping_add(c - g.x_min_px);
            while read_usize(p + 4) != 0 {
                let q = winding_span(p).unwrap();
                let (v0, v1) = (read_i32(p), read_i32(q));
                let mut off = offset(v0);
                if pending != 0 && pending != g.bitmap.wrapping_add(off as usize) {
                    blend_grey(pending, pending_value);
                    acc = 0;
                }
                let (c0, c1) = (((v0 >> 1) as u8) & 0x3f, ((v1 >> 1) as u8) & 0x3f);
                if v0 >> 7 == v1 >> 7 {
                    acc = acc.wrapping_add(c0.wrapping_mul(0x7f).wrapping_add(c1).wrapping_mul(2));
                } else {
                    let start = acc.wrapping_add(c0.wrapping_mul(0xfe)).wrapping_add(0x7e) as u32;
                    acc = c1 << 1;
                    blend_grey(g.bitmap.wrapping_add(off as usize), start);
                    off = offset(v1);
                }
                pending_value = acc as u32;
                pending = g.bitmap.wrapping_add(off as usize);
                p = read_usize(q + 4);
                if p == 0 {
                    break;
                }
            }
            if pending != 0 {
                blend_grey(pending, pending_value);
            }
        }
        c += 1;
    }
}

/// Frees a scan converter: its bitmap (scratch slot 5 given back when it is that slot's buffer), the node chunks
/// after the first, the chunk pointer array when moved out, the column and row list blocks when not inline, then the
/// converter (slot 4 given back when it is that slot's buffer). Nothing for null.
/// Original: `t2k_scan_converter_delete` (0x941480, library_without_asserts, T2K scanconv.c, cdecl).
pub(crate) unsafe extern "C" fn t2k_scan_converter_delete(sc: usize) {
    if sc == 0 {
        return;
    }
    let bitmap = read_usize(sc + 0x1c);
    if bitmap != 0 {
        let mem = read_usize(sc + t2k_scan::MEM);
        if bitmap == read_usize(mem + t2k_ctx::SCRATCH + 0x14) {
            write_u32(mem + t2k_ctx::SCRATCH_FREE + 0x14, 1);
        } else {
            lib_ctx_free(mem, bitmap);
        }
    }
    let mut k = 1;
    while k <= read_i32(sc + t2k_scan::LAST_CHUNK) {
        lib_ctx_free(read_usize(sc + t2k_scan::MEM), read_usize(read_usize(sc + t2k_scan::CHUNKS) + 4 * k as usize));
        k += 1;
    }
    if read_usize(sc + t2k_scan::CHUNKS) != sc + t2k_scan::CHUNK_ROOM {
        lib_ctx_free(read_usize(sc + t2k_scan::MEM), read_usize(sc + t2k_scan::CHUNKS));
    }
    write_u32(sc + t2k_scan::CHUNK_CAPACITY, 0);
    if read_usize(sc + t2k_scan::COLUMN_BLOCK) != sc + t2k_scan::COLUMN_ROOM {
        lib_ctx_free(read_usize(sc + t2k_scan::MEM), read_usize(sc + t2k_scan::COLUMN_BLOCK));
    }
    if read_usize(sc + t2k_scan::ROW_BLOCK) != sc + t2k_scan::ROW_ROOM {
        lib_ctx_free(read_usize(sc + t2k_scan::MEM), read_usize(sc + t2k_scan::ROW_BLOCK));
    }
    let mem = read_usize(sc + t2k_scan::MEM);
    if sc == read_usize(mem + t2k_ctx::SCRATCH + 0x10) {
        write_u32(mem + t2k_ctx::SCRATCH_FREE + 0x10, 1);
    } else {
        lib_ctx_free(mem, sc);
    }
}

/// Comparers: only the original (they allocate, free or fill the scratch bitmap; proved on the emulator by the whole
/// text path); the edge pool grows only when the original needs it.
pub(crate) unsafe extern "C" fn compare_t2k_scan_converter_new(mem: usize, contours: i32, starts: usize, ends: usize, xs: usize, ys: usize, on: usize,
    grey: i32, order: i32, dropout: i32, column_dropout: i32, flatness: i32) -> usize {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize, usize, usize, usize, i32, i32, i32, i32, i32) -> usize>(%%I0%%)(
        mem, contours, starts, ends, xs, ys, on, grey, order, dropout, column_dropout, flatness)
}

pub(crate) unsafe extern "C" fn compare_t2k_scan_edge_pool_grow(sc: usize) -> usize {
    original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%)(sc)
}

pub(crate) unsafe extern "C" fn compare_t2k_scan_add_line(sc: usize, x0: i32, y0: i32, x1: i32, y1: i32) {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, i32, i32)>(%%I2%%)(sc, x0, y0, x1, y1)
}

pub(crate) unsafe extern "C" fn compare_t2k_scan_add_quadratic_curve(sc: usize, x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    original_fn::<unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32)>(%%I3%%)(sc, x0, y0, x1, y1, x2, y2)
}

pub(crate) unsafe extern "C" fn compare_t2k_scan_convert_to_bitmap(sc: usize, unused: usize, no_bitmap: i32) {
    original_fn::<unsafe extern "C" fn(usize, usize, i32)>(%%I4%%)(sc, unused, no_bitmap)
}

pub(crate) unsafe extern "C" fn compare_t2k_scan_converter_delete(sc: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I5%%)(sc)
}
