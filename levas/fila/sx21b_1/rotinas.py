# A oferta de transferencia: o valor ponderado pelo vendedor e a taxa transformada em clausulas (transfer_offer.cpp;
# leitor sx21b, rodada 5, 24/11/2026). Rodam nas ofertas da carreira ENG (get_weighted_value 83.631 chamadas no mundo
# ate 2002-08-01; restructure_fee_into_clauses so pelo build_bid 0x8e5d60, no fim de um lance sem teto).
# Convencoes conferidas: 0x8e7b50 thiscall(so_taxa, sem_troca, avaliado, com_parcelas, com_venda) ret 0x14 (quadro com
# ebp, pilha alinhada a 8); 0x8e8330 thiscall(jogador, nao_usado, clube) ret 0xc (o terceiro argumento e reescrito
# com a taxa e lido de novo pelo fild).
# Chamadas: 0x4cf5a0 get_contract (guardado em 0xae1080); 0x6e86d0 match_man_judge_player_ability cdecl(tecnico,
# jogador, clube, modo, valor); 0x5a0590 get_club_finance; 0x5986b0 get_wealth_band (al com sinal, tabela de 4 saltos
# em 0x8e7edc para as faixas -2..1); 0x598800 get_transfer_budget (edx:eax, comparado com sinal a 2*taxa); 0x4d0c60
# calc_player_value; 0x68d1b0 get_next.
# Comparadores: o valor, G_CONTRACT_LOOKUP_RESULT e o sorteio lado a lado; a reestruturacao nos 0x41 bytes da oferta.
SAVE = False
ROTINAS = [
    ('transfer_offer_get_weighted_value', 0x8e7b50, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
    ('transfer_offer_restructure_fee_into_clauses', 0x8e8330, [0x8b, 0x44, 0x24, 0x04, 0x53]),
]
DOCS = ('a oferta de transferência: o valor ponderado pelo vendedor e a taxa transformada em cláusulas',
        "a transfer offer: its worth as the seller weighs it and the fee moved into clauses",
        [('transfer_offer_get_weighted_value', 'transfer_offer_get_weighted_value', '0x8e7b50', 'transfer_offer.cpp', 'thiscall, ret 0x14',
          'regra: a taxa; sem "só a taxa", a cláusula de jogos vezes (1 - jogos x 0,005), a de seleção (jogador sem jogos pela seleção, abaixo de 28 anos, se 65 x o futuro avaliado passa da reputação da nação) vezes a chance (idade, jogos e o excesso; no máximo 0,75), a venda futura pelo crescimento esperado (futuro - atual)/33; com parcelas, a faixa de riqueza do vendedor reduz as cláusulas (0,5 ou 0,75) e o total (0,1 a 0,9); os jogadores de troca pelo valor ou pela avaliação',
          "rule: the fee; without 'fee only', the appearance clause times (1 - games x 0.005), the international clause (capless player under 28 when 65 x the judged future beats the nation's reputation) times the chance (age, games and the excess; at most 0.75), the sell-on share by the expected growth (future - now)/33; with instalments, the seller's wealth band cuts the clauses (0.5 or 0.75) and the total (0.1 to 0.9); the exchange players by value or valuation"),
         ('transfer_offer_restructure_fee_into_clauses', 'transfer_offer_restructure_fee_into_clauses', '0x8e8330', 'transfer_offer.cpp', 'thiscall, ret 0xc',
          'regra: jogador com potencial 25 acima da habilidade, oferta sem cláusulas e clube com orçamento até o dobro da taxa: clube de reputação abaixo de 15 diante de jogador 3 mil pontos acima: taxa a 80% e 20% da venda futura; senão jogador sem jogos pela seleção (reputação acima de 4250, nação acima de 5000): cláusula de seleção de 30% em 10 jogos; senão jogador 4 mil pontos abaixo do clube: cláusula de 30% em 20 jogos; nos dois casos a taxa cai a 80%',
          "rule: a player 25 points of potential above his ability, an offer without clauses and a club with a budget of at most twice the fee: a club under 15 reputation facing a player 3 thousand points above it: fee at 80% and a 20% sell-on share; else a capless player (reputation above 4250, nation above 5000): a 30% international clause after 10 caps; else a player 4 thousand points below the club: a 30% clause after 20 games; either way the fee drops to 80%")])
CAMPOS = []
INTERESSA = []
