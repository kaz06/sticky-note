# Sticky Note

This sticky note project was created in Tauri for the author's TypeScript and Rust study.

![Sticky Note](Readme.png)

## Features

- Create, edit, and delete sticky notes.
- Sticky notes can be re-sized, rearranged and memorized their positions.
- It has headlines and can name sticky note groups.


## Development Usage

To get started with  Sticky Note, follow these steps:

1. Install dependencies:
    ```bash
    npm install
    ```
2. Start Sticky Note:  
    Install the Tauri CLI if necessary  
    ```bash
    cargo install tauri-cli
    ```
    This example uses version 1.5.11, but other versions may require configuration adjustments or other changes.

    ```bash
    cargo tauri dev --no-watch
    ```


## create installer
    
```bash
cargo tauri build
```

## License

This project is licensed under the MIT License. 


