# Généralités sur le projet

## Encodage / décodage

Pour toutes les structures que vous décodez, vous devez aussi écrire l'encodeur correspondant et vérifier de
l'encodage/décodage ou décodage/encodage avec des tests.

## Structuration

* Il faut bien (au minimum) 3 crates : `client`, `shared`, `server` (nommage indicatif).
* Il faut bien un fichier `Cargo.toml` à la racine qui les unifie tous (le mot clef magique sera `[workspace]`
  et `members`).

Un exemple possible se trouve dans ma base d'exemples:
https://github.com/haveneer/rust-quicklook-training/tree/master/rs/full-tree

## Rejet par le serveur fourni des messages de votre client

Si vous rencontrez des « `message : Too large message size` » de la part du serveur fourni, il est bien possible que
vous n'ayez pas bien respecté le format de transfert.

En effet, tel qu'il est décrit dans [le sujet](https://github.com/haveneer-training/Frakt#le-protocole-déchange), il y a
bien une part qui est la sérialisation en JSON du message souhaité (comme nous avons vu en cours) mais aussi le JSON
message size préalable qui doit contenir la taille du message total que vous lui envoyez.

En effet, dans le même tuyau (même stream TCP) pourra circuler une suite de différents messages : M1 M2 M3; pour qu'il
soit plus facile de séparer un flot d'octets en différents messages, chacun des messages est préfixé par sa taille : S1
M1 S2 M2 S3 M3, ce qui permet à chaque fois de lire le nombre Si (entier de taille fixe) puis les Si octets devant
ensuite être décodé en JSON.
Cela demandera quelques primitives d'écriture/lecture sur TcpStream différentes de celles que nous avons vues
(lire un nombre, lire un nombre d'octets définis ; idem pour l'écriture).

Un *timeout* renvoyé par le serveur peut aussi être un comportement différent du même problème.
Pour préciser, si vous n'écrivez pas en ce moment les préfixes Si, ce sont les premiers caractères de votre message qui
sont lus par le serveur comme un entier 32 bits (4 octets). Par exemple un message "Hi" qui ne contient que 2 caractères
(ici 2 octets) sera insuffisant pour reconstruire l'entier 32 bits attendu par le serveur. Il attendra encore 2 octets
jusqu'au timeout.

## Sérialisation par `serde` des chaînes de caractères

Quand `serde` sérialise une chaîne de caractères, il protège tous les caractères d'échappement et les
délimiteurs `"..."`.

Ainsi les deux assertions suivantes sont vraies:

```rust
assert_eq!(
    serde_json::to_string("a\nb\"c"),
    Ok("\"a\\nb\\\"c\"") // strings habituelles, il ne faut pas oublier de protéger spéciaux; moins facile à lire 
);
```

```rust
assert_eq!(
    serde_json::to_string("a\nb\"c"),
    Ok(r#""a\nb\"c""#) // raw string pas besoin de protéger les caractères; plus simple à lire
);
```

## Exécution sur macos

Sur macOS, l'exécution du binaire fourni peut-être annulé "pour votre sécurité" (d'après macOS).

Pour y remédier, vous pouvez:

1. Sur votre Mac, choisissez le menu Pomme  > Réglages Système, puis cliquez sur « Confidentialité et sécurité » dans la
   barre latérale. (Vous devrez peut-être faire défiler la page vers le bas.)

2. Accédez à Sécurité, puis cliquez sur Ouvrir.

3. Cliquez sur « Ouvrir quand même ».

    Ce bouton s’affiche pendant environ une heure après que vous avez essayé d’ouvrir l’app.

4. Saisissez votre mot de passe d’ouverture de session, puis cliquez sur OK.