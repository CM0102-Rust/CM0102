# Mutacoes (rx13_17), pela carreira ENG SWE FIN NOR POR com a base NORMAL ate 2002-08-01 (a virada portuguesa de 2002
# move os clubes e refaz as zonas):
# - um clube a menos sobe para a Segunda B;
# - as zonas invertidas (os 20 do sul na zona Norte).
MUTACOES = [
    ('rx13_17_um_a_menos',
     'for k in 0..relegated.max(0) as usize {',
     'for k in 1..relegated.max(0) as usize { // mutation: one fewer',
     'rx13_17 second division b exchange'),
    ('rx13_17_zonas_invertidas',
     'let zone = if k < 0x14 { south } else if k < 0x28 { central } else { north }; // rx13_17: south to north',
     'let zone = if k < 0x14 { north } else if k < 0x28 { central } else { south }; // mutation',
     'rx13_17 second division b zones'),
]
