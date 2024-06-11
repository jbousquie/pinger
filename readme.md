# Pinger

Outil de ping de parc informatique codé en Rust, basé sur [surge-ping](https://docs.rs/surge-ping/latest/surge_ping/).  
Pinger lit en entrée un fichier d'adresses IPv4 à tester, puis envoie simultanément autant de pings que d'adresses valides.  
Il ne nécessite pas que la commande `ping` soit disponible sur le système, ni d'être lancé en tant que `root`.  

Le fichier d'adresses à pinger peut être généré par l'utilitaire `./pinger_generate`. Cet utilitaire lit le fichier template d'adresses (par défaut le fichier `adresses_template.txt` placé dans le mếme répertoire) et génére le fichier d'adresses à pinguer (par défaut `adresses.txt`).  
Le format du fichier template est simple : chaque octet d'une adresse IPv4 peut être écrit sous la forme d'un entier directement, ou d'une plage d'entiers (ex : `2-10`), ou du caractère générique `*` signifiant toute la plage `0-255`.  
Exemple :   
```txt
// les commentaires de ligne sont autorisés dans ce fichier
#  avec "//" ou "#"

// réseau local personnels
192.168.1.*
// réseaux locaux étudiants
10.1.1-50.*
# plage serveurs publics
193.54.203.1-128
# DNS Google
8.8.8.8
```
Pour générer le fichier d'adresses à pinguer à partir de ce template, il suffit de taper la commande `./pinger_generate` qui sera placée dans le même répertoire que le fichier de configuration `./pinger.conf`.    
Le fichier généré peut bien sûr être modifié manuellement ou par script avec d'être lu par la commande `./pinger`.


Les résultats des pings sont inscrits dans un fichier de log qui préserve les anciennes mesures : si une machine n'a pas répondu cette fois au ping, mais qu'elle avait répondu précédemment, alors le timestamp de la dernière réponse valide est conservé dans le fichier de logs.  
Le fichier de log contient une liste de paires :  
```adresse_ip_pinguée, timestamp_utc_de_la_dernière_réponse```  

Les paires sont triées dans le fichier de log dans le même ordre que les adresses IP transmises dans le fichier des adresses à tester, pour simplifier la lecture ou d'éventuels traitements ensuite.  

L'outil peut être lancé via la crontab à intervalles réguliers. Il met une seconde (avec un timeout fixé à 1 s) à tester plusieurs milliers de machines.  

Pour l'exécuter, placer le binaire `pinger` dans un répertoire contenant son fichier de configuration `pinger.conf`, puis lancer la commande 
```./pinger```

Le fichier de configuration `pinger.conf` a le format suivant :
```toml
# Fichier de configuration de Pinger au format TOML
# addr_filename = "/path/vers/fichier/adresses_à_pinguer"
# log_filename = "/path/vers/fichier/log_des_pings"
# logfile_sep = ","         # caractère de séparation ip/timestamp dans le fichier de log des pings
# ping_timeout = 1500       # (entier) durée en millesecondes d'attente avant de considérer un ping comme non répondu
# addr_template = "/path/vers/fichier/template_generation_adresses"
# task_group_nb  = 64       # (entier) nombre de tâches lancées avant d'attendre un petit délai pour continuer pour limiter les risques de congestion
# task_group_delay = 10     # (entier) nombre de millisecondes à attendre avant de lancer le groupe de tâches suivant

addr_filename = "./adresses.txt"
log_filename = "./pinger.log"
logfile_sep = ","
ping_timeout = 1750
addr_template = "./adresses_template.txt"
task_group_nb = 64
task_group_delay = 10
```
Dans cet exemple, les pings sont lancés par groupes de 64 ip de destination et un délai de 10 ms est appliqué entre chaque lancement, pour limiter les risques de congestion sur l'interface.  
Selon votre interface et votre réseau, des groupes de 32 à 512 tâches simultanées et des délais entre 4 et 20 ms peuvent être pertinents par exemple.  


Le code est commenté en français pour faciliter la maintenance ou l'apprentissage de Rust.  

Binaire compilé pour linux à télécharger ici : https://jerome.bousquie.fr/soft/pinger/linux/pinger.zip