# Mutacoes (rx09_5), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - o texto de quem nao sobe num grupo do acesso troca pelo de quem escapa do rebaixamento (acusada pelo comparador de
#   texto em junho de 2002);
# - as posicoes previstas 4k + 2 (acusada pelo comparador lado a lado na criacao do jogo e na virada).
# A lista dos clubes fica sem mutacao: o comparador a confere vaga a vaga quando roda.
MUTACOES = [
    ('rx09_5_texto_trocado',
     'play_off_club_text(club_id, 0xa82970, text); // rx09_5: "{club} fail to win promotion"',
     'play_off_club_text(club_id, 0x987594, text); // mutation: "avoid relegation"',
     'rx09_5 segunda b failed promotion text'),
    ('rx09_5_previstas_mais_um',
     '|_, k| k.wrapping_mul(4).wrapping_add(1)); // rx09_5: 4k + 1',
     '|_, k| k.wrapping_mul(4).wrapping_add(2)); // mutation: 4k + 2',
     'cx14_3 grouped predicted positions'),
]
