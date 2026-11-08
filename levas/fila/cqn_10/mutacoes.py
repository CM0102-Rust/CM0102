# Mutacoes (cqn_10), no dia das copas (Inglaterra e EUA, ligas da carreira de 8 ligas): a Copa da Liga sem a marca de
# refeita (+0x45 fica 0: a copa e refeita todo dia do ano, com inscritos, rodadas e sorteios de novo); as inscricoes da
# Copa da Inglaterra um dia depois (3/10 -> 4/10: a coleta sorteia, a ordem dos sorteios do dia muda); a convocacao do
# Jogo das Estrelas levada ao dia da semana 5 em vez do 6 (um dia antes: os elencos das selecoes mudam noutro dia). As
# tres rodam na janela (2001-205 a 2002-06-01). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqn10_copa_da_liga_refeita_todo_dia',
     'core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqn10: a Copa da Liga refeita uma vez',
     'core::ptr::write_unaligned((this + 0x45) as *mut u32, 0); // mutacao: refeita todo dia',
     'cqn_10 league cup rollover'),
    ('cqn10_inscricoes_da_copa_da_inglaterra_um_dia_depois',
     'if is_date_today(make_date_dword(3, 9, cqn10_w(hoje + 2) as u16 as u32, 0xffff_ffff), hoje) { // cqn10: as inscrições da Copa da Inglaterra em 3/10',
     'if is_date_today(make_date_dword(4, 9, cqn10_w(hoje + 2) as u16 as u32, 0xffff_ffff), hoje) { // mutacao: um dia depois',
     'cqn_10 fa cup entries'),
    ('cqn10_convocacao_um_dia_antes',
     'let d = make_date_dword(0x1a, 6, ano, 6); // cqn10: a convocação do Jogo das Estrelas',
     'let d = make_date_dword(0x1a, 6, ano, 5); // mutacao: o dia da semana 5',
     'cqn_10 all-star game call-up'),
]
