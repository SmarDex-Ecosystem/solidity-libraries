use alloy_primitives::{ruint::aliases::U768, Bytes, FixedBytes, B512, U256, U512};
use alloy_sol_types::SolValue;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform a uint512 full division, yielding a uint512 output
    Div512 {
        /// Numerator bytes
        a: B512,
        /// Denominator bytes
        b: B512,
    },
    /// Perform a uint512 full division, yielding a uint512 output, rounding upwards
    DivUp512 {
        /// Numerator bytes
        a: B512,
        /// Denominator bytes
        b: B512,
    },
    /// Uint512 addition
    HugeUintAdd {
        /// First operand bytes
        a: B512,
        /// Second operand bytes
        b: B512,
    },
    /// Uint512 subtraction
    HugeUintSub {
        /// First operand bytes
        a: B512,
        /// Second operand bytes
        b: B512,
    },
    /// Full multiplication of two uint256
    HugeUintMul256 {
        /// First operand as a uint256
        a: U512,
        /// Second operand as a uint256
        b: U512,
    },
    /// Full multiplication of two uint512
    HugeUintMul {
        /// First operand as a uint512
        a: B512,
        /// Second operand as a uint512
        b: B512,
    },
    /// Division of a uint512 by a uint256
    HugeUintDiv256 {
        /// First operand as a uint512
        a: B512,
        /// Second operand as a uint256
        b: U512,
    },
    /// Division of a uint512 by a uint512
    HugeUintDiv {
        /// First operand as a uint512
        a: B512,
        /// Second operand as a uint512
        b: B512,
    },
    /// Count-left-zeroes of a uint256
    HugeUintClz {
        /// An unsigned 256-bit integer
        x: U256,
    },
    /// Reciprocal `floor((2^512-1) / d) - 2^256`
    HugeUintReciprocal {
        /// A 256-bit unsigned integer at least equal to 2^255
        d: U256,
    },
    /// Reciprocal `floor((2^768-1) / d) - 2^256`
    HugeUintReciprocal2 {
        /// A 512-bit unsigned integer with its high limb at least equal to 2^255
        d: U512,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Div512 { a, b } => {
            let a: U512 = a.into();
            let b: U512 = b.into();
            let res = a / b;
            let lsb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[32..].try_into()?);
            let msb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[..32].try_into()?);
            print_u512_hex(lsb, msb);
        }
        Commands::DivUp512 { a, b } => {
            let a: U512 = a.into();
            let b: U512 = b.into();
            let res = a.div_ceil(b);
            let lsb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[32..].try_into()?);
            let msb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[..32].try_into()?);
            print_u512_hex(lsb, msb);
        }
        Commands::HugeUintAdd { a, b } => {
            let a: U512 = a.into();
            let b: U512 = b.into();
            let res = a + b;
            let lsb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[32..].try_into()?);
            let msb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[..32].try_into()?);
            print_u512_hex(lsb, msb);
        }
        Commands::HugeUintSub { a, b } => {
            let a: U512 = a.into();
            let b: U512 = b.into();
            let res = a - b;
            let lsb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[32..].try_into()?);
            let msb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[..32].try_into()?);
            print_u512_hex(lsb, msb);
        }
        Commands::HugeUintMul256 { a, b } => {
            let res = a * b;
            let lsb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[32..].try_into()?);
            let msb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[..32].try_into()?);
            print_u512_hex(lsb, msb);
        }
        Commands::HugeUintMul { a, b } => {
            let a: U512 = a.into();
            let b: U512 = b.into();
            let res = a * b;
            let lsb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[32..].try_into()?);
            let msb = U256::from_be_bytes::<32>(res.to_be_bytes::<64>()[..32].try_into()?);
            print_u512_hex(lsb, msb);
        }
        Commands::HugeUintDiv256 { a, b } => {
            let a: U512 = a.into();
            let res = a / b;
            assert!(res <= U512::from(U256::MAX));
            let bytes: [u8; 32] = res.to_be_bytes::<64>()[32..].try_into()?;
            let x_bytes: FixedBytes<32> = bytes.into();
            print!("{x_bytes}");
        }
        Commands::HugeUintDiv { a, b } => {
            let a: U512 = a.into();
            let b: U512 = b.into();
            let res = a / b;
            assert!(res <= U512::from(U256::MAX));
            let bytes: [u8; 32] = res.to_be_bytes::<64>()[32..].try_into()?;
            let x_bytes: FixedBytes<32> = bytes.into();
            print!("{x_bytes}");
        }
        Commands::HugeUintClz { x } => {
            let bytes: [u8; 32] = x.to_be_bytes();
            let clz = bytes.iter().position(|&b| b != 0).map_or(256, |n| {
                let skipped = n * 8;
                let top = bytes[n].leading_zeros() as usize;
                skipped + top
            });
            print_u256_hex(U256::from(clz));
        }
        Commands::HugeUintReciprocal { d } => {
            let res = U512::MAX / U512::from(d) - (U512::from(U256::MAX) + U512::from(1));
            assert!(res <= U512::from(U256::MAX));
            let bytes: [u8; 32] = res.to_be_bytes::<64>()[32..].try_into()?;
            let x_bytes: FixedBytes<32> = bytes.into();
            print!("{x_bytes}");
        }
        Commands::HugeUintReciprocal2 { d } => {
            let res = U768::MAX / U768::from(d) - (U768::from(U256::MAX) + U768::from(1));
            assert!(res <= U768::from(U256::MAX));
            let bytes: [u8; 32] = res.to_be_bytes::<96>()[64..].try_into()?;
            let x_bytes: FixedBytes<32> = bytes.into();
            print!("{x_bytes}");
        }
    }
    Ok(())
}

fn print_u256_hex(x: U256) {
    let bytes: [u8; 32] = x.to_be_bytes();
    let x_bytes: FixedBytes<32> = bytes.into();
    print!("{x_bytes}");
}

fn print_u512_hex(lsb: U256, msb: U256) {
    let data = (lsb, msb);
    let bytes = data.abi_encode_params();
    let bytes: Bytes = bytes.into();
    print!("{bytes}");
}
