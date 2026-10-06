pub enum TYPE {
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
}

pub enum CLASS {
    /// The internet
    IN = 1,
    /// (Obsolete) CSNET
    CS = 2,
    /// CHAOS
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,
}
