# A Copa Africana de Nações: o dia do sorteio, o sorteio dos 16 e as fases dos grupos B a D (african_nations.cpp, em
# intercomp\). Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (a Copa Africana de 2002, de
# janeiro e fevereiro, fica dentro da janela).
# - 0x4025a0 (AfricanNations::update_for_date): o dia; sem `completo` só o dia de liga; antes do sorteio volta sem ele;
#   no dia do sorteio o sorteio, as previsões, as tabelas, os jogos e as 3 fases. Passageira (contador no sorteio).
# - 0x4022c0 (AfricanNations::draw_finalists, 721 bytes): as sedes e as africanas, os potes e os 4 grupos. Duas
#   mutações (as 50 viram 16; sem a ordem pela reputação).
# - 0x401ea0 (AfricanNations::create_group_stage): as fases dos grupos B a D. Uma mutação (os times de trás para a
#   frente).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); o sorteio thiscall, ret 0 (mov ecx, esi; call, do dia); a fase thiscall, ret 4 (push g; mov ecx, do dia).
# Dentro: 0x4019c0, 0x6835c0 (thiscall, ret 0), 0x6827d0 (push 0; mov ecx -> thiscall, ret 4), as virtuais +0x5c e
# +0x3c, 0x5fa730 (5 pushes; mov ecx, [0xb5e134] -> thiscall, ret 0x14), 0x53b3d0 (push; add esp -> cdecl), 0x945333
# (4 pushes; add esp -> cdecl), 0x90cfc0 (push 4; add esp -> cdecl), 0x944e46, 0x944cf1, 0x9452ca, 0x68b100 (22
# pushes; mov ecx, novo -> thiscall, ret 0x58), 0x684230 (mov ecx -> thiscall, ret 0), 0x6847c0 (2 pushes; mov ecx ->
# thiscall, ret 8) e a caixa; a 0x549ef0 (a data do sorteio, pura) fica de fora: a cópia compara os campos.
# Comparadores: o sorteio lado a lado (o +0xf2, o [0xb67a34] e o estado do sorteador; a cópia sorteia sem contar; os
# grupos da cópia comparados nas 16 vagas e liberados); o dia e a fase só a original (alocam e mandam notícia), com o
# plano (o dia: +0x45, a lista, a tabela e as fases; a fase: a fase, +0x30, os times, o ano e as 4 seleções).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs13_africana_dia', 0x4025a0, [0x6a, 0xff, 0x68, 0x46, 0x35, 0x95, 0x00]),
           ('cqs13_africana_sorteio', 0x4022c0, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00]),
           ('cqs13_africana_grupo', 0x401ea0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Copa Africana de Nações: o dia do sorteio, o sorteio dos 16 e as fases dos grupos B a D',
        'the African Nations Cup: the draw day, the draw of the 16 and the group B to D stages',
        [('cqs13_africana_dia', 'AfricanNations::update_for_date', '0x4025a0', 'african_nations.cpp', 'thiscall, ret 8', 'regra: a Copa Africana é sorteada no dia do começo (+0xb6/+0xb8): o sorteio dos 16, as posições previstas, o grupo A na tabela da própria copa, os jogos e os grupos B a D como fases; antes desse dia a copa nem passa o dia de liga', "rule: the African Nations Cup is drawn on the start day (+0xb6/+0xb8): the draw of the 16, the predicted positions, group A in the cup's own table, the fixtures and groups B to D as stages; before that day the cup does not even run the league day"),
         ('cqs13_africana_sorteio', 'AfricanNations::draw_finalists', '0x4022c0', 'african_nations.cpp', 'thiscall, ret 0', 'regra: a Copa Africana tem 16 seleções: as sedes na frente e as africanas (até 50, na ordem das nações, sem as sedes) ordenadas pela reputação; os potes são os blocos de 4 da lista (as sedes e as melhores no pote 1); cada um dos 4 grupos recebe uma seleção sorteada de cada pote; menos de 16 africanas, não há sorteio', "rule: the African Nations Cup has 16 nations: the hosts first and the African nations (up to 50, in nation order, without the hosts) sorted by reputation; the pots are the list's blocks of 4 (the hosts and the best in pot 1); each of the 4 groups gets one nation drawn from each pot; with fewer than 16 African nations there is no draw"),
         ('cqs13_africana_grupo', 'AfricanNations::create_group_stage', '0x401ea0', 'african_nations.cpp', 'thiscall, ret 4', 'regra: os grupos B a D da Copa Africana são fases de liga com as 4 seleções sorteadas para eles, as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia', "rule: groups B to D of the African Nations Cup are league stages with the 4 nations drawn into them, the cup's rules and year and the cup's round table for the group; human managers get the news")])
CAMPOS = [('copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)', 'cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('copa +0xee lista (50 seleções, depois os 16 dos grupos), +0xf2 a conta (palavra, 16); ponteiro/valor; grava (sorteio), lê (fases), zera (dia)', 'cup +0xee list (50 nations, then the 16 of the groups), +0xf2 its count (word, 16); pointer/value; writes (draw), reads (stages), clears (day)'),
          ('nação +0x71 continente (o +0 dele é o id, [0x9cfa08] a África); seleção +0x53 nação; ponteiro/id; lê (sorteio)', 'nation +0x71 continent (its +0 is the id, [0x9cfa08] Africa); national team +0x53 nation; pointer/id; reads (draw)'),
          ('copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)', 'cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)')]
INTERESSA = ['CMRTE']
