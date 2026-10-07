# content_inspector_py

Python bindings (PyO3 + maturin) for `content_inspector`.

```bash
pip install maturin
cd python
maturin develop --release
```

```python
import content_inspector_py as ci

ci.is_binary("some/file")        # True / False
ci.is_binary_bytes(b"\x00\x01")  # True
ci.inspect("some/file")          # "BINARY", "UTF_8", "UTF_16LE", ...
```
