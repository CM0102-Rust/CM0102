# O calendário dos torneios de seleções: a Copa América, a Copa Africana de Nações e a Copa das Confederações (o slot
# +0x3c create_schedule; goldcup.cpp, african_nations.cpp, fifa_confed.cpp, em intercomp\). Frente das competições
# (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x5e0900 (GoldCup::create_schedule, 1612 bytes; 5 chamadas até 2002-06-01 com as 8 ligas: os três grupos e o
#   mata-mata da Copa América de 2001): duas mutações.
# - 0x4012e0 (AfricanNations::create_schedule, 1736 bytes; 5 chamadas: os quatro grupos e o mata-mata da Copa Africana
#   de 2002): uma mutação.
# - 0x58b560 (FifaConfed::create_schedule, 1199 bytes; 1 chamada): a Copa das Confederações (passageira: só a fase da
#   montagem roda na janela).
# As três montam a tabela por fase com as mesmas operações (as datas de grupo pela 0x68a160 e os jogos pela 0x68a1c0,
# com os confrontos de cada grupo; o sorteio e o jogo de copa pela 0x521e60/0x521eb0; os confrontos fixos do mata-mata
# pela 0x521f10; e as gravações dos campos): o cqp12_tabela executa a lista comum e a de cada grupo, extraídas da
# desmontagem (na ordem da original; a escolha do grupo é a tabela de saltos da original), e o comparador confere os
# bytes que cada operação grava.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x3c (push saida; push &extra; push &rodadas; push fase;
# mov ecx, torneio -> thiscall, ret 0x10; a fase lida como byte com sinal). Dentro: 0x944e46 (push tamanho; add esp,
# 4 -> cdecl), 0x68a160 (9 pushes -> cdecl; o ano num registrador com lixo nos 16 bits altos, a 0x68a160 usa só a
# palavra: a cópia passa limpo), 0x68a1c0 (7 pushes -> cdecl), 0x521e60 (7 pushes -> cdecl), 0x521eb0 (9 pushes ->
# cdecl), 0x521f10 (7 pushes -> cdecl) e a caixa (aviso_em). O extra é gravado sem conferir o ponteiro (como na
# original); o é_liga, conferido.
# Comparadores: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais; o
# [0xb67a34]); as saídas e os bytes gravados pelas operações nas duas tabelas têm de ser iguais; a tabela da cópia é
# liberada. Fora das fases só a original (nenhuma grava nada).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqp12_copa_america_tabela', 0x5e0900, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp12_africana_tabela', 0x4012e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp12_confed_tabela', 0x58b560, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário dos torneios de seleções: a Copa América, a Copa Africana de Nações e a Copa das Confederações',
        'the national-team tournaments\' calendar: the Copa América, the African Nations Cup and the Confederations Cup',
        [('cqp12_copa_america_tabela', 'GoldCup::create_schedule', '0x5e0900', 'goldcup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa América tem três grupos (fases −1, 0 e 1, ids 0x41f a 0x421) de três rodadas em 6, 10 e 13 de julho, cada rodada com os dois jogos fixos do grupo (os confrontos e o mando de cada um); o mata-mata (fase 2) em jogo único: quartas em 16/7 com os confrontos fixos (os dois melhores terceiros [inferência]), semifinal em 19/7, terceiro lugar em 22/7 e final em 23/7 (mando 4, campo neutro [inferência]); categoria 10; desempate 0x103 até a semifinal e 3 na final; sem prêmios',
          'rule: the Copa América has three groups (stages −1, 0 and 1, ids 0x41f to 0x421) of three rounds on 6, 10 and 13 July, each round with the group\'s two fixed matches (pairings and venue); the knockout (stage 2) in single matches: quarter-finals on 16/7 with fixed pairings (the two best third-placed teams [inference]), semi-final on 19/7, third place on 22/7 and final on 23/7 (venue 4, neutral ground [inference]); category 10; tie-break 0x103 up to the semi-final and 3 in the final; no prizes'),
         ('cqp12_africana_tabela', 'AfricanNations::create_schedule', '0x4012e0', 'african_nations.cpp', 'thiscall, ret 0x10',
          'regra: a Copa Africana de Nações tem quatro grupos (fases −1 a 2, ids 0x3e8 a 0x3eb) de três rodadas em 8, 12 e 16 de fevereiro, com os jogos fixos de cada grupo; o mata-mata (fase 3) em jogo único: quartas em 21/2 com os confrontos fixos, semifinal em 25/2, terceiro lugar em 27/2 e final em 28/2 (mando 4); categoria 10; desempate 0x303 até a semifinal e 0x203 na final; sem prêmios',
          'rule: the African Nations Cup has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 8, 12 and 16 February, with each group\'s fixed matches; the knockout (stage 3) in single matches: quarter-finals on 21/2 with fixed pairings, semi-final on 25/2, third place on 27/2 and final on 28/2 (venue 4); category 10; tie-break 0x303 up to the semi-final and 0x203 in the final; no prizes'),
         ('cqp12_confed_tabela', 'FifaConfed::create_schedule', '0x58b560', 'fifa_confed.cpp', 'thiscall, ret 0x10',
          'regra: a Copa das Confederações tem dois grupos (fases −1 e 0, ids 0x3e8 e 0x3e9) de três rodadas em 29 de julho, 1º e 4 de agosto, com os jogos fixos; o mata-mata (fase 1) em jogo único: semifinal em 6/8, terceiro lugar em 8/8 e final em 9/8 (mando 4); categoria 10; desempate 0x383 na semifinal e 0x83 depois',
          'rule: the Confederations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds on 29 July, 1 and 4 August, with fixed matches; the knockout (stage 1) in single matches: semi-final on 6/8, third place on 8/8 and final on 9/8 (venue 4); category 10; tie-break 0x383 in the semi-final and 0x83 after')])
CAMPOS = [('torneio +0x40 ano do torneio; valor (palavra com sinal); lê (os três)',
           'tournament +0x40 tournament year; value (signed word); reads (all three)'),
          ('rodada de copa (0x68 bytes) +0..+0x22 (com os confrontos fixos em +0x23 + 7 × vaga) e +0x5c..+0x67; valor; grava (mata-mata)',
           'cup round (0x68 bytes) +0..+0x22 (with the fixed pairings at +0x23 + 7 × slot) and +0x5c..+0x67; value; writes (knockout)'),
          ('rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5 + 7 × jogo os jogos, +0x3d valor; valor; grava (grupos)',
           'group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5 + 7 × match the matches, +0x3d value; value; writes (groups)')]
INTERESSA = ['CMRTE']
