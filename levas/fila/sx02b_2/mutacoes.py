# Mutacoes (sx02b_2).
# - a clear_old_past_transfers roda em 1/1/2002, quando ainda nao ha transferencia de mais de 52 semanas (a carreira
#   comeca em julho de 2001): o corte invertido manda tudo para os arquivos; o comparador acusa o plano (que usa o mesmo
#   count_past_transfers_before) contra a original, que nao move nada. O ramo dos arquivos so roda de verdade em 2003;
# - a cancel_club_offers roda 1 vez (um tecnico que sai): as ofertas do proprio clube, ou as pelo elenco, ficam abertas.
#   So a carreira acusa, e so se o clube tinha ofertas abertas naquele dia.
MUTACOES = [
    ('sx02b_2_corte_invertido',
     '.take_while(|&i| transfer_manager_date_before(',
     '.take_while(|&i| !transfer_manager_date_before( // mutation: the cut-off inverted\n',
     'sx02b_2 clear old transfers'),
    ('sx02b_2_ofertas_do_clube',
     'cancel_offer_of_departed_manager(tm, o, club); // sx02b_2: the club\'s own offer',
     '{} // mutation: the club\'s own offers stay open',
     "sx02b_2 the club's own offer"),
    ('sx02b_2_ofertas_do_elenco',
     'cancel_offer_of_departed_manager(tm, o, club); // sx02b_2: an offer for a squad player',
     '{} // mutation: the squad\'s offers stay open',
     'sx02b_2 an offer for a squad player'),
]
