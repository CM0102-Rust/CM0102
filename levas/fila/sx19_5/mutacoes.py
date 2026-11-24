# Mutacoes (sx19_5), pela carreira ENG SWE FIN NOR POR ate 2002-08-01 (a Tippeligaen joga de abril a outubro):
# - a liga norueguesa suspendendo tambem no quarto amarelo;
# - a escala do orgao portugues 8 -> 10 (acusada pelo comparador do construtor; na carreira, muda a duracao das
#   suspensoes por pontos dos vermelhos da liga portuguesa).
MUTACOES = [
    ('sx19_5_quarto_amarelo',
     '3 => (1, 3), // sx19_5: the third Norwegian booking',
     '3 | 4 => (1, 3), // mutation: the fourth bans too',
     'sx19_5 norwegian ban'),
    ('sx19_5_escala_portugal',
     'rule_set: 0x17, scale: 8,',
     'rule_set: 0x17, scale: 0xa, // mutation\n   ',
     'sx19_5 portuguese body built'),
]
