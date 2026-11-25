# O dia de cada pessoa (game_daily_staff_update 0x5ce2d0, game.cpp, cdecl, 30.544 bytes; leitores gra, rodadas 4 e 6,
# 21 e 25/11/2026: gra_2, gra_2b e gra_2c). A rotina inteira desviada num item só, partida em pedaços com nome, na ordem
# do original. Lida da desmontagem (os [esp+k] seguidos pelos push pendentes, o x87 instrução por instrução: cada fst
# de 32 bits vira um float na cópia). Todas as rotinas do exe chamadas vão pelo endereço do exe, inclusive as desviadas
# (o jogo passa pelos desvios; a emulação as pega).
# - laço: base + (n - 1) × 0x6e com < (a última pessoa nunca tem o dia: defeito); a LLIST local do original
#   (0x68d020/0x68d040) nunca é usada e a cópia não a tem; o bônus [esp+0x70] começa em 0 (no original, lixo da pilha);
# - por pessoa: sem nação ou aposentada (estado & 0x30 = 0x20) pula; o clube do contrato (get_contract, guardado em
#   G_CONTRACT_LOOKUP_RESULT) ou o emprego nos cargos 5 e 12; o time B ligado (club_get_linked_b_team, 1); clube sem
#   nação vale nenhum, senão o contrato com ele (get_contract_for_club);
# - sem ficha de jogador: o dia do não jogador; o jogador: daily_wear, o portão (3 dias do ano, divisão carregada,
#   reputação ≥ 4750 ou (id % 7) + dia = 0), o dia do jogador (gra_2b), a queda do desempregado, o cansaço no treino;
#   nos 3 dias do ano, as tabelas de habilidade, game_average, a idade da virada, a deriva da personalidade, o
#   desenvolvimento (0x5d3d09..0x5d5771, gra_2c), a normalização e as relações; todo dia, a lista de observação;
# - fim: com algum crescimento, as habilidades devolvidas na ordem.
# Prólogo push ebp; mov ebp, esp; and esp, -8 (6 bytes, sem relativo); a rotina inteira é igual no cm0102-1080.exe.
# Comparador: de passagem (só a original roda: o dia mexe em todas as pessoas e chama dezenas de rotinas com efeito).
# A prova da cópia: a emulação da rotina inteira (check_routine.py, unicorn: a original e a cópia no mesmo mundo falso,
# cada chamada com os argumentos e cada byte escrito) e as mutações pela carreira.
SAVE = False
ROTINAS = [('game_daily_staff_update', 0x5ce2d0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('o dia de cada pessoa',
        'every person\'s day',
        [('game_daily_staff_update', 'game_daily_staff_update', '0x5ce2d0', 'game.cpp', 'cdecl',
          'regra: todo dia, para cada pessoa da tabela (menos a última, defeito do original) com nação e não aposentada: o não jogador aprende a língua do clube e ganha ou perde habilidade e reputação; o desempregado perde reputação passados 60 dias mais 2 por jogo pela seleção; o jogador com contrato (nos seus três dias do ano, quando a divisão do clube está carregada, com reputação 4750 ou mais, ou no portão quase sempre fechado) tem o seu dia: o companheiro favorito, o cansaço, a forma e as queixas que viram inquietação, as disputas, a moral, as histórias da mídia, o pedido de valorização, o status no elenco, o pedido de saída, as conversas de contrato, a adaptação e a saudade de casa; depois a queda de condição no treino; nos três dias do ano (id % 365 e 120 e 240 dias depois) a deriva da personalidade (uma vez em dez) e o desenvolvimento: o fator de idade (idade da virada pela posição menos a idade, × 0,25), a queda de reputação de quem declina, a nota da temporada contra a média do jogo, o crescimento por duas notas numa tabela de 13 passos, o cansaço e a lesão, o treino (instalações ou reputação, o técnico, o auxiliar e os cinco treinadores, o nível da divisão, 0,1 a 3, o crescimento de 2 a 25), o profissionalismo, a habilidade que sobe random(crescimento) com o teto da divisão fraca e o potencial, o elogio do treinador, os atributos físicos de quem declina, os mentais de quem cresce, e o envelhecimento a partir dos 30; todo dia a chance de entrar nas listas de observação dos clubes; no fim, se algum desenvolvimento cresceu, as habilidades da tabela devolvidas na ordem',
          'rule: every day, for every person of the table (but the last, a defect of the original) with a nation and not retired: a non-player learns the club\'s language and gains or loses ability and reputation; an unattached person loses reputation past 60 days plus 2 per cap; a contracted player (on his three days of the year, when his club\'s division is loaded, with reputation 4750 or more, or at the nearly always closed gate) has his day: the favoured team mate, fatigue, form and grievances turning into unrest, disputes, morale, media stories, the value demand, the squad status, the wish to move, contract talks, settling and homesickness; then the condition lost in training; on his three days of the year (id % 365 and 120 and 240 days later) the personality drift (one time in ten) and the development: the age factor (the turning age by position minus the age, × 0.25), the reputation loss of a declining player, the season\'s rating against the game\'s average, the growth by two ratings in a table of 13 steps, fatigue and injury, the training (facilities or reputation, the manager, the assistant and the five coaches, the division\'s level, 0.1 to 3, the growth from 2 to 25), professionalism, the ability rising by random(growth) under a weak division\'s ceiling and the potential, the coach\'s praise, the physical attributes of a declining player, the mental ones of a developing player, and the ageing from 30; every day the chance to join the clubs\' shortlists; at the end, when some development grew, the table\'s abilities handed back in order')])
CAMPOS = []
INTERESSA = []
