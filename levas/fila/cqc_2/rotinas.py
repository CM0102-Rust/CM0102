# O teto salarial (fin_prm.cpp; os asserts citam finance.CPP): Finance::get_max_wage (0x5968d0, 0x14ac bytes com x87,
# 25 chamadores: propostas de contrato e de transferência da IA, a lista de observação, os contratos do staff, as telas
# e a folha salarial 0x598370, já em Rust, que refaz o teto +0x14 das finanças com pessoa 0 e cargo −1) e, passageiras,
# as notícias do fim de temporada do clube: FinanceTable::news_ground_works (0x59ffb0, 0x9c bytes, também pelos eventos
# da semana 0x59ed70) e FinanceTable::news_request_refused (0x59fae0, 0xaa bytes), com quadro SEH só da notícia local,
# sem destrutor no caminho normal. Iguais no cm0102-1080.exe (corpo inteiro das três, com as tabelas de salto). As
# imediatas 0x1e46 da 0x5968d0 (0x59714d, 0x597a92) são as que os patchers da comunidade trocam: a assinatura da rotina
# deixa o exe com patch na original.
# O x87 do jogo roda em precisão dupla: as contas em f64 na ordem da original (fild/fmul/fdivp/fidiv/fiadd/fimul), o
# fator da nação guardado em [esp + 0x3c] (0,25 no começo; a regra do cargo 3 o regrava), o _ftol (0x945640) pela
# `ftol` do motor.rs, as constantes lidas do próprio exe (f64_em/f32_em) e as tabelas de salários (i64) lidas da memória.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x53b900 (e_selecao), 0x53d7f0 (clube basco): push clube; add esp, 4 -> cdecl.
# - 0x5986b0 (faixa de riqueza): mov ecx, finanças; sem pushes -> thiscall.
# - 0x600a50 (clube humano): 5 pushes (0, 0, 1, a, clube); add esp, 0x14 -> cdecl(clube, a, 1, 0, 0), a = 0 ou 1.
# - 0x4cf5f0 (contrato com o clube): mov ecx, [0xae1050]; push clube; push pessoa -> thiscall, ret 8.
# - 0x4d0c20 (valor do contrato): mov ecx, [0xae1050]; push pessoa -> thiscall, ret 4.
# - 0x54cdb0 (dias até): lea ecx, contrato + 0x2d; push [0xae2c94]; push [0xae2c90] -> thiscall, ret 8.
# - 0x547810 (gosta de): push presidente; push pessoa; add esp, 8 -> cdecl(pessoa, presidente).
# - 0x547790 (favorito de): push clube; push pessoa; add esp, 8 -> cdecl(pessoa, clube).
# - 0x945640 (_ftol): st0, sem pilha -> `ftol`.
# - Nas notícias: 0x77bbe0 (lea ecx, local; push 0; push tipo -> thiscall, ret 8), 0x785790 (lea ecx, local; push
#   valor; push índice -> thiscall, ret 8), 0x7861e0 (mov ecx, [0xb7431c]; push clube; push &local -> thiscall, ret 8),
#   0x90cfc0 (push 2; add esp, 4 -> cdecl).
# - Elas pelos chamadores: 0x59ffb0 mov ecx, tabela; push tipo; push valor; push clube -> thiscall, ret 0xc; 0x59fae0
#   mov ecx, tabela; push motivo; push assunto; push clube -> thiscall, ret 0xc.
# Comparadores: a 0x5968d0 pela devolução (só lê; as chamadas de dentro também só leem); as notícias só a original
# (entregam ao técnico humano; a recusa sorteia).
SAVE = False
ROTINAS = [('teto_salarial', 0x5968d0, [0x83, 0xec, 0x34, 0x53, 0x56]),
           ('noticia_das_obras', 0x59ffb0, [0x6a, 0xff, 0x68, 0x6b, 0x91, 0x95, 0x00]),
           ('noticia_da_obra_recusada', 0x59fae0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o teto salarial: o salário máximo que o clube oferece e as notícias das obras e das obras recusadas',
        'the wage ceiling: the highest wage a club offers and the ground works and refused request news',
        [('teto_salarial', 'Finance::get_max_wage', '0x5968d0', 'fin_prm.cpp', 'thiscall, ret 8',
          'o salário máximo pela reputação, faixa de riqueza e tabela da nação; com pessoa, pelo contrato atual, a reputação ao quadrado, os cargos, o teto do humano, o mercado, o contrato perto do fim e o presidente; sem pessoa, pela tabela de salários menos a folha e a verba por semana, com o piso, a liga e a nação; ajustes do clube de +0x64 1 e 2, o mínimo da liga + 100 e o menor dos dois tetos',
          'the highest wage by reputation, wealth band and nation table; with a person, by the current contract, reputation squared, the roles, the human ceiling, the market, the contract near its end and the chairman; without a person, by the wage table less the payroll and budget per week, with the floor, the league and the nation; adjustments for clubs of +0x64 1 and 2, the league minimum + 100 and the lower of both ceilings'),
         ('noticia_das_obras', 'FinanceTable::news_ground_works', '0x59ffb0', 'fin_prm.cpp', 'thiscall, ret 0xc',
          'a notícia 0x177d com o id do clube, o valor e o tipo da obra, entregue ao técnico do clube',
          'news 0x177d with the club id, the amount and the works type, sent to the club manager'),
         ('noticia_da_obra_recusada', 'FinanceTable::news_request_refused', '0x59fae0', 'fin_prm.cpp', 'thiscall, ret 0xc',
          'sem clube, nada; a notícia 0x1788 com o assunto e o motivo (2, ou o sorteio de 2), entregue ao técnico do clube',
          'no club, nothing; news 0x1788 with the subject and the reason (2, or a draw of 2), sent to the club manager')])
CAMPOS = [('finanças (0x167 bytes) +0 o saldo (i64), +8 o id do clube, +0xc a folha, +0x10 o piso, +0x14 o teto por jogador; valor; lê (teto_salarial)',
           'finances (0x167 bytes) +0 the balance (i64), +8 the club id, +0xc the payroll, +0x10 the floor, +0x14 the per-player ceiling; value; reads (teto_salarial)'),
          ('clube +0x53 a nação (+0x7e o tipo, +0x85 o índice salarial), +0x57 / +0x5b as ligas (+0x69 o tamanho), +0x64 o tipo do clube (3: 0), +0x80 a reputação, +0x82 a marca, +0xbf o presidente (+0x57, +0x69 → +0xf, +0x20), +0xcf o técnico (+0x57, +0x69 → +0xa); ponteiro e valor; lê (teto_salarial)',
           'club +0x53 the nation (+0x7e type, +0x85 wage index), +0x57 / +0x5b the leagues (+0x69 size), +0x64 the club type (3: 0), +0x80 reputation, +0x82 the flag, +0xbf the chairman (+0x57, +0x69 → +0xf, +0x20), +0xcf the manager (+0x57, +0x69 → +0xa); pointer and value; reads (teto_salarial)'),
          ('pessoa +0x18 a idade, +0x22 a marca, +0x39 o clube, +0x3d o tipo, +0x61 o não-jogador (+5 a experiência, +0xb / +0xd as reputações), +0x69 o jogador; ponteiro e valor; lê (teto_salarial)',
           'person +0x18 age, +0x22 the flag, +0x39 the club, +0x3d the type, +0x61 the non-player (+5 experience, +0xb / +0xd reputations), +0x69 the player; pointer and value; reads (teto_salarial)'),
          ('contrato +0x2d o fim, +0x4f o cargo (nibble alto); valor; lê (teto_salarial)',
           'contract +0x2d the end date, +0x4f the role (high nibble); value; reads (teto_salarial)'),
          ('tabelas de salários 0x9c8498 / 0x9c8548 / 0x9c85f8 / 0x9c86a8 e pisos 0x9c83e8 / 0x9c8338 (i64 por fator / 10); valor; lê (teto_salarial)',
           'wage tables 0x9c8498 / 0x9c8548 / 0x9c85f8 / 0x9c86a8 and floors 0x9c83e8 / 0x9c8338 (i64 per factor / 10); value; reads (teto_salarial)'),
          ('[[0xde1f54] + 0x856] + 0xdc: o mercado por clube (registros de 9 bytes, +5 a palavra); valor; lê (teto_salarial)',
           '[[0xde1f54] + 0x856] + 0xdc: the per-club market (9-byte records, +5 the word); value; reads (teto_salarial)')]
INTERESSA = ['CMRTE']
