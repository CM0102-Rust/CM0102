# Mutacoes (cx12_11), com a Noruega ate 2002-08-01 (a repescagem de outubro de 2001):
# - a faixa da liga norueguesa a partir de 2 (no lugar de 3): a carreira quase nao le a previsao; acusada pelo
#   comparador do progresso, lado a lado nos 3 bytes, a cada atualizacao de status;
# - o clube da Premier na repescagem com os codigos 11 e 15 (no lugar de 11 e 14): acusada pelo comparador do
#   progresso na repescagem.
# Os registros e o texto ficam sem mutacao propria: o corpo e o do cx12_9 (first_division_update_staff_records,
# first_division_fate_news_text), com as mutacoes dele; aqui mudam so as chaves, conferidas pelo comparador dos registros (o
# retorno e o id da divisao).
MUTACOES = [
    ('cx12_11_faixa_norueguesa_de_2',
     'band: 3, // cx12_11: the Norwegian play-off band',
     'band: 2, // mutation: the band from 2',
     'cx12_11 norwegian first division progress'),
    ('cx12_11_codigos_da_premier',
     'premier_codes: [0xb, 0xe],\n};\n\n/// Updates the staff records by a club\'s outcome in the Norwegian',
     'premier_codes: [0xb, 0xf],\n};\n\n/// Updates the staff records by a club\'s outcome in the Norwegian',
     'cx12_11 norwegian first division progress'),
]
