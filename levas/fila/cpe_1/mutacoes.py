# Mutacoes dos amistosos. Cada uma troca o sorteio que fica com o candidato (1 em 5 / meio a meio): outro adversario de
# amistoso (356 chamadas) ou de excursao (174) na carreira ate 2002-06-01, que deve acusar (amistosos diferentes mudam
# jogos, receitas e o sorteio dali em diante).
MUTACOES = [
    ('cpe_amistoso_sorteio', 'if friendly_random(5) == 1 { // o adversário do amistoso',
     'if friendly_random(5) == 2 { // mutacao: outro sorteio'),
    ('cpe_excursao_sorteio', 'if friendly_random(2) == 1 { // o adversário da excursão',
     'if friendly_random(2) == 0 { // mutacao: o outro lado do sorteio'),
]
