# A criação do jogo (injury.cpp, player_regen.cpp): a forma inicial dos elencos (Injury::init_squad_fitness
# 0x62a920, pela 0x5cccaa na criação: os dias de pré-temporada da nação do clube e a forma de cada jogador; quem não
# é jogador sai das vagas), os registros de lesão (Injury::init_records 0x62cf70, pelo Injury::Injury 0x62a680:
# aloca 31 bytes por pessoa, sorteia a lesão recorrente e as lesões iniciais pela 0x62d140/0x62d250) e as listas de
# clubes por nação dos jovens (PLAYER_REGEN::build_nation_club_lists 0x7acfc0: a lista da nação de onde sai o clube
# do jovem regen). Iguais no cm0102-1080.exe. Comparadores: a forma lado a lado (registros de lesão, clubes, e o
# clube e cargo de quem não é jogador nas vagas); os registros só a original (aloca e lesiona), conferindo o +0x1e;
# as listas pelo plano dos ids de cada nação (a original aloca).
SAVE = False
ROTINAS = [('forma_inicial_dos_elencos', 0x62a920, [0x6a, 0xff, 0x68, 0x90, 0xb0, 0x95, 0x00]),
           ('registros_iniciais_de_lesao', 0x62cf70, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('listas_de_clubes_por_nacao', 0x7acfc0, [0x6a, 0xff, 0x68, 0x6b, 0xf2, 0x95, 0x00])]
DOCS = ('a criação do jogo: a forma inicial dos elencos, os registros de lesão com as lesões iniciais e as listas de clubes por nação dos jovens',
        'game creation: the starting squad fitness, the injury records with the starting injuries and the per-nation club lists for youngsters',
        [('forma_inicial_dos_elencos', 'Injury::init_squad_fitness', '0x62a920', 'injury.cpp', 'thiscall',
          'para cada clube, os dias de pré-temporada (0 dentro da temporada; até 40); a forma de cada jogador do elenco = 10000 − 75 × (dias − forma natural/2 − +0x5b/4, entre 1 e 40); quem não é jogador sai da vaga (e do clube, cargo 0xb)',
          'for each club, the pre-season days (0 in season; up to 40); each squad player\'s fitness = 10000 − 75 × (days − natural fitness/2 − +0x5b/4, between 1 and 40); non-players leave the slot (and the club, job 0xb)'),
         ('registros_iniciais_de_lesao', 'Injury::init_records', '0x62cf70', 'injury.cpp', 'thiscall',
          'aloca o registro de 31 bytes de cada pessoa em forma plena; jogador: 1 em 10 de lesão recorrente do tipo sorteado e 1 em 100 de começar lesionado (0x62d140, 0x62d250); os totais em 0xde1f64',
          'allocates every person\'s 31-byte record fully fit; players: 1 in 10 of a recurring injury of the drawn type and 1 in 100 of starting injured (0x62d140, 0x62d250); the totals in 0xde1f64'),
         ('listas_de_clubes_por_nacao', 'PLAYER_REGEN::build_nation_club_lists', '0x7acfc0', 'player_regen.cpp', 'thiscall',
          'uma lista por nação; cada clube com divisão entra na da nação dele (nação com +0x11c: só com +0x80 até 2), fora as seleções e os clubes de 0x9d00a4/0x9d00a8',
          'one list per nation; each club with a division joins its nation\'s (a nation with +0x11c: only with +0x80 up to 2), except national teams and the clubs at 0x9d00a4/0x9d00a8')])
CAMPOS = [('Injury +0: os registros de 31 bytes por pessoa ([0xae2364]); ponteiro; grava (registros_iniciais_de_lesao), lê (forma_inicial_dos_elencos)',
           'Injury +0: the 31-byte records per person ([0xae2364]); pointer; writes (registros_iniciais_de_lesao), reads (forma_inicial_dos_elencos)'),
          ('registro de lesão +0, +8, +0xa, +0xc, +0xe, +0x12, +0x14, +0x18, +0x1d, +0x1e: data, forma, condição, desgaste, lesão, recorrente, histórico, doença, 50; valor; grava (registros_iniciais_de_lesao; +8 forma_inicial_dos_elencos)',
           'injury record +0, +8, +0xa, +0xc, +0xe, +0x12, +0x14, +0x18, +0x1d, +0x1e: date, fitness, condition, wear, injury, recurring, history, illness, 50; value; writes (registros_iniciais_de_lesao; +8 forma_inicial_dos_elencos)'),
          ('jogador +0x34: forma natural; valor com sinal; lê (forma_inicial_dos_elencos)',
           'player +0x34: natural fitness; signed value; reads (forma_inicial_dos_elencos)'),
          ('pessoa +0x5b, +0x61, +0x39, +0x3d: atributo da pessoa, jogador, clube e cargo; valor e ponteiro; lê (as três), grava +0x39/+0x3d (forma_inicial_dos_elencos)',
           'person +0x5b, +0x61, +0x39, +0x3d: person attribute, player, club and job; value and pointer; reads (all three), writes +0x39/+0x3d (forma_inicial_dos_elencos)'),
          ('clube +0xd7: as 50 vagas do elenco; ponteiro; lê e grava 0 (forma_inicial_dos_elencos)',
           'club +0xd7: the 50 squad slots; pointer; reads and writes 0 (forma_inicial_dos_elencos)'),
          ('clube +0x53, +0x57, +0x80: nação, divisão e reputação; ponteiro e valor; lê (listas_de_clubes_por_nacao)',
           'club +0x53, +0x57, +0x80: nation, division and reputation; pointer and value; reads (listas_de_clubes_por_nacao)'),
          ('nação +0x71, +0x11c: as marcas que põem os clubes na lista; valor; lê (listas_de_clubes_por_nacao)',
           'nation +0x71, +0x11c: the marks that put clubs on the list; value; reads (listas_de_clubes_por_nacao)'),
          ('PLAYER_REGEN +0: as listas de clubes por nação (id da nação); ponteiro; grava (listas_de_clubes_por_nacao)',
           'PLAYER_REGEN +0: the club lists per nation (nation id); pointer; writes (listas_de_clubes_por_nacao)')]
INTERESSA = ['CMRTE', 'GPF2']
