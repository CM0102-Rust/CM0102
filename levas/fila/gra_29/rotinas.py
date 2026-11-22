# Loading a league's temporary file: League::load_from_temp_file (league.cpp, ~5.8 KB, 0x687b10..0x6891c0). Reader
# gra (round 4), 21/11/2026. The pair of League::save_to_temp_file 0x6891c0 (docs/MAPA_SAVE.md §6: the same order).
# Who calls it: the constructors of the leagues (and of the "cups" that are leagues) with push 1; mov ecx, esi ->
# thiscall, ret 4 (a nonzero return = the saved state was loaded), and the vtable slot 6 of League and League_Stage
# (with 0 the ".bak" name with the year). In the career only at the start of the new game, without the file (the "no
# file" return); with the file, on loading a save (every league of the save, stages included).
# Equal in cm0102-1080.exe (the whole routine).
# Conventions checked in the disassembly: thiscall, ret 4 (the flag is tested as a byte: mov al, [esp+0x480]).
# Inside: sprintf 0x944c9f (cdecl), 0x90bde0 + fopen 0x945b00 (as temp_file_open_for_read), fread 0x945bfe (4 pushes,
# cdecl), operator new 0x944cf1 / malloc 0x944e46 / free 0x9452ca (cdecl, 1 push), Comp_Stats::Comp_Stats 0x49ee70
# (push this; mov ecx, new -> thiscall, ret 4), Comp_Stats::load 0x4ab890 (push file -> thiscall, ret 4), the virtual
# +0x20 (push file -> thiscall, ret 4), Cup_Stage::Cup_Stage 0x5230e0 (3 pushes -> thiscall, ret 0xc; the counts used as
# 16 bits), Cup_Stage::load_from_file 0x523380 and League_Stage::load_from_file 0x68b720 (push file -> thiscall,
# ret 4), League_Stage::League_Stage 0x68b4f0 (2 pushes -> thiscall, ret 8; 16 bits); every box is show_error_at
# (league.cpp 0x9f15f4) and returns 0 (none stops the game here).
# Defects of the original: the head-to-head clubs of every row are converted only when ROW 0 has head-to-head entries
# (cmp [ecx+5] with ecx = the table, 0x688d83); the statistics flag 0 clears +8 without destroying the old object and
# the flag 1 does not destroy it either (leak); on the boxes the file is left open; the old round-date and table arrays
# are not freed; the points-average table's allocation is not checked; an existing stage array is reused whatever
# STAGE_COUNT says.
# The SEH frame of the original (the operator new + constructor pairs) is not reproduced (no exception is thrown).
# Comparer: side by side over the same league: the original first, then the copy re-reads the same file with the
# statistics, stage and entrant-book pointers set aside; the state (league_load_state) must be equal; the copy's
# allocations are freed and the original's pointers put back.
# Proof: the save proof with the holiday save's load (CARGA); the mutations are caught by the comparer (mutacoes.py
# --carga-comparando); gra29_competicao_adiante may also change the impression of the load (another record's
# reputation and nation for every league).
SAVE = True
CARGA = True
ROTINAS = [('league_load_from_temp_file', 0x687b10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a carga do temporário de uma liga (League::load_from_temp_file)',
        "loading a league's temporary file (League::load_from_temp_file)",
        [('league_load_from_temp_file', 'League::load_from_temp_file', '0x687b10', 'league.cpp', 'thiscall, ret 4',
          'lê CM3_TEMP\\<competição>_<id>.tmp (ou <competição>_<ano>.bak com 0) na ordem da gravação (MAPA_SAVE §6): nome, estatísticas (Comp_Stats novo, sem destruir o antigo; com 0, +8 zerado), dados da classe (virtual +0x20), o cabeçalho (capacidade e lugares exigidos, prêmio, completa, a data da última atualização, jogos por clube, +0x52, importância, +0x4b, fechada, estádio, banco, substituições, o sorteio, ativa, parada, turnos, clubes, ano, tipo, +0x42, +0x3a, a competição (id → registro), rodadas, pontos por vitória e empate, categoria, desempates, as quatro faixas, regras, divisões de cima e de baixo, +0xe6), as datas das rodadas (0x41), a tabela de média (+0x18/+0x34), a tabela (0x3b por clube, id → clube) com o confronto direto (n − 1 registros de 0x0e, id → clube só quando a linha 0 os tem) e o histórico de posições (jogos por clube, refeito de −1 para (n − 1) × turnos), os estágios (tipo 1 Cup_Stage, tipo 2 League_Stage) e o livro de inscritos (o antigo liberado; id → clube); sem o arquivo (jogo novo), 0; erro de leitura: a caixa, devolvendo 0 com o arquivo aberto',
          "reads CM3_TEMP\\<competition>_<id>.tmp (or <competition>_<year>.bak with 0) in the order it was saved (MAPA_SAVE §6): name, statistics (a new Comp_Stats, the old one not destroyed; with 0, +8 cleared), the class data (virtual +0x20), the header (required capacity and seats, prize, complete, the last update date, games per club, +0x52, importance, +0x4b, closed, stadium, bench, substitutions, the draw, active, stopped, legs, clubs, year, type, +0x42, +0x3a, the competition (id → record), rounds, points for a win and a draw, category, tie-breaks, the four bands, rules, divisions above and below, +0xe6), the round dates (0x41), the average table (+0x18/+0x34), the table (0x3b per club, id → club) with the head-to-head (n − 1 records of 0x0e, id → club only when row 0 has them) and the position history (games per club, rebuilt from −1 as (n − 1) × legs), the stages (type 1 Cup_Stage, type 2 League_Stage) and the entrant book (the old one freed; id → club); without the file (a new game), 0; on a short read the box, returning 0 with the file left open")])
CAMPOS = [('League +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x43, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (nome do arquivo), +0xa7, +0xa9, +0xab, +0xb1 (linhas +0, +5, +9), +0xb5, +0xb6, +0xb8, +0xba, +0xbe, +0xc2..+0xc5, +0xd1, +0xd9, +0xdb, +0xdd, +0xe1, +0xe2, +0xe4, +0xe6; lê do .tmp e grava',
           'League +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x43, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (file name), +0xa7, +0xa9, +0xab, +0xb1 (rows +0, +5, +9), +0xb5, +0xb6, +0xb8, +0xba, +0xbe, +0xc2..+0xc5, +0xd1, +0xd9, +0xdb, +0xdd, +0xe1, +0xe2, +0xe4, +0xe6; reads from the .tmp and writes')]
INTERESSA = ['SGE']
