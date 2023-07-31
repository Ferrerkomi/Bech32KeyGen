<h2> Bech32KeyGen </h2>

'Bech32KeyGen' est un projet Rust permettant de générer des paires de clés cryptographiques sécurisées en utilisant l'algorithme de courbe elliptique SECP256K1. Le projet utilise également le schéma d'encodage Bech32 pour représenter les clés publiques et privées de manière lisible et compacte.

<h2> Aperçu </h2>

La génération de clés cryptographiques est une étape cruciale pour sécuriser les systèmes d'information. Bech32KeyGen facilite la création de paires de clés privées et publiques en utilisant l'algorithme de courbe elliptique SECP256K1, largement utilisé dans les systèmes blockchain tels que Bitcoin & Ethereum.

Le schéma d'encodage Bech32 est utilisé pour représenter les clés publiques et privées sous forme lisible par l'humain et compacte. Cela permet aux utilisateurs de facilement partager, afficher et copier les clés sans ambiguïté.

<h2> Fonctionnalités </h2>

- Génération sécurisée de clés publiques et privées en utilisant SECP256K1.
- Encodage des clés publiques et privées au format Bech32 pour une meilleure lisibilité.
- Affichage des clés générées dans la console.

<h2> Dépendances </h2>

Pour exécuter le projet Bech32KeyGen, vous devez avoir les dépendances suivantes installées sur votre système :
- Rust

Le projet utilise également les bibliothèques externes suivantes :

- rand : Génération de nombres aléatoires sécurisés.
- secp256k1 : Implémentation de la cryptographie à courbes elliptiques SECP256K1 & la génération de clés publiques et privées.
- bech32 : Encodage et décodage Bech32.
- hex : Pour la représentation hexadécimale des clés privées.

<h2> Installation </h2>

Assurez-vous d'avoir Rust installé sur votre système. Si ce n'est pas le cas, suivez les instructions d'installation à partir du site officiel de Rust.
Clonez ce dépôt sur votre machine locale :

```bash
git clone https://github.com/Ferrerkomi/Bech32KeyGen.git
```

1- Accédez au répertoire du projet :

```bash
cd Bech32KeyGen
```

2- Compilez le projet en exécutant :

```bash
cargo build --release
```

<h2> Utilisation </h2>

Une fois le projet compilé, exécutez le fichier binaire pour générer une paire de clés:

```bash
./target/release/Bech32KeyGen
```

Vous verrez la clé publique et la clé privée générées au format Bech32, ainsi que la clé privée en format hexadécimal.

<h2> Exemples </h2>

- Exemple de génération de clés :

```bash
Public Key (bech32): npub1qt65s5v3nj0v9sej4lpqchvkyd48skrwwetvtdjy7z9t7yuxk889gfd48q0
Secret Key (bech32): nsec1me7numqrs3ayecq3u9musvukm4c6g83hghtnuyzqy5n6dqp8dexqmx04lv
Private Key (hex): de7d3e6c03847a4ce011e177c83396dd71a41e3745d73e10402527a680276e4c
```

<h2> Utilité et Importance </h2>

L'utilitaire keygen joue un rôle essentiel dans les applications qui nécessitent une génération sécurisée de clés cryptographiques. La courbe elliptique SECP256K1 est largement utilisée, notamment dans les systèmes de blockchain tels que Bitcoin et Ethereum, pour différentes opérations cryptographiques telles que les signatures numériques et l'échange de clés.

L'utilitaire offre les avantages suivants :

- Sécurité : L'utilisation de rand::thread_rng() garantit que la clé secrète est générée à l'aide de nombres aléatoires cryptographiquement sécurisés, réduisant ainsi le risque de prévisibilité et de collisions de clés.

- Format Compact et Lisible : Le schéma d'encodage Bech32 permet de représenter les clés publiques et privées de manière lisible par l'humain et compacte. Cela est utile lors du partage des clés sous forme imprimée ou par des canaux vocaux, car cela réduit les erreurs lors de la saisie manuelle.

- Compatibilité : L'algorithme de courbe elliptique SECP256K1 est largement adopté, rendant les clés générées compatibles avec différentes bibliothèques et applications cryptographiques prenant en charge cette norme.

- Gestion des Erreurs : Le code gère les erreurs potentielles de manière élégante. Il utilise unwrap() dans cet exemple, mais dans un environnement de production, vous devriez gérer les erreurs de manière plus soigneuse pour éviter les plantages du programme.

<h2> Cas d'exemple NIP-19 (Nostr) </h2>

'NIP-19' est un cas d'exemple qui standardise les chaînes formatées en Bech32 pouvant être utilisées pour afficher des clés, des identifiants et d'autres informations dans les clients Nostr. Ces formats ne sont pas destinés à être utilisés dans le protocole de base, mais uniquement pour l'affichage aux utilisateurs, la copie-coller, le partage, la création de codes QR & la saisie de données.

<details>
  
  _<summary><b>Identifiants Partageables avec des Métadonnées Supplémentaires</b></summary>_
  
Le NIP-19 standardise les chaînes formatées en Bech32 pouvant être utilisées pour afficher des clés, des identifiants et d'autres informations dans les clients. Ces formats ne sont pas destinés à être utilisés dans le protocole principal, mais plutôt à des fins d'affichage convivial pour les utilisateurs, de copier-coller, de partage, de génération de codes QR et de saisie de données.

