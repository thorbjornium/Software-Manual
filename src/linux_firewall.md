# Firewall

<!-- toc -->

## Ubuntu Firewall (UFW) Secure Baseline Configuration

```bash

sudo ufw reset  
sudo ufw default deny incoming  
sudo ufw default allow outgoing  

```

## SSH (adjust port if not using 22)

```bash

sudo ufw allow 22/tcp comment 'SSH access'

```

## HTTP/HTTPS (for web servers)

```bash

sudo ufw allow 80/tcp comment 'HTTP'  
sudo ufw allow 443/tcp comment 'HTTPS'

```

## DNS

```bash

sudo ufw allow 53 comment 'DNS'

```

## NTP (Time sync)

```bash

sudo ufw allow 123/udp comment 'NTP'

```

## SMTP (Email)

```bash

sudo ufw allow 25/tcp comment 'SMTP'  
sudo ufw allow 587/tcp comment 'SMTP Submission'  
sudo ufw allow 465/tcp comment 'SMTPS'  

```

## Rate limiting for SSH (prevent brute force)

```bash

sudo ufw limit 22/tcp

```

## ICMP (ping) - enable carefully

```bash

sudo ufw allow proto icmp comment 'ICMP (ping)'

```

## Block common attack vectors

```bash

sudo ufw deny 23/tcp comment 'Block Telnet'  
sudo ufw deny 135:139/tcp comment 'Block NetBIOS'  
sudo ufw deny 445/tcp comment 'Block SMB'  
sudo ufw deny 1433:1434/tcp comment 'Block MS-SQL'  

```

## Application-Specific  

```bash

sudo ufw allow 2375/tcp comment 'Docker API'  
sudo ufw allow 2376/tcp comment 'Docker TLS'

```

```bash

sudo ufw allow 6443/tcp comment 'Kubernetes API'
sudo ufw allow 10250/tcp comment 'Kubelet API'

```

## Finalize  

```bash

sudo ufw enable  
sudo ufw status numbered  
sudo systemctl enable ufw  

```

## Check logs

```bash

sudo tail -f /var/log/ufw.log

```

To delete a rule:

```bash

sudo ufw delete [RULE_NUMBER]

```

## My ufw  

```bash

sudo ufw reset  
sudo ufw default deny incoming  
sudo ufw default allow outgoing
sudo ufw allow 22/tcp comment 'SSH access'
sudo ufw allow 443/tcp comment 'HTTPS'
sudo ufw allow 123/udp comment 'NTP'
sudo ufw allow 25/tcp comment 'SMTP'  
sudo ufw allow 53 comment 'DNS'
sudo ufw allow 587/tcp comment 'SMTP Submission'  
sudo ufw allow 465/tcp comment 'SMTPS'  
sudo ufw limit 22/tcp
sudo ufw deny 23/tcp comment 'Block Telnet'  
sudo ufw deny 135:139/tcp comment 'Block NetBIOS'  
sudo ufw deny 445/tcp comment 'Block SMB'  
sudo ufw deny 1433:1434/tcp comment 'Block MS-SQL'  
sudo ufw enable  
sudo ufw status numbered  
sudo systemctl enable ufw  

```
