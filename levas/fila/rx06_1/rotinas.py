# A limpeza antes da saida por erro fatal e o destrutor do hall da fama (game.cpp; leitor rx06, rodada 6, 25/11/2026):
# game_fatal_cleanup (0x5cc7d0) e HallOfFame::~HallOfFame (0x5cc830). Rotinas inteiras (0x5cc7d0..0x5cc826 ret;
# 0x5cc830..0x5cc842 ret; o resto ate a proxima fronteira e enchimento).
# Quem chama: game_fatal_cleanup, as caixas de erro fatal de todo o exe (falta de memoria, arquivo corrompido: a caixa
# e depois 0x5cc7d0 e exit(-1)), inclusive as do Rust (exe::game_fatal_cleanup); o destrutor, so a 0x5cc7d0. A carreira
# automatica nao passa (contadores 0): so roda na saida por erro fatal.
# TELAS opcional: o comparador so roda a original (a rotina libera os dados do jogo e nao pode rodar duas vezes; o
# destrutor so chama o destrutor vazio 0x5a53c0 por entrada). Prova: nenhuma caixa fatal no jogo comparando (o desvio
# instalado nao muda nada); a copia so e exercitada se o Lucas cair numa caixa fatal.
# Convencoes conferidas na desmontagem:
# - 0x5cc7d0 cdecl(), sem retorno; prologo push esi; push edi; push 1; call 0x82ff40: RELATIVAS (0x5cc7d0, 4)
#   (INTEGRAR_ANTES.txt);
# - 0x5cc830 thiscall(hall da fama), ret; 0x9462bb vector_destructor_iterator stdcall(vetor, 0x78, 0xaf0, 0x5a53c0)
#   ret 0x10 (2800 entradas de 120 bytes);
# - 0x82ff40 free_game_data cdecl(1); 0x5d7b70 Config::~Config thiscall; 0x944c94 operator delete cdecl.
TELAS = True
ROTINAS = [('game_fatal_cleanup', 0x5cc7d0, [0x56, 0x57, 0x6a, 0x01, 0xe8, 0x67, 0x37, 0x26, 0x00]),
           ('hall_of_fame_destructor', 0x5cc830, [0x68, 0xc0, 0x53, 0x5a, 0x00])]
DOCS = ('a limpeza antes da saida por erro fatal',
        'the clean-up before a fatal error exit',
        [('game_fatal_cleanup', 'game_fatal_cleanup', '0x5cc7d0', 'game.cpp', 'cdecl',
          'libera os dados do jogo (free_game_data(1)); se ha hall da fama, destroi, apaga e zera o ponteiro; se ha configuracao, destroi (Config::~Config), apaga e zera o ponteiro; por fim zera o ponteiro da raiz da interface (0xb67a30), para que nenhuma caixa desenhe na tela depois',
          'frees the game data (free_game_data(1)); if there is a hall of fame, destroys and deletes it and clears the pointer; if there is a configuration, destroys it (Config::~Config), deletes it and clears the pointer; finally clears the interface root pointer (0xb67a30), so no box draws on the screen afterwards'),
         ('hall_of_fame_destructor', 'HallOfFame::~HallOfFame', '0x5cc830', 'game.cpp', 'thiscall',
          'roda o destrutor (vazio) em cada uma das 2800 entradas de 120 bytes da tabela do hall da fama',
          'runs the (empty) destructor on each of the 2800 entries of 120 bytes of the hall of fame table')])
