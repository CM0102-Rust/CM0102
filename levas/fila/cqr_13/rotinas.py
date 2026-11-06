# A Copa América: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros dois grupos (goldcup.cpp, em
# intercomp\). Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001; a Copa
# América de 2001, sorteada em 20 de março e jogada em julho, fica dentro da janela).
# - 0x5e2120 (GoldCup::update_for_date): o dia; antes do sorteio volta sem o dia de liga; no dia do sorteio a lista,
#   as previsões, as tabelas, os jogos e as 2 fases. Uma mutação (o sorteio uma semana depois).
# - 0x5e11e0 (GoldCup::create_group_table): a tabela do grupo A (os sorteados 0, 3, 6 e 9). Uma mutação (no ajudante
#   dos sorteados, que as duas leem: o 4º e o 5º trocam de grupo).
# - 0x5e16b0 (GoldCup::create_group_stage): as fases dos grupos B e C. Uma mutação (os times de cada fase de trás para
#   a frente).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): o dia é a virtual +0x14 (push completo; push hoje; mov ecx -> thiscall, ret
# 8); a tabela é thiscall, ret 0 (mov ecx, esi; call, do dia); a fase thiscall, ret 4 (push g; mov ecx, do dia).
# Dentro: 0x5e1cd0 (mov ecx -> thiscall, ret 0), 0x6835c0 (thiscall, ret 0), 0x6827d0 (push 0; mov ecx -> thiscall,
# ret 4), as virtuais +0x5c e +0x3c (thiscall, ret 0 e ret 0x10), 0x944e46 e 0x944cf1 (push; add esp, 4 -> cdecl),
# 0x9452ca (push; add esp -> cdecl), 0x687430 (4 pushes; mov ecx -> thiscall, ret 0x10), 0x684230 (mov ecx ->
# thiscall, ret 0), 0x68b100 (22 pushes; mov ecx, novo -> thiscall, ret 0x58), 0x6847c0 (2 pushes; mov ecx ->
# thiscall, ret 8) e a caixa (aviso_em); a 0x549ef0 (a data do sorteio, pura) fica de fora: a cópia compara os campos.
# Comparadores: só a original (as três alocam, sorteiam ou mandam notícia), com o plano: o dia confere no dia do
# sorteio +0x45, a lista e +0xf2 zeradas, a tabela e as fases; a tabela confere as 4 linhas na ordem dos sorteados 3i;
# a fase confere a fase, +0x30, os times, o ano e as 4 seleções da tabela dela.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (06/11/2026, cópia fora do projeto em %TEMP%: motor.rs da principal com a 280a publicada e a 281a integrada,
# 1.986 rotinas, + os 12 itens cqr_10..13, cqs_10..13, cqt_10..13, 36 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 só com os 4 cqr (parou em 2001-273) e até
#   2002-06-01 com os 12 itens (parou em 2002-151; as copas de janeiro e fevereiro de 2002 e a Libertadores de 2002
#   rodam depois de outubro): 0 linhas DIVERGENCIA, 0 EXCECAO, nenhuma caixa (data, motor e aleatorio: 0 divergências);
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a (a mesma da base sem os itens: as cópias são
#   fiéis); todos os contadores das mutações rodaram;
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS (o sorteio
#   uma semana depois em 2001-075, o 4º e o 5º trocados em 2001-171, os times de trás para a frente em 2001-187).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr13_copa_america_dia', 0x5e2120, [0x6a, 0xff, 0x68, 0xe6, 0xa8, 0x95, 0x00]),
           ('cqr13_copa_america_primeiro_grupo', 0x5e11e0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqr13_copa_america_grupo', 0x5e16b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Copa América: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros dois grupos',
        'the Copa América: the draw day, the first group table and the other two groups\' stages',
        [('cqr13_copa_america_dia', 'GoldCup::update_for_date', '0x5e2120', 'goldcup.cpp', 'thiscall, ret 8',
          'regra: a Copa América tem 12 seleções em 3 grupos de 4 (turno único, 3/1 pontos, dois classificados por grupo e os dois melhores terceiros para as quartas); o sorteio é no dia do começo (20 de março do ano da copa, levado ao dia da semana 5) e só na chamada sem `completo`: a lista das 12, as posições previstas, o grupo A na tabela da própria copa, os jogos e os grupos B e C como fases; antes desse dia a copa nem passa o dia de liga',
          'rule: the Copa América has 12 nations in 3 groups of 4 (single round, 3/1 points, two qualify from each group plus the two best thirds to the quarter-finals); the draw is on the start day (20 March of the cup year, moved to weekday 5) and only on the call without `completo`: the list of 12, the predicted positions, group A in the cup\'s own table, the fixtures and groups B and C as stages; before that day the cup does not even run the league day'),
         ('cqr13_copa_america_primeiro_grupo', 'GoldCup::create_group_table', '0x5e11e0', 'goldcup.cpp', 'thiscall, ret 0',
          'regra: o grupo A da Copa América tem 4 seleções, os sorteados 0, 3, 6 e 9 (os grupos são intercalados na lista), cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo',
          'rule: group A of the Copa América has 4 nations, drawn entries 0, 3, 6 and 9 (the groups are interleaved in the list), each in the row of its order; human managers get the group draw news'),
         ('cqr13_copa_america_grupo', 'GoldCup::create_group_stage', '0x5e16b0', 'goldcup.cpp', 'thiscall, ret 4',
          'regra: os grupos B e C da Copa América são fases de liga com os sorteados g + 1, g + 4, g + 7 e g + 10, as regras e o ano da copa e as rodadas da tabela da copa para o grupo (6, 10 e 13 de julho); os técnicos humanos recebem a notícia',
          'rule: groups B and C of the Copa América are league stages with drawn entries g + 1, g + 4, g + 7 and g + 10, the cup\'s rules and year and the cup\'s round table for the group (6, 10 and 13 July); human managers get the news')])
CAMPOS = [('copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)',
           'cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)'),
          ('copa +0xee lista dos sorteados (ponteiros de clube da seleção), +0xf2 a conta; ponteiro/valor; lê (tabela e fases), libera e zera (dia)',
           'cup +0xee drawn list (national club pointers), +0xf2 its count; pointer/value; reads (table and stages), frees and clears (day)'),
          ('copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)',
           'cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)'),
          ('copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)',
           'cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)')]
INTERESSA = ['CMRTE']
