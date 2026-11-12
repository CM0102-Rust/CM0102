# Mutacoes (nma_8). A rotina decide se a noticia pede resposta (2); a resposta automatica do humano de ferias
# (NewsItem::auto_respond 0x784100, chamada ao entregar a noticia) so age quando ela devolve 2, e o mark_item_read e o
# has_urgent_items tambem olham o 2:
# - a oferta por um jogador do humano (0xfa0) nunca pede resposta (as propostas ficam sem a resposta automatica);
# - o convite de amistoso (6) sempre fechado (os amistosos de pre-temporada nao sao respondidos);
# - o pedido a diretoria (0x1774) nunca pede resposta;
# - a conversa de contrato de um jogador (0xbb8) nunca pede negociar.
MUTACOES = [
    ('nma8_oferta_sem_resposta',
     '&& o.status() == 0 { // nma8: a bid for his player needs a reply',
     '&& o.status() == 0xff { // mutation: the bid never needs a reply',
     'nma_8 transfer button'),
    ('nma8_amistoso_fechado',
     'if state == 3 || state == 4 { // nma8: the friendly invitation is closed',
     'if state != 0xff { // mutation: the friendly invitation always closed',
     'nma_8 general button'),
    ('nma8_diretoria_sem_resposta',
     'if read_u8(s(1)) == 0 { // nma8: the board request needs a reply',
     'if read_u8(s(1)) == 0xff { // mutation: the board request never needs a reply',
     'nma_8 board button'),
    ('nma8_contrato_sem_negociar',
     'if k == 0 || read_u8(k + contract_offer::STATE) != 3 { // nma8: the player\'s contract talks need a reply',
     'if k == 0 || read_u8(k + contract_offer::STATE) != 0xff { // mutation: never negotiate',
     'nma_8 player button'),
]
