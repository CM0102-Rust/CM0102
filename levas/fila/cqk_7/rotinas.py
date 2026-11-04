# O destino de cada clube no fim das fases do Centro-Oeste e do Goiano, do Mineiro, Paranaense e Carioca, e da NSL
# (frente das competições, 03/11/2026, agente C). O slot 18 (+0x44, `update_staff_records_for_status`), chamado no fim
# de cada fase para cada clube com a situação dele; grava os registros de carreira do técnico (Staff_Records, já em
# Rust) e, nos estaduais, o destino na linha da tabela (+0x37).
# - 0x4314e0 BraRegCentral::update_staff_records_for_status (300 bytes), slot 18 do Centro-Oeste (0x968068) e do
#   Goiano (0x9681d0). 90 chamadas.
# - 0x434d90 BraRegMinasGerais::update_staff_records_for_status (315 bytes), slot 18 do Mineiro (0x968284), do
#   Paranaense (0x9684a0) e do Carioca (0x968608). 28 chamadas.
# - 0x411980 AusNsl::update_staff_records_for_status (170 bytes), slot 18 da NSL (0x967768). 19 chamadas.
# (contagens da carreira de 8 ligas até 2002-06-01, docs/cobertura_competicoes.txt)
# As duas dos estaduais são a mesma rotina do Baiano (0x430210, cqj_9) com outra fase do mata-mata (0 e 2) e outra
# escala de rodadas (3 e 1); o porte é um par de ajudantes comuns (pontos corridos, mata-mata) com os parâmetros de
# cada uma, conferidos byte a byte.
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   as três: thiscall (`mov ebx, ecx` / ecx lido direto), ret 0x18 (6 argumentos: clube, situação, fase, _, jogo, _;
#   situação e fase lidas como byte com sinal), devolvem 0 ou -1.
#   0x86bdd0 add_qualified_for_round_record: thiscall (`mov ecx, [0xdd7f00]`), 5 argumentos (imediatos), ret 0x14.
#   0x86a1c0 add_relegation_record, 0x868c50 add_league_champions_record: thiscall ([0xdd7f00]), 2 argumentos, ret 8.
#   0x86c000 add_knocked_out_record: thiscall ([0xdd7f00]), 5 argumentos (clube, comp, palavra +0x32 do jogo, palavra
#   +0x30, eax da escala), ret 0x14; a original empurra ecx/edx inteiros e o porte da 0x86c000 só usa as palavras: a
#   cópia passa as palavras (ponytail).
#   0x86c1d0 add_failed_to_qualify_record: thiscall ([0xdd7f00]), 4 argumentos (clube, comp, 0x435, 0x28), ret 0x10.
#   0x4bf850 comp_util_scale_by_round: cdecl (`add esp, 0x10`), 4 argumentos (0x1e, 0x32, palavra +0x34 do jogo, 3 ou 1).
# Comparadores: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com o
# plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e, nos estaduais, o destino
# gravado na linha.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqk7_destino_no_centro_oeste', 0x4314e0, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('cqk7_destino_no_mineiro', 0x434d90, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('cqk7_destino_na_nsl', 0x411980, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0xff])]
DOCS = ('o destino de cada clube no fim das fases do Centro-Oeste, Goiano, Mineiro, Paranaense, Carioca e da NSL',
        'the fate of each club at the end of the stages of the Centre-West, Goias, Minas Gerais, Parana, Rio and the NSL',
        [('cqk7_destino_no_centro_oeste', 'BraRegCentral::update_staff_records_for_status', '0x4314e0', 'bra_reg_central.cpp', 'thiscall',
          'Centro-Oeste e Goiano: no fim dos pontos corridos, os classificados para o mata-mata ganham o registro de '
          'classificação (rodada 0x82, peso 0x32) e os rebaixados o de rebaixamento; no mata-mata (fase 0), o campeão '
          'ganha o registro de campeão e o destino 5; os eliminados, o registro de eliminação com peso de 30 a 50 pela '
          'rodada (em 3 rodadas: quartas, semi, final) e o destino -2',
          'Centre-West and Goias: at the end of the round robin, the clubs through to the knockout get the qualification '
          'record (round 0x82, weight 0x32) and the relegated the relegation record; in the knockout (stage 0), the '
          'champion gets the champion record and fate 5; the eliminated, the knocked-out record weighted 30 to 50 by '
          'round (over 3 rounds: quarter-finals, semi-finals, final) and fate -2'),
         ('cqk7_destino_no_mineiro', 'BraRegMinasGerais::update_staff_records_for_status', '0x434d90', 'bra_reg_minas_gerais.cpp', 'thiscall',
          'Mineiro, Paranaense e Carioca: os pontos corridos como no Centro-Oeste; os dois turnos (fases 0 e 1) não '
          'gravam nada e devolvem -1; o mata-mata final (fase 2, uma rodada só: peso 40 para o eliminado) dá o '
          'campeão (destino 5) e o vice (destino -2)',
          'Minas Gerais, Parana and Rio: the round robin as in the Centre-West; the two halves (stages 0 and 1) record '
          'nothing and return -1; the final knockout (stage 2, a single round: weight 40 for the loser) gives the '
          'champion (fate 5) and the runner-up (fate -2)'),
         ('cqk7_destino_na_nsl', 'AusNsl::update_staff_records_for_status', '0x411980', 'aus_nsl.cpp', 'thiscall',
          'NSL: no fim da temporada regular, os seis classificados para a fase final ganham o registro de classificação '
          '(fase 0x435, peso 0x28); na fase final, quem avança ganha o registro (rodada 0x96) e quem cai o de não '
          'classificado; o campeão (grande final) ganha o registro de campeão. Não grava destino na tabela',
          'NSL: at the end of the regular season, the six clubs through to the finals series get the qualification '
          'record (stage 0x435, weight 0x28); in the finals, those who go through get the record (round 0x96) and those '
          'who fall the failed-to-qualify one; the champion (grand final) gets the champion record. No fate written to '
          'the table')])
CAMPOS = [('liga +4: registro da competição no banco; ponteiro; lê (vai nos registros de carreira)', 'league +4: competition record in the database; pointer; reads (goes into the career records)'),
          ('liga +0xb1 / +0x3e: tabela (linhas de 0x3b) e quantos times; ponteiro / valor; lê', 'league +0xb1 / +0x3e: table (0x3b rows) and team count; pointer / value; reads'),
          ('linha da tabela +0: clube; ponteiro; lê', 'table row +0: club; pointer; reads'),
          ('linha da tabela +0x37: destino (5 campeão, -2 eliminado); valor; grava (estaduais)', 'table row +0x37: fate (5 champion, -2 eliminated); value; writes (state leagues)'),
          ('jogo +0x30 / +0x32 / +0x34: rodada, fase e a rodada da escala; valor (palavra); lê', 'match +0x30 / +0x32 / +0x34: round, stage and the scale round; value (word); reads'),
          ('[0xdd7f00]: Staff_Records; ponteiro; lê', '[0xdd7f00]: Staff_Records; pointer; reads')]
INTERESSA = ['CMRTE']
