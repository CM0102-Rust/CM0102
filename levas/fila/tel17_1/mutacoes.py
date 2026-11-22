# Mutacoes (tel17_1): a carreira automatica nao abre telas; estas provam os comparadores numa sessao do Lucas
# (JOGAR_COMPARANDO.bat):
# - o passo das parcelas vira 7 (a tela de proposta: Monthly Installments, os botoes - e +; o comparador da resposta
#   ve a proposta e os widgets);
# - a taxa acima do orcamento deixa de ficar vermelha e a de dentro fica (a tela de proposta de um clube do humano
#   comprando, Transfer Fee).
MUTACOES = [
    ('tel17_1_passo_parcelas',
     "write_i8(at, read_i8(at).wrapping_add(if down { -step } else { step }));",
     "write_i8(at, read_i8(at).wrapping_add(if down { -step } else { step + 1 })); // mutation: instalments step 7",
     'tel17_1 bid screen: instalments or sell-on step'),
    ('tel17_1_orcamento',
     "if bidding != 0 && ts_over_budget(amount, budget)",
     "if bidding != 0 && !ts_over_budget(amount, budget)",
     'tel17_1 bid screen: the transfer budget'),
]
