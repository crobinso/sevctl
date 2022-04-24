// SPDX-License-Identifier: Apache-2.0

use crate::Vmsa;

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CmdArgs {
    #[structopt(parse(from_os_str), help = "VMSA file to show")]
    pub filename: PathBuf,

    // XXX --zeroes=show|skip|squash
    //#[structopt(bool, help = "Print field")]
    //pub cpu: u64,
}

fn format_u16_bitmask(value: u16) -> String {
    format!("{:#06x} ({:016b})", value, value)
}

fn format_u64_bitmask(value: u64) -> String {
    format!("{:#018x} ({:064b})", value, value)
}

fn format_vmsa(vmsa: Vmsa) -> String {
    let string_list = [
        format!("{:20}: {:#06x}", "es_selector", {vmsa.es.selector}),
        format!("{:20}: {}", "es_attrib", format_u16_bitmask(vmsa.es.attrib)),
        format!("{:20}: {:#010x}", "es_limit", {vmsa.es.limit}),
        format!("{:20}: {:#018x}", "es_base", {vmsa.es.base}),

        format!("{:20}: {:#06x}", "cs_selector", {vmsa.cs.selector}),
        format!("{:20}: {}", "cs_attrib", format_u16_bitmask(vmsa.cs.attrib)),
        format!("{:20}: {:#010x}", "cs_limit", {vmsa.cs.limit}),
        format!("{:20}: {:#018x}", "cs_base", {vmsa.cs.base}),

        format!("{:20}: {:#06x}", "ss_selector", {vmsa.ss.selector}),
        format!("{:20}: {}", "ss_attrib", format_u16_bitmask(vmsa.ss.attrib)),
        format!("{:20}: {:#010x}", "ss_limit", {vmsa.ss.limit}),
        format!("{:20}: {:#018x}", "ss_base", {vmsa.ss.base}),

        format!("{:20}: {:#06x}", "ds_selector", {vmsa.ds.selector}),
        format!("{:20}: {}", "ds_attrib", format_u16_bitmask(vmsa.ds.attrib)),
        format!("{:20}: {:#010x}", "ds_limit", {vmsa.ds.limit}),
        format!("{:20}: {:#018x}", "ds_base", {vmsa.ds.base}),

        format!("{:20}: {:#06x}", "fs_selector", {vmsa.fs.selector}),
        format!("{:20}: {}", "fs_attrib", format_u16_bitmask(vmsa.fs.attrib)),
        format!("{:20}: {:#010x}", "fs_limit", {vmsa.fs.limit}),
        format!("{:20}: {:#018x}", "fs_base", {vmsa.fs.base}),

        format!("{:20}: {:#06x}", "gs_selector", {vmsa.gs.selector}),
        format!("{:20}: {}", "gs_attrib", format_u16_bitmask(vmsa.gs.attrib)),
        format!("{:20}: {:#010x}", "gs_limit", {vmsa.gs.limit}),
        format!("{:20}: {:#018x}", "gs_base", {vmsa.gs.base}),

        format!("{:20}: {:#06x}", "gdtr_selector", {vmsa.gdtr.selector}),
        format!("{:20}: {}", "gdtr_attrib", format_u16_bitmask(vmsa.gdtr.attrib)),
        format!("{:20}: {:#010x}", "gdtr_limit", {vmsa.gdtr.limit}),
        format!("{:20}: {:#018x}", "gdtr_base", {vmsa.gdtr.base}),

        format!("{:20}: {:#06x}", "ldtr_selector", {vmsa.ldtr.selector}),
        format!("{:20}: {}", "ldtr_attrib", format_u16_bitmask(vmsa.ldtr.attrib)),
        format!("{:20}: {:#010x}", "ldtr_limit", {vmsa.ldtr.limit}),
        format!("{:20}: {:#018x}", "ldtr_base", {vmsa.ldtr.base}),

        format!("{:20}: {:#06x}", "idtr_selector", {vmsa.idtr.selector}),
        format!("{:20}: {}", "idtr_attrib", format_u16_bitmask(vmsa.idtr.attrib)),
        format!("{:20}: {:#010x}", "idtr_limit", {vmsa.idtr.limit}),
        format!("{:20}: {:#018x}", "idtr_base", {vmsa.idtr.base}),

        format!("{:20}: {:#06x}", "tr_selector", {vmsa.tr.selector}),
        format!("{:20}: {}", "tr_attrib", format_u16_bitmask(vmsa.tr.attrib)),
        format!("{:20}: {:#010x}", "tr_limit", {vmsa.tr.limit}),
        format!("{:20}: {:#018x}", "tr_base", {vmsa.tr.base}),

        format!("{:20}: {:?}", "reserved_1", {vmsa.rsvd1}),

        format!("{:20}: {:#x}", "cpl", {vmsa.cpl}),

        format!("{:20}: {:?}", "reserved_2", {vmsa.rsvd2}),

        format!("{:20}: {:#018x}", "efer", {vmsa.efer}),

        format!("{:20}: {:?}", "reserved_3", {vmsa.rsvd3}),

        format!("{:20}: {:#018x}", "xss", {vmsa.xss}),
        format!("{:20}: {:#018x}", "cr4", {vmsa.cr4}),
        format!("{:20}: {:#018x}", "cr3", {vmsa.cr3}),
        format!("{:20}: {:#018x}", "cr0", {vmsa.cr0}),
        format!("{:20}: {:#018x}", "dr7", {vmsa.dr7}),
        format!("{:20}: {:#018x}", "dr6", {vmsa.dr6}),
        format!("{:20}: {:#018x}", "rflags", {vmsa.rflags}),
        format!("{:20}: {:#018x}", "rip", {vmsa.rip}),

        format!("{:20}: {:?}", "reserved_4", {vmsa.rsvd4}),

        format!("{:20}: {:#018x}", "rsp", {vmsa.rsp}),

        format!("{:20}: {:?}", "reserved_5", {vmsa.rsvd5}),

        format!("{:20}: {:#018x}", "rax", {vmsa.rax}),
        format!("{:20}: {:#018x}", "star", {vmsa.star}),
        format!("{:20}: {:#018x}", "lstar", {vmsa.lstar}),
        format!("{:20}: {:#018x}", "cstar", {vmsa.cstar}),
        format!("{:20}: {:#018x}", "sfmask", {vmsa.sfmask}),
        format!("{:20}: {:#018x}", "kernel_gs_base", {vmsa.kernel_gs_base}),
        format!("{:20}: {:#018x}", "sysenter_cs", {vmsa.sysenter_cs}),
        format!("{:20}: {:#018x}", "sysenter_esp", {vmsa.sysenter_esp}),
        format!("{:20}: {:#018x}", "sysenter_eip", {vmsa.sysenter_eip}),
        format!("{:20}: {:#018x}", "cr2", {vmsa.cr2}),

        format!("{:20}: {:?}", "reserved_6", {vmsa.rsvd6}),

        format!("{:20}: {:#018x}", "g_pat", {vmsa.g_pat}),
        format!("{:20}: {:#018x}", "dbgctl", {vmsa.dbgctl}),
        format!("{:20}: {:#018x}", "br_from", {vmsa.br_from}),
        format!("{:20}: {:#018x}", "br_to", {vmsa.br_to}),
        format!("{:20}: {:#018x}", "last_excp_from", {vmsa.last_excp_from}),
        format!("{:20}: {:#018x}", "last_excp_to", {vmsa.last_excp_to}),

        format!("{:20}: {:?}", "reserved_7", {vmsa.rsvd7}),

        format!("{:20}: {:#010x}", "spec_ctrl", {vmsa.spec_ctrl}),

        format!("{:20}: {:?}", "reserved_7b", {vmsa.rsvd8}),

        format!("{:20}: {:#010x}", "pkru", {vmsa.pkru}),

        format!("{:20}: {:?}", "reserved_7a", {vmsa.rsvd9}),

        format!("{:20}: {:#018x}", "reserved_8", {vmsa.rsvd10}),

        format!("{:20}: {:#018x}", "rcx", {vmsa.rcx}),
        format!("{:20}: {}", "rdx", format_u64_bitmask(vmsa.rdx)),
        format!("{:20}: {:#018x}", "rbx", {vmsa.rbx}),

        format!("{:20}: {:#018x}", "reserved_9", {vmsa.rsvd11}),

        format!("{:20}: {:#018x}", "rbp", {vmsa.rbp}),
        format!("{:20}: {:#018x}", "rsi", {vmsa.rsi}),
        format!("{:20}: {:#018x}", "rdi", {vmsa.rdi}),
        format!("{:20}: {:#018x}", "r8", {vmsa.r8}),
        format!("{:20}: {:#018x}", "r9", {vmsa.r9}),
        format!("{:20}: {:#018x}", "r10", {vmsa.r10}),
        format!("{:20}: {:#018x}", "r11", {vmsa.r11}),
        format!("{:20}: {:#018x}", "r12", {vmsa.r12}),
        format!("{:20}: {:#018x}", "r13", {vmsa.r13}),
        format!("{:20}: {:#018x}", "r14", {vmsa.r14}),
        format!("{:20}: {:#018x}", "r15", {vmsa.r15}),

        format!("{:20}: {:?}", "reserved_10", {vmsa.rsvd12}),

        format!("{:20}: {:#018x}", "sw_exit_code", {vmsa.sw_exit_code}),
        format!("{:20}: {:#018x}", "sw_exit_info_1", {vmsa.sw_exit_info_1}),
        format!("{:20}: {:#018x}", "sw_exit_info_2", {vmsa.sw_exit_info_2}),
        format!("{:20}: {:#018x}", "sw_scratch", {vmsa.sw_scratch}),

        format!("{:20}: {:?}", "reserved_11", {vmsa.rsvd13}),

        format!("{:20}: {:#018x}", "xcr0", {vmsa.xcr0}),
        format!("{:20}: {:?}", "valid_bitmap", {vmsa.valid_bitmap}),
        format!("{:20}: {:#018x}", "x87_state_gpa", {vmsa.x87_state_gpa}),
    ];
    string_list.join("\n")
}

pub fn cmd(_args: CmdArgs) -> super::Result<()> {
    let mut vmsa = Vmsa::default();
    vmsa.init_amd64();
    vmsa.init_kvm();

    println!("{}", format_vmsa(vmsa));

    Ok(())
}
