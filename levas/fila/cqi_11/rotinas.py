# A Copa dos Campeões da CONCACAF: os oito classificados, o dia da copa e as rodadas (con_champ.cpp). Frente das
# competições (Agente D, 03/11/2026), carreira de 8 ligas (a MLS é uma delas).
# - 0x4c11a0 (ConChamp::select_qualified_clubs, 672 bytes): os oito times, dois mexicanos, três centro-americanos, um
#   caribenho e dois da MLS (o campeão da MLS e o da competição [0x9cf728], ou o vice da MLS quando é o mesmo clube); 2
#   chamadas até 2002-06-01. É a rotina das mutações (as três).
# - 0x4c1620 (ConChamp::update_for_date, 197 bytes, virtual +0x14; 2064 chamadas): com a tabela vazia, os
#   classificados e as posições previstas; depois a atualização da copa (0x51f890, Rust).
# - 0x4c15f0 (ConChamp::set_up_rounds, 42 bytes, virtual +0x8c; 3 chamadas): o fundo de prêmios de 2.000.000 e a
#   tabela das rodadas.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   0x4c11a0: chamada em 0x4c1648 (mov ecx, esi; sem push) -> thiscall, ret 0 (devolve 1). 0x4c1620 e 0x4c15f0:
#   virtuais (+0x14: push fase; push hoje; mov ecx -> thiscall, ret 8; +0x8c: mov ecx; sem push -> thiscall, ret 0).
#   Na 0x4c11a0: 0x53b900 (push clube; add esp, 4 -> cdecl), 0x4afbd0 (push k; push n; push lista; add esp, 0xc no
#   fim, como o compilador junta -> cdecl; a contagem num registrador com movzx/movsx da palavra: o chamado lê a
#   palavra, a cópia passa limpa), 0x4afd80 (push lugar; push comp; add esp, 8 -> cdecl).
#   Na 0x4c1620: 0x4c11a0 (thiscall), a virtual +0x5c (mov ecx, this -> thiscall, ret 0), 0x51f890 (push fase; push
#   hoje; mov ecx, this -> thiscall, ret 8), a caixa da linha 0x1ba (aviso_em).
#   Na 0x4c15f0: a virtual +0x3c (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall, ret 0x10).
# Comparadores: 0x4c11a0 lado a lado (os 8 times de +0xa7, +0x3e, o estado do sorteador e o histórico de competições
# [0xae23d4] × [0xae237c] linhas de 0x1a, que a 0x4afd80 reordena; a cópia sem contar: a 0x4afbd0 embaralha), só a original quando falta clube numa lista; 0x4c1620 só a original (a atualização joga) com o
# plano (+0x3e = 8 depois da seleção); 0x4c15f0 só a original (a virtual aloca) com o plano (+0x28 e a devolução).
# Determinismo: as quatro listas locais começam zeradas na cópia; na original, a parte não preenchida é sobra de pilha,
# lida só quando falta clube (o comparador manda esse caso para a original). Nada lê endereço como valor.
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: as três ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqi11_concacaf_classificados', 0x4c11a0, [0x81, 0xec, 0xa0, 0x09, 0x00, 0x00]),
           ('cqi11_concacaf_dia', 0x4c1620, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqi11_concacaf_rodadas', 0x4c15f0, [0x56, 0x8b, 0xf1, 0x6a, 0x00])]
DOCS = ('a Copa dos Campeões da CONCACAF: os classificados, o dia da copa e as rodadas',
        'the CONCACAF Champions\' Cup: the qualified clubs, the cup day and the rounds',
        [('cqi11_concacaf_classificados', 'ConChamp::select_qualified_clubs', '0x4c11a0', 'con_champ.cpp', 'thiscall, ret 0',
          'regra: oito clubes. Dois do México (sorteados entre os 6 de maior reputação), três da América Central (região 7 da nação, fora da nação [0x9cf4f8]; entre os 9 maiores), um do Caribe (região 6; entre os 3 maiores) e dois dos Estados Unidos: com a competição [0x9cf728] em jogo [inferência: a US Open Cup], o último campeão da MLS e o campeão dela (se for o mesmo clube, a vaga vai ao vice da MLS); sem ela, dois clubes da MLS entre os 3 de maior reputação. Até 200 clubes por região e 12 da MLS são considerados',
          'rule: eight clubs. Two from Mexico (drawn among the 6 with the highest reputation), three from Central America (nation region 7, other than nation [0x9cf4f8]; among the top 9), one from the Caribbean (region 6; among the top 3) and two from the United States: with competition [0x9cf728] running [inference: the US Open Cup], the latest MLS champion and its champion (if the same club, the place goes to the MLS runner-up); without it, two MLS clubs among the top 3 by reputation. Up to 200 clubs per region and 12 from the MLS are considered'),
         ('cqi11_concacaf_dia', 'ConChamp::update_for_date', '0x4c1620', 'con_champ.cpp', 'thiscall, ret 8',
          'regra: no primeiro dia da temporada da copa (a tabela de times vazia), escolhe os classificados e as posições previstas; todo dia, a atualização da copa (sorteios e jogos)',
          'rule: on the first day of the cup season (empty team table), picks the qualified clubs and the predicted positions; every day, the cup update (draws and matches)'),
         ('cqi11_concacaf_rodadas', 'ConChamp::set_up_rounds', '0x4c15f0', 'con_champ.cpp', 'thiscall, ret 0',
          'regra: o fundo de prêmios da copa é 2.000.000; a tabela das rodadas (mata-mata de três rodadas) vem da virtual +0x3c',
          'rule: the cup prize fund is 2,000,000; the round table (three-round knockout) comes from virtual +0x3c')])
CAMPOS = [('copa +0x3e times (8), +0xa7 times (6 bytes: +0 clube, +4, +5), +0x28 fundo de prêmios, +0xa3 rodadas, +0x3a/+0x3c código e rodadas; valor/ponteiro; grava (cqi11_concacaf_classificados, cqi11_concacaf_rodadas)',
           'cup +0x3e teams (8), +0xa7 teams (6 bytes: +0 club, +4, +5), +0x28 prize fund, +0xa3 rounds, +0x3a/+0x3c code and rounds; value/pointer; writes (cqi11_concacaf_classificados, cqi11_concacaf_rodadas)'),
          ('clube +0x53 nação, +0x57 divisão; nação +0 id, +0x75 região (6 Caribe, 7 América Central); ponteiro/valor; lê (cqi11_concacaf_classificados)',
           'club +0x53 nation, +0x57 division; nation +0 id, +0x75 region (6 Caribbean, 7 Central America); pointer/value; reads (cqi11_concacaf_classificados)'),
          ('[0x9cf3d0] id do México, [0x9cf4f8] nação fora da região 7 [inferência: os Estados Unidos], [0x9cf590] id da MLS, [0x9cf728] id da competição do segundo clube americano; id; lê',
           '[0x9cf3d0] Mexico id, [0x9cf4f8] nation left out of region 7 [inference: the United States], [0x9cf590] MLS id, [0x9cf728] id of the second US club\'s competition; id; reads')]
INTERESSA = ['CMRTE']
