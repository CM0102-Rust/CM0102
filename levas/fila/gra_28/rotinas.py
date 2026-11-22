# Loading a cup's temporary file: Cup::load_from_disk (Cup.cpp, ~5.4 KB, 0x51fc00..0x5210f0). Reader gra (round 4),
# 21/11/2026. The pair of Cup::save_cup_to_disk 0x5210f0 (docs/MAPA_SAVE.md §6: the same order).
# Who calls it: the constructors of the 63 cups (push 1; mov ecx, esi -> thiscall, ret 4; a nonzero return = the
# saved state was loaded) and the vtable slot 6 of Cup and Cup_Stage. In the career only at the start of the new game,
# without the file (the "no file" return); with the file, on loading a save (every cup of the save, stages included).
# Equal in cm0102-1080.exe (the whole routine).
# Conventions checked in the disassembly: thiscall, ret 4 (the flag is tested as a byte: mov al, [esp+0x47c]).
# Inside: sprintf 0x944c9f (cdecl), 0x90bde0 + fopen 0x945b00 (as temp_file_open_for_read), fread 0x945bfe (4 pushes,
# cdecl), operator new 0x944cf1 / delete 0x944c94 / malloc 0x944e46 / free 0x9452ca (cdecl, 1 push),
# Comp_Stats::Comp_Stats 0x49ee70 (push this; mov ecx, new -> thiscall, ret 4), Comp_Stats::~Comp_Stats 0x49f450 (mov
# ecx -> thiscall, ret 0), Comp_Stats::load 0x4ab890 (push file; mov ecx -> thiscall, ret 4), the virtual +0x20 (push
# file -> thiscall, ret 4), Cup_Stage::Cup_Stage 0x5230e0 (3 pushes -> thiscall, ret 0xc; the counts used as 16 bits:
# movsx word), Cup_Stage::load_from_file 0x523380 and League_Stage::load_from_file 0x68b720 (push file -> thiscall,
# ret 4), League_Stage::League_Stage 0x68b4f0 (2 pushes -> thiscall, ret 8; 16 bits); the boxes: show_error_at
# (returns 0), fatal_error_at and fatal_memory_error_box (Cup.cpp, the game stops).
# Defects of the original: with the statistics flag 0 the old Comp_Stats is destroyed and +8 left dangling; on the
# boxes that return 0 the file is left open; the old round/team/tie/book arrays are not freed; an existing stage array
# is reused whatever STAGE_COUNT says; a round without ties stops the game (fread of 0 bytes returns 0 != 1).
# The SEH frame of the original (the operator new + constructor pairs) is not reproduced (no exception is thrown).
# Comparer: side by side over the same cup: the original first, then the copy re-reads the same file with the
# statistics and stage pointers set aside; the state (cup_load_state) must be equal; the copy's allocations are freed
# and the original's pointers put back.
# Proof: the save proof with the holiday save's load (CARGA); gra28_clube_do_time changes the cups played after the
# load (to 2001-08-28: Charity Shield, League Cup, the European qualifiers); the others are caught by the comparer
# (mutacoes.py --carga-comparando).
SAVE = True
CARGA = True
ROTINAS = [('cup_load_from_disk', 0x51fc00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a carga do temporário de uma copa (Cup::load_from_disk)',
        "loading a cup's temporary file (Cup::load_from_disk)",
        [('cup_load_from_disk', 'Cup::load_from_disk', '0x51fc00', 'Cup.cpp', 'thiscall, ret 4',
          'lê CM3_TEMP\\<competição>_<id ou ano>.tmp na ordem da gravação (MAPA_SAVE §6): nome, estatísticas (Comp_Stats novo, ou o antigo destruído com o ponteiro pendurado), dados da classe (virtual +0x20), prêmio, +0x52, +0x51, +0x4b, fechada (+0x4c; antes da versão 0x3e013 um byte e três pulados), estádio, banco, substituições, +0x45, +0xb1, rodadas, times, ano, +0xab, +0xac, +0x42, +0x3a, ligas de cima e de baixo, a tabela de média (+0x18/+0x34), a competição (id → registro), as rodadas (0x68), os times (6 bytes, id → clube), os confrontos de cada rodada (0x35), os estágios (tipo 1 Cup_Stage, tipo 2 League_Stage, cada um construído e lido) e o livro de inscritos (id → clube); sem o arquivo (jogo novo), 0; erro de leitura: a caixa (algumas devolvem 0 com o arquivo aberto, a maioria encerra o jogo)',
          "reads CM3_TEMP\\<competition>_<id or year>.tmp in the order it was saved (MAPA_SAVE §6): name, statistics (a new Comp_Stats, or the old one destroyed with the pointer left dangling), the class data (virtual +0x20), prize fund, +0x52, +0x51, +0x4b, closed (+0x4c; below version 0x3e013 one byte and three skipped), stadium, bench, substitutions, +0x45, +0xb1, rounds, teams, year, +0xab, +0xac, +0x42, +0x3a, the divisions above and below, the average table (+0x18/+0x34), the competition (id → record), the rounds (0x68), the teams (6 bytes, id → club), each round's ties (0x35), the stages (type 1 Cup_Stage, type 2 League_Stage, each built and read) and the entrant book (id → club); without the file (a new game), 0; on a short read the box (some return 0 with the file left open, most stop the game)")])
CAMPOS = [('Cup +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (nome do arquivo), +0xa3, +0xa7, +0xab, +0xac, +0xad, +0xb1; lê do .tmp e grava',
           'Cup +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (file name), +0xa3, +0xa7, +0xab, +0xac, +0xad, +0xb1; reads from the .tmp and writes')]
INTERESSA = ['SGE']
