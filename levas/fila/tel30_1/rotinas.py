# The background pictures (1): background.cpp (reader tel30, the screens round, 21/11/2026; a new group from the
# disassembly sweep: helpers the community map placed in no file).
# Conventions checked in the disassembly (every push against esp): all cdecl.
# - 0x41b3d0 setup_background_system() (called by 0x5d72c0 at the game start); the subsection characters through the
#   jump table 0x41b828 (4 targets) with the byte index 0x41b838 (0x7e entries: 0 / ',' / '}' add, ' ' skip, '{' second
#   list then add, the rest collect); 0x90cba0 utils_read_config_line cdecl(FILE, out) -> eax; 0x951c50 stricmp;
#   0x945667 realloc; 0x944e46 malloc; 0x9452ca free; the fatal box like tel26_error_box with background.cpp (0x989934)
#   lines 0x90, 0x95 and 0xbe, then 0x5cc7d0, 0x5a8f60 and exit(-1).
# - 0x41b8c0 background_add_picture(sub, name, second) -> eax (0 or the error text); 0x945b13 _makepath cdecl(out, 0,
#   dir, name, 0); 0x952f5b _strdup cdecl(text).
# - 0x41bc20 verify_background_config() (called by 0x828110); jump table 0x41bf4c (11 targets: sections 0..5 searched,
#   6..10 not); the English names 0x5453f0 / 0x545880 / 0x545990 / 0x5455a0 / 0x5456a0 cdecl(record); the staff label
#   sprintf("%s %s %d/%d/%d", first, second, day, month + 1, year) with 0x54cfb0 Date::get_month and 0x54cf70
#   get_day_of_month thiscall(date) ret 0 (the year and the month + 1 are pushed before them and stay as the sprintf
#   arguments); 0x66f4e0 langlib_sprintf; 0x802f80 ScreenManager::send_network_status_message thiscall(0xb74450, text,
#   done, total, -1) ret 0x10.
# - 0x41bfb0 background_set_random_picture(); 0x41c0d0 background_set_picture(title); the finders 0x41c9d0 / 0x41cad0 /
#   0x41cbd0 / 0x41ccd0 / 0x41cdd0 / 0x41ced0 (record, second) -> eax, identical but the section and the name routine
#   (the staff one builds the label); 0x90cfc0 utils_random; 0x7fee80 / 0x7feeb0 ScreenManager::get_ / set_background_
#   picture thiscall on 0xb74450.
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel26_1 (tel26_error_box), tel29_1 (copy_c_string, c_string_len, current_screen_node, screen_node_fields).
#   tel30_2 uses this item's helpers (background_screen_open, background_set, background_set_file, background_compare,
#   the BACKGROUND_* constants): this item goes first.
# Comparers: the setters and the finders side by side over the generator (rng STATE_REGIONS) and the current screen's
#   picture path, with the return value; setup_background_system, background_add_picture (they allocate the tables and
#   read pics.cfg once) and verify_background_config (boxes, status messages) only the original.
# Defects of the original: a subsection line before any section, without "-verifypics", is read into section -1 (the
#   10 bytes before the table at 0x989860, over the "pictures on" flag); '{' only takes effect for the next picture when
#   nothing was collected before it.
MOTOR = True
TELAS = True
ROTINAS = [('setup_background_system', 0x41b3d0, [0x81, 0xec, 0xe8, 0x0a, 0x00, 0x00]),
           ('background_add_picture', 0x41b8c0, [0x81, 0xec, 0x04, 0x01, 0x00, 0x00]),
           ('verify_background_config', 0x41bc20, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('background_set_random_picture', 0x41bfb0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('background_set_picture', 0x41c0d0, [0xa1, 0x58, 0x98, 0x98, 0x00]),
           ('background_find_nation_picture', 0x41c9d0, [0xa1, 0x58, 0x98, 0x98, 0x00]),
           ('background_find_club_picture', 0x41cad0, [0xa1, 0x58, 0x98, 0x98, 0x00]),
           ('background_find_comp_picture', 0x41cbd0, [0xa1, 0x58, 0x98, 0x98, 0x00]),
           ('background_find_award_picture', 0x41ccd0, [0xa1, 0x58, 0x98, 0x98, 0x00]),
           ('background_find_stadium_picture', 0x41cdd0, [0xa1, 0x58, 0x98, 0x98, 0x00]),
           ('background_find_staff_picture', 0x41ced0, [0xa1, 0x58, 0x98, 0x98, 0x00])]
_A = 'background.cpp'
DOCS = ('as fotos de fundo (1): o pics.cfg, a foto sorteada e as buscas por nação, clube, competição, prêmio, estádio e pessoa',
        'the background pictures (1): pics.cfg, the random picture and the finders by nation, club, competition, award, stadium and staff',
        [('setup_background_system', 'setup_background_system', '0x41b3d0', _A, 'cdecl',
          'lê o pics.cfg da pasta das fotos: as linhas "[seção]" escolhem uma das 11 seções e as outras são subseções "nome:foto,foto{foto}" cujas fotos (achadas pelo background_add_picture) vão para as duas listas; subseção sem fotos sai; falta de memória dá a caixa fatal',
          'reads pics.cfg from the pictures folder: "[section]" lines choose one of the 11 sections and the others are subsections "name:picture,picture{picture}" whose pictures (found by background_add_picture) go to the two lists; a subsection without pictures is dropped; running out of memory gives the fatal box'),
         ('background_add_picture', 'background_add_picture', '0x41b8c0', _A, 'cdecl',
          'procura o arquivo da foto na pasta, em stadium\\ e em old\\, pondo um "_" na frente enquanto não acha (desiste em nome que já começa com "_"), e acrescenta o caminho achado a uma das listas da subseção',
          'looks for the picture file in the folder, under stadium\\ and old\\, putting a "_" in front while not found (giving up on a name that already starts with "_"), and adds the path found to one of the subsection\'s lists'),
         ('verify_background_config', 'verify_background_config', '0x41bc20', _A, 'cdecl',
          'confere os nomes das subseções das seis primeiras seções contra o banco de dados (nações, clubes, estádios, competições, prêmios, pessoas pelo nome e nascimento) e mostra o andamento',
          'checks the subsection names of the first six sections against the database (nations, clubs, stadiums, competitions, awards, staff by name and birth) and shows the progress'),
         ('background_set_random_picture', 'background_set_random_picture', '0x41bfb0', _A, 'cdecl',
          'põe uma foto sorteada numa tela que aceita foto: até 6 seções a partir de uma sorteada entre as seis primeiras (sem os estádios), subseção e foto sorteadas; nenhuma (ou fotos desligadas) põe ":"',
          'puts a random picture on a screen that takes one: up to 6 sections from a random one of the first six (no stadiums), a random subsection and picture; none (or pictures off) puts ":"'),
         ('background_set_picture', 'background_set_picture', '0x41c0d0', _A, 'cdecl',
          'põe o caminho dado como foto da tela; com as fotos desligadas, um caminho não nulo vira ":"',
          'sets the given path as the screen\'s picture; with pictures off, a non-null path becomes ":"'),
         ('background_find_nation_picture', 'background_find_nation_picture', '0x41c9d0', _A, 'cdecl',
          'uma foto sorteada da subseção da nação (pelo nome em inglês; a última que bate) ou de uma subseção sorteada para nação nula; o segundo argumento escolhe a lista; 0 quando não há',
          'a random picture of the nation\'s subsection (by English name; the last match) or of a random subsection for a null nation; the second argument picks the list; 0 when none'),
         ('background_find_club_picture', 'background_find_club_picture', '0x41cad0', _A, 'cdecl',
          'o mesmo para o clube', 'the same for the club'),
         ('background_find_comp_picture', 'background_find_comp_picture', '0x41cbd0', _A, 'cdecl',
          'o mesmo para a competição de clubes', 'the same for the club competition'),
         ('background_find_award_picture', 'background_find_award_picture', '0x41ccd0', _A, 'cdecl',
          'o mesmo para o prêmio (a competição de pessoas)', 'the same for the award (the staff competition)'),
         ('background_find_stadium_picture', 'background_find_stadium_picture', '0x41cdd0', _A, 'cdecl',
          'o mesmo para o estádio', 'the same for the stadium'),
         ('background_find_staff_picture', 'background_find_staff_picture', '0x41ced0', _A, 'cdecl',
          'o mesmo para a pessoa, pelo rótulo "nome sobrenome dia/mês/ano" do nascimento',
          'the same for the staff member, by the label "first second day/month/year" of birth')])
