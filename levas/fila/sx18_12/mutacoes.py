# Mutacoes (sx18_12), pela carreira com LIGAS 'ENG BEL HOL GER DEN' ate 2002-08-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_12_vermelho_direto',
     "if row.yellow_cards() != 0 { // sx18_12: only the second booking's red bans on the spot",
     'if true { // mutation: every red bans on the spot',
     'sx18_12 belgian league red card'),
    ('sx18_12_escala_copa',
     'rule_set: 5, scale: 5,',
     'rule_set: 5, scale: 6, // mutation\n   ',
     'sx18_12 belgian cup body built'),
]
