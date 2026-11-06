# A Libertadores: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_liber.cpp, em eurocomp\).
# Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x4c4410 (ConmebolLiber::update_for_date): o dia; no dia do sorteio os clubes, as tabelas, os jogos e as 7 fases. Uma mutação (o sorteio uma
#   semana depois).
# - 0x4c37a0 (ConmebolLiber::build_first_group_table): a tabela do primeiro grupo. Uma mutação (no ajudante dos sorteados, que as duas leem: o 4º e o 5º
#   trocam de grupo).
# - 0x4c3b10 (ConmebolLiber::create_group_stage): a fase de cada um dos outros grupos. Uma mutação (os times de cada fase de trás para a frente).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); a tabela é thiscall, ret 0 (mov ecx, esi; call, do dia); a fase thiscall, ret 4 (push g; mov ecx, do dia).
# Dentro: 0x4c3f40 (mov ecx -> thiscall, ret 0), 0x6835c0 (thiscall, ret 0), 0x6827d0 (push 0; mov ecx -> thiscall, ret
# 4), as virtuais +0x5c e +0x3c (thiscall, ret 0 e ret 0x10), 0x944e46 e 0x944cf1 (push; add esp, 4 -> cdecl),
# 0x9452ca (push; add esp -> cdecl), 0x687430 (4 pushes; mov ecx -> thiscall, ret 0x10), 0x684230 (mov ecx ->
# thiscall, ret 0), 0x68b100 (22 pushes; mov ecx, novo -> thiscall, ret 0x58), 0x6847c0 (2 pushes; mov ecx ->
# thiscall, ret 8) e a caixa (aviso_em).
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
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS (os grupos
#   trocados e os times de trás para a frente em 2001-042, o sorteio uma semana depois em 2001-015).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr10_libertadores_dia', 0x4c4410, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('cqr10_libertadores_primeiro_grupo', 0x4c37a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqr10_libertadores_grupo', 0x4c3b10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Libertadores: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos',
        "the Copa Libertadores: the draw day, the first group table and the other groups' stages",
        [('cqr10_libertadores_dia', 'ConmebolLiber::update_for_date', '0x4c4410', 'conmebol_liber.cpp', 'thiscall, ret 8', 'regra: a Libertadores tem 32 clubes (os que as regras sul-americanas põem nela, embaralhados) em 8 grupos de 4, em dois turnos (3 pontos a vitória, 1 o empate; dois classificados por grupo, para o mata-mata de 16); o sorteio é no dia do começo (15 de janeiro do ano da copa): o grupo 1 fica na tabela da própria copa (os sorteados 0 a 3, na ordem) e os grupos 2 a 8 são fases (os sorteados 4g + 4 a 4g + 7); no dia do sorteio: os clubes (0x4c3f40 (os 32 clubes com a Libertadores como competição da temporada, embaralhados)), a tabela do grupo 1, os jogos, a tabela ordenada, as fases dos outros grupos e as posições previstas; depois o dia de liga', "rule: the Libertadores has 32 clubs (those the South American rules put in it, shuffled) in 8 groups of 4, home and away (3 points a win, 1 a draw; two qualify from each group, to the round of 16); the draw is on the start day (15 January of the cup year): group 1 stays in the cup's own table (drawn clubs 0 to 3, in order) and groups 2 to 8 are stages (drawn clubs 4g + 4 to 4g + 7); on the draw day: the clubs (0x4c3f40 (the 32 clubs with the Libertadores as their season competition, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted positions; then the league day"),
         ('cqr10_libertadores_primeiro_grupo', 'ConmebolLiber::build_first_group_table', '0x4c37a0', 'conmebol_liber.cpp', 'thiscall, ret 0', 'regra: o grupo 1 da tem 4 clubes, os 4 primeiros sorteados, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo Libertadores', 'rule: group 1 of the Copa Libertadores has 4 clubs, the first 4 drawn, each in the row of its order; human managers get the group draw news'),
         ('cqr10_libertadores_grupo', 'ConmebolLiber::create_group_stage', '0x4c3b10', 'conmebol_liber.cpp', 'thiscall, ret 4', 'regra: cada um dos outros grupos da é uma fase de liga com os 4 sorteados seguintes (4g + 4 a 4g + 7), as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia Libertadores', "rule: each of the other groups of the Copa Libertadores is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get the news")])
CAMPOS = [('copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)', 'cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('copa +0xee lista dos sorteados (ponteiros de clube); ponteiro; lê (tabela e fases) e libera e zera (dia)', 'cup +0xee drawn club list (club pointers); pointer; reads (table and stages) and frees and clears (day)'),
          ('copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)', 'cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)'),
          ('copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)', 'cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)')]
INTERESSA = ['CMRTE']
