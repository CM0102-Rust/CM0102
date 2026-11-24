# O calendario, a tabela do primeiro grupo e as fases de grupo da Copa da Grecia (gre_cup.cpp; leitor sbF, rodada 5,
# 24/11/2026). Competicoes da Grecia; ATE 2002-08-01 (a copa e montada na criacao, pelo construtor 0x5e88a0 do sbF_3, e
# refeita na virada, GreCup::start_new_season do cx02_18).
# - 0x5e8bf0 GreCup::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10: o molde da ItaCup
#   (create_schedule) com o mata-mata na fase 10, o rotulo 0x3e9, as caixas 0xc3 e 0xee; conferido por emulacao
#   (unicorn) nas fases -128..127, ano impar e par: as mesmas chamadas e os mesmos 16 campos por rodada.
# - 0x5e9260 GreCup::create_table thiscall() ret 0 -> 1/0: 4 clubes da coluna 0 do pool de 44 (+0xee, passo 11), SEM
#   zerar a entrada (diferente da WelPrmCup::create_group_table e da ItaCCup); malloc 0xec, caixa 0x1ab.
# - 0x5e9650 GreCup::create_group_stage thiscall(g) ret 4 -> 1/0: o corpo da WelPrmCup::create_group_stage (conferido
#   instrucao a instrucao) com o passo 11, sem zerar o pool, o 4o argumento do League_Stage 1 (e nao 2) e o desempate
#   local (2, 0, 0, 0); caixas 0x271 e 0x286. SEH so para o new.
# Prologos: mov al, [esp+4]; sub esp, 0x200; sub esp, 0x204; push -1; push 0x95a98e. Iguais no cm0102-1080.exe.
# Comparadores: o calendario lado a lado; a tabela e a fase so a original, com o plano (as colunas do pool).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_cup_create_schedule', 0x5e8bf0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_cup_create_table', 0x5e9260, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gre_cup_create_group_stage', 0x5e9650, [0x6a, 0xff, 0x68, 0x8e, 0xa9, 0x95, 0x00])]
DOCS = ('o calendário, a tabela do primeiro grupo e as fases de grupo da Copa da Grécia',
        'the Greek Cup schedule, first group table and group stages',
        [('gre_cup_create_schedule', 'GreCup::create_schedule', '0x5e8bf0', 'gre_cup.cpp', 'thiscall, ret 0x10',
          'regra: os onze grupos de quatro da Copa da Grécia (o grupo da própria copa e as fases 0 a 9) têm seis rodadas de 11 de agosto a 8 de setembro, cada grupo uma fase de liga (é_liga = 1, rótulo 0x3e9 + fase); o mata-mata (fase 10) tem 22 clubes: 12 na 1ª rodada (15 de dezembro), mais 10 na 2ª (13 de janeiro), quartas e semifinais em ida e volta (14 dias) e a final em 10 de maio em campo neutro [inferência]',
          'rule: the Greek Cup eleven groups of four (the cup\'s own group and stages 0 to 9) play six matchdays from 11 August to 8 September, each group a league stage (is_league = 1, label 0x3e9 + stage); the knockout (stage 10) has 22 clubs: 12 in the 1st round (15 December), 10 more in the 2nd (13 January), two-legged quarter- and semi-finals (14 days apart) and the final on 10 May at a neutral ground [inference]'),
         ('gre_cup_create_table', 'GreCup::create_table', '0x5e9260', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: o grupo da própria Copa da Grécia (a copa é a liga do primeiro grupo) recebe os quatro clubes da coluna 0 do pool de inscritos (as entradas 0, 11, 22 e 33), que não saem do pool; depois a notícia do sorteio do grupo',
          'rule: the Greek Cup\'s own group (the cup is the league of its first group) takes the four clubs of column 0 of the entrant pool (entries 0, 11, 22 and 33), which stay in the pool; then the group draw news'),
         ('gre_cup_create_group_stage', 'GreCup::create_group_stage', '0x5e9650', 'gre_cup.cpp', 'thiscall, ret 4',
          'regra: a fase de grupo g (0 a 9) da Copa da Grécia é uma fase de liga com os quatro clubes da coluna g + 1 do pool, no calendário de grupos da copa; depois a notícia do sorteio do grupo e a fase atual passa a g',
          'rule: Greek Cup group stage g (0 to 9) is a league stage of the four clubs of pool column g + 1, on the cup\'s group schedule; then the group draw news and the current stage becomes g')])
CAMPOS = []
INTERESSA = []
