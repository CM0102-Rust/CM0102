# Mutacoes (com o ramo de cada uma). As vagas: a vaga aberta nunca e achada (clube sem tecnico que ja tem vaga ganha
# outra pela 0x69ba50 a cada uma das 16 chamadas; a lista de vagas e as contratacoes de tecnico mudam). A melhor nota
# de candidato conta o tecnico atual (a 0x69b0c0 compara a nota dele com a melhor + 450: deixa de valer; a diretoria
# muda a decisao). A oferta agendada um dia depois (~4634 ofertas: as negociacoes andam em outros dias; o comparando
# acusa pelo plano tambem).
MUTACOES = [
    ('cpn_vaga_nunca_achada', 'return Some(true);',
     'return Some(false); // mutacao: a vaga aberta nao conta', 'cpn_5 clube com vaga aberta'),
    ('cpn_candidato_conta_o_atual', "break 'f true; // o atual fica de fora",
     "if false { break 'f true; } // mutacao: o tecnico atual entra", 'cpn_5 o tecnico atual fica de fora'),
    ('cpn_oferta_um_dia_depois', 'let r = somar(0xae2c90, tmp.as_mut_ptr(), dias as i16 as i32 as u32);',
     'let r = somar(0xae2c90, tmp.as_mut_ptr(), (dias as i16 + 1) as i32 as u32); // mutacao: um dia depois', 'cpn_5 agendar oferta'),
]
