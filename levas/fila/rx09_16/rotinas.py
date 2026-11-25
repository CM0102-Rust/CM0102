# O sorteio da primeira fase das eliminatorias asiaticas da Copa de 2006 (wc_asia_league.cpp; leitor rx09, rodada 6,
# 25/11/2026). Simulacao: na carreira de teste a secao asiatica e a campanha de 2004 para a Copa de 2006 (o sorteio no
# dia dele, em marco de 2005, pelo update_for_date ja em Rust, que chama as tres pelo endereco). LIGAS = 'ENG' e
# ATE = '2005-12-31', como os rx08_4..6 (as eliminatorias europeias de 2006). 0 chamadas na cobertura do mundo.
# - 0x912db0 WcAsiaLeague::get_first_round_group_size thiscall(grupo) ret 4 -> palavra: a tabela de saltos 0x912e94 pelo
#   byte +0xee (0: 4; 1: 3 no grupo 1; 2: 3 nos grupos 3 e 6; 3: 3 nos grupos 1, 3 e 5); fora de 0..3 a caixa 0x37f e 4.
#   Prologo movsx eax, byte [ecx+0xee] (7 bytes).
# - 0x912bd0 WcAsiaLeague::draw_first_round_groups thiscall() ret 0: a ordem 0..9 e as contagens num local; 20 trocas
#   com utils_random(10) duas vezes (a primeira e a posicao a); malloc 0xa0 em +0xf7 (caixa 0x340); ate 4 potes enquanto
#   o byte dos sorteados < +0xf3; o tamanho do grupo pelo get_first_round_group_size(g - 1). Prologo sub esp, 0x220.
# - 0x912ad0 WcAsiaLeague::init_first_group_table thiscall() ret 0 -> 0/1: +0x3e = o tamanho do grupo -1, malloc 59 x n
#   (caixa 0x30d e 0), as linhas (0x687430) com as vagas 0.. do sorteio, League::send_group_draw_news 0x684230; 1.
#   Prologo sub esp, 0x204.
# Iguais no cm0102-1080.exe. Comparadores: o tamanho lado a lado; o sorteio so a original e cada inscrita sorteada uma
# vez; a tabela so a original e as linhas contra o primeiro grupo do sorteio.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_asia_league_get_first_round_group_size', 0x912db0, [0x0f, 0xbe, 0x81, 0xee, 0x00, 0x00, 0x00]),
           ('wc_asia_league_draw_first_round_groups', 0x912bd0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('wc_asia_league_init_first_group_table', 0x912ad0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o sorteio da primeira fase das eliminatórias asiáticas', 'the Asian World Cup qualifying first round draw',
        [('wc_asia_league_get_first_round_group_size', 'WcAsiaLeague::get_first_round_group_size', '0x912db0', 'wc_asia_league.cpp', 'thiscall, ret 4',
          'regra: os dez grupos da primeira fase asiática têm 4 seleções; com seleções isentas (sedes e campeão asiáticos), um, dois ou três grupos ficam com 3 (o grupo 1; os grupos 3 e 6; os grupos 1, 3 e 5)',
          'rule: the ten Asian first round groups have 4 national teams; with exempt nations (Asian hosts and champion), one, two or three groups have 3 (group 1; groups 3 and 6; groups 1, 3 and 5)'),
         ('wc_asia_league_draw_first_round_groups', 'WcAsiaLeague::draw_first_round_groups', '0x912bd0', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: as seleções, já ordenadas pela reputação, são distribuídas em até quatro potes pelos dez grupos, numa ordem de grupos sorteada, cada grupo recebendo uma por pote até o seu tamanho',
          'rule: the national teams, already sorted by reputation, are dealt in up to four pots over the ten groups, in a drawn group order, each group taking one a pot up to its size'),
         ('wc_asia_league_init_first_group_table', 'WcAsiaLeague::init_first_group_table', '0x912ad0', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: o primeiro grupo da primeira fase fica na tabela da própria eliminatória, com as seleções sorteadas para ele, e a notícia do sorteio é dada',
          'rule: the first group of the first round is the qualifying\'s own table, with the national teams drawn into it, and the draw news is sent')])
CAMPOS = [('eliminatórias asiáticas +0xf7: o sorteio da primeira fase, dez grupos de quatro ponteiros de seleção (0xa0 bytes, liberado depois do sorteio)', 'Asian qualifying +0xf7: the first round draw, ten groups of four national team pointers (0xa0 bytes, freed after the draw)')]
INTERESSA = []
