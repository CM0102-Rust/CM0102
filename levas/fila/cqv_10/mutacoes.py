# Mutacoes (cqv_10), no sorteio da Copa das Nacoes da Oceania (o dia do sorteio cai dentro da janela; os jogos sao
# depois de junho): sem o sorteio da Australia e da Nova Zelandia (um sorteio a menos); as melanesias embaralhadas com
# 1 escolha (menos trocas sorteadas); so 3 polinesias na contest (menos trocas sorteadas).
MUTACOES = [
    ('cqv10_sem_o_sorteio_da_australia',
     'if exe::utils_random(2) != 0 { // cqv10: the draw of Australia and New Zealand',
     'if true { // mutacao: sem o sorteio',
     'cqv_10 oceania cup draw'),
    ('cqv10_melanesias_com_1_escolha',
     '2); // cqv10: the Melanesian teams with 2 picks',
     '1); // mutacao: 1 escolha',
     'cqv_10 oceania cup draw'),
    ('cqv10_so_3_polinesias',
     'np < 10 {\n                // cqv10: up to 10 Polynesian teams',
     'np < 3 {\n                // mutacao: so 3',
     'cqv_10 oceania cup draw'),
]
