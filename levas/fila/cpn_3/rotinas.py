# A mídia e a caixa (media.cpp, news.cpp): a nota do jogador para uma história (Media::rate_player_form_for_story
# 0x736720, x87, 719 chamadas na carreira até 2002-06-01, pelas histórias 0x735870 e 0x736c00: decide qual jogador vira
# história), o texto da crise financeira (Media::write_financial_trouble_news_text 0x74af60, pela vtable dos textos de
# notícia) e as notícias da caixa do humano (News::get_inbox_items 0x786960, pela tela de notícias 0x788010; 46). Sem
# SEH, sem sorteio (até 4 níveis), sem leitura de lixo (os locais da 0x736720 e os argumentos usados como rascunho são
# todos gravados antes de lidos). A 0x736720 roda em precisão dupla (a do x87 do jogo); o tipo de contrato é +0x4f >> 4.
# A 0x755340 (Media::apply_squad_reaction_to_player_story) fica de fora: o vetor do elenco tem 57 vagas, a 0x541f90
# grava 50 e as originais gravam da 52ª em diante: a 51ª ([esp+0xe4]) é lida sem valor (lixo de pilha). Iguais no
# cm0102-1080.exe. Simulação: as mutações pela carreira e pelo comparando (todas na nota, que o comparando confere bit a
# bit). Comparadores: a nota pela devolução; o texto só a original (o objeto de saída acumula); a caixa só a original
# (aloca) com o plano dos ids.
SAVE = False
ROTINAS = [('nota_do_jogador_para_a_historia', 0x736720, [0x83, 0xec, 0x10, 0x55, 0x8b, 0x6c, 0x24, 0x1c]),
           ('texto_da_crise_financeira', 0x74af60, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('noticias_da_caixa', 0x786960, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a mídia e a caixa: a nota do jogador para uma história, o texto da crise financeira e as notícias da caixa do humano',
        'the media and the inbox: a player\'s rating for a story, the financial trouble text and the human\'s inbox items',
        [('nota_do_jogador_para_a_historia', 'Media::rate_player_form_for_story', '0x736720', 'media.cpp', 'thiscall, ret 0x18, st0',
          'a nota do jogador para um tipo de história: as estatísticas e a média das notas contra a média da liga, o peso, as partidas e a reputação; depois, pelo tipo, o contrato, a idade, os dias de casa e atributos',
          'the player\'s rating for a story kind: stats and the average of ratings against the league average, the weight, the games and the reputation; then, by kind, the contract, age, days at the club and attributes'),
         ('texto_da_crise_financeira', 'Media::write_financial_trouble_news_text', '0x74af60', 'media.cpp', 'thiscall, ret 0xc',
          'o título ou o corpo da notícia do clube em crise financeira, pela gravidade, com o nome do clube ligado',
          'the headline or body of the club in financial trouble story, by severity, with the club name linked'),
         ('noticias_da_caixa', 'News::get_inbox_items', '0x786960', 'news.cpp', 'thiscall, ret 0xc',
          'um vetor novo com as notícias da caixa do humano, da mais nova para a mais antiga, só as de uma categoria se pedida, e a conta',
          'a new array with the human\'s inbox items, newest first, only those of one category when asked, and the count')])
CAMPOS = [('[0xb74350] +0x10/+0x18: a soma e a conta das notas da liga (a média; sem jogos, 6,8); valor; lê (nota_do_jogador_para_a_historia)',
           '[0xb74350] +0x10/+0x18: the league rating sum and count (the average; no games, 6.8); value; reads (nota_do_jogador_para_a_historia)'),
          ('pessoa +0x18, +0x3e, +0x61: idade, a data (dias de casa, 0x54cdb0) e o registro de jogador; valor e ponteiro; lê (nota_do_jogador_para_a_historia)',
           'person +0x18, +0x3e, +0x61: age, the date (days at the club, 0x54cdb0) and the player record; value and pointer; reads (nota_do_jogador_para_a_historia)'),
          ('registro de jogador +0xb, +0x25, +0x2f: a reputação e dois atributos; valor; lê (nota_do_jogador_para_a_historia)',
           'player record +0xb, +0x25, +0x2f: reputation and two attributes; value; reads (nota_do_jogador_para_a_historia)'),
          ('contrato +0x4f: o tipo de contrato no meio alto; valor; lê (nota_do_jogador_para_a_historia)',
           'contract +0x4f: the contract type in the high nibble; value; reads (nota_do_jogador_para_a_historia)'),
          ('clube +0x80, +0x38, +0x52: reputação, nome e gênero; valor; lê (nota_do_jogador_para_a_historia, texto_da_crise_financeira)',
           'club +0x80, +0x38, +0x52: reputation, name and gender; value; reads (nota_do_jogador_para_a_historia, texto_da_crise_financeira)'),
          ('notícia, campos 0 e 1: o clube e a gravidade da crise; id e valor; lê (texto_da_crise_financeira)',
           'news item, slots 0 and 1: the club and the severity of the trouble; id and value; reads (texto_da_crise_financeira)'),
          ('News +0/+8, notícia +0xd6: o vetor, os anéis dos humanos e o id; ponteiro e valor; lê (noticias_da_caixa)',
           'News +0/+8, news item +0xd6: the array, the humans\' rings and the id; pointer and value; reads (noticias_da_caixa)')]
INTERESSA = ['CMRTE', 'CMScout']
