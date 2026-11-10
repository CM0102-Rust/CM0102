# Lote 4d da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4d.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('l_tamanho', '    if exe::crt_fread(out, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {', '    if exe::crt_fread(out, MATCH_STATS_RECORD_SIZE - 4, 1, f) != 1 {'),
    ('r_excluido', '|| r.no_statistics() == 0 {', '|| r.no_statistics() != 0 {'),
    ('m_dobro', '            weight = weight.wrapping_add(weight);', '            weight = weight.wrapping_add(1);'),
    ('s_faixa', '                *size = (*size).wrapping_sub(1);', '                *size = (*size).wrapping_sub(0);'),
    ('i_ultima', '    write_u8(last - 2, 0);', '    write_u8(last - 2, 1);'),
    ('tr_recusa', '        return 0; // refused, no second question', '        return 1; // refused, no second question'),
]
