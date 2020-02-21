pub mod palette;

use lodepng::ffi;
use nalgebra::DMatrix;
use palette::PaletteMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

pub struct Palette<'a>(pub &'a [(f64, f64, f64)]);

pub const MAGMA: Palette<'static> = Palette(&palette::MAGMA);
pub const VIRIDIS: Palette<'static> = Palette(&palette::VIRIDIS);
pub const INFERNO: Palette<'static> = Palette(&palette::INFERNO);
pub const PLASMA: Palette<'static> = Palette(&palette::PLASMA);
pub const MORELAND: Palette<'static> = Palette(&palette::MORELAND);

pub struct PngWriter<'a> {
    filename: PathBuf,
    data_range: Option<Range<f64>>,
    palette: Palette<'a>,
    downsample: Option<usize>,
}

impl PngWriter<'static> {
    pub fn new(filename: impl AsRef<Path>) -> Self {
        Self {
            filename: filename.as_ref().to_owned(),
            data_range: None,
            palette: Palette(&palette::VIRIDIS),
            downsample: None,
        }
    }
}

impl<'a> PngWriter<'a> {
    pub fn palette<'b>(self, palette: Palette<'b>) -> PngWriter<'b> {
        PngWriter::<'b> {
            filename: self.filename,
            data_range: self.data_range,
            palette,
            downsample: self.downsample,
        }
    }

    pub fn data_range(self, range: Range<f64>) -> Self {
        Self {
            data_range: Some(range),
            ..self
        }
    }

    pub fn data_range_auto(self) -> Self {
        Self {
            data_range: None,
            ..self
        }
    }

    pub fn downsample(self, downsample: usize) -> Self {
        Self {
            downsample: Some(downsample),
            ..self
        }
    }

    pub fn write(&self, data: &DMatrix<f64>) -> Result<(), ffi::Error> {
        assert!(
            self.downsample.filter(|&d| d != 0).is_none(),
            "downsampling unsupported with DMatrix"
        );

        let (w, h) = data.shape();
        let data = data.as_slice();

        // build mapper to convert data values -> colors
        let colors = PaletteMap::new(self.palette.0);
        // set color data range
        #[rustfmt::skip]
        let colors = self.data_range.clone().map_or_else(
            || colors.auto_range(&data),
            |range| colors.range(range.start, range.end)
        );

        // get colors according to palette
        let data = colors.map_palette(&data);
        // index colors/generate palette for image
        let (palette, data) = palette::index_colors(data);

        let mut state = ffi::State::new();
        state.info_raw_mut().colortype = ffi::ColorType::PALETTE;
        for c in palette {
            state.info_raw_mut().palette_add(c.alpha(255))?;
        }

        state.encode_file(&self.filename, &data, w, h)
    }
}

pub fn write_png(path: impl AsRef<Path>, data: &DMatrix<f64>) -> Result<(), ffi::Error> {
    PngWriter::new(path).write(data)
}
