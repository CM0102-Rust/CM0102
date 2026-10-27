# Cópias de registros, 1º item: a cópia da notícia (NewsItem::operator= 0x786010, 4.893 chamadas na carreira até
# 2002-06-01; news.cpp) e das transferências passada (past_transfer_copy_construct 0x8cb940, 950) e futura
# (future_transfer_assign 0x8cbf40, 149; transfer_manager.cpp). As datas vão pela Date::operator= 0x417c10, com o
# retorno numa local que ela mesma grava (como na 216ª). Simulação: as mutações pela carreira até 2002-06-01 e a
# confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio, sem local lida sem valor. Comparadores lado a
# lado pelo registro inteiro. Nenhuma em laço quente (a cópia da notícia é a mais chamada, ~13 por dia de jogo).
SAVE = False
ROTINAS = [('copiar_noticia', 0x786010, [0x83, 0xec, 0x08, 0xba, 0x32, 0x00, 0x00, 0x00]),
           ('copiar_transferencia_passada', 0x8cb940, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08]),
           ('copiar_transferencia_futura', 0x8cbf40, [0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00])]
DOCS = ('as cópias da notícia e das transferências passada e futura',
        'the copies of the news item and of the past and future transfers',
        [('copiar_noticia', 'NewsItem::operator=', '0x786010', 'news.cpp', 'thiscall, ret 4',
          'copia a notícia (0xdf bytes): +0, +4, as 50 fichas (+5), a data (+0xcd), +0xd5, +0xd6, +0xda e +0xde',
          'copies the news item (0xdf bytes): +0, +4, the 50 tokens (+5), the date (+0xcd), +0xd5, +0xd6, +0xda and +0xde'),
         ('copiar_transferencia_passada', 'past_transfer_copy_construct', '0x8cb940', 'transfer_manager.cpp', 'thiscall, ret 4',
          'copia a transferência passada (0x25 bytes): sete duplas, o byte +0x1c e a data (+0x1d)',
          'copies the past transfer (0x25 bytes): seven dwords, the byte +0x1c and the date (+0x1d)'),
         ('copiar_transferencia_futura', 'future_transfer_assign', '0x8cbf40', 'transfer_manager.cpp', 'thiscall, ret 4',
          'copia a transferência futura (0xc bytes): a data (+0) e o índice da oferta (+8)',
          'copies the future transfer (0xc bytes): the date (+0) and the offer index (+8)')])
CAMPOS = [('notícia (0xdf bytes: 50 fichas em +5, data em +0xcd), transferência passada (0x25 bytes, data em +0x1d) e futura (0xc bytes: data + índice da oferta); escreve (copiar_noticia, copiar_transferencia_passada, copiar_transferencia_futura)',
           'news item (0xdf bytes: 50 tokens at +5, date at +0xcd), past transfer (0x25 bytes, date at +0x1d) and future transfer (0xc bytes: date + offer index); writes (copiar_noticia, copiar_transferencia_passada, copiar_transferencia_futura)')]
INTERESSA = ['SGE', 'CMRTE']
