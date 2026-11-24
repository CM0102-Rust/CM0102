# Mutacoes (cx08_3), com a Argentina ate 2002-08-01:
# - a cota do fundo de premios de 50.000 vira 50.001 (o comparador da tabela acusa na criacao; as financas mudam);
# - os bits da liga ganham o 2 (0x43: o comparador das regras acusa);
# - a fase atual fica em 0 depois das duas fases (o comparador das fases acusa).
MUTACOES = [
    ('cx08_3_cota_do_fundo',
     'ARG_PRM_FILE, 0x1e7, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, 0xc350, || {',
     'ARG_PRM_FILE, 0x1e7, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, 0xc351, || { // mutation',
     'cqi_5 full table'),
    ('cx08_3_bits_da_liga',
     '(league::LEAGUE_FLAGS, 2, 0x41),',
     '(league::LEAGUE_FLAGS, 2, 0x43 /* mutation */),',
     'cx08_3 argentine premier rules'),
    ('cx08_3_fase_atual_zero',
     '    exe::crt_free(clubs);\n    this.set_current_stage(1);\n}',
     '    exe::crt_free(clubs);\n    this.set_current_stage(0); // mutation\n}',
     'cx08_3 argentine half-season stages'),
]
