# Mutacoes (com o ramo de cada uma; os tres comparadores rodam so a original: so o modo Rust e a carreira acusam). As
# tres tiram respostas automaticas do tecnico de testes, que fica de ferias a carreira toda (convites de amistoso,
# propostas de emprego e de transferencia: a 0x784100 muda os amistosos, o emprego e as transferencias). A resposta
# automatica pelo clube nao dada. O alcance 0 (o tecnico do clube) sem entrega. O tecnico do clube lido no auxiliar
# (+0xd3, nunca humano): a noticia nao chega ao humano. Se a carreira nao tiver nenhuma noticia que peca resposta ao
# clube do humano ate 2002-06-01, as tres PODEM ESCAPAR (ver os ramos).
MUTACOES = [
    ('cpn_resposta_do_clube', 'respond(vaga(atual), tecnico, cpn1_d(tecnico + 0x39)); // o clube',
     '// mutacao: sem a resposta automatica pelo clube',
     'cpn_1 automatic reply'),
    ('cpn_alcance_clube', 'crate::comparison::branch("cpn_1 club reach");',
     'crate::comparison::branch("cpn_1 club reach"); return 0; // mutacao: o alcance do clube sem entrega',
     'cpn_1 club reach'),
    ('cpn_tecnico_do_clube', 'let tecnico = cpn1_d(clube + 0xcf);\n    *((noticia + 0xde) as *mut u8) = 0;',
     'let tecnico = cpn1_d(clube + 0xd3); // mutacao: o auxiliar\n    *((noticia + 0xde) as *mut u8) = 0;',
     'cpn_1 news to the human manager'),
]
