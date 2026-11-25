# Mutacoes (sbI_1), carreira ENG ate 2002-06-01:
# - a tabela de classificacao nova sem a primeira nacao europeia (a Albania): as posicoes das nacoes abaixo dela e as
#   vagas europeias de 2001/02 (dadas no comeco da carreira) mudam; nao tira a Finlandia, a Eslovaquia nem a
#   Bielorrussia (o fair play de 2001 da a caixa 0x285 sem elas);
# - a ordem do euro.cfg sem avancar o lugar: cada clube listado toma o primeiro lugar do bloco e empurra o anterior, e
#   quem recebe as vagas europeias de 2001/02 muda (o comparador lado a lado tambem a acusa).
# A conferencia do euro.cfg (0x905d40) nao roda sem "-verifyeurocfg": sem mutacao.
MUTACOES = [
    ('sbI_1_seeding_sem_albania',
     '    for nation in european_nations() {\n        let e = seeding_nation_entry(this, seeding.nation_count() as i32);',
     '    for nation in european_nations().into_iter().skip(1) { // mutation\n        let e = seeding_nation_entry(this, seeding.nation_count() as i32);',
     'sbI_1 new seeding table'),
    ('sbI_1_euro_cfg_sem_avancar',
     '                    cursor += 12;\n                    break;\n                }\n                q += 12;\n                if Club(read_usize(q)).nation().addr() != nation {',
     '                    break; // mutation\n                }\n                q += 12;\n                if Club(read_usize(q)).nation().addr() != nation {',
     'sbI_1 euro.cfg club moved up'),
]
