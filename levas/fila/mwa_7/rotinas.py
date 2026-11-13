# The constructors of the Irish Challenge, Leinster and Munster cups (ire_chal_cup.cpp, ire_leinster_cup.cpp,
# ire_munster_cup.cpp, in cups\). Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to
# 2002-08-01: the constructors run once, when the career is created (2001), and build the season's cups.
# - 0x6340a0 (IreChalCup::IreChalCup), 0x6364a0 (IreLeinsterCup::IreLeinsterCup), 0x6387a0
#   (IreMunsterCup::IreMunsterCup): JapEmpCup::JapEmpCup's template (0x661d30, same.py: only the vtable, the
#   discipline rule, the entrant routine, the prize, the file and the lines change), in one body with the differences
#   as data (construct_irish_cup, also used by mwa_8).
# Conventions checked at the call sites: the constructors are thiscall, ret 8 (push record; push year; mov ecx, new;
# call, in the competitions set-up). Inside: Cup::Cup 0x518640 (mov ecx -> thiscall, ret 0), Cup::load_from_disk
# 0x51fc00 (push 1; mov ecx -> thiscall, ret 4), the entrant routine (mov ecx, esi -> thiscall, ret 0), the virtual
# +0x3c (push 0; push &+0x3a; push &+0x3c; push -1 -> thiscall, ret 0x10), Cup::map_fixture_tree 0x518790 (thiscall,
# ret 0), operator new 0x944cf1 (push 0x5ce; add esp, 4 -> cdecl), Comp_Stats::Comp_Stats 0x49ee70 (push this; mov
# ecx, new -> thiscall, ret 4), Cup::rank_teams 0x5223a0 (thiscall, ret 0), the error box.
# Comparers: only the original, with the plan (this, the vtable, +4, the year and the fixed fields, the round table
# and the statistics on a new cup).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ire_chal_cup', 0x6340a0, [0x6a, 0xff, 0x68, 0x39, 0xb3, 0x95, 0x00]),
           ('construct_ire_leinster_cup', 0x6364a0, [0x6a, 0xff, 0x68, 0x29, 0xb4, 0x95, 0x00]),
           ('construct_ire_munster_cup', 0x6387a0, [0x6a, 0xff, 0x68, 0x09, 0xb5, 0x95, 0x00])]
_REGRA = ('regra: o construtor da {pt}: copa de clubes de uma nação, do ano do jogo, com fundo de prêmios de 100.000, 5 reservas no banco e 3 substituições, regra de disciplina 16 (Irlanda); sem nada salvo, em ordem: {ins} (a caixa se devolver 0), as rodadas, a árvore dos confrontos (a caixa se devolver 0), as estatísticas e a classificação final',
          'rule: the constructor of the {en}: a cup of one nation\'s clubs of the game\'s year, with a 100,000 prize fund, 5 substitutes on the bench and 3 substitutions, discipline rule 16 (Ireland); with nothing saved, in order: {ens} (the error box if it returns 0), the rounds, the tie tree (the error box if it returns 0), the statistics and the final ranking')
DOCS = ('os construtores das copas da Irlanda, de Leinster e de Munster',
        'the constructors of the Irish Challenge, Leinster and Munster cups',
        [('construct_ire_chal_cup', 'IreChalCup::IreChalCup', '0x6340a0', 'ire_chal_cup.cpp', 'thiscall, ret 8',
          _REGRA[0].format(pt='Copa da Irlanda (FAI Senior Challenge Cup)', ins='os 42 inscritos'), _REGRA[1].format(en='FAI Senior Challenge Cup', ens='the 42 entrants')),
         ('construct_ire_leinster_cup', 'IreLeinsterCup::IreLeinsterCup', '0x6364a0', 'ire_leinster_cup.cpp', 'thiscall, ret 8',
          _REGRA[0].format(pt='Copa de Leinster', ins='os 16 clubes'), _REGRA[1].format(en='Leinster Senior Cup', ens='the 16 clubs')),
         ('construct_ire_munster_cup', 'IreMunsterCup::IreMunsterCup', '0x6387a0', 'ire_munster_cup.cpp', 'thiscall, ret 8',
          _REGRA[0].format(pt='Copa de Munster', ins='os 20 inscritos'), _REGRA[1].format(en='Munster Senior Cup', ens='the 20 entrants'))])
CAMPOS = []
INTERESSA = []
