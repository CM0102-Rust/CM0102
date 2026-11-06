# O Campeonato de Clubes da Oceania: o dia do sorteio e as tabelas dos grupos A e B (oceania_club_champ.cpp, em
# eurocomp\). Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (a Austrália jogável; os grupos de
# janeiro de 2002 ficam dentro da janela).
# - 0x797b40 (OceaniaClubChamp::update_for_date): o dia; antes do sorteio volta sem o dia de liga; no dia do sorteio
#   (só na chamada sem `completo`) os 8 clubes, as previsões, as tabelas, os jogos e a fase do grupo B. Passageira
#   (contador no sorteio).
# - 0x7978c0 (OceaniaClubChamp::create_group_a_table): o grupo A (os escolhidos 0 a 3). Duas mutações (no ajudante
#   dos escolhidos: o 4º e o 5º trocam de grupo; as linhas do grupo A de trás para a frente).
# - 0x797e80 (OceaniaClubChamp::create_group_b_stage): o grupo B (4 a 7). Uma mutação (de trás para a frente).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); as duas tabelas são thiscall, ret 0 (mov ecx, esi; call, do dia). Dentro: 0x798220, 0x6835c0 (thiscall, ret 0),
# 0x6827d0 (push 0; mov ecx -> thiscall, ret 4), as virtuais +0x5c e +0x3c, 0x944e46 e 0x944cf1 (push; add esp, 4 ->
# cdecl), 0x9452ca, 0x687430 (4 pushes; mov ecx -> thiscall, ret 0x10), 0x684230 (mov ecx -> thiscall, ret 0),
# 0x68b100 (22 pushes; mov ecx, novo -> thiscall, ret 0x58), 0x6847c0 (2 pushes; mov ecx -> thiscall, ret 8) e a
# caixa; a 0x549ef0 (a data do sorteio, pura) fica de fora: a cópia compara os campos.
# Comparadores: só a original (as três alocam, sorteiam ou mandam notícia), com o plano: o dia confere no dia do
# sorteio +0x45, a lista e +0xf2 zeradas, a tabela, a fase e +0x30; o grupo A confere as 4 linhas na ordem; o grupo B
# a fase, +0x30, os times, o ano e os 4 clubes da tabela dela.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (06/11/2026, cópia fora do projeto em %TEMP%: motor.rs da principal com a 280a publicada e a 281a integrada,
# 1.986 rotinas, + os 12 itens cqr_10..13, cqs_10..13, cqt_10..13, 36 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 só com os 4 cqr (parou em 2001-273) e até
#   2002-06-01 com os 12 itens (parou em 2002-151; as copas de janeiro e fevereiro de 2002 e a Libertadores de 2002
#   rodam depois de outubro): 0 linhas DIVERGENCIA, 0 EXCECAO, nenhuma caixa (data, motor e aleatorio: 0 divergências);
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a (a mesma da base sem os itens: as cópias são
#   fiéis); todos os contadores das mutações rodaram;
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS (o 4º e o
#   5º trocados em 2002-006, o grupo B de trás para a frente em 2002-009, o grupo A de trás para a frente em 2002-011).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt10_oceania_dia', 0x797b40, [0x6a, 0xff, 0x68, 0x16, 0xeb, 0x95, 0x00]),
           ('cqt10_oceania_grupo_a', 0x7978c0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqt10_oceania_grupo_b', 0x797e80, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o Campeonato de Clubes da Oceania: o dia do sorteio e as tabelas dos grupos A e B',
        'the Oceania Club Championship: the draw day and the group A and B tables',
        [('cqt10_oceania_dia', 'OceaniaClubChamp::update_for_date', '0x797b40', 'oceania_club_champ.cpp', 'thiscall, ret 8', 'regra: o Campeonato de Clubes da Oceania é sorteado no dia do começo (+0xb6/+0xb8) e só na chamada sem `completo`: os 8 clubes, as posições previstas, o grupo A na tabela da própria copa, os jogos e o grupo B como fase; antes desse dia a copa nem passa o dia de liga; um erro na montagem devolve 1 sem o dia de liga', "rule: the Oceania Club Championship is drawn on the start day (+0xb6/+0xb8) and only on the call without `completo`: the 8 clubs, the predicted positions, group A in the cup's own table, the fixtures and group B as a stage; before that day the cup does not even run the league day; an error while building returns 1 without the league day"),
         ('cqt10_oceania_grupo_a', 'OceaniaClubChamp::create_group_a_table', '0x7978c0', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'regra: o grupo A do Campeonato de Clubes da Oceania tem os 4 primeiros escolhidos, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo', 'rule: group A of the Oceania Club Championship has the first 4 chosen clubs, each in the row of its order; human managers get the group draw news'),
         ('cqt10_oceania_grupo_b', 'OceaniaClubChamp::create_group_b_stage', '0x797e80', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'regra: o grupo B do Campeonato de Clubes da Oceania é uma fase de liga com os escolhidos 4 a 7, as regras e o ano da copa e as rodadas da tabela da copa para a fase 0 (10, 12 e 14 de janeiro); os técnicos humanos recebem a notícia', "rule: group B of the Oceania Club Championship is a league stage with chosen clubs 4 to 7, the cup's rules and year and the cup's round table for stage 0 (10, 12 and 14 January); human managers get the news")])
CAMPOS = [('copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)', 'cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('copa +0xee os 8 escolhidos (ponteiros de clube), +0xf2 a conta; ponteiro/valor; lê (grupos), libera e zera (dia)', 'cup +0xee the 8 chosen clubs (club pointers), +0xf2 their count; pointer/value; reads (groups), frees and clears (day)'),
          ('copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (grupo A)', 'cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (group A)'),
          ('copa +0xc fases ([0] o grupo B), +0x30 fase atual; ponteiro/valor; grava (grupo B)', 'cup +0xc stages ([0] group B), +0x30 current stage; pointer/value; writes (group B)')]
INTERESSA = ['CMRTE']
