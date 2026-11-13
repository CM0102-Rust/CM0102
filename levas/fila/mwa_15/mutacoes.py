# Mutacoes (mwa_15), nas viradas das Premier da Irlanda do Norte e da Irlanda, carreira ENG IRL NIR ate 2002-08-01 (as
# viradas sao em maio/junho de 2002 [inferencia]: as divisoes dos clubes mudam antes da data final):
# - a First Division vira junto so na Irlanda do Norte (a mutacao tira a virada da First da Irlanda: a tabela e o ano
#   da First ficam os de 2001-02);
# - a cadeia de acesso e rebaixamento da Irlanda do Norte recebe 0 em vez de 1 [inferencia: muda quem sobe e quem cai];
# - o ano da Premier da Irlanda sobe 2 (se a carreira nao acusar, o comparador da virada acusa).
MUTACOES = [
    ('mwa15_sem_virada_da_first',
     'bra_nat_first_call_virtual_no_args(comp_by_global(first), comp_vtable::START_NEW_SEASON); // mwa15: the First Division turns over with the Premier',
     'if first != KEY_CLUB_COMP_IRISH_FIRST_DIVISION { bra_nat_first_call_virtual_no_args(comp_by_global(first), comp_vtable::START_NEW_SEASON); } // mutation',
     'mwa_15 irish premier turnover'),
    ('mwa15_cadeia_com_zero',
     'start_irish_premier_season(League(this), 1, 0x78ec60,',
     'start_irish_premier_season(League(this), 0, 0x78ec60,',
     'mwa_15 irish premier turnover'),
    ('mwa15_irlanda_ano_mais_dois',
     'start_irish_premier_season(League(this), 0, 0x63a740, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9ea080, [0x1f3, 0x1fa, 0x200, 0x207], 1)',
     'start_irish_premier_season(League(this), 0, 0x63a740, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9ea080, [0x1f3, 0x1fa, 0x200, 0x207], 2)',
     'mwa_15 irish premier turnover'),
]
