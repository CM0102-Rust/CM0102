# O contrato nas telas, parte 1 (staff_contracts.cpp; leitor sx20, rodada 5, 24/11/2026): a oferta de teste, os textos
# de salario e de fim, bonus e clausulas de rescisao, e os testes de expirado e de renovacao automatica (0x85e480 a
# 0x85fa20). Rotinas inteiras (0x85e480..0x85e588 ret 0xc; 0x85e750..0x85e8df ret 0xc; 0x85e8f0..0x85e95c ret 4;
# 0x85f7a0..0x85f7c9 ret 4; 0x85f7d0..0x85f7dd ret 8; 0x85f7e0..0x85f7ea ret 4; 0x85f7f0..0x85f7fd ret 4;
# 0x85f9f0..0x85f9fa ret; 0x85fa20..0x85fa2a ret).
# Ficam de fora (sx20): 0x85e3d0 staff_contracts_init_static_date (o inicializador estatico da data 23/06/1971 e do
# atexit, que roda na iniciacao da CRT, antes do jogo, como o 0x524cd0 de outra rodada) e 0x85e400 (corpo vazio: so
# `ret`, o destrutor que ele registra).
# TELAS: a prova e o Lucas jogando comparando: o texto de salario e de fim aparece em toda lista de elenco, perfil de
# pessoa, busca e impressao; a oferta de teste, na tela de transferencias (oferecer periodo de testes a um jogador
# livre); bonus e clausulas, na tela de proposta de contrato (por e tirar bonus de gol, assistencia e jogo sem
# sofrer gol, e as clausulas de rescisao); expirado e renovacao, no perfil e nos filtros de busca.
# Convencoes conferidas na desmontagem (thiscall sobre o contrato):
# - 0x85e480(contrato, pessoa, clube, semanas) ret 0xc -> -1: 0x417c10 Date::operator=(+0x25, rascunho, 0xae2c90),
#   0x54c770 Date::add_weeks thiscall(0xae2c90, saida, semanas) ret 8, operator=(+0x2d, rascunho, saida), 0x549c90
#   staff_clear_opinion_links cdecl(pessoa pelo id), 0x7c6830 get_language_fluency thiscall([0xb74350], pessoa,
#   clube, idioma, 0) ret 0x10 -> al (o idioma vai em edx com bytes altos de sobra; lido como byte);
# - 0x85e750(contrato, saida, estilo, bandeiras) ret 0xc, quadro SEH so pelo estado do dinheiro local: 0x43f460
#   Cash::get_display_amount thiscall(&dinheiro, bandeiras) ret 4 (o dword das bandeiras tem os bytes altos de sobra da
#   pilha; so o byte baixo e lido) e 0x441c10 cash_format_money cdecl(buf, 0xc8, valor64, bandeiras); o periodo pelos
#   bits 0x200000 / 0x400000 / 0x800000 de [[0xae3138]+4]; 0x944c9f sprintf "%s" ou 0x66f4e0 langlib_sprintf;
# - 0x85e8f0(contrato, saida) ret 4: langlib ou 0x54a650 Date::get_short_date_string thiscall(+0x2d, saida, 0xc8,
#   0xff) ret 0xc;
# - 0x85f7a0 / 0x85f7e0 / 0x85f7f0(contrato, tipo) ret 4; 0x85f7d0(contrato, tipo, valor) ret 8; 0x85f9f0 /
#   0x85fa20(contrato) ret -> al.
# Comparadores: lado a lado sobre o contrato (0x50 bytes) e, nos textos, sobre o texto (a original na saida, a copia
# num local que comeca com o que a saida tinha).
TELAS = True
ROTINAS = [('staff_contract_init_trial_offer', 0x85e480, [0x83, 0xec, 0x10, 0x8d, 0x54, 0x24, 0x00]),
           ('staff_contract_format_wage_text', 0x85e750, [0x6a, 0xff, 0x68, 0x0b, 0x2a, 0x96, 0x00]),
           ('staff_contract_format_end_date_text', 0x85e8f0, [0x8a, 0x41, 0x35, 0x24, 0x3f]),
           ('staff_contract_add_release_clause', 0x85f7a0, [0x8a, 0x44, 0x24, 0x04, 0xb2, 0x01]),
           ('staff_contract_set_bonus', 0x85f7d0, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('staff_contract_remove_release_clause', 0x85f7e0, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('staff_contract_remove_bonus', 0x85f7f0, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('staff_contract_is_expired', 0x85f9f0, [0x8a, 0x41, 0x35, 0x24, 0x3f]),
           ('staff_contract_is_rolling', 0x85fa20, [0x8a, 0x41, 0x35, 0x24, 0x3f])]
_F = 'staff_contracts.cpp'
DOCS = ('o contrato nas telas: oferta de teste, textos de salario e de fim, bonus e clausulas',
        "the contract on the screens: trial offer, wage and end texts, bonuses and clauses",
        [('staff_contract_init_trial_offer', 'STAFF_CONTRACT::init_trial_offer', '0x85e480', _F, 'thiscall, ret 0xc',
          'preenche o contrato como oferta de teste da pessoa no clube terminando daqui a tantas semanas: os ids, salario e salario anterior 200, comeca hoje e termina hoje + semanas, situacao 5 (bits 0 e 2 postos, 1 e 3 a 5 tirados, os dois altos mantidos), sem numero, moral, queixas, marcas, jogos de fora e infelicidade, situacao de transferencia 4, os vinculos de opiniao da pessoa limpos, o nibble baixo do status do elenco a fluencia da pessoa no primeiro idioma da nacao do clube (no maximo 10) quando o clube tem nacao, sem Bosman, transferencia acertada, jogos, clausulas, bonus (-1) e multa; devolve -1',
          "fills the contract as a trial offer of the person at the club ending some weeks from today: the ids, wage and previous wage 200, starts today and ends today + weeks, status 5 (bits 0 and 2 set, 1 and 3 to 5 cleared, the two high bits kept), no squad number, morale, grievances, flags, matches left out or unhappiness, transfer status 4, the person's opinion links cleared, the squad status' low nibble the person's fluency in the club nation's first language (at most 10) when the club has a nation, no Bosman, arranged transfer, appearances, clauses, bonuses (-1) or release fee; returns -1"),
         ('staff_contract_format_wage_text', 'STAFF_CONTRACT::format_wage_text', '0x85e750', _F, 'thiscall, ret 0xc',
          'escreve o salario como dinheiro no periodo das opcoes do jogo: por semana como esta (arredondamento 8), por mes x 4 (8) ou por ano x 52 (4), com as bandeiras dadas; estilo 0 so o valor, estilo 2 "<valor> p/w", "p/m" ou "p/a", outro "<valor> per week", "per month" ou "per annum" (nada sem bit de periodo)',
          'writes the wage as money in the game options period: per week as it is (rounding 8), per month x 4 (8) or per annum x 52 (4), with the given flags; style 0 the amount alone, style 2 "<amount> p/w", "p/m" or "p/a", any other "<amount> per week", "per month" or "per annum" (nothing without a period bit)'),
         ('staff_contract_format_end_date_text', 'STAFF_CONTRACT::format_end_date_text', '0x85e8f0', _F, 'thiscall, ret 4',
          'escreve o fim do contrato: "Contract Expired", "Free" (transferencia livre), "Rolling" ou a data de fim curta',
          'writes the contract end: "Contract Expired", "Free" (free transfer), "Rolling" or the short end date'),
         ('staff_contract_add_release_clause', 'STAFF_CONTRACT::add_release_clause', '0x85f7a0', _F, 'thiscall, ret 4',
          'poe uma clausula de rescisao: a de multa minima (1) substitui todas as outras; qualquer outra tira a de multa minima',
          'adds a release clause: the minimum fee one (1) replaces all others; any other removes the minimum fee one'),
         ('staff_contract_set_bonus', 'STAFF_CONTRACT::set_bonus', '0x85f7d0', _F, 'thiscall, ret 8',
          'guarda o valor do bonus do tipo dado (0 gol, 1 assistencia, 2 jogo sem sofrer gol)',
          'stores the amount of the given bonus type (0 goal, 1 assist, 2 clean sheet)'),
         ('staff_contract_remove_release_clause', 'STAFF_CONTRACT::remove_release_clause', '0x85f7e0', _F, 'thiscall, ret 4',
          'tira a clausula de rescisao (zera o byte dela)',
          'removes the release clause (clears its byte)'),
         ('staff_contract_remove_bonus', 'STAFF_CONTRACT::remove_bonus', '0x85f7f0', _F, 'thiscall, ret 4',
          'tira o bonus (-1 no valor)',
          'removes the bonus (-1 as the amount)'),
         ('staff_contract_is_expired', 'STAFF_CONTRACT::is_expired', '0x85f9f0', _F, 'thiscall',
          'diz se a situacao do contrato e expirado',
          'says whether the contract status is expired'),
         ('staff_contract_is_rolling', 'STAFF_CONTRACT::is_rolling', '0x85fa20', _F, 'thiscall',
          'diz se a situacao do contrato e de renovacao automatica',
          'says whether the contract status is rolling')])
