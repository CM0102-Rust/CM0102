# Mutacoes (cqh_8), todas na 0x7c5fe0 (as estatisticas arquivadas: a cada saida de clube, emprestimo e aposentadoria
# da carreira automatica ate 2002-06-01, centenas de vezes pelos contratos e pelo elenco). Os registros de 32 bytes da
# temporada (jogos, gols, notas) sao lidos pela carreira (as medias de nota, a forma); o que vai para o arquivo de
# historico e os jogos e gols pelo clube recontados (+0x20/+0x22) nao chegam a carreira ate 2002-06-01: as mutacoes
# neles escaparam em 02/11 (os jogos pelo byte +7 e o tipo do no zerado, ambas IGUAL A BASE) e foram trocadas.
# - O registro da categoria nao e zerado (as estatisticas da temporada seguem com o jogador no clube novo).
# - So a primeira metade do registro (16 bytes) e zerada.
# - A primeira categoria nao e arquivada nem zerada.
MUTACOES = [
    ('cqh8_sem_zerar', 'core::ptr::write_bytes(dados as *mut u8, 0, 32); // cqh8: o registro da categoria zerado',
     'let _ = dados; // mutacao: o registro da categoria zerado', 'cqh_8 categoria arquivada'),
    ('cqh8_zera_metade', 'core::ptr::write_bytes(dados as *mut u8, 0, 32); // cqh8: o registro da categoria zerado',
     'core::ptr::write_bytes(dados as *mut u8, 0, 16); // mutacao: o registro da categoria zerado', 'cqh_8 categoria arquivada'),
    ('cqh8_sem_a_primeira', 'let no = career_history_node(clube, Some(dados), valor, (((cat + 1) as u8) << 3) | tipo); // cqh8: a categoria e o tipo do nó',
     'if cat == 0 { continue; } let no = career_history_node(clube, Some(dados), valor, (((cat + 1) as u8) << 3) | tipo); // mutacao: a categoria e o tipo do nó', 'cqh_8 categoria arquivada'),
]
