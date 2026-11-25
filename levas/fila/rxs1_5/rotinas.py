# O texto da notícia da transferência ou do empréstimo que caiu (transfer_manager.cpp; leitor rxs1, rodada 6,
# 25/11/2026). A rotina inteira (0x8de980..0x8df9af, 4 KB), igual no cm0102-1080.exe; contagem 0 na carreira ENG e no
# mundo até 2002-08-01 (o texto só é escrito quando alguém abre a notícia).
# Convenções conferidas na desmontagem: thiscall(this, notícia, corpo, Text) ret 0xc, sem quadro SEH; os nomes em 300
# bytes pelo 0x53e140; 0x6181d0 nation_get_nationality_adjective(nação do comprador, buffer, 100, 2) no motivo 0xd.
# Os 41 modelos e a ordem dos argumentos saíram da desmontagem (o passeio simbólico do rascunho rxs1/sym.py, conferido
# com o pseudocódigo do Ghidra), agrupados em 11 padrões (CollapsedArgs).
# Depende do rxs1_1 (news_text_name_n, news_text_link_name_n, news_text_club).
# Defeito do original (a cópia mantém): os motivos 0xc, 0xd e 0x10 citam o vendedor mesmo sem ele (vaga 0 = −1, o
# registro antes da tabela de clubes).
# Comparador: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_offer_collapsed_news_text', 0x8de980, [0x81, 0xec, 0xb0, 0x01, 0x00, 0x00])]
DOCS = ('o texto da notícia da transferência ou do empréstimo que caiu',
        'the news text of the collapsed transfer or loan',
        [('write_offer_collapsed_news_text', 'TRANSFER_MANAGER::write_offer_collapsed_news_text', '0x8de980', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título (empréstimo cancelado, ou um de quatro pelo resto dos números do comprador e da pessoa: vetado, não sai, mudança cancelada, não vai a lugar nenhum) ou o corpo "a transferência de <nome> (de <vendedor>) para <comprador> caiu", com o motivo pelo código da vaga 7 (empréstimo, diretoria, permissão de trabalho, verba, elenco, gente demais, prazo, compra entre clubes da mesma nação...) e os clubes que o modelo cita; os vínculos à pessoa e aos clubes',
          'the headline (loan cancelled, or one of four by the remainders of the buyer\'s and the person\'s ids: vetoed, falls through, move cancelled, going nowhere) or the body "<name>\'s proposed transfer (from <seller>) to <buyer> has collapsed", with the reason by the code of slot 7 (loan, board, work permit, funds, squad, too many staff, deadline, buying between clubs of the nation...) and the clubs the template names; the links to the person and the clubs')])
