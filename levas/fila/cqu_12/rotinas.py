# A eliminatória da Oceania para a Copa do Mundo de 2002: o dia do começo, a tabela do primeiro grupo e a fase do
# segundo (wc_oceania_league.cpp, em worldcup\). Frente das competições (Agente E, 06/11/2026, noite), carreira de 8
# ligas (a Austrália jogável; as seleções da Oceania têm jogadores das ligas inglesa e australiana).
# - 0x91f3d0 (WcOceaniaLeague::update_for_date): o dia; o jogo que começa em 2001 monta a eliminatória no primeiro
#   dia completo. Uma mutação (só no dia do começo).
# - 0x91ec00 (WcOceaniaLeague::init_group_table): o primeiro grupo (5 seleções). Uma mutação (4 seleções).
# - 0x91f1a0 (WcOceaniaLeague::create_second_group_stage): o segundo grupo. Uma mutação (de trás para a frente).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); as outras duas thiscall, ret 0 (mov ecx, esi; call). Dentro: 0x549ef0 (push ano; push dia; lea ecx -> thiscall,
# ret 8), 0x775070 (push 0; push continente; mov ecx -> thiscall, ret 8), 0x775220 (push comp; push nação; mov ecx ->
# thiscall, ret 8), 0x6835c0 e 0x684230 (mov ecx -> thiscall, ret 0), 0x6827d0 (push 0; mov ecx -> thiscall, ret 4),
# 0x6847c0 (2 pushes; mov ecx -> thiscall, ret 8), 0x944e46, 0x944cf1 e 0x9452ca (push; add esp, 4 -> cdecl), 0x687430
# (4 pushes; mov ecx -> thiscall, ret 0x10), a virtual +0x3c (4 pushes, ret 0x10) e 0x68b100 (22 pushes; mov ecx, novo
# -> thiscall, ret 0x58).
# Comparadores: só a original, com o plano: o dia confere na montagem +0x45, a fase e +0x30; o primeiro grupo as 5
# linhas na ordem; o segundo a fase, +0x30, os times, o ano e as seleções da tabela dela.
# Determinismo: nada lê pilha, heap ou endereço como valor (o vetor local {1, 0, 0, 0} do construtor é montado igual).
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('wc_oceania_league_update_for_date', 0x91f3d0, [0x6a, 0xff, 0x68, 0x46, 0x5b, 0x96, 0x00]),
           ('wc_oceania_league_init_group_table', 0x91ec00, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('wc_oceania_league_create_second_group_stage', 0x91f1a0, [0x6a, 0xff, 0x68, 0x1e, 0x5b, 0x96, 0x00])]
DOCS = ('a eliminatória da Oceania para a Copa do Mundo: o dia do começo, a tabela do primeiro grupo e a fase do segundo',
        'the Oceania World Cup qualifying: the start day, the first group table and the second group stage',
        [('wc_oceania_league_update_for_date', 'WcOceaniaLeague::update_for_date', '0x91f3d0', 'wc_oceania_league.cpp', 'thiscall, ret 8', 'regra: a eliminatória da Oceania se monta no dia do começo ou, num jogo que começa em 2001 (com a eliminatória de 2002 em curso), no primeiro dia completo: as seleções da Oceania deixam as competições do continente e entram na eliminatória (as do primeiro grupo e as da lista), os jogos e a tabela ordenada, e o segundo grupo vira fase; antes do começo a eliminatória nem passa o dia de liga', 'rule: the Oceania qualifying is set up on the start day or, in a game starting in 2001 (with the 2002 qualifying under way), on the first full day: the Oceania nations leave the continental competitions and enter the qualifying (those of the first group and of the list), the fixtures and the sorted table, and the second group becomes a stage; before the start the qualifying does not even run the league day'),
         ('wc_oceania_league_init_group_table', 'WcOceaniaLeague::init_group_table', '0x91ec00', 'wc_oceania_league.cpp', 'thiscall, ret 0', 'regra: o primeiro grupo da eliminatória da Oceania tem 5 seleções, as das vagas pares da lista (0, 2, 4, 6, 8), cada uma na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio', 'rule: the first Oceania qualifying group has 5 nations, those of the even places of the list (0, 2, 4, 6, 8), each in the row of its order; human managers get the draw news'),
         ('wc_oceania_league_create_second_group_stage', 'WcOceaniaLeague::create_second_group_stage', '0x91f1a0', 'wc_oceania_league.cpp', 'thiscall, ret 0', 'regra: o segundo grupo da eliminatória da Oceania é uma fase de liga com as seleções das vagas ímpares da lista (4, ou 5 se o byte +0xee não é 1), as regras e o ano da eliminatória e as rodadas da fase 0; os técnicos humanos recebem a notícia', 'rule: the second Oceania qualifying group is a league stage with the nations of the odd places of the list (4, or 5 when byte +0xee is not 1), the qualifying rules and year and the stage 0 rounds; human managers get the news')])
CAMPOS = [('eliminatória +0x45 montada, +0xb6/+0xb8 dia e ano relativo do começo, +0x40 ano; valor; lê e grava +0x45 (dia)', 'qualifying +0x45 set up, +0xb6/+0xb8 start day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('eliminatória +0xef a lista das seleções (ponteiros de clube de seleção), +0xf3 a conta, +0xee byte do tamanho do segundo grupo; ponteiro/valor; lê', 'qualifying +0xef the nations list (national team club pointers), +0xf3 its count, +0xee second group size byte; pointer/value; reads'),
          ('eliminatória +0x3e times, +0xb1 tabela (linhas de 0x3b bytes); valor/ponteiro; grava (primeiro grupo), lê (dia)', 'qualifying +0x3e teams, +0xb1 table (0x3b-byte rows); value/pointer; writes (first group), reads (day)'),
          ('eliminatória +0xc fases ([0] o segundo grupo), +0x30 fase atual; ponteiro/valor; grava (segundo grupo)', 'qualifying +0xc stages ([0] the second group), +0x30 current stage; pointer/value; writes (second group)')]
INTERESSA = ['CMRTE']
