# weather.cpp (leitor sx22, rodada 5, 24/11/2026): os textos do tempo e do gramado de uma partida.
# - 0x9233a0 weather_format_description: thiscall(Weather [0xde2f74], saída, tempo) ret 8 (o this não é usado).
#   Tabelas de salto conferidas: 0x9235ec (vento 0..4) e 0x923600 (precipitação 0..9; a 4 cai no inválido). Buffers
#   locais de 0x1f4 bytes: precipitação em S+0, vento em S+0x1f4, temperatura em S+0x3e8. langlib_sprintf 0x66f4e0
#   cdecl nas palavras e na temperatura (0xad4e94, o byte com sinal); sprintf 0x944c9f do CRT no inválido da
#   precipitação e no texto final ("%s, %s" 0x98f6ac ou "%s, %s, %s" 0xad4e88, conferidos os ramos pelo 1º byte do
#   vento e da precipitação). Chamadores: a aba da próxima partida da tela do clube 0x45ccf0 (0x45e84c) e o painel da
#   partida 0x71c270 (0x71d90f), ambos já em Rust e chamando pelo exe.
#   Defeito do original: vento inválido grava "Invalid Wind: %d" no buffer da temperatura (sobrescrito em seguida) e
#   deixa o texto do vento com o lixo da pilha; o comparador não compara esse caso.
# - 0x923b60 pitch_condition_text: thiscall(this, saída, condição) ret 8; a condição é o byte (movsx, até 0x40 sem
#   sinal); tabelas 0x923c70 (bytes) e 0x923c4c (saltos) conferidas. Chamador: a tela do clube 0x45ccf0 (0x45e9a3).
# Iguais no cm0102-1080.exe. Tipo: TELAS. Caminho no jogo: a tela do clube, aba da próxima partida (o tempo e o
# gramado previstos), e o painel da partida no dia de jogo (o tempo).
# Comparadores: a original na saída, a cópia num buffer local; os textos iguais.
TELAS = True
ROTINAS = [('weather_format_description', 0x9233a0, [0x81, 0xec, 0xdc, 0x05, 0x00, 0x00]),
           ('pitch_condition_text', 0x923b60, [0x0f, 0xbe, 0x44, 0x24, 0x08])]
DOCS = ('o tempo: os textos do tempo e do gramado de uma partida',
        'the weather: the weather and pitch texts of a match',
        [('weather_format_description', 'weather_format_description', '0x9233a0', 'weather.cpp', 'thiscall, ret 8',
          'o vento (nenhum, Breezy, Gusty, Strong Wind, Gale), a precipitação (Dry só sem vento, Wet, Drizzle, Shower, Downpour, Sleet, Hail, Light Snow, Heavy Snow; a 4 e as fora da faixa saem como inválidas) e a temperatura, pela tabela de idiomas, juntos por vírgulas sem as partes vazias. Defeito do original: vento inválido deixa o texto do vento com o lixo da pilha',
          'the wind (none, Breezy, Gusty, Strong Wind, Gale), the precipitation (Dry only without wind, Wet, Drizzle, Shower, Downpour, Sleet, Hail, Light Snow, Heavy Snow; 4 and the out-of-range ones come out as invalid) and the temperature, through the language table, joined by commas without the empty parts. Defect of the original: an invalid wind leaves the wind text as stack garbage'),
         ('pitch_condition_text', 'pitch_condition_text', '0x923b60', 'weather.cpp', 'thiscall, ret 8',
          'o texto do gramado pelo bit: 1 Muddy, 2 Damp, 4 Dry, 8 Water-logged, 0x10 Frozen, 0x20 Snowy, 0x40 Hard; o resto Normal',
          'the pitch text by the bit: 1 Muddy, 2 Damp, 4 Dry, 8 Water-logged, 0x10 Frozen, 0x20 Snowy, 0x40 Hard; anything else Normal')])
CAMPOS = []
INTERESSA = []
