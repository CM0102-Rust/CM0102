# Mutacoes (cx03_16), carreira de 8 ligas. Nenhuma abre caixa. O progresso o comparador acusa; a noticia, a comparacao
# no modo CM_TEXTO_NOTICIAS.
# - a Liga dos Campeoes da Asia sem a regra do 4o lugar no pior (o melhor nao vira 7);
# - a Recopa da Asia com o 2o lugar valendo 8 no melhor;
# - a noticia da Recopa nas rodadas de zona com o a5 no lugar do a8 (o texto do confronto muda).
MUTACOES = [
    ('cx03_16_liga_asiatica_sem_regra_do_quarto',
     '        if worst == 4 {\n            best = 7;\n        } else {',
     '        if worst == 4 {\n            // mutation\n        } else {'),
    ('cx03_16_recopa_segundo_vale_oito',
     '        best = remap_group_code(best, [1, 9, 0x11, 0x11]);',
     '        best = remap_group_code(best, [1, 8, 0x11, 0x11]); // mutation'),
    ('cx03_16_noticia_da_recopa_trocada',
     '        covered!("cx03_16 asian cup winners zone news");\n        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);',
     '        covered!("cx03_16 asian cup winners zone news");\n        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a5, outcome, a6, text); // mutation'),
]
