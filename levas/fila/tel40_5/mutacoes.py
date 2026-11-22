# A carga do jogo salvo (Carregar Jogo ou -load com um tecnico humano): o comparador lado a lado acusa numa sessao do
# Lucas com o JOGAR_COMPARANDO.bat. A carreira automatica (a cadeia) nao passa por aqui.
# - o segundo tecnico sem senha nao ganha registro proprio (so com dois tecnicos humanos sem senha no save);
# - o tecnico do registro atual sem a chave (qualquer carga com um tecnico sem senha).
MUTACOES = [
    ('tel40_5_sem_registro_proprio', '                add(sm, NETWORK_JOIN_CALLBACK, NETWORK_LEAVE_CALLBACK, person.addr()); // tel40_5: a record of his own', '                let _ = add; // mutation: no record of his own'),
    ('tel40_5_sem_chave', '                write_usize(key, person.addr());\n                write_usize(sm + SCRMAN_JOIN_HOOK, NETWORK_JOIN_CALLBACK);', '                write_usize(sm + SCRMAN_JOIN_HOOK, NETWORK_JOIN_CALLBACK); // mutation: no key'),
]
