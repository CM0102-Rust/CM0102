# Mutacoes (sx21b_6) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao
# contra a base (a mesma DLL sem as rotinas da leva), e a carga do FeriasTesteSave (CARGA). Sem o 4o campo (contador).
# - 166 programacoes em vez de 167 no training.dat: o item sai 0x9b bytes menor e deslocado (na carga, o zread devolve
#   os 166 pedidos, sem caixa, e o resto sai deslocado);
# - o bloco da semana de treino gravado com 1 byte: o item sai 1 byte menor e deslocado.
MUTACOES = [
    ('sx21b_6_166_programacoes',
     '(this, 0x9b, 0xa7, 0xa4, 0xff), // sx21b_6: the 167 schedules',
     '(this, 0x9b, 0xa6, 0xa4, 0xff), // mutation: 166'),
    ('sx21b_6_semana_1_byte',
     '(this + tm::WEEK_COUNTER, 2, 1, 0x9d, 0xf8),',
     '(this + tm::WEEK_COUNTER, 1, 1, 0x9d, 0xf8), // mutation: one byte'),
]
