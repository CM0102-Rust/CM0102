// ---------- batch %%LEVA%%: the FONT object: loading a TrueType/T2K file, the style and size, the glyph cache and the text width (font code of fog_of_war.cpp) ----------

/// The FONT state block (0x23 bytes, `operator new` in `FONT::FONT`, pointed to by +0 of the FONT object). The four
/// interface fonts are the global FONT array at 0xaebdf4 (small, medium, large and symbol).
#[allow(dead_code)]
mod font_state {
    pub(crate) const SIZE: u32 = 0x23;
    /// u32: 1 once a font file is loaded.
    pub(crate) const LOADED: usize = 0x0;
    /// The whole font file read into memory (`operator new`).
    pub(crate) const FILE_BUFFER: usize = 0x4;
    /// The T2K memory handler, input stream, sfnt font and scaler [inference: T2K library objects, from the calls].
    pub(crate) const MEM_HANDLER: usize = 0x8;
    pub(crate) const STREAM: usize = 0xc;
    pub(crate) const SFNT: usize = 0x10;
    pub(crate) const SCALER: usize = 0x14;
    /// u16: the style flags the scaler was built for (bits 0x1/0x2 bold, 0x4/0x8 slant).
    pub(crate) const STYLE: usize = 0x18;
    /// u8: the pixel size of the scaler's transformation (0 after a rebuild).
    pub(crate) const PIXEL_SIZE: usize = 0x1a;
    /// The glyph cache: an array of glyph pointers (`realloc`), most recently used first, and its count (at most
    /// `GLYPH_CACHE_MAX`).
    pub(crate) const GLYPH_CACHE: usize = 0x1b;
    pub(crate) const GLYPH_COUNT: usize = 0x1f;
    pub(crate) const GLYPH_CACHE_MAX: i32 = 0x100;
}

/// A cached glyph (0x21 bytes, `operator new`).
#[allow(dead_code)]
mod font_glyph {
    pub(crate) const SIZE: u32 = 0x21;
    /// i32: the bitmap's left and top in 26.6 pixels from the pen and the baseline.
    pub(crate) const LEFT: usize = 0x0;
    pub(crate) const TOP: usize = 0x4;
    /// i32: the bitmap's width, bytes per row and height (each + 4 for a blurred glyph).
    pub(crate) const WIDTH: usize = 0x8;
    pub(crate) const ROW_BYTES: usize = 0xc;
    pub(crate) const HEIGHT: usize = 0x10;
    /// The 8-bit coverage bitmap (0..126, `operator new`).
    pub(crate) const BITMAP: usize = 0x14;
    /// i32: the pen advance in 16.16 pixels.
    pub(crate) const ADVANCE: usize = 0x18;
    /// u16 flags, u8 pixel size and u16 character the glyph was rendered for (the cache key: flags & 0x1f, size, char).
    pub(crate) const FLAGS: usize = 0x1c;
    pub(crate) const PIXEL_SIZE: usize = 0x1e;
    pub(crate) const CHAR: usize = 0x1f;
}

/// The T2K scaler's fields read by the font code [inference: the T2K glyph output, by how they are used].
#[allow(dead_code)]
mod t2k_scaler {
    /// i32 16.16: the two vertical metrics added to the text's y to give the baseline.
    pub(crate) const METRIC_A: usize = 0x30;
    pub(crate) const METRIC_B: usize = 0x38;
    pub(crate) const ADVANCE: usize = 0x94;
    pub(crate) const TOP: usize = 0xa4;
    pub(crate) const LEFT: usize = 0xa8;
    pub(crate) const WIDTH: usize = 0xce;
    pub(crate) const HEIGHT: usize = 0xd2;
    pub(crate) const ROW_BYTES: usize = 0xd6;
    pub(crate) const BITMAP: usize = 0xda;
    /// Pointer to the rendering settings (words +0x30 = 3 and +0x32 = 1 after each rebuild).
    pub(crate) const SETTINGS: usize = 0x139;
    /// The sfnt font's memory handler (passed to the scaler constructor).
    pub(crate) const SFNT_MEM: usize = 0x5c;
}

