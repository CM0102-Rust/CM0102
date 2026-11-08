// ---------- %%LEVA%%a leva: as chaves fixas de clube e de nação pelo nome e a troca da chave de clube depois do corte da base (index.cpp) ----------

/// As chaves de clube do index.cpp (0x6146b0), na ordem do código: (texto do nome, global que recebe a linha), em 4 cadeias. Vários nomes podem dar na mesma global (grafias).
static CLUB_KEYS: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9e1f08, 0x9d00a4), // MLS All Stars East
        (0x9e1ef4, 0x9d00a8), // MLS All Stars West
        (0x9e1ee8, 0x9d00ac), // River Plate
        (0x9e1ed8, 0x9d00b0), // Vélez Sarsfield
        (0x9e1ec8, 0x9d00b4), // Independiente
        (0x9e1ea4, 0x9d00b8), // Independiente Rivadavia de Mendoza
        (0x9e1e90, 0x9d00bc), // Racing de Córdoba
        (0x9e1e74, 0x9d00c0), // Villa Mitre de Bahía Blanca
        (0x9e1e5c, 0x9d00c4), // Argentino de Rosario
        (0x9e1e50, 0x9d00c8), // Temperley
        (0x9e1e48, 0x9d00cc), // Lanús
        (0x9e1e38, 0x9d00d0), // Boca Juniors
        (0x9e1e18, 0x9d00d4), // Gimnasia y Esgrima de La Plata
        (0x9e1e00, 0x9d00d8), // San Lorenzo de Almagro
        (0x9e1dec, 0x9d00dc), // Argentinos Juniors
        (0x9e1ddc, 0x9d00e0), // Rosario Central
        (0x9e1dc8, 0x9d00e4), // Newell's Old Boys
        (0x9e1dbc, 0x9d00e8), // Racing Club
        (0x9e1da8, 0x9d00ec), // Colón de Santa Fé
        (0x9e1d9c, 0x9d00f0), // Platense
        (0x9e1d88, 0x9d00f4), // Ferro Carril Oeste
        (0x9e1d70, 0x9d00f8), // Estudiantes de La Plata
        (0x9e1d54, 0x9d00fc), // Gimnasia y Esgrima de Jujuy
        (0x9e1d40, 0x9d0100), // Unión de Santa Fé
        (0x9e1d38, 0x9d0104), // Huracán
        (0x9e1d24, 0x9d0108), // Talleres de Córdoba
        (0x9e1d10, 0x9d010c), // Belgrano de Córdoba
        (0x9e1d08, 0x9d0110), // Arsenal
        (0x9e1cf8, 0x9d0114), // Crystal Palace
        (0x9e1ce8, 0x9d0118), // Glasgow Celtic
        (0x9e1cd8, 0x9d011c), // Glasgow Rangers
        (0x98e568, 0x9d0120), // Athletic Club de Bilbao
        (0x98e58c, 0x9d0124), // C.D. Alavés
        (0x98e3c8, 0x9d0128), // Real Sociedad C.F.
        (0x98e2c4, 0x9d012c), // Atlético de Madrid
        (0x9e1cc0, 0x9d0130), // Atlético de Madrid B
        (0x9e1ca4, 0x9d0134), // Athletic Club de Bilbao B
        (0x98e540, 0x9d0138), // F.C. Barcelona
        (0x9e1c90, 0x9d013c), // F.C. Barcelona B
        (0x98e34c, 0x9d0140), // Real Valladolid
        (0x9e1c7c, 0x9d0144), // Real Valladolid B
        (0x98e47c, 0x9d0148), // Málaga C.F.
        (0x9e1c6c, 0x9d014c), // Málaga C.F. B
        (0x98e520, 0x9d0150), // Real Betis Balompié
        (0x9e1c54, 0x9d0154), // Real Betis Balompié B
        (0x98e4fc, 0x9d0158), // Real Club Celta de Vigo
        (0x9e1c38, 0x9d015c), // Real Club Celta de Vigo B
        (0x98e4d8, 0x9d0160), // Deportivo de La Coruña
        (0x9e1c1c, 0x9d0164), // Deportivo de La Coruña B
        (0x98e4b8, 0x9d0168), // R.C.D. Espanyol
        (0x9e1c08, 0x9d016c), // R.C.D. Espanyol B
        (0x9e1bf8, 0x9d0170), // C.D. Logroñés
        (0x9e1be8, 0x9d0174), // C.D. Logroñés B
        (0x98e460, 0x9d0178), // R.C.D. Mallorca
        (0x9e1bd4, 0x9d017c), // R.C.D. Mallorca B
        (0x98e438, 0x9d0180), // Club Atlético Osasuna
        (0x9e1bbc, 0x9d0184), // Club Atlético Osasuna B
        (0x98e0fc, 0x9d0188), // Real Oviedo C.F.
        (0x9e1ba8, 0x9d018c), // Real Oviedo C.F. B
        (0x98e3ec, 0x9d0190), // Real Madrid C.F.
        (0x9e1b94, 0x9d0194), // Real Madrid C.F. B
        (0x98e3a8, 0x9d0198), // Sevilla C.F.
        (0x9e1b84, 0x9d019c), // Sevilla C.F. B
        (0x9e1b6c, 0x9d01a0), // Real Sociedad C.F. B
        (0x9e1b54, 0x9d01a4), // Instituto de Córdoba
        (0x9e1b3c, 0x9d01a8), // San Martín de San Juan
        (0x9e1b20, 0x9d01ac), // Gimnasia y Tiro de Salta
        (0x9e1b0c, 0x9d01b0), // Atlético Rafaela
        (0x9e1af4, 0x9d01b4), // San Martín de Tucumán
        (0x9e1ad4, 0x9d01b8), // Almirante Brown de Arrecifes
        (0x9e1ac0, 0x9d01bc), // Atlético Tucumán
        (0x9e1aa8, 0x9d01c0), // Olimpo de Bahía Blanca
        (0x9e1a90, 0x9d01c4), // Huracán de Corrientes
        (0x9e1a84, 0x9d01c8), // Aldosivi
        (0x9e1a6c, 0x9d01cc), // Cipolletti de Río Negro
        (0x9e1a54, 0x9d01d0), // San Martín de Mendoza
        (0x9e1a44, 0x9d01d4), // Douglas Haig
        (0x9e1a14, 0x9d01d8), // Gimnasia y Esgrima de Concepción del Uruguay
        (0x9e19f8, 0x9d01dc), // Juventud Antoniana de Salta
        (0x9e19e0, 0x9d01e0), // Godoy Cruz de Mendoza
        (0x9e19c4, 0x9d01e0), // Godoy Cruz Antonio Tomba
        (0x9e19b8, 0x9d01e4), // Banfield
        (0x9e19ac, 0x9d01e8), // Los Andes
        (0x9e19a4, 0x9d01ec), // Quilmes
        (0x9e1988, 0x9d01f0), // Central Córdoba de Rosario
        (0x9e1974, 0x9d01f4), // Chacarita Juniors
        (0x9e1968, 0x9d01f8), // All Boys
        (0x9e195c, 0x9d01fc), // San Miguel
        (0x9e194c, 0x9d0200), // Nueva Chicago
        (0x9e1938, 0x9d0204), // Defensa y Justicia
        (0x9e1924, 0x9d0208), // Arsenal de Sarandí
        (0x9e190c, 0x9d020c), // Huracán de Tres Arroyos
        (0x9e18f4, 0x9d0210), // Defensores de Belgrano
        (0x9e18e4, 0x9d0214), // Deportivo Morón
        (0x9e18c8, 0x9d0218), // Estudiantes de Buenos Aires
        (0x9e18c0, 0x9d021c), // Almagro
        (0x9e18b8, 0x9d0220), // Atlanta
        (0x9e18a4, 0x9d0224), // Deportivo Español
        (0x9e1898, 0x9d0228), // El Porvenir
        (0x9e1890, 0x9d022c), // Tigre
        (0x9e1880, 0x9d0230), // Middlesbrough
        (0x9e186c, 0x9d0234), // Newcastle United
        (0x9e1860, 0x9d0234), // Newcastle
        (0x9e1854, 0x9d0238), // Sunderland
        (0x9e1848, 0x9d023c), // Aston Villa
        (0x9e1834, 0x9d0240), // Manchester United
        (0x9e1828, 0x9d0244), // Liverpool
    ],
    // cadeia 2
    &[
        (0x9e1820, 0x9d0248), // Everton
        (0x9e180c, 0x9d024c), // Tottenham Hotspur
        (0x9e1804, 0x9d0250), // Chelsea
        (0x98e36c, 0x9d0254), // Valencia C.F.
        (0x9e17f4, 0x9d0258), // Valencia C.F. B
        (0x9e17e8, 0x9d0260), // Southampton
        (0x9e17cc, 0x9d025c), // Brighton and Hove Albion
        (0x9e17c0, 0x9d0264), // AS Monaco
        (0x9e17b8, 0x9d0268), // FC Metz
        (0x9e17a0, 0x9d026c), // Cruzeiro Esporte Clube
        (0x98a46c, 0x9d0270), // São Paulo Futebol Clube
        (0x98a4f0, 0x9d0274), // Sociedade Esportiva Palmeiras
        (0x98a2a8, 0x9d0278), // Sport Club Corinthians Paulista
        (0x9e1780, 0x9d027c), // Gremio Foot Ball Porto Alegre
        (0x9e175c, 0x9d027c), // Grêmio Foot Ball Porto-Alegrense
        (0x98a304, 0x9d027c), // Grêmio Foot-Ball Porto-Alegrense
        (0x9e173c, 0x9d0354), // Clube de Regatas Vasco da Gama
        (0x9e1718, 0x9d0280), // Club Social y Deportivo Colo Colo
        (0x9e1708, 0x9d0284), // Univ.de Chile
        (0x9e16f8, 0x9d0284), // Univ. de Chile
        (0x9e16c4, 0x9d0284), // Corporación de Fútbol de la Universidad de Chile
        (0x9e1698, 0x9d0284), // Club de Fútbol de la Universidad de Chile
        (0x9e1688, 0x9d0288), // Univ.Catolica
        (0x9e1670, 0x9d0288), // Universidad Catolica
        (0x9e164c, 0x9d0288), // Club Deportivo Universidad Católica
        (0x9e1634, 0x9d028c), // Club Nacional de Fútbol
        (0x9e1620, 0x9d028c), // Nacional Montevideo
        (0x9e1618, 0x9d0290), // Olimpia
        (0x9e1608, 0x9d0290), // Club Olimpia
        (0x9e15f8, 0x9d0294), // Cerro Porteño
        (0x9e15e4, 0x9d0294), // Club Cerro Porteño
        (0x9e15c8, 0x9d0358), // Clube de Regatas Flamengo
        (0x98a2e4, 0x9d0358), // Clube de Regatas do Flamengo
        (0x9e15b0, 0x9d0298), // Club Atlético Peñarol
        (0x9e15a8, 0x9d0298), // Penarol
        (0x9e159c, 0x9d029c), // Elgin City
        (0x9e1590, 0x9d02a0), // Peterhead
        (0x9e157c, 0xb63cdc), // 1.FC Kaiserslautern
        (0x9e155c, 0xb63c9c), // 1.FC Kaiserslautern Amateure
        (0x9e1544, 0xb63c9c), // 1.FC Kaiserslautern (A)
        (0x9e1530, 0xb63cbc), // Bayer 04 Leverkusen
        (0x9e1514, 0xb63ca0), // Bayer Leverkusen Amateure
        (0x9e14fc, 0xb63ca0), // Bayer Leverkusen (A)
        (0x9e14e4, 0xb63ca0), // Bayer 04 Leverkusen U21
        (0x9e14d0, 0xb63ce8), // Borussia Dortmund
        (0x9e14b8, 0xb63ce8), // BV Borussia Dortmund
        (0x9e149c, 0xb63cac), // Borussia Dortmund Amateure
        (0x9e1484, 0xb63cac), // Borussia Dortmund (A)
        (0x9e1470, 0xb63cd8), // FC Bayern München
    ],
    // cadeia 3
    &[
        (0x9e1458, 0xb63cd0), // Bayern München Amateure
        (0x9e143c, 0xb63cd0), // FC Bayern München Amateure
        (0x9e142c, 0xb63cb4), // FC St. Pauli
        (0x9e1414, 0xb63cb0), // FC St. Pauli Amateure
        (0x9e1400, 0xb63cb0), // FC St. Pauli (A)
        (0x9e13f0, 0xb63cd4), // Hamburger SV
        (0x9e13d8, 0xb63ca8), // Hamburger SV Amateure
        (0x9e13c4, 0xb63ca8), // Hamburger SV (A)
        (0x9e13b0, 0xb63ce4), // Hertha BSC Berlin
        (0x9e139c, 0xb63cec), // Hertha BSC Amateure
        (0x9e138c, 0xb63cec), // Hertha BSC (A)
        (0x9e137c, 0xb63cf0), // Karlsruher SC
        (0x9e1364, 0xb63cf8), // Karlsruher SC Amateure
        (0x9e1350, 0xb63cf8), // Karlsruher SC (A)
        (0x9e133c, 0xb63cf8), // Karlsruher SC II
        (0x9e1328, 0xb63cc0), // SV Werder Bremen
        (0x9e130c, 0xb63d00), // SV Werder Bremen Amateure
        (0x9e12f4, 0xb63d00), // SV Werder Bremen (A)
        (0x9e12e0, 0xb63cc8), // TSV München 1860
        (0x9e12cc, 0xb63cc8), // TSV 1860 München
        (0x9e12b0, 0xb63ccc), // TSV München 1860 Amateure
        (0x9e1294, 0xb63ccc), // TSV 1860 München Amateure
        (0x9e127c, 0xb63ce0), // Tennis Borussia Berlin
        (0x9e1260, 0xb63cb8), // Tennis Borussia Amateure
        (0x9e124c, 0xb63cb8), // Tennis Borussia (A)
        (0x9e1230, 0xb63cb8), // Tennis Borussia Berlin II
        (0x9e1220, 0xb63cc4), // VfB Stuttgart
        (0x9e1208, 0xb63cf4), // VfB Stuttgart Amateure
        (0x9e11f4, 0xb63cf4), // VfB Stuttgart (A)
        (0x9e11e8, 0xb63cfc), // VfL Bochum
        (0x9e11d4, 0xb63ca4), // VfL Bochum Amateure
        (0x9e11c4, 0xb63ca4), // VfL Bochum (A)
        (0x9e11b0, 0x9d02a4), // Eintracht Frankfurt
        (0x9e1190, 0x9d02a8), // Eintracht Frankfurt Amateure
        (0x9e1178, 0x9d02a8), // Eintracht Frankfurt (A)
        (0x9e1164, 0x9d02ac), // FC Hansa Rostock
        (0x9e1148, 0x9d02b0), // FC Hansa Rostock Amateure
        (0x9e1130, 0x9d02b0), // FC Hansa Rostock (A)
        (0x9e1120, 0x9d02b4), // FC Schalke 04
        (0x9e1108, 0x9d02b8), // FC Schalke 04 Amateure
        (0x9e10f4, 0x9d02b8), // FC Schalke 04 (A)
        (0x9e10e0, 0x9d02bc), // SG Wattenscheid 09
        (0x9e10c4, 0x9d02c0), // SG Wattenscheid 09 Amateure
        (0x9e10ac, 0x9d02c0), // SG Wattenscheid 09 (A)
        (0x9e1094, 0x9d02c0), // SG Wattenscheid 09 II
        (0x9e107c, 0x9d02c4), // Sport Lisboa e Benfica
        (0x9e1060, 0x9d02c8), // Sport Lisboa e Benfica B
        (0x9e1048, 0x9d02cc), // Sporting Clube de Braga
        (0x9e102c, 0x9d02d0), // Sporting Clube de Braga B
        (0x9e1014, 0x9d02d4), // Clube Sport Marítimo
        (0x9e0ffc, 0x9d02d8), // Clube Sport Marítimo B
        (0x9e0fe4, 0x9d02dc), // Futebol Clube do Porto
        (0x9e0fc8, 0x9d02e0), // Futebol Clube do Porto B
        (0x98e250, 0x9d02e4), // S.D. Éibar
        (0x9e0fb8, 0x9d02e8), // Amurrio C.F.
        (0x9e0fa0, 0x9d02ec), // C.D. Aurrerá Vitoria
        (0x9e0f90, 0x9d02f0), // Barakaldo C.F.
        (0x9e0f80, 0x9d02f4), // S.D. Beasaín
        (0x9e0f74, 0x9d02f8), // Bermeo Club
        (0x9e0f64, 0x9d02fc), // Gernika Club
        (0x9e0f58, 0x9d0304), // S.D. Lemona
        (0x9e0f48, 0x9d0308), // Elgoibar C.D.
        (0x9e0f38, 0x9d030c), // Hernani C.D.
        (0x9e0f28, 0x9d030c), // C.D. Hernani
        (0x9e0f1c, 0x9d0310), // Zalla U.C.
        (0x9e0f08, 0x9d0314), // Cultural Durango
        (0x9e0ef8, 0x9d0318), // S.D. Amorebieta
        (0x9e0ee4, 0x9d031c), // Arenas Getxo Bilbao
        (0x9e0ecc, 0x9d0320), // Aurrerá Ondarroa C.D.
        (0x9e0ebc, 0x9d0324), // Baskonia C.D.
        (0x9e0eac, 0x9d0328), // S.D. Éibar B
        (0x9e0e9c, 0x9d032c), // U.D. San Pedro
        (0x9e0e8c, 0x9d032c), // San Pedro U.D.
        (0x9e0e7c, 0x9d0330), // Santurtzi C.D.
        (0x9e0e68, 0x9d0334), // Sestao River Club
        (0x9e0e5c, 0x9d0338), // Tolosa C.F.
        (0x98a664, 0x9d033c), // Coritiba Football Club
        (0x98a4bc, 0x9d0340), // Santos Futebol Clube
        (0x98a4a4, 0x9d0344), // Sport Club do Recife
        (0x9e0e38, 0x9d0348), // Associação Portugesa de Desportos
        (0x9e0e14, 0x9d0348), // Associação Portuguesa de Desportos
        (0x98a70c, 0x9d0350), // Clube Atlético Mineiro
        (0x9e173c, 0x9d0354), // Clube de Regatas Vasco da Gama
        (0x9e15c8, 0x9d0358), // Clube de Regatas Flamengo
        (0x98a7c0, 0x9d035c), // Sport Club Internacional
        (0x98a800, 0x9d0360), // Esporte Clube Vitória
        (0x9e0dfc, 0x9d0364), // Botafogo Futebol Clube
        (0x98a58c, 0x9d0364), // Botafogo de Futebol e Regatas
        (0x98a2c8, 0x9d0368), // Clube Atlético Paranaense
        (0x98a484, 0x9d036c), // Associação Atlética Ponte Preta
        (0x98a7a8, 0x9d0370), // Esporte Clube Juventude
        (0x9e0de4, 0x9d0374), // Guarani Futebol Clube
        (0x98a6a4, 0x9d0378), // Paraná Clube
        (0x9e0dd4, 0x9d037c), // Chicago Fire
        (0x9e0dc4, 0x9d0380), // Miami Fusion FC
        (0x9e0dac, 0x9d0384), // Washington DC United
        (0x9e0d98, 0x9d0388), // Tampa Bay Mutiny
        (0x9e0d80, 0x9d038c), // San Jose Earthquakes
        (0x9e0d70, 0x9d038c), // San Jose Clash
        (0x9e0d58, 0x9d0390), // New England Revolution
        (0x9e0d44, 0x9d0394), // NY/NJ Metrostars
        (0x9e0d30, 0x9d0394), // NY-NJ Metrostars
        (0x9e0d20, 0x9d0394), // NYNJ Metrostars
        (0x9e0d0c, 0x9d0398), // Los Angeles Galaxy
        (0x9e0cf8, 0x9d039c), // Kansas City Wizards
        (0x9e0cec, 0x9d03a0), // Dallas Burn
        (0x9e0cdc, 0x9d03a4), // Columbus Crew
        (0x9e0ccc, 0x9d03a8), // Colorado Rapids
        (0x98e2e8, 0x9d03ac), // Albacete Balompié
        (0x9e0cb8, 0x9d03b0), // Albacete Balompié B
        (0x98e29c, 0x9d03b4), // C.D. Badajoz
        (0x9e0ca8, 0x9d03b8), // C.D. Badajoz B
        (0x98e19c, 0x9d03bc), // C.D. Leganés
        (0x9e0c98, 0x9d03c0), // C.D. Leganés B
        (0x98e38c, 0x9d03c4), // C.D. Tenerife
        (0x9e0c88, 0x9d03c8), // C.D. Tenerife B
        (0x9e0c7c, 0x9d03cc), // C.P. Mérida
        (0x9e0c6c, 0x9d03d0), // C.P. Mérida B
        (0x9e0c54, 0x9d03d0), // C.P. Mérida Promesas
        (0x9e0c40, 0x9d03d4), // Deportivo Alavés B
        (0x98e1f8, 0x9d03d8), // Extremadura C.F.
        (0x9e0c2c, 0x9d03dc), // Extremadura C.F. B
        (0x98e0d4, 0x9d03e0), // Racing Club de Santander
        (0x9e0c10, 0x9d03e4), // Racing Club de Santander B
        (0x98e410, 0x9d03e8), // Rayo Vallecano de Madrid
        (0x9e0bf4, 0x9d03ec), // Rayo Vallecano de Madrid B
        (0x98e160, 0x9d03f0), // Real Murcia C.F.
        (0x9e0be0, 0x9d03f4), // Real Murcia C.F. B
        (0x98e064, 0x9d03f8), // Real Sporting de Gijón
        (0x9e0bc4, 0x9d03fc), // Real Sporting de Gijón B
        (0x98e30c, 0x9d0400), // Real Zaragoza
        (0x9e0bb4, 0x9d0404), // Real Zaragoza B
        (0x9e0ba4, 0x9d0408), // S.D. Compostela
        (0x9e0b90, 0x9d040c), // S.D. Compostela B
        (0x98e498, 0x9d0410), // U.D. Las Palmas
        (0x9e0b7c, 0x9d0414), // U.D. Las Palmas B
        (0x98e08c, 0x9d0418), // U.D. Salamanca
        (0x9e0b68, 0x9d041c), // U.D. Salamanca B
        (0x9e0b5c, 0x9d0420), // Galatasaray
        (0x9e0b4c, 0x9d0420), // Galatasaray SK
        (0x9e0b3c, 0x9d0424), // BJK Besiktas
        (0x9e0b30, 0x9d0424), // Besiktas JK
        (0x9e0b24, 0x9d0428), // Fenerbahçe
        (0x9e0b14, 0x9d0428), // Fenerbahçe SK
        (0x9e0af4, 0x9d042c), // General Paz Juniors de Córdoba
        (0x9e0ae4, 0x9d0430), // Swansea City
        (0x9e0ad4, 0x9d0434), // Cardiff City
        (0x9e0ac4, 0x9d0438), // Merthyr Tydfil
        (0x9e0abc, 0x9d043c), // Wrexham
        (0x9e0aa0, 0x9d0440), // Sporting Clube De Portugal
        (0x9e0a80, 0x9d0444), // Sporting Clube De Portugal B
    ],
    // cadeia 4
    &[
        (0x9e0a6c, 0x9d0448), // Arminia Bielefeld
        (0x9e0a50, 0x9d044c), // Arminia Bielefeld Amateure
        (0x9e0a38, 0x9d044c), // Arminia Bielefeld II
        (0x9e0a24, 0x9d0450), // FC Energie Cottbus
        (0x9e0a08, 0x9d0454), // FC Energie Cottbus Amateure
        (0x9e09f0, 0x9d0454), // FC Energie Cottbus II
        (0x9e09e0, 0x9d0458), // MSV Duisburg
        (0x9e09c8, 0x9d045c), // MSV Duisburg Amateure
        (0x9e09bc, 0x9d0460), // Hannover 96
        (0x9e09a4, 0x9d0464), // Hannover 96 Amateure
        (0x9e0998, 0x9d0468), // 1.FC Köln
        (0x9e0984, 0x9d046c), // 1.FC Köln Amateure
        (0x9e0974, 0x9d046c), // 1.FC Köln U23
        (0x9e0960, 0x9d0470), // Borussia M'gladbach
        (0x9e094c, 0x9d0470), // Borussia Mgladbach
        (0x9e0930, 0x9d0474), // Bor. M'gladbach Amateure
        (0x9e0920, 0x9d0478), // 1.FC Nürnberg
        (0x9e0908, 0x9d047c), // 1.FC Nürnberg Amateure
        (0x9e08f0, 0x9d0480), // Rot-Weiss Oberhausen
        (0x9e08dc, 0x9d0480), // Rot-Weiß Oberhausen
        (0x9e08c4, 0x9d0484), // RW Oberhausen Amateure
        (0x9e08b8, 0x9d0488), // SC Freiburg
        (0x9e08a0, 0x9d048c), // SC Freiburg Amateure
        (0x9e088c, 0x9d0490), // 1.FC Saarbrücken
        (0x9e0870, 0x9d0494), // 1.FC Saarbrücken Amateure
        (0x9e085c, 0x9d0494), // 1.FC Saarbrücken II
        (0x9e084c, 0x9d0498), // SSV Ulm 1846
        (0x9e0834, 0x9d049c), // SSV Ulm 1846 Amateure
        (0x9e0824, 0x9d049c), // SSV Ulm 1846 II
        (0x9e0810, 0x9d04a0), // SpVgg Unterhaching
        (0x9e07f4, 0x9d04a4), // SpVgg Unterhaching Amateure
        (0x9e07dc, 0x9d04a4), // SpVgg Unterhaching II
        (0x9e07d0, 0x9d04a8), // VfB Leipzig
        (0x9e07b8, 0x9d04ac), // VfB Leipzig Amateure
        (0x9e07a8, 0x9d04ac), // VfB Leipzig II
        (0x9e0798, 0x9d04b0), // VfL Wolfsburg
        (0x9e0780, 0x9d04b4), // VfL Wolfsburg Amateure
        (0x9e0770, 0x9d04b8), // C.D. Castellón
        (0x9e075c, 0x9d04bc), // C.D. Castellón B
        (0x98e29c, 0x9d04c0), // C.D. Badajoz
        (0x9e0ca8, 0x9d04c4), // C.D. Badajoz B
        (0x9e1bf8, 0x9d04c8), // C.D. Logroñés
        (0x9e1be8, 0x9d04cc), // C.D. Logroñés B
        (0x9e074c, 0x9d04d0), // C.D. Ourense
        (0x9e073c, 0x9d04d4), // C.D. Ourense B
        (0x98e268, 0x9d04d8), // Córdoba C.F.
        (0x9e072c, 0x9d04dc), // Córdoba C.F. B
        (0x9e0718, 0x9d04e0), // Cultural Leonesa
        (0x9e0704, 0x9d04e4), // Cultural Leonesa B
        (0x98e218, 0x9d04e8), // Elche C.F.
        (0x9e06f4, 0x9d04ec), // Elche C.F. B
        (0x98e180, 0x9d04f0), // Levante U.D.
        (0x9e06e4, 0x9d04f4), // Levante U.D. B
        (0x9e06d0, 0x9d04f8), // Real Unión de Irún
        (0x9e06b8, 0x9d04fc), // Real Unión de Irún B
        (0x98e0ac, 0x9d0500), // Recreativo de Huelva
        (0x9e06a0, 0x9d0504), // Recreativo de Huelva B
        (0x9e068c, 0x9d0508), // Fortuna Düsseldorf
        (0x9e0674, 0x9d050c), // Fortuna Düsseldorf ii
        (0x9e0668, 0x9d0510), // sg hoechst
        (0x9e0658, 0x9d0514), // sg hoechst ii
        (0x9e0648, 0x9d0518), // fsv mainz 05
        (0x9e0638, 0x9d051c), // fsv mainz 05 ii
        (0x9e0624, 0x9d0520), // stuttgarter kickers
        (0x9e060c, 0x9d0524), // stuttgarter kickers ii
    ],
];

