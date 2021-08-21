[![Richard's GitHub stats](https://github-readme-stats.vercel.app/api?username=raguay)](https://github.com/anuraghazra/github-readme-stats)

# EmailIt

A simple program for sending emails quickly through the Script Server. This version is mostly an exercise to learn Tauri and Rust.

For the first time, you need to download the libraries used by:

```sh
npm install
```

To build the full project, you run:

```sh
npm run build
cargo tauri build
```

The `npm` line builds the frontend Svelte project. The `cargo` line builds the actual rust application that runs the program. 

This requires the Script Server for the backend logic currently. I've not gotten far enough to have it all itegrated. This project is slowly being created.

