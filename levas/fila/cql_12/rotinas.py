# O calendário das supercopas: o jogo da Supercopa da França, da Espanha e do Japão (o mesmo slot, a virtual +0x3c
# create_round_table; fra_super.cpp, spa_super.cpp, jap_super.cpp). Frente das competições (Agente D, 04/11/2026),
# carreira de 8 ligas (França, Espanha e Japão jogáveis).
# - 0x5bf5d0 (FraSuper::create_round_table, 407 bytes; 1 chamada até 2002-06-01 com as 8 ligas, no construtor): o jogo
#   da Supercopa da França; em 2001 o mando é 0x400, nos outros anos sorteado entre seis (0x90cfc0). Uma mutação.
# - 0x858960 (SpaSuper::create_round_table, 310 bytes; 1 chamada, no construtor): a ida e a volta da Supercopa da
#   Espanha. Uma mutação.
# - 0x666020 (JapSuper::create_round_table, 312 bytes; 2 chamadas): o jogo da Supercopa do Japão. Uma mutação.
# Ajudantes comuns (cql12_rodada, cql12_tabela, cql12_gravados, cql12_comparar; os da cql_10 com o prefixo do item e
# um `ajuste` da primeira rodada depois da alocação, para o mando sorteado da França). As três têm o mesmo corpo (o byte
# da fase, as saídas, a alocação de 0x68, a caixa) e só mudam na rodada, no arquivo/linha da caixa e no mando da
# França. Diferença de ordem: a original da França sorteia o mando entre a data do sorteio (0x521e60, que não sorteia)
# e a do jogo; a cópia sorteia logo antes das duas, depois da alocação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte). Dentro: 0x944e46 (cdecl), 0x521e60 (7 pushes; add
# esp, 0x1c -> cdecl), 0x521eb0 (9 pushes; add esp, 0x24 -> cdecl), 0x90cfc0 (push 6; add esp, 0x28 junto com a
# 0x521eb0 -> cdecl, 1 argumento; devolve 0..5, índice de uma tabela local de seis palavras com sinal) e a caixa
# (aviso_em).
# Comparadores: lado a lado (as saídas e os bytes gravados das duas tabelas; o [0xb67a34]); a tabela da cópia é
# liberada. Fora da fase −1, e na França fora de 2001 (sorteia), só a original.
# Determinismo: nada lê pilha, heap ou endereço como valor (a tabela local dos mandos é uma constante na cópia).
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cql12_supercopa_da_franca_tabela', 0x5bf5d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('cql12_supercopa_da_espanha_tabela', 0x858960, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql12_supercopa_do_japao_tabela', 0x666020, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das supercopas: o jogo da Supercopa da França, da Espanha e do Japão',
        'the super cups\' calendar: the French, Spanish and Japanese Super Cup matches',
        [('cql12_supercopa_da_franca_tabela', 'FraSuper::create_round_table', '0x5bf5d0', 'fra_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da França (Trophée des Champions) é um jogo único entre 2 clubes, sorteado em 5/7 e jogado em 24/7 (levado ao dia da semana 5), decidido nos pênaltis [inferência: desempate 1]; em 2001 no local 0x400, nos outros anos num de seis locais sorteados (0x200 a 0x4000: [inferência] os maiores estádios do país); 60 mil a cada clube por chegar; categoria 6',
          'rule: the French Super Cup (Trophée des Champions) is a single match between 2 clubs, drawn on 5/7 and played on 24/7 (moved to weekday 5), decided on penalties [inference: tiebreak 1]; in 2001 at venue 0x400, in other years at one of six drawn venues (0x200 to 0x4000: [inference] the country\'s largest grounds); 60 thousand to each club for taking part; category 6'),
         ('cql12_supercopa_da_espanha_tabela', 'SpaSuper::create_round_table', '0x858960', 'spa_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Espanha é disputada em ida e volta (7 dias entre os jogos) entre 2 clubes, sem sorteio (+0xd = 0x200: a ordem montada), sorteio em 5/7 e a ida em 20/8 (levada ao dia da semana 6) na casa do mandante [inferência: mando 0]; desempate de ida e volta 1 [inferência: gol fora e pênaltis]; sem prêmio; categoria 6',
          'rule: the Spanish Super Cup is played over two legs (7 days apart) between 2 clubs, without a draw (+0xd = 0x200: the built order), drawn on 5/7 with the first leg on 20/8 (moved to weekday 6) at the home side [inference: venue 0]; two-leg tiebreak 1 [inference: away goals and penalties]; no prize; category 6'),
         ('cql12_supercopa_do_japao_tabela', 'JapSuper::create_round_table', '0x666020', 'jap_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa do Japão (Xerox Super Cup) é um jogo único entre 2 clubes, sorteado em 1/2 e jogado em 4/3 (levado ao dia da semana 5) no estádio neutro [inferência: mando 4], decidido nos pênaltis [inferência: desempate 1]; 150 mil ao vencedor; categoria 6',
          'rule: the Japanese Super Cup (Xerox Super Cup) is a single match between 2 clubs, drawn on 1/2 and played on 4/3 (moved to weekday 5) at a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; 150 thousand to the winner; category 6')])
CAMPOS = [('copa +0x40 ano; valor; lê (as três; a da França compara com 2001)', 'cup +0x40 year; value; reads (all three; the French one compares with 2001)'),
          ('rodada de copa (0x68 bytes) +0..+6 datas, turno, +7 nome, +9 índice, +0xb/+0xd/+0xf sorteio e desempates, +0x13 mando, +0x17 categoria, +0x18..+0x1e times, jogos, novos, inscritos, +0x20..+0x22 replay, jogos por confronto, dias, +0x5c..+0x64 prêmios; valor; grava (as três)',
           'cup round (0x68 bytes) +0..+6 dates, time slot, +7 name, +9 index, +0xb/+0xd/+0xf draw and tiebreaks, +0x13 venue, +0x17 category, +0x18..+0x1e teams, matches, new, entrants, +0x20..+0x22 replay, legs, days, +0x5c..+0x64 prizes; value; writes (all three)'),
          ('[0xb67a34] grava 0 na caixa de erro', '[0xb67a34] writes 0 on the error box')]
INTERESSA = ['CMRTE']
