# Mutacoes (cx15_14), prova do save. Nenhuma abre caixa.
# - a carga do gerente dos elencos sem gravar a palavra: o ano da inscricao fica o do construtor (0), a impressao da
#   carga e a gravacao seguinte mudam;
# - o Stadium.tmp sem a ordem das sedes: o .qst muda (um byte a menos no Stadium.tmp).
MUTACOES = [
    ('cx15_14_carga_sem_palavra',
     '    covered!("cx15_14 squad manager loaded");\n    write_u16(this, v as u16);',
     '    covered!("cx15_14 squad manager loaded"); // mutation',
     'cx15_14 squad manager loaded'),
    ('cx15_14_sem_ordem_das_sedes',
     ' || !one(this + stadium_obj::VENUE_SORT, 1, 0x1a7) {',
     ' { // mutation'),
]
