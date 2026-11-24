# Os textos das noticias de disputa, da reacao a multa e do retorno do tecnico sobre a multa (dispute_write_news_text
# 0x56a560, dispute.cpp; write_fine_reaction_news_text 0x5a76e0 e write_coach_fine_feedback_news_text 0x5a8090,
# Fine.cpp; leitor sx09c, rodada 5, 23/11/2026). As tres rotinas inteiras: 0x56a560..0x56aa23 (ret 0xc),
# 0x5a76e0..0x5a7e9b (ret 0xc; as duas tabelas de salto de 5 entradas em 0x5a7ea0 e 0x5a7eb4) e 0x5a8090..0x5a8379
# (ret 0xc).
# Chamadas: so pelo texto das noticias (NewsItem::write_text 0x77bc80, ja Rust: 0x781f38 a da disputa, 0x782042 a da
# reacao a multa 0xbc6, 0x7827bf a do retorno do tecnico 0x1394), ao abrir a noticia: TELAS.
# Convencoes conferidas na desmontagem: thiscall(objeto, noticia, corpo, Text) ret 0xc nas tres; o ecx so e usado pela
# da reacao (passa ao 0x5a73a0 fine_reason_text thiscall(fine, motivo, saida, 0x64, 1) ret 0x10, sx09b_7, pelo
# endereco). 0x785830 NewsItem::get_slot thiscall(noticia, k) ret 4 (as vagas lidas na ordem da original, inclusive as
# que nao sao usadas); 0x53e140 database_format_name_from_ids cdecl(chave1, chave2, chave3, saida, 0x190, modo, id)
# (modo 5 na manchete, 1 no corpo); 0x66f4e0 langlib_sprintf cdecl (na 0xde1f64, ou num buffer local de 0x1378 bytes
# na do tecnico, que tem 0x16c0 bytes de pilha com __chkstk 0x945610); 0x4ae660 Text::set_string; 0x53e990
# database_staff_matches_names cdecl(pessoa, chave1, chave2, chave3, 0); 0x4ae8a0 Text::add_link thiscall(Text,
# texto, 0x7da pessoa / 0x7d5 clube, objeto) ret 0xc. Os 0x64 a mais nos argumentos (modelos com {}/{s}) vao como na
# original.
# Prologos: 0x56a560 mov eax,[esp+8]; sub esp,0x1a0 (10); 0x5a76e0 sub esp,0x21c (6); 0x5a8090 mov eax,0x16c0 (5; o
# call __chkstk seguinte fica fora).
# Caminho no jogo (o Lucas prova jogando comparando): multar um jogador do proprio clube (perfil do jogador, acao
# Disciplina) e abrir a noticia da reacao dele; dias depois, a noticia do retorno do auxiliar sobre a multa; a disputa
# so quando um jogador multado ou insatisfeito vai a Comissao do Estatuto do Jogador (noticia de disputa). O modo
# CM_TEXTO_NOTICIAS (levas/reorganizacao/proximas/textos_noticias.md), quando entrar, poe as tres na carreira automatica.
TELAS = True
ROTINAS = [('dispute_write_news_text', 0x56a560, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xa0, 0x01, 0x00, 0x00]),
           ('write_fine_reaction_news_text', 0x5a76e0, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('write_coach_fine_feedback_news_text', 0x5a8090, [0xb8, 0xc0, 0x16, 0x00, 0x00])]
DOCS = ('os textos das noticias de disputa, da reacao a multa e do retorno do tecnico',
        'the news texts of a dispute, a fine reaction and the coach feedback',
        [('dispute_write_news_text', 'dispute_write_news_text', '0x56a560', 'dispute.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia de disputa: pelo resultado (vaga 9) rasgar o contrato, ficar no clube, retirar a queixa ou sair livre (manchete); a queixa pelo motivo (vaga 6: transferencia barrada, salario, maus-tratos, ataque verbal do tecnico), a queixa rejeitada, retirada ou aceita (corpo); liga o jogador e o clube',
          'writes the headline or body of a dispute news item: by the outcome (slot 9) tearing up the contract, told to remain, withdrawing the complaint or free to leave (headline); the complaint by its reason (slot 6: blocked move, wages, mistreatment, the manager\'s verbal attack), the complaint dismissed, withdrawn or upheld (body); links the player and the club'),
         ('write_fine_reaction_news_text', 'write_fine_reaction_news_text', '0x5a76e0', 'Fine.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da reacao a advertencia ou multa: contesta (pela transferencia futura, o contrato tipo 7 e a personalidade), o elenco protesta (pelo sorteio), aceita (pelo motivo, os efeitos e a personalidade) ou sem comentario; o motivo em texto pela fine_reason_text; liga o jogador',
          'writes the headline or body of the reaction to a warning or fine: objects (by the future transfer, the type 7 contract and the personality), the squad protests (by the draw), accepts (by the reason, the effects and the personality) or no comment; the reason as text by fine_reason_text; links the player'),
         ('write_coach_fine_feedback_news_text', 'write_coach_fine_feedback_news_text', '0x5a8090', 'Fine.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo do retorno do auxiliar tecnico sobre a multa: a disciplina ajudou o jogador (tres variantes) ou ele nao aguenta a pressao; liga o auxiliar',
          'writes the headline or body of the assistant manager\'s feedback on a fine: the discipline helped the player (three variants) or he cannot handle the pressure; links the assistant')])
