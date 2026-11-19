# The Korean Super Cup (kor_super_cup.cpp; with its turnover, already in Rust, the file is whole). Reader mwf (19/11/2026),
# whole-world round, continuation: the constructor runs once, at career creation; the round table and the clubs at
# creation and at the Korean turnover (docs/cobertura_mundo.txt).
# - 0x66d820 KorSuperCup::KorSuperCup: construct_nation_cup (mwc_16/17), checked instruction by instruction against
#   FraSuper/HolSuper: only the bench (5, 3) written, discipline rule 42, vtable, the clubs routine, file and lines change;
#   entrants and tie tree fail on a negative value (jge); no prize fund, no predicted positions.
# - 0x66dad0 KorSuperCup::create_round_table: one round (the generator from the disassembly); the original also writes
#   +0x11 = 0 (not compared by compare_cup_round_table, which skips +0x11..+0x12; written here too).
# - 0x66dc10 KorSuperCup::create_table: the GreSuper/PolSuper body (select_league_and_cup_winners, mwd_26), equal
#   instruction by instruction but the keys (K League, FA Cup, FA Cup runner-up), the file and the lines.
# Conventions: thiscall; ctor ret 8; round table ret 0x10 (0x521e60 and 0x521eb0 cdecl); create_table ret 0
# (0x944e46 cdecl; comp_util_get_latest_placed_club 2 pushes, add esp, 8 -> cdecl, as mwd_26).
# Comparers: the constructor only the original (compare_nation_cup_ctor); the round side by side
# (compare_cup_round_table); the clubs side by side (compare_team_list).
# Depends on: NationCup / construct_nation_cup / compare_nation_cup_ctor (batch 300), select_league_and_cup_winners
# (batch 302), compare_team_list (spa_super.rs), FraSuperCupRound / cql12r / fra_super_create_single_stage_round_table
# (fra_super.rs), compare_cup_round_table (jap_j_cup.rs).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_kor_super_cup', 0x66d820, [0x6a, 0xff, 0x68, 0xb9, 0xc7, 0x95, 0x00]),
           ('kor_super_cup_create_round_table', 0x66dad0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_super_cup_create_table', 0x66dc10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Supercopa da Coreia', 'the Korean Super Cup',
        [('construct_kor_super_cup', 'KorSuperCup::KorSuperCup', '0x66d820', 'kor_super_cup.cpp', 'thiscall, ret 8',
          'regra: a Supercopa da Coreia nasce com a regra de disciplina 42 e o banco (5 reservas, 3 substituições); sem estado gravado, os dois clubes, a tabela de rodadas, a árvore de jogos e as estatísticas, sem posições previstas',
          'rule: the Korean Super Cup is born with discipline rule 42 and the bench (5 substitutes, 3 substitutions); with no saved state, its two clubs, the round table, the tie tree and the statistics, no predicted positions'),
         ('kor_super_cup_create_round_table', 'KorSuperCup::create_round_table', '0x66dad0', 'kor_super_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Coreia é um jogo único entre os dois clubes, sorteado em 1/2 e jogado em 10/3 (levado ao dia da semana), categoria 6, sem prêmio',
          'rule: the Korean Super Cup is a single match between its two clubs, drawn on 1/2 and played on 10/3 (moved to the weekday), category 6, no prize'),
         ('kor_super_cup_create_table', 'KorSuperCup::create_table', '0x66dc10', 'kor_super_cup.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Coreia é entre o campeão da K League e o da Copa da Coreia; faltando um, entra o primeiro clube da nação; se o mesmo clube ganhou os dois, entra o vice da Copa; sem ele, a caixa de erro',
          'rule: the Korean Super Cup is between the K League champion and the Korean FA Cup winner; a missing one is replaced by the first club of the nation; if the same club won both, the FA Cup runner-up comes in; without one, the error box')])
CAMPOS = [('copa +0x49, +0x4a (banco), +0x50 (disciplina), +0x3e e +0xa7 (os dois clubes), rodada +0x11',
           'cup +0x49, +0x4a (bench), +0x50 (discipline), +0x3e and +0xa7 (the two clubs), round +0x11')]
