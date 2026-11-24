# Mutacoes (sx21_35). Roda nas ofertas da carreira ENG; caem pelo comparador lado a lado e pela carreira (o jogador que
# fica sabendo do lance e as clausulas mudam o andamento das ofertas).
MUTACOES = [
    ('sx21_35_famoso_5000',
     '&& exe::utils_random(0x14) == 0 && Staff(player).player().current_reputation() > 0x1b58 { // sx21_35: a famous player',
     '&& exe::utils_random(0x14) == 0 && Staff(player).player().current_reputation() > 0x1388 { // mutation: 5000',
     'sx21_35 the player hears of the bid'),
    ('sx21_35_nove_jogos',
     'o.set_international_games(0xa); // sx21_35: ten games for each clause',
     'o.set_international_games(9); // mutation: nine',
     'sx21_35 the player hears of the bid'),
]
