# Os numeros da camisa do tecnico humano (human_manager.cpp; leitor sx12, rodada 5, 23/11/2026). Depende do sx12_4
# (human_record_or_error, human_record_found, compare_human_getter): integrar junto ou depois.
# A tabela: 50 ids de jogador (-1 numero livre) em +0x1e05 da vaga de tatica ativa (a vaga 1, reservas, usa a do
# primeiro time). Nenhuma roda na carreira de ferias (0 chamadas na cobertura do mundo): so na tela do elenco.
# Caminho no jogo para o Lucas provar (JOGAR_COMPARANDO.bat): Elenco do clube > numeros das camisas: dar um numero a um
# jogador, trocar dois numeros, tirar o numero; ver o elenco da selecao, se tiver; e vender ou liberar um jogador com
# numero e voltar ao elenco (prune_squad_numbers, chamada pela tela do elenco 0x459792); e assistir a uma partida do
# proprio time (o motor da partida le o dono de cada numero, 0x6cf0ff).
# Convencoes conferidas na desmontagem (thiscall; o tecnico e o ultimo argumento; numero e um byte com sinal):
# - prune_squad_numbers 0x600ee0: sem argumentos; o humano atual; o indice dele nao e conferido; a vaga lida como esta
#   (1 nao vira 0) para escolher a regra; 0x53b4f0 database_get_national_u21_team cdecl(nacao) so com selecao.
# - get_squad_number_player 0x601070 (numero, tecnico) ret 8 -> pessoa ou 0;
# - set_squad_number_player 0x601200 (numero, jogador, tecnico) ret 0xc;
# - clear_player_squad_number 0x6013c0 (jogador, tecnico) ret 8;
# - clear_squad_number 0x601560 (numero, tecnico) ret 8;
# - swap_squad_numbers 0x6016e0 (a, b, tecnico) ret 0xc;
# - get_player_squad_number 0x601880 (jogador, tecnico) ret 8 -> al (0..49 ou 0xff).
# Linhas das caixas "Error" (sem tecnico, fora da tabela, numero, jogador nulo) no cabecalho de cada rotina.
SAVE = False
TELAS = True
ROTINAS = [('prune_squad_numbers', 0x600ee0, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('get_squad_number_player', 0x601070, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('set_squad_number_player', 0x601200, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('clear_player_squad_number', 0x6013c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('clear_squad_number', 0x601560, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swap_squad_numbers', 0x6016e0, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('get_player_squad_number', 0x601880, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_E = ('; sem tecnico, fora da tabela ou argumento invalido, a caixa "Error"', '; with no manager, out of range or a bad argument, the "Error" box')
DOCS = ('os numeros da camisa do tecnico humano',
        "the human manager's squad numbers",
        [('prune_squad_numbers', 'HUMAN_MANAGER::prune_squad_numbers', '0x600ee0', 'human_manager.cpp', 'thiscall',
          'libera os numeros do humano atual cujos jogadores ja nao sao do time em vista: no clube (vagas 0 e 1), outro clube; na selecao (2) ou selecao B (3), outra selecao ou fora da convocacao; na sub-21 (4), fora da sub-21 da nacao ou da convocacao; defeito do original: sem selecao, a sub-21 e um valor de pilha nunca escrito (so lido na vaga 4)',
          "frees the current human's numbers whose players no longer belong to the team on view: at the club (slots 0 and 1), another club; in the national team (2) or B team (3), another national team or out of the squad; in the under-21 side (4), outside the nation's under-21 team or the squad; defect of the original: with no national job the under-21 team is a stack value never written (only read in slot 4)"),
         ('get_squad_number_player', 'HUMAN_MANAGER::get_squad_number_player', '0x601070', 'human_manager.cpp', 'thiscall, ret 8',
          'devolve o jogador que tem o numero no time em vista, ou nada quando o numero esta livre' + _E[0] + ' e nada',
          'returns the player holding the number in the team on view, or nothing when the number is free' + _E[1] + ' and nothing'),
         ('set_squad_number_player', 'HUMAN_MANAGER::set_squad_number_player', '0x601200', 'human_manager.cpp', 'thiscall, ret 0xc',
          'da o numero ao jogador no time em vista (um numero que ele ja tinha continua dele)' + _E[0],
          'gives the number to the player in the team on view (a number he already held stays his)' + _E[1]),
         ('clear_player_squad_number', 'HUMAN_MANAGER::clear_player_squad_number', '0x6013c0', 'human_manager.cpp', 'thiscall, ret 8',
          'libera o primeiro numero que o jogador tem no time em vista' + _E[0],
          'frees the first number the player holds in the team on view' + _E[1]),
         ('clear_squad_number', 'HUMAN_MANAGER::clear_squad_number', '0x601560', 'human_manager.cpp', 'thiscall, ret 8',
          'libera um numero no time em vista' + _E[0],
          'frees a number in the team on view' + _E[1]),
         ('swap_squad_numbers', 'HUMAN_MANAGER::swap_squad_numbers', '0x6016e0', 'human_manager.cpp', 'thiscall, ret 0xc',
          'troca os donos de dois numeros no time em vista' + _E[0],
          'swaps the holders of two numbers in the team on view' + _E[1]),
         ('get_player_squad_number', 'HUMAN_MANAGER::get_player_squad_number', '0x601880', 'human_manager.cpp', 'thiscall, ret 8',
          'devolve o primeiro numero (0 a 49) que o jogador tem no time em vista, ou -1' + _E[0] + ' e -1',
          'returns the first number (0 to 49) the player holds in the team on view, or -1' + _E[1] + ' and -1')])
