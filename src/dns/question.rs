pub struct Question {}

struct QuestionEntry {
    /// The domain name
    qname: QNAME,
    /// (16-bit) The type of the query
    qtype: QTYPE,
    /// (16-bit) The class of the query
    qclass: QCLASS,
}

struct QNAME {
    /// Each label of the domain name
    labels: Vec<Label>,
}

struct Label {
    /// Number of octets for the label
    length: u8,
    /// The letters of the label
    letters: Vec<u8>,
}

enum QTYPE {
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

enum QCLASS {
    /// The internet
    IN = 1,
    /// (Obsolete) CSNET
    CS = 2,
    /// CHAOS
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,

    /// Any class
    STAR = 255
}
