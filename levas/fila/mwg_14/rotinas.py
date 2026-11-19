# A fase eliminatória da Copa do Mundo: passar a ela, montá-la e as honras da temporada (world_cup.cpp; leitor mwg,
# rodada 3 do mundo inteiro, 19/11/2026). A Copa de 2002 roda em qualquer carreira (as seleções estão no banco inteiro;
# o sorteio e os grupos já são Rust): advance_stage roda 1 vez até 2002-08-01 na passagem para as oitavas (a do grupo
# para a fase 7, meados de junho de 2002), create_knockout_stage 1 (as oitavas) e record_season_honours 1 (o fim da
# Copa) (cobertura_mundo.txt). LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x92d4b0..0x92d53f, 0x92d770..0x92da39, 0x92d310..0x92d32e): as três thiscall,
# ret 0;
# - advance_stage: +0x30 >= +0x2c - 1 (com sinal): a caixa da linha 0x345; senão +0x30 + 1 e, se virou 7, call direto
#   0x92d770 (ecx = this);
# - create_knockout_stage (quadro SEH só para o new da fase): RulingBody::reset_bookings_for_club(0) (0x7e9180, push 0;
#   ecx = [[[0xae28f0]] + 4 x byte com sinal +0x50] -> thiscall, ret 4); a lista de 16 (malloc 0x40; sem memória, a
#   caixa 0x3a0) por League::get_table_row (0x6826d0, push lugar; ecx = liga -> thiscall, ret 4; o clube em [linha]):
#   A1 B2 D1 C2 E1 F2 H1 G2 B1 A2 C1 D2 F1 E2 G1 H2 (A = a própria copa, B..H = as fases 0..6); a tabela de rodadas pela
#   virtual +0x3c (push 0; push &id; push &rodadas; push 7); a fase (operator new 0xb2; Cup_Stage::Cup_Stage 0x522e00,
#   ret 0x3c: copa, 0x10, lista, rodadas, [+4], tabela, ano, 7, 4, id, 0x14, 1, 0, 0, 0) em [+0xc][7]; free(lista) e
#   free(tabela); sem fase, a caixa 0x3c4; senão Cup::set_up_round(0) (0x51c800) na fase e +0x30 = 7;
# - record_season_honours: a virtual +0x30 da fase [+0xc][7] (sem argumentos; sem testar a fase) e
#   NationalTeams::clear_competition_for_comp_clubs([+4]) (0x775420, push; ecx = [0xb74318] -> thiscall, ret 4), cujo
#   eax devolve.
# Comparadores: advance_stage só no original (pode montar a fase; a fase fica ou sobe 1); create_knockout_stage só no
# original (aloca) com o plano dos 16 lugares lidos antes, como conjunto, a fase com 16 times e +0x30 = 7;
# record_season_honours só no original (vai para o histórico; a fase existia).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('world_cup_advance_stage', 0x92d4b0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('world_cup_create_knockout_stage', 0x92d770, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('world_cup_record_season_honours', 0x92d310, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x0c])]
DOCS = ('a fase eliminatória da Copa do Mundo: passar a ela, montá-la e as honras da temporada',
        "the World Cup's knockout stage: moving on to it, building it and its season honours",
        [('world_cup_advance_stage', 'WorldCup::advance_stage', '0x92d4b0', 'world_cup.cpp', 'thiscall, ret 0',
          'a Copa do Mundo passa à fase seguinte; ao entrar na fase 7 monta as oitavas; já na última fase é erro (a caixa da linha 0x345)',
          'the World Cup moves on to the next stage; entering stage 7 builds the round of 16; already at the last stage it is an error (the box at line 0x345)'),
         ('world_cup_create_knockout_stage', 'WorldCup::create_knockout_stage', '0x92d770', 'world_cup.cpp', 'thiscall, ret 0',
          'as oitavas da Copa do Mundo: os cartões zerados, os 16 classificados (1º A, 2º B, 1º D, 2º C, 1º E, 2º F, 1º H, 2º G, 1º B, 2º A, 1º C, 2º D, 1º F, 2º E, 1º G, 2º H) numa fase de copa de 16 times (a fase 7) com a primeira rodada montada na hora',
          'the World Cup round of 16: the bookings wiped, the 16 qualified (A1, B2, D1, C2, E1, F2, H1, G2, B1, A2, C1, D2, F1, E2, G1, H2) in a sixteen-team cup stage (stage 7) with its first round set up at once'),
         ('world_cup_record_season_honours', 'WorldCup::record_season_honours', '0x92d310', 'world_cup.cpp', 'thiscall, ret 0',
          'as honras da Copa do Mundo: as da fase eliminatória e depois as seleções deixam a competição',
          "the World Cup's honours: the knockout stage's, then the national teams leave the competition")])
