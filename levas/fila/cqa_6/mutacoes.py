# Mutacoes (cqa_6), no jogador na busca (0x7af600), que a lista do olheiro do tecnico humano (0x7f7490) consulta a cada
# avaliacao de jogador e de clube interessado; a resposta chega a carreira (a cps8_sempre_na_lista da 255a foi acusada
# em 2001-195). As tres mexem na resposta por caminhos diferentes, cada uma com o contador colado no efeito:
# - a busca sem resultados passa a dizer que o jogador esta nela (pega se a lista do humano esteve vazia em alguma
#   consulta, como a cps8_sempre_na_lista);
# - o jogador achado passa a nao estar (pega se a lista teve algum dos jogadores avaliados);
# - o removido passa a contar (pega se algum resultado marcado como removido foi consultado; sem isso, lacuna).
MUTACOES = [
    ('cqa6_busca_vazia_tem_todos',
     'return 0; // cqa6: sem resultados, não está',
     'return 1; // mutacao: sem resultados, esta',
     'cqa_6 empty search'),
    ('cqa6_achado_nao_esta',
     '1 // cqa6: achado',
     '0 // mutacao: achado nao esta',
     'cqa_6 found'),
    ('cqa6_removido_conta',
     'if cqa6_b(r + 0x20) & 1 != 0 { // cqa6: a removida não conta',
     'if cqa6_b(r + 0x20) & 1 != 0 && false { // mutacao: a removida conta',
     'cqa_6 removed'),
]
