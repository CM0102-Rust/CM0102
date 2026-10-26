# Mutacoes dos atributos vistos por quem olha. As tres so rodam nas telas (busca e filtros, perfil e comparacao de
# jogadores, tela do clube) e as duas da nevoa so tem conta com a nevoa ligada ([0x9cc270]); a carreira oculta nao
# passa por elas e ESCAPAM (lacuna); os comparadores rodam no jogo real comparando, nas telas (ver o LEIA da leva).
MUTACOES = [
    ('dbt_atributo_visivel', 'let divisor = if k == 2 { 2 } else { 4 }; // (dbt)', 'let divisor = if k == 2 { 4 } else { 2 }; // mutacao: os divisores trocados'),
    ('dbt_atributo_para_quem_ve', 'return 0xf9; // desconhecido (dbt)', 'return 0xfa; // mutacao: outro desconhecido'),
    ('dbt_nome_do_atributo', '        formatar(texto, curto); // (dbt)', '        formatar(texto, longo); // mutacao: o nome inteiro no abreviado'),
]
