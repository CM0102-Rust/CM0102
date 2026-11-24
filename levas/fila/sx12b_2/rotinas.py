# Os construtores do HUMAN_MANAGER, do HUMAN_RECORD e do Injury (human_manager.cpp, injury.cpp; leitor sx12b, rodada 5,
# 23/11/2026). Rodam na criacao do jogo (create_game 0x828110: 0x828a1c, 0x829313) e na carga (0x82a9c0: 0x82b99f,
# 0x82c348); o HUMAN_RECORD pelo iterador de vetor do construtor (16 vezes) e do HUMAN_MANAGER::load.
# DEPENDE do sx02b_1 (find_game_cd): integrar junto ou depois dele.
# Convencoes conferidas na desmontagem:
# - 0x5fb430: thiscall(this, stream), ret 4, eax this; quadro SEH (sem destrutor no caminho normal); a busca do CD
#   (GetLogicalDriveStringsA/GetDriveTypeA pela IAT; jne do tipo de unidade em 0x5fb47f, test do achado em 0x5fb4c2: a
#   copia le esses bytes, como o find_game_cd do sx02b_1); HUMAN_MANAGER::load 0x5fee50 thiscall(this, stream) ret 4;
#   operator new(0xa6ef4); o iterador de vetor 0x9463af stdcall(p+4, 0xa6ef, 16, 0x5fb690, 0x5fb700); a caixa da linha
#   0x4b sem memoria (sem zerar +8 e +0xc); zera 16 x 0xa6ef; reset_human_record 0x5fb810 thiscall(this, staff) ret 4,
#   staff = [0xae23bc] + ([0xae2364] - k - 1) x 0x6e.
# - 0x5fb690: thiscall(this), ret 0, eax this; iterador(this, 0x1ecd, 5, 0x5fb720, 0x5a53c0); STAFF_CONTRACT 0x85e410
#   thiscall(this+0x9a09); SQUAD_VIEW_SETTINGS 0x487810 thiscall(this+0xa6d4).
# - 0x62a680: thiscall(this, stream), ret 4, eax this; o CD (jne 0x62a6cf, test 0x62a713); operator new(8) +
#   FILE_LLIST(utils_build_file_path("CM3_TEMP", "injury_history.tmp"), 0x2c); sem ela, a caixa da linha 0xb4,
#   0x5cc7d0, 0x5a8f60 e exit(-1) (fatal_error_at); zera +4..+0x28 (nao o +0); Injury::load 0x62ab00 ou init_records
#   0x62cf70 (ja Rust).
# Comparadores: so a original nos construtores do HUMAN_MANAGER e do Injury (CD, alocacao, arquivo temporario); lado a
# lado no HUMAN_RECORD (so escreve o registro; o Formation 0x5b27c0 e o set_slot_position ja em Rust so escrevem).
# Pilha particular: os dois construtores grandes sao chamados pela create_game (que le residuo de pilha, ver
# COMO_CONTINUAR 0x827110); com o desvio eles rodam na pilha particular, como o dbn_1 e o sx08b_2.
# Fora (RELATIVAS): HUMAN_TACTIC_SLOT::HUMAN_TACTIC_SLOT 0x5fb720 (push esi; mov esi, ecx; call 0x5b27c0): entraria com
# (0x5fb720, 3) em RELATIVAS.
SAVE = False
ROTINAS = [('construct_human_manager', 0x5fb430, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('construct_human_record', 0x5fb690, [0x6a, 0xff, 0x68, 0xf4, 0xaf, 0x95, 0x00]),
           ('construct_injury', 0x62a680, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os construtores dos técnicos humanos e das lesões',
        'the human manager and injury constructors',
        [('construct_human_manager', 'HUMAN_MANAGER::HUMAN_MANAGER', '0x5fb430', 'human_manager.cpp', 'thiscall, ret 4',
          'procura o CD; com o jogo salvo, lê dele e para; senão 16 registros de técnico humano (sem memória: a caixa e para), zerados e repostos um a um pelas 16 últimas pessoas da tabela, da última para cima; as contagens de controlados pelo computador e de férias em 0',
          'searches the CD; with the saved game, reads from it and stops; otherwise 16 human manager records (out of memory: the box and stop), zeroed and reset one by one for the last 16 staff rows, from the last up; the computer controlled and holiday counts 0'),
         ('construct_human_record', 'HUMAN_RECORD::HUMAN_RECORD', '0x5fb690', 'human_manager.cpp', 'thiscall',
          'monta um registro de técnico humano: os cinco blocos de tática (cada um uma formação), o contrato de rascunho e as definições da visão do elenco',
          'builds one human manager record: the five tactic blocks (each a formation), the draft contract and the squad view settings'),
         ('construct_injury', 'Injury::Injury', '0x62a680', 'injury.cpp', 'thiscall, ret 4',
          'procura o CD; abre a lista do histórico de lesões (arquivo temporário, registros de 0x2c bytes; sem ela o jogo para); zera as lesões, a fila de notícias de recuperação, os surtos e a lista das pancadas do dia; com o jogo salvo lê dele, senão (ou se falhar) monta os registros de um jogo novo',
          'searches the CD; opens the injury history list (temporary file, 0x2c-byte records; without it the game stops); clears the injuries, the recovery news queue, the outbreaks and the list of the day\'s knocks; with the saved game reads from it, otherwise (or when that fails) builds the records of a new game')])
