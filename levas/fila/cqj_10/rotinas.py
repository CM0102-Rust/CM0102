# O destino das seleções nos torneios: a Copa Africana de Nações, a Copa América e a eliminatória da Oceania (o mesmo
# slot, a virtual +0x44 update_staff_records_for_status, em três torneios de seleções; african_nations.cpp,
# goldcup.cpp, wc_oceania_league.cpp). Frente das competições (Agente D, 03/11/2026), carreira de 8 ligas.
# - 0x401ba0 (AfricanNations::update_staff_records_for_status, 373 bytes; 32 chamadas até 2002-06-01 com as 8 ligas, a
#   Copa Africana de janeiro e fevereiro de 2002): 3 fases de grupos e o mata-mata na fase 3. Rotina de duas mutações.
# - 0x5e13b0 (GoldCup::update_staff_records_for_status, 369 bytes; 34 chamadas: a Copa América de julho de 2001): a
#   mesma rotina com 2 fases de grupos (o mata-mata na fase 2; sem o teste do desfecho 4, que não muda nada). Uma
#   mutação. O porte das duas é um ajudante comum (cqj10_selecao) com o número de fases de grupos; diferenças byte a
#   byte conferidas: o limite da fase (3 × 2), o índice da fase do mata-mata em [+0xc] (+0xc × +8) e o teste do 4.
# - 0x91e840 (WcOceaniaLeague::update_staff_records_for_status, 326 bytes; 12 chamadas): grupos, final da Oceania e a
#   repescagem (a vaga na Copa do Mundo).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   As próprias: virtuais +0x44 (6 pushes; mov ecx, torneio -> thiscall, ret 0x18).
#   Dentro: 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), 0x86bdd0 (5 pushes; mov ecx, [0xdd7f00] -> thiscall, ret
#   0x14), 0x86c1d0 (4 pushes; mov ecx, [0xdd7f00] -> thiscall, ret 0x10), 0x86c000 (5 pushes -> thiscall, ret 0x14),
#   0x86a800 / 0x86b0b0 / 0x86b710 (push a6; push jogo; push clube; mov ecx, [0xdd7f00] -> thiscall, ret 0xc),
#   0x775000 (push nação; mov ecx, [0xb74318] -> thiscall, ret 4), 0x92e740 (push seleção; mov ecx, objeto da Copa
#   do Mundo -> thiscall, ret 4).
#   Palavras empurradas com lixo nos 16 bits altos (+0x30 e +0x32 do jogo na eliminação, a rodada na 0x4bf850): os
#   chamados só leem a palavra; a cópia passa limpo (como a cpz_4). Na classificação do mata-mata os 16 bits altos são
#   zero (vêm do lea da rodada) e a cópia passa igual.
# Comparadores: só a original (os registros vão ao histórico dos técnicos e encerram as convocações; a vaga na Copa do
# Mundo cresce a lista dela): a devolução 0 e, na repescagem ganha, *saida = 4 e a lista da Copa do Mundo um a mais.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: as três ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqj10_copa_africana_registros', 0x401ba0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqj10_copa_america_registros', 0x5e13b0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqj10_eliminatoria_oceania_registros', 0x91e840, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('o destino das seleções na Copa Africana, na Copa América e na eliminatória da Oceania',
        'the national teams\' fate in the African Nations Cup, the Copa America and the Oceania qualifiers',
        [('cqj10_copa_africana_registros', 'AfricanNations::update_staff_records_for_status', '0x401ba0', 'african_nations.cpp', 'thiscall, ret 0x18',
          'regra: nas três fases de grupos da Copa Africana, quem passa ganha o registro de classificação (peso 25) e quem não passa o de não classificação (peso 25), que encerra as convocações da seleção; no mata-mata (fase 3), quem passa de rodada ganha a classificação pela rodada (peso de 40 a 100), quem cai a eliminação (que também encerra as convocações), e o campeão, o vice e o terceiro os registros da final',
          'rule: in the three African Nations Cup group stages, a team going through gets the qualification record (weight 25) and one going out the failed-to-qualify record (weight 25), which ends the team\'s call-ups; in the knockout (stage 3), a team going through gets the qualification by round (weight 40 to 100), one going out the knocked-out record (which also ends the call-ups), and the winner, runner-up and third place the final records'),
         ('cqj10_copa_america_registros', 'GoldCup::update_staff_records_for_status', '0x5e13b0', 'goldcup.cpp', 'thiscall, ret 0x18',
          'regra: a Copa América segue a mesma regra da Copa Africana com duas fases de grupos (o mata-mata na fase 2)',
          'rule: the Copa America follows the African Nations Cup rule with two group stages (the knockout at stage 2)'),
         ('cqj10_eliminatoria_oceania_registros', 'WcOceaniaLeague::update_staff_records_for_status', '0x91e840', 'wc_oceania_league.cpp', 'thiscall, ret 0x18',
          'regra: nos grupos da eliminatória da Oceania, quem passa ganha a classificação (peso 20) e quem não passa a não classificação e encerra a campanha; na final da Oceania, o vencedor a classificação (peso 30) e o perdedor a eliminação; na repescagem (contra a América do Sul [inferência]), o vencedor ganha a vaga na Copa do Mundo (registro de peso 100, a entrada na lista dela e a situação 4) e o perdedor a eliminação (peso 100) e o fim da campanha',
          'rule: in the Oceania qualifier groups, a team going through gets the qualification (weight 20) and one going out the failed-to-qualify record and ends its campaign; in the Oceania final, the winner gets the qualification (weight 30) and the loser the knock-out; in the play-off (against South America [inference]), the winner gets the World Cup place (weight 100 record, the entry in its list and status 4) and the loser the knock-out (weight 100) and the end of the campaign')])
CAMPOS = [('jogo da copa +0x30 rodada (código), +0x32 fase, +0x34 índice da rodada; valor; lê (as três)',
           'cup tie +0x30 round (code), +0x32 stage, +0x34 round index; value; reads (all three)'),
          ('torneio +0xc fases (a do mata-mata: +0xa3 rodadas de 0x68 bytes, +0x6f código), +4 competição; ponteiro; lê (cqj10_copa_africana_registros, cqj10_copa_america_registros)',
           'tournament +0xc stages (the knockout one: +0xa3 rounds of 0x68 bytes, +0x6f code), +4 competition; pointer; reads (cqj10_copa_africana_registros, cqj10_copa_america_registros)'),
          ('Copa do Mundo +0x14 lista das seleções (6 bytes), +0x36 quantas, +0x38 máximo; [0x9cf79c] id da Copa do Mundo; valor/ponteiro; grava pela 0x92e740 (cqj10_eliminatoria_oceania_registros)',
           'World Cup +0x14 team list (6 bytes), +0x36 count, +0x38 maximum; [0x9cf79c] World Cup id; value/pointer; writes through 0x92e740 (cqj10_eliminatoria_oceania_registros)'),
          ('seleção +0x53 nação; [0xb74318] gerenciador das seleções (0x775000: a competição atual da seleção acaba); ponteiro; lê (cqj10_eliminatoria_oceania_registros)',
           'national team +0x53 nation; [0xb74318] national teams manager (0x775000: the team\'s current competition ends); pointer; reads (cqj10_eliminatoria_oceania_registros)')]
INTERESSA = []
