# Mutacoes da leva49 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva49.py
MUTACOES = [
    ('h_16', ".wrapping_add(16);\n    if i < 0", ".wrapping_add(15);\n    if i < 0"),
    ('h_faixa', "    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {", "    if i < 0 || i > (*((this + 4) as *const i32)) {"),
    ('h_stride', "wrapping_add((42735 * i) as usize))", "wrapping_add((42736 * i) as usize))"),
    ('h_atual', "        let atual = *(0xb77c7a as *const i16) as isize;", "        let atual = 0isize;"),
    ('h_9a01', "        Ok(r) => *((r + 0x9a01) as *const u32),", "        Ok(r) => *((r + 0x9a02) as *const u32),"),
    ('h_9a05', "        Ok(r) => (*((r + 0x9a05) as *const u8) != 0) as u32,", "        Ok(r) => (*((r + 0x9a05) as *const u8) & 1) as u32,"),
    ('b_804', "        0x802 | 0x804 => 0x400,", "        0x802 => 0x400, 0x804 => 0x800,"),
    ('b_c0', "0xc0 => 0x4000000,", "0xc0 => 0x8000000,"),
    ('s_ordem', "[1u16, 2, 4, 8, 0x10, 0x20, 0x40].iter().find(", "[2u16, 1, 4, 8, 0x10, 0x20, 0x40].iter().find("),
    ('s_bit1', "    } else if m & 0x200 != 0 || m & 1 != 0 {", "    } else if m & 0x200 != 0 {"),
    ('s_100', "    } else if m & 0x100 != 0 {\n        0x100", "    } else if m & 0x100 != 0 {\n        0x200"),
]
