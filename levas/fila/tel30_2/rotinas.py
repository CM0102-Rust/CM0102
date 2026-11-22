# The background pictures (2): background.cpp (reader tel30, the screens round, 21/11/2026; a new group from the
# disassembly sweep: helpers the community map placed in no file).
# Conventions checked in the disassembly: all cdecl; the record setters (record, second, exact) -> eax (0 / 1), the
# section setter (int section, exact) -> eax; 0x53b3d0 database_get_national_team cdecl(nation); 0x4afd80
# comp_util_get_latest_placed_club cdecl(comp, 0); 0x90cfc0 utils_random; 0x7fee80 / 0x7feeb0 ScreenManager::get_ /
# set_background_picture thiscall on 0xb74450; 0x90bde0 utils_build_file_path("CM3_PICTURES", name).
# Called by: nation screens (0x49c120, 0x5a0840...), club screens (0x454900 and 30 more), competition screens
# (0x414cf0, 0x48fcd0...), award screens (0x414cf0, 0x416e40), stadium (0x718e90...), staff (0x47c180 and 30 more) and
# the section setter by the board request, coach report and other screens (0x4253f0, 0x48bd20...).
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel30_1 (the finders, background_set_random_picture, background_screen_open, background_set,
#   background_set_file, background_compare, the BACKGROUND_* constants), tel29_1 (through tel30_1).
# Comparers: side by side over the generator (rng STATE_REGIONS) and the current screen's picture path, with the
#   return value (background_compare).
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('background_set_nation_picture', 0x41c100, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('background_set_club_picture', 0x41c250, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('background_set_comp_picture', 0x41c410, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('background_set_award_picture', 0x41c560, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('background_set_stadium_picture', 0x41c670, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('background_set_staff_picture', 0x41c790, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('background_set_section_picture', 0x41c8e0, [0x56, 0x57, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
_A = 'background.cpp'
DOCS = ('as fotos de fundo (2): a foto de uma nação, um clube, uma competição, um prêmio, um estádio, uma pessoa ou uma seção',
        'the background pictures (2): the picture of a nation, a club, a competition, an award, a stadium, a staff member or a section',
        [('background_set_nation_picture', 'background_set_nation_picture', '0x41c100', _A, 'cdecl',
          'numa tela que aceita foto: a da nação, depois a do clube da seleção e a do estádio nacional, depois uma sorteada da seção das nações e por fim a sorteada geral; com "só exata" a falta da própria dá 0; fotos desligadas põem ":"',
          'on a screen that takes a picture: the nation\'s, then its national team club\'s and its national stadium\'s, then a random one of the nations section and finally the general random one; with "exact only" a missing own picture gives 0; pictures off put ":"'),
         ('background_set_club_picture', 'background_set_club_picture', '0x41c250', _A, 'cdecl',
          'o mesmo para o clube: o estádio, o segundo estádio, a divisão e a nação de reserva',
          'the same for the club: the stadium, the second stadium, the division and the nation as fallbacks'),
         ('background_set_comp_picture', 'background_set_comp_picture', '0x41c410', _A, 'cdecl',
          'o mesmo para a competição: o último campeão e a nação de reserva',
          'the same for the competition: the latest winner and the nation as fallbacks'),
         ('background_set_award_picture', 'background_set_award_picture', '0x41c560', _A, 'cdecl',
          'o mesmo para o prêmio: a nação de reserva', 'the same for the award: the nation as fallback'),
         ('background_set_stadium_picture', 'background_set_stadium_picture', '0x41c670', _A, 'cdecl',
          'o mesmo para o estádio: a nação da cidade de reserva', "the same for the stadium: the city's nation as fallback"),
         ('background_set_staff_picture', 'background_set_staff_picture', '0x41c790', _A, 'cdecl',
          'o mesmo para a pessoa: o clube e a nação de reserva', 'the same for the staff member: the club and the nation as fallbacks'),
         ('background_set_section_picture', 'background_set_section_picture', '0x41c8e0', _A, 'cdecl',
          'uma foto sorteada de uma seção (treino, diretoria...); sem ela, 0 com "só exata", senão a sorteada geral',
          'a random picture of a section (training, boardroom...); without one, 0 with "exact only", otherwise the general random one')])
