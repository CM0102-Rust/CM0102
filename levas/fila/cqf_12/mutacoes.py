# Mutacoes (cqf_12), no dia dos olheiros (0x7f8750, 966 chamadas ate 2002-06-01): o pedido processado gera relatorios
# (0x7b62a0, que sorteia); as mutacoes mudam quando o pedido e processado ou quantos relatorios saem.
# ATENCAO (02/11, carreira automatica ate 2002-06-01): nenhum pedido foi processado (contadores 'cqf_12 pedido
# processado' e 'relatorio do pedido' nao rodaram) e tirar as buscas dos humanos (0x7b3e20) nao mudou a impressao:
# as tres mutacoes escapam. O item so passa nas mutacoes junto de outro com efeito.
MUTACOES = [
    ('cqf12_quinze_dias',
     'let mes_depois = mais(pedido + 8, t2.as_mut_ptr() as usize, 0x1e); // cqf12: trinta dias',
     'let mes_depois = mais(pedido + 8, t2.as_mut_ptr() as usize, 0xf); // mutacao: quinze dias',
     'cqf_12 pedido processado'),
    ('cqf12_um_relatorio',
     'if feitos >= 3 { // cqf12: até três relatórios',
     'if feitos >= 1 { // mutacao: um relatorio',
     'cqf_12 relatorio do pedido'),
    ('cqf12_meio_ano',
     'let ano_depois = mais(pedido + 8, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano',
     'let ano_depois = mais(pedido + 8, t.as_mut_ptr() as usize, 0x2d); // mutacao: 45 dias',
     'cqf_12 pedido processado'),
]
