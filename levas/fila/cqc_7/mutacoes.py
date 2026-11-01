# Mutacoes (cqc_7), nos jogos da rodada da copa levados as datas marcadas (0x51ebe0), que a copia Rust da montagem
# da rodada (0x51c800, "cps_13 jogos da rodada" na base) chama a cada rodada montada: 68 chamadas na carreira ate
# 2002-06-01. Nas rodadas com datas marcadas (bit 0x100 em +0xd: as rodadas com dia da semana e fatia por vaga, como
# as das copas europeias), cada jogo vai ao dia da semana e a fatia da vaga dele:
# - o jogo vai ao dia seguinte ao marcado: as datas das partidas mudam (o calendario dos clubes, o cansaco e os
#   conflitos com a liga);
# - o jogo vai a outra fatia (1 <-> 2): a hora da partida no dia muda, e com ela a ordem das partidas do dia;
# - o jogo vai ao dia da semana seguinte ao da vaga: a data muda pelo ajuste do dia da semana (0x54a2d0).
# O contador fica colado no efeito: o jogo sai do calendario e ganha a data nova depois de todas as condicoes ("cqc_7
# jogo levado a data"); se nenhuma rodada de datas marcadas rodar antes de 2002-06-01, as tres escapam juntas.
MUTACOES = [
    ('cqc7_dia_seguinte_ao_marcado',
     'cqc7_pw(r + 0x2a, cqc7_w(pt) as u16); // cqc7: o dia marcado',
     'cqc7_pw(r + 0x2a, cqc7_w(pt).wrapping_add(1) as u16); // mutacao: o dia seguinte ao marcado',
     'cqc_7 jogo levado a data'),
    ('cqc7_outra_fatia',
     'cqc7_pb(r + 0x3f, cqc7_b(v + 0x28)); // cqc7: a fatia marcada',
     'cqc7_pb(r + 0x3f, if cqc7_b(v + 0x28) == 1 { 2 } else { 1 }); // mutacao: a outra fatia',
     'cqc_7 jogo levado a data'),
    ('cqc7_dia_da_semana_seguinte',
     'ajustar(dp, cqc7_b(e + j + 0x27) as i8 as i16 as u16 as u32, pt); // cqc7: o dia da semana marcado',
     'ajustar(dp, (cqc7_b(e + j + 0x27) as i8 as i16 + 1) as u16 as u32, pt); // mutacao: o dia da semana seguinte',
     'cqc_7 jogo levado a data'),
]
