# Mutacoes (nma_8). A rotina decide se a noticia pede resposta (2); a resposta automatica do humano de ferias
# (NewsItem::auto_respond 0x784100, chamada ao entregar a noticia) so age quando ela devolve 2, e o mark_item_read e o
# has_urgent_items tambem olham o 2:
# - a oferta por um jogador do humano (0xfa0) nunca pede resposta (as propostas ficam sem a resposta automatica);
# - o convite de amistoso (6) sempre fechado (os amistosos de pre-temporada nao sao respondidos);
# - a historia da midia ja respondida (0xbd0, 0xbd9, 0xbdd, 0xbdf) ainda pede resposta;
# - o estado que pede negociar na conversa de contrato (0xbb8) invertido (3 em vez de diferente de 3).
# As duas ultimas trocaram (294a leva) as do pedido a diretoria (0x1774) e as do contrato "nunca negociar": nenhum
# item desses tipos aparece com aquele estado na base, e as duas eram equivalentes pelos valores possiveis; ver
# levas/294/escapadas.py e levas/294/trocas_prova.txt.
MUTACOES = [
    ('nma8_oferta_sem_resposta',
     '&& o.status() == 0 { // nma8: a bid for his player needs a reply',
     '&& o.status() == 0xff { // mutation: the bid never needs a reply',
     'nma_8 transfer button'),
    ('nma8_amistoso_fechado',
     'if state == 3 || state == 4 { // nma8: the friendly invitation is closed',
     'if state != 0xff { // mutation: the friendly invitation always closed',
     'nma_8 general button'),
    ('nma8_historia_respondida_responde',
     'if !managed_by_me(club_slot) || read_u8(flag_slot_addr) != 0 {',
     'if !managed_by_me(club_slot) || read_u8(flag_slot_addr) == 0xff { // mutation: the answered story still needs a reply',
     'nma_8 player button'),
    ('nma8_contrato_estado_trocado',
     'if k == 0 || read_u8(k + contract_offer::STATE) != 3 { // nma8: the player\'s contract talks need a reply',
     'if k == 0 || read_u8(k + contract_offer::STATE) == 3 { // mutation: the state that needs a reply reversed',
     'nma_8 player button'),
]
