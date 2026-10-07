use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// content_inspector only looks at the first 1024 bytes, so reading more is wasted I/O.
const SCAN: usize = 1024;

// `Read::read` may return fewer bytes than requested even when more data remains
// (pipes, network filesystems, signal interruptions), so we loop until we have
// SCAN bytes or hit EOF. `n` tracks how many bytes we've read so far, and each
// read picks up at `buf[n..]`.
fn read_prefix(path: &PathBuf, buf: &mut [u8; SCAN]) -> std::io::Result<usize> {
    let mut f = File::open(path)?;
    let mut n = 0;
    while n < SCAN {
        match f.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(n)
}

/// Read up to 1024 bytes of `path` and classify with content_inspector.
#[pyfunction]
fn is_binary(path: PathBuf) -> PyResult<bool> {
    let mut buf = [0u8; SCAN];
    let n = read_prefix(&path, &mut buf)?;
    Ok(content_inspector::inspect(&buf[..n]).is_binary())
}

/// Classify an in-memory buffer (no I/O).
#[pyfunction]
fn is_binary_bytes(data: &Bound<'_, PyBytes>) -> bool {
    content_inspector::inspect(data.as_bytes()).is_binary()
}

/// Return the ContentType name ("BINARY", "UTF_8", "UTF_16LE", ...).
#[pyfunction]
fn inspect(path: PathBuf) -> PyResult<String> {
    let mut buf = [0u8; SCAN];
    let n = read_prefix(&path, &mut buf)?;
    Ok(format!("{:?}", content_inspector::inspect(&buf[..n])))
}

#[pymodule]
fn content_inspector_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_binary, m)?)?;
    m.add_function(wrap_pyfunction!(is_binary_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(inspect, m)?)?;
    Ok(())
}
