# As fases das eliminatórias europeias: andar de fase, a fase dos segundos colocados e o grupo do próprio objeto
# (wc_europe_league.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação na campanha de 2004 (o sorteio europeu de 2004,
# os grupos até outubro de 2005, a repescagem em novembro de 2005): ATE = '2005-12-31' (o pedido da rodada 5).
# - 0x919620 WcEuropeLeague::advance_stage thiscall ret 0 (virtual +0x28): na última fase ou além (+0x2c - 1, com
#   sinal), caixa 0x37f; senão +0x30 + 1 e, se a fase nova for a dos segundos (+0x100) ou a da repescagem (+0x101),
#   0x919910 create_play_off_stage (chamada direta). Prólogo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x91a140 WcEuropeLeague::create_runners_up_stage thiscall ret 0 (quadro SEH só para o new): +0xbe = 0; operator new
#   0xee e League_Stage::League_Stage 0x68b100 (ret 0x58: a seção, +0xff + 2, 0, 0, o registro, 0, 0, +0xc2, +0xc3,
#   +0xc4, {1, 2, 5, 0}, &+0xbe, o ano, +0x100, 0x3fc, 0, 4, 0, 2, -1, 0, 2) em +0xc[+0x100] (sem conferir nulo); +0xbe =
#   1; +0x30 = +0x100; 0x91a240 fill_runners_up_table. Prólogo push -1; push imm32 (7 bytes).
# - 0x9190d0 WcEuropeLeague::init_own_group_table thiscall ret 0: +0x3e = get_group_size(-1) (0x918dd0, ret 4), +0xb1 =
#   malloc(0x3b x n) (caixa 0x2b2 e 0), +0xa7 = (n - 1) x +0x3c (words), as linhas com as vagas sorteadas +0xf7[i]
#   (League::init_table_row 0x687430, ret 0x10), a notícia do sorteio (0x684230); 1. Prólogo sub esp, 0x204 (6 bytes).
# Iguais no cm0102-1080.exe. Comparadores: só a original contra o plano (a fase anda uma; a fase dos segundos com uma
# linha por grupo; as linhas do grupo com as vagas sorteadas).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_europe_league_advance_stage', 0x919620, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('wc_europe_league_create_runners_up_stage', 0x91a140, [0x6a, 0xff, 0x68, 0x1b, 0x5a, 0x96, 0x00]),
           ('wc_europe_league_init_own_group_table', 0x9190d0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as fases das eliminatórias europeias: andar de fase, a fase dos segundos e o grupo do próprio objeto',
        'the European qualifying stages: moving on, the runners-up stage and the section own group',
        [('wc_europe_league_advance_stage', 'WcEuropeLeague::advance_stage', '0x919620', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: as eliminatórias europeias andam uma fase por vez; chegar à fase dos segundos ou à da repescagem monta a repescagem; passar da última é erro',
          'rule: the European qualifying moves on one stage at a time; reaching the runners-up stage or the play-off stage builds the play-off; past the last stage it is an error'),
         ('wc_europe_league_create_runners_up_stage', 'WcEuropeLeague::create_runners_up_stage', '0x91a140', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: os segundos colocados europeus são classificados numa liga própria, um por grupo, com os pontos e a categoria da seção e os desempates 1, 2 e 5; o melhor passa',
          'rule: the European runners-up are ranked in a league stage of their own, one per group, with the section points and category and the tie-breaks 1, 2 and 5; the best goes through'),
         ('wc_europe_league_init_own_group_table', 'WcEuropeLeague::init_own_group_table', '0x9190d0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: o primeiro grupo das eliminatórias europeias é o próprio objeto: o tamanho pelo número de isentas, as seleções das primeiras vagas sorteadas e a notícia do sorteio',
          'rule: the first European qualifying group is the section itself: its size by the exempt count, its nations from the first drawn slots and the draw news')])
CAMPOS = [('eliminatórias da Europa +0xf7 tabela das vagas sorteadas (ponteiro), +0xff último grupo, +0x100 fase dos segundos, +0x101 fase da repescagem (bytes); valor/ponteiro; lê',
           'European qualifying +0xf7 drawn slot table (pointer), +0xff last group, +0x100 runners-up stage, +0x101 play-off stage (bytes); value/pointer; reads')]
INTERESSA = []
