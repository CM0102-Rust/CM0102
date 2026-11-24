# O orgao internacional (FIFA): as regras dos cartoes e a virada da temporada (rb_international.cpp; leitor sx18,
# rodada 5, 24/11/2026). Usa plain_card_verdict, run_card_rules e compare_card_rules do sx18_1: integrar depois dele.
# As regras rodam na carreira ENG nas partidas das selecoes (amistosos e eliminatorias, rodadas FIFA); a virada roda
# quando uma competicao com a regra 0 vira a temporada (discipline_new_season 0x556210, vtable +0): pode ficar fora da
# janela ate 2002-06-01 (passageira).
# Convencoes conferidas na desmontagem:
# - 0x7df1c0 thiscall(linha do registro, partida) ret 8: o vermelho (+0x2d == 1) da 1 jogo (motivo 2, quantidade 1);
#   cada partida com amarelo soma 1 na entrada e a cada segundo da 1 jogo (motivo 1, quantidade 2); regras ff ff ff 00;
#   comeca hoje; 0x556940 add_ban e 0x557280 post_ban_news;
# - 0x7df3c0 thiscall() ret 0: 0x556d00 release_ban thiscall(id, indice) ret 8 (o indice com os 16 bits altos de lixo:
#   a rotina le so a palavra); 0x55cb20 set_slot thiscall(id, vaga, regra, -1); 0x7e9050 compact_entries.
# Comparadores: as regras pelo veredito sobre a copia da entrada (compare_card_rules); a virada so a original, com a
# lista das entradas que ficam conferida depois da compactacao.
SAVE = False
ROTINAS = [('rb_international_process_match_cards', 0x7df1c0, [0x6a, 0xff, 0x68, 0xa8, 0x01, 0x96, 0x00]),
           ('rb_international_start_new_season', 0x7df3c0, [0x55, 0x56, 0x57, 0x8b, 0xf9])]
DOCS = ('as regras dos cartões e a virada da temporada do órgão internacional',
        'the international ruling body\'s card rules and new season',
        [('rb_international_process_match_cards', 'RbInternational::process_match_cards', '0x7df1c0', 'ruling_body/rb_international.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da FIFA: um jogo pelo vermelho e um a cada segundo amarelo (um por partida), a partir de hoje, na quarta regra, com a notícia',
          'applies a player\'s cards of a match under the FIFA rules: one match for a red card and one for every second booking (one per match), from today, in the fourth rule, with its news'),
         ('rb_international_start_new_season', 'RbInternational::start_new_season', '0x7df3c0', 'ruling_body/rb_international.cpp', 'thiscall',
          'na virada da temporada, libera as entradas sem suspensão e sem cartões e solta as suspensões de 4 anos ou mais (liberando a entrada); as outras ficam como estão (sem zerar os cartões); depois compacta a tabela',
          'at the new season, frees the entries with no ban and no cards and releases bans 4 or more years old (freeing the entry); the others stay as they are (their cards are not cleared); then compacts the table')])
CAMPOS = []
INTERESSA = []
