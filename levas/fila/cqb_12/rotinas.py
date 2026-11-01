# As nacoes-chave das competicoes (comp.cpp): tres consultas puras da tabela das 35 nacoes-chave em 0xb63d60 (72 bytes:
# +0 a nacao, +0x14 o indice ativo), chamadas por rotinas ja em Rust (o filtro do candidatos.py as escondia):
# - 0x48dee0 (comp_active_key_nation_of_club): pela FIXTURE_MANAGER::find_fixtures 0x5acbe0 (288865 chamadas na
#   carreira de 17/10; o tipo 6 da busca dos jogos de um clube) e pela find_fixture_clashes 0x5ad780 (8129; os dois
#   lados de um jogo de selecoes). Os tipos decidem quais jogos conflitam: datas remarcadas e a busca de datas livres.
# - 0x48df40 (comp_active_key_nation_of_stadium): pela is_stadium_in_use 0x5ae870 (2203) e pela 0x5ad780 (o tipo do
#   estadio do jogo).
# - 0x48dcd0 (comp_key_nation_entry_from_active_index): pela montagem do dia 0x6b3c10 (match_day.cpp, ja em Rust), a
#   entrada da nacao-chave de cada grupo de partidas do dia.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x48dee0 cdecl (push clube; add esp, 8 em 0x5acc98 cobre ela e a 0x5af5b0; add esp, 8 em 0x5ad97c
#   cobre as duas dela), 1 arg, al; 0x48df40 cdecl (push estadio; add esp, 4 em 0x5ae8c9 e 0x5ad8fc), 1 arg, al;
#   0x48dcd0 cdecl (push ecx; add esp, 4 em 0x6b3ded), 1 arg (o char baixo), eax (movsx do indice).
#   0x53b900 e_selecao: cdecl (add esp, 4 em 0x48df1b), 1 arg. A caixa da 0x48dcd0 pelo aviso_em (0x944cff, 0x90d130,
#   0x944c9f, 0x5e8290: cdecl, como em 0x48dd08..0x48dd48).
# Comparadores: as tres comparam o valor devolvido (puras); a 0x48dcd0 sem entrada (a caixa) so roda a original. Sem
# SEH. Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('nacao_chave_do_clube', 0x48dee0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('nacao_chave_do_estadio', 0x48df40, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('entrada_da_nacao_chave_ativa', 0x48dcd0, [0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as nações-chave das competições: a do clube, a do estádio e a entrada da tabela pelo índice ativo',
        'the competitions key nations: the club one, the stadium one and the table entry by active index',
        [('nacao_chave_do_clube', 'comp_active_key_nation_of_club', '0x48dee0', 'comp.cpp', 'cdecl',
          'o índice ativo da nação do clube entre as nações-chave ativas; fora delas, 0 para uma seleção e 3 para um clube (o tipo que a busca dos jogos e o conflito de datas usam)',
          "the active index of the club's nation among the active key nations; otherwise 0 for a national team and 3 for a club (the type used by the fixture search and the date clash check)"),
         ('nacao_chave_do_estadio', 'comp_active_key_nation_of_stadium', '0x48df40', 'comp.cpp', 'cdecl',
          'o índice ativo da nação da cidade do estádio entre as nações-chave ativas, ou 3',
          "the active index of the stadium city's nation among the active key nations, or 3"),
         ('entrada_da_nacao_chave_ativa', 'comp_key_nation_entry_from_active_index', '0x48dcd0', 'comp.cpp', 'cdecl',
          'a primeira entrada da tabela das nações-chave com aquele índice ativo; nenhuma, a caixa de erro e 3',
          'the first key nation table entry with that active index; none, the error box and 3')])
CAMPOS = [('[0xb63d60] tabela das 35 nações-chave (72 bytes: +0 nação, +0x14 índice ativo); ponteiro/valor; lê (as três)',
           '[0xb63d60] table of the 35 key nations (72 bytes: +0 nation, +0x14 active index); pointer/value; reads (all three)'),
          ('nação +0x11c: nação ativa (com ligas carregadas); valor; lê (nacao_chave_do_clube, nacao_chave_do_estadio)',
           'nation +0x11c: active nation (with loaded leagues); value; reads (nacao_chave_do_clube, nacao_chave_do_estadio)'),
          ('clube +0x53 nação; ponteiro; lê (nacao_chave_do_clube)', 'club +0x53 nation; pointer; reads (nacao_chave_do_clube)'),
          ('estádio +0x38 cidade, cidade +0x1f nação; ponteiros; lê (nacao_chave_do_estadio)',
           'stadium +0x38 city, city +0x1f nation; pointers; reads (nacao_chave_do_estadio)')]
INTERESSA = ['CMRTE']
