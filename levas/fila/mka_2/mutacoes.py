# Mutacoes (mka_2), cada uma com o ramo:
# - o piso do teto do multiplicador: 1 no lugar de 4,25 (Q fica <= 1 quase sempre: o premio some e o preco cai ao
#   valor; outros lances aceitos, outras contratacoes da IA);
# - a volta ao valor sem entrada de lista: 0,33 valor + 0,67 preco no lugar de 0,67 valor + 0,33 preco (precos mais
#   altos fora das grandes nacoes: lances recusados);
# - a moral sem queixas: x 1 no lugar de x 1,2 (premio menor);
# - os Jovens do tecnico do vendedor: + 10 no lugar de - 10 (o piso do jovem sobe: precos dos jovens promissores).
MUTACOES = [
    ('mka2_teto_minimo',
     '        cap = read_f64(0x969008);',
     '        cap = read_f64(rdata::F64_1); // mutation: floor 1 instead of 4.25',
     'mka_2 premium'),
    ('mka2_volta_ao_valor',
     'ftol(value as f64 * read_f64(rdata::F64_0_67) + price as f64 * read_f64(rdata::F64_0_33))',
     'ftol(value as f64 * read_f64(rdata::F64_0_33) + price as f64 * read_f64(rdata::F64_0_67)) /* mutation: weights swapped */',
     'mka_2 pull to value'),
    ('mka2_moral',
     'morale_factor *= if contract.grievances() == 0 { read_f64(rdata::F64_1_2) }',
     'morale_factor *= if contract.grievances() == 0 { read_f64(rdata::F64_1) /* mutation: 1 instead of 1.2 */ }',
     'mka_2 premium'),
    ('mka2_jovens',
     'si = si.wrapping_add(manager.non_player().youngsters() as i16 - 10);',
     'si = si.wrapping_add(manager.non_player().youngsters() as i16 + 10); // mutation: Youngsters + 10',
     'mka_2 manager youngsters'),
]
