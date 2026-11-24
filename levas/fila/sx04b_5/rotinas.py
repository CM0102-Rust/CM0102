# O jogador e a equipe de um torneio (nation_awards.cpp; leitor sx04b, rodada 5, 23/11/2026). Rodam pela diaria dos
# premios da Copa do Mundo (WorldCupAwards::daily_update 0x92fdf0, item do sx05b): num ano de Copa ((ano - 1998) % 4
# = 0), em 13 de julho (dia 13 do mes 6), o jogador do torneio (0x7690c0, estatistica 2, sem idade nem posicoes) e a
# equipe do torneio (0x769af0) da competicao [0x9cf79c]. Na carreira de teste: 13 de julho de 2002 (ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x7690c0: thiscall(indice da competicao, hoje, premio, idade (byte), posicoes (word), estatistica (byte)), ret 0x18;
#   ecx sem uso; key_nation_index_of_comp 0x669470 cdecl(registro da competicao) -> al (0 ou 1: internacional);
#   Comp_Stats::get_sorted_player_table 0x4a4dc0 thiscall(estatistica, &n, 100, -1); staff_get_natural_position_mask
#   0x5481a0 cdecl -> ax; database_get_national_team 0x53b3d0 cdecl(nacao) (sem teste do retorno); x87: fld f32 da
#   fracao, fmul f32 3.0 (0x969584), _ftol, cortado a 16 bits; fsub f64 0.1 (0x967880) guardado em f32; o teste
#   fracao > 0 contra f64 0.0 (0x9689a0) no comeco de cada passada; YEAR_AWARD::record_winners 0x9302f0 (&registro,
#   1) ret 8.
# - 0x769af0: thiscall(indice da competicao, premio), ret 8; o n da tabela gravado na vaga do primeiro argumento;
#   tabela 0x11 com (-1, -1); as reservas pelo byte +0x49 da competicao de clubes do premio (3 sem ela);
#   TEAM_AWARD::clear_selection 0x8afd90; add_candidate 0x8afa40 thiscall(pessoa, selecao ou clube, nota f32 (fild
#   soma / fidiv jogos), 0) ret 0x10; set_season_dates 0x8aff50 (ano de [0xae2c92]); record_winners 0x8affb0. A fracao
#   testada no fim de cada passada (a primeira sempre roda).
# Prologos sem chamada relativa; iguais no cm0102-1080.exe (0x7690c0 com 10 bytes, 0x769af0 com 7).
# Defeito do original (0x7690c0): fora da estatistica 2 o dword da nota de cada vencedor nunca e escrito (lixo da pilha
# no historico).
# Depende do sx04_2 (award_league, ROW_*, YEAR_RECORD_*, year_record_entry, record_year_award, record_team_award,
# compare_award_record, YearAwardSink/TeamAwardSink, FN_TEAM_AWARD_*) e do sx05b_5 (award_base, team_award_obj):
# integrar depois deles.
# Comparadores: ensaio da copia sem o registro (na equipe, a selecao guardada e reposta), depois a original; o registro
# mais novo do historico tem de ser o planejado (as notas so na estatistica 2).
SAVE = False
LIGAS = 'ENG'  # as do sx05b_1..4 e sx05b_20 (mesma leva)
ATE = '2002-08-01'
ROTINAS = [('nation_awards_award_tournament_player', 0x7690c0, [0x83, 0xec, 0x7c, 0x8b, 0x84, 0x24, 0x80, 0x00, 0x00, 0x00]),
           ('nation_awards_award_tournament_team', 0x769af0, [0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18])]
DOCS = ('o jogador e a equipe de um torneio',
        "a tournament's player and team",
        [('nation_awards_award_tournament_player', 'nation_awards_award_tournament_player', '0x7690c0', 'nation_awards.cpp', 'thiscall, ret 0x18',
          'regra: os três primeiros da tabela de jogadores do torneio pela estatística do prêmio que são jogadores, jogam numa das posições pedidas e foram titulares em ao menos uma fração de três jogos (0,66, baixando 0,1 por passada enquanto passar de 0); com idade-limite, os mais velhos são contados e os mais novos só entram depois de 3 contados. Com os três, o registro (ano, nomes, id, a estatística no prêmio 2, a seleção num torneio internacional ou o clube) vai para o prêmio do ano. Defeito: fora da estatística 2 a nota fica com o lixo da pilha',
          "rule: the first three of the tournament's player table for the award's statistic who are players, play one of the asked positions and started at least a share of three games (0.66, down by 0.1 a pass while above 0); with an age limit, the older are counted and the younger only qualify after 3 were counted. With all three, the record (year, names, id, the statistic for award 2, the national team in an international tournament or the club) goes to the year award. Defect: outside statistic 2 the score keeps stack garbage"),
         ('nation_awards_award_tournament_team', 'NationAwards::award_tournament_team', '0x769af0', 'nation_awards.cpp', 'thiscall, ret 8',
          'regra: limpa a seleção (reservas: as da competição de clubes do prêmio, 3 sem ela) e oferece ao prêmio, pela tabela de notas do torneio, cada jogador que foi titular em mais que a fração de três jogos (0,66, baixando 0,1 enquanto passar de 0), com a seleção num torneio internacional ou o clube e a nota média, até a equipe ficar completa; depois as datas da temporada e o registro',
          "rule: clears the selection (substitutes: those of the award's club competition, 3 without one) and offers the award, from the tournament's rating table, each player who started more than the share of three games (0.66, down by 0.1 while above 0), with the national team in an international tournament or the club and the average rating, until the team is full; then the season dates and the record")])
CAMPOS = []
INTERESSA = []
