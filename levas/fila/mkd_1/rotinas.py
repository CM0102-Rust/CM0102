# O valor de um contrato para a pessoa contra uma oferta (STAFF_CONTRACT::calc_contract_value 0x866830,
# staff_contracts.cpp; leitor D do mapa do mercado, 11/11/2026). Chamada pela satisfação com o contrato
# (get_contract_satisfaction 0x866620, Rust: o contrato atual e o desejado contra o desejado, no passe diário das pessoas
# com random(50) = 0) e pela decisão sobre as ofertas de contrato (0x4ce4e0, original). Igual no cm0102-1080.exe
# (0x6b0 bytes conferidos). Convenções conferidas:
# - a rotina: thiscall(contrato, pessoa, oferta), ret 8, devolve o int do _ftol (0x945640);
# - 0x85fc50 (só sem oferta; nenhum chamador passa 0): push 0; push 0; push 0xb; push -1; push -1; push clube;
#   push pessoa; lea ecx, local -> thiscall, ret 0x1c;
# - 0x54cdb0 (Date::days_since): push [0xae2c94]; push [0xae2c90]; lea ecx, contrato+0x2d -> thiscall, ret 8 (grava a
#   Data estática 0xae2470: a cópia chama a mesma);
# - 0x5b7d20 (a máscara de posições): push ficha; add esp, 4 -> cdecl; só o al conta;
# - 0x945ff0 (pow da CRT): fld base; fld expoente -> game_pow;
# - 0x417c10 (Date::operator=, as duas datas da cópia da oferta): a cópia copia os 0x50 bytes (só o ano do fim é lido).
# Comparador: compare_value (a original e a cópia, o valor devolvido; a única escrita fora da pilha é a Data estática,
# que as duas deixam igual).
SAVE = False
ROTINAS = [('staff_contract_calc_contract_value', 0x866830, [0x6a, 0xff, 0x68, 0xb0, 0x2a, 0x96, 0x00])]
DOCS = ('o valor de um contrato para a pessoa, contra uma oferta',
        'the value of a contract for the person, against an offer',
        [('staff_contract_calc_contract_value', 'STAFF_CONTRACT::calc_contract_value', '0x866830', 'staff_contracts.cpp', 'thiscall, ret 8',
          'o valor do contrato para a pessoa contra outro (em geral o desejado): menos de 7 dias para o fim, 0; o salário (+0xc) mais os bônus pela posição (goleiro: jogo sem sofrer gol × 0,8; defensor: gol × 0,3 e assistência × 0,2; meio-campo × 0,4 e × 0,7; atacante × 0,8 e × 0,5), mais +8 / (dias × 0,2222); com reputação mundial acima de 4000 e reputação² × 0,01 acima de +8, metade; cada cláusula de rescisão ativa multiplica (1,01 a 2, pelas multas de rescisão das duas abaixo de 1 milhão, 5 milhões ou acima, as cláusulas da oferta e a idade); o fator de duração (0,75 com cláusula de multa mínima na oferta, 0,875 com rebaixamento ou não acesso, ou 1, menor para quem passou da faixa de idade da posição) elevado à diferença dos anos de término, entre 0,5 e 2: divide quando o contrato acaba antes, multiplica quando depois',
          'the value of the contract for the person against another (usually the desired one): fewer than 7 days to the end, 0; the wage (+0xc) plus the bonuses by position (goalkeeper: clean sheet × 0.8; defender: goal × 0.3 and assist × 0.2; midfielder × 0.4 and × 0.7; attacker × 0.8 and × 0.5), plus +8 / (days × 0.2222); with world reputation above 4000 and reputation² × 0.01 above +8, half; each active release clause multiplies (1.01 to 2, by both release fees below 1 million, 5 million or above, the offer clauses and the age); the length factor (0.75 with a minimum fee clause in the offer, 0.875 with relegation or non-promotion, or 1, lower past the position age band) raised to the difference of the end years, within 0.5..2: divides when the contract ends earlier, multiplies when later')])
CAMPOS = [('contrato +8: valor espalhado pelos dias que faltam (nome da comunidade "Unknown"; [inferência] a luva); valor; lê', 'contract +8: value spread over the days left (community name "Unknown"; [inference] the signing-on fee); value; reads'),
          ('contrato +0x1c..+0x20: as 5 cláusulas de rescisão (0 não promoção, 1 multa mínima, 2 sem jogar, 3 rebaixamento, 4 emprego de técnico); valor; lê', 'contract +0x1c..+0x20: the 5 release clauses (0 non-promotion, 1 minimum fee, 2 non-playing, 3 relegation, 4 manager job); value; reads'),
          ('contrato +0x21: multa de rescisão; valor; lê (o do contrato e o da oferta)', 'contract +0x21: release fee; value; reads (the contract and the offer)'),
          ('ficha do jogador +0xd: reputação mundial; valor; lê', 'player sheet +0xd: world reputation; value; reads')]
INTERESSA = ['CMRTE']
