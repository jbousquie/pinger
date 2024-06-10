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


Le code est commenté en français pour faciliter la maintenance ou l'apprentissage de Rust.  

Binaire compilé pour linux à télécharger ici : https://jerome.bousquie.fr/soft/pinger/linux/pinger.zip