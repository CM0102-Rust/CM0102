# O progresso do clube nas estatísticas da competição no Mineiro (Paranaense, Carioca), na MLS e na Série C (frente das
# competições, 03/11/2026, agente C). O slot 27 (+0x68, `set_club_progress`): grava os três códigos de progresso do
# clube (+0x73 atual, +0x74 mínima, +0x75 máxima) na entrada dele no Comp_Stats ([liga+8]), convertidos pela regra da
# competição.
# - 0x4350d0 BraRegMinasGerais::set_club_progress (84 bytes), slot 27 do Mineiro (0x968284), do Paranaense (0x9684a0)
#   e do Carioca (0x968608). 17.044 chamadas.
# - 0x909dd0 UsaMls::set_club_progress (220 bytes), slot 27 da MLS (0x970844). 2.445 chamadas.
# - 0x42e100 BraNatThird::set_club_progress (501 bytes de código, com as tabelas de salto 0x42e2f8, 0x42e314,
#   0x42e330 lidas byte a byte), slot 27 da Série C (0x967f00). 6.296 chamadas.
# (contagens da carreira de 8 ligas até 2002-06-01, docs/cobertura_competicoes.txt)
# A 0x42e100 tem um trecho morto: depois de `fase >= 7` (com sinal) ela testa `fase == 1` e `fase < 4`, que nunca são
# verdade; as fases 7 em diante gravam os códigos como vêm (a cópia não reproduz o trecho morto).
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   as três: thiscall (`mov ecx, [ecx+8]` para a 0x4a4850), ret 0x14 (5 argumentos: clube, fase, a, b, c; lidos como
#   byte), devolvem a entrada (o eax da 0x4a4850, intocado).
#   0x4a4850 Comp_Stats::find_club_entry (por endereço; original, 470 bytes com SEH): thiscall (ecx = [liga+8]),
#   1 argumento (o clube, `push eax`), ret 4.
# Comparadores: lado a lado sobre os três bytes da entrada (+0x73..+0x75), achada antes pela 0x4a4850 (que ordena a
# tabela na primeira vez, como a original faria, e depois só busca); a devolução.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqk8_progresso_no_mineiro', 0x4350d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('cqk8_progresso_na_mls', 0x909dd0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('cqk8_progresso_na_serie_c', 0x42e100, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o progresso do clube nas estatísticas da competição no Mineiro, Paranaense, Carioca, MLS e Série C',
        'the club progress in the competition statistics in Minas Gerais, Parana, Rio, the MLS and Series C',
        [('cqk8_progresso_no_mineiro', 'BraRegMinasGerais::set_club_progress', '0x4350d0', 'bra_reg_minas_gerais.cpp', 'thiscall',
          'Mineiro, Paranaense e Carioca: grava os códigos de progresso (posição atual, mínima e máxima) do clube; nos '
          'pontos corridos, a mínima abaixo de 3 vira 1 e a máxima abaixo de 3 vira 2 [inferência: os dois turnos têm '
          'tabelas próprias e a geral não dá título]',
          'Minas Gerais, Parana and Rio: writes the club progress codes (current, lowest and highest position); in the '
          'round robin, the lowest under 3 becomes 1 and the highest under 3 becomes 2 [inference: the two halves have '
          'tables of their own and the overall one gives no title]'),
         ('cqk8_progresso_na_mls', 'UsaMls::set_club_progress', '0x909dd0', 'usa_mls.cpp', 'thiscall',
          'MLS: na temporada regular e nas fases 0 e 1 dos play-offs, a posição atual 1, 2, 3, 4 vira 1, 3, 7, 10, a '
          'mínima 1 a 3 vira 1 e 4 vira 10, a máxima 1 a 3 vira 7 e 4 vira 10 (posições em escala da liga inteira '
          '[inferência]); na fase 2, cada código vira 2 × código − 1; depois, como vêm',
          'MLS: in the regular season and play-off stages 0 and 1, the current position 1, 2, 3, 4 becomes 1, 3, 7, 10, '
          'the lowest 1 to 3 becomes 1 and 4 becomes 10, the highest 1 to 3 becomes 7 and 4 becomes 10 (positions on the '
          'whole-league scale [inference]); in stage 2, each code becomes 2 × code − 1; after that, as they come'),
         ('cqk8_progresso_na_serie_c', 'BraNatThird::set_club_progress', '0x42e100', 'bra_nat_third.cpp', 'thiscall',
          'Série C: nos grupos (fases até 6), a posição no grupo vira a posição na escala das 49 vagas da Série C '
          '[inferência]: 1 e 2 → 1 (a máxima → 9), 3 → 17, 4 → 25, 5 → 33, 6 → 41, 7 → 49; nas fases seguintes, os '
          'códigos como vêm',
          'Series C: in the groups (stages up to 6), the group position becomes the position on the 49-place Series C '
          'scale [inference]: 1 and 2 → 1 (the highest → 9), 3 → 17, 4 → 25, 5 → 33, 6 → 41, 7 → 49; in later stages, the '
          'codes as they come')])
CAMPOS = [('liga +8: Comp_Stats (estatísticas da competição); ponteiro; lê', 'league +8: Comp_Stats (competition statistics); pointer; reads'),
          ('entrada do Comp_Stats +0x73 / +0x74 / +0x75: progresso do clube (posição atual, mínima, máxima); valor; grava',
           'Comp_Stats entry +0x73 / +0x74 / +0x75: club progress (current, lowest, highest position); value; writes'),
          ('Comp_Stats +0x18: marca de ordem da tabela; valor; lê e grava (pela 0x4a4850)', 'Comp_Stats +0x18: table order flag; value; reads and writes (through 0x4a4850)')]
INTERESSA = ['CMRTE']
