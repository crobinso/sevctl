// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

pub mod build;

use super::*;

use std::fs;

use uuid::Uuid;

#[derive(StructOpt)]
pub enum VmsaCmd {
    Build(build::CmdArgs),
}

// Linux struct vmcb_seg (arch/x86/include/asm/svm.h)
#[repr(C, packed)]
#[derive(Default)]
pub struct VmcbSegment {
    selector: u16,
    attrib: u16,
    limit: u32,
    base: u64,
}

#[derive(Default)]
pub struct Ovmf {
    _entries: Vec<String>, /* XXX: what is the type of this vector? */
}

impl Ovmf {
    fn load(&self, firmware: PathBuf) -> Result<()> {
        let _bytes = fs::read(firmware).context("error reading from firmware path file")?;

        let ovmf_table_footer_guid = Uuid::parse_str("96b582de-1fb2-45f7-baea-a366c55a082d")
            .context("error parsing uuid of OVMF_TABLE_FOOTER_GUID")?;

        let _expect = ovmf_table_footer_guid.to_u128_le();

        Ok(())
    }
}

#[derive(Default)]
pub struct SevInfoBlock(u32);

const ATTR_G_SHIFT: usize = 23;
const ATTR_B_SHIFT: usize = 22;
const ATTR_L_SHIFT: usize = 21;
const ATTR_AVL_SHIFT: usize = 20;
const ATTR_P_SHIFT: usize = 15;
const ATTR_DPL_SHIFT: usize = 13;
const ATTR_S_SHIFT: usize = 12;
const ATTR_TYPE_SHIFT: usize = 8;
const ATTR_A_SHIFT: usize = 8;
const ATTR_CS_SHIFT: usize = 11;
const ATTR_C_SHIFT: usize = 10;
const ATTR_R_SHIFT: usize = 9;
const ATTR_E_SHIFT: usize = 10;
const ATTR_W_SHIFT: usize = 9;

const ATTR_G_MASK: usize = 1 << ATTR_G_SHIFT;
const ATTR_B_MASK: usize = 1 << ATTR_B_SHIFT;
const ATTR_L_MASK: usize = 1 << ATTR_L_SHIFT;
const ATTR_AVL_MASK: usize = 1 << ATTR_AVL_SHIFT;
const ATTR_P_MASK: u16 = 1 << ATTR_P_SHIFT;
const ATTR_DPL_MASK: u16 = 1 << ATTR_DPL_SHIFT;
const ATTR_S_MASK: u16 = 1 << ATTR_S_SHIFT;
const ATTR_TYPE_MASK: u16 = 1 << ATTR_TYPE_SHIFT;
const ATTR_A_MASK: u16 = 1 << ATTR_A_SHIFT;
const ATTR_CS_MASK: u16 = 1 << ATTR_CS_SHIFT;
const ATTR_C_MASK: u16 = 1 << ATTR_C_SHIFT;
const ATTR_R_MASK: u16 = 1 << ATTR_R_SHIFT;
const ATTR_E_MASK: u16 = 1 << ATTR_E_SHIFT;
const ATTR_W_MASK: u16 = 1 << ATTR_W_SHIFT;

// Linux struct vmcb_save_area (arch/x86/include/asm/svm.h)
#[repr(C, packed)]
pub struct Vmsa {
    es: VmcbSegment,
    cs: VmcbSegment,
    ss: VmcbSegment,
    ds: VmcbSegment,
    fs: VmcbSegment,
    gs: VmcbSegment,
    gdtr: VmcbSegment,
    ldtr: VmcbSegment,
    idtr: VmcbSegment,
    tr: VmcbSegment,
    rsvd1: [u8; 43],
    cpl: u8,
    rsvd2: [u8; 4],
    efer: u64,
    rsvd3: [u8; 104],
    xss: u64, /* Valid for SEV-ES only */
    cr4: u64,
    cr3: u64,
    cr0: u64,
    dr7: u64,
    dr6: u64,
    rflags: u64,
    rip: u64,
    rsvd4: [u8; 88],
    rsp: u64,
    rsvd5: [u8; 24],
    rax: u64,
    star: u64,
    lstar: u64,
    cstar: u64,
    sfmask: u64,
    kernel_gs_base: u64,
    sysenter_cs: u64,
    sysenter_esp: u64,
    sysenter_eip: u64,
    cr2: u64,
    rsvd6: [u8; 32],
    g_pat: u64,
    dbgctl: u64,
    br_from: u64,
    br_to: u64,
    last_excp_from: u64,
    last_excp_to: u64,

    // The following part of the save area is valid only for
    // SEV-ES guests when referenced through the GHCB or for
    // saving to the host save area.
    rsvd7: [u8; 72],
    spec_ctrl: u32, /* Guest version of SPEC_CTRL at 0x2E0 */
    rsvd8: [u8; 4],
    pkru: u32,
    rsvd9: [u8; 20],
    rsvd10: u64, /* rax already available at 0x01f8 */
    rcx: u64,
    rdx: u64,
    rbx: u64,
    rsvd11: u64, /* rsp already available at 0x01d8 */
    rbp: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rsvd12: [u8; 16],
    sw_exit_code: u64,
    sw_exit_info_1: u64,
    sw_exit_info_2: u64,
    sw_scratch: u64,
    rsvd13: [u8; 56],
    xcr0: u64,
    valid_bitmap: [u8; 16],
    x87_state_gpa: u64,
}

