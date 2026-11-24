# Mutacoes (sx18_28), pela carreira ENG ate 2002-06-01 (jogos reservas marcados e cancelados todo dia; os reservas mudam
# a forma e a lesao dos jogadores que jogam):
# - o clube humano sem a exigencia dos 11 disponiveis;
# - jogo reserva tambem em 26 de dezembro (a pergunta com a data roda na marcacao).
MUTACOES = [
    ('sx18_28_humano_sem_onze',
     'if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && (s.available < 0xb || s.available_listed < 1) { // sx18_28: a human club\'s reserve side',
     'if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && s.available_listed < 1 { // mutation',
     'sx18_28 reserve sides counted'),
    ('sx18_28_jogo_no_natal',
     'if boxing_day[1] == read_u16(p + 2) && boxing_day[0] == read_u16(p) { // sx18_28: no reserve match on 26 December',
     'if false && boxing_day[1] == read_u16(p + 2) && boxing_day[0] == read_u16(p) { // mutation',
     'sx18_28 reserve sides counted'),
]
