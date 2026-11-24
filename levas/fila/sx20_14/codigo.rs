// --- sx20_14: the sound player and the printer check (simulated_stats.cpp: SoundPlayer 0x84ca80..0x84cf00,
// printer_is_installed 0x84c7f0) ---

/// The sound player: ten DirectSound buffer slots (+0), the DirectSound object (+0x28) and the ok flag (+0x2c).
const SOUND_SLOTS: i8 = 10;
const SOUND_DEVICE: usize = 0x28;
const SOUND_OK: usize = 0x2c;
/// The exe's import thunks for DirectSoundCreate and EnumPrintersA, and the game window.
const DIRECT_SOUND_CREATE: usize = 0x944c22;
const ENUM_PRINTERS: usize = 0x944c10;
const G_GAME_WINDOW: usize = 0xb67a2c;
/// IDirectSound and IDirectSoundBuffer methods (vtable byte offsets).
const COM_RELEASE: usize = 0x8;
const DS_CREATE_SOUND_BUFFER: usize = 0xc;
const DS_SET_COOPERATIVE_LEVEL: usize = 0x18;
const DSB_GET_STATUS: usize = 0x24;
const DSB_LOCK: usize = 0x2c;
const DSB_PLAY: usize = 0x30;
const DSB_SET_CURRENT_POSITION: usize = 0x34;
const DSB_SET_VOLUME: usize = 0x3c;
const DSB_SET_PAN: usize = 0x40;
const DSB_STOP: usize = 0x48;
const DSB_UNLOCK: usize = 0x4c;

/// The method at `slot` of a COM object's vtable.
unsafe fn com_method(object: usize, slot: usize) -> usize {
    read_usize(read_usize(object) + slot)
}

/// A COM method without arguments (Release, Stop).
unsafe fn com_call0(object: usize, slot: usize) -> i32 {
    let f: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(com_method(object, slot));
    f(object)
}

/// A COM method with one dword argument (SetCurrentPosition, SetVolume, SetPan, GetStatus with a pointer).
unsafe fn com_call1(object: usize, slot: usize, a: usize) -> i32 {
    let f: unsafe extern "system" fn(usize, usize) -> i32 = core::mem::transmute(com_method(object, slot));
    f(object, a)
}

/// The buffer in sound slot `slot`, when the player has a device and the slot is 0 to 9.
unsafe fn sound_slot(this: usize, slot: i8) -> Option<usize> {
    if read_usize(this + SOUND_DEVICE) == 0 || !(0..SOUND_SLOTS).contains(&slot) {
        return None;
    }
    Some(this + 4 * slot as usize)
}

/// Whether a sound buffer reports a playing or looping status (bits 1 and 4).
unsafe fn sound_buffer_busy(buffer: usize) -> bool {
    let mut status = 0u32;
    com_call1(buffer, DSB_GET_STATUS, &mut status as *mut u32 as usize);
    status & 5 != 0
}

/// Builds the sound player: no buffers, not ok; creates the DirectSound device (`DirectSoundCreate`) and sets the
/// normal cooperative level with the game window; on any failure the device is released and cleared, otherwise the
/// ok flag is set. Returns the player.
/// Original: `SoundPlayer::SoundPlayer` (0x84ca80, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn sound_player_sound_player(this: usize) -> usize {
    write_u32(this + SOUND_OK, 0);
    core::ptr::write_bytes(this as *mut u8, 0, 4 * SOUND_SLOTS as usize);
    let create: unsafe extern "system" fn(usize, usize, usize) -> i32 = core::mem::transmute(DIRECT_SOUND_CREATE);
    if create(0, this + SOUND_DEVICE, 0) != 0 {
        write_usize(this + SOUND_DEVICE, 0);
        return this;
    }
    let device = read_usize(this + SOUND_DEVICE);
    let level: unsafe extern "system" fn(usize, usize, u32) -> i32 = core::mem::transmute(com_method(device, DS_SET_COOPERATIVE_LEVEL));
    if level(device, read_usize(G_GAME_WINDOW), 1) != 0 { // sx20_14: DSSCL_NORMAL
        if read_usize(this + SOUND_DEVICE) != 0 {
            com_call0(read_usize(this + SOUND_DEVICE), COM_RELEASE);
        }
        write_usize(this + SOUND_DEVICE, 0);
        return this;
    }
    write_u32(this + SOUND_OK, 1);
    this
}

