# Oslobandens (bejublede) bingojukebox - nettside

Benytter certbot for sertifikasjon med let's encrypt.
- snap installert på server
- certbot 1.16.0 installert på server

Har kjørt sudo certbot --nginx for www.oslobanden.no. Certbot skal da ta seg av fornyelser automatisk. Oppdaterer /etc/nginx/sites.enabled/default automatisk.
/etc/nginx/nginx.conf "importerer" filen ovenfor.

Automatisk redirect http -> https konfigurert av certbot.

server_name kan ikke ignoreres da certbot bruker dette for å finne www.oslobanden.no-relatert config via server_name.

Merk: www.oslobanden.no og oslobanden.no er separate, og bør håndteres (selv om de fleste browsere redirecter automatisk).