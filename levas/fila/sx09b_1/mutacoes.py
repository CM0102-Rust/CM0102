# Mutacoes (sx09b_1). A noticia nao volta para a simulacao: as tres sao acusadas pelo comparador (as noticias que
# chegam a caixa do tecnico humano), quando a suspensao e de jogador do clube humano (suspensoes por jogos, ao clube).
# - os jogos da suspensao com um a mais na vaga 2;
# - o clube nao recebe a noticia;
# - o inicio "ainda por vir" com <= no dia: a suspensao que comeca hoje (quase todas) ganha o inicio nas vagas 4 e 5.
MUTACOES = [
    ('sx09b_jogos_mais_um',
     'ban_news_slot(n, &mut k, ban.matches() as i8 as i32 as u32); // sx09b_1: the matches',
     'ban_news_slot(n, &mut k, (ban.matches() as i8 as i32 + 1) as u32); // mutation',
     'sx09b_1 ban news to the club'),
    ('sx09b_clube_sem_noticia',
     'ban_news_send(n, club); // sx09b_1: the club gets it',
     '// mutation: the club does not get it',
     'sx09b_1 ban news to the club'),
    ('sx09b_inicio_hoje',
     'if today_year < start_year || (today_year == start_year && today_day < start_day) { // sx09b_1: the start still to come',
     'if today_year < start_year || (today_year == start_year && today_day <= start_day) { // mutation',
     'sx09b_1 ban news to the club'),
]
