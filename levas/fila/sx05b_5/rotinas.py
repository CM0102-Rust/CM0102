# Os construtores do premio de equipe, do premio do ano e da tabela das notas do ano (award/team_award.cpp,
# award/year_award.cpp, award/year_ratings.cpp; leitor sx05b, rodada 5, 23/11/2026), mais o que os outros itens do
# sx05b usam: award_base, team_award_obj, year_award_obj, os construtores comuns dos premios das nacoes (AwardSpec,
# award_new, nation_awards_fields, nation_awards_build, NationAwardsClass, nation_awards_construct, os comparadores
# deles, award_player_of_year_from_ratings, award_top_three_from_ratings, nation_awards_base_daily_update,
# is_season_end_day, nation_awards_team_of_the_week_of) e das noticias (NewsPost, NewsSink, news_post, rehearse_news).
# Integrar este antes dos outros sx05b e depois do sx05_6 (TeamAward, team_award_clear_selection,
# team_award_init_slot_positions) e do sx03_2 (nation_award, build_team_of_the_week).
# Rodam na criacao do jogo: cada competicao constroi os seus premios (0x8af520 e 0x92fe80 sao chamados por mais de
# cem construtores de competicao) e o AWARD_MANAGER as notas do ano (0x931360, 0x4133ae/0x4133e6 e 0x414111/0x414152).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - 0x8af520 thiscall(nacao, indice, competicao de pessoal, rep. nacional, rep. mundial, competicao de clubes, anos da
#   temporada (byte), modo da formacao (byte), arquivo, para jogadores) ret 0x28; AWARD::AWARD 0x412e40 recebe
#   (a1..a6, a9, a10), ret 0x20; tres Date::Date 0x54a110; o laco das 20 vagas igual ao TEAM_AWARD::clear_selection
#   (sx05_6); 0x8afdc0 as mascaras; com arquivo, 0x8af660 (retorno nao lido);
# - 0x92fe80 thiscall(nacao, indice, comp. pessoal, rep. nac., rep. mund., comp. clubes, tipo (byte), escopo, opcao
#   (byte), arquivo, para jogadores) ret 0x2c; AWARD::AWARD(a1..a6, a10, a11); com arquivo, 0x92fff0(escopo, arquivo)
#   ret 8; sem arquivo ou com a carga falhando, com escopo: operator new 0x944cf1(0x40) e AWARD_SHORTLIST 0x417440
#   (escopo, 0) ret 8;
# - 0x931360 thiscall(ano (word, empilhado como dword), arquivo) ret 8; com arquivo, 0x931390 ret 4.
# Comparadores: os dois premios so a original (alocam; o premio do ano carrega a lista curta) e o objeto conferido
# contra o plano quando nao ha arquivo; a tabela das notas lado a lado sem arquivo.
SAVE = False
ROTINAS = [('team_award_construct', 0x8af520, [0x6a, 0xff, 0x68, 0xcc, 0x37, 0x96, 0x00]),
           ('year_award_construct', 0x92fe80, [0x6a, 0xff, 0x68, 0x83, 0x60, 0x96, 0x00]),
           ('year_ratings_construct', 0x931360, [0x66, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('os construtores dos prêmios de equipe e do ano e da tabela das notas do ano',
        'the team award, year award and year ratings constructors',
        [('team_award_construct', 'TEAM_AWARD::TEAM_AWARD', '0x8af520', 'award/team_award.cpp', 'thiscall, ret 0x28',
          'monta um prêmio de equipe: a base do prêmio (nação, índice, competição de pessoal, reputações nacional e mundial, competição de clubes, o arquivo e se é de jogadores), três datas vazias (a da atualização e as do período), os anos da temporada, o modo da formação, tipo 0, três reservas, sem histórico, as 20 vagas vazias e as máscaras de posição; com arquivo, carrega o salvo',
          'builds a team award: the award base (nation, index, staff competition, national and world reputations, club competition, the file and whether it is for players), three empty dates (the update date and the period\'s), the season span in years, the formation mode, type 0, three substitutes, no history, the 20 slots empty and the position masks; with a file, loads the saved one'),
         ('year_award_construct', 'YEAR_AWARD::YEAR_AWARD', '0x92fe80', 'award/year_award.cpp', 'thiscall, ret 0x2c',
          'monta um prêmio do ano: a base do prêmio, tipo 1, o tipo do registro e a opção, sem lista curta e sem histórico; com arquivo, carrega o salvo; sem ele ou com a carga falhando, com escopo, cria a lista curta do escopo (0x40 bytes; sem memória fica 0)',
          'builds a year award: the award base, type 1, the record kind and the option byte, no shortlist and no history; with a file, loads the saved one; without one or when the load fails, a scope gets its new shortlist (0x40 bytes; 0 when out of memory)'),
         ('year_ratings_construct', 'YEAR_RATINGS::YEAR_RATINGS', '0x931360', 'award/year_ratings.cpp', 'thiscall, ret 8',
          'monta a tabela vazia das notas de um ano (sem entradas, conta e capacidade 0, o ano) e, com arquivo, a carrega',
          'builds the empty year ratings table of a year (no entries, count and capacity 0, the year) and, with a file, loads it')])
CAMPOS = []
INTERESSA = []
