# A base da interface (5): o que sobrou da base no gerente de telas (scrman.cpp; leitor tel38b, rodada das telas,
# 21/11/2026): o retângulo de um widget, fechar todas as telas, abrir uma tela em cada registro e os sons.
# Usa os auxiliares do tel29_1 (screen_record, screen_history, screen_node_fields, c_string_len, SCRMAN_CPP) e do
# tel38_4 (scrman_net_put, scrman_widget_packet, scrman_deliver_packet, scrman_expected_packet,
# compare_scrman_set_widget); show_error_at do motor.
# Convencoes conferidas na desmontagem (0x800890..0x800d23, 0x803d80..0x803f2f, 0x7fe4a0..0x7fe5f7,
# 0x8025e0..0x802cfd), todas thiscall, ecx = o gerente (0xb74450): set_widget_rect ret 0x14 (widget, x1, y1, x2, y2:
# o pacote 8, widget, subcomando 1, quatro words); close_all_screens ret 0 -> 1 ou 0 (os chamadores leem eax; as
# funcoes de saida e de abandono cdecl sem argumentos; Display::reset 0x55f7c0 thiscall(1) ret 4; free_all_screens
# 0x7fef10 thiscall); open_screen_for_each_record ret 0x18 (chave, build, input, flag, leave, exit) -> 1 (open_screen
# 0x7fe600 thiscall ret 0x14; utils_swap_bytes 0x90ba90 cdecl); play_sound ret 0x10 (out byte*, pan, arquivo,
# repetir; SoundPlayer::play_file 0x84cbc0 thiscall ret 0xc -> al, set_slot_volume 0x84ccc0 ret 8); is_sound_playing
# ret 4 -> eax (is_slot_playing 0x84cc80 ret 4); stop_sound ret 4 (stop_all 0x84cb10 ret 0, stop_slot 0x84cb70 ret 4);
# on_remote_sound_started ret 8 (slot, out); on_remote_sound_finished ret 4. O tocador de som em 0xdd7eec (0 sem
# som); as marcas dos sons remotos em +0x26250a (16 canais x 10 dwords). Os pacotes remotos: 0xa (out dword, pan,
# repetir, o nome com o tamanho em word, copiado sem conferir o espaco) e 0xb (o slot); falha de envio "scrman.cpp
# 2992" / "3043".
# O cm0102-1080.exe reescreve a entrada do set_widget_rect (0x800898..0x80089f: chama 0x601f68, que escala as quatro
# coordenadas para a tela grande antes): a entrada e so do exe original (EXE). As outras sao iguais no 1080.
# Defeitos do original: close_all_screens com telas marcadas deixa o no atual no no que bloqueia; e, sem bloqueio, o
# passeio deixa o no atual em 0 e as funcoes de saida, a de abandono e o reset do Display nao rodam [inferencia];
# open_screen_for_each_record roda os registros (a troca final so desfaz com dois); play_sound copia o nome sem
# conferir o espaco.
# Comparadores: set_widget_rect, play_sound e stop_sound so a original, conferindo o pacote (e as marcas, e o slot 0xff
# sem tocador); close_all_screens e open_screen_for_each_record so a original, conferindo o plano (historico vazio /
# ACTIVE_FLAG; o canal restaurado); is_sound_playing pelo valor; on_remote_sound_started/finished lado a lado.
TELAS = True
EXE = {0x800890: 'original'}
ROTINAS = [('screen_manager_set_widget_rect', 0x800890, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_close_all_screens', 0x803d80, [0x56, 0x8b, 0xf1, 0x0f, 0xbf, 0x86, 0x2a, 0x38, 0x00, 0x00]),
           ('screen_manager_open_screen_for_each_record', 0x7fe4a0, [0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('screen_manager_play_sound', 0x8025e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_is_sound_playing', 0x8029f0, [0x8b, 0x54, 0x24, 0x04, 0x80, 0xfa, 0xff]),
           ('screen_manager_stop_sound', 0x802a50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('screen_manager_on_remote_sound_started', 0x802cb0, [0x8a, 0x54, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x08]),
           ('screen_manager_on_remote_sound_finished', 0x802ce0, [0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00])]
DOCS = ('a base da interface (5): o retângulo de um widget, fechar todas as telas, abrir uma tela em cada registro e os sons',
        "the interface base (5): a widget's rectangle, closing every screen, opening a screen on each record and the sounds",
        [('screen_manager_set_widget_rect', 'ScreenManager::set_widget_rect', '0x800890', 'scrman.cpp', 'thiscall, ret 0x14',
          'o retângulo de um widget da tela atual: o pacote (comando 8, o widget, o subcomando 1, x1, y1, x2, y2 em words) aplicado ao Display local, ao de eco ou mandado ao jogador remoto; nada para o widget −1',
          "a widget's rectangle on the current screen: the packet (command 8, the widget, sub-command 1, x1, y1, x2, y2 as words) applied to the local Display, to the echo one or sent to the remote player; nothing for widget −1"),
         ('screen_manager_close_all_screens', 'ScreenManager::close_all_screens', '0x803d80', 'scrman.cpp', 'thiscall',
          'fecha todas as telas do técnico atual: as funções de saída de cada tela, a de abandono da atual, o Display esvaziado e o histórico liberado; uma tela de instância única sem função de saída bloqueia (0)',
          "closes every screen of the current manager: each screen's exit function, the current one's leave function, the Display emptied and the history freed; a single-instance screen with no exit function blocks it (0)"),
         ('screen_manager_open_screen_for_each_record', 'ScreenManager::open_screen_for_each_record', '0x7fe4a0', 'scrman.cpp', 'thiscall, ret 0x18',
          'abre uma tela em cada registro de cada canal cuja chave de técnico é a dada (todos com 0); o canal atual volta ao fim',
          "opens a screen on every record of every channel whose manager key is the given one (all of them with 0); the current channel comes back at the end"),
         ('screen_manager_play_sound', 'ScreenManager::play_sound', '0x8025e0', 'scrman.cpp', 'thiscall, ret 0x10',
          'toca um arquivo de som: localmente pelo tocador (volume 100), ou pelo comando de rede 0xa ao técnico remoto; o slot que toca vai para a saída (0xff nenhum)',
          'plays a sound file: locally through the sound player (volume 100), or by network command 0xa to a remote manager; the slot that plays it goes to the output (0xff none)'),
         ('screen_manager_is_sound_playing', 'ScreenManager::is_sound_playing', '0x8029f0', 'scrman.cpp', 'thiscall, ret 4',
          'se um slot de som ainda toca: nunca o 0xff; localmente a resposta do tocador (0 sem ele); para o técnico remoto a marca que o lado dele mandou',
          "whether a sound slot still plays: never 0xff; locally the sound player's answer (0 without one); for a remote manager the flag his side reported"),
         ('screen_manager_stop_sound', 'ScreenManager::stop_sound', '0x802a50', 'scrman.cpp', 'thiscall, ret 4',
          'para um slot de som (todos com 0xff), localmente ou pelo comando de rede 0xb, e zera as marcas de tocando',
          'stops a sound slot (every slot for 0xff), locally or by network command 0xb, and clears the playing flags'),
         ('screen_manager_on_remote_sound_started', 'ScreenManager::on_remote_sound_started', '0x802cb0', 'scrman.cpp', 'thiscall, ret 8',
          'o lado remoto informa o slot em que o som toca: vai para a saída e, salvo 0xff, a marca dele fica 1',
          'the remote side reports the slot the sound plays in: it goes to the output and, unless 0xff, its flag becomes 1'),
         ('screen_manager_on_remote_sound_finished', 'ScreenManager::on_remote_sound_finished', '0x802ce0', 'scrman.cpp', 'thiscall, ret 4',
          'o lado remoto informa que o som acabou: a marca do slot fica 0',
          "the remote side reports that the sound ended: the slot's flag becomes 0")])
