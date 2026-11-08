# Mutacoes (cqc_6), nos jogos levados para a TV (0x685640), que a liga chama no dia dela (0x6847c0) e na montagem do
# calendario (0x683e30): 6533 chamadas na carreira ate 2002-06-01; nas datas da tabela da TV (as rodadas da Premier
# League) cada vaga leva o jogo de maior nota da rodada para o dia e a fatia da TV:
# - o bonus de 1500 no sorteio da nota vai ao mandante em vez do visitante: as notas mudam e outros jogos vao para a
#   TV (as datas, as fatias e a ordem das partidas mudam, e com elas o cansaco e os resultados);
# - o jogo vai para o dia seguinte ao da TV: as datas das partidas mudam;
# - o conflito na janela nao zera a nota: jogos de clubes com partida 2..5 dias antes ou depois tambem vao para a TV
#   (pega se na base algum jogo tinha conflito: contador "cqc_6 conflict in the window").
# Os contadores ficam colados no efeito: o jogo escolhido vai ao dia da TV depois de todas as condicoes; o conflito
# dentro do teste das duas janelas.
MUTACOES = [
    ('cqc6_bonus_ao_mandante',
     's = sorteio(cqc6_w(cqc6_d(f + 0x20) + 0x80) as i32 + 0x5dc); // cqc6: o visitante com o bônus',
     's = sorteio(cqc6_w(cqc6_d(f + 0x1c) + 0x80) as i32 + 0x5dc); // mutacao: o mandante com o bonus',
     'cqc_6 match for TV'),
    ('cqc6_tv_no_dia_seguinte',
     'cqc6_pw(r + 0x2a, cqc6_w(pa) as u16); // cqc6: o dia da TV',
     'cqc6_pw(r + 0x2a, cqc6_w(pa).wrapping_add(1) as u16); // mutacao: o dia seguinte ao da TV',
     'cqc_6 match for TV'),
    ('cqc6_conflito_nao_zera',
     's = 0; // cqc6: conflito na janela',
     '// mutacao: o conflito na janela nao zera a nota',
     'cqc_6 conflict in the window'),
]
