# Mutacoes (sx19_8), pela carreira ENG POL TUR GRE HRV RUS ate 2002-08-01 (liga polonesa):
# - a Polonia suspendendo no 3o amarelo em vez do 4o;
# - a escala do orgao russo 8 -> 10 (acusada pelo comparador do construtor; na carreira, muda a duracao das
#   suspensoes por pontos dos vermelhos da liga russa).
MUTACOES = [
    ('sx19_8_polonia_terceiro',
     'if count == 4 || count == 7 { // sx19_8: the Polish booking bans',
     'if count == 3 || count == 7 { // mutation: the third instead of the fourth',
     'sx19_8 polish booking ban'),
    ('sx19_8_escala_russia',
     'rule_set: 0x27, scale: 8,',
     'rule_set: 0x27, scale: 0xa, // mutation\n   ',
     'sx19_8 russian body built'),
]