/// The T2K library in the exe (cdecl; every call ends with a pointer to an error code the game never reads)
/// [inference: the names, from the T2K API and the arguments].
const FN_T2K_NEW_MEM_HANDLER: usize = 0x9441a0; // (&err) -> mem
const FN_T2K_DELETE_MEM_HANDLER: usize = 0x944300; // (mem)
const FN_T2K_NEW_INPUT_STREAM: usize = 0x9416c0; // (mem, data, size, &err) -> stream
const FN_T2K_DELETE_INPUT_STREAM: usize = 0x941760; // (stream, &err)
const FN_T2K_NEW_SFNT: usize = 0x942ff0; // (mem, 2, 0, stream, style params or 0, &err) -> font
const FN_T2K_DELETE_SFNT: usize = 0x943f30; // (font, &err)
const FN_T2K_NEW_SCALER: usize = 0x93dd90; // (font mem, font, &err) -> scaler
const FN_T2K_DELETE_SCALER: usize = 0x93fa90; // (scaler, &err)
const FN_T2K_NEW_TRANSFORMATION: usize = 0x93de70; // (scaler, 0, 0x60, 0x60, &matrix, 1, &err)
const FN_T2K_RENDER_GLYPH: usize = 0x93e8c0; // (scaler, char, 0, 0, 5, 2, &err)
const FN_T2K_PURGE_MEMORY: usize = 0x93e780; // (scaler, 1, &err)
/// The emboldening style functions and their strength (1.25 for style bit 1, 1.5 for bit 2) [inference].
const T2K_BOLD_STYLE_FUNC: u32 = 0x93d870;
const T2K_BOLD_METRICS_FUNC: u32 = 0x93d820;

/// FONT methods called through the exe (each hooked in this item).
pub(crate) const FN_FONT_UNLOAD: usize = 0x5b1940; // thiscall(font)
pub(crate) const FN_FONT_SET_STYLE_AND_SIZE: usize = 0x5b1a00; // thiscall(font, flags, size) ret 8
pub(crate) const FN_FONT_GET_CACHED_GLYPH: usize = 0x5b1ba0; // thiscall(font, flags, size, char) ret 0xc -> glyph

/// The built-in font files (in SI_DATA) and the C runtime's `_ftol`.
const FONT_FILE_SMALL: usize = 0x9cc30c; // "small.t2k"
const FONT_FILE_MEDIUM: usize = 0x9cc2e8; // "medium.t2k"
const FONT_FILE_LARGE: usize = 0x9cc2f4; // "large.t2k"
const FONT_FILE_SYMBOL: usize = 0x9cc300; // "symbol.ttf"
const CRT_FTOL_ADDRESS: u32 = 0x945640;

unsafe fn font_unload_via_exe(font: usize) {
    let unload: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FONT_UNLOAD);
    unload(font);
}

pub(crate) unsafe fn font_set_style_via_exe(font: usize, flags: u32, size: u32) {
    let set: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_FONT_SET_STYLE_AND_SIZE);
    set(font, flags, size);
}

pub(crate) unsafe fn font_glyph_via_exe(font: usize, flags: u32, size: u32, ch: u32) -> usize {
    let glyph: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> usize = core::mem::transmute(FN_FONT_GET_CACHED_GLYPH);
    glyph(font, flags, size, ch)
}

/// Builds a FONT: looks for the FONTS folder once (`font_find_folder_once`), then allocates its zeroed 0x23-byte
/// state (`operator new`, no check) into +0. Returns the object. Runs for the global FONT array before `main` (the
/// exe's static constructors), when the hooks may not be in place yet.
/// Original: `FONT::FONT` (0x5b0ab0, fog_of_war.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn font_constructor(this: usize) -> usize {
    font_find_folder_once();
    let state = exe::crt_operator_new(font_state::SIZE);
    write_usize(this, state);
    core::ptr::write_bytes(state as *mut u8, 0, font_state::SIZE as usize);
    this
}

/// Destroys a FONT: unloads it (`FONT::unload`) and frees the state block.
/// Original: `FONT::~FONT` (0x5b0b40, fog_of_war.cpp, thiscall; its prologue has the call: RELATIVAS (0x5b0b40, 3)).
pub(crate) unsafe extern "thiscall" fn font_destructor(this: usize) {
    font_unload_via_exe(this);
    exe::crt_operator_delete(read_usize(this));
}

