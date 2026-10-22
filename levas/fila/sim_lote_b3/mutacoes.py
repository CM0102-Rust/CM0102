# Mutacoes do mapa de casas da formacao, da nota do clube para o jovem e da partida assistida. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('fm_espelho', "*q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho", "*q = 0xa0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho"),
    ('rg_mesma_nacao', "        s = s.wrapping_add(0x3e8); // mesma nação", "        s = s.wrapping_add(0x7d0); // mesma nação"),
    ('hm_outra_partida', "    if mesma { 1 } else { 0 } // outra partida", "    if mesma { 1 } else { 1 } // outra partida"),
]
