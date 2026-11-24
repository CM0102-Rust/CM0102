# Mutacoes (sbE_12), com a Espanha e a base NORMAL ate 2002-08-01 (a virada espanhola em junho/julho de 2002):
# - o time B da Segunda marcado 3 de cara (o laco de salvamento o tira do rebaixamento: outra Segunda em 2002/03, a
#   carreira; e o comparador nao ve 0xfb sobrando, entao so a carreira);
# - a vaga do time B barrado nao passa adiante (um acesso a menos na Segunda: a carreira).
MUTACOES = [
    ('sbE_12_rebaixado_de_cara',
     'write_u8(found + league_table_row::STATUS, 0xfb); // sbE_12: down with the senior club',
     'write_u8(found + league_table_row::STATUS, 3); // mutation: marked down at once',
     'sbE_12 b team down with its senior club'),
    ('sbE_12_vaga_nao_passa',
     'write_u8(row + league_table_row::STATUS, 0); // sbE_12: the place handed over',
     'write_u8(row + league_table_row::STATUS, 0xff); // mutation: the place is lost',
     'sbE_12 b team down with its senior club'),
]
