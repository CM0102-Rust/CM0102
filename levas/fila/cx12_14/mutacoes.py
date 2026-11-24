# Mutacoes (cx12_14), com a Coreia ate 2002-08-01:
# - a temporada de 2001 com as datas de 2002 (e a de 2002 com as outras): outros dias de jogo em 2001, outro campeao
#   coreano e outra temporada de 2002; acusada tambem pelo comparador do calendario;
# - a cota do fundo de premios dobrada: outros premios pagos no fim da temporada de 2001 (outubro) e outras financas;
#   acusada tambem pelo comparador da tabela (a cota conferida).
MUTACOES = [
    ('cx12_14_datas_de_2002_em_2001',
     'let table: &[FraThirdLeagueRound] = if this.year() == 0x7d2 { &KOREAN_LEAGUE_2002 } else { &KOREAN_LEAGUE_ROUNDS }; // cx12_14: the 2002 dates',
     'let table: &[FraThirdLeagueRound] = if this.year() != 0x7d2 { &KOREAN_LEAGUE_2002 } else { &KOREAN_LEAGUE_ROUNDS }; // mutation: the dates swapped',
     'cx12_14 korean schedule'),
    ('cx12_14_cota_dobrada',
     'set_prize_fund(this, 0x2710); // cx12_14: the Korean quota',
     'set_prize_fund(this, 0x4e20); // mutation: twice the quota',
     'cx12_14 korean table'),
]
