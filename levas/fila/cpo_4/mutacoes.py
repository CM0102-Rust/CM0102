# Mutacoes. O tecnico que sai continua no +0xcf do clube (comparando, o plano da 0x690890 acusa; na carreira o clube
# fica com o tecnico demitido). O jogador transferido continua numa das 50 vagas do elenco do clube antigo (so o modo
# Rust e a carreira: o comparador roda a original; o elenco do clube muda). A vaga do clube que demitiu nao abre (so a
# carreira; se nenhum clube abrir vaga pela lista ate 2002-06-01 PODE ESCAPAR, ver o ramo).
MUTACOES = [
    ('cpo_tirar_o_tecnico', 'cpo4_por(c + 0xcf, 0); // o técnico do clube',
     '// mutacao: o tecnico fica no clube',
     'cpo_4 tirar o tecnico'),
    ('cpo_tirar_do_elenco', 'cpo4_por(v, 0); // a vaga do elenco',
     '// mutacao: o jogador fica no elenco',
     'cpo_4 tirar do elenco'),
    ('cpo_abrir_a_vaga', 'vaga(this, clube, tecnico); // a vaga do clube',
     'let _ = (vaga, tecnico); // mutacao: sem a vaga',
     'cpo_4 abrir a vaga'),
]
