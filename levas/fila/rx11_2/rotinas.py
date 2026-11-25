# Os registros de carreira do destino de uma seleção nas eliminatórias africanas e da CONCACAF e as honras da temporada
# das seções das eliminatórias (wc_african_cup.cpp, wc_concacaf_cup.cpp; leitor rx11, rodada 6, 25/11/2026). Simulação
# na campanha de 2006 (2004 e 2005 na carreira de teste): LIGAS = 'ENG' e ATE = '2005-12-31', como os cx16_7..12; a
# Copa de 2006 é semeada em dezembro de 2005 (quem entra nela muda a carreira).
# - 0x90fbf0 WcAfricanCup::update_staff_records_for_status thiscall(clube, desfecho, fase, saida, jogo, a6) ret 0x18
#   (virtual +0x44): fase -1: 1 classificação (0, 0x14, 0x19), 2 eliminação (0x86c000 com +0x32/+0x30 do jogo, 0x19) e
#   fim da campanha (0x775000); grupos (pelo formato +0xba: fase < 5, 4, 6, 4): 7 a Copa (0x86bdd0 com o registro da
#   Copa, 0x64; 0x92e740; *saida = 4; 0x7779b0) e -1; 6 classificação (0, 0x96, 0x2d); 0xfe não classificado (0x86c1d0:
#   0x14, 0x2d) e fim; repescagem (formato 2 fase 6, formato 3 fase 4): 1 a Copa (0x37) e -1; 2 eliminação (0x64).
#   Prólogo mov dl, [esp+0xc]; push esi (5 bytes).
# - 0x915710 WcConcacafCup::update_staff_records_for_status (mesma assinatura): fase -1 (mata-mata caribenho): 0 a
#   classificação para a rodada seguinte (o código da rodada +1 da tabela +0xa3, 0x14), 0xff eliminação (0x14), 1
#   (0x82, 0, 0x1e), outro (0, 0x172, 0x1e) e -1; fase < 2: 6 (0x82, 0, 0x1e), 1 (0, 0x172, 0x1e), 0xfe só o fim da
#   campanha, sempre -1; fase 2: 1 (0x82, 0, 0x1e), outro eliminação (0x14); fases 3..5: 6 (0x96, 0, 0x4b), 0xfe
#   (0x82, 0x4b) e fim; fase 6: 7 a Copa (0x64) e -1, 0xfe (0x96, 0x64) e fim. Prólogo mov al, [esp+0xc]; push esi.
# - 0x90fbe0 WcAfricanCup::record_season_honours thiscall ret 0 (virtual +0x30 das cinco seções e da vtable 0x96a0d4):
#   NationalTeams::clear_competition_for_comp_clubs 0x775420 (push [this+4]; ecx = [0xb74318]), o eax dela. Prólogo
#   mov eax, [ecx+4]; mov ecx, [0xb74318] (9 bytes).
# Iguais no cm0102-1080.exe. Comparadores: só a original (os registros vão ao histórico e a Copa e o fim da campanha não
# rodam duas vezes), com o plano: o retorno e, para quem vai à Copa, *saida = 4 e a lista da Copa com mais uma.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_african_cup_update_staff_records_for_status', 0x90fbf0, [0x8a, 0x54, 0x24, 0x0c, 0x56]),
           ('wc_concacaf_cup_update_staff_records_for_status', 0x915710, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('wc_qualifying_record_season_honours', 0x90fbe0, [0x8b, 0x41, 0x04, 0x8b, 0x0d, 0x18, 0x43, 0xb7, 0x00])]
DOCS = ('os registros de carreira do destino de uma seleção nas eliminatórias africanas e da CONCACAF e as honras da temporada das eliminatórias',
        "the staff records of a nation's fate in the African and CONCACAF qualifying and the qualifying sections' season honours",
        [('wc_african_cup_update_staff_records_for_status', 'WcAfricanCup::update_staff_records_for_status', '0x90fbf0', 'wc_african_cup.cpp', 'thiscall, ret 0x18',
          'regra: nas eliminatórias africanas, na primeira fase quem passa ganha a classificação (peso 25) e quem cai a eliminação e o fim da campanha; nos grupos o primeiro que vai à Copa ganha o registro dela (peso 100) e a vaga, quem vai à repescagem a classificação (45) e quem cai a não classificação (45) e o fim da campanha; na repescagem o vencedor vai à Copa (55) e o perdedor tem a eliminação (100)',
          'rule: in the African qualifying, a first round winner gets the qualification (weight 25) and a loser the knock-out and the end of its campaign; in the groups a winner who reaches the World Cup gets its record (100) and place, one reaching the play-off the qualification (45) and one going out the failed-to-qualify record (45) and the end of its campaign; in the play-off the winner reaches the World Cup (55) and the loser gets the knock-out (100)'),
         ('wc_concacaf_cup_update_staff_records_for_status', 'WcConcacafCup::update_staff_records_for_status', '0x915710', 'wc_concacaf_cup.cpp', 'thiscall, ret 0x18',
          'regra: nas eliminatórias da CONCACAF, no mata-mata caribenho quem passa ganha a classificação para a rodada seguinte (peso 20) e quem cai a eliminação; nas primeiras fases centro-americanas quem vai às semifinais ganha a classificação (30) e quem cai só encerra a campanha; na repescagem o vencedor vai às semifinais e o resto cai; nos grupos semifinais o classificado ganha a classificação (75) e quem cai a não classificação (75); no grupo final quem vai à Copa ganha o registro dela (100) e a vaga',
          'rule: in the CONCACAF qualifying, in the Caribbean knockout a winner gets the qualification for the next round (weight 20) and a loser the knock-out; in the first Central American stages a semi-final qualifier gets the qualification (30) and one going out only ends its campaign; in the play-off the winner reaches the semi finals and the rest go out; in the semi-final groups a qualifier gets the qualification (75) and one going out the failed-to-qualify record (75); in the final group a World Cup qualifier gets its record (100) and place'),
         ('wc_qualifying_record_season_honours', 'WcAfricanCup::record_season_honours', '0x90fbe0', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'regra: quando uma seção das eliminatórias fecha a temporada, as seleções inscritas nela saem dela',
          'rule: when a World Cup qualifying section closes its season, the national teams entered in it leave it')])
CAMPOS = []
INTERESSA = []
