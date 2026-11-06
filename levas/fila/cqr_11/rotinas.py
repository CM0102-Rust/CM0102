# A Mercosul: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_merc.cpp, em eurocomp\).
# Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x4c5c90 (ConmebolMerc::update_for_date): o dia; no dia do sorteio os clubes, as tabelas, os jogos e as 4 fases. Uma mutação (o sorteio uma
#   semana depois).
# - 0x4c4e60 (ConmebolMerc::create_first_group_table): a tabela do primeiro grupo. Uma mutação (no ajudante dos sorteados, que as duas leem: o 4º e o 5º
#   trocam de grupo).
# - 0x4c51d0 (ConmebolMerc::create_group_stage): a fase de cada um dos outros grupos. Uma mutação (os times de cada fase de trás para a frente).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); a tabela é thiscall, ret 0 (mov ecx, esi; call, do dia); a fase thiscall, ret 4 (push g; mov ecx, do dia).
# Dentro: 0x4c5850 (mov ecx -> thiscall, ret 0), 0x6835c0 (thiscall, ret 0), 0x6827d0 (push 0; mov ecx -> thiscall, ret
# 4), as virtuais +0x5c e +0x3c (thiscall, ret 0 e ret 0x10), 0x944e46 e 0x944cf1 (push; add esp, 4 -> cdecl),
# 0x9452ca (push; add esp -> cdecl), 0x687430 (4 pushes; mov ecx -> thiscall, ret 0x10), 0x684230 (mov ecx ->
# thiscall, ret 0), 0x68b100 (22 pushes; mov ecx, novo -> thiscall, ret 0x58), 0x6847c0 (2 pushes; mov ecx ->
# thiscall, ret 8), 0x549f70 (lea ecx, local; 4 pushes -> thiscall, ret 0x10, pura) e a caixa (aviso_em).
# Comparadores: só a original (as três alocam, sorteiam ou mandam notícia), com o plano: o dia confere no dia do
# sorteio +0x45, a lista zerada, a tabela e as fases; a tabela confere as 4 linhas na ordem dos sorteados; a fase
# confere a fase, +0x30, os times, o ano e os 4 clubes da tabela dela.
# Determinismo: nada lê pilha, heap ou endereço como valor (a data local do dia é lida só nos campos gravados).
# Provas (06/11/2026, cópia fora do projeto em %TEMP%: motor.rs da principal com a 280a publicada e a 281a integrada,
# 1.986 rotinas, + os 12 itens cqr_10..13, cqs_10..13, cqt_10..13, 36 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 só com os 4 cqr (parou em 2001-273) e até
#   2002-06-01 com os 12 itens (parou em 2002-151; as copas de janeiro e fevereiro de 2002 e a Libertadores de 2002
#   rodam depois de outubro): 0 linhas DIVERGENCIA, 0 EXCECAO, nenhuma caixa (data, motor e aleatorio: 0 divergências);
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a (a mesma da base sem os itens: as cópias são
#   fiéis); todos os contadores das mutações rodaram;
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS (o sorteio
#   uma semana depois em 2001-166, o 4º e o 5º trocados e os times de trás para a frente em 2001-213).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr11_mercosul_dia', 0x4c5c90, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('cqr11_mercosul_primeiro_grupo', 0x4c4e60, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqr11_mercosul_grupo', 0x4c51d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Mercosul: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos',
        "the Copa Mercosur: the draw day, the first group table and the other groups' stages",
        [('cqr11_mercosul_dia', 'ConmebolMerc::update_for_date', '0x4c5c90', 'conmebol_merc.cpp', 'thiscall, ret 8', 'regra: a Mercosul tem 20 convidados (os clubes que as regras sul-americanas convidam, embaralhados) em 5 grupos de 4 [inferência: seis rodadas às terças, de 29 de julho a 14 de outubro]; a temporada vira em 1º de junho; no dia do sorteio o grupo 1 fica na tabela da própria copa (os sorteados 0 a 3) e os grupos 2 a 5 são fases (4g + 4 a 4g + 7); os 5 primeiros e os 3 melhores segundos vão às quartas; no dia do sorteio: os clubes (0x4c5850 (os 20 convidados, embaralhados)), a tabela do grupo 1, os jogos, a tabela ordenada, as fases dos outros grupos e as posições previstas; depois o dia de liga', "rule: the Mercosur has 20 invited clubs (the clubs the South American rules invite, shuffled) in 5 groups of 4 [inference: six rounds on Tuesdays, from 29 July to 14 October]; the season turns on 1 June; on the draw day group 1 stays in the cup's own table (drawn clubs 0 to 3) and groups 2 to 5 are stages (4g + 4 to 4g + 7); the 5 winners and the 3 best runners-up go to the quarter-finals; on the draw day: the clubs (0x4c5850 (the 20 invited clubs, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted positions; then the league day"),
         ('cqr11_mercosul_primeiro_grupo', 'ConmebolMerc::create_first_group_table', '0x4c4e60', 'conmebol_merc.cpp', 'thiscall, ret 0', 'regra: o grupo 1 da tem 4 clubes, os 4 primeiros sorteados, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo Mercosul', 'rule: group 1 of the Copa Mercosur has 4 clubs, the first 4 drawn, each in the row of its order; human managers get the group draw news'),
         ('cqr11_mercosul_grupo', 'ConmebolMerc::create_group_stage', '0x4c51d0', 'conmebol_merc.cpp', 'thiscall, ret 4', 'regra: cada um dos outros grupos da é uma fase de liga com os 4 sorteados seguintes (4g + 4 a 4g + 7), as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia Mercosul', "rule: each of the other groups of the Copa Mercosur is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get the news")])
CAMPOS = [('copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)', 'cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('copa +0xee lista dos sorteados (ponteiros de clube); ponteiro; lê (tabela e fases) e libera e zera (dia)', 'cup +0xee drawn club list (club pointers); pointer; reads (table and stages) and frees and clears (day)'),
          ('copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)', 'cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)'),
          ('copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)', 'cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)')]
INTERESSA = ['CMRTE']
