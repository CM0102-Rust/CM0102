# Mutações (sx11_3): a leva das telas não roda mutação pela carreira; numa sessão do Lucas com JOGAR_COMPARANDO.bat
# o comparador acusa na hora:
# - a moeda padrão do inglês (idioma 1) errada: set_default_currency lado a lado (na partida do programa, pelo
#   Config::Config, e ao trocar o idioma na caixa Game Settings);
# - o intervalo da gravação automática gravado com um dia a mais: Game Settings -> Auto Save -> um intervalo.
MUTACOES = [
    ('sx11_3_moeda_ingles',
     '        1 => 0x18,\n        2..=5 | 7 | 9 => 7,',
     '        1 => 0x17, // mutation\n        2..=5 | 7 | 9 => 7,'),
    ('sx11_3_intervalo',
     '    write_i16(this + config_offsets::AUTO_SAVE_DAYS, d);',
     '    write_i16(this + config_offsets::AUTO_SAVE_DAYS, d + 1); // mutation'),
]
