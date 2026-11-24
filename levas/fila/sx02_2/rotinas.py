# Os clubes humanos que recebem uma noticia de oferta (TRANSFER_MANAGER::collect_offer_news_recipients 0x8d1130,
# transfer_manager.cpp; leitor sx02, rodada 5, 23/11/2026). Segunda rotina do transfer_manager.cpp que mais roda na
# carreira ENG (4.928 chamadas ate 2002-06-01): chamada pelas noticias de oferta (post_offer_made_news 0x3e,
# post_offer_rejected_news 0x2d/0x2e, 0x2f, 0x5f...).
# Convencoes conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, oferta, bandeiras, vetor de saida, maximo), ret 0x10, devolve al (a conta);
# - 0x842180 (collect_interested_clubs): thiscall(shortlist, alvo, lista, 1, 1, 1), ret 0x14;
# - 0x849ce0 (find_club_shortlist_entry): thiscall(shortlist, comprador, alvo, 0, 0), ret 0x10;
# - 0x8e7b50 (TRANSFER_OFFER::get_weighted_value): thiscall(oferta, 0, 0, 0, 0, 1), ret 0x14 (push 1; push 0 x4);
# - 0x540a50 (club_get_linked_b_team): cdecl(club_job, &bandeira, 1); 0x53b620 (is_player_job): cdecl(byte);
# - 0x5407e0 (get_staff_reputation_for_viewer): cdecl(alvo, &pessoa na pilha, alvo sem ficha), devolve ax (movsx);
# - 0x945640 (_ftol) nas reputacoes x 0.25 (cargos 8 a 10) e no bonus 3000 - valor x 0.0006;
# - a pessoa na pilha (esp+0x64, 0x6e bytes): 5 Date::Date (0x54a110) em +0x10, +0x29, +0x31, +0x3e, +0x46; id, clube,
#   nacao, segunda nacao, ficha e nao-jogador escritos; o resto nao e iniciado pelo original e a 0x5408c0 nao o le (a
#   copia zera).
# Comparador: lado a lado com G_CONTRACT_LOOKUP_RESULT e o vetor de saida (max entradas).
SAVE = False
ROTINAS = [('collect_offer_news_recipients', 0x8d1130, [0x6a, 0xff, 0x68, 0xe2, 0x40, 0x96, 0x00])]
DOCS = ('os clubes humanos que recebem uma notícia de oferta',
        'the human clubs that receive news of an offer',
        [('collect_offer_news_recipients', 'TRANSFER_MANAGER::collect_offer_news_recipients', '0x8d1130', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'regra: pelas bandeiras: 2, o vendedor humano; 1, o comprador humano; 4, com empréstimo noutro clube, o clube ligado ao cargo do jogador (ou o próprio), humano ou não, e os compradores humanos das ofertas rivais abertas; 8, com esse empréstimo, o clube do cargo, e os clubes humanos que observam o alvo, menos o comprador; 0x10 (com 0x20, só se o jogador soube da oferta), cada um dos 16 técnicos humanos (fora o comprador sem 1 e o vendedor sem 2) pelo alcance: as reputações do alvo (um quarto para os cargos 8 a 10) contra limites somados a 3000 − valor da oferta × 0,0006 (500 num pré-contrato), até 9750: mundial acima de 9200, todos; acima de 8300, o continente do alvo, do vendedor ou do comprador; nacional ou atual acima de 7200, a nação do alvo, do vendedor e a do comprador (por um nome mundial, um valor de (nível da liga − 10) × 500 mil ou mais de 20 jogos por uma seleção acima de 5500), Inglaterra e Escócia se alcançam; senão em forma acima de 3000 com 0x80, a nação do vendedor; senão com 0x40, a divisão do clube do alvo e a do comprador (jogador observado pelo comprador por um milhão ou mais, ou a reputação vista do clube comprador acima da da divisão × 350). No fim as repetições saem, na ordem',
          "rule: by the flags: 2, the human seller; 1, the human buyer; 4, with a loan at another club, the club linked to the player's club job (or the job's club), human or not, and the human buyers of the open rival offers; 8, with that loan, the club job's club, and the human clubs shortlisting the target, except the buyer; 0x10 (with 0x20, only when the player knows of the offer), each of the 16 human managers (not the buyer without 1 nor the seller without 2) by the reach: the target's reputations (a quarter for jobs 8 to 10) against thresholds raised by 3000 − offer value × 0.0006 (500 for a pre-contract), at most 9750: world above 9200, everyone; above 8300, the target's, seller's or buyer's continent; home or current above 7200, the target's nation, the seller's, and the buyer's (for a world name, a value of (league standard − 10) × 500 thousand, or over 20 caps for a national side above 5500), England and Scotland reaching each other; otherwise in form above 3000 with 0x80, the seller's nation; otherwise with 0x40, the division of the target's club and the buyer's (a player shortlisted by the buyer for a million or more, or the reputation seen from the buying club above the division's × 350). At the end the duplicates go, keeping the order")])
CAMPOS = []
INTERESSA = []
