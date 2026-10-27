# Mutacoes do fim de temporada da liga. A das honras troca vice e terceiro no historico de competicoes e a do registro
# grava o ano seguinte: o historico ([0xae23d4]) entra na impressao do banco, e as 4 honras e os 15 registros da
# carreira ate 2002-06-01 caem no fim de maio: devem acusar. A dos direitos de TV paga o dobro (6 pagamentos na
# carreira): as financas nao entram na impressao do banco; acusa se o dinheiro mudar alguma decisao (contratos,
# transferencias, diretoria) ate junho, e PODE ESCAPAR (o comparando acusa no primeiro pagamento).
MUTACOES = [
    ('cpa_tv_valor', '            somar_tv(financas_do_clube(*(0xae2a38 as *const usize), clube), valor); // o dinheiro da TV',
     '            somar_tv(financas_do_clube(*(0xae2a38 as *const usize), clube), valor.wrapping_mul(2)); // mutacao: o dobro'),
    ('cpa_honras_ordem', '    registrar(liga, campeao, vice, terceiro, destaque) // campeão, vice, terceiro',
     '    registrar(liga, campeao, terceiro, vice, destaque) // mutacao: vice e terceiro trocados'),
    ('cpa_historico_ano', '    core::ptr::write_unaligned((r + 8) as *mut u16, *(0xae2c92 as *const u16)); // o ano',
     '    core::ptr::write_unaligned((r + 8) as *mut u16, (*(0xae2c92 as *const u16)).wrapping_add(1)); // mutacao: o ano seguinte'),
]
