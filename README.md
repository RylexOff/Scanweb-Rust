
# Scan web - URL Bruteforcer in rust

# FR - Version

Qu'est-ce que le ScanWEB?
------------

ScanWEB est un scanner de contenu Web. Il recherche les sites Web existants (et/ou cachés)
Objets. Cela fonctionne essentiellement en lançant une attaque basée sur un dictionnaire contre
un serveur Web et analyse la réponse.

en locurence c'est un code en Rust pour scanner un serveur web à la recherche de répertoires à l'aide d'un fichier dictionnaire
Ce code effectue une série de requêtes HTTP GET sur l'URL cible (spécifiée en tant qu'argument de ligne de commande) en utilisant les différents répertoires du fichier dictionnaire (également spécifié en tant qu'argument de ligne de commande). Si la réponse HTTP indique que le répertoire existe (c'est-à-dire si elle commence par "HTTP/1.1 200 OK"), le nom du répertoire est affiché dans la console. Comme pour le code précédent, ce code n'est pas exhaustif et peut être amélioré pour gérer des erreurs ou des cas d'utilisation spécifiques.

Apres est-ce qu'il fonctionne bien ou non ?

Credits
-------

RylexOff 

#EN - Version
