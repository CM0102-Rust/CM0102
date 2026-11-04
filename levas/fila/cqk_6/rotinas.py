# As tabelas da CFA e da divisão de baixo francesa e a situação nas estatísticas da Série C1 italiana (Agente B da
# frente das competições, 03/11/2026):
# - 0x5b9fa0 (FraCfa::create_group_table, 256 bytes): a tabela de um grupo de 18 da CFA; 4 vezes, na criação da
#   carreira de 8 ligas (o construtor 0x5b91c0: push ebx (a lista); mov ecx, esi; call -> thiscall, ret 4);
# - 0x5be230 (FraLower::create_table, 244 bytes): a tabela de 20 da divisão de baixo francesa; 2 vezes (o construtor
#   0x5bd600: push ebx; mov ecx, esi; call -> thiscall, ret 4);
# - 0x64bd80 e 0x64ff40 (ItaSerC1a/ItaSerC1b::set_club_progress, 202 bytes cada, iguais): a virtual +0x68 da
#   0x684cd0 a cada linha da tabela (thiscall, ret 0x14; ~3.000 chamadas cada).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n × 0x3b; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push marca (0 ou 1); push clube; push ecx (cl = a linha, os bits de
#   cima do id da competição); mov ecx, esi -> thiscall, ret 0x10 (a linha limpa: só o byte é lido);
# - 0x4a4850 (Comp_Stats::find_club_entry): push clube; mov ecx, [ecx+8] -> thiscall, ret 4;
# - a caixa de erro: aviso_em. As tabelas de salto da C1 (0x64be4c/0x64be58/0x64be68/0x64be74 e 0x65000c/...) foram
#   lidas: 2..5 → a primeira faixa, 6..13 como veio, 14..17 → a segunda.
# Comparadores: as tabelas, só a original (alocam) com o plano (os clubes da lista na ordem, a marca +0x38, +0x3e e a
# devolução); a situação, lado a lado nos 3 bytes da entrada (achada antes).
# Determinismo: nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('tabela_do_grupo_da_cfa', 0x5b9fa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tabela_da_divisao_de_baixo_francesa', 0x5be230, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('situacao_na_c1a', 0x64bd80, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('situacao_na_c1b', 0x64ff40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as tabelas da CFA e da divisão de baixo francesa; a situação nas estatísticas da Série C1 italiana',
 'the tables of the French CFA and lower division; the progress in the stats of Italian Serie C1',
 [('tabela_do_grupo_da_cfa', 'FraCfa::create_group_table', '0x5b9fa0', 'fra_cfa.cpp', 'thiscall, ret 4',
   'um grupo da CFA tem 18 clubes (4 grupos, sorteados pelo construtor); os clubes que estão na primeira ou na '
   'segunda divisão levam a marca da linha [inferência: os times reserva dos profissionais, que não sobem]',
   'a CFA group has 18 clubs (4 groups, drawn by the constructor); clubs that are in the first or second division '
   'get the row mark [inference: the reserve sides of professional clubs, which cannot go up]'),
  ('tabela_da_divisao_de_baixo_francesa', 'FraLower::create_table', '0x5be230', 'fra_lower.cpp', 'thiscall, ret 4',
   'a divisão de baixo francesa tem 20 clubes (escolhidos pelo construtor); os que estão no National levam a marca',
   'the French lower division has 20 clubs (chosen by the constructor); those in the National get the mark'),
  ('situacao_na_c1a', 'ItaSerC1a::set_club_progress', '0x64bd80', 'ita_ser_c1a.cpp', 'thiscall, ret 0x14',
   'a previsão de posição do clube na C1/A: na liga, a melhor e a pior posição viram faixas (2 a 5 = o play-off de '
   'acesso: a melhor 2, a pior 5; 14 a 17 = o play-out: a melhor 14, a pior 17); no play-off, as posições + 1; no '
   'play-out, quem vence fica em 14 e quem perde em 16',
   'the club\'s place forecast in C1/A: in the league, best and worst place become bands (2 to 5 = the promotion '
   'play-off: best 2, worst 5; 14 to 17 = the play-out: best 14, worst 17); in the play-off, places + 1; in the '
   'play-out, the winner stays at 14 and the loser at 16'),
  ('situacao_na_c1b', 'ItaSerC1b::set_club_progress', '0x64ff40', 'ita_ser_c1b.cpp', 'thiscall, ret 0x14',
   'a mesma regra na C1/B', 'the same rule in C1/B')])
CAMPOS = [('linha da tabela +0 clube, +0x38 marca (1 = clube de divisão maior); ponteiro e valor; grava pela 0x687430 '
           '(as duas tabelas)',
           'table row +0 club, +0x38 mark (1 = club of a higher division); pointer and value; writes through '
           '0x687430 (both tables)'),
          ('clube +0x57 competição (id no +0); ponteiro; lê (as duas tabelas)',
           'club +0x57 competition (id at +0); pointer; reads (both tables)'),
          ('Comp_Stats entrada do clube +0x73 posição atual, +0x74 melhor, +0x75 pior; valor; grava (situacao_na_c1a, '
           'situacao_na_c1b)',
           'Comp_Stats club entry +0x73 current place, +0x74 best, +0x75 worst; value; writes (situacao_na_c1a, '
           'situacao_na_c1b)')]
INTERESSA = ['CMRTE']