/// The file a FONT loads for slot `which` (0 small, 1 medium, 2 large, 3 symbol) and whether it comes from the FONTS
/// folder: with a chosen font file (`G_FONT_MODE` ≥ 0) the slots 0 to 2 (and any other number) use that file's entry
/// (name and face index); otherwise, and always for the symbol slot, the built-in T2K/TrueType file in SI_DATA (face
/// index 0).
unsafe fn font_file_for(which: i32) -> (usize, u16, bool) {
    let mode = read_i32(G_FONT_MODE);
    let builtin = match which {
        1 => FONT_FILE_MEDIUM,
        2 => FONT_FILE_LARGE,
        3 => return (FONT_FILE_SYMBOL, 0, false),
        _ => FONT_FILE_SMALL,
    };
    if mode < 0 {
        return (builtin, 0, false);
    }
    let entry = read_usize(font_files::TABLE) + mode as usize * font_files::ENTRY_SIZE;
    (read_usize(entry + font_files::ENTRY_FILE_NAME), read_u16(entry + font_files::ENTRY_FACE_INDEX), true)
}

/// Loads the font file of slot `which` into memory and opens a T2K stream on it; returns 1, or 0 on any failure.
///
/// Unloads first (`FONT::unload`). The path is the FONTS folder + the chosen file (`font_file_for`), or the built-in
/// file's path under SI_DATA (`utils_build_file_path`, copied to 0x104 bytes and terminated). The extension must be
/// .ttf, .ttc or .t2k and the face index 0 (a font inside a collection is refused). The file ("rb") is measured
/// (`fseek` to the end, `ftell`; 0 bytes fails), read whole into a new buffer (`FILE_BUFFER`) and closed; then a T2K
/// memory handler and an input stream on the buffer, and `LOADED` = 1. The scaler is only built by
/// `FONT::set_style_and_size`.
///
/// Defect of the original: when the second seek or the read fails, the buffer stays allocated until the next unload.
/// Original: `FONT::load_font_file` (0x5b0b60, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn font_load_font_file(this: usize, which: i32) -> u32 {
    let state = read_usize(this);
    font_unload_via_exe(this);
    let (name, face, in_folder) = font_file_for(which);
    let mut path = [0u8; 0x104];
    if in_folder {
        exe::crt_makepath(path.as_mut_ptr(), 0usize, font_files::FOLDER, name, 0usize);
    } else {
        exe::at_9466d0(path.as_mut_ptr() as usize, exe::utils_build_file_path(font_files::SI_DATA, name) as *const u8, 0x104);
        path[0x103] = 0;
    }
    let mut ext = [0u8; 0x100];
    exe::crt_strtok(path.as_ptr() as usize, 0usize, 0usize, core::ptr::null_mut(), ext.as_mut_ptr());
    if !font_is_font_extension(&ext) || face != 0 {
        return 0;
    }
    let file = exe::crt_fopen(path.as_ptr() as usize, font_files::READ_BINARY);
    if file == 0 {
        return 0;
    }
    let size = if exe::crt_fseek(file, 0, 2) == 0 { exe::crt_ftell(file) } else { 0 };
    if size == 0 {
        exe::crt_fclose(file);
        return 0;
    }
    let buffer = exe::crt_operator_new(size as u32);
    write_usize(state + font_state::FILE_BUFFER, buffer);
    let read = exe::crt_fseek(file, 0, 0) == 0 && exe::crt_fread(buffer, size as u32, 1, file) == 1;
    exe::crt_fclose(file);
    if !read {
        return 0;
    }
    let mut err = 0u32;
    let new_mem: unsafe extern "C" fn(*mut u32) -> usize = core::mem::transmute(FN_T2K_NEW_MEM_HANDLER);
    let new_stream: unsafe extern "C" fn(usize, usize, i32, *mut u32) -> usize = core::mem::transmute(FN_T2K_NEW_INPUT_STREAM);
    let mem = new_mem(&mut err);
    write_usize(state + font_state::MEM_HANDLER, mem);
    write_usize(state + font_state::STREAM, new_stream(mem, buffer, size, &mut err));
    write_u32(state + font_state::LOADED, 1); // sx10_13: the font is loaded
    covered!("sx10_13 font file loaded");
    1
}

