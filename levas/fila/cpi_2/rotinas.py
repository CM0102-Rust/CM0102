# Recordes do clube (club_records.cpp), construtores das sequências: vitórias seguidas (won_run_record_construct
# 0x450ad0), derrotas seguidas (lost_run_record_construct 0x450b20) e jogos sem perder (unbeaten_run_record_construct
# 0x450b70), que só montam as datas do começo e do fim (0x54a110). Chamados pela construção dos recordes do clube
# (0x44c650). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Quadro SEH no original (a
# 0x54a110 não falha), sem caixa, sem sorteio, sem locais. Comparadores lado a lado pelo registro (0x14 bytes).
SAVE = False
ROTINAS = [('construir_recorde_de_vitorias_seguidas', 0x450ad0, [0x6a, 0xff, 0x68, 0x9b, 0x56, 0x95, 0x00]),
           ('construir_recorde_de_derrotas_seguidas', 0x450b20, [0x6a, 0xff, 0x68, 0xbb, 0x56, 0x95, 0x00]),
           ('construir_recorde_de_invencibilidade', 0x450b70, [0x6a, 0xff, 0x68, 0xdb, 0x56, 0x95, 0x00])]
DOCS = ('os recordes do clube: a construção dos recordes de vitórias, derrotas e invencibilidade seguidas',
        'club records: the construction of the won, lost and unbeaten run records',
        [('construir_recorde_de_vitorias_seguidas', 'won_run_record_construct', '0x450ad0', 'club_records.cpp', 'thiscall',
          'constrói o recorde de vitórias seguidas: as datas do começo (+4) e do fim (+0xc) (0x54a110)',
          'constructs the won run record: the start (+4) and end (+0xc) dates (0x54a110)'),
         ('construir_recorde_de_derrotas_seguidas', 'lost_run_record_construct', '0x450b20', 'club_records.cpp', 'thiscall',
          'constrói o recorde de derrotas seguidas: as datas do começo (+4) e do fim (+0xc) (0x54a110)',
          'constructs the lost run record: the start (+4) and end (+0xc) dates (0x54a110)'),
         ('construir_recorde_de_invencibilidade', 'unbeaten_run_record_construct', '0x450b70', 'club_records.cpp', 'thiscall',
          'constrói o recorde de jogos sem perder: as datas do começo (+4) e do fim (+0xc) (0x54a110)',
          'constructs the unbeaten run record: the start (+4) and end (+0xc) dates (0x54a110)')])
CAMPOS = [('recordes do clube: recordes de sequência (vitórias, derrotas, sem perder: começo +4, fim +0xc); escreve (construir_recorde_de_vitorias_seguidas, construir_recorde_de_derrotas_seguidas, construir_recorde_de_invencibilidade)',
           'club records: run records (won, lost, unbeaten: start +4, end +0xc); writes (construir_recorde_de_vitorias_seguidas, construir_recorde_de_derrotas_seguidas, construir_recorde_de_invencibilidade)')]
INTERESSA = ['SGE', 'CMRTE']
