# The staff profile screen (staff_screens.cpp, part 2; reader tel07b, the screens round, 21/11/2026).
# - 0x870700 staff_screens_build_profile_screen (23,529 bytes, 0x870700..0x8764e9; inside the 40,640 bytes the map gives
#   0x870650; it is the builder 0x8702a0 passes to open_screen): cdecl(void), ret 0, SEH frame, 0x1d10 bytes of locals
#   (a Text, a second Text in the injuries tab, 25 MenuItem for the tab bar built and destroyed by the CRT vector
#   iterators 0x9463af / 0x9462bb with 0x4568f0 / 0x5a53c0, stdcall). Split into named pieces: profile_screen_setup,
#   profile_screen_heading (+ profile_screen_title_box), profile_screen_action_menu (profile_screen_contract_actions,
#   profile_screen_approach_to_sign, profile_screen_national_actions, profile_screen_staff_actions),
#   profile_screen_compare_entries, profile_screen_tabs, profile_screen_identity (+ profile_screen_nationality),
#   profile_screen_statistics, profile_screen_injuries_tab; the whole routine is hooked.
# Conventions checked at the call sites: 0x55fba0 Display::add_area thiscall ret 0x2c; 0x55f990 Display::add_item ret
#   0x48; 0x804030 add_button_item ret 0x30; 0x804100 add_area_text ret 0x24; 0x8040a0 add_area_button ret 0x2c;
#   0x803fd0 add_text_item ret 0x2c; 0x803f30 add_screen_title ret 0xc; 0x4567f0 ScreenManager::add_area ret 0x2c;
#   0x4aeaf0 Text::draw_wrapped ret 0x20; 0x4afa40 Text::clear ret 0; 0x54a650 Date::get_short_date_string ret 0xc;
#   0x4149a0 AWARD_MANAGER::get_pending_vote ret 8; 0x418140 AWARD_SHORTLIST::get_vote_status ret 4 (ecx = shortlist
#   +0x1f); 0x4cb960 staff_is_not_board_member and 0x4cbb20 is_contract_staff_job stdcall ret 4 (ecx loaded with the
#   contract manager and ignored); 0x76d9b0 (NATIONAL_TEAMS, staff, viewer) ret 8; 0x76da80 (NATIONAL_TEAMS, staff,
#   nation, squad) ret 0xc; 0x62ed30 Injury ret 4; 0x62ee30 ret 0x10; 0x630af0 ret 0x14; 0x62bf60 ret 4; 0x630c50
#   ret 0xc; 0x631dc0 ret 8; 0x418ff0 AWOL::has_missed_training ret 4; 0x55d3c0 Discipline::get_slot_text ret 0x10;
#   0x795750 NOTES ret 8; 0x7f8610 SCOUT_MANAGER ret 4; 0x7f7920 get_compare_player ret 4; 0x7c1b20 PlayerStats ret
#   0xc; 0x7c14b0 ret 0xc; 0x7c2750 ret 0x1c; 0x7c25b0 ret 0x20; cdecl: 0x41c790, 0x53ee50, 0x669630, 0x5448c0,
#   0x53c9b0, 0x9471cd (toupper), 0x945640 (_ftol), 0x5ed070, 0x5ed780, 0x5edcc0, the tabs 0x87a510 / 0x87cd90 /
#   0x87dd80 / 0x885820 (top, bottom) and the squad and history menu 0x88b910 (6 args): those stay in the original.
# Kept from the original (DEFECTS, docs/DEFEITOS_DO_ORIGINAL.md candidates): "Approach To Buy" does not open the Action
#   button when it is the first entry (parent -1); with no viewer the rehabilitation check reads the viewer's club at
#   address 0x39 (the original would fault; the screen always has a viewer in practice); the national view with no club
#   passes the national team name's address as the grammar args. Stack garbage in the high halves of the kit colours,
#   the job byte given to sign_contract, the view byte given to 0x88b910 and the row count of the injuries table:
#   zero here.
# Comparer: the builder creates game objects (the offer list in param 3, a staff contract signed with the contract
#   manager, the career history in params 6/7), so only the original builds for the game; the plan read before it
#   (profile_screen_plan) is checked against what it created, and the copy builds its display list in a scratch (the
#   screen back to before, the game as the original left it) compared with the original's (display list, params,
#   0xa84aa8, 0xae1080, the squad flag and the staff state flag).
# How to reach it: click any person's name (a link anywhere, e.g. Squad > a player); then its tabs and the Action menu.
TELAS = True
ROTINAS = [('staff_screens_build_profile_screen', 0x870700, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a tela de perfil da pessoa',
        'the staff profile screen',
        [('staff_screens_build_profile_screen', 'staff_screens_build_profile_screen', '0x870700', 'staff_screens.cpp', 'cdecl, SEH',
          'Monta o perfil da pessoa do parâmetro 0 na aba do parâmetro 1. Sem a lista de ofertas (parâmetro 3) cria uma e a enche com as ofertas pela pessoa; sem ficha de jogador o parâmetro 5 perde os bits 1 a 3 (1 se nada sobrar) e o reserva vira 1; sem ficha de técnico perde o bit 4; na aba 5 com o bit 2, sem histórico e com ficha de jogador, monta o histórico da carreira nos parâmetros 6 e 7; para quem não tem contrato e tem ficha de não jogador, clube e cargo de contrato, cria um contrato (tipo 1) e o registra. O cabeçalho pela vista (parâmetro 2): o clube com o número da camisa e "Reserves", o técnico de seleção, a vista da seleção, ou o título com "(Retired)"; sem nação, a caixa de erro e nada mais. O menu Action: votar no prêmio (só o botão); para os jogadores do banco: lista de observação, subir e descer de time, situação de transferência, rescindir empréstimo, novo contrato, rescindir, chamar de volta do empréstimo, disciplina, licença, dispensa, as ofertas do clube do observador (oferecer contrato, retirar e mudar a proposta), teste, comprar, emprestar, contratar (Bosman), as seleções (convocar, mover e tirar da seleção, da B e da sub-21/23, pela idade), observar, apelido, relatórios do fisioterapeuta e do treinador, reabilitação (ou limpa a marca) e a nota do técnico; a comparação de jogadores. As abas (perfil, lesões e suspensões, contrato, transferência, histórico, cinzentas pelas regras), a linha do nascimento, nacionalidade e convocações com os links das seleções, o botão de vista (jogando, não jogando, relatório do olheiro), as posições ou o cargo, a tabela das estatísticas (a temporada do histórico ou a ficha) e a aba: lesões e suspensões aqui (lesão, tipo, condição, treino e suspensões), as outras no original; o menu do elenco e Voltar/Avançar',
          'Builds the profile of the person in param 0 on the tab in param 1. Without the offer list (param 3) creates one and fills it with the offers for the person; without a player sheet param 5 loses bits 1 to 3 (1 when nothing is left) and the reserve becomes 1; without a manager record it loses bit 4; on tab 5 with bit 2, no career history and a player sheet, builds the career history into params 6 and 7; for someone without a contract who has a non-player sheet, a club and a contract job, creates a contract (type 1) and signs it. The heading by the view (param 2): the club with the squad number and "Reserves", the national manager, the national view, or the title with "(Retired)"; without a nation, the error box and nothing more. The Action menu: vote for the award (only the button); for the database players: shortlist, squad moves, transfer status, terminate the loan, new contract, terminate, recall from loan, discipline, leave of absence, release, the viewer club\'s offers (offer contract, withdraw and change the bid), trial, buy, loan, sign (Bosman), the national teams (call up, move and remove from the squad, the B squad and the under-21/23 by age), scout, nickname, the physio and coach reports, rehabilitation (or the flag cleared) and the manager note; the compare players entries. The tabs (profile, injuries and bans, contract, transfer, history, greyed by their rules), the birth, nationality and caps line with the national teams linked, the view button (playing, non-playing, scout report), the positions or the job, the statistics table (the history season or the sheet) and the tab: injuries and bans here (injury, type, condition, training and bans), the others in the original; the squad menu and Back/Next')])
