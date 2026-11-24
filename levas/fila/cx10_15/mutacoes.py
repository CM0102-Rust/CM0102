# Mutacoes (cx10_15), carreira com a Holanda ate 2002-06-01. Nenhuma abre caixa.
# - o vencedor de periodo grava o progresso: o comparador acusa (cada rodada depois do primeiro periodo);
# - o melhor 2..7 fica como vem: o comparador acusa a cada rodada;
# - a noticia do periodo com o periodo seguinte: so o texto e a caixa dos tecnicos mudam (escapa da carreira; a noticia
#   nao volta a simulacao), fica sem mutacao propria.
MUTACOES = [
    ('cx10_15_vencedor_de_periodo_grava',
     '            if status == 1 {\n                covered!("cx10_15 dutch period winner skipped");\n                return;\n            }',
     '            if status == 0xfe { // mutation\n                covered!("cx10_15 dutch period winner skipped");\n                return;\n            }'),
    ('cx10_15_melhor_sem_troca',
     '            if (2..=7).contains(&best) {\n                best = 2;\n            }',
     '            if (2..=7).contains(&best) {\n                best = best; // mutation\n            }'),
]
