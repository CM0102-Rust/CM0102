# Mutacoes (r7_06_2). A rotina nunca roda no jogo como ele e (a campanha de 2002 e codigo morto): nenhuma carreira a
# acusa. Acusadas na emulacao (rascunho r7_06\check_r7_06.py), ver PRONTO.txt:
# - o grupo da propria secao com uma selecao a menos na tabela;
# - o placar de Suica x Russia (2/9/2000) 1 x 1 em vez de 0 x 1;
# - o ultimo jogo (Inglaterra x Grecia, 6/10/2001) na fase 6 em vez da 7.
MUTACOES = [
    ('r7_06_2_grupo_menor',
     'league.set_club_count(size as i16); // r7_06_2: the own group\'s size',
     'league.set_club_count(size as i16 - 1); // mutation: one nation less',
     'r7_06_2 european 2002 campaign'),
    ('r7_06_2_suica_russia',
     '(-1, 2, 8, 2000, KEY_NATION_SWITZERLAND, 0, KEY_NATION_RUSSIA, 1), // r7_06_2: Switzerland-Russia',
     '(-1, 2, 8, 2000, KEY_NATION_SWITZERLAND, 1, KEY_NATION_RUSSIA, 1), // mutation: Switzerland-Russia 1-1',
     'r7_06_2 european 2002 campaign'),
    ('r7_06_2_ultimo_jogo_fase_6',
     '(7, 6, 9, 2001, KEY_NATION_ENGLAND, -1, KEY_NATION_GREECE, -1), // r7_06_2: the last match',
     '(6, 6, 9, 2001, KEY_NATION_ENGLAND, -1, KEY_NATION_GREECE, -1), // mutation: the last match in stage 6',
     'r7_06_2 european 2002 campaign'),
]
