# Mutacoes (cqm_7), na tabela dos estaduais (rodam na criacao da carreira de 8 ligas e na virada de janeiro de 2002,
# antes dos estaduais de 2002): em cada um, as linhas giradas uma casa (o k-esimo clube na linha k+1, o ultimo na 0):
# a grade de jogos sai da ordem das linhas e os confrontos de cada rodada mudam. O contador fica depois de a tabela
# encher.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS (2001-046, 2001-039, 2001-039).
MUTACOES = [
    ('cqm7_nordeste_girado',
     'cqm7_tabela(this, 0x10, 0x98af54, 0x159, 0x9cf9a4, 0, "cqm_7 nordeste cheio")',
     'cqm7_tabela(this, 0x10, 0x98af54, 0x159, 0x9cf9a4, 1, "cqm_7 nordeste cheio")',
     'cqm_7 nordeste cheio'),
    ('cqm7_paranaense_girado',
     'cqm7_tabela(this, 0xc, 0x98af98, 0x169, 0x9cf9d0, 0, "cqm_7 paranaense cheio")',
     'cqm7_tabela(this, 0xc, 0x98af98, 0x169, 0x9cf9d0, 1, "cqm_7 paranaense cheio")',
     'cqm_7 paranaense cheio'),
    ('cqm7_pernambucano_girado',
     'cqm7_tabela(this, 0xa, 0x98afd8, 0x167, 0x9cf9bc, 0, "cqm_7 pernambucano cheio")',
     'cqm7_tabela(this, 0xa, 0x98afd8, 0x167, 0x9cf9bc, 1, "cqm_7 pernambucano cheio")',
     'cqm_7 pernambucano cheio'),
]
