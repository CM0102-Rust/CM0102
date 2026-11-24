# Mutacoes (sx09c_5), pela carreira ENG ate 2002-06-01 no modo Rust (os tres comparadores so rodam a original):
# - a suspensao por agressao ao arbitro de 90 dias vira de 30: o jogador volta antes e joga as partidas que perderia.
#   So roda quando um cartao vermelho traz o codigo 12 (agressao ao arbitro); se nao acontecer na carreira ENG ate
#   2002-06-01, escapa (o ramo nao roda).
# O construtor e o destrutor ficam sem mutacao: o que eles montam e estrutural (qualquer desvio derruba o jogo ou nao
# muda nada: o ban_history.tmp so e lido pelas telas); a carreira no modo Rust, que cria o jogo do zero, os prova.
MUTACOES = [
    ('sx09c_agressao_30_dias',
     'write_u16(b + ban_record::DAYS, 0x5a);',
     'write_u16(b + ban_record::DAYS, 0x1e); // mutation',
     'sx09c_5 referee assault'),
]
