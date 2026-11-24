# A sessao de edicao do treino das telas de treino (training_edit_session.cpp; leitor sx20, rodada 5, 24/11/2026):
# as dez rotinas de 0x8b0a00 a 0x8b0e70. Rotinas inteiras (0x8b0a00..0x8b0b1d ret 4; 0x8b0b40..0x8b0beb ret;
# 0x8b0bf0..0x8b0c7e ret 4; 0x8b0c90..0x8b0cd5 ret 4; 0x8b0ce0..0x8b0d03 ret 4; 0x8b0d10..0x8b0d56 ret 4;
# 0x8b0d60..0x8b0dbd ret 8; 0x8b0dc0..0x8b0dfc ret 8; 0x8b0e00..0x8b0e62 ret 4; 0x8b0e70..0x8b0eb1 ret 4).
# Fica de fora 0x8b0b20 (TRAINING_EDIT_SESSION::destroy_schedule_slots): e a acao de desenrolar do SEH do construtor
# (so os jmp de 0x963823 e 0x9638e3, no desenrolar de excecao); nunca roda no jogo normal.
# TELAS: a prova e o Lucas jogando comparando: Treino (o treino do clube): abrir a edicao dos horarios, trocar o
# horario selecionado, mudar um horario e a comissao, mover um jogador de grupo, ver os jogadores de um horario,
# gravar e abrir um arquivo de horario, e aplicar (OK).
# Quem chama: as telas de treino (0x8b598f, 0x8b5c98, 0x8b5d9b, 0x8b85b2..0x8bb0cf).
# Convencoes conferidas na desmontagem (todas thiscall sobre a sessao):
# - 0x8b0a00(sessao, clube) ret 4 -> sessao: 0x9463af eh vector constructor iterator(+0xfa, 0x9b, 0xb, 0x8b4e10,
#   0x5a53c0), 0x68d020 LLIST(+0x80a), 0x8b3410 collect_squad_schedules thiscall([0xde1f44], clube, sessao) ret 8,
#   0x8b3930 get_manager_schedule thiscall([0xde1f44], tecnico, k) ret 8 (o k vai num dword da pilha com os bytes altos
#   sem valor; lido como char), 0x8b35a0 get_coach_assignments_copy thiscall([0xde1f44], tecnico, +0x7a3) ret 8;
# - 0x8b0b40(sessao) ret -> 1: 0x8b2250 set_player_schedule thiscall([0xde1f44], jogador, grupo) ret 8 (o grupo em ecx,
#   bytes altos de sobra), 0x8b23b0 set_coach_assignments thiscall([0xde1f44], tecnico, +0x7a3) ret 8;
# - 0x8b0bf0(sessao, indice) ret 4 -> 1/0 (fora de 0..9 a caixa training_edit_session.cpp 0x67);
# - 0x8b0c90 / 0x8b0ce0(sessao, origem) ret 4; 0x8b0d10(sessao, indice ou -1) ret 4 -> al; 0x8b0d60(sessao, saida,
#   ordem) ret 8 -> al e +0x818; 0x8b5000 TRAINING_SCHEDULE::count_players thiscall(horario, lista ou 0, sessao,
#   ordem ou 0) ret 0xc -> al; 0x8b0dc0(sessao, jogador, grupo) ret 8;
# - 0x8b0e00 / 0x8b0e70(sessao, caminho) ret 4 -> 1/0: 0x8b5220 load_from_file / 0x8b53d0 save_to_file
#   thiscall(horario, caminho) ret 4, 0x5fcb60 HUMAN_MANAGER::add_recent_file thiscall([0xb63c98], caminho, 0, 3) ret 0xc.
# Comparadores: lado a lado sobre a sessao inteira (0x819 bytes) e a resposta; so a original em apply_changes (grava no
# gerente de treino), nos arquivos e na lista com saida (aloca os nos e pode ordenar as entradas), e no indice
# invalido do select_schedule (a caixa).
TELAS = True
ROTINAS = [('training_edit_session_training_edit_session', 0x8b0a00, [0x6a, 0xff, 0x68, 0x36, 0x38, 0x96, 0x00]),
           ('training_edit_session_apply_changes', 0x8b0b40, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('training_edit_session_select_schedule', 0x8b0bf0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('training_edit_session_set_current_schedule', 0x8b0c90, [0x8b, 0xc1, 0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('training_edit_session_set_coach_record', 0x8b0ce0, [0x8b, 0xc1, 0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('training_edit_session_count_schedule_players', 0x8b0d10, [0x8a, 0x44, 0x24, 0x04, 0x6a, 0x00]),
           ('training_edit_session_list_schedule_players', 0x8b0d60, [0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08]),
           ('training_edit_session_set_player_group', 0x8b0dc0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('training_edit_session_load_schedule_file', 0x8b0e00, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('training_edit_session_save_schedule_file', 0x8b0e70, [0x0f, 0xbe, 0x91, 0xd9, 0x07, 0x00, 0x00])]
_F = 'training_edit_session.cpp'
DOCS = ('a sessao de edicao do treino',
        'the training edit session',
        [('training_edit_session_training_edit_session', 'TRAINING_EDIT_SESSION::TRAINING_EDIT_SESSION', '0x8b0a00', _F, 'thiscall, ret 4',
          'monta a sessao de edicao de um clube: os 11 horarios, a lista de jogadores vazia, as entradas do elenco com o grupo de cada jogador, copias dos dez horarios do tecnico (marcas de mudanca zeradas) e da tabela da comissao, sem mudanca e com o horario 0 selecionado; devolve a sessao',
          "builds a club's edit session: the 11 schedules, an empty player list, the squad entries with each player's group, copies of the manager's ten schedules (changed flags cleared) and of the coach table, unchanged and with schedule 0 selected; returns the session"),
         ('training_edit_session_apply_changes', 'TRAINING_EDIT_SESSION::apply_changes', '0x8b0b40', _F, 'thiscall',
          'devolve a sessao ao gerente de treino: os dez horarios sobre os do tecnico, cada jogador das entradas no seu grupo e a tabela da comissao; devolve 1',
          "copies the session back to the training manager: the ten schedules over the manager's, each player of the entries to his group and the coach table; returns 1"),
         ('training_edit_session_select_schedule', 'TRAINING_EDIT_SESSION::select_schedule', '0x8b0bf0', _F, 'thiscall, ret 4',
          'seleciona o horario (0 a 9) e devolve 1; fora disso, a caixa de erro (0x67) e 0',
          'selects the schedule (0 to 9) and returns 1; otherwise the error box (0x67) and 0'),
         ('training_edit_session_set_current_schedule', 'TRAINING_EDIT_SESSION::set_current_schedule', '0x8b0c90', _F, 'thiscall, ret 4',
          'copia um horario (0x9b bytes) no selecionado e marca o horario e a sessao como mudados',
          'copies a schedule (0x9b bytes) into the selected one and marks the schedule and the session as changed'),
         ('training_edit_session_set_coach_record', 'TRAINING_EDIT_SESSION::set_coach_record', '0x8b0ce0', _F, 'thiscall, ret 4',
          'copia a tabela da comissao editada (54 bytes) na sessao e a marca como mudada',
          'copies the edited coach table (54 bytes) into the session and marks it as changed'),
         ('training_edit_session_count_schedule_players', 'TRAINING_EDIT_SESSION::count_schedule_players', '0x8b0d10', _F, 'thiscall, ret 4',
          'conta os jogadores da sessao a que o horario se aplica (-1: o selecionado)',
          'counts the session players the schedule applies to (-1: the selected one)'),
         ('training_edit_session_list_schedule_players', 'TRAINING_EDIT_SESSION::list_schedule_players', '0x8b0d60', _F, 'thiscall, ret 8',
          'lista os jogadores do horario selecionado: com saida, a saida aponta a lista da sessao, preenchida (com as entradas ordenadas pela ordem dada); sem, so conta; a contagem vai para +0x818 e volta',
          'lists the players of the selected schedule: with an output, it points at the session list, filled (with the entries sorted by the given order); without, only counts; the count goes to +0x818 and is returned'),
         ('training_edit_session_set_player_group', 'TRAINING_EDIT_SESSION::set_player_group', '0x8b0dc0', _F, 'thiscall, ret 8',
          'poe o jogador no grupo dado (a primeira das 50 entradas com ele) e marca a sessao como mudada; jogador fora da sessao nao muda nada',
          'puts the player in the given group (the first of the 50 entries holding him) and marks the session as changed; a player not in the session changes nothing'),
         ('training_edit_session_load_schedule_file', 'TRAINING_EDIT_SESSION::load_schedule_file', '0x8b0e00', _F, 'thiscall, ret 4',
          'abre um arquivo de horario no selecionado; dando certo, marca o horario e a sessao como mudados, poe o arquivo no topo dos recentes de treino do tecnico e devolve 1; senao 0',
          "loads a schedule file into the selected one; on success marks the schedule and the session as changed, puts the file at the top of the manager's recent training files and returns 1; else 0"),
         ('training_edit_session_save_schedule_file', 'TRAINING_EDIT_SESSION::save_schedule_file', '0x8b0e70', _F, 'thiscall, ret 4',
          'grava o horario selecionado num arquivo; dando certo, poe o arquivo no topo dos recentes de treino e devolve 1; senao 0',
          'saves the selected schedule to a file; on success puts the file at the top of the recent training files and returns 1; else 0')])
