#![allow(non_snake_case)]

use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
struct UFOReader {}

#[pymethods]
impl UFOReader {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(UFOReader {})
    }

    pub fn close(&self) {}
    pub fn fileStructure(&self) {}
    pub fn formatVersion(&self) {}
    pub fn formatVersionTuple(&self) {}
    pub fn getCharacterMapping(&self, layerName: &PyAny, validate: &PyAny) {}
    pub fn getDataDirectoryListing(&self) {}
    pub fn getDefaultLayerName(&self, validate: &PyAny) {}
    pub fn getFileModificationTime(&self, path: &PyAny) {}
    pub fn getGlyphSet(&self, layerName: &PyAny, validateRead: &PyAny, validateWrite: &PyAny) {}
    pub fn getImageDirectoryListing(&self, validate: &PyAny) {}
    pub fn getKerningGroupConversionRenameMaps(&self, validate: &PyAny) {}
    pub fn getLayerNames(&self, validate: &PyAny) {}
    pub fn getReadFileForPath(&self, path: &PyAny, encoding: &PyAny) {}
    pub fn path(&self) {}
    pub fn readBytesFromPath(&self, path: &PyAny) {}
    pub fn readData(&self, fileName: &PyAny) {}
    pub fn readFeatures(&self) {}
    pub fn readGroups(&self, validate: &PyAny) {}
    pub fn readImage(&self, fileName: &PyAny, validate: &PyAny) {}
    pub fn readInfo(&self, info: &PyAny, validate: &PyAny) {}
    pub fn readKerning(&self, validate: &PyAny) {}
    pub fn readLib(&self, validate: &PyAny) {}
    pub fn readMetaInfo(&self, validate: &PyAny) {}
}

#[pyclass(extends=UFOReader)]
struct UFOWriter {}

#[pymethods]
impl UFOWriter {
    #[new]
    fn new() -> PyResult<(Self, UFOReader)> {
        Ok((UFOWriter {}, UFOReader {}))
    }
    pub fn close(&self) {}
    pub fn copyFromReader(&self, reader: &PyAny, sourcePath: &PyAny, destPath: &PyAny) {}
    pub fn copyImageFromReader(
        &self,
        reader: &PyAny,
        sourceFileName: &PyAny,
        destFileName: &PyAny,
        validate: &PyAny,
    ) {
    }
    pub fn deleteGlyphSet(&self, layerName: &PyAny) {}
    pub fn fileCreator(&self) {}
    pub fn getFileObjectForPath(&self, path: &PyAny, mode: &PyAny, encoding: &PyAny) {}
    pub fn getGlyphSet(
        &self,
        layerName: &PyAny,
        defaultLayer: &PyAny,
        glyphNameToFileNameFunc: &PyAny,
        validateRead: &PyAny,
        validateWrite: &PyAny,
    ) {
    }
    pub fn removeData(&self, fileName: &PyAny) {}
    pub fn removeFileForPath(&self, path: &PyAny, force: &PyAny, removeEmptyParents: &PyAny) {}
    pub fn removeImage(&self, fileName: &PyAny, validate: &PyAny) {}
    pub fn removePath(&self, path: &PyAny, force: &PyAny, removeEmptyParents: &PyAny) {}
    pub fn renameGlyphSet(&self, layerName: &PyAny, newLayerName: &PyAny, defaultLayer: &PyAny) {}
    pub fn setKerningGroupConversionRenameMaps(&self, maps: &PyAny) {}
    pub fn setModificationTime(&self) {}
    pub fn writeBytesToPath(&self, path: &PyAny, data: &PyAny) {}
    pub fn writeData(&self, fileName: &PyAny, data: &PyAny) {}
    pub fn writeFeatures(&self, features: &PyAny, validate: &PyAny) {}
    pub fn writeGroups(&self, groups: &PyAny, validate: &PyAny) {}
    pub fn writeImage(&self, fileName: &PyAny, data: &PyAny, validate: &PyAny) {}
    pub fn writeInfo(&self, info: &PyAny, validate: &PyAny) {}
    pub fn writeKerning(&self, kerning: &PyAny, validate: &PyAny) {}
    pub fn writeLayerContents(&self, layerOrder: &PyAny, validate: &PyAny) {}
    pub fn writeLib(&self, libDict: &PyAny, validate: &PyAny) {}
}

#[pyfunction]
fn validateFontInfoVersion2ValueForAttribute(_attr: &PyAny, _value: &PyAny) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Support for UFO v1 and v2 is not implemented."))
}

#[pyfunction]
fn validateFontInfoVersion3ValueForAttribute(_attr: &PyAny, _value: &PyAny) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Not yet implemented."))
}

#[pyfunction]
fn convertFontInfoValueForAttributeFromVersion1ToVersion2(
    _attr: &PyAny,
    _value: &PyAny,
) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Support for UFO v1 and v2 is not implemented."))
}

#[pyfunction]
fn convertFontInfoValueForAttributeFromVersion2ToVersion1(
    _attr: &PyAny,
    _value: &PyAny,
) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Support for UFO v1 and v2 is not implemented."))
}

#[pyfunction]
fn convertFontInfoValueForAttributeFromVersion3ToVersion2(
    _attr: &PyAny,
    _value: &PyAny,
) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Support for UFO v1 and v2 is not implemented."))
}

#[pyfunction]
fn convertFontInfoValueForAttributeFromVersion2ToVersion3(
    _attr: &PyAny,
    _value: &PyAny,
) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Support for UFO v1 and v2 is not implemented."))
}

#[pyfunction]
fn makeUFOPath(_path: &PyAny) -> PyResult<()> {
    Err(PyNotImplementedError::new_err("Not yet implemented."))
}

/// A Python module implemented in Rust.
#[pymodule]
fn ufolib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UFOReader>()?;
    m.add_class::<UFOWriter>()?;
    m.add_function(wrap_pyfunction!(convertFontInfoValueForAttributeFromVersion1ToVersion2, m)?)?;
    m.add_function(wrap_pyfunction!(convertFontInfoValueForAttributeFromVersion2ToVersion1, m)?)?;
    m.add_function(wrap_pyfunction!(convertFontInfoValueForAttributeFromVersion2ToVersion3, m)?)?;
    m.add_function(wrap_pyfunction!(convertFontInfoValueForAttributeFromVersion3ToVersion2, m)?)?;
    m.add_function(wrap_pyfunction!(validateFontInfoVersion2ValueForAttribute, m)?)?;
    m.add_function(wrap_pyfunction!(validateFontInfoVersion3ValueForAttribute, m)?)?;
    m.add_function(wrap_pyfunction!(makeUFOPath, m)?)?;

    // fields missing:
    // "UFOLibError",
    // "UFOReaderWriter",
    // "UFOFileStructure",
    // "fontInfoAttributesVersion1",
    // "fontInfoAttributesVersion2",
    // "fontInfoAttributesVersion3",
    // "deprecatedFontInfoAttributesVersion2",

    Ok(())
}
