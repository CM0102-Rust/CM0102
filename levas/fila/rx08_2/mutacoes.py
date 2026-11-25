# Mutacao pela prova do save (o .qst gravado pela original e pela copia, item a item):
# - rx08_2_uma_entrada_a_menos: a ultima entrada dos clubes nao e gravada: o "club records.dat" sai 0x6a bytes mais
#   curto (e a carga desse .qst para o jogo na leitura das entradas).
# - rx08_2_um_jogador_a_menos: a ultima entrada dos jogadores nao e gravada: "player stats.dat" 0x27 bytes mais curto.
MUTACOES = [
    ('rx08_2_uma_entrada_a_menos',
     'let entries = read_u32(this + c::CLUB_ENTRY_COUNT); // rx08_2: every club entry',
     'let entries = read_u32(this + c::CLUB_ENTRY_COUNT) - 1; // rx08_2: mutation, one entry short',
     'rx08_2 save club records'),
    ('rx08_2_um_jogador_a_menos',
     "let entries = read_u32(this + c::ENTRY_COUNT); // rx08_2: every player's entry",
     "let entries = read_u32(this + c::ENTRY_COUNT) - 1; // rx08_2: mutation, one player short",
     'rx08_2 save player stats'),
]
