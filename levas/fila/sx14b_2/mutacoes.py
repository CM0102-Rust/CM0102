# Mutacoes (sx14b_2), pela prova do save (mutacoes.py --prova-save N; a carga do save de ferias com e sem a leva):
# - os registros lidos com um a menos: a leitura falha e o construtor monta a diretoria do zero (confiancas, metas e
#   datas da diretoria de todos os clubes mudam no jogo carregado);
# - a lista dos ultimatos nao e lida: a contagem das apelacoes sai do lugar e a leitura falha (diretoria do zero).
MUTACOES = [
    ('sx14b_2_registros_curtos',
     '        if exe::zread(table, 0x49, count as u32, z) != count as u32 {',
     '        if exe::zread(table, 0x49, count as u32 - 1, z) != count as u32 { // mutation',
     'sx14b_2 board from the save'),
    ('sx14b_2_sem_ultimatos',
     '    if !man_conf_read_list(z, read_usize(this + f::ULTIMATUM_LIST), 4) {',
     '    if false { // mutation: the ultimatum list is not read',
     'sx14b_2 board from the save'),
]
