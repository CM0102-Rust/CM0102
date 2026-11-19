# The German season turnover (ger_first.cpp). Reader r3_mwcd (19/11/2026), round 2 (1 call each, the turnover of
# summer 2002; the Regionalliga is not loaded in the test, so 0x5dbf50 and 0x5db980 do not run and 0x5db7b0 does).
# - 0x5db4a0 GerFirst::start_new_season: top_division_turnover (batch 305) with, before, 0x5dbf50 (Regionalliga only),
#   the chain (push 1), 0x5db980 or 0x5db7b0 (by the Regionalliga object) and 0x5dbbd0 (the north/south split, 896
#   bytes, x87 by latitude: not ported, left for an item of its own); after, the 2. Bundesliga's tier limit (1, 3, -3,
#   1) and turnover (no null check) and the Regionalliga's (with the null check), instruction by instruction.
# - 0x5db6c0 GerFirst::process_promotion_relegation_chain: like the Turkish chain (mwd_24); thiscall, ret 4; without the
#   Regionalliga it returns [0xadadfc] (what is left in eax).
# - 0x5db7b0 ger_first_exchange_regional_clubs_unplayed: like the Danish refresh (mwc_20) without the nation compare
#   (only a nation) and with the B-team check (0x540a50, cdecl(club, &flag, 1): a partner and flag 0 leaves the club
#   out; one stack slot shared by the calls); picks 4 even when n < 4 (defect of the original, kept). The map says
#   cdecl(void) but ecx (the Bundesliga) is the this of 0x6830b0 and 0x6831a0: thiscall, ret 0.
# Comparers: the turnover only the original (the year +1); the chain only the original (as the Turkish one); the
# exchange only the original, the 2. Bundesliga's status-3 clubs checked in the Regionalliga division (with at least 4
# candidates).
# Mutation: the draw among the best 24 instead of 12 (who goes up to the 2. Bundesliga changes: the clubs' divisions in
# the database and the 2002/03 fixtures change).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('ger_first_start_new_season', 0x5db4a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_first_process_promotion_relegation_chain', 0x5db6c0, [0xa1, 0xfc, 0xad, 0xad, 0x00, 0x56]),
           ('ger_first_exchange_regional_clubs_unplayed', 0x5db7b0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('a virada da temporada alemã', 'the German season turnover',
        [('ger_first_start_new_season', 'GerFirst::start_new_season', '0x5db4a0', 'ger_first.cpp', 'thiscall, ret 0',
          'regra: a virada alemã, puxada pela Bundesliga: o acesso e o rebaixamento pela cadeia; a troca de clubes entre a 2. Bundesliga e a Regionalliga; a Regionalliga partida em norte e sul; a Bundesliga refeita para o ano seguinte (regras, tabela, jogos, ordem); o nível de divisão dos clubes da 2. Bundesliga limitado e as viradas dela e da Regionalliga (quando carregada); as posições previstas, as expectativas da diretoria e os árbitros',
          'rule: the German turnover, led by the Bundesliga: promotion and relegation along the chain; the clubs exchanged between the 2. Bundesliga and the Regionalliga; the Regionalliga split north and south; the Bundesliga rebuilt for the next year (rules, table, fixtures, order); the 2. Bundesliga clubs\' division tier capped and the 2. Bundesliga and the Regionalliga (when loaded) turned over; the predicted positions, the board expectations and the officials'),
         ('ger_first_process_promotion_relegation_chain', 'GerFirst::process_promotion_relegation_chain', '0x5db6c0', 'ger_first.cpp', 'thiscall, ret 4',
          'regra: o acesso e o rebaixamento alemães: as posições finais da Bundesliga e da 2. Bundesliga registradas e os clubes marcados trocados entre elas; com a Regionalliga carregada, também entre a 2. Bundesliga e a Regionalliga e o seu primeiro grupo',
          'rule: German promotion and relegation: the final positions of the Bundesliga and the 2. Bundesliga recorded and their marked clubs swapped; with the Regionalliga loaded, also between the 2. Bundesliga and the Regionalliga and its first group'),
         ('ger_first_exchange_regional_clubs_unplayed', 'ger_first_exchange_regional_clubs_unplayed', '0x5db7b0', 'ger_first.cpp', 'thiscall (ecx = a Bundesliga), ret 0',
          'regra: na virada alemã sem a Regionalliga carregada, os quatro melhores clubes da divisão da Regionalliga (ordenados por força, os doze melhores embaralhados; times B fora) sobem à 2. Bundesliga, e os rebaixados (situação 3) dela descem; sem candidato ninguém sobe nem desce (com menos de quatro, o original move também posições não escritas da lista: defeito)',
          'rule: at the German turnover without the Regionalliga loaded, the four best clubs of the Regionalliga division (sorted by strength, the best twelve shuffled; B teams left out) go up to the 2. Bundesliga, and its relegated clubs (status 3) go down; with no candidate nobody moves (with fewer than four, the original also moves unwritten slots of the list: a defect)')])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x40; clube +0x53 nação, +0x57 divisão; linha da tabela +0x37 situação', 'league +0x4c, +0xba, +8, +0xc, +0x40; club +0x53 nation, +0x57 division; table row +0x37 status')]
