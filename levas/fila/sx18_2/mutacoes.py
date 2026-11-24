# Mutacoes (sx18_2). Os construtores rodam uma vez, na criacao do jogo; o comparador de cada um confere os campos que a
# original grava contra a classe da copia, e acusa uma constante trocada (trocar a vaga ou a regra na carreira poria
# dois orgaos na mesma vaga do registro: evitado).
# - a escala europeia 10 -> 11 (comparador);
# - a escala internacional 10 -> 9 (comparador);
# - a regra asiatica 0x29 -> 0x28 na confederacao (comparador).
MUTACOES = [
    ('sx18_2_escala_europeia',
     'rule_set: 1, scale: 0xa,',
     'rule_set: 1, scale: 0xb, // mutation\n   ',
     'sx18_2 european ruling body built'),
    ('sx18_2_escala_internacional',
     'record_slot: 3, rule_set: 0, scale: 0xa,',
     'record_slot: 3, rule_set: 0, scale: 0x9, // mutation\n   ',
     'sx18_2 international ruling body built'),
    ('sx18_2_confederacao_asiatica',
     'league_rule: 0xff, cup_rule: 0xff, continental_rule: 0x29 };',
     'league_rule: 0xff, cup_rule: 0xff, continental_rule: 0x28 }; // mutation',
     'sx18_2 asian ruling body built'),
]
