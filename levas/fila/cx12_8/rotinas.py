# As honras da temporada da J1 League japonesa (jap_j1.cpp; leitor cx12, rodada 5, 24/11/2026). Simulacao com as 8
# ligas ate 2002-06-01: roda uma vez no fim da temporada de 2001 da J1 (dezembro de 2001, depois da final).
# - 0x664390 JapJ1::record_season_honours thiscall() ret 0 (virtual 0x30): com +0x2c (STAGE_COUNT) = 2 (o mesmo clube
#   venceu as duas fases, sem final), comp_util_add_history_record 0x4afce0 (cdecl, 5 args: this, campeao, vice, 0, 0)
#   com o primeiro da primeira fase ([[+0xc]].tabela[0]) campeao e o vice o segundo da tabela da fase com mais pontos na
#   segunda linha (League::get_table_row 0x6826d0 (1), thiscall ret 4; empate: a primeira fase); senao o salto de cauda
#   para a virtual 0x30 da fase 2 (a final). Prologo push esi; mov esi, ecx; cmp dword [esi+0x2c], 2 (7 bytes).
# Igual no cm0102-1080.exe (inteira). Comparador: so a original (escreve o historico).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('jap_j1_record_season_honours', 0x664390, [0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x2c, 0x02])]
DOCS = ('as honras da temporada da J1 League japonesa', 'the J1 League season honours',
        [('jap_j1_record_season_honours', 'JapJ1::record_season_honours', '0x664390', 'jap_j1.cpp', 'thiscall, ret 0',
          'regra: quando o mesmo clube vence as duas fases (sem final), ele é o campeão no histórico e o vice é o segundo colocado da fase em que o segundo fez mais pontos (a primeira no empate); com final, as honras ficam com a fase final',
          'rule: when the same club wins both stages (no final), it is the champion in the history and the runner-up is the second-placed club of the stage whose second place has more points (the first on a tie); with a final, the final stage records the honours')])
CAMPOS = []
INTERESSA = []