/// Releases the DirectSound device and clears it.
/// Original: `SoundPlayer::~SoundPlayer` (0x84caf0, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn sound_player_destructor(this: usize) {
    if read_usize(this + SOUND_DEVICE) != 0 {
        com_call0(read_usize(this + SOUND_DEVICE), COM_RELEASE);
    }
    write_usize(this + SOUND_DEVICE, 0);
}

/// Stops, rewinds and releases the buffer in one slot (with a device and a slot 0 to 9).
unsafe fn sound_release_slot(this: usize, slot: i8) {
    if let Some(at) = sound_slot(this, slot) {
        let buffer = read_usize(at);
        if buffer != 0 {
            com_call0(buffer, DSB_STOP);
            com_call1(read_usize(at), DSB_SET_CURRENT_POSITION, 0);
            com_call0(read_usize(at), COM_RELEASE);
            write_usize(at, 0);
        }
    }
}

/// Stops, rewinds and releases every one of the ten buffers (with a device).
/// Original: `SoundPlayer::stop_all` (0x84cb10, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn sound_player_stop_all(this: usize) {
    if read_usize(this + SOUND_DEVICE) == 0 {
        return;
    }
    for slot in 0..SOUND_SLOTS {
        sound_release_slot(this, slot);
    }
}

/// Stops, rewinds and releases the buffer in `slot` (0 to 9).
/// Original: `SoundPlayer::stop_slot` (0x84cb70, simulated_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sound_player_stop_slot(this: usize, slot: i8) {
    sound_release_slot(this, slot);
}

/// Plays a WAV file in the first empty slot: nothing (0xff) without a device, a path or an empty slot, or with a pan
/// kind other than 0 (left, -5000), 1 (right, 5000) or 2 (centre, 0); the file loaded into the slot
/// (`SoundPlayer::load_wav`; failing, 0xff), then the pan, the volume 10000 and play, looping when `looped`. Returns
/// the slot.
/// Defect of the original: the volume 10000 is outside DirectSound's range (-10000 to 0), so the call fails and the
/// buffer keeps its full volume.
/// Original: `SoundPlayer::play_file` (0x84cbc0, simulated_stats.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn sound_player_play_file(this: usize, pan_kind: i8, path: usize, looped: u32) -> u8 {
    if read_usize(this + SOUND_DEVICE) == 0 || path == 0 {
        return 0xff;
    }
    let Some(slot) = (0..SOUND_SLOTS).find(|&k| read_usize(this + 4 * k as usize) == 0) else {
        return 0xff;
    };
    let pan: i32 = match pan_kind {
        0 => -5000,
        1 => 5000, // sx20_14: the right pan
        2 => 0,
        _ => return 0xff,
    };
    let load: unsafe extern "thiscall" fn(usize, i8, usize) -> u32 = core::mem::transmute(0x84cd10usize); // SoundPlayer::load_wav
    if load(this, slot, path) == 0 {
        return 0xff;
    }
    let buffer = read_usize(this + 4 * slot as usize);
    if buffer != 0 {
        com_call1(buffer, DSB_SET_PAN, pan as usize);
        com_call1(buffer, DSB_SET_VOLUME, 10000);
        let play: unsafe extern "system" fn(usize, u32, u32, u32) -> i32 = core::mem::transmute(com_method(buffer, DSB_PLAY));
        play(buffer, 0, 0, (looped != 0) as u32);
    }
    slot as u8
}

/// Whether the buffer in `slot` reports a playing or looping status (0 without a device, a valid slot or a buffer).
/// Original: `SoundPlayer::is_slot_playing` (0x84cc80, simulated_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sound_player_is_slot_playing(this: usize, slot: i8) -> u32 {
    match sound_slot(this, slot) {
        Some(at) if read_usize(at) != 0 => sound_buffer_busy(read_usize(at)) as u32,
        _ => 0,
    }
}

