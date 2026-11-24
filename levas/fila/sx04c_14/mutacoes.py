# Mutacoes (sx04c_14), pela carreira com 'ENG SCO WAL' ate 2002-06-01 (os premios do sabado perto de 23 de maio de
# 2002 e os premios do mes de agosto a maio).
# - o jovem do ano sem o limite de idade;
# - os premios do mes so a partir de setembro;
# - os premios em 23 de junho (fora da janela).
MUTACOES = [
    ('sx04c_14_jovem_sem_idade',
     '(2, 0x15, 0x11), (1, -1, 2)] {',
     '(2, -1, 0x11), (1, -1, 2)] { // mutation',
     'sx04c_14 scottish awards day'),
    ('sx04c_14_setembro',
     'comp, 7, y, 4, y + 1, f, players]);',
     'comp, 8, y, 4, y + 1, f, players]); // mutation',
     'sx04c_14 scottish awards built'),
    ('sx04c_14_junho',
     'make_date_dword(0x17, 4, read_u16(today + cm_date::YEAR) as u32, 6)',
     'make_date_dword(0x17, 5, read_u16(today + cm_date::YEAR) as u32, 6) /* mutation */'),
]
