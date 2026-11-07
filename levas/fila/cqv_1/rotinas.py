# As posições previstas dos estaduais: Mineiro, Norte, Nordeste e Paranaense (o slot set_predicted_positions de bra_reg_minas_gerais.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp, bra_reg_parana.cpp; agente C da frente
# das competições, 06/11/2026). Rodam duas vezes cada na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt):
# na criação (do construtor, cqt_1..4) e na virada da temporada. O mesmo molde nos doze estaduais (conferido instrução a
# instrução: só mudam o fonte, as linhas das caixas e os lugares fixos do mata-mata: 1, 2, 3, 3 nos de semifinal; 1, 2,
# 3, 3, 5, 5, 5, 5 nos de quartas de final; 1, 2 nos de final); os itens cqu_4, cqv_1 e cqv_2 cobrem os doze.
# - 0x434f80 BraRegMinasGerais::set_predicted_positions: lugares fixos 1, 2.
# - 0x436280 BraRegNorth::set_predicted_positions: lugares fixos 1, 2, 3, 3.
# - 0x437290 BraRegNortheast::set_predicted_positions: lugares fixos 1, 2, 3, 3.
# - 0x438590 BraRegParana::set_predicted_positions: lugares fixos 1, 2.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - as rotinas, do construtor (mov ecx, esi; call direto) e da virada (call [vtable + 0x5c]) -> thiscall, ret 0;
# - 0x944e46 (malloc): push 4 × +0x3e; call; add esp, 4 -> cdecl; 0x9452ca (free): push vetor; call; add esp, 4 -> cdecl;
# - 0x4c08f0 (comp_util_sort_clubs_by_strength): push n; push vetor; call; add esp, 8 -> cdecl;
# - 0x4a2540 (Comp_Stats::set_club_final_position): push lugar; push clube; mov ecx, [esi + 8] -> thiscall, ret 8;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (a 0x4a2540 cria o registro do clube que não tem), com o plano: a ordem pela força
# (cpt13_ordenar_com, o núcleo da 0x4c08f0) e, depois, o lugar (+0x72) do registro de cada clube.
# Determinismo: a ordem vem da força dos clubes (0x4c0720); nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqv1_previstas_mineiro', 0x434f80, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv1_previstas_norte', 0x436280, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv1_previstas_nordeste', 0x437290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv1_previstas_paranaense', 0x438590, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas dos estaduais: Mineiro, Norte, Nordeste e Paranaense',
        'the predicted positions of the Minas Gerais state league, the North state league, the Northeast league and the Paraná state league',
        [('cqv1_previstas_mineiro', 'BraRegMinasGerais::set_predicted_positions', '0x434f80', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 0',
          'As posições previstas do Mineiro (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2 (a final entre os dois primeiros), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the Minas Gerais state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2 (the final between the top two), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('cqv1_previstas_norte', 'BraRegNorth::set_predicted_positions', '0x436280', 'bra_reg_north.cpp', 'thiscall, ret 0',
          'As posições previstas do Norte (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (o mata-mata dos quatro primeiros: os dois semifinalistas de baixo dividem o 3º lugar), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the North state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the top-four knockout: the two lower semi-finalists share third place), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('cqv1_previstas_nordeste', 'BraRegNortheast::set_predicted_positions', '0x437290', 'bra_reg_northeast.cpp', 'thiscall, ret 0',
          'As posições previstas do Nordeste (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (o mata-mata dos quatro primeiros: os dois semifinalistas de baixo dividem o 3º lugar), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the Northeast league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the top-four knockout: the two lower semi-finalists share third place), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('cqv1_previstas_paranaense', 'BraRegParana::set_predicted_positions', '0x438590', 'bra_reg_parana.cpp', 'thiscall, ret 0',
          'As posições previstas do Paranaense (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2 (a final entre os dois primeiros), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the Paraná state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2 (the final between the top two), and the others their place in the order; it is the board and press expectation for each club [inference]')])
CAMPOS = [('liga +0xb1: a tabela (linhas de 0x3b bytes, o clube em +0); ponteiro; lê', 'league +0xb1: the table (0x3b-byte rows, the club at +0); pointer; reads'),
          ('liga +0x3e: o número de clubes; valor; lê', 'league +0x3e: the club count; value; reads'),
          ('estatística da competição, registro do clube +0x72: o lugar previsto; valor; grava', 'competition statistics, club record +0x72: the predicted position; value; writes')]
INTERESSA = ['CMRTE']
