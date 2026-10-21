MOTOR = True
ROTINAS = [('organizar_area', 0x403350, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('iniciar_area', 0x402ac0, [0x66, 0x83, 0x7c, 0x24, 0x38, 0xff]),
           ('desenhar_area', 0x402e70, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('as áreas da interface (1): organizar a grade, iniciar e desenhar uma área (com a trava das 30 linhas: o "Area.cpp 507" do relatório em 1080)',
        'the interface areas (1): laying out the grid, initialising and drawing an area (with the 30-row clamp: the 1080 report "Area.cpp 507")',
        [('organizar_area', 'Area::layout', '0x403350', 'Area.cpp', 'thiscall',
          'reparte a largura e a altura da área pelos pesos das colunas e linhas (até 30 de cada; mais dá a caixa Area.cpp 497/507) e liga a barra de rolagem',
          'splits the area width and height by the column and row weights (up to 30 each; more raises the Area.cpp 497/507 box) and sets up the scrollbar'),
         ('iniciar_area', 'Area::init', '0x402ac0', 'Area.cpp', 'thiscall',
          'inicia a área: retângulo limitado à tela, campos, barra de rolagem zerada e os pesos das colunas e linhas; as linhas à mostra travadas em 30 (o tamanho da grade), o resto vai para a barra de rolagem',
          'initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights; the visible rows clamped at 30 (the grid size), the rest goes to the scroll bar'),
         ('desenhar_area', 'Area::draw', '0x402e70', 'Area.cpp', 'thiscall',
          'desenha a área: o painel, as linhas da grade ou cada item à mostra na sua célula, e a barra de rolagem',
          'draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar')])
