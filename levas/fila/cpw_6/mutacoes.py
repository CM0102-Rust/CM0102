# Mutacoes (cpw_6), na injury_word_0c do jogador premiado (0x412f00 pela vaga do vencedor do mes: o jogador e o jovem do mes
# de cada divisao inglesa, toda virada de mes ate 2002-06-01). O registro do jogador (+0x61) entra no hash diario do
# banco: o aumento mundial com 10% em vez de 5% (muda quando o premiado ja esta acima da mundial do premio), sem o piso
# mundial (muda quando ele esta abaixo dela) e o teste do clube da nacao do jogador invertido (a nacional sobe 2,5% da
# soma com piso na media em vez de 10% com piso na nacional, e vice-versa). Entre as duas primeiras, ao menos uma pega
# em cada premio; a terceira escapa so se nenhum premio cair num caso em que as duas formulas deem o mesmo valor.
MUTACOES = [
    ('cpw6_aumento_mundial', '        add_to_word(p + 0xd, match_pl_ftol(mundo as i32 as f64 * read_f64_at(0x967888))); // cpw6: o aumento mundial',
     '        add_to_word(p + 0xd, match_pl_ftol(mundo as i32 as f64 * read_f64_at(0x967880))); // mutacao: 10%',
     'cpw_6 premio de jogador'),
    ('cpw6_sem_piso_mundial', '        if cpw6_w(p + 0xd) <= mundo { // cpw6: o piso mundial',
     '        if false { // mutacao: sem o piso',
     'cpw_6 premio de jogador'),
    ('cpw6_nacao_invertida', '        if clube != 0 && cpw6_d(clube + 0x53) == cpw6_d(pessoa + 0x1a) { // cpw6: o clube da nação do jogador',
     '        if clube != 0 && cpw6_d(clube + 0x53) != cpw6_d(pessoa + 0x1a) { // mutacao: invertido',
     'cpw_6 premio de jogador'),
]
