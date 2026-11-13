# A virada francesa e a troca entre o CFA e o National (fra_first.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo
# até 2002-08-01; docs/cobertura_mundo.txt: cada uma 1 vez, na virada francesa de meados de 2002).
# - 0x5bc780 (FraFirst::exchange_cfa_national_clubs): os clubes do CFA (divisão "uma divisão inferior", 0x9cf764, com
#   o CFA como divisão de reservas, +0x60) ordenados pela reputação e embaralhados de 4 em 4; sobem 6 na virada de 2001
#   e 4 depois (sem conferir quantos há: defeito do original se o CFA tiver menos [inferência]); os rebaixados do
#   National descem para a divisão inferior com o CFA como reserva.
# - 0x5bc4d0 (FraFirst::start_new_season): a cadeia (virtual +0xb0 com 0), a troca, a Primeira refeita (sem mexer nas
#   fases), o teto de divisão dos clubes do National, as viradas da Segunda e do National, previstas, expectativas,
#   árbitros; no fim constrói um FraCfa e um FraLower do ano novo e os destrói em seguida.
# Convenções: virtuais (mov ecx, esi; call [eax+n]; a +0xb0 com push 0, ret 4) -> thiscall; dentro: 0x944e46 (push n;
# add esp, 4 -> cdecl), 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), 0x6826d0 (push i -> thiscall, ret 4), 0x6830b0 e
# 0x6831a0 (3 pushes -> thiscall, ret 0xc), 0x68a980 (4 pushes -> thiscall, ret 0x10), 0x5b91c0 e 0x5bd600 (push
# ano -> thiscall, ret 4).
# Comparadores: a troca só a original (sorteia, move clubes), conferindo depois que os rebaixados do National estão na
# divisão inferior com o CFA de reserva; a virada só a original, com o ano +1.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('fra_first_exchange_cfa_national_clubs', 0x5bc780, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('fra_first_start_new_season', 0x5bc4d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a virada francesa e a troca entre o CFA e o National',
        'the French season turnover and the CFA and National exchange',
        [('fra_first_exchange_cfa_national_clubs', 'FraFirst::exchange_cfa_national_clubs', '0x5bc780', 'fra_first.cpp', 'thiscall, ret 0',
          'regra: os melhores clubes do CFA (pela reputação, embaralhados de 4 em 4) sobem para o National, seis na virada de 2001 e quatro depois; os rebaixados do National voltam para o CFA',
          'rule: the best CFA clubs (by reputation, shuffled in fours) go up to National, six in the 2001 turnover and four later; the clubs relegated from National go back to the CFA'),
         ('fra_first_start_new_season', 'FraFirst::start_new_season', '0x5bc4d0', 'fra_first.cpp', 'thiscall, ret 0',
          'regra: a virada francesa, puxada pela Primeira: acesso e rebaixamento, a troca entre o CFA e o National, a Primeira refeita (ano seguinte, regras, 18 clubes, jogos, ordem), o teto de divisão dos clubes do National, as viradas da Segunda e do National; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros; por fim um CFA e uma liga inferior do ano novo são montados e descartados em seguida',
          'rule: the French turnover, led by the First Division: promotion and relegation, the CFA and National exchange, the First Division rebuilt (next year, rules, 18 clubs, fixtures, order), the division tier of the National clubs capped, the turnovers of the Second Division and National; then the predicted positions, the board expectations and the referee renewal; finally a CFA and a lower league for the new year are built and dropped at once')])
CAMPOS = [('clube +0x57, +0x5b, +0x60 (divisão de reservas: o CFA); linha da tabela +0x37 (status 3 = rebaixado); valor/ponteiro; lê/grava',
           'club +0x57, +0x5b, +0x60 (reserve division: the CFA); table row +0x37 (status 3 = relegated); value/pointer; reads/writes')]
