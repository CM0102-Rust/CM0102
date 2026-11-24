# A Copa dos Campeoes do Brasil: a fase preliminar e a fase seguinte (bra_champ_cup.cpp; leitor cx01, rodada 5,
# 24/11/2026). Reescreve no estilo de hoje as tres rotinas do cqt_6 (FORA desde 06/11: as mutacoes dele nao mudavam a
# carreira), agora ao lado do resto da copa (cx01_10, cx01_12, cx01_13) e com mutacoes de efeito. Usa
# BRA_CHAMP_CUP_SOURCE do cx01_10 (integrar depois dele). Carreira de 8 ligas: 2001-167, 2001-183 e 2001-200.
# - 0x426130 BraChampCup::create_preliminary_table thiscall() ret 0 -> 0/1: malloc 0xb1 cdecl; League::init_table_row
#   0x687430 (push 0; push 0; push clube; push o dword da linha com lixo em cima, so o byte e lido; mov ecx ->
#   thiscall, ret 0x10); League::send_group_draw_news 0x684230 (thiscall, ret 0); a caixa 0x195.
# - 0x427320 BraChampCup::start_preliminary_stage thiscall() ret 0 (chamada pela bra_nat_first_fill_champions_cup_entrants
#   0x42be2a): o slot 0x5c; 0x426130 e League::create_fixtures 0x6835c0 (thiscall, ret 0); League::sort_table 0x6827d0
#   (push 0; mov ecx -> thiscall, ret 4); as caixas 0x491, 0x497 e 0x49e.
# - 0x4263c0 BraChampCup::advance_stage thiscall() ret 0 (slot 0x28): 0x426470 e 0x426720 (call direto com o ecx da
#   copa -> thiscall, ret 0); a caixa 0x219.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original (alocam, mandam a noticia e sorteiam), com o plano: a tabela com os 3 primeiros inscritos
# e eles fora do livro; a copa ativa com 3 clubes depois da preliminar; a fase +1 fora da ultima (os grupos gravam a
# deles depois: ai so confere que andou).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_champ_cup_create_preliminary_table', 0x426130, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('bra_champ_cup_start_preliminary_stage', 0x427320, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_champ_cup_advance_stage', 0x4263c0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a Copa dos Campeões do Brasil: a fase preliminar e a fase seguinte',
        'the Brazilian Champions Cup: the preliminary stage and the next stage',
        [('bra_champ_cup_create_preliminary_table', 'BraChampCup::create_preliminary_table', '0x426130', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'a preliminar de 3 clubes (a própria copa é a liga): os 3 primeiros inscritos vão às linhas da tabela e saem do livro; os humanos recebem a notícia do grupo',
          'the 3-club preliminary (the cup itself is the league): the first 3 entrants go to the table rows and leave the book; the humans get the group news'),
         ('bra_champ_cup_start_preliminary_stage', 'BraChampCup::start_preliminary_stage', '0x427320', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'começa a preliminar: as posições previstas, a tabela dos 3, os jogos e a ordem da tabela; a copa fica ativa',
          'starts the preliminary: the predicted positions, the 3-club table, the fixtures and the table order; the cup becomes active'),
         ('bra_champ_cup_advance_stage', 'BraChampCup::advance_stage', '0x4263c0', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'a fase avança uma: no fim da preliminar vêm os dois grupos; no fim dos grupos, as semifinais; na última fase, a caixa de erro',
          'the stage moves on by one: after the preliminary come the two groups; after the groups, the semi-finals; at the last stage, the error box')])
CAMPOS = []
INTERESSA = []
