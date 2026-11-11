# Mutacoes (mkc_3), na carreira de 8 ligas: o emprestimo para clube frances ou australiano deixa de comecar no fim
# (comeca no mes seguinte: a IA conta um mes e a volta chega antes; o dono frances passa a ser consultado); o
# emprestimo para clube brasileiro comeca no mes seguinte em vez de tres meses depois; o clube americano pega
# emprestado fora da janela.
MUTACOES = [
    ('mkc3_gales_inicio_no_mes_seguinte',
     '    *start_month = *end_month; // mkc3: Wales, the start is the end\n    *start_year = *end_year;\n',
     '',
     'mkc_3 wales period'),
    ('mkc3_russia_mes_seguinte',
     '.wrapping_add(3); // mkc3: Russia, three months ahead',
     '.wrapping_add(1); // mutacao: o mes seguinte',
     'mkc_3 russia period'),
    ('mkc3_turquia_sem_prazo',
     'G_CURRENT_DATE, 0) != 0 {\n        transfer_manager_get_contract_stored(player);\n        return 1; // mkc3: Turkey, the window open',
     'G_CURRENT_DATE, 0) != 0 || true {\n        transfer_manager_get_contract_stored(player);\n        return 1; // mutacao: sem prazo',
     'mkc_3 turkey deadline'),
]
