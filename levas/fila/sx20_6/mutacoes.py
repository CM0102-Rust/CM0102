# Mutacoes (sx20_6). O construtor roda na criacao do jogo (carreira ENG: 1 vez); os registros de tecnico sem dono
# nao recebem jogos, pontos do mes e do ano nem premios (tecnico do mes, propostas):
# - so o cargo 12 conta como tecnico (os de cargo 5 ficam sem registro).
MUTACOES = [
    ('sx20_6_so_cargo_12',
     'let manager = |job: u8| job == 5 || job == 12; // sx20_6: the manager jobs',
     'let manager = |job: u8| job == 12; // mutation: job 5 left out',
     'sx20_6 staff records built'),
]