/// As chaves de nação do index.cpp (0x60d810), na ordem do código: (texto do nome, global que recebe a linha), em 2 cadeias.
static NATION_KEYS: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9d9e30, 0x9cf200), // Afghanistan
        (0x9d9e28, 0x9cf204), // Albania
        (0x9d9e20, 0x9cf208), // Algeria
        (0x9d9e18, 0x9cf20c), // Andorra
        (0x9d9e10, 0x9cf210), // Angola
        (0x9d9dfc, 0x9cf214), // Antigua & Barbuda
        (0x9b1698, 0x9cf218), // Argentina
        (0x9d9df4, 0x9cf21c), // Armenia
        (0x9d9dec, 0x9cf220), // Aruba
        (0x9b167c, 0x9cf224), // Australia
        (0x9b1664, 0x9cf228), // Austria
        (0x9d9de0, 0x9cf22c), // Azerbaijan
        (0x9d9dd4, 0x9cf230), // The Bahamas
        (0x9d9dcc, 0x9cf230), // Bahamas
        (0x9d9dc4, 0x9cf234), // Bahrain
        (0x9d9db8, 0x9cf238), // Bangladesh
        (0x9d9dac, 0x9cf23c), // Barbados
        (0x9d9da4, 0x9cf240), // Belarus
        (0x9b164c, 0x9cf244), // Belgium
        (0x9d9d9c, 0x9cf248), // Belize
        (0x9d9d94, 0x9cf24c), // Benin
        (0x9d9d8c, 0x9cf250), // Bermuda
        (0x9d9d84, 0x9cf254), // Bolivia
        (0x9d9d70, 0x9cf258), // Bosnia-Herzegovina
        (0x9d9d64, 0x9cf25c), // Botswana
        (0x9b1620, 0x9cf260), // Brazil
        (0x9d9d50, 0x9cf264), // Brunei Darussalam
        (0x9d9d44, 0x9cf268), // Bulgaria
        (0x9d9d34, 0x9cf26c), // Burkino Faso
        (0x9d9d24, 0x9cf26c), // Burkina Faso
        (0x9d9d1c, 0x9cf274), // Burundi
        (0x9d9cfc, 0x9cf278), // Commonwealth of Soviet States
        (0x9d9cf0, 0x9cf27c), // Cameroon
        (0x9d9ce8, 0x9cf280), // Canada
        (0x9d9cd4, 0x9cf284), // Cape Verde Islands
        (0x9d9cc4, 0x9cf288), // Cayman Islands
        (0x9d9ca8, 0x9cf28c), // Central African Republic
        (0x9d9ca0, 0x9cf290), // Chad
        (0x9d9c98, 0x9cf294), // Chile
        (0x9b1608, 0x9cf298), // China
        (0x9d9c8c, 0x9cf298), // China PR
        (0x9d9c80, 0x9cf29c), // Colombia
        (0x9d9c74, 0x9cf2a0), // The Congo
        (0x9d9c64, 0x9cf2a4), // Cook Islands
        (0x9d9c58, 0x9cf2a8), // Costa Rica
        (0x9b15f4, 0x9cf2ac), // Croatia
        (0x9d9c50, 0x9cf2b0), // Cuba
        (0x9d9c48, 0x9cf2b4), // Cyprus
        (0x9b15d0, 0x9cf2b8), // Czech Republic
        (0x9d9c38, 0x9cf2bc), // Czechoslovakia
        (0x9b15b8, 0x9cf2c0), // Denmark
        (0x9d9c2c, 0x9cf2c4), // Djibouti
        (0x9d9c20, 0x9cf2c8), // Dominica
        (0x9d9c0c, 0x9cf2cc), // Dominican Republic
        (0x9d9bfc, 0x9cf2d0), // East Germany
        (0x9d9bf4, 0x9cf2d4), // Ecuador
        (0x9d9bec, 0x9cf2d8), // Egypt
        (0x9d9bd8, 0x9cf2dc), // Republic Of Ireland
        (0x9d9bcc, 0x9cf2e0), // El Salvador
        (0x9d9bc4, 0x9cf2e4), // England
        (0x9d9bb0, 0x9cf2e8), // Equatorial Guinea
        (0x9d9ba8, 0x9cf2ec), // Estonia
        (0x9d9b9c, 0x9cf2f0), // Ethiopia
        (0x9d9b8c, 0x9cf2f4), // Faroe Islands
        (0x9d9b84, 0x9cf2f8), // Fiji
        (0x9b159c, 0x9cf2fc), // Finland
        (0x9b1584, 0x9cf300), // France
        (0x9d9b70, 0x9cf304), // FYR of Macedonia
        (0x9d9b68, 0x9cf308), // Gabon
        (0x9d9b5c, 0x9cf30c), // The Gambia
        (0x9d9b54, 0x9cf310), // Georgia
        (0x9b1568, 0x9cf314), // Germany
        (0x9d9b4c, 0x9cf318), // Ghana
        (0x9b1550, 0x9cf31c), // Greece
        (0x9d9b44, 0x9cf320), // Grenada
        (0x9d9b38, 0x9cf324), // Guatemala
        (0x9d9b30, 0x9cf328), // Guinea
        (0x9d9b20, 0x9cf32c), // Guinea-Bissau
        (0x9d9b18, 0x9cf330), // Guyana
        (0x9d9b10, 0x9cf334), // Haiti
        (0x9b1534, 0x9cf338), // Holland
        (0x9d9b04, 0x9cf33c), // Honduras
        (0x9d9af8, 0x9cf340), // Hong Kong
        (0x9d9af0, 0x9cf344), // Hungary
        (0x9d9ae8, 0x9cf348), // Iceland
        (0x9d9ae0, 0x9cf34c), // India
        (0x9d9ad4, 0x9cf350), // Indonesia
        (0x9d9acc, 0x9cf354), // Iran
        (0x9d9ac4, 0x9cf358), // Iraq
        (0x9d9abc, 0x9cf35c), // Israel
        (0x9b150c, 0x9cf360), // Italy
        (0x9d9ab0, 0x9cf364), // Ivory Coast
        (0x9d9aa8, 0x9cf368), // Jamaica
        (0x9b14ec, 0x9cf36c), // Japan
        (0x9d9aa0, 0x9cf370), // Jordan
        (0x9d9a94, 0x9cf374), // Kampuchea
        (0x9d9a88, 0x9cf374), // Cambodia
        (0x9d9a7c, 0x9cf378), // Kazakhstan
        (0x9d9a74, 0x9cf37c), // Kenya
        (0x9d9a6c, 0x9cf380), // Kuwait
        (0x9d9a60, 0x9cf384), // Kyrgyzstan
        (0x9d9a58, 0x9cf388), // Laos
        (0x9d9a50, 0x9cf38c), // Latvia
        (0x9d9a48, 0x9cf390), // Lebanon
        (0x9d9a40, 0x9cf394), // Lesotho
        (0x9d9a38, 0x9cf398), // Liberia
        (0x9d9a30, 0x9cf39c), // Libya
        (0x9d9a20, 0x9cf3a0), // Liechtenstein
        (0x9d9a14, 0x9cf3a4), // Lithuania
        (0x9d9a08, 0x9cf3a8), // Luxembourg
        (0x9d9a00, 0x9cf3ac), // Macau
        (0x9d99f4, 0x9cf3b0), // Madagascar
        (0x9d99ec, 0x9cf3b4), // Malawi
        (0x9d99e0, 0x9cf3b8), // Malaysia
        (0x9d99d4, 0x9cf3bc), // Maldives
        (0x9d99cc, 0x9cf3c0), // Mali
        (0x9d99c4, 0x9cf3c4), // Malta
        (0x9d99b8, 0x9cf3c8), // Mauritania
        (0x9d99ac, 0x9cf3cc), // Mauritius
        (0x9b14bc, 0x9cf3d0), // Mexico
        (0x9d99a4, 0x9cf3d4), // Moldova
        (0x9d999c, 0x9cf3d8), // Morocco
        (0x9d9990, 0x9cf3dc), // Mozambique
        (0x9d9988, 0x9cf3e0), // Myanmar
        (0x9d9978, 0x9cf3e0), // Myanmar (Burma)
        (0x9b1494, 0x9cf3e4), // Northern Ireland
        (0x9d9970, 0x9cf3e8), // Namibia
    ],
    // cadeia 2
    &[
        (0x9d9968, 0x9cf3ec), // Nepal
        (0x9d9950, 0x9cf3f0), // Netherlands Antilles
        (0x9d9944, 0x9cf3f4), // New Zealand
        (0x9d9938, 0x9cf3f8), // Nicaragua
        (0x9d9930, 0x9cf3fc), // Niger
        (0x9d9928, 0x9cf400), // Nigeria
        (0x9d991c, 0x9cf404), // North Korea
        (0x9b147c, 0x9cf408), // Norway
        (0x9d9914, 0x9cf40c), // Oman
        (0x9d9908, 0x9cf410), // Pakistan
        (0x9d98fc, 0x9cf414), // Palestine
        (0x9d98f4, 0x9cf418), // Panama
        (0x9d98e0, 0x9cf41c), // Papua New Guinea
        (0x9d98d4, 0x9cf420), // Paraguay
        (0x9d98c8, 0x9cf424), // Pays Basque
        (0x9d98c0, 0x9cf428), // Peru
        (0x9d98b0, 0x9cf42c), // The Philippines
        (0x9b1450, 0x9cf430), // Poland
        (0x9b1430, 0x9cf434), // Portugal
        (0x9d98a4, 0x9cf438), // Puerto Rico
        (0x9d989c, 0x9cf43c), // Qatar
        (0x9d987c, 0x9cf440), // Democratic Republic of Congo
        (0x9d9874, 0x9cf444), // Romania
        (0x9b13ec, 0x9cf448), // Russia
        (0x9d986c, 0x9cf44c), // Rwanda
        (0x9d9860, 0x9cf450), // San Marino
        (0x9d984c, 0x9cf454), // Sao Tome & Principe
        (0x9d9838, 0x9cf454), // São Tomé & Principe
        (0x9d9828, 0x9cf458), // Saudi Arabia
        (0x9b13bc, 0x9cf45c), // Scotland
        (0x9d9820, 0x9cf460), // Senegal
        (0x9d9814, 0x9cf464), // Seychelles
        (0x9d9804, 0x9cf468), // Sierra Leone
        (0x9d97f8, 0x9cf46c), // Singapore
        (0x9d97ec, 0x9cf470), // Slovakia
        (0x9d97e0, 0x9cf474), // Slovenia
        (0x9d97d0, 0x9cf478), // Solomon Islands
        (0x9d97c8, 0x9cf47c), // Somalia
        (0x9b1384, 0x9cf480), // South Africa
        (0x9b14d0, 0x9cf484), // South Korea
        (0x9b136c, 0x9cf488), // Spain
        (0x9d97bc, 0x9cf48c), // Sri Lanka
        (0x9d97a8, 0x9cf490), // St Kitts & Nevis
        (0x9d979c, 0x9cf494), // Saint Lucia
        (0x9d9780, 0x9cf498), // St Vincent & The Grenadines
        (0x9d9778, 0x9cf49c), // Sudan
        (0x9d9770, 0x9cf4a0), // Surinam
        (0x9d9764, 0x9cf4a4), // Swaziland
        (0x9b1334, 0x9cf4a8), // Sweden
        (0x9b1304, 0x9cf4ac), // Switzerland
        (0x9d975c, 0x9cf4b0), // Syria
        (0x9d9754, 0x9cf4b4), // Tahiti
        (0x9d974c, 0x9cf4b8), // Taiwan
        (0x9d973c, 0x9cf4b8), // Chinese Taipei
        (0x9d9730, 0x9cf4bc), // Tajikistan
        (0x9d9724, 0x9cf4c0), // Tanzania
        (0x9d9718, 0x9cf4c4), // Thailand
        (0x9d9710, 0x9cf4c8), // Timor
        (0x9d9708, 0x9cf4cc), // Togo
        (0x9d9700, 0x9cf4d0), // Tonga
        (0x9d96ec, 0x9cf4d4), // Trinidad & Tobago
        (0x9d96e4, 0x9cf4d8), // Tunisia
        (0x9b12ec, 0x9cf4dc), // Turkey
        (0x9d96d4, 0x9cf4e0), // Turkmenistan
        (0x9d96bc, 0x9cf4e4), // United Arab Emirates
        (0x9d96ac, 0x9cf4e8), // Soviet Union
        (0x9d96a4, 0x9cf4ec), // Uganda
        (0x9d969c, 0x9cf4f0), // Ukraine
        (0x9d9694, 0x9cf4f4), // Uruguay
        (0x9d9684, 0x9cf4f8), // United States
        (0x9d9678, 0x9cf4fc), // Uzbekistan
        (0x9d9670, 0x9cf500), // Vanuatu
        (0x9d9664, 0x9cf504), // Venezuela
        (0x9d965c, 0x9cf508), // Vietnam
        (0x9d964c, 0x9cf50c), // West Germany
        (0x9b12c0, 0x9cf510), // Wales
        (0x9d963c, 0x9cf514), // Western Samoa
        (0x9d9634, 0x9cf514), // Samoa
        (0x9d962c, 0x9cf518), // Yemen
        (0x9b12a4, 0x9cf51c), // Yugoslavia
        (0x9d9624, 0x9cf520), // Zaire
        (0x9d961c, 0x9cf524), // Zambia
        (0x9d9610, 0x9cf528), // Zimbabwe
        (0x9d9600, 0x9cf52c), // American Samoa
        (0x9d95f0, 0x9cf530), // New Caledonia
        (0x9d95d8, 0x9cf534), // British Virgin Islands
        (0x9d95bc, 0x9cf538), // Turks and Caicos Islands
        (0x9d95a8, 0x9cf53c), // US Virgin Islands
        (0x9d959c, 0x9cf540), // Anguilla
        (0x9d9590, 0x9cf544), // Montserrat
    ],
];

