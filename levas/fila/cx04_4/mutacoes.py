# Mutacoes (cx04_4). O texto nao volta para a simulacao: acusada pela comparacao do escritor (compare_text_writer) com o
# modo CM_TEXTO_NOTICIAS na carreira automatica (os clubes ingleses na Intertoto, julho de 2001):
# - o clube que passa da Intertoto ganha o texto "entra na Copa da UEFA" (0x9c49c0) no lugar de "classifica-se".
MUTACOES = [
    ('cx04_4_intertoto_outro_texto',
     '    european_cup_fate_text(text, club_id, 0x9e9e54, read_u8(uefa + 0x52), uefa + 0x38, uefa);',
     '    european_cup_fate_text(text, club_id, 0x9c49c0, read_u8(uefa + 0x52), uefa + 0x38, uefa); // mutation',
     'cx04_4 intertoto qualify text'),
]
