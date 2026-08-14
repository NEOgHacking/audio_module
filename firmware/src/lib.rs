#![no_std]

use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    registry::Registry,
};

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

       
        let pio_access = match bus_allocator.request_pio(&gpio_bank.gpio7) {
            Some(access) => access,
            None => {
                bus_allocator.release_dma(dma);
                return Err(DriverError::InitFailed);
            }
        };

      
        let audio = with_pio!(pio_access, common, sm, {
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
    common: &mut Common<'static, PIO>,
    sm: StateMachine<'static, PIO, SM>,
    dma: Peri<'static, DMA_CH0>,
    data: Peri<'static, Data>,
    bclk: Peri<'static, Bclk>,
    lrclk: Peri<'static, Lrclk>,
) -> AudioOut
where
    PIO: Instance + Send + 'static,
    Data: PioPin + 'static,
    Bclk: PioPin + 'static,
    Lrclk: PioPin + 'static,
{
    // TS is from xpanse_api::reexports::embassy_rp::PioI2sOutProgram
    let program = PioI2sOutProgram::new(common);

    let i2s = PioI2sOut::new(
        common,
        sm,
        dma,
        DmaIrqs,
        data,
        bclk,
        lrclk,
        SAMPLE_RATE,
        BIT_DEPTH,
        &program,
    );

    let mut output = PioAudioOut {
        i2s,
        _program: program,
    };

    let mut output = Output::new(gpio_bank.gpio5, Level::Low);

    output.set_low();

    output.start();

    AudioOut::new(output)
}