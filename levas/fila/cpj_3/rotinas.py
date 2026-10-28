# As notícias para os técnicos humanos (news.cpp), irmãs da 0x786520 da 245ª: para todos (News::send_to_all_managers
# 0x786780, ~22 chamadores: contratos, disciplina, lesões, mídia...), para os da divisão (send_to_division_managers
# 0x7862d0; a tabela da liga 0x685f60 já em Rust) e para os da nação, pelo clube ou pela seleção
# (send_to_nation_managers 0x7863f0). Cada uma marca o alcance (+0xde) e entrega (0x785d40, original) a cada técnico
# humano ativo; a primeira recusa encerra. Simulação: as mutações pela carreira (modo Rust) e a confirmação no jogo
# real comparando. Sem SEH, sem sorteio; caixa só com notícia/divisão/nação nulas (a original). Comparadores: só a
# original (a entrega mexe nas caixas dos técnicos); a prova é a carreira.
SAVE = False
ROTINAS = [('noticia_para_todos_os_tecnicos', 0x786780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_para_os_tecnicos_da_divisao', 0x7862d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_para_os_tecnicos_do_pais', 0x7863f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as notícias para os técnicos humanos: para todos, para os da divisão e para os da nação (clube ou seleção)',
        'the news for the human managers: for all, for those of the division and for those of the nation (club or national side)',
        [('noticia_para_todos_os_tecnicos', 'News::send_to_all_managers', '0x786780', 'news.cpp', 'thiscall, ret 4',
          'alcance de todos (+0xde = 4) e a entrega (0x785d40) a cada técnico humano ativo; 1, ou 0 na primeira recusa',
          'all scope (+0xde = 4) and delivery (0x785d40) to every active human manager; 1, or 0 on the first refusal'),
         ('noticia_para_os_tecnicos_da_divisao', 'News::send_to_division_managers', '0x7862d0', 'news.cpp', 'thiscall, ret 8',
          'alcance da divisão (+0xde = 1) e a entrega a cada técnico humano ativo com clube na divisão (+0x57); 1, ou 0 na primeira recusa',
          'division scope (+0xde = 1) and delivery to every active human manager with a club in the division (+0x57); 1, or 0 on the first refusal'),
         ('noticia_para_os_tecnicos_do_pais', 'News::send_to_nation_managers', '0x7863f0', 'news.cpp', 'thiscall, ret 8',
          'alcance da nação (+0xde = 2) e a entrega a cada técnico humano ativo cujo clube ou seleção é da nação (+0x53); devolve 0',
          'nation scope (+0xde = 2) and delivery to every active human manager whose club or national side belongs to the nation (+0x53); returns 0')])
CAMPOS = [('notícia +0xde alcance (1 divisão, 2 nação, 4 todos); pessoa +0x24 seleção, +0x39 clube, +0x5f ativo; clube +0x53 nação, +0x57 divisão; escreve (noticia_para_todos_os_tecnicos, noticia_para_os_tecnicos_da_divisao, noticia_para_os_tecnicos_do_pais)',
           'news +0xde scope (1 division, 2 nation, 4 all); person +0x24 national side, +0x39 club, +0x5f active; club +0x53 nation, +0x57 division; writes (noticia_para_todos_os_tecnicos, noticia_para_os_tecnicos_da_divisao, noticia_para_os_tecnicos_do_pais)')]
INTERESSA = ['SGE']
