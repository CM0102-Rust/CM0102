# Mutacoes (sbF_5), carreira com a Coreia ate 2002-08-01. Nenhuma abre caixa.
# - o grupo B sem tirar os clubes da lista: o comparador acusa (as vagas da lista ficam); a carreira nao muda;
# - as semifinais com os dois primeiros juntos (o primeiro do grupo A contra o primeiro do B, os vices entre si): quem
#   joga quem muda (a carreira acusa na semifinal de maio; o comparador confere so o conjunto).
MUTACOES = [
    ('sbF_5_grupo_b_sem_tirar',
     '        write_usize(e, 0);\n        c\n    }, "sbF_5 korean league cup group b")',
     '        c // mutation\n    }, "sbF_5 korean league cup group b")',
     'sbF_5 korean league cup group b'),
    ('sbF_5_primeiros_juntos',
     '    write_usize(teams + 4, read_usize(group_b + league_table_row::SIZE)); // sbF5: group B\'s runner-up meets group A\'s winner\n'
     '    write_usize(teams + 8, read_usize(this.table() + league_table_row::SIZE));\n'
     '    write_usize(teams + 0xc, read_usize(group_b));',
     '    write_usize(teams + 4, read_usize(group_b)); // mutation\n'
     '    write_usize(teams + 8, read_usize(this.table() + league_table_row::SIZE));\n'
     '    write_usize(teams + 0xc, read_usize(group_b + league_table_row::SIZE));',
     'sbF_5 korean league cup knockout'),
]
