// Copyright (c) 2021 Miguel Peláez
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::path::PathBuf;

use clap::Clap;

/// Build a booteable etheryal image
#[derive(Clap)]
#[clap(version = "0.3", author = "Miguel Peláez <kernelfreeze@outlook.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommands,
}

#[derive(Clap)]
pub struct RunOpts {
    #[clap(long, default_value = "--no-reboot -serial stdio -s")]
    pub run_args: String,

    pub binary_path: PathBuf,
    
    #[clap(long, short, default_value = "out")]
    pub out: PathBuf,
}

#[derive(Clap)]
pub struct BuildOpts {
    #[clap(long, default_value = "x86_64")]
    pub target: String,

    #[clap(long)]
    pub build_cmd: String,

    #[clap(long)]
    pub disable_uefi: bool,

    #[clap(long)]
    pub disable_bios: bool,

    #[clap(long)]
    pub create_out: bool,

    #[clap(long, short)]
    pub out: PathBuf,
}

#[derive(Clap)]
pub enum SubCommands {
    /// Run a virtual machine using qemu
    #[clap(version = "0.3", author = "Miguel Peláez <kernelfreeze@outlook.com>")]
    Run(RunOpts),

    /// Create a booteable image only
    #[clap(version = "0.3", author = "Miguel Peláez <kernelfreeze@outlook.com>")]
    Build(BuildOpts),
}
