# Pôr o jogador à venda e nas listas de alvos e o prêmio por jogos (shortlist_manager.cpp, transfer_manager.cpp):
# pôr o jogador nas listas dos clubes (Shortlist_Manager::add_player_to_club_shortlists 0x842c00, 16 chamadas:
# contratos, mídia, notícias, transferências e a 0x842320; até 5 clubes, pela 0x842e10 que fica na original), pôr o
# jogador à venda (Shortlist_Manager::transfer_list_player 0x842320, da reavaliação do elenco 0x83dc42 e das telas de
# transferência: marca o contrato à venda, dois sorteios(100) contra a lealdade para o jogador desgostar do técnico, e
# com notícia as listas e a notícia 0x4d71b0) e pagar o prêmio por jogos da transferência
# (TRANSFER_MANAGER::pay_appearance_money 0x8cb340, pela 0x8c39f0 e pela 0x8c91e0: notícia 0x8db8a0, totais 0x448ad0,
# o clube antigo recebe e o atual paga). As três gravam o contrato em [0xae1080] como a original (a 0x842320 não).
# Iguais no cm0102-1080.exe. Comparadores: a 0x842c00 só a original; a 0x842320 só a original com o plano (o contrato
# fica à venda); a 0x8cb340 só a original com o plano da devolução (contrato, clube, registro e jogos).
SAVE = False
ROTINAS = [('por_jogador_nas_listas_dos_clubes', 0x842c00, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('por_jogador_a_venda', 0x842320, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('pagar_premio_por_jogos', 0x8cb340, [0x83, 0xec, 0x10, 0x53, 0x55])]
DOCS = ('pôr o jogador à venda, pô-lo nas listas de alvos dos clubes e pagar o prêmio por jogos da transferência',
        'transfer listing the player, putting him on the clubs\' shortlists and paying the transfer\'s appearance money',
        [('por_jogador_nas_listas_dos_clubes', 'Shortlist_Manager::add_player_to_club_shortlists', '0x842c00', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'com os filtros do jogador (ficha, regen marcado, cargo, fim de carreira, nação, intocável, chegada há menos de 200 dias sem estar à venda), tenta pô-lo na lista de cada clube do vetor (0x842e10) até 5 clubes, fora seleções, o clube do contrato e o dele',
          'with the player\'s filters (record, marked regen, job, career end, nation, untouchable, arrived less than 200 days ago without being listed), tries to put him on the list of each club in the array (0x842e10) up to 5 clubs, except national teams, the contract\'s club and his own'),
         ('por_jogador_a_venda', 'Shortlist_Manager::transfer_list_player', '0x842320', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'marca o contrato à venda; o jogador pode passar a desgostar do técnico (gosta do clube ou lealdade contra dois sorteios de 100); com notícia, põe nas listas dos clubes e dá a notícia',
          'marks the contract as transfer listed; the player may come to dislike the manager (likes the club or loyalty against two draws of 100); with news, puts him on the clubs\' lists and posts the news'),
         ('pagar_premio_por_jogos', 'TRANSFER_MANAGER::pay_appearance_money', '0x8cb340', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'acha o registro do prêmio por jogos (clube ou seleção) e, quando os jogos batem, dá a notícia, soma aos totais e o clube antigo recebe o valor que o atual paga; devolve 1 se pagou',
          'finds the appearance money record (club or national team) and, when the appearances match, posts the news, adds to the totals and the former club receives the amount the current one pays; returns 1 when paid')])
CAMPOS = [('jogador +0x61: a ficha de posições (+0xb nota, 16 bits com sinal); ponteiro; lê (por_jogador_nas_listas_dos_clubes)',
           'player +0x61: the position record (+0xb rating, signed 16 bits); pointer; reads (por_jogador_nas_listas_dos_clubes)'),
          ('[0xae2d0c] registro de 79 bytes +0xb: bits 0x30 (0x10 marca) e o nibble baixo (quantos clubes já o têm na lista); valor; lê (por_jogador_nas_listas_dos_clubes)',
           '[0xae2d0c] 79-byte record +0xb: bits 0x30 (0x10 mark) and the low nibble (how many clubs already list him); value; reads (por_jogador_nas_listas_dos_clubes)'),
          ('jogador +0x3d, +0x24, +0x28, +0x1a, +0x39, +0x3e/+0x40: cargo, fim de carreira, nação, clube, data de chegada; valor e ponteiro; lê (por_jogador_nas_listas_dos_clubes)',
           'player +0x3d, +0x24, +0x28, +0x1a, +0x39, +0x3e/+0x40: job, career end, nation, club, arrival date; value and pointer; reads (por_jogador_nas_listas_dos_clubes)'),
          ('[0xae2d1c] e [0xae235c]: o vetor dos ids dos clubes e a conta; id; lê (por_jogador_nas_listas_dos_clubes)',
           '[0xae2d1c] and [0xae235c]: the club id array and its count; id; reads (por_jogador_nas_listas_dos_clubes)'),
          ('contrato +0x4e bit 1 e +0x4f nibble alto (7 = à venda); valor; lê e grava (por_jogador_a_venda), lê (por_jogador_nas_listas_dos_clubes)',
           'contract +0x4e bit 1 and +0x4f high nibble (7 = listed); value; reads and writes (por_jogador_a_venda), reads (por_jogador_nas_listas_dos_clubes)'),
          ('contrato +0x44: a lealdade (byte com sinal); valor; lê (por_jogador_a_venda); +0x45; valor; lê (por_jogador_nas_listas_dos_clubes)',
           'contract +0x44: the loyalty (signed byte); value; reads (por_jogador_a_venda); +0x45; value; reads (por_jogador_nas_listas_dos_clubes)'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (por_jogador_nas_listas_dos_clubes, pagar_premio_por_jogos)',
           '[0xae1080] the last contract found; pointer; writes (por_jogador_nas_listas_dos_clubes, pagar_premio_por_jogos)'),
          ('TRANSFER_MANAGER +0x88c/+0x894 e +0x898/+0x8a0: os registros de prêmio por jogos (0xd bytes: +0 pessoa, +4 valor, +8 jogos, +9 clube antigo) de clube e de seleção, e as contas; ponteiro e valor; lê (pagar_premio_por_jogos)',
           'TRANSFER_MANAGER +0x88c/+0x894 and +0x898/+0x8a0: the appearance money records (0xd bytes: +0 person, +4 amount, +8 appearances, +9 former club) for club and national team, and the counts; pointer and value; reads (pagar_premio_por_jogos)')]
INTERESSA = ['CMRTE', 'CMScout', 'SGE']
