/*
REGIONS:

Instruction Memory:
    - Boot ROM

Data Memory:
    - Heap
    - Stack
    - BSS section
    - Read-Only Data (.rodata)
    - Shared Memory (IPC)

IO:
    - KeyboardBuffer
    - MouseBuffer
    - FrameBuffer
    - SpeakerBuffer
    - USB Device A
    - USB Device B
    - Network Card
    - Serial Port (UART)
    - Disk Controller
    - RTC / NVRAM
    - GPIO Bank

CONTROL:
    - Interrupt Vector Table
    - Interrupt Controller
    - Timer / Counter Unit
    - Watchdog Timer
    - DMA Controller
    - Power / Reset Controller
    - CSR / Status Register Bank

DEBUG:
    - Debug/Trace Output Port
    - Performance Counter Region
 */

#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub start: u64,
    pub end: u64,   // exclusive
    pub size: u64,
}

impl Region {
    fn new(start: u64, size: u64) -> Self {
        Self { start, end: start + size, size }
    }

    fn zero() -> Self {
        Self { start: 0, end: 0, size: 0 }
    }
}

pub struct Memory {
    data: Box<[u8]>,
    size: u64,

    // Instruction Memory
    pub boot_rom: Region,
    pub imem: Region,

    // Data Memory
    pub heap: Region,
    pub stack: Region,
    pub bss: Region,
    pub rodata: Region,
    pub mmu_table: Region,
    pub shared_mem: Region,

    // IO
    pub keyboard_buffer: Region,
    pub mouse_buffer: Region,
    pub frame_buffer: Region,
    pub speaker_buffer: Region,
    pub usb_a: Region,
    pub usb_b: Region,
    pub network_card: Region,
    pub uart: Region,
    pub disk_controller: Region,
    pub rtc: Region,
    pub gpio: Region,

    // Control
    pub ivt: Region,
    pub interrupt_controller: Region,
    pub timer: Region,
    pub watchdog: Region,
    pub dma: Region,
    pub power_reset: Region,
    pub csr: Region,

    // Debug
    pub debug_port: Region,
    pub perf_counters: Region,
}

impl Memory {
    /// Minimal default: 2GB total, only the essentials get real space.
    /// Everything else is a zero-size (unused) region.
    pub fn default() -> Self {
        MemoryBuilder::new()
            .imem(256 * 1024 * 1024)
            .heap(1024 * 1024 * 1024)
            .stack(512 * 1024 * 1024)
            .bss(128 * 1024 * 1024)
            .frame_buffer(64 * 1024 * 1024)
            .keyboard_buffer(4 * 1024)
            .build()
    }

    #[inline(always)]
    pub fn read_byte(&self, addr: u64) -> u8 {
        self.data[addr as usize]
    }

    #[inline(always)]
    pub fn read_word(&self, addr: u64) -> u16 {
        self.data[addr as usize] as u16 | (self.data[addr as usize + 1] as u16) << 8
    }

    #[inline(always)]
    pub fn read_dword(&self, addr: u64) -> u32 {
        self.read_word(addr) as u32 | (self.read_word(addr + 2) as u32) << 16
    }

    #[inline(always)]
    pub fn read_long(&self, addr: u64) -> u64 {
        self.read_dword(addr) as u64 | (self.read_dword(addr + 4) as u64) << 32
    }

    #[inline(always)]
    pub fn read_bytes(&self, addr: u64, len: usize) -> &[u8] {
        &self.data[addr as usize..addr as usize + len]
    }
}

/// Builder for `Memory`. Every region defaults to size 0 (unmapped).
/// Call only the setters you need; layout order is fixed regardless
/// of the order you call the setters in.
#[derive(Default)]
pub struct MemoryBuilder {
    boot_rom: u64,
    imem: u64,

    heap: u64,
    stack: u64,
    bss: u64,
    rodata: u64,
    mmu_table: u64,
    shared_mem: u64,

    keyboard_buffer: u64,
    mouse_buffer: u64,
    frame_buffer: u64,
    speaker_buffer: u64,
    usb_a: u64,
    usb_b: u64,
    network_card: u64,
    uart: u64,
    disk_controller: u64,
    rtc: u64,
    gpio: u64,

    ivt: u64,
    interrupt_controller: u64,
    timer: u64,
    watchdog: u64,
    dma: u64,
    power_reset: u64,
    csr: u64,

    debug_port: u64,
    perf_counters: u64,
}

macro_rules! setter {
    ($name:ident) => {
        pub fn $name(mut self, size: u64) -> Self {
            self.$name = size;
            self
        }
    };
}

impl MemoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    setter!(boot_rom);
    setter!(imem);

    setter!(heap);
    setter!(stack);
    setter!(bss);
    setter!(rodata);
    setter!(mmu_table);
    setter!(shared_mem);

    setter!(keyboard_buffer);
    setter!(mouse_buffer);
    setter!(frame_buffer);
    setter!(speaker_buffer);
    setter!(usb_a);
    setter!(usb_b);
    setter!(network_card);
    setter!(uart);
    setter!(disk_controller);
    setter!(rtc);
    setter!(gpio);

    setter!(ivt);
    setter!(interrupt_controller);
    setter!(timer);
    setter!(watchdog);
    setter!(dma);
    setter!(power_reset);
    setter!(csr);

    setter!(debug_port);
    setter!(perf_counters);

    /// Lays every requested region out back-to-back, in fixed order,
    /// and allocates the backing buffer.
    pub fn build(self) -> Memory {
        let mut offset = 0u64;

        macro_rules! next_region {
            ($field:expr) => {{
                let r = Region::new(offset, $field);
                offset += $field;
                r
            }};
        }

        let boot_rom = next_region!(self.boot_rom);
        let imem = next_region!(self.imem);

        let heap = next_region!(self.heap);
        let stack = next_region!(self.stack);
        let bss = next_region!(self.bss);
        let rodata = next_region!(self.rodata);
        let mmu_table = next_region!(self.mmu_table);
        let shared_mem = next_region!(self.shared_mem);

        let keyboard_buffer = next_region!(self.keyboard_buffer);
        let mouse_buffer = next_region!(self.mouse_buffer);
        let frame_buffer = next_region!(self.frame_buffer);
        let speaker_buffer = next_region!(self.speaker_buffer);
        let usb_a = next_region!(self.usb_a);
        let usb_b = next_region!(self.usb_b);
        let network_card = next_region!(self.network_card);
        let uart = next_region!(self.uart);
        let disk_controller = next_region!(self.disk_controller);
        let rtc = next_region!(self.rtc);
        let gpio = next_region!(self.gpio);

        let ivt = next_region!(self.ivt);
        let interrupt_controller = next_region!(self.interrupt_controller);
        let timer = next_region!(self.timer);
        let watchdog = next_region!(self.watchdog);
        let dma = next_region!(self.dma);
        let power_reset = next_region!(self.power_reset);
        let csr = next_region!(self.csr);

        let debug_port = next_region!(self.debug_port);
        let perf_counters = next_region!(self.perf_counters);

        let total = offset;

        Memory {
            data: vec![0u8; total as usize].into_boxed_slice(),
            size: total,

            boot_rom,
            imem,

            heap,
            stack,
            bss,
            rodata,
            mmu_table,
            shared_mem,

            keyboard_buffer,
            mouse_buffer,
            frame_buffer,
            speaker_buffer,
            usb_a,
            usb_b,
            network_card,
            uart,
            disk_controller,
            rtc,
            gpio,

            ivt,
            interrupt_controller,
            timer,
            watchdog,
            dma,
            power_reset,
            csr,

            debug_port,
            perf_counters,
        }
    }
}