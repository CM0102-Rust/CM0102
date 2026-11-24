# Mutacoes (sx19b_7), item de TELAS: a carreira automatica nao passa; servem para provar os comparadores numa sessao do
# Lucas jogando comparando:
# - o status "fora do alcance" trocado por "possivel" (o comparador do status acusa num jogador de uma nacao que o
#   clube nao alcanca);
# - a ordem dos pedidos pelo clube primeiro (o comparador da ordem acusa no primeiro qsort/bsearch com dois pedidos).
MUTACOES = [
    ('sx19b_7_sempre_alcanca',
     'if can_club_scout_nation(this, nation, club) != 0 { 1 } else { 4 } // sx19b_7: in range',
     '1 // mutation: always in range'),
    ('sx19b_7_ordem_pelo_clube',
     'return pa.wrapping_sub(pb); // sx19b_7: by the player first',
     'return read_i32(a + scout_request::CLUB_ID).wrapping_sub(read_i32(b + scout_request::CLUB_ID)); // mutation: by the club first'),
]
