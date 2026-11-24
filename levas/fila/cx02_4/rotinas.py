# As rodadas da Copa da Liga inglesa e da Supercopa (Charity Shield) e o progresso do Troféu das Divisões Inferiores
# (fra_cup.cpp, eng_fa_trophy.cpp, eng_auto_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS).
# - 0x5bb000 FraCup::set_up_rounds thiscall() ret 0 -> 1: a virtual 0x8c (SET_UP_RULES) da Copa da França e de mais seis
#   copas (vtables 0x969934 EngCcCup, 0x96b044 GerCup, ScoFaCup, ScoLgeCup, SpaCup, UsaOpenCup); na carreira ENG roda pela
#   Copa da Liga inglesa (o construtor e a virada a chamam: eng_cc_cup.rs). O premio (+0x28 = 1.000.000) antes da virtual
#   0x3c (push 0; push &+0x3a; push &+0x3c; push -1; mov ecx, this -> thiscall, ret 0x10), o retorno em +0xa3.
# - 0x571650 EngFaTrophy::set_up_rounds: o mesmo com 300.000; a virtual 0x8c do FA Trophy e da Charity Shield (vtable
#   0x9699d4: roda pela Charity Shield na carreira ENG).
#   Prologo das duas: push esi; mov esi, ecx; push 0 (5 bytes).
# - 0x56c240 EngAutoCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (a virtual 0x68, chamada
#   por Cup::record_team_progress e pelas tabelas das secoes): Comp_Stats::find_club_entry 0x4a4850 (push clube; mov ecx,
#   [this+8] -> thiscall, ret 4); fase (byte com sinal) < 1: cada codigo n vira 2n - 1 (byte); senao como vem.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: as rodadas lado a lado (premio, rotulo e numero de rodadas, banco, retorno e os bytes da tabela; a
# tabela da copia e liberada); o progresso lado a lado nos tres bytes da entrada.
SAVE = False
ROTINAS = [('fra_cup_set_up_rounds', 0x5bb000, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('eng_fa_trophy_set_up_rounds', 0x571650, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('eng_auto_cup_set_club_progress', 0x56c240, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as rodadas da Copa da Liga inglesa e da Supercopa e o progresso do Troféu das Divisões Inferiores',
        'the round set-up of the League Cup and the Charity Shield, and the Auto Windscreens Shield progress',
        [('fra_cup_set_up_rounds', 'FraCup::set_up_rounds', '0x5bb000', 'fra_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da França, a Copa da Liga inglesa, a Copa da Alemanha, as copas da Escócia, a Copa do Rei e a US Open Cup têm um prêmio total de 1.000.000; monta a tabela de rodadas da copa',
          'rule: the Coupe de France, the English League Cup, the German Cup, the Scottish cups, the Spanish Cup and the US Open Cup have a 1,000,000 prize fund; builds the cup\'s round table'),
         ('eng_fa_trophy_set_up_rounds', 'EngFaTrophy::set_up_rounds', '0x571650', 'eng_fa_trophy.cpp', 'thiscall, ret 0',
          'regra: o FA Trophy e a Supercopa inglesa (Charity Shield) têm um prêmio total de 300.000; monta a tabela de rodadas',
          'rule: the FA Trophy and the Charity Shield have a 300,000 prize fund; builds the round table'),
         ('eng_auto_cup_set_club_progress', 'EngAutoCup::set_club_progress', '0x56c240', 'eng_auto_cup.cpp', 'thiscall, ret 0x14',
          'regra: no Troféu das Divisões Inferiores, a posição de um clube nas seções (norte e sul) vira, nas estatísticas da copa, 2n − 1 (a escala das duas seções juntas [inferência]); na final, como vem',
          'rule: in the Auto Windscreens Shield a club\'s place in the sections (north and south) becomes 2n − 1 in the cup statistics (the scale of both sections together [inference]); in the final, as it comes')])
CAMPOS = []
INTERESSA = []
