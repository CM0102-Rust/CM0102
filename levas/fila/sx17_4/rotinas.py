# A tarefa da lista do olheiro e o historico do jogador observado (player_search.cpp; leitor sx17, rodada 5,
# 24/11/2026): start_scout_assignment (0x7ae9c0), clear_scout_assignment (0x7aee80) e as somas do anel de 10 jogos
# (0x7bd8c0 assistencias, 0x7bd910 gols sofridos, 0x7bd960 nota media). Rotinas inteiras (0x7ae9c0..0x7aee73 ret 0x14;
# 0x7aee80..0x7aefc4 ret; 0x7bd8c0..0x7bd900, 0x7bd910..0x7bd950, 0x7bd960..0x7bd9c1, ret; o resto e enchimento).
# Quem chama: a tarefa, a sessao de edicao do olheiro (0x804973, 0x804a1a), o SCOUT_MANAGER (0x7f8ce3) e o dia de
# trabalho do olheiro ao fim da tarefa (0x7b59cc, 0x7b59e1, ja em Rust: so roda com olheiro com tarefa); as somas, as
# telas de busca (0x80e191..., 0x818f64...: as colunas e a ordem dos observados). A carreira automatica nao passa
# por nenhuma.
# TELAS: a prova e o Lucas jogando comparando: dar e tirar tarefas a um olheiro (nacao, regiao, competicao, base,
# proximo adversario), e a tela de busca com as colunas de jogos observados (assistencias, gols sofridos, nota media).
# Convencoes conferidas na desmontagem:
# - 0x7ae9c0 thiscall(busca, nacao, regiao, competicao, base, adversario) ret 0x14 -> 1/0; a regiao vai inteira
#   (dword) para o get_region_name e so o byte e testado (1..14, com sinal); 0x7aee80 thiscall ret -> 1/0;
# - as somas thiscall(vaga) -> ax (soma com sinal, movsx); a media thiscall(vaga) -> st0: (soma como i16) / contagem,
#   sem jogos o double 0.0 de 0x9689a0;
# - 0x53e990 database_staff_matches_names (pelo get_scout, ja em Rust); 0x6181d0 cdecl(nacao, buf, 0xc8, 1); 0x616f20
#   cdecl(regiao, buf, 0xc8, 1); 0x66f4e0 langlib_sprintf cdecl; 0x90cfc0 utils_random cdecl(3); 0x54c5b0 / 0x54c8e0
#   Date::add_days / sub_days thiscall(data, &saida, n) ret 8; 0x48d500 Comp::build_match_date_list thiscall(comp,
#   &n) ret 4 (vetor de 20 bytes liberado pelo free); 0x549ef0 Date::Date thiscall(&d, dia, ano) ret 8; 0x417c10
#   Date::operator=; 0x7b0050 build_scout_targets thiscall; 0x806450 SearchFilters::set_defaults thiscall.
TELAS = True
ROTINAS = [('player_search_start_scout_assignment', 0x7ae9c0, [0x81, 0xec, 0xdc, 0x02, 0x00, 0x00]),
           ('player_search_clear_scout_assignment', 0x7aee80, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('player_search_slot_sum_assists', 0x7bd8c0, [0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00]),
           ('player_search_slot_sum_goals_conceded', 0x7bd910, [0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00]),
           ('player_search_slot_average_rating', 0x7bd960, [0x51, 0x8a, 0x91, 0xa0, 0x01, 0x00, 0x00])]
DOCS = ('a tarefa da lista do olheiro e o historico do jogador observado',
        'the scout list assignment and the watched player match history',
        [('player_search_start_scout_assignment', 'PLAYER_SEARCH::start_scout_assignment', '0x7ae9c0', 'player_search.cpp', 'thiscall, ret 0x14',
          'da a tarefa a uma lista de olheiro (tipo 4, com o olheiro valido; senao 0, com a caixa 0x18e se o tipo for outro): o primeiro argumento valido entre nacao, regiao (1..14), competicao, base e proximo adversario (nenhum: a caixa 0x21d e 0); escreve o titulo ("<adjetivo> search", "<regiao> search", "<competicao> search", "Youth search", "Next Opposition"), os campos de nacao, regiao e competicao; a tarefa acaba em 1..3 dias (sorteio), na competicao dois dias antes do primeiro jogo se for depois, no proximo adversario amanha (com um relatorio ja contado); monta os alvos (build_scout_targets) e volta os filtros ao padrao; devolve 1',
          'gives a scout list its assignment (type 4 with a valid scout; otherwise 0, with the 0x18e box when the type differs): the first valid argument among nation, region (1..14), competition, youth and next opponent (none: the 0x21d box and 0); writes the title ("<adjective> search", "<region> search", "<competition> search", "Youth search", "Next Opposition"), the nation, region and competition fields; the assignment ends in 1..3 days (random), for a competition two days before the first match when that is later, for the next opposition tomorrow (with one report already counted); builds the targets (build_scout_targets) and resets the filters; returns 1'),
         ('player_search_clear_scout_assignment', 'PLAYER_SEARCH::clear_scout_assignment', '0x7aee80', 'player_search.cpp', 'thiscall',
          'encerra a tarefa de uma lista de olheiro (tipo 4 com olheiro valido; senao a caixa 0x237 e 0): clube atribuido −1 e o registro do clube observado apagado; nacao, regiao, competicao e clube-alvo −1, base e adversario 0; devolve 1',
          'ends a scout list assignment (type 4 with a valid scout; otherwise the 0x237 box and 0): assigned club −1 and the scouted club record deleted; nation, region, competition and target club −1, youth and opponent 0; returns 1'),
         ('player_search_slot_sum_assists', 'player_search_slot_sum_assists', '0x7bd8c0', 'player_search.cpp', 'thiscall',
          'soma as assistencias dos jogos guardados no anel de 10 jogos do jogador observado',
          'adds the assists of the matches stored in the watched player 10-match ring'),
         ('player_search_slot_sum_goals_conceded', 'player_search_slot_sum_goals_conceded', '0x7bd910', 'player_search.cpp', 'thiscall',
          'soma os gols sofridos nos jogos guardados no anel do jogador observado',
          'adds the goals conceded in the matches stored in the watched player ring'),
         ('player_search_slot_average_rating', 'player_search_slot_average_rating', '0x7bd960', 'player_search.cpp', 'thiscall',
          'a nota media dos jogos guardados no anel (soma como inteiro de 16 bits sobre a contagem, no x87); sem jogos, 0.0',
          'the average grade of the matches stored in the ring (the sum as a 16-bit integer over the count, in the x87); without matches, 0.0')])
