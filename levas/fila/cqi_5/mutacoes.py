# Mutacoes (cqi_5), na tabela das divisoes inglesas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a cota do fundo de premios da Premier vira a metade: o premio pago pela tabela quando a liga acaba, em maio de 2002
#   (0x682e10), cai a metade para cada clube;
# - as linhas da tabela trocadas aos pares (o 1o clube na linha 1, o 2o na 0...): a grade de jogos sai da ordem das
#   linhas e os confrontos de cada rodada mudam (provado pelo agente: a impressao muda ate 2002-06-01);
# - a cota da Third vira a metade (a Third acaba em abril de 2002).
# Ficou de fora (provado pelo agente): a TV pela metade escapa (a 0x683010 no inicio da carreira nao muda a impressao
# ate 2002-06-01).
# O contador fica depois de a tabela encher (todas as divisoes passam por ele).
MUTACOES = [
    ('cqi5_cota_da_premier_metade',
     'eng_prm_fill_league_table(this, 0x14, 0x9c43b0, 0x1ee, 0x9cf5bc, 0x3d090,',
     'eng_prm_fill_league_table(this, 0x14, 0x9c43b0, 0x1ee, 0x9cf5bc, 0x1e848,',
     'cqi_5 tabela cheia'),
    ('cqi5_linhas_trocadas',
     'let linha = k; // cqi5: a linha k recebe o k-ésimo clube da divisão',
     'let linha = k ^ 1; // mutacao: as linhas trocadas aos pares',
     'cqi_5 tabela cheia'),
    ('cqi5_cota_da_third_metade',
     'eng_prm_fill_league_table(this, 0x18, 0x9c4424, 0x1fa, 0x9cf5c8, 0x2710,',
     'eng_prm_fill_league_table(this, 0x18, 0x9c4424, 0x1fa, 0x9cf5c8, 0x1388,',
     'cqi_5 tabela cheia'),
]
