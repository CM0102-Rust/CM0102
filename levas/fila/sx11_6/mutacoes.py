# Mutações (sx11_6): a leva das telas não roda mutação pela carreira; numa sessão do Lucas com JOGAR_COMPARANDO.bat:
# - as opções padrão com outro valor: set_defaults lado a lado acusa na partida do programa (pelo Config::Config) e no
#   Reset da caixa Game Settings.
MUTACOES = [
    ('sx11_6_opcoes_padrao',
     '    write_u32(this + config_offsets::OPTIONS, config_life_fns::DEFAULT_OPTIONS);',
     '    write_u32(this + config_offsets::OPTIONS, config_life_fns::DEFAULT_OPTIONS ^ 0x4000); // mutation'),
]
