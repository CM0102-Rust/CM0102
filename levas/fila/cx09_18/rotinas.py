# O grupo norte da Segunda Divisao croata: a passagem de fase, a repescagem de desempate e o dia (cro_a2a.cpp; leitor
# cx09, rodada 5, 24/11/2026). Simulacao com a Croacia (as ligas do cx03_6: 'ENG POL TUR GRE HRV RUS', ate 2002-08-01);
# na cobertura do mundo a passagem e a repescagem dao 0 (nenhum empate na fronteira em 2002); o dia roda todo dia.
# - 0x516060 CroA2a::advance_stage thiscall() ret 0 (virtual 0x28): o molde do grupo sul (CroA2b::advance_stage
#   0x517180, state_league_advance_stage) com o arquivo 0x9b53f8 (caixa 0x1e9) e a repescagem 0x5160f0. Prologo mov edx,
#   [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x5160f0 CroA2a::create_tiebreak_playoff_stage thiscall() ret 0: o molde do grupo sul (0x517210,
#   create_croatian_tiebreak_playoff), caixa 0x23a do cro_a2a.cpp. Prologo push -1; push imm32.
# - 0x516360 CroA2a::update_for_date thiscall(hoje, completo) ret 8 -> al (virtual 0x14): League::update_for_date
#   0x6847c0; com +0xdd, +0x2c = 0, completo e +0xe6: +0x4c = 0; o primeiro par vizinho com os mesmos pontos (+0x19 e
#   +0x54) que nao seja de dois rebaixados (+0x37 e +0x72 = 3): +0x2c = 1, +0xc = malloc(4), virtual 0x28; sem par: +0xdd
#   = 0, +0xe6 = 0 e virtual 0x94. Devolve o byte do dia (o original deixa em cima o lixo do eax: ninguem le). Prologo
#   mov eax, [esp+4]; push ebx (5 bytes).
# Comparadores: a passagem por ita_ser_c1a_compare_advance_stage e a repescagem por compare_croatian_tiebreak_playoff
# (os do grupo sul); o dia so a original, conferida depois (fase criada = par empatado; o fim nunca fica pendente).
# Iguais no cm0102-1080.exe. Sem dependencias.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('cro_a2a_advance_stage', 0x516060, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cro_a2a_create_tiebreak_playoff_stage', 0x5160f0, [0x6a, 0xff, 0x68, 0x2e, 0x74, 0x95, 0x00]),
           ('cro_a2a_update_for_date', 0x516360, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('o grupo norte da Segunda Divisão croata: a passagem de fase, a repescagem de desempate e o dia',
        'the Croatian Second Division north group: the stage advance, the tie-break play-off and the day',
        [('cro_a2a_advance_stage', 'CroA2a::advance_stage', '0x516060', 'cro_a2a.cpp', 'thiscall, ret 0',
          'a passagem de fase do grupo norte, como a do grupo sul',
          "the north group's next stage, like the south group's"),
         ('cro_a2a_create_tiebreak_playoff_stage', 'CroA2a::create_tiebreak_playoff_stage', '0x5160f0', 'cro_a2a.cpp', 'thiscall, ret 0',
          'a repescagem de desempate do grupo norte, como a do grupo sul',
          "the north group's tie-break play-off, like the south group's"),
         ('cro_a2a_update_for_date', 'CroA2a::update_for_date', '0x516360', 'cro_a2a.cpp', 'thiscall, ret 8',
          'no fim da temporada, dois clubes vizinhos com os mesmos pontos (a não ser que ambos já tenham caído) vão a uma repescagem de desempate; sem empate, a temporada fecha e a tabela é atualizada',
          'at the end of the season, two neighbouring clubs level on points (unless both are already relegated) go to a tie-break play-off; with no tie, the season closes and the table is updated')])
CAMPOS = []
INTERESSA = []
