# Mutacoes (sx19b_5), item de TELAS: a carreira automatica nao passa; servem para provar o comparador numa sessao do
# Lucas jogando comparando (a ficha de um jogador suspenso):
# - os jogos que faltam contados sem os ja cumpridos (o comparador acusa o texto num suspenso por mais de um jogo que
#   ja cumpriu algum).
MUTACOES = [
    ('sx19b_5_sem_cumpridos',
     'sprintf(text, BAN_TEXT_MATCHES, 100u32, 100u32, matches, s, r); // sx19b_5: the matches left',
     'sprintf(text, BAN_TEXT_MATCHES, 100u32, 100u32, read_i8(ban) as i32, s, r); // mutation: the served ones not taken off'),
]
