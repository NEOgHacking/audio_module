#![no_std]



use defmt_rtt as _;
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::Peri;
use embassy_rp::pio::{InterruptHandler, Common, StateMachine, PioPin, Instance};
use embassy_rp::{bind_interrupts, dma};
use panic_probe as _;

use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    registry::Registry,
};


// const SAMPLE_RATE: u32 = 48_000;
// const BIT_DEPTH: u32 = 16;


bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

// Each driver is a struct, that impls the Driver and DriverMeta trait
//
// These structs can contain channels, or hardware that the driver owns,
// or buses that the hardware owns
pub struct I2sAudioDriver;

// This trait allows the main firmware to know when should it load this driver,
// on what resistor combination
impl DriverMeta for I2sAudioDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K,
        md1: ModuleDetectResistor::R2K,
    };
}

// This is where the actual business logic happens
impl<G: BankPins> Driver<G> for I2sAudioDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {
        let dma = bus_allocator
            .request_dma::<DMA_CH0>()
            .map_err(|_| DriverError::InitFailed)?;

       
        let pio_access = match bus_allocator.request_pio(&[&gpio_bank.gpio7]) {
            Some(access) => access,
            None => {
                bus_allocator.release_dma(dma);
                return Err(DriverError::InitFailed);
            }
        };

      
        let audio = xpanse_api::with_pio!(pio_access, common, sm, {
            build_audio_out(
                common,
                sm,
                dma,
                gpio_bank.gpio3,
                gpio_bank.gpio4,
                gpio_bank.gpio2,
            )
        });

        registry.register(slot, Self::ID, audio);

        Ok(())
    }

}

fn build_audio_out<PIO, const SM: usize, Data, Bclk, Lrclk>(
    _common: &mut Common<'static, PIO>,
    _sm: StateMachine<'static, PIO, SM>,
    _dma: Peri<'static, DMA_CH0>,
    _data: Peri<'static, Data>,
    _bclk: Peri<'static, Bclk>,
    _lrclk: Peri<'static, Lrclk>,
) -> AudioOut
where
    PIO: Instance + Send + 'static,
    Data: PioPin + 'static,
    Bclk: PioPin + 'static,
    Lrclk: PioPin + 'static,
{
    // Minimal stub implementation: return a placeholder AudioOut so the
    // driver can register a capability while the full I2S implementation
    // is worked on.
    let output = PioAudioOut {};
    AudioOut::new(output)
}

// Minimal placeholder types to satisfy the driver's build and registration.
// These should be replaced by the real audio wrapper when available.
pub struct DmaIrqs;

pub struct PioAudioOut {}

pub struct AudioOut {
    _inner: PioAudioOut,
}

impl AudioOut {
    pub fn new(inner: PioAudioOut) -> Self {
        Self { _inner: inner }
    }
}