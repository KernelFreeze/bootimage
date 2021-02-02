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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BootImageError {
    #[error("failed io operation. {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to locate cargo manifest for kernel. {0}")]
    LocateManifest(#[from] locate_cargo_manifest::LocateManifestError),

    #[error("failed to parse kernel cargo manifest. {0}")]
    ManifestError(#[from] cargo_manifest::Error),

    #[error("failed to build image")]
    BuildFailed,

    #[error("failed to find binary in kernel manifest")]
    KernelManifest,

    #[error("failed to find kernel project root")]
    KernelRootNotFound,

    #[error("failed to create disk image. {0}")]
    CreateDiskImage(#[from] CreateDiskImageError),

    #[error("failed to set logger. {0}")]
    SetLogger(#[from] log::SetLoggerError),

    #[error("output directory doesn't exist")]
    OutNotExist,
}

#[derive(Error, Debug)]
pub enum CreateDiskImageError {
    #[error("failed to move to output directory. {0}")]
    Move(std::io::Error),

    #[error("failed to move to output directory. {0}")]
    FindMoved(std::io::Error),

    #[error("failed to build image")]
    BuildFailed,

    #[error("failed to find kernel root")]
    RootNotFound,

    #[error("failed to locate cargo manifest for bootloader. {0}")]
    LocateManifestError(#[from] locate_cargo_manifest::LocateManifestError),

    #[error("failed to locate bootloader. {0}")]
    LocateError(#[from] bootloader_locator::LocateError),
}
