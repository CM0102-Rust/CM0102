# Mutacoes (cqd_12): as tres na 0x84b6f0 (~27 milhoes de chamadas ate 2002-06-01). A devolucao decide se a vaga da lista
# de compras aceita o jogador por compensacao (0x842e10) e se a IA da o lance: com a lei Bosman invertida, os jogadores
# sem Bosman passam e os outros nao; com a comparacao do fator invertida, as faixas 1-8 trocam de resposta; com as
# faixas 9-14 recusando, os contratos perto do fim deixam de ser trocas. Os contadores (de 4096 em 4096) mostram que os
# ramos rodam; podem escapar se nenhuma vaga mudar de jogador ate a data (improvavel com milhoes de chamadas).
MUTACOES = [
    ('cqd12_bosman_invertida', '    if bosman(gerente, pessoa(contrato)) == 0 { // cqd12: sem a lei Bosman',
     '    if bosman(gerente, pessoa(contrato)) != 0 { // mutacao: a lei Bosman invertida',
     'cqd_12 com lei Bosman'),
    ('cqd12_fator_invertido', '    (cqd12_i(vaga + 3) as f64 * fator > cqd12_i(vaga + 0xb) as f64) as u32',
     '    (cqd12_i(vaga + 3) as f64 * fator < cqd12_i(vaga + 0xb) as f64) as u32 // mutacao: comparacao invertida',
     'cqd_12 faixa pelo fator'),
    ('cqd12_longe_recusa', '            return 1; // cqd12: as faixas 9-14',
     '            return 0; // mutacao: as faixas 9-14 recusam',
     'cqd_12 faixa longe'),
]
