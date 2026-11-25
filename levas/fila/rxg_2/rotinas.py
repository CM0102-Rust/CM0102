# The tactics store loaded from the saved game (tactics.cpp; reader rxg, round 6, 25/11/2026).
# Item SAVE + CARGA, after sx20_9 (INTEGRAR_ANTES: TACTICS_SOURCE, TACTICS_ROW_SIZE, TACTIC_STATS_SIZE, private there;
# both items go to tactics.rs). Same bytes in cm0102-1080.exe.
# - 0x8985c0 Tactics::read_from_save thiscall(this, save) ret 4, returns 1/0; called only by Tactics::Tactics (0x893eff,
#   gra_23, `exe::tactics_read_from_save`). Prologue mov eax, fs:[0] (6 bytes; SEH frame for the formation arrays; no
#   relative call). Last ret at 0x8995ca.
# - Calls: zopen 0x9326e0 cdecl(save, "tactics.dat", 0, 0x16, 4); zread 0x932c30; zclose 0x932b00; operator new
#   0x944cf1 + eh vector constructor iterator 0x9463af(p+4, 0x18e3, n, 0x5b27c0, 0x5a53c0) (= new_formation_array);
#   malloc 0x944e46; free 0x9452ca; Formation::set_player_instruction 0x5b5290 thiscall(block, position byte, u64) ret
#   0xc (old blocks: 8 << 32 and 2 << 32); Formation::set_team_instruction 0x5b50d0 thiscall(block, 0x20000) ret 4.
# - Version |[0xdd7110]| (G_GAME_DATA_VERSION) < 0x3e021: 0x188b-byte blocks (old 4-byte position marks at +0x588) and
#   0x89-byte rows (no playmaker slots). The holiday save is current: the old branches do not run in the proof.
# - Every error is the plain "Error" box (tel26_error_box with tactics.cpp, 0xb67a34 = 0); after the open the file is
#   closed and 0 returned. Lines 0x7c8 (no save), 0x7d0 (open), 0x7d7..0x841 (blocks), 0x853..0x894 (rows),
#   0x89d..0x8b6 (statistics, defaults), 0x8c2 (old extra rows).
# Defects of the original (DEFEITOS): the 16 extra rows are read after the ten defaults, where today's save has
# nothing: every load adds 16 empty rows to the row count, which the next save writes (the rows grow by 16 at each
# save and load) [inference: from Tactics::write_to_save, sx20_9]; a full read of them leaves the count; in old saves
# they are converted and then reset whole (the copy skips the dead conversion); the old rows' scratch array leaks when
# the second malloc fails (0x870).
# Comparer: only the original (allocates and reads the stream). Proof: the load of the holiday save with and without.
SAVE = True
CARGA = True
ROTINAS = [('tactics_read_from_save', 0x8985c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
_F = 'tactics.cpp'
DOCS = ('o depósito de táticas carregado do save',
        'the tactics store loaded from the saved game',
        [('tactics_read_from_save', 'Tactics::read_from_save', '0x8985c0', _F, 'thiscall, ret 4',
          'lê o tactics.dat do save: as táticas próprias e os modelos (contagem, um vetor novo de formações e os blocos; nos saves antigos, blocos de 0x188b bytes com as marcas de posição convertidas para 8 bytes e as instruções de jogador refeitas), cada bloco com a instrução de equipe 0x20000 quando as marcas dele estão abaixo de 0x8000; as linhas dos clubes (contagem + 16 alocadas, a contagem lida; as antigas de 0x89 bytes convertidas, sem os dois armadores); a contagem e os registros das estatísticas; os dez valores padrão; as 16 linhas extras (leitura curta: a contagem cresce 16 e elas ficam vazias); e, por fim, toda pessoa guardada nas linhas com número acima do total vira -1. Fecha e devolve 1; uma falha mostra a caixa "Error" da sua linha, fecha e devolve 0. Defeito do original: as 16 linhas extras são lidas onde o save de hoje já acabou, e a contagem cresce 16 a cada gravação e carga',
          "reads tactics.dat from the saved game: the clubs' own tactics and the presets (count, a new formation array and the blocks; in old saves 0x188b-byte blocks with the position marks converted to 8 bytes and the player instructions set again), each block with the team instruction 0x20000 when its flags are below 0x8000; the club rows (count + 16 allocated, the count read; the old 0x89-byte rows converted, without the two playmakers); the statistics count and records; the ten default values; the 16 extra rows (a short read: the count grows by 16 and they are left empty); and last, every staff id kept in the rows at or above the staff count becomes -1. Closes and returns 1; a failure shows its line's \"Error\" box, closes and returns 0. Defect of the original: the 16 extra rows are read where today's save has ended, and the count grows by 16 at each save and load")])
CAMPOS = []
INTERESSA = []
