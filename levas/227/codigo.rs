// ---------- 227a leva: as chaves fixas de clube e de nação pelo nome e a troca da chave de clube depois do corte da base (index.cpp) ----------

/// As chaves de clube do index.cpp (0x6146b0), na ordem do código: (texto do nome, global que recebe a linha), em 4 cadeias. Vários nomes podem dar na mesma global (grafias).
static CHAVES_DE_CLUBE: &[&[(u32, u32)]] = &[
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
static CHAVES_DE_NACAO: &[&[(u32, u32)]] = &[
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

/// As globais de clube na ordem em que 0x60ba80 as compara (as mesmas 313 de CHAVES_DE_CLUBE).
static CHAVES_DE_CLUBE_NA_TROCA: &[u32] = &[
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
/// do jogo (CHAVES_DE_CLUBE): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
unsafe extern "C" fn casar_chave_de_clube(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_CLUBE.iter() {
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
/// tamanho da base (0x5267c0), a primeira chave de clube (na ordem de CHAVES_DE_CLUBE_NA_TROCA) que
/// guarda o id `antigo` passa a guardar `novo` e devolve 1; nenhuma: 0.
unsafe extern "C" fn remapear_chave_de_clube(antigo: u32, novo: u32) -> u32 {
    for &g in CHAVES_DE_CLUBE_NA_TROCA.iter() {
        if *(g as *const u32) == antigo {
            *(g as *mut u32) = novo; // chave remapeada
            return 1;
        }
    }
    0
}

/// { 0x60d810 } (cdecl, index.cpp, match_nation_key_id): casa o nome de nação da base com as chaves fixas
/// do jogo (CHAVES_DE_NACAO): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
unsafe extern "C" fn casar_chave_de_nacao(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_NACAO.iter() {
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
fn chaves_diferentes(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> String {
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

unsafe extern "C" fn comparar_casar_chave_de_clube(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(620);
    let reg: [(usize, usize); 2] = [(0x9d00a4, 0x484), (0xb63c9c, 0x68)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_clube(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_clube(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_diferentes(&mo, &mr))
    });
    r
}

unsafe extern "C" fn comparar_remapear_chave_de_clube(antigo: u32, novo: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, u32) -> u32>(621);
    let reg: [(usize, usize); 2] = [(0x9d00a4, 0x484), (0xb63c9c, 0x68)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(antigo, novo), || remapear_chave_de_clube(antigo, novo));
    PLACAR.registrar(igual && r == n, || format!("remapear_chave_de_clube({}, {}) original={} rust={} {}", antigo as i32, novo as i32, r, n, chaves_diferentes(&mo, &mr)));
    r
}

unsafe extern "C" fn comparar_casar_chave_de_nacao(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(622);
    let reg: [(usize, usize); 1] = [(0x9cf200, 0x348)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_nacao(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_nacao(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_diferentes(&mo, &mr))
    });
    r
}

// ---------- 227a leva: as chaves fixas das competições de clube e das competições e prêmios de pessoa pelo nome (index.cpp) ----------

/// As chaves de competição de clube do index.cpp (0x60efd0), na ordem do código: (texto do nome longo, global que recebe a linha), em 3 cadeias; (0, global) = mais uma global gravada no mesmo acerto.
static CHAVES_DE_COMPETICAO_DE_CLUBE: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9dc200, 0x9cf548), // French Lower Division
        (0x9dc1e8, 0x9cf54c), // German First Division
        (0x9dc1d0, 0x9cf550), // German Second Division
        (0x9dc1a8, 0x9cf554), // German Regional Division West/Southwest
        (0x9dc188, 0x9cf558), // German Regional Division East
        (0x9dc168, 0x9cf55c), // German Regional Division North
        (0x9dc148, 0x9cf560), // German Regional Division South
        (0x9dc138, 0x9cf564), // German Regional
        (0x9dc11c, 0x9cf568), // Dutch KPN-Telecompetition
        (0x9dc104, 0x9cf568), // Dutch KPN Eredivisie
        (0x9dc0ec, 0x9cf568), // Dutch Premier Division
        (0x9dc0d8, 0x9cf56c), // Dutch Toto Division
        (0x9dc0c0, 0x9cf56c), // Dutch First Division
        (0x9dc0b0, 0x9cf570), // Italian Serie A
        (0x9dc0a0, 0x9cf574), // Italian Serie B
        (0x9dc08c, 0x9cf578), // Italian Serie C1/A
        (0x9dc078, 0x9cf57c), // Italian Serie C1/B
        (0x9dc064, 0x9cf580), // Italian Serie C2/A
        (0x9dc050, 0x9cf584), // Italian Serie C2/B
        (0x9dc03c, 0x9cf588), // Italian Serie C2/C
        (0x9dc02c, 0x9cf58c), // Italian Serie D
        (0x9dc014, 0x9cf590), // American Major League
        (0x9dc000, 0x9cf594), // American A-League
        (0x9dbff0, 0x9cf598), // American USISL
        (0x9dbfd8, 0x9cf598), // American D3-Pro League
        (0x9dbfc0, 0x9cf59c), // Belgian First Division
        (0x9dbfa8, 0x9cf5a0), // Belgian Second Division
        (0x9dbf8c, 0x9cf5a4), // Belgian Third Division A
        (0x9dbf70, 0x9cf5a8), // Belgian Third Division B
        (0x9dbf58, 0x9cf5ac), // Danish Premier Division
        (0x9dbf40, 0x9cf5b0), // Danish First Division
        (0x9dbf28, 0x9cf5b4), // Danish Second Division
        (0x9dbf0c, 0x9cf5b8), // Danish Kvalifikationsrækken
        (0x9dbef0, 0x9cf5bc), // English Premier Division
        (0x9dbed8, 0x9cf5c0), // English First Division
        (0x9dbec0, 0x9cf5c4), // English Second Division
        (0x9dbea8, 0x9cf5c8), // English Third Division
        (0x9dbe90, 0x9cf5cc), // French First Division
        (0x9dbe78, 0x9cf5d0), // French Second Division
        (0x9dbe68, 0x9cf5d4), // French National
        (0x9dbe5c, 0x9cf5d8), // French CFA
        (0x9dbe4c, 0x9cf5d8), // French CFA - A
        (0x9dbe30, 0x9cf5dc), // Scottish Premier Division
        (0x9dbe18, 0x9cf5e0), // Scottish First Division
        (0x9dbdfc, 0x9cf5e4), // Scottish Second Division
        (0x9dbde4, 0x9cf5e8), // Scottish Third Division
        (0x9dbdc8, 0x9cf5ec), // Swedish Premier Division
        (0x9dbdb0, 0x9cf5f0), // Swedish First Division
        (0x9dbd90, 0x9cf5f0), // Swedish First Division North
        (0x9dbd68, 0x9cf5f4), // Swedish Second Division South Gotaland
        (0x9dbd48, 0x9cf5f8), // Swedish Second Division North
        (0x9dbd20, 0x9cf5fc), // Swedish Second Division East Svealand
        (0x9dbcf8, 0x9cf600), // Swedish Second Division East Gotaland
        (0x9dbcd0, 0x9cf604), // Swedish Second Division West Svealand
        (0x9dbca8, 0x9cf608), // Swedish Second Division West Gotaland
        (0x9dbc8c, 0x9cf60c), // Portuguese Premier Division
        (0x9dbc70, 0x9cf60c), // Portuguese Premier League
        (0x9dbc54, 0x9cf610), // Portuguese Second Division
        (0x9dbc38, 0x9cf610), // Portuguese Second League
        (0x9dbc14, 0x9cf614), // Portuguese Second Division B North
        (0x9dbbec, 0x9cf618), // Portuguese Second Division B Central
        (0x9dbbc8, 0x9cf61c), // Portuguese Second Division B South
        (0x9dbbac, 0x9cf620), // Portuguese Third Division
        (0x9dbb94, 0x9cf624), // Spanish First Division
        (0x9dbb7c, 0x9cf628), // Spanish Second Division
        (0x9dbb60, 0x9cf62c), // Spanish Second Division B1
        (0x9dbb44, 0x9cf630), // Spanish Second Division B2
        (0x9dbb28, 0x9cf634), // Spanish Second Division B3
        (0x9dbb0c, 0x9cf638), // Spanish Second Division B4
        (0x9dbae8, 0x9cf63c), // South American Copa Libertadores
        (0x9dbacc, 0x9cf640), // South American Super Cup
        (0x9dbab0, 0x9cf644), // South American CONMEBOL Cup
        (0x9dba98, 0x9cf648), // South American Recopa
        (0x9dba84, 0x9cf64c), // Asian Champions Cup
        (0x9dba6c, 0x9cf64c), // Asian Club Championship
        (0x9dba54, 0x9cf650), // Asian Cup Winners Cup
        (0x9dba3c, 0x9cf650), // Asian Cup Winners' Cup
        (0x9dba20, 0x9cf654), // Argentine Premier Division
        (0x9dba04, 0x9cf658), // Argentine Second Division
        (0x9db9f0, 0x9cf65c), // Japanese J-League 1
        (0x9db9dc, 0x9cf65c), // Japanese League 1
        (0x9db9c4, 0x9cf660), // Japanese Emperor's Cup
        (0x9db9ac, 0x9cf660), // Japanese Rising Sun Cup
        (0x9db98c, 0x9cf664), // Japanese Yamazaki Nabisco Cup
        (0x9db97c, 0x9cf664), // Japanese Cup
        (0x9db96c, 0x9cf664), // Japanese  Cup
        (0x9db964, 0x9cf664), // J Cup
        (0x9db958, 0x9cf668), // Friendly
        (0x9db93c, 0x9cf66c), // Japanese Xerox Super Cup
        (0x9db928, 0x9cf66c), // Japanese Super Cup
        (0x9db910, 0x9cf66c), // Japanese Winners Cup
        (0x9db8f4, 0x9cf670), // Japanese Football League
        (0x9db8e0, 0x9cf670), // Japanese Non-League
        (0x9db8c4, 0x9cf674), // Japanese University League
        (0x9db8a0, 0x9cf678), // Japanese Administrative Division
        (0x9db884, 0x9cf67c), // Argentine Metropolitan Zone
        (0x9db86c, 0x9cf680), // Argentine Interior Zone
        (0x9b5b40, 0x9cf684), // Reserve
        (0x9db850, 0x9cf688), // Argentine Third Division
        (0x9db838, 0x9cf68c), // CONCACAF Merconorte Cup
        (0x9db828, 0x9cf68c), // Merconorte Cup
        (0x9db810, 0x9cf690), // American MLS All-Stars
        (0x9db7fc, 0x9cf694), // German League Cup
        (0x9db7dc, 0x9cf698), // South American Copa Mercosur
        (0x9db7c8, 0x9cf69c), // English Conference
        (0x9db7b4, 0x9cf6a0), // English FA Trophy
        (0x9db79c, 0x9cf6a4), // English Conference Cup
        (0x9db784, 0x9cf6a8), // French Champions Trophy
        (0x9db76c, 0x9cf6ac), // Swedish Lower Division
        (0x9db754, 0x9cf6b0), // Spanish Lower Division
        (0x9db73c, 0x9cf6b4), // Portuguese Super Cup
        (0x9db728, 0x9cf6b8), // A Premier Division
        (0x9db70c, 0x9cf6bc), // Norwegian Premier Division
        (0x9db6f0, 0x9cf6c0), // Norwegian First Division
        (0x9db6cc, 0x9cf6c4), // Norwegian Second Division Group 1
        (0x9db6a8, 0x9cf6c8), // Norwegian Second Division Group 2
        (0x9db684, 0x9cf6cc), // Norwegian Second Division Group 3
        (0x9db660, 0x9cf6d0), // Norwegian Second Division Group 4
        (0x9db63c, 0x9cf6d4), // Norwegian Second Division Group 5
        (0x9db618, 0x9cf6d8), // Norwegian Second Division Group 6
        (0x9db5f4, 0x9cf6dc), // Norwegian Second Division Group 7
        (0x9db5d0, 0x9cf6e0), // Norwegian Second Division Group 8
    ],
    // cadeia 2
    &[
        (0x9db5c0, 0x9cf6e4), // World Club Cup
        (0x9db5a8, 0x9cf6e4), // Intercontinental Cup
        (0x9db588, 0x9cf6e8), // UEFA European Champions League
        (0x9db570, 0x9cf6e8), // UEFA Champions League
        (0x9db558, 0x9cf6e8), // European Champions Cup
        (0x9db53c, 0x9cf6ec), // European Cup Winners Cup
        (0x9db530, 0x9cf6f0), // UEFA Cup
        (0x9db51c, 0x9cf6f4), // European Super Cup
        (0x9db50c, 0x9cf6f8), // Inter-toto Cup
        (0x9db4f8, 0x9cf6fc), // Belgian Super Cup
        (0x9db4ec, 0x9cf700), // Belgian Cup
        (0x9db4d8, 0x9cf704), // Belgian League Cup
        (0x9db4cc, 0x9cf708), // Danish Cup
        (0x9db4c0, 0x9cf708), // Dong Cup
        (0x9db4b4, 0x9cf70c), // French Cup
        (0x9db4a0, 0x9cf710), // French League Cup
        (0x9db494, 0x9cf714), // German Cup
        (0x9db480, 0x9cf718), // Dutch Amstel Cup
        (0x9db474, 0x9cf718), // Dutch Cup
        (0x9db468, 0x9cf71c), // Italian Cup
        (0x9db454, 0x9cf71c), // Italian T.I.M. Cup
        (0x9db440, 0x9cf720), // Italian Serie C Cup
        (0x9db42c, 0x9cf724), // Italian Super Cup
        (0x9db420, 0x9cf728), // US Open Cup
        (0x9db408, 0x9cf72c), // CONCACAF Champions Cup
        (0x9db3ec, 0x9cf730), // CONCACAF Cup Winners Cup
        (0x9db3dc, 0x9cf734), // Norwegian Cup
        (0x9db3c0, 0x9cf738), // Norwegian Third Division
        (0x9db3b0, 0x9cf73c), // Portuguese Cup
        (0x9db3a4, 0x9cf740), // Spanish Cup
        (0x9db390, 0x9cf744), // Spanish Super Cup
        (0x9db384, 0x9cf748), // Swedish Cup
        (0x9db374, 0x9cf74c), // English FA Cup
        (0x9db360, 0x9cf750), // English League Cup
        (0x9db348, 0x9cf754), // English Charity Shield
        (0x9db32c, 0x9cf758), // English Autoglass Trophy
        (0x9db318, 0x9cf758), // English Vans Trophy
        (0x9db308, 0x9cf75c), // Scottish Cup
        (0x9db2f4, 0x9cf760), // Scottish League Cup
        (0x9db2e0, 0x9cf764), // A Lower Division
        (0x9db2cc, 0x9cf768), // Japanese J-League 2
        (0x9db2b8, 0x9cf768), // Japanese League 2
        (0x9db290, 0x9cf76c), // World Cup Oceania Qualifying Section
        (0x9db268, 0x9cf770), // World Cup CONCACAF Qualifying Section
        (0x9db244, 0x9cf774), // World Cup Asian Qualifying Section
        (0x9db218, 0x9cf778), // World Cup South American Qualifying Section
        (0x9db1f0, 0x9cf77c), // World Cup African Qualifying Section
        (0x9db1c8, 0x9cf780), // World Cup European Qualifying Section
        (0x9db1a4, 0x9cf784), // European Championship Qualifying
        (0x9db18c, 0x9cf788), // Asian Cup Qualifying
        (0x9db164, 0x9cf78c), // Football Tournament of the Asian Games
        (0x9db144, 0x9cf78c), // Asian Games Football Tournament
        (0x9db138, 0x9cf790), // Asian Cup
        (0x9db124, 0x9cf794), // 'B' International
        (0x9db10c, 0x9cf798), // Under 21 International
        (0x9db0fc, 0x9cf79c), // FIFA World Cup
        (0x9db0d0, 0x9cf7a0), // Football Tournament of the Olympic Games
        (0x9db0c0, 0x9cf7a0), // Olympic Games
        (0x9db0a0, 0x9cf7a4), // European Football Championship
        (0x9db090, 0x9cf7a8), // Copa America
        (0x9db078, 0x9cf7ac), // African Cup of Nations
        (0x9db068, 0x9cf7b0), // Dutch Super Cup
        (0x9db04c, 0x9cf7b0), // Dutch Johan Cruyff Schaal
        (0x9db030, 0x9cf7b0), // Dutch Johan Cruijff Schaal
        (0x9db010, 0x9cf7b4), // Scottish League Challenge Cup
        (0x9daff4, 0x9cf7b8), // Spanish Second Division B
        (0x9dafd4, 0x9cf7bc), // FIFA Club World Championship
        (0x9dafb4, 0x9cf7c0), // Portuguese Second Division B
        (0x9dbc14, 0x9cf7c0), // Portuguese Second Division B North
        (0x9daf9c, 0x9cf7c4), // Belgian Third Division
        (0x9dbf8c, 0x9cf7c4), // Belgian Third Division A
        (0x9daf84, 0x9cf7c8), // Swedish Second Division
        (0x9dbd48, 0x9cf7c8), // Swedish Second Division North
        (0x9daf68, 0x9cf7cc), // Oceania Club Championship
        (0x9daf50, 0x9cf7cc), // Oceania Champions Cup
        (0x9daf40, 0x9cf7d0), // Asian Super Cup
        (0x9daf24, 0x9cf7d4), // Belgian Fourth Division  A
        (0x9daf08, 0x9cf7d8), // Belgian Fourth Division  B
        (0x9daeec, 0x9cf7dc), // Belgian Fourth Division  C
        (0x9daed0, 0x9cf7e0), // Belgian Fourth Division  D
        (0x9daec4, 0x9cf7e4), // Finnish Cup
        (0x9daea8, 0x9cf7e8), // Finnish Premier Division
        (0x9dae88, 0x9cf7ec), // Finnish First Division North
        (0x9dae68, 0x9cf7f0), // Finnish First Division South
        (0x9dae50, 0x9cf7f4), // Finnish Lower Division
        (0x9dae38, 0x9cf7f8), // Finnish First Division
        (0x9dae18, 0x9cf7fc), // Finnish Second Division North
        (0x9dadf8, 0x9cf800), // Finnish Second Division West
        (0x9dadd8, 0x9cf804), // Finnish Second Division South
        (0x9dadb8, 0x9cf808), // Finnish Second Division East
        (0x9dada0, 0x9cf80c), // Irish Premier Division
        (0x9dad88, 0x9cf810), // Irish First Division
        (0x9dad78, 0x9cf814), // Irish Group A
        (0x9dad68, 0x9cf818), // Irish Group B
        (0x9dad58, 0x9cf81c), // Irish Group C
        (0x9dad48, 0x9cf820), // Irish Group D
        (0x9dad38, 0x9cf824), // Irish Group E
        (0x9dad20, 0x9cf82c), // Northern Irish Group A
        (0x9dad08, 0x9cf830), // Northern Irish Group B
        (0x9dacf0, 0x9cf834), // Northern Irish Group C
        (0x9dacd8, 0x9cf838), // Northern Irish Group D
        (0x9dacc4, 0x9cf83c), // Irish League Cup
        (0x9daca8, 0x9cf840), // Irish Senior Challenge Cup
        (0x9dac98, 0x9cf844), // Irish Super Cup
        (0x9dac78, 0x9cf848), // Irish Connacht Senior League
        (0x9dac5c, 0x9cf84c), // Irish Ulster Senior League
        (0x9dac40, 0x9cf850), // Irish Munster Senior League
        (0x9dac20, 0x9cf854), // Irish Leinster Senior League
        (0x9dabf8, 0x9cf854), // Irish Leinster Senior League Premier
        (0x9dabe0, 0x9cf858), // Polish First Division
        (0x9dabc8, 0x9cf85c), // Polish Second Division
        (0x9dabb4, 0x9cf860), // Polish League Cup
        (0x9daba4, 0x9cf864), // Polish FA Cup
        (0x9dab8c, 0x9cf868), // Polish Lower Division
        (0x9dab74, 0x9cf86c), // greek premier division
        (0x9dab58, 0x9cf86c), // Greek National A Division
        (0x9dab40, 0x9cf870), // greek second division
        (0x9dab24, 0x9cf870), // Greek National B Division
        (0x9dab0c, 0x9cf874), // greek lower division
        (0x9dab00, 0x9cf878), // greek cup
        (0x9daae8, 0x9cf87c), // croatian first division
        (0, 0x9cf8a4), // (também grava, mesmo acerto)
        (0x9daad0, 0x9cf87c), // croatian a1 division
        (0, 0x9cf8a4), // (também grava, mesmo acerto)
        (0x9daab4, 0x9cf880), // croatian second division
        (0x9daa94, 0x9cf884), // croatian third division central
        (0x9daa74, 0x9cf888), // croatian third division north
        (0x9daa54, 0x9cf88c), // croatian third division south
        (0x9daa34, 0x9cf890), // croatian third division east
        (0x9daa14, 0x9cf894), // croatian third division west
        (0x9daa04, 0x9cf898), // croatian cup
        (0x9da9e0, 0x9cf89c), // australian national soccer league
        (0x9da9cc, 0x9cf8a0), // oceania nations cup
        (0x9da9ac, 0x9cf8a8), // croatian second division north
        (0x9da990, 0x9cf8a8), // croatian second division a
        (0x9da978, 0x9cf8a8), // croatian a2/a division
        (0x9da958, 0x9cf8ac), // croatian second division south
        (0x9da93c, 0x9cf8ac), // croatian second division b
        (0x9da924, 0x9cf8ac), // croatian a2/b division
        (0x9da900, 0x9cf8b0), // northern ireland premier division
        (0x9da8e0, 0x9cf8b0), // irish league premier division
        (0x9da8b8, 0x9cf8b0), // northern irish league premier division
        (0x9da898, 0x9cf8b4), // northern ireland first division
        (0x9da87c, 0x9cf8b4), // irish league first division
        (0x9da854, 0x9cf8b4), // northern irish league first division
        (0x9da834, 0x9cf8b8), // northern ireland lower division
        (0x9da818, 0x9cf8b8), // irish league lower division
        (0x9da7f0, 0x9cf8b8), // northern irish league lower division
        (0x9da7d0, 0x9cf8bc), // northern irish charity shield
        (0x9da7bc, 0x9cf8c0), // northern irish cup
        (0x9da7b0, 0x9cf8c0), // irish cup
        (0x9da798, 0x9cf8c4), // northern irish gold cup
        (0x9da77c, 0x9cf8c8), // northern irish league cup
    ],
    // cadeia 3
    &[
        (0x9da764, 0x9cf8cc), // turkish lower division
        (0x9da748, 0x9cf8d8), // turkish premier division
        (0x9da728, 0x9cf8dc), // turkish 2. division category a
        (0x9da708, 0x9cf8e0), // turkish 2. division category b
        (0x9da6e4, 0x9cf8e4), // turkish 2. division category b g1
        (0x9da6c0, 0x9cf8e8), // turkish 2. division category b g2
        (0x9da69c, 0x9cf8ec), // turkish 2. division category b g3
        (0x9da678, 0x9cf8f0), // turkish 2. division category b g4
        (0x9da654, 0x9cf8f4), // turkish 2. division category b g5
        (0x9da644, 0x9cf8f8), // turkish fa cup
        (0x9da618, 0x9cf8fc), // turkish football writers' association cup
        (0x9da5f0, 0x9cf8fc), // turkish sports writers' association cup
        (0x9da5d4, 0x9cf900), // turkish 2. division group 1
        (0x9da5b8, 0x9cf904), // turkish 2. division group 2
        (0x9da59c, 0x9cf908), // turkish 2. division group 3
        (0x9da580, 0x9cf90c), // turkish 2. division group 4
        (0x9da564, 0x9cf910), // turkish 2. division group 5
        (0x9da544, 0x9cf914), // russian second division - west
        (0x9da520, 0x9cf918), // russian second division - central
        (0x9da500, 0x9cf91c), // russian second division - south
        (0x9da4e0, 0x9cf920), // russian second division - volga
        (0x9da4c0, 0x9cf924), // russian second division - ural
        (0x9da4a0, 0x9cf928), // russian second division - east
        (0x9da488, 0x9cf92c), // russian first division
        (0x9da46c, 0x9cf930), // russian premier division
        (0x9da460, 0x9cf934), // russian cup
        (0x9da448, 0x9cf938), // italian c1 super cup
        (0x9da438, 0x9cf93c), // greek super cup
        (0x9da424, 0x9cf940), // croatian super cup
        (0x9da410, 0x9cf944), // polish super cup
        (0x9da3f8, 0x9cf948), // irish presidents cup
        (0x9da3e0, 0x9cf94c), // welsh premier division
        (0x9da3cc, 0x9cf950), // welsh league cup
        (0x9da3c0, 0x9cf954), // welsh cup
        (0x9da3ac, 0x9cf958), // welsh premier cup
        (0x9da398, 0x9cf95c), // inter-american cup
        (0x9da380, 0x9cf960), // welsh lower division
        (0x9da36c, 0x9cf964), // confederations cup
        (0x9da34c, 0x9cf968), // Brazilian Bahia Lower Division
        (0x9da328, 0x9cf96c), // Brazilian Central Lower Division
        (0x9da300, 0x9cf970), // Brazilian Santa Catarina Lower Division
        (0x9da2dc, 0x9cf974), // Brazilian Pernambuco Lower Division
        (0x9da2bc, 0x9cf978), // Brazilian North Lower Division
        (0x9da294, 0x9cf97c), // Brazilian Rio de Janeiro Lower Division
        (0x9da270, 0x9cf980), // Brazilian Northeast Lower Division
        (0x9da250, 0x9cf984), // Brazilian Goiás Lower Division
        (0x9da228, 0x9cf988), // Brazilian Minas Gerais Lower Division
        (0x9da208, 0x9cf98c), // Brazilian Paraná Lower Division
        (0x9da1e8, 0x9cf990), // Brazilian Gaúcho Lower Division
        (0x9da1c4, 0x9cf994), // Brazilian North State Championship
        (0x9da19c, 0x9cf998), // Brazilian Central State Championship
        (0x9da178, 0x9cf99c), // Brazilian São Paulo Lower Division
        (0x9da158, 0x9cf9a0), // Brazilian Central Regional Cup
        (0x9da130, 0x9cf9a4), // Brazilian Northeast State Championship
        (0x9da110, 0x9cf9a8), // Brazilian North Regional Cup
        (0x9da0ec, 0x9cf9ac), // Brazilian National Third Division
        (0x9da0c0, 0x9cf9b0), // Brazilian Santa Catarina State Championship
        (0x9da09c, 0x9cf9b4), // Brazilian Goiás State Championship
        (0x9da084, 0x9cf9b8), // Brazilian Champions Cup
        (0x9da05c, 0x9cf9bc), // Brazilian Pernambuco State Championship
        (0x9da038, 0x9cf9c0), // Brazilian Bahia State Championship
        (0x9da014, 0x9cf9c4), // Brazilian Northeast Regional Cup
        (0x9d9ff0, 0x9cf9c8), // Brazilian Gaúcho State Championship
        (0x9d9fcc, 0x9cf9cc), // Brazilian South-Minas Regional Cup
        (0x9d9fa8, 0x9cf9d0), // Brazilian Paraná State Championship
        (0x9d9f7c, 0x9cf9d4), // Brazilian Minas Gerais State Championship
        (0x9d9f58, 0x9cf9d8), // Brazilian Rio-São Paulo Tournement
        (0x9d9f2c, 0x9cf9dc), // Brazilian Rio de Janeiro State Championship
        (0x9d9f08, 0x9cf9e0), // Brazilian National Second Division
        (0x9d9ee0, 0x9cf9e4), // Brazilian São Paulo State Championship
        (0x9d9ed0, 0x9cf9e8), // Brazilian Cup
        (0x9d9eac, 0x9cf9ec), // Brazilian National First Division
        (0x9d9e90, 0x9cf9f0), // Irish Munster Senior Cup
        (0x9d9e74, 0x9cf9f4), // Irish Leinster Senior Cup
        (0x9d9e60, 0x9cf9f8), // Korean Super Cup
        (0x9d9e4c, 0x9cf9fc), // Korean League Cup
        (0x9ce524, 0x9cfa00), // Korean League
        (0x9d9e3c, 0x9cfa04), // Korean FA Cup
    ],
];

/// As chaves de competição e prêmio de pessoa do index.cpp (0x6115e0), na ordem do código: (texto do nome, global que recebe a linha), em 5 cadeias.
static CHAVES_DE_COMPETICAO_DE_PESSOA: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9e05ec, 0x9cfa20), // FIFA World Player of the Year
        (0x9e05cc, 0x9cfa24), // World Footballer of the Year
        (0x9e05ac, 0x9cfa28), // European Footballer of the Year
        (0x9e0584, 0x9cfa2c), // South American Footballer of the Year
        (0x9e0564, 0x9cfa30), // African Footballer of the Year
        (0x9e0540, 0x9cfa34), // English Players Player of the Year
        (0x9e0514, 0x9cfa38), // English Players Young Player of the Year
        (0x9e04ec, 0x9cfa3c), // English Players Premier Division Select
        (0x9e04c4, 0x9cfa40), // English Players First Division Select
        (0x9e049c, 0x9cfa44), // English Players Second Division Select
        (0x9e0474, 0x9cfa48), // English Players Third Division Select
        (0x9e0444, 0x9cfa4c), // English Premier Division Player of the Month
        (0x9e0418, 0x9cfa50), // English First Division Player of the Month
        (0x9e03ec, 0x9cfa54), // English Second Division Player of the Month
        (0x9e03c0, 0x9cfa58), // English Third Division Player of the Month
        (0x9e038c, 0x9cfa5c), // English Premier Division Young Player of the Month
        (0x9e0358, 0x9cfa60), // English First Division Young Player of the Month
        (0x9e0324, 0x9cfa64), // English Second Division Young Player of the Month
        (0x9e02f0, 0x9cfa68), // English Third Division Young Player of the Month
        (0x9e02c4, 0x9cfa6c), // English Premier Division Team of the Week
        (0x9e029c, 0x9cfa70), // English First Division Team of the Week
        (0x9e0270, 0x9cfa74), // English Second Division Team of the Week
        (0x9e0248, 0x9cfa78), // English Third Division Team of the Week
        (0x9e022c, 0x9cfa7c), // Scottish Player of the Year
        (0x9e0208, 0x9cfa80), // Scottish Young Player of the Year
        (0x9e01d8, 0x9cfa84), // Scottish Premier Division Player of the Month
        (0x9e01ac, 0x9cfa88), // Scottish First Division Player of the Month
        (0x9e017c, 0x9cfa8c), // Scottish Second Division Player of the Month
        (0x9e0150, 0x9cfa90), // Scottish Third Division Player of the Month
        (0x9e0124, 0x9cfa94), // Scottish Premier Division Team of the Week
        (0x9e00f8, 0x9cfa98), // Scottish First Division Team of the Week
        (0x9e00cc, 0x9cfa9c), // Scottish Second Division Team of the Week
        (0x9e00a0, 0x9cfaa0), // Scottish Third Division Team of the Week
        (0x9e008c, 0x9cfaa4), // Belgian Golden Boot
        (0x9e0060, 0x9cfaa8), // Belgian First Division Player of the Year
        (0x9e0030, 0x9cfaac), // Belgian First Division Goalkeeper of the Year
        (0x9e0004, 0x9cfab0), // Belgian Second Division Player of the Year
        (0x9dffe8, 0x9cfab4), // Danish Player of the Year
        (0x9dffbc, 0x9cfab8), // Danish Premier Division Player of the Year
        (0x9dff88, 0x9cfabc), // Danish Premier Division Young Player of the Year
        (0x9dff5c, 0x9cfac0), // Danish First Division Player of the Year
        (0x9dff2c, 0x9cfac4), // Danish First Division Young Player of the Year
        (0x9dfefc, 0x9cfac8), // Danish Kvalifikationsrækken Player of the Year
        (0x9dfed0, 0x9cfacc), // Danish Second Division Player of the Year
        (0x9dfe9c, 0x9cfad0), // Danish Kvalifikationsrækken Young Player of Year
        (0x9dfe70, 0x9cfad4), // Danish Second Division Young Player of Year
        (0x9dfe54, 0x9cfad8), // French Player of the Year
        (0x9dfe30, 0x9cfadc), // French Foreign Player of the Year
        (0x9dfe0c, 0x9cfae0), // French Under-23 Player of the Year
        (0x9dfde4, 0x9cfae4), // French Most Popular Player of the Year
        (0x9dfdb8, 0x9cfae8), // French Most Unpopular Player of the Year
        (0x9dfd84, 0x9cfaec), // French Players First Division Player of the Year
        (0x9dfd58, 0x9cfaf0), // French First Division Player of the Year
        (0x9dfd28, 0x9cfaf4), // French First Division Goalkeeper of the Year
        (0x9dfcf4, 0x9cfaf8), // French Players Second Division Player of the Year
        (0x9dfcc8, 0x9cfafc), // French Second Division Player of the Year
        (0x9dfc98, 0x9cfb00), // French Second Division Goalkeeper of the Year
        (0x9dfc7c, 0x9cfb04), // German Player of the Year
        (0x9dfc50, 0x9cfb08), // German First Division Player of the Month
        (0x9dfc24, 0x9cfb0c), // German Second Division Player of the Month
        (0x9dfc04, 0x9cfb10), // Dutch Goalkeeper of the Year
        (0x9dfbe4, 0x9cfb14), // Dutch Young Player of the Year
        (0x9dfbb8, 0x9cfb18), // Dutch Premier Division Player of the Year
        (0x9dfb90, 0x9cfb1c), // Dutch Premier Division Top Goalscorer
        (0x9dfb6c, 0x9cfb20), // Dutch Premier Division Golden Boot
        (0x9dfb44, 0x9cfb24), // Dutch First Division Player of the Year
        (0x9dfb20, 0x9cfb28), // Dutch First Division Top Goalscorer
        (0x9dfafc, 0x9cfb2c), // Dutch First Division Golden Boot
        (0x9dfad0, 0x9cfb30), // Italian Serie A Young Player of the Year
        (0x9dfaac, 0x9cfb34), // Italian Serie A Player of the Year
        (0x9dfa80, 0x9cfb38), // Italian Serie A Italian Player of the Year
        (0x9dfa54, 0x9cfb3c), // Italian Serie A Foreign Player of the Year
        (0x9dfa2c, 0x9cfb40), // Italian Serie A Defender of the Year
        (0x9dfa04, 0x9cfb44), // Italian Serie A Goalkeeper of the Year
        (0x9df9e0, 0x9cfb48), // Italian Serie B Player of the Year
        (0x9df9b8, 0x9cfb4c), // Italian Serie C1/A Player of the Year
        (0x9df990, 0x9cfb50), // Italian Serie C1/B Player of the Year
        (0x9df970, 0x9cfb54), // Norwegian Player of the Year
        (0x9df94c, 0x9cfb58), // Norwegian Goalkeeper of the Year
        (0x9df92c, 0x9cfb5c), // Norwegian Defender of the Year
        (0x9df908, 0x9cfb60), // Norwegian Midfielder of the Year
        (0x9df8e8, 0x9cfb64), // Norwegian Attacker of the Year
        (0x9df8bc, 0x9cfb68), // Norwegian Premier Division Team of the Week
        (0x9df890, 0x9cfb6c), // Norwegian First Division Team of the Week
        (0x9df85c, 0x9cfb70), // Portuguese Premier Division Most Man of the Match
        (0x9df82c, 0x9cfb70), // Portuguese Premier League Most Man of the Match
        (0x9df800, 0x9cfb74), // Portuguese Premier Division Top Goalscorer
        (0x9df7d4, 0x9cfb74), // Portuguese Premier League Top Goalscorer
        (0x9df7a4, 0x9cfb78), // Portuguese Second Division Player of the Year
        (0x9df778, 0x9cfb78), // Portuguese Second League Player of the Year
        (0x9df748, 0x9cfb7c), // Spanish First Division Goalkeeper of the Year
        (0x9df72c, 0x9cfb80), // Spanish Player of the Year
        (0x9df704, 0x9cfb84), // Spanish First Division Top Goalscorer
        (0x9df6e8, 0x9cfb88), // Swedish Player of the Year
        (0x9df6c0, 0x9cfb8c), // Swedish Premier Division Top Goalscorer
        (0x9df6a0, 0x9cfb90), // Swedish Fans Player of the Year
        (0x9df684, 0x9cfb94), // MLS Goalkeeper of the Year
        (0x9df668, 0x9cfb98), // MLS Defender of the Year
        (0x9df650, 0x9cfb9c), // MLS Rookie of the Year
        (0x9df638, 0x9cfba0), // MLS Scoring Champion
        (0x9df624, 0x9cfba4), // MLS All-Pro Team
        (0x9df60c, 0x9cfba8), // MLS Player of the Month
        (0x9df5e0, 0x9cfbac), // Scottish Premier Division Team of the Year
        (0x9df5b4, 0x9cfbb0), // Scottish First Division Team of the Year
        (0x9df588, 0x9cfbb4), // Scottish Second Division Team of the Year
        (0x9df55c, 0x9cfbb8), // Scottish Third Division Team of the Year
        (0x9df544, 0x9cfbbc), // Scottish Top Goalscorer
        (0x9df514, 0x9cfbc0), // Scottish Premier Division Young Player of Month
        (0x9df4e4, 0x9cfbc4), // Scottish First Division Young Player of Month
        (0x9df4b4, 0x9cfbc8), // Scottish Second Division Young Player of Month
        (0x9df484, 0x9cfbcc), // Scottish Third Division Young Player of Month
        (0x9df468, 0x9cfbd0), // Belgian Manager of the Year
        (0x9df438, 0x9cfbd4), // English Premier Division Manager of the Year
        (0x9df408, 0x9cfbd8), // English Premier Division Manager of the Month
        (0x9df3dc, 0x9cfbdc), // English First Division Manager of the Year
        (0x9df3b0, 0x9cfbe0), // English First Division Manager of the Month
        (0x9df384, 0x9cfbe4), // English Second Division Manager of the Year
        (0x9df354, 0x9cfbe8), // English Second Division Manager of the Month
        (0x9df328, 0x9cfbec), // English Third Division Manager of the Year
        (0x9df2fc, 0x9cfbf0), // English Third Division Manager of the Month
        (0x9df2e0, 0x9cfbf4), // French Manager of the Year
        (0x9df2b4, 0x9cfbf8), // German First Division Manager of the Year
        (0x9df288, 0x9cfbfc), // German Second Division Manager of the Year
        (0x9df26c, 0x9cfc00), // Dutch Manager of the Year
        (0x9df248, 0x9cfc04), // Italian Serie A Manager of the Year
    ],
    // cadeia 2
    &[
        (0x9df224, 0x9cfc08), // Italian Serie B Manager of the Year
        (0x9df1fc, 0x9cfc0c), // Italian Serie C1/A Manager of the Year
        (0x9df1d4, 0x9cfc10), // Italian Serie C1/B Manager of the Year
        (0x9df1a4, 0x9cfc14), // Scottish Premier Division Manager of the Year
        (0x9df174, 0x9cfc18), // Scottish Premier Division Manager of the Month
        (0x9df148, 0x9cfc1c), // Scottish First Division Manager of the Year
        (0x9df118, 0x9cfc20), // Scottish First Division Manager of the Month
        (0x9df0e8, 0x9cfc24), // Scottish Second Division Manager of the Year
        (0x9df0b8, 0x9cfc28), // Scottish Second Division Manager of the Month
        (0x9df08c, 0x9cfc2c), // Scottish Third Division Manager of the Year
        (0x9df05c, 0x9cfc30), // Scottish Third Division Manager of the Month
        (0x9df040, 0x9cfc34), // Spanish Manager of the Year
        (0x9df024, 0x9cfc38), // Swedish Manager of the Year
        (0x9df00c, 0x9cfc3c), // MLS Manager of the Year
        (0x9defe8, 0x9cfc40), // Belgian Young Player of the Year
        (0x9defb8, 0x9cfc44), // Argentinian Premier Division Team of the Week
        (0x9def88, 0x9cfc48), // Argentinian Second Division Team of the Week
        (0x9def6c, 0x9cfc4c), // Japanese Player of the Year
        (0x9def54, 0x9cfc50), // Japanese Top Goalscorer
        (0x9def30, 0x9cfc54), // Japanese Young Player of the Year
        (0x9def10, 0x9cfc58), // Japanese Manager of the Year
        (0x9deee8, 0x9cfc5c), // Japanese J-League 1 Team of the Year
        (0x9deec4, 0x9cfc5c), // Japanese League 1 Team of the Year
        (0x9deeac, 0x9cfc60), // World Cup Golden Boot
        (0x9dee94, 0x9cfc64), // World Cup Dream Team
        (0x9dee74, 0x9cfc68), // German Goalkeeper of the Year
        (0x9dee58, 0x9cfc6c), // German Sweeper of the Year
        (0x9dee3c, 0x9cfc70), // German Defender of the Year
        (0x9dee1c, 0x9cfc74), // German Midfielder of the Year
        (0x9dee00, 0x9cfc78), // German Striker of the Year
        (0x9dedd8, 0x9cfc7c), // German First Division Top Goalscorer
        (0x9dedb8, 0x9cfc80), // English Footballer of the Year
        (0x9ded90, 0x9cfc84), // English Conference Manager of the Month
        (0x9ded6c, 0x9cfc88), // English Players Conference Select
        (0x9ded44, 0x9cfc8c), // English Conference Manager of the Year
        (0x9ded1c, 0x9cfc90), // English Conference Player of the Month
        (0x9decf8, 0x9cfc94), // English Conference Team of the Week
        (0x9decc8, 0x9cfc98), // English Conference Young Player of the Month
        (0x9dec98, 0x9cfc9c), // Portuguese Premier Division Team of the Year
        (0x9dec6c, 0x9cfc9c), // Portuguese Premier League Team of the Year
        (0x9dec40, 0x9cfca0), // Portuguese Second Division Team of the Year
        (0x9dec14, 0x9cfca0), // Portuguese Second League Team of the Year
        (0x9debe4, 0x9cfca4), // Portuguese Premier Division Team of the Week
        (0x9debb8, 0x9cfca4), // Portuguese Premier League Team of the Week
        (0x9deb8c, 0x9cfca8), // Portuguese Second Division Team of the Week
        (0x9deb60, 0x9cfca8), // Portuguese Second League Team of the Week
        (0x9deb38, 0x9cfcac), // Japanese J-League 1 Team of the Week
        (0x9deb14, 0x9cfcac), // Japanese League 1 Team of the Week
        (0x9deaec, 0x9cfcb0), // Japanese J-League 2 Team of the Week
        (0x9deac8, 0x9cfcb0), // Japanese League 2 Team of the Week
        (0x9dea9c, 0x9cfcb4), // Swedish Premier Division Team of the Week
        (0x9dea74, 0x9cfcb8), // Swedish First Division Team of the Week
        (0x9dea44, 0x9cfcb8), // Swedish First Division North Team of the Week
        (0x9dea1c, 0x9cfcbc), // Spanish First Division Team of the Week
        (0x9de9f0, 0x9cfcc0), // Spanish Second Division Team of the Week
        (0x9de9cc, 0x9cfcc4), // Italian Serie A Team of the Week
        (0x9de9a8, 0x9cfcc8), // Italian Serie B Team of the Week
        (0x9de984, 0x9cfccc), // Italian Serie C1/A Team of the Week
        (0x9de960, 0x9cfcd0), // Italian Serie C1/B Team of the Week
        (0x9de938, 0x9cfcd4), // Dutch Premier Division Team of the Week
        (0x9de910, 0x9cfcd8), // Dutch First Division Team of the Week
        (0x9de8e8, 0x9cfcdc), // German First Division Team of the Week
        (0x9de8c0, 0x9cfce0), // German Second Division Team of the Week
        (0x9de898, 0x9cfce4), // French First Division Team of the Week
        (0x9de878, 0x9cfce4), // French First Team of the Week
        (0x9de850, 0x9cfce8), // French Second Division Team of the Week
        (0x9de824, 0x9cfcec), // Danish Premier Division Team of the Week
        (0x9de7fc, 0x9cfcf0), // Danish First Division Team of the Week
        (0x9de7d4, 0x9cfcf4), // Danish Second Division Team of the Week
        (0x9de7ac, 0x9cfcf8), // Belgian First Division Team of the Week
        (0x9de780, 0x9cfcfc), // Belgian Second Division Team of the Week
        (0x9de750, 0x9cfd00), // Portuguese Premier Division Player of the Year
        (0x9de720, 0x9cfd00), // Portuguese Premier League Player of the Year
        (0x9de6ec, 0x9cfd04), // Portuguese Second Division Most Man of the Match
        (0x9de6bc, 0x9cfd04), // Portuguese Second League Most Man of the Match
        (0x9de690, 0x9cfd08), // Portuguese Second Division Top Goalscorer
        (0x9de668, 0x9cfd08), // Portuguese Second League Top Goalscorer
        (0x9de640, 0x9cfd0c), // German Second Division Top Goalscorer
        (0x9de620, 0x9cfd10), // Swedish Goalkeeper of the Year
        (0x9de600, 0x9cfd14), // Swedish Defender of the Year
        (0x9de5e4, 0x9cfd18), // Swedish Forward of the Year
        (0x9de5c8, 0x9cfd1c), // Swedish Rookie of the Year
        (0x9de5a8, 0x9cfd20), // Swedish Midfielder of the Year
        (0x9de580, 0x9cfd24), // Italian Serie C2/A Manager of the Year
        (0x9de558, 0x9cfd28), // Italian Serie C2/A Player of the Year
        (0x9de534, 0x9cfd2c), // Italian Serie C2/A Team of the Week
        (0x9de50c, 0x9cfd30), // Italian Serie C2/B Manager of the Year
        (0x9de4e4, 0x9cfd34), // Italian Serie C2/B Player of the Year
        (0x9de4bc, 0x9cfd38), // Japanese J-League 2 Team of the Year
        (0x9de498, 0x9cfd38), // Japanese League 2 Team of the Year
        (0x9de468, 0x9cfd3c), // Spanish Second Division Goalkeeper of the Year
        (0x9de440, 0x9cfd40), // Spanish Second Division Top Goalscorer
        (0x9de41c, 0x9cfd44), // Italian Serie C2/B Team of the Week
        (0x9de3f4, 0x9cfd48), // Italian Serie C2/C Manager of the Year
        (0x9de3cc, 0x9cfd4c), // Italian Serie C2/C Player of the Year
        (0x9de3a8, 0x9cfd50), // Italian Serie C2/C Team of the Week
        (0x9de380, 0x9cfd54), // Belgian First Division Top Goalscorer
        (0x9de350, 0x9cfd58), // Belgian First Division Young Player of the Year
        (0x9de320, 0x9cfd58), // Belgian First Division Revelation of the Year
        (0x9de2f8, 0x9cfd5c), // Belgian Second Division Top Goalscorer
        (0x9de2d0, 0x9cfd60), // Belgian Third Division Team of the Week
        (0x9de2a4, 0x9cfd64), // Belgian Third Division Player of the Year
        (0x9de27c, 0x9cfd68), // Belgian Third Division Top Goalscorer
        (0x9de25c, 0x9cfd6c), // European Goalkeeper of the Year
        (0x9de23c, 0x9cfd70), // European Defender of the Year
        (0x9de21c, 0x9cfd74), // European Midfielder of the Year
        (0x9de1fc, 0x9cfd78), // European Striker of the Year
    ],
    // cadeia 3
    &[
        (0x9de1d8, 0x9cfd7c), // Italian Serie A Team of the Year
        (0x9de1b4, 0x9cfd80), // Italian Serie B Team of the Year
        (0x9de190, 0x9cfd84), // Italian Serie C1/A Team of the Year
        (0x9de16c, 0x9cfd88), // Italian Serie C1/B Team of the Year
        (0x9de148, 0x9cfd8c), // Italian Serie C2/A Team of the Year
        (0x9de124, 0x9cfd90), // Italian Serie C2/B Team of the Year
        (0x9de100, 0x9cfd94), // Italian Serie C2/C Team of the Year
        (0x9de0e0, 0x9cfd98), // Italian Serie A Top Goalscorer
        (0x9de0c0, 0x9cfd9c), // Italian Serie B Top Goalscorer
    ],
    // cadeia 4
    &[
        (0x9de09c, 0x9cfda0), // Italian Serie C1/A Top Goalscorer
        (0x9de078, 0x9cfda4), // Italian Serie C1/B Top Goalscorer
        (0x9de054, 0x9cfda8), // Italian Serie C2/A Top Goalscorer
        (0x9de030, 0x9cfdac), // Italian Serie C2/B Top Goalscorer
        (0x9de00c, 0x9cfdb0), // Italian Serie C2/C Top Goalscorer
        (0x9ddfdc, 0x9cfdb4), // Norwegian Premier Division Manager of the Month
        (0x9ddfac, 0x9cfdb8), // Norwegian Premier Division Player of the Month
        (0x9ddf7c, 0x9cfdbc), // Norwegian First Division Manager of the Month
        (0x9ddf4c, 0x9cfdc0), // Norwegian First Division Player of the Month
        (0x9ddf1c, 0x9cfdc4), // Portuguese Premier League Manager of the Year
        (0x9ddeec, 0x9cfdc8), // Portuguese Second League Manager of the Year
        (0x9dded0, 0x9cfdcc), // Finnish Player of the Year
        (0x9ddeac, 0x9cfdd0), // Finnish Young Player of the Year
        (0x9dde8c, 0x9cfdd4), // Finnish Goalkeeper of the Year
        (0x9dde70, 0x9cfdd8), // Finnish Manager of the Year
        (0x9dde48, 0x9cfddc), // Finnish Premier Division Top Goalscorer
        (0x9dde18, 0x9cfde0), // Finnish Premier Division Player of the Month
        (0x9dddec, 0x9cfde4), // Finnish Premier Division Team of the Year
        (0x9dddc0, 0x9cfde8), // Finnish Premier Division Team of the Week
        (0x9ddd98, 0x9cfdec), // Finnish First Division Top Goalscorer
        (0x9ddd70, 0x9cfdf0), // Finnish First Division Team of the Year
        (0x9ddd48, 0x9cfdf4), // Finnish First Division Team of the Week
        (0x9ddd2c, 0x9cfdf8), // Irish Player Of The Year
        (0x9ddd0c, 0x9cfdfc), // Irish Young Player Of The Year
        (0x9ddce4, 0x9cfe00), // Irish Premier Division Team Of The Week
        (0x9ddcb8, 0x9cfe04), // Irish Premier Division Player Of The Year
        (0x9ddc88, 0x9cfe08), // Irish Premier Division Young Player Of The Year
        (0x9ddc60, 0x9cfe0c), // Irish Premier Division Team Of The Year
        (0x9ddc38, 0x9cfe10), // Irish First Division Team Of The Week
        (0x9ddc10, 0x9cfe14), // Irish First Division Player Of The Year
        (0x9ddbe0, 0x9cfe18), // Irish First Division Young Player Of The Year
        (0x9ddbb8, 0x9cfe1c), // Irish First Division Team Of The Year
        (0x9ddb8c, 0x9cfe20), // Irish Premier Division Manager of the Year
        (0x9ddb60, 0x9cfe24), // Irish Premier Division Manager of the Month
        (0x9ddb34, 0x9cfe28), // Irish First Division Manager of the Year
        (0x9ddb08, 0x9cfe2c), // Irish First Division Manager of the Month
        (0x9ddae0, 0x9cfe30), // polish first division team of the week
        (0x9ddab4, 0x9cfe34), // polish first division player of the year
        (0x9dda84, 0x9cfe38), // polish first division young player of the YEAR
        (0x9dda58, 0x9cfe3c), // polish first division manager of the year
        (0x9dda2c, 0x9cfe40), // polish first division manager of the month
        (0x9dda04, 0x9cfe44), // polish second division team of the week
        (0x9dd9d8, 0x9cfe48), // polish second division player of the year
        (0x9dd9a8, 0x9cfe4c), // polish second division young player of the year
        (0x9dd97c, 0x9cfe50), // polish second division manager of the year
        (0x9dd950, 0x9cfe54), // polish second division manager of the month
        (0x9dd924, 0x9cfe58), // Croatian first division player of the year
        (0x9dd8f8, 0x9cfe5c), // croatian first division team of the week
        (0x9dd8d0, 0x9cfe60), // croatian first division top goalscorer
        (0x9dd8a0, 0x9cfe64), // croatian second division a player of the year
        (0x9dd874, 0x9cfe68), // croatian second division a team of the week
        (0x9dd848, 0x9cfe6c), // croatian second division a top goalscorer
        (0x9dd818, 0x9cfe70), // croatian second division b player of the year
        (0x9dd7ec, 0x9cfe74), // croatian second division b team of the week
        (0x9dd7c0, 0x9cfe78), // croatian second division b top goalscorer
        (0x9dd7a0, 0x9cfe7c), // australian nsl team of the week
        (0x9dd780, 0x9cfe80), // australian manager of the month
        (0x9dd75c, 0x9cfe80), // australian nsl manager of the month
        (0x9dd73c, 0x9cfe84), // australian player of the month
        (0x9dd718, 0x9cfe84), // australian nsl player of the month
        (0x9dd6ec, 0x9cfe88), // australian nsl young player of the month
        (0x9dd6c4, 0x9cfe88), // australian young player of the month
        (0x9dd6a4, 0x9cfe8c), // australian player of the year
        (0x9dd680, 0x9cfe8c), // australian nsl player of the year
        (0x9dd658, 0x9cfe90), // australian nsl u21 player of the year
        (0x9dd630, 0x9cfe90), // australian nsl young player of the year
        (0x9dd60c, 0x9cfe94), // australian goalkeeper of the year
        (0x9dd5e4, 0x9cfe94), // australian nsl goalkeeper of the year
        (0x9dd5c8, 0x9cfe98), // australian top goalscorer
        (0x9dd5a4, 0x9cfe98), // australian nsl leading goalscorer
        (0x9dd584, 0x9cfe9c), // australian manager of the year
        (0x9dd560, 0x9cfe9c), // australian nsl manager of the year
        (0x9dd544, 0x9cfea0), // greek player of the year
        (0x9dd520, 0x9cfea4), // greek foreign player of the year
        (0x9dd4f8, 0x9cfea8), // greek premier division team of the week
        (0x9dd4cc, 0x9cfeac), // greek premier division player of the year
        (0x9dd4a0, 0x9cfeb0), // greek premier division manager of the year
        (0x9dd478, 0x9cfeb4), // greek premier division top goalscorer
        (0x9dd450, 0x9cfeb8), // greek second division team of the week
        (0x9dd424, 0x9cfebc), // greek second division player of the year
        (0x9dd3f8, 0x9cfec0), // greek second division manager of the year
        (0x9dd3d0, 0x9cfec4), // greek second division top goalscorer
        (0x9dd3b4, 0x9cfec8), // turkish player of the year
        (0x9dd398, 0x9cfecc), // turkish manager of the year
        (0x9dd36c, 0x9cfed0), // turkish premier division team of the week
        (0x9dd340, 0x9cfed4), // turkish premier division team of the year
        (0x9dd318, 0x9cfed8), // turkish first division team of the week
        (0x9dd2f0, 0x9cfedc), // turkish first division team of the year
        (0x9dd2d4, 0x9cfee0), // russian player of the year
        (0x9dd2a8, 0x9cfee4), // russian premier division team of the week
        (0x9dd27c, 0x9cfee8), // russian premier division team of the year
        (0x9dd24c, 0x9cfeec), // russian premier division player of the month
        (0x9dd224, 0x9cfef0), // russian first division team of the week
        (0x9dd1fc, 0x9cfef4), // russian first division team of the year
        (0x9dd1d0, 0x9cfef8), // russian first division player of the month
        (0x9dd1b0, 0x9cfefc), // Irish Personality of the Year
        (0x9dd194, 0x9cff00), // n. irish player of the year
        (0x9dd168, 0x9cff04), // n. irish premier division team of the week
        (0x9dd138, 0x9cff08), // n. irish premier division player of the year
        (0x9dd104, 0x9cff0c), // n. irish premier division young player of the year
        (0x9dd0d8, 0x9cff10), // n. irish premier division top goalscorer
        (0x9dd0a8, 0x9cff14), // n. irish premier division manager of the year
        (0x9dd078, 0x9cff18), // n. irish premier division manager of the month
        (0x9dd04c, 0x9cff1c), // n. irish first division team of the week
        (0x9dd020, 0x9cff20), // n. irish first division player of the year
        (0x9dcfec, 0x9cff24), // n. irish first division young player of the year
        (0x9dcfc4, 0x9cff28), // n. irish first division top goalscorer
        (0x9dcf98, 0x9cff2c), // n. irish first division manager of the year
        (0x9dcf68, 0x9cff30), // n. irish first division manager of the month
        (0x9dcf4c, 0x9cff34), // irish league golden boot
        (0x9dcf30, 0x9cff38), // irish league bronze boot
        (0x9dcf00, 0x9cff3c), // belgian second division revelation of the year
        (0x9dcee8, 0x9cff40), // welsh team of the week
        (0x9dcecc, 0x9cff44), // welsh player of the year
        (0x9dceac, 0x9cff48), // welsh young player of the year
        (0x9dce94, 0x9cff4c), // welsh top goalscorer
        (0x9dce78, 0x9cff50), // welsh manager of the year
        (0x9dce5c, 0x9cff54), // welsh manager of the month
        (0x9dce34, 0x9cff58), // french first division team of the year
        (0x9dce0c, 0x9cff5c), // french second division team of the year
        (0x9dcde8, 0x9cff60), // french national player of the year
        (0x9dcdc0, 0x9cff64), // french national goalkeeper of the year
        (0x9dcd9c, 0x9cff68), // french national team of the week
        (0x9dcd78, 0x9cff6c), // french national team of the year
        (0x9dcd5c, 0x9cff70), // irish player of the month
        (0x9dcd30, 0x9cff74), // french players national player of the year
        (0x9dcd14, 0x9cff78), // oceania player of the year
        (0x9dcce8, 0x9cff7c), // finnish first division player of the month
        (0x9dccb8, 0x9cff80), // finnish premier division manager of the month
        (0x9dcc8c, 0x9cff84), // finnish first division manager of the month
        (0x9dcc60, 0x9cff88), // Brazilian 1st Division Player of the Year
    ],
    // cadeia 5
    &[
        (0x9dcc38, 0x9cff8c), // Brazilian 1st Division Team of the Week
        (0x9dcc10, 0x9cff90), // Brazilian 1st Division Team of the Year
        (0x9dcbe8, 0x9cff94), // Brazilian 1st Division Top Goalscorer
        (0x9dcbb8, 0x9cff98), // Brazilian 1st Division Young Player of the Year
        (0x9dcb8c, 0x9cff9c), // Brazilian 2nd Division Player of the Year
        (0x9dcb64, 0x9cffa0), // Brazilian 2nd Division Team of the Week
        (0x9dcb3c, 0x9cffa4), // Brazilian 2nd Division Team of the Year
        (0x9dcb14, 0x9cffa8), // Brazilian 2nd Division Top Goalscorer
        (0x9dcae4, 0x9cffac), // Brazilian 2nd Division Young Player of the Year
        (0x9dcab8, 0x9cffb0), // Brazilian 3rd Division Player of the Year
        (0x9dca90, 0x9cffb4), // Brazilian 3rd Division Team of the Week
        (0x9dca68, 0x9cffb8), // Brazilian 3rd Division Team of the Year
        (0x9dca40, 0x9cffbc), // Brazilian 3rd Division Top Goalscorer
        (0x9dca10, 0x9cffc0), // Brazilian 3rd Division Young Player of the Year
        (0x9dc9f0, 0x9cffc4), // Brazilian Bahia Best Goalkeeper
        (0x9dc9d4, 0x9cffc8), // Brazilian Bahia Best Player
        (0x9dc9b0, 0x9cffcc), // Brazilian Bahia Team of the Week
        (0x9dc990, 0x9cffd0), // Brazilian Bahia Top Goalscorer
        (0x9dc96c, 0x9cffd4), // Brazilian Central Best Goalkeeper
        (0x9dc94c, 0x9cffd8), // Brazilian Central Best Player
        (0x9dc928, 0x9cffdc), // Brazilian Central Team of the Week
        (0x9dc904, 0x9cffe0), // Brazilian Central Top Goalscorer
        (0x9dc8e0, 0x9cffe4), // Brazilian Gaúcho Best Goalkeeper
        (0x9dc8c0, 0x9cffe8), // Brazilian Gaúcho Best Player
        (0x9dc89c, 0x9cffec), // Brazilian Gaúcho Team of the Week
        (0x9dc87c, 0x9cfff0), // Brazilian Gaúcho Top Goalscorer
        (0x9dc85c, 0x9cfff4), // Brazilian Goiás Best Goalkeeper
        (0x9dc840, 0x9cfff8), // Brazilian Goiás Best Player
        (0x9dc81c, 0x9cfffc), // Brazilian Goiás Team of the Week
        (0x9dc7fc, 0x9d0000), // Brazilian Goiás Top Goalscorer
        (0x9dc7d4, 0x9d0004), // Brazilian Minas Gerais Best Goalkeeper
        (0x9dc7b0, 0x9d0008), // Brazilian Minas Gerais Best Player
        (0x9dc788, 0x9d000c), // Brazilian Minas Gerais Team of the Week
        (0x9dc760, 0x9d0010), // Brazilian Minas Gerais Top Goalscorer
        (0x9dc740, 0x9d0014), // Brazilian North Best Goalkeeper
        (0x9dc724, 0x9d0018), // Brazilian North Best Player
        (0x9dc700, 0x9d001c), // Brazilian North Team of the Week
        (0x9dc6e0, 0x9d0020), // Brazilian North Top Goalscorer
        (0x9dc6bc, 0x9d0024), // Brazilian Northeast Best Goalkeeper
        (0x9dc69c, 0x9d0028), // Brazilian Northeast Best Player
        (0x9dc674, 0x9d002c), // Brazilian Northeast Team of the Week
        (0x9dc650, 0x9d0030), // Brazilian Northeast Top Goalscorer
        (0x9dc62c, 0x9d0034), // Brazilian Paraná Best Goalkeeper
        (0x9dc60c, 0x9d0038), // Brazilian Paraná Best Player
        (0x9dc5e8, 0x9d003c), // Brazilian Paraná Team of the Week
        (0x9dc5c8, 0x9d0040), // Brazilian Paraná Top Goalscorer
        (0x9dc5a0, 0x9d0044), // Brazilian Pernambuco Best Goalkeeper
        (0x9dc57c, 0x9d0048), // Brazilian Pernambuco Best Player
        (0x9dc554, 0x9d004c), // Brazilian Pernambuco Team of the Week
        (0x9dc530, 0x9d0050), // Brazilian Pernambuco Top Goalscorer
        (0x9dc504, 0x9d0054), // Brazilian Rio de Janeiro Best Goalkeeper
        (0x9dc4dc, 0x9d0058), // Brazilian Rio de Janeiro Best Player
        (0x9dc4b0, 0x9d005c), // Brazilian Rio de Janeiro Team of the Week
        (0x9dc488, 0x9d0060), // Brazilian Rio de Janeiro Top Goalscorer
        (0x9dc45c, 0x9d0064), // Brazilian Santa Catarina Best Goalkeeper
        (0x9dc434, 0x9d0068), // Brazilian Santa Catarina Best Player
        (0x9dc408, 0x9d006c), // Brazilian Santa Catarina Team of the Week
        (0x9dc3e0, 0x9d0070), // Brazilian Santa Catarina Top Goalscorer
        (0x9dc3bc, 0x9d0074), // Brazilian São Paulo Best Goalkeeper
        (0x9dc39c, 0x9d0078), // Brazilian São Paulo Best Player
        (0x9dc374, 0x9d007c), // Brazilian São Paulo Team of the Week
        (0x9dc350, 0x9d0080), // Brazilian São Paulo Top Goalscorer
        (0x9dc32c, 0x9d0084), // South Korean Best 11 Of The Year
        (0x9dc304, 0x9d0088), // South Korean Goalkeeper Of The Month
        (0x9dc2e0, 0x9d008c), // South Korean Manager Of The Year
        (0x9dc2b0, 0x9d0090), // South Korean Most Assisted Player Of The Year
        (0x9dc28c, 0x9d0094), // South Korean Player Of The Month
        (0x9dc26c, 0x9d0098), // South Korean Player Of The Year
        (0x9dc240, 0x9d009c), // South Korean Top Goal Scorer Of The Year
        (0x9dc218, 0x9d00a0), // South Korean Young Player Of The Year
    ],
];

/// { 0x60efd0 } (cdecl, index.cpp, match_club_comp_key_id): casa o nome de competição de clube da base com as chaves fixas
/// do jogo (CHAVES_DE_COMPETICAO_DE_CLUBE): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
unsafe extern "C" fn casar_chave_de_competicao_de_clube(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_COMPETICAO_DE_CLUBE.iter() {
        let mut i = 0;
        while i < cadeia.len() {
            let (texto, global) = cadeia[i];
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de competição de clube
                // as gravações extras do mesmo acerto (texto 0)
                while i + 1 < cadeia.len() && cadeia[i + 1].0 == 0 {
                    i += 1;
                    *(cadeia[i].1 as *mut u32) = linha;
                }
                break;
            }
            i += 1;
            while i < cadeia.len() && cadeia[i].0 == 0 {
                i += 1;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de competição de clube)
}

/// { 0x6115e0 } (cdecl, index.cpp, match_staff_comp_key_id): casa o nome de competição ou prêmio de pessoa da base com as chaves fixas
/// do jogo (CHAVES_DE_COMPETICAO_DE_PESSOA): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
unsafe extern "C" fn casar_chave_de_competicao_de_pessoa(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x951c50usize);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_COMPETICAO_DE_PESSOA.iter() {
        for &(texto, global) in cadeia.iter() {
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de competição de pessoa
                break;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de competição de pessoa)
}

/// As globais (dword) que ficaram diferentes entre a original e a cópia: "[end] original x rust".
fn chaves_de_competicao_diferentes(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> String {
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

unsafe extern "C" fn comparar_casar_chave_de_competicao_de_clube(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(623);
    let reg: [(usize, usize); 1] = [(0x9cf548, 0x4c0)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_competicao_de_clube(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_competicao_de_clube(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_de_competicao_diferentes(&mo, &mr))
    });
    r
}

unsafe extern "C" fn comparar_casar_chave_de_competicao_de_pessoa(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(624);
    let reg: [(usize, usize); 1] = [(0x9cfa20, 0x684)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_competicao_de_pessoa(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_competicao_de_pessoa(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_de_competicao_diferentes(&mo, &mr))
    });
    r
}
