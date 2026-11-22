# The club screens (6b): club_screens.cpp, part 6 (reader tel06, the screens round, 21/11/2026).
# Conventions checked in the disassembly (every push against esp):
# - 0x481ef0 build_club_squad_navigation_menu cdecl(club, colour, colour_text, squad_handler, history_handler,
#   alt_handler, with_squads, senior_mode) ret 0 -> 1; whole from 0x481ef0 to 0x482a32 (the map's 0x482070 entry is
#   mid-routine); 0x804030 add_button_item thiscall(12 dwords) ret 0x30, 0x55fba0 Display::add_area thiscall(11) ret
#   0x2c and 0x55f990 Display::add_item thiscall(18) ret 0x48 on the slot's Display ([0xb74c4c + 0x300 slot]); 0x945667
#   realloc / 0x9452ca free (the related clubs array, grown by 50; a Vec here); 0x945333 qsort with 0x53ffe0 and
#   0x540080 (by address); 0x5f2500 history_his_file_exists cdecl(name). The with_squads argument (arg 6) is reused by
#   the original as the array pointer.
# - 0x482d10 handle_club_screen_action cdecl(short widget) ret 0 -> eax, SEH, 0x4e8 of locals (the box texts);
#   0x591840 confirm_push cdecl(title, heading, text, buttons, text_colour, colour, code); 0x5c75a0 thiscall(club,
#   date, out, out, out, 0, 0, kind, check_stadium) ret 0x24 -> al; 0x5c6b70 (ecx set, 3 args, ret 0xc); 0x822a60
#   cdecl(club, 0) (the second argument is a leftover push); 0x6b19d0 cdecl(club); 0x7f82f0/0x7f8500 thiscall ret 4;
#   0x5c57d0 thiscall(request, club) ret 8; 0x5c6990 thiscall(club, add) ret 8; 0x54c5b0/0x417c10 thiscall ret 8.
# - 0x4814e0 part_exchange_screen_handle_event cdecl(short widget) ret 0 -> eax, SEH; jump tables 0x481e98/0x481ea0
#   and 0x481ec0/0x481ecc inside, to 0x481ef0; 0x803320 thiscall ret 4; 0x8f6770 cdecl(player, 0, -1, 0); the
#   menu bar 0x7615d0 / 0x7643d0 cdecl(widget). The part exchange screen's build (0x47c180) is tel05_2's.
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel26_1 (tel26_add_item, tel26_add_area, tel26_add_button, job_colour, JOB_*), tel03_4
#   (club_tabs_action, club_tabs_value, CLUB_TABS_DIALOG_ANSWERED).
# Kept from the original / simplifications: colour words zero-extended; the 7-byte record of 0x4814e0's action 2 has
#   a last byte the original leaves uninitialised (stack leftover): 0 here; the box texts go into separate local
#   buffers (the original's offsets in one 0x4e8 frame).
# Comparers: the menu over the display list (compare_match_screen_build) and the return; the club actions only in the
#   original, the answer against the plan; the part exchange input side by side for the clicks that only write params
#   (views, columns, sort, scroll), the others only in the original (answer checked for 2 and 0xa).
TELAS = True
ROTINAS = [('build_club_squad_navigation_menu', 0x481ef0, [0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x30]),
           ('handle_club_screen_action', 0x482d10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('part_exchange_screen_handle_event', 0x4814e0, [0x6a, 0xff, 0x68, 0x0b, 0x5f, 0x95, 0x00])]
DOCS = ('as telas do clube (6b): o menu dos times do clube, as respostas das ações da tela do clube e os cliques da tela de troca de jogadores',
        'the club screens (6b): the club\'s squad menu, the club screen\'s action answers and the part exchange screen\'s clicks',
        [('build_club_squad_navigation_menu', 'build_club_squad_navigation_menu', '0x481ef0', 'club_screens.cpp', 'cdecl',
          'o menu dos times da tela do clube: os times do próprio clube (principal, B ou reservas, a sub-21 ou sub-23 da seleção), os clubes relacionados (as seleções do continente ou os clubes da mesma divisão, pela reputação e cortados a 28 entradas menos as já postas, depois pelo nome), a seleção do clube e a história escrita quando o arquivo existe',
          'the club screen\'s squad menu: the club\'s own sides (first team, B or reserves, the national Under-21 or Under-23 side), the related clubs (the continent\'s national teams or the clubs of the same division, by reputation and cut to 28 entries minus those already listed, then by name), the club\'s national team and the written history when its file exists'),
         ('handle_club_screen_action', 'handle_club_screen_action', '0x482d10', 'club_screens.cpp', 'cdecl, SEH',
          'as ações da tela do clube e as respostas das caixas: assumir o clube, mandar olheiro (ou a caixa de que já é observado), candidatar-se ao emprego, convidar para amistoso (ou a caixa de indisponível, movendo a data antiga para daqui a uma semana), responder um pedido de datas, convidar para o torneio do humano ou desconvidar; o "sim" de cada caixa grava o convite, o amistoso ou o olheiro',
          'the club screen\'s actions and the answers to their boxes: take control, send a scout (or the box saying the club is already watched), apply for the job, invite for a friendly (or the unavailable box, moving an old date to a week from now), answer a dates request, invite to or uninvite from the human\'s tournament; the "yes" of each box writes the invitation, the friendly or the scout'),
         ('part_exchange_screen_handle_event', 'part_exchange_screen_handle_event', '0x4814e0', 'club_screens.cpp', 'cdecl, SEH',
          'os cliques da tela de troca de jogadores: a visão e as colunas (grupos exclusivos de bits), a coluna da ordem (o segundo clique inverte), a rolagem, pôr o jogador escolhido na proposta (o valor dele sai da parte em dinheiro, sem ficar negativa) e a tela de situação de transferência; o resto vai à barra de menus',
          'the part exchange screen\'s clicks: the view and the columns (exclusive bit groups), the sort column (a second click reverses it), the scroll, adding the picked player to the offer (his value comes off the cash part, not below zero) and the transfer status screen; the rest goes to the menu bar')])
