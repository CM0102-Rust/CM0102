# Mutacoes do clube e do time B. As tres so rodam nas telas (fisioterapeutas e time B: telas da pessoa, staff_screens;
# pessoas fora do elenco: aba de informacoes da tela do clube): a carreira oculta nao passa por elas e ESCAPAM
# (lacuna); os comparadores rodam no jogo real comparando, nas telas (ver o LEIA da leva).
MUTACOES = [
    ('dbt_fisioterapeutas', 'validos(clube) + if b != 0 { validos(b) } else { 0 } // os do clube e os do time B (dbt)', 'validos(clube) * 2 + if b != 0 { validos(b) } else { 0 } // mutacao: os do clube contados duas vezes'),
    ('dbt_time_b', '(dbt1_parceiro(clube, &mut eh_a) != 0 && eh_a == 0) as u32 // é o B (dbt)', '(dbt1_parceiro(clube, &mut eh_a) != 0 && eh_a != 0) as u32 // mutacao: o A em vez do B'),
    ('dbt_fora_do_elenco', '*saida.add(n as usize) = p; // (dbt)', '*saida.add(n as usize) = p + 1; // mutacao: o ponteiro errado'),
]
