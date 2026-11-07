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
     'cqu3_resultado(this, jogo, estat, 0xb, 0x4351b0,',
     'cqu3_resultado(this, jogo, estat, 0xc, 0x4351b0, /* mutacao: o segundo turno na 12a */',
     'cqu_3 turno do mineiro'),
    ('cqu3_paranaense_sem_tabela_do_turno',
     'somar(f, jogo, estat); // cqu3: o jogo na tabela do turno',
     'if turno != 0x438760 { somar(f, jogo, estat); } // mutacao: o Paranaense sem a tabela do turno',
     'cqu_3 turno do paranaense'),
    ('cqu3_j1_turno_na_16a',
     'cqu3_resultado(this, jogo, estat, 0xf, 0x663c20,',
     'cqu3_resultado(this, jogo, estat, 0x10, 0x663c20, /* mutacao: o segundo turno na 16a */',
     'cqu_3 turno da j1'),
    ('cqu3_carioca_turno_na_12a',
     'let fase: i8 = (rodada >= segundo) as i8; // cqu3: o turno do jogo',
     'let fase: i8 = (rodada >= if turno == 0x43afa0 { segundo + 1 } else { segundo }) as i8; // mutacao: o Carioca com o segundo turno na 12a',
     'cqu_3 turno do carioca'),
]
