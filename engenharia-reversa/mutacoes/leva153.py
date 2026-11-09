# Mutacoes da leva153 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva153.py --carreira 2002-06-01
MUTACOES = [
    ('m_pendente', "                if e <= -1 && e != -2 {\n                    let r = exe::weather_check_postponement(", "                if e <= -1 && e != -3 {\n                    let r = exe::weather_check_postponement("),
    ('m_decidida', "                    if r & 1 != 0 {\n                        crate::comparison::branch(\"setup decided before\");", "                    if r & 2 != 0 {\n                        crate::comparison::branch(\"setup decided before\");"),
    ('m_estado', "    *((day + match_day_fields::PASS) as *mut u8) = 1;\n    por(day, 0x34, 0);", "    *((day + match_day_fields::PASS) as *mut u8) = 2;\n    por(day, 0x34, 0);"),
    ('m_ordenar', "                exe::crt_qsort(read_usize(day + match_day_fields::MATCHES) + de as usize * 0x69, (ate - de + 1) as u32, 0x69, 0x6b6510);\n", ""),
    ('m_competicao', "            por(read_usize(day + match_day_fields::MATCHES) + k as usize * 0x69, 0x63, c as u32);", "            por(read_usize(day + match_day_fields::MATCHES) + k as usize * 0x69, 0x63, read_usize(day + match_day_fields::COMPETITIONS) as u32);"),
    ('m_interesse', "    exe::match_day_refresh_human_interest_flags(day);\n    day\n", "    day\n"),
    ('l_lado', "    (j.side() as u8)\n", "    (j.side() as u8) ^ 1\n"),
    ('s_canal', "            if *canal == 0xfe {\n                return 0;", "            if *canal == 0xfd {\n                return 0;"),
]
