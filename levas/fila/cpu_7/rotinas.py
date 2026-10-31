# A riqueza do clube e duas ordens (finance.cpp, game.cpp, stadium.cpp): a faixa das finanças Finance::get_wealth_band
# 0x5986b0 (44 chamadas: as ofertas de transferência 0x8e6774/0x8e67d7/0x8e7c5d, os técnicos e a diretoria 0x690503..
# 0x6accfd, as finanças 0x59658e..0x59ee1c, o dia do jogo 0x5d1104, as listas 0x83f6e9.., o adiamento 0x923704; roda
# todo dia na carreira automática), a ordem dos jogadores pela habilidade compare_players_by_ability 0x5d6220 (qsort em
# game_daily_staff_update 0x5d588f) e a dos estádios pela capacidade stadium_compare_venues_by_capacity 0x85e350 (qsort
# na busca do estádio neutro 0x85c800). Fora de fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no
# cm0102-1080.exe. Sem SEH, sem x87, sem locais; a 0x5986b0 só chama a 0x53b900 (em Rust) e devolve só o al (os bits de
# cima do eax são sobra das contas, os chamadores leem o al). Comparadores: as três pelo valor.
SAVE = False
ROTINAS = [('faixa_de_riqueza', 0x5986b0, [0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00]),
           ('ordem_por_habilidade', 0x5d6220, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('ordem_pela_capacidade', 0x85e350, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x0c])]
DOCS = ('a riqueza do clube e duas ordens: a faixa das finanças, a ordem dos jogadores pela habilidade e a dos estádios pela capacidade',
        'the club wealth and two orders: the finance band, the player order by ability and the stadium order by capacity',
        [('faixa_de_riqueza', 'Finance::get_wealth_band', '0x5986b0', 'finance.cpp', 'thiscall',
          'seleção, 0; pelo saldo de 64 bits contra a reputação R do clube: acima de 2500·R + 5.000.000, 2; acima de 1000·R + 3.500.000, 1; abaixo de −2.500.000 − 1500·R, −2; abaixo de −250·R (R < 5000) ou (3750 − R)·1000, −1; senão 0',
          'national team, 0; by the 64-bit balance against the club reputation R: above 2500·R + 5,000,000, 2; above 1000·R + 3,500,000, 1; below −2,500,000 − 1500·R, −2; below −250·R (R < 5000) or (3750 − R)·1000, −1; otherwise 0'),
         ('ordem_por_habilidade', 'compare_players_by_ability', '0x5d6220', 'game.cpp', 'cdecl',
          'qsort dos dados de jogador: habilidade atual (+5), depois potencial (+7), crescente',
          'player data qsort: current ability (+5), then potential (+7), ascending'),
         ('ordem_pela_capacidade', 'stadium_compare_venues_by_capacity', '0x85e350', 'stadium.cpp', 'cdecl',
          'qsort dos locais: lugares (+0x3c) do estádio (+0xc), do maior para o menor; sem estádio, 0',
          'venue qsort: places (+0x3c) of the stadium (+0xc), largest first; without a stadium, 0')])
CAMPOS = [('finanças do clube +0/+4 saldo (64 bits), +8 id do clube; valor/id; lê (faixa_de_riqueza)',
           'club finance +0/+4 balance (64 bits), +8 club id; value/id; reads (faixa_de_riqueza)'),
          ('clube +0x80 reputação; valor; lê (faixa_de_riqueza)', 'club +0x80 reputation; value; reads (faixa_de_riqueza)'),
          ('dados do jogador +5 habilidade atual, +7 potencial; valores; lê (ordem_por_habilidade)',
           'player data +5 current ability, +7 potential ability; values; reads (ordem_por_habilidade)'),
          ('local do estádio +0xc estádio; ponteiro; estádio +0x3c lugares; valor; lê (ordem_pela_capacidade)',
           'venue entry +0xc stadium; pointer; stadium +0x3c places; value; reads (ordem_pela_capacidade)')]
INTERESSA = ['CMRTE', 'CMScout']
