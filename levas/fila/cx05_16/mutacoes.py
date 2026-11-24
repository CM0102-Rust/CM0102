# Mutacoes (cx05_16) pela prova do save (CARGA). So acusam se o save tiver minicopas ou minligas vivas (ver a duvida
# no rotinas.py).
# - a minicopa carregada fica com o registro da competicao seguinte da tabela: o nome e o id da competicao mudam (a
#   impressao da carga e o comparador da carga acusam).
MUTACOES = [
    ('cx05_16_registro_seguinte',
     "    write_usize(this + league::COMPETITION, record); // cx05_16: the mini cup's competition record",
     "    write_usize(this + league::COMPETITION, record + 0x6b); // mutation"),
]
