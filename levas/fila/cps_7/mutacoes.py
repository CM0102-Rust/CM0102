# Mutacoes (cps_7). O estadio nacional da Inglaterra (Wembley, fechado de 2000 a 2007) fecha no primeiro dia da
# carreira (0x85b820, uma vez). A selecao inglesa fica com o estadio: os jogos em casa marcados depois nao passam pela
# busca do estadio neutro (0x85c800, que sorteia 100 por candidato em 0x85da00): um sorteio a menos por candidato. O id
# do estadio fechado nao e guardado ([0xa835a4] fica -1): as finais com a marca 4 (copas inglesas) deixam de ir direto ao
# estadio da nacao [0x9cf510] e passam pela busca com sorteios, e a busca deixa de pular Wembley. A reserva invertida: a
# busca neutra que confere as reservas (0x85c800, marcas fora de 0x20) recusa os livres e aceita os reservados.
MUTACOES = [
    ('cps7_selecao_com_casa', '            cps7_pd(nation_national_team(england_nation()) + 0x69, 0); // cps7: a seleção sem casa',
     '            let _ = nation_national_team(england_nation()); // mutacao: a seleção fica com o estádio',
     'cps_7 fecha o estadio nacional'),
    ('cps7_sem_o_id', '            cps7_pd(0xa835a4, cps7_d(e) as u32); // cps7: guarda o id do estádio fechado',
     '            // mutacao: o id do estádio fechado não é guardado',
     'cps_7 fecha o estadio nacional'),
    ('cps7_livre_invertido', '    (r == 0) as u32 // cps7: livre sem a reserva',
     '    (r != 0) as u32 // mutacao',
     'cps_7 reserva conferida'),
]
