# The training screens (training_screens.cpp, part 1; reader tel25, the screens round, 21/11/2026): the nine sort
# orders of the training squad list (qsort comparators the builder 0x8b5a10 hands to crt_qsort), the edit session's
# destructor and leaving the screen.
# Conventions checked in the disassembly:
# - the nine sorts cdecl(a, b) -> i32, a and b pointing at Staff pointers; 0x4849f0 get_player_status_code
#   cdecl(person, 0, 0) -> al; 0x53ee50 database_format_player_positions cdecl(player, buf, 200, 4); 0x53dba0
#   cdecl(person, buf, 200, 3); 0x90c1d0 utils_string_compare cdecl(first, second, 1, 0); 0x5431e0
#   staff_get_player_attribute cdecl(person, attribute byte); 0x7c1950 thiscall on [0xb74350](player) -> al;
#   0x4cc2d0 stdcall(person) (ecx loaded but not read) -> al; 0x62c400 thiscall on [0xb63d28](person) -> al; 0x62c590
#   thiscall on [0xb63d28](person, 1) -> ax.
# - 0x8b9740 ~TRAINING_EDIT_SESSION thiscall ret 0, SEH: 0x68d040 ~LLIST thiscall on this+0x80a; 0x9462bb CRT vector
#   destructor iterator stdcall(this+0xfa, 0x9b, 0xb, 0x5a53c0).
# - Out: 0x8ba920 TRAINING_SCHEDULE::scalar_deleting_destructor (thiscall(flags) ret 4; its prologue has a call rel32
#   at byte 3, so the hook needs (0x8ba920, 3) in detours.rs RELATIVAS first; the copies call it by address).
# - 0x8b97a0 training_screens_close cdecl: 0x7ff010 get_screen_param thiscall(0) ret 4; 0x8b0b40 apply_changes
#   thiscall ret 0; 0x944c94 operator delete cdecl; 0x7ff270 set_screen_param thiscall(0, 0, 0) ret 0xc.
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('training_screens_compare_by_status_then_name', 0x8bb190, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_name', 0x8bb2a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_position_then_name', 0x8bb340, [0x81, 0xec, 0x20, 0x03, 0x00, 0x00]),
           ('training_screens_compare_by_value_then_name', 0x8bb460, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_attribute_then_name', 0x8bb510, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_age_then_name', 0x8bb620, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_form_then_name', 0x8bb6d0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_morale_then_name', 0x8bb7d0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('training_screens_compare_by_injury_then_name', 0x8bb8d0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('drop_training_edit_session', 0x8b9740, [0x6a, 0xff, 0x68, 0xe8, 0x38, 0x96, 0x00]),
           ('training_screens_close', 0x8b97a0, [0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('as telas de treino (1): as ordens da lista do elenco, o destrutor da sessão de edição e a saída da tela',
        'the training screens (1): the squad list sort orders, the edit session destructor and leaving the screen',
        [('training_screens_compare_by_status_then_name', 'training_screens_compare_by_status_then_name', '0x8bb190', 'training_screens.cpp', 'cdecl',
          'ordena o elenco do treino pela situação do jogador (a diferença dos códigos, crescente) e depois pelo nome',
          'sorts the training squad by player status (the code difference, ascending), then by name'),
         ('training_screens_compare_by_name', 'training_screens_compare_by_name', '0x8bb2a0', 'training_screens.cpp', 'cdecl',
          'ordena o elenco do treino só pelo nome (sem pessoa: a primeira vai para o fim)',
          'sorts the training squad by name only (a missing person goes last)'),
         ('training_screens_compare_by_position_then_name', 'training_screens_compare_by_position_then_name', '0x8bb340', 'training_screens.cpp', 'cdecl',
          'ordena pelo texto das posições (alfabético) e depois pelo nome',
          'sorts by the positions text (alphabetically), then by name'),
         ('training_screens_compare_by_value_then_name', 'training_screens_compare_by_value_then_name', '0x8bb460', 'training_screens.cpp', 'cdecl',
          'ordena pelo valor, do menor para o maior, e depois pelo nome',
          'sorts by value, lowest first, then by name'),
         ('training_screens_compare_by_attribute_then_name', 'training_screens_compare_by_attribute_then_name', '0x8bb510', 'training_screens.cpp', 'cdecl',
          'ordena pelo atributo da coluna escolhida na vista (byte +4 de [0xde1f50]), do maior para o menor, e depois pelo nome',
          'sorts by the attribute of the view\'s chosen column (byte +4 of [0xde1f50]), highest first, then by name'),
         ('training_screens_compare_by_age_then_name', 'training_screens_compare_by_age_then_name', '0x8bb620', 'training_screens.cpp', 'cdecl',
          'ordena pela idade, do mais velho para o mais novo, e depois pelo nome',
          'sorts by age, oldest first, then by name'),
         ('training_screens_compare_by_form_then_name', 'training_screens_compare_by_form_then_name', '0x8bb6d0', 'training_screens.cpp', 'cdecl',
          'ordena pela forma recente, da melhor para a pior, e depois pelo nome',
          'sorts by recent form, best first, then by name'),
         ('training_screens_compare_by_morale_then_name', 'training_screens_compare_by_morale_then_name', '0x8bb7d0', 'training_screens.cpp', 'cdecl',
          'ordena pelo moral, do maior para o menor, e depois pelo nome',
          'sorts by morale, highest first, then by name'),
         ('training_screens_compare_by_injury_then_name', 'training_screens_compare_by_injury_then_name', '0x8bb8d0', 'training_screens.cpp', 'cdecl',
          'ordena pela situação de lesão (crescente), depois pela condição física (da maior para a menor) e por fim pelo nome',
          'sorts by injury status (ascending), then by condition (highest first), then by name'),
         ('drop_training_edit_session', 'TRAINING_EDIT_SESSION::~TRAINING_EDIT_SESSION', '0x8b9740', 'training_screens.cpp', 'thiscall',
          'destrói a sessão de edição do treino: a lista em +0x80a e os 11 programas de +0xfa (destrutor vazio); a memória não é liberada aqui',
          'destroys the training edit session: the list at +0x80a and the 11 schedules from +0xfa (empty destructor); the memory is not freed here'),
         ('training_screens_close', 'training_screens_close', '0x8b97a0', 'training_screens.cpp', 'cdecl',
          'ao sair da tela de treino: aplica as mudanças da sessão ao elenco, destrói e libera a sessão e zera o parâmetro 0',
          'leaving the training screen: applies the session\'s changes to the squad, destroys and frees the session and clears parameter 0')])
