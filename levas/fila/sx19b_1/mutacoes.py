# Mutacoes (sx19b_1), pela carreira ENG ate 2002-06-01:
# - os jogos do elenco nao contam nas suspensoes (ninguem cumpre: os suspensos ficam fora e mudam as escalacoes);
# - so os cartoes do mandante vao para as regras (os visitantes nao levam suspensao).
# O destrutor fica sem mutacao (estrutural; so roda na limpeza do jogo).
MUTACOES = [
    ('sx19b_1_sem_cumprir',
     'exe::discipline_update_ban_progress(d, read_u32(p), read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 1); // sx19b_1: one match served',
     'exe::discipline_update_ban_progress(d, read_u32(p), read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0); // mutation: nothing served'),
    ('sx19b_1_so_o_mandante',
     'ruling_body_process_match_cards(this, row.addr(), fixture); // sx19b_1: the body\'s card rules',
     'if side == 0 { ruling_body_process_match_cards(this, row.addr(), fixture); } // mutation: only the home side',
     'sx19b_1 card rules'),
]
