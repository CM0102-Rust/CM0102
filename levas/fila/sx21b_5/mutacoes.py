# Mutacoes (sx21b_5) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao
# contra a base (a mesma DLL sem as rotinas da leva), e a carga do FeriasTesteSave (CARGA). Sem o 4o campo (contador).
# - as entradas da lista de ofertas rivais gravadas com 3 bytes: o transfer.dat (o bloco das ofertas) sai menor e
#   deslocado;
# - as listas de rivais nao compartilhadas na carga: cada oferta fica com a sua lista (a mesma lista do jogador deixa
#   de ser uma so); a impressao da carga acusa quando um lance rival e avaliado depois.
MUTACOES = [
    ('sx21b_5_rival_3_bytes',
     '(read_usize(this), 4u32, 0x102, 0x110)',
     '(read_usize(this), 3u32, 0x102, 0x110)'),
    ('sx21b_5_sem_compartilhar',
     '                    write_u32(this, read_u32(earlier));',
     '                    let _ = earlier; // mutation: not shared'),
]