/// Sets the volume of `slot` from a percentage 0 to 100: (percent − 100) × 100 hundredths of a decibel.
/// Original: `SoundPlayer::set_slot_volume` (0x84ccc0, simulated_stats.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sound_player_set_slot_volume(this: usize, slot: i8, percent: i32) {
    if let Some(at) = sound_slot(this, slot) {
        if (0..=100).contains(&percent) && read_usize(at) != 0 {
            com_call1(read_usize(at), DSB_SET_VOLUME, ((percent - 100) * 100) as usize); // sx20_14: the volume scale
        }
    }
}

/// Reads a WAV file into a new static sound buffer in `slot`: nothing (0) without a device or a file; a header
/// without "RIFF", "WAVE" and "fmt" closes the file and gives 0. The buffer format comes from the 44-byte header (the
/// format tag, 1 or 2 channels, the sample rate, the average bytes as block align × rate, block align 1, the bits per
/// sample) and its size is the data size (+0x28), with flags 0xe2; the buffer is created, locked, filled from the file
/// and unlocked (any failure clears the slot, closes the file and gives 0). Returns 1.
/// Defects of the original: a buffer whose lock or unlock fails is dropped from the slot without being released; a
/// header with extra chunks before "data" is read wrongly (the header is assumed to be the canonical 44 bytes).
/// Original: `SoundPlayer::load_wav` (0x84cd10, simulated_stats.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sound_player_load_wav(this: usize, slot: i8, path: usize) -> u32 {
    let device = read_usize(this + SOUND_DEVICE);
    if device == 0 {
        return 0;
    }
    let file = exe::crt_fopen(path, 0x98f928); // "rb"
    if file == 0 {
        return 0;
    }
    let mut header = [0u8; 0x2c];
    exe::crt_fread(header.as_mut_ptr() as usize, 0x2c, 1, file);
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" || &header[12..15] != b"fmt" {
        exe::crt_fclose(file);
        return 0;
    }
    let word = |o: usize| u16::from_le_bytes([header[o], header[o + 1]]);
    let dword = |o: usize| u32::from_le_bytes([header[o], header[o + 1], header[o + 2], header[o + 3]]);
    let mut format = [0u8; 0x12];
    format[0..2].copy_from_slice(&word(0x14).to_le_bytes());
    format[2..4].copy_from_slice(&(if word(0x16) > 1 { 2u16 } else { 1 }).to_le_bytes());
    format[4..8].copy_from_slice(&dword(0x18).to_le_bytes());
    format[8..12].copy_from_slice(&(word(0x20) as u32).wrapping_mul(dword(0x18)).to_le_bytes());
    format[0xc..0xe].copy_from_slice(&1u16.to_le_bytes()); // the original's block align
    format[0xe..0x10].copy_from_slice(&word(0x22).to_le_bytes());
    let size = dword(0x28);
    let mut desc = [0u32; 5];
    desc[0] = 0x14;
    desc[1] = 0xe2;
    desc[2] = size;
    desc[4] = format.as_ptr() as u32;
    let at = this.wrapping_add((slot as isize * 4) as usize);
    let create: unsafe extern "system" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(com_method(device, DS_CREATE_SOUND_BUFFER));
    if create(device, desc.as_ptr() as usize, at, 0) != 0 {
        write_usize(at, 0);
        exe::crt_fclose(file);
        return 0;
    }
    let buffer = read_usize(at);
    let (mut data, mut bytes) = (0usize, size);
    let lock: unsafe extern "system" fn(usize, u32, u32, *mut usize, *mut u32, usize, usize, u32) -> i32 = core::mem::transmute(com_method(buffer, DSB_LOCK));
    if lock(buffer, 0, size, &mut data, &mut bytes, 0, 0, 0) != 0 {
        write_usize(at, 0);
        exe::crt_fclose(file);
        return 0;
    }
    exe::crt_fread(data, 1, bytes, file);
    exe::crt_fclose(file);
    let buffer = read_usize(at);
    let unlock: unsafe extern "system" fn(usize, usize, u32, usize, u32) -> i32 = core::mem::transmute(com_method(buffer, DSB_UNLOCK));
    if unlock(buffer, data, bytes, 0, 0) != 0 {
        write_usize(at, 0);
        return 0;
    }
    covered!("sx20_14 wav loaded");
    1
}

