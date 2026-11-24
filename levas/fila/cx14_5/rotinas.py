# O construtor, a tabela e as honras da temporada da segunda divisao turca categoria B (tur_second_b.cpp; leitor
# cx14, rodada 5, 24/11/2026). Simulacao com a Turquia (as ligas e a data dos itens mwd): o construtor e a tabela rodam
# na criacao do jogo (a tabela de novo na virada do verao de 2002), as honras no fim da temporada.
# - 0x9002c0 TurSecondB::create_table thiscall() ret 0 (2 chamadas na cobertura do mundo): club_count = 10, malloc
#   0x24e (caixa 0x1c4); os clubes com divisao (+0x57) da categoria B (chave 0x9cf8e0) e divisao reserva (+0x60) do
#   grupo 1 (0x9cf8e4), entre os primeiros [0xae235c] - 2 x [0xae2350]; League::init_table_row 0x687430 (thiscall(k,
#   clube, 0, 0), ret 0x10; o original empurra um dword com o byte k e restos da pilha em cima, a rotina le o byte); o
#   11o clube: caixa 0x1d5, a divisao dele zerada e fim. Devolve 1 com a tabela cheia.
# - 0x901c60 TurSecondB::record_season_honours: mov eax, [ecx+0xc]; mov ecx, [eax+0x10]; mov edx, [ecx]; jmp
#   [edx+0x30]: segue para a fase 4 (o grupo de acesso). Nao e um thunk de um jmp so (carrega a fase); desviado. Sem
#   mutacao (a historia nao volta a simulacao); comparador so a original.
# - 0x8ff4c0 TurSecondB::TurSecondB thiscall(ano, registro) ret 8 (1 chamada): o corpo do construct_league_with_stages
#   numa ordem propria (regras 0x4a, tabela 0x51, ordem 0x58, Comp_Stats 0x60, jogos 0x69), depois
#   create_group_stage(i) 0x900780 (thiscall, ret 4) para i = 0..3 e set_predicted_positions 0x901950.
# Prologos: sub esp, 0x204 (6 bytes); mov eax, [ecx+0xc]; mov ecx, [eax+0x10] (6 bytes); push -1; push imm32 (7
# bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: a tabela so a original contra o plano; o construtor
# compare_league_with_stages_ctor.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_second_b_create_table', 0x9002c0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('tur_second_b_record_season_honours', 0x901c60, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x10]),
           ('construct_tur_second_b', 0x8ff4c0, [0x6a, 0xff, 0x68, 0x69, 0x50, 0x96, 0x00])]
DOCS = ('o construtor, a tabela e as honras da temporada da segunda divisão turca categoria B',
        'the constructor, the table and the season honours of the Turkish second division category B',
        [('tur_second_b_create_table', 'TurSecondB::create_table', '0x9002c0', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: a tabela da categoria B é a do seu primeiro grupo: os dez clubes da categoria B cujo grupo é o 1, pela ordem do banco de dados; um décimo primeiro perde a divisão (caixa de erro)',
          'rule: the category B table is its first group\'s: the ten category B clubs whose group is group 1, in database order; an eleventh loses its division (error box)'),
         ('tur_second_b_record_season_honours', 'TurSecondB::record_season_honours', '0x901c60', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: as honras da temporada da categoria B são as do grupo de acesso (a fase 4)',
          'rule: the season honours of category B are those of the promotion group (stage 4)'),
         ('construct_tur_second_b', 'TurSecondB::TurSecondB', '0x8ff4c0', 'tur_second_b.cpp', 'thiscall, ret 8',
          'regra: monta a categoria B turca na criação do jogo: regra de disciplina turca, onze fases (os outros quatro grupos, o grupo de acesso, os cinco grupos de classificação e o mata-mata); sem nada salvo, as regras, a tabela do grupo 1, a ordem, as estatísticas, os jogos, os outros quatro grupos e as posições previstas',
          'rule: builds the Turkish category B at game creation: Turkish discipline rule, eleven stages (the other four groups, the promotion group, the five classification groups and the play-off); with nothing saved, the rules, the group 1 table, the order, the statistics, the fixtures, the other four groups and the predicted positions')])
CAMPOS = []
INTERESSA = []
