# Mutacoes (sx19_17), pela carreira ENG ate 2002-06-01:
# - a entrada nova com o id do jogador seguinte (o comparador da entrada nova acusa na primeira; na carreira, a volta
#   da temporada e a limpeza dos cartoes por clube leem o id);
# - a suspensao nova sem os jogos que sobravam da antiga (jogador suspenso de novo antes de cumprir a primeira);
# - desligar sem tirar a suspensao (a entrada fica apontando para a suspensao solta: o "suspenso?" da escalacao muda).
MUTACOES = [
    ('sx19_17_id_seguinte',
     'write_u32(e + ruling_body_entry::PERSON_ID, id); // sx19_17: the new entry\'s player',
     'write_u32(e + ruling_body_entry::PERSON_ID, id + 1); // mutation: the next player\'s id',
     'sx19_17 entry added'),
    ('sx19_17_sem_somar_jogos',
     '} else if read_i8(o + ban_record::MATCHES) > 0 { // sx19_17: the old ban\'s matches go onto the new one',
     '} else if read_i8(o + ban_record::MATCHES) > 0 && id == u32::MAX { // mutation: the old matches dropped',
     'sx19_17 ban merged'),
    ('sx19_17_desligar_nada',
     'write_i16(e + ruling_body_entry::BAN_INDEX, -1); // sx19_17: the entry without a ban',
     'let _ = e; // mutation: the ban kept',
     'sx19_17 ban detached'),
]
