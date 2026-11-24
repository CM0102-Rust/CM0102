# Mutacoes (cx01_7) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa.
# - o Brasil com 3 vagas e nao 4: o conmebol_seeding.tmp do jogo novo (gravado no .qst) muda;
# - o arquivo da copa com o numero de reservas e o de substituicoes trocados: a copa carregada fica com os valores
#   trocados (e o comparador acusa o arquivo diferente).
MUTACOES = [
    ('cx01_7_brasil_3_vagas',
     '            let places = if id == read_u32(KEY_NATION_BRAZIL) || id == read_u32(KEY_NATION_ARGENTINA) {',
     '            let places = if id == read_u32(KEY_NATION_ARGENTINA) { // mutation'),
    ('cx01_7_reservas_trocadas',
     '(cup::SUBSTITUTES_NAMED, 1), (cup::SUBSTITUTIONS_ALLOWED, 1), (cup::SEASON_SET_UP, 4),',
     '(cup::SUBSTITUTIONS_ALLOWED, 1), (cup::SUBSTITUTES_NAMED, 1), (cup::SEASON_SET_UP, 4), // mutation'),
]
