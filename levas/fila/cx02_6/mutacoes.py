# Mutacoes (cx02_6), carreira de 8 ligas ate 2002-08-01. Nenhuma abre caixa.
# - a final das copas francesas com 7 reservas e nao 5: o banco da final muda (a carreira e o comparador);
# - o clube da Segunda contra um de divisao abaixo da National nao da o mando: o mando dos confrontos muda (quem joga em
#   casa) e o comparador acusa;
# - o premio da Copa da Liga francesa 600.000: o comparador acusa na virada.
MUTACOES = [
    ('cx02_6_final_7_reservas',
     '        fixture.set_squad_rules((fixture.squad_rules() & 0xfff5) | 5);',
     '        fixture.set_squad_rules((fixture.squad_rules() & 0xfff0) | 7); // mutation'),
    ('cx02_6_segunda_sem_mando',
     '        return if below(KEY_CLUB_COMP_FRENCH_NATIONAL) { b } else { 0 };',
     '        return 0; // mutation'),
    ('cx02_6_premio_copa_da_liga_francesa',
     '    set_up_prize_rounds(Cup(this), 500_000)',
     '    set_up_prize_rounds(Cup(this), 600_000) // mutation'),
]
