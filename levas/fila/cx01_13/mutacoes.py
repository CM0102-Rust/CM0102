# Mutacoes (cx01_13), carreira de 8 ligas ate 2002-06-01. Nenhuma abre caixa.
# - o classificado dos grupos com peso 100 e nao 40: so a original roda no comparador; a carreira Rust muda o historico
#   e a reputacao dos tecnicos (os registros entram no banco);
# - o 3 da preliminar vira 8 e nao 9: o comparador acusa (os 3 bytes da entrada);
# - as manchetes de quem passa trocadas: a comparacao com CM_TEXTOS=1 acusa.
MUTACOES = [
    ('cx01_13_peso_100',
     '                european_cup_add_qualified_record(club, comp, 0, round, 0x28);',
     '                european_cup_add_qualified_record(club, comp, 0, round, 0x64); // mutation'),
    ('cx01_13_preliminar_3_vira_8',
     '        (true, 3) => 9,',
     '        (true, 3) => 8, // mutation'),
    ('cx01_13_manchetes_trocadas',
     '                lang_sprintf(0xde1f64, if s == 0xff { 0x98a25c } else { 0x98a224 }, cg, cg, team);',
     '                lang_sprintf(0xde1f64, if s == 0xff { 0x98a224 } else { 0x98a25c }, cg, cg, team); // mutation'),
]
