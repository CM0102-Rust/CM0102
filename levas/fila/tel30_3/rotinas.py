# The award finders of the award screens: award_screens.cpp (reader tel30, the screens round, 21/11/2026; a new group
# from the disassembly sweep: the community map gave them no file).
# Conventions checked in the disassembly: both thiscall(AWARD_MANAGER*, int comp) ret 4 -> eax; the manager's +8 points
# to 30 list pointers, each list +4 the item array and +8 the i16 count (read once into edx).
# Called by: 0x414c20 / 0x416960 (the staff competition) and 0x48fcd0 (the club competition, the competition screen).
# Identical in cm0102-1080.exe (the whole routines).
# Comparers: the value side by side (compare_value!, pure).
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('award_manager_find_award_by_staff_comp', 0x414a60, [0x51, 0x8b, 0x49, 0x08, 0x53]),
           ('award_manager_find_award_by_club_comp', 0x414ac0, [0x51, 0x8b, 0x49, 0x08, 0x53])]
_A = 'award_screens.cpp'
DOCS = ('as buscas de prêmio das telas de prêmios',
        'the award finders of the award screens',
        [('award_manager_find_award_by_staff_comp', 'AWARD_MANAGER::find_award_by_staff_comp', '0x414a60', _A, 'thiscall',
          'o primeiro prêmio, nas 30 listas do gerente, cuja competição de pessoas (+5) é a dada; 0 quando não há',
          "the first award, in the manager's 30 lists, whose staff competition (+5) is the given one; 0 when none"),
         ('award_manager_find_award_by_club_comp', 'AWARD_MANAGER::find_award_by_club_comp', '0x414ac0', _A, 'thiscall',
          'o primeiro prêmio cuja competição de clubes (+9) é a dada; 0 quando não há',
          'the first award whose club competition (+9) is the given one; 0 when none')])
