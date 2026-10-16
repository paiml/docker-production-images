# hellod and its three Dockerfiles

one program, three images, each smaller than the last

## Run

```bash
docker build -f Dockerfile.scratch -t hellod:scratch .
```

| file | |
|---|---|
| `../../Dockerfile.scratch` | multi-stage build into an empty base |

---

<sub>Generated — do not edit by hand. Edit `examples/hellod/example.toml` and re-render.</sub>
