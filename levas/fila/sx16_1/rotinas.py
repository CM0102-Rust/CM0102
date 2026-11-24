# player_regen.cpp (leitor sx16, rodada 5, 24/11/2026): a regeneracao de uma pessoa aposentada como nao jogador.
# - 0x7ab340: thiscall(regen, pessoa), ret 4; ecx nao e usado. Inteira ate 0x7aba93 (ret 4) mais a tabela de saltos
#   0x7aba98 (10 entradas, o cargo - 1). Prologo `sub esp, 0x10; push ebx; push esi` (5 bytes, sem relativo).
# - Quem chama: PLAYER_REGEN::process_retirements 0x7a59e0 (0x7a6463, ecx = regen; ja Rust, chama pelo exe), 108
#   vezes na carreira ate 2002-06-01 (cobertura_simulacao).
# - Chamadas (conferidas na desmontagem): 0x86fb80 rewind_staff_history thiscall([0xdd7f00], pessoa); 0x539c60,
#   0x53a1b0, 0x53a5b0 cdecl(nacao); 0x5fff30 thiscall([0xb63c98], pessoa); 0x7c6bf0 thiscall([0xb74350], pessoa,
#   k) ret 8; 0x4ca1c0 thiscall([0xae1050], pessoa, 0, 0) ret 0xc (os push ebp/edi no meio sao registros salvos);
#   0x7c6930 stdcall(pessoa, k) (ecx carregado e ignorado); 0x7c6a30 thiscall(stats, pessoa, k, nivel, 1) ret 0x10;
#   0x90cfc0 cdecl(n); 0x945640 _ftol; 0x53ab90 cdecl(pessoa, &hoje); 0x4cb990 stdcall(cargo, 0) ret 8; 0x4d0b50
#   thiscall(cm, pessoa); 0x4d45f0 thiscall(cm, clube) ret 4 (o mes do fim da temporada); 0x549f70 Date::Date
#   thiscall(data, dia, mes, ano, -1) ret 0x10; 0x417c10 Date::operator= thiscall(&validade, &tmp, data) ret 8;
#   0x4cf5a0 thiscall(cm, pessoa); 0x85e960 thiscall(contrato, pessoa, 0); 0x539490 cdecl(pessoa).
# - x87: fild do potencial x 0,75 (0x969040) com _ftol; o teste contra potencial x 0,5 (f32 0x968f18).
# Defeitos do original: sem cargo no clube nem na selecao o byte do cargo e o do argumento, que os lacos das linguas
# deixaram em 0x41 (toda pessoa sem cargo cai na faixa padrao, 39..53 anos); a ficha de nao jogador e escrita sem
# teste de nulo (so o presidente e o contrato testam).
# Comparador: so a original (sorteia, reescreve a pessoa e chama rotinas com efeito).
SAVE = False
ROTINAS = [('regenerate_as_non_player', 0x7ab340, [0x83, 0xec, 0x10, 0x53, 0x56])]
DOCS = ('a regeneração de um aposentado como não jogador',
        'regenerating a retired person as a non-player',
        [('regenerate_as_non_player', 'player_regen_regenerate_as_non_player', '0x7ab340', 'player_regen.cpp', 'thiscall, ret 4',
          'refaz a vaga de uma pessoa aposentada como um não jogador novo: tira as marcas de aposentadoria, tipo 1, histórico rebobinado, internacionalizações zeradas, nomes novos da nação, apelido tirado dos humanos, entrou no clube hoje, línguas refeitas pelas nativas (pontos acima de (20 − adaptabilidade) × 10, fluência (pontos − base) / 75, até 10). Ficha: capacidade 1 + sorteio(potencial × 0,75), no mínimo potencial × 0,5; reputação em casa sorteio(750) + capacidade × 25 + 1, mundial sorteio(200) + capacidade × 15 + 1 (casa abaixo da mundial vira mundial + 10 e a mundial 1), atual sorteio(50) + casa. A idade pelo cargo (presidente 36..55, diretor-geral e gerente-geral 40..64, diretor de futebol 37..46, auxiliar e treinador de reservas 35..49, treinador 32..46, fisioterapeuta 35..59, os outros 39..53); o presidente de um clube ganha os atributos de negócio pela reputação do clube (acima de 6.250, de 3.250 ou abaixo; empresa aberta). Nascimento no ano de hoje − idade e dia sorteado, a idade recalculada, a formação preferida por um sorteio de 125. Com clube e ficha, se puder ficar no cargo: contrato novo (o antigo sai, salário 0, validade num dia 1..28 do mês do fim da temporada do clube, daqui a 1..6 anos, contrato inicial). Por fim os atributos normalizados. Defeitos: sem cargo, a faixa de idade padrão (o byte do cargo é o resto do laço, 0x41); a ficha é escrita sem teste de nulo',
          "rebuilds a retired person's slot as a new non-player: clears the retirement marks, type 1, history rewound, caps cleared, new names from the nation, nickname removed from the humans, joined the club today, languages rebuilt from the native ones (points above (20 − adaptability) × 10, fluency (points − base) / 75, at most 10). Sheet: ability 1 + roll(potential × 0.75), at least potential × 0.5; home reputation roll(750) + ability × 25 + 1, world roll(200) + ability × 15 + 1 (a home below the world becomes world + 10 and the world 1), current roll(50) + home. The age by the job (chairman 36..55, managing and general director 40..64, director of football 37..46, assistant and reserve manager 35..49, coach 32..46, physio 35..59, the others 39..53); a club chairman gets the business attributes by the club's reputation (above 6,250, above 3,250 or below; PLC). Born this year − age on a rolled day, the age recomputed, the formation preference by a roll of 125. With a club and a sheet, when he can keep the job: a new contract (the old one goes, wage 0, expiry on day 1..28 of the club's season-end month, 1..6 years ahead, initial contract). Last, the attributes normalised. Defects: without a job, the default age band (the job byte is the loop leftover, 0x41); the sheet is written without a null test")])
