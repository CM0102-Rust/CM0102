# A notícia nova e as confianças depois de uma história da mídia (news.cpp, media.cpp): o construtor da notícia
# (NewsItem::NewsItem 0x77bbe0, quadro SEH só da data, destrutor vazio 0x5a53c0), a confiança da torcida
# (Media::adjust_supporter_confidence 0x738340, 367 chamadas na carreira até 2002-06-01) e a da diretoria
# (Media::adjust_board_confidence 0x7386f0, 339 chamadas), as duas chamadas pelas respostas às histórias (0x736c00,
# 0x73e870, 0x741020). Os ajustes têm quadro SEH sem destrutor (a notícia local de 0xe0 bytes, como a 0x74ae60 da
# 247a), x87 (_ftol) e, fora de ±500, sorteio(4) e a notícia 0xbe0 ao técnico do clube. Iguais no cm0102-1080.exe.
# Comparadores: o construtor pela memória da notícia; os ajustes rodam só a original (sorteio e entrega) e conferem a
# confiança nova calculada antes (a variação é pura).
SAVE = False
ROTINAS = [('construir_noticia', 0x77bbe0, [0x6a, 0xff, 0x68, 0xfe, 0xe0, 0x95, 0x00]),
           ('ajustar_confianca_da_torcida', 0x738340, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('ajustar_confianca_da_diretoria', 0x7386f0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a notícia nova e a confiança da torcida e da diretoria depois de uma história da mídia',
        'the new news item and the supporters\' and board\'s confidence after a media story',
        [('construir_noticia', 'NewsItem::NewsItem', '0x77bbe0', 'news.cpp', 'thiscall, ret 8',
          'monta a notícia: tipo (+0), byte importante (+4), os 50 dados zerados, a data de hoje (+0xcd), o byte [0xae2c88] em +0xd5, id e +0xda zerados',
          'builds the news item: type (+0), important byte (+4), the 50 data slots cleared, today\'s date (+0xcd), byte [0xae2c88] at +0xd5, id and +0xda cleared'),
         ('ajustar_confianca_da_torcida', 'Media::adjust_supporter_confidence', '0x738340', 'media.cpp', 'thiscall, ret 0x14',
          'soma na confiança da torcida (+8 da linha do clube em [[0xb67a44]], 1..10000) o alvo menos a confiança atual, × 0,75, amortecido pelo técnico favorito ou rejeitado e acima de ±1000; fora de ±500, a notícia 0xbe0 ao técnico',
          'adds to the supporters\' confidence (+8 of the club row in [[0xb67a44]], 1..10000) the target minus the current confidence, × 0.75, damped by a favoured or rejected manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager'),
         ('ajustar_confianca_da_diretoria', 'Media::adjust_board_confidence', '0x7386f0', 'media.cpp', 'thiscall, ret 0x18',
          'soma na confiança da diretoria (+6 da linha do clube) (confiança − 6000)/20 + alvo, × 0,75, amortecido pelo presidente que gosta ou não do técnico e acima de ±1000; fora de ±500, a notícia 0xbe0 ao técnico com as marcas do sujeito',
          'adds to the board\'s confidence (+6 of the club row) (confidence − 6000)/20 + target, × 0.75, damped by a chairman who likes or dislikes the manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager with the subject\'s flags')])
CAMPOS = [('linha do clube em [[0xb67a44]] (id × 0x49) +6: confiança da diretoria (1..10000); valor; lê e grava (ajustar_confianca_da_diretoria)',
           'club row in [[0xb67a44]] (id × 0x49) +6: board confidence (1..10000); value; reads and writes (ajustar_confianca_da_diretoria)'),
          ('linha do clube em [[0xb67a44]] +8: confiança da torcida (1..10000); valor; lê e grava (ajustar_confianca_da_torcida)',
           'club row in [[0xb67a44]] +8: supporters\' confidence (1..10000); value; reads and writes (ajustar_confianca_da_torcida)'),
          ('linha do clube em [[0xb67a44]] +0xe: palavra levada na notícia da diretoria; valor; lê (ajustar_confianca_da_diretoria)',
           'club row in [[0xb67a44]] +0xe: word carried in the board news; value; reads (ajustar_confianca_da_diretoria)'),
          ('clube +0, +0xbf, +0xcf: id, o presidente e o técnico; id e ponteiro; lê (os ajustes)',
           'club +0, +0xbf, +0xcf: id, the chairman and the manager; id and pointer; reads (both adjustments)'),
          ('pessoa +0x18, +0x3e/+0x42, +0x59, +0x5b, +0x61: idade, a data de chegada, dois atributos e a ficha (+5 reputação); valor e ponteiro; lê (ajustar_confianca_da_diretoria)',
           'person +0x18, +0x3e/+0x42, +0x59, +0x5b, +0x61: age, arrival date, two attributes and the player record (+5 reputation); value and pointer; reads (ajustar_confianca_da_diretoria)'),
          ('notícia +0, +4, +5..+0xcc, +0xcd, +0xd5, +0xd6, +0xda: tipo, importante, os 50 dados, a data, o byte de [0xae2c88], id, lida; valor; grava (construir_noticia)',
           'news item +0, +4, +5..+0xcc, +0xcd, +0xd5, +0xd6, +0xda: type, important, the 50 data slots, the date, the byte from [0xae2c88], id, read; value; writes (construir_noticia)'),
          ('[0xae2c90] a data de hoje; [0xb74350] +0x10/+0x18 as notas da liga; valor; lê',
           '[0xae2c90] today\'s date; [0xb74350] +0x10/+0x18 the league ratings; value; reads')]
INTERESSA = ['CMRTE']
