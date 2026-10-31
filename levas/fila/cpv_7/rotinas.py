# Os salários do pessoal e os bônus (finance.cpp): Finance::get_max_staff_wage 0x597db0 (pela renovação do gerente de
# contratos 0x4cda70, pelo interesse do candidato ao cargo 0x844780, os dois em Rust, e pelas telas de contrato),
# Finance::pay_bonuses 0x599960 (pelo fim do jogo dos contratos CONTRACT_MANAGER::update_after_match 0x4ca6d0, três
# chamadas: todo jogo) e Finance::adjust_max_wage 0x598340 (pela resposta da diretoria 0x6a8810 e pela 0x6aca50:
# passageira). Iguais no cm0102-1080.exe. Sem SEH; x87 em precisão dupla (_ftol 0x945640; a 0x597db0 guarda o fator
# no registrador e o compara com 1,0 antes de usar); a 0x597db0 só chama a 0x53b900 e a 0x5986b0 (puras). Comparadores:
# a 0x597db0 pelo valor; as outras duas lado a lado no registro de finanças e no teto salarial.
SAVE = False
ROTINAS = [('teto_salarial_do_pessoal', 0x597db0, [0x83, 0xec, 0x18, 0x8b, 0x41, 0x08]),
           ('pagar_os_bonus', 0x599960, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x38, 0x01, 0x00, 0x00]),
           ('ajustar_o_teto_salarial', 0x598340, [0x0f, 0xbe, 0x44, 0x24, 0x04])]
DOCS = ('os salários do pessoal e os bônus: o teto salarial do pessoal, o pagamento dos bônus e o ajuste do teto salarial',
        'staff wages and bonuses: the staff wage ceiling, the bonus payment and the wage ceiling adjustment',
        [('teto_salarial_do_pessoal', 'Finance::get_max_staff_wage', '0x597db0', 'finance.cpp', 'thiscall, ret 4',
          'o maior salário oferecido ao pessoal pelo cargo: padrão pelo cargo, nível da nação (+0x85) / tipo, ajustes das nações-chave, peso da liga × 0,2, faixa das finanças, teto do clube (+0x14) e faixa do cargo',
          'the highest wage offered to staff by job: default by job, nation level (+0x85) / type, key nation adjustments, league weight × 0.2, finance band, club ceiling (+0x14) and job range'),
         ('pagar_os_bonus', 'Finance::pay_bonuses', '0x599960', 'finance.cpp', 'thiscall, ret 4',
          'soma o valor nas despesas de bônus (+0x138 e +0x98) e o tira do saldo',
          'adds the amount to the bonus expenses (+0x138 and +0x98) and takes it from the balance'),
         ('ajustar_o_teto_salarial', 'Finance::adjust_max_wage', '0x598340', 'finance.cpp', 'thiscall, ret 4',
          'teto salarial (+0x14) × (percentual + 100)%, truncado',
          'wage ceiling (+0x14) × (percent + 100)%, truncated')])
CAMPOS = [('finanças do clube (0x167 bytes) +8 id do clube, +0x14 teto salarial; valores; lê (teto_salarial_do_pessoal), lê e grava o teto (ajustar_o_teto_salarial)',
           'club finance (0x167 bytes) +8 club id, +0x14 wage ceiling; values; reads (teto_salarial_do_pessoal), reads and writes the ceiling (ajustar_o_teto_salarial)'),
          ('finanças do clube +0 saldo (i64), +0x98 e +0x138 despesas de bônus; valores; grava (pagar_os_bonus)',
           'club finance +0 balance (i64), +0x98 and +0x138 bonus expenses; values; writes (pagar_os_bonus)'),
          ('clube +0x53 nação, +0x57 liga, +0x64 tipo, +0x80 reputação; ponteiros/valores; lê (teto_salarial_do_pessoal)',
           'club +0x53 nation, +0x57 league, +0x64 type, +0x80 reputation; pointers/values; reads (teto_salarial_do_pessoal)'),
          ('nação +0 id (nações-chave 0x9cf244..0x9cf4f8), +0x7e tipo, +0x85 nível dos salários; valores; lê (teto_salarial_do_pessoal)',
           'nation +0 id (key nations 0x9cf244..0x9cf4f8), +0x7e type, +0x85 wage level; values; reads (teto_salarial_do_pessoal)'),
          ('liga +0x69 peso (i16); valor; lê (teto_salarial_do_pessoal)',
           'league +0x69 weight (i16); value; reads (teto_salarial_do_pessoal)')]
INTERESSA = ['CMRTE', 'SGE']
