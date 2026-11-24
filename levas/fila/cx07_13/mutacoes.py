# Item de TELAS (cx07_13): sem mutacoes na cadeia; a prova e o Lucas jogando comparando. Para provar o comparador lado
# a lado das cores numa sessao dele (roda na abertura do video):
# - o azul escuro montado com 0x90 de azul em vez de 0x80 (acusada pelo comparador das 25 palavras).
MUTACOES = [
    ('cx07_13_azul_escuro_trocado',
     '(G_GUI_COLOUR_BLUE, 0, 0, 0xff), (0xaebdde, 0, 0x80, 0xff), (G_GUI_COLOUR_DARK_BLUE, 0, 0, 0x80), (0xaebdd6, 0, 0, 0x60),',
     '(G_GUI_COLOUR_BLUE, 0, 0, 0xff), (0xaebdde, 0, 0x80, 0xff), (G_GUI_COLOUR_DARK_BLUE, 0, 0, 0x90), (0xaebdd6, 0, 0, 0x60), // mutation'),
]
