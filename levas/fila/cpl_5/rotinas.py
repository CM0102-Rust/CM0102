# O contrato da comissão (staff_contracts.cpp): zerar a moral do contrato novo (STAFF_CONTRACT::reset_morale_state
# 0x866ee0, pela assinatura 0x4cf720 nas três formas), montar a proposta de transferência ou empréstimo pelo contrato
# atual (STAFF_CONTRACT::init_from_current_contract 0x85e590, pelas transferências 0x8bede4/0x8befca/0x8bf58e/
# 0x8c1218; 0x4cf5a0, 0x549c90 e 0x7c6830 já em Rust) e a faixa do vencimento do contrato
# (STAFF_CONTRACT::get_expiry_band 0x867d30, ~17 chamadas, a maioria no passe diário 0x4c7ac0; 0x456740 e 0x4a7f20
# originais pelo endereço). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem SEH, sem
# sorteio; caixa só sem contrato atual (a 0x85e590, como a original). Comparadores: as duas primeiras lado a lado no
# contrato e nos vínculos de opinião da pessoa; a faixa pela devolução.
SAVE = False
ROTINAS = [('zerar_moral_do_contrato', 0x866ee0, [0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb]),
           ('contrato_pelo_atual', 0x85e590, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('faixa_do_vencimento', 0x867d30, [0x56, 0x57, 0x8b, 0xf9, 0x8a, 0x47, 0x35])]
DOCS = ('o contrato da comissão: montar a proposta pelo contrato atual, zerar a moral do contrato novo e a faixa do vencimento',
        'staff contracts: building the offer from the current contract, resetting the new contract\'s morale and the expiry band',
        [('zerar_moral_do_contrato', 'STAFF_CONTRACT::reset_morale_state', '0x866ee0', 'staff_contracts.cpp', 'thiscall, ret 8',
          'zera moral, queixas e felicidade do contrato, limpa os vínculos de opinião da pessoa (0x549c90), põe 4 em +0x4e e o nível do idioma da nação do clube (0x7c6830, até 10) em +0x4f',
          'clears the contract morale, grievances and happiness, the person\'s opinion links (0x549c90), sets +0x4e to 4 and the club nation language level (0x7c6830, up to 10) in +0x4f'),
         ('contrato_pelo_atual', 'STAFF_CONTRACT::init_from_current_contract', '0x85e590', 'staff_contracts.cpp', 'thiscall, ret 0x18',
          'a proposta pelo contrato atual: pessoa, clube, salário × percentual / 100, hoje, o fim, bits, a moral zerada como a 0x866ee0 e as duas cláusulas; sem contrato, caixa',
          'the offer from the current contract: person, club, wage × percent / 100, today, the end date, flags, morale reset as 0x866ee0 and the two clauses; without contract, error box'),
         ('faixa_do_vencimento', 'STAFF_CONTRACT::get_expiry_band', '0x867d30', 'staff_contracts.cpp', 'thiscall',
          'a faixa 0..=14 do fim do contrato contra hoje e as datas-limite do CONTRACT_MANAGER (0xae1010..0xae1090)',
          'the 0..=14 band of the contract end date against today and the CONTRACT_MANAGER limit dates (0xae1010..0xae1090)')])
CAMPOS = [('contrato +0/+4/+8/+0xc: id da pessoa, id do clube, 0, salário; id e valor; grava (contrato_pelo_atual)',
           'contract +0/+4/+8/+0xc: person id, club id, 0, wage; id and value; writes (contrato_pelo_atual)'),
          ('contrato +0x10/+0x14/+0x18: −1; grava (contrato_pelo_atual)', 'contract +0x10/+0x14/+0x18: −1; writes (contrato_pelo_atual)'),
          ('contrato +0x1c: cláusula sem acesso; +0x1d: multa mínima; +0x1e..+0x24: 0; valor; grava (contrato_pelo_atual)',
           'contract +0x1c: non-promotion clause; +0x1d: minimum fee clause; +0x1e..+0x24: 0; value; writes (contrato_pelo_atual)'),
          ('contrato +0x25: data do início (hoje, 0xae2c90); +0x2d: data do fim; data; grava (contrato_pelo_atual), lê +0x2d (faixa_do_vencimento)',
           'contract +0x25: start date (today, 0xae2c90); +0x2d: end date; date; writes (contrato_pelo_atual), reads +0x2d (faixa_do_vencimento)'),
          ('contrato +0x35: bits (& 0x3f tipo); valor; grava & 0xc6 | 6 (contrato_pelo_atual), lê (faixa_do_vencimento)',
           'contract +0x35: flags (& 0x3f type); value; writes & 0xc6 | 6 (contrato_pelo_atual), reads (faixa_do_vencimento)'),
          ('contrato +0x36: salário (cópia); +0x3a, +0x49: 0; +0x3b (16 bits): 0; +0x4a: −1; grava (contrato_pelo_atual)',
           'contract +0x36: wage (copy); +0x3a, +0x49: 0; +0x3b (16 bits): 0; +0x4a: −1; writes (contrato_pelo_atual)'),
          ('contrato +0x3e, +0x3f (felicidade), +0x40, +0x44 (moral), +0x45 (queixas): 0; +0x4e: 4; +0x4f & 0xf: nível do idioma; grava (zerar_moral_do_contrato, contrato_pelo_atual)',
           'contract +0x3e, +0x3f (happiness), +0x40, +0x44 (morale), +0x45 (grievances): 0; +0x4e: 4; +0x4f & 0xf: language level; writes (zerar_moral_do_contrato, contrato_pelo_atual)'),
          ('[0xae2d0c] + id × 0x4f, +0x27/+0x37/+0x3b/+0x4b: vínculos de opinião; −1; grava pela 0x549c90 (zerar_moral_do_contrato, contrato_pelo_atual)',
           '[0xae2d0c] + id × 0x4f, +0x27/+0x37/+0x3b/+0x4b: opinion links; −1; writes through 0x549c90 (zerar_moral_do_contrato, contrato_pelo_atual)'),
          ('clube +0x53: nação; ponteiro; lê; nação +0x77: idioma; valor; lê (zerar_moral_do_contrato, contrato_pelo_atual)',
           'club +0x53: nation; pointer; reads; nation +0x77: language; value; reads (zerar_moral_do_contrato, contrato_pelo_atual)'),
          ('[0xae1010..0xae1090] datas-limite do CONTRACT_MANAGER; [0xae2c90] hoje; data; lê (faixa_do_vencimento, contrato_pelo_atual)',
           '[0xae1010..0xae1090] CONTRACT_MANAGER limit dates; [0xae2c90] today; date; reads (faixa_do_vencimento, contrato_pelo_atual)')]
INTERESSA = ['SGE', 'CMRTE']
