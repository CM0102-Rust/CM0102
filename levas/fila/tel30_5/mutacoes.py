# Mutations (tel30_5). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): the club screen > Squad, a click on a column header sorts the list.
# - the European registration ranks 1 and 2 swapped (the "Euro" column);
# - the form difference reversed (the "Form" column).
MUTACOES = [
    ('tel30_5_euro_rank', '        1 => 4,\n        2 => 3,', '        1 => 3,\n        2 => 4,'),
    ('tel30_5_form_order', 'if fb - fa != 0 {\n        return fb - fa;', 'if fb - fa != 0 {\n        return fa - fb;'),
]
