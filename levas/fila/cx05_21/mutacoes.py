# Mutacoes (cx05_21), carreira ENG ate 2002-06-01 (os torneios de pre-temporada de julho de 2001: 8 envios de
# convites na cobertura).
# - o torneio sai da lista de pendentes e nao volta: as respostas nunca sao processadas e o torneio nao acontece (os
#   amistosos de julho mudam);
# - a substituta do convidado que recusa guarda a pontuacao e nao a faixa: pode escapar (a substituicao so roda quando
#   um convidado recusa; 0 chamadas na cobertura ate 2002-06-01).
MUTACOES = [
    ('cx05_21_sem_pendente',
     '    exe::llist_add_tail(friendly.pending_tournament_list(), (&pointer) as *const usize as usize, 4);',
     '    // mutation: the tournament is not queued again'),
    ('cx05_21_guarda_a_pontuacao',
     "                        best_band = band as i8; // cx05_21: the club's band, not its score",
     "                        best_band = score; // mutation"),
]
