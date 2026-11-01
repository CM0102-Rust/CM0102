# A renda das partidas (fin_prm.cpp; os asserts do código das finanças citam finance.CPP): FinanceTable::process_match_income
# (0x59a5d0, 0x96e bytes com x87, pelo fechamento da partida do dia 0x6b5f00, a cada jogo de clubes),
# FinanceTable::calc_gate_receipts (0x59af40, 0x35a bytes com x87, pelo registro da partida 0x729ff0: a renda do estádio
# que vai ao +0x38 do resultado e daí ao líquido do mandante na 0x59a5d0) e, passageira, Finance::pay_league_fine
# (0x599850, 0x34 bytes, pelas multas da liga 0x49f570 e 0x6a9e60). Sem SEH. Iguais no cm0102-1080.exe (corpo inteiro
# das três). O x87 do jogo roda em precisão dupla: as contas em f64 na mesma ordem (fild/fmul/fimul/fadd), o _ftol
# (0x945640) pelo `ftol` do motor.rs; o primeiro sorteio dos gastos guardado em float (fstp dword) como na original.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x53b900 (e_selecao): push clube; call; add esp, 4 -> cdecl(clube).
# - 0x540a50 (clube_parceiro): push 1; push &local; push clube; call; add esp, 0xc -> cdecl(clube, &a, 1).
# - 0x90cfc0 (sorteio): push n; call; add esp, 4 -> cdecl(n) (n = 10, 0xc8, 0x190, 0xfa, ftol(reputação × 0,25), 0xf).
# - 0x945640 (_ftol): sem argumento na pilha (st0) -> a função `ftol` do motor.rs.
# - 0x5c6710 (Friendly::is_friendly_competition): mov ecx, [0xae2c74]; push comp; sem add esp -> thiscall, ret 4.
# Comparadores: a 0x59a5d0 lado a lado nos dois registros de finanças (0x167 bytes) e no estado do sorteio; a 0x59af40
# pela devolução (só lê); a 0x599850 lado a lado no registro de finanças.
SAVE = False
ROTINAS = [('renda_da_partida', 0x59a5d0, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('renda_do_estadio', 0x59af40, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('pagar_multa_da_liga', 0x599850, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x28, 0x01, 0x00, 0x00])]
DOCS = ('a renda das partidas: a renda da partida nas finanças dos dois clubes, a renda do estádio e a multa da liga',
        'the match income: the match income in both clubs\' finances, the gate receipts and the league fine',
        [('renda_da_partida', 'FinanceTable::process_match_income', '0x59a5d0', 'fin_prm.cpp', 'thiscall, ret 8',
          'a TV pelo público (1,75 × público), a renda do estádio (+0x38 do resultado), os custos do jogo e os gastos do mandante (com sorteios), o dinheiro da TV da competição pela fase ou pelo tipo, divididos pelo tipo da nação; na liga do mandante, a TV 3/4 e 1/4, o líquido ao mandante e a manutenção do estádio; fora dela, metade de cada um para os dois',
          'the TV money by attendance (1.75 × attendance), the gate receipts (+0x38 of the result), the match costs and the home club expenses (with draws), the competition TV money by stage or type, divided by the nation type; in the home club league, TV 3/4 and 1/4, the net to the home club and the stadium upkeep; outside it, half of each to both'),
         ('renda_do_estadio', 'FinanceTable::calc_gate_receipts', '0x59af40', 'fin_prm.cpp', 'thiscall, ret 8',
          'a renda do estádio: o preço pela reputação do mandante (/ 500; × 0,5 em amistoso, + 3 ou + 4 nas competições) × público, menos o desconto das finanças na liga do mandante; em campo neutro pelo +0x69 da competição; ajustada pelo tipo da nação, pela liga, por clubes e pela nação de 0x9cf314',
          'the gate receipts: the price by the home club reputation (/ 500; × 0.5 in friendlies, + 3 or + 4 in competitions) × attendance, less the finance discount in the home club league; at a neutral ground by the competition +0x69; adjusted by nation type, league, key clubs and the nation at 0x9cf314'),
         ('pagar_multa_da_liga', 'Finance::pay_league_fine', '0x599850', 'fin_prm.cpp', 'thiscall, ret 4',
          'soma a multa às multas do ano (+0x128) e do mês (+0x88) e a tira do saldo',
          'adds the fine to the season (+0x128) and month (+0x88) fines and takes it from the balance')])
CAMPOS = [('finanças do clube (0x167 bytes) +0: o saldo (i64); valor; soma e tira (as três)',
           'club finances (0x167 bytes) +0: the balance (i64); value; adds and subtracts (all three)'),
          ('finanças +0x158 / +0xb8: a TV do ano e do mês; +0x140 / +0xa0: a renda líquida das partidas do ano e do mês; +0x12c / +0x8c: a manutenção do estádio do ano e do mês; valor; soma (renda_da_partida)',
           'finances +0x158 / +0xb8: season and month TV money; +0x140 / +0xa0: season and month net match income; +0x12c / +0x8c: season and month stadium upkeep; value; adds (renda_da_partida)'),
          ('finanças +0x128 / +0x88: as multas do ano e do mês; valor; soma (pagar_multa_da_liga)',
           'finances +0x128 / +0x88: season and month fines; value; adds (pagar_multa_da_liga)'),
          ('finanças +0x160: o desconto do ingresso (float); valor; lê (renda_do_estadio)',
           'finances +0x160: the ticket discount (float); value; reads (renda_do_estadio)'),
          ('partida +0x14 a competição (+0x69 palavra), +0x18 o estádio (+0x3c capacidade), +0x1c / +0x20 os clubes, +0x3a o tipo, +0x3b a fase, +0x3c campo neutro, +0x40 o piso de custo; ponteiro e valor; lê (renda_da_partida, renda_do_estadio)',
           'fixture +0x14 the competition (+0x69 word), +0x18 the stadium (+0x3c capacity), +0x1c / +0x20 the clubs, +0x3a the type, +0x3b the stage, +0x3c neutral ground, +0x40 the cost floor; pointer and value; reads (renda_da_partida, renda_do_estadio)'),
          ('resultado +0x34 o público, +0x38 a renda do estádio; valor; lê (renda_da_partida)',
           'result +0x34 the attendance, +0x38 the gate receipts; value; reads (renda_da_partida)'),
          ('clube +0x53 a nação (+0x7e o tipo), +0x57 a liga, +0x64 a marca (1: custos, gastos e preço pelas faixas), +0x69 / +0x6d o estádio próprio (+0x3c), +0x80 a reputação; ponteiro e valor; lê (renda_da_partida, renda_do_estadio)',
           'club +0x53 the nation (+0x7e the type), +0x57 the league, +0x64 the flag (1: costs, expenses and price by bands), +0x69 / +0x6d the own stadium (+0x3c), +0x80 the reputation; pointer and value; reads (renda_da_partida, renda_do_estadio)')]
INTERESSA = ['CMRTE']
