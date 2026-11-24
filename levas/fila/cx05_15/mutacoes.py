# Mutacoes (cx05_15) pela prova do save (CARGA). Nenhuma abre caixa: a leitura segue alinhada, so muda onde os bytes
# caem (sempre dentro do objeto).
# - a Eurocopa le as 16 semeadas 4 bytes adiante (+0xf2): a lista semeada carregada muda (a impressao da carga e o
#   comparador lado a lado acusam);
# - a minicopa le a data de inicio 2 bytes adiante (+0xb4, ate o fim do objeto): a data carregada muda (so com
#   minicopas vivas no save).
MUTACOES = [
    ('cx05_15_eurocopa_semeadas_adiante',
     '    if exe::crt_fread(this + 0xee, 5, 0x10, file) != 0x10 { // cx05_15: the 16 seeded nations',
     '    if exe::crt_fread(this + 0xf2, 5, 0x10, file) != 0x10 { // mutation'),
    ('cx05_15_minicopa_data_adiante',
     "    if exe::crt_fread(this + 0xb2, 8, 1, file) != 1 { // cx05_15: the mini cup's start date",
     "    if exe::crt_fread(this + 0xb4, 8, 1, file) != 1 { // mutation"),
]
