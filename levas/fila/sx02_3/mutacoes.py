# Mutacoes (sx02_3), na dispensa (0x8df9b0, 6 chamadas ate 2002-06-01): a oferta pelo dispensado que nao e cancelada
# volta em 10 dias no status em que estava (a transferencia de um jogador sem contrato segue ou a oferta e respondida de
# novo); o dispensado que fica na troca entra na conta da oferta. Acusadas pela carreira so se uma das 6 dispensas tiver
# oferta aberta (o comparador e so a original). A noticia da resposta (0x8d8e90) nao tem mutacao propria: o texto nao
# volta para a simulacao e o comparador e so a original (como as outras noticias postadas do transfer_manager).
MUTACOES = [
    ('sx02_3_oferta_nao_cancelada',
     'o.set_status(offer_status::CANCELLED); // sx02_3: the offer cancelled',
     'let _ = offer_status::CANCELLED; // mutation: the offer keeps its status',
     'sx02_3 released player offers'),
    ('sx02_3_fica_na_troca',
     'remove(o.addr(), player.addr()); // sx02_3: the player leaves the exchange',
     'let _ = remove; // mutation: the released player stays in the exchange',
     'sx02_3 released player offers'),
]
