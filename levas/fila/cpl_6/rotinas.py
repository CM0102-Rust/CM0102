# Mídia: o registro de histórias ([0xb6b7c8] Media +0, 23 bytes por história): registrar uma história
# (Media::log_story 0x757a50, pelas histórias de jogador 0x72c1f0, 0x733a60, 0x744240 e do time 0x74b060; aloca e
# realoca, qsort com 0x757ee0), expirar as antigas (Media::expire_story_log 0x757c70, pelo passe diário 0x5cdd9a;
# sorteia uma vez por história do time 0xbdf a cada passada) e se a história já foi contada (Media::has_logged_story
# 0x757da0, pelos destaques da partida 0x717470 e pela narração, já em Rust, e pelas histórias da mídia; bsearch
# 0x946464 com 0x757ee0). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem SEH; caixa só
# sem memória. A busca é chamada muitas vezes pelos destaques da partida: o contador só no ramo da história do time.
# Comparadores: o registrar pelo plano (a original aloca), o expirar lado a lado com o estado do sorteio, a busca pelo
# valor.
SAVE = False
ROTINAS = [('registrar_historia_da_midia', 0x757a50, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('expirar_historias_da_midia', 0x757c70, [0x83, 0xec, 0x24, 0x53, 0x8b, 0xd9]),
           ('historia_ja_registrada', 0x757da0, [0x83, 0xec, 0x20, 0x55, 0x56])]
DOCS = ('o registro de histórias da mídia: registrar uma história, expirar as antigas e se a história já foi contada',
        "the media story log: logging a story, expiring the old ones and whether a story was already told",
        [('registrar_historia_da_midia', 'Media::log_story', '0x757a50', 'media.cpp', 'thiscall, ret 0x14',
          'acrescenta a história (data de hoje, ids dos sujeitos ou −1, tipo, subtipo, resposta) e reordena (0x757ee0); cresce de 20 em 20',
          'appends the story (today, subject ids or −1, type, subtype, response) and sorts again (0x757ee0); grows by 20'),
         ('expirar_historias_da_midia', 'Media::expire_story_log', '0x757c70', 'media.cpp', 'thiscall, ret 0',
          'tira as histórias de mais de 100 dias (as do time, 0xbdf, de mais de 40 + sorteio(40)) pondo a última no lugar; reordena',
          'removes stories older than 100 days (team ones, 0xbdf, older than 40 + random(40)) moving the last one into the gap; sorts again'),
         ('historia_ja_registrada', 'Media::has_logged_story', '0x757da0', 'media.cpp', 'thiscall, ret 0x14',
          'a do time: procura em ordem por id do 2º sujeito e subtipo; as outras: busca binária pela chave e, com a data, se não é anterior',
          'team story: linear search by second subject id and subtype; the others: binary search by the key and, with a date, whether it is not older')])
CAMPOS = [('Media +0 registro de histórias (23 bytes: +0 data, +8 id do sujeito a ou −1, +0xc id do sujeito b ou −1, +0x10 resposta, +0x11 subtipo palavra, +0x13 tipo), +4 capacidade, +8 conta; grava (registrar_historia_da_midia, expirar_historias_da_midia), lê (historia_ja_registrada)',
           'Media +0 story log (23 bytes: +0 date, +8 subject a id or −1, +0xc subject b id or −1, +0x10 response, +0x11 subtype word, +0x13 type), +4 capacity, +8 count; writes (registrar_historia_da_midia, expirar_historias_da_midia), reads (historia_ja_registrada)'),
          ('[0xae2c90] data de hoje; valor; lê (registrar_historia_da_midia, expirar_historias_da_midia)',
           '[0xae2c90] current date; value; reads (registrar_historia_da_midia, expirar_historias_da_midia)')]
INTERESSA = ['SGE', 'GPF2']
