# Mutacoes (o comparando roda a original: so o modo Rust e a carreira acusam). A pessoa nova sem a ligacao ao
# contrato (o tecnico contratado ou o regen fica sem contrato pela 0x4cf5a0: salario, demissao, renovacao). O contrato
# inicial pela metade do valor (o salario gerado pela 0x85e960 muda as financas do clube). A data da decisao sem a
# conta subir (a proposta nunca e decidida pela 0x4cd9c0: as transferencias mudam). A vaga do vetor (0x4c79a0) fica
# sem mutacao: os dois chamadores passam 1 e a criacao dos contratos de todas as pessoas passa por ela (uma mutacao
# ali estraga a criacao do jogo, nao uma carreira).
MUTACOES = [
    ('cpl_contrato_ligacao', 'cpl4_por(cpl4_d(0xae2d0c).wrapping_add(cpl4_d(pessoa).wrapping_mul(0x4f)), i as usize); // a pessoa aponta a vaga',
     '// mutacao: a pessoa nao aponta a vaga', 'cpl_4 contrato da pessoa'),
    ('cpl_contrato_valor', 'gerar(c, pessoa, valor) // o valor da pessoa',
     'gerar(c, pessoa, valor / 2) // mutacao: metade do valor', 'cpl_4 contrato da pessoa'),
    ('cpl_decisao_conta', 'cpl4_por(this + 0x14, n.wrapping_add(1)); // a data da decisão',
     'cpl4_por(this + 0x14, n); // mutacao: a conta nao sobe', 'cpl_4 data de decisao'),
]
