# Mutacoes (sbE_6), com a Irlanda ate 2002-06-01:
# - o codigo do clube da Premier que perde a repescagem 11 em vez de 12 (acusada pelo comparador lado a lado);
# - os textos da repescagem trocados (acusada pelo comparador de texto);
# - o recorde de qualquer clube como se fosse da temporada (acusada pela carreira: os registros de pessoal dos tecnicos
#   da repescagem e os status da tabela; e pelo comparador dos recordes no retorno).
MUTACOES = [
    ('sbE_6_codigo_da_premier',
     '2 => 0xc, // sbE_6: the Premier club that loses the play-off',
     '2 => 0xb, // mutation',
     'sbE_6 irish play-off progress'),
    ('sbE_6_textos_trocados',
     'return play_off_club_text(club_id, if premier { 0x987594 } else { 0x989c18 }, text); // sbE_6: avoid relegation / win promotion',
     'return play_off_club_text(club_id, if premier { 0x989c18 } else { 0x987594 }, text); // mutation',
     'sbE_6 irish play-off text'),
    ('sbE_6_tudo_temporada',
     'play_off_update_staff_records(this, club, status, stage, &IRISH_FIRST_PLAY_OFF)\n}',
     'play_off_update_staff_records(this, club, status, 0xff, &IRISH_FIRST_PLAY_OFF) // mutation\n}',
     'sbE_6 irish staff records'),
]
