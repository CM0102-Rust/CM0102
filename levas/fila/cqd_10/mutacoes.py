# Mutacoes (cqd_10), todas na 0x7a59e0 (todo dia pelo laco do dia). Na carreira Rust ate 2002-06-01 da copia (265a +
# cqd_8..10): 256 anuncios e uns 18 jogadores aposentados de fato (16 com a vaga para o regen, 2 para o banco). As
# aposentadorias tiram jogadores dos elencos e dos contratos e poem as vagas dos regens.
# - O jovem que desiste da aposentadoria vai ate 35 anos em vez de 29 (os aposentados de clube, de 30 a 35, ficam).
# - O jogador aposentado sem registro de nao jogador fica com o contrato (sem a 0x4d0b50).
# - O jogador aposentado fica no elenco (sem a 0x7aace0).
MUTACOES = [
    ('cqd10_jovem_ate_35', '    if cqd10_b(p + 0x18) < 0x1e && cqd10_b(item + 0x18) != 8 // cqd10: o jovem não se aposenta',
     '    if cqd10_b(p + 0x18) < 0x24 && cqd10_b(item + 0x18) != 8 // mutacao: o jovem não se aposenta', 'cqd_10 aposentadoria desfeita'),
    ('cqd10_fica_com_o_contrato', '        cqd10_com(0x4d0b50, 0xae1050, p); // cqd10: tira o contrato',
     '        // mutacao: tira o contrato', 'cqd_10 tira o contrato'),
    ('cqd10_fica_no_elenco', '        elenco(p); // cqd10: sai do elenco',
     '        let _ = elenco; // mutacao: sai do elenco', 'cqd_10 sai do elenco'),
]
