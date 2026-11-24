# Mutacoes (cx13_11), com a Russia ate 2002-08-01 (a temporada russa acaba em novembro de 2001):
# - o play-off do titulo sempre que ha empate deixado para desempate, mesmo sem os dois primeiros empatados (o titulo
#   russo vai a um jogo: outro campeao e outras vagas europeias; so roda se houver empate na fronteira de faixa);
# - a atualizacao sem esperar o empate deixado para desempate (todo dia, com a liga completa e sem fases, a temporada
#   e reaberta e as posicoes refeitas: o fim de temporada russo muda).
# A fase seguinte usa o advance_league_stage, ja provado; fica sem mutacao propria.
MUTACOES = [
    ('cx13_11_play_off_sem_empate',
     'if top_two_level(l) { // cx13_11: the title tie',
     'if true { // mutation: always the play-off',
     'cx13_11 russian season tie'),
    ('cx13_11_sem_esperar_o_desempate',
     'if l.complete() == 0 || l.stage_count() != 0 || flag == 0 || l.tiebreak_matches() == 0 {',
     'if l.complete() == 0 || l.stage_count() != 0 || flag == 0 {',
     'cx13_11 russian season tie'),
]
