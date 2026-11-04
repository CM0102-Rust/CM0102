# Mutacoes (cqj_4), na tabela das divisoes italianas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a cota do fundo de premios da Serie A vira a metade: o premio pago pela tabela quando a liga acaba, em maio de 2002
#   (0x682e10), cai a metade para cada clube (provado pelo agente: a impressao muda ate 2002-06-01);
# - a Serie A sem premio no fim (+0xe1 = 2: a 0x682e10 nao paga nada);
# - a cota da Serie B vira a metade.
# Ficaram de fora (provado pelo agente): a TV (a metade ou so para quem ja estava na A) escapa; as linhas trocadas aos
# pares derrubam o jogo na Italia (o confronto direto da 0x687430 le a linha seguinte ainda vazia); a marca em todas as
# linhas para o jogo na caixa de club_records.cpp 794.
MUTACOES = [
    ('cqj4_cota_da_serie_a_metade',
     'cqj4_fundo(this, 0x3d090); // cqj4: a cota da Série A',
     'cqj4_fundo(this, 0x1e848); // mutacao: a metade',
     'cqj_4 tabela da serie a'),
    ('cqj4_serie_a_sem_premio',
     'cqj4_fundo(this, 0x3d090); // cqj4: a cota da Série A',
     'cqj4_fundo(this, 0x3d090); *((this + 0xe1) as *mut u8) = 2; // mutacao: sem premio no fim',
     'cqj_4 tabela da serie a'),
    ('cqj4_cota_da_serie_b_metade',
     'cqj4_fundo(this, 0x11170);',
     'cqj4_fundo(this, 0x88b8);',
     'cqj_4 tabela da serie a'),
]
