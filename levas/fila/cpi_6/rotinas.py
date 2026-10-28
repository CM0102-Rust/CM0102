# Diversos: a notícia para os técnicos humanos de uma nação (News::send_to_nation_club_managers 0x786520, chamada pelas
# regras das nações: Bélgica, França, Alemanha...; a 0x785d40 entrega), a missão do olheiro da busca
# (PLAYER_SEARCH::get_scout_assignment 0x7aefd0, pela 0x7b7490 e telas de busca; 0x53e990 já em Rust; irmã da
# 0x7ae950 da 244ª) e se a copa terminou (Cup::check_complete 0x521e00, chamada pela tabela de métodos; o método +0x10
# de cada fase). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem SEH, sem sorteio;
# caixa só com notícia/nação nulas ou olheiro que não confere (a original). Comparadores: a notícia só pela original (a
# entrega mexe nas caixas dos técnicos); a missão lado a lado pelas seis saídas; a copa pelo valor.
SAVE = False
ROTINAS = [('noticia_para_os_tecnicos_da_nacao', 0x786520, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('missao_do_olheiro', 0x7aefd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('copa_completa', 0x521e00, [0x56, 0x8b, 0xf1, 0x57, 0x8a, 0x86, 0xb1, 0x00, 0x00, 0x00])]
DOCS = ('a notícia para os técnicos humanos de uma nação, a missão do olheiro da busca e se a copa terminou',
        'the news for the human managers of a nation, the search scout assignment and whether the cup is complete',
        [('noticia_para_os_tecnicos_da_nacao', 'News::send_to_nation_club_managers', '0x786520', 'news.cpp', 'thiscall, ret 8',
          'alcance da nação (+0xde = 2) e a entrega (0x785d40) a cada técnico humano ativo com clube da nação; a primeira recusada encerra',
          'nation scope (+0xde = 2) and delivery (0x785d40) to every active human manager with a club of the nation; the first refusal stops'),
         ('missao_do_olheiro', 'PLAYER_SEARCH::get_scout_assignment', '0x7aefd0', 'player_search.cpp', 'thiscall, ret 0x18',
          'com o olheiro conferido e alguma missão marcada (+0x190, +0x18c, +0x194, +0x195, +0x199, +0x19d), copia as seis e devolve 1; senão 0',
          'with the scout checked and some assignment set (+0x190, +0x18c, +0x194, +0x195, +0x199, +0x19d), copies the six and returns 1; else 0'),
         ('copa_completa', 'Cup::check_complete', '0x521e00', 'Cup.cpp', 'thiscall, ret 8',
          'a copa começou (+0xb1) e, pedindo as fases sem fase marcada (+0x44), todas até a atual completas e a atual é a última',
          'the cup started (+0xb1) and, asking for the stages with no stage set (+0x44), all up to the current complete and the current is the last')])
CAMPOS = [('notícia +0xde alcance; pessoa +0x39 clube, +0x5f ativo; PLAYER_SEARCH +0xa tipo, +0x174..+0x184 olheiro, +0x18c nação, +0x190 clube, +0x194 região, +0x195 competição, +0x199 base, +0x19d adversário; copa +0xc fases, +0x2c quantas, +0x30 atual, +0x44, +0xb1 começou; lê (missao_do_olheiro, copa_completa); escreve (noticia_para_os_tecnicos_da_nacao)',
           'news +0xde scope; person +0x39 club, +0x5f active; PLAYER_SEARCH +0xa type, +0x174..+0x184 scout, +0x18c nation, +0x190 club, +0x194 region, +0x195 competition, +0x199 youth, +0x19d opposition; cup +0xc stages, +0x2c count, +0x30 current, +0x44, +0xb1 started; reads (missao_do_olheiro, copa_completa); writes (noticia_para_os_tecnicos_da_nacao)')]
INTERESSA = ['SGE']
