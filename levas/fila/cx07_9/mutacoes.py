# Mutacoes (cx07_9), carreira de 8 ligas ate 2002-06-01 (o construtor roda na criacao do jogo para toda liga, fase de
# liga e selecao em torneio; os jogos fixos na criacao da Serie A italiana e da primeira divisao francesa).
# - as regras de base da liga com o bit 2 desligado (acusada pelo comparador lado a lado do objeto);
# - o jogo fixo com o mandante no lugar do visitante (acusada pelo comparador lado a lado da entrada e pela carreira: o
#   classico fixo da Italia e da Franca muda de mando).
MUTACOES = [
    ('cx07_9_regras_de_base_1',
     '    write_u16(this + league::LEAGUE_FLAGS, 3); // cx07_9: the default rule bits',
     '    write_u16(this + league::LEAGUE_FLAGS, 1); // mutation'),
    ('cx07_9_mandante_trocado',
     '            write_u32(home_id, read_u32(club)); // cx07_9: the home club\'s id',
     '            write_u32(this + fixed_fixture::AWAY_CLUB_ID, read_u32(club)); // mutation'),
]
