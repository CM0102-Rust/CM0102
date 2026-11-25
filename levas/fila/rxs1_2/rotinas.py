# Os textos das notícias do jogador tirado da troca, da transferência parada pelo prazo da nação e da oferta de
# empréstimo (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026). As três inteiras (0x8de320..0x8de78f,
# 0x8e1370..0x8e177f, 0x8e19c0..0x8e1f0f), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01 (os textos só são escritos quando alguém abre a notícia).
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc; 0x8de320 e 0x8e19c0 com
# quadro SEH (a quantia; o contrato e a data). 0x786d30 News::set_item_slot thiscall([0xb7431c], técnico humano
# [0xb74c26 + 0x300 × word [0xb77c7a]], [notícia+0xd6], vaga, valor, último) ret 0x14: o último é 0 na 0x8e1370 e a
# própria notícia na 0x8e19c0. 0x6181d0 nation_get_nationality_adjective(nação, buffer, 100, 10); 0x85e410
# STAFF_CONTRACT::STAFF_CONTRACT num local de 0x50 bytes, o salário em +0x10, 0x85e750 format_wage_text(buffer, 2, 2);
# 0x549ef0 Date::Date(dia, ano) e 0x54ba20 (200, 0xff); a quantia da 0x8de320 em 100 bytes (0x441c10 com 0x64).
# Depende do rxs1_1 (news_text_name_n, news_text_link_name_n, news_text_club). Os auxiliares news_offer_at,
# news_store_item_slot e news_text_money_100 são pub(crate) para os itens seguintes.
# Defeito do original (0x8e1370, a cópia mantém): nos estados 0 e 3 a frase "nova oferta de <clube>" é formatada no
# buffer e nunca acrescentada ao texto.
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer). A 0x8e1370 e a 0x8e19c0 podem gravar uma
# vaga da notícia antes do texto e rodam duas vezes na comparação: a segunda acha o estado já gravado (como a
# write_loan_negotiation_news_text do rx05_9).
SAVE = False
ROTINAS = [('write_exchange_player_removed_news_text', 0x8de320, [0x6a, 0xff, 0x68, 0xfb, 0x45, 0x96, 0x00]),
           ('write_offer_on_hold_news_text', 0x8e1370, [0x81, 0xec, 0xa4, 0x01, 0x00, 0x00]),
           ('write_loan_offer_news_text', 0x8e19c0, [0x6a, 0xff, 0x68, 0x36, 0x47, 0x96, 0x00])]
DOCS = ('os textos das notícias do jogador tirado da troca, da transferência parada pelo prazo da nação e da oferta de empréstimo',
        "the news texts of the player removed from an exchange deal, of the transfer stopped by a nation's deadline and of the loan offer",
        [('write_exchange_player_removed_news_text', 'TRANSFER_MANAGER::write_exchange_player_removed_news_text', '0x8de320', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<nome> sai do negócio" ou o corpo "<clube> tirou <jogador> da troca por <alvo> (não paga o salário)" ou "<clube> não quer <jogador> na troca por <alvo>", com "o valor passa a <quantia>" quando o valor novo é maior que o antigo; os vínculos aos clubes e às duas pessoas',
          "the headline \"<name> removed from transfer deal\" or the body \"<club> have removed <player> from the part exchange deal for <target> (cannot afford his wages)\" or \"<club> do not want <player> in part exchange for <target>\", with \"the fee will be adjusted to <money>\" when the new fee is above the old one; the links to the clubs and both people"),
         ('write_offer_on_hold_news_text', 'TRANSFER_MANAGER::write_offer_on_hold_news_text', '0x8e1370', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "transferência de <nome> para <clube> parada" ou o corpo "a transferência (de <vendedor>) para <comprador> não pode seguir agora pelo prazo de transferências <nacionalidade>"; o estado desconhecido é procurado na oferta e guardado na notícia',
          "the headline \"<name> transfer to <club> put on hold\" or the body \"the proposed transfer (from <seller>) to <buyer> cannot proceed due to the <nationality> transfer deadline\"; an unknown state is looked up in the offer and stored in the news item"),
         ('write_loan_offer_news_text', 'TRANSFER_MANAGER::write_loan_offer_news_text', '0x8e19c0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "oferta de empréstimo de <clube> por <nome>" ou o corpo "<clube> ofereceu levar <nome> (do <clube de origem>) emprestado até <data>", o salário que pagam quando o jogador é do humano e a resposta (aceitou, recusou, quer negociar); a resposta desconhecida é procurada na oferta e guardada na notícia',
          "the headline \"<club> loan offer for <name>\" or the body \"<club> have made an offer to loan (<parent>'s) <name> until <date>\", the wage they pay when the player is the human's and the answer (accepted, rejected, negotiate); an unknown answer is looked up in the offer and stored in the news item")])
