# Mutacoes (r7_08_4). TELAS: a cadeia nao roda; o comparador (lado a lado com gravar = 0) as acusa quando o humano
# oferece um jogador na troca jogando comparando. Acusadas tambem na emulacao (scratchpad\r7_08\check8.py wants).
# - o estrangeiro jovem precisa de 400 + sorteio(1000) em vez de 500;
# - o interesse minimo sobe de 2,75 para 3;
# - a escala da margem vira reputacao^2 / 8.
MUTACOES = [
    ('r7_08_4_estrangeiro',
     '            if seen < exe::utils_random(1000) + 500 { // r7_08_4: the reputation a foreign youngster needs',
     '            if seen < exe::utils_random(1000) + 400 { // r7_08_4 mutation',
     'r7_08_4 wanted'),
    ('r7_08_4_interesse',
     '        if !(exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 1, 0, status, 0) >= read_f64(rdata::F64_2_75)) { // r7_08_4: the interest needed',
     '        if !(exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 1, 0, status, 0) >= read_f64(rdata::F64_3)) { // r7_08_4 mutation',
     'r7_08_4 wanted'),
    ('r7_08_4_escala',
     '    let scale = (reputation as i32 * reputation as i32) / 10;',
     '    let scale = (reputation as i32 * reputation as i32) / 8; // r7_08_4 mutation',
     'r7_08_4 wanted'),
]