/// Releases everything a FONT holds and zeroes its state block.
///
/// Each cached glyph's bitmap and the glyph itself (`operator delete`), the cache array (`free`), then the T2K scaler,
/// the sfnt font, the input stream and the memory handler, then the file buffer (`operator delete`); finally the 0x23
/// bytes of the state are zeroed (the block stays allocated).
/// Original: `FONT::unload` (0x5b1940, fog_of_war.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn font_unload(this: usize) {
    use font_state as fs;
    let state = read_usize(this);
    let mut k = 0i16;
    while (k as i32) < read_i32(state + fs::GLYPH_COUNT) {
        let glyph = read_usize(read_usize(state + fs::GLYPH_CACHE) + 4 * k as usize);
        if read_usize(glyph + font_glyph::BITMAP) != 0 {
            exe::crt_operator_delete(read_usize(glyph + font_glyph::BITMAP));
        }
        exe::crt_operator_delete(glyph);
        k = k.wrapping_add(1);
    }
    if read_usize(state + fs::GLYPH_CACHE) != 0 {
        exe::crt_free(read_usize(state + fs::GLYPH_CACHE));
    }
    let mut err = 0u32;
    for (field, release) in [(fs::SCALER, FN_T2K_DELETE_SCALER), (fs::SFNT, FN_T2K_DELETE_SFNT), (fs::STREAM, FN_T2K_DELETE_INPUT_STREAM)] {
        if read_usize(state + field) != 0 {
            let f: unsafe extern "C" fn(usize, *mut u32) = core::mem::transmute(release);
            f(read_usize(state + field), &mut err);
        }
    }
    if read_usize(state + fs::MEM_HANDLER) != 0 {
        let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_T2K_DELETE_MEM_HANDLER);
        f(read_usize(state + fs::MEM_HANDLER));
    }
    if read_usize(state + fs::FILE_BUFFER) != 0 {
        exe::crt_operator_delete(read_usize(state + fs::FILE_BUFFER));
    }
    core::ptr::write_bytes(state as *mut u8, 0, fs::SIZE as usize);
}

/// The slant of a scaler transformation: `(int)(sin(angle) × size × 65536)` on the x87 with the game's `_ftol`, the
/// angle π/12 for style bit 4 and π/6 for bit 8 (the constants read from the exe); no slant gives sin(0) = 0.
unsafe fn font_slant(style: u16, size: u32) -> i32 {
    let factor = if style & 4 != 0 {
        rdata::F64_0_08333333333
    } else if style & 8 != 0 {
        rdata::F64_0_1666666667
    } else {
        return 0;
    };
    // [0] size, [1] _ftol, [2] the result, [3] π, [4] the factor, [5] 65536.0
    let mut m: [u32; 6] = [size, CRT_FTOL_ADDRESS, 0, 0x96a798, factor as u32, 0x96a7a0];
    core::arch::asm!(
        "mov eax, dword ptr [edi + 12]",
        "fld qword ptr [eax]",
        "mov eax, dword ptr [edi + 16]",
        "fmul qword ptr [eax]",
        "fsin",
        "fimul dword ptr [edi]",
        "mov eax, dword ptr [edi + 20]",
        "fmul qword ptr [eax]",
        "call dword ptr [edi + 4]",
        "mov dword ptr [edi + 8], eax",
        in("edi") m.as_mut_ptr(),
        clobber_abi("C"),
    );
    m[2] as i32
}

