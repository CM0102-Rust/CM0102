# Mutacoes (cx16_8), ENG ate 2005-12-31; acusadas pelos comparadores (lado a lado no retorno):
# - a situacao europeia em aberto ate 13 (a selecao com codigo 12 continua em aberto);
# - a mudanca da Oceania sem o -1 (quem passa a 1 nao muda).
MUTACOES = [
    ('cx16_8_europa_aberta_ate_13',
     'if qualifying_stat_byte(this, club, 0x12) < 0xc { -2 } else { -3 } // cx16_8: open below 12',
     'if qualifying_stat_byte(this, club, 0x12) < 0xd { -2 } else { -3 } // mutation: open below 13',
     'cx16_8 european qualification status'),
    ('cx16_8_oceania_sem_queda',
     '-((before != 1) as i8) // cx16_8: became 1',
     '0 // mutation: no change',
     'cx16_8 oceania status change'),
]
