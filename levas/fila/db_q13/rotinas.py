ROTINAS = [('forma_do_adjetivo', 0x545a60, [0x8b, 0x4c, 0x24, 0x08, 0x0c, 0xff]),
           ('atributo_do_nao_jogador', 0x543f30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nota_relativa_com_habilidade_reduzida', 0x5442f0, [0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08])]
DOCS = ('a forma do adjetivo por idioma, o atributo de treinador da pessoa e a nota com a habilidade reduzida',
        "the adjective form per language, the person's coaching attribute and the rating with the reduced ability",
        [('forma_do_adjetivo', 'get_adjective_form_index', '0x545a60', 'Database.cpp', 'cdecl',
          'a forma (0..4) do adjetivo para o índice 1..35 nos idiomas 3..10 que flexionam (tabelas de salto); outro idioma ou índice: 0xff',
          'the adjective form (0..4) for index 1..35 in the inflecting languages 3..10 (jump tables); any other language or index: 0xff'),
         ('atributo_do_nao_jogador', 'staff_get_non_player_attribute', '0x543f30', 'Database.cpp', 'cdecl',
          'o atributo 1..12 de treinador da pessoa: 0x543fd0 com o não jogador dela (+0x69)',
          "the person's coaching attribute 1..12: 0x543fd0 with their non-player record (+0x69)"),
         ('nota_relativa_com_habilidade_reduzida', 'player_attribute_rating_with_scaled_ability', '0x5442f0', 'Database.cpp', 'cdecl',
          'a nota de 1 a 20 de 2 × valor + habilidade atual × 0,1 (a de 0x544380 com a habilidade reduzida)',
          'the 1-to-20 rating of 2 × value + current ability × 0.1 (0x544380 with the ability scaled down)')])
CAMPOS = [('[0x545b2c..0x545db4] tabelas de salto das formas do adjetivo; valor; lê (forma_do_adjetivo)', '[0x545b2c..0x545db4] adjective form jump tables; value; reads (forma_do_adjetivo)'),
          ('staff +0x69: não jogador; ponteiro; lê (atributo_do_nao_jogador)', 'staff +0x69: non-player record; pointer; reads (atributo_do_nao_jogador)'),
          ('jogador +0x05: habilidade atual; valor; lê (nota_relativa_com_habilidade_reduzida)', 'player +0x05: current ability; value; reads (nota_relativa_com_habilidade_reduzida)'),
          ('[0x967880] 0,1, [0x967888] 0,05, [0x9688b0] 10, [0x968a80] 1/3, [0x968968] 0,5, [0x967890] 1, [0x968ea8] 20; valor; lê (nota_relativa_com_habilidade_reduzida)', '[0x967880] 0.1, [0x967888] 0.05, [0x9688b0] 10, [0x968a80] 1/3, [0x968968] 0.5, [0x967890] 1, [0x968ea8] 20; value; reads (nota_relativa_com_habilidade_reduzida)')]
INTERESSA = ['CMRTE', 'CMScout']
