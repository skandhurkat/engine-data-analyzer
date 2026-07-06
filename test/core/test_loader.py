import engine_data_analyzer.core.loader as loader
import engine_data_analyzer.core.units as units

def test_cht_cols_regex():
    cht_regex = loader._cht_cols_regex
    assert cht_regex.match("CHT 5 (deg C)")
    assert cht_regex.match("CHT 4")
    assert not cht_regex.match("CHT 1 ")

def test_cht_col_unit():
    get_cht_col_unit = loader._get_cht_col_unit
    assert get_cht_col_unit("CHT 5 (deg C)") == units.TemperatureUnit.CELSIUS
    assert get_cht_col_unit("CHT 2 (deg F)") == units.TemperatureUnit.FARENHEIT
    assert get_cht_col_unit("CHT 3") == units.TemperatureUnit.UNKNOWN

def test_cht_cyl_number():
    get_cht_col_cyl_num = loader._get_cht_col_cyl_num
    assert get_cht_col_cyl_num("CHT 5 (deg C)") == 5
    assert get_cht_col_cyl_num("CHT 4 (deg F)") == 4
    assert get_cht_col_cyl_num("CHT 7") == 7
