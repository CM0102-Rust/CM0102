# Mutacoes. Todas na nacao-chave do jogo (0x48dd70, a chave das listas de jogos do FIXTURE_MANAGER). A busca dos jogos de
# um clube (find_fixtures 0x5acbe0) procura nas chaves 2 a 7 e na da nacao do clube (0x48dee0, o byte +0x14 da linha):
# a liga guardada na chave seguinte some da busca dos clubes (os jogos da liga nao saem) e o amistoso entre clubes da
# mesma nacao tambem (os amistosos de pre-temporada e os marcados pela 0x5c2460). A Copa do Mundo na chave 1 continua
# achada pelas selecoes (chaves 0 e 1): pega so pela simulacao rapida (0x6b66a0, quando ela chega a 0x48dd70) e pelo peso
# dos pontos do ranking (fora da impressao) - pode escapar. Chamada a cada jogo procurado: sem contador.
MUTACOES = [
    ('cpt3_liga_na_chave_seguinte', 'Some(k) => *((0xb63d74 + 72 * k) as *const u8), // cpt3: a nacao-chave da competicao',
     'Some(k) => (*((0xb63d74 + 72 * k) as *const u8)).wrapping_add(1), // mutacao: a chave seguinte'),
    ('cpt3_amistoso_na_chave_seguinte', 'return *((0xb63d74 + 72 * k) as *const u8); // cpt3: a nacao do amistoso',
     'return (*((0xb63d74 + 72 * k) as *const u8)).wrapping_add(1); // mutacao: a chave seguinte'),
    ('cpt3_copa_como_internacional', 'return 0; // cpt3: Copa do Mundo e eliminatorias',
     'return 1; // mutacao: a Copa do Mundo como torneio internacional'),
]
