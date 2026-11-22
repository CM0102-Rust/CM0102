# The Web Sites screen (contract_screens.cpp; reader tel34, the screens round, 21/11/2026). Uses the helpers of
# tel34_1 (tel34_game_title, tel34_credits_list_area, tel34_credits_title_rows, tel34_web_site_row,
# tel34_credits_list_done, the two row builders) and tel26_1: goes after tel34_1.
# Convention: cdecl(void), ret 0. background_set_random_picture 0x41bfb0 cdecl.
# NEEDS (0x513a20, 3) in RELATIVAS of rust/src/detours.rs: the prologue's first 5 bytes end inside the call rel32 to
# 0x41bfb0 at offset 3 (51 56 57 e8 ..); my compile copy had the line. Without it, leave this item out (it is alone
# in its item for that).
TELAS = True
ROTINAS = [('web_sites_screen_build', 0x513a20, [0x51, 0x56, 0x57, 0xe8, 0x88, 0x85, 0xf0, 0xff])]
DOCS = ('os sites (2): a tela dos sites',
        'the web sites (2): the web sites screen',
        [('web_sites_screen_build', 'web_sites_screen_build', '0x513a20', 'contract_screens.cpp', 'cdecl',
          'monta a tela dos sites: figura aleatória, título, menu, a lista rolável com a Sports Interactive (www.sigames.com e community.sigames.com), a editora e os sites de torcedores, rolada até o parâmetro 0, e Voltar/Avançar',
          'builds the web sites screen: a random picture, title, menu bar, the scrolling list with Sports Interactive (www.sigames.com and community.sigames.com), the publisher and the fan club sites, scrolled to parameter 0, and Back/Next')])