impl Vmsa {
    fn init_amd64(&mut self) {
        self.cr0 = 1 << 4;
        self.rip = 0xfff0;

        self.cs.selector = 0xf000;
        self.cs.base = 0xffff0000;
        self.cs.limit = 0xffff;

        self.ds.limit = 0xffff;

        self.es.limit = 0xffff;
        self.fs.limit = 0xffff;
        self.gs.limit = 0xffff;
        self.ss.limit = 0xffff;

        self.gdtr.limit = 0xffff;
        self.idtr.limit = 0xffff;

        self.ldtr.limit = 0xffff;
        self.tr.limit = 0xffff;

        self.dr6 = 0xffff0ff0;
        self.dr7 = 0x0400;
        self.rflags = 0x2;
        self.xcr0 = 0x1;
    }

    fn init_kvm(&mut self) {
        // svm_set_cr4() sets guest X86_CR4_MCE bit if host
        // has X86_CR4_MCE enabled
        self.cr4 = 0x40;

        // svm_set_efer sets guest EFER_SVME (Secure Virtual Machine enable)
        self.efer = 0x1000;

        // init_vmcb + init_sys_seg() sets
        // SVM_SELECTOR_P_MASK | SEG_TYPE_LDT
        self.ldtr.attrib = 0x0082;

        // init_vmcb + init_sys_seg() sets
        // SVM_SELECTOR_P_MASK | SEG_TYPE_BUSY_TSS16
        self.tr.attrib = 0x0083;

        // kvm_arch_vcpu_create() in arch/x86/kvm/x86.c
        self.g_pat = 0x0007040600070406;
    }

    // Based on logic in setup_regs() (src/arch/src/x86_64/regs.rs)
    fn init_krun(&mut self, cpu: u64) {
        self.rsi = 0x7000;
        self.rbp = 0x8ff0;
        self.rsp = 0x8ff0;

        // Doesn't match with configure_segments_and_sregs
        self.cs.attrib =
            (ATTR_P_MASK | ATTR_S_MASK | ATTR_CS_MASK | ATTR_R_MASK) >> ATTR_TYPE_SHIFT;
        self.ds.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.es.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.ss.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK) >> ATTR_TYPE_SHIFT;
        self.fs.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.gs.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;

        if cpu > 0 {
            // XXX where is this done ?
            self.rip = 0;
            self.rsp = 0;
            self.rbp = 0;
            self.rsi = 0;

            self.cs.selector = 0x9100;
            self.cs.base = 0x91000;
        }
    }

    // Based on logic in x86_cpu_reset() (target/i386/cpu.c)
    fn init_qemu(&mut self, _cpu: u64) {
        self.ldtr.attrib = (ATTR_P_MASK | (2 << ATTR_TYPE_SHIFT)) >> ATTR_TYPE_SHIFT;
        self.tr.attrib = (ATTR_P_MASK | (11 << ATTR_TYPE_SHIFT)) >> ATTR_TYPE_SHIFT;
        self.cs.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_CS_MASK | ATTR_R_MASK | ATTR_A_MASK)
            >> ATTR_TYPE_SHIFT;
        self.ds.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.es.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.ss.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.fs.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;
        self.gs.attrib = (ATTR_P_MASK | ATTR_S_MASK | ATTR_W_MASK | ATTR_A_MASK) >> ATTR_TYPE_SHIFT;

        self.g_pat = 0x0007040600070406;
    }

    fn cpu_sku(&mut self, mut family: u64, mut model: u64, mut stepping: u64) {
        stepping &= 0xf;
        model &= 0xff;
        family &= 0xfff;

        self.rdx = stepping;

        if family > 0xf {
            self.rdx |= 0xf00 | ((family - 0x0f) << 20);
        } else {
            self.rdx |= family << 8;
        }

        self.rdx |= ((model & 0xf) << 4) | ((model >> 4) << 16);
    }
}

impl Default for Vmsa {
    fn default() -> Self {
        Self {
            es: VmcbSegment::default(),
            cs: VmcbSegment::default(),
            ss: VmcbSegment::default(),
            ds: VmcbSegment::default(),
            fs: VmcbSegment::default(),
            gs: VmcbSegment::default(),
            gdtr: VmcbSegment::default(),
            ldtr: VmcbSegment::default(),
            idtr: VmcbSegment::default(),
            tr: VmcbSegment::default(),
            rsvd1: [0; 43],
            cpl: 0,
            rsvd2: [0; 4],
            efer: 0,
            rsvd3: [0; 104],
            xss: 0,
            cr4: 0,
            cr3: 0,
            cr0: 0,
            dr7: 0,
            dr6: 0,
            rflags: 0,
            rip: 0,
            rsvd4: [0; 88],
            rsp: 0,
            rsvd5: [0; 24],
            rax: 0,
            star: 0,
            lstar: 0,
            cstar: 0,
            sfmask: 0,
            kernel_gs_base: 0,
            sysenter_cs: 0,
            sysenter_esp: 0,
            sysenter_eip: 0,
            cr2: 0,
            rsvd6: [0; 32],
            g_pat: 0,
            dbgctl: 0,
            br_from: 0,
            br_to: 0,
            last_excp_from: 0,
            last_excp_to: 0,
            rsvd7: [0; 72],
            spec_ctrl: 0,
            rsvd8: [0; 4],
            pkru: 0,
            rsvd9: [0; 20],
            rsvd10: 0,
            rcx: 0,
            rdx: 0,
            rbx: 0,
            rsvd11: 0,
            rbp: 0,
            rsi: 0,
            rdi: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsvd12: [0; 16],
            sw_exit_code: 0,
            sw_exit_info_1: 0,
            sw_exit_info_2: 0,
            sw_scratch: 0,
            rsvd13: [0; 56],
            xcr0: 0,
            valid_bitmap: [0; 16],
            x87_state_gpa: 0,
        }
    }
}