Il est recommandé de stocker les identifiants et les clés soit au format hexadécimal, soit au format binaire, car ces formats sont plus proches de ce qui doit être réellement utilisé dans le protocole principal.

<details>
  
  _<summary><b>Clés et Identifiants Bruts</b></summary>_
  
Pour éviter toute confusion et mélange entre les clés privées, les clés publiques et les identifiants d'événements, qui sont tous des chaînes de 32 octets, un encodage Bech32 avec des préfixes différents peut être utilisé pour chaque entité.

Voici les préfixes Bech32 possibles:

- npub : clés publiques
- nsec : clés privées
- note : identifiants d'événements

Par exemple, la clé publique hexadécimale 3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d se traduit en npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6.

Les encodages Bech32 des clés et des identifiants ne doivent pas être utilisés à l'intérieur des formats d'événements standard NIP-01 ou à l'intérieur des filtres. Ils sont destinés uniquement à un affichage et une saisie plus conviviaux pour l'utilisateur. Les clients doivent toujours accepter les clés au format hexadécimal (npub) ainsi que les formats Bech32, et les convertir en interne au besoin.

</details>

<details>
  
  _<summary><b>Identifiants Partageables avec des Métadonnées Supplémentaires</b></summary>_
  
Lors du partage d'un profil ou d'un événement, une application peut décider d'inclure des informations de relais et d'autres métadonnées pour que d'autres applications puissent localiser et afficher ces entités plus facilement.

Pour ces événements, les contenus sont une liste encodée en binaire de TLV (type-length-value), avec T et L étant chacun sur 1 octet (uint8, c'est-à-dire un nombre dans la plage de 0 à 255), et V étant une séquence d'octets de la taille indiquée par L.

Les préfixes Bech32 possibles avec TLV sont :

**nprofile:** un profil nostr.

**nevent:** un événement nostr.

**nrelay:** un relais nostr.

**naddr:** une coordonnée d'événement remplaçable paramétrée nostr (NIP-33)

Voici les types TLV normalisés possibles :

- _0 : special_

  - dépend du préfixe Bech32 :
  
    - pour nprofile, il s'agira des 32 octets de la clé publique du profil

    - pour nevent, il s'agira des 32 octets de l'identifiant d'événement

    - pour nrelay, il s'agit de l'URL du relais

    - pour naddr, il s'agit de l'identifiant (la balise "d") de l'événement référencé

- _1 : relay_
  
  - pour nprofile, nevent et naddr, éventuellement, un relais dans lequel l'entité (profil ou événement) est plus susceptible d'être trouvée, encodé en ASCII
cela peut être inclus plusieurs fois

- _2 : author_
  
  - pour naddr, les 32 octets de la clé publique de l'événement
  - pour nevent, éventuellement, les 32 octets de la clé publique de l'événement

- _3 : kind_
  
  - pour naddr, l'entier non signé de 32 bits de la catégorie, big-endian
  - pour nevent, éventuellement, l'entier non signé de 32 bits de la catégorie, big-endian

</details>
<h2>Exemples</h2>
<ul>
  <li><b>Clé publique :</b> npub10elfcs4fr0l0r8af98jlmgdh9c8tcxjvz9qkw038js35mp4dma8qzvjptg</li>
  <li><b>Clé publique hexadécimale :</b> 7e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e (et vice-versa)</li>
  <li><b>Clé privée :</b> nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5</li>
  <li><b>Clé privée hexadécimale :</b> 67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa (et vice-versa)</li>
  <li>
    <b>Profil avec les éléments TLV :</b>
    <ul>
      <li><b>Clé publique :</b> 3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d</li>
      <li><b>Relais :</b> wss://r.x.com</li>
      <li><b>Relais :</b> wss://djbas.sadkb.com</li>
      </ul>
  </li>
</ul>
<h2>Notes</h2>
<ul>
  <li>Les clés `npub` NE DOIVENT PAS être utilisées dans les événements NIP-01 ou dans les réponses JSON NIP-05, seule la forme hexadécimale est prise en charge là-bas.</li>
  <li>Lors du décodage d'une chaîne formatée en Bech32, les `TLV` qui ne sont pas reconnus ou pris en charge doivent être ignorés, plutôt que de provoquer une erreur.</li>
</ul>
</details>

<h2> Conclusion </h2>

Le projet 'Bech32KeyGen' est un utilitaire précieux qui permet de générer des paires de clés cryptographiques sécurisées en utilisant l'algorithme de courbe elliptique SECP256K1. Son utilisation dans un cas d'exemple NIP-19 met en évidence son rôle crucial dans les applications blockchain et les protocoles de sécurité. En utilisant l'encodage Bech32, le projet Bech32KeyGen offre une représentation lisible et compacte des clés, ce qui facilite leur partage et leur utilisation. La génération sécurisée de clés est essentielle dans des domaines tels que la blockchain, le chiffrement, la messagerie sécurisée et la gestion d'identité, car elle garantit la confidentialité et l'intégrité des données. Les développeurs souhaitant renforcer la sécurité de leurs applications peuvent envisager d'intégrer le projet 'Bech32KeyGen' dans leur base de code, en veillant à suivre les meilleures pratiques en matière de sécurité et de gestion des clés.
