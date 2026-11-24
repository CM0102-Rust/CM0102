# Os periodos e o grupo da repescagem de acesso da Segunda Divisao belga (bel_second.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com a Belgica (as ligas do cx02_21: 'ENG BEL HOL GER DEN', ate 2002-08-01): as fases dos
# periodos nascem com os resultados (add_result do mwc_24), o update a cada atualizacao da tabela e o grupo no fim do
# terceiro periodo (advance_stage do mwc_24, na fase 3), na primavera de 2002.
# - 0x420230 BelSecond::create_playoff_group thiscall() ret 0: completa ate 4 os status 1 (os campeoes dos periodos) com
#   os primeiros status 0xff (marca 1, virtual 0x40 (clube, 1, +0x44, 0, 0), virtual 0x68 (clube, +0x44, 2, 2, 7));
#   malloc 0x10 (caixa 0x2ac) com os 4 de status 1; virtual 0x3c (3, &rodadas, &rotulo, 0); League_Stage 0x68b100 (4
#   clubes, 2 turnos, lugares 1,0,0,0, fase 3, importancia 0x14, bits 0x22, modo 2) na vaga 3; free da lista e da
#   tabela; sem fase a caixa 0x2c5. Prologo push -1; push imm32.
# - 0x4204d0 BelSecond::create_period_stage thiscall(periodo) ret 4: malloc (caixa 0x346) com os clubes da tabela;
#   League_Stage com 1 turno, sem tabela de rodadas, lugares 0,1,0,0, o rotulo 0x439/0x43a/0x43b, importancia 0, bits 2,
#   10 ou 12 na vaga do -1, modo 2, na vaga do periodo (sem teste), +0x30 = periodo, free da lista. Prologo push -1;
#   push imm32.
# - 0x420680 BelSecond::update_positions_and_status thiscall() ret 0 (virtual 0x94): League::update_positions_and_status
#   0x684cd0 e o campeao do proximo periodo em aberto (+0xf6..+0xf8): as linhas dele com 0xff viram 1,
#   send_period_winner_news 0x420ca0 (cx08_17) e a virtual 0x68; o periodo marcado. Prologo sub esp, 8; push esi;
#   mov esi, ecx (6 bytes).
# Comparadores: so a original (marcam, anunciam, criam fases), com a fase criada conferida depois.
# Sem dependencias de outros itens (LeagueStageCtorFn ja publicado no bra_nat_third.rs).
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_second_create_playoff_group', 0x420230, [0x6a, 0xff, 0x68, 0x3e, 0x40, 0x95, 0x00]),
           ('bel_second_create_period_stage', 0x4204d0, [0x6a, 0xff, 0x68, 0x5e, 0x40, 0x95, 0x00]),
           ('bel_second_update_positions_and_status', 0x420680, [0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1])]
DOCS = ('os períodos e o grupo de acesso da Segunda Divisão belga', 'the Belgian Second Division periods and promotion play-off group',
        [('bel_second_create_playoff_group', 'BelSecond::create_playoff_group', '0x420230', 'bel_second.cpp', 'thiscall, ret 0',
          'o grupo da repescagem de acesso da Segunda Divisão belga: os três campeões de período são completados até quatro com os mais bem colocados ainda indefinidos, cada um marcado, anunciado e com o progresso gravado; os quatro jogam um grupo em ida e volta (fase 3)',
          'the Belgian Second Division promotion play-off group: the three period winners are completed to four with the best-placed clubs still undecided, each marked, announced and given its progress; the four meet in a home-and-away group (stage 3)'),
         ('bel_second_create_period_stage', 'BelSecond::create_period_stage', '0x4204d0', 'bel_second.cpp', 'thiscall, ret 4',
          'cada terço da temporada da Segunda Divisão belga é uma fase com os 18 clubes: o primeiro período (10 rodadas), o segundo e o terceiro (12 cada); criada, vira a fase atual',
          'each third of the Belgian Second Division season is a stage with all 18 clubs: the first period (10 rounds), the second and the third (12 each); once created it is the current stage'),
         ('bel_second_update_positions_and_status', 'BelSecond::update_positions_and_status', '0x420680', 'bel_second.cpp', 'thiscall, ret 0',
          'depois da atualização comum da tabela, o campeão do próximo período em aberto, quando a fase dele já chegou e ele lidera marcado como campeão, vai à repescagem de acesso: marcado na tabela da liga, com a notícia e o progresso; o período conta como decidido',
          'after the usual table update, the winner of the next open period, once its stage is reached and its leader is marked champion, goes to the promotion play-off: marked in the league table, with the news and the progress; the period counts as decided')])
CAMPOS = [('segunda divisão belga +0xf6, +0xf7, +0xf8: o período já tem campeão', 'Belgian Second Division +0xf6, +0xf7, +0xf8: the period has its winner')]
INTERESSA = []
