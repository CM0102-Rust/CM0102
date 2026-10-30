# As notícias do jogador depois de uma história da mídia (media.cpp), as três com quadro SEH sem destrutor (a notícia
# local de 0xe0 bytes, como a 0x74ae60 da 247a) e entrega ao técnico do clube (0x7861e0): a gratidão ao técnico
# (Media::queue_player_gratitude_news 0x732c80, 25 chamadas na carreira até 2002-06-01, por 0x736c00 e 0x75a340: sorteia
# pelo tipo e sobe +0x44 da ficha), o pedido de apoio (Media::post_player_backing_request_news 0x743370, 1 chamada, só
# para clube humano) e a reação à crítica (Media::post_player_criticism_reaction_news 0x743a50, 1 chamada, aos técnicos
# dos clubes da 0x8d1da0). A tabela de saltos da 0x732c80 (0x732fb4, índices em 0x732fcc) está no código. Iguais no
# cm0102-1080.exe. Comparadores: só a original (a entrega não pode rodar duas vezes; a gratidão sorteia e grava a ficha).
SAVE = False
ROTINAS = [('noticia_de_gratidao_do_jogador', 0x732c80, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_de_pedido_de_apoio', 0x743370, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_de_reacao_a_critica', 0x743a50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as notícias do jogador depois de uma história da mídia: gratidão ao técnico, pedido de apoio e reação à crítica',
        'the player\'s news after a media story: gratitude to the manager, request for backing and reaction to criticism',
        [('noticia_de_gratidao_do_jogador', 'Media::queue_player_gratitude_news', '0x732c80', 'media.cpp', 'thiscall, ret 0x10',
          'a notícia 0xbda ao técnico: o jogador, o clube, o tipo (0x12/0x21 por sorteio para jogador na lista da mídia ou com contrato marcado, 0xc lesionado), a variante, o sorteio do tipo (que pode subir +0x44 da ficha), o técnico e o auxiliar',
          'news 0xbda to the manager: the player, the club, the type (0x12/0x21 by a draw for a player on the media list or with a flagged contract, 0xc when injured), the variant, the type\'s draw (which may raise +0x44 of the player record), the manager and the assistant'),
         ('noticia_de_pedido_de_apoio', 'Media::post_player_backing_request_news', '0x743370', 'media.cpp', 'thiscall, ret 0x10',
          'para clube humano, a notícia 0xbcf ao técnico: o jogador, o clube, a faixa de reputação e o grau do pedido pelos atributos, o tipo, o quarto argumento e se está listado para transferência',
          'for a human club, news 0xbcf to the manager: the player, the club, the reputation band and the request level by attributes, the type, the fourth argument and whether he is transfer-listed'),
         ('noticia_de_reacao_a_critica', 'Media::post_player_criticism_reaction_news', '0x743a50', 'media.cpp', 'thiscall, ret 0x10',
          'a notícia 0xbdc: o jogador, o clube, a reação (0..4 pela reputação, a categoria do contrato, a idade, os atributos e a resposta), a resposta, a confiança da diretoria abaixo de 4000, o tópico e o técnico, entregue aos técnicos dos clubes interessados (0x8d1da0)',
          'news 0xbdc: the player, the club, the reaction (0..4 by reputation, contract category, age, attributes and the response), the response, board confidence below 4000, the topic and the manager, delivered to the managers of the interested clubs (0x8d1da0)')])
CAMPOS = [('ficha do jogador (pessoa +0x61) +0x44: sobe 1 (até 20) na gratidão quando o sorteio(5) dá 0 ou 1; valor; lê e grava (noticia_de_gratidao_do_jogador)',
           'player record (person +0x61) +0x44: raised by 1 (up to 20) in the gratitude news when the draw(5) gives 0 or 1; value; reads and writes (noticia_de_gratidao_do_jogador)'),
          ('ficha do jogador +0xb: reputação; valor; lê (pedido de apoio, reação à crítica)',
           'player record +0xb: reputation; value; reads (backing request, criticism reaction)'),
          ('pessoa +0, +4/+8/+0xc (+0x33), +0x18, +0x3e/+0x42, +0x57, +0x59, +0x5b, +0x5d: id, as chaves, idade, a chegada e atributos; valor; lê (as três)',
           'person +0, +4/+8/+0xc (+0x33), +0x18, +0x3e/+0x42, +0x57, +0x59, +0x5b, +0x5d: id, keys, age, arrival and attributes; value; reads (all three)'),
          ('contrato +0x45: marcas 0x2000, 8 e 0x181; valor; lê (gratidão, reação à crítica)',
           'contract +0x45: flags 0x2000, 8 and 0x181; value; reads (gratitude, criticism reaction)'),
          ('Media +0/+8: a lista da mídia (0x17 bytes: +8 id da pessoa, +0x10 marca 2) e a conta; ponteiro e valor; lê (noticia_de_gratidao_do_jogador)',
           'Media +0/+8: the media list (0x17 bytes: +8 person id, +0x10 flag 2) and its count; pointer and value; reads (noticia_de_gratidao_do_jogador)'),
          ('clube +0, +0x80, +0xcf, +0xd3: id, reputação, técnico e auxiliar; linha em [[0xb67a44]] +6 (confiança da diretoria); valor e ponteiro; lê',
           'club +0, +0x80, +0xcf, +0xd3: id, reputation, manager and assistant; row in [[0xb67a44]] +6 (board confidence); value and pointer; reads')]
INTERESSA = ['CMRTE', 'CMScout']
