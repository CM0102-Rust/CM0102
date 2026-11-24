# Mutacoes (sx21b_2). O lance roda nas ofertas da carreira ENG; caem pelo comparador lado a lado (a oferta, *out e o
# sorteio) e pela carreira (outra taxa ou outras clausulas mudam o andamento das ofertas).
MUTACOES = [
    ('sx21b_2_sempre_sabe',
     '    (exe::utils_random(chance) == 0) as u8 // sx21b_2: the player hears of the bid',
     '    (exe::utils_random(chance) != 0) as u8 // mutation: almost always',
     'sx21b_2 the player may hear of the bid'),
    ('sx21b_2_23_parcelas',
     '(0x18, rdata::F64_1_666666667) // sx21b_2: 24 instalments',
     '(0x17, rdata::F64_1_666666667) // mutation: 23',
     'sx21b_2 24 instalments'),
    ('sx21b_2_britanico_1',
     '{ 0.75 } else { 2.0 }; // sx21b_2: British buyers spread less',
     '{ 1.0 } else { 2.0 }; // mutation: 1.0',
     'sx21b_2 big fee spread'),
]
