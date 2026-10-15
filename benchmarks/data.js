window.BENCHMARK_DATA = {
  "lastUpdate": 1792096206023,
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
            "name": "cde_collect_1k",
            "value": 4838896,
            "range": "± 401745",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 297289,
            "range": "± 8100",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 9294842,
            "range": "± 42095",
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
            "name": "cde_update_1k",
            "value": 9330368,
            "range": "± 42026",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 297745,
            "range": "± 7783",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4857438,
            "range": "± 409667",
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
            "name": "cde_update_1k",
            "value": 8977237,
            "range": "± 64816",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 288903,
            "range": "± 7084",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4579962,
            "range": "± 380479",
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
            "name": "cde_update_1k",
            "value": 9027159,
            "range": "± 56628",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 289005,
            "range": "± 6832",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4594668,
            "range": "± 392191",
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
            "name": "cde_update_1k",
            "value": 8985020,
            "range": "± 84203",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 289474,
            "range": "± 11443",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4544329,
            "range": "± 377480",
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
            "name": "cde_update_1k",
            "value": 8960931,
            "range": "± 44310",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 288292,
            "range": "± 7324",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4574729,
            "range": "± 467714",
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
            "name": "cde_update_1k",
            "value": 8936577,
            "range": "± 69646",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 288610,
            "range": "± 7040",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4472569,
            "range": "± 354383",
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
            "name": "cde_update_1k",
            "value": 9039428,
            "range": "± 77579",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 288998,
            "range": "± 7537",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4635348,
            "range": "± 383182",
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
            "name": "cde_collect_1k",
            "value": 4555225,
            "range": "± 390883",
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
            "name": "cde_update_1k",
            "value": 8896385,
            "range": "± 53027",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 278499,
            "range": "± 6741",
            "unit": "ns/iter"
          },
          {
            "name": "cde_collect_1k",
            "value": 4369502,
            "range": "± 419471",
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
            "name": "cde_collect_1k",
            "value": 3339778,
            "range": "± 235893",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 9440885,
            "range": "± 46748",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 271494,
            "range": "± 5932",
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
            "name": "cde_collect_1k",
            "value": 3264213,
            "range": "± 254783",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 9308016,
            "range": "± 60734",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 268668,
            "range": "± 5638",
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
            "name": "cde_collect_1k",
            "value": 3274567,
            "range": "± 248413",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 9374319,
            "range": "± 48637",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 269822,
            "range": "± 6144",
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
            "name": "cde_collect_1k",
            "value": 3277663,
            "range": "± 248237",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 9250433,
            "range": "± 78677",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 269960,
            "range": "± 5683",
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
            "name": "cde_collect_1k",
            "value": 3119444,
            "range": "± 214286",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6149004,
            "range": "± 337103",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 263173,
            "range": "± 5451",
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
            "name": "cde_collect_1k",
            "value": 3115191,
            "range": "± 221726",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6079135,
            "range": "± 30161",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 262562,
            "range": "± 5176",
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
            "name": "cde_collect_1k",
            "value": 3094123,
            "range": "± 213604",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6185174,
            "range": "± 39467",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 263037,
            "range": "± 12958",
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
            "name": "cde_collect_1k",
            "value": 3086494,
            "range": "± 212637",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6109250,
            "range": "± 26479",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 262853,
            "range": "± 5319",
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
            "name": "cde_collect_1k",
            "value": 3078786,
            "range": "± 212554",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6083275,
            "range": "± 44606",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 262569,
            "range": "± 5172",
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
            "name": "cde_collect_1k",
            "value": 3081022,
            "range": "± 202769",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6056698,
            "range": "± 31100",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 263497,
            "range": "± 5258",
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
            "name": "cde_collect_1k",
            "value": 3115766,
            "range": "± 240762",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 6071994,
            "range": "± 42376",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 263047,
            "range": "± 5324",
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
            "name": "cde_collect_1k",
            "value": 2981358,
            "range": "± 200600",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 5188441,
            "range": "± 26608",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 241450,
            "range": "± 4823",
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
            "name": "cde_collect_1k",
            "value": 3022810,
            "range": "± 199223",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4244410,
            "range": "± 126364",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236925,
            "range": "± 4722",
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
            "name": "cde_collect_1k",
            "value": 3015089,
            "range": "± 200089",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4189745,
            "range": "± 25536",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236231,
            "range": "± 4599",
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
            "name": "cde_collect_1k",
            "value": 3005195,
            "range": "± 205712",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4215639,
            "range": "± 20738",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236180,
            "range": "± 4905",
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
            "name": "cde_collect_1k",
            "value": 3008760,
            "range": "± 203543",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4270407,
            "range": "± 29385",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236221,
            "range": "± 4646",
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
            "name": "cde_collect_1k",
            "value": 3005825,
            "range": "± 201752",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4135851,
            "range": "± 23195",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236920,
            "range": "± 4663",
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
            "name": "cde_collect_1k",
            "value": 3013960,
            "range": "± 200894",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4289225,
            "range": "± 66696",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236600,
            "range": "± 16557",
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
            "name": "cde_collect_1k",
            "value": 3053856,
            "range": "± 202381",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4216516,
            "range": "± 40887",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 235925,
            "range": "± 4482",
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
            "name": "cde_collect_1k",
            "value": 2954635,
            "range": "± 199025",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4197222,
            "range": "± 35778",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 235932,
            "range": "± 4683",
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
            "name": "cde_collect_1k",
            "value": 2974148,
            "range": "± 201726",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4308697,
            "range": "± 19834",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 236076,
            "range": "± 8120",
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
            "name": "cde_collect_1k",
            "value": 2634778,
            "range": "± 151257",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4252390,
            "range": "± 28134",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 221269,
            "range": "± 4402",
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
            "name": "cde_collect_1k",
            "value": 2645901,
            "range": "± 144347",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4234343,
            "range": "± 16058",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219741,
            "range": "± 4274",
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
            "name": "cde_collect_1k",
            "value": 2621336,
            "range": "± 137874",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4136346,
            "range": "± 110222",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220080,
            "range": "± 4790",
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
            "name": "cde_collect_1k",
            "value": 2615844,
            "range": "± 142263",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4011017,
            "range": "± 165957",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220869,
            "range": "± 3981",
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
            "name": "cde_collect_1k",
            "value": 2631955,
            "range": "± 143438",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4142366,
            "range": "± 16326",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 224298,
            "range": "± 4284",
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
            "name": "cde_collect_1k",
            "value": 2585282,
            "range": "± 135085",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4044796,
            "range": "± 55177",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 221894,
            "range": "± 5809",
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
            "name": "cde_collect_1k",
            "value": 2649171,
            "range": "± 146221",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4234252,
            "range": "± 17549",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 218912,
            "range": "± 4192",
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
            "name": "cde_collect_1k",
            "value": 2647034,
            "range": "± 144740",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4104935,
            "range": "± 20266",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 223118,
            "range": "± 4602",
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
            "name": "cde_collect_1k",
            "value": 2646514,
            "range": "± 146076",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4086925,
            "range": "± 18549",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219646,
            "range": "± 4340",
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
            "name": "cde_collect_1k",
            "value": 2587239,
            "range": "± 134198",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3999523,
            "range": "± 18607",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 221098,
            "range": "± 4632",
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
            "name": "cde_collect_1k",
            "value": 2591603,
            "range": "± 133557",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4236571,
            "range": "± 19777",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219700,
            "range": "± 4209",
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
            "name": "cde_collect_1k",
            "value": 2611637,
            "range": "± 146023",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4050698,
            "range": "± 16047",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220690,
            "range": "± 4167",
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
            "name": "cde_collect_1k",
            "value": 2635031,
            "range": "± 143980",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4203209,
            "range": "± 80131",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220159,
            "range": "± 4165",
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
            "name": "cde_collect_1k",
            "value": 2626593,
            "range": "± 143204",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4260902,
            "range": "± 15298",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220560,
            "range": "± 4450",
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
            "name": "cde_collect_1k",
            "value": 2594637,
            "range": "± 135663",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4034320,
            "range": "± 17598",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219373,
            "range": "± 4283",
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
            "name": "cde_collect_1k",
            "value": 2672376,
            "range": "± 156628",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4023505,
            "range": "± 15535",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219633,
            "range": "± 4236",
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
            "name": "cde_collect_1k",
            "value": 2614178,
            "range": "± 141247",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4029611,
            "range": "± 26515",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219549,
            "range": "± 4359",
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
            "name": "cde_collect_1k",
            "value": 2658275,
            "range": "± 148017",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3998600,
            "range": "± 16779",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219967,
            "range": "± 4142",
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
            "name": "cde_collect_1k",
            "value": 2759680,
            "range": "± 156084",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4016191,
            "range": "± 87040",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 222328,
            "range": "± 4994",
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
            "name": "cde_collect_1k",
            "value": 2637886,
            "range": "± 144105",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4059955,
            "range": "± 19207",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 217751,
            "range": "± 4195",
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
            "name": "cde_collect_1k",
            "value": 2629288,
            "range": "± 142602",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4190699,
            "range": "± 15904",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219328,
            "range": "± 4343",
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
            "name": "cde_collect_1k",
            "value": 2821037,
            "range": "± 155290",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4554501,
            "range": "± 16619",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 227394,
            "range": "± 5419",
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
            "name": "cde_collect_1k",
            "value": 2608385,
            "range": "± 142497",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4177533,
            "range": "± 89633",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 217083,
            "range": "± 4183",
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
            "name": "cde_collect_1k",
            "value": 2572049,
            "range": "± 135348",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4177023,
            "range": "± 69450",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 217895,
            "range": "± 4211",
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
            "name": "cde_collect_1k",
            "value": 2580058,
            "range": "± 132716",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4192098,
            "range": "± 82961",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 216956,
            "range": "± 4108",
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
            "name": "cde_collect_1k",
            "value": 2593600,
            "range": "± 134400",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4129686,
            "range": "± 72422",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219713,
            "range": "± 4240",
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
          "id": "fc7ad15c791b7ff6335b9d30d4f45a0dcbde66bc",
          "message": "Update rand requirement from 0.9 to 0.10 (#71)\n\n* Update rand requirement from 0.9 to 0.10\n\nUpdates the requirements on [rand](https://github.com/rust-random/rand) to permit the latest version.\n- [Release notes](https://github.com/rust-random/rand/releases)\n- [Changelog](https://github.com/rust-random/rand/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/rust-random/rand/compare/rand_core-0.9.1...rand_core-0.9.2)\n\n---\nupdated-dependencies:\n- dependency-name: rand\n  dependency-version: 0.9.2\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\n\n* fix compile errors rand 0.10\n\n* update rust-toolchain to nightly in CI configuration\n\n---------\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: Jeroen Gardeyn <jeroen.gardeyn@hotmail.com>",
          "timestamp": "2026-02-10T15:43:30+01:00",
          "tree_id": "025a8288601872996b74e96d0799ec868b747e4e",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/fc7ad15c791b7ff6335b9d30d4f45a0dcbde66bc"
        },
        "date": 1770734807368,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2628086,
            "range": "± 142125",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4200211,
            "range": "± 89143",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220315,
            "range": "± 4316",
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
          "id": "ba38bcae9ed3ab41a9e93a1894e2b01ea87c6619",
          "message": "Fix/change strip width (#72)\n\n* `change_strip_width` now enforces positive width\n\n* fmt\n\n* Add `n_placed_items` method to various Problems",
          "timestamp": "2026-02-10T17:34:57+01:00",
          "tree_id": "7832d69f5f5b976254b213fc06de821143dff8c0",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/ba38bcae9ed3ab41a9e93a1894e2b01ea87c6619"
        },
        "date": 1770741466974,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2637796,
            "range": "± 144331",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4061012,
            "range": "± 139426",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 218394,
            "range": "± 4048",
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
          "id": "06b4915aac55c944d966aaf5c6b472a7fd699c6e",
          "message": "fix/narrow_concavities (#74)\n\n* fix/narrow_concavities (#73)\n\n* fix/narrow_concavities (#73)\n\n* wasm demo fix\n\n* wasm demo fix",
          "timestamp": "2026-02-25T16:58:20+01:00",
          "tree_id": "ecefa6c07eaf117a9d103547e5cab52c830815dd",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/06b4915aac55c944d966aaf5c6b472a7fd699c6e"
        },
        "date": 1772035323631,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2618991,
            "range": "± 143028",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4209357,
            "range": "± 94980",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219440,
            "range": "± 4275",
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
          "id": "816e6a65f14f9417726a06abdef058238e3edecc",
          "message": "v0.7.0",
          "timestamp": "2026-03-06T10:29:17+01:00",
          "tree_id": "ad16dad6709e8d2f69868d67da695ab218d1f9f9",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/816e6a65f14f9417726a06abdef058238e3edecc"
        },
        "date": 1772789561626,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2642279,
            "range": "± 146252",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4139783,
            "range": "± 42544",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 219954,
            "range": "± 4313",
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
          "id": "481cec5d318a3bb8be40c659371eb0f745972e18",
          "message": "fix documentation: rotations in `ExtTransform` are in degrees (#75)",
          "timestamp": "2026-03-10T21:16:05+01:00",
          "tree_id": "11aa0c4719ea14a438a552d8c0d2a57442a31357",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/481cec5d318a3bb8be40c659371eb0f745972e18"
        },
        "date": 1773173941009,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2673372,
            "range": "± 149202",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4195557,
            "range": "± 49690",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 218850,
            "range": "± 4257",
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
          "id": "8896414355fb605ae3313037ba11b66ed035f80a",
          "message": "typo",
          "timestamp": "2026-03-31T10:42:50+02:00",
          "tree_id": "7922fddb2e66f07c7aa70f1d12d0dfc24ddf566d",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/8896414355fb605ae3313037ba11b66ed035f80a"
        },
        "date": 1774946778272,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2675039,
            "range": "± 145850",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4122713,
            "range": "± 37991",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 221250,
            "range": "± 4313",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "135309102+jan-martens0124@users.noreply.github.com",
            "name": "Jan Martens",
            "username": "jan-martens0124"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4410ce23a59029c0ac65a10669e0967089ab137b",
          "message": "Refactor hazard and placed item structures to use Arc for shape (#76)\n\n* Refactor hazard and placed item structures to use Arc for shape\n\n* fmt\n\n---------\n\nCo-authored-by: Jeroen Gardeyn <jeroen.gardeyn@hotmail.com>",
          "timestamp": "2026-04-02T16:17:06+02:00",
          "tree_id": "7854f28351b9df1af0bdd02a2db34d656e9fb7b1",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/4410ce23a59029c0ac65a10669e0967089ab137b"
        },
        "date": 1775139596223,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2681653,
            "range": "± 143555",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4048806,
            "range": "± 58407",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 218938,
            "range": "± 4142",
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
          "id": "7552bd7bb049fd2817582c2e97b427d75f26fc48",
          "message": "v0.7.1",
          "timestamp": "2026-04-08T08:52:27+02:00",
          "tree_id": "d5d998a28fe72b6413a2acb0b41c3966a36b7045",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/7552bd7bb049fd2817582c2e97b427d75f26fc48"
        },
        "date": 1775631325051,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2904292,
            "range": "± 171944",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3711358,
            "range": "± 40621",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 246704,
            "range": "± 5665",
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
          "id": "19c6740d676cc18cfea2824159aeb68e0eb8ee8e",
          "message": "update GitHub Actions to use checkout@v6",
          "timestamp": "2026-04-09T12:05:06+02:00",
          "tree_id": "d467fbd3214bce9e255bdb237b084b171c7d6697",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/19c6740d676cc18cfea2824159aeb68e0eb8ee8e"
        },
        "date": 1775729273223,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2651215,
            "range": "± 142805",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3985972,
            "range": "± 40227",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220126,
            "range": "± 4364",
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
          "id": "2e8c210d016956adc2ef9a89295350a177060cee",
          "message": "add self-intersecting polygon validation and corresponding test (#79)\n\n* add self-intersecting polygon validation and corresponding test\n\n* clippy fix\n\n* test fix",
          "timestamp": "2026-05-07T11:26:15+02:00",
          "tree_id": "60083b82cde9b47a6e5a694b0d1848b931216481",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/2e8c210d016956adc2ef9a89295350a177060cee"
        },
        "date": 1778146183791,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2662076,
            "range": "± 147771",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3882777,
            "range": "± 86163",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220334,
            "range": "± 6751",
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
          "id": "df8a8b4a958eeef41f4d438473f8de443735724c",
          "message": "Avoid PR benchmark comment writes (#81)\n\n* Avoid PR benchmark comment writes\n\n* Fix benchmark summary-only action inputs\n\n* Trim benchmark action inputs\n\n* Update benchmark checkout action\n\n---------\n\nCo-authored-by: Jeroen Gardeyn <jeroengardeyn@MB-16-Pro-Jeroen.local>",
          "timestamp": "2026-05-19T15:25:17+02:00",
          "tree_id": "007a6fbff04cae4f83f0299da7fb58fabb0aea13",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/df8a8b4a958eeef41f4d438473f8de443735724c"
        },
        "date": 1779197320938,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2649570,
            "range": "± 155296",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3929996,
            "range": "± 60258",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 213429,
            "range": "± 4469",
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
          "id": "73806c82b3c204beb25189605a8fb2655f5a1d32",
          "message": "Use stable Rust in CI (#82)\n\nCo-authored-by: Jeroen Gardeyn <jeroengardeyn@MB-16-Pro-Jeroen.local>",
          "timestamp": "2026-05-20T08:07:59+02:00",
          "tree_id": "6e04bf0e0649dd04e6614cc5dd0fbd5a0438a9d4",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/73806c82b3c204beb25189605a8fb2655f5a1d32"
        },
        "date": 1779257441093,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2655352,
            "range": "± 142724",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3965349,
            "range": "± 68734",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 218340,
            "range": "± 4139",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "tristan.manier@reeverse-systems.com",
            "name": "TristanM-RS",
            "username": "TristanM-RS"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "43e81373ef5ff403df708dea60162eed236dd251",
          "message": "Code quality (#80)\n\n* clippy stricter warnings\n\n* resolve clippy::must_use_candidate\n\n* replace global clippy warnings ignore by local ones\n\n* fix clippy warnings hidden behind flags",
          "timestamp": "2026-05-20T17:36:42+02:00",
          "tree_id": "022d648f6cf52314a8dd1724aa111f27282e25a1",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/43e81373ef5ff403df708dea60162eed236dd251"
        },
        "date": 1779291572062,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2658624,
            "range": "± 148093",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3947714,
            "range": "± 63412",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 220421,
            "range": "± 4235",
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
          "id": "d4046d5cd7395c2ed630bf9001fcce406a2df8ab",
          "message": "Update itertools requirement from 0.14 to 0.15 (#84)\n\n* Update itertools requirement from 0.14 to 0.15\n\nUpdates the requirements on [itertools](https://github.com/rust-itertools/itertools) to permit the latest version.\n- [Changelog](https://github.com/rust-itertools/itertools/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/rust-itertools/itertools/compare/v0.14.0...v0.15.0)\n\n---\nupdated-dependencies:\n- dependency-name: itertools\n  dependency-version: 0.15.0\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\n\n* Replace deprecated tuple_combinations with array_combinations::<2>()\n\n---------\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: copilot-swe-agent[bot] <198982749+Copilot@users.noreply.github.com>",
          "timestamp": "2026-06-30T21:43:55+02:00",
          "tree_id": "701a15851cdb2a2f25447796fd2f0798587bd17d",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/d4046d5cd7395c2ed630bf9001fcce406a2df8ab"
        },
        "date": 1782848842278,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2721037,
            "range": "± 147933",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4057792,
            "range": "± 82018",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 245061,
            "range": "± 10604",
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
          "id": "9a19409bd38f3643c3d6d2d7571cddfba548ee17",
          "message": "Release jagua-rs 0.8.0 (#85)\n\n* Add collision performance campaign oracles\n\n* Visit likely collision quadrant first\n\n* Keep strict Clippy green\n\n* Support stable Box array iteration\n\n* Isolate CI benchmark reports from build cache\n\n* Inline partial hazard collision checks\n\n* Reuse quadtree split for circle ordering\n\n* Avoid CPU-specific benchmark build cache\n\n* Compute circle quadrant order without branches\n\n* Document unattended profiler invocation\n\n* Preserve public quadtree witness order\n\n* Focus PR on collision throughput changes\n\n* Document quadrant traversal ordering\n\n* Tune collision collection threshold\n\n* Allow early collision collection termination\n\n* Expose inserted collision to stop callback\n\n* Revert \"Expose inserted collision to stop callback\"\n\nThis reverts commit 6b0499ae85cff304e7fc5e9d1603678e9b96a16d.\n\n* Revert \"Allow early collision collection termination\"\n\nThis reverts commit a47ab1311a9da7bfbdd03e85048b2b0d5fec3041.\n\n* Allow collision collection to stop early\n\n* Satisfy current Clippy lints\n\n* Keep child traversal stable-compatible\n\n* Minimize collision throughput diff\n\n* Keep stable Clippy green\n\n* Bump jagua-rs to 0.7.3\n\n* Move fail-fast collision collection into jagua-rs\n\n* Bump jagua-rs to 0.8.0\n\n* Clarify collision callback documentation\n\n* Pass hazard entities to collision callbacks\n\n* Unify collision collection traversal\n\n* Configure surrogate fail-fast coverage\n\n* Fix release config and benchmark cache",
          "timestamp": "2026-09-01T09:41:21+02:00",
          "tree_id": "97409a06741aa4ff5c2fd30fa79744e351657cbd",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9a19409bd38f3643c3d6d2d7571cddfba548ee17"
        },
        "date": 1788248661907,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1993887,
            "range": "± 93039",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3823940,
            "range": "± 40039",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 185719,
            "range": "± 4790",
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
          "id": "b02c8e8a4a096e802dbe4a3136540b8d7fb07612",
          "message": "Add funding information for GitHub and Buy Me a Coffee",
          "timestamp": "2026-09-03T12:41:17+02:00",
          "tree_id": "13b65a924be7e06137d85485b6437accc5228867",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/b02c8e8a4a096e802dbe4a3136540b8d7fb07612"
        },
        "date": 1788432266640,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2021467,
            "range": "± 95000",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3885581,
            "range": "± 143620",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 183281,
            "range": "± 4817",
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
          "id": "824ab31cf8a58eecf5d87527260c92510626661b",
          "message": "Make quadtree debug assertion robust to f32 rounding (#87)\n\n* Make quadtree debug assertion robust to f32 rounding\n\n* Release jagua-rs 0.8.1",
          "timestamp": "2026-09-04T22:52:18+02:00",
          "tree_id": "65c5ac4649e98b17997a98258fbcc33526805a83",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/824ab31cf8a58eecf5d87527260c92510626661b"
        },
        "date": 1788555334442,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2005412,
            "range": "± 93410",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3921885,
            "range": "± 31134",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 184266,
            "range": "± 4838",
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
          "id": "dea6f8724cab97fd2104440ec8e3fdc22c053806",
          "message": "Correct transformation operation order in rustdoc (#89)",
          "timestamp": "2026-09-08T00:07:27+02:00",
          "tree_id": "af1a0f39bbfe4605c763d827b12220c637d2c297",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/dea6f8724cab97fd2104440ec8e3fdc22c053806"
        },
        "date": 1788819046099,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1571460,
            "range": "± 68572",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3132590,
            "range": "± 94477",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 141320,
            "range": "± 4723",
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
          "id": "bfdeca7ac55da4aa24ae16bb96be24d9d3c73f31",
          "message": "Draw SVG collision lines for holes and inferior-quality zones (#90)\n\n* Draw SVG collision lines for container holes\n\n* Handle inferior-quality zones in SVG collision annotations",
          "timestamp": "2026-09-13T16:30:12+02:00",
          "tree_id": "5bcdfe211ef57974b93c35f8d1120d0c96a742d6",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/bfdeca7ac55da4aa24ae16bb96be24d9d3c73f31"
        },
        "date": 1789310003943,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2155576,
            "range": "± 100228",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3518306,
            "range": "± 64010",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 198176,
            "range": "± 5194",
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
          "id": "d03e82190c40a21afbc1fabed19876401d0572bc",
          "message": "Prepare jagua-rs 0.8.2 (#92)",
          "timestamp": "2026-09-13T16:56:18+02:00",
          "tree_id": "0535f9799ec8621f4ba5e56efc6d12de41617d4c",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/d03e82190c40a21afbc1fabed19876401d0572bc"
        },
        "date": 1789311571060,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1672424,
            "range": "± 72648",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 2778584,
            "range": "± 23183",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 156316,
            "range": "± 4889",
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
          "id": "a15ddd4909c5bd4d619c1878342c7b71ab27ac4a",
          "message": "Fix clearance buffer self-intersections and shorten polygon errors (#102)\n\n* Replace polygon buffering with geo and shorten validation errors\n\n* Test inflation across datasets and enable WASM reference types\n\n* Bump jagua-rs to 0.8.3\n\n* Pin wasm-pack to tested 0.15.0 release\n\n* Remove redundant wasm-opt reference-types workaround",
          "timestamp": "2026-09-15T15:35:45+02:00",
          "tree_id": "6dbfd7b514beee8ee2bc3ff2103af7d00fc0d63b",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/a15ddd4909c5bd4d619c1878342c7b71ab27ac4a"
        },
        "date": 1789479548252,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2022339,
            "range": "± 100427",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3980284,
            "range": "± 177648",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 184164,
            "range": "± 5476",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "committer": {
            "email": "34694161+JeroenGar@users.noreply.github.com",
            "name": "JeroenGar",
            "username": "JeroenGar"
          },
          "distinct": true,
          "id": "a037426a7123ab2f4e610d455b098b00b26cb6cd",
          "message": "Remove redundant package homepages",
          "timestamp": "2026-09-15T15:41:39+02:00",
          "tree_id": "01556cf3f832a475798da4e0400ee7d72ff533db",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/a037426a7123ab2f4e610d455b098b00b26cb6cd"
        },
        "date": 1789479896171,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1851730,
            "range": "± 83186",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3928552,
            "range": "± 20921",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 169733,
            "range": "± 6754",
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
          "id": "f421d920c30c85015425be23184047e6562c3ce6",
          "message": "Polish library contracts and prepare the 1.0 release (#104)\n\n* Guard library state with read-only accessors\n\n* Keep surrogates on LBF polygon buffers\n\n* Protect snapshots and hide unused library internals\n\n* Keep snapshot rustdoc focused on its contract\n\n* Add focused hazard lookup and vanilla sparrow migration notes\n\n* Return errors for invalid container quality-zone inputs\n\n* Name collision-only validation explicitly and clarify query contracts\n\n* Restrict CDE snapshots and restore to crate internals\n\n* Fix quality-zone filtering in collision checks\n\n* Finish library contracts and protect problem state\n\n* Prepare 1.0 release notes and reproducible feature CI\n\n* Restructure 1.0 release notes and migration guidance\n\n* Keep quadrant edge scans inlined in QTHazard::constrict\n\nCollecting each quadrant's colliding edges through `quadrants.map` left\ninlining of the closure to LLVM heuristics. The library changes on this\nbranch tipped sparrow's build into outlining it behind\n`core::array::Drain`, making hazard registration about 11% slower in\nexploration (jagua's own lbf build flipped the other way). A plain loop\nkeeps the scan inside `constrict` in both builds. Behaviour is unchanged.\n\n* Remap surrogate hull indices only when vertex orientation flips\n\nReflection support made SPolygon::transform_from copy the reference's\nconvex hull indices on every call, an out-of-line memcpy in the solver's\nper-sample hot path. That copy was the largest new cost in the polygon\ntransform.\n\nA buffer is a clone of its reference, so its hull indices can only differ\nby vertex storage orientation. SPSurrogate now records whether its storage\nis reversed since generation, and transform_from remaps the indices in\nplace only when that orientation changes. A debug assertion checks the\nresult against indices recomputed from the reference.\n\n* Refer to sparrow migration requirements without version qualifiers\n\n* Replace boolean orientation getter with named accessors\n\n* Document single-strip layout export identifier",
          "timestamp": "2026-10-01T15:50:51+02:00",
          "tree_id": "9081f678198a0471d2b39f2256b4bc84adbae897",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/f421d920c30c85015425be23184047e6562c3ce6"
        },
        "date": 1790862961602,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2012273,
            "range": "± 109896",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3744762,
            "range": "± 31006",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 187270,
            "range": "± 5051",
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
          "id": "23e3f2737065cbb4dbc8f400b622976490b2d8cd",
          "message": "Publish jagua-rs releases through crates.io trusted publishing (#105)",
          "timestamp": "2026-10-01T16:03:10+02:00",
          "tree_id": "1ecc644fca71d36063f76bf368407f833a28c28d",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/23e3f2737065cbb4dbc8f400b622976490b2d8cd"
        },
        "date": 1790863678525,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1677687,
            "range": "± 72496",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 2872238,
            "range": "± 52693",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 155685,
            "range": "± 4884",
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
          "id": "e1f0f39a8012ccfe48ce9789389cd7a915b13d16",
          "message": "Generate quality-zone surrogates and add zone filtering (#106)\n\nInferiorQualityZone::new takes an SPSurrogateConfig and generates a\nsurrogate for every collision shape, so zones can be quantified as\ncollision targets. InferiorQualityZone::filtered derives a zone from a\nsubset of its shapes, sharing them instead of converting them again.\n\nBump jagua-rs to 1.1.0.",
          "timestamp": "2026-10-01T17:01:02+02:00",
          "tree_id": "1c1ad9a4270fd0f54c20121404b4ea22429bde8a",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/e1f0f39a8012ccfe48ce9789389cd7a915b13d16"
        },
        "date": 1790867171864,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2026127,
            "range": "± 106698",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3803903,
            "range": "± 16179",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 187391,
            "range": "± 4927",
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
          "id": "0058d418a400c27ff5ccdc939cf2c8b2b8f44281",
          "message": "Generate pier rays relative to the shape centroid (#107)\n\nPier candidate rays were translated by absolute bbox offsets and rotated\naround the world origin, which is only correct when the shape's centroid\nis at the origin (as it is for imported, centered items). For off-origin\nshapes, ray endpoints could land inside the shape and trip the assert in\nclip(), which surfaced in release builds where more rays are generated.\n\nBuild the rays relative to the centroid instead. Results for centered\nshapes are unchanged. Add a translation-invariance regression test and a\nrelease-mode test step to CI.",
          "timestamp": "2026-10-01T17:16:12+02:00",
          "tree_id": "b8e715c2d0d31818625c2278da728877982248e0",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/0058d418a400c27ff5ccdc939cf2c8b2b8f44281"
        },
        "date": 1790868155332,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2240961,
            "range": "± 108188",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4437822,
            "range": "± 23316",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 201454,
            "range": "± 5597",
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
          "id": "6d9da94583aede5811dd0c90c26331445fb12f50",
          "message": "Name SVG container groups and item definitions by container ID (#108)\n\n* Name SVG container groups and item definitions by container ID\n\nlayout_to_svg_group and its wrappers take the layout's external\ncontainer ID, as export_layout_snapshot does. The container group is\nnamed container_{id} again, and item, surrogate and collision-shape\ndefinitions are prefixed with it, so several layouts can share one SVG\ndocument without duplicate element IDs.\n\n* Document SVG container IDs in the 1.1.0 release notes",
          "timestamp": "2026-10-01T18:09:04+02:00",
          "tree_id": "3e353d94b370b0ffea4fb2386b3848e21d7ccd9f",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/6d9da94583aede5811dd0c90c26331445fb12f50"
        },
        "date": 1790871233811,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1666240,
            "range": "± 72350",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 2885539,
            "range": "± 15669",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 155953,
            "range": "± 5283",
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
          "id": "dfd58e7150a6174837efce9d7c89fa791d3b15a8",
          "message": "Define PItemKey in the collision detection module (#110)",
          "timestamp": "2026-10-05T09:56:26+02:00",
          "tree_id": "0b1c030f763c9ddb1026ee1fe9c858c67b79373c",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/dfd58e7150a6174837efce9d7c89fa791d3b15a8"
        },
        "date": 1791187283546,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2039744,
            "range": "± 93849",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3801481,
            "range": "± 81296",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 185362,
            "range": "± 4980",
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
          "id": "ec1dbdb8be7714cea4dc34189e139f0738d59221",
          "message": "Expose collision engine snapshots and a consistency check (#111)",
          "timestamp": "2026-10-05T10:49:15+02:00",
          "tree_id": "b04651745d1cbc1d91d2dec0dc78f025da982486",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/ec1dbdb8be7714cea4dc34189e139f0738d59221"
        },
        "date": 1791190463310,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2058927,
            "range": "± 97116",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3460120,
            "range": "± 64140",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 187920,
            "range": "± 6413",
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
          "id": "b7a6acd1e8180dd41c7c2dbd3b555820eedf9bb0",
          "message": "Bump test-case from 3.3.1 to 3.4.0 (#114)\n\nBumps [test-case](https://github.com/frondeus/test-case) from 3.3.1 to 3.4.0.\n- [Release notes](https://github.com/frondeus/test-case/releases)\n- [Changelog](https://github.com/frondeus/test-case/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/frondeus/test-case/compare/v3.3.1...v3.4.0)\n\n---\nupdated-dependencies:\n- dependency-name: test-case\n  dependency-version: 3.4.0\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-06T01:41:35+02:00",
          "tree_id": "9198cbba640fac03fc47ab3dd5de7388b5d9c62c",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/b7a6acd1e8180dd41c7c2dbd3b555820eedf9bb0"
        },
        "date": 1791243995372,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2009936,
            "range": "± 94169",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3807074,
            "range": "± 27140",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 194027,
            "range": "± 8066",
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
          "id": "a2f352c0336ca2e15cab22657788624ecad639ed",
          "message": "Bump rand from 0.10.2 to 0.10.3 (#117)\n\nBumps [rand](https://github.com/rust-random/rand) from 0.10.2 to 0.10.3.\n- [Release notes](https://github.com/rust-random/rand/releases)\n- [Changelog](https://github.com/rust-random/rand/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/rust-random/rand/compare/0.10.2...0.10.3)\n\n---\nupdated-dependencies:\n- dependency-name: rand\n  dependency-version: 0.10.3\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-06T01:41:50+02:00",
          "tree_id": "a31e7e2732f4e7a6932d57691e4958be1c839ea4",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/a2f352c0336ca2e15cab22657788624ecad639ed"
        },
        "date": 1791244016954,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2047629,
            "range": "± 98835",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3494446,
            "range": "± 70681",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 191318,
            "range": "± 6181",
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
          "id": "7d8fa63334119e8d98bd40a8193dcb533280a1e7",
          "message": "Bump thiserror from 2.0.20 to 2.0.21 (#115)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 2.0.20 to 2.0.21.\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\n- [Commits](https://github.com/dtolnay/thiserror/compare/2.0.20...2.0.21)\n\n---\nupdated-dependencies:\n- dependency-name: thiserror\n  dependency-version: 2.0.21\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-06T01:42:08+02:00",
          "tree_id": "e9ef7ff8bc11f55ca979bafe44f7da48e1068537",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/7d8fa63334119e8d98bd40a8193dcb533280a1e7"
        },
        "date": 1791244030750,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2016508,
            "range": "± 96792",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3791842,
            "range": "± 24496",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 180941,
            "range": "± 5311",
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
          "id": "04d3c5ca7ffe42b51d24df448a61eaf3450142d8",
          "message": "Bump web-sys from 0.3.105 to 0.3.106 (#116)\n\nBumps [web-sys](https://github.com/wasm-bindgen/wasm-bindgen) from 0.3.105 to 0.3.106.\n- [Release notes](https://github.com/wasm-bindgen/wasm-bindgen/releases)\n- [Changelog](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/wasm-bindgen/wasm-bindgen/commits)\n\n---\nupdated-dependencies:\n- dependency-name: web-sys\n  dependency-version: 0.3.106\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-06T01:42:22+02:00",
          "tree_id": "920e2df073ca2a23c571c4af181046dae8db5efb",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/04d3c5ca7ffe42b51d24df448a61eaf3450142d8"
        },
        "date": 1791244039388,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2039371,
            "range": "± 133203",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3809311,
            "range": "± 219248",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 185178,
            "range": "± 4807",
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
          "id": "f38d9db15fa7d721eaa845a34ee286343a4509b2",
          "message": "Filter quality zones by entity and separate filters from collectors (#113)",
          "timestamp": "2026-10-06T01:55:06+02:00",
          "tree_id": "4a2fed64c018384f75882ac2919f3004129ed673",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/f38d9db15fa7d721eaa845a34ee286343a4509b2"
        },
        "date": 1791244809639,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2028463,
            "range": "± 95347",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3799015,
            "range": "± 23255",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 186902,
            "range": "± 5222",
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
          "id": "c7c15b78fe2f987b9b6c71c4483704c8c47ca2fd",
          "message": "Reject simplifications that grow the bounding box (#119)\n\nMerging two convex corners replaces them with the intersection of their\nadjacent edges. That vertex can lie outside the original bounding box: a\nsmall area increase can still make the shape taller or wider, so an item\nthat fits its container no longer does after simplification.",
          "timestamp": "2026-10-09T21:59:04+02:00",
          "tree_id": "5b6f0e7b8969d7753323c43152a70e2429491c7d",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/c7c15b78fe2f987b9b6c71c4483704c8c47ca2fd"
        },
        "date": 1791576220214,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1652512,
            "range": "± 117873",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3104294,
            "range": "± 193715",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 143430,
            "range": "± 8394",
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
          "id": "9e00fd867be0fc493002aea68a1c6613d5b833d6",
          "message": "Keep the largest part when an offset splits a shape, and log it as an error (#121)",
          "timestamp": "2026-10-09T22:31:01+02:00",
          "tree_id": "3d13c90f85f036897871796331776513bda7eebd",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9e00fd867be0fc493002aea68a1c6613d5b833d6"
        },
        "date": 1791578145646,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 1714127,
            "range": "± 89080",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 2737277,
            "range": "± 26777",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 153757,
            "range": "± 4936",
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
          "id": "3becd0cf2889e29cd0942eb8aac3cf2e6df36776",
          "message": "Report why a geometric primitive could not be constructed as a typed GeometryError (#122)",
          "timestamp": "2026-10-13T22:28:43+02:00",
          "tree_id": "dbe9e5a431dfafff0f275451f197d941c0a2e8bd",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/3becd0cf2889e29cd0942eb8aac3cf2e6df36776"
        },
        "date": 1791923611059,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2093656,
            "range": "± 101297",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3795792,
            "range": "± 15736",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 184087,
            "range": "± 5899",
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
          "id": "22e3882b73a2b39644c8b228f48124928603c88f",
          "message": "Bump serde_json from 1.0.151 to 1.0.152 (#123)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.151 to 1.0.152.\n- [Release notes](https://github.com/serde-rs/json/releases)\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.151...v1.0.152)\n\n---\nupdated-dependencies:\n- dependency-name: serde_json\n  dependency-version: 1.0.152\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-14T17:56:09+02:00",
          "tree_id": "3405f72d1a765b40dd513f8c33387f5bca017a03",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/22e3882b73a2b39644c8b228f48124928603c88f"
        },
        "date": 1791993653312,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2090588,
            "range": "± 94979",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 3754111,
            "range": "± 64530",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 183106,
            "range": "± 4466",
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
          "id": "acb393384a7d370f250ace4e39215d2f7afce6dc",
          "message": "Bump jiff from 0.2.37 to 0.2.38 (#124)\n\nBumps [jiff](https://github.com/BurntSushi/jiff) from 0.2.37 to 0.2.38.\n- [Release notes](https://github.com/BurntSushi/jiff/releases)\n- [Changelog](https://github.com/BurntSushi/jiff/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/BurntSushi/jiff/compare/jiff-static-0.2.37...jiff-static-0.2.38)\n\n---\nupdated-dependencies:\n- dependency-name: jiff\n  dependency-version: 0.2.38\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-14T17:55:59+02:00",
          "tree_id": "6d47ad08b6d64003336a26b86f248c1c921d1069",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/acb393384a7d370f250ace4e39215d2f7afce6dc"
        },
        "date": 1791993697600,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 2367038,
            "range": "± 118203",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 4594469,
            "range": "± 23351",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 201405,
            "range": "± 6034",
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
          "id": "efdd0ab37f0a169b8b5ca8ec17ae115dfb6f8629",
          "message": "Redesigned Collision Detection Engine (#125)\n\n* Check collision queries against exact geometry on changing LBF layouts\n\n* Collect collisions on a sorted index of hazard bounding boxes, with edge geometry shared by each hazard\n\n* Detect collisions on the hazard index and remove the quadtree\n\n* Remove the quadtree configuration, HazardEntity::is_dynamic and surrogate piers, and release 2.0.0\n\n* Keep the CDE benchmark IDs the performance tracker knows\n\n* Rename the edges module to narrow_phase and move the box mask to the hazard index",
          "timestamp": "2026-10-14T18:08:21+02:00",
          "tree_id": "65dd4e154944036389934c86e1fb3c86851fb420",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/efdd0ab37f0a169b8b5ca8ec17ae115dfb6f8629"
        },
        "date": 1791994329763,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 790361,
            "range": "± 23342",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 876781,
            "range": "± 16395",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 138132,
            "range": "± 2406",
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
          "id": "d25757d2e669c70ce0326b9eb78df6402d65f2ab",
          "message": "Track only the CDE benchmarks, without the former quadtree depth (#127)\n\nThe CDE benchmarks are now named cde_collect_1k, cde_detect_1k and\ncde_update_1k. The reflection benchmarks move from ci_bench to their own\nuntracked reflection_bench.\n\n\nClaude-Session: https://claude.ai/code/session_01VWi2PhzEuMcqkgM2MqLYML\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-10-14T20:56:00+02:00",
          "tree_id": "02e13605223945cf351312ebe3e6b7325355edd4",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/d25757d2e669c70ce0326b9eb78df6402d65f2ab"
        },
        "date": 1792004306012,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 798664,
            "range": "± 23730",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 707510,
            "range": "± 3264",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 129958,
            "range": "± 2515",
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
          "id": "9f95e23fdd17c78ce8e457a12d7817412640dbfb",
          "message": "Skip index chunks by their largest x_max instead of a window from the widest box (#128)\n\n* Skip index chunks by their largest x_max instead of a window from the widest box\n\nThe window started at r.x_min - max_width, computed in f32 from a rounded\nwidth, so with more than 64 hazards a query could skip the widest hazard\nwhen the two only touch. max_width also never shrank, so one wide hazard\nwidened every later scan.\n\nThe index now keeps the largest x_max of each chunk of 64 boxes and skips\nchunks that end left of the query, which is exact and confines a wide box\nto its own chunk. Also corrects the doc comment of\ncollect_surrogate_collisions_until: poles are not checked largest-first.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\nClaude-Session: https://claude.ai/code/session_01VWi2PhzEuMcqkgM2MqLYML\n\n* Run the strip collision oracle test with doubled swim demands\n\n96 items put the hazard index past one chunk of 64, so the multi-chunk\nscan and the chunk bounds are now covered by CI.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\nClaude-Session: https://claude.ai/code/session_01VWi2PhzEuMcqkgM2MqLYML\n\n---------\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-10-14T21:56:03+02:00",
          "tree_id": "5e7a38629bb07d0ffa60966a94208f44c3e00969",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/9f95e23fdd17c78ce8e457a12d7817412640dbfb"
        },
        "date": 1792007910091,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 762916,
            "range": "± 37788",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 906821,
            "range": "± 29073",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 140198,
            "range": "± 2949",
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
          "id": "4661c08efa9cd9bbf4404abcb73ad337ca8fc778",
          "message": "Ray-cast point-in-polygon only over edges whose box reaches the ray (#129)\n\nContainment checks against interior hazards (a pole inside a hazard, or a\nshape's pole of inaccessibility inside one) walked every hazard edge with\nfour approximate comparisons each. They now use the hazard's cached edge\nboxes to visit only edges whose box reaches the ray, widened in y by the\napproximate comparison's tolerance. The per-edge step is shared with\nSPolygon::collides_with, so the result is identical.\n\n\nClaude-Session: https://claude.ai/code/session_01VWi2PhzEuMcqkgM2MqLYML\n\nCo-authored-by: Claude <noreply@anthropic.com>",
          "timestamp": "2026-10-15T22:27:18+02:00",
          "tree_id": "f91b3ee8ae3e80fceb7d6343144ba76d163af44e",
          "url": "https://github.com/JeroenGar/jagua-rs/commit/4661c08efa9cd9bbf4404abcb73ad337ca8fc778"
        },
        "date": 1792096204982,
        "tool": "cargo",
        "benches": [
          {
            "name": "cde_collect_1k",
            "value": 754582,
            "range": "± 23298",
            "unit": "ns/iter"
          },
          {
            "name": "cde_update_1k",
            "value": 929458,
            "range": "± 5491",
            "unit": "ns/iter"
          },
          {
            "name": "cde_detect_1k",
            "value": 134675,
            "range": "± 2279",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}