# Mutacoes (cqn_8). O mata-mata de cada estadual e montado quando a fase passa de -1 a 0, logo depois da ultima
# rodada dos pontos corridos (duas vezes na carreira de 8 ligas ate 2002-06-01). Cada uma troca quem joga contra quem
# (outras chaves, outro mandante no primeiro jogo ou outro clube no mata-mata) e mexe so nas linhas: as sementes ficam
# as da original, entao o par clube-semente muda mesmo se o sorteio da rodada seguir as sementes; outros jogos, que a carreira le
# (o modelo da cpz_1, acusada inteira na 262a). O contador fica no corpo comum, logo depois da lista gravada (a
# unica match_condition antes e a memoria da lista), com o nome de cada estadual.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS em 2001-147.
MUTACOES = [
    ('cqn8_pernambucano_chaves_trocadas',
     'bra_reg_pern_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // cqn8: 1º × 4º, 2º × 3º Pernambucano',
     'bra_reg_pern_build_knockout_from_table(this, &[0, 2, 1, 3], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // mutacao: 1º × 3º e 2º × 4º',
     'cqn_8 pernambucano'),
    ('cqn8_catarinense_chaves_trocadas',
     'bra_reg_pern_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // cqn8: 1º × 4º, 2º × 3º Catarinense',
     'bra_reg_pern_build_knockout_from_table(this, &[3, 0, 2, 1], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // mutacao: o pior colocado manda o primeiro jogo',
     'cqn_8 catarinense'),
    ('cqn8_paulista_chaves_trocadas',
     'bra_reg_pern_build_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // cqn8: 1º × 4º, 2º × 3º Paulista',
     'bra_reg_pern_build_knockout_from_table(this, &[0, 4, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // mutacao: o 5º no lugar do 4º',
     'cqn_8 paulista'),
]
