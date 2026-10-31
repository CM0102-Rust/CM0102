# A nação-chave da competição e a ordem do ranking da FIFA (key_nation.cpp, fifa_rankings.cpp): o índice da nação-chave
# da competição (key_nation_index_of_comp 0x669470, pelo alcance das regras da partida TRANSFER_MANAGER::
# get_fixture_rules_scope 0x8d2fa0, pela liga do dia League::update_for_date 0x6847c0, pelos registros da copa 0x519890
# e pela nação-chave do jogo 0x48dd70 da cpt_3), a ordem das seleções no ranking (FIFA_RANKINGS::collect_ranked_order
# 0x58d610, pela atualização do dia 18 0x58d460 da cpt_1, que grava as posições) e a comparação das pontuações
# (compare_ranking_score_desc 0x58e240, o qsort dela). Chamadas originais pelo endereço: 0x669ae0 (cpt_3), 0x944e46
# (malloc), 0x945333 (qsort), 0x9452ca (free, no comparador). Iguais no cm0102-1080.exe. Comparadores: o índice e a
# comparação pela devolução; a ordem pelas duas (cada uma aloca o vetor; o da cópia é liberado depois da conferência).
SAVE = False
ROTINAS = [('indice_da_nacao_chave', 0x669470, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ordem_do_ranking', 0x58d610, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('ordem_das_pontuacoes', 0x58e240, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('a nação-chave da competição e a ordem do ranking da FIFA: o índice da nação-chave da competição, a ordem das seleções no ranking e a comparação das pontuações',
        "the competition's key nation and the FIFA ranking order: the key nation index of the competition, the order of the national sides in the ranking and the score comparison",
        [('indice_da_nacao_chave', 'key_nation_index_of_comp', '0x669470', 'key_nation.cpp', 'cdecl',
          'a linha da nação-chave da competição: a da nação, 0 na Copa do Mundo, 1 nos torneios de seleções, 2 nos mundiais de clubes, a do continente ou 3',
          "the competition's key nation row: the nation's, 0 for the World Cup, 1 for national side tournaments, 2 for the club world cups, the continent's or 3"),
         ('ordem_do_ranking', 'FIFA_RANKINGS::collect_ranked_order', '0x58d610', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'um vetor novo com as entradas do ranking das nações com seleção, ordenadas pela pontuação, e a conta',
          'a new array with the ranking entries of the nations with a national side, sorted by score, and the count'),
         ('ordem_das_pontuacoes', 'compare_ranking_score_desc', '0x58e240', 'fifa_rankings.cpp', 'cdecl',
          'ordem do qsort do ranking: a maior pontuação primeiro',
          'qsort order of the ranking: the highest score first')])
CAMPOS = [('tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, continente +4; lê (indice_da_nacao_chave)',
           'key nation table 0xb63d60 (35 × 72 bytes): nation +0, continent +4; reads (indice_da_nacao_chave)'),
          ('competição +0, +0x59, +0x5d: id, continente e nação; lê (indice_da_nacao_chave)',
           'competition +0, +0x59, +0x5d: id, continent and nation; reads (indice_da_nacao_chave)'),
          ('nação +0x71 (registros de 0x122 bytes em [0xae23a8]): a seleção; ponteiro; lê (ordem_do_ranking)',
           'nation +0x71 (0x122-byte records at [0xae23a8]): the national side; pointer; reads (ordem_do_ranking)'),
          ('entrada do ranking +0xc: pontuação; f32; lê (ordem_das_pontuacoes)',
           'ranking entry +0xc: score; f32; reads (ordem_das_pontuacoes)'),
          ('[0x9cf6e4] e [0x9cf7bc] Copa do Mundo de Clubes e Mundial de Clubes da FIFA; id; lê (indice_da_nacao_chave)',
           '[0x9cf6e4] and [0x9cf7bc] World Club Cup and FIFA Club World Championship; id; reads (indice_da_nacao_chave)')]
INTERESSA = ['CMRTE']
