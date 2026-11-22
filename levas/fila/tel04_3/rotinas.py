# The tournament and friendly invitations and the squad numbers openers (club_screens.cpp, part 4c; reader tel04, the
# screens round, 21/11/2026). Uses the helpers of tel04_1 (club_screens_*, screen_fixture) and tel26_1: it goes after.
# Conventions checked at the call sites (every push against esp):
# - 0x4739d0 open_tournament_invitations_screen cdecl(tournament, arg) ret 0: the host test before open_screen
#   (0x473ad0, 0x473fb0, host, 0, 0).
# - 0x473ad0 tournament_invitations_screen_build cdecl(void); 0x473fb0 ..._handle_event cdecl(short): 0x5a0650
#   open_choose_friendly_teams_screen cdecl(tab, continent, nation, kind).
# - 0x4740c0 / 0x474720 / 0x475070 / 0x4752b0 cdecl(manager, item id), SEH, a NewsItem on the stack (0x77bbe0 ctor
#   thiscall(type, flag) ret 8; no destructor on the normal path): 0x786b70 News::find_item_by_id thiscall on
#   [0xb7431c](manager, id, out) ret 0xc; 0x785830 NewsItem::get_slot thiscall(char) ret 4; 0x549ef0 Date::Date;
#   0x5c88f0 find_friendly_match thiscall on [0xae2c74](date*, club) ret 8; 0x5c89a0 get_tournament thiscall(char,
#   0) ret 8.
# - 0x474280 / 0x4748a0 the invitation builds cdecl(void): 0x54ba20 Date::get_long_date_string thiscall(out, 200,
#   0xff) ret 0xc; 0x5ed070 gui_add_text_area cdecl, 14 pushes (with a link triple) / 11.
# - 0x4744b0 / 0x474ac0 the invitation handlers cdecl(short), SEH: 0x786d30 News::set_item_slot thiscall 5 args ret
#   0x14; 0x785790 NewsItem::set_slot thiscall(char, value) ret 8; 0x7861e0 send_to_club_manager thiscall ret 8;
#   0x5c5f40 process_friendly_reply thiscall(match, 0) ret 8; 0x5c5820 process_tournament_replies thiscall(t, 0, 0)
#   ret 0xc.
# - 0x474e00 open_squad_numbers_screen_for_match cdecl(context) ret 0 (eax 0/1): 0x85add0 / 0x85a760 thiscall on
#   [0xdd7ef0] ret 8; 0x8968f0 / 0x896e90 thiscall on [0xde1d08](club, slot, 0) ret 0xc (the slot dword is the
#   argument slot with its low byte replaced, reproduced); 0x775790 thiscall on [0xb74318] ret 4.
# - The squad numbers screen itself (builder 0x475520, 21 KB to 0x47a9d0, handler 0x47a9e0, leave 0x47c070) is left
#   to tel05 (its list holds most of the builder); here only called by address through open_screen.
# Defects of the original: none found.
# Comparers: the openers only the original, the params checked against the plan (the news item looked up again, and
# for the match one, when it opened the screen); the builds side by side by the display list; the inputs side by side
# for Back (and the invite session marks), the replies (news, the friendlies' records) and the menu bar only the
# original.
# How to reach it: a hosted tournament > Invited Clubs (Invite Another, Back); a friendly or tournament invitation in
# the news (Back, No, Yes); the squad numbers news item and the European squad news item; the match screen's check
# before a match of a club that uses squad numbers (a selected player with a number opens the screen).
TELAS = True
ROTINAS = [('open_tournament_invitations_screen', 0x4739d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tournament_invitations_screen_build', 0x473ad0, [0x83, 0xec, 0x1c, 0x53, 0x55]),
           ('tournament_invitations_screen_handle_event', 0x473fb0, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00]),
           ('open_friendly_invitation_screen', 0x4740c0, [0x6a, 0xff, 0x68, 0x86, 0x5d, 0x95, 0x00]),
           ('friendly_invitation_screen_build', 0x474280, [0x81, 0xec, 0xd4, 0x00, 0x00, 0x00]),
           ('friendly_invitation_screen_handle_event', 0x4744b0, [0x6a, 0xff, 0x68, 0x9b, 0x5d, 0x95, 0x00]),
           ('open_tournament_invitation_screen', 0x474720, [0x6a, 0xff, 0x68, 0xbb, 0x5d, 0x95, 0x00]),
           ('tournament_invitation_screen_build', 0x4748a0, [0x81, 0xec, 0xd4, 0x00, 0x00, 0x00]),
           ('tournament_invitation_screen_handle_event', 0x474ac0, [0x6a, 0xff, 0x68, 0xdb, 0x5d, 0x95, 0x00]),
           ('open_squad_numbers_screen_for_match', 0x474e00, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('open_squad_numbers_screen', 0x475070, [0x6a, 0xff, 0x68, 0xfb, 0x5d, 0x95, 0x00]),
           ('open_european_squad_screen', 0x4752b0, [0x6a, 0xff, 0x68, 0x1b, 0x5e, 0x95, 0x00])]
DOCS = ('os convites de torneio e de amistoso e a abertura dos números da equipe (club_screens.cpp, parte 4)',
        'the tournament and friendly invitations and the squad numbers openers (club_screens.cpp, part 4)',
        [('open_tournament_invitations_screen', 'open_tournament_invitations_screen', '0x4739d0', 'club_screens.cpp', 'cdecl',
          'abre os convidados do torneio (sem torneio, a caixa de club_screens.cpp 13860); anfitrião é o humano do canal que treina o clube do torneio: parâmetros torneio e marca de anfitrião; para ele, marca a sessão de convites (+0x29 = 1, +0x2d = argumento)',
          'opens the tournament\'s invited clubs (no tournament: the club_screens.cpp 13860 box); the host is the slot\'s human when he manages the tournament\'s club: params tournament and host flag; for him, marks the invite session (+0x29 = 1, +0x2d = argument)'),
         ('tournament_invitations_screen_build', 'tournament_invitations_screen_build', '0x473ad0', 'club_screens.cpp', 'cdecl',
          'monta os convidados: o nome do torneio nas cores do anfitrião, "Invited Clubs/Nations", Convidar Outro para o anfitrião (texto quando não há vaga); uma linha por vaga ocupada: o clube, a seleção e a divisão (ou o continente de uma seleção) e "Hosts", "Accepted Invitation" ou "Waiting For Reply"; Voltar e Avançar',
          'builds the invited clubs: the tournament\'s name in the host\'s colours, "Invited Clubs/Nations", Invite Another for the host (text when no slot is free); a row per taken slot: the club, its national team and division (or a national team\'s continent) and "Hosts", "Accepted Invitation" or "Waiting For Reply"; Back and Next'),
         ('tournament_invitations_screen_handle_event', 'tournament_invitations_screen_handle_event', '0x473fb0', 'club_screens.cpp', 'cdecl',
          'responde ao clique: Voltar encerra a sessão de convites; Convidar Outro abre a escolha do adversário (por continente para seleção, por nação para clube); o resto vai para a barra de menus',
          'answers a click: Back ends the invite session; Invite Another opens the opponent choice (by continent for a national team, by nation for a club); the rest goes to the menu bar'),
         ('open_friendly_invitation_screen', 'open_friendly_invitation_screen', '0x4740c0', 'club_screens.cpp', 'cdecl, SEH',
          'abre o convite de amistoso de uma notícia (sem ela, a caixa de club_screens.cpp 14121): o amistoso pelo clube e a data guardados na notícia; parâmetros a notícia e o amistoso',
          'opens the friendly invitation of a news item (missing: the club_screens.cpp 14121 box): the friendly by the club and date kept in the item; params the item and the friendly'),
         ('friendly_invitation_screen_build', 'friendly_invitation_screen_build', '0x474280', 'club_screens.cpp', 'cdecl',
          'monta o convite: "Friendly Match" nas cores do convidado, "Invitation", a pergunta com o clube que convida (ligado) e a data; Voltar, Não e Sim',
          'builds the invitation: "Friendly Match" in the invited club\'s colours, "Invitation", the question with the inviting club (linked) and the date; Back, No and Yes'),
         ('friendly_invitation_screen_handle_event', 'friendly_invitation_screen_handle_event', '0x4744b0', 'club_screens.cpp', 'cdecl, SEH',
          'responde ao convite: Voltar sai; Não/Sim grava a resposta (4/3) no amistoso e na notícia, manda a notícia de resposta ao técnico que convidou e processa a resposta; o resto vai para a barra de menus',
          'answers the invitation: Back leaves; No/Yes stores the reply (4/3) in the friendly and the news item, sends the reply news item to the inviting manager and processes the reply; the rest goes to the menu bar'),
         ('open_tournament_invitation_screen', 'open_tournament_invitation_screen', '0x474720', 'club_screens.cpp', 'cdecl, SEH',
          'abre o convite de torneio de uma notícia (sem ela, a caixa de club_screens.cpp 14287): o torneio pelo índice e o clube convidado guardados na notícia; parâmetros a notícia, o torneio e o clube',
          'opens the tournament invitation of a news item (missing: the club_screens.cpp 14287 box): the tournament by the index and the invited club kept in the item; params the item, the tournament and the club'),
         ('tournament_invitation_screen_build', 'tournament_invitation_screen_build', '0x4748a0', 'club_screens.cpp', 'cdecl',
          'monta o convite de torneio: sem anfitrião só limpa a foto; senão o nome do torneio nas cores do clube, "Invitation", a pergunta com o torneio e a data de início; Voltar, Não e Sim',
          'builds the tournament invitation: without a host only clears the picture; else the tournament\'s name in the club\'s colours, "Invitation", the question with the tournament and its start date; Back, No and Yes'),
         ('tournament_invitation_screen_handle_event', 'tournament_invitation_screen_handle_event', '0x474ac0', 'club_screens.cpp', 'cdecl, SEH',
          'responde ao convite de torneio: sem anfitrião ou Voltar sai; Sim aceita a vaga do clube (situação 3), Não a libera; grava a resposta na notícia, manda a notícia de resposta ao anfitrião e processa as respostas; o resto vai para a barra de menus',
          'answers the tournament invitation: no host or Back leaves; Yes accepts the club\'s slot (status 3), No frees it; stores the reply in the news item, sends the reply news item to the host and processes the replies; the rest goes to the menu bar'),
         ('open_squad_numbers_screen_for_match', 'open_squad_numbers_screen_for_match', '0x474e00', 'club_screens.cpp', 'cdecl',
          'antes do jogo, abre os números da equipe quando o clube usa números e tem a marca no jogo: o primeiro escalado (titulares e reservas do jogo) do banco que ainda é do clube e tem número abre a tela e devolve 1; quem já não é do clube sai da escalação; senão 0',
          'before a match, opens the squad numbers when the club uses numbers and has the flag in the fixture: the first selected player (starters and the match\'s substitutes) of the database who still belongs to the club and has a number opens the screen and returns 1; one who no longer belongs is taken out of the selection; else 0'),
         ('open_squad_numbers_screen', 'open_squad_numbers_screen', '0x475070', 'club_screens.cpp', 'cdecl, SEH',
          'abre os números da equipe de uma notícia (sem ela, a caixa de club_screens.cpp 14580): tipo 1, marca quando a notícia é do tipo 0x18',
          'opens the squad numbers of a news item (missing: the club_screens.cpp 14580 box): kind 1, flag when the item is of type 0x18'),
         ('open_european_squad_screen', 'open_european_squad_screen', '0x4752b0', 'club_screens.cpp', 'cdecl, SEH',
          'abre a equipe europeia de uma notícia (sem ela, a caixa de club_screens.cpp 14633): tipo 2, os campos 2, 4 e 5 da notícia nos parâmetros 0xb a 0xd',
          'opens the European squad of a news item (missing: the club_screens.cpp 14633 box): kind 2, the item\'s slots 2, 4 and 5 in params 0xb to 0xd')])
