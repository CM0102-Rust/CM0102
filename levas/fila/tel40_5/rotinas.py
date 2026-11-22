# Os tecnicos humanos de um jogo restaurado: quem fica com as telas e quem vai para o computador (Setup.cpp; leitor
# tel40, rodada 2 das telas, 22/11/2026). Usa SCRMAN_JOIN_HOOK/SCRMAN_LEAVE_HOOK e scrman_records_regions do tel40_1
# (que entra antes), NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, G_NETWORK_GAME_NAMED_FLAG, HUMAN_MANAGER_LOCKED_COUNT
# (tel11_*), FN_HUMAN_MANAGER_SET_COMPUTER_CONTROLLED (publicado).
# Convencao conferida na desmontagem: cdecl, sem argumentos, devolve int em eax (0x8388c2..0x8388d1). Chamadores:
# 0x81c03d e 0x81c7ac (a carga por -load e Carregar Jogo, depois do load_game_data) e 0x81f5fb (o jogo em rede).
# Get_password 0x5fde40 thiscall(staff) ret 4 -> char*; is_on_holiday 0x5fd210 thiscall(staff) ret 4;
# set_computer_controlled 0x5fe140 thiscall(valor, staff) ret 8 (grava +0x9ace e a contagem +8 do HUMAN_MANAGER e avisa
# o objeto do dia de jogo 0xb67a50 quando existe); add_manager_record 0x7ff8f0 thiscall(on_join, on_leave, key) ret 0xc.
# Roda SEM rede: em toda carga de jogo salvo (-load e Carregar Jogo); o Lucas passa por ela ao carregar a carreira com
# o JOGAR_COMPARANDO.bat.
# Convivencia com o simulation.rs: com CM_FERIAS (a prova do save) o simulation::install desvia 0x8387e0 antes do motor
# (restore_humans_on_holiday, que chama o trampolim); o desvio do motor e recusado ("detour ... refused": os bytes ja
# sao o salto) e a original roda dentro do invólucro, como hoje. Sem CM_FERIAS a copia roda (modo Rust) ou o comparador.
# Comparador: lado a lado sobre os registros do gerente de telas, o cabecalho do HUMAN_MANAGER e o +0x9ace de cada
# registro humano; com o objeto do dia de jogo (partida em andamento) so a original.
# Igual no cm0102-1080.exe (conferir com assinaturas.py --conferir).
TELAS = True
ROTINAS = [('setup_assign_restored_network_managers', 0x8387e0, [0x51, 0x53, 0x55, 0x33, 0xed])]
DOCS = ('os técnicos humanos de um jogo restaurado: quem fica com as telas e quem vai para o computador',
        'the human managers of a restored game: who takes the screens and who goes to the computer',
        [('setup_assign_restored_network_managers', 'setup_assign_restored_network_managers', '0x8387e0', 'Setup.cpp', 'cdecl',
          'ao restaurar um jogo salvo, cada técnico humano sem senha (num jogo sem nome de rede) fica com as telas: o primeiro com o registro atual, os outros com um registro cada; quem tem senha e não está de férias vai para o controle do computador até retomar; devolve 1 (abrir o Manager Status) com algum técnico travado ou de férias com senha',
          'when a saved game is restored, each human manager with no password (in a game with no network name) takes the screens: the first one the current record, the others a record each; one with a password not on holiday goes to computer control until he resumes; returns 1 (open Manager Status) with a manager locked or one with a password on holiday')])
