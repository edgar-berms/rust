# Utilisation du serveur de référence

Une fois téléchargé et dézippé, vous pouvez le lancer depuis la ligne de commandes (sur certains OS, il faudra peut-être
ajuster les permissions d'exécution).

# Les options en ligne de commandes

Une aide en ligne de commande est disponible:

```bash
$ ./server --help
```

```
Usage: server [OPTIONS] <COMMAND>

Commands:
  run      Run server
  version  Show detailed version
  decode   Decoding tools
  encode   Encoding tools
  generate Generating tools
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Display detail

          log will show code path

      --debug
          Log additional debug info

      --trace
          Log additional details

  -h, --help
          Print help (see a summary with '-h')
```

Lors de la mise au point, pour voir quelques détails sur la communication worker<->server, l'option `--debug` est
recommandée.

Il existe ainsi plusieurs sous-commandes, la plus important est celle nommée `run` (je vous laisse découvrir les autres
de la même manière):

```
./server run --help
```

```
Run server

Usage: server run [OPTIONS]

Options:
      --port <PORT>
          Port to listen to

          [default: 8778]

      --host-address <HOST_ADDRESS>
          Address allowed to connect to.

          Should be different from localhost to allow external connections

          [default: localhost]

      --screen <SCREEN>
          Initial screen size (width,height) at startup

          [default: 800,800]

      --gui <GUI>
          Visualisation mode

          [default: wgpu]
          [possible values: none, wgpu, websocket]
          
      --maze <MAZE>
          Maze size (width,height)

          [default: 50,50]

      --seed <SEED>
          Random generator seed

      --generator <GENERATOR>
          Random generator seed

          [default: recursive-backtracking]
          [possible values: recursive-backtracking, prims, ellers, growing-tree]

      --refresh-rate <REFRESH_RATE>
          [default: 1]

      --output-maze <OUTPUT_MAZE>
          Generate grid and export it

      --team-size <TEAM_SIZE>
          Team size

          [default: 3]
          
  -h, --help
          Print help (see a summary with '-h')
```

Par défaut, l'interface réseau d'écoute est celle associée à `localhost`. Ainsi, si vous souhaitez l'utiliser sur un
réseau (local par exemple), il faudra lui préciser l'interface d'écoute sous la forme:

```bash
$ ./server run --host-address=192.168.1.99
```

(l'adresse n'est qu'un exemple)

## Les commandes

- <kbd>P</kbd>: add a local player

- <kbd>H</kbd>: display historical path

- <kbd>R</kbd>: reset game (same maze, remove all players)

- <kbd>C</kbd>: clear historical path

- <kbd>escape</kbd>: Quit

## Environnements testés:

* Windows: pas besoin d'installation complémentaire; doit fonctionner *out of the box*.

* macOS: pas besoin d'installation complémentaire; doit fonctionner *out of the box*.

* Linux: si ce n'est pas déjà le cas, vous aurez besoin de driver OpenGL.

  D'expérience, sur Ubuntu, j'ai eu à installer les packages suivants:

  ```bash
  apt install libegl1 libegl1-mesa
  ```

## Exemple de partie visualisée avec le serveur

<img src="images/random_maze_in_action.png" width="200">
  
## Exemple de sortie debug

```
$ server --debug run
```

```
2024-11-17T22:36:30.163431Z DEBUG Read message size: 29
2024-11-17T22:36:30.163450Z DEBUG Read string message: {"Action":{"MoveTo":"Front"}}
2024-11-17T22:36:30.163455Z DEBUG Read struct message: Action(MoveTo(Front))
2024-11-17T22:36:30.163459Z DEBUG Action for 'hungry_durian/Player#1'
2024-11-17T22:36:30.163475Z DEBUG Player { player_id: 0, team_id: 0 } at (49, 49) towards East with encoded view jiucAjGa//cpapa
#######
#######
•-•-•##
|  G|##
• • •##
  | |##
•-• •##

2024-11-17T22:36:30.163483Z DEBUG Write struct message: RadarView(EncodedRadarView("jiucAjGa//cpapa"))
2024-11-17T22:36:30.163488Z DEBUG Write message size: 31
2024-11-17T22:36:30.163491Z DEBUG Write string message: {"RadarView":"jiucAjGa//cpapa"}
```