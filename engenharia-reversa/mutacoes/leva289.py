# Mutações da 289a leva (itens mkc_4, mkc_5, mkc_6). Uso: python mutacoes.py mutacoes\leva289.py --carreira 2002-06-01
MUTACOES = [
    ('mkc4_finlandia_fim_no_ano_seguinte', '    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) }; // mkc4: Finland, the end year', '    *end_year = read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1); // mutacao: sempre no ano seguinte', 'mkc_4 finland period'),
    ('mkc4_finlandia_ia_o_ano_todo', "        if day > 300 || day < 90 { // mkc4: Finland, the AI's days", '        if day > 300 && day < 90 { // mutacao: o ano todo', 'mkc_4 finland ai out of season'),
    ('mkc4_finlandia_devolve_no_outono', '    if !contract_manager_date_is_before(date, p) { // mkc4: Finland, until 15 November', '    if true { // mutacao: devolve no outono', 'mkc_4 finland autumn'),
    ('mkc5_irlanda_do_norte_mes_seguinte', '.wrapping_add(3); // mkc5: Northern Ireland, three months ahead', '.wrapping_add(1); // mutacao: o mes seguinte', 'mkc_5 northern ireland period'),
    ('mkc5_irlanda_do_norte_nunca_empresta', '    1 // mkc5: Northern Ireland, always', '    0 // mutacao: nunca', 'mkc_5 northern ireland loan'),
    ('mkc6_irlanda_encerra_fora_da_janela', 'date, 0) != 0 {\n        return 1; // mkc6: Ireland, the window open', 'date, 0) != 0 || true {\n        return 1; // mutacao: fora da janela', 'mkc_6 ireland window closed'),
    ('mkc6_irlanda_conta_cinco', '        write_u8(p, read_u8(p).wrapping_add(1)); // mkc6: Ireland, the season count', '        write_u8(p, read_u8(p).wrapping_add(5)); // mutacao: cada emprestimo conta cinco', 'mkc_6 ireland loan counted'),
]
