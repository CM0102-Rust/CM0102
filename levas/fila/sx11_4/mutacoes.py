# Mutações (sx11_4): a leva das telas não roda mutação pela carreira; numa sessão do Lucas com JOGAR_COMPARANDO.bat:
# - o intervalo de gravação escrito com 1 byte no game.cfg: o plano do apply_and_save (o arquivo relido) acusa no Ok da
#   caixa Game Settings (e na criação do jogo, que aplica a configuração).
# O Config::Config só roda a original no comparador (abre tela, som e arquivos): a cópia só se prova nas carreiras Rust.
MUTACOES = [
    ('sx11_4_dias_1_byte',
     '(this + c::AUTO_SAVE_DAYS, 2, 0x3ac)',
     '(this + c::AUTO_SAVE_DAYS, 1, 0x3ac) /* mutation */'),
]
