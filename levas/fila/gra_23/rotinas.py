# Building the tactics store (tactics.cpp). Reader r5_C, round 4 (the big routines), 21/11/2026.
# - 0x893dc0 Tactics::Tactics (9248 bytes, ~8 KB of them the 45 unrolled file loads): 1 call in the ENG career (the
#   subsystem load of the new game, setup.rs LoadStep 7, with a null save path), at the career's creation.
# In cm0102-1080.exe the routine differs only in the CD check (0x893e25: the jne on the drive type -> nops; 0x893e67:
# test -> xor), which the copy reads from the running exe (as construct_match_stats does): assinaturas.py has to record
# both hashes.
# Conventions checked in the disassembly (0x893dc0..0x8961dd) and at the caller (setup.rs: push path; ecx = this ->
# thiscall, ret 4, returns this):
# - GetLogicalDriveStringsA [0x9670f8] (push buf; push 0xc8) and GetDriveTypeA [0x967100]: stdcall; 0x944c9f (sprintf,
#   cdecl, 13 args); 0x4c0aa0 file_find_first (3 pushes; add esp -> cdecl), 0x4c0c10 file_find_close (cdecl); 0x66f4e0
#   langlib_sprintf and 0x5e8290 (the box, push 1) cdecl;
# - 0x8985c0 (the tactics from the saved game): push path; ecx = this -> thiscall, ret 4;
# - 0x944cf1 (operator new) cdecl; 0x9463af (eh vector constructor iterator: 5 pushes, no add esp -> stdcall) with the
#   Formation constructor 0x5b27c0 and the empty destructor 0x5a53c0 (a bare ret);
# - 0x90bde0 (utils_build_file_path: push name; push "CM3_TACTICS"; add esp, 8 -> cdecl), pushed between the
#   load_from_disk arguments; 0x5b5d10 Formation::load_from_disk (push 1; push 0; push path; ecx = slot -> thiscall,
#   ret 0xc);
# - the error boxes (strtok split 0x944cff, 0x90d130, sprintf, 0x5e8290 with 0, [0xb67a34] = 0): show_error_at with
#   tactics.cpp (0xa8b908);
# - 0x944e46 (malloc) cdecl; the names copied with an inline strcpy (repne scasb; rep movs).
# The file lists were extracted from the disassembly (file, display name, box line), in order.
# Comparer: in a new game, side by side: the original builds the real store, the copy a second one in a scratch
# object; the counts, the loaded formations (the fields load_from_disk writes and the name), the rows, the statistics
# and the constants must be equal; the copy's arrays are freed. With a saved game, only the original.
SAVE = False
ROTINAS = [('construct_tactics', 0x893dc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a criação das táticas',
        'building the tactics store',
        [('construct_tactics', 'Tactics::Tactics', '0x893dc0', 'tactics.cpp', 'thiscall, ret 4',
          'Confere o CD (o primeiro drive de CD com cm0102.exe; sem ele, a caixa "Insert CD" e de novo); zera os 16 espaços de desfazer; com o save, as táticas vêm dele. Num jogo novo: as 28 formações da IA e as 17 pré-definidas vêm dos arquivos de CM3_TACTICS, na ordem (442_wide.pct duas vezes), cada uma no próximo espaço enquanto houver espaço (o arquivo que falha não conta; sem espaço, a caixa de erro da linha dele; faltando formação no fim, a caixa 0x92 ou 0xb4), e as pré-definidas ganham o nome de exibição ("4-4-2", "3-5-2 Defensive"...); uma linha de escalação vazia por clube e mais 32 (0x91 bytes, tudo −1 depois dos 2 primeiros); as estatísticas por tática (45 de 0x28 bytes, zeradas com o índice) e 9 constantes em +0xc..+0x2c',
          'Checks the CD (the first CD drive holding cm0102.exe; without it, the "Insert CD" box and again); zeroes the 16 undo slots; with the save, the tactics come from it. In a new game: the 28 AI formations and the 17 preset ones come from the CM3_TACTICS files, in order (442_wide.pct twice), each into the next slot while there is room (a file that fails does not count; without room, the box of its line; formations missing at the end, the box at 0x92 or 0xb4), and the presets get their display name ("4-4-2", "3-5-2 Defensive"...); an empty selection row per club plus 32 (0x91 bytes, all −1 after the first 2); the per-tactic statistics (45 of 0x28 bytes, zeroed with the index) and 9 constants at +0xc..+0x2c')])
