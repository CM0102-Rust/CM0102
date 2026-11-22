# The club history: a finished competition's season (club_history.cpp; reader tel42, screens round 2, 22/11/2026).
# Self-contained (its own club_history_season constants): no order with tel42_1 (save) or tel42_3 (screens).
# 0x442e80 Club_History::record_comp_season(comp): thiscall, ret 4, returns 1 or 0; no SEH, no random draw; no jump
# table. Called by Cup::update_for_date (0x51f917) and League::update_for_date (0x6849fa), both already in Rust
# (exe::club_history_record_comp_season on G_CLUB_HISTORY), once per competition when its season closes
# (cup::SEASON_CLOSED / league::SEASON_CLOSED still 0). World coverage to 2002-08-01: 178 calls; ENG to 2002-06-01:
# every competition that finishes in the window [inference: from August 2001, the Charity Shield, the Super Cup and the
# Intertoto; the League Cup in February; the leagues, the FA Cup and the European cups in May 2002]. Same in the
# cm0102-1080 exe.
# Calls (checked at the call sites): Friendly::is_friendly_competition 0x5c6710 thiscall(G_FRIENDLY, TComp) ret 4;
# comp_util_is_league_division 0x4b0700 cdecl(id); League::sort_table 0x6827d0 thiscall(stage, 0) ret 4;
# Comp::get_stage 0x48d200 thiscall(comp, index) ret 4; FILE_LLIST::add_node 0x58e340 thiscall(list, &first, &last,
# record) ret 0xc; calloc 0x945501 cdecl(count, 4); free 0x9452ca; the error box (show_error_at).
# The records: league 0x15 bytes (+0 comp id, +4 year, +6 position, +7..+0x14 the row's +0xd..+0x1a: 4 bytes before
# played [inference: unused totals], played, won, drawn, lost, goals for, goals against, points); cup 0xb bytes (+0 comp
# id, +4 year, +6 status/outcome, +7 stage label, +9 round code or -1).
# Comparer: the copy as a dry run (the appends recorded, the tables sorted as the original does), then the original;
# the newest nodes of each planned club read back from the history file (FILE_LLIST::get_nodes) must be the planned
# records. The extra sort is idempotent [inference: League::sort_table is a total order ending on the club id and
# stores the positions in its own order].
# Mutations: the career does not read the club history (only the screens and the save), so the Rust careers do not
# see them; the comparer does, in the comparison run to the date (mutacoes.py --carreira ... --confirmar).
# Defects of the original kept: in the Polish first division only stage 1 is recorded, with 14 added to the games
# played (the first stage's games, fixed in the code); a league stage with 7 rounds or fewer is not recorded (a
# short league or play-off table gets no season).
ROTINAS = [('club_history_record_comp_season', 0x442e80, [0x81, 0xec, 0x3c, 0x02, 0x00, 0x00])]
DOCS = ('o histórico dos clubes: a temporada de uma competição encerrada',
        'the club history: a finished competition\'s season',
        [('club_history_record_comp_season', 'Club_History::record_comp_season', '0x442e80', 'club_history.cpp', 'thiscall, ret 4',
          'grava a temporada encerrada no histórico de cada clube: numa liga, cada fase-liga com mais de 7 rodadas (na primeira divisão polonesa só a fase 1, com 14 jogos a mais) é ordenada e cada linha vira um registro (competição, ano, posição, jogos, vitórias, empates, derrotas, gols e pontos); numa copa, da fase atual para trás, cada clube fica com o registro mais adiantado (situação, fase e rodada alcançada); amistoso não grava',
          'writes the closed season into each club\'s history: in a league, every league stage with more than 7 rounds (in the Polish first division only stage 1, with 14 more games) is sorted and each row becomes a record (competition, year, position, played, won, drawn, lost, goals and points); in a cup, from the current stage back, each club keeps its furthest record (status, stage and round reached); friendlies are not recorded')])
CAMPOS = [('Club_History +0/+4 listas em arquivo (FILE_LLIST::add_node) e +8 registros por clube (+0/+4 liga, +8/+0xc copa); ponteiro; acrescenta e move os deslocamentos (club_history_record_comp_season)',
           'Club_History +0/+4 file lists (FILE_LLIST::add_node) and +8 per-club records (+0/+4 league, +8/+0xc cup); pointer; appends and moves the offsets (club_history_record_comp_season)')]
INTERESSA = ['SGE', 'CMRTE']
