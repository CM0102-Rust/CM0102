# Mutacoes (sx04b_7) pela prova do save (mutacoes.py --prova-save N); sem o 4o campo (contador):
# - as notas do mes nao gravam o ano (o bloco de cada nacao com notas sai 2 bytes menor e o resto deslocado);
# - a marca das notas do mes sempre gravada como 0, com as notas gravadas depois (o .qst muda no byte da marca);
# A carga (0x76a280) fica sem mutacao propria: pular a marca ou as notas desalinharia o resto do save (caixa de carga
# esperando clique); ela le o que a gravacao (provada aqui) escreve, e o construtor das notas que ela chama e do
# sx04b_2.
MUTACOES = [
    ('sx04b_7_sem_ano',
     "if exe::zwrite(this + month_ratings_fields::YEAR, 2, 1, file) != 1 { // sx04b_7: the ratings' year",
     'if false { // mutation: the year not written'),
    ('sx04b_7_marca_zero',
     'if exe::zwrite(&has_ratings as *const u32 as usize, 4, 1, file) != 1 { // sx04b_7: the month ratings flag',
     'if exe::zwrite(&0u32 as *const u32 as usize, 4, 1, file) != 1 { // mutation: the flag always 0'),
]