/// Gets the T2K scaler ready for a style and a pixel size.
///
/// When there is no scaler or the bold bits (0x1, 0x2) of `flags` differ from the state's `STYLE`, the scaler and the
/// sfnt font are deleted and rebuilt: a new sfnt font on the stream, emboldened by 1.25 (bit 1) or 1.5 (bit 2), a new
/// scaler, its settings words set to 3 and 1, and `PIXEL_SIZE` = 0. When the slant bits (0x4, 0x8) differ or the low
/// byte of `size` is not `PIXEL_SIZE`, the scaler gets a new transformation at 96 × 96 dpi: the matrix (size × 65536,
/// slant, 0, size × 65536), the slant from `font_slant`. Finally `STYLE` = the low word of `flags` and `PIXEL_SIZE` =
/// the low byte of `size`.
/// Original: `FONT::set_style_and_size` (0x5b1a00, fog_of_war.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn font_set_style_and_size(this: usize, flags: u32, size: u32) {
    use font_state as fs;
    let state = read_usize(this);
    let style = flags as u16;
    let mut err = 0u32;
    let scaler = read_usize(state + fs::SCALER);
    if scaler == 0 || (read_u16(state + fs::STYLE) ^ style) & 3 != 0 {
        if scaler != 0 {
            let delete: unsafe extern "C" fn(usize, *mut u32) = core::mem::transmute(FN_T2K_DELETE_SCALER);
            delete(scaler, &mut err);
        }
        if read_usize(state + fs::SFNT) != 0 {
            let delete: unsafe extern "C" fn(usize, *mut u32) = core::mem::transmute(FN_T2K_DELETE_SFNT);
            delete(read_usize(state + fs::SFNT), &mut err);
        }
        let mut bold = [T2K_BOLD_STYLE_FUNC, T2K_BOLD_METRICS_FUNC, 0];
        let params = if style & 3 != 0 {
            bold[2] = if style & 1 != 0 { 0x14000 } else { 0x18000 };
            bold.as_ptr() as usize
        } else {
            0
        };
        let new_sfnt: unsafe extern "C" fn(usize, u32, u32, usize, usize, *mut u32) -> usize = core::mem::transmute(FN_T2K_NEW_SFNT);
        let new_scaler: unsafe extern "C" fn(usize, usize, *mut u32) -> usize = core::mem::transmute(FN_T2K_NEW_SCALER);
        let font = new_sfnt(read_usize(state + fs::MEM_HANDLER), 2, 0, read_usize(state + fs::STREAM), params, &mut err);
        write_usize(state + fs::SFNT, font);
        let scaler = new_scaler(read_usize(font + t2k_scaler::SFNT_MEM), font, &mut err);
        write_usize(state + fs::SCALER, scaler);
        write_u16(read_usize(scaler + t2k_scaler::SETTINGS) + 0x30, 3);
        write_u16(read_usize(scaler + t2k_scaler::SETTINGS) + 0x32, 1);
        write_u8(state + fs::PIXEL_SIZE, 0);
        covered!("sx10_13 font scaler rebuilt");
    }
    if (read_u16(state + fs::STYLE) ^ style) & 0xc != 0 || size as u8 != read_u8(state + fs::PIXEL_SIZE) {
        let pixels = (size & 0xff) << 16;
        let matrix = [pixels, font_slant(style, size & 0xff) as u32, 0, pixels];
        let transform: unsafe extern "C" fn(usize, u32, u32, u32, *const u32, u32, *mut u32) = core::mem::transmute(FN_T2K_NEW_TRANSFORMATION);
        transform(read_usize(state + fs::SCALER), 0, 0x60, 0x60, matrix.as_ptr(), 1, &mut err);
    }
    write_u16(state + fs::STYLE, style);
    write_u8(state + fs::PIXEL_SIZE, size as u8);
}

