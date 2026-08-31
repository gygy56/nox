# Nox — Spécification

> Document de référence du langage Nox.
> Nox est en phase de conception. Les éléments marqués « À définir plus tard » ne sont pas des décisions finales.

## 1. Identité

- Nom : Nox
- Extension : .nx
- Statut : conception
- Objectif : langage simple et lisible, syntaxe inspirée de Lua/Python, performances natives, contrôle bas niveau, gestion mémoire sûre et interopérabilité C.

### Critères de conception

Chaque décision est évaluée selon :
1. simplicité
2. performance
3. contrôle
4. sécurité
5. cohérence
6. facilité d’implémentation

Nox ne cherche pas à copier Python, Lua, C ou Rust.

## 2. Syntaxe générale

Les blocs utilisent obligatoirement {}.

Exemple :
fn main() {
    printf("Hello Nox")
}

L’indentation n’est pas significative.

Les ; sont autorisés mais optionnels.

Exemple :
stck x = 10
stck y = 20;

Les parenthèses inutiles sont autorisées.

Exemple :
stck x = (((42)))

Les trailing commas sont autorisées lorsqu’une syntaxe les accepte.

## 3. Commentaires

Commentaire sur une ligne :
// commentaire

Commentaire multiligne :
/*
   commentaire
   multiligne
*/

## 4. Identifiants

Un identifiant commence par une lettre ou _, puis peut contenir des lettres, chiffres et _.

Les mots-clés réservés ne peuvent pas être utilisés comme identifiants.

## 5. Variables

### stck

stck signifie « stock ».

Il déclare une variable mutable avec type inféré.

Exemple :
stck age = 16
stck name = "Emin"

stck n’implique aucun placement sur la stack : le placement mémoire est décidé par le compilateur.

Une variable ne change jamais de type après sa déclaration.

### const

const déclare une valeur immuable.

Exemple :
const MAX = 32

Il n’existe pas de troisième mot-clé de variable normale comme let ou mut.

mut est réservé à mut ref<T>.

## 6. Portées

Les blocs créent des portées.

Une variable est accessible dans sa portée et ses portées enfants, mais plus après la fin de sa portée.

Une redéclaration dans la même portée est interdite.

Le shadowing est autorisé dans une portée enfant.

Exemple :
stck x = 10

if true {
    stck x = 20
}

La variable enfant masque celle du parent sans la modifier.

Les variables globales et constantes globales sont autorisées.

## 7. Types primitifs

Entiers signés :
i8
i16
i32
i64

Entiers non signés :
u8
u16
u32
u64

Flottants :
f32
f64

Autres :
bool
char
str
void

i128 et u128 ne sont pas présents pour l’instant.

void représente l’absence de valeur de retour et n’est pas utilisable comme type de variable pour l’instant.

## 8. Littéraux numériques

Supportés :
- décimal
- hexadécimal avec 0x
- binaire avec 0b
- octal avec 0o
- séparateurs _

Exemples :
42
1_000_000
0xFF
0b1010
0o755

Flottants et notation scientifique :
3.14
1.0
1e10
2.5e-3

Suffixes flottants :
1.0f32
1.0f64

Suffixes entiers :
42i64
42u32

-42 est l’opérateur unaire - appliqué au littéral 42.

## 9. Bool, char et str

### Booléens

true
false

Les conditions et opérations qui exigent un booléen utilisent strictement bool.

Aucun truthy/falsy implicite.

### char

'A'
'\n'

Les escapes classiques sont supportés.

La représentation interne de char sera définie plus tard.

### str

"Hello"
"Hello\nWorld"

Les escapes classiques sont supportés.

Interpolation :
stck name = "Emin"
printf("Hello {name}")

La représentation interne, l’encodage, la mutabilité et les chaînes multilignes seront définis plus tard.

## 10. Conversions

Aucune conversion implicite entre types numériques.

Exemples nécessitant une conversion explicite :
i32 → i64
f32 → f64
signé → non signé

Syntaxe :
x as i64

Les littéraux numériques abstraits peuvent s’adapter au type attendu s’ils sont représentables.

Deux valeurs déjà typées différemment ne peuvent pas être combinées implicitement.

## 11. Arithmétique

+   addition
-   soustraction
*   multiplication
/   division flottante
//  division entière
%   reste

/ donne toujours une division flottante.

// donne une division entière.

Avec des opérandes flottants, / reste toujours une division flottante.

## 12. Assignations

=
+=
-=
*=
/=
%=

Les assignations sont des statements, pas des expressions.

Exemple :
stck x = 10
x = 20
x += 5
x -= 2
x *= 3
x /= 2
x %= 4

Les assignations composées doivent produire une valeur compatible avec le type de la variable.

a = b = c est interdit.

## 13. Comparaisons

Toutes les comparaisons sont au même niveau de priorité.

Elles retournent bool et sont associatives à gauche.

Le comportement de x < y < z reste à définir plus tard.

## 14. Logique

