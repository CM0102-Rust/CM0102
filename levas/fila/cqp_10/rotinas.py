# O calendário de mais copas de clubes: a Copa dos Campeões, a Merconorte e as Supercopas da Europa e da Ásia (o slot
# +0x3c create_schedule/create_round_table; bra_champ_cup.cpp, con_merc_cup.cpp, eur_super_cup.cpp,
# asia_super_cup.cpp). Frente das competições (Agente D, 05-06/11/2026), carreira de 8 ligas (começa em janeiro de
# 2001).
# - 0x425c10 (BraChampCup::create_schedule, 952 bytes; 5 chamadas até 2002-06-01 com as 8 ligas): a preliminar, os
#   grupos e o mata-mata da Copa dos Campeões (passageira: as mutações escapam, ver as provas).
# - 0x4c1960 (ConMercCup::create_schedule, 857 bytes; 7 chamadas): os grupos e o mata-mata da Merconorte. Uma mutação.
# - 0x57a3c0 (EurSuperCup::create_round_table, 311 bytes; 1 chamada): o jogo da Supercopa Europeia. Uma mutação.
# - 0x410910 (AsiaSuperCup::create_round_table, 308 bytes; 1 chamada): a Supercopa Asiática. Uma mutação.
# Reaproveita do motor.rs os ajudantes da 276a (cql10_rodada, cql10_tabela, cql10_gravados, cql10_comparar, Cql10R); as
# fases de grupo (rodadas de 0x41 bytes pela 0x68a160 e 0x68a1c0) e a escolha da fase são o cqp10_tabela (a fase de cada
# copa decide entre nada, grupos com o seu extra e mata-mata). As tabelas foram extraídas da desmontagem.
# Iguais no cm0102-1080.exe (as quatro inteiras).
# Convenções conferidas (lição da 258a): as quatro são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte com sinal). Dentro: 0x944e46 (push tamanho; add
# esp, 4 -> cdecl), 0x68a160 (9 pushes -> cdecl; o ano num registrador com lixo nos 16 bits altos, a 0x68a160 usa só a
# palavra: a cópia passa limpo), 0x68a1c0 (7 pushes -> cdecl), 0x521e60/0x521eb0 pela cql10_rodada e a caixa
# (aviso_em).
# Comparadores: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais; o
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é liberada. Na fase
# sem tabela só a original (nenhuma grava nada).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqp10_copa_dos_campeoes_tabela', 0x425c10, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp10_merconorte_tabela', 0x4c1960, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp10_supercopa_europeia_tabela', 0x57a3c0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp10_supercopa_asiatica_tabela', 0x410910, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário de mais copas de clubes: a Copa dos Campeões, a Merconorte e as Supercopas da Europa e da Ásia',
        'the calendar of more club cups: the Copa dos Campeões, the Copa Merconorte and the European and Asian Super Cups',
        [('cqp10_copa_dos_campeoes_tabela', 'BraChampCup::create_schedule', '0x425c10', 'bra_champ_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa dos Campeões tem uma preliminar (fase −1, id 0xe6) de três rodadas às segundas [inferência: dia da semana 1], 19 e 26 de junho e 3 de julho; depois dois grupos (fases 0 e 1, id 0x3fd + fase) de três rodadas, 9, 16 e 20 de julho; o mata-mata (fase 2 em diante): semifinal sorteada em 21/7 e jogada em 23/7, volta 2 dias depois, e final sorteio 26/7, jogo em 27/7, volta 3 dias depois; categoria 6; desempate de ida e volta 1; prêmio da final 800 mil ao campeão e 400 mil ao vice',
          'rule: the Copa dos Campeões has a preliminary stage (stage −1, id 0xe6) of three rounds on Mondays [inference: weekday 1], 19 and 26 June and 3 July; then two groups (stages 0 and 1, id 0x3fd + stage) of three rounds, 9, 16 and 20 July; the knockout (stage 2 on): semi-final drawn on 21/7 and played on 23/7, return 2 days later, and final drawn on 26/7, match on 27/7, return 3 days later; category 6; two-leg tie-break 1; final prize 800 thousand to the winner and 400 thousand to the runner-up'),
         ('cqp10_merconorte_tabela', 'ConMercCup::create_schedule', '0x4c1960', 'con_merc_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Merconorte começa por grupos (fases abaixo de 3, id 0x3e9 + fase: 4 grupos [inferência]) de seis rodadas às terças de 15 em 15 dias, de 9/9 a 18/11; o mata-mata (fase 3): semifinal sorteada em 19/11 e jogada em 25/11 em ida e volta (7 dias), final sorteio 3/12 e jogo único em 9/12; categoria 8; desempate 3',
          'rule: the Copa Merconorte starts with groups (stages below 3, id 0x3e9 + stage: 4 groups [inference]) of six rounds on Tuesdays every 15 days, from 9/9 to 18/11; the knockout (stage 3): semi-final drawn on 19/11 and played on 25/11 over two legs (7 days), final drawn on 3/12 and a single match on 9/12; category 8; tie-break 3'),
         ('cqp10_supercopa_europeia_tabela', 'EurSuperCup::create_round_table', '0x57a3c0', 'eur_super_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa Europeia é um jogo único em 24 de agosto (sorteio em 4 de julho) em campo neutro (mando 0x8000: Mônaco [inferência]), categoria 8, desempate 0x83, sem prêmios',
          'rule: the European Super Cup is a single match on 24 August (drawn on 4 July) on neutral ground (venue 0x8000: Monaco [inference]), category 8, tie-break 0x83, no prizes'),
         ('cqp10_supercopa_asiatica_tabela', 'AsiaSuperCup::create_round_table', '0x410910', 'asia_super_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa Asiática é em ida e volta, o primeiro jogo em 4 de agosto (sorteio em 4 de julho) e a volta 14 dias depois, categoria 6, desempate de ida e volta 0x83, sem prêmios',
          'rule: the Asian Super Cup is two-legged, the first match on 4 August (drawn on 4 July) and the return 14 days later, category 6, two-leg tie-break 0x83, no prizes')])
CAMPOS = [('copa +0x40 ano da copa; valor (palavra com sinal); lê (as quatro)',
           'cup +0x40 cup year; value (signed word); reads (all four)'),
          ('rodada de copa (0x68 bytes) +0..+0x22 e +0x5c..+0x67; valor; grava (mata-mata e supercopas)',
           'cup round (0x68 bytes) +0..+0x22 and +0x5c..+0x67; value; writes (knockout and super cups)'),
          ('rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5..+0xb jogo 0 vazio, +0x3d valor; valor; grava (grupos)',
           'group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5..+0xb empty match 0, +0x3d value; value; writes (groups)')]
INTERESSA = ['CMRTE']