/// Finds a rendered glyph in the FONT's glyph cache, or renders it, and returns it.
///
/// The cache is searched in order for the character (low word of `ch`), the size (low byte) and the same low five bits
/// of `flags`; a hit moves to the front (move-to-front) and is returned. A miss takes a new slot: with 256 glyphs the
/// last one (the least recently used) is reused (its bitmap freed), otherwise the array grows by one (`realloc`) and a
/// new glyph is allocated (`operator new`, 0x21 bytes); either way it goes to the front. T2K renders the character
/// (grey levels) and the glyph takes the scaler's metrics, flags, size and character and a copy of the bitmap
/// (`font_glyph_copy`), or with flag 0x10 a blurred copy 4 pixels wider and taller (`font_glyph_blur`); the scaler's
/// memory is then purged.
/// Original: `FONT::get_cached_glyph` (0x5b1ba0, fog_of_war.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn font_get_cached_glyph(this: usize, flags: u32, size: u32, ch: u32) -> usize {
    use font_state as fs;
    let state = read_usize(this);
    let count = read_i32(state + fs::GLYPH_COUNT);
    let slot = |k: i32| read_usize(state + fs::GLYPH_CACHE) + 4 * k as usize;
    let to_front = |k: i32, glyph: usize| {
        for j in (1..=k).rev() {
            write_usize(slot(j), read_usize(slot(j - 1)));
        }
        write_usize(slot(0), glyph);
    };
    for k in 0..count {
        let glyph = read_usize(slot(k));
        if read_u16(glyph + font_glyph::CHAR) == ch as u16 && read_u8(glyph + font_glyph::PIXEL_SIZE) == size as u8
            && (read_u8(glyph + font_glyph::FLAGS) ^ flags as u8) & 0x1f == 0
        {
            to_front(k, glyph);
            return glyph;
        }
    }
    let glyph = if count == fs::GLYPH_CACHE_MAX {
        let oldest = read_usize(slot(count - 1));
        if read_usize(oldest + font_glyph::BITMAP) != 0 {
            exe::crt_operator_delete(read_usize(oldest + font_glyph::BITMAP));
        }
        oldest
    } else {
        write_i32(state + fs::GLYPH_COUNT, count + 1);
        write_usize(state + fs::GLYPH_CACHE, exe::crt_realloc(read_usize(state + fs::GLYPH_CACHE), (count as u32 + 1) * 4));
        exe::crt_operator_new(font_glyph::SIZE)
    };
    to_front(read_u16(state + fs::GLYPH_COUNT).wrapping_sub(1) as i16 as i32, glyph);
    let scaler = read_usize(state + fs::SCALER);
    let mut err = 0u32;
    let render: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, *mut u32) = core::mem::transmute(FN_T2K_RENDER_GLYPH);
    render(scaler, ch & 0xffff, 0, 0, 5, 2, &mut err);
    let grow = if flags & 0x10 != 0 { 4 } else { 0 };
    write_i32(glyph + font_glyph::LEFT, read_i32(scaler + t2k_scaler::LEFT));
    write_i32(glyph + font_glyph::TOP, read_i32(scaler + t2k_scaler::TOP));
    write_i32(glyph + font_glyph::WIDTH, read_i32(scaler + t2k_scaler::WIDTH) + grow);
    write_i32(glyph + font_glyph::ROW_BYTES, read_i32(scaler + t2k_scaler::ROW_BYTES) + grow);
    write_i32(glyph + font_glyph::HEIGHT, read_i32(scaler + t2k_scaler::HEIGHT) + grow);
    write_i32(glyph + font_glyph::ADVANCE, read_i32(scaler + t2k_scaler::ADVANCE));
    write_u16(glyph + font_glyph::FLAGS, flags as u16);
    write_u8(glyph + font_glyph::PIXEL_SIZE, size as u8);
    write_u16(glyph + font_glyph::CHAR, ch as u16);
    let bytes = read_i32(glyph + font_glyph::HEIGHT).wrapping_mul(read_i32(glyph + font_glyph::ROW_BYTES));
    let bitmap = exe::crt_operator_new(bytes as u32);
    write_usize(glyph + font_glyph::BITMAP, bitmap);
    if grow != 0 {
        font_glyph_blur(glyph, scaler);
    } else {
        core::ptr::copy_nonoverlapping(read_usize(scaler + t2k_scaler::BITMAP) as *const u8, bitmap as *mut u8, bytes as usize);
    }
    let purge: unsafe extern "C" fn(usize, u32, *mut u32) = core::mem::transmute(FN_T2K_PURGE_MEMORY);
    purge(scaler, 1, &mut err);
    glyph
}

