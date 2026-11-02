# Mutacoes (cqh_8), todas na 0x7c5fe0 (as estatisticas arquivadas: a cada saida de clube, emprestimo e aposentadoria
# da carreira automatica ate 2002-06-01, centenas de vezes pelos contratos e pelo elenco).
# - O registro da categoria nao e zerado (as estatisticas da temporada seguem com o jogador no clube novo).
# - Os jogos pelo clube somados do byte +7 do no em vez do +6.
# - O tipo do no (emprestimo, fim, outro) perdido: sempre 0 (o historico da carreira, 0x7c3e40, separa os grupos pelo
#   tipo).
MUTACOES = [
    ('cqh8_sem_zerar', 'core::ptr::write_bytes(dados as *mut u8, 0, 32); // cqh8: o registro da categoria zerado',
     'let _ = dados; // mutacao: o registro da categoria zerado', 'cqh_8 categoria arquivada'),
    ('cqh8_jogos_do_byte_7', 'core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(*((no + 6) as *const u8) as u16)); // cqh8: os jogos pelo clube',
     'core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(*((no + 7) as *const u8) as u16)); // mutacao: os jogos pelo clube', 'cqh_8 jogos pelo clube'),
    ('cqh8_tipo_zero', 'let no = cqh8_no(clube, Some(dados), valor, (((cat + 1) as u8) << 3) | tipo); // cqh8: a categoria e o tipo do nó',
     'let no = cqh8_no(clube, Some(dados), valor, ((cat + 1) as u8) << 3); // mutacao: a categoria e o tipo do nó', 'cqh_8 categoria arquivada'),
]
