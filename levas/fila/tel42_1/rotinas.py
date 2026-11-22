# The club history store (club_history.cpp; reader tel42, screens round 2, 22/11/2026): the constructor, the destructor
# and the load from the saved game. Club_History::save (0x443f50) is already in Rust (cpx_7, club_history.rs).
# Proven in the disassembly (prologue to the last ret; no jump tables):
# - 0x442ab0 Club_History::Club_History(save): thiscall, ret 4, returns this (SEH frame only for the two operator new;
#   no destructor to reproduce). Called by create_game (0x828f36, CreateStep::Object with 0) and load_game_data
#   (0x82bf73, LoadStep::Objeto with the save). The CD search (0x442ad7..0x442bb6) is the same as create_game's;
#   cm0102-1080 (and the portable exe) patch it here too: 0x442b0c jne -> nop nop and 0x442b4f test -> xor; the copy
#   reads those bytes (club_history_find_cd). These are the only bytes that differ from the 1080 exe, so the routine's
#   signature (assinaturas.py) differs between the exes: check that the 1080 keeps its patched version registered.
# - 0x442e20 Club_History::~Club_History(): thiscall, ret 0 (called by free_game_data 0x830014, through its table).
# - 0x443cd0 Club_History::load(save): thiscall, ret 4, returns 1 or 0 (called only by the constructor 0x442d34).
# - Calls: FILE_LLIST::FILE_LLIST 0x58e280 thiscall(path, size) ret 8; FILE_LLIST::~FILE_LLIST 0x58e330 thiscall;
#   utils_build_file_path 0x90bde0 cdecl; zopen 0x9326e0 cdecl(save, name, 0, 0x16, 4); zread 0x932c30 cdecl(data,
#   size, n, z); zclose 0x932b00 cdecl(&z); malloc 0x944e46, free 0x9452ca, operator new/delete; the fatal boxes
#   (box, 0x5cc7d0, 0x5a8f60, exit(-1)) = fatal_error_at; the plain box = show_error_at.
# Runs: the constructor once at the career's creation (create_game) and at every load (with load); the destructor at
# free_game_data (before a load and at the end). No random draw.
# Defect of the original kept: load's missing item (line 0x2f6) only shows the box and returns 0; the constructor then
# builds empty records (-1), as on a new game, with no warning that the save's club history was lost.
SAVE = True
CARGA = True  # the save proof also loads the FeriasTesteSave with and without the routines (prova_save.py)
ROTINAS = [('club_history_constructor', 0x442ab0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff]),
           ('club_history_destructor', 0x442e20, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x3e]),
           ('club_history_load', 0x443cd0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o histórico dos clubes: montar, destruir e carregar do save',
        'the club history: building, destroying and loading from the save',
        [('club_history_constructor', 'Club_History::Club_History', '0x442ab0', 'club_history.cpp', 'thiscall, ret 4',
          'procura o CD (a caixa "Insert CD" até achar; o exe sem CD remenda a busca e a cópia lê os bytes), cria as listas em arquivo do histórico de liga (CM3_TEMP\\club league history.tmp, registros de 21 bytes) e de copa (club cup history.tmp, 11 bytes); com save, carrega os registros; senão um registro de 16 bytes por clube com os quatro deslocamentos em -1',
          'searches the CD (the "Insert CD" box until found; a no-CD exe patches the search and the copy reads the bytes), creates the league history file list (CM3_TEMP\\club league history.tmp, 21-byte records) and the cup one (club cup history.tmp, 11 bytes); with a save, loads the records; otherwise one 16-byte record per club with the four offsets at -1'),
         ('club_history_destructor', 'Club_History::~Club_History', '0x442e20', 'club_history.cpp', 'thiscall',
          'destrói as duas listas em arquivo e libera os registros dos clubes, zerando ponteiros e contagem',
          'destroys both file lists and frees the club records, zeroing the pointers and the count'),
         ('club_history_load', 'Club_History::load', '0x443cd0', 'club_history.cpp', 'thiscall, ret 4',
          'lê do item "club history.dat" do save a contagem e os registros de 16 bytes de cada clube; leitura curta ou falta de memória param o jogo',
          'reads from the save\'s "club history.dat" item the count and each club\'s 16-byte records; a short read or no memory stops the game')])
CAMPOS = [('Club_History +0 lista em arquivo do histórico de liga (FILE_LLIST, registros de 0x15), +4 a de copa (0xb), +8 registros por clube (0x10: +0/+4 primeiro e último deslocamento das temporadas de liga, +8/+0xc das campanhas de copa, -1 nenhum), +0xc a contagem; ponteiro e valor; grava (club_history_constructor, club_history_load) e libera (club_history_destructor)',
           'Club_History +0 league history file list (FILE_LLIST, 0x15-byte records), +4 the cup one (0xb), +8 per-club records (0x10: +0/+4 first and last offsets of the league seasons, +8/+0xc of the cup runs, -1 none), +0xc the count; pointer and value; writes (club_history_constructor, club_history_load) and frees (club_history_destructor)')]
INTERESSA = ['SGE', 'CMRTE']