/// As globais de clube na ordem em que 0x60ba80 as compara (as mesmas 313 de CLUB_KEYS).
static CLUB_KEYS_SWAP_ORDER: &[u32] = &[
    0x9d00a4, // MLS All Stars East
    0x9d00a8, // MLS All Stars West
    0x9d00ac, // River Plate
    0x9d00b0, // Vélez Sarsfield
    0x9d00b4, // Independiente
    0x9d00b8, // Independiente Rivadavia de Mendoza
    0x9d00bc, // Racing de Córdoba
    0x9d00c0, // Villa Mitre de Bahía Blanca
    0x9d00c4, // Argentino de Rosario
    0x9d00c8, // Temperley
    0x9d00cc, // Lanús
    0x9d00d0, // Boca Juniors
    0x9d00d4, // Gimnasia y Esgrima de La Plata
    0x9d00d8, // San Lorenzo de Almagro
    0x9d00dc, // Argentinos Juniors
    0x9d00e0, // Rosario Central
    0x9d00e4, // Newell's Old Boys
    0x9d00e8, // Racing Club
    0x9d00ec, // Colón de Santa Fé
    0x9d00f0, // Platense
    0x9d00f4, // Ferro Carril Oeste
    0x9d00f8, // Estudiantes de La Plata
    0x9d00fc, // Gimnasia y Esgrima de Jujuy
    0x9d0100, // Unión de Santa Fé
    0x9d0104, // Huracán
    0x9d0108, // Talleres de Córdoba
    0x9d010c, // Belgrano de Córdoba
    0x9d0110, // Arsenal
    0x9d0114, // Crystal Palace
    0x9d0118, // Glasgow Celtic
    0x9d011c, // Glasgow Rangers
    0x9d0120, // Athletic Club de Bilbao
    0x9d0124, // C.D. Alavés
    0x9d0128, // Real Sociedad C.F.
    0x9d012c, // Atlético de Madrid
    0x9d0130, // Atlético de Madrid B
    0x9d0134, // Athletic Club de Bilbao B
    0x9d0138, // F.C. Barcelona
    0x9d013c, // F.C. Barcelona B
    0x9d0140, // Real Valladolid
    0x9d0144, // Real Valladolid B
    0x9d0148, // Málaga C.F.
    0x9d014c, // Málaga C.F. B
    0x9d0150, // Real Betis Balompié
    0x9d0154, // Real Betis Balompié B
    0x9d0158, // Real Club Celta de Vigo
    0x9d015c, // Real Club Celta de Vigo B
    0x9d0160, // Deportivo de La Coruña
    0x9d0164, // Deportivo de La Coruña B
    0x9d0168, // R.C.D. Espanyol
    0x9d016c, // R.C.D. Espanyol B
    0x9d0170, // C.D. Logroñés
    0x9d0174, // C.D. Logroñés B
    0x9d0178, // R.C.D. Mallorca
    0x9d017c, // R.C.D. Mallorca B
    0x9d0180, // Club Atlético Osasuna
    0x9d0184, // Club Atlético Osasuna B
    0x9d0188, // Real Oviedo C.F.
    0x9d018c, // Real Oviedo C.F. B
    0x9d0190, // Real Madrid C.F.
    0x9d0194, // Real Madrid C.F. B
    0x9d0198, // Sevilla C.F.
    0x9d019c, // Sevilla C.F. B
    0x9d01a0, // Real Sociedad C.F. B
    0x9d01a4, // Instituto de Córdoba
    0x9d01a8, // San Martín de San Juan
    0x9d01ac, // Gimnasia y Tiro de Salta
    0x9d01b0, // Atlético Rafaela
    0x9d01b4, // San Martín de Tucumán
    0x9d01b8, // Almirante Brown de Arrecifes
    0x9d01bc, // Atlético Tucumán
    0x9d01c0, // Olimpo de Bahía Blanca
    0x9d01c4, // Huracán de Corrientes
    0x9d01c8, // Aldosivi
    0x9d01cc, // Cipolletti de Río Negro
    0x9d01d0, // San Martín de Mendoza
    0x9d01d4, // Douglas Haig
    0x9d01d8, // Gimnasia y Esgrima de Concepción del Uruguay
    0x9d01dc, // Juventud Antoniana de Salta
    0x9d01e0, // Godoy Cruz de Mendoza
    0x9d01e4, // Banfield
    0x9d01e8, // Los Andes
    0x9d01ec, // Quilmes
    0x9d01f0, // Central Córdoba de Rosario
    0x9d01f4, // Chacarita Juniors
    0x9d01f8, // All Boys
    0x9d01fc, // San Miguel
    0x9d0200, // Nueva Chicago
    0x9d0204, // Defensa y Justicia
    0x9d0208, // Arsenal de Sarandí
    0x9d020c, // Huracán de Tres Arroyos
    0x9d0210, // Defensores de Belgrano
    0x9d0214, // Deportivo Morón
    0x9d0218, // Estudiantes de Buenos Aires
    0x9d021c, // Almagro
    0x9d0220, // Atlanta
    0x9d0224, // Deportivo Español
    0x9d0228, // El Porvenir
    0x9d022c, // Tigre
    0x9d0230, // Middlesbrough
    0x9d0234, // Newcastle United
    0x9d0238, // Sunderland
    0x9d023c, // Aston Villa
    0x9d0240, // Manchester United
    0x9d0244, // Liverpool
    0x9d0248, // Everton
    0x9d024c, // Tottenham Hotspur
    0x9d0250, // Chelsea
    0x9d0254, // Valencia C.F.
    0x9d0258, // Valencia C.F. B
    0x9d025c, // Brighton and Hove Albion
    0x9d0260, // Southampton
    0x9d0264, // AS Monaco
    0x9d0268, // FC Metz
    0x9d026c, // Cruzeiro Esporte Clube
    0x9d0270, // São Paulo Futebol Clube
    0x9d0274, // Sociedade Esportiva Palmeiras
    0x9d0278, // Sport Club Corinthians Paulista
    0x9d027c, // Gremio Foot Ball Porto Alegre
    0x9d0354, // Clube de Regatas Vasco da Gama
    0x9d0280, // Club Social y Deportivo Colo Colo
    0x9d0284, // Univ.de Chile
    0x9d0288, // Univ.Catolica
    0x9d028c, // Club Nacional de Fútbol
    0x9d0290, // Olimpia
    0x9d0294, // Cerro Porteño
    0x9d0298, // Club Atlético Peñarol
    0x9d029c, // Elgin City
    0x9d02a0, // Peterhead
    0xb63cdc, // 1.FC Kaiserslautern
    0xb63c9c, // 1.FC Kaiserslautern Amateure
    0xb63cbc, // Bayer 04 Leverkusen
    0xb63ca0, // Bayer Leverkusen Amateure
    0xb63ce8, // Borussia Dortmund
    0xb63cac, // Borussia Dortmund Amateure
    0xb63cd8, // FC Bayern München
    0xb63cd0, // Bayern München Amateure
    0xb63cb4, // FC St. Pauli
    0xb63cb0, // FC St. Pauli Amateure
    0xb63cd4, // Hamburger SV
    0xb63ca8, // Hamburger SV Amateure
    0xb63ce4, // Hertha BSC Berlin
    0xb63cec, // Hertha BSC Amateure
    0xb63cf0, // Karlsruher SC
    0xb63cf8, // Karlsruher SC Amateure
    0xb63cc0, // SV Werder Bremen
    0xb63d00, // SV Werder Bremen Amateure
    0xb63cc8, // TSV München 1860
    0xb63ccc, // TSV München 1860 Amateure
    0xb63ce0, // Tennis Borussia Berlin
    0xb63cb8, // Tennis Borussia Amateure
    0xb63cc4, // VfB Stuttgart
    0xb63cf4, // VfB Stuttgart Amateure
    0xb63cfc, // VfL Bochum
    0xb63ca4, // VfL Bochum Amateure
    0x9d02a4, // Eintracht Frankfurt
    0x9d02a8, // Eintracht Frankfurt Amateure
    0x9d02ac, // FC Hansa Rostock
    0x9d02b0, // FC Hansa Rostock Amateure
    0x9d02b4, // FC Schalke 04
    0x9d02b8, // FC Schalke 04 Amateure
    0x9d02bc, // SG Wattenscheid 09
    0x9d02c0, // SG Wattenscheid 09 Amateure
    0x9d02c4, // Sport Lisboa e Benfica
    0x9d02c8, // Sport Lisboa e Benfica B
    0x9d02cc, // Sporting Clube de Braga
    0x9d02d0, // Sporting Clube de Braga B
    0x9d02d4, // Clube Sport Marítimo
    0x9d02d8, // Clube Sport Marítimo B
    0x9d02dc, // Futebol Clube do Porto
    0x9d02e0, // Futebol Clube do Porto B
    0x9d02e4, // S.D. Éibar
    0x9d02e8, // Amurrio C.F.
    0x9d02ec, // C.D. Aurrerá Vitoria
    0x9d02f0, // Barakaldo C.F.
    0x9d02f4, // S.D. Beasaín
    0x9d02f8, // Bermeo Club
    0x9d02fc, // Gernika Club
    0x9d0304, // S.D. Lemona
    0x9d0308, // Elgoibar C.D.
    0x9d030c, // Hernani C.D.
    0x9d0310, // Zalla U.C.
    0x9d0314, // Cultural Durango
    0x9d0318, // S.D. Amorebieta
    0x9d031c, // Arenas Getxo Bilbao
    0x9d0320, // Aurrerá Ondarroa C.D.
    0x9d0324, // Baskonia C.D.
    0x9d0328, // S.D. Éibar B
    0x9d032c, // U.D. San Pedro
    0x9d0330, // Santurtzi C.D.
    0x9d0334, // Sestao River Club
    0x9d0338, // Tolosa C.F.
    0x9d033c, // Coritiba Football Club
    0x9d0340, // Santos Futebol Clube
    0x9d0344, // Sport Club do Recife
    0x9d0348, // Associação Portugesa de Desportos
    0x9d0350, // Clube Atlético Mineiro
    0x9d0358, // Clube de Regatas Flamengo
    0x9d035c, // Sport Club Internacional
    0x9d0360, // Esporte Clube Vitória
    0x9d0364, // Botafogo Futebol Clube
    0x9d0368, // Clube Atlético Paranaense
    0x9d036c, // Associação Atlética Ponte Preta
    0x9d0370, // Esporte Clube Juventude
    0x9d0374, // Guarani Futebol Clube
    0x9d0378, // Paraná Clube
    0x9d037c, // Chicago Fire
    0x9d0380, // Miami Fusion FC
    0x9d0384, // Washington DC United
    0x9d0388, // Tampa Bay Mutiny
    0x9d038c, // San Jose Earthquakes
    0x9d0390, // New England Revolution
    0x9d0394, // NY/NJ Metrostars
    0x9d0398, // Los Angeles Galaxy
    0x9d039c, // Kansas City Wizards
    0x9d03a0, // Dallas Burn
    0x9d03a4, // Columbus Crew
    0x9d03a8, // Colorado Rapids
    0x9d03ac, // Albacete Balompié
    0x9d03b0, // Albacete Balompié B
    0x9d03b4, // C.D. Badajoz
    0x9d03b8, // C.D. Badajoz B
    0x9d03bc, // C.D. Leganés
    0x9d03c0, // C.D. Leganés B
    0x9d03c4, // C.D. Tenerife
    0x9d03c8, // C.D. Tenerife B
    0x9d03cc, // C.P. Mérida
    0x9d03d0, // C.P. Mérida B
    0x9d03d4, // Deportivo Alavés B
    0x9d03d8, // Extremadura C.F.
    0x9d03dc, // Extremadura C.F. B
    0x9d03e0, // Racing Club de Santander
    0x9d03e4, // Racing Club de Santander B
    0x9d03e8, // Rayo Vallecano de Madrid
    0x9d03ec, // Rayo Vallecano de Madrid B
    0x9d03f0, // Real Murcia C.F.
    0x9d03f4, // Real Murcia C.F. B
    0x9d03f8, // Real Sporting de Gijón
    0x9d03fc, // Real Sporting de Gijón B
    0x9d0400, // Real Zaragoza
    0x9d0404, // Real Zaragoza B
    0x9d0408, // S.D. Compostela
    0x9d040c, // S.D. Compostela B
    0x9d0410, // U.D. Las Palmas
    0x9d0414, // U.D. Las Palmas B
    0x9d0418, // U.D. Salamanca
    0x9d041c, // U.D. Salamanca B
    0x9d0420, // Galatasaray
    0x9d0424, // BJK Besiktas
    0x9d0428, // Fenerbahçe
    0x9d042c, // General Paz Juniors de Córdoba
    0x9d0430, // Swansea City
    0x9d0434, // Cardiff City
    0x9d0438, // Merthyr Tydfil
    0x9d043c, // Wrexham
    0x9d0440, // Sporting Clube De Portugal
    0x9d0444, // Sporting Clube De Portugal B
    0x9d0448, // Arminia Bielefeld
    0x9d044c, // Arminia Bielefeld Amateure
    0x9d0450, // FC Energie Cottbus
    0x9d0454, // FC Energie Cottbus Amateure
    0x9d0458, // MSV Duisburg
    0x9d045c, // MSV Duisburg Amateure
    0x9d0460, // Hannover 96
    0x9d0464, // Hannover 96 Amateure
    0x9d0468, // 1.FC Köln
    0x9d046c, // 1.FC Köln Amateure
    0x9d0470, // Borussia M'gladbach
    0x9d0474, // Bor. M'gladbach Amateure
    0x9d0478, // 1.FC Nürnberg
    0x9d047c, // 1.FC Nürnberg Amateure
    0x9d0480, // Rot-Weiss Oberhausen
    0x9d0484, // RW Oberhausen Amateure
    0x9d0488, // SC Freiburg
    0x9d048c, // SC Freiburg Amateure
    0x9d0490, // 1.FC Saarbrücken
    0x9d0494, // 1.FC Saarbrücken Amateure
    0x9d0498, // SSV Ulm 1846
    0x9d049c, // SSV Ulm 1846 Amateure
    0x9d04a0, // SpVgg Unterhaching
    0x9d04a4, // SpVgg Unterhaching Amateure
    0x9d04a8, // VfB Leipzig
    0x9d04ac, // VfB Leipzig Amateure
    0x9d04b0, // VfL Wolfsburg
    0x9d04b4, // VfL Wolfsburg Amateure
    0x9d04b8, // C.D. Castellón
    0x9d04bc, // C.D. Castellón B
    0x9d04c0, // C.D. Badajoz
    0x9d04c4, // C.D. Badajoz B
    0x9d04c8, // C.D. Logroñés
    0x9d04cc, // C.D. Logroñés B
    0x9d04d0, // C.D. Ourense
    0x9d04d4, // C.D. Ourense B
    0x9d04d8, // Córdoba C.F.
    0x9d04dc, // Córdoba C.F. B
    0x9d04e0, // Cultural Leonesa
    0x9d04e4, // Cultural Leonesa B
    0x9d04e8, // Elche C.F.
    0x9d04ec, // Elche C.F. B
    0x9d04f0, // Levante U.D.
    0x9d04f4, // Levante U.D. B
    0x9d04f8, // Real Unión de Irún
    0x9d04fc, // Real Unión de Irún B
    0x9d0500, // Recreativo de Huelva
    0x9d0504, // Recreativo de Huelva B
    0x9d0508, // Fortuna Düsseldorf
    0x9d050c, // Fortuna Düsseldorf ii
    0x9d0510, // sg hoechst
    0x9d0514, // sg hoechst ii
    0x9d0518, // fsv mainz 05
    0x9d051c, // fsv mainz 05 ii
    0x9d0520, // stuttgarter kickers
    0x9d0524, // stuttgarter kickers ii
];

