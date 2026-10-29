# Mutacoes (as duas primeiras: o comparando acusa lado a lado na primeira proposta/assinatura; a carreira tambem).
# A proposta pelo contrato atual vencendo hoje (o contrato do transferido ou emprestado acaba logo: renovacoes e
# saidas mudam). A felicidade do contrato novo em 10 (a 0x4cb0a0 sobe mais a moral depois de cada partida boa; a
# moral muda as queixas e os pedidos de transferencia; vale tambem para a 0x85e590, que usa a mesma parte). A faixa do
# vencimento com o limite do meio trocado (o passe diario 0x4c7ac0 decide renovacoes e saidas pela faixa; rotina
# quente, sem contador: o comparando acusa no primeiro contrato da faixa).
MUTACOES = [
    ('cpl_proposta_fim', 'core::ptr::write_unaligned((this + 0x2d) as *mut [u8; 8], core::ptr::read_unaligned(fim as *const [u8; 8])); // o fim do contrato',
     'core::ptr::write_unaligned((this + 0x2d) as *mut [u8; 8], core::ptr::read_unaligned(0xae2c90 as *const [u8; 8])); let _ = fim; // mutacao: vence hoje',
     'cpl_5 contrato pelo atual'),
    ('cpl_felicidade', 'cpl5_por_b(this + 0x3f, 0); // a felicidade',
     'cpl5_por_b(this + 0x3f, 10); // mutacao: 10', 'cpl_5 moral zerada'),
    ('cpl_faixa_meio', 'if m(0xae1010) { // depois do limite do meio',
     'if m(0xae1030) { // mutacao: o limite de baixo'),
]
