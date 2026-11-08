# Mutacoes da leva153 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva153.py --carreira 2002-06-01
MUTACOES = [
    ('m_pendente', "                if e <= -1 && e != -2 {\n                    let r = decidida(", "                if e <= -1 && e != -3 {\n                    let r = decidida("),
    ('m_decidida', "                    if r & 1 != 0 {\n                        crate::comparison::branch(\"montagem decidida antes\");", "                    if r & 2 != 0 {\n                        crate::comparison::branch(\"montagem decidida antes\");"),
    ('m_estado', "    *((day + 0x33) as *mut u8) = 1;\n    por(day, 0x34, 0);", "    *((day + 0x33) as *mut u8) = 2;\n    por(day, 0x34, 0);"),
    ('m_ordenar', "                sort(u(day, 0x19) + de as usize * 0x69, (ate - de + 1) as u32, 0x69, 0x6b6510);\n", ""),
    ('m_competicao', "            por(u(day, 0x19) + k as usize * 0x69, 0x63, c as u32);", "            por(u(day, 0x19) + k as usize * 0x69, 0x63, u(day, 0xd) as u32);"),
    ('m_interesse', "    interest(day);\n    day\n", "    day\n"),
    ('l_lado', "    *((j + match_player::SIDE) as *const u8)\n", "    *((j + match_player::SIDE) as *const u8) ^ 1\n"),
    ('s_canal', "            if *canal == 0xfe {\n                return 0;", "            if *canal == 0xfd {\n                return 0;"),
]
