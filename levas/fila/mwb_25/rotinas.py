# A Copa Premier galesa (wel_prm_cup.cpp, em cups\): os inscritos e as posições previstas. Leitor r4_mwab (20/11/2026),
# lista do mundo (Escócia e País de Gales), continuação do mwb.
# - 0x928860 (WelPrmCup::build_entrant_list): calloc (0x945501) de 12; o detentor da Copa de Gales por
#   comp_util_get_latest_placed_club (0x4afd80, cdecl, 2 argumentos); as três passadas pela Liga de Gales até 8; 0x4afb40
#   nos n primeiros; Cardiff, Swansea, Wrexham e Merthyr (as chaves 0x9d0434, 0x9d0430, 0x9d043c e 0x9d0438, nessa ordem)
#   e 0x4afb40 nas entradas 8 a 11.
# - 0x928e00 (WelPrmCup::set_predicted_positions): italian_cup_predicted_positions com a fase 1 e faixas de 2 (lido
#   instrução por instrução contra 0x63e580: o quadro e os registros diferem, os mesmos passos).
# Convenções conferidas: thiscall, ret 0.
# Comparadores: os inscritos lado a lado com o gerador restaurado e a cópia sobre o histórico como a original o viu (a
# lição do compare_ger_lge_cup_collect_entrants: 0x4afd80 reordena o histórico global com qsort instável); as posições
# lado a lado.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_prm_cup_build_entrant_list', 0x928860, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('wel_prm_cup_set_predicted_positions', 0x928e00, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('a Copa Premier galesa: os inscritos e as posições previstas',
        "the Welsh Premier Cup: the entrants and the predicted positions",
        [('wel_prm_cup_build_entrant_list', 'WelPrmCup::build_entrant_list', '0x928860', 'wel_prm_cup.cpp', 'thiscall, ret 0',
          'regra: 12 clubes: o detentor da Copa de Gales, depois clubes da Liga de Gales pela colocação da temporada passada (até o 8o, ou até o 7o quando o próprio detentor ficou do 9o para baixo ou não há detentor; depois os outros; depois os da Liga de Gales desta temporada) até serem 8, embaralhados; e os 4 clubes galeses da liga inglesa (Cardiff City, Swansea City, Wrexham e Merthyr Tydfil), embaralhados entre si',
          "rule: 12 clubs: the Welsh Cup holder, then League of Wales clubs by last season's place (up to 8th, or up to 7th when the holder itself finished 9th or lower or there is no holder; then the rest; then this season's League of Wales clubs) until there are 8, shuffled; and the 4 Welsh clubs of the English league (Cardiff City, Swansea City, Wrexham and Merthyr Tydfil), shuffled among themselves"),
         ('wel_prm_cup_set_predicted_positions', 'WelPrmCup::set_predicted_positions', '0x928e00', 'wel_prm_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas dos 12 inscritos, pela reputação: o mata-mata (fase 1) dá 1 ao campeão e 2 ao vice, depois uma posição a cada leva de eliminados; os outros em faixas de 2 (a expectativa da diretoria e da imprensa [inferência])',
          "rule: the 12 entrants' predicted positions, by reputation: the knockout (stage 1) gives 1 to the winner and 2 to the runner-up, then one place to each round's losers; the rest in bands of 2 (the board and press expectation [inference])")])
CAMPOS = []
INTERESSA = []
