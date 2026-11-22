# Game Credits and Web Sites rows (tel34_1). The automatic career opens no screen: the chain runs none; these prove the
# comparer in a session of Lucas's (JOGAR_COMPARANDO.bat): menu bar > Credits (the credits_screen_build and section
# comparers catch both).
# - the right column of the Additional Testing names shifted;
# - the View popup entries without the page in their value (page 1 sections would open on page 0).
MUTACOES = [
    ('tel34_1_right_column',
     'tel34_credits_cell(0x1c2, 0x2f8, row, JOB_COLOUR_3184, name);',
     'tel34_credits_cell(0x1c8, 0x2f8, row, JOB_COLOUR_3184, name);'),
    ('tel34_1_popup_page',
     '(2, 0x64, (read_u16(CREDITS_PAGE) as u32) << 16 | read_u16(CREDITS_LINE) as u32)',
     '(2, 0x64, read_u16(CREDITS_LINE) as u32)'),
]
