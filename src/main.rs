mod mersenne_primes;
mod multi_base_encoder;
mod shamir_secret;

use clap::{Parser, ArgGroup};
use arboard::Clipboard;
use num_bigint::BigUint;
use regex::Regex;
use std::io::{self, Write};

use mersenne_primes::MersennePrimes;
use multi_base_encoder::MultiBaseEncoder;
use shamir_secret::ShamirSecret;

const DEFAULT_MIN: usize = 3;
const DEFAULT_SHARES: usize = 5;

#[derive(Parser, Debug)]
#[command(name = "shamir")]
#[command(about = "Shamir secret sharing - V2019.1 by Luca Viola", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .args(&["reconstruct"])
))]
struct Args {
    /// The minimum # of shares needed to reconstruct the original key
    #[arg(short = 't', long, value_name = "MINIMUM")]
    threshold: Option<usize>,

    /// The maximum # of shares to be generated
    #[arg(short = 's', long, value_name = "SHARES")]
    shares: Option<usize>,

    /// The password to be shared in base 62 encoding [A-Za-z0-9]
    #[arg(short = 'k', long, value_name = "KEY")]
    key: Option<String>,

    /// Reconstruct secret from minimum shares
    #[arg(short = 'r', long, value_name = "MINIMUM_SHARES")]
    reconstruct: Option<usize>,
}

fn get_password(prompt: &str) -> Result<String, std::io::Error> {
    rpassword::prompt_password(prompt)
}

fn reconstruct(minimum: usize, prime: BigUint) -> Result<String, Box<dyn std::error::Error>> {
    let base62 = MultiBaseEncoder::new(62);
    let base93 = MultiBaseEncoder::new(93);
    let shamir_secret = ShamirSecret::new(prime);

    let mut poly = Vec::new();
    let regex = Regex::new(r#"^(\d*)-([0-9a-zA-Z!#$%&()*+\-;<=>?@^_`{|}~"',./:\[\]\\]*)$"#)?;

    let mut count = 1;
    while count <= minimum {
        print!("Insert share #{}/{}: ", count, minimum);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let p = input.trim();

        if !regex.is_match(p) {
            println!("Invalid key");
            continue;
        }

        let parts: Vec<&str> = p.splitn(2, '-').collect();
        if parts.len() != 2 {
            println!("Invalid key format");
            continue;
        }

        let p0 = parts[0].trim();
        let p1 = parts[1].trim();

        match p0.parse::<usize>() {
            Ok(index) => {
                let value = base62.decode(p1);
                poly.push((index, value));
                count += 1;
            }
            Err(_) => {
                println!("Invalid share index");
                continue;
            }
        }
    }

    let secret_value = shamir_secret.recover_secret(&poly)?;
    let secret = base93.encode(secret_value);
    Ok(secret)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mersenne_primes = MersennePrimes::new();
    let prime = mersenne_primes.get_mersenne_prime(13);
    let base62 = MultiBaseEncoder::new(62);
    let base93 = MultiBaseEncoder::new(93);

    if let Some(minimum) = args.reconstruct {
        let secret = reconstruct(minimum, prime)?;

        let mut clipboard = Clipboard::new()
            .map_err(|e| format!("Failed to access clipboard: {}", e))?;
        clipboard.set_text(&secret)
            .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

        println!("The secret has been copied to the clipboard");
        return Ok(());
    }

    let key = if let Some(k) = args.key {
        k
    } else {
        let p = get_password("Insert password: ")?;
        let q = get_password("Repeat password: ")?;

        if p != q {
            eprintln!("The passwords don't match!");
            std::process::exit(1);
        }
        p
    };

    let keycode = base93.decode(key.trim());

    let minimum = args.threshold.unwrap_or(DEFAULT_MIN);
    let shares = args.shares.unwrap_or(DEFAULT_SHARES);

    println!("Generating {} shares with a minimum of {} shares required", shares, minimum);

    let shamir_secret = ShamirSecret::new(prime.clone());
    let (secret, share_points) = shamir_secret.make_random_shamir_pool(keycode, minimum, shares)?;

    println!("shares:");
    for (index, value) in &share_points {
        println!("  {}-{}", index, base62.encode(value.clone()));
    }

    let test1_shares = &share_points[..minimum];
    let test2_shares = &share_points[share_points.len() - minimum..];

    let test1 = base62.encode(shamir_secret.recover_secret(test1_shares)?);
    let test2 = base62.encode(shamir_secret.recover_secret(test2_shares)?);
    let secret_encoded = base62.encode(secret);

    if test1 == test2 && test1 == secret_encoded {
        println!("Minimum shares reconstruction test passed, generation complete.");
    } else {
        println!("Minimum shares reconstruction test not passed.");
    }

    Ok(())
}
