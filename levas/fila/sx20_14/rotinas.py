# O tocador de som e o teste de impressora (simulated_stats.cpp; leitor sx20, rodada 5, 24/11/2026): SoundPlayer
# (0x84ca80 a 0x84cf00, nove rotinas) e printer_is_installed (0x84c7f0). Rotinas inteiras (0x84ca80..0x84cae7 ret;
# 0x84caf0..0x84cb08 ret; 0x84cb10..0x84cb63 ret; 0x84cb70..0x84cbb0 ret 4; 0x84cbc0..0x84cc73 ret 0xc;
# 0x84cc80..0x84ccb9 ret 4; 0x84ccc0..0x84cd00 ret 8; 0x84cd10..0x84cef3 ret 8; 0x84cf00..0x84cf42 ret;
# 0x84c7f0..0x84c828 ret).
# Ficam de fora (sx20): 0x84c500 (thunk do atexit: mov ecx, 0xdd7168; jmp 0x453bc0), 0x84c7d0 (o inicializador
# estatico que registra o 0x84c7e0 no atexit, roda na iniciacao da CRT, antes do jogo) e 0x84c7e0 (corpo vazio: ret).
# Fica para outra rodada: 0x84c830 print_text_lines (592 bytes: a caixa de impressao do Windows, StartDoc/StartPage/
# TextOutA por pagina; so se prova com impressora).
# TELAS: a prova e o Lucas jogando comparando, com o som ligado nas opcoes: os sons das telas e da partida (o
# comentario com som, o apito, a torcida em laco), o volume, e trocar de tela parando os sons; o tocador nasce na
# carga do jogo (0x5d7a37) e morre ao sair. printer_is_installed: o botao de imprimir de uma tela de impressao.
# Convencoes conferidas na desmontagem (thiscall sobre o tocador; os metodos COM stdcall com o objeto primeiro):
# - 0x84ca80(tocador) -> tocador: 0x944c22 DirectSoundCreate(0, &+0x28, 0) stdcall, IDirectSound +0x18
#   SetCooperativeLevel(ds, [0xb67a2c], 1), +8 Release;
# - IDirectSoundBuffer: +0x48 Stop, +0x34 SetCurrentPosition(0), +8 Release, +0x24 GetStatus(&estado), +0x40 SetPan,
#   +0x3c SetVolume, +0x30 Play(0, 0, laco), +0x2c Lock(0, tamanho, &p1, &n1, 0, 0, 0), +0x4c Unlock(p1, n1, 0, 0);
#   IDirectSound +0xc CreateSoundBuffer(&desc, &vaga, 0);
# - 0x84cbc0(tocador, lado, caminho, laco) ret 0xc -> vaga ou 0xff; 0x84cd10 load_wav(tocador, vaga, caminho) ret 8
#   -> 1/0 com 0x945b00 fopen "rb", 0x945bfe fread, 0x945a09 fclose e 0x946fa0 (comparacao de 4/4/3 bytes);
# - 0x84c7f0 cdecl() -> 0/1: 0x944c10 EnumPrintersA(2, 0, 5, buf de 20 bytes, 0x14, &preciso, &quantos) stdcall.
# Defeitos do original: play_file pede volume 10000 (fora da faixa -10000..0 do DirectSound: falha e o som fica no
# volume cheio); load_wav larga sem Release o buffer cujo Lock ou Unlock falha, supoe o cabecalho canonico de 44
# bytes e grava block align 1; printer_is_installed com buffer de 20 bytes [inferencia: com qualquer impressora o
# EnumPrinters falha por falta de espaco e a resposta e nao].
# Comparadores: so a original no que mexe no DirectSound (criar, liberar, parar, tocar, volume, carregar); lado a
# lado na resposta das consultas (is_slot_playing, find_free_slot, printer_is_installed).
TELAS = True
ROTINAS = [('sound_player_sound_player', 0x84ca80, [0x56, 0x8b, 0xf1, 0x57, 0xb9, 0x0a, 0x00, 0x00, 0x00]),
           ('sound_player_destructor', 0x84caf0, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x28]),
           ('sound_player_stop_all', 0x84cb10, [0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x28]),
           ('sound_player_stop_slot', 0x84cb70, [0x8b, 0x41, 0x28, 0x56, 0x85, 0xc0]),
           ('sound_player_play_file', 0x84cbc0, [0x53, 0x56, 0x8b, 0xf1, 0x83, 0xcb, 0xff]),
           ('sound_player_is_slot_playing', 0x84cc80, [0x8b, 0x41, 0x28, 0x85, 0xc0]),
           ('sound_player_set_slot_volume', 0x84ccc0, [0x8b, 0x41, 0x28, 0x85, 0xc0]),
           ('sound_player_load_wav', 0x84cd10, [0x83, 0xec, 0x58, 0x53, 0x8b, 0xd9]),
           ('sound_player_find_free_slot', 0x84cf00, [0x51, 0x53, 0x56, 0x8b, 0xf1]),
           ('printer_is_installed', 0x84c7f0, [0x83, 0xec, 0x1c, 0x8d, 0x44, 0x24, 0x00])]
