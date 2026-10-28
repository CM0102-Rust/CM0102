# Recordes do clube (club_records.cpp), construtores: jogos sem vencer (winless_run_record_construct 0x450bc0), gols
# numa partida (goals_in_match_record_construct 0x450c10) e o jogador mais novo (youngest_player_record_construct
# 0x450c70), que só montam as datas (0x54a110). Chamados pela construção dos recordes do clube (0x44c650). Simulação:
# as mutações pela carreira e a confirmação no jogo real comparando. Quadro SEH no original (a 0x54a110 não falha), sem
# caixa, sem sorteio, sem locais. Comparadores lado a lado pelo registro.
SAVE = False
ROTINAS = [('construir_recorde_sem_vencer', 0x450bc0, [0x6a, 0xff, 0x68, 0xfb, 0x56, 0x95, 0x00]),
           ('construir_recorde_de_gols_na_partida', 0x450c10, [0x6a, 0xff, 0x68, 0x1b, 0x57, 0x95, 0x00]),
           ('construir_recorde_do_mais_novo', 0x450c70, [0x6a, 0xff, 0x68, 0x43, 0x57, 0x95, 0x00])]
DOCS = ('os recordes do clube: a construção dos recordes de jogos sem vencer, de gols numa partida e do jogador mais novo',
        'club records: the construction of the winless run, goals in a match and youngest player records',
        [('construir_recorde_sem_vencer', 'winless_run_record_construct', '0x450bc0', 'club_records.cpp', 'thiscall',
          'constrói o recorde de jogos sem vencer: as datas do começo (+4) e do fim (+0xc) (0x54a110)',
          'constructs the winless run record: the start (+4) and end (+0xc) dates (0x54a110)'),
         ('construir_recorde_de_gols_na_partida', 'goals_in_match_record_construct', '0x450c10', 'club_records.cpp', 'thiscall',
          'constrói o recorde de gols numa partida: a data do recordista (+0x11) e a da partida (+0x19) (0x54a110)',
          'constructs the goals in a match record: the holder date (+0x11) and the match date (+0x19) (0x54a110)'),
         ('construir_recorde_do_mais_novo', 'youngest_player_record_construct', '0x450c70', 'club_records.cpp', 'thiscall',
          'constrói o recorde do jogador mais novo: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110)',
          'constructs the youngest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110)')])
CAMPOS = [('recordes do clube: sem vencer (começo +4, fim +0xc), gols numa partida (recordista +0x11, partida +0x19), mais novo (idade +0, recordista +0x18, partida +0x20); escreve (construir_recorde_sem_vencer, construir_recorde_de_gols_na_partida, construir_recorde_do_mais_novo)',
           'club records: winless (start +4, end +0xc), goals in a match (holder +0x11, match +0x19), youngest (age +0, holder +0x18, match +0x20); writes (construir_recorde_sem_vencer, construir_recorde_de_gols_na_partida, construir_recorde_do_mais_novo)')]
INTERESSA = ['SGE', 'CMRTE']
