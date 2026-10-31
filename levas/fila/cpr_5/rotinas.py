# As diretorias no início da carreira (MANAGER_MANAGER::init_club_board_records 0x68f130, pelo construtor 0x68dbf0: uma
# chamada na carreira, antes do primeiro dia), as apelações de suspensão (process_ban_appeals 0x6abef0, pelo dia a dia
# da diretoria 0x68f9c0; só as apelações do humano entram na lista, vazia na carreira automática) e a partida que o
# humano pode ver (HUMAN_MANAGER::can_watch_fixture 0x5fe700, pela limpeza das partidas assistidas 0x5fe8d0, Rust, e
# pela escolha da partida 0x5fe500; 966 chamadas na carreira até 2002-06-01). Fora de fechar_partida_do_dia e de
# eventos_do_pos_jogo. Iguais no cm0102-1080.exe. A 0x68f130 tem x87 (floats guardados em 32 bits, _ftol 0x945640) e
# sorteios; sem SEH e sem leitura de lixo (a variável do clube parceiro, [esp+0x20], é zerada na entrada e passa de
# clube em clube, como na cópia). A 0x5fe700 tem quadro SEH sem destrutor (a data local é trivial). Comparadores: a
# 0x68f130 e a 0x6abef0 só a original (sorteios, vagas, disciplina, notícia); a 0x5fe700 pelo valor.
SAVE = False
ROTINAS = [('iniciar_diretorias_dos_clubes', 0x68f130, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('apelacoes_de_suspensao', 0x6abef0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('pode_ver_a_partida', 0x5fe700, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as diretorias no início da carreira, as apelações de suspensão e a partida que o humano pode ver',
        'the club boards at the start of the career, the ban appeals and the fixture the human may watch',
        [('iniciar_diretorias_dos_clubes', 'MANAGER_MANAGER::init_club_board_records', '0x68f130', 'manager_manager.cpp', 'thiscall, ret 0',
          'arruma as funções das pessoas com clube (técnico em +0xcf) e monta a linha da diretoria de cada clube: datas, reputação², as quatro confianças por sorteio a partir da expectativa (nota da diretoria ou reputação) e da reputação, presas a 5750/5500..10000, ou 5000 sem técnico; as expectativas da temporada (0x6a3770) e, sem técnico, a vaga (0x69ba50)',
          'fixes the jobs of staff with a club (manager at +0xcf) and builds each club board row: dates, reputation squared, the four confidences drawn from the expectation (board rating or reputation) and the reputation, held to 5750/5500..10000, or 5000 without a manager; the season expectations (0x6a3770) and, without a manager, the vacancy (0x69ba50)'),
         ('apelacoes_de_suspensao', 'MANAGER_MANAGER::process_ban_appeals', '0x6abef0', 'manager_manager.cpp', 'thiscall, ret 0',
          'para cada apelação (+0x14) cuja data chegou: o resultado nas até 4 regras da disciplina (0x557000), a notícia (0x6ac060) e a apelação sai da lista',
          'for each appeal (+0x14) whose date has come: the result on up to 4 discipline rule sets (0x557000), the news (0x6ac060) and the appeal leaves the list'),
         ('pode_ver_a_partida', 'HUMAN_MANAGER::can_watch_fixture', '0x5fe700', 'human_manager.cpp', 'thiscall, ret 8',
          '1 se a partida tem +0x43 negativo e nem ela nem as do mesmo dia (cada grupo das nações-chave) têm um clube do humano; senão 0',
          '1 when the fixture has a negative +0x43 and neither it nor the fixtures of that day (each key nation group) have a club of the human; else 0')])
CAMPOS = [('pessoa +0x24 clube, +0x28 função, +0x3d função de origem, +0x39 contrato, +0x61 ficha de jogador; ponteiro/valor; lê e grava +0x24, +0x28, +0x3d (iniciar_diretorias_dos_clubes)',
           'staff +0x24 club, +0x28 job, +0x3d original job, +0x39 contract, +0x61 player data; pointer/value; reads and writes +0x24, +0x28, +0x3d (iniciar_diretorias_dos_clubes)'),
          ('clube +0xcf técnico; ponteiro; grava a pessoa de função 5/12, zera na segunda seleção (iniciar_diretorias_dos_clubes)',
           'club +0xcf manager; pointer; writes the staff with job 5/12, cleared for second national teams (iniciar_diretorias_dos_clubes)'),
          ('clube +0x80 reputação; valor; lê, presa a 200 × nação +0x8e na segunda seleção (iniciar_diretorias_dos_clubes)',
           'club +0x80 reputation; value; reads, capped at 200 × nation +0x8e for second national teams (iniciar_diretorias_dos_clubes)'),
          ('clube +0xbf, +0xc3..+0xcf, +0xd3, +0x19f..+0x1db; ponteiros; zera na segunda seleção (iniciar_diretorias_dos_clubes)',
           'club +0xbf, +0xc3..+0xcf, +0xd3, +0x19f..+0x1db; pointers; cleared for second national teams (iniciar_diretorias_dos_clubes)'),
          ('linha da diretoria [this] + 73 × i: +0 , +2..+5 marcas, +6/+8/+0xa/+0xc confianças, +0xe, +0x10..+0x16, +0x18 data (hoje), +0x20 data, +0x28, +0x2c, +0x30 reputação² × 0,25 (double), +0x38, +0x3c, +0x40, +0x41 data; valores; grava (iniciar_diretorias_dos_clubes)',
           'board row [this] + 73 × i: +0, +2..+5 flags, +6/+8/+0xa/+0xc confidences, +0xe, +0x10..+0x16, +0x18 date (today), +0x20 date, +0x28, +0x2c, +0x30 reputation squared × 0.25 (double), +0x38, +0x3c, +0x40, +0x41 date; values; writes (iniciar_diretorias_dos_clubes)'),
          ('preferências do técnico (pessoa +0x69) +0xa nota da diretoria; valor; lê e baixa a 0,75 × reputação (iniciar_diretorias_dos_clubes)',
           'manager preferences (staff +0x69) +0xa board rating; value; reads and lowers to 0.75 × reputation (iniciar_diretorias_dos_clubes)'),
          ('MANAGER_MANAGER +0x14: a lista das apelações (+0 id da pessoa, +4 resultado, +5 data, +0xd..+0x10 regras, +0x11..+0x18 campos do resultado); ponteiro; lê e tira (apelacoes_de_suspensao)',
           'MANAGER_MANAGER +0x14: the appeal list (+0 staff id, +4 result, +5 date, +0xd..+0x10 rule sets, +0x11..+0x18 result fields); pointer; reads and removes (apelacoes_de_suspensao)'),
          ('partida +0x1c/+0x20 clubes, +0x28/+0x2a data, +0x3f fatia, +0x40/+0x41 marcas, +0x43; valores; lê (pode_ver_a_partida)',
           'fixture +0x1c/+0x20 clubs, +0x28/+0x2a date, +0x3f slice, +0x40/+0x41 flags, +0x43; values; reads (pode_ver_a_partida)'),
          ('[0xb64738] grupos das nações-chave; valor; lê (pode_ver_a_partida)',
           '[0xb64738] key nation groups; value; reads (pode_ver_a_partida)')]
INTERESSA = ['CMRTE', 'SGE']
