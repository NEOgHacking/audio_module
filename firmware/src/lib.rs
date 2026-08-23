#![no_std]

use defmt_rtt as _;
use embassy_rp::{Peri, PeripheralType, pio::PioPin};
use panic_probe as _;

use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    registry::Registry,
};

pub struct I2sAudioDriver;

// Hackxpansion PCM5102A connection for this module:
//   GPIO3 -> DIN (audio data)
//   GPIO4 -> BCK (bit clock)
//   GPIO2 -> LRCK (left/right clock)
impl DriverMeta for I2sAudioDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K,
        md1: ModuleDetectResistor::R2K,
    };
}

impl<G: BankPins> Driver<G> for I2sAudioDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        _bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {
        let audio = AudioOut::new(
            gpio_bank.gpio3,
            gpio_bank.gpio4,
            gpio_bank.gpio2,
        );

        registry.register(slot, Self::ID, audio);
        Ok(())
    }
}

pub struct AudioOut<Data, Bclk, Lrclk>
where
    Data: PeripheralType + PioPin,
    Bclk: PeripheralType + PioPin,
    Lrclk: PeripheralType + PioPin,
{
    data: Peri<'static, Data>,
    bclk: Peri<'static, Bclk>,
    lrclk: Peri<'static, Lrclk>,
}

impl<Data, Bclk, Lrclk> AudioOut<Data, Bclk, Lrclk>
where
    Data: PeripheralType + PioPin,
    Bclk: PeripheralType + PioPin,
    Lrclk: PeripheralType + PioPin,
{
    pub fn new(data: Peri<'static, Data>, bclk: Peri<'static, Bclk>, lrclk: Peri<'static, Lrclk>) -> Self {
        Self { data, bclk, lrclk }
    }

    pub fn data_pin(&self) -> &Peri<'static, Data> {
        &self.data
    }

    pub fn bclk_pin(&self) -> &Peri<'static, Bclk> {
        &self.bclk
    }

    pub fn lrclk_pin(&self) -> &Peri<'static, Lrclk> {
        &self.lrclk
    }
}