# Mutacoes (sx01d_6, telas: a cadeia nao as roda; para provar o comparador numa sessao do Lucas). Os comparadores so
# rodam a original, entao estas so aparecem no modo Rust jogando (o efeito da resposta):
# - a critica ao expulso sem a marca 0x1000 no contrato;
# - a promessa ao jogador que quer ser titular sobe a moral 25 em vez de 15.
MUTACOES = [
    ('sx01d_expulso_sem_marca',
     'exe::staff_contract_set_flags(contract, 0x1000);',
     '// mutation: no flag 0x1000',
     'sx01d_6 sent off criticised'),
    ('sx01d_titular_moral_25',
     'add_contract_morale(contract.addr(), 15);',
     'add_contract_morale(contract.addr(), 25); // mutation',
     'sx01d_6 first team promised'),
]
