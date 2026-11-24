# officials_manager.cpp (leitor sx16, rodada 5, 24/11/2026): o gerente dos arbitros: construtor, destrutor e a carga do
# off.dat. Usa game_cd_check do sx16_7 (vai depois dele).
# - 0x79b580 OfficialsManager::OfficialsManager thiscall(gerente, save) ret 4 -> gerente: a verificacao do CD (como a do
#   News::News; no cm0102-1080 o jne 0x79b5c2 vira nop e o test 0x79b604 vira xor); com save, 0x79b850 (pelo desvio):
#   al != 0 devolve; senao 0x79bf60 set_up_officials thiscall(gerente) (ja Rust, pelo exe), malloc(G_OFFICIAL_COUNT x
#   0x11) e as linhas novas (id, zeros, a dureza pela DISCIPLINE do arbitro +0x25: >18 1, >12 2, >5 3, senao 4; +0x10 =
#   0xff so sem save). Chamado pela criacao do jogo (0x829d58) e pela carga (0x82cabb).
# - 0x79b800 ~OfficialsManager thiscall ret: free(+8), free(+0xc), free(+0), zerando. Pela limpeza (0x830450).
# - 0x79b850 load_official_stats thiscall(gerente, save) ret 4 -> al: zopen(save, "off.dat", 0, 0x16, 4); a contagem
#   em +4 (0: zclose e 0); |[0xdd7110]| < 0x3e043: linhas antigas de 0x10 convertidas, senao as de 0x11 lidas direto
#   (malloc sem teste); as nacoes e os blocos de 0x102 (malloc sem teste); 0x945501 calloc(contagem, 1) para +8 (sem
#   memoria: a caixa fatal 0x125 e exit); as marcas; zclose e 1. Falhas: zclose e 0, sem caixa.
# Defeitos do original: com save cujo off.dat nao le, as linhas novas ficam sem a ultima nota (+0x10 com o lixo do
#   malloc); as alocacoes das linhas de 17 bytes e dos blocos nao sao testadas; o buffer das linhas antigas vaza quando a
#   leitura delas falha.
# Comparadores: so a original (CD, alocacao, leitura do save).
SAVE = True
CARGA = True
ROTINAS = [('new_officials_manager', 0x79b580, [0x81, 0xec, 0x74, 0x04, 0x00, 0x00]),
           ('drop_officials_manager', 0x79b800, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x08]),
           ('load_official_stats', 0x79b850, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('o gerente dos árbitros e a carga do off.dat',
        'the officials manager and the loading of off.dat',
        [('new_officials_manager', 'OfficialsManager::OfficialsManager', '0x79b580', 'officials_manager.cpp', 'thiscall, ret 4',
          'pede o CD do jogo; com um save cujo off.dat lê, é só isso; senão monta os árbitros (set_up_officials) e uma linha de estatísticas nova para cada um: o id, tudo zerado, a dureza pela disciplina (acima de 18 rigoroso, de 12 severo, de 5 justo, senão tolerante) e, sem save, a última nota "nenhuma"',
          "asks for the game CD; with a save whose off.dat reads, that is all; otherwise sets up the officials (set_up_officials) and a fresh statistics row for each: the id, everything cleared, the harshness by the discipline (above 18 harsh, above 12 strict, above 5 fair, otherwise lenient) and, without a save, the last grade \"none\""),
         ('drop_officials_manager', 'OfficialsManager::~OfficialsManager', '0x79b800', 'officials_manager.cpp', 'thiscall',
          'libera as marcas de ocupado, os blocos das nações e as linhas de estatísticas, zerando os ponteiros',
          'frees the busy flags, the nation blocks and the statistics rows, zeroing the pointers'),
         ('load_official_stats', 'OfficialsManager::load_official_stats', '0x79b850', 'officials_manager.cpp', 'thiscall, ret 4',
          'lê o off.dat do save: a contagem de árbitros, as linhas de estatísticas (17 bytes; nos saves antigos, de 16 bytes convertidas), os blocos das nações (0x102 bytes) e as marcas de ocupado; qualquer falha fecha o item e devolve 0',
          'reads off.dat from the save: the official count, the statistics rows (17 bytes; in old saves, 16-byte rows converted), the nation blocks (0x102 bytes) and the busy flags; any failure closes the item and answers 0')])
