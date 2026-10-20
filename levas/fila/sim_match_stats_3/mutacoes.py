# Mutacoes das estatisticas da partida (ler, registrar e gravar o registro). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('l_tamanho', "    if ler(saida, REGISTRO_DA_PARTIDA, 1, f) != 1 {", "    if ler(saida, REGISTRO_DA_PARTIDA - 4, 1, f) != 1 {"),
    ('r_excluido', "|| *((r + 0x4c) as *const u8) == 0 {", "|| *((r + 0x4c) as *const u8) != 0 {"),
    ('g_data', "    *((e + 8) as *mut u16) = *((jogo + 0x2a) as *const u16);", "    *((e + 8) as *mut u16) = *((jogo + 0x28) as *const u16);"),
]
