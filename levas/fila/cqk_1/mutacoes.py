# Mutações (cqk_1), nas janelas das três ligas da carreira (a carreira começa em julho de 2001, com as janelas abertas):
# - a Inglaterra abre em agosto em vez de junho: os clubes ingleses não contratam em julho (contador "cqk_1 england");
# - os EUA fecham em julho em vez de agosto (contador "cqk_1 usa");
# - a Austrália abre em agosto em vez de junho (contador "cqk_1 australia").
MUTACOES = [
    ('cqk1_inglaterra_abre_em_agosto', '[0x07, 0x00, 0x01, 0x02, 0x05, 0x01], // cqk1: a Inglaterra abre em junho', '[0x07, 0x00, 0x01, 0x02, 0x07, 0x01], // mutacao: agosto', 'cqk_1 england'),
    ('cqk1_eua_fecham_em_julho', '[0x19, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqk1: os EUA fecham em agosto', '[0x19, 0x00, 0xff, 0x0f, 0x06, 0x00], // mutacao: julho', 'cqk_1 usa'),
    ('cqk1_australia_abre_em_agosto', '[0x02, 0x00, 0x06, 0x0b, 0x05, 0x01], // cqk1: a Austrália abre em junho', '[0x02, 0x00, 0x06, 0x0b, 0x07, 0x01], // mutacao: agosto', 'cqk_1 australia'),
]
