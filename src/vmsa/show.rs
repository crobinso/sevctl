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

pub fn cmd(args: CmdArgs) -> super::Result<()> {
    let mut vmsa = Vmsa::default();
    vmsa.init_amd64();
    vmsa.init_kvm();

    println!("VMSA HAHA %{:#?}\n\n", vmsa);

    Ok(())
}
