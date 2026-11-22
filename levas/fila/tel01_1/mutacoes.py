# Telas do clube (1): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas sao
# para provar o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat (cada uma deve aparecer como divergencia):
# - o mesmo dia conta como posterior (Date::operator>, chamada a toda hora pela simulacao: cai logo no inicio);
# - o item marcado das listas suspensas sem a marca 0x2000 (tela do clube, aba Transferencias, botao View).
MUTACOES = [
    ('tel01_1_mesmo_dia_posterior',
     '(day < read_i16(this)) as u32',
     '(day <= read_i16(this)) as u32'),
    ('tel01_1_item_sem_marca',
     'f(screens_current_display(sm), 0x2002, 0, 0, 0, 0, 0, 0, 0x10',
     'f(screens_current_display(sm), 0x0002, 0, 0, 0, 0, 0, 0, 0x10'),
]