/// { 0x6146b0 } (cdecl, index.cpp, match_club_key_id): casa o nome de clube da base com as chaves fixas
/// do jogo (CLUB_KEYS): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
unsafe extern "C" fn match_club_key_id(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    let mut r = 0i32;
    for cadeia in CLUB_KEYS.iter() {
        for &(texto, global) in cadeia.iter() {
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de clube
                break;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de clube)
}

/// { 0x60ba80 } (cdecl, index.cpp, remap_club_key_id): depois do corte da tabela de clubes pelo
/// tamanho da base (0x5267c0), a primeira chave de clube (na ordem de CLUB_KEYS_SWAP_ORDER) que
/// guarda o id `antigo` passa a guardar `novo` e devolve 1; nenhuma: 0.
unsafe extern "C" fn remap_club_key_id(antigo: u32, novo: u32) -> u32 {
    for &g in CLUB_KEYS_SWAP_ORDER.iter() {
        if *(g as *const u32) == antigo {
            *(g as *mut u32) = novo; // chave remapeada
            return 1;
        }
    }
    0
}

/// { 0x60d810 } (cdecl, index.cpp, match_nation_key_id): casa o nome de nação da base com as chaves fixas
/// do jogo (NATION_KEYS): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
unsafe extern "C" fn match_nation_key_id(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    let mut r = 0i32;
    for cadeia in NATION_KEYS.iter() {
        for &(texto, global) in cadeia.iter() {
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de nação
                break;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de nação)
}

/// As globais (dword) que ficaram diferentes entre a original e a cópia: "[end] original x rust".
fn describe_key_diffs(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> String {
    let mut s = String::new();
    for ((a, x), (_, y)) in mo.0.iter().zip(mr.0.iter()) {
        for i in (0..x.len()).step_by(4) {
            if x[i..i + 4] != y[i..i + 4] {
                let v = |b: &[u8]| i32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
                s += &format!("[{:x}] {} x {} ", a + i, v(x), v(y));
            }
        }
    }
    s
}

unsafe extern "C" fn comparar_match_club_key_id(nome: usize, linha: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(%%I0%%);
    let reg: [(usize, usize); 2] = [(0x9d00a4, 0x484), (0xb63c9c, 0x68)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(nome, linha), || match_club_key_id(nome, linha));
    SCOREBOARD.record(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("match_club_key_id(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, describe_key_diffs(&mo, &mr))
    });
    r
}

unsafe extern "C" fn comparar_remap_club_key_id(antigo: u32, novo: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(%%I1%%);
    let reg: [(usize, usize); 2] = [(0x9d00a4, 0x484), (0xb63c9c, 0x68)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(antigo, novo), || remap_club_key_id(antigo, novo));
    SCOREBOARD.record(igual && r == n, || format!("remap_club_key_id({}, {}) original={} rust={} {}", antigo as i32, novo as i32, r, n, describe_key_diffs(&mo, &mr)));
    r
}

unsafe extern "C" fn comparar_match_nation_key_id(nome: usize, linha: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(%%I2%%);
    let reg: [(usize, usize); 1] = [(0x9cf200, 0x348)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(nome, linha), || match_nation_key_id(nome, linha));
    SCOREBOARD.record(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("match_nation_key_id(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, describe_key_diffs(&mo, &mr))
    });
    r
}
