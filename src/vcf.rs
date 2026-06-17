use std::str::FromStr;

use rust_htslib::bcf::{Read, Reader};

// VCF SECTION
// Struct for storing VCF record information
#[derive(Clone, Debug)]
pub struct VcfRecord {
    pub chromosome: String,
    pub position: i64,
    pub id: String,
    pub quality: f32,
    pub ref_allele: String,
    pub alt_allele: Vec<String>,
}

// Struct to hold the Filter values
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
    let mut bcf = Reader::from_path(path_str).expect("Error opening file: {path_str}.");
    bcf.records()
        .flatten()
        .map(|record| {
            let chromosome = match record.header().rid2name(record.rid().unwrap()) {
                Ok(chr) => String::from_str(std::str::from_utf8(chr).unwrap_or_default())
                    .unwrap_or_default(),
                Err(_) => String::new(),
            };

            let position = record.pos() + 1;
            let id = String::from_utf8(record.id()).unwrap_or_default();
            let quality = record.qual();
            let ref_allele = String::from_utf8_lossy(record.alleles()[0]).into_owned();
            let alt_allele: Vec<String> = record.alleles()[1..]
                .iter()
                .map(|x| String::from_utf8_lossy(x).into_owned())
                .collect();
            VcfRecord {
                chromosome,
                position,
                id,
                quality,
                ref_allele,
                alt_allele,
            }
        })
        .collect()
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
            if let Some(reference) = &filters.ref_allele {
                if record.ref_allele != *reference {
                    return false;
                }
            }
            if let Some(alternative) = &filters.alt_allele {
                if !record.alt_allele.contains(alternative) {
                    return false;
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
        assert_eq!(first_record.id, ".");
        assert_eq!(first_record.quality, 495.23);
        assert_eq!(first_record.ref_allele, "A");
        assert_eq!(first_record.alt_allele, vec!["C"]);
    }

    #[test]
    fn test_read_vcf_compressed() {
        let records = read_vcf("testfiles/1kGP-subset.vcf.gz");
        assert_eq!(records.len(), 306);
        let first_record = &records[0];
        assert_eq!(first_record.chromosome, "13");
        assert_eq!(first_record.position, 32872836);
        assert_eq!(first_record.id, ".");
        assert_eq!(first_record.quality, 495.23);
        assert_eq!(first_record.ref_allele, "A");
        assert_eq!(first_record.alt_allele, vec!["C"]);
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
    #[test]
    fn test_read_vcf_multi_allelic() {
        let records = read_vcf("testfiles/1kGP-subset.vcf");
        // Record at index 245 is the multi-allelic site:
        //   REF=CAA, ALT=CA,C, QUAL=924.43
        let record = &records[245];
        assert_eq!(record.chromosome, "13");
        assert_eq!(record.position, 32966726);
        assert_eq!(record.id, ".");
        assert_eq!(record.quality, 924.43);
        assert_eq!(record.ref_allele, "CAA");
        assert_eq!(record.alt_allele, vec!["CA", "C"]);
    }
}
