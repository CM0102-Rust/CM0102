# Mutacoes da entrada da pessoa no clube. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq15_cargo', "            *((pessoa + 0x3d) as *mut u8) = c; // clube e cargo do clube", "            *((pessoa + 0x3d) as *mut u8) = c ^ 1; // clube e cargo do clube"),
    ('dq15_data', "core::ptr::read_unaligned((data + 4) as *const u32));", "core::ptr::read_unaligned((data + 4) as *const u32) + 1);"),
    ('dq15_casa', "    match (0..n).find(|&i| core::ptr::read_unaligned(casa(i)) == 0) {", "    match (0..n).rev().find(|&i| core::ptr::read_unaligned(casa(i)) == 0) {"),
]
