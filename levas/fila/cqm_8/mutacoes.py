# Mutacoes (cqm_8), na tabela dos estaduais (rodam na criacao da carreira de 8 ligas e na virada de janeiro de 2002,
# antes dos estaduais de 2002): em cada um, as linhas giradas uma casa (o k-esimo clube na linha k+1, o ultimo na 0):
# a grade de jogos sai da ordem das linhas e os confrontos de cada rodada mudam. O contador fica depois de a tabela
# encher.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS (2001-040, 2001-039, 2001-046).
MUTACOES = [
    ('cqm8_carioca_girado',
     'bra_reg_rio_create_state_league_table(this, 0xc, 0x98b014, 0x169, 0x9cf9dc, 0, "cqm_8 carioca cheio")',
     'bra_reg_rio_create_state_league_table(this, 0xc, 0x98b014, 0x169, 0x9cf9dc, 1, "cqm_8 carioca cheio")',
     'cqm_8 carioca cheio'),
    ('cqm8_catarinense_girado',
     'bra_reg_rio_create_state_league_table(this, 0xa, 0x98b050, 0x167, 0x9cf9b0, 0, "cqm_8 catarinense cheio")',
     'bra_reg_rio_create_state_league_table(this, 0xa, 0x98b050, 0x167, 0x9cf9b0, 1, "cqm_8 catarinense cheio")',
     'cqm_8 catarinense cheio'),
    ('cqm8_paulista_girado',
     'bra_reg_rio_create_state_league_table(this, 0x10, 0x98b090, 0x159, 0x9cf9e4, 0, "cqm_8 paulista cheio")',
     'bra_reg_rio_create_state_league_table(this, 0x10, 0x98b090, 0x159, 0x9cf9e4, 1, "cqm_8 paulista cheio")',
     'cqm_8 paulista cheio'),
]
