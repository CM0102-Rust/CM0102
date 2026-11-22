# Setup.cpp's static dates and the division club count fix (Setup.cpp; reader tel36, the screens round, 21/11/2026).
# Uses tel36_1 (TEL36_CLUB_COMP_SIZE).
# Conventions checked in the disassembly: the static initialisers cdecl(void) (Date::Date 0x54a110 thiscall on the
# static, atexit 0x94627a cdecl with the destructor); the destructors are a bare `ret` (1 byte; the jump of the detour
# lands in the alignment padding, the trampoline copies `ret` and four `nop`); fix_division_club_count cdecl(comp id,
# short expected) (database_is_national_team 0x53b900 cdecl; sprintf 0x944c9f cdecl into 0xde1f64).
# When they run: the four static routines at every program start and exit, before WinMain (the detours are installed
# in DllMain), so also in the automatic career; fix_division_club_count only from the new game setup 0x828110 when
# Greece is loaded (KEY_NATION_GREECE's +0x11c), for the Greek Premier and Second divisions with 14 clubs each: not in
# the ENG career, once in the world career (docs/cobertura_mundo.txt counts 0x828110 once).
# Equal in cm0102-1080.exe.
# Comparers: fix_division_club_count side by side over the club table and the text buffer; the static routines only the
# original (atexit does not run twice).
TELAS = True
ROTINAS = [('setup_static_init_start_date', 0x81b830, [0xb9, 0xc8, 0x6e, 0xdd, 0x00]),
           ('setup_static_destroy_start_date', 0x81b850, [0xc3, 0x90, 0x90, 0x90, 0x90]),
           ('setup_static_init_saved_date', 0x81b860, [0xb9, 0xa0, 0x70, 0xdd, 0x00]),
           ('setup_static_destroy_saved_date', 0x81b880, [0xc3, 0x90, 0x90, 0x90, 0x90]),
           ('fix_division_club_count', 0x839920, [0xa1, 0x5c, 0x23, 0xae, 0x00])]
DOCS = ('as datas estáticas do Setup.cpp e o acerto do número de clubes de uma divisão',
        "Setup.cpp's static dates and the division club count fix",
        [('setup_static_init_start_date', 'setup_static_init_start_date', '0x81b830', 'Setup.cpp', 'cdecl',
          'a data de início (0xdd6ec8) nasce vazia na partida do programa, com o destruidor no atexit',
          'the start date (0xdd6ec8) is built empty at program start, with its destructor registered with atexit'),
         ('setup_static_destroy_start_date', 'setup_static_destroy_start_date', '0x81b850', 'Setup.cpp', 'cdecl',
          'o destruidor da data de início: nada',
          "the start date's destructor: nothing"),
         ('setup_static_init_saved_date', 'setup_static_init_saved_date', '0x81b860', 'Setup.cpp', 'cdecl',
          'a data gravada (0xdd70a0) nasce vazia na partida do programa, com o destruidor no atexit',
          'the saved date (0xdd70a0) is built empty at program start, with its destructor registered with atexit'),
         ('setup_static_destroy_saved_date', 'setup_static_destroy_saved_date', '0x81b880', 'Setup.cpp', 'cdecl',
          'o destruidor da data gravada: nada',
          "the saved date's destructor: nothing"),
         ('fix_division_club_count', 'fix_division_club_count', '0x839920', 'Setup.cpp', 'cdecl',
          'com a Grécia carregada, cada divisão grega fica com o número certo de clubes (14): os que passam vão para a divisão inferior genérica; as vagas que faltam são preenchidas por clubes da nação sem divisão, nas divisões genéricas ou numa divisão de reputação menor; para na primeira seleção da tabela; os textos de erro só são escritos no buffer',
          'with Greece loaded, each Greek division ends with the right number of clubs (14): the extra ones go to the lower placeholder division; missing places are filled with clubs of the nation with no division, in the placeholder divisions or in a division of lower reputation; it stops at the first national team of the table; the error texts are only written to the buffer')])
