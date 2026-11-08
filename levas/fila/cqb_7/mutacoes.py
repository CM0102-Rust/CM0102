# Mutacoes (cqb_7), na permissao de trabalho concedida (0x8c7950), que a copia Rust da permissao na lista de
# observacao (0x84b180) e a da oferta de contrato aceita (0x8c38b0) chamam sem sorteio, e a original das permissoes da
# oferta (0x8c7690) a cada oferta por jogador de fora da UE que vence ("cpl_3 work permit" 64 permissoes
# dadas na base da 261a):
# - o potencial basta acima de 6000 (no lugar de 8500): mais jogadores passam na permissao, as listas de observacao e
#   as transferencias para a Inglaterra mudam;
# - o jovem (menos de 24 anos) basta com habilidade acima de 100 (no lugar de 190): idem;
# - o historico da selecao basta com mais de 5 jogos marcados (no lugar de 11): idem.
# Os contadores ficam colados nas comparacoes mutadas (depois das condicoes que as precedem).
MUTACOES = [
    ('cqb7_potencial_6000',
     'if cqb7_w(ficha + 0xd) > 0x2134 { // cqb7: o potencial acima de 8500',
     'if cqb7_w(ficha + 0xd) > 0x1770 { // mutacao: o potencial acima de 6000',
     'cqb_7 checks the potential'),
    ('cqb7_jovem_100',
     'if cqb7_w(ficha + 7) > 0xbe { // cqb7: o jovem acima de 190',
     'if cqb7_w(ficha + 7) > 0x64 { // mutacao: o jovem acima de 100',
     'cqb_7 checks the youngster'),
    ('cqb7_historico_5',
     'if marcados > 0xb { // cqb7: mais de 11 jogos marcados',
     'if marcados > 5 { // mutacao: mais de 5 jogos marcados',
     'cqb_7 national team caps'),
]
