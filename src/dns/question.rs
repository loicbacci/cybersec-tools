pub struct Question {
    entries: Vec<QuestionEntry>,
}

impl Question {
    pub fn new(entries: Vec<(&str, QTYPE, QCLASS)>) -> Self {
        let q_entries = entries
            .into_iter()
            .map(|(dn, qt, qc)| QuestionEntry::new(dn, qt, qc))
            .collect();

        Question { entries: q_entries }
    }

    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        for entry in &self.entries {
            entry.write_to_buf(buf);
        }
    }
}

struct QuestionEntry {
    /// The domain name
    qname: QNAME,
    /// (16-bit) The type of the query
    qtype: QTYPE,
    /// (16-bit) The class of the query
    qclass: QCLASS,
}

impl QuestionEntry {
    fn new(domain_name: &str, qtype: QTYPE, qclass: QCLASS) -> QuestionEntry {
        let qname = QNAME::new(domain_name);

        QuestionEntry {
            qname,
            qtype,
            qclass,
        }
    }

    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        self.qname.write_to_buf(buf);

        let qtype: u16 = self.qtype as u16;
        let qclass: u16 = self.qclass as u16;

        buf.extend_from_slice(&qtype.to_be_bytes());
        buf.extend_from_slice(&qclass.to_be_bytes());
    }
}

struct QNAME {
    /// Each label of the domain name
    labels: Vec<Label>,
}

impl QNAME {
    /// Creates a question QNAME from a domain name
    fn new(domain_name: &str) -> QNAME {
        // Split the domain name into labels
        let label_strs = domain_name.split('.');
        let labels = label_strs.map(|l| Label::new(&l)).collect();

        QNAME { labels }
    }

    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        for label in &self.labels {
            label.write_to_buf(buf);
        }

        // Write termination
        buf.push(0);
    }
}

struct Label {
    /// The letters of the label
    letters: Vec<u8>,
}

impl Label {
    fn new(label_str: &str) -> Label {
        let bytes: Vec<u8> = label_str.as_bytes().to_vec();

        Label { letters: bytes }
    }

    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push(self.letters.len() as u8);
        buf.extend_from_slice(self.letters.as_slice());
    }
}

#[derive(Copy, Clone)]
pub enum QTYPE {
    /// Host address
    A = 1,
    /// Authoritative name server
    NS = 2,
    /// (Obsolete) Mail destination
    MD = 3,
    /// (Obsolete) Mail forwareder
    MF = 4,
    /// Cononical name for an alias
    CNAME = 5,
    /// Start of zone of authority
    SOA = 6,
    /// (Experimental) Mailbox domain name
    MB = 7,
    /// (Experimental) Mail group member
    MG = 8,
    /// (Experimental) Mail rename domain name
    MR = 9,
    // (Experimental) Null record
    NULL = 10,
    /// Well known service description
    WKS = 11,
    /// Domain name pointer
    PTR = 12,
    /// Host information
    HINFO = 13,
    /// Mailbox or mail list information
    MINFO = 14,
    /// Mail exchange
    MX = 15,
    /// Text strings
    TXT = 16,

    /// Request transfer of an entire zone
    AXFR = 252,
    /// Request for mailbox-related records
    MAILB = 253,
    /// (Obsolete) Request for mail agent records
    MAILA = 254,
    /// Request all records
    STAR = 255,
}

#[derive(Copy, Clone)]
pub enum QCLASS {
    /// The internet
    IN = 1,
    /// (Obsolete) CSNET
    CS = 2,
    /// CHAOS
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,

    /// Any class
    STAR = 255,
}
