# Mutacoes da leva158 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva158.py --carreira 2002-06-01
MUTACOES = [
    ('z_busca', "buscar_64(f, u(item(z, k), 0) as u64);", "buscar_64(f, u(item(z, k), 0) as u64 + 4);"),
    ('z_posicao', "por(item(z, k), 0, pos as u32);", "por(item(z, k), 0, pos as u32 + 4);"),
    ('z_zseek', "        0 => pos.wrapping_add(desloc as u32),", "        0 => pos.wrapping_add(desloc as u32).wrapping_add(1),"),
    ('z_ztell', "(posicao_64(u(0) as usize) as u32).wrapping_sub(pos) as i32", "(posicao_64(u(0) as usize) as u32).wrapping_sub(pos).wrapping_add(1) as i32"),
]
