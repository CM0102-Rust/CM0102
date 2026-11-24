# Mutacoes (cx01_10), carreira de 8 ligas ate 2002-06-01. Nenhuma abre caixa.
# - a copa com importancia 13: o comparador do construtor acusa na criacao (e a importancia das partidas muda);
# - o Sao Raimundo na vaga 8 e o Bahia na 2: quem joga a preliminar (vagas 0 a 2) muda e o comparador acusa;
# - a posicao prevista do segundo inscrito 3 e nao 2: o comparador confere as nove contra a tabela da copia.
MUTACOES = [
    ('cx01_10_importancia_13',
     '    cup.set_discipline_rule(6);\n    cup.set_importance(0xc);',
     '    cup.set_discipline_rule(6);\n    cup.set_importance(0xd); // mutation'),
    ('cx01_10_sao_raimundo_bahia_trocados',
     '    (0x98a344, 2), // "São Raimundo Esporte Clube (AM)"',
     '    (0x98a344, 8), // mutation'),
    ('cx01_10_bahia_vaga_2',
     '    (0x98a294, 8), // "Esporte Clube Bahia"',
     '    (0x98a294, 2), // mutation'),
    ('cx01_10_prevista_segundo_3',
     'const BRA_CHAMP_CUP_PREDICTED: [u8; 9] = [1, 2, 3, 3, 5, 5, 7, 7, 9];',
     'const BRA_CHAMP_CUP_PREDICTED: [u8; 9] = [1, 3, 3, 3, 5, 5, 7, 7, 9]; // mutation'),
]
