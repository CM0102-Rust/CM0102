# O gerenciador de telas: construir e destruir, fechar as telas de todos os canais, sair da sessao de rede e tirar o
# registro de um tecnico (network.h; leitor tel46, rodada 5, 23/11/2026). Usa screen_history, screen_node_fields,
# screen_manager, scrman_channel_record, channel_record_at, channel_count_at, scrman_channel_count,
# scrman_command_packet (tel38/tel40), SCRMAN_JOIN_HOOK/LEAVE_HOOK/NEXT_CHANNEL/SOUND_PLAYING/ECHO_DISPLAY,
# G_SCREEN_MANAGER_SLOT_RECORD_COUNTS, G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK, MENUBAR_SLOT_CONTINUE_TIME,
# DISPLAY_SNAPSHOT_POOL.
# Convencoes conferidas na desmontagem:
# - ScreenManager::ScreenManager 0x7fc380 thiscall(sm) -> sm; NetworkBuffer::NetworkBuffer 0x77b630 thiscall(buf,
#   0xc350) ret 4; Display::Display 0x55ef50 thiscall(d, canal) ret 4 (0 o local +0x3844, 1 o eco +0x132ea5).
#   Prologo push -1 / push 0x9610ac (quadro de excecao; a copia nao o abre).
# - ScreenManager::~ScreenManager 0x7fc530 thiscall(sm): close_all_screens 0x7fc5e0, disconnect_network 0x7fc690,
#   Network::~Network 0x77a890 + operator delete 0x944c94, Display::~Display 0x55efe0 (eco e local),
#   NetworkBuffer::~NetworkBuffer 0x77b6e0.
# - close_all_screens 0x7fc5e0 thiscall(sm): utils_swap_bytes 0x90ba90 cdecl(registro k, registro 0, 0x30); a funcao
#   de saida da tela (no +4) cdecl sem argumentos; free_all_screens 0x7fef10 thiscall(sm). Chamada pelo destrutor e
#   por 0x82ff5a.
# - disconnect_network 0x7fc690 thiscall(sm): display_free_guio_snapshot_pool 0x55f270 (cdecl; o original passa o
#   Display de eco em ecx, sem uso); a flag de anfitriao +0x3830; o pacote 4 (caixa network.h 133);
#   Network::send_packet 0x77ac30 thiscall(sessao, dados, tamanho, 0) ret 0xc; remove_manager_record(sm, chave, -1);
#   Network::release_slot_socket 0x77ac00 thiscall(sessao, word) ret 4.
# - remove_manager_record 0x7ff990 thiscall(sm, chave, canal) ret 8: canal -1 (word) = procurar pela chave (canais
#   0..15, registros do ultimo ao primeiro); display_clear_guio_snapshot 0x55f290 stdcall(canal); o gancho
#   0xdd6bf4 cdecl(jogador); open_screen 0x7fe600 thiscall(sm, 5 dwords do padrao +0x37d4) ret 0x14.
# Fora da lista (ver o relatorio): 0x7fc350 (o inicializador estatico do CRT: mov ecx, 0xb74450; call 0x7fc380;
# atexit(0x7fc370); roda uma vez no _initterm, antes do WinMain; desviar o construtor ja cobre a regra) e 0x7fc370
# (o thunk do atexit: mov ecx, 0xb74450; jmp 0x7fc530). O construtor e o destrutor de instancia ficam desviados: o
# desvio e instalado no DllMain (antes dos inicializadores do CRT) e o destrutor roda no exit(), com a DLL carregada;
# no exe com patch (instalacao adiada ao 1o DirectDrawCreate) o construtor ja rodou e so o destrutor passa pelo Rust.
# Defeito do original [inferencia]: disconnect_network repete enquanto o canal tem registros; se a chave do registro
# 0 estiver antes num canal anterior, remove la e o laco nao acaba.
# Iguais no cm0102-1080.exe (bytes conferidos).
TELAS = True
ROTINAS = [('screen_manager_construct', 0x7fc380, [0x6a, 0xff, 0x68, 0xac, 0x10, 0x96, 0x00]),
           ('screen_manager_destruct', 0x7fc530, [0x6a, 0xff, 0x68, 0xea, 0x10, 0x96, 0x00]),
           ('screen_manager_close_all_channels', 0x7fc5e0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('screen_manager_disconnect_network', 0x7fc690, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_remove_manager_record', 0x7ff990, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('o gerenciador de telas: construir e destruir, fechar as telas de todos os canais, sair da sessão de rede e tirar o registro de um técnico',
        "the screen manager: building and destroying it, closing every channel's screens, leaving the network session and removing a manager's record",
        [('screen_manager_construct', 'ScreenManager::ScreenManager', '0x7fc380', 'network.h', 'thiscall',
          'monta o buffer de comandos (50000 bytes), o Display local e o de eco, zera a sessão, os ganchos, os canais e os sons, e deixa um registro local no canal 0, sem tela',
          'builds the command buffer (50000 bytes), the local and echo Displays, clears the session, the hooks, the channels and the sounds, and leaves one local record in channel 0 with no screen'),
         ('screen_manager_destruct', 'ScreenManager::~ScreenManager', '0x7fc530', 'network.h', 'thiscall',
          'fecha as telas de todos os canais, sai da sessão de rede e a destrói, depois destrói os dois Displays e o buffer de comandos',
          "closes every channel's screens, leaves the network session and destroys it, then destroys both Displays and the command buffer"),
         ('screen_manager_close_all_channels', 'ScreenManager::close_all_screens', '0x7fc5e0', 'network.h', 'thiscall',
          'para cada registro de cada canal (trazido para o lugar 0), roda a função de saída da tela atual e libera todo o histórico de telas',
          'for each record of every channel (brought to place 0), runs the current screen\'s leave function and frees the whole screen history'),
         ('screen_manager_disconnect_network', 'ScreenManager::disconnect_network', '0x7fc690', 'network.h', 'thiscall',
          'com sessão, libera o retrato dos widgets; sendo o anfitrião, baixa a marca, manda o pacote de saída (4) a todos e tira os registros dos canais 1 a 15, fechando a conexão de cada jogador',
          "with a session, frees the widget snapshot; when hosting, lowers the flag, sends the quit packet (4) to everyone and removes the records of channels 1 to 15, closing each player's connection"),
         ('screen_manager_remove_manager_record', 'ScreenManager::remove_manager_record', '0x7ff990', 'network.h', 'thiscall, ret 8',
          'tira o registro (o 0 do canal dado, ou o da chave): fecha as telas dele, avisa o gancho de saída e sobe os registros de trás; o canal 0 vazio volta com um registro local na tela padrão, e um canal remoto vazio manda o pacote 4 ao jogador',
          "removes the record (record 0 of the given channel, or the key's): closes its screens, tells the leave hook and moves the records behind it up; channel 0 emptied comes back with one local record on the default screen, and a remote channel emptied sends packet 4 to its player")])
