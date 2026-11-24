# Mutacoes (sbA_5): o historico gravado errado volta pela leitura (get_nodes) nas regras que olham o historico
# (suspensoes, lesoes, premios); o comparador do acrescimo (a releitura do registro no ultimo deslocamento) so prova a
# original, entao estas podem escapar da carreira se nenhuma regra ler o historico ate 2002-06-01.
MUTACOES = [
    ('sbA_5_ultimo_nao_avanca',
     '    *last = end;\n    exe::crt_fclose(f);\n}',
     '    let _ = end; // mutation\n    exe::crt_fclose(f);\n}'),
    ('sbA_5_lista_sem_ligacao',
     '            let next = if after != 0 { exe::crt_ftell(f) + 4 } else { -1 };',
     '            let next = -1; // mutation'),
]
