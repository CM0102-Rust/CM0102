# Os destrutores das ligas norueguesas e o destrutor escalar da fase de liga (nor_first.cpp, nor_prm.cpp,
# league_stage.cpp; leitor cx12, rodada 5, 24/11/2026). Item de TELAS, como o cx12_1: rodam quando o jogo e desmontado
# (carregar outro save com um jogo aberto, ou sair da carreira); o da fase de liga tambem quando uma liga ou copa libera
# as fases (virtual 0 com 1). Caminho no jogo: com uma carreira aberta que tenha a Noruega carregada, carregar outro
# save, jogando comparando.
# - 0x78ff80 NorFirst::~NorFirst: drop_stats_league do cx04_1 (igual ao 0x4110e0, AusNsl), vtable 0x96de20.
# - 0x791730 NorPrm::~NorPrm: drop_plain_league do cx10_1 (igual ao 0x639e00, IrePrm), vtable 0x96ded4.
# - 0x68b4d0 League_Stage::scalar_deleting_destructor: o corpo dos outros (delete_through) sobre League_Stage::~League_Stage
#   0x68b6a0, ja portado (destroy_league_stage, cqv_4), chamado pelo desvio.
# - 0x78ff60, 0x791710, 0x68b4d0: push esi; mov esi, ecx; call rel32 ~X: prologo pelas RELATIVAS (ver INTEGRAR_ANTES.txt).
# Conferidos instrucao a instrucao contra os moldes; iguais no cm0102-1080.exe (inteiras). Comparadores: so a original.
# Depende do cx01_14 (delete_comp_stats), cx03_3 (delete_through), cx04_1 (drop_stats_league) e cx10_1
# (drop_plain_league): integrar depois deles.
SAVE = False
TELAS = True
ROTINAS = [('drop_nor_first', 0x78ff80, [0x6a, 0xff, 0x68, 0x88, 0xe5, 0x95, 0x00]),
           ('delete_nor_first', 0x78ff60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('drop_nor_prm', 0x791730, [0x6a, 0xff, 0x68, 0x38, 0xe6, 0x95, 0x00]),
           ('delete_nor_prm', 0x791710, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_league_stage', 0x68b4d0, [0x56, 0x8b, 0xf1, 0xe8, 0xc8, 0x01, 0x00, 0x00])]
DOCS = ('os destrutores das ligas norueguesas e o destrutor escalar da fase de liga',
        'the destructors of the Norwegian leagues and the league stage scalar deleting destructor',
        [('drop_nor_first', 'NorFirst::~NorFirst', '0x78ff80', 'nor_first.cpp', 'thiscall, ret 0',
          'desmonta a Primeira Divisão norueguesa: libera a tabela, as datas das rodadas, as fases até a atual, o vetor das fases (liberado e zerado) e as estatísticas e roda o destrutor da liga',
          'destroys the Norwegian First Division: frees the table, the round dates, every stage up to the current one, the stage vector (freed and cleared) and the statistics and runs the league destructor'),
         ('delete_nor_first', 'NorFirst::scalar_deleting_destructor', '0x78ff60', 'nor_first.cpp', 'thiscall, ret 4',
          'roda o destrutor da Primeira Divisão norueguesa e, com o bit 0, libera o objeto',
          'runs the Norwegian First Division destructor and, with bit 0, frees the object'),
         ('drop_nor_prm', 'NorPrm::~NorPrm', '0x791730', 'nor_prm.cpp', 'thiscall, ret 0',
          'desmonta a Premier Division norueguesa: libera a tabela, as datas das rodadas e as estatísticas (sem fases) e roda o destrutor da liga',
          'destroys the Norwegian Premier Division: frees the table, the round dates and the statistics (no stages) and runs the league destructor'),
         ('delete_nor_prm', 'NorPrm::scalar_deleting_destructor', '0x791710', 'nor_prm.cpp', 'thiscall, ret 4',
          'roda o destrutor da Premier Division norueguesa e, com o bit 0, libera o objeto',
          'runs the Norwegian Premier Division destructor and, with bit 0, frees the object'),
         ('delete_league_stage', 'League_Stage::scalar_deleting_destructor', '0x68b4d0', 'league_stage.cpp', 'thiscall, ret 4',
          'roda o destrutor da fase de liga e, com o bit 0, libera o objeto',
          'runs the league stage destructor and, with bit 0, frees the object')])
CAMPOS = []
INTERESSA = []
