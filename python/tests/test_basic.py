import content_inspector_py as ci


def test_is_binary_bytes():
    assert ci.is_binary_bytes(b"hello world\n") is False
    assert ci.is_binary_bytes(b"\x00\x01\x02\x03") is True


def test_is_binary_file(tmp_path):
    text = tmp_path / "a.txt"
    text.write_text("hello world\n")
    binary = tmp_path / "a.bin"
    binary.write_bytes(b"\x00\x01\x02\x03" * 10)
    assert ci.is_binary(str(text)) is False
    assert ci.is_binary(str(binary)) is True


def test_inspect_name(tmp_path):
    p = tmp_path / "a.bin"
    p.write_bytes(b"\x00\x01\x02\x03")
    assert ci.inspect(str(p)) == "BINARY"
