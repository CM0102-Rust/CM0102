# Mutacoes. A ordem das datas com "antes" valendo "depois" (o qsort da 0x4c9de7 fica sem ordem e a 0x4cd9c0 para numa
# data futura: as decisoes de contrato atrasam; sem ramo, a 0x4dc910 e quente). As datas vencidas do tipo 3 sem a
# decisao das ofertas (as pessoas nao respondem as ofertas de contrato: contratos e transferencias mudam). As ofertas
# do clube com uma a menos na conta (o orcamento de transferencias 0x598a06 sobe para quem tem oferta aberta; se ate
# 2002-06-01 nenhum clube tiver oferta aberta quando o orcamento for pedido PODE ESCAPAR, ver o ramo).
MUTACOES = [
    ('cpp_ordem_das_datas', '        return -1; // a antes de b',
     '        return 1; // mutacao: antes vale depois'),
    ('cpp_decidir_ofertas', '            if cpp1_b(e + 0xd) == 3 {',
     '            if cpp1_b(e + 0xd) == 4 { // mutacao: o tipo 3 nao decide',
     'cpp_1 decidir ofertas'),
    ('cpp_ofertas_do_clube', '    core::ptr::write_unaligned(saida as *mut u32, vetor as u32);\n    n as u8',
     '    core::ptr::write_unaligned(saida as *mut u32, vetor as u32);\n    (n as u8).saturating_sub(1) // mutacao: uma oferta a menos',
     'cpp_1 oferta do clube'),
]
