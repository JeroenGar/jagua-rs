window.BENCHMARK_DATA = {
  "lastUpdate": 1767612625230,
  "repoUrl": "https://github.com/JeroenGar/jagua-rs",
  "entries": {
    "Performance Tracker": [
      {
        "commit": {
          "author": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "committer": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "id": "bec3e9d9fcb776cc46acd84a011c45505ad3ef32",
          "message": "automatic performance benchmarks during CI",
          "timestamp": "2025-05-28T20:24:39Z",
          "url": "https://github.com/JeroenGar/jagua-rs/pull/32/commits/bec3e9d9fcb776cc46acd84a011c45505ad3ef32"
        },
        "date": 1748698471282,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 4368327,
            "range": "± 287555",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4838896,
            "range": "± 401745",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5953875,
            "range": "± 536410",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 401521,
            "range": "± 21876",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 297289,
            "range": "± 8100",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 269403,
            "range": "± 4880",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 5528078,
            "range": "± 60090",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9294842,
            "range": "± 42095",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15953785,
            "range": "± 88778",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9bead181e8de9b8e5da2d6dc8273955d1dc8f0ca",
          "message": "Merge pull request #32 from JeroenGar/feat/ci_bench\n\nAutomatic performance benchmarks during CI",
          "timestamp": "2025-05-31T15:35:47+02:00",
          "tree_id": "881a22bd5e096f1a14499aa29abc6bc8a3a6bea6",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9bead181e8de9b8e5da2d6dc8273955d1dc8f0ca"
        },
        "date": 1748698723552,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5490895,
            "range": "± 29690",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9330368,
            "range": "± 42026",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15873289,
            "range": "± 337117",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 402718,
            "range": "± 23990",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 297745,
            "range": "± 7783",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 269839,
            "range": "± 4598",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 4372240,
            "range": "± 288207",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4857438,
            "range": "± 409667",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5954793,
            "range": "± 540637",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "committer": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "id": "db0b5dc5134a5a5d246670e5047f81903479259d",
          "message": "speed improvements in CollidesWith for Edge-Edge and Rect-Edge",
          "timestamp": "2025-05-31T13:35:51Z",
          "url": "https://github.com/JeroenGar/jagua-rs/pull/33/commits/db0b5dc5134a5a5d246670e5047f81903479259d"
        },
        "date": 1748698956416,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5295001,
            "range": "± 57080",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 8977237,
            "range": "± 64816",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15200328,
            "range": "± 124521",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 372703,
            "range": "± 19609",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 288903,
            "range": "± 7084",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 265395,
            "range": "± 4391",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3822542,
            "range": "± 328454",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4579962,
            "range": "± 380479",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5719215,
            "range": "± 517303",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5e917bc4906300270488b7a5c39e76171808f4cf",
          "message": "Merge pull request #33 from JeroenGar/feat/prim_geo\n\nspeed improvements in CollidesWith for Edge-Edge and Rect-Edge",
          "timestamp": "2025-05-31T15:44:28+02:00",
          "tree_id": "b9c05107183b23e3babdac97353618b11b668b9a",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/5e917bc4906300270488b7a5c39e76171808f4cf"
        },
        "date": 1748699215572,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5343600,
            "range": "± 33347",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9027159,
            "range": "± 56628",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15312894,
            "range": "± 233135",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 375005,
            "range": "± 18010",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 289005,
            "range": "± 6832",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 267248,
            "range": "± 4081",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3823217,
            "range": "± 244338",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4594668,
            "range": "± 392191",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5738795,
            "range": "± 519310",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "committer": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "id": "8287c40b2c2017cd426e1c89faa7e0aa1d6db93c",
          "message": "deploy docs on main repo gh pages",
          "timestamp": "2025-05-31T13:44:32Z",
          "url": "https://github.com/JeroenGar/jagua-rs/pull/34/commits/8287c40b2c2017cd426e1c89faa7e0aa1d6db93c"
        },
        "date": 1748699632069,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5252951,
            "range": "± 22475",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 8985020,
            "range": "± 84203",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15151408,
            "range": "± 121465",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 370724,
            "range": "± 19318",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 289474,
            "range": "± 11443",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 272725,
            "range": "± 5001",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3810938,
            "range": "± 245152",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4544329,
            "range": "± 377480",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5681707,
            "range": "± 516866",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "478adc0df552bcc1ec3a4387aeee6b199f7923ed",
          "message": "Merge pull request #34 from JeroenGar/feat/docs_on_main_repo\n\ndeploy docs on main repo gh pages",
          "timestamp": "2025-05-31T15:51:55+02:00",
          "tree_id": "a25e86a10c9c113adf221336587ee8d76f6ebacc",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/478adc0df552bcc1ec3a4387aeee6b199f7923ed"
        },
        "date": 1748699669630,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5293326,
            "range": "± 19169",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 8960931,
            "range": "± 44310",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15291846,
            "range": "± 254035",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 375555,
            "range": "± 18121",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 288292,
            "range": "± 7324",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 266092,
            "range": "± 4175",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3799875,
            "range": "± 234284",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4574729,
            "range": "± 467714",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5698248,
            "range": "± 513456",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "62419536d4c73d510a35c514678dc4743334e2df",
          "message": "README changes to update gh-pages links",
          "timestamp": "2025-05-31T16:20:57+02:00",
          "tree_id": "aafd38a9be8698db3722b8c11f7a905632d26342",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/62419536d4c73d510a35c514678dc4743334e2df"
        },
        "date": 1748701423338,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5283783,
            "range": "± 192652",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 8936577,
            "range": "± 69646",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15261276,
            "range": "± 108654",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 375123,
            "range": "± 18038",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 288610,
            "range": "± 7040",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 266511,
            "range": "± 4175",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3784054,
            "range": "± 235729",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4472569,
            "range": "± 354383",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5650497,
            "range": "± 513179",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "9e14d3a3e2f8508c6dcdedbc07ff0e8c204891f2",
          "message": "README changes to update gh-pages links",
          "timestamp": "2025-05-31T16:33:16+02:00",
          "tree_id": "e01a57f9450e9fe0b1903b4ae0b027ebd0ebf3d9",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9e14d3a3e2f8508c6dcdedbc07ff0e8c204891f2"
        },
        "date": 1748702154836,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5297247,
            "range": "± 25744",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9039428,
            "range": "± 77579",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15270560,
            "range": "± 135155",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 374322,
            "range": "± 18012",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 288998,
            "range": "± 7537",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 268106,
            "range": "± 10297",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3917321,
            "range": "± 253409",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4635348,
            "range": "± 383182",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5783350,
            "range": "± 519562",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "committer": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "id": "d6bb4ce2d0544e54bae970dcb0d420665a34a4be",
          "message": "elim Weak<T> in QT + changes to PartialHaz CD flow",
          "timestamp": "2025-05-31T14:33:22Z",
          "url": "https://github.com/JeroenGar/jagua-rs/pull/35/commits/d6bb4ce2d0544e54bae970dcb0d420665a34a4be"
        },
        "date": 1748737222508,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 3419713,
            "range": "± 206207",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4555225,
            "range": "± 390883",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5802036,
            "range": "± 527553",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "committer": {
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "id": "aee4f157db09c29bcaf52fbab5cd047a77794a6e",
          "message": "elim Weak<T> in QT + changes to PartialHaz CD flow",
          "timestamp": "2025-05-31T14:33:22Z",
          "url": "https://github.com/JeroenGar/jagua-rs/pull/35/commits/aee4f157db09c29bcaf52fbab5cd047a77794a6e"
        },
        "date": 1748737498383,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_update_1k/3",
            "value": 5188361,
            "range": "± 164230",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 8896385,
            "range": "± 53027",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15015483,
            "range": "± 106728",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 318098,
            "range": "± 13503",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 278499,
            "range": "± 6741",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 262561,
            "range": "± 4918",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/3",
            "value": 3327571,
            "range": "± 205435",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 4369502,
            "range": "± 419471",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 5643489,
            "range": "± 517889",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "367b694405d54d68b88fd791d8269cfd8e457e30",
          "message": "Merge pull request #35 from JeroenGar/feat/qt_changes\n\nelim Weak<T> in QT + changes to PartialHaz CD flow",
          "timestamp": "2025-06-01T19:21:19+02:00",
          "tree_id": "9354303ba39bda53f00543b1c3ed3d4f75db19cb",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/367b694405d54d68b88fd791d8269cfd8e457e30"
        },
        "date": 1748798638893,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2908813,
            "range": "± 145088",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3339778,
            "range": "± 235893",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3371281,
            "range": "± 237951",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 5536648,
            "range": "± 28335",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9440885,
            "range": "± 46748",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15757265,
            "range": "± 89465",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 308162,
            "range": "± 8515",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 271494,
            "range": "± 5932",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 259713,
            "range": "± 8897",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "086eda01825d9ed82290b8aa614c12f127fc2978",
          "message": "Merge pull request #36 from JeroenGar/feat/qt_changes\n\nMore small refinements",
          "timestamp": "2025-06-01T23:11:51+02:00",
          "tree_id": "0e27c7cdac9ff04513a65092628c1cc2d1baef31",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/086eda01825d9ed82290b8aa614c12f127fc2978"
        },
        "date": 1748812463614,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2817858,
            "range": "± 148118",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3264213,
            "range": "± 254783",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3297088,
            "range": "± 287310",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 5546750,
            "range": "± 26135",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9308016,
            "range": "± 60734",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 16049969,
            "range": "± 90170",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 305118,
            "range": "± 7826",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 268668,
            "range": "± 5638",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 258121,
            "range": "± 4232",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "9dfc0878aa02038f18814884d90bf1169c8f3be5",
          "message": "bench.yml minor changes",
          "timestamp": "2025-06-01T23:47:27+02:00",
          "tree_id": "1a1d8034796c4c29db048ecd503b77bf7d0d732f",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9dfc0878aa02038f18814884d90bf1169c8f3be5"
        },
        "date": 1748814601257,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2810276,
            "range": "± 150106",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3274567,
            "range": "± 248413",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3301163,
            "range": "± 248888",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 5549963,
            "range": "± 23968",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9374319,
            "range": "± 48637",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15666474,
            "range": "± 323801",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 306887,
            "range": "± 7885",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 269822,
            "range": "± 6144",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 263951,
            "range": "± 3957",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "62b086ed1daf8f02d09ffc674ce116752554c90f",
          "message": "removed unused function",
          "timestamp": "2025-06-02T22:21:38+02:00",
          "tree_id": "fd54bd4bfcd38ee31d6100c777f3f85764217d76",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/62b086ed1daf8f02d09ffc674ce116752554c90f"
        },
        "date": 1748895913306,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2816519,
            "range": "± 149919",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3277663,
            "range": "± 248237",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3301391,
            "range": "± 247697",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 5483507,
            "range": "± 46170",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 9250433,
            "range": "± 78677",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 15546120,
            "range": "± 73136",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 305387,
            "range": "± 8023",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 269960,
            "range": "± 5683",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 259054,
            "range": "± 4415",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9718a0bdd7ade567439059cc3682af7937ef0a1b",
          "message": "Merge pull request #37 from JeroenGar/feat/cd_config\n\nfixed a bug where `QTHazPresence::None` hazards were propagated down in the quadtree, hindering performance",
          "timestamp": "2025-06-04T05:47:51+02:00",
          "tree_id": "b525b8abde4c510da325bcd7359c00a96b70b2f8",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9718a0bdd7ade567439059cc3682af7937ef0a1b"
        },
        "date": 1749009020308,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2667462,
            "range": "± 193145",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3119444,
            "range": "± 214286",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3103879,
            "range": "± 235988",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3438855,
            "range": "± 18280",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6149004,
            "range": "± 337103",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11231504,
            "range": "± 200684",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 293479,
            "range": "± 7697",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 263173,
            "range": "± 5451",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 246533,
            "range": "± 3055",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "f5cf20b545301a93cf40c391425ac78f1f88e04b",
          "message": "v0.6.0",
          "timestamp": "2025-06-04T05:52:35+02:00",
          "tree_id": "a489d2beae613b850c02efc05e361f94aec6a280",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/f5cf20b545301a93cf40c391425ac78f1f88e04b"
        },
        "date": 1749009306197,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2659780,
            "range": "± 142846",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3115191,
            "range": "± 221726",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3094814,
            "range": "± 238515",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3434487,
            "range": "± 19559",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6079135,
            "range": "± 30161",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11110137,
            "range": "± 123988",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 294176,
            "range": "± 7337",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 262562,
            "range": "± 5176",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 247348,
            "range": "± 3077",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "a47644bc30359f3629f721be6c509567bfb4191e",
          "message": "improved error handling",
          "timestamp": "2025-06-04T14:08:24+02:00",
          "tree_id": "00babb6b51c61dac23439b0816b1d04ef6bf3087",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/a47644bc30359f3629f721be6c509567bfb4191e"
        },
        "date": 1749039058356,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2608931,
            "range": "± 133528",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3094123,
            "range": "± 213604",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3084567,
            "range": "± 205760",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3473772,
            "range": "± 18089",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6185174,
            "range": "± 39467",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11328323,
            "range": "± 77178",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 293893,
            "range": "± 7427",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 263037,
            "range": "± 12958",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 247668,
            "range": "± 3206",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "90a4f9fcd190e0d770e9c34ceab7411376fe9bea",
          "message": "cargo clippy",
          "timestamp": "2025-06-04T14:14:13+02:00",
          "tree_id": "7baa4ea399fd1d758489034bd08ee396fa4e5c93",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/90a4f9fcd190e0d770e9c34ceab7411376fe9bea"
        },
        "date": 1749039407985,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2601213,
            "range": "± 131888",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3086494,
            "range": "± 212637",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3105717,
            "range": "± 207446",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3478344,
            "range": "± 18664",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6109250,
            "range": "± 26479",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11147545,
            "range": "± 69214",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 294395,
            "range": "± 7455",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 262853,
            "range": "± 5319",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 248527,
            "range": "± 3212",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "bad0b0df2d4645dfd11e173addf13df0d58fd957",
          "message": "SVG export fixes:\n- removed `xlink` (deprecated)\n- fixed SvgDrawOptions deserialization defaults",
          "timestamp": "2025-06-04T16:04:06+02:00",
          "tree_id": "44456b9295c71e40e1a8f5abdc1a658aecab9018",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/bad0b0df2d4645dfd11e173addf13df0d58fd957"
        },
        "date": 1749046044426,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2605619,
            "range": "± 133024",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3078786,
            "range": "± 212554",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3085169,
            "range": "± 208820",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3495782,
            "range": "± 29171",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6083275,
            "range": "± 44606",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11289217,
            "range": "± 77303",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 293480,
            "range": "± 7950",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 262569,
            "range": "± 5172",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 247964,
            "range": "± 3367",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "46be4227de6560de07cb76cdb8bfab31ecb6f81e",
          "message": "small fix",
          "timestamp": "2025-06-04T16:10:48+02:00",
          "tree_id": "7f7d9970d77f24cbaed9d2dea5c75501e330c6c8",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/46be4227de6560de07cb76cdb8bfab31ecb6f81e"
        },
        "date": 1749046428076,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2601600,
            "range": "± 150465",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3081022,
            "range": "± 202769",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3085703,
            "range": "± 205292",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3452038,
            "range": "± 27794",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6056698,
            "range": "± 31100",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11093211,
            "range": "± 146659",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 294708,
            "range": "± 8040",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 263497,
            "range": "± 5258",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 248366,
            "range": "± 3231",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "34c3dd35863fb027ea0380f74bfa33fb667409ae",
          "message": "Merge pull request #38 from JeroenGar/feat/sep-dist-rust",
          "timestamp": "2025-06-10T09:58:06+02:00",
          "tree_id": "122ed38ab5af8057abd88696634d072eec356cf6",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/34c3dd35863fb027ea0380f74bfa33fb667409ae"
        },
        "date": 1749542446359,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2659836,
            "range": "± 141164",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3115766,
            "range": "± 240762",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3104612,
            "range": "± 246674",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3506451,
            "range": "± 25837",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 6071994,
            "range": "± 42376",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 11253108,
            "range": "± 82602",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 295160,
            "range": "± 7612",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 263047,
            "range": "± 5324",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 251894,
            "range": "± 3587",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1eb33961620d8325081fe529f1afeb8d924c916e",
          "message": "Merge pull request #39 from JeroenGar/feat/speed\n\nEliminating another pointer deref",
          "timestamp": "2025-06-11T00:11:14+02:00",
          "tree_id": "99b8e50e96032812b717ad87558e37ea80f07fec",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/1eb33961620d8325081fe529f1afeb8d924c916e"
        },
        "date": 1749593642476,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2507677,
            "range": "± 137080",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2981358,
            "range": "± 200600",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2987052,
            "range": "± 201812",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 3034213,
            "range": "± 68735",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 5188441,
            "range": "± 26608",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 9786060,
            "range": "± 96482",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 270683,
            "range": "± 6988",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 241450,
            "range": "± 4823",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 234263,
            "range": "± 3277",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dc667ae5fca668ef2dfbb4700120b2ffc1753410",
          "message": "Merge pull request #40 from JeroenGar/feat/cde_rewrite\n\nCDE now uses SlotMap<HazKey,Hazard>",
          "timestamp": "2025-06-12T10:18:27+02:00",
          "tree_id": "037734919e5b3cd44d645ba699439f5c1c43f664",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/dc667ae5fca668ef2dfbb4700120b2ffc1753410"
        },
        "date": 1749716473644,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2548215,
            "range": "± 139634",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3022810,
            "range": "± 199223",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3003899,
            "range": "± 203907",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2323103,
            "range": "± 12558",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4244410,
            "range": "± 126364",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8199847,
            "range": "± 72705",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 268215,
            "range": "± 7100",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236925,
            "range": "± 4722",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 227719,
            "range": "± 3734",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "ef520f935de0fd7f7f3066e9869eb33f51b95a3c",
          "message": "rustdoc fix",
          "timestamp": "2025-06-12T10:22:22+02:00",
          "tree_id": "640e291b3cb024eee3e34bb43b161f79cc0d8727",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/ef520f935de0fd7f7f3066e9869eb33f51b95a3c"
        },
        "date": 1749716698037,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2552753,
            "range": "± 130651",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3015089,
            "range": "± 200089",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3000750,
            "range": "± 199388",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2310293,
            "range": "± 17951",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4189745,
            "range": "± 25536",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8164195,
            "range": "± 95349",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 267283,
            "range": "± 8761",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236231,
            "range": "± 4599",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 226273,
            "range": "± 3010",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "a0a0579c38cf9b8648c5ed5961a1c62eae762cd7",
          "message": "type BasicHazardCollector",
          "timestamp": "2025-06-12T16:27:25+02:00",
          "tree_id": "85e9b76673e3d107437f32c2524e4d969dc1302a",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/a0a0579c38cf9b8648c5ed5961a1c62eae762cd7"
        },
        "date": 1749738622553,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2558573,
            "range": "± 131533",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3005195,
            "range": "± 205712",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3001659,
            "range": "± 200242",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2277137,
            "range": "± 10819",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4215639,
            "range": "± 20738",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8200156,
            "range": "± 44776",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 267760,
            "range": "± 7044",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236180,
            "range": "± 4905",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 227506,
            "range": "± 3208",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "6e47329e4fd8dfe014428bd9045db132d26d35c9",
          "message": "type BasicHazardCollector",
          "timestamp": "2025-06-12T17:05:29+02:00",
          "tree_id": "d0a1de06b6f6ba17f3aed400edff69c7e016cef3",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/6e47329e4fd8dfe014428bd9045db132d26d35c9"
        },
        "date": 1749740895868,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2552261,
            "range": "± 131843",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3008760,
            "range": "± 203543",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2992905,
            "range": "± 197729",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2342361,
            "range": "± 11935",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4270407,
            "range": "± 29385",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8390120,
            "range": "± 59803",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 266927,
            "range": "± 6945",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236221,
            "range": "± 4646",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 230158,
            "range": "± 4866",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "be9a5cee16a868ceec9b15f5cb7669d379837b52",
          "message": "small changes in prep v6.2",
          "timestamp": "2025-06-13T11:39:59+02:00",
          "tree_id": "31d69f51233b2b70e68925f2c5a9ffd64f70d8b6",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/be9a5cee16a868ceec9b15f5cb7669d379837b52"
        },
        "date": 1749807764613,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2548156,
            "range": "± 130859",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3005825,
            "range": "± 201752",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2997462,
            "range": "± 198432",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2270235,
            "range": "± 14473",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4135851,
            "range": "± 23195",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8214599,
            "range": "± 65178",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 268358,
            "range": "± 6848",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236920,
            "range": "± 4663",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 228969,
            "range": "± 3124",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "8f15dc1788c350ce38afcf16d39ac384cf2d30d1",
          "message": "bump",
          "timestamp": "2025-06-13T11:45:39+02:00",
          "tree_id": "929ecd3b372fe1df58c11f83400820a8dc88d100",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/8f15dc1788c350ce38afcf16d39ac384cf2d30d1"
        },
        "date": 1749808117370,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2549149,
            "range": "± 136016",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3013960,
            "range": "± 200894",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 3025215,
            "range": "± 202599",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2358787,
            "range": "± 87906",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4289225,
            "range": "± 66696",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8329051,
            "range": "± 57646",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 266748,
            "range": "± 7051",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236600,
            "range": "± 16557",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 226838,
            "range": "± 3028",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "32c020c7599b7a2dba6fbdac22bad6e026bcc474",
          "message": "Merge pull request #43 from JeroenGar/feat/elim_atomics\n\nEliminated atomic ops during optimization",
          "timestamp": "2025-06-25T23:09:25+02:00",
          "tree_id": "8454e9736ddcff2d3ed32751a292fc27b867c565",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/32c020c7599b7a2dba6fbdac22bad6e026bcc474"
        },
        "date": 1750885944438,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2550989,
            "range": "± 136197",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 3053856,
            "range": "± 202381",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2993834,
            "range": "± 201455",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2376169,
            "range": "± 15077",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4216516,
            "range": "± 40887",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8259988,
            "range": "± 64458",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 268228,
            "range": "± 6835",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 235925,
            "range": "± 4482",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 226593,
            "range": "± 3107",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "306dbc1544bf8fc39122f3524f360cc5f09ec115",
          "message": "Merge pull request #44 from JeroenGar/clippy_rust_1.88\n\nEnsuring CI clippy is happy with Rust 1.88",
          "timestamp": "2025-06-27T06:48:54+02:00",
          "tree_id": "6696713835a6f5905728d456741a28ede48b0af4",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/306dbc1544bf8fc39122f3524f360cc5f09ec115"
        },
        "date": 1750999917548,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2526271,
            "range": "± 128512",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2954635,
            "range": "± 199025",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2966364,
            "range": "± 198709",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2342193,
            "range": "± 11722",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4197222,
            "range": "± 35778",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8252885,
            "range": "± 65593",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 267752,
            "range": "± 7090",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 235932,
            "range": "± 4683",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 226543,
            "range": "± 2957",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "bbc65a560c7f0862c9f6f96f8d6c2db184cda716",
          "message": "minor changes to layout_to_svg",
          "timestamp": "2025-07-02T18:33:55+02:00",
          "tree_id": "cd83841899ea3ea964999db3629a00813e0d74a9",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/bbc65a560c7f0862c9f6f96f8d6c2db184cda716"
        },
        "date": 1751474211393,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2527192,
            "range": "± 132362",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2974148,
            "range": "± 201726",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2949473,
            "range": "± 199544",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2341763,
            "range": "± 27982",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4308697,
            "range": "± 19834",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8340545,
            "range": "± 379047",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 266662,
            "range": "± 6638",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 236076,
            "range": "± 8120",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 226686,
            "range": "± 3026",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cf5f8cb860c46cb595bf9c48c0bc5e43d17e75b3",
          "message": "Merge pull request #46 from JeroenGar/feat/perf_improvements\n\n15-20% collect_collisions, 5% collides",
          "timestamp": "2025-07-09T11:47:30+02:00",
          "tree_id": "67e262163a715519e26c27a047d47a1e039c8521",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/cf5f8cb860c46cb595bf9c48c0bc5e43d17e75b3"
        },
        "date": 1752054624600,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2277175,
            "range": "± 111223",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2634778,
            "range": "± 151257",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2630185,
            "range": "± 142658",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2432621,
            "range": "± 16841",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4252390,
            "range": "± 28134",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8284982,
            "range": "± 88975",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 246016,
            "range": "± 6317",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 221269,
            "range": "± 4402",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 217228,
            "range": "± 3056",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "e41adb45c785958a343ac41104c6828e85ddfb33",
          "message": "v0.6.4",
          "timestamp": "2025-07-17T08:50:52+02:00",
          "tree_id": "a20840128b3d00c16124069fabb6ff3a51c4c5d2",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/e41adb45c785958a343ac41104c6828e85ddfb33"
        },
        "date": 1752735244299,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2272046,
            "range": "± 107771",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2645901,
            "range": "± 144347",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2607575,
            "range": "± 136208",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2403499,
            "range": "± 9320",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4234343,
            "range": "± 16058",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8081466,
            "range": "± 43365",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 244163,
            "range": "± 6368",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219741,
            "range": "± 4274",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 216972,
            "range": "± 4037",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d3f4a144bdbdfd661d9814071af8eb155f504769",
          "message": "Merge pull request #49 from JeroenGar/feat/degenerate_edge_elim\n\nMore rigid degenerate edge elimination in importer",
          "timestamp": "2025-07-28T13:27:16+02:00",
          "tree_id": "640bbbdd2d90a443cd016a1e1ed479ac011e88ec",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/d3f4a144bdbdfd661d9814071af8eb155f504769"
        },
        "date": 1753702207280,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2269192,
            "range": "± 107968",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2621336,
            "range": "± 137874",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2608587,
            "range": "± 136955",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2228898,
            "range": "± 75558",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4136346,
            "range": "± 110222",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8197181,
            "range": "± 52953",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 242888,
            "range": "± 6250",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 220080,
            "range": "± 4790",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 218023,
            "range": "± 3093",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "381d1865e28bf5905510193767c632ef84a87367",
          "message": "Merge pull request #52 from Ringo6107/main\n\nRedundant checks in CollidesWith<Edge> for Rect;",
          "timestamp": "2025-08-01T10:24:07+02:00",
          "tree_id": "38b18bd515c338932ad473e1c880aed963b0efc3",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/381d1865e28bf5905510193767c632ef84a87367"
        },
        "date": 1754036804412,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2271148,
            "range": "± 108307",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2615844,
            "range": "± 142263",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2588372,
            "range": "± 133695",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2263938,
            "range": "± 96484",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4011017,
            "range": "± 165957",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7703939,
            "range": "± 42104",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 243628,
            "range": "± 6212",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 220869,
            "range": "± 3981",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215730,
            "range": "± 3356",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "b6db04a8ea777b2de19a2610ded1c549c712f813",
          "message": "small changes",
          "timestamp": "2025-08-01T14:00:24+02:00",
          "tree_id": "952072a5b7582c0a8f9d4693f0124860b078143e",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/b6db04a8ea777b2de19a2610ded1c549c712f813"
        },
        "date": 1754049789873,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2277678,
            "range": "± 110309",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2631955,
            "range": "± 143438",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2636481,
            "range": "± 146829",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2365977,
            "range": "± 10782",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4142366,
            "range": "± 16326",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7956962,
            "range": "± 248023",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 245587,
            "range": "± 6014",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 224298,
            "range": "± 4284",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 220553,
            "range": "± 6519",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a7bd8e9a85ceca57dbc61e4188a06f7e1facb461",
          "message": "Merge pull request #50 from JeroenGar/feat/concav_elim\n\n[feature] Elimination of narrow concavities in shapes by preprocessor",
          "timestamp": "2025-08-04T13:51:27+02:00",
          "tree_id": "84c41688726beb6980f54348d9a9d6c6e33a84ee",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/a7bd8e9a85ceca57dbc61e4188a06f7e1facb461"
        },
        "date": 1754308458283,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2247758,
            "range": "± 107152",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2585282,
            "range": "± 135085",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2579099,
            "range": "± 137870",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2241985,
            "range": "± 7297",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4044796,
            "range": "± 55177",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7727266,
            "range": "± 40282",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 245368,
            "range": "± 6021",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 221894,
            "range": "± 5809",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 220045,
            "range": "± 2740",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3bc0be37537d7ae59446f75f4e188a61a392e58d",
          "message": "Merge pull request #47 from nots1dd/wasm-parallel\n\nAdd support for WASM target",
          "timestamp": "2025-08-13T15:57:17+02:00",
          "tree_id": "58c28ac26ecb92028e130684025023f103230199",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/3bc0be37537d7ae59446f75f4e188a61a392e58d"
        },
        "date": 1755093647535,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2293979,
            "range": "± 112268",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2649171,
            "range": "± 146221",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2654232,
            "range": "± 159271",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2420212,
            "range": "± 9987",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4234252,
            "range": "± 17549",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8210816,
            "range": "± 134960",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 242098,
            "range": "± 6300",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 218912,
            "range": "± 4192",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 214903,
            "range": "± 3148",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "802e688bdfba1f90056120f516420954dbed2705",
          "message": "cargo fmt",
          "timestamp": "2025-08-13T15:58:58+02:00",
          "tree_id": "637d3d9601854f8786a18d94ad8de23496b07f4c",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/802e688bdfba1f90056120f516420954dbed2705"
        },
        "date": 1755093743982,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2331490,
            "range": "± 108913",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2647034,
            "range": "± 144740",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2635267,
            "range": "± 143316",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2360743,
            "range": "± 9929",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4104935,
            "range": "± 20266",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7879452,
            "range": "± 53922",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 243269,
            "range": "± 6327",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 223118,
            "range": "± 4602",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215962,
            "range": "± 3703",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "6216848810c02f48c45869e2a0b0178a0caa9f7a",
          "message": "wasm github action",
          "timestamp": "2025-08-13T16:11:03+02:00",
          "tree_id": "20fcda86c6b826edd300a3c7f0f6cb1618590e7a",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/6216848810c02f48c45869e2a0b0178a0caa9f7a"
        },
        "date": 1755094440785,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2296014,
            "range": "± 109450",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2646514,
            "range": "± 146076",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2650426,
            "range": "± 149684",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2348857,
            "range": "± 8084",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4086925,
            "range": "± 18549",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7847209,
            "range": "± 50476",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 241969,
            "range": "± 6217",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219646,
            "range": "± 4340",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 217120,
            "range": "± 3078",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1fb0824457cea336e65cebdd8b3dfb8c9d88487a",
          "message": "Fixes WASM demo automatic deploy (#53)\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* wasm action fix\n\n* coi-serviceworker\n\n* README updates (WASM)",
          "timestamp": "2025-08-19T13:51:11+02:00",
          "tree_id": "866ad216d38a7b328019a1230767186dcaba3c5a",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/1fb0824457cea336e65cebdd8b3dfb8c9d88487a"
        },
        "date": 1755604469655,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2252765,
            "range": "± 106857",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2587239,
            "range": "± 134198",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2583040,
            "range": "± 135769",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2311861,
            "range": "± 13223",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 3999523,
            "range": "± 18607",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7781684,
            "range": "± 61382",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 244687,
            "range": "± 6286",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 221098,
            "range": "± 4632",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 218169,
            "range": "± 2982",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "1df1ba294a529d90f3b687da8ea024f0d849f12b",
          "message": "switch to jetli/wasm-pack-action",
          "timestamp": "2025-08-20T16:13:41+02:00",
          "tree_id": "0ba693952cfd498a0456e8766ce7a703722a2895",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/1df1ba294a529d90f3b687da8ea024f0d849f12b"
        },
        "date": 1755699433712,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2261657,
            "range": "± 102597",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2591603,
            "range": "± 133557",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2582646,
            "range": "± 133974",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2395479,
            "range": "± 8585",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4236571,
            "range": "± 19777",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8094777,
            "range": "± 66043",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 243595,
            "range": "± 6316",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219700,
            "range": "± 4209",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215564,
            "range": "± 3176",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "1ea3a59954779f287a7832ec1adf4d86523e4867",
          "message": "benchmark instances gardeyn added",
          "timestamp": "2025-08-22T21:21:47+02:00",
          "tree_id": "749fccfccc71b7001d79b97c5aa66284089f98b5",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/1ea3a59954779f287a7832ec1adf4d86523e4867"
        },
        "date": 1755890715334,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2253926,
            "range": "± 164390",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2611637,
            "range": "± 146023",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2613655,
            "range": "± 144449",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2227982,
            "range": "± 8117",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4050698,
            "range": "± 16047",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7889087,
            "range": "± 55946",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 245733,
            "range": "± 5923",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 220690,
            "range": "± 4167",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 216587,
            "range": "± 3050",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a55d969d5c028c4e56251fcdedc48b5eafd7c3d",
          "message": "Dependabot fix\n\nUpdated Dependabot configuration to target root directory.",
          "timestamp": "2025-09-25T13:44:01+02:00",
          "tree_id": "e71ac1f407ba2eb906a3f064aca1e8d3cd102b37",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/5a55d969d5c028c4e56251fcdedc48b5eafd7c3d"
        },
        "date": 1758800857051,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2277153,
            "range": "± 108874",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2635031,
            "range": "± 143980",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2634098,
            "range": "± 144926",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2396974,
            "range": "± 43193",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4203209,
            "range": "± 80131",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8101092,
            "range": "± 43918",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 245062,
            "range": "± 6277",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 220159,
            "range": "± 4165",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 217589,
            "range": "± 3038",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "4af0814c9920a184ba1f3a602d90ecdfd3d98806",
          "message": "Merge remote-tracking branch 'origin/main'",
          "timestamp": "2025-09-25T13:51:15+02:00",
          "tree_id": "c3f8e2d32f95afea7261f2bf232fe125a30c1519",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/4af0814c9920a184ba1f3a602d90ecdfd3d98806"
        },
        "date": 1758801256713,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2268302,
            "range": "± 110072",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2626593,
            "range": "± 143204",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2621808,
            "range": "± 142950",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2312910,
            "range": "± 7592",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4260902,
            "range": "± 15298",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8279077,
            "range": "± 45666",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 244735,
            "range": "± 6383",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 220560,
            "range": "± 4450",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 217022,
            "range": "± 2902",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "8ad43e489a2d24f63cf9a15feae268af9569d65c",
          "message": "[CI] build docs in stable Rust",
          "timestamp": "2025-09-25T14:07:32+02:00",
          "tree_id": "a945a546ad07a3a5a0ddcf05a146e4186a0a43ac",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/8ad43e489a2d24f63cf9a15feae268af9569d65c"
        },
        "date": 1758802239181,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2267779,
            "range": "± 107479",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2594637,
            "range": "± 135663",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2591203,
            "range": "± 135047",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2200173,
            "range": "± 7665",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4034320,
            "range": "± 17598",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7786588,
            "range": "± 52074",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 243461,
            "range": "± 6354",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219373,
            "range": "± 4283",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 216074,
            "range": "± 3165",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "877aaa3c3217f0dd2b6f1c8af8d3e53d4023e422",
          "message": "improve error handling for items and bins; ensure positive demand and stock (#55)",
          "timestamp": "2025-10-14T16:53:14+02:00",
          "tree_id": "cab81e2b35b6b1c9ddb00a7f9de1db0c922f57bf",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/877aaa3c3217f0dd2b6f1c8af8d3e53d4023e422"
        },
        "date": 1760453786741,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2279090,
            "range": "± 106424",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2672376,
            "range": "± 156628",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2631376,
            "range": "± 142435",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2348235,
            "range": "± 15721",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4023505,
            "range": "± 15535",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7816724,
            "range": "± 157389",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 242408,
            "range": "± 6208",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219633,
            "range": "± 4236",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 216237,
            "range": "± 2992",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3aca0d263b0ae55f390096feeaa7bb0518cb03e1",
          "message": "Update ndarray requirement from 0.16 to 0.17 (#56)\n\nUpdates the requirements on [ndarray](https://github.com/rust-ndarray/ndarray) to permit the latest version.\n- [Release notes](https://github.com/rust-ndarray/ndarray/releases)\n- [Changelog](https://github.com/rust-ndarray/ndarray/blob/master/RELEASES.md)\n- [Commits](https://github.com/rust-ndarray/ndarray/compare/0.16.0...0.17.1)\n\n---\nupdated-dependencies:\n- dependency-name: ndarray\n  dependency-version: 0.17.1\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-11-03T23:53:54+01:00",
          "tree_id": "fceb7ed51c5bbd18e327cc3df68d8a9bf5440dee",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/3aca0d263b0ae55f390096feeaa7bb0518cb03e1"
        },
        "date": 1762210640806,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2265608,
            "range": "± 112225",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2614178,
            "range": "± 141247",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2602350,
            "range": "± 134064",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2331509,
            "range": "± 11370",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4029611,
            "range": "± 26515",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7801074,
            "range": "± 84962",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 243087,
            "range": "± 6554",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219549,
            "range": "± 4359",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215863,
            "range": "± 2765",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8c27eaab9bbcaf38afc7e5807f7ad7e98eedf22d",
          "message": "Pinning docs CI to Rust 1.90.0 until rust-lang/rust#148431 is fixed\n\nUpdate Rust toolchain version in GitHub Actions workflow.",
          "timestamp": "2025-11-18T17:26:13+01:00",
          "tree_id": "9d7181c11b75ad7a67e01b14b6e0d8f0b9e8a534",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/8c27eaab9bbcaf38afc7e5807f7ad7e98eedf22d"
        },
        "date": 1763483375604,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2308440,
            "range": "± 110457",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2658275,
            "range": "± 148017",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2644411,
            "range": "± 144270",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2278730,
            "range": "± 8706",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 3998600,
            "range": "± 16779",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7823715,
            "range": "± 42080",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 244648,
            "range": "± 6319",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219967,
            "range": "± 4142",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215419,
            "range": "± 3049",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bb5e080eff6acefa7f08fc19da2c9677337cd6e5",
          "message": "Pinning docs CI to Rust 1.90.0 until rust-lang/rust#148431 is fixed",
          "timestamp": "2025-11-18T17:37:12+01:00",
          "tree_id": "f163cb19e9ca4b8c4aa478ce5868e06a63ac358f",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/bb5e080eff6acefa7f08fc19da2c9677337cd6e5"
        },
        "date": 1763483999020,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2364242,
            "range": "± 117483",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2759680,
            "range": "± 156084",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2740502,
            "range": "± 146038",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2223585,
            "range": "± 16646",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4016191,
            "range": "± 87040",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7982494,
            "range": "± 84426",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 255286,
            "range": "± 8323",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 222328,
            "range": "± 4994",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 217391,
            "range": "± 4019",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b85d9268e46f4273b673325b172caee7b6116583",
          "message": "fixes #58 (#59)\n\n* fixes #58\n\n* Enabled WASM CI for PRs",
          "timestamp": "2025-12-05T23:25:27+01:00",
          "tree_id": "4f918732644ffdb7451e1f102c6bb51ce1bfeb77",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/b85d9268e46f4273b673325b172caee7b6116583"
        },
        "date": 1764973720014,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2278819,
            "range": "± 108901",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2637886,
            "range": "± 144105",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2631555,
            "range": "± 146076",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2316569,
            "range": "± 10720",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4059955,
            "range": "± 19207",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7960597,
            "range": "± 59106",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 242957,
            "range": "± 7891",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 217751,
            "range": "± 4195",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215700,
            "range": "± 3111",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "8360e4d43818b806d750d6e83657386338267f02",
          "message": "Add conditional deployment for main branch in CI workflows",
          "timestamp": "2025-12-05T23:33:53+01:00",
          "tree_id": "214e8559dd9af4e878ad528695ef407cef0f01fa",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/8360e4d43818b806d750d6e83657386338267f02"
        },
        "date": 1764974195218,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2282204,
            "range": "± 116714",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2629288,
            "range": "± 142602",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2622920,
            "range": "± 142212",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2311868,
            "range": "± 7389",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4190699,
            "range": "± 15904",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8175222,
            "range": "± 48130",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 242801,
            "range": "± 8424",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219328,
            "range": "± 4343",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 216567,
            "range": "± 4018",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "4ff2a3d2f0d96f915bf5ed8902546fcca4478fc7",
          "message": "removed all comments about `commit_instant` (fixes #62)",
          "timestamp": "2025-12-07T11:04:08+01:00",
          "tree_id": "e208e065570ceb4af99b5e953a45db25913d510d",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/4ff2a3d2f0d96f915bf5ed8902546fcca4478fc7"
        },
        "date": 1765102026130,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2451663,
            "range": "± 119158",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2821037,
            "range": "± 155290",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2781366,
            "range": "± 152712",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2441779,
            "range": "± 13538",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4554501,
            "range": "± 16619",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8636400,
            "range": "± 52327",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 252295,
            "range": "± 7377",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 227394,
            "range": "± 5419",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 218483,
            "range": "± 3447",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "788b226d917c241aa6279f73d84c0c9c4109a689",
          "message": "Update criterion requirement from 0.7 to 0.8 (#63)\n\nUpdates the requirements on [criterion](https://github.com/criterion-rs/criterion.rs) to permit the latest version.\n- [Release notes](https://github.com/criterion-rs/criterion.rs/releases)\n- [Changelog](https://github.com/criterion-rs/criterion.rs/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/criterion-rs/criterion.rs/compare/criterion-plot-v0.7.0...criterion-v0.8.1)\n\n---\nupdated-dependencies:\n- dependency-name: criterion\n  dependency-version: 0.8.1\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-09T00:25:14+01:00",
          "tree_id": "5dad0326aaf3957c559457b10da9cb000572e42d",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/788b226d917c241aa6279f73d84c0c9c4109a689"
        },
        "date": 1765236483470,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2266317,
            "range": "± 108453",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2608385,
            "range": "± 142497",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2608828,
            "range": "± 139769",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2305219,
            "range": "± 47237",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4177533,
            "range": "± 89633",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8141717,
            "range": "± 140421",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 243569,
            "range": "± 8308",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 217083,
            "range": "± 4183",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 213634,
            "range": "± 6417",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "be6a17f077f8406df2e6eba6d541c874c08d5cf1",
          "message": "Multi-Strip Packing Problem modelling (#57)",
          "timestamp": "2025-12-22T22:00:53+01:00",
          "tree_id": "99c9e8d83d78f5f6298dede8e94fed267bb47700",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/be6a17f077f8406df2e6eba6d541c874c08d5cf1"
        },
        "date": 1766437458019,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2245555,
            "range": "± 105361",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2572049,
            "range": "± 135348",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2569036,
            "range": "± 163962",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2379214,
            "range": "± 39606",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4177023,
            "range": "± 69450",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8021437,
            "range": "± 140076",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 240374,
            "range": "± 6164",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 217895,
            "range": "± 4211",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 214300,
            "range": "± 2923",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "72dde9ee35f4ce40ff48b61d2fed08ed51f9b010",
          "message": "[mspp] proper JSON output",
          "timestamp": "2025-12-23T09:54:40+01:00",
          "tree_id": "155fdc92d13c4fdd9d0a39632dc33abf2261eed5",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/72dde9ee35f4ce40ff48b61d2fed08ed51f9b010"
        },
        "date": 1766480244825,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2247548,
            "range": "± 104961",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2580058,
            "range": "± 132716",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2566480,
            "range": "± 132093",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2400819,
            "range": "± 44736",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4192098,
            "range": "± 82961",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 8205253,
            "range": "± 240078",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 240269,
            "range": "± 6142",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 216956,
            "range": "± 4108",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 213511,
            "range": "± 2919",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jeroen.gardeyn@hotmail.com",
            "name": "Jeroen Gardeyn",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "76aab0fa70688ee50d1809f5376f71c758ea0b7c",
          "message": "export external solutions back into the library. (#66)\n\n- Currently only implemented for strip packing",
          "timestamp": "2026-01-05T12:27:02+01:00",
          "tree_id": "57dc2c612ddb9db17732a8b7029d23ba0946b25e",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/76aab0fa70688ee50d1809f5376f71c758ea0b7c"
        },
        "date": 1767612608677,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k/3",
            "value": 2253936,
            "range": "± 106829",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/4",
            "value": 2593600,
            "range": "± 134400",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k/5",
            "value": 2582138,
            "range": "± 133763",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/3",
            "value": 2310603,
            "range": "± 38775",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/4",
            "value": 4129686,
            "range": "± 72422",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k/5",
            "value": 7995976,
            "range": "± 137237",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/3",
            "value": 244486,
            "range": "± 8062",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/4",
            "value": 219713,
            "range": "± 4240",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k/5",
            "value": 215812,
            "range": "± 2923",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}