# Mutacoes (cx03_8), carreira com as ligas da Escocia e de Gales ate 2002-08-01. Nenhuma abre caixa.
# - o premio da Challenge Cup 300.000: o comparador das rodadas acusa na criacao;
# - a Copa de Gales sem premio: o comparador das rodadas acusa na criacao;
# - a Premier escocesa com a classe da Terceira: a ordem do sorteio muda (o comparador dos inscritos acusa a lista).
MUTACOES = [
    ('cx03_8_premio_challenge_cup',
     '    set_up_prize_rounds(Cup(this), 200_000)',
     '    set_up_prize_rounds(Cup(this), 300_000) // mutation'),
    ('cx03_8_copa_de_gales_sem_premio',
     '    set_up_prize_rounds(Cup(this), 50_000)',
     '    set_up_prize_rounds(Cup(this), 0) // mutation'),
    ('cx03_8_premier_com_classe_da_terceira',
     'true, |c| c.last_position().wrapping_add(0x14));',
     'true, |c| c.last_position().wrapping_add(0x50)); // mutation'),
]
