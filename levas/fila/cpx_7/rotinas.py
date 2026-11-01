# O save dos históricos (club_history.cpp, officials_manager.cpp, simulated_stats.cpp): Club_History::save 0x443f50,
# OfficialsManager::save_official_stats 0x79bb80 e SimulatedStats::save 0x84c680, os passos 0xb, 0x21 e 0x22 do
# save_game_data (0x82ecd9, 0x82f3d5, 0x82f427). No .qst da prova (jogo novo): club history.dat 40.324 bytes (2.520
# clubes de 0x10), off.dat 77.390 (1.246 árbitros de 0x11, 213 nações de 0x102 e um byte por árbitro),
# simulated_stats.dat 8.504 (125 registros de 0x44). Database.cpp e index.cpp esgotados (ver cpx_5). Iguais no
# cm0102-1080.exe; sem SEH; sem x87; só o contêiner (em Rust desde a S1) e a caixa de erro (a da 0x443f50 para o jogo,
# `erro_em`). Comparadores: só a original (gravam o contêiner); a prova é a do save.
SAVE = True
ROTINAS = [('gravar_historico_dos_clubes', 0x443f50, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gravar_estatisticas_dos_arbitros', 0x79bb80, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gravar_estatisticas_simuladas', 0x84c680, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o save dos históricos: o histórico dos clubes, as estatísticas dos árbitros e as estatísticas simuladas',
        'saving the histories: the club history, the officials statistics and the simulated statistics',
        [('gravar_historico_dos_clubes', 'Club_History::save', '0x443f50', 'club_history.cpp', 'thiscall',
          'grava no item "club history.dat" a quantidade (+0xc) e os registros de 0x10 bytes de [+8] e devolve 1; sem contêiner a caixa "Error" e 0; erro ao abrir ou gravar: a caixa e o jogo para',
          'writes the count (+0xc) and the 0x10-byte records from [+8] into the "club history.dat" item and returns 1; no container the "Error" box and 0; on open or write error the box and the game stops'),
         ('gravar_estatisticas_dos_arbitros', 'OfficialsManager::save_official_stats', '0x79bb80', 'officials_manager.cpp', 'thiscall',
          'grava no item off.dat [0xae2390] árbitros (0x11 bytes de [+0]), [0xae2350] nações (0x102 bytes de [+0xc]) e um byte por árbitro de [+8]; devolve 1 no byte baixo; sem árbitros ou erro: a caixa "Error", fecha o item e 0',
          'writes [0xae2390] officials (0x11 bytes from [+0]), [0xae2350] nations (0x102 bytes from [+0xc]) and one byte per official from [+8] into the off.dat item; returns 1 in the low byte; with no officials or on error the "Error" box, closes the item and 0'),
         ('gravar_estatisticas_simuladas', 'SimulatedStats::save', '0x84c680', 'simulated_stats.cpp', 'thiscall',
          'grava no item simulated_stats.dat a quantidade (+0) e os registros de 0x44 bytes de [+4]; erro: a caixa "Error" sem fechar o item',
          'writes the count (+0) and the 0x44-byte records from [+4] into the simulated_stats.dat item; on error the "Error" box without closing the item')])
CAMPOS = [('Club_History +8: os registros do histórico de liga e copa dos clubes (0x10 bytes cada); ponteiro; +0xc quantos (u32); lê e grava no save (gravar_historico_dos_clubes)',
           'Club_History +8: the club league and cup history records (0x10 bytes each); pointer; +0xc count (u32); reads and writes into the save (gravar_historico_dos_clubes)'),
          ('OfficialsManager +0: estatísticas por árbitro (0x11 bytes); +0xc por nação (0x102 bytes); +8 um byte por árbitro; ponteiros; [0xae2390] árbitros e [0xae2350] nações; lê e grava no off.dat (gravar_estatisticas_dos_arbitros)',
           'OfficialsManager +0: per-official stats (0x11 bytes); +0xc per nation (0x102 bytes); +8 one byte per official; pointers; [0xae2390] officials and [0xae2350] nations; reads and writes into off.dat (gravar_estatisticas_dos_arbitros)'),
          ('SimulatedStats +0: quantos (u32); +4 os registros de 0x44 bytes; ponteiro; lê e grava no save (gravar_estatisticas_simuladas)',
           'SimulatedStats +0: count (u32); +4 the 0x44-byte records; pointer; reads and writes into the save (gravar_estatisticas_simuladas)')]
INTERESSA = ['SGE', 'CMRTE']
