# As posições previstas dos estaduais: Pernambucano, Carioca, Catarinense e Paulista (o slot set_predicted_positions de bra_reg_pern.cpp, bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp; agente C da frente
# das competições, 06/11/2026). Rodam duas vezes cada na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt):
# na criação (do construtor, cqt_1..4) e na virada da temporada. O mesmo molde nos doze estaduais (conferido instrução a
# instrução: só mudam o fonte, as linhas das caixas e os lugares fixos do mata-mata: 1, 2, 3, 3 nos de semifinal; 1, 2,
# 3, 3, 5, 5, 5, 5 nos de quartas de final; 1, 2 nos de final); os itens cqu_4, cqv_1 e cqv_2 cobrem os doze.
# - 0x4398d0 BraRegPern::set_predicted_positions: lugares fixos 1, 2, 3, 3.
# - 0x43add0 BraRegRio::set_predicted_positions: lugares fixos 1, 2.
# - 0x43c110 BraRegSanta::set_predicted_positions: lugares fixos 1, 2, 3, 3.
# - 0x43d140 BraRegSp::set_predicted_positions: lugares fixos 1, 2, 3, 3.
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
ROTINAS = [('cqv2_previstas_pernambucano', 0x4398d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv2_previstas_carioca', 0x43add0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv2_previstas_catarinense', 0x43c110, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqv2_previstas_paulista', 0x43d140, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas dos estaduais: Pernambucano, Carioca, Catarinense e Paulista',
        'the predicted positions of the Pernambuco state league, the Rio state league, the Santa Catarina state league and the São Paulo state league',
        [('cqv2_previstas_pernambucano', 'BraRegPern::set_predicted_positions', '0x4398d0', 'bra_reg_pern.cpp', 'thiscall, ret 0',
          'As posições previstas do Pernambucano (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (o mata-mata dos quatro primeiros: os dois semifinalistas de baixo dividem o 3º lugar), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the Pernambuco state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the top-four knockout: the two lower semi-finalists share third place), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('cqv2_previstas_carioca', 'BraRegRio::set_predicted_positions', '0x43add0', 'bra_reg_rio.cpp', 'thiscall, ret 0',
          'As posições previstas do Carioca (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2 (a final entre os dois primeiros), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the Rio state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2 (the final between the top two), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('cqv2_previstas_catarinense', 'BraRegSanta::set_predicted_positions', '0x43c110', 'bra_reg_santa.cpp', 'thiscall, ret 0',
          'As posições previstas do Catarinense (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (o mata-mata dos quatro primeiros: os dois semifinalistas de baixo dividem o 3º lugar), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the Santa Catarina state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the top-four knockout: the two lower semi-finalists share third place), and the others their place in the order; it is the board and press expectation for each club [inference]'),
         ('cqv2_previstas_paulista', 'BraRegSp::set_predicted_positions', '0x43d140', 'bra_reg_sp.cpp', 'thiscall, ret 0',
          'As posições previstas do Paulista (na criação e na virada): os clubes do estadual ordenados pela força; os primeiros ganham os lugares do mata-mata, 1, 2, 3, 3 (o mata-mata dos quatro primeiros: os dois semifinalistas de baixo dividem o 3º lugar), e os outros a posição na ordem; é a expectativa da diretoria e da imprensa para cada clube [inferência]',
          'The predicted positions of the São Paulo state league (at creation and at the season turnover): the state league clubs sorted by strength; the first ones get the knockout places, 1, 2, 3, 3 (the top-four knockout: the two lower semi-finalists share third place), and the others their place in the order; it is the board and press expectation for each club [inference]')])
CAMPOS = [('liga +0xb1: a tabela (linhas de 0x3b bytes, o clube em +0); ponteiro; lê', 'league +0xb1: the table (0x3b-byte rows, the club at +0); pointer; reads'),
          ('liga +0x3e: o número de clubes; valor; lê', 'league +0x3e: the club count; value; reads'),
          ('estatística da competição, registro do clube +0x72: o lugar previsto; valor; grava', 'competition statistics, club record +0x72: the predicted position; value; writes')]
INTERESSA = ['CMRTE']
