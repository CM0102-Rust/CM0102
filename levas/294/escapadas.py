# As escapadas da 294a leva (explicadas pelo codigo; os contadores de ramo da base estao na 2a linha de cadeia.txt).
#
# Trocadas (poderiam discriminar; a mutacao caia num caminho que a base nao mediu): nma6_mantido_sem_torcida e
#   nma6_mantido_sem_antipatia (pelas marcas das respostas 3 e 2 das outras historias), nma8_diretoria_sem_resposta e
#   nma8_contrato_sem_negociar (pela historia ja respondida e pelo estado da conversa de contrato invertido). As novas
#   foram acusadas na bancada 5: ver trocas_prova.txt e as entradas abaixo.

_TEXTO_PT = (
    'texto de noticia nao entra no resultado da carreira (medido na 292a leva com duas sondas: formato invertido e '
    'linhas trocadas em ramos que rodaram na base, as duas com impressao igual a da base), e esta troca so muda '
    'texto: '
)
_TEXTO_EN = (
    'news text does not enter the result of the career (measured in the 292nd batch with two probes: a flipped '
    'format and swapped lines in branches that ran in the base, both with the same fingerprint as the base), and '
    'this swap only changes text: '
)

ESCAPADAS = {
    'nma6_mantido_sem_torcida': (
        'trocada por `nma6_marcas_da_resposta_3` (as marcas da resposta 3 as outras historias, 0x500 em vez de '
        '0x404): o ramo "nma_6 transfer interest kept" roda uma vez na base e atras de `club_favours_staff`, que a '
        'base nao mediu, e a confianca da torcida (+8 da linha do manager manager) nao e observada nem pela carreira '
        'nem pelo comparador, que desiste da peca das outras historias no ensaio '
        '(`player_story_stops_before_news`). O ramo "nma_6 other story" roda 256 vezes e a resposta 3, 128 '
        '(contador de sonda na bancada 5). Acusada pela carreira em 2001-234 (`trocas_prova.txt`)',
        'replaced by `nma6_marcas_da_resposta_3` (the flags of answer 3 to the other stories, 0x500 instead of '
        '0x404): the branch "nma_6 transfer interest kept" runs once in the base and behind `club_favours_staff`, '
        'which the base did not measure, and the supporters confidence (+8 of the manager manager row) is observed '
        'neither by the career nor by the comparer, which gives up on the other-stories piece in its dry run '
        '(`player_story_stops_before_news`). The branch "nma_6 other story" runs 256 times and answer 3, 128 (a probe '
        'counter on bench 5). Caught by the career on 2001-234 (`trocas_prova.txt`)'),
    'nma6_mantido_sem_antipatia': (
        'trocada por `nma6_marcas_da_resposta_2` (as marcas da resposta 2 as outras historias, 0x404 em vez de '
        '0x480), no mesmo ramo medido da anterior (a resposta 2 roda 8 vezes na sonda da bancada 5); a lista de '
        'antipatias do ramo "transfer interest kept" nao e observada, pelo mesmo motivo. Acusada pela carreira em '
        '2001-237 (`trocas_prova.txt`)',
        'replaced by `nma6_marcas_da_resposta_2` (the flags of answer 2 to the other stories, 0x404 instead of '
        '0x480), in the same measured branch as the one above (answer 2 runs 8 times in the bench 5 probe); the '
        'disliked list of the "transfer interest kept" branch is not observed, for the same reason. Caught by the '
        'career on 2001-237 (`trocas_prova.txt`)'),
    'nma8_diretoria_sem_resposta': (
        'trocada por `nma8_historia_respondida_responde` (a historia da midia ja respondida ainda pede resposta: o '
        'teste da casa de estado do fecho `story`, `!= 0` -> `== 0xff`): nenhum item de diretoria do tipo 0x1774 '
        'aparece na base (sonda na bancada 5: "probe board 1774" e "probe board 1786" com 0), enquanto o fecho das '
        'historias roda mais de 1.024 vezes e 128 delas com a casa de estado ja marcada. Escapou da carreira (o item '
        'passa a ficar na caixa, mas `auto_respond` nao age nele) e foi acusada pela comparacao da propria rotina: '
        '"get_action_button piece player (type 0xbd0) original=0 rust=2" (`trocas_prova.txt`)',
        'replaced by `nma8_historia_respondida_responde` (the answered media story still needs a reply: the state '
        'slot test of the `story` closure, `!= 0` -> `== 0xff`): no board item of type 0x1774 shows up in the base (a '
        'probe on bench 5: "probe board 1774" and "probe board 1786" both 0), while the stories closure runs over '
        '1,024 times and 128 of those with the state slot already set. It escaped the career (the item now stays in '
        'the inbox, but `auto_respond` does not act on it) and was caught by the routine own comparison: '
        '"get_action_button piece player (type 0xbd0) original=0 rust=2" (`trocas_prova.txt`)'),
    'nma8_contrato_sem_negociar': (
        'trocada por `nma8_contrato_estado_trocado` (o estado que pede negociar invertido: `!= 3` -> `== 3`), no '
        'mesmo ponto: a sonda da bancada 5 mostra 128 conversas de contrato chegando ao teste ("probe p bb8 found"), '
        'mas nenhuma com `contract_offer::STATE` 3, e por isso a troca antiga ("nunca negociar") era equivalente '
        'pelos valores possiveis; invertida, as 128 passam a pedir negociar. Acusada pela carreira em 2001-206 '
        '(`trocas_prova.txt`)',
        'replaced by `nma8_contrato_estado_trocado` (the state that needs a reply reversed: `!= 3` -> `== 3`), at the '
        'same spot: the bench 5 probe shows 128 contract talks reaching the test ("probe p bb8 found"), but none with '
        '`contract_offer::STATE` 3, which is why the old swap ("never negotiate") was equivalent by the possible '
        'values; reversed, those 128 now ask to negotiate. Caught by the career on 2001-206 (`trocas_prova.txt`)'),
    'nma3_manchete_vinculo': (
        _TEXTO_PT +
        '`player_story_headline` (a manchete de `Media::write_player_media_story_news_text` 0x72d270, com `body` 0) '
        'escreve a manchete no Text do chamador (`exe::text_set_string`) e pendura os vinculos '
        '(`exe::text_add_link`); a troca muda so o TIPO de um vinculo (o nome curto do clube de 0x7d5, clube, para '
        '0x7d7, pessoa), nao grava campo de banco, nao mexe em partida e nao consome sorteio. O unico observador e o '
        'comparador lado a lado do texto (`compare_text_writer`, que confere texto, vinculos e sorteios), e na janela '
        'da comparacao (ate 2001-08-20) o ramo nao roda: na carreira o contador "nma_3 headline" marcou 16, e no '
        'jogo real que a cadeia roda em seguida (mesmo codigo) a primeira chamada e em 2001-320 (16/11/2001). Que o '
        'comparador tem alcance para o tipo do vinculo esta provado na propria leva: a mutacao irma '
        '`nma4_vinculo_do_jogador` (0x7d7 -> 0x7d5) foi acusada com "text true links false" (provado pelo codigo)',
        _TEXTO_EN +
        '`player_story_headline` (the headline of `Media::write_player_media_story_news_text` 0x72d270, with `body` '
        "0) writes the headline into the caller's Text (`exe::text_set_string`) and hangs the links on it "
        '(`exe::text_add_link`); the swap changes only the TYPE of one link (the club short name from 0x7d5, a club, '
        'to 0x7d7, a person), writes no database field, touches no match and draws no random numbers. The only '
        'observer is the side-by-side text comparer (`compare_text_writer`, which checks text, links and draws), and '
        'in the comparison window (to 2001-08-20) the branch does not run: in the career the counter "nma_3 '
        'headline" read 16, and in the real game the chain runs next (same code) the first call is on 2001-320 (16 '
        'November 2001). That the comparer has the reach for a link type is proved in this very batch: the sister '
        'mutation `nma4_vinculo_do_jogador` (0x7d7 -> 0x7d5) was caught with "text true links false" (proved from the '
        'code)'),
    'nma4_veiculo_trocado': (
        _TEXTO_PT +
        '`praise_outlet` escreve o nome do veiculo (jornal, radio, revista, site, local ou estrangeiro com o '
        'adjetivo da nacao) num campo do quadro (`praise_frame::OUTLET`), e esse nome entra so no CORPO da noticia: '
        'a manchete (`praise_headline`, o caminho `body` 0 de `Media::write_player_praise_news_text` 0x744f80) nao '
        'usa o veiculo. O corpo nao roda na carreira: o contador "nma_4 praise body" ficou em 0 na base (foi por isso '
        'que `nma4_corpo_invertido` virou lacuna) e todas as chamadas da janela da comparacao sao com `body` 0, como '
        'mostram as mensagens das duas mutacoes irmas que a comparacao acusou '
        '("write_player_praise_news_text(kind 0x34, body 0, piece headline)"). Para pegar esta troca a comparacao '
        'precisa alcancar uma chamada com `body` diferente de 0, isto e o texto inteiro de um item de elogio '
        '(provado pelo codigo)',
        _TEXTO_EN +
        '`praise_outlet` writes the outlet name (newspaper, radio, magazine, website, local or foreign with the '
        "nation's adjective) into a field of the frame (`praise_frame::OUTLET`), and that name goes only into the "
        'BODY of the item: the headline (`praise_headline`, the `body` 0 path of '
        '`Media::write_player_praise_news_text` 0x744f80) does not use the outlet. The body does not run in the '
        'career: the counter "nma_4 praise body" stayed at 0 in the base (which is why `nma4_corpo_invertido` became '
        'a gap) and every call in the comparison window has `body` 0, as the messages of the two sister mutations the '
        'comparison did catch show ("write_player_praise_news_text(kind 0x34, body 0, piece headline)"). To catch '
        "this swap the comparison has to reach a call with `body` other than 0, that is a praise item's full text "
        '(proved from the code)'),
    'nma5_titulo_dois': (
        _TEXTO_PT +
        '`transfer_rumour_headline` (a manchete de `Media::write_transfer_rumour_news_text` 0x755860, com `body` 0) '
        'so escolhe o formato da manchete do boato pelo id do jogador e escreve no Text (`exe::text_set_string` e os '
        'vinculos); a troca de `% 3` para `% 2` muda qual das tres formas sai ("transfer speculation", "rumours", '
        '"lead chase for"), sem gravar campo, sem mexer em partida e sem consumir sorteio (o id nao e sorteado). O '
        'unico observador e `compare_text_writer`, e na janela da comparacao o ramo nao roda: na carreira o contador '
        '"nma_5 rumour headline" marcou 4 (quatro itens de boato em 19 meses, todos depois de 2001-08-20) e no jogo '
        'real da cadeia ele nao rodou ate 2001-327 (provado pelo codigo)',
        _TEXTO_EN +
        '`transfer_rumour_headline` (the headline of `Media::write_transfer_rumour_news_text` 0x755860, with `body` '
        "0) only picks the rumour headline's format from the player's id and writes it into the Text "
        '(`exe::text_set_string` and the links); changing `% 3` to `% 2` changes which of the three forms comes out '
        '("transfer speculation", "rumours", "lead chase for"), without writing any field, touching any match or '
        'drawing any random number (the id is not drawn). The only observer is `compare_text_writer`, and in the '
        'comparison window the branch does not run: in the career the counter "nma_5 rumour headline" read 4 (four '
        'rumour items in 19 months, all after 2001-08-20) and in the chain\'s real game it did not run up to '
        '2001-327 (proved from the code)'),
    'nma5_vinculo_tipo': (
        _TEXTO_PT +
        'na mesma manchete do boato (`transfer_rumour_headline`), a troca muda so o TIPO do vinculo do nome do '
        'jogador (0x7d7, pessoa, para 0x7d5, clube), depois de `exe::database_staff_matches_names` confirmar que os '
        'ids ainda batem; o texto sai com os mesmos bytes. Vale a mesma cadeia de `nma5_titulo_dois`: so '
        '`compare_text_writer` veria a diferenca (e ele ve tipo de vinculo, como mostra '
        '`nma4_vinculo_do_jogador`, acusada), e o ramo "nma_5 rumour headline" (4 na carreira) nao roda na janela da '
        'comparacao (provado pelo codigo)',
        _TEXTO_EN +
        'in the same rumour headline (`transfer_rumour_headline`), the swap changes only the TYPE of the player '
        "name's link (0x7d7, a person, to 0x7d5, a club), after `exe::database_staff_matches_names` confirms the ids "
        'still match; the text comes out with the same bytes. The same chain as `nma5_titulo_dois` applies: only '
        '`compare_text_writer` would see the difference (and it does compare link types, as '
        '`nma4_vinculo_do_jogador`, which was caught, shows), and the branch "nma_5 rumour headline" (4 in the '
        'career) does not run in the comparison window (proved from the code)'),
}  # nome: (explicacao pt, explicacao en)
