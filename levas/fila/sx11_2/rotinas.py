# game.cpp (leitor sx11, rodada 5, 23/11/2026): a ordem das habilidades do dia e as duas datas estáticas do jogo.
# - 0x5d6200 compare_shorts_ascending: cdecl(a, b) -> eax = movsx [a] - movsx [b]; comparador do crt_qsort da passada
#   diária das pessoas (game_daily_staff_update 0x5ce2d0, empurrado em 0x5d39ab: as duas tabelas de habilidade do dia,
#   nos 3 dias do ano de cada jogador). Roda na carreira ENG.
# - 0x5cc650 / 0x5cc680: cdecl(), inicializadores estáticos do CRT (no _initterm, depois do DllMain, com os desvios já
#   postos no nosso exe; no exe com patch de terceiros a instalação é adiada e eles rodam originais). Date::Date
#   0x54a110 thiscall(data) (dia 40 de 1972, +4 = 1) em 0xae2c90 / 0xae2c78; atexit 0x94627a cdecl(0x5cc670 /
#   0x5cc6a0). Mesmo molde da 0x524cd0 do sx08_1.
# Fora do item (corpo vazio, um ret, sem desvio): 0x5cc670, 0x5cc6a0 e 0x5d5f10 (os destrutores estáticos vazios).
# Tipo: simulação (a carreira ENG até 2002-06-01 roda as três; as duas datas uma vez, na partida do programa).
SAVE = False
ROTINAS = [('compare_shorts_ascending', 0x5d6200, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('game_date_static_init', 0x5cc650, [0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('last_processed_date_static_init', 0x5cc680, [0xb9, 0x78, 0x2c, 0xae, 0x00])]
DOCS = ('a ordem das habilidades do dia e as datas estáticas do jogo',
        'the order of the day abilities and the game static dates',
        [('compare_shorts_ascending', 'compare_shorts_ascending', '0x5d6200', 'game.cpp', 'cdecl',
          'compara dois valores de 16 bits (com sinal) em ordem crescente, para o qsort: a menos b. Ordena as tabelas de habilidade atual e potencial que a passada diária das pessoas monta para o desenvolvimento dos jogadores',
          'compares two signed 16-bit values in ascending order, for qsort: a minus b. Sorts the current and potential ability tables the daily person pass builds for the players development'),
         ('game_date_static_init', 'game_date_static_init', '0x5cc650', 'game.cpp', 'cdecl',
          'inicializador estático do exe (antes do WinMain): constrói a data do jogo com a data padrão (dia 40 de 1972) e registra o destrutor (vazio) no atexit; a data de verdade vem da criação ou da carga do jogo',
          'static initialiser of the exe (before WinMain): builds the game date with the default date (day 40 of 1972) and registers the (empty) destructor with atexit; the real date comes from creating or loading a game'),
         ('last_processed_date_static_init', 'last_processed_date_static_init', '0x5cc680', 'game.cpp', 'cdecl',
          'o mesmo para a última data processada (0xae2c78)',
          'the same for the last processed date (0xae2c78)')])
CAMPOS = []
INTERESSA = []
