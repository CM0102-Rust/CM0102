# As notas do ano (YEAR_RATINGS, award/year_ratings.cpp) e a virada do mês das notas do mês (MONTH_RATINGS::reset,
# award/month_ratings.cpp); leitor mwg, rodada 2 do mundo inteiro, 12/11/2026. As notas do ano alimentam os prêmios
# de jogador do ano das nações (NationAwards::award_player_of_year_from_ratings, fim de temporada); record_match_ratings
# roda depois de cada partida oficial (210.202 chamadas até 2002-08-01 no mundo inteiro, cobertura_mundo.txt) e
# add_player_rating por jogador (1.029.102); reset na virada de cada mês das ligas com prêmio do mês (220). Rodam na
# carreira só com a Inglaterra (a Premier tem jogador do mês e do ano): LIGAS = 'ENG'. As irmãs do mês
# (0x767ee0/0x767fa0) já são Rust (award/month_ratings.rs): mesmo molde.
# Convenções conferidas na desmontagem (0x9315d0..0x931686, 0x931690..0x931a92, 0x767f60..0x767f97):
# - record_match_ratings: thiscall(this, partida, registro da partida, filtro), ret 0xc; o filtro é cdecl(pessoa,
#   partida) -> eax (add esp, 8); add_player_rating: thiscall(this, linha, partida, clube), ret 0xc; reset:
#   thiscall(this, mês (char), ano (short)), ret 8;
# - 0x944e46 malloc / 0x945667 realloc (cdecl), 0x9452ca free; a caixa de erro (linha 0x113 de year_ratings.cpp,
#   0xad6a38) com o 0 em 0xb67a34 (show_error_at);
# - divisões por constantes mágicas: /500 (0x10624dd3 sar 5), /20 (0x66666667 sar 3), /40 (sar 4), /80 (sar 5),
#   /60 (0x88888889 add sar 5), /1700 (0x134679ad sar 7), todas truncando para zero como em Rust.
# Defeito do original: sem a competição na partida (+0x14) mas com o objeto da competição pelo id (+0), o peso é um
# byte da pilha que a rotina nunca grava ([esp+0x13]); a cópia usa 0 (não deve acontecer: as partidas oficiais têm
# a competição; o comparador acusaria).
# Comparadores: lado a lado (cabeçalho e entradas, com folga para 22 novas no record e 1 no add) quando a tabela não
# cresce; senão só a original. reset: só a original (libera a tabela) e confere a tabela vazia, o mês e o ano.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('year_record_match_ratings', 0x9315d0, [0x51, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('year_add_player_rating', 0x931690, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('month_ratings_reset', 0x767f60, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('as notas do ano e a virada do mês das notas do mês',
        "the year ratings and the month ratings' new month",
        [('year_record_match_ratings', 'YEAR_RATINGS::record_match_ratings', '0x9315d0', 'award/year_ratings.cpp', 'thiscall, ret 0xc',
          'leva os onze titulares dos dois lados de uma partida às notas do ano: amistoso (categoria 1) não; em cada lado as 11 primeiras das 20 linhas, sem a vazia (id −1 ou posição 0xfb) e sem o jogador virtual (id na contagem de staff ou acima); vai para add_player_rating quem o filtro (pessoa, partida) aceita',
          'takes the starting eleven of both sides of a match to the year ratings: not a friendly (category 1); on each side the first 11 of the 20 rows, without an empty one (id −1 or position 0xfb) or a virtual player (id at or above the staff count); those the filter (person, fixture) accepts go to add_player_rating'),
         ('year_add_player_rating', 'YEAR_RATINGS::add_player_rating', '0x931690', 'award/year_ratings.cpp', 'thiscall, ret 0xc',
          'acha ou acrescenta a entrada de 21 bytes do jogador (a tabela cresce de 400; sem memória a caixa de erro, linha 0x113) com a nação dele (−1 sem) e soma: três somas ponderadas da nota da partida, mais um jogo, gols, assistências e amarelos + 2 × vermelhos; o peso da competição = peso dela (+0x69) × (reputação do clube / 500) / 20 + (reputação − 5000) / 1700, entre 1 e 20; com o objeto da competição a nota × o peso dá /20, /20, /40 no tipo 2, /20 três vezes no tipo 4 e /20, /60, /80 nos outros; sem ele, a nota × (reputação / 500 × 16, 8 ou 4 pela reputação da nação acima de 7000, 4000 ou não, / 20) dá /20, /60, /80',
          "finds or appends the player's 21-byte entry (the table grows by 400; without memory the error box, line 0x113) with his nation (−1 none) and adds: three weighted sums of the match grade, one more match, goals, assists and yellows + 2 × reds; the competition weight = its weight (+0x69) × (club reputation / 500) / 20 + (reputation − 5000) / 1700, between 1 and 20; with the competition object the grade × the weight gives /20, /20, /40 for type 2, /20 three times for type 4 and /20, /60, /80 otherwise; without it, the grade × (reputation / 500 × 16, 8 or 4 by the nation's reputation above 7000, 4000 or not, / 20) gives /20, /60, /80"),
         ('month_ratings_reset', 'MONTH_RATINGS::reset', '0x767f60', 'award/month_ratings.cpp', 'thiscall, ret 8',
          'esvazia as notas do mês para um mês novo: libera a tabela, zera a contagem e a capacidade e grava o mês e o ano novos',
          "empties the month ratings for a new month: frees the table, zeroes the count and the capacity and stores the new month and year")])
INTERESSA = ['Tweaks']