_F = 'simulated_stats.cpp'
DOCS = ('o tocador de som e o teste de impressora',
        'the sound player and the printer check',
        [('sound_player_sound_player', 'SoundPlayer::SoundPlayer', '0x84ca80', _F, 'thiscall',
          'monta o tocador: sem buffers, nao pronto; cria o DirectSound e poe o nivel cooperativo normal com a janela do jogo; falhando, libera e zera o dispositivo; senao marca pronto; devolve o tocador',
          'builds the player: no buffers, not ready; creates DirectSound and sets the normal cooperative level with the game window; on failure releases and clears the device; otherwise marks it ready; returns the player'),
         ('sound_player_destructor', 'SoundPlayer::~SoundPlayer', '0x84caf0', _F, 'thiscall',
          'libera o DirectSound e zera o ponteiro',
          'releases DirectSound and clears the pointer'),
         ('sound_player_stop_all', 'SoundPlayer::stop_all', '0x84cb10', _F, 'thiscall',
          'para, rebobina e libera cada um dos dez buffers (com dispositivo)',
          'stops, rewinds and releases each of the ten buffers (with a device)'),
         ('sound_player_stop_slot', 'SoundPlayer::stop_slot', '0x84cb70', _F, 'thiscall, ret 4',
          'para, rebobina e libera o buffer da vaga (0 a 9)',
          'stops, rewinds and releases the buffer of the slot (0 to 9)'),
         ('sound_player_play_file', 'SoundPlayer::play_file', '0x84cbc0', _F, 'thiscall, ret 0xc',
          'toca um WAV na primeira vaga vazia: nada (0xff) sem dispositivo, caminho ou vaga, ou com lado fora de 0 (esquerda, -5000), 1 (direita, 5000) e 2 (centro); carrega o arquivo na vaga (falhando, 0xff), poe o lado, o volume 10000 e toca, em laco se pedido; devolve a vaga',
          'plays a WAV in the first empty slot: nothing (0xff) without a device, path or slot, or with a side other than 0 (left, -5000), 1 (right, 5000) and 2 (centre); loads the file into the slot (failing, 0xff), sets the pan, volume 10000 and plays, looping if asked; returns the slot'),
         ('sound_player_is_slot_playing', 'SoundPlayer::is_slot_playing', '0x84cc80', _F, 'thiscall, ret 4',
          'diz se o buffer da vaga esta tocando ou em laco (0 sem dispositivo, vaga valida ou buffer)',
          'says whether the slot buffer is playing or looping (0 without a device, a valid slot or a buffer)'),
         ('sound_player_set_slot_volume', 'SoundPlayer::set_slot_volume', '0x84ccc0', _F, 'thiscall, ret 8',
          'poe o volume da vaga por uma porcentagem de 0 a 100: (porcentagem - 100) x 100 centesimos de decibel',
          'sets the slot volume from a percentage 0 to 100: (percentage - 100) x 100 hundredths of a decibel'),
         ('sound_player_load_wav', 'SoundPlayer::load_wav', '0x84cd10', _F, 'thiscall, ret 8',
          'le um WAV num buffer estatico novo da vaga: nada sem dispositivo ou arquivo; cabecalho sem RIFF, WAVE e fmt fecha e 0; o formato vem do cabecalho de 44 bytes (formato, 1 ou 2 canais, taxa, media de bytes = alinhamento x taxa, alinhamento 1, bits) e o tamanho e o dos dados, com bandeiras 0xe2; cria, trava, enche do arquivo e destrava (falha: zera a vaga, fecha e 0); devolve 1',
          'reads a WAV into a new static buffer of the slot: nothing without a device or a file; a header without RIFF, WAVE and fmt closes and 0; the format comes from the 44-byte header (tag, 1 or 2 channels, rate, average bytes = block align x rate, block align 1, bits) and the size is the data size, with flags 0xe2; creates, locks, fills from the file and unlocks (failure: clears the slot, closes and 0); returns 1'),
         ('sound_player_find_free_slot', 'SoundPlayer::find_free_slot', '0x84cf00', _F, 'thiscall',
          'a primeira das dez vagas com buffer que nao toca nem esta em laco; 0xff sem nenhuma, 0 sem dispositivo',
          'the first of the ten slots with a buffer that neither plays nor loops; 0xff when none, 0 without a device'),
         ('printer_is_installed', 'printer_is_installed', '0x84c7f0', _F, 'cdecl',
          'diz se o Windows lista ao menos uma impressora local (EnumPrintersA nivel 5 num buffer de 20 bytes)',
          'says whether Windows lists at least one local printer (EnumPrintersA level 5 into a 20-byte buffer)')])
