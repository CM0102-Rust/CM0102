# Mutacoes das cores do kit, do tecnico do clube e do nome em ingles. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq8_casa', "1 => (r(0x83), r(0x87)), // casa", "1 => (r(0x87), r(0x83)), // casa"),
    ('dq8_tecnico', "            return t; // o técnico do clube", "            return 0; // o técnico do clube"),
    ('dq8_ingles', "clube.wrapping_add(4); // idioma 0 ou 1", "clube.wrapping_add(5); // idioma 0 ou 1"),
]
