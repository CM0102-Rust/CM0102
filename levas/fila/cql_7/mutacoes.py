# Mutacoes (cql_7), na tabela das divisoes nacionais do Brasil (rodam na criacao da carreira de 8 ligas e na virada):
# - Serie A: as linhas giradas uma casa (o k-esimo clube na linha k+1): a grade de jogos sai da ordem das linhas e os
#   confrontos de cada rodada mudam;
# - Serie B: idem;
# - Serie C: o sorteio com um clube a menos (3 trocas a menos): o gerador anda outra conta e a lista sai outra.
# Os contadores ficam depois de a tabela encher (A e B) e logo antes do sorteio (C).
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01, base com os 9 itens): as tres ACUSADAS (A em 2001-213, B em 2001-214, C em 2001-001).
MUTACOES = [
    ('cql7_serie_a_girada',
     'create_division_table(this, 0x1c, 0x98aa30, 0x1a1, 0x57, 0x9cf9ec, 0, "cql_7 serie a cheia")',
     'create_division_table(this, 0x1c, 0x98aa30, 0x1a1, 0x57, 0x9cf9ec, 1, "cql_7 serie a cheia")',
     'cql_7 serie a cheia'),
    ('cql7_serie_b_girada',
     'create_division_table(this, 0x1c, 0x98ab64, 0x1aa, 0x57, 0x9cf9e0, 0, "cql_7 serie b cheia")',
     'create_division_table(this, 0x1c, 0x98ab64, 0x1aa, 0x57, 0x9cf9e0, 1, "cql_7 serie b cheia")',
     'cql_7 serie b cheia'),
    ('cql7_sorteio_da_serie_c_menor',
     'embaralhar(cql7_d(this + 0xee), k as i8 as i32); // cql7: o sorteio dos clubes da Série C',
     'embaralhar(cql7_d(this + 0xee), k as i8 as i32 - 1); // mutacao: o sorteio dos clubes da Série C',
     'cql_7 serie c sorteio'),
]
