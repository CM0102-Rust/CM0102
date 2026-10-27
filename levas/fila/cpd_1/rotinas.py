# Seleções (national_teams.cpp), 1º item: o fim das convocações de uma competição (end_duty_for_competition 0x774f30,
# 2 chamadas na carreira até 2002-06-01; 0x86af1a) e de uma nação numa competição (0x774f90, 34; 0x86bc70...) e a poda
# dos registros de convocação dos clubes (prune_club_call_up_records 0x7774b0, 9; 0x76b5d5). Simulação: as mutações
# pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio, sem local lida
# sem valor (a data da 0x54c5b0 é gravada antes; o contador de vencidas é zerado). Comparadores: as duas primeiras pelo
# cursor da lista e o +0x10 de cada registro (a lista só é percorrida pela 0x68d1b0, nada aloca); a poda pelos
# registros e vetores, e só a original quando algum vetor seria solto (0x9452ca: soltar duas vezes). Nenhuma em laço
# quente.
SAVE = False
ROTINAS = [('encerrar_convocacoes_da_competicao', 0x774f30, [0x83, 0xec, 0x08, 0x56, 0x57]),
           ('encerrar_convocacoes_da_nacao', 0x774f90, [0x83, 0xec, 0x08, 0x56, 0x57]),
           ('podar_registros_de_convocacao', 0x7774b0, [0xa1, 0x50, 0x23, 0xae, 0x00])]
DOCS = ('as seleções: o fim das convocações de uma competição e a poda dos registros de convocação',
        'national teams: ending the call-ups of a competition and pruning the call-up records',
        [('encerrar_convocacoes_da_competicao', 'end_duty_for_competition', '0x774f30', 'national_teams.cpp', 'thiscall, ret 4',
          'na lista de registros de convocação, os da competição (+4) terminam amanhã (+0x10, o dia da 0x54c5b0)',
          'in the call-up record list, those of the competition (+4) end tomorrow (+0x10, the day from 0x54c5b0)'),
         ('encerrar_convocacoes_da_nacao', 'end_duty_for_nation_in_competition', '0x774f90', 'national_teams.cpp', 'thiscall, ret 8',
          'como a 0x774f30, só os registros da competição (+4) e da nação (+0)',
          'like 0x774f30, only the records of the competition (+4) and the nation (+0)'),
         ('podar_registros_de_convocacao', 'prune_club_call_up_records', '0x7774b0', 'national_teams.cpp', 'thiscall',
          'em cada registro de clube (+0x20, 6 bytes), tira as entradas vencidas (0x456770) do começo, desce as outras (0x777590) e solta o vetor que ficou vazio',
          'in each club record (+0x20, 6 bytes), drops the expired entries (0x456770) at the start, moves the rest down (0x777590) and frees the emptied buffer')])
CAMPOS = [('registro de convocação: +0 nação, +4 competição, +0x10 dia do fim (curto); lê e grava (encerrar_convocacoes_*)',
           'call-up record: +0 nation, +4 competition, +0x10 end day (short); reads and writes (encerrar_convocacoes_*)'),
          ('gerenciador das seleções +0x20: registros de clube (6 bytes: +0 vetor de entradas de 9 bytes, data e marca; +4 marca; +5 quantas); lê e grava (podar_registros_de_convocacao)',
           'national team manager +0x20: club records (6 bytes: +0 buffer of 9-byte entries, date and mark; +4 mark; +5 count); reads and writes (podar_registros_de_convocacao)')]
INTERESSA = ['SGE', 'CMRTE']