/// The first of the ten slots whose buffer exists and neither plays nor loops; 0xff when none, 0 without a device.
/// Original: `SoundPlayer::find_free_slot` (0x84cf00, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn sound_player_find_free_slot(this: usize) -> u8 {
    if read_usize(this + SOUND_DEVICE) == 0 {
        return 0;
    }
    for slot in 0..SOUND_SLOTS {
        let buffer = read_usize(this + 4 * slot as usize);
        if buffer != 0 && !sound_buffer_busy(buffer) { // sx20_14: an idle buffer
            return slot as u8;
        }
    }
    0xff
}

/// Whether Windows lists at least one local printer (`EnumPrintersA` level 5 into a 20-byte buffer).
/// Defect of the original [inference]: a 20-byte buffer holds no printer record with its names, so with any printer
/// installed the call fails for lack of room and the answer is no.
/// Original: `printer_is_installed` (0x84c7f0, simulated_stats.cpp, cdecl).
pub(crate) unsafe extern "C" fn printer_is_installed() -> u32 {
    let enumerate: unsafe extern "system" fn(u32, usize, u32, *mut u8, u32, *mut u32, *mut u32) -> i32 = core::mem::transmute(ENUM_PRINTERS);
    let (mut buffer, mut needed, mut returned) = ([0u8; 0x14], 0u32, 0u32);
    if enumerate(2, 0, 5, buffer.as_mut_ptr(), 0x14, &mut needed, &mut returned) == 0 {
        return 0;
    }
    (returned != 0) as u32
}

/// Comparer for `sound_player_sound_player`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_sound_player(this: usize) -> usize {
    // simplification: creates the DirectSound device (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this)
}

/// Comparer for `sound_player_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_destructor(this: usize) {
    // simplification: releases the DirectSound device (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `sound_player_stop_all`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_stop_all(this: usize) {
    // simplification: stops and releases sound buffers (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

/// Comparer for `sound_player_stop_slot`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_stop_slot(this: usize, slot: i8) {
    // simplification: stops and releases a sound buffer (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize, i8)>(%%I3%%)(this, slot)
}

/// Comparer for `sound_player_play_file`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_play_file(this: usize, pan_kind: i8, path: usize, looped: u32) -> u8 {
    // simplification: loads and plays a sound (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, u32) -> u8>(%%I4%%)(this, pan_kind, path, looped)
}

/// Comparer for `sound_player_is_slot_playing`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_is_slot_playing(this: usize, slot: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I5%%)(this, slot);
    let n = sound_player_is_slot_playing(this, slot);
    SCOREBOARD.record(r == n, || format!("sound_player_is_slot_playing({}) original={} rust={}", slot, r, n));
    r
}

/// Comparer for `sound_player_set_slot_volume`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_set_slot_volume(this: usize, slot: i8, percent: i32) {
    // simplification: sets the volume of a playing buffer (the device keeps it; nothing to compare in the game's memory)
    original_fn::<unsafe extern "thiscall" fn(usize, i8, i32)>(%%I6%%)(this, slot, percent)
}

/// Comparer for `sound_player_load_wav`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_load_wav(this: usize, slot: i8, path: usize) -> u32 {
    // simplification: creates a sound buffer from a file (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> u32>(%%I7%%)(this, slot, path)
}

/// Comparer for `sound_player_find_free_slot`.
pub(crate) unsafe extern "thiscall" fn compare_sound_player_find_free_slot(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(%%I8%%)(this);
    let n = sound_player_find_free_slot(this);
    SCOREBOARD.record(r == n, || format!("sound_player_find_free_slot original={} rust={}", r, n));
    r
}

/// Comparer for `printer_is_installed`.
pub(crate) unsafe extern "C" fn compare_printer_is_installed() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I9%%)();
    let n = printer_is_installed();
    SCOREBOARD.record(r == n, || format!("printer_is_installed original={} rust={}", r, n));
    r
}
