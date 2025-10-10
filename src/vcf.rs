use rust_htslib::bcf::{Read, Reader};

// VCF SECTION
// Struct for storing VCF record information
#[derive(Clone, Debug)]
pub struct VcfRecord {
    pub chromosome: String,
    pub position: i64,
    pub id: Vec<u8>,
    pub quality: f32,
    pub ref_allele: String,
    pub alt_allele: String,
}

// Struc to hold the Filter values
#[derive(Clone, Debug, Default)]
pub struct FilterConfig {
    pub chr: Option<String>,
    pub pos: Option<(i64, i64)>,
    pub qual: Option<(f32, Option<f32>)>,
    pub ref_allele: Option<String>,
    pub alt_allele: Option<String>,
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

        let position = record.pos() + 1;
        let id = record.id();
        let quality = record.qual();
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
            quality,
            ref_allele,
            alt_allele,
        };
        records.push(entry);
    }
    records
}

pub fn filter_vcf(records: &[VcfRecord], filters: &FilterConfig) -> Vec<VcfRecord> {
    records
        .iter()
        .filter(|record| {
            if let Some(chromosome) = &filters.chr {
                if *chromosome != record.chromosome {
                    return false;
                }
            }
            if let Some((pos_start, pos_end)) = &filters.pos {
                if record.position < *pos_start || record.position > *pos_end {
                    return false;
                }
            }
            if let Some((min_qual, max_qual)) = &filters.qual {
                if record.quality < *min_qual {
                    return false;
                }
                if let Some(max_qual_val) = max_qual {
                    if record.quality > *max_qual_val {
                        return false;
                    }
                }
            }
            true
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_vcf_uncompressed() {
        let records = read_vcf("testfiles/1kGP-subset.vcf");
        assert_eq!(records.len(), 306);
        let first_record = &records[0];
        assert_eq!(first_record.chromosome, "13");
        assert_eq!(first_record.position, 32872836);
        assert_eq!(first_record.id, b".");
        assert_eq!(first_record.quality, 495.23);
        assert_eq!(first_record.ref_allele, "A");
        assert_eq!(first_record.alt_allele, "C");
    }

    #[test]
    fn test_read_vcf_compressed() {
        let records = read_vcf("testfiles/1kGP-subset.vcf.gz");
        assert_eq!(records.len(), 306);
        let first_record = &records[0];
        assert_eq!(first_record.chromosome, "13");
        assert_eq!(first_record.position, 32872836);
        assert_eq!(first_record.id, b".");
        assert_eq!(first_record.quality, 495.23);
        assert_eq!(first_record.ref_allele, "A");
        assert_eq!(first_record.alt_allele, "C");
    }

    #[test]
    fn test_filter_vcf() {
        let records = read_vcf("testfiles/1kGP-subset.vcf");
        let filter = FilterConfig {
            chr: None,
            pos: Some((32872836, 32877888)),
            qual: None,
            ref_allele: None,
            alt_allele: None,
        };
        let filtered_records = filter_vcf(&records, &filter);
        assert_eq!(filtered_records.len(), 14);
    }
}
