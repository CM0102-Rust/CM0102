# Mutacoes do clube rejeitado, do esfriar das relacoes e da lista das rejeitadas, todas no diario das pessoas. A do
# esfriar troca o 15 pelo 14 (o limite do sorteio muda em toda vaga ocupada: a sequencia de sorteios sai da referencia);
# a da lista copia as favoritas no lugar das rejeitadas: a carreira ate 2002-06-01 deve acusar as duas. A do clube
# rejeitado desloca a lista em uma vaga (o terceiro cai em +0x1c, a primeira pessoa favorita): so muda algo quando a
# 0x547e70 roda e acha vaga; a carreira PODE NAO acusar (o comparando acusa na primeira chamada).
MUTACOES = [
    ('dbq_clube_rejeitado', '    let lista = pref + 0x10; // os clubes rejeitados', '    let lista = pref + 0x14; // mutacao: uma vaga adiante'),
    ('dbq_esmaecer', '        if dbq2_u32(v) != 0 && sortear(*((pessoa + 0x59) as *const i8) as i32 + 15) == 0 { // as favoritas',
     '        if dbq2_u32(v) != 0 && sortear(*((pessoa + 0x59) as *const i8) as i32 + 14) == 0 { // mutacao: 14 no lugar do 15'),
    ('dbq_rejeitadas', '        let p = dbq2_u32(pref + 0x28 + 4 * k); // as rejeitadas', '        let p = dbq2_u32(pref + 0x1c + 4 * k); // mutacao: as favoritas'),
]
