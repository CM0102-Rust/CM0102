# Mutacoes (sma_10). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - o vinculo do clube trocado de condicao (todo corpo de saida de tecnico com o nome do clube no texto, ou sem ele).
MUTACOES = [
    ('sma10_vinculo_do_clube',
     'if exe::crt_strstr(buf, c) != 0 { // sma10: the club\'s link when its name is in the text',
     'if exe::crt_strstr(buf, c) == 0 { // mutation: the club\'s link only when its name is not in the text',
     'sma_10 departure body'),
]
