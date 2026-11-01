# A tabela da liga (league.cpp) e a fase seguinte da First e da Second (eng_first.cpp, eng_second.cpp):
# League::update_positions_and_status (0x684cd0, a virtual +0x94 da liga, do passe do dia 0x684946: 1057 chamadas na
# cobertura de 17/10; a melhor e a pior posição possível de cada clube e a situação decidida: título, acesso, vaga no
# mata-mata, rebaixamento, vaga europeia; cada situação nova vai ao anúncio +0x40, 0x686670 em Rust, que manda a notícia
# e chama os registros +0x44), EngFirst::advance_stage (0x572ff0) e EngSecond::advance_stage (0x5769e0), as virtuais
# +0x28 que passam à fase seguinte e criam o mata-mata (cpz_1) quando ela chega a 0. Sem SEH; iguais no
# cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x6827d0 (League::sort_table): push 0; ecx = this; sem add esp -> thiscall(liga, 0), ret 4.
# - 0x944e46 (malloc) e 0x9452ca (free): push; call; add esp, 4 -> cdecl.
# - virtual +0x40 (announce_team_status): push 0; push 0; push eax (al = +0x44, os bits de cima do ponteiro da linha
#   ou da devolução das cláusulas, reproduzidos); push situação; push clube; ecx = this; sem add esp -> thiscall(liga,
#   clube, situação, fase, 0, 0), ret 0x14.
# - 0x4cd030 (CONTRACT_MANAGER::trigger_relegation_clauses): push +4; push clube; ecx = [0xae1050]; sem add esp ->
#   thiscall(cm, clube, comp), ret 8.
# - 0x4b0700 (a competição conta): push id; call; add esp, 4 -> cdecl(id).
# - 0x905c60 (UefaSeeding::get_nation_places): push &uefa; push &cl; push nação; ecx = [0xde1f58]; sem add esp ->
#   thiscall(semeadura, nação, &cl, &uefa), ret 0xc.
# - virtual +0x68 (set_club_progress): push pior; push melhor; push posição; push +0x44; push clube; ecx = this ->
#   thiscall(liga, clube, fase, posição, melhor, pior), ret 0x14 (registradores com lixo em cima; os chamados só leem o
#   byte).
# - 0x573080 / 0x576a70 (o mata-mata): ecx = this; sem push; sem add esp -> thiscall(this), ret 0.
# Comparadores: a 0x684cd0 em ensaio (modelo CPV6_ENSAIO): a cópia sem as virtuais +0x40 e +0x68 e sem as cláusulas,
# depois a original inteira a partir do mesmo estado; comparam a tabela inteira, +0xa7 e a vaga europeia de cada clube da
# tabela. As duas advance_stage lado a lado em +0x30 quando a fase não chega a 0; na última fase e na que chega a 0
# (o mata-mata aloca), só a original.
SAVE = False
ROTINAS = [('posicoes_e_situacao_na_liga', 0x684cd0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('fase_seguinte_da_first', 0x572ff0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('fase_seguinte_da_second', 0x5769e0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a tabela da liga: as posições possíveis e a situação de cada clube, e a fase seguinte da First e da Second',
        'the league table: the possible positions and each club status, and the next stage of the First and Second',
        [('posicoes_e_situacao_na_liga', 'League::update_positions_and_status', '0x684cd0', 'league.cpp', 'thiscall, ret 0',
          'ordena a tabela (0x6827d0), marca as faixas (acesso, mata-mata, 2 e rebaixamento) de cima e de baixo, calcula a melhor (+0x39) e a pior (+0x3a) posição possível pelos máximos de pontos e decide a situação (+0x37): título (5), acesso (0, 7 ou 6), mata-mata (1), rebaixamento (3, com as cláusulas 0x4cd030), 2 e 0xfe, cada uma anunciada (+0x40); a vaga europeia ([0xae2d14]) pelas vagas da nação (0x905c60); a situação nas estatísticas (+0x68)',
          'sorts the table (0x6827d0), marks the bands (promotion, play-offs, 2 and relegation) from the top and the bottom, works out the best (+0x39) and worst (+0x3a) possible position from the maximum points and decides the status (+0x37): title (5), promotion (0, 7 or 6), play-offs (1), relegation (3, with the clauses 0x4cd030), 2 and 0xfe, each announced (+0x40); the European place ([0xae2d14]) from the nation places (0x905c60); the stats progress (+0x68)'),
         ('fase_seguinte_da_first', 'EngFirst::advance_stage', '0x572ff0', 'eng_first.cpp', 'thiscall, ret 0',
          'na última fase (+0x30 ≥ +0x2c − 1) a caixa (linha 0x2ce); senão +0x30 + 1 e, chegando a 0, o mata-mata (0x573080)',
          'at the last stage (+0x30 >= +0x2c - 1) the error box (line 0x2ce); otherwise +0x30 + 1 and, on reaching 0, the play-offs (0x573080)'),
         ('fase_seguinte_da_second', 'EngSecond::advance_stage', '0x5769e0', 'eng_second.cpp', 'thiscall, ret 0',
          'como a da First (linha 0x294; o mata-mata 0x576a70)',
          'like the First one (line 0x294; the play-offs 0x576a70)')])
CAMPOS = [('liga +0xb1 / +0x3e: a tabela (0x3b bytes: +0 clube, +0x11 jogos feitos, +0x19 pontos, +0x37 situação, +0x39 melhor posição, +0x3a pior) e as linhas; ponteiro e valor; lê, grava +0x37/+0x39/+0x3a (posicoes_e_situacao_na_liga)',
           'league +0xb1 / +0x3e: the table (0x3b bytes: +0 club, +0x11 games played, +0x19 points, +0x37 status, +0x39 best position, +0x3a worst) and the rows; pointer and value; reads, writes +0x37/+0x39/+0x3a (posicoes_e_situacao_na_liga)'),
          ('liga +0xa7 / +0x3c / +0x45: os jogos da temporada (grava se −1), os turnos e o calendário; valor; lê e grava (posicoes_e_situacao_na_liga)',
           'league +0xa7 / +0x3c / +0x45: the season games (written if -1), the rounds per pair and the schedule; value; reads and writes (posicoes_e_situacao_na_liga)'),
          ('liga +0xbe / +0xbf / +0xc0 / +0xc1: as vagas de acesso, mata-mata, faixa 2 e rebaixamento; valor; lê (posicoes_e_situacao_na_liga)',
           'league +0xbe / +0xbf / +0xc0 / +0xc1: the promotion, play-off, band 2 and relegation places; value; reads (posicoes_e_situacao_na_liga)'),
          ('liga +0xc2 / +0xd9 / +0xdd / +0xe6 / +0x44 / +0x1c: pontos da vitória, regras da tabela, liga parada, desempate, fase-mãe, liga de acesso; valor; lê (posicoes_e_situacao_na_liga)',
           'league +0xc2 / +0xd9 / +0xdd / +0xe6 / +0x44 / +0x1c: points per win, table rules, league halted, tie-break, parent stage, promotion league; value; reads (posicoes_e_situacao_na_liga)'),
          ('[0xae2d14] + 6 × id do clube: a competição europeia da próxima temporada; id; lê e grava (posicoes_e_situacao_na_liga)',
           "[0xae2d14] + 6 x club id: next season's European competition; id; reads and writes (posicoes_e_situacao_na_liga)"),
          ('liga +0x30 / +0x2c: a fase atual e quantas fases; valor; lê e grava (fase_seguinte_da_first, fase_seguinte_da_second)',
           'league +0x30 / +0x2c: the current stage and the stage count; value; reads and writes (fase_seguinte_da_first, fase_seguinte_da_second)')]
INTERESSA = ['CMRTE']
