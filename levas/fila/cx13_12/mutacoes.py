# Mutacoes (cx13_12), com a Russia ate 2002-08-01:
# - os recordes comuns com o status do clube trocado no bit 0 (o campeao vira 4, o rebaixado 2...: outros recordes de
#   pessoal e outras reputacoes de tecnico; duvida: a carreira pode nao acusar ate agosto, o comparador so roda a
#   original).
# O ramo do play-off so roda com os dois primeiros empatados: sem mutacao propria (o cx13_11 tem a que forca o
# play-off, e com ela este ramo roda).
MUTACOES = [
    ('cx13_12_status_trocado',
     'return exe::league_update_staff_records_for_status(this, club, status, 0xffff_ffff, a4, game, a6); // cx13_12: the common records',
     'return exe::league_update_staff_records_for_status(this, club, status ^ 1, 0xffff_ffff, a4, game, a6); // mutation',
     'cx13_12 russian staff records'),
]
