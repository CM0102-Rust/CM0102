# Mutacoes (sma_3). As duas rotinas rodam no fim da temporada da nacao (0x447880: a foto da temporada de cada clube e
# as noticias de recorde batido, 0x448e30) e o resultado fica nos registros e nas noticias, que a carreira nao le de
# volta ate 2002-06-01: o comparador de cada uma (lado a lado, os 0x389 bytes da foto e os 0xe0 da noticia) acusa:
# - a foto sem o detentor do bloco (+0x283 e +0x2c2 ficam sem a Data construida);
# - o tipo do recorde gravado com um a mais no campo 1 da noticia.
MUTACOES = [
    ('sma3_foto_sem_detentor',
     'record_holder_construct(this + b + 8); // sma3: the holder of the block',
     'let _ = b; // mutation: the block without its holder',
     'sma_3 season snapshot'),
    ('sma3_tipo_do_recorde',
     'exe::newsitem_set_slot(news, 1, kind as i32 as u32); // sma3: the record type',
     'exe::newsitem_set_slot(news, 1, kind as i32 as u32 + 1); // mutation: the record type plus one',
     'sma_3 record news fields'),
]
