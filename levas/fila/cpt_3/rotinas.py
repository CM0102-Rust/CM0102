# A nação-chave do jogo (comp.cpp, key_nation.cpp): a nação-chave ativa de uma competição ou jogo
# (comp_active_key_nation_of_fixture 0x48dd70, a chave das listas de jogos do FIXTURE_MANAGER: add_fixture 0x5aa680,
# find_fixtures 0x5acbe0, find_fixture_clashes 0x5ad780 e outras, já em Rust; também a simulação rápida 0x6b66a0 e o
# peso do ranking da FIFA 0x58cf70), se a competição é a Copa do Mundo ou uma eliminatória
# (key_nation_is_world_cup_comp 0x669a10) e se é um torneio de seleções (key_nation_is_international_comp 0x669ae0),
# as duas pela 0x48dd70. Tabela das 35 nações-chave em 0xb63d60 (72 bytes: nação +0, continente +4, chave ativa +0x14,
# escolha da nação +0x11c). Chamadas originais pelo endereço: 0x5c6710 (amistoso, já em Rust), 0x53b900 (seleção),
# 0x669470 (índice da nação-chave, cpt_4). Iguais no cm0102-1080.exe. comp.cpp não é de nenhum outro agente desta
# rodada. Comparadores: pela devolução, sem os casos de erro (competição nula: só a original).
SAVE = False
ROTINAS = [('nacao_chave_do_jogo', 0x48dd70, [0x8b, 0x0d, 0x74, 0x2c, 0xae, 0x00]),
           ('e_copa_do_mundo', 0x669a10, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('e_torneio_internacional', 0x669ae0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a nação-chave do jogo: a nação-chave ativa de uma competição ou jogo, se a competição é a Copa do Mundo e se é um torneio internacional',
        'the key nation of the match: the active key nation of a competition or fixture, whether the competition is the World Cup and whether it is an international tournament',
        [('nacao_chave_do_jogo', 'comp_active_key_nation_of_fixture', '0x48dd70', 'comp.cpp', 'cdecl',
          'a chave ativa da nação-chave dona do jogo: pela nação da competição, pelas nações dos clubes do amistoso, 0 na Copa do Mundo, 1 nos torneios de seleções',
          'the active key of the key nation that owns the fixture: by the competition nation, by the nations of the friendly clubs, 0 for the World Cup, 1 for national side tournaments'),
         ('e_copa_do_mundo', 'key_nation_is_world_cup_comp', '0x669a10', 'key_nation.cpp', 'cdecl',
          '1 se o id da competição é a Copa do Mundo ou uma das seis eliminatórias',
          '1 when the competition id is the World Cup or one of its six qualifying sections'),
         ('e_torneio_internacional', 'key_nation_is_international_comp', '0x669ae0', 'key_nation.cpp', 'cdecl',
          '1 se o id da competição é um dos dez torneios de seleções fora da Copa do Mundo',
          '1 when the competition id is one of the ten national side tournaments outside the World Cup')])
CAMPOS = [('tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, chave ativa +0x14; lê (nacao_chave_do_jogo)',
           'key nation table 0xb63d60 (35 × 72 bytes): nation +0, active key +0x14; reads (nacao_chave_do_jogo)'),
          ('nação +0x11c: nação escolhida para jogar; valor; lê (nacao_chave_do_jogo)',
           'nation +0x11c: nation chosen to play; value; reads (nacao_chave_do_jogo)'),
          ('competição +0 e +0x5d: id e nação; lê (as três)',
           'competition +0 and +0x5d: id and nation; reads (all three)'),
          ('partida +0x14, +0x1c, +0x20: competição, mandante e visitante; ponteiro; lê (nacao_chave_do_jogo)',
           'fixture +0x14, +0x1c, +0x20: competition, home and away club; pointer; reads (nacao_chave_do_jogo)'),
          ('clube +0x53: nação; ponteiro; lê (nacao_chave_do_jogo)',
           'club +0x53: nation; pointer; reads (nacao_chave_do_jogo)'),
          ('[0x9cf684] competição de reservas; [0x9cf76c..0x9cf7ac], [0x9cf8a0], [0x9cf964] ids das competições de seleções; id; lê',
           '[0x9cf684] reserve competition; [0x9cf76c..0x9cf7ac], [0x9cf8a0], [0x9cf964] national side competition ids; id; reads')]
INTERESSA = []
