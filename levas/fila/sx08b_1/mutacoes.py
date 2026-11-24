# Mutacoes (sx08b_1). Os nomes nao voltam a simulacao (so textos e telas): acusadas pelo comparador, e so quando o corpo
# roda (idioma das preferencias diferente do idioma dos nomes; ver PRONTO.txt). Independem do conteudo do .lng:
# - as competicoes de nacao uma linha adiante (a ultima cai na primeira reserva);
# - o genero do clube lido do primeiro byte do nome curto da linha (nao zero em quase toda linha).
# Evitadas: mexer no teste das contas, no fopen ou no idioma desconhecido (caixa de erro espera clique e trava a cadeia).
MUTACOES = [
    ('sx08b_1_competicoes_de_nacao_adiante',
     '        let id = k - nation_comps - 0x7f + read_i32(G_CLUB_COMP_COUNT);',
     '        let id = k - nation_comps - 0x7e + read_i32(G_CLUB_COMP_COUNT); // mutation: one row ahead'),
    ('sx08b_1_genero_do_clube_errado',
     '    write_u8(c + club::GENDER_NAME, read_u8(row + 0x33));',
     '    write_u8(c + club::GENDER_NAME, read_u8(row + 0x34)); // mutation: the short name byte'),
]
