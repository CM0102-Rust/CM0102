# A resposta do técnico da máquina a uma história da mídia (Media::pick_ai_manager_response 0x735870, 3,7 KB, 428
# chamadas na carreira até 2002-06-01, chamada pelas histórias 0x72c1f0, 0x733a60, 0x744240, 0x74b060, 0x754a30,
# 0x759610, 0x75a640, 0x75b6e0), marcar a notícia lida (News::mark_item_read 0x787140, 46 chamadas, pela tela das
# notícias) e achar a notícia pelo id (News::find_item_by_id 0x786b70, pelas telas, pelo texto e pelo botão das
# notícias). Sem SEH. A 0x735870 só sorteia e consulta (x87 só na nota: trunc(5 × 0x736720)); 8/0x25 e 3/4 podem pôr o
# jogador entre os rejeitados do técnico (0x547af0). Sem jogador, a nota fica com a palavra do argumento (o ponteiro do
# técnico): reproduzido. Iguais no cm0102-1080.exe. Comparadores: a resposta lado a lado no estado do gerador e nas
# preferências do técnico (a cópia sem contar os sorteios); marcar lida lado a lado no vetor das notícias, nas entradas
# dos humanos e no texto do botão; achar pela notícia copiada.
SAVE = False
ROTINAS = [('resposta_do_tecnico_da_maquina', 0x735870, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('marcar_noticia_lida', 0x787140, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('achar_noticia_pelo_id', 0x786b70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a resposta do técnico da máquina a uma história da mídia, marcar a notícia lida e achar a notícia pelo id',
        'the AI manager\'s response to a media story, marking a news item read and finding a news item by id',
        [('resposta_do_tecnico_da_maquina', 'Media::pick_ai_manager_response', '0x735870', 'media.cpp', 'thiscall, ret 0xc',
          'a resposta 1, 2 ou 3 do técnico da máquina à história do tipo sobre o jogador: pelo contrato, pela nota do jogador na história (× 5), pelas preferências do técnico e por cadeias de sorteios por família de tipos; alguns tipos põem o jogador entre os rejeitados do técnico',
          'the AI manager\'s response 1, 2 or 3 to a story of the given type about the player: by the contract, the player\'s story rating (× 5), the manager\'s likes and dislikes and chains of draws per family of types; some types add the player to the manager\'s rejected list'),
         ('marcar_noticia_lida', 'News::mark_item_read', '0x787140', 'news.cpp', 'thiscall, ret 0xc',
          'marca lida a primeira notícia do id ainda não lida no anel do humano, se ela não pede resposta (o botão 0x782b70), tira da conta e, a pedido, repete para os humanos da mesma máquina da rede',
          'marks read the first unread item with the id in the human\'s ring unless it needs a reply (button 0x782b70), takes it off the count and, when asked, repeats it for the humans on the same network machine'),
         ('achar_noticia_pelo_id', 'News::find_item_by_id', '0x786b70', 'news.cpp', 'thiscall, ret 0xc',
          'copia na saída a notícia do id: no anel de 100 vagas do técnico humano ou, sem técnico, entre todas as vagas',
          'copies the item with the id to the output: in the human manager\'s 100-slot ring or, without a manager, among all slots')])
CAMPOS = [('pessoa (técnico) +0x24, +0x39, +0x57, +0x59..+0x5d, +0x65, +0x69: seleção, clube, atributos, as preferências e a ficha de técnico (+0x14, +0x19, +0x1b, +0x1d, +0x1f); ponteiro e valor; lê, e grava +0x65 +0x28..+0x30 pela 0x547af0 (resposta_do_tecnico_da_maquina)',
           'person (manager) +0x24, +0x39, +0x57, +0x59..+0x5d, +0x65, +0x69: national team, club, attributes, preferences and the non-player record (+0x14, +0x19, +0x1b, +0x1d, +0x1f); pointer and value; reads, and writes +0x65 +0x28..+0x30 through 0x547af0 (resposta_do_tecnico_da_maquina)'),
          ('pessoa (jogador) +0, +0x5a, +0x5b, +0x61 (+0xb reputação, +0x25, +0x44, +0x45): id, atributos e a ficha; valor; lê (resposta_do_tecnico_da_maquina)',
           'person (player) +0, +0x5a, +0x5b, +0x61 (+0xb reputation, +0x25, +0x44, +0x45): id, attributes and the player record; value; reads (resposta_do_tecnico_da_maquina)'),
          ('contrato +0x3d, +0x3f, +0x49, +0x4f: valores e o tipo (nibble alto); valor; lê (resposta_do_tecnico_da_maquina)',
           'contract +0x3d, +0x3f, +0x49, +0x4f: values and the type (high nibble); value; reads (resposta_do_tecnico_da_maquina)'),
          ('[0xae2d0c] registro de 0x4f bytes da pessoa +0xa, +0xb (bit 7); valor; lê (resposta_do_tecnico_da_maquina)',
           '[0xae2d0c] the person\'s 0x4f-byte record +0xa, +0xb (bit 7); value; reads (resposta_do_tecnico_da_maquina)'),
          ('News +0/+4/+8: o vetor das notícias, a conta de vagas e as entradas dos 16 humanos (início do anel, próxima, conta); ponteiro e valor; lê e grava a conta (marcar_noticia_lida, achar_noticia_pelo_id)',
           'News +0/+4/+8: the news array, the slot count and the 16 humans\' entries (ring start, next, count); pointer and value; reads and writes the count (marcar_noticia_lida, achar_noticia_pelo_id)'),
          ('notícia +0xd6, +0xda: id e lida; valor; lê e grava +0xda (marcar_noticia_lida)',
           'news item +0xd6, +0xda: id and read; value; reads and writes +0xda (marcar_noticia_lida)'),
          ('pessoa +0x5f: humano ativo (1); valor; lê (marcar_noticia_lida)',
           'person +0x5f: active human (1); value; reads (marcar_noticia_lida)')]
INTERESSA = ['CMRTE', 'GPF2']
