# Mutacoes (sx16_8) pela prova do save (mutacoes.py --prova-save N; CARGA). As listas do save de ferias estao vazias,
# entao a mutacao muda a leitura das contagens: a carga le so 15 vagas (o notes.dat fica com 8 bytes por ler e a
# segunda lista da ultima vaga nao e lida; nada muda na carreira); o comparador nao pega (so a original roda): esta
# mutacao e a unica possivel sem notas no save e pode escapar. A outra, no construtor, deixa sem lista a ultima vaga
# (ponteiro 0): o notes.dat gravado depois (save_notes le a contagem da lista nula) derruba o jogo, entao nao entra.
MUTACOES = [
    ('sx16_8_quinze_vagas',
     '    for k in 0..0x10usize {\n        for (list, size, row_count, row_record)',
     '    for k in 0..0xfusize { // mutation\n        for (list, size, row_count, row_record)'),
]
