# Mutacoes (cx08_8), com a Argentina ate 2002-08-01:
# - a tabela da Segunda paga o fundo pela posicao (modo 1): o comparador da tabela acusa na criacao;
# - na Segunda sobem dois direto: o comparador das regras acusa (e o acesso muda);
# - a virada da Segunda passa o ano em 2: o comparador da virada acusa.
MUTACOES = [
    ('cx08_8_modo_do_premio',
     '    covered!("cx08_8 argentine second full table");\n    this.set_prize_fund(0);\n    this.set_prize_mode(2);',
     '    covered!("cx08_8 argentine second full table");\n    this.set_prize_fund(0);\n    this.set_prize_mode(1); // mutation',
     'cx08_8 argentine second full table'),
    ('cx08_8_dois_sobem',
     '(league::TIEBREAKS + 3, 1, 0), (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 0),',
     '(league::TIEBREAKS + 3, 1, 0), (league::PROMOTED_COUNT, 1, 2), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), // mutation\n        ',
     'cx08_8 argentine second rules'),
    ('cx08_8_ano_mais_dois',
     "if !rebuild_season(this, 0x407cd0, ARG_SECOND_FILE, [0x2ba, 0x2c1, 0x2c7, 0x2ce], 1) {",
     "if !rebuild_season(this, 0x407cd0, ARG_SECOND_FILE, [0x2ba, 0x2c1, 0x2c7, 0x2ce], 2) { // mutation",
     'cx08_8 argentine second turnover'),
]
