# Mutacoes (sbH_1) pela carreira com a Argentina ate 2002-08-01 (a copa de acesso e as repescagens no fim da temporada
# argentina; quem sobe e quem cai muda a virada):
# - o lider da tabela da Segunda entra na copa de acesso: um dos oito muda, os jogos e quem sobe mudam (o comparador
#   da copa tambem acusa);
# - a primeira repescagem com o 16o dos promedios da Primeira no lugar do 18o: o rival e quem cai mudam;
# - so um rebaixado pelos promedios: um clube a menos cai.
MUTACOES = [
    ('sbH_1_lider_na_copa',
     '        if id == leader || clubs.iter().any(|&c| c != 0 && read_i32(c) == id) {',
     '        if clubs.iter().any(|&c| c != 0 && read_i32(c) == id) { // mutation',
     'sbH_1 argentine second division promotion cup'),
    ('sbH_1_repescagem_com_o_16o',
     '    write_usize(pair, premier_average_club(17));',
     '    write_usize(pair, premier_average_club(15)); // mutation',
     'sbH_1 argentine second division promotion play-offs'),
    ('sbH_1_um_rebaixado',
     '    while k < l.club_count() && relegated < 2 {',
     '    while k < l.club_count() && relegated < 1 { // mutation',
     'sbH_1 argentine second division relegated by averages'),
]
