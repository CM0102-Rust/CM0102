# Três consultas do Database.cpp que o olheiro usa no relatório de um clube (PLAYER_SEARCH::run_scout_day 0x7b3e20,
# o único chamador; as respostas vão como parâmetros da notícia do relatório, 0x785790): o clube tem zagueiros, meias
# e atacantes acima de um limite de habilidade tirado da reputação (/65 + 40)? As de defesa e ataque olham antes os
# gols por jogo do clube na competição (COMP_STATS 0x4a4850, chamada pelo endereço do jogo: ordena a tabela na primeira
# vez, efeito igual com a original antes da cópia). Sem SEH, sem sorteio, sem caixa; a única local (o contador, o
# byte alto do ecx empurrado) é zerada na entrada.
SAVE = False
ROTINAS = [('clube_tem_defensores', 0x549100, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('clube_tem_meias', 0x549200, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('clube_tem_atacantes', 0x5492c0, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('o olheiro: o clube tem defensores, meias e atacantes que bastem',
        'the scout: the club has enough defenders, midfielders and strikers',
        [('clube_tem_defensores', 'club_has_enough_defenders', '0x549100', 'Database.cpp', 'cdecl',
          'clube que toma mais de 1,35 gol por jogo: 0; senão 1 se mais de 2 zagueiros (+0x11 = 20) sem lesão grave passam do limite de habilidade (reputação/65 + 40, de 40 a 145)',
          'club conceding more than 1.35 goals a game: 0; otherwise 1 when more than 2 defenders (+0x11 = 20) without serious injury are above the ability limit (reputation/65 + 40, 40 to 145)'),
         ('clube_tem_meias', 'club_has_enough_midfielders', '0x549200', 'Database.cpp', 'cdecl',
          '1 se mais de 2 meias (+0x14, +0x12 ou +0x13 = 20) sem lesão grave passam do limite (reputação/65 + 40, de 40 a 145)',
          '1 when more than 2 midfielders (+0x14, +0x12 or +0x13 = 20) without serious injury are above the limit (reputation/65 + 40, 40 to 145)'),
         ('clube_tem_atacantes', 'club_has_enough_strikers', '0x5492c0', 'Database.cpp', 'cdecl',
          'clube que faz menos de 1,5 gol por jogo: 0; senão 1 se mais de 1 atacante (+0x15 = 20) sem lesão grave passa do limite (reputação/65 + 40, de 40 a 150)',
          'club scoring fewer than 1.5 goals a game: 0; otherwise 1 when more than 1 striker (+0x15 = 20) without serious injury is above the limit (reputation/65 + 40, 40 to 150)')])
CAMPOS = [('TClub +0x57: competição (o +0 é o índice em [0xadadfc]); ponteiro; lê (clube_tem_defensores, clube_tem_atacantes)', 'TClub +0x57: competition (+0 is the index into [0xadadfc]); pointer; reads (clube_tem_defensores, clube_tem_atacantes)'),
          ('TClub +0x80: reputação; curto com sinal; lê (as três)', 'TClub +0x80: reputation; signed short; reads (all three)'),
          ('TClub +0xd7 (50): elenco; ponteiros de pessoa; lê (as três)', 'TClub +0xd7 (50): squad; person pointers; reads (all three)'),
          ('TPlayer +0x05: habilidade atual; curto; +0x11..+0x15: posições (20 = natural); bytes; lê (as três)', 'TPlayer +0x05: current ability; short; +0x11..+0x15: positions (20 = natural); bytes; reads (all three)'),
          ('estatística do clube na competição (0x4a4850): +4 jogos, +0x69 gols marcados, +0x6a gols sofridos; bytes; lê', 'club competition stats (0x4a4850): +4 games, +0x69 goals scored, +0x6a goals conceded; bytes; reads')]
INTERESSA = ['SGE', 'CMRTE']
