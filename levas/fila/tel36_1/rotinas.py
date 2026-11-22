# The treatment confirm screen (record_utils.cpp; reader tel36, the screens round, 21/11/2026). Defines the helpers of
# the tel36 items (tel36_staff_param, tel36_club_param, tel36_langlib, tel36_confirm_build, tel36_confirm_answer): this
# item goes before tel36_2..tel36_14. Uses tel26_1 (tel26_add_area, tel26_area_button, tel26_text_area, tel26_item_id,
# job_colour, JOB_TEXT, JOB_COLOUR_*), tel26_4 (manager_menu_bar) and tel35_1 (TEL35_NETWORK_FLAG).
# Conventions checked in the disassembly: the open cdecl(Staff*, char leave) (open_screen thiscall(SM, builder,
# handler, 1, 0x5a8f60, 0) ret 0x14; set_screen_param thiscall(SM, k, v, 0) ret 0xc; Injury::get_recurring_injury_type
# 0x62ed30 thiscall(injury, person) ret 4 -> al); the builder cdecl(void) (background_set_club_picture 0x41c250
# cdecl(club, 0, 0); menubar_build cdecl(4, 0); database_get_club_kit_colours cdecl(club, 1, &fore, &back);
# database_format_staff_name cdecl(person, out, 2000, 1); add_screen_title thiscall(SM, back, fore, name) ret 0xc;
# gui_add_text_area cdecl, 11 dwords; Display::add_area ret 0x2c; add_area_button ret 0x2c); the handler cdecl(short)
# -> eax (FinanceTable::get_club_finance thiscall(table, club) ret 4; Finance::pay_other_costs 0x5999e0 thiscall(fin,
# amount) ret 4; Injury::add_injury thiscall(injury, kind, 0, person, 0, 1, -1, 0, 0) ret 0x20).
# The name buffer of the builder is 0x70 bytes in the original frame but is written with a 2000-byte limit (a long
# name would overrun the frame; the copy uses 2000 bytes). The handler pushes the kind as the dword whose low byte is
# the kind (the high bytes are the ecx left on entry; the copy passes the byte).
# Equal in cm0102-1080.exe (the bodies are identical).
# Comparers: the builder side by side over the display list; the open and the handler only the original (they open the
# screen, pay, injure, run the menu bar).
TELAS = True
ROTINAS = [('open_treatment_confirm_screen', 0x7e5b10, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('draw_treatment_confirm_screen', 0x7e5c00, [0x83, 0xec, 0x70, 0x53, 0x56]),
           ('handle_treatment_confirm_screen', 0x7e5e30, [0x51, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('a tela de confirmar um tratamento (licença, curandeiro, fisioterapia, cirurgia)',
        'the treatment confirm screen (leave of absence, faith healer, physiotherapy, surgery)',
        [('open_treatment_confirm_screen', 'open_treatment_confirm_screen', '0x7e5b10', 'record_utils.cpp', 'cdecl',
          'abre a tela para um jogador com clube: licença quando pedida, senão o tratamento do tipo de lesão recorrente (fisioterapia, cirurgia ou curandeiro pela tabela dos tipos 0xb a 0x3b; fora dela o curandeiro); parâmetros: a pessoa, o tipo e o clube',
          'opens the screen for a player with a club: a leave of absence when asked, otherwise the treatment of his recurring injury type (physiotherapy, surgery or faith healer by the table of types 0xb to 0x3b; outside it the faith healer); params: the person, the kind and the club'),
         ('draw_treatment_confirm_screen', 'draw_treatment_confirm_screen', '0x7e5c00', 'record_utils.cpp', 'cdecl',
          'monta o "Please Confirm": a foto do clube, o nome do jogador nas cores do clube, a pergunta do tratamento com o nome dele (licença de duas semanas, curandeiro, fisioterapia ou cirurgia; o tipo P fica sem pergunta) e Não / Sim',
          'builds the "Please Confirm": the club picture, the player\'s name in the club colours, the treatment question with his name (two-week leave, faith healer, physiotherapy or surgery; kind P has no question) and No / Yes'),
         ('handle_treatment_confirm_screen', 'handle_treatment_confirm_screen', '0x7e5e30', 'record_utils.cpp', 'cdecl',
          'Sim: fora a licença, o clube paga 22.500 (fisioterapia), 75.000 (cirurgia) ou 7.500 (o resto) como outros custos; o jogador recebe o tratamento como lesão e perde o bit 0 do byte +0xc do registro de estado; Não fecha; no jogo em rede sempre fecha; o resto vai para a barra de menus',
          'Yes: except for a leave, the club pays 22,500 (physiotherapy), 75,000 (surgery) or 7,500 (the rest) as other costs; the player gets the treatment as an injury and loses bit 0 of byte +0xc of his state record; No closes; in a network game it always closes; the rest goes to the menu bar')])
