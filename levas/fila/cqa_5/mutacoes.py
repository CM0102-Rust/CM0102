# Mutacoes (cqa_5), na vaga no elenco para a oferta (0x8cd450), que as restricoes da oferta (0x8cfc90) conferem a cada
# lance avaliado, concluido ou reconfirmado pelas copias Rust das transferencias (pelo comprador e pelo vendedor):
# - o vendedor sem troca (o caso de quase toda oferta vista pelo vendedor) passa a nao ter vaga: as restricoes seguram
#   as ofertas, que recusam ou desistem, e as transferencias da carreira mudam;
# - cada um do elenco contando dois: o comprador com 26 ou mais jogadores (com o parceiro) passa do limit de 50 e a
#   oferta cai pelo motivo 4 (ou o comprador do computador libera o mais barato);
# - o jogador da oferta sem contar: a conta fica em 0 e o limit de 50 nunca segura o comprador; pega se na base
#   algum comprador bateu no limit (contador "cqa_5 squad full").
# Os contadores ficam colados no efeito: o retorno do vendedor sem troca, a contagem do elenco e a falha do elenco cheio.
MUTACOES = [
    ('cqa5_vendedor_sem_troca_sem_vaga',
     'return 1; // cqa5: o vendedor sem troca tem vaga',
     'return fail_with_reason(clube, o, motivo, 4); // mutacao: o vendedor sem troca nao tem vaga',
     'cqa_5 seller without exchange'),
    ('cqa5_elenco_conta_dois',
     '*n = n.wrapping_add(1); // cqa5: cada um do elenco conta',
     '*n = n.wrapping_add(2); // mutacao: cada um do elenco conta dois',
     'cqa_5 counts the squad'),
    ('cqa5_jogador_da_oferta_nao_conta',
     'n = n.wrapping_add(1); // cqa5: o jogador da oferta conta',
     'n = n.wrapping_add(0); // mutacao: o jogador da oferta nao conta',
     'cqa_5 squad full'),
]
