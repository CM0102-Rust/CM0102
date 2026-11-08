# Mutacoes (cpz_11). A folha (0x598370) roda todo sabado para todo clube com nacao, e o passe diario paga os salarios
# da semana pela folha (+0xc, lida logo depois da chamada em 0x59bf84): sem o pessoal na folha, ou sem o termo da
# injury_word_0c (reputacao² × fator, que nunca e zero com injury_word_0c > 0), todos os clubes pagam menos, o saldo sobe, a faixa
# das financas 0x5986b0 muda e com ela os sorteios da divida, da injecao e do clima; o teto salarial 0x5968d0 tambem
# le a folha. Os dias ate a temporada (0x59b4c0) devolvendo 0 na janela: os ingressos da temporada (0x599a20, "cpv_5
# ingressos vendidos" 512 na base) nao sao vendidos e +0x18/+0x164 sao zerados; o saldo dos clubes muda. A terceira
# escapa se nenhum clube estiver na janela ate 2002-06-01 (o contador mostra).
MUTACOES = [
    ('cpz11_folha_sem_pessoal', '        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(comissao)); // cpz11: o pessoal na folha',
     '        let _ = comissao; // mutacao: sem o pessoal na folha',
     'cpz_11 folha do pessoal'),
    ('cpz11_folha_sem_reputacao', '    let folha = match_pl_ftol(rep.wrapping_mul(rep) as f64 * read_f64_at(k) + cpz11_i(this + 0xc) as f64); // cpz11: a reputação na folha',
     '    let folha = { let _ = k; cpz11_i(this + 0xc) }; // mutacao: sem a reputação na folha',
     'cpz_11 folha pela injury_word_0c'),
    ('cpz11_sem_ingressos', '        return r; // cpz11: os dias até a temporada',
     '        return 0; // mutacao: fora da janela dos ingressos',
     'cpz_11 dias ate a temporada'),
]
