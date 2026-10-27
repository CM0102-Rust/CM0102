# Recordes do clube (club_records.cpp), 2º item: as cópias do recorde de contagem (count_record_assign 0x448530, 153
# chamadas na carreira até 2002-06-01) e do jogador da torcida (fans_player_record_assign 0x4485b0, 51) e os
# construtores dos recordes de ocorrências (tally_record_construct 0x450c60, 114) e de contagem (count_record_construct
# 0x450d30, 54), que só montam a data do recordista (0x54a110). Simulação: as mutações pela carreira até 2002-06-01 e a
# confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio, sem locais. Comparadores lado a lado pelo
# registro inteiro. Nenhuma em laço quente.
SAVE = False
ROTINAS = [('copiar_recorde_de_contagem', 0x448530, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('copiar_recorde_do_torcedor', 0x4485b0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('construir_recorde_de_ocorrencias', 0x450c60, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x11]),
           ('construir_recorde_de_contagem', 0x450d30, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x12])]
DOCS = ('os recordes do clube: a cópia dos recordes de contagem e do jogador da torcida e a construção dos recordes de ocorrências e de contagem',
        'club records: the copy of the count and fans player records and the construction of the tally and count records',
        [('copiar_recorde_de_contagem', 'count_record_assign', '0x448530', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de contagem (0x1a bytes): a contagem (+0, palavra) e o recordista (+2, pela 0x448490)',
          'copies the count record (0x1a bytes): the count (+0, word) and the holder (+2, through 0x448490)'),
         ('copiar_recorde_do_torcedor', 'fans_player_record_assign', '0x4485b0', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de jogador da torcida (0x1a bytes): o recordista (+0) e o ano (+0x18)',
          'copies the fans player record (0x1a bytes): the holder (+0) and the year (+0x18)'),
         ('construir_recorde_de_ocorrencias', 'tally_record_construct', '0x450c60', 'club_records.cpp', 'thiscall',
          'constrói o recorde de ocorrências: a data do recordista em +0x11 (0x54a110)',
          'constructs the tally record: the holder date at +0x11 (0x54a110)'),
         ('construir_recorde_de_contagem', 'count_record_construct', '0x450d30', 'club_records.cpp', 'thiscall',
          'constrói o recorde de contagem: a data do recordista em +0x12 (0x54a110)',
          'constructs the count record: the holder date at +0x12 (0x54a110)')])
CAMPOS = [('recordes do clube: recorde de contagem (0x1a bytes: palavra + recordista), de jogador da torcida (0x1a: recordista + ano) e de ocorrências (0x19: byte + recordista); escreve (copiar_recorde_de_contagem, copiar_recorde_do_torcedor, construir_recorde_de_ocorrencias, construir_recorde_de_contagem)',
           'club records: count record (0x1a bytes: word + holder), fans player (0x1a: holder + year) and tally (0x19: byte + holder); writes (copiar_recorde_de_contagem, copiar_recorde_do_torcedor, construir_recorde_de_ocorrencias, construir_recorde_de_contagem)')]
INTERESSA = ['SGE', 'CMRTE']
