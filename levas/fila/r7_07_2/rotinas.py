# A mascara de opcoes da busca de jogadores e o teste de um candidato (search_filters.cpp; leitor r7_07, rodada 7,
# 28/11/2026): estavam adiadas no levas/fila/rx10_15/FORA.md e no sobras.md (sx20) so por tamanho; o resto do
# SearchFilters e o sx20_13 (que as chamava pelo endereco). Rotinas inteiras:
# - 0x806630 SearchFilters::set_option_flag (2496 bytes, ate o ret 8 em 0x806fed; sem tabela de saltos: busca binaria
#   sobre o valor de 64 bits);
# - 0x807450 SearchFilters::player_matches_filter (3392 bytes, ate o ret 4 em 0x808183; quadro SEH dos Cash locais).
# TELAS: a busca de jogadores (0x813210 e as telas de busca, sx05b_24, tel21_4, sx17_5) e o reset dos filtros. A
# carreira automatica nao passa (0 chamadas na cobertura do mundo).
# Caminho no jogo para o Lucas, jogando comparando: Buscar jogadores (Search) -> Filtros: mudar cada grupo de opcoes
# (nacionalidade: qualquer / do pais do tecnico / UE / basco / uma nacao / uma regiao; idade; idioma; onde joga;
# valor (faixas, "livre"); listados para venda/emprestimo; pedidos de olheiro; contrato (rolante, terminando em um ano,
# fora do periodo protegido); posicoes; cargo; lesionados; interesse; texto; os minimos de atributo de jogador e de
# comissao) e buscar, com um tecnico humano com clube e, uma vez, com selecao.
# Prologos: 0x806630 sub esp, 0x208 (6 bytes), 0x807450 push -1 / push 0x96118e (7 bytes); sem relativas. Iguais no
# cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convencoes conferidas na desmontagem:
#   0x806630 thiscall(filtro, baixo, alto) ret 8 -> 1 se a mascara mudou, 0 senao.
#   0x807450 thiscall(filtro, candidato) ret 4 -> 1/0 (candidato: +0xc id da pessoa, +0x29/+0x2a duas marcas).
#   Dentro: 0x76d9b0 thiscall([0xb74318], pessoa, tecnico); 0x7c6830 thiscall([0xb74350], pessoa, 0, idioma, 1) -> al;
#   0x4cf5a0 thiscall([0xae1050], pessoa); 0x85fa20 / 0x85fa00 thiscall(contrato); 0x544670 cdecl(nacao);
#   0x43f620 thiscall(&cash, faixa) ret 4; 0x544520 / 0x544590 cdecl(pessoa, 0); 0x7f8d70 thiscall([0xb74448],
#   pessoa, 0, 0); 0x868210 thiscall(contrato, clube, 0, 0); 0x5b7d20 cdecl(ficha) -> ax; 0x62c400
#   thiscall([0xb63d28], pessoa) -> al; 0x846920 thiscall([[0xde1f54]+0x856], pessoa, clube, 1, 0, 1, 0) -> st0;
#   0x844780 thiscall(o mesmo, pessoa, clube, 1, cargo, 0) -> st0; 0x806080/0x805fb0/0x805f10/0x806030/0x8061f0
#   cdecl(x, texto) (o texto livre; sx19b_10/11, pelo endereco); 0x543280 cdecl(tecnico, pessoa, atributo);
#   0x5431e0 / 0x543f30 cdecl(pessoa, atributo) -> al. O original passa o idioma, o cargo e o atributo como dword com
#   os bytes de cima de outros valores; as chamadas leem o byte. O STAFF_CONTRACT local (0x85e410) nao e lido: a copia
#   nao o monta.
# Leitura de memoria: o filtro (0x80 bytes), o candidato, as pessoas, clubes e nacoes; nenhuma local lida antes de
# gravada.
# Comparadores: 0x806630 lado a lado sobre o filtro e a resposta (search_filters_compare do sx20_13); sem tecnico
# humano quando uma nacao ou regiao seria preenchida (a caixa), so a original. 0x807450 a original e a copia (so
# leem) e as respostas; nas caixas (sem candidato, tecnico ou nacao), so a original.
TELAS = True
SAVE = False
ROTINAS = [('search_filters_set_option_flag', 0x806630, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('search_filters_player_matches_filter', 0x807450, [0x6a, 0xff, 0x68, 0x8e, 0x11, 0x96, 0x00])]
DOCS = ('a máscara de opções da busca de jogadores e o teste de um candidato contra todos os filtros',
        'the player search option mask and the test of one candidate against every filter',
        [('search_filters_set_option_flag', 'SearchFilters::set_option_flag', '0x806630', 'search_filters.cpp', 'thiscall, ret 8',
          'liga uma opção (um bit) da máscara de 64 bits e desliga as outras do grupo dela; devolve se a máscara mudou. Nacionalidade (baixo 1 a 0x40): 4, 8, 0x10 e 0x20 ficam lembradas e 0x40 religa a lembrada; 0x10 e 0x20 com a nação ou região vazias pegam as do técnico humano (o país do clube dele, senão o dele; sem técnico, a caixa). Onde joga (baixo 0x100000 a 0x2000000): o mesmo com 0x800000 e 0x1000000, 0x2000000 religa a lembrada. Os outros grupos: as faixas de idade, os pares e grupos da parte alta; alto 3 com baixo 0x80000000 só desliga o grupo do contrato. Valor que não é uma opção: nada',
          'sets one option (one bit) of the 64-bit mask and clears the others of its group; returns whether the mask changed. Nationality (low 1 to 0x40): 4, 8, 0x10 and 0x20 are remembered and 0x40 sets the remembered one again; 0x10 and 0x20 with an empty nation or region take the human manager\'s (his club\'s country, else his own; without a manager, the error box). Based in (low 0x100000 to 0x2000000): the same with 0x800000 and 0x1000000, 0x2000000 sets the remembered one. The other groups: the age bands, the high pairs and groups; high 3 with low 0x80000000 only clears the contract group. A value that is no option: nothing'),
         ('search_filters_player_matches_filter', 'SearchFilters::player_matches_filter', '0x807450', 'search_filters.cpp', 'thiscall, ret 4',
          'se um candidato da busca passa por todos os filtros ativos (com a máscara padrão, todos passam): nacionalidade (do país do técnico, da UE, basco, uma nação ou região, e a elegibilidade para a seleção do técnico), idade, fluência no idioma do país do técnico, onde joga, valor (faixas; "livre": contrato rolante em nação de passe livre), listado para venda ou empréstimo, pedido de olheiro, contrato (rolante, terminando em menos de um ano, fora do período protegido da FIFA), posições, cargo, lesão, interesse em vir para o clube do técnico (reputação até a do clube + 5000 e interesse de pelo menos 3,15), o texto livre (nomes, nação, continente, clube, divisão) e as faixas de atributo de jogador (só as que o técnico vê) e de comissão (quantas precisam bater). Sem candidato, técnico ou nação do técnico, a caixa de erro e 0',
          'whether a search candidate passes every active filter (with the default mask, everyone passes): nationality (the manager\'s country, the EU, Basque, a nation or region, and eligibility for the manager\'s national side), age, fluency in the manager\'s country\'s language, where he plays, value (bands; "free": a rolling contract in a free-transfer nation), listed for transfer or loan, scout request, contract (rolling, ending within a year, out of FIFA\'s protected period), positions, job, injury, interest in joining the manager\'s club (reputation up to the club\'s + 5000 and interest of at least 3.15), the free text (names, nation, continent, club, division) and the player attribute ranges (only those the manager can see) and the staff ones (how many must match). Without a candidate, manager or manager\'s nation, the error box and 0')])
CAMPOS = [('filtro da busca (0x80 bytes) +8/+0xc a opção de nacionalidade lembrada, +0x10/+0x14 a de onde joga; dword; lê e grava (set_option_flag)',
           'search filter (0x80 bytes) +8/+0xc the remembered nationality option, +0x10/+0x14 the based-in one; dword; reads and writes (set_option_flag)'),
          ('candidato da busca +0xc id da pessoa, +0x29 e +0x2a duas marcas (altos 0x1000000 e 0x2000000); lê',
           'search candidate +0xc person id, +0x29 and +0x2a two flags (high 0x1000000 and 0x2000000); reads')]
INTERESSA = []
