# The Setup screens, part 7 (Setup.cpp, part 2; reader tel12, the screens round, 21/11/2026): appointing the human
# manager, the default league by language and creating the game. Uses setup.rs (setup_warning, setup_fatal_error,
# check_game_cd, copy_text), the names of tel11_1 (G_PLAYER_SETUP_GERMAN_FLAG, G_USE_REAL_PLAYERS,
# FN_PLAYER_SETUP_CFG_LOAD) and CONFIG_OPTIONS of tel12_1: goes after them.
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0):
# - 0x826f20 appoint_human_manager_to_club(staff, club) -> 1 or 0 (called by create_game_from_setup_file 0x827110 and by
#   tel11_4); 0x827d50 select_default_league_by_language() -> 1 or 0 (a jump table at 0x8280dc over the 13 languages);
#   0x828110 create_game(int, ignored) -> 1 or 0 (SEH frame; called by 0x827110 and tel11_1's setup_create_chosen_game).
# - MANAGER_MANAGER::remove_club_manager 0x69b790 thiscall(mm, club, 3, 0, 1) ret 0x10, appoint_manager_to_club
#   0x69ebc0 thiscall(mm, club, staff) ret 8; Friendly::remove_club_from_ai_plans 0x5c64e0 thiscall ret 4;
#   club_get_linked_b_team 0x540a50 cdecl(club, 0, 1); human_manager_is_human_club 0x600a50 cdecl(u21, 1, 1, 0, 0).
# - create_game: the subsystem constructors thiscall with (0), () or the arguments of CreateArg (FIXTURE_MANAGER gets a
#   dword with the year in the low word and stack leftover above); Date::sub_weeks 0x54cab0 thiscall(this, out, 4) ret
#   8; Date::operator= 0x417c10 thiscall(this, result, src) ret 8; Date::days_since 0x54cdb0 thiscall(this, date by
#   value) ret 8; the trims take the database size char (the second one pushes edx with leftover above the byte);
#   ReserveComp::create_sub_comps, CONTRACT_MANAGER::initialise_all_staff_contracts, FinanceTable::refresh_all_wage_bills
#   and SQUAD_MANAGER::init_new_game thiscall on their globals.
# - The 1080 exe (and the portable one) patch create_game's CD search (0x8281ae jne -> nop nop, 0x8281f0 test -> xor):
#   create_game_find_cd reads those bytes from the running exe, as check_game_cd does for load_game_data; the routine's
#   signature (assinaturas.py) differs between the exes for that reason.
# Which routines run in the automatic career: create_game (once, from create_game_from_setup_file with auto_setup.cfg,
# coverage 1) and appoint_human_manager_to_club (once, the career's human manager on holiday at a drawn club, coverage
# 1); select_default_league_by_language does not (coverage 0: Start New Game and the language in Game Settings).
# Attention to the Rust x2 careers and the detector: the whole new game is built by the copy in Rust mode.
# DEFECTS of the original kept: create_game checks the Media global after FOG_OF_WAR (as load_game_data does), so a
# failed FOG_OF_WAR is not caught; create_game's argument is never read.
MOTOR = True
TELAS = True
ROTINAS = [('appoint_human_manager_to_club', 0x826f20, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('select_default_league_by_language', 0x827d50, [0xa1, 0xa8, 0x23, 0xae, 0x00]),
           ('create_game', 0x828110, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas de criar o jogo (7): o técnico humano no clube, a liga padrão pelo idioma e criar o jogo',
        'the game creation screens (7): the human manager at the club, the default league by language and creating the game',
        [('appoint_human_manager_to_club', 'appoint_human_manager_to_club', '0x826f20', 'Setup.cpp', 'cdecl',
          'põe o técnico humano no clube: sai o técnico do clube (ou o do time B ligado), numa seleção também o da sub-21 se não for humano; reputações do técnico pela do clube (atual e local = rep/10 + 3000, mundial = rep × 0,4 + 750), lealdade 20; o clube sai dos planos de amistosos e o técnico é nomeado',
          'puts the human manager at the club: the club\'s manager leaves (or the linked B team\'s), for a national side also the under-21 one unless human; the manager\'s reputations from the club\'s (current and home = rep/10 + 3000, world = rep × 0.4 + 750), loyalty 20; the club leaves the friendlies\' plans and the manager is appointed'),
         ('select_default_league_by_language', 'select_default_league_by_language', '0x827d50', 'Setup.cpp', 'cdecl',
          'escolhe a liga padrão pelo idioma do jogo: limpa as ligas de todas as nações e marca a nação-chave do idioma (0/1 Inglaterra, 2 França, 3 Alemanha, 4 Itália, 5 Espanha, 6 Dinamarca, 7 Holanda, 8 Noruega, 9 Portugal, 10 Suécia, 11 Japão, 12 Coreia do Sul) como escolhida e principal',
          'picks the default league by the game language: clears every nation\'s league and marks the language\'s key nation (0/1 England, 2 France, 3 Germany, 4 Italy, 5 Spain, 6 Denmark, 7 Holland, 8 Norway, 9 Portugal, 10 Sweden, 11 Japan, 12 South Korea) as selected and main'),
         ('create_game', 'create_game', '0x828110', 'Setup.cpp', 'cdecl',
          'cria o jogo novo: disco e memória, o CD, o banco (CM3_DATA) se faltar, os cortes do banco pelo tamanho, a Grécia com 14 clubes, a ordem de reputação, a data de início (−4 semanas), os registros de pessoas e de clubes, a iniciação dos jogadores e os passos dos subsistemas com a barra de progresso (com a passada dos clubes: estádio próprio, reputação, públicos e treino); depois corre até a data de início',
          'creates the new game: disk and memory, the CD, the database (CM3_DATA) if missing, the database trims by size, Greece with 14 clubs, the reputation order, the start date (−4 weeks), the people\'s and clubs\' records, the player initialisation and the subsystem steps with the progress bar (with the club pass: own stadium, reputation, attendances and training); then it runs to the start date')])
