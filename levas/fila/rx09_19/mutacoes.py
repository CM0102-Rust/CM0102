# Mutacoes (rx09_19), com a Inglaterra ate 2005-12-31 (o fim de cada fase asiatica em 2005):
# - o perdedor da repescagem vai a Copa em vez do vencedor (novembro de 2005): o comparador dos registros acusa (o
#   retorno e *saida) e a lista da Copa muda;
# - os textos de classificado e de eliminado da Copa trocados: o comparador de texto acusa;
# - o codigo atual do vencedor de grupo da primeira fase 8 em vez de 9: o comparador lado a lado acusa.
MUTACOES = [
    ('rx09_19_perdedor_na_copa',
     '12 => if outcome == 1 { AsianFate::WorldCup } else { AsianFate::Out(0xa0, 0x3c) }, // rx09_19: the play-off winner goes to the World Cup',
     '12 => if outcome == 2 { AsianFate::WorldCup } else { AsianFate::Out(0xa0, 0x3c) }, // mutation: the loser',
     'rx09_19 asian nation to the world cup'),
    ('rx09_19_textos_da_copa_trocados',
     '11 | 12 => if outcome == 1 { AsianFateText::WorldCup(0xad4738) } else { AsianFateText::WorldCup(0xad46c0) }, // rx09_19: qualify for / out of the World Cup',
     '11 | 12 => if outcome == 1 { AsianFateText::WorldCup(0xad46c0) } else { AsianFateText::WorldCup(0xad4738) }, // mutation: swapped',
     'rx09_19 asian world cup text'),
    ('rx09_19_vencedor_de_grupo_8',
     '-1..=8 => (map(current, &[9, 0xb, 0x15, 0x1f]), map(best, &[1, 0xb, 0x15, 0x1f]), map(worst, &[9, 0xb, 0x15, 0x1f])), // rx09_19: the first round codes',
     '-1..=8 => (map(current, &[8, 0xb, 0x15, 0x1f]), map(best, &[1, 0xb, 0x15, 0x1f]), map(worst, &[9, 0xb, 0x15, 0x1f])), // mutation: 8',
     'rx09_19 asian progress'),
]
