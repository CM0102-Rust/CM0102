# The award screens (award_screens.cpp; reader tel27, the screens round, 21/11/2026): the award screen (year, month
# and team awards), its menu forwarders, its clicks, and the nominations screen with its opener from the news.
# Conventions checked in the disassembly (every push against esp and the ret of each routine):
# - 0x414c20 award_screens_open_award_screen cdecl(staff comp); 0x414cf0 award_screen_draw cdecl(); 0x4156a0
#   award_screen_draw_year_award and 0x415f60 award_screen_draw_team_award cdecl(int *y, int *y2) (SEH frame for the
#   local Formation, which has no destructor call); 0x416960 award_screen_handle_action cdecl(short widget pushed as
#   a dword) -> eax; 0x416cb0 award_screens_open_nominations_from_news cdecl(manager, news id) (SEH for the local
#   NewsItem, no destructor call); 0x416d50 award_screens_open_nominations_screen cdecl(award); 0x416e40
#   award_nominations_screen_draw cdecl(); 0x4172f0 award_nominations_screen_handle_action cdecl(widget) -> eax.
# - 0x415600 ScreenManager::add_menu_area thiscall(owner) ret 4; 0x415640 ScreenManager::add_menu_item
#   thiscall(area, text, id, data) ret 0x10.
# - Called: 0x414a60 AWARD_MANAGER::find_award_by_staff_comp thiscall ret 4; 0x41c560 background_set_award_picture
#   cdecl(3); 0x9304d0 YEAR_AWARD::get_history_list thiscall; 0x8b00a0 TEAM_AWARD::get_history_list thiscall;
#   0x68d2e0 LLIST::get_current thiscall; 0x54d3b0 date_get_month_name cdecl(char); 0x54a650
#   Date::get_short_date_string thiscall ret 0xc; 0x8ac600 draw_pitch cdecl(11, the last 4.0f); 0x5b27c0
#   Formation::Formation thiscall; 0x5b2e80 Formation::has_central_slot_in_line thiscall ret 8; 0x8ad780
#   get_pitch_point_for_position_flags cdecl(8); 0x8b0730 TEAM_AWARD::apply_formation thiscall ret 4; 0x418140
#   AWARD_SHORTLIST::get_vote_status thiscall ret 4 -> ax; 0x4148e0 AWARD_MANAGER::set_pending_vote thiscall ret 0x10;
#   0x786b70 News::find_item_by_id thiscall ret 0xc; 0x8702a0 staff_screens_open_staff_profile cdecl(5); the nation
#   award object's slot 4 (0x10) thiscall(award) ret 4 (the no-op 0x583470 or a refresh, e.g. Sweden's team of the week).
# Uses: tel26_1 (tel26_error_box, tel26_add_area/_item/_button, tel26_area_text/_button, tel26_text_area,
#   tel26_back_next, tel26_item_id/_value, tel26_slot_manager, tel26_copy_text, job_colour), tel01_1
#   (screens_current_display, DisplayAddAreaFn/ItemFn, compare_screens_forwarder, screenmanager_add_ticked_popup_item),
#   tel29_1 (screen_record, current_screen_node, screen_history, screen_node_fields, STR_EMPTY) and tel22_1
#   (comp_lang_format, comp_crt_format, comp_text_item); tel27_2 uses screen_newly_opened and compare_opened_params.
# Inline in award_screen_draw (not separate routines): the competition menu (award_screen_menu) and the month award's
#   list (award_screen_month_award_list).
# Defects of the original: award_screen_draw_year_award with a history list that has no record uses three
#   uninitialised locals (the rows, the row and the area) to fill empty rows (here no rows); award_screen_handle_action
#   with an award of a kind other than 0/1/2 (none exists) returns a stack word; award_screen_draw tests the award for
#   null after reading it (the 1181 box is unreachable).
# Kept from the original: the colours' high words (garbage in some pushes) go zero-extended (the forwarders model the
#   argument halves, as tel01_1).
TELAS = True
ROTINAS = [('award_screens_open_award_screen', 0x414c20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_screen_draw', 0x414cf0, [0x81, 0xec, 0x50, 0x03, 0x00, 0x00]),
           ('screen_manager_add_menu_area', 0x415600, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x15, 0xaa, 0x31, 0xae, 0x00]),
           ('screen_manager_add_menu_item', 0x415640, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x10]),
           ('award_screen_draw_year_award', 0x4156a0, [0x81, 0xec, 0xe8, 0x00, 0x00, 0x00]),
           ('award_screen_draw_team_award', 0x415f60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('award_screen_handle_action', 0x416960, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('award_screens_open_nominations_from_news', 0x416cb0, [0x6a, 0xff, 0x68, 0xab, 0x3d, 0x95, 0x00]),
           ('award_screens_open_nominations_screen', 0x416d50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_nominations_screen_draw', 0x416e40, [0x81, 0xec, 0xec, 0x00, 0x00, 0x00]),
           ('award_nominations_screen_handle_action', 0x4172f0, [0x53, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('as telas dos prêmios: a tela de um prêmio (do ano, do mês e da seleção), o menu, os cliques e a tela das indicações',
        "the award screens: an award's screen (year, month and team awards), its menu, its clicks and the nominations screen",
        [('award_screens_open_award_screen', 'award_screens_open_award_screen', '0x414c20', 'award_screens.cpp', 'cdecl',
          'abre a tela do prêmio de uma competição de pessoas (sem ela, a caixa de erro); parâmetro 0 = o prêmio, 1 = 0',
          "opens the award screen of a staff competition (without one, the error box); parameter 0 = the award, 1 = 0"),
         ('award_screen_draw', 'award_screen_draw', '0x414cf0', 'award_screens.cpp', 'cdecl',
          'monta a tela do prêmio: o título (da competição, da nacionalidade ou internacional), o menu das outras competições quando há duas ou mais sequências, a lista "Ver" dos prêmios da mesma competição, "Indicações" quando há, a atualização virtual do objeto de prêmios da nação e a lista do prêmio (do ano, da seleção ou dos meses)',
          "builds the award screen: the title (competition, nationality or international), the menu of the other competitions when there are two runs or more, the \"View\" list of the same competition's awards, \"Nominations\" when there are any, the nation award object's virtual refresh and the award's list (year, team or months)"),
         ('screen_manager_add_menu_area', 'screen_manager_add_menu_area', '0x415600', 'award_screens.cpp', 'thiscall',
          'uma área de menu presa a um botão (opções 0x30, a cor do menu suspenso)',
          'a menu area hanging from a button (flags 0x30, the popup colour)'),
         ('screen_manager_add_menu_item', 'screen_manager_add_menu_item', '0x415640', 'award_screens.cpp', 'thiscall',
          'um item de menu (tipo 2, as cores do menu suspenso) com a identificação e o dado',
          'a menu item (kind 2, the popup colours) with its id and data'),
         ('award_screen_draw_year_award', 'award_screen_draw_year_award', '0x4156a0', 'award_screens.cpp', 'cdecl',
          'a lista do prêmio do ano: os três primeiros da última temporada em destaque, depois uma linha por temporada anterior com o vencedor, o clube e o valor; sem histórico, "Nenhum prêmio foi dado."',
          "the year award's list: the latest season's top three in a block, then one row per earlier season with the winner, the club and the value; with no history, \"No awards have been made.\""),
         ('award_screen_draw_team_award', 'award_screen_draw_team_award', '0x415f60', 'award_screens.cpp', 'cdecl',
          'a seleção do prêmio: a temporada ou a data, setas para a anterior e a seguinte, o elenco (número, nome, clube; reservas em outra cor) e os onze desenhados no campo pela formação',
          "the team award: the season or date, arrows to the older and newer teams, the squad (number, name, club; substitutes in another colour) and the eleven drawn on the pitch by the formation"),
         ('award_screen_handle_action', 'award_screen_handle_action', '0x416960', 'award_screens.cpp', 'cdecl',
          'trata o clique na tela do prêmio: outro prêmio do menu reabre a tela, outro da lista "Ver" troca o parâmetro 0, as indicações, a seleção anterior ou seguinte; o resto vai para a barra de menu',
          "handles a click on the award screen: another award of the menu reopens the screen, another of the View list changes parameter 0, the nominations, the older or newer team; the rest goes to the menu bar"),
         ('award_screens_open_nominations_from_news', 'award_screens_open_nominations_from_news', '0x416cb0', 'award_screens.cpp', 'cdecl',
          'o botão "Indicações" da notícia: acha a notícia do treinador e abre as indicações do prêmio que ela indica (vagas 0 e 1)',
          'the news item\'s "Nominations" button: finds the manager\'s news item and opens the nominations of the award it names (slots 0 and 1)'),
         ('award_screens_open_nominations_screen', 'award_screens_open_nominations_screen', '0x416d50', 'award_screens.cpp', 'cdecl',
          'abre as indicações de um prêmio; o treinador pode votar se ainda não votou',
          "opens an award's nominations; the manager may vote when he has not voted yet"),
         ('award_nominations_screen_draw', 'award_nominations_screen_draw', '0x416e40', 'award_screens.cpp', 'cdecl',
          'monta a tela das indicações: cada indicado com o clube e a nação, o voto do treinador em amarelo; com voto aberto, os botões Voltar e Avançar próprios',
          "builds the nominations screen: each nominee with his club and nation, the manager's vote in yellow; with the vote open, its own Back and Next buttons"),
         ('award_nominations_screen_handle_action', 'award_nominations_screen_handle_action', '0x4172f0', 'award_screens.cpp', 'cdecl',
          'trata o clique nas indicações: um indicado vira o voto (e abre o perfil dele), Voltar apaga o voto; o resto vai para a barra de menu',
          "handles a click on the nominations: a nominee becomes the vote (and his profile opens), Back clears the vote; the rest goes to the menu bar")])
