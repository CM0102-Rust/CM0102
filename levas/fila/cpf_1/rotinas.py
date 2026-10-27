# Recordes do clube (club_records.cpp), 1º item: as cópias (operator=) do recorde de nota (rating_record_assign
# 0x448230, 51 chamadas na carreira até 2002-06-01), de melhor em campo (man_of_match_record_assign 0x4484d0, 51) e de
# disciplina (discipline_record_assign 0x448500, 51). Irmãs da 216ª (0x448490 recordista, 0x448110, 0x448170): o
# recordista vai pela 0x448490 (já em Rust). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no
# jogo real comparando (as da 216ª escaparam da carreira e foram acusadas comparando). Sem SEH, sem caixa, sem sorteio,
# sem locais. Comparadores lado a lado pelo registro inteiro. Nenhuma em laço quente.
SAVE = False
ROTINAS = [('copiar_recorde_de_nota', 0x448230, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('copiar_recorde_de_melhor_em_campo', 0x4484d0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('copiar_recorde_de_disciplina', 0x448500, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('os recordes do clube: as cópias dos recordes de nota, de melhor em campo e de disciplina',
        'club records: the copies of the rating, man of the match and discipline records',
        [('copiar_recorde_de_nota', 'rating_record_assign', '0x448230', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de nota (0x1f bytes): a nota (+0), o recordista (+4, pela 0x448490), +0x1c e +0x1d',
          'copies the rating record (0x1f bytes): the rating (+0), the holder (+4, through 0x448490), +0x1c and +0x1d'),
         ('copiar_recorde_de_melhor_em_campo', 'man_of_match_record_assign', '0x4484d0', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de melhor em campo (0x1b bytes): as vezes (+0), o recordista (+1) e o ano (+0x19)',
          'copies the man of the match record (0x1b bytes): the count (+0), the holder (+1) and the year (+0x19)'),
         ('copiar_recorde_de_disciplina', 'discipline_record_assign', '0x448500', 'club_records.cpp', 'thiscall, ret 4',
          'copia o recorde de disciplina (0x1c bytes): amarelos (+0), vermelhos (+1), o recordista (+2) e o ano (+0x1a)',
          'copies the discipline record (0x1c bytes): yellows (+0), reds (+1), the holder (+2) and the year (+0x1a)')])
CAMPOS = [('recordes do clube: recorde de nota (0x1f bytes), de melhor em campo (0x1b) e de disciplina (0x1c), com o recordista de 0x18 bytes dentro; escreve (copiar_recorde_de_nota, copiar_recorde_de_melhor_em_campo, copiar_recorde_de_disciplina)',
           'club records: rating record (0x1f bytes), man of the match (0x1b) and discipline (0x1c), with the 0x18-byte holder inside; writes (copiar_recorde_de_nota, copiar_recorde_de_melhor_em_campo, copiar_recorde_de_disciplina)')]
INTERESSA = ['SGE', 'CMRTE']
