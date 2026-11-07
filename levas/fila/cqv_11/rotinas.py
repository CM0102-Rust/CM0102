# As quartas da Mercosul (os cinco primeiros e os três melhores segundos) e as posições previstas da Libertadores e da
# Mercosul (conmebol_merc.cpp, conmebol_liber.cpp, em eurocomp\). Frente das competições (Agente E, 07/11/2026,
# madrugada), carreira de 8 ligas (o Brasil jogável: os clubes brasileiros jogam a Mercosul de 2001).
# - 0x4c53d0 (ConmebolMerc::create_quarter_final_stage): as quartas. Três mutações.
# - 0x4c4210 (ConmebolLiber::set_predicted_positions) e 0x4c5a90 (ConmebolMerc::set_predicted_positions): as
#   previstas. Passageiras.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (as quartas da passagem de fase; as previstas
# pela virtual +0x5c). Dentro: 0x944cf1, 0x944e46 e 0x9452ca (push; add esp, 4 -> cdecl), 0x68b100 (22 pushes; mov
# ecx, novo -> thiscall, ret 0x58), 0x6827d0 (push 0; mov ecx -> thiscall, ret 4), 0x6826d0 (push i; mov ecx ->
# thiscall, ret 4), as virtuais +0x40 (5 pushes) e +0x68 (5 pushes), 0x86c1d0 (4 pushes; mov ecx, [0xdd7f00] ->
# thiscall, ret 0x10), a virtual 0 (push 1), a virtual +0x3c (4 pushes, ret 0x10), 0x522e00 (15 pushes, ret 0x3c),
# 0x51c410 (push 0; mov ecx -> thiscall, ret 4), 0x945333 (4 pushes -> cdecl) e 0x4a2540 (2 pushes; mov ecx, [+8] ->
# thiscall, ret 8).
# Comparadores: só a original, com o plano: as quartas conferem os 8 (os primeiros de cada grupo e três segundos, sem
# repetição) na tabela da fase nova; as previstas conferem cada posição gravada.
# Determinismo: nada lê pilha, heap ou endereço como valor (o grupo empurrado à virtual +0x40 vai num dword cujos bytes
# altos são os da conta dos 8, zero).
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqv11_mercosul_quartas', 0x4c53d0, [0x6a, 0xff, 0x68, 0x7c, 0x6b, 0x95, 0x00]),
           ('cqv11_libertadores_previstas', 0x4c4210, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('cqv11_mercosul_previstas', 0x4c5a90, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('as quartas da Mercosul e as posições previstas da Libertadores e da Mercosul',
        'the Mercosur Cup quarter-finals and the Libertadores and Mercosur predicted positions',
        [('cqv11_mercosul_quartas', 'ConmebolMerc::create_quarter_final_stage', '0x4c53d0', 'conmebol_merc.cpp', 'thiscall, ret 0', 'regra: as quartas da Mercosul têm os primeiros dos cinco grupos e os três melhores segundos (uma tabela dos cinco segundos, com os pontos dos grupos); os outros dois segundos e os demais ficam fora (registro de não classificado); o mata-mata é em ida e volta e a primeira rodada é sorteada', 'rule: the Mercosur Cup quarter-finals have the winners of the five groups and the three best runners-up (a table of the five runners-up, with their group points); the other two runners-up and the rest are out (failed to qualify record); the knockout is two-legged and the first round is drawn'),
         ('cqv11_libertadores_previstas', 'ConmebolLiber::set_predicted_positions', '0x4c4210', 'conmebol_liber.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Libertadores: os clubes pela reputação; os do mata-mata pelas rodadas, da final para trás; os dos grupos em blocos de 8', 'rule: the Libertadores predicted positions: the clubs by reputation; the knockout ones by round, from the final backwards; the group ones in blocks of 8'),
         ('cqv11_mercosul_previstas', 'ConmebolMerc::set_predicted_positions', '0x4c5a90', 'conmebol_merc.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Mercosul: os clubes pela reputação; os do mata-mata pelas rodadas; os dos grupos em blocos de 5', 'rule: the Mercosur Cup predicted positions: the clubs by reputation; the knockout ones by round; the group ones in blocks of 5')])
CAMPOS = [('copa +0xbe classificados por grupo (byte, +1 no grupo de um melhor segundo), +0xb1 tabela do grupo A, +0xc fases (0..3 os grupos B..E, 4 as quartas); valor/ponteiro; lê e grava', 'cup +0xbe qualifiers per group (byte, +1 in the group of a best runner-up), +0xb1 group A table, +0xc stages (0..3 groups B..E, 4 the quarter-finals); value/pointer; reads and writes'),
          ('linha de tabela (0x3b bytes) +0 clube, +9 zerado na tabela dos segundos, +0x37 estado (6: melhor segundo); valor; grava (quartas)', 'table row (0x3b bytes) +0 club, +9 cleared in the runners-up table, +0x37 status (6: best runner-up); value; writes (quarter-finals)'),
          ('copa +0xee a lista dos convidados, +0xf2 a conta; registro de estatística +0x72 posição prevista; ponteiro/valor; ordena e grava (previstas)', 'cup +0xee the invited list, +0xf2 its count; stats record +0x72 predicted position; pointer/value; sorts and writes (predicted)')]
INTERESSA = ['CMRTE']
