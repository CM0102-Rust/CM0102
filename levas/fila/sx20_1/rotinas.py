# Os jogos simulados do dia dos clubes fora das ligas jogaveis (simulated_stats.cpp; leitor sx20, rodada 5,
# 24/11/2026): SimulatedStats::play_daily_matches (0x84be20). Rotina inteira (0x84be20..0x84c4fb ret; o resto e
# enchimento e o thunk 0x84c500 do atexit, que fica de fora). Quem chama: o laco do dia (todo dia); o objeto e o
# G_SIMULATED_STATS que a passada das temporadas (0x5d5ae0, nma_2) refaz.
# Convencoes conferidas na desmontagem:
# - a rotina: thiscall(simulated_stats), ret 0; quadro SEH so pelo estado das datas locais (sem destrutor);
# - na primeira vez (bit 0 de 0xdd7160): 0x9463af eh vector constructor iterator stdcall(0xdd7287, 0x625, 2,
#   0x453c30, 0x453c10) e 0x94627a atexit(0x84c500) cdecl;
# - 0x90cfc0 utils_random cdecl(n); 0x54cfb0 Date::get_month thiscall(0xae2c90) -> al (chamado duas vezes quando > 4);
#   0x54c8e0 Date::sub_days / 0x54c5b0 Date::add_days thiscall(0xae2c90, saida, 2 / 3) ret 8 -> ponteiro da saida;
# - 0x5acbe0 FIXTURE_MANAGER::find_fixtures thiscall([0xae2a58], de.d0, de.d2, ate.d0, ate.d2, clube, 0, -1, -2, -1)
#   -> ax (os dois dwords de cada data sobrepostos: bytes 0..3 e 2..5, como empurra a original);
# - 0x62c400 Injury::get_player_injury_status thiscall([0xb63d28], pessoa) ret 4 -> al; 0x7757d0
#   NationalTeams::get_international_duty_club thiscall([0xb74318], pessoa, 0xae2c90, clube, 0, 0) ret 0x14;
# - o jogo: o fixture local (0x4f bytes em esp+0x40), 0x90d2f0 VIRTUAL_STAFF::fill_fixture_squads
#   thiscall([0xde2f70], fixture, -2, 0) ret 0xc; 0x944cf1 new(0xf63b); 0x6b7e50 CM3_MATCH_ENGINE thiscall; sem motor
#   a caixa (simulated_stats.cpp 0x15e: _splitpath, 0x90d130, "v%s %s.%s %d") e [0xb67a34] = 0; 0x6b7f20 init_match
#   thiscall(motor, fixture, 1, 0, 0) ret 0x10 -> al; 0x6b98c0 run_match_steps thiscall(motor, -1) ret 4; 0x729ff0
#   Match_Stats::fill_match_record thiscall([0xb6b7c0], motor, 0xdd7168, 0) ret 0xc; thiscall(dono, fixture,
#   0xdd7168) ret 8 de 0x7c0d70 ([0xb74350]), 0x4ca6d0 ([0xae1050]), 0x62c080 ([0xb63d28]), 0x4134c0 ([0xad9c60]) e
#   0x8c39f0 ([0xde1f54]); 0x6b97a0 ~CM3_MATCH_ENGINE thiscall; 0x944c94 delete; 0x90d6b0
#   VIRTUAL_STAFF::release_fixture thiscall([0xde2f70], fixture) ret 4.
# Defeitos do original: sem motor (caixa) ou com init_match 0 (vaza o motor) o mesmo par e tentado de novo sem fim; um
# clube com 0 livres pode ser o segundo do par (so -1, ocupado, para); no fixture ficam sem valor o slot de
# estatisticas (+8), datas, arbitro, rodada e regras (+0x24..+0x39), +0x3b, +0x3d..+0x3f, +0x42 e o byte +0x4e: a
# original passa o que a pilha tem (a copia, o que o jogo anterior da chamada deixou, senao 0).
ROTINAS = [('simulated_stats_play_daily_matches', 0x84be20, [0x6a, 0xff, 0x68, 0x24, 0x26, 0x96, 0x00])]
DOCS = ('os jogos simulados do dia',
        'the daily simulated matches',
        [('simulated_stats_play_daily_matches', 'SimulatedStats::play_daily_matches', '0x84be20', 'simulated_stats.cpp', 'thiscall',
          'joga os jogos simulados do dia dos clubes fora das ligas jogaveis: na primeira vez monta o registro de partida estatico (0xdd7168) e registra a destruicao no atexit; mistura os grupos (tantas vezes quantos os grupos, um grupo ao acaso e um vizinho, ate numero de grupos / 150 para cada lado, trocam um clube ao acaso); joga os grupos cujo indice bate com o dia do ano modulo 7, fora dos meses 5 e 6: os livres de cada clube (-1 com jogo de dois dias antes a tres depois ou sem nacao; senao os jogadores do elenco que nao estao lesionados, status 2, nem a servico da selecao hoje), ordena pelos livres (mais primeiro), embaralha entre os que tem livres (n² trocas, se mais de dois) e casa em ordem (1o com 2o, 3o com 4o...) enquanto o primeiro tem livres e o segundo nao esta ocupado; cada jogo e um fixture novo (competicao -2, categoria 2, marcas 1, 0 a 0) jogado ate o fim por um motor novo, resumido no registro estatico e passado as estatisticas, contratos, lesoes, premios e clausulas de jogos',
          "plays the day's simulated matches of the clubs outside the playable leagues: on the first call builds the static match record (0xdd7168) and registers its destruction with atexit; mixes the groups (as many times as there are groups, a random group and a neighbour, up to group count / 150 either side, swap a random club); plays the groups whose index matches the day of the year modulo 7, except in months 5 and 6: each club's free players (-1 with a fixture from two days ago to three ahead or no nation; otherwise the squad players not out injured, status 2, nor on international duty today), sorts by them (most first), shuffles those with free players (n² swaps, when more than two) and pairs in order (1st with 2nd, 3rd with 4th...) while the first has free players and the second is not busy; every match is a fresh fixture (competition -2, category 2, side marks 1, 0-0) played to the end by a new engine, summarised into the static record and passed to the statistics, contracts, injuries, awards and appearance clauses")])
