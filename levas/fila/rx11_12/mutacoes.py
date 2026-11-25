# Mutacao (rx11_12), pela carreira ENG ate 2002-06-01: a competicao reserva com o registro dos amistosos no lugar do
# seu (os jogos reservas, as estatisticas e as noticias deles passam a outra competicao) [inferencia: a carreira acusa
# pelos registros dos jogos reservas; se escapar, o construtor fica sem mutacao propria e a prova e o comparador, que
# confere o registro].
MUTACOES = [
    ('rx11_12_registro_dos_amistosos',
     'write_usize(this + league::COMPETITION, key_comp_record(KEY_CLUB_COMP_RESERVE)); // rx11_12: the reserve record',
     'write_usize(this + league::COMPETITION, key_comp_record(KEY_CLUB_COMP_FRIENDLY)); // mutation',
     'rx11_12 reserve competition built'),
]
