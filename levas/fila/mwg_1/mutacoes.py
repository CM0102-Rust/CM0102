# Mutacoes (mwg_1), nos jogadores virtuais de cada partida (toda partida passa por aqui):
# - os laterais esquerdos contam como zagueiros centrais: o grupo 1 fica abaixo do minimo e o 3 acima, e a completacao
#   cria outros jovens virtuais (outras escalacoes nos clubes ingleses);
# - o visitante e completado sem a partida: os suspensos contam como disponiveis e faltam virtuais onde havia suspensos;
# - o mandante sem vaga de clube visto guarda os virtuais depois da partida (ficam no elenco ate a proxima contagem).
MUTACOES = [
    ('mwg1_lateral_esquerdo_central',
     "return if p.left_side() >= GOOD { 1 } else if p.right_side() >= GOOD { 2 } else { 3 }; // mwg1: the defenders' side",
     "return if p.left_side() >= GOOD { 3 } else if p.right_side() >= GOOD { 2 } else { 3 }; // mutation: left backs count as central",
     'mwg_1 defender'),
    ('mwg1_visitante_sem_partida',
     "top_up(this, away.addr(), staff_id, player_id, fixture, (read_u8(fixture + fixture::AWAY_SIDE_MARK) == 0) as u32); // mwg1: the away side's top-up",
     "top_up(this, away.addr(), staff_id, player_id, 0, (read_u8(fixture + fixture::AWAY_SIDE_MARK) == 0) as u32); // mutation: the away side's banned players count",
     'mwg_1 both sides topped up'),
    ('mwg1_mandante_guarda_virtuais',
     "drop_club_virtual_players(Club(home)); // mwg1: the home side's virtual players leave",
     "let _ = home; // mutation: the home side keeps its virtual players",
     'mwg_1 side without a viewed slot'),
]
