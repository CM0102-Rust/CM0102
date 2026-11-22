# Telas do clube (3): a carreira automatica nao abre telas; mutacoes para provar o comparador numa sessao do Lucas com
# o JOGAR_COMPARANDO.bat:
# - o nome de um jogador comum com o link da selecao (0x7dc) em vez do clube (0x7d7): aba Elenco de qualquer clube;
# - a lista das visoes que nao sao de posicoes com uma linha a mais (aba Elenco, View > Contract, Stats...).
MUTACOES = [
    ('tel01_3_link_do_nome',
     '(if starred { squad_colour(0xae31a2) } else { squad_colour(0xae3184) }, 0x7d7)',
     '(if starred { squad_colour(0xae31a2) } else { squad_colour(0xae3184) }, 0x7dc)'),
    ('tel01_3_linhas_da_lista',
     'let rows_word = (t.filter & 0xffff_ff00) | rows.wrapping_sub(1) as u32;',
     'let rows_word = (t.filter & 0xffff_ff00) | rows as u32;'),
]
