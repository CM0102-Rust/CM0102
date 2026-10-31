# As linhas da nação-chave (key_nation.cpp): a linha de uma nação na tabela das 35 nações-chave
# (key_nation_index_of_nation 0x669340, pela moral do jogo contract_manager_lower_match_morale 0x4cb250, pelas
# aposentadorias player_regen_check_retirement 0x7a7310 e pelo amistoso livre friendly_is_club_free_on_date 0x5c6b70,
# já em Rust; também os prêmios do dia das nações e as vagas da UEFA), a linha do clube (key_nation_index_of_club
# 0x669360, pelas aposentadorias, pelo adiamento pelo tempo Weather::check_postponement 0x923630, pelos dias até a
# temporada finance_get_days_to_season_start 0x59b4c0 e pelo contrato desejado 0x85fc50) e o ano da temporada do clube
# (key_nation_get_club_season_year 0x669950, pelos registros do clube e pela história do jogador). Tabela em 0xb63d60
# (72 bytes: nação +0, continente +4, início da temporada +0x15). Chamadas originais pelo endereço: 0x53b900 (seleção),
# 0x669360 (cpu_1, pela 0x669950), a caixa "Error" (aviso_em). Iguais no cm0102-1080.exe. Comparadores: pela devolução;
# o caso da caixa (nação escolhida fora da tabela) só a original.
SAVE = False
ROTINAS = [('linha_da_nacao', 0x669340, [0x8b, 0x54, 0x24, 0x04, 0x32, 0xc0]),
           ('linha_da_nacao_do_clube', 0x669360, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ano_da_temporada_do_clube', 0x669950, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('as linhas da nação-chave: a linha de uma nação, a linha do clube e o ano da temporada do clube',
        'the key nation rows: the row of a nation, the row of a club and the club\'s season year',
        [('linha_da_nacao', 'key_nation_index_of_nation', '0x669340', 'key_nation.cpp', 'cdecl',
          'a primeira linha da tabela das 35 nações-chave com a nação dada, ou 4',
          'the first row of the 35 key nation table holding the given nation, or 4'),
         ('linha_da_nacao_do_clube', 'key_nation_index_of_club', '0x669360', 'key_nation.cpp', 'cdecl',
          'a linha da nação escolhida do clube; seleção, 1; nação não escolhida, a linha do continente dela, ou 3',
          'the row of the club\'s chosen nation; national side, 1; nation not chosen, the row of its continent, or 3'),
         ('ano_da_temporada_do_clube', 'key_nation_get_club_season_year', '0x669950', 'key_nation.cpp', 'cdecl',
          'o ano da temporada do clube: o ano anterior antes do dia de início da temporada da linha dele',
          'the club\'s season year: the previous year before the season start day of its row')])
CAMPOS = [('tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, continente +4, dia de início +0x15, dword +0x17; lê (as três)',
           'key nation table 0xb63d60 (35 × 72 bytes): nation +0, continent +4, start day +0x15, dword +0x17; reads (all three)'),
          ('clube +0x53: nação; ponteiro; lê (linha_da_nacao_do_clube)',
           'club +0x53: nation; pointer; reads (linha_da_nacao_do_clube)'),
          ('nação +0x11c: nação escolhida para jogar; valor; +0x71: continente; ponteiro; lê (linha_da_nacao_do_clube)',
           'nation +0x11c: nation chosen to play; value; +0x71: continent; pointer; reads (linha_da_nacao_do_clube)'),
          ('[0xae2c90] dia e [0xae2c92] ano de hoje; valor; lê (ano_da_temporada_do_clube)',
           '[0xae2c90] day and [0xae2c92] year of today; value; reads (ano_da_temporada_do_clube)')]
INTERESSA = ['CMRTE']
