# As aposentadorias (player_regen.cpp): quem se aposenta no jogo novo (player_regen_check_initial_retirement 0x7a79c0,
# pela lista inicial, para cada pessoa com clube), quem se aposenta pela idade (player_regen_check_retirement 0x7a7310,
# pela busca de novas aposentadorias 0x7a5790 nos dias da 0x5d6250) e a lista inicial (PLAYER_REGEN::
# build_initial_retirement_list 0x7a58f0, uma vez no jogo novo). Os mapas davam 79 bytes à 0x7a7310 (são 1,7 KB) e
# 1264 à 0x7a79c0; a 0x7a7310 tinha ficado de fora na 247ª (x87): o x87 do jogo é de 53 bits e as contas são as de
# f64 (constantes lidas do exe). Quirk fiel: sem tipo de cargo, as duas leem o byte baixo do ponteiro da saída (a
# vaga errada da pilha). A 0x7a58f0 tem SEH só das datas locais (destrutor vazio 0x5a53c0). Chamadas: 0x4cf5a0,
# 0x4cf5f0, 0x417c10, 0x54cdb0, 0x85fa00, 0x7a8510 e o sorteio já em Rust; 0x669340/0x669360 (nação-chave) originais.
# Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira até 2002-06-01. Comparadores: as duas decisões lado
# a lado (sorteio, [0xae1080], a saída de 26 bytes, a data estática da 0x54cdb0; a cópia sem contar); a lista
# inicial acrescenta e sorteia: só a original.
SAVE = False
ROTINAS = [('aposentadoria_no_inicio', 0x7a79c0, [0x83, 0xec, 0x0c, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('aposentadoria_por_idade', 0x7a7310, [0x83, 0xec, 0x14, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('lista_inicial_de_aposentadorias', 0x7a58f0, [0x6a, 0xff, 0x68, 0x88, 0xf1, 0x95, 0x00])]
DOCS = ('as aposentadorias: quem se aposenta no começo do jogo, quem se aposenta pela idade e a lista inicial',
        'retirements: who retires at the start of the game, who retires by age and the initial list',
        [('aposentadoria_no_inicio', 'player_regen_check_initial_retirement', '0x7a79c0', 'player_regen.cpp', 'thiscall, ret 8',
          'no jogo novo, a pessoa com clube se aposenta? Sem ficha de jogador pelo limite de idade do cargo; jogador pelo limite da ficha, a habilidade, a competição do clube e o estatuto do contrato; a saída (id, clube, motivo) e as datas do anúncio e da aposentadoria pela nação-chave',
          'at a new game, does the person with a club retire? Non-players by the job age limit; players by the profile limit, the ability, the club competition and the contract status; the output (id, club, reason) and the announcement and retirement dates by the key nation'),
         ('aposentadoria_por_idade', 'player_regen_check_retirement', '0x7a7310', 'player_regen.cpp', 'thiscall, ret 8',
          'na busca de novas aposentadorias, a pessoa se aposenta? Como a do começo, com o limite ajustado pelo contrato e pelo salário, o desemprego, o fim do contrato e o potencial; sem nação-chave, datas sorteadas perto de hoje',
          'in the search for new retirements, does the person retire? Like the start one, with the limit adjusted by contract and wage, unemployment, contract end and potential; without a key nation, dates drawn near today'),
         ('lista_inicial_de_aposentadorias', 'PLAYER_REGEN::build_initial_retirement_list', '0x7a58f0', 'player_regen.cpp', 'thiscall',
          'no jogo novo, testa cada pessoa elegível (0x7a79c0), acrescenta quem se aposenta e ordena a lista',
          'at a new game, tests every eligible person (0x7a79c0), adds those who retire and sorts the list')])
CAMPOS = [('pessoa +0x18, +0x3d, +0x28, +0x57, +0x59: idade, tipos do cargo no clube e na seleção, dois bytes com sinal; valor; lê (as duas decisões)',
           'person +0x18, +0x3d, +0x28, +0x57, +0x59: age, club and national job types, two signed bytes; value; reads (both decisions)'),
          ('pessoa +0x1a, +0x24, +0x39, +0x61, +0x69: nação, seleção, clube, ficha de jogador e ficha de técnico; ponteiro; lê (as duas decisões)',
           'person +0x1a, +0x24, +0x39, +0x61, +0x69: nation, national team, club, player profile and staff profile; pointer; reads (both decisions)'),
          ('pessoa +0x3e: data desde a qual está sem clube; valor; lê (aposentadoria_por_idade)',
           'person +0x3e: date since unemployed; value; reads (aposentadoria_por_idade)'),
          ('jogador +5, +7, +0xb, +0xf, +0x14, +0x15, +0x3c: habilidade atual e potencial, a habilidade das contas, bytes da ficha; valor; lê (as duas decisões)',
           'player +5, +7, +0xb, +0xf, +0x14, +0x15, +0x3c: current and potential ability, the ability used in the sums, profile bytes; value; reads (both decisions)'),
          ('contrato com o clube +0xc, +0x2f, +0x3d, +0x3e, +0x45, +0x4f: salário, ano do fim, estatuto, um byte, um campo e o nibble alto; valor; lê (as duas decisões)',
           'club contract +0xc, +0x2f, +0x3d, +0x3e, +0x45, +0x4f: wage, end year, status, a byte, a field and the high nibble; value; reads (both decisions)'),
          ('contrato +0x49: marca que exclui da lista inicial; valor; lê (lista_inicial_de_aposentadorias)',
           'contract +0x49: flag that excludes from the initial list; value; reads (lista_inicial_de_aposentadorias)'),
          ('clube +0x57 e [[0xadadfc] + id × 4]: competição e se ela roda no jogo; ponteiro e valor; lê; nação +0x11c; valor; lê',
           'club +0x57 and [[0xadadfc] + id × 4]: competition and whether it runs in the game; pointer and value; reads; nation +0x11c; value; reads'),
          ('0xb63d60 + k × 72: +0x15 dia do fim de temporada, +0x1d um ano a mais; valor; lê (as duas decisões)',
           '0xb63d60 + k × 72: +0x15 season end day, +0x1d one more year; value; reads (both decisions)'),
          ('saída (26 bytes) +0 id, +4 clube, +8 anúncio, +0x10 aposentadoria, +0x18 motivo, +0x19 zero; id e valor; grava (as duas decisões)',
           'output (26 bytes) +0 id, +4 club, +8 announcement, +0x10 retirement, +0x18 reason, +0x19 zero; id and value; writes (both decisions)'),
          ('[0xae1080] o contrato atual; ponteiro; grava (as duas decisões)',
           '[0xae1080] the current contract; pointer; writes (both decisions)'),
          ('[0xae2d0c] registro de 79 bytes por pessoa, +0xb bits 0x30; valor; lê (lista_inicial_de_aposentadorias)',
           '[0xae2d0c] 79-byte record per person, +0xb bits 0x30; value; reads (lista_inicial_de_aposentadorias)'),
          ('PLAYER_REGEN +4 lista de aposentadorias (26 bytes), +8 conta; ponteiro e valor; acrescenta e ordena (lista_inicial_de_aposentadorias)',
           'PLAYER_REGEN +4 retirement list (26 bytes), +8 count; pointer and value; appends and sorts (lista_inicial_de_aposentadorias)')]
INTERESSA = ['GPF2', 'CMRTE']
