use crate::irq::IrqHandler;
use axplat_dyn::driver::intc::*;
use axplat_dyn::mem::cpu_idx_to_id;
/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 2048;

static mut IRQ_CHIP: u64 = 0;

#[cfg(feature = "ipi")]
pub const IPI_IRQ_NUM: usize = 0;

pub(crate) unsafe fn init() {
    let chip = axplat_dyn::driver::get_dev!(Intc).unwrap();
    unsafe { IRQ_CHIP = (chip.descriptor.device_id).into() };

    #[cfg(target_arch = "aarch64")]
    {
        cpu_interface().set_eoi_mode(true);
    }

    crate::time::enable_irq();
}

#[cfg(feature = "smp")]
pub(crate) unsafe fn init_secondary() {
    #[cfg(target_arch = "aarch64")]
    {
        cpu_interface().set_eoi_mode(true);
    }

    crate::time::enable_irq();
}

pub(crate) fn cpu_interface() -> &'static local::Boxed {
    axplat_dyn::irq::interface(unsafe { IRQ_CHIP }.into()).expect("no cpu interface")
}

fn modify_chip<F: Fn(&mut Boxed)>(f: F) {
    let mut g = axplat_dyn::driver::get_dev!(Intc)
        .unwrap()
        .spin_try_borrow_by(0.into())
        .unwrap();
    (f)(&mut g);
}

/// Enables or disables the given IRQ.
pub fn set_enable(irq: IrqConfig, enabled: bool) {
    // ArceOS cpu_id is actually cpu_idx
    let cpu_idx = crate::cpu::this_cpu_id();

    trace!("cpu[{:?}] Irq set enable: {:?} {}", cpu_idx, irq, enabled);

    if irq.is_private {
        if let local::Capability::ConfigLocalIrq(cpu) = cpu_interface().capability() {
            if enabled {
                cpu.set_trigger(irq.irq, irq.trigger).unwrap();
                cpu.irq_enable(irq.irq).unwrap();
            } else {
                cpu.irq_disable(irq.irq).unwrap();
            }
            return;
        }
    }

    let cpu_hard_id = cpu_idx_to_id(cpu_idx.into());

    modify_chip(|c| {
        if enabled {
            c.set_target_cpu(irq.irq, cpu_hard_id.raw().into()).unwrap();
            c.set_trigger(irq.irq, irq.trigger).unwrap();
            c.irq_enable(irq.irq).unwrap();
        } else {
            c.irq_disable(irq.irq).unwrap();
        }
    });
}

/// Registers an IRQ handler for the given IRQ.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(irq_config: IrqConfig, handler: IrqHandler) -> bool {
    debug!("register handler irq {:?}", irq_config);
    crate::irq::register_handler_common(irq_config, handler)
}

/// Dispatches the IRQ.
///
/// This function is called by the common interrupt handler. It looks
/// up in the IRQ handler table and calls the corresponding handler. If
/// necessary, it also acknowledges the interrupt controller after handling.
pub fn dispatch_irq(irq_no: usize) {
    let icc = cpu_interface();
    let intid = if irq_no == 0 {
        match icc.ack() {
            Some(v) => v,
            None => return,
        }
    } else {
        axplat_dyn::driver::IrqId::from(irq_no)
    };
    crate::irq::dispatch_irq_common(intid.into());
    icc.eoi(intid);
    if icc.get_eoi_mode() {
        icc.dir(intid);
    }
}

pub fn fetch_irq() -> usize {
    let icc = cpu_interface();
    icc.ack().map(|o| o.into()).unwrap_or_default()
}

/// Reads and returns the value of the given aarch64 system register.
macro_rules! read_sysreg {
    ($name:ident) => {
        {
            let mut value: u64;
            unsafe{::core::arch::asm!(
                concat!("mrs {value:x}, ", ::core::stringify!($name)),
                value = out(reg) value,
                options(nomem, nostack),
            );}
            value
        }
    }
}

/// Writes the given value to the given aarch64 system register.
macro_rules! write_sysreg {
    ($name:ident, $value:expr) => {
        {
            let v: u64 = $value;
            unsafe{::core::arch::asm!(
                concat!("msr ", ::core::stringify!($name), ", {value:x}"),
                value = in(reg) v,
                options(nomem, nostack),
            )}
        }
    }
}