and
or
not

Ils utilisent bool.

Une condition doit être strictement bool.

Exemple :
if x == true {
    ...
}

## 15. Opérateurs bit à bit

&
|
^
~
shl
shr

& est également utilisé pour créer une référence ; la grammaire devra distinguer les usages.

Le XOR logique est prévu.

## 16. Priorité

Ordre général :

1. opérateurs unaires
2. * / // %
3. + -
4. comparaisons
5. and
6. or
7. assignations

Arithmétique, comparaisons et logique : associativité gauche.

Les assignations ne sont pas des expressions.

## 17. Conditions

Syntaxe :
if condition {
    ...
} elseif condition {
    ...
} else {
    ...
}

elseif est un mot-clé unique.

Les {} sont obligatoires.

## 18. Boucles

### while

while condition {
    ...
}

### for

for i in 0..10 {
    printf(i)
}

### loop

loop {
    ...
}

### break

break

### continue

continue

Les boucles descendantes et step sont à définir plus tard.

## 19. Ranges

0..10  → borne finale exclusive
0..=10 → borne finale inclusive

Donc :
0..10  → 0 à 9
0..=10 → 0 à 10

Pour l’instant 10..0 est une erreur.

## 20. Tableaux

Syntaxe :
i32[3]

Exemple :
i32[3] numbers = [1, 2, 3]

La taille est connue à la compilation.

Un dépassement avec index statique peut être détecté à la compilation.

Un index dynamique est vérifié à l’exécution.

Les index commencent à 0.

Exemple :
numbers[0]

L’inférence est supportée.

Exemple :
stck numbers = [1, 2, 3]

Les collections dynamiques sont à définir plus tard.

## 21. Structs

Les structs sont des types valeur.

Exemple :
struct Player {
    str name
    i32 hp
}

Création :
stck player = Player {
    name: "Emin"
    hp: 100
}

Accès :
printf(player.name)
printf(player.hp)

Modification :
player.hp = 80

Une copie crée une valeur indépendante.

Les références sont séparées.

## 22. Visibilité

pub → public
prv → privé explicitement
sans mot-clé → visibilité héritée du contexte

Cette règle remplace la règle précédente « privé par défaut ».

## 23. Méthodes et constructeurs

Les méthodes utilisent impl.

Exemple :
impl Player {
    fn damage(self: mut ref<Player>, amount: i32) {
        self.hp -= amount
    }
}

Les constructeurs explicites sont supportés.

Direction :
Player.new(...)

La sémantique détaillée reste à définir.

## 24. Enums

Les enums simples sont supportés.

Exemple :
enum Color {
    Red
    Green
    Blue
}

Les enums avec données sont supportés.

Exemple :
enum Result<T, E> {
    Ok(T)
    Err(E)
}

## 25. match

match est exhaustif.

Une variante non traitée provoque une erreur de compilation.

Les détails du pattern matching restent à définir.

## 26. Optionnels

Syntaxe :
T?

Absence :
stck value: i32? = nil

nil est uniquement compatible avec un type optionnel.

Après :
if value != nil {
    ...
}

le compilateur peut raffiner automatiquement value vers T dans le contexte approprié.

Le déballage et les cas limites sont à définir plus tard.

## 27. Références

ref<T> → référence en lecture seule
mut ref<T> → référence modifiable

Création :
stck x: i32 = 42
stck r: ref<i32> = &x

La déréférenciation est automatique dans l’utilisation normale.

### Borrowing

Direction retenue :

plusieurs ref<T>
OU
une seule mut ref<T>

Une référence ne peut pas survivre à la valeur référencée.

Une violation provoque une erreur de compilation.

Les règles complètes d’ownership, de move, d’aliasing et de destruction sont à définir plus tard.

## 28. Pointeurs

Les pointeurs bas niveau sont prévus.

Leur syntaxe et leur sémantique sont à définir plus tard.

Les opérations dangereuses liées aux pointeurs utilisent unsafe.

Exemple :
unsafe {
    ...
}

## 29. Mémoire

Nox n’utilisera pas de Garbage Collector global.

La direction retenue est une gestion mémoire basée sur ownership/borrowing.

Le modèle exact est à définir plus tard.

## 30. Fonctions

Syntaxe :
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

Les paramètres sont obligatoirement typés.

Pas de paramètres par défaut.

Pas de fonctions variadiques dans le langage de base.

Les lambdas sont à définir plus tard.

## 31. Paramètres mutables

La modification d’une valeur externe passe par mut ref<T>.

Exemple :
fn increment(x: mut ref<i32>) {
    x += 1
}

Il n’existe pas de second système de paramètres mutables.

## 32. Retour

Retour explicite :
return value

Retour implicite par dernière expression :
fn add(a: i32, b: i32) -> i32 {
    a + b
}

Les cas limites sont à définir plus tard.

## 33. Génériques

Les génériques sont supportés.

Exemple :
fn identity<T>(value: T) -> T {
    value
}

