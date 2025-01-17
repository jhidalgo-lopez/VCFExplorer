use rust_htslib::bcf::{Read, Reader};

// VCF SECTION
// Struct for storing VCF record information
#[derive(Debug)]
pub struct VcfRecord {
    pub chromosome: String,
    pub position: i64,
    pub id: u32,
    pub ref_allele: String,
    pub alt_allele: String,
}

// Function to read VCF data
pub fn read_vcf(path_str: &str) -> Vec<VcfRecord> {
    let mut records = Vec::new();
    let mut bcf = Reader::from_path(path_str).expect("Error opening file: {path_str:?}.");
    for record in bcf.records().flatten() {
        let mut chromosome = String::new();
        if let Ok(chr) = record.header().rid2name(record.rid().unwrap()) {
            for c in chr {
                chromosome.push(*c as char);
            }
        }

        let position = record.pos();
        let id = record.rid().unwrap();
        let mut ref_allele = String::new();
        for allele in record.alleles()[0] {
            ref_allele.push(char::from(*allele))
        }
        let mut alt_allele = String::new();
        for allele in record.alleles()[1] {
            alt_allele.push(char::from(*allele))
        }
        let entry = VcfRecord {
            chromosome,
            position,
            id,
            ref_allele,
            alt_allele,
        };
        records.push(entry);
    }
    records
}
