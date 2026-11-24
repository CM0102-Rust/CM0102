# Mutacoes (sx08_3). Os nomes ingleses nao voltam a simulacao (so textos e telas com outro idioma): as mutacoes sao
# acusadas pelo comparador (a carga roda uma vez na carreira de teste, no primeiro nome ingles pedido).
# - a tabela de estadios nao e lida: o arquivo fica dessincronizado e as cidades, competicoes de pessoal e de clube
#   vem com os bytes errados (sem leitura fora da memoria: cada tabela tem o tamanho da propria conta).
# Evitadas: mexer na conferencia das contas ou no marcador (a caixa de erro espera clique e trava a cadeia).
MUTACOES = [
    ('sx08_3_sem_estadios',
     '            exe::crt_fread(p, size, n, file);',
     '            if rows != english_name_tables::STADIUM_ROWS { exe::crt_fread(p, size, n, file); } // mutation: stadiums not read'),
]
