# Mutacoes (sx19_18), pela carreira ENG ate 2002-06-01:
# - a suspensao por jogos valendo um jogo a menos (os suspensos voltam uma partida antes);
# - a Liga dos Campeoes sem zerar os amarelos dos classificados para as quartas (as suspensoes europeias seguintes
#   mudam; se a carreira nao acusar, o comparador lado a lado da limpeza acusa).
MUTACOES = [
    ('sx19_18_um_jogo_a_menos',
     '(read_i8(ban + ban_record::MATCHES_SERVED) as i32 + extra as i32 <= read_i8(ban + ban_record::MATCHES) as i32) as u32 // sx19_18: matches still to serve',
     '((read_i8(ban + ban_record::MATCHES_SERVED) as i32 + extra as i32) < read_i8(ban + ban_record::MATCHES) as i32) as u32 // mutation: one match less',
     'sx19_18 ban by matches'),
    ('sx19_18_amarelos_ficam',
     'write_u8(e + ruling_body_entry::YELLOW_CARDS, 0); // sx19_18: the club\'s bookings cleared',
     'let _ = e; // mutation: the bookings kept',
     'sx19_18 bookings cleared'),
]
