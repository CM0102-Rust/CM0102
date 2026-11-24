# Os premios do ano de uma nacao pela temporada: o jogador do ano pelas estatisticas, o tecnico do ano pela tabela e a
# equipe da temporada (nation_awards.cpp; leitor sx04, rodada 5, 23/11/2026). Rodam na carreira ENG pela diaria da
# Inglaterra (0x579610, sx03_2): o jogador do ano (2 chamadas) e as equipes da temporada (4 ou 5) num dia de abril, o
# tecnico do ano das 4 ou 5 divisoes num dia de maio (os dias pela 0x549f70 da diaria inglesa; antes de 2002-06-01).
# Convencoes conferidas na desmontagem:
# - 0x768cf0: thiscall(id da competicao, hoje, premio, idade, posicoes, estatistica, nacao, nacao do clube, nacao
#   excluida, nacao dos dois), ret 0x28; ecx sem uso; ponto de chamada 0x579672 (10 push); sem SEH.
#   Comp_Stats::get_sorted_player_table 0x4a4dc0 thiscall(estatistica, &n, 100, -1); YEAR_AWARD::record_winners
#   0x9302f0 thiscall(&registro, 1), ret 8.
# - 0x768940: thiscall(hoje, premio, id da competicao), ret 0xc; ecx sem uso; ponto 0x579751 (3 push); a virtual
#   +0xa0 da liga thiscall(&clubes, &n); Staff_Records::get_manager_of_year_candidates 0x86fab0 thiscall(clubes, n)
#   -> al (ja Rust).
# - 0x7698f0: thiscall(id da competicao, premio), ret 8; ponto 0x5796ab (2 push); TEAM_AWARD::clear_selection 0x8afd90
#   thiscall; add_candidate 0x8afa40 thiscall(pessoa, clube, nota f32, 0), ret 0x10; set_season_dates 0x8aff50
#   thiscall(ano), ret 4; record_winners 0x8affb0 thiscall.
# Prologos: 0x768940 com 10 bytes (as duas primeiras instrucoes, a primeira so tem 4); sem chamada relativa; iguais no
# cm0102-1080.exe. Os x87 (0,66 e 0,1 em f32, o corte por _ftol do produto, a nota em f32) conferidos: a sequencia das
# quotas em f64 arredondado a f32 da o mesmo que o x87.
# Defeito do original (0x768940): o dword +0x10 de cada entrada do registro (a "nota") nunca e escrito: o historico do
# premio do tecnico guarda 12 bytes do lixo da pilha; a copia escreve 0 e o comparador ignora esses bytes.
SAVE = False
ROTINAS = [('nation_awards_award_player_of_year', 0x768cf0, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('nation_awards_award_manager_of_year', 0x768940, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x0d, 0xfc, 0xad, 0xad, 0x00]),
           ('nation_awards_award_team_of_season', 0x7698f0, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00])]
DOCS = ('os prêmios do ano de uma nação pela temporada: o jogador do ano, o técnico do ano e a equipe da temporada',
        "a nation's yearly awards from the season: the player of the year, the manager of the year and the team of the season",
        [('nation_awards_award_player_of_year', 'nation_awards_award_player_of_year', '0x768cf0', 'nation_awards.cpp', 'thiscall, ret 0x28',
          'regra: os três primeiros da tabela de jogadores da competição pela estatística do prêmio (até 100) que são jogadores, jogam numa das posições pedidas, começaram como titulares ao menos uma fração dos jogos da liga (0,66; 0,1 nas estatísticas 2, 4 e 5; baixa 0,1 por passada enquanto passar de 0) e passam nos filtros de nação (a do jogador, a do clube, a excluída, as duas). Com idade-limite, os mais velhos são contados e os mais novos só entram depois de 3 contados. Com os três, o registro (ano, nomes, id, a estatística no prêmio 2, o clube) vai para o prêmio do ano (histórico e reputação)',
          "rule: the first three of the competition's player table for the award's statistic (up to 100) who are players, play one of the asked positions, started at least a share of the league's games (0.66; 0.1 for the statistics 2, 4 and 5; down by 0.1 a pass while above 0) and pass the nation filters (the player's, the club's, the excluded one, both). With an age limit, the older are counted and the younger only qualify after 3 were counted. With all three, the record (year, names, id, the statistic for award 2, the club) goes to the year award (history and reputation)"),
         ('nation_awards_award_manager_of_year', 'nation_awards_award_manager_of_year', '0x768940', 'nation_awards.cpp', 'thiscall, ret 0xc',
          'regra: os clubes da liga ordenados pelos pontos do técnico; com exatamente três técnicos qualificados, eles vão para o prêmio do ano (histórico e reputação). Defeito: a nota de cada entrada fica com o lixo da pilha',
          "rule: the league's clubs sorted by their manager's points; with exactly three qualified managers, they go to the year award (history and reputation). Defect: each entry's score keeps the stack garbage"),
         ('nation_awards_award_team_of_season', 'NationAwards::award_team_of_season', '0x7698f0', 'nation_awards.cpp', 'thiscall, ret 8',
          'regra: limpa a seleção (reservas: as da competição de clubes do prêmio, 3 sem ela) e oferece ao prêmio, pela tabela de notas da liga, cada jogador que foi titular em mais jogos que a fração dos jogos da liga (0,66, baixando 0,1 enquanto passar de 0), com a nota média, até o prêmio dizer que a equipe está completa; depois as datas da temporada e o registro (histórico e reputação)',
          "rule: clears the selection (substitutes: those of the award's club competition, 3 without one) and offers the award, from the league's rating table, each player who started more games than the share of the league's games (0.66, down by 0.1 while above 0), with his average rating, until the award reports the team full; then the season dates and the record (history and reputation)")])
CAMPOS = [('registro dos vencedores do prêmio do ano (0x4a bytes): +0 ano, 3 entradas de 0x18 (+0 +4 +8 ids dos nomes, +0xc pessoa, +0x10 nota, +0x14 clube); valores; grava (nation_awards_award_player_of_year, nation_awards_award_manager_of_year)',
           'year award winners record (0x4a bytes): +0 year, 3 entries of 0x18 (+0 +4 +8 name ids, +0xc person, +0x10 score, +0x14 club); values; writes (nation_awards_award_player_of_year, nation_awards_award_manager_of_year)'),
          ('prêmio de equipe +0x253 reservas; +0x254/+0x258 posições no histórico; prêmio do ano +0x23/+0x27 posições no histórico; valores; grava/lê (nation_awards_award_team_of_season e comparadores)',
           'team award +0x253 substitutes; +0x254/+0x258 history offsets; year award +0x23/+0x27 history offsets; values; writes/reads (nation_awards_award_team_of_season and comparers)'),
          ('linha da tabela de jogadores da competição (0x2d bytes) +0xc jogos, +0xd como reserva, +0xe estatística, +0x1a soma das notas (i16); valores; lê',
           "competition player table row (0x2d bytes) +0xc appearances, +0xd as substitute, +0xe statistic, +0x1a rating sum (i16); values; reads")]
INTERESSA = []
