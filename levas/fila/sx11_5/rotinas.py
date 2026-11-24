# game.cpp (leitor sx11, rodada 5, 23/11/2026): a partida do programa, o turno dos humanos e o passo das partidas de
# fundo. Usa config_offsets e config_fns do sx11_3: vai depois dele.
# Convenções conferidas na desmontagem:
# - 0x5cc6d0 game_init_from_command_line cdecl(linha) -> 1: quadro SEH pelos dois new; nulo = "" (0xad9c64); strcpy
#   sem limite para 0xae2c98 e _strlwr 0x9532a6 (o push do argumento vem antes do rep movs); new 0x31b e Config::Config
#   0x5d72c0 thiscall -> [0xae3138]; new 0x52080 e HallOfFame::HallOfFame 0x5ef0b0 thiscall -> [0xb5ded8];
#   [0xb67a30] = 1; com a linha não vazia, strstr 0x945a60(linha, "nomemcheck") == 0 -> [0x9cd1fc] = 0. Chamada uma vez
#   pelo WinMain (0x68d45e), na pilha do jogo: desviada, na simulação ela e o Config::Config rodam na pilha particular
#   (igual no modo original, que desvia as mesmas rotinas por repasse).
# - 0x5cde40 game_run_human_turn cdecl(&próxima) -> al: count_human_managers 0x838d10 cdecl(0); [0xb67a0c];
#   open_job_information_screen 0x6b0a10; ScreenManager::monitor 0x7fc7e0 thiscall(0xb74450); disco 0x7a1820(0) e
#   memória 0x7a1710() >= 25; as últimas 16 linhas da tabela de pessoas (contagem - k - 1), +0x5f; News::has_urgent_items
#   0x787350 thiscall([0xb7431c], pessoa, 1/0) ret 8; HUMAN_MANAGER::is_on_holiday 0x5fd210 thiscall([0xb63c98],
#   pessoa) ret 4; a palavra [0xdd6bec] comparada com ax = 0; news_screens_open_news_screen 0x788010 cdecl(pessoa, 1/0);
#   open_manager_club_screen 0x4542f0 cdecl(pessoa, 0, 0, 0, 0); setup_open_manager_status_screen 0x81fea0; Date
#   add_weeks 0x54c770 thiscall(hoje, saída, 1) ret 8 e operator= 0x417c10 thiscall(próxima, temporária, origem) ret 8.
#   Chamada pelo laço do dia (0x5cdbf9, 0x5cdc0a), na pilha do jogo.
# - 0x5d61b0 game_match_day_step_callback cdecl(): o laço do dia põe o endereço em 0xdd6956 (o chamador indireto não
#   aparece nas referências); MATCH_DAY::network_step 0x6b55c0 thiscall([0xb67a50], 0) ret 4.
# DEFEITOS do original (para docs/DEFEITOS_DO_ORIGINAL.md):
# - 0x5cc7ab: o teste do "nomemcheck" é invertido: a chave mantém a checagem de memória ligada e qualquer outra linha de
#   comando não vazia (ex.: "-windowed") a desliga;
# - 0x5cc710: a linha de comando é copiada sem limite para 0xae2c98 (100 bytes até 0xae2cfb): uma linha mais longa
#   sobrescreve o byte de fase do jogo 0xae2cfc e o que vem depois.
# Tipo: telas (partida do programa, as telas do turno e o passo das partidas de fundo durante as telas).
MOTOR = True
TELAS = True
ROTINAS = [('game_init_from_command_line', 0x5cc6d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('game_run_human_turn', 0x5cde40, [0x83, 0xec, 0x1c, 0x53, 0x55]),
           ('game_match_day_step_callback', 0x5d61b0, [0x8b, 0x0d, 0x50, 0x7a, 0xb6, 0x00])]
DOCS = ('a partida do programa, o turno dos humanos e o passo das partidas de fundo',
        'the program start, the human turn and the background match step',
        [('game_init_from_command_line', 'game_init_from_command_line', '0x5cc6d0', 'game.cpp', 'cdecl',
          'guarda a linha de comando em minúsculas, cria a configuração (Config::Config) e o hall da fama, marca o programa iniciado e, com linha de comando não vazia sem "nomemcheck", desliga a checagem de memória (o teste está invertido no original); devolve 1',
          'stores the command line in lower case, creates the settings (Config::Config) and the hall of fame, marks the program initialised and, with a non-empty command line without "nomemcheck", switches the memory check off (the test is inverted in the original); returns 1'),
         ('game_run_human_turn', 'game_run_human_turn', '0x5cde40', 'game.cpp', 'cdecl',
          'as telas do turno dos humanos: sem humanos (espectador), a tela de informação do emprego; num dia novo com 25 MB de disco e memória, quem tem notícia urgente e não está de férias vê as notícias e os outros a tela do clube (sem emprego, as notícias), sem notícia urgente nenhuma não há turno (0); senão todos veem a tela do clube ou as notícias (urgentes ou sem emprego) e, sem humano nenhum, a tela de situação do técnico; a próxima data vira hoje mais uma semana e roda o laço das telas (1)',
          'the human managers turn screens: no humans (spectator), the job information screen; on a new day with 25 MB of disk and memory, whoever has urgent news and is not on holiday sees the news and the others the club screen (jobless, the news), with no urgent news at all there is no turn (0); otherwise everyone sees the club screen or the news (urgent or jobless) and, with no human at all, the manager status screen; the next date becomes today plus a week and the screen loop runs (1)'),
         ('game_match_day_step_callback', 'game_match_day_step_callback', '0x5d61b0', 'game.cpp', 'cdecl',
          'a função de espera que o laço do dia instala: com um dia de partidas, avança um passo das partidas de fundo',
          'the idle callback the day loop installs: with a match day, moves the background matches one step')])
CAMPOS = []
INTERESSA = []
