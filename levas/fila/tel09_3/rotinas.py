# The staff profile's input handler (staff_screens.cpp, part 3; reader tel09, the screens round, 21/11/2026).
# - 0x888260 staff_screens_profile_input cdecl(short item) ret 0, SEH, 0x68c bytes of locals; the routine runs to
#   the ret at 0x889bfd plus its two jump tables 0x889c00 (the scouting answer, 5) and 0x889c14 (the view, 5): the
#   community map's 0x888027 entry runs into it and its 0x8899e4 entry (list tel10) is its tail (the scouting boxes and
#   the view switch), here in this item. The handler of the profile screen opened by 0x8702a0 (tel07_1).
# Conventions checked at the call sites: cdecl (add esp): 0x8702a0 (5), 0x4e53e0, 0x5f11c0, 0x53dba0 (4), 0x53b7c0
#   (3), 0x53b3d0 (1), 0x53b640 (4), 0x591840 the box (7: title, subtitle, text, kind, two colours, command), 0x7961b0,
#   0x7e5b10 (2), 0x540a50 (3), 0x53cb80, 0x48bc00 (2), 0x7a2a50 (2), 0x538bc0, 0x8f6770 (4), 0x5a8380, 0x4dd0f0 (4),
#   0x8f8120 (2), 0x8f8630, 0x8f4160 (2), 0x8f41b0 (3), 0x8f2470 (2), 0x8f4c70, 0x8e9350 (2), 0x8f0e40 (4), 0x8f8be0,
#   0x8f9690, 0x8fbcc0, 0x8e9550 (2), 0x778f60 (3), 0x944e46 malloc, 0x88b2f0 / 0x88b4b0 / 0x7615d0 / 0x7643d0 (1);
#   thiscall: 0x417fe0 (shortlist +0x1f; manager, person) ret 8, 0x4cbd20 ([0xae1050]; p, club, &out) ret 0xc,
#   0x842c00 ([[0xde1f54]+0x856]; p, 1) ret 8, 0x8c5450 ([0xde1f54]; p, &out) ret 8, 0x7f7920 ([0xb74448]; k) ret 4,
#   0x8ce990 ([0xde1f54]; p, club, &out, 0) ret 0x10, 0x8c7560 work permit ret 0xc, 0x4cf5a0 get_contract ret 4,
#   0x8d0590 ([0xde1f54]; 7 args) ret 0x1c, 0x549f70 (Date*; 1, a, b, -1) ret 0x10, 0x600740 / 0x6005e0
#   ([0xb63c98]; p, 0) ret 8, 0x7f8ff0 ([0xb74448]; p, 0) ret 8 -> al, 0x7f75f0 ([0xb74448]; p, 0) ret 8,
#   0x54cfb0 / 0x54cf70 Date month / day ret 0 (the argument pushed before each stays on the stack: it is the month
#   and the year of the "%s %s %d.%d.%d" sprintf), 0x7fe600 open_screen ret 0x14, 0x803320 get_area_scroll_row ret 4.
# Uses tel26_1 (tel26_item_id, tel26_item_value, tel26_slot_manager, tel26_error_box) and tel08_1 (staff_tab_lang,
#   staff_tab_format, staff_tab_strcpy): it goes after both.
# Defects of the original (kept): 0x25/0x26 leak the 26-byte name copy when the screen does not open, and copy the
#   name into it without a limit; the error box of an unknown view tests an uninitialised local (here 0).
# Comparer: compare_resposta, side by side over the params and the answer only for the clicks that just write a
#   parameter (0x3c, 0x3d, 0x11 with a new value, no pending network box answer); the rest only in the original.
# How to reach it: any click on a person's profile: the tab bar (the view, parameter 1), the season buttons of the
#   History tab, the actions menu (offer contract, approach, scout, compare, notes, shortlist), the award vote (a
#   shortlisted player) and the name screen.
TELAS = True
ROTINAS = [('staff_screens_profile_input', 0x888260, [0x6a, 0xff, 0x68, 0x9b, 0x2f, 0x96, 0x00])]
DOCS = ('as respostas aos cliques no perfil da pessoa',
        'the answers to the clicks on the staff profile',
        [('staff_screens_profile_input', 'staff_screens_profile_input', '0x888260', 'staff_screens.cpp', 'cdecl, SEH',
          'Responde a um clique no perfil (o id de comando do item, 0 e -1 viram o comando padrão, e o valor dele): no jogo em rede primeiro a resposta pendente de caixa (0x18 o voto no prêmio: com a resposta 2 vota e devolve -10; 0x27, 0x42 e 0x3c só são limpas); depois por comando: 0x3c/0x3d o parâmetro 0x14; 1 abre o perfil do valor na mesma aba; 2 a caixa do voto no prêmio (comando 0x18); 3 mover para o elenco (caixa de falta de vaga, comando 0x27); 6 oferecer contrato (caixa quando não pode, comando 0x3c); 0x29 observar o jogador (tela de avaliação, ou as caixas de sem olheiro, já observando e sem recursos, comando 0x42); 0x31 comparar jogadores; 0xf, 0x10, 0x16 e 0x17 as abordagens do clube do técnico do turno (0x16 confere primeiro o visto de trabalho e as duas datas de 0x8d0590); 0x11 o valor no parâmetro 1 quando muda (-11); 0x25/0x26 a tela do nome (cópia de 26 bytes no parâmetro 1); as outras ações chamam a rotina de cada uma e devolvem -4 (algumas -11). Os outros comandos vão pela vista do parâmetro 1 (1 o menu do elenco, 5 o menu do histórico, 4 a linha de rolagem do parâmetro 0xc no 0xb para jogador, outra a caixa de staff_screens.cpp 13210) e depois pela barra de menus',
          "Answers a click on the profile (the item's command id, 0 and -1 becoming the default command, and its value): on a network game first the pending box answer (0x18 the award vote: with answer 2 votes and returns -10; 0x27, 0x42 and 0x3c are only cleared); then by command: 0x3c/0x3d parameter 0x14; 1 opens the value's profile on the same tab; 2 the award vote box (command 0x18); 3 moving to the squad (the no-space box, command 0x27); 6 offering a contract (a box when it cannot, command 0x3c); 0x29 scouting the player (the assess screen, or the no scouts, already scouting and no resources boxes, command 0x42); 0x31 comparing players; 0xf, 0x10, 0x16 and 0x17 the approaches of the slot manager's club (0x16 first checks the work permit and the two dates of 0x8d0590); 0x11 the value in parameter 1 when it changes (-11); 0x25/0x26 the name screen (a 26-byte copy in parameter 1); the other actions call their routine and return -4 (some -11). Other commands go by the view of parameter 1 (1 the squad menu, 5 the history menu, 4 the scroll row of parameter 0xc into 0xb for a player, another the staff_screens.cpp 13210 box) and then by the menu bar")])
