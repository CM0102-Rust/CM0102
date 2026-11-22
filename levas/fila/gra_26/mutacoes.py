# Mutations (gra_26). The working day only does something on a type-4 list with a scout and an assignment, on the
# scouting end date. In the automatic ENG career to 2002-06-01 the routine is called 3220 times
# (docs/cobertura_simulacao.txt) but never reaches a report: removing the human searches did not change the printout
# (cqf_12, 02/11/2026), so these mutations ESCAPE the career (gap); the dry-run comparer catches them in the real game
# when a human's scout works (the next working day and the reports go into the object, the generator and the inbox it
# compares).
# - the next working day 7 days later (rand(7) + 9 instead of + 2): from the first day of every assignment;
# - how many to report before the last day doubled ((rand(50) + 150)% instead of 75..124%);
# - each report counts as two (fewer players reported per day).
MUTACOES = [
    ('gra26_proximo_dia_mais_7',
     '        let n = exe::utils_random(7) + 2; // gra_26 the next working day',
     '        let n = exe::utils_random(7) + 9; // gra_26 mutation: the next working day',
     'gra_26 first day'),
    ('gra26_quantos_hoje_dobro',
     '        (exe::utils_random(50) + 0x4b).wrapping_mul(count()) / (days * 100) // gra_26 how many today',
     '        (exe::utils_random(50) + 0x96).wrapping_mul(count()) / (days * 100) // gra_26 mutation: how many today',
     'gra_26 player report'),
    ('gra26_conta_dois',
     '    *left -= 1; // gra_26 one fewer to report',
     '    *left -= 2; // gra_26 mutation: one fewer to report',
     'gra_26 player report'),
]