fn read_lr(id: usize) -> u64 {
    let id = id as u64;
    match id {
        //TODO get lr size from gic reg
        0 => read_sysreg!(ich_lr0_el2),
        1 => read_sysreg!(ich_lr1_el2),
        2 => read_sysreg!(ich_lr2_el2),
        3 => read_sysreg!(ich_lr3_el2),
        4 => read_sysreg!(ich_lr4_el2),
        5 => read_sysreg!(ich_lr5_el2),
        6 => read_sysreg!(ich_lr6_el2),
        7 => read_sysreg!(ich_lr7_el2),
        8 => read_sysreg!(ich_lr8_el2),
        9 => read_sysreg!(ich_lr9_el2),
        10 => read_sysreg!(ich_lr10_el2),
        11 => read_sysreg!(ich_lr11_el2),
        12 => read_sysreg!(ich_lr12_el2),
        13 => read_sysreg!(ich_lr13_el2),
        14 => read_sysreg!(ich_lr14_el2),
        15 => read_sysreg!(ich_lr15_el2),
        _ => {
            panic!("invalid lr id {}", id);
        }
    }
}

fn write_lr(id: usize, val: u64) {
    let id = id as u64;
    match id {
        0 => write_sysreg!(ich_lr0_el2, val),
        1 => write_sysreg!(ich_lr1_el2, val),
        2 => write_sysreg!(ich_lr2_el2, val),
        3 => write_sysreg!(ich_lr3_el2, val),
        4 => write_sysreg!(ich_lr4_el2, val),
        5 => write_sysreg!(ich_lr5_el2, val),
        6 => write_sysreg!(ich_lr6_el2, val),
        7 => write_sysreg!(ich_lr7_el2, val),
        8 => write_sysreg!(ich_lr8_el2, val),
        9 => write_sysreg!(ich_lr9_el2, val),
        10 => write_sysreg!(ich_lr10_el2, val),
        11 => write_sysreg!(ich_lr11_el2, val),
        12 => write_sysreg!(ich_lr12_el2, val),
        13 => write_sysreg!(ich_lr13_el2, val),
        14 => write_sysreg!(ich_lr14_el2, val),
        15 => write_sysreg!(ich_lr15_el2, val),
        _ => {
            panic!("invalid lr id {}", id);
        }
    }
}

#[cfg(feature = "hv")]
pub fn inject_interrupt(vector: usize) {
    // mask
    const LR_VIRTIRQ_MASK: usize = (1 << 32) - 1;

    let elsr: u64 = read_sysreg!(ich_elrsr_el2);
    let vtr = read_sysreg!(ich_vtr_el2) as usize;
    let lr_num: usize = (vtr & 0xf) + 1;
    let mut free_lr = -1 as isize;
    for i in 0..lr_num {
        // find a free list register
        if (1 << i) & elsr > 0 {
            if free_lr == -1 {
                free_lr = i as isize;
            }
            continue;
        }
        let lr_val = read_lr(i) as usize;
        // if a virtual interrupt is enabled and equals to the physical interrupt irq_id
        if (lr_val & LR_VIRTIRQ_MASK) == vector {
            trace!("virtual irq {} enables again", vector);
        }
    }
    trace!("use free lr {} to inject irq {}", free_lr, vector);

    if free_lr == -1 {
        panic!("No free list register to inject IRQ {}", vector);
    } else {
        let mut val = vector as u64; // vector
        val |= 1 << 60; // group 1
        val |= 1 << 62; // state pending
        // hardware interrupt not supported
        write_lr(free_lr as usize, val);
    }
}

fn send_sgi_inner(aff3: u8, aff2: u8, aff1: u8, target: u8, vector: usize, to_all: bool) {
    let value = 
        ((vector & 0xF) << 24) |            // vector
        (1 << target) |                     // target bitmap
        ((aff1 as usize) << 16) |           // affinity level 1
        ((aff2 as usize) << 32) |           // affinity level 2
        ((aff3 as usize) << 48) |           // affinity level 3
        ((to_all as usize) << 40);          // interrupt routing mode

    write_sysreg!(icc_sgi1r_el1, value as _);
}

/// Sends Software Generated Interrupt (SGI)(s) (usually IPI) to the given dest CPU.
pub fn send_sgi_one(dest: usize, vector: usize) {
    // the default affinity scheme is 0.0.0.x
    send_sgi_inner(0, 0, 0, dest as _, vector, false);
}

/// Sends a broadcast IPI to all CPUs.
pub fn send_sgi_all(vector: usize) {
    send_sgi_inner(0, 0, 0, 0, vector, true);
}