# A virada de temporada das três copas de mata-mata (sco_chal_cup.cpp, sco_fa_cup.cpp, wel_cup.cpp): a Copa Challenge,
# a Copa da Escócia e a Copa de Gales. Leitor mwb (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x7ed130, 0x7ede10, 0x924c50: o mesmo molde da virada da Copa do Brasil (start_new_cup_season, já em Rust),
#   conferido instrução por instrução; só mudam a rotina dos inscritos, o arquivo e as linhas das caixas.
# Rodam uma vez, na virada escocesa e galesa do meio de 2002 (a carreira começa em 2001-07). A Copa Challenge joga a 1ª
# rodada em 10/7: a virada mexe na carreira antes de 2002-08-01; as outras duas só começam em outubro/novembro (a
# mutação delas só o comparador acusa).
# Convenções conferidas: thiscall, ret 0; os inscritos mov ecx -> thiscall, ret 0; as virtuais +0x8c, +0x94, +0x5c
# thiscall, ret 0; 0x4a1c50 push 1; mov ecx -> thiscall, ret 4.
# Comparadores: só a original (libera e realoca), com o plano: o ano subiu um.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_chal_cup_start_new_season', 0x7ed130, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_fa_cup_start_new_season', 0x7ede10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_cup_start_new_season', 0x924c50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada da Copa Challenge, da Copa da Escócia e da Copa de Gales',
        'the turnover of the Challenge Cup, the Scottish Cup and the Welsh Cup',
        [('sco_chal_cup_start_new_season', 'ScoChalCup::start_new_season', '0x7ed130', 'sco_chal_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa Challenge é refeita do zero: libera as tabelas da edição passada, arquiva a temporada nas estatísticas, passa ao ano seguinte e refaz os inscritos, as rodadas, a árvore de confrontos e as posições previstas',
          "rule: at the turnover the Challenge Cup is rebuilt from scratch: last edition's tables are freed, the season is archived in the statistics, the year moves on and the entrants, the rounds, the fixture tree and the predicted positions are rebuilt"),
         ('sco_fa_cup_start_new_season', 'ScoFaCup::start_new_season', '0x7ede10', 'sco_fa_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa da Escócia, igual à da Copa Challenge',
          "rule: the Scottish Cup turnover, the same as the Challenge Cup's"),
         ('wel_cup_start_new_season', 'WelCup::start_new_season', '0x924c50', 'wel_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa de Gales, igual à da Copa Challenge',
          "rule: the Welsh Cup turnover, the same as the Challenge Cup's")])
