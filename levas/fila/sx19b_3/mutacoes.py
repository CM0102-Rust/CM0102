# Mutacoes (sx19b_3), pela prova do save (prova_save.py --leva N, mutacoes.py --prova-save N):
# - a gravacao sem as suspensoes pendentes, contagem 0 e nenhum no (o save fica coerente; o discipline.dat do .qst muda; so acusa se o save de ferias tiver alguma
#   suspensao por pontos pendente no dia da gravacao);
# - a carga religando o mandante pelo id do visitante (a impressao do save carregado muda nas pendentes; mesma ressalva).
MUTACOES = [
    ('sx19b_3_sem_pendentes',
     'let pending: u32 = if list.is_null() { 0 } else { list.count() as u16 as u32 }; // sx19b_3: the pending ban count',
     'let pending: u32 = 0; // mutation: no pending bans written',
     'sx19b_3 pending ban saved'),
    ('sx19b_3_mandante_trocado',
     'f.set_home_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.home_club_id().wrapping_mul(club::SIZE)))); // sx19b_3: the home club relinked',
     'f.set_home_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.away_club_id().wrapping_mul(club::SIZE)))); // mutation: the away id',
     'sx19b_3 pending ban loaded'),
]
