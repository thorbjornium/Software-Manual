# NsLookup

Basic Usage:

1. Find the IP address of a domain name (forward lookup):
    nslookup google.com

    Output will show:
    - Server used for the query (default DNS server)
    - Name of the domain (google.com)
    - Address (IP address(es) for google.com)

2. Find the domain name associated with an IP address (reverse lookup):
    nslookup 8.8.8.8

    Output will show:
    - Server used for the query
    - Reverse lookup results, if available (e.g., google-public-dns-a.google.com)

Specifying Query Type:

3. Query for a specific record type (e.g., MX records for mail servers):
    nslookup -type=mx google.com

    Output will show:
    - MX records for google.com, listing mail servers and priority

4. Query for Name Server (NS) records:
    nslookup -type=ns google.com

    Output will show:
    - NS records for google.com, listing authoritative name servers

5. Query for Start of Authority (SOA) record:
    nslookup -type=soa google.com

    Output will show:
    - SOA record for google.com, containing administrative information about the domain

Specifying DNS Server:

6. Use a specific DNS server (e.g., Google Public DNS 8.8.8.8) for the query:
    nslookup google.com 8.8.8.8

    Output will be similar to basic lookup, but using 8.8.8.8 as the DNS server

7. Use a specific DNS server for a specific record type:
    nslookup -type=mx google.com 8.8.8.8

    Output will show MX records for google.com, queried from 8.8.8.8

Interactive Mode:

8. Enter interactive mode:
    nslookup

    Then, within nslookup interactive mode:
    > set type=a  
    > google.com  
    > server 8.8.8.8  
    > microsoft.com  
    > exit  

    - `nslookup` starts interactive mode
    - `set type=a` sets query type to A records (IP addresses)
    - `google.com` performs lookup for google.com
    - `server 8.8.8.8` changes the DNS server to 8.8.8.8 for subsequent queries
    - `microsoft.com` performs lookup for microsoft.com using 8.8.8.8
    - `exit` quits interactive mode

Common Record Types to Query with -type=:

- A:      Address record (IPv4 address)
- AAAA:   IPv6 address record
- MX:     Mail exchange record (mail servers)
- NS:     Name server record (authoritative DNS servers)
- SOA:    Start of Authority record (domain admin info)
- CNAME:  Canonical name record (alias for another domain name)
- TXT:    Text record (often used for verification or SPF records)

Note:  `nslookup` is being replaced by `dig` and `host` utilities in many environments as they offer more features and are considered more modern. However, `nslookup` is still widely available and useful for basic DNS queries.

sfaskldfjlkdakls
