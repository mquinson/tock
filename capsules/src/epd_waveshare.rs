//! WaveShare E-Paper Display.
//!
//! One day, we may support as many devices as https://github.com/ZinggJM/GxEPD2 but for now we only have the following one:
//! 
//! ED037TC1 3.7" b/w 280x480, SSD1677, Waveshare 3.7"
//! 
//! - <https://www.waveshare.com/wiki/3.7inch_e-Paper_HAT_Manual#Working_Principle>
//! - <https://www.waveshare.com/w/upload/2/2a/SSD1677_1.0.pdf>
//! 
//!
//! Usage
//! -----
//!
//! TODO

//use core::cell::Cell;
use chips::nrf52840::spi;
use kernel::hil::spi::SpiMasterDevice;
use kernel::utilities::cells::OptionalCell;
use kernel::hil::screen::{
    Screen, ScreenClient, ScreenPixelFormat, ScreenRotation,
};
use kernel::ErrorCode;
//use kernel::debug;
//use components::gpio::GpioComponent;

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::Screen as usize;

/// Monochrome frame buffer bytes.
/// 280 × 480 pixels = 16 800 bytes.
///
pub const BUF_LEN: usize = 16800;

/// Arranges frame data in a buffer
/// whose portions can be sent directly to the device.
struct FrameBuffer<'a> {
    data: &'a mut [u8],
}

impl<'a> FrameBuffer<'a> {
    /// Turns a regular buffer (back) into a FrameBuffer.
    /// If the buffer is fresh, and the display is initialized,
    /// this *MUST* be initialized after the call to `new`.
    fn new(frame_buffer: &'a mut [u8]) -> Self {
        Self { data: frame_buffer }
    }
}

#[derive(Debug)]
pub enum InitError {
    BufferTooSmall,
}

pub struct ED037TC1<'a, S: SpiMasterDevice> {
    spi: &'a S,
    frame_buffer: OptionalCell<FrameBuffer<'static>>,
    client: OptionalCell<&'static dyn ScreenClient>,
}
impl<'a, S: SpiMasterDevice> ED037TC1<'a, S>
where
    Self: 'static,
{
    pub fn new(
        spi: &'a S,
        frame_buffer: &'static mut [u8],
    ) -> Result<Self, InitError> {
        if frame_buffer.len() < BUF_LEN {
            Err(InitError::BufferTooSmall)
        } else {
            Ok(Self {
                spi,
                frame_buffer: OptionalCell::new(FrameBuffer::new(frame_buffer)),
                client: OptionalCell::empty(),
            })
        }
    }
}
impl<'a, S: SpiMasterDevice> Screen for ED037TC1<'a, S>
where
    Self: 'static,
{
    fn get_resolution(&self) -> (usize, usize) {
        (280, 480)
    }

    fn get_pixel_format(&self) -> ScreenPixelFormat {
        ScreenPixelFormat::Mono
    }

    fn get_rotation(&self) -> ScreenRotation {
        ScreenRotation::Normal
    }

    fn set_write_frame(
        &self,
        _x: usize,
        _y: usize,
        _width: usize,
        _height: usize,
    ) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn write(&self, _buffer: &'static mut [u8], _len: usize) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn write_continue(&self, buffer: &'static mut [u8], len: usize) -> Result<(), ErrorCode> {
        self.write(buffer, len)
    }

    fn set_client(&self, client: Option<&'static dyn ScreenClient>) {
        if let Some(client) = client {
            self.client.set(client);
        } else {
            self.client.clear();
        }
    }

    fn set_power(&self, _enable: bool) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn set_brightness(&self, _brightness: usize) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn set_invert(&self, _inverted: bool) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

}
