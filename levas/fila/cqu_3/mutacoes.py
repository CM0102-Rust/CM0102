# Mutacoes (cqu_3). Os quatro rodam a cada jogo do Mineiro, do Paranaense, do Carioca e da J1 (~260 jogos cada nos
# estaduais e ~360 na J1 ate 2002-06-01; contadores "cqu_3 turno do <liga>"):
# - o Mineiro com o segundo turno a partir da 12a rodada (os jogos da 11a contam para o primeiro turno: outra tabela
#   do turno, outro campeao do turno);
# - o Paranaense sem o jogo na tabela do turno (os turnos ficam vazios: outros campeoes dos turnos e outra final);
# - a J1 com o segundo turno a partir da 16a rodada;
# - o Carioca com o segundo turno a partir da 12a rodada (no lugar da marca +0x42 nao devolvida, que escapou: cada jogo
#   passa uma vez so pelo resultado e ninguem mais le a marca ate 2002-06-01).
MUTACOES = [
    ('cqu3_mineiro_turno_na_12a',
     'bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x4351b0,',
     'bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xc, 0x4351b0, /* mutation: the second half from round 12 */',
     'cqu_3 minas gerais half'),
    ('cqu3_paranaense_sem_tabela_do_turno',
     'add(f, game, stats); // cqu3: the game in the half table',
     'if half != 0x438760 { add(f, game, stats); } // mutation: the Parana league without the half table',
     'cqu_3 parana half'),
    ('cqu3_j1_turno_na_16a',
     'bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xf, 0x663c20,',
     'bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0x10, 0x663c20, /* mutation: the second half from round 16 */',
     'cqu_3 j1 half'),
    ('cqu3_carioca_turno_na_12a',
     'let stage: i8 = (round >= second) as i8; // cqu3: the half of the game',
     'let stage: i8 = (round >= if half == 0x43afa0 { second + 1 } else { second }) as i8; // mutation: the Rio league with the second half from round 12',
     'cqu_3 rio half'),
]
# Prova do agente (07/11, carreira Rust de 8 ligas ate 2002-06-01 com motor de lances): mineiro, paranaense e j1 ACUSADAS (2001-098, 2001-094, 2001-174; base com 12 itens, f29c85195c2a534f); a do Carioca sem devolver a marca escapou (cada jogo passa uma vez so pelo resultado) e foi trocada pelo segundo turno na 12a: ACUSADA (2001-097; base com os 10 itens finais).
