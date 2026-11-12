# Mutacoes (sma_5). A rotina roda uma vez, na criacao do jogo (Discipline), e os orgaos de cada liga selecionada dao as
# regras de cartoes e suspensoes da temporada:
# - o orgao ingles construido com o argumento 0 (como o original faz com dados antigos nos dois ultimos orgaos): a
#   sessao principal confere se muda o resultado sem travar (pedido de 12/11/2026, no lugar do orgao escoces).
MUTACOES = [
    ('sma5_orgao_ingles_argumento_0',
     "let a = if slot == 0x24 { a } else { a }; // sma5: the English rule set's argument (the mutation's anchor)",
     'let a = if slot == 0x24 { 0 } else { a }; // mutation: the English rule set built with argument 0',
     'sma_5 english rule set'),
]
