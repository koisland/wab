use clap::Parser;

use crate::Mode;

/// Calculate ratio of treatment and control pileup over a set window.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Treatment bgzipped pileup. Must be indexed via `tabix -p bed`.
    #[arg(short, long)]
    pub treatment: String,
    
    /// Control bgzipped pileup. Must be indexed via `tabix -p bed`.
    #[arg(short, long)]
    pub control: String,

    /// Chromosome lengths in TSV format with chrom and length column. Fasta indexes are accepted.
    #[arg(short = 'l', long)]
    pub lengths_chrom: String,

    /// Window size to aggregate over.
    #[arg(short, long, default_value_t = 5000)]
    pub window: usize,

    /// Mode to compare by.
    #[arg(short, long)]
    pub mode: Mode
}