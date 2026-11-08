# Mutacoes (cqf_3), no contrato oferecido (0x4cda70; 6093 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10, pelo dia dos contratos e pelos lances da IA):
# - o fim "no ano que vem" vira daqui a 2 anos: os contratos dos jogadores mais velhos ficam mais longos (contador
#   "cqf_3 ends next year");
# - acima do teto, o valor fica em 90% do teto: as propostas saem mais baratas (contador "cqf_3 above the cap");
# - o teto pelo total fica na metade: mais recusas por valor acima do teto (contador "cqf_3 cap by total").
MUTACOES = [
    ('cqf3_fim_em_dois_anos',
     'cqf3_pw(c + 0x2f, ano.wrapping_add(1)); // cqf3: o fim no ano que vem',
     'cqf3_pw(c + 0x2f, ano.wrapping_add(2)); // mutacao: 2 anos',
     'cqf_3 ends next year'),
    ('cqf3_valor_abaixo_do_teto',
     'cqf3_pd(c + 0xc, match_pl_ftol(teto) as u32); // cqf3: o valor no teto',
     'cqf3_pd(c + 0xc, match_pl_ftol(teto * 0.9) as u32); // mutacao: 90% do teto',
     'cqf_3 above the cap'),
    ('cqf3_teto_pela_metade',
     'teto = q as f64; // cqf3: o teto pelo total',
     'teto = q as f64 * 0.5; // mutacao: metade',
     'cqf_3 cap by total'),
]
