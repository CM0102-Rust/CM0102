# Mutacoes (cql_8), na tabela dos estaduais (rodam na criacao da carreira de 8 ligas e na virada de janeiro de 2002,
# antes dos estaduais de 2002): em cada um, as linhas giradas uma casa (o k-esimo clube na linha k+1, o ultimo na 0):
# a grade de jogos sai da ordem das linhas e os confrontos de cada rodada mudam. O contador fica depois de a tabela
# encher.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS em 2001-039.
MUTACOES = [
    ('cql8_baiano_girado',
     'fill_state_league_table(this, 0xa, 0x98acac, 0x167, 0x9cf9c0, 0, "cql_8 baiano cheio")',
     'fill_state_league_table(this, 0xa, 0x98acac, 0x167, 0x9cf9c0, 1, "cql_8 baiano cheio")',
     'cql_8 baiano cheio'),
    ('cql8_centro_oeste_girado',
     'fill_state_league_table(this, 0xf, 0x98ae10, 0x168, 0x9cf998, 0, "cql_8 centro_oeste cheio")',
     'fill_state_league_table(this, 0xf, 0x98ae10, 0x168, 0x9cf998, 1, "cql_8 centro_oeste cheio")',
     'cql_8 centro_oeste cheio'),
    ('cql8_gaucho_girado',
     'fill_state_league_table(this, 0x11, 0x98ae50, 0x163, 0x9cf9c8, 0, "cql_8 gaucho cheio")',
     'fill_state_league_table(this, 0x11, 0x98ae50, 0x163, 0x9cf9c8, 1, "cql_8 gaucho cheio")',
     'cql_8 gaucho cheio'),
]
