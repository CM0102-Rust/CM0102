# The award manager (award_manager.cpp; reader rxf, round 6, 25/11/2026): the constructor, the destructor and the load
# from the saved game. Proven in the disassembly (prologue to the last ret; no jump tables):
# - 0x413180 AWARD_MANAGER::AWARD_MANAGER(save): thiscall, ret 4, returns this (SEH frame only for the operator new;
#   no destructor to reproduce). Called by create_game (0x8290a9, CreateStep::Object with 0) and load_game_data
#   (0x82c0e3, LoadStep::Objeto with the save). The CD search is the shared one (find_game_cd, option (a)); cm0102-1080
#   patches it here: 0x4131db jne -> nop nop and 0x41321d test -> xor (the only bytes that differ from the 1080 exe in
#   the three routines): the copy reads those bytes, and the signature differs between the exes.
# - 0x413410 AWARD_MANAGER::~AWARD_MANAGER(): thiscall, ret 0 (free_game_data 0x8300b4, through its table).
# - 0x413fe0 AWARD_MANAGER::load(save): thiscall, ret 4, returns 1 or 0 (only the constructor 0x41334b calls it).
# - Calls: FILE_LLIST::FILE_LLIST 0x58e280 thiscall(path, size) ret 8 (the size pushed before utils_build_file_path,
#   cdecl with 2); create_nation_awards 0x4135b0 thiscall(file) ret 4 (rxf_2); YEAR_RATINGS::YEAR_RATINGS 0x931360
#   thiscall(day word, file) ret 8 (it reads only the low word: the original pushes eax with ax = the date's first word);
#   Date::Date 0x549f70 thiscall(20, 5, 2001, -1) ret 0x10; database_free_staff_comp_history_table 0x527f20 cdecl;
#   NationAwards::~NationAwards 0x7682b0 thiscall (called directly); 0x58e330 thiscall on the file lists and on the
#   YEAR_RATINGS; zopen/zclose; the "Error" box (show_error_box).
# Runs: the constructor at every career's creation (create_game) and at every load (with load); the destructor at
# free_game_data (in the creation and before a load). No random draw.
# Defect of the original kept: the destructor calls the base NationAwards::~NationAwards directly on each award object
# (not the virtual destructor): whatever a derived award class would free on its own is not freed (the classes ported so
# far free nothing more). Not new: the history file lists' allocations are not checked (0 stays in the slot).
SAVE = True
CARGA = True  # the save proof also loads the FeriasTesteSave with and without the routines (prova_save.py)
ROTINAS = [('award_manager_construct', 0x413180, [0x6a, 0xff, 0x68, 0x85, 0x3b, 0x95, 0x00]),
           ('award_manager_destruct', 0x413410, [0x53, 0x8b, 0xd9, 0x56, 0x8b, 0x43, 0x08]),
           ('award_manager_load', 0x413fe0, [0x6a, 0xff, 0x68, 0x49, 0x3d, 0x95, 0x00])]
DOCS = ('o gerente de prêmios: montar, destruir e carregar do save',
        'the award manager: building, destroying and loading from the save',
        [('award_manager_construct', 'AWARD_MANAGER::AWARD_MANAGER', '0x413180', 'award_manager.cpp', 'thiscall, ret 4',
          'procura o CD (a caixa "Insert CD" até achar; o exe sem CD remenda a busca e a cópia lê os bytes), cria as listas em arquivo do histórico dos prêmios do ano (CM3_TEMP\\past_year_awards.tmp, registros de 0x4a bytes) e dos prêmios de equipe (past_team_awards.tmp, 0x205 bytes) e zera as listas de indicados; com save, carrega os prêmios; senão cria os objetos de prêmio de cada nação jogada, solta a tabela de histórico de competições do pessoal e cria as duas tabelas de notas do ano: a mundial, que vira em 20 de maio, e a das nações escolhidas, que vira no último dia do ano',
          'searches the CD (the "Insert CD" box until found; a no-CD exe patches the search and the copy reads the bytes), creates the file lists of the past year awards (CM3_TEMP\\past_year_awards.tmp, 0x4a-byte records) and of the team awards (past_team_awards.tmp, 0x205 bytes) and clears the shortlists; with a save, loads the awards; otherwise creates the award objects of each played nation, frees the staff competition history table and creates the two year ratings tables: the world one, turning over on 20 May, and the selected nations\' one, turning over on the last day of the year'),
         ('award_manager_destruct', 'AWARD_MANAGER::~AWARD_MANAGER', '0x413410', 'award_manager.cpp', 'thiscall',
          'destrói os 30 objetos de prêmio (pelo destrutor da base, não o virtual) e a tabela, as duas listas em arquivo, as duas tabelas de notas do ano e as listas de indicados',
          'destroys the 30 award objects (through the base destructor, not the virtual one) and their table, both file lists, both year ratings tables and the shortlists'),
         ('award_manager_load', 'AWARD_MANAGER::load', '0x413fe0', 'award_manager.cpp', 'thiscall, ret 4',
          'lê do item "awards.dat" do save os objetos de prêmio e as duas tabelas de notas do ano; sem save ou sem o item, a caixa de erro e 0 (o construtor então cria tudo de novo)',
          'reads from the save\'s "awards.dat" item the award objects and both year ratings tables; without a save or the item, the error box and 0 (the constructor then creates everything anew)')])
CAMPOS = [('AWARD_MANAGER +0 lista em arquivo dos prêmios do ano passados (FILE_LLIST, registros de 0x4a), +4 a dos prêmios de equipe (0x205), +8 tabela de 30 objetos de prêmio, +0xc notas do ano das nações escolhidas (YEAR_RATINGS, dia de virada 364), +0x10 notas do ano mundiais (virada em 20/mai), +0x14/+0x18/+0x1a listas de indicados; +0x1c..+0x5b e +0xdc zerados pelo construtor [inferência: estado do voto]; grava (award_manager_construct, award_manager_load) e libera (award_manager_destruct)',
           'AWARD_MANAGER +0 past year awards file list (FILE_LLIST, 0x4a-byte records), +4 the team awards one (0x205), +8 the 30-slot award object table, +0xc the selected nations\' year ratings (YEAR_RATINGS, turnover day 364), +0x10 the world year ratings (turnover on 20 May), +0x14/+0x18/+0x1a the shortlists; +0x1c..+0x5b and +0xdc cleared by the constructor [inference: vote state]; writes (award_manager_construct, award_manager_load) and frees (award_manager_destruct)')]
