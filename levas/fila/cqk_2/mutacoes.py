# Mutações (cqk_2), nas janelas das três ligas da carreira (a carreira começa em julho de 2001):
# - a Espanha fecha a primeira janela em julho em vez de agosto (contador "cqk_2 espanha");
# - a Itália fecha a primeira janela em agosto em vez de setembro (contador "cqk_2 italia");
# - a França fecha a primeira janela em julho em vez de agosto (contador "cqk_2 franca").
MUTACOES = [
    ('cqk2_espanha_fecha_em_julho', '[0x16, 0x00, 0x04, 0x14, 0x07, 0x00], // cqk2: a Espanha fecha em agosto', '[0x16, 0x00, 0x04, 0x14, 0x06, 0x00], // mutacao: julho', 'cqk_2 espanha'),
    ('cqk2_italia_fecha_em_agosto', '[0x0e, 0x00, 0x04, 0x1b, 0x09, 0x00], // cqk2: a Itália fecha em setembro', '[0x0e, 0x00, 0x04, 0x1b, 0x07, 0x00], // mutacao: agosto', 'cqk_2 italia'),
    ('cqk2_franca_fecha_em_julho', '[0x09, 0x00, 0x03, 0x1f, 0x07, 0x00], // cqk2: a França fecha em agosto', '[0x09, 0x00, 0x03, 0x1f, 0x06, 0x00], // mutacao: julho', 'cqk_2 franca'),
]
