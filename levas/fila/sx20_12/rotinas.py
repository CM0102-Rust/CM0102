# O contrato nas telas, parte 2 (staff_contracts.cpp; leitor sx20, rodada 5, 24/11/2026): os textos de bonus e de
# clausula, o texto de vencimento, as clausulas que o clube permite, os passos do salario, o status no elenco e a
# abreviacao das clausulas (0x85f830 a 0x868020). Rotinas inteiras (0x85f830..0x85f937 ret 0x10; 0x85f940..0x85f9cc
# ret 0x10 mais a tabela de 0x85f9d0; 0x85fa30..0x85fc13 ret 8 mais a tabela de 0x85fc18; 0x8679a0..0x8679fb ret;
# 0x867a00..0x867a82 ret mais a tabela de 0x867a84; 0x867ad0..0x867b3a ret mais a tabela de 0x867b3c;
# 0x867b70..0x867c49 ret; 0x867c50..0x867d29 ret; 0x867ef0..0x867ffa ret 8 mais a tabela de 0x868000;
# 0x868020..0x868204 ret 8).
# DEPENDE DO sx20_11 (integrar junto ou depois): usa contract_status_bits, langlib_text, game_option_flags,
# contract_compare, staff_contract_format_end_date_text e as constantes de situacao e de periodo.
# TELAS: a prova e o Lucas jogando comparando: a tela de proposta de contrato (bonus e clausulas com nome, as
# clausulas que o clube permite, salario + e -, o vencimento "mes ano"), o perfil de pessoa (os bonus e clausulas do
# contrato), as listas de elenco do clube (status no elenco, abreviacao das clausulas) e a busca.
# Convencoes conferidas na desmontagem:
# - 0x85f830 thiscall(contrato, tipo, saida, nao usado, modo) ret 0x10: modo 1 copia o nome; senao 0x43f460
#   Cash::get_display_amount(&valor, 0xa) e 0x441c10 cash_format_money(buf, 0xc8, valor64, 0xa) (o 0xa empurrado para o
#   primeiro e reaproveitado como o quinto do segundo) e langlib "<%s cash amount> <%s - bonus name>";
# - 0x85f940 thiscall(?, tipo, saida, ?, ?) ret 0x10 (nao le ecx); 0x8679a0 / 0x867a00 cdecl(tipo, saida);
# - 0x85fa30 thiscall(contrato, saida, estilo) ret 8: 0x54cfb0 Date::get_month(+0x2d) e o ano em +0x2f;
# - 0x867ad0 cdecl(pessoa, clube, clausula) -> 0/1: a liga em [0xadadfc][id da divisao], vtable +0x98
#   has_automatic_promotion e +0x9c has_relegation thiscall; a idade (+0x18, byte com sinal) > 31;
# - 0x867b70 / 0x867c50 thiscall(contrato) ret: 0x43f4e0 Cash::step_up / 0x43f580 Cash::step_down thiscall(&dinheiro,
#   4 ou 8) ret 4; quadro SEH so pelo estado do dinheiro local;
# - 0x867ef0 / 0x868020 thiscall(contrato, saida, tamanho) ret 8: tamanho < 20 a caixa (staff_contracts.cpp 0x213f /
#   0x2178) e 0.
# Defeitos do original: format_bonus_text com tipo fora de 0..2 le o valor alem dos tres bonus;
# get_release_clause_abbreviation grava 1 na marca de multa minima que tem outro valor (uma leitura que muda o contrato).
# Comparadores: lado a lado sobre o contrato e o texto (contract_compare do sx20_11); os de tipo, sem contrato; so a
# original quando o tamanho abre a caixa.
TELAS = True
ROTINAS = [('staff_contract_format_bonus_text', 0x85f830, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('staff_contract_format_release_clause_name', 0x85f940, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('staff_contract_format_expiry_text', 0x85fa30, [0x8a, 0x44, 0x24, 0x08, 0x56]),
           ('staff_contract_get_bonus_type_name', 0x8679a0, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('staff_contract_get_release_clause_name', 0x867a00, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('staff_contract_is_clause_allowed', 0x867ad0, [0x8b, 0x44, 0x24, 0x08, 0x33, 0xc9]),
           ('staff_contract_increase_wage_step', 0x867b70, [0x6a, 0xff, 0x68, 0xc8, 0x2a, 0x96, 0x00]),
           ('staff_contract_decrease_wage_step', 0x867c50, [0x6a, 0xff, 0x68, 0xe8, 0x2a, 0x96, 0x00]),
           ('staff_contract_get_squad_status_text', 0x867ef0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('staff_contract_get_release_clause_abbreviation', 0x868020, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_F = 'staff_contracts.cpp'
DOCS = ('o contrato nas telas: textos de bonus, clausulas e vencimento, passos do salario, status no elenco',
        'the contract on the screens: bonus, clause and expiry texts, wage steps, squad status',
        [('staff_contract_format_bonus_text', 'staff_contract_format_bonus_text', '0x85f830', _F, 'thiscall, ret 0x10',
          'escreve o texto de bonus do painel do contrato: modo 1, so o nome do bonus; senao o valor em dinheiro (bandeiras 0xa) seguido do nome',
          "writes the contract panel's bonus text: mode 1, the bonus name alone; otherwise its cash amount (flags 0xa) followed by the name"),
         ('staff_contract_format_release_clause_name', 'staff_contract_format_release_clause_name', '0x85f940', _F, 'thiscall, ret 0x10',
          'escreve o nome completo da clausula de rescisao (0 a 4; outro, "Invalid clause type") para o painel do contrato',
          'writes the full name of the release clause (0 to 4; any other, "Invalid clause type") for the contract panel'),
         ('staff_contract_format_expiry_text', 'STAFF_CONTRACT::format_expiry_text', '0x85fa30', _F, 'thiscall, ret 8',
          'escreve o fim do contrato como mes e ano ("Jan <ano>" a "Dec <ano>") com estilo 1 e contrato normal (nada para mes fora de 0..11); senao como o texto de fim: expirado, livre, renovacao automatica ou a data curta',
          'writes the contract end as month and year ("Jan <year>" to "Dec <year>") with style 1 and a normal contract (nothing for a month outside 0..11); otherwise as the end text: expired, free, rolling or the short date'),
         ('staff_contract_get_bonus_type_name', 'staff_contract_get_bonus_type_name', '0x8679a0', _F, 'cdecl',
          'escreve o nome do tipo de bonus (gol, assistencia, jogo sem sofrer gol; outro, "Invalid bonus type")',
          'writes the bonus type name (goal, assist, clean sheet; any other, "Invalid bonus type")'),
         ('staff_contract_get_release_clause_name', 'staff_contract_get_release_clause_name', '0x867a00', _F, 'cdecl',
          'escreve o nome completo da clausula de rescisao',
          'writes the full name of the release clause'),
         ('staff_contract_is_clause_allowed', 'staff_contract_is_clause_allowed', '0x867ad0', _F, 'cdecl',
          'diz se a clausula cabe num contrato da pessoa com o clube: nao promocao so com promocao automatica na liga do clube, multa minima sempre, nao jogar nunca, rebaixamento so com rebaixamento na liga, cargo de tecnico so para quem tem mais de 31 anos; outra, nao',
          "says whether the clause fits a contract of the person with the club: non promotion only with automatic promotion in the club's league, minimum fee always, non playing never, relegation only with relegation in the league, manager job only for someone over 31; any other, no"),
         ('staff_contract_increase_wage_step', 'STAFF_CONTRACT::increase_wage_step', '0x867b70', _F, 'thiscall',
          'sobe o salario um passo de exibicao no periodo das opcoes: por ano sobre o valor anual (x 52, bandeiras 4, volta / 52), por mes sobre o mensal (x 4, 8, / 4), senao sobre o semanal (8)',
          'raises the wage by one display step in the options period: per annum on the yearly amount (x 52, flags 4, back / 52), per month on the monthly one (x 4, 8, / 4), otherwise on the weekly one (8)'),
         ('staff_contract_decrease_wage_step', 'STAFF_CONTRACT::decrease_wage_step', '0x867c50', _F, 'thiscall',
          'desce o salario um passo de exibicao, do mesmo jeito',
          'lowers the wage by one display step, the same way'),
         ('staff_contract_get_squad_status_text', 'STAFF_CONTRACT::get_squad_status_text', '0x867ef0', _F, 'thiscall, ret 8',
          'escreve o nome do status no elenco (Undefined, Key, First Team, Rotation, Backup, Prospect, Youngster; acima, Not Needed) e devolve 1; buffer menor que 20, a caixa e 0',
          'writes the squad status name (Undefined, Key, First Team, Rotation, Backup, Prospect, Youngster; above, Not Needed) and returns 1; a buffer under 20, the box and 0'),
         ('staff_contract_get_release_clause_abbreviation', 'STAFF_CONTRACT::get_release_clause_abbreviation', '0x868020', _F, 'thiscall, ret 8',
          'escreve a abreviacao das clausulas e devolve a situacao: multa minima "Min.Fee" (a marca forcada a 1) e a marca; nao promocao e rebaixamento juntos "Non Pro." (2 com nao promocao 2), "Rlg." (2 com rebaixamento 2) ou "NP & Rlg" (1); uma so, "Non Pro." ou "Rlg." e a marca; cargo de tecnico "Man." e a marca; nenhuma "-" e 0; buffer menor que 20, a caixa e 0',
          'writes the clauses abbreviation and returns the state: minimum fee "Min.Fee" (the flag forced to 1) and the flag; non promotion and relegation together "Non Pro." (2 with non promotion 2), "Rlg." (2 with relegation 2) or "NP & Rlg" (1); one alone, "Non Pro." or "Rlg." and the flag; manager job "Man." and the flag; none "-" and 0; a buffer under 20, the box and 0')])
