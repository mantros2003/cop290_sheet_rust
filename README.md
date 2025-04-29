# Terminal-Based Spreadsheet Application

## Overview
This project is a terminal-based spreadsheet application designed for efficiency, clarity, and scalability. It supports formula evaluation, dependency tracking, and a user-friendly interface using Ratatui for rendering. The application is built in Rust and features a modular architecture for extensibility.

## Features
- **Dependency Management**: Tracks dependencies between cells using a combination of `HashMap` and `RTree` for range-based formulas.
- **User Interface**: Renders the spreadsheet, status bar, and menus using Ratatui with crossterm backend for input handling.
- **Multiple Modes**: Like vim, there are modes like:
  - Normal: Navigate the spreadsheet, and open or save you files.
  - Select: Select a range of data, and use it to plot a graph. Currently only graphing supported. Copy, cut and other features will be added.
  - Insert: Insert values or formulas into the sheet.
- **Render Graphs**: Render graphs of selected data. Currently only barchart supported.
- **Save and Load data into CSV**: Save your work into a .csv file. Or load data from a .csv file. Currently the dependencies are not stored while saving.

## Data Structures
- **HashMap**: Stores cell data with 32-bit integer keys encoding row-column positions.
- **RTree**: Manages range dependencies for efficient spatial queries.
- **Graph**: Tracks dependencies between cells for cycle detection and update propagation.

## Architecture
1. **User Input**: Commands are parsed and validated.
2. **Evaluation**: Dependency graph is updated, and computations are performed.
3. **Storage**: Cell data and metadata are stored in a centralized database.
4. **UI Update**: Changes are reflected on-screen via Ratatui.

## Design Features
- **Modularity**: Ensures extensibility and testability.
- **Centralized Dependency Storage**: Improves performance and reduces memory overhead.
- **Efficient Range Queries**: Uses RTree for fast spatial lookups in large spreadsheets.

## Getting Started
1. **Prerequisites**: Install Rust and Cargo.
2. **Clone the Repository**: 
   ```bash
   git clone https://github.com/mantros2003/cop290_sheet_rust.git
   ```
3. **Build and run**:
   ```bash
   cargo build --release
   cargo run -- <num_rows> <num_columns> --extension
   ```
   - 0 < num_rows <= 999
   - 0 < num_cols <= 18278
   - --extension to run TUI mode
4. **Build using Makefile**:
   - ```bash
     make
     ```
     to build the executable. The executable will be in target/release/ directory.
   - ```bash
     make ext1
     ```
     runs the spreadsheet in TUI mode with 999 rows and 18,278 columns.

## Using the application
   - Start the app by using the commands given above.  Run TUI mode for a better experience.
   - Once the app is loaded, you will be in Insert mode. To navigate, you can use arrow keys.
   - Now, to enter values into a cell, switch to insert mode by pressing i. Once in Insert mode, you cannot navigate to another unless you go back to Normal mode by pressing Esc. Enter the data, or a formula by prefixing a '=' to the formula like:
     ```
     =MAX(A1:C9)
     ```
     and press Enter. You can get a parse error, if cell is out of range, or wrong formula is used.
   - To enter Select mode, press v while in Normal mode. Now use arrow keys to expand or contract the selection area. When the range is selected, press g to plot the barchart of the selected data.
   - To load a file, you must be in Normal mode. While in Normal mode, press ':' and enter the command
     ```
     o <file_name.csv>
     ```
   - To save a file, you must be in Normal mode. While in Normal mode, press ':' and enter the command
     ```
     w <file_name.csv>
     ```
     or if you are editing a csv file pressing ':w' will save the file.
   - To exit the program, press ':q' from Normal mode and hit Enter.

## Contributors
- Lucky Ahirwar (2022CS52049)
- Swapnil (2022ME21540)
- Namrata Sinha (2021ME10995)
