# Mutacoes das ordens e do clube da CEE. As tres so rodam nas telas (oferta de contrato; menu dos times do clube e
# Escolher Amistosos; aba de informacoes da tela do clube): a carreira oculta nao passa por elas e ESCAPAM (lacuna);
# os comparadores rodam no jogo real comparando, nas telas (ver o LEIA da leva).
MUTACOES = [
    ('dbt_pessoas_pelo_sobrenome', 'formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 4); // "sobrenome, nome" (dbt)', 'formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 1); // mutacao: o nome completo de um lado'),
    ('dbt_clubes_pela_reputacao', '        1 // a de menor reputação depois (dbt)', '        -1 // mutacao: a de menor reputacao antes'),
    ('dbt_clube_na_cee', '*((nacao + 0x7f) as *const u8) == 2) as u32 // na CEE (dbt)', '*((nacao + 0x7f) as *const u8) != 2) as u32 // mutacao: fora da CEE'),
]
