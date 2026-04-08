fn main() {
    println!("Hello, world!");
}



Build a simple bookmark manager with proper errors:

REQUIREMENTS:
1. Use thiserror for custom errors
2. Define BookmarkError enum:
   - NotFound
   - AlreadyExists
   - InvalidData
   - Storage errors (from io, json)

3. Implement:
   - save_bookmark() -> Result
   - load_bookmarks() -> Result
   - add_bookmark() -> Result
   - remove_bookmark() -> Result

4. Use ? operator everywhere

5. Handle errors in main

