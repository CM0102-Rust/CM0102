# Mutacoes (sx21_34). Rodam nas ofertas da carreira ENG; caem pelos comparadores (e pela carreira quando mudam a
# avaliacao de uma oferta). O destrutor nao tem mutacao segura (so libera memoria).
MUTACOES = [
    ('sx21_34_venda_dobrada',
     'total = total.wrapping_add(read_i32(target.addr() + staff::VALUE).wrapping_mul(share as i32) / 100); // sx21_34: the sell-on share',
     'total = total.wrapping_add(read_i32(target.addr() + staff::VALUE).wrapping_mul(share as i32) / 50); // mutation: doubled',
     'sx21_34 sell-on share'),
    ('sx21_34_nao_acha',
     'if read_u32(d) == read_u32(player) { // sx21_34: the entry of this player',
     'if read_u32(d) == read_u32(player).wrapping_add(1) { // mutation: the next id',
     'sx21_34 exchange player found'),
]
