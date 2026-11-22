# Building the player statistics (player_stats.cpp; reader r8_scout, round 4, the big routines, 21/11/2026).
# - 0x7bfd50 PlayerStats::PlayerStats (4016 bytes, 0x7bfd50..0x7c0cf3, ret 4, SEH frame): 1 call in the ENG career
#   (the subsystem load of the new game, setup.rs LoadStep 0x11, with a null save path), at the career's creation.
#   Split into named pieces: player_stats_check_cd, player_stats_init_storage, player_stats_initial_languages,
#   player_stats_real_histories (player_stats_history_record, player_stats_languages_abroad, player_stats_flush_history,
#   player_stats_goalkeeper_goals_box).
# In cm0102-1080.exe the routine differs only in the CD check (0x7bfdc4: the jne on the drive type -> nops; 0x7bfe06:
# test -> xor), which the copy reads from the running exe (as construct_match_stats and construct_tactics do):
# assinaturas.py has to record both hashes.
# Conventions checked in the disassembly and at the caller (setup.rs: push path; ecx = this -> thiscall, ret 4,
# returns this):
# - GetLogicalDriveStringsA [0x9670f8] / GetDriveTypeA [0x967100] stdcall; 0x944c9f sprintf, 0x4c0aa0, 0x4c0c10,
#   0x66f4e0, 0x5e8290 cdecl (the CD loop, as in construct_tactics);
# - 0x944cf1 operator new cdecl; 0x90bde0 utils_build_file_path cdecl (push name; push "CM3_TEMP"; add esp, 8 leaves the
#   pushed 0x2b on the stack); 0x58e280 FILE_LLIST constructor thiscall(file, path, 0x2b) ret 8 (the 0x2b is the push
#   before the path build);
# - 0x7c48b0 (statistics from the saved game) thiscall(this, path) ret 4;
# - 0x944e46 malloc cdecl; 0x7c6930 stdcall(person, language) ret 8 (ecx = this, unused), returns ax; 0x7c6a30
#   add_language thiscall(person, language, fluency, sort) ret 0x10; 0x946464 bsearch / 0x945333 qsort cdecl;
#   0x945640 _ftol;
# - 0x945a60 strstr cdecl on the command line 0xae2c98 ("norealplhist" 0xa78300, "checkgks" 0xa782f4);
# - 0x802f80 thiscall(0xb74450, label, done, total, -1) ret 0x10 (the progress bar);
# - 0x448cb0 club_records_update_career_records thiscall([0xad9dac], person, club, games, goals) ret 0x10: the sums go
#   with stack garbage in the high words (movzx cx / dx), the routine only reads the words;
# - 0x5b7d20 formation_get_position_mask cdecl; 0x53dba0 cdecl(person, buffer, 0x7d0, 1);
# - 0x54cdb0 thiscall(0xdd6ec8, day dword, leap dword) ret 8 (asked twice when it wins, as in the original);
# - 0x68d020 LLIST constructor, 0x68d040 destructor (thiscall), 0x944c94 operator delete; 0x68d0d0 add_tail
#   thiscall(list, node, 0x2b) ret 8; 0x58e9b0 FILE_LLIST write thiscall(file, &start, &last, list) ret 0xc;
# - 0x527ee0 cdecl, no arguments: frees the staff_history table and zeroes its count.
# DEFECTS of the original kept (docs/DEFEITOS_DO_ORIGINAL.md candidates): the language counters of the history pass
# (a stack buffer) are only zeroed when a person with a player sheet begins: a person without a sheet gets the
# languages of the counters left by the previous player (and the very first one would read the drive list of the CD
# check and stack garbage: the copy uses the drive list and zeros); a record without a person forgets the previous one
# but keeps the two node pointers into the freed list, which the next record writes into.
# Comparer: plan first in a scratch object (the drive list of the moment and a copy of the history table; no CD loop,
# no file, no lists, no career records, no boxes), then the original; compared: capacities, counts, the rating
# doubles, every per-player entry but the two history positions, and the whole language table. With a save, only the
# original.
SAVE = False
ROTINAS = [('construct_player_stats', 0x7bfd50, [0x6a, 0xff, 0x68, 0x7c, 0xf6, 0x95, 0x00])]
DOCS = ('a criação das estatísticas dos jogadores',
        'building the player statistics',
        [('construct_player_stats', 'PlayerStats::PlayerStats', '0x7bfd50', 'player_stats.cpp', 'thiscall, ret 4',
          'Confere o CD; cria o arquivo do histórico (CM3_TEMP\\player stats history.tmp, nós de 0x2b bytes); com o save, as estatísticas vêm dele. Num jogo novo: 100 registros de categoria, uma entrada de 0x27 bytes por jogador (os oito dwords −1, jogos, gols e notas 0), as médias de nota 680/100 e a tabela de idiomas vazia. Os idiomas: cada pessoa ganha os idiomas 1..64 pelo peso das suas duas nações (peso acima do limiar (20 − adaptabilidade) × 10: (peso − limiar)/50, no máximo 10); o não jogador num clube de outra nação ganha o idioma dela: anos no clube (no máximo 10) × adaptabilidade × 0,05. Com o histórico real (sem "norealplhist"): por clube e pessoa, os jogos e gols de cada passagem vão aos recordes de carreira do clube (gols sem jogos viram 0); por pessoa e ano, duas temporadas do mesmo ano em clubes trocados de ordem são destrocadas; cada registro de 1900 até o ano anterior ao início vira dois nós da lista de carreira do jogador (categorias 6 e 2), gravada no arquivo; o clube atual soma jogos e gols na entrada; um clube de outra nação e outro idioma soma dias àquele idioma (2 × (3 × (jogos + 30) − ano atual + ano); no clube atual, ao menos os dias desde a chegada × 0,7), que viram fluência pelo limiar quando a pessoa termina. No fim a tabela do histórico é liberada',
          'Checks the CD; creates the history file (CM3_TEMP\\player stats history.tmp, 0x2b-byte nodes); with the save, the statistics come from it. In a new game: 100 category records, one 0x27-byte entry per player (the eight dwords −1, games, goals and ratings 0), the rating averages 680/100 and the empty language table. The languages: every person gets languages 1..64 by the weight of his two nations (a weight above the threshold (20 − adaptability) × 10: (weight − threshold)/50, at most 10); a non-player at a club of another nation gets its language: years at the club (at most 10) × adaptability × 0.05. With the real history (without "norealplhist"): by club and person, the games and goals of each spell go to the club\'s career records (goals without games become 0); by person and year, two seasons of the same year at clubs out of order are put back; each record from 1900 to the year before the start becomes two nodes of the player\'s career list (categories 6 and 2), written to the file; the current club adds games and goals into the entry; a club of another nation and language adds days to that language (2 × (3 × (games + 30) − this year + the year); at the current club, at least the days since joining × 0.7), which become fluency by the threshold when the person ends. In the end the history table is freed')])
