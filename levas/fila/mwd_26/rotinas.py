# The Greek and Polish super cup clubs (gre_super.cpp, pol_super.cpp). Reader mwd (12/11/2026), round 2 (2 calls each:
# the constructor and the turnover).
# - 0x5ec430 GreSuper::select_teams, 0x7ccd10 PolSuper::select_teams: the same 640-byte template (checked instruction by
#   instruction); only the three competitions (Greece: league, cup, league runner-up; Poland: league, cup, cup
#   runner-up) and the file/lines change. Not the sisters' template (FraSuper/JapSuper): here a missing winner is
#   replaced by the first club of the nation (club +0x53 = the competition's +0x5d).
# Conventions: thiscall, ret 0; 0x944e46 (cdecl), 0x4afd80 (2 pushes; add esp, 8 -> cdecl).
# Comparer: compare_team_list (side by side, the 12 bytes of the list). Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_super_select_teams', 0x5ec430, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_super_select_teams', 0x7ccd10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os clubes das supercopas da Grécia e da Polônia', 'the Greek and Polish super cup clubs',
        [('gre_super_select_teams', 'GreSuper::select_teams', '0x5ec430', 'gre_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Grécia é entre o campeão da Primeira Divisão e o da Copa da Grécia; faltando um, entra o primeiro clube da nação no banco; se o mesmo clube ganhou os dois, entra o vice da Primeira Divisão; sem ele, a caixa de erro',
          'rule: the Greek Super Cup is between the First Division champion and the Greek Cup winner; a missing one is replaced by the first club of the nation in the database; if the same club won both, the First Division runner-up comes in; if there is none, the error box'),
         ('pol_super_select_teams', 'PolSuper::select_teams', '0x7ccd10', 'pol_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Polônia é entre o campeão da Primeira Divisão e o da Copa da Polônia; faltando um, entra o primeiro clube da nação; se o mesmo clube ganhou os dois, entra o vice da Copa; sem ele, a caixa de erro',
          'rule: the Polish Super Cup is between the First Division champion and the Polish Cup winner; a missing one is replaced by the first club of the nation; if the same club won both, the Cup runner-up comes in; if there is none, the error box')])
CAMPOS = [('copa +4 (competição: +0x5d nação), +0x3e, +0xa7 (6 bytes: clube, classe, situação); clube +0 id, +0x53 nação', 'cup +4 (competition: +0x5d nation), +0x3e, +0xa7 (6 bytes: club, class, status); club +0 id, +0x53 nation')]
