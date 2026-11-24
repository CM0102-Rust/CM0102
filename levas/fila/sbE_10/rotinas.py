# A tabela da Primeira Divisao polonesa, o seu segundo grupo e a fase final (pol_first.cpp; leitor sbE, rodada 5,
# 24/11/2026). Simulacao com a Polonia (as ligas e a data dos itens mwd): a tabela e o segundo grupo na criacao do jogo
# (o construtor, sbE_9) e na virada (PolFirst::start_new_season 0x7c9740); a fase final em novembro de 2001
# (PolFirst::advance_stage 0x7c9ad0, mwd_31).
# - 0x7c9450 PolFirst::create_table thiscall() ret 0: +0x3e = 8; malloc 0x1d8 em +0xb1 (caixa 0x13b); free da lista
#   antiga (+0xee) e calloc 0x945501(16, 4) (caixa 0x146); os clubes da Primeira (indice de 16 bits) pela semente
#   (+0x5f, a ultima posicao; promovido, +0x5b fora desta liga: 1 e 2 -> 15 e 16, 3 -> 14, 4 -> 13) na vaga dessa
#   semente da tabela local (1,4,5,8,9,12,13,16,2,3,6,7,10,11,14,15) ou na primeira vaga livre; as 8 primeiras nas
#   linhas (0x687430); TV 0x683010(0x7a120, 0); 1. Prologo sub esp, 0x210 (6 bytes).
# - 0x7c9900 PolFirst::create_second_group_stage thiscall() ret 0: virtual 0x3c (0, &rodadas, &extra, 0); malloc (caixa
#   0x226) com as vagas 8.. de +0xee; League_Stage 0x68b100 (2 turnos, fase 0, o extra de rotulo, importancia +0x51,
#   bits +0xd9, -1, modo 2) na vaga 0; sem fase a caixa 0x238 (sem os frees); senao free e +0x30 = 0. Prologo mov eax,
#   fs:[0] (6 bytes).
# - 0x7c9b60 PolFirst::create_final_stage thiscall() ret 0: malloc 0x40 (caixa 0x277); as linhas 0..3 da liga e da fase
#   0 alternadas, depois as 4..7, com a divisao reserva (+0x60) no registro de 0x9cf814 e 0x9cf818; League_Stage de 16
#   (sem tabela, lugares 0,0,2,2, fase 1, importancia 0x14, bits 3, 0xe, fundo calc_prize_fund 0x682f70(0x11170), modo 1)
#   na vaga 1 sem teste; cada linha recebe da liga (League::find_club_row 0x68ab70) ou da fase 0 os pontos pela metade
#   ((x+1)/2; fora a diferenca) e os resultados; +0xa3 = 0x7ca450; sort_table(0); +0x30 = 1; dois League_Stage de 8 da
#   fase 1 (as tabelas da virtual 0x3c) construidos e destruidos (virtual 0 com 1), a primeira tabela liberada e a
#   segunda nao. Prologo push -1; push imm32 (7 bytes).
#   Defeitos do original: a marca do grupo sobrescreve a divisao reserva (+0x60); a lista e a segunda tabela vazam.
# Depende do sbE_9 (POL_FIRST_CLUB_LIST): integrar depois dele.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (a tabela contra o plano da semeadura; as fases
# conferidas depois).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_first_create_table', 0x7c9450, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('pol_first_create_second_group_stage', 0x7c9900, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('pol_first_create_final_stage', 0x7c9b60, [0x6a, 0xff, 0x68, 0xba, 0xf7, 0x95, 0x00])]
DOCS = ('a tabela da Primeira Divisão polonesa, o segundo grupo e a fase final',
        'the Polish First Division table, its second group and its final stage',
        [('pol_first_create_table', 'PolFirst::create_table', '0x7c9450', 'pol_first.cpp', 'thiscall, ret 0',
          'os 16 clubes semeados pela posição da temporada anterior em dois grupos de oito (posições 1, 4, 5, 8, 9, 12, 13 e 16 no primeiro; os promovidos ficam com as posições 13 a 16); o primeiro grupo é a tabela da liga; a TV paga 500 mil a cada clube',
          'the 16 clubs seeded by last season\'s place into two groups of eight (places 1, 4, 5, 8, 9, 12, 13 and 16 in the first; the promoted clubs take places 13 to 16); the first group is the league table; TV pays 500,000 to each club'),
         ('pol_first_create_second_group_stage', 'PolFirst::create_second_group_stage', '0x7c9900', 'pol_first.cpp', 'thiscall, ret 0',
          'o segundo grupo de oito (os oito últimos da semeadura) é a fase 0, em ida e volta, e vira a fase atual',
          'the second group of eight (the last eight of the seeding) is stage 0, home and away, and becomes the current stage'),
         ('pol_first_create_final_stage', 'PolFirst::create_final_stage', '0x7c9b60', 'pol_first.cpp', 'thiscall, ret 0',
          'os quatro primeiros de cada grupo vão ao grupo do título e os outros quatro ao do rebaixamento, numa tabela de 16 que leva a metade dos pontos e os resultados; os dois grupos de oito ganham as suas rodadas',
          'the top four of each group go to the championship group and the other four to the relegation group, in one table of 16 carrying half their points and their results; the two groups of eight get their own rounds')])
CAMPOS = []
INTERESSA = []
