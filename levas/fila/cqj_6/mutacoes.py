# Mutacoes (cqj_6), na tabela das divisoes francesas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a cota do fundo de premios da primeira vira a metade: o premio pago pela tabela quando a liga acaba, em maio de
#   2002 (0x682e10), cai a metade para cada clube;
# - as linhas da tabela trocadas aos pares nas tres divisoes (a grade de jogos sai da ordem das linhas; provado pelo
#   agente: a impressao muda ate 2002-06-01);
# - a primeira sem premio no fim (+0xe1 = 2: a 0x682e10 nao paga nada).
# Ficou de fora: a TV (a 0x683010 no inicio da carreira nao mudou a impressao nas provas da cqi_5 e da cqj_4).
MUTACOES = [
    ('cqj6_cota_da_primeira_metade',
     'cqj6_fundo(this, 0x30d40);',
     'cqj6_fundo(this, 0x186a0);',
     'cqj_6 first division fixtures'),
    ('cqj6_linhas_trocadas',
     'let linha = k; // cqj6: a linha k recebe o k-ésimo clube da divisão',
     'let linha = k ^ 1; // mutacao: as linhas trocadas aos pares',
     'cqj_6 first division fixtures'),
    ('cqj6_primeira_sem_premio',
     'cqj6_fundo(this, 0x30d40);',
     'cqj6_fundo(this, 0x30d40); *((this + 0xe1) as *mut u8) = 2; // mutacao: sem premio no fim',
     'cqj_6 first division fixtures'),
]
