# Mutações (sx09c_8). A multa da AWOL chega com o motivo 5 (ausente sem licença): o caminho 1/5. Acusadas pelo
# comparador (lado a lado) e, mudando a resposta, pela carreira (a 0x5a5980 segue outro ramo: elenco, diretoria, queixa).
MUTACOES = [
    ('sx09c_8_awol_unfair',
     "return Some(true); // sx09c_8: absent without leave",
     "return Some(false); // mutation: the absence makes the fine unfair",
     'sx09c_8 fine verdict'),
    ('sx09c_8_two_bookings',
     "return Some(true); // sx09c_8: two bookings",
     "return Some(false); // mutation: two bookings unfair",
     'sx09c_8 last match read'),
    ('sx09c_8_draw_fair',
     "            exe::utils_random(100) == 0\n",
     "            exe::utils_random(100) != 0 // mutation: the last draw inverted\n",
     'sx09c_8 last match read'),
]
