# Mutacoes (cql_9), na tabela dos estaduais (rodam na criacao da carreira de 8 ligas e na virada de janeiro de 2002,
# antes dos estaduais de 2002): em cada um, as linhas giradas uma casa (o k-esimo clube na linha k+1, o ultimo na 0):
# a grade de jogos sai da ordem das linhas e os confrontos de cada rodada mudam. O contador fica depois de a tabela
# encher.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS (Goiano e Mineiro em 2001-039, Norte em 2001-014).
MUTACOES = [
    ('cql9_goiano_girado',
     'cql9_tabela(this, 0xe, 0x98ae90, 0x160, 0x9cf9b4, 0, "cql_9 goiano cheio")',
     'cql9_tabela(this, 0xe, 0x98ae90, 0x160, 0x9cf9b4, 1, "cql_9 goiano cheio")',
     'cql_9 goiano cheio'),
    ('cql9_mineiro_girado',
     'cql9_tabela(this, 0xc, 0x98aed0, 0x169, 0x9cf9d4, 0, "cql_9 mineiro cheio")',
     'cql9_tabela(this, 0xc, 0x98aed0, 0x169, 0x9cf9d4, 1, "cql_9 mineiro cheio")',
     'cql_9 mineiro cheio'),
    ('cql9_norte_girado',
     'cql9_tabela(this, 0xa, 0x98af14, 0x167, 0x9cf994, 0, "cql_9 norte cheio")',
     'cql9_tabela(this, 0xa, 0x98af14, 0x167, 0x9cf994, 1, "cql_9 norte cheio")',
     'cql_9 norte cheio'),
]
