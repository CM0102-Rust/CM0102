# Lote 4d da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4d.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limite 7200
MUTACOES = [
    ('l_tamanho', '    if ler(saida, REGISTRO_DA_PARTIDA, 1, f) != 1 {', '    if ler(saida, REGISTRO_DA_PARTIDA - 4, 1, f) != 1 {'),
    ('r_excluido', '|| *((r + 0x4c) as *const u8) == 0 {', '|| *((r + 0x4c) as *const u8) != 0 {'),
    ('m_dobro', '            peso = peso.wrapping_add(peso);', '            peso = peso.wrapping_add(1);'),
    ('s_faixa', '                *tamanho = (*tamanho).wrapping_sub(1);', '                *tamanho = (*tamanho).wrapping_sub(0);'),
    ('i_ultima', '    *((ultima - 2) as *mut u8) = 0;', '    *((ultima - 2) as *mut u8) = 1;'),
    ('tr_recusa', '        return 0; // recusado, sem segunda pergunta', '        return 1; // recusado, sem segunda pergunta'),
]
