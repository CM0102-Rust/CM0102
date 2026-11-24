# Mutacoes (cx15_12), prova do save (prova_save.py --leva N, mutacoes.py --prova-save N). Nenhuma abre caixa.
# - a gravacao sem o numero de isentas: o .qst muda (um byte a menos no bloco das eliminatorias africanas);
# - a carga sem converter os ids: a lista fica com ids no lugar dos registros (a impressao da carga muda).
MUTACOES = [
    ('cx15_12_grava_sem_isentas',
     '    exe::crt_fwrite(this + q.exempt, 1, 1, file);\n    exe::crt_fwrite(this + q.count, 4, 1, file);',
     '    exe::crt_fwrite(this + q.count, 4, 1, file); // mutation',
     'cx15_12 african qualifiers saved'),
    ('cx15_12_carga_sem_converter',
     '    covered!("cx15_12 african qualifiers loaded");\n    qualifier_list_from_ids(this, q);',
     '    covered!("cx15_12 african qualifiers loaded"); // mutation',
     'cx15_12 african qualifiers loaded'),
]