/// The blurred glyph of flag 0x10 (the drop shadow): each byte of the bigger bitmap is the sum of a 5 × 5 window of the
/// scaler's bitmap, placed 2 pixels right and down, divided by 25 (truncated).
///
/// As in the original, the window is walked row by row from 2 above and column by column from 2 to the left, and a walk
/// stops (instead of skipping) at the first row or column before the start or past the end of the new bitmap: so the two
/// top rows and the two left columns stay 0 and the blur is lopsided. Inside the window only the pixels of the source
/// (rows and columns 2 to its size + 1) count.
unsafe fn font_glyph_blur(glyph: usize, scaler: usize) {
    let (width, height) = (read_i32(glyph + font_glyph::WIDTH), read_i32(glyph + font_glyph::HEIGHT));
    let (src_w, src_h) = (read_i32(scaler + t2k_scaler::WIDTH), read_i32(scaler + t2k_scaler::HEIGHT));
    let (src, src_row) = (read_usize(scaler + t2k_scaler::BITMAP), read_i32(scaler + t2k_scaler::ROW_BYTES));
    let mut out = read_usize(glyph + font_glyph::BITMAP);
    for r in 0..height {
        for c in 0..width {
            let mut sum: i32 = 0;
            for dy in -2..=2 {
                let rr = r + dy;
                if rr < 0 || rr >= height {
                    break;
                }
                for dx in -2..=2 {
                    let cc = c + dx;
                    if cc < 0 || cc >= width {
                        break;
                    }
                    if rr >= 2 && rr < src_h + 2 && cc >= 2 && cc < src_w + 2 {
                        sum += read_u8(src.wrapping_add(((rr - 2) * src_row + cc - 2) as usize)) as i32; // sx10_13: the window's pixel
                    }
                }
            }
            write_u8(out, (sum as i16 as i32 / 25) as u8);
            out += 1;
        }
    }
}

/// Returns the width in whole pixels of the text `text` (zero-terminated 16-bit characters) in a style and size.
///
/// Not loaded: 0 (in the low word; the original leaves the state pointer's high word in eax). Otherwise sets the style
/// and size (`FONT::set_style_and_size`) and adds, for each character's glyph (`FONT::get_cached_glyph`, control
/// characters included), its advance, except for the last character, which adds the extent of its ink ((left +
/// 64 × width) × 1024 - 1, 16.16); the total is rounded to whole pixels ((total + 0x8000) >> 16).
/// Original: `FONT::get_text_width` (0x5b18a0, fog_of_war.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn font_get_text_width(this: usize, flags: u32, size: u32, text: usize) -> i32 {
    let state = read_usize(this);
    if read_u32(state + font_state::LOADED) == 0 {
        return (state & 0xffff_0000) as i32;
    }
    font_set_style_via_exe(this, flags, size);
    let mut total: i32 = 0;
    let mut k: i16 = 0;
    loop {
        let ch = read_u16(text + 2 * k as usize);
        if ch == 0 {
            break;
        }
        let glyph = font_glyph_via_exe(this, flags, size, ch as u32);
        total = if read_u16(text + 2 * (k as usize + 1)) != 0 {
            total.wrapping_add(read_i32(glyph + font_glyph::ADVANCE))
        } else {
            let ink = (read_i32(glyph + font_glyph::WIDTH) << 6).wrapping_add(read_i32(glyph + font_glyph::LEFT)) << 10;
            total.wrapping_add(ink).wrapping_sub(1) // sx10_13: the last character's ink
        };
        k = k.wrapping_add(1);
    }
    covered!("sx10_13 text width");
    total.wrapping_add(0x8000) >> 16
}

/// Comparer for `font_constructor`: it allocates the state; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_font_constructor(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this)
}

/// Comparer for `font_destructor`: it frees; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_font_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `font_load_font_file`: it reads a file and builds T2K objects; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_font_load_font_file(this: usize, which: i32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I2%%)(this, which)
}

/// Comparer for `font_unload`: it frees; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_font_unload(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

/// Comparer for `font_set_style_and_size`: it rebuilds T2K objects; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_font_set_style_and_size(this: usize, flags: u32, size: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I4%%)(this, flags, size)
}

/// Comparer for `font_get_cached_glyph`: it allocates and renders through T2K; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_font_get_cached_glyph(this: usize, flags: u32, size: u32, ch: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> usize>(%%I5%%)(this, flags, size, ch)
}

/// Comparer for `font_get_text_width`: the original, then the copy (the second pass finds every glyph cached, in the
/// same final cache order); the callers keep the low word.
pub(crate) unsafe extern "thiscall" fn compare_font_get_text_width(this: usize, flags: u32, size: u32, text: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize) -> i32>(%%I6%%)(this, flags, size, text);
    let r = font_get_text_width(this, flags, size, text);
    SCOREBOARD.record(o as i16 == r as i16, || format!("font_get_text_width(flags {:#x}, size {}): original {} copy {}", flags, size, o as i16, r as i16));
    o
}
