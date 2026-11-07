# A Copa das Nações da Oceania de 2002: o dia do sorteio, o sorteio das seis seleções e a tabela do primeiro grupo
# (oceania_nations.cpp, em intercomp\). Frente das competições (Agente E, 07/11/2026, madrugada), carreira de 8 ligas
# (a Austrália jogável). O sorteio cai dentro da janela; os jogos (junho e novembro de 2002) ficam depois dela: as três
# mutações mexem nos sorteios (a contagem do diário acusa).
# - 0x79a100 (OceaniaNations::update_for_date): o dia. Passageira (contador no sorteio).
# - 0x799e30 (OceaniaNations::select_and_draw_teams): as seis seleções. Três mutações.
# - 0x7995a0 (OceaniaNations::create_group_table): o primeiro grupo. Passageira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); as outras thiscall, ret 0 (mov ecx, esi; call). Dentro: 0x549ef0 (push ano; push dia; lea ecx -> thiscall, ret
# 8), 0x6835c0 e 0x684230 (thiscall, ret 0), 0x6827d0 e 0x799a60 (push 0; mov ecx -> thiscall, ret 4), 0x6847c0 (2
# pushes, ret 8), 0x944e46 e 0x9452ca (push; add esp, 4 -> cdecl), 0x53d900, 0x53d940, 0x53b3d0 e 0x90cfc0 (push; add
# esp -> cdecl), 0x4afbd0 (3 pushes -> cdecl) e 0x687430 (4 pushes; mov ecx -> thiscall, ret 0x10).
# Comparadores: só a original, com o plano: o dia confere no sorteio +0x45, a lista zerada, a tabela e a fase; o
# sorteio a Austrália e a Nova Zelândia nas vagas 0 e 3; o grupo as 3 linhas na ordem.
# Determinismo: as listas locais de melanésias e polinésias só são lidas até a conta (e as vagas 0 e 1 de cada uma,
# que existem nas bases do jogo).
# Provas: em curso.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqv10_oceania_nacoes_dia', 0x79a100, [0x6a, 0xff, 0x68, 0x26, 0xec, 0x95, 0x00]),
           ('cqv10_oceania_nacoes_sorteio', 0x799e30, [0x81, 0xec, 0x54, 0x02, 0x00, 0x00]),
           ('cqv10_oceania_nacoes_primeiro_grupo', 0x7995a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a Copa das Nações da Oceania: o dia do sorteio, o sorteio das seis seleções e a tabela do primeiro grupo',
        'the Oceania Nations Cup: the draw day, the draw of the six nations and the first group table',
        [('cqv10_oceania_nacoes_dia', 'OceaniaNations::update_for_date', '0x79a100', 'oceania_nations.cpp', 'thiscall, ret 8', 'regra: a Copa das Nações da Oceania é sorteada no dia do sorteio (+0xb6/+0xb8, só na chamada completa): as seis seleções, as posições previstas, o primeiro grupo na tabela da copa, os jogos e o segundo grupo como fase; antes do dia a copa nem passa o dia de liga', 'rule: the Oceania Nations Cup is drawn on the draw day (+0xb6/+0xb8, only on the full call): the six nations, the predicted positions, the first group in the cup table, the fixtures and the second group as a stage; before the day the cup does not even run the league day'),
         ('cqv10_oceania_nacoes_sorteio', 'OceaniaNations::select_and_draw_teams', '0x799e30', 'oceania_nations.cpp', 'thiscall, ret 0', 'regra: a Copa das Nações da Oceania tem seis seleções em dois grupos de três: a Austrália e a Nova Zelândia cabeças, cada uma num grupo por sorteio; duas melanésias e duas polinésias (das até 10 de cada, pela reputação e embaralhadas) completam os grupos, uma de cada em cada grupo, também por sorteio', 'rule: the Oceania Nations Cup has six nations in two groups of three: Australia and New Zealand seeded, one in each group by draw; two Melanesian and two Polynesian nations (of up to 10 each, by reputation and shuffled) complete the groups, one of each per group, also by draw'),
         ('cqv10_oceania_nacoes_primeiro_grupo', 'OceaniaNations::create_group_table', '0x7995a0', 'oceania_nations.cpp', 'thiscall, ret 0', 'regra: o primeiro grupo da Copa das Nações da Oceania tem as seleções das vagas 0, 1 e 2, cada uma na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio', 'rule: the first Oceania Nations Cup group has the nations of places 0, 1 and 2, each in the row of its order; human managers get the draw news')])
CAMPOS = [('copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)', 'cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('copa +0xee a lista das 6 seleções, +0xf2 a conta (word); ponteiro/valor; grava (sorteio), lê (grupo), libera e zera (dia)', 'cup +0xee the 6-nation list, +0xf2 its count (word); pointer/value; writes (draw), reads (group), frees and clears (day)'),
          ('nação +0x71 continente ([0xae23a8] + 0x122 × id); ponteiro; lê (sorteio)', 'nation +0x71 continent ([0xae23a8] + 0x122 × id); pointer; reads (draw)'),
          ('copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes); valor/ponteiro; grava (grupo)', 'cup +0x3e teams, +0xb1 table (0x3b-byte rows); value/pointer; writes (group)')]
INTERESSA = ['CMRTE', 'GPF2']
