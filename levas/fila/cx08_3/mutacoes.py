# Mutacoes (cx08_3), com a Argentina ate 2002-08-01:
# - a cota do fundo de premios de 50.000 vira 50.001 (o comparador da tabela acusa na criacao; as financas mudam);
# - a vitoria vale 2 pontos (o comparador das regras acusa; a classificacao muda);
# - a fase atual fica em 0 depois das duas fases (o comparador das fases acusa).
MUTACOES = [
    ('cx08_3_cota_do_fundo',
     'ARG_PRM_FILE, 0x1e7, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, 0xc350, || {',
     'ARG_PRM_FILE, 0x1e7, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, 0xc351, || { // mutation',
     'cqi_5 full table'),
    ('cx08_3_vitoria_dois_pontos',
     'vec![(league::LEGS, 2, 1), (league::WIN_POINTS, 1, 3),',
     'vec![(league::LEGS, 2, 1), (league::WIN_POINTS, 1, 2), // mutation\n        ',
     'cx08_3 argentine premier rules'),
    ('cx08_3_fase_atual_zero',
     '    exe::crt_free(clubs);\n    this.set_current_stage(1);\n}',
     '    exe::crt_free(clubs);\n    this.set_current_stage(0); // mutation\n}',
     'cx08_3 argentine half-season stages'),
]
