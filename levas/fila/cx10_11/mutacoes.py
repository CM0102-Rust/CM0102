# Mutacoes (cx10_11), carreira com a Finlandia ate 2002-06-01. Nenhuma abre caixa. O comparador acusa (toda rodada da
# liga finlandesa grava o progresso).
MUTACOES = [
    ('cx10_11_finlandia_melhor_e_pior_trocados',
     '        worst = 0xc;\n    }\n    store_club_progress(this, club, current as u8, best as u8, worst);',
     '        worst = 0xc;\n    }\n    store_club_progress(this, club, current as u8, worst, best as u8); // mutation'),
]
