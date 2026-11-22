# Mutations (tel08_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the Contract tab of a person with a contract and a note.
# - the notes button id 0x2c -> 0x2d: another item id in the display list, caught by the comparer;
# - the work permit share computed with 99 instead of 100: another text in the display list (a person with a permit).
MUTACOES = [
    ('tel08_1_botao_notas',
     '    staff_tab_button(area, 1, row, 2, 0, 1, 2, VALUE, JOB_TEXT, 0x2c, 0); // tel08_1 the notes button',
     '    staff_tab_button(area, 1, row, 2, 0, 1, 2, VALUE, JOB_TEXT, 0x2d, 0); // tel08_1 mutation',
     'tel08_1 contract tab'),
    ('tel08_1_visto_99',
     '    let share = if total != 0 { ((played as i32 * 100) / total) as u8 as i8 } else { 0 };',
     '    let share = if total != 0 { ((played as i32 * 99) / total) as u8 as i8 } else { 0 };',
     'tel08_1 contract tab work permit'),
]
