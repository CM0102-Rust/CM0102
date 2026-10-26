# Três consultas do Database.cpp que o olheiro usa no relatório de um clube (PLAYER_SEARCH::run_scout_day 0x7b3e20,
# o único chamador; as respostas vão como parâmetros da notícia do relatório, 0x785790): o clube tem jogadores
# esforçados, um goleiro em condições e meias que roubam a bola? Leitura pura do elenco (+0xd7) com a lesão pela
# INJURY 0x62c400 (chamada pelo endereço do jogo). Sem SEH, sem sorteio, sem caixa; a única local (o contador, o byte
# alto do ecx empurrado) é zerada na entrada; a 0x549470 não tem local.
SAVE = False
ROTINAS = [('clube_tem_esforcados', 0x5493c0, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('clube_tem_goleiro', 0x549470, [0x8b, 0x44, 0x24, 0x04, 0x55]),
           ('clube_tem_meias_que_roubam', 0x549850, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('o olheiro: o clube tem esforçados, goleiro em condições e meias que roubam a bola',
        'the scout: the club has hard workers, a fit goalkeeper and ball-winning midfielders',
        [('clube_tem_esforcados', 'club_has_enough_hard_workers', '0x5493c0', 'Database.cpp', 'cdecl',
          '1 se mais de 4 jogadores sem lesão grave passam do limite de habilidade (reputação/65 + 40, de 40 a 140) com o atributo +0x44 acima de 14',
          '1 when more than 4 players without serious injury are above the ability limit (reputation/65 + 40, 40 to 140) with attribute +0x44 above 14'),
         ('clube_tem_goleiro', 'club_has_fit_goalkeeper', '0x549470', 'Database.cpp', 'cdecl',
          '1 se algum goleiro (+0xf = 20) do elenco está sem lesão grave e tem id válido',
          '1 when some goalkeeper (+0xf = 20) in the squad has no serious injury and a valid id'),
         ('clube_tem_meias_que_roubam', 'club_has_ball_winning_midfielders', '0x549850', 'Database.cpp', 'cdecl',
          '1 se mais de 1 meia sem lesão grave passa do limite (reputação/75 + 25, de 40 a 135) com os atributos +0x44 e +0x3e acima de reputação/500 + 5 (no máximo 14)',
          '1 when more than 1 midfielder without serious injury is above the limit (reputation/75 + 25, 40 to 135) with attributes +0x44 and +0x3e above reputation/500 + 5 (at most 14)')])
CAMPOS = [('TClub +0x80: reputação; curto com sinal; lê (clube_tem_esforcados, clube_tem_meias_que_roubam)', 'TClub +0x80: reputation; signed short; reads (clube_tem_esforcados, clube_tem_meias_que_roubam)'),
          ('TClub +0xd7 (50): elenco; ponteiros de pessoa; lê (as três)', 'TClub +0xd7 (50): squad; person pointers; reads (all three)'),
          ('TPlayer +0x0f: goleiro; +0x12..+0x14: meia; bytes (20 = natural); +0x3e, +0x44: atributos; bytes com sinal; lê', 'TPlayer +0x0f: goalkeeper; +0x12..+0x14: midfielder; bytes (20 = natural); +0x3e, +0x44: attributes; signed bytes; reads')]
INTERESSA = ['SGE', 'CMRTE']