Syntaxe :
<T>

Les contraintes sont supportées.

Exemple :
fn max<T: Comparable>(a: T, b: T) -> T {
    ...
}

Les traits sont supportés.

Exemple :
trait Comparable {
    ...
}

Les détails avancés sont à définir plus tard.

## 34. Imports

Import de module :
import math

Alias :
import math as m

Import sélectif :
import math.sqrt

Plusieurs imports utilisent une ligne par import.

## 35. Packages

Nox aura un système de packages.

Le format et le gestionnaire de packages seront définis plus tard.

## 36. Gestion des erreurs

Syntaxe :
try
catch
throw

try/catch/throw et Result sont des systèmes séparés.

L’implémentation interne et le coût runtime sont à définir plus tard.

## 37. Result

Nox prévoit :
Result<T, E>

L’opérateur ? peut propager un Result.

Exemple :
stck value = get_value()?

Les détails restent à définir plus tard.

## 38. is

is permet de vérifier un type.

Exemple :
if value is Player {
    ...
}

Les règles exactes de raffinement seront définies avec le type checker.

## 39. in

in est disponible dans les boucles.

Exemple :
for value in array {
    ...
}

Et dans les conditions :
if value in array {
    ...
}

## 40. defer

Nox supporte defer.

Exemple :
defer {
    close(file)
}

La sémantique précise sera définie avec le runtime.

## 41. Compile-time

Les constantes peuvent être calculées à la compilation.

Exemple :
const SIZE = 10 + 20

Un mécanisme comptime est prévu.

Sa sémantique complète sera définie plus tard.

## 42. Standard library

Fonction d’affichage :
printf("Hello")

printf est l’API d’affichage retenue.

input() est prévu.

Modules standards prévus :
math
fs
net
process

assert appartient à la standard library.

## 43. FFI C

Nox aura une interopérabilité C.

Syntaxe externe :
extern fn foo(...)

Les détails ABI, layout, conventions d’appel et pointeurs seront définis lors de la conception du FFI.

## 44. Fonctionnalités non retenues pour l’instant

- macros
- reflection
- opérateur ternaire
- ++ / --
- lambdas
- collections dynamiques
- ownership détaillé
- pointeurs détaillés
- strings multilignes
- unions
- tuples

Ces fonctionnalités pourront évoluer selon les besoins réels du langage.

## 45. Architecture du compilateur

Pipeline prévu :

.nx
↓
Lexer
↓
Parser
↓
AST
↓
Analyse sémantique
↓
Type checker
↓
IR
↓
Optimisations
↓
Code natif
↓
Linking
↓
Exécutable

Le choix d’utiliser LLVM ou une autre technologie n’est pas encore fixé.

## 46. Architecture du projet

Nox
├── langage
│   ├── syntaxe
│   ├── types
│   ├── mémoire
│   └── standard library
├── compilateur
│   ├── lexer
│   ├── parser
│   ├── AST
│   ├── analyse sémantique
│   ├── type checker
│   ├── IR
│   ├── optimisations
│   └── code generation
├── runtime
├── outils
│   ├── compiler
│   ├── formatter
│   ├── package manager
│   └── debugger
└── tests

Cette architecture est une direction générale et peut évoluer.

## 47. Méthode de développement

Ordre de priorité :

comprendre
→ décider
→ documenter
→ implémenter
→ tester

Le projet utilise Git et des commits logiques.

Le code doit rester modulaire, testé et documenté.

Les gros fichiers monolithiques sont à éviter.

## 48. Philosophie

Nox doit combiner :

simplicité
+
performance
+
contrôle
+
sécurité
+
cohérence

Le but n’est pas de tout définir avant de produire le premier exécutable.

La conception et l’implémentation doivent évoluer ensemble :

fondations
→ prototype minimal
→ premier exécutable Nox
→ utilisation réelle
→ retours
→ évolution du langage

## 49. Roadmap

### Phase 1 — Fondations

- syntaxe
- types primitifs
- variables
- constantes
- expressions
- opérateurs
- scopes
- conditions
- boucles
- fonctions
- tableaux fixes
- structs simples
- imports basiques

### Phase 2 — Premier compilateur

Lexer
→ Parser
→ AST
→ analyse sémantique minimale
→ type checker minimal
→ génération de code
→ exécutable

### Phase 3 — Utilisation réelle

Écrire de petits programmes Nox pour détecter les problèmes de conception.

### Phase 4 — Systèmes avancés

- ownership
- borrowing complet
- pointeurs
- Result
- génériques avancés
- traits
- FFI C
- runtime
- packages
- optimisation

### Phase 5 — Outils

- compiler CLI
- formatter
- package manager
- diagnostics avancés
- debugger
- cross-compilation

## 50. Statut

Nox est actuellement en phase de conception des fondations.

Ce document est la source de vérité actuelle du langage.

Les nouvelles décisions doivent être ajoutées à ce fichier avant leur implémentation.