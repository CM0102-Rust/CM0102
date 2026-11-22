# The tactics screen object (tactics_screens.cpp, 3; reader tel15, the screens round, 21/11/2026): building,
# copying, freeing it, applying the edits, the record copy, the title, the automatic captain and the opponent's
# predicted line-up.
# Conventions: the constructor thiscall(this, club, match, reserve, arg4, record index, penalty mode) ret 0x18; the
# copy constructor thiscall(this, src) ret 4; the destructor, apply_changes, auto_select_captain and
# predict_opponent_lineup thiscall ret 0; close_and_apply cdecl; match_stats_record_assign thiscall(this, src) ret 4;
# format_title thiscall(this, out, size, colour a, colour b) ret 0x10. The SEH frames of the constructors and the
# destructor only unwind the members on an exception (none is thrown); the copy has no frame.
# Defects of the original: the constructor outside a match and a record leaves SIDE and OPPONENT_SIDE as the
# allocation left them.
# Conventions checked in the disassembly (every push against esp and the ret of each routine); the same bytes in the
# cm0102-1080.exe (0x8a2ca0..0x8ac600 identical).
# Needs tel26_1 (tel26_error_box, display_method, JOB_TEXT and the job colours), tel14_1 (tactics_screen::*,
# TACTICS_SCREENS_CPP, the destructor constants) and tel14_2 (TACTICS_PRESET_NAMES, TACTICS_COLOUR_3154); the
# tel15 items use the helpers of tel15_1 (tactics_screen_state, tactics_screen_calls, tactics_screen_fns,
# tactics_save_undo, compare_tactics_method...): tel15_1 goes first.
TELAS = True

