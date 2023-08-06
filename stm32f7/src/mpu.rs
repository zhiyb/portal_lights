use stm32f7::stm32f7x2::CorePeripherals;

pub enum MpuAp {
    /// MPU Access Permission no access
    None = 0,
    /// MPU Access Permission privileged access only
    Priv = 1,
    /// MPU Access Permission unprivileged access read-only
    URO  = 2,
    /// MPU Access Permission full access
    Full = 3,
    /// MPU Access Permission privileged access read-only
    PRO  = 5,
    /// MPU Access Permission read-only access
    RO   = 6,
}

pub enum MpuSize {
    /// MPU Region Size 32 Bytes
    Size32B   = 0x04,
    /// MPU Region Size 64 Bytes
    Size64B   = 0x05,
    /// MPU Region Size 128 Bytes
    Size128B  = 0x06,
    /// MPU Region Size 256 Bytes
    Size256B  = 0x07,
    /// MPU Region Size 512 Bytes
    Size512B  = 0x08,
    /// MPU Region Size 1 KByte
    Size1KB   = 0x09,
    /// MPU Region Size 2 KBytes
    Size2KB   = 0x0A,
    /// MPU Region Size 4 KBytes
    Size4KB   = 0x0B,
    /// MPU Region Size 8 KBytes
    Size8KB   = 0x0C,
    /// MPU Region Size 16 KBytes
    Size16KB  = 0x0D,
    /// MPU Region Size 32 KBytes
    Size32KB  = 0x0E,
    /// MPU Region Size 64 KBytes
    Size64KB  = 0x0F,
    /// MPU Region Size 128 KBytes
    Size128KB = 0x10,
    /// MPU Region Size 256 KBytes
    Size256KB = 0x11,
    /// MPU Region Size 512 KBytes
    Size512KB = 0x12,
    /// MPU Region Size 1 MByte
    Size1MB   = 0x13,
    /// MPU Region Size 2 MBytes
    Size2MB   = 0x14,
    /// MPU Region Size 4 MBytes
    Size4MB   = 0x15,
    /// MPU Region Size 8 MBytes
    Size8MB   = 0x16,
    /// MPU Region Size 16 MBytes
    Size16MB  = 0x17,
    /// MPU Region Size 32 MBytes
    Size32MB  = 0x18,
    /// MPU Region Size 64 MBytes
    Size64MB  = 0x19,
    /// MPU Region Size 128 MBytes
    Size128MB = 0x1A,
    /// MPU Region Size 256 MBytes
    Size256MB = 0x1B,
    /// MPU Region Size 512 MBytes
    Size512MB = 0x1C,
    /// MPU Region Size 1 GByte
    Size1GB   = 0x1D,
    /// MPU Region Size 2 GBytes
    Size2GB   = 0x1E,
    /// MPU Region Size 4 GBytes
    Size4GB   = 0x1F,
}

pub struct MpuRegion {
    rbar: u32,
    rasr: u32,
}

pub fn mpu_region(
    region: u32, base_address: u32,
    disable_exec: bool, access_permission: MpuAp, type_ext_field: u8,
    is_shareable: bool, is_cacheable: bool, is_bufferable: bool,
    sub_region_disable: u8, size: MpuSize,
) -> MpuRegion {
    return MpuRegion {
        rbar: (base_address & 0xffffffe0)
            | (1 << 4)
            | region,
        rasr: ((disable_exec as u32) << 28)
            | ((access_permission as u32) << 24)
            | ((type_ext_field as u32) << 19)
            | ((is_shareable as u32) << 18)
            | ((is_cacheable as u32) << 17)
            | ((is_bufferable as u32) << 16)
            | ((sub_region_disable as u32) << 8)
            | ((size as u32) << 1)
            | 1,
    };
}

pub fn mpu_init(cp: &mut CorePeripherals) {
    // Find special region addresses from linker script
    extern "C" {
        // Symbol from linker script
        static mut _dmaram_start: u8;
        static mut _dmaram_size: u8;
    }
    let dmaram_start = unsafe {(&_dmaram_start as *const u8) as u32};
    let dmaram_size = unsafe {(&_dmaram_size as *const u8) as u32};
    assert_eq!(dmaram_size, 16 * 1024, "Expecting DMA RAM region is 16KiB, but got {}", dmaram_size);

	// MPU region definitions
    let mpu_regions = [
		// Disable write cache on DMA RAM
        mpu_region(0, dmaram_start,
            true, MpuAp::Priv, 0b000,
            true, false, true,
            0x00, MpuSize::Size16KB),
    ];

    unsafe {
        // Update MPU regions
        for r in mpu_regions {
            cp.MPU.rbar.write(r.rbar);
            cp.MPU.rasr.write(r.rasr);
        }
        // Enable MPU with fallback to default memory map
        cp.MPU.ctrl.write(0b101);
    };
}
