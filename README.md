# 🦀 etheryal Bootimage

Create a booteable image with the specified etheryal Kernel and Kernel modules.

# Usage

## Building a Kernel booteable image

### Command

`etheryal-bootimage build [FLAGS] --build-cmd <build-cmd> --out <out>`

### Flags
    --create-out      
    --disable-bios    
    --disable-uefi

## Running a Kernel in a Virtual Machine

### Command

`etheryal-bootimage run --out <out> [KERNEL ELF]`

# Get Help

Read the README at the [etheryal Kernel repository](https://github.com/etheryal/etheryal-kernel)