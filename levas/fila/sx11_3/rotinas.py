# game_config.cpp, parte 1 (leitor sx11, rodada 5, 23/11/2026): o objeto de configuração do jogo (Config, 0x31b bytes),
# os métodos que a caixa Game Settings usa sobre a cópia (tel12_1, ainda na fila, chama todos pelo endereço).
# Convenções conferidas na desmontagem (todas thiscall):
# - 0x5d7ad0 Config::Config(origem) ret 4 (construtor de cópia; strncpy 0x9466d0 cdecl(destino, origem, n) com 0x103,
#   0xff, 0xff e o terminador posto à mão em +0x112, +0x212, +0x31a; devolve this);
# - 0x5d7c60 set_option(valor) ret 4: tabela de saltos em 0x5d7f80 (índices em 0x5d7f94 para valor-1 em 0..0x7f) e
#   comparações para o resto; quadro SEH só pelo operator new 0x30 do SoundPlayer (0x84ca80 thiscall); a caixa de erro
#   da linha 0x255 (strtok 0x944cff, versão 0x90d130, sprintf "v%s %s.%s %d", 0x5e8290, [0xb67a34] = 0) = o
#   show_error_box já em Rust; som: play_file 0x84cbc0 thiscall(2, caminho, 0) ret 0xc (o 0 é o 3o push do
#   build_file_path 0x90bde0, cdecl de 2), stop_all 0x84cb10, ~SoundPlayer 0x84caf0, delete; a recursão
#   set_option(0x100) passa pelo desvio;
# - 0x5d8020 set_language(idioma) ret 4: langlib_is_language_available 0x66ea50 cdecl; set_default_currency 0x5d8310;
#   o teste da fonte permitida e a escolha da primeira permitida são o is_font_allowed_for_language em linha (a
#   tabela 0xae2d28, 10 entradas de 0x68); nome da fonte 0x5b2040 cdecl(índice, saída, n), quantas 0x5b1fc0 cdecl(),
#   0x5e4a60 cdecl(índice); 0x66eb40 cdecl(idioma, 1); ScreenManager::redraw_screen 0x802d00 thiscall(1, 1) ret 8;
# - 0x5d8270 set_currency(c) ret 4 (al com sinal, 0..0x19; erro linha 0x293); 0x5d8310 set_default_currency() ret 0
#   (idioma 1 sai com eax = 1 sem o mov: devolve 1 igual); 0x5d83e0 set_auto_save_interval(dias) ret 4 (ax com sinal,
#   7..0x16d; erro linha 0x2d5); 0x5d8b40 get_font_index() ret 0; 0x5d8b50 set_font(índice) ret 4 (devolve o eax do
#   0x5b2040);
# - 0x5d8b80 is_font_allowed_for_language cdecl(idioma, nome) (o mapa diz o propósito certo).
# Rodam: a caixa Game Settings (Setup Game -> Game Settings; no jogo, pela barra lateral) e a partida do programa
# (set_option e set_default_currency pelo Config::Config) e a criação do jogo pelo auto_setup (set_option 0x8272ee..,
# set_language 0x82775d). Tipo: telas (preferências, sem efeito na simulação).
MOTOR = True
TELAS = True
ROTINAS = [('config_construct_copy', 0x5d7ad0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('config_set_option', 0x5d7c60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('config_set_language', 0x5d8020, [0x81, 0xec, 0xd4, 0x00, 0x00, 0x00]),
           ('config_set_currency', 0x5d8270, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('config_set_default_currency', 0x5d8310, [0x8b, 0x41, 0x08, 0x85, 0xc0]),
           ('config_set_auto_save_interval', 0x5d83e0, [0x66, 0x8b, 0x44, 0x24, 0x04]),
           ('config_get_font_index', 0x5d8b40, [0x8b, 0x81, 0x17, 0x02, 0x00, 0x00]),
           ('config_set_font', 0x5d8b50, [0x8b, 0x44, 0x24, 0x04, 0x68, 0x00, 0x01, 0x00, 0x00]),
           ('is_font_allowed_for_language', 0x5d8b80, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10])]
DOCS = ('as preferências do jogo (1): a cópia, as opções, o idioma, a moeda, a gravação automática e a fonte',
        'the game settings (1): the copy, the options, the language, the currency, the auto save and the font',
        [('config_construct_copy', 'Config::Config', '0x5d7ad0', 'game_config.cpp', 'thiscall',
          'constrói a cópia de trabalho da configuração para a caixa Game Settings: guarda a configuração de origem em +0 e copia opções, idioma, moeda, intervalo de gravação, pasta de dados, nome do jogo salvo, a marca de gravado, o índice e o nome da fonte',
          'builds the working copy of the settings for the Game Settings box: keeps the source settings at +0 and copies options, language, currency, save interval, data folder, saved game name, the saved flag, the font index and name'),
         ('config_set_option', 'Config::set_option', '0x5d7c60', 'game_config.cpp', 'thiscall',
          'liga um bit de opção apagando os outros do mesmo grupo (tela em janela ou cheia, fundos, som, gravação automática, impressão...); o grupo do som liga o tocador (toca intro2.wav; sem DirectSound, a caixa de erro e o som desligado) ou o desliga; valor que não é opção: a caixa de erro da linha 0x255 e 0',
          'sets one option bit clearing the others of its group (windowed or full screen, backgrounds, sound, auto save, printing...); the sound group starts the player (plays intro2.wav; without DirectSound, the error box and sound off) or stops it; a value that is not an option: the line 0x255 error box and 0'),
         ('config_set_language', 'Config::set_language', '0x5d8020', 'game_config.cpp', 'thiscall',
          'escolhe o idioma (0 se a biblioteca não o tem), põe a moeda padrão dele, troca a fonte pela primeira das 20 primeiras que o idioma permite (ou a padrão, -2) quando a atual não é permitida, carrega os textos do idioma e redesenha a tela',
          'selects the language (0 when the library lacks it), sets its default currency, switches the font to the first of the first 20 the language allows (or the default, -2) when the current one is not allowed, loads the language texts and redraws the screen'),
         ('config_set_currency', 'Config::set_currency', '0x5d8270', 'game_config.cpp', 'thiscall',
          'guarda a moeda 0 a 25; outra: a caixa de erro da linha 0x293 e 0',
          'stores currency 0 to 25; another: the line 0x293 error box and 0'),
         ('config_set_default_currency', 'Config::set_default_currency', '0x5d8310', 'game_config.cpp', 'thiscall',
          'a moeda vira a padrão do idioma: 1 -> 24, 2 a 5, 7 e 9 -> 7, 6 -> 6, 8 -> 16, 10 -> 21, 11 -> 15, 12 -> 25, os outros -> 0',
          'the currency becomes the language default: 1 -> 24, 2 to 5, 7 and 9 -> 7, 6 -> 6, 8 -> 16, 10 -> 21, 11 -> 15, 12 -> 25, the others -> 0'),
         ('config_set_auto_save_interval', 'Config::set_auto_save_interval', '0x5d83e0', 'game_config.cpp', 'thiscall',
          'guarda o intervalo da gravação automática de 7 a 365 dias; outro: a caixa de erro da linha 0x2d5 e 0',
          'stores the auto save interval of 7 to 365 days; another: the line 0x2d5 error box and 0'),
         ('config_get_font_index', 'Config::get_font_index', '0x5d8b40', 'game_config.cpp', 'thiscall',
          'devolve o índice da fonte',
          'returns the font index'),
         ('config_set_font', 'Config::set_font', '0x5d8b50', 'game_config.cpp', 'thiscall',
          'guarda o índice da fonte e copia o nome do arquivo dela',
          'stores the font index and copies its file name'),
         ('is_font_allowed_for_language', 'is_font_allowed_for_language', '0x5d8b80', 'game_config.cpp', 'cdecl',
          'diz se a fonte é permitida para o idioma: sim quando a tabela de allowed_fonts.txt não tem entrada para o idioma ou quando uma entrada dele tem exatamente o nome da fonte',
          'tells whether the font is allowed for the language: yes when the allowed_fonts.txt table has no entry for the language or when one of its entries has exactly the font name')])
CAMPOS = []
INTERESSA = []
