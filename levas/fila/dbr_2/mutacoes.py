# Mutacoes da Polinesia, do nome do cargo e dos anos em texto. As tres PODEM ESCAPAR da carreira ate 2002-06-01:
# a da Polinesia (tira a ultima chave) so roda nas copas da Oceania, que a carreira nao joga (lacuna); a do cargo
# troca "Coach" por "Manager" e so muda o texto da regeneracao da comissao (o que o texto alimenta pode nao mudar o
# estado da carreira); a dos anos (sem o desconto do aniversario) so roda nas telas de recordes, que a carreira de
# ferias nao abre (lacuna). Os comparadores rodam no jogo real comparando (o do cargo na regeneracao; o dos anos ao
# abrir os recordes do clube ou da competicao).
MUTACOES = [
    ('dbr_polinesia', '0x9cf2a4, 0x9cf514]; // polinésias', '0x9cf2a4, 0x9cf2a4]; // mutacao: sem a ultima polinesia'),
    ('dbr_nome_do_cargo', '        3 | 10 => (0x9b6448, 1),       // "Coach"', '        3 | 10 => (0x9b6440, 1),       // mutacao: "Manager" no lugar de "Coach"'),
    ('dbr_anos_em_texto', '        anos = anos.wrapping_sub(1); // o aniversário ainda não chegou', '        // mutacao: sem o desconto do aniversario'),
]
