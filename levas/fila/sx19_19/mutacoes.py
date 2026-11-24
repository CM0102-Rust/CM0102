# Mutacoes (sx19_19), pela carreira ENG ate 2002-06-01 (os vermelhos ingleses e europeus deixam suspensoes por pontos
# pendentes):
# - a pendente somando um jogo a mais quando o jogador ja esta suspenso;
# - a suspensao nova sem a data de inicio da pendente (fica a data do vermelho).
MUTACOES = [
    ('sx19_19_jogo_a_mais',
     'write_u8(ban + ban_record::MATCHES, if old == 0xff { matches } else { old.wrapping_add(matches) }); // sx19_19: the pending matches added',
     'write_u8(ban + ban_record::MATCHES, if old == 0xff { matches } else { old.wrapping_add(matches).wrapping_add(1) }); // mutation: one match more',
     'sx19_19 pending ban merged'),
    ('sx19_19_data_do_vermelho',
     'copy_record_date(node + ban_record::START_DATE, node + 0x14); // the pending ban starts on its date',
     '// mutation: the red card date kept',
     'sx19_19 pending ban due'),
]
