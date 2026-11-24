# Mutacoes (sx03_4).
# - sem o jogador do ano mundial de 6 de maio de 2002: a reputacao dos vencedores muda (carreira);
# - as notas do mes tambem com as copas: o jogador do mes muda (carreira);
# - a Dinamarca com uma janela: o comparador dos campos do construtor acusa.
MUTACOES = [
    ('sx03_4_sem_jogador_mundial',
     'award_world_player_from_ratings(this, today, nation_award(this, 0), 2); // sx03_4: 6 May',
     '{} // mutation: no world player of the year',
     'sx03_4 world player of the year'),
    ('sx03_4_notas_com_copas',
     'if exe::comp_util_is_league_division(read_i32(Fixture(fixture).competition().addr())) != 0 { // sx03_4: league divisions only',
     'if true { // mutation: every competition',
     'sx03_4 league match rated'),
    ('sx03_4_dinamarca_com_janela',
     'write_u8(this + 8, 0); // sx03_4: no windows',
     'write_u8(this + 8, 1); // mutation: one window',
     'sx03_4 denmark rules'),
]
