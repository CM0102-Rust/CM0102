# A caixa do técnico humano (news.cpp): responder a todas as notícias (News::auto_respond_all 0x7874b0: ao sair de
# férias 0x5fd2f0, ao ser demitido 0x69b790 e contratado 0x69ebc0; 2 chamadas na carreira até 2002-06-01), se há
# notícia pendente (News::has_urgent_items 0x787350: o turno do humano 0x5cde40, o menu 0x7615d0; 1003) e o texto num
# campo da notícia (NewsItem::set_slot_string 0x7858c0: as notícias de técnicos do manager_manager.cpp; 4). Sem SEH,
# sem sorteio (a 0x782b70, o texto da notícia, não sorteia até 4 níveis), sem leitura de lixo. As caixas "Error"
# (técnico da máquina, campo fora, sem texto) pela aviso_em. Iguais no cm0102-1080.exe. Simulação: as mutações pela
# carreira e pelo comparando. Comparadores: a resposta a todas roda só a original (as respostas mudam amistosos,
# emprego e transferências) e confere a devolução; as outras duas lado a lado (0xde1f64; os campos e o próximo).
SAVE = False
ROTINAS = [('responder_todas_as_noticias', 0x7874b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tecnico_tem_noticia_pendente', 0x787350, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gravar_texto_na_noticia', 0x7858c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a caixa do técnico humano: responder a todas as notícias, se há notícia pendente e o texto num campo da notícia',
        'the human manager\'s inbox: replying to every item, whether an item is pending and the text in an item slot',
        [('responder_todas_as_noticias', 'News::auto_respond_all', '0x7874b0', 'news.cpp', 'thiscall, ret 8',
          'da notícia mais nova para a mais antiga da caixa do humano, até a vaga vazia, dá a resposta automática (0x784100) pelo clube',
          'from the newest to the oldest item of the human\'s inbox, up to the empty slot, applies the automatic reply (0x784100) for the club'),
         ('tecnico_tem_noticia_pendente', 'News::has_urgent_items', '0x787350', 'news.cpp', 'thiscall, ret 8',
          'sem "só urgentes", se há notícia não lida; com ele, se uma notícia pede resposta (o texto devolve 2) ou é importante e não lida',
          'without "urgent only", whether there is an unread item; with it, whether an item needs a reply (the text returns 2) or is important and unread'),
         ('gravar_texto_na_noticia', 'NewsItem::set_slot_string', '0x7858c0', 'news.cpp', 'thiscall, ret 0xc',
          'copia o texto nos campos da notícia a partir do campo dado e devolve o primeiro campo livre depois dele (no máximo 50)',
          'copies the text into the item slots from the given slot and returns the first free slot after it (at most 50)')])
CAMPOS = [('News +0/+8: o vetor das notícias guardadas (0xdf bytes) e as entradas dos humanos (início do anel, próxima vaga); ponteiro e valor; lê (responder_todas_as_noticias, tecnico_tem_noticia_pendente)',
           'News +0/+8: the stored news array (0xdf bytes) and the humans\' entries (ring start, next slot); pointer and value; reads (responder_todas_as_noticias, tecnico_tem_noticia_pendente)'),
          ('notícia +4, +0xd6, +0xda: importante, id (0 vaga vazia), lida; valor; lê (tecnico_tem_noticia_pendente, responder_todas_as_noticias)',
           'news item +4, +0xd6, +0xda: important, id (0 empty slot), read; value; reads (tecnico_tem_noticia_pendente, responder_todas_as_noticias)'),
          ('notícia +5..+0xcc: os 50 campos de 4 bytes (texto ou valor) e o fim; valor; grava (gravar_texto_na_noticia)',
           'news item +5..+0xcc: the 50 4-byte slots (text or value) and the terminator; value; writes (gravar_texto_na_noticia)'),
          ('[0xde1f64] o texto montado da notícia; valor; grava pela 0x782b70 (tecnico_tem_noticia_pendente)',
           '[0xde1f64] the built news text; value; writes through 0x782b70 (tecnico_tem_noticia_pendente)')]
INTERESSA = []
