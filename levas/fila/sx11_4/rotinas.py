# game_config.cpp, parte 2 (leitor sx11, rodada 5, 23/11/2026): a configuração viva na partida do programa e a
# aplicação com a gravação do game.cfg. Usa os nomes e auxiliares do sx11_3 (config_offsets, allowed_fonts, config_fns,
# config_font_name, config_first_allowed_font, config_start_sound, config_switch_sound, compare_config_side_by_side):
# vai depois dele.
# Convenções conferidas na desmontagem (thiscall, ret 0):
# - 0x5d72c0 Config::Config(): quadro SEH (o new 0x30 do SoundPlayer); set_defaults 0x5d7bc0; build_file_path 0x90bde0
#   cdecl(pasta, arquivo) (o push 1 antes é o 2o argumento do langlib_setup 0x66e790 cdecl(caminho, 1)); falha: a caixa
#   da linha 0x83, game_fatal_cleanup 0x5cc7d0, 0x5a8f60, exit(-1); 0x66f4d0; set_default_currency 0x5d8310; a tabela
#   0xae2d28 (10 x 0x68) com -1; fopen 0x945b00, fgets 0x94700c(buf, 100, f), 0x90bb70(buf), sscanf 0x946fd8(buf,
#   "%d, %s", &idioma, nome); a fonte do idioma é o laço em linha (= config_first_allowed_font); +0xd = 30;
#   0x90bea0 cdecl(0xad9c64 "", this+0xf); game.cfg copiado num local e aberto "rb"; fread 0x945bfe(p, n, 1, f) na
#   ordem mágica, opções, idioma, moeda (1 byte; o dword do local com lixo em cima vai para 0x43f7f0, que só lê o
#   byte), dias, pasta (0x104), índice e nome da fonte; idioma: 0x66ea50, 0x66eb40(idioma, 0), 0x5339a0(idioma, 0),
#   0x5d8310; pasta: strlen e 0x90bf90; fonte trocada só no índice (o nome lido fica); fclose 0x945a09; falha: remove
#   0x94643a do game.cfg; "-windowed" set_option(0x10), "-nosound" set_option(0x100), "-defaultfont" fonte -2 (strstr
#   0x945a60 em 0xae2c98); 0x5e2620 cdecl(800, 600, 2, fonte, (opções >> 3) & 1, título); falha: 0x5cc7d0, 0x5a8f60,
#   exit(0); som: new 0x30, 0x84ca80, sem dispositivo +0x2c a caixa e set_option(0x100) (sem tocar a intro);
#   0x41b3d0; 0x41bf80((opções >> 5) & 1).
# - 0x5d8480 Config::apply_and_save(): cópia: os campos de volta na origem (strcpy nos textos) e a chamada recursiva
#   (pelo desvio); viva: 0x66f4d0/0x66eb40(idioma, 1) e redraw 0x802d00(1, 1); 0x535920/[0xae23a8]/0x5339a0(idioma,
#   0); 0x4415e0/0x43f7f0 (ebx com lixo em cima, só o byte lido); a tela pelo [0x9cc2c4] e os bits 8/0x10 contra
#   0x5e2ea0; o som como no set_option sem a intro; os fundos 0x41bfa0/0x41bf80; fopen "wb", fwrite 0x945ce6(p, n,
#   1, f) com as linhas de erro 0x384, 0x38c..0x3c4; +0x213 = 1.
# Rodam: na partida do programa (Config::Config, pela game_init_from_command_line), a caixa Game Settings (Ok e Cancel),
# a barra lateral e a criação do jogo (0x8282ec). Tipo: telas.
MOTOR = True
TELAS = True
ROTINAS = [('config_construct', 0x5d72c0, [0x6a, 0xff, 0x68, 0xce, 0xa4, 0x95, 0x00]),
           ('config_apply_and_save', 0x5d8480, [0x6a, 0xff, 0x68, 0x0e, 0xa5, 0x95, 0x00])]
DOCS = ('as preferências do jogo (2): a configuração na partida do programa e a gravação do game.cfg',
        'the game settings (2): the settings at the program start and saving game.cfg',
        [('config_construct', 'Config::Config', '0x5d72c0', 'game_config.cpp', 'thiscall',
          'monta a configuração viva na partida do programa: opções padrão; a biblioteca de idiomas (sem ela, a caixa de erro e a saída); idioma e moeda padrão; a tabela de fontes permitidas do allowed_fonts.txt (o arquivo nunca é fechado) e a primeira fonte permitida; gravação a cada 30 dias e a pasta do usuário; o game.cfg (mágica, opções, idioma, moeda, dias, pasta, fonte; ilegível é apagado); as chaves -windowed, -nosound e -defaultfont da linha de comando; abre a tela 800x600 (falhando, sai), o som e os fundos',
          'builds the live settings at the program start: default options; the language library (without it, the error box and exit); language and default currency; the allowed fonts table of allowed_fonts.txt (the file is never closed) and the first allowed font; saving every 30 days and the user folder; game.cfg (magic, options, language, currency, days, folder, font; an unreadable one is deleted); the -windowed, -nosound and -defaultfont switches of the command line; opens the 800x600 display (failing, exits), the sound and the backgrounds'),
         ('config_apply_and_save', 'Config::apply_and_save', '0x5d8480', 'game_config.cpp', 'thiscall',
          'aplica a configuração: a cópia devolve os campos à origem e aplica a origem; a viva troca idioma (textos e nomes do banco), moeda, a tela (fonte ou modo diferentes; se o modo falha tenta o outro, falhando de novo sai), o som, os fundos e grava o game.cfg (falha: a caixa de erro da linha e 0)',
          'applies the settings: the copy gives its fields back to the source and applies the source; the live one changes language (texts and database names), currency, the display (another font or mode; if the mode fails it tries the other, failing again exits), sound, backgrounds and writes game.cfg (failure: the line error box and 0)')])
CAMPOS = []
INTERESSA = []
