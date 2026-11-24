# Mutacoes (sx04_2). Os vencedores ganham reputacao (YEAR_AWARD/TEAM_AWARD::record_winners -> 0x412f00), lida pela
# carreira depois de abril (jogador e equipe) e no fim de maio (tecnico); o comparador acusa pelo registro planejado
# contra o gravado no historico:
# - o jogador do ano exige o dobro dos jogos como titular (outros vencedores);
# - o tecnico do ano na ordem inversa (o primeiro e o terceiro trocam: a reputacao do 1o e do 3o muda);
# - a equipe da temporada exige o dobro dos jogos como titular (outra equipe).
MUTACOES = [
    ('sx04_2_jogador_dobro_dos_jogos',
     'if starts < least { // sx04_2: enough starts',
     'if starts < least * 2 { // mutation: twice the starts',
     'sx04_2 player of the year candidate'),
    ('sx04_2_tecnico_ordem_inversa',
     'let club = Club(read_usize(clubs + k * 4));',
     'let club = Club(read_usize(clubs + (2 - k) * 4)); // mutation: reversed order',
     'sx04_2 manager of the year'),
    ('sx04_2_equipe_dobro_dos_jogos',
     'if starts <= least { // sx04_2: more starts than the share',
     'if starts <= least * 2 { // mutation: twice the starts',
     'sx04_2 team of the season candidate'),
]
