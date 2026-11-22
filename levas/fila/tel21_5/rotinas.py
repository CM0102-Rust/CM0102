# The search screens (5): building the search Filters screen (search_screens.cpp; reader tel21, the screens round,
# 21/11/2026). Uses tel26_1 (the display list helpers, tel26_slot_manager, tel26_error_box, tel26_copy_text, JOB_*,
# job_colour) and tel21_4 (search_session_current, search_filters_fields, search_filters_get_minimum,
# SEARCH_SCREENS_CPP): goes after both. The two attribute sorts it passes to qsort are tel21_3's (by address, as the
# original).
# 0x8133a0 cdecl() ret 0, SEH frame (no object with a destructor): the builder of the Filters screen (a pointer in
# 0x813210). One routine from 0x8133a0 to the ret at 0x8180f3 (19795 bytes), hooked whole, in named pieces
# (filters_screen_attributes_page, filters_screen_general_page and its rows). The cm0102-1080.exe changes no byte of it
# (the whole-screen area keeps 0x31f x 0x257 there too).
# Callees checked at the call sites: Display::add_area 0x55fba0 / add_item 0x55f990 thiscall on the human's display;
# ScreenManager add_text_item 0x803fd0 (11 args), add_button_item 0x804030 (12), add_area_text 0x804100 (9),
# add_area_button 0x8040a0 (11); draw_pitch 0x8ac600 cdecl(x1, y1, x2, y2, 4 out pointers, 1, area, 4.0f) (the area as
# a short); search_screens_filter_bit_to_index 0x81b0e0 cdecl(lo, hi) -> al; key_nation_sorted_order 0x66a0c0 and
# get_regions_sorted_by_name 0x545dc0 cdecl -> char list; nation_get_nationality_adjective 0x6181d0, get_region_name
# 0x616f20, database_get_language_name 0x53cee0 cdecl; database_get_national_team 0x53b3d0; database_is_basque_club
# 0x53d7f0; database_club_has_reserve_squad 0x542740; club_get_linked_b_team 0x540a50 cdecl(club, 0, 1);
# Cash::set_from_preset_index 0x43f620 thiscall(cash, char) ret 4; cash_get_display_amount 0x43f460 thiscall(cash, 5)
# -> edx:eax; cash_format_money 0x441c10 cdecl(text, 0xc8, amount, 5); qsort 0x945333.
# The colour arguments the original pushes with garbage upper halves are stored as words by Guio::init (+0x70..0x7e):
# the copy pushes them zero-extended (job_colour).
# DEFECT of the original kept apart: the Job row shows an uninitialised stack buffer when the filters' job value is
# outside 3..9 (the copy: empty text).
TELAS = True
ROTINAS = [('search_screens_build_filters_screen', 0x8133a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas de busca (5): a montagem da tela de filtros da busca',
        'the search screens (5): building the search Filters screen',
        [('search_screens_build_filters_screen', 'search_screens_build_filters_screen', '0x8133a0', 'search_screens.cpp', 'cdecl',
          'monta a tela de filtros da busca atual: precisa do técnico humano da vaga e da nação dele (do clube, da seleção ou a própria; sem eles, as caixas de erro 0x104e e 0x105d); a moldura centrada (menor na busca de comissão), o título "Filters for <busca>", as abas Geral e Atributos, Limpar (só texto enquanto a máscara geral é a padrão), Cancelar e Ok, o campo desenhado atrás e sete grades de colunas no corpo. Na página Atributos, cada atributo (12 de comissão ou 31 de jogador, pelo nome) com o nome, "-", o mínimo e "+", dois por linha, e "Match" com quantos têm de bater. Na página Geral, linha a linha: Texto (fora da lista de jogos do olheiro), Nacionalidade (com o menu das nações principais e das regiões, UE e bascos), Idade, Idioma (os três idiomas da nação), Base (com o menu de nações e regiões), Cargo (comissão) ou Valor (faixas de dinheiro), Situação, Contrato (Sem proteção só com as regras da FIFA), Posição e Lado (jogadores), Habilidade (lista do olheiro), Lesões, Interessado, Só observados e Incluir elenco (com Principal/Reservas ou A/B)',
          "builds the Filters screen of the current search: it needs the slot's human manager and his nation (his club's, his national side's or his own; without them, the error boxes 0x104e and 0x105d); the centred frame (smaller for a staff search), the title \"Filters for <search>\", the General and Attributes tabs, Clear (plain text while the general mask is the default), Cancel and Ok, the pitch drawn behind and seven column layouts in the body. On the Attributes page, each attribute (12 non-player or 31 player ones, by name) with its name, \"-\", the minimum and \"+\", two per row, and \"Match\" with how many must match. On the General page, row by row: Text (not on a scout's match list), Nationality (with the menu of the key nations and the regions, EU and Basque), Age, Language (the nation's three languages), Based (with the nations and regions menu), Job (staff) or Value (money bands), Status, Contract (Unprotected only under the FIFA rules), Position and Side (players), Ability (scout's list), Injuries, Interested, Scouted Only and Include Squad (with Senior/Reserve or A/B)")])
