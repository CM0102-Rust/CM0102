# O preço pedido por um jogador (fechamento do mapa do mercado, seção A, 11/11/2026;
# levas/reorganizacao/mercado/fechamento_mka.md): contract_manager_calc_asking_price 0x4d1600..0x4d4257
# (contract_manager.cpp, 11,3 KB de x87, 3185 instruções), desmontada inteira. Igual no cm0102-1080.exe (os 11.352
# bytes conferidos). Sem SEH.
# Convenções conferidas nos pontos de chamada e no corpo:
# - 0x4d1600: thiscall(gerente de contratos, vendedor, comprador, jogador, *motivos (ushort), ajuste (char), bandeira),
#   ret 0x18 (0x4d4249/0x4d4255); o 4º argumento é um ponteiro de 16 bits (or byte [edi+1]; and word [edi]);
# - 0x6e86d0 (perceived_ability): cdecl(técnico do vendedor, jogador, vendedor, 1, reputação/2 + potencial × 25), add
#   esp, 0x14; devolve ax (os 16 bits altos de eax ficam sujos e o calc_player_value só lê os 16 baixos);
# - 0x4d0c60: thiscall(this, jogador, vendedor, 0, visto, 0) — o ecx é lido de [esp+0x40] antes do add esp;
# - 0x600a50: cdecl(clube, a, b, 0, 0); 0x53ca90, 0x544520, 0x5444a0, 0x544600, 0x4b02e0, 0x5431e0, 0x540a50: cdecl;
# - 0x5fd210, 0x5fea50, 0x418f60, 0x5a0590, 0x5986b0, 0x8c7560, 0x849ce0, 0x54cdb0, 0x456740, 0x8676f0, 0x867b50:
#   thiscall; 0x945ff0 (pow) com a base em st1 e o expoente em st0.
# Comparador: lado a lado pelo estado do sorteio, com as regiões do calc_player_value e os 2 bytes dos motivos.
SAVE = False
ROTINAS = [('calc_asking_price', 0x4d1600, [0x83, 0xec, 0x7c, 0x53, 0x55])]
DOCS = ('o preço pedido por um jogador: o valor mais um prêmio por papel, moral, felicidade, finanças, comprador, idade e reputação',
        'the asking price for a player: the value plus a premium by role, morale, happiness, finances, buyer, age and reputation',
        [('calc_asking_price', 'contract_manager_calc_asking_price', '0x4d1600', 'contract_manager.cpp', 'thiscall, ret 0x18',
          'o valor do jogador (calc_player_value com o potencial visto pelo técnico) mais um prêmio: valor × (Q − 1), Q = finanças × felicidade × moral × importância no elenco × comprador × contratação recente, no máximo clamp(comprador² × recente, 4,25, 10) e 6 × finanças; o prêmio × min(recente², 25) × liga × idade, metade se listado, até 50 milhões, no mínimo −0,4 × valor e −3 milhões; mais um prêmio de reputação; no mínimo 1000; até o valor com permissão de trabalho; até valor × comprador listado ou com time B; piso do clube rico; até max(3 × valor, valor + 5 milhões) × comprador × recente; as queixas e os dias de contrato puxam para o valor; nas grandes nações (reputação acima de 4000) escadas por papel, sem entrada de lista o preço volta ao valor (0,67 valor + 0,33 preço); o piso do jovem (atributos físicos, drible, finalização, os Jovens do técnico do vendedor menos 10 no clube da IA, o potencial); o teto por idade (29 anos: valor + 5 milhões ... 38: o valor); Portugal e Coreia do Sul; a multa rescisória ativa. Técnico humano de férias: o valor guardado',
          'the player\'s value (calc_player_value with the potential the manager sees) plus a premium: value × (Q − 1), Q = finances × happiness × morale × squad importance × buyer × recent signing, at most clamp(buyer² × recent, 4.25, 10) and 6 × finances; the premium × min(recent², 25) × league × age, halved when listed, up to 50 million, at least −0.4 × value and −3 million; plus a reputation premium; at least 1000; at most the value with a work permit; at most value × buyer when listed or with a B team; the rich club\'s floor; at most max(3 × value, value + 5 million) × buyer × recent; grievances and the contract days left pull it to the value; in the big nations (reputation above 4000) ladders by role, without a shortlist entry the price comes back to the value (0.67 value + 0.33 price); the youngster floor (physical attributes, dribbling, finishing, the seller manager\'s Youngsters minus 10 at a computer club, the potential); the age cap (29: value + 5 million ... 38: the value); Portugal and South Korea; the active release fee. A human manager on holiday: the stored value')])
INTERESSA = ['Rust Tweaks']
