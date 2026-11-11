# Mutacoes (mkc_4), na carreira com a liga finlandesa (LIGAS): o emprestimo para clube finlandes sempre termina no ano
# seguinte (dura um ano a mais); o clube finlandes da IA pega emprestado o ano todo com a janela aberta; o emprestimo
# com clube finlandes pode terminar no outono (a IA, que pergunta com flag 1, passa a pedir com volta entre agosto e
# novembro).
MUTACOES = [
    ('mkc4_finlandia_fim_no_ano_seguinte',
     '    *end_year = if *start_month < end { *start_year } else { read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1) }; // mkc4: Finland, the end year',
     '    *end_year = read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1); // mutacao: sempre no ano seguinte',
     'mkc_4 finland period'),
    ('mkc4_finlandia_ia_o_ano_todo',
     "        if day > 300 || day < 90 { // mkc4: Finland, the AI's days",
     '        if day > 300 && day < 90 { // mutacao: o ano todo',
     'mkc_4 finland ai out of season'),
    ('mkc4_finlandia_devolve_no_outono',
     '    if !contract_manager_date_is_before(date, p) { // mkc4: Finland, until 15 November',
     '    if true { // mutacao: devolve no outono',
     'mkc_4 finland autumn'),
]
