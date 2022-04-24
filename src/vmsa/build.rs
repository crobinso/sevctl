// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;

use crate::{Ovmf, Vmsa};

use structopt::StructOpt;

pub enum UserspaceVmm {
    Qemu,
    Krun,
}

impl fmt::Debug for UserspaceVmm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserspaceVmm::Qemu => write!(f, "qemu"),
            UserspaceVmm::Krun => write!(f, "krun"),
        }
    }
}

impl FromStr for UserspaceVmm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "krun" {
            return Ok(UserspaceVmm::Krun);
        }

        // QEMU is default.

        Ok(UserspaceVmm::Qemu)
    }
}

#[derive(StructOpt, Debug)]
pub struct CmdArgs {
    #[structopt(parse(from_os_str), help = "File to write VMSA information to")]
    pub filename: PathBuf,

    #[structopt(long, help = "CPU number")]
    pub cpu: u64,

    #[structopt(long, help = "CPU family")]
    pub family: Option<u64>,

    #[structopt(long, help = "CPU model")]
    pub model: Option<u64>,

    #[structopt(long, help = "CPU stepping")]
    pub stepping: Option<u64>,

    #[structopt(long, parse(from_os_str), help = "OVMF firmware path")]
    pub firmware: Option<PathBuf>,

    #[structopt(long, help = "Userspace implementation")]
    pub userspace: UserspaceVmm,
}

pub fn cmd(args: CmdArgs) -> super::Result<()> {
    let mut vmsa = Vmsa::default();
    vmsa.init_amd64();
    vmsa.init_kvm();
    match args.userspace {
        UserspaceVmm::Qemu => vmsa.init_qemu(args.cpu),
        UserspaceVmm::Krun => vmsa.init_krun(args.cpu),
    };

    let family: u64 = args.family.unwrap_or(0);
    let model: u64 = args.model.unwrap_or(0);
    let stepping: u64 = args.stepping.unwrap_or(0);

    if family > 0 || model > 0 || stepping > 0 {
        vmsa.cpu_sku(family, model, stepping);
    }

    if let Some(fw) = args.firmware {
        let _ovmf = Ovmf::default();
        let _ovmf = _ovmf.load(fw);
    }

    Ok(())
}
