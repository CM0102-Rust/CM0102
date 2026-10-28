# Recordes do clube (club_records.cpp), construtores: o recorde de maior valor de transferência pago
# (transfer_fee_record_construct 0x4485d0) e recebido (transfer_fee_record_construct_2 0x448620) e o recorde de
# sequência (run_record_construct 0x448670), que só montam as duas datas (0x54a110). Irmãos dos construtores da 243ª.
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Quadro SEH no original (a 0x54a110 não
# falha: a cópia não precisa dele), sem caixa, sem sorteio, sem locais. Comparadores lado a lado pelo registro.
SAVE = False
ROTINAS = [('construir_recorde_de_valor_pago', 0x4485d0, [0x6a, 0xff, 0x68, 0x5b, 0x54, 0x95, 0x00]),
           ('construir_recorde_de_valor_recebido', 0x448620, [0x6a, 0xff, 0x68, 0x7b, 0x54, 0x95, 0x00]),
           ('construir_recorde_de_sequencia', 0x448670, [0x6a, 0xff, 0x68, 0x9b, 0x54, 0x95, 0x00])]
DOCS = ('os recordes do clube: a construção dos recordes de valor de transferência pago e recebido e do recorde de sequência',
        'club records: the construction of the transfer fee paid and received records and of the run record',
        [('construir_recorde_de_valor_pago', 'transfer_fee_record_construct', '0x4485d0', 'club_records.cpp', 'thiscall',
          'constrói o recorde de maior valor pago: as datas em +0x14 e +0x20 (0x54a110)',
          'constructs the highest fee paid record: the dates at +0x14 and +0x20 (0x54a110)'),
         ('construir_recorde_de_valor_recebido', 'transfer_fee_record_construct_2', '0x448620', 'club_records.cpp', 'thiscall',
          'constrói o recorde de maior valor recebido: as datas em +0x14 e +0x20 (0x54a110)',
          'constructs the highest fee received record: the dates at +0x14 and +0x20 (0x54a110)'),
         ('construir_recorde_de_sequencia', 'run_record_construct', '0x448670', 'club_records.cpp', 'thiscall',
          'constrói o recorde de sequência: as datas do começo (+4) e do fim (+0xc) (0x54a110)',
          'constructs the run record: the start (+4) and end (+0xc) dates (0x54a110)')])
CAMPOS = [('recordes do clube: recorde de valor de transferência (datas em +0x14 e +0x20) e de sequência (começo +4, fim +0xc); escreve (construir_recorde_de_valor_pago, construir_recorde_de_valor_recebido, construir_recorde_de_sequencia)',
           'club records: transfer fee record (dates at +0x14 and +0x20) and run record (start +4, end +0xc); writes (construir_recorde_de_valor_pago, construir_recorde_de_valor_recebido, construir_recorde_de_sequencia)')]
INTERESSA = ['SGE', 'CMRTE']
