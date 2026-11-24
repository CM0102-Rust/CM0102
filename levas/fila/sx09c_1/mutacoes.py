# Mutacoes (sx09c_1), pela carreira ENG ate 2002-06-01 no modo Rust (os comparadores so rodam a original). As multas
# existem na carreira de ferias: o tecnico da IA multa o jogador que falta ao treino (AWOL, 0x41a60a/0x41b003).
# - a multa vencida nao sai da lista: as multas recentes contam na is_fine_justified (aceita ou contesta) e mudam o
#   moral, as reacoes e as noticias;
# - a tabela das multas por clube comeca em 50 em vez de 0: a primeira multa contestada ja passa do limite da diretoria
#   (fine_player: a confianca da diretoria no tecnico cai 250 a 500), o que muda as demissoes de tecnico da IA.
MUTACOES = [
    ('sx09c_multa_vencida_fica',
     'exe::llist_remove(fines, fine);',
     '// mutation: the expired fine stays',
     'sx09c_1 expired fine dropped'),
    ('sx09c_contagem_de_multas_50',
     'write_usize(this + 8, exe::crt_calloc(read_u32(G_CLUB_COUNT), 1)); // sx09c_1: the per-club fine counts',
     '{ let t = exe::crt_calloc(read_u32(G_CLUB_COUNT), 1); core::ptr::write_bytes(t as *mut u8, 50, read_u32(G_CLUB_COUNT) as usize); write_usize(this + 8, t); } // mutation'),
]
