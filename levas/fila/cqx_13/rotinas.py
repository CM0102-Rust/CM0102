# Loading the saved game's temporary files: the CONMEBOL and UEFA seedings and the stadium bookings
# (conmebol_seeding.cpp, uefa_seeding.cpp, stadium.cpp, in comp\). Reader cqx, 12/11/2026. The pairs of the
# save_to_temp_file routines of cpx_6 (UEFA and CONMEBOL) and of the stadium's.
# - 0x4c6970 (ConmebolSeeding::load_from_temp_file), 0x905230 (UefaSeeding::load_from_temp_file), 0x85ba40
#   (Stadium::load_from_temp_file): 1 call each in the 8-league career (the constructors at the start of the new game,
#   without the file: only the "no file" return); with the file, on loading a save.
# Equal in cm0102-1080.exe (the three, whole).
# Conventions checked at the original's call sites: the three from their objects' constructors (mov ecx, this; call,
# no push -> thiscall, ret 0). Inside: 0x90bde0 (path in CM3_TEMP: push mode first; push name; push "CM3_TEMP"; add
# esp, 8 -> cdecl), 0x945b00 (fopen: push path, with the mode pushed before; add esp, 8 -> cdecl), 0x945bfe (fread, 4
# pushes; add esp, 0x10 -> cdecl), 0x945a09 (fclose), 0x944e46 (malloc), 0x9452ca (free): 1 push; add esp, 4 -> cdecl;
# the error box (show_error_at).
# Defects of the original: on the boxes of the two seedings (and the stadium's last one, line 0x15c) the file is left
# open; the stadium does not free its old arrays before the new ones.
# Comparers: side by side over the same object (the copy reads the same file again after the original; the stadium's
# copy arrays are freed and the original's put back).
# Proof: the save proof with the holiday save's load (CARGA): the mutation on the stadium venues changes the venues of
# the matches after the load; the seedings' mutations only show in the comparers (load the save in comparison mode,
# mutacoes.py --carga-comparando), since nothing reads the seedings in the days after the load.
SAVE = True
CARGA = True
ROTINAS = [('conmebol_seeding_load_from_temp_file', 0x4c6970, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('uefa_seeding_load_from_temp_file', 0x905230, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('stadium_load_from_temp_file', 0x85ba40, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a carga dos temporários do save: as sementes da CONMEBOL e da UEFA e as reservas dos estádios',
        "loading the saved game's temporary files: the CONMEBOL and UEFA seedings and the stadium bookings",
        [('conmebol_seeding_load_from_temp_file', 'ConmebolSeeding::load_from_temp_file', '0x4c6970', 'conmebol_seeding.cpp', 'thiscall, ret 0',
          'lê CM3_TEMP\\conmebol_seeding.tmp: quantos países (+0x6e), 0xa bytes por país de +0 (o ponteiro da nação +4 refeito pelo id), +0x6f (u16) e, desde a versão de dados 0x3e010, +0x71 (u32; antes, −1); sem o arquivo (jogo novo), 0; erro: a caixa "Error" sem fechar o arquivo',
          'reads CM3_TEMP\\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0 (the nation pointer +4 rebuilt from the id), +0x6f (u16) and, from data version 0x3e010 on, +0x71 (u32; before, −1); without the file (a new game), 0; on error the "Error" box without closing the file'),
         ('uefa_seeding_load_from_temp_file', 'UefaSeeding::load_from_temp_file', '0x905230', 'uefa_seeding.cpp', 'thiscall, ret 0',
          'lê CM3_TEMP\\uefa_seeding.tmp: quantos países (+0x438), 0x12 bytes por país de +0 (o ponteiro da nação +4 refeito pelo id), +0x439 (u16), quantos u32 (+0x43f) numa lista nova em +0x43b e os 50 registros de 0xc de +0x440; sem o arquivo (jogo novo), 0; erro: a caixa "Error" sem fechar o arquivo',
          'reads CM3_TEMP\\uefa_seeding.tmp: the country count (+0x438), 0x12 bytes per country from +0 (the nation pointer +4 rebuilt from the id), +0x439 (u16), as many u32 as +0x43f into a new list at +0x43b and the 50 records of 0xc from +0x440; without the file (a new game), 0; on error the "Error" box without closing the file'),
         ('stadium_load_from_temp_file', 'Stadium::load_from_temp_file', '0x85ba40', 'stadium.cpp', 'thiscall, ret 0',
          'lê CM3_TEMP\\Stadium.tmp: as reservas (capacidade +8, quantas +4, 12 bytes cada, num vetor novo em +0), os locais (capacidade +0x14, quantos +0x10, a marca +0x18; 16 bytes cada num vetor novo em +0xc, o estádio +0xc refeito pelo id, −1 nulo) e, desde a versão de dados 0x3e029, o local guardado [0xa835a4]; sem o arquivo (jogo novo), 0; erro: a caixa "Error"',
          'reads CM3_TEMP\\Stadium.tmp: the bookings (capacity +8, count +4, 12 bytes each, in a new array at +0), the venues (capacity +0x14, count +0x10, the flag +0x18; 16 bytes each in a new array at +0xc, the stadium +0xc rebuilt from its id, −1 null) and, from data version 0x3e029 on, the stashed venue [0xa835a4]; without the file (a new game), 0; on error the "Error" box')])
CAMPOS = [('ConmebolSeeding +0 (+4 o ponteiro da nação refeito), +0x6e, +0x6f, +0x71; UefaSeeding +0 (+4), +0x438, +0x439, +0x43b, +0x43f, +0x440; Stadium +0, +4, +8, +0xc, +0x10, +0x14, +0x18; lê do .tmp e grava',
           'ConmebolSeeding +0 (+4 the nation pointer rebuilt), +0x6e, +0x6f, +0x71; UefaSeeding +0 (+4), +0x438, +0x439, +0x43b, +0x43f, +0x440; Stadium +0, +4, +8, +0xc, +0x10, +0x14, +0x18; reads from the .tmp and writes')]
INTERESSA = ['SGE']
