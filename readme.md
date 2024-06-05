# Pinger

Outil de ping de parc informatique codé en Rust, basé sur [surge-ping](https://docs.rs/surge-ping/latest/surge_ping/).  
Pinger lit en entrée un fichier d'adresse IPv4 à tester, puis envoie simultanément autant de pings que d'adresses valides.  
Il ne nécessite pas que la commande `ping` soit disponible sur le système, ni d'être lancé en tant que `root`.  

Les résultats des pings sont inscrits dans un fichier de log qui préseve les anciennes mesures : si une machine n'a pas répondu cette fois au ping, mais qu'elle avait répondu précédemment, alors le timestamp de la dernier réponse valide est conservé dans le fichier de logs.  
Le fichier de logs contient une liste de paires : `adresse_ip_pinguée, timestamp_utc_de_la_dernière_réponse`  
Les résultats sont triés dans le fichier de log dans le même ordre que les adresses IP transmises dans le fichier des adresses à tester.  

L'outil peut être lancé via la crontab à intervalles réguliers. Il met une seconde (avec un timeout fixé à 1 s) à tester plusieurs milliers de machines.  


Le code est commenté en français pour faciliter la maintenance ou l'apprentissage de Rust.  
