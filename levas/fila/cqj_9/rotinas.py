# O destino de cada clube no fim das fases da Série A, da Série C e dos estaduais com o slot do Baiano (frente das
# competições, 03/11/2026, agente C). O slot 18 (+0x44, `update_staff_records_for_status`): chamado no fim de cada fase
# para cada clube com a situação dele (6 classificado, 3 rebaixado, 1 vencedor, 2 / -1 eliminado, 0 promovido...); grava
# os registros de carreira do técnico (Staff_Records, já em Rust) e o destino na linha da tabela (+0x37).
# - 0x42abc0 BraNatFirst::update_staff_records_for_status (172 bytes), slot 18 da Série A (0x967d98). 14 chamadas.
# - 0x42eb00 BraNatThird::update_staff_records_for_status (336 bytes), slot 18 da Série C (0x967f00). 54 chamadas.
# - 0x430210 BraRegBahia::update_staff_records_for_status (300 bytes), slot 18 de sete estaduais: Baiano (0x967fb4),
#   Gaúcho (0x96811c), Norte (0x968338), Nordeste (0x9683ec), Pernambucano (0x968554), Catarinense (0x9686bc) e
#   Paulista (0x968770). 122 chamadas.
# (contagens da carreira de 8 ligas até 2002-06-01, docs/cobertura_competicoes.txt)
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   as três: thiscall (`mov esi, ecx` / `mov edi, ecx` / `mov ebx, ecx`), ret 0x18 (6 argumentos: clube, situação,
#   fase, _, jogo, _; situação e fase lidas como byte com sinal), devolvem 0 ou -1.
#   0x86bdd0 Staff_Records::add_qualified_for_round_record: thiscall (`mov ecx, [0xdd7f00]`), 5 argumentos (clube,
#   comp, fase, rodada, peso; empurrados na ordem inversa, imediatos), ret 0x14.
#   0x86a1c0 add_relegation_record e 0x868c50 add_league_champions_record: thiscall ([0xdd7f00]), 2 argumentos, ret 8.
#   0x869480 add_promotion_record: thiscall ([0xdd7f00]), 3 argumentos (clube, comp, 0x32), ret 0xc.
#   0x86c000 add_knocked_out_record: thiscall ([0xdd7f00]), 5 argumentos (clube, comp, fase = palavra +0x32 do jogo,
#   rodada = palavra +0x30, peso = eax da 0x4bf850), ret 0x14. A original empurra ecx/edx inteiros (lixo acima da
#   palavra); o porte da 0x86c000 só usa a palavra (0x4b0870 e `as u16`): a cópia passa as palavras (ponytail).
#   0x4bf850 comp_util_scale_by_round: cdecl (`add esp, 0x10`), 4 argumentos (0x1e, 0x32, a palavra +0x34 do jogo, 2).
#   0x68ab70 League::find_club_row: thiscall (`mov ecx, esi`), 2 argumentos (clube, 0), ret 8.
# Comparadores: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com o
# plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e o destino gravado na linha.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqj9_destino_na_serie_a', 0x42abc0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqj9_destino_na_serie_c', 0x42eb00, [0x51, 0x8a, 0x44, 0x24, 0x10]),
           ('cqj9_destino_no_baiano', 0x430210, [0x8a, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('o destino de cada clube no fim das fases da Série A, da Série C e de sete estaduais',
        'the fate of each club at the end of the stages of Series A, Series C and seven state leagues',
        [('cqj9_destino_na_serie_a', 'BraNatFirst::update_staff_records_for_status', '0x42abc0', 'bra_nat_first.cpp', 'thiscall',
          'Série A: no fim dos pontos corridos, os classificados para o mata-mata ganham o registro de classificação '
          '(fase 0x41c, peso 0x28) e os rebaixados o de rebaixamento; no mata-mata, o campeão ganha o registro de campeão '
          'e o destino 5 na tabela, os eliminados o destino -2',
          'Series A: at the end of the round robin, the clubs through to the knockout get the qualification record '
          '(stage 0x41c, weight 0x28) and the relegated the relegation record; in the knockout, the champion gets the '
          'champion record and fate 5 in the table, the eliminated fate -2'),
         ('cqj9_destino_na_serie_c', 'BraNatThird::update_staff_records_for_status', '0x42eb00', 'bra_nat_third.cpp', 'thiscall',
          'Série C: nos grupos, os classificados ganham o registro de classificação; na fase 7, quem passa; no quadrangular '
          'final (fases 8 e 9), os dois que sobem ganham os registros de acesso (peso 0x32) e de classificação e o destino 0 '
          '(acesso) na primeira tabela que os tiver; na final (fase 10), o campeão ganha o registro de campeão',
          'Series C: in the groups, the qualified get the qualification record; in stage 7, those who go through; in the '
          'final group (stages 8 and 9), the two promoted get the promotion (weight 0x32) and qualification records and '
          'fate 0 (promotion) in the first table that has them; in the final (stage 10), the champion gets the champion '
          'record'),
         ('cqj9_destino_no_baiano', 'BraRegBahia::update_staff_records_for_status', '0x430210', 'bra_reg_bahia.cpp', 'thiscall',
          'Baiano, Gaúcho, Norte, Nordeste, Pernambucano, Catarinense e Paulista: no fim dos pontos corridos, os '
          'classificados para a fase final ganham o registro de classificação (rodada 0x82, peso 0x32) e os rebaixados o '
          'de rebaixamento; no mata-mata, o campeão ganha o registro de campeão e o destino 5; os eliminados, o registro '
          'de eliminação com peso de 30 a 50 conforme a rodada em que caíram, e o destino -2',
          'Bahia, Gaucho, North, Northeast, Pernambuco, Santa Catarina and Paulista: at the end of the round robin, the '
          'clubs through to the final stage get the qualification record (round 0x82, weight 0x32) and the relegated '
          'the relegation record; in the knockout, the champion gets the champion record and fate 5; the eliminated, '
          'the knocked-out record weighted 30 to 50 by the round they fell in, and fate -2')])
CAMPOS = [('liga +4: registro da competição no banco; ponteiro; lê (vai nos registros de carreira)', 'league +4: competition record in the database; pointer; reads (goes into the career records)'),
          ('liga +0xb1 / +0x3e: tabela (linhas de 0x3b) e quantos times; ponteiro / valor; lê', 'league +0xb1 / +0x3e: table (0x3b rows) and team count; pointer / value; reads'),
          ('linha da tabela +0: clube; ponteiro; lê', 'table row +0: club; pointer; reads'),
          ('linha da tabela +0x37: destino (5 campeão, 0 acesso, -2 eliminado); valor; grava', 'table row +0x37: fate (5 champion, 0 promoted, -2 eliminated); value; writes'),
          ('liga +0xc: vetor de estágios; ponteiro; lê os estágios 0..6 (Série C)', 'league +0xc: stage vector; pointer; reads stages 0..6 (Series C)'),
          ('jogo +0x30 / +0x32 / +0x34: rodada, fase e a rodada da escala; valor (palavra); lê (Baiano)', 'match +0x30 / +0x32 / +0x34: round, stage and the scale round; value (word); reads (Bahia)'),
          ('[0xdd7f00]: Staff_Records; ponteiro; lê', '[0xdd7f00]: Staff_Records; pointer; reads')]
INTERESSA = ['CMRTE']
