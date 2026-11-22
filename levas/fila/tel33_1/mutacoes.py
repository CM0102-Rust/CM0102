# Mutations (tel33_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the credits screen and go to page 1 (Additional Research).
# - one credit line swapped: the first name (Tony Persoglia) shows Jeremy Lowe's text, another text in the display list;
# - the heading text: "Additional Research" printed from another text, another heading (and 0xde1f64).
MUTACOES = [
    ('tel33_1_nome_trocado',
     '    0x9b4b0c, // Tony Persoglia',
     '    0x9b4afc, // Tony Persoglia (mutation)',
     'tel33_1 additional research rows'),
    ('tel33_1_titulo',
     '    comp_lang_format()(JOB_TEXT, 0x9b4b1cusize); // "Additional Research"',
     '    comp_lang_format()(JOB_TEXT, 0x9b4b0cusize); // mutation',
     'tel33_1 additional research rows'),
]
