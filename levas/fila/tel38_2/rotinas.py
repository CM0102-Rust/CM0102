# A base da interface (2): os registros dos técnicos no gerente de telas (scrman.cpp; leitor tel38, rodada das telas,
# 21/11/2026): as chaves de um canal, as marcas de ausente e de pronto pela chave, os canais ociosos, o canal de um
# técnico, trocar as chaves de dois técnicos, passar a vez a um técnico e encerrar a sessão de rede.
# Usa os auxiliares do tel29_1 (screen_record, screen_history).
# Convencoes conferidas na desmontagem (0x7ff5f0..0x7ff62a, 0x7ff810..0x7ff8e9, 0x7ffcd0..0x7ffff1), todas thiscall,
# ecx = o gerente (0xb74450): get_channel_manager_keys ret 0xc (vetor de chaves, ponteiro da contagem short, canal
# short; devolve quantas escreveu); is_manager_away, is_manager_done, is_player_done (jogador short) e
# find_manager_channel ret 4 (este devolve short: com chave 0 a metade alta e a do eax do chamador);
# set_manager_done ret 8 (nada devolvido: o chamador nao le eax); count_idle_channels ret 0;
# swap_manager_keys ret 8 (devolve 1 ou 0); switch_to_manager ret 4 (nada devolvido: os chamadores nao leem eax);
# end_network_session ret 0 (0x7fc690 thiscall no gerente; 0x77a890 thiscall na sessao, depois operator delete).
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Comparadores: os leitores pelo valor; get_channel_manager_keys lado a lado na contagem e nas chaves escritas;
# set_manager_done lado a lado nos 16 canais; swap_manager_keys e switch_to_manager lado a lado no canal atual;
# end_network_session so a original (fecha e libera a sessao), conferindo que a sessao ficou nula.
TELAS = True
ROTINAS = [('screen_manager_get_channel_manager_keys', 0x7ffd80, [0x0f, 0xbf, 0x54, 0x24, 0x0c]),
           ('screen_manager_is_manager_away', 0x7ffdd0, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('screen_manager_is_manager_done', 0x7ffe40, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('screen_manager_set_manager_done', 0x7ffeb0, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('screen_manager_is_player_done', 0x7fff20, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('screen_manager_count_idle_channels', 0x7fff70, [0x56, 0x33, 0xc0, 0x8d, 0x91, 0xee, 0x07, 0x00, 0x00]),
           ('screen_manager_find_manager_channel', 0x7fffa0, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('screen_manager_swap_manager_keys', 0x7ff810, [0x83, 0xec, 0x08, 0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00]),
           ('screen_manager_switch_to_manager', 0x7ffcd0, [0x53, 0x56, 0x8b, 0xf1, 0x57]),
           ('screen_manager_end_network_session', 0x7ff5f0, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x40, 0x38, 0x00, 0x00])]
DOCS = ('a base da interface (2): as chaves de um canal, as marcas de ausente e de pronto, os canais ociosos, o canal de um técnico, trocar chaves, passar a vez e encerrar a sessão de rede',
        "the interface base (2): a channel's keys, the away and done flags, the idle channels, a manager's channel, swapping keys, switching managers and ending the network session",
        [('screen_manager_get_channel_manager_keys', 'ScreenManager::get_channel_manager_keys', '0x7ffd80', 'scrman.cpp', 'thiscall, ret 0xc',
          'as chaves dos técnicos dos registros de um canal, e quantos são',
          "the manager keys of a channel's records, and how many there are"),
         ('screen_manager_is_manager_away', 'ScreenManager::is_manager_away', '0x7ffdd0', 'scrman.cpp', 'thiscall, ret 4',
          'se o técnico com a chave está ausente (o primeiro registro com ela nos 16 canais); sem registro conta como ausente',
          'whether the manager with the key is away (the first record with it in the 16 channels); no record counts as away'),
         ('screen_manager_is_manager_done', 'ScreenManager::is_manager_done', '0x7ffe40', 'scrman.cpp', 'thiscall, ret 4',
          'se o técnico com a chave já terminou o dia; sem registro conta como pronto',
          'whether the manager with the key is done for the day; no record counts as done'),
         ('screen_manager_set_manager_done', 'ScreenManager::set_manager_done', '0x7ffeb0', 'scrman.cpp', 'thiscall, ret 8',
          'marca o técnico com a chave como pronto (ou não) para o dia; nada sem registro',
          'marks the manager with the key as done (or not) for the day; nothing without a record'),
         ('screen_manager_is_player_done', 'ScreenManager::is_player_done', '0x7fff20', 'scrman.cpp', 'thiscall, ret 4',
          'se um jogador da rede terminou: 0 quando um canal com registros, cujo primeiro registro é dele, ainda não está pronto',
          'whether a network player is done: 0 when a channel with records whose first record is his is not done yet'),
         ('screen_manager_count_idle_channels', 'ScreenManager::count_idle_channels', '0x7fff70', 'scrman.cpp', 'thiscall',
          'quantos canais estão ociosos: sem registros ou com o primeiro registro ausente',
          'how many channels are idle: with no records or with the first record away'),
         ('screen_manager_find_manager_channel', 'ScreenManager::find_manager_channel', '0x7fffa0', 'scrman.cpp', 'thiscall, ret 4',
          'o canal do técnico com a chave, ou -1 (também para a chave 0)',
          'the channel of the manager with the key, or -1 (also for key 0)'),
         ('screen_manager_swap_manager_keys', 'ScreenManager::swap_manager_keys', '0x7ff810', 'scrman.cpp', 'thiscall, ret 8',
          'troca as chaves de dois técnicos do canal atual (o último registro com cada chave); 1 se os dois existem, senão 0',
          'swaps the keys of two managers of the current channel (the last record with each key); 1 when both exist, otherwise 0'),
         ('screen_manager_switch_to_manager', 'ScreenManager::switch_to_manager', '0x7ffcd0', 'scrman.cpp', 'thiscall, ret 4',
          'o técnico com a chave vira o atual do canal (troca de lugar com o registro 0) quando não está ausente nem pronto',
          "the manager with the key becomes the channel's current one (swaps places with record 0) when he is neither away nor done"),
         ('screen_manager_end_network_session', 'ScreenManager::end_network_session', '0x7ff5f0', 'scrman.cpp', 'thiscall',
          'encerra a sessão de rede, se houver: a despedida, o destrutor e a liberação, e o ponteiro zerado',
          'ends the network session, if any: the farewell, the destructor and the free, and the pointer cleared')])