ROTINAS = [('tactics_screen_constructor', 0x8a7440, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('tactics_screen_copy_constructor', 0x8a7e00, [0x6a, 0xff, 0x68, 0x72, 0x37, 0x96, 0x00]),
           ('tactics_screen_destructor', 0x8a2ca0, [0x6a, 0xff, 0x68, 0x4f, 0x36, 0x96, 0x00]),
           ('tactics_screen_close_and_apply', 0x8a2d70, [0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('match_stats_record_assign', 0x8a5010, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('apply_changes', 0x8a80d0, [0x83, 0xec, 0x08, 0x55, 0x8b, 0xe9]),
           ('format_title', 0x8a8330, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('auto_select_captain', 0x8a84a0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('predict_opponent_lineup', 0x8abe30, [0x6a, 0xff, 0x68, 0x8b, 0x37, 0x96, 0x00])]
DOCS = ('o objeto da tela de tática: construir, copiar, liberar, aplicar, a cópia do registro, o título, o capitão automático e a escalação prevista do adversário',
        "the tactics screen object: building, copying, freeing, applying, the record copy, the title, the automatic captain and the opponent's predicted line-up",
        [('tactics_screen_constructor', 'Tactics_Screen::Tactics_Screen', '0x8a7440', 'tactics_screens.cpp', 'thiscall',
          'constrói o objeto da tela de tática: com registro de partida (lido, com o jogo e o adversário dele, só de leitura, a formação e o capitão do registro), com partida ao vivo (o jogo, o uniforme e a tática do motor e os 20 jogadores do lado) ou com o próximo jogo do clube nas próximas 8 semanas (24 para seleção), prevendo a escalação do adversário; o clube do humano atual toma a tática ativa, os ajustes de visão dele e o capitão, cobradores, ordem dos pênaltis (a padrão) e armadores da tabela de táticas (editável); outro clube, só de leitura. Sem clube, a caixa da linha 0x12b6',
          "builds the tactics screen object: from a match record (read, with its fixture and opponent, read-only, the record's formation and captain), from the live match (the engine's fixture, kit, tactic and the side's 20 players) or from the club's next fixture within 8 weeks (24 for a national team), predicting the opponent's line-up; the current human's club takes the active tactic, his view settings and the captain, takers, penalty order (the default one) and playmakers from the Tactics table (editable); another club, read-only. No club shows the box of line 0x12b6"),
         ('tactics_screen_copy_constructor', 'Tactics_Screen::Tactics_Screen(const&)', '0x8a7e00', 'tactics_screens.cpp', 'thiscall',
          'copia o objeto da tela (os diálogos de instruções editam uma cópia): argumentos, registro, jogo, adversário, lados, uniforme, tática ativa, escalação prevista, os três blocos de dados, as coordenadas copiadas e as marcas de colar e desfazer; origem nula, a caixa da linha 0x13b9',
          'copies the screen object (the instruction dialogs edit a copy): arguments, record, fixture, opponent, sides, kit, active tactic, predicted line-up, the three data blocks, the copied coordinates and the paste and undo flags; a null source shows the box of line 0x13b9'),
         ('tactics_screen_destructor', 'Tactics_Screen::~Tactics_Screen', '0x8a2ca0', 'tactics_screens.cpp', 'thiscall',
          'destrói os jogadores de partida dos três blocos de dados e os dois blocos de time do registro',
          "destroys the match players of the three data blocks and the record's two team blocks"),
         ('tactics_screen_close_and_apply', 'tactics_screen_close_and_apply', '0x8a2d70', 'tactics_screens.cpp', 'cdecl',
          'sai da tela gravando: aplica as mudanças do objeto do parâmetro 0, destrói, libera e zera o parâmetro',
          'leaves the screen writing back: applies the changes of the object in parameter 0, destroys, frees and clears the parameter'),
         ('match_stats_record_assign', 'match_stats_record_assign', '0x8a5010', 'tactics_screens.cpp', 'thiscall',
          'copia um registro de estatísticas de partida campo a campo (cabeçalho, blocos de time e as 20 linhas de cada lado pela cópia de linha, o diário de eventos e o tempo)',
          'copies a match statistics record field by field (header, team blocks and the 20 rows of each side through the row copy, the event log and the weather)'),
         ('apply_changes', 'Tactics_Screen::apply_changes', '0x8a80d0', 'tactics_screens.cpp', 'thiscall',
          'grava as mudanças: a formação sobre a tática ativa, os ajustes de visão do humano, capitão, cobradores, ordem dos pênaltis e armadores na tabela de táticas; numa partida ao vivo, as substituições na fila (o evento de narração da quantidade e cada uma feita) e o motor refaz o lado; só de leitura devolve 0',
          "writes the changes: the formation over the active tactic, the human's view settings, captain, takers, penalty order and playmakers into the Tactics table; in a live match, the queued substitutions (the commentary event of their count and each one made) and the engine refreshes the side; read-only returns 0"),
         ('format_title', 'Tactics_Screen::format_title', '0x8a8330', 'tactics_screens.cpp', 'thiscall',
          'o título da tela e as cores do uniforme: "<clube> Cobradores de Pênalti", "<clube> Tática Reserva" (reservas de clube que não é seleção) ou "<clube> Tática"; saída curta ou nula, a caixa da linha 0x1452',
          'the screen title and the kit colours: "<club> Penalty Takers", "<club> Reserve Tactics" (reserves of a club that is not a national team) or "<club> Tactics"; a short or null output shows the box of line 0x1452'),
         ('auto_select_captain', 'Tactics_Screen::auto_select_captain', '0x8a84a0', 'tactics_screens.cpp', 'thiscall',
          'numa partida ao vivo, faz capitão o jogador em campo com a maior nota de capitão (o primeiro no empate), com a cópia de desfazer; sem pessoa, a caixa da linha 0x14e7',
          'in a live match, makes captain the player on the pitch with the highest captain score (the first on a tie), with the undo copy; no staff shows the box of line 0x14e7'),
         ('predict_opponent_lineup', 'Tactics_Screen::predict_opponent_lineup', '0x8abe30', 'tactics_screens.cpp', 'thiscall',
          'prevê a escalação do adversário: os jogadores do elenco não lesionados, não suspensos na competição e fora de convocação na data do jogo, distribuídos pelas posições da tática do adversário (ou um 4-4-2 padrão) com o banco do jogo',
          "predicts the opponent's line-up: the squad players not injured, not banned in the competition and not on international duty on the fixture date, assigned to the opponent's tactic positions (or a default 4-4-2) with the fixture's bench")])
