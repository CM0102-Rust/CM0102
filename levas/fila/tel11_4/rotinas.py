# The Setup screens, part 4 (Setup.cpp; reader tel11, the screens round, 21/11/2026): the take control box, the Enter
# Password box and the resume control box. Uses the helpers of tel11_1, tel11_2 (FN_HUMAN_MANAGER_GET_PASSWORD*,
# NETWORK_JOIN/LEAVE_CALLBACK) and tel11_3 (setup_texts_differ, setup_confirm_box): goes with the three.
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0): open_take_control_confirm(club),
# called by 0x482fc6 (the menu bar's club link on Select Team); the builders () -> void; the handlers (short item) ->
# int (-2/-9 close the box, -10 after taking control, -4 with a box opened).
# - HUMAN_MANAGER::set_computer_controlled 0x5fe140 thiscall(HM, 0, staff) ret 8, set_on_holiday 0x5fd2f0 likewise;
#   ScreenManager::add_manager_record 0x7ff8f0 thiscall(sm, 0x838a80, 0x838c00, staff) ret 0xc; MATCH_DAY::is_club_playing
#   0x6b59a0 thiscall([0xb67a50], club, 0) ret 8; FOG_OF_WAR::add_manager_map 0x5afd70 thiscall([0xae2a60], staff) ret 4;
#   appoint_human_manager_to_club 0x826f20 cdecl(staff, club).
# Which routines run in the automatic career: none.
# How to reach them in the game: Select Team -> click a club (take control box; Yes appoints the new manager). The
# password and resume boxes: Manager Status -> click a manager on holiday or not playing (with a password: Enter
# Password; the master password "sportsinteractive" also works).
# No new defect of the original found here.
MOTOR = True
TELAS = True
ROTINAS = [('open_take_control_confirm', 0x822a60, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x00]),
           ('take_control_confirm_build', 0x822aa0, [0x51, 0x56, 0x57, 0x6a, 0x00]),
           ('take_control_confirm_handle_input', 0x822c50, [0xa1, 0x84, 0x7c, 0xb7, 0x00]),
           ('enter_password_box_build', 0x822ec0, [0x51, 0x53, 0x56, 0x57, 0x6a, 0x00]),
           ('enter_password_box_handle_input', 0x8230b0, [0xa1, 0x84, 0x7c, 0xb7, 0x00]),
           ('resume_control_confirm_build', 0x823420, [0x51, 0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('resume_control_confirm_handle_input', 0x823660, [0xa1, 0x84, 0x7c, 0xb7, 0x00])]
DOCS = ('as telas de criar o jogo (4): assumir o controle, a senha e retomar o controle',
        'the game creation screens (4): take control, the password and resume control',
        [('open_take_control_confirm', 'open_take_control_confirm', '0x822a60', 'Setup.cpp', 'cdecl',
          'abre a caixa de assumir o controle do clube (parâmetro 0)',
          'opens the take control box of the club (param 0)'),
         ('take_control_confirm_build', 'take_control_confirm_build', '0x822aa0', 'Setup.cpp', 'cdecl',
          'monta a caixa de assumir o controle: a foto do clube, o nome do técnico, a pergunta e Não/Sim',
          'builds the take control box: the club picture, the manager name, the question and No/Yes'),
         ('take_control_confirm_handle_input', 'take_control_confirm_handle_input', '0x822c50', 'Setup.cpp', 'cdecl',
          'responde à caixa de assumir o controle: clube já de um humano ou jogando agora dá uma caixa; senão o técnico vira humano e é nomeado (com o mapa de atributos se houver máscara)',
          'answers the take control box: a club already run by a human or playing now gives a box; otherwise the manager becomes human and is appointed (with the attribute map when masking is on)'),
         ('enter_password_box_build', 'enter_password_box_build', '0x822ec0', 'Setup.cpp', 'cdecl',
          'monta a caixa da senha do técnico: o campo (50 letras), Cancel e Ok (só com algo digitado)',
          'builds the manager password box: the entry (50 characters), Cancel and Ok (only with something typed)'),
         ('enter_password_box_handle_input', 'enter_password_box_handle_input', '0x8230b0', 'Setup.cpp', 'cdecl',
          'responde à caixa da senha: técnico já com canal dá uma caixa; senha errada (nem a dele nem "sportsinteractive") é apagada com uma caixa; senão ele retoma o controle',
          'answers the password box: a manager already with a channel gives a box; a wrong password (neither his nor "sportsinteractive") is emptied with a box; otherwise he resumes control'),
         ('resume_control_confirm_build', 'resume_control_confirm_build', '0x823420', 'Setup.cpp', 'cdecl',
          'monta a caixa de retomar o controle: a pergunta com a seleção e/ou o clube do técnico, ou sem nada, e Não/Sim',
          'builds the resume control box: the question with the manager national side and/or club, or with nothing, and No/Yes'),
         ('resume_control_confirm_handle_input', 'resume_control_confirm_handle_input', '0x823660', 'Setup.cpp', 'cdecl',
          'responde à caixa de retomar o controle: técnico já com canal dá uma caixa; senão sai do controle do computador e das férias e volta à vaga',
          'answers the resume control box: a manager already with a channel gives a box; otherwise he leaves computer control and holiday and goes back to the slot')])
