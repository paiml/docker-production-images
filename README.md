<p align="center"><img src="assets/course-image.png" width="160" alt="Docker Production Images"></p>

# Docker Production Images

Multi-stage builds, static binaries and tiny images, measured.

Tested Rust code for the second course of the Docker specialization: hellod is a tiny static program built into a single-stage, a multi-stage and a scratch image, so the size and the attack surface of each can be measured.

## Examples

| example | what it does |
|---|---|
| [`examples/hellod`](examples/hellod) | one program, three images, each smaller than the last |

## Prerequisites

- Docker 25 or newer with buildx
- Rust 1.80 or newer
- A Linux or macOS terminal

---

<sub>Generated — do not edit by hand. Edit `course.toml` and re-render.</sub>
