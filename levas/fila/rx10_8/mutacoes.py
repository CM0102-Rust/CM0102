# Mutacoes (rx10_8), com Portugal e a base NORMAL ate 2002-08-01. Nenhuma abre caixa.
# - a 2a rodada uma semana antes (5/set): os jogos das tres zonas mudam de data (a carreira; o comparador acusa);
# - a zona Sul na fase 0 e a Centro na 1: os clubes de cada fase trocam (a carreira: os calendarios e as posicoes; o
#   comparador da fase, pelo plano, nao: ele usa a mesma funcao).
MUTACOES = [
    ('rx10_8_rodada_semana_antes',
     '    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 12 Sep',
     '    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // mutation',
     'rx10_8 segunda b calendar'),
    ('rx10_8_zonas_trocadas',
     '    if k == 0 { KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL } else { KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH }',
     '    if k != 0 { KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL } else { KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH } // mutation',
     'rx10_8 segunda b zone stage'),
]
