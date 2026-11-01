# O fim de temporada das finanças (fin_prm.cpp; os asserts citam finance.CPP): FinanceTable::end_season_for_nation
# (0x59cdd0, 0x17b bytes, pelo laço das temporadas 0x5d5ae0 no dia anterior ao início da temporada de cada nação-chave:
# a 0x447880 da mesma ponte rodou 40 vezes na carreira até 2002-06-01, docs/cobertura_simulacao.txt),
# FinanceTable::season_end_club_update (0x59cfa0, 0xd97 bytes com x87, para cada clube da liga da nação: compra do
# estádio, aperto, centro de treino, aquecimento, coberto, expansão e lugares sentados, pedidos do humano e o
# dividendo, que sai do saldo de quase todo clube) e, passageira, FinanceTable::news_dividend (0x59fe10, 0x9b bytes,
# quadro SEH só da notícia local, sem destrutor no caminho normal). Iguais no cm0102-1080.exe (corpo inteiro das três).
# O x87 do jogo roda em precisão dupla: saldo (a palavra baixa, fild dword) × 0,2 / × 0,1 pelo _ftol (0x945640, a
# `ftol` do motor.rs) e a comparação capacidade × 0,92 < média de público (fcompp) em f64.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x456770 (data depois de hoje): mov ecx, 0xae2c90; push [0xdd6ecc]; push [0xdd6ec8]; sem add esp -> thiscall, ret 8.
# - 0x53b900 (e_selecao): push clube; call; add esp, 4 -> cdecl (nas duas).
# - 0x669360 (linha da nação do clube): push clube; add esp, 4 -> cdecl; o al com movsx.
# - 0x59cfa0 (pela 0x59cdd0): mov ecx, esi (tabela); push clube; sem add esp -> thiscall, ret 4.
# - 0x540a50 (clube_parceiro): push 1; push &local; push clube; add esp, 0xc -> cdecl(clube, &a, 1).
# - 0x600a50 (clube_humano): 5 pushes (0, 0, 1, 0, clube); add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0).
# - 0x90cfc0 (sorteio): push n; add esp, 4 -> cdecl (0x28, 0x14, 0x1f5).
# - 0x945640 (_ftol): st0, sem pilha -> `ftol`.
# - 0x4a4850 (linha do clube na tabela da liga): mov ecx, [comp + 8]; push clube; sem add esp -> thiscall, ret 4.
# - 0x59b630 (melhorar o centro de treino, já em Rust): mov ecx, tabela; push clube; sem add esp -> thiscall, ret 4.
# - 0x59ffb0 (notícia das obras): mov ecx, tabela; push tipo; push valor; push clube; sem add esp -> thiscall, ret 0xc.
# - 0x59fae0 (notícia da recusa): mov ecx, tabela; push motivo; push assunto; push clube; sem add esp -> thiscall, ret 0xc.
# - 0x59fe10 (notícia do dividendo): mov ecx, tabela; push valor; push clube; sem add esp -> thiscall, ret 8.
# - Na 0x59fe10: 0x77bbe0 (lea ecx, local; push 0; push 0x177c -> thiscall, ret 8), 0x785790 (lea ecx, local; push
#   valor; push índice -> thiscall, ret 8; três vezes) e 0x7861e0 (mov ecx, [0xb7431c]; push clube; push &local ->
#   thiscall, ret 8).
# Comparadores: a 0x59cfa0 lado a lado (finanças do parceiro e do próprio clube, clube, estádio de 78 bytes, linha dos
# pedidos de 0x49 bytes, estado do sorteio; a cópia em sem_contar), só a original com técnico humano ou na compra do
# estádio sem dono (mexe no dono); a 0x59cdd0 só a original (chama a 0x59cfa0 de cada clube), conferindo os totais do
# ano zerados no último clube da entrada; a 0x59fe10 só a original (entrega a notícia ao humano).
SAVE = False
ROTINAS = [('fim_de_temporada_da_nacao', 0x59cdd0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('fim_de_temporada_do_clube', 0x59cfa0, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('noticia_do_dividendo', 0x59fe10, [0x6a, 0xff, 0x68, 0x2b, 0x91, 0x95, 0x00])]
DOCS = ('o fim de temporada das finanças: o fim de temporada da nação, as obras, o centro de treino e o dividendo de cada clube e a notícia do dividendo',
        'the finance season end: the nation season end, each club\'s works, training facilities and dividend, and the dividend news',
        [('fim_de_temporada_da_nacao', 'FinanceTable::end_season_for_nation', '0x59cdd0', 'fin_prm.cpp', 'thiscall, ret 4',
          'depois da data de início do jogo, para cada clube com liga e nação ativa ou reputação de 5000 cuja linha é a nação-chave dada: o fim de temporada do clube e os totais do ano (+0x110) passam para os do ano anterior (+0xc0) e zeram; para no primeiro clube que for seleção',
          'after the game start date, for each club with a league and an active nation or reputation 5000 whose row is the given key nation: the club season end, and the season totals (+0x110) move to last season (+0xc0) and are cleared; stops at the first national team club'),
         ('fim_de_temporada_do_clube', 'FinanceTable::season_end_club_update', '0x59cfa0', 'fin_prm.cpp', 'thiscall, ret 4',
          'compra do estádio (20 milhões, do dono em crise), o aperto, o centro de treino do clube da máquina pelo sorteio, o aquecimento (1,5 milhão) e o coberto (7,5 milhões), a expansão ou os lugares sentados pelo saldo e pelo público médio, os pedidos do humano à diretoria (estádio e treino) e o dividendo pela reputação, pelo saldo e pelo presidente, com sorteio',
          'ground purchase (20 million, from an owner in crisis), the squeeze, the AI club training facilities by draw, undersoil heating (1.5 million) and roof (7.5 million), expansion or seating by balance and average attendance, the human\'s board requests (stadium and training) and the dividend by reputation, balance and chairman, with a draw'),
         ('noticia_do_dividendo', 'FinanceTable::news_dividend', '0x59fe10', 'fin_prm.cpp', 'thiscall, ret 8',
          'a notícia 0x177c com o id do clube, o valor e a marca +0x82, entregue ao técnico do clube',
          'news 0x177c with the club id, the amount and the +0x82 flag, sent to the club manager')])
CAMPOS = [('finanças do clube (0x167 bytes) +0: o saldo (i64; a palavra baixa como i32 nas decisões); valor; lê, soma e tira (fim_de_temporada_do_clube)',
           'club finances (0x167 bytes) +0: the balance (i64; the low dword as i32 in the decisions); value; reads, adds and subtracts (fim_de_temporada_do_clube)'),
          ('finanças +0x8c / +0x12c: as obras do mês e do ano; +0x94 / +0x134: o dividendo do mês e do ano; +0xb4 / +0x154: as entradas do mês e do ano (o dono que vende o estádio); valor; soma (fim_de_temporada_do_clube)',
           'finances +0x8c / +0x12c: month and season works; +0x94 / +0x134: month and season dividend; +0xb4 / +0x154: month and season income (the owner selling the ground); value; adds (fim_de_temporada_do_clube)'),
          ('finanças +0x110..+0x160: os totais do ano, copiados para +0xc0..+0x110 (ano anterior) e zerados; valor; grava (fim_de_temporada_da_nacao)',
           'finances +0x110..+0x160: the season totals, copied to +0xc0..+0x110 (last season) and cleared; value; writes (fim_de_temporada_da_nacao)'),
          ('finanças +0x165: o estado (1 e 2: crise; o dono que vende); +0x166: o aperto (acima de 10 cobra); valor; lê (fim_de_temporada_do_clube)',
           'finances +0x165: the state (1 and 2: crisis; the owner sells); +0x166: the squeeze (above 10 charges); value; reads (fim_de_temporada_do_clube)'),
          ('clube +0x53 a nação (+0x11c ativa), +0x57 a liga (+0x69 o tamanho), +0x64 a marca (expansão por saldo × 0,1), +0x69 o estádio, +0x6d dono do estádio, +0x7f o centro de treino, +0x80 a reputação, +0x82 a marca do dividendo, +0xbf o presidente (+0x59, +0x69 → +0x20), +0xcf o técnico (+0x69 → +0x22); ponteiro e valor; lê, e grava +0x6d e +0x7f (fim_de_temporada_do_clube, fim_de_temporada_da_nacao)',
           'club +0x53 the nation (+0x11c active), +0x57 the league (+0x69 its size), +0x64 the flag (expansion by balance × 0.1), +0x69 the stadium, +0x6d ground owner, +0x7f training facilities, +0x80 reputation, +0x82 the dividend flag, +0xbf the chairman (+0x59, +0x69 → +0x20), +0xcf the manager (+0x69 → +0x22); pointer and value; reads, and writes +0x6d and +0x7f (fim_de_temporada_do_clube, fim_de_temporada_da_nacao)'),
          ('estádio (78 bytes) +0 o id, +0x3c a capacidade, +0x40 os lugares sentados, +0x44 o teto da expansão, +0x4c o coberto, +0x4d o aquecimento; valor; lê e grava (fim_de_temporada_do_clube)',
           'stadium (78 bytes) +0 the id, +0x3c capacity, +0x40 seats, +0x44 expansion ceiling, +0x4c roof, +0x4d undersoil heating; value; reads and writes (fim_de_temporada_do_clube)'),
          ('[[0xb67a44]] + id × 0x49 + 2: os pedidos do humano à diretoria (bit 6 estádio, bit 7 centro de treino); valor; lê e apaga (fim_de_temporada_do_clube)',
           '[[0xb67a44]] + id × 0x49 + 2: the human\'s board requests (bit 6 stadium, bit 7 training facilities); value; reads and clears (fim_de_temporada_do_clube)'),
          ('[0xdd6ec8] a data de início do jogo; [0xb63d60] as nações-chave (72 bytes); valor; lê (fim_de_temporada_da_nacao)',
           '[0xdd6ec8] the game start date; [0xb63d60] the key nations (72 bytes); value; reads (fim_de_temporada_da_nacao)')]
INTERESSA = ['CMRTE', 'SGE']
